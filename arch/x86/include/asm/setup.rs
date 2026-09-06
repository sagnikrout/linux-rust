//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/setup.h
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

pub const COMMAND_LINE_SIZE: c_int = 2048;

//
// Reserved space for vmalloc and iomap - defined in asm/page.h
//

pub const OLD_CL_MAGIC: c_uint = 0xA33F;
pub const OLD_CL_ADDRESS: c_uint = 0x020	/* Relative to real mode data */;
pub const NEW_CL_POINTER: c_uint = 0x228	/* Relative to real mode data */;

// Interrupt control for vSMPowered x86_64 systems

extern "C" {
    pub fn vsmp_init();
}

extern "C" {
    pub fn setup_bios_corruption_check();
}
extern "C" {
    pub fn early_platform_quirks();
}
extern "C" {
    pub fn reserve_standard_io_resources();
}
extern "C" {
    pub fn i386_reserve_resources();
}
extern "C" {
    pub fn __startup_64(p2v_offset: c_ulong, bp: *mut boot_params) -> c_ulong;
}
extern "C" {
    pub fn startup_64_setup_gdt_idt();
}
extern "C" {
    pub fn startup_64_load_idt(vc_handler: *mut c_void);
}
extern "C" {
    pub fn __pi_startup_64_load_idt(vc_handler: *mut c_void);
}
extern "C" {
    pub fn early_setup_idt();
}
extern "C" {
    pub fn do_early_exception(regs: *mut pt_regs, trapnr: c_int) -> void __init;
}

extern "C" {
    pub fn x86_intel_mid_early_setup();
}

extern "C" {
    pub fn x86_ce4100_early_setup();
}

//
// This is set up by the setup-routine at boot-time
//
// Apply no randomization if KASLR was disabled at boot or if KASAN
// is enabled. KASAN shadow mappings rely on regions being PGD aligned.
//
extern "C" {
    pub fn kaslr_enabled(!IS_ENABLED(CONFIG_KASAN: ) &&) -> return;
}
//
// Do NOT EVER look at the BIOS memory size location.
// It does not work on many machines.
//

// exceedingly early brk-like allocator
//
// Reserve space in the .brk section, which is a block of memory from which the
// caller is allowed to allocate very early (before even memblock is available)
// by calling extend_brk().  All allocated memory will be eventually converted
// to memblock.  Any leftover unallocated memory will be freed.
//
// The size is in bytes.
//

extern "C" {
    pub fn probe_roms();
}
extern "C" {
    pub fn clear_bss();
}

extern "C" {
    pub fn i386_start_kernel() -> asmlinkage void __init __noreturn;
}
extern "C" {
    pub fn mk_early_pgtbl_32() -> void __init;
}

extern "C" {
    pub fn x86_64_start_kernel(real_mode: *mut c_char) -> asmlinkage void __init __noreturn;
}
extern "C" {
    pub fn x86_64_start_reservations(real_mode_data: *mut c_char) -> asmlinkage void __init __noreturn;
}

pub const builtin_cmdline_added: c_int = 0;

