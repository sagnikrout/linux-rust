//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rp5c01.c
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
// Ricoh RP5C01 RTC Driver
//
// Copyright 2009 Geert Uytterhoeven
//
// Based on the A3000 TOD code in arch/m68k/amiga/config.c
// Copyright (C) 1993 Hamish Macdonald
//

    enum {
    RP5C01_1_SECOND		= 0x0,	/* MODE 00 */
    RP5C01_10_SECOND	= 0x1,	/* MODE 00 */
    RP5C01_1_MINUTE		= 0x2,	/* MODE 00 and MODE 01 */
    RP5C01_10_MINUTE	= 0x3,	/* MODE 00 and MODE 01 */
    RP5C01_1_HOUR		= 0x4,	/* MODE 00 and MODE 01 */
    RP5C01_10_HOUR		= 0x5,	/* MODE 00 and MODE 01 */
    RP5C01_DAY_OF_WEEK	= 0x6,	/* MODE 00 and MODE 01 */
    RP5C01_1_DAY		= 0x7,	/* MODE 00 and MODE 01 */
    RP5C01_10_DAY		= 0x8,	/* MODE 00 and MODE 01 */
    RP5C01_1_MONTH		= 0x9,	/* MODE 00 */
    RP5C01_10_MONTH		= 0xa,	/* MODE 00 */
    RP5C01_1_YEAR		= 0xb,	/* MODE 00 */
    RP5C01_10_YEAR		= 0xc,	/* MODE 00 */
    RP5C01_12_24_SELECT	= 0xa,	/* MODE 01 */
    RP5C01_LEAP_YEAR	= 0xb,	/* MODE 01 */
    RP5C01_MODE		= 0xd,	/* all modes */
    RP5C01_TEST		= 0xe,	/* all modes */
    RP5C01_RESET		= 0xf,	/* all modes */
    };

// seconds or smaller units

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp5c01_priv {
    pub regs: *mut u32 __iomem,
    pub rtc: *mut rtc_device,
    pub /: *mut *mut spinlock_t lock; / against concurrent RTC/NVRAM access,
}

    static inline unsigned int rp5c01_read(struct rp5c01_priv *priv,
    unsigned int reg)
    {
    return __raw_readl(&priv.regs[reg]) & 0xf;
    }
    static inline void rp5c01_write(struct rp5c01_priv *priv, unsigned int val,
    unsigned int reg)
    {
    __raw_writel(val, &priv.regs[reg]);
    }
#[no_mangle]
unsafe extern "C" fn rp5c01_lock(priv: *mut rp5c01_priv) {
    static void rp5c01_lock(struct rp5c01_priv *priv)
    {
    rp5c01_write(priv, RP5C01_MODE_MODE00, RP5C01_MODE);
    }
#[no_mangle]
unsafe extern "C" fn rp5c01_unlock(priv: *mut rp5c01_priv) {
    static void rp5c01_unlock(struct rp5c01_priv *priv)
    {
    rp5c01_write(priv, RP5C01_MODE_TIMER_EN | RP5C01_MODE_MODE01,
    RP5C01_MODE);
    }
#[no_mangle]
unsafe extern "C" fn rp5c01_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rp5c01_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rp5c01_priv *priv = dev_get_drvdata(dev);
    spin_lock_irq(&priv.lock);
    rp5c01_lock(priv);
    tm.tm_sec  = rp5c01_read(priv, RP5C01_10_SECOND) * 10 +
    rp5c01_read(priv, RP5C01_1_SECOND);
    tm.tm_min  = rp5c01_read(priv, RP5C01_10_MINUTE) * 10 +
    rp5c01_read(priv, RP5C01_1_MINUTE);
    tm.tm_hour = rp5c01_read(priv, RP5C01_10_HOUR) * 10 +
    rp5c01_read(priv, RP5C01_1_HOUR);
    tm.tm_mday = rp5c01_read(priv, RP5C01_10_DAY) * 10 +
    rp5c01_read(priv, RP5C01_1_DAY);
    tm.tm_wday = rp5c01_read(priv, RP5C01_DAY_OF_WEEK);
    tm.tm_mon  = rp5c01_read(priv, RP5C01_10_MONTH) * 10 +
    rp5c01_read(priv, RP5C01_1_MONTH) - 1;
    tm.tm_year = rp5c01_read(priv, RP5C01_10_YEAR) * 10 +
    rp5c01_read(priv, RP5C01_1_YEAR);
    if (tm.tm_year <= 69)
    tm.tm_year += 100;
    rp5c01_unlock(priv);
    spin_unlock_irq(&priv.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp5c01_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rp5c01_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rp5c01_priv *priv = dev_get_drvdata(dev);
    spin_lock_irq(&priv.lock);
    rp5c01_lock(priv);
    rp5c01_write(priv, tm.tm_sec / 10, RP5C01_10_SECOND);
    rp5c01_write(priv, tm.tm_sec % 10, RP5C01_1_SECOND);
    rp5c01_write(priv, tm.tm_min / 10, RP5C01_10_MINUTE);
    rp5c01_write(priv, tm.tm_min % 10, RP5C01_1_MINUTE);
    rp5c01_write(priv, tm.tm_hour / 10, RP5C01_10_HOUR);
    rp5c01_write(priv, tm.tm_hour % 10, RP5C01_1_HOUR);
    rp5c01_write(priv, tm.tm_mday / 10, RP5C01_10_DAY);
    rp5c01_write(priv, tm.tm_mday % 10, RP5C01_1_DAY);
    if (tm.tm_wday != -1)
    rp5c01_write(priv, tm.tm_wday, RP5C01_DAY_OF_WEEK);
    rp5c01_write(priv, (tm.tm_mon + 1) / 10, RP5C01_10_MONTH);
    rp5c01_write(priv, (tm.tm_mon + 1) % 10, RP5C01_1_MONTH);
    if (tm.tm_year >= 100)
    tm.tm_year -= 100;
    rp5c01_write(priv, tm.tm_year / 10, RP5C01_10_YEAR);
    rp5c01_write(priv, tm.tm_year % 10, RP5C01_1_YEAR);
    rp5c01_unlock(priv);
    spin_unlock_irq(&priv.lock);
    return 0;
    }
    static const struct rtc_class_ops rp5c01_rtc_ops = {
    .read_time	= rp5c01_read_time,
    .set_time	= rp5c01_set_time,
    };
//
// The NVRAM is organized as 2 blocks of 13 nibbles of 4 bits.
// We provide access to them like AmigaOS does: the high nibble of each 8-bit
// byte is stored in BLOCK10, the low nibble in BLOCK11.
//
    static int rp5c01_nvram_read(void *_priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct rp5c01_priv *priv = _priv;
    u8 *buf = val;
    spin_lock_irq(&priv.lock);
    for (; bytes; bytes--) {
    u8 data;
    rp5c01_write(priv,
    RP5C01_MODE_TIMER_EN | RP5C01_MODE_RAM_BLOCK10,
    RP5C01_MODE);
    data = rp5c01_read(priv, pos) << 4;
    rp5c01_write(priv,
    RP5C01_MODE_TIMER_EN | RP5C01_MODE_RAM_BLOCK11,
    RP5C01_MODE);
    data |= rp5c01_read(priv, pos++);
    rp5c01_write(priv, RP5C01_MODE_TIMER_EN | RP5C01_MODE_MODE01,
    RP5C01_MODE);
// buf++ = data;
    }
    spin_unlock_irq(&priv.lock);
    return 0;
    }
    static int rp5c01_nvram_write(void *_priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct rp5c01_priv *priv = _priv;
    u8 *buf = val;
    spin_lock_irq(&priv.lock);
    for (; bytes; bytes--) {
    let mut data: u8 = *buf++;
    rp5c01_write(priv,
    RP5C01_MODE_TIMER_EN | RP5C01_MODE_RAM_BLOCK10,
    RP5C01_MODE);
    rp5c01_write(priv, data >> 4, pos);
    rp5c01_write(priv,
    RP5C01_MODE_TIMER_EN | RP5C01_MODE_RAM_BLOCK11,
    RP5C01_MODE);
    rp5c01_write(priv, data & 0xf, pos++);
    rp5c01_write(priv, RP5C01_MODE_TIMER_EN | RP5C01_MODE_MODE01,
    RP5C01_MODE);
    }
    spin_unlock_irq(&priv.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp5c01_rtc_probe(dev: *mut platform_device) -> int __init {
    static int __init rp5c01_rtc_probe(struct platform_device *dev)
    {
    struct resource *res;
    struct rp5c01_priv *priv;
    struct rtc_device *rtc;
    int error;
    struct nvmem_config nvmem_cfg = {
    .name = "rp5c01_nvram",
    .word_size = 1,
    .stride = 1,
    .size = RP5C01_MODE,
    .reg_read = rp5c01_nvram_read,
    .reg_write = rp5c01_nvram_write,
    };
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    priv = devm_kzalloc(&dev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_ioremap(&dev.dev, res.start, resource_size(res));
    if (!priv.regs)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    platform_set_drvdata(dev, priv);
    rtc = devm_rtc_allocate_device(&dev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &rp5c01_rtc_ops;
    priv.rtc = rtc;
    nvmem_cfg.priv = priv;
    error = devm_rtc_nvmem_register(rtc, &nvmem_cfg);
    if (error)
    return error;
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver rp5c01_rtc_driver = {
    .driver	= {
    .name	= "rtc-rp5c01",
    },
    };
    module_platform_driver_probe(rp5c01_rtc_driver, rp5c01_rtc_probe);
    MODULE_AUTHOR("Geert Uytterhoeven <geert@linux-m68k.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Ricoh RP5C01 RTC driver");
    MODULE_ALIAS("platform:rtc-rp5c01");
