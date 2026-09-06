//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/displays/panel-dpi.c
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
// Generic MIPI DPI Panel Driver
//
// Copyright (C) 2013 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_drv_data {
    pub dssdev: omap_dss_device,
    pub in: *mut omap_dss_device,
    pub data_lines: c_int,
    pub videomode: omap_video_timings,
    pub enable_gpio: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn panel_dpi_connect(dssdev: *mut omap_dss_device) -> c_int {
    static int panel_dpi_connect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (omapdss_device_is_connected(dssdev))
    return 0;
    return in.ops.dpi.connect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn panel_dpi_disconnect(dssdev: *mut omap_dss_device) {
    static void panel_dpi_disconnect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (!omapdss_device_is_connected(dssdev))
    return;
    in.ops.dpi.disconnect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn panel_dpi_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int panel_dpi_enable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    if (!omapdss_device_is_connected(dssdev))
    return -ENODEV;
    if (omapdss_device_is_enabled(dssdev))
    return 0;
    if (ddata.data_lines)
    in.ops.dpi.set_data_lines(in, ddata.data_lines);
    in.ops.dpi.set_timings(in, &ddata.videomode);
    r = in.ops.dpi.enable(in);
    if (r)
    return r;
    gpiod_set_value_cansleep(ddata.enable_gpio, 1);
    dssdev.state = OMAP_DSS_DISPLAY_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn panel_dpi_disable(dssdev: *mut omap_dss_device) {
    static void panel_dpi_disable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (!omapdss_device_is_enabled(dssdev))
    return;
    gpiod_set_value_cansleep(ddata.enable_gpio, 0);
    in.ops.dpi.disable(in);
    dssdev.state = OMAP_DSS_DISPLAY_DISABLED;
    }
    static void panel_dpi_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    ddata.videomode = *timings;
    dssdev.panel.timings = *timings;
    in.ops.dpi.set_timings(in, timings);
    }
    static void panel_dpi_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
// timings = ddata->videomode;
    }
    static int panel_dpi_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.dpi.check_timings(in, timings);
    }
    static struct omap_dss_driver panel_dpi_ops = {
    .connect	= panel_dpi_connect,
    .disconnect	= panel_dpi_disconnect,
    .enable		= panel_dpi_enable,
    .disable	= panel_dpi_disable,
    .set_timings	= panel_dpi_set_timings,
    .get_timings	= panel_dpi_get_timings,
    .check_timings	= panel_dpi_check_timings,
    .get_resolution	= omapdss_default_get_resolution,
    };
#[no_mangle]
unsafe extern "C" fn panel_dpi_probe_of(pdev: *mut platform_device) -> c_int {
    static int panel_dpi_probe_of(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    struct omap_dss_device *in;
    int r;
    struct display_timing timing;
    struct videomode vm;
    struct gpio_desc *gpio;
    gpio = devm_gpiod_get_optional(&pdev.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    ddata.enable_gpio = gpio;
    r = of_get_display_timing(node, "panel-timing", &timing);
    if (r) {
    dev_err(&pdev.dev, "failed to get video timing\n");
    return r;
    }
    videomode_from_timing(&timing, &vm);
    videomode_to_omap_video_timings(&vm, &ddata.videomode);
    in = omapdss_of_find_source_for_first_ep(node);
    if (IS_ERR(in)) {
    dev_err(&pdev.dev, "failed to find video source\n");
    return PTR_ERR(in);
    }
    ddata.in = in;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn panel_dpi_probe(pdev: *mut platform_device) -> c_int {
    static int panel_dpi_probe(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata;
    struct omap_dss_device *dssdev;
    int r;
    if (!pdev.dev.of_node)
    return -ENODEV;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (ddata == core::ptr::null_mut())
    return -ENOMEM;
    platform_set_drvdata(pdev, ddata);
    r = panel_dpi_probe_of(pdev);
    if (r)
    return r;
    dssdev = &ddata.dssdev;
    dssdev.dev = &pdev.dev;
    dssdev.driver = &panel_dpi_ops;
    dssdev.type = OMAP_DISPLAY_TYPE_DPI;
    dssdev.owner = THIS_MODULE;
    dssdev.panel.timings = ddata.videomode;
    dssdev.phy.dpi.data_lines = ddata.data_lines;
    r = omapdss_register_display(dssdev);
    if (r) {
    dev_err(&pdev.dev, "Failed to register panel\n");
    goto err_reg;
    }
    return 0;
    err_reg:
    omap_dss_put_device(ddata.in);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn panel_dpi_remove(pdev: *mut platform_device) {
    static void panel_dpi_remove(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct omap_dss_device *dssdev = &ddata.dssdev;
    struct omap_dss_device *in = ddata.in;
    omapdss_unregister_display(dssdev);
    panel_dpi_disable(dssdev);
    panel_dpi_disconnect(dssdev);
    omap_dss_put_device(in);
    }
    static const struct of_device_id panel_dpi_of_match[] = {
    { .compatible = "omapdss,panel-dpi", },
    {},
    };
    MODULE_DEVICE_TABLE(of, panel_dpi_of_match);
    static struct platform_driver panel_dpi_driver = {
    .probe = panel_dpi_probe,
    .remove = panel_dpi_remove,
    .driver = {
    .name = "panel-dpi",
    .of_match_table = panel_dpi_of_match,
    },
    };
    module_platform_driver(panel_dpi_driver);
    MODULE_AUTHOR("Tomi Valkeinen <tomi.valkeinen@ti.com>");
    MODULE_DESCRIPTION("Generic MIPI DPI Panel Driver");
    MODULE_LICENSE("GPL");
