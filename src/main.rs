extern crate libloading;
extern crate libc;

unsafe extern "C" fn callback() -> () {
    println!("callback()");
}

fn main() {
    println!("main()");
    call_dynamic();
}

fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("lib.dll")?;
        let foo: libloading::Symbol<unsafe extern fn(unsafe extern "C" fn()) -> u32> = lib.get(b"foo")?;
        Ok(foo(callback))
    }
}