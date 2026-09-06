//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sbc_fitpc2_wdt.c
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
// Watchdog driver for SBC-FITPC2 board
//
// Author: Denis Turischev <denis@compulab.co.il>
//
// Adapted from the IXP2000 watchdog driver by Deepak Saxena.
//

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    static unsigned int margin = 60;	/* (secs) Default is 1 minute */
    static unsigned long wdt_status;
    static DEFINE_MUTEX(wdt_lock);
pub const WDT_IN_USE: c_int = 0;
pub const WDT_OK_TO_CLOSE: c_int = 1;
pub const COMMAND_PORT: c_uint = 0x4c;
pub const DATA_PORT: c_uint = 0x48;
pub const IFACE_ON_COMMAND: c_int = 1;
pub const REBOOT_COMMAND: c_int = 2;

#[no_mangle]
unsafe extern "C" fn wdt_send_data(command: c_uchar, data: c_uchar) {
    static void wdt_send_data(unsigned char command, unsigned char data)
    {
    outb(data, DATA_PORT);
    msleep(200);
    outb(command, COMMAND_PORT);
    msleep(100);
    }
#[no_mangle]
unsafe extern "C" fn wdt_enable() {
    static void wdt_enable(void)
    {
    mutex_lock(&wdt_lock);
    wdt_send_data(IFACE_ON_COMMAND, 1);
    wdt_send_data(REBOOT_COMMAND, margin);
    mutex_unlock(&wdt_lock);
    }
#[no_mangle]
unsafe extern "C" fn wdt_disable() {
    static void wdt_disable(void)
    {
    mutex_lock(&wdt_lock);
    wdt_send_data(IFACE_ON_COMMAND, 0);
    wdt_send_data(REBOOT_COMMAND, 0);
    mutex_unlock(&wdt_lock);
    }
#[no_mangle]
unsafe extern "C" fn fitpc2_wdt_open(inode: *mut inode, file: *mut file) -> c_int {
    static int fitpc2_wdt_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(WDT_IN_USE, &wdt_status))
    return -EBUSY;
    clear_bit(WDT_OK_TO_CLOSE, &wdt_status);
    wdt_enable();
    return stream_open(inode, file);
    }
    static ssize_t fitpc2_wdt_write(struct file *file, const char __user *data,
    size_t len, loff_t *ppos)
    {
    size_t i;
    if (!len)
    return 0;
    if (nowayout) {
    len = 0;
    goto out;
    }
    clear_bit(WDT_OK_TO_CLOSE, &wdt_status);
    for (i = 0; i != len; i++) {
    char c;
    if (get_user(c, data + i))
    return -EFAULT;
    if (c == 'V')
    set_bit(WDT_OK_TO_CLOSE, &wdt_status);
    }
    out:
    wdt_enable();
    return len;
    }
    static const struct watchdog_info ident = {
    .options	= WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT |
    WDIOF_KEEPALIVEPING,
    .identity	= WATCHDOG_NAME,
    };
    static long fitpc2_wdt_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    let mut ret: c_int = -ENOTTY;
    int time;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    ret = copy_to_user((struct watchdog_info __user *)arg, &ident,
    sizeof(ident)) ? -EFAULT : 0;
    break;
    case WDIOC_GETSTATUS:
    ret = put_user(0, (int __user *)arg);
    break;
    case WDIOC_GETBOOTSTATUS:
    ret = put_user(0, (int __user *)arg);
    break;
    case WDIOC_KEEPALIVE:
    wdt_enable();
    ret = 0;
    break;
    case WDIOC_SETTIMEOUT:
    ret = get_user(time, (int __user *)arg);
    if (ret)
    break;
    if (time < 31 || time > 255) {
    ret = -EINVAL;
    break;
    }
    margin = time;
    wdt_enable();
    fallthrough;
    case WDIOC_GETTIMEOUT:
    ret = put_user(margin, (int __user *)arg);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fitpc2_wdt_release(inode: *mut inode, file: *mut file) -> c_int {
    static int fitpc2_wdt_release(struct inode *inode, struct file *file)
    {
    if (test_bit(WDT_OK_TO_CLOSE, &wdt_status)) {
    wdt_disable();
    pr_info("Device disabled\n");
    } else {
    pr_warn("Device closed unexpectedly - timer will not stop\n");
    wdt_enable();
    }
    clear_bit(WDT_IN_USE, &wdt_status);
    clear_bit(WDT_OK_TO_CLOSE, &wdt_status);
    return 0;
    }
    static const struct file_operations fitpc2_wdt_fops = {
    .owner		= THIS_MODULE,
    .write		= fitpc2_wdt_write,
    .unlocked_ioctl	= fitpc2_wdt_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= fitpc2_wdt_open,
    .release	= fitpc2_wdt_release,
    };
    static struct miscdevice fitpc2_wdt_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &fitpc2_wdt_fops,
    };
#[no_mangle]
unsafe extern "C" fn fitpc2_wdt_init() -> int __init {
    static int __init fitpc2_wdt_init(void)
    {
    int err;
    const char *brd_name;
    brd_name = dmi_get_system_info(DMI_BOARD_NAME);
    if (!brd_name || !strstr(brd_name, "SBC-FITPC2"))
    return -ENODEV;
    pr_info("%s found\n", brd_name);
    if (!request_region(COMMAND_PORT, 1, WATCHDOG_NAME)) {
    pr_err("I/O address 0x%04x already in use\n", COMMAND_PORT);
    return -EIO;
    }
    if (!request_region(DATA_PORT, 1, WATCHDOG_NAME)) {
    pr_err("I/O address 0x%04x already in use\n", DATA_PORT);
    err = -EIO;
    goto err_data_port;
    }
    if (margin < 31 || margin > 255) {
    pr_err("margin must be in range 31 - 255 seconds, you tried to set %d\n",
    margin);
    err = -EINVAL;
    goto err_margin;
    }
    err = misc_register(&fitpc2_wdt_miscdev);
    if (err) {
    pr_err("cannot register miscdev on minor=%d (err=%d)\n",
    WATCHDOG_MINOR, err);
    goto err_margin;
    }
    return 0;
    err_margin:
    release_region(DATA_PORT, 1);
    err_data_port:
    release_region(COMMAND_PORT, 1);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fitpc2_wdt_exit() -> void __exit {
    static void __exit fitpc2_wdt_exit(void)
    {
    misc_deregister(&fitpc2_wdt_miscdev);
    release_region(DATA_PORT, 1);
    release_region(COMMAND_PORT, 1);
    }
    module_init(fitpc2_wdt_init);
    module_exit(fitpc2_wdt_exit);
    MODULE_AUTHOR("Denis Turischev <denis@compulab.co.il>");
    MODULE_DESCRIPTION("SBC-FITPC2 Watchdog");
    module_param(margin, int, 0);
    MODULE_PARM_DESC(margin, "Watchdog margin in seconds (default 60s)");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started");
    MODULE_LICENSE("GPL");
