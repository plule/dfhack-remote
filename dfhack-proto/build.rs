use std::path::Path;
use std::{collections::HashMap, io::BufRead, path::PathBuf};

use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use syn::Ident;

struct Rpc {
    pub name: String,
    pub input: String,
    pub output: String,
}

struct Plugin {
    pub plugin_name: String,

    pub struct_ident: Ident,
    pub member_ident: Ident,
    pub rpcs: Vec<Rpc>,
}

impl Plugin {
    pub fn new(plugin_name: &str) -> Self {
        let plugin_name = plugin_name.to_owned();
        let mut base_name = plugin_name.clone();
        if base_name.is_empty() {
            base_name = "core".to_string();
        }

        let struct_name = base_name.to_pascal_case();
        let member_name = base_name.to_snake_case();

        Self {
            member_ident: format_ident!("{}", &member_name),
            struct_ident: format_ident!("{}", &struct_name),
            plugin_name,
            rpcs: Vec::new(),
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DFHACK_REGEN");

    let proto_include_dir = dfhack_proto_srcs::include_dir();
    let protos = dfhack_proto_srcs::protos();

    assert!(!protos.is_empty(), "No protobuf file for code generation.");

    // Generate in the sources if DFHACK_REGEN is set
    // in OUT_DIR otherwise.
    // TODO It should always be OUT_DIR, then expose macros.
    let out_path = match std::env::var("DFHACK_REGEN") {
        Ok(_) => {
            let dst = PathBuf::from("src/generated");
            if dst.exists() {
                std::fs::remove_dir_all(&dst).unwrap();
            }
            std::fs::create_dir_all(&dst).unwrap();
            dst
        }
        Err(_) => PathBuf::from(std::env::var("OUT_DIR").unwrap()),
    };

    // Generate the protobuf message files
    generate_messages_rs(&protos, proto_include_dir, &out_path);

    // Generate the plugin stubs
    generate_stubs_rs(&protos, &out_path)
}

fn generate_messages_rs(protos: &Vec<PathBuf>, include_dir: &str, out_path: &Path) {
    let mut out_path = out_path.to_path_buf();
    out_path.push("messages");
    std::fs::create_dir_all(&out_path).unwrap();
    messages_protoc_codegen(protos, include_dir, &out_path);
    messages_generate_mod_rs(protos, &out_path);
}

// Call the protoc code generation
fn messages_protoc_codegen(protos: &Vec<PathBuf>, include_dir: &str, out_path: &Path) {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include(include_dir)
        .run()
        .expect("Codegen failed.");
}

fn messages_generate_mod_rs(protos: &Vec<PathBuf>, out_path: &Path) {
    let mut file = quote!();

    for proto in protos {
        let mod_name = proto
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mod_name: Ident = format_ident!("{}", mod_name);

        file.extend(quote! {
            mod #mod_name;
            pub use self::#mod_name::*;
        });
    }
    // Write mod.rs
    let mut mod_rs_path = out_path.to_path_buf();
    mod_rs_path.push("mod.rs");
    let tree = syn::parse2(file).unwrap();
    let formatted = prettyplease::unparse(&tree);

    std::fs::write(mod_rs_path, formatted).unwrap();
}

fn generate_stubs_rs(protos: &Vec<PathBuf>, out_path: &Path) {
    let plugins = read_protos_rpcs(protos);
    let mut out_path = out_path.to_path_buf();
    out_path.push("stubs");
    std::fs::create_dir_all(&out_path).unwrap();

    let mut file = quote!();
    generate_stubs_mod_rs(&plugins, &mut file);

    for plugin in &plugins {
        generate_stub_rs(plugin, &mut file);
    }

    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    let tree = syn::parse2(file).unwrap();
    let formatted = prettyplease::unparse(&tree);

    std::fs::write(mod_rs_path, formatted).unwrap();
}

fn generate_stubs_mod_rs(plugins: &Vec<Plugin>, file: &mut TokenStream) {
    let mut plugins_impl = quote!();

    file.extend(quote! {
        use crate::messages::*;
        #[cfg(feature = "reflection")]
        use protobuf::Message;
    });

    let mut reflection_vec_building = quote!();

    file.extend(quote! {
        #[doc = "Generated list of DFHack stubs. Each stub communicates with a plugin."]
        pub struct Stubs<TChannel: crate::Channel> {
            channel: TChannel,
        }
    });

    for plugin in plugins {
        let doc = format!("RPCs of the {} plugin", plugin.plugin_name);
        let struct_ident = plugin.struct_ident.clone();
        let member_ident = plugin.member_ident.clone();
        plugins_impl.extend(quote! {
            #[doc = #doc]
            pub fn #member_ident(&mut self) -> crate::stubs::#struct_ident<TChannel> {
                crate::stubs::#struct_ident::new(&mut self.channel)
            }
        });

        reflection_vec_building.extend(quote! {
            methods.extend(Core::<TChannel>::list_methods());
        });
    }

    file.extend(quote! {
        impl<TChannel: crate::Channel> From<TChannel> for Stubs<TChannel> {
            #[doc = "Initialize all the generated stubs."]
            fn from(channel: TChannel) -> Self {
                Self { channel }
            }
        }

        impl<TChannel: crate::Channel> Stubs<TChannel> {
            #plugins_impl
        }

        #[cfg(feature = "reflection")]
        impl<TChannel: crate::Channel> crate::reflection::StubReflection for Stubs<TChannel> {
            fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
                let mut methods = Vec::new();
                #reflection_vec_building
                methods
            }
        }
    });
}

fn generate_stub_rs(plugin: &Plugin, file: &mut TokenStream) {
    let plugin_name = &plugin.plugin_name;
    let plugin_doc = format!("RPC for the \"{}\" plugin.", plugin_name);
    let struct_ident = plugin.struct_ident.clone();

    file.extend(quote! {
        #[doc = #plugin_doc]
        pub struct #struct_ident<'a, TChannel: crate::Channel> {
            #[doc = "Reference to the client to exchange messages."]
            pub channel: &'a mut TChannel,
        }
    });

    let mut plugin_impl = quote! {
        #[doc = "Initialize the plugin from a channel to DFHack."]
        pub fn new(channel: &'a mut TChannel) -> Self {
            Self { channel }
        }
    };

    let mut reflection_vec = quote!();

    for rpc in &plugin.rpcs {
        let function_name = &rpc.name;
        let doc = format!(
            "Method `{}` from the plugin `{}`",
            function_name, plugin_name
        );
        let function_ident = format_ident!("{}", rpc.name.to_snake_case());
        let input_ident = format_ident!("{}", rpc.input);
        let output_ident = format_ident!("{}", rpc.output);

        let mut return_token = output_ident.to_token_stream();

        let mut parameters = quote! {
            &mut self,
            request: #input_ident,
        };

        let mut prep = quote!();
        let mut post = quote!();

        // Syntaxic sugars for simple messages (empty, single value)
        if rpc.input == "EmptyMessage" {
            parameters = quote! {
                &mut self,
            };
            prep = quote! {
                let request = EmptyMessage::new();
            }
        }

        if rpc.output == "EmptyMessage" {
            return_token = quote! {
                ()
            };
            post = quote! {
                let _response = ();
            }
        }

        if rpc.input == "IntMessage" {
            parameters = quote! {
                &mut self,
                value: i32,
            };
            prep = quote! {
                let mut request = IntMessage::new();
                request.set_value(value);
            }
        }

        if rpc.output == "IntMessage" {
            return_token = quote! {
                i32
            };
            post = quote! {
                let _response = _response.get_value();
            }
        }

        if rpc.input == "SingleBool" {
            parameters = quote! {
                &mut self,
                value: bool,
            };
            prep = quote! {
                let mut request = SingleBool::new();
                request.set_Value(value);
            }
        }

        if rpc.output == "SingleBool" {
            return_token = quote! {
                bool
            };
            post = quote! {
                let _response = _response.get_Value();
            }
        }

        if rpc.output == "StringMessage" {
            return_token = quote! {
                String
            };
            post = quote! {
                let _response = _response.get_value().to_string();
            }
        }

        plugin_impl.extend(quote! {
            #[doc = #doc]
            pub fn #function_ident(
                #parameters
            ) -> Result<#return_token, TChannel::TError> {
                #prep
                let _response: #output_ident = self.channel.request(
                    #plugin_name.to_string(),
                    #function_name.to_string(),
                    request,
                )?;
                #post
                Ok(_response)
            }
        });

        reflection_vec.extend(quote! {
            crate::reflection::RemoteProcedureDescriptor {
                name: #function_name.to_string(),
                plugin_name: #plugin_name.to_string(),
                input_type: #input_ident::descriptor_static()
                    .full_name()
                    .to_string(),
                output_type: #output_ident::descriptor_static()
                    .full_name()
                    .to_string(),
            },
        });
    }

    file.extend(quote! {
        impl<'a, TChannel: crate::Channel> #struct_ident<'a, TChannel> {
            #plugin_impl
        }

        #[cfg(feature = "reflection")]
        impl<TChannel: crate::Channel> crate::reflection::StubReflection for #struct_ident<'_, TChannel> {
            fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
                vec![
                    #reflection_vec
                ]
            }
        }
    });
}

fn read_protos_rpcs(protos: &Vec<PathBuf>) -> Vec<Plugin> {
    let mut plugins = HashMap::<String, Plugin>::new();

    for proto in protos {
        let (plugin_name, mut proto_rpcs) = read_proto_rpc(proto);

        let plugin = plugins
            .entry(plugin_name.clone())
            .or_insert_with(|| Plugin::new(&plugin_name));
        plugin.rpcs.append(&mut proto_rpcs);
    }

    let mut plugins: Vec<Plugin> = plugins.into_values().collect();

    for plugin in &mut plugins {
        plugin.rpcs.sort_by(|a, b| a.name.cmp(&b.name));
    }

    plugins.sort_by(|a, b| a.plugin_name.cmp(&b.plugin_name));

    plugins
}

fn read_proto_rpc(proto: &PathBuf) -> (String, Vec<Rpc>) {
    let file = std::fs::File::open(proto).unwrap();
    let plugin_regex = Regex::new(r"// Plugin: (\w+)").unwrap();
    let rpc_regex = Regex::new(r"// RPC (\w+) : (\w+) -> (\w+)").unwrap();

    let mut plugin = "".to_string();
    let mut rpcs = Vec::new();

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let plugin_capture = plugin_regex.captures(&line);
        if let Some(plugin_capture) = plugin_capture {
            plugin = plugin_capture[1].to_string();
        }

        let rpc_capture = rpc_regex.captures(&line);
        if let Some(rpc_capture) = rpc_capture {
            rpcs.push(Rpc {
                name: rpc_capture[1].to_string(),
                input: rpc_capture[2].to_string(),
                output: rpc_capture[3].to_string(),
            });
        }
    }

    (plugin, rpcs)
}
