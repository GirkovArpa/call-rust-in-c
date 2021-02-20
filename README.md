# Call Rust In C

Simple demo of sending Rust function pointer to C library, which executes it.

## Instructions

```c
__declspec(dllexport) void foo(void (*callback)()) {
    callback();
}
```
Compile the above C code to a dynamic link library with:
```bash
gcc --shared lib.c -o lib.dll
```
```rust
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
```
Compile and run the above Rust code with:
```bash
cargo run
```
You should see this in your console:
```
main()
callback()
```