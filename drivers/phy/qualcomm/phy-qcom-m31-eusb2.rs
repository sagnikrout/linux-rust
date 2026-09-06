//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-m31-eusb2.c
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
// Copyright (c) 2024-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

    {				\
    .off = o,		\
    .mask = b,		\
    .val = v,		\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31_phy_tbl_entry {
    pub off: u32,
    pub mask: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31_eusb2_priv_data {
    pub setup_seq: *const m31_phy_tbl_entry,
    pub setup_seq_nregs: c_uint,
    pub override_seq: *const m31_phy_tbl_entry,
    pub override_seq_nregs: c_uint,
    pub reset_seq: *const m31_phy_tbl_entry,
    pub reset_seq_nregs: c_uint,
    pub fsel: c_uint,
}

    static const struct m31_phy_tbl_entry m31_eusb2_setup_tbl[] = {
    M31_EUSB_PHY_INIT_CFG(USB_PHY_CFG0, UTMI_PHY_CMN_CTRL_OVERRIDE_EN, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_UTMI_CTRL5, POR, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL_COMMON0, PHY_ENABLE, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_CFG1, PLL_EN, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_FSEL_SEL, FSEL_SEL, 1),
    };
    static const struct m31_phy_tbl_entry m31_eusb_phy_override_tbl[] = {
    M31_EUSB_PHY_INIT_CFG(USB_PHY_XCFGI_39_32, HSTX_PE, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_XCFGI_71_64, HSTX_SWING, 7),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_XCFGI_31_24, HSTX_SLEW, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_XCFGI_7_0, PLL_LOCK_TIME, 0),
    };
    static const struct m31_phy_tbl_entry m31_eusb_phy_reset_tbl[] = {
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL2, USB2_SUSPEND_N_SEL, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL2, USB2_SUSPEND_N, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_UTMI_CTRL0, SLEEPM, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL_COMMON0, SIDDQ_SEL, 1),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL_COMMON0, SIDDQ, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_UTMI_CTRL5, POR, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_HS_PHY_CTRL2, USB2_SUSPEND_N_SEL, 0),
    M31_EUSB_PHY_INIT_CFG(USB_PHY_CFG0, UTMI_PHY_CMN_CTRL_OVERRIDE_EN, 0),
    };
    static const struct regulator_bulk_data m31_eusb_phy_vregs[] = {
    { .supply = "vdd" },
    { .supply = "vdda12" },
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31eusb2_phy {
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub data: *const m31_eusb2_priv_data,
    pub mode: enum phy_mode,
    pub vregs: *mut regulator_bulk_data,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub repeater: *mut phy,
}

    static int m31eusb2_phy_write_readback(void __iomem *base, u32 offset,
    const u32 mask, u32 val)
    {
    u32 write_val;
    u32 tmp;
    tmp = readl(base + offset);
    tmp &= ~mask;
    write_val = tmp | val;
    writel(write_val, base + offset);
    tmp = readl(base + offset);
    tmp &= mask;
    if (tmp != val) {
    pr_err("write: %x to offset: %x FAILED\n", val, offset);
    return -EINVAL;
    }
    return 0;
    }
    static int m31eusb2_phy_write_sequence(struct m31eusb2_phy *phy,
    const struct m31_phy_tbl_entry *tbl,
    int num)
    {
    int i;
    int ret;
    for (i = 0 ; i < num; i++, tbl++) {
    dev_dbg(&phy.phy.dev, "Offset:%x BitMask:%x Value:%x",
    tbl.off, tbl.mask, tbl.val);
    ret = m31eusb2_phy_write_readback(phy.base,
    tbl.off, tbl.mask,
    tbl.val << __ffs(tbl.mask));
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m31eusb2_phy_set_mode(uphy: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int m31eusb2_phy_set_mode(struct phy *uphy, enum phy_mode mode, int submode)
    {
    struct m31eusb2_phy *phy = phy_get_drvdata(uphy);
    phy.mode = mode;
    return phy_set_mode_ext(phy.repeater, mode, submode);
    }
#[no_mangle]
unsafe extern "C" fn m31eusb2_phy_init(uphy: *mut phy) -> c_int {
    static int m31eusb2_phy_init(struct phy *uphy)
    {
    struct m31eusb2_phy *phy = phy_get_drvdata(uphy);
    const struct m31_eusb2_priv_data *data = phy.data;
    int ret;
    ret = regulator_bulk_enable(M31_EUSB_NUM_VREGS, phy.vregs);
    if (ret) {
    dev_err(&uphy.dev, "failed to enable regulator, %d\n", ret);
    return ret;
    }
    ret = phy_init(phy.repeater);
    if (ret) {
    dev_err(&uphy.dev, "repeater init failed. %d\n", ret);
    goto disable_vreg;
    }
    ret = clk_prepare_enable(phy.clk);
    if (ret) {
    dev_err(&uphy.dev, "failed to enable ref clock, %d\n", ret);
    goto disable_repeater;
    }
// Perform phy reset
    reset_control_assert(phy.reset);
    udelay(5);
    reset_control_deassert(phy.reset);
    m31eusb2_phy_write_sequence(phy, data.setup_seq, data.setup_seq_nregs);
    m31eusb2_phy_write_readback(phy.base,
    USB_PHY_HS_PHY_CTRL_COMMON0, FSEL,
    FIELD_PREP(FSEL, data.fsel));
    m31eusb2_phy_write_sequence(phy, data.override_seq, data.override_seq_nregs);
    m31eusb2_phy_write_sequence(phy, data.reset_seq, data.reset_seq_nregs);
    return 0;
    disable_repeater:
    phy_exit(phy.repeater);
    disable_vreg:
    regulator_bulk_disable(M31_EUSB_NUM_VREGS, phy.vregs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn m31eusb2_phy_exit(uphy: *mut phy) -> c_int {
    static int m31eusb2_phy_exit(struct phy *uphy)
    {
    struct m31eusb2_phy *phy = phy_get_drvdata(uphy);
    clk_disable_unprepare(phy.clk);
    regulator_bulk_disable(M31_EUSB_NUM_VREGS, phy.vregs);
    phy_exit(phy.repeater);
    return 0;
    }
    static const struct phy_ops m31eusb2_phy_gen_ops = {
    .init		= m31eusb2_phy_init,
    .exit		= m31eusb2_phy_exit,
    .set_mode	= m31eusb2_phy_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn m31eusb2_phy_probe(pdev: *mut platform_device) -> c_int {
    static int m31eusb2_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    const struct m31_eusb2_priv_data *data;
    struct device *dev = &pdev.dev;
    struct m31eusb2_phy *phy;
    int ret;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    data = device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    phy.data = data;
    phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.base))
    return PTR_ERR(phy.base);
    phy.reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(phy.reset))
    return PTR_ERR(phy.reset);
    phy.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(phy.clk))
    return dev_err_probe(dev, PTR_ERR(phy.clk),
    "failed to get clk\n");
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &m31eusb2_phy_gen_ops);
    if (IS_ERR(phy.phy))
    return dev_err_probe(dev, PTR_ERR(phy.phy),
    "failed to create phy\n");
    ret = devm_regulator_bulk_get_const(dev, M31_EUSB_NUM_VREGS,
    m31_eusb_phy_vregs, &phy.vregs);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get regulator supplies\n");
    phy_set_drvdata(phy.phy, phy);
    phy.repeater = devm_phy_optional_get(dev, core::ptr::null_mut());
    if (IS_ERR(phy.repeater))
    return dev_err_probe(dev, PTR_ERR(phy.repeater),
    "failed to get repeater\n");
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct m31_eusb2_priv_data m31_eusb_v1_data = {
    .setup_seq = m31_eusb2_setup_tbl,
    .setup_seq_nregs = ARRAY_SIZE(m31_eusb2_setup_tbl),
    .override_seq = m31_eusb_phy_override_tbl,
    .override_seq_nregs = ARRAY_SIZE(m31_eusb_phy_override_tbl),
    .reset_seq = m31_eusb_phy_reset_tbl,
    .reset_seq_nregs = ARRAY_SIZE(m31_eusb_phy_reset_tbl),
    .fsel = FSEL_38_4_MHZ_VAL,
    };
    static const struct of_device_id m31eusb2_phy_id_table[] = {
    { .compatible = "qcom,sm8750-m31-eusb2-phy", .data = &m31_eusb_v1_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, m31eusb2_phy_id_table);
    static struct platform_driver m31eusb2_phy_driver = {
    .probe = m31eusb2_phy_probe,
    .driver = {
    .name = "qcom-m31eusb2-phy",
    .of_match_table = m31eusb2_phy_id_table,
    },
    };
    module_platform_driver(m31eusb2_phy_driver);
    MODULE_AUTHOR("Wesley Cheng <quic_wcheng@quicinc.com>");
    MODULE_DESCRIPTION("eUSB2 Qualcomm M31 HSPHY driver");
    MODULE_LICENSE("GPL");
