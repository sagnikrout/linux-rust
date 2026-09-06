//! Automatically rewritten from C to Rust
//! Source: drivers/pps/clients/pps-gpio.c
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
// pps-gpio.c -- PPS client driver using GPIO
//
// Copyright (C) 2010 Ricardo Martins <rasm@fe.up.pt>
// Copyright (C) 2011 James Nuss <jamesnuss@nanometrics.ca>
//

// Info for each registered platform device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_gpio_device_data {
    pub /: *mut *mut int irq; / IRQ used as PPS source,
    pub /: *mut *mut *mut pps_device pps; / PPS source device,
    pub /: *mut *mut pps_source_info info; / PPS source information,
    pub /: *mut *mut *mut gpio_desc gpio_pin; / GPIO port descriptors,
    pub echo_pin: *mut gpio_desc,
    pub /: *mut *mut timer_list echo_timer; / timer to reset echo active state,
    pub assert_falling_edge: bool,
    pub /: *mut *mut unsigned int echo_active_ms; / PPS echo active duration,
    pub /: *mut *mut unsigned long echo_timeout; / timer timeout value in jiffies,
    pub /: *mut *mut pps_event_time ts; / timestamp captured in hardirq,
}

//
// Report the PPS event
//
// Primary hardirq handler -- runs in hardirq context even on PREEMPT_RT.
// Only captures the timestamp; all other work is deferred to the thread.
//
#[no_mangle]
unsafe extern "C" fn pps_gpio_irq_hardirq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pps_gpio_irq_hardirq(int irq, void *data)
    {
    struct pps_gpio_device_data *info = data;
    pps_get_ts(&info.ts);
    return IRQ_WAKE_THREAD;
    }
//
// Threaded handler -- processes the PPS event using the timestamp
// captured in hardirq context above.
//
#[no_mangle]
unsafe extern "C" fn pps_gpio_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pps_gpio_irq_thread(int irq, void *data)
    {
    struct pps_gpio_device_data *info = data;
    pps_event(info.pps, &info.ts, PPS_CAPTUREASSERT, data);
    return IRQ_HANDLED;
    }
// This function will only be called when an ECHO GPIO is defined
#[no_mangle]
unsafe extern "C" fn pps_gpio_echo(pps: *mut pps_device, event: c_int, data: *mut c_void) {
    static void pps_gpio_echo(struct pps_device *pps, int event, void *data)
    {
// add_timer() needs to write into info->echo_timer
    struct pps_gpio_device_data *info = data;
    switch (event) {
    case PPS_CAPTUREASSERT:
    if (pps.params.mode & PPS_ECHOASSERT)
    gpiod_set_value(info.echo_pin, 1);
    break;
    }
// fire the timer
    if (info.pps.params.mode & (PPS_ECHOASSERT | PPS_ECHOCLEAR)) {
    info.echo_timer.expires = jiffies + info.echo_timeout;
    add_timer(&info.echo_timer);
    }
    }
// Timer callback to reset the echo pin to the inactive state
#[no_mangle]
unsafe extern "C" fn pps_gpio_echo_timer_callback(t: *mut timer_list) {
    static void pps_gpio_echo_timer_callback(struct timer_list *t)
    {
    const struct pps_gpio_device_data *info;
    info = timer_container_of(info, t, echo_timer);
    gpiod_set_value(info.echo_pin, 0);
    }
#[no_mangle]
unsafe extern "C" fn pps_gpio_setup(dev: *mut device) -> c_int {
    static int pps_gpio_setup(struct device *dev)
    {
    struct pps_gpio_device_data *data = dev_get_drvdata(dev);
    int ret;
    u32 value;
    data.gpio_pin = devm_gpiod_get(dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(data.gpio_pin))
    return dev_err_probe(dev, PTR_ERR(data.gpio_pin),
    "failed to request PPS GPIO\n");
    data.assert_falling_edge =
    device_property_read_bool(dev, "assert-falling-edge");
    data.echo_pin = devm_gpiod_get_optional(dev, "echo", GPIOD_OUT_LOW);
    if (IS_ERR(data.echo_pin))
    return dev_err_probe(dev, PTR_ERR(data.echo_pin),
    "failed to request ECHO GPIO\n");
    if (!data.echo_pin)
    return 0;
    ret = device_property_read_u32(dev, "echo-active-ms", &value);
    if (ret) {
    dev_err(dev, "failed to get echo-active-ms from FW\n");
    return ret;
    }
// sanity check on echo_active_ms
    if (!value || value > 999) {
    dev_err(dev, "echo-active-ms: %u - bad value from FW\n", value);
    return -EINVAL;
    }
    data.echo_active_ms = value;
    return 0;
    }
    static unsigned long
    get_irqf_trigger_flags(const struct pps_gpio_device_data *data)
    {
    return data.assert_falling_edge ? IRQF_TRIGGER_FALLING :
    IRQF_TRIGGER_RISING;
    }
#[no_mangle]
unsafe extern "C" fn pps_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int pps_gpio_probe(struct platform_device *pdev)
    {
    struct pps_gpio_device_data *data;
    struct device *dev = &pdev.dev;
    int ret;
    int pps_default_params;
// allocate space for device info
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    dev_set_drvdata(dev, data);
// GPIO setup
    ret = pps_gpio_setup(dev);
    if (ret)
    return ret;
// IRQ setup
    ret = gpiod_to_irq(data.gpio_pin);
    if (ret < 0) {
    dev_err(dev, "failed to map GPIO to IRQ: %d\n", ret);
    return -EINVAL;
    }
    data.irq = ret;
// initialize PPS specific parts of the bookkeeping data structure.
    data.info.mode = PPS_CAPTUREASSERT | PPS_OFFSETASSERT |
    PPS_ECHOASSERT | PPS_CANWAIT | PPS_TSFMT_TSPEC;
    data.info.owner = THIS_MODULE;
    snprintf(data.info.name, PPS_MAX_NAME_LEN - 1, "%s.%d",
    pdev.name, pdev.id);
    if (data.echo_pin) {
    data.info.echo = pps_gpio_echo;
    data.echo_timeout = msecs_to_jiffies(data.echo_active_ms);
    timer_setup(&data.echo_timer, pps_gpio_echo_timer_callback, 0);
    }
// register PPS source
    pps_default_params = PPS_CAPTUREASSERT | PPS_OFFSETASSERT;
    data.pps = pps_register_source(&data.info, pps_default_params);
    if (IS_ERR(data.pps)) {
    dev_err(dev, "failed to register IRQ %d as PPS source\n",
    data.irq);
    return PTR_ERR(data.pps);
    }
// register IRQ interrupt handler
    ret = request_threaded_irq(data.irq,
    pps_gpio_irq_hardirq, pps_gpio_irq_thread,
    get_irqf_trigger_flags(data) | IRQF_ONESHOT,
    data.info.name, data);
    if (ret) {
    pps_unregister_source(data.pps);
    dev_err(dev, "failed to acquire IRQ %d\n", data.irq);
    return -EINVAL;
    }
    dev_dbg(&data.pps.dev, "Registered IRQ %d as PPS source\n",
    data.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pps_gpio_remove(pdev: *mut platform_device) {
    static void pps_gpio_remove(struct platform_device *pdev)
    {
    struct pps_gpio_device_data *data = platform_get_drvdata(pdev);
    free_irq(data.irq, data);
    pps_unregister_source(data.pps);
    timer_delete_sync(&data.echo_timer);
// reset echo pin in any case
    gpiod_set_value(data.echo_pin, 0);
    dev_info(&pdev.dev, "removed IRQ %d as PPS source\n", data.irq);
    }
    static const struct of_device_id pps_gpio_dt_ids[] = {
    { .compatible = "pps-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pps_gpio_dt_ids);
    static struct platform_driver pps_gpio_driver = {
    .probe		= pps_gpio_probe,
    .remove		= pps_gpio_remove,
    .driver		= {
    .name	= PPS_GPIO_NAME,
    .of_match_table	= pps_gpio_dt_ids,
    },
    };
    module_platform_driver(pps_gpio_driver);
    MODULE_AUTHOR("Ricardo Martins <rasm@fe.up.pt>");
    MODULE_AUTHOR("James Nuss <jamesnuss@nanometrics.ca>");
    MODULE_DESCRIPTION("Use GPIO pin as PPS source");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.2.0");
