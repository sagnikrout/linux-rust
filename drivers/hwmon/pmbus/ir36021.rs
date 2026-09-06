//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ir36021.c
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
// Hardware monitoring driver for Infineon IR36021
//
// Copyright (c) 2021 Allied Telesis
//

    static struct pmbus_driver_info ir36021_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT
    | PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT
    | PMBUS_HAVE_PIN | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2
    | PMBUS_HAVE_STATUS_TEMP,
    };
#[no_mangle]
unsafe extern "C" fn ir36021_probe(client: *mut i2c_client) -> c_int {
    static int ir36021_probe(struct i2c_client *client)
    {
    u8 buf[I2C_SMBUS_BLOCK_MAX];
    int ret;
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read PMBUS_MFR_MODEL\n");
    if (ret != 1 || buf[0] != 0x2d)
    return dev_err_probe(&client.dev, -ENODEV,
    "MFR_MODEL unrecognised\n");
    return pmbus_do_probe(client, &ir36021_info);
    }
    static const struct i2c_device_id ir36021_id[] = {
    { .name = "ir36021" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ir36021_id);
    static const struct of_device_id __maybe_unused ir36021_of_id[] = {
    { .compatible = "infineon,ir36021" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ir36021_of_id);
    static struct i2c_driver ir36021_driver = {
    .driver = {
    .name = "ir36021",
    .of_match_table = of_match_ptr(ir36021_of_id),
    },
    .probe = ir36021_probe,
    .id_table = ir36021_id,
    };
    module_i2c_driver(ir36021_driver);
    MODULE_AUTHOR("Chris Packham <chris.packham@alliedtelesis.co.nz>");
    MODULE_DESCRIPTION("PMBus driver for Infineon IR36021");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
