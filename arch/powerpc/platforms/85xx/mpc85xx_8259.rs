//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mpc85xx_8259.c
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
// MPC85xx 8259 functions for DS Board Setup
//
// Author Xianghua Xiao (x.xiao@freescale.com)
// Roy Zang <tie-fei.zang@freescale.com>
// - Add PCI/PCI Express support
// Copyright 2007 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn mpc85xx_8259_cascade(desc: *mut irq_desc) {
    static void mpc85xx_8259_cascade(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut cascade_irq: c_uint = i8259_irq();
    if (cascade_irq)
    generic_handle_irq(cascade_irq);
    chip.irq_eoi(&desc.irq_data);
    }
#[no_mangle]
pub unsafe extern "C" fn mpc85xx_8259_init() -> void __init {
    void __init mpc85xx_8259_init(void)
    {
    struct device_node *np;
    struct device_node *cascade_node = core::ptr::null_mut();
    int cascade_irq;
// Initialize the i8259 controller
    for_each_node_by_type(np, "interrupt-controller") {
    if (of_device_is_compatible(np, "chrp,iic")) {
    cascade_node = np;
    break;
    }
    }
    if (cascade_node == core::ptr::null_mut()) {
    pr_debug("i8259: Could not find i8259 PIC\n");
    return;
    }
    cascade_irq = irq_of_parse_and_map(cascade_node, 0);
    if (!cascade_irq) {
    pr_err("i8259: Failed to map cascade interrupt\n");
    return;
    }
    pr_debug("i8259: cascade mapped to irq %d\n", cascade_irq);
    i8259_init(cascade_node, 0);
    of_node_put(cascade_node);
    irq_set_chained_handler(cascade_irq, mpc85xx_8259_cascade);
    }
