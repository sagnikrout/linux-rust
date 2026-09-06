//! Automatically rewritten from C to Rust
//! Source: drivers/char/scx200_gpio.c
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
// linux/drivers/char/scx200_gpio.c
    National Semiconductor SCx200 GPIO driver.  Allows a user space
    process to play with the GPIO pins.
    Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com> */

    static struct platform_device *pdev;
    MODULE_AUTHOR("Christer Weinigel <wingel@nano-system.com>");
    MODULE_DESCRIPTION("NatSemi/AMD SCx200 GPIO Pin Driver");
    MODULE_LICENSE("GPL");
    static int major = 0;		/* default to dynamic major */
    module_param(major, int, 0);
    MODULE_PARM_DESC(major, "Major device number");

    struct nsc_gpio_ops scx200_gpio_ops = {
    .owner		= THIS_MODULE,
    .gpio_config	= scx200_gpio_configure,
    .gpio_dump	= nsc_gpio_dump,
    .gpio_get	= scx200_gpio_get,
    .gpio_set	= scx200_gpio_set,
    .gpio_change	= scx200_gpio_change,
    .gpio_current	= scx200_gpio_current
    };
    EXPORT_SYMBOL_GPL(scx200_gpio_ops);
#[no_mangle]
unsafe extern "C" fn scx200_gpio_open(inode: *mut inode, file: *mut file) -> c_int {
    static int scx200_gpio_open(struct inode *inode, struct file *file)
    {
    let mut m: unsigned = iminor(inode);
    file.private_data = &scx200_gpio_ops;
    if (m >= MAX_PINS)
    return -EINVAL;
    return nonseekable_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn scx200_gpio_release(inode: *mut inode, file: *mut file) -> c_int {
    static int scx200_gpio_release(struct inode *inode, struct file *file)
    {
    return 0;
    }
    static const struct file_operations scx200_gpio_fileops = {
    .owner   = THIS_MODULE,
    .write   = nsc_gpio_write,
    .read    = nsc_gpio_read,
    .open    = scx200_gpio_open,
    .release = scx200_gpio_release,
    };
    static struct cdev scx200_gpio_cdev;  /* use 1 cdev for all pins */
#[no_mangle]
unsafe extern "C" fn scx200_gpio_init() -> int __init {
    static int __init scx200_gpio_init(void)
    {
    int rc;
    dev_t devid;
    if (!scx200_gpio_present()) {
    printk(KERN_ERR DRVNAME ": no SCx200 gpio present\n");
    return -ENODEV;
    }
// support dev_dbg() with pdev->dev
    pdev = platform_device_alloc(DRVNAME, 0);
    if (!pdev)
    return -ENOMEM;
    rc = platform_device_add(pdev);
    if (rc)
    goto undo_malloc;
// nsc_gpio uses dev_dbg(), so needs this
    scx200_gpio_ops.dev = &pdev.dev;
    if (major) {
    devid = MKDEV(major, 0);
    rc = register_chrdev_region(devid, MAX_PINS, "scx200_gpio");
    } else {
    rc = alloc_chrdev_region(&devid, 0, MAX_PINS, "scx200_gpio");
    major = MAJOR(devid);
    }
    if (rc < 0) {
    dev_err(&pdev.dev, "SCx200 chrdev_region err: %d\n", rc);
    goto undo_platform_device_add;
    }
    cdev_init(&scx200_gpio_cdev, &scx200_gpio_fileops);
    cdev_add(&scx200_gpio_cdev, devid, MAX_PINS);
    return 0; /* succeed */
    undo_platform_device_add:
    platform_device_del(pdev);
    undo_malloc:
    platform_device_put(pdev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn scx200_gpio_cleanup() -> void __exit {
    static void __exit scx200_gpio_cleanup(void)
    {
    cdev_del(&scx200_gpio_cdev);
// cdev_put(&scx200_gpio_cdev);
    unregister_chrdev_region(MKDEV(major, 0), MAX_PINS);
    platform_device_unregister(pdev);
    }
    module_init(scx200_gpio_init);
    module_exit(scx200_gpio_cleanup);
