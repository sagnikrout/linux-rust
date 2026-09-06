//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/sii9234.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2017 Samsung Electronics
//
// Authors:
// Tomasz Stanislawski <t.stanislaws@samsung.com>
// Maciej Purski <m.purski@samsung.com>
//
// Based on sii9234 driver created by:
// Adam Hampson <ahampson@sta.samsung.com>
// Erik Gilling <konkers@android.com>
// Shankar Bandal <shankar.b@samsung.com>
// Dharam Kumar <dharam.kr@samsung.com>
//

pub const CBUS_DEVCAP_OFFSET: c_uint = 0x80;
pub const SII9234_MHL_VERSION: c_uint = 0x11;
pub const SII9234_SCRATCHPAD_SIZE: c_uint = 0x10;
pub const SII9234_INT_STAT_SIZE: c_uint = 0x33;

pub const MHL_INIT_TIMEOUT: c_uint = 0x0C;
// MHL Tx registers and bits
pub const MHL_TX_SRST: c_uint = 0x05;
pub const MHL_TX_SYSSTAT_REG: c_uint = 0x09;
pub const MHL_TX_INTR1_REG: c_uint = 0x71;
pub const MHL_TX_INTR4_REG: c_uint = 0x74;
pub const MHL_TX_INTR1_ENABLE_REG: c_uint = 0x75;
pub const MHL_TX_INTR4_ENABLE_REG: c_uint = 0x78;
pub const MHL_TX_INT_CTRL_REG: c_uint = 0x79;
pub const MHL_TX_TMDS_CCTRL: c_uint = 0x80;
pub const MHL_TX_DISC_CTRL1_REG: c_uint = 0x90;
pub const MHL_TX_DISC_CTRL2_REG: c_uint = 0x91;
pub const MHL_TX_DISC_CTRL3_REG: c_uint = 0x92;
pub const MHL_TX_DISC_CTRL4_REG: c_uint = 0x93;
pub const MHL_TX_DISC_CTRL5_REG: c_uint = 0x94;
pub const MHL_TX_DISC_CTRL6_REG: c_uint = 0x95;
pub const MHL_TX_DISC_CTRL7_REG: c_uint = 0x96;
pub const MHL_TX_DISC_CTRL8_REG: c_uint = 0x97;
pub const MHL_TX_STAT2_REG: c_uint = 0x99;
pub const MHL_TX_MHLTX_CTL1_REG: c_uint = 0xA0;
pub const MHL_TX_MHLTX_CTL2_REG: c_uint = 0xA1;
pub const MHL_TX_MHLTX_CTL4_REG: c_uint = 0xA3;
pub const MHL_TX_MHLTX_CTL6_REG: c_uint = 0xA5;
pub const MHL_TX_MHLTX_CTL7_REG: c_uint = 0xA6;

pub const ATT_THRESH_SHIFT: c_uint = 0x04;

pub const DEGLITCH_TIME_MASK: c_uint = 0x07;
pub const DEGLITCH_TIME_2MS: c_int = 0;
pub const DEGLITCH_TIME_4MS: c_int = 1;
pub const DEGLITCH_TIME_8MS: c_int = 2;
pub const DEGLITCH_TIME_16MS: c_int = 3;
pub const DEGLITCH_TIME_40MS: c_int = 4;
pub const DEGLITCH_TIME_50MS: c_int = 5;
pub const DEGLITCH_TIME_60MS: c_int = 6;
pub const DEGLITCH_TIME_128MS: c_int = 7;

pub const RGND_INTP_MASK: c_uint = 0x03;
pub const RGND_INTP_OPEN: c_int = 0;
pub const RGND_INTP_2K: c_int = 1;
pub const RGND_INTP_1K: c_int = 2;
pub const RGND_INTP_SHORT: c_int = 3;
// HDMI registers
pub const HDMI_RX_TMDS0_CCTRL1_REG: c_uint = 0x10;
pub const HDMI_RX_TMDS_CLK_EN_REG: c_uint = 0x11;
pub const HDMI_RX_TMDS_CH_EN_REG: c_uint = 0x12;
pub const HDMI_RX_PLL_CALREFSEL_REG: c_uint = 0x17;
pub const HDMI_RX_PLL_VCOCAL_REG: c_uint = 0x1A;
pub const HDMI_RX_EQ_DATA0_REG: c_uint = 0x22;
pub const HDMI_RX_EQ_DATA1_REG: c_uint = 0x23;
pub const HDMI_RX_EQ_DATA2_REG: c_uint = 0x24;
pub const HDMI_RX_EQ_DATA3_REG: c_uint = 0x25;
pub const HDMI_RX_EQ_DATA4_REG: c_uint = 0x26;
pub const HDMI_RX_TMDS_ZONE_CTRL_REG: c_uint = 0x4C;
pub const HDMI_RX_TMDS_MODE_CTRL_REG: c_uint = 0x4D;
// CBUS registers
pub const CBUS_INT_STATUS_1_REG: c_uint = 0x08;
pub const CBUS_INTR1_ENABLE_REG: c_uint = 0x09;
pub const CBUS_MSC_REQ_ABORT_REASON_REG: c_uint = 0x0D;
pub const CBUS_INT_STATUS_2_REG: c_uint = 0x1E;
pub const CBUS_INTR2_ENABLE_REG: c_uint = 0x1F;
pub const CBUS_LINK_CONTROL_2_REG: c_uint = 0x31;
pub const CBUS_MHL_STATUS_REG_0: c_uint = 0xB0;
pub const CBUS_MHL_STATUS_REG_1: c_uint = 0xB1;

// TPI registers
pub const TPI_DPD_REG: c_uint = 0x3D;
// Timeouts in msec
pub const T_SRC_VBUS_CBUS_TO_STABLE: c_int = 200;
pub const T_SRC_CBUS_FLOAT: c_int = 100;
pub const T_SRC_CBUS_DEGLITCH: c_int = 2;
pub const T_SRC_RXSENSE_DEGLITCH: c_int = 110;

pub const I2C_TPI_ADDR: c_uint = 0x3D;
pub const I2C_HDMI_ADDR: c_uint = 0x49;
pub const I2C_CBUS_ADDR: c_uint = 0x64;
    enum sii9234_state {
    ST_OFF,
    ST_D3,
    ST_RGND_INIT,
    ST_RGND_1K,
    ST_RSEN_HIGH,
    ST_MHL_ESTABLISHED,
    ST_FAILURE_DISCOVERY,
    ST_FAILURE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sii9234 {
    pub client: [*mut i2c_client; 4],
    pub bridge: drm_bridge,
    pub dev: *mut device,
    pub gpio_reset: *mut gpio_desc,
    pub i2c_error: c_int,
    pub supplies: [regulator_bulk_data; 4],
    pub /: *mut *mut mutex lock; / Protects fields below and device registers,
    pub state: enum sii9234_state,
}

    enum sii9234_client_id {
    I2C_MHL,
    I2C_TPI,
    I2C_HDMI,
    I2C_CBUS,
    };
    static const char * const sii9234_client_name[] = {
    [I2C_MHL] = "MHL",
    [I2C_TPI] = "TPI",
    [I2C_HDMI] = "HDMI",
    [I2C_CBUS] = "CBUS",
    };
    static int sii9234_writeb(struct sii9234 *ctx, int id, int offset,
    int value)
    {
    int ret;
    struct i2c_client *client = ctx.client[id];
    if (ctx.i2c_error)
    return ctx.i2c_error;
    ret = i2c_smbus_write_byte_data(client, offset, value);
    if (ret < 0)
    dev_err(ctx.dev, "writeb: %4s[0x%02x] <- 0x%02x\n",
    sii9234_client_name[id], offset, value);
    ctx.i2c_error = ret;
    return ret;
    }
    static int sii9234_writebm(struct sii9234 *ctx, int id, int offset,
    int value, int mask)
    {
    int ret;
    struct i2c_client *client = ctx.client[id];
    if (ctx.i2c_error)
    return ctx.i2c_error;
    ret = i2c_smbus_write_byte(client, offset);
    if (ret < 0) {
    dev_err(ctx.dev, "writebm: %4s[0x%02x] <- 0x%02x\n",
    sii9234_client_name[id], offset, value);
    ctx.i2c_error = ret;
    return ret;
    }
    ret = i2c_smbus_read_byte(client);
    if (ret < 0) {
    dev_err(ctx.dev, "writebm: %4s[0x%02x] <- 0x%02x\n",
    sii9234_client_name[id], offset, value);
    ctx.i2c_error = ret;
    return ret;
    }
    value = (value & mask) | (ret & ~mask);
    ret = i2c_smbus_write_byte_data(client, offset, value);
    if (ret < 0) {
    dev_err(ctx.dev, "writebm: %4s[0x%02x] <- 0x%02x\n",
    sii9234_client_name[id], offset, value);
    ctx.i2c_error = ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_readb(ctx: *mut sii9234, id: c_int, offset: c_int) -> c_int {
    static int sii9234_readb(struct sii9234 *ctx, int id, int offset)
    {
    int ret;
    struct i2c_client *client = ctx.client[id];
    if (ctx.i2c_error)
    return ctx.i2c_error;
    ret = i2c_smbus_write_byte(client, offset);
    if (ret < 0) {
    dev_err(ctx.dev, "readb: %4s[0x%02x]\n",
    sii9234_client_name[id], offset);
    ctx.i2c_error = ret;
    return ret;
    }
    ret = i2c_smbus_read_byte(client);
    if (ret < 0) {
    dev_err(ctx.dev, "readb: %4s[0x%02x]\n",
    sii9234_client_name[id], offset);
    ctx.i2c_error = ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_clear_error(ctx: *mut sii9234) -> c_int {
    static int sii9234_clear_error(struct sii9234 *ctx)
    {
    let mut ret: c_int = ctx.i2c_error;
    ctx.i2c_error = 0;
    return ret;
    }

    sii9234_writeb(sii9234, I2C_MHL, offset, value)

    sii9234_writebm(sii9234, I2C_MHL, offset, value, mask)

    sii9234_readb(sii9234, I2C_MHL, offset)

    sii9234_writeb(sii9234, I2C_CBUS, offset, value)

    sii9234_writebm(sii9234, I2C_CBUS, offset, value, mask)

    sii9234_readb(sii9234, I2C_CBUS, offset)

    sii9234_writeb(sii9234, I2C_HDMI, offset, value)

    sii9234_writebm(sii9234, I2C_HDMI, offset, value, mask)

    sii9234_readb(sii9234, I2C_HDMI, offset)

    sii9234_writeb(sii9234, I2C_TPI, offset, value)

    sii9234_writebm(sii9234, I2C_TPI, offset, value, mask)

    sii9234_readb(sii9234, I2C_TPI, offset)
#[no_mangle]
unsafe extern "C" fn sii9234_tmds_control(ctx: *mut sii9234, enable: bool) -> u8 {
    static u8 sii9234_tmds_control(struct sii9234 *ctx, bool enable)
    {
    mhl_tx_writebm(ctx, MHL_TX_TMDS_CCTRL, enable ? ~0 : 0,
    BIT_TMDS_CCTRL_TMDS_OE);
    mhl_tx_writebm(ctx, MHL_TX_INT_CTRL_REG, enable ? ~0 : 0,
    MHL_HPD_OUT_OVR_EN | MHL_HPD_OUT_OVR_VAL);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_cbus_reset(ctx: *mut sii9234) -> c_int {
    static int sii9234_cbus_reset(struct sii9234 *ctx)
    {
    int i;
    mhl_tx_writebm(ctx, MHL_TX_SRST, ~0, BIT_CBUS_RESET);
    msleep(T_SRC_CBUS_DEGLITCH);
    mhl_tx_writebm(ctx, MHL_TX_SRST, 0, BIT_CBUS_RESET);
    for (i = 0; i < 4; i++) {
//
// Enable WRITE_STAT interrupt for writes to all
// 4 MSC Status registers.
//
    cbus_writeb(ctx, 0xE0 + i, 0xF2);
//
// Enable SET_INT interrupt for writes to all
// 4 MSC Interrupt registers.
//
    cbus_writeb(ctx, 0xF0 + i, 0xF2);
    }
    return sii9234_clear_error(ctx);
    }
// Require to check mhl information of samsung in cbus_init_register
#[no_mangle]
unsafe extern "C" fn sii9234_cbus_init(ctx: *mut sii9234) -> c_int {
    static int sii9234_cbus_init(struct sii9234 *ctx)
    {
    cbus_writeb(ctx, 0x07, 0xF2);
    cbus_writeb(ctx, 0x40, 0x03);
    cbus_writeb(ctx, 0x42, 0x06);
    cbus_writeb(ctx, 0x36, 0x0C);
    cbus_writeb(ctx, 0x3D, 0xFD);
    cbus_writeb(ctx, 0x1C, 0x01);
    cbus_writeb(ctx, 0x1D, 0x0F);
    cbus_writeb(ctx, 0x44, 0x02);
// Setup our devcap
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_DEV_STATE, 0x00);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_MHL_VERSION,
    SII9234_MHL_VERSION);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_CAT,
    MHL_DCAP_CAT_SOURCE);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_ADOPTER_ID_H, 0x01);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_ADOPTER_ID_L, 0x41);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_VID_LINK_MODE,
    MHL_DCAP_VID_LINK_RGB444 | MHL_DCAP_VID_LINK_YCBCR444);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_VIDEO_TYPE,
    MHL_DCAP_VT_GRAPHICS);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_LOG_DEV_MAP,
    MHL_DCAP_LD_GUI);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_BANDWIDTH, 0x0F);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_FEATURE_FLAG,
    MHL_DCAP_FEATURE_RCP_SUPPORT | MHL_DCAP_FEATURE_RAP_SUPPORT
    | MHL_DCAP_FEATURE_SP_SUPPORT);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_DEVICE_ID_H, 0x0);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_DEVICE_ID_L, 0x0);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_SCRATCHPAD_SIZE,
    SII9234_SCRATCHPAD_SIZE);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_INT_STAT_SIZE,
    SII9234_INT_STAT_SIZE);
    cbus_writeb(ctx, CBUS_DEVCAP_OFFSET + MHL_DCAP_RESERVED, 0);
    cbus_writebm(ctx, 0x31, 0x0C, 0x0C);
    cbus_writeb(ctx, 0x30, 0x01);
    cbus_writebm(ctx, 0x3C, 0x30, 0x38);
    cbus_writebm(ctx, 0x22, 0x0D, 0x0F);
    cbus_writebm(ctx, 0x2E, 0x15, 0x15);
    cbus_writeb(ctx, CBUS_INTR1_ENABLE_REG, 0);
    cbus_writeb(ctx, CBUS_INTR2_ENABLE_REG, 0);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn force_usb_id_switch_open(ctx: *mut sii9234) {
    static void force_usb_id_switch_open(struct sii9234 *ctx)
    {
// Disable CBUS discovery
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL1_REG, 0, 0x01);
// Force USB ID switch to open
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL6_REG, ~0, USB_ID_OVR);
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL3_REG, ~0, 0x86);
// Force upstream HPD to 0 when not in MHL mode.
    mhl_tx_writebm(ctx, MHL_TX_INT_CTRL_REG, 0, 0x30);
    }
#[no_mangle]
unsafe extern "C" fn release_usb_id_switch_open(ctx: *mut sii9234) {
    static void release_usb_id_switch_open(struct sii9234 *ctx)
    {
    msleep(T_SRC_CBUS_FLOAT);
// Clear USB ID switch to open
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL6_REG, 0, USB_ID_OVR);
// Enable CBUS discovery
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL1_REG, ~0, 0x01);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_power_init(ctx: *mut sii9234) -> c_int {
    static int sii9234_power_init(struct sii9234 *ctx)
    {
// Force the SiI9234 into the D0 state.
    tpi_writeb(ctx, TPI_DPD_REG, 0x3F);
// Enable TxPLL Clock
    hdmi_writeb(ctx, HDMI_RX_TMDS_CLK_EN_REG, 0x01);
// Enable Tx Clock Path & Equalizer
    hdmi_writeb(ctx, HDMI_RX_TMDS_CH_EN_REG, 0x15);
// Power Up TMDS
    mhl_tx_writeb(ctx, 0x08, 0x35);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_hdmi_init(ctx: *mut sii9234) -> c_int {
    static int sii9234_hdmi_init(struct sii9234 *ctx)
    {
    hdmi_writeb(ctx, HDMI_RX_TMDS0_CCTRL1_REG, 0xC1);
    hdmi_writeb(ctx, HDMI_RX_PLL_CALREFSEL_REG, 0x03);
    hdmi_writeb(ctx, HDMI_RX_PLL_VCOCAL_REG, 0x20);
    hdmi_writeb(ctx, HDMI_RX_EQ_DATA0_REG, 0x8A);
    hdmi_writeb(ctx, HDMI_RX_EQ_DATA1_REG, 0x6A);
    hdmi_writeb(ctx, HDMI_RX_EQ_DATA2_REG, 0xAA);
    hdmi_writeb(ctx, HDMI_RX_EQ_DATA3_REG, 0xCA);
    hdmi_writeb(ctx, HDMI_RX_EQ_DATA4_REG, 0xEA);
    hdmi_writeb(ctx, HDMI_RX_TMDS_ZONE_CTRL_REG, 0xA0);
    hdmi_writeb(ctx, HDMI_RX_TMDS_MODE_CTRL_REG, 0x00);
    mhl_tx_writeb(ctx, MHL_TX_TMDS_CCTRL, 0x34);
    hdmi_writeb(ctx, 0x45, 0x44);
    hdmi_writeb(ctx, 0x31, 0x0A);
    hdmi_writeb(ctx, HDMI_RX_TMDS0_CCTRL1_REG, 0xC1);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_mhl_tx_ctl_int(ctx: *mut sii9234) -> c_int {
    static int sii9234_mhl_tx_ctl_int(struct sii9234 *ctx)
    {
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL1_REG, 0xD0);
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL2_REG, 0xFC);
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL4_REG, 0xEB);
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL7_REG, 0x0C);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_reset(ctx: *mut sii9234) -> c_int {
    static int sii9234_reset(struct sii9234 *ctx)
    {
    int ret;
    sii9234_clear_error(ctx);
    ret = sii9234_power_init(ctx);
    if (ret < 0)
    return ret;
    ret = sii9234_cbus_reset(ctx);
    if (ret < 0)
    return ret;
    ret = sii9234_hdmi_init(ctx);
    if (ret < 0)
    return ret;
    ret = sii9234_mhl_tx_ctl_int(ctx);
    if (ret < 0)
    return ret;
// Enable HDCP Compliance safety
    mhl_tx_writeb(ctx, 0x2B, 0x01);
// CBUS discovery cycle time for each drive and float = 150us
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL1_REG, 0x04, 0x06);
// Clear bit 6 (reg_skip_rgnd)
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL2_REG, (1 << 7) /* Reserved */
    | 2 << ATT_THRESH_SHIFT | DEGLITCH_TIME_50MS);
//
// Changed from 66 to 65 for 94[1:0] = 01 = 5k reg_cbusmhl_pup_sel
// 1.8V CBUS VTH & GND threshold
// to meet CTS 3.3.7.2 spec
//
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL5_REG, 0x77);
    cbus_writebm(ctx, CBUS_LINK_CONTROL_2_REG, ~0, MHL_INIT_TIMEOUT);
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL6_REG, 0xA0);
// RGND & single discovery attempt (RGND blocking)
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL6_REG, BLOCK_RGND_INT |
    DVRFLT_SEL | SINGLE_ATT);
// Use VBUS path of discovery state machine
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL8_REG, 0);
// 0x92[3] sets the CBUS / ID switch
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL6_REG, ~0, USB_ID_OVR);
//
// To allow RGND engine to operate correctly.
// When moving the chip from D2 to D0 (power up, init regs)
// the values should be
// 94[1:0] = 01  reg_cbusmhl_pup_sel[1:0] should be set for 5k
// 93[7:6] = 10  reg_cbusdisc_pup_sel[1:0] should be
// set for 10k (default)
// 93[5:4] = 00  reg_cbusidle_pup_sel[1:0] = open (default)
//
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL3_REG, ~0, 0x86);
//
// Change from CC to 8C to match 5K
// to meet CTS 3.3.72 spec
//
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL4_REG, ~0, 0x8C);
// Configure the interrupt as active high
    mhl_tx_writebm(ctx, MHL_TX_INT_CTRL_REG, 0, 0x06);
    msleep(25);
// Release usb_id switch
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL6_REG, 0,  USB_ID_OVR);
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL1_REG, 0x27);
    ret = sii9234_clear_error(ctx);
    if (ret < 0)
    return ret;
    ret = sii9234_cbus_init(ctx);
    if (ret < 0)
    return ret;
// Enable Auto soft reset on SCDT = 0
    mhl_tx_writeb(ctx, 0x05, 0x04);
// HDMI Transcode mode enable
    mhl_tx_writeb(ctx, 0x0D, 0x1C);
    mhl_tx_writeb(ctx, MHL_TX_INTR4_ENABLE_REG,
    RGND_READY_MASK | CBUS_LKOUT_MASK
    | MHL_DISC_FAIL_MASK | MHL_EST_MASK);
    mhl_tx_writeb(ctx, MHL_TX_INTR1_ENABLE_REG, 0x60);
// This point is very important before measure RGND impedance
    force_usb_id_switch_open(ctx);
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL4_REG, 0, 0xF0);
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL5_REG, 0, 0x03);
    release_usb_id_switch_open(ctx);
// Force upstream HPD to 0 when not in MHL mode
    mhl_tx_writebm(ctx, MHL_TX_INT_CTRL_REG, 0, 1 << 5);
    mhl_tx_writebm(ctx, MHL_TX_INT_CTRL_REG, ~0, 1 << 4);
    return sii9234_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_goto_d3(ctx: *mut sii9234) -> c_int {
    static int sii9234_goto_d3(struct sii9234 *ctx)
    {
    int ret;
    dev_dbg(ctx.dev, "sii9234: detection started d3\n");
    ret = sii9234_reset(ctx);
    if (ret < 0)
    goto exit;
    hdmi_writeb(ctx, 0x01, 0x03);
    tpi_writebm(ctx, TPI_DPD_REG, 0, 1);
// I2C above is expected to fail because power goes down
    sii9234_clear_error(ctx);
    ctx.state = ST_D3;
    return 0;
    exit:
    dev_err(ctx.dev, "%s failed\n", __func__);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_hw_on(ctx: *mut sii9234) -> c_int {
    static int sii9234_hw_on(struct sii9234 *ctx)
    {
    return regulator_bulk_enable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_hw_off(ctx: *mut sii9234) {
    static void sii9234_hw_off(struct sii9234 *ctx)
    {
    gpiod_set_value(ctx.gpio_reset, 1);
    msleep(20);
    regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_hw_reset(ctx: *mut sii9234) {
    static void sii9234_hw_reset(struct sii9234 *ctx)
    {
    gpiod_set_value(ctx.gpio_reset, 1);
    msleep(20);
    gpiod_set_value(ctx.gpio_reset, 0);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_cable_in(ctx: *mut sii9234) {
    static void sii9234_cable_in(struct sii9234 *ctx)
    {
    int ret;
    mutex_lock(&ctx.lock);
    if (ctx.state != ST_OFF)
    goto unlock;
    ret = sii9234_hw_on(ctx);
    if (ret < 0)
    goto unlock;
    sii9234_hw_reset(ctx);
    sii9234_goto_d3(ctx);
// To avoid irq storm, when hw is in meta state
    enable_irq(to_i2c_client(ctx.dev).irq);
    unlock:
    mutex_unlock(&ctx.lock);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_cable_out(ctx: *mut sii9234) {
    static void sii9234_cable_out(struct sii9234 *ctx)
    {
    mutex_lock(&ctx.lock);
    if (ctx.state == ST_OFF)
    goto unlock;
    disable_irq(to_i2c_client(ctx.dev).irq);
    tpi_writeb(ctx, TPI_DPD_REG, 0);
// Turn on&off hpd feature for only QCT HDMI
    sii9234_hw_off(ctx);
    ctx.state = ST_OFF;
    unlock:
    mutex_unlock(&ctx.lock);
    }
#[no_mangle]
unsafe extern "C" fn sii9234_rgnd_ready_irq(ctx: *mut sii9234) -> enum sii9234_state {
    static enum sii9234_state sii9234_rgnd_ready_irq(struct sii9234 *ctx)
    {
    int value;
    if (ctx.state == ST_D3) {
    int ret;
    dev_dbg(ctx.dev, "RGND_READY_INT\n");
    sii9234_hw_reset(ctx);
    ret = sii9234_reset(ctx);
    if (ret < 0) {
    dev_err(ctx.dev, "sii9234_reset() failed\n");
    return ST_FAILURE;
    }
    return ST_RGND_INIT;
    }
// Got interrupt in inappropriate state
    if (ctx.state != ST_RGND_INIT)
    return ST_FAILURE;
    value = mhl_tx_readb(ctx, MHL_TX_STAT2_REG);
    if (sii9234_clear_error(ctx))
    return ST_FAILURE;
    if ((value & RGND_INTP_MASK) != RGND_INTP_1K) {
    dev_warn(ctx.dev, "RGND is not 1k\n");
    return ST_RGND_INIT;
    }
    dev_dbg(ctx.dev, "RGND 1K!!\n");
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL4_REG, ~0, 0x8C);
    mhl_tx_writeb(ctx, MHL_TX_DISC_CTRL5_REG, 0x77);
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL6_REG, ~0, 0x05);
    if (sii9234_clear_error(ctx))
    return ST_FAILURE;
    msleep(T_SRC_VBUS_CBUS_TO_STABLE);
    return ST_RGND_1K;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_mhl_established(ctx: *mut sii9234) -> enum sii9234_state {
    static enum sii9234_state sii9234_mhl_established(struct sii9234 *ctx)
    {
    dev_dbg(ctx.dev, "mhl est interrupt\n");
// Discovery override
    mhl_tx_writeb(ctx, MHL_TX_MHLTX_CTL1_REG, 0x10);
// Increase DDC translation layer timer (byte mode)
    cbus_writeb(ctx, 0x07, 0x32);
    cbus_writebm(ctx, 0x44, ~0, 1 << 1);
// Keep the discovery enabled. Need RGND interrupt
    mhl_tx_writebm(ctx, MHL_TX_DISC_CTRL1_REG, ~0, 1);
    mhl_tx_writeb(ctx, MHL_TX_INTR1_ENABLE_REG,
    RSEN_CHANGE_INT_MASK | HPD_CHANGE_INT_MASK);
    if (sii9234_clear_error(ctx))
    return ST_FAILURE;
    return ST_MHL_ESTABLISHED;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_hpd_change(ctx: *mut sii9234) -> enum sii9234_state {
    static enum sii9234_state sii9234_hpd_change(struct sii9234 *ctx)
    {
    int value;
    value = cbus_readb(ctx, CBUS_MSC_REQ_ABORT_REASON_REG);
    if (sii9234_clear_error(ctx))
    return ST_FAILURE;
    if (value & SET_HPD_DOWNSTREAM) {
// Downstream HPD High, Enable TMDS
    sii9234_tmds_control(ctx, true);
    } else {
// Downstream HPD Low, Disable TMDS
    sii9234_tmds_control(ctx, false);
    }
    return ctx.state;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_rsen_change(ctx: *mut sii9234) -> enum sii9234_state {
    static enum sii9234_state sii9234_rsen_change(struct sii9234 *ctx)
    {
    int value;
// Workaround code to handle wrong interrupt
    if (ctx.state != ST_RGND_1K) {
    dev_err(ctx.dev, "RSEN_HIGH without RGND_1K\n");
    return ST_FAILURE;
    }
    value = mhl_tx_readb(ctx, MHL_TX_SYSSTAT_REG);
    if (value < 0)
    return ST_FAILURE;
    if (value & RSEN_STATUS) {
    dev_dbg(ctx.dev, "MHL cable connected.. RSEN High\n");
    return ST_RSEN_HIGH;
    }
    dev_dbg(ctx.dev, "RSEN lost\n");
//
// Once RSEN loss is confirmed, we need to check
// based on cable status and chip power status, whether
// it is SINK Loss (HDMI cable not connected, TV Off)
// or MHL cable disconnection
// TODO: Define the below mhl_disconnection()
//
    msleep(T_SRC_RXSENSE_DEGLITCH);
    value = mhl_tx_readb(ctx, MHL_TX_SYSSTAT_REG);
    if (value < 0)
    return ST_FAILURE;
    dev_dbg(ctx.dev, "sys_stat: %x\n", value);
    if (value & RSEN_STATUS) {
    dev_dbg(ctx.dev, "RSEN recovery\n");
    return ST_RSEN_HIGH;
    }
    dev_dbg(ctx.dev, "RSEN Really LOW\n");
// To meet CTS 3.3.22.2 spec
    sii9234_tmds_control(ctx, false);
    force_usb_id_switch_open(ctx);
    release_usb_id_switch_open(ctx);
    return ST_FAILURE;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sii9234_irq_thread(int irq, void *data)
    {
    struct sii9234 *ctx = data;
    int intr1, intr4;
    int intr1_en, intr4_en;
    int cbus_intr1, cbus_intr2;
    dev_dbg(ctx.dev, "%s\n", __func__);
    mutex_lock(&ctx.lock);
    intr1 = mhl_tx_readb(ctx, MHL_TX_INTR1_REG);
    intr4 = mhl_tx_readb(ctx, MHL_TX_INTR4_REG);
    intr1_en = mhl_tx_readb(ctx, MHL_TX_INTR1_ENABLE_REG);
    intr4_en = mhl_tx_readb(ctx, MHL_TX_INTR4_ENABLE_REG);
    cbus_intr1 = cbus_readb(ctx, CBUS_INT_STATUS_1_REG);
    cbus_intr2 = cbus_readb(ctx, CBUS_INT_STATUS_2_REG);
    if (sii9234_clear_error(ctx))
    goto done;
    dev_dbg(ctx.dev, "irq %02x/%02x %02x/%02x %02x/%02x\n",
    intr1, intr1_en, intr4, intr4_en, cbus_intr1, cbus_intr2);
    if (intr4 & RGND_READY_INT)
    ctx.state = sii9234_rgnd_ready_irq(ctx);
    if (intr1 & RSEN_CHANGE_INT)
    ctx.state = sii9234_rsen_change(ctx);
    if (intr4 & MHL_EST_INT)
    ctx.state = sii9234_mhl_established(ctx);
    if (intr1 & HPD_CHANGE_INT)
    ctx.state = sii9234_hpd_change(ctx);
    if (intr4 & CBUS_LKOUT_INT)
    ctx.state = ST_FAILURE;
    if (intr4 & MHL_DISC_FAIL_INT)
    ctx.state = ST_FAILURE_DISCOVERY;
    done:
// Clean interrupt status and pending flags
    mhl_tx_writeb(ctx, MHL_TX_INTR1_REG, intr1);
    mhl_tx_writeb(ctx, MHL_TX_INTR4_REG, intr4);
    cbus_writeb(ctx, CBUS_MHL_STATUS_REG_0, 0xFF);
    cbus_writeb(ctx, CBUS_MHL_STATUS_REG_1, 0xFF);
    cbus_writeb(ctx, CBUS_INT_STATUS_1_REG, cbus_intr1);
    cbus_writeb(ctx, CBUS_INT_STATUS_2_REG, cbus_intr2);
    sii9234_clear_error(ctx);
    if (ctx.state == ST_FAILURE) {
    dev_dbg(ctx.dev, "try to reset after failure\n");
    sii9234_hw_reset(ctx);
    sii9234_goto_d3(ctx);
    }
    if (ctx.state == ST_FAILURE_DISCOVERY) {
    dev_err(ctx.dev, "discovery failed, no power for MHL?\n");
    tpi_writebm(ctx, TPI_DPD_REG, 0, 1);
    ctx.state = ST_D3;
    }
    mutex_unlock(&ctx.lock);
    return IRQ_HANDLED;
    }
    static int sii9234_init_resources(struct sii9234 *ctx,
    struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    int ret;
    if (!ctx.dev.of_node) {
    dev_err(ctx.dev, "no DT device\n");
    return -ENODEV;
    }
    ctx.gpio_reset = devm_gpiod_get(ctx.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.gpio_reset))
    return dev_err_probe(ctx.dev, PTR_ERR(ctx.gpio_reset),
    "failed to get reset gpio from DT\n");
    ctx.supplies[0].supply = "avcc12";
    ctx.supplies[1].supply = "avcc33";
    ctx.supplies[2].supply = "iovcc18";
    ctx.supplies[3].supply = "cvcc12";
    ret = devm_regulator_bulk_get(ctx.dev, 4, ctx.supplies);
    if (ret)
    return dev_err_probe(ctx.dev, ret, "regulator_bulk failed\n");
    ctx.client[I2C_MHL] = client;
    ctx.client[I2C_TPI] = devm_i2c_new_dummy_device(&client.dev, adapter,
    I2C_TPI_ADDR);
    if (IS_ERR(ctx.client[I2C_TPI])) {
    dev_err(ctx.dev, "failed to create TPI client\n");
    return PTR_ERR(ctx.client[I2C_TPI]);
    }
    ctx.client[I2C_HDMI] = devm_i2c_new_dummy_device(&client.dev, adapter,
    I2C_HDMI_ADDR);
    if (IS_ERR(ctx.client[I2C_HDMI])) {
    dev_err(ctx.dev, "failed to create HDMI RX client\n");
    return PTR_ERR(ctx.client[I2C_HDMI]);
    }
    ctx.client[I2C_CBUS] = devm_i2c_new_dummy_device(&client.dev, adapter,
    I2C_CBUS_ADDR);
    if (IS_ERR(ctx.client[I2C_CBUS])) {
    dev_err(ctx.dev, "failed to create CBUS client\n");
    return PTR_ERR(ctx.client[I2C_CBUS]);
    }
    return 0;
    }
    static enum drm_mode_status sii9234_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    if (mode.clock > MHL1_MAX_CLK)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
    static const struct drm_bridge_funcs sii9234_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .mode_valid = sii9234_mode_valid,
    };
#[no_mangle]
unsafe extern "C" fn sii9234_probe(client: *mut i2c_client) -> c_int {
    static int sii9234_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct sii9234 *ctx;
    struct device *dev = &client.dev;
    int ret;
    ctx = devm_drm_bridge_alloc(dev, struct sii9234, bridge,
    &sii9234_bridge_funcs);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.dev = dev;
    mutex_init(&ctx.lock);
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(dev, "I2C adapter lacks SMBUS feature\n");
    return -EIO;
    }
    if (!client.irq) {
    dev_err(dev, "no irq provided\n");
    return -EINVAL;
    }
    irq_set_status_flags(client.irq, IRQ_NOAUTOEN);
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    sii9234_irq_thread,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "sii9234", ctx);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "failed to install IRQ handler\n");
    ret = sii9234_init_resources(ctx, client);
    if (ret < 0)
    return ret;
    i2c_set_clientdata(client, ctx);
    ctx.bridge.of_node = dev.of_node;
    drm_bridge_add(&ctx.bridge);
    sii9234_cable_in(ctx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sii9234_remove(client: *mut i2c_client) {
    static void sii9234_remove(struct i2c_client *client)
    {
    struct sii9234 *ctx = i2c_get_clientdata(client);
    sii9234_cable_out(ctx);
    drm_bridge_remove(&ctx.bridge);
    }
    static const struct of_device_id sii9234_dt_match[] = {
    { .compatible = "sil,sii9234" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sii9234_dt_match);
    static const struct i2c_device_id sii9234_id[] = {
    { .name = "SII9234" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sii9234_id);
    static struct i2c_driver sii9234_driver = {
    .driver = {
    .name	= "sii9234",
    .of_match_table = sii9234_dt_match,
    },
    .probe = sii9234_probe,
    .remove = sii9234_remove,
    .id_table = sii9234_id,
    };
    module_i2c_driver(sii9234_driver);
    MODULE_DESCRIPTION("Silicon Image SII9234 HDMI/MHL bridge driver");
    MODULE_LICENSE("GPL");
