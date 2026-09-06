//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/indydog.c
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
// IndyDog	0.3	A Hardware Watchdog Device for SGI IP22
//
// (c) Copyright 2002 Guido Guenther <agx@sigxcpu.org>,
// All Rights Reserved.
//
// based on softdog.c by Alan Cox <alan@lxorguk.ukuu.org.uk>
//

    static unsigned long indydog_alive;
    static DEFINE_SPINLOCK(indydog_lock);

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn indydog_start() {
    static void indydog_start(void)
    {
    spin_lock(&indydog_lock);
    sgimc.cpuctrl0 |= SGIMC_CCTRL0_WDOG;
    spin_unlock(&indydog_lock);
    }
#[no_mangle]
unsafe extern "C" fn indydog_stop() {
    static void indydog_stop(void)
    {
    spin_lock(&indydog_lock);
    sgimc.cpuctrl0 &= ~SGIMC_CCTRL0_WDOG;
    spin_unlock(&indydog_lock);
    pr_info("Stopped watchdog timer\n");
    }
#[no_mangle]
unsafe extern "C" fn indydog_ping() {
    static void indydog_ping(void)
    {
    sgimc.watchdogt = 0;
    }
//
// Allow only one person to hold it open
//
#[no_mangle]
unsafe extern "C" fn indydog_open(inode: *mut inode, file: *mut file) -> c_int {
    static int indydog_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(0, &indydog_alive))
    return -EBUSY;
    if (nowayout)
    __module_get(THIS_MODULE);
// Activate timer
    indydog_start();
    indydog_ping();
    pr_info("Started watchdog timer\n");
    return stream_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn indydog_release(inode: *mut inode, file: *mut file) -> c_int {
    static int indydog_release(struct inode *inode, struct file *file)
    {
// Shut off the timer.
// Lock it in if it's a module and we defined ...NOWAYOUT
    if (!nowayout)
    indydog_stop();		/* Turn the WDT off */
    clear_bit(0, &indydog_alive);
    return 0;
    }
    static ssize_t indydog_write(struct file *file, const char *data,
    size_t len, loff_t *ppos)
    {
// Refresh the timer.
    if (len)
    indydog_ping();
    return len;
    }
    static long indydog_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    int options, retval = -EINVAL;
    static const struct watchdog_info ident = {
    .options		= WDIOF_KEEPALIVEPING,
    .firmware_version	= 0,
    .identity		= "Hardware Watchdog for SGI IP22",
    };
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    if (copy_to_user((struct watchdog_info *)arg,
    &ident, sizeof(ident)))
    return -EFAULT;
    return 0;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    return put_user(0, (int *)arg);
    case WDIOC_SETOPTIONS:
    {
    if (get_user(options, (int *)arg))
    return -EFAULT;
    if (options & WDIOS_DISABLECARD) {
    indydog_stop();
    retval = 0;
    }
    if (options & WDIOS_ENABLECARD) {
    indydog_start();
    retval = 0;
    }
    return retval;
    }
    case WDIOC_KEEPALIVE:
    indydog_ping();
    return 0;
    case WDIOC_GETTIMEOUT:
    return put_user(WATCHDOG_TIMEOUT, (int *)arg);
    default:
    return -ENOTTY;
    }
    }
    static int indydog_notify_sys(struct notifier_block *this,
    unsigned long code, void *unused)
    {
    if (code == SYS_DOWN || code == SYS_HALT)
    indydog_stop();		/* Turn the WDT off */
    return NOTIFY_DONE;
    }
    static const struct file_operations indydog_fops = {
    .owner		= THIS_MODULE,
    .write		= indydog_write,
    .unlocked_ioctl	= indydog_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= indydog_open,
    .release	= indydog_release,
    };
    static struct miscdevice indydog_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &indydog_fops,
    };
    static struct notifier_block indydog_notifier = {
    .notifier_call = indydog_notify_sys,
    };
#[no_mangle]
unsafe extern "C" fn watchdog_init() -> int __init {
    static int __init watchdog_init(void)
    {
    int ret;
    ret = register_reboot_notifier(&indydog_notifier);
    if (ret) {
    pr_err("cannot register reboot notifier (err=%d)\n", ret);
    return ret;
    }
    ret = misc_register(&indydog_miscdev);
    if (ret) {
    pr_err("cannot register miscdev on minor=%d (err=%d)\n",
    WATCHDOG_MINOR, ret);
    unregister_reboot_notifier(&indydog_notifier);
    return ret;
    }
    pr_info("Hardware Watchdog Timer for SGI IP22: 0.3\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_exit() -> void __exit {
    static void __exit watchdog_exit(void)
    {
    misc_deregister(&indydog_miscdev);
    unregister_reboot_notifier(&indydog_notifier);
    }
    module_init(watchdog_init);
    module_exit(watchdog_exit);
    MODULE_AUTHOR("Guido Guenther <agx@sigxcpu.org>");
    MODULE_DESCRIPTION("Hardware Watchdog Device for SGI IP22");
    MODULE_LICENSE("GPL");
