//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-intel-mrfld.c
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
// extcon driver for Basin Cove PMIC
//
// Copyright (c) 2019, Intel Corporation.
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

pub const BCOVE_USBIDCTRL: c_uint = 0x19;

pub const BCOVE_USBIDSTS: c_uint = 0x1a;

pub const BCOVE_USBIDSTS_RARBRC_SHIFT: c_int = 1;
pub const BCOVE_USBIDSTS_NO_ACA: c_int = 0;
pub const BCOVE_USBIDSTS_R_ID_A: c_int = 1;
pub const BCOVE_USBIDSTS_R_ID_B: c_int = 2;
pub const BCOVE_USBIDSTS_R_ID_C: c_int = 3;

    BCOVE_CHGRIRQ_BATTDET | BCOVE_CHGRIRQ_USBIDDET)
pub const BCOVE_CHGRCTRL0: c_uint = 0x4b;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrfld_extcon_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub edev: *mut extcon_dev,
    pub status: c_uint,
    pub id: c_uint,
}

    static const unsigned int mrfld_extcon_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_CHG_USB_SDP,
    EXTCON_CHG_USB_CDP,
    EXTCON_CHG_USB_DCP,
    EXTCON_CHG_USB_ACA,
    EXTCON_NONE,
    };
    static int mrfld_extcon_clear(struct mrfld_extcon_data *data, unsigned int reg,
    unsigned int mask)
    {
    return regmap_update_bits(data.regmap, reg, mask, 0x00);
    }
    static int mrfld_extcon_set(struct mrfld_extcon_data *data, unsigned int reg,
    unsigned int mask)
    {
    return regmap_update_bits(data.regmap, reg, mask, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_sw_control(data: *mut mrfld_extcon_data, enable: bool) -> c_int {
    static int mrfld_extcon_sw_control(struct mrfld_extcon_data *data, bool enable)
    {
    let mut mask: c_uint = BCOVE_CHGRCTRL0_SWCONTROL;
    struct device *dev = data.dev;
    int ret;
    if (enable)
    ret = mrfld_extcon_set(data, BCOVE_CHGRCTRL0, mask);
    else
    ret = mrfld_extcon_clear(data, BCOVE_CHGRCTRL0, mask);
    if (ret)
    dev_err(dev, "can't set SW control: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_get_id(data: *mut mrfld_extcon_data) -> c_int {
    static int mrfld_extcon_get_id(struct mrfld_extcon_data *data)
    {
    struct regmap *regmap = data.regmap;
    unsigned int id;
    bool ground;
    int ret;
    ret = regmap_read(regmap, BCOVE_USBIDSTS, &id);
    if (ret)
    return ret;
    if (id & BCOVE_USBIDSTS_FLOAT)
    return INTEL_USB_ID_FLOAT;
    switch ((id & BCOVE_USBIDSTS_RARBRC_MASK) >> BCOVE_USBIDSTS_RARBRC_SHIFT) {
    case BCOVE_USBIDSTS_R_ID_A:
    return INTEL_USB_RID_A;
    case BCOVE_USBIDSTS_R_ID_B:
    return INTEL_USB_RID_B;
    case BCOVE_USBIDSTS_R_ID_C:
    return INTEL_USB_RID_C;
    }
//
// PMIC A0 reports USBIDSTS_GND = 1 for ID_GND,
// but PMIC B0 reports USBIDSTS_GND = 0 for ID_GND.
// Thus we must check this bit at last.
//
    ground = id & BCOVE_USBIDSTS_GND;
    switch ('A' + BCOVE_MAJOR(data.id)) {
    case 'A':
    return ground ? INTEL_USB_ID_GND : INTEL_USB_ID_FLOAT;
    case 'B':
    return ground ? INTEL_USB_ID_FLOAT : INTEL_USB_ID_GND;
    }
// Unknown or unsupported type
    return INTEL_USB_ID_FLOAT;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_role_detect(data: *mut mrfld_extcon_data) -> c_int {
    static int mrfld_extcon_role_detect(struct mrfld_extcon_data *data)
    {
    unsigned int id;
    bool usb_host;
    int ret;
    ret = mrfld_extcon_get_id(data);
    if (ret < 0)
    return ret;
    id = ret;
    usb_host = (id == INTEL_USB_ID_GND) || (id == INTEL_USB_RID_A);
    extcon_set_state_sync(data.edev, EXTCON_USB_HOST, usb_host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_cable_detect(data: *mut mrfld_extcon_data) -> c_int {
    static int mrfld_extcon_cable_detect(struct mrfld_extcon_data *data)
    {
    struct regmap *regmap = data.regmap;
    unsigned int status, change;
    int ret;
//
// It seems SCU firmware clears the content of BCOVE_CHGRIRQ1
// and makes it useless for OS. Instead we compare a previously
// stored status to the current one, provided by BCOVE_SCHGRIRQ1.
//
    ret = regmap_read(regmap, BCOVE_SCHGRIRQ1, &status);
    if (ret)
    return ret;
    change = status ^ data.status;
    if (!change)
    return -ENODATA;
    if (change & BCOVE_CHGRIRQ_USBIDDET) {
    ret = mrfld_extcon_role_detect(data);
    if (ret)
    return ret;
    }
    data.status = status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mrfld_extcon_interrupt(int irq, void *dev_id)
    {
    struct mrfld_extcon_data *data = dev_id;
    int ret;
    ret = mrfld_extcon_cable_detect(data);
    mrfld_extcon_clear(data, BCOVE_MIRQLVL1, BCOVE_LVL1_CHGR);
    return ret ? IRQ_NONE: IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_probe(pdev: *mut platform_device) -> c_int {
    static int mrfld_extcon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev.parent);
    struct regmap *regmap = pmic.regmap;
    struct mrfld_extcon_data *data;
    unsigned int status;
    unsigned int id;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = dev;
    data.regmap = regmap;
    data.edev = devm_extcon_dev_allocate(dev, mrfld_extcon_cable);
    if (IS_ERR(data.edev))
    return PTR_ERR(data.edev);
    ret = devm_extcon_dev_register(dev, data.edev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "can't register extcon device\n");
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), mrfld_extcon_interrupt,
    IRQF_ONESHOT | IRQF_SHARED, pdev.name,
    data);
    if (ret)
    return dev_err_probe(dev, ret, "can't register IRQ handler\n");
    ret = regmap_read(regmap, BCOVE_ID, &id);
    if (ret)
    return dev_err_probe(dev, ret, "can't read PMIC ID\n");
    data.id = id;
    ret = mrfld_extcon_sw_control(data, true);
    if (ret)
    return ret;
// Get initial state
    mrfld_extcon_role_detect(data);
//
// Cached status value is used for cable detection, see comments
// in mrfld_extcon_cable_detect(), we need to sync cached value
// with a real state of the hardware.
//
    regmap_read(regmap, BCOVE_SCHGRIRQ1, &status);
    data.status = status;
    mrfld_extcon_clear(data, BCOVE_MIRQLVL1, BCOVE_LVL1_CHGR);
    mrfld_extcon_clear(data, BCOVE_MCHGRIRQ1, BCOVE_CHGRIRQ_ALL);
    mrfld_extcon_set(data, BCOVE_USBIDCTRL, BCOVE_USBIDCTRL_ALL);
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_extcon_remove(pdev: *mut platform_device) {
    static void mrfld_extcon_remove(struct platform_device *pdev)
    {
    struct mrfld_extcon_data *data = platform_get_drvdata(pdev);
    mrfld_extcon_sw_control(data, false);
    }
    static const struct platform_device_id mrfld_extcon_id_table[] = {
    { .name = "mrfld_bcove_pwrsrc" },
    {}
    };
    MODULE_DEVICE_TABLE(platform, mrfld_extcon_id_table);
    static struct platform_driver mrfld_extcon_driver = {
    .driver = {
    .name	= "mrfld_bcove_pwrsrc",
    },
    .probe		= mrfld_extcon_probe,
    .remove		= mrfld_extcon_remove,
    .id_table	= mrfld_extcon_id_table,
    };
    module_platform_driver(mrfld_extcon_driver);
    MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
    MODULE_DESCRIPTION("extcon driver for Intel Merrifield Basin Cove PMIC");
    MODULE_LICENSE("GPL v2");
