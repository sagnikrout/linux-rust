//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/max8998-irq.c
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
// Interrupt controller support for MAX8998
//
// Copyright (C) 2010 Samsung Electronics Co.Ltd
// Author: Joonyoung Shim <jy0922.shim@samsung.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8998_irq_data {
    pub reg: c_int,
    pub mask: c_int,
}

    static struct max8998_irq_data max8998_irqs[] = {
    [MAX8998_IRQ_DCINF] = {
    .reg = 1,
    .mask = MAX8998_IRQ_DCINF_MASK,
    },
    [MAX8998_IRQ_DCINR] = {
    .reg = 1,
    .mask = MAX8998_IRQ_DCINR_MASK,
    },
    [MAX8998_IRQ_JIGF] = {
    .reg = 1,
    .mask = MAX8998_IRQ_JIGF_MASK,
    },
    [MAX8998_IRQ_JIGR] = {
    .reg = 1,
    .mask = MAX8998_IRQ_JIGR_MASK,
    },
    [MAX8998_IRQ_PWRONF] = {
    .reg = 1,
    .mask = MAX8998_IRQ_PWRONF_MASK,
    },
    [MAX8998_IRQ_PWRONR] = {
    .reg = 1,
    .mask = MAX8998_IRQ_PWRONR_MASK,
    },
    [MAX8998_IRQ_WTSREVNT] = {
    .reg = 2,
    .mask = MAX8998_IRQ_WTSREVNT_MASK,
    },
    [MAX8998_IRQ_SMPLEVNT] = {
    .reg = 2,
    .mask = MAX8998_IRQ_SMPLEVNT_MASK,
    },
    [MAX8998_IRQ_ALARM1] = {
    .reg = 2,
    .mask = MAX8998_IRQ_ALARM1_MASK,
    },
    [MAX8998_IRQ_ALARM0] = {
    .reg = 2,
    .mask = MAX8998_IRQ_ALARM0_MASK,
    },
    [MAX8998_IRQ_ONKEY1S] = {
    .reg = 3,
    .mask = MAX8998_IRQ_ONKEY1S_MASK,
    },
    [MAX8998_IRQ_TOPOFFR] = {
    .reg = 3,
    .mask = MAX8998_IRQ_TOPOFFR_MASK,
    },
    [MAX8998_IRQ_DCINOVPR] = {
    .reg = 3,
    .mask = MAX8998_IRQ_DCINOVPR_MASK,
    },
    [MAX8998_IRQ_CHGRSTF] = {
    .reg = 3,
    .mask = MAX8998_IRQ_CHGRSTF_MASK,
    },
    [MAX8998_IRQ_DONER] = {
    .reg = 3,
    .mask = MAX8998_IRQ_DONER_MASK,
    },
    [MAX8998_IRQ_CHGFAULT] = {
    .reg = 3,
    .mask = MAX8998_IRQ_CHGFAULT_MASK,
    },
    [MAX8998_IRQ_LOBAT1] = {
    .reg = 4,
    .mask = MAX8998_IRQ_LOBAT1_MASK,
    },
    [MAX8998_IRQ_LOBAT2] = {
    .reg = 4,
    .mask = MAX8998_IRQ_LOBAT2_MASK,
    },
    };
    static inline struct max8998_irq_data *
    irq_to_max8998_irq(struct max8998_dev *max8998, struct irq_data *data)
    {
    return &max8998_irqs[data.hwirq];
    }
#[no_mangle]
unsafe extern "C" fn max8998_irq_lock(data: *mut irq_data) {
    static void max8998_irq_lock(struct irq_data *data)
    {
    struct max8998_dev *max8998 = irq_data_get_irq_chip_data(data);
    mutex_lock(&max8998.irqlock);
    }
#[no_mangle]
unsafe extern "C" fn max8998_irq_sync_unlock(data: *mut irq_data) {
    static void max8998_irq_sync_unlock(struct irq_data *data)
    {
    struct max8998_dev *max8998 = irq_data_get_irq_chip_data(data);
    int i;
    for (i = 0; i < ARRAY_SIZE(max8998.irq_masks_cur); i++) {
//
// If there's been a change in the mask write it back
// to the hardware.
//
    if (max8998.irq_masks_cur[i] != max8998.irq_masks_cache[i]) {
    max8998.irq_masks_cache[i] = max8998.irq_masks_cur[i];
    max8998_write_reg(max8998.i2c, MAX8998_REG_IRQM1 + i,
    max8998.irq_masks_cur[i]);
    }
    }
    mutex_unlock(&max8998.irqlock);
    }
#[no_mangle]
unsafe extern "C" fn max8998_irq_unmask(data: *mut irq_data) {
    static void max8998_irq_unmask(struct irq_data *data)
    {
    struct max8998_dev *max8998 = irq_data_get_irq_chip_data(data);
    struct max8998_irq_data *irq_data = irq_to_max8998_irq(max8998, data);
    max8998.irq_masks_cur[irq_data.reg - 1] &= ~irq_data.mask;
    }
#[no_mangle]
unsafe extern "C" fn max8998_irq_mask(data: *mut irq_data) {
    static void max8998_irq_mask(struct irq_data *data)
    {
    struct max8998_dev *max8998 = irq_data_get_irq_chip_data(data);
    struct max8998_irq_data *irq_data = irq_to_max8998_irq(max8998, data);
    max8998.irq_masks_cur[irq_data.reg - 1] |= irq_data.mask;
    }
    static struct irq_chip max8998_irq_chip = {
    .name = "max8998",
    .irq_bus_lock = max8998_irq_lock,
    .irq_bus_sync_unlock = max8998_irq_sync_unlock,
    .irq_mask = max8998_irq_mask,
    .irq_unmask = max8998_irq_unmask,
    };
#[no_mangle]
unsafe extern "C" fn max8998_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max8998_irq_thread(int irq, void *data)
    {
    struct max8998_dev *max8998 = data;
    u8 irq_reg[MAX8998_NUM_IRQ_REGS];
    int ret;
    int i;
    ret = max8998_bulk_read(max8998.i2c, MAX8998_REG_IRQ1,
    MAX8998_NUM_IRQ_REGS, irq_reg);
    if (ret < 0) {
    dev_err(max8998.dev, "Failed to read interrupt register: %d\n",
    ret);
    return IRQ_NONE;
    }
// Apply masking
    for (i = 0; i < MAX8998_NUM_IRQ_REGS; i++)
    irq_reg[i] &= ~max8998.irq_masks_cur[i];
// Report
    for (i = 0; i < MAX8998_IRQ_NR; i++) {
    if (irq_reg[max8998_irqs[i].reg - 1] & max8998_irqs[i].mask) {
    irq = irq_find_mapping(max8998.irq_domain, i);
    if (WARN_ON(!irq)) {
    disable_irq_nosync(max8998.irq);
    return IRQ_NONE;
    }
    handle_nested_irq(irq);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn max8998_irq_resume(max8998: *mut max8998_dev) -> c_int {
    int max8998_irq_resume(struct max8998_dev *max8998)
    {
    if (max8998.irq && max8998.irq_domain)
    max8998_irq_thread(max8998.irq, max8998);
    return 0;
    }
    static int max8998_irq_domain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    struct max8997_dev *max8998 = d.host_data;
    irq_set_chip_data(irq, max8998);
    irq_set_chip_and_handler(irq, &max8998_irq_chip, handle_edge_irq);
    irq_set_nested_thread(irq, 1);
    irq_set_noprobe(irq);
    return 0;
    }
    static const struct irq_domain_ops max8998_irq_domain_ops = {
    .map = max8998_irq_domain_map,
    };
#[no_mangle]
pub unsafe extern "C" fn max8998_irq_init(max8998: *mut max8998_dev) -> c_int {
    int max8998_irq_init(struct max8998_dev *max8998)
    {
    int i;
    int ret;
    struct irq_domain *domain;
    if (!max8998.irq) {
    dev_warn(max8998.dev,
    "No interrupt specified, no interrupts\n");
    return 0;
    }
    mutex_init(&max8998.irqlock);
// Mask the individual interrupt sources
    for (i = 0; i < MAX8998_NUM_IRQ_REGS; i++) {
    max8998.irq_masks_cur[i] = 0xff;
    max8998.irq_masks_cache[i] = 0xff;
    max8998_write_reg(max8998.i2c, MAX8998_REG_IRQM1 + i, 0xff);
    }
    max8998_write_reg(max8998.i2c, MAX8998_REG_STATUSM1, 0xff);
    max8998_write_reg(max8998.i2c, MAX8998_REG_STATUSM2, 0xff);
    domain = irq_domain_create_simple(core::ptr::null_mut(), MAX8998_IRQ_NR,
    max8998.irq_base, &max8998_irq_domain_ops, max8998);
    if (!domain) {
    dev_err(max8998.dev, "could not create irq domain\n");
    return -ENODEV;
    }
    max8998.irq_domain = domain;
    ret = request_threaded_irq(max8998.irq, core::ptr::null_mut(), max8998_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "max8998-irq", max8998);
    if (ret) {
    dev_err(max8998.dev, "Failed to request IRQ %d: %d\n",
    max8998.irq, ret);
    return ret;
    }
    if (!max8998.ono)
    return 0;
    ret = request_threaded_irq(max8998.ono, core::ptr::null_mut(), max8998_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING |
    IRQF_ONESHOT, "max8998-ono", max8998);
    if (ret)
    dev_err(max8998.dev, "Failed to request IRQ %d: %d\n",
    max8998.ono, ret);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn max8998_irq_exit(max8998: *mut max8998_dev) {
    void max8998_irq_exit(struct max8998_dev *max8998)
    {
    if (max8998.ono)
    free_irq(max8998.ono, max8998);
    if (max8998.irq)
    free_irq(max8998.irq, max8998);
    }
