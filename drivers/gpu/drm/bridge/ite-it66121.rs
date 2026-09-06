//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/ite-it66121.c
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
// Copyright (C) 2020 BayLibre, SAS
// Author: Phong LE <ple@baylibre.com>
// Copyright (C) 2018-2019, Artem Mygaiev
// Copyright (C) 2017, Fresco Logic, Incorporated.
//

pub const IT66121_VENDOR_ID0_REG: c_uint = 0x00;
pub const IT66121_VENDOR_ID1_REG: c_uint = 0x01;
pub const IT66121_DEVICE_ID0_REG: c_uint = 0x02;
pub const IT66121_DEVICE_ID1_REG: c_uint = 0x03;

pub const IT66121_MASTER_SEL_REG: c_uint = 0x10;

pub const IT66121_AFE_DRV_REG: c_uint = 0x61;

pub const IT66121_INPUT_MODE_REG: c_uint = 0x70;

pub const IT66121_INPUT_CSC_REG: c_uint = 0x72;

pub const IT66121_INPUT_CSC_RGB_TO_YUV: c_uint = 0x02;
pub const IT66121_INPUT_CSC_YUV_TO_RGB: c_uint = 0x03;
pub const IT66121_INPUT_CSC_NO_CONV: c_uint = 0x00;
pub const IT66121_AFE_XP_REG: c_uint = 0x62;

pub const IT66121_AFE_IP_REG: c_uint = 0x64;

pub const IT66121_AFE_XP_EC1_REG: c_uint = 0x68;

pub const IT66121_SW_RST_REG: c_uint = 0x04;

pub const IT66121_DDC_COMMAND_REG: c_uint = 0x15;
pub const IT66121_DDC_COMMAND_BURST_READ: c_uint = 0x0;
pub const IT66121_DDC_COMMAND_EDID_READ: c_uint = 0x3;
pub const IT66121_DDC_COMMAND_FIFO_CLR: c_uint = 0x9;
pub const IT66121_DDC_COMMAND_SCL_PULSE: c_uint = 0xA;
pub const IT66121_DDC_COMMAND_ABORT: c_uint = 0xF;
pub const IT66121_HDCP_REG: c_uint = 0x20;

pub const IT66121_INT_STATUS1_REG: c_uint = 0x06;

pub const IT66121_DDC_HEADER_REG: c_uint = 0x11;
pub const IT66121_DDC_HEADER_HDCP: c_uint = 0x74;
pub const IT66121_DDC_HEADER_EDID: c_uint = 0xA0;
pub const IT66121_DDC_OFFSET_REG: c_uint = 0x12;
pub const IT66121_DDC_BYTE_REG: c_uint = 0x13;
pub const IT66121_DDC_SEGMENT_REG: c_uint = 0x14;
pub const IT66121_DDC_RD_FIFO_REG: c_uint = 0x17;
pub const IT66121_CLK_BANK_REG: c_uint = 0x0F;

pub const IT66121_CLK_BANK_0: c_int = 0;
pub const IT66121_CLK_BANK_1: c_int = 1;
pub const IT66121_INT_REG: c_uint = 0x05;

pub const IT66121_INT_MASK1_REG: c_uint = 0x09;

pub const IT66121_INT_CLR1_REG: c_uint = 0x0C;

pub const IT66121_AV_MUTE_REG: c_uint = 0xC1;

pub const IT66121_PKT_CTS_CTRL_REG: c_uint = 0xC5;

pub const IT66121_PKT_GEN_CTRL_REG: c_uint = 0xC6;

pub const IT66121_PKT_NULL_CTRL_REG: c_uint = 0xC9;

// Null packet data registers (used for HDMI Vendor Specific InfoFrame)

pub const IT66121_AVIINFO_DB1_REG: c_uint = 0x158;
pub const IT66121_AVIINFO_DB2_REG: c_uint = 0x159;
pub const IT66121_AVIINFO_DB3_REG: c_uint = 0x15A;
pub const IT66121_AVIINFO_DB4_REG: c_uint = 0x15B;
pub const IT66121_AVIINFO_DB5_REG: c_uint = 0x15C;
pub const IT66121_AVIINFO_CSUM_REG: c_uint = 0x15D;
pub const IT66121_AVIINFO_DB6_REG: c_uint = 0x15E;
pub const IT66121_AVIINFO_DB7_REG: c_uint = 0x15F;
pub const IT66121_AVIINFO_DB8_REG: c_uint = 0x160;
pub const IT66121_AVIINFO_DB9_REG: c_uint = 0x161;
pub const IT66121_AVIINFO_DB10_REG: c_uint = 0x162;
pub const IT66121_AVIINFO_DB11_REG: c_uint = 0x163;
pub const IT66121_AVIINFO_DB12_REG: c_uint = 0x164;
pub const IT66121_AVIINFO_DB13_REG: c_uint = 0x165;
pub const IT66121_AVI_INFO_PKT_REG: c_uint = 0xCD;

pub const IT66121_AUD_INFO_PKT_REG: c_uint = 0xCE;

pub const IT66121_AUD_INFO_DB1_REG: c_uint = 0x168;
pub const IT66121_AUD_INFO_CSUM_REG: c_uint = 0x16D;
pub const IT66121_HDMI_MODE_REG: c_uint = 0xC0;

pub const IT66121_HDMI_MODE_DVI: c_int = 0;
pub const IT66121_SYS_STATUS_REG: c_uint = 0x0E;

pub const IT66121_DDC_STATUS_REG: c_uint = 0x16;

pub const IT66121_EDID_SLEEP_US: c_int = 20000;
pub const IT66121_EDID_TIMEOUT_US: c_int = 200000;
pub const IT66121_EDID_FIFO_SIZE: c_int = 32;
pub const IT66121_CLK_CTRL0_REG: c_uint = 0x58;

pub const IT66121_CLK_STATUS1_REG: c_uint = 0x5E;
pub const IT66121_CLK_STATUS2_REG: c_uint = 0x5F;
pub const IT66121_AUD_CTRL0_REG: c_uint = 0xE0;

pub const IT66121_AUD_CTRL1_REG: c_uint = 0xE1;
pub const IT66121_AUD_FIFOMAP_REG: c_uint = 0xE2;
pub const IT66121_AUD_CTRL3_REG: c_uint = 0xE3;
pub const IT66121_AUD_SRCVALID_FLAT_REG: c_uint = 0xE4;

pub const IT66121_AUD_HDAUDIO_REG: c_uint = 0xE5;
pub const IT66121_AUD_PKT_CTS0_REG: c_uint = 0x130;
pub const IT66121_AUD_PKT_CTS1_REG: c_uint = 0x131;
pub const IT66121_AUD_PKT_CTS2_REG: c_uint = 0x132;
pub const IT66121_AUD_PKT_N0_REG: c_uint = 0x133;
pub const IT66121_AUD_PKT_N1_REG: c_uint = 0x134;
pub const IT66121_AUD_PKT_N2_REG: c_uint = 0x135;
pub const IT66121_AUD_CHST_MODE_REG: c_uint = 0x191;
pub const IT66121_AUD_CHST_CAT_REG: c_uint = 0x192;
pub const IT66121_AUD_CHST_SRCNUM_REG: c_uint = 0x193;
pub const IT66121_AUD_CHST_CHTNUM_REG: c_uint = 0x194;
pub const IT66121_AUD_CHST_CA_FS_REG: c_uint = 0x198;
pub const IT66121_AUD_CHST_OFS_WL_REG: c_uint = 0x199;
pub const IT66121_AUD_PKT_CTS_CNT0_REG: c_uint = 0x1A0;
pub const IT66121_AUD_PKT_CTS_CNT1_REG: c_uint = 0x1A1;
pub const IT66121_AUD_PKT_CTS_CNT2_REG: c_uint = 0x1A2;
pub const IT66121_AUD_FS_22P05K: c_uint = 0x4;
pub const IT66121_AUD_FS_44P1K: c_uint = 0x0;
pub const IT66121_AUD_FS_88P2K: c_uint = 0x8;
pub const IT66121_AUD_FS_176P4K: c_uint = 0xC;
pub const IT66121_AUD_FS_24K: c_uint = 0x6;
pub const IT66121_AUD_FS_48K: c_uint = 0x2;
pub const IT66121_AUD_FS_96K: c_uint = 0xA;
pub const IT66121_AUD_FS_192K: c_uint = 0xE;
pub const IT66121_AUD_FS_768K: c_uint = 0x9;
pub const IT66121_AUD_FS_32K: c_uint = 0x3;
pub const IT66121_AUD_FS_OTHER: c_uint = 0x1;
pub const IT66121_AUD_SWL_21BIT: c_uint = 0xD;
pub const IT66121_AUD_SWL_24BIT: c_uint = 0xB;
pub const IT66121_AUD_SWL_23BIT: c_uint = 0x9;
pub const IT66121_AUD_SWL_22BIT: c_uint = 0x5;
pub const IT66121_AUD_SWL_20BIT: c_uint = 0x3;
pub const IT66121_AUD_SWL_17BIT: c_uint = 0xC;
pub const IT66121_AUD_SWL_19BIT: c_uint = 0x8;
pub const IT66121_AUD_SWL_18BIT: c_uint = 0x4;
pub const IT66121_AUD_SWL_16BIT: c_uint = 0x2;
pub const IT66121_AUD_SWL_NOT_INDICATED: c_uint = 0x0;

    enum chip_id {
    ID_IT6610,
    ID_IT66121,
    ID_IT66122,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct it66121_chip_info {
    pub id: enum chip_id,
    pub pid: u16 vid,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct it66121_ctx {
    pub regmap: *mut regmap,
    pub bridge: drm_bridge,
    pub dev: *mut device,
    pub gpio_reset: *mut gpio_desc,
    pub client: *mut i2c_client,
    pub bus_width: u32,
    pub /: *mut *mut mutex lock; / Protects fields below and device registers,
    struct {
    pub ch_enable: u8,
    pub fs: u8,
    pub swl: u8,
    pub auto_cts: bool,
    pub audio: },
    pub id: enum chip_id,
}

    static const struct regmap_range_cfg it66121_regmap_banks[] = {
    {
    .name = "it66121",
    .range_min = 0x00,
    .range_max = 0x1FF,
    .selector_reg = IT66121_CLK_BANK_REG,
    .selector_mask = 0x1,
    .selector_shift = 0,
    .window_start = 0x00,
    .window_len = 0x100,
    },
    };
    static const struct regmap_config it66121_regmap_config = {
    .val_bits = 8,
    .reg_bits = 8,
    .max_register = 0x1FF,
    .ranges = it66121_regmap_banks,
    .num_ranges = ARRAY_SIZE(it66121_regmap_banks),
    };
#[no_mangle]
unsafe extern "C" fn it66121_hw_reset(ctx: *mut it66121_ctx) {
    static void it66121_hw_reset(struct it66121_ctx *ctx)
    {
    gpiod_set_value(ctx.gpio_reset, 1);
    msleep(20);
    gpiod_set_value(ctx.gpio_reset, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn it66121_preamble_ddc(ctx: *mut it66121_ctx) -> c_int {
    static inline int it66121_preamble_ddc(struct it66121_ctx *ctx)
    {
    return regmap_write(ctx.regmap, IT66121_MASTER_SEL_REG, IT66121_MASTER_SEL_HOST);
    }
#[no_mangle]
pub unsafe extern "C" fn it66121_fire_afe(ctx: *mut it66121_ctx) -> c_int {
    static inline int it66121_fire_afe(struct it66121_ctx *ctx)
    {
    return regmap_write(ctx.regmap, IT66121_AFE_DRV_REG, 0);
    }
// TOFIX: Handle YCbCr Input & Output
#[no_mangle]
unsafe extern "C" fn it66121_configure_input(ctx: *mut it66121_ctx) -> c_int {
    static int it66121_configure_input(struct it66121_ctx *ctx)
    {
    int ret;
    let mut mode: u8 = IT66121_INPUT_MODE_RGB;
    if (ctx.bus_width == 12)
    mode |= IT66121_INPUT_MODE_DDR;
    ret = regmap_write(ctx.regmap, IT66121_INPUT_MODE_REG, mode);
    if (ret)
    return ret;
    return regmap_write(ctx.regmap, IT66121_INPUT_CSC_REG, IT66121_INPUT_CSC_NO_CONV);
    }
//
// it66121_configure_afe() - Configure the analog front end
// @ctx: it66121_ctx object
// @mode: mode to configure
//
// RETURNS:
// zero if success, a negative error code otherwise.
//
    static int it66121_configure_afe(struct it66121_ctx *ctx,
    const struct drm_display_mode *mode)
    {
    int ret;
    ret = regmap_write(ctx.regmap, IT66121_AFE_DRV_REG,
    IT66121_AFE_DRV_RST);
    if (ret)
    return ret;
    if (mode.clock > IT66121_AFE_CLK_HIGH) {
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_REG,
    IT66121_AFE_XP_GAINBIT |
    IT66121_AFE_XP_ENO,
    IT66121_AFE_XP_GAINBIT);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_GAINBIT |
    IT66121_AFE_IP_ER0,
    IT66121_AFE_IP_GAINBIT);
    if (ret)
    return ret;
    if (ctx.id == ID_IT66121 || ctx.id == ID_IT66122) {
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_EC1, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_EC1_REG,
    IT66121_AFE_XP_EC1_LOWCLK, 0x80);
    if (ret)
    return ret;
    }
    } else {
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_REG,
    IT66121_AFE_XP_GAINBIT |
    IT66121_AFE_XP_ENO,
    IT66121_AFE_XP_ENO);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_GAINBIT |
    IT66121_AFE_IP_ER0,
    IT66121_AFE_IP_ER0);
    if (ret)
    return ret;
    if (ctx.id == ID_IT66121 || ctx.id == ID_IT66122) {
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_EC1,
    IT66121_AFE_IP_EC1);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_EC1_REG,
    IT66121_AFE_XP_EC1_LOWCLK,
    IT66121_AFE_XP_EC1_LOWCLK);
    if (ret)
    return ret;
    }
    }
// Clear reset flags
    ret = regmap_write_bits(ctx.regmap, IT66121_SW_RST_REG,
    IT66121_SW_RST_REF | IT66121_SW_RST_VID, 0);
    if (ret)
    return ret;
    if (ctx.id == ID_IT6610) {
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_REG,
    IT6610_AFE_XP_BYPASS,
    IT6610_AFE_XP_BYPASS);
    if (ret)
    return ret;
    }
    return it66121_fire_afe(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn it66121_wait_ddc_ready(ctx: *mut it66121_ctx) -> c_int {
    static inline int it66121_wait_ddc_ready(struct it66121_ctx *ctx)
    {
    int ret, val;
    u32 error = IT66121_DDC_STATUS_NOACK | IT66121_DDC_STATUS_WAIT_BUS |
    IT66121_DDC_STATUS_ARBI_LOSE;
    let mut done: u32 = IT66121_DDC_STATUS_TX_DONE;
    ret = regmap_read_poll_timeout(ctx.regmap, IT66121_DDC_STATUS_REG, val,
    val & (error | done), IT66121_EDID_SLEEP_US,
    IT66121_EDID_TIMEOUT_US);
    if (ret)
    return ret;
    if (val & error)
    return -EAGAIN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it66121_abort_ddc_ops(ctx: *mut it66121_ctx) -> c_int {
    static int it66121_abort_ddc_ops(struct it66121_ctx *ctx)
    {
    int ret;
    unsigned int swreset, cpdesire;
    ret = regmap_read(ctx.regmap, IT66121_SW_RST_REG, &swreset);
    if (ret)
    return ret;
    ret = regmap_read(ctx.regmap, IT66121_HDCP_REG, &cpdesire);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_HDCP_REG,
    cpdesire & (~IT66121_HDCP_CPDESIRED & 0xFF));
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_SW_RST_REG,
    (swreset | IT66121_SW_RST_HDCP));
    if (ret)
    return ret;
    ret = it66121_preamble_ddc(ctx);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_DDC_COMMAND_REG,
    IT66121_DDC_COMMAND_ABORT);
    if (ret)
    return ret;
    return it66121_wait_ddc_ready(ctx);
    }
    static int it66121_get_edid_block(void *context, u8 *buf,
    unsigned int block, size_t len)
    {
    struct it66121_ctx *ctx = context;
    let mut remain: c_int = len;
    let mut offset: c_int = 0;
    int ret, cnt;
    offset = (block % 2) * len;
    block = block / 2;
    while (remain > 0) {
    cnt = (remain > IT66121_EDID_FIFO_SIZE) ?
    IT66121_EDID_FIFO_SIZE : remain;
    ret = regmap_write(ctx.regmap, IT66121_DDC_COMMAND_REG,
    IT66121_DDC_COMMAND_FIFO_CLR);
    if (ret)
    return ret;
    ret = it66121_wait_ddc_ready(ctx);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_DDC_OFFSET_REG, offset);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_DDC_BYTE_REG, cnt);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_DDC_SEGMENT_REG, block);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_DDC_COMMAND_REG,
    IT66121_DDC_COMMAND_EDID_READ);
    if (ret)
    return ret;
    offset += cnt;
    remain -= cnt;
    ret = it66121_wait_ddc_ready(ctx);
    if (ret) {
    it66121_abort_ddc_ops(ctx);
    return ret;
    }
    ret = regmap_noinc_read(ctx.regmap, IT66121_DDC_RD_FIFO_REG,
    buf, cnt);
    if (ret)
    return ret;
    buf += cnt;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it66121_is_hpd_detect(ctx: *mut it66121_ctx) -> bool {
    static bool it66121_is_hpd_detect(struct it66121_ctx *ctx)
    {
    int val;
    if (regmap_read(ctx.regmap, IT66121_SYS_STATUS_REG, &val))
    return false;
    return val & IT66121_SYS_STATUS_HPDETECT;
    }
    static int it66121_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    if (!(flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR))
    return -EINVAL;
    ret = drm_bridge_attach(encoder, ctx.bridge.next_bridge, bridge, flags);
    if (ret)
    return ret;
    if (ctx.id == ID_IT66121 || ctx.id == ID_IT66122) {
    ret = regmap_write_bits(ctx.regmap, IT66121_CLK_BANK_REG,
    IT66121_CLK_BANK_PWROFF_RCLK, 0);
    if (ret)
    return ret;
    }
    ret = regmap_write_bits(ctx.regmap, IT66121_INT_REG,
    IT66121_INT_TX_CLK_OFF, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_DRV_REG,
    IT66121_AFE_DRV_PWD, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_REG,
    IT66121_AFE_XP_PWDI | IT66121_AFE_XP_PWDPLL, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_PWDPLL, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_DRV_REG,
    IT66121_AFE_DRV_RST, 0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_XP_REG,
    IT66121_AFE_XP_RESETB, IT66121_AFE_XP_RESETB);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AFE_IP_REG,
    IT66121_AFE_IP_RESETB, IT66121_AFE_IP_RESETB);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_SW_RST_REG,
    IT66121_SW_RST_REF,
    IT66121_SW_RST_REF);
    if (ret)
    return ret;
// Per programming manual, sleep here for bridge to settle
    msleep(50);
    return 0;
    }
    static void it66121_set_mode(struct it66121_ctx *ctx,
    struct drm_connector *connector,
    struct drm_atomic_commit *state)
    {
    const struct drm_connector_state *conn_state;
    const struct drm_crtc_state *crtc_state;
    const struct drm_display_mode *mode;
    struct drm_crtc *crtc;
    conn_state = drm_atomic_get_new_connector_state(state, connector);
    if (WARN_ON(!conn_state))
    return;
    crtc = conn_state.crtc;
    if (WARN_ON(!crtc))
    return;
    crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    if (WARN_ON(!crtc_state))
    return;
    mode = &crtc_state.adjusted_mode;
    mutex_lock(&ctx.lock);
// Set TX mode to HDMI or DVI
    if (regmap_write(ctx.regmap, IT66121_HDMI_MODE_REG,
    connector.display_info.is_hdmi ?
    IT66121_HDMI_MODE_HDMI : IT66121_HDMI_MODE_DVI))
    goto unlock;
    if ((ctx.id == ID_IT66121 || ctx.id == ID_IT66122) &&
    regmap_write_bits(ctx.regmap, IT66121_CLK_BANK_REG,
    IT66121_CLK_BANK_PWROFF_TXCLK,
    IT66121_CLK_BANK_PWROFF_TXCLK)) {
    goto unlock;
    }
    if (it66121_configure_input(ctx))
    goto unlock;
    if (it66121_configure_afe(ctx, mode))
    goto unlock;
    if ((ctx.id == ID_IT66121 || ctx.id == ID_IT66122) &&
    regmap_write_bits(ctx.regmap, IT66121_CLK_BANK_REG,
    IT66121_CLK_BANK_PWROFF_TXCLK, 0)) {
    goto unlock;
    }
    unlock:
    mutex_unlock(&ctx.lock);
    }
#[no_mangle]
unsafe extern "C" fn it66121_set_mute(ctx: *mut it66121_ctx, mute: bool) -> c_int {
    static int it66121_set_mute(struct it66121_ctx *ctx, bool mute)
    {
    int ret;
    let mut val: c_uint = 0;
    if (mute)
    val = IT66121_AV_MUTE_ON;
    ret = regmap_write_bits(ctx.regmap, IT66121_AV_MUTE_REG, IT66121_AV_MUTE_ON, val);
    if (ret)
    return ret;
    return regmap_write(ctx.regmap, IT66121_PKT_GEN_CTRL_REG,
    IT66121_PKT_GEN_CTRL_ON | IT66121_PKT_GEN_CTRL_RPT);
    }
pub const MAX_OUTPUT_SEL_FORMATS: c_int = 1;
    static u32 *it66121_bridge_atomic_get_output_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    unsigned int *num_output_fmts)
    {
    u32 *output_fmts;
    output_fmts = kcalloc(MAX_OUTPUT_SEL_FORMATS, sizeof(*output_fmts),
    GFP_KERNEL);
    if (!output_fmts)
    return core::ptr::null_mut();
// TOFIX handle more than MEDIA_BUS_FMT_RGB888_1X24 as output format
    output_fmts[0] =  MEDIA_BUS_FMT_RGB888_1X24;
// num_output_fmts = 1;
    return output_fmts;
    }
pub const MAX_INPUT_SEL_FORMATS: c_int = 1;
    static u32 *it66121_bridge_atomic_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    u32 *input_fmts;
// num_input_fmts = 0;
    input_fmts = kcalloc(MAX_INPUT_SEL_FORMATS, sizeof(*input_fmts),
    GFP_KERNEL);
    if (!input_fmts)
    return core::ptr::null_mut();
    if (ctx.bus_width == 12)
// IT66121FN Datasheet specifies Little-Endian ordering
    input_fmts[0] = MEDIA_BUS_FMT_RGB888_2X12_LE;
    else
// TOFIX support more input bus formats in 24bit width
    input_fmts[0] = MEDIA_BUS_FMT_RGB888_1X24;
// num_input_fmts = 1;
    return input_fmts;
    }
    static void it66121_bridge_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    struct drm_connector *connector;
    connector = drm_atomic_get_new_connector_for_encoder(state, bridge.encoder);
    if (WARN_ON(!connector))
    return;
    drm_atomic_helper_connector_hdmi_update_infoframes(connector, state);
    it66121_set_mode(ctx, connector, state);
    it66121_set_mute(ctx, false);
    }
    static void it66121_bridge_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    it66121_set_mute(ctx, true);
    }
    static int it66121_bridge_check(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    if (ctx.id == ID_IT6610) {
// The IT6610 only supports these settings
    bridge_state.input_bus_cfg.flags |= DRM_BUS_FLAG_DE_HIGH |
    DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE;
    bridge_state.input_bus_cfg.flags &=
    ~DRM_BUS_FLAG_PIXDATA_DRIVE_POSEDGE;
    }
    return 0;
    }
    static enum drm_connector_status
    it66121_bridge_detect(struct drm_bridge *bridge, struct drm_connector *connector)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    return it66121_is_hpd_detect(ctx) ? connector_status_connected
    : connector_status_disconnected;
    }
#[no_mangle]
unsafe extern "C" fn it66121_bridge_hpd_enable(bridge: *mut drm_bridge) {
    static void it66121_bridge_hpd_enable(struct drm_bridge *bridge)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_INT_MASK1_REG, IT66121_INT_MASK1_HPD, 0);
    if (ret)
    dev_err(ctx.dev, "failed to enable HPD IRQ\n");
    }
#[no_mangle]
unsafe extern "C" fn it66121_bridge_hpd_disable(bridge: *mut drm_bridge) {
    static void it66121_bridge_hpd_disable(struct drm_bridge *bridge)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_INT_MASK1_REG,
    IT66121_INT_MASK1_HPD, IT66121_INT_MASK1_HPD);
    if (ret)
    dev_err(ctx.dev, "failed to disable HPD IRQ\n");
    }
    static enum drm_mode_status
    it66121_bridge_hdmi_tmds_char_rate_valid(const struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    unsigned long long tmds_rate)
    {
    const struct it66121_ctx *ctx =
    container_of(bridge, const struct it66121_ctx, bridge);
    unsigned long long max_rate;
    max_rate = (ctx.bus_width == 12) ? 74250000ULL : 148500000ULL;
    if (tmds_rate > max_rate)
    return MODE_CLOCK_HIGH;
    if (tmds_rate < HDMI_TMDS_CHAR_RATE_MIN_HZ)
    return MODE_CLOCK_LOW;
    return MODE_OK;
    }
#[no_mangle]
unsafe extern "C" fn it66121_bridge_hdmi_clear_avi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int it66121_bridge_hdmi_clear_avi_infoframe(struct drm_bridge *bridge)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
// Clear both IT66121_AVI_INFO_PKT_ON and IT66121_AVI_INFO_PKT_RPT
    return regmap_write(ctx.regmap, IT66121_AVI_INFO_PKT_REG, 0);
    }
    static int it66121_bridge_hdmi_write_avi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    mutex_lock(&ctx.lock);
// Write new AVI infoframe packet
    ret = regmap_bulk_write(ctx.regmap, IT66121_AVIINFO_DB1_REG,
    &buffer[HDMI_INFOFRAME_HEADER_SIZE],
    HDMI_AVI_INFOFRAME_SIZE);
    if (ret)
    goto unlock;
    ret = regmap_write(ctx.regmap, IT66121_AVIINFO_CSUM_REG, buffer[3]);
    if (ret)
    goto unlock;
// Enable AVI infoframe
    ret = regmap_write(ctx.regmap, IT66121_AVI_INFO_PKT_REG,
    IT66121_AVI_INFO_PKT_ON | IT66121_AVI_INFO_PKT_RPT);
    unlock:
    mutex_unlock(&ctx.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn it66121_bridge_hdmi_clear_hdmi_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int it66121_bridge_hdmi_clear_hdmi_infoframe(struct drm_bridge *bridge)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
// Clear both IT66121_PKT_NULL_CTRL_ON and IT66121_PKT_NULL_CTRL_RPT
    return regmap_write(ctx.regmap, IT66121_PKT_NULL_CTRL_REG, 0);
    }
    static int it66121_bridge_hdmi_write_hdmi_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    mutex_lock(&ctx.lock);
// Write new HDMI Vendor Specific Infoframe packet
    ret = regmap_bulk_write(ctx.regmap, IT66121_PKT_NULL_HB(0), buffer, len);
    if (ret)
    goto unlock;
// Enable HDMI Vendor Specific Infoframe
    ret = regmap_write(ctx.regmap, IT66121_PKT_NULL_CTRL_REG,
    IT66121_PKT_NULL_CTRL_ON | IT66121_PKT_NULL_CTRL_RPT);
    unlock:
    mutex_unlock(&ctx.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn it66121_bridge_hdmi_clear_audio_infoframe(bridge: *mut drm_bridge) -> c_int {
    static int it66121_bridge_hdmi_clear_audio_infoframe(struct drm_bridge *bridge)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
// Clear both IT66121_AUD_INFO_PKT_ON and IT66121_AUD_INFO_PKT_RPT
    return regmap_write(ctx.regmap, IT66121_AUD_INFO_PKT_REG, 0);
    }
    static int it66121_bridge_hdmi_write_audio_infoframe(struct drm_bridge *bridge,
    const u8 *buffer, size_t len)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    int ret;
    mutex_lock(&ctx.lock);
// Write new Audio infoframe packet
    ret = regmap_bulk_write(ctx.regmap, IT66121_AUD_INFO_DB1_REG,
    &buffer[HDMI_INFOFRAME_HEADER_SIZE],
    min_t(size_t, len - HDMI_INFOFRAME_HEADER_SIZE, 5));
    if (ret)
    goto unlock;
    ret = regmap_write(ctx.regmap, IT66121_AUD_INFO_CSUM_REG, buffer[3]);
    if (ret)
    goto unlock;
// Enable Audio infoframe
    ret = regmap_write(ctx.regmap, IT66121_AUD_INFO_PKT_REG,
    IT66121_AUD_INFO_PKT_ON | IT66121_AUD_INFO_PKT_RPT);
    unlock:
    mutex_unlock(&ctx.lock);
    return ret;
    }
    static const struct drm_edid *it66121_bridge_edid_read(struct drm_bridge *bridge,
    struct drm_connector *connector)
    {
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    const struct drm_edid *drm_edid;
    int ret;
    mutex_lock(&ctx.lock);
    ret = it66121_preamble_ddc(ctx);
    if (ret) {
    drm_edid = core::ptr::null_mut();
    goto out_unlock;
    }
    ret = regmap_write(ctx.regmap, IT66121_DDC_HEADER_REG,
    IT66121_DDC_HEADER_EDID);
    if (ret) {
    drm_edid = core::ptr::null_mut();
    goto out_unlock;
    }
    drm_edid = drm_edid_read_custom(connector, it66121_get_edid_block, ctx);
    out_unlock:
    mutex_unlock(&ctx.lock);
    return drm_edid;
    }
#[no_mangle]
unsafe extern "C" fn it66121_irq_threaded_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t it66121_irq_threaded_handler(int irq, void *dev_id)
    {
    int ret;
    unsigned int val;
    struct it66121_ctx *ctx = dev_id;
    struct device *dev = ctx.dev;
    enum drm_connector_status status;
    let mut event: bool = false;
    mutex_lock(&ctx.lock);
    ret = regmap_read(ctx.regmap, IT66121_SYS_STATUS_REG, &val);
    if (ret)
    goto unlock;
    if (!(val & IT66121_SYS_STATUS_ACTIVE_IRQ))
    goto unlock;
    ret = regmap_read(ctx.regmap, IT66121_INT_STATUS1_REG, &val);
    if (ret) {
    dev_err(dev, "Cannot read STATUS1_REG %d\n", ret);
    } else if (val & IT66121_INT_STATUS1_HPD_STATUS) {
    regmap_write_bits(ctx.regmap, IT66121_INT_CLR1_REG,
    IT66121_INT_CLR1_HPD, IT66121_INT_CLR1_HPD);
    status = it66121_is_hpd_detect(ctx) ? connector_status_connected
    : connector_status_disconnected;
    event = true;
    }
    regmap_write_bits(ctx.regmap, IT66121_SYS_STATUS_REG,
    IT66121_SYS_STATUS_CLEAR_IRQ,
    IT66121_SYS_STATUS_CLEAR_IRQ);
    unlock:
    mutex_unlock(&ctx.lock);
    if (event)
    drm_bridge_hpd_notify(&ctx.bridge, status);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn it661221_set_chstat(ctx: *mut it66121_ctx, iec60958_chstat[]: u8) -> c_int {
    static int it661221_set_chstat(struct it66121_ctx *ctx, u8 iec60958_chstat[])
    {
    int ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CHST_MODE_REG, iec60958_chstat[0] & 0x7C);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CHST_CAT_REG, iec60958_chstat[1]);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CHST_SRCNUM_REG, iec60958_chstat[2] & 0x0F);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CHST_CHTNUM_REG,
    (iec60958_chstat[2] >> 4) & 0x0F);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CHST_CA_FS_REG, iec60958_chstat[3]);
    if (ret)
    return ret;
    return regmap_write(ctx.regmap, IT66121_AUD_CHST_OFS_WL_REG, iec60958_chstat[4]);
    }
#[no_mangle]
unsafe extern "C" fn it661221_set_lpcm_audio(ctx: *mut it66121_ctx, audio_src_num: u8, audio_swl: u8) -> c_int {
    static int it661221_set_lpcm_audio(struct it66121_ctx *ctx, u8 audio_src_num, u8 audio_swl)
    {
    int ret;
    let mut audio_enable: c_uint = 0;
    let mut audio_format: c_uint = 0;
    switch (audio_swl) {
    case 16:
    audio_enable |= IT66121_AUD_16BIT;
    break;
    case 18:
    audio_enable |= IT66121_AUD_18BIT;
    break;
    case 20:
    audio_enable |= IT66121_AUD_20BIT;
    break;
    case 24:
    default:
    audio_enable |= IT66121_AUD_24BIT;
    break;
    }
    audio_format |= 0x40;
    switch (audio_src_num) {
    case 4:
    audio_enable |= IT66121_AUD_EN_I2S3 | IT66121_AUD_EN_I2S2 |
    IT66121_AUD_EN_I2S1 | IT66121_AUD_EN_I2S0;
    break;
    case 3:
    audio_enable |= IT66121_AUD_EN_I2S2 | IT66121_AUD_EN_I2S1 |
    IT66121_AUD_EN_I2S0;
    break;
    case 2:
    audio_enable |= IT66121_AUD_EN_I2S1 | IT66121_AUD_EN_I2S0;
    break;
    case 1:
    default:
    audio_format &= ~0x40;
    audio_enable |= IT66121_AUD_EN_I2S0;
    break;
    }
    audio_format |= 0x01;
    ctx.audio.ch_enable = audio_enable;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CTRL0_REG, audio_enable & 0xF0);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CTRL1_REG, audio_format);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_FIFOMAP_REG, 0xE4);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CTRL3_REG, 0x00);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_SRCVALID_FLAT_REG, 0x00);
    if (ret)
    return ret;
    return regmap_write(ctx.regmap, IT66121_AUD_HDAUDIO_REG, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn it661221_set_ncts(ctx: *mut it66121_ctx, fs: u8) -> c_int {
    static int it661221_set_ncts(struct it66121_ctx *ctx, u8 fs)
    {
    int ret;
    unsigned int n;
    switch (fs) {
    case IT66121_AUD_FS_32K:
    n = 4096;
    break;
    case IT66121_AUD_FS_44P1K:
    n = 6272;
    break;
    case IT66121_AUD_FS_48K:
    n = 6144;
    break;
    case IT66121_AUD_FS_88P2K:
    n = 12544;
    break;
    case IT66121_AUD_FS_96K:
    n = 12288;
    break;
    case IT66121_AUD_FS_176P4K:
    n = 25088;
    break;
    case IT66121_AUD_FS_192K:
    n = 24576;
    break;
    case IT66121_AUD_FS_768K:
    n = 24576;
    break;
    default:
    n = 6144;
    break;
    }
    ret = regmap_write(ctx.regmap, IT66121_AUD_PKT_N0_REG, (u8)((n) & 0xFF));
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_PKT_N1_REG, (u8)((n >> 8) & 0xFF));
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_PKT_N2_REG, (u8)((n >> 16) & 0xF));
    if (ret)
    return ret;
    if (ctx.audio.auto_cts) {
    let mut loop_cnt: u8 = 255;
    let mut cts_stable_cnt: u8 = 0;
    let mut sum_cts: c_uint = 0;
    let mut cts: c_uint = 0;
    let mut last_cts: c_uint = 0;
    unsigned int diff;
    unsigned int val;
    while (loop_cnt--) {
    msleep(30);
    regmap_read(ctx.regmap, IT66121_AUD_PKT_CTS_CNT2_REG, &val);
    cts = val << 12;
    regmap_read(ctx.regmap, IT66121_AUD_PKT_CTS_CNT1_REG, &val);
    cts |= val << 4;
    regmap_read(ctx.regmap, IT66121_AUD_PKT_CTS_CNT0_REG, &val);
    cts |= val >> 4;
    if (cts == 0) {
    continue;
    } else {
    if (last_cts > cts)
    diff = last_cts - cts;
    else
    diff = cts - last_cts;
    last_cts = cts;
    if (diff < 5) {
    cts_stable_cnt++;
    sum_cts += cts;
    } else {
    cts_stable_cnt = 0;
    sum_cts = 0;
    continue;
    }
    if (cts_stable_cnt >= 32) {
    last_cts = (sum_cts >> 5);
    break;
    }
    }
    }
    regmap_write(ctx.regmap, IT66121_AUD_PKT_CTS0_REG, (u8)((last_cts) & 0xFF));
    regmap_write(ctx.regmap, IT66121_AUD_PKT_CTS1_REG, (u8)((last_cts >> 8) & 0xFF));
    regmap_write(ctx.regmap, IT66121_AUD_PKT_CTS2_REG, (u8)((last_cts >> 16) & 0x0F));
    }
    ret = regmap_write(ctx.regmap, 0xF8, 0xC3);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, 0xF8, 0xA5);
    if (ret)
    return ret;
    if (ctx.audio.auto_cts) {
    ret = regmap_write_bits(ctx.regmap, IT66121_PKT_CTS_CTRL_REG,
    IT66121_PKT_CTS_CTRL_SEL,
    1);
    } else {
    ret = regmap_write_bits(ctx.regmap, IT66121_PKT_CTS_CTRL_REG,
    IT66121_PKT_CTS_CTRL_SEL,
    0);
    }
    if (ret)
    return ret;
    return regmap_write(ctx.regmap, 0xF8, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn it661221_audio_output_enable(ctx: *mut it66121_ctx, enable: bool) -> c_int {
    static int it661221_audio_output_enable(struct it66121_ctx *ctx, bool enable)
    {
    int ret;
    if (enable) {
    ret = regmap_write_bits(ctx.regmap, IT66121_SW_RST_REG,
    IT66121_SW_RST_AUD | IT66121_SW_RST_AREF,
    0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_AUD_CTRL0_REG,
    IT66121_AUD_EN_I2S3 | IT66121_AUD_EN_I2S2 |
    IT66121_AUD_EN_I2S1 | IT66121_AUD_EN_I2S0,
    ctx.audio.ch_enable);
    } else {
    ret = regmap_write_bits(ctx.regmap, IT66121_AUD_CTRL0_REG,
    IT66121_AUD_EN_I2S3 | IT66121_AUD_EN_I2S2 |
    IT66121_AUD_EN_I2S1 | IT66121_AUD_EN_I2S0,
    ctx.audio.ch_enable & 0xF0);
    if (ret)
    return ret;
    ret = regmap_write_bits(ctx.regmap, IT66121_SW_RST_REG,
    IT66121_SW_RST_AUD | IT66121_SW_RST_AREF,
    IT66121_SW_RST_AUD | IT66121_SW_RST_AREF);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn it661221_audio_ch_enable(ctx: *mut it66121_ctx, enable: bool) -> c_int {
    static int it661221_audio_ch_enable(struct it66121_ctx *ctx, bool enable)
    {
    int ret;
    if (enable) {
    ret = regmap_write(ctx.regmap, IT66121_AUD_SRCVALID_FLAT_REG, 0);
    if (ret)
    return ret;
    ret = regmap_write(ctx.regmap, IT66121_AUD_CTRL0_REG, ctx.audio.ch_enable);
    } else {
    ret = regmap_write(ctx.regmap, IT66121_AUD_CTRL0_REG, ctx.audio.ch_enable & 0xF0);
    }
    return ret;
    }
    static int it66121_hdmi_audio_prepare(struct drm_bridge *bridge,
    struct drm_connector *connector,
    struct hdmi_codec_daifmt *daifmt,
    struct hdmi_codec_params *params)
    {
    u8 fs;
    u8 swl;
    int ret;
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    struct device *dev = ctx.dev;
    static u8 iec60958_chstat[5];
    let mut channels: c_uint = params.channels;
    let mut sample_rate: c_uint = params.sample_rate;
    let mut sample_width: c_uint = params.sample_width;
    mutex_lock(&ctx.lock);
    dev_dbg(dev, "%s: %u, %u, %u, %u\n", __func__,
    daifmt.fmt, sample_rate, sample_width, channels);
    switch (daifmt.fmt) {
    case HDMI_I2S:
    dev_dbg(dev, "Using HDMI I2S\n");
    break;
    default:
    dev_err(dev, "Invalid or unsupported DAI format %d\n", daifmt.fmt);
    ret = -EINVAL;
    goto out;
    }
// Set audio clock recovery (N/CTS)
    ret = regmap_write(ctx.regmap, IT66121_CLK_CTRL0_REG,
    IT66121_CLK_CTRL0_AUTO_OVER_SAMPLING |
    IT66121_CLK_CTRL0_EXT_MCLK_256FS |
    IT66121_CLK_CTRL0_AUTO_IPCLK);
    if (ret)
    goto out;
    ret = regmap_write_bits(ctx.regmap, IT66121_AUD_CTRL0_REG,
    IT66121_AUD_CTRL0_AUD_SEL, 0); // remove spdif selection
    if (ret)
    goto out;
    switch (sample_rate) {
    case 44100L:
    fs = IT66121_AUD_FS_44P1K;
    break;
    case 88200L:
    fs = IT66121_AUD_FS_88P2K;
    break;
    case 176400L:
    fs = IT66121_AUD_FS_176P4K;
    break;
    case 32000L:
    fs = IT66121_AUD_FS_32K;
    break;
    case 48000L:
    fs = IT66121_AUD_FS_48K;
    break;
    case 96000L:
    fs = IT66121_AUD_FS_96K;
    break;
    case 192000L:
    fs = IT66121_AUD_FS_192K;
    break;
    case 768000L:
    fs = IT66121_AUD_FS_768K;
    break;
    default:
    fs = IT66121_AUD_FS_48K;
    break;
    }
    ctx.audio.fs = fs;
    ret = it661221_set_ncts(ctx, fs);
    if (ret) {
    dev_err(dev, "Failed to set N/CTS: %d\n", ret);
    goto out;
    }
// Set audio format register (except audio channel enable)
    ret = it661221_set_lpcm_audio(ctx, (channels + 1) / 2, sample_width);
    if (ret) {
    dev_err(dev, "Failed to set LPCM audio: %d\n", ret);
    goto out;
    }
// Set audio channel status
    iec60958_chstat[0] = 0;
    if ((channels + 1) / 2 == 1)
    iec60958_chstat[0] |= 0x1;
    iec60958_chstat[0] &= ~(1 << 1);
    iec60958_chstat[1] = 0;
    iec60958_chstat[2] = (channels + 1) / 2;
    iec60958_chstat[2] |= (channels << 4) & 0xF0;
    iec60958_chstat[3] = fs;
    switch (sample_width) {
    case 21L:
    swl = IT66121_AUD_SWL_21BIT;
    break;
    case 24L:
    swl = IT66121_AUD_SWL_24BIT;
    break;
    case 23L:
    swl = IT66121_AUD_SWL_23BIT;
    break;
    case 22L:
    swl = IT66121_AUD_SWL_22BIT;
    break;
    case 20L:
    swl = IT66121_AUD_SWL_20BIT;
    break;
    case 17L:
    swl = IT66121_AUD_SWL_17BIT;
    break;
    case 19L:
    swl = IT66121_AUD_SWL_19BIT;
    break;
    case 18L:
    swl = IT66121_AUD_SWL_18BIT;
    break;
    case 16L:
    swl = IT66121_AUD_SWL_16BIT;
    break;
    default:
    swl = IT66121_AUD_SWL_NOT_INDICATED;
    break;
    }
    iec60958_chstat[4] = (((~fs) << 4) & 0xF0) | swl;
    ret = it661221_set_chstat(ctx, iec60958_chstat);
    if (ret) {
    dev_err(dev, "Failed to set channel status: %d\n", ret);
    goto out;
    }
// Enable audio channel enable while input clock stable (if SPDIF).
    ret = it661221_audio_ch_enable(ctx, true);
    if (ret) {
    dev_err(dev, "Failed to enable audio channel: %d\n", ret);
    goto out;
    }
    ret = regmap_write_bits(ctx.regmap, IT66121_INT_MASK1_REG,
    IT66121_INT_MASK1_AUD_OVF,
    0);
    if (ret)
    goto out;
    dev_dbg(dev, "HDMI audio enabled.\n");
    out:
    mutex_unlock(&ctx.lock);
    if (!ret)
    ret = drm_atomic_helper_connector_hdmi_update_audio_infoframe(connector,
    &params.cea);
    return ret;
    }
    static int it66121_hdmi_audio_startup(struct drm_bridge *bridge,
    struct drm_connector *connector)
    {
    int ret;
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    mutex_lock(&ctx.lock);
    ret = it661221_audio_output_enable(ctx, true);
    if (ret)
    dev_err(ctx.dev, "Failed to enable audio output: %d\n", ret);
    mutex_unlock(&ctx.lock);
    return ret;
    }
    static void it66121_hdmi_audio_shutdown(struct drm_bridge *bridge,
    struct drm_connector *connector)
    {
    int ret;
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    drm_atomic_helper_connector_hdmi_clear_audio_infoframe(connector);
    mutex_lock(&ctx.lock);
    ret = it661221_audio_output_enable(ctx, false);
    if (ret)
    dev_err(ctx.dev, "Failed to disable audio output: %d\n", ret);
    mutex_unlock(&ctx.lock);
    }
    static int it66121_hdmi_audio_mute_stream(struct drm_bridge *bridge,
    struct drm_connector *connector,
    bool enable, int direction)
    {
    int ret;
    struct it66121_ctx *ctx = container_of(bridge, struct it66121_ctx, bridge);
    dev_dbg(ctx.dev, "%s: enable=%s, direction=%d\n",
    __func__, enable ? "true" : "false", direction);
    mutex_lock(&ctx.lock);
    if (enable) {
    ret = regmap_write_bits(ctx.regmap, IT66121_AUD_SRCVALID_FLAT_REG,
    IT66121_AUD_FLAT_SRC0 | IT66121_AUD_FLAT_SRC1 |
    IT66121_AUD_FLAT_SRC2 | IT66121_AUD_FLAT_SRC3,
    IT66121_AUD_FLAT_SRC0 | IT66121_AUD_FLAT_SRC1 |
    IT66121_AUD_FLAT_SRC2 | IT66121_AUD_FLAT_SRC3);
    } else {
    ret = regmap_write_bits(ctx.regmap, IT66121_AUD_SRCVALID_FLAT_REG,
    IT66121_AUD_FLAT_SRC0 | IT66121_AUD_FLAT_SRC1 |
    IT66121_AUD_FLAT_SRC2 | IT66121_AUD_FLAT_SRC3,
    0);
    }
    mutex_unlock(&ctx.lock);
    return ret;
    }
    static const struct drm_bridge_funcs it66121_bridge_funcs = {
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .attach = it66121_bridge_attach,
    .atomic_get_output_bus_fmts = it66121_bridge_atomic_get_output_bus_fmts,
    .atomic_get_input_bus_fmts = it66121_bridge_atomic_get_input_bus_fmts,
    .atomic_enable = it66121_bridge_enable,
    .atomic_disable = it66121_bridge_disable,
    .atomic_check = it66121_bridge_check,
    .detect = it66121_bridge_detect,
    .edid_read = it66121_bridge_edid_read,
    .hpd_enable = it66121_bridge_hpd_enable,
    .hpd_disable = it66121_bridge_hpd_disable,
    .hdmi_tmds_char_rate_valid = it66121_bridge_hdmi_tmds_char_rate_valid,
    .hdmi_clear_avi_infoframe = it66121_bridge_hdmi_clear_avi_infoframe,
    .hdmi_write_avi_infoframe = it66121_bridge_hdmi_write_avi_infoframe,
    .hdmi_clear_hdmi_infoframe = it66121_bridge_hdmi_clear_hdmi_infoframe,
    .hdmi_write_hdmi_infoframe = it66121_bridge_hdmi_write_hdmi_infoframe,
    .hdmi_clear_audio_infoframe = it66121_bridge_hdmi_clear_audio_infoframe,
    .hdmi_write_audio_infoframe = it66121_bridge_hdmi_write_audio_infoframe,
    .hdmi_audio_startup = it66121_hdmi_audio_startup,
    .hdmi_audio_prepare = it66121_hdmi_audio_prepare,
    .hdmi_audio_shutdown = it66121_hdmi_audio_shutdown,
    .hdmi_audio_mute_stream = it66121_hdmi_audio_mute_stream,
    };
    static const char * const it66121_supplies[] = {
    "vcn33", "vcn18", "vrf12"
    };
    static const struct it66121_chip_info it66xx_chip_info[] = {
    {.id = ID_IT6610, .vid = 0xca00, .pid = 0x0611 },
    {.id = ID_IT66121, .vid = 0x4954, .pid = 0x0612 },
    {.id = ID_IT66122, .vid = 0x4954, .pid = 0x0622 },
    };
#[no_mangle]
unsafe extern "C" fn it66121_probe(client: *mut i2c_client) -> c_int {
    static int it66121_probe(struct i2c_client *client)
    {
    u32 revision_id, vendor_ids[2] = { 0 }, device_ids[2] = { 0 };
    struct device_node *ep;
    int ret, i;
    struct it66121_ctx *ctx;
    struct device *dev = &client.dev;
    const struct it66121_chip_info *chip_info;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(dev, "I2C check functionality failed.\n");
    return -ENXIO;
    }
    ctx = devm_drm_bridge_alloc(dev, struct it66121_ctx, bridge,
    &it66121_bridge_funcs);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ep = of_graph_get_endpoint_by_regs(dev.of_node, 0, 0);
    if (!ep)
    return -EINVAL;
    ctx.dev = dev;
    ctx.client = client;
    of_property_read_u32(ep, "bus-width", &ctx.bus_width);
    of_node_put(ep);
    if (ctx.bus_width != 12 && ctx.bus_width != 24)
    return -EINVAL;
    ep = of_graph_get_remote_node(dev.of_node, 1, -1);
    if (!ep) {
    dev_err(ctx.dev, "The endpoint is unconnected\n");
    return -EINVAL;
    }
    ctx.bridge.next_bridge = of_drm_find_and_get_bridge(ep);
    of_node_put(ep);
    if (!ctx.bridge.next_bridge) {
    dev_dbg(ctx.dev, "Next bridge not found, deferring probe\n");
    return -EPROBE_DEFER;
    }
    i2c_set_clientdata(client, ctx);
    mutex_init(&ctx.lock);
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE(it66121_supplies),
    it66121_supplies);
    if (ret) {
    dev_err(dev, "Failed to enable power supplies\n");
    return ret;
    }
    ctx.gpio_reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.gpio_reset))
    return dev_err_probe(dev, PTR_ERR(ctx.gpio_reset),
    "Failed to get reset GPIO\n");
    it66121_hw_reset(ctx);
    ctx.regmap = devm_regmap_init_i2c(client, &it66121_regmap_config);
    if (IS_ERR(ctx.regmap))
    return PTR_ERR(ctx.regmap);
    regmap_read(ctx.regmap, IT66121_VENDOR_ID0_REG, &vendor_ids[0]);
    regmap_read(ctx.regmap, IT66121_VENDOR_ID1_REG, &vendor_ids[1]);
    regmap_read(ctx.regmap, IT66121_DEVICE_ID0_REG, &device_ids[0]);
    regmap_read(ctx.regmap, IT66121_DEVICE_ID1_REG, &device_ids[1]);
// Revision is shared with DEVICE_ID1
    revision_id = FIELD_GET(IT66121_REVISION_MASK, device_ids[1]);
    device_ids[1] &= IT66121_DEVICE_ID1_MASK;
    for (i = 0; i < ARRAY_SIZE(it66xx_chip_info); i++) {
    chip_info = &it66xx_chip_info[i];
    if ((vendor_ids[1] << 8 | vendor_ids[0]) == chip_info.vid &&
    (device_ids[1] << 8 | device_ids[0]) == chip_info.pid) {
    ctx.id = chip_info.id;
    break;
    }
    }
    if (i == ARRAY_SIZE(it66xx_chip_info))
    return -ENODEV;
    ctx.bridge.of_node = dev.of_node;
    ctx.bridge.type = DRM_MODE_CONNECTOR_HDMIA;
    ctx.bridge.ops = DRM_BRIDGE_OP_DETECT | DRM_BRIDGE_OP_EDID |
    DRM_BRIDGE_OP_HDMI;
    ctx.bridge.vendor = "ITE";
    ctx.bridge.product = "IT66121";
    if (client.irq > 0) {
    ctx.bridge.ops |= DRM_BRIDGE_OP_HPD;
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    it66121_irq_threaded_handler,
    IRQF_ONESHOT, dev_name(dev),
    ctx);
    if (ret < 0) {
    dev_err(dev, "Failed to request irq %d:%d\n", client.irq, ret);
    return ret;
    }
    }
    if (of_property_present(dev.of_node, "#sound-dai-cells")) {
    ctx.bridge.ops |= DRM_BRIDGE_OP_HDMI_AUDIO;
    ctx.bridge.hdmi_audio_dev = dev;
    ctx.bridge.hdmi_audio_max_i2s_playback_channels = 8;
// of-graph not supported, phandle match only
    ctx.bridge.hdmi_audio_dai_port = -1;
    } else {
    dev_info(dev, "No \"#sound-dai-cells\", no audio\n");
    }
    drm_bridge_add(&ctx.bridge);
    dev_info(ctx.dev, "IT66121 revision %d probed\n", revision_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it66121_remove(client: *mut i2c_client) {
    static void it66121_remove(struct i2c_client *client)
    {
    struct it66121_ctx *ctx = i2c_get_clientdata(client);
    drm_bridge_remove(&ctx.bridge);
    mutex_destroy(&ctx.lock);
    }
    static const struct of_device_id it66121_dt_match[] = {
    { .compatible = "ite,it6610" },
    { .compatible = "ite,it66121" },
    { .compatible = "ite,it66122" },
    { }
    };
    MODULE_DEVICE_TABLE(of, it66121_dt_match);
    static const struct i2c_device_id it66121_id[] = {
    { .name = "it6610" },
    { .name = "it66121" },
    { .name = "it66122" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, it66121_id);
    static struct i2c_driver it66121_driver = {
    .driver = {
    .name	= "it66121",
    .of_match_table = it66121_dt_match,
    },
    .probe = it66121_probe,
    .remove = it66121_remove,
    .id_table = it66121_id,
    };
    module_i2c_driver(it66121_driver);
    MODULE_AUTHOR("Phong LE");
    MODULE_DESCRIPTION("IT66121 HDMI transmitter driver");
    MODULE_LICENSE("GPL v2");
