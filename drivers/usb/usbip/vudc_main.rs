//! Automatically rewritten from C to Rust
//! Source: drivers/usb/usbip/vudc_main.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2015 Karol Kosik <karo9@interia.eu>
// Copyright (C) 2015-2016 Samsung Electronics
// Igor Kotrasinski <i.kotrasinsk@samsung.com>
// Krzysztof Opasiak <k.opasiak@samsung.com>
//

    let mut vudc_number: static unsigned int = 1;
    module_param_named(num, vudc_number, uint, S_IRUGO);
    MODULE_PARM_DESC(num, "number of emulated controllers");
    static struct platform_driver vudc_driver = {
    .probe		= vudc_probe,
    .remove		= vudc_remove,
    .driver		= {
    .name	= GADGET_NAME,
    .dev_groups = vudc_groups,
    },
    };
    static LIST_HEAD(vudc_devices);
#[no_mangle]
unsafe extern "C" fn vudc_init() -> int __init {
    static int __init vudc_init(void)
    {
    let mut retval: c_int = -ENOMEM;
    int i;
    struct vudc_device *udc_dev = core::ptr::null_mut(), *udc_dev2 = core::ptr::null_mut();
    if (usb_disabled())
    return -ENODEV;
    if (vudc_number < 1) {
    pr_err("Number of emulated UDC must be no less than 1");
    return -EINVAL;
    }
    retval = platform_driver_register(&vudc_driver);
    if (retval < 0)
    goto out;
    for (i = 0; i < vudc_number; i++) {
    udc_dev = alloc_vudc_device(i);
    if (!udc_dev) {
    retval = -ENOMEM;
    goto cleanup;
    }
    retval = platform_device_add(udc_dev.pdev);
    if (retval < 0) {
    put_vudc_device(udc_dev);
    goto cleanup;
    }
    list_add_tail(&udc_dev.dev_entry, &vudc_devices);
    if (!platform_get_drvdata(udc_dev.pdev)) {
//
// The udc was added successfully but its probe
// function failed for some reason.
//
    retval = -EINVAL;
    goto cleanup;
    }
    }
    goto out;
    cleanup:
    list_for_each_entry_safe(udc_dev, udc_dev2, &vudc_devices, dev_entry) {
    list_del(&udc_dev.dev_entry);
//
// Just do platform_device_del() here, put_vudc_device()
// calls the platform_device_put()
//
    platform_device_del(udc_dev.pdev);
    put_vudc_device(udc_dev);
    }
    platform_driver_unregister(&vudc_driver);
    out:
    return retval;
    }
    module_init(vudc_init);
#[no_mangle]
unsafe extern "C" fn vudc_cleanup() -> void __exit {
    static void __exit vudc_cleanup(void)
    {
    struct vudc_device *udc_dev = core::ptr::null_mut(), *udc_dev2 = core::ptr::null_mut();
    list_for_each_entry_safe(udc_dev, udc_dev2, &vudc_devices, dev_entry) {
    list_del(&udc_dev.dev_entry);
//
// Just do platform_device_del() here, put_vudc_device()
// calls the platform_device_put()
//
    platform_device_del(udc_dev.pdev);
    put_vudc_device(udc_dev);
    }
    platform_driver_unregister(&vudc_driver);
    }
    module_exit(vudc_cleanup);
    MODULE_DESCRIPTION("USB over IP Device Controller");
    MODULE_AUTHOR("Krzysztof Opasiak, Karol Kosik, Igor Kotrasinski");
    MODULE_LICENSE("GPL");
