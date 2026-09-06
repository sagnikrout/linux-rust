//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-bq4802.c
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
// rtc-bq4802.c: TI BQ4802 RTC driver.
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//

    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_DESCRIPTION("TI BQ4802 RTC driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq4802 {
    pub regs: *mut void __iomem,
    pub ioport: c_ulong,
    pub rtc: *mut rtc_device,
    pub lock: spinlock_t,
    pub r: *mut resource,
    pub int): *mut *mut *mut u8 (read)(struct bq4802 ,,
    pub u8): *mut *mut *mut void (write)(struct bq4802 , int,,
}

#[no_mangle]
unsafe extern "C" fn bq4802_read_io(p: *mut bq4802, off: c_int) -> u8 {
    static u8 bq4802_read_io(struct bq4802 *p, int off)
    {
    return inb(p.ioport + off);
    }
#[no_mangle]
unsafe extern "C" fn bq4802_write_io(p: *mut bq4802, off: c_int, val: u8) {
    static void bq4802_write_io(struct bq4802 *p, int off, u8 val)
    {
    outb(val, p.ioport + off);
    }
#[no_mangle]
unsafe extern "C" fn bq4802_read_mem(p: *mut bq4802, off: c_int) -> u8 {
    static u8 bq4802_read_mem(struct bq4802 *p, int off)
    {
    return readb(p.regs + off);
    }
#[no_mangle]
unsafe extern "C" fn bq4802_write_mem(p: *mut bq4802, off: c_int, val: u8) {
    static void bq4802_write_mem(struct bq4802 *p, int off, u8 val)
    {
    writeb(val, p.regs + off);
    }
#[no_mangle]
unsafe extern "C" fn bq4802_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int bq4802_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct bq4802 *p = dev_get_drvdata(dev);
    unsigned long flags;
    unsigned int century;
    u8 val;
    spin_lock_irqsave(&p.lock, flags);
    val = p.read(p, 0x0e);
    p.write(p, 0xe, val | 0x08);
    tm.tm_sec = p.read(p, 0x00);
    tm.tm_min = p.read(p, 0x02);
    tm.tm_hour = p.read(p, 0x04);
    tm.tm_mday = p.read(p, 0x06);
    tm.tm_mon = p.read(p, 0x09);
    tm.tm_year = p.read(p, 0x0a);
    tm.tm_wday = p.read(p, 0x08);
    century = p.read(p, 0x0f);
    p.write(p, 0x0e, val);
    spin_unlock_irqrestore(&p.lock, flags);
    tm.tm_sec = bcd2bin(tm.tm_sec);
    tm.tm_min = bcd2bin(tm.tm_min);
    tm.tm_hour = bcd2bin(tm.tm_hour);
    tm.tm_mday = bcd2bin(tm.tm_mday);
    tm.tm_mon = bcd2bin(tm.tm_mon);
    tm.tm_year = bcd2bin(tm.tm_year);
    tm.tm_wday = bcd2bin(tm.tm_wday);
    century = bcd2bin(century);
    tm.tm_year += (century * 100);
    tm.tm_year -= 1900;
    tm.tm_mon--;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bq4802_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int bq4802_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct bq4802 *p = dev_get_drvdata(dev);
    u8 sec, min, hrs, day, mon, yrs, century, val;
    unsigned long flags;
    unsigned int year;
    year = tm.tm_year + 1900;
    century = year / 100;
    yrs = year % 100;
    mon = tm.tm_mon + 1;   /* tm_mon starts at zero */
    day = tm.tm_mday;
    hrs = tm.tm_hour;
    min = tm.tm_min;
    sec = tm.tm_sec;
    sec = bin2bcd(sec);
    min = bin2bcd(min);
    hrs = bin2bcd(hrs);
    day = bin2bcd(day);
    mon = bin2bcd(mon);
    yrs = bin2bcd(yrs);
    century = bin2bcd(century);
    spin_lock_irqsave(&p.lock, flags);
    val = p.read(p, 0x0e);
    p.write(p, 0x0e, val | 0x08);
    p.write(p, 0x00, sec);
    p.write(p, 0x02, min);
    p.write(p, 0x04, hrs);
    p.write(p, 0x06, day);
    p.write(p, 0x09, mon);
    p.write(p, 0x0a, yrs);
    p.write(p, 0x0f, century);
    p.write(p, 0x0e, val);
    spin_unlock_irqrestore(&p.lock, flags);
    return 0;
    }
    static const struct rtc_class_ops bq4802_ops = {
    .read_time	= bq4802_read_time,
    .set_time	= bq4802_set_time,
    };
#[no_mangle]
unsafe extern "C" fn bq4802_probe(pdev: *mut platform_device) -> c_int {
    static int bq4802_probe(struct platform_device *pdev)
    {
    struct bq4802 *p = devm_kzalloc(&pdev.dev, sizeof(*p), GFP_KERNEL);
    let mut err: c_int = -ENOMEM;
    if (!p)
    goto out;
    spin_lock_init(&p.lock);
    p.r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!p.r) {
    p.r = platform_get_resource(pdev, IORESOURCE_IO, 0);
    err = -EINVAL;
    if (!p.r)
    goto out;
    }
    if (p.r.flags & IORESOURCE_IO) {
    p.ioport = p.r.start;
    p.read = bq4802_read_io;
    p.write = bq4802_write_io;
    } else if (p.r.flags & IORESOURCE_MEM) {
    p.regs = devm_ioremap(&pdev.dev, p.r.start,
    resource_size(p.r));
    if (!p.regs){
    err = -ENOMEM;
    goto out;
    }
    p.read = bq4802_read_mem;
    p.write = bq4802_write_mem;
    } else {
    err = -EINVAL;
    goto out;
    }
    platform_set_drvdata(pdev, p);
    p.rtc = devm_rtc_device_register(&pdev.dev, "bq4802",
    &bq4802_ops, THIS_MODULE);
    if (IS_ERR(p.rtc)) {
    err = PTR_ERR(p.rtc);
    goto out;
    }
    err = 0;
    out:
    return err;
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:rtc-bq4802");
    static struct platform_driver bq4802_driver = {
    .driver		= {
    .name	= "rtc-bq4802",
    },
    .probe		= bq4802_probe,
    };
    module_platform_driver(bq4802_driver);
