//! Automatically rewritten from C to Rust
//! Source: drivers/hid/i2c-hid/i2c-hid-of.c
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


//
// HID over I2C Open Firmware Subclass
//
// Copyright (c) 2012 Benjamin Tissoires <benjamin.tissoires@gmail.com>
// Copyright (c) 2012 Ecole Nationale de l'Aviation Civile, France
// Copyright (c) 2012 Red Hat, Inc
//
// This code was forked out of the core code, which was partly based on
// "USB HID support for Linux":
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2007-2008 Oliver Neukum
// Copyright (c) 2006-2010 Jiri Kosina
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_hid_of {
    pub ops: i2chid_ops,
    pub client: *mut i2c_client,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; 2],
    pub post_power_delay_ms: c_int,
    pub post_reset_delay_ms: c_int,
}

#[no_mangle]
unsafe extern "C" fn i2c_hid_of_power_up(ops: *mut i2chid_ops) -> c_int {
    static int i2c_hid_of_power_up(struct i2chid_ops *ops)
    {
    struct i2c_hid_of *ihid_of = container_of(ops, struct i2c_hid_of, ops);
    struct device *dev = &ihid_of.client.dev;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ihid_of.supplies),
    ihid_of.supplies);
    if (ret) {
    dev_warn(dev, "Failed to enable supplies: %d\n", ret);
    return ret;
    }
    if (ihid_of.post_power_delay_ms)
    msleep(ihid_of.post_power_delay_ms);
    gpiod_set_value_cansleep(ihid_of.reset_gpio, 0);
    if (ihid_of.post_reset_delay_ms)
    msleep(ihid_of.post_reset_delay_ms);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_hid_of_power_down(ops: *mut i2chid_ops) {
    static void i2c_hid_of_power_down(struct i2chid_ops *ops)
    {
    struct i2c_hid_of *ihid_of = container_of(ops, struct i2c_hid_of, ops);
    gpiod_set_value_cansleep(ihid_of.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(ihid_of.supplies),
    ihid_of.supplies);
    }
#[no_mangle]
unsafe extern "C" fn i2c_hid_of_probe(client: *mut i2c_client) -> c_int {
    static int i2c_hid_of_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct i2c_hid_of *ihid_of;
    u16 hid_descriptor_address;
    let mut quirks: u32 = 0;
    int ret;
    u32 val;
    ihid_of = devm_kzalloc(dev, sizeof(*ihid_of), GFP_KERNEL);
    if (!ihid_of)
    return -ENOMEM;
    ihid_of.client = client;
    ihid_of.ops.power_up = i2c_hid_of_power_up;
    ihid_of.ops.power_down = i2c_hid_of_power_down;
    ret = device_property_read_u32(dev, "hid-descr-addr", &val);
    if (ret) {
    dev_err(dev, "HID register address not provided\n");
    return -ENODEV;
    }
    if (val >> 16) {
    dev_err(dev, "Bad HID register address: 0x%08x\n", val);
    return -EINVAL;
    }
    hid_descriptor_address = val;
    if (!device_property_read_u32(dev, "post-power-on-delay-ms", &val))
    ihid_of.post_power_delay_ms = val;
//
// Note this is a kernel internal device-property set by x86 platform code,
// this MUST not be used in devicetree files without first adding it to
// the DT bindings.
//
    if (!device_property_read_u32(dev, "post-reset-deassert-delay-ms", &val))
    ihid_of.post_reset_delay_ms = val;
// Start out with reset asserted
    ihid_of.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ihid_of.reset_gpio))
    return PTR_ERR(ihid_of.reset_gpio);
    ihid_of.supplies[0].supply = "vdd";
    ihid_of.supplies[1].supply = "vddl";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(ihid_of.supplies),
    ihid_of.supplies);
    if (ret)
    return ret;
    if (device_property_read_bool(dev, "touchscreen-inverted-x"))
    quirks |= HID_QUIRK_X_INVERT;
    if (device_property_read_bool(dev, "touchscreen-inverted-y"))
    quirks |= HID_QUIRK_Y_INVERT;
    return i2c_hid_core_probe(client, &ihid_of.ops,
    hid_descriptor_address, quirks);
    }

    static const struct of_device_id i2c_hid_of_match[] = {
    { .compatible = "hid-over-i2c" },
    {},
    };
    MODULE_DEVICE_TABLE(of, i2c_hid_of_match);

    static const struct i2c_device_id i2c_hid_of_id_table[] = {
    { .name = "hid" },
    { .name = "hid-over-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, i2c_hid_of_id_table);
    static struct i2c_driver i2c_hid_of_driver = {
    .driver = {
    .name	= "i2c_hid_of",
    .pm	= &i2c_hid_core_pm,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(i2c_hid_of_match),
    },
    .probe		= i2c_hid_of_probe,
    .remove		= i2c_hid_core_remove,
    .shutdown	= i2c_hid_core_shutdown,
    .id_table	= i2c_hid_of_id_table,
    };
    module_i2c_driver(i2c_hid_of_driver);
    MODULE_DESCRIPTION("HID over I2C OF driver");
    MODULE_AUTHOR("Benjamin Tissoires <benjamin.tissoires@gmail.com>");
    MODULE_LICENSE("GPL");
