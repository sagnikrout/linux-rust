//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/imx/imx8qxp-pixel-combiner.c
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
// Copyright 2020 NXP
//

pub const PC_CTRL_REG: c_uint = 0x0;

    FIELD_PREP(PC_DISP0_PIX_DATA_FORMAT_MASK, (fmt))

    FIELD_PREP(PC_DISP1_PIX_DATA_FORMAT_MASK, (fmt))
pub const PC_SW_RESET_REG: c_uint = 0x20;

    PC_DISP_SW_RESET_N(0) |	\
    PC_DISP_SW_RESET_N(1))
pub const PC_REG_SET: c_uint = 0x4;
pub const PC_REG_CLR: c_uint = 0x8;

    enum imx8qxp_pc_pix_data_format {
    RGB,
    YUV444,
    YUV422,
    SPLIT_RGB,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8qxp_pc_channel {
    pub bridge: drm_bridge,
    pub pc: *mut imx8qxp_pc,
    pub stream_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8qxp_pc {
    pub dev: *mut device,
    pub ch: [*mut imx8qxp_pc_channel; 2],
    pub clk_apb: *mut clk,
    pub base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn imx8qxp_pc_read(pc: *mut imx8qxp_pc, offset: c_uint) -> u32 {
    static inline u32 imx8qxp_pc_read(struct imx8qxp_pc *pc, unsigned int offset)
    {
    return readl(pc.base + offset);
    }
    static inline void
    imx8qxp_pc_write(struct imx8qxp_pc *pc, unsigned int offset, u32 value)
    {
    writel(value, pc.base + offset);
    }
    static inline void
    imx8qxp_pc_write_set(struct imx8qxp_pc *pc, unsigned int offset, u32 value)
    {
    imx8qxp_pc_write(pc, offset + PC_REG_SET, value);
    }
    static inline void
    imx8qxp_pc_write_clr(struct imx8qxp_pc *pc, unsigned int offset, u32 value)
    {
    imx8qxp_pc_write(pc, offset + PC_REG_CLR, value);
    }
    static enum drm_mode_status
    imx8qxp_pc_bridge_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    if (mode.hdisplay > 2560)
    return MODE_BAD_HVALUE;
    return MODE_OK;
    }
    static int imx8qxp_pc_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct imx8qxp_pc_channel *ch = bridge.driver_private;
    struct imx8qxp_pc *pc = ch.pc;
    if (!(flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR)) {
    DRM_DEV_ERROR(pc.dev,
    "do not support creating a drm_connector\n");
    return -EINVAL;
    }
    return drm_bridge_attach(encoder,
    ch.bridge.next_bridge, bridge,
    DRM_BRIDGE_ATTACH_NO_CONNECTOR);
    }
    static void
    imx8qxp_pc_bridge_mode_set(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    struct imx8qxp_pc_channel *ch = bridge.driver_private;
    struct imx8qxp_pc *pc = ch.pc;
    u32 val;
    int ret;
    ret = pm_runtime_get_sync(pc.dev);
    if (ret < 0)
    DRM_DEV_ERROR(pc.dev,
    "failed to get runtime PM sync: %d\n", ret);
    ret = clk_prepare_enable(pc.clk_apb);
    if (ret)
    DRM_DEV_ERROR(pc.dev, "%s: failed to enable apb clock: %d\n",
    __func__,  ret);
// HSYNC to pixel link is active low.
    imx8qxp_pc_write_clr(pc, PC_CTRL_REG,
    PC_DISP_HSYNC_POLARITY(ch.stream_id));
// VSYNC to pixel link is active low.
    imx8qxp_pc_write_clr(pc, PC_CTRL_REG,
    PC_DISP_VSYNC_POLARITY(ch.stream_id));
// Data enable to pixel link is active high.
    imx8qxp_pc_write_set(pc, PC_CTRL_REG,
    PC_DISP_DVALID_POLARITY(ch.stream_id));
// Mask the first frame output which may be incomplete.
    imx8qxp_pc_write_set(pc, PC_CTRL_REG, PC_VSYNC_MASK_ENABLE);
// Only support RGB currently.
    val = imx8qxp_pc_read(pc, PC_CTRL_REG);
    if (ch.stream_id == 0) {
    val &= ~PC_DISP0_PIX_DATA_FORMAT_MASK;
    val |= PC_DISP0_PIX_DATA_FORMAT(RGB);
    } else {
    val &= ~PC_DISP1_PIX_DATA_FORMAT_MASK;
    val |= PC_DISP1_PIX_DATA_FORMAT(RGB);
    }
    imx8qxp_pc_write(pc, PC_CTRL_REG, val);
// Only support bypass mode currently.
    imx8qxp_pc_write_set(pc, PC_CTRL_REG, PC_DISP_BYPASS(ch.stream_id));
    clk_disable_unprepare(pc.clk_apb);
    }
    static void imx8qxp_pc_bridge_atomic_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct imx8qxp_pc_channel *ch = bridge.driver_private;
    struct imx8qxp_pc *pc = ch.pc;
    pm_runtime_put(pc.dev);
    }
    static const u32 imx8qxp_pc_bus_output_fmts[] = {
    MEDIA_BUS_FMT_RGB888_1X36_CPADLO,
    MEDIA_BUS_FMT_RGB666_1X36_CPADLO,
    };
#[no_mangle]
unsafe extern "C" fn imx8qxp_pc_bus_output_fmt_supported(fmt: u32) -> bool {
    static bool imx8qxp_pc_bus_output_fmt_supported(u32 fmt)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(imx8qxp_pc_bus_output_fmts); i++) {
    if (imx8qxp_pc_bus_output_fmts[i] == fmt)
    return true;
    }
    return false;
    }
    static u32 *
    imx8qxp_pc_bridge_atomic_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    u32 *input_fmts;
    if (!imx8qxp_pc_bus_output_fmt_supported(output_fmt))
    return core::ptr::null_mut();
// num_input_fmts = 1;
    input_fmts = kmalloc_obj(*input_fmts);
    if (!input_fmts)
    return core::ptr::null_mut();
    switch (output_fmt) {
    case MEDIA_BUS_FMT_RGB888_1X36_CPADLO:
    input_fmts[0] = MEDIA_BUS_FMT_RGB888_1X30_CPADLO;
    break;
    case MEDIA_BUS_FMT_RGB666_1X36_CPADLO:
    input_fmts[0] = MEDIA_BUS_FMT_RGB666_1X30_CPADLO;
    break;
    default:
    kfree(input_fmts);
    input_fmts = core::ptr::null_mut();
    break;
    }
    return input_fmts;
    }
    static u32 *
    imx8qxp_pc_bridge_atomic_get_output_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    unsigned int *num_output_fmts)
    {
// num_output_fmts = ARRAY_SIZE(imx8qxp_pc_bus_output_fmts);
    return kmemdup(imx8qxp_pc_bus_output_fmts,
    sizeof(imx8qxp_pc_bus_output_fmts), GFP_KERNEL);
    }
    static const struct drm_bridge_funcs imx8qxp_pc_bridge_funcs = {
    .atomic_duplicate_state	= drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state	= drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state	= drm_atomic_helper_bridge_create_state,
    .mode_valid		= imx8qxp_pc_bridge_mode_valid,
    .attach			= imx8qxp_pc_bridge_attach,
    .mode_set		= imx8qxp_pc_bridge_mode_set,
    .atomic_disable		= imx8qxp_pc_bridge_atomic_disable,
    .atomic_get_input_bus_fmts =
    imx8qxp_pc_bridge_atomic_get_input_bus_fmts,
    .atomic_get_output_bus_fmts =
    imx8qxp_pc_bridge_atomic_get_output_bus_fmts,
    };
#[no_mangle]
unsafe extern "C" fn imx8qxp_pc_bridge_probe(pdev: *mut platform_device) -> c_int {
    static int imx8qxp_pc_bridge_probe(struct platform_device *pdev)
    {
    struct imx8qxp_pc *pc;
    struct imx8qxp_pc_channel *ch;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *child, *remote;
    u32 i;
    int ret;
    pc = devm_kzalloc(dev, sizeof(*pc), GFP_KERNEL);
    if (!pc)
    return -ENOMEM;
    pc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pc.base))
    return PTR_ERR(pc.base);
    pc.dev = dev;
    pc.clk_apb = devm_clk_get(dev, "apb");
    if (IS_ERR(pc.clk_apb)) {
    ret = PTR_ERR(pc.clk_apb);
    if (ret != -EPROBE_DEFER)
    DRM_DEV_ERROR(dev, "failed to get apb clock: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, pc);
    pm_runtime_enable(dev);
    for_each_available_child_of_node(np, child) {
    ret = of_property_read_u32(child, "reg", &i);
    if (ret || i > 1) {
    ret = -EINVAL;
    DRM_DEV_ERROR(dev,
    "invalid channel(%u) node address\n", i);
    goto free_child;
    }
    ch = devm_drm_bridge_alloc(dev, struct imx8qxp_pc_channel, bridge,
    &imx8qxp_pc_bridge_funcs);
    if (IS_ERR(ch)) {
    ret = PTR_ERR(ch);
    goto free_child;
    }
    pc.ch[i] = ch;
    ch.pc = pc;
    ch.stream_id = i;
    remote = of_graph_get_remote_node(child, 1, 0);
    if (!remote) {
    ret = -ENODEV;
    DRM_DEV_ERROR(dev,
    "channel%u failed to get port1's remote node: %d\n",
    i, ret);
    goto free_child;
    }
    ch.bridge.next_bridge = of_drm_find_and_get_bridge(remote);
    if (!ch.bridge.next_bridge) {
    of_node_put(remote);
    ret = -EPROBE_DEFER;
    DRM_DEV_DEBUG_DRIVER(dev,
    "channel%u failed to find next bridge: %d\n",
    i, ret);
    goto free_child;
    }
    of_node_put(remote);
    ch.bridge.driver_private = ch;
    ch.bridge.of_node = child;
    drm_bridge_add(&ch.bridge);
    }
    return 0;
    free_child:
    of_node_put(child);
    if (i == 1 && pc.ch[0] && pc.ch[0].bridge.next_bridge)
    drm_bridge_remove(&pc.ch[0].bridge);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8qxp_pc_bridge_remove(pdev: *mut platform_device) {
    static void imx8qxp_pc_bridge_remove(struct platform_device *pdev)
    {
    struct imx8qxp_pc *pc = platform_get_drvdata(pdev);
    struct imx8qxp_pc_channel *ch;
    int i;
    for (i = 0; i < 2; i++) {
    ch = pc.ch[i];
    if (ch)
    drm_bridge_remove(&ch.bridge);
    }
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn imx8qxp_pc_runtime_suspend(dev: *mut device) -> c_int {
    static int imx8qxp_pc_runtime_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct imx8qxp_pc *pc = platform_get_drvdata(pdev);
    int ret;
    ret = clk_prepare_enable(pc.clk_apb);
    if (ret)
    DRM_DEV_ERROR(pc.dev, "%s: failed to enable apb clock: %d\n",
    __func__,  ret);
// Disable pixel combiner by full reset.
    imx8qxp_pc_write_clr(pc, PC_SW_RESET_REG, PC_FULL_RESET_N);
    clk_disable_unprepare(pc.clk_apb);
// Ensure the reset takes effect.
    usleep_range(10, 20);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8qxp_pc_runtime_resume(dev: *mut device) -> c_int {
    static int imx8qxp_pc_runtime_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct imx8qxp_pc *pc = platform_get_drvdata(pdev);
    int ret;
    ret = clk_prepare_enable(pc.clk_apb);
    if (ret) {
    DRM_DEV_ERROR(pc.dev, "%s: failed to enable apb clock: %d\n",
    __func__, ret);
    return ret;
    }
// out of reset
    imx8qxp_pc_write_set(pc, PC_SW_RESET_REG, PC_FULL_RESET_N);
    clk_disable_unprepare(pc.clk_apb);
    return ret;
    }
    static const struct dev_pm_ops imx8qxp_pc_pm_ops = {
    RUNTIME_PM_OPS(imx8qxp_pc_runtime_suspend, imx8qxp_pc_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id imx8qxp_pc_dt_ids[] = {
    { .compatible = "fsl,imx8qm-pixel-combiner", },
    { .compatible = "fsl,imx8qxp-pixel-combiner", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx8qxp_pc_dt_ids);
    static struct platform_driver imx8qxp_pc_bridge_driver = {
    .probe	= imx8qxp_pc_bridge_probe,
    .remove = imx8qxp_pc_bridge_remove,
    .driver	= {
    .pm = pm_ptr(&imx8qxp_pc_pm_ops),
    .name = DRIVER_NAME,
    .of_match_table = imx8qxp_pc_dt_ids,
    },
    };
    module_platform_driver(imx8qxp_pc_bridge_driver);
    MODULE_DESCRIPTION("i.MX8QM/QXP pixel combiner bridge driver");
    MODULE_AUTHOR("Liu Ying <victor.liu@nxp.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
