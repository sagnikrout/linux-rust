//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-tpg-gen1.c
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
// Qualcomm MSM Camera Subsystem - TPG (Test Pattern Generator) Module
//
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// TPG global registers
pub const TPG_HW_VERSION: c_uint = 0x0;

    (((u32)(gen) << 28) | ((u32)(rev) << 16) | (u32)(step))

pub const TPG_HW_STATUS: c_uint = 0x4;
pub const TPG_CTRL: c_uint = 0x64;

pub const TPG_CLEAR: c_uint = 0x1F4;
// TPG VC-based registers

// TPG DT-based registers

// v2.0.0: USER[19:4], ENC[23:20]

// v2.1.0: USER[27:4], ENC[31:28]

pub const PERCENT_BASE: c_int = 100;
// Default user-specified payload for TPG test generator.
// Keep consistent with CSID TPG default: 0xBE.
//
pub const TPG_USER_SPECIFIED_PAYLOAD_DEFAULT: c_uint = 0xBE;
pub const TPG_LFSR_SEED_DEFAULT: c_uint = 0x12345678;

    FIELD_PREP(TPG_VC_n_COLOR_BARS_CFG_ROTATE_PERIOD, 0xA)
    static const char * const testgen_payload_modes[] = {
    [TPG_PAYLOAD_MODE_DISABLED]		= "Disabled",
    [TPG_PAYLOAD_MODE_INCREMENTING]		= "Incrementing",
    [TPG_PAYLOAD_MODE_ALTERNATING_55_AA]	= "Alternating 0x55/0xAA",
    [TPG_PAYLOAD_MODE_RANDOM]		= "Pseudo-random Data",
    [TPG_PAYLOAD_MODE_USER_SPECIFIED]	= "User Specified",
    [TPG_PAYLOAD_MODE_COLOR_BARS]		= "Color bars",
    };
#[no_mangle]
unsafe extern "C" fn tpg_stream_on(tpg: *mut tpg_device) -> c_int {
    static int tpg_stream_on(struct tpg_device *tpg)
    {
    struct tpg_testgen_config *tg = &tpg.testgen;
    struct v4l2_mbus_framefmt *input_format;
    const struct tpg_format_info *format;
    u8 payload_mode = (tg.mode > TPG_PAYLOAD_MODE_DISABLED) ?
    tg.mode - 1 : 0;
    let mut lane_cnt: u8 = tpg.res.lane_cnt;
    u8 vc, dt, last_vc = 0;
    u32 val;
    for (vc = 0; vc <= MSM_TPG_ACTIVE_VC; vc++) {
    last_vc = vc;
    input_format = &tpg.fmt;
    format = tpg_get_fmt_entry(tpg.res.formats.formats,
    tpg.res.formats.nformats,
    input_format.code);
    if (IS_ERR(format))
    return -EINVAL;
// VC configuration
    val = FIELD_PREP(TPG_VC_n_CFG0_NUM_ACTIVE_DT, MSM_TPG_ACTIVE_DT) |
    FIELD_PREP(TPG_VC_n_CFG0_NUM_FRAMES, 0);
    writel(val, tpg.base + TPG_VC_n_CFG0(vc));
    writel(TPG_LFSR_SEED_DEFAULT, tpg.base + TPG_VC_n_LSFR_SEED(vc));
    val = DIV_ROUND_UP(input_format.width * format.bpp * TPG_HBI_PCT_DEFAULT,
    BITS_PER_BYTE * lane_cnt * PERCENT_BASE);
    writel(val, tpg.base + TPG_VC_n_HBI_CFG(vc));
    val = input_format.height * TPG_VBI_PCT_DEFAULT / PERCENT_BASE;
    writel(val, tpg.base + TPG_VC_n_VBI_CFG(vc));
    writel(TPG_COLOR_BARS_CFG_STANDARD, tpg.base + TPG_VC_n_COLOR_BARS_CFG(vc));
// DT configuration
    for (dt = 0; dt <= MSM_TPG_ACTIVE_DT; dt++) {
    val = FIELD_PREP(TPG_VC_m_DT_n_CFG_0_FRAME_HEIGHT,
    input_format.height & 0xffff) |
    FIELD_PREP(TPG_VC_m_DT_n_CFG_0_FRAME_WIDTH,
    input_format.width & 0xffff);
    writel(val, tpg.base + TPG_VC_m_DT_n_CFG_0(vc, dt));
    val = FIELD_PREP(TPG_VC_m_DT_n_CFG_1_DATA_TYPE, format.data_type);
    writel(val, tpg.base + TPG_VC_m_DT_n_CFG_1(vc, dt));
    if (tpg.hw_version == TPG_HW_VER_2_0_0) {
    val = FIELD_PREP(TPG_VC_m_DT_n_CFG_2_PAYLOAD_MODE, payload_mode) |
    FIELD_PREP(TPG_V2_0_0_VC_m_DT_n_CFG_2_USER_SPECIFIED_PAYLOAD,
    TPG_USER_SPECIFIED_PAYLOAD_DEFAULT) |
    FIELD_PREP(TPG_V2_0_0_VC_m_DT_n_CFG_2_ENCODE_FORMAT,
    format.encode_format);
    } else if (tpg.hw_version >= TPG_HW_VER_2_1_0) {
    val = FIELD_PREP(TPG_VC_m_DT_n_CFG_2_PAYLOAD_MODE, payload_mode) |
    FIELD_PREP(TPG_V2_1_0_VC_m_DT_n_CFG_2_USER_SPECIFIED_PAYLOAD,
    TPG_USER_SPECIFIED_PAYLOAD_DEFAULT) |
    FIELD_PREP(TPG_V2_1_0_VC_m_DT_n_CFG_2_ENCODE_FORMAT,
    format.encode_format);
    }
    writel(val, tpg.base + TPG_VC_m_DT_n_CFG_2(vc, dt));
    }
    }
// Global TPG control
    val = FIELD_PREP(TPG_CTRL_TEST_EN, 1) |
    FIELD_PREP(TPG_CTRL_NUM_ACTIVE_LANES, lane_cnt - 1) |
    FIELD_PREP(TPG_CTRL_NUM_ACTIVE_VC, last_vc);
    writel(val, tpg.base + TPG_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpg_reset(tpg: *mut tpg_device) -> c_int {
    static int tpg_reset(struct tpg_device *tpg)
    {
    writel(0, tpg.base + TPG_CTRL);
    writel(1, tpg.base + TPG_CLEAR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpg_stream_off(tpg: *mut tpg_device) {
    static void tpg_stream_off(struct tpg_device *tpg)
    {
    tpg_reset(tpg);
    }
#[no_mangle]
unsafe extern "C" fn tpg_configure_stream(tpg: *mut tpg_device, enable: u8) -> c_int {
    static int tpg_configure_stream(struct tpg_device *tpg, u8 enable)
    {
    if (enable)
    return tpg_stream_on(tpg);
    tpg_stream_off(tpg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpg_configure_testgen_pattern(tpg: *mut tpg_device, val: i32) -> c_int {
    static int tpg_configure_testgen_pattern(struct tpg_device *tpg, s32 val)
    {
    if (val >= 0 && val <= TPG_PAYLOAD_MODE_COLOR_BARS)
    tpg.testgen.mode = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpg_hw_version(tpg: *mut tpg_device) -> u32 {
    static u32 tpg_hw_version(struct tpg_device *tpg)
    {
    let mut hw_version: u32 = readl(tpg.base + TPG_HW_VERSION);
    tpg.hw_version = hw_version;
    dev_dbg(tpg.camss.dev, "tpg HW Version = %u.%u.%u\n",
    (u32)FIELD_GET(HW_VERSION_GENERATION, hw_version),
    (u32)FIELD_GET(HW_VERSION_REVISION, hw_version),
    (u32)FIELD_GET(HW_VERSION_STEPPING, hw_version));
    return hw_version;
    }
#[no_mangle]
unsafe extern "C" fn tpg_subdev_init(tpg: *mut tpg_device) {
    static void tpg_subdev_init(struct tpg_device *tpg)
    {
    tpg.testgen.modes = testgen_payload_modes;
    tpg.testgen.nmodes = TPG_PAYLOAD_MODE_NUM_SUPPORTED_GEN1;
    }
    const struct tpg_hw_ops tpg_ops_gen1 = {
    .configure_stream = tpg_configure_stream,
    .configure_testgen_pattern = tpg_configure_testgen_pattern,
    .hw_version = tpg_hw_version,
    .reset = tpg_reset,
    .subdev_init = tpg_subdev_init,
    };
