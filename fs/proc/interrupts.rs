//! Automatically rewritten from C to Rust
//! Source: fs/proc/interrupts.c
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
// /proc/interrupts
//
    static void *int_seq_start(struct seq_file *f, loff_t *pos)
    {
    return *pos <= irq_get_nr_irqs() ? pos : core::ptr::null_mut();
    }
    static void *int_seq_next(struct seq_file *f, void *v, loff_t *pos)
    {
    (*pos)++;
    if (*pos > irq_get_nr_irqs())
    return core::ptr::null_mut();
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn int_seq_stop(f: *mut seq_file, v: *mut c_void) {
    static void int_seq_stop(struct seq_file *f, void *v)
    {
// Nothing to do
    }
    static const struct seq_operations int_seq_ops = {
    .start = int_seq_start,
    .next  = int_seq_next,
    .stop  = int_seq_stop,
    .show  = show_interrupts
    };
#[no_mangle]
unsafe extern "C" fn proc_interrupts_init() -> int __init {
    static int __init proc_interrupts_init(void)
    {
    proc_create_seq("interrupts", 0, core::ptr::null_mut(), &int_seq_ops);
    return 0;
    }
    fs_initcall(proc_interrupts_init);
