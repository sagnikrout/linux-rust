//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/displays/encoder-tpd12s015.c
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
// TPD12S015 HDMI ESD protection & level shifter chip driver
//
// Copyright (C) 2013 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_drv_data {
    pub dssdev: omap_dss_device,
    pub in: *mut omap_dss_device,
    pub ct_cp_hpd_gpio: *mut gpio_desc,
    pub ls_oe_gpio: *mut gpio_desc,
    pub hpd_gpio: *mut gpio_desc,
    pub timings: omap_video_timings,
}

    static int tpd_connect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    r = in.ops.hdmi.connect(in, dssdev);
    if (r)
    return r;
    dst.src = dssdev;
    dssdev.dst = dst;
    if (ddata.ct_cp_hpd_gpio) {
    gpiod_set_value_cansleep(ddata.ct_cp_hpd_gpio, 1);
// DC-DC converter needs at max 300us to get to 90% of 5V
    udelay(300);
    }
    return 0;
    }
    static void tpd_disconnect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    WARN_ON(dst != dssdev.dst);
    if (dst != dssdev.dst)
    return;
    gpiod_set_value_cansleep(ddata.ct_cp_hpd_gpio, 0);
    dst.src = core::ptr::null_mut();
    dssdev.dst = core::ptr::null_mut();
    in.ops.hdmi.disconnect(in, &ddata.dssdev);
    }
#[no_mangle]
unsafe extern "C" fn tpd_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int tpd_enable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    if (dssdev.state == OMAP_DSS_DISPLAY_ACTIVE)
    return 0;
    in.ops.hdmi.set_timings(in, &ddata.timings);
    r = in.ops.hdmi.enable(in);
    if (r)
    return r;
    dssdev.state = OMAP_DSS_DISPLAY_ACTIVE;
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tpd_disable(dssdev: *mut omap_dss_device) {
    static void tpd_disable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (dssdev.state != OMAP_DSS_DISPLAY_ACTIVE)
    return;
    in.ops.hdmi.disable(in);
    dssdev.state = OMAP_DSS_DISPLAY_DISABLED;
    }
    static void tpd_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    ddata.timings = *timings;
    dssdev.panel.timings = *timings;
    in.ops.hdmi.set_timings(in, timings);
    }
    static void tpd_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
// timings = ddata->timings;
    }
    static int tpd_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    r = in.ops.hdmi.check_timings(in, timings);
    return r;
    }
    static int tpd_read_edid(struct omap_dss_device *dssdev,
    u8 *edid, int len)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    int r;
    if (!gpiod_get_value_cansleep(ddata.hpd_gpio))
    return -ENODEV;
    gpiod_set_value_cansleep(ddata.ls_oe_gpio, 1);
    r = in.ops.hdmi.read_edid(in, edid, len);
    gpiod_set_value_cansleep(ddata.ls_oe_gpio, 0);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tpd_detect(dssdev: *mut omap_dss_device) -> bool {
    static bool tpd_detect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    return gpiod_get_value_cansleep(ddata.hpd_gpio);
    }
    static int tpd_set_infoframe(struct omap_dss_device *dssdev,
    const struct hdmi_avi_infoframe *avi)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.hdmi.set_infoframe(in, avi);
    }
    static int tpd_set_hdmi_mode(struct omap_dss_device *dssdev,
    bool hdmi_mode)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.hdmi.set_hdmi_mode(in, hdmi_mode);
    }
    static const struct omapdss_hdmi_ops tpd_hdmi_ops = {
    .connect		= tpd_connect,
    .disconnect		= tpd_disconnect,
    .enable			= tpd_enable,
    .disable		= tpd_disable,
    .check_timings		= tpd_check_timings,
    .set_timings		= tpd_set_timings,
    .get_timings		= tpd_get_timings,
    .read_edid		= tpd_read_edid,
    .detect			= tpd_detect,
    .set_infoframe		= tpd_set_infoframe,
    .set_hdmi_mode		= tpd_set_hdmi_mode,
    };
#[no_mangle]
unsafe extern "C" fn tpd_probe_of(pdev: *mut platform_device) -> c_int {
    static int tpd_probe_of(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    struct omap_dss_device *in;
    in = omapdss_of_find_source_for_first_ep(node);
    if (IS_ERR(in)) {
    dev_err(&pdev.dev, "failed to find video source\n");
    return PTR_ERR(in);
    }
    ddata.in = in;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpd_probe(pdev: *mut platform_device) -> c_int {
    static int tpd_probe(struct platform_device *pdev)
    {
    struct omap_dss_device *dssdev;
    struct panel_drv_data *ddata;
    int r;
    struct gpio_desc *gpio;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    platform_set_drvdata(pdev, ddata);
    if (pdev.dev.of_node) {
    r = tpd_probe_of(pdev);
    if (r)
    return r;
    } else {
    return -ENODEV;
    }
    gpio = devm_gpiod_get_index_optional(&pdev.dev, core::ptr::null_mut(), 0,
    GPIOD_OUT_LOW);
    if (IS_ERR(gpio)) {
    r = PTR_ERR(gpio);
    goto err_gpio;
    }
    ddata.ct_cp_hpd_gpio = gpio;
    gpio = devm_gpiod_get_index_optional(&pdev.dev, core::ptr::null_mut(), 1,
    GPIOD_OUT_LOW);
    if (IS_ERR(gpio)) {
    r = PTR_ERR(gpio);
    goto err_gpio;
    }
    ddata.ls_oe_gpio = gpio;
    gpio = devm_gpiod_get_index(&pdev.dev, core::ptr::null_mut(), 2,
    GPIOD_IN);
    if (IS_ERR(gpio)) {
    r = PTR_ERR(gpio);
    goto err_gpio;
    }
    ddata.hpd_gpio = gpio;
    dssdev = &ddata.dssdev;
    dssdev.ops.hdmi = &tpd_hdmi_ops;
    dssdev.dev = &pdev.dev;
    dssdev.type = OMAP_DISPLAY_TYPE_HDMI;
    dssdev.output_type = OMAP_DISPLAY_TYPE_HDMI;
    dssdev.owner = THIS_MODULE;
    dssdev.port_num = 1;
    r = omapdss_register_output(dssdev);
    if (r) {
    dev_err(&pdev.dev, "Failed to register output\n");
    goto err_reg;
    }
    return 0;
    err_reg:
    err_gpio:
    omap_dss_put_device(ddata.in);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tpd_remove(pdev: *mut platform_device) {
    static void tpd_remove(struct platform_device *pdev)
    {
    struct panel_drv_data *ddata = platform_get_drvdata(pdev);
    struct omap_dss_device *dssdev = &ddata.dssdev;
    struct omap_dss_device *in = ddata.in;
    omapdss_unregister_output(&ddata.dssdev);
    WARN_ON(omapdss_device_is_enabled(dssdev));
    if (omapdss_device_is_enabled(dssdev))
    tpd_disable(dssdev);
    WARN_ON(omapdss_device_is_connected(dssdev));
    if (omapdss_device_is_connected(dssdev))
    tpd_disconnect(dssdev, dssdev.dst);
    omap_dss_put_device(in);
    }
    static const struct of_device_id tpd_of_match[] = {
    { .compatible = "omapdss,ti,tpd12s015", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tpd_of_match);
    static struct platform_driver tpd_driver = {
    .probe	= tpd_probe,
    .remove	= tpd_remove,
    .driver	= {
    .name	= "tpd12s015",
    .of_match_table = tpd_of_match,
    },
    };
    module_platform_driver(tpd_driver);
    MODULE_AUTHOR("Tomi Valkeinen <tomi.valkeinen@ti.com>");
    MODULE_DESCRIPTION("TPD12S015 driver");
    MODULE_LICENSE("GPL");
