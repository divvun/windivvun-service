extern crate winapi_tlb_bindgen;
use std::io::prelude::*;

// E45885BF-50CB-4F8F-9B19-95767EAF0F5C


fn main() {
  let out_dir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();

	let idl_content = {
		let mut idl_file = std::fs::File::open(r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\spellcheckprovider.idl").unwrap();
		let mut idl_content = String::new();
		idl_file.read_to_string(&mut idl_content).unwrap();
		idl_content
	};

	let idl_out = {
		let idl_out = out_dir.join("spellcheckprovider.idl");
		let mut idl_out_file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(idl_out.clone()).unwrap();
		
		// Don't care about the version and UUID, this is just needed to generate the tlb that tlb_bindgen needs
		idl_out_file.write_all(b"[
				uuid(00B573B4-E925-4413-9F57-FAC7FE382719),
				version(1.0),
		]
		library SpellCheckProvider {").unwrap();
		idl_out_file.write_all(idl_content.as_bytes()).unwrap();
		idl_out_file.write_all(b"};").unwrap();
		idl_out
	};

	let midl_command_status =
		std::process::Command::new("midl.exe") // Expected to be running in "x64 Native Tools Command Prompt"
		.arg(idl_out)
		.arg("/tlb")
		.arg("SpellCheckProvider.tlb")
		.current_dir(&out_dir)
		.status().unwrap();
  
  assert!(midl_command_status.success());

  let spellcheckprovider_rs = {
		let spellcheckprovider_rs = out_dir.join("spellcheckprovider.rs");
		let spellcheckprovider_rs = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(spellcheckprovider_rs).unwrap();
		std::io::BufWriter::new(spellcheckprovider_rs)
	};

  winapi_tlb_bindgen::build(
    &out_dir.join(r"SpellCheckProvider.tlb"),
    false,
    spellcheckprovider_rs
  ).unwrap();
}