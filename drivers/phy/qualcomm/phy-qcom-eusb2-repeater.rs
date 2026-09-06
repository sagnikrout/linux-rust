//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-eusb2-repeater.c
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
// Copyright (c) 2023, Linaro Limited
//

// eUSB2 status registers
pub const EUSB2_RPTR_STATUS: c_uint = 0x08;

// eUSB2 control registers
pub const EUSB2_EN_CTL1: c_uint = 0x46;

pub const EUSB2_FORCE_EN_5: c_uint = 0xe8;

pub const EUSB2_FORCE_VAL_5: c_uint = 0xeD;

pub const EUSB2_TUNE_USB2_CROSSOVER: c_uint = 0x50;
pub const EUSB2_TUNE_IUSB2: c_uint = 0x51;
pub const EUSB2_TUNE_RES_FSDIF: c_uint = 0x52;
pub const EUSB2_TUNE_HSDISC: c_uint = 0x53;
pub const EUSB2_TUNE_SQUELCH_U: c_uint = 0x54;
pub const EUSB2_TUNE_USB2_SLEW: c_uint = 0x55;
pub const EUSB2_TUNE_USB2_EQU: c_uint = 0x56;
pub const EUSB2_TUNE_USB2_PREEM: c_uint = 0x57;
pub const EUSB2_TUNE_USB2_HS_COMP_CUR: c_uint = 0x58;
pub const EUSB2_TUNE_EUSB_SLEW: c_uint = 0x59;
pub const EUSB2_TUNE_EUSB_EQU: c_uint = 0x5A;
pub const EUSB2_TUNE_EUSB_HS_COMP_CUR: c_uint = 0x5B;
    static const int squelch_detector[] = {
    [0] = -6000,
    [1] = -5000,
    [2] = -4000,
    [3] = -3000,
    [4] = -2000,
    [5] = -1000,
    [6] = 0,
    [7] = 1000,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eusb2_repeater_init_tbl_reg {
    pub reg: c_uint,
    pub value: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eusb2_repeater_cfg {
    pub init_tbl: *const eusb2_repeater_init_tbl_reg,
    pub init_tbl_num: c_int,
    pub vreg_list: *const *const c_char,
    pub num_vregs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eusb2_repeater {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub phy: *mut phy,
    pub vregs: *mut regulator_bulk_data,
    pub cfg: *const eusb2_repeater_cfg,
    pub base: u32,
    pub mode: enum phy_mode,
}

    static const char * const pm8550b_vreg_l[] = {
    "vdd18", "vdd3",
    };
    static const struct eusb2_repeater_init_tbl_reg pm8550b_init_tbl[] = {
    { EUSB2_TUNE_IUSB2, 0x8 },
    { EUSB2_TUNE_SQUELCH_U, 0x3 },
    { EUSB2_TUNE_USB2_PREEM, 0x5 },
    };
    static const struct eusb2_repeater_init_tbl_reg smb2360_init_tbl[] = {
    { EUSB2_TUNE_IUSB2, 0x5 },
    { EUSB2_TUNE_SQUELCH_U, 0x3 },
    { EUSB2_TUNE_USB2_PREEM, 0x2 },
    };
    static const struct eusb2_repeater_init_tbl_reg smb2370_init_tbl[] = {
    { EUSB2_TUNE_IUSB2, 0x4 },
    { EUSB2_TUNE_SQUELCH_U, 0x3 },
    { EUSB2_TUNE_USB2_SLEW, 0x7 },
    { EUSB2_TUNE_USB2_PREEM, 0x0 },
    };
    static const struct eusb2_repeater_cfg pm8550b_eusb2_cfg = {
    .init_tbl	= pm8550b_init_tbl,
    .init_tbl_num	= ARRAY_SIZE(pm8550b_init_tbl),
    .vreg_list	= pm8550b_vreg_l,
    .num_vregs	= ARRAY_SIZE(pm8550b_vreg_l),
    };
    static const struct eusb2_repeater_cfg pmiv0104_eusb2_cfg = {
// No PMIC-specific init sequence, only board level tuning via DT
    .init_tbl	= (struct eusb2_repeater_init_tbl_reg[]) {},
    .init_tbl_num	= 0,
    .vreg_list	= pm8550b_vreg_l,
    .num_vregs	= ARRAY_SIZE(pm8550b_vreg_l),
    };
    static const struct eusb2_repeater_cfg smb2360_eusb2_cfg = {
    .init_tbl	= smb2360_init_tbl,
    .init_tbl_num	= ARRAY_SIZE(smb2360_init_tbl),
    .vreg_list	= pm8550b_vreg_l,
    .num_vregs	= ARRAY_SIZE(pm8550b_vreg_l),
    };
    static const struct eusb2_repeater_cfg smb2370_eusb2_cfg = {
    .init_tbl	= smb2370_init_tbl,
    .init_tbl_num	= ARRAY_SIZE(smb2370_init_tbl),
    .vreg_list	= pm8550b_vreg_l,
    .num_vregs	= ARRAY_SIZE(pm8550b_vreg_l),
    };
#[no_mangle]
unsafe extern "C" fn eusb2_repeater_init_vregs(rptr: *mut eusb2_repeater) -> c_int {
    static int eusb2_repeater_init_vregs(struct eusb2_repeater *rptr)
    {
    let mut num: c_int = rptr.cfg.num_vregs;
    struct device *dev = rptr.dev;
    int i;
    rptr.vregs = devm_kcalloc(dev, num, sizeof(*rptr.vregs), GFP_KERNEL);
    if (!rptr.vregs)
    return -ENOMEM;
    for (i = 0; i < num; i++)
    rptr.vregs[i].supply = rptr.cfg.vreg_list[i];
    return devm_regulator_bulk_get(dev, num, rptr.vregs);
    }
#[no_mangle]
unsafe extern "C" fn eusb2_repeater_init(phy: *mut phy) -> c_int {
    static int eusb2_repeater_init(struct phy *phy)
    {
    struct eusb2_repeater *rptr = phy_get_drvdata(phy);
    struct device_node *np = rptr.dev.of_node;
    struct regmap *regmap = rptr.regmap;
    let mut base: u32 = rptr.base;
    u32 poll_val;
    s32 dt_val;
    int ret;
    int i;
    u8 val;
    ret = regulator_bulk_enable(rptr.cfg.num_vregs, rptr.vregs);
    if (ret)
    return ret;
    regmap_write(regmap, base + EUSB2_EN_CTL1, EUSB2_RPTR_EN);
// Write registers from init table
    for (int i = 0; i < rptr.cfg.init_tbl_num; i++)
    regmap_write(regmap, base + rptr.cfg.init_tbl[i].reg,
    rptr.cfg.init_tbl[i].value);
// Override registers from devicetree values
    if (!of_property_read_u8(np, "qcom,tune-usb2-preem", &val))
    regmap_write(regmap, base + EUSB2_TUNE_USB2_PREEM, val);
    if (!of_property_read_u8(np, "qcom,tune-usb2-disc-thres", &val))
    regmap_write(regmap, base + EUSB2_TUNE_HSDISC, val);
    if (!of_property_read_u8(np, "qcom,tune-usb2-amplitude", &val))
    regmap_write(regmap, base + EUSB2_TUNE_IUSB2, val);
    if (!of_property_read_u8(np, "qcom,tune-res-fsdif", &val))
    regmap_write(regmap, base + EUSB2_TUNE_RES_FSDIF, val);
    if (!of_property_read_s32(np, "qcom,squelch-detector-bp", &dt_val)) {
    for (i = 0; i < ARRAY_SIZE(squelch_detector); i++) {
    if (squelch_detector[i] == dt_val) {
    regmap_write(regmap, base + EUSB2_TUNE_SQUELCH_U, i);
    break;
    }
    }
    }
// Wait for status OK
    ret = regmap_read_poll_timeout(regmap, base + EUSB2_RPTR_STATUS, poll_val,
    poll_val & RPTR_OK, 10, 5);
    if (ret)
    dev_err(rptr.dev, "initialization timed-out\n");
    return ret;
    }
    static int eusb2_repeater_set_mode(struct phy *phy,
    enum phy_mode mode, int submode)
    {
    struct eusb2_repeater *rptr = phy_get_drvdata(phy);
    struct regmap *regmap = rptr.regmap;
    let mut base: u32 = rptr.base;
    switch (mode) {
    case PHY_MODE_USB_HOST:
//
// CM.Lx is prohibited when repeater is already into Lx state as
// per eUSB 1.2 Spec. Below implement software workaround until
// PHY and controller is fixing seen observation.
//
    regmap_write(regmap, base + EUSB2_FORCE_EN_5, F_CLK_19P2M_EN);
    regmap_write(regmap, base + EUSB2_FORCE_VAL_5, V_CLK_19P2M_EN);
    break;
    case PHY_MODE_USB_DEVICE:
//
// In device mode clear host mode related workaround as there
// is no repeater reset available, and enable/disable of
// repeater doesn't clear previous value due to shared
// regulators (say host <-> device mode switch).
//
    regmap_write(regmap, base + EUSB2_FORCE_EN_5, 0);
    regmap_write(regmap, base + EUSB2_FORCE_VAL_5, 0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eusb2_repeater_exit(phy: *mut phy) -> c_int {
    static int eusb2_repeater_exit(struct phy *phy)
    {
    struct eusb2_repeater *rptr = phy_get_drvdata(phy);
    return regulator_bulk_disable(rptr.cfg.num_vregs, rptr.vregs);
    }
    static const struct phy_ops eusb2_repeater_ops = {
    .init		= eusb2_repeater_init,
    .exit		= eusb2_repeater_exit,
    .set_mode	= eusb2_repeater_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn eusb2_repeater_probe(pdev: *mut platform_device) -> c_int {
    static int eusb2_repeater_probe(struct platform_device *pdev)
    {
    struct eusb2_repeater *rptr;
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct device_node *np = dev.of_node;
    u32 res;
    int ret;
    rptr = devm_kzalloc(dev, sizeof(*rptr), GFP_KERNEL);
    if (!rptr)
    return -ENOMEM;
    rptr.dev = dev;
    dev_set_drvdata(dev, rptr);
    rptr.cfg = of_device_get_match_data(dev);
    if (!rptr.cfg)
    return -EINVAL;
    rptr.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!rptr.regmap)
    return -ENODEV;
    ret = of_property_read_u32(np, "reg", &res);
    if (ret < 0)
    return ret;
    rptr.base = res;
    ret = eusb2_repeater_init_vregs(rptr);
    if (ret < 0) {
    dev_err(dev, "unable to get supplies\n");
    return ret;
    }
    rptr.phy = devm_phy_create(dev, np, &eusb2_repeater_ops);
    if (IS_ERR(rptr.phy)) {
    dev_err(dev, "failed to create PHY: %d\n", ret);
    return PTR_ERR(rptr.phy);
    }
    phy_set_drvdata(rptr.phy, rptr);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eusb2_repeater_remove(pdev: *mut platform_device) {
    static void eusb2_repeater_remove(struct platform_device *pdev)
    {
    struct eusb2_repeater *rptr = platform_get_drvdata(pdev);
    if (!rptr)
    return;
    eusb2_repeater_exit(rptr.phy);
    }
    static const struct of_device_id eusb2_repeater_of_match_table[] = {
    {
    .compatible = "qcom,pm8550b-eusb2-repeater",
    .data = &pm8550b_eusb2_cfg,
    },
    {
    .compatible = "qcom,pmiv0104-eusb2-repeater",
    .data = &pmiv0104_eusb2_cfg,
    },
    {
    .compatible = "qcom,smb2360-eusb2-repeater",
    .data = &smb2360_eusb2_cfg,
    },
    {
    .compatible = "qcom,smb2370-eusb2-repeater",
    .data = &smb2370_eusb2_cfg,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, eusb2_repeater_of_match_table);
    static struct platform_driver eusb2_repeater_driver = {
    .probe		= eusb2_repeater_probe,
    .remove		= eusb2_repeater_remove,
    .driver = {
    .name	= "qcom-eusb2-repeater",
    .of_match_table = eusb2_repeater_of_match_table,
    },
    };
    module_platform_driver(eusb2_repeater_driver);
    MODULE_DESCRIPTION("Qualcomm PMIC eUSB2 Repeater driver");
    MODULE_LICENSE("GPL");
