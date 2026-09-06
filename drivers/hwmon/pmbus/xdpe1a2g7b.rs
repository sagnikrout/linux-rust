//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/xdpe1a2g7b.c
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
// Hardware monitoring driver for Infineon Multi-phase Digital XDPE1A2G5B
// and XDPE1A2G7B Controllers
//
// Copyright (c) 2026 Infineon Technologies. All rights reserved.
//

pub const XDPE1A2G7B_PAGE_NUM: c_int = 2;
pub const XDPE1A2G7B_NVIDIA_195MV: c_uint = 0x1E /* NVIDIA mode 1.95mV, VID step is 5mV */;
    static int xdpe1a2g7b_identify(struct i2c_client *client,
    struct pmbus_driver_info *info)
    {
    u8 vout_params;
    int vout_mode;
//
// XDPE1A2G5B and XDPE1A2G7B support both Linear and NVIDIA PWM VID data
// formats via VOUT_MODE. Note that the device pages/loops are not fully
// independent: configuration is shared, so programming each page/loop
// separately is not supported.
//
    vout_mode = pmbus_read_byte_data(client, 0, PMBUS_VOUT_MODE);
    if (vout_mode < 0)
    return vout_mode;
    switch (vout_mode >> 5) {
    case 0:
    info.format[PSC_VOLTAGE_OUT] = linear;
    return 0;
    case 1:
    info.format[PSC_VOLTAGE_OUT] = vid;
    vout_params = vout_mode & GENMASK(4, 0);
// Check for VID Code Type
    switch (vout_params) {
    case XDPE1A2G7B_NVIDIA_195MV:
// VID vrm_version for PAGE0 and PAGE1
    info.vrm_version[0] = nvidia195mv;
    info.vrm_version[1] = nvidia195mv;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }

    static const struct regulator_desc xdpe1a2g7b_reg_desc[] = {
    PMBUS_REGULATOR("vout", 0),
    PMBUS_REGULATOR("vout", 1),
    };

    static struct pmbus_driver_info xdpe1a2g7b_info = {
    .pages = XDPE1A2G7B_PAGE_NUM,
    .identify = xdpe1a2g7b_identify,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_POUT | PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT,
    .func[1] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | PMBUS_HAVE_STATUS_INPUT,

    .num_regulators = XDPE1A2G7B_PAGE_NUM,
    .reg_desc = xdpe1a2g7b_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn xdpe1a2g7b_probe(client: *mut i2c_client) -> c_int {
    static int xdpe1a2g7b_probe(struct i2c_client *client)
    {
    struct pmbus_driver_info *info;
    info = devm_kmemdup(&client.dev, &xdpe1a2g7b_info, sizeof(*info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id xdpe1a2g7b_id[] = {
    { .name = "xdpe1a2g5b" },
    { .name = "xdpe1a2g7b" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, xdpe1a2g7b_id);
    static const struct of_device_id __maybe_unused xdpe1a2g7b_of_match[] = {
    { .compatible = "infineon,xdpe1a2g5b" },
    { .compatible = "infineon,xdpe1a2g7b" },
    {}
    };
    MODULE_DEVICE_TABLE(of, xdpe1a2g7b_of_match);
    static struct i2c_driver xdpe1a2g7b_driver = {
    .driver = {
    .name = "xdpe1a2g7b",
    .of_match_table = of_match_ptr(xdpe1a2g7b_of_match),
    },
    .probe = xdpe1a2g7b_probe,
    .id_table = xdpe1a2g7b_id,
    };
    module_i2c_driver(xdpe1a2g7b_driver);
    MODULE_AUTHOR("Ashish Yadav <ashish.yadav@infineon.com>");
    MODULE_DESCRIPTION("PMBus driver for Infineon XDPE1A2G5B/7B");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
