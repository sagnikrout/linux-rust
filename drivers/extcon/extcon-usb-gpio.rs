//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-usb-gpio.c
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
// drivers/extcon/extcon-usb-gpio.c - USB GPIO extcon driver
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Author: Roger Quadros <rogerq@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_extcon_info {
    pub dev: *mut device,
    pub edev: *mut extcon_dev,
    pub id_gpiod: *mut gpio_desc,
    pub vbus_gpiod: *mut gpio_desc,
    pub id_irq: c_int,
    pub vbus_irq: c_int,
    pub debounce_jiffies: c_ulong,
    pub wq_detcable: delayed_work,
}

    static const unsigned int usb_extcon_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_NONE,
    };
//
// "USB" = VBUS and "USB-HOST" = !ID, so we have:
// Both "USB" and "USB-HOST" can't be set as active at the
// same time so if "USB-HOST" is active (i.e. ID is 0)  we keep "USB" inactive
// even if VBUS is on.
//
// State              |    ID   |   VBUS
// ----------------------------------------
// [1] USB            |    H    |    H
// [2] none           |    H    |    L
// [3] USB-HOST       |    L    |    H
// [4] USB-HOST       |    L    |    L
//
// In case we have only one of these signals:
// - VBUS only - we want to distinguish between [1] and [2], so ID is always 1.
// - ID only - we want to distinguish between [1] and [4], so VBUS = ID.
//
#[no_mangle]
unsafe extern "C" fn usb_extcon_detect_cable(work: *mut work_struct) {
    static void usb_extcon_detect_cable(struct work_struct *work)
    {
    int id, vbus;
    struct usb_extcon_info *info = container_of(to_delayed_work(work),
    struct usb_extcon_info,
    wq_detcable);
// check ID and VBUS and update cable state
    id = info.id_gpiod ?
    gpiod_get_value_cansleep(info.id_gpiod) : 1;
    vbus = info.vbus_gpiod ?
    gpiod_get_value_cansleep(info.vbus_gpiod) : id;
// at first we clean states which are no longer active
    if (id)
    extcon_set_state_sync(info.edev, EXTCON_USB_HOST, false);
    if (!vbus)
    extcon_set_state_sync(info.edev, EXTCON_USB, false);
    if (!id) {
    extcon_set_state_sync(info.edev, EXTCON_USB_HOST, true);
    } else {
    if (vbus)
    extcon_set_state_sync(info.edev, EXTCON_USB, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn usb_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t usb_irq_handler(int irq, void *dev_id)
    {
    struct usb_extcon_info *info = dev_id;
    queue_delayed_work(system_power_efficient_wq, &info.wq_detcable,
    info.debounce_jiffies);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn usb_extcon_probe(pdev: *mut platform_device) -> c_int {
    static int usb_extcon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct usb_extcon_info *info;
    int ret;
    if (!np)
    return -EINVAL;
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = dev;
    info.id_gpiod = devm_gpiod_get_optional(&pdev.dev, "id", GPIOD_IN);
    info.vbus_gpiod = devm_gpiod_get_optional(&pdev.dev, "vbus",
    GPIOD_IN);
    if (!info.id_gpiod && !info.vbus_gpiod) {
    dev_err(dev, "failed to get gpios\n");
    return -ENODEV;
    }
    if (IS_ERR(info.id_gpiod))
    return PTR_ERR(info.id_gpiod);
    if (IS_ERR(info.vbus_gpiod))
    return PTR_ERR(info.vbus_gpiod);
    info.edev = devm_extcon_dev_allocate(dev, usb_extcon_cable);
    if (IS_ERR(info.edev)) {
    dev_err(dev, "failed to allocate extcon device\n");
    return -ENOMEM;
    }
    ret = devm_extcon_dev_register(dev, info.edev);
    if (ret < 0) {
    dev_err(dev, "failed to register extcon device\n");
    return ret;
    }
    if (info.id_gpiod)
    ret = gpiod_set_debounce(info.id_gpiod,
    USB_GPIO_DEBOUNCE_MS * 1000);
    if (!ret && info.vbus_gpiod)
    ret = gpiod_set_debounce(info.vbus_gpiod,
    USB_GPIO_DEBOUNCE_MS * 1000);
    if (ret < 0)
    info.debounce_jiffies = msecs_to_jiffies(USB_GPIO_DEBOUNCE_MS);
    INIT_DELAYED_WORK(&info.wq_detcable, usb_extcon_detect_cable);
    if (info.id_gpiod) {
    info.id_irq = gpiod_to_irq(info.id_gpiod);
    if (info.id_irq < 0) {
    dev_err(dev, "failed to get ID IRQ\n");
    return info.id_irq;
    }
    ret = devm_request_threaded_irq(dev, info.id_irq, core::ptr::null_mut(),
    usb_irq_handler,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    pdev.name, info);
    if (ret < 0) {
    dev_err(dev, "failed to request handler for ID IRQ\n");
    return ret;
    }
    }
    if (info.vbus_gpiod) {
    info.vbus_irq = gpiod_to_irq(info.vbus_gpiod);
    if (info.vbus_irq < 0) {
    dev_err(dev, "failed to get VBUS IRQ\n");
    return info.vbus_irq;
    }
    ret = devm_request_threaded_irq(dev, info.vbus_irq, core::ptr::null_mut(),
    usb_irq_handler,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    pdev.name, info);
    if (ret < 0) {
    dev_err(dev, "failed to request handler for VBUS IRQ\n");
    return ret;
    }
    }
    platform_set_drvdata(pdev, info);
    device_set_wakeup_capable(&pdev.dev, true);
// Perform initial detection
    usb_extcon_detect_cable(&info.wq_detcable.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_extcon_remove(pdev: *mut platform_device) {
    static void usb_extcon_remove(struct platform_device *pdev)
    {
    struct usb_extcon_info *info = platform_get_drvdata(pdev);
    cancel_delayed_work_sync(&info.wq_detcable);
    device_init_wakeup(&pdev.dev, false);
    }

#[no_mangle]
unsafe extern "C" fn usb_extcon_suspend(dev: *mut device) -> c_int {
    static int usb_extcon_suspend(struct device *dev)
    {
    struct usb_extcon_info *info = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (device_may_wakeup(dev)) {
    if (info.id_gpiod) {
    ret = enable_irq_wake(info.id_irq);
    if (ret)
    return ret;
    }
    if (info.vbus_gpiod) {
    ret = enable_irq_wake(info.vbus_irq);
    if (ret) {
    if (info.id_gpiod)
    disable_irq_wake(info.id_irq);
    return ret;
    }
    }
    }
    if (!device_may_wakeup(dev))
    pinctrl_pm_select_sleep_state(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usb_extcon_resume(dev: *mut device) -> c_int {
    static int usb_extcon_resume(struct device *dev)
    {
    struct usb_extcon_info *info = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (!device_may_wakeup(dev))
    pinctrl_pm_select_default_state(dev);
    if (device_may_wakeup(dev)) {
    if (info.id_gpiod) {
    ret = disable_irq_wake(info.id_irq);
    if (ret)
    return ret;
    }
    if (info.vbus_gpiod) {
    ret = disable_irq_wake(info.vbus_irq);
    if (ret) {
    if (info.id_gpiod)
    enable_irq_wake(info.id_irq);
    return ret;
    }
    }
    }
    queue_delayed_work(system_power_efficient_wq,
    &info.wq_detcable, 0);
    return ret;
    }

    static SIMPLE_DEV_PM_OPS(usb_extcon_pm_ops,
    usb_extcon_suspend, usb_extcon_resume);
    static const struct of_device_id usb_extcon_dt_match[] = {
    { .compatible = "linux,extcon-usb-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, usb_extcon_dt_match);
    static const struct platform_device_id usb_extcon_platform_ids[] = {
    { .name = "extcon-usb-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, usb_extcon_platform_ids);
    static struct platform_driver usb_extcon_driver = {
    .probe		= usb_extcon_probe,
    .remove		= usb_extcon_remove,
    .driver		= {
    .name	= "extcon-usb-gpio",
    .pm	= &usb_extcon_pm_ops,
    .of_match_table = usb_extcon_dt_match,
    },
    .id_table = usb_extcon_platform_ids,
    };
    module_platform_driver(usb_extcon_driver);
    MODULE_AUTHOR("Roger Quadros <rogerq@ti.com>");
    MODULE_DESCRIPTION("USB GPIO extcon driver");
    MODULE_LICENSE("GPL v2");
