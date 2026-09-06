//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ath79_wdt.c
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
// Atheros AR71XX/AR724X/AR913X built-in hardware watchdog timer.
//
// Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
// Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
//
// This driver was based on: drivers/watchdog/ixp4xx_wdt.c
// Author: Deepak Saxena <dsaxena@plexity.net>
// Copyright 2004 (c) MontaVista, Software, Inc.
//
// which again was based on sa1100 driver,
// Copyright (C) 2000 Oleg Drokin <green@crimea.edu>
//

pub const WDOG_REG_CTRL: c_uint = 0x00;
pub const WDOG_REG_TIMER: c_uint = 0x04;

pub const WDOG_CTRL_ACTION_MASK: c_int = 3;

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started "
    "(default=" __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    let mut timeout: static int = WDT_TIMEOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds "
    "(default=" __MODULE_STRING(WDT_TIMEOUT) "s)");
    static unsigned long wdt_flags;
pub const WDT_FLAGS_BUSY: c_int = 0;
pub const WDT_FLAGS_EXPECT_CLOSE: c_int = 1;
    static struct clk *wdt_clk;
    static unsigned long wdt_freq;
    static int boot_status;
    static int max_timeout;
    static void __iomem *wdt_base;
#[no_mangle]
pub unsafe extern "C" fn ath79_wdt_wr(reg: unsigned, val: u32) {
    static inline void ath79_wdt_wr(unsigned reg, u32 val)
    {
    iowrite32(val, wdt_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn ath79_wdt_rr(reg: unsigned) -> u32 {
    static inline u32 ath79_wdt_rr(unsigned reg)
    {
    return ioread32(wdt_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn ath79_wdt_keepalive() {
    static inline void ath79_wdt_keepalive(void)
    {
    ath79_wdt_wr(WDOG_REG_TIMER, wdt_freq * timeout);
// flush write
    ath79_wdt_rr(WDOG_REG_TIMER);
    }
#[no_mangle]
pub unsafe extern "C" fn ath79_wdt_enable() {
    static inline void ath79_wdt_enable(void)
    {
    ath79_wdt_keepalive();
//
// Updating the TIMER register requires a few microseconds
// on the AR934x SoCs at least. Use a small delay to ensure
// that the TIMER register is updated within the hardware
// before enabling the watchdog.
//
    udelay(2);
    ath79_wdt_wr(WDOG_REG_CTRL, WDOG_CTRL_ACTION_FCR);
// flush write
    ath79_wdt_rr(WDOG_REG_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn ath79_wdt_disable() {
    static inline void ath79_wdt_disable(void)
    {
    ath79_wdt_wr(WDOG_REG_CTRL, WDOG_CTRL_ACTION_NONE);
// flush write
    ath79_wdt_rr(WDOG_REG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn ath79_wdt_set_timeout(val: c_int) -> c_int {
    static int ath79_wdt_set_timeout(int val)
    {
    if (val < 1 || val > max_timeout)
    return -EINVAL;
    timeout = val;
    ath79_wdt_keepalive();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ath79_wdt_open(inode: *mut inode, file: *mut file) -> c_int {
    static int ath79_wdt_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(WDT_FLAGS_BUSY, &wdt_flags))
    return -EBUSY;
    clear_bit(WDT_FLAGS_EXPECT_CLOSE, &wdt_flags);
    ath79_wdt_enable();
    return stream_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn ath79_wdt_release(inode: *mut inode, file: *mut file) -> c_int {
    static int ath79_wdt_release(struct inode *inode, struct file *file)
    {
    if (test_bit(WDT_FLAGS_EXPECT_CLOSE, &wdt_flags))
    ath79_wdt_disable();
    else {
    pr_crit("device closed unexpectedly, watchdog timer will not stop!\n");
    ath79_wdt_keepalive();
    }
    clear_bit(WDT_FLAGS_BUSY, &wdt_flags);
    clear_bit(WDT_FLAGS_EXPECT_CLOSE, &wdt_flags);
    return 0;
    }
    static ssize_t ath79_wdt_write(struct file *file, const char *data,
    size_t len, loff_t *ppos)
    {
    if (len) {
    if (!nowayout) {
    size_t i;
    clear_bit(WDT_FLAGS_EXPECT_CLOSE, &wdt_flags);
    for (i = 0; i != len; i++) {
    char c;
    if (get_user(c, data + i))
    return -EFAULT;
    if (c == 'V')
    set_bit(WDT_FLAGS_EXPECT_CLOSE,
    &wdt_flags);
    }
    }
    ath79_wdt_keepalive();
    }
    return len;
    }
    static const struct watchdog_info ath79_wdt_info = {
    .options		= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE | WDIOF_CARDRESET,
    .firmware_version	= 0,
    .identity		= "ATH79 watchdog",
    };
    static long ath79_wdt_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    int __user *p = argp;
    int err;
    int t;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    err = copy_to_user(argp, &ath79_wdt_info,
    sizeof(ath79_wdt_info)) ? -EFAULT : 0;
    break;
    case WDIOC_GETSTATUS:
    err = put_user(0, p);
    break;
    case WDIOC_GETBOOTSTATUS:
    err = put_user(boot_status, p);
    break;
    case WDIOC_KEEPALIVE:
    ath79_wdt_keepalive();
    err = 0;
    break;
    case WDIOC_SETTIMEOUT:
    err = get_user(t, p);
    if (err)
    break;
    err = ath79_wdt_set_timeout(t);
    if (err)
    break;
    fallthrough;
    case WDIOC_GETTIMEOUT:
    err = put_user(timeout, p);
    break;
    default:
    err = -ENOTTY;
    break;
    }
    return err;
    }
    static const struct file_operations ath79_wdt_fops = {
    .owner		= THIS_MODULE,
    .write		= ath79_wdt_write,
    .unlocked_ioctl	= ath79_wdt_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= ath79_wdt_open,
    .release	= ath79_wdt_release,
    };
    static struct miscdevice ath79_wdt_miscdev = {
    .minor = WATCHDOG_MINOR,
    .name = "watchdog",
    .fops = &ath79_wdt_fops,
    };
#[no_mangle]
unsafe extern "C" fn ath79_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ath79_wdt_probe(struct platform_device *pdev)
    {
    u32 ctrl;
    int err;
    if (wdt_base)
    return -EBUSY;
    wdt_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt_base))
    return PTR_ERR(wdt_base);
    wdt_clk = devm_clk_get_enabled(&pdev.dev, "wdt");
    if (IS_ERR(wdt_clk))
    return PTR_ERR(wdt_clk);
    wdt_freq = clk_get_rate(wdt_clk);
    if (!wdt_freq)
    return -EINVAL;
    max_timeout = (0xfffffffful / wdt_freq);
    if (timeout < 1 || timeout > max_timeout) {
    timeout = max_timeout;
    dev_info(&pdev.dev,
    "timeout value must be 0 < timeout < %d, using %d\n",
    max_timeout, timeout);
    }
    ctrl = ath79_wdt_rr(WDOG_REG_CTRL);
    boot_status = (ctrl & WDOG_CTRL_LAST_RESET) ? WDIOF_CARDRESET : 0;
    err = misc_register(&ath79_wdt_miscdev);
    if (err) {
    dev_err(&pdev.dev,
    "unable to register misc device, err=%d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ath79_wdt_remove(pdev: *mut platform_device) {
    static void ath79_wdt_remove(struct platform_device *pdev)
    {
    misc_deregister(&ath79_wdt_miscdev);
    }
#[no_mangle]
unsafe extern "C" fn ath79_wdt_shutdown(pdev: *mut platform_device) {
    static void ath79_wdt_shutdown(struct platform_device *pdev)
    {
    ath79_wdt_disable();
    }

    static const struct of_device_id ath79_wdt_match[] = {
    { .compatible = "qca,ar7130-wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ath79_wdt_match);

    static struct platform_driver ath79_wdt_driver = {
    .probe		= ath79_wdt_probe,
    .remove		= ath79_wdt_remove,
    .shutdown	= ath79_wdt_shutdown,
    .driver		= {
    .name	= DRIVER_NAME,
    .of_match_table = of_match_ptr(ath79_wdt_match),
    },
    };
    module_platform_driver(ath79_wdt_driver);
    MODULE_DESCRIPTION("Atheros AR71XX/AR724X/AR913X hardware watchdog driver");
    MODULE_AUTHOR("Gabor Juhos <juhosg@openwrt.org");
    MODULE_AUTHOR("Imre Kaloz <kaloz@openwrt.org");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
