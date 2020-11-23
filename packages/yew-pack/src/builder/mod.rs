use crate::error::Result;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Command;
use wasm_bindgen_cli_support::{Bindgen, EncodeInto};

pub fn build_yew_project(options: crate::cli::BuildOptions) -> Result<()> {
    /*
    [1] Build the project with cargo, generating a wasm32-unknown-unknown target (is there a more specific, better target to leverage?)
    [2] Generate the appropriate build folders
    [3] Wasm-bindgen the .wasm fiile, and move it into the {builddir}/modules/xxxx/xxxx_bg.wasm
    [4] Wasm-opt the .wasm file with whatever optimizations need to be done
    [5] Link up the html page to the wasm module
    */

    let BuildConfig {
        crate_dir,
        out_dir,
        target_dir,
        workspace_dir,
    } = BuildConfig::from_build_options(&options)?;

    // [1] Build the .wasm module
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&crate_dir)
        .arg("build")
        .arg("--lib")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        // Hide cargo output, because we are using stderr to track build time.
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn()?; //.expect("Failed to execute child");
    let err_code = child.wait()?; //.expect("Failed to wait on child");

    // [2] Establish the output directory structure
    let bindgen_outdir = out_dir.join("wasm");

    // [3] Bindgen the final binary for use easy linking
    let mut bindgen_builder = Bindgen::new();
    bindgen_builder
        .input_path(target_dir.join("wasm32-unknown-unknown/release/yew_pack_example.wasm"))
        .web(true)?
        .debug(true)
        .demangle(true)
        .keep_debug(true)
        .remove_name_section(true)
        .remove_producers_section(true)
        .out_name("module")
        .generate(&bindgen_outdir)?;

    // [4]
    // TODO: wasm-opt

    // [5] Generate the html file with the module name
    // TODO: support names via options
    let mut file = std::fs::File::create(out_dir.join("index.html"))?;
    file.write_all(gen_page("./wasm/module.js").as_str().as_bytes())?;

    Ok(())
}

fn gen_page(module: &str) -> String {
    format!(
        r#"
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body>
    <!-- Note the usage of `type=module` here as this is an ES6 module -->
    <script type="module">
      import init from "{}";
      init();
    </script>
  </body>
</html>
"#,
        module
    )
}

struct BuildConfig {
    out_dir: PathBuf,
    crate_dir: PathBuf,
    workspace_dir: PathBuf,
    target_dir: PathBuf,
}
impl BuildConfig {
    pub(crate) fn from_build_options(options: &crate::cli::BuildOptions) -> Result<Self> {
        let crate_dir = crate::cargo::crate_root()?;
        let workspace_dir = crate::cargo::workspace_root()?;
        let target_dir = workspace_dir.join("target");
        let out_dir = crate_dir.join("public");

        Ok(Self {
            out_dir,
            crate_dir,
            workspace_dir,
            target_dir,
        })
    }
}
