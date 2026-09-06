//! Automatically rewritten from C to Rust
//! Source: drivers/misc/dummy-irq.c
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
//
// Dummy IRQ handler driver.
//
// This module only registers itself as a handler that is specified to it
// by the 'irq' parameter.
//
// The sole purpose of this module is to help with debugging of systems on
// which spurious IRQs would happen on disabled IRQ vector.
//
// Copyright (C) 2013 Jiri Kosina
//

    let mut irq: static int = -1;
#[no_mangle]
unsafe extern "C" fn dummy_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dummy_interrupt(int irq, void *dev_id)
    {
    let mut count: static int = 0;
    if (count == 0) {
    printk(KERN_INFO "dummy-irq: interrupt occurred on IRQ %d\n",
    irq);
    count++;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn dummy_irq_init() -> int __init {
    static int __init dummy_irq_init(void)
    {
    if (irq < 0) {
    printk(KERN_ERR "dummy-irq: no IRQ given.  Use irq=N\n");
    return -EIO;
    }
    if (request_irq(irq, &dummy_interrupt, IRQF_SHARED, "dummy_irq", &irq)) {
    printk(KERN_ERR "dummy-irq: cannot register IRQ %d\n", irq);
    return -EIO;
    }
    printk(KERN_INFO "dummy-irq: registered for IRQ %d\n", irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_irq_exit() -> void __exit {
    static void __exit dummy_irq_exit(void)
    {
    printk(KERN_INFO "dummy-irq unloaded\n");
    free_irq(irq, &irq);
    }
    module_init(dummy_irq_init);
    module_exit(dummy_irq_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jiri Kosina");
    module_param_hw(irq, uint, irq, 0444);
    MODULE_PARM_DESC(irq, "The IRQ to register for");
    MODULE_DESCRIPTION("Dummy IRQ handler driver");
