//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/htcpen.c
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
// HTC Shift touchscreen driver
//
// Copyright (C) 2008 Pau Oliva Fora <pof@eslack.org>
//

    MODULE_AUTHOR("Pau Oliva Fora <pau@eslack.org>");
    MODULE_DESCRIPTION("HTC Shift touchscreen driver");
    MODULE_LICENSE("GPL");
pub const HTCPEN_PORT_IRQ_CLEAR: c_uint = 0x068;
pub const HTCPEN_PORT_INIT: c_uint = 0x06c;
pub const HTCPEN_PORT_INDEX: c_uint = 0x0250;
pub const HTCPEN_PORT_DATA: c_uint = 0x0251;
pub const HTCPEN_IRQ: c_int = 3;
pub const DEVICE_ENABLE: c_uint = 0xa2;
pub const DEVICE_DISABLE: c_uint = 0xa3;
pub const X_INDEX: c_int = 3;
pub const Y_INDEX: c_int = 5;
pub const TOUCH_INDEX: c_uint = 0xb;
pub const LSB_XY_INDEX: c_uint = 0xc;
pub const X_AXIS_MAX: c_int = 2040;
pub const Y_AXIS_MAX: c_int = 2040;
    static bool invert_x;
    module_param(invert_x, bool, 0644);
    MODULE_PARM_DESC(invert_x, "If set, X axis is inverted");
    static bool invert_y;
    module_param(invert_y, bool, 0644);
    MODULE_PARM_DESC(invert_y, "If set, Y axis is inverted");
#[no_mangle]
unsafe extern "C" fn htcpen_interrupt(irq: c_int, handle: *mut c_void) -> irqreturn_t {
    static irqreturn_t htcpen_interrupt(int irq, void *handle)
    {
    struct input_dev *htcpen_dev = handle;
    unsigned short x, y, xy;
// 0 = press; 1 = release
    outb_p(TOUCH_INDEX, HTCPEN_PORT_INDEX);
    if (inb_p(HTCPEN_PORT_DATA)) {
    input_report_key(htcpen_dev, BTN_TOUCH, 0);
    } else {
    outb_p(X_INDEX, HTCPEN_PORT_INDEX);
    x = inb_p(HTCPEN_PORT_DATA);
    outb_p(Y_INDEX, HTCPEN_PORT_INDEX);
    y = inb_p(HTCPEN_PORT_DATA);
    outb_p(LSB_XY_INDEX, HTCPEN_PORT_INDEX);
    xy = inb_p(HTCPEN_PORT_DATA);
// get high resolution value of X and Y using LSB
    x = X_AXIS_MAX - ((x * 8) + ((xy >> 4) & 0xf));
    y = (y * 8) + (xy & 0xf);
    if (invert_x)
    x = X_AXIS_MAX - x;
    if (invert_y)
    y = Y_AXIS_MAX - y;
    if (x != X_AXIS_MAX && x != 0) {
    input_report_key(htcpen_dev, BTN_TOUCH, 1);
    input_report_abs(htcpen_dev, ABS_X, x);
    input_report_abs(htcpen_dev, ABS_Y, y);
    }
    }
    input_sync(htcpen_dev);
    inb_p(HTCPEN_PORT_IRQ_CLEAR);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn htcpen_open(dev: *mut input_dev) -> c_int {
    static int htcpen_open(struct input_dev *dev)
    {
    outb_p(DEVICE_ENABLE, HTCPEN_PORT_INIT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn htcpen_close(dev: *mut input_dev) {
    static void htcpen_close(struct input_dev *dev)
    {
    outb_p(DEVICE_DISABLE, HTCPEN_PORT_INIT);
    synchronize_irq(HTCPEN_IRQ);
    }
#[no_mangle]
unsafe extern "C" fn htcpen_isa_probe(dev: *mut device, id: c_uint) -> c_int {
    static int htcpen_isa_probe(struct device *dev, unsigned int id)
    {
    struct input_dev *htcpen_dev;
    let mut err: c_int = -EBUSY;
    if (!request_region(HTCPEN_PORT_IRQ_CLEAR, 1, "htcpen")) {
    printk(KERN_ERR "htcpen: unable to get IO region 0x%x\n",
    HTCPEN_PORT_IRQ_CLEAR);
    goto request_region1_failed;
    }
    if (!request_region(HTCPEN_PORT_INIT, 1, "htcpen")) {
    printk(KERN_ERR "htcpen: unable to get IO region 0x%x\n",
    HTCPEN_PORT_INIT);
    goto request_region2_failed;
    }
    if (!request_region(HTCPEN_PORT_INDEX, 2, "htcpen")) {
    printk(KERN_ERR "htcpen: unable to get IO region 0x%x\n",
    HTCPEN_PORT_INDEX);
    goto request_region3_failed;
    }
    htcpen_dev = input_allocate_device();
    if (!htcpen_dev) {
    printk(KERN_ERR "htcpen: can't allocate device\n");
    err = -ENOMEM;
    goto input_alloc_failed;
    }
    htcpen_dev.name = "HTC Shift EC TouchScreen";
    htcpen_dev.id.bustype = BUS_ISA;
    htcpen_dev.evbit[0] = BIT_MASK(EV_ABS) | BIT_MASK(EV_KEY);
    htcpen_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(htcpen_dev, ABS_X, 0, X_AXIS_MAX, 0, 0);
    input_set_abs_params(htcpen_dev, ABS_Y, 0, Y_AXIS_MAX, 0, 0);
    htcpen_dev.open = htcpen_open;
    htcpen_dev.close = htcpen_close;
    err = request_irq(HTCPEN_IRQ, htcpen_interrupt, 0, "htcpen",
    htcpen_dev);
    if (err) {
    printk(KERN_ERR "htcpen: irq busy\n");
    goto request_irq_failed;
    }
    inb_p(HTCPEN_PORT_IRQ_CLEAR);
    err = input_register_device(htcpen_dev);
    if (err)
    goto input_register_failed;
    dev_set_drvdata(dev, htcpen_dev);
    return 0;
    input_register_failed:
    free_irq(HTCPEN_IRQ, htcpen_dev);
    request_irq_failed:
    input_free_device(htcpen_dev);
    input_alloc_failed:
    release_region(HTCPEN_PORT_INDEX, 2);
    request_region3_failed:
    release_region(HTCPEN_PORT_INIT, 1);
    request_region2_failed:
    release_region(HTCPEN_PORT_IRQ_CLEAR, 1);
    request_region1_failed:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn htcpen_isa_remove(dev: *mut device, id: c_uint) {
    static void htcpen_isa_remove(struct device *dev, unsigned int id)
    {
    struct input_dev *htcpen_dev = dev_get_drvdata(dev);
    input_unregister_device(htcpen_dev);
    free_irq(HTCPEN_IRQ, htcpen_dev);
    release_region(HTCPEN_PORT_INDEX, 2);
    release_region(HTCPEN_PORT_INIT, 1);
    release_region(HTCPEN_PORT_IRQ_CLEAR, 1);
    }

    static int htcpen_isa_suspend(struct device *dev, unsigned int n,
    pm_message_t state)
    {
    outb_p(DEVICE_DISABLE, HTCPEN_PORT_INIT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn htcpen_isa_resume(dev: *mut device, n: c_uint) -> c_int {
    static int htcpen_isa_resume(struct device *dev, unsigned int n)
    {
    outb_p(DEVICE_ENABLE, HTCPEN_PORT_INIT);
    return 0;
    }

    static struct isa_driver htcpen_isa_driver = {
    .probe		= htcpen_isa_probe,
    .remove		= htcpen_isa_remove,

    .suspend	= htcpen_isa_suspend,
    .resume		= htcpen_isa_resume,

    .driver = {
    .owner	= THIS_MODULE,
    .name	= "htcpen",
    }
    };
    static const struct dmi_system_id htcshift_dmi_table[] __initconst = {
    {
    .ident = "Shift",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "High Tech Computer Corp"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Shift"),
    },
    },
    { }
    };
    MODULE_DEVICE_TABLE(dmi, htcshift_dmi_table);
#[no_mangle]
unsafe extern "C" fn htcpen_isa_init() -> int __init {
    static int __init htcpen_isa_init(void)
    {
    if (!dmi_check_system(htcshift_dmi_table))
    return -ENODEV;
    return isa_register_driver(&htcpen_isa_driver, 1);
    }
#[no_mangle]
unsafe extern "C" fn htcpen_isa_exit() -> void __exit {
    static void __exit htcpen_isa_exit(void)
    {
    isa_unregister_driver(&htcpen_isa_driver);
    }
    module_init(htcpen_isa_init);
    module_exit(htcpen_isa_exit);
