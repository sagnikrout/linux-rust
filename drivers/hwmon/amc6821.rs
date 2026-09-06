//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/amc6821.c
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
// amc6821.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
// Copyright (C) 2009 T. Mertelj <tomaz.mertelj@guest.arnes.si>
//
// Based on max6650.c:
// Copyright (C) 2007 Hans J. Koch <hjk@hansjkoch.de>
//
// Conversion to regmap and with_info API:
// Copyright (C) 2024 Guenter Roeck <linux@roeck-us.net>
//

//
// Addresses to scan.
//
    static const unsigned short normal_i2c[] = {0x18, 0x19, 0x1a, 0x2c, 0x2d, 0x2e,
    0x4c, 0x4d, 0x4e, I2C_CLIENT_END};
//
// Insmod parameters
//
    static int pwminv = -1; /*Inverted PWM output. */
    module_param(pwminv, int, 0444);
    static int init = 1; /*Power-on initialization.*/
    module_param(init, int, 0444);
pub const AMC6821_REG_DEV_ID: c_uint = 0x3D;
pub const AMC6821_REG_COMP_ID: c_uint = 0x3E;
pub const AMC6821_REG_CONF1: c_uint = 0x00;
pub const AMC6821_REG_CONF2: c_uint = 0x01;
pub const AMC6821_REG_CONF3: c_uint = 0x3F;
pub const AMC6821_REG_CONF4: c_uint = 0x04;
pub const AMC6821_REG_STAT1: c_uint = 0x02;
pub const AMC6821_REG_STAT2: c_uint = 0x03;
pub const AMC6821_REG_TEMP_LO: c_uint = 0x06;
pub const AMC6821_REG_TDATA_LOW: c_uint = 0x08;
pub const AMC6821_REG_TDATA_HI: c_uint = 0x09;
pub const AMC6821_REG_LTEMP_HI: c_uint = 0x0A;
pub const AMC6821_REG_RTEMP_HI: c_uint = 0x0B;
pub const AMC6821_REG_LTEMP_LIMIT_MIN: c_uint = 0x15;
pub const AMC6821_REG_LTEMP_LIMIT_MAX: c_uint = 0x14;
pub const AMC6821_REG_RTEMP_LIMIT_MIN: c_uint = 0x19;
pub const AMC6821_REG_RTEMP_LIMIT_MAX: c_uint = 0x18;
pub const AMC6821_REG_LTEMP_CRIT: c_uint = 0x1B;
pub const AMC6821_REG_RTEMP_CRIT: c_uint = 0x1D;
pub const AMC6821_REG_PSV_TEMP: c_uint = 0x1C;
pub const AMC6821_REG_DCY: c_uint = 0x22;
pub const AMC6821_REG_LTEMP_FAN_CTRL: c_uint = 0x24;
pub const AMC6821_REG_RTEMP_FAN_CTRL: c_uint = 0x25;
pub const AMC6821_REG_DCY_LOW_TEMP: c_uint = 0x21;
pub const AMC6821_REG_TACH_LLIMITL: c_uint = 0x10;
pub const AMC6821_REG_TACH_HLIMITL: c_uint = 0x12;
pub const AMC6821_REG_TACH_SETTINGL: c_uint = 0x1e;

//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amc6821_data {
    pub regmap: *mut regmap,
    pub update_lock: mutex,
    pub fan_state: c_ulong,
    pub fan_max_state: c_ulong,
    pub fan_cooling_levels: *mut c_uint,
    pub pwm_polarity: enum pwm_polarity,
}

//
// Return 0 on success or negative error code.
//
// temps returns set of three temperatures, in °C:
// temps[0]: Passive cooling temperature, applies to both channels
// temps[1]: Low temperature, start slope calculations
// temps[2]: High temperature
//
// Channel 0: local, channel 1: remote.
//
#[no_mangle]
unsafe extern "C" fn amc6821_get_auto_point_temps(regmap: *mut regmap, channel: c_int, temps: *mut u8) -> c_int {
    static int amc6821_get_auto_point_temps(struct regmap *regmap, int channel, u8 *temps)
    {
    u32 regs[] = {
    AMC6821_REG_DCY_LOW_TEMP,
    AMC6821_REG_PSV_TEMP,
    channel ? AMC6821_REG_RTEMP_FAN_CTRL : AMC6821_REG_LTEMP_FAN_CTRL
    };
    u8 regvals[3];
    int slope;
    int err;
    err = regmap_multi_reg_read(regmap, regs, regvals, 3);
    if (err)
    return err;
    temps[0] = regvals[1];
    temps[1] = FIELD_GET(AMC6821_TEMP_LIMIT_MASK, regvals[2]) * 4;
// slope is 32 >> <slope bits> in °C
    slope = 32 >> FIELD_GET(AMC6821_TEMP_SLOPE_MASK, regvals[2]);
    if (slope)
    temps[2] = temps[1] + DIV_ROUND_CLOSEST(255 - regvals[0], slope);
    else
    temps[2] = 255;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_temp_read_values(regmap: *mut regmap, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int amc6821_temp_read_values(struct regmap *regmap, u32 attr, int channel, long *val)
    {
    int reg, err;
    u32 regval;
    switch (attr) {
    case hwmon_temp_input:
    reg = channel ? AMC6821_REG_RTEMP_HI : AMC6821_REG_LTEMP_HI;
    break;
    case hwmon_temp_min:
    reg = channel ? AMC6821_REG_RTEMP_LIMIT_MIN : AMC6821_REG_LTEMP_LIMIT_MIN;
    break;
    case hwmon_temp_max:
    reg = channel ? AMC6821_REG_RTEMP_LIMIT_MAX : AMC6821_REG_LTEMP_LIMIT_MAX;
    break;
    case hwmon_temp_crit:
    reg = channel ? AMC6821_REG_RTEMP_CRIT : AMC6821_REG_LTEMP_CRIT;
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = regmap_read(regmap, reg, &regval);
    if (err)
    return err;
// val = sign_extend32(regval, 7) * 1000;
    return 0;
    }
    static int amc6821_read_alarms(struct regmap *regmap, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    int reg, mask, err;
    u32 regval;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_min_alarm:
    reg = AMC6821_REG_STAT1;
    mask = channel ? AMC6821_STAT1_RTL : AMC6821_STAT1_LTL;
    break;
    case hwmon_temp_max_alarm:
    reg = AMC6821_REG_STAT1;
    mask = channel ? AMC6821_STAT1_RTH : AMC6821_STAT1_LTH;
    break;
    case hwmon_temp_crit_alarm:
    reg = AMC6821_REG_STAT2;
    mask = channel ? AMC6821_STAT2_RTC : AMC6821_STAT2_LTC;
    break;
    case hwmon_temp_fault:
    reg = AMC6821_REG_STAT1;
    mask = AMC6821_STAT1_RTF;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_fault:
    reg = AMC6821_REG_STAT1;
    mask = AMC6821_STAT1_FANS;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = regmap_read(regmap, reg, &regval);
    if (err)
    return err;
// val = !!(regval & mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_temp_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int amc6821_temp_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    return amc6821_temp_read_values(data.regmap, attr, channel, val);
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    case hwmon_temp_fault:
    return amc6821_read_alarms(data.regmap, hwmon_temp, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn amc6821_temp_write(dev: *mut device, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int amc6821_temp_write(struct device *dev, u32 attr, int channel, long val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    int reg;
    val = DIV_ROUND_CLOSEST(clamp_val(val, -128000, 127000), 1000);
    switch (attr) {
    case hwmon_temp_min:
    reg = channel ? AMC6821_REG_RTEMP_LIMIT_MIN : AMC6821_REG_LTEMP_LIMIT_MIN;
    break;
    case hwmon_temp_max:
    reg = channel ? AMC6821_REG_RTEMP_LIMIT_MAX : AMC6821_REG_LTEMP_LIMIT_MAX;
    break;
    case hwmon_temp_crit:
    reg = channel ? AMC6821_REG_RTEMP_CRIT : AMC6821_REG_LTEMP_CRIT;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return regmap_write(data.regmap, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn amc6821_pwm_read(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int amc6821_pwm_read(struct device *dev, u32 attr, long *val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u32 regval;
    int err;
    switch (attr) {
    case hwmon_pwm_enable:
    err = regmap_read(regmap, AMC6821_REG_CONF1, &regval);
    if (err)
    return err;
    switch (regval & (AMC6821_CONF1_FDRC0 | AMC6821_CONF1_FDRC1)) {
    case 0:
// val = 1;	/* manual
    break;
    case AMC6821_CONF1_FDRC0:
// val = 4;	/* target rpm (fan1_target) controlled
    break;
    case AMC6821_CONF1_FDRC1:
// val = 2;	/* remote temp controlled
    break;
    default:
// val = 3;	/* max(local, remote) temp controlled
    break;
    }
    return 0;
    case hwmon_pwm_mode:
    err = regmap_read(regmap, AMC6821_REG_CONF2, &regval);
    if (err)
    return err;
// val = !!(regval & AMC6821_CONF2_TACH_MODE);
    return 0;
    case hwmon_pwm_auto_channels_temp:
    err = regmap_read(regmap, AMC6821_REG_CONF1, &regval);
    if (err)
    return err;
    switch (regval & (AMC6821_CONF1_FDRC0 | AMC6821_CONF1_FDRC1)) {
    case 0:
    case AMC6821_CONF1_FDRC0:
// val = 0;	/* manual or target rpm controlled
    break;
    case AMC6821_CONF1_FDRC1:
// val = 2;	/* remote temp controlled
    break;
    default:
// val = 3;	/* max(local, remote) temp controlled
    break;
    }
    return 0;
    case hwmon_pwm_input:
    err = regmap_read(regmap, AMC6821_REG_DCY, &regval);
    if (err)
    return err;
// val = regval;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn amc6821_pwm_write(dev: *mut device, attr: u32, val: c_long) -> c_int {
    static int amc6821_pwm_write(struct device *dev, u32 attr, long val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u32 mode;
    switch (attr) {
    case hwmon_pwm_enable:
    switch (val) {
    case 1:
    mode = 0;
    break;
    case 2:
    mode = AMC6821_CONF1_FDRC1;
    break;
    case 3:
    mode = AMC6821_CONF1_FDRC0 | AMC6821_CONF1_FDRC1;
    break;
    case 4:
    mode = AMC6821_CONF1_FDRC0;
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(regmap, AMC6821_REG_CONF1,
    AMC6821_CONF1_FDRC0 | AMC6821_CONF1_FDRC1,
    mode);
    case hwmon_pwm_mode:
    if (val < 0 || val > 1)
    return -EINVAL;
    return regmap_update_bits(regmap, AMC6821_REG_CONF2,
    AMC6821_CONF2_TACH_MODE,
    val ? AMC6821_CONF2_TACH_MODE : 0);
    break;
    case hwmon_pwm_input:
    if (val < 0 || val > 255)
    return -EINVAL;
    return regmap_write(regmap, AMC6821_REG_DCY, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn amc6821_fan_read_rpm(regmap: *mut regmap, attr: u32, val: *mut c_long) -> c_int {
    static int amc6821_fan_read_rpm(struct regmap *regmap, u32 attr, long *val)
    {
    int reg, err;
    u8 regs[2];
    u32 regval;
    switch (attr) {
    case hwmon_fan_input:
    reg = AMC6821_REG_TDATA_LOW;
    break;
    case hwmon_fan_min:
    reg = AMC6821_REG_TACH_LLIMITL;
    break;
    case hwmon_fan_max:
    reg = AMC6821_REG_TACH_HLIMITL;
    break;
    case hwmon_fan_target:
    reg = AMC6821_REG_TACH_SETTINGL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = regmap_bulk_read(regmap, reg, regs, 2);
    if (err)
    return err;
    regval = (regs[1] << 8) | regs[0];
// val = regval ? 6000000 / regval : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_fan_read(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int amc6821_fan_read(struct device *dev, u32 attr, long *val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u32 regval;
    int err;
    switch (attr) {
    case hwmon_fan_input:
    case hwmon_fan_min:
    case hwmon_fan_max:
    case hwmon_fan_target:
    return amc6821_fan_read_rpm(regmap, attr, val);
    case hwmon_fan_fault:
    return amc6821_read_alarms(regmap, hwmon_fan, attr, 0, val);
    case hwmon_fan_pulses:
    err = regmap_read(regmap, AMC6821_REG_CONF4, &regval);
    if (err)
    return err;
// val = (regval & AMC6821_CONF4_PSPR) ? 4 : 2;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn amc6821_fan_write(dev: *mut device, attr: u32, val: c_long) -> c_int {
    static int amc6821_fan_write(struct device *dev, u32 attr, long val)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    u8 regs[2];
    int reg;
    if (attr == hwmon_fan_pulses) {
    if (val != 2 && val != 4)
    return -EINVAL;
    return regmap_update_bits(regmap, AMC6821_REG_CONF4,
    AMC6821_CONF4_PSPR,
    val == 4 ? AMC6821_CONF4_PSPR : 0);
    }
    if (val < 0)
    return -EINVAL;
    switch (attr) {
    case hwmon_fan_min:
    if (!val)	/* no unlimited minimum speed */
    return -EINVAL;
    reg = AMC6821_REG_TACH_LLIMITL;
    break;
    case hwmon_fan_max:
    reg = AMC6821_REG_TACH_HLIMITL;
    break;
    case hwmon_fan_target:
    if (!val)	/* no unlimited target speed */
    return -EINVAL;
    reg = AMC6821_REG_TACH_SETTINGL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    val = val ? 6000000 / clamp_val(val, 1, 6000000) : 0;
    val = clamp_val(val, 0, 0xffff);
    regs[0] = val & 0xff;
    regs[1] = val >> 8;
    return regmap_bulk_write(data.regmap, reg, regs, 2);
    }
    static ssize_t temp_auto_point_temp_show(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    let mut ix: c_int = to_sensor_dev_attr_2(devattr).index;
    let mut nr: c_int = to_sensor_dev_attr_2(devattr).nr;
    u8 temps[3];
    int err;
    mutex_lock(&data.update_lock);
    err = amc6821_get_auto_point_temps(data.regmap, nr, temps);
    mutex_unlock(&data.update_lock);
    if (err)
    return err;
    return sysfs_emit(buf, "%d\n", temps[ix] * 1000);
    }
    static ssize_t pwm1_auto_point_pwm_show(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    let mut ix: c_int = to_sensor_dev_attr(devattr).index;
    u32 val;
    int err;
    switch (ix) {
    case 0:
    val = 0;
    break;
    case 1:
    err = regmap_read(data.regmap, AMC6821_REG_DCY_LOW_TEMP, &val);
    if (err)
    return err;
    break;
    default:
    val = 255;
    break;
    }
    return sysfs_emit(buf, "%d\n", val);
    }
//
// Set TEMP[0-4] (low temperature) and SLP[0-2] (slope) of local or remote
// TEMP-FAN control register.
//
// Return 0 on success or negative error code.
//
// Channel 0: local, channel 1: remote
//
#[no_mangle]
pub unsafe extern "C" fn set_slope_register(regmap: *mut regmap, channel: c_int, temps: *mut u8) -> c_int {
    static inline int set_slope_register(struct regmap *regmap, int channel, u8 *temps)
    {
    let mut regval: u8 = FIELD_PREP(AMC6821_TEMP_LIMIT_MASK, temps[1] / 4);
    u8 tmp, dpwm;
    int err, dt;
    u32 pwm;
    err = regmap_read(regmap, AMC6821_REG_DCY_LOW_TEMP, &pwm);
    if (err)
    return err;
    dpwm = 255 - pwm;
    dt = temps[2] - temps[1];
    for (tmp = 4; tmp > 0; tmp--) {
    if (dt * (32 >> tmp) >= dpwm)
    break;
    }
    regval |= FIELD_PREP(AMC6821_TEMP_SLOPE_MASK, tmp);
    return regmap_write(regmap,
    channel ? AMC6821_REG_RTEMP_FAN_CTRL : AMC6821_REG_LTEMP_FAN_CTRL,
    regval);
    }
    static ssize_t temp_auto_point_temp_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    let mut ix: c_int = to_sensor_dev_attr_2(attr).index;
    let mut nr: c_int = to_sensor_dev_attr_2(attr).nr;
    struct regmap *regmap = data.regmap;
    u8 temps[3], otemps[3];
    long val;
    int ret;
    ret = kstrtol(buf, 10, &val);
    if (ret)
    return ret;
    mutex_lock(&data.update_lock);
    ret = amc6821_get_auto_point_temps(data.regmap, nr, temps);
    if (ret)
    goto unlock;
    switch (ix) {
    case 0:
//
// Passive cooling temperature. Range limit against low limit
// of both channels.
//
    ret = amc6821_get_auto_point_temps(data.regmap, 1 - nr, otemps);
    if (ret)
    goto unlock;
    val = DIV_ROUND_CLOSEST(clamp_val(val, 0, 63000), 1000);
    val = clamp_val(val, 0, min(temps[1], otemps[1]));
    ret = regmap_write(regmap, AMC6821_REG_PSV_TEMP, val);
    break;
    case 1:
//
// Low limit; must be between passive and high limit,
// and not exceed 124. Step size is 4 degrees C.
//
    val = clamp_val(val, DIV_ROUND_UP(temps[0], 4) * 4000, 124000);
    temps[1] = DIV_ROUND_CLOSEST(val, 4000) * 4;
    val = temps[1] / 4;
// Auto-adjust high limit if necessary
    temps[2] = clamp_val(temps[2], temps[1] + 1, 255);
    ret = set_slope_register(regmap, nr, temps);
    break;
    case 2:
// high limit, must be higher than low limit
    val = clamp_val(val, (temps[1] + 1) * 1000, 255000);
    temps[2] = DIV_ROUND_CLOSEST(val, 1000);
    ret = set_slope_register(regmap, nr, temps);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    unlock:
    mutex_unlock(&data.update_lock);
    return ret ? : count;
    }
    static ssize_t pwm1_auto_point_pwm_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct amc6821_data *data = dev_get_drvdata(dev);
    struct regmap *regmap = data.regmap;
    int i, ret;
    u8 val;
    ret = kstrtou8(buf, 10, &val);
    if (ret)
    return ret;
    mutex_lock(&data.update_lock);
    ret = regmap_write(regmap, AMC6821_REG_DCY_LOW_TEMP, val);
    if (ret)
    goto unlock;
    for (i = 0; i < 2; i++) {
    u8 temps[3];
    ret = amc6821_get_auto_point_temps(regmap, i, temps);
    if (ret)
    break;
    ret = set_slope_register(regmap, i, temps);
    if (ret)
    break;
    }
    unlock:
    mutex_unlock(&data.update_lock);
    return ret ? : count;
    }
    static SENSOR_DEVICE_ATTR_RO(pwm1_auto_point1_pwm, pwm1_auto_point_pwm, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm1_auto_point2_pwm, pwm1_auto_point_pwm, 1);
    static SENSOR_DEVICE_ATTR_RO(pwm1_auto_point3_pwm, pwm1_auto_point_pwm, 2);
    static SENSOR_DEVICE_ATTR_2_RO(temp1_auto_point1_temp, temp_auto_point_temp,
    0, 0);
    static SENSOR_DEVICE_ATTR_2_RW(temp1_auto_point2_temp, temp_auto_point_temp,
    0, 1);
    static SENSOR_DEVICE_ATTR_2_RW(temp1_auto_point3_temp, temp_auto_point_temp,
    0, 2);
    static SENSOR_DEVICE_ATTR_2_RW(temp2_auto_point1_temp, temp_auto_point_temp,
    1, 0);
    static SENSOR_DEVICE_ATTR_2_RW(temp2_auto_point2_temp, temp_auto_point_temp,
    1, 1);
    static SENSOR_DEVICE_ATTR_2_RW(temp2_auto_point3_temp, temp_auto_point_temp,
    1, 2);
    static struct attribute *amc6821_attrs[] = {
    &sensor_dev_attr_pwm1_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point3_pwm.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point3_temp.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point3_temp.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(amc6821);
    static int amc6821_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_temp:
    return amc6821_temp_read(dev, attr, channel, val);
    case hwmon_fan:
    return amc6821_fan_read(dev, attr, val);
    case hwmon_pwm:
    return amc6821_pwm_read(dev, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int amc6821_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_temp:
    return amc6821_temp_write(dev, attr, channel, val);
    case hwmon_fan:
    return amc6821_fan_write(dev, attr, val);
    case hwmon_pwm:
    return amc6821_pwm_write(dev, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t amc6821_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_crit_alarm:
    case hwmon_temp_fault:
    return 0444;
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_crit:
    return 0644;
    default:
    return 0;
    }
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    case hwmon_fan_fault:
    return 0444;
    case hwmon_fan_pulses:
    case hwmon_fan_min:
    case hwmon_fan_max:
    case hwmon_fan_target:
    return 0644;
    default:
    return 0;
    }
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_mode:
    case hwmon_pwm_enable:
    case hwmon_pwm_input:
    return 0644;
    case hwmon_pwm_auto_channels_temp:
    return 0444;
    default:
    return 0;
    }
    default:
    return 0;
    }
    }
    static const struct hwmon_channel_info * const amc6821_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_CRIT | HWMON_T_MIN_ALARM |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM |
    HWMON_T_FAULT),
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX |
    HWMON_F_TARGET | HWMON_F_PULSES | HWMON_F_FAULT),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE | HWMON_PWM_MODE |
    HWMON_PWM_AUTO_CHANNELS_TEMP),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops amc6821_hwmon_ops = {
    .is_visible = amc6821_is_visible,
    .read = amc6821_read,
    .write = amc6821_write,
    };
    static const struct hwmon_chip_info amc6821_chip_info = {
    .ops = &amc6821_hwmon_ops,
    .info = amc6821_info,
    };
#[no_mangle]
unsafe extern "C" fn amc6821_set_sw_dcy(data: *mut amc6821_data, duty_cycle: u8) -> c_int {
    static int amc6821_set_sw_dcy(struct amc6821_data *data, u8 duty_cycle)
    {
    int ret;
    ret = regmap_write(data.regmap, AMC6821_REG_DCY, duty_cycle);
    if (ret)
    return ret;
    return regmap_update_bits(data.regmap, AMC6821_REG_CONF1,
    AMC6821_CONF1_FDRC0 | AMC6821_CONF1_FDRC1, 0);
    }
#[no_mangle]
unsafe extern "C" fn amc6821_get_max_state(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    static int amc6821_get_max_state(struct thermal_cooling_device *cdev, unsigned long *state)
    {
    struct amc6821_data *data = cdev.devdata;
    if (!data)
    return -EINVAL;
// state = data->fan_max_state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_get_cur_state(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    static int amc6821_get_cur_state(struct thermal_cooling_device *cdev, unsigned long *state)
    {
    struct amc6821_data *data = cdev.devdata;
    if (!data)
    return -EINVAL;
// state = data->fan_state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_set_cur_state(cdev: *mut thermal_cooling_device, state: c_ulong) -> c_int {
    static int amc6821_set_cur_state(struct thermal_cooling_device *cdev, unsigned long state)
    {
    struct amc6821_data *data = cdev.devdata;
    int ret;
    if (!data || state > data.fan_max_state)
    return -EINVAL;
    ret = amc6821_set_sw_dcy(data, data.fan_cooling_levels[state]);
    if (ret)
    return ret;
    data.fan_state = state;
    return 0;
    }
    static const struct thermal_cooling_device_ops amc6821_cooling_ops = {
    .get_max_state = amc6821_get_max_state,
    .get_cur_state = amc6821_get_cur_state,
    .set_cur_state = amc6821_set_cur_state,
    };
// Return 0 if detection is successful, -ENODEV otherwise
#[no_mangle]
unsafe extern "C" fn amc6821_detect(client: *mut i2c_client, info: *mut i2c_board_info) -> c_int {
    static int amc6821_detect(struct i2c_client *client, struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    let mut address: c_int = client.addr;
    int dev_id, comp_id;
    dev_dbg(&adapter.dev, "amc6821_detect called.\n");
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_dbg(&adapter.dev,
    "amc6821: I2C bus doesn't support byte mode, "
    "skipping.\n");
    return -ENODEV;
    }
    dev_id = i2c_smbus_read_byte_data(client, AMC6821_REG_DEV_ID);
    comp_id = i2c_smbus_read_byte_data(client, AMC6821_REG_COMP_ID);
    if (dev_id != 0x21 || comp_id != 0x49) {
    dev_dbg(&adapter.dev,
    "amc6821: detection failed at 0x%02x.\n",
    address);
    return -ENODEV;
    }
//
// Bit 7 of the address register is ignored, so we can check the
// ID registers again
//
    dev_id = i2c_smbus_read_byte_data(client, 0x80 | AMC6821_REG_DEV_ID);
    comp_id = i2c_smbus_read_byte_data(client, 0x80 | AMC6821_REG_COMP_ID);
    if (dev_id != 0x21 || comp_id != 0x49) {
    dev_dbg(&adapter.dev,
    "amc6821: detection failed at 0x%02x.\n",
    address);
    return -ENODEV;
    }
    dev_info(&adapter.dev, "amc6821: chip found at 0x%02x.\n", address);
    strscpy(info.type, "amc6821", I2C_NAME_SIZE);
    return 0;
    }
    static enum pwm_polarity amc6821_pwm_polarity(struct i2c_client *client,
    struct device_node *fan_np)
    {
    let mut polarity: enum pwm_polarity = PWM_POLARITY_NORMAL;
    struct of_phandle_args args;
//
// For backward compatibility, the pwminv module parameter takes
// always the precedence over any other device description
//
    if (pwminv == 0)
    return PWM_POLARITY_NORMAL;
    if (pwminv > 0)
    return PWM_POLARITY_INVERSED;
    if (of_parse_phandle_with_args(fan_np, "pwms", "#pwm-cells", 0, &args))
    goto out;
    of_node_put(args.np);
    if (args.args_count != 2)
    goto out;
    if (args.args[1] & PWM_POLARITY_INVERTED)
    polarity = PWM_POLARITY_INVERSED;
    out:
    return polarity;
    }
    static int amc6821_of_fan_read_data(struct i2c_client *client,
    struct amc6821_data *data,
    struct device_node *fan_np)
    {
    int num;
    data.pwm_polarity = amc6821_pwm_polarity(client, fan_np);
    num = of_property_count_u32_elems(fan_np, "cooling-levels");
    if (num <= 0)
    return 0;
    data.fan_max_state = num - 1;
    data.fan_cooling_levels = devm_kcalloc(&client.dev, num,
    sizeof(u32),
    GFP_KERNEL);
    if (!data.fan_cooling_levels)
    return -ENOMEM;
    return of_property_read_u32_array(fan_np, "cooling-levels",
    data.fan_cooling_levels, num);
    }
#[no_mangle]
unsafe extern "C" fn amc6821_init_client(client: *mut i2c_client, data: *mut amc6821_data) -> c_int {
    static int amc6821_init_client(struct i2c_client *client, struct amc6821_data *data)
    {
    struct regmap *regmap = data.regmap;
    u32 regval;
    int err;
    if (init) {
    err = regmap_set_bits(regmap, AMC6821_REG_CONF4, AMC6821_CONF4_MODE);
    if (err)
    return err;
    err = regmap_clear_bits(regmap, AMC6821_REG_CONF3, AMC6821_CONF3_THERM_FAN_EN);
    if (err)
    return err;
    err = regmap_clear_bits(regmap, AMC6821_REG_CONF2,
    AMC6821_CONF2_RTFIE |
    AMC6821_CONF2_LTOIE |
    AMC6821_CONF2_RTOIE);
    if (err)
    return err;
    regval = AMC6821_CONF1_START;
    if (data.pwm_polarity == PWM_POLARITY_INVERSED)
    regval |= AMC6821_CONF1_PWMINV;
    err = regmap_update_bits(regmap, AMC6821_REG_CONF1,
    AMC6821_CONF1_THERMOVIE | AMC6821_CONF1_FANIE |
    AMC6821_CONF1_START | AMC6821_CONF1_PWMINV,
    regval);
    if (err)
    return err;
// Software DCY-control mode with fan enabled when cooling-levels present
    if (data.fan_cooling_levels) {
    err = amc6821_set_sw_dcy(data,
    data.fan_cooling_levels[data.fan_max_state]);
    if (err)
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amc6821_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool amc6821_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case AMC6821_REG_STAT1:
    case AMC6821_REG_STAT2:
    case AMC6821_REG_TEMP_LO:
    case AMC6821_REG_TDATA_LOW:
    case AMC6821_REG_LTEMP_HI:
    case AMC6821_REG_RTEMP_HI:
    case AMC6821_REG_TDATA_HI:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config amc6821_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .volatile_reg = amc6821_volatile_reg,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn amc6821_probe(client: *mut i2c_client) -> c_int {
    static int amc6821_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct amc6821_data *data;
    struct device *hwmon_dev;
    struct regmap *regmap;
    struct device_node *fan_np __free(device_node) = core::ptr::null_mut();
    int err;
    data = devm_kzalloc(dev, sizeof(struct amc6821_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    regmap = devm_regmap_init_i2c(client, &amc6821_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to initialize regmap\n");
    data.regmap = regmap;
    fan_np = of_get_child_by_name(dev.of_node, "fan");
    if (fan_np) {
    err = amc6821_of_fan_read_data(client, data, fan_np);
    if (err)
    return dev_err_probe(dev, err,
    "Failed to read fan device tree data\n");
    }
    err = amc6821_init_client(client, data);
    if (err)
    return err;
    if (of_device_is_compatible(dev.of_node, "tsd,mule")) {
    err = devm_of_platform_populate(dev);
    if (err)
    return dev_err_probe(dev, err,
    "Failed to create sub-devices\n");
    }
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data, &amc6821_chip_info,
    amc6821_groups);
    if (IS_ERR(hwmon_dev))
    return dev_err_probe(dev, PTR_ERR(hwmon_dev),
    "Failed to initialize hwmon\n");
    if (IS_ENABLED(CONFIG_THERMAL) && fan_np && data.fan_cooling_levels)
    return PTR_ERR_OR_ZERO(devm_thermal_of_child_cooling_device_register(dev,
    fan_np, client.name, data, &amc6821_cooling_ops));
    return 0;
    }
    static const struct i2c_device_id amc6821_id[] = {
    { .name = "amc6821" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, amc6821_id);
    static const struct of_device_id __maybe_unused amc6821_of_match[] = {
    {
    .compatible = "ti,amc6821",
    },
    {
    .compatible = "tsd,mule",
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, amc6821_of_match);
    static struct i2c_driver amc6821_driver = {
    .class = I2C_CLASS_HWMON,
    .driver = {
    .name	= "amc6821",
    .of_match_table = of_match_ptr(amc6821_of_match),
    },
    .probe = amc6821_probe,
    .id_table = amc6821_id,
    .detect = amc6821_detect,
    .address_list = normal_i2c,
    };
    module_i2c_driver(amc6821_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("T. Mertelj <tomaz.mertelj@guest.arnes.si>");
    MODULE_DESCRIPTION("Texas Instruments amc6821 hwmon driver");
