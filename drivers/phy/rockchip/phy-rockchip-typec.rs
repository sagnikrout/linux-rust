//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-typec.c
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author: Chris Zhong <zyw@rock-chips.com>
// Kever Yang <kever.yang@rock-chips.com>
//
// The ROCKCHIP Type-C PHY has two PLL clocks. The first PLL clock
// is used for USB3, the second PLL clock is used for DP. This Type-C PHY has
// 3 working modes: USB3 only mode, DP only mode, and USB3+DP mode.
// At USB3 only mode, both PLL clocks need to be initialized, this allows the
// PHY to switch mode between USB3 and USB3+DP, without disconnecting the USB
// device.
// In The DP only mode, only the DP PLL needs to be powered on, and the 4 lanes
// are all used for DP.
//
// This driver gets extcon cable state and property, then decides which mode to
// select:
//
// 1. USB3 only mode:
// EXTCON_USB or EXTCON_USB_HOST state is true, and
// EXTCON_PROP_USB_SS property is true.
// EXTCON_DISP_DP state is false.
//
// 2. DP only mode:
// EXTCON_DISP_DP state is true, and
// EXTCON_PROP_USB_SS property is false.
// If EXTCON_USB_HOST state is true, it is DP + USB2 mode, since the USB2 phy
// is a separate phy, so this case is still DP only mode.
//
// 3. USB3+DP mode:
// EXTCON_USB_HOST and EXTCON_DISP_DP are both true, and
// EXTCON_PROP_USB_SS property is true.
//
// This Type-C PHY driver supports normal and flip orientation. The orientation
// is reported by the EXTCON_PROP_USB_TYPEC_POLARITY property: true is flip
// orientation, false is normal orientation.
//

// For CMN_TXPUCAL_CTRL, CMN_TXPDCAL_CTRL

//
// For CMN_TXPUCAL_CTRL, CMN_TXPDCAL_CTRL,
// CMN_TXPU_ADJ_CTRL, CMN_TXPDCAL_CTRL
//
// NOTE: some of these registers are documented to be 2's complement
// signed numbers, but then documented to be always positive.  Weird.
// In such a case, using CMN_CALIB_CODE_POS() avoids the unnecessary
// sign extension.
//
pub const CMN_CALIB_CODE_WIDTH: c_int = 7;
pub const CMN_CALIB_CODE_OFFSET: c_int = 0;

    sign_extend32((x) >> CMN_CALIB_CODE_OFFSET, CMN_CALIB_CODE_WIDTH)

    (((x) >> CMN_CALIB_CODE_OFFSET) & CMN_CALIB_CODE_POS_MASK)

// Use this for "n" in macros like "_MULT_XXX" to target the aux channel
pub const AUX_CH_LANE: c_int = 8;

pub const TX_VMARGIN_OFFSET: c_int = 3;
pub const TX_VMARGIN_MASK: c_uint = 0x7;

pub const TX_RESCAL_CODE_OFFSET: c_int = 0;
pub const TX_RESCAL_CODE_MASK: c_uint = 0x3f;

//
// Selects which PLL clock will be driven on the analog high speed
// clock 0: PLL 0 div 1
// clock 1: PLL 1 div 2
//

pub const CLK_PLL_MASK: c_uint = 0x33;

pub const DP_MODE_ENTER_A0: c_uint = 0xc101;
pub const DP_MODE_ENTER_A2: c_uint = 0xc104;
pub const PHY_MODE_SET_TIMEOUT: c_int = 100000;
pub const PIN_ASSIGN_C_E: c_uint = 0x51d9;
pub const PIN_ASSIGN_D_F: c_uint = 0x5100;
pub const MODE_DISCONNECT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb3phy_reg {
    pub offset: u32,
    pub enable_bit: u32,
    pub write_enable: u32,
}

//
// struct rockchip_usb3phy_port_cfg - usb3-phy port configuration.
// @reg: the base address for usb3-phy config.
// @typec_conn_dir: the register of type-c connector direction.
// @usb3tousb2_en: the register of type-c force usb2 to usb2 enable.
// @external_psm: the register of type-c phy external psm clock.
// @pipe_status: the register of type-c phy pipe status.
// @usb3_host_disable: the register of type-c usb3 host disable.
// @usb3_host_port: the register of type-c usb3 host port.
// @uphy_dp_sel: the register of type-c phy DP select control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_usb3phy_port_cfg {
    pub reg: c_uint,
    pub typec_conn_dir: usb3phy_reg,
    pub usb3tousb2_en: usb3phy_reg,
    pub external_psm: usb3phy_reg,
    pub pipe_status: usb3phy_reg,
    pub usb3_host_disable: usb3phy_reg,
    pub usb3_host_port: usb3phy_reg,
    pub uphy_dp_sel: usb3phy_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_typec_phy {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub extcon: *mut extcon_dev,
    pub grf_regs: *mut regmap,
    pub clk_core: *mut clk,
    pub clk_ref: *mut clk,
    pub uphy_rst: *mut reset_control,
    pub pipe_rst: *mut reset_control,
    pub tcphy_rst: *mut reset_control,
    pub port_cfgs: *const rockchip_usb3phy_port_cfg,
// mutex to protect access to individual PHYs
    pub lock: mutex,
    pub flip: bool,
    pub mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_reg {
    pub value: u16,
    pub addr: u32,
}

    static struct phy_reg usb3_pll_cfg[] = {
    { 0xf0,		CMN_PLL0_VCOCAL_INIT },
    { 0x18,		CMN_PLL0_VCOCAL_ITER },
    { 0xd0,		CMN_PLL0_INTDIV },
    { 0x4a4a,	CMN_PLL0_FRACDIV },
    { 0x34,		CMN_PLL0_HIGH_THR },
    { 0x1ee,	CMN_PLL0_SS_CTRL1 },
    { 0x7f03,	CMN_PLL0_SS_CTRL2 },
    { 0x20,		CMN_PLL0_DSM_DIAG },
    { 0,		CMN_DIAG_PLL0_OVRD },
    { 0,		CMN_DIAG_PLL0_FBH_OVRD },
    { 0,		CMN_DIAG_PLL0_FBL_OVRD },
    { 0x7,		CMN_DIAG_PLL0_V2I_TUNE },
    { 0x45,		CMN_DIAG_PLL0_CP_TUNE },
    { 0x8,		CMN_DIAG_PLL0_LF_PROG },
    };
    static struct phy_reg dp_pll_cfg[] = {
    { 0xf0,		CMN_PLL1_VCOCAL_INIT },
    { 0x18,		CMN_PLL1_VCOCAL_ITER },
    { 0x30b9,	CMN_PLL1_VCOCAL_START },
    { 0x21c,	CMN_PLL1_INTDIV },
    { 0,		CMN_PLL1_FRACDIV },
    { 0x5,		CMN_PLL1_HIGH_THR },
    { 0x35,		CMN_PLL1_SS_CTRL1 },
    { 0x7f1e,	CMN_PLL1_SS_CTRL2 },
    { 0x20,		CMN_PLL1_DSM_DIAG },
    { 0,		CMN_PLLSM1_USER_DEF_CTRL },
    { 0,		CMN_DIAG_PLL1_OVRD },
    { 0,		CMN_DIAG_PLL1_FBH_OVRD },
    { 0,		CMN_DIAG_PLL1_FBL_OVRD },
    { 0x6,		CMN_DIAG_PLL1_V2I_TUNE },
    { 0x45,		CMN_DIAG_PLL1_CP_TUNE },
    { 0x8,		CMN_DIAG_PLL1_LF_PROG },
    { 0x100,	CMN_DIAG_PLL1_PTATIS_TUNE1 },
    { 0x7,		CMN_DIAG_PLL1_PTATIS_TUNE2 },
    { 0x4,		CMN_DIAG_PLL1_INCLK_CTRL },
    };
    static const struct rockchip_usb3phy_port_cfg rk3399_usb3phy_port_cfgs[] = {
    {
    .reg = 0xff7c0000,
    .typec_conn_dir	= { 0xe580, 0, 16 },
    .usb3tousb2_en	= { 0xe580, 3, 19 },
    .external_psm	= { 0xe588, 14, 30 },
    .pipe_status	= { 0xe5c0, 0, 0 },
    .usb3_host_disable = { 0x2434, 0, 16 },
    .usb3_host_port = { 0x2434, 12, 28 },
    .uphy_dp_sel	= { 0x6268, 19, 19 },
    },
    {
    .reg = 0xff800000,
    .typec_conn_dir	= { 0xe58c, 0, 16 },
    .usb3tousb2_en	= { 0xe58c, 3, 19 },
    .external_psm	= { 0xe594, 14, 30 },
    .pipe_status	= { 0xe5c0, 16, 16 },
    .usb3_host_disable = { 0x2444, 0, 16 },
    .usb3_host_port = { 0x2444, 12, 28 },
    .uphy_dp_sel	= { 0x6268, 3, 19 },
    },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn tcphy_cfg_24m(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_cfg_24m(struct rockchip_typec_phy *tcphy)
    {
    u32 i, rdata;
//
// cmn_ref_clk_sel = 3, select the 24Mhz for clk parent
// cmn_psm_clk_dig_div = 2, set the clk division to 2
//
    writel(0x830, tcphy.base + PMA_CMN_CTRL1);
    for (i = 0; i < 4; i++) {
//
// The following PHY configuration assumes a 24 MHz reference
// clock.
//
    writel(0x90, tcphy.base + XCVR_DIAG_LANE_FCM_EN_MGN(i));
    writel(0x960, tcphy.base + TX_RCVDET_EN_TMR(i));
    writel(0x30, tcphy.base + TX_RCVDET_ST_TMR(i));
    }
    rdata = readl(tcphy.base + CMN_DIAG_HSCLK_SEL);
    rdata &= ~CLK_PLL_MASK;
    rdata |= CLK_PLL_CONFIG;
    writel(rdata, tcphy.base + CMN_DIAG_HSCLK_SEL);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_cfg_usb3_pll(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_cfg_usb3_pll(struct rockchip_typec_phy *tcphy)
    {
    u32 i;
// load the configuration of PLL0
    for (i = 0; i < ARRAY_SIZE(usb3_pll_cfg); i++)
    writel(usb3_pll_cfg[i].value,
    tcphy.base + usb3_pll_cfg[i].addr);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_cfg_dp_pll(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_cfg_dp_pll(struct rockchip_typec_phy *tcphy)
    {
    u32 i;
// set the default mode to RBR
    writel(DP_PLL_CLOCK_ENABLE | DP_PLL_ENABLE | DP_PLL_DATA_RATE_RBR,
    tcphy.base + DP_CLK_CTL);
// load the configuration of PLL1
    for (i = 0; i < ARRAY_SIZE(dp_pll_cfg); i++)
    writel(dp_pll_cfg[i].value, tcphy.base + dp_pll_cfg[i].addr);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_tx_usb3_cfg_lane(tcphy: *mut rockchip_typec_phy, lane: u32) {
    static void tcphy_tx_usb3_cfg_lane(struct rockchip_typec_phy *tcphy, u32 lane)
    {
    writel(0x7799, tcphy.base + TX_PSC_A0(lane));
    writel(0x7798, tcphy.base + TX_PSC_A1(lane));
    writel(0x5098, tcphy.base + TX_PSC_A2(lane));
    writel(0x5098, tcphy.base + TX_PSC_A3(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_000(lane));
    writel(0xbf, tcphy.base + XCVR_DIAG_BIDI_CTRL(lane));
    }
#[no_mangle]
unsafe extern "C" fn tcphy_rx_usb3_cfg_lane(tcphy: *mut rockchip_typec_phy, lane: u32) {
    static void tcphy_rx_usb3_cfg_lane(struct rockchip_typec_phy *tcphy, u32 lane)
    {
    writel(0xa6fd, tcphy.base + RX_PSC_A0(lane));
    writel(0xa6fd, tcphy.base + RX_PSC_A1(lane));
    writel(0xa410, tcphy.base + RX_PSC_A2(lane));
    writel(0x2410, tcphy.base + RX_PSC_A3(lane));
    writel(0x23ff, tcphy.base + RX_PSC_CAL(lane));
    writel(0x13, tcphy.base + RX_SIGDET_HL_FILT_TMR(lane));
    writel(0x03e7, tcphy.base + RX_REE_CTRL_DATA_MASK(lane));
    writel(0x1004, tcphy.base + RX_DIAG_SIGDET_TUNE(lane));
    writel(0x2010, tcphy.base + RX_PSC_RDY(lane));
    writel(0xfb, tcphy.base + XCVR_DIAG_BIDI_CTRL(lane));
    }
#[no_mangle]
unsafe extern "C" fn tcphy_dp_cfg_lane(tcphy: *mut rockchip_typec_phy, lane: u32) {
    static void tcphy_dp_cfg_lane(struct rockchip_typec_phy *tcphy, u32 lane)
    {
    u16 rdata;
    writel(0xbefc, tcphy.base + XCVR_PSM_RCTRL(lane));
    writel(0x6799, tcphy.base + TX_PSC_A0(lane));
    writel(0x6798, tcphy.base + TX_PSC_A1(lane));
    writel(0x98, tcphy.base + TX_PSC_A2(lane));
    writel(0x98, tcphy.base + TX_PSC_A3(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_000(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_001(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_010(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_011(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_100(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_101(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_110(lane));
    writel(0, tcphy.base + TX_TXCC_MGNFS_MULT_111(lane));
    writel(0, tcphy.base + TX_TXCC_CPOST_MULT_10(lane));
    writel(0, tcphy.base + TX_TXCC_CPOST_MULT_01(lane));
    writel(0, tcphy.base + TX_TXCC_CPOST_MULT_00(lane));
    writel(0, tcphy.base + TX_TXCC_CPOST_MULT_11(lane));
    writel(0x128, tcphy.base + TX_TXCC_CAL_SCLR_MULT(lane));
    writel(0x400, tcphy.base + TX_DIAG_TX_DRV(lane));
    rdata = readl(tcphy.base + XCVR_DIAG_PLLDRC_CTRL(lane));
    rdata = (rdata & 0x8fff) | 0x6000;
    writel(rdata, tcphy.base + XCVR_DIAG_PLLDRC_CTRL(lane));
    }
    static inline int property_enable(struct rockchip_typec_phy *tcphy,
    const struct usb3phy_reg *reg, bool en)
    {
    let mut mask: u32 = 1 << reg.write_enable;
    let mut val: u32 = en << reg.enable_bit;
    return regmap_write(tcphy.grf_regs, reg.offset, val | mask);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_dp_aux_set_flip(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_dp_aux_set_flip(struct rockchip_typec_phy *tcphy)
    {
    u16 tx_ana_ctrl_reg_1;
//
// Select the polarity of the xcvr:
// 1, Reverses the polarity (If TYPEC, Pulls ups aux_p and pull
// down aux_m)
// 0, Normal polarity (if TYPEC, pulls up aux_m and pulls down
// aux_p)
//
    tx_ana_ctrl_reg_1 = readl(tcphy.base + TX_ANA_CTRL_REG_1);
    if (!tcphy.flip)
    tx_ana_ctrl_reg_1 |= AUXDA_POLARITY;
    else
    tx_ana_ctrl_reg_1 &= ~AUXDA_POLARITY;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_dp_aux_calibration(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_dp_aux_calibration(struct rockchip_typec_phy *tcphy)
    {
    u16 val;
    u16 tx_ana_ctrl_reg_1;
    u16 tx_ana_ctrl_reg_2;
    s32 pu_calib_code, pd_calib_code;
    s32 pu_adj, pd_adj;
    u16 calib;
//
// Calculate calibration code as per docs: use an average of the
// pull down and pull up.  Then add in adjustments.
//
    val = readl(tcphy.base + CMN_TXPUCAL_CTRL);
    pu_calib_code = CMN_CALIB_CODE_POS(val);
    val = readl(tcphy.base + CMN_TXPDCAL_CTRL);
    pd_calib_code = CMN_CALIB_CODE_POS(val);
    val = readl(tcphy.base + CMN_TXPU_ADJ_CTRL);
    pu_adj = CMN_CALIB_CODE(val);
    val = readl(tcphy.base + CMN_TXPD_ADJ_CTRL);
    pd_adj = CMN_CALIB_CODE(val);
    calib = (pu_calib_code + pd_calib_code) / 2 + pu_adj + pd_adj;
// disable txda_cal_latch_en for rewrite the calibration values
    tx_ana_ctrl_reg_1 = readl(tcphy.base + TX_ANA_CTRL_REG_1);
    tx_ana_ctrl_reg_1 &= ~TXDA_CAL_LATCH_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
// write the calibration, then delay 10 ms as sample in docs
    val = readl(tcphy.base + TX_DIG_CTRL_REG_2);
    val &= ~(TX_RESCAL_CODE_MASK << TX_RESCAL_CODE_OFFSET);
    val |= calib << TX_RESCAL_CODE_OFFSET;
    writel(val, tcphy.base + TX_DIG_CTRL_REG_2);
    usleep_range(10000, 10050);
//
// Enable signal for latch that sample and holds calibration values.
// Activate this signal for 1 clock cycle to sample new calibration
// values.
//
    tx_ana_ctrl_reg_1 |= TXDA_CAL_LATCH_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    usleep_range(150, 200);
// set TX Voltage Level and TX Deemphasis to 0
    writel(0, tcphy.base + PHY_DP_TX_CTL);
// re-enable decap
    tx_ana_ctrl_reg_2 = XCVR_DECAP_EN;
    writel(tx_ana_ctrl_reg_2, tcphy.base + TX_ANA_CTRL_REG_2);
    udelay(1);
    tx_ana_ctrl_reg_2 |= XCVR_DECAP_EN_DEL;
    writel(tx_ana_ctrl_reg_2, tcphy.base + TX_ANA_CTRL_REG_2);
    writel(0, tcphy.base + TX_ANA_CTRL_REG_3);
    tx_ana_ctrl_reg_1 |= TXDA_UPHY_SUPPLY_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    udelay(1);
    tx_ana_ctrl_reg_1 |= TXDA_UPHY_SUPPLY_EN_DEL;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    writel(0, tcphy.base + TX_ANA_CTRL_REG_5);
//
// Programs txda_drv_ldo_prog[15:0], Sets driver LDO
// voltage 16'h1001 for DP-AUX-TX and RX
//
    writel(0x1001, tcphy.base + TX_ANA_CTRL_REG_4);
// re-enables Bandgap reference for LDO
    tx_ana_ctrl_reg_1 |= TXDA_DRV_LDO_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    udelay(5);
    tx_ana_ctrl_reg_1 |= TXDA_BGREF_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
//
// re-enables the transmitter pre-driver, driver data selection MUX,
// and receiver detect circuits.
//
    tx_ana_ctrl_reg_2 |= TXDA_DRV_PREDRV_EN;
    writel(tx_ana_ctrl_reg_2, tcphy.base + TX_ANA_CTRL_REG_2);
    udelay(1);
    tx_ana_ctrl_reg_2 |= TXDA_DRV_PREDRV_EN_DEL;
    writel(tx_ana_ctrl_reg_2, tcphy.base + TX_ANA_CTRL_REG_2);
//
// Do all the undocumented magic:
// - Turn on TXDA_DP_AUX_EN, whatever that is, even though sample
// never shows this going on.
// - Turn on TXDA_DECAP_EN (and TXDA_DECAP_EN_DEL) even though
// docs say for aux it's always 0.
// - Turn off the LDO and BGREF, which we just spent time turning
// on above (???).
//
// Without this magic, things seem worse.
//
    tx_ana_ctrl_reg_1 |= TXDA_DP_AUX_EN;
    tx_ana_ctrl_reg_1 |= TXDA_DECAP_EN;
    tx_ana_ctrl_reg_1 &= ~TXDA_DRV_LDO_EN;
    tx_ana_ctrl_reg_1 &= ~TXDA_BGREF_EN;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
    udelay(1);
    tx_ana_ctrl_reg_1 |= TXDA_DECAP_EN_DEL;
    writel(tx_ana_ctrl_reg_1, tcphy.base + TX_ANA_CTRL_REG_1);
//
// Undo the work we did to set the LDO voltage.
// This doesn't seem to help nor hurt, but it kinda goes with the
// undocumented magic above.
//
    writel(0, tcphy.base + TX_ANA_CTRL_REG_4);
// Don't set voltage swing to 400 mV peak to peak (differential)
    writel(0, tcphy.base + TXDA_COEFF_CALC_CTRL);
// Init TXDA_CYA_AUXDA_CYA for unknown magic reasons
    writel(0, tcphy.base + TXDA_CYA_AUXDA_CYA);
//
// More undocumented magic, presumably the goal of which is to
// make the "auxda_source_aux_oen" be ignored and instead to decide
// about "high impedance state" based on what software puts in the
// register TXDA_COEFF_CALC_CTRL (see TX_HIGH_Z).  Since we only
// program that register once and we don't set the bit TX_HIGH_Z,
// presumably the goal here is that we should never put the analog
// driver in high impedance state.
//
    val = readl(tcphy.base + TX_DIG_CTRL_REG_2);
    val |= TX_HIGH_Z_TM_EN;
    writel(val, tcphy.base + TX_DIG_CTRL_REG_2);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_phy_init(tcphy: *mut rockchip_typec_phy, mode: u8) -> c_int {
    static int tcphy_phy_init(struct rockchip_typec_phy *tcphy, u8 mode)
    {
    const struct rockchip_usb3phy_port_cfg *cfg = tcphy.port_cfgs;
    int ret, i;
    u32 val;
    ret = clk_prepare_enable(tcphy.clk_core);
    if (ret) {
    dev_err(tcphy.dev, "Failed to prepare_enable core clock\n");
    return ret;
    }
    ret = clk_prepare_enable(tcphy.clk_ref);
    if (ret) {
    dev_err(tcphy.dev, "Failed to prepare_enable ref clock\n");
    goto err_clk_core;
    }
    reset_control_deassert(tcphy.tcphy_rst);
    property_enable(tcphy, &cfg.typec_conn_dir, tcphy.flip);
    tcphy_dp_aux_set_flip(tcphy);
    tcphy_cfg_24m(tcphy);
    if (mode == MODE_DFP_DP) {
    tcphy_cfg_dp_pll(tcphy);
    for (i = 0; i < 4; i++)
    tcphy_dp_cfg_lane(tcphy, i);
    writel(PIN_ASSIGN_C_E, tcphy.base + PMA_LANE_CFG);
    } else {
    tcphy_cfg_usb3_pll(tcphy);
    tcphy_cfg_dp_pll(tcphy);
    if (tcphy.flip) {
    tcphy_tx_usb3_cfg_lane(tcphy, 3);
    tcphy_rx_usb3_cfg_lane(tcphy, 2);
    tcphy_dp_cfg_lane(tcphy, 0);
    tcphy_dp_cfg_lane(tcphy, 1);
    } else {
    tcphy_tx_usb3_cfg_lane(tcphy, 0);
    tcphy_rx_usb3_cfg_lane(tcphy, 1);
    tcphy_dp_cfg_lane(tcphy, 2);
    tcphy_dp_cfg_lane(tcphy, 3);
    }
    writel(PIN_ASSIGN_D_F, tcphy.base + PMA_LANE_CFG);
    }
    writel(DP_MODE_ENTER_A2, tcphy.base + DP_MODE_CTL);
    reset_control_deassert(tcphy.uphy_rst);
    ret = readx_poll_timeout(readl, tcphy.base + PMA_CMN_CTRL1,
    val, val & CMN_READY, 10,
    PHY_MODE_SET_TIMEOUT);
    if (ret < 0) {
    dev_err(tcphy.dev, "wait pma ready timeout\n");
    ret = -ETIMEDOUT;
    goto err_wait_pma;
    }
    reset_control_deassert(tcphy.pipe_rst);
    return 0;
    err_wait_pma:
    reset_control_assert(tcphy.uphy_rst);
    reset_control_assert(tcphy.tcphy_rst);
    clk_disable_unprepare(tcphy.clk_ref);
    err_clk_core:
    clk_disable_unprepare(tcphy.clk_core);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcphy_phy_deinit(tcphy: *mut rockchip_typec_phy) {
    static void tcphy_phy_deinit(struct rockchip_typec_phy *tcphy)
    {
    reset_control_assert(tcphy.tcphy_rst);
    reset_control_assert(tcphy.uphy_rst);
    reset_control_assert(tcphy.pipe_rst);
    clk_disable_unprepare(tcphy.clk_core);
    clk_disable_unprepare(tcphy.clk_ref);
    }
#[no_mangle]
unsafe extern "C" fn tcphy_get_mode(tcphy: *mut rockchip_typec_phy) -> c_int {
    static int tcphy_get_mode(struct rockchip_typec_phy *tcphy)
    {
    struct extcon_dev *edev = tcphy.extcon;
    union extcon_property_value property;
    unsigned int id;
    u8 mode;
    int ret, ufp, dp;
    if (!edev)
    return MODE_DFP_USB;
    ufp = extcon_get_state(edev, EXTCON_USB);
    dp = extcon_get_state(edev, EXTCON_DISP_DP);
    mode = MODE_DFP_USB;
    id = EXTCON_USB_HOST;
    if (ufp > 0) {
    mode = MODE_UFP_USB;
    id = EXTCON_USB;
    } else if (dp > 0) {
    mode = MODE_DFP_DP;
    id = EXTCON_DISP_DP;
    ret = extcon_get_property(edev, id, EXTCON_PROP_USB_SS,
    &property);
    if (ret) {
    dev_err(tcphy.dev, "get superspeed property failed\n");
    return ret;
    }
    if (property.intval)
    mode |= MODE_DFP_USB;
    }
    ret = extcon_get_property(edev, id, EXTCON_PROP_USB_TYPEC_POLARITY,
    &property);
    if (ret) {
    dev_err(tcphy.dev, "get polarity property failed\n");
    return ret;
    }
    tcphy.flip = property.intval ? 1 : 0;
    return mode;
    }
    static int tcphy_cfg_usb3_to_usb2_only(struct rockchip_typec_phy *tcphy,
    bool value)
    {
    const struct rockchip_usb3phy_port_cfg *cfg = tcphy.port_cfgs;
    property_enable(tcphy, &cfg.usb3tousb2_en, value);
    property_enable(tcphy, &cfg.usb3_host_disable, value);
    property_enable(tcphy, &cfg.usb3_host_port, !value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_usb3_phy_power_on(phy: *mut phy) -> c_int {
    static int rockchip_usb3_phy_power_on(struct phy *phy)
    {
    struct rockchip_typec_phy *tcphy = phy_get_drvdata(phy);
    const struct rockchip_usb3phy_port_cfg *cfg = tcphy.port_cfgs;
    const struct usb3phy_reg *reg = &cfg.pipe_status;
    int timeout, new_mode, ret = 0;
    u32 val;
    mutex_lock(&tcphy.lock);
    new_mode = tcphy_get_mode(tcphy);
    if (new_mode < 0) {
    ret = new_mode;
    goto unlock_ret;
    }
// DP-only mode; fall back to USB2
    if (!(new_mode & (MODE_DFP_USB | MODE_UFP_USB))) {
    tcphy_cfg_usb3_to_usb2_only(tcphy, true);
    goto unlock_ret;
    }
    if (tcphy.mode == new_mode)
    goto unlock_ret;
    if (tcphy.mode == MODE_DISCONNECT) {
    ret = tcphy_phy_init(tcphy, new_mode);
    if (ret)
    goto unlock_ret;
    }
// wait TCPHY for pipe ready
    for (timeout = 0; timeout < 100; timeout++) {
    regmap_read(tcphy.grf_regs, reg.offset, &val);
    if (!(val & BIT(reg.enable_bit))) {
    tcphy.mode |= new_mode & (MODE_DFP_USB | MODE_UFP_USB);
// enable usb3 host
    tcphy_cfg_usb3_to_usb2_only(tcphy, false);
    goto unlock_ret;
    }
    usleep_range(10, 20);
    }
    if (tcphy.mode == MODE_DISCONNECT)
    tcphy_phy_deinit(tcphy);
    ret = -ETIMEDOUT;
    unlock_ret:
    mutex_unlock(&tcphy.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_usb3_phy_power_off(phy: *mut phy) -> c_int {
    static int rockchip_usb3_phy_power_off(struct phy *phy)
    {
    struct rockchip_typec_phy *tcphy = phy_get_drvdata(phy);
    mutex_lock(&tcphy.lock);
    tcphy_cfg_usb3_to_usb2_only(tcphy, false);
    if (tcphy.mode == MODE_DISCONNECT)
    goto unlock;
    tcphy.mode &= ~(MODE_UFP_USB | MODE_DFP_USB);
    if (tcphy.mode == MODE_DISCONNECT)
    tcphy_phy_deinit(tcphy);
    unlock:
    mutex_unlock(&tcphy.lock);
    return 0;
    }
    static const struct phy_ops rockchip_usb3_phy_ops = {
    .power_on	= rockchip_usb3_phy_power_on,
    .power_off	= rockchip_usb3_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn rockchip_dp_phy_power_on(phy: *mut phy) -> c_int {
    static int rockchip_dp_phy_power_on(struct phy *phy)
    {
    struct rockchip_typec_phy *tcphy = phy_get_drvdata(phy);
    const struct rockchip_usb3phy_port_cfg *cfg = tcphy.port_cfgs;
    int new_mode, ret = 0;
    u32 val;
    mutex_lock(&tcphy.lock);
    new_mode = tcphy_get_mode(tcphy);
    if (new_mode < 0) {
    ret = new_mode;
    goto unlock_ret;
    }
    if (!(new_mode & MODE_DFP_DP)) {
    ret = -ENODEV;
    goto unlock_ret;
    }
    if (tcphy.mode == new_mode)
    goto unlock_ret;
//
// If the PHY has been power on, but the mode is not DP only mode,
// re-init the PHY for setting all of 4 lanes to DP.
//
    if (new_mode == MODE_DFP_DP && tcphy.mode != MODE_DISCONNECT) {
    tcphy_phy_deinit(tcphy);
    ret = tcphy_phy_init(tcphy, new_mode);
    } else if (tcphy.mode == MODE_DISCONNECT) {
    ret = tcphy_phy_init(tcphy, new_mode);
    }
    if (ret)
    goto unlock_ret;
    property_enable(tcphy, &cfg.uphy_dp_sel, 1);
    ret = readx_poll_timeout(readl, tcphy.base + DP_MODE_CTL,
    val, val & DP_MODE_A2, 1000,
    PHY_MODE_SET_TIMEOUT);
    if (ret < 0) {
    dev_err(tcphy.dev, "failed to wait TCPHY enter A2\n");
    goto power_on_finish;
    }
    tcphy_dp_aux_calibration(tcphy);
    writel(DP_MODE_ENTER_A0, tcphy.base + DP_MODE_CTL);
    ret = readx_poll_timeout(readl, tcphy.base + DP_MODE_CTL,
    val, val & DP_MODE_A0, 1000,
    PHY_MODE_SET_TIMEOUT);
    if (ret < 0) {
    writel(DP_MODE_ENTER_A2, tcphy.base + DP_MODE_CTL);
    dev_err(tcphy.dev, "failed to wait TCPHY enter A0\n");
    goto power_on_finish;
    }
    tcphy.mode |= MODE_DFP_DP;
    power_on_finish:
    if (tcphy.mode == MODE_DISCONNECT)
    tcphy_phy_deinit(tcphy);
    unlock_ret:
    mutex_unlock(&tcphy.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_dp_phy_power_off(phy: *mut phy) -> c_int {
    static int rockchip_dp_phy_power_off(struct phy *phy)
    {
    struct rockchip_typec_phy *tcphy = phy_get_drvdata(phy);
    mutex_lock(&tcphy.lock);
    if (tcphy.mode == MODE_DISCONNECT)
    goto unlock;
    tcphy.mode &= ~MODE_DFP_DP;
    writel(DP_MODE_ENTER_A2, tcphy.base + DP_MODE_CTL);
    if (tcphy.mode == MODE_DISCONNECT)
    tcphy_phy_deinit(tcphy);
    unlock:
    mutex_unlock(&tcphy.lock);
    return 0;
    }
    static const struct phy_ops rockchip_dp_phy_ops = {
    .power_on	= rockchip_dp_phy_power_on,
    .power_off	= rockchip_dp_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static int tcphy_parse_dt(struct rockchip_typec_phy *tcphy,
    struct device *dev)
    {
    tcphy.grf_regs = syscon_regmap_lookup_by_phandle(dev.of_node,
    "rockchip,grf");
    if (IS_ERR(tcphy.grf_regs)) {
    dev_err(dev, "could not find grf dt node\n");
    return PTR_ERR(tcphy.grf_regs);
    }
    tcphy.clk_core = devm_clk_get(dev, "tcpdcore");
    if (IS_ERR(tcphy.clk_core)) {
    dev_err(dev, "could not get uphy core clock\n");
    return PTR_ERR(tcphy.clk_core);
    }
    tcphy.clk_ref = devm_clk_get(dev, "tcpdphy-ref");
    if (IS_ERR(tcphy.clk_ref)) {
    dev_err(dev, "could not get uphy ref clock\n");
    return PTR_ERR(tcphy.clk_ref);
    }
    tcphy.uphy_rst = devm_reset_control_get(dev, "uphy");
    if (IS_ERR(tcphy.uphy_rst)) {
    dev_err(dev, "no uphy_rst reset control found\n");
    return PTR_ERR(tcphy.uphy_rst);
    }
    tcphy.pipe_rst = devm_reset_control_get(dev, "uphy-pipe");
    if (IS_ERR(tcphy.pipe_rst)) {
    dev_err(dev, "no pipe_rst reset control found\n");
    return PTR_ERR(tcphy.pipe_rst);
    }
    tcphy.tcphy_rst = devm_reset_control_get(dev, "uphy-tcphy");
    if (IS_ERR(tcphy.tcphy_rst)) {
    dev_err(dev, "no tcphy_rst reset control found\n");
    return PTR_ERR(tcphy.tcphy_rst);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn typec_phy_pre_init(tcphy: *mut rockchip_typec_phy) {
    static void typec_phy_pre_init(struct rockchip_typec_phy *tcphy)
    {
    const struct rockchip_usb3phy_port_cfg *cfg = tcphy.port_cfgs;
    reset_control_assert(tcphy.tcphy_rst);
    reset_control_assert(tcphy.uphy_rst);
    reset_control_assert(tcphy.pipe_rst);
// select external psm clock
    property_enable(tcphy, &cfg.external_psm, 1);
    property_enable(tcphy, &cfg.usb3tousb2_en, 0);
    tcphy.mode = MODE_DISCONNECT;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_typec_phy_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_typec_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *child_np;
    struct rockchip_typec_phy *tcphy;
    struct phy_provider *phy_provider;
    struct resource *res;
    const struct rockchip_usb3phy_port_cfg *phy_cfgs;
    int index, ret;
    tcphy = devm_kzalloc(dev, sizeof(*tcphy), GFP_KERNEL);
    if (!tcphy)
    return -ENOMEM;
    phy_cfgs = of_device_get_match_data(dev);
    if (!phy_cfgs) {
    dev_err(dev, "phy configs are not assigned!\n");
    return -EINVAL;
    }
    tcphy.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(tcphy.base))
    return PTR_ERR(tcphy.base);
// find out a proper config which can be matched with dt.
    index = 0;
    while (phy_cfgs[index].reg) {
    if (phy_cfgs[index].reg == res.start) {
    tcphy.port_cfgs = &phy_cfgs[index];
    break;
    }
    ++index;
    }
    if (!tcphy.port_cfgs) {
    dev_err(dev, "no phy-config can be matched with %pOFn node\n",
    np);
    return -EINVAL;
    }
    ret = tcphy_parse_dt(tcphy, dev);
    if (ret)
    return ret;
    tcphy.dev = dev;
    platform_set_drvdata(pdev, tcphy);
    mutex_init(&tcphy.lock);
    typec_phy_pre_init(tcphy);
    tcphy.extcon = extcon_get_edev_by_phandle(dev, 0);
    if (IS_ERR(tcphy.extcon)) {
    if (PTR_ERR(tcphy.extcon) == -ENODEV) {
    tcphy.extcon = core::ptr::null_mut();
    } else {
    if (PTR_ERR(tcphy.extcon) != -EPROBE_DEFER)
    dev_err(dev, "Invalid or missing extcon\n");
    return PTR_ERR(tcphy.extcon);
    }
    }
    pm_runtime_enable(dev);
    for_each_available_child_of_node(np, child_np) {
    struct phy *phy;
    if (of_node_name_eq(child_np, "dp-port"))
    phy = devm_phy_create(dev, child_np,
    &rockchip_dp_phy_ops);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_node_name_eq(child_np, _arg: "usb3-port")) -> else {
    else if (of_node_name_eq(child_np, "usb3-port"))
    phy = devm_phy_create(dev, child_np,
    &rockchip_usb3_phy_ops);
    else
    continue;
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy: %pOFn\n",
    child_np);
    pm_runtime_disable(dev);
    of_node_put(child_np);
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, tcphy);
    }
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    dev_err(dev, "Failed to register phy provider\n");
    pm_runtime_disable(dev);
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_typec_phy_remove(pdev: *mut platform_device) {
    static void rockchip_typec_phy_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id rockchip_typec_phy_dt_ids[] = {
    {
    .compatible = "rockchip,rk3399-typec-phy",
    .data = &rk3399_usb3phy_port_cfgs
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rockchip_typec_phy_dt_ids);
    static struct platform_driver rockchip_typec_phy_driver = {
    .probe		= rockchip_typec_phy_probe,
    .remove		= rockchip_typec_phy_remove,
    .driver		= {
    .name	= "rockchip-typec-phy",
    .of_match_table = rockchip_typec_phy_dt_ids,
    },
    };
    module_platform_driver(rockchip_typec_phy_driver);
    MODULE_AUTHOR("Chris Zhong <zyw@rock-chips.com>");
    MODULE_AUTHOR("Kever Yang <kever.yang@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip USB TYPE-C PHY driver");
    MODULE_LICENSE("GPL v2");
