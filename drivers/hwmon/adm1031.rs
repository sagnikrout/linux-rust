//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adm1031.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// adm1031.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
// Based on lm75.c and lm85.c
// Supports adm1030 / adm1031
// Copyright (C) 2004 Alexandre d'Alton <alex@alexdalton.org>
// Reworked by Jean Delvare <jdelvare@suse.de>
//

// Following macros takes channel parameter starting from 0 to 2

pub const ADM1031_REG_CONF1: c_uint = 0x00;
pub const ADM1031_REG_CONF2: c_uint = 0x01;
pub const ADM1031_REG_EXT_TEMP: c_uint = 0x06;
pub const ADM1031_CONF1_MONITOR_ENABLE: c_uint = 0x01	/* Monitoring enable */;
pub const ADM1031_CONF1_PWM_INVERT: c_uint = 0x08	/* PWM Invert */;
pub const ADM1031_CONF1_AUTO_MODE: c_uint = 0x80	/* Auto FAN */;
pub const ADM1031_CONF2_PWM1_ENABLE: c_uint = 0x01;
pub const ADM1031_CONF2_PWM2_ENABLE: c_uint = 0x02;
pub const ADM1031_CONF2_TACH1_ENABLE: c_uint = 0x04;
pub const ADM1031_CONF2_TACH2_ENABLE: c_uint = 0x08;

pub const ADM1031_UPDATE_RATE_MASK: c_uint = 0x1c;
pub const ADM1031_UPDATE_RATE_SHIFT: c_int = 2;
// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x2c, 0x2d, 0x2e, I2C_CLIENT_END };
    enum chips { adm1030, adm1031 };
    typedef u8 auto_chan_table_t[8][2];
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm1031_data {
    pub client: *mut i2c_client,
    pub groups: [*const attribute_group; 3],
    pub update_lock: mutex,
    pub chip_type: c_int,
    pub /: *mut *mut bool valid; / true if following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub /: *mut *mut unsigned int update_interval; / In milliseconds,
//
// The chan_select_table contains the possible configurations for
// auto fan control.
//
    pub chan_select_table: *const auto_chan_table_t,
    pub alarm: u16,
    pub conf1: u8,
    pub conf2: u8,
    pub fan: [u8; 2],
    pub fan_div: [u8; 2],
    pub fan_min: [u8; 2],
    pub pwm: [u8; 2],
    pub old_pwm: [u8; 2],
    pub temp: [i8; 3],
    pub ext_temp: [u8; 3],
    pub auto_temp: [u8; 3],
    pub auto_temp_min: [u8; 3],
    pub auto_temp_off: [u8; 3],
    pub auto_temp_max: [u8; 3],
    pub temp_offset: [i8; 3],
    pub temp_min: [i8; 3],
    pub temp_max: [i8; 3],
    pub temp_crit: [i8; 3],
}

#[no_mangle]
pub unsafe extern "C" fn adm1031_read_value(client: *mut i2c_client, reg: u8) -> u8 {
    static inline u8 adm1031_read_value(struct i2c_client *client, u8 reg)
    {
    return i2c_smbus_read_byte_data(client, reg);
    }
    static inline int
    adm1031_write_value(struct i2c_client *client, u8 reg, unsigned int value)
    {
    return i2c_smbus_write_byte_data(client, reg, value);
    }
    static struct adm1031_data *adm1031_update_device(struct device *dev)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long next_update;
    int chan;
    mutex_lock(&data.update_lock);
    next_update = data.last_updated
    + msecs_to_jiffies(data.update_interval);
    if (time_after(jiffies, next_update) || !data.valid) {
    dev_dbg(&client.dev, "Starting adm1031 update\n");
    for (chan = 0;
    chan < ((data.chip_type == adm1031) ? 3 : 2); chan++) {
    u8 oldh, newh;
    oldh =
    adm1031_read_value(client, ADM1031_REG_TEMP(chan));
    data.ext_temp[chan] =
    adm1031_read_value(client, ADM1031_REG_EXT_TEMP);
    newh =
    adm1031_read_value(client, ADM1031_REG_TEMP(chan));
    if (newh != oldh) {
    data.ext_temp[chan] =
    adm1031_read_value(client,
    ADM1031_REG_EXT_TEMP);

    oldh =
    adm1031_read_value(client,
    ADM1031_REG_TEMP(chan));
// oldh is actually newer
    if (newh != oldh)
    dev_warn(&client.dev,
    "Remote temperature may be wrong.\n");

    }
    data.temp[chan] = newh;
    data.temp_offset[chan] =
    adm1031_read_value(client,
    ADM1031_REG_TEMP_OFFSET(chan));
    data.temp_min[chan] =
    adm1031_read_value(client,
    ADM1031_REG_TEMP_MIN(chan));
    data.temp_max[chan] =
    adm1031_read_value(client,
    ADM1031_REG_TEMP_MAX(chan));
    data.temp_crit[chan] =
    adm1031_read_value(client,
    ADM1031_REG_TEMP_CRIT(chan));
    data.auto_temp[chan] =
    adm1031_read_value(client,
    ADM1031_REG_AUTO_TEMP(chan));
    }
    data.conf1 = adm1031_read_value(client, ADM1031_REG_CONF1);
    data.conf2 = adm1031_read_value(client, ADM1031_REG_CONF2);
    data.alarm = adm1031_read_value(client, ADM1031_REG_STATUS(0))
    | (adm1031_read_value(client, ADM1031_REG_STATUS(1)) << 8);
    if (data.chip_type == adm1030)
    data.alarm &= 0xc0ff;
    for (chan = 0; chan < (data.chip_type == adm1030 ? 1 : 2);
    chan++) {
    data.fan_div[chan] =
    adm1031_read_value(client,
    ADM1031_REG_FAN_DIV(chan));
    data.fan_min[chan] =
    adm1031_read_value(client,
    ADM1031_REG_FAN_MIN(chan));
    data.fan[chan] =
    adm1031_read_value(client,
    ADM1031_REG_FAN_SPEED(chan));
    data.pwm[chan] =
    (adm1031_read_value(client,
    ADM1031_REG_PWM) >> (4 * chan)) & 0x0f;
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }

    ((val + 500) / 1000)))

    (val) | 0x70 : (val))

    (11250 * 60) / ((reg) * (div)) : 0)
#[no_mangle]
unsafe extern "C" fn FAN_TO_REG(reg: c_int, div: c_int) -> c_int {
    static int FAN_TO_REG(int reg, int div)
    {
    int tmp;
    tmp = FAN_FROM_REG(clamp_val(reg, 0, 65535), div);
    return tmp > 255 ? 255 : tmp;
    }

    (((reg) & 0x1F) | (((val) << 5) & 0xe0))

    ((((val) / 500) & 0xf8) | ((reg) & 0x7))

    (AUTO_TEMP_MIN_FROM_REG(reg) - 5000)

    (AUTO_TEMP_RANGE_FROM_REG(reg) +	\
    AUTO_TEMP_MIN_FROM_REG(reg))
#[no_mangle]
unsafe extern "C" fn AUTO_TEMP_MAX_TO_REG(val: c_int, reg: c_int, pwm: c_int) -> c_int {
    static int AUTO_TEMP_MAX_TO_REG(int val, int reg, int pwm)
    {
    int ret;
    let mut range: c_int = ((val - AUTO_TEMP_MIN_FROM_REG(reg)) * 10) / (16 - pwm);
    ret = ((reg & 0xf8) |
    (range < 10000 ? 0 :
    range < 20000 ? 1 :
    range < 40000 ? 2 : range < 80000 ? 3 : 4));
    return ret;
    }
// FAN auto control

    (*(data).chan_select_table)[FAN_CHAN_FROM_REG((data).conf1)][idx % 2]
//
// The tables below contains the possible values for the auto fan
// control bitfields. the index in the table is the register value.
// MSb is the auto fan control enable bit, so the four first entries
// in the table disables auto fan control when both bitfields are zero.
//
    static const auto_chan_table_t auto_channel_select_table_adm1031 = {
    { 0, 0 }, { 0, 0 }, { 0, 0 }, { 0, 0 },
    { 2 /* 0b010 */ , 4 /* 0b100 */ },
    { 2 /* 0b010 */ , 2 /* 0b010 */ },
    { 4 /* 0b100 */ , 4 /* 0b100 */ },
    { 7 /* 0b111 */ , 7 /* 0b111 */ },
    };
    static const auto_chan_table_t auto_channel_select_table_adm1030 = {
    { 0, 0 }, { 0, 0 }, { 0, 0 }, { 0, 0 },
    { 2 /* 0b10 */		, 0 },
    { 0xff /* invalid */	, 0 },
    { 0xff /* invalid */	, 0 },
    { 3 /* 0b11 */		, 0 },
    };
//
// That function checks if a bitfield is valid and returns the other bitfield
// nearest match if no exact match where found.
//
    static int
    get_fan_auto_nearest(struct adm1031_data *data, int chan, u8 val, u8 reg)
    {
    int i;
    let mut first_match: c_int = -1, exact_match = -1;
    u8 other_reg_val =
    (*data.chan_select_table)[FAN_CHAN_FROM_REG(reg)][chan ? 0 : 1];
    if (val == 0)
    return 0;
    for (i = 0; i < 8; i++) {
    if ((val == (*data.chan_select_table)[i][chan]) &&
    ((*data.chan_select_table)[i][chan ? 0 : 1] ==
    other_reg_val)) {
// We found an exact match
    exact_match = i;
    break;
    } else if (val == (*data.chan_select_table)[i][chan] &&
    first_match == -1) {
//
// Save the first match in case of an exact match has
// not been found
//
    first_match = i;
    }
    }
    if (exact_match >= 0)
    return exact_match;
#[no_mangle]
pub unsafe extern "C" fn if(0: first_match >=) -> else {
    else if (first_match >= 0)
    return first_match;
    return -EINVAL;
    }
    static ssize_t fan_auto_channel_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", GET_FAN_AUTO_BITFIELD(data, nr));
    }
    static ssize_t
    fan_auto_channel_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    u8 reg;
    int ret;
    u8 old_fan_mode;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    old_fan_mode = data.conf1;
    mutex_lock(&data.update_lock);
    ret = get_fan_auto_nearest(data, nr, val, data.conf1);
    if (ret < 0) {
    mutex_unlock(&data.update_lock);
    return ret;
    }
    reg = ret;
    data.conf1 = FAN_CHAN_TO_REG(reg, data.conf1);
    if ((data.conf1 & ADM1031_CONF1_AUTO_MODE) ^
    (old_fan_mode & ADM1031_CONF1_AUTO_MODE)) {
    if (data.conf1 & ADM1031_CONF1_AUTO_MODE) {
//
// Switch to Auto Fan Mode
// Save PWM registers
// Set PWM registers to 33% Both
//
    data.old_pwm[0] = data.pwm[0];
    data.old_pwm[1] = data.pwm[1];
    adm1031_write_value(client, ADM1031_REG_PWM, 0x55);
    } else {
// Switch to Manual Mode
    data.pwm[0] = data.old_pwm[0];
    data.pwm[1] = data.old_pwm[1];
// Restore PWM registers
    adm1031_write_value(client, ADM1031_REG_PWM,
    data.pwm[0] | (data.pwm[1] << 4));
    }
    }
    data.conf1 = FAN_CHAN_TO_REG(reg, data.conf1);
    adm1031_write_value(client, ADM1031_REG_CONF1, data.conf1);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RW(auto_fan1_channel, fan_auto_channel, 0);
    static SENSOR_DEVICE_ATTR_RW(auto_fan2_channel, fan_auto_channel, 1);
// Auto Temps
    static ssize_t auto_temp_off_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n",
    AUTO_TEMP_OFF_FROM_REG(data.auto_temp[nr]));
    }
    static ssize_t auto_temp_min_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n",
    AUTO_TEMP_MIN_FROM_REG(data.auto_temp[nr]));
    }
    static ssize_t
    auto_temp_min_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, 0, 127000);
    mutex_lock(&data.update_lock);
    data.auto_temp[nr] = AUTO_TEMP_MIN_TO_REG(val, data.auto_temp[nr]);
    adm1031_write_value(client, ADM1031_REG_AUTO_TEMP(nr),
    data.auto_temp[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t auto_temp_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n",
    AUTO_TEMP_MAX_FROM_REG(data.auto_temp[nr]));
    }
    static ssize_t
    auto_temp_max_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, 0, 127000);
    mutex_lock(&data.update_lock);
    data.temp_max[nr] = AUTO_TEMP_MAX_TO_REG(val, data.auto_temp[nr],
    data.pwm[nr]);
    adm1031_write_value(client, ADM1031_REG_AUTO_TEMP(nr),
    data.temp_max[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RO(auto_temp1_off, auto_temp_off, 0);
    static SENSOR_DEVICE_ATTR_RW(auto_temp1_min, auto_temp_min, 0);
    static SENSOR_DEVICE_ATTR_RW(auto_temp1_max, auto_temp_max, 0);
    static SENSOR_DEVICE_ATTR_RO(auto_temp2_off, auto_temp_off, 1);
    static SENSOR_DEVICE_ATTR_RW(auto_temp2_min, auto_temp_min, 1);
    static SENSOR_DEVICE_ATTR_RW(auto_temp2_max, auto_temp_max, 1);
    static SENSOR_DEVICE_ATTR_RO(auto_temp3_off, auto_temp_off, 2);
    static SENSOR_DEVICE_ATTR_RW(auto_temp3_min, auto_temp_min, 2);
    static SENSOR_DEVICE_ATTR_RW(auto_temp3_max, auto_temp_max, 2);
// pwm
    static ssize_t pwm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", PWM_FROM_REG(data.pwm[nr]));
    }
    static ssize_t pwm_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret, reg;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    mutex_lock(&data.update_lock);
    if ((data.conf1 & ADM1031_CONF1_AUTO_MODE) &&
    (((val>>4) & 0xf) != 5)) {
// In automatic mode, the only PWM accepted is 33%
    mutex_unlock(&data.update_lock);
    return -EINVAL;
    }
    data.pwm[nr] = PWM_TO_REG(val);
    reg = adm1031_read_value(client, ADM1031_REG_PWM);
    adm1031_write_value(client, ADM1031_REG_PWM,
    nr ? ((data.pwm[nr] << 4) & 0xf0) | (reg & 0xf)
    : (data.pwm[nr] & 0xf) | (reg & 0xf0));
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RW(pwm1, pwm, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2, pwm, 1);
    static SENSOR_DEVICE_ATTR_RW(auto_fan1_min_pwm, pwm, 0);
    static SENSOR_DEVICE_ATTR_RW(auto_fan2_min_pwm, pwm, 1);
// Fans
//
// That function checks the cases where the fan reading is not
// relevant.  It is used to provide 0 as fan reading when the fan is
// not supposed to run
//
#[no_mangle]
unsafe extern "C" fn trust_fan_readings(data: *mut adm1031_data, chan: c_int) -> c_int {
    static int trust_fan_readings(struct adm1031_data *data, int chan)
    {
    let mut res: c_int = 0;
    if (data.conf1 & ADM1031_CONF1_AUTO_MODE) {
    switch (data.conf1 & 0x60) {
    case 0x00:
//
// remote temp1 controls fan1,
// remote temp2 controls fan2
//
    res = data.temp[chan+1] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[chan+1]);
    break;
    case 0x20:	/* remote temp1 controls both fans */
    res =
    data.temp[1] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[1]);
    break;
    case 0x40:	/* remote temp2 controls both fans */
    res =
    data.temp[2] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[2]);
    break;
    case 0x60:	/* max controls both fans */
    res =
    data.temp[0] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[0])
    || data.temp[1] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[1])
    || (data.chip_type == adm1031
    && data.temp[2] >=
    AUTO_TEMP_MIN_FROM_REG_DEG(data.auto_temp[2]));
    break;
    }
    } else {
    res = data.pwm[chan] > 0;
    }
    return res;
    }
    static ssize_t fan_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    int value;
    value = trust_fan_readings(data, nr) ? FAN_FROM_REG(data.fan[nr],
    FAN_DIV_FROM_REG(data.fan_div[nr])) : 0;
    return sprintf(buf, "%d\n", value);
    }
    static ssize_t fan_div_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", FAN_DIV_FROM_REG(data.fan_div[nr]));
    }
    static ssize_t fan_min_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n",
    FAN_FROM_REG(data.fan_min[nr],
    FAN_DIV_FROM_REG(data.fan_div[nr])));
    }
    static ssize_t fan_min_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    mutex_lock(&data.update_lock);
    if (val) {
    data.fan_min[nr] =
    FAN_TO_REG(val, FAN_DIV_FROM_REG(data.fan_div[nr]));
    } else {
    data.fan_min[nr] = 0xff;
    }
    adm1031_write_value(client, ADM1031_REG_FAN_MIN(nr), data.fan_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t fan_div_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    u8 tmp;
    int old_div;
    int new_min;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    tmp = val == 8 ? 0xc0 :
    val == 4 ? 0x80 :
    val == 2 ? 0x40 :
    val == 1 ? 0x00 :
    0xff;
    if (tmp == 0xff)
    return -EINVAL;
    mutex_lock(&data.update_lock);
// Get fresh readings
    data.fan_div[nr] = adm1031_read_value(client,
    ADM1031_REG_FAN_DIV(nr));
    data.fan_min[nr] = adm1031_read_value(client,
    ADM1031_REG_FAN_MIN(nr));
// Write the new clock divider and fan min
    old_div = FAN_DIV_FROM_REG(data.fan_div[nr]);
    data.fan_div[nr] = tmp | (0x3f & data.fan_div[nr]);
    new_min = data.fan_min[nr] * old_div / val;
    data.fan_min[nr] = new_min > 0xff ? 0xff : new_min;
    adm1031_write_value(client, ADM1031_REG_FAN_DIV(nr),
    data.fan_div[nr]);
    adm1031_write_value(client, ADM1031_REG_FAN_MIN(nr),
    data.fan_min[nr]);
// Invalidate the cache: fan speed is no longer valid
    data.valid = false;
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RO(fan1_input, fan, 0);
    static SENSOR_DEVICE_ATTR_RW(fan1_min, fan_min, 0);
    static SENSOR_DEVICE_ATTR_RW(fan1_div, fan_div, 0);
    static SENSOR_DEVICE_ATTR_RO(fan2_input, fan, 1);
    static SENSOR_DEVICE_ATTR_RW(fan2_min, fan_min, 1);
    static SENSOR_DEVICE_ATTR_RW(fan2_div, fan_div, 1);
// Temps
    static ssize_t temp_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    int ext;
    ext = nr == 0 ?
    ((data.ext_temp[nr] >> 6) & 0x3) * 2 :
    (((data.ext_temp[nr] >> ((nr - 1) * 3)) & 7));
    return sprintf(buf, "%d\n", TEMP_FROM_REG_EXT(data.temp[nr], ext));
    }
    static ssize_t temp_offset_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n",
    TEMP_OFFSET_FROM_REG(data.temp_offset[nr]));
    }
    static ssize_t temp_min_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp_min[nr]));
    }
    static ssize_t temp_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp_max[nr]));
    }
    static ssize_t temp_crit_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp_crit[nr]));
    }
    static ssize_t temp_offset_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, -15000, 15000);
    mutex_lock(&data.update_lock);
    data.temp_offset[nr] = TEMP_OFFSET_TO_REG(val);
    adm1031_write_value(client, ADM1031_REG_TEMP_OFFSET(nr),
    data.temp_offset[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp_min_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, -55000, 127000);
    mutex_lock(&data.update_lock);
    data.temp_min[nr] = TEMP_TO_REG(val);
    adm1031_write_value(client, ADM1031_REG_TEMP_MIN(nr),
    data.temp_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, -55000, 127000);
    mutex_lock(&data.update_lock);
    data.temp_max[nr] = TEMP_TO_REG(val);
    adm1031_write_value(client, ADM1031_REG_TEMP_MAX(nr),
    data.temp_max[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp_crit_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    val = clamp_val(val, -55000, 127000);
    mutex_lock(&data.update_lock);
    data.temp_crit[nr] = TEMP_TO_REG(val);
    adm1031_write_value(client, ADM1031_REG_TEMP_CRIT(nr),
    data.temp_crit[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_offset, temp_offset, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_min, temp_min, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_max, temp_max, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_crit, temp_crit, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_input, temp, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_offset, temp_offset, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_min, temp_min, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_max, temp_max, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_crit, temp_crit, 1);
    static SENSOR_DEVICE_ATTR_RO(temp3_input, temp, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_offset, temp_offset, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_min, temp_min, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_max, temp_max, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_crit, temp_crit, 2);
// Alarms
    static ssize_t alarms_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", data.alarm);
    }
    static DEVICE_ATTR_RO(alarms);
    static ssize_t alarm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut bitnr: c_int = to_sensor_dev_attr(attr).index;
    struct adm1031_data *data = adm1031_update_device(dev);
    return sprintf(buf, "%d\n", (data.alarm >> bitnr) & 1);
    }
    static SENSOR_DEVICE_ATTR_RO(fan1_alarm, alarm, 0);
    static SENSOR_DEVICE_ATTR_RO(fan1_fault, alarm, 1);
    static SENSOR_DEVICE_ATTR_RO(temp2_max_alarm, alarm, 2);
    static SENSOR_DEVICE_ATTR_RO(temp2_min_alarm, alarm, 3);
    static SENSOR_DEVICE_ATTR_RO(temp2_crit_alarm, alarm, 4);
    static SENSOR_DEVICE_ATTR_RO(temp2_fault, alarm, 5);
    static SENSOR_DEVICE_ATTR_RO(temp1_max_alarm, alarm, 6);
    static SENSOR_DEVICE_ATTR_RO(temp1_min_alarm, alarm, 7);
    static SENSOR_DEVICE_ATTR_RO(fan2_alarm, alarm, 8);
    static SENSOR_DEVICE_ATTR_RO(fan2_fault, alarm, 9);
    static SENSOR_DEVICE_ATTR_RO(temp3_max_alarm, alarm, 10);
    static SENSOR_DEVICE_ATTR_RO(temp3_min_alarm, alarm, 11);
    static SENSOR_DEVICE_ATTR_RO(temp3_crit_alarm, alarm, 12);
    static SENSOR_DEVICE_ATTR_RO(temp3_fault, alarm, 13);
    static SENSOR_DEVICE_ATTR_RO(temp1_crit_alarm, alarm, 14);
// Update Interval
    static const unsigned int update_intervals[] = {
    16000, 8000, 4000, 2000, 1000, 500, 250, 125,
    };
    static ssize_t update_interval_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", data.update_interval);
    }
    static ssize_t update_interval_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adm1031_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    int i, err;
    u8 reg;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
//
// Find the nearest update interval from the table.
// Use it to determine the matching update rate.
//
    for (i = 0; i < ARRAY_SIZE(update_intervals) - 1; i++) {
    if (val >= update_intervals[i])
    break;
    }
// if not found, we point to the last entry (lowest update interval)
// set the new update rate while preserving other settings
    reg = adm1031_read_value(client, ADM1031_REG_FAN_FILTER);
    reg &= ~ADM1031_UPDATE_RATE_MASK;
    reg |= i << ADM1031_UPDATE_RATE_SHIFT;
    adm1031_write_value(client, ADM1031_REG_FAN_FILTER, reg);
    mutex_lock(&data.update_lock);
    data.update_interval = update_intervals[i];
    mutex_unlock(&data.update_lock);
    return count;
    }
    static DEVICE_ATTR_RW(update_interval);
    static struct attribute *adm1031_attributes[] = {
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan1_div.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan1_fault.dev_attr.attr,
    &sensor_dev_attr_pwm1.dev_attr.attr,
    &sensor_dev_attr_auto_fan1_channel.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_offset.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_crit.dev_attr.attr,
    &sensor_dev_attr_temp1_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_offset.dev_attr.attr,
    &sensor_dev_attr_temp2_min.dev_attr.attr,
    &sensor_dev_attr_temp2_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp2_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_crit.dev_attr.attr,
    &sensor_dev_attr_temp2_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_fault.dev_attr.attr,
    &sensor_dev_attr_auto_temp1_off.dev_attr.attr,
    &sensor_dev_attr_auto_temp1_min.dev_attr.attr,
    &sensor_dev_attr_auto_temp1_max.dev_attr.attr,
    &sensor_dev_attr_auto_temp2_off.dev_attr.attr,
    &sensor_dev_attr_auto_temp2_min.dev_attr.attr,
    &sensor_dev_attr_auto_temp2_max.dev_attr.attr,
    &sensor_dev_attr_auto_fan1_min_pwm.dev_attr.attr,
    &dev_attr_update_interval.attr,
    &dev_attr_alarms.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group adm1031_group = {
    .attrs = adm1031_attributes,
    };
    static struct attribute *adm1031_attributes_opt[] = {
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan2_div.dev_attr.attr,
    &sensor_dev_attr_fan2_min.dev_attr.attr,
    &sensor_dev_attr_fan2_alarm.dev_attr.attr,
    &sensor_dev_attr_fan2_fault.dev_attr.attr,
    &sensor_dev_attr_pwm2.dev_attr.attr,
    &sensor_dev_attr_auto_fan2_channel.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp3_offset.dev_attr.attr,
    &sensor_dev_attr_temp3_min.dev_attr.attr,
    &sensor_dev_attr_temp3_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_max.dev_attr.attr,
    &sensor_dev_attr_temp3_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_crit.dev_attr.attr,
    &sensor_dev_attr_temp3_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_fault.dev_attr.attr,
    &sensor_dev_attr_auto_temp3_off.dev_attr.attr,
    &sensor_dev_attr_auto_temp3_min.dev_attr.attr,
    &sensor_dev_attr_auto_temp3_max.dev_attr.attr,
    &sensor_dev_attr_auto_fan2_min_pwm.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group adm1031_group_opt = {
    .attrs = adm1031_attributes_opt,
    };
// Return 0 if detection is successful, -ENODEV otherwise
    static int adm1031_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    const char *name;
    int id, co;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    id = i2c_smbus_read_byte_data(client, 0x3d);
    co = i2c_smbus_read_byte_data(client, 0x3e);
    if (!((id == 0x31 || id == 0x30) && co == 0x41))
    return -ENODEV;
    name = (id == 0x30) ? "adm1030" : "adm1031";
    strscpy(info.type, name, I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm1031_init_client(client: *mut i2c_client) {
    static void adm1031_init_client(struct i2c_client *client)
    {
    unsigned int read_val;
    unsigned int mask;
    int i;
    struct adm1031_data *data = i2c_get_clientdata(client);
    mask = (ADM1031_CONF2_PWM1_ENABLE | ADM1031_CONF2_TACH1_ENABLE);
    if (data.chip_type == adm1031) {
    mask |= (ADM1031_CONF2_PWM2_ENABLE |
    ADM1031_CONF2_TACH2_ENABLE);
    }
// Initialize the ADM1031 chip (enables fan speed reading )
    read_val = adm1031_read_value(client, ADM1031_REG_CONF2);
    if ((read_val | mask) != read_val)
    adm1031_write_value(client, ADM1031_REG_CONF2, read_val | mask);
    read_val = adm1031_read_value(client, ADM1031_REG_CONF1);
    if ((read_val | ADM1031_CONF1_MONITOR_ENABLE) != read_val) {
    adm1031_write_value(client, ADM1031_REG_CONF1,
    read_val | ADM1031_CONF1_MONITOR_ENABLE);
    }
// Read the chip's update rate
    mask = ADM1031_UPDATE_RATE_MASK;
    read_val = adm1031_read_value(client, ADM1031_REG_FAN_FILTER);
    i = (read_val & mask) >> ADM1031_UPDATE_RATE_SHIFT;
// Save it as update interval
    data.update_interval = update_intervals[i];
    }
#[no_mangle]
unsafe extern "C" fn adm1031_probe(client: *mut i2c_client) -> c_int {
    static int adm1031_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct adm1031_data *data;
    data = devm_kzalloc(dev, sizeof(struct adm1031_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.client = client;
    data.chip_type = (uintptr_t)i2c_get_match_data(client);
    mutex_init(&data.update_lock);
    if (data.chip_type == adm1030)
    data.chan_select_table = &auto_channel_select_table_adm1030;
    else
    data.chan_select_table = &auto_channel_select_table_adm1031;
// Initialize the ADM1031 chip
    adm1031_init_client(client);
// sysfs hooks
    data.groups[0] = &adm1031_group;
    if (data.chip_type == adm1031)
    data.groups[1] = &adm1031_group_opt;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, data.groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id adm1031_id[] = {
    { .name = "adm1030", .driver_data = adm1030 },
    { .name = "adm1031", .driver_data = adm1031 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adm1031_id);
    static struct i2c_driver adm1031_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name = "adm1031",
    },
    .probe		= adm1031_probe,
    .id_table	= adm1031_id,
    .detect		= adm1031_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(adm1031_driver);
    MODULE_AUTHOR("Alexandre d'Alton <alex@alexdalton.org>");
    MODULE_DESCRIPTION("ADM1031/ADM1030 driver");
    MODULE_LICENSE("GPL");
