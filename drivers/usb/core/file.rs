//! Automatically rewritten from C to Rust
//! Source: drivers/usb/core/file.c
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


// SPDX-License-Identifier: GPL-2.0
//
// drivers/usb/core/file.c
//
// (C) Copyright Linus Torvalds 1999
// (C) Copyright Johannes Erdfelt 1999-2001
// (C) Copyright Andreas Gal 1999
// (C) Copyright Gregory P. Smith 1999
// (C) Copyright Deti Fliegl 1999 (new USB architecture)
// (C) Copyright Randy Dunlap 2000
// (C) Copyright David Brownell 2000-2001 (kernel hotplug, usb_device_id,
// more docs, etc)
// (C) Copyright Yggdrasil Computing, Inc. 2000
// (usb_device_id matching changes by Adam J. Richter)
// (C) Copyright Greg Kroah-Hartman 2002-2003
//
// Released under the GPLv2 only.
//

pub const MAX_USB_MINORS: c_int = 256;
    static const struct file_operations *usb_minors[MAX_USB_MINORS];
    static DECLARE_RWSEM(minor_rwsem);
#[no_mangle]
unsafe extern "C" fn usb_open(inode: *mut inode, file: *mut file) -> c_int {
    static int usb_open(struct inode *inode, struct file *file)
    {
    let mut err: c_int = -ENODEV;
    const struct file_operations *new_fops;
    down_read(&minor_rwsem);
    new_fops = fops_get(usb_minors[iminor(inode)]);
    if (!new_fops)
    goto done;
    replace_fops(file, new_fops);
// Curiouser and curiouser... NULL ->open() as "no device" ?
    if (file.f_op.open)
    err = file.f_op.open(inode, file);
    done:
    up_read(&minor_rwsem);
    return err;
    }
    static const struct file_operations usb_fops = {
    .owner =	THIS_MODULE,
    .open =		usb_open,
    .llseek =	noop_llseek,
    };
    static char *usb_devnode(const struct device *dev, umode_t *mode)
    {
    struct usb_class_driver *drv;
    drv = dev_get_drvdata(dev);
    if (!drv || !drv.devnode)
    return core::ptr::null_mut();
    return drv.devnode(dev, mode);
    }
    const struct class usbmisc_class = {
    .name		= "usbmisc",
    .devnode	= usb_devnode,
    };
#[no_mangle]
pub unsafe extern "C" fn usb_major_init() -> c_int {
    int usb_major_init(void)
    {
    int error;
    error = register_chrdev(USB_MAJOR, "usb", &usb_fops);
    if (error)
    printk(KERN_ERR "Unable to get major %d for usb devices\n",
    USB_MAJOR);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn usb_major_cleanup() {
    void usb_major_cleanup(void)
    {
    unregister_chrdev(USB_MAJOR, "usb");
    }
//
// usb_register_dev - register a USB device, and ask for a minor number
// @intf: pointer to the usb_interface that is being registered
// @class_driver: pointer to the usb_class_driver for this device
//
// This should be called by all USB drivers that use the USB major number.
// If CONFIG_USB_DYNAMIC_MINORS is enabled, the minor number will be
// dynamically allocated out of the list of available ones.  If it is not
// enabled, the minor number will be based on the next available free minor,
// starting at the class_driver->minor_base.
//
// This function also creates a usb class device in the sysfs tree.
//
// usb_deregister_dev() must be called when the driver is done with
// the minor numbers given out by this function.
//
// Return: -EINVAL if something bad happens with trying to register a
// device, and 0 on success.
//
    int usb_register_dev(struct usb_interface *intf,
    struct usb_class_driver *class_driver)
    {
    let mut retval: c_int = 0;
    let mut minor_base: c_int = class_driver.minor_base;
    int minor;
    char name[20];

//
// We don't care what the device tries to start at, we want to start
// at zero to pack the devices into the smallest available space with
// no holes in the minor range.
//
    minor_base = 0;

    if (class_driver.fops == core::ptr::null_mut())
    return -EINVAL;
    if (intf.minor >= 0)
    return -EADDRINUSE;
    dev_dbg(&intf.dev, "looking for a minor, starting at %d\n", minor_base);
    down_write(&minor_rwsem);
    for (minor = minor_base; minor < MAX_USB_MINORS; ++minor) {
    if (usb_minors[minor])
    continue;
    usb_minors[minor] = class_driver.fops;
    intf.minor = minor;
    break;
    }
    if (intf.minor < 0) {
    up_write(&minor_rwsem);
    return -EXFULL;
    }
// create a usb class device for this usb interface
    snprintf(name, sizeof(name), class_driver.name, minor - minor_base);
    intf.usb_dev = device_create(&usbmisc_class, &intf.dev,
    MKDEV(USB_MAJOR, minor), class_driver,
    "%s", kbasename(name));
    if (IS_ERR(intf.usb_dev)) {
    usb_minors[minor] = core::ptr::null_mut();
    intf.minor = -1;
    retval = PTR_ERR(intf.usb_dev);
    }
    up_write(&minor_rwsem);
    return retval;
    }
    EXPORT_SYMBOL_GPL(usb_register_dev);
//
// usb_deregister_dev - deregister a USB device's dynamic minor.
// @intf: pointer to the usb_interface that is being deregistered
// @class_driver: pointer to the usb_class_driver for this device
//
// Used in conjunction with usb_register_dev().  This function is called
// when the USB driver is finished with the minor numbers gotten from a
// call to usb_register_dev() (usually when the device is disconnected
// from the system.)
//
// This function also removes the usb class device from the sysfs tree.
//
// This should be called by all drivers that use the USB major number.
//
    void usb_deregister_dev(struct usb_interface *intf,
    struct usb_class_driver *class_driver)
    {
    if (intf.minor == -1)
    return;
    dev_dbg(&intf.dev, "removing %d minor\n", intf.minor);
    device_destroy(&usbmisc_class, MKDEV(USB_MAJOR, intf.minor));
    down_write(&minor_rwsem);
    usb_minors[intf.minor] = core::ptr::null_mut();
    up_write(&minor_rwsem);
    intf.usb_dev = core::ptr::null_mut();
    intf.minor = -1;
    }
    EXPORT_SYMBOL_GPL(usb_deregister_dev);
