extern crate wasmer_runtime;

use wasmer_runtime::imports;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("bad args");
    }
    let binary: Vec<u8> = std::fs::read(&args[1]).unwrap();
    
    let mut import_object = imports! {
    };
    import_object.allow_missing_functions = true;
    let instance = wasmer_runtime::instantiate(&binary, &import_object).unwrap();
    eprintln!("instantiated");
}
