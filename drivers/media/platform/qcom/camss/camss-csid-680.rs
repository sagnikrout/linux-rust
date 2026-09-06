//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-csid-680.c
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
// Qualcomm MSM Camera Subsystem - CSID (CSI Decoder) Module
//
// Copyright (C) 2020-2025 Linaro Ltd.
//

pub const CSID_RESET_CMD: c_uint = 0x10;

pub const CSID_IRQ_CMD: c_uint = 0x14;

pub const CSID_REG_UPDATE_CMD: c_uint = 0x18;

pub const CSID_TOP_IRQ_STATUS: c_uint = 0x7c;
pub const CSID_TOP_IRQ_MASK: c_uint = 0x80;
pub const CSID_TOP_IRQ_CLEAR: c_uint = 0x84;

pub const CSID_BUF_DONE_IRQ_STATUS: c_uint = 0x8c;

pub const CSID_BUF_DONE_IRQ_MASK: c_uint = 0x90;
pub const CSID_BUF_DONE_IRQ_CLEAR: c_uint = 0x94;
pub const CSID_CSI2_RX_IRQ_STATUS: c_uint = 0x9c;
pub const CSID_CSI2_RX_IRQ_MASK: c_uint = 0xa0;
pub const CSID_CSI2_RX_IRQ_CLEAR: c_uint = 0xa4;
pub const CSID_RESET_CFG: c_uint = 0xc;

pub const CSID_CSI2_RX_CFG0: c_uint = 0x200;
pub const CSI2_RX_CFG0_NUM_ACTIVE_LANES: c_int = 0;
pub const CSI2_RX_CFG0_DL0_INPUT_SEL: c_int = 4;
pub const CSI2_RX_CFG0_DL1_INPUT_SEL: c_int = 8;
pub const CSI2_RX_CFG0_DL2_INPUT_SEL: c_int = 12;
pub const CSI2_RX_CFG0_DL3_INPUT_SEL: c_int = 16;
pub const CSI2_RX_CFG0_PHY_NUM_SEL: c_int = 20;
pub const CSI2_RX_CFG0_PHY_SEL_BASE_IDX: c_int = 1;
pub const CSI2_RX_CFG0_PHY_TYPE_SEL: c_int = 24;

pub const CSID_CSI2_RX_CFG1: c_uint = 0x204;

pub const CSID_CSI2_RX_CAPTURE_CTRL: c_uint = 0x208;

pub const CSID_CSI2_RX_TOTAL_PKTS_RCVD: c_uint = 0x240;
pub const CSID_CSI2_RX_STATS_ECC: c_uint = 0x244;
pub const CSID_CSI2_RX_CRC_ERRORS: c_uint = 0x248;

pub const RDI_CFG0_DECODE_FORMAT: c_int = 12;
pub const RDI_CFG0_DATA_TYPE: c_int = 16;
pub const RDI_CFG0_VIRTUAL_CHANNEL: c_int = 22;
pub const RDI_CFG0_DT_ID: c_int = 27;

pub const CSID_RDI_CTRL_HALT_CMD_HALT_AT_FRAME_BOUNDARY: c_int = 0;
pub const CSID_RDI_CTRL_HALT_CMD_RESUME_AT_FRAME_BOUNDARY: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn reg_update_rdi(csid: *mut csid_device, n: c_int) -> c_int {
    static inline int reg_update_rdi(struct csid_device *csid, int n)
    {
    return BIT(4 + n) + BIT(20 + n);
    }
#[no_mangle]
unsafe extern "C" fn csid_reg_update(csid: *mut csid_device, port_id: c_int) {
    static void csid_reg_update(struct csid_device *csid, int port_id)
    {
    csid.reg_update |= reg_update_rdi(csid, port_id);
    writel(csid.reg_update, csid.base + CSID_REG_UPDATE_CMD);
    }
    static inline void csid_reg_update_clear(struct csid_device *csid,
    int port_id)
    {
    csid.reg_update &= ~reg_update_rdi(csid, port_id);
    writel(csid.reg_update, csid.base + CSID_REG_UPDATE_CMD);
    }
    static void __csid_configure_rx(struct csid_device *csid,
    struct csid_phy_config *phy, int vc)
    {
    u32 val;
    struct camss *camss;
    camss = csid.camss;
    val = (phy.lane_cnt - 1) << CSI2_RX_CFG0_NUM_ACTIVE_LANES;
    val |= phy.lane_assign << CSI2_RX_CFG0_DL0_INPUT_SEL;
    if (camss.tpg && csid.tpg_linked &&
    camss.tpg[phy.csiphy_id].testgen.mode != TPG_PAYLOAD_MODE_DISABLED) {
    val |= FIELD_PREP(CSI2_RX_CFG0_TPG_MUX_SEL, phy.csiphy_id + 1);
    val |= CSI2_RX_CFG0_TPG_MUX_EN;
    } else {
    val |= (phy.csiphy_id + CSI2_RX_CFG0_PHY_SEL_BASE_IDX)
    << CSI2_RX_CFG0_PHY_NUM_SEL;
    }
    writel(val, csid.base + CSID_CSI2_RX_CFG0);
    val = CSI2_RX_CFG1_PACKET_ECC_CORRECTION_EN;
    if (vc > 3)
    val |= CSI2_RX_CFG1_VC_MODE;
    writel(val, csid.base + CSID_CSI2_RX_CFG1);
    }
#[no_mangle]
unsafe extern "C" fn __csid_ctrl_rdi(csid: *mut csid_device, enable: c_int, rdi: u8) {
    static void __csid_ctrl_rdi(struct csid_device *csid, int enable, u8 rdi)
    {
    u32 val;
    if (enable)
    val = CSID_RDI_CTRL_HALT_CMD_RESUME_AT_FRAME_BOUNDARY;
    else
    val = CSID_RDI_CTRL_HALT_CMD_HALT_AT_FRAME_BOUNDARY;
    writel(val, csid.base + CSID_RDI_CTRL(rdi));
    }
#[no_mangle]
unsafe extern "C" fn __csid_configure_top(csid: *mut csid_device) {
    static void __csid_configure_top(struct csid_device *csid)
    {
    u32 val;
    val = CSID_TOP_IO_PATH_CFG0_OUTPUT_IFE_EN | CSID_TOP_IO_PATH_CFG0_INTERNAL_CSID;
    writel(val, csid.camss.csid_wrapper_base +
    CSID_TOP_IO_PATH_CFG0(csid.id));
    }
#[no_mangle]
unsafe extern "C" fn __csid_configure_rdi_stream(csid: *mut csid_device, enable: u8, port: u8, vc: u8) {
    static void __csid_configure_rdi_stream(struct csid_device *csid, u8 enable, u8 port, u8 vc)
    {
    struct v4l2_mbus_framefmt *input_format = &csid.fmt[MSM_CSID_PAD_FIRST_SRC + port];
    const struct csid_format_info *format = csid_get_fmt_entry(csid.res.formats.formats,
    csid.res.formats.nformats,
    input_format.code);
    let mut lane_cnt: u8 = csid.phy.lane_cnt;
    u8 dt_id;
    u32 val;
    if (!lane_cnt)
    lane_cnt = 4;
    val = 0;
    writel(val, csid.base + CSID_RDI_FRM_DROP_PERIOD(port));
//
// DT_ID is a two bit bitfield that is concatenated with
// the four least significant bits of the five bit VC
// bitfield to generate an internal CID value.
//
// CSID_RDI_CFG0(port)
// DT_ID : 28:27
// VC    : 26:22
// DT    : 21:16
//
// CID   : VC 3:0 << 2 | DT_ID 1:0
//
    dt_id = port & 0x03;
// note: for non-RDI path, this should be format->decode_format
    val |= DECODE_FORMAT_PAYLOAD_ONLY << RDI_CFG0_DECODE_FORMAT;
    val |= format.data_type << RDI_CFG0_DATA_TYPE;
    val |= vc << RDI_CFG0_VIRTUAL_CHANNEL;
    val |= dt_id << RDI_CFG0_DT_ID;
    writel(val, csid.base + CSID_RDI_CFG0(port));
    val = RDI_CFG1_TIMESTAMP_STB_FRAME;
    val |= RDI_CFG1_BYTE_CNTR_EN;
    val |= RDI_CFG1_TIMESTAMP_EN;
    val |= RDI_CFG1_DROP_H_EN;
    val |= RDI_CFG1_DROP_V_EN;
    val |= RDI_CFG1_CROP_H_EN;
    val |= RDI_CFG1_CROP_V_EN;
    val |= RDI_CFG1_PACKING_MIPI;
    writel(val, csid.base + CSID_RDI_CFG1(port));
    val = 0;
    writel(val, csid.base + CSID_RDI_IRQ_SUBSAMPLE_PERIOD(port));
    val = 1;
    writel(val, csid.base + CSID_RDI_IRQ_SUBSAMPLE_PATTERN(port));
    val = 0;
    writel(val, csid.base + CSID_RDI_CTRL(port));
    val = readl(csid.base + CSID_RDI_CFG0(port));
    if (enable)
    val |= RDI_CFG0_ENABLE;
    else
    val &= ~RDI_CFG0_ENABLE;
    writel(val, csid.base + CSID_RDI_CFG0(port));
    }
#[no_mangle]
unsafe extern "C" fn csid_configure_stream(csid: *mut csid_device, enable: u8) {
    static void csid_configure_stream(struct csid_device *csid, u8 enable)
    {
    int i;
    __csid_configure_top(csid);
// Loop through all enabled ports and configure a stream for each
    for (i = 0; i < MSM_CSID_MAX_SRC_STREAMS; i++) {
    if (csid.phy.en_vc & BIT(i)) {
    __csid_configure_rdi_stream(csid, enable, i, 0);
    __csid_configure_rx(csid, &csid.phy, 0);
    __csid_ctrl_rdi(csid, enable, i);
    }
    }
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
    u32 val;
    int i;
    reinit_completion(&csid.reset_complete);
    writel(CSID_IRQ_CMD_CLEAR, csid.base + CSID_IRQ_CMD);
// preserve registers
    val = CSID_RESET_CFG_MODE_IMMEDIATE | CSID_RESET_CFG_LOCATION_COMPLETE;
    writel(val, csid.base + CSID_RESET_CFG);
    val = CSID_RESET_CMD_HW_RESET | CSID_RESET_CMD_SW_RESET;
    writel(val, csid.base + CSID_RESET_CMD);
    time = wait_for_completion_timeout(&csid.reset_complete,
    msecs_to_jiffies(CSID_RESET_TIMEOUT_MS));
    if (!time) {
    dev_err(csid.camss.dev, "CSID reset timeout\n");
    return -EIO;
    }
    for (i = 0; i < MSM_CSID_MAX_SRC_STREAMS; i++) {
// Enable RUP done for the client port
    writel(CSID_CSI2_RDIN_RUP_DONE, csid.base + CSID_CSI2_RDIN_IRQ_MASK(i));
    }
// Clear RDI status
    writel(~0u, csid.base + CSID_BUF_DONE_IRQ_CLEAR);
// Enable BUF_DONE bit for all write-master client ports
    writel(~0u, csid.base + CSID_BUF_DONE_IRQ_MASK);
// Unmask all TOP interrupts
    writel(~0u, csid.base + CSID_TOP_IRQ_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csid_rup_complete(csid: *mut csid_device, rdi: c_int) {
    static void csid_rup_complete(struct csid_device *csid, int rdi)
    {
    csid_reg_update_clear(csid, rdi);
    }
//
// csid_isr - CSID module interrupt service routine
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
    u32 buf_done_val, val, val_top;
    int i;
// Latch and clear TOP status
    val_top = readl(csid.base + CSID_TOP_IRQ_STATUS);
    writel(val_top, csid.base + CSID_TOP_IRQ_CLEAR);
// Latch and clear CSID_CSI2 status
    val = readl(csid.base + CSID_CSI2_RX_IRQ_STATUS);
    writel(val, csid.base + CSID_CSI2_RX_IRQ_CLEAR);
// Latch and clear top level BUF_DONE status
    buf_done_val = readl(csid.base + CSID_BUF_DONE_IRQ_STATUS);
    writel(buf_done_val, csid.base + CSID_BUF_DONE_IRQ_CLEAR);
// Process state for each RDI channel
    for (i = 0; i < MSM_CSID_MAX_SRC_STREAMS; i++) {
    val = readl(csid.base + CSID_CSI2_RDIN_IRQ_STATUS(i));
    if (val)
    writel(val, csid.base + CSID_CSI2_RDIN_IRQ_CLEAR(i));
    if (val & CSID_CSI2_RDIN_RUP_DONE)
    csid_rup_complete(csid, i);
    if (buf_done_val & BIT(BUF_DONE_IRQ_STATUS_RDI_OFFSET + i))
    camss_buf_done(csid.camss, csid.id, i);
    }
// Issue clear command
    writel(CSID_IRQ_CMD_CLEAR, csid.base + CSID_IRQ_CMD);
// Reset complete
    if (val_top & CSID_TOP_IRQ_RESET)
    complete(&csid.reset_complete);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn csid_subdev_reg_update(csid: *mut csid_device, port_id: c_int, is_clear: bool) {
    static void csid_subdev_reg_update(struct csid_device *csid, int port_id, bool is_clear)
    {
    if (is_clear)
    csid_reg_update_clear(csid, port_id);
    else
    csid_reg_update(csid, port_id);
    }
    static void csid_subdev_init(struct csid_device *csid) {}
    const struct csid_hw_ops csid_ops_680 = {
    .configure_testgen_pattern = core::ptr::null_mut(),
    .configure_stream = csid_configure_stream,
    .hw_version = csid_hw_version,
    .isr = csid_isr,
    .reset = csid_reset,
    .src_pad_code = csid_src_pad_code,
    .subdev_init = csid_subdev_init,
    .reg_update = csid_subdev_reg_update,
    };
