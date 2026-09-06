//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/tc358775.c
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
// TC358775 DSI to LVDS bridge driver
//
// Copyright (C) 2020 SMART Wireless Computing
// Author: Vinay Simha BN <simhavcs@gmail.com>
//
// #define DEBUG

// Registers
// DSI D-PHY Layer Registers
pub const D0W_DPHYCONTTX: c_uint = 0x0004  /* Data Lane 0 DPHY Tx Control */;
pub const CLW_DPHYCONTRX: c_uint = 0x0020  /* Clock Lane DPHY Rx Control */;
pub const D0W_DPHYCONTRX: c_uint = 0x0024  /* Data Lane 0 DPHY Rx Control */;
pub const D1W_DPHYCONTRX: c_uint = 0x0028  /* Data Lane 1 DPHY Rx Control */;
pub const D2W_DPHYCONTRX: c_uint = 0x002C  /* Data Lane 2 DPHY Rx Control */;
pub const D3W_DPHYCONTRX: c_uint = 0x0030  /* Data Lane 3 DPHY Rx Control */;
pub const COM_DPHYCONTRX: c_uint = 0x0038  /* DPHY Rx Common Control */;
pub const CLW_CNTRL: c_uint = 0x0040  /* Clock Lane Control */;
pub const D0W_CNTRL: c_uint = 0x0044  /* Data Lane 0 Control */;
pub const D1W_CNTRL: c_uint = 0x0048  /* Data Lane 1 Control */;
pub const D2W_CNTRL: c_uint = 0x004C  /* Data Lane 2 Control */;
pub const D3W_CNTRL: c_uint = 0x0050  /* Data Lane 3 Control */;
pub const DFTMODE_CNTRL: c_uint = 0x0054  /* DFT Mode Control */;
// DSI PPI Layer Registers
pub const PPI_STARTPPI: c_uint = 0x0104  /* START control bit of PPI-TX function. */;
pub const PPI_START_FUNCTION: c_int = 1;
pub const PPI_BUSYPPI: c_uint = 0x0108;
pub const PPI_LINEINITCNT: c_uint = 0x0110  /* Line Initialization Wait Counter  */;
pub const PPI_LPTXTIMECNT: c_uint = 0x0114;
pub const PPI_LANEENABLE: c_uint = 0x0134  /* Enables each lane at the PPI layer. */;
pub const PPI_TX_RX_TA: c_uint = 0x013C  /* DSI Bus Turn Around timing parameters */;
// Analog timer function enable
pub const PPI_CLS_ATMR: c_uint = 0x0140  /* Delay for Clock Lane in LPRX  */;
pub const PPI_D0S_ATMR: c_uint = 0x0144  /* Delay for Data Lane 0 in LPRX */;
pub const PPI_D1S_ATMR: c_uint = 0x0148  /* Delay for Data Lane 1 in LPRX */;
pub const PPI_D2S_ATMR: c_uint = 0x014C  /* Delay for Data Lane 2 in LPRX */;
pub const PPI_D3S_ATMR: c_uint = 0x0150  /* Delay for Data Lane 3 in LPRX */;
pub const PPI_D0S_CLRSIPOCOUNT: c_uint = 0x0164  /* For lane 0 */;
pub const PPI_D1S_CLRSIPOCOUNT: c_uint = 0x0168  /* For lane 1 */;
pub const PPI_D2S_CLRSIPOCOUNT: c_uint = 0x016C  /* For lane 2 */;
pub const PPI_D3S_CLRSIPOCOUNT: c_uint = 0x0170  /* For lane 3 */;
pub const CLS_PRE: c_uint = 0x0180  /* Digital Counter inside of PHY IO */;
pub const D0S_PRE: c_uint = 0x0184  /* Digital Counter inside of PHY IO */;
pub const D1S_PRE: c_uint = 0x0188  /* Digital Counter inside of PHY IO */;
pub const D2S_PRE: c_uint = 0x018C  /* Digital Counter inside of PHY IO */;
pub const D3S_PRE: c_uint = 0x0190  /* Digital Counter inside of PHY IO */;
pub const CLS_PREP: c_uint = 0x01A0  /* Digital Counter inside of PHY IO */;
pub const D0S_PREP: c_uint = 0x01A4  /* Digital Counter inside of PHY IO */;
pub const D1S_PREP: c_uint = 0x01A8  /* Digital Counter inside of PHY IO */;
pub const D2S_PREP: c_uint = 0x01AC  /* Digital Counter inside of PHY IO */;
pub const D3S_PREP: c_uint = 0x01B0  /* Digital Counter inside of PHY IO */;
pub const CLS_ZERO: c_uint = 0x01C0  /* Digital Counter inside of PHY IO */;
pub const D0S_ZERO: c_uint = 0x01C4  /* Digital Counter inside of PHY IO */;
pub const D1S_ZERO: c_uint = 0x01C8  /* Digital Counter inside of PHY IO */;
pub const D2S_ZERO: c_uint = 0x01CC  /* Digital Counter inside of PHY IO */;
pub const D3S_ZERO: c_uint = 0x01D0  /* Digital Counter inside of PHY IO */;
pub const PPI_CLRFLG: c_uint = 0x01E0  /* PRE Counters has reached set values */;
pub const PPI_CLRSIPO: c_uint = 0x01E4  /* Clear SIPO values, Slave mode use only. */;
pub const HSTIMEOUT: c_uint = 0x01F0  /* HS Rx Time Out Counter */;
pub const HSTIMEOUTENABLE: c_uint = 0x01F4  /* Enable HS Rx Time Out Counter */;
pub const DSI_STARTDSI: c_uint = 0x0204  /* START control bit of DSI-TX function */;
pub const DSI_RX_START: c_int = 1;
pub const DSI_BUSYDSI: c_uint = 0x0208;
pub const DSI_LANEENABLE: c_uint = 0x0210  /* Enables each lane at the Protocol layer. */;
pub const DSI_LANESTATUS0: c_uint = 0x0214  /* Displays lane is in HS RX mode. */;
pub const DSI_LANESTATUS1: c_uint = 0x0218  /* Displays lane is in ULPS or STOP state */;
pub const DSI_INTSTATUS: c_uint = 0x0220  /* Interrupt Status */;
pub const DSI_INTMASK: c_uint = 0x0224  /* Interrupt Mask */;
pub const DSI_INTCLR: c_uint = 0x0228  /* Interrupt Clear */;
pub const DSI_LPTXTO: c_uint = 0x0230  /* Low Power Tx Time Out Counter */;
pub const DSIERRCNT: c_uint = 0x0300  /* DSI Error Count */;
pub const APLCTRL: c_uint = 0x0400  /* Application Layer Control */;
pub const RDPKTLN: c_uint = 0x0404  /* Command Read Packet Length */;
pub const VPCTRL: c_uint = 0x0450  /* Video Path Control */;

pub const HTIM1: c_uint = 0x0454  /* Horizontal Timing Control 1 */;
pub const HTIM2: c_uint = 0x0458  /* Horizontal Timing Control 2 */;
pub const VTIM1: c_uint = 0x045C  /* Vertical Timing Control 1 */;
pub const VTIM2: c_uint = 0x0460  /* Vertical Timing Control 2 */;
pub const VFUEN: c_uint = 0x0464  /* Video Frame Timing Update Enable */;

// Mux Input Select for LVDS LINK Input
pub const LV_MX0003: c_uint = 0x0480  /* Bit 0 to 3 */;
pub const LV_MX0407: c_uint = 0x0484  /* Bit 4 to 7 */;
pub const LV_MX0811: c_uint = 0x0488  /* Bit 8 to 11 */;
pub const LV_MX1215: c_uint = 0x048C  /* Bit 12 to 15 */;
pub const LV_MX1619: c_uint = 0x0490  /* Bit 16 to 19 */;
pub const LV_MX2023: c_uint = 0x0494  /* Bit 20 to 23 */;
pub const LV_MX2427: c_uint = 0x0498  /* Bit 24 to 27 */;

    FLD_VAL(b2, 20, 16) | FLD_VAL(b3, 28, 24))
// Input bit numbers used in mux registers
    enum {
    LVI_R0,
    LVI_R1,
    LVI_R2,
    LVI_R3,
    LVI_R4,
    LVI_R5,
    LVI_R6,
    LVI_R7,
    LVI_G0,
    LVI_G1,
    LVI_G2,
    LVI_G3,
    LVI_G4,
    LVI_G5,
    LVI_G6,
    LVI_G7,
    LVI_B0,
    LVI_B1,
    LVI_B2,
    LVI_B3,
    LVI_B4,
    LVI_B5,
    LVI_B6,
    LVI_B7,
    LVI_HS,
    LVI_VS,
    LVI_DE,
    LVI_L0
    };
pub const LVCFG: c_uint = 0x049C  /* LVDS Configuration  */;
pub const LVPHY0: c_uint = 0x04A0  /* LVDS PHY 0 */;

pub const LVPHY1: c_uint = 0x04A4  /* LVDS PHY 1 */;
pub const SYSSTAT: c_uint = 0x0500  /* System Status  */;
pub const SYSRST: c_uint = 0x0504  /* System Reset  */;

// GPIO Registers
pub const GPIOC: c_uint = 0x0520  /* GPIO Control  */;
pub const GPIOO: c_uint = 0x0524  /* GPIO Output  */;
pub const GPIOI: c_uint = 0x0528  /* GPIO Input  */;
// I2C Registers
pub const I2CTIMCTRL: c_uint = 0x0540  /* I2C IF Timing and Enable Control */;
pub const I2CMADDR: c_uint = 0x0544  /* I2C Master Addressing */;
pub const WDATAQ: c_uint = 0x0548  /* Write Data Queue */;
pub const RDATAQ: c_uint = 0x054C  /* Read Data Queue */;
// Chip ID and Revision ID Register
pub const IDREG: c_uint = 0x0580;
pub const LPX_PERIOD: c_int = 4;
pub const TTA_GET: c_uint = 0x40000;
pub const TTA_SURE: c_int = 6;
pub const SINGLE_LINK: c_int = 1;
pub const DUAL_LINK: c_int = 2;
pub const TC358775XBG_ID: c_uint = 0x00007500;
// Debug Registers
pub const DEBUG00: c_uint = 0x05A0  /* Debug */;
pub const DEBUG01: c_uint = 0x05A4  /* LVDS Data */;

pub const TC358775_VPCTRL_VSDELAY__MASK: c_uint = 0x3FF00000;
pub const TC358775_VPCTRL_VSDELAY__SHIFT: c_int = 20;
#[no_mangle]
pub unsafe extern "C" fn TC358775_VPCTRL_VSDELAY(val: u32) -> u32 {
    static inline u32 TC358775_VPCTRL_VSDELAY(uint32_t val)
    {
    return ((val) << TC358775_VPCTRL_VSDELAY__SHIFT) &
    TC358775_VPCTRL_VSDELAY__MASK;
    }
pub const TC358775_VPCTRL_OPXLFMT__MASK: c_uint = 0x00000100;
pub const TC358775_VPCTRL_OPXLFMT__SHIFT: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn TC358775_VPCTRL_OPXLFMT(val: u32) -> u32 {
    static inline u32 TC358775_VPCTRL_OPXLFMT(uint32_t val)
    {
    return ((val) << TC358775_VPCTRL_OPXLFMT__SHIFT) &
    TC358775_VPCTRL_OPXLFMT__MASK;
    }
pub const TC358775_VPCTRL_MSF__MASK: c_uint = 0x00000001;
pub const TC358775_VPCTRL_MSF__SHIFT: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn TC358775_VPCTRL_MSF(val: u32) -> u32 {
    static inline u32 TC358775_VPCTRL_MSF(uint32_t val)
    {
    return ((val) << TC358775_VPCTRL_MSF__SHIFT) &
    TC358775_VPCTRL_MSF__MASK;
    }
pub const TC358775_LVCFG_PCLKDIV__MASK: c_uint = 0x000000f0;
pub const TC358775_LVCFG_PCLKDIV__SHIFT: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn TC358775_LVCFG_PCLKDIV(val: u32) -> u32 {
    static inline u32 TC358775_LVCFG_PCLKDIV(uint32_t val)
    {
    return ((val) << TC358775_LVCFG_PCLKDIV__SHIFT) &
    TC358775_LVCFG_PCLKDIV__MASK;
    }
pub const TC358775_LVCFG_LVDLINK__MASK: c_uint = 0x00000002;
pub const TC358775_LVCFG_LVDLINK__SHIFT: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn TC358775_LVCFG_LVDLINK(val: u32) -> u32 {
    static inline u32 TC358775_LVCFG_LVDLINK(uint32_t val)
    {
    return ((val) << TC358775_LVCFG_LVDLINK__SHIFT) &
    TC358775_LVCFG_LVDLINK__MASK;
    }
    enum tc358775_ports {
    TC358775_DSI_IN,
    TC358775_LVDS_OUT0,
    TC358775_LVDS_OUT1,
    };
    enum tc3587x5_type {
    TC358765 = 0x65,
    TC358775 = 0x75,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_data {
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub bridge: drm_bridge,
    pub panel_bridge: *mut drm_bridge,
    pub host_node: *mut device_node,
    pub dsi: *mut mipi_dsi_device,
    pub num_dsi_lanes: u8,
    pub vdd: *mut regulator,
    pub vddio: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub stby_gpio: *mut gpio_desc,
    pub /: *mut *mut u8 lvds_link; / single-link or dual-link,
    pub bpc: u8,
    pub type: enum tc3587x5_type,
}

    static inline struct tc_data *bridge_to_tc(struct drm_bridge *b)
    {
    return container_of(b, struct tc_data, bridge);
    }
    static void tc_bridge_atomic_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc_data *tc = bridge_to_tc(bridge);
    struct device *dev = &tc.dsi.dev;
    int ret;
    ret = regulator_enable(tc.vddio);
    if (ret < 0)
    dev_err(dev, "regulator vddio enable failed, %d\n", ret);
    usleep_range(10000, 11000);
    ret = regulator_enable(tc.vdd);
    if (ret < 0)
    dev_err(dev, "regulator vdd enable failed, %d\n", ret);
    usleep_range(10000, 11000);
    gpiod_set_value(tc.stby_gpio, 0);
    usleep_range(10000, 11000);
    gpiod_set_value(tc.reset_gpio, 0);
    usleep_range(10, 20);
    }
    static void tc_bridge_atomic_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc_data *tc = bridge_to_tc(bridge);
    struct device *dev = &tc.dsi.dev;
    int ret;
    gpiod_set_value(tc.reset_gpio, 1);
    usleep_range(10, 20);
    gpiod_set_value(tc.stby_gpio, 1);
    usleep_range(10000, 11000);
    ret = regulator_disable(tc.vdd);
    if (ret < 0)
    dev_err(dev, "regulator vdd disable failed, %d\n", ret);
    usleep_range(10000, 11000);
    ret = regulator_disable(tc.vddio);
    if (ret < 0)
    dev_err(dev, "regulator vddio disable failed, %d\n", ret);
    usleep_range(10000, 11000);
    }
#[no_mangle]
unsafe extern "C" fn d2l_read(i2c: *mut i2c_client, addr: u16, val: *mut u32) {
    static void d2l_read(struct i2c_client *i2c, u16 addr, u32 *val)
    {
    int ret;
    u8 buf_addr[2];
    put_unaligned_be16(addr, buf_addr);
    ret = i2c_master_send(i2c, buf_addr, sizeof(buf_addr));
    if (ret < 0)
    goto fail;
    ret = i2c_master_recv(i2c, (u8 *)val, sizeof(*val));
    if (ret < 0)
    goto fail;
    pr_debug("d2l: I2C : addr:%04x value:%08x\n", addr, *val);
    return;
    fail:
    dev_err(&i2c.dev, "Error %d reading from subaddress 0x%x\n",
    ret, addr);
    }
#[no_mangle]
unsafe extern "C" fn d2l_write(i2c: *mut i2c_client, addr: u16, val: u32) {
    static void d2l_write(struct i2c_client *i2c, u16 addr, u32 val)
    {
    u8 data[6];
    int ret;
    put_unaligned_be16(addr, data);
    put_unaligned_le32(val, data + 2);
    ret = i2c_master_send(i2c, data, ARRAY_SIZE(data));
    if (ret < 0)
    dev_err(&i2c.dev, "Error %d writing to subaddress 0x%x\n",
    ret, addr);
    }
    static void tc_bridge_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc_data *tc = bridge_to_tc(bridge);
    u32 hback_porch, hsync_len, hfront_porch, hactive, htime1, htime2;
    u32 vback_porch, vsync_len, vfront_porch, vactive, vtime1, vtime2;
    let mut val: u32 = 0;
    u16 dsiclk, clkdiv, byteclk, t1, t2, t3, vsdelay;
    struct drm_connector *connector =
    drm_atomic_get_new_connector_for_encoder(state, bridge.encoder);
    struct drm_connector_state *conn_state =
    drm_atomic_get_new_connector_state(state, connector);
    struct drm_crtc_state *crtc_state =
    drm_atomic_get_new_crtc_state(state, conn_state.crtc);
    struct drm_display_mode *mode = &crtc_state.adjusted_mode;
    hback_porch = mode.htotal - mode.hsync_end;
    hsync_len  = mode.hsync_end - mode.hsync_start;
    vback_porch = mode.vtotal - mode.vsync_end;
    vsync_len  = mode.vsync_end - mode.vsync_start;
    htime1 = (hback_porch << 16) + hsync_len;
    vtime1 = (vback_porch << 16) + vsync_len;
    hfront_porch = mode.hsync_start - mode.hdisplay;
    hactive = mode.hdisplay;
    vfront_porch = mode.vsync_start - mode.vdisplay;
    vactive = mode.vdisplay;
    htime2 = (hfront_porch << 16) + hactive;
    vtime2 = (vfront_porch << 16) + vactive;
    d2l_read(tc.i2c, IDREG, &val);
    dev_info(tc.dev, "DSI2LVDS Chip ID.%02x Revision ID. %02x **\n",
    (val >> 8) & 0xFF, val & 0xFF);
    d2l_write(tc.i2c, SYSRST, SYS_RST_REG | SYS_RST_DSIRX | SYS_RST_BM |
    SYS_RST_LCD | SYS_RST_I2CM);
    usleep_range(30000, 40000);
    d2l_write(tc.i2c, PPI_TX_RX_TA, TTA_GET | TTA_SURE);
    d2l_write(tc.i2c, PPI_LPTXTIMECNT, LPX_PERIOD);
    d2l_write(tc.i2c, PPI_D0S_CLRSIPOCOUNT, 3);
    d2l_write(tc.i2c, PPI_D1S_CLRSIPOCOUNT, 3);
    d2l_write(tc.i2c, PPI_D2S_CLRSIPOCOUNT, 3);
    d2l_write(tc.i2c, PPI_D3S_CLRSIPOCOUNT, 3);
    val = ((L0EN << tc.num_dsi_lanes) - L0EN) | DSI_CLEN_BIT;
    d2l_write(tc.i2c, PPI_LANEENABLE, val);
    d2l_write(tc.i2c, DSI_LANEENABLE, val);
    d2l_write(tc.i2c, PPI_STARTPPI, PPI_START_FUNCTION);
    d2l_write(tc.i2c, DSI_STARTDSI, DSI_RX_START);
// Video event mode vs pulse mode bit, does not exist for tc358775
    if (tc.type == TC358765)
    val = EVTMODE;
    else
    val = 0;
    if (tc.bpc == 8)
    val |= TC358775_VPCTRL_OPXLFMT(1);
    else /* bpc = 6; */
    val |= TC358775_VPCTRL_MSF(1);
    dsiclk = mode.crtc_clock * 3 * tc.bpc / tc.num_dsi_lanes / 1000;
    clkdiv = dsiclk / (tc.lvds_link == DUAL_LINK ? DIVIDE_BY_6 : DIVIDE_BY_3);
    byteclk = dsiclk / 4;
    t1 = hactive * (tc.bpc * 3 / 8) / tc.num_dsi_lanes;
    t2 = ((100000 / clkdiv)) * (hactive + hback_porch + hsync_len + hfront_porch) / 1000;
    t3 = ((t2 * byteclk) / 100) - (hactive * (tc.bpc * 3 / 8) /
    tc.num_dsi_lanes);
    vsdelay = (clkdiv * (t1 + t3) / byteclk) - hback_porch - hsync_len - hactive;
    val |= TC358775_VPCTRL_VSDELAY(vsdelay);
    d2l_write(tc.i2c, VPCTRL, val);
    d2l_write(tc.i2c, HTIM1, htime1);
    d2l_write(tc.i2c, VTIM1, vtime1);
    d2l_write(tc.i2c, HTIM2, htime2);
    d2l_write(tc.i2c, VTIM2, vtime2);
    d2l_write(tc.i2c, VFUEN, VFUEN_EN);
    d2l_write(tc.i2c, SYSRST, SYS_RST_LCD);
    d2l_write(tc.i2c, LVPHY0, LV_PHY0_PRBS_ON(4) | LV_PHY0_ND(6));
    dev_dbg(tc.dev, "bus_formats %04x bpc %d\n",
    connector.display_info.bus_formats[0],
    tc.bpc);
    if (connector.display_info.bus_formats[0] ==
    MEDIA_BUS_FMT_RGB888_1X7X4_SPWG) {
// VESA-24
    d2l_write(tc.i2c, LV_MX0003, LV_MX(LVI_R0, LVI_R1, LVI_R2, LVI_R3));
    d2l_write(tc.i2c, LV_MX0407, LV_MX(LVI_R4, LVI_R7, LVI_R5, LVI_G0));
    d2l_write(tc.i2c, LV_MX0811, LV_MX(LVI_G1, LVI_G2, LVI_G6, LVI_G7));
    d2l_write(tc.i2c, LV_MX1215, LV_MX(LVI_G3, LVI_G4, LVI_G5, LVI_B0));
    d2l_write(tc.i2c, LV_MX1619, LV_MX(LVI_B6, LVI_B7, LVI_B1, LVI_B2));
    d2l_write(tc.i2c, LV_MX2023, LV_MX(LVI_B3, LVI_B4, LVI_B5, LVI_L0));
    d2l_write(tc.i2c, LV_MX2427, LV_MX(LVI_HS, LVI_VS, LVI_DE, LVI_R6));
    } else {
// JEIDA-18 and JEIDA-24
    d2l_write(tc.i2c, LV_MX0003, LV_MX(LVI_R2, LVI_R3, LVI_R4, LVI_R5));
    d2l_write(tc.i2c, LV_MX0407, LV_MX(LVI_R6, LVI_R1, LVI_R7, LVI_G2));
    d2l_write(tc.i2c, LV_MX0811, LV_MX(LVI_G3, LVI_G4, LVI_G0, LVI_G1));
    d2l_write(tc.i2c, LV_MX1215, LV_MX(LVI_G5, LVI_G6, LVI_G7, LVI_B2));
    d2l_write(tc.i2c, LV_MX1619, LV_MX(LVI_B0, LVI_B1, LVI_B3, LVI_B4));
    d2l_write(tc.i2c, LV_MX2023, LV_MX(LVI_B5, LVI_B6, LVI_B7, LVI_L0));
    d2l_write(tc.i2c, LV_MX2427, LV_MX(LVI_HS, LVI_VS, LVI_DE, LVI_R0));
    }
    d2l_write(tc.i2c, VFUEN, VFUEN_EN);
    val = LVCFG_LVEN_BIT;
    if (tc.lvds_link == DUAL_LINK) {
    val |= TC358775_LVCFG_LVDLINK(1);
    val |= TC358775_LVCFG_PCLKDIV(DIVIDE_BY_6);
    } else {
    val |= TC358775_LVCFG_PCLKDIV(DIVIDE_BY_3);
    }
    d2l_write(tc.i2c, LVCFG, val);
    }
    static enum drm_mode_status
    tc_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    struct tc_data *tc = bridge_to_tc(bridge);
//
// Maximum pixel clock speed 135MHz for single-link
// 270MHz for dual-link
//
    if ((mode.clock > 135000 && tc.lvds_link == SINGLE_LINK) ||
    (mode.clock > 270000 && tc.lvds_link == DUAL_LINK))
    return MODE_CLOCK_HIGH;
    switch (info.bus_formats[0]) {
    case MEDIA_BUS_FMT_RGB888_1X7X4_SPWG:
    case MEDIA_BUS_FMT_RGB888_1X7X4_JEIDA:
// RGB888
    tc.bpc = 8;
    break;
    case MEDIA_BUS_FMT_RGB666_1X7X3_SPWG:
// RGB666
    tc.bpc = 6;
    break;
    default:
    dev_warn(tc.dev,
    "unsupported LVDS bus format 0x%04x\n",
    info.bus_formats[0]);
    return MODE_NOMODE;
    }
    return MODE_OK;
    }
#[no_mangle]
unsafe extern "C" fn tc358775_parse_dt(np: *mut device_node, tc: *mut tc_data) -> c_int {
    static int tc358775_parse_dt(struct device_node *np, struct tc_data *tc)
    {
    struct device_node *endpoint;
    struct device_node *remote;
    let mut dsi_lanes: c_int = -1;
    endpoint = of_graph_get_endpoint_by_regs(tc.dev.of_node,
    TC358775_DSI_IN, -1);
    dsi_lanes = drm_of_get_data_lanes_count(endpoint, 1, 4);
// Quirk old dtb: Use data lanes from the DSI host side instead of bridge
    if (dsi_lanes == -EINVAL || dsi_lanes == -ENODEV) {
    remote = of_graph_get_remote_endpoint(endpoint);
    dsi_lanes = drm_of_get_data_lanes_count(remote, 1, 4);
    of_node_put(remote);
    if (dsi_lanes >= 1)
    dev_warn(tc.dev, "no dsi-lanes for the bridge, using host lanes\n");
    }
    of_node_put(endpoint);
    if (dsi_lanes < 0)
    return dsi_lanes;
    tc.num_dsi_lanes = dsi_lanes;
    tc.host_node = of_graph_get_remote_node(np, 0, 0);
    if (!tc.host_node)
    return -ENODEV;
    of_node_put(tc.host_node);
    tc.lvds_link = SINGLE_LINK;
    endpoint = of_graph_get_endpoint_by_regs(tc.dev.of_node,
    TC358775_LVDS_OUT1, -1);
    if (endpoint) {
    remote = of_graph_get_remote_port_parent(endpoint);
    of_node_put(endpoint);
    if (remote) {
    if (of_device_is_available(remote))
    tc.lvds_link = DUAL_LINK;
    of_node_put(remote);
    }
    }
    dev_dbg(tc.dev, "no.of dsi lanes: %d\n", tc.num_dsi_lanes);
    dev_dbg(tc.dev, "operating in %d-link mode\n",	tc.lvds_link);
    return 0;
    }
    static int tc_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct tc_data *tc = bridge_to_tc(bridge);
// Attach the panel-bridge to the dsi bridge
    return drm_bridge_attach(encoder, tc.panel_bridge,
    &tc.bridge, flags);
    }
    static const struct drm_bridge_funcs tc_bridge_funcs = {
    .attach = tc_bridge_attach,
    .atomic_pre_enable = tc_bridge_atomic_pre_enable,
    .atomic_enable = tc_bridge_atomic_enable,
    .mode_valid = tc_mode_valid,
    .atomic_post_disable = tc_bridge_atomic_post_disable,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    };
#[no_mangle]
unsafe extern "C" fn tc_attach_host(tc: *mut tc_data) -> c_int {
    static int tc_attach_host(struct tc_data *tc)
    {
    struct device *dev = &tc.i2c.dev;
    struct mipi_dsi_host *host;
    struct mipi_dsi_device *dsi;
    int ret;
    const struct mipi_dsi_device_info info = { .type = "tc358775",
    .channel = 0,
    .node = core::ptr::null_mut(),
    };
    host = of_find_mipi_dsi_host_by_node(tc.host_node);
    if (!host)
    return dev_err_probe(dev, -EPROBE_DEFER, "failed to find dsi host\n");
    dsi = devm_mipi_dsi_device_register_full(dev, host, &info);
    if (IS_ERR(dsi)) {
    dev_err(dev, "failed to create dsi device\n");
    return PTR_ERR(dsi);
    }
    tc.dsi = dsi;
    dsi.lanes = tc.num_dsi_lanes;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM;
//
// The hs_rate and lp_rate are data rate values. The HS mode is
// differential, while the LP mode is single ended. As the HS mode
// uses DDR, the DSI clock frequency is half the hs_rate. The 10 Mbs
// data rate for LP mode is not specified in the bridge data sheet,
// but seems to be part of the MIPI DSI spec.
//
    if (tc.type == TC358765)
    dsi.hs_rate = 800000000;
    else
    dsi.hs_rate = 1000000000;
    dsi.lp_rate = 10000000;
    ret = devm_mipi_dsi_attach(dev, dsi);
    if (ret < 0) {
    dev_err(dev, "failed to attach dsi to host\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc_probe(client: *mut i2c_client) -> c_int {
    static int tc_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct tc_data *tc;
    int ret;
    tc = devm_drm_bridge_alloc(dev, struct tc_data, bridge,
    &tc_bridge_funcs);
    if (IS_ERR(tc))
    return PTR_ERR(tc);
    tc.dev = dev;
    tc.i2c = client;
    tc.type = (enum tc3587x5_type)(unsigned long)of_device_get_match_data(dev);
    tc.panel_bridge = devm_drm_of_get_bridge(dev, dev.of_node,
    TC358775_LVDS_OUT0, 0);
    if (IS_ERR(tc.panel_bridge))
    return PTR_ERR(tc.panel_bridge);
    ret = tc358775_parse_dt(dev.of_node, tc);
    if (ret)
    return ret;
    tc.vddio = devm_regulator_get(dev, "vddio-supply");
    if (IS_ERR(tc.vddio)) {
    ret = PTR_ERR(tc.vddio);
    dev_err(dev, "vddio-supply not found\n");
    return ret;
    }
    tc.vdd = devm_regulator_get(dev, "vdd-supply");
    if (IS_ERR(tc.vdd)) {
    ret = PTR_ERR(tc.vdd);
    dev_err(dev, "vdd-supply not found\n");
    return ret;
    }
    tc.stby_gpio = devm_gpiod_get_optional(dev, "stby", GPIOD_OUT_HIGH);
    if (IS_ERR(tc.stby_gpio))
    return PTR_ERR(tc.stby_gpio);
    tc.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(tc.reset_gpio)) {
    ret = PTR_ERR(tc.reset_gpio);
    dev_err(dev, "cannot get reset-gpios %d\n", ret);
    return ret;
    }
    tc.bridge.of_node = dev.of_node;
    tc.bridge.pre_enable_prev_first = true;
    drm_bridge_add(&tc.bridge);
    i2c_set_clientdata(client, tc);
    ret = tc_attach_host(tc);
    if (ret)
    goto err_bridge_remove;
    return 0;
    err_bridge_remove:
    drm_bridge_remove(&tc.bridge);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc_remove(client: *mut i2c_client) {
    static void tc_remove(struct i2c_client *client)
    {
    struct tc_data *tc = i2c_get_clientdata(client);
    drm_bridge_remove(&tc.bridge);
    }
    static const struct i2c_device_id tc358775_i2c_ids[] = {
    { .name = "tc358765", .driver_data = TC358765 },
    { .name = "tc358775", .driver_data = TC358775 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tc358775_i2c_ids);
    static const struct of_device_id tc358775_of_ids[] = {
    { .compatible = "toshiba,tc358765", .data = (void *)TC358765, },
    { .compatible = "toshiba,tc358775", .data = (void *)TC358775, },
    { }
    };
    MODULE_DEVICE_TABLE(of, tc358775_of_ids);
    static struct i2c_driver tc358775_driver = {
    .driver = {
    .name = "tc358775",
    .of_match_table = tc358775_of_ids,
    },
    .id_table = tc358775_i2c_ids,
    .probe = tc_probe,
    .remove	= tc_remove,
    };
    module_i2c_driver(tc358775_driver);
    MODULE_AUTHOR("Vinay Simha BN <simhavcs@gmail.com>");
    MODULE_DESCRIPTION("TC358775 DSI/LVDS bridge driver");
    MODULE_LICENSE("GPL v2");
