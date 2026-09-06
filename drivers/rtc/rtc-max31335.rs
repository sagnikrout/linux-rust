//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-max31335.c
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
// RTC driver for the MAX31335
//
// Copyright (C) 2023 Analog Devices
//
// Antoniu Miclaus <antoniu.miclaus@analog.com>
//

// MAX31335 Register Map
pub const MAX31335_STATUS1: c_uint = 0x00;
pub const MAX31335_INT_EN1: c_uint = 0x01;
pub const MAX31335_STATUS2: c_uint = 0x02;
pub const MAX31335_INT_EN2: c_uint = 0x03;
pub const MAX31335_RTC_RESET: c_uint = 0x04;
pub const MAX31335_RTC_CONFIG: c_uint = 0x05;
pub const MAX31335_RTC_CONFIG2: c_uint = 0x06;
pub const MAX31335_TIMESTAMP_CONFIG: c_uint = 0x07;
pub const MAX31335_TIMER_CONFIG: c_uint = 0x08;
pub const MAX31335_SECONDS_1_128: c_uint = 0x09;
pub const MAX31335_SECONDS: c_uint = 0x0A;
pub const MAX31335_MINUTES: c_uint = 0x0B;
pub const MAX31335_HOURS: c_uint = 0x0C;
pub const MAX31335_DAY: c_uint = 0x0D;
pub const MAX31335_DATE: c_uint = 0x0E;
pub const MAX31335_MONTH: c_uint = 0x0F;
pub const MAX31335_YEAR: c_uint = 0x0F;
pub const MAX31335_ALM1_SEC: c_uint = 0x11;
pub const MAX31335_ALM1_MIN: c_uint = 0x12;
pub const MAX31335_ALM1_HRS: c_uint = 0x13;
pub const MAX31335_ALM1_DAY_DATE: c_uint = 0x14;
pub const MAX31335_ALM1_MON: c_uint = 0x15;
pub const MAX31335_ALM1_YEAR: c_uint = 0x16;
pub const MAX31335_ALM2_MIN: c_uint = 0x17;
pub const MAX31335_ALM2_HRS: c_uint = 0x18;
pub const MAX31335_ALM2_DAY_DATE: c_uint = 0x19;
pub const MAX31335_TIMER_COUNT: c_uint = 0x1A;
pub const MAX31335_TIMER_INIT: c_uint = 0x1B;
pub const MAX31335_PWR_MGMT: c_uint = 0x1C;
pub const MAX31335_TRICKLE_REG: c_uint = 0x1D;
pub const MAX31335_AGING_OFFSET: c_uint = 0x1E;
pub const MAX31335_TS_CONFIG: c_uint = 0x30;
pub const MAX31335_TEMP_ALARM_HIGH_MSB: c_uint = 0x31;
pub const MAX31335_TEMP_ALARM_HIGH_LSB: c_uint = 0x32;
pub const MAX31335_TEMP_ALARM_LOW_MSB: c_uint = 0x33;
pub const MAX31335_TEMP_ALARM_LOW_LSB: c_uint = 0x34;
pub const MAX31335_TEMP_DATA_MSB: c_uint = 0x35;
pub const MAX31335_TEMP_DATA_LSB: c_uint = 0x36;
pub const MAX31335_TS0_SEC_1_128: c_uint = 0x40;
pub const MAX31335_TS0_SEC: c_uint = 0x41;
pub const MAX31335_TS0_MIN: c_uint = 0x42;
pub const MAX31335_TS0_HOUR: c_uint = 0x43;
pub const MAX31335_TS0_DATE: c_uint = 0x44;
pub const MAX31335_TS0_MONTH: c_uint = 0x45;
pub const MAX31335_TS0_YEAR: c_uint = 0x46;
pub const MAX31335_TS0_FLAGS: c_uint = 0x47;
pub const MAX31335_TS1_SEC_1_128: c_uint = 0x48;
pub const MAX31335_TS1_SEC: c_uint = 0x49;
pub const MAX31335_TS1_MIN: c_uint = 0x4A;
pub const MAX31335_TS1_HOUR: c_uint = 0x4B;
pub const MAX31335_TS1_DATE: c_uint = 0x4C;
pub const MAX31335_TS1_MONTH: c_uint = 0x4D;
pub const MAX31335_TS1_YEAR: c_uint = 0x4E;
pub const MAX31335_TS1_FLAGS: c_uint = 0x4F;
pub const MAX31335_TS2_SEC_1_128: c_uint = 0x50;
pub const MAX31335_TS2_SEC: c_uint = 0x51;
pub const MAX31335_TS2_MIN: c_uint = 0x52;
pub const MAX31335_TS2_HOUR: c_uint = 0x53;
pub const MAX31335_TS2_DATE: c_uint = 0x54;
pub const MAX31335_TS2_MONTH: c_uint = 0x55;
pub const MAX31335_TS2_YEAR: c_uint = 0x56;
pub const MAX31335_TS2_FLAGS: c_uint = 0x57;
pub const MAX31335_TS3_SEC_1_128: c_uint = 0x58;
pub const MAX31335_TS3_SEC: c_uint = 0x59;
pub const MAX31335_TS3_MIN: c_uint = 0x5A;
pub const MAX31335_TS3_HOUR: c_uint = 0x5B;
pub const MAX31335_TS3_DATE: c_uint = 0x5C;
pub const MAX31335_TS3_MONTH: c_uint = 0x5D;
pub const MAX31335_TS3_YEAR: c_uint = 0x5E;
pub const MAX31335_TS3_FLAGS: c_uint = 0x5F;
// MAX31335_STATUS1 Bit Definitions

// MAX31335_INT_EN1 Bit Definitions

// MAX31335_STATUS2 Bit Definitions

// MAX31335_INT_EN2 Bit Definitions

// MAX31335_RTC_RESET Bit Definitions

// MAX31335_RTC_CONFIG1 Bit Definitions

// MAX31335_RTC_CONFIG2 Bit Definitions

// MAX31335_TIMESTAMP_CONFIG Bit Definitions

// MAX31335_TIMER_CONFIG Bit Definitions

// MAX31335_HOURS Bit Definitions

// MAX31335_MONTH Bit Definitions

// MAX31335_PWR_MGMT Bit Definitions

// MAX31335_TRICKLE_REG Bit Definitions

// MAX31335_TS_CONFIG Bit Definitions

// MAX31335_TS_FLAGS Bit Definitions

// MAX31335 Miscellaneous Definitions
pub const MAX31335_TRICKLE_SCHOTTKY_DIODE: c_int = 1;
pub const MAX31335_TRICKLE_STANDARD_DIODE: c_int = 4;
pub const MAX31335_RAM_SIZE: c_int = 32;
pub const MAX31335_TIME_SIZE: c_uint = 0x07;
// MAX31331 Register Map
pub const MAX31331_RTC_CONFIG2: c_uint = 0x04;

// Supported Maxim RTC
    enum max_rtc_ids {
    ID_MAX31331,
    ID_MAX31335,
    MAX_RTC_ID_NR
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_desc {
    pub sec_reg: u8,
    pub alarm1_sec_reg: u8,
    pub int_en_reg: u8,
    pub int_status_reg: u8,
    pub ram_reg: u8,
    pub ram_size: u8,
    pub temp_reg: u8,
    pub trickle_reg: u8,
    pub clkout_reg: u8,
    pub id: enum max_rtc_ids,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max31335_data {
    pub regmap: *mut regmap,
    pub rtc: *mut rtc_device,
    pub clkout: clk_hw,
    pub clkin: *mut clk,
    pub chip: *const chip_desc,
    pub irq: c_int,
}

    static const int max31335_clkout_freq[] = { 1, 64, 1024, 32768 };
    static const struct chip_desc chip[MAX_RTC_ID_NR] = {
    [ID_MAX31331] = {
    .id = ID_MAX31331,
    .int_en_reg = 0x01,
    .int_status_reg = 0x00,
    .sec_reg = 0x08,
    .alarm1_sec_reg = 0x0F,
    .ram_reg = 0x20,
    .ram_size = 32,
    .trickle_reg = 0x1B,
    .clkout_reg = 0x04,
    },
    [ID_MAX31335] = {
    .id = ID_MAX31335,
    .int_en_reg = 0x01,
    .int_status_reg = 0x00,
    .sec_reg = 0x0A,
    .alarm1_sec_reg = 0x11,
    .ram_reg = 0x40,
    .ram_size = 32,
    .temp_reg = 0x35,
    .trickle_reg = 0x1D,
    .clkout_reg = 0x06,
    },
    };
    static const u16 max31335_trickle_resistors[] = {3000, 6000, 11000};
#[no_mangle]
unsafe extern "C" fn max31335_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max31335_volatile_reg(struct device *dev, unsigned int reg)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    const struct chip_desc *chip = max31335.chip;
// time keeping registers
    if (reg >= chip.sec_reg && reg < chip.sec_reg + MAX31335_TIME_SIZE)
    return true;
// interrupt status register
    if (reg == chip.int_status_reg)
    return true;
// temperature registers if valid
    if (chip.temp_reg && (reg == chip.temp_reg || reg == chip.temp_reg + 1))
    return true;
    return false;
    }
    static const struct regmap_config regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x5F,
    .volatile_reg = max31335_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn max31335_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max31335_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    u8 date[7];
    int ret;
    ret = regmap_bulk_read(max31335.regmap, max31335.chip.sec_reg, date,
    sizeof(date));
    if (ret)
    return ret;
    tm.tm_sec  = bcd2bin(date[0] & 0x7f);
    tm.tm_min  = bcd2bin(date[1] & 0x7f);
    tm.tm_hour = bcd2bin(date[2] & 0x3f);
    tm.tm_wday = bcd2bin(date[3] & 0x7) - 1;
    tm.tm_mday = bcd2bin(date[4] & 0x3f);
    tm.tm_mon  = bcd2bin(date[5] & 0x1f) - 1;
    tm.tm_year = bcd2bin(date[6]) + 100;
    if (FIELD_GET(MAX31335_MONTH_CENTURY, date[5]))
    tm.tm_year += 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31335_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max31335_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    u8 date[7];
    date[0] = bin2bcd(tm.tm_sec);
    date[1] = bin2bcd(tm.tm_min);
    date[2] = bin2bcd(tm.tm_hour);
    date[3] = bin2bcd(tm.tm_wday + 1);
    date[4] = bin2bcd(tm.tm_mday);
    date[5] = bin2bcd(tm.tm_mon + 1);
    date[6] = bin2bcd(tm.tm_year % 100);
    if (tm.tm_year >= 200)
    date[5] |= FIELD_PREP(MAX31335_MONTH_CENTURY, 1);
    return regmap_bulk_write(max31335.regmap, max31335.chip.sec_reg, date,
    sizeof(date));
    }
#[no_mangle]
unsafe extern "C" fn max31335_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max31335_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    int ret, ctrl, status;
    struct rtc_time time;
    u8 regs[6];
    ret = regmap_bulk_read(max31335.regmap, max31335.chip.alarm1_sec_reg, regs,
    sizeof(regs));
    if (ret)
    return ret;
    alrm.time.tm_sec  = bcd2bin(regs[0] & 0x7f);
    alrm.time.tm_min  = bcd2bin(regs[1] & 0x7f);
    alrm.time.tm_hour = bcd2bin(regs[2] & 0x3f);
    alrm.time.tm_mday = bcd2bin(regs[3] & 0x3f);
    alrm.time.tm_mon  = bcd2bin(regs[4] & 0x1f) - 1;
    alrm.time.tm_year = bcd2bin(regs[5]) + 100;
    ret = max31335_read_time(dev, &time);
    if (ret)
    return ret;
    if (time.tm_year >= 200)
    alrm.time.tm_year += 100;
    ret = regmap_read(max31335.regmap, max31335.chip.int_en_reg, &ctrl);
    if (ret)
    return ret;
    ret = regmap_read(max31335.regmap, max31335.chip.int_status_reg, &status);
    if (ret)
    return ret;
    alrm.enabled = FIELD_GET(MAX31335_INT_EN1_A1IE, ctrl);
    alrm.pending = FIELD_GET(MAX31335_STATUS1_A1F, status);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31335_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max31335_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    unsigned int reg;
    u8 regs[6];
    int ret;
    regs[0] = bin2bcd(alrm.time.tm_sec);
    regs[1] = bin2bcd(alrm.time.tm_min);
    regs[2] = bin2bcd(alrm.time.tm_hour);
    regs[3] = bin2bcd(alrm.time.tm_mday);
    regs[4] = bin2bcd(alrm.time.tm_mon + 1);
    regs[5] = bin2bcd(alrm.time.tm_year % 100);
    ret = regmap_bulk_write(max31335.regmap, max31335.chip.alarm1_sec_reg,
    regs, sizeof(regs));
    if (ret)
    return ret;
    reg = FIELD_PREP(MAX31335_INT_EN1_A1IE, alrm.enabled);
    ret = regmap_update_bits(max31335.regmap, max31335.chip.int_en_reg,
    MAX31335_INT_EN1_A1IE, reg);
    if (ret)
    return ret;
    return regmap_update_bits(max31335.regmap, max31335.chip.int_status_reg,
    MAX31335_STATUS1_A1F, 0);
    }
#[no_mangle]
unsafe extern "C" fn max31335_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int max31335_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    return regmap_update_bits(max31335.regmap, max31335.chip.int_en_reg,
    MAX31335_INT_EN1_A1IE, enabled);
    }
#[no_mangle]
unsafe extern "C" fn max31335_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max31335_handle_irq(int irq, void *dev_id)
    {
    struct max31335_data *max31335 = dev_id;
    struct mutex *lock = &max31335.rtc.ops_lock;
    int ret, status;
    mutex_lock(lock);
    ret = regmap_read(max31335.regmap, max31335.chip.int_status_reg, &status);
    if (ret)
    goto exit;
    if (FIELD_GET(MAX31335_STATUS1_A1F, status)) {
    ret = regmap_update_bits(max31335.regmap, max31335.chip.int_status_reg,
    MAX31335_STATUS1_A1F, 0);
    if (ret)
    goto exit;
    rtc_update_irq(max31335.rtc, 1, RTC_AF | RTC_IRQF);
    }
    exit:
    mutex_unlock(lock);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops max31335_rtc_ops = {
    .read_time = max31335_read_time,
    .set_time = max31335_set_time,
    .read_alarm = max31335_read_alarm,
    .set_alarm = max31335_set_alarm,
    .alarm_irq_enable = max31335_alarm_irq_enable,
    };
    static int max31335_trickle_charger_setup(struct device *dev,
    struct max31335_data *max31335)
    {
    u32 ohms, chargeable;
    int i, trickle_cfg;
    const char *diode;
    if (device_property_read_u32(dev, "aux-voltage-chargeable",
    &chargeable))
    return 0;
    if (device_property_read_u32(dev, "trickle-resistor-ohms", &ohms))
    return 0;
    if (device_property_read_string(dev, "adi,tc-diode", &diode))
    return 0;
    if (!strcmp(diode, "schottky"))
    trickle_cfg = MAX31335_TRICKLE_SCHOTTKY_DIODE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(diode, _arg: "standard+schottky")) -> else {
    else if (!strcmp(diode, "standard+schottky"))
    trickle_cfg = MAX31335_TRICKLE_STANDARD_DIODE;
    else
    return dev_err_probe(dev, -EINVAL,
    "Invalid tc-diode value: %s\n", diode);
    for (i = 0; i < ARRAY_SIZE(max31335_trickle_resistors); i++)
    if (ohms == max31335_trickle_resistors[i])
    break;
    if (i >= ARRAY_SIZE(max31335_trickle_resistors))
    return 0;
    i = i + trickle_cfg;
    return regmap_write(max31335.regmap, max31335.chip.trickle_reg,
    FIELD_PREP(MAX31335_TRICKLE_REG_TRICKLE, i) |
    FIELD_PREP(MAX31335_TRICKLE_REG_EN_TRICKLE,
    chargeable));
    }
    static unsigned long max31335_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct max31335_data *max31335 = clk_hw_to_max31335(hw);
    unsigned int freq_mask;
    unsigned int reg;
    int ret;
    ret = regmap_read(max31335.regmap, max31335.chip.clkout_reg, &reg);
    if (ret)
    return 0;
    freq_mask = __roundup_pow_of_two(ARRAY_SIZE(max31335_clkout_freq)) - 1;
    return max31335_clkout_freq[reg & freq_mask];
    }
    static int max31335_clkout_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    int index;
    index = find_closest(req.rate, max31335_clkout_freq,
    ARRAY_SIZE(max31335_clkout_freq));
    req.rate = max31335_clkout_freq[index];
    return 0;
    }
    static int max31335_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct max31335_data *max31335 = clk_hw_to_max31335(hw);
    unsigned int freq_mask;
    int index;
    index = find_closest(rate, max31335_clkout_freq,
    ARRAY_SIZE(max31335_clkout_freq));
    freq_mask = __roundup_pow_of_two(ARRAY_SIZE(max31335_clkout_freq)) - 1;
    return regmap_update_bits(max31335.regmap, max31335.chip.clkout_reg,
    freq_mask, index);
    }
#[no_mangle]
unsafe extern "C" fn max31335_clkout_enable(hw: *mut clk_hw) -> c_int {
    static int max31335_clkout_enable(struct clk_hw *hw)
    {
    struct max31335_data *max31335 = clk_hw_to_max31335(hw);
    return regmap_set_bits(max31335.regmap, max31335.chip.clkout_reg,
    MAX31335_RTC_CONFIG2_ENCLKO);
    }
#[no_mangle]
unsafe extern "C" fn max31335_clkout_disable(hw: *mut clk_hw) {
    static void max31335_clkout_disable(struct clk_hw *hw)
    {
    struct max31335_data *max31335 = clk_hw_to_max31335(hw);
    regmap_clear_bits(max31335.regmap, max31335.chip.clkout_reg,
    MAX31335_RTC_CONFIG2_ENCLKO);
    }
#[no_mangle]
unsafe extern "C" fn max31335_clkout_is_enabled(hw: *mut clk_hw) -> c_int {
    static int max31335_clkout_is_enabled(struct clk_hw *hw)
    {
    struct max31335_data *max31335 = clk_hw_to_max31335(hw);
    unsigned int reg;
    int ret;
    ret = regmap_read(max31335.regmap, max31335.chip.clkout_reg, &reg);
    if (ret)
    return ret;
    return !!(reg & MAX31335_RTC_CONFIG2_ENCLKO);
    }
    static const struct clk_ops max31335_clkout_ops = {
    .recalc_rate = max31335_clkout_recalc_rate,
    .determine_rate = max31335_clkout_determine_rate,
    .set_rate = max31335_clkout_set_rate,
    .enable = max31335_clkout_enable,
    .disable = max31335_clkout_disable,
    .is_enabled = max31335_clkout_is_enabled,
    };
    static struct clk_init_data max31335_clk_init = {
    .name = "max31335-clkout",
    .ops = &max31335_clkout_ops,
    };
    static int max31335_nvmem_reg_read(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    struct max31335_data *max31335 = priv;
    let mut reg: c_uint = max31335.chip.ram_reg + offset;
    return regmap_bulk_read(max31335.regmap, reg, val, bytes);
    }
    static int max31335_nvmem_reg_write(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    struct max31335_data *max31335 = priv;
    let mut reg: c_uint = max31335.chip.ram_reg + offset;
    return regmap_bulk_write(max31335.regmap, reg, val, bytes);
    }
    static struct nvmem_config max31335_nvmem_cfg = {
    .reg_read = max31335_nvmem_reg_read,
    .reg_write = max31335_nvmem_reg_write,
    .word_size = 8,
    .size = MAX31335_RAM_SIZE,
    };

    static int max31335_read_temp(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    u8 reg[2];
    s16 temp;
    int ret;
    if (type != hwmon_temp || attr != hwmon_temp_input)
    return -EOPNOTSUPP;
    ret = regmap_bulk_read(max31335.regmap, max31335.chip.temp_reg,
    reg, 2);
    if (ret)
    return ret;
    temp = get_unaligned_be16(reg);
// val = (temp / 64) * 250;
    return 0;
    }
    static umode_t max31335_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type == hwmon_temp && attr == hwmon_temp_input)
    return 0444;
    return 0;
    }
    static const struct hwmon_channel_info *max31335_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max31335_hwmon_ops = {
    .is_visible = max31335_is_visible,
    .read = max31335_read_temp,
    };
    static const struct hwmon_chip_info max31335_chip_info = {
    .ops = &max31335_hwmon_ops,
    .info = max31335_info,
    };

#[no_mangle]
unsafe extern "C" fn max31335_clkout_register(dev: *mut device) -> c_int {
    static int max31335_clkout_register(struct device *dev)
    {
    struct max31335_data *max31335 = dev_get_drvdata(dev);
    int ret;
    if (!device_property_present(dev, "#clock-cells"))
    return regmap_clear_bits(max31335.regmap, max31335.chip.clkout_reg,
    MAX31335_RTC_CONFIG2_ENCLKO);
    max31335.clkout.init = &max31335_clk_init;
    ret = devm_clk_hw_register(dev, &max31335.clkout);
    if (ret)
    return dev_err_probe(dev, ret, "cannot register clock\n");
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get,
    &max31335.clkout);
    if (ret)
    return dev_err_probe(dev, ret, "cannot add hw provider\n");
    max31335.clkout.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(max31335.clkout.clk))
    return dev_err_probe(dev, PTR_ERR(max31335.clkout.clk),
    "cannot enable clkout\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31335_probe(client: *mut i2c_client) -> c_int {
    static int max31335_probe(struct i2c_client *client)
    {
    struct max31335_data *max31335;

    struct device *hwmon;

    const struct chip_desc *match;
    int ret;
    max31335 = devm_kzalloc(&client.dev, sizeof(*max31335), GFP_KERNEL);
    if (!max31335)
    return -ENOMEM;
    max31335.regmap = devm_regmap_init_i2c(client, &regmap_config);
    if (IS_ERR(max31335.regmap))
    return PTR_ERR(max31335.regmap);
    i2c_set_clientdata(client, max31335);
    match = i2c_get_match_data(client);
    if (!match)
    return -ENODEV;
    max31335.chip = match;
    max31335.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(max31335.rtc))
    return PTR_ERR(max31335.rtc);
    max31335.rtc.ops = &max31335_rtc_ops;
    max31335.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    max31335.rtc.range_max = RTC_TIMESTAMP_END_2199;
    max31335.rtc.alarm_offset_max = 24 * 60 * 60;
    ret = max31335_clkout_register(&client.dev);
    if (ret)
    return ret;
    if (client.irq > 0) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), max31335_handle_irq,
    IRQF_ONESHOT,
    "max31335", max31335);
    if (ret) {
    dev_warn(&client.dev,
    "unable to request IRQ, alarm max31335 disabled\n");
    client.irq = 0;
    } else {
    max31335.irq = client.irq;
    }
    }
    if (!client.irq)
    clear_bit(RTC_FEATURE_ALARM, max31335.rtc.features);
    max31335_nvmem_cfg.priv = max31335;
    ret = devm_rtc_nvmem_register(max31335.rtc, &max31335_nvmem_cfg);
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "cannot register rtc nvmem\n");

    if (max31335.chip.temp_reg) {
    hwmon = devm_hwmon_device_register_with_info(&client.dev, client.name, max31335,
    &max31335_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon))
    return dev_err_probe(&client.dev, PTR_ERR(hwmon),
    "cannot register hwmon device\n");
    }

    ret = max31335_trickle_charger_setup(&client.dev, max31335);
    if (ret)
    return ret;
    return devm_rtc_register_device(max31335.rtc);
    }
    static const struct i2c_device_id max31335_id[] = {
    { .name = "max31331", .driver_data = (kernel_ulong_t)&chip[ID_MAX31331] },
    { .name = "max31335", .driver_data = (kernel_ulong_t)&chip[ID_MAX31335] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max31335_id);
    static const struct of_device_id max31335_of_match[] = {
    { .compatible = "adi,max31331", .data = &chip[ID_MAX31331] },
    { .compatible = "adi,max31335", .data = &chip[ID_MAX31335] },
    { }
    };
    MODULE_DEVICE_TABLE(of, max31335_of_match);
    static struct i2c_driver max31335_driver = {
    .driver = {
    .name = "rtc-max31335",
    .of_match_table = max31335_of_match,
    },
    .probe = max31335_probe,
    .id_table = max31335_id,
    };
    module_i2c_driver(max31335_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com>");
    MODULE_AUTHOR("Saket Kumar Purwar <Saket.Kumarpurwar@analog.com>");
    MODULE_DESCRIPTION("MAX31335 RTC driver");
    MODULE_LICENSE("GPL");
