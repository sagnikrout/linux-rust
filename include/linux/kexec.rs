//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kexec.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
pub const IND_DESTINATION_BIT: c_int = 0;
pub const IND_INDIRECTION_BIT: c_int = 1;
pub const IND_DONE_BIT: c_int = 2;
pub const IND_SOURCE_BIT: c_int = 3;

// Verify architecture specific macros are defined

//
// This structure is used to hold the arguments that are used when loading
// kernel binaries.
//
pub type kimage_entry_t = c_ulong;
//
// This is a copy of the UAPI struct kexec_segment and must be identical
// to it because it gets copied straight from user space into kernel
// memory. Do not modify this structure unless you change the way segments
// get ingested from user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_segment {
//
// This pointer can point to user memory if kexec_load() system
// call is used or will point to kernel memory if
// kexec_file_load() system call is used.
//
// Use ->buf when expecting to deal with user memory and use ->kbuf
// when expecting to deal with kernel memory.
//
    pub buf: *mut void __user,
    pub kbuf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_kexec_segment {
    pub buf: compat_uptr_t,
    pub bufsz: compat_size_t,
    pub /: *mut *mut *mut compat_ulong_t mem; / User space sees this as a (void ) ...,
    pub memsz: compat_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct purgatory_info {
//
// Pointer to elf header at the beginning of kexec_purgatory.
// Note: kexec_purgatory is read only
//
    pub ehdr: *const Elf_Ehdr,
//
// Temporary, modifiable buffer for sechdrs used for relocation.
// This memory can be freed post image load.
//
    pub sechdrs: *mut Elf_Shdr,
//
// Temporary, modifiable buffer for stripped purgatory used for
// relocation. This memory can be freed post image load.
//
    pub purgatory_buf: *mut c_void,
}

extern "C" {
    pub fn int(kernel_buf: *const kexec_probe_t)(char, kernel_size: c_ulong) -> typedef;
}
extern "C" {
    pub fn int(loader_data: *mut kexec_cleanup_t)(void) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_file_ops {
    pub probe: *mut kexec_probe_t,
    pub load: *mut kexec_load_t,
    pub cleanup: *mut kexec_cleanup_t,

    pub verify_sig: *mut kexec_verify_sig_t,

}

extern "C" {
    pub fn kexec_image_post_load_cleanup_default(image: *mut kimage) -> c_int;
}
//
// If kexec_buf.mem is set to this value, kexec_locate_mem_hole()
// will try to allocate free memory. Arch may overwrite it.
//

pub const KEXEC_BUF_MEM_UNKNOWN: c_int = 0;

//
// struct kexec_buf - parameters for finding a place for a buffer in memory
// @image:	kexec image in which memory to search.
// @buffer:	Contents which will be copied to the allocated memory.
// @bufsz:	Size of @buffer.
// @mem:	On return will have address of the buffer in memory.
// @memsz:	Size for the buffer in memory.
// @buf_align:	Minimum alignment needed.
// @buf_min:	The buffer can't be placed below this address.
// @buf_max:	The buffer can't be placed above this address.
// @cma:	CMA page if the buffer is backed by CMA.
// @top_down:	Allocate from top of memory.
// @random:	Place the buffer at a random position.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_buf {
    pub image: *mut kimage,
    pub buffer: *mut c_void,
    pub bufsz: c_ulong,
    pub mem: c_ulong,
    pub memsz: c_ulong,
    pub buf_align: c_ulong,
    pub buf_min: c_ulong,
    pub buf_max: c_ulong,
    pub cma: *mut page,
    pub top_down: bool,

    pub random: bool,

}

// temp_start = start + (end - start) / USHRT_MAX * i;

extern "C" {
    pub fn kexec_load_purgatory(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int;
}

extern "C" {
    pub fn kexec_image_probe_default(_arg: image, _arg: buf, _arg: buf_len) -> return;
}

extern "C" {
    pub fn kexec_image_post_load_cleanup_default(_arg: image) -> return;
}

extern "C" {
    pub fn kexec_kernel_verify_pe_sig(kernel: *const c_char, kernel_len: c_ulong) -> c_int;
}

extern "C" {
    pub fn kexec_add_buffer(kbuf: *mut kexec_buf) -> c_int;
}
extern "C" {
    pub fn kexec_locate_mem_hole(kbuf: *mut kexec_buf) -> c_int;
}

//
// arch_kexec_locate_mem_hole - Find free memory to place the segments.
// @kbuf:                       Parameters for the memory search.
//
// On success, kbuf->mem will have the start address of the memory region found.
//
// Return: 0 on success, negative errno on error.
//
extern "C" {
    pub fn kexec_locate_mem_hole(_arg: kbuf) -> return;
}

//
// arch_kexec_apply_relocations_add - apply relocations of type RELA
// @pi:		Purgatory to be relocated.
// @section:	Section relocations applying to.
// @relsec:	Section containing RELAs.
// @symtab:	Corresponding symtab.
//
// Return: 0 on success, negative errno on error.
//

//
// arch_kexec_apply_relocations - apply relocations of type REL
// @pi:		Purgatory to be relocated.
// @section:	Section relocations applying to.
// @relsec:	Section containing RELs.
// @symtab:	Corresponding symtab.
//
// Return: 0 on success, negative errno on error.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_elf_info {
//
// Where the ELF binary contents are kept.
// Memory managed by the user of the struct.
//
    pub buffer: *const c_char,
    pub ehdr: *const elfhdr,
    pub proghdrs: *const elf_phdr,
}

extern "C" {
    pub fn kexec_free_elf_info(elf_info: *mut kexec_elf_info);
}
extern "C" {
    pub fn kexec_elf_probe(buf: *const c_char, len: c_ulong) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage {
    pub head: kimage_entry_t,
    pub entry: *mut kimage_entry_t,
    pub last_entry: *mut kimage_entry_t,
    pub start: c_ulong,
    pub control_code_page: *mut page,
    pub swap_page: *mut page,
    pub /: *mut *mut *mut void vmcoreinfo_data_copy; / locates in the crash memory,
    pub nr_segments: c_ulong,
    pub segment: [kexec_segment; KEXEC_SEGMENT_MAX],
    pub segment_cma: [*mut page; KEXEC_SEGMENT_MAX],
    pub control_pages: list_head,
    pub dest_pages: list_head,
    pub unusable_pages: list_head,
// Address of next control page to allocate for crash kernels.
    pub control_page: c_ulong,
// Flags to indicate special processing
    pub 1: unsigned int type :,
pub const KEXEC_TYPE_DEFAULT: c_int = 0;
pub const KEXEC_TYPE_CRASH: c_int = 1;
    pub 1: unsigned int preserve_context :,
// If set, we are using file mode kexec syscall
    pub file_mode:1: c_uint,

// If set, it is safe to update kexec segments that are
// excluded from SHA calculation.
//
    pub hotplug_support:1: c_uint,

    pub no_cma:1: c_uint,

    pub arch: kimage_arch,

// Additional fields for file based kexec syscall
    pub kernel_buf: *mut c_void,
    pub kernel_buf_len: c_ulong,
    pub initrd_buf: *mut c_void,
    pub initrd_buf_len: c_ulong,
    pub cmdline_buf: *mut c_char,
    pub cmdline_buf_len: c_ulong,
// File operations provided by image loader
    pub fops: *const kexec_file_ops,
// Image loader handling the kernel can store a pointer here
    pub image_loader_data: *mut c_void,
// Information for loading purgatory
    pub purgatory_info: purgatory_info,
// Force carrying over the DTB from the current boot
    pub force_dtb: bool,

    pub hp_action: c_int,
    pub elfcorehdr_index: c_int,
    pub elfcorehdr_updated: bool,

// Virtual address of IMA measurement buffer for kexec syscall
    pub ima_buffer: *mut c_void,
    pub ima_buffer_addr: phys_addr_t,
    pub ima_buffer_size: usize,
    pub ima_segment_index: c_ulong,
    pub is_ima_segment_index_set: bool,

    pub scratch: *mut kexec_segment,
    pub fdt: phys_addr_t,
    pub kho: },
// Core ELF header buffer
    pub elf_headers: *mut c_void,
    pub elf_headers_sz: c_ulong,
    pub elf_load_addr: c_ulong,
// dm crypt keys buffer
    pub dm_crypt_keys_addr: c_ulong,
    pub dm_crypt_keys_sz: c_ulong,
}

// kexec interface functions
extern "C" {
    pub fn machine_kexec(image: *mut kimage);
}
extern "C" {
    pub fn machine_kexec_prepare(image: *mut kimage) -> c_int;
}
extern "C" {
    pub fn machine_kexec_cleanup(image: *mut kimage);
}
extern "C" {
    pub fn kernel_kexec() -> c_int;
}

extern "C" {
    pub fn kexec_load_permitted(kexec_image_type: c_int) -> bool;
}

// Macro flag: #define kexec_flush_icache_page(page)

// List of defined/legal kexec flags

// List of defined/legal kexec file flags

// flag to track if kexec reboot is in progress

extern "C" {
    pub fn page_to_pfn(_arg: page) -> return;
}

extern "C" {
    pub fn pfn_to_page(_arg: boot_pfn) -> return;
}

extern "C" {
    pub fn phys_to_boot_phys(long)addr): __pa((unsigned) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: boot_phys_to_phys(entry)) -> return;
}

extern "C" {
    pub fn kimage_unmap_segment(buffer: *mut c_void);
}

extern "C" {
    pub fn set_kexec_sig_enforced();
}

