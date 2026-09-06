//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_rb532_cf.c
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
// A low-level PATA driver to handle a Compact Flash connected on the
// Mikrotik's RouterBoard 532 board.
//
// Copyright (C) 2007 Gabor Juhos <juhosg at openwrt.org>
// Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
//
// This file was based on: drivers/ata/pata_ixp4xx_cf.c
// Copyright (C) 2006-07 Tower Technologies
// Author: Alessandro Zummo <a.zummo@towertech.it>
//
// Also was based on the driver for Linux 2.4.xx published by Mikrotik for
// their RouterBoard 1xx and 5xx series devices. The original Mikrotik code
// seems not to have a license.
//

pub const RB500_CF_MAXPORTS: c_int = 1;
pub const RB500_CF_IO_DELAY: c_int = 400;
pub const RB500_CF_REG_BASE: c_uint = 0x0800;
pub const RB500_CF_REG_ERR: c_uint = 0x080D;
pub const RB500_CF_REG_CTRL: c_uint = 0x080E;
// 32bit buffered data register offset
pub const RB500_CF_REG_DBUF32: c_uint = 0x0C00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb532_cf_info {
    pub iobase: *mut void __iomem,
    pub gpio_line: *mut gpio_desc,
    pub irq: c_uint,
}

// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn rb532_pata_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t rb532_pata_irq_handler(int irq, void *dev_instance)
    {
    struct ata_host *ah = dev_instance;
    struct rb532_cf_info *info = ah.private_data;
    if (gpiod_get_value(info.gpio_line)) {
    irq_set_irq_type(info.irq, IRQ_TYPE_LEVEL_LOW);
    ata_sff_interrupt(info.irq, dev_instance);
    } else {
    irq_set_irq_type(info.irq, IRQ_TYPE_LEVEL_HIGH);
    }
    return IRQ_HANDLED;
    }
    static struct ata_port_operations rb532_pata_port_ops = {
    .inherits		= &ata_sff_port_ops,
    .sff_data_xfer		= ata_sff_data_xfer32,
    };
// ------------------------------------------------------------------------
    static const struct scsi_host_template rb532_pata_sht = {
    ATA_PIO_SHT(DRV_NAME),
    };
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn rb532_pata_setup_ports(ah: *mut ata_host) {
    static void rb532_pata_setup_ports(struct ata_host *ah)
    {
    struct rb532_cf_info *info = ah.private_data;
    struct ata_port *ap;
    ap = ah.ports[0];
    ap.ops		= &rb532_pata_port_ops;
    ap.pio_mask	= ATA_PIO4;
    ap.ioaddr.cmd_addr	= info.iobase + RB500_CF_REG_BASE;
    ap.ioaddr.ctl_addr	= info.iobase + RB500_CF_REG_CTRL;
    ap.ioaddr.altstatus_addr = info.iobase + RB500_CF_REG_CTRL;
    ata_sff_std_ports(&ap.ioaddr);
    ap.ioaddr.data_addr	= info.iobase + RB500_CF_REG_DBUF32;
    ap.ioaddr.error_addr	= info.iobase + RB500_CF_REG_ERR;
    }
#[no_mangle]
unsafe extern "C" fn rb532_pata_driver_probe(pdev: *mut platform_device) -> c_int {
    static int rb532_pata_driver_probe(struct platform_device *pdev)
    {
    int irq;
    struct gpio_desc *gpiod;
    struct ata_host *ah;
    struct rb532_cf_info *info;
    void __iomem *iobase;
    int ret;
    iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(iobase))
    return PTR_ERR(iobase);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (!irq)
    return -EINVAL;
    gpiod = devm_gpiod_get(&pdev.dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(gpiod)) {
    dev_err(&pdev.dev, "no GPIO found for irq%d\n", irq);
    return PTR_ERR(gpiod);
    }
    gpiod_set_consumer_name(gpiod, DRV_NAME);
// allocate host
    ah = ata_host_alloc(&pdev.dev, RB500_CF_MAXPORTS);
    if (!ah)
    return -ENOMEM;
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    ah.private_data = info;
    info.gpio_line = gpiod;
    info.irq = irq;
    info.iobase = iobase;
    rb532_pata_setup_ports(ah);
    ret = ata_host_activate(ah, irq, rb532_pata_irq_handler,
    IRQF_TRIGGER_LOW, &rb532_pata_sht);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rb532_pata_driver_remove(pdev: *mut platform_device) {
    static void rb532_pata_driver_remove(struct platform_device *pdev)
    {
    struct ata_host *ah = platform_get_drvdata(pdev);
    ata_host_detach(ah);
    }
    static struct platform_driver rb532_pata_platform_driver = {
    .probe		= rb532_pata_driver_probe,
    .remove		= rb532_pata_driver_remove,
    .driver	 = {
    .name   = DRV_NAME,
    },
    };

    module_platform_driver(rb532_pata_platform_driver);
    MODULE_AUTHOR("Gabor Juhos <juhosg at openwrt.org>");
    MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
    MODULE_DESCRIPTION(DRV_DESC);
    MODULE_VERSION(DRV_VERSION);
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
