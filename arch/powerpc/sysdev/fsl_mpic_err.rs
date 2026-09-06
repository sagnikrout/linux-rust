//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_mpic_err.c
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
// Copyright (C) 2012 Freescale Semiconductor, Inc.
//
// Author: Varun Sethi <varun.sethi@freescale.com>
//

pub const MPIC_ERR_INT_BASE: c_uint = 0x3900;
pub const MPIC_ERR_INT_EISR: c_uint = 0x0000;
pub const MPIC_ERR_INT_EIMR: c_uint = 0x0010;
#[no_mangle]
pub unsafe extern "C" fn mpic_fsl_err_read(base: *mut u32 __iomem, err_reg: c_uint) -> u32 {
    static inline u32 mpic_fsl_err_read(u32 __iomem *base, unsigned int err_reg)
    {
    return in_be32(base + (err_reg >> 2));
    }
#[no_mangle]
pub unsafe extern "C" fn mpic_fsl_err_write(base: *mut u32 __iomem, value: u32) {
    static inline void mpic_fsl_err_write(u32 __iomem *base, u32 value)
    {
    out_be32(base + (MPIC_ERR_INT_EIMR >> 2), value);
    }
#[no_mangle]
unsafe extern "C" fn fsl_mpic_mask_err(d: *mut irq_data) {
    static void fsl_mpic_mask_err(struct irq_data *d)
    {
    u32 eimr;
    struct mpic *mpic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = virq_to_hw(d.irq) - mpic.err_int_vecs[0];
    eimr = mpic_fsl_err_read(mpic.err_regs, MPIC_ERR_INT_EIMR);
    eimr |= (1 << (31 - src));
    mpic_fsl_err_write(mpic.err_regs, eimr);
    }
#[no_mangle]
unsafe extern "C" fn fsl_mpic_unmask_err(d: *mut irq_data) {
    static void fsl_mpic_unmask_err(struct irq_data *d)
    {
    u32 eimr;
    struct mpic *mpic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = virq_to_hw(d.irq) - mpic.err_int_vecs[0];
    eimr = mpic_fsl_err_read(mpic.err_regs, MPIC_ERR_INT_EIMR);
    eimr &= ~(1 << (31 - src));
    mpic_fsl_err_write(mpic.err_regs, eimr);
    }
    static struct irq_chip fsl_mpic_err_chip = {
    .irq_disable	= fsl_mpic_mask_err,
    .irq_mask	= fsl_mpic_mask_err,
    .irq_unmask	= fsl_mpic_unmask_err,
    };
#[no_mangle]
pub unsafe extern "C" fn mpic_setup_error_int(mpic: *mut mpic, intvec: c_int) -> int __init {
    int __init mpic_setup_error_int(struct mpic *mpic, int intvec)
    {
    int i;
    mpic.err_regs = ioremap(mpic.paddr + MPIC_ERR_INT_BASE, 0x1000);
    if (!mpic.err_regs) {
    pr_err("could not map mpic error registers\n");
    return -ENOMEM;
    }
    mpic.hc_err = fsl_mpic_err_chip;
    mpic.hc_err.name = mpic.name;
    mpic.flags |= MPIC_FSL_HAS_EIMR;
// allocate interrupt vectors for error interrupts
    for (i = MPIC_MAX_ERR - 1; i >= 0; i--)
    mpic.err_int_vecs[i] = intvec--;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mpic_map_error_int(mpic: *mut mpic, virq: c_uint, hw: irq_hw_number_t) -> c_int {
    int mpic_map_error_int(struct mpic *mpic, unsigned int virq, irq_hw_number_t  hw)
    {
    if ((mpic.flags & MPIC_FSL_HAS_EIMR) &&
    (hw >= mpic.err_int_vecs[0] &&
    hw <= mpic.err_int_vecs[MPIC_MAX_ERR - 1])) {
    WARN_ON(mpic.flags & MPIC_SECONDARY);
    pr_debug("mpic: mapping as Error Interrupt\n");
    irq_set_chip_data(virq, mpic);
    irq_set_chip_and_handler(virq, &mpic.hc_err,
    handle_level_irq);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_error_int_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_error_int_handler(int irq, void *data)
    {
    struct mpic *mpic = (struct mpic *) data;
    u32 eisr, eimr;
    int errint;
    eisr = mpic_fsl_err_read(mpic.err_regs, MPIC_ERR_INT_EISR);
    eimr = mpic_fsl_err_read(mpic.err_regs, MPIC_ERR_INT_EIMR);
    if (!(eisr & ~eimr))
    return IRQ_NONE;
    while (eisr) {
    int ret;
    errint = __builtin_clz(eisr);
    ret = generic_handle_domain_irq(mpic.irqhost,
    mpic.err_int_vecs[errint]);
    if (WARN_ON(ret)) {
    eimr |=  1 << (31 - errint);
    mpic_fsl_err_write(mpic.err_regs, eimr);
    }
    eisr &= ~(1 << (31 - errint));
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn mpic_err_int_init(mpic: *mut mpic, irqnum: irq_hw_number_t) -> void __init {
    void __init mpic_err_int_init(struct mpic *mpic, irq_hw_number_t irqnum)
    {
    unsigned int virq;
    int ret;
    virq = irq_create_mapping(mpic.irqhost, irqnum);
    if (!virq) {
    pr_err("Error interrupt setup failed\n");
    return;
    }
// Mask all error interrupts
    mpic_fsl_err_write(mpic.err_regs, ~0);
    ret = request_irq(virq, fsl_error_int_handler, IRQF_NO_THREAD,
    "mpic-error-int", mpic);
    if (ret)
    pr_err("Failed to register error interrupt handler\n");
    }
