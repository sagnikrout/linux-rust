//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/lpc_sch.c
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
// lpc_sch.c - LPC interface for Intel Poulsbo SCH
//
// LPC bridge function of the Intel SCH contains many other
// functional units, such as Interrupt controllers, Timers,
// Power Management, System Management, GPIO, RTC, and LPC
// Configuration Registers.
//
// Copyright (c) 2010 CompuLab Ltd
// Copyright (c) 2014 Intel Corp.
// Author: Denis Turischev <denis@compulab.co.il>
//

pub const SMBASE: c_uint = 0x40;
pub const SMBUS_IO_SIZE: c_int = 64;
pub const GPIO_BASE: c_uint = 0x44;
pub const GPIO_IO_SIZE: c_int = 64;
pub const GPIO_IO_SIZE_CENTERTON: c_int = 128;
pub const WDTBASE: c_uint = 0x84;
pub const WDT_IO_SIZE: c_int = 64;
    enum sch_chipsets {
    LPC_SCH = 0,		/* Intel Poulsbo SCH */
    LPC_ITC,		/* Intel Tunnel Creek */
    LPC_CENTERTON,		/* Intel Centerton */
    LPC_QUARK_X1000,	/* Intel Quark X1000 */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc_sch_info {
    pub io_size_smbus: c_uint,
    pub io_size_gpio: c_uint,
    pub io_size_wdt: c_uint,
}

    static struct lpc_sch_info sch_chipset_info[] = {
    [LPC_SCH] = {
    .io_size_smbus = SMBUS_IO_SIZE,
    .io_size_gpio = GPIO_IO_SIZE,
    },
    [LPC_ITC] = {
    .io_size_smbus = SMBUS_IO_SIZE,
    .io_size_gpio = GPIO_IO_SIZE,
    .io_size_wdt = WDT_IO_SIZE,
    },
    [LPC_CENTERTON] = {
    .io_size_smbus = SMBUS_IO_SIZE,
    .io_size_gpio = GPIO_IO_SIZE_CENTERTON,
    .io_size_wdt = WDT_IO_SIZE,
    },
    [LPC_QUARK_X1000] = {
    .io_size_gpio = GPIO_IO_SIZE,
    .io_size_wdt = WDT_IO_SIZE,
    },
    };
    static const struct pci_device_id lpc_sch_ids[] = {
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_SCH_LPC), .driver_data = LPC_SCH },
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_ITC_LPC), .driver_data = LPC_ITC },
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_CENTERTON_ILB), .driver_data = LPC_CENTERTON },
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QUARK_X1000_ILB), .driver_data = LPC_QUARK_X1000 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, lpc_sch_ids);
pub const LPC_NO_RESOURCE: c_int = 1;
pub const LPC_SKIP_RESOURCE: c_int = 2;
    static int lpc_sch_get_io(struct pci_dev *pdev, int where, const char *name,
    struct resource *res, int size)
    {
    unsigned int base_addr_cfg;
    unsigned short base_addr;
    if (size == 0)
    return LPC_NO_RESOURCE;
    pci_read_config_dword(pdev, where, &base_addr_cfg);
    base_addr = 0;
    if (!(base_addr_cfg & (1 << 31)))
    dev_warn(&pdev.dev, "Decode of the %s I/O range disabled\n",
    name);
    else
    base_addr = (unsigned short)base_addr_cfg;
    if (base_addr == 0) {
    dev_warn(&pdev.dev, "I/O space for %s uninitialized\n", name);
    return LPC_SKIP_RESOURCE;
    }
    res.start = base_addr;
    res.end = base_addr + size - 1;
    res.flags = IORESOURCE_IO;
    return 0;
    }
    static int lpc_sch_populate_cell(struct pci_dev *pdev, int where,
    const char *name, int size, int id,
    struct mfd_cell *cell)
    {
    struct resource *res;
    int ret;
    res = devm_kzalloc(&pdev.dev, sizeof(*res), GFP_KERNEL);
    if (!res)
    return -ENOMEM;
    ret = lpc_sch_get_io(pdev, where, name, res, size);
    if (ret)
    return ret;
    memset(cell, 0, sizeof(*cell));
    cell.name = name;
    cell.resources = res;
    cell.num_resources = 1;
    cell.ignore_resource_conflicts = true;
    cell.id = id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc_sch_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int lpc_sch_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct mfd_cell lpc_sch_cells[3];
    struct lpc_sch_info *info = &sch_chipset_info[id.driver_data];
    let mut cells: c_uint = 0;
    int ret;
    ret = lpc_sch_populate_cell(dev, SMBASE, "isch_smbus",
    info.io_size_smbus,
    id.device, &lpc_sch_cells[cells]);
    if (ret < 0)
    return ret;
    if (ret == 0)
    cells++;
    ret = lpc_sch_populate_cell(dev, GPIO_BASE, "sch_gpio",
    info.io_size_gpio,
    id.device, &lpc_sch_cells[cells]);
    if (ret < 0)
    return ret;
    if (ret == 0)
    cells++;
    ret = lpc_sch_populate_cell(dev, WDTBASE, "ie6xx_wdt",
    info.io_size_wdt,
    id.device, &lpc_sch_cells[cells]);
    if (ret < 0)
    return ret;
    if (ret == 0)
    cells++;
    if (cells == 0) {
    dev_err(&dev.dev, "All decode registers disabled.\n");
    return -ENODEV;
    }
    return mfd_add_devices(&dev.dev, 0, lpc_sch_cells, cells, core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn lpc_sch_remove(dev: *mut pci_dev) {
    static void lpc_sch_remove(struct pci_dev *dev)
    {
    mfd_remove_devices(&dev.dev);
    }
    static struct pci_driver lpc_sch_driver = {
    .name		= "lpc_sch",
    .id_table	= lpc_sch_ids,
    .probe		= lpc_sch_probe,
    .remove		= lpc_sch_remove,
    };
    module_pci_driver(lpc_sch_driver);
    MODULE_AUTHOR("Denis Turischev <denis@compulab.co.il>");
    MODULE_DESCRIPTION("LPC interface for Intel Poulsbo SCH");
    MODULE_LICENSE("GPL");
