//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/displays/connector-analog-tv.c
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
// Analog TV Connector driver
//
// Copyright (C) 2013 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_drv_data {
    pub dssdev: omap_dss_device,
    pub in: *mut omap_dss_device,
    pub dev: *mut device,
    pub timings: omap_video_timings,
    pub invert_polarity: bool,
}

    static const struct omap_video_timings tvc_pal_timings = {
    .x_res		= 720,
    .y_res		= 574,
    .pixelclock	= 13500000,
    .hsw		= 64,
    .hfp		= 12,
    .hbp		= 68,
    .vsw		= 5,
    .vfp		= 5,
    .vbp		= 41,
    .interlace	= true,
    };
    static const struct of_device_id tvc_of_match[];

#[no_mangle]
unsafe extern "C" fn tvc_connect(dssdev: *mut omap_dss_device) -> c_int {
    static int tvc_connect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(ddata.dev, "connect\n");
    if (omapdss_device_is_connected(dssdev))
    return 0;
    return in.ops.atv.connect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn tvc_disconnect(dssdev: *mut omap_dss_device) {
    static void tvc_disconnect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(ddata.dev, "disconnect\n");
    if (!omapdss_device_is_connected(dssdev))
    return;
    in.ops.atv.disconnect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn tvc_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int tvc_enable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    dev_dbg(ddata.dev, "enable\n");
    if (!omapdss_device_is_connected(dssdev))
    return -ENODEV;
    if (omapdss_device_is_enabled(dssdev))
    return 0;
    in.ops.atv.set_timings(in, &ddata.timings);
    if (!ddata.dev.of_node) {
    in.ops.atv.set_type(in, OMAP_DSS_VENC_TYPE_COMPOSITE);
    in.ops.atv.invert_vid_out_polarity(in,
    ddata.invert_polarity);
    }
    r = in.ops.atv.enable(in);
    if (r)
    return r;
    dssdev.state = OMAP_DSS_DISPLAY_ACTIVE;
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tvc_disable(dssdev: *mut omap_dss_device) {
    static void tvc_disable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(ddata.dev, "disable\n");
    if (!omapdss_device_is_enabled(dssdev))
    return;
    in.ops.atv.disable(in);
    dssdev.state = OMAP_DSS_DISPLAY_DISABLED;
    }
    static void tvc_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    ddata.timings = *timings;
    dssdev.panel.timings = *timings;
    in.ops.atv.set_timings(in, timings);
    }
    static void tvc_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
// timings = ddata->timings;
    }
    static int tvc_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.atv.check_timings(in, timings);
    }
#[no_mangle]
unsafe extern "C" fn tvc_get_wss(dssdev: *mut omap_dss_device) -> u32 {
    static u32 tvc_get_wss(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.atv.get_wss(in);
    }
#[no_mangle]
unsafe extern "C" fn tvc_set_wss(dssdev: *mut omap_dss_device, wss: u32) -> c_int {
    static int tvc_set_wss(struct omap_dss_device *dssdev, u32 wss)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.atv.set_wss(in, wss);
    }
    static struct omap_dss_driver tvc_driver = {
    .connect		= tvc_connect,
    .disconnect		= tvc_disconnect,
    .enable			= tvc_enable,
    .disable		= tvc_disable,
    .set_timings		= tvc_set_timings,
    .get_timings		= tvc_get_timings,
    .check_timings		= tvc_check_timings,
    .get_resolution		= omapdss_default_get_resolution,
    .get_wss		= tvc_get_wss,
    .set_wss		= tvc_set_wss,
    };
#[no_mangle]
unsafe extern "C" fn tvc_probe(pdev: *mut platform_device) -> c_int {
    static int tvc_probe(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata;
    struct omap_dss_device *dssdev;
    int r;
    if (!pdev.dev.of_node)
    return -ENODEV;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    platform_set_drvdata(pdev, ddata);
    ddata.dev = &pdev.dev;
    ddata.in = omapdss_of_find_source_for_first_ep(pdev.dev.of_node);
    r = PTR_ERR_OR_ZERO(ddata.in);
    if (r) {
    dev_err(&pdev.dev, "failed to find video source\n");
    return r;
    }
    ddata.timings = tvc_pal_timings;
    dssdev = &ddata.dssdev;
    dssdev.driver = &tvc_driver;
    dssdev.dev = &pdev.dev;
    dssdev.type = OMAP_DISPLAY_TYPE_VENC;
    dssdev.owner = THIS_MODULE;
    dssdev.panel.timings = tvc_pal_timings;
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
unsafe extern "C" fn tvc_remove(pdev: *mut platform_device) {
    static void tvc_remove(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct omap_dss_device *dssdev = &ddata.dssdev;
    struct omap_dss_device *in = ddata.in;
    omapdss_unregister_display(&ddata.dssdev);
    tvc_disable(dssdev);
    tvc_disconnect(dssdev);
    omap_dss_put_device(in);
    }
    static const struct of_device_id tvc_of_match[] = {
    { .compatible = "omapdss,svideo-connector", },
    { .compatible = "omapdss,composite-video-connector", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tvc_of_match);
    static struct platform_driver tvc_connector_driver = {
    .probe	= tvc_probe,
    .remove	= tvc_remove,
    .driver	= {
    .name	= "connector-analog-tv",
    .of_match_table = tvc_of_match,
    },
    };
    module_platform_driver(tvc_connector_driver);
    MODULE_AUTHOR("Tomi Valkeinen <tomi.valkeinen@ti.com>");
    MODULE_DESCRIPTION("Analog TV Connector driver");
    MODULE_LICENSE("GPL");
