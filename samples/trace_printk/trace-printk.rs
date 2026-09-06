//! Automatically rewritten from C to Rust
//! Source: samples/trace_printk/trace-printk.c
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


// SPDX-License-Identifier: GPL-2.0-only

// Must not be static to force gcc to consider these non constant
    char *trace_printk_test_global_str =
    "This is a dynamic string that will use trace_puts\n";
    char *trace_printk_test_global_str_irq =
    "(irq) This is a dynamic string that will use trace_puts\n";
    char *trace_printk_test_global_str_fmt =
    "%sThis is a %s that will use trace_printk\n";
    static struct irq_work irqwork;
#[no_mangle]
unsafe extern "C" fn trace_printk_irq_work(work: *mut irq_work) {
    static void trace_printk_irq_work(struct irq_work *work)
    {
    trace_printk("(irq) This is a static string that will use trace_bputs\n");
    trace_printk(trace_printk_test_global_str_irq);
    trace_printk("(irq) This is a %s that will use trace_bprintk()\n",
    "static string");
    trace_printk(trace_printk_test_global_str_fmt,
    "(irq) ", "dynamic string");
    }
#[no_mangle]
unsafe extern "C" fn trace_printk_init() -> int __init {
    static int __init trace_printk_init(void)
    {
    init_irq_work(&irqwork, trace_printk_irq_work);
    trace_printk("This is a static string that will use trace_bputs\n");
    trace_printk(trace_printk_test_global_str);
// Kick off printing in irq context
    irq_work_queue(&irqwork);
    irq_work_sync(&irqwork);
    trace_printk("This is a %s that will use trace_bprintk()\n",
    "static string");
    trace_printk(trace_printk_test_global_str_fmt, "", "dynamic string");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_printk_exit() -> void __exit {
    static void __exit trace_printk_exit(void)
    {
    }
    module_init(trace_printk_init);
    module_exit(trace_printk_exit);
    MODULE_AUTHOR("Steven Rostedt");
    MODULE_DESCRIPTION("trace-printk");
    MODULE_LICENSE("GPL");
