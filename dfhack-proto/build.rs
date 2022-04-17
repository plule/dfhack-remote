use std::{collections::HashMap, io::BufRead, path::PathBuf};

use heck::{ToPascalCase, ToSnakeCase};
use prettyplease;
use quote::ToTokens;
use quote::__private::Ident;
use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;
use regex::Regex;
use syn;

struct RPC {
    pub name: String,
    pub input: String,
    pub output: String,
}

struct Plugin {
    pub plugin_name: String,

    pub struct_ident: Ident,
    pub member_ident: Ident,
    pub rpcs: Vec<RPC>,
}

impl Plugin {
    pub fn new(plugin_name: &String) -> Self {
        let plugin_name = plugin_name.clone();
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

    let proto_include_dir = dfhack_proto_srcs::include_dir();
    let protos = dfhack_proto_srcs::protos();

    // Recreate the generated folder
    // should go to OUT_DIR instead
    let out_path = PathBuf::from("src/generated");
    if out_path.exists() {
        std::fs::remove_dir_all("src/generated").unwrap();
    }
    std::fs::create_dir_all("src/generated").unwrap();

    // Generate the protobuf message files
    generate_messages_rs(&protos, &proto_include_dir, &out_path);

    // Generate the plugins
    generate_plugins_rs(&protos, &out_path)
}

fn generate_messages_rs(protos: &Vec<PathBuf>, include_dir: &str, out_path: &PathBuf) {
    let mut out_path = out_path.clone();
    out_path.push("messages");
    std::fs::create_dir_all(&out_path).unwrap();
    messages_protoc_codegen(protos, include_dir, &out_path);
    messages_generate_mod_rs(protos, &out_path);
}

// Call the protoc code generation
fn messages_protoc_codegen(protos: &Vec<PathBuf>, include_dir: &str, out_path: &PathBuf) {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include(include_dir)
        .run()
        .expect("Codegen failed.");
}

fn messages_generate_mod_rs(protos: &Vec<PathBuf>, out_path: &PathBuf) {
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
    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    let tree = syn::parse2(file).unwrap();
    let formatted = prettyplease::unparse(&tree);

    std::fs::write(mod_rs_path, formatted).unwrap();
}

fn generate_plugins_rs(protos: &Vec<PathBuf>, out_path: &PathBuf) {
    let plugins = read_protos_rpcs(&protos);
    let mut out_path = out_path.clone();
    out_path.push("plugins");
    std::fs::create_dir_all(&out_path).unwrap();

    let mut file = quote!();
    generate_plugins_mod_rs(&plugins, &mut file);

    for (plugin_name, plugin) in &plugins {
        generate_plugin_rs(&plugin_name, &plugin, &mut file);
    }

    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    let tree = syn::parse2(file).unwrap();
    let formatted = prettyplease::unparse(&tree);

    std::fs::write(mod_rs_path, formatted).unwrap();
}

fn generate_plugins_mod_rs(plugins: &HashMap<String, Plugin>, file: &mut TokenStream) {
    let mut plugins_struct = quote!();
    let mut new_method = quote!();

    file.extend(quote! {
        use std::{cell::RefCell, rc::Rc};
        use crate::messages::*;
        use std::marker::PhantomData;
    });

    for (_, plugin) in plugins {
        let doc = format!("RPCs of the {} plugin", plugin.plugin_name);
        let struct_ident = plugin.struct_ident.clone();
        let member_ident = plugin.member_ident.clone();
        plugins_struct.extend(quote! {
            #[doc = #doc]
            pub #member_ident: crate::plugins::#struct_ident<E, TProtocol>,
        });

        new_method.extend(quote! {
            #member_ident: #struct_ident::new(std::rc::Rc::clone(&protocol)),
        });
    }
    file.extend(quote! {
        #[doc = "Generated list of DFHack plugins"]
        pub struct Plugins<TProtocol: crate::ProtocolTrait<E>, E> {
            #plugins_struct
        }
    });

    file.extend(quote! {
        impl<TProtocol: crate::ProtocolTrait<E>, E> From<TProtocol> for Plugins<TProtocol, E> {
            #[doc = "Initialize all the generated plugins"]
            fn from(protocol: TProtocol) -> Self {
                let protocol = std::rc::Rc::new(std::cell::RefCell::new(protocol));
                Self {
                    #new_method
                }
            }
        }
    });
}

fn generate_plugin_rs(plugin_name: &String, plugin: &Plugin, file: &mut TokenStream) {
    let plugin_doc = format!("RPC for the \"{}\" plugin.", plugin_name);
    let struct_ident = plugin.struct_ident.clone();

    file.extend(quote! {
        #[doc = #plugin_doc]
        pub struct #struct_ident<E, TProtocol: crate::ProtocolTrait<E>> {
            #[doc = "Reference to the client to exchange messages."]
            pub protocol: Rc<RefCell<TProtocol>>,

            #[doc = "Name of the plugin. All the RPC are attached to this name."]
            pub name: String,

            phantom: PhantomData<E>,
        }
    });

    let mut plugin_impl = quote! {
        #[doc = "Instanciate a new plugin instance"]
        pub fn new(protocol: Rc<RefCell<TProtocol>>) -> Self {
            Self {
                protocol,
                name: #plugin_name.to_string(),
                phantom: PhantomData,
            }
        }
    };

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
            ) -> Result<#return_token, E> {
                #prep
                let _response: #output_ident = self.protocol.borrow_mut().request(
                    #plugin_name.to_string(),
                    #function_name.to_string(),
                    request,
                )?;
                #post
                Ok(_response)
            }
        });
    }

    file.extend(quote! {
        impl<E, TProtocol: crate::ProtocolTrait<E>> #struct_ident<E, TProtocol> {
            #plugin_impl
        }
    });
}

fn read_protos_rpcs(protos: &Vec<PathBuf>) -> HashMap<String, Plugin> {
    let mut rpcs = HashMap::<String, Plugin>::new();

    for proto in protos {
        let (plugin_name, mut proto_rpcs) = read_proto_rpc(proto);

        let plugin = rpcs
            .entry(plugin_name.clone())
            .or_insert(Plugin::new(&plugin_name));
        plugin.rpcs.append(&mut proto_rpcs);
    }

    rpcs
}

fn read_proto_rpc(proto: &PathBuf) -> (String, Vec<RPC>) {
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
            rpcs.push(RPC {
                name: rpc_capture[1].to_string(),
                input: rpc_capture[2].to_string(),
                output: rpc_capture[3].to_string(),
            });
        }
    }

    (plugin, rpcs)
}
