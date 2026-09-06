//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/piix4-poweroff.c
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
// Copyright (C) 2016 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//

    static struct pci_dev *pm_dev;
    static resource_size_t io_offset;
    enum piix4_pm_io_reg {
    PIIX4_FUNC3IO_PMSTS			= 0x00,

    PIIX4_FUNC3IO_PMCNTRL			= 0x04,

    };
pub const PIIX4_SUSPEND_MAGIC: c_uint = 0x00120002;
    let mut piix4_pm_io_region: static int = PCI_BRIDGE_RESOURCES;
#[no_mangle]
unsafe extern "C" fn piix4_poweroff() {
    static void piix4_poweroff(void)
    {
    int spec_devid;
    u16 sts;
// Ensure the power button status is clear
    while (1) {
    sts = inw(io_offset + PIIX4_FUNC3IO_PMSTS);
    if (!(sts & PIIX4_FUNC3IO_PMSTS_PWRBTN_STS))
    break;
    outw(sts, io_offset + PIIX4_FUNC3IO_PMSTS);
    }
// Enable entry to suspend
    outw(PIIX4_FUNC3IO_PMCNTRL_SUS_TYP_SOFF | PIIX4_FUNC3IO_PMCNTRL_SUS_EN,
    io_offset + PIIX4_FUNC3IO_PMCNTRL);
// If the special cycle occurs too soon this doesn't work...
    mdelay(10);
//
// The PIIX4 will enter the suspend state only after seeing a special
// cycle with the correct magic data on the PCI bus. Generate that
// cycle now.
//
    spec_devid = PCI_DEVID(0, PCI_DEVFN(0x1f, 0x7));
    pci_bus_write_config_dword(pm_dev.bus, spec_devid, 0,
    PIIX4_SUSPEND_MAGIC);
// Give the system some time to power down, then error
    mdelay(1000);
    pr_emerg("Unable to poweroff system\n");
    }
    static int piix4_poweroff_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    int res;
    if (pm_dev)
    return -EINVAL;
// Request access to the PIIX4 PM IO registers
    res = pci_request_region(dev, piix4_pm_io_region,
    "PIIX4 PM IO registers");
    if (res) {
    dev_err(&dev.dev, "failed to request PM IO registers: %d\n",
    res);
    return res;
    }
    pm_dev = dev;
    io_offset = pci_resource_start(dev, piix4_pm_io_region);
    pm_power_off = piix4_poweroff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn piix4_poweroff_remove(dev: *mut pci_dev) {
    static void piix4_poweroff_remove(struct pci_dev *dev)
    {
    if (pm_power_off == piix4_poweroff)
    pm_power_off = core::ptr::null_mut();
    pci_release_region(dev, piix4_pm_io_region);
    pm_dev = core::ptr::null_mut();
    }
    static const struct pci_device_id piix4_poweroff_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_3) },
    { 0 },
    };
    MODULE_DEVICE_TABLE(pci, piix4_poweroff_ids);
    static struct pci_driver piix4_poweroff_driver = {
    .name		= "piix4-poweroff",
    .id_table	= piix4_poweroff_ids,
    .probe		= piix4_poweroff_probe,
    .remove		= piix4_poweroff_remove,
    };
    module_pci_driver(piix4_poweroff_driver);
    MODULE_AUTHOR("Paul Burton <paul.burton@mips.com>");
    MODULE_DESCRIPTION("Intel PIIX4 power-off driver");
    MODULE_LICENSE("GPL");
