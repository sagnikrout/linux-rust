//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rv3032.c
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
// RTC driver for the Micro Crystal RV3032
//
// Copyright (C) 2020 Micro Crystal SA
//
// Alexandre Belloni <alexandre.belloni@bootlin.com>
//

pub const RV3032_SEC: c_uint = 0x01;
pub const RV3032_MIN: c_uint = 0x02;
pub const RV3032_HOUR: c_uint = 0x03;
pub const RV3032_WDAY: c_uint = 0x04;
pub const RV3032_DAY: c_uint = 0x05;
pub const RV3032_MONTH: c_uint = 0x06;
pub const RV3032_YEAR: c_uint = 0x07;
pub const RV3032_ALARM_MIN: c_uint = 0x08;
pub const RV3032_ALARM_HOUR: c_uint = 0x09;
pub const RV3032_ALARM_DAY: c_uint = 0x0A;
pub const RV3032_STATUS: c_uint = 0x0D;
pub const RV3032_TLSB: c_uint = 0x0E;
pub const RV3032_TMSB: c_uint = 0x0F;
pub const RV3032_CTRL1: c_uint = 0x10;
pub const RV3032_CTRL2: c_uint = 0x11;
pub const RV3032_CTRL3: c_uint = 0x12;
pub const RV3032_TS_CTRL: c_uint = 0x13;
pub const RV3032_CLK_IRQ: c_uint = 0x14;
pub const RV3032_EEPROM_ADDR: c_uint = 0x3D;
pub const RV3032_EEPROM_DATA: c_uint = 0x3E;
pub const RV3032_EEPROM_CMD: c_uint = 0x3F;
pub const RV3032_RAM1: c_uint = 0x40;
pub const RV3032_PMU: c_uint = 0xC0;
pub const RV3032_OFFSET: c_uint = 0xC1;
pub const RV3032_CLKOUT1: c_uint = 0xC2;
pub const RV3032_CLKOUT2: c_uint = 0xC3;
pub const RV3032_TREF0: c_uint = 0xC4;
pub const RV3032_TREF1: c_uint = 0xC5;

pub const RV3032_PMU_BSM_DSM: c_int = 1;
pub const RV3032_PMU_BSM_LSM: c_int = 2;

pub const RV3032_EEPROM_CMD_UPDATE: c_uint = 0x11;
pub const RV3032_EEPROM_CMD_WRITE: c_uint = 0x21;
pub const RV3032_EEPROM_CMD_READ: c_uint = 0x22;
pub const RV3032_EEPROM_USER: c_uint = 0xCB;
pub const RV3032_EEBUSY_POLL: c_int = 10000;
pub const RV3032_EEBUSY_TIMEOUT: c_int = 100000;
pub const OFFSET_STEP_PPT: c_int = 238419;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv3032_data {
    pub regmap: *mut regmap,
    pub rtc: *mut rtc_device,
    pub trickle_charger_set: bool,

    pub clkout_hw: clk_hw,

}

    static u16 rv3032_trickle_resistors[] = {1000, 2000, 7000, 11000};
    static u16 rv3032_trickle_voltages[] = {0, 1750, 3000, 4400};
#[no_mangle]
unsafe extern "C" fn rv3032_exit_eerd(rv3032: *mut rv3032_data, eerd: u32) -> c_int {
    static int rv3032_exit_eerd(struct rv3032_data *rv3032, u32 eerd)
    {
    if (eerd)
    return 0;
    return regmap_update_bits(rv3032.regmap, RV3032_CTRL1, RV3032_CTRL1_EERD, 0);
    }
#[no_mangle]
unsafe extern "C" fn rv3032_enter_eerd(rv3032: *mut rv3032_data, eerd: *mut u32) -> c_int {
    static int rv3032_enter_eerd(struct rv3032_data *rv3032, u32 *eerd)
    {
    u32 ctrl1, status;
    int ret;
    ret = regmap_read(rv3032.regmap, RV3032_CTRL1, &ctrl1);
    if (ret)
    return ret;
// eerd = ctrl1 & RV3032_CTRL1_EERD;
    if (*eerd)
    return 0;
    ret = regmap_update_bits(rv3032.regmap, RV3032_CTRL1,
    RV3032_CTRL1_EERD, RV3032_CTRL1_EERD);
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(rv3032.regmap, RV3032_TLSB, status,
    !(status & RV3032_TLSB_EEBUSY),
    RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    if (ret) {
    rv3032_exit_eerd(rv3032, *eerd);
    return ret;
    }
    return 0;
    }
    static int rv3032_update_cfg(struct rv3032_data *rv3032, unsigned int reg,
    unsigned int mask, unsigned int val)
    {
    u32 status, eerd;
    int ret;
    ret = rv3032_enter_eerd(rv3032, &eerd);
    if (ret)
    return ret;
    ret = regmap_update_bits(rv3032.regmap, reg, mask, val);
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_CMD, RV3032_EEPROM_CMD_UPDATE);
    if (ret)
    goto exit_eerd;
    usleep_range(46000, RV3032_EEBUSY_TIMEOUT);
    ret = regmap_read_poll_timeout(rv3032.regmap, RV3032_TLSB, status,
    !(status & RV3032_TLSB_EEBUSY),
    RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    exit_eerd:
    rv3032_exit_eerd(rv3032, eerd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rv3032_handle_irq(int irq, void *dev_id)
    {
    struct rv3032_data *rv3032 = dev_id;
    let mut events: c_ulong = 0;
    let mut status: u32 = 0, ctrl = 0;
    if (regmap_read(rv3032.regmap, RV3032_STATUS, &status) < 0 ||
    status == 0) {
    return IRQ_NONE;
    }
    if (status & RV3032_STATUS_TF) {
    status |= RV3032_STATUS_TF;
    ctrl |= RV3032_CTRL2_TIE;
    events |= RTC_PF;
    }
    if (status & RV3032_STATUS_AF) {
    status |= RV3032_STATUS_AF;
    ctrl |= RV3032_CTRL2_AIE;
    events |= RTC_AF;
    }
    if (status & RV3032_STATUS_UF) {
    status |= RV3032_STATUS_UF;
    ctrl |= RV3032_CTRL2_UIE;
    events |= RTC_UF;
    }
    if (events) {
    rtc_update_irq(rv3032.rtc, 1, events);
    regmap_update_bits(rv3032.regmap, RV3032_STATUS, status, 0);
    regmap_update_bits(rv3032.regmap, RV3032_CTRL2, ctrl, 0);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rv3032_get_time(struct device *dev, struct rtc_time *tm)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    u8 date[7];
    int ret, status;
    ret = regmap_read(rv3032.regmap, RV3032_STATUS, &status);
    if (ret < 0)
    return ret;
    if (status & (RV3032_STATUS_PORF | RV3032_STATUS_VLF))
    return -EINVAL;
    ret = regmap_bulk_read(rv3032.regmap, RV3032_SEC, date, sizeof(date));
    if (ret)
    return ret;
    tm.tm_sec  = bcd2bin(date[0] & 0x7f);
    tm.tm_min  = bcd2bin(date[1] & 0x7f);
    tm.tm_hour = bcd2bin(date[2] & 0x3f);
    tm.tm_wday = date[3] & 0x7;
    tm.tm_mday = bcd2bin(date[4] & 0x3f);
    tm.tm_mon  = bcd2bin(date[5] & 0x1f) - 1;
    tm.tm_year = bcd2bin(date[6]) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rv3032_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    u8 date[7];
    int ret;
    date[0] = bin2bcd(tm.tm_sec);
    date[1] = bin2bcd(tm.tm_min);
    date[2] = bin2bcd(tm.tm_hour);
    date[3] = tm.tm_wday;
    date[4] = bin2bcd(tm.tm_mday);
    date[5] = bin2bcd(tm.tm_mon + 1);
    date[6] = bin2bcd(tm.tm_year - 100);
    ret = regmap_bulk_write(rv3032.regmap, RV3032_SEC, date,
    sizeof(date));
    if (ret)
    return ret;
    ret = regmap_update_bits(rv3032.regmap, RV3032_STATUS,
    RV3032_STATUS_PORF | RV3032_STATUS_VLF, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_get_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int rv3032_get_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    u8 alarmvals[3];
    int status, ctrl, ret;
    ret = regmap_bulk_read(rv3032.regmap, RV3032_ALARM_MIN, alarmvals,
    sizeof(alarmvals));
    if (ret)
    return ret;
    ret = regmap_read(rv3032.regmap, RV3032_STATUS, &status);
    if (ret < 0)
    return ret;
    ret = regmap_read(rv3032.regmap, RV3032_CTRL2, &ctrl);
    if (ret < 0)
    return ret;
    alrm.time.tm_sec  = 0;
    alrm.time.tm_min  = bcd2bin(alarmvals[0] & 0x7f);
    alrm.time.tm_hour = bcd2bin(alarmvals[1] & 0x3f);
    alrm.time.tm_mday = bcd2bin(alarmvals[2] & 0x3f);
    alrm.enabled = !!(ctrl & RV3032_CTRL2_AIE);
    alrm.pending = (status & RV3032_STATUS_AF) && alrm.enabled;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int rv3032_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    u8 alarmvals[3];
    let mut ctrl: u8 = 0;
    int ret;
    ret = regmap_update_bits(rv3032.regmap, RV3032_CTRL2,
    RV3032_CTRL2_AIE | RV3032_CTRL2_UIE, 0);
    if (ret)
    return ret;
    alarmvals[0] = bin2bcd(alrm.time.tm_min);
    alarmvals[1] = bin2bcd(alrm.time.tm_hour);
    alarmvals[2] = bin2bcd(alrm.time.tm_mday);
    ret = regmap_update_bits(rv3032.regmap, RV3032_STATUS,
    RV3032_STATUS_AF, 0);
    if (ret)
    return ret;
    ret = regmap_bulk_write(rv3032.regmap, RV3032_ALARM_MIN, alarmvals,
    sizeof(alarmvals));
    if (ret)
    return ret;
    if (alrm.enabled) {
    if (rv3032.rtc.uie_rtctimer.enabled)
    ctrl |= RV3032_CTRL2_UIE;
    if (rv3032.rtc.aie_timer.enabled)
    ctrl |= RV3032_CTRL2_AIE;
    }
    ret = regmap_update_bits(rv3032.regmap, RV3032_CTRL2,
    RV3032_CTRL2_UIE | RV3032_CTRL2_AIE, ctrl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int rv3032_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    let mut ctrl: c_int = 0, ret;
    if (enabled) {
    if (rv3032.rtc.uie_rtctimer.enabled)
    ctrl |= RV3032_CTRL2_UIE;
    if (rv3032.rtc.aie_timer.enabled)
    ctrl |= RV3032_CTRL2_AIE;
    }
    ret = regmap_update_bits(rv3032.regmap, RV3032_STATUS,
    RV3032_STATUS_AF | RV3032_STATUS_UF, 0);
    if (ret)
    return ret;
    ret = regmap_update_bits(rv3032.regmap, RV3032_CTRL2,
    RV3032_CTRL2_UIE | RV3032_CTRL2_AIE, ctrl);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_read_offset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int rv3032_read_offset(struct device *dev, long *offset)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    int ret, value, steps;
    ret = regmap_read(rv3032.regmap, RV3032_OFFSET, &value);
    if (ret < 0)
    return ret;
    steps = FIELD_GET_SIGNED(RV3032_OFFSET_MSK, value);
// offset = DIV_ROUND_CLOSEST(steps * OFFSET_STEP_PPT, 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_set_offset(dev: *mut device, offset: c_long) -> c_int {
    static int rv3032_set_offset(struct device *dev, long offset)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    offset = clamp(offset, -7629L, 7391L) * 1000;
    offset = DIV_ROUND_CLOSEST(offset, OFFSET_STEP_PPT);
    return rv3032_update_cfg(rv3032, RV3032_OFFSET, RV3032_OFFSET_MSK,
    FIELD_PREP(RV3032_OFFSET_MSK, offset));
    }
#[no_mangle]
unsafe extern "C" fn rv3032_param_get(dev: *mut device, param: *mut rtc_param) -> c_int {
    static int rv3032_param_get(struct device *dev, struct rtc_param *param)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    int ret;
    switch(param.param) {
    u32 value;
    case RTC_PARAM_BACKUP_SWITCH_MODE:
    ret = regmap_read(rv3032.regmap, RV3032_PMU, &value);
    if (ret < 0)
    return ret;
    value = FIELD_GET(RV3032_PMU_BSM, value);
    switch(value) {
    case RV3032_PMU_BSM_DSM:
    param.uvalue = RTC_BSM_DIRECT;
    break;
    case RV3032_PMU_BSM_LSM:
    param.uvalue = RTC_BSM_LEVEL;
    break;
    default:
    param.uvalue = RTC_BSM_DISABLED;
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_param_set(dev: *mut device, param: *mut rtc_param) -> c_int {
    static int rv3032_param_set(struct device *dev, struct rtc_param *param)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    switch(param.param) {
    u8 mode;
    case RTC_PARAM_BACKUP_SWITCH_MODE:
    if (rv3032.trickle_charger_set)
    return -EINVAL;
    switch (param.uvalue) {
    case RTC_BSM_DISABLED:
    mode = 0;
    break;
    case RTC_BSM_DIRECT:
    mode = RV3032_PMU_BSM_DSM;
    break;
    case RTC_BSM_LEVEL:
    mode = RV3032_PMU_BSM_LSM;
    break;
    default:
    return -EINVAL;
    }
    return rv3032_update_cfg(rv3032, RV3032_PMU, RV3032_PMU_BSM,
    FIELD_PREP(RV3032_PMU_BSM, mode));
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_ioctl(dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    static int rv3032_ioctl(struct device *dev, unsigned int cmd, unsigned long arg)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    int status, val = 0, ret = 0;
    switch (cmd) {
    case RTC_VL_READ:
    ret = regmap_read(rv3032.regmap, RV3032_STATUS, &status);
    if (ret < 0)
    return ret;
    if (status & (RV3032_STATUS_PORF | RV3032_STATUS_VLF))
    val = RTC_VL_DATA_INVALID;
    return put_user(val, (unsigned int __user *)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
#[no_mangle]
unsafe extern "C" fn rv3032_nvram_write(priv: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int rv3032_nvram_write(void *priv, unsigned int offset, void *val, size_t bytes)
    {
    return regmap_bulk_write(priv, RV3032_RAM1 + offset, val, bytes);
    }
#[no_mangle]
unsafe extern "C" fn rv3032_nvram_read(priv: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int rv3032_nvram_read(void *priv, unsigned int offset, void *val, size_t bytes)
    {
    return regmap_bulk_read(priv, RV3032_RAM1 + offset, val, bytes);
    }
#[no_mangle]
unsafe extern "C" fn rv3032_eeprom_write(priv: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int rv3032_eeprom_write(void *priv, unsigned int offset, void *val, size_t bytes)
    {
    struct rv3032_data *rv3032 = priv;
    u32 status, eerd;
    int i, ret;
    u8 *buf = val;
    ret = rv3032_enter_eerd(rv3032, &eerd);
    if (ret)
    return ret;
    for (i = 0; i < bytes; i++) {
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_ADDR,
    RV3032_EEPROM_USER + offset + i);
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_DATA, buf[i]);
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_CMD,
    RV3032_EEPROM_CMD_WRITE);
    if (ret)
    goto exit_eerd;
    usleep_range(RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    ret = regmap_read_poll_timeout(rv3032.regmap, RV3032_TLSB, status,
    !(status & RV3032_TLSB_EEBUSY),
    RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    if (ret)
    goto exit_eerd;
    }
    exit_eerd:
    rv3032_exit_eerd(rv3032, eerd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_eeprom_read(priv: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int rv3032_eeprom_read(void *priv, unsigned int offset, void *val, size_t bytes)
    {
    struct rv3032_data *rv3032 = priv;
    u32 status, eerd, data;
    int i, ret;
    u8 *buf = val;
    ret = rv3032_enter_eerd(rv3032, &eerd);
    if (ret)
    return ret;
    for (i = 0; i < bytes; i++) {
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_ADDR,
    RV3032_EEPROM_USER + offset + i);
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_CMD,
    RV3032_EEPROM_CMD_READ);
    if (ret)
    goto exit_eerd;
    ret = regmap_read_poll_timeout(rv3032.regmap, RV3032_TLSB, status,
    !(status & RV3032_TLSB_EEBUSY),
    RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    if (ret)
    goto exit_eerd;
    ret = regmap_read(rv3032.regmap, RV3032_EEPROM_DATA, &data);
    if (ret)
    goto exit_eerd;
    buf[i] = data;
    }
    exit_eerd:
    rv3032_exit_eerd(rv3032, eerd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_trickle_charger_setup(dev: *mut device, rv3032: *mut rv3032_data) -> c_int {
    static int rv3032_trickle_charger_setup(struct device *dev, struct rv3032_data *rv3032)
    {
    u32 val, ohms, voltage;
    int i;
    val = FIELD_PREP(RV3032_PMU_TCM, 1) | FIELD_PREP(RV3032_PMU_BSM, RV3032_PMU_BSM_DSM);
    if (!device_property_read_u32(dev, "trickle-voltage-millivolt", &voltage)) {
    for (i = 0; i < ARRAY_SIZE(rv3032_trickle_voltages); i++)
    if (voltage == rv3032_trickle_voltages[i])
    break;
    if (i < ARRAY_SIZE(rv3032_trickle_voltages))
    val = FIELD_PREP(RV3032_PMU_TCM, i) |
    FIELD_PREP(RV3032_PMU_BSM, RV3032_PMU_BSM_LSM);
    }
    if (device_property_read_u32(dev, "trickle-resistor-ohms", &ohms))
    return 0;
    for (i = 0; i < ARRAY_SIZE(rv3032_trickle_resistors); i++)
    if (ohms == rv3032_trickle_resistors[i])
    break;
    if (i >= ARRAY_SIZE(rv3032_trickle_resistors)) {
    dev_warn(dev, "invalid trickle resistor value\n");
    return 0;
    }
    rv3032.trickle_charger_set = true;
    return rv3032_update_cfg(rv3032, RV3032_PMU,
    RV3032_PMU_TCR | RV3032_PMU_TCM | RV3032_PMU_BSM,
    val | FIELD_PREP(RV3032_PMU_TCR, i));
    }

    static int clkout_xtal_rates[] = {
    32768,
    1024,
    64,
    1,
    };
pub const RV3032_HFD_STEP: c_int = 8192;
    static unsigned long rv3032_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    int clkout, ret;
    struct rv3032_data *rv3032 = clkout_hw_to_rv3032(hw);
    ret = regmap_read(rv3032.regmap, RV3032_CLKOUT2, &clkout);
    if (ret < 0)
    return 0;
    if (clkout & RV3032_CLKOUT2_OS) {
    let mut rate: c_ulong = FIELD_GET(RV3032_CLKOUT2_HFD_MSK, clkout) << 8;
    ret = regmap_read(rv3032.regmap, RV3032_CLKOUT1, &clkout);
    if (ret < 0)
    return 0;
    rate += clkout + 1;
    return rate * RV3032_HFD_STEP;
    }
    return clkout_xtal_rates[FIELD_GET(RV3032_CLKOUT2_FD_MSK, clkout)];
    }
    static int rv3032_clkout_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    int i, hfd;
    if (req.rate < RV3032_HFD_STEP)
    for (i = 0; i < ARRAY_SIZE(clkout_xtal_rates); i++)
    if (clkout_xtal_rates[i] <= req.rate) {
    req.rate = clkout_xtal_rates[i];
    return 0;
    }
    hfd = DIV_ROUND_CLOSEST(req.rate, RV3032_HFD_STEP);
    req.rate = RV3032_HFD_STEP * clamp(hfd, 0, 8192);
    return 0;
    }
    static int rv3032_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct rv3032_data *rv3032 = clkout_hw_to_rv3032(hw);
    u32 status, eerd;
    int i, hfd, ret;
    for (i = 0; i < ARRAY_SIZE(clkout_xtal_rates); i++) {
    if (clkout_xtal_rates[i] == rate) {
    return rv3032_update_cfg(rv3032, RV3032_CLKOUT2, 0xff,
    FIELD_PREP(RV3032_CLKOUT2_FD_MSK, i));
    }
    }
    hfd = DIV_ROUND_CLOSEST(rate, RV3032_HFD_STEP);
    hfd = clamp(hfd, 1, 8192) - 1;
    ret = rv3032_enter_eerd(rv3032, &eerd);
    if (ret)
    return ret;
    ret = regmap_write(rv3032.regmap, RV3032_CLKOUT1, hfd & 0xff);
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_CLKOUT2, RV3032_CLKOUT2_OS |
    FIELD_PREP(RV3032_CLKOUT2_HFD_MSK, hfd >> 8));
    if (ret)
    goto exit_eerd;
    ret = regmap_write(rv3032.regmap, RV3032_EEPROM_CMD, RV3032_EEPROM_CMD_UPDATE);
    if (ret)
    goto exit_eerd;
    usleep_range(46000, RV3032_EEBUSY_TIMEOUT);
    ret = regmap_read_poll_timeout(rv3032.regmap, RV3032_TLSB, status,
    !(status & RV3032_TLSB_EEBUSY),
    RV3032_EEBUSY_POLL, RV3032_EEBUSY_TIMEOUT);
    exit_eerd:
    rv3032_exit_eerd(rv3032, eerd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3032_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int rv3032_clkout_prepare(struct clk_hw *hw)
    {
    struct rv3032_data *rv3032 = clkout_hw_to_rv3032(hw);
    return rv3032_update_cfg(rv3032, RV3032_PMU, RV3032_PMU_NCLKE, 0);
    }
#[no_mangle]
unsafe extern "C" fn rv3032_clkout_unprepare(hw: *mut clk_hw) {
    static void rv3032_clkout_unprepare(struct clk_hw *hw)
    {
    struct rv3032_data *rv3032 = clkout_hw_to_rv3032(hw);
    rv3032_update_cfg(rv3032, RV3032_PMU, RV3032_PMU_NCLKE, RV3032_PMU_NCLKE);
    }
#[no_mangle]
unsafe extern "C" fn rv3032_clkout_is_prepared(hw: *mut clk_hw) -> c_int {
    static int rv3032_clkout_is_prepared(struct clk_hw *hw)
    {
    int val, ret;
    struct rv3032_data *rv3032 = clkout_hw_to_rv3032(hw);
    ret = regmap_read(rv3032.regmap, RV3032_PMU, &val);
    if (ret < 0)
    return ret;
    return !(val & RV3032_PMU_NCLKE);
    }
    static const struct clk_ops rv3032_clkout_ops = {
    .prepare = rv3032_clkout_prepare,
    .unprepare = rv3032_clkout_unprepare,
    .is_prepared = rv3032_clkout_is_prepared,
    .recalc_rate = rv3032_clkout_recalc_rate,
    .determine_rate = rv3032_clkout_determine_rate,
    .set_rate = rv3032_clkout_set_rate,
    };
    static int rv3032_clkout_register_clk(struct rv3032_data *rv3032,
    struct i2c_client *client)
    {
    int ret;
    struct clk *clk;
    let mut init: clk_init_data = {};
    struct device_node *node = client.dev.of_node;
    ret = regmap_update_bits(rv3032.regmap, RV3032_TLSB, RV3032_TLSB_CLKF, 0);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(rv3032.regmap, RV3032_CTRL2, RV3032_CTRL2_CLKIE, 0);
    if (ret < 0)
    return ret;
    ret = regmap_write(rv3032.regmap, RV3032_CLK_IRQ, 0);
    if (ret < 0)
    return ret;
    init.name = "rv3032-clkout";
    init.ops = &rv3032_clkout_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    rv3032.clkout_hw.init = &init;
    of_property_read_string(node, "clock-output-names", &init.name);
    clk = devm_clk_register(&client.dev, &rv3032.clkout_hw);
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn rv3032_hwmon_read_temp(dev: *mut device, mC: *mut c_long) -> c_int {
    static int rv3032_hwmon_read_temp(struct device *dev, long *mC)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    u8 buf[2];
    int temp, prev = 0;
    int ret;
    ret = regmap_bulk_read(rv3032.regmap, RV3032_TLSB, buf, sizeof(buf));
    if (ret)
    return ret;
    temp = sign_extend32(buf[1], 7) << 4;
    temp |= FIELD_GET(RV3032_TLSB_TEMP, buf[0]);
// No blocking or shadowing on RV3032_TLSB and RV3032_TMSB
    do {
    prev = temp;
    ret = regmap_bulk_read(rv3032.regmap, RV3032_TLSB, buf, sizeof(buf));
    if (ret)
    return ret;
    temp = sign_extend32(buf[1], 7) << 4;
    temp |= FIELD_GET(RV3032_TLSB_TEMP, buf[0]);
    } while (temp != prev);
// mC = (temp * 1000) / 16;
    return 0;
    }
    static umode_t rv3032_hwmon_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type != hwmon_temp)
    return 0;
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    default:
    return 0;
    }
    }
    static int rv3032_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    int err;
    switch (attr) {
    case hwmon_temp_input:
    err = rv3032_hwmon_read_temp(dev, temp);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
    static const struct hwmon_channel_info * const rv3032_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops rv3032_hwmon_hwmon_ops = {
    .is_visible = rv3032_hwmon_is_visible,
    .read = rv3032_hwmon_read,
    };
    static const struct hwmon_chip_info rv3032_hwmon_chip_info = {
    .ops = &rv3032_hwmon_hwmon_ops,
    .info = rv3032_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn rv3032_hwmon_register(dev: *mut device) {
    static void rv3032_hwmon_register(struct device *dev)
    {
    struct rv3032_data *rv3032 = dev_get_drvdata(dev);
    if (!IS_REACHABLE(CONFIG_HWMON))
    return;
    devm_hwmon_device_register_with_info(dev, "rv3032", rv3032, &rv3032_hwmon_chip_info, core::ptr::null_mut());
    }
    static const struct rtc_class_ops rv3032_rtc_ops = {
    .read_time = rv3032_get_time,
    .set_time = rv3032_set_time,
    .read_offset = rv3032_read_offset,
    .set_offset = rv3032_set_offset,
    .ioctl = rv3032_ioctl,
    .read_alarm = rv3032_get_alarm,
    .set_alarm = rv3032_set_alarm,
    .alarm_irq_enable = rv3032_alarm_irq_enable,
    .param_get = rv3032_param_get,
    .param_set = rv3032_param_set,
    };
    static const struct regmap_config regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xCA,
    };
#[no_mangle]
unsafe extern "C" fn rv3032_probe(client: *mut i2c_client) -> c_int {
    static int rv3032_probe(struct i2c_client *client)
    {
    struct rv3032_data *rv3032;
    int ret, status;
    struct nvmem_config nvmem_cfg = {
    .name = "rv3032_nvram",
    .word_size = 1,
    .stride = 1,
    .size = 16,
    .type = NVMEM_TYPE_BATTERY_BACKED,
    .reg_read = rv3032_nvram_read,
    .reg_write = rv3032_nvram_write,
    };
    struct nvmem_config eeprom_cfg = {
    .name = "rv3032_eeprom",
    .word_size = 1,
    .stride = 1,
    .size = 32,
    .type = NVMEM_TYPE_EEPROM,
    .reg_read = rv3032_eeprom_read,
    .reg_write = rv3032_eeprom_write,
    };
    rv3032 = devm_kzalloc(&client.dev, sizeof(struct rv3032_data),
    GFP_KERNEL);
    if (!rv3032)
    return -ENOMEM;
    rv3032.regmap = devm_regmap_init_i2c(client, &regmap_config);
    if (IS_ERR(rv3032.regmap))
    return PTR_ERR(rv3032.regmap);
    i2c_set_clientdata(client, rv3032);
    ret = regmap_read(rv3032.regmap, RV3032_STATUS, &status);
    if (ret < 0)
    return ret;
    rv3032.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(rv3032.rtc))
    return PTR_ERR(rv3032.rtc);
    if (client.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&client.dev))
    irqflags = 0;
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), rv3032_handle_irq,
    irqflags | IRQF_ONESHOT,
    "rv3032", rv3032);
    if (ret) {
    dev_warn(&client.dev, "unable to request IRQ, alarms disabled\n");
    client.irq = 0;
    }
    }
    if (!client.irq)
    clear_bit(RTC_FEATURE_ALARM, rv3032.rtc.features);
    rv3032_trickle_charger_setup(&client.dev, rv3032);
    set_bit(RTC_FEATURE_BACKUP_SWITCH_MODE, rv3032.rtc.features);
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, rv3032.rtc.features);
    rv3032.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rv3032.rtc.range_max = RTC_TIMESTAMP_END_2099;
    rv3032.rtc.ops = &rv3032_rtc_ops;
    ret = devm_rtc_register_device(rv3032.rtc);
    if (ret)
    return ret;
    nvmem_cfg.priv = rv3032.regmap;
    devm_rtc_nvmem_register(rv3032.rtc, &nvmem_cfg);
    eeprom_cfg.priv = rv3032;
    devm_rtc_nvmem_register(rv3032.rtc, &eeprom_cfg);

    rv3032_clkout_register_clk(rv3032, client);

    rv3032_hwmon_register(&client.dev);
    return 0;
    }
    static const struct acpi_device_id rv3032_i2c_acpi_match[] = {
    { "MCRY3032" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, rv3032_i2c_acpi_match);
    static const __maybe_unused struct of_device_id rv3032_of_match[] = {
    { .compatible = "microcrystal,rv3032", },
    { }
    };
    MODULE_DEVICE_TABLE(of, rv3032_of_match);
    static struct i2c_driver rv3032_driver = {
    .driver = {
    .name = "rtc-rv3032",
    .acpi_match_table = rv3032_i2c_acpi_match,
    .of_match_table = of_match_ptr(rv3032_of_match),
    },
    .probe		= rv3032_probe,
    };
    module_i2c_driver(rv3032_driver);
    MODULE_AUTHOR("Alexandre Belloni <alexandre.belloni@bootlin.com>");
    MODULE_DESCRIPTION("Micro Crystal RV3032 RTC driver");
    MODULE_LICENSE("GPL v2");
