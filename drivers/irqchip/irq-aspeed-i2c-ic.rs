//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-aspeed-i2c-ic.c
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
// Aspeed 24XX/25XX I2C Interrupt Controller.
//
// Copyright (C) 2012-2017 ASPEED Technology Inc.
// Copyright 2017 IBM Corporation
// Copyright 2017 Google, Inc.
//

pub const ASPEED_I2C_IC_NUM_BUS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_i2c_ic {
    pub base: *mut void __iomem,
    pub parent_irq: c_int,
    pub irq_domain: *mut irq_domain,
}

//
// The aspeed chip provides a single hardware interrupt for all of the I2C
// busses, so we use a dummy interrupt chip to translate this single interrupt
// into multiple interrupts, each associated with a single I2C bus.
//
#[no_mangle]
unsafe extern "C" fn aspeed_i2c_ic_irq_handler(desc: *mut irq_desc) {
    static void aspeed_i2c_ic_irq_handler(struct irq_desc *desc)
    {
    struct aspeed_i2c_ic *i2c_ic = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long bit, status;
    chained_irq_enter(chip, desc);
    status = readl(i2c_ic.base);
    for_each_set_bit(bit, &status, ASPEED_I2C_IC_NUM_BUS)
    generic_handle_domain_irq(i2c_ic.irq_domain, bit);
    chained_irq_exit(chip, desc);
    }
//
// Set simple handler and mark IRQ as valid. Nothing interesting to do here
// since we are using a dummy interrupt chip.
//
    static int aspeed_i2c_ic_map_irq_domain(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &dummy_irq_chip, handle_simple_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
    static const struct irq_domain_ops aspeed_i2c_ic_irq_domain_ops = {
    .map = aspeed_i2c_ic_map_irq_domain,
    };
    static int __init aspeed_i2c_ic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    struct aspeed_i2c_ic *i2c_ic;
    let mut ret: c_int = 0;
    i2c_ic = kzalloc_obj(*i2c_ic);
    if (!i2c_ic)
    return -ENOMEM;
    i2c_ic.base = of_iomap(node, 0);
    if (!i2c_ic.base) {
    ret = -ENOMEM;
    goto err_free_ic;
    }
    i2c_ic.parent_irq = irq_of_parse_and_map(node, 0);
    if (!i2c_ic.parent_irq) {
    ret = -EINVAL;
    goto err_iounmap;
    }
    i2c_ic.irq_domain = irq_domain_create_linear(of_fwnode_handle(node), ASPEED_I2C_IC_NUM_BUS,
    &aspeed_i2c_ic_irq_domain_ops,
    core::ptr::null_mut());
    if (!i2c_ic.irq_domain) {
    ret = -ENOMEM;
    goto err_iounmap;
    }
    i2c_ic.irq_domain.name = "aspeed-i2c-domain";
    irq_set_chained_handler_and_data(i2c_ic.parent_irq,
    aspeed_i2c_ic_irq_handler, i2c_ic);
    pr_info("i2c controller registered, irq %d\n", i2c_ic.parent_irq);
    return 0;
    err_iounmap:
    iounmap(i2c_ic.base);
    err_free_ic:
    kfree(i2c_ic);
    return ret;
    }
    IRQCHIP_DECLARE(ast2400_i2c_ic, "aspeed,ast2400-i2c-ic", aspeed_i2c_ic_of_init);
    IRQCHIP_DECLARE(ast2500_i2c_ic, "aspeed,ast2500-i2c-ic", aspeed_i2c_ic_of_init);
