//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/wdt285.c
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
// Intel 21285 watchdog driver
// Copyright (c) Phil Blundell <pb@nexus.co.uk>, 1998
//
// based on
//
// SoftDog	0.05:	A Software Watchdog Device
//
// (c) Copyright 1996 Alan Cox <alan@lxorguk.ukuu.org.uk>,
// All Rights Reserved.
//

//
// Define this to stop the watchdog actually rebooting the machine.
//

    static unsigned int soft_margin = 60;		/* in seconds */
    static unsigned int reload;
    static unsigned long timer_alive;

//
// If the timer expires..
//
#[no_mangle]
unsafe extern "C" fn watchdog_fire(irq: c_int, dev_id: *mut c_void) {
    static void watchdog_fire(int irq, void *dev_id)
    {
    pr_crit("Would Reboot\n");
// CSR_TIMER4_CNTL = 0;
// CSR_TIMER4_CLR = 0;
    }

//
// Refresh the timer.
//
#[no_mangle]
unsafe extern "C" fn watchdog_ping() {
    static void watchdog_ping(void)
    {
// CSR_TIMER4_LOAD = reload;
    }
//
// Allow only one person to hold it open
//
#[no_mangle]
unsafe extern "C" fn watchdog_open(inode: *mut inode, file: *mut file) -> c_int {
    static int watchdog_open(struct inode *inode, struct file *file)
    {
    int ret;
    if (*CSR_SA110_CNTL & (1 << 13))
    return -EBUSY;
    if (test_and_set_bit(1, &timer_alive))
    return -EBUSY;
    reload = soft_margin * (mem_fclk_21285 / 256);
// CSR_TIMER4_CLR = 0;
    watchdog_ping();
// CSR_TIMER4_CNTL = TIMER_CNTL_ENABLE | TIMER_CNTL_AUTORELOAD
    | TIMER_CNTL_DIV256;

    ret = request_irq(IRQ_TIMER4, watchdog_fire, 0, "watchdog", core::ptr::null_mut());
    if (ret) {
// CSR_TIMER4_CNTL = 0;
    clear_bit(1, &timer_alive);
    }

//
// Setting this bit is irreversible; once enabled, there is
// no way to disable the watchdog.
//
// CSR_SA110_CNTL |= 1 << 13;
    ret = 0;

    stream_open(inode, file);
    return ret;
    }
//
// Shut off the timer.
// Note: if we really have enabled the watchdog, there
// is no way to turn off.
//
#[no_mangle]
unsafe extern "C" fn watchdog_release(inode: *mut inode, file: *mut file) -> c_int {
    static int watchdog_release(struct inode *inode, struct file *file)
    {

    free_irq(IRQ_TIMER4, core::ptr::null_mut());
    clear_bit(1, &timer_alive);

    return 0;
    }
    static ssize_t watchdog_write(struct file *file, const char __user *data,
    size_t len, loff_t *ppos)
    {
//
// Refresh the timer.
//
    if (len)
    watchdog_ping();
    return len;
    }
    static const struct watchdog_info ident = {
    .options	= WDIOF_SETTIMEOUT,
    .identity	= "Footbridge Watchdog",
    };
    static long watchdog_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    int __user *int_arg = (int __user *)arg;
    int new_margin, ret = -ENOTTY;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    ret = 0;
    if (copy_to_user((void __user *)arg, &ident, sizeof(ident)))
    ret = -EFAULT;
    break;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    ret = put_user(0, int_arg);
    break;
    case WDIOC_KEEPALIVE:
    watchdog_ping();
    ret = 0;
    break;
    case WDIOC_SETTIMEOUT:
    ret = get_user(new_margin, int_arg);
    if (ret)
    break;
// Arbitrary, can't find the card's limits
    if (new_margin < 0 || new_margin > 60) {
    ret = -EINVAL;
    break;
    }
    soft_margin = new_margin;
    reload = soft_margin * (mem_fclk_21285 / 256);
    watchdog_ping();
    fallthrough;
    case WDIOC_GETTIMEOUT:
    ret = put_user(soft_margin, int_arg);
    break;
    }
    return ret;
    }
    static const struct file_operations watchdog_fops = {
    .owner		= THIS_MODULE,
    .write		= watchdog_write,
    .unlocked_ioctl	= watchdog_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= watchdog_open,
    .release	= watchdog_release,
    };
    static struct miscdevice watchdog_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &watchdog_fops,
    };
#[no_mangle]
unsafe extern "C" fn footbridge_watchdog_init() -> int __init {
    static int __init footbridge_watchdog_init(void)
    {
    int retval;
    if (machine_is_netwinder())
    return -ENODEV;
    retval = misc_register(&watchdog_miscdev);
    if (retval < 0)
    return retval;
    pr_info("Footbridge Watchdog Timer: 0.01, timer margin: %d sec\n",
    soft_margin);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn footbridge_watchdog_exit() -> void __exit {
    static void __exit footbridge_watchdog_exit(void)
    {
    misc_deregister(&watchdog_miscdev);
    }
    MODULE_AUTHOR("Phil Blundell <pb@nexus.co.uk>");
    MODULE_DESCRIPTION("Footbridge watchdog driver");
    MODULE_LICENSE("GPL");
    module_param(soft_margin, int, 0);
    MODULE_PARM_DESC(soft_margin, "Watchdog timeout in seconds");
    module_init(footbridge_watchdog_init);
    module_exit(footbridge_watchdog_exit);
