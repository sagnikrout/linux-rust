//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/displays/panel-tpo-td043mtea1.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TPO TD043MTEA1 Panel driver
//
// Author: Gražvydas Ignotas <notasas@gmail.com>
// Converted to new DSS device model: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

pub const TPO_R02_MODE_800x480: c_int = 7;

    TPO_R03_EN_VGL_PUMP |  TPO_R03_EN_PWM | \
    TPO_R03_DRIVING_CAP_100 | TPO_R03_EN_PRE_CHARGE | \
    TPO_R03_SOFTWARE_CTL)

    TPO_R03_EN_PRE_CHARGE | TPO_R03_SOFTWARE_CTL)
    static const u16 tpo_td043_def_gamma[12] = {
    105, 315, 381, 431, 490, 537, 579, 686, 780, 837, 880, 1023
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_drv_data {
    pub dssdev: omap_dss_device,
    pub in: *mut omap_dss_device,
    pub videomode: omap_video_timings,
    pub data_lines: c_int,
    pub spi: *mut spi_device,
    pub vcc_reg: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub gamma: [u16; 12],
    pub mode: u32,
    pub hmirror:1: u32,
    pub vmirror:1: u32,
    pub powered_on:1: u32,
    pub spi_suspended:1: u32,
    pub power_on_resume:1: u32,
}

    static const struct omap_video_timings tpo_td043_timings = {
    .x_res		= 800,
    .y_res		= 480,
    .pixelclock	= 36000000,
    .hsw		= 1,
    .hfp		= 68,
    .hbp		= 214,
    .vsw		= 1,
    .vfp		= 39,
    .vbp		= 34,
    .vsync_level	= OMAPDSS_SIG_ACTIVE_LOW,
    .hsync_level	= OMAPDSS_SIG_ACTIVE_LOW,
    .data_pclk_edge	= OMAPDSS_DRIVE_SIG_FALLING_EDGE,
    .de_level	= OMAPDSS_SIG_ACTIVE_HIGH,
    .sync_pclk_edge	= OMAPDSS_DRIVE_SIG_RISING_EDGE,
    };

#[no_mangle]
unsafe extern "C" fn tpo_td043_write(spi: *mut spi_device, addr: u8, data: u8) -> c_int {
    static int tpo_td043_write(struct spi_device *spi, u8 addr, u8 data)
    {
    struct spi_message	m;
    struct spi_transfer	xfer;
    u16			w;
    int			r;
    spi_message_init(&m);
    memset(&xfer, 0, sizeof(xfer));
    w = ((u16)addr << 10) | (1 << 8) | data;
    xfer.tx_buf = &w;
    xfer.bits_per_word = 16;
    xfer.len = 2;
    spi_message_add_tail(&xfer, &m);
    r = spi_sync(spi, &m);
    if (r < 0)
    dev_warn(&spi.dev, "failed to write to LCD reg (%d)\n", r);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_write_gamma(spi: *mut spi_device, gamma[12]: u16) {
    static void tpo_td043_write_gamma(struct spi_device *spi, u16 gamma[12])
    {
    u8 i, val;
// gamma bits [9:8]
    for (val = i = 0; i < 4; i++)
    val |= (gamma[i] & 0x300) >> ((i + 1) * 2);
    tpo_td043_write(spi, 0x11, val);
    for (val = i = 0; i < 4; i++)
    val |= (gamma[i+4] & 0x300) >> ((i + 1) * 2);
    tpo_td043_write(spi, 0x12, val);
    for (val = i = 0; i < 4; i++)
    val |= (gamma[i+8] & 0x300) >> ((i + 1) * 2);
    tpo_td043_write(spi, 0x13, val);
// gamma bits [7:0]
    for (val = i = 0; i < 12; i++)
    tpo_td043_write(spi, 0x14 + i, gamma[i] & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_write_mirror(spi: *mut spi_device, h: bool, v: bool) -> c_int {
    static int tpo_td043_write_mirror(struct spi_device *spi, bool h, bool v)
    {
    u8 reg4 = TPO_R04_NFLIP_H | TPO_R04_NFLIP_V |
    TPO_R04_CP_CLK_FREQ_1H | TPO_R04_VGL_FREQ_1H;
    if (h)
    reg4 &= ~TPO_R04_NFLIP_H;
    if (v)
    reg4 &= ~TPO_R04_NFLIP_V;
    return tpo_td043_write(spi, 4, reg4);
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_set_hmirror(dssdev: *mut omap_dss_device, enable: bool) -> c_int {
    static int tpo_td043_set_hmirror(struct omap_dss_device *dssdev, bool enable)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dssdev.dev);
    ddata.hmirror = enable;
    return tpo_td043_write_mirror(ddata.spi, ddata.hmirror,
    ddata.vmirror);
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_get_hmirror(dssdev: *mut omap_dss_device) -> bool {
    static bool tpo_td043_get_hmirror(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dssdev.dev);
    return ddata.hmirror;
    }
    static ssize_t tpo_td043_vmirror_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", ddata.vmirror);
    }
    static ssize_t tpo_td043_vmirror_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    int val;
    int ret;
    ret = kstrtoint(buf, 0, &val);
    if (ret < 0)
    return ret;
    val = !!val;
    ret = tpo_td043_write_mirror(ddata.spi, ddata.hmirror, val);
    if (ret < 0)
    return ret;
    ddata.vmirror = val;
    return count;
    }
    static ssize_t tpo_td043_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", ddata.mode);
    }
    static ssize_t tpo_td043_mode_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    long val;
    int ret;
    ret = kstrtol(buf, 0, &val);
    if (ret != 0 || val & ~7)
    return -EINVAL;
    ddata.mode = val;
    val |= TPO_R02_NCLK_RISING;
    tpo_td043_write(ddata.spi, 2, val);
    return count;
    }
    static ssize_t tpo_td043_gamma_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    let mut len: isize = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(ddata.gamma); i++)
    len += sysfs_emit_at(buf, len, "%u ", ddata.gamma[i]);
    if (len)
    buf[len - 1] = '\n';
    return len;
    }
    static ssize_t tpo_td043_gamma_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    unsigned int g[12];
    int ret;
    int i;
    ret = sscanf(buf, "%u %u %u %u %u %u %u %u %u %u %u %u",
    &g[0], &g[1], &g[2], &g[3], &g[4], &g[5],
    &g[6], &g[7], &g[8], &g[9], &g[10], &g[11]);
    if (ret != 12)
    return -EINVAL;
    for (i = 0; i < 12; i++)
    ddata.gamma[i] = g[i];
    tpo_td043_write_gamma(ddata.spi, ddata.gamma);
    return count;
    }
    static DEVICE_ATTR(vmirror, S_IRUGO | S_IWUSR,
    tpo_td043_vmirror_show, tpo_td043_vmirror_store);
    static DEVICE_ATTR(mode, S_IRUGO | S_IWUSR,
    tpo_td043_mode_show, tpo_td043_mode_store);
    static DEVICE_ATTR(gamma, S_IRUGO | S_IWUSR,
    tpo_td043_gamma_show, tpo_td043_gamma_store);
    static struct attribute *tpo_td043_attrs[] = {
    &dev_attr_vmirror.attr,
    &dev_attr_mode.attr,
    &dev_attr_gamma.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group tpo_td043_attr_group = {
    .attrs = tpo_td043_attrs,
    };
#[no_mangle]
unsafe extern "C" fn tpo_td043_power_on(ddata: *mut panel_drv_data) -> c_int {
    static int tpo_td043_power_on(struct panel_drv_data *ddata)
    {
    int r;
    if (ddata.powered_on)
    return 0;
    r = regulator_enable(ddata.vcc_reg);
    if (r != 0)
    return r;
// wait for panel to stabilize
    msleep(160);
    gpiod_set_value_cansleep(ddata.reset_gpio, 0);
    tpo_td043_write(ddata.spi, 2,
    TPO_R02_MODE(ddata.mode) | TPO_R02_NCLK_RISING);
    tpo_td043_write(ddata.spi, 3, TPO_R03_VAL_NORMAL);
    tpo_td043_write(ddata.spi, 0x20, 0xf0);
    tpo_td043_write(ddata.spi, 0x21, 0xf0);
    tpo_td043_write_mirror(ddata.spi, ddata.hmirror,
    ddata.vmirror);
    tpo_td043_write_gamma(ddata.spi, ddata.gamma);
    ddata.powered_on = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_power_off(ddata: *mut panel_drv_data) {
    static void tpo_td043_power_off(struct panel_drv_data *ddata)
    {
    if (!ddata.powered_on)
    return;
    tpo_td043_write(ddata.spi, 3,
    TPO_R03_VAL_STANDBY | TPO_R03_EN_PWM);
    gpiod_set_value_cansleep(ddata.reset_gpio, 1);
// wait for at least 2 vsyncs before cutting off power
    msleep(50);
    tpo_td043_write(ddata.spi, 3, TPO_R03_VAL_STANDBY);
    regulator_disable(ddata.vcc_reg);
    ddata.powered_on = 0;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_connect(dssdev: *mut omap_dss_device) -> c_int {
    static int tpo_td043_connect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (omapdss_device_is_connected(dssdev))
    return 0;
    return in.ops.dpi.connect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_disconnect(dssdev: *mut omap_dss_device) {
    static void tpo_td043_disconnect(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (!omapdss_device_is_connected(dssdev))
    return;
    in.ops.dpi.disconnect(in, dssdev);
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int tpo_td043_enable(struct omap_dss_device *dssdev)
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
//
// If we are resuming from system suspend, SPI clocks might not be
// enabled yet, so we'll program the LCD from SPI PM resume callback.
//
    if (!ddata.spi_suspended) {
    r = tpo_td043_power_on(ddata);
    if (r) {
    in.ops.dpi.disable(in);
    return r;
    }
    }
    dssdev.state = OMAP_DSS_DISPLAY_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_disable(dssdev: *mut omap_dss_device) {
    static void tpo_td043_disable(struct omap_dss_device *dssdev)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    if (!omapdss_device_is_enabled(dssdev))
    return;
    in.ops.dpi.disable(in);
    if (!ddata.spi_suspended)
    tpo_td043_power_off(ddata);
    dssdev.state = OMAP_DSS_DISPLAY_DISABLED;
    }
    static void tpo_td043_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    ddata.videomode = *timings;
    dssdev.panel.timings = *timings;
    in.ops.dpi.set_timings(in, timings);
    }
    static void tpo_td043_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
// timings = ddata->videomode;
    }
    static int tpo_td043_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    struct panel_drv_data *ddata = to_panel_data(dssdev);
    struct omap_dss_device *in = ddata.in;
    return in.ops.dpi.check_timings(in, timings);
    }
    static struct omap_dss_driver tpo_td043_ops = {
    .connect	= tpo_td043_connect,
    .disconnect	= tpo_td043_disconnect,
    .enable		= tpo_td043_enable,
    .disable	= tpo_td043_disable,
    .set_timings	= tpo_td043_set_timings,
    .get_timings	= tpo_td043_get_timings,
    .check_timings	= tpo_td043_check_timings,
    .set_mirror	= tpo_td043_set_hmirror,
    .get_mirror	= tpo_td043_get_hmirror,
    .get_resolution	= omapdss_default_get_resolution,
    };
#[no_mangle]
unsafe extern "C" fn tpo_td043_probe(spi: *mut spi_device) -> c_int {
    static int tpo_td043_probe(struct spi_device *spi)
    {
    struct panel_drv_data *ddata;
    struct omap_dss_device *dssdev;
    int r;
    dev_dbg(&spi.dev, "%s\n", __func__);
    if (!spi.dev.of_node)
    return -ENODEV;
    spi.bits_per_word = 16;
    spi.mode = SPI_MODE_0;
    r = spi_setup(spi);
    if (r < 0) {
    dev_err(&spi.dev, "spi_setup failed: %d\n", r);
    return r;
    }
    ddata = devm_kzalloc(&spi.dev, sizeof(*ddata), GFP_KERNEL);
    if (ddata == core::ptr::null_mut())
    return -ENOMEM;
    dev_set_drvdata(&spi.dev, ddata);
    ddata.spi = spi;
    ddata.in = omapdss_of_find_source_for_first_ep(spi.dev.of_node);
    r = PTR_ERR_OR_ZERO(ddata.in);
    if (r) {
    dev_err(&spi.dev, "failed to find video source: %d\n", r);
    return r;
    }
    ddata.mode = TPO_R02_MODE_800x480;
    memcpy(ddata.gamma, tpo_td043_def_gamma, sizeof(ddata.gamma));
    ddata.vcc_reg = devm_regulator_get(&spi.dev, "vcc");
    if (IS_ERR(ddata.vcc_reg)) {
    r = dev_err_probe(&spi.dev, PTR_ERR(ddata.vcc_reg),
    "failed to get LCD VCC regulator\n");
    goto err_regulator;
    }
    ddata.reset_gpio = devm_gpiod_get(&spi.dev, "reset", GPIOD_OUT_HIGH);
    r = PTR_ERR_OR_ZERO(ddata.reset_gpio);
    if (r) {
    dev_err(&spi.dev, "couldn't request reset GPIO\n");
    goto err_gpio_req;
    }
    gpiod_set_consumer_name(ddata.reset_gpio, "lcd reset");
    r = sysfs_create_group(&spi.dev.kobj, &tpo_td043_attr_group);
    if (r) {
    dev_err(&spi.dev, "failed to create sysfs files\n");
    goto err_sysfs;
    }
    ddata.videomode = tpo_td043_timings;
    dssdev = &ddata.dssdev;
    dssdev.dev = &spi.dev;
    dssdev.driver = &tpo_td043_ops;
    dssdev.type = OMAP_DISPLAY_TYPE_DPI;
    dssdev.owner = THIS_MODULE;
    dssdev.panel.timings = ddata.videomode;
    r = omapdss_register_display(dssdev);
    if (r) {
    dev_err(&spi.dev, "Failed to register panel\n");
    goto err_reg;
    }
    return 0;
    err_reg:
    sysfs_remove_group(&spi.dev.kobj, &tpo_td043_attr_group);
    err_sysfs:
    err_gpio_req:
    err_regulator:
    omap_dss_put_device(ddata.in);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_remove(spi: *mut spi_device) {
    static void tpo_td043_remove(struct spi_device *spi)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(&spi.dev);
    struct omap_dss_device *dssdev = &ddata.dssdev;
    struct omap_dss_device *in = ddata.in;
    dev_dbg(&ddata.spi.dev, "%s\n", __func__);
    omapdss_unregister_display(dssdev);
    tpo_td043_disable(dssdev);
    tpo_td043_disconnect(dssdev);
    omap_dss_put_device(in);
    sysfs_remove_group(&spi.dev.kobj, &tpo_td043_attr_group);
    }

#[no_mangle]
unsafe extern "C" fn tpo_td043_spi_suspend(dev: *mut device) -> c_int {
    static int tpo_td043_spi_suspend(struct device *dev)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    dev_dbg(dev, "tpo_td043_spi_suspend, tpo %p\n", ddata);
    ddata.power_on_resume = ddata.powered_on;
    tpo_td043_power_off(ddata);
    ddata.spi_suspended = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpo_td043_spi_resume(dev: *mut device) -> c_int {
    static int tpo_td043_spi_resume(struct device *dev)
    {
    struct panel_drv_data *ddata = dev_get_drvdata(dev);
    int ret;
    dev_dbg(dev, "tpo_td043_spi_resume\n");
    if (ddata.power_on_resume) {
    ret = tpo_td043_power_on(ddata);
    if (ret)
    return ret;
    }
    ddata.spi_suspended = 0;
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(tpo_td043_spi_pm,
    tpo_td043_spi_suspend, tpo_td043_spi_resume);
    static const struct of_device_id tpo_td043_of_match[] = {
    { .compatible = "omapdss,tpo,td043mtea1", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tpo_td043_of_match);
    static struct spi_driver tpo_td043_spi_driver = {
    .driver = {
    .name	= "panel-tpo-td043mtea1",
    .pm	= &tpo_td043_spi_pm,
    .of_match_table = tpo_td043_of_match,
    .suppress_bind_attrs = true,
    },
    .probe	= tpo_td043_probe,
    .remove	= tpo_td043_remove,
    };
    module_spi_driver(tpo_td043_spi_driver);
    MODULE_ALIAS("spi:tpo,td043mtea1");
    MODULE_AUTHOR("Gražvydas Ignotas <notasas@gmail.com>");
    MODULE_DESCRIPTION("TPO TD043MTEA1 LCD Driver");
    MODULE_LICENSE("GPL");
