//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-msc313.c
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
// Real time clocks driver for MStar/SigmaStar ARMv7 SoCs.
// Based on "Real Time Clock driver for msb252x." that was contained
// in various MStar kernels.
//
// (C) 2019 Daniel Palmer
// (C) 2021 Romain Perier
//

// Registers
pub const REG_RTC_CTRL: c_uint = 0x00;
pub const REG_RTC_FREQ_CW_L: c_uint = 0x04;
pub const REG_RTC_FREQ_CW_H: c_uint = 0x08;
pub const REG_RTC_LOAD_VAL_L: c_uint = 0x0C;
pub const REG_RTC_LOAD_VAL_H: c_uint = 0x10;
pub const REG_RTC_MATCH_VAL_L: c_uint = 0x14;
pub const REG_RTC_MATCH_VAL_H: c_uint = 0x18;
pub const REG_RTC_STATUS_INT: c_uint = 0x1C;
pub const REG_RTC_CNT_VAL_L: c_uint = 0x20;
pub const REG_RTC_CNT_VAL_H: c_uint = 0x24;
// Control bits for REG_RTC_CTRL

// Control bits for REG_RTC_STATUS_INT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc313_rtc {
    pub rtc_dev: *mut rtc_device,
    pub rtc_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn msc313_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int msc313_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct msc313_rtc *priv = dev_get_drvdata(dev);
    unsigned long seconds;
    seconds = readw(priv.rtc_base + REG_RTC_MATCH_VAL_L)
    | ((unsigned long)readw(priv.rtc_base + REG_RTC_MATCH_VAL_H) << 16);
    rtc_time64_to_tm(seconds, &alarm.time);
    if (!(readw(priv.rtc_base + REG_RTC_CTRL) & INT_MASK_BIT))
    alarm.enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int msc313_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct msc313_rtc *priv = dev_get_drvdata(dev);
    u16 reg;
    reg = readw(priv.rtc_base + REG_RTC_CTRL);
    if (enabled)
    reg &= ~INT_MASK_BIT;
    else
    reg |= INT_MASK_BIT;
    writew(reg, priv.rtc_base + REG_RTC_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int msc313_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct msc313_rtc *priv = dev_get_drvdata(dev);
    unsigned long seconds;
    seconds = rtc_tm_to_time64(&alarm.time);
    writew((seconds & 0xFFFF), priv.rtc_base + REG_RTC_MATCH_VAL_L);
    writew((seconds >> 16) & 0xFFFF, priv.rtc_base + REG_RTC_MATCH_VAL_H);
    msc313_rtc_alarm_irq_enable(dev, alarm.enabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_get_enabled(priv: *mut msc313_rtc) -> bool {
    static bool msc313_rtc_get_enabled(struct msc313_rtc *priv)
    {
    return readw(priv.rtc_base + REG_RTC_CTRL) & CNT_EN_BIT;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_set_enabled(priv: *mut msc313_rtc) {
    static void msc313_rtc_set_enabled(struct msc313_rtc *priv)
    {
    u16 reg;
    reg = readw(priv.rtc_base + REG_RTC_CTRL);
    reg |= CNT_EN_BIT;
    writew(reg, priv.rtc_base + REG_RTC_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int msc313_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct msc313_rtc *priv = dev_get_drvdata(dev);
    u32 seconds;
    u16 reg;
    if (!msc313_rtc_get_enabled(priv))
    return -EINVAL;
    reg = readw(priv.rtc_base + REG_RTC_CTRL);
    writew(reg | READ_EN_BIT, priv.rtc_base + REG_RTC_CTRL);
// Wait for HW latch done
    while (readw(priv.rtc_base + REG_RTC_CTRL) & READ_EN_BIT)
    udelay(1);
    seconds = readw(priv.rtc_base + REG_RTC_CNT_VAL_L)
    | ((unsigned long)readw(priv.rtc_base + REG_RTC_CNT_VAL_H) << 16);
    rtc_time64_to_tm(seconds, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int msc313_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct msc313_rtc *priv = dev_get_drvdata(dev);
    unsigned long seconds;
    u16 reg;
    seconds = rtc_tm_to_time64(tm);
    writew(seconds & 0xFFFF, priv.rtc_base + REG_RTC_LOAD_VAL_L);
    writew((seconds >> 16) & 0xFFFF, priv.rtc_base + REG_RTC_LOAD_VAL_H);
// Enable load for loading value into internal RTC counter
    reg = readw(priv.rtc_base + REG_RTC_CTRL);
    writew(reg | LOAD_EN_BIT, priv.rtc_base + REG_RTC_CTRL);
// Wait for HW latch done
    while (readw(priv.rtc_base + REG_RTC_CTRL) & LOAD_EN_BIT)
    udelay(1);
    msc313_rtc_set_enabled(priv);
    return 0;
    }
    static const struct rtc_class_ops msc313_rtc_ops = {
    .read_time = msc313_rtc_read_time,
    .set_time = msc313_rtc_set_time,
    .read_alarm = msc313_rtc_read_alarm,
    .set_alarm = msc313_rtc_set_alarm,
    .alarm_irq_enable = msc313_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn msc313_rtc_interrupt(irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t msc313_rtc_interrupt(s32 irq, void *dev_id)
    {
    struct msc313_rtc *priv = dev_id;
    u16 reg;
    reg = readw(priv.rtc_base + REG_RTC_STATUS_INT);
    if (!(reg & ALM_INT_BIT))
    return IRQ_NONE;
    reg = readw(priv.rtc_base + REG_RTC_CTRL);
    reg |= INT_CLEAR_BIT;
    reg &= ~INT_FORCE_BIT;
    writew(reg, priv.rtc_base + REG_RTC_CTRL);
    rtc_update_irq(priv.rtc_dev, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn msc313_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int msc313_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct msc313_rtc *priv;
    unsigned long rate;
    struct clk *clk;
    int ret;
    int irq;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct msc313_rtc), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.rtc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.rtc_base))
    return PTR_ERR(priv.rtc_base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    priv.rtc_dev = devm_rtc_allocate_device(dev);
    if (IS_ERR(priv.rtc_dev))
    return PTR_ERR(priv.rtc_dev);
    priv.rtc_dev.ops = &msc313_rtc_ops;
    priv.rtc_dev.range_max = U32_MAX;
    ret = devm_request_irq(dev, irq, msc313_rtc_interrupt, IRQF_SHARED,
    dev_name(&pdev.dev), priv);
    if (ret) {
    dev_err(dev, "Could not request IRQ\n");
    return ret;
    }
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(dev, "No input reference clock\n");
    return PTR_ERR(clk);
    }
    rate = clk_get_rate(clk);
    writew(rate & 0xFFFF, priv.rtc_base + REG_RTC_FREQ_CW_L);
    writew((rate >> 16) & 0xFFFF, priv.rtc_base + REG_RTC_FREQ_CW_H);
    platform_set_drvdata(pdev, priv);
    return devm_rtc_register_device(priv.rtc_dev);
    }
    static const struct of_device_id msc313_rtc_of_match_table[] = {
    { .compatible = "mstar,msc313-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, msc313_rtc_of_match_table);
    static struct platform_driver msc313_rtc_driver = {
    .probe = msc313_rtc_probe,
    .driver = {
    .name = "msc313-rtc",
    .of_match_table = msc313_rtc_of_match_table,
    },
    };
    module_platform_driver(msc313_rtc_driver);
    MODULE_AUTHOR("Daniel Palmer <daniel@thingy.jp>");
    MODULE_AUTHOR("Romain Perier <romain.perier@gmail.com>");
    MODULE_DESCRIPTION("MStar RTC Driver");
    MODULE_LICENSE("GPL v2");
