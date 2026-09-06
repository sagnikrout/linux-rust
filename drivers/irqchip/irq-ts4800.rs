//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ts4800.c
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


//
// Multiplexed-IRQs driver for TS-4800's FPGA
//
// Copyright (c) 2015 - Savoir-faire Linux
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const IRQ_MASK: c_uint = 0x4;
pub const IRQ_STATUS: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts4800_irq_data {
    pub base: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub domain: *mut irq_domain,
    pub parent_irq: c_uint,
}

#[no_mangle]
unsafe extern "C" fn ts4800_irq_mask(d: *mut irq_data) {
    static void ts4800_irq_mask(struct irq_data *d)
    {
    struct ts4800_irq_data *data = irq_data_get_irq_chip_data(d);
    let mut reg: u16 = readw(data.base + IRQ_MASK);
    let mut mask: u16 = 1 << d.hwirq;
    writew(reg | mask, data.base + IRQ_MASK);
    }
#[no_mangle]
unsafe extern "C" fn ts4800_irq_unmask(d: *mut irq_data) {
    static void ts4800_irq_unmask(struct irq_data *d)
    {
    struct ts4800_irq_data *data = irq_data_get_irq_chip_data(d);
    let mut reg: u16 = readw(data.base + IRQ_MASK);
    let mut mask: u16 = 1 << d.hwirq;
    writew(reg & ~mask, data.base + IRQ_MASK);
    }
#[no_mangle]
unsafe extern "C" fn ts4800_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    static void ts4800_irq_print_chip(struct irq_data *d, struct seq_file *p)
    {
    struct ts4800_irq_data *data = irq_data_get_irq_chip_data(d);
    seq_puts(p, dev_name(&data.pdev.dev));
    }
    static const struct irq_chip ts4800_chip = {
    .irq_mask	= ts4800_irq_mask,
    .irq_unmask	= ts4800_irq_unmask,
    .irq_print_chip	= ts4800_irq_print_chip,
    };
    static int ts4800_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct ts4800_irq_data *data = d.host_data;
    irq_set_chip_and_handler(irq, &ts4800_chip, handle_simple_irq);
    irq_set_chip_data(irq, data);
    irq_set_noprobe(irq);
    return 0;
    }
    static const struct irq_domain_ops ts4800_ic_ops = {
    .map = ts4800_irqdomain_map,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn ts4800_ic_chained_handle_irq(desc: *mut irq_desc) {
    static void ts4800_ic_chained_handle_irq(struct irq_desc *desc)
    {
    struct ts4800_irq_data *data = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut status: u16 = readw(data.base + IRQ_STATUS);
    chained_irq_enter(chip, desc);
    if (unlikely(status == 0)) {
    handle_bad_irq(desc);
    goto out;
    }
    do {
    let mut bit: c_uint = __ffs(status);
    generic_handle_domain_irq(data.domain, bit);
    status &= ~(1 << bit);
    } while (status);
    out:
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn ts4800_ic_probe(pdev: *mut platform_device) -> c_int {
    static int ts4800_ic_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct ts4800_irq_data *data;
    int parent_irq;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.pdev = pdev;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base))
    return PTR_ERR(data.base);
    writew(0xFFFF, data.base + IRQ_MASK);
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    dev_err(&pdev.dev, "failed to get parent IRQ\n");
    return -EINVAL;
    }
    data.domain = irq_domain_create_linear(dev_fwnode(&pdev.dev), 8, &ts4800_ic_ops, data);
    if (!data.domain) {
    dev_err(&pdev.dev, "cannot add IRQ domain\n");
    return -ENOMEM;
    }
    irq_set_chained_handler_and_data(parent_irq,
    ts4800_ic_chained_handle_irq, data);
    data.parent_irq = parent_irq;
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts4800_ic_remove(pdev: *mut platform_device) {
    static void ts4800_ic_remove(struct platform_device *pdev)
    {
    struct ts4800_irq_data *data = platform_get_drvdata(pdev);
    unsigned int hwirq;
    irq_set_chained_handler_and_data(data.parent_irq, core::ptr::null_mut(), core::ptr::null_mut());
    for (hwirq = 0; hwirq < 8; hwirq++)
    irq_dispose_mapping(irq_find_mapping(data.domain, hwirq));
    irq_dispose_mapping(data.parent_irq);
    irq_domain_remove(data.domain);
    }
    static const struct of_device_id ts4800_ic_of_match[] = {
    { .compatible = "technologic,ts4800-irqc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ts4800_ic_of_match);
    static struct platform_driver ts4800_ic_driver = {
    .probe		= ts4800_ic_probe,
    .remove		= ts4800_ic_remove,
    .driver = {
    .name		= "ts4800-irqc",
    .of_match_table	= ts4800_ic_of_match,
    },
    };
    module_platform_driver(ts4800_ic_driver);
    MODULE_AUTHOR("Damien Riegel <damien.riegel@savoirfairelinux.com>");
    MODULE_DESCRIPTION("Multiplexed-IRQs driver for TS-4800's FPGA");
    MODULE_LICENSE("GPL v2");
