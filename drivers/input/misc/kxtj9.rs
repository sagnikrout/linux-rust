//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/kxtj9.c
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
// Copyright (C) 2011 Kionix, Inc.
// Written by Chris Hudson <chudson@kionix.com>
//

pub const G_MAX: c_int = 8000;
// OUTPUT REGISTERS
pub const XOUT_L: c_uint = 0x06;
pub const WHO_AM_I: c_uint = 0x0F;
// CONTROL REGISTERS
pub const INT_REL: c_uint = 0x1A;
pub const CTRL_REG1: c_uint = 0x1B;
pub const INT_CTRL1: c_uint = 0x1E;
pub const DATA_CTRL: c_uint = 0x21;
// CONTROL REGISTER 1 BITS
pub const PC1_OFF: c_uint = 0x7F;

// Data ready function enable bit: set during probe if using irq mode

// DATA CONTROL REGISTER BITS
pub const ODR12_5F: c_int = 0;
pub const ODR25F: c_int = 1;
pub const ODR50F: c_int = 2;
pub const ODR100F: c_int = 3;
pub const ODR200F: c_int = 4;
pub const ODR400F: c_int = 5;
pub const ODR800F: c_int = 6;
// INTERRUPT CONTROL REGISTER 1 BITS
// Set these during probe if using irq mode

// INPUT_ABS CONSTANTS
pub const FUZZ: c_int = 3;
pub const FLAT: c_int = 3;
// RESUME STATE INDICES
pub const RES_DATA_CTRL: c_int = 0;
pub const RES_CTRL_REG1: c_int = 1;
pub const RES_INT_CTRL1: c_int = 2;
pub const RESUME_ENTRIES: c_int = 3;
//
// The following table lists the maximum appropriate poll interval for each
// available output data rate.
//
    static const struct {
    unsigned int cutoff;
    u8 mask;
    } kxtj9_odr_table[] = {
    { 3,	ODR800F },
    { 5,	ODR400F },
    { 10,	ODR200F },
    { 20,	ODR100F },
    { 40,	ODR50F  },
    { 80,	ODR25F  },
    { 0,	ODR12_5F},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kxtj9_data {
    pub client: *mut i2c_client,
    pub pdata: kxtj9_platform_data,
    pub input_dev: *mut input_dev,
    pub last_poll_interval: c_uint,
    pub shift: u8,
    pub ctrl_reg1: u8,
    pub data_ctrl: u8,
    pub int_ctrl: u8,
}

#[no_mangle]
unsafe extern "C" fn kxtj9_i2c_read(tj9: *mut kxtj9_data, addr: u8, data: *mut u8, len: c_int) -> c_int {
    static int kxtj9_i2c_read(struct kxtj9_data *tj9, u8 addr, u8 *data, int len)
    {
    struct i2c_msg msgs[] = {
    {
    .addr = tj9.client.addr,
    .flags = tj9.client.flags,
    .len = 1,
    .buf = &addr,
    },
    {
    .addr = tj9.client.addr,
    .flags = tj9.client.flags | I2C_M_RD,
    .len = len,
    .buf = data,
    },
    };
    return i2c_transfer(tj9.client.adapter, msgs, 2);
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_report_acceleration_data(tj9: *mut kxtj9_data) {
    static void kxtj9_report_acceleration_data(struct kxtj9_data *tj9)
    {
    s16 acc_data[3]; /* Data bytes from hardware xL, xH, yL, yH, zL, zH */
    s16 x, y, z;
    int err;
    err = kxtj9_i2c_read(tj9, XOUT_L, (u8 *)acc_data, 6);
    if (err < 0)
    dev_err(&tj9.client.dev, "accelerometer data read failed\n");
    x = le16_to_cpu(acc_data[tj9.pdata.axis_map_x]);
    y = le16_to_cpu(acc_data[tj9.pdata.axis_map_y]);
    z = le16_to_cpu(acc_data[tj9.pdata.axis_map_z]);
    x >>= tj9.shift;
    y >>= tj9.shift;
    z >>= tj9.shift;
    input_report_abs(tj9.input_dev, ABS_X, tj9.pdata.negate_x ? -x : x);
    input_report_abs(tj9.input_dev, ABS_Y, tj9.pdata.negate_y ? -y : y);
    input_report_abs(tj9.input_dev, ABS_Z, tj9.pdata.negate_z ? -z : z);
    input_sync(tj9.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t kxtj9_isr(int irq, void *dev)
    {
    struct kxtj9_data *tj9 = dev;
    int err;
// data ready is the only possible interrupt type
    kxtj9_report_acceleration_data(tj9);
    err = i2c_smbus_read_byte_data(tj9.client, INT_REL);
    if (err < 0)
    dev_err(&tj9.client.dev,
    "error clearing interrupt status: %d\n", err);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_update_g_range(tj9: *mut kxtj9_data, new_g_range: u8) -> c_int {
    static int kxtj9_update_g_range(struct kxtj9_data *tj9, u8 new_g_range)
    {
    switch (new_g_range) {
    case KXTJ9_G_2G:
    tj9.shift = 4;
    break;
    case KXTJ9_G_4G:
    tj9.shift = 3;
    break;
    case KXTJ9_G_8G:
    tj9.shift = 2;
    break;
    default:
    return -EINVAL;
    }
    tj9.ctrl_reg1 &= 0xe7;
    tj9.ctrl_reg1 |= new_g_range;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_update_odr(tj9: *mut kxtj9_data, poll_interval: c_uint) -> c_int {
    static int kxtj9_update_odr(struct kxtj9_data *tj9, unsigned int poll_interval)
    {
    int err;
    int i;
// Use the lowest ODR that can support the requested poll interval
    for (i = 0; i < ARRAY_SIZE(kxtj9_odr_table); i++) {
    tj9.data_ctrl = kxtj9_odr_table[i].mask;
    if (poll_interval < kxtj9_odr_table[i].cutoff)
    break;
    }
    err = i2c_smbus_write_byte_data(tj9.client, CTRL_REG1, 0);
    if (err < 0)
    return err;
    err = i2c_smbus_write_byte_data(tj9.client, DATA_CTRL, tj9.data_ctrl);
    if (err < 0)
    return err;
    err = i2c_smbus_write_byte_data(tj9.client, CTRL_REG1, tj9.ctrl_reg1);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_device_power_on(tj9: *mut kxtj9_data) -> c_int {
    static int kxtj9_device_power_on(struct kxtj9_data *tj9)
    {
    if (tj9.pdata.power_on)
    return tj9.pdata.power_on();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_device_power_off(tj9: *mut kxtj9_data) {
    static void kxtj9_device_power_off(struct kxtj9_data *tj9)
    {
    int err;
    tj9.ctrl_reg1 &= PC1_OFF;
    err = i2c_smbus_write_byte_data(tj9.client, CTRL_REG1, tj9.ctrl_reg1);
    if (err < 0)
    dev_err(&tj9.client.dev, "soft power off failed\n");
    if (tj9.pdata.power_off)
    tj9.pdata.power_off();
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_enable(tj9: *mut kxtj9_data) -> c_int {
    static int kxtj9_enable(struct kxtj9_data *tj9)
    {
    int err;
    err = kxtj9_device_power_on(tj9);
    if (err < 0)
    return err;
// ensure that PC1 is cleared before updating control registers
    err = i2c_smbus_write_byte_data(tj9.client, CTRL_REG1, 0);
    if (err < 0)
    return err;
// only write INT_CTRL_REG1 if in irq mode
    if (tj9.client.irq) {
    err = i2c_smbus_write_byte_data(tj9.client,
    INT_CTRL1, tj9.int_ctrl);
    if (err < 0)
    return err;
    }
    err = kxtj9_update_g_range(tj9, tj9.pdata.g_range);
    if (err < 0)
    return err;
// turn on outputs
    tj9.ctrl_reg1 |= PC1_ON;
    err = i2c_smbus_write_byte_data(tj9.client, CTRL_REG1, tj9.ctrl_reg1);
    if (err < 0)
    return err;
    err = kxtj9_update_odr(tj9, tj9.last_poll_interval);
    if (err < 0)
    return err;
// clear initial interrupt if in irq mode
    if (tj9.client.irq) {
    err = i2c_smbus_read_byte_data(tj9.client, INT_REL);
    if (err < 0) {
    dev_err(&tj9.client.dev,
    "error clearing interrupt: %d\n", err);
    goto fail;
    }
    }
    return 0;
    fail:
    kxtj9_device_power_off(tj9);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_disable(tj9: *mut kxtj9_data) {
    static void kxtj9_disable(struct kxtj9_data *tj9)
    {
    kxtj9_device_power_off(tj9);
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_input_open(input: *mut input_dev) -> c_int {
    static int kxtj9_input_open(struct input_dev *input)
    {
    struct kxtj9_data *tj9 = input_get_drvdata(input);
    return kxtj9_enable(tj9);
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_input_close(dev: *mut input_dev) {
    static void kxtj9_input_close(struct input_dev *dev)
    {
    struct kxtj9_data *tj9 = input_get_drvdata(dev);
    kxtj9_disable(tj9);
    }
//
// When IRQ mode is selected, we need to provide an interface to allow the user
// to change the output data rate of the part.  For consistency, we are using
// the set_poll method, which accepts a poll interval in milliseconds, and then
// calls update_odr() while passing this value as an argument.  In IRQ mode, the
// data outputs will not be read AT the requested poll interval, rather, the
// lowest ODR that can support the requested interval.  The client application
// will be responsible for retrieving data from the input node at the desired
// interval.
//
// Returns currently selected poll interval (in ms)
    static ssize_t kxtj9_get_poll(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct kxtj9_data *tj9 = i2c_get_clientdata(client);
    return sprintf(buf, "%d\n", tj9.last_poll_interval);
    }
// Allow users to select a new poll interval (in ms)
    static ssize_t kxtj9_set_poll(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct kxtj9_data *tj9 = i2c_get_clientdata(client);
    struct input_dev *input_dev = tj9.input_dev;
    unsigned int interval;
    int error;
    error = kstrtouint(buf, 10, &interval);
    if (error < 0)
    return error;
// Lock the device to prevent races with open/close (and itself)
    guard(mutex)(&input_dev.mutex);
    guard(disable_irq)(&client.irq);
//
// Set current interval to the greater of the minimum interval or
// the requested interval
//
    tj9.last_poll_interval = max(interval, tj9.pdata.min_interval);
    kxtj9_update_odr(tj9, tj9.last_poll_interval);
    return count;
    }
    static DEVICE_ATTR(poll, S_IRUGO|S_IWUSR, kxtj9_get_poll, kxtj9_set_poll);
    static struct attribute *kxtj9_attrs[] = {
    &dev_attr_poll.attr,
    core::ptr::null_mut()
    };
    static umode_t kxtj9_attr_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct i2c_client *client = to_i2c_client(dev);
    return client.irq ? attr.mode : 0;
    }
    static struct attribute_group kxtj9_group = {
    .attrs = kxtj9_attrs,
    .is_visible = kxtj9_attr_is_visible,
    };
    __ATTRIBUTE_GROUPS(kxtj9);
#[no_mangle]
unsafe extern "C" fn kxtj9_poll(input: *mut input_dev) {
    static void kxtj9_poll(struct input_dev *input)
    {
    struct kxtj9_data *tj9 = input_get_drvdata(input);
    let mut poll_interval: c_uint = input_get_poll_interval(input);
    kxtj9_report_acceleration_data(tj9);
    if (poll_interval != tj9.last_poll_interval) {
    kxtj9_update_odr(tj9, poll_interval);
    tj9.last_poll_interval = poll_interval;
    }
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_platform_exit(data: *mut c_void) {
    static void kxtj9_platform_exit(void *data)
    {
    struct kxtj9_data *tj9 = data;
    if (tj9.pdata.exit)
    tj9.pdata.exit();
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_verify(tj9: *mut kxtj9_data) -> c_int {
    static int kxtj9_verify(struct kxtj9_data *tj9)
    {
    int retval;
    retval = kxtj9_device_power_on(tj9);
    if (retval < 0)
    return retval;
    retval = i2c_smbus_read_byte_data(tj9.client, WHO_AM_I);
    if (retval < 0) {
    dev_err(&tj9.client.dev, "read err int source\n");
    goto out;
    }
    retval = (retval != 0x07 && retval != 0x08) ? -EIO : 0;
    out:
    kxtj9_device_power_off(tj9);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_probe(client: *mut i2c_client) -> c_int {
    static int kxtj9_probe(struct i2c_client *client)
    {
    const struct kxtj9_platform_data *pdata =
    dev_get_platdata(&client.dev);
    struct kxtj9_data *tj9;
    struct input_dev *input_dev;
    int err;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_I2C | I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&client.dev, "client is not i2c capable\n");
    return -ENXIO;
    }
    if (!pdata) {
    dev_err(&client.dev, "platform data is core::ptr::null_mut(); exiting\n");
    return -EINVAL;
    }
    tj9 = devm_kzalloc(&client.dev, sizeof(*tj9), GFP_KERNEL);
    if (!tj9) {
    dev_err(&client.dev,
    "failed to allocate memory for module data\n");
    return -ENOMEM;
    }
    tj9.client = client;
    tj9.pdata = *pdata;
    if (pdata.init) {
    err = pdata.init();
    if (err < 0)
    return err;
    }
    err = devm_add_action_or_reset(&client.dev, kxtj9_platform_exit, tj9);
    if (err)
    return err;
    err = kxtj9_verify(tj9);
    if (err < 0) {
    dev_err(&client.dev, "device not recognized\n");
    return err;
    }
    i2c_set_clientdata(client, tj9);
    tj9.ctrl_reg1 = tj9.pdata.res_12bit | tj9.pdata.g_range;
    tj9.last_poll_interval = tj9.pdata.init_interval;
    input_dev = devm_input_allocate_device(&client.dev);
    if (!input_dev) {
    dev_err(&client.dev, "input device allocate failed\n");
    return -ENOMEM;
    }
    input_set_drvdata(input_dev, tj9);
    tj9.input_dev = input_dev;
    input_dev.name = "kxtj9_accel";
    input_dev.id.bustype = BUS_I2C;
    input_dev.open = kxtj9_input_open;
    input_dev.close = kxtj9_input_close;
    input_set_abs_params(input_dev, ABS_X, -G_MAX, G_MAX, FUZZ, FLAT);
    input_set_abs_params(input_dev, ABS_Y, -G_MAX, G_MAX, FUZZ, FLAT);
    input_set_abs_params(input_dev, ABS_Z, -G_MAX, G_MAX, FUZZ, FLAT);
    if (client.irq <= 0) {
    err = input_setup_polling(input_dev, kxtj9_poll);
    if (err)
    return err;
    }
    err = input_register_device(input_dev);
    if (err) {
    dev_err(&client.dev,
    "unable to register input polled device %s: %d\n",
    input_dev.name, err);
    return err;
    }
    if (client.irq) {
// If in irq mode, populate INT_CTRL_REG1 and enable DRDY.
    tj9.int_ctrl |= KXTJ9_IEN | KXTJ9_IEA | KXTJ9_IEL;
    tj9.ctrl_reg1 |= DRDYE;
    err = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), kxtj9_isr,
    IRQF_TRIGGER_RISING |
    IRQF_ONESHOT,
    "kxtj9-irq", tj9);
    if (err) {
    dev_err(&client.dev, "request irq failed: %d\n", err);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_suspend(dev: *mut device) -> c_int {
    static int kxtj9_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct kxtj9_data *tj9 = i2c_get_clientdata(client);
    struct input_dev *input_dev = tj9.input_dev;
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev))
    kxtj9_disable(tj9);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxtj9_resume(dev: *mut device) -> c_int {
    static int kxtj9_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct kxtj9_data *tj9 = i2c_get_clientdata(client);
    struct input_dev *input_dev = tj9.input_dev;
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev))
    kxtj9_enable(tj9);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(kxtj9_pm_ops, kxtj9_suspend, kxtj9_resume);
    static const struct i2c_device_id kxtj9_id[] = {
    { .name = NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, kxtj9_id);
    static struct i2c_driver kxtj9_driver = {
    .driver = {
    .name		= NAME,
    .dev_groups	= kxtj9_groups,
    .pm		= pm_sleep_ptr(&kxtj9_pm_ops),
    },
    .probe		= kxtj9_probe,
    .id_table	= kxtj9_id,
    };
    module_i2c_driver(kxtj9_driver);
    MODULE_DESCRIPTION("KXTJ9 accelerometer driver");
    MODULE_AUTHOR("Chris Hudson <chudson@kionix.com>");
    MODULE_LICENSE("GPL");
