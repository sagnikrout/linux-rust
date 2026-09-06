//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/adc-keys.c
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
// Input driver for resistor ladder connected on ADC
//
// Copyright (c) 2016 Alexandre Belloni
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_keys_button {
    pub voltage: u32,
    pub keycode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_keys_state {
    pub channel: *mut iio_channel,
    pub num_keys: u32,
    pub last_key: u32,
    pub keyup_voltage: u32,
    pub map: *const adc_keys_button,
}

#[no_mangle]
unsafe extern "C" fn adc_keys_poll(input: *mut input_dev) {
    static void adc_keys_poll(struct input_dev *input)
    {
    struct adc_keys_state *st = input_get_drvdata(input);
    int i, value, ret;
    u32 diff, closest = 0xffffffff;
    let mut keycode: c_int = 0;
    ret = iio_read_channel_processed(st.channel, &value);
    if (unlikely(ret < 0)) {
// Forcibly release key if any was pressed
    value = st.keyup_voltage;
    } else {
    for (i = 0; i < st.num_keys; i++) {
    diff = abs(st.map[i].voltage - value);
    if (diff < closest) {
    closest = diff;
    keycode = st.map[i].keycode;
    }
    }
    }
    if (abs(st.keyup_voltage - value) < closest)
    keycode = 0;
    if (st.last_key && st.last_key != keycode)
    input_report_key(input, st.last_key, 0);
    if (keycode)
    input_report_key(input, keycode, 1);
    input_sync(input);
    st.last_key = keycode;
    }
#[no_mangle]
unsafe extern "C" fn adc_keys_load_keymap(dev: *mut device, st: *mut adc_keys_state) -> c_int {
    static int adc_keys_load_keymap(struct device *dev, struct adc_keys_state *st)
    {
    struct adc_keys_button *map;
    int i;
    st.num_keys = device_get_child_node_count(dev);
    if (st.num_keys == 0) {
    dev_err(dev, "keymap is missing\n");
    return -EINVAL;
    }
    map = devm_kmalloc_array(dev, st.num_keys, sizeof(*map), GFP_KERNEL);
    if (!map)
    return -ENOMEM;
    i = 0;
    device_for_each_child_node_scoped(dev, child) {
    if (fwnode_property_read_u32(child, "press-threshold-microvolt",
    &map[i].voltage)) {
    dev_err(dev, "Key with invalid or missing voltage\n");
    return -EINVAL;
    }
    map[i].voltage /= 1000;
    if (fwnode_property_read_u32(child, "linux,code",
    &map[i].keycode)) {
    dev_err(dev, "Key with invalid or missing linux,code\n");
    return -EINVAL;
    }
    i++;
    }
    st.map = map;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc_keys_probe(pdev: *mut platform_device) -> c_int {
    static int adc_keys_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct adc_keys_state *st;
    struct input_dev *input;
    enum iio_chan_type type;
    int i, value;
    int error;
    st = devm_kzalloc(dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    st.channel = devm_iio_channel_get(dev, "buttons");
    if (IS_ERR(st.channel))
    return PTR_ERR(st.channel);
    if (!st.channel.indio_dev)
    return -ENXIO;
    error = iio_get_channel_type(st.channel, &type);
    if (error < 0)
    return error;
    if (type != IIO_VOLTAGE) {
    dev_err(dev, "Incompatible channel type %d\n", type);
    return -EINVAL;
    }
    if (device_property_read_u32(dev, "keyup-threshold-microvolt",
    &st.keyup_voltage)) {
    dev_err(dev, "Invalid or missing keyup voltage\n");
    return -EINVAL;
    }
    st.keyup_voltage /= 1000;
    error = adc_keys_load_keymap(dev, st);
    if (error)
    return error;
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    input_set_drvdata(input, st);
    input.name = pdev.name;
    input.phys = "adc-keys/input0";
    input.id.bustype = BUS_HOST;
    input.id.vendor = 0x0001;
    input.id.product = 0x0001;
    input.id.version = 0x0100;
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < st.num_keys; i++)
    __set_bit(st.map[i].keycode, input.keybit);
    if (device_property_read_bool(dev, "autorepeat"))
    __set_bit(EV_REP, input.evbit);
    error = input_setup_polling(input, adc_keys_poll);
    if (error) {
    dev_err(dev, "Unable to set up polling: %d\n", error);
    return error;
    }
    if (!device_property_read_u32(dev, "poll-interval", &value))
    input_set_poll_interval(input, value);
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "Unable to register input device: %d\n", error);
    return error;
    }
    return 0;
    }

    static const struct of_device_id adc_keys_of_match[] = {
    { .compatible = "adc-keys", },
    { }
    };
    MODULE_DEVICE_TABLE(of, adc_keys_of_match);

    static struct platform_driver adc_keys_driver = {
    .driver = {
    .name = "adc_keys",
    .of_match_table = of_match_ptr(adc_keys_of_match),
    },
    .probe = adc_keys_probe,
    };
    module_platform_driver(adc_keys_driver);
    MODULE_AUTHOR("Alexandre Belloni <alexandre.belloni@free-electrons.com>");
    MODULE_DESCRIPTION("Input driver for resistor ladder connected on ADC");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");
