//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/opal.c
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
// Copyright (c) 2016 IBM Corporation.
//

// Global OPAL struct used by opal-call.S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal {
    pub base: u64,
    pub entry: u64,
    pub opal: },
    pub opal_con_id: static u32,
// see opal-wrappers.S
    pub buffer): *const *const int64_t opal_console_write(int64_t term_number, u64 length, u8,
    pub buffer): *mut *mut int64_t opal_console_read(int64_t term_number, uint64_t length, u8,
    pub length): *mut int64_t opal_console_write_buffer_space(uint64_t term_number, uint64_t,
    pub term_number): int64_t opal_console_flush(uint64_t,
    pub outstanding_event_mask): *mut int64_t opal_poll_events(uint64_t,
    pub vmlinux_addr): *mut void opal_kentry(unsigned long fdt_addr, void,
#[no_mangle]
unsafe extern "C" fn opal_con_open() -> c_int {
    static int opal_con_open(void)
    {
//
// When OPAL loads the boot kernel it stashes the OPAL base and entry
// address in r8 and r9 so the kernel can use the OPAL console
// before unflattening the devicetree. While executing the wrapper will
// probably trash r8 and r9 so this kentry hook restores them before
// entering the decompressed kernel.
//
    pub opal_kentry: platform_ops.kentry =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn opal_con_putc(c: c_uchar) {
    static void opal_con_putc(unsigned char c)
    {
    pub rc: i64,
    pub len: uint64_t olen,,
    do {
    pub &olen): rc = opal_console_write_buffer_space(opal_con_id,,
    pub be64_to_cpu(olen): len =,
    if (rc)
    pub 1): } while (len <,
    pub cpu_to_be64(1): olen =,
    pub &c): opal_console_write(opal_con_id, &olen,,
    }
#[no_mangle]
unsafe extern "C" fn opal_con_close() {
    static void opal_con_close(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn opal_init() {
    static void opal_init(void)
    {
    pub opal_node: *mut c_void,
    pub finddevice("/ibm,opal"): opal_node =,
    if (!opal_node)
    if (getprop(opal_node, "opal-base-address", &opal.base, sizeof(u64)) < 0)
    pub be64_to_cpu(opal.base): opal.base =,
    if (getprop(opal_node, "opal-entry-address", &opal.entry, sizeof(u64)) < 0)
    pub be64_to_cpu(opal.entry): opal.entry =,
    }
#[no_mangle]
pub unsafe extern "C" fn opal_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int {
    int opal_console_init(void *devp, struct serial_console_data *scdp)
    {
    if (devp) {
    pub sizeof(u32)): int n = getprop(devp, "reg", &opal_con_id,,
    if (n != sizeof(u32))
    pub -1: return,
    pub be32_to_cpu(opal_con_id): opal_con_id =,
    } else
    pub 0: opal_con_id =,
    pub opal_con_open: scdp->open =,
    pub opal_con_putc: scdp->putc =,
    pub opal_con_close: scdp->close =,
    pub 0: return,
    }
