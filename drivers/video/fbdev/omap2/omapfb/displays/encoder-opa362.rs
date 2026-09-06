//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/displays/encoder-opa362.c
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
// OPA362 analog video amplifier with output/power control
//
// Copyright (C) 2014 Golden Delicious Computers
// Author: H. Nikolaus Schaller <hns@goldelico.com>
//
// based on encoder-tfp410
//
// Copyright (C) 2013 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_drv_data {
    pub dssdev: omap_dss_device,
    pub in: *mut omap_dss_device,
    pub enable_gpio: *mut gpio_desc,
    pub timings: omap_video_timings,
}

    static int opa362_connect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    dev_dbg(dssdev.dev, "connect\n");
    if (omapdss_device_is_connected(dssdev))
    return -EBUSY;
    r = in.ops.atv.connect(in, dssdev);
    if (r)
    return r;
    dst.src = dssdev;
    dssdev.dst = dst;
    return 0;
    }
    static void opa362_disconnect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(dssdev.dev, "disconnect\n");
    WARN_ON(!omapdss_device_is_connected(dssdev));
    if (!omapdss_device_is_connected(dssdev))
    return;
    WARN_ON(dst != dssdev.dst);
    if (dst != dssdev.dst)
    return;
    dst.src = core::ptr::null_mut();
    dssdev.dst = core::ptr::null_mut();
    in.ops.atv.disconnect(in, &ddata.dssdev);
    }
#[no_mangle]
unsafe extern "C" fn opa362_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int opa362_enable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    dev_dbg(dssdev.dev, "enable\n");
    if (!omapdss_device_is_connected(dssdev))
    return -ENODEV;
    if (omapdss_device_is_enabled(dssdev))
    return 0;
    in.ops.atv.set_timings(in, &ddata.timings);
    r = in.ops.atv.enable(in);
    if (r)
    return r;
    if (ddata.enable_gpio)
    gpiod_set_value_cansleep(ddata.enable_gpio, 1);
    dssdev.state = OMAP_DSS_DISPLAY_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opa362_disable(dssdev: *mut omap_dss_device) {
    static void opa362_disable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(dssdev.dev, "disable\n");
    if (!omapdss_device_is_enabled(dssdev))
    return;
    if (ddata.enable_gpio)
    gpiod_set_value_cansleep(ddata.enable_gpio, 0);
    in.ops.atv.disable(in);
    dssdev.state = OMAP_DSS_DISPLAY_DISABLED;
    }
    static void opa362_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(dssdev.dev, "set_timings\n");
    ddata.timings = *timings;
    dssdev.panel.timings = *timings;
    in.ops.atv.set_timings(in, timings);
    }
    static void opa362_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    dev_dbg(dssdev.dev, "get_timings\n");
// timings = ddata->timings;
    }
    static int opa362_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    dev_dbg(dssdev.dev, "check_timings\n");
    return in.ops.atv.check_timings(in, timings);
    }
    static void opa362_set_type(struct omap_dss_device *dssdev,
    enum omap_dss_venc_type type)
    {
// we can only drive a COMPOSITE output
    WARN_ON(type != OMAP_DSS_VENC_TYPE_COMPOSITE);
    }
    static const struct omapdss_atv_ops opa362_atv_ops = {
    .connect	= opa362_connect,
    .disconnect	= opa362_disconnect,
    .enable		= opa362_enable,
    .disable	= opa362_disable,
    .check_timings	= opa362_check_timings,
    .set_timings	= opa362_set_timings,
    .get_timings	= opa362_get_timings,
    .set_type	= opa362_set_type,
    };
#[no_mangle]
unsafe extern "C" fn opa362_probe(pdev: *mut platform_device) -> c_int {
    static int opa362_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct panel_drv_data *ddata;
    struct omap_dss_device *dssdev, *in;
    struct gpio_desc *gpio;
    int r;
    dev_dbg(&pdev.dev, "probe\n");
    if (node == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "Unable to find device tree\n");
    return -EINVAL;
    }
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    platform_set_drvdata(pdev, ddata);
    gpio = devm_gpiod_get_optional(&pdev.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    ddata.enable_gpio = gpio;
    in = omapdss_of_find_source_for_first_ep(node);
    if (IS_ERR(in)) {
    dev_err(&pdev.dev, "failed to find video source\n");
    return PTR_ERR(in);
    }
    ddata.in = in;
    dssdev = &ddata.dssdev;
    dssdev.ops.atv = &opa362_atv_ops;
    dssdev.dev = &pdev.dev;
    dssdev.type = OMAP_DISPLAY_TYPE_VENC;
    dssdev.output_type = OMAP_DISPLAY_TYPE_VENC;
    dssdev.owner = THIS_MODULE;
    r = omapdss_register_output(dssdev);
    if (r) {
    dev_err(&pdev.dev, "Failed to register output\n");
    goto err_reg;
    }
    return 0;
    err_reg:
    omap_dss_put_device(ddata.in);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn opa362_remove(pdev: *mut platform_device) {
    static void opa362_remove(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct omap_dss_device *dssdev = &ddata.dssdev;
    struct omap_dss_device *in = ddata.in;
    omapdss_unregister_output(&ddata.dssdev);
    WARN_ON(omapdss_device_is_enabled(dssdev));
    if (omapdss_device_is_enabled(dssdev))
    opa362_disable(dssdev);
    WARN_ON(omapdss_device_is_connected(dssdev));
    if (omapdss_device_is_connected(dssdev))
    opa362_disconnect(dssdev, dssdev.dst);
    omap_dss_put_device(in);
    }
    static const struct of_device_id opa362_of_match[] = {
    { .compatible = "omapdss,ti,opa362", },
    {},
    };
    MODULE_DEVICE_TABLE(of, opa362_of_match);
    static struct platform_driver opa362_driver = {
    .probe	= opa362_probe,
    .remove	= opa362_remove,
    .driver	= {
    .name	= "amplifier-opa362",
    .of_match_table = opa362_of_match,
    },
    };
    module_platform_driver(opa362_driver);
    MODULE_AUTHOR("H. Nikolaus Schaller <hns@goldelico.com>");
    MODULE_DESCRIPTION("OPA362 analog video amplifier with output/power control");
    MODULE_LICENSE("GPL v2");
