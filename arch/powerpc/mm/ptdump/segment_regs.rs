//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ptdump/segment_regs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018, Christophe Leroy CS S.I.
// <christophe.leroy@c-s.fr>
//
// This dumps the content of Segment Registers
//

#[no_mangle]
unsafe extern "C" fn seg_show(m: *mut seq_file, i: c_int) {
    static void seg_show(struct seq_file *m, int i)
    {
    let mut val: u32 = mfsr(i << 28);
    seq_printf(m, "0x%01x0000000-0x%01xfffffff ", i, i);
    seq_printf(m, "Kern key %d ", (val >> 30) & 1);
    seq_printf(m, "User key %d ", (val >> 29) & 1);
    if (val & 0x80000000) {
    seq_printf(m, "Device 0x%03x", (val >> 20) & 0x1ff);
    seq_printf(m, "-0x%05x", val & 0xfffff);
    } else {
    if (val & 0x10000000)
    seq_puts(m, "No Exec ");
    seq_printf(m, "VSID 0x%06x", val & 0xffffff);
    }
    seq_puts(m, "\n");
    }
#[no_mangle]
unsafe extern "C" fn sr_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int sr_show(struct seq_file *m, void *v)
    {
    int i;
    seq_puts(m, "---[ User Segments ]---\n");
    for (i = 0; i < ALIGN(TASK_SIZE, SZ_256M) >> 28; i++)
    seg_show(m, i);
    seq_puts(m, "\n---[ Kernel Segments ]---\n");
    for (; i < 16; i++)
    seg_show(m, i);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sr);
#[no_mangle]
unsafe extern "C" fn sr_init() -> int __init {
    static int __init sr_init(void)
    {
    debugfs_create_file("segment_registers", 0400, arch_debugfs_dir,
    core::ptr::null_mut(), &sr_fops);
    return 0;
    }
    device_initcall(sr_init);
