//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-jz4740.c
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
// Copyright (C) 2009-2010, Lars-Peter Clausen <lars@metafoo.de>
// Copyright (C) 2010, Paul Cercueil <paul@crapouillou.net>
// JZ4740 SoC RTC driver
//

pub const JZ_REG_RTC_CTRL: c_uint = 0x00;
pub const JZ_REG_RTC_SEC: c_uint = 0x04;
pub const JZ_REG_RTC_SEC_ALARM: c_uint = 0x08;
pub const JZ_REG_RTC_REGULATOR: c_uint = 0x0C;
pub const JZ_REG_RTC_HIBERNATE: c_uint = 0x20;
pub const JZ_REG_RTC_WAKEUP_FILTER: c_uint = 0x24;
pub const JZ_REG_RTC_RESET_COUNTER: c_uint = 0x28;
pub const JZ_REG_RTC_SCRATCHPAD: c_uint = 0x34;
pub const JZ_REG_RTC_CKPCR: c_uint = 0x40;
// The following are present on the jz4780
pub const JZ_REG_RTC_WENR: c_uint = 0x3C;

// Magic value to enable writes on jz4780
pub const JZ_RTC_WENR_MAGIC: c_uint = 0xA55A;
pub const JZ_RTC_WAKEUP_FILTER_MASK: c_uint = 0x0000FFE0;
pub const JZ_RTC_RESET_COUNTER_MASK: c_uint = 0x00000FE0;

    enum jz4740_rtc_type {
    ID_JZ4740,
    ID_JZ4760,
    ID_JZ4780,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz4740_rtc {
    pub base: *mut void __iomem,
    pub type: enum jz4740_rtc_type,
    pub rtc: *mut rtc_device,
    pub clk32k: clk_hw,
    pub lock: spinlock_t,
}

    static struct device *dev_for_power_off;
#[no_mangle]
pub unsafe extern "C" fn jz4740_rtc_reg_read(rtc: *mut jz4740_rtc, reg: usize) -> u32 {
    static inline uint32_t jz4740_rtc_reg_read(struct jz4740_rtc *rtc, size_t reg)
    {
    return readl(rtc.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_wait_write_ready(rtc: *mut jz4740_rtc) -> c_int {
    static int jz4740_rtc_wait_write_ready(struct jz4740_rtc *rtc)
    {
    uint32_t ctrl;
    return readl_poll_timeout(rtc.base + JZ_REG_RTC_CTRL, ctrl,
    ctrl & JZ_RTC_CTRL_WRDY, 0, 1000);
    }
#[no_mangle]
pub unsafe extern "C" fn jz4780_rtc_enable_write(rtc: *mut jz4740_rtc) -> c_int {
    static inline int jz4780_rtc_enable_write(struct jz4740_rtc *rtc)
    {
    uint32_t ctrl;
    int ret;
    ret = jz4740_rtc_wait_write_ready(rtc);
    if (ret != 0)
    return ret;
    writel(JZ_RTC_WENR_MAGIC, rtc.base + JZ_REG_RTC_WENR);
    return readl_poll_timeout(rtc.base + JZ_REG_RTC_WENR, ctrl,
    ctrl & JZ_RTC_WENR_WEN, 0, 1000);
    }
    static inline int jz4740_rtc_reg_write(struct jz4740_rtc *rtc, size_t reg,
    uint32_t val)
    {
    let mut ret: c_int = 0;
    if (rtc.type >= ID_JZ4760)
    ret = jz4780_rtc_enable_write(rtc);
    if (ret == 0)
    ret = jz4740_rtc_wait_write_ready(rtc);
    if (ret == 0)
    writel(val, rtc.base + reg);
    return ret;
    }
    static int jz4740_rtc_ctrl_set_bits(struct jz4740_rtc *rtc, uint32_t mask,
    bool set)
    {
    int ret;
    unsigned long flags;
    uint32_t ctrl;
    spin_lock_irqsave(&rtc.lock, flags);
    ctrl = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_CTRL);
// Don't clear interrupt flags by accident
    ctrl |= JZ_RTC_CTRL_1HZ | JZ_RTC_CTRL_AF;
    if (set)
    ctrl |= mask;
    else
    ctrl &= ~mask;
    ret = jz4740_rtc_reg_write(rtc, JZ_REG_RTC_CTRL, ctrl);
    spin_unlock_irqrestore(&rtc.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_read_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int jz4740_rtc_read_time(struct device *dev, struct rtc_time *time)
    {
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    uint32_t secs, secs2;
    let mut timeout: c_int = 5;
    if (jz4740_rtc_reg_read(rtc, JZ_REG_RTC_SCRATCHPAD) != 0x12345678)
    return -EINVAL;
// If the seconds register is read while it is updated, it can contain a
// bogus value. This can be avoided by making sure that two consecutive
// reads have the same value.
//
    secs = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_SEC);
    secs2 = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_SEC);
    while (secs != secs2 && --timeout) {
    secs = secs2;
    secs2 = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_SEC);
    }
    if (timeout == 0)
    return -EIO;
    rtc_time64_to_tm(secs, time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_set_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int jz4740_rtc_set_time(struct device *dev, struct rtc_time *time)
    {
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    int ret;
    ret = jz4740_rtc_reg_write(rtc, JZ_REG_RTC_SEC, rtc_tm_to_time64(time));
    if (ret)
    return ret;
    return jz4740_rtc_reg_write(rtc, JZ_REG_RTC_SCRATCHPAD, 0x12345678);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int jz4740_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    uint32_t secs;
    uint32_t ctrl;
    secs = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_SEC_ALARM);
    ctrl = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_CTRL);
    alrm.enabled = !!(ctrl & JZ_RTC_CTRL_AE);
    alrm.pending = !!(ctrl & JZ_RTC_CTRL_AF);
    rtc_time64_to_tm(secs, &alrm.time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int jz4740_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    int ret;
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    let mut secs: u32 = lower_32_bits(rtc_tm_to_time64(&alrm.time));
    ret = jz4740_rtc_reg_write(rtc, JZ_REG_RTC_SEC_ALARM, secs);
    if (!ret)
    ret = jz4740_rtc_ctrl_set_bits(rtc,
    JZ_RTC_CTRL_AE | JZ_RTC_CTRL_AF_IRQ, alrm.enabled);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int jz4740_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    return jz4740_rtc_ctrl_set_bits(rtc, JZ_RTC_CTRL_AF_IRQ, enable);
    }
    static const struct rtc_class_ops jz4740_rtc_ops = {
    .read_time	= jz4740_rtc_read_time,
    .set_time	= jz4740_rtc_set_time,
    .read_alarm	= jz4740_rtc_read_alarm,
    .set_alarm	= jz4740_rtc_set_alarm,
    .alarm_irq_enable = jz4740_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t jz4740_rtc_irq(int irq, void *data)
    {
    struct jz4740_rtc *rtc = data;
    uint32_t ctrl;
    let mut events: c_ulong = 0;
    ctrl = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_CTRL);
    if (ctrl & JZ_RTC_CTRL_1HZ)
    events |= (RTC_UF | RTC_IRQF);
    if (ctrl & JZ_RTC_CTRL_AF)
    events |= (RTC_AF | RTC_IRQF);
    rtc_update_irq(rtc.rtc, 1, events);
    jz4740_rtc_ctrl_set_bits(rtc, JZ_RTC_CTRL_1HZ | JZ_RTC_CTRL_AF, false);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_poweroff(dev: *mut device) {
    static void jz4740_rtc_poweroff(struct device *dev)
    {
    struct jz4740_rtc *rtc = dev_get_drvdata(dev);
    jz4740_rtc_reg_write(rtc, JZ_REG_RTC_HIBERNATE, 1);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_power_off() {
    static void jz4740_rtc_power_off(void)
    {
    jz4740_rtc_poweroff(dev_for_power_off);
    kernel_halt();
    }
    static const struct of_device_id jz4740_rtc_of_match[] = {
    { .compatible = "ingenic,jz4740-rtc", .data = (void *)ID_JZ4740 },
    { .compatible = "ingenic,jz4760-rtc", .data = (void *)ID_JZ4760 },
    { .compatible = "ingenic,jz4770-rtc", .data = (void *)ID_JZ4780 },
    { .compatible = "ingenic,jz4780-rtc", .data = (void *)ID_JZ4780 },
    {},
    };
    MODULE_DEVICE_TABLE(of, jz4740_rtc_of_match);
    static void jz4740_rtc_set_wakeup_params(struct jz4740_rtc *rtc,
    struct device_node *np,
    unsigned long rate)
    {
    unsigned long wakeup_ticks, reset_ticks;
    unsigned int min_wakeup_pin_assert_time = 60; /* Default: 60ms */
    unsigned int reset_pin_assert_time = 100; /* Default: 100ms */
    of_property_read_u32(np, "ingenic,reset-pin-assert-time-ms",
    &reset_pin_assert_time);
    of_property_read_u32(np, "ingenic,min-wakeup-pin-assert-time-ms",
    &min_wakeup_pin_assert_time);
//
// Set minimum wakeup pin assertion time: 100 ms.
// Range is 0 to 2 sec if RTC is clocked at 32 kHz.
//
    wakeup_ticks = (min_wakeup_pin_assert_time * rate) / 1000;
    if (wakeup_ticks < JZ_RTC_WAKEUP_FILTER_MASK)
    wakeup_ticks &= JZ_RTC_WAKEUP_FILTER_MASK;
    else
    wakeup_ticks = JZ_RTC_WAKEUP_FILTER_MASK;
    jz4740_rtc_reg_write(rtc, JZ_REG_RTC_WAKEUP_FILTER, wakeup_ticks);
//
// Set reset pin low-level assertion time after wakeup: 60 ms.
// Range is 0 to 125 ms if RTC is clocked at 32 kHz.
//
    reset_ticks = (reset_pin_assert_time * rate) / 1000;
    if (reset_ticks < JZ_RTC_RESET_COUNTER_MASK)
    reset_ticks &= JZ_RTC_RESET_COUNTER_MASK;
    else
    reset_ticks = JZ_RTC_RESET_COUNTER_MASK;
    jz4740_rtc_reg_write(rtc, JZ_REG_RTC_RESET_COUNTER, reset_ticks);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_clk32k_enable(hw: *mut clk_hw) -> c_int {
    static int jz4740_rtc_clk32k_enable(struct clk_hw *hw)
    {
    struct jz4740_rtc *rtc = container_of(hw, struct jz4740_rtc, clk32k);
    return jz4740_rtc_reg_write(rtc, JZ_REG_RTC_CKPCR,
    JZ_RTC_CKPCR_CK32PULL_DIS |
    JZ_RTC_CKPCR_CK32CTL_EN);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_clk32k_disable(hw: *mut clk_hw) {
    static void jz4740_rtc_clk32k_disable(struct clk_hw *hw)
    {
    struct jz4740_rtc *rtc = container_of(hw, struct jz4740_rtc, clk32k);
    jz4740_rtc_reg_write(rtc, JZ_REG_RTC_CKPCR, 0);
    }
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_clk32k_is_enabled(hw: *mut clk_hw) -> c_int {
    static int jz4740_rtc_clk32k_is_enabled(struct clk_hw *hw)
    {
    struct jz4740_rtc *rtc = container_of(hw, struct jz4740_rtc, clk32k);
    u32 ckpcr;
    ckpcr = jz4740_rtc_reg_read(rtc, JZ_REG_RTC_CKPCR);
    return !!(ckpcr & JZ_RTC_CKPCR_CK32CTL_EN);
    }
    static const struct clk_ops jz4740_rtc_clk32k_ops = {
    .enable = jz4740_rtc_clk32k_enable,
    .disable = jz4740_rtc_clk32k_disable,
    .is_enabled = jz4740_rtc_clk32k_is_enabled,
    };
#[no_mangle]
unsafe extern "C" fn jz4740_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int jz4740_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct jz4740_rtc *rtc;
    unsigned long rate;
    struct clk *clk;
    int ret, irq;
    rtc = devm_kzalloc(dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.type = (uintptr_t)device_get_match_data(dev);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    rtc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.base))
    return PTR_ERR(rtc.base);
    clk = devm_clk_get_enabled(dev, "rtc");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Failed to get RTC clock\n");
    spin_lock_init(&rtc.lock);
    platform_set_drvdata(pdev, rtc);
    device_init_wakeup(dev, true);
    ret = dev_pm_set_wake_irq(dev, irq);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to set wake irq\n");
    rtc.rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rtc.rtc))
    return dev_err_probe(dev, PTR_ERR(rtc.rtc),
    "Failed to allocate rtc device\n");
    rtc.rtc.ops = &jz4740_rtc_ops;
    rtc.rtc.range_max = U32_MAX;
    rate = clk_get_rate(clk);
    jz4740_rtc_set_wakeup_params(rtc, np, rate);
// Each 1 Hz pulse should happen after (rate) ticks
    jz4740_rtc_reg_write(rtc, JZ_REG_RTC_REGULATOR, rate - 1);
    ret = devm_rtc_register_device(rtc.rtc);
    if (ret)
    return ret;
    ret = devm_request_irq(dev, irq, jz4740_rtc_irq, 0,
    pdev.name, rtc);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request rtc irq\n");
    if (of_device_is_system_power_controller(np)) {
    dev_for_power_off = dev;
    if (!pm_power_off)
    pm_power_off = jz4740_rtc_power_off;
    else
    dev_warn(dev, "Poweroff handler already present!\n");
    }
    if (device_property_present(dev, "#clock-cells")) {
    rtc.clk32k.init = CLK_HW_INIT_HW("clk32k", __clk_get_hw(clk),
    &jz4740_rtc_clk32k_ops, 0);
    ret = devm_clk_hw_register(dev, &rtc.clk32k);
    if (ret)
    return dev_err_probe(dev, ret,
    "Unable to register clk32k clock\n");
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get,
    &rtc.clk32k);
    if (ret)
    return dev_err_probe(dev, ret,
    "Unable to register clk32k clock provider\n");
    }
    return 0;
    }
    static struct platform_driver jz4740_rtc_driver = {
    .probe	 = jz4740_rtc_probe,
    .driver	 = {
    .name  = "jz4740-rtc",
    .of_match_table = jz4740_rtc_of_match,
    },
    };
    module_platform_driver(jz4740_rtc_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("RTC driver for the JZ4740 SoC\n");
