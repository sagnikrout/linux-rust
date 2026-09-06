//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/mc13783_ts.c
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
// Driver for the Freescale Semiconductor MC13783 touchscreen.
//
// Copyright 2004-2007 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright (C) 2009 Sascha Hauer, Pengutronix
//
// Initial development of this code was funded by
// Phytec Messtechnik GmbH, http://www.phytec.de
//

pub const DEFAULT_SAMPLE_TOLERANCE: c_int = 300;
    let mut sample_tolerance: static unsigned int = DEFAULT_SAMPLE_TOLERANCE;
    module_param(sample_tolerance, uint, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(sample_tolerance,
    "If the minimal and maximal value read out for one axis (out "
    "of three) differ by this value (default: "
    __stringify(DEFAULT_SAMPLE_TOLERANCE) ") or more, the reading "
    "is supposed to be wrong and is discarded.  Set to 0 to "
    "disable this check.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13783_ts_priv {
    pub idev: *mut input_dev,
    pub mc13xxx: *mut mc13xxx,
    pub work: delayed_work,
    pub sample: [c_uint; 4],
    pub touch: *mut mc13xxx_ts_platform_data,
}

#[no_mangle]
unsafe extern "C" fn mc13783_ts_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mc13783_ts_handler(int irq, void *data)
    {
    struct mc13783_ts_priv *priv = data;
//
// Kick off reading coordinates. Note that if work happens already
// be queued for future execution (it rearms itself) it will not
// be rescheduled for immediate execution here. However the rearm
// delay is HZ / 50 which is acceptable.
//
    schedule_delayed_work(&priv.work, 0);
    return IRQ_HANDLED;
    }

    if (a0 > a1)						\
    swap(a0, a1);					\
    if (a1 > a2)						\
    swap(a1, a2);					\
    if (a0 > a1)						\
    swap(a0, a1);					\
    })
#[no_mangle]
unsafe extern "C" fn mc13783_ts_report_sample(priv: *mut mc13783_ts_priv) {
    static void mc13783_ts_report_sample(struct mc13783_ts_priv *priv)
    {
    struct input_dev *idev = priv.idev;
    int x0, x1, x2, y0, y1, y2;
    int cr0, cr1;
//
// the values are 10-bit wide only, but the two least significant
// bits are for future 12 bit use and reading yields 0
//
    x0 = priv.sample[0] & 0xfff;
    x1 = priv.sample[1] & 0xfff;
    x2 = priv.sample[2] & 0xfff;
    y0 = priv.sample[3] & 0xfff;
    y1 = (priv.sample[0] >> 12) & 0xfff;
    y2 = (priv.sample[1] >> 12) & 0xfff;
    cr0 = (priv.sample[2] >> 12) & 0xfff;
    cr1 = (priv.sample[3] >> 12) & 0xfff;
    dev_dbg(&idev.dev,
    "x: (% 4d,% 4d,% 4d) y: (% 4d, % 4d,% 4d) cr: (% 4d, % 4d)\n",
    x0, x1, x2, y0, y1, y2, cr0, cr1);
    sort3(x0, x1, x2);
    sort3(y0, y1, y2);
    cr0 = (cr0 + cr1) / 2;
    if (!cr0 || !sample_tolerance ||
    (x2 - x0 < sample_tolerance &&
    y2 - y0 < sample_tolerance)) {
// report the median coordinate and average pressure
    if (cr0) {
    input_report_abs(idev, ABS_X, x1);
    input_report_abs(idev, ABS_Y, y1);
    dev_dbg(&idev.dev, "report (%d, %d, %d)\n",
    x1, y1, 0x1000 - cr0);
    schedule_delayed_work(&priv.work, HZ / 50);
    } else {
    dev_dbg(&idev.dev, "report release\n");
    }
    input_report_abs(idev, ABS_PRESSURE,
    cr0 ? 0x1000 - cr0 : cr0);
    input_report_key(idev, BTN_TOUCH, cr0);
    input_sync(idev);
    } else {
    dev_dbg(&idev.dev, "discard event\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn mc13783_ts_work(work: *mut work_struct) {
    static void mc13783_ts_work(struct work_struct *work)
    {
    struct mc13783_ts_priv *priv =
    container_of(work, struct mc13783_ts_priv, work.work);
    let mut mode: c_uint = MC13XXX_ADC_MODE_TS;
    let mut channel: c_uint = 12;
    if (mc13xxx_adc_do_conversion(priv.mc13xxx,
    mode, channel,
    priv.touch.ato, priv.touch.atox,
    priv.sample) == 0)
    mc13783_ts_report_sample(priv);
    }
#[no_mangle]
unsafe extern "C" fn mc13783_ts_open(dev: *mut input_dev) -> c_int {
    static int mc13783_ts_open(struct input_dev *dev)
    {
    struct mc13783_ts_priv *priv = input_get_drvdata(dev);
    int ret;
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_irq_request(priv.mc13xxx, MC13XXX_IRQ_TS,
    mc13783_ts_handler, MC13783_TS_NAME, priv);
    if (ret)
    goto out;
    ret = mc13xxx_reg_rmw(priv.mc13xxx, MC13XXX_ADC0,
    MC13XXX_ADC0_TSMOD_MASK, MC13XXX_ADC0_TSMOD0);
    if (ret)
    mc13xxx_irq_free(priv.mc13xxx, MC13XXX_IRQ_TS, priv);
    out:
    mc13xxx_unlock(priv.mc13xxx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc13783_ts_close(dev: *mut input_dev) {
    static void mc13783_ts_close(struct input_dev *dev)
    {
    struct mc13783_ts_priv *priv = input_get_drvdata(dev);
    mc13xxx_lock(priv.mc13xxx);
    mc13xxx_reg_rmw(priv.mc13xxx, MC13XXX_ADC0,
    MC13XXX_ADC0_TSMOD_MASK, 0);
    mc13xxx_irq_free(priv.mc13xxx, MC13XXX_IRQ_TS, priv);
    mc13xxx_unlock(priv.mc13xxx);
    cancel_delayed_work_sync(&priv.work);
    }
#[no_mangle]
unsafe extern "C" fn mc13783_ts_probe(pdev: *mut platform_device) -> int __init {
    static int __init mc13783_ts_probe(struct platform_device *pdev)
    {
    struct mc13783_ts_priv *priv;
    struct input_dev *idev;
    let mut ret: c_int = -ENOMEM;
    priv = kzalloc_obj(*priv);
    idev = input_allocate_device();
    if (!priv || !idev)
    goto err_free_mem;
    INIT_DELAYED_WORK(&priv.work, mc13783_ts_work);
    priv.mc13xxx = dev_get_drvdata(pdev.dev.parent);
    priv.idev = idev;
    priv.touch = dev_get_platdata(&pdev.dev);
    if (!priv.touch) {
    dev_err(&pdev.dev, "missing platform data\n");
    ret = -ENODEV;
    goto err_free_mem;
    }
    idev.name = MC13783_TS_NAME;
    idev.dev.parent = &pdev.dev;
    idev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    idev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(idev, ABS_X, 0, 0xfff, 0, 0);
    input_set_abs_params(idev, ABS_Y, 0, 0xfff, 0, 0);
    input_set_abs_params(idev, ABS_PRESSURE, 0, 0xfff, 0, 0);
    idev.open = mc13783_ts_open;
    idev.close = mc13783_ts_close;
    input_set_drvdata(idev, priv);
    ret = input_register_device(priv.idev);
    if (ret) {
    dev_err(&pdev.dev,
    "register input device failed with %d\n", ret);
    goto err_free_mem;
    }
    platform_set_drvdata(pdev, priv);
    return 0;
    err_free_mem:
    input_free_device(idev);
    kfree(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc13783_ts_remove(pdev: *mut platform_device) {
    static void mc13783_ts_remove(struct platform_device *pdev)
    {
    struct mc13783_ts_priv *priv = platform_get_drvdata(pdev);
    input_unregister_device(priv.idev);
    kfree(priv);
    }
    static struct platform_driver mc13783_ts_driver = {
    .remove		= mc13783_ts_remove,
    .driver		= {
    .name	= MC13783_TS_NAME,
    },
    };
    module_platform_driver_probe(mc13783_ts_driver, mc13783_ts_probe);
    MODULE_DESCRIPTION("MC13783 input touchscreen driver");
    MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" MC13783_TS_NAME);
