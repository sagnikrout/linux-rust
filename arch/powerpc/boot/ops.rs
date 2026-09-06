//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/ops.h
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
// Global definition of all the bootwrapper operations.
//
// Author: Mark A. Greer <mgreer@mvista.com>
//
// 2006 (c) MontaVista Software, Inc.
//

pub const BOOT_COMMAND_LINE_SIZE: c_int = 2048;
pub const MAX_PATH_LEN: c_int = 256;

extern "C" {
    pub fn void(r3: *mut *mut kernel_entry_t)(unsigned long, r4: c_ulong, r5: *mut c_void) -> typedef;
}
// Platform specific operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_ops {
    pub (*fixups)(void): *mut c_void,
    pub ): *const *const void (image_hdr)(void,
    pub size): *mut *mut *mut void  (malloc)(unsigned long,
    pub ptr): *mut *mut void (free)(void,
    pub size): *mut *mut *mut *mut void  (realloc)(void ptr, unsigned long,
    pub (*exit)(void): *mut c_void,
    pub size): *mut *mut *mut void  (vmlinux_alloc)(unsigned long,
    pub vmlinux_addr): *mut *mut void (kentry)(unsigned long fdt_addr, void,
}

// Device Tree operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dt_ops {
    pub name): *const *const *const void  (finddevice)(char,
    pub buflen): c_int,
    pub buflen): *const *const void buf, int,
    pub phandle): *const *const int (del_node)(void,
    pub phandle): *const *const *const void (get_parent)(void,
// The node must not already exist.
    pub name): *const *const *const *const void (create_node)(void parent, char,
    pub proplen): *const *const char propval, int,
    pub compat): *const c_char,
    pub (*finalize)(void): *mut c_ulong,
    pub len): *const *const *const *const *const char (get_path)(void phandle, char buf, int,
}

// Console operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_ops {
    pub (*open)(void): *mut c_int,
    pub len): *const *const *const void (write)(char buf, int,
    pub getline_timeout): *mut *mut *mut void (edit_cmdline)(char buf, int len, unsigned int,
    pub (*close)(void): *mut c_void,
    pub data: *mut c_void,
}

// Serial console operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_console_data {
    pub (*open)(void): *mut c_int,
    pub c): *mut *mut void (putc)(unsigned char,
    pub (*getc)(void): *mut c_uchar,
    pub (*tstc)(void): *mut u8,
    pub (*close)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_info {
    pub promptr: *mut c_void,
    pub initrd_size: unsigned long initrd_addr,,
    pub cmdline: *mut c_char,
    pub cmdline_len: c_int,
}

extern "C" {
    pub fn start();
}
extern "C" {
    pub fn fdt_init(blob: *mut c_void);
}
extern "C" {
    pub fn serial_console_init() -> c_int;
}
extern "C" {
    pub fn ns16550_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int;
}
extern "C" {
    pub fn cpm_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int;
}
extern "C" {
    pub fn mpc5200_psc_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int;
}
extern "C" {
    pub fn opal_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int;
}
extern "C" {
    pub fn flush_cache(: *mut c_void, long: unsigned);
}
extern "C" {
    pub fn dt_xlate_reg(node: *mut c_void, res: c_int, addr: *mut c_ulong, size: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn dt_xlate_addr(node: *mut c_void, buf: *mut u32, buflen: c_int, xlated_addr: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn dt_is_compatible(node: *mut c_void, compat: *const c_char) -> c_int;
}
extern "C" {
    pub fn dt_get_reg_format(node: *mut c_void, naddr: *mut u32, nsize: *mut u32);
}
extern "C" {
    pub fn dt_get_virtual_reg(node: *mut c_void, addr: *mut c_void, nres: c_int) -> c_int;
}

extern "C" {
    pub fn find_node_by_prop_value_str(_arg: prev, _arg: "device_type", _arg: type) -> return;
}
extern "C" {
    pub fn finddevice(_arg: path) -> return;
}
extern "C" {
    pub fn dt_fixup_memory(start: u64, size: u64);
}
extern "C" {
    pub fn dt_fixup_cpu_clocks(cpufreq: u32, tbfreq: u32, busfreq: u32);
}
extern "C" {
    pub fn dt_fixup_clock(path: *const c_char, freq: u32);
}
extern "C" {
    pub fn dt_fixup_mac_address_by_alias(alias: *const c_char, addr: *const u8);
}
extern "C" {
    pub fn dt_fixup_mac_address(index: u32, addr: *const u8);
}
extern "C" {
    pub fn __dt_fixup_mac_addresses(startindex: u32, ...);
}

extern "C" {
    pub fn udelay(delay: c_long);
}
