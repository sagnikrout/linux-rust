//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/irps5401.c
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
// Hardware monitoring driver for the Infineon IRPS5401M PMIC.
//
// Copyright (c) 2019 SED Systems, a division of Calian Ltd.
//
// The device supports VOUT_PEAK, IOUT_PEAK, and TEMPERATURE_PEAK, however
// this driver does not currently support them.
//

    PMBUS_HAVE_STATUS_INPUT | \
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT | \
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT | \
    PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | \
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP)

    PMBUS_HAVE_STATUS_INPUT | \
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT | \
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT | \
    PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | \
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP)
    static struct pmbus_driver_info irps5401_info = {
    .pages = 5,
    .func[0] = IRPS5401_SW_FUNC,
    .func[1] = IRPS5401_SW_FUNC,
    .func[2] = IRPS5401_SW_FUNC,
    .func[3] = IRPS5401_SW_FUNC,
    .func[4] = IRPS5401_LDO_FUNC,
    };
#[no_mangle]
unsafe extern "C" fn irps5401_probe(client: *mut i2c_client) -> c_int {
    static int irps5401_probe(struct i2c_client *client)
    {
    return pmbus_do_probe(client, &irps5401_info);
    }
    static const struct i2c_device_id irps5401_id[] = {
    { .name = "irps5401" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, irps5401_id);
    static struct i2c_driver irps5401_driver = {
    .driver = {
    .name = "irps5401",
    },
    .probe = irps5401_probe,
    .id_table = irps5401_id,
    };
    module_i2c_driver(irps5401_driver);
    MODULE_AUTHOR("Robert Hancock");
    MODULE_DESCRIPTION("PMBus driver for Infineon IRPS5401");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
