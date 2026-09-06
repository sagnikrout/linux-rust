//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-csid-340.c
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
// Qualcomm MSM Camera Subsystem - CSID (CSI Decoder) Module 340
//
// Copyright (c) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const CSI2_RX_CFG0_PHY_NUM_SEL_BASE_IDX: c_int = 1;

pub const CSID_CTRL_HALT_AT_FRAME_BOUNDARY: c_int = 0;
pub const CSID_CTRL_RESUME_AT_FRAME_BOUNDARY: c_int = 1;
    enum csid_iface {
    CSID_IFACE_PIX,
    CSID_IFACE_RDI0,
    CSID_IFACE_RDI1,
    CSID_IFACE_RDI2,
    };
    static enum csid_iface csid_port_iface_map[MSM_CSID_MAX_SRC_STREAMS] = {
    [0] = CSID_IFACE_RDI0,
    [1] = CSID_IFACE_RDI1,
    [2] = CSID_IFACE_RDI2,
    [3] = CSID_IFACE_PIX,
    };
#[no_mangle]
unsafe extern "C" fn __csid_configure_rx(csid: *mut csid_device, phy: *mut csid_phy_config) {
    static void __csid_configure_rx(struct csid_device *csid, struct csid_phy_config *phy)
    {
    u32 val;
    val = FIELD_PREP(CSI2_RX_CFG0_NUM_ACTIVE_LANES_MASK, phy.lane_cnt - 1);
    val |= FIELD_PREP(CSI2_RX_CFG0_DLX_INPUT_SEL_MASK, phy.lane_assign);
    val |= FIELD_PREP(CSI2_RX_CFG0_PHY_NUM_SEL_MASK,
    phy.csiphy_id + CSI2_RX_CFG0_PHY_NUM_SEL_BASE_IDX);
    writel_relaxed(val, csid.base + CSID_CSI2_RX_CFG0);
    val = CSI2_RX_CFG1_PACKET_ECC_CORRECTION_EN;
    writel_relaxed(val, csid.base + CSID_CSI2_RX_CFG1);
    }
#[no_mangle]
unsafe extern "C" fn __csid_configure_stream(csid: *mut csid_device, enable: u8, port: u8, vc: u8) {
    static void __csid_configure_stream(struct csid_device *csid, u8 enable, u8 port, u8 vc)
    {
    struct v4l2_mbus_framefmt *input_format = &csid.fmt[MSM_CSID_PAD_FIRST_SRC + port];
    const struct csid_format_info *format = csid_get_fmt_entry(csid.res.formats.formats,
    csid.res.formats.nformats,
    input_format.code);
    let mut iface: enum csid_iface = csid_port_iface_map[port];
    u8 dt_id;
    u32 val;
//
// DT_ID is a two bit bitfield that is concatenated with
// the four least significant bits of the five bit VC
// bitfield to generate an internal CID value.
//
// CSID_CFG0(port)
// DT_ID : 28:27
// VC    : 26:22
// DT    : 21:16
//
// CID   : VC 3:0 << 2 | DT_ID 1:0
//
    dt_id = port & 0x03;
    if (iface == CSID_IFACE_PIX)
    val = FIELD_PREP(CSID_CFG0_DECODE_FORMAT_MASK, format.decode_format);
    else /* RDI is raw, no decoding */
    val = CSID_CFG0_DECODE_FORMAT_NOP;
    val |= FIELD_PREP(CSID_CFG0_DT_MASK, format.data_type);
    val |= FIELD_PREP(CSID_CFG0_VC_MASK, vc);
    val |= FIELD_PREP(CSID_CFG0_DTID_MASK, dt_id);
    if (enable)
    val |= CSID_CFG0_ENABLE;
    dev_dbg(csid.camss.dev, "CSID%u: Stream %s (dt:0x%x df=0x%x port=%u vc=%u)\n",
    csid.id, enable ? "enable" : "disable", format.data_type,
    format.decode_format, port, vc);
    writel_relaxed(val, csid.base + CSID_CFG0(iface));
    writel_relaxed(enable, csid.base + CSID_CTRL(iface));
    }
#[no_mangle]
unsafe extern "C" fn csid_configure_streams(csid: *mut csid_device, enable: u8) {
    static void csid_configure_streams(struct csid_device *csid, u8 enable)
    {
    int i;
    __csid_configure_rx(csid, &csid.phy);
    for (i = 0; i < MSM_CSID_MAX_SRC_STREAMS; i++) {
    if (csid.phy.en_vc & BIT(i))
    __csid_configure_stream(csid, !!enable, i, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn csid_reset(csid: *mut csid_device) -> c_int {
    static int csid_reset(struct csid_device *csid)
    {
    unsigned long time;
    writel_relaxed(CSID_IRQ_MASK_RST_DONE, csid.base + CSID_IRQ_MASK);
    writel_relaxed(CSID_IRQ_MASK_RST_DONE, csid.base + CSID_IRQ_CLEAR);
    writel_relaxed(CSID_IRQ_CMD_CLEAR, csid.base + CSID_IRQ_CMD);
    reinit_completion(&csid.reset_complete);
// Reset with registers preserved
    writel(CSID_RST_IRQ | CSID_RST_IFE_CLK | CSID_RST_PHY_CLK | CSID_RST_CSID_CLK,
    csid.base + CSID_RST_STROBES);
    time = wait_for_completion_timeout(&csid.reset_complete,
    msecs_to_jiffies(CSID_RESET_TIMEOUT_MS));
    if (!time) {
    dev_err(csid.camss.dev, "CSID%u: reset timeout\n", csid.id);
    return -EIO;
    }
    dev_dbg(csid.camss.dev, "CSID%u: reset done\n", csid.id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csid_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t csid_isr(int irq, void *dev)
    {
    struct csid_device *csid = dev;
    u32 val;
    val = readl_relaxed(csid.base + CSID_IRQ_STATUS);
    writel_relaxed(val, csid.base + CSID_IRQ_CLEAR);
    writel_relaxed(CSID_IRQ_CMD_CLEAR, csid.base + CSID_IRQ_CMD);
    if (val & CSID_IRQ_MASK_RST_DONE)
    complete(&csid.reset_complete);
    else
    dev_warn_ratelimited(csid.camss.dev, "Spurious CSID interrupt\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn csid_configure_testgen_pattern(csid: *mut csid_device, val: i32) -> c_int {
    static int csid_configure_testgen_pattern(struct csid_device *csid, s32 val)
    {
    return -EOPNOTSUPP; /* Not part of CSID */
    }
    static void csid_subdev_init(struct csid_device *csid) {}
    const struct csid_hw_ops csid_ops_340 = {
    .configure_testgen_pattern = csid_configure_testgen_pattern,
    .configure_stream = csid_configure_streams,
    .hw_version = csid_hw_version,
    .isr = csid_isr,
    .reset = csid_reset,
    .src_pad_code = csid_src_pad_code,
    .subdev_init = csid_subdev_init,
    };
