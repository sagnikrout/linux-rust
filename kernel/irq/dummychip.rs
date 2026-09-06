//! Automatically rewritten from C to Rust
//! Source: kernel/irq/dummychip.c
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
// Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
// Copyright (C) 2005-2006, Thomas Gleixner, Russell King
//
// This file contains the dummy interrupt chip implementation
//

//
// What should we do if we get a hw irq event on an illegal vector?
// Each architecture has to answer this themselves.
//
#[no_mangle]
unsafe extern "C" fn ack_bad(data: *mut irq_data) {
    static void ack_bad(struct irq_data *data)
    {
    struct irq_desc *desc = irq_data_to_desc(data);
    print_irq_desc(data.irq, desc);
    ack_bad_irq(data.irq);
    }
//
// NOP functions
//
    static void noop(struct irq_data *data) { }
#[no_mangle]
unsafe extern "C" fn noop_ret(data: *mut irq_data) -> c_uint {
    static unsigned int noop_ret(struct irq_data *data)
    {
    return 0;
    }
//
// Generic no controller implementation
//
    struct irq_chip no_irq_chip = {
    .name		= "none",
    .irq_startup	= noop_ret,
    .irq_shutdown	= noop,
    .irq_enable	= noop,
    .irq_disable	= noop,
    .irq_ack	= ack_bad,
    .flags		= IRQCHIP_SKIP_SET_WAKE,
    };
//
// Generic dummy implementation which can be used for
// real dumb interrupt sources
//
    struct irq_chip dummy_irq_chip = {
    .name		= "dummy",
    .irq_startup	= noop_ret,
    .irq_shutdown	= noop,
    .irq_enable	= noop,
    .irq_disable	= noop,
    .irq_ack	= noop,
    .irq_mask	= noop,
    .irq_unmask	= noop,
    .flags		= IRQCHIP_SKIP_SET_WAKE,
    };
    EXPORT_SYMBOL_GPL(dummy_irq_chip);
