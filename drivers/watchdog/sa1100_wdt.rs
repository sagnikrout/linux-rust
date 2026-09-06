//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sa1100_wdt.c
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
// Watchdog driver for the SA11x0/PXA2xx
//
// (c) Copyright 2000 Oleg Drokin <green@crimea.edu>
// Based on SoftDog driver by Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Neither Oleg Drokin nor iXcelerator.com admit liability nor provide
// warranty for any of this software. This material is provided
// "AS-IS" and at no charge.
//
// (c) Copyright 2000           Oleg Drokin <green@crimea.edu>
//
// 27/11/2000 Initial release
//

pub const REG_OSMR0: c_uint = 0x0000  /* OS timer Match Reg. 0 */;
pub const REG_OSMR1: c_uint = 0x0004  /* OS timer Match Reg. 1 */;
pub const REG_OSMR2: c_uint = 0x0008  /* OS timer Match Reg. 2 */;
pub const REG_OSMR3: c_uint = 0x000c  /* OS timer Match Reg. 3 */;
pub const REG_OSCR: c_uint = 0x0010  /* OS timer Counter Reg. */;
pub const REG_OSSR: c_uint = 0x0014  /* OS timer Status Reg. */;
pub const REG_OWER: c_uint = 0x0018  /* OS timer Watch-dog Enable Reg. */;
pub const REG_OIER: c_uint = 0x001C  /* OS timer Interrupt Enable Reg. */;

    static unsigned long oscr_freq;
    static unsigned long sa1100wdt_users;
    static unsigned int pre_margin;
    static int boot_status;
    static void __iomem *reg_base;
#[no_mangle]
pub unsafe extern "C" fn sa1100_wr(val: u32, offset: u32) {
    static inline void sa1100_wr(u32 val, u32 offset)
    {
    writel_relaxed(val, reg_base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn sa1100_rd(offset: u32) -> u32 {
    static inline u32 sa1100_rd(u32 offset)
    {
    return readl_relaxed(reg_base + offset);
    }
//
// Allow only one person to hold it open
//
#[no_mangle]
unsafe extern "C" fn sa1100dog_open(inode: *mut inode, file: *mut file) -> c_int {
    static int sa1100dog_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(1, &sa1100wdt_users))
    return -EBUSY;
// Activate SA1100 Watchdog timer
    sa1100_wr(sa1100_rd(REG_OSCR) + pre_margin, REG_OSMR3);
    sa1100_wr(OSSR_M3, REG_OSSR);
    sa1100_wr(OWER_WME, REG_OWER);
    sa1100_wr(sa1100_rd(REG_OIER) | OIER_E3, REG_OIER);
    return stream_open(inode, file);
    }
//
// The watchdog cannot be disabled.
//
// Previous comments suggested that turning off the interrupt by
// clearing REG_OIER[E3] would prevent the watchdog timing out but this
// does not appear to be true (at least on the PXA255).
//
#[no_mangle]
unsafe extern "C" fn sa1100dog_release(inode: *mut inode, file: *mut file) -> c_int {
    static int sa1100dog_release(struct inode *inode, struct file *file)
    {
    pr_crit("Device closed - timer will not stop\n");
    clear_bit(1, &sa1100wdt_users);
    return 0;
    }
    static ssize_t sa1100dog_write(struct file *file, const char __user *data,
    size_t len, loff_t *ppos)
    {
    if (len)
// Refresh OSMR3 timer.
    sa1100_wr(sa1100_rd(REG_OSCR) + pre_margin, REG_OSMR3);
    return len;
    }
    static const struct watchdog_info ident = {
    .options	= WDIOF_CARDRESET | WDIOF_SETTIMEOUT
    | WDIOF_KEEPALIVEPING,
    .identity	= "SA1100/PXA255 Watchdog",
    .firmware_version	= 1,
    };
    static long sa1100dog_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    let mut ret: c_int = -ENOTTY;
    int time;
    void __user *argp = (void __user *)arg;
    int __user *p = argp;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    ret = copy_to_user(argp, &ident,
    sizeof(ident)) ? -EFAULT : 0;
    break;
    case WDIOC_GETSTATUS:
    ret = put_user(0, p);
    break;
    case WDIOC_GETBOOTSTATUS:
    ret = put_user(boot_status, p);
    break;
    case WDIOC_KEEPALIVE:
    sa1100_wr(sa1100_rd(REG_OSCR) + pre_margin, REG_OSMR3);
    ret = 0;
    break;
    case WDIOC_SETTIMEOUT:
    ret = get_user(time, p);
    if (ret)
    break;
    if (time <= 0 || (oscr_freq * (long long)time >= 0xffffffff)) {
    ret = -EINVAL;
    break;
    }
    pre_margin = oscr_freq * time;
    sa1100_wr(sa1100_rd(REG_OSCR) + pre_margin, REG_OSMR3);
    fallthrough;
    case WDIOC_GETTIMEOUT:
    ret = put_user(pre_margin / oscr_freq, p);
    break;
    }
    return ret;
    }
    static const struct file_operations sa1100dog_fops = {
    .owner		= THIS_MODULE,
    .write		= sa1100dog_write,
    .unlocked_ioctl	= sa1100dog_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open		= sa1100dog_open,
    .release	= sa1100dog_release,
    };
    static struct miscdevice sa1100dog_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &sa1100dog_fops,
    };
    static int margin = 60;		/* (secs) Default is 1 minute */
    static struct clk *clk;
#[no_mangle]
unsafe extern "C" fn sa1100dog_probe(pdev: *mut platform_device) -> c_int {
    static int sa1100dog_probe(struct platform_device *pdev)
    {
    int ret;
    int *platform_data;
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENXIO;
    reg_base = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!reg_base)
    return -ENOMEM;
    clk = clk_get(core::ptr::null_mut(), "OSTIMER0");
    if (IS_ERR(clk)) {
    pr_err("SA1100/PXA2xx Watchdog Timer: clock not found: %d\n",
    (int) PTR_ERR(clk));
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("SA1100/PXA2xx Watchdog Timer: clock failed to prepare+enable: %d\n",
    ret);
    goto err;
    }
    oscr_freq = clk_get_rate(clk);
    platform_data = pdev.dev.platform_data;
    if (platform_data && *platform_data)
    boot_status = WDIOF_CARDRESET;
    pre_margin = oscr_freq * margin;
    ret = misc_register(&sa1100dog_miscdev);
    if (ret == 0) {
    pr_info("SA1100/PXA2xx Watchdog Timer: timer margin %d sec\n",
    margin);
    return 0;
    }
    clk_disable_unprepare(clk);
    err:
    clk_put(clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sa1100dog_remove(pdev: *mut platform_device) {
    static void sa1100dog_remove(struct platform_device *pdev)
    {
    misc_deregister(&sa1100dog_miscdev);
    clk_disable_unprepare(clk);
    clk_put(clk);
    }
    static struct platform_driver sa1100dog_driver = {
    .driver.name = "sa1100_wdt",
    .probe = sa1100dog_probe,
    .remove = sa1100dog_remove,
    };
    module_platform_driver(sa1100dog_driver);
    MODULE_AUTHOR("Oleg Drokin <green@crimea.edu>");
    MODULE_DESCRIPTION("SA1100/PXA2xx Watchdog");
    module_param(margin, int, 0);
    MODULE_PARM_DESC(margin, "Watchdog margin in seconds (default 60s)");
    MODULE_LICENSE("GPL");
