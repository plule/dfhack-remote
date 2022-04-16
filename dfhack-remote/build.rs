extern crate protobuf_codegen_pure;
use std::{collections::HashMap, io::BufRead, path::PathBuf};

use heck::{ToPascalCase, ToSnakeCase};
use regex::Regex;

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

    generate_plugins_rs(&protos, &out_path)
}

fn generate_plugins_rs(protos: &Vec<PathBuf>, out_path: &PathBuf) {
    let rpcs = read_protos_rpcs(&protos);
    let mut out_path = out_path.clone();
    out_path.push("plugins");
    std::fs::create_dir_all(&out_path).unwrap();
    let mut mod_rs_lines = Vec::new();
    for (plugin_name, plugin) in rpcs {
        let mut module_name = plugin_name.to_snake_case();
        if module_name.is_empty() {
            module_name = "core".to_string();
        }

        mod_rs_lines.push(format!("mod {};", module_name));
        mod_rs_lines.push(format!("pub use self::{}::*;", module_name));

        generate_plugin_rs(&module_name, &plugin_name, &plugin, &out_path);
    }

    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs_lines.join("\n")).unwrap();
}

fn generate_plugin_rs(
    module_name: &String,
    plugin_name: &String,
    plugin: &Plugin,
    out_path: &PathBuf,
) {
    let mut out_path = out_path.clone();
    out_path.push(format!("{}.rs", module_name));

    let mut struct_name = plugin_name.to_pascal_case();
    if struct_name.is_empty() {
        struct_name = "Core".to_string();
    }

    let mut lines = Vec::new();
    lines.push(r"use std::{cell::RefCell, rc::Rc};".to_string());
    lines.push(r"use crate::protocol::Protocol;".to_string());
    lines.push(r"use crate::generated::messages::*;".to_string());

    lines.push(format!("/// {} plugin", plugin_name));
    lines.push(format!(r"pub struct {} {{", struct_name));
    lines.push(r"    /// Reference to the client to exchange messages".to_string());
    lines.push(r"    pub client: Rc<RefCell<Protocol>>,".to_string());
    lines.push(r"    /// Name of the plugin. All the RPC are attached to this name.".to_string());
    lines.push(r"    pub name: String,".to_string());
    lines.push(r"}".to_string());

    lines.push(format!("impl {} {{", struct_name));
    lines.push(r"    /// Instanciate a new plugin instance.".to_string());
    lines.push(r"    /// You should likely use [crate::DFHack] instead.".to_string());
    lines.push(r"    pub fn new(client: Rc<RefCell<Protocol>>) -> Self {".to_string());
    lines.push(r"        Self {".to_string());
    lines.push(r"            client,".to_string());
    lines.push(format!("            name: \"{}\".to_string()", plugin_name));
    lines.push(r"        }".to_string());
    lines.push(r"    }".to_string());

    for rpc in &plugin.rpcs {
        let function_name = rpc.name.to_snake_case();
        lines.push(r"    crate::plugins::make_plugin_request!(".to_string());
        lines.push(format!(
            "        /// Method `{}` from the plugin `{}`",
            function_name, plugin_name
        ));
        lines.push(format!(
            "        {}, \"{}\", {}, {}",
            function_name, rpc.name, rpc.input, rpc.output
        ));
        lines.push(r"    );".to_string());
    }

    lines.push(r"}".to_string());

    std::fs::write(out_path, lines.join("\n")).unwrap();
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
    // Generate the mod.rs file
    let mut mod_rs = Vec::new();

    for proto in protos {
        let mod_name = proto
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        mod_rs.push(format!("mod {};", mod_name));
        mod_rs.push(format!("pub use self::{}::*;", mod_name));
    }
    // Write mod.rs
    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs.join("\n")).unwrap();
}

struct RPC {
    pub name: String,
    pub input: String,
    pub output: String,
}

#[derive(Default)]
struct Plugin {
    pub rpcs: Vec<RPC>,
}

fn read_protos_rpcs(protos: &Vec<PathBuf>) -> HashMap<String, Plugin> {
    let mut rpcs = HashMap::<String, Plugin>::new();

    for proto in protos {
        let (plugin, mut proto_rpcs) = read_proto_rpc(proto);
        let plugin = rpcs.entry(plugin).or_default();
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
