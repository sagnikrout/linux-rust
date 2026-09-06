//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/jornada720_ts.c
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
// drivers/input/touchscreen/jornada720_ts.c
//
// Copyright (C) 2007 Kristoffer Ericson <Kristoffer.Ericson@gmail.com>
//
// Copyright (C) 2006 Filip Zyzniewski <filip.zyzniewski@tefnet.pl>
// based on HP Jornada 56x touchscreen driver by Alex Lange <chicken@handhelds.org>
//
// HP Jornada 710/720/729 Touchscreen Driver
//

    MODULE_AUTHOR("Kristoffer Ericson <kristoffer.ericson@gmail.com>");
    MODULE_DESCRIPTION("HP Jornada 710/720/728 touchscreen driver");
    MODULE_LICENSE("GPL v2");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jornada_ts {
    pub dev: *mut input_dev,
    pub gpio: *mut gpio_desc,
    pub /: *mut *mut int x_data[4]; / X sample values,
    pub /: *mut *mut int y_data[4]; / Y sample values,
}

#[no_mangle]
unsafe extern "C" fn jornada720_ts_collect_data(jornada_ts: *mut jornada_ts) {
    static void jornada720_ts_collect_data(struct jornada_ts *jornada_ts)
    {
// 3 low word X samples
    jornada_ts.x_data[0] = jornada_ssp_byte(TXDUMMY);
    jornada_ts.x_data[1] = jornada_ssp_byte(TXDUMMY);
    jornada_ts.x_data[2] = jornada_ssp_byte(TXDUMMY);
// 3 low word Y samples
    jornada_ts.y_data[0] = jornada_ssp_byte(TXDUMMY);
    jornada_ts.y_data[1] = jornada_ssp_byte(TXDUMMY);
    jornada_ts.y_data[2] = jornada_ssp_byte(TXDUMMY);
// combined x samples bits
    jornada_ts.x_data[3] = jornada_ssp_byte(TXDUMMY);
// combined y samples bits
    jornada_ts.y_data[3] = jornada_ssp_byte(TXDUMMY);
    }
#[no_mangle]
unsafe extern "C" fn jornada720_ts_average(coords[4]: c_int) -> c_int {
    static int jornada720_ts_average(int coords[4])
    {
    int coord, high_bits = coords[3];
    coord  = coords[0] | ((high_bits & 0x03) << 8);
    coord += coords[1] | ((high_bits & 0x0c) << 6);
    coord += coords[2] | ((high_bits & 0x30) << 4);
    return coord / 3;
    }
#[no_mangle]
unsafe extern "C" fn jornada720_ts_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t jornada720_ts_interrupt(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    struct jornada_ts *jornada_ts = platform_get_drvdata(pdev);
    struct input_dev *input = jornada_ts.dev;
    int x, y;
// If gpio is high then report pen up
    if (gpiod_get_value(jornada_ts.gpio)) {
    input_report_key(input, BTN_TOUCH, 0);
    input_sync(input);
    } else {
    jornada_ssp_start();
// proper reply to request is always TXDUMMY
    if (jornada_ssp_inout(GETTOUCHSAMPLES) == TXDUMMY) {
    jornada720_ts_collect_data(jornada_ts);
    x = jornada720_ts_average(jornada_ts.x_data);
    y = jornada720_ts_average(jornada_ts.y_data);
    input_report_key(input, BTN_TOUCH, 1);
    input_report_abs(input, ABS_X, x);
    input_report_abs(input, ABS_Y, y);
    input_sync(input);
    }
    jornada_ssp_end();
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn jornada720_ts_probe(pdev: *mut platform_device) -> c_int {
    static int jornada720_ts_probe(struct platform_device *pdev)
    {
    struct jornada_ts *jornada_ts;
    struct input_dev *input_dev;
    int error, irq;
    jornada_ts = devm_kzalloc(&pdev.dev, sizeof(*jornada_ts), GFP_KERNEL);
    if (!jornada_ts)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    platform_set_drvdata(pdev, jornada_ts);
    jornada_ts.gpio = devm_gpiod_get(&pdev.dev, "penup", GPIOD_IN);
    if (IS_ERR(jornada_ts.gpio))
    return PTR_ERR(jornada_ts.gpio);
    irq = gpiod_to_irq(jornada_ts.gpio);
    if (irq <= 0)
    return irq < 0 ? irq : -EINVAL;
    jornada_ts.dev = input_dev;
    input_dev.name = "HP Jornada 7xx Touchscreen";
    input_dev.phys = "jornadats/input0";
    input_dev.id.bustype = BUS_HOST;
    input_dev.dev.parent = &pdev.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(input_dev, ABS_X, 270, 3900, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 180, 3700, 0, 0);
    error = devm_request_irq(&pdev.dev, irq, jornada720_ts_interrupt,
    IRQF_TRIGGER_RISING,
    "HP7XX Touchscreen driver", pdev);
    if (error) {
    dev_err(&pdev.dev, "HP7XX TS : Unable to acquire irq!\n");
    return error;
    }
    error = input_register_device(jornada_ts.dev);
    if (error)
    return error;
    return 0;
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:jornada_ts");
    static struct platform_driver jornada720_ts_driver = {
    .probe		= jornada720_ts_probe,
    .driver		= {
    .name	= "jornada_ts",
    },
    };
    module_platform_driver(jornada720_ts_driver);
