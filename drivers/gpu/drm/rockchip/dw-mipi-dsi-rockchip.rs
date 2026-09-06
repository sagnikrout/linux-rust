//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/rockchip/dw-mipi-dsi-rockchip.c
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:
// Chris Zhong <zyw@rock-chips.com>
// Nickey Yang <nickey.yang@rock-chips.com>
//

pub const DSI_PHY_RSTZ: c_uint = 0xa0;
pub const PHY_DISFORCEPLL: c_int = 0;

pub const PHY_DISABLECLK: c_int = 0;

pub const PHY_RSTZ: c_int = 0;

pub const PHY_SHUTDOWNZ: c_int = 0;

pub const DSI_PHY_IF_CFG: c_uint = 0xa4;

pub const DSI_PHY_STATUS: c_uint = 0xb0;

pub const DSI_PHY_TST_CTRL0: c_uint = 0xb4;

pub const PHY_UNTESTCLK: c_int = 0;

pub const PHY_UNTESTCLR: c_int = 0;
pub const DSI_PHY_TST_CTRL1: c_uint = 0xb8;

pub const PHY_UNTESTEN: c_int = 0;

pub const DSI_INT_ST0: c_uint = 0xbc;
pub const DSI_INT_ST1: c_uint = 0xc0;
pub const DSI_INT_MSK0: c_uint = 0xc4;
pub const DSI_INT_MSK1: c_uint = 0xc8;
pub const PHY_STATUS_TIMEOUT_US: c_int = 10000;
pub const CMD_PKT_STATUS_TIMEOUT_US: c_int = 20000;

pub const CP_CURRENT_3UA: c_uint = 0x1;
pub const CP_CURRENT_4_5UA: c_uint = 0x2;
pub const CP_CURRENT_7_5UA: c_uint = 0x6;
pub const CP_CURRENT_6UA: c_uint = 0x9;
pub const CP_CURRENT_12UA: c_uint = 0xb;

pub const LPF_RESISTORS_15_5KOHM: c_uint = 0x1;
pub const LPF_RESISTORS_13KOHM: c_uint = 0x2;
pub const LPF_RESISTORS_11_5KOHM: c_uint = 0x4;
pub const LPF_RESISTORS_10_5KOHM: c_uint = 0x8;
pub const LPF_RESISTORS_8KOHM: c_uint = 0x10;

pub const LOW_PROGRAM_EN: c_int = 0;

pub const TER_RESISTOR_LOW: c_int = 0;

pub const PLL_BIAS_CUR_SEL_CAP_VCO_CONTROL: c_uint = 0x10;
pub const PLL_CP_CONTROL_PLL_LOCK_BYPASS: c_uint = 0x11;
pub const PLL_LPF_AND_CP_CONTROL: c_uint = 0x12;
pub const PLL_INPUT_DIVIDER_RATIO: c_uint = 0x17;
pub const PLL_LOOP_DIVIDER_RATIO: c_uint = 0x18;
pub const PLL_INPUT_AND_LOOP_DIVIDER_RATIOS_CONTROL: c_uint = 0x19;
pub const BANDGAP_AND_BIAS_CONTROL: c_uint = 0x20;
pub const TERMINATION_RESISTER_CONTROL: c_uint = 0x21;
pub const AFE_BIAS_BANDGAP_ANALOG_PROGRAMMABILITY: c_uint = 0x22;
pub const HS_RX_CONTROL_OF_LANE_CLK: c_uint = 0x34;
pub const HS_RX_CONTROL_OF_LANE_0: c_uint = 0x44;
pub const HS_RX_CONTROL_OF_LANE_1: c_uint = 0x54;
pub const HS_TX_CLOCK_LANE_REQUEST_STATE_TIME_CONTROL: c_uint = 0x60;
pub const HS_TX_CLOCK_LANE_PREPARE_STATE_TIME_CONTROL: c_uint = 0x61;
pub const HS_TX_CLOCK_LANE_HS_ZERO_STATE_TIME_CONTROL: c_uint = 0x62;
pub const HS_TX_CLOCK_LANE_TRAIL_STATE_TIME_CONTROL: c_uint = 0x63;
pub const HS_TX_CLOCK_LANE_EXIT_STATE_TIME_CONTROL: c_uint = 0x64;
pub const HS_TX_CLOCK_LANE_POST_TIME_CONTROL: c_uint = 0x65;
pub const HS_TX_DATA_LANE_REQUEST_STATE_TIME_CONTROL: c_uint = 0x70;
pub const HS_TX_DATA_LANE_PREPARE_STATE_TIME_CONTROL: c_uint = 0x71;
pub const HS_TX_DATA_LANE_HS_ZERO_STATE_TIME_CONTROL: c_uint = 0x72;
pub const HS_TX_DATA_LANE_TRAIL_STATE_TIME_CONTROL: c_uint = 0x73;
pub const HS_TX_DATA_LANE_EXIT_STATE_TIME_CONTROL: c_uint = 0x74;
pub const HS_RX_DATA_LANE_THS_SETTLE_CONTROL: c_uint = 0x75;
pub const HS_RX_CONTROL_OF_LANE_2: c_uint = 0x84;
pub const HS_RX_CONTROL_OF_LANE_3: c_uint = 0x94;

pub const PX30_GRF_PD_VO_CON1: c_uint = 0x0438;

pub const RK3128_GRF_LVDS_CON0: c_uint = 0x0150;

pub const RK3288_GRF_SOC_CON6: c_uint = 0x025c;

pub const RK3368_GRF_SOC_CON7: c_uint = 0x41c;

pub const RK3399_GRF_SOC_CON20: c_uint = 0x6250;

pub const RK3399_GRF_SOC_CON22: c_uint = 0x6258;

pub const RK3399_GRF_SOC_CON23: c_uint = 0x625c;

pub const RK3399_GRF_SOC_CON24: c_uint = 0x6260;

pub const RK3568_GRF_VO_CON2: c_uint = 0x0368;

pub const RK3506_SYS_GRF_SOC_CON6: c_uint = 0x0018;

//
// Note these registers do not appear in the datasheet, they are
// however present in the BSP driver which is where these values
// come from. Name GRF_VO_CON3 is assumed.
//
pub const RK3568_GRF_VO_CON3: c_uint = 0x36c;

pub const RV1126_GRF_DSIPHY_CON: c_uint = 0x10220;

    enum {
    DW_DSI_USAGE_IDLE,
    DW_DSI_USAGE_DSI,
    DW_DSI_USAGE_PHY,
    };
    enum {
    BANDGAP_97_07,
    BANDGAP_98_05,
    BANDGAP_99_02,
    BANDGAP_100_00,
    BANDGAP_93_17,
    BANDGAP_94_15,
    BANDGAP_95_12,
    BANDGAP_96_10,
    };
    enum {
    BIASEXTR_87_1,
    BIASEXTR_91_5,
    BIASEXTR_95_9,
    BIASEXTR_100,
    BIASEXTR_105_94,
    BIASEXTR_111_88,
    BIASEXTR_118_8,
    BIASEXTR_127_7,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_dw_dsi_chip_data {
    pub reg: u32,
    pub lcdsel_grf_reg: u32,
    pub lcdsel_big: u32,
    pub lcdsel_lit: u32,
    pub enable_grf_reg: u32,
    pub enable: u32,
    pub lanecfg1_grf_reg: u32,
    pub lanecfg1: u32,
    pub lanecfg2_grf_reg: u32,
    pub lanecfg2: u32,
    pub phy): *mut *mut int (dphy_rx_init)(struct phy,
    pub phy): *mut *mut int (dphy_rx_power_on)(struct phy,
    pub phy): *mut *mut int (dphy_rx_power_off)(struct phy,
    pub flags: c_uint,
    pub max_data_lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi_rockchip {
    pub dev: *mut device,
    pub encoder: rockchip_encoder,
    pub base: *mut void __iomem,
    pub grf_regmap: *mut regmap,
    pub pclk: *mut clk,
    pub pllref_clk: *mut clk,
    pub grf_clk: *mut clk,
    pub phy_cfg_clk: *mut clk,
// dual-channel
    pub is_slave: bool,
    pub slave: *mut dw_mipi_dsi_rockchip,
// optional external dphy
    pub phy: *mut phy,
    pub phy_opts: union phy_configure_opts,
// being a phy for other mipi hosts
    pub usage_mode: c_uint,
    pub usage_mutex: mutex,
    pub dphy: *mut phy,
    pub dphy_config: phy_configure_opts_mipi_dphy,
    pub /: *mut *mut unsigned int lane_mbps; / per lane,
    pub input_div: u16,
    pub feedback_div: u16,
    pub format: u32,
    pub dmd: *mut dw_mipi_dsi,
    pub cdata: *const rockchip_dw_dsi_chip_data,
    pub pdata: dw_mipi_dsi_plat_data,
    pub dsi_bound: bool,
}

    static struct dw_mipi_dsi_rockchip *to_dsi(struct drm_encoder *encoder)
    {
    struct rockchip_encoder *rkencoder = to_rockchip_encoder(encoder);
    return container_of(rkencoder, struct dw_mipi_dsi_rockchip, encoder);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dphy_pll_parameter_map {
    pub max_mbps: c_uint,
    pub hsfreqrange: u8,
    pub icpctrl: u8,
    pub lpfctrl: u8,
}

// The table is based on 27MHz DPHY pll reference clock.
    static const struct dphy_pll_parameter_map dppa_map[] = {
    {  89, 0x00, CP_CURRENT_3UA, LPF_RESISTORS_13KOHM },
    {  99, 0x10, CP_CURRENT_3UA, LPF_RESISTORS_13KOHM },
    { 109, 0x20, CP_CURRENT_3UA, LPF_RESISTORS_13KOHM },
    { 129, 0x01, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 139, 0x11, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 149, 0x21, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 169, 0x02, CP_CURRENT_6UA, LPF_RESISTORS_13KOHM },
    { 179, 0x12, CP_CURRENT_6UA, LPF_RESISTORS_13KOHM },
    { 199, 0x22, CP_CURRENT_6UA, LPF_RESISTORS_13KOHM },
    { 219, 0x03, CP_CURRENT_4_5UA, LPF_RESISTORS_13KOHM },
    { 239, 0x13, CP_CURRENT_4_5UA, LPF_RESISTORS_13KOHM },
    { 249, 0x23, CP_CURRENT_4_5UA, LPF_RESISTORS_13KOHM },
    { 269, 0x04, CP_CURRENT_6UA, LPF_RESISTORS_11_5KOHM },
    { 299, 0x14, CP_CURRENT_6UA, LPF_RESISTORS_11_5KOHM },
    { 329, 0x05, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 359, 0x15, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 399, 0x25, CP_CURRENT_3UA, LPF_RESISTORS_15_5KOHM },
    { 449, 0x06, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 499, 0x16, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 549, 0x07, CP_CURRENT_7_5UA, LPF_RESISTORS_10_5KOHM },
    { 599, 0x17, CP_CURRENT_7_5UA, LPF_RESISTORS_10_5KOHM },
    { 649, 0x08, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 699, 0x18, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 749, 0x09, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 799, 0x19, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 849, 0x29, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 899, 0x39, CP_CURRENT_7_5UA, LPF_RESISTORS_11_5KOHM },
    { 949, 0x0a, CP_CURRENT_12UA, LPF_RESISTORS_8KOHM },
    { 999, 0x1a, CP_CURRENT_12UA, LPF_RESISTORS_8KOHM },
    {1049, 0x2a, CP_CURRENT_12UA, LPF_RESISTORS_8KOHM },
    {1099, 0x3a, CP_CURRENT_12UA, LPF_RESISTORS_8KOHM },
    {1149, 0x0b, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1199, 0x1b, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1249, 0x2b, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1299, 0x3b, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1349, 0x0c, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1399, 0x1c, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1449, 0x2c, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM },
    {1500, 0x3c, CP_CURRENT_12UA, LPF_RESISTORS_10_5KOHM }
    };
#[no_mangle]
unsafe extern "C" fn max_mbps_to_parameter(max_mbps: c_uint) -> c_int {
    static int max_mbps_to_parameter(unsigned int max_mbps)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(dppa_map); i++)
    if (dppa_map[i].max_mbps >= max_mbps)
    return i;
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn dsi_write(dsi: *mut dw_mipi_dsi_rockchip, reg: u32, val: u32) {
    static inline void dsi_write(struct dw_mipi_dsi_rockchip *dsi, u32 reg, u32 val)
    {
    writel(val, dsi.base + reg);
    }
    static void dw_mipi_dsi_phy_write(struct dw_mipi_dsi_rockchip *dsi,
    u8 test_code,
    u8 test_data)
    {
//
// With the falling edge on TESTCLK, the TESTDIN[7:0] signal content
// is latched internally as the current test code. Test data is
// programmed internally by rising edge on TESTCLK.
//
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_TESTCLK | PHY_UNTESTCLR);
    dsi_write(dsi, DSI_PHY_TST_CTRL1, PHY_TESTEN | PHY_TESTDOUT(0) |
    PHY_TESTDIN(test_code));
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_UNTESTCLK | PHY_UNTESTCLR);
    dsi_write(dsi, DSI_PHY_TST_CTRL1, PHY_UNTESTEN | PHY_TESTDOUT(0) |
    PHY_TESTDIN(test_data));
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_TESTCLK | PHY_UNTESTCLR);
    }
//
// ns2bc - Nanoseconds to byte clock cycles
//
#[no_mangle]
pub unsafe extern "C" fn ns2bc(dsi: *mut dw_mipi_dsi_rockchip, ns: c_int) -> c_uint {
    static inline unsigned int ns2bc(struct dw_mipi_dsi_rockchip *dsi, int ns)
    {
    return DIV_ROUND_UP(ns * dsi.lane_mbps / 8, 1000);
    }
//
// ns2ui - Nanoseconds to UI time periods
//
#[no_mangle]
pub unsafe extern "C" fn ns2ui(dsi: *mut dw_mipi_dsi_rockchip, ns: c_int) -> c_uint {
    static inline unsigned int ns2ui(struct dw_mipi_dsi_rockchip *dsi, int ns)
    {
    return DIV_ROUND_UP(ns * dsi.lane_mbps, 1000);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_phy_init(priv_data: *mut c_void) -> c_int {
    static int dw_mipi_dsi_phy_init(void *priv_data)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    int ret, i, vco;
    if (dsi.phy)
    return 0;
//
// Get vco from frequency(lane_mbps)
// vco	frequency table
// 000 - between   80 and  200 MHz
// 001 - between  200 and  300 MHz
// 010 - between  300 and  500 MHz
// 011 - between  500 and  700 MHz
// 100 - between  700 and  900 MHz
// 101 - between  900 and 1100 MHz
// 110 - between 1100 and 1300 MHz
// 111 - between 1300 and 1500 MHz
//
    vco = (dsi.lane_mbps < 200) ? 0 : (dsi.lane_mbps + 100) / 200;
    i = max_mbps_to_parameter(dsi.lane_mbps);
    if (i < 0) {
    DRM_DEV_ERROR(dsi.dev,
    "failed to get parameter for %dmbps clock\n",
    dsi.lane_mbps);
    return i;
    }
    ret = clk_prepare_enable(dsi.phy_cfg_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable phy_cfg_clk\n");
    return ret;
    }
    dw_mipi_dsi_phy_write(dsi, PLL_BIAS_CUR_SEL_CAP_VCO_CONTROL,
    BYPASS_VCO_RANGE |
    VCO_RANGE_CON_SEL(vco) |
    VCO_IN_CAP_CON_LOW |
    REF_BIAS_CUR_SEL);
    dw_mipi_dsi_phy_write(dsi, PLL_CP_CONTROL_PLL_LOCK_BYPASS,
    CP_CURRENT_SEL(dppa_map[i].icpctrl));
    dw_mipi_dsi_phy_write(dsi, PLL_LPF_AND_CP_CONTROL,
    CP_PROGRAM_EN | LPF_PROGRAM_EN |
    LPF_RESISTORS_SEL(dppa_map[i].lpfctrl));
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_0,
    HSFREQRANGE_SEL(dppa_map[i].hsfreqrange));
    dw_mipi_dsi_phy_write(dsi, PLL_INPUT_DIVIDER_RATIO,
    INPUT_DIVIDER(dsi.input_div));
    dw_mipi_dsi_phy_write(dsi, PLL_LOOP_DIVIDER_RATIO,
    LOOP_DIV_LOW_SEL(dsi.feedback_div) |
    LOW_PROGRAM_EN);
//
// We need set PLL_INPUT_AND_LOOP_DIVIDER_RATIOS_CONTROL immediately
// to make the configured LSB effective according to IP simulation
// and lab test results.
// Only in this way can we get correct mipi phy pll frequency.
//
    dw_mipi_dsi_phy_write(dsi, PLL_INPUT_AND_LOOP_DIVIDER_RATIOS_CONTROL,
    PLL_LOOP_DIV_EN | PLL_INPUT_DIV_EN);
    dw_mipi_dsi_phy_write(dsi, PLL_LOOP_DIVIDER_RATIO,
    LOOP_DIV_HIGH_SEL(dsi.feedback_div) |
    HIGH_PROGRAM_EN);
    dw_mipi_dsi_phy_write(dsi, PLL_INPUT_AND_LOOP_DIVIDER_RATIOS_CONTROL,
    PLL_LOOP_DIV_EN | PLL_INPUT_DIV_EN);
    dw_mipi_dsi_phy_write(dsi, AFE_BIAS_BANDGAP_ANALOG_PROGRAMMABILITY,
    LOW_PROGRAM_EN | BIASEXTR_SEL(BIASEXTR_127_7));
    dw_mipi_dsi_phy_write(dsi, AFE_BIAS_BANDGAP_ANALOG_PROGRAMMABILITY,
    HIGH_PROGRAM_EN | BANDGAP_SEL(BANDGAP_96_10));
    dw_mipi_dsi_phy_write(dsi, BANDGAP_AND_BIAS_CONTROL,
    POWER_CONTROL | INTERNAL_REG_CURRENT |
    BIAS_BLOCK_ON | BANDGAP_ON);
    dw_mipi_dsi_phy_write(dsi, TERMINATION_RESISTER_CONTROL,
    TER_RESISTOR_LOW | TER_CAL_DONE |
    SETRD_MAX | TER_RESISTORS_ON);
    dw_mipi_dsi_phy_write(dsi, TERMINATION_RESISTER_CONTROL,
    TER_RESISTOR_HIGH | LEVEL_SHIFTERS_ON |
    SETRD_MAX | POWER_MANAGE |
    TER_RESISTORS_ON);
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_REQUEST_STATE_TIME_CONTROL,
    TLP_PROGRAM_EN | ns2bc(dsi, 500));
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_PREPARE_STATE_TIME_CONTROL,
    THS_PRE_PROGRAM_EN | ns2ui(dsi, 40));
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_HS_ZERO_STATE_TIME_CONTROL,
    THS_ZERO_PROGRAM_EN | ns2bc(dsi, 300));
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_TRAIL_STATE_TIME_CONTROL,
    THS_PRE_PROGRAM_EN | ns2ui(dsi, 100));
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_EXIT_STATE_TIME_CONTROL,
    BIT(5) | ns2bc(dsi, 100));
    dw_mipi_dsi_phy_write(dsi, HS_TX_CLOCK_LANE_POST_TIME_CONTROL,
    BIT(5) | (ns2bc(dsi, 60) + 7));
    dw_mipi_dsi_phy_write(dsi, HS_TX_DATA_LANE_REQUEST_STATE_TIME_CONTROL,
    TLP_PROGRAM_EN | ns2bc(dsi, 500));
    dw_mipi_dsi_phy_write(dsi, HS_TX_DATA_LANE_PREPARE_STATE_TIME_CONTROL,
    THS_PRE_PROGRAM_EN | (ns2ui(dsi, 50) + 20));
    dw_mipi_dsi_phy_write(dsi, HS_TX_DATA_LANE_HS_ZERO_STATE_TIME_CONTROL,
    THS_ZERO_PROGRAM_EN | (ns2bc(dsi, 140) + 2));
    dw_mipi_dsi_phy_write(dsi, HS_TX_DATA_LANE_TRAIL_STATE_TIME_CONTROL,
    THS_PRE_PROGRAM_EN | (ns2ui(dsi, 60) + 8));
    dw_mipi_dsi_phy_write(dsi, HS_TX_DATA_LANE_EXIT_STATE_TIME_CONTROL,
    BIT(5) | ns2bc(dsi, 100));
    clk_disable_unprepare(dsi.phy_cfg_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_phy_power_on(priv_data: *mut c_void) {
    static void dw_mipi_dsi_phy_power_on(void *priv_data)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    int ret;
    ret = phy_set_mode(dsi.phy, PHY_MODE_MIPI_DPHY);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "failed to set phy mode: %d\n", ret);
    return;
    }
    phy_configure(dsi.phy, &dsi.phy_opts);
    phy_power_on(dsi.phy);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_phy_power_off(priv_data: *mut c_void) {
    static void dw_mipi_dsi_phy_power_off(void *priv_data)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    phy_power_off(dsi.phy);
    }
    static int
    dw_mipi_dsi_get_lane_mbps(void *priv_data, const struct drm_display_mode *mode,
    unsigned long mode_flags, u32 lanes, u32 format,
    unsigned int *lane_mbps)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    int bpp;
    unsigned long mpclk, tmp;
    let mut target_mbps: c_uint = 1000;
    let mut max_mbps: c_uint = dppa_map[ARRAY_SIZE(dppa_map) - 1].max_mbps;
    let mut best_freq: c_ulong = 0;
    unsigned long fvco_min, fvco_max, fin, fout;
    unsigned int min_prediv, max_prediv;
    unsigned int _prediv, best_prediv;
    unsigned long _fbdiv, best_fbdiv;
    let mut min_delta: c_ulong = ULONG_MAX;
    dsi.format = format;
    bpp = mipi_dsi_pixel_format_to_bpp(dsi.format);
    if (bpp < 0) {
    DRM_DEV_ERROR(dsi.dev,
    "failed to get bpp for pixel format %d\n",
    dsi.format);
    return bpp;
    }
    mpclk = DIV_ROUND_UP(mode.clock, MSEC_PER_SEC);
    if (mpclk) {
// take 1 / 0.8, since mbps must big than bandwidth of RGB
    tmp = mpclk * (bpp / lanes) * 10 / 8;
    if (tmp < max_mbps)
    target_mbps = tmp;
    else
    DRM_DEV_ERROR(dsi.dev,
    "DPHY clock frequency is out of range\n");
    }
// for external phy only a the mipi_dphy_config is necessary
    if (dsi.phy) {
    phy_mipi_dphy_get_default_config(mode.clock * 1000 * 10 / 8,
    bpp, lanes,
    &dsi.phy_opts.mipi_dphy);
    dsi.lane_mbps = target_mbps;
// lane_mbps = dsi->lane_mbps;
    return 0;
    }
    fin = clk_get_rate(dsi.pllref_clk);
    fout = target_mbps * USEC_PER_SEC;
// constraint: 5Mhz <= Fref / N <= 40MHz
    min_prediv = DIV_ROUND_UP(fin, 40 * USEC_PER_SEC);
    max_prediv = fin / (5 * USEC_PER_SEC);
// constraint: 80MHz <= Fvco <= 1500Mhz
    fvco_min = 80 * USEC_PER_SEC;
    fvco_max = 1500 * USEC_PER_SEC;
    for (_prediv = min_prediv; _prediv <= max_prediv; _prediv++) {
    u64 tmp;
    u32 delta;
// Fvco = Fref * M / N
    tmp = (u64)fout * _prediv;
    do_div(tmp, fin);
    _fbdiv = tmp;
//
// Due to the use of a "by 2 pre-scaler," the range of the
// feedback multiplication value M is limited to even division
// numbers, and m must be greater than 6, not bigger than 512.
//
    if (_fbdiv < 6 || _fbdiv > 512)
    continue;
    _fbdiv += _fbdiv % 2;
    tmp = (u64)_fbdiv * fin;
    do_div(tmp, _prediv);
    if (tmp < fvco_min || tmp > fvco_max)
    continue;
    delta = abs(fout - tmp);
    if (delta < min_delta) {
    best_prediv = _prediv;
    best_fbdiv = _fbdiv;
    min_delta = delta;
    best_freq = tmp;
    }
    }
    if (best_freq) {
    dsi.lane_mbps = DIV_ROUND_UP(best_freq, USEC_PER_SEC);
// lane_mbps = dsi->lane_mbps;
    dsi.input_div = best_prediv;
    dsi.feedback_div = best_fbdiv;
    } else {
    DRM_DEV_ERROR(dsi.dev, "Can not find best_freq for DPHY\n");
    return -EINVAL;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hstt {
    pub maxfreq: c_uint,
    pub timing: dw_mipi_dsi_dphy_timing,
}

    {					\
    .maxfreq = _maxfreq,		\
    .timing = {			\
    .clk_lp2hs = _c_lp2hs,	\
    .clk_hs2lp = _c_hs2lp,	\
    .data_lp2hs = _d_lp2hs,	\
    .data_hs2lp = _d_hs2lp,	\
    }				\
    }
// Table A-3 High-Speed Transition Times
    static struct hstt hstt_table[] = {
    HSTT(  90,  32, 20,  26, 13),
    HSTT( 100,  35, 23,  28, 14),
    HSTT( 110,  32, 22,  26, 13),
    HSTT( 130,  31, 20,  27, 13),
    HSTT( 140,  33, 22,  26, 14),
    HSTT( 150,  33, 21,  26, 14),
    HSTT( 170,  32, 20,  27, 13),
    HSTT( 180,  36, 23,  30, 15),
    HSTT( 200,  40, 22,  33, 15),
    HSTT( 220,  40, 22,  33, 15),
    HSTT( 240,  44, 24,  36, 16),
    HSTT( 250,  48, 24,  38, 17),
    HSTT( 270,  48, 24,  38, 17),
    HSTT( 300,  50, 27,  41, 18),
    HSTT( 330,  56, 28,  45, 18),
    HSTT( 360,  59, 28,  48, 19),
    HSTT( 400,  61, 30,  50, 20),
    HSTT( 450,  67, 31,  55, 21),
    HSTT( 500,  73, 31,  59, 22),
    HSTT( 550,  79, 36,  63, 24),
    HSTT( 600,  83, 37,  68, 25),
    HSTT( 650,  90, 38,  73, 27),
    HSTT( 700,  95, 40,  77, 28),
    HSTT( 750, 102, 40,  84, 28),
    HSTT( 800, 106, 42,  87, 30),
    HSTT( 850, 113, 44,  93, 31),
    HSTT( 900, 118, 47,  98, 32),
    HSTT( 950, 124, 47, 102, 34),
    HSTT(1000, 130, 49, 107, 35),
    HSTT(1050, 135, 51, 111, 37),
    HSTT(1100, 139, 51, 114, 38),
    HSTT(1150, 146, 54, 120, 40),
    HSTT(1200, 153, 57, 125, 41),
    HSTT(1250, 158, 58, 130, 42),
    HSTT(1300, 163, 58, 135, 44),
    HSTT(1350, 168, 60, 140, 45),
    HSTT(1400, 172, 64, 144, 47),
    HSTT(1450, 176, 65, 148, 48),
    HSTT(1500, 181, 66, 153, 50)
    };
    static int
    dw_mipi_dsi_phy_get_timing(void *priv_data, unsigned int lane_mbps,
    struct dw_mipi_dsi_dphy_timing *timing)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(hstt_table); i++)
    if (lane_mbps < hstt_table[i].maxfreq)
    break;
    if (i == ARRAY_SIZE(hstt_table))
    i--;
// timing = hstt_table[i].timing;
    return 0;
    }
    static const struct dw_mipi_dsi_phy_ops dw_mipi_dsi_rockchip_phy_ops = {
    .init = dw_mipi_dsi_phy_init,
    .power_on = dw_mipi_dsi_phy_power_on,
    .power_off = dw_mipi_dsi_phy_power_off,
    .get_lane_mbps = dw_mipi_dsi_get_lane_mbps,
    .get_timing = dw_mipi_dsi_phy_get_timing,
    };
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_rockchip_config(dsi: *mut dw_mipi_dsi_rockchip) {
    static void dw_mipi_dsi_rockchip_config(struct dw_mipi_dsi_rockchip *dsi)
    {
    if (dsi.cdata.lanecfg1_grf_reg)
    regmap_write(dsi.grf_regmap, dsi.cdata.lanecfg1_grf_reg,
    dsi.cdata.lanecfg1);
    if (dsi.cdata.lanecfg2_grf_reg)
    regmap_write(dsi.grf_regmap, dsi.cdata.lanecfg2_grf_reg,
    dsi.cdata.lanecfg2);
    if (dsi.cdata.enable_grf_reg)
    regmap_write(dsi.grf_regmap, dsi.cdata.enable_grf_reg,
    dsi.cdata.enable);
    }
    static void dw_mipi_dsi_rockchip_set_lcdsel(struct dw_mipi_dsi_rockchip *dsi,
    int mux)
    {
    if (dsi.cdata.lcdsel_grf_reg)
    regmap_write(dsi.grf_regmap, dsi.cdata.lcdsel_grf_reg,
    mux ? dsi.cdata.lcdsel_lit : dsi.cdata.lcdsel_big);
    }
    static int
    dw_mipi_dsi_encoder_atomic_check(struct drm_encoder *encoder,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    struct rockchip_crtc_state *s = to_rockchip_crtc_state(crtc_state);
    struct dw_mipi_dsi_rockchip *dsi = to_dsi(encoder);
    switch (dsi.format) {
    case MIPI_DSI_FMT_RGB888:
    s.output_mode = ROCKCHIP_OUT_MODE_P888;
    break;
    case MIPI_DSI_FMT_RGB666:
    s.output_mode = ROCKCHIP_OUT_MODE_P666;
    break;
    case MIPI_DSI_FMT_RGB565:
    s.output_mode = ROCKCHIP_OUT_MODE_P565;
    break;
    default:
    WARN_ON(1);
    return -EINVAL;
    }
    s.output_type = DRM_MODE_CONNECTOR_DSI;
    if (dsi.slave)
    s.output_flags = ROCKCHIP_OUTPUT_DSI_DUAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_encoder_enable(encoder: *mut drm_encoder) {
    static void dw_mipi_dsi_encoder_enable(struct drm_encoder *encoder)
    {
    struct dw_mipi_dsi_rockchip *dsi = to_dsi(encoder);
    int ret, mux;
    mux = drm_of_encoder_active_endpoint_id(dsi.dev.of_node,
    &dsi.encoder.encoder);
    if (mux < 0)
    return;
//
// For the RK3399, the clk of grf must be enabled before writing grf
// register. And for RK3288 or other soc, this grf_clk must be NULL,
// the clk_prepare_enable return true directly.
//
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable grf_clk: %d\n", ret);
    return;
    }
    dw_mipi_dsi_rockchip_set_lcdsel(dsi, mux);
    if (dsi.slave)
    dw_mipi_dsi_rockchip_set_lcdsel(dsi.slave, mux);
    clk_disable_unprepare(dsi.grf_clk);
    }
    static const struct drm_encoder_funcs dw_mipi_dsi_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_encoder_helper_funcs
    dw_mipi_dsi_encoder_helper_funcs = {
    .atomic_check = dw_mipi_dsi_encoder_atomic_check,
    .enable = dw_mipi_dsi_encoder_enable,
    };
    static int rockchip_dsi_drm_create_encoder(struct dw_mipi_dsi_rockchip *dsi,
    struct drm_device *drm_dev)
    {
    struct drm_encoder *encoder = &dsi.encoder.encoder;
    int ret;
    encoder.possible_crtcs = drm_of_find_possible_crtcs(drm_dev,
    dsi.dev.of_node);
    ret = drm_encoder_init(drm_dev, encoder,
    &dw_mipi_dsi_encoder_funcs,
    DRM_MODE_ENCODER_DSI, core::ptr::null_mut());
    if (ret) {
    DRM_ERROR("Failed to initialize encoder with drm\n");
    return ret;
    }
    drm_encoder_helper_add(encoder, &dw_mipi_dsi_encoder_helper_funcs);
    return 0;
    }
    static struct device
// dw_mipi_dsi_rockchip_find_second(struct dw_mipi_dsi_rockchip *dsi)
    {
    const struct of_device_id *match;
    struct device_node *node = core::ptr::null_mut(), *local;
    match = of_match_device(dsi.dev.driver.of_match_table, dsi.dev);
    local = of_graph_get_remote_node(dsi.dev.of_node, 1, 0);
    if (!local)
    return core::ptr::null_mut();
    while ((node = of_find_compatible_node(node, core::ptr::null_mut(),
    match.compatible))) {
    struct device_node *remote;
// found ourself
    if (node == dsi.dev.of_node)
    continue;
    remote = of_graph_get_remote_node(node, 1, 0);
    if (!remote)
    continue;
// same display device in port1-ep0 for both
    if (remote == local) {
    struct dw_mipi_dsi_rockchip *dsi2;
    struct platform_device *pdev;
    pdev = of_find_device_by_node(node);
//
// we have found the second, so will either return it
// or return with an error. In any case won't need the
// nodes anymore nor continue the loop.
//
    of_node_put(remote);
    of_node_put(node);
    of_node_put(local);
    if (!pdev)
    return ERR_PTR(-EPROBE_DEFER);
    dsi2 = platform_get_drvdata(pdev);
    if (!dsi2) {
    platform_device_put(pdev);
    return ERR_PTR(-EPROBE_DEFER);
    }
    return &pdev.dev;
    }
    of_node_put(remote);
    }
    of_node_put(local);
    return core::ptr::null_mut();
    }
    static int dw_mipi_dsi_rockchip_bind(struct device *dev,
    struct device *master,
    void *data)
    {
    struct dw_mipi_dsi_rockchip *dsi = dev_get_drvdata(dev);
    struct drm_device *drm_dev = data;
    struct device *second;
    bool master1, master2;
    int ret;
    second = dw_mipi_dsi_rockchip_find_second(dsi);
    if (IS_ERR(second))
    return PTR_ERR(second);
    if (second) {
    master1 = of_property_read_bool(dsi.dev.of_node,
    "clock-master");
    master2 = of_property_read_bool(second.of_node,
    "clock-master");
    if (master1 && master2) {
    DRM_DEV_ERROR(dsi.dev, "only one clock-master allowed\n");
    return -EINVAL;
    }
    if (!master1 && !master2) {
    DRM_DEV_ERROR(dsi.dev, "no clock-master defined\n");
    return -EINVAL;
    }
// we are the slave in dual-DSI
    if (!master1) {
    dsi.is_slave = true;
    return 0;
    }
    dsi.slave = dev_get_drvdata(second);
    if (!dsi.slave) {
    DRM_DEV_ERROR(dev, "could not get slaves data\n");
    return -ENODEV;
    }
    dsi.slave.is_slave = true;
    dw_mipi_dsi_set_slave(dsi.dmd, dsi.slave.dmd);
    put_device(second);
    }
    pm_runtime_get_sync(dsi.dev);
    if (dsi.slave)
    pm_runtime_get_sync(dsi.slave.dev);
    ret = clk_prepare_enable(dsi.pllref_clk);
    if (ret) {
    DRM_DEV_ERROR(dev, "Failed to enable pllref_clk: %d\n", ret);
    goto out_pm_runtime;
    }
//
// With the GRF clock running, write lane and dual-mode configurations
// that won't change immediately. If we waited until enable() to do
// this, things like panel preparation would not be able to send
// commands over DSI.
//
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable grf_clk: %d\n", ret);
    goto out_pll_clk;
    }
    dw_mipi_dsi_rockchip_config(dsi);
    if (dsi.slave)
    dw_mipi_dsi_rockchip_config(dsi.slave);
    clk_disable_unprepare(dsi.grf_clk);
    ret = rockchip_dsi_drm_create_encoder(dsi, drm_dev);
    if (ret) {
    DRM_DEV_ERROR(dev, "Failed to create drm encoder\n");
    goto out_pll_clk;
    }
    rockchip_drm_encoder_set_crtc_endpoint_id(&dsi.encoder,
    dev.of_node, 0, 0);
    ret = dw_mipi_dsi_bind(dsi.dmd, &dsi.encoder.encoder);
    if (ret) {
    DRM_DEV_ERROR(dev, "Failed to bind: %d\n", ret);
    goto out_pll_clk;
    }
    dsi.dsi_bound = true;
    return 0;
    out_pll_clk:
    clk_disable_unprepare(dsi.pllref_clk);
    out_pm_runtime:
    pm_runtime_put(dsi.dev);
    if (dsi.slave)
    pm_runtime_put(dsi.slave.dev);
    return ret;
    }
    static void dw_mipi_dsi_rockchip_unbind(struct device *dev,
    struct device *master,
    void *data)
    {
    struct dw_mipi_dsi_rockchip *dsi = dev_get_drvdata(dev);
    if (dsi.is_slave)
    return;
    dsi.dsi_bound = false;
    dw_mipi_dsi_unbind(dsi.dmd);
    clk_disable_unprepare(dsi.pllref_clk);
    pm_runtime_put(dsi.dev);
    if (dsi.slave)
    pm_runtime_put(dsi.slave.dev);
    }
    static const struct component_ops dw_mipi_dsi_rockchip_ops = {
    .bind	= dw_mipi_dsi_rockchip_bind,
    .unbind	= dw_mipi_dsi_rockchip_unbind,
    };
    static int dw_mipi_dsi_rockchip_host_attach(void *priv_data,
    struct mipi_dsi_device *device)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    struct device *second;
    int ret;
    mutex_lock(&dsi.usage_mutex);
    if (dsi.usage_mode != DW_DSI_USAGE_IDLE) {
    DRM_DEV_ERROR(dsi.dev, "dsi controller already in use\n");
    mutex_unlock(&dsi.usage_mutex);
    return -EBUSY;
    }
    dsi.usage_mode = DW_DSI_USAGE_DSI;
    mutex_unlock(&dsi.usage_mutex);
    ret = component_add(dsi.dev, &dw_mipi_dsi_rockchip_ops);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to register component: %d\n",
    ret);
    goto out;
    }
    second = dw_mipi_dsi_rockchip_find_second(dsi);
    if (IS_ERR(second)) {
    ret = PTR_ERR(second);
    goto out;
    }
    if (second) {
    ret = component_add(second, &dw_mipi_dsi_rockchip_ops);
    if (ret) {
    DRM_DEV_ERROR(second,
    "Failed to register component: %d\n",
    ret);
    goto out;
    }
    }
    return 0;
    out:
    mutex_lock(&dsi.usage_mutex);
    dsi.usage_mode = DW_DSI_USAGE_IDLE;
    mutex_unlock(&dsi.usage_mutex);
    return ret;
    }
    static int dw_mipi_dsi_rockchip_host_detach(void *priv_data,
    struct mipi_dsi_device *device)
    {
    struct dw_mipi_dsi_rockchip *dsi = priv_data;
    struct device *second;
    second = dw_mipi_dsi_rockchip_find_second(dsi);
    if (second && !IS_ERR(second))
    component_del(second, &dw_mipi_dsi_rockchip_ops);
    component_del(dsi.dev, &dw_mipi_dsi_rockchip_ops);
    mutex_lock(&dsi.usage_mutex);
    dsi.usage_mode = DW_DSI_USAGE_IDLE;
    mutex_unlock(&dsi.usage_mutex);
    return 0;
    }
    static const struct dw_mipi_dsi_host_ops dw_mipi_dsi_rockchip_host_ops = {
    .attach = dw_mipi_dsi_rockchip_host_attach,
    .detach = dw_mipi_dsi_rockchip_host_detach,
    };
    static int dw_mipi_dsi_rockchip_dphy_bind(struct device *dev,
    struct device *master,
    void *data)
    {
//
// Nothing to do when used as a dphy.
// Just make the rest of Rockchip-DRM happy
// by being here.
//
    return 0;
    }
    static void dw_mipi_dsi_rockchip_dphy_unbind(struct device *dev,
    struct device *master,
    void *data)
    {
// Nothing to do when used as a dphy.
    }
    static const struct component_ops dw_mipi_dsi_rockchip_dphy_ops = {
    .bind	= dw_mipi_dsi_rockchip_dphy_bind,
    .unbind	= dw_mipi_dsi_rockchip_dphy_unbind,
    };
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_init(phy: *mut phy) -> c_int {
    static int dw_mipi_dsi_dphy_init(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    int ret;
    mutex_lock(&dsi.usage_mutex);
    if (dsi.usage_mode != DW_DSI_USAGE_IDLE) {
    DRM_DEV_ERROR(dsi.dev, "dsi controller already in use\n");
    mutex_unlock(&dsi.usage_mutex);
    return -EBUSY;
    }
    dsi.usage_mode = DW_DSI_USAGE_PHY;
    mutex_unlock(&dsi.usage_mutex);
    ret = component_add(dsi.dev, &dw_mipi_dsi_rockchip_dphy_ops);
    if (ret < 0)
    goto err_graph;
    if (dsi.cdata.dphy_rx_init) {
    ret = clk_prepare_enable(dsi.pclk);
    if (ret < 0)
    goto err_init;
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    clk_disable_unprepare(dsi.pclk);
    goto err_init;
    }
    ret = dsi.cdata.dphy_rx_init(phy);
    clk_disable_unprepare(dsi.grf_clk);
    clk_disable_unprepare(dsi.pclk);
    if (ret < 0)
    goto err_init;
    }
    return 0;
    err_init:
    component_del(dsi.dev, &dw_mipi_dsi_rockchip_dphy_ops);
    err_graph:
    mutex_lock(&dsi.usage_mutex);
    dsi.usage_mode = DW_DSI_USAGE_IDLE;
    mutex_unlock(&dsi.usage_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_exit(phy: *mut phy) -> c_int {
    static int dw_mipi_dsi_dphy_exit(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    component_del(dsi.dev, &dw_mipi_dsi_rockchip_dphy_ops);
    mutex_lock(&dsi.usage_mutex);
    dsi.usage_mode = DW_DSI_USAGE_IDLE;
    mutex_unlock(&dsi.usage_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int dw_mipi_dsi_dphy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct phy_configure_opts_mipi_dphy *config = &opts.mipi_dphy;
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    int ret;
    ret = phy_mipi_dphy_config_validate(&opts.mipi_dphy);
    if (ret)
    return ret;
    dsi.dphy_config = *config;
    dsi.lane_mbps = div_u64(config.hs_clk_rate, 1000 * 1000 * 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_power_on(phy: *mut phy) -> c_int {
    static int dw_mipi_dsi_dphy_power_on(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    int i, ret;
    DRM_DEV_DEBUG(dsi.dev, "lanes %d - data_rate_mbps %u\n",
    dsi.dphy_config.lanes, dsi.lane_mbps);
    i = max_mbps_to_parameter(dsi.lane_mbps);
    if (i < 0) {
    DRM_DEV_ERROR(dsi.dev, "failed to get parameter for %dmbps clock\n",
    dsi.lane_mbps);
    return i;
    }
    ret = pm_runtime_resume_and_get(dsi.dev);
    if (ret < 0) {
    DRM_DEV_ERROR(dsi.dev, "failed to enable device: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(dsi.pclk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable pclk: %d\n", ret);
    goto err_pclk;
    }
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable grf_clk: %d\n", ret);
    goto err_grf_clk;
    }
    ret = clk_prepare_enable(dsi.phy_cfg_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable phy_cfg_clk: %d\n", ret);
    goto err_phy_cfg_clk;
    }
// do soc-variant specific init
    if (dsi.cdata.dphy_rx_power_on) {
    ret = dsi.cdata.dphy_rx_power_on(phy);
    if (ret < 0) {
    DRM_DEV_ERROR(dsi.dev, "hardware-specific phy bringup failed: %d\n", ret);
    goto err_pwr_on;
    }
    }
//
// Configure hsfreqrange according to frequency values
// Set clock lane and hsfreqrange by lane0(test code 0x44)
//
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_CLK, 0);
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_0,
    HSFREQRANGE_SEL(dppa_map[i].hsfreqrange));
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_1, 0);
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_2, 0);
    dw_mipi_dsi_phy_write(dsi, HS_RX_CONTROL_OF_LANE_3, 0);
// Normal operation
    dw_mipi_dsi_phy_write(dsi, 0x0, 0);
    clk_disable_unprepare(dsi.phy_cfg_clk);
    clk_disable_unprepare(dsi.grf_clk);
    return ret;
    err_pwr_on:
    clk_disable_unprepare(dsi.phy_cfg_clk);
    err_phy_cfg_clk:
    clk_disable_unprepare(dsi.grf_clk);
    err_grf_clk:
    clk_disable_unprepare(dsi.pclk);
    err_pclk:
    pm_runtime_put(dsi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_power_off(phy: *mut phy) -> c_int {
    static int dw_mipi_dsi_dphy_power_off(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    int ret;
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable grf_clk: %d\n", ret);
    return ret;
    }
    if (dsi.cdata.dphy_rx_power_off) {
    ret = dsi.cdata.dphy_rx_power_off(phy);
    if (ret < 0)
    DRM_DEV_ERROR(dsi.dev, "hardware-specific phy shutdown failed: %d\n", ret);
    }
    clk_disable_unprepare(dsi.grf_clk);
    clk_disable_unprepare(dsi.pclk);
    pm_runtime_put(dsi.dev);
    return ret;
    }
    static const struct phy_ops dw_mipi_dsi_dphy_ops = {
    .configure	= dw_mipi_dsi_dphy_configure,
    .power_on	= dw_mipi_dsi_dphy_power_on,
    .power_off	= dw_mipi_dsi_dphy_power_off,
    .init		= dw_mipi_dsi_dphy_init,
    .exit		= dw_mipi_dsi_dphy_exit,
    };
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_rockchip_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dw_mipi_dsi_rockchip_resume(struct device *dev)
    {
    struct dw_mipi_dsi_rockchip *dsi = dev_get_drvdata(dev);
    int ret;
//
// Re-configure DSI state, if we were previously initialized. We need
// to do this before rockchip_drm_drv tries to re-enable() any panels.
//
    if (dsi.dsi_bound) {
    ret = clk_prepare_enable(dsi.grf_clk);
    if (ret) {
    DRM_DEV_ERROR(dsi.dev, "Failed to enable grf_clk: %d\n", ret);
    return ret;
    }
    dw_mipi_dsi_rockchip_config(dsi);
    if (dsi.slave)
    dw_mipi_dsi_rockchip_config(dsi.slave);
    clk_disable_unprepare(dsi.grf_clk);
    }
    return 0;
    }
    static const struct dev_pm_ops dw_mipi_dsi_rockchip_pm_ops = {
    SET_LATE_SYSTEM_SLEEP_PM_OPS(core::ptr::null_mut(), dw_mipi_dsi_rockchip_resume)
    };
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_rockchip_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mipi_dsi_rockchip_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct dw_mipi_dsi_rockchip *dsi;
    struct phy_provider *phy_provider;
    struct resource *res;
    const struct rockchip_dw_dsi_chip_data *cdata =
    of_device_get_match_data(dev);
    int ret, i;
    dsi = devm_kzalloc(dev, sizeof(*dsi), GFP_KERNEL);
    if (!dsi)
    return -ENOMEM;
    dsi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(dsi.base)) {
    DRM_DEV_ERROR(dev, "Unable to get dsi registers\n");
    return PTR_ERR(dsi.base);
    }
    i = 0;
    while (cdata[i].reg) {
    if (cdata[i].reg == res.start) {
    dsi.cdata = &cdata[i];
    break;
    }
    i++;
    }
    if (!dsi.cdata) {
    DRM_DEV_ERROR(dev, "no dsi-config for %s node\n", np.name);
    return -EINVAL;
    }
// try to get a possible external dphy
    dsi.phy = devm_phy_optional_get(dev, "dphy");
    if (IS_ERR(dsi.phy)) {
    ret = PTR_ERR(dsi.phy);
    DRM_DEV_ERROR(dev, "failed to get mipi dphy: %d\n", ret);
    return ret;
    }
    dsi.pclk = devm_clk_get(dev, "pclk");
    if (IS_ERR(dsi.pclk)) {
    ret = PTR_ERR(dsi.pclk);
    DRM_DEV_ERROR(dev, "Unable to get pclk: %d\n", ret);
    return ret;
    }
    dsi.pllref_clk = devm_clk_get(dev, "ref");
    if (IS_ERR(dsi.pllref_clk)) {
    if (dsi.phy) {
//
// if external phy is present, pll will be
// generated there.
//
    dsi.pllref_clk = core::ptr::null_mut();
    } else {
    ret = PTR_ERR(dsi.pllref_clk);
    DRM_DEV_ERROR(dev,
    "Unable to get pll reference clock: %d\n",
    ret);
    return ret;
    }
    }
    if (dsi.cdata.flags & DW_MIPI_NEEDS_PHY_CFG_CLK) {
    dsi.phy_cfg_clk = devm_clk_get(dev, "phy_cfg");
    if (IS_ERR(dsi.phy_cfg_clk)) {
    ret = PTR_ERR(dsi.phy_cfg_clk);
    DRM_DEV_ERROR(dev,
    "Unable to get phy_cfg_clk: %d\n", ret);
    return ret;
    }
    }
    if (dsi.cdata.flags & DW_MIPI_NEEDS_GRF_CLK) {
    dsi.grf_clk = devm_clk_get(dev, "grf");
    if (IS_ERR(dsi.grf_clk)) {
    ret = PTR_ERR(dsi.grf_clk);
    DRM_DEV_ERROR(dev, "Unable to get grf_clk: %d\n", ret);
    return ret;
    }
    }
    dsi.grf_regmap = syscon_regmap_lookup_by_phandle(np, "rockchip,grf");
    if (IS_ERR(dsi.grf_regmap)) {
    DRM_DEV_ERROR(dev, "Unable to get rockchip,grf\n");
    return PTR_ERR(dsi.grf_regmap);
    }
    dsi.dev = dev;
    dsi.pdata.base = dsi.base;
    dsi.pdata.max_data_lanes = dsi.cdata.max_data_lanes;
    dsi.pdata.phy_ops = &dw_mipi_dsi_rockchip_phy_ops;
    dsi.pdata.host_ops = &dw_mipi_dsi_rockchip_host_ops;
    dsi.pdata.priv_data = dsi;
    platform_set_drvdata(pdev, dsi);
    mutex_init(&dsi.usage_mutex);
    dsi.dphy = devm_phy_create(dev, core::ptr::null_mut(), &dw_mipi_dsi_dphy_ops);
    if (IS_ERR(dsi.dphy)) {
    DRM_DEV_ERROR(&pdev.dev, "failed to create PHY\n");
    return PTR_ERR(dsi.dphy);
    }
    phy_set_drvdata(dsi.dphy, dsi);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    dsi.dmd = dw_mipi_dsi_probe(pdev, &dsi.pdata);
    if (IS_ERR(dsi.dmd)) {
    ret = PTR_ERR(dsi.dmd);
    if (ret != -EPROBE_DEFER)
    DRM_DEV_ERROR(dev,
    "Failed to probe dw_mipi_dsi: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_rockchip_remove(pdev: *mut platform_device) {
    static void dw_mipi_dsi_rockchip_remove(struct platform_device *pdev)
    {
    struct dw_mipi_dsi_rockchip *dsi = platform_get_drvdata(pdev);
    dw_mipi_dsi_remove(dsi.dmd);
    }
    static const struct rockchip_dw_dsi_chip_data px30_chip_data[] = {
    {
    .reg = 0xff450000,
    .lcdsel_grf_reg = PX30_GRF_PD_VO_CON1,
    .lcdsel_big = FIELD_PREP_WM16_CONST(PX30_DSI_LCDC_SEL, 0),
    .lcdsel_lit = FIELD_PREP_WM16_CONST(PX30_DSI_LCDC_SEL, 1),
    .lanecfg1_grf_reg = PX30_GRF_PD_VO_CON1,
    .lanecfg1 = FIELD_PREP_WM16_CONST((PX30_DSI_TURNDISABLE |
    PX30_DSI_FORCERXMODE |
    PX30_DSI_FORCETXSTOPMODE), 0),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rk3128_chip_data[] = {
    {
    .reg = 0x10110000,
    .lanecfg1_grf_reg = RK3128_GRF_LVDS_CON0,
    .lanecfg1 = FIELD_PREP_WM16_CONST((RK3128_DSI_TURNDISABLE |
    RK3128_DSI_FORCERXMODE |
    RK3128_DSI_FORCETXSTOPMODE), 0),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rk3288_chip_data[] = {
    {
    .reg = 0xff960000,
    .lcdsel_grf_reg = RK3288_GRF_SOC_CON6,
    .lcdsel_big = FIELD_PREP_WM16_CONST(RK3288_DSI0_LCDC_SEL, 0),
    .lcdsel_lit = FIELD_PREP_WM16_CONST(RK3288_DSI0_LCDC_SEL, 1),
    .max_data_lanes = 4,
    },
    {
    .reg = 0xff964000,
    .lcdsel_grf_reg = RK3288_GRF_SOC_CON6,
    .lcdsel_big = FIELD_PREP_WM16_CONST(RK3288_DSI1_LCDC_SEL, 0),
    .lcdsel_lit = FIELD_PREP_WM16_CONST(RK3288_DSI1_LCDC_SEL, 1),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rk3368_chip_data[] = {
    {
    .reg = 0xff960000,
    .lanecfg1_grf_reg = RK3368_GRF_SOC_CON7,
    .lanecfg1 = FIELD_PREP_WM16_CONST((RK3368_DSI_TURNDISABLE |
    RK3368_DSI_FORCETXSTOPMODE |
    RK3368_DSI_FORCERXMODE), 0),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn rk3399_dphy_tx1rx1_init(phy: *mut phy) -> c_int {
    static int rk3399_dphy_tx1rx1_init(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
//
// Set TX1RX1 source to isp1.
// Assume ISP0 is supplied by the RX0 dphy.
//
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_SRC_SEL_ISP0, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_MASTERSLAVEZ, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_BASEDIR, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_ENABLE, 0));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk3399_dphy_tx1rx1_power_on(phy: *mut phy) -> c_int {
    static int rk3399_dphy_tx1rx1_power_on(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
// tester reset pulse
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_TESTCLK | PHY_TESTCLR);
    usleep_range(100, 150);
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_MASTERSLAVEZ, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_BASEDIR, 1));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_FORCERXMODE, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_FORCETXSTOPMODE, 0));
// Disable lane turn around, which is ignored in receive mode
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON24,
    FIELD_PREP_WM16(RK3399_TXRX_TURNREQUEST, 0));
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_TURNDISABLE, 0xf));
    usleep_range(100, 150);
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_TESTCLK | PHY_UNTESTCLR);
    usleep_range(100, 150);
// Enable dphy lanes
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_ENABLE,
    GENMASK(dsi.dphy_config.lanes - 1, 0)));
    usleep_range(100, 150);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk3399_dphy_tx1rx1_power_off(phy: *mut phy) -> c_int {
    static int rk3399_dphy_tx1rx1_power_off(struct phy *phy)
    {
    struct dw_mipi_dsi_rockchip *dsi = phy_get_drvdata(phy);
    regmap_write(dsi.grf_regmap, RK3399_GRF_SOC_CON23,
    FIELD_PREP_WM16(RK3399_DSI1_ENABLE, 0));
    return 0;
    }
    static const struct rockchip_dw_dsi_chip_data rk3399_chip_data[] = {
    {
    .reg = 0xff960000,
    .lcdsel_grf_reg = RK3399_GRF_SOC_CON20,
    .lcdsel_big = FIELD_PREP_WM16_CONST(RK3399_DSI0_LCDC_SEL, 0),
    .lcdsel_lit = FIELD_PREP_WM16_CONST(RK3399_DSI0_LCDC_SEL, 1),
    .lanecfg1_grf_reg = RK3399_GRF_SOC_CON22,
    .lanecfg1 = FIELD_PREP_WM16_CONST((RK3399_DSI0_TURNREQUEST |
    RK3399_DSI0_TURNDISABLE |
    RK3399_DSI0_FORCETXSTOPMODE |
    RK3399_DSI0_FORCERXMODE), 0),
    .flags = DW_MIPI_NEEDS_PHY_CFG_CLK | DW_MIPI_NEEDS_GRF_CLK,
    .max_data_lanes = 4,
    },
    {
    .reg = 0xff968000,
    .lcdsel_grf_reg = RK3399_GRF_SOC_CON20,
    .lcdsel_big = FIELD_PREP_WM16_CONST(RK3399_DSI1_LCDC_SEL, 0),
    .lcdsel_lit = FIELD_PREP_WM16_CONST(RK3399_DSI1_LCDC_SEL, 1),
    .lanecfg1_grf_reg = RK3399_GRF_SOC_CON23,
    .lanecfg1 = FIELD_PREP_WM16_CONST((RK3399_DSI1_TURNDISABLE |
    RK3399_DSI1_FORCETXSTOPMODE |
    RK3399_DSI1_FORCERXMODE |
    RK3399_DSI1_ENABLE), 0),
    .lanecfg2_grf_reg = RK3399_GRF_SOC_CON24,
    .lanecfg2 = (FIELD_PREP_WM16_CONST(RK3399_TXRX_MASTERSLAVEZ, 1) |
    FIELD_PREP_WM16_CONST(RK3399_TXRX_ENABLECLK, 1) |
    FIELD_PREP_WM16_CONST(RK3399_TXRX_BASEDIR, 0)),
    .enable_grf_reg = RK3399_GRF_SOC_CON23,
    .enable = FIELD_PREP_WM16_CONST(RK3399_DSI1_ENABLE, RK3399_DSI1_ENABLE),
    .flags = DW_MIPI_NEEDS_PHY_CFG_CLK | DW_MIPI_NEEDS_GRF_CLK,
    .max_data_lanes = 4,
    .dphy_rx_init = rk3399_dphy_tx1rx1_init,
    .dphy_rx_power_on = rk3399_dphy_tx1rx1_power_on,
    .dphy_rx_power_off = rk3399_dphy_tx1rx1_power_off,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rk3506_chip_data[] = {
    {
    .reg = 0xff640000,
    .lanecfg1_grf_reg = RK3506_SYS_GRF_SOC_CON6,
    .lanecfg1 = (FIELD_PREP_WM16_CONST(RK3506_DSI_TURNDISABLE, 0) |
    FIELD_PREP_WM16_CONST(RK3506_DSI_FORCERXMODE, 0) |
    FIELD_PREP_WM16_CONST(RK3506_DSI_FORCETXSTOPMODE, 0)),
    .max_data_lanes = 2,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rk3568_chip_data[] = {
    {
    .reg = 0xfe060000,
    .lanecfg1_grf_reg = RK3568_GRF_VO_CON2,
    .lanecfg1 = (FIELD_PREP_WM16_CONST(RK3568_DSI0_SKEWCALHS, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI0_FORCETXSTOPMODE, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI0_TURNDISABLE, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI0_FORCERXMODE, 0)),
    .max_data_lanes = 4,
    },
    {
    .reg = 0xfe070000,
    .lanecfg1_grf_reg = RK3568_GRF_VO_CON3,
    .lanecfg1 = (FIELD_PREP_WM16_CONST(RK3568_DSI1_SKEWCALHS, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI1_FORCETXSTOPMODE, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI1_TURNDISABLE, 0) |
    FIELD_PREP_WM16_CONST(RK3568_DSI1_FORCERXMODE, 0)),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
    static const struct rockchip_dw_dsi_chip_data rv1126_chip_data[] = {
    {
    .reg = 0xffb30000,
    .lanecfg1_grf_reg = RV1126_GRF_DSIPHY_CON,
    .lanecfg1 = (FIELD_PREP_WM16_CONST(RV1126_DSI_TURNDISABLE, 0) |
    FIELD_PREP_WM16_CONST(RV1126_DSI_FORCERXMODE, 0) |
    FIELD_PREP_WM16_CONST(RV1126_DSI_FORCETXSTOPMODE, 0)),
    .max_data_lanes = 4,
    },
    { /* sentinel */ }
    };
    static const struct of_device_id dw_mipi_dsi_rockchip_dt_ids[] = {
    {
    .compatible = "rockchip,px30-mipi-dsi",
    .data = &px30_chip_data,
    }, {
    .compatible = "rockchip,rk3128-mipi-dsi",
    .data = &rk3128_chip_data,
    }, {
    .compatible = "rockchip,rk3288-mipi-dsi",
    .data = &rk3288_chip_data,
    }, {
    .compatible = "rockchip,rk3368-mipi-dsi",
    .data = &rk3368_chip_data,
    }, {
    .compatible = "rockchip,rk3399-mipi-dsi",
    .data = &rk3399_chip_data,
    }, {
    .compatible = "rockchip,rk3506-mipi-dsi",
    .data = &rk3506_chip_data,
    }, {
    .compatible = "rockchip,rk3568-mipi-dsi",
    .data = &rk3568_chip_data,
    }, {
    .compatible = "rockchip,rv1126-mipi-dsi",
    .data = &rv1126_chip_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dw_mipi_dsi_rockchip_dt_ids);
    struct platform_driver dw_mipi_dsi_rockchip_driver = {
    .probe		= dw_mipi_dsi_rockchip_probe,
    .remove		= dw_mipi_dsi_rockchip_remove,
    .driver		= {
    .of_match_table = dw_mipi_dsi_rockchip_dt_ids,
    .pm	= &dw_mipi_dsi_rockchip_pm_ops,
    .name	= "dw-mipi-dsi-rockchip",
//
// For dual-DSI display, one DSI pokes at the other DSI's
// drvdata in dw_mipi_dsi_rockchip_find_second(). This is not
// safe for asynchronous probe.
//
    .probe_type = PROBE_FORCE_SYNCHRONOUS,
    },
    };
