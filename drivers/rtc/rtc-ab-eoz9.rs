//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ab-eoz9.c
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
// Real Time Clock driver for AB-RTCMC-32.768kHz-EOZ9 chip.
// Copyright (C) 2019 Orolia
//

pub const ABEOZ9_REG_CTRL1: c_uint = 0x00;

pub const ABEOZ9_REG_CTRL_INT: c_uint = 0x01;

pub const ABEOZ9_REG_CTRL_INT_FLAG: c_uint = 0x02;

pub const ABEOZ9_REG_CTRL_STATUS: c_uint = 0x03;

pub const ABEOZ9_REG_SEC: c_uint = 0x08;
pub const ABEOZ9_REG_MIN: c_uint = 0x09;
pub const ABEOZ9_REG_HOURS: c_uint = 0x0A;

pub const ABEOZ9_REG_DAYS: c_uint = 0x0B;
pub const ABEOZ9_REG_WEEKDAYS: c_uint = 0x0C;
pub const ABEOZ9_REG_MONTHS: c_uint = 0x0D;
pub const ABEOZ9_REG_YEARS: c_uint = 0x0E;
pub const ABEOZ9_SEC_LEN: c_int = 7;
pub const ABEOZ9_REG_ALARM_SEC: c_uint = 0x10;

pub const ABEOZ9_REG_ALARM_MIN: c_uint = 0x11;

pub const ABEOZ9_REG_ALARM_HOURS: c_uint = 0x12;

pub const ABEOZ9_REG_ALARM_DAYS: c_uint = 0x13;

pub const ABEOZ9_REG_ALARM_WEEKDAYS: c_uint = 0x14;

pub const ABEOZ9_REG_ALARM_MONTHS: c_uint = 0x15;

pub const ABEOZ9_REG_ALARM_YEARS: c_uint = 0x16;
pub const ABEOZ9_ALARM_LEN: c_int = 7;

pub const ABEOZ9_REG_REG_TEMP: c_uint = 0x20;
pub const ABEOZ953_TEMP_MAX: c_int = 120;

pub const ABEOZ9_REG_EEPROM: c_uint = 0x30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abeoz9_rtc_data {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
    pub hwmon_dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn abeoz9_check_validity(dev: *mut device) -> c_int {
    static int abeoz9_check_validity(struct device *dev)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int ret;
    int val;
    ret = regmap_read(regmap, ABEOZ9_REG_CTRL_STATUS, &val);
    if (ret < 0) {
    dev_err(dev,
    "unable to get CTRL_STATUS register (%d)\n", ret);
    return ret;
    }
    if (val & ABEOZ9_REG_CTRL_STATUS_PON) {
    dev_warn(dev, "power-on reset detected, date is invalid\n");
    return -EINVAL;
    }
    if (val & ABEOZ9_REG_CTRL_STATUS_V1F) {
    dev_warn(dev,
    "voltage drops below VLOW1 threshold, date is invalid\n");
    return -EINVAL;
    }
    if ((val & ABEOZ9_REG_CTRL_STATUS_V2F)) {
    dev_warn(dev,
    "voltage drops below VLOW2 threshold, date is invalid\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_reset_validity(regmap: *mut regmap) -> c_int {
    static int abeoz9_reset_validity(struct regmap *regmap)
    {
    return regmap_update_bits(regmap, ABEOZ9_REG_CTRL_STATUS,
    ABEOZ9_REG_CTRL_STATUS_V1F |
    ABEOZ9_REG_CTRL_STATUS_V2F |
    ABEOZ9_REG_CTRL_STATUS_PON,
    0);
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int abeoz9_rtc_get_time(struct device *dev, struct rtc_time *tm)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    u8 regs[ABEOZ9_SEC_LEN];
    int ret;
    ret = abeoz9_check_validity(dev);
    if (ret)
    return ret;
    ret = regmap_bulk_read(data.regmap, ABEOZ9_REG_SEC,
    regs,
    sizeof(regs));
    if (ret) {
    dev_err(dev, "reading RTC time failed (%d)\n", ret);
    return ret;
    }
    tm.tm_sec = bcd2bin(regs[ABEOZ9_REG_SEC - ABEOZ9_REG_SEC] & 0x7F);
    tm.tm_min = bcd2bin(regs[ABEOZ9_REG_MIN - ABEOZ9_REG_SEC] & 0x7F);
    if (regs[ABEOZ9_REG_HOURS - ABEOZ9_REG_SEC] & ABEOZ9_HOURS_PM) {
    tm.tm_hour =
    bcd2bin(regs[ABEOZ9_REG_HOURS - ABEOZ9_REG_SEC] & 0x1f);
    if (regs[ABEOZ9_REG_HOURS - ABEOZ9_REG_SEC] & ABEOZ9_HOURS_PM)
    tm.tm_hour += 12;
    } else {
    tm.tm_hour = bcd2bin(regs[ABEOZ9_REG_HOURS - ABEOZ9_REG_SEC]);
    }
    tm.tm_mday = bcd2bin(regs[ABEOZ9_REG_DAYS - ABEOZ9_REG_SEC]);
    tm.tm_wday = bcd2bin(regs[ABEOZ9_REG_WEEKDAYS - ABEOZ9_REG_SEC]);
    tm.tm_mon  = bcd2bin(regs[ABEOZ9_REG_MONTHS - ABEOZ9_REG_SEC]) - 1;
    tm.tm_year = bcd2bin(regs[ABEOZ9_REG_YEARS - ABEOZ9_REG_SEC]) + 100;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int abeoz9_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u8 regs[ABEOZ9_SEC_LEN];
    int ret;
    regs[ABEOZ9_REG_SEC - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_sec);
    regs[ABEOZ9_REG_MIN - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_min);
    regs[ABEOZ9_REG_HOURS - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_hour);
    regs[ABEOZ9_REG_DAYS - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_mday);
    regs[ABEOZ9_REG_WEEKDAYS - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_wday);
    regs[ABEOZ9_REG_MONTHS - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_mon + 1);
    regs[ABEOZ9_REG_YEARS - ABEOZ9_REG_SEC] = bin2bcd(tm.tm_year - 100);
    ret = regmap_bulk_write(data.regmap, ABEOZ9_REG_SEC,
    regs,
    sizeof(regs));
    if (ret) {
    dev_err(dev, "set RTC time failed (%d)\n", ret);
    return ret;
    }
    return abeoz9_reset_validity(regmap);
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int abeoz9_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u8 regs[ABEOZ9_ALARM_LEN];
    u8 val[2];
    int ret;
    ret = abeoz9_check_validity(dev);
    if (ret)
    return ret;
    ret = regmap_bulk_read(regmap, ABEOZ9_REG_CTRL_INT, val, sizeof(val));
    if (ret)
    return ret;
    alarm.enabled = val[0] & ABEOZ9_REG_CTRL_INT_AIE;
    alarm.pending = val[1] & ABEOZ9_REG_CTRL_INT_FLAG_AF;
    ret = regmap_bulk_read(regmap, ABEOZ9_REG_ALARM_SEC, regs, sizeof(regs));
    if (ret)
    return ret;
    alarm.time.tm_sec = bcd2bin(FIELD_GET(ABEOZ9_BIT_ALARM_SEC, regs[0]));
    alarm.time.tm_min = bcd2bin(FIELD_GET(ABEOZ9_BIT_ALARM_MIN, regs[1]));
    alarm.time.tm_hour = bcd2bin(FIELD_GET(ABEOZ9_BIT_ALARM_HOURS, regs[2]));
    alarm.time.tm_mday = bcd2bin(FIELD_GET(ABEOZ9_BIT_ALARM_DAYS, regs[3]));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_alarm_irq_enable(dev: *mut device, enable: u32) -> c_int {
    static int abeoz9_rtc_alarm_irq_enable(struct device *dev, u32 enable)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    return regmap_update_bits(data.regmap, ABEOZ9_REG_CTRL_INT,
    ABEOZ9_REG_CTRL_INT_AIE,
    FIELD_PREP(ABEOZ9_REG_CTRL_INT_AIE, enable));
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int abeoz9_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    u8 regs[ABEOZ9_ALARM_LEN] = {0};
    int ret;
    ret = regmap_update_bits(data.regmap, ABEOZ9_REG_CTRL_INT_FLAG,
    ABEOZ9_REG_CTRL_INT_FLAG_AF, 0);
    if (ret)
    return ret;
    regs[0] = ABEOZ9_BIT_ALARM_AE | FIELD_PREP(ABEOZ9_BIT_ALARM_SEC,
    bin2bcd(alarm.time.tm_sec));
    regs[1] = ABEOZ9_BIT_ALARM_AE | FIELD_PREP(ABEOZ9_BIT_ALARM_MIN,
    bin2bcd(alarm.time.tm_min));
    regs[2] = ABEOZ9_BIT_ALARM_AE | FIELD_PREP(ABEOZ9_BIT_ALARM_HOURS,
    bin2bcd(alarm.time.tm_hour));
    regs[3] = ABEOZ9_BIT_ALARM_AE | FIELD_PREP(ABEOZ9_BIT_ALARM_DAYS,
    bin2bcd(alarm.time.tm_mday));
    ret = regmap_bulk_write(data.regmap, ABEOZ9_REG_ALARM_SEC, regs,
    sizeof(regs));
    if (ret)
    return ret;
    return abeoz9_rtc_alarm_irq_enable(dev, alarm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t abeoz9_rtc_irq(int irq, void *dev)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    unsigned int val;
    int ret;
    ret = regmap_read(data.regmap, ABEOZ9_REG_CTRL_INT_FLAG, &val);
    if (ret)
    return IRQ_NONE;
    if (!FIELD_GET(ABEOZ9_REG_CTRL_INT_FLAG_AF, val))
    return IRQ_NONE;
    regmap_update_bits(data.regmap, ABEOZ9_REG_CTRL_INT_FLAG,
    ABEOZ9_REG_CTRL_INT_FLAG_AF, 0);
    rtc_update_irq(data.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_trickle_parse_dt(node: *mut device_node) -> c_int {
    static int abeoz9_trickle_parse_dt(struct device_node *node)
    {
    let mut ohms: u32 = 0;
    if (of_property_read_u32(node, "trickle-resistor-ohms", &ohms))
    return 0;
    switch (ohms) {
    case 1000:
    return ABEOZ9_REG_EEPROM_R1K;
    case 5000:
    return ABEOZ9_REG_EEPROM_R5K;
    case 20000:
    return ABEOZ9_REG_EEPROM_R20K;
    case 80000:
    return ABEOZ9_REG_EEPROM_R80K;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn abeoz9_rtc_setup(dev: *mut device, node: *mut device_node) -> c_int {
    static int abeoz9_rtc_setup(struct device *dev, struct device_node *node)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int ret;
// Enable Self Recovery, Clock for Watch and EEPROM refresh functions
    ret = regmap_update_bits(regmap, ABEOZ9_REG_CTRL1,
    ABEOZ9_REG_CTRL1_MASK,
    ABEOZ9_REG_CTRL1_WE |
    ABEOZ9_REG_CTRL1_EERE |
    ABEOZ9_REG_CTRL1_SRON);
    if (ret < 0) {
    dev_err(dev, "unable to set CTRL_1 register (%d)\n", ret);
    return ret;
    }
    ret = regmap_write(regmap, ABEOZ9_REG_CTRL_INT, 0);
    if (ret < 0) {
    dev_err(dev,
    "unable to set control CTRL_INT register (%d)\n",
    ret);
    return ret;
    }
    ret = regmap_write(regmap, ABEOZ9_REG_CTRL_INT_FLAG, 0);
    if (ret < 0) {
    dev_err(dev,
    "unable to set control CTRL_INT_FLAG register (%d)\n",
    ret);
    return ret;
    }
    ret = abeoz9_trickle_parse_dt(node);
// Enable built-in termometer
    ret |= ABEOZ9_REG_EEPROM_THE;
    ret = regmap_update_bits(regmap, ABEOZ9_REG_EEPROM,
    ABEOZ9_REG_EEPROM_MASK,
    ret);
    if (ret < 0) {
    dev_err(dev, "unable to set EEPROM register (%d)\n", ret);
    return ret;
    }
    return ret;
    }
    static const struct rtc_class_ops rtc_ops = {
    .read_time = abeoz9_rtc_get_time,
    .set_time = abeoz9_rtc_set_time,
    .read_alarm = abeoz9_rtc_read_alarm,
    .set_alarm = abeoz9_rtc_set_alarm,
    .alarm_irq_enable = abeoz9_rtc_alarm_irq_enable,
    };
    static const struct regmap_config abeoz9_rtc_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x3f,
    };

    static int abeoz9z3_temp_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    struct abeoz9_rtc_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int ret;
    unsigned int val;
    ret = regmap_read(regmap, ABEOZ9_REG_CTRL_STATUS, &val);
    if (ret < 0)
    return ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_read(regmap, ABEOZ9_REG_REG_TEMP, &val);
    if (ret < 0)
    return ret;
// temp = 1000 * (val + ABEOZ953_TEMP_MIN);
    return 0;
    case hwmon_temp_max:
// temp = 1000 * ABEOZ953_TEMP_MAX;
    return 0;
    case hwmon_temp_min:
// temp = 1000 * ABEOZ953_TEMP_MIN;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t abeoz9_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_max:
    case hwmon_temp_min:
    return 0444;
    default:
    return 0;
    }
    }
    static const struct hwmon_channel_info * const abeoz9_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops abeoz9_hwmon_ops = {
    .is_visible = abeoz9_is_visible,
    .read = abeoz9z3_temp_read,
    };
    static const struct hwmon_chip_info abeoz9_chip_info = {
    .ops = &abeoz9_hwmon_ops,
    .info = abeoz9_info,
    };
    static void abeoz9_hwmon_register(struct device *dev,
    struct abeoz9_rtc_data *data)
    {
    data.hwmon_dev =
    devm_hwmon_device_register_with_info(dev,
    "abeoz9",
    data,
    &abeoz9_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(data.hwmon_dev)) {
    dev_warn(dev, "unable to register hwmon device %ld\n",
    PTR_ERR(data.hwmon_dev));
    }
    }

    static void abeoz9_hwmon_register(struct device *dev,
    struct abeoz9_rtc_data *data)
    {
    }

#[no_mangle]
unsafe extern "C" fn abeoz9_probe(client: *mut i2c_client) -> c_int {
    static int abeoz9_probe(struct i2c_client *client)
    {
    struct abeoz9_rtc_data *data = core::ptr::null_mut();
    struct device *dev = &client.dev;
    struct regmap *regmap;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_I2C_BLOCK))
    return -ENODEV;
    regmap = devm_regmap_init_i2c(client, &abeoz9_rtc_regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(dev, "regmap allocation failed: %d\n", ret);
    return ret;
    }
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = regmap;
    dev_set_drvdata(dev, data);
    ret = abeoz9_rtc_setup(dev, client.dev.of_node);
    if (ret)
    return ret;
    data.rtc = devm_rtc_allocate_device(dev);
    ret = PTR_ERR_OR_ZERO(data.rtc);
    if (ret)
    return ret;
    data.rtc.ops = &rtc_ops;
    data.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    data.rtc.range_max = RTC_TIMESTAMP_END_2099;
    clear_bit(RTC_FEATURE_ALARM, data.rtc.features);
    if (client.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&client.dev))
    irqflags = 0;
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    abeoz9_rtc_irq,
    irqflags | IRQF_ONESHOT,
    dev_name(dev), dev);
    if (ret) {
    dev_err(dev, "failed to request alarm irq\n");
    return ret;
    }
    } else {
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, data.rtc.features);
    }
    if (client.irq > 0 || device_property_read_bool(dev, "wakeup-source")) {
    ret = device_init_wakeup(dev, true);
    set_bit(RTC_FEATURE_ALARM, data.rtc.features);
    }
    ret = devm_rtc_register_device(data.rtc);
    if (ret)
    return ret;
    abeoz9_hwmon_register(dev, data);
    return 0;
    }

    static const struct of_device_id abeoz9_dt_match[] = {
    { .compatible = "abracon,abeoz9" },
    { },
    };
    MODULE_DEVICE_TABLE(of, abeoz9_dt_match);

    static const struct i2c_device_id abeoz9_id[] = {
    { .name = "abeoz9" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, abeoz9_id);
    static struct i2c_driver abeoz9_driver = {
    .driver = {
    .name = "rtc-ab-eoz9",
    .of_match_table = of_match_ptr(abeoz9_dt_match),
    },
    .probe = abeoz9_probe,
    .id_table = abeoz9_id,
    };
    module_i2c_driver(abeoz9_driver);
    MODULE_AUTHOR("Artem Panfilov <panfilov.artyom@gmail.com>");
    MODULE_DESCRIPTION("Abracon AB-RTCMC-32.768kHz-EOZ9 RTC driver");
    MODULE_LICENSE("GPL");
