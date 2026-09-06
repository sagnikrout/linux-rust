//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/atomisp2/pm.c
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
// Dummy driver for Intel's Image Signal Processor found on Bay Trail
// and Cherry Trail devices. The sole purpose of this driver is to allow
// the ISP to be put in D3.
//
// Copyright (C) 2018 Hans de Goede <hdegoede@redhat.com>
//
// Based on various non upstream patches for ISP support:
// Copyright (C) 2010-2017 Intel Corporation. All rights reserved.
// Copyright (c) 2010 Silicon Hive www.siliconhive.com.
//

// PCI configuration regs
pub const PCI_INTERRUPT_CTRL: c_uint = 0x9c;
pub const PCI_CSI_CONTROL: c_uint = 0xe8;
pub const PCI_CSI_CONTROL_PORTS_OFF_MASK: c_uint = 0x7;
// IOSF BT_MBI_UNIT_PMC regs
pub const ISPSSPM0: c_uint = 0x39;
pub const ISPSSPM0_ISPSSC_OFFSET: c_int = 0;
pub const ISPSSPM0_ISPSSC_MASK: c_uint = 0x00000003;
pub const ISPSSPM0_ISPSSS_OFFSET: c_int = 24;
pub const ISPSSPM0_ISPSSS_MASK: c_uint = 0x03000000;
pub const ISPSSPM0_IUNIT_POWER_ON: c_uint = 0x0;
pub const ISPSSPM0_IUNIT_POWER_OFF: c_uint = 0x3;
#[no_mangle]
unsafe extern "C" fn isp_set_power(dev: *mut pci_dev, enable: bool) -> c_int {
    static int isp_set_power(struct pci_dev *dev, bool enable)
    {
    unsigned long timeout;
    let mut val: u32 = enable ? ISPSSPM0_IUNIT_POWER_ON : ISPSSPM0_IUNIT_POWER_OFF;
// Write to ISPSSPM0 bit[1:0] to power on/off the IUNIT
    iosf_mbi_modify(BT_MBI_UNIT_PMC, MBI_REG_READ, ISPSSPM0,
    val, ISPSSPM0_ISPSSC_MASK);
//
// There should be no IUNIT access while power-down is
// in progress. HW sighting: 4567865.
// Wait up to 50 ms for the IUNIT to shut down.
// And we do the same for power on.
//
    timeout = jiffies + msecs_to_jiffies(50);
    do {
    u32 tmp;
// Wait until ISPSSPM0 bit[25:24] shows the right value
    iosf_mbi_read(BT_MBI_UNIT_PMC, MBI_REG_READ, ISPSSPM0, &tmp);
    tmp = (tmp & ISPSSPM0_ISPSSS_MASK) >> ISPSSPM0_ISPSSS_OFFSET;
    if (tmp == val)
    return 0;
    usleep_range(1000, 2000);
    } while (time_before(jiffies, timeout));
    dev_err(&dev.dev, "IUNIT power-%s timeout.\n", enable ? "on" : "off");
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn isp_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int isp_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    pm_runtime_allow(&dev.dev);
    pm_runtime_put_sync_suspend(&dev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isp_remove(dev: *mut pci_dev) {
    static void isp_remove(struct pci_dev *dev)
    {
    pm_runtime_get_sync(&dev.dev);
    pm_runtime_forbid(&dev.dev);
    }
#[no_mangle]
unsafe extern "C" fn isp_pci_suspend(dev: *mut device) -> c_int {
    static int isp_pci_suspend(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    u32 val;
    pci_write_config_dword(pdev, PCI_INTERRUPT_CTRL, 0);
//
// MRFLD IUNIT DPHY is located in an always-power-on island
// MRFLD HW design need all CSI ports are disabled before
// powering down the IUNIT.
//
    pci_read_config_dword(pdev, PCI_CSI_CONTROL, &val);
    val |= PCI_CSI_CONTROL_PORTS_OFF_MASK;
    pci_write_config_dword(pdev, PCI_CSI_CONTROL, val);
//
// We lose config space access when punit power gates
// the ISP. Can't use pci_set_power_state() because
// pmcsr won't actually change when we write to it.
//
    pci_save_state(pdev);
    pdev.current_state = PCI_D3cold;
    isp_set_power(pdev, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isp_pci_resume(dev: *mut device) -> c_int {
    static int isp_pci_resume(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    isp_set_power(pdev, true);
    pdev.current_state = PCI_D0;
    pci_restore_state(pdev);
    return 0;
    }
    static UNIVERSAL_DEV_PM_OPS(isp_pm_ops, isp_pci_suspend,
    isp_pci_resume, core::ptr::null_mut());
    static const struct pci_device_id isp_id_table[] = {
    { PCI_VDEVICE(INTEL, 0x0f38), },
    { PCI_VDEVICE(INTEL, 0x22b8), },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, isp_id_table);
    static struct pci_driver isp_pci_driver = {
    .name = "intel_atomisp2_pm",
    .id_table = isp_id_table,
    .probe = isp_probe,
    .remove = isp_remove,
    .driver.pm = &isp_pm_ops,
    };
    module_pci_driver(isp_pci_driver);
    MODULE_DESCRIPTION("Intel AtomISP2 dummy / power-management drv (for suspend)");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_LICENSE("GPL v2");
