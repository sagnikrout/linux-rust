//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/amlogic/c3/mipi-csi2/c3-mipi-csi2.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//

// C3 CSI-2 submodule definition
    enum {
    SUBMD_APHY,
    SUBMD_DPHY,
    SUBMD_HOST,
    };

pub const CSI2_SUBMD_SHIFT: c_int = 16;

pub const MIPI_CSI2_CLOCK_NUM_MAX: c_int = 3;

// C3 CSI-2 APHY register

// C3 CSI-2 DPHY register

// C3 CSI-2 HOST register

pub const C3_MIPI_CSI2_MAX_WIDTH: c_int = 2888;
pub const C3_MIPI_CSI2_MIN_WIDTH: c_int = 160;
pub const C3_MIPI_CSI2_MAX_HEIGHT: c_int = 2240;
pub const C3_MIPI_CSI2_MIN_HEIGHT: c_int = 120;
pub const C3_MIPI_CSI2_DEFAULT_WIDTH: c_int = 1920;
pub const C3_MIPI_CSI2_DEFAULT_HEIGHT: c_int = 1080;

// C3 CSI-2 pad list
    enum {
    C3_MIPI_CSI2_PAD_SINK,
    C3_MIPI_CSI2_PAD_SRC,
    C3_MIPI_CSI2_PAD_MAX
    };
//
// struct c3_csi_info - MIPI CSI2 information
//
// @clocks: array of MIPI CSI2 clock names
// @clock_num: actual clock number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_csi_info {
    pub clocks: [*mut c_char; MIPI_CSI2_CLOCK_NUM_MAX],
    pub clock_num: u32,
}

//
// struct c3_csi_device - MIPI CSI2 platform device
//
// @dev: pointer to the struct device
// @aphy: MIPI CSI2 aphy register address
// @dphy: MIPI CSI2 dphy register address
// @host: MIPI CSI2 host register address
// @clks: array of MIPI CSI2 clocks
// @sd: MIPI CSI2 sub-device
// @pads: MIPI CSI2 sub-device pads
// @notifier: notifier to register on the v4l2-async API
// @src_pad: source sub-device pad
// @bus: MIPI CSI2 bus information
// @info: version-specific MIPI CSI2 information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_csi_device {
    pub dev: *mut device,
    pub aphy: *mut void __iomem,
    pub dphy: *mut void __iomem,
    pub host: *mut void __iomem,
    pub clks: [clk_bulk_data; MIPI_CSI2_CLOCK_NUM_MAX],
    pub sd: v4l2_subdev,
    pub pads: [media_pad; C3_MIPI_CSI2_PAD_MAX],
    pub notifier: v4l2_async_notifier,
    pub src_pad: *mut media_pad,
    pub bus: v4l2_mbus_config_mipi_csi2,
    pub info: *const c3_csi_info,
}

    static const u32 c3_mipi_csi_formats[] = {
    MEDIA_BUS_FMT_SBGGR10_1X10,
    MEDIA_BUS_FMT_SGBRG10_1X10,
    MEDIA_BUS_FMT_SGRBG10_1X10,
    MEDIA_BUS_FMT_SRGGB10_1X10,
    MEDIA_BUS_FMT_SBGGR12_1X12,
    MEDIA_BUS_FMT_SGBRG12_1X12,
    MEDIA_BUS_FMT_SGRBG12_1X12,
    MEDIA_BUS_FMT_SRGGB12_1X12,
    };
// Hardware configuration
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_write(csi: *mut c3_csi_device, reg: u32, val: u32) {
    static void c3_mipi_csi_write(struct c3_csi_device *csi, u32 reg, u32 val)
    {
    void __iomem *addr;
    switch (CSI2_SUBMD(reg)) {
    case SUBMD_APHY:
    addr = csi.aphy + CSI2_REG_ADDR(reg);
    break;
    case SUBMD_DPHY:
    addr = csi.dphy + CSI2_REG_ADDR(reg);
    break;
    case SUBMD_HOST:
    addr = csi.host + CSI2_REG_ADDR(reg);
    break;
    default:
    dev_err(csi.dev, "Invalid sub-module: %lu\n", CSI2_SUBMD(reg));
    return;
    }
    writel(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_cfg_aphy(csi: *mut c3_csi_device) {
    static void c3_mipi_csi_cfg_aphy(struct c3_csi_device *csi)
    {
    c3_mipi_csi_write(csi, CSI_PHY_CNTL0,
    CSI_PHY_CNTL0_HS_LP_BIAS_EN |
    CSI_PHY_CNTL0_HS_RX_TRIM_11 |
    CSI_PHY_CNTL0_LP_LOW_VTH_2 |
    CSI_PHY_CNTL0_LP_HIGH_VTH_4 |
    CSI_PHY_CNTL0_DATA_LANE0_HS_DIG_EN |
    CSI_PHY_CNTL0_DATA_LANE1_HS_DIG_EN |
    CSI_PHY_CNTL0_CLK0_LANE_HS_DIG_EN |
    CSI_PHY_CNTL0_DATA_LANE2_HS_DIG_EN |
    CSI_PHY_CNTL0_DATA_LANE3_HS_DIG_EN);
    c3_mipi_csi_write(csi, CSI_PHY_CNTL1,
    CSI_PHY_CNTL1_HS_EQ_CAP_SMALL |
    CSI_PHY_CNTL1_HS_EQ_RES_MED |
    CSI_PHY_CNTL1_CLK_CHN_EQ_MAX_GAIN |
    CSI_PHY_CNTL1_DATA_CHN_EQ_MAX_GAIN |
    CSI_PHY_CNTL1_COM_BG_EN |
    CSI_PHY_CNTL1_HS_SYNC_EN);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_cfg_dphy(csi: *mut c3_csi_device, rate: i64) {
    static void c3_mipi_csi_cfg_dphy(struct c3_csi_device *csi, s64 rate)
    {
    u32 val;
    u32 settle;
// Calculate the high speed settle
    val = DIV_ROUND_UP_ULL(1000000000, rate);
    settle = (16 * val + 230) / 10;
    c3_mipi_csi_write(csi, MIPI_PHY_CLK_LANE_CTRL,
    MIPI_PHY_CLK_LANE_CTRL_HS_RX_EN |
    MIPI_PHY_CLK_LANE_CTRL_END_EN |
    MIPI_PHY_CLK_LANE_CTRL_LPEN_DIS |
    MIPI_PHY_CLK_LANE_CTRL_TCLK_ZERO_EN |
    MIPI_PHY_CLK_LANE_CTRL_TCLK_ZERO_HS_8);
    c3_mipi_csi_write(csi, MIPI_PHY_TCLK_MISS, MIPI_PHY_TCLK_MISS_CYCLES_9);
    c3_mipi_csi_write(csi, MIPI_PHY_TCLK_SETTLE,
    MIPI_PHY_TCLK_SETTLE_CYCLES_31);
    c3_mipi_csi_write(csi, MIPI_PHY_THS_EXIT, MIPI_PHY_THS_EXIT_CYCLES_8);
    c3_mipi_csi_write(csi, MIPI_PHY_THS_SKIP, MIPI_PHY_THS_SKIP_CYCLES_10);
    c3_mipi_csi_write(csi, MIPI_PHY_THS_SETTLE, settle);
    c3_mipi_csi_write(csi, MIPI_PHY_TINIT, MIPI_PHY_TINIT_CYCLES_20000);
    c3_mipi_csi_write(csi, MIPI_PHY_TMBIAS, MIPI_PHY_TMBIAS_CYCLES_256);
    c3_mipi_csi_write(csi, MIPI_PHY_TULPS_C, MIPI_PHY_TULPS_C_CYCLES_4096);
    c3_mipi_csi_write(csi, MIPI_PHY_TULPS_S, MIPI_PHY_TULPS_S_CYCLES_256);
    c3_mipi_csi_write(csi, MIPI_PHY_TLP_EN_W, MIPI_PHY_TLP_EN_W_CYCLES_12);
    c3_mipi_csi_write(csi, MIPI_PHY_TLPOK, MIPI_PHY_TLPOK_CYCLES_256);
    c3_mipi_csi_write(csi, MIPI_PHY_TWD_INIT,
    MIPI_PHY_TWD_INIT_DOG_0X400000);
    c3_mipi_csi_write(csi, MIPI_PHY_TWD_HS, MIPI_PHY_TWD_HS_DOG_0X400000);
    c3_mipi_csi_write(csi, MIPI_PHY_DATA_LANE_CTRL1,
    MIPI_PHY_DATA_LANE_CTRL1_INSERT_ERRESC |
    MIPI_PHY_DATA_LANE_CTRL1_HS_SYNC_CHK_EN |
    MIPI_PHY_DATA_LANE_CTRL1_PIPE_ALL_EN |
    MIPI_PHY_DATA_LANE_CTRL1_PIPE_DELAY_3);
// Set the order of lanes
    c3_mipi_csi_write(csi, MIPI_PHY_MUX_CTRL0,
    MIPI_PHY_MUX_CTRL0_SFEN3_SRC_LANE3 |
    MIPI_PHY_MUX_CTRL0_SFEN2_SRC_LANE2 |
    MIPI_PHY_MUX_CTRL0_SFEN1_SRC_LANE1 |
    MIPI_PHY_MUX_CTRL0_SFEN0_SRC_LANE0);
    c3_mipi_csi_write(csi, MIPI_PHY_MUX_CTRL1,
    MIPI_PHY_MUX_CTRL1_LANE3_SRC_SFEN3 |
    MIPI_PHY_MUX_CTRL1_LANE2_SRC_SFEN2 |
    MIPI_PHY_MUX_CTRL1_LANE1_SRC_SFEN1 |
    MIPI_PHY_MUX_CTRL1_LANE0_SRC_SFEN0);
// Enable digital data and clock lanes
    c3_mipi_csi_write(csi, MIPI_PHY_CTRL,
    MIPI_PHY_CTRL_DATA_LANE0_EN |
    MIPI_PHY_CTRL_DATA_LANE1_EN |
    MIPI_PHY_CTRL_DATA_LANE2_EN |
    MIPI_PHY_CTRL_DATA_LANE3_EN |
    MIPI_PHY_CTRL_CLOCK_LANE_EN);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_cfg_host(csi: *mut c3_csi_device) {
    static void c3_mipi_csi_cfg_host(struct c3_csi_device *csi)
    {
// Reset CSI-2 controller output
    c3_mipi_csi_write(csi, CSI2_HOST_CSI2_RESETN,
    CSI2_HOST_CSI2_RESETN_ACTIVE);
    c3_mipi_csi_write(csi, CSI2_HOST_CSI2_RESETN,
    CSI2_HOST_CSI2_RESETN_EXIT);
// Set data lane number
    c3_mipi_csi_write(csi, CSI2_HOST_N_LANES, csi.bus.num_data_lanes - 1);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_start_stream(csi: *mut c3_csi_device) -> c_int {
    static int c3_mipi_csi_start_stream(struct c3_csi_device *csi)
    {
    s64 link_freq;
    s64 lane_rate;
    link_freq = v4l2_get_link_freq(csi.src_pad, 0, 0);
    if (link_freq < 0) {
    dev_err(csi.dev,
    "Unable to obtain link frequency: %lld\n", link_freq);
    return link_freq;
    }
    lane_rate = link_freq * 2;
    if (lane_rate > 1500000000) {
    dev_err(csi.dev, "Invalid lane rate: %lld\n", lane_rate);
    return -EINVAL;
    }
    c3_mipi_csi_cfg_aphy(csi);
    c3_mipi_csi_cfg_dphy(csi, lane_rate);
    c3_mipi_csi_cfg_host(csi);
    return 0;
    }
    static int c3_mipi_csi_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct c3_csi_device *csi = v4l2_get_subdevdata(sd);
    struct media_pad *sink_pad;
    struct v4l2_subdev *src_sd;
    int ret;
    sink_pad = &csi.pads[C3_MIPI_CSI2_PAD_SINK];
    csi.src_pad = media_pad_remote_pad_unique(sink_pad);
    if (IS_ERR(csi.src_pad)) {
    dev_dbg(csi.dev, "Failed to get source pad for MIPI CSI-2\n");
    return -EPIPE;
    }
    src_sd = media_entity_to_v4l2_subdev(csi.src_pad.entity);
    pm_runtime_resume_and_get(csi.dev);
    c3_mipi_csi_start_stream(csi);
    ret = v4l2_subdev_enable_streams(src_sd, csi.src_pad.index, BIT(0));
    if (ret) {
    pm_runtime_put(csi.dev);
    return ret;
    }
    return 0;
    }
    static int c3_mipi_csi_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct c3_csi_device *csi = v4l2_get_subdevdata(sd);
    struct v4l2_subdev *src_sd;
    if (csi.src_pad) {
    src_sd = media_entity_to_v4l2_subdev(csi.src_pad.entity);
    v4l2_subdev_disable_streams(src_sd, csi.src_pad.index,
    BIT(0));
    }
    csi.src_pad = core::ptr::null_mut();
    pm_runtime_put(csi.dev);
    return 0;
    }
    static int c3_mipi_csi_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    struct v4l2_mbus_framefmt *fmt;
    switch (code.pad) {
    case C3_MIPI_CSI2_PAD_SINK:
    if (code.index >= ARRAY_SIZE(c3_mipi_csi_formats))
    return -EINVAL;
    code.code = c3_mipi_csi_formats[code.index];
    break;
    case C3_MIPI_CSI2_PAD_SRC:
    if (code.index)
    return -EINVAL;
    fmt = v4l2_subdev_state_get_format(state, code.pad);
    code.code = fmt.code;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int c3_mipi_csi_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *fmt;
    unsigned int i;
    if (format.pad != C3_MIPI_CSI2_PAD_SINK)
    return v4l2_subdev_get_fmt(sd, state, format);
    fmt = v4l2_subdev_state_get_format(state, format.pad);
    for (i = 0; i < ARRAY_SIZE(c3_mipi_csi_formats); i++) {
    if (format.format.code == c3_mipi_csi_formats[i]) {
    fmt.code = c3_mipi_csi_formats[i];
    break;
    }
    }
    if (i == ARRAY_SIZE(c3_mipi_csi_formats))
    fmt.code = c3_mipi_csi_formats[0];
    fmt.width = clamp_t(u32, format.format.width,
    C3_MIPI_CSI2_MIN_WIDTH, C3_MIPI_CSI2_MAX_WIDTH);
    fmt.height = clamp_t(u32, format.format.height,
    C3_MIPI_CSI2_MIN_HEIGHT, C3_MIPI_CSI2_MAX_HEIGHT);
    fmt.colorspace = V4L2_COLORSPACE_RAW;
    fmt.xfer_func = V4L2_XFER_FUNC_NONE;
    fmt.ycbcr_enc = V4L2_YCBCR_ENC_601;
    fmt.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    format.format = *fmt;
// Synchronize the format to source pad
    fmt = v4l2_subdev_state_get_format(state, C3_MIPI_CSI2_PAD_SRC);
// fmt = format->format;
    return 0;
    }
    static int c3_mipi_csi_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct v4l2_mbus_framefmt *sink_fmt;
    struct v4l2_mbus_framefmt *src_fmt;
    sink_fmt = v4l2_subdev_state_get_format(state, C3_MIPI_CSI2_PAD_SINK);
    src_fmt = v4l2_subdev_state_get_format(state, C3_MIPI_CSI2_PAD_SRC);
    sink_fmt.width = C3_MIPI_CSI2_DEFAULT_WIDTH;
    sink_fmt.height = C3_MIPI_CSI2_DEFAULT_HEIGHT;
    sink_fmt.field = V4L2_FIELD_NONE;
    sink_fmt.code = C3_MIPI_CSI2_DEFAULT_FMT;
    sink_fmt.colorspace = V4L2_COLORSPACE_RAW;
    sink_fmt.xfer_func = V4L2_XFER_FUNC_NONE;
    sink_fmt.ycbcr_enc = V4L2_YCBCR_ENC_601;
    sink_fmt.quantization = V4L2_QUANTIZATION_FULL_RANGE;
// src_fmt = *sink_fmt;
    return 0;
    }
    static const struct v4l2_subdev_pad_ops c3_mipi_csi_pad_ops = {
    .enum_mbus_code = c3_mipi_csi_enum_mbus_code,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = c3_mipi_csi_set_fmt,
    .enable_streams = c3_mipi_csi_enable_streams,
    .disable_streams = c3_mipi_csi_disable_streams,
    };
    static const struct v4l2_subdev_ops c3_mipi_csi_subdev_ops = {
    .pad = &c3_mipi_csi_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops c3_mipi_csi_internal_ops = {
    .init_state = c3_mipi_csi_init_state,
    };
// Media entity operations
    static const struct media_entity_operations c3_mipi_csi_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
// PM runtime
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_runtime_suspend(dev: *mut device) -> c_int {
    static int c3_mipi_csi_runtime_suspend(struct device *dev)
    {
    struct c3_csi_device *csi = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(csi.info.clock_num, csi.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_runtime_resume(dev: *mut device) -> c_int {
    static int c3_mipi_csi_runtime_resume(struct device *dev)
    {
    struct c3_csi_device *csi = dev_get_drvdata(dev);
    return clk_bulk_prepare_enable(csi.info.clock_num, csi.clks);
    }
    static const struct dev_pm_ops c3_mipi_csi_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    RUNTIME_PM_OPS(c3_mipi_csi_runtime_suspend,
    c3_mipi_csi_runtime_resume, core::ptr::null_mut())
    };
// Probe/remove & platform driver
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_subdev_init(csi: *mut c3_csi_device) -> c_int {
    static int c3_mipi_csi_subdev_init(struct c3_csi_device *csi)
    {
    struct v4l2_subdev *sd = &csi.sd;
    int ret;
    v4l2_subdev_init(sd, &c3_mipi_csi_subdev_ops);
    sd.owner = THIS_MODULE;
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    sd.internal_ops = &c3_mipi_csi_internal_ops;
    snprintf(sd.name, sizeof(sd.name), "%s", MIPI_CSI2_SUBDEV_NAME);
    sd.entity.function = MEDIA_ENT_F_VID_IF_BRIDGE;
    sd.entity.ops = &c3_mipi_csi_entity_ops;
    sd.dev = csi.dev;
    v4l2_set_subdevdata(sd, csi);
    csi.pads[C3_MIPI_CSI2_PAD_SINK].flags = MEDIA_PAD_FL_SINK;
    csi.pads[C3_MIPI_CSI2_PAD_SRC].flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&sd.entity, C3_MIPI_CSI2_PAD_MAX,
    csi.pads);
    if (ret)
    return ret;
    ret = v4l2_subdev_init_finalize(sd);
    if (ret) {
    media_entity_cleanup(&sd.entity);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_subdev_deinit(csi: *mut c3_csi_device) {
    static void c3_mipi_csi_subdev_deinit(struct c3_csi_device *csi)
    {
    v4l2_subdev_cleanup(&csi.sd);
    media_entity_cleanup(&csi.sd.entity);
    }
// Subdev notifier register
    static int c3_mipi_csi_notify_bound(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *sd,
    struct v4l2_async_connection *asc)
    {
    struct c3_csi_device *csi = v4l2_get_subdevdata(notifier.sd);
    struct media_pad *sink = &csi.sd.entity.pads[C3_MIPI_CSI2_PAD_SINK];
    return v4l2_create_fwnode_links_to_pad(sd, sink, MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    }
    static const struct v4l2_async_notifier_operations c3_mipi_csi_notify_ops = {
    .bound = c3_mipi_csi_notify_bound,
    };
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_async_register(csi: *mut c3_csi_device) -> c_int {
    static int c3_mipi_csi_async_register(struct c3_csi_device *csi)
    {
    struct v4l2_fwnode_endpoint vep = {
    .bus_type = V4L2_MBUS_CSI2_DPHY,
    };
    struct v4l2_async_connection *asc;
    struct fwnode_handle *ep;
    int ret;
    v4l2_async_subdev_nf_init(&csi.notifier, &csi.sd);
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(csi.dev), 0, 0,
    FWNODE_GRAPH_ENDPOINT_NEXT);
    if (!ep)
    return -ENOTCONN;
    ret = v4l2_fwnode_endpoint_parse(ep, &vep);
    if (ret)
    goto err_put_handle;
    csi.bus = vep.bus.mipi_csi2;
    asc = v4l2_async_nf_add_fwnode_remote(&csi.notifier, ep,
    struct v4l2_async_connection);
    if (IS_ERR(asc)) {
    ret = PTR_ERR(asc);
    goto err_put_handle;
    }
    csi.notifier.ops = &c3_mipi_csi_notify_ops;
    ret = v4l2_async_nf_register(&csi.notifier);
    if (ret)
    goto err_cleanup_nf;
    ret = v4l2_async_register_subdev(&csi.sd);
    if (ret)
    goto err_unregister_nf;
    fwnode_handle_put(ep);
    return 0;
    err_unregister_nf:
    v4l2_async_nf_unregister(&csi.notifier);
    err_cleanup_nf:
    v4l2_async_nf_cleanup(&csi.notifier);
    err_put_handle:
    fwnode_handle_put(ep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_async_unregister(csi: *mut c3_csi_device) {
    static void c3_mipi_csi_async_unregister(struct c3_csi_device *csi)
    {
    v4l2_async_unregister_subdev(&csi.sd);
    v4l2_async_nf_unregister(&csi.notifier);
    v4l2_async_nf_cleanup(&csi.notifier);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_ioremap_resource(csi: *mut c3_csi_device) -> c_int {
    static int c3_mipi_csi_ioremap_resource(struct c3_csi_device *csi)
    {
    struct device *dev = csi.dev;
    struct platform_device *pdev = to_platform_device(dev);
    csi.aphy = devm_platform_ioremap_resource_byname(pdev, "aphy");
    if (IS_ERR(csi.aphy))
    return PTR_ERR(csi.aphy);
    csi.dphy = devm_platform_ioremap_resource_byname(pdev, "dphy");
    if (IS_ERR(csi.dphy))
    return PTR_ERR(csi.dphy);
    csi.host = devm_platform_ioremap_resource_byname(pdev, "host");
    if (IS_ERR(csi.host))
    return PTR_ERR(csi.host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_get_clocks(csi: *mut c3_csi_device) -> c_int {
    static int c3_mipi_csi_get_clocks(struct c3_csi_device *csi)
    {
    const struct c3_csi_info *info = csi.info;
    for (unsigned int i = 0; i < info.clock_num; i++)
    csi.clks[i].id = info.clocks[i];
    return devm_clk_bulk_get(csi.dev, info.clock_num, csi.clks);
    }
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_probe(pdev: *mut platform_device) -> c_int {
    static int c3_mipi_csi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct c3_csi_device *csi;
    int ret;
    csi = devm_kzalloc(dev, sizeof(*csi), GFP_KERNEL);
    if (!csi)
    return -ENOMEM;
    csi.info = of_device_get_match_data(dev);
    csi.dev = dev;
    ret = c3_mipi_csi_ioremap_resource(csi);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to ioremap resource\n");
    ret = c3_mipi_csi_get_clocks(csi);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get clocks\n");
    platform_set_drvdata(pdev, csi);
    pm_runtime_enable(dev);
    ret = c3_mipi_csi_subdev_init(csi);
    if (ret)
    goto err_disable_runtime_pm;
    ret = c3_mipi_csi_async_register(csi);
    if (ret)
    goto err_deinit_subdev;
    return 0;
    err_deinit_subdev:
    c3_mipi_csi_subdev_deinit(csi);
    err_disable_runtime_pm:
    pm_runtime_disable(dev);
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn c3_mipi_csi_remove(pdev: *mut platform_device) {
    static void c3_mipi_csi_remove(struct platform_device *pdev)
    {
    struct c3_csi_device *csi = platform_get_drvdata(pdev);
    c3_mipi_csi_async_unregister(csi);
    c3_mipi_csi_subdev_deinit(csi);
    pm_runtime_disable(&pdev.dev);
    };
    static const struct c3_csi_info c3_mipi_csi_info = {
    .clocks = {"vapb", "phy0"},
    .clock_num = 2
    };
    static const struct of_device_id c3_mipi_csi_of_match[] = {
    {
    .compatible = "amlogic,c3-mipi-csi2",
    .data = &c3_mipi_csi_info,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, c3_mipi_csi_of_match);
    static struct platform_driver c3_mipi_csi_driver = {
    .probe = c3_mipi_csi_probe,
    .remove = c3_mipi_csi_remove,
    .driver = {
    .name = "c3-mipi-csi2",
    .of_match_table = c3_mipi_csi_of_match,
    .pm = pm_ptr(&c3_mipi_csi_pm_ops),
    },
    };
    module_platform_driver(c3_mipi_csi_driver);
    MODULE_AUTHOR("Keke Li <keke.li@amlogic.com>");
    MODULE_DESCRIPTION("Amlogic C3 MIPI CSI-2 receiver");
    MODULE_LICENSE("GPL");
