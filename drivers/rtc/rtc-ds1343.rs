//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1343.c
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
// rtc-ds1343.c
//
// Driver for Dallas Semiconductor DS1343 Low Current, SPI Compatible
// Real Time Clock
//
// Author : Raghavendra Chandra Ganiga <ravi23ganiga@gmail.com>
// Ankur Srivastava <sankurece@gmail.com> : DS1343 Nvram Support
//

pub const DALLAS_MAXIM_DS1343: c_int = 0;
pub const DALLAS_MAXIM_DS1344: c_int = 1;
// RTC DS1343 Registers
pub const DS1343_SECONDS_REG: c_uint = 0x00;
pub const DS1343_MINUTES_REG: c_uint = 0x01;
pub const DS1343_HOURS_REG: c_uint = 0x02;
pub const DS1343_DAY_REG: c_uint = 0x03;
pub const DS1343_DATE_REG: c_uint = 0x04;
pub const DS1343_MONTH_REG: c_uint = 0x05;
pub const DS1343_YEAR_REG: c_uint = 0x06;
pub const DS1343_ALM0_SEC_REG: c_uint = 0x07;
pub const DS1343_ALM0_MIN_REG: c_uint = 0x08;
pub const DS1343_ALM0_HOUR_REG: c_uint = 0x09;
pub const DS1343_ALM0_DAY_REG: c_uint = 0x0A;
pub const DS1343_ALM1_SEC_REG: c_uint = 0x0B;
pub const DS1343_ALM1_MIN_REG: c_uint = 0x0C;
pub const DS1343_ALM1_HOUR_REG: c_uint = 0x0D;
pub const DS1343_ALM1_DAY_REG: c_uint = 0x0E;
pub const DS1343_CONTROL_REG: c_uint = 0x0F;
pub const DS1343_STATUS_REG: c_uint = 0x10;
pub const DS1343_TRICKLE_REG: c_uint = 0x11;
pub const DS1343_NVRAM: c_uint = 0x20;
pub const DS1343_NVRAM_LEN: c_int = 96;
// DS1343 Control Registers bits
pub const DS1343_EOSC: c_uint = 0x80;
pub const DS1343_DOSF: c_uint = 0x20;
pub const DS1343_EGFIL: c_uint = 0x10;
pub const DS1343_SQW: c_uint = 0x08;
pub const DS1343_INTCN: c_uint = 0x04;
pub const DS1343_A1IE: c_uint = 0x02;
pub const DS1343_A0IE: c_uint = 0x01;
// DS1343 Status Registers bits
pub const DS1343_OSF: c_uint = 0x80;
pub const DS1343_IRQF1: c_uint = 0x02;
pub const DS1343_IRQF0: c_uint = 0x01;
// DS1343 Trickle Charger Registers bits
pub const DS1343_TRICKLE_MAGIC: c_uint = 0xa0;
pub const DS1343_TRICKLE_DS1: c_uint = 0x08;
pub const DS1343_TRICKLE_1K: c_uint = 0x01;
pub const DS1343_TRICKLE_2K: c_uint = 0x02;
pub const DS1343_TRICKLE_4K: c_uint = 0x03;
    static const struct spi_device_id ds1343_id[] = {
    { "ds1343", DALLAS_MAXIM_DS1343 },
    { "ds1344", DALLAS_MAXIM_DS1344 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ds1343_id);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1343_priv {
    pub rtc: *mut rtc_device,
    pub map: *mut regmap,
    pub irq: c_int,
}

    static ssize_t ds1343_show_glitchfilter(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev.parent);
    int glitch_filt_status, data;
    int res;
    res = regmap_read(priv.map, DS1343_CONTROL_REG, &data);
    if (res)
    return res;
    glitch_filt_status = !!(data & DS1343_EGFIL);
    if (glitch_filt_status)
    return sprintf(buf, "enabled\n");
    else
    return sprintf(buf, "disabled\n");
    }
    static ssize_t ds1343_store_glitchfilter(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev.parent);
    let mut data: c_int = 0;
    int res;
    if (strncmp(buf, "enabled", 7) == 0)
    data = DS1343_EGFIL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(buf, _arg: "disabled", _arg: 8)) -> else {
    else if (strncmp(buf, "disabled", 8))
    return -EINVAL;
    res = regmap_update_bits(priv.map, DS1343_CONTROL_REG,
    DS1343_EGFIL, data);
    if (res)
    return res;
    return count;
    }
    static DEVICE_ATTR(glitch_filter, 0644, ds1343_show_glitchfilter,
    ds1343_store_glitchfilter);
    static int ds1343_nvram_write(void *priv, unsigned int off, void *val,
    size_t bytes)
    {
    struct ds1343_priv *ds1343 = priv;
    return regmap_bulk_write(ds1343.map, DS1343_NVRAM + off, val, bytes);
    }
    static int ds1343_nvram_read(void *priv, unsigned int off, void *val,
    size_t bytes)
    {
    struct ds1343_priv *ds1343 = priv;
    return regmap_bulk_read(ds1343.map, DS1343_NVRAM + off, val, bytes);
    }
    static ssize_t ds1343_show_tricklecharger(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev.parent);
    int res, data;
    char *diodes = "disabled", *resistors = " ";
    res = regmap_read(priv.map, DS1343_TRICKLE_REG, &data);
    if (res)
    return res;
    if ((data & 0xf0) == DS1343_TRICKLE_MAGIC) {
    switch (data & 0x0c) {
    case DS1343_TRICKLE_DS1:
    diodes = "one diode,";
    break;
    default:
    diodes = "no diode,";
    break;
    }
    switch (data & 0x03) {
    case DS1343_TRICKLE_1K:
    resistors = "1k Ohm";
    break;
    case DS1343_TRICKLE_2K:
    resistors = "2k Ohm";
    break;
    case DS1343_TRICKLE_4K:
    resistors = "4k Ohm";
    break;
    default:
    diodes = "disabled";
    break;
    }
    }
    return sprintf(buf, "%s %s\n", diodes, resistors);
    }
    static DEVICE_ATTR(trickle_charger, 0444, ds1343_show_tricklecharger, core::ptr::null_mut());
    static struct attribute *ds1343_attrs[] = {
    &dev_attr_glitch_filter.attr,
    &dev_attr_trickle_charger.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ds1343_attr_group = {
    .attrs  = ds1343_attrs,
    };
#[no_mangle]
unsafe extern "C" fn ds1343_read_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds1343_read_time(struct device *dev, struct rtc_time *dt)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev);
    unsigned char buf[7];
    int res;
    res = regmap_bulk_read(priv.map, DS1343_SECONDS_REG, buf, 7);
    if (res)
    return res;
    dt.tm_sec	= bcd2bin(buf[0]);
    dt.tm_min	= bcd2bin(buf[1]);
    dt.tm_hour	= bcd2bin(buf[2] & 0x3F);
    dt.tm_wday	= bcd2bin(buf[3]) - 1;
    dt.tm_mday	= bcd2bin(buf[4]);
    dt.tm_mon	= bcd2bin(buf[5] & 0x1F) - 1;
    dt.tm_year	= bcd2bin(buf[6]) + 100; /* year offset from 1900 */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1343_set_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds1343_set_time(struct device *dev, struct rtc_time *dt)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev);
    u8 buf[7];
    buf[0] = bin2bcd(dt.tm_sec);
    buf[1] = bin2bcd(dt.tm_min);
    buf[2] = bin2bcd(dt.tm_hour) & 0x3F;
    buf[3] = bin2bcd(dt.tm_wday + 1);
    buf[4] = bin2bcd(dt.tm_mday);
    buf[5] = bin2bcd(dt.tm_mon + 1);
    buf[6] = bin2bcd(dt.tm_year - 100);
    return regmap_bulk_write(priv.map, DS1343_SECONDS_REG,
    buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn ds1343_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ds1343_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev);
    unsigned char buf[4];
    unsigned int val;
    int res;
    if (priv.irq <= 0)
    return -EINVAL;
    res = regmap_read(priv.map, DS1343_STATUS_REG, &val);
    if (res)
    return res;
    alarm.pending = !!(val & DS1343_IRQF0);
    res = regmap_read(priv.map, DS1343_CONTROL_REG, &val);
    if (res)
    return res;
    alarm.enabled = !!(val & DS1343_A0IE);
    res = regmap_bulk_read(priv.map, DS1343_ALM0_SEC_REG, buf, 4);
    if (res)
    return res;
    alarm.time.tm_sec = bcd2bin(buf[0]) & 0x7f;
    alarm.time.tm_min = bcd2bin(buf[1]) & 0x7f;
    alarm.time.tm_hour = bcd2bin(buf[2]) & 0x3f;
    alarm.time.tm_mday = bcd2bin(buf[3]) & 0x3f;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1343_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ds1343_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev);
    unsigned char buf[4];
    let mut res: c_int = 0;
    if (priv.irq <= 0)
    return -EINVAL;
    res = regmap_update_bits(priv.map, DS1343_CONTROL_REG, DS1343_A0IE, 0);
    if (res)
    return res;
    buf[0] = bin2bcd(alarm.time.tm_sec);
    buf[1] = bin2bcd(alarm.time.tm_min);
    buf[2] = bin2bcd(alarm.time.tm_hour);
    buf[3] = bin2bcd(alarm.time.tm_mday);
    res = regmap_bulk_write(priv.map, DS1343_ALM0_SEC_REG, buf, 4);
    if (res)
    return res;
    if (alarm.enabled)
    res = regmap_update_bits(priv.map, DS1343_CONTROL_REG,
    DS1343_A0IE, DS1343_A0IE);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn ds1343_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int ds1343_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct ds1343_priv *priv = dev_get_drvdata(dev);
    if (priv.irq <= 0)
    return -EINVAL;
    return regmap_update_bits(priv.map, DS1343_CONTROL_REG,
    DS1343_A0IE, enabled ? DS1343_A0IE : 0);
    }
#[no_mangle]
unsafe extern "C" fn ds1343_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ds1343_thread(int irq, void *dev_id)
    {
    struct ds1343_priv *priv = dev_id;
    unsigned int stat;
    let mut res: c_int = 0;
    rtc_lock(priv.rtc);
    res = regmap_read(priv.map, DS1343_STATUS_REG, &stat);
    if (res)
    goto out;
    if (stat & DS1343_IRQF0) {
    stat &= ~DS1343_IRQF0;
    regmap_write(priv.map, DS1343_STATUS_REG, stat);
    rtc_update_irq(priv.rtc, 1, RTC_AF | RTC_IRQF);
    regmap_update_bits(priv.map, DS1343_CONTROL_REG,
    DS1343_A0IE, 0);
    }
    out:
    rtc_unlock(priv.rtc);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops ds1343_rtc_ops = {
    .read_time	= ds1343_read_time,
    .set_time	= ds1343_set_time,
    .read_alarm	= ds1343_read_alarm,
    .set_alarm	= ds1343_set_alarm,
    .alarm_irq_enable = ds1343_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn ds1343_probe(spi: *mut spi_device) -> c_int {
    static int ds1343_probe(struct spi_device *spi)
    {
    struct ds1343_priv *priv;
    struct regmap_config config = { .reg_bits = 8, .val_bits = 8,
    .write_flag_mask = 0x80, };
    unsigned int data;
    int res;
    struct nvmem_config nvmem_cfg = {
    .name = "ds1343-",
    .word_size = 1,
    .stride = 1,
    .size = DS1343_NVRAM_LEN,
    .reg_read = ds1343_nvram_read,
    .reg_write = ds1343_nvram_write,
    };
    priv = devm_kzalloc(&spi.dev, sizeof(struct ds1343_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// RTC DS1347 works in spi mode 3 and
// its chip select is active high. Active high should be defined as
// "inverse polarity" as GPIO-based chip selects can be logically
// active high but inverted by the GPIO library.
//
    spi.mode |= SPI_MODE_3;
    spi.mode ^= SPI_CS_HIGH;
    spi.bits_per_word = 8;
    res = spi_setup(spi);
    if (res)
    return res;
    spi_set_drvdata(spi, priv);
    priv.map = devm_regmap_init_spi(spi, &config);
    if (IS_ERR(priv.map)) {
    dev_err(&spi.dev, "spi regmap init failed for rtc ds1343\n");
    return PTR_ERR(priv.map);
    }
    res = regmap_read(priv.map, DS1343_SECONDS_REG, &data);
    if (res)
    return res;
    regmap_read(priv.map, DS1343_CONTROL_REG, &data);
    data |= DS1343_INTCN;
    data &= ~(DS1343_EOSC | DS1343_A1IE | DS1343_A0IE);
    regmap_write(priv.map, DS1343_CONTROL_REG, data);
    regmap_read(priv.map, DS1343_STATUS_REG, &data);
    data &= ~(DS1343_OSF | DS1343_IRQF1 | DS1343_IRQF0);
    regmap_write(priv.map, DS1343_STATUS_REG, data);
    priv.rtc = devm_rtc_allocate_device(&spi.dev);
    if (IS_ERR(priv.rtc))
    return PTR_ERR(priv.rtc);
    priv.rtc.ops = &ds1343_rtc_ops;
    priv.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    priv.rtc.range_max = RTC_TIMESTAMP_END_2099;
    res = rtc_add_group(priv.rtc, &ds1343_attr_group);
    if (res)
    dev_err(&spi.dev,
    "unable to create sysfs entries for rtc ds1343\n");
    res = devm_rtc_register_device(priv.rtc);
    if (res)
    return res;
    nvmem_cfg.priv = priv;
    devm_rtc_nvmem_register(priv.rtc, &nvmem_cfg);
    priv.irq = spi.irq;
    if (priv.irq >= 0) {
    res = devm_request_threaded_irq(&spi.dev, spi.irq, core::ptr::null_mut(),
    ds1343_thread, IRQF_ONESHOT,
    "ds1343", priv);
    if (res) {
    priv.irq = -1;
    dev_err(&spi.dev,
    "unable to request irq for rtc ds1343\n");
    } else {
    device_init_wakeup(&spi.dev, true);
    devm_pm_set_wake_irq(&spi.dev, spi.irq);
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ds1343_suspend(dev: *mut device) -> c_int {
    static int ds1343_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    if (spi.irq >= 0 && device_may_wakeup(dev))
    enable_irq_wake(spi.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1343_resume(dev: *mut device) -> c_int {
    static int ds1343_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    if (spi.irq >= 0 && device_may_wakeup(dev))
    disable_irq_wake(spi.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(ds1343_pm, ds1343_suspend, ds1343_resume);
    static struct spi_driver ds1343_driver = {
    .driver = {
    .name = "ds1343",
    .pm = &ds1343_pm,
    },
    .probe = ds1343_probe,
    .id_table = ds1343_id,
    };
    module_spi_driver(ds1343_driver);
    MODULE_DESCRIPTION("DS1343 RTC SPI Driver");
    MODULE_AUTHOR("Raghavendra Chandra Ganiga <ravi23ganiga@gmail.com>,"
    "Ankur Srivastava <sankurece@gmail.com>");
    MODULE_LICENSE("GPL v2");
