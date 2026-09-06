//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/setup.h
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

extern "C" {
    pub fn ppc_printk_progress(s: *mut c_char, hex: c_ushort);
}
// Used in very early kernel initialization.
extern "C" {
    pub fn reloc_offset() -> c_ulong;
}
extern "C" {
    pub fn add_reloc_offset(long: unsigned) -> c_ulong;
}
extern "C" {
    pub fn reloc_got2(long: unsigned);
}

extern "C" {
    pub fn check_for_initrd();
}
extern "C" {
    pub fn mem_topology_setup();
}

extern "C" {
    pub fn initmem_init();
}

extern "C" {
    pub fn setup_panic();
}
pub const ARCH_PANIC_TIMEOUT: c_int = 180;

extern "C" {
    pub fn pseries_reloc_on_exception() -> bool;
}
extern "C" {
    pub fn pseries_enable_reloc_on_exc() -> bool;
}
extern "C" {
    pub fn pseries_disable_reloc_on_exc();
}
extern "C" {
    pub fn pseries_big_endian_exceptions();
}
extern "C" {
    pub fn pseries_little_endian_exceptions() -> void __init;
}

extern "C" {
    pub fn rfi_flush_enable(enable: bool);
}
// These are bit flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l1d_flush_type {
    L1D_FLUSH_NONE		= 0x1,
    L1D_FLUSH_FALLBACK	= 0x2,
    L1D_FLUSH_ORI		= 0x4,
    L1D_FLUSH_MTTRIG	= 0x8,
}

extern "C" {
    pub fn setup_rfi_flush(l1d_flush_type: enum, enable: bool);
}
extern "C" {
    pub fn setup_entry_flush(enable: bool);
}
extern "C" {
    pub fn setup_uaccess_flush(enable: bool);
}
extern "C" {
    pub fn do_rfi_flush_fixups(types: l1d_flush_type);
}

extern "C" {
    pub fn setup_barrier_nospec() -> void __init;
}

extern "C" {
    pub fn do_uaccess_flush_fixups(types: l1d_flush_type);
}
extern "C" {
    pub fn do_entry_flush_fixups(types: l1d_flush_type);
}
extern "C" {
    pub fn do_barrier_nospec_fixups(enable: bool);
}

extern "C" {
    pub fn do_barrier_nospec_fixups_range(enable: bool, start: *mut c_void, end: *mut c_void);
}

extern "C" {
    pub fn setup_spectre_v2() -> void __init;
}

extern "C" {
    pub fn do_btb_flush_fixups() -> void __init;
}

extern "C" {
    pub fn early_init(dt_ptr: c_ulong) -> unsigned long __init;
}
extern "C" {
    pub fn machine_init(dt_ptr: u64) -> void __init;
}

extern "C" {
    pub fn early_setup(dt_ptr: c_ulong) -> void __init;
}
extern "C" {
    pub fn early_setup_secondary();
}
// prom_init (OpenFirmware)

