//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/parade-ps8622.c
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
// Parade PS8622 eDP/LVDS bridge driver
//
// Copyright (C) 2014 Google, Inc.
//

// Brightness scale on the Parade chip
pub const PS8622_MAX_BRIGHTNESS: c_uint = 0xff;
// Timings taken from the version 1.7 datasheet for the PS8622/PS8625
pub const PS8622_POWER_RISE_T1_MIN_US: c_int = 10;
pub const PS8622_POWER_RISE_T1_MAX_US: c_int = 10000;
pub const PS8622_RST_HIGH_T2_MIN_US: c_int = 3000;
pub const PS8622_RST_HIGH_T2_MAX_US: c_int = 30000;
pub const PS8622_PWMO_END_T12_MS: c_int = 200;
pub const PS8622_POWER_FALL_T16_MAX_US: c_int = 10000;
pub const PS8622_POWER_OFF_T17_MS: c_int = 500;

    (PS8622_RST_HIGH_T2_MAX_US + PS8622_POWER_RISE_T1_MIN_US))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps8622_bridge {
    pub client: *mut i2c_client,
    pub bridge: drm_bridge,
    pub panel_bridge: *mut drm_bridge,
    pub v12: *mut regulator,
    pub bl: *mut backlight_device,
    pub gpio_slp: *mut gpio_desc,
    pub gpio_rst: *mut gpio_desc,
    pub max_lane_count: u32,
    pub lane_count: u32,
    pub enabled: bool,
}

    static inline struct ps8622_bridge *
    bridge_to_ps8622(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct ps8622_bridge, bridge);
    }
#[no_mangle]
unsafe extern "C" fn ps8622_set(client: *mut i2c_client, page: u8, reg: u8, val: u8) -> c_int {
    static int ps8622_set(struct i2c_client *client, u8 page, u8 reg, u8 val)
    {
    int ret;
    struct i2c_adapter *adap = client.adapter;
    struct i2c_msg msg;
    u8 data[] = {reg, val};
    msg.addr = client.addr + page;
    msg.flags = 0;
    msg.len = sizeof(data);
    msg.buf = data;
    ret = i2c_transfer(adap, &msg, 1);
    if (ret != 1)
    pr_warn("PS8622 I2C write (0x%02x,0x%02x,0x%02x) failed: %d\n",
    client.addr + page, reg, val, ret);
    return !(ret == 1);
    }
#[no_mangle]
unsafe extern "C" fn ps8622_send_config(ps8622: *mut ps8622_bridge) -> c_int {
    static int ps8622_send_config(struct ps8622_bridge *ps8622)
    {
    struct i2c_client *cl = ps8622.client;
    let mut err: c_int = 0;
// HPD low
    err = ps8622_set(cl, 0x02, 0xa1, 0x01);
    if (err)
    goto error;
// SW setting: [1:0] SW output 1.2V voltage is lower to 96%
    err = ps8622_set(cl, 0x04, 0x14, 0x01);
    if (err)
    goto error;
// RCO SS setting: [5:4] = b01 0.5%, b10 1%, b11 1.5%
    err = ps8622_set(cl, 0x04, 0xe3, 0x20);
    if (err)
    goto error;
// [7] RCO SS enable
    err = ps8622_set(cl, 0x04, 0xe2, 0x80);
    if (err)
    goto error;
// RPHY Setting
// [3:2] CDR tune wait cycle before measure for fine tune
// b00: 1us b01: 0.5us b10:2us, b11: 4us
//
    err = ps8622_set(cl, 0x04, 0x8a, 0x0c);
    if (err)
    goto error;
// [3] RFD always on
    err = ps8622_set(cl, 0x04, 0x89, 0x08);
    if (err)
    goto error;
// CTN lock in/out: 20000ppm/80000ppm. Lock out 2 times.
    err = ps8622_set(cl, 0x04, 0x71, 0x2d);
    if (err)
    goto error;
// 2.7G CDR settings: NOF=40LSB for HBR CDR  setting
    err = ps8622_set(cl, 0x04, 0x7d, 0x07);
    if (err)
    goto error;
// [1:0] Fmin=+4bands
    err = ps8622_set(cl, 0x04, 0x7b, 0x00);
    if (err)
    goto error;
// [7:5] DCO_FTRNG=+-40%
    err = ps8622_set(cl, 0x04, 0x7a, 0xfd);
    if (err)
    goto error;
// 1.62G CDR settings: [5:2]NOF=64LSB [1:0]DCO scale is 2/5
    err = ps8622_set(cl, 0x04, 0xc0, 0x12);
    if (err)
    goto error;
// Gitune=-37%
    err = ps8622_set(cl, 0x04, 0xc1, 0x92);
    if (err)
    goto error;
// Fbstep=100%
    err = ps8622_set(cl, 0x04, 0xc2, 0x1c);
    if (err)
    goto error;
// [7] LOS signal disable
    err = ps8622_set(cl, 0x04, 0x32, 0x80);
    if (err)
    goto error;
// RPIO Setting: [7:4] LVDS driver bias current : 75% (250mV swing)
    err = ps8622_set(cl, 0x04, 0x00, 0xb0);
    if (err)
    goto error;
// [7:6] Right-bar GPIO output strength is 8mA
    err = ps8622_set(cl, 0x04, 0x15, 0x40);
    if (err)
    goto error;
// EQ Training State Machine Setting, RCO calibration start
    err = ps8622_set(cl, 0x04, 0x54, 0x10);
    if (err)
    goto error;
// Logic, needs more than 10 I2C command
// [4:0] MAX_LANE_COUNT set to max supported lanes
    err = ps8622_set(cl, 0x01, 0x02, 0x80 | ps8622.max_lane_count);
    if (err)
    goto error;
// [4:0] LANE_COUNT_SET set to chosen lane count
    err = ps8622_set(cl, 0x01, 0x21, 0x80 | ps8622.lane_count);
    if (err)
    goto error;
    err = ps8622_set(cl, 0x00, 0x52, 0x20);
    if (err)
    goto error;
// HPD CP toggle enable
    err = ps8622_set(cl, 0x00, 0xf1, 0x03);
    if (err)
    goto error;
    err = ps8622_set(cl, 0x00, 0x62, 0x41);
    if (err)
    goto error;
// Counter number, add 1ms counter delay
    err = ps8622_set(cl, 0x00, 0xf6, 0x01);
    if (err)
    goto error;
// [6]PWM function control by DPCD0040f[7], default is PWM block
    err = ps8622_set(cl, 0x00, 0x77, 0x06);
    if (err)
    goto error;
// 04h Adjust VTotal toleranceto fix the 30Hz no display issue
    err = ps8622_set(cl, 0x00, 0x4c, 0x04);
    if (err)
    goto error;
// DPCD00400='h00, Parade OUI ='h001cf8
    err = ps8622_set(cl, 0x01, 0xc0, 0x00);
    if (err)
    goto error;
// DPCD00401='h1c
    err = ps8622_set(cl, 0x01, 0xc1, 0x1c);
    if (err)
    goto error;
// DPCD00402='hf8
    err = ps8622_set(cl, 0x01, 0xc2, 0xf8);
    if (err)
    goto error;
// DPCD403~408 = ASCII code, D2SLV5='h4432534c5635
    err = ps8622_set(cl, 0x01, 0xc3, 0x44);
    if (err)
    goto error;
// DPCD404
    err = ps8622_set(cl, 0x01, 0xc4, 0x32);
    if (err)
    goto error;
// DPCD405
    err = ps8622_set(cl, 0x01, 0xc5, 0x53);
    if (err)
    goto error;
// DPCD406
    err = ps8622_set(cl, 0x01, 0xc6, 0x4c);
    if (err)
    goto error;
// DPCD407
    err = ps8622_set(cl, 0x01, 0xc7, 0x56);
    if (err)
    goto error;
// DPCD408
    err = ps8622_set(cl, 0x01, 0xc8, 0x35);
    if (err)
    goto error;
// DPCD40A, Initial Code major revision '01'
    err = ps8622_set(cl, 0x01, 0xca, 0x01);
    if (err)
    goto error;
// DPCD40B, Initial Code minor revision '05'
    err = ps8622_set(cl, 0x01, 0xcb, 0x05);
    if (err)
    goto error;
    if (ps8622.bl) {
// DPCD720, internal PWM
    err = ps8622_set(cl, 0x01, 0xa5, 0xa0);
    if (err)
    goto error;
// FFh for 100% brightness, 0h for 0% brightness
    err = ps8622_set(cl, 0x01, 0xa7,
    ps8622.bl.props.brightness);
    if (err)
    goto error;
    } else {
// DPCD720, external PWM
    err = ps8622_set(cl, 0x01, 0xa5, 0x80);
    if (err)
    goto error;
    }
// Set LVDS output as 6bit-VESA mapping, single LVDS channel
    err = ps8622_set(cl, 0x01, 0xcc, 0x13);
    if (err)
    goto error;
// Enable SSC set by register
    err = ps8622_set(cl, 0x02, 0xb1, 0x20);
    if (err)
    goto error;
// Set SSC enabled and +/-1% central spreading
    err = ps8622_set(cl, 0x04, 0x10, 0x16);
    if (err)
    goto error;
// Logic end
// MPU Clock source: LC => RCO
    err = ps8622_set(cl, 0x04, 0x59, 0x60);
    if (err)
    goto error;
// LC -> RCO
    err = ps8622_set(cl, 0x04, 0x54, 0x14);
    if (err)
    goto error;
// HPD high
    err = ps8622_set(cl, 0x02, 0xa1, 0x91);
    error:
    return err ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn ps8622_backlight_update(bl: *mut backlight_device) -> c_int {
    static int ps8622_backlight_update(struct backlight_device *bl)
    {
    struct ps8622_bridge *ps8622 = dev_get_drvdata(&bl.dev);
    int ret, brightness = backlight_get_brightness(bl);
    if (!ps8622.enabled)
    return -EINVAL;
    ret = ps8622_set(ps8622.client, 0x01, 0xa7, brightness);
    return ret;
    }
    static const struct backlight_ops ps8622_backlight_ops = {
    .update_status	= ps8622_backlight_update,
    };
    static void ps8622_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct ps8622_bridge *ps8622 = bridge_to_ps8622(bridge);
    int ret;
    if (ps8622.enabled)
    return;
    gpiod_set_value(ps8622.gpio_rst, 0);
    if (ps8622.v12) {
    ret = regulator_enable(ps8622.v12);
    if (ret)
    DRM_ERROR("fails to enable ps8622.v12");
    }
    gpiod_set_value(ps8622.gpio_slp, 1);
//
// T1 is the range of time that it takes for the power to rise after we
// enable the lcd/ps8622 fet. T2 is the range of time in which the
// data sheet specifies we should deassert the reset pin.
//
// If it takes T1.max for the power to rise, we need to wait atleast
// T2.min before deasserting the reset pin. If it takes T1.min for the
// power to rise, we need to wait at most T2.max before deasserting the
// reset pin.
//
    usleep_range(PS8622_RST_HIGH_T2_MIN_US + PS8622_POWER_RISE_T1_MAX_US,
    PS8622_RST_HIGH_T2_MAX_US + PS8622_POWER_RISE_T1_MIN_US);
    gpiod_set_value(ps8622.gpio_rst, 1);
// wait 20ms after RST high
    usleep_range(20000, 30000);
    ret = ps8622_send_config(ps8622);
    if (ret) {
    DRM_ERROR("Failed to send config to bridge (%d)\n", ret);
    return;
    }
    ps8622.enabled = true;
    }
    static void ps8622_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
// Delay after panel is disabled
    msleep(PS8622_PWMO_END_T12_MS);
    }
    static void ps8622_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct ps8622_bridge *ps8622 = bridge_to_ps8622(bridge);
    if (!ps8622.enabled)
    return;
    ps8622.enabled = false;
//
// This doesn't matter if the regulators are turned off, but something
// else might keep them on. In that case, we want to assert the slp gpio
// to lower power.
//
    gpiod_set_value(ps8622.gpio_slp, 0);
    if (ps8622.v12)
    regulator_disable(ps8622.v12);
//
// Sleep for at least the amount of time that it takes the power rail to
// fall to prevent asserting the rst gpio from doing anything.
//
    usleep_range(PS8622_POWER_FALL_T16_MAX_US,
    2 * PS8622_POWER_FALL_T16_MAX_US);
    gpiod_set_value(ps8622.gpio_rst, 0);
    msleep(PS8622_POWER_OFF_T17_MS);
    }
    static int ps8622_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct ps8622_bridge *ps8622 = bridge_to_ps8622(bridge);
    return drm_bridge_attach(ps8622.bridge.encoder, ps8622.panel_bridge,
    &ps8622.bridge, flags);
    }
    static const struct drm_bridge_funcs ps8622_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_pre_enable = ps8622_pre_enable,
    .atomic_disable = ps8622_disable,
    .atomic_post_disable = ps8622_post_disable,
    .attach = ps8622_attach,
    };
    static const struct of_device_id ps8622_devices[] = {
    {.compatible = "parade,ps8622",},
    {.compatible = "parade,ps8625",},
    {}
    };
    MODULE_DEVICE_TABLE(of, ps8622_devices);
#[no_mangle]
unsafe extern "C" fn ps8622_probe(client: *mut i2c_client) -> c_int {
    static int ps8622_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct device *dev = &client.dev;
    struct ps8622_bridge *ps8622;
    struct drm_bridge *panel_bridge;
    int ret;
    ps8622 = devm_drm_bridge_alloc(dev, struct ps8622_bridge, bridge,
    &ps8622_bridge_funcs);
    if (IS_ERR(ps8622))
    return PTR_ERR(ps8622);
    panel_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 0, 0);
    if (IS_ERR(panel_bridge))
    return PTR_ERR(panel_bridge);
    ps8622.panel_bridge = panel_bridge;
    ps8622.client = client;
    ps8622.v12 = devm_regulator_get(dev, "vdd12");
    if (IS_ERR(ps8622.v12)) {
    dev_info(dev, "no 1.2v regulator found for PS8622\n");
    ps8622.v12 = core::ptr::null_mut();
    }
    ps8622.gpio_slp = devm_gpiod_get(dev, "sleep", GPIOD_OUT_HIGH);
    if (IS_ERR(ps8622.gpio_slp)) {
    ret = PTR_ERR(ps8622.gpio_slp);
    dev_err(dev, "cannot get gpio_slp %d\n", ret);
    return ret;
    }
//
// Assert the reset pin high to avoid the bridge being
// initialized prematurely
//
    ps8622.gpio_rst = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ps8622.gpio_rst)) {
    ret = PTR_ERR(ps8622.gpio_rst);
    dev_err(dev, "cannot get gpio_rst %d\n", ret);
    return ret;
    }
    ps8622.max_lane_count = id.driver_data;
    if (of_property_read_u32(dev.of_node, "lane-count",
    &ps8622.lane_count)) {
    ps8622.lane_count = ps8622.max_lane_count;
    } else if (ps8622.lane_count > ps8622.max_lane_count) {
    dev_info(dev, "lane-count property is too high,"
    "using max_lane_count\n");
    ps8622.lane_count = ps8622.max_lane_count;
    }
    if (!of_property_read_bool(dev.of_node, "use-external-pwm")) {
    ps8622.bl = backlight_device_register("ps8622-backlight",
    dev, ps8622, &ps8622_backlight_ops,
    core::ptr::null_mut());
    if (IS_ERR(ps8622.bl)) {
    DRM_ERROR("failed to register backlight\n");
    ret = PTR_ERR(ps8622.bl);
    ps8622.bl = core::ptr::null_mut();
    return ret;
    }
    ps8622.bl.props.max_brightness = PS8622_MAX_BRIGHTNESS;
    ps8622.bl.props.brightness = PS8622_MAX_BRIGHTNESS;
    }
    ps8622.bridge.type = DRM_MODE_CONNECTOR_LVDS;
    ps8622.bridge.of_node = dev.of_node;
    drm_bridge_add(&ps8622.bridge);
    i2c_set_clientdata(client, ps8622);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps8622_remove(client: *mut i2c_client) {
    static void ps8622_remove(struct i2c_client *client)
    {
    struct ps8622_bridge *ps8622 = i2c_get_clientdata(client);
    backlight_device_unregister(ps8622.bl);
    drm_bridge_remove(&ps8622.bridge);
    }
    static const struct i2c_device_id ps8622_i2c_table[] = {
// Device type, driver_data holds the maximal lane_count
    { .name = "ps8622", .driver_data = 1 },
    { .name = "ps8625", .driver_data = 2 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ps8622_i2c_table);
    static struct i2c_driver ps8622_driver = {
    .id_table	= ps8622_i2c_table,
    .probe		= ps8622_probe,
    .remove		= ps8622_remove,
    .driver		= {
    .name	= "ps8622",
    .of_match_table = ps8622_devices,
    },
    };
    module_i2c_driver(ps8622_driver);
    MODULE_AUTHOR("Vincent Palatin <vpalatin@chromium.org>");
    MODULE_DESCRIPTION("Parade ps8622/ps8625 eDP-LVDS converter driver");
    MODULE_LICENSE("GPL v2");
