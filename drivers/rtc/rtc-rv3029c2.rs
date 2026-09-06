//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rv3029c2.c
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
// Micro Crystal RV-3029 / RV-3049 rtc class driver
//
// Author: Gregory Hermant <gregory.hermant@calao-systems.com>
// Michael Buesch <m@bues.ch>
//
// based on previously existing rtc class drivers
//

// Register map
// control section
pub const RV3029_ONOFF_CTRL: c_uint = 0x00;

pub const RV3029_IRQ_CTRL: c_uint = 0x01;

pub const RV3029_IRQ_FLAGS: c_uint = 0x02;

pub const RV3029_STATUS: c_uint = 0x03;

pub const RV3029_RST_CTRL: c_uint = 0x04;

pub const RV3029_CONTROL_SECTION_LEN: c_uint = 0x05;
// watch section
pub const RV3029_W_SEC: c_uint = 0x08;
pub const RV3029_W_MINUTES: c_uint = 0x09;
pub const RV3029_W_HOURS: c_uint = 0x0A;

pub const RV3029_W_DATE: c_uint = 0x0B;
pub const RV3029_W_DAYS: c_uint = 0x0C;
pub const RV3029_W_MONTHS: c_uint = 0x0D;
pub const RV3029_W_YEARS: c_uint = 0x0E;
pub const RV3029_WATCH_SECTION_LEN: c_uint = 0x07;
// alarm section
pub const RV3029_A_SC: c_uint = 0x10;
pub const RV3029_A_MN: c_uint = 0x11;
pub const RV3029_A_HR: c_uint = 0x12;
pub const RV3029_A_DT: c_uint = 0x13;
pub const RV3029_A_DW: c_uint = 0x14;
pub const RV3029_A_MO: c_uint = 0x15;
pub const RV3029_A_YR: c_uint = 0x16;

pub const RV3029_ALARM_SECTION_LEN: c_uint = 0x07;
// timer section
pub const RV3029_TIMER_LOW: c_uint = 0x18;
pub const RV3029_TIMER_HIGH: c_uint = 0x19;
// temperature section
pub const RV3029_TEMP_PAGE: c_uint = 0x20;
// eeprom data section
pub const RV3029_E2P_EEDATA1: c_uint = 0x28;
pub const RV3029_E2P_EEDATA2: c_uint = 0x29;
pub const RV3029_E2PDATA_SECTION_LEN: c_uint = 0x02;
// eeprom control section
pub const RV3029_CONTROL_E2P_EECTRL: c_uint = 0x30;

    RV3029_TRICKLE_5K |\
    RV3029_TRICKLE_20K |\
    RV3029_TRICKLE_80K)
pub const RV3029_TRICKLE_SHIFT: c_int = 4;
pub const RV3029_CONTROL_E2P_XOFFS: c_uint = 0x31 /* XTAL offset */;

pub const RV3029_CONTROL_E2P_QCOEF: c_uint = 0x32 /* XTAL temp drift coef */;
pub const RV3029_CONTROL_E2P_TURNOVER: c_uint = 0x33 /* XTAL turnover temp (in *C) */;
pub const RV3029_CONTROL_E2P_TOV_MASK: c_uint = 0x3F /* XTAL turnover temp mask */;
// user ram section
pub const RV3029_RAM_PAGE: c_uint = 0x38;
pub const RV3029_RAM_SECTION_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv3029_data {
    pub dev: *mut device,
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn rv3029_eeprom_busywait(rv3029: *mut rv3029_data) -> c_int {
    static int rv3029_eeprom_busywait(struct rv3029_data *rv3029)
    {
    unsigned int sr;
    int i, ret;
    for (i = 100; i > 0; i--) {
    ret = regmap_read(rv3029.regmap, RV3029_STATUS, &sr);
    if (ret < 0)
    break;
    if (!(sr & RV3029_STATUS_EEBUSY))
    break;
    usleep_range(1000, 10000);
    }
    if (i <= 0) {
    dev_err(rv3029.dev, "EEPROM busy wait timeout.\n");
    return -ETIMEDOUT;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_eeprom_exit(rv3029: *mut rv3029_data) -> c_int {
    static int rv3029_eeprom_exit(struct rv3029_data *rv3029)
    {
// Re-enable eeprom refresh
    return regmap_update_bits(rv3029.regmap, RV3029_ONOFF_CTRL,
    RV3029_ONOFF_CTRL_EERE,
    RV3029_ONOFF_CTRL_EERE);
    }
#[no_mangle]
unsafe extern "C" fn rv3029_eeprom_enter(rv3029: *mut rv3029_data) -> c_int {
    static int rv3029_eeprom_enter(struct rv3029_data *rv3029)
    {
    unsigned int sr;
    int ret;
// Check whether we are in the allowed voltage range.
    ret = regmap_read(rv3029.regmap, RV3029_STATUS, &sr);
    if (ret < 0)
    return ret;
    if (sr & RV3029_STATUS_VLOW2)
    return -ENODEV;
    if (sr & RV3029_STATUS_VLOW1) {
// We clear the bits and retry once just in case
// we had a brown out in early startup.
//
    ret = regmap_update_bits(rv3029.regmap, RV3029_STATUS,
    RV3029_STATUS_VLOW1, 0);
    if (ret < 0)
    return ret;
    usleep_range(1000, 10000);
    ret = regmap_read(rv3029.regmap, RV3029_STATUS, &sr);
    if (ret < 0)
    return ret;
    if (sr & RV3029_STATUS_VLOW1) {
    dev_err(rv3029.dev,
    "Supply voltage is too low to safely access the EEPROM.\n");
    return -ENODEV;
    }
    }
// Disable eeprom refresh.
    ret = regmap_update_bits(rv3029.regmap, RV3029_ONOFF_CTRL,
    RV3029_ONOFF_CTRL_EERE, 0);
    if (ret < 0)
    return ret;
// Wait for any previous eeprom accesses to finish.
    ret = rv3029_eeprom_busywait(rv3029);
    if (ret < 0)
    rv3029_eeprom_exit(rv3029);
    return ret;
    }
    static int rv3029_eeprom_read(struct rv3029_data *rv3029, u8 reg,
    u8 buf[], size_t len)
    {
    int ret, err;
    err = rv3029_eeprom_enter(rv3029);
    if (err < 0)
    return err;
    ret = regmap_bulk_read(rv3029.regmap, reg, buf, len);
    err = rv3029_eeprom_exit(rv3029);
    if (err < 0)
    return err;
    return ret;
    }
    static int rv3029_eeprom_write(struct rv3029_data *rv3029, u8 reg,
    u8 const buf[], size_t len)
    {
    unsigned int tmp;
    int ret, err;
    size_t i;
    err = rv3029_eeprom_enter(rv3029);
    if (err < 0)
    return err;
    for (i = 0; i < len; i++, reg++) {
    ret = regmap_read(rv3029.regmap, reg, &tmp);
    if (ret < 0)
    break;
    if (tmp != buf[i]) {
    tmp = buf[i];
    ret = regmap_write(rv3029.regmap, reg, tmp);
    if (ret < 0)
    break;
    }
    ret = rv3029_eeprom_busywait(rv3029);
    if (ret < 0)
    break;
    }
    err = rv3029_eeprom_exit(rv3029);
    if (err < 0)
    return err;
    return ret;
    }
    static int rv3029_eeprom_update_bits(struct rv3029_data *rv3029,
    u8 reg, u8 mask, u8 set)
    {
    u8 buf;
    int ret;
    ret = rv3029_eeprom_read(rv3029, reg, &buf, 1);
    if (ret < 0)
    return ret;
    buf &= ~mask;
    buf |= set & mask;
    ret = rv3029_eeprom_write(rv3029, reg, &buf, 1);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rv3029_handle_irq(int irq, void *dev_id)
    {
    struct device *dev = dev_id;
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    unsigned int flags, controls;
    let mut events: c_ulong = 0;
    int ret;
    rtc_lock(rv3029.rtc);
    ret = regmap_read(rv3029.regmap, RV3029_IRQ_CTRL, &controls);
    if (ret) {
    dev_warn(dev, "Read IRQ Control Register error %d\n", ret);
    rtc_unlock(rv3029.rtc);
    return IRQ_NONE;
    }
    ret = regmap_read(rv3029.regmap, RV3029_IRQ_FLAGS, &flags);
    if (ret) {
    dev_warn(dev, "Read IRQ Flags Register error %d\n", ret);
    rtc_unlock(rv3029.rtc);
    return IRQ_NONE;
    }
    if (flags & RV3029_IRQ_FLAGS_AF) {
    flags &= ~RV3029_IRQ_FLAGS_AF;
    controls &= ~RV3029_IRQ_CTRL_AIE;
    events |= RTC_AF;
    }
    if (events) {
    rtc_update_irq(rv3029.rtc, 1, events);
    regmap_write(rv3029.regmap, RV3029_IRQ_FLAGS, flags);
    regmap_write(rv3029.regmap, RV3029_IRQ_CTRL, controls);
    }
    rtc_unlock(rv3029.rtc);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rv3029_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    unsigned int sr;
    int ret;
    u8 regs[RV3029_WATCH_SECTION_LEN] = { 0, };
    ret = regmap_read(rv3029.regmap, RV3029_STATUS, &sr);
    if (ret < 0)
    return ret;
    if (sr & (RV3029_STATUS_VLOW2 | RV3029_STATUS_PON))
    return -EINVAL;
    ret = regmap_bulk_read(rv3029.regmap, RV3029_W_SEC, regs,
    RV3029_WATCH_SECTION_LEN);
    if (ret < 0)
    return ret;
    tm.tm_sec = bcd2bin(regs[RV3029_W_SEC - RV3029_W_SEC]);
    tm.tm_min = bcd2bin(regs[RV3029_W_MINUTES - RV3029_W_SEC]);
// HR field has a more complex interpretation
    {
    let mut _hr: u8 = regs[RV3029_W_HOURS - RV3029_W_SEC];
    if (_hr & RV3029_REG_HR_12_24) {
// 12h format
    tm.tm_hour = bcd2bin(_hr & 0x1f);
    if (_hr & RV3029_REG_HR_PM)	/* PM flag set */
    tm.tm_hour += 12;
    } else /* 24h format */
    tm.tm_hour = bcd2bin(_hr & 0x3f);
    }
    tm.tm_mday = bcd2bin(regs[RV3029_W_DATE - RV3029_W_SEC]);
    tm.tm_mon = bcd2bin(regs[RV3029_W_MONTHS - RV3029_W_SEC]) - 1;
    tm.tm_year = bcd2bin(regs[RV3029_W_YEARS - RV3029_W_SEC]) + 100;
    tm.tm_wday = bcd2bin(regs[RV3029_W_DAYS - RV3029_W_SEC]) - 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int rv3029_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    let mut tm: *mut rtc_time const = &alarm.time;
    unsigned int controls, flags;
    int ret;
    u8 regs[8];
    ret = regmap_bulk_read(rv3029.regmap, RV3029_A_SC, regs,
    RV3029_ALARM_SECTION_LEN);
    if (ret < 0)
    return ret;
    ret = regmap_read(rv3029.regmap, RV3029_IRQ_CTRL, &controls);
    if (ret)
    return ret;
    ret = regmap_read(rv3029.regmap, RV3029_IRQ_FLAGS, &flags);
    if (ret < 0)
    return ret;
    tm.tm_sec = bcd2bin(regs[RV3029_A_SC - RV3029_A_SC] & 0x7f);
    tm.tm_min = bcd2bin(regs[RV3029_A_MN - RV3029_A_SC] & 0x7f);
    tm.tm_hour = bcd2bin(regs[RV3029_A_HR - RV3029_A_SC] & 0x3f);
    tm.tm_mday = bcd2bin(regs[RV3029_A_DT - RV3029_A_SC] & 0x3f);
    tm.tm_mon = bcd2bin(regs[RV3029_A_MO - RV3029_A_SC] & 0x1f) - 1;
    tm.tm_year = bcd2bin(regs[RV3029_A_YR - RV3029_A_SC] & 0x7f) + 100;
    tm.tm_wday = bcd2bin(regs[RV3029_A_DW - RV3029_A_SC] & 0x07) - 1;
    alarm.enabled = !!(controls & RV3029_IRQ_CTRL_AIE);
    alarm.pending = (flags & RV3029_IRQ_FLAGS_AF) && alarm.enabled;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int rv3029_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    return regmap_update_bits(rv3029.regmap, RV3029_IRQ_CTRL,
    RV3029_IRQ_CTRL_AIE,
    enable ? RV3029_IRQ_CTRL_AIE : 0);
    }
#[no_mangle]
unsafe extern "C" fn rv3029_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int rv3029_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    let mut tm: *mut rtc_time const = &alarm.time;
    int ret;
    u8 regs[8];
// Activate all the alarms with AE_x bit
    regs[RV3029_A_SC - RV3029_A_SC] = bin2bcd(tm.tm_sec) | RV3029_A_AE_X;
    regs[RV3029_A_MN - RV3029_A_SC] = bin2bcd(tm.tm_min) | RV3029_A_AE_X;
    regs[RV3029_A_HR - RV3029_A_SC] = (bin2bcd(tm.tm_hour) & 0x3f)
    | RV3029_A_AE_X;
    regs[RV3029_A_DT - RV3029_A_SC] = (bin2bcd(tm.tm_mday) & 0x3f)
    | RV3029_A_AE_X;
    regs[RV3029_A_MO - RV3029_A_SC] = (bin2bcd(tm.tm_mon + 1) & 0x1f)
    | RV3029_A_AE_X;
    regs[RV3029_A_DW - RV3029_A_SC] = (bin2bcd(tm.tm_wday + 1) & 0x7)
    | RV3029_A_AE_X;
    regs[RV3029_A_YR - RV3029_A_SC] = (bin2bcd(tm.tm_year - 100))
    | RV3029_A_AE_X;
// Write the alarm
    ret = regmap_bulk_write(rv3029.regmap, RV3029_A_SC, regs,
    RV3029_ALARM_SECTION_LEN);
    if (ret < 0)
    return ret;
    return rv3029_alarm_irq_enable(dev, alarm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn rv3029_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rv3029_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    u8 regs[8];
    int ret;
    regs[RV3029_W_SEC - RV3029_W_SEC] = bin2bcd(tm.tm_sec);
    regs[RV3029_W_MINUTES - RV3029_W_SEC] = bin2bcd(tm.tm_min);
    regs[RV3029_W_HOURS - RV3029_W_SEC] = bin2bcd(tm.tm_hour);
    regs[RV3029_W_DATE - RV3029_W_SEC] = bin2bcd(tm.tm_mday);
    regs[RV3029_W_MONTHS - RV3029_W_SEC] = bin2bcd(tm.tm_mon + 1);
    regs[RV3029_W_DAYS - RV3029_W_SEC] = bin2bcd(tm.tm_wday + 1) & 0x7;
    regs[RV3029_W_YEARS - RV3029_W_SEC] = bin2bcd(tm.tm_year - 100);
    ret = regmap_bulk_write(rv3029.regmap, RV3029_W_SEC, regs,
    RV3029_WATCH_SECTION_LEN);
    if (ret < 0)
    return ret;
// clear PON and VLOW2 bits
    return regmap_update_bits(rv3029.regmap, RV3029_STATUS,
    RV3029_STATUS_PON | RV3029_STATUS_VLOW2, 0);
    }
#[no_mangle]
unsafe extern "C" fn rv3029_ioctl(dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    static int rv3029_ioctl(struct device *dev, unsigned int cmd, unsigned long arg)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    let mut vl: c_ulong = 0;
    int sr, ret = 0;
    switch (cmd) {
    case RTC_VL_READ:
    ret = regmap_read(rv3029.regmap, RV3029_STATUS, &sr);
    if (ret < 0)
    return ret;
    if (sr & RV3029_STATUS_VLOW1)
    vl = RTC_VL_ACCURACY_LOW;
    if (sr & (RV3029_STATUS_VLOW2 | RV3029_STATUS_PON))
    vl |= RTC_VL_DATA_INVALID;
    return put_user(vl, (unsigned int __user *)arg);
    case RTC_VL_CLR:
    return regmap_update_bits(rv3029.regmap, RV3029_STATUS,
    RV3029_STATUS_VLOW1, 0);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static int rv3029_nvram_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    return regmap_bulk_write(priv, RV3029_RAM_PAGE + offset, val, bytes);
    }
    static int rv3029_nvram_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    return regmap_bulk_read(priv, RV3029_RAM_PAGE + offset, val, bytes);
    }
    static const struct rv3029_trickle_tab_elem {
    u32 r;		/* resistance in ohms */
    u8 conf;	/* trickle config bits */
    } rv3029_trickle_tab[] = {
    {
    .r	= 1076,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_5K |
    RV3029_TRICKLE_20K | RV3029_TRICKLE_80K,
    }, {
    .r	= 1091,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_5K |
    RV3029_TRICKLE_20K,
    }, {
    .r	= 1137,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_5K |
    RV3029_TRICKLE_80K,
    }, {
    .r	= 1154,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_5K,
    }, {
    .r	= 1371,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_20K |
    RV3029_TRICKLE_80K,
    }, {
    .r	= 1395,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_20K,
    }, {
    .r	= 1472,
    .conf	= RV3029_TRICKLE_1K | RV3029_TRICKLE_80K,
    }, {
    .r	= 1500,
    .conf	= RV3029_TRICKLE_1K,
    }, {
    .r	= 3810,
    .conf	= RV3029_TRICKLE_5K | RV3029_TRICKLE_20K |
    RV3029_TRICKLE_80K,
    }, {
    .r	= 4000,
    .conf	= RV3029_TRICKLE_5K | RV3029_TRICKLE_20K,
    }, {
    .r	= 4706,
    .conf	= RV3029_TRICKLE_5K | RV3029_TRICKLE_80K,
    }, {
    .r	= 5000,
    .conf	= RV3029_TRICKLE_5K,
    }, {
    .r	= 16000,
    .conf	= RV3029_TRICKLE_20K | RV3029_TRICKLE_80K,
    }, {
    .r	= 20000,
    .conf	= RV3029_TRICKLE_20K,
    }, {
    .r	= 80000,
    .conf	= RV3029_TRICKLE_80K,
    },
    };
#[no_mangle]
unsafe extern "C" fn rv3029_trickle_config(dev: *mut device) {
    static void rv3029_trickle_config(struct device *dev)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    struct device_node *of_node = dev.of_node;
    const struct rv3029_trickle_tab_elem *elem;
    int i, err;
    u32 ohms;
    u8 trickle_set_bits;
    if (!of_node)
    return;
// Configure the trickle charger.
    err = of_property_read_u32(of_node, "trickle-resistor-ohms", &ohms);
    if (err) {
// Disable trickle charger.
    trickle_set_bits = 0;
    } else {
// Enable trickle charger.
    for (i = 0; i < ARRAY_SIZE(rv3029_trickle_tab); i++) {
    elem = &rv3029_trickle_tab[i];
    if (elem.r >= ohms)
    break;
    }
    trickle_set_bits = elem.conf;
    dev_info(dev,
    "Trickle charger enabled at %d ohms resistance.\n",
    elem.r);
    }
    err = rv3029_eeprom_update_bits(rv3029, RV3029_CONTROL_E2P_EECTRL,
    RV3029_TRICKLE_MASK,
    trickle_set_bits);
    if (err < 0)
    dev_err(dev, "Failed to update trickle charger config\n");
    }

#[no_mangle]
unsafe extern "C" fn rv3029_read_temp(rv3029: *mut rv3029_data, temp_mC: *mut c_int) -> c_int {
    static int rv3029_read_temp(struct rv3029_data *rv3029, int *temp_mC)
    {
    unsigned int temp;
    int ret;
    ret = regmap_read(rv3029.regmap, RV3029_TEMP_PAGE, &temp);
    if (ret < 0)
    return ret;
// temp_mC = ((int)temp - 60) * 1000;
    return 0;
    }
    static ssize_t rv3029_hwmon_show_temp(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    int ret, temp_mC;
    ret = rv3029_read_temp(rv3029, &temp_mC);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%d\n", temp_mC);
    }
    static ssize_t rv3029_hwmon_set_update_interval(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    let mut th_set_bits: c_uint = 0;
    unsigned long interval_ms;
    int ret;
    ret = kstrtoul(buf, 10, &interval_ms);
    if (ret < 0)
    return ret;
    if (interval_ms != 0) {
    th_set_bits |= RV3029_EECTRL_THE;
    if (interval_ms >= 16000)
    th_set_bits |= RV3029_EECTRL_THP;
    }
    ret = rv3029_eeprom_update_bits(rv3029, RV3029_CONTROL_E2P_EECTRL,
    RV3029_EECTRL_THE | RV3029_EECTRL_THP,
    th_set_bits);
    if (ret < 0)
    return ret;
    return count;
    }
    static ssize_t rv3029_hwmon_show_update_interval(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    int ret, interval_ms;
    u8 eectrl;
    ret = rv3029_eeprom_read(rv3029, RV3029_CONTROL_E2P_EECTRL,
    &eectrl, 1);
    if (ret < 0)
    return ret;
    if (eectrl & RV3029_EECTRL_THE) {
    if (eectrl & RV3029_EECTRL_THP)
    interval_ms = 16000;
    else
    interval_ms = 1000;
    } else {
    interval_ms = 0;
    }
    return sprintf(buf, "%d\n", interval_ms);
    }
    static SENSOR_DEVICE_ATTR(temp1_input, S_IRUGO, rv3029_hwmon_show_temp,
    core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(update_interval, S_IWUSR | S_IRUGO,
    rv3029_hwmon_show_update_interval,
    rv3029_hwmon_set_update_interval, 0);
    static struct attribute *rv3029_hwmon_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_update_interval.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(rv3029_hwmon);
#[no_mangle]
unsafe extern "C" fn rv3029_hwmon_register(dev: *mut device, name: *const c_char) {
    static void rv3029_hwmon_register(struct device *dev, const char *name)
    {
    struct rv3029_data *rv3029 = dev_get_drvdata(dev);
    struct device *hwmon_dev;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, name, rv3029,
    rv3029_hwmon_groups);
    if (IS_ERR(hwmon_dev)) {
    dev_warn(dev, "unable to register hwmon device %ld\n",
    PTR_ERR(hwmon_dev));
    }
    }

#[no_mangle]
unsafe extern "C" fn rv3029_hwmon_register(dev: *mut device, name: *const c_char) {
    static void rv3029_hwmon_register(struct device *dev, const char *name)
    {
    }

    static const struct rtc_class_ops rv3029_rtc_ops = {
    .read_time	= rv3029_read_time,
    .set_time	= rv3029_set_time,
    .ioctl		= rv3029_ioctl,
    .read_alarm	= rv3029_read_alarm,
    .set_alarm	= rv3029_set_alarm,
    .alarm_irq_enable = rv3029_alarm_irq_enable,
    };
    static int rv3029_probe(struct device *dev, struct regmap *regmap, int irq,
    const char *name)
    {
    struct rv3029_data *rv3029;
    struct nvmem_config nvmem_cfg = {
    .name = "rv3029_nvram",
    .word_size = 1,
    .stride = 1,
    .size = RV3029_RAM_SECTION_LEN,
    .type = NVMEM_TYPE_BATTERY_BACKED,
    .reg_read = rv3029_nvram_read,
    .reg_write = rv3029_nvram_write,
    };
    let mut rc: c_int = 0;
    rv3029 = devm_kzalloc(dev, sizeof(*rv3029), GFP_KERNEL);
    if (!rv3029)
    return -ENOMEM;
    rv3029.regmap = regmap;
    rv3029.irq = irq;
    rv3029.dev = dev;
    dev_set_drvdata(dev, rv3029);
    rv3029_trickle_config(dev);
    rv3029_hwmon_register(dev, name);
    rv3029.rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rv3029.rtc))
    return PTR_ERR(rv3029.rtc);
    if (rv3029.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(dev))
    irqflags = 0;
    rc = devm_request_threaded_irq(dev, rv3029.irq,
    core::ptr::null_mut(), rv3029_handle_irq,
    irqflags | IRQF_ONESHOT,
    "rv3029", dev);
    if (rc) {
    dev_warn(dev, "unable to request IRQ, alarms disabled\n");
    rv3029.irq = 0;
    }
    }
    if (!rv3029.irq)
    clear_bit(RTC_FEATURE_ALARM, rv3029.rtc.features);
    rv3029.rtc.ops = &rv3029_rtc_ops;
    rv3029.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rv3029.rtc.range_max = RTC_TIMESTAMP_END_2079;
    rc = devm_rtc_register_device(rv3029.rtc);
    if (rc)
    return rc;
    nvmem_cfg.priv = rv3029.regmap;
    devm_rtc_nvmem_register(rv3029.rtc, &nvmem_cfg);
    return 0;
    }
    static const struct regmap_range rv3029_holes_range[] = {
    regmap_reg_range(0x05, 0x07),
    regmap_reg_range(0x0f, 0x0f),
    regmap_reg_range(0x17, 0x17),
    regmap_reg_range(0x1a, 0x1f),
    regmap_reg_range(0x21, 0x27),
    regmap_reg_range(0x34, 0x37),
    };
    static const struct regmap_access_table rv3029_regs = {
    .no_ranges =	rv3029_holes_range,
    .n_no_ranges =	ARRAY_SIZE(rv3029_holes_range),
    };
    static const struct regmap_config config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &rv3029_regs,
    .wr_table = &rv3029_regs,
    .max_register = 0x3f,
    };

#[no_mangle]
unsafe extern "C" fn rv3029_i2c_probe(client: *mut i2c_client) -> c_int {
    static int rv3029_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_I2C_BLOCK |
    I2C_FUNC_SMBUS_BYTE)) {
    dev_err(&client.dev, "Adapter does not support SMBUS_I2C_BLOCK or SMBUS_I2C_BYTE\n");
    return -ENODEV;
    }
    regmap = devm_regmap_init_i2c(client, &config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return rv3029_probe(&client.dev, regmap, client.irq, client.name);
    }
    static const struct i2c_device_id rv3029_id[] = {
    { .name = "rv3029" },
    { .name = "rv3029c2" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, rv3029_id);
    static const __maybe_unused struct of_device_id rv3029_of_match[] = {
    { .compatible = "microcrystal,rv3029" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rv3029_of_match);
    static struct i2c_driver rv3029_driver = {
    .driver = {
    .name = "rv3029",
    .of_match_table = of_match_ptr(rv3029_of_match),
    },
    .probe		= rv3029_i2c_probe,
    .id_table	= rv3029_id,
    };
#[no_mangle]
unsafe extern "C" fn rv3029_register_driver() -> int __init {
    static int __init rv3029_register_driver(void)
    {
    return i2c_add_driver(&rv3029_driver);
    }
#[no_mangle]
unsafe extern "C" fn rv3029_unregister_driver() {
    static void rv3029_unregister_driver(void)
    {
    i2c_del_driver(&rv3029_driver);
    }

#[no_mangle]
unsafe extern "C" fn rv3029_register_driver() -> int __init {
    static int __init rv3029_register_driver(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3029_unregister_driver() {
    static void rv3029_unregister_driver(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn rv3049_probe(spi: *mut spi_device) -> c_int {
    static int rv3049_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return rv3029_probe(&spi.dev, regmap, spi.irq, "rv3049");
    }
    static struct spi_driver rv3049_driver = {
    .driver = {
    .name    = "rv3049",
    },
    .probe   = rv3049_probe,
    };
#[no_mangle]
unsafe extern "C" fn rv3049_register_driver() -> int __init {
    static int __init rv3049_register_driver(void)
    {
    return spi_register_driver(&rv3049_driver);
    }
#[no_mangle]
unsafe extern "C" fn rv3049_unregister_driver() -> void __exit {
    static void __exit rv3049_unregister_driver(void)
    {
    spi_unregister_driver(&rv3049_driver);
    }

#[no_mangle]
unsafe extern "C" fn rv3049_register_driver() -> int __init {
    static int __init rv3049_register_driver(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv3049_unregister_driver() -> void __exit {
    static void __exit rv3049_unregister_driver(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn rv30x9_init() -> int __init {
    static int __init rv30x9_init(void)
    {
    int ret;
    ret = rv3029_register_driver();
    if (ret)
    return ret;
    ret = rv3049_register_driver();
    if (ret)
    rv3029_unregister_driver();
    return ret;
    }
    module_init(rv30x9_init)
#[no_mangle]
unsafe extern "C" fn rv30x9_exit() -> void __exit {
    static void __exit rv30x9_exit(void)
    {
    rv3049_unregister_driver();
    rv3029_unregister_driver();
    }
    module_exit(rv30x9_exit)
    MODULE_AUTHOR("Gregory Hermant <gregory.hermant@calao-systems.com>");
    MODULE_AUTHOR("Michael Buesch <m@bues.ch>");
    MODULE_DESCRIPTION("Micro Crystal RV3029/RV3049 RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:rv3049");
