//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ti/phy-am654-serdes.c
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
// PCIe SERDES driver for AM654x SoC
//
// Copyright (C) 2018 - 2019 Texas Instruments Incorporated - http://www.ti.com
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const CMU_R004: c_uint = 0x4;
pub const CMU_R060: c_uint = 0x60;
pub const CMU_R07C: c_uint = 0x7c;
pub const CMU_R088: c_uint = 0x88;
pub const CMU_R0D0: c_uint = 0xd0;
pub const CMU_R0E8: c_uint = 0xe8;
pub const LANE_R048: c_uint = 0x248;
pub const LANE_R058: c_uint = 0x258;
pub const LANE_R06c: c_uint = 0x26c;
pub const LANE_R070: c_uint = 0x270;
pub const LANE_R19C: c_uint = 0x39c;
pub const COMLANE_R004: c_uint = 0xa04;
pub const COMLANE_R138: c_uint = 0xb38;
pub const VERSION_VAL: c_uint = 0x70;
pub const COMLANE_R190: c_uint = 0xb90;
pub const COMLANE_R194: c_uint = 0xb94;
pub const COMRXEQ_R004: c_uint = 0x1404;
pub const COMRXEQ_R008: c_uint = 0x1408;
pub const COMRXEQ_R00C: c_uint = 0x140c;
pub const COMRXEQ_R014: c_uint = 0x1414;
pub const COMRXEQ_R018: c_uint = 0x1418;
pub const COMRXEQ_R01C: c_uint = 0x141c;
pub const COMRXEQ_R04C: c_uint = 0x144c;
pub const COMRXEQ_R088: c_uint = 0x1488;
pub const COMRXEQ_R094: c_uint = 0x1494;
pub const COMRXEQ_R098: c_uint = 0x1498;
pub const SERDES_CTRL: c_uint = 0x1fd0;
pub const WIZ_LANEXCTL_STS: c_uint = 0x1fe0;
pub const TX0_DISABLE_STATE: c_uint = 0x4;
pub const TX0_SLEEP_STATE: c_uint = 0x5;
pub const TX0_SNOOZE_STATE: c_uint = 0x6;
pub const TX0_ENABLE_STATE: c_uint = 0x7;
pub const RX0_DISABLE_STATE: c_uint = 0x4;
pub const RX0_SLEEP_STATE: c_uint = 0x5;
pub const RX0_SNOOZE_STATE: c_uint = 0x6;
pub const RX0_ENABLE_STATE: c_uint = 0x7;
pub const WIZ_PLL_CTRL: c_uint = 0x1ff4;
pub const PLL_DISABLE_STATE: c_uint = 0x4;
pub const PLL_SLEEP_STATE: c_uint = 0x5;
pub const PLL_SNOOZE_STATE: c_uint = 0x6;
pub const PLL_ENABLE_STATE: c_uint = 0x7;

pub const LANE_USB3: c_uint = 0x0;
pub const LANE_PCIE0_LANE0: c_uint = 0x1;
pub const LANE_PCIE1_LANE0: c_uint = 0x0;
pub const LANE_PCIE0_LANE1: c_uint = 0x1;
pub const SERDES_NUM_CLOCKS: c_int = 3;

pub const AM654_SERDES_CTRL_CLKSEL_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdes_am654_clk_mux {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub clk_id: c_int,
    pub clk_data: clk_init_data,
}

    container_of(_hw, struct serdes_am654_clk_mux, hw)
    static const struct regmap_config serdes_am654_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 0x1ffc,
    };
    enum serdes_am654_fields {
// CMU PLL Control
    CMU_PLL_CTRL,
    LANE_PLL_CTRL_RXEQ_RXIDLE,
// CMU VCO bias current and VREG setting
    AHB_PMA_CM_VCO_VBIAS_VREG,
    AHB_PMA_CM_VCO_BIAS_VREG,
    AHB_PMA_CM_SR,
    AHB_SSC_GEN_Z_O_20_13,
// AHB PMA Lane Configuration
    AHB_PMA_LN_AGC_THSEL_VREGH,
// AGC and Signal detect threshold for Gen3
    AHB_PMA_LN_GEN3_AGC_SD_THSEL,
    AHB_PMA_LN_RX_SELR_GEN3,
    AHB_PMA_LN_TX_DRV,
// CMU Master Reset
    CMU_MASTER_CDN,
// P2S ring buffer initial startup pointer difference
    P2S_RBUF_PTR_DIFF,
    CONFIG_VERSION,
// Lane 1 Master Reset
    L1_MASTER_CDN,
// CMU OK Status
    CMU_OK_I_0,
// Mid-speed initial calibration control
    COMRXEQ_MS_INIT_CTRL_7_0,
// High-speed initial calibration control
    COMRXEQ_HS_INIT_CAL_7_0,
// Mid-speed recalibration control
    COMRXEQ_MS_RECAL_CTRL_7_0,
// High-speed recalibration control
    COMRXEQ_HS_RECAL_CTRL_7_0,
// ATT configuration
    COMRXEQ_CSR_ATT_CONFIG,
// Edge based boost adaptation window length
    COMRXEQ_CSR_EBSTADAPT_WIN_LEN,
// COMRXEQ control 3 & 4
    COMRXEQ_CTRL_3_4,
// COMRXEQ control 14, 15 and 16
    COMRXEQ_CTRL_14_15_16,
// Threshold for errors in pattern data
    COMRXEQ_CSR_DLEV_ERR_THRESH,
// COMRXEQ control 25
    COMRXEQ_CTRL_25,
// Mid-speed rate change calibration control
    CSR_RXEQ_RATE_CHANGE_CAL_RUN_RATE2_O,
// High-speed rate change calibration control
    COMRXEQ_HS_RCHANGE_CTRL_7_0,
// Serdes reset
    POR_EN,
// Tx Enable Value
    TX0_ENABLE,
// Rx Enable Value
    RX0_ENABLE,
// PLL Enable Value
    PLL_ENABLE,
// PLL ready for use
    PLL_OK,
// sentinel
    MAX_FIELDS
    };
    static const struct reg_field serdes_am654_reg_fields[] = {
    [CMU_PLL_CTRL]			= REG_FIELD(CMU_R004, 8, 15),
    [AHB_PMA_CM_VCO_VBIAS_VREG]	= REG_FIELD(CMU_R060, 8, 15),
    [CMU_MASTER_CDN]		= REG_FIELD(CMU_R07C, 24, 31),
    [AHB_PMA_CM_VCO_BIAS_VREG]	= REG_FIELD(CMU_R088, 24, 31),
    [AHB_PMA_CM_SR]			= REG_FIELD(CMU_R0D0, 24, 31),
    [AHB_SSC_GEN_Z_O_20_13]		= REG_FIELD(CMU_R0E8, 8, 15),
    [LANE_PLL_CTRL_RXEQ_RXIDLE]	= REG_FIELD(LANE_R048, 8, 15),
    [AHB_PMA_LN_AGC_THSEL_VREGH]	= REG_FIELD(LANE_R058, 16, 23),
    [AHB_PMA_LN_GEN3_AGC_SD_THSEL]	= REG_FIELD(LANE_R06c, 0, 7),
    [AHB_PMA_LN_RX_SELR_GEN3]	= REG_FIELD(LANE_R070, 16, 23),
    [AHB_PMA_LN_TX_DRV]		= REG_FIELD(LANE_R19C, 16, 23),
    [P2S_RBUF_PTR_DIFF]		= REG_FIELD(COMLANE_R004, 0, 7),
    [CONFIG_VERSION]		= REG_FIELD(COMLANE_R138, 16, 23),
    [L1_MASTER_CDN]			= REG_FIELD(COMLANE_R190, 8, 15),
    [CMU_OK_I_0]			= REG_FIELD(COMLANE_R194, 19, 19),
    [COMRXEQ_MS_INIT_CTRL_7_0]	= REG_FIELD(COMRXEQ_R004, 24, 31),
    [COMRXEQ_HS_INIT_CAL_7_0]	= REG_FIELD(COMRXEQ_R008, 0, 7),
    [COMRXEQ_MS_RECAL_CTRL_7_0]	= REG_FIELD(COMRXEQ_R00C, 8, 15),
    [COMRXEQ_HS_RECAL_CTRL_7_0]	= REG_FIELD(COMRXEQ_R00C, 16, 23),
    [COMRXEQ_CSR_ATT_CONFIG]	= REG_FIELD(COMRXEQ_R014, 16, 23),
    [COMRXEQ_CSR_EBSTADAPT_WIN_LEN]	= REG_FIELD(COMRXEQ_R018, 16, 23),
    [COMRXEQ_CTRL_3_4]		= REG_FIELD(COMRXEQ_R01C, 8, 15),
    [COMRXEQ_CTRL_14_15_16]		= REG_FIELD(COMRXEQ_R04C, 0, 7),
    [COMRXEQ_CSR_DLEV_ERR_THRESH]	= REG_FIELD(COMRXEQ_R088, 16, 23),
    [COMRXEQ_CTRL_25]		= REG_FIELD(COMRXEQ_R094, 24, 31),
    [CSR_RXEQ_RATE_CHANGE_CAL_RUN_RATE2_O] = REG_FIELD(COMRXEQ_R098, 8, 15),
    [COMRXEQ_HS_RCHANGE_CTRL_7_0]	= REG_FIELD(COMRXEQ_R098, 16, 23),
    [POR_EN]			= REG_FIELD(SERDES_CTRL, 29, 29),
    [TX0_ENABLE]			= REG_FIELD(WIZ_LANEXCTL_STS, 29, 31),
    [RX0_ENABLE]			= REG_FIELD(WIZ_LANEXCTL_STS, 13, 15),
    [PLL_ENABLE]			= REG_FIELD(WIZ_PLL_CTRL, 29, 31),
    [PLL_OK]			= REG_FIELD(WIZ_PLL_CTRL, 28, 28),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdes_am654 {
    pub regmap: *mut regmap,
    pub fields: [*mut regmap_field; MAX_FIELDS],
    pub dev: *mut device,
    pub control: *mut mux_control,
    pub busy: bool,
    pub type: u32,
    pub of_node: *mut device_node,
    pub clk_data: clk_onecell_data,
    pub clks: [*mut clk; SERDES_NUM_CLOCKS],
}

#[no_mangle]
unsafe extern "C" fn serdes_am654_enable_pll(phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_enable_pll(struct serdes_am654 *phy)
    {
    int ret;
    u32 val;
    ret = regmap_field_write(phy.fields[PLL_ENABLE], PLL_ENABLE_STATE);
    if (ret)
    return ret;
    return regmap_field_read_poll_timeout(phy.fields[PLL_OK], val, val,
    1000, PLL_LOCK_TIME);
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_disable_pll(phy: *mut serdes_am654) {
    static void serdes_am654_disable_pll(struct serdes_am654 *phy)
    {
    struct device *dev = phy.dev;
    int ret;
    ret = regmap_field_write(phy.fields[PLL_ENABLE], PLL_DISABLE_STATE);
    if (ret)
    dev_err(dev, "Failed to disable PLL\n");
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_enable_txrx(phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_enable_txrx(struct serdes_am654 *phy)
    {
    let mut ret: c_int = 0;
// Enable TX
    ret |= regmap_field_write(phy.fields[TX0_ENABLE], TX0_ENABLE_STATE);
// Enable RX
    ret |= regmap_field_write(phy.fields[RX0_ENABLE], RX0_ENABLE_STATE);
    if (ret)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_disable_txrx(phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_disable_txrx(struct serdes_am654 *phy)
    {
    let mut ret: c_int = 0;
// Disable TX
    ret |= regmap_field_write(phy.fields[TX0_ENABLE], TX0_DISABLE_STATE);
// Disable RX
    ret |= regmap_field_write(phy.fields[RX0_ENABLE], RX0_DISABLE_STATE);
    if (ret)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_power_on(x: *mut phy) -> c_int {
    static int serdes_am654_power_on(struct phy *x)
    {
    struct serdes_am654 *phy = phy_get_drvdata(x);
    struct device *dev = phy.dev;
    int ret;
    u32 val;
    ret = serdes_am654_enable_pll(phy);
    if (ret) {
    dev_err(dev, "Failed to enable PLL\n");
    return ret;
    }
    ret = serdes_am654_enable_txrx(phy);
    if (ret) {
    dev_err(dev, "Failed to enable TX RX\n");
    return ret;
    }
    return regmap_field_read_poll_timeout(phy.fields[CMU_OK_I_0], val,
    val, SLEEP_TIME, PLL_LOCK_TIME);
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_power_off(x: *mut phy) -> c_int {
    static int serdes_am654_power_off(struct phy *x)
    {
    struct serdes_am654 *phy = phy_get_drvdata(x);
    serdes_am654_disable_txrx(phy);
    serdes_am654_disable_pll(phy);
    return 0;
    }

    regmap_update_bits(phy.regmap, (offset),\
    GENMASK((a), (b)), (val) << (b))
#[no_mangle]
unsafe extern "C" fn serdes_am654_usb3_init(phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_usb3_init(struct serdes_am654 *phy)
    {
    SERDES_AM654_CFG(0x0000, 31, 24, 0x17);
    SERDES_AM654_CFG(0x0004, 15, 8, 0x02);
    SERDES_AM654_CFG(0x0004, 7, 0, 0x0e);
    SERDES_AM654_CFG(0x0008, 23, 16, 0x2e);
    SERDES_AM654_CFG(0x0008, 31, 24, 0x2e);
    SERDES_AM654_CFG(0x0060, 7, 0, 0x4b);
    SERDES_AM654_CFG(0x0060, 15, 8, 0x98);
    SERDES_AM654_CFG(0x0060, 23, 16, 0x60);
    SERDES_AM654_CFG(0x00d0, 31, 24, 0x45);
    SERDES_AM654_CFG(0x00e8, 15, 8, 0x0e);
    SERDES_AM654_CFG(0x0220, 7, 0, 0x34);
    SERDES_AM654_CFG(0x0220, 15, 8, 0x34);
    SERDES_AM654_CFG(0x0220, 31, 24, 0x37);
    SERDES_AM654_CFG(0x0224, 7, 0, 0x37);
    SERDES_AM654_CFG(0x0224, 15, 8, 0x37);
    SERDES_AM654_CFG(0x0228, 23, 16, 0x37);
    SERDES_AM654_CFG(0x0228, 31, 24, 0x37);
    SERDES_AM654_CFG(0x022c, 7, 0, 0x37);
    SERDES_AM654_CFG(0x022c, 15, 8, 0x37);
    SERDES_AM654_CFG(0x0230, 15, 8, 0x2a);
    SERDES_AM654_CFG(0x0230, 23, 16, 0x2a);
    SERDES_AM654_CFG(0x0240, 23, 16, 0x10);
    SERDES_AM654_CFG(0x0240, 31, 24, 0x34);
    SERDES_AM654_CFG(0x0244, 7, 0, 0x40);
    SERDES_AM654_CFG(0x0244, 23, 16, 0x34);
    SERDES_AM654_CFG(0x0248, 15, 8, 0x0d);
    SERDES_AM654_CFG(0x0258, 15, 8, 0x16);
    SERDES_AM654_CFG(0x0258, 23, 16, 0x84);
    SERDES_AM654_CFG(0x0258, 31, 24, 0xf2);
    SERDES_AM654_CFG(0x025c, 7, 0, 0x21);
    SERDES_AM654_CFG(0x0260, 7, 0, 0x27);
    SERDES_AM654_CFG(0x0260, 15, 8, 0x04);
    SERDES_AM654_CFG(0x0268, 15, 8, 0x04);
    SERDES_AM654_CFG(0x0288, 15, 8, 0x2c);
    SERDES_AM654_CFG(0x0330, 31, 24, 0xa0);
    SERDES_AM654_CFG(0x0338, 23, 16, 0x03);
    SERDES_AM654_CFG(0x0338, 31, 24, 0x00);
    SERDES_AM654_CFG(0x033c, 7, 0, 0x00);
    SERDES_AM654_CFG(0x0344, 31, 24, 0x18);
    SERDES_AM654_CFG(0x034c, 7, 0, 0x18);
    SERDES_AM654_CFG(0x039c, 23, 16, 0x3b);
    SERDES_AM654_CFG(0x0a04, 7, 0, 0x03);
    SERDES_AM654_CFG(0x0a14, 31, 24, 0x3c);
    SERDES_AM654_CFG(0x0a18, 15, 8, 0x3c);
    SERDES_AM654_CFG(0x0a38, 7, 0, 0x3e);
    SERDES_AM654_CFG(0x0a38, 15, 8, 0x3e);
    SERDES_AM654_CFG(0x0ae0, 7, 0, 0x07);
    SERDES_AM654_CFG(0x0b6c, 23, 16, 0xcd);
    SERDES_AM654_CFG(0x0b6c, 31, 24, 0x04);
    SERDES_AM654_CFG(0x0b98, 23, 16, 0x03);
    SERDES_AM654_CFG(0x1400, 7, 0, 0x3f);
    SERDES_AM654_CFG(0x1404, 23, 16, 0x6f);
    SERDES_AM654_CFG(0x1404, 31, 24, 0x6f);
    SERDES_AM654_CFG(0x140c, 7, 0, 0x6f);
    SERDES_AM654_CFG(0x140c, 15, 8, 0x6f);
    SERDES_AM654_CFG(0x1410, 15, 8, 0x27);
    SERDES_AM654_CFG(0x1414, 7, 0, 0x0c);
    SERDES_AM654_CFG(0x1414, 23, 16, 0x07);
    SERDES_AM654_CFG(0x1418, 23, 16, 0x40);
    SERDES_AM654_CFG(0x141c, 7, 0, 0x00);
    SERDES_AM654_CFG(0x141c, 15, 8, 0x1f);
    SERDES_AM654_CFG(0x1428, 31, 24, 0x08);
    SERDES_AM654_CFG(0x1434, 31, 24, 0x00);
    SERDES_AM654_CFG(0x1444, 7, 0, 0x94);
    SERDES_AM654_CFG(0x1460, 31, 24, 0x7f);
    SERDES_AM654_CFG(0x1464, 7, 0, 0x43);
    SERDES_AM654_CFG(0x1464, 23, 16, 0x6f);
    SERDES_AM654_CFG(0x1464, 31, 24, 0x43);
    SERDES_AM654_CFG(0x1484, 23, 16, 0x8f);
    SERDES_AM654_CFG(0x1498, 7, 0, 0x4f);
    SERDES_AM654_CFG(0x1498, 23, 16, 0x4f);
    SERDES_AM654_CFG(0x007c, 31, 24, 0x0d);
    SERDES_AM654_CFG(0x0b90, 15, 8, 0x0f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_pcie_init(phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_pcie_init(struct serdes_am654 *phy)
    {
    let mut ret: c_int = 0;
    ret |= regmap_field_write(phy.fields[CMU_PLL_CTRL], 0x2);
    ret |= regmap_field_write(phy.fields[AHB_PMA_CM_VCO_VBIAS_VREG], 0x98);
    ret |= regmap_field_write(phy.fields[AHB_PMA_CM_VCO_BIAS_VREG], 0x98);
    ret |= regmap_field_write(phy.fields[AHB_PMA_CM_SR], 0x45);
    ret |= regmap_field_write(phy.fields[AHB_SSC_GEN_Z_O_20_13], 0xe);
    ret |= regmap_field_write(phy.fields[LANE_PLL_CTRL_RXEQ_RXIDLE], 0x5);
    ret |= regmap_field_write(phy.fields[AHB_PMA_LN_AGC_THSEL_VREGH], 0x83);
    ret |= regmap_field_write(phy.fields[AHB_PMA_LN_GEN3_AGC_SD_THSEL], 0x83);
    ret |= regmap_field_write(phy.fields[AHB_PMA_LN_RX_SELR_GEN3],	0x81);
    ret |= regmap_field_write(phy.fields[AHB_PMA_LN_TX_DRV], 0x3b);
    ret |= regmap_field_write(phy.fields[P2S_RBUF_PTR_DIFF], 0x3);
    ret |= regmap_field_write(phy.fields[CONFIG_VERSION], VERSION_VAL);
    ret |= regmap_field_write(phy.fields[COMRXEQ_MS_INIT_CTRL_7_0], 0xf);
    ret |= regmap_field_write(phy.fields[COMRXEQ_HS_INIT_CAL_7_0], 0x4f);
    ret |= regmap_field_write(phy.fields[COMRXEQ_MS_RECAL_CTRL_7_0], 0xf);
    ret |= regmap_field_write(phy.fields[COMRXEQ_HS_RECAL_CTRL_7_0], 0x4f);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CSR_ATT_CONFIG], 0x7);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CSR_EBSTADAPT_WIN_LEN], 0x7f);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CTRL_3_4], 0xf);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CTRL_14_15_16], 0x9a);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CSR_DLEV_ERR_THRESH], 0x32);
    ret |= regmap_field_write(phy.fields[COMRXEQ_CTRL_25], 0x80);
    ret |= regmap_field_write(phy.fields[CSR_RXEQ_RATE_CHANGE_CAL_RUN_RATE2_O], 0xf);
    ret |= regmap_field_write(phy.fields[COMRXEQ_HS_RCHANGE_CTRL_7_0], 0x4f);
    ret |= regmap_field_write(phy.fields[CMU_MASTER_CDN], 0x1);
    ret |= regmap_field_write(phy.fields[L1_MASTER_CDN], 0x2);
    if (ret)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_init(x: *mut phy) -> c_int {
    static int serdes_am654_init(struct phy *x)
    {
    struct serdes_am654 *phy = phy_get_drvdata(x);
    switch (phy.type) {
    case PHY_TYPE_PCIE:
    return serdes_am654_pcie_init(phy);
    case PHY_TYPE_USB3:
    return serdes_am654_usb3_init(phy);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_reset(x: *mut phy) -> c_int {
    static int serdes_am654_reset(struct phy *x)
    {
    struct serdes_am654 *phy = phy_get_drvdata(x);
    let mut ret: c_int = 0;
    serdes_am654_disable_pll(phy);
    serdes_am654_disable_txrx(phy);
    ret |= regmap_field_write(phy.fields[POR_EN], 0x1);
    mdelay(1);
    ret |= regmap_field_write(phy.fields[POR_EN], 0x0);
    if (ret)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_release(x: *mut phy) {
    static void serdes_am654_release(struct phy *x)
    {
    struct serdes_am654 *phy = phy_get_drvdata(x);
    phy.type = PHY_NONE;
    phy.busy = false;
    mux_control_deselect(phy.control);
    }
    static struct phy *serdes_am654_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct serdes_am654 *am654_phy;
    struct phy *phy;
    int ret;
    phy = of_phy_simple_xlate(dev, args);
    if (IS_ERR(phy))
    return phy;
    am654_phy = phy_get_drvdata(phy);
    if (am654_phy.busy)
    return ERR_PTR(-EBUSY);
    ret = mux_control_select(am654_phy.control, args.args[1]);
    if (ret) {
    dev_err(dev, "Failed to select SERDES Lane Function\n");
    return ERR_PTR(ret);
    }
    am654_phy.busy = true;
    am654_phy.type = args.args[0];
    return phy;
    }
    static const struct phy_ops ops = {
    .reset		= serdes_am654_reset,
    .init		= serdes_am654_init,
    .power_on	= serdes_am654_power_on,
    .power_off	= serdes_am654_power_off,
    .release	= serdes_am654_release,
    .owner		= THIS_MODULE,
    };
pub const SERDES_NUM_MUX_COMBINATIONS: c_int = 16;
pub const LICLK: c_int = 0;
pub const EXT_REFCLK: c_int = 1;
pub const RICLK: c_int = 2;
    static const int
    serdes_am654_mux_table[SERDES_NUM_MUX_COMBINATIONS][SERDES_NUM_CLOCKS] = {
//
// Each combination maps to one of
// "Figure 12-1986. SerDes Reference Clock Distribution"
// in TRM.
//
// Parent of CMU refclk, Left output, Right output
// either of EXT_REFCLK, LICLK, RICLK
//
    { EXT_REFCLK, EXT_REFCLK, EXT_REFCLK },	/* 0000 */
    { RICLK, EXT_REFCLK, EXT_REFCLK },	/* 0001 */
    { EXT_REFCLK, RICLK, LICLK },		/* 0010 */
    { RICLK, RICLK, EXT_REFCLK },		/* 0011 */
    { LICLK, EXT_REFCLK, EXT_REFCLK },	/* 0100 */
    { EXT_REFCLK, EXT_REFCLK, EXT_REFCLK },	/* 0101 */
    { LICLK, RICLK, LICLK },		/* 0110 */
    { EXT_REFCLK, RICLK, LICLK },		/* 0111 */
    { EXT_REFCLK, EXT_REFCLK, LICLK },	/* 1000 */
    { RICLK, EXT_REFCLK, LICLK },		/* 1001 */
    { EXT_REFCLK, RICLK, EXT_REFCLK },	/* 1010 */
    { RICLK, RICLK, EXT_REFCLK },		/* 1011 */
    { LICLK, EXT_REFCLK, LICLK },		/* 1100 */
    { EXT_REFCLK, EXT_REFCLK, LICLK },	/* 1101 */
    { LICLK, RICLK, EXT_REFCLK },		/* 1110 */
    { EXT_REFCLK, RICLK, EXT_REFCLK },	/* 1111 */
    };
#[no_mangle]
unsafe extern "C" fn serdes_am654_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 serdes_am654_clk_mux_get_parent(struct clk_hw *hw)
    {
    struct serdes_am654_clk_mux *mux = to_serdes_am654_clk_mux(hw);
    struct regmap *regmap = mux.regmap;
    let mut reg: c_uint = mux.reg;
    unsigned int val;
    regmap_read(regmap, reg, &val);
    val &= AM654_SERDES_CTRL_CLKSEL_MASK;
    val >>= AM654_SERDES_CTRL_CLKSEL_SHIFT;
    return serdes_am654_mux_table[val][mux.clk_id];
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int serdes_am654_clk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct serdes_am654_clk_mux *mux = to_serdes_am654_clk_mux(hw);
    struct regmap *regmap = mux.regmap;
    const char *name = clk_hw_get_name(hw);
    let mut reg: c_uint = mux.reg;
    let mut clk_id: c_int = mux.clk_id;
    int parents[SERDES_NUM_CLOCKS];
    const int *p;
    u32 val;
    int found, i;
    int ret;
// get existing setting
    regmap_read(regmap, reg, &val);
    val &= AM654_SERDES_CTRL_CLKSEL_MASK;
    val >>= AM654_SERDES_CTRL_CLKSEL_SHIFT;
    for (i = 0; i < SERDES_NUM_CLOCKS; i++)
    parents[i] = serdes_am654_mux_table[val][i];
// change parent of this clock. others left intact
    parents[clk_id] = index;
// Find the match
    for (val = 0; val < SERDES_NUM_MUX_COMBINATIONS; val++) {
    p = serdes_am654_mux_table[val];
    found = 1;
    for (i = 0; i < SERDES_NUM_CLOCKS; i++) {
    if (parents[i] != p[i]) {
    found = 0;
    break;
    }
    }
    if (found)
    break;
    }
    if (!found) {
//
// This can never happen, unless we missed
// a valid combination in serdes_am654_mux_table.
//
    WARN(1, "Failed to find the parent of %s clock\n", name);
    return -EINVAL;
    }
    val <<= AM654_SERDES_CTRL_CLKSEL_SHIFT;
    ret = regmap_update_bits(regmap, reg, AM654_SERDES_CTRL_CLKSEL_MASK,
    val);
    return ret;
    }
    static const struct clk_ops serdes_am654_clk_mux_ops = {
    .determine_rate = __clk_mux_determine_rate,
    .set_parent = serdes_am654_clk_mux_set_parent,
    .get_parent = serdes_am654_clk_mux_get_parent,
    };
    static int serdes_am654_clk_register(struct serdes_am654 *am654_phy,
    const char *clock_name, int clock_num)
    {
    struct device_node *node = am654_phy.of_node;
    struct device *dev = am654_phy.dev;
    struct serdes_am654_clk_mux *mux;
    const char **parent_names;
    struct clk_init_data *init;
    unsigned int num_parents;
    struct regmap *regmap;
    const __be32 *addr;
    unsigned int reg;
    struct clk *clk;
    mux = devm_kzalloc(dev, sizeof(*mux), GFP_KERNEL);
    if (!mux)
    return -ENOMEM;
    init = &mux.clk_data;
    struct device_node *regmap_node __free(device_node) =
    of_parse_phandle(node, "ti,serdes-clk", 0);
    if (!regmap_node)
    return dev_err_probe(dev, -ENODEV, "Fail to get serdes-clk node\n");
    regmap = syscon_node_to_regmap(regmap_node.parent);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Fail to get Syscon regmap\n");
    num_parents = of_clk_get_parent_count(node);
    if (num_parents < 2)
    return dev_err_probe(dev, -EINVAL, "SERDES clock must have parents\n");
    parent_names = devm_kzalloc(dev, (sizeof(char *) * num_parents),
    GFP_KERNEL);
    if (!parent_names)
    return -ENOMEM;
    of_clk_parent_fill(node, parent_names, num_parents);
    addr = of_get_address(regmap_node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!addr)
    return -EINVAL;
    reg = be32_to_cpu(*addr);
    init.ops = &serdes_am654_clk_mux_ops;
    init.flags = CLK_SET_RATE_NO_REPARENT;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.name = clock_name;
    mux.regmap = regmap;
    mux.reg = reg;
    mux.clk_id = clock_num;
    mux.hw.init = init;
    clk = devm_clk_register(dev, &mux.hw);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    am654_phy.clks[clock_num] = clk;
    return 0;
    }
    static const struct of_device_id serdes_am654_id_table[] = {
    {
    .compatible = "ti,phy-am654-serdes",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, serdes_am654_id_table);
#[no_mangle]
unsafe extern "C" fn serdes_am654_regfield_init(am654_phy: *mut serdes_am654) -> c_int {
    static int serdes_am654_regfield_init(struct serdes_am654 *am654_phy)
    {
    struct regmap *regmap = am654_phy.regmap;
    struct device *dev = am654_phy.dev;
    int i;
    for (i = 0; i < MAX_FIELDS; i++) {
    am654_phy.fields[i] = devm_regmap_field_alloc(dev,
    regmap,
    serdes_am654_reg_fields[i]);
    if (IS_ERR(am654_phy.fields[i])) {
    dev_err(dev, "Unable to allocate regmap field %d\n", i);
    return PTR_ERR(am654_phy.fields[i]);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_probe(pdev: *mut platform_device) -> c_int {
    static int serdes_am654_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct clk_onecell_data *clk_data;
    struct serdes_am654 *am654_phy;
    struct mux_control *control;
    const char *clock_name;
    struct regmap *regmap;
    void __iomem *base;
    struct phy *phy;
    int ret;
    int i;
    am654_phy = devm_kzalloc(dev, sizeof(*am654_phy), GFP_KERNEL);
    if (!am654_phy)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &serdes_am654_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "Failed to initialize regmap\n");
    return PTR_ERR(regmap);
    }
    control = devm_mux_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(control))
    return PTR_ERR(control);
    am654_phy.dev = dev;
    am654_phy.of_node = node;
    am654_phy.regmap = regmap;
    am654_phy.control = control;
    am654_phy.type = PHY_NONE;
    ret = serdes_am654_regfield_init(am654_phy);
    if (ret) {
    dev_err(dev, "Failed to initialize regfields\n");
    return ret;
    }
    platform_set_drvdata(pdev, am654_phy);
    for (i = 0; i < SERDES_NUM_CLOCKS; i++) {
    ret = of_property_read_string_index(node, "clock-output-names",
    i, &clock_name);
    if (ret) {
    dev_err(dev, "Failed to get clock name\n");
    return ret;
    }
    ret = serdes_am654_clk_register(am654_phy, clock_name, i);
    if (ret) {
    dev_err(dev, "Failed to initialize clock %s\n",
    clock_name);
    return ret;
    }
    }
    clk_data = &am654_phy.clk_data;
    clk_data.clks = am654_phy.clks;
    clk_data.clk_num = SERDES_NUM_CLOCKS;
    ret = of_clk_add_provider(node, of_clk_src_onecell_get, clk_data);
    if (ret)
    return ret;
    pm_runtime_enable(dev);
    phy = devm_phy_create(dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(phy)) {
    ret = PTR_ERR(phy);
    goto clk_err;
    }
    phy_set_drvdata(phy, am654_phy);
    phy_provider = devm_of_phy_provider_register(dev, serdes_am654_xlate);
    if (IS_ERR(phy_provider)) {
    ret = PTR_ERR(phy_provider);
    goto clk_err;
    }
    return 0;
    clk_err:
    of_clk_del_provider(node);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serdes_am654_remove(pdev: *mut platform_device) {
    static void serdes_am654_remove(struct platform_device *pdev)
    {
    struct serdes_am654 *am654_phy = platform_get_drvdata(pdev);
    struct device_node *node = am654_phy.of_node;
    pm_runtime_disable(&pdev.dev);
    of_clk_del_provider(node);
    }
    static struct platform_driver serdes_am654_driver = {
    .probe		= serdes_am654_probe,
    .remove		= serdes_am654_remove,
    .driver		= {
    .name	= "phy-am654",
    .of_match_table = serdes_am654_id_table,
    },
    };
    module_platform_driver(serdes_am654_driver);
    MODULE_AUTHOR("Texas Instruments Inc.");
    MODULE_DESCRIPTION("TI AM654x SERDES driver");
    MODULE_LICENSE("GPL v2");
