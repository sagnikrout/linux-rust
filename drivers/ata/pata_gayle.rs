//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_gayle.c
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
// Amiga Gayle PATA controller driver
//
// Copyright (c) 2018 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Based on gayle.c:
//
// Created 12 Jul 1997 by Geert Uytterhoeven
//

pub const GAYLE_CONTROL: c_uint = 0x101a;
    static const struct scsi_host_template pata_gayle_sht = {
    ATA_PIO_SHT(DRV_NAME),
    };
// FIXME: is this needed?
    static unsigned int pata_gayle_data_xfer(struct ata_queued_cmd *qc,
    unsigned char *buf,
    unsigned int buflen, int rw)
    {
    struct ata_device *dev = qc.dev;
    struct ata_port *ap = dev.link.ap;
    void __iomem *data_addr = ap.ioaddr.data_addr;
    let mut words: c_uint = buflen >> 1;
// Transfer multiple of 2 bytes
    if (rw == READ)
    raw_insw((u16 *)data_addr, (u16 *)buf, words);
    else
    raw_outsw((u16 *)data_addr, (u16 *)buf, words);
// Transfer trailing byte, if any.
    if (unlikely(buflen & 0x01)) {
    unsigned char pad[2] = { };
// Point buf to the tail of buffer
    buf += buflen - 1;
    if (rw == READ) {
    raw_insw((u16 *)data_addr, (u16 *)pad, 1);
// buf = pad[0];
    } else {
    pad[0] = *buf;
    raw_outsw((u16 *)data_addr, (u16 *)pad, 1);
    }
    words++;
    }
    return words << 1;
    }
//
// Provide our own set_mode() as we don't want to change anything that has
// already been configured..
//
    static int pata_gayle_set_mode(struct ata_link *link,
    struct ata_device **unused)
    {
    struct ata_device *dev;
    ata_for_each_dev(dev, link, ENABLED) {
// We don't really care
    dev.pio_mode = dev.xfer_mode = XFER_PIO_0;
    dev.xfer_shift = ATA_SHIFT_PIO;
    dev.flags |= ATA_DFLAG_PIO;
    ata_dev_info(dev, "configured for PIO\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_gayle_irq_check(ap: *mut ata_port) -> bool {
    static bool pata_gayle_irq_check(struct ata_port *ap)
    {
    u8 ch;
    ch = z_readb((unsigned long)ap.private_data);
    return !!(ch & GAYLE_IRQ_IDE);
    }
#[no_mangle]
unsafe extern "C" fn pata_gayle_irq_clear(ap: *mut ata_port) {
    static void pata_gayle_irq_clear(struct ata_port *ap)
    {
    (void)z_readb((unsigned long)ap.ioaddr.status_addr);
    z_writeb(0x7c, (unsigned long)ap.private_data);
    }
    static struct ata_port_operations pata_gayle_a1200_ops = {
    .inherits	= &ata_sff_port_ops,
    .sff_data_xfer	= pata_gayle_data_xfer,
    .sff_irq_check	= pata_gayle_irq_check,
    .sff_irq_clear	= pata_gayle_irq_clear,
    .cable_detect	= ata_cable_unknown,
    .set_mode	= pata_gayle_set_mode,
    };
    static struct ata_port_operations pata_gayle_a4000_ops = {
    .inherits	= &ata_sff_port_ops,
    .sff_data_xfer	= pata_gayle_data_xfer,
    .cable_detect	= ata_cable_unknown,
    .set_mode	= pata_gayle_set_mode,
    };
#[no_mangle]
unsafe extern "C" fn pata_gayle_init_one(pdev: *mut platform_device) -> c_int {
    static int pata_gayle_init_one(struct platform_device *pdev)
    {
    struct resource *res;
    struct gayle_ide_platform_data *pdata;
    struct ata_host *host;
    struct ata_port *ap;
    void __iomem *base;
    int ret;
    pdata = dev_get_platdata(&pdev.dev);
    dev_info(&pdev.dev, "Amiga Gayle IDE controller (A%u style)\n",
    pdata.explicit_ack ? 1200 : 4000);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    if (!devm_request_mem_region(&pdev.dev, res.start,
    resource_size(res), DRV_NAME)) {
    pr_err(DRV_NAME ": resources busy\n");
    return -EBUSY;
    }
// allocate host
    host = ata_host_alloc(&pdev.dev, 1);
    if (!host)
    return -ENOMEM;
    ap = host.ports[0];
    if (pdata.explicit_ack)
    ap.ops = &pata_gayle_a1200_ops;
    else
    ap.ops = &pata_gayle_a4000_ops;
    ap.pio_mask = ATA_PIO4;
    ap.flags |= ATA_FLAG_SLAVE_POSS | ATA_FLAG_NO_IORDY;
    base = ZTWO_VADDR(pdata.base);
    ap.ioaddr.data_addr		= base;
    ap.ioaddr.error_addr		= base + 2 + 1 * 4;
    ap.ioaddr.feature_addr		= base + 2 + 1 * 4;
    ap.ioaddr.nsect_addr		= base + 2 + 2 * 4;
    ap.ioaddr.lbal_addr		= base + 2 + 3 * 4;
    ap.ioaddr.lbam_addr		= base + 2 + 4 * 4;
    ap.ioaddr.lbah_addr		= base + 2 + 5 * 4;
    ap.ioaddr.device_addr		= base + 2 + 6 * 4;
    ap.ioaddr.status_addr		= base + 2 + 7 * 4;
    ap.ioaddr.command_addr		= base + 2 + 7 * 4;
    ap.ioaddr.altstatus_addr	= base + GAYLE_CONTROL;
    ap.ioaddr.ctl_addr		= base + GAYLE_CONTROL;
    ap.private_data = (void *)ZTWO_VADDR(pdata.irqport);
    ata_port_desc(ap, "cmd 0x%lx ctl 0x%lx", pdata.base,
    pdata.base + GAYLE_CONTROL);
    ret = ata_host_activate(host, IRQ_AMIGA_PORTS, ata_sff_interrupt,
    IRQF_SHARED, &pata_gayle_sht);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_gayle_remove_one(pdev: *mut platform_device) {
    static void pata_gayle_remove_one(struct platform_device *pdev)
    {
    struct ata_host *host = platform_get_drvdata(pdev);
    ata_host_detach(host);
    }
    static struct platform_driver pata_gayle_driver = {
    .probe = pata_gayle_init_one,
    .remove = pata_gayle_remove_one,
    .driver = {
    .name = "amiga-gayle-ide",
    },
    };
    module_platform_driver(pata_gayle_driver);
    MODULE_AUTHOR("Bartlomiej Zolnierkiewicz");
    MODULE_DESCRIPTION("low-level driver for Amiga Gayle PATA");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:amiga-gayle-ide");
    MODULE_VERSION(DRV_VERSION);
