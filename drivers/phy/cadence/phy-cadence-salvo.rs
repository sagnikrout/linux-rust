//! Automatically rewritten from C to Rust
//! Source: drivers/phy/cadence/phy-cadence-salvo.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Salvo PHY is a 28nm PHY, it is a legacy PHY, and only
// for USB3 and USB2.
//
// Copyright (c) 2019-2020 NXP
//

pub const USB3_PHY_OFFSET: c_uint = 0x0;
pub const USB2_PHY_OFFSET: c_uint = 0x38000;
// USB3 PHY register definition
pub const PHY_PMA_CMN_CTRL1: c_uint = 0xC800;
pub const TB_ADDR_CMN_DIAG_HSCLK_SEL: c_uint = 0x01e0;
pub const TB_ADDR_CMN_PLL0_VCOCAL_INIT_TMR: c_uint = 0x0084;
pub const TB_ADDR_CMN_PLL0_VCOCAL_ITER_TMR: c_uint = 0x0085;
pub const TB_ADDR_CMN_PLL0_INTDIV: c_uint = 0x0094;
pub const TB_ADDR_CMN_PLL0_FRACDIV: c_uint = 0x0095;
pub const TB_ADDR_CMN_PLL0_HIGH_THR: c_uint = 0x0096;
pub const TB_ADDR_CMN_PLL0_SS_CTRL1: c_uint = 0x0098;
pub const TB_ADDR_CMN_PLL0_SS_CTRL2: c_uint = 0x0099;
pub const TB_ADDR_CMN_PLL0_DSM_DIAG: c_uint = 0x0097;
pub const TB_ADDR_CMN_DIAG_PLL0_OVRD: c_uint = 0x01c2;
pub const TB_ADDR_CMN_DIAG_PLL0_FBH_OVRD: c_uint = 0x01c0;
pub const TB_ADDR_CMN_DIAG_PLL0_FBL_OVRD: c_uint = 0x01c1;
pub const TB_ADDR_CMN_DIAG_PLL0_V2I_TUNE: c_uint = 0x01C5;
pub const TB_ADDR_CMN_DIAG_PLL0_CP_TUNE: c_uint = 0x01C6;
pub const TB_ADDR_CMN_DIAG_PLL0_LF_PROG: c_uint = 0x01C7;
pub const TB_ADDR_CMN_DIAG_PLL0_TEST_MODE: c_uint = 0x01c4;
pub const TB_ADDR_CMN_PSM_CLK_CTRL: c_uint = 0x0061;
pub const TB_ADDR_XCVR_DIAG_RX_LANE_CAL_RST_TMR: c_uint = 0x40ea;
pub const TB_ADDR_XCVR_PSM_RCTRL: c_uint = 0x4001;
pub const TB_ADDR_TX_PSC_A0: c_uint = 0x4100;
pub const TB_ADDR_TX_PSC_A1: c_uint = 0x4101;
pub const TB_ADDR_TX_PSC_A2: c_uint = 0x4102;
pub const TB_ADDR_TX_PSC_A3: c_uint = 0x4103;
pub const TB_ADDR_TX_DIAG_ECTRL_OVRD: c_uint = 0x41f5;
pub const TB_ADDR_TX_PSC_CAL: c_uint = 0x4106;
pub const TB_ADDR_TX_PSC_RDY: c_uint = 0x4107;
pub const TB_ADDR_RX_PSC_A0: c_uint = 0x8000;
pub const TB_ADDR_RX_PSC_A1: c_uint = 0x8001;
pub const TB_ADDR_RX_PSC_A2: c_uint = 0x8002;
pub const TB_ADDR_RX_PSC_A3: c_uint = 0x8003;
pub const TB_ADDR_RX_PSC_CAL: c_uint = 0x8006;
pub const TB_ADDR_RX_PSC_RDY: c_uint = 0x8007;
pub const TB_ADDR_TX_TXCC_MGNLS_MULT_000: c_uint = 0x4058;
pub const TB_ADDR_TX_DIAG_BGREF_PREDRV_DELAY: c_uint = 0x41e7;
pub const TB_ADDR_RX_SLC_CU_ITER_TMR: c_uint = 0x80e3;
pub const TB_ADDR_RX_SIGDET_HL_FILT_TMR: c_uint = 0x8090;
pub const TB_ADDR_RX_SAMP_DAC_CTRL: c_uint = 0x8058;
pub const TB_ADDR_RX_DIAG_SIGDET_TUNE: c_uint = 0x81dc;
pub const TB_ADDR_RX_DIAG_LFPSDET_TUNE2: c_uint = 0x81df;
pub const TB_ADDR_RX_DIAG_BS_TM: c_uint = 0x81f5;
pub const TB_ADDR_RX_DIAG_DFE_CTRL1: c_uint = 0x81d3;
pub const TB_ADDR_RX_DIAG_ILL_IQE_TRIM4: c_uint = 0x81c7;
pub const TB_ADDR_RX_DIAG_ILL_E_TRIM0: c_uint = 0x81c2;
pub const TB_ADDR_RX_DIAG_ILL_IQ_TRIM0: c_uint = 0x81c1;
pub const TB_ADDR_RX_DIAG_ILL_IQE_TRIM6: c_uint = 0x81c9;
pub const TB_ADDR_RX_DIAG_RXFE_TM3: c_uint = 0x81f8;
pub const TB_ADDR_RX_DIAG_RXFE_TM4: c_uint = 0x81f9;
pub const TB_ADDR_RX_DIAG_LFPSDET_TUNE: c_uint = 0x81dd;
pub const TB_ADDR_RX_DIAG_DFE_CTRL3: c_uint = 0x81d5;
pub const TB_ADDR_RX_DIAG_SC2C_DELAY: c_uint = 0x81e1;
pub const TB_ADDR_RX_REE_VGA_GAIN_NODFE: c_uint = 0x81bf;
pub const TB_ADDR_XCVR_PSM_CAL_TMR: c_uint = 0x4002;
pub const TB_ADDR_XCVR_PSM_A0BYP_TMR: c_uint = 0x4004;
pub const TB_ADDR_XCVR_PSM_A0IN_TMR: c_uint = 0x4003;
pub const TB_ADDR_XCVR_PSM_A1IN_TMR: c_uint = 0x4005;
pub const TB_ADDR_XCVR_PSM_A2IN_TMR: c_uint = 0x4006;
pub const TB_ADDR_XCVR_PSM_A3IN_TMR: c_uint = 0x4007;
pub const TB_ADDR_XCVR_PSM_A4IN_TMR: c_uint = 0x4008;
pub const TB_ADDR_XCVR_PSM_A5IN_TMR: c_uint = 0x4009;
pub const TB_ADDR_XCVR_PSM_A0OUT_TMR: c_uint = 0x400a;
pub const TB_ADDR_XCVR_PSM_A1OUT_TMR: c_uint = 0x400b;
pub const TB_ADDR_XCVR_PSM_A2OUT_TMR: c_uint = 0x400c;
pub const TB_ADDR_XCVR_PSM_A3OUT_TMR: c_uint = 0x400d;
pub const TB_ADDR_XCVR_PSM_A4OUT_TMR: c_uint = 0x400e;
pub const TB_ADDR_XCVR_PSM_A5OUT_TMR: c_uint = 0x400f;
pub const TB_ADDR_TX_RCVDET_EN_TMR: c_uint = 0x4122;
pub const TB_ADDR_TX_RCVDET_ST_TMR: c_uint = 0x4123;
pub const TB_ADDR_XCVR_DIAG_LANE_FCM_EN_MGN_TMR: c_uint = 0x40f2;
pub const TB_ADDR_TX_RCVDETSC_CTRL: c_uint = 0x4124;
// USB2 PHY register definition
pub const UTMI_REG15: c_uint = 0xaf;
pub const UTMI_AFE_RX_REG0: c_uint = 0x0d;
pub const UTMI_AFE_RX_REG5: c_uint = 0x12;
pub const UTMI_AFE_BC_REG4: c_uint = 0x29;
// Align UTMI_AFE_RX_REG0 bit[7:6] define
    enum usb2_disconn_threshold {
    USB2_DISCONN_THRESHOLD_575 = 0x0,
    USB2_DISCONN_THRESHOLD_610 = 0x1,
    USB2_DISCONN_THRESHOLD_645 = 0x3,
    };

// TB_ADDR_TX_RCVDETSC_CTRL

//
// UTMI_REG15
//
// Gate how many us for the txvalid signal until analog
// HS/FS transmitters have powered up
//

// 0us, txvalid is ready just after HS/FS transmitters have powered up

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_reg_pairs {
    pub val: u16,
    pub off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_salvo_data {
    pub reg_offset_shift: u8,
    pub init_sequence_val: *const cdns_reg_pairs,
    pub init_sequence_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_salvo_phy {
    pub phy: *mut phy,
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub data: *mut cdns_salvo_data,
    pub usb2_disconn: enum usb2_disconn_threshold,
}

    static const struct of_device_id cdns_salvo_phy_of_match[];
    static const struct cdns_salvo_data cdns_nxp_salvo_data;
#[no_mangle]
unsafe extern "C" fn cdns_is_nxp_phy(salvo_phy: *mut cdns_salvo_phy) -> bool {
    static bool cdns_is_nxp_phy(struct cdns_salvo_phy *salvo_phy)
    {
    return salvo_phy.data == &cdns_nxp_salvo_data;
    }
#[no_mangle]
unsafe extern "C" fn cdns_salvo_read(salvo_phy: *mut cdns_salvo_phy, offset: u32, reg: u32) -> u16 {
    static u16 cdns_salvo_read(struct cdns_salvo_phy *salvo_phy, u32 offset, u32 reg)
    {
    return (u16)readl(salvo_phy.base + offset +
    reg * (1 << salvo_phy.data.reg_offset_shift));
    }
    static void cdns_salvo_write(struct cdns_salvo_phy *salvo_phy, u32 offset,
    u32 reg, u16 val)
    {
    writel(val, salvo_phy.base + offset +
    reg * (1 << salvo_phy.data.reg_offset_shift));
    }
//
// Below bringup sequence pair are from Cadence PHY's User Guide
// and NXP platform tuning results.
//
    static const struct cdns_reg_pairs cdns_nxp_sequence_pair[] = {
    {0x0830, PHY_PMA_CMN_CTRL1},
    {0x0010, TB_ADDR_CMN_DIAG_HSCLK_SEL},
    {0x00f0, TB_ADDR_CMN_PLL0_VCOCAL_INIT_TMR},
    {0x0018, TB_ADDR_CMN_PLL0_VCOCAL_ITER_TMR},
    {0x00d0, TB_ADDR_CMN_PLL0_INTDIV},
    {0x4aaa, TB_ADDR_CMN_PLL0_FRACDIV},
    {0x0034, TB_ADDR_CMN_PLL0_HIGH_THR},
    {0x01ee, TB_ADDR_CMN_PLL0_SS_CTRL1},
    {0x7f03, TB_ADDR_CMN_PLL0_SS_CTRL2},
    {0x0020, TB_ADDR_CMN_PLL0_DSM_DIAG},
    {0x0000, TB_ADDR_CMN_DIAG_PLL0_OVRD},
    {0x0000, TB_ADDR_CMN_DIAG_PLL0_FBH_OVRD},
    {0x0000, TB_ADDR_CMN_DIAG_PLL0_FBL_OVRD},
    {0x0007, TB_ADDR_CMN_DIAG_PLL0_V2I_TUNE},
    {0x0027, TB_ADDR_CMN_DIAG_PLL0_CP_TUNE},
    {0x0008, TB_ADDR_CMN_DIAG_PLL0_LF_PROG},
    {0x0022, TB_ADDR_CMN_DIAG_PLL0_TEST_MODE},
    {0x000a, TB_ADDR_CMN_PSM_CLK_CTRL},
    {0x0139, TB_ADDR_XCVR_DIAG_RX_LANE_CAL_RST_TMR},
    {0xbefc, TB_ADDR_XCVR_PSM_RCTRL},
    {0x7799, TB_ADDR_TX_PSC_A0},
    {0x7798, TB_ADDR_TX_PSC_A1},
    {0x509b, TB_ADDR_TX_PSC_A2},
    {0x0003, TB_ADDR_TX_DIAG_ECTRL_OVRD},
    {0x509b, TB_ADDR_TX_PSC_A3},
    {0x2090, TB_ADDR_TX_PSC_CAL},
    {0x2090, TB_ADDR_TX_PSC_RDY},
    {0xA6FD, TB_ADDR_RX_PSC_A0},
    {0xA6FD, TB_ADDR_RX_PSC_A1},
    {0xA410, TB_ADDR_RX_PSC_A2},
    {0x2410, TB_ADDR_RX_PSC_A3},
    {0x23FF, TB_ADDR_RX_PSC_CAL},
    {0x2010, TB_ADDR_RX_PSC_RDY},
    {0x0020, TB_ADDR_TX_TXCC_MGNLS_MULT_000},
    {0x00ff, TB_ADDR_TX_DIAG_BGREF_PREDRV_DELAY},
    {0x0002, TB_ADDR_RX_SLC_CU_ITER_TMR},
    {0x0013, TB_ADDR_RX_SIGDET_HL_FILT_TMR},
    {0x0000, TB_ADDR_RX_SAMP_DAC_CTRL},
    {0x1004, TB_ADDR_RX_DIAG_SIGDET_TUNE},
    {0x4041, TB_ADDR_RX_DIAG_LFPSDET_TUNE2},
    {0x0480, TB_ADDR_RX_DIAG_BS_TM},
    {0x8006, TB_ADDR_RX_DIAG_DFE_CTRL1},
    {0x003f, TB_ADDR_RX_DIAG_ILL_IQE_TRIM4},
    {0x543f, TB_ADDR_RX_DIAG_ILL_E_TRIM0},
    {0x543f, TB_ADDR_RX_DIAG_ILL_IQ_TRIM0},
    {0x0000, TB_ADDR_RX_DIAG_ILL_IQE_TRIM6},
    {0x8000, TB_ADDR_RX_DIAG_RXFE_TM3},
    {0x0003, TB_ADDR_RX_DIAG_RXFE_TM4},
    {0x2408, TB_ADDR_RX_DIAG_LFPSDET_TUNE},
    {0x05ca, TB_ADDR_RX_DIAG_DFE_CTRL3},
    {0x0258, TB_ADDR_RX_DIAG_SC2C_DELAY},
    {0x1fff, TB_ADDR_RX_REE_VGA_GAIN_NODFE},
    {0x02c6, TB_ADDR_XCVR_PSM_CAL_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A0BYP_TMR},
    {0x02c6, TB_ADDR_XCVR_PSM_A0IN_TMR},
    {0x0010, TB_ADDR_XCVR_PSM_A1IN_TMR},
    {0x0010, TB_ADDR_XCVR_PSM_A2IN_TMR},
    {0x0010, TB_ADDR_XCVR_PSM_A3IN_TMR},
    {0x0010, TB_ADDR_XCVR_PSM_A4IN_TMR},
    {0x0010, TB_ADDR_XCVR_PSM_A5IN_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A0OUT_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A1OUT_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A2OUT_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A3OUT_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A4OUT_TMR},
    {0x0002, TB_ADDR_XCVR_PSM_A5OUT_TMR},
// Change rx detect parameter
    {0x0960, TB_ADDR_TX_RCVDET_EN_TMR},
    {0x01e0, TB_ADDR_TX_RCVDET_ST_TMR},
    {0x0090, TB_ADDR_XCVR_DIAG_LANE_FCM_EN_MGN_TMR},
    };
#[no_mangle]
unsafe extern "C" fn cdns_salvo_phy_init(phy: *mut phy) -> c_int {
    static int cdns_salvo_phy_init(struct phy *phy)
    {
    struct cdns_salvo_phy *salvo_phy = phy_get_drvdata(phy);
    struct cdns_salvo_data *data = salvo_phy.data;
    int ret, i;
    u16 value;
    ret = clk_prepare_enable(salvo_phy.clk);
    if (ret)
    return ret;
    for (i = 0; i < data.init_sequence_length; i++) {
    const struct cdns_reg_pairs *reg_pair = data.init_sequence_val + i;
    cdns_salvo_write(salvo_phy, USB3_PHY_OFFSET, reg_pair.off, reg_pair.val);
    }
// RXDET_IN_P3_32KHZ, Receiver detect slow clock enable
    value = cdns_salvo_read(salvo_phy, USB3_PHY_OFFSET, TB_ADDR_TX_RCVDETSC_CTRL);
    value |= RXDET_IN_P3_32KHZ;
    cdns_salvo_write(salvo_phy, USB3_PHY_OFFSET, TB_ADDR_TX_RCVDETSC_CTRL,
    RXDET_IN_P3_32KHZ);
    value = cdns_salvo_read(salvo_phy, USB2_PHY_OFFSET, UTMI_REG15);
    value &= ~TXVALID_GATE_THRESHOLD_HS_MASK;
    cdns_salvo_write(salvo_phy, USB2_PHY_OFFSET, UTMI_REG15,
    value | TXVALID_GATE_THRESHOLD_HS_0US);
    cdns_salvo_write(salvo_phy, USB2_PHY_OFFSET, UTMI_AFE_RX_REG5, 0x5);
    value = cdns_salvo_read(salvo_phy, USB2_PHY_OFFSET, UTMI_AFE_RX_REG0);
    value &= ~RX_USB2_DISCONN_MASK;
    value = FIELD_PREP(RX_USB2_DISCONN_MASK, salvo_phy.usb2_disconn);
    cdns_salvo_write(salvo_phy, USB2_PHY_OFFSET, UTMI_AFE_RX_REG0, value);
    udelay(10);
    clk_disable_unprepare(salvo_phy.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_salvo_phy_power_on(phy: *mut phy) -> c_int {
    static int cdns_salvo_phy_power_on(struct phy *phy)
    {
    struct cdns_salvo_phy *salvo_phy = phy_get_drvdata(phy);
    return clk_prepare_enable(salvo_phy.clk);
    }
#[no_mangle]
unsafe extern "C" fn cdns_salvo_phy_power_off(phy: *mut phy) -> c_int {
    static int cdns_salvo_phy_power_off(struct phy *phy)
    {
    struct cdns_salvo_phy *salvo_phy = phy_get_drvdata(phy);
    clk_disable_unprepare(salvo_phy.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_salvo_set_mode(phy: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int cdns_salvo_set_mode(struct phy *phy, enum phy_mode mode, int submode)
    {
    struct cdns_salvo_phy *salvo_phy = phy_get_drvdata(phy);
    if (!cdns_is_nxp_phy(salvo_phy))
    return 0;
    if (mode == PHY_MODE_USB_DEVICE)
    cdns_salvo_write(salvo_phy, USB2_PHY_OFFSET, UTMI_AFE_BC_REG4,
    SET_B_SESSION_VALID);
    else
    cdns_salvo_write(salvo_phy, USB2_PHY_OFFSET, UTMI_AFE_BC_REG4,
    CLR_B_SESSION_VALID);
    return 0;
    }
    static const struct phy_ops cdns_salvo_phy_ops = {
    .init		= cdns_salvo_phy_init,
    .power_on	= cdns_salvo_phy_power_on,
    .power_off	= cdns_salvo_phy_power_off,
    .owner		= THIS_MODULE,
    .set_mode	= cdns_salvo_set_mode,
    };
#[no_mangle]
unsafe extern "C" fn cdns_salvo_phy_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_salvo_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct cdns_salvo_phy *salvo_phy;
    struct cdns_salvo_data *data;
    u32 val;
    data = (struct cdns_salvo_data *)of_device_get_match_data(dev);
    salvo_phy = devm_kzalloc(dev, sizeof(*salvo_phy), GFP_KERNEL);
    if (!salvo_phy)
    return -ENOMEM;
    salvo_phy.data = data;
    salvo_phy.clk = devm_clk_get_optional(dev, "salvo_phy_clk");
    if (IS_ERR(salvo_phy.clk))
    return PTR_ERR(salvo_phy.clk);
    if (of_property_read_u32(dev.of_node, "cdns,usb2-disconnect-threshold-microvolt", &val))
    val = 575;
    if (val < 610)
    salvo_phy.usb2_disconn = USB2_DISCONN_THRESHOLD_575;
#[no_mangle]
pub unsafe extern "C" fn if(645: val <) -> else {
    else if (val < 645)
    salvo_phy.usb2_disconn = USB2_DISCONN_THRESHOLD_610;
    else
    salvo_phy.usb2_disconn = USB2_DISCONN_THRESHOLD_645;
    salvo_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(salvo_phy.base))
    return PTR_ERR(salvo_phy.base);
    salvo_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &cdns_salvo_phy_ops);
    if (IS_ERR(salvo_phy.phy))
    return PTR_ERR(salvo_phy.phy);
    phy_set_drvdata(salvo_phy.phy, salvo_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct cdns_salvo_data cdns_nxp_salvo_data = {
    2,
    cdns_nxp_sequence_pair,
    ARRAY_SIZE(cdns_nxp_sequence_pair),
    };
    static const struct of_device_id cdns_salvo_phy_of_match[] = {
    {
    .compatible = "nxp,salvo-phy",
    .data = &cdns_nxp_salvo_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, cdns_salvo_phy_of_match);
    static struct platform_driver cdns_salvo_phy_driver = {
    .probe	= cdns_salvo_phy_probe,
    .driver = {
    .name	= "cdns-salvo-phy",
    .of_match_table	= cdns_salvo_phy_of_match,
    }
    };
    module_platform_driver(cdns_salvo_phy_driver);
    MODULE_AUTHOR("Peter Chen <peter.chen@nxp.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence SALVO PHY Driver");
