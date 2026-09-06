//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-smo8800.c
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
// dell-smo8800.c - Dell Latitude ACPI SMO88XX freefall sensor driver
//
// Copyright (C) 2012 Sonal Santan <sonal.santan@gmail.com>
// Copyright (C) 2014 Pali Rohár <pali@kernel.org>
//
// This is loosely based on lis3lv02d driver.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smo8800_device {
    pub /: *mut *mut u32 irq; / acpi device irq,
    pub /: *mut *mut atomic_t counter; / count after last read,
    pub /: *mut *mut miscdevice miscdev; / for /dev/freefall,
    pub /: *mut *mut unsigned long misc_opened; / whether the device is open,
    pub /: *mut *mut wait_queue_head_t misc_wait; / Wait queue for the misc dev,
    pub /: *mut *mut *mut device dev; / acpi device,
}

#[no_mangle]
unsafe extern "C" fn smo8800_interrupt_quick(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smo8800_interrupt_quick(int irq, void *data)
    {
    struct smo8800_device *smo8800 = data;
    atomic_inc(&smo8800.counter);
    wake_up_interruptible(&smo8800.misc_wait);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn smo8800_interrupt_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smo8800_interrupt_thread(int irq, void *data)
    {
    struct smo8800_device *smo8800 = data;
    dev_info(smo8800.dev, "detected free fall\n");
    return IRQ_HANDLED;
    }
    static ssize_t smo8800_misc_read(struct file *file, char __user *buf,
    size_t count, loff_t *pos)
    {
    struct smo8800_device *smo8800 = container_of(file.private_data,
    struct smo8800_device, miscdev);
    let mut data: u32 = 0;
    unsigned char byte_data;
    let mut retval: isize = 1;
    if (count < 1)
    return -EINVAL;
    atomic_set(&smo8800.counter, 0);
    retval = wait_event_interruptible(smo8800.misc_wait,
    (data = atomic_xchg(&smo8800.counter, 0)));
    if (retval)
    return retval;
    retval = 1;
    byte_data = min_t(u32, data, 255);
    if (put_user(byte_data, buf))
    retval = -EFAULT;
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn smo8800_misc_open(inode: *mut inode, file: *mut file) -> c_int {
    static int smo8800_misc_open(struct inode *inode, struct file *file)
    {
    struct smo8800_device *smo8800 = container_of(file.private_data,
    struct smo8800_device, miscdev);
    if (test_and_set_bit(0, &smo8800.misc_opened))
    return -EBUSY; /* already open */
    atomic_set(&smo8800.counter, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smo8800_misc_release(inode: *mut inode, file: *mut file) -> c_int {
    static int smo8800_misc_release(struct inode *inode, struct file *file)
    {
    struct smo8800_device *smo8800 = container_of(file.private_data,
    struct smo8800_device, miscdev);
    clear_bit(0, &smo8800.misc_opened); /* release the device */
    return 0;
    }
    static const struct file_operations smo8800_misc_fops = {
    .owner = THIS_MODULE,
    .read = smo8800_misc_read,
    .open = smo8800_misc_open,
    .release = smo8800_misc_release,
    };
#[no_mangle]
unsafe extern "C" fn smo8800_probe(device: *mut platform_device) -> c_int {
    static int smo8800_probe(struct platform_device *device)
    {
    int err;
    struct smo8800_device *smo8800;
    smo8800 = devm_kzalloc(&device.dev, sizeof(*smo8800), GFP_KERNEL);
    if (!smo8800) {
    dev_err(&device.dev, "failed to allocate device data\n");
    return -ENOMEM;
    }
    smo8800.dev = &device.dev;
    smo8800.miscdev.minor = MISC_DYNAMIC_MINOR;
    smo8800.miscdev.name = "freefall";
    smo8800.miscdev.fops = &smo8800_misc_fops;
    init_waitqueue_head(&smo8800.misc_wait);
    err = misc_register(&smo8800.miscdev);
    if (err) {
    dev_err(&device.dev, "failed to register misc dev: %d\n", err);
    return err;
    }
    platform_set_drvdata(device, smo8800);
    err = platform_get_irq(device, 0);
    if (err < 0)
    goto error;
    smo8800.irq = err;
    err = request_threaded_irq(smo8800.irq, smo8800_interrupt_quick,
    smo8800_interrupt_thread,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    DRIVER_NAME, smo8800);
    if (err) {
    dev_err(&device.dev,
    "failed to request thread for IRQ %d: %d\n",
    smo8800.irq, err);
    goto error;
    }
    dev_dbg(&device.dev, "device /dev/freefall registered with IRQ %d\n",
    smo8800.irq);
    return 0;
    error:
    misc_deregister(&smo8800.miscdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn smo8800_remove(device: *mut platform_device) {
    static void smo8800_remove(struct platform_device *device)
    {
    struct smo8800_device *smo8800 = platform_get_drvdata(device);
    free_irq(smo8800.irq, smo8800);
    misc_deregister(&smo8800.miscdev);
    dev_dbg(&device.dev, "device /dev/freefall unregistered\n");
    }
    static struct platform_driver smo8800_driver = {
    .probe = smo8800_probe,
    .remove = smo8800_remove,
    .driver = {
    .name = DRIVER_NAME,
    .acpi_match_table = smo8800_ids,
    },
    };
    module_platform_driver(smo8800_driver);
    MODULE_DESCRIPTION("Dell Latitude freefall driver (ACPI SMO88XX)");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sonal Santan, Pali Rohár");
