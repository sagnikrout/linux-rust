//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lm63.c
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
// lm63.c - driver for the National Semiconductor LM63 temperature sensor
// with integrated fan control
// Copyright (C) 2004-2008  Jean Delvare <jdelvare@suse.de>
// Based on the lm90 driver.
//
// The LM63 is a sensor chip made by National Semiconductor. It measures
// two temperatures (its own and one external one) and the speed of one
// fan, those speed it can additionally control. Complete datasheet can be
// obtained from National's website at:
// http://www.national.com/pf/LM/LM63.html
//
// The LM63 is basically an LM86 with fan speed monitoring and control
// capabilities added. It misses some of the LM86 features though:
// - No low limit for local temperature.
// - No critical limit for local temperature.
// - Critical limit for remote temperature can be changed only once. We
// will consider that the critical limit is read-only.
//
// The datasheet isn't very clear about what the tachometer reading is.
// I had a explanation from National Semiconductor though. The two lower
// bits of the read value have to be masked out. The value is still 16 bit
// in width.
//

//
// Addresses to scan
// Address is fully defined internally and cannot be changed except for
// LM64 which has one pin dedicated to address selection.
// LM63 and LM96163 have address 0x4c.
// LM64 can have address 0x18 or 0x4e.
//
    static const unsigned short normal_i2c[] = { 0x18, 0x4c, 0x4e, I2C_CLIENT_END };
//
// The LM63 registers
//
pub const LM63_REG_CONFIG1: c_uint = 0x03;
pub const LM63_REG_CONVRATE: c_uint = 0x04;
pub const LM63_REG_CONFIG2: c_uint = 0xBF;
pub const LM63_REG_CONFIG_FAN: c_uint = 0x4A;
pub const LM63_REG_TACH_COUNT_MSB: c_uint = 0x47;
pub const LM63_REG_TACH_COUNT_LSB: c_uint = 0x46;
pub const LM63_REG_TACH_LIMIT_MSB: c_uint = 0x49;
pub const LM63_REG_TACH_LIMIT_LSB: c_uint = 0x48;
pub const LM63_REG_PWM_VALUE: c_uint = 0x4C;
pub const LM63_REG_PWM_FREQ: c_uint = 0x4D;
pub const LM63_REG_LUT_TEMP_HYST: c_uint = 0x4F;

pub const LM63_REG_LOCAL_TEMP: c_uint = 0x00;
pub const LM63_REG_LOCAL_HIGH: c_uint = 0x05;
pub const LM63_REG_REMOTE_TEMP_MSB: c_uint = 0x01;
pub const LM63_REG_REMOTE_TEMP_LSB: c_uint = 0x10;
pub const LM63_REG_REMOTE_OFFSET_MSB: c_uint = 0x11;
pub const LM63_REG_REMOTE_OFFSET_LSB: c_uint = 0x12;
pub const LM63_REG_REMOTE_HIGH_MSB: c_uint = 0x07;
pub const LM63_REG_REMOTE_HIGH_LSB: c_uint = 0x13;
pub const LM63_REG_REMOTE_LOW_MSB: c_uint = 0x08;
pub const LM63_REG_REMOTE_LOW_LSB: c_uint = 0x14;
pub const LM63_REG_REMOTE_TCRIT: c_uint = 0x19;
pub const LM63_REG_REMOTE_TCRIT_HYST: c_uint = 0x21;
pub const LM63_REG_ALERT_STATUS: c_uint = 0x02;
pub const LM63_REG_ALERT_MASK: c_uint = 0x16;
pub const LM63_REG_MAN_ID: c_uint = 0xFE;
pub const LM63_REG_CHIP_ID: c_uint = 0xFF;
pub const LM96163_REG_TRUTHERM: c_uint = 0x30;
pub const LM96163_REG_REMOTE_TEMP_U_MSB: c_uint = 0x31;
pub const LM96163_REG_REMOTE_TEMP_U_LSB: c_uint = 0x32;
pub const LM96163_REG_CONFIG_ENHANCED: c_uint = 0x45;
pub const LM63_PWM_BASE_FAST_HZ: c_int = 180000;
pub const LM63_PWM_BASE_SLOW_HZ: c_int = 700;
pub const LM63_MAX_CONVRATE: c_int = 9;
pub const LM63_MAX_CONVRATE_HZ: c_int = 32;
pub const LM96163_MAX_CONVRATE_HZ: c_int = 26;
//
// Conversions and various macros
// For tachometer counts, the LM63 uses 16-bit values.
// For local temperature and high limit, remote critical limit and hysteresis
// value, it uses signed 8-bit values with LSB = 1 degree Celsius.
// For remote temperature, low and high limits, it uses signed 11-bit values
// with LSB = 0.125 degree Celsius, left-justified in 16-bit registers.
// For LM64 the actual remote diode temperature is 16 degree Celsius higher
// than the register reading. Remote temperature setpoints have to be
// adapted accordingly.
//

    5400000 / (reg))

    (5400000 / (val)) & 0xFFFC)

    127000), 1000)

    255000), 1000)

    127875), 125) * 32)

    255875), 125) * 32)

    1000)

    ((1000 << (LM63_MAX_CONVRATE - (rate))) / (max))
    enum chips { lm63, lm64, lm96163 };
//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm63_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub groups: [*const attribute_group; 5],
    pub /: *mut *mut bool valid; / false until following fields are valid,
    pub /: *mut *mut char lut_valid; / zero until lut fields are valid,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub /: *mut *mut unsigned long lut_last_updated; / in jiffies,
    pub kind: enum chips,
    pub temp2_offset: c_int,
    pub /: *mut *mut int update_interval; / in milliseconds,
    pub max_convrate_hz: c_int,
    pub /: *mut *mut int lut_size; / 8 or 12,
// registers values
    pub config_fan: u8 config,,
    pub input: *mut *mut u16 fan[2]; / 0:,
    1: low limit */
    pub pwm1_freq: u8,
    pub output: *mut *mut u8 pwm1[13]; / 0: current,
    1-12: lookup table */
    pub input: *mut *mut s8 temp8[15]; / 0: local,
    1: local high limit
    2: remote critical limit
    3-14: lookup table */
    pub input: *mut *mut s16 temp11[4]; / 0: remote,
    1: remote low limit
    2: remote high limit
    3: remote offset */
    pub /: *mut *mut u16 temp11u; / remote input (unsigned),
    pub temp2_crit_hyst: u8,
    pub lut_temp_hyst: u8,
    pub alarms: u8,
    pub pwm_highres: bool,
    pub lut_temp_highres: bool,
    pub /: *mut *mut bool remote_unsigned; / true if unsigned remote upper limits,
    pub trutherm: bool,
}

#[no_mangle]
pub unsafe extern "C" fn temp8_from_reg(data: *mut lm63_data, nr: c_int) -> c_int {
    static inline int temp8_from_reg(struct lm63_data *data, int nr)
    {
    if (data.remote_unsigned)
    return TEMP8_FROM_REG((u8)data.temp8[nr]);
    return TEMP8_FROM_REG(data.temp8[nr]);
    }
#[no_mangle]
pub unsafe extern "C" fn lut_temp_from_reg(data: *mut lm63_data, nr: c_int) -> c_int {
    static inline int lut_temp_from_reg(struct lm63_data *data, int nr)
    {
    return data.temp8[nr] * (data.lut_temp_highres ? 500 : 1000);
    }
#[no_mangle]
pub unsafe extern "C" fn lut_temp_to_reg(data: *mut lm63_data, val: c_long) -> c_int {
    static inline int lut_temp_to_reg(struct lm63_data *data, long val)
    {
    val -= data.temp2_offset;
    if (data.lut_temp_highres)
    return DIV_ROUND_CLOSEST(clamp_val(val, 0, 127500), 500);
    else
    return DIV_ROUND_CLOSEST(clamp_val(val, 0, 127000), 1000);
    }
//
// Update the lookup table register cache.
// client->update_lock must be held when calling this function.
//
#[no_mangle]
unsafe extern "C" fn lm63_update_lut(data: *mut lm63_data) {
    static void lm63_update_lut(struct lm63_data *data)
    {
    struct i2c_client *client = data.client;
    int i;
    if (time_after(jiffies, data.lut_last_updated + 5 * HZ) ||
    !data.lut_valid) {
    for (i = 0; i < data.lut_size; i++) {
    data.pwm1[1 + i] = i2c_smbus_read_byte_data(client,
    LM63_REG_LUT_PWM(i));
    data.temp8[3 + i] = i2c_smbus_read_byte_data(client,
    LM63_REG_LUT_TEMP(i));
    }
    data.lut_temp_hyst = i2c_smbus_read_byte_data(client,
    LM63_REG_LUT_TEMP_HYST);
    data.lut_last_updated = jiffies;
    data.lut_valid = 1;
    }
    }
    static struct lm63_data *lm63_update_device(struct device *dev)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long next_update;
    mutex_lock(&data.update_lock);
    next_update = data.last_updated +
    msecs_to_jiffies(data.update_interval);
    if (time_after(jiffies, next_update) || !data.valid) {
    if (data.config & 0x04) { /* tachometer enabled  */
// order matters for fan1_input
    data.fan[0] = i2c_smbus_read_byte_data(client,
    LM63_REG_TACH_COUNT_LSB) & 0xFC;
    data.fan[0] |= i2c_smbus_read_byte_data(client,
    LM63_REG_TACH_COUNT_MSB) << 8;
    data.fan[1] = (i2c_smbus_read_byte_data(client,
    LM63_REG_TACH_LIMIT_LSB) & 0xFC)
    | (i2c_smbus_read_byte_data(client,
    LM63_REG_TACH_LIMIT_MSB) << 8);
    }
    data.pwm1_freq = i2c_smbus_read_byte_data(client, LM63_REG_PWM_FREQ) & 0x1f;
    if (data.pwm1_freq == 0)
    data.pwm1_freq = 1;
    data.pwm1[0] = i2c_smbus_read_byte_data(client,
    LM63_REG_PWM_VALUE);
    data.temp8[0] = i2c_smbus_read_byte_data(client,
    LM63_REG_LOCAL_TEMP);
    data.temp8[1] = i2c_smbus_read_byte_data(client,
    LM63_REG_LOCAL_HIGH);
// order matters for temp2_input
    data.temp11[0] = i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_TEMP_MSB) << 8;
    data.temp11[0] |= i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_TEMP_LSB);
    data.temp11[1] = (i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_LOW_MSB) << 8)
    | i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_LOW_LSB);
    data.temp11[2] = (i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_HIGH_MSB) << 8)
    | i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_HIGH_LSB);
    data.temp11[3] = (i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_OFFSET_MSB) << 8)
    | i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_OFFSET_LSB);
    if (data.kind == lm96163)
    data.temp11u = (i2c_smbus_read_byte_data(client,
    LM96163_REG_REMOTE_TEMP_U_MSB) << 8)
    | i2c_smbus_read_byte_data(client,
    LM96163_REG_REMOTE_TEMP_U_LSB);
    data.temp8[2] = i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_TCRIT);
    data.temp2_crit_hyst = i2c_smbus_read_byte_data(client,
    LM63_REG_REMOTE_TCRIT_HYST);
    data.alarms = i2c_smbus_read_byte_data(client,
    LM63_REG_ALERT_STATUS) & 0x7F;
    data.last_updated = jiffies;
    data.valid = true;
    }
    lm63_update_lut(data);
    mutex_unlock(&data.update_lock);
    return data;
    }
//
// Trip points in the lookup table should be in ascending order for both
// temperatures and PWM output values.
//
#[no_mangle]
unsafe extern "C" fn lm63_lut_looks_bad(dev: *mut device, data: *mut lm63_data) -> c_int {
    static int lm63_lut_looks_bad(struct device *dev, struct lm63_data *data)
    {
    int i;
    mutex_lock(&data.update_lock);
    lm63_update_lut(data);
    for (i = 1; i < data.lut_size; i++) {
    if (data.pwm1[1 + i - 1] > data.pwm1[1 + i]
    || data.temp8[3 + i - 1] > data.temp8[3 + i]) {
    dev_warn(dev,
    "Lookup table doesn't look sane (check entries %d and %d)\n",
    i, i + 1);
    break;
    }
    }
    mutex_unlock(&data.update_lock);
    let mut i: return = = data.lut_size ? 0 : 1;
    }
//
// Sysfs callback functions and files
//
    static ssize_t show_fan(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    int fan;
    mutex_lock(&data.update_lock);
    fan = FAN_FROM_REG(data.fan[attr.index]);
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", fan);
    }
    static ssize_t set_fan(struct device *dev, struct device_attribute *dummy,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.fan[1] = FAN_TO_REG(val);
    i2c_smbus_write_byte_data(client, LM63_REG_TACH_LIMIT_LSB,
    data.fan[1] & 0xFF);
    i2c_smbus_write_byte_data(client, LM63_REG_TACH_LIMIT_MSB,
    data.fan[1] >> 8);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t show_pwm1(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    let mut nr: c_int = attr.index;
    int pwm;
    mutex_lock(&data.update_lock);
    if (data.pwm_highres)
    pwm = data.pwm1[nr];
    else
    pwm = data.pwm1[nr] >= 2 * data.pwm1_freq ?
    255 : (data.pwm1[nr] * 255 + data.pwm1_freq) /
    (2 * data.pwm1_freq);
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", pwm);
    }
    static ssize_t set_pwm1(struct device *dev, struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = attr.index;
    unsigned long val;
    int err;
    u8 reg;
    if (!(data.config_fan & 0x20)) /* register is read-only */
    return -EPERM;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    reg = nr ? LM63_REG_LUT_PWM(nr - 1) : LM63_REG_PWM_VALUE;
    val = clamp_val(val, 0, 255);
    mutex_lock(&data.update_lock);
    data.pwm1[nr] = data.pwm_highres ? val :
    (val * data.pwm1_freq * 2 + 127) / 255;
    i2c_smbus_write_byte_data(client, reg, data.pwm1[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t pwm1_enable_show(struct device *dev,
    struct device_attribute *dummy, char *buf)
    {
    struct lm63_data *data = lm63_update_device(dev);
    return sprintf(buf, "%d\n", data.config_fan & 0x20 ? 1 : 2);
    }
    static ssize_t pwm1_enable_store(struct device *dev,
    struct device_attribute *dummy,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    if (val < 1 || val > 2)
    return -EINVAL;
//
// Only let the user switch to automatic mode if the lookup table
// looks sane.
//
    if (val == 2 && lm63_lut_looks_bad(dev, data))
    return -EPERM;
    mutex_lock(&data.update_lock);
    data.config_fan = i2c_smbus_read_byte_data(client,
    LM63_REG_CONFIG_FAN);
    if (val == 1)
    data.config_fan |= 0x20;
    else
    data.config_fan &= ~0x20;
    i2c_smbus_write_byte_data(client, LM63_REG_CONFIG_FAN,
    data.config_fan);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t pwm1_freq_show(struct device *dev,
    struct device_attribute *dummy, char *buf)
    {
    struct lm63_data *data = lm63_update_device(dev);
    unsigned int base, freq;
    mutex_lock(&data.update_lock);
    base = (data.config_fan & 0x08) ?
    LM63_PWM_BASE_SLOW_HZ : LM63_PWM_BASE_FAST_HZ;
    freq = data.pwm1_freq;
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%u\n", base / freq);
    }
//
// Pick the closest CONFIG_FAN.SCS + PFR for the requested frequency.
// PWM_FREQ writes need the LUT unlocked, same as set_pwm1(). LUT PWM
// bytes are register-relative; rewrite them after a frequency change
// if duty cycles must be preserved.
//
    static ssize_t pwm1_freq_store(struct device *dev,
    struct device_attribute *dummy,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val, pfr_fast, pfr_slow, err_fast, err_slow, pfr;
    bool slow_clock;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val == 0)
    return -EINVAL;
    pfr_fast = clamp_val(DIV_ROUND_CLOSEST((unsigned long)LM63_PWM_BASE_FAST_HZ, val),
    1UL, 31UL);
    pfr_slow = clamp_val(DIV_ROUND_CLOSEST((unsigned long)LM63_PWM_BASE_SLOW_HZ, val),
    1UL, 31UL);
    err_fast = abs_diff(LM63_PWM_BASE_FAST_HZ / pfr_fast, val);
    err_slow = abs_diff(LM63_PWM_BASE_SLOW_HZ / pfr_slow, val);
    if (err_slow < err_fast) {
    slow_clock = true;
    pfr = pfr_slow;
    } else {
    slow_clock = false;
    pfr = pfr_fast;
    }
    mutex_lock(&data.update_lock);
    ret = i2c_smbus_read_byte_data(client, LM63_REG_CONFIG_FAN);
    if (ret < 0) {
    mutex_unlock(&data.update_lock);
    return ret;
    }
    data.config_fan = ret;
    if (!(data.config_fan & 0x20)) { /* register is read-only */
    mutex_unlock(&data.update_lock);
    return -EPERM;
    }
    if (data.kind == lm96163) {
    ret = i2c_smbus_read_byte_data(client, LM96163_REG_CONFIG_ENHANCED);
    if (ret < 0) {
    mutex_unlock(&data.update_lock);
    return ret;
    }
    data.pwm_highres = !slow_clock && pfr == 8 && (ret & 0x10);
    }
    if (slow_clock)
    data.config_fan |= 0x08;
    else
    data.config_fan &= ~0x08;
    i2c_smbus_write_byte_data(client, LM63_REG_CONFIG_FAN, data.config_fan);
    i2c_smbus_write_byte_data(client, LM63_REG_PWM_FREQ, pfr);
    data.pwm1_freq = pfr;
    mutex_unlock(&data.update_lock);
    return count;
    }
//
// There are 8bit registers for both local(temp1) and remote(temp2) sensor.
// For remote sensor registers temp2_offset has to be considered,
// for local sensor it must not.
// So we need separate 8bit accessors for local and remote sensor.
//
    static ssize_t show_local_temp8(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    return sprintf(buf, "%d\n", TEMP8_FROM_REG(data.temp8[attr.index]));
    }
    static ssize_t show_remote_temp8(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
#[no_mangle]
pub unsafe extern "C" fn sprintf(_arg: buf, _arg: "%d\n", _arg: temp8_from_reg(data, _arg: attr->index) -> return {
    return sprintf(buf, "%d\n", temp8_from_reg(data, attr.index)
    + data.temp2_offset);
    }
    static ssize_t show_lut_temp(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
#[no_mangle]
pub unsafe extern "C" fn sprintf(_arg: buf, _arg: "%d\n", _arg: lut_temp_from_reg(data, _arg: attr->index) -> return {
    return sprintf(buf, "%d\n", lut_temp_from_reg(data, attr.index)
    + data.temp2_offset);
    }
    static ssize_t set_temp8(struct device *dev, struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut nr: c_int = attr.index;
    long val;
    int err;
    int temp;
    u8 reg;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    switch (nr) {
    case 2:
    reg = LM63_REG_REMOTE_TCRIT;
    if (data.remote_unsigned)
    temp = TEMP8U_TO_REG(val - data.temp2_offset);
    else
    temp = TEMP8_TO_REG(val - data.temp2_offset);
    break;
    case 1:
    reg = LM63_REG_LOCAL_HIGH;
    temp = TEMP8_TO_REG(val);
    break;
    default:	/* lookup table */
    reg = LM63_REG_LUT_TEMP(nr - 3);
    temp = lut_temp_to_reg(data, val);
    }
    data.temp8[nr] = temp;
    i2c_smbus_write_byte_data(client, reg, temp);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t show_temp11(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    let mut nr: c_int = attr.index;
    int temp;
    mutex_lock(&data.update_lock);
    if (!nr) {
//
// Use unsigned temperature unless its value is zero.
// If it is zero, use signed temperature.
//
    if (data.temp11u)
    temp = TEMP11_FROM_REG(data.temp11u);
    else
    temp = TEMP11_FROM_REG(data.temp11[nr]);
    } else {
    if (data.remote_unsigned && nr == 2)
    temp = TEMP11_FROM_REG((u16)data.temp11[nr]);
    else
    temp = TEMP11_FROM_REG(data.temp11[nr]);
    }
    temp += data.temp2_offset;
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", temp);
    }
    static ssize_t set_temp11(struct device *dev, struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    static const u8 reg[6] = {
    LM63_REG_REMOTE_LOW_MSB,
    LM63_REG_REMOTE_LOW_LSB,
    LM63_REG_REMOTE_HIGH_MSB,
    LM63_REG_REMOTE_HIGH_LSB,
    LM63_REG_REMOTE_OFFSET_MSB,
    LM63_REG_REMOTE_OFFSET_LSB,
    };
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long val;
    int err;
    let mut nr: c_int = attr.index;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    if (data.remote_unsigned && nr == 2)
    data.temp11[nr] = TEMP11U_TO_REG(val - data.temp2_offset);
    else
    data.temp11[nr] = TEMP11_TO_REG(val - data.temp2_offset);
    i2c_smbus_write_byte_data(client, reg[(nr - 1) * 2],
    data.temp11[nr] >> 8);
    i2c_smbus_write_byte_data(client, reg[(nr - 1) * 2 + 1],
    data.temp11[nr] & 0xff);
    mutex_unlock(&data.update_lock);
    return count;
    }
//
// Hysteresis register holds a relative value, while we want to present
// an absolute to user-space
//
    static ssize_t temp2_crit_hyst_show(struct device *dev,
    struct device_attribute *dummy, char *buf)
    {
    struct lm63_data *data = lm63_update_device(dev);
    int temp;
    mutex_lock(&data.update_lock);
    temp = temp8_from_reg(data, 2) + data.temp2_offset
    - TEMP8_FROM_REG(data.temp2_crit_hyst);
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", temp);
    }
    static ssize_t show_lut_temp_hyst(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    int temp;
    mutex_lock(&data.update_lock);
    temp = lut_temp_from_reg(data, attr.index) + data.temp2_offset
    - TEMP8_FROM_REG(data.lut_temp_hyst);
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", temp);
    }
//
// The LM63 has a single hysteresis register shared by all LUT entries.
// Expose it as a chip-wide hysteresis amount in millidegrees; the
// per-point pwm1_auto_pointN_temp_hyst attributes remain read-only and
// show the resulting absolute trip-down temperature for each entry.
//
    static ssize_t pwm1_auto_point_temp_hyst_show(struct device *dev,
    struct device_attribute *dummy,
    char *buf)
    {
    struct lm63_data *data = lm63_update_device(dev);
    u8 hyst;
    mutex_lock(&data.update_lock);
    hyst = data.lut_temp_hyst;
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", TEMP8_FROM_REG(hyst));
    }
    static ssize_t pwm1_auto_point_temp_hyst_store(struct device *dev,
    struct device_attribute *dummy,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.lut_temp_hyst = HYST_TO_REG(val);
    i2c_smbus_write_byte_data(client, LM63_REG_LUT_TEMP_HYST,
    data.lut_temp_hyst);
    mutex_unlock(&data.update_lock);
    return count;
    }
//
// And now the other way around, user-space provides an absolute
// hysteresis value and we have to store a relative one
//
    static ssize_t temp2_crit_hyst_store(struct device *dev,
    struct device_attribute *dummy,
    const char *buf, size_t count)
    {
    struct lm63_data *data = lm63_update_device(dev);
    struct i2c_client *client = data.client;
    long val;
    int err;
    long hyst;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    hyst = temp8_from_reg(data, 2) + data.temp2_offset - val;
    i2c_smbus_write_byte_data(client, LM63_REG_REMOTE_TCRIT_HYST,
    HYST_TO_REG(hyst));
    mutex_unlock(&data.update_lock);
    return count;
    }
//
// Set conversion rate.
// client->update_lock must be held when calling this function.
//
#[no_mangle]
unsafe extern "C" fn lm63_set_convrate(data: *mut lm63_data, interval: c_uint) {
    static void lm63_set_convrate(struct lm63_data *data, unsigned int interval)
    {
    struct i2c_client *client = data.client;
    unsigned int update_interval;
    int i;
// Shift calculations to avoid rounding errors
    interval <<= 6;
// find the nearest update rate
    update_interval = (1 << (LM63_MAX_CONVRATE + 6)) * 1000
    / data.max_convrate_hz;
    for (i = 0; i < LM63_MAX_CONVRATE; i++, update_interval >>= 1)
    if (interval >= update_interval * 3 / 4)
    break;
    i2c_smbus_write_byte_data(client, LM63_REG_CONVRATE, i);
    data.update_interval = UPDATE_INTERVAL(data.max_convrate_hz, i);
    }
    static ssize_t update_interval_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", data.update_interval);
    }
    static ssize_t update_interval_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    lm63_set_convrate(data, clamp_val(val, 0, 100000));
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp2_type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    return sprintf(buf, data.trutherm ? "1\n" : "2\n");
    }
    static ssize_t temp2_type_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct lm63_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    unsigned long val;
    int ret;
    u8 reg;
    ret = kstrtoul(buf, 10, &val);
    if (ret < 0)
    return ret;
    if (val != 1 && val != 2)
    return -EINVAL;
    mutex_lock(&data.update_lock);
    data.trutherm = val == 1;
    reg = i2c_smbus_read_byte_data(client, LM96163_REG_TRUTHERM) & ~0x02;
    i2c_smbus_write_byte_data(client, LM96163_REG_TRUTHERM,
    reg | (data.trutherm ? 0x02 : 0x00));
    data.valid = false;
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t alarms_show(struct device *dev, struct device_attribute *dummy,
    char *buf)
    {
    struct lm63_data *data = lm63_update_device(dev);
    return sprintf(buf, "%u\n", data.alarms);
    }
    static ssize_t show_alarm(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct lm63_data *data = lm63_update_device(dev);
    let mut bitnr: c_int = attr.index;
    return sprintf(buf, "%u\n", (data.alarms >> bitnr) & 1);
    }
    static SENSOR_DEVICE_ATTR(fan1_input, S_IRUGO, show_fan, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(fan1_min, S_IWUSR | S_IRUGO, show_fan,
    set_fan, 1);
    static SENSOR_DEVICE_ATTR(pwm1, S_IWUSR | S_IRUGO, show_pwm1, set_pwm1, 0);
    static DEVICE_ATTR_RW(pwm1_enable);
    static DEVICE_ATTR_RW(pwm1_freq);
    static DEVICE_ATTR_RW(pwm1_auto_point_temp_hyst);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point1_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 1);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point1_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 3);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point1_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point2_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 2);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point2_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 4);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point2_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point3_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 3);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point3_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 5);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point3_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 5);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point4_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 4);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point4_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 6);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point4_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 6);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point5_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 5);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point5_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 7);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point5_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 7);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point6_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 6);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point6_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 8);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point6_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 8);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point7_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 7);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point7_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 9);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point7_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 9);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point8_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 8);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point8_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 10);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point8_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 10);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point9_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 9);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point9_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 11);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point9_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 11);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point10_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 10);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point10_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 12);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point10_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 12);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point11_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 11);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point11_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 13);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point11_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 13);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point12_pwm, S_IWUSR | S_IRUGO,
    show_pwm1, set_pwm1, 12);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point12_temp, S_IWUSR | S_IRUGO,
    show_lut_temp, set_temp8, 14);
    static SENSOR_DEVICE_ATTR(pwm1_auto_point12_temp_hyst, S_IRUGO,
    show_lut_temp_hyst, core::ptr::null_mut(), 14);
    static SENSOR_DEVICE_ATTR(temp1_input, S_IRUGO, show_local_temp8, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(temp1_max, S_IWUSR | S_IRUGO, show_local_temp8,
    set_temp8, 1);
    static SENSOR_DEVICE_ATTR(temp2_input, S_IRUGO, show_temp11, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(temp2_min, S_IWUSR | S_IRUGO, show_temp11,
    set_temp11, 1);
    static SENSOR_DEVICE_ATTR(temp2_max, S_IWUSR | S_IRUGO, show_temp11,
    set_temp11, 2);
    static SENSOR_DEVICE_ATTR(temp2_offset, S_IWUSR | S_IRUGO, show_temp11,
    set_temp11, 3);
    static SENSOR_DEVICE_ATTR(temp2_crit, S_IRUGO, show_remote_temp8,
    set_temp8, 2);
    static DEVICE_ATTR_RW(temp2_crit_hyst);
    static DEVICE_ATTR_RW(temp2_type);
// Individual alarm files
    static SENSOR_DEVICE_ATTR(fan1_min_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(temp2_crit_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(temp2_fault, S_IRUGO, show_alarm, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(temp2_min_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(temp2_max_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(temp1_max_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 6);
// Raw alarm file for compatibility
    static DEVICE_ATTR_RO(alarms);
    static DEVICE_ATTR_RW(update_interval);
    static struct attribute *lm63_attributes[] = {
    &sensor_dev_attr_pwm1.dev_attr.attr,
    &dev_attr_pwm1_enable.attr,
    &dev_attr_pwm1_freq.attr,
    &dev_attr_pwm1_auto_point_temp_hyst.attr,
    &sensor_dev_attr_pwm1_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point1_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point2_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point3_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point3_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point3_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point4_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point4_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point4_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point5_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point5_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point5_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point6_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point6_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point6_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point7_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point7_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point7_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point8_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point8_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point8_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_min.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp2_offset.dev_attr.attr,
    &sensor_dev_attr_temp2_crit.dev_attr.attr,
    &dev_attr_temp2_crit_hyst.attr,
    &sensor_dev_attr_temp2_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_fault.dev_attr.attr,
    &sensor_dev_attr_temp2_min_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_max_alarm.dev_attr.attr,
    &dev_attr_alarms.attr,
    &dev_attr_update_interval.attr,
    core::ptr::null_mut()
    };
    static struct attribute *lm63_attributes_temp2_type[] = {
    &dev_attr_temp2_type.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group lm63_group_temp2_type = {
    .attrs = lm63_attributes_temp2_type,
    };
    static struct attribute *lm63_attributes_extra_lut[] = {
    &sensor_dev_attr_pwm1_auto_point9_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point9_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point9_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point10_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point10_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point10_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point11_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point11_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point11_temp_hyst.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point12_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point12_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point12_temp_hyst.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group lm63_group_extra_lut = {
    .attrs = lm63_attributes_extra_lut,
    };
//
// On LM63, temp2_crit can be set only once, which should be job
// of the bootloader.
// On LM64, temp2_crit can always be set.
// On LM96163, temp2_crit can be set if bit 1 of the configuration
// register is true.
//
    static umode_t lm63_attribute_mode(struct kobject *kobj,
    struct attribute *attr, int index)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct lm63_data *data = dev_get_drvdata(dev);
    if (attr == &sensor_dev_attr_temp2_crit.dev_attr.attr
    && (data.kind == lm64 ||
    (data.kind == lm96163 && (data.config & 0x02))))
    return attr.mode | S_IWUSR;
    return attr.mode;
    }
    static const struct attribute_group lm63_group = {
    .is_visible = lm63_attribute_mode,
    .attrs = lm63_attributes,
    };
    static struct attribute *lm63_attributes_fan1[] = {
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan1_min_alarm.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group lm63_group_fan1 = {
    .attrs = lm63_attributes_fan1,
    };
//
// Real code
//
// Return 0 if detection is successful, -ENODEV otherwise
    static int lm63_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    u8 man_id, chip_id, reg_config1, reg_config2;
    u8 reg_alert_status, reg_alert_mask;
    let mut address: c_int = client.addr;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    man_id = i2c_smbus_read_byte_data(client, LM63_REG_MAN_ID);
    chip_id = i2c_smbus_read_byte_data(client, LM63_REG_CHIP_ID);
    reg_config1 = i2c_smbus_read_byte_data(client, LM63_REG_CONFIG1);
    reg_config2 = i2c_smbus_read_byte_data(client, LM63_REG_CONFIG2);
    reg_alert_status = i2c_smbus_read_byte_data(client,
    LM63_REG_ALERT_STATUS);
    reg_alert_mask = i2c_smbus_read_byte_data(client, LM63_REG_ALERT_MASK);
    if (man_id != 0x01 /* National Semiconductor */
    || (reg_config1 & 0x18) != 0x00
    || (reg_config2 & 0xF8) != 0x00
    || (reg_alert_status & 0x20) != 0x00
    || (reg_alert_mask & 0xA4) != 0xA4) {
    dev_dbg(&adapter.dev,
    "Unsupported chip (man_id=0x%02X, chip_id=0x%02X)\n",
    man_id, chip_id);
    return -ENODEV;
    }
    if (chip_id == 0x41 && address == 0x4c)
    strscpy(info.type, "lm63", I2C_NAME_SIZE);
#[no_mangle]
pub unsafe extern "C" fn if(0x4e): chip_id == 0x51 && (address == 0x18 || address ==) -> else {
    else if (chip_id == 0x51 && (address == 0x18 || address == 0x4e))
    strscpy(info.type, "lm64", I2C_NAME_SIZE);
#[no_mangle]
pub unsafe extern "C" fn if(0x4c: chip_id == 0x49 && address ==) -> else {
    else if (chip_id == 0x49 && address == 0x4c)
    strscpy(info.type, "lm96163", I2C_NAME_SIZE);
    else
    return -ENODEV;
    return 0;
    }
//
// Ideally we shouldn't have to initialize anything, since the BIOS
// should have taken care of everything
//
#[no_mangle]
unsafe extern "C" fn lm63_init_client(data: *mut lm63_data) {
    static void lm63_init_client(struct lm63_data *data)
    {
    struct i2c_client *client = data.client;
    struct device *dev = &client.dev;
    u8 convrate;
    data.config = i2c_smbus_read_byte_data(client, LM63_REG_CONFIG1);
    data.config_fan = i2c_smbus_read_byte_data(client,
    LM63_REG_CONFIG_FAN);
// Start converting if needed
    if (data.config & 0x40) { /* standby */
    dev_dbg(dev, "Switching to operational mode\n");
    data.config &= 0xA7;
    i2c_smbus_write_byte_data(client, LM63_REG_CONFIG1,
    data.config);
    }
// Tachometer is always enabled on LM64
    if (data.kind == lm64)
    data.config |= 0x04;
// We may need pwm1_freq before ever updating the client data
    data.pwm1_freq = i2c_smbus_read_byte_data(client, LM63_REG_PWM_FREQ) & 0x1f;
    if (data.pwm1_freq == 0)
    data.pwm1_freq = 1;
    switch (data.kind) {
    case lm63:
    case lm64:
    data.max_convrate_hz = LM63_MAX_CONVRATE_HZ;
    data.lut_size = 8;
    break;
    case lm96163:
    data.max_convrate_hz = LM96163_MAX_CONVRATE_HZ;
    data.lut_size = 12;
    data.trutherm
    = i2c_smbus_read_byte_data(client,
    LM96163_REG_TRUTHERM) & 0x02;
    break;
    }
    convrate = i2c_smbus_read_byte_data(client, LM63_REG_CONVRATE);
    if (unlikely(convrate > LM63_MAX_CONVRATE))
    convrate = LM63_MAX_CONVRATE;
    data.update_interval = UPDATE_INTERVAL(data.max_convrate_hz,
    convrate);
//
// For LM96163, check if high resolution PWM
// and unsigned temperature format is enabled.
//
    if (data.kind == lm96163) {
    u8 config_enhanced
    = i2c_smbus_read_byte_data(client,
    LM96163_REG_CONFIG_ENHANCED);
    if (config_enhanced & 0x20)
    data.lut_temp_highres = true;
    if ((config_enhanced & 0x10)
    && !(data.config_fan & 0x08) && data.pwm1_freq == 8)
    data.pwm_highres = true;
    if (config_enhanced & 0x08)
    data.remote_unsigned = true;
    }
// Show some debug info about the LM63 configuration
    if (data.kind == lm63)
    dev_dbg(dev, "Alert/tach pin configured for %s\n",
    (data.config & 0x04) ? "tachometer input" :
    "alert output");
    dev_dbg(dev, "PWM clock %s kHz, output frequency %u Hz\n",
    (data.config_fan & 0x08) ? "1.4" : "360",
    ((data.config_fan & 0x08) ? 700 : 180000) / data.pwm1_freq);
    dev_dbg(dev, "PWM output active %s, %s mode\n",
    (data.config_fan & 0x10) ? "low" : "high",
    (data.config_fan & 0x20) ? "manual" : "auto");
    }
    static const struct i2c_device_id lm63_id[];
#[no_mangle]
unsafe extern "C" fn lm63_probe(client: *mut i2c_client) -> c_int {
    static int lm63_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct lm63_data *data;
    let mut groups: c_int = 0;
    data = devm_kzalloc(dev, sizeof(struct lm63_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
// Set the device type
    data.kind = (uintptr_t)i2c_get_match_data(client);
    if (data.kind == lm64)
    data.temp2_offset = 16000;
// Initialize chip
    lm63_init_client(data);
// Register sysfs hooks
    data.groups[groups++] = &lm63_group;
    if (data.config & 0x04)	/* tachometer enabled */
    data.groups[groups++] = &lm63_group_fan1;
    if (data.kind == lm96163) {
    data.groups[groups++] = &lm63_group_temp2_type;
    data.groups[groups++] = &lm63_group_extra_lut;
    }
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, data.groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
//
// Driver data (common to all clients)
//
    static const struct i2c_device_id lm63_id[] = {
    { .name = "lm63", .driver_data = lm63 },
    { .name = "lm64", .driver_data = lm64 },
    { .name = "lm96163", .driver_data = lm96163 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm63_id);
    static const struct of_device_id __maybe_unused lm63_of_match[] = {
    {
    .compatible = "national,lm63",
    .data = (void *)lm63
    },
    {
    .compatible = "national,lm64",
    .data = (void *)lm64
    },
    {
    .compatible = "national,lm96163",
    .data = (void *)lm96163
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, lm63_of_match);
    static struct i2c_driver lm63_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "lm63",
    .of_match_table = of_match_ptr(lm63_of_match),
    },
    .probe		= lm63_probe,
    .id_table	= lm63_id,
    .detect		= lm63_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(lm63_driver);
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("LM63 driver");
    MODULE_LICENSE("GPL");
