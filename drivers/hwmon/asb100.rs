//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/asb100.c
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
// asb100.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
//
// Copyright (C) 2004 Mark M. Hoffman <mhoffman@lightlink.com>
//
// (derived from w83781d.c)
//
// Copyright (C) 1998 - 2003  Frodo Looijaard <frodol@dds.nl>,
// Philip Edelbrock <phil@netroedge.com>, and
// Mark Studebaker <mdsxyz123@yahoo.com>
//
// This driver supports the hardware sensor chips: Asus ASB100 and
// ASB100-A "BACH".
//
// ASB100-A supports pwm1, while plain ASB100 does not.  There is no known
// way for the driver to tell which one is there.
//
// Chip		#vin	#fanin	#pwm	#temp	wchipid	vendid	i2c	ISA
// asb100	7	3	1	4	0x31	0x0694	yes	no
//

// I2C addresses to scan
    static const unsigned short normal_i2c[] = { 0x2d, I2C_CLIENT_END };
    static unsigned short force_subclients[4];
    module_param_array(force_subclients, short, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(force_subclients,
    "List of subclient addresses: {bus, clientaddr, subclientaddr1, subclientaddr2}");
// Voltage IN registers 0-6

// FAN IN registers 1-3

// TEMPERATURE registers 1-4
    static const u16 asb100_reg_temp[]	= {0, 0x27, 0x150, 0x250, 0x17};
    static const u16 asb100_reg_temp_max[]	= {0, 0x39, 0x155, 0x255, 0x18};
    static const u16 asb100_reg_temp_hyst[]	= {0, 0x3a, 0x153, 0x253, 0x19};

pub const ASB100_REG_TEMP2_CONFIG: c_uint = 0x0152;
pub const ASB100_REG_TEMP3_CONFIG: c_uint = 0x0252;
pub const ASB100_REG_CONFIG: c_uint = 0x40;
pub const ASB100_REG_ALARM1: c_uint = 0x41;
pub const ASB100_REG_ALARM2: c_uint = 0x42;
pub const ASB100_REG_SMIM1: c_uint = 0x43;
pub const ASB100_REG_SMIM2: c_uint = 0x44;
pub const ASB100_REG_VID_FANDIV: c_uint = 0x47;
pub const ASB100_REG_I2C_ADDR: c_uint = 0x48;
pub const ASB100_REG_CHIPID: c_uint = 0x49;
pub const ASB100_REG_I2C_SUBADDR: c_uint = 0x4a;
pub const ASB100_REG_PIN: c_uint = 0x4b;
pub const ASB100_REG_IRQ: c_uint = 0x4c;
pub const ASB100_REG_BANK: c_uint = 0x4e;
pub const ASB100_REG_CHIPMAN: c_uint = 0x4f;
pub const ASB100_REG_WCHIPID: c_uint = 0x58;
// bit 7 -> enable, bits 0-3 -> duty cycle
pub const ASB100_REG_PWM1: c_uint = 0x59;
//
// CONVERSIONS
// Rounding and limit checking is only done on the TO_REG variants.
//
// These constants are a guess, consistent w/ w83781d
pub const ASB100_IN_MIN: c_int = 0;
pub const ASB100_IN_MAX: c_int = 4080;
//
// IN: 1/1000 V (0V to 4.08V)
// REG: 16mV/bit
//
#[no_mangle]
unsafe extern "C" fn IN_TO_REG(val: unsigned) -> u8 {
    static u8 IN_TO_REG(unsigned val)
    {
    let mut nval: unsigned = clamp_val(val, ASB100_IN_MIN, ASB100_IN_MAX);
    return (nval + 8) / 16;
    }
#[no_mangle]
unsafe extern "C" fn IN_FROM_REG(reg: u8) -> unsigned {
    static unsigned IN_FROM_REG(u8 reg)
    {
    return reg * 16;
    }
#[no_mangle]
unsafe extern "C" fn FAN_TO_REG(rpm: c_long, div: c_int) -> u8 {
    static u8 FAN_TO_REG(long rpm, int div)
    {
    if (rpm == -1)
    return 0;
    if (rpm == 0)
    return 255;
    rpm = clamp_val(rpm, 1, 1000000);
    return clamp_val((1350000 + rpm * div / 2) / (rpm * div), 1, 254);
    }
#[no_mangle]
unsafe extern "C" fn FAN_FROM_REG(val: u8, div: c_int) -> c_int {
    static int FAN_FROM_REG(u8 val, int div)
    {
    let mut val: return = = 0 ? -1 : val == 255 ? 0 : 1350000 / (val * div);
    }
// These constants are a guess, consistent w/ w83781d

pub const ASB100_TEMP_MAX: c_int = 127000;
//
// TEMP: 0.001C/bit (-128C to +127C)
// REG: 1C/bit, two's complement
//
#[no_mangle]
unsafe extern "C" fn TEMP_TO_REG(temp: c_long) -> u8 {
    static u8 TEMP_TO_REG(long temp)
    {
    let mut ntemp: c_int = clamp_val(temp, ASB100_TEMP_MIN, ASB100_TEMP_MAX);
    ntemp += (ntemp < 0 ? -500 : 500);
    return (u8)(ntemp / 1000);
    }
#[no_mangle]
unsafe extern "C" fn TEMP_FROM_REG(reg: u8) -> c_int {
    static int TEMP_FROM_REG(u8 reg)
    {
    return (s8)reg * 1000;
    }
//
// PWM: 0 - 255 per sensors documentation
// REG: (6.25% duty cycle per bit)
//
#[no_mangle]
unsafe extern "C" fn ASB100_PWM_TO_REG(pwm: c_int) -> u8 {
    static u8 ASB100_PWM_TO_REG(int pwm)
    {
    pwm = clamp_val(pwm, 0, 255);
    return (u8)(pwm / 16);
    }
#[no_mangle]
unsafe extern "C" fn ASB100_PWM_FROM_REG(reg: u8) -> c_int {
    static int ASB100_PWM_FROM_REG(u8 reg)
    {
    return reg * 16;
    }

//
// FAN DIV: 1, 2, 4, or 8 (defaults to 2)
// REG: 0, 1, 2, or 3 (respectively) (defaults to 1)
//
#[no_mangle]
unsafe extern "C" fn DIV_TO_REG(val: c_long) -> u8 {
    static u8 DIV_TO_REG(long val)
    {
    let mut val: return = = 8 ? 3 : val == 4 ? 2 : val == 1 ? 0 : 1;
    }
//
// For each registered client, we need to keep some data in memory. That
// data is pointed to by client->data. The structure itself is
// dynamically allocated, at the same time the client itself is allocated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asb100_data {
    pub hwmon_dev: *mut device,
    pub lock: mutex,
    pub update_lock: mutex,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
// array of 2 pointers to subclients
    pub lm75: [*mut i2c_client; 2],
    pub /: *mut *mut bool valid; / true if following fields are valid,
    pub /: *mut *mut u8 in[7]; / Register value,
    pub /: *mut *mut u8 in_max[7]; / Register value,
    pub /: *mut *mut u8 in_min[7]; / Register value,
    pub /: *mut *mut u8 fan[3]; / Register value,
    pub /: *mut *mut u8 fan_min[3]; / Register value,
    pub /: *mut *mut u16 temp[4]; / Register value (0 and 3 are u8 only),
    pub /: *mut *mut u16 temp_max[4]; / Register value (0 and 3 are u8 only),
    pub /: *mut *mut u16 temp_hyst[4]; / Register value (0 and 3 are u8 only),
    pub /: *mut *mut u8 fan_div[3]; / Register encoding, right justified,
    pub /: *mut *mut u8 pwm; / Register encoding,
    pub /: *mut *mut u8 vid; / Register encoding, combined,
    pub /: *mut *mut u32 alarms; / Register encoding, combined,
    pub vrm: u8,
}

    static int asb100_read_value(struct i2c_client *client, u16 reg);
    static void asb100_write_value(struct i2c_client *client, u16 reg, u16 val);
    static int asb100_probe(struct i2c_client *client);
    static int asb100_detect(struct i2c_client *client,
    struct i2c_board_info *info);
    static void asb100_remove(struct i2c_client *client);
    static struct asb100_data *asb100_update_device(struct device *dev);
    static void asb100_init_client(struct i2c_client *client);
    static const struct i2c_device_id asb100_id[] = {
    { .name = "asb100" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, asb100_id);
    static struct i2c_driver asb100_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "asb100",
    },
    .probe		= asb100_probe,
    .remove		= asb100_remove,
    .id_table	= asb100_id,
    .detect		= asb100_detect,
    .address_list	= normal_i2c,
    };
// 7 Voltages

    static ssize_t show_##reg(struct device *dev, struct device_attribute *attr, \
    char *buf) \
    { \
    int nr = to_sensor_dev_attr(attr).index; \
    struct asb100_data *data = asb100_update_device(dev); \
    return sprintf(buf, "%d\n", IN_FROM_REG(data.reg[nr])); \
    }
    show_in_reg(in)
    show_in_reg(in_min)
    show_in_reg(in_max)

    static ssize_t set_in_##reg(struct device *dev, struct device_attribute *attr, \
    const char *buf, size_t count) \
    { \
    int nr = to_sensor_dev_attr(attr).index; \
    struct i2c_client *client = to_i2c_client(dev); \
    struct asb100_data *data = i2c_get_clientdata(client); \
    unsigned long val; \
    int err = kstrtoul(buf, 10, &val); \
    if (err) \
    return err; \
    mutex_lock(&data.update_lock); \
    data.in_##reg[nr] = IN_TO_REG(val); \
    asb100_write_value(client, ASB100_REG_IN_##REG(nr), \
    data.in_##reg[nr]); \
    mutex_unlock(&data.update_lock); \
    return count; \
    }
    set_in_reg(MIN, min)
    set_in_reg(MAX, max)

    static SENSOR_DEVICE_ATTR(in##offset##_input, S_IRUGO, \
    show_in, core::ptr::null_mut(), offset); \
    static SENSOR_DEVICE_ATTR(in##offset##_min, S_IRUGO | S_IWUSR, \
    show_in_min, set_in_min, offset); \
    static SENSOR_DEVICE_ATTR(in##offset##_max, S_IRUGO | S_IWUSR, \
    show_in_max, set_in_max, offset)
    sysfs_in(0);
    sysfs_in(1);
    sysfs_in(2);
    sysfs_in(3);
    sysfs_in(4);
    sysfs_in(5);
    sysfs_in(6);
// 3 Fans
    static ssize_t show_fan(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", FAN_FROM_REG(data.fan[nr],
    DIV_FROM_REG(data.fan_div[nr])));
    }
    static ssize_t show_fan_min(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", FAN_FROM_REG(data.fan_min[nr],
    DIV_FROM_REG(data.fan_div[nr])));
    }
    static ssize_t show_fan_div(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", DIV_FROM_REG(data.fan_div[nr]));
    }
    static ssize_t set_fan_min(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct i2c_client *client = to_i2c_client(dev);
    struct asb100_data *data = i2c_get_clientdata(client);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.fan_min[nr] = FAN_TO_REG(val, DIV_FROM_REG(data.fan_div[nr]));
    asb100_write_value(client, ASB100_REG_FAN_MIN(nr), data.fan_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
//
// Note: we save and restore the fan minimum here, because its value is
// determined in part by the fan divisor.  This follows the principle of
// least surprise; the user doesn't expect the fan minimum to change just
// because the divisor changed.
//
    static ssize_t set_fan_div(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct i2c_client *client = to_i2c_client(dev);
    struct asb100_data *data = i2c_get_clientdata(client);
    unsigned long min;
    int reg;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    min = FAN_FROM_REG(data.fan_min[nr],
    DIV_FROM_REG(data.fan_div[nr]));
    data.fan_div[nr] = DIV_TO_REG(val);
    switch (nr) {
    case 0:	/* fan 1 */
    reg = asb100_read_value(client, ASB100_REG_VID_FANDIV);
    reg = (reg & 0xcf) | (data.fan_div[0] << 4);
    asb100_write_value(client, ASB100_REG_VID_FANDIV, reg);
    break;
    case 1:	/* fan 2 */
    reg = asb100_read_value(client, ASB100_REG_VID_FANDIV);
    reg = (reg & 0x3f) | (data.fan_div[1] << 6);
    asb100_write_value(client, ASB100_REG_VID_FANDIV, reg);
    break;
    case 2:	/* fan 3 */
    reg = asb100_read_value(client, ASB100_REG_PIN);
    reg = (reg & 0x3f) | (data.fan_div[2] << 6);
    asb100_write_value(client, ASB100_REG_PIN, reg);
    break;
    }
    data.fan_min[nr] =
    FAN_TO_REG(min, DIV_FROM_REG(data.fan_div[nr]));
    asb100_write_value(client, ASB100_REG_FAN_MIN(nr), data.fan_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }

    static SENSOR_DEVICE_ATTR(fan##offset##_input, S_IRUGO, \
    show_fan, core::ptr::null_mut(), offset - 1); \
    static SENSOR_DEVICE_ATTR(fan##offset##_min, S_IRUGO | S_IWUSR, \
    show_fan_min, set_fan_min, offset - 1); \
    static SENSOR_DEVICE_ATTR(fan##offset##_div, S_IRUGO | S_IWUSR, \
    show_fan_div, set_fan_div, offset - 1)
    sysfs_fan(1);
    sysfs_fan(2);
    sysfs_fan(3);
// 4 Temp. Sensors
#[no_mangle]
unsafe extern "C" fn sprintf_temp_from_reg(reg: u16, buf: *mut c_char, nr: c_int) -> c_int {
    static int sprintf_temp_from_reg(u16 reg, char *buf, int nr)
    {
    let mut ret: c_int = 0;
    switch (nr) {
    case 1: case 2:
    ret = sprintf(buf, "%d\n", LM75_TEMP_FROM_REG(reg));
    break;
    case 0: case 3: default:
    ret = sprintf(buf, "%d\n", TEMP_FROM_REG(reg));
    break;
    }
    return ret;
    }

    static ssize_t show_##reg(struct device *dev, struct device_attribute *attr, \
    char *buf) \
    { \
    int nr = to_sensor_dev_attr(attr).index; \
    struct asb100_data *data = asb100_update_device(dev); \
    return sprintf_temp_from_reg(data.reg[nr], buf, nr); \
    }
    show_temp_reg(temp);
    show_temp_reg(temp_max);
    show_temp_reg(temp_hyst);

    static ssize_t set_##reg(struct device *dev, struct device_attribute *attr, \
    const char *buf, size_t count) \
    { \
    int nr = to_sensor_dev_attr(attr).index; \
    struct i2c_client *client = to_i2c_client(dev); \
    struct asb100_data *data = i2c_get_clientdata(client); \
    long val; \
    int err = kstrtol(buf, 10, &val); \
    if (err) \
    return err; \
    mutex_lock(&data.update_lock); \
    switch (nr) { \
    case 1: case 2: \
    data.reg[nr] = LM75_TEMP_TO_REG(val); \
    break; \
    case 0: case 3: default: \
    data.reg[nr] = TEMP_TO_REG(val); \
    break; \
    } \
    asb100_write_value(client, ASB100_REG_TEMP_##REG(nr+1), \
    data.reg[nr]); \
    mutex_unlock(&data.update_lock); \
    return count; \
    }
    set_temp_reg(MAX, temp_max);
    set_temp_reg(HYST, temp_hyst);

    static SENSOR_DEVICE_ATTR(temp##num##_input, S_IRUGO, \
    show_temp, core::ptr::null_mut(), num - 1); \
    static SENSOR_DEVICE_ATTR(temp##num##_max, S_IRUGO | S_IWUSR, \
    show_temp_max, set_temp_max, num - 1); \
    static SENSOR_DEVICE_ATTR(temp##num##_max_hyst, S_IRUGO | S_IWUSR, \
    show_temp_hyst, set_temp_hyst, num - 1)
    sysfs_temp(1);
    sysfs_temp(2);
    sysfs_temp(3);
    sysfs_temp(4);
// VID
    static ssize_t cpu0_vid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", vid_from_reg(data.vid, data.vrm));
    }
    static DEVICE_ATTR_RO(cpu0_vid);
// VRM
    static ssize_t vrm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct asb100_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", data.vrm);
    }
    static ssize_t vrm_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct asb100_data *data = dev_get_drvdata(dev);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    if (val > 255)
    return -EINVAL;
    data.vrm = val;
    return count;
    }
// Alarms
    static DEVICE_ATTR_RW(vrm);
    static ssize_t alarms_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%u\n", data.alarms);
    }
    static DEVICE_ATTR_RO(alarms);
    static ssize_t show_alarm(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut bitnr: c_int = to_sensor_dev_attr(attr).index;
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%u\n", (data.alarms >> bitnr) & 1);
    }
    static SENSOR_DEVICE_ATTR(in0_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(in1_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(in2_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(in3_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(in4_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 8);
    static SENSOR_DEVICE_ATTR(fan1_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 6);
    static SENSOR_DEVICE_ATTR(fan2_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 7);
    static SENSOR_DEVICE_ATTR(fan3_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 11);
    static SENSOR_DEVICE_ATTR(temp1_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(temp2_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 5);
    static SENSOR_DEVICE_ATTR(temp3_alarm, S_IRUGO, show_alarm, core::ptr::null_mut(), 13);
// 1 PWM
    static ssize_t pwm1_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", ASB100_PWM_FROM_REG(data.pwm & 0x0f));
    }
    static ssize_t pwm1_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct asb100_data *data = i2c_get_clientdata(client);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.pwm &= 0x80; /* keep the enable bit */
    data.pwm |= (0x0f & ASB100_PWM_TO_REG(val));
    asb100_write_value(client, ASB100_REG_PWM1, data.pwm);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t pwm1_enable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct asb100_data *data = asb100_update_device(dev);
    return sprintf(buf, "%d\n", (data.pwm & 0x80) ? 1 : 0);
    }
    static ssize_t pwm1_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct asb100_data *data = i2c_get_clientdata(client);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.pwm &= 0x0f; /* keep the duty cycle bits */
    data.pwm |= (val ? 0x80 : 0x00);
    asb100_write_value(client, ASB100_REG_PWM1, data.pwm);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static DEVICE_ATTR_RW(pwm1);
    static DEVICE_ATTR_RW(pwm1_enable);
    static struct attribute *asb100_attributes[] = {
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in0_min.dev_attr.attr,
    &sensor_dev_attr_in0_max.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_min.dev_attr.attr,
    &sensor_dev_attr_in1_max.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_min.dev_attr.attr,
    &sensor_dev_attr_in2_max.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in3_min.dev_attr.attr,
    &sensor_dev_attr_in3_max.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in4_min.dev_attr.attr,
    &sensor_dev_attr_in4_max.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in5_min.dev_attr.attr,
    &sensor_dev_attr_in5_max.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in6_min.dev_attr.attr,
    &sensor_dev_attr_in6_max.dev_attr.attr,
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan1_div.dev_attr.attr,
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan2_min.dev_attr.attr,
    &sensor_dev_attr_fan2_div.dev_attr.attr,
    &sensor_dev_attr_fan3_input.dev_attr.attr,
    &sensor_dev_attr_fan3_min.dev_attr.attr,
    &sensor_dev_attr_fan3_div.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_max_hyst.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp2_max_hyst.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp3_max.dev_attr.attr,
    &sensor_dev_attr_temp3_max_hyst.dev_attr.attr,
    &sensor_dev_attr_temp4_input.dev_attr.attr,
    &sensor_dev_attr_temp4_max.dev_attr.attr,
    &sensor_dev_attr_temp4_max_hyst.dev_attr.attr,
    &sensor_dev_attr_in0_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_alarm.dev_attr.attr,
    &sensor_dev_attr_in4_alarm.dev_attr.attr,
    &sensor_dev_attr_fan1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan2_alarm.dev_attr.attr,
    &sensor_dev_attr_fan3_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_alarm.dev_attr.attr,
    &dev_attr_cpu0_vid.attr,
    &dev_attr_vrm.attr,
    &dev_attr_alarms.attr,
    &dev_attr_pwm1.attr,
    &dev_attr_pwm1_enable.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group asb100_group = {
    .attrs = asb100_attributes,
    };
#[no_mangle]
unsafe extern "C" fn asb100_detect_subclients(client: *mut i2c_client) -> c_int {
    static int asb100_detect_subclients(struct i2c_client *client)
    {
    int i, id, err;
    let mut address: c_int = client.addr;
    unsigned short sc_addr[2];
    struct asb100_data *data = i2c_get_clientdata(client);
    struct i2c_adapter *adapter = client.adapter;
    id = i2c_adapter_id(adapter);
    if (force_subclients[0] == id && force_subclients[1] == address) {
    for (i = 2; i <= 3; i++) {
    if (force_subclients[i] < 0x48 ||
    force_subclients[i] > 0x4f) {
    dev_err(&client.dev,
    "invalid subclient address %d; must be 0x48-0x4f\n",
    force_subclients[i]);
    err = -ENODEV;
    goto ERROR_SC_2;
    }
    }
    asb100_write_value(client, ASB100_REG_I2C_SUBADDR,
    (force_subclients[2] & 0x07) |
    ((force_subclients[3] & 0x07) << 4));
    sc_addr[0] = force_subclients[2];
    sc_addr[1] = force_subclients[3];
    } else {
    let mut val: c_int = asb100_read_value(client, ASB100_REG_I2C_SUBADDR);
    sc_addr[0] = 0x48 + (val & 0x07);
    sc_addr[1] = 0x48 + ((val >> 4) & 0x07);
    }
    if (sc_addr[0] == sc_addr[1]) {
    dev_err(&client.dev,
    "duplicate addresses 0x%x for subclients\n",
    sc_addr[0]);
    err = -ENODEV;
    goto ERROR_SC_2;
    }
    data.lm75[0] = i2c_new_dummy_device(adapter, sc_addr[0]);
    if (IS_ERR(data.lm75[0])) {
    dev_err(&client.dev,
    "subclient %d registration at address 0x%x failed.\n",
    1, sc_addr[0]);
    err = PTR_ERR(data.lm75[0]);
    goto ERROR_SC_2;
    }
    data.lm75[1] = i2c_new_dummy_device(adapter, sc_addr[1]);
    if (IS_ERR(data.lm75[1])) {
    dev_err(&client.dev,
    "subclient %d registration at address 0x%x failed.\n",
    2, sc_addr[1]);
    err = PTR_ERR(data.lm75[1]);
    goto ERROR_SC_3;
    }
    return 0;
// Undo inits in case of errors
    ERROR_SC_3:
    i2c_unregister_device(data.lm75[0]);
    ERROR_SC_2:
    return err;
    }
// Return 0 if detection is successful, -ENODEV otherwise
    static int asb100_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    int val1, val2;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    pr_debug("detect failed, smbus byte data not supported!\n");
    return -ENODEV;
    }
    val1 = i2c_smbus_read_byte_data(client, ASB100_REG_BANK);
    val2 = i2c_smbus_read_byte_data(client, ASB100_REG_CHIPMAN);
// If we're in bank 0
    if ((!(val1 & 0x07)) &&
// Check for ASB100 ID (low byte)
    (((!(val1 & 0x80)) && (val2 != 0x94)) ||
// Check for ASB100 ID (high byte )
    ((val1 & 0x80) && (val2 != 0x06)))) {
    pr_debug("detect failed, bad chip id 0x%02x!\n", val2);
    return -ENODEV;
    }
// Put it now into bank 0 and Vendor ID High Byte
    i2c_smbus_write_byte_data(client, ASB100_REG_BANK,
    (i2c_smbus_read_byte_data(client, ASB100_REG_BANK) & 0x78)
    | 0x80);
// Determine the chip type.
    val1 = i2c_smbus_read_byte_data(client, ASB100_REG_WCHIPID);
    val2 = i2c_smbus_read_byte_data(client, ASB100_REG_CHIPMAN);
    if (val1 != 0x31 || val2 != 0x06)
    return -ENODEV;
    strscpy(info.type, "asb100", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asb100_probe(client: *mut i2c_client) -> c_int {
    static int asb100_probe(struct i2c_client *client)
    {
    int err;
    struct asb100_data *data;
    data = devm_kzalloc(&client.dev, sizeof(struct asb100_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    mutex_init(&data.lock);
    mutex_init(&data.update_lock);
// Attach secondary lm75 clients
    err = asb100_detect_subclients(client);
    if (err)
    return err;
// Initialize the chip
    asb100_init_client(client);
// A few vars need to be filled upon startup
    data.fan_min[0] = asb100_read_value(client, ASB100_REG_FAN_MIN(0));
    data.fan_min[1] = asb100_read_value(client, ASB100_REG_FAN_MIN(1));
    data.fan_min[2] = asb100_read_value(client, ASB100_REG_FAN_MIN(2));
// Register sysfs hooks
    err = sysfs_create_group(&client.dev.kobj, &asb100_group);
    if (err)
    goto ERROR3;
    data.hwmon_dev = hwmon_device_register(&client.dev);
    if (IS_ERR(data.hwmon_dev)) {
    err = PTR_ERR(data.hwmon_dev);
    goto ERROR4;
    }
    return 0;
    ERROR4:
    sysfs_remove_group(&client.dev.kobj, &asb100_group);
    ERROR3:
    i2c_unregister_device(data.lm75[1]);
    i2c_unregister_device(data.lm75[0]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn asb100_remove(client: *mut i2c_client) {
    static void asb100_remove(struct i2c_client *client)
    {
    struct asb100_data *data = i2c_get_clientdata(client);
    hwmon_device_unregister(data.hwmon_dev);
    sysfs_remove_group(&client.dev.kobj, &asb100_group);
    i2c_unregister_device(data.lm75[1]);
    i2c_unregister_device(data.lm75[0]);
    }
//
// The SMBus locks itself, usually, but nothing may access the chip between
// bank switches.
//
#[no_mangle]
unsafe extern "C" fn asb100_read_value(client: *mut i2c_client, reg: u16) -> c_int {
    static int asb100_read_value(struct i2c_client *client, u16 reg)
    {
    struct asb100_data *data = i2c_get_clientdata(client);
    struct i2c_client *cl;
    int res, bank;
    mutex_lock(&data.lock);
    bank = (reg >> 8) & 0x0f;
    if (bank > 2)
// switch banks
    i2c_smbus_write_byte_data(client, ASB100_REG_BANK, bank);
    if (bank == 0 || bank > 2) {
    res = i2c_smbus_read_byte_data(client, reg & 0xff);
    } else {
// switch to subclient
    cl = data.lm75[bank - 1];
// convert from ISA to LM75 I2C addresses
    switch (reg & 0xff) {
    case 0x50: /* TEMP */
    res = i2c_smbus_read_word_swapped(cl, 0);
    break;
    case 0x52: /* CONFIG */
    res = i2c_smbus_read_byte_data(cl, 1);
    break;
    case 0x53: /* HYST */
    res = i2c_smbus_read_word_swapped(cl, 2);
    break;
    case 0x55: /* MAX */
    default:
    res = i2c_smbus_read_word_swapped(cl, 3);
    break;
    }
    }
    if (bank > 2)
    i2c_smbus_write_byte_data(client, ASB100_REG_BANK, 0);
    mutex_unlock(&data.lock);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn asb100_write_value(client: *mut i2c_client, reg: u16, value: u16) {
    static void asb100_write_value(struct i2c_client *client, u16 reg, u16 value)
    {
    struct asb100_data *data = i2c_get_clientdata(client);
    struct i2c_client *cl;
    int bank;
    mutex_lock(&data.lock);
    bank = (reg >> 8) & 0x0f;
    if (bank > 2)
// switch banks
    i2c_smbus_write_byte_data(client, ASB100_REG_BANK, bank);
    if (bank == 0 || bank > 2) {
    i2c_smbus_write_byte_data(client, reg & 0xff, value & 0xff);
    } else {
// switch to subclient
    cl = data.lm75[bank - 1];
// convert from ISA to LM75 I2C addresses
    switch (reg & 0xff) {
    case 0x52: /* CONFIG */
    i2c_smbus_write_byte_data(cl, 1, value & 0xff);
    break;
    case 0x53: /* HYST */
    i2c_smbus_write_word_swapped(cl, 2, value);
    break;
    case 0x55: /* MAX */
    i2c_smbus_write_word_swapped(cl, 3, value);
    break;
    }
    }
    if (bank > 2)
    i2c_smbus_write_byte_data(client, ASB100_REG_BANK, 0);
    mutex_unlock(&data.lock);
    }
#[no_mangle]
unsafe extern "C" fn asb100_init_client(client: *mut i2c_client) {
    static void asb100_init_client(struct i2c_client *client)
    {
    struct asb100_data *data = i2c_get_clientdata(client);
    data.vrm = vid_which_vrm();
// Start monitoring
    asb100_write_value(client, ASB100_REG_CONFIG,
    (asb100_read_value(client, ASB100_REG_CONFIG) & 0xf7) | 0x01);
    }
    static struct asb100_data *asb100_update_device(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct asb100_data *data = i2c_get_clientdata(client);
    int i;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ + HZ / 2)
    || !data.valid) {
    dev_dbg(&client.dev, "starting device update...\n");
// 7 voltage inputs
    for (i = 0; i < 7; i++) {
    data.in[i] = asb100_read_value(client,
    ASB100_REG_IN(i));
    data.in_min[i] = asb100_read_value(client,
    ASB100_REG_IN_MIN(i));
    data.in_max[i] = asb100_read_value(client,
    ASB100_REG_IN_MAX(i));
    }
// 3 fan inputs
    for (i = 0; i < 3; i++) {
    data.fan[i] = asb100_read_value(client,
    ASB100_REG_FAN(i));
    data.fan_min[i] = asb100_read_value(client,
    ASB100_REG_FAN_MIN(i));
    }
// 4 temperature inputs
    for (i = 1; i <= 4; i++) {
    data.temp[i-1] = asb100_read_value(client,
    ASB100_REG_TEMP(i));
    data.temp_max[i-1] = asb100_read_value(client,
    ASB100_REG_TEMP_MAX(i));
    data.temp_hyst[i-1] = asb100_read_value(client,
    ASB100_REG_TEMP_HYST(i));
    }
// VID and fan divisors
    i = asb100_read_value(client, ASB100_REG_VID_FANDIV);
    data.vid = i & 0x0f;
    data.vid |= (asb100_read_value(client,
    ASB100_REG_CHIPID) & 0x01) << 4;
    data.fan_div[0] = (i >> 4) & 0x03;
    data.fan_div[1] = (i >> 6) & 0x03;
    data.fan_div[2] = (asb100_read_value(client,
    ASB100_REG_PIN) >> 6) & 0x03;
// PWM
    data.pwm = asb100_read_value(client, ASB100_REG_PWM1);
// alarms
    data.alarms = asb100_read_value(client, ASB100_REG_ALARM1) +
    (asb100_read_value(client, ASB100_REG_ALARM2) << 8);
    data.last_updated = jiffies;
    data.valid = true;
    dev_dbg(&client.dev, "... device update complete\n");
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
    module_i2c_driver(asb100_driver);
    MODULE_AUTHOR("Mark M. Hoffman <mhoffman@lightlink.com>");
    MODULE_DESCRIPTION("ASB100 Bach driver");
    MODULE_LICENSE("GPL");
