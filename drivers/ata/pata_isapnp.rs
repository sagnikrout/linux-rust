//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_isapnp.c
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
// pata-isapnp.c - ISA PnP PATA controller driver.
// Copyright 2005/2006 Red Hat Inc, all rights reserved.
//
// Based in part on ide-pnp.c by Andrey Panin <pazke@donpac.ru>
//

    static const struct scsi_host_template isapnp_sht = {
    ATA_PIO_SHT(DRV_NAME),
    };
    static struct ata_port_operations isapnp_port_ops = {
    .inherits	= &ata_sff_port_ops,
    .cable_detect	= ata_cable_40wire,
    };
    static struct ata_port_operations isapnp_noalt_port_ops = {
    .inherits	= &ata_sff_port_ops,
    .cable_detect	= ata_cable_40wire,
// No altstatus so we don't want to use the lost interrupt poll
    .lost_interrupt = ATA_OP_NULL,
    };
//
// isapnp_init_one		-	attach an isapnp interface
// @idev: PnP device
// @dev_id: matching detect line
//
// Register an ISA bus IDE interface. Such interfaces are PIO 0 and
// non shared IRQ.
//
#[no_mangle]
unsafe extern "C" fn isapnp_init_one(idev: *mut pnp_dev, dev_id: *const pnp_device_id) -> c_int {
    static int isapnp_init_one(struct pnp_dev *idev, const struct pnp_device_id *dev_id)
    {
    struct ata_host *host;
    struct ata_port *ap;
    void __iomem *cmd_addr, *ctl_addr;
    let mut irq: c_int = 0;
    let mut handler: irq_handler_t = core::ptr::null_mut();
    if (pnp_port_valid(idev, 0) == 0)
    return -ENODEV;
    if (pnp_irq_valid(idev, 0)) {
    irq = pnp_irq(idev, 0);
    handler = ata_sff_interrupt;
    }
// allocate host
    host = ata_host_alloc(&idev.dev, 1);
    if (!host)
    return -ENOMEM;
// acquire resources and fill host
    cmd_addr = devm_ioport_map(&idev.dev, pnp_port_start(idev, 0), 8);
    if (!cmd_addr)
    return -ENOMEM;
    ap = host.ports[0];
    ap.ops = &isapnp_noalt_port_ops;
    ap.pio_mask = ATA_PIO0;
    ap.flags |= ATA_FLAG_SLAVE_POSS;
    ap.ioaddr.cmd_addr = cmd_addr;
    if (pnp_port_valid(idev, 1)) {
    ctl_addr = devm_ioport_map(&idev.dev,
    pnp_port_start(idev, 1), 1);
    if (!ctl_addr)
    return -ENOMEM;
    ap.ioaddr.altstatus_addr = ctl_addr;
    ap.ioaddr.ctl_addr = ctl_addr;
    ap.ops = &isapnp_port_ops;
    }
    ata_sff_std_ports(&ap.ioaddr);
    ata_port_desc(ap, "cmd 0x%llx ctl 0x%llx",
    (unsigned long long)pnp_port_start(idev, 0),
    (unsigned long long)pnp_port_start(idev, 1));
// activate
    return ata_host_activate(host, irq, handler, 0,
    &isapnp_sht);
    }
//
// isapnp_remove_one	-	unplug an isapnp interface
// @idev: PnP device
//
// Remove a previously configured PnP ATA port. Called only on module
// unload events as the core does not currently deal with ISAPnP docking.
//
#[no_mangle]
unsafe extern "C" fn isapnp_remove_one(idev: *mut pnp_dev) {
    static void isapnp_remove_one(struct pnp_dev *idev)
    {
    struct device *dev = &idev.dev;
    struct ata_host *host = dev_get_drvdata(dev);
    ata_host_detach(host);
    }
    static struct pnp_device_id isapnp_devices[] = {
// Generic ESDI/IDE/ATA compatible hard disk controller
    { .id = "PNP0600" },
    { }
    };
    MODULE_DEVICE_TABLE(pnp, isapnp_devices);
    static struct pnp_driver isapnp_driver = {
    .name		= DRV_NAME,
    .id_table	= isapnp_devices,
    .probe		= isapnp_init_one,
    .remove		= isapnp_remove_one,
    };
    module_pnp_driver(isapnp_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("low-level driver for ISA PnP ATA");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_VERSION);
