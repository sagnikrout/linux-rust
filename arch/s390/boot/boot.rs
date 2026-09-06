//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/boot/boot.h
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

pub const IPL_START: c_uint = 0x200;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmlinux_info {
    pub entry: c_ulong,
    pub /: *mut *mut unsigned long image_size; / does not include .bss,
    pub /: *mut *mut unsigned long bss_size; / uncompressed image .bss size,
    pub bootdata_off: c_ulong,
    pub bootdata_size: c_ulong,
    pub bootdata_preserved_off: c_ulong,
    pub bootdata_preserved_size: c_ulong,
    pub got_start: c_ulong,
    pub got_end: c_ulong,
    pub amode31_size: c_ulong,
    pub init_mm_off: c_ulong,
    pub swapper_pg_dir_off: c_ulong,
    pub invalid_pg_dir_off: c_ulong,
    pub alt_instructions: c_ulong,
    pub alt_instructions_end: c_ulong,

    pub stack_prot_start: c_ulong,
    pub stack_prot_end: c_ulong,

    pub kasan_early_shadow_page_off: c_ulong,
    pub kasan_early_shadow_pte_off: c_ulong,
    pub kasan_early_shadow_pmd_off: c_ulong,
    pub kasan_early_shadow_pud_off: c_ulong,
    pub kasan_early_shadow_p4d_off: c_ulong,

}

extern "C" {
    pub fn startup_kernel();
}
extern "C" {
    pub fn detect_max_physmem_end() -> c_ulong;
}
extern "C" {
    pub fn detect_physmem_online_ranges(max_physmem_end: c_ulong);
}
extern "C" {
    pub fn physmem_set_usable_limit(limit: c_ulong);
}
extern "C" {
    pub fn physmem_reserve(type: reserved_range_type, addr: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn physmem_free(type: reserved_range_type);
}
// for continuous/multiple allocations per type
// for single allocations, 1 per type
extern "C" {
    pub fn get_physmem_alloc_pos() -> c_ulong;
}
extern "C" {
    pub fn dump_physmem_reserved();
}
extern "C" {
    pub fn is_ipl_block_dump() -> bool;
}
extern "C" {
    pub fn store_ipl_parmblock();
}
extern "C" {
    pub fn read_ipl_report() -> c_int;
}
extern "C" {
    pub fn save_ipl_cert_comp_list();
}
extern "C" {
    pub fn setup_boot_command_line();
}
extern "C" {
    pub fn parse_boot_command_line();
}
extern "C" {
    pub fn verify_facilities();
}
extern "C" {
    pub fn print_missing_facilities();
}
extern "C" {
    pub fn sclp_early_setup_buffer();
}
extern "C" {
    pub fn alt_debug_setup(str: *mut c_char);
}
extern "C" {
    pub fn do_pgm_check(regs: *mut pt_regs);
}
extern "C" {
    pub fn setup_vmem(kernel_start: c_ulong, kernel_end: c_ulong, asce_limit: c_ulong);
}
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) boot_printk(char, ...) -> c_int;
}
extern "C" {
    pub fn print_stacktrace(sp: c_ulong);
}
extern "C" {
    pub fn error(m: *mut c_char);
}
extern "C" {
    pub fn get_random(limit: c_ulong, value: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn boot_rb_dump();
}
extern "C" {
    pub fn jump_to_kernel(psw: *mut psw_t) -> void __noreturn;
}

// Symbols defined by linker scripts

