//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/udbg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// (c) 2001, 2006 IBM Corporation.
//

extern "C" {
    pub fn void(c: *mut *mut udbg_putc)(char) -> extern;
}
extern "C" {
    pub fn void(_arg: *mut udbg_flush)(void) -> extern;
}
extern "C" {
    pub fn int(_arg: *mut udbg_getc)(void) -> extern;
}
extern "C" {
    pub fn int(_arg: *mut udbg_getc_poll)(void) -> extern;
}
extern "C" {
    pub fn udbg_puts(s: *const c_char);
}
extern "C" {
    pub fn udbg_write(s: *const c_char, n: c_int) -> c_int;
}
extern "C" {
    pub fn register_early_udbg_console();
}
extern "C" {
    pub fn udbg_progress(s: *mut c_char, hex: c_ushort);
}
extern "C" {
    pub fn udbg_uart_init_mmio(addr: *mut void __iomem, stride: c_uint) -> void __init;
}
extern "C" {
    pub fn udbg_uart_init_pio(port: c_ulong, stride: c_uint) -> void __init;
}
extern "C" {
    pub fn udbg_uart_setup(speed: c_uint, clock: c_uint) -> void __init;
}
extern "C" {
    pub fn udbg_probe_uart_speed(clock: c_uint) -> unsigned int __init;
}
extern "C" {
    pub fn udbg_scc_init(force_scc: c_int) -> void __init;
}
extern "C" {
    pub fn udbg_adb_init(force_btext: c_int) -> c_int;
}
extern "C" {
    pub fn udbg_adb_init_early();
}
extern "C" {
    pub fn udbg_early_init() -> void __init;
}
extern "C" {
    pub fn udbg_init_debug_lpar() -> void __init;
}
extern "C" {
    pub fn udbg_init_debug_lpar_hvsi() -> void __init;
}
extern "C" {
    pub fn udbg_init_pmac_realmode() -> void __init;
}
extern "C" {
    pub fn udbg_init_pas_realmode() -> void __init;
}
extern "C" {
    pub fn udbg_init_rtas_panel() -> void __init;
}
extern "C" {
    pub fn udbg_init_rtas_console() -> void __init;
}
extern "C" {
    pub fn udbg_init_btext() -> void __init;
}
extern "C" {
    pub fn udbg_init_44x_as1() -> void __init;
}
extern "C" {
    pub fn udbg_init_cpm() -> void __init;
}
extern "C" {
    pub fn udbg_init_usbgecko() -> void __init;
}
extern "C" {
    pub fn udbg_init_memcons() -> void __init;
}
extern "C" {
    pub fn udbg_init_ehv_bc() -> void __init;
}
extern "C" {
    pub fn udbg_init_ps3gelic() -> void __init;
}
extern "C" {
    pub fn udbg_init_debug_opal_raw() -> void __init;
}
extern "C" {
    pub fn udbg_init_debug_opal_hvsi() -> void __init;
}
extern "C" {
    pub fn udbg_init_debug_16550() -> void __init;
}

