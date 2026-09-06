//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/ds2782_battery.c
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
// I2C client/driver for the Maxim/Dallas DS2782 Stand-Alone Fuel Gauge IC
//
// Copyright (C) 2009 Bluewater Systems Ltd
//
// Author: Ryan Mallon
//
// DS2786 added by Yulia Vilensky <vilensky@compulab.co.il>
//
// UEvent sending added by Evgeny Romanov <romanov@neurosoft.ru>
//

pub const DS2782_REG_RARC: c_uint = 0x06	/* Remaining active relative capacity */;
pub const DS278x_REG_VOLT_MSB: c_uint = 0x0c;
pub const DS278x_REG_TEMP_MSB: c_uint = 0x0a;
pub const DS278x_REG_CURRENT_MSB: c_uint = 0x0e;
// EEPROM Block
pub const DS2782_REG_RSNSP: c_uint = 0x69	/* Sense resistor value */;
// Current unit measurement in uA for a 1 milli-ohm sense resistor
pub const DS2782_CURRENT_UNITS: c_int = 1563;
pub const DS2786_REG_RARC: c_uint = 0x02	/* Remaining active relative capacity */;
pub const DS2786_CURRENT_UNITS: c_int = 25;
pub const DS278x_DELAY: c_int = 1000;
    struct ds278x_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds278x_battery_ops {
    pub current_uA): *mut *mut *mut int (get_battery_current)(struct ds278x_info info, int,
    pub voltage_uV): *mut *mut *mut int (get_battery_voltage)(struct ds278x_info info, int,
    pub capacity): *mut *mut *mut int (get_battery_capacity)(struct ds278x_info info, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds278x_info {
    pub client: *mut i2c_client,
    pub battery: *mut power_supply,
    pub battery_desc: power_supply_desc,
    pub ops: *const ds278x_battery_ops,
    pub bat_work: delayed_work,
    pub rsns: c_int,
    pub capacity: c_int,
    pub /: *mut *mut int status; / State Of Charge,
}

    static DEFINE_IDA(battery_id);
#[no_mangle]
pub unsafe extern "C" fn ds278x_read_reg(info: *mut ds278x_info, reg: c_int, val: *mut u8) -> c_int {
    static inline int ds278x_read_reg(struct ds278x_info *info, int reg, u8 *val)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(info.client, reg);
    if (ret < 0) {
    dev_err(&info.client.dev, "register read failed\n");
    return ret;
    }
// val = ret;
    return 0;
    }
    static inline int ds278x_read_reg16(struct ds278x_info *info, int reg_msb,
    s16 *val)
    {
    int ret;
    ret = i2c_smbus_read_word_data(info.client, reg_msb);
    if (ret < 0) {
    dev_err(&info.client.dev, "register read failed\n");
    return ret;
    }
// val = swab16(ret);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds278x_get_temp(info: *mut ds278x_info, temp: *mut c_int) -> c_int {
    static int ds278x_get_temp(struct ds278x_info *info, int *temp)
    {
    s16 raw;
    int err;
//
// Temperature is measured in units of 0.125 degrees celcius, the
// power_supply class measures temperature in tenths of degrees
// celsius. The temperature value is stored as a 10 bit number, plus
// sign in the upper bits of a 16 bit register.
//
    err = ds278x_read_reg16(info, DS278x_REG_TEMP_MSB, &raw);
    if (err)
    return err;
// temp = ((raw / 32) * 125) / 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2782_get_current(info: *mut ds278x_info, current_uA: *mut c_int) -> c_int {
    static int ds2782_get_current(struct ds278x_info *info, int *current_uA)
    {
    int sense_res;
    int err;
    u8 sense_res_raw;
    s16 raw;
//
// The units of measurement for current are dependent on the value of
// the sense resistor.
//
    err = ds278x_read_reg(info, DS2782_REG_RSNSP, &sense_res_raw);
    if (err)
    return err;
    if (sense_res_raw == 0) {
    dev_err(&info.client.dev, "sense resistor value is 0\n");
    return -ENXIO;
    }
    sense_res = 1000 / sense_res_raw;
    dev_dbg(&info.client.dev, "sense resistor = %d milli-ohms\n",
    sense_res);
    err = ds278x_read_reg16(info, DS278x_REG_CURRENT_MSB, &raw);
    if (err)
    return err;
// current_uA = raw * (DS2782_CURRENT_UNITS / sense_res);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2782_get_voltage(info: *mut ds278x_info, voltage_uV: *mut c_int) -> c_int {
    static int ds2782_get_voltage(struct ds278x_info *info, int *voltage_uV)
    {
    s16 raw;
    int err;
//
// Voltage is measured in units of 4.88mV. The voltage is stored as
// a 10-bit number plus sign, in the upper bits of a 16-bit register
//
    err = ds278x_read_reg16(info, DS278x_REG_VOLT_MSB, &raw);
    if (err)
    return err;
// voltage_uV = (raw / 32) * 4800;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2782_get_capacity(info: *mut ds278x_info, capacity: *mut c_int) -> c_int {
    static int ds2782_get_capacity(struct ds278x_info *info, int *capacity)
    {
    int err;
    u8 raw;
    err = ds278x_read_reg(info, DS2782_REG_RARC, &raw);
    if (err)
    return err;
// capacity = raw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2786_get_current(info: *mut ds278x_info, current_uA: *mut c_int) -> c_int {
    static int ds2786_get_current(struct ds278x_info *info, int *current_uA)
    {
    int err;
    s16 raw;
    err = ds278x_read_reg16(info, DS278x_REG_CURRENT_MSB, &raw);
    if (err)
    return err;
// current_uA = (raw / 16) * (DS2786_CURRENT_UNITS / info->rsns);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2786_get_voltage(info: *mut ds278x_info, voltage_uV: *mut c_int) -> c_int {
    static int ds2786_get_voltage(struct ds278x_info *info, int *voltage_uV)
    {
    s16 raw;
    int err;
//
// Voltage is measured in units of 1.22mV. The voltage is stored as
// a 12-bit number plus sign, in the upper bits of a 16-bit register
//
    err = ds278x_read_reg16(info, DS278x_REG_VOLT_MSB, &raw);
    if (err)
    return err;
// voltage_uV = (raw / 8) * 1220;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2786_get_capacity(info: *mut ds278x_info, capacity: *mut c_int) -> c_int {
    static int ds2786_get_capacity(struct ds278x_info *info, int *capacity)
    {
    int err;
    u8 raw;
    err = ds278x_read_reg(info, DS2786_REG_RARC, &raw);
    if (err)
    return err;
// Relative capacity is displayed with resolution 0.5 %
// capacity = raw/2 ;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds278x_get_status(info: *mut ds278x_info, status: *mut c_int) -> c_int {
    static int ds278x_get_status(struct ds278x_info *info, int *status)
    {
    int err;
    int current_uA;
    int capacity;
    err = info.ops.get_battery_current(info, &current_uA);
    if (err)
    return err;
    err = info.ops.get_battery_capacity(info, &capacity);
    if (err)
    return err;
    info.capacity = capacity;
    if (capacity == 100)
// status = POWER_SUPPLY_STATUS_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(0: current_uA ==) -> else {
    else if (current_uA == 0)
// status = POWER_SUPPLY_STATUS_NOT_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(0: current_uA <) -> else {
    else if (current_uA < 0)
// status = POWER_SUPPLY_STATUS_DISCHARGING;
    else
// status = POWER_SUPPLY_STATUS_CHARGING;
    return 0;
    }
    static int ds278x_battery_get_property(struct power_supply *psy,
    enum power_supply_property prop,
    union power_supply_propval *val)
    {
    struct ds278x_info *info = to_ds278x_info(psy);
    int ret;
    switch (prop) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = ds278x_get_status(info, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = info.ops.get_battery_capacity(info, &val.intval);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = info.ops.get_battery_voltage(info, &val.intval);
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = info.ops.get_battery_current(info, &val.intval);
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = ds278x_get_temp(info, &val.intval);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ds278x_bat_update(info: *mut ds278x_info) {
    static void ds278x_bat_update(struct ds278x_info *info)
    {
    let mut old_status: c_int = info.status;
    let mut old_capacity: c_int = info.capacity;
    ds278x_get_status(info, &info.status);
    if ((old_status != info.status) || (old_capacity != info.capacity))
    power_supply_changed(info.battery);
    }
#[no_mangle]
unsafe extern "C" fn ds278x_bat_work(work: *mut work_struct) {
    static void ds278x_bat_work(struct work_struct *work)
    {
    struct ds278x_info *info;
    info = container_of(work, struct ds278x_info, bat_work.work);
    ds278x_bat_update(info);
    schedule_delayed_work(&info.bat_work, DS278x_DELAY);
    }
    static enum power_supply_property ds278x_battery_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn ds278x_power_supply_init(battery: *mut power_supply_desc) {
    static void ds278x_power_supply_init(struct power_supply_desc *battery)
    {
    battery.type			= POWER_SUPPLY_TYPE_BATTERY;
    battery.properties		= ds278x_battery_props;
    battery.num_properties		= ARRAY_SIZE(ds278x_battery_props);
    battery.get_property		= ds278x_battery_get_property;
    battery.external_power_changed	= core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn ds278x_suspend(dev: *mut device) -> c_int {
    static int ds278x_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct ds278x_info *info = i2c_get_clientdata(client);
    cancel_delayed_work(&info.bat_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds278x_resume(dev: *mut device) -> c_int {
    static int ds278x_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct ds278x_info *info = i2c_get_clientdata(client);
    schedule_delayed_work(&info.bat_work, DS278x_DELAY);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(ds278x_battery_pm_ops, ds278x_suspend, ds278x_resume);
    enum ds278x_num_id {
    DS2782 = 0,
    DS2786,
    };
    static const struct ds278x_battery_ops ds278x_ops[] = {
    [DS2782] = {
    .get_battery_current  = ds2782_get_current,
    .get_battery_voltage  = ds2782_get_voltage,
    .get_battery_capacity = ds2782_get_capacity,
    },
    [DS2786] = {
    .get_battery_current  = ds2786_get_current,
    .get_battery_voltage  = ds2786_get_voltage,
    .get_battery_capacity = ds2786_get_capacity,
    }
    };
#[no_mangle]
unsafe extern "C" fn ds278x_free_ida(data: *mut c_void) {
    static void ds278x_free_ida(void *data)
    {
    let mut num: c_int = (uintptr_t)data;
    ida_free(&battery_id, num);
    }
#[no_mangle]
unsafe extern "C" fn ds278x_battery_probe(client: *mut i2c_client) -> c_int {
    static int ds278x_battery_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct ds278x_platform_data *pdata = client.dev.platform_data;
    let mut psy_cfg: power_supply_config = {};
    struct ds278x_info *info;
    int ret;
    int num;
//
// ds2786 should have the sense resistor value set
// in the platform data
//
    if (id.driver_data == DS2786 && !pdata) {
    dev_err(&client.dev, "missing platform data for ds2786\n");
    return -EINVAL;
    }
// Get an ID for this battery
    num = ida_alloc(&battery_id, GFP_KERNEL);
    if (num < 0)
    return num;
    ret = devm_add_action_or_reset(&client.dev, ds278x_free_ida, (void *)(uintptr_t)num);
    if (ret)
    return ret;
    info = devm_kzalloc(&client.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.battery_desc.name = devm_kasprintf(&client.dev, GFP_KERNEL,
    "%s-%d", client.name, num);
    if (!info.battery_desc.name)
    return -ENOMEM;
    if (id.driver_data == DS2786)
    info.rsns = pdata.rsns;
    i2c_set_clientdata(client, info);
    info.client = client;
    info.ops  = &ds278x_ops[id.driver_data];
    ds278x_power_supply_init(&info.battery_desc);
    psy_cfg.drv_data = info;
    info.capacity = 100;
    info.status = POWER_SUPPLY_STATUS_FULL;
    info.battery = devm_power_supply_register(&client.dev,
    &info.battery_desc,
    &psy_cfg);
    if (IS_ERR(info.battery)) {
    dev_err(&client.dev, "failed to register battery\n");
    return PTR_ERR(info.battery);
    }
    ret = devm_delayed_work_autocancel(&client.dev, &info.bat_work, ds278x_bat_work);
    if (ret)
    return ret;
    schedule_delayed_work(&info.bat_work, DS278x_DELAY);
    return 0;
    }
    static const struct i2c_device_id ds278x_id[] = {
    { .name = "ds2782", .driver_data = DS2782 },
    { .name = "ds2786", .driver_data = DS2786 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ds278x_id);
    static struct i2c_driver ds278x_battery_driver = {
    .driver 	= {
    .name	= "ds2782-battery",
    .pm	= &ds278x_battery_pm_ops,
    },
    .probe		= ds278x_battery_probe,
    .id_table	= ds278x_id,
    };
    module_i2c_driver(ds278x_battery_driver);
    MODULE_AUTHOR("Ryan Mallon");
    MODULE_DESCRIPTION("Maxim/Dallas DS2782 Stand-Alone Fuel Gauge IC driver");
    MODULE_LICENSE("GPL");
