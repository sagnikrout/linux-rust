//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lm78.c
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
// lm78.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
// Copyright (c) 1998, 1999  Frodo Looijaard <frodol@dds.nl>
// Copyright (c) 2007, 2011  Jean Delvare <jdelvare@suse.de>
//

// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
    0x2e, 0x2f, I2C_CLIENT_END };
    enum chips { lm78, lm79 };
// Many LM78 constants specified below
// Length of ISA address segment
pub const LM78_EXTENT: c_int = 8;
// Where are the ISA address/data registers relative to the base address
pub const LM78_ADDR_REG_OFFSET: c_int = 5;
pub const LM78_DATA_REG_OFFSET: c_int = 6;
// The LM78 registers

pub const LM78_REG_TEMP: c_uint = 0x27;
pub const LM78_REG_TEMP_OVER: c_uint = 0x39;
pub const LM78_REG_TEMP_HYST: c_uint = 0x3a;
pub const LM78_REG_ALARM1: c_uint = 0x41;
pub const LM78_REG_ALARM2: c_uint = 0x42;
pub const LM78_REG_VID_FANDIV: c_uint = 0x47;
pub const LM78_REG_CONFIG: c_uint = 0x40;
pub const LM78_REG_CHIPID: c_uint = 0x49;
pub const LM78_REG_I2C_ADDR: c_uint = 0x48;
//
// Conversions. Rounding and limit checking is only done on the TO_REG
// variants.
//
// IN: mV (0V to 4.08V)
// REG: 16mV/bit
//
#[no_mangle]
pub unsafe extern "C" fn IN_TO_REG(val: c_ulong) -> u8 {
    static inline u8 IN_TO_REG(unsigned long val)
    {
    let mut nval: c_ulong = clamp_val(val, 0, 4080);
    return (nval + 8) / 16;
    }

#[no_mangle]
pub unsafe extern "C" fn FAN_TO_REG(rpm: c_long, div: c_int) -> u8 {
    static inline u8 FAN_TO_REG(long rpm, int div)
    {
    if (rpm <= 0)
    return 255;
    if (rpm > 1350000)
    return 1;
    return clamp_val((1350000 + rpm * div / 2) / (rpm * div), 1, 254);
    }
#[no_mangle]
pub unsafe extern "C" fn FAN_FROM_REG(val: u8, div: c_int) -> c_int {
    static inline int FAN_FROM_REG(u8 val, int div)
    {
    let mut val: return = = 0 ? -1 : val == 255 ? 0 : 1350000 / (val * div);
    }
//
// TEMP: mC (-128C to +127C)
// REG: 1C/bit, two's complement
//
#[no_mangle]
pub unsafe extern "C" fn TEMP_TO_REG(val: c_long) -> i8 {
    static inline s8 TEMP_TO_REG(long val)
    {
    let mut nval: c_int = clamp_val(val, -128000, 127000) ;
    return nval < 0 ? (nval - 500) / 1000 : (nval + 500) / 1000;
    }
#[no_mangle]
pub unsafe extern "C" fn TEMP_FROM_REG(val: i8) -> c_int {
    static inline int TEMP_FROM_REG(s8 val)
    {
    return val * 1000;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm78_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub type: enum chips,
// For ISA device only
    pub name: *const c_char,
    pub isa_addr: c_int,
    pub update_lock: mutex,
    pub /: *mut *mut bool valid; / true if following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub /: *mut *mut u8 in[7]; / Register value,
    pub /: *mut *mut u8 in_max[7]; / Register value,
    pub /: *mut *mut u8 in_min[7]; / Register value,
    pub /: *mut *mut u8 fan[3]; / Register value,
    pub /: *mut *mut u8 fan_min[3]; / Register value,
    pub /: *mut *mut s8 temp; / Register value,
    pub /: *mut *mut s8 temp_over; / Register value,
    pub /: *mut *mut s8 temp_hyst; / Register value,
    pub /: *mut *mut u8 fan_div[3]; / Register encoding, shifted right,
    pub /: *mut *mut u8 vid; / Register encoding, combined,
    pub /: *mut *mut u16 alarms; / Register encoding, combined,
}

    static int lm78_read_value(struct lm78_data *data, u8 reg);
    static int lm78_write_value(struct lm78_data *data, u8 reg, u8 value);
    static struct lm78_data *lm78_update_device(struct device *dev);
    static void lm78_init_device(struct lm78_data *data);
// 7 Voltages
    static ssize_t in_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", IN_FROM_REG(data.in[attr.index]));
    }
    static ssize_t in_min_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", IN_FROM_REG(data.in_min[attr.index]));
    }
    static ssize_t in_max_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", IN_FROM_REG(data.in_max[attr.index]));
    }
    static ssize_t in_min_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = dev_get_drvdata(dev);
    let mut nr: c_int = attr.index;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.in_min[nr] = IN_TO_REG(val);
    lm78_write_value(data, LM78_REG_IN_MIN(nr), data.in_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t in_max_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = dev_get_drvdata(dev);
    let mut nr: c_int = attr.index;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.in_max[nr] = IN_TO_REG(val);
    lm78_write_value(data, LM78_REG_IN_MAX(nr), data.in_max[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RO(in0_input, in, 0);
    static SENSOR_DEVICE_ATTR_RW(in0_min, in_min, 0);
    static SENSOR_DEVICE_ATTR_RW(in0_max, in_max, 0);
    static SENSOR_DEVICE_ATTR_RO(in1_input, in, 1);
    static SENSOR_DEVICE_ATTR_RW(in1_min, in_min, 1);
    static SENSOR_DEVICE_ATTR_RW(in1_max, in_max, 1);
    static SENSOR_DEVICE_ATTR_RO(in2_input, in, 2);
    static SENSOR_DEVICE_ATTR_RW(in2_min, in_min, 2);
    static SENSOR_DEVICE_ATTR_RW(in2_max, in_max, 2);
    static SENSOR_DEVICE_ATTR_RO(in3_input, in, 3);
    static SENSOR_DEVICE_ATTR_RW(in3_min, in_min, 3);
    static SENSOR_DEVICE_ATTR_RW(in3_max, in_max, 3);
    static SENSOR_DEVICE_ATTR_RO(in4_input, in, 4);
    static SENSOR_DEVICE_ATTR_RW(in4_min, in_min, 4);
    static SENSOR_DEVICE_ATTR_RW(in4_max, in_max, 4);
    static SENSOR_DEVICE_ATTR_RO(in5_input, in, 5);
    static SENSOR_DEVICE_ATTR_RW(in5_min, in_min, 5);
    static SENSOR_DEVICE_ATTR_RW(in5_max, in_max, 5);
    static SENSOR_DEVICE_ATTR_RO(in6_input, in, 6);
    static SENSOR_DEVICE_ATTR_RW(in6_min, in_min, 6);
    static SENSOR_DEVICE_ATTR_RW(in6_max, in_max, 6);
// Temperature
    static ssize_t temp1_input_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp));
    }
    static ssize_t temp1_max_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp_over));
    }
    static ssize_t temp1_max_store(struct device *dev,
    struct device_attribute *da, const char *buf,
    size_t count)
    {
    struct lm78_data *data = dev_get_drvdata(dev);
    long val;
    int err;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.temp_over = TEMP_TO_REG(val);
    lm78_write_value(data, LM78_REG_TEMP_OVER, data.temp_over);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp1_max_hyst_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp_hyst));
    }
    static ssize_t temp1_max_hyst_store(struct device *dev,
    struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct lm78_data *data = dev_get_drvdata(dev);
    long val;
    int err;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.temp_hyst = TEMP_TO_REG(val);
    lm78_write_value(data, LM78_REG_TEMP_HYST, data.temp_hyst);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static DEVICE_ATTR_RO(temp1_input);
    static DEVICE_ATTR_RW(temp1_max);
    static DEVICE_ATTR_RW(temp1_max_hyst);
// 3 Fans
    static ssize_t fan_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    let mut nr: c_int = attr.index;
    return sprintf(buf, "%d\n", FAN_FROM_REG(data.fan[nr],
    DIV_FROM_REG(data.fan_div[nr])));
    }
    static ssize_t fan_min_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    let mut nr: c_int = attr.index;
    return sprintf(buf, "%d\n", FAN_FROM_REG(data.fan_min[nr],
    DIV_FROM_REG(data.fan_div[nr])));
    }
    static ssize_t fan_min_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = dev_get_drvdata(dev);
    let mut nr: c_int = attr.index;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.fan_min[nr] = FAN_TO_REG(val, DIV_FROM_REG(data.fan_div[nr]));
    lm78_write_value(data, LM78_REG_FAN_MIN(nr), data.fan_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t fan_div_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", DIV_FROM_REG(data.fan_div[attr.index]));
    }
//
// Note: we save and restore the fan minimum here, because its value is
// determined in part by the fan divisor.  This follows the principle of
// least surprise; the user doesn't expect the fan minimum to change just
// because the divisor changed.
//
    static ssize_t fan_div_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct lm78_data *data = dev_get_drvdata(dev);
    let mut nr: c_int = attr.index;
    unsigned long min;
    u8 reg;
    unsigned long val;
    int err;
    err = kstrtoul(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    min = FAN_FROM_REG(data.fan_min[nr],
    DIV_FROM_REG(data.fan_div[nr]));
    switch (val) {
    case 1:
    data.fan_div[nr] = 0;
    break;
    case 2:
    data.fan_div[nr] = 1;
    break;
    case 4:
    data.fan_div[nr] = 2;
    break;
    case 8:
    data.fan_div[nr] = 3;
    break;
    default:
    dev_err(dev,
    "fan_div value %ld not supported. Choose one of 1, 2, 4 or 8!\n",
    val);
    mutex_unlock(&data.update_lock);
    return -EINVAL;
    }
    reg = lm78_read_value(data, LM78_REG_VID_FANDIV);
    switch (nr) {
    case 0:
    reg = (reg & 0xcf) | (data.fan_div[nr] << 4);
    break;
    case 1:
    reg = (reg & 0x3f) | (data.fan_div[nr] << 6);
    break;
    }
    lm78_write_value(data, LM78_REG_VID_FANDIV, reg);
    data.fan_min[nr] =
    FAN_TO_REG(min, DIV_FROM_REG(data.fan_div[nr]));
    lm78_write_value(data, LM78_REG_FAN_MIN(nr), data.fan_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RO(fan1_input, fan, 0);
    static SENSOR_DEVICE_ATTR_RW(fan1_min, fan_min, 0);
    static SENSOR_DEVICE_ATTR_RO(fan2_input, fan, 1);
    static SENSOR_DEVICE_ATTR_RW(fan2_min, fan_min, 1);
    static SENSOR_DEVICE_ATTR_RO(fan3_input, fan, 2);
    static SENSOR_DEVICE_ATTR_RW(fan3_min, fan_min, 2);
// Fan 3 divisor is locked in H/W
    static SENSOR_DEVICE_ATTR_RW(fan1_div, fan_div, 0);
    static SENSOR_DEVICE_ATTR_RW(fan2_div, fan_div, 1);
    static SENSOR_DEVICE_ATTR_RO(fan3_div, fan_div, 2);
// VID
    static ssize_t cpu0_vid_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%d\n", vid_from_reg(data.vid, 82));
    }
    static DEVICE_ATTR_RO(cpu0_vid);
// Alarms
    static ssize_t alarms_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    return sprintf(buf, "%u\n", data.alarms);
    }
    static DEVICE_ATTR_RO(alarms);
    static ssize_t alarm_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct lm78_data *data = lm78_update_device(dev);
    let mut nr: c_int = to_sensor_dev_attr(da).index;
    return sprintf(buf, "%u\n", (data.alarms >> nr) & 1);
    }
    static SENSOR_DEVICE_ATTR_RO(in0_alarm, alarm, 0);
    static SENSOR_DEVICE_ATTR_RO(in1_alarm, alarm, 1);
    static SENSOR_DEVICE_ATTR_RO(in2_alarm, alarm, 2);
    static SENSOR_DEVICE_ATTR_RO(in3_alarm, alarm, 3);
    static SENSOR_DEVICE_ATTR_RO(in4_alarm, alarm, 8);
    static SENSOR_DEVICE_ATTR_RO(in5_alarm, alarm, 9);
    static SENSOR_DEVICE_ATTR_RO(in6_alarm, alarm, 10);
    static SENSOR_DEVICE_ATTR_RO(fan1_alarm, alarm, 6);
    static SENSOR_DEVICE_ATTR_RO(fan2_alarm, alarm, 7);
    static SENSOR_DEVICE_ATTR_RO(fan3_alarm, alarm, 11);
    static SENSOR_DEVICE_ATTR_RO(temp1_alarm, alarm, 4);
    static struct attribute *lm78_attrs[] = {
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in0_min.dev_attr.attr,
    &sensor_dev_attr_in0_max.dev_attr.attr,
    &sensor_dev_attr_in0_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_min.dev_attr.attr,
    &sensor_dev_attr_in1_max.dev_attr.attr,
    &sensor_dev_attr_in1_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_min.dev_attr.attr,
    &sensor_dev_attr_in2_max.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in3_min.dev_attr.attr,
    &sensor_dev_attr_in3_max.dev_attr.attr,
    &sensor_dev_attr_in3_alarm.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in4_min.dev_attr.attr,
    &sensor_dev_attr_in4_max.dev_attr.attr,
    &sensor_dev_attr_in4_alarm.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in5_min.dev_attr.attr,
    &sensor_dev_attr_in5_max.dev_attr.attr,
    &sensor_dev_attr_in5_alarm.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in6_min.dev_attr.attr,
    &sensor_dev_attr_in6_max.dev_attr.attr,
    &sensor_dev_attr_in6_alarm.dev_attr.attr,
    &dev_attr_temp1_input.attr,
    &dev_attr_temp1_max.attr,
    &dev_attr_temp1_max_hyst.attr,
    &sensor_dev_attr_temp1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan1_div.dev_attr.attr,
    &sensor_dev_attr_fan1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan2_min.dev_attr.attr,
    &sensor_dev_attr_fan2_div.dev_attr.attr,
    &sensor_dev_attr_fan2_alarm.dev_attr.attr,
    &sensor_dev_attr_fan3_input.dev_attr.attr,
    &sensor_dev_attr_fan3_min.dev_attr.attr,
    &sensor_dev_attr_fan3_div.dev_attr.attr,
    &sensor_dev_attr_fan3_alarm.dev_attr.attr,
    &dev_attr_alarms.attr,
    &dev_attr_cpu0_vid.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(lm78);
//
// ISA related code
//

// ISA device, if found
    static struct platform_device *pdev;
    let mut isa_address: static unsigned short = 0x290;
    static struct lm78_data *lm78_data_if_isa(void)
    {
    return pdev ? platform_get_drvdata(pdev) : core::ptr::null_mut();
    }
// Returns 1 if the I2C chip appears to be an alias of the ISA chip
#[no_mangle]
unsafe extern "C" fn lm78_alias_detect(client: *mut i2c_client, chipid: u8) -> c_int {
    static int lm78_alias_detect(struct i2c_client *client, u8 chipid)
    {
    struct lm78_data *isa;
    int i;
    if (!pdev)	/* No ISA chip */
    return 0;
    isa = platform_get_drvdata(pdev);
    if (lm78_read_value(isa, LM78_REG_I2C_ADDR) != client.addr)
    return 0;	/* Address doesn't match */
    if ((lm78_read_value(isa, LM78_REG_CHIPID) & 0xfe) != (chipid & 0xfe))
    return 0;	/* Chip type doesn't match */
//
// We compare all the limit registers, the config register and the
// interrupt mask registers
//
    for (i = 0x2b; i <= 0x3d; i++) {
    if (lm78_read_value(isa, i) !=
    i2c_smbus_read_byte_data(client, i))
    return 0;
    }
    if (lm78_read_value(isa, LM78_REG_CONFIG) !=
    i2c_smbus_read_byte_data(client, LM78_REG_CONFIG))
    return 0;
    for (i = 0x43; i <= 0x46; i++) {
    if (lm78_read_value(isa, i) !=
    i2c_smbus_read_byte_data(client, i))
    return 0;
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn lm78_alias_detect(client: *mut i2c_client, chipid: u8) -> c_int {
    static int lm78_alias_detect(struct i2c_client *client, u8 chipid)
    {
    return 0;
    }
    static struct lm78_data *lm78_data_if_isa(void)
    {
    return core::ptr::null_mut();
    }

    static int lm78_i2c_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    int i;
    struct lm78_data *isa = lm78_data_if_isa();
    const char *client_name;
    struct i2c_adapter *adapter = client.adapter;
    let mut address: c_int = client.addr;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
//
// We block updates of the ISA device to minimize the risk of
// concurrent access to the same LM78 chip through different
// interfaces.
//
    if (isa)
    mutex_lock(&isa.update_lock);
    if ((i2c_smbus_read_byte_data(client, LM78_REG_CONFIG) & 0x80)
    || i2c_smbus_read_byte_data(client, LM78_REG_I2C_ADDR) != address)
    goto err_nodev;
// Explicitly prevent the misdetection of Winbond chips
    i = i2c_smbus_read_byte_data(client, 0x4f);
    if (i == 0xa3 || i == 0x5c)
    goto err_nodev;
// Determine the chip type.
    i = i2c_smbus_read_byte_data(client, LM78_REG_CHIPID);
    if (i == 0x00 || i == 0x20	/* LM78 */
    || i == 0x40)			/* LM78-J */
    client_name = "lm78";
#[no_mangle]
pub unsafe extern "C" fn if(0xc0: (i & 0xfe) ==) -> else {
    else if ((i & 0xfe) == 0xc0)
    client_name = "lm79";
    else
    goto err_nodev;
    if (lm78_alias_detect(client, i)) {
    dev_dbg(&adapter.dev,
    "Device at 0x%02x appears to be the same as ISA device\n",
    address);
    goto err_nodev;
    }
    if (isa)
    mutex_unlock(&isa.update_lock);
    strscpy(info.type, client_name, I2C_NAME_SIZE);
    return 0;
    err_nodev:
    if (isa)
    mutex_unlock(&isa.update_lock);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn lm78_i2c_probe(client: *mut i2c_client) -> c_int {
    static int lm78_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct lm78_data *data;
    data = devm_kzalloc(dev, sizeof(struct lm78_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    data.type = (uintptr_t)i2c_get_match_data(client);
// Initialize the LM78 chip
    lm78_init_device(data);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, lm78_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id lm78_i2c_id[] = {
    { .name = "lm78", .driver_data = lm78 },
    { .name = "lm79", .driver_data = lm79 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm78_i2c_id);
    static struct i2c_driver lm78_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "lm78",
    },
    .probe		= lm78_i2c_probe,
    .id_table	= lm78_i2c_id,
    .detect		= lm78_i2c_detect,
    .address_list	= normal_i2c,
    };
//
// The SMBus locks itself, but ISA access must be locked explicitly!
// We don't want to lock the whole ISA bus, so we lock each client
// separately.
// We ignore the LM78 BUSY flag at this moment - it could lead to deadlocks,
// would slow down the LM78 access and should not be necessary.
//
#[no_mangle]
unsafe extern "C" fn lm78_read_value(data: *mut lm78_data, reg: u8) -> c_int {
    static int lm78_read_value(struct lm78_data *data, u8 reg)
    {
    struct i2c_client *client = data.client;

    if (!client) { /* ISA device */
    int res;
    mutex_lock(&data.lock);
    outb_p(reg, data.isa_addr + LM78_ADDR_REG_OFFSET);
    res = inb_p(data.isa_addr + LM78_DATA_REG_OFFSET);
    mutex_unlock(&data.lock);
    return res;
    } else

    return i2c_smbus_read_byte_data(client, reg);
    }
#[no_mangle]
unsafe extern "C" fn lm78_write_value(data: *mut lm78_data, reg: u8, value: u8) -> c_int {
    static int lm78_write_value(struct lm78_data *data, u8 reg, u8 value)
    {
    struct i2c_client *client = data.client;

    if (!client) { /* ISA device */
    mutex_lock(&data.lock);
    outb_p(reg, data.isa_addr + LM78_ADDR_REG_OFFSET);
    outb_p(value, data.isa_addr + LM78_DATA_REG_OFFSET);
    mutex_unlock(&data.lock);
    return 0;
    } else

    return i2c_smbus_write_byte_data(client, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn lm78_init_device(data: *mut lm78_data) {
    static void lm78_init_device(struct lm78_data *data)
    {
    u8 config;
    int i;
// Start monitoring
    config = lm78_read_value(data, LM78_REG_CONFIG);
    if ((config & 0x09) != 0x01)
    lm78_write_value(data, LM78_REG_CONFIG,
    (config & 0xf7) | 0x01);
// A few vars need to be filled upon startup
    for (i = 0; i < 3; i++) {
    data.fan_min[i] = lm78_read_value(data,
    LM78_REG_FAN_MIN(i));
    }
    mutex_init(&data.update_lock);
    }
    static struct lm78_data *lm78_update_device(struct device *dev)
    {
    struct lm78_data *data = dev_get_drvdata(dev);
    int i;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ + HZ / 2)
    || !data.valid) {
    dev_dbg(dev, "Starting lm78 update\n");
    for (i = 0; i <= 6; i++) {
    data.in[i] =
    lm78_read_value(data, LM78_REG_IN(i));
    data.in_min[i] =
    lm78_read_value(data, LM78_REG_IN_MIN(i));
    data.in_max[i] =
    lm78_read_value(data, LM78_REG_IN_MAX(i));
    }
    for (i = 0; i < 3; i++) {
    data.fan[i] =
    lm78_read_value(data, LM78_REG_FAN(i));
    data.fan_min[i] =
    lm78_read_value(data, LM78_REG_FAN_MIN(i));
    }
    data.temp = lm78_read_value(data, LM78_REG_TEMP);
    data.temp_over =
    lm78_read_value(data, LM78_REG_TEMP_OVER);
    data.temp_hyst =
    lm78_read_value(data, LM78_REG_TEMP_HYST);
    i = lm78_read_value(data, LM78_REG_VID_FANDIV);
    data.vid = i & 0x0f;
    if (data.type == lm79)
    data.vid |=
    (lm78_read_value(data, LM78_REG_CHIPID) &
    0x01) << 4;
    else
    data.vid |= 0x10;
    data.fan_div[0] = (i >> 4) & 0x03;
    data.fan_div[1] = i >> 6;
    data.alarms = lm78_read_value(data, LM78_REG_ALARM1) +
    (lm78_read_value(data, LM78_REG_ALARM2) << 8);
    data.last_updated = jiffies;
    data.valid = true;
    data.fan_div[2] = 1;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }

#[no_mangle]
unsafe extern "C" fn lm78_isa_probe(pdev: *mut platform_device) -> c_int {
    static int lm78_isa_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *hwmon_dev;
    struct lm78_data *data;
    struct resource *res;
// Reserve the ISA region
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!devm_request_region(dev, res.start + LM78_ADDR_REG_OFFSET,
    2, "lm78"))
    return -EBUSY;
    data = devm_kzalloc(dev, sizeof(struct lm78_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    mutex_init(&data.lock);
    data.isa_addr = res.start;
    platform_set_drvdata(pdev, data);
    if (lm78_read_value(data, LM78_REG_CHIPID) & 0x80) {
    data.type = lm79;
    data.name = "lm79";
    } else {
    data.type = lm78;
    data.name = "lm78";
    }
// Initialize the LM78 chip
    lm78_init_device(data);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, data.name,
    data, lm78_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver lm78_isa_driver = {
    .driver = {
    .name	= "lm78",
    },
    .probe		= lm78_isa_probe,
    };
// return 1 if a supported chip is found, 0 otherwise
#[no_mangle]
unsafe extern "C" fn lm78_isa_found(address: c_ushort) -> int __init {
    static int __init lm78_isa_found(unsigned short address)
    {
    int val, save, found = 0;
    int port;
//
// Some boards declare base+0 to base+7 as a PNP device, some base+4
// to base+7 and some base+5 to base+6. So we better request each port
// individually for the probing phase.
//
    for (port = address; port < address + LM78_EXTENT; port++) {
    if (!request_region(port, 1, "lm78")) {
    pr_debug("Failed to request port 0x%x\n", port);
    goto release;
    }
    }
//
// We need the timeouts for at least some LM78-like
// chips. But only if we read 'undefined' registers.
// There used to be a "#define REALLY_SLOW_IO" to enforce that, but
// this has been without any effect since more than a decade, so it
// has been dropped.
//
    val = inb_p(address + 1);
    if (inb_p(address + 2) != val
    || inb_p(address + 3) != val
    || inb_p(address + 7) != val)
    goto release;
//
// We should be able to change the 7 LSB of the address port. The
// MSB (busy flag) should be clear initially, set after the write.
//
    save = inb_p(address + LM78_ADDR_REG_OFFSET);
    if (save & 0x80)
    goto release;
    val = ~save & 0x7f;
    outb_p(val, address + LM78_ADDR_REG_OFFSET);
    if (inb_p(address + LM78_ADDR_REG_OFFSET) != (val | 0x80)) {
    outb_p(save, address + LM78_ADDR_REG_OFFSET);
    goto release;
    }
// We found a device, now see if it could be an LM78
    outb_p(LM78_REG_CONFIG, address + LM78_ADDR_REG_OFFSET);
    val = inb_p(address + LM78_DATA_REG_OFFSET);
    if (val & 0x80)
    goto release;
    outb_p(LM78_REG_I2C_ADDR, address + LM78_ADDR_REG_OFFSET);
    val = inb_p(address + LM78_DATA_REG_OFFSET);
    if (val < 0x03 || val > 0x77)	/* Not a valid I2C address */
    goto release;
// The busy flag should be clear again
    if (inb_p(address + LM78_ADDR_REG_OFFSET) & 0x80)
    goto release;
// Explicitly prevent the misdetection of Winbond chips
    outb_p(0x4f, address + LM78_ADDR_REG_OFFSET);
    val = inb_p(address + LM78_DATA_REG_OFFSET);
    if (val == 0xa3 || val == 0x5c)
    goto release;
// Explicitly prevent the misdetection of ITE chips
    outb_p(0x58, address + LM78_ADDR_REG_OFFSET);
    val = inb_p(address + LM78_DATA_REG_OFFSET);
    if (val == 0x90)
    goto release;
// Determine the chip type
    outb_p(LM78_REG_CHIPID, address + LM78_ADDR_REG_OFFSET);
    val = inb_p(address + LM78_DATA_REG_OFFSET);
    if (val == 0x00 || val == 0x20	/* LM78 */
    || val == 0x40			/* LM78-J */
    || (val & 0xfe) == 0xc0)	/* LM79 */
    found = 1;
    if (found)
    pr_info("Found an %s chip at %#x\n",
    val & 0x80 ? "LM79" : "LM78", (int)address);
    release:
    for (port--; port >= address; port--)
    release_region(port, 1);
    return found;
    }
#[no_mangle]
unsafe extern "C" fn lm78_isa_device_add(address: c_ushort) -> int __init {
    static int __init lm78_isa_device_add(unsigned short address)
    {
    struct resource res = {
    .start	= address,
    .end	= address + LM78_EXTENT - 1,
    .name	= "lm78",
    .flags	= IORESOURCE_IO,
    };
    int err;
    pdev = platform_device_alloc("lm78", address);
    if (!pdev) {
    err = -ENOMEM;
    pr_err("Device allocation failed\n");
    goto exit;
    }
    err = platform_device_add_resources(pdev, &res, 1);
    if (err) {
    pr_err("Device resource addition failed (%d)\n", err);
    goto exit_device_put;
    }
    err = platform_device_add(pdev);
    if (err) {
    pr_err("Device addition failed (%d)\n", err);
    goto exit_device_put;
    }
    return 0;
    exit_device_put:
    platform_device_put(pdev);
    exit:
    pdev = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lm78_isa_register() -> int __init {
    static int __init lm78_isa_register(void)
    {
    int res;
    if (lm78_isa_found(isa_address)) {
    res = platform_driver_register(&lm78_isa_driver);
    if (res)
    goto exit;
// Sets global pdev as a side effect
    res = lm78_isa_device_add(isa_address);
    if (res)
    goto exit_unreg_isa_driver;
    }
    return 0;
    exit_unreg_isa_driver:
    platform_driver_unregister(&lm78_isa_driver);
    exit:
    return res;
    }
#[no_mangle]
unsafe extern "C" fn lm78_isa_unregister() {
    static void lm78_isa_unregister(void)
    {
    if (pdev) {
    platform_device_unregister(pdev);
    platform_driver_unregister(&lm78_isa_driver);
    }
    }

#[no_mangle]
unsafe extern "C" fn lm78_isa_register() -> int __init {
    static int __init lm78_isa_register(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm78_isa_unregister() {
    static void lm78_isa_unregister(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn sm_lm78_init() -> int __init {
    static int __init sm_lm78_init(void)
    {
    int res;
//
// We register the ISA device first, so that we can skip the
// registration of an I2C interface to the same device.
//
    res = lm78_isa_register();
    if (res)
    goto exit;
    res = i2c_add_driver(&lm78_driver);
    if (res)
    goto exit_unreg_isa_device;
    return 0;
    exit_unreg_isa_device:
    lm78_isa_unregister();
    exit:
    return res;
    }
#[no_mangle]
unsafe extern "C" fn sm_lm78_exit() -> void __exit {
    static void __exit sm_lm78_exit(void)
    {
    lm78_isa_unregister();
    i2c_del_driver(&lm78_driver);
    }
    MODULE_AUTHOR("Frodo Looijaard, Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("LM78/LM79 driver");
    MODULE_LICENSE("GPL");
    module_init(sm_lm78_init);
    module_exit(sm_lm78_exit);
