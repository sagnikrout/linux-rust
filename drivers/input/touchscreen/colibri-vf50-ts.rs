//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/colibri-vf50-ts.c
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
// Toradex Colibri VF50 Touchscreen driver
//
// Copyright 2015 Toradex AG
//
// Originally authored by Stefan Agner for 3.0 kernel
//

pub const COLI_TOUCH_MIN_DELAY_US: c_int = 1000;
pub const COLI_TOUCH_MAX_DELAY_US: c_int = 2000;
pub const COLI_PULLUP_MIN_DELAY_US: c_int = 10000;
pub const COLI_PULLUP_MAX_DELAY_US: c_int = 11000;
pub const COLI_TOUCH_NO_OF_AVGS: c_int = 5;
pub const COLI_TOUCH_REQ_ADC_CHAN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf50_touch_device {
    pub pdev: *mut platform_device,
    pub ts_input: *mut input_dev,
    pub channels: *mut iio_channel,
    pub gpio_xp: *mut gpio_desc,
    pub gpio_xm: *mut gpio_desc,
    pub gpio_yp: *mut gpio_desc,
    pub gpio_ym: *mut gpio_desc,
    pub pen_irq: c_int,
    pub min_pressure: c_int,
    pub stop_touchscreen: bool,
}

//
// Enables given plates and measures touch parameters using ADC
//
    static int adc_ts_measure(struct iio_channel *channel,
    struct gpio_desc *plate_p, struct gpio_desc *plate_m)
    {
    int i, value = 0, val = 0;
    int error;
    gpiod_set_value(plate_p, 1);
    gpiod_set_value(plate_m, 1);
    usleep_range(COLI_TOUCH_MIN_DELAY_US, COLI_TOUCH_MAX_DELAY_US);
    for (i = 0; i < COLI_TOUCH_NO_OF_AVGS; i++) {
    error = iio_read_channel_raw(channel, &val);
    if (error < 0) {
    value = error;
    goto error_iio_read;
    }
    value += val;
    }
    value /= COLI_TOUCH_NO_OF_AVGS;
    error_iio_read:
    gpiod_set_value(plate_p, 0);
    gpiod_set_value(plate_m, 0);
    return value;
    }
//
// Enable touch detection using falling edge detection on XM
//
#[no_mangle]
unsafe extern "C" fn vf50_ts_enable_touch_detection(vf50_ts: *mut vf50_touch_device) {
    static void vf50_ts_enable_touch_detection(struct vf50_touch_device *vf50_ts)
    {
// Enable plate YM (needs to be strong GND, high active)
    gpiod_set_value(vf50_ts.gpio_ym, 1);
//
// Let the platform mux to idle state in order to enable
// Pull-Up on GPIO
//
    pinctrl_pm_select_idle_state(&vf50_ts.pdev.dev);
// Wait for the pull-up to be stable on high
    usleep_range(COLI_PULLUP_MIN_DELAY_US, COLI_PULLUP_MAX_DELAY_US);
    }
//
// ADC touch screen sampling bottom half irq handler
//
#[no_mangle]
unsafe extern "C" fn vf50_ts_irq_bh(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t vf50_ts_irq_bh(int irq, void *private)
    {
    struct vf50_touch_device *vf50_ts = private;
    struct device *dev = &vf50_ts.pdev.dev;
    int val_x, val_y, val_z1, val_z2, val_p = 0;
    let mut discard_val_on_start: bool = true;
// Disable the touch detection plates
    gpiod_set_value(vf50_ts.gpio_ym, 0);
// Let the platform mux to default state in order to mux as ADC
    pinctrl_pm_select_default_state(dev);
    while (!vf50_ts.stop_touchscreen) {
// X-Direction
    val_x = adc_ts_measure(&vf50_ts.channels[0],
    vf50_ts.gpio_xp, vf50_ts.gpio_xm);
    if (val_x < 0)
    break;
// Y-Direction
    val_y = adc_ts_measure(&vf50_ts.channels[1],
    vf50_ts.gpio_yp, vf50_ts.gpio_ym);
    if (val_y < 0)
    break;
//
// Touch pressure
// Measure on XP/YM
//
    val_z1 = adc_ts_measure(&vf50_ts.channels[2],
    vf50_ts.gpio_yp, vf50_ts.gpio_xm);
    if (val_z1 < 0)
    break;
    val_z2 = adc_ts_measure(&vf50_ts.channels[3],
    vf50_ts.gpio_yp, vf50_ts.gpio_xm);
    if (val_z2 < 0)
    break;
// Validate signal (avoid calculation using noise)
    if (val_z1 > 64 && val_x > 64) {
//
// Calculate resistance between the plates
// lower resistance means higher pressure
//
    let mut r_x: c_int = (1000 * val_x) / VF_ADC_MAX;
    val_p = (r_x * val_z2) / val_z1 - r_x;
    } else {
    val_p = 2000;
    }
    val_p = 2000 - val_p;
    dev_dbg(dev,
    "Measured values: x: %d, y: %d, z1: %d, z2: %d, p: %d\n",
    val_x, val_y, val_z1, val_z2, val_p);
//
// If touch pressure is too low, stop measuring and reenable
// touch detection
//
    if (val_p < vf50_ts.min_pressure || val_p > 2000)
    break;
//
// The pressure may not be enough for the first x and the
// second y measurement, but, the pressure is ok when the
// driver is doing the third and fourth measurement. To
// take care of this, we drop the first measurement always.
//
    if (discard_val_on_start) {
    discard_val_on_start = false;
    } else {
//
// Report touch position and sleep for
// the next measurement.
//
    input_report_abs(vf50_ts.ts_input,
    ABS_X, VF_ADC_MAX - val_x);
    input_report_abs(vf50_ts.ts_input,
    ABS_Y, VF_ADC_MAX - val_y);
    input_report_abs(vf50_ts.ts_input,
    ABS_PRESSURE, val_p);
    input_report_key(vf50_ts.ts_input, BTN_TOUCH, 1);
    input_sync(vf50_ts.ts_input);
    }
    usleep_range(COLI_PULLUP_MIN_DELAY_US,
    COLI_PULLUP_MAX_DELAY_US);
    }
// Report no more touch, re-enable touch detection
    input_report_abs(vf50_ts.ts_input, ABS_PRESSURE, 0);
    input_report_key(vf50_ts.ts_input, BTN_TOUCH, 0);
    input_sync(vf50_ts.ts_input);
    vf50_ts_enable_touch_detection(vf50_ts);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vf50_ts_open(dev_input: *mut input_dev) -> c_int {
    static int vf50_ts_open(struct input_dev *dev_input)
    {
    struct vf50_touch_device *touchdev = input_get_drvdata(dev_input);
    struct device *dev = &touchdev.pdev.dev;
    dev_dbg(dev, "Input device %s opened, starting touch detection\n",
    dev_input.name);
    touchdev.stop_touchscreen = false;
// Mux detection before request IRQ, wait for pull-up to settle
    vf50_ts_enable_touch_detection(touchdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vf50_ts_close(dev_input: *mut input_dev) {
    static void vf50_ts_close(struct input_dev *dev_input)
    {
    struct vf50_touch_device *touchdev = input_get_drvdata(dev_input);
    struct device *dev = &touchdev.pdev.dev;
    touchdev.stop_touchscreen = true;
// Make sure IRQ is not running past close
    mb();
    synchronize_irq(touchdev.pen_irq);
    gpiod_set_value(touchdev.gpio_ym, 0);
    pinctrl_pm_select_default_state(dev);
    dev_dbg(dev, "Input device %s closed, disable touch detection\n",
    dev_input.name);
    }
    static int vf50_ts_get_gpiod(struct device *dev, struct gpio_desc **gpio_d,
    const char *con_id, enum gpiod_flags flags)
    {
// gpio_d = devm_gpiod_get(dev, con_id, flags);
    if (IS_ERR(*gpio_d))
    return dev_err_probe(dev, PTR_ERR(*gpio_d),
    "Could not get gpio_%s\n", con_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vf50_ts_channel_release(data: *mut c_void) {
    static void vf50_ts_channel_release(void *data)
    {
    struct iio_channel *channels = data;
    iio_channel_release_all(channels);
    }
#[no_mangle]
unsafe extern "C" fn vf50_ts_probe(pdev: *mut platform_device) -> c_int {
    static int vf50_ts_probe(struct platform_device *pdev)
    {
    struct input_dev *input;
    struct iio_channel *channels;
    struct device *dev = &pdev.dev;
    struct vf50_touch_device *touchdev;
    int num_adc_channels;
    int error;
    channels = iio_channel_get_all(dev);
    if (IS_ERR(channels))
    return PTR_ERR(channels);
    error = devm_add_action(dev, vf50_ts_channel_release, channels);
    if (error) {
    iio_channel_release_all(channels);
    dev_err(dev, "Failed to register iio channel release action");
    return error;
    }
    num_adc_channels = 0;
    while (channels[num_adc_channels].indio_dev)
    num_adc_channels++;
    if (num_adc_channels != COLI_TOUCH_REQ_ADC_CHAN) {
    dev_err(dev, "Inadequate ADC channels specified\n");
    return -EINVAL;
    }
    touchdev = devm_kzalloc(dev, sizeof(*touchdev), GFP_KERNEL);
    if (!touchdev)
    return -ENOMEM;
    touchdev.pdev = pdev;
    touchdev.channels = channels;
    error = of_property_read_u32(dev.of_node, "vf50-ts-min-pressure",
    &touchdev.min_pressure);
    if (error)
    return error;
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "Failed to allocate TS input device\n");
    return -ENOMEM;
    }
    input.name = DRIVER_NAME;
    input.id.bustype = BUS_HOST;
    input.dev.parent = dev;
    input.open = vf50_ts_open;
    input.close = vf50_ts_close;
    input_set_capability(input, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input, ABS_X, 0, VF_ADC_MAX, 0, 0);
    input_set_abs_params(input, ABS_Y, 0, VF_ADC_MAX, 0, 0);
    input_set_abs_params(input, ABS_PRESSURE, 0, VF_ADC_MAX, 0, 0);
    touchdev.ts_input = input;
    input_set_drvdata(input, touchdev);
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "Failed to register input device\n");
    return error;
    }
    error = vf50_ts_get_gpiod(dev, &touchdev.gpio_xp, "xp", GPIOD_OUT_LOW);
    if (error)
    return error;
    error = vf50_ts_get_gpiod(dev, &touchdev.gpio_xm,
    "xm", GPIOD_OUT_LOW);
    if (error)
    return error;
    error = vf50_ts_get_gpiod(dev, &touchdev.gpio_yp, "yp", GPIOD_OUT_LOW);
    if (error)
    return error;
    error = vf50_ts_get_gpiod(dev, &touchdev.gpio_ym, "ym", GPIOD_OUT_LOW);
    if (error)
    return error;
    touchdev.pen_irq = platform_get_irq(pdev, 0);
    if (touchdev.pen_irq < 0)
    return touchdev.pen_irq;
    error = devm_request_threaded_irq(dev, touchdev.pen_irq,
    core::ptr::null_mut(), vf50_ts_irq_bh, IRQF_ONESHOT,
    "vf50 touch", touchdev);
    if (error) {
    dev_err(dev, "Failed to request IRQ %d: %d\n",
    touchdev.pen_irq, error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id vf50_touch_of_match[] = {
    { .compatible = "toradex,vf50-touchscreen", },
    { }
    };
    MODULE_DEVICE_TABLE(of, vf50_touch_of_match);
    static struct platform_driver vf50_touch_driver = {
    .driver = {
    .name = "toradex,vf50_touchctrl",
    .of_match_table = vf50_touch_of_match,
    },
    .probe = vf50_ts_probe,
    };
    module_platform_driver(vf50_touch_driver);
    MODULE_AUTHOR("Sanchayan Maity");
    MODULE_DESCRIPTION("Colibri VF50 Touchscreen driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
