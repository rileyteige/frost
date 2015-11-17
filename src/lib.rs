#![feature(no_std, lang_items)]
#![feature(const_fn, unique, core_str_ext)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
pub mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page
	vga_buffer::clear_screen();
	print!("Hello World{}", "!");

	loop{}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop{}}