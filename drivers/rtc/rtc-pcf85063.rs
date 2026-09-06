//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pcf85063.c
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


// SPDX-License-Identifier: GPL-2.0
//
// An I2C driver for the PCF85063 RTC
// Copyright 2014 Rose Technology
//
// Author: Søren Andersen <san@rosetechnology.dk>
// Maintainers: http://www.nslu2-linux.org
//
// Copyright (C) 2019 Micro Crystal AG
// Author: Alexandre Belloni <alexandre.belloni@bootlin.com>
//

//
// Information for this driver was pulled from the following datasheets.
//
// https://www.nxp.com/docs/en/data-sheet/PCF85063A.pdf
// https://www.nxp.com/docs/en/data-sheet/PCF85063TP.pdf
//
// PCF85063A -- Rev. 7 — 30 March 2018
// PCF85063TP -- Rev. 4 — 6 May 2015
//
// https://www.microcrystal.com/fileadmin/Media/Products/RTC/App.Manual/RV-8263-C7_App-Manual.pdf
// RV8263 -- Rev. 1.0 — January 2019
//
// https://www.microcrystal.com/fileadmin/Media/Products/RTC/App.Manual/RV-8063-C7_App-Manual.pdf
// RV8063 -- Rev. 1.1 - October 2018
//
pub const PCF85063_REG_CTRL1: c_uint = 0x00 /* status */;

pub const PCF85063_REG_CTRL1_SWR: c_uint = 0x58;
pub const PCF85063_REG_CTRL2: c_uint = 0x01;

pub const PCF85063_REG_OFFSET: c_uint = 0x02;

pub const PCF85063_OFFSET_STEP0: c_int = 4340;
pub const PCF85063_OFFSET_STEP1: c_int = 4069;
pub const PCF85063_REG_CLKO_F_MASK: c_uint = 0x07 /* frequency mask */;
pub const PCF85063_REG_CLKO_F_32768HZ: c_uint = 0x00;
pub const PCF85063_REG_CLKO_F_OFF: c_uint = 0x07;
pub const PCF85063_REG_RAM: c_uint = 0x03;
pub const PCF85063_REG_SC: c_uint = 0x04 /* datetime */;
pub const PCF85063_REG_SC_OS: c_uint = 0x80;
pub const PCF85063_REG_ALM_S: c_uint = 0x0b;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf85063_config {
    pub regmap: regmap_config,
    pub has_alarms:1: unsigned,
    pub force_cap_7000:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf85063 {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,

    pub clkout_hw: clk_hw,

}

#[no_mangle]
unsafe extern "C" fn pcf85063_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf85063_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    int rc;
    u8 regs[7];
//
// while reading, the time/date registers are blocked and not updated
// anymore until the access is finished. To not lose a second
// event, the access must be finished within one second. So, read all
// time/date registers in one turn.
//
    rc = regmap_bulk_read(pcf85063.regmap, PCF85063_REG_SC, regs,
    sizeof(regs));
    if (rc)
    return rc;
// if the clock has lost its power it makes no sense to use its time
    if (regs[0] & PCF85063_REG_SC_OS) {
    dev_warn(&pcf85063.rtc.dev, "Power loss detected, invalid time\n");
    return -EINVAL;
    }
    tm.tm_sec = bcd2bin(regs[0] & 0x7F);
    tm.tm_min = bcd2bin(regs[1] & 0x7F);
    tm.tm_hour = bcd2bin(regs[2] & 0x3F); /* rtc hr 0-23 */
    tm.tm_mday = bcd2bin(regs[3] & 0x3F);
    tm.tm_wday = regs[4] & 0x07;
    tm.tm_mon = bcd2bin(regs[5] & 0x1F) - 1; /* rtc mn 1-12 */
    tm.tm_year = bcd2bin(regs[6]);
    tm.tm_year += 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf85063_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    int rc;
    u8 regs[7];
//
// to accurately set the time, reset the divider chain and keep it in
// reset state until all time/date registers are written
//
    rc = regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL1,
    PCF85063_REG_CTRL1_EXT_TEST |
    PCF85063_REG_CTRL1_STOP,
    PCF85063_REG_CTRL1_STOP);
    if (rc)
    return rc;
// hours, minutes and seconds
    regs[0] = bin2bcd(tm.tm_sec) & 0x7F; /* clear OS flag */
    regs[1] = bin2bcd(tm.tm_min);
    regs[2] = bin2bcd(tm.tm_hour);
// Day of month, 1 - 31
    regs[3] = bin2bcd(tm.tm_mday);
// Day, 0 - 6
    regs[4] = tm.tm_wday & 0x07;
// month, 1 - 12
    regs[5] = bin2bcd(tm.tm_mon + 1);
// year and century
    regs[6] = bin2bcd(tm.tm_year - 100);
// write all registers at once
    rc = regmap_bulk_write(pcf85063.regmap, PCF85063_REG_SC,
    regs, sizeof(regs));
    if (rc)
    return rc;
//
// Write the control register as a separate action since the size of
// the register space is different between the PCF85063TP and
// PCF85063A devices.  The rollover point can not be used.
//
    return regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL1,
    PCF85063_REG_CTRL1_STOP, 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pcf85063_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    u8 buf[4];
    unsigned int val;
    int ret;
    ret = regmap_bulk_read(pcf85063.regmap, PCF85063_REG_ALM_S,
    buf, sizeof(buf));
    if (ret)
    return ret;
    alrm.time.tm_sec = bcd2bin(buf[0] & 0x7f);
    alrm.time.tm_min = bcd2bin(buf[1] & 0x7f);
    alrm.time.tm_hour = bcd2bin(buf[2] & 0x3f);
    alrm.time.tm_mday = bcd2bin(buf[3] & 0x3f);
    ret = regmap_read(pcf85063.regmap, PCF85063_REG_CTRL2, &val);
    if (ret)
    return ret;
    alrm.enabled =  !!(val & PCF85063_CTRL2_AIE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pcf85063_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    u8 buf[5];
    int ret;
    buf[0] = bin2bcd(alrm.time.tm_sec);
    buf[1] = bin2bcd(alrm.time.tm_min);
    buf[2] = bin2bcd(alrm.time.tm_hour);
    buf[3] = bin2bcd(alrm.time.tm_mday);
    buf[4] = PCF85063_AEN; /* Do not match on week day */
    ret = regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL2,
    PCF85063_CTRL2_AIE | PCF85063_CTRL2_AF, 0);
    if (ret)
    return ret;
    ret = regmap_bulk_write(pcf85063.regmap, PCF85063_REG_ALM_S,
    buf, sizeof(buf));
    if (ret)
    return ret;
    return regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL2,
    PCF85063_CTRL2_AIE | PCF85063_CTRL2_AF,
    alrm.enabled ? PCF85063_CTRL2_AIE | PCF85063_CTRL2_AF : PCF85063_CTRL2_AF);
    }
    static int pcf85063_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    return regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL2,
    PCF85063_CTRL2_AIE,
    enabled ? PCF85063_CTRL2_AIE : 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_rtc_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcf85063_rtc_handle_irq(int irq, void *dev_id)
    {
    struct pcf85063 *pcf85063 = dev_id;
    unsigned int val;
    int err;
    err = regmap_read(pcf85063.regmap, PCF85063_REG_CTRL2, &val);
    if (err)
    return IRQ_NONE;
    if (val & PCF85063_CTRL2_AF) {
    rtc_update_irq(pcf85063.rtc, 1, RTC_IRQF | RTC_AF);
    regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL2,
    PCF85063_CTRL2_AIE | PCF85063_CTRL2_AF,
    0);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_read_offset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int pcf85063_read_offset(struct device *dev, long *offset)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    long val;
    u32 reg;
    int ret;
    ret = regmap_read(pcf85063.regmap, PCF85063_REG_OFFSET, &reg);
    if (ret < 0)
    return ret;
    val = sign_extend32(reg & ~PCF85063_OFFSET_MODE,
    PCF85063_OFFSET_SIGN_BIT);
    if (reg & PCF85063_OFFSET_MODE)
// offset = val * PCF85063_OFFSET_STEP1;
    else
// offset = val * PCF85063_OFFSET_STEP0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_set_offset(dev: *mut device, offset: c_long) -> c_int {
    static int pcf85063_set_offset(struct device *dev, long offset)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    s8 mode0, mode1, reg;
    unsigned int error0, error1;
    if (offset > PCF85063_OFFSET_STEP0 * 63)
    return -ERANGE;
    if (offset < PCF85063_OFFSET_STEP0 * -64)
    return -ERANGE;
    mode0 = DIV_ROUND_CLOSEST(offset, PCF85063_OFFSET_STEP0);
    mode1 = DIV_ROUND_CLOSEST(offset, PCF85063_OFFSET_STEP1);
    error0 = abs(offset - (mode0 * PCF85063_OFFSET_STEP0));
    error1 = abs(offset - (mode1 * PCF85063_OFFSET_STEP1));
    if (mode1 > 63 || mode1 < -64 || error0 < error1)
    reg = mode0 & ~PCF85063_OFFSET_MODE;
    else
    reg = mode1 | PCF85063_OFFSET_MODE;
    return regmap_write(pcf85063.regmap, PCF85063_REG_OFFSET, reg);
    }
    static int pcf85063_ioctl(struct device *dev, unsigned int cmd,
    unsigned long arg)
    {
    struct pcf85063 *pcf85063 = dev_get_drvdata(dev);
    int status, ret = 0;
    switch (cmd) {
    case RTC_VL_READ:
    ret = regmap_read(pcf85063.regmap, PCF85063_REG_SC, &status);
    if (ret < 0)
    return ret;
    status = (status & PCF85063_REG_SC_OS) ? RTC_VL_DATA_INVALID : 0;
    return put_user(status, (unsigned int __user *)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct rtc_class_ops pcf85063_rtc_ops = {
    .read_time	= pcf85063_rtc_read_time,
    .set_time	= pcf85063_rtc_set_time,
    .read_offset	= pcf85063_read_offset,
    .set_offset	= pcf85063_set_offset,
    .read_alarm	= pcf85063_rtc_read_alarm,
    .set_alarm	= pcf85063_rtc_set_alarm,
    .alarm_irq_enable = pcf85063_rtc_alarm_irq_enable,
    .ioctl		= pcf85063_ioctl,
    };
    static int pcf85063_nvmem_read(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    unsigned int tmp;
    int ret;
    ret = regmap_read(priv, PCF85063_REG_RAM, &tmp);
    if (ret < 0)
    return ret;
// (u8 *)val = tmp;
    return 0;
    }
    static int pcf85063_nvmem_write(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    return regmap_write(priv, PCF85063_REG_RAM, *(u8 *)val);
    }
    static int pcf85063_load_capacitance(struct pcf85063 *pcf85063,
    const struct device_node *np,
    unsigned int force_cap)
    {
    let mut load: u32 = 7000;
    let mut reg: u8 = 0;
    if (force_cap)
    load = force_cap;
    else
    of_property_read_u32(np, "quartz-load-femtofarads", &load);
    switch (load) {
    default:
    dev_warn(&pcf85063.rtc.dev, "Unknown quartz-load-femtofarads value: %d. Assuming 7000",
    load);
    fallthrough;
    case 7000:
    break;
    case 12500:
    reg = PCF85063_REG_CTRL1_CAP_SEL;
    break;
    }
    return regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL1,
    PCF85063_REG_CTRL1_CAP_SEL, reg);
    }

//
// Handling of the clkout
//

    static int clkout_rates[] = {
    32768,
    16384,
    8192,
    4096,
    2048,
    1024,
    1,
    0
    };
    static unsigned long pcf85063_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct pcf85063 *pcf85063 = clkout_hw_to_pcf85063(hw);
    unsigned int buf;
    let mut ret: c_int = regmap_read(pcf85063.regmap, PCF85063_REG_CTRL2, &buf);
    if (ret < 0)
    return 0;
    buf &= PCF85063_REG_CLKO_F_MASK;
    return clkout_rates[buf];
    }
    static int pcf85063_clkout_determine_rate(struct clk_hw *hw,
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
    static int pcf85063_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct pcf85063 *pcf85063 = clkout_hw_to_pcf85063(hw);
    int i;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] == rate)
    return regmap_update_bits(pcf85063.regmap,
    PCF85063_REG_CTRL2,
    PCF85063_REG_CLKO_F_MASK, i);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_clkout_control(hw: *mut clk_hw, enable: bool) -> c_int {
    static int pcf85063_clkout_control(struct clk_hw *hw, bool enable)
    {
    struct pcf85063 *pcf85063 = clkout_hw_to_pcf85063(hw);
    unsigned int buf;
    int ret;
    ret = regmap_read(pcf85063.regmap, PCF85063_REG_CTRL2, &buf);
    if (ret < 0)
    return ret;
    buf &= PCF85063_REG_CLKO_F_MASK;
    if (enable) {
    if (buf == PCF85063_REG_CLKO_F_OFF)
    buf = PCF85063_REG_CLKO_F_32768HZ;
    else
    return 0;
    } else {
    if (buf != PCF85063_REG_CLKO_F_OFF)
    buf = PCF85063_REG_CLKO_F_OFF;
    else
    return 0;
    }
    return regmap_update_bits(pcf85063.regmap, PCF85063_REG_CTRL2,
    PCF85063_REG_CLKO_F_MASK, buf);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int pcf85063_clkout_prepare(struct clk_hw *hw)
    {
    return pcf85063_clkout_control(hw, 1);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_clkout_unprepare(hw: *mut clk_hw) {
    static void pcf85063_clkout_unprepare(struct clk_hw *hw)
    {
    pcf85063_clkout_control(hw, 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_clkout_is_prepared(hw: *mut clk_hw) -> c_int {
    static int pcf85063_clkout_is_prepared(struct clk_hw *hw)
    {
    struct pcf85063 *pcf85063 = clkout_hw_to_pcf85063(hw);
    unsigned int buf;
    let mut ret: c_int = regmap_read(pcf85063.regmap, PCF85063_REG_CTRL2, &buf);
    if (ret < 0)
    return 0;
    return (buf & PCF85063_REG_CLKO_F_MASK) != PCF85063_REG_CLKO_F_OFF;
    }
    static const struct clk_ops pcf85063_clkout_ops = {
    .prepare = pcf85063_clkout_prepare,
    .unprepare = pcf85063_clkout_unprepare,
    .is_prepared = pcf85063_clkout_is_prepared,
    .recalc_rate = pcf85063_clkout_recalc_rate,
    .determine_rate = pcf85063_clkout_determine_rate,
    .set_rate = pcf85063_clkout_set_rate,
    };
    static struct clk *pcf85063_clkout_register_clk(struct pcf85063 *pcf85063)
    {
    struct clk *clk;
    let mut init: clk_init_data = {};
    struct device_node *node = pcf85063.rtc.dev.parent.of_node;
    struct device_node *fixed_clock;
    fixed_clock = of_get_child_by_name(node, "clock");
    if (fixed_clock) {
//
// skip registering square wave clock when a fixed
// clock has been registered. The fixed clock is
// registered automatically when being referenced.
//
    of_node_put(fixed_clock);
    return core::ptr::null_mut();
    }
    init.name = "pcf85063-clkout";
    init.ops = &pcf85063_clkout_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    pcf85063.clkout_hw.init = &init;
// optional override of the clockname
    of_property_read_string(node, "clock-output-names", &init.name);
// register the clock
    clk = devm_clk_register(&pcf85063.rtc.dev, &pcf85063.clkout_hw);
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return clk;
    }

    static int pcf85063_probe(struct device *dev, struct regmap *regmap, int irq,
    const struct pcf85063_config *config)
    {
    struct pcf85063 *pcf85063;
    unsigned int tmp;
    int err;
    struct nvmem_config nvmem_cfg = {
    .name = "pcf85063_nvram",
    .reg_read = pcf85063_nvmem_read,
    .reg_write = pcf85063_nvmem_write,
    .type = NVMEM_TYPE_BATTERY_BACKED,
    .size = 1,
    };
    dev_dbg(dev, "%s\n", __func__);
    pcf85063 = devm_kzalloc(dev, sizeof(struct pcf85063),
    GFP_KERNEL);
    if (!pcf85063)
    return -ENOMEM;
    pcf85063.regmap = regmap;
    dev_set_drvdata(dev, pcf85063);
    err = regmap_read(pcf85063.regmap, PCF85063_REG_SC, &tmp);
    if (err)
    return dev_err_probe(dev, err, "RTC chip is not present\n");
    pcf85063.rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(pcf85063.rtc))
    return PTR_ERR(pcf85063.rtc);
//
// If a Power loss is detected, SW reset the device.
// From PCF85063A datasheet:
// There is a low probability that some devices will have corruption
// of the registers after the automatic power-on reset...
//
    if (tmp & PCF85063_REG_SC_OS) {
    dev_warn(dev, "POR issue detected, sending a SW reset\n");
    err = regmap_write(pcf85063.regmap, PCF85063_REG_CTRL1,
    PCF85063_REG_CTRL1_SWR);
    if (err < 0)
    dev_warn(dev, "SW reset failed, trying to continue\n");
    }
    err = pcf85063_load_capacitance(pcf85063, dev.of_node,
    config.force_cap_7000 ? 7000 : 0);
    if (err < 0)
    dev_warn(dev, "failed to set xtal load capacitance: %d",
    err);
    pcf85063.rtc.ops = &pcf85063_rtc_ops;
    pcf85063.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    pcf85063.rtc.range_max = RTC_TIMESTAMP_END_2099;
    set_bit(RTC_FEATURE_ALARM_RES_2S, pcf85063.rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, pcf85063.rtc.features);
    clear_bit(RTC_FEATURE_ALARM, pcf85063.rtc.features);
    if (config.has_alarms && irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(dev))
    irqflags = 0;
    err = devm_request_threaded_irq(dev, irq,
    core::ptr::null_mut(), pcf85063_rtc_handle_irq,
    irqflags | IRQF_ONESHOT,
    "pcf85063", pcf85063);
    if (err) {
    dev_warn(&pcf85063.rtc.dev,
    "unable to request IRQ, alarms disabled\n");
    } else {
    set_bit(RTC_FEATURE_ALARM, pcf85063.rtc.features);
    device_init_wakeup(dev, true);
    err = dev_pm_set_wake_irq(dev, irq);
    if (err)
    dev_err(&pcf85063.rtc.dev,
    "failed to enable irq wake\n");
    }
    }
    nvmem_cfg.priv = pcf85063.regmap;
    devm_rtc_nvmem_register(pcf85063.rtc, &nvmem_cfg);

// register clk in common clk framework
    pcf85063_clkout_register_clk(pcf85063);

    return devm_rtc_register_device(pcf85063.rtc);
    }

    static const struct pcf85063_config config_pcf85063 = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x0a,
    },
    };
    static const struct pcf85063_config config_pcf85063tp = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x0a,
    },
    };
    static const struct pcf85063_config config_pcf85063a = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x11,
    },
    .has_alarms = 1,
    };
    static const struct pcf85063_config config_rv8263 = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x11,
    },
    .has_alarms = 1,
    .force_cap_7000 = 1,
    };
    static const struct i2c_device_id pcf85063_ids[] = {
    { .name = "pca85073a", .driver_data = (kernel_ulong_t)&config_pcf85063a },
    { .name = "pcf85063", .driver_data = (kernel_ulong_t)&config_pcf85063 },
    { .name = "pcf85063tp", .driver_data = (kernel_ulong_t)&config_pcf85063tp },
    { .name = "pcf85063a", .driver_data = (kernel_ulong_t)&config_pcf85063a },
    { .name = "rv8263", .driver_data = (kernel_ulong_t)&config_rv8263 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcf85063_ids);

    static const struct of_device_id pcf85063_of_match[] = {
    { .compatible = "nxp,pca85073a", .data = &config_pcf85063a },
    { .compatible = "nxp,pcf85063", .data = &config_pcf85063 },
    { .compatible = "nxp,pcf85063tp", .data = &config_pcf85063tp },
    { .compatible = "nxp,pcf85063a", .data = &config_pcf85063a },
    { .compatible = "microcrystal,rv8263", .data = &config_rv8263 },
    {}
    };
    MODULE_DEVICE_TABLE(of, pcf85063_of_match);

#[no_mangle]
unsafe extern "C" fn pcf85063_i2c_probe(client: *mut i2c_client) -> c_int {
    static int pcf85063_i2c_probe(struct i2c_client *client)
    {
    const struct pcf85063_config *config;
    struct regmap *regmap;
    config = i2c_get_match_data(client);
    if (!config)
    return -ENODEV;
    regmap = devm_regmap_init_i2c(client, &config.regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcf85063_probe(&client.dev, regmap, client.irq, config);
    }
    static struct i2c_driver pcf85063_driver = {
    .driver		= {
    .name	= "rtc-pcf85063",
    .of_match_table = of_match_ptr(pcf85063_of_match),
    },
    .probe		= pcf85063_i2c_probe,
    .id_table	= pcf85063_ids,
    };
#[no_mangle]
unsafe extern "C" fn pcf85063_register_driver() -> c_int {
    static int pcf85063_register_driver(void)
    {
    return i2c_add_driver(&pcf85063_driver);
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_unregister_driver() {
    static void pcf85063_unregister_driver(void)
    {
    i2c_del_driver(&pcf85063_driver);
    }

#[no_mangle]
unsafe extern "C" fn pcf85063_register_driver() -> c_int {
    static int pcf85063_register_driver(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf85063_unregister_driver() {
    static void pcf85063_unregister_driver(void)
    {
    }

    static const struct pcf85063_config config_rv8063 = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x11,
    .read_flag_mask = BIT(7) | BIT(5),
    .write_flag_mask = BIT(5),
    },
    .has_alarms = 1,
    .force_cap_7000 = 1,
    };
    static const struct spi_device_id rv8063_id[] = {
    { "rv8063" },
    {}
    };
    MODULE_DEVICE_TABLE(spi, rv8063_id);
    static const struct of_device_id rv8063_of_match[] = {
    { .compatible = "microcrystal,rv8063" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rv8063_of_match);
#[no_mangle]
unsafe extern "C" fn rv8063_probe(spi: *mut spi_device) -> c_int {
    static int rv8063_probe(struct spi_device *spi)
    {
    const struct pcf85063_config *config = &config_rv8063;
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &config.regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcf85063_probe(&spi.dev, regmap, spi.irq, config);
    }
    static struct spi_driver rv8063_driver = {
    .driver         = {
    .name   = "rv8063",
    .of_match_table = rv8063_of_match,
    },
    .probe          = rv8063_probe,
    .id_table	= rv8063_id,
    };
#[no_mangle]
unsafe extern "C" fn rv8063_register_driver() -> int __init {
    static int __init rv8063_register_driver(void)
    {
    return spi_register_driver(&rv8063_driver);
    }
#[no_mangle]
unsafe extern "C" fn rv8063_unregister_driver() -> void __exit {
    static void __exit rv8063_unregister_driver(void)
    {
    spi_unregister_driver(&rv8063_driver);
    }

#[no_mangle]
unsafe extern "C" fn rv8063_register_driver() -> int __init {
    static int __init rv8063_register_driver(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv8063_unregister_driver() -> void __exit {
    static void __exit rv8063_unregister_driver(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn pcf85063_init() -> int __init {
    static int __init pcf85063_init(void)
    {
    int ret;
    ret = pcf85063_register_driver();
    if (ret)
    return ret;
    ret = rv8063_register_driver();
    if (ret)
    pcf85063_unregister_driver();
    return ret;
    }
    module_init(pcf85063_init);
#[no_mangle]
unsafe extern "C" fn pcf85063_exit() -> void __exit {
    static void __exit pcf85063_exit(void)
    {
    rv8063_unregister_driver();
    pcf85063_unregister_driver();
    }
    module_exit(pcf85063_exit);
    MODULE_AUTHOR("Søren Andersen <san@rosetechnology.dk>");
    MODULE_DESCRIPTION("PCF85063 RTC driver");
    MODULE_LICENSE("GPL");
