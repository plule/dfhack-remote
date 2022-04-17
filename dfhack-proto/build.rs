use std::{collections::HashMap, io::BufRead, path::PathBuf};

use heck::{ToPascalCase, ToSnakeCase};
use regex::Regex;

struct RPC {
    pub name: String,
    pub input: String,
    pub output: String,
}

struct Plugin {
    pub plugin_name: String,
    pub module_name: String,
    pub struct_name: String,
    pub member_name: String,
    pub rpcs: Vec<RPC>,
}

impl Plugin {
    pub fn new(plugin_name: &String) -> Self {
        let plugin_name = plugin_name.clone();
        let mut base_name = plugin_name.clone();
        if base_name.is_empty() {
            base_name = "core".to_string();
        }

        let module_name = base_name.to_snake_case();
        let struct_name = base_name.to_pascal_case();
        let member_name = base_name.to_snake_case();

        Self {
            plugin_name,
            module_name,
            struct_name,
            member_name,
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

fn generate_plugins_rs(protos: &Vec<PathBuf>, out_path: &PathBuf) {
    let plugins = read_protos_rpcs(&protos);
    let mut out_path = out_path.clone();
    out_path.push("plugins");
    std::fs::create_dir_all(&out_path).unwrap();

    for (plugin_name, plugin) in &plugins {
        generate_plugin_rs(&plugin_name, &plugin, &out_path);
    }

    generate_plugin_mod_rs(&plugins, out_path);
}

fn generate_plugin_mod_rs(plugins: &HashMap<String, Plugin>, out_path: PathBuf) {
    let mut mod_rs_lines = Vec::new();
    for (_, plugin) in plugins {
        mod_rs_lines.push(format!("mod {};", &plugin.module_name));
        mod_rs_lines.push(format!("pub use self::{}::*;", &plugin.module_name));
    }

    mod_rs_lines.push(r"/// Generated list of DFHack plugins".to_string());
    mod_rs_lines.push(r"pub struct Plugins<TProtocol: crate::ProtocolTrait<E>, E> {".to_string());

    for (_, plugin) in plugins {
        mod_rs_lines.push(format!("    /// RPCs of the {} plugin", plugin.plugin_name));
        mod_rs_lines.push(format!(
            "    pub {}: crate::plugins::{}<E, TProtocol>,",
            plugin.module_name, plugin.struct_name
        ));
    }
    mod_rs_lines.push(r"}".to_string());
    mod_rs_lines
        .push(r"impl<TProtocol: crate::ProtocolTrait<E>, E> Plugins<TProtocol, E> {".to_string());

    mod_rs_lines.push(r"    /// Initiate all the generated plugins".to_string());
    mod_rs_lines.push(
        r"    pub fn new(protocol: std::rc::Rc<std::cell::RefCell<TProtocol>>) -> Self {"
            .to_string(),
    );
    mod_rs_lines.push(r"        Self {".to_string());

    for (_, plugin) in plugins {
        mod_rs_lines.push(format!(
            "            {}: {}::new(std::rc::Rc::clone(&protocol)),",
            plugin.member_name, plugin.struct_name
        ));
    }

    mod_rs_lines.push(r"        }".to_string());
    mod_rs_lines.push(r"    }".to_string());
    mod_rs_lines.push(r"}".to_string());

    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs_lines.join("\n")).unwrap();
}

fn generate_plugin_rs(plugin_name: &String, plugin: &Plugin, out_path: &PathBuf) {
    let mut out_path = out_path.clone();
    out_path.push(format!("{}.rs", plugin.module_name));

    let mut lines = Vec::new();
    lines.push(r"use std::{cell::RefCell, rc::Rc};".to_string());
    //lines.push(r"use crate::ProtocolTrait::Protocol;".to_string());
    lines.push(r"use crate::messages::*;".to_string());
    lines.push(r"use std::marker::PhantomData;".to_string());

    lines.push(format!("/// {} plugin", plugin_name));
    lines.push(format!(
        r"pub struct {}<E, TProtocol: crate::ProtocolTrait<E>> {{",
        plugin.struct_name
    ));
    lines.push(r"    /// Reference to the client to exchange messages".to_string());
    lines.push(r"    pub protocol: Rc<RefCell<TProtocol>>,".to_string());
    lines.push(r"    /// Name of the plugin. All the RPC are attached to this name.".to_string());
    lines.push(r"    pub name: String,".to_string());
    lines.push(r"    phantom: PhantomData<E>,".to_string());
    lines.push(r"}".to_string());

    lines.push(format!(
        "impl<E, TProtocol: crate::ProtocolTrait<E>> {}<E, TProtocol> {{",
        plugin.struct_name
    ));
    lines.push(r"    /// Instanciate a new plugin instance.".to_string());
    lines.push(r"    pub fn new(protocol: Rc<RefCell<TProtocol>>) -> Self {".to_string());
    lines.push(r"        Self {".to_string());
    lines.push(r"            protocol,".to_string());
    lines.push(format!(
        "            name: \"{}\".to_string(),",
        plugin_name
    ));
    lines.push(r"            phantom: PhantomData,".to_string());
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
