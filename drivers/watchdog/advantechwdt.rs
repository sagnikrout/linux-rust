//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/advantechwdt.c
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
// Advantech Single Board Computer WDT driver
//
// (c) Copyright 2000-2001 Marek Michalkiewicz <marekm@linux.org.pl>
//
// Based on acquirewdt.c which is based on wdt.c.
// Original copyright messages:
//
// (c) Copyright 1996 Alan Cox <alan@lxorguk.ukuu.org.uk>,
// All Rights Reserved.
//
// Neither Alan Cox nor CymruNet Ltd. admit liability nor provide
// warranty for any of this software. This material is provided
// "AS-IS" and at no charge.
//
// (c) Copyright 1995    Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// 14-Dec-2001 Matt Domsch <Matt_Domsch@dell.com>
// Added nowayout module option to override CONFIG_WATCHDOG_NOWAYOUT
//
// 16-Oct-2002 Rob Radez <rob@osinvestor.com>
// Clean up ioctls, clean up init + exit, add expect close support,
// add wdt_start and wdt_stop as parameters.
//

// the watchdog platform device
    static struct platform_device *advwdt_platform_device;
    static unsigned long advwdt_is_open;
    static char adv_expect_close;
//
// You must set these - there is no sane way to probe for this board.
//
// To enable or restart, write the timeout value in seconds (1 to 63)
// to I/O port wdt_start.  To disable, read I/O port wdt_stop.
// Both are 0x443 for most boards (tested on a PCA-6276VE-00B1), but
// check your manual (at least the PCA-6159 seems to be different -
// the manual says wdt_stop is 0x43, not 0x443).
// (0x43 is also a write-only control register for the 8254 timer!)
//
    let mut wdt_stop: static int = 0x443;
    module_param(wdt_stop, int, 0);
    MODULE_PARM_DESC(wdt_stop, "Advantech WDT 'stop' io port (default 0x443)");
    let mut wdt_start: static int = 0x443;
    module_param(wdt_start, int, 0);
    MODULE_PARM_DESC(wdt_start, "Advantech WDT 'start' io port (default 0x443)");
    static int timeout = WATCHDOG_TIMEOUT;	/* in seconds */
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. 1<= timeout <=63, default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ".");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
//
// Watchdog Operations
//
#[no_mangle]
unsafe extern "C" fn advwdt_ping() {
    static void advwdt_ping(void)
    {
// Write a watchdog value
    outb_p(timeout, wdt_start);
    }
#[no_mangle]
unsafe extern "C" fn advwdt_disable() {
    static void advwdt_disable(void)
    {
    inb_p(wdt_stop);
    }
#[no_mangle]
unsafe extern "C" fn advwdt_set_heartbeat(t: c_int) -> c_int {
    static int advwdt_set_heartbeat(int t)
    {
    if (t < 1 || t > 63)
    return -EINVAL;
    timeout = t;
    return 0;
    }
//
// /dev/watchdog handling
//
    static ssize_t advwdt_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    if (count) {
    if (!nowayout) {
    size_t i;
    adv_expect_close = 0;
    for (i = 0; i != count; i++) {
    char c;
    if (get_user(c, buf + i))
    return -EFAULT;
    if (c == 'V')
    adv_expect_close = 42;
    }
    }
    advwdt_ping();
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn advwdt_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long advwdt_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    int new_timeout;
    void __user *argp = (void __user *)arg;
    int __user *p = argp;
    static const struct watchdog_info ident = {
    .options = WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE,
    .firmware_version = 1,
    .identity = WATCHDOG_NAME,
    };
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    if (copy_to_user(argp, &ident, sizeof(ident)))
    return -EFAULT;
    break;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    return put_user(0, p);
    case WDIOC_SETOPTIONS:
    {
    int options, retval = -EINVAL;
    if (get_user(options, p))
    return -EFAULT;
    if (options & WDIOS_DISABLECARD) {
    advwdt_disable();
    retval = 0;
    }
    if (options & WDIOS_ENABLECARD) {
    advwdt_ping();
    retval = 0;
    }
    return retval;
    }
    case WDIOC_KEEPALIVE:
    advwdt_ping();
    break;
    case WDIOC_SETTIMEOUT:
    if (get_user(new_timeout, p))
    return -EFAULT;
    if (advwdt_set_heartbeat(new_timeout))
    return -EINVAL;
    advwdt_ping();
    fallthrough;
    case WDIOC_GETTIMEOUT:
    return put_user(timeout, p);
    default:
    return -ENOTTY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn advwdt_open(inode: *mut inode, file: *mut file) -> c_int {
    static int advwdt_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(0, &advwdt_is_open))
    return -EBUSY;
//
// Activate
//
    advwdt_ping();
    return stream_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn advwdt_close(inode: *mut inode, file: *mut file) -> c_int {
    static int advwdt_close(struct inode *inode, struct file *file)
    {
    if (adv_expect_close == 42) {
    advwdt_disable();
    } else {
    pr_crit("Unexpected close, not stopping watchdog!\n");
    advwdt_ping();
    }
    clear_bit(0, &advwdt_is_open);
    adv_expect_close = 0;
    return 0;
    }
//
// Kernel Interfaces
//
    static const struct file_operations advwdt_fops = {
    .owner		= THIS_MODULE,
    .write		= advwdt_write,
    .unlocked_ioctl	= advwdt_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= advwdt_open,
    .release	= advwdt_close,
    };
    static struct miscdevice advwdt_miscdev = {
    .minor	= WATCHDOG_MINOR,
    .name	= "watchdog",
    .fops	= &advwdt_fops,
    };
//
// Init & exit routines
//
#[no_mangle]
unsafe extern "C" fn advwdt_probe(dev: *mut platform_device) -> int __init {
    static int __init advwdt_probe(struct platform_device *dev)
    {
    int ret;
    if (wdt_stop != wdt_start) {
    if (!request_region(wdt_stop, 1, WATCHDOG_NAME)) {
    pr_err("I/O address 0x%04x already in use\n",
    wdt_stop);
    ret = -EIO;
    goto out;
    }
    }
    if (!request_region(wdt_start, 1, WATCHDOG_NAME)) {
    pr_err("I/O address 0x%04x already in use\n", wdt_start);
    ret = -EIO;
    goto unreg_stop;
    }
// Check that the heartbeat value is within it's range ;
// if not reset to the default
    if (advwdt_set_heartbeat(timeout)) {
    advwdt_set_heartbeat(WATCHDOG_TIMEOUT);
    pr_info("timeout value must be 1<=x<=63, using %d\n", timeout);
    }
    ret = misc_register(&advwdt_miscdev);
    if (ret != 0) {
    pr_err("cannot register miscdev on minor=%d (err=%d)\n",
    WATCHDOG_MINOR, ret);
    goto unreg_regions;
    }
    pr_info("initialized. timeout=%d sec (nowayout=%d)\n",
    timeout, nowayout);
    out:
    return ret;
    unreg_regions:
    release_region(wdt_start, 1);
    unreg_stop:
    if (wdt_stop != wdt_start)
    release_region(wdt_stop, 1);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn advwdt_remove(dev: *mut platform_device) {
    static void advwdt_remove(struct platform_device *dev)
    {
    misc_deregister(&advwdt_miscdev);
    release_region(wdt_start, 1);
    if (wdt_stop != wdt_start)
    release_region(wdt_stop, 1);
    }
#[no_mangle]
unsafe extern "C" fn advwdt_shutdown(dev: *mut platform_device) {
    static void advwdt_shutdown(struct platform_device *dev)
    {
// Turn the WDT off if we have a soft shutdown
    advwdt_disable();
    }
    static struct platform_driver advwdt_driver = {
    .remove		= advwdt_remove,
    .shutdown	= advwdt_shutdown,
    .driver		= {
    .name	= DRV_NAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn advwdt_init() -> int __init {
    static int __init advwdt_init(void)
    {
    int err;
    pr_info("WDT driver for Advantech single board computer initialising\n");
    advwdt_platform_device = platform_device_register_simple(DRV_NAME,
    -1, core::ptr::null_mut(), 0);
    if (IS_ERR(advwdt_platform_device))
    return PTR_ERR(advwdt_platform_device);
    err = platform_driver_probe(&advwdt_driver, advwdt_probe);
    if (err)
    goto unreg_platform_device;
    return 0;
    unreg_platform_device:
    platform_device_unregister(advwdt_platform_device);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn advwdt_exit() -> void __exit {
    static void __exit advwdt_exit(void)
    {
    platform_device_unregister(advwdt_platform_device);
    platform_driver_unregister(&advwdt_driver);
    pr_info("Watchdog Module Unloaded\n");
    }
    module_init(advwdt_init);
    module_exit(advwdt_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marek Michalkiewicz <marekm@linux.org.pl>");
    MODULE_DESCRIPTION("Advantech Single Board Computer WDT driver");
