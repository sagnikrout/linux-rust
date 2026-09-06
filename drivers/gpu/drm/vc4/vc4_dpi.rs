//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/vc4_dpi.c
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
// Copyright (C) 2016 Broadcom Limited
//
// DOC: VC4 DPI module
//
// The VC4 DPI hardware supports MIPI DPI type 4 and Nokia ViSSI
// signals.  On BCM2835, these can be routed out to GPIO0-27 with the
// ALT2 function.
//

pub const DPI_C: c_uint = 0x00;

// The order field takes the incoming 24 bit RGB from the pixel valve
// and shuffles the 3 channels.
//

// The format field takes the ORDER-shuffled pixel valve data and
// formats it onto the output lines.
//

// This define is named in the hardware, but actually just outputs 0.

// Outputs 00000000rrrrrggggggbbbbb

// Outputs 000rrrrr00gggggg000bbbbb

// Outputs 00rrrrr000gggggg00bbbbb0

// Outputs 000000rrrrrrggggggbbbbbb

// Outputs 00rrrrrr00gggggg00bbbbbb

// Outputs rrrrrrrrggggggggbbbbbbbb

// Reverses the polarity of the corresponding signal

// Outputs the signal the falling clock edge instead of rising.

// Disables the signal

// Power gate to the device, full reset at 0 -> 1 transition

// All other registers besides DPI_C return the ID
pub const DPI_ID: c_uint = 0x04;

// General DPI hardware state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_dpi {
    pub encoder: vc4_encoder,
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub pixel_clock: *mut clk,
    pub core_clock: *mut clk,
    pub regset: debugfs_regset32,
}

    container_of_const(_encoder, struct vc4_dpi, encoder.base)

    ({										\
    kunit_fail_current_test("Accessing a register in a unit test!\n");	\
    readl(dpi.regs + (offset));						\
    })

    do {										\
    kunit_fail_current_test("Accessing a register in a unit test!\n");	\
    writel(val, dpi.regs + (offset));					\
    } while (0)
    static const struct debugfs_reg32 dpi_regs[] = {
    VC4_REG32(DPI_C),
    VC4_REG32(DPI_ID),
    };
#[no_mangle]
unsafe extern "C" fn vc4_dpi_encoder_disable(encoder: *mut drm_encoder) {
    static void vc4_dpi_encoder_disable(struct drm_encoder *encoder)
    {
    struct drm_device *dev = encoder.dev;
    struct vc4_dpi *dpi = to_vc4_dpi(encoder);
    int idx;
    if (!drm_dev_enter(dev, &idx))
    return;
    clk_disable_unprepare(dpi.pixel_clock);
    drm_dev_exit(idx);
    }
#[no_mangle]
unsafe extern "C" fn vc4_dpi_encoder_enable(encoder: *mut drm_encoder) {
    static void vc4_dpi_encoder_enable(struct drm_encoder *encoder)
    {
    struct drm_device *dev = encoder.dev;
    struct drm_display_mode *mode = &encoder.crtc.mode;
    struct vc4_dpi *dpi = to_vc4_dpi(encoder);
    struct drm_connector_list_iter conn_iter;
    struct drm_connector *connector = core::ptr::null_mut(), *connector_scan;
    let mut dpi_c: u32 = DPI_ENABLE;
    int idx;
    int ret;
// Look up the connector attached to DPI so we can get the
// bus_format.  Ideally the bridge would tell us the
// bus_format we want, but it doesn't yet, so assume that it's
// uniform throughout the bridge chain.
//
    drm_connector_list_iter_begin(dev, &conn_iter);
    drm_for_each_connector_iter(connector_scan, &conn_iter) {
    if (connector_scan.encoder == encoder) {
    connector = connector_scan;
    break;
    }
    }
    drm_connector_list_iter_end(&conn_iter);
// Default to 18bit if no connector or format found.
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_18BIT_666_RGB_1, DPI_FORMAT);
    if (connector) {
    if (connector.display_info.num_bus_formats) {
    let mut bus_format: u32 = connector.display_info.bus_formats[0];
    dpi_c &= ~DPI_FORMAT_MASK;
    switch (bus_format) {
    case MEDIA_BUS_FMT_RGB888_1X24:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_24BIT_888_RGB,
    DPI_FORMAT);
    break;
    case MEDIA_BUS_FMT_BGR888_1X24:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_24BIT_888_RGB,
    DPI_FORMAT);
    dpi_c |= VC4_SET_FIELD(DPI_ORDER_BGR,
    DPI_ORDER);
    break;
    case MEDIA_BUS_FMT_BGR666_1X24_CPADHI:
    dpi_c |= VC4_SET_FIELD(DPI_ORDER_BGR, DPI_ORDER);
    fallthrough;
    case MEDIA_BUS_FMT_RGB666_1X24_CPADHI:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_18BIT_666_RGB_2,
    DPI_FORMAT);
    break;
    case MEDIA_BUS_FMT_BGR666_1X18:
    dpi_c |= VC4_SET_FIELD(DPI_ORDER_BGR, DPI_ORDER);
    fallthrough;
    case MEDIA_BUS_FMT_RGB666_1X18:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_18BIT_666_RGB_1,
    DPI_FORMAT);
    break;
    case MEDIA_BUS_FMT_RGB565_1X16:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_16BIT_565_RGB_1,
    DPI_FORMAT);
    break;
    case MEDIA_BUS_FMT_RGB565_1X24_CPADHI:
    dpi_c |= VC4_SET_FIELD(DPI_FORMAT_16BIT_565_RGB_2,
    DPI_FORMAT);
    break;
    default:
    drm_err(dev, "Unknown media bus format %d\n",
    bus_format);
    break;
    }
    }
    if (connector.display_info.bus_flags & DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE)
    dpi_c |= DPI_PIXEL_CLK_INVERT;
    if (connector.display_info.bus_flags & DRM_BUS_FLAG_DE_LOW)
    dpi_c |= DPI_OUTPUT_ENABLE_INVERT;
    }
    if (mode.flags & DRM_MODE_FLAG_CSYNC) {
    if (mode.flags & DRM_MODE_FLAG_NCSYNC)
    dpi_c |= DPI_OUTPUT_ENABLE_INVERT;
    } else {
    dpi_c |= DPI_OUTPUT_ENABLE_MODE;
    if (mode.flags & DRM_MODE_FLAG_NHSYNC)
    dpi_c |= DPI_HSYNC_INVERT;
#[no_mangle]
pub unsafe extern "C" fn if(DRM_MODE_FLAG_PHSYNC): !(mode->flags &) -> else {
    else if (!(mode.flags & DRM_MODE_FLAG_PHSYNC))
    dpi_c |= DPI_HSYNC_DISABLE;
    if (mode.flags & DRM_MODE_FLAG_NVSYNC)
    dpi_c |= DPI_VSYNC_INVERT;
#[no_mangle]
pub unsafe extern "C" fn if(DRM_MODE_FLAG_PVSYNC): !(mode->flags &) -> else {
    else if (!(mode.flags & DRM_MODE_FLAG_PVSYNC))
    dpi_c |= DPI_VSYNC_DISABLE;
    }
    if (!drm_dev_enter(dev, &idx))
    return;
    DPI_WRITE(DPI_C, dpi_c);
    ret = clk_set_rate(dpi.pixel_clock, mode.clock * 1000);
    if (ret)
    drm_err(dev, "Failed to set clock rate: %d\n", ret);
    ret = clk_prepare_enable(dpi.pixel_clock);
    if (ret)
    drm_err(dev, "Failed to set clock rate: %d\n", ret);
    drm_dev_exit(idx);
    }
    static enum drm_mode_status vc4_dpi_encoder_mode_valid(struct drm_encoder *encoder,
    const struct drm_display_mode *mode)
    {
    if (mode.flags & DRM_MODE_FLAG_INTERLACE)
    return MODE_NO_INTERLACE;
    return MODE_OK;
    }
    static const struct drm_encoder_helper_funcs vc4_dpi_encoder_helper_funcs = {
    .disable = vc4_dpi_encoder_disable,
    .enable = vc4_dpi_encoder_enable,
    .mode_valid = vc4_dpi_encoder_mode_valid,
    };
#[no_mangle]
unsafe extern "C" fn vc4_dpi_late_register(encoder: *mut drm_encoder) -> c_int {
    static int vc4_dpi_late_register(struct drm_encoder *encoder)
    {
    struct drm_device *drm = encoder.dev;
    struct vc4_dpi *dpi = to_vc4_dpi(encoder);
    vc4_debugfs_add_regset32(drm, "dpi_regs", &dpi.regset);
    return 0;
    }
    static const struct drm_encoder_funcs vc4_dpi_encoder_funcs = {
    .late_register = vc4_dpi_late_register,
    };
    static const struct of_device_id vc4_dpi_dt_match[] = {
    { .compatible = "brcm,bcm2835-dpi", .data = core::ptr::null_mut() },
    {}
    };
// Sets up the next link in the display chain, whether it's a panel or
// a bridge.
//
#[no_mangle]
unsafe extern "C" fn vc4_dpi_init_bridge(dpi: *mut vc4_dpi) -> c_int {
    static int vc4_dpi_init_bridge(struct vc4_dpi *dpi)
    {
    struct drm_device *drm = dpi.encoder.base.dev;
    struct device *dev = &dpi.pdev.dev;
    struct drm_bridge *bridge;
    bridge = drmm_of_get_bridge(drm, dev.of_node, 0, 0);
    if (IS_ERR(bridge)) {
// If nothing was connected in the DT, that's not an
// error.
//
    if (PTR_ERR(bridge) == -ENODEV)
    return 0;
    else
    return PTR_ERR(bridge);
    }
    return drm_bridge_attach(&dpi.encoder.base, bridge, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn vc4_dpi_disable_clock(ptr: *mut c_void) {
    static void vc4_dpi_disable_clock(void *ptr)
    {
    struct vc4_dpi *dpi = ptr;
    clk_disable_unprepare(dpi.core_clock);
    }
#[no_mangle]
unsafe extern "C" fn vc4_dpi_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int vc4_dpi_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct drm_device *drm = dev_get_drvdata(master);
    struct vc4_dpi *dpi;
    int ret;
    dpi = drmm_kzalloc(drm, sizeof(*dpi), GFP_KERNEL);
    if (!dpi)
    return -ENOMEM;
    dpi.encoder.type = VC4_ENCODER_TYPE_DPI;
    dpi.pdev = pdev;
    dpi.regs = vc4_ioremap_regs(pdev, 0);
    if (IS_ERR(dpi.regs))
    return PTR_ERR(dpi.regs);
    dpi.regset.base = dpi.regs;
    dpi.regset.regs = dpi_regs;
    dpi.regset.nregs = ARRAY_SIZE(dpi_regs);
    if (DPI_READ(DPI_ID) != DPI_ID_VALUE) {
    dev_err(dev, "Port returned 0x%08x for ID instead of 0x%08x\n",
    DPI_READ(DPI_ID), DPI_ID_VALUE);
    return -ENODEV;
    }
    dpi.core_clock = devm_clk_get(dev, "core");
    if (IS_ERR(dpi.core_clock)) {
    ret = PTR_ERR(dpi.core_clock);
    if (ret != -EPROBE_DEFER)
    drm_err(drm, "Failed to get core clock: %d\n", ret);
    return ret;
    }
    dpi.pixel_clock = devm_clk_get(dev, "pixel");
    if (IS_ERR(dpi.pixel_clock)) {
    ret = PTR_ERR(dpi.pixel_clock);
    if (ret != -EPROBE_DEFER)
    drm_err(drm, "Failed to get pixel clock: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(dpi.core_clock);
    if (ret) {
    drm_err(drm, "Failed to turn on core clock: %d\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(dev, vc4_dpi_disable_clock, dpi);
    if (ret)
    return ret;
    ret = drmm_encoder_init(drm, &dpi.encoder.base,
    &vc4_dpi_encoder_funcs,
    DRM_MODE_ENCODER_DPI,
    core::ptr::null_mut());
    if (ret)
    return ret;
    drm_encoder_helper_add(&dpi.encoder.base, &vc4_dpi_encoder_helper_funcs);
    ret = vc4_dpi_init_bridge(dpi);
    if (ret)
    return ret;
    dev_set_drvdata(dev, dpi);
    return 0;
    }
    static const struct component_ops vc4_dpi_ops = {
    .bind   = vc4_dpi_bind,
    };
#[no_mangle]
unsafe extern "C" fn vc4_dpi_dev_probe(pdev: *mut platform_device) -> c_int {
    static int vc4_dpi_dev_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &vc4_dpi_ops);
    }
#[no_mangle]
unsafe extern "C" fn vc4_dpi_dev_remove(pdev: *mut platform_device) {
    static void vc4_dpi_dev_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &vc4_dpi_ops);
    }
    struct platform_driver vc4_dpi_driver = {
    .probe = vc4_dpi_dev_probe,
    .remove = vc4_dpi_dev_remove,
    .driver = {
    .name = "vc4_dpi",
    .of_match_table = vc4_dpi_dt_match,
    },
    };
