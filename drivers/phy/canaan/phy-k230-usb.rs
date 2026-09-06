//! Automatically rewritten from C to Rust
//! Source: drivers/phy/canaan/phy-k230-usb.c
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
// Canaan usb PHY driver
//
// Copyright (C) 2026 Jiayu Du <jiayu.riscv@isrc.iscas.ac.cn>
//

pub const MAX_PHYS: c_int = 2;
// Register offsets within the HiSysConfig system controller
pub const K230_USB0_TEST_REG_BASE: c_uint = 0x70;
pub const K230_USB0_CTL_REG_BASE: c_uint = 0xb0;
pub const K230_USB1_TEST_REG_BASE: c_uint = 0x90;
pub const K230_USB1_CTL_REG_BASE: c_uint = 0xb8;
// Relative offsets within each PHY's control/test block
pub const CTL0_OFFSET: c_uint = 0x00;
pub const CTL1_OFFSET: c_uint = 0x04;
pub const TEST_CTL3_OFFSET: c_uint = 0x0c;
// Bit definitions for TEST_CTL3

// USB control register 0 in HiSysConfig system controller
// PLL Integral Path Tune

// PLL Proportional Path Tune

// PLL Bandwidth Adjustment

// VReg18 Bypass Control

// Retention Mode Enable

// Reserved Request Input

// External VBUS Valid Select

// OTG Block Disable Control

// Drive VBUS Enable

// Autoresume Mode Enable

// HS Transceiver Asynchronous Control

// USB 1.1 Transmit Data

// USB 1.1 SE0 Generation

// USB 1.1 Data Enable

// Disconnect Threshold

// Squelch Threshold

// USB control register 1 in HiSysConfig system controller
// Data Detect Voltage

// VBUS Valid Threshold

// Transmitter High-Speed Crossover

// FS/LS Source Impedance

// HS DC Voltage Level

// HS Transmitter Rise/Fall Time

// USB Source Impedance

// HS Transmitter Pre-Emphasis Current Control

// HS Transmitter Pre-Emphasis Duration Control

// charging detection

    ( \
    FIELD_PREP(USB_CTL0_PLLITUNE_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_PLLPTUNE_MASK, 0xc) | \
    FIELD_PREP(USB_CTL0_PLLBTUNE_MASK, 0x1) | \
    FIELD_PREP(USB_CTL0_VREGBYPASS_MASK, 0x1) | \
    FIELD_PREP(USB_CTL0_RETENABLEN_MASK, 0x1) | \
    FIELD_PREP(USB_CTL0_RESREQIN_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_VBUSVLDEXTSEL0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_OTGDISABLE0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_DRVVBUS0_MASK, 0x1) | \
    FIELD_PREP(USB_CTL0_AUTORSMENB0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_HSXCVREXTCTL0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_FSDATAEXT0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_FSSE0EXT0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_TXENABLEN0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL0_COMPDISTUNE0_MASK, 0x3) | \
    FIELD_PREP(USB_CTL0_SQRXTUNE0_MASK, 0x3) \
    )

    ( \
    FIELD_PREP(USB_CTL1_VDATREFTUNE0_MASK, 0x1) | \
    FIELD_PREP(USB_CTL1_OTGTUNE0_MASK, 0x3) | \
    FIELD_PREP(USB_CTL1_TXHSXVTUNE0_MASK, 0x3) | \
    FIELD_PREP(USB_CTL1_TXFSLSTUNE0_MASK, 0x3) | \
    FIELD_PREP(USB_CTL1_TXVREFTUNE0_MASK, 0x3) | \
    FIELD_PREP(USB_CTL1_TXRISETUNE0_MASK, 0x1) | \
    FIELD_PREP(USB_CTL1_TXRESTUNE0_MASK, 0x1) | \
    FIELD_PREP(USB_CTL1_TXPREEMPAMPTUNE0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL1_TXPREEMPPULSETUNE0_MASK, 0x0) | \
    FIELD_PREP(USB_CTL1_CHRGSRCPUENB0_MASK, 0x0) \
    )
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k230_usb_phy_instance {
    pub global: *mut k230_usb_phy_global,
    pub phy: *mut phy,
    pub test_offset: u32,
    pub ctl_offset: u32,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k230_usb_phy_global {
    pub phys: [k230_usb_phy_instance; MAX_PHYS],
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn k230_usb_phy_power_on(phy: *mut phy) -> c_int {
    static int k230_usb_phy_power_on(struct phy *phy)
    {
    struct k230_usb_phy_instance *inst = phy_get_drvdata(phy);
    struct k230_usb_phy_global *global = inst.global;
    void __iomem *base = global.base;
    u32 val;
// Apply recommended settings
    writel(K230_PHY_CTL0_VAL, base + inst.ctl_offset + CTL0_OFFSET);
    writel(K230_PHY_CTL1_VAL, base + inst.ctl_offset + CTL1_OFFSET);
// Configure test register (pull-ups/pull-downs)
    val = readl(base + inst.test_offset + TEST_CTL3_OFFSET);
    val |= USB_IDPULLUP0;
    if (inst.index == 1)
    val |= (USB_DMPULLDOWN0 | USB_DPPULLDOWN0);
    else
    val &= ~(USB_DMPULLDOWN0 | USB_DPPULLDOWN0);
    writel(val, base + inst.test_offset + TEST_CTL3_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k230_usb_phy_power_off(phy: *mut phy) -> c_int {
    static int k230_usb_phy_power_off(struct phy *phy)
    {
    struct k230_usb_phy_instance *inst = phy_get_drvdata(phy);
    struct k230_usb_phy_global *global = inst.global;
    void __iomem *base = global.base;
    u32 val;
    val = readl(base + inst.test_offset + TEST_CTL3_OFFSET);
    val &= ~(USB_DMPULLDOWN0 | USB_DPPULLDOWN0);
    writel(val, base + inst.test_offset + TEST_CTL3_OFFSET);
    return 0;
    }
    static const struct phy_ops k230_usb_phy_ops = {
    .power_on = k230_usb_phy_power_on,
    .power_off = k230_usb_phy_power_off,
    .owner = THIS_MODULE,
    };
    static struct phy *k230_usb_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct k230_usb_phy_global *global = dev_get_drvdata(dev);
    let mut idx: c_uint = args.args[0];
    if (idx >= MAX_PHYS)
    return ERR_PTR(-EINVAL);
    return global.phys[idx].phy;
    }
#[no_mangle]
unsafe extern "C" fn k230_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int k230_usb_phy_probe(struct platform_device *pdev)
    {
    struct k230_usb_phy_global *global;
    struct device *dev = &pdev.dev;
    struct phy_provider *provider;
    int i;
    global = devm_kzalloc(dev, sizeof(*global), GFP_KERNEL);
    if (!global)
    return -ENOMEM;
    dev_set_drvdata(dev, global);
    global.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(global.base))
    return dev_err_probe(dev, PTR_ERR(global.base),
    "failed to map registers\n");
    static const struct {
    u32 test_offset;
    u32 ctl_offset;
    } phy_reg_info[MAX_PHYS] = {
    [0] = { K230_USB0_TEST_REG_BASE, K230_USB0_CTL_REG_BASE },
    [1] = { K230_USB1_TEST_REG_BASE, K230_USB1_CTL_REG_BASE },
    };
    for (i = 0; i < MAX_PHYS; i++) {
    struct k230_usb_phy_instance *inst = &global.phys[i];
    struct phy *phy;
    inst.global = global;
    inst.index = i;
    inst.test_offset = phy_reg_info[i].test_offset;
    inst.ctl_offset  = phy_reg_info[i].ctl_offset;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &k230_usb_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy%d\n", i);
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, inst);
    inst.phy = phy;
    }
    provider = devm_of_phy_provider_register(dev, k230_usb_phy_xlate);
    if (IS_ERR(provider))
    return PTR_ERR(provider);
    return 0;
    }
    static const struct of_device_id k230_usb_phy_of_match[] = {
    { .compatible = "canaan,k230-usb-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, k230_usb_phy_of_match);
    static struct platform_driver k230_usb_phy_driver = {
    .probe = k230_usb_phy_probe,
    .driver = {
    .name = "k230-usb-phy",
    .of_match_table = k230_usb_phy_of_match,
    },
    };
    module_platform_driver(k230_usb_phy_driver);
    MODULE_DESCRIPTION("Canaan Kendryte K230 USB 2.0 PHY driver");
    MODULE_AUTHOR("Jiayu Du <jiayu.riscv@isrc.iscas.ac.cn>");
    MODULE_LICENSE("GPL");
