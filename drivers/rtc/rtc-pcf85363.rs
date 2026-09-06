//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pcf85363.c
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
// drivers/rtc/rtc-pcf85363.c
//
// Driver for NXP PCF85363 real-time clock.
//
// Copyright (C) 2017 Eric Nelson
//

//
// Date/Time registers
//
pub const DT_100THS: c_uint = 0x00;
pub const DT_SECS: c_uint = 0x01;
pub const DT_MINUTES: c_uint = 0x02;
pub const DT_HOURS: c_uint = 0x03;
pub const DT_DAYS: c_uint = 0x04;
pub const DT_WEEKDAYS: c_uint = 0x05;
pub const DT_MONTHS: c_uint = 0x06;
pub const DT_YEARS: c_uint = 0x07;
//
// Alarm registers
//
pub const DT_SECOND_ALM1: c_uint = 0x08;
pub const DT_MINUTE_ALM1: c_uint = 0x09;
pub const DT_HOUR_ALM1: c_uint = 0x0a;
pub const DT_DAY_ALM1: c_uint = 0x0b;
pub const DT_MONTH_ALM1: c_uint = 0x0c;
pub const DT_MINUTE_ALM2: c_uint = 0x0d;
pub const DT_HOUR_ALM2: c_uint = 0x0e;
pub const DT_WEEKDAY_ALM2: c_uint = 0x0f;
pub const DT_ALARM_EN: c_uint = 0x10;
//
// Time stamp registers
//
pub const DT_TIMESTAMP1: c_uint = 0x11;
pub const DT_TIMESTAMP2: c_uint = 0x17;
pub const DT_TIMESTAMP3: c_uint = 0x1d;
pub const DT_TS_MODE: c_uint = 0x23;
//
// control registers
//
pub const CTRL_OFFSET: c_uint = 0x24;
pub const CTRL_OSCILLATOR: c_uint = 0x25;
pub const CTRL_BATTERY: c_uint = 0x26;
pub const CTRL_PIN_IO: c_uint = 0x27;
pub const CTRL_FUNCTION: c_uint = 0x28;
pub const CTRL_INTA_EN: c_uint = 0x29;
pub const CTRL_INTB_EN: c_uint = 0x2a;
pub const CTRL_FLAGS: c_uint = 0x2b;
pub const CTRL_RAMBYTE: c_uint = 0x2c;
pub const CTRL_WDOG: c_uint = 0x2d;
pub const CTRL_STOP_EN: c_uint = 0x2e;
pub const CTRL_RESETS: c_uint = 0x2f;
pub const CTRL_RAM: c_uint = 0x40;

pub const PIN_IO_INTA_CLK: c_int = 0;
pub const PIN_IO_INTA_BAT: c_int = 1;
pub const PIN_IO_INTA_OUT: c_int = 2;
pub const PIN_IO_INTA_HIZ: c_int = 3;

pub const OSC_CAP_6000: c_uint = 0x01;
pub const OSC_CAP_12500: c_uint = 0x02;

pub const RESET_CPR: c_uint = 0xa4;
pub const NVRAM_SIZE: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf85363 {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcf85x63_config {
    pub regmap: regmap_config,
    pub num_nvram: c_uint,
}

#[no_mangle]
unsafe extern "C" fn pcf85363_load_capacitance(pcf85363: *mut pcf85363, node: *mut device_node) -> c_int {
    static int pcf85363_load_capacitance(struct pcf85363 *pcf85363, struct device_node *node)
    {
    let mut load: u32 = 7000;
    let mut value: u8 = 0;
    of_property_read_u32(node, "quartz-load-femtofarads", &load);
    switch (load) {
    default:
    dev_warn(&pcf85363.rtc.dev, "Unknown quartz-load-femtofarads value: %d. Assuming 7000",
    load);
    fallthrough;
    case 7000:
    break;
    case 6000:
    value = OSC_CAP_6000;
    break;
    case 12500:
    value = OSC_CAP_12500;
    break;
    }
    return regmap_update_bits(pcf85363.regmap, CTRL_OSCILLATOR,
    OSC_CAP_SEL, value);
    }
#[no_mangle]
unsafe extern "C" fn pcf85363_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf85363_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf85363 *pcf85363 = dev_get_drvdata(dev);
    unsigned char buf[DT_YEARS + 1];
    int ret, len = sizeof(buf);
// read the RTC date and time registers all at once
    ret = regmap_bulk_read(pcf85363.regmap, DT_100THS, buf, len);
    if (ret) {
    dev_err(dev, "%s: error %d\n", __func__, ret);
    return ret;
    }
    tm.tm_year = bcd2bin(buf[DT_YEARS]);
// adjust for 1900 base of rtc_time
    tm.tm_year += 100;
    tm.tm_wday = buf[DT_WEEKDAYS] & 7;
    buf[DT_SECS] &= 0x7F;
    tm.tm_sec = bcd2bin(buf[DT_SECS]);
    buf[DT_MINUTES] &= 0x7F;
    tm.tm_min = bcd2bin(buf[DT_MINUTES]);
    tm.tm_hour = bcd2bin(buf[DT_HOURS]);
    tm.tm_mday = bcd2bin(buf[DT_DAYS]);
    tm.tm_mon = bcd2bin(buf[DT_MONTHS]) - 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf85363_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pcf85363_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pcf85363 *pcf85363 = dev_get_drvdata(dev);
    unsigned char tmp[11];
    unsigned char *buf = &tmp[2];
    int ret;
    tmp[0] = STOP_EN_STOP;
    tmp[1] = RESET_CPR;
    buf[DT_100THS] = 0;
    buf[DT_SECS] = bin2bcd(tm.tm_sec);
    buf[DT_MINUTES] = bin2bcd(tm.tm_min);
    buf[DT_HOURS] = bin2bcd(tm.tm_hour);
    buf[DT_DAYS] = bin2bcd(tm.tm_mday);
    buf[DT_WEEKDAYS] = tm.tm_wday;
    buf[DT_MONTHS] = bin2bcd(tm.tm_mon + 1);
    buf[DT_YEARS] = bin2bcd(tm.tm_year % 100);
    ret = regmap_bulk_write(pcf85363.regmap, CTRL_STOP_EN,
    tmp, 2);
    if (ret)
    return ret;
    ret = regmap_bulk_write(pcf85363.regmap, DT_100THS,
    buf, sizeof(tmp) - 2);
    if (ret)
    return ret;
    return regmap_write(pcf85363.regmap, CTRL_STOP_EN, 0);
    }
#[no_mangle]
unsafe extern "C" fn pcf85363_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pcf85363_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pcf85363 *pcf85363 = dev_get_drvdata(dev);
    unsigned char buf[DT_MONTH_ALM1 - DT_SECOND_ALM1 + 1];
    unsigned int val;
    int ret;
    ret = regmap_bulk_read(pcf85363.regmap, DT_SECOND_ALM1, buf,
    sizeof(buf));
    if (ret)
    return ret;
    alrm.time.tm_sec = bcd2bin(buf[0]);
    alrm.time.tm_min = bcd2bin(buf[1]);
    alrm.time.tm_hour = bcd2bin(buf[2]);
    alrm.time.tm_mday = bcd2bin(buf[3]);
    alrm.time.tm_mon = bcd2bin(buf[4]) - 1;
    ret = regmap_read(pcf85363.regmap, CTRL_INTA_EN, &val);
    if (ret)
    return ret;
    alrm.enabled =  !!(val & INT_A1IE);
    return 0;
    }
    static int _pcf85363_rtc_alarm_irq_enable(struct pcf85363 *pcf85363, unsigned
    int enabled)
    {
    unsigned int alarm_flags = ALRM_SEC_A1E | ALRM_MIN_A1E | ALRM_HR_A1E |
    ALRM_DAY_A1E | ALRM_MON_A1E;
    int ret;
    ret = regmap_update_bits(pcf85363.regmap, DT_ALARM_EN, alarm_flags,
    enabled ? alarm_flags : 0);
    if (ret)
    return ret;
    ret = regmap_update_bits(pcf85363.regmap, CTRL_INTA_EN,
    INT_A1IE, enabled ? INT_A1IE : 0);
    if (ret || enabled)
    return ret;
// clear current flags
    return regmap_update_bits(pcf85363.regmap, CTRL_FLAGS, FLAGS_A1F, 0);
    }
    static int pcf85363_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct pcf85363 *pcf85363 = dev_get_drvdata(dev);
    return _pcf85363_rtc_alarm_irq_enable(pcf85363, enabled);
    }
#[no_mangle]
unsafe extern "C" fn pcf85363_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pcf85363_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pcf85363 *pcf85363 = dev_get_drvdata(dev);
    unsigned char buf[DT_MONTH_ALM1 - DT_SECOND_ALM1 + 1];
    int ret;
    buf[0] = bin2bcd(alrm.time.tm_sec);
    buf[1] = bin2bcd(alrm.time.tm_min);
    buf[2] = bin2bcd(alrm.time.tm_hour);
    buf[3] = bin2bcd(alrm.time.tm_mday);
    buf[4] = bin2bcd(alrm.time.tm_mon + 1);
//
// Disable the alarm interrupt before changing the value to avoid
// spurious interrupts
//
    ret = _pcf85363_rtc_alarm_irq_enable(pcf85363, 0);
    if (ret)
    return ret;
    ret = regmap_bulk_write(pcf85363.regmap, DT_SECOND_ALM1, buf,
    sizeof(buf));
    if (ret)
    return ret;
    return _pcf85363_rtc_alarm_irq_enable(pcf85363, alrm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn pcf85363_rtc_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcf85363_rtc_handle_irq(int irq, void *dev_id)
    {
    struct pcf85363 *pcf85363 = i2c_get_clientdata(dev_id);
    unsigned int flags;
    int err;
    err = regmap_read(pcf85363.regmap, CTRL_FLAGS, &flags);
    if (err)
    return IRQ_NONE;
    if (flags & FLAGS_A1F) {
    rtc_update_irq(pcf85363.rtc, 1, RTC_IRQF | RTC_AF);
    regmap_update_bits(pcf85363.regmap, CTRL_FLAGS, FLAGS_A1F, 0);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
    static const struct rtc_class_ops rtc_ops = {
    .read_time	= pcf85363_rtc_read_time,
    .set_time	= pcf85363_rtc_set_time,
    .read_alarm	= pcf85363_rtc_read_alarm,
    .set_alarm	= pcf85363_rtc_set_alarm,
    .alarm_irq_enable = pcf85363_rtc_alarm_irq_enable,
    };
    static int pcf85363_nvram_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct pcf85363 *pcf85363 = priv;
    return regmap_bulk_read(pcf85363.regmap, CTRL_RAM + offset,
    val, bytes);
    }
    static int pcf85363_nvram_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct pcf85363 *pcf85363 = priv;
    return regmap_bulk_write(pcf85363.regmap, CTRL_RAM + offset,
    val, bytes);
    }
    static int pcf85x63_nvram_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct pcf85363 *pcf85363 = priv;
    unsigned int tmp_val;
    int ret;
    ret = regmap_read(pcf85363.regmap, CTRL_RAMBYTE, &tmp_val);
    (*(unsigned char *) val) = (unsigned char) tmp_val;
    return ret;
    }
    static int pcf85x63_nvram_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct pcf85363 *pcf85363 = priv;
    unsigned char tmp_val;
    tmp_val = *((unsigned char *)val);
    return regmap_write(pcf85363.regmap, CTRL_RAMBYTE,
    (unsigned int)tmp_val);
    }
    static const struct pcf85x63_config pcf_85263_config = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x2f,
    },
    .num_nvram = 1
    };
    static const struct pcf85x63_config pcf_85363_config = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x7f,
    },
    .num_nvram = 2
    };
#[no_mangle]
unsafe extern "C" fn pcf85363_probe(client: *mut i2c_client) -> c_int {
    static int pcf85363_probe(struct i2c_client *client)
    {
    struct pcf85363 *pcf85363;
    const struct pcf85x63_config *config = &pcf_85363_config;
    const void *data = of_device_get_match_data(&client.dev);
    static struct nvmem_config nvmem_cfg[] = {
    {
    .name = "pcf85x63-",
    .word_size = 1,
    .stride = 1,
    .size = 1,
    .reg_read = pcf85x63_nvram_read,
    .reg_write = pcf85x63_nvram_write,
    }, {
    .name = "pcf85363-",
    .word_size = 1,
    .stride = 1,
    .size = NVRAM_SIZE,
    .reg_read = pcf85363_nvram_read,
    .reg_write = pcf85363_nvram_write,
    },
    };
    int ret, i, err;
    bool wakeup_source;
    if (data)
    config = data;
    pcf85363 = devm_kzalloc(&client.dev, sizeof(struct pcf85363),
    GFP_KERNEL);
    if (!pcf85363)
    return -ENOMEM;
    pcf85363.regmap = devm_regmap_init_i2c(client, &config.regmap);
    if (IS_ERR(pcf85363.regmap)) {
    dev_err(&client.dev, "regmap allocation failed\n");
    return PTR_ERR(pcf85363.regmap);
    }
    i2c_set_clientdata(client, pcf85363);
    pcf85363.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(pcf85363.rtc))
    return PTR_ERR(pcf85363.rtc);
    err = pcf85363_load_capacitance(pcf85363, client.dev.of_node);
    if (err < 0)
    return dev_err_probe(&client.dev, err,
    "failed to set xtal load capacitance\n");
    pcf85363.rtc.ops = &rtc_ops;
    pcf85363.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    pcf85363.rtc.range_max = RTC_TIMESTAMP_END_2099;
    wakeup_source = device_property_read_bool(&client.dev,
    "wakeup-source");
    if (client.irq > 0 || wakeup_source) {
    err = regmap_write(pcf85363.regmap, CTRL_FLAGS, 0);
    if (err)
    return dev_err_probe(&client.dev, err,
    "failed to clear flags\n");
    err = regmap_update_bits(pcf85363.regmap, CTRL_PIN_IO,
    PIN_IO_INTAPM, PIN_IO_INTA_OUT);
    if (err)
    return dev_err_probe(&client.dev, err,
    "failed to set interrupt pin mode\n");
    }
    if (client.irq > 0) {
    let mut irqflags: c_ulong = IRQF_TRIGGER_LOW;
    if (dev_fwnode(&client.dev))
    irqflags = 0;
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), pcf85363_rtc_handle_irq,
    irqflags | IRQF_ONESHOT,
    "pcf85363", client);
    if (ret) {
    dev_warn(&client.dev,
    "unable to request IRQ, alarms disabled\n");
    client.irq = 0;
    }
    }
    if (client.irq > 0 || wakeup_source) {
    device_init_wakeup(&client.dev, true);
    set_bit(RTC_FEATURE_ALARM, pcf85363.rtc.features);
    } else {
    clear_bit(RTC_FEATURE_ALARM, pcf85363.rtc.features);
    }
    ret = devm_rtc_register_device(pcf85363.rtc);
    for (i = 0; i < config.num_nvram; i++) {
    nvmem_cfg[i].priv = pcf85363;
    devm_rtc_nvmem_register(pcf85363.rtc, &nvmem_cfg[i]);
    }
    return ret;
    }
    static const __maybe_unused struct of_device_id dev_ids[] = {
    { .compatible = "nxp,pcf85263", .data = &pcf_85263_config },
    { .compatible = "nxp,pcf85363", .data = &pcf_85363_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dev_ids);
    static struct i2c_driver pcf85363_driver = {
    .driver	= {
    .name	= "pcf85363",
    .of_match_table = of_match_ptr(dev_ids),
    },
    .probe = pcf85363_probe,
    };
    module_i2c_driver(pcf85363_driver);
    MODULE_AUTHOR("Eric Nelson");
    MODULE_DESCRIPTION("pcf85263/pcf85363 I2C RTC driver");
    MODULE_LICENSE("GPL");
