//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tiny/arcpgu.c
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
// ARC PGU DRM driver.
//
// Copyright (C) 2016 Synopsys, Inc. (www.synopsys.com)
//

pub const ARCPGU_REG_CTRL: c_uint = 0x00;
pub const ARCPGU_REG_STAT: c_uint = 0x04;
pub const ARCPGU_REG_FMT: c_uint = 0x10;
pub const ARCPGU_REG_HSYNC: c_uint = 0x14;
pub const ARCPGU_REG_VSYNC: c_uint = 0x18;
pub const ARCPGU_REG_ACTIVE: c_uint = 0x1c;
pub const ARCPGU_REG_BUF0_ADDR: c_uint = 0x40;
pub const ARCPGU_REG_STRIDE: c_uint = 0x50;
pub const ARCPGU_REG_START_SET: c_uint = 0x84;
pub const ARCPGU_REG_ID: c_uint = 0x3FC;
pub const ARCPGU_CTRL_ENABLE_MASK: c_uint = 0x02;
pub const ARCPGU_CTRL_VS_POL_MASK: c_uint = 0x1;
pub const ARCPGU_CTRL_VS_POL_OFST: c_uint = 0x3;
pub const ARCPGU_CTRL_HS_POL_MASK: c_uint = 0x1;
pub const ARCPGU_CTRL_HS_POL_OFST: c_uint = 0x4;

pub const ARCPGU_STAT_BUSY_MASK: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcpgu_drm_private {
    pub drm: drm_device,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub pipe: drm_simple_display_pipe,
    pub sim_conn: drm_connector,
}

    static inline void arc_pgu_write(struct arcpgu_drm_private *arcpgu,
    unsigned int reg, u32 value)
    {
    iowrite32(value, arcpgu.regs + reg);
    }
    static inline u32 arc_pgu_read(struct arcpgu_drm_private *arcpgu,
    unsigned int reg)
    {
    return ioread32(arcpgu.regs + reg);
    }
pub const XRES_DEF: c_int = 640;
pub const YRES_DEF: c_int = 480;
pub const XRES_MAX: c_int = 8192;
pub const YRES_MAX: c_int = 8192;
#[no_mangle]
unsafe extern "C" fn arcpgu_drm_connector_get_modes(connector: *mut drm_connector) -> c_int {
    static int arcpgu_drm_connector_get_modes(struct drm_connector *connector)
    {
    int count;
    count = drm_add_modes_noedid(connector, XRES_MAX, YRES_MAX);
    drm_set_preferred_mode(connector, XRES_DEF, YRES_DEF);
    return count;
    }
    static const struct drm_connector_helper_funcs
    arcpgu_drm_connector_helper_funcs = {
    .get_modes = arcpgu_drm_connector_get_modes,
    };
    static const struct drm_connector_funcs arcpgu_drm_connector_funcs = {
    .reset = drm_atomic_helper_connector_reset,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = drm_connector_cleanup,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
#[no_mangle]
unsafe extern "C" fn arcpgu_drm_sim_init(drm: *mut drm_device, connector: *mut drm_connector) -> c_int {
    static int arcpgu_drm_sim_init(struct drm_device *drm, struct drm_connector *connector)
    {
    drm_connector_helper_add(connector, &arcpgu_drm_connector_helper_funcs);
    return drm_connector_init(drm, connector, &arcpgu_drm_connector_funcs,
    DRM_MODE_CONNECTOR_VIRTUAL);
    }

    static const u32 arc_pgu_supported_formats[] = {
    DRM_FORMAT_RGB565,
    DRM_FORMAT_XRGB8888,
    DRM_FORMAT_ARGB8888,
    };
#[no_mangle]
unsafe extern "C" fn arc_pgu_set_pxl_fmt(arcpgu: *mut arcpgu_drm_private) {
    static void arc_pgu_set_pxl_fmt(struct arcpgu_drm_private *arcpgu)
    {
    const struct drm_framebuffer *fb = arcpgu.pipe.plane.state.fb;
    let mut pixel_format: u32 = fb.format.format;
    let mut format: u32 = DRM_FORMAT_INVALID;
    int i;
    u32 reg_ctrl;
    for (i = 0; i < ARRAY_SIZE(arc_pgu_supported_formats); i++) {
    if (arc_pgu_supported_formats[i] == pixel_format)
    format = arc_pgu_supported_formats[i];
    }
    if (WARN_ON(format == DRM_FORMAT_INVALID))
    return;
    reg_ctrl = arc_pgu_read(arcpgu, ARCPGU_REG_CTRL);
    if (format == DRM_FORMAT_RGB565)
    reg_ctrl &= ~ARCPGU_MODE_XRGB8888;
    else
    reg_ctrl |= ARCPGU_MODE_XRGB8888;
    arc_pgu_write(arcpgu, ARCPGU_REG_CTRL, reg_ctrl);
    }
    static enum drm_mode_status arc_pgu_mode_valid(struct drm_simple_display_pipe *pipe,
    const struct drm_display_mode *mode)
    {
    struct arcpgu_drm_private *arcpgu = pipe_to_arcpgu_priv(pipe);
    long rate, clk_rate = mode.clock * 1000;
    long diff = clk_rate / 200; /* +-0.5% allowed by HDMI spec */
    rate = clk_round_rate(arcpgu.clk, clk_rate);
    if ((max(rate, clk_rate) - min(rate, clk_rate) < diff) && (rate > 0))
    return MODE_OK;
    return MODE_NOCLOCK;
    }
#[no_mangle]
unsafe extern "C" fn arc_pgu_mode_set(arcpgu: *mut arcpgu_drm_private) {
    static void arc_pgu_mode_set(struct arcpgu_drm_private *arcpgu)
    {
    struct drm_display_mode *m = &arcpgu.pipe.crtc.state.adjusted_mode;
    u32 val;
    arc_pgu_write(arcpgu, ARCPGU_REG_FMT,
    ENCODE_PGU_XY(m.crtc_htotal, m.crtc_vtotal));
    arc_pgu_write(arcpgu, ARCPGU_REG_HSYNC,
    ENCODE_PGU_XY(m.crtc_hsync_start - m.crtc_hdisplay,
    m.crtc_hsync_end - m.crtc_hdisplay));
    arc_pgu_write(arcpgu, ARCPGU_REG_VSYNC,
    ENCODE_PGU_XY(m.crtc_vsync_start - m.crtc_vdisplay,
    m.crtc_vsync_end - m.crtc_vdisplay));
    arc_pgu_write(arcpgu, ARCPGU_REG_ACTIVE,
    ENCODE_PGU_XY(m.crtc_hblank_end - m.crtc_hblank_start,
    m.crtc_vblank_end - m.crtc_vblank_start));
    val = arc_pgu_read(arcpgu, ARCPGU_REG_CTRL);
    if (m.flags & DRM_MODE_FLAG_PVSYNC)
    val |= ARCPGU_CTRL_VS_POL_MASK << ARCPGU_CTRL_VS_POL_OFST;
    else
    val &= ~(ARCPGU_CTRL_VS_POL_MASK << ARCPGU_CTRL_VS_POL_OFST);
    if (m.flags & DRM_MODE_FLAG_PHSYNC)
    val |= ARCPGU_CTRL_HS_POL_MASK << ARCPGU_CTRL_HS_POL_OFST;
    else
    val &= ~(ARCPGU_CTRL_HS_POL_MASK << ARCPGU_CTRL_HS_POL_OFST);
    arc_pgu_write(arcpgu, ARCPGU_REG_CTRL, val);
    arc_pgu_write(arcpgu, ARCPGU_REG_STRIDE, 0);
    arc_pgu_write(arcpgu, ARCPGU_REG_START_SET, 1);
    arc_pgu_set_pxl_fmt(arcpgu);
    clk_set_rate(arcpgu.clk, m.crtc_clock * 1000);
    }
    static void arc_pgu_enable(struct drm_simple_display_pipe *pipe,
    struct drm_crtc_state *crtc_state,
    struct drm_plane_state *plane_state)
    {
    struct arcpgu_drm_private *arcpgu = pipe_to_arcpgu_priv(pipe);
    arc_pgu_mode_set(arcpgu);
    clk_prepare_enable(arcpgu.clk);
    arc_pgu_write(arcpgu, ARCPGU_REG_CTRL,
    arc_pgu_read(arcpgu, ARCPGU_REG_CTRL) |
    ARCPGU_CTRL_ENABLE_MASK);
    }
#[no_mangle]
unsafe extern "C" fn arc_pgu_disable(pipe: *mut drm_simple_display_pipe) {
    static void arc_pgu_disable(struct drm_simple_display_pipe *pipe)
    {
    struct arcpgu_drm_private *arcpgu = pipe_to_arcpgu_priv(pipe);
    clk_disable_unprepare(arcpgu.clk);
    arc_pgu_write(arcpgu, ARCPGU_REG_CTRL,
    arc_pgu_read(arcpgu, ARCPGU_REG_CTRL) &
    ~ARCPGU_CTRL_ENABLE_MASK);
    }
    static void arc_pgu_update(struct drm_simple_display_pipe *pipe,
    struct drm_plane_state *state)
    {
    struct arcpgu_drm_private *arcpgu;
    struct drm_gem_dma_object *gem;
    if (!pipe.plane.state.fb)
    return;
    arcpgu = pipe_to_arcpgu_priv(pipe);
    gem = drm_fb_dma_get_gem_obj(pipe.plane.state.fb, 0);
    arc_pgu_write(arcpgu, ARCPGU_REG_BUF0_ADDR, gem.dma_addr);
    }
    static const struct drm_simple_display_pipe_funcs arc_pgu_pipe_funcs = {
    .update = arc_pgu_update,
    .mode_valid = arc_pgu_mode_valid,
    .enable	= arc_pgu_enable,
    .disable = arc_pgu_disable,
    };
    static const struct drm_mode_config_funcs arcpgu_drm_modecfg_funcs = {
    .fb_create  = drm_gem_fb_create,
    .atomic_check = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    };
    DEFINE_DRM_GEM_DMA_FOPS(arcpgu_drm_ops);
#[no_mangle]
unsafe extern "C" fn arcpgu_load(arcpgu: *mut arcpgu_drm_private) -> c_int {
    static int arcpgu_load(struct arcpgu_drm_private *arcpgu)
    {
    struct platform_device *pdev = to_platform_device(arcpgu.drm.dev);
    struct device_node *encoder_node __free(device_node) = core::ptr::null_mut();
    struct device_node *endpoint_node = core::ptr::null_mut();
    struct drm_connector *connector = core::ptr::null_mut();
    struct drm_device *drm = &arcpgu.drm;
    int ret;
    arcpgu.clk = devm_clk_get(drm.dev, "pxlclk");
    if (IS_ERR(arcpgu.clk))
    return PTR_ERR(arcpgu.clk);
    ret = drmm_mode_config_init(drm);
    if (ret)
    return ret;
    drm.mode_config.min_width = 0;
    drm.mode_config.min_height = 0;
    drm.mode_config.max_width = 1920;
    drm.mode_config.max_height = 1080;
    drm.mode_config.funcs = &arcpgu_drm_modecfg_funcs;
    arcpgu.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(arcpgu.regs))
    return PTR_ERR(arcpgu.regs);
    dev_info(drm.dev, "arc_pgu ID: 0x%x\n",
    arc_pgu_read(arcpgu, ARCPGU_REG_ID));
// Get the optional framebuffer memory resource
    ret = of_reserved_mem_device_init(drm.dev);
    if (ret && ret != -ENODEV)
    return ret;
    if (dma_set_mask_and_coherent(drm.dev, DMA_BIT_MASK(32)))
    return -ENODEV;
//
// There is only one output port inside each device. It is linked with
// encoder endpoint.
//
    endpoint_node = of_graph_get_endpoint_by_regs(pdev.dev.of_node, 0, -1);
    if (endpoint_node) {
    encoder_node = of_graph_get_remote_port_parent(endpoint_node);
    of_node_put(endpoint_node);
    } else {
    connector = &arcpgu.sim_conn;
    dev_info(drm.dev, "no encoder found. Assumed virtual LCD on simulation platform\n");
    ret = arcpgu_drm_sim_init(drm, connector);
    if (ret < 0)
    return ret;
    }
    ret = drm_simple_display_pipe_init(drm, &arcpgu.pipe, &arc_pgu_pipe_funcs,
    arc_pgu_supported_formats,
    ARRAY_SIZE(arc_pgu_supported_formats),
    core::ptr::null_mut(), connector);
    if (ret)
    return ret;
    if (encoder_node) {
// Locate drm bridge from the hdmi encoder DT node
    struct drm_bridge *bridge __free(drm_bridge_put) =
    of_drm_find_and_get_bridge(encoder_node);
    if (!bridge)
    return -EPROBE_DEFER;
    ret = drm_simple_display_pipe_attach_bridge(&arcpgu.pipe, bridge);
    if (ret)
    return ret;
    }
    drm_mode_config_reset(drm);
    drm_kms_helper_poll_init(drm);
    platform_set_drvdata(pdev, drm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arcpgu_unload(drm: *mut drm_device) -> c_int {
    static int arcpgu_unload(struct drm_device *drm)
    {
    drm_kms_helper_poll_fini(drm);
    drm_atomic_helper_shutdown(drm);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn arcpgu_show_pxlclock(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int arcpgu_show_pxlclock(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct drm_device *drm = node.minor.dev;
    struct arcpgu_drm_private *arcpgu = dev_to_arcpgu(drm);
    let mut clkrate: c_ulong = clk_get_rate(arcpgu.clk);
    let mut mode_clock: c_ulong = arcpgu.pipe.crtc.mode.crtc_clock * 1000;
    seq_printf(m, "hw  : %lu\n", clkrate);
    seq_printf(m, "mode: %lu\n", mode_clock);
    return 0;
    }
    static struct drm_info_list arcpgu_debugfs_list[] = {
    { "clocks", arcpgu_show_pxlclock, 0 },
    };
#[no_mangle]
unsafe extern "C" fn arcpgu_debugfs_init(minor: *mut drm_minor) {
    static void arcpgu_debugfs_init(struct drm_minor *minor)
    {
    drm_debugfs_create_files(arcpgu_debugfs_list,
    ARRAY_SIZE(arcpgu_debugfs_list),
    minor.debugfs_root, minor);
    }

    static const struct drm_driver arcpgu_drm_driver = {
    .driver_features = DRIVER_MODESET | DRIVER_GEM | DRIVER_ATOMIC,
    .name = "arcpgu",
    .desc = "ARC PGU Controller",
    .major = 1,
    .minor = 0,
    .patchlevel = 0,
    .fops = &arcpgu_drm_ops,
    DRM_GEM_DMA_DRIVER_OPS,
    DRM_FBDEV_DMA_DRIVER_OPS,

    .debugfs_init = arcpgu_debugfs_init,

    };
#[no_mangle]
unsafe extern "C" fn arcpgu_probe(pdev: *mut platform_device) -> c_int {
    static int arcpgu_probe(struct platform_device *pdev)
    {
    struct arcpgu_drm_private *arcpgu;
    int ret;
    arcpgu = devm_drm_dev_alloc(&pdev.dev, &arcpgu_drm_driver,
    struct arcpgu_drm_private, drm);
    if (IS_ERR(arcpgu))
    return PTR_ERR(arcpgu);
    ret = arcpgu_load(arcpgu);
    if (ret)
    return ret;
    ret = drm_dev_register(&arcpgu.drm, 0);
    if (ret)
    goto err_unload;
    drm_client_setup_with_fourcc(&arcpgu.drm, DRM_FORMAT_RGB565);
    return 0;
    err_unload:
    arcpgu_unload(&arcpgu.drm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arcpgu_remove(pdev: *mut platform_device) {
    static void arcpgu_remove(struct platform_device *pdev)
    {
    struct drm_device *drm = platform_get_drvdata(pdev);
    drm_dev_unregister(drm);
    arcpgu_unload(drm);
    }
    static const struct of_device_id arcpgu_of_table[] = {
    {.compatible = "snps,arcpgu"},
    {}
    };
    MODULE_DEVICE_TABLE(of, arcpgu_of_table);
    static struct platform_driver arcpgu_platform_driver = {
    .probe = arcpgu_probe,
    .remove = arcpgu_remove,
    .driver = {
    .name = "arcpgu",
    .of_match_table = arcpgu_of_table,
    },
    };
    drm_module_platform_driver(arcpgu_platform_driver);
    MODULE_AUTHOR("Carlos Palminha <palminha@synopsys.com>");
    MODULE_DESCRIPTION("ARC PGU DRM driver");
    MODULE_LICENSE("GPL");
