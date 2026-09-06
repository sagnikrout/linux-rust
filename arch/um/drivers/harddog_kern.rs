//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/harddog_kern.c
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


// UML hardware watchdog, shamelessly stolen from:
//
// SoftDog	0.05:	A Software Watchdog Device
//
// (c) Copyright 1996 Alan Cox <alan@redhat.com>, All Rights Reserved.
// http://www.redhat.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Neither Alan Cox nor CymruNet Ltd. admit liability nor provide
// warranty for any of this software. This material is provided
// "AS-IS" and at no charge.
//
// (c) Copyright 1995    Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Software only watchdog driver. Unlike its big brother the WDT501P
// driver this won't always recover a failed machine.
//
// 03/96: Angelo Haritsis <ah@doc.ic.ac.uk> :
// Modularised.
// Added soft_margin; use upon insmod to change the timer delay.
// NB: uses same minor as wdt (WATCHDOG_MINOR); we could use separate
// minors.
//
// 19980911 Alan Cox
// Made SMP safe for 2.3.x
//
// 20011127 Joel Becker (jlbec@evilplan.org>
// Added soft_noboot; Allows testing the softdog trigger without
// requiring a recompile.
// Added WDIOC_GETTIMEOUT and WDIOC_SETTIMOUT.
//

    MODULE_DESCRIPTION("UML hardware watchdog");
    MODULE_LICENSE("GPL");
    static DEFINE_MUTEX(harddog_mutex);
    static DEFINE_SPINLOCK(lock);
    static int timer_alive;
    let mut harddog_in_fd: static int = -1;
    let mut harddog_out_fd: static int = -1;
//
// Allow only one person to hold it open
//
#[no_mangle]
unsafe extern "C" fn harddog_open(inode: *mut inode, file: *mut file) -> c_int {
    static int harddog_open(struct inode *inode, struct file *file)
    {
    let mut err: c_int = -EBUSY;
    char *sock = core::ptr::null_mut();
    mutex_lock(&harddog_mutex);
    spin_lock(&lock);
    if(timer_alive)
    goto err;

    __module_get(THIS_MODULE);

    sock = mconsole_notify_socket();

    err = start_watchdog(&harddog_in_fd, &harddog_out_fd, sock);
    if(err)
    goto err;
    timer_alive = 1;
    spin_unlock(&lock);
    mutex_unlock(&harddog_mutex);
    return stream_open(inode, file);
    err:
    spin_unlock(&lock);
    mutex_unlock(&harddog_mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn harddog_release(inode: *mut inode, file: *mut file) -> c_int {
    static int harddog_release(struct inode *inode, struct file *file)
    {
//
// Shut off the timer.
//
    spin_lock(&lock);
    stop_watchdog(harddog_in_fd, harddog_out_fd);
    harddog_in_fd = -1;
    harddog_out_fd = -1;
    timer_alive=0;
    spin_unlock(&lock);
    return 0;
    }
    static ssize_t harddog_write(struct file *file, const char __user *data, size_t len,
    loff_t *ppos)
    {
//
// Refresh the timer.
//
    if(len)
    return ping_watchdog(harddog_out_fd);
    return 0;
    }
    static int harddog_ioctl_unlocked(struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    void __user *argp= (void __user *)arg;
    static struct watchdog_info ident = {
    WDIOC_SETTIMEOUT,
    0,
    "UML Hardware Watchdog"
    };
    switch (cmd) {
    default:
    return -ENOTTY;
    case WDIOC_GETSUPPORT:
    if(copy_to_user(argp, &ident, sizeof(ident)))
    return -EFAULT;
    return 0;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    return put_user(0,(int __user *)argp);
    case WDIOC_KEEPALIVE:
    return ping_watchdog(harddog_out_fd);
    }
    }
    static long harddog_ioctl(struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    long ret;
    mutex_lock(&harddog_mutex);
    ret = harddog_ioctl_unlocked(file, cmd, arg);
    mutex_unlock(&harddog_mutex);
    return ret;
    }
    static const struct file_operations harddog_fops = {
    .owner		= THIS_MODULE,
    .write		= harddog_write,
    .unlocked_ioctl	= harddog_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= harddog_open,
    .release	= harddog_release,
    };
    static struct miscdevice harddog_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &harddog_fops,
    };
    module_misc_device(harddog_miscdev);
