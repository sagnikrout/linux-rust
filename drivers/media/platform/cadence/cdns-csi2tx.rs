//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/cadence/cdns-csi2tx.c
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
// Driver for Cadence MIPI-CSI2 TX Controller
//
// Copyright (C) 2017-2019 Cadence Design Systems Inc.
//

pub const CSI2TX_DEVICE_CONFIG_REG: c_uint = 0x00;

pub const CSI2TX_CONFIG_REG: c_uint = 0x20;

pub const CSI2TX_DPHY_CFG_REG: c_uint = 0x28;

pub const CSI2TX_DPHY_CLK_WAKEUP_REG: c_uint = 0x2c;

// CSI2TX V2 Registers
pub const CSI2TX_V2_DPHY_CFG_REG: c_uint = 0x28;

pub const CSI2TX_LANES_MAX: c_int = 4;
pub const CSI2TX_STREAMS_MAX: c_int = 4;
    enum csi2tx_pads {
    CSI2TX_PAD_SOURCE,
    CSI2TX_PAD_SINK_STREAM0,
    CSI2TX_PAD_SINK_STREAM1,
    CSI2TX_PAD_SINK_STREAM2,
    CSI2TX_PAD_SINK_STREAM3,
    CSI2TX_PAD_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi2tx_fmt {
    pub mbus: u32,
    pub dt: u32,
    pub bpp: u32,
}

    struct csi2tx_priv;
// CSI2TX Variant Operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi2tx_vops {
    pub csi2tx): *mut *mut void (dphy_setup)(struct csi2tx_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi2tx_priv {
    pub dev: *mut device,
    pub count: c_uint,
//
// Used to prevent race conditions between multiple,
// concurrent calls to start and stop.
//
    pub lock: mutex,
    pub base: *mut void __iomem,
    pub vops: *mut csi2tx_vops,
    pub esc_clk: *mut clk,
    pub p_clk: *mut clk,
    pub pixel_clk: [*mut clk; CSI2TX_STREAMS_MAX],
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; CSI2TX_PAD_MAX],
    pub pad_fmts: [v4l2_mbus_framefmt; CSI2TX_PAD_MAX],
    pub has_internal_dphy: bool,
    pub lanes: [u8; CSI2TX_LANES_MAX],
    pub num_lanes: c_uint,
    pub max_lanes: c_uint,
    pub max_streams: c_uint,
}

    static const struct csi2tx_fmt csi2tx_formats[] = {
    {
    .mbus	= MEDIA_BUS_FMT_UYVY8_1X16,
    .bpp	= 2,
    .dt	= MIPI_CSI2_DT_YUV422_8B,
    },
    {
    .mbus	= MEDIA_BUS_FMT_RGB888_1X24,
    .bpp	= 3,
    .dt	= MIPI_CSI2_DT_RGB888,
    },
    };
    static const struct v4l2_mbus_framefmt fmt_default = {
    .width		= 1280,
    .height		= 720,
    .code		= MEDIA_BUS_FMT_RGB888_1X24,
    .field		= V4L2_FIELD_NONE,
    .colorspace	= V4L2_COLORSPACE_DEFAULT,
    };
    static inline
    struct csi2tx_priv *v4l2_subdev_to_csi2tx(struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct csi2tx_priv, subdev);
    }
    static const struct csi2tx_fmt *csi2tx_get_fmt_from_mbus(u32 mbus)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(csi2tx_formats); i++)
    if (csi2tx_formats[i].mbus == mbus)
    return &csi2tx_formats[i];
    return core::ptr::null_mut();
    }
    static int csi2tx_enum_mbus_code(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index >= ARRAY_SIZE(csi2tx_formats))
    return -EINVAL;
    code.code = csi2tx_formats[code.index].mbus;
    return 0;
    }
    static struct v4l2_mbus_framefmt *
    __csi2tx_get_pad_format(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct csi2tx_priv *csi2tx = v4l2_subdev_to_csi2tx(subdev);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY)
    return v4l2_subdev_state_get_format(sd_state, fmt.pad);
    return &csi2tx.pad_fmts[fmt.pad];
    }
    static int csi2tx_get_pad_format(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    const struct v4l2_mbus_framefmt *format;
// Multiplexed pad?
    if (fmt.pad == CSI2TX_PAD_SOURCE)
    return -EINVAL;
    format = __csi2tx_get_pad_format(subdev, sd_state, fmt);
    if (!format)
    return -EINVAL;
    fmt.format = *format;
    return 0;
    }
    static int csi2tx_set_pad_format(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    const struct v4l2_mbus_framefmt *src_format = &fmt.format;
    struct v4l2_mbus_framefmt *dst_format;
// Multiplexed pad?
    if (fmt.pad == CSI2TX_PAD_SOURCE)
    return -EINVAL;
    if (!csi2tx_get_fmt_from_mbus(fmt.format.code))
    src_format = &fmt_default;
    dst_format = __csi2tx_get_pad_format(subdev, sd_state, fmt);
    if (!dst_format)
    return -EINVAL;
// dst_format = *src_format;
    return 0;
    }
    static const struct v4l2_subdev_pad_ops csi2tx_pad_ops = {
    .enum_mbus_code	= csi2tx_enum_mbus_code,
    .get_fmt	= csi2tx_get_pad_format,
    .set_fmt	= csi2tx_set_pad_format,
    };
// Set Wake Up value in the D-PHY
#[no_mangle]
unsafe extern "C" fn csi2tx_dphy_set_wakeup(csi2tx: *mut csi2tx_priv) {
    static void csi2tx_dphy_set_wakeup(struct csi2tx_priv *csi2tx)
    {
    writel(CSI2TX_DPHY_CLK_WAKEUP_ULPS_CYCLES(32),
    csi2tx.base + CSI2TX_DPHY_CLK_WAKEUP_REG);
    }
//
// Finishes the D-PHY initialization
// reg dphy cfg value to be used
//
#[no_mangle]
unsafe extern "C" fn csi2tx_dphy_init_finish(csi2tx: *mut csi2tx_priv, reg: u32) {
    static void csi2tx_dphy_init_finish(struct csi2tx_priv *csi2tx, u32 reg)
    {
    unsigned int i;
    udelay(10);
// Enable our (clock and data) lanes
    reg |= CSI2TX_DPHY_CFG_CLK_ENABLE;
    for (i = 0; i < csi2tx.num_lanes; i++)
    reg |= CSI2TX_DPHY_CFG_LANE_ENABLE(csi2tx.lanes[i] - 1);
    writel(reg, csi2tx.base + CSI2TX_DPHY_CFG_REG);
    udelay(10);
// Switch to HS mode
    reg &= ~CSI2TX_DPHY_CFG_MODE_MASK;
    writel(reg | CSI2TX_DPHY_CFG_MODE_HS,
    csi2tx.base + CSI2TX_DPHY_CFG_REG);
    }
// Configures D-PHY in CSIv1.3
#[no_mangle]
unsafe extern "C" fn csi2tx_dphy_setup(csi2tx: *mut csi2tx_priv) {
    static void csi2tx_dphy_setup(struct csi2tx_priv *csi2tx)
    {
    u32 reg;
    unsigned int i;
    csi2tx_dphy_set_wakeup(csi2tx);
// Put our lanes (clock and data) out of reset
    reg = CSI2TX_DPHY_CFG_CLK_RESET | CSI2TX_DPHY_CFG_MODE_LPDT;
    for (i = 0; i < csi2tx.num_lanes; i++)
    reg |= CSI2TX_DPHY_CFG_LANE_RESET(csi2tx.lanes[i] - 1);
    writel(reg, csi2tx.base + CSI2TX_DPHY_CFG_REG);
    csi2tx_dphy_init_finish(csi2tx, reg);
    }
// Configures D-PHY in CSIv2
#[no_mangle]
unsafe extern "C" fn csi2tx_v2_dphy_setup(csi2tx: *mut csi2tx_priv) {
    static void csi2tx_v2_dphy_setup(struct csi2tx_priv *csi2tx)
    {
    u32 reg;
    csi2tx_dphy_set_wakeup(csi2tx);
// Put our lanes (clock and data) out of reset
    reg = CSI2TX_V2_DPHY_CFG_RESET | CSI2TX_V2_DPHY_CFG_MODE_LPDT;
    writel(reg, csi2tx.base + CSI2TX_V2_DPHY_CFG_REG);
    csi2tx_dphy_init_finish(csi2tx, reg);
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_reset(csi2tx: *mut csi2tx_priv) {
    static void csi2tx_reset(struct csi2tx_priv *csi2tx)
    {
    writel(CSI2TX_CONFIG_SRST_REQ, csi2tx.base + CSI2TX_CONFIG_REG);
    udelay(10);
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_start(csi2tx: *mut csi2tx_priv) -> c_int {
    static int csi2tx_start(struct csi2tx_priv *csi2tx)
    {
    struct media_entity *entity = &csi2tx.subdev.entity;
    struct media_link *link;
    unsigned int i;
    csi2tx_reset(csi2tx);
    writel(CSI2TX_CONFIG_CFG_REQ, csi2tx.base + CSI2TX_CONFIG_REG);
    udelay(10);
    if (csi2tx.vops && csi2tx.vops.dphy_setup) {
    csi2tx.vops.dphy_setup(csi2tx);
    udelay(10);
    }
//
// Create a static mapping between the CSI virtual channels
// and the input streams.
//
// This should be enhanced, but v4l2 lacks the support for
// changing that mapping dynamically at the moment.
//
// We're protected from the userspace setting up links at the
// same time by the upper layer having called
// media_pipeline_start().
//
    list_for_each_entry(link, &entity.links, list) {
    struct v4l2_mbus_framefmt *mfmt;
    const struct csi2tx_fmt *fmt;
    unsigned int stream;
    let mut pad_idx: c_int = -1;
// Only consider our enabled input pads
    for (i = CSI2TX_PAD_SINK_STREAM0; i < CSI2TX_PAD_MAX; i++) {
    struct media_pad *pad = &csi2tx.pads[i];
    if ((pad == link.sink) &&
    (link.flags & MEDIA_LNK_FL_ENABLED)) {
    pad_idx = i;
    break;
    }
    }
    if (pad_idx < 0)
    continue;
    mfmt = &csi2tx.pad_fmts[pad_idx];
    fmt = csi2tx_get_fmt_from_mbus(mfmt.code);
    if (!fmt)
    continue;
    stream = pad_idx - CSI2TX_PAD_SINK_STREAM0;
//
// We use the stream ID there, but it's wrong.
//
// A stream could very well send a data type that is
// not equal to its stream ID. We need to find a
// proper way to address it.
//
    writel(CSI2TX_DT_CFG_DT(fmt.dt),
    csi2tx.base + CSI2TX_DT_CFG_REG(stream));
    writel(CSI2TX_DT_FORMAT_BYTES_PER_LINE(mfmt.width * fmt.bpp) |
    CSI2TX_DT_FORMAT_MAX_LINE_NUM(mfmt.height + 1),
    csi2tx.base + CSI2TX_DT_FORMAT_REG(stream));
//
// TODO: This needs to be calculated based on the
// output CSI2 clock rate.
//
    writel(CSI2TX_STREAM_IF_CFG_FILL_LEVEL(4),
    csi2tx.base + CSI2TX_STREAM_IF_CFG_REG(stream));
    }
// Disable the configuration mode
    writel(0, csi2tx.base + CSI2TX_CONFIG_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_stop(csi2tx: *mut csi2tx_priv) {
    static void csi2tx_stop(struct csi2tx_priv *csi2tx)
    {
    writel(CSI2TX_CONFIG_CFG_REQ | CSI2TX_CONFIG_SRST_REQ,
    csi2tx.base + CSI2TX_CONFIG_REG);
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int csi2tx_s_stream(struct v4l2_subdev *subdev, int enable)
    {
    struct csi2tx_priv *csi2tx = v4l2_subdev_to_csi2tx(subdev);
    let mut ret: c_int = 0;
    mutex_lock(&csi2tx.lock);
    if (enable) {
//
// If we're not the first users, there's no need to
// enable the whole controller.
//
    if (!csi2tx.count) {
    ret = csi2tx_start(csi2tx);
    if (ret)
    goto out;
    }
    csi2tx.count++;
    } else {
    csi2tx.count--;
//
// Let the last user turn off the lights.
//
    if (!csi2tx.count)
    csi2tx_stop(csi2tx);
    }
    out:
    mutex_unlock(&csi2tx.lock);
    return ret;
    }
    static const struct v4l2_subdev_video_ops csi2tx_video_ops = {
    .s_stream	= csi2tx_s_stream,
    };
    static const struct v4l2_subdev_ops csi2tx_subdev_ops = {
    .pad		= &csi2tx_pad_ops,
    .video		= &csi2tx_video_ops,
    };
    static int csi2tx_get_resources(struct csi2tx_priv *csi2tx,
    struct platform_device *pdev)
    {
    unsigned int i;
    u32 dev_cfg;
    int ret;
    csi2tx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(csi2tx.base))
    return PTR_ERR(csi2tx.base);
    csi2tx.p_clk = devm_clk_get(&pdev.dev, "p_clk");
    if (IS_ERR(csi2tx.p_clk)) {
    dev_err(&pdev.dev, "Couldn't get p_clk\n");
    return PTR_ERR(csi2tx.p_clk);
    }
    csi2tx.esc_clk = devm_clk_get(&pdev.dev, "esc_clk");
    if (IS_ERR(csi2tx.esc_clk)) {
    dev_err(&pdev.dev, "Couldn't get the esc_clk\n");
    return PTR_ERR(csi2tx.esc_clk);
    }
    ret = clk_prepare_enable(csi2tx.p_clk);
    if (ret) {
    dev_err(&pdev.dev, "Couldn't prepare and enable p_clk\n");
    return ret;
    }
    dev_cfg = readl(csi2tx.base + CSI2TX_DEVICE_CONFIG_REG);
    clk_disable_unprepare(csi2tx.p_clk);
    csi2tx.max_lanes = dev_cfg & CSI2TX_DEVICE_CONFIG_LANES_MASK;
    if (csi2tx.max_lanes > CSI2TX_LANES_MAX) {
    dev_err(&pdev.dev, "Invalid number of lanes: %u\n",
    csi2tx.max_lanes);
    return -EINVAL;
    }
    csi2tx.max_streams = (dev_cfg & CSI2TX_DEVICE_CONFIG_STREAMS_MASK) >> 4;
    if (csi2tx.max_streams > CSI2TX_STREAMS_MAX) {
    dev_err(&pdev.dev, "Invalid number of streams: %u\n",
    csi2tx.max_streams);
    return -EINVAL;
    }
    csi2tx.has_internal_dphy = !!(dev_cfg & CSI2TX_DEVICE_CONFIG_HAS_DPHY);
    for (i = 0; i < csi2tx.max_streams; i++) {
    char clk_name[23];
    snprintf(clk_name, sizeof(clk_name), "pixel_if%u_clk", i);
    csi2tx.pixel_clk[i] = devm_clk_get(&pdev.dev, clk_name);
    if (IS_ERR(csi2tx.pixel_clk[i])) {
    dev_err(&pdev.dev, "Couldn't get clock %s\n",
    clk_name);
    return PTR_ERR(csi2tx.pixel_clk[i]);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_check_lanes(csi2tx: *mut csi2tx_priv) -> c_int {
    static int csi2tx_check_lanes(struct csi2tx_priv *csi2tx)
    {
    let mut v4l2_ep: v4l2_fwnode_endpoint = { .bus_type = 0 };
    struct device_node *ep;
    int ret, i;
    ep = of_graph_get_endpoint_by_regs(csi2tx.dev.of_node, 0, 0);
    if (!ep)
    return -EINVAL;
    ret = v4l2_fwnode_endpoint_parse(of_fwnode_handle(ep), &v4l2_ep);
    if (ret) {
    dev_err(csi2tx.dev, "Could not parse v4l2 endpoint\n");
    goto out;
    }
    if (v4l2_ep.bus_type != V4L2_MBUS_CSI2_DPHY) {
    dev_err(csi2tx.dev, "Unsupported media bus type: 0x%x\n",
    v4l2_ep.bus_type);
    ret = -EINVAL;
    goto out;
    }
    csi2tx.num_lanes = v4l2_ep.bus.mipi_csi2.num_data_lanes;
    if (csi2tx.num_lanes > csi2tx.max_lanes) {
    dev_err(csi2tx.dev,
    "Current configuration uses more lanes than supported\n");
    ret = -EINVAL;
    goto out;
    }
    for (i = 0; i < csi2tx.num_lanes; i++) {
    if (v4l2_ep.bus.mipi_csi2.data_lanes[i] < 1) {
    dev_err(csi2tx.dev, "Invalid lane[%d] number: %u\n",
    i, v4l2_ep.bus.mipi_csi2.data_lanes[i]);
    ret = -EINVAL;
    goto out;
    }
    }
    memcpy(csi2tx.lanes, v4l2_ep.bus.mipi_csi2.data_lanes,
    sizeof(csi2tx.lanes));
    out:
    of_node_put(ep);
    return ret;
    }
    static const struct csi2tx_vops csi2tx_vops = {
    .dphy_setup = csi2tx_dphy_setup,
    };
    static const struct csi2tx_vops csi2tx_v2_vops = {
    .dphy_setup = csi2tx_v2_dphy_setup,
    };
    static const struct of_device_id csi2tx_of_table[] = {
    {
    .compatible = "cdns,csi2tx",
    .data = &csi2tx_vops
    },
    {
    .compatible = "cdns,csi2tx-1.3",
    .data = &csi2tx_vops
    },
    {
    .compatible = "cdns,csi2tx-2.1",
    .data = &csi2tx_v2_vops
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, csi2tx_of_table);
#[no_mangle]
unsafe extern "C" fn csi2tx_probe(pdev: *mut platform_device) -> c_int {
    static int csi2tx_probe(struct platform_device *pdev)
    {
    struct csi2tx_priv *csi2tx;
    const struct of_device_id *of_id;
    unsigned int i;
    int ret;
    csi2tx = kzalloc_obj(*csi2tx);
    if (!csi2tx)
    return -ENOMEM;
    platform_set_drvdata(pdev, csi2tx);
    mutex_init(&csi2tx.lock);
    csi2tx.dev = &pdev.dev;
    ret = csi2tx_get_resources(csi2tx, pdev);
    if (ret)
    goto err_free_priv;
    of_id = of_match_node(csi2tx_of_table, pdev.dev.of_node);
    csi2tx.vops = (struct csi2tx_vops *)of_id.data;
    v4l2_subdev_init(&csi2tx.subdev, &csi2tx_subdev_ops);
    csi2tx.subdev.owner = THIS_MODULE;
    csi2tx.subdev.dev = &pdev.dev;
    csi2tx.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    snprintf(csi2tx.subdev.name, sizeof(csi2tx.subdev.name),
    "%s.%s", KBUILD_MODNAME, dev_name(&pdev.dev));
    ret = csi2tx_check_lanes(csi2tx);
    if (ret)
    goto err_free_priv;
// Create our media pads
    csi2tx.subdev.entity.function = MEDIA_ENT_F_VID_IF_BRIDGE;
    csi2tx.pads[CSI2TX_PAD_SOURCE].flags = MEDIA_PAD_FL_SOURCE;
    for (i = CSI2TX_PAD_SINK_STREAM0; i < CSI2TX_PAD_MAX; i++)
    csi2tx.pads[i].flags = MEDIA_PAD_FL_SINK;
//
// Only the input pads are considered to have a format at the
// moment. The CSI link can multiplex various streams with
// different formats, and we can't expose this in v4l2 right
// now.
//
    for (i = CSI2TX_PAD_SINK_STREAM0; i < CSI2TX_PAD_MAX; i++)
    csi2tx.pad_fmts[i] = fmt_default;
    ret = media_entity_pads_init(&csi2tx.subdev.entity, CSI2TX_PAD_MAX,
    csi2tx.pads);
    if (ret)
    goto err_free_priv;
    ret = v4l2_async_register_subdev(&csi2tx.subdev);
    if (ret < 0)
    goto err_free_priv;
    dev_info(&pdev.dev,
    "Probed CSI2TX with %u/%u lanes, %u streams, %s D-PHY\n",
    csi2tx.num_lanes, csi2tx.max_lanes, csi2tx.max_streams,
    csi2tx.has_internal_dphy ? "internal" : "no");
    return 0;
    err_free_priv:
    kfree(csi2tx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn csi2tx_remove(pdev: *mut platform_device) {
    static void csi2tx_remove(struct platform_device *pdev)
    {
    struct csi2tx_priv *csi2tx = platform_get_drvdata(pdev);
    v4l2_async_unregister_subdev(&csi2tx.subdev);
    kfree(csi2tx);
    }
    static struct platform_driver csi2tx_driver = {
    .probe	= csi2tx_probe,
    .remove = csi2tx_remove,
    .driver	= {
    .name		= "cdns-csi2tx",
    .of_match_table	= csi2tx_of_table,
    },
    };
    module_platform_driver(csi2tx_driver);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@bootlin.com>");
    MODULE_DESCRIPTION("Cadence CSI2-TX controller");
    MODULE_LICENSE("GPL");
