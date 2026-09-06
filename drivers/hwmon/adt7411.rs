//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adt7411.c
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
//
// Driver for the ADT7411 (I2C/SPI 8 channel 10 bit ADC & temperature-sensor)
//
// Copyright (C) 2008, 2010 Pengutronix
//
// TODO: SPI, use power-down mode for suspend?, interrupt handling?
//

pub const ADT7411_REG_STAT_1: c_uint = 0x00;

pub const ADT7411_REG_STAT_2: c_uint = 0x01;

pub const ADT7411_REG_INT_TEMP_VDD_LSB: c_uint = 0x03;
pub const ADT7411_REG_EXT_TEMP_AIN14_LSB: c_uint = 0x04;
pub const ADT7411_REG_VDD_MSB: c_uint = 0x06;
pub const ADT7411_REG_INT_TEMP_MSB: c_uint = 0x07;
pub const ADT7411_REG_EXT_TEMP_AIN1_MSB: c_uint = 0x08;
pub const ADT7411_REG_CFG1: c_uint = 0x18;

pub const ADT7411_REG_CFG2: c_uint = 0x19;

pub const ADT7411_REG_CFG3: c_uint = 0x1a;

pub const ADT7411_REG_VDD_HIGH: c_uint = 0x23;
pub const ADT7411_REG_VDD_LOW: c_uint = 0x24;

    ? 0x2b + 2 * ((nr)-2) \
    : 0x27)

    ? 0x2c + 2 * ((nr)-2) \
    : 0x28)
pub const ADT7411_REG_DEVICE_ID: c_uint = 0x4d;
pub const ADT7411_REG_MANUFACTURER_ID: c_uint = 0x4e;
pub const ADT7411_DEVICE_ID: c_uint = 0x2;
pub const ADT7411_MANUFACTURER_ID: c_uint = 0x41;
    static const unsigned short normal_i2c[] = { 0x48, 0x4a, 0x4b, I2C_CLIENT_END };
    static const u8 adt7411_in_alarm_reg[] = {
    ADT7411_REG_STAT_2,
    ADT7411_REG_STAT_1,
    ADT7411_REG_STAT_1,
    ADT7411_REG_STAT_1,
    ADT7411_REG_STAT_1,
    ADT7411_REG_STAT_2,
    ADT7411_REG_STAT_2,
    ADT7411_REG_STAT_2,
    ADT7411_REG_STAT_2,
    };
    static const u8 adt7411_in_alarm_bits[] = {
    ADT7411_STAT_2_VDD,
    ADT7411_STAT_1_EXT_TEMP_HIGH_AIN1,
    ADT7411_STAT_1_AIN2,
    ADT7411_STAT_1_AIN3,
    ADT7411_STAT_1_AIN4,
    ADT7411_STAT_2_AIN5,
    ADT7411_STAT_2_AIN6,
    ADT7411_STAT_2_AIN7,
    ADT7411_STAT_2_AIN8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adt7411_data {
    pub next_update: c_ulong,
    pub vref_cached: c_long,
    pub client: *mut i2c_client,
    pub use_ext_temp: bool,
}

//
// When reading a register containing (up to 4) lsb, all associated
// msb-registers get locked by the hardware. After _one_ of those msb is read,
// _all_ are unlocked.
//
    static int adt7411_read_10_bit(struct i2c_client *client, u8 lsb_reg,
    u8 msb_reg, u8 lsb_shift)
    {
    int val, tmp;
    val = i2c_smbus_read_byte_data(client, lsb_reg);
    if (val < 0)
    return val;
    tmp = (val >> lsb_shift) & 3;
    val = i2c_smbus_read_byte_data(client, msb_reg);
    if (val < 0)
    return val;
    val = (val << 2) | tmp;
    return val;
    }
    static int adt7411_modify_bit(struct i2c_client *client, u8 reg, u8 bit,
    bool flag)
    {
    int ret, val;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    return ret;
    if (flag)
    val = ret | bit;
    else
    val = ret & ~bit;
    return i2c_smbus_write_byte_data(client, reg, val);
    }
    static ssize_t adt7411_show_bit(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sensor_device_attribute_2 *attr2 = to_sensor_dev_attr_2(attr);
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut ret: c_int = i2c_smbus_read_byte_data(client, attr2.index);
    return ret < 0 ? ret : sprintf(buf, "%u\n", !!(ret & attr2.nr));
    }
    static ssize_t adt7411_set_bit(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct sensor_device_attribute_2 *s_attr2 = to_sensor_dev_attr_2(attr);
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    unsigned long flag;
    ret = kstrtoul(buf, 0, &flag);
    if (ret || flag > 1)
    return -EINVAL;
    scoped_guard(hwmon_lock, dev) {
    ret = adt7411_modify_bit(client, s_attr2.index, s_attr2.nr, flag);
// force update
    data.next_update = jiffies;
    }
    return ret < 0 ? ret : count;
    }

    SENSOR_DEVICE_ATTR_2(__name, S_IRUGO | S_IWUSR, adt7411_show_bit, \
    adt7411_set_bit, __bit, __reg)
    static ADT7411_BIT_ATTR(no_average, ADT7411_REG_CFG2, ADT7411_CFG2_DISABLE_AVG);
    static ADT7411_BIT_ATTR(fast_sampling, ADT7411_REG_CFG3, ADT7411_CFG3_ADC_CLK_225);
    static ADT7411_BIT_ATTR(adc_ref_vdd, ADT7411_REG_CFG3, ADT7411_CFG3_REF_VDD);
    static struct attribute *adt7411_attrs[] = {
    &sensor_dev_attr_no_average.dev_attr.attr,
    &sensor_dev_attr_fast_sampling.dev_attr.attr,
    &sensor_dev_attr_adc_ref_vdd.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(adt7411);
#[no_mangle]
unsafe extern "C" fn adt7411_read_in_alarm(dev: *mut device, channel: c_int, val: *mut c_long) -> c_int {
    static int adt7411_read_in_alarm(struct device *dev, int channel, long *val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    ret = i2c_smbus_read_byte_data(client, adt7411_in_alarm_reg[channel]);
    if (ret < 0)
    return ret;
// val = !!(ret & adt7411_in_alarm_bits[channel]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adt7411_read_in_vdd(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int adt7411_read_in_vdd(struct device *dev, u32 attr, long *val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    switch (attr) {
    case hwmon_in_input:
    ret = adt7411_read_10_bit(client, ADT7411_REG_INT_TEMP_VDD_LSB,
    ADT7411_REG_VDD_MSB, 2);
    if (ret < 0)
    return ret;
// val = ret * 7000 / 1024;
    return 0;
    case hwmon_in_min:
    ret = i2c_smbus_read_byte_data(client, ADT7411_REG_VDD_LOW);
    if (ret < 0)
    return ret;
// val = ret * 7000 / 256;
    return 0;
    case hwmon_in_max:
    ret = i2c_smbus_read_byte_data(client, ADT7411_REG_VDD_HIGH);
    if (ret < 0)
    return ret;
// val = ret * 7000 / 256;
    return 0;
    case hwmon_in_alarm:
    return adt7411_read_in_alarm(dev, 0, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn adt7411_update_vref(dev: *mut device) -> c_int {
    static int adt7411_update_vref(struct device *dev)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int val;
    if (time_after_eq(jiffies, data.next_update)) {
    val = i2c_smbus_read_byte_data(client, ADT7411_REG_CFG3);
    if (val < 0)
    return val;
    if (val & ADT7411_CFG3_REF_VDD) {
    val = adt7411_read_in_vdd(dev, hwmon_in_input,
    &data.vref_cached);
    if (val < 0)
    return val;
    } else {
    data.vref_cached = 2250;
    }
    data.next_update = jiffies + HZ;
    }
    return 0;
    }
    static int adt7411_read_in_chan(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    int reg, lsb_reg, lsb_shift;
    let mut nr: c_int = channel - 1;
    ret = adt7411_update_vref(dev);
    if (ret < 0)
    return ret;
    switch (attr) {
    case hwmon_in_input:
    lsb_reg = ADT7411_REG_EXT_TEMP_AIN14_LSB + (nr >> 2);
    lsb_shift = 2 * (nr & 0x03);
    ret = adt7411_read_10_bit(client, lsb_reg,
    ADT7411_REG_EXT_TEMP_AIN1_MSB + nr,
    lsb_shift);
    if (ret < 0)
    return ret;
// val = ret * data->vref_cached / 1024;
    ret = 0;
    break;
    case hwmon_in_min:
    case hwmon_in_max:
    reg = (attr == hwmon_in_min)
    ? ADT7411_REG_IN_LOW(channel)
    : ADT7411_REG_IN_HIGH(channel);
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    return ret;
// val = ret * data->vref_cached / 256;
    ret = 0;
    break;
    case hwmon_in_alarm:
    ret = adt7411_read_in_alarm(dev, channel, val);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static int adt7411_read_in(struct device *dev, u32 attr, int channel,
    long *val)
    {
    if (channel == 0)
    return adt7411_read_in_vdd(dev, attr, val);
    else
    return adt7411_read_in_chan(dev, attr, channel, val);
    }
    static int adt7411_read_temp_alarm(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret, bit;
    ret = i2c_smbus_read_byte_data(client, ADT7411_REG_STAT_1);
    if (ret < 0)
    return ret;
    switch (attr) {
    case hwmon_temp_min_alarm:
    bit = channel ? ADT7411_STAT_1_EXT_TEMP_LOW
    : ADT7411_STAT_1_INT_TEMP_LOW;
    break;
    case hwmon_temp_max_alarm:
    bit = channel ? ADT7411_STAT_1_EXT_TEMP_HIGH_AIN1
    : ADT7411_STAT_1_INT_TEMP_HIGH;
    break;
    case hwmon_temp_fault:
    bit = ADT7411_STAT_1_EXT_TEMP_FAULT;
    break;
    default:
    return -EOPNOTSUPP;
    }
// val = !!(ret & bit);
    return 0;
    }
    static int adt7411_read_temp(struct device *dev, u32 attr, int channel,
    long *val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret, reg, regl, regh;
    switch (attr) {
    case hwmon_temp_input:
    regl = channel ? ADT7411_REG_EXT_TEMP_AIN14_LSB :
    ADT7411_REG_INT_TEMP_VDD_LSB;
    regh = channel ? ADT7411_REG_EXT_TEMP_AIN1_MSB :
    ADT7411_REG_INT_TEMP_MSB;
    ret = adt7411_read_10_bit(client, regl, regh, 0);
    if (ret < 0)
    return ret;
    ret = ret & 0x200 ? ret - 0x400 : ret; /* 10 bit signed */
// val = ret * 250;
    return 0;
    case hwmon_temp_min:
    case hwmon_temp_max:
    reg = (attr == hwmon_temp_min)
    ? ADT7411_REG_TEMP_LOW(channel)
    : ADT7411_REG_TEMP_HIGH(channel);
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    return ret;
    ret = ret & 0x80 ? ret - 0x100 : ret; /* 8 bit signed */
// val = ret * 1000;
    return 0;
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_fault:
    return adt7411_read_temp_alarm(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int adt7411_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_in:
    return adt7411_read_in(dev, attr, channel, val);
    case hwmon_temp:
    return adt7411_read_temp(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn adt7411_write_in_vdd(dev: *mut device, attr: u32, val: c_long) -> c_int {
    static int adt7411_write_in_vdd(struct device *dev, u32 attr, long val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int reg;
    val = clamp_val(val, 0, 255 * 7000 / 256);
    val = DIV_ROUND_CLOSEST(val * 256, 7000);
    switch (attr) {
    case hwmon_in_min:
    reg = ADT7411_REG_VDD_LOW;
    break;
    case hwmon_in_max:
    reg = ADT7411_REG_VDD_HIGH;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return i2c_smbus_write_byte_data(client, reg, val);
    }
    static int adt7411_write_in_chan(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret, reg;
    ret = adt7411_update_vref(dev);
    if (ret < 0)
    return ret;
    val = clamp_val(val, 0, 255 * data.vref_cached / 256);
    val = DIV_ROUND_CLOSEST(val * 256, data.vref_cached);
    switch (attr) {
    case hwmon_in_min:
    reg = ADT7411_REG_IN_LOW(channel);
    break;
    case hwmon_in_max:
    reg = ADT7411_REG_IN_HIGH(channel);
    break;
    default:
    return -EOPNOTSUPP;
    }
    ret = i2c_smbus_write_byte_data(client, reg, val);
    return ret;
    }
    static int adt7411_write_in(struct device *dev, u32 attr, int channel,
    long val)
    {
    if (channel == 0)
    return adt7411_write_in_vdd(dev, attr, val);
    else
    return adt7411_write_in_chan(dev, attr, channel, val);
    }
    static int adt7411_write_temp(struct device *dev, u32 attr, int channel,
    long val)
    {
    struct adt7411_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int reg;
    val = clamp_val(val, -128000, 127000);
    val = DIV_ROUND_CLOSEST(val, 1000);
    switch (attr) {
    case hwmon_temp_min:
    reg = ADT7411_REG_TEMP_LOW(channel);
    break;
    case hwmon_temp_max:
    reg = ADT7411_REG_TEMP_HIGH(channel);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return i2c_smbus_write_byte_data(client, reg, val);
    }
    static int adt7411_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_in:
    return adt7411_write_in(dev, attr, channel, val);
    case hwmon_temp:
    return adt7411_write_temp(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t adt7411_is_visible(const void *_data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct adt7411_data *data = _data;
    bool visible;
    switch (type) {
    case hwmon_in:
    visible = channel == 0 || channel >= 3 || !data.use_ext_temp;
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_alarm:
    return visible ? S_IRUGO : 0;
    case hwmon_in_min:
    case hwmon_in_max:
    return visible ? S_IRUGO | S_IWUSR : 0;
    }
    break;
    case hwmon_temp:
    visible = channel == 0 || data.use_ext_temp;
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_fault:
    return visible ? S_IRUGO : 0;
    case hwmon_temp_min:
    case hwmon_temp_max:
    return visible ? S_IRUGO | S_IWUSR : 0;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static int adt7411_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    int val;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    val = i2c_smbus_read_byte_data(client, ADT7411_REG_MANUFACTURER_ID);
    if (val < 0 || val != ADT7411_MANUFACTURER_ID) {
    dev_dbg(&client.dev,
    "Wrong manufacturer ID. Got %d, expected %d\n",
    val, ADT7411_MANUFACTURER_ID);
    return -ENODEV;
    }
    val = i2c_smbus_read_byte_data(client, ADT7411_REG_DEVICE_ID);
    if (val < 0 || val != ADT7411_DEVICE_ID) {
    dev_dbg(&client.dev,
    "Wrong device ID. Got %d, expected %d\n",
    val, ADT7411_DEVICE_ID);
    return -ENODEV;
    }
    strscpy(info.type, "adt7411", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adt7411_init_device(data: *mut adt7411_data) -> c_int {
    static int adt7411_init_device(struct adt7411_data *data)
    {
    int ret;
    u8 val;
    ret = i2c_smbus_read_byte_data(data.client, ADT7411_REG_CFG3);
    if (ret < 0)
    return ret;
//
// We must only write zero to bit 1 and bit 2 and only one to bit 3
// according to the datasheet.
//
    val = ret;
    val &= ~(ADT7411_CFG3_RESERVED_BIT1 | ADT7411_CFG3_RESERVED_BIT2);
    val |= ADT7411_CFG3_RESERVED_BIT3;
    ret = i2c_smbus_write_byte_data(data.client, ADT7411_REG_CFG3, val);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(data.client, ADT7411_REG_CFG1);
    if (ret < 0)
    return ret;
    data.use_ext_temp = ret & ADT7411_CFG1_EXT_TDM;
//
// We must only write zero to bit 1 and only one to bit 3 according to
// the datasheet.
//
    val = ret;
    val &= ~ADT7411_CFG1_RESERVED_BIT1;
    val |= ADT7411_CFG1_RESERVED_BIT3;
// enable monitoring
    val |= ADT7411_CFG1_START_MONITOR;
    return i2c_smbus_write_byte_data(data.client, ADT7411_REG_CFG1, val);
    }
    static const struct hwmon_channel_info * const adt7411_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX | HWMON_I_ALARM),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_MAX | HWMON_T_MAX_ALARM,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MIN_ALARM |
    HWMON_T_MAX | HWMON_T_MAX_ALARM | HWMON_T_FAULT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops adt7411_hwmon_ops = {
    .is_visible = adt7411_is_visible,
    .read = adt7411_read,
    .write = adt7411_write,
    };
    static const struct hwmon_chip_info adt7411_chip_info = {
    .ops = &adt7411_hwmon_ops,
    .info = adt7411_info,
    };
#[no_mangle]
unsafe extern "C" fn adt7411_probe(client: *mut i2c_client) -> c_int {
    static int adt7411_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct adt7411_data *data;
    struct device *hwmon_dev;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.client = client;
    ret = adt7411_init_device(data);
    if (ret < 0)
    return ret;
// force update on first occasion
    data.next_update = jiffies;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data,
    &adt7411_chip_info,
    adt7411_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id adt7411_id[] = {
    { .name = "adt7411" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adt7411_id);
    static struct i2c_driver adt7411_driver = {
    .driver		= {
    .name		= "adt7411",
    },
    .probe = adt7411_probe,
    .id_table = adt7411_id,
    .detect = adt7411_detect,
    .address_list = normal_i2c,
    .class = I2C_CLASS_HWMON,
    };
    module_i2c_driver(adt7411_driver);
    MODULE_AUTHOR("Sascha Hauer, Wolfram Sang <kernel@pengutronix.de>");
    MODULE_DESCRIPTION("ADT7411 driver");
    MODULE_LICENSE("GPL v2");
