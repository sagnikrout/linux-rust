//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/max15301.c
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
// Hardware monitoring driver for Maxim MAX15301
//
// Copyright (c) 2021 Flextronics International Sweden AB
//
// Even though the specification does not specifically mention it,
// extensive empirical testing has revealed that auto-detection of
// limit-registers will fail in a random fashion unless the delay
// parameter is set to above about 80us. The default delay is set
// to 100us to include some safety margin.
//

    static const struct i2c_device_id max15301_id[] = {
    { .name = "bmr461" },
    { .name = "max15301" },
    { .name = "max15303" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max15301_id);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max15301_data {
    pub id: c_int,
    pub info: pmbus_driver_info,
}

    let mut delay: static ushort = MAX15301_WAIT_TIME;
    module_param(delay, ushort, 0644);
    MODULE_PARM_DESC(delay, "Delay between chip accesses in us");
    static struct max15301_data max15301_data = {
    .info = {
    .pages = 1,
    .func[0] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2
    | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT,
    }
    };
#[no_mangle]
unsafe extern "C" fn max15301_probe(client: *mut i2c_client) -> c_int {
    static int max15301_probe(struct i2c_client *client)
    {
    int status;
    u8 device_id[I2C_SMBUS_BLOCK_MAX + 1];
    const struct i2c_device_id *mid;
    struct pmbus_driver_info *info = &max15301_data.info;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA
    | I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    status = i2c_smbus_read_block_data(client, PMBUS_IC_DEVICE_ID, device_id);
    if (status < 0) {
    dev_err(&client.dev, "Failed to read Device Id\n");
    return status;
    }
    for (mid = max15301_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, device_id, strlen(mid.name)))
    break;
    }
    if (!mid.name[0]) {
    dev_err(&client.dev, "Unsupported device\n");
    return -ENODEV;
    }
    info.access_delay = delay;
    return pmbus_do_probe(client, info);
    }
    static struct i2c_driver max15301_driver = {
    .driver = {
    .name = "max15301",
    },
    .probe = max15301_probe,
    .id_table = max15301_id,
    };
    module_i2c_driver(max15301_driver);
    MODULE_AUTHOR("Erik Rosen <erik.rosen@metormote.com>");
    MODULE_DESCRIPTION("PMBus driver for Maxim MAX15301");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
