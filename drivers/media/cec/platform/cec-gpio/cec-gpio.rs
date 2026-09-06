//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/cec-gpio/cec-gpio.c
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
// Copyright 2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_gpio {
    pub adap: *mut cec_adapter,
    pub notifier: *mut cec_notifier,
    pub dev: *mut device,
    pub cec_gpio: *mut gpio_desc,
    pub cec_irq: c_int,
    pub cec_is_low: bool,
    pub hpd_gpio: *mut gpio_desc,
    pub hpd_irq: c_int,
    pub hpd_is_high: bool,
    pub hpd_ts: ktime_t,
    pub v5_gpio: *mut gpio_desc,
    pub v5_irq: c_int,
    pub v5_is_high: bool,
    pub v5_ts: ktime_t,
}

#[no_mangle]
unsafe extern "C" fn cec_gpio_read(adap: *mut cec_adapter) -> c_int {
    static int cec_gpio_read(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    if (cec.cec_is_low)
    return 0;
    return gpiod_get_value(cec.cec_gpio);
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_high(adap: *mut cec_adapter) {
    static void cec_gpio_high(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    if (!cec.cec_is_low)
    return;
    cec.cec_is_low = false;
    gpiod_set_value(cec.cec_gpio, 1);
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_low(adap: *mut cec_adapter) {
    static void cec_gpio_low(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    if (cec.cec_is_low)
    return;
    cec.cec_is_low = true;
    gpiod_set_value(cec.cec_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_5v_irq_handler_thread(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t cec_gpio_5v_irq_handler_thread(int irq, void *priv)
    {
    struct cec_gpio *cec = priv;
    let mut val: c_int = gpiod_get_value_cansleep(cec.v5_gpio);
    let mut is_high: bool = val > 0;
    if (val < 0 || is_high == cec.v5_is_high)
    return IRQ_HANDLED;
    cec.v5_is_high = is_high;
    cec_queue_pin_5v_event(cec.adap, cec.v5_is_high, cec.v5_ts);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_5v_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t cec_gpio_5v_irq_handler(int irq, void *priv)
    {
    struct cec_gpio *cec = priv;
    cec.v5_ts = ktime_get();
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_hpd_irq_handler_thread(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t cec_gpio_hpd_irq_handler_thread(int irq, void *priv)
    {
    struct cec_gpio *cec = priv;
    let mut val: c_int = gpiod_get_value_cansleep(cec.hpd_gpio);
    let mut is_high: bool = val > 0;
    if (val < 0 || is_high == cec.hpd_is_high)
    return IRQ_HANDLED;
    cec.hpd_is_high = is_high;
    cec_queue_pin_hpd_event(cec.adap, cec.hpd_is_high, cec.hpd_ts);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_hpd_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t cec_gpio_hpd_irq_handler(int irq, void *priv)
    {
    struct cec_gpio *cec = priv;
    cec.hpd_ts = ktime_get();
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_cec_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t cec_gpio_cec_irq_handler(int irq, void *priv)
    {
    struct cec_gpio *cec = priv;
    let mut val: c_int = gpiod_get_value(cec.cec_gpio);
    if (val >= 0)
    cec_pin_changed(cec.adap, val > 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_cec_enable_irq(adap: *mut cec_adapter) -> bool {
    static bool cec_gpio_cec_enable_irq(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    enable_irq(cec.cec_irq);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_cec_disable_irq(adap: *mut cec_adapter) {
    static void cec_gpio_cec_disable_irq(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    disable_irq(cec.cec_irq);
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_status(adap: *mut cec_adapter, file: *mut seq_file) {
    static void cec_gpio_status(struct cec_adapter *adap, struct seq_file *file)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    seq_printf(file, "mode: %s\n", cec.cec_is_low ? "low-drive" : "read");
    seq_printf(file, "using irq: %d\n", cec.cec_irq);
    if (cec.hpd_gpio)
    seq_printf(file, "hpd: %s\n",
    cec.hpd_is_high ? "high" : "low");
    if (cec.v5_gpio)
    seq_printf(file, "5V: %s\n",
    cec.v5_is_high ? "high" : "low");
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_read_hpd(adap: *mut cec_adapter) -> c_int {
    static int cec_gpio_read_hpd(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    if (!cec.hpd_gpio)
    return -ENOTTY;
    return gpiod_get_value_cansleep(cec.hpd_gpio);
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_read_5v(adap: *mut cec_adapter) -> c_int {
    static int cec_gpio_read_5v(struct cec_adapter *adap)
    {
    struct cec_gpio *cec = cec_get_drvdata(adap);
    if (!cec.v5_gpio)
    return -ENOTTY;
    return gpiod_get_value_cansleep(cec.v5_gpio);
    }
    static const struct cec_pin_ops cec_gpio_pin_ops = {
    .read = cec_gpio_read,
    .low = cec_gpio_low,
    .high = cec_gpio_high,
    .enable_irq = cec_gpio_cec_enable_irq,
    .disable_irq = cec_gpio_cec_disable_irq,
    .status = cec_gpio_status,
    .read_hpd = cec_gpio_read_hpd,
    .read_5v = cec_gpio_read_5v,
    };
#[no_mangle]
unsafe extern "C" fn cec_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int cec_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *hdmi_dev;
    struct cec_gpio *cec;
    let mut caps: u32 = CEC_CAP_DEFAULTS | CEC_CAP_MONITOR_ALL | CEC_CAP_MONITOR_PIN;
    int ret;
    hdmi_dev = cec_notifier_parse_hdmi_phandle(dev);
    if (PTR_ERR(hdmi_dev) == -EPROBE_DEFER)
    return PTR_ERR(hdmi_dev);
    if (IS_ERR(hdmi_dev))
    caps |= CEC_CAP_PHYS_ADDR;
    cec = devm_kzalloc(dev, sizeof(*cec), GFP_KERNEL);
    if (!cec)
    return -ENOMEM;
    cec.dev = dev;
    cec.cec_gpio = devm_gpiod_get(dev, "cec", GPIOD_OUT_HIGH_OPEN_DRAIN);
    if (IS_ERR(cec.cec_gpio))
    return PTR_ERR(cec.cec_gpio);
    cec.cec_irq = gpiod_to_irq(cec.cec_gpio);
    cec.hpd_gpio = devm_gpiod_get_optional(dev, "hpd", GPIOD_IN);
    if (IS_ERR(cec.hpd_gpio))
    return PTR_ERR(cec.hpd_gpio);
    cec.v5_gpio = devm_gpiod_get_optional(dev, "v5", GPIOD_IN);
    if (IS_ERR(cec.v5_gpio))
    return PTR_ERR(cec.v5_gpio);
    cec.adap = cec_pin_allocate_adapter(&cec_gpio_pin_ops,
    cec, pdev.name, caps);
    if (IS_ERR(cec.adap))
    return PTR_ERR(cec.adap);
    ret = devm_request_irq(dev, cec.cec_irq, cec_gpio_cec_irq_handler,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_NO_AUTOEN,
    cec.adap.name, cec);
    if (ret)
    goto del_adap;
    if (cec.hpd_gpio) {
    cec.hpd_irq = gpiod_to_irq(cec.hpd_gpio);
    ret = devm_request_threaded_irq(dev, cec.hpd_irq,
    cec_gpio_hpd_irq_handler,
    cec_gpio_hpd_irq_handler_thread,
    IRQF_ONESHOT |
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING,
    "hpd-gpio", cec);
    if (ret)
    goto del_adap;
    }
    if (cec.v5_gpio) {
    cec.v5_irq = gpiod_to_irq(cec.v5_gpio);
    ret = devm_request_threaded_irq(dev, cec.v5_irq,
    cec_gpio_5v_irq_handler,
    cec_gpio_5v_irq_handler_thread,
    IRQF_ONESHOT |
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING,
    "v5-gpio", cec);
    if (ret)
    goto del_adap;
    }
    if (!IS_ERR(hdmi_dev)) {
    cec.notifier = cec_notifier_cec_adap_register(hdmi_dev, core::ptr::null_mut(),
    cec.adap);
    if (!cec.notifier) {
    ret = -ENOMEM;
    goto del_adap;
    }
    }
    ret = cec_register_adapter(cec.adap, &pdev.dev);
    if (ret)
    goto unreg_notifier;
    platform_set_drvdata(pdev, cec);
    return 0;
    unreg_notifier:
    cec_notifier_cec_adap_unregister(cec.notifier, cec.adap);
    del_adap:
    cec_delete_adapter(cec.adap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cec_gpio_remove(pdev: *mut platform_device) {
    static void cec_gpio_remove(struct platform_device *pdev)
    {
    struct cec_gpio *cec = platform_get_drvdata(pdev);
    cec_notifier_cec_adap_unregister(cec.notifier, cec.adap);
    cec_unregister_adapter(cec.adap);
    }
    static const struct of_device_id cec_gpio_match[] = {
    {
    .compatible	= "cec-gpio",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cec_gpio_match);
    static struct platform_driver cec_gpio_pdrv = {
    .probe	= cec_gpio_probe,
    .remove = cec_gpio_remove,
    .driver = {
    .name		= "cec-gpio",
    .of_match_table	= cec_gpio_match,
    },
    };
    module_platform_driver(cec_gpio_pdrv);
    MODULE_AUTHOR("Hans Verkuil <hverkuil@kernel.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("CEC GPIO driver");
