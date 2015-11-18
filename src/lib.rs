#![feature(no_std, lang_items)]
#![feature(const_fn, unique, core_str_ext, iter_cmp)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
pub mod vga_buffer;
pub mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
	let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
	let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF-sections tag required");

	// ATTENTION: we have a very small stack and no guard page
	vga_buffer::clear_screen();
	println!("Hello, World!");

	println!("memory areas:");
	for area in memory_map_tag.memory_areas() {
		println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	}

	// println!("kernel sections:");
	// for section in elf_sections_tag.sections() {
	// 	println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
	// 		section.addr, section.size, section.flags);
	// }

	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

	let multiboot_start = multiboot_information_address;
	let multiboot_end = multiboot_start + (boot_info.total_size as usize);

	println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
	println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

	let mut frame_allocator = memory::AreaFrameAllocator::new(
		kernel_start as usize, kernel_end as usize,
		multiboot_start, multiboot_end, memory_map_tag.memory_areas());

	for i in 0.. {
		use memory::FrameAllocator;
		if let None = frame_allocator.allocate_frame() {
			println!("allocated {} frames", i);
			break;
		}
	}

	loop{}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
	println!("\n\nPANIC in {} at line {}:", file, line);
	println!("    {}", fmt);
	loop{}
}