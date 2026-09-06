//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pcf8563.c
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
// An I2C driver for the Philips PCF8563 RTC
// Copyright 2005-06 Tower Technologies
//
// Author: Alessandro Zummo <a.zummo@towertech.it>
// Maintainers: http://www.nslu2-linux.org
//
// based on the other drivers in this same directory.
//
// https://www.nxp.com/docs/en/data-sheet/PCF8563.pdf
//

pub const PCF8563_REG_ST1: c_uint = 0x00 /* status */;
pub const PCF8563_REG_ST2: c_uint = 0x01;

pub const PCF8563_REG_SC: c_uint = 0x02 /* datetime */;
pub const PCF8563_REG_MN: c_uint = 0x03;
pub const PCF8563_REG_HR: c_uint = 0x04;
pub const PCF8563_REG_DM: c_uint = 0x05;
pub const PCF8563_REG_DW: c_uint = 0x06;
pub const PCF8563_REG_MO: c_uint = 0x07;
pub const PCF8563_REG_YR: c_uint = 0x08;
pub const PCF8563_REG_AMN: c_uint = 0x09 /* alarm */;
pub const PCF8563_REG_CLKO: c_uint = 0x0D /* clock out */;
pub const PCF8563_REG_CLKO_FE: c_uint = 0x80 /* clock out enabled */;
pub const PCF8563_REG_CLKO_F_MASK: c_uint = 0x03 /* frequenc mask */;
pub const PCF8563_REG_CLKO_F_32768HZ: c_uint = 0x00;
pub const PCF8563_REG_CLKO_F_1024HZ: c_uint = 0x01;
pub const PCF8563_REG_CLKO_F_32HZ: c_uint = 0x02;
pub const PCF8563_REG_CLKO_F_1HZ: c_uint = 0x03;
pub const PCF8563_REG_TMRC: c_uint = 0x0E /* timer control */;

pub const PCF8563_TMRC_4096: c_int = 0;
pub const PCF8563_TMRC_64: c_int = 1;
pub const PCF8563_TMRC_1: c_int = 2;
pub const PCF8563_TMRC_1_60: c_int = 3;
pub const PCF8563_TMRC_MASK: c_int = 3;
pub const PCF8563_REG_TMR: c_uint = 0x0F /* timer */;
pub const PCF8563_SC_LV: c_uint = 0x80 /* low voltage */;
pub const PCF8563_MO_C: c_uint = 0x80 /* century */;
    static struct i2c_driver pcf8563_driver;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf8563 {
    pub rtc: *mut rtc_device,
//
// The meaning of MO_C bit varies by the chip type.
// From PCF8563 datasheet: this bit is toggled when the years
// register overflows from 99 to 00
// 0 indicates the century is 20xx
// 1 indicates the century is 19xx
// From RTC8564 datasheet: this bit indicates change of
// century. When the year digit data overflows from 99 to 00,
// this bit is set. By presetting it to 0 while still in the
// 20th century, it will be set in year 2000, ...
// There seems no reliable way to know how the system use this
// bit.  So let's do it heuristically, assuming we are live in
// 1970...2069.
//
    pub /: *mut *mut int c_polarity; / 0: MO_C=1 means 19xx, otherwise MO_C=1 means 20xx,
    pub regmap: *mut regmap,

    pub clkout_hw: clk_hw,

}

#[no_mangle]
unsafe extern "C" fn pcf8563_set_alarm_mode(pcf8563: *mut pcf8563, on: bool) -> c_int {
    static int pcf8563_set_alarm_mode(struct pcf8563 *pcf8563, bool on)
    {
    u32 buf;
    int err;
    err = regmap_read(pcf8563.regmap, PCF8563_REG_ST2, &buf);
    if (err < 0)
    return err;
    if (on)
    buf |= PCF8563_BIT_AIE;
    else
    buf &= ~PCF8563_BIT_AIE;
    buf &= ~(PCF8563_BIT_AF | PCF8563_BITS_ST2_N);
    return regmap_write(pcf8563.regmap, PCF8563_REG_ST2, buf);
    }
    static int pcf8563_get_alarm_mode(struct pcf8563 *pcf8563, unsigned char *en,
    unsigned char *pen)
    {
    u32 buf;
    int err;
    err = regmap_read(pcf8563.regmap, PCF8563_REG_ST2, &buf);
    if (err < 0)
    return err;
    if (en)
// en = !!(buf & PCF8563_BIT_AIE);
    if (pen)
// pen = !!(buf & PCF8563_BIT_AF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcf8563_irq(int irq, void *dev_id)
    {
    struct pcf8563 *pcf8563 = dev_id;
    char pending;
    int err;
    err = pcf8563_get_alarm_mode(pcf8563, core::ptr::null_mut(), &pending);
    if (err)
    return IRQ_NONE;
    if (pending) {
    rtc_update_irq(pcf8563.rtc, 1, RTC_IRQF | RTC_AF);
    pcf8563_set_alarm_mode(pcf8563, 1);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
//
// In the routines that deal directly with the pcf8563 hardware, we use
// rtc_time -- month 0-11, hour 0-23, yr = calendar year-epoch.
//
#[no_mangle]
unsafe extern "C" fn pcf8563_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf8563_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    unsigned char buf[9];
    int err;
    err = regmap_bulk_read(pcf8563.regmap, PCF8563_REG_ST1, buf,
    sizeof(buf));
    if (err < 0)
    return err;
    if (buf[PCF8563_REG_SC] & PCF8563_SC_LV) {
    dev_err(dev,
    "low voltage detected, date/time is not reliable.\n");
    return -EINVAL;
    }
    dev_dbg(dev,
    "%s: raw data is st1=%02x, st2=%02x, sec=%02x, min=%02x, hr=%02x, "
    "mday=%02x, wday=%02x, mon=%02x, year=%02x\n",
    __func__,
    buf[0], buf[1], buf[2], buf[3],
    buf[4], buf[5], buf[6], buf[7],
    buf[8]);
    tm.tm_sec = bcd2bin(buf[PCF8563_REG_SC] & 0x7F);
    tm.tm_min = bcd2bin(buf[PCF8563_REG_MN] & 0x7F);
    tm.tm_hour = bcd2bin(buf[PCF8563_REG_HR] & 0x3F); /* rtc hr 0-23 */
    tm.tm_mday = bcd2bin(buf[PCF8563_REG_DM] & 0x3F);
    tm.tm_wday = buf[PCF8563_REG_DW] & 0x07;
    tm.tm_mon = bcd2bin(buf[PCF8563_REG_MO] & 0x1F) - 1; /* rtc mn 1-12 */
    tm.tm_year = bcd2bin(buf[PCF8563_REG_YR]) + 100;
// detect the polarity heuristically. see note above.
    pcf8563.c_polarity = (buf[PCF8563_REG_MO] & PCF8563_MO_C) ?
    (tm.tm_year >= 100) : (tm.tm_year < 100);
    dev_dbg(dev, "%s: tm is secs=%d, mins=%d, hours=%d, "
    "mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__,
    tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon, tm.tm_year, tm.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf8563_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    unsigned char buf[9];
    dev_dbg(dev, "%s: secs=%d, mins=%d, hours=%d, "
    "mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__,
    tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon, tm.tm_year, tm.tm_wday);
// hours, minutes and seconds
    buf[PCF8563_REG_SC] = bin2bcd(tm.tm_sec);
    buf[PCF8563_REG_MN] = bin2bcd(tm.tm_min);
    buf[PCF8563_REG_HR] = bin2bcd(tm.tm_hour);
    buf[PCF8563_REG_DM] = bin2bcd(tm.tm_mday);
// month, 1 - 12
    buf[PCF8563_REG_MO] = bin2bcd(tm.tm_mon + 1);
// year and century
    buf[PCF8563_REG_YR] = bin2bcd(tm.tm_year - 100);
    if (pcf8563.c_polarity ? (tm.tm_year >= 100) : (tm.tm_year < 100))
    buf[PCF8563_REG_MO] |= PCF8563_MO_C;
    buf[PCF8563_REG_DW] = tm.tm_wday & 0x07;
    return regmap_bulk_write(pcf8563.regmap, PCF8563_REG_SC,
    buf + PCF8563_REG_SC,
    sizeof(buf) - PCF8563_REG_SC);
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_rtc_ioctl(dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    static int pcf8563_rtc_ioctl(struct device *dev, unsigned int cmd, unsigned long arg)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    int ret;
    switch (cmd) {
    case RTC_VL_READ:
    ret = regmap_test_bits(pcf8563.regmap, PCF8563_REG_SC,
    PCF8563_SC_LV);
    if (ret < 0)
    return ret;
    return put_user(ret ? RTC_VL_DATA_INVALID : 0,
    (unsigned int __user *)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_rtc_read_alarm(dev: *mut device, tm: *mut rtc_wkalrm) -> c_int {
    static int pcf8563_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *tm)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    unsigned char buf[4];
    int err;
    err = regmap_bulk_read(pcf8563.regmap, PCF8563_REG_AMN, buf,
    sizeof(buf));
    if (err < 0)
    return err;
    dev_dbg(dev,
    "%s: raw data is min=%02x, hr=%02x, mday=%02x, wday=%02x\n",
    __func__, buf[0], buf[1], buf[2], buf[3]);
    tm.time.tm_sec = 0;
    tm.time.tm_min = bcd2bin(buf[0] & 0x7F);
    tm.time.tm_hour = bcd2bin(buf[1] & 0x3F);
    tm.time.tm_mday = bcd2bin(buf[2] & 0x3F);
    tm.time.tm_wday = bcd2bin(buf[3] & 0x7);
    err = pcf8563_get_alarm_mode(pcf8563, &tm.enabled, &tm.pending);
    if (err < 0)
    return err;
    dev_dbg(dev, "%s: tm is mins=%d, hours=%d, mday=%d, wday=%d,"
    " enabled=%d, pending=%d\n", __func__, tm.time.tm_min,
    tm.time.tm_hour, tm.time.tm_mday, tm.time.tm_wday,
    tm.enabled, tm.pending);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_rtc_set_alarm(dev: *mut device, tm: *mut rtc_wkalrm) -> c_int {
    static int pcf8563_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *tm)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    unsigned char buf[4];
    int err;
    buf[0] = bin2bcd(tm.time.tm_min);
    buf[1] = bin2bcd(tm.time.tm_hour);
    buf[2] = bin2bcd(tm.time.tm_mday);
    buf[3] = tm.time.tm_wday & 0x07;
    err = regmap_bulk_write(pcf8563.regmap, PCF8563_REG_AMN, buf,
    sizeof(buf));
    if (err)
    return err;
    return pcf8563_set_alarm_mode(pcf8563, !!tm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int pcf8563_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct pcf8563 *pcf8563 = dev_get_drvdata(dev);
    dev_dbg(dev, "%s: en=%d\n", __func__, enabled);
    return pcf8563_set_alarm_mode(pcf8563, !!enabled);
    }

//
// Handling of the clkout
//

    static const int clkout_rates[] = {
    32768,
    1024,
    32,
    1,
    };
    static unsigned long pcf8563_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct pcf8563 *pcf8563 = clkout_hw_to_pcf8563(hw);
    u32 buf;
    int ret;
    ret = regmap_read(pcf8563.regmap, PCF8563_REG_CLKO, &buf);
    if (ret < 0)
    return 0;
    buf &= PCF8563_REG_CLKO_F_MASK;
    return clkout_rates[buf];
    }
    static int pcf8563_clkout_determine_rate(struct clk_hw *hw,
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
    static int pcf8563_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct pcf8563 *pcf8563 = clkout_hw_to_pcf8563(hw);
    int i, ret;
    u32 buf;
    ret = regmap_read(pcf8563.regmap, PCF8563_REG_CLKO, &buf);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] == rate) {
    buf &= ~PCF8563_REG_CLKO_F_MASK;
    buf |= i;
    return regmap_update_bits(pcf8563.regmap,
    PCF8563_REG_CLKO,
    PCF8563_REG_CLKO_F_MASK,
    buf);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_clkout_control(hw: *mut clk_hw, enable: bool) -> c_int {
    static int pcf8563_clkout_control(struct clk_hw *hw, bool enable)
    {
    struct pcf8563 *pcf8563 = clkout_hw_to_pcf8563(hw);
    u32 buf;
    int ret;
    ret = regmap_read(pcf8563.regmap, PCF8563_REG_CLKO, &buf);
    if (ret < 0)
    return ret;
    if (enable)
    buf |= PCF8563_REG_CLKO_FE;
    else
    buf &= ~PCF8563_REG_CLKO_FE;
    return regmap_update_bits(pcf8563.regmap, PCF8563_REG_CLKO,
    PCF8563_REG_CLKO_FE, buf);
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int pcf8563_clkout_prepare(struct clk_hw *hw)
    {
    return pcf8563_clkout_control(hw, 1);
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_clkout_unprepare(hw: *mut clk_hw) {
    static void pcf8563_clkout_unprepare(struct clk_hw *hw)
    {
    pcf8563_clkout_control(hw, 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf8563_clkout_is_prepared(hw: *mut clk_hw) -> c_int {
    static int pcf8563_clkout_is_prepared(struct clk_hw *hw)
    {
    struct pcf8563 *pcf8563 = clkout_hw_to_pcf8563(hw);
    u32 buf;
    int ret;
    ret = regmap_read(pcf8563.regmap, PCF8563_REG_CLKO, &buf);
    if (ret < 0)
    return ret;
    return !!(buf & PCF8563_REG_CLKO_FE);
    }
    static const struct clk_ops pcf8563_clkout_ops = {
    .prepare = pcf8563_clkout_prepare,
    .unprepare = pcf8563_clkout_unprepare,
    .is_prepared = pcf8563_clkout_is_prepared,
    .recalc_rate = pcf8563_clkout_recalc_rate,
    .determine_rate = pcf8563_clkout_determine_rate,
    .set_rate = pcf8563_clkout_set_rate,
    };
    static struct clk *pcf8563_clkout_register_clk(struct pcf8563 *pcf8563)
    {
    struct device_node *node = pcf8563.rtc.dev.parent.of_node;
    let mut init: clk_init_data = {};
    struct clk *clk;
    int ret;
// disable the clkout output
    ret = regmap_clear_bits(pcf8563.regmap, PCF8563_REG_CLKO,
    PCF8563_REG_CLKO_FE);
    if (ret < 0)
    return ERR_PTR(ret);
    init.name = "pcf8563-clkout";
    init.ops = &pcf8563_clkout_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    pcf8563.clkout_hw.init = &init;
// optional override of the clockname
    of_property_read_string(node, "clock-output-names", &init.name);
// register the clock
    clk = devm_clk_register(&pcf8563.rtc.dev, &pcf8563.clkout_hw);
    if (!IS_ERR(clk))
    devm_of_clk_add_hw_provider(pcf8563.rtc.dev.parent,
    of_clk_hw_simple_get,
    &pcf8563.clkout_hw);
    return clk;
    }

    static const struct rtc_class_ops pcf8563_rtc_ops = {
    .ioctl		= pcf8563_rtc_ioctl,
    .read_time	= pcf8563_rtc_read_time,
    .set_time	= pcf8563_rtc_set_time,
    .read_alarm	= pcf8563_rtc_read_alarm,
    .set_alarm	= pcf8563_rtc_set_alarm,
    .alarm_irq_enable = pcf8563_irq_enable,
    };
    static const struct regmap_config regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xF,
    };
#[no_mangle]
unsafe extern "C" fn pcf8563_probe(client: *mut i2c_client) -> c_int {
    static int pcf8563_probe(struct i2c_client *client)
    {
    struct pcf8563 *pcf8563;
    int err;
    dev_dbg(&client.dev, "%s\n", __func__);
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    pcf8563 = devm_kzalloc(&client.dev, sizeof(struct pcf8563),
    GFP_KERNEL);
    if (!pcf8563)
    return -ENOMEM;
    pcf8563.regmap = devm_regmap_init_i2c(client, &regmap_config);
    if (IS_ERR(pcf8563.regmap))
    return PTR_ERR(pcf8563.regmap);
    i2c_set_clientdata(client, pcf8563);
    device_set_wakeup_capable(&client.dev, 1);
// Set timer to lowest frequency to save power (ref Haoyu datasheet)
    err = regmap_set_bits(pcf8563.regmap, PCF8563_REG_TMRC,
    PCF8563_TMRC_1_60);
    if (err < 0) {
    dev_err(&client.dev, "%s: write error\n", __func__);
    return err;
    }
// Clear flags and disable interrupts
    err = regmap_write(pcf8563.regmap, PCF8563_REG_ST2, 0);
    if (err < 0) {
    dev_err(&client.dev, "%s: write error\n", __func__);
    return err;
    }
    pcf8563.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(pcf8563.rtc))
    return PTR_ERR(pcf8563.rtc);
    pcf8563.rtc.ops = &pcf8563_rtc_ops;
// the pcf8563 alarm only supports a minute accuracy
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, pcf8563.rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, pcf8563.rtc.features);
    clear_bit(RTC_FEATURE_ALARM, pcf8563.rtc.features);
    pcf8563.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    pcf8563.rtc.range_max = RTC_TIMESTAMP_END_2099;
    pcf8563.rtc.set_start_time = true;
    if (client.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&client.dev))
    irqflags = 0;
    err = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), pcf8563_irq,
    IRQF_SHARED | IRQF_ONESHOT | irqflags,
    pcf8563_driver.driver.name, client);
    if (err) {
    dev_err(&client.dev, "unable to request IRQ %d\n",
    client.irq);
    return err;
    }
    } else {
    client.irq = 0;
    }
    if (client.irq > 0 || device_property_read_bool(&client.dev, "wakeup-source")) {
    device_init_wakeup(&client.dev, true);
    set_bit(RTC_FEATURE_ALARM, pcf8563.rtc.features);
    }
    err = devm_rtc_register_device(pcf8563.rtc);
    if (err)
    return err;

// register clk in common clk framework
    pcf8563_clkout_register_clk(pcf8563);

    return 0;
    }
    static const struct i2c_device_id pcf8563_id[] = {
    { .name = "pcf8563" },
    { .name = "rtc8564" },
    { .name = "pca8565" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcf8563_id);

    static const struct of_device_id pcf8563_of_match[] = {
    { .compatible = "nxp,pcf8563" },
    { .compatible = "epson,rtc8564" },
    { .compatible = "microcrystal,rv8564" },
    { .compatible = "nxp,pca8565" },
    {}
    };
    MODULE_DEVICE_TABLE(of, pcf8563_of_match);

    static struct i2c_driver pcf8563_driver = {
    .driver		= {
    .name	= "rtc-pcf8563",
    .of_match_table = of_match_ptr(pcf8563_of_match),
    },
    .probe		= pcf8563_probe,
    .id_table	= pcf8563_id,
    };
    module_i2c_driver(pcf8563_driver);
    MODULE_AUTHOR("Alessandro Zummo <a.zummo@towertech.it>");
    MODULE_DESCRIPTION("Philips PCF8563/Epson RTC8564 RTC driver");
    MODULE_LICENSE("GPL");
