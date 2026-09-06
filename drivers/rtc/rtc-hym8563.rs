//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-hym8563.c
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
// Haoyu HYM8563 RTC driver
//
// Copyright (C) 2013 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
// based on rtc-HYM8563
// Copyright (C) 2010 ROCKCHIP, Inc.
//

pub const HYM8563_CTL1: c_uint = 0x00;

pub const HYM8563_CTL2: c_uint = 0x01;

pub const HYM8563_SEC: c_uint = 0x02;

pub const HYM8563_SEC_MASK: c_uint = 0x7f;
pub const HYM8563_MIN: c_uint = 0x03;
pub const HYM8563_MIN_MASK: c_uint = 0x7f;
pub const HYM8563_HOUR: c_uint = 0x04;
pub const HYM8563_HOUR_MASK: c_uint = 0x3f;
pub const HYM8563_DAY: c_uint = 0x05;
pub const HYM8563_DAY_MASK: c_uint = 0x3f;
pub const HYM8563_WEEKDAY: c_uint = 0x06;
pub const HYM8563_WEEKDAY_MASK: c_uint = 0x07;
pub const HYM8563_MONTH: c_uint = 0x07;

pub const HYM8563_MONTH_MASK: c_uint = 0x1f;
pub const HYM8563_YEAR: c_uint = 0x08;
pub const HYM8563_ALM_MIN: c_uint = 0x09;
pub const HYM8563_ALM_HOUR: c_uint = 0x0a;
pub const HYM8563_ALM_DAY: c_uint = 0x0b;
pub const HYM8563_ALM_WEEK: c_uint = 0x0c;
// Each alarm check can be disabled by setting this bit in the register

pub const HYM8563_CLKOUT: c_uint = 0x0d;

pub const HYM8563_CLKOUT_32768: c_int = 0;
pub const HYM8563_CLKOUT_1024: c_int = 1;
pub const HYM8563_CLKOUT_32: c_int = 2;
pub const HYM8563_CLKOUT_1: c_int = 3;
pub const HYM8563_CLKOUT_MASK: c_int = 3;
pub const HYM8563_TMR_CTL: c_uint = 0x0e;

pub const HYM8563_TMR_CTL_4096: c_int = 0;
pub const HYM8563_TMR_CTL_64: c_int = 1;
pub const HYM8563_TMR_CTL_1: c_int = 2;
pub const HYM8563_TMR_CTL_1_60: c_int = 3;
pub const HYM8563_TMR_CTL_MASK: c_int = 3;
pub const HYM8563_TMR_CNT: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hym8563 {
    pub client: *mut i2c_client,
    pub rtc: *mut rtc_device,

    pub clkout_hw: clk_hw,

}

//
// RTC handling
//
#[no_mangle]
unsafe extern "C" fn hym8563_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int hym8563_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    u8 buf[7];
    int ret;
    ret = i2c_smbus_read_i2c_block_data(client, HYM8563_SEC, 7, buf);
    if (ret < 0)
    return ret;
    if (buf[0] & HYM8563_SEC_VL) {
    dev_warn(&client.dev,
    "no valid clock/calendar values available\n");
    return -EINVAL;
    }
    tm.tm_sec = bcd2bin(buf[0] & HYM8563_SEC_MASK);
    tm.tm_min = bcd2bin(buf[1] & HYM8563_MIN_MASK);
    tm.tm_hour = bcd2bin(buf[2] & HYM8563_HOUR_MASK);
    tm.tm_mday = bcd2bin(buf[3] & HYM8563_DAY_MASK);
    tm.tm_wday = bcd2bin(buf[4] & HYM8563_WEEKDAY_MASK); /* 0 = Sun */
    tm.tm_mon = bcd2bin(buf[5] & HYM8563_MONTH_MASK) - 1; /* 0 = Jan */
    tm.tm_year = bcd2bin(buf[6]) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hym8563_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int hym8563_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    u8 buf[7];
    int ret;
// Years >= 2100 are to far in the future, 19XX is to early
    if (tm.tm_year < 100 || tm.tm_year >= 200)
    return -EINVAL;
    buf[0] = bin2bcd(tm.tm_sec);
    buf[1] = bin2bcd(tm.tm_min);
    buf[2] = bin2bcd(tm.tm_hour);
    buf[3] = bin2bcd(tm.tm_mday);
    buf[4] = bin2bcd(tm.tm_wday);
    buf[5] = bin2bcd(tm.tm_mon + 1);
//
// While the HYM8563 has a century flag in the month register,
// it does not seem to carry it over a subsequent write/read.
// So we'll limit ourself to 100 years, starting at 2000 for now.
//
    buf[6] = bin2bcd(tm.tm_year - 100);
//
// CTL1 only contains TEST-mode bits apart from stop,
// so no need to read the value first
//
    ret = i2c_smbus_write_byte_data(client, HYM8563_CTL1,
    HYM8563_CTL1_STOP);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_i2c_block_data(client, HYM8563_SEC, 7, buf);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, HYM8563_CTL1, 0);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int hym8563_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int data;
    data = i2c_smbus_read_byte_data(client, HYM8563_CTL2);
    if (data < 0)
    return data;
    if (enabled)
    data |= HYM8563_CTL2_AIE;
    else
    data &= ~HYM8563_CTL2_AIE;
    return i2c_smbus_write_byte_data(client, HYM8563_CTL2, data);
    };
#[no_mangle]
unsafe extern "C" fn hym8563_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int hym8563_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rtc_time *alm_tm = &alm.time;
    u8 buf[4];
    int ret;
    ret = i2c_smbus_read_i2c_block_data(client, HYM8563_ALM_MIN, 4, buf);
    if (ret < 0)
    return ret;
// The alarm only has a minute accuracy
    alm_tm.tm_sec = 0;
    alm_tm.tm_min = (buf[0] & HYM8563_ALM_BIT_DISABLE) ?
    -1 :
    bcd2bin(buf[0] & HYM8563_MIN_MASK);
    alm_tm.tm_hour = (buf[1] & HYM8563_ALM_BIT_DISABLE) ?
    -1 :
    bcd2bin(buf[1] & HYM8563_HOUR_MASK);
    alm_tm.tm_mday = (buf[2] & HYM8563_ALM_BIT_DISABLE) ?
    -1 :
    bcd2bin(buf[2] & HYM8563_DAY_MASK);
    alm_tm.tm_wday = (buf[3] & HYM8563_ALM_BIT_DISABLE) ?
    -1 :
    bcd2bin(buf[3] & HYM8563_WEEKDAY_MASK);
    ret = i2c_smbus_read_byte_data(client, HYM8563_CTL2);
    if (ret < 0)
    return ret;
    if (ret & HYM8563_CTL2_AIE)
    alm.enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hym8563_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int hym8563_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rtc_time *alm_tm = &alm.time;
    u8 buf[4];
    int ret;
    ret = i2c_smbus_read_byte_data(client, HYM8563_CTL2);
    if (ret < 0)
    return ret;
    ret &= ~HYM8563_CTL2_AIE;
    ret = i2c_smbus_write_byte_data(client, HYM8563_CTL2, ret);
    if (ret < 0)
    return ret;
    buf[0] = (alm_tm.tm_min < 60 && alm_tm.tm_min >= 0) ?
    bin2bcd(alm_tm.tm_min) : HYM8563_ALM_BIT_DISABLE;
    buf[1] = (alm_tm.tm_hour < 24 && alm_tm.tm_hour >= 0) ?
    bin2bcd(alm_tm.tm_hour) : HYM8563_ALM_BIT_DISABLE;
    buf[2] = (alm_tm.tm_mday <= 31 && alm_tm.tm_mday >= 1) ?
    bin2bcd(alm_tm.tm_mday) : HYM8563_ALM_BIT_DISABLE;
    buf[3] = (alm_tm.tm_wday < 7 && alm_tm.tm_wday >= 0) ?
    bin2bcd(alm_tm.tm_wday) : HYM8563_ALM_BIT_DISABLE;
    ret = i2c_smbus_write_i2c_block_data(client, HYM8563_ALM_MIN, 4, buf);
    if (ret < 0)
    return ret;
    return hym8563_rtc_alarm_irq_enable(dev, alm.enabled);
    }
    static const struct rtc_class_ops hym8563_rtc_ops = {
    .read_time		= hym8563_rtc_read_time,
    .set_time		= hym8563_rtc_set_time,
    .alarm_irq_enable	= hym8563_rtc_alarm_irq_enable,
    .read_alarm		= hym8563_rtc_read_alarm,
    .set_alarm		= hym8563_rtc_set_alarm,
    };
//
// Handling of the clkout
//

    static int clkout_rates[] = {
    32768,
    1024,
    32,
    1,
    };
    static unsigned long hym8563_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct hym8563 *hym8563 = clkout_hw_to_hym8563(hw);
    struct i2c_client *client = hym8563.client;
    let mut ret: c_int = i2c_smbus_read_byte_data(client, HYM8563_CLKOUT);
    if (ret < 0)
    return 0;
    ret &= HYM8563_CLKOUT_MASK;
    return clkout_rates[ret];
    }
    static int hym8563_clkout_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] <= req.rate) {
    req.rate = clkout_rates[i];
    return 0;
    }
    req.rate = clkout_rates[0];
    return 0;
    }
    static int hym8563_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct hym8563 *hym8563 = clkout_hw_to_hym8563(hw);
    struct i2c_client *client = hym8563.client;
    let mut ret: c_int = i2c_smbus_read_byte_data(client, HYM8563_CLKOUT);
    int i;
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] == rate) {
    ret &= ~HYM8563_CLKOUT_MASK;
    ret |= i;
    return i2c_smbus_write_byte_data(client,
    HYM8563_CLKOUT, ret);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn hym8563_clkout_control(hw: *mut clk_hw, enable: bool) -> c_int {
    static int hym8563_clkout_control(struct clk_hw *hw, bool enable)
    {
    struct hym8563 *hym8563 = clkout_hw_to_hym8563(hw);
    struct i2c_client *client = hym8563.client;
    let mut ret: c_int = i2c_smbus_read_byte_data(client, HYM8563_CLKOUT);
    if (ret < 0)
    return ret;
    if (enable)
    ret |= HYM8563_CLKOUT_ENABLE;
    else
    ret &= ~HYM8563_CLKOUT_ENABLE;
    return i2c_smbus_write_byte_data(client, HYM8563_CLKOUT, ret);
    }
#[no_mangle]
unsafe extern "C" fn hym8563_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int hym8563_clkout_prepare(struct clk_hw *hw)
    {
    return hym8563_clkout_control(hw, 1);
    }
#[no_mangle]
unsafe extern "C" fn hym8563_clkout_unprepare(hw: *mut clk_hw) {
    static void hym8563_clkout_unprepare(struct clk_hw *hw)
    {
    hym8563_clkout_control(hw, 0);
    }
#[no_mangle]
unsafe extern "C" fn hym8563_clkout_is_prepared(hw: *mut clk_hw) -> c_int {
    static int hym8563_clkout_is_prepared(struct clk_hw *hw)
    {
    struct hym8563 *hym8563 = clkout_hw_to_hym8563(hw);
    struct i2c_client *client = hym8563.client;
    let mut ret: c_int = i2c_smbus_read_byte_data(client, HYM8563_CLKOUT);
    if (ret < 0)
    return ret;
    return !!(ret & HYM8563_CLKOUT_ENABLE);
    }
    static const struct clk_ops hym8563_clkout_ops = {
    .prepare = hym8563_clkout_prepare,
    .unprepare = hym8563_clkout_unprepare,
    .is_prepared = hym8563_clkout_is_prepared,
    .recalc_rate = hym8563_clkout_recalc_rate,
    .determine_rate = hym8563_clkout_determine_rate,
    .set_rate = hym8563_clkout_set_rate,
    };
    static struct clk *hym8563_clkout_register_clk(struct hym8563 *hym8563)
    {
    struct i2c_client *client = hym8563.client;
    struct device_node *node = client.dev.of_node;
    let mut init: clk_init_data = {};
    struct clk *clk;
    int ret;
    ret = i2c_smbus_write_byte_data(client, HYM8563_CLKOUT,
    0);
    if (ret < 0)
    return ERR_PTR(ret);
    init.name = "hym8563-clkout";
    init.ops = &hym8563_clkout_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    hym8563.clkout_hw.init = &init;
// optional override of the clockname
    of_property_read_string(node, "clock-output-names", &init.name);
// register the clock
    clk = clk_register(&client.dev, &hym8563.clkout_hw);
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return clk;
    }

//
// The alarm interrupt is implemented as a level-low interrupt in the
// hym8563, while the timer interrupt uses a falling edge.
// We don't use the timer at all, so the interrupt is requested to
// use the level-low trigger.
//
#[no_mangle]
unsafe extern "C" fn hym8563_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hym8563_irq(int irq, void *dev_id)
    {
    struct hym8563 *hym8563 = (struct hym8563 *)dev_id;
    struct i2c_client *client = hym8563.client;
    int data, ret;
    rtc_lock(hym8563.rtc);
// Clear the alarm flag
    data = i2c_smbus_read_byte_data(client, HYM8563_CTL2);
    if (data < 0) {
    dev_err(&client.dev, "%s: error reading i2c data %d\n",
    __func__, data);
    goto out;
    }
    data &= ~HYM8563_CTL2_AF;
    ret = i2c_smbus_write_byte_data(client, HYM8563_CTL2, data);
    if (ret < 0) {
    dev_err(&client.dev, "%s: error writing i2c data %d\n",
    __func__, ret);
    }
    out:
    rtc_unlock(hym8563.rtc);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hym8563_init_device(client: *mut i2c_client) -> c_int {
    static int hym8563_init_device(struct i2c_client *client)
    {
    int ret;
// Clear stop flag if present
    ret = i2c_smbus_write_byte_data(client, HYM8563_CTL1, 0);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(client, HYM8563_CTL2);
    if (ret < 0)
    return ret;
// Disable alarm and timer interrupts
    ret &= ~HYM8563_CTL2_AIE;
    ret &= ~HYM8563_CTL2_TIE;
// Clear any pending alarm and timer flags
    if (ret & HYM8563_CTL2_AF)
    ret &= ~HYM8563_CTL2_AF;
    if (ret & HYM8563_CTL2_TF)
    ret &= ~HYM8563_CTL2_TF;
    ret &= ~HYM8563_CTL2_TI_TP;
    return i2c_smbus_write_byte_data(client, HYM8563_CTL2, ret);
    }

#[no_mangle]
unsafe extern "C" fn hym8563_suspend(dev: *mut device) -> c_int {
    static int hym8563_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int ret;
    if (device_may_wakeup(dev)) {
    ret = enable_irq_wake(client.irq);
    if (ret) {
    dev_err(dev, "enable_irq_wake failed, %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hym8563_resume(dev: *mut device) -> c_int {
    static int hym8563_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(client.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(hym8563_pm_ops, hym8563_suspend, hym8563_resume);
#[no_mangle]
unsafe extern "C" fn hym8563_probe(client: *mut i2c_client) -> c_int {
    static int hym8563_probe(struct i2c_client *client)
    {
    struct hym8563 *hym8563;
    int ret;
    hym8563 = devm_kzalloc(&client.dev, sizeof(*hym8563), GFP_KERNEL);
    if (!hym8563)
    return -ENOMEM;
    hym8563.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(hym8563.rtc))
    return PTR_ERR(hym8563.rtc);
    hym8563.client = client;
    i2c_set_clientdata(client, hym8563);
    ret = hym8563_init_device(client);
    if (ret) {
    dev_err(&client.dev, "could not init device, %d\n", ret);
    return ret;
    }
    if (client.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&client.dev))
    irqflags = 0;
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), hym8563_irq,
    irqflags | IRQF_ONESHOT,
    client.name, hym8563);
    if (ret < 0) {
    dev_err(&client.dev, "irq %d request failed, %d\n",
    client.irq, ret);
    return ret;
    }
    }
    if (client.irq > 0 ||
    device_property_read_bool(&client.dev, "wakeup-source")) {
    device_init_wakeup(&client.dev, true);
    }
// check state of calendar information
    ret = i2c_smbus_read_byte_data(client, HYM8563_SEC);
    if (ret < 0)
    return ret;
    dev_dbg(&client.dev, "rtc information is %s\n",
    (ret & HYM8563_SEC_VL) ? "invalid" : "valid");
    hym8563.rtc.ops = &hym8563_rtc_ops;
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, hym8563.rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, hym8563.rtc.features);

    hym8563_clkout_register_clk(hym8563);

    return devm_rtc_register_device(hym8563.rtc);
    }
    static const struct i2c_device_id hym8563_id[] = {
    { .name = "hym8563" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hym8563_id);
    static const struct of_device_id hym8563_dt_idtable[] = {
    { .compatible = "haoyu,hym8563" },
    {},
    };
    MODULE_DEVICE_TABLE(of, hym8563_dt_idtable);
    static struct i2c_driver hym8563_driver = {
    .driver		= {
    .name	= "rtc-hym8563",
    .pm	= &hym8563_pm_ops,
    .of_match_table	= hym8563_dt_idtable,
    },
    .probe		= hym8563_probe,
    .id_table	= hym8563_id,
    };
    module_i2c_driver(hym8563_driver);
    MODULE_AUTHOR("Heiko Stuebner <heiko@sntech.de>");
    MODULE_DESCRIPTION("HYM8563 RTC driver");
    MODULE_LICENSE("GPL");
