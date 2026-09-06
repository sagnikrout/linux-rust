//! Automatically rewritten from C to Rust
//! Source: drivers/char/xillybus/xillybus_of.c
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
// linux/drivers/misc/xillybus_of.c
//
// Copyright 2011 Xillybus Ltd, http://xillybus.com
//
// Driver for the Xillybus FPGA/host framework using Open Firmware.
//

    MODULE_DESCRIPTION("Xillybus driver for Open Firmware");
    MODULE_AUTHOR("Eli Billauer, Xillybus Ltd.");
    MODULE_ALIAS("xillybus_of");
    MODULE_LICENSE("GPL v2");
    static const char xillyname[] = "xillybus_of";
// Match table for of_platform binding
    static const struct of_device_id xillybus_of_match[] = {
    { .compatible = "xillybus,xillybus-1.00.a", },
    { .compatible = "xlnx,xillybus-1.00.a", }, /* Deprecated */
    {}
    };
    MODULE_DEVICE_TABLE(of, xillybus_of_match);
#[no_mangle]
unsafe extern "C" fn xilly_drv_probe(op: *mut platform_device) -> c_int {
    static int xilly_drv_probe(struct platform_device *op)
    {
    struct device *dev = &op.dev;
    struct xilly_endpoint *endpoint;
    int rc;
    int irq;
    endpoint = xillybus_init_endpoint(dev);
    if (!endpoint)
    return -ENOMEM;
    dev_set_drvdata(dev, endpoint);
    endpoint.owner = THIS_MODULE;
    endpoint.registers = devm_platform_ioremap_resource(op, 0);
    if (IS_ERR(endpoint.registers))
    return PTR_ERR(endpoint.registers);
    irq = platform_get_irq(op, 0);
    rc = devm_request_irq(dev, irq, xillybus_isr, 0, xillyname, endpoint);
    if (rc)
    return -ENODEV;
    return xillybus_endpoint_discovery(endpoint);
    }
#[no_mangle]
unsafe extern "C" fn xilly_drv_remove(op: *mut platform_device) {
    static void xilly_drv_remove(struct platform_device *op)
    {
    struct device *dev = &op.dev;
    struct xilly_endpoint *endpoint = dev_get_drvdata(dev);
    xillybus_endpoint_remove(endpoint);
    }
    static struct platform_driver xillybus_platform_driver = {
    .probe = xilly_drv_probe,
    .remove = xilly_drv_remove,
    .driver = {
    .name = xillyname,
    .of_match_table = xillybus_of_match,
    },
    };
    module_platform_driver(xillybus_platform_driver);
