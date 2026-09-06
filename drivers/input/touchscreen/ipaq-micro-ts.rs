//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ipaq-micro-ts.c
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
// h3600 atmel micro companion support, touchscreen subdevice
// Author : Alessandro Gardich <gremlin@gremlin.it>
// Author : Dmitry Artamonow <mad_soft@inbox.ru>
// Author : Linus Walleij <linus.walleij@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct touchscreen_data {
    pub input: *mut input_dev,
    pub micro: *mut ipaq_micro,
}

#[no_mangle]
unsafe extern "C" fn micro_ts_receive(data: *mut c_void, len: c_int, msg: *mut c_uchar) {
    static void micro_ts_receive(void *data, int len, unsigned char *msg)
    {
    struct touchscreen_data *ts = data;
    if (len == 4) {
    input_report_abs(ts.input, ABS_X,
    be16_to_cpup((__be16 *) &msg[2]));
    input_report_abs(ts.input, ABS_Y,
    be16_to_cpup((__be16 *) &msg[0]));
    input_report_key(ts.input, BTN_TOUCH, 1);
    input_sync(ts.input);
    } else if (len == 0) {
    input_report_abs(ts.input, ABS_X, 0);
    input_report_abs(ts.input, ABS_Y, 0);
    input_report_key(ts.input, BTN_TOUCH, 0);
    input_sync(ts.input);
    }
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_toggle_receive(ts: *mut touchscreen_data, enable: bool) {
    static void micro_ts_toggle_receive(struct touchscreen_data *ts, bool enable)
    {
    struct ipaq_micro *micro = ts.micro;
    guard(spinlock_irq)(&micro.lock);
    if (enable) {
    micro.ts = micro_ts_receive;
    micro.ts_data = ts;
    } else {
    micro.ts = core::ptr::null_mut();
    micro.ts_data = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_open(input: *mut input_dev) -> c_int {
    static int micro_ts_open(struct input_dev *input)
    {
    struct touchscreen_data *ts = input_get_drvdata(input);
    micro_ts_toggle_receive(ts, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_close(input: *mut input_dev) {
    static void micro_ts_close(struct input_dev *input)
    {
    struct touchscreen_data *ts = input_get_drvdata(input);
    micro_ts_toggle_receive(ts, false);
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_probe(pdev: *mut platform_device) -> c_int {
    static int micro_ts_probe(struct platform_device *pdev)
    {
    struct ipaq_micro *micro = dev_get_drvdata(pdev.dev.parent);
    struct touchscreen_data *ts;
    int error;
    ts = devm_kzalloc(&pdev.dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.micro = micro;
    ts.input = devm_input_allocate_device(&pdev.dev);
    if (!ts.input) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    ts.input.name = "ipaq micro ts";
    ts.input.open = micro_ts_open;
    ts.input.close = micro_ts_close;
    input_set_drvdata(ts.input, ts);
    input_set_capability(ts.input, EV_KEY, BTN_TOUCH);
    input_set_capability(ts.input, EV_ABS, ABS_X);
    input_set_capability(ts.input, EV_ABS, ABS_Y);
    input_set_abs_params(ts.input, ABS_X, 0, 1023, 0, 0);
    input_set_abs_params(ts.input, ABS_Y, 0, 1023, 0, 0);
    error = input_register_device(ts.input);
    if (error) {
    dev_err(&pdev.dev, "error registering touch input\n");
    return error;
    }
    platform_set_drvdata(pdev, ts);
    dev_info(&pdev.dev, "iPAQ micro touchscreen\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_suspend(dev: *mut device) -> c_int {
    static int micro_ts_suspend(struct device *dev)
    {
    struct touchscreen_data *ts = dev_get_drvdata(dev);
    micro_ts_toggle_receive(ts, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_ts_resume(dev: *mut device) -> c_int {
    static int micro_ts_resume(struct device *dev)
    {
    struct touchscreen_data *ts = dev_get_drvdata(dev);
    struct input_dev *input = ts.input;
    guard(mutex)(&input.mutex);
    if (input_device_enabled(input))
    micro_ts_toggle_receive(ts, true);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(micro_ts_dev_pm_ops,
    micro_ts_suspend, micro_ts_resume);
    static struct platform_driver micro_ts_device_driver = {
    .driver	= {
    .name	= "ipaq-micro-ts",
    .pm	= pm_sleep_ptr(&micro_ts_dev_pm_ops),
    },
    .probe	= micro_ts_probe,
    };
    module_platform_driver(micro_ts_device_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("driver for iPAQ Atmel micro touchscreen");
    MODULE_ALIAS("platform:ipaq-micro-ts");
