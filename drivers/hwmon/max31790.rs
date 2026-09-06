//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max31790.c
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
// max31790.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring.
//
// (C) 2015 by Il Han <corone.il.han@gmail.com>
//

// MAX31790 registers
pub const MAX31790_REG_GLOBAL_CONFIG: c_uint = 0x00;

pub const MAX31790_REG_FAN_FAULT_STATUS2: c_uint = 0x10;
pub const MAX31790_REG_FAN_FAULT_STATUS1: c_uint = 0x11;

// Fan Config register bits
pub const MAX31790_FAN_CFG_RPM_MODE: c_uint = 0x80;
pub const MAX31790_FAN_CFG_CTRL_MON: c_uint = 0x10;
pub const MAX31790_FAN_CFG_TACH_INPUT_EN: c_uint = 0x08;
pub const MAX31790_FAN_CFG_TACH_INPUT: c_uint = 0x01;
// Fan Dynamics register bits
pub const MAX31790_FAN_DYN_SR_SHIFT: c_int = 5;
pub const MAX31790_FAN_DYN_SR_MASK: c_uint = 0xE0;

    >> MAX31790_FAN_DYN_SR_SHIFT)
pub const FAN_RPM_MIN: c_int = 120;
pub const FAN_RPM_MAX: c_int = 7864320;
pub const FAN_COUNT_REG_MAX: c_uint = 0xffe0;

    ((60 * (sr) * 8192) / ((reg) >> 4)) : \
    FAN_RPM_MAX)

pub const NR_CHANNEL: c_int = 6;
pub const PWM_INPUT_SCALE: c_int = 255;
pub const MAX31790_REG_PWMOUT_SCALE: c_int = 511;
//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max31790_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut bool valid; / zero until following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
// register values
    pub fan_config: [u8; NR_CHANNEL],
    pub fan_dynamics: [u8; NR_CHANNEL],
    pub fault_status: u16,
    pub 2]: *mut *mut u16 tach[NR_CHANNEL,
    pub pwm: [u16; NR_CHANNEL],
    pub target_count: [u16; NR_CHANNEL],
}

    static struct max31790_data *max31790_update_device(struct device *dev)
    {
    struct max31790_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int i, rv;
    if (time_after(jiffies, data.last_updated + HZ) || !data.valid) {
    data.valid = false;
    rv = i2c_smbus_read_byte_data(client,
    MAX31790_REG_FAN_FAULT_STATUS1);
    if (rv < 0)
    return ERR_PTR(rv);
    data.fault_status |= rv & 0x3F;
    rv = i2c_smbus_read_byte_data(client,
    MAX31790_REG_FAN_FAULT_STATUS2);
    if (rv < 0)
    return ERR_PTR(rv);
    data.fault_status |= (rv & 0x3F) << 6;
    for (i = 0; i < NR_CHANNEL; i++) {
    rv = i2c_smbus_read_word_swapped(client,
    MAX31790_REG_TACH_COUNT(i));
    if (rv < 0)
    return ERR_PTR(rv);
    data.tach[i] = rv;
    if (data.fan_config[i]
    & MAX31790_FAN_CFG_TACH_INPUT) {
    rv = i2c_smbus_read_word_swapped(client,
    MAX31790_REG_TACH_COUNT(NR_CHANNEL
    + i));
    if (rv < 0)
    return ERR_PTR(rv);
    data.tach[NR_CHANNEL + i] = rv;
    } else {
    rv = i2c_smbus_read_word_swapped(client,
    MAX31790_REG_PWM_DUTY_CYCLE(i));
    if (rv < 0)
    return ERR_PTR(rv);
    data.pwm[i] = rv;
    rv = i2c_smbus_read_word_swapped(client,
    MAX31790_REG_TARGET_COUNT(i));
    if (rv < 0)
    return ERR_PTR(rv);
    data.target_count[i] = rv;
    }
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    return data;
    }
    static const u8 tach_period[8] = { 1, 2, 4, 8, 16, 32, 32, 32 };
#[no_mangle]
unsafe extern "C" fn get_tach_period(fan_dynamics: u8) -> u8 {
    static u8 get_tach_period(u8 fan_dynamics)
    {
    return tach_period[SR_FROM_REG(fan_dynamics)];
    }
#[no_mangle]
unsafe extern "C" fn bits_for_tach_period(rpm: c_int) -> u8 {
    static u8 bits_for_tach_period(int rpm)
    {
    u8 bits;
    if (rpm < 500)
    bits = 0x0;
#[no_mangle]
pub unsafe extern "C" fn if(1000: rpm <) -> else {
    else if (rpm < 1000)
    bits = 0x1;
#[no_mangle]
pub unsafe extern "C" fn if(2000: rpm <) -> else {
    else if (rpm < 2000)
    bits = 0x2;
#[no_mangle]
pub unsafe extern "C" fn if(4000: rpm <) -> else {
    else if (rpm < 4000)
    bits = 0x3;
#[no_mangle]
pub unsafe extern "C" fn if(8000: rpm <) -> else {
    else if (rpm < 8000)
    bits = 0x4;
    else
    bits = 0x5;
    return bits;
    }
    static int max31790_read_fan(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct max31790_data *data = max31790_update_device(dev);
    int sr, rpm;
    if (IS_ERR(data))
    return PTR_ERR(data);
    switch (attr) {
    case hwmon_fan_input:
    sr = get_tach_period(data.fan_dynamics[channel % NR_CHANNEL]);
    if (data.tach[channel] == FAN_COUNT_REG_MAX)
    rpm = 0;
    else
    rpm = RPM_FROM_REG(data.tach[channel], sr);
// val = rpm;
    return 0;
    case hwmon_fan_target:
    sr = get_tach_period(data.fan_dynamics[channel]);
    rpm = RPM_FROM_REG(data.target_count[channel], sr);
// val = rpm;
    return 0;
    case hwmon_fan_fault:
// val = !!(data->fault_status & (1 << channel));
    data.fault_status &= ~(1 << channel);
//
// If a fault bit is set, we need to write into one of the fan
// configuration registers to clear it. Note that this also
// clears the fault for the companion channel if enabled.
//
    if (*val) {
    let mut reg: c_int = MAX31790_REG_TARGET_COUNT(channel % NR_CHANNEL);
    return i2c_smbus_write_byte_data(data.client, reg,
    data.target_count[channel % NR_CHANNEL] >> 8);
    }
    return 0;
    case hwmon_fan_enable:
// val = !!(data->fan_config[channel] & MAX31790_FAN_CFG_TACH_INPUT_EN);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int max31790_write_fan(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct max31790_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int target_count;
    let mut err: c_int = 0;
    u8 bits, fan_config;
    int sr;
    switch (attr) {
    case hwmon_fan_target:
    val = clamp_val(val, FAN_RPM_MIN, FAN_RPM_MAX);
    bits = bits_for_tach_period(val);
    data.fan_dynamics[channel] =
    ((data.fan_dynamics[channel] &
    ~MAX31790_FAN_DYN_SR_MASK) |
    (bits << MAX31790_FAN_DYN_SR_SHIFT));
    err = i2c_smbus_write_byte_data(client,
    MAX31790_REG_FAN_DYNAMICS(channel),
    data.fan_dynamics[channel]);
    if (err < 0)
    break;
    sr = get_tach_period(data.fan_dynamics[channel]);
    target_count = RPM_TO_REG(val, sr);
    target_count = clamp_val(target_count, 0x1, 0x7FF);
    data.target_count[channel] = target_count << 5;
    err = i2c_smbus_write_word_swapped(client,
    MAX31790_REG_TARGET_COUNT(channel),
    data.target_count[channel]);
    break;
    case hwmon_fan_enable:
    fan_config = data.fan_config[channel];
    if (val == 0) {
    fan_config &= ~MAX31790_FAN_CFG_TACH_INPUT_EN;
    } else if (val == 1) {
    fan_config |= MAX31790_FAN_CFG_TACH_INPUT_EN;
    } else {
    err = -EINVAL;
    break;
    }
    if (fan_config != data.fan_config[channel]) {
    err = i2c_smbus_write_byte_data(client, MAX31790_REG_FAN_CONFIG(channel),
    fan_config);
    if (!err)
    data.fan_config[channel] = fan_config;
    }
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max31790_fan_is_visible(_data: *const c_void, attr: u32, channel: c_int) -> umode_t {
    static umode_t max31790_fan_is_visible(const void *_data, u32 attr, int channel)
    {
    const struct max31790_data *data = _data;
    let mut fan_config: u8 = data.fan_config[channel % NR_CHANNEL];
    switch (attr) {
    case hwmon_fan_input:
    case hwmon_fan_fault:
    if (channel < NR_CHANNEL ||
    (fan_config & MAX31790_FAN_CFG_TACH_INPUT))
    return 0444;
    return 0;
    case hwmon_fan_target:
    if (channel < NR_CHANNEL &&
    !(fan_config & MAX31790_FAN_CFG_TACH_INPUT))
    return 0644;
    return 0;
    case hwmon_fan_enable:
    if (channel < NR_CHANNEL)
    return 0644;
    return 0;
    default:
    return 0;
    }
    }
    static int max31790_read_pwm(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct max31790_data *data = max31790_update_device(dev);
    u8 fan_config;
    if (IS_ERR(data))
    return PTR_ERR(data);
    fan_config = data.fan_config[channel];
    switch (attr) {
    case hwmon_pwm_input:
// val = data->pwm[channel] >> 8;
    return 0;
    case hwmon_pwm_enable:
    if (fan_config & MAX31790_FAN_CFG_CTRL_MON)
// val = 0;
#[no_mangle]
pub unsafe extern "C" fn if(MAX31790_FAN_CFG_RPM_MODE: fan_config &) -> else {
    else if (fan_config & MAX31790_FAN_CFG_RPM_MODE)
// val = 2;
    else
// val = 1;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int max31790_write_pwm(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct max31790_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    u8 fan_config;
    let mut err: c_int = 0;
    switch (attr) {
    case hwmon_pwm_input:
    if (val < 0 || val > 255) {
    err = -EINVAL;
    break;
    }
    val = DIV_ROUND_CLOSEST(val * MAX31790_REG_PWMOUT_SCALE,
    PWM_INPUT_SCALE);
    data.valid = false;
    err = i2c_smbus_write_word_swapped(client,
    MAX31790_REG_PWMOUT(channel),
    val << 7);
    break;
    case hwmon_pwm_enable:
    fan_config = data.fan_config[channel];
    if (val == 0) {
    fan_config |= MAX31790_FAN_CFG_CTRL_MON;
//
// Disable RPM mode; otherwise disabling fan speed
// monitoring is not possible.
//
    fan_config &= ~MAX31790_FAN_CFG_RPM_MODE;
    } else if (val == 1) {
    fan_config &= ~(MAX31790_FAN_CFG_CTRL_MON | MAX31790_FAN_CFG_RPM_MODE);
    } else if (val == 2) {
    fan_config &= ~MAX31790_FAN_CFG_CTRL_MON;
//
// The chip sets MAX31790_FAN_CFG_TACH_INPUT_EN on its
// own if MAX31790_FAN_CFG_RPM_MODE is set.
// Do it here as well to reflect the actual register
// value in the cache.
//
    fan_config |= (MAX31790_FAN_CFG_RPM_MODE | MAX31790_FAN_CFG_TACH_INPUT_EN);
    } else {
    err = -EINVAL;
    break;
    }
    if (fan_config != data.fan_config[channel]) {
    err = i2c_smbus_write_byte_data(client, MAX31790_REG_FAN_CONFIG(channel),
    fan_config);
    if (!err)
    data.fan_config[channel] = fan_config;
    }
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max31790_pwm_is_visible(_data: *const c_void, attr: u32, channel: c_int) -> umode_t {
    static umode_t max31790_pwm_is_visible(const void *_data, u32 attr, int channel)
    {
    const struct max31790_data *data = _data;
    let mut fan_config: u8 = data.fan_config[channel];
    switch (attr) {
    case hwmon_pwm_input:
    case hwmon_pwm_enable:
    if (!(fan_config & MAX31790_FAN_CFG_TACH_INPUT))
    return 0644;
    return 0;
    default:
    return 0;
    }
    }
    static int max31790_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_fan:
    return max31790_read_fan(dev, attr, channel, val);
    case hwmon_pwm:
    return max31790_read_pwm(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int max31790_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_fan:
    return max31790_write_fan(dev, attr, channel, val);
    case hwmon_pwm:
    return max31790_write_pwm(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t max31790_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_fan:
    return max31790_fan_is_visible(data, attr, channel);
    case hwmon_pwm:
    return max31790_pwm_is_visible(data, attr, channel);
    default:
    return 0;
    }
    }
    static const struct hwmon_channel_info * const max31790_info[] = {
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_TARGET | HWMON_F_FAULT | HWMON_F_ENABLE,
    HWMON_F_INPUT | HWMON_F_FAULT,
    HWMON_F_INPUT | HWMON_F_FAULT,
    HWMON_F_INPUT | HWMON_F_FAULT,
    HWMON_F_INPUT | HWMON_F_FAULT,
    HWMON_F_INPUT | HWMON_F_FAULT,
    HWMON_F_INPUT | HWMON_F_FAULT),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE,
    HWMON_PWM_INPUT | HWMON_PWM_ENABLE),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops max31790_hwmon_ops = {
    .is_visible = max31790_is_visible,
    .read = max31790_read,
    .write = max31790_write,
    };
    static const struct hwmon_chip_info max31790_chip_info = {
    .ops = &max31790_hwmon_ops,
    .info = max31790_info,
    };
    static int max31790_init_client(struct i2c_client *client,
    struct max31790_data *data)
    {
    int i, rv;
    for (i = 0; i < NR_CHANNEL; i++) {
    rv = i2c_smbus_read_byte_data(client,
    MAX31790_REG_FAN_CONFIG(i));
    if (rv < 0)
    return rv;
    data.fan_config[i] = rv;
    rv = i2c_smbus_read_byte_data(client,
    MAX31790_REG_FAN_DYNAMICS(i));
    if (rv < 0)
    return rv;
    data.fan_dynamics[i] = rv;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31790_probe(client: *mut i2c_client) -> c_int {
    static int max31790_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct max31790_data *data;
    struct device *hwmon_dev;
    int err;
    if (!i2c_check_functionality(adapter,
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(struct max31790_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
//
// Initialize the max31790 chip
//
    err = max31790_init_client(client, data);
    if (err)
    return err;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data,
    &max31790_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id max31790_id[] = {
    { .name = "max31790" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max31790_id);
    static struct i2c_driver max31790_driver = {
    .probe		= max31790_probe,
    .driver = {
    .name	= "max31790",
    },
    .id_table	= max31790_id,
    };
    module_i2c_driver(max31790_driver);
    MODULE_AUTHOR("Il Han <corone.il.han@gmail.com>");
    MODULE_DESCRIPTION("MAX31790 sensor driver");
    MODULE_LICENSE("GPL");
