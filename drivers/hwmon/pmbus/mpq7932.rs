//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/mpq7932.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// mpq7932.c - hwmon with optional regulator driver for mps mpq7932
// Copyright 2022 Monolithic Power Systems, Inc
//
// Author: Saravanan Sekar <saravanan@linumiz.com>
//

pub const MPQ7932_BUCK_UV_MIN: c_int = 206250;
pub const MPQ7932_UV_STEP: c_int = 6250;
pub const MPQ7932_N_VOLTAGES: c_int = 256;
pub const MPQ7932_VOUT_MAX: c_uint = 0xFF;
pub const MPQ7932_NUM_PAGES: c_int = 6;
pub const MPQ2286_NUM_PAGES: c_int = 1;
pub const MPQ7932_TON_DELAY: c_uint = 0x60;
pub const MPQ7932_VOUT_STARTUP_SLEW: c_uint = 0xA3;
pub const MPQ7932_VOUT_SHUTDOWN_SLEW: c_uint = 0xA5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpq7932_data {
    pub info: pmbus_driver_info,
    pub pdata: pmbus_platform_data,
}

    static const struct regulator_desc mpq7932_regulators_desc[] = {
    PMBUS_REGULATOR_STEP("buck", 0, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    PMBUS_REGULATOR_STEP("buck", 1, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    PMBUS_REGULATOR_STEP("buck", 2, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    PMBUS_REGULATOR_STEP("buck", 3, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    PMBUS_REGULATOR_STEP("buck", 4, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    PMBUS_REGULATOR_STEP("buck", 5, MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    };
    static const struct regulator_desc mpq7932_regulators_desc_one[] = {
    PMBUS_REGULATOR_STEP_ONE_NODE("buck", MPQ7932_N_VOLTAGES,
    MPQ7932_UV_STEP, MPQ7932_BUCK_UV_MIN),
    };

    static int mpq7932_write_word_data(struct i2c_client *client, int page, int reg,
    u16 word)
    {
    switch (reg) {
//
// chip supports only byte access for VOUT_COMMAND otherwise
// access results -EREMOTEIO
//
    case PMBUS_VOUT_COMMAND:
    return pmbus_write_byte_data(client, page, reg, word & 0xFF);
    default:
    return -ENODATA;
    }
    }
    static int mpq7932_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    switch (reg) {
//
// chip supports neither (PMBUS_VOUT_MARGIN_HIGH, PMBUS_VOUT_MARGIN_LOW)
// nor (PMBUS_MFR_VOUT_MIN, PMBUS_MFR_VOUT_MAX). As a result set voltage
// fails due to error in pmbus_regulator_get_low_margin, so faked.
//
    case PMBUS_MFR_VOUT_MIN:
    return 0;
    case PMBUS_MFR_VOUT_MAX:
    return MPQ7932_VOUT_MAX;
//
// chip supports only byte access for VOUT_COMMAND otherwise
// access results in -EREMOTEIO
//
    case PMBUS_READ_VOUT:
    return pmbus_read_byte_data(client, page, PMBUS_VOUT_COMMAND);
    default:
    return -ENODATA;
    }
    }
#[no_mangle]
unsafe extern "C" fn mpq7932_probe(client: *mut i2c_client) -> c_int {
    static int mpq7932_probe(struct i2c_client *client)
    {
    struct mpq7932_data *data;
    struct pmbus_driver_info *info;
    struct device *dev = &client.dev;
    int i;
    data = devm_kzalloc(dev, sizeof(struct mpq7932_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    info = &data.info;
    info.pages = (int)(unsigned long)device_get_match_data(&client.dev);
    info.format[PSC_VOLTAGE_OUT] = direct;
    info.m[PSC_VOLTAGE_OUT] = 160;
    info.b[PSC_VOLTAGE_OUT] = -33;
    for (i = 0; i < info.pages; i++) {
    info.func[i] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_STATUS_TEMP;
    }

    info.num_regulators = info.pages;
    if (info.num_regulators == 1)
    info.reg_desc = mpq7932_regulators_desc_one;
    else
    info.reg_desc = mpq7932_regulators_desc;

    info.read_word_data = mpq7932_read_word_data;
    info.write_word_data = mpq7932_write_word_data;
    data.pdata.flags = PMBUS_NO_CAPABILITY;
    dev.platform_data = &data.pdata;
    return pmbus_do_probe(client, info);
    }
    static const struct of_device_id mpq7932_of_match[] = {
    { .compatible = "mps,mpq2286", .data = (void *)MPQ2286_NUM_PAGES },
    { .compatible = "mps,mpq7932", .data = (void *)MPQ7932_NUM_PAGES },
    {},
    };
    MODULE_DEVICE_TABLE(of, mpq7932_of_match);
    static const struct i2c_device_id mpq7932_id[] = {
    { .name = "mpq2286" },
    { .name = "mpq7932" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mpq7932_id);
    static struct i2c_driver mpq7932_regulator_driver = {
    .driver = {
    .name = "mpq7932",
    .of_match_table = mpq7932_of_match,
    },
    .probe = mpq7932_probe,
    .id_table = mpq7932_id,
    };
    module_i2c_driver(mpq7932_regulator_driver);
    MODULE_AUTHOR("Saravanan Sekar <saravanan@linumiz.com>");
    MODULE_DESCRIPTION("MPQ7932 PMIC regulator driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
