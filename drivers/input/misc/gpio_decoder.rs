//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/gpio_decoder.c
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
// Copyright (C) 2016 Texas Instruments Incorporated - http://www.ti.com
//
// A generic driver to read multiple gpio lines and translate the
// encoded numeric value into an input event.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_decoder {
    pub input_gpios: *mut gpio_descs,
    pub dev: *mut device,
    pub axis: u32,
    pub last_stable: u32,
}

#[no_mangle]
unsafe extern "C" fn gpio_decoder_get_gpios_state(decoder: *mut gpio_decoder) -> c_int {
    static int gpio_decoder_get_gpios_state(struct gpio_decoder *decoder)
    {
    struct gpio_descs *gpios = decoder.input_gpios;
    DECLARE_BITMAP(values, 32);
    unsigned int size;
    int err;
    size = min(gpios.ndescs, 32U);
    err = gpiod_get_array_value_cansleep(size, gpios.desc, gpios.info, values);
    if (err) {
    dev_err(decoder.dev, "Error reading GPIO: %d\n", err);
    return err;
    }
    return bitmap_read(values, 0, size);
    }
#[no_mangle]
unsafe extern "C" fn gpio_decoder_poll_gpios(input: *mut input_dev) {
    static void gpio_decoder_poll_gpios(struct input_dev *input)
    {
    struct gpio_decoder *decoder = input_get_drvdata(input);
    int state;
    state = gpio_decoder_get_gpios_state(decoder);
    if (state >= 0 && state != decoder.last_stable) {
    input_report_abs(input, decoder.axis, state);
    input_sync(input);
    decoder.last_stable = state;
    }
    }
#[no_mangle]
unsafe extern "C" fn gpio_decoder_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_decoder_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_decoder *decoder;
    struct input_dev *input;
    u32 max;
    int err;
    decoder = devm_kzalloc(dev, sizeof(*decoder), GFP_KERNEL);
    if (!decoder)
    return -ENOMEM;
    decoder.dev = dev;
    device_property_read_u32(dev, "linux,axis", &decoder.axis);
    decoder.input_gpios = devm_gpiod_get_array(dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(decoder.input_gpios))
    return dev_err_probe(dev, PTR_ERR(decoder.input_gpios),
    "unable to acquire input gpios\n");
    if (decoder.input_gpios.ndescs < 2)
    return dev_err_probe(dev, -EINVAL, "not enough gpios found\n");
    if (decoder.input_gpios.ndescs > 31)
    return dev_err_probe(dev, -EINVAL, "too many gpios found\n");
    if (device_property_read_u32(dev, "decoder-max-value", &max))
    max = BIT(decoder.input_gpios.ndescs) - 1;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input_set_drvdata(input, decoder);
    input.name = pdev.name;
    input.id.bustype = BUS_HOST;
    input_set_abs_params(input, decoder.axis, 0, max, 0, 0);
    err = input_setup_polling(input, gpio_decoder_poll_gpios);
    if (err)
    return dev_err_probe(dev, err, "failed to set up polling\n");
    err = input_register_device(input);
    if (err)
    return dev_err_probe(dev, err, "failed to register input device\n");
    return 0;
    }
    static const struct of_device_id gpio_decoder_of_match[] = {
    { .compatible = "gpio-decoder", },
    { }
    };
    MODULE_DEVICE_TABLE(of, gpio_decoder_of_match);
    static struct platform_driver gpio_decoder_driver = {
    .probe		= gpio_decoder_probe,
    .driver		= {
    .name	= "gpio-decoder",
    .of_match_table = gpio_decoder_of_match,
    }
    };
    module_platform_driver(gpio_decoder_driver);
    MODULE_DESCRIPTION("GPIO decoder input driver");
    MODULE_AUTHOR("Vignesh R <vigneshr@ti.com>");
    MODULE_LICENSE("GPL v2");
