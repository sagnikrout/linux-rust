//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-csid-4-7.c
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
// camss-csid-4-7.c
//
// Qualcomm MSM Camera Subsystem - CSID (CSI Decoder) Module
//
// Copyright (C) 2020 Linaro Ltd.
//

pub const CAMSS_CSID_CORE_CTRL_0: c_uint = 0x004;
pub const CAMSS_CSID_CORE_CTRL_1: c_uint = 0x008;
pub const CAMSS_CSID_RST_CMD: c_uint = 0x010;

pub const CAMSS_CSID_CID_n_CFG_DECODE_FORMAT_SHIFT: c_int = 4;

pub const CAMSS_CSID_IRQ_CLEAR_CMD: c_uint = 0x064;
pub const CAMSS_CSID_IRQ_MASK: c_uint = 0x068;
pub const CAMSS_CSID_IRQ_STATUS: c_uint = 0x06c;
pub const CAMSS_CSID_TG_CTRL: c_uint = 0x0a8;
pub const CAMSS_CSID_TG_CTRL_DISABLE: c_uint = 0xa06436;
pub const CAMSS_CSID_TG_CTRL_ENABLE: c_uint = 0xa06437;
pub const CAMSS_CSID_TG_VC_CFG: c_uint = 0x0ac;
pub const CAMSS_CSID_TG_VC_CFG_H_BLANKING: c_uint = 0x3ff;
pub const CAMSS_CSID_TG_VC_CFG_V_BLANKING: c_uint = 0x7f;

#[no_mangle]
unsafe extern "C" fn csid_configure_stream(csid: *mut csid_device, enable: u8) {
    static void csid_configure_stream(struct csid_device *csid, u8 enable)
    {
    struct csid_testgen_config *tg = &csid.testgen;
    let mut sink_code: u32 = csid.fmt[MSM_CSID_PAD_SINK].code;
    let mut src_code: u32 = csid.fmt[MSM_CSID_PAD_SRC].code;
    u32 val;
    if (enable) {
    struct v4l2_mbus_framefmt *input_format;
    const struct csid_format_info *format;
    u8 vc = 0; /* Virtual Channel 0 */
    u8 cid = vc * 4; /* id of Virtual Channel and Data Type set */
    u8 dt_shift;
    if (tg.enabled) {
// Config Test Generator
    u32 num_bytes_per_line, num_lines;
    input_format = &csid.fmt[MSM_CSID_PAD_SRC];
    format = csid_get_fmt_entry(csid.res.formats.formats,
    csid.res.formats.nformats,
    input_format.code);
    num_bytes_per_line = input_format.width * format.bpp * format.spp / 8;
    num_lines = input_format.height;
// 31:24 V blank, 23:13 H blank, 3:2 num of active DT
// 1:0 VC
    val = ((CAMSS_CSID_TG_VC_CFG_V_BLANKING & 0xff) << 24) |
    ((CAMSS_CSID_TG_VC_CFG_H_BLANKING & 0x7ff) << 13);
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_VC_CFG);
// 28:16 bytes per lines, 12:0 num of lines
    val = ((num_bytes_per_line & 0x1fff) << 16) |
    (num_lines & 0x1fff);
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_DT_n_CGG_0(0));
// 5:0 data type
    val = format.data_type;
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_DT_n_CGG_1(0));
// 2:0 output test pattern
    val = tg.mode - 1;
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_DT_n_CGG_2(0));
    } else {
    struct csid_phy_config *phy = &csid.phy;
    input_format = &csid.fmt[MSM_CSID_PAD_SINK];
    format = csid_get_fmt_entry(csid.res.formats.formats,
    csid.res.formats.nformats,
    input_format.code);
    val = phy.lane_cnt - 1;
    val |= phy.lane_assign << 4;
    writel_relaxed(val, csid.base + CAMSS_CSID_CORE_CTRL_0);
    val = phy.csiphy_id << 17;
    val |= 0x9;
    writel_relaxed(val, csid.base + CAMSS_CSID_CORE_CTRL_1);
    }
// Config LUT
    dt_shift = (cid % 4) * 8;
    val = readl_relaxed(csid.base + CAMSS_CSID_CID_LUT_VC_n(vc));
    val &= ~(0xff << dt_shift);
    val |= format.data_type << dt_shift;
    writel_relaxed(val, csid.base + CAMSS_CSID_CID_LUT_VC_n(vc));
    val = CAMSS_CSID_CID_n_CFG_ISPIF_EN;
    val |= CAMSS_CSID_CID_n_CFG_RDI_EN;
    val |= format.decode_format << CAMSS_CSID_CID_n_CFG_DECODE_FORMAT_SHIFT;
    val |= CAMSS_CSID_CID_n_CFG_RDI_MODE_RAW_DUMP;
    if ((sink_code == MEDIA_BUS_FMT_SBGGR10_1X10 &&
    src_code == MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE) ||
    (sink_code == MEDIA_BUS_FMT_Y10_1X10 &&
    src_code == MEDIA_BUS_FMT_Y10_2X8_PADHI_LE)) {
    val |= CAMSS_CSID_CID_n_CFG_RDI_MODE_PLAIN_PACKING;
    val |= CAMSS_CSID_CID_n_CFG_PLAIN_FORMAT_16;
    val |= CAMSS_CSID_CID_n_CFG_PLAIN_ALIGNMENT_LSB;
    }
    writel_relaxed(val, csid.base + CAMSS_CSID_CID_n_CFG(cid));
    if (tg.enabled) {
    val = CAMSS_CSID_TG_CTRL_ENABLE;
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_CTRL);
    }
    } else {
    if (tg.enabled) {
    val = CAMSS_CSID_TG_CTRL_DISABLE;
    writel_relaxed(val, csid.base + CAMSS_CSID_TG_CTRL);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn csid_configure_testgen_pattern(csid: *mut csid_device, val: i32) -> c_int {
    static int csid_configure_testgen_pattern(struct csid_device *csid, s32 val)
    {
    if (val > 0 && val <= csid.testgen.nmodes)
    csid.testgen.mode = val;
    return 0;
    }
//
// isr - CSID module interrupt service routine
// @irq: Interrupt line
// @dev: CSID device
//
// Return IRQ_HANDLED on success
//
#[no_mangle]
unsafe extern "C" fn csid_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t csid_isr(int irq, void *dev)
    {
    struct csid_device *csid = dev;
    u32 value;
    value = readl_relaxed(csid.base + CAMSS_CSID_IRQ_STATUS);
    writel_relaxed(value, csid.base + CAMSS_CSID_IRQ_CLEAR_CMD);
    if ((value >> 11) & 0x1)
    complete(&csid.reset_complete);
    return IRQ_HANDLED;
    }
//
// csid_reset - Trigger reset on CSID module and wait to complete
// @csid: CSID device
//
// Return 0 on success or a negative error code otherwise
//
#[no_mangle]
unsafe extern "C" fn csid_reset(csid: *mut csid_device) -> c_int {
    static int csid_reset(struct csid_device *csid)
    {
    unsigned long time;
    reinit_completion(&csid.reset_complete);
    writel_relaxed(0x7fff, csid.base + CAMSS_CSID_RST_CMD);
    time = wait_for_completion_timeout(&csid.reset_complete,
    msecs_to_jiffies(CSID_RESET_TIMEOUT_MS));
    if (!time) {
    dev_err(csid.camss.dev, "CSID reset timeout\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csid_subdev_init(csid: *mut csid_device) {
    static void csid_subdev_init(struct csid_device *csid)
    {
    csid.testgen.modes = csid_testgen_modes;
    csid.testgen.nmodes = CSID_PAYLOAD_MODE_NUM_SUPPORTED_GEN1;
    }
    const struct csid_hw_ops csid_ops_4_7 = {
    .configure_stream = csid_configure_stream,
    .configure_testgen_pattern = csid_configure_testgen_pattern,
    .hw_version = csid_hw_version,
    .isr = csid_isr,
    .reset = csid_reset,
    .src_pad_code = csid_src_pad_code,
    .subdev_init = csid_subdev_init,
    };
