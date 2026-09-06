//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ir38064.c
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
// Hardware monitoring driver for Infineon IR38064
//
// Copyright (c) 2017 Google Inc
//
// VOUT_MODE is not supported by the device. The driver fakes VOUT linear16
// mode with exponent value -8 as direct mode with m=256/b=0/R=0.
//
// The device supports VOUT_PEAK, IOUT_PEAK, and TEMPERATURE_PEAK, however
// this driver does not currently support them.
//

    static const struct regulator_desc ir38064_reg_desc[] = {
    PMBUS_REGULATOR_ONE("vout"),
    };

    static struct pmbus_driver_info ir38064_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .m[PSC_VOLTAGE_OUT] = 256,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = 0,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT,

    .num_regulators = 1,
    .reg_desc = ir38064_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn ir38064_probe(client: *mut i2c_client) -> c_int {
    static int ir38064_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &ir38064_info);
    }
    static const struct i2c_device_id ir38064_id[] = {
    { .name = "ir38060" },
    { .name = "ir38064" },
    { .name = "ir38164" },
    { .name = "ir38263" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ir38064_id);
    static const struct of_device_id __maybe_unused ir38064_of_match[] = {
    { .compatible = "infineon,ir38060" },
    { .compatible = "infineon,ir38064" },
    { .compatible = "infineon,ir38164" },
    { .compatible = "infineon,ir38263" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ir38064_of_match);
// This is the driver that will be inserted
    static struct i2c_driver ir38064_driver = {
    .driver = {
    .name = "ir38064",
    .of_match_table = of_match_ptr(ir38064_of_match),
    },
    .probe = ir38064_probe,
    .id_table = ir38064_id,
    };
    module_i2c_driver(ir38064_driver);
    MODULE_AUTHOR("Maxim Sloyko <maxims@google.com>");
    MODULE_DESCRIPTION("PMBus driver for Infineon IR38064 and compatible chips");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
