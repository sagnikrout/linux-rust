//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mpspec.h
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

//
// Summit or generic (i.e. installer) kernels need lots of bus entries.
// Maximum 256 PCI busses, plus 1 ISA bus in each of 4 cabinets.
//

pub const MAX_IRQ_SOURCES: c_int = 256;

pub const MAX_MP_BUSSES: c_int = 256;
// Each PCI slot may be a combo card with its own bus.  4 IRQ pins per slot.

extern "C" {
    pub fn DECLARE_BITMAP(_arg: mp_bus_not_pci, _arg: MAX_MP_BUSSES) -> extern;
}

extern "C" {
    pub fn e820__memblock_alloc_reserved_mpc_new();
}
extern "C" {
    pub fn mpparse_find_mptable();
}
extern "C" {
    pub fn mpparse_parse_early_smp_config();
}
extern "C" {
    pub fn mpparse_parse_smp_config();
}

pub const enable_update_mptable: c_int = 0;

extern "C" {
    pub fn DECLARE_BITMAP(_arg: phys_cpu_present_map, _arg: MAX_LOCAL_APIC) -> extern;
}
