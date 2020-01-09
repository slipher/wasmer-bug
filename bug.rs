extern crate wasmer_runtime;

use wasmer_runtime::imports;
use wasmer_runtime::func;

fn emscripten_saveSetjmp(_env: i32, _label: i32, _table: i32, _size: i32)-> i32 {
    panic!("setjmp called");
}
fn emscripten_testSetjmp(_id: i32, _table: i32, _size: i32)-> i32 {
    panic!("testSetjmp called");
}

fn emscripten_roundf(x: f32) -> f32 {
    panic!("roundf called");
}

fn emscripten_invoke_vi(_0: i32, _1: i32) {
    panic!("wtf");
}
fn emscripten_invoke_iii(_0: i32, _1: i32, _2: i32) -> i32 {
    panic!("what is this shit");
}
fn emscripten_setTempRet0(_: i32) {
    panic!("setTempRet0");
}
fn emscripten_getTempRet0() -> i32 {
    panic!("getTempRet0");
}

fn env_time(_: i32) -> i32 {
    panic!("called env_time");
}
fn env_ctime(_:i32)->i32 {
    panic!("called env_ctime");
}
fn env__exit(_:i32) {
    panic!("called env__exit");
}
fn env_localtime(_:i32)->i32 {
    panic!("called env_localtime");
}
fn env_emscripten_longjmp(_:i32,_:i32) {
    panic!("called env_emscripten_longjmp");
}
fn env_invoke_viiii(_:i32,_:i32,_:i32,_:i32,_:i32) {
    panic!("called env_invoke_viiii");
}
fn env_invoke_iiiii(_:i32,_:i32,_:i32,_:i32,_:i32) -> i32 {
    panic!("called env_invoke_iiiii");
}
fn env_invoke_iiii(_:i32,_:i32,_:i32,_:i32)->i32 {
    panic!("called env_invoke_iiii");
}
fn wsp1_fd_close(_:i32)->i32 {
    panic!("called wsp1_fd_close");
}
fn wsp1_proc_exit(_:i32) {
    panic!("called wsp1_proc_exit");
}
fn env_syscall221(_:i32,_:i32)->i32 {
    panic!("called env_syscall221");
}
fn env_syscall54(_:i32,_:i32)->i32 {
    panic!("called env_syscall54");
}
fn env_syscall5(_:i32,_:i32)->i32 {
    panic!("called env_syscall5");
}
fn wsp1_fd_write(_:i32,_:i32,_:i32,_:i32)->i32 {
    panic!("called wsp1_fd_write");
}
fn env_clock_gettime(_:i32,_:i32)->i32 {
    panic!("called env_clock_gettime");
}

fn print_str(ctx: &mut wasmer_runtime::Ctx, ptr: u32, len: u32) {
    eprintln!("print_str");
}

fn handle_sync_message(ctx: &mut wasmer_runtime::Ctx, data: u32, size: u32, replybuf: u32, bufsize: u32) -> u32 {
    panic!("handle_sync_message");
}


fn w_fd_read(_0: i32, _1: i32, _2: i32, _3: i32) -> i32 {
    panic!("fd_read");
}
fn w_fd_seek(_0: i32, _1: i64, _2: i32, _3: i32) -> i32 {
    panic!("fd_seek");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("bad args");
    }
    let binary: Vec<u8> = std::fs::read(&args[1]).unwrap();
    
    let mut import_object = imports! {
        "env" => {
            "saveSetjmp" => func!(emscripten_saveSetjmp),
            "testSetjmp" => func!(emscripten_testSetjmp),
            "roundf" => func!(emscripten_roundf),
            "invoke_vi" => func!(emscripten_invoke_vi),
            "invoke_iii" => func!(emscripten_invoke_iii),
            "setTempRet0" => func!(emscripten_setTempRet0),
            "getTempRet0" => func!(emscripten_getTempRet0),
            "time" => func!(env_time),
            "ctime" => func!(env_ctime),
            "_exit" => func!(env__exit),
            "localtime" => func!(env_localtime),
            "emscripten_longjmp" => func!(env_emscripten_longjmp),
            "invoke_viiii" => func!(env_invoke_viiii),
            "invoke_iiiii" => func!(env_invoke_iiiii),
            "invoke_iiii" => func!(env_invoke_iiii),
            "__syscall5" => func!(env_syscall5),
            "__syscall221" => func!(env_syscall221),
            "__syscall54" => func!(env_syscall54),
            "clock_gettime" => func!(env_clock_gettime),
            "WasmLog" => func!(print_str),
            "WasmSendMsg" => func!(handle_sync_message),
        },
        "wasi_snapshot_preview1" => {
            "fd_read" => func!(w_fd_read),
            "fd_seek" => func!(w_fd_seek),
            "fd_write" => func!(wsp1_fd_write),
            "fd_close" => func!(wsp1_fd_close),
            "proc_exit" => func!(wsp1_proc_exit),
        },
    };

    let instance = wasmer_runtime::instantiate(&binary, &import_object).unwrap();
    eprintln!("instantiated");
    instance.call("_start", &[]).unwrap();
    eprintln!("static inited");
}
