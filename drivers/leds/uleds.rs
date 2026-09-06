//! Automatically rewritten from C to Rust
//! Source: drivers/leds/uleds.c
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
// Userspace driver for the LED subsystem
//
// Copyright (C) 2016 David Lechner <david@lechnology.com>
//
// Based on uinput.c: Aristeu Sergio Rozanski Filho <aris@cathedrallabs.org>
//

    enum uleds_state {
    ULEDS_STATE_UNKNOWN,
    ULEDS_STATE_REGISTERED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uleds_device {
    pub user_dev: uleds_user_dev,
    pub led_cdev: led_classdev,
    pub mutex: mutex,
    pub state: enum uleds_state,
    pub waitq: wait_queue_head_t,
    pub brightness: c_int,
    pub new_data: bool,
}

    static struct miscdevice uleds_misc;
    static void uleds_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct uleds_device *udev = container_of(led_cdev, struct uleds_device,
    led_cdev);
    if (udev.brightness != brightness) {
    udev.brightness = brightness;
    udev.new_data = true;
    wake_up_interruptible(&udev.waitq);
    }
    }
#[no_mangle]
unsafe extern "C" fn uleds_open(inode: *mut inode, file: *mut file) -> c_int {
    static int uleds_open(struct inode *inode, struct file *file)
    {
    struct uleds_device *udev;
    udev = kzalloc_obj(*udev);
    if (!udev)
    return -ENOMEM;
    udev.led_cdev.name = udev.user_dev.name;
    udev.led_cdev.brightness_set = uleds_brightness_set;
    mutex_init(&udev.mutex);
    init_waitqueue_head(&udev.waitq);
    udev.state = ULEDS_STATE_UNKNOWN;
    file.private_data = udev;
    stream_open(inode, file);
    return 0;
    }
    static ssize_t uleds_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct uleds_device *udev = file.private_data;
    const char *name;
    int ret;
    if (count == 0)
    return 0;
    ret = mutex_lock_interruptible(&udev.mutex);
    if (ret)
    return ret;
    if (udev.state == ULEDS_STATE_REGISTERED) {
    ret = -EBUSY;
    goto out;
    }
    if (count != sizeof(struct uleds_user_dev)) {
    ret = -EINVAL;
    goto out;
    }
    if (copy_from_user(&udev.user_dev, buffer,
    sizeof(struct uleds_user_dev))) {
    ret = -EFAULT;
    goto out;
    }
    name = udev.user_dev.name;
    if (!name[0] || !strcmp(name, ".") || !strcmp(name, "..") ||
    strnchr(name, sizeof(udev.user_dev.name), '/') ||
    !strnchr(name, sizeof(udev.user_dev.name), '\0')) {
    ret = -EINVAL;
    goto out;
    }
    if (udev.user_dev.max_brightness <= 0) {
    ret = -EINVAL;
    goto out;
    }
    udev.led_cdev.max_brightness = udev.user_dev.max_brightness;
    ret = devm_led_classdev_register(uleds_misc.this_device,
    &udev.led_cdev);
    if (ret < 0)
    goto out;
    udev.new_data = true;
    udev.state = ULEDS_STATE_REGISTERED;
    ret = count;
    out:
    mutex_unlock(&udev.mutex);
    return ret;
    }
    static ssize_t uleds_read(struct file *file, char __user *buffer, size_t count,
    loff_t *ppos)
    {
    struct uleds_device *udev = file.private_data;
    ssize_t retval;
    if (count < sizeof(udev.brightness))
    return 0;
    do {
    retval = mutex_lock_interruptible(&udev.mutex);
    if (retval)
    return retval;
    if (udev.state != ULEDS_STATE_REGISTERED) {
    retval = -ENODEV;
    } else if (!udev.new_data && (file.f_flags & O_NONBLOCK)) {
    retval = -EAGAIN;
    } else if (udev.new_data) {
    if (copy_to_user(buffer, &udev.brightness,
    sizeof(udev.brightness))) {
    retval = -EFAULT;
    } else {
    udev.new_data = false;
    retval = sizeof(udev.brightness);
    }
    }
    mutex_unlock(&udev.mutex);
    if (retval)
    break;
    if (!(file.f_flags & O_NONBLOCK))
    retval = wait_event_interruptible(udev.waitq,
    udev.new_data ||
    udev.state != ULEDS_STATE_REGISTERED);
    } while (retval == 0);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn uleds_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t uleds_poll(struct file *file, poll_table *wait)
    {
    struct uleds_device *udev = file.private_data;
    poll_wait(file, &udev.waitq, wait);
    if (udev.new_data)
    return EPOLLIN | EPOLLRDNORM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uleds_release(inode: *mut inode, file: *mut file) -> c_int {
    static int uleds_release(struct inode *inode, struct file *file)
    {
    struct uleds_device *udev = file.private_data;
    if (udev.state == ULEDS_STATE_REGISTERED) {
    udev.state = ULEDS_STATE_UNKNOWN;
    devm_led_classdev_unregister(uleds_misc.this_device,
    &udev.led_cdev);
    }
    kfree(udev);
    return 0;
    }
    static const struct file_operations uleds_fops = {
    .owner		= THIS_MODULE,
    .open		= uleds_open,
    .release	= uleds_release,
    .read		= uleds_read,
    .write		= uleds_write,
    .poll		= uleds_poll,
    };
    static struct miscdevice uleds_misc = {
    .fops		= &uleds_fops,
    .minor		= MISC_DYNAMIC_MINOR,
    .name		= ULEDS_NAME,
    };
    module_misc_device(uleds_misc);
    MODULE_AUTHOR("David Lechner <david@lechnology.com>");
    MODULE_DESCRIPTION("Userspace driver for the LED subsystem");
    MODULE_LICENSE("GPL");
