//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/xdpe12284.c
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
// Hardware monitoring driver for Infineon Multi-phase Digital VR Controllers
//
// Copyright (c) 2020 Mellanox Technologies. All rights reserved.
//

pub const XDPE122_PROT_VR12_5MV: c_uint = 0x01 /* VR12.0 mode, 5-mV DAC */;
pub const XDPE122_PROT_VR12_5_10MV: c_uint = 0x02 /* VR12.5 mode, 10-mV DAC */;
pub const XDPE122_PROT_IMVP9_10MV: c_uint = 0x03 /* IMVP9 mode, 10-mV DAC */;
pub const XDPE122_AMD_625MV: c_uint = 0x10 /* AMD mode 6.25mV */;
pub const XDPE122_PAGE_NUM: c_int = 2;
    static int xdpe122_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    long val;
    s16 exponent;
    s32 mantissa;
    int ret;
    switch (reg) {
    case PMBUS_VOUT_OV_FAULT_LIMIT:
    case PMBUS_VOUT_UV_FAULT_LIMIT:
    ret = pmbus_read_word_data(client, page, phase, reg);
    if (ret < 0)
    return ret;
// Convert register value to LINEAR11 data.
    exponent = ((s16)ret) >> 11;
    mantissa = ((s16)((ret & GENMASK(10, 0)) << 5)) >> 5;
    val = mantissa * 1000L;
    if (exponent >= 0)
    val <<= exponent;
    else
    val >>= -exponent;
// Convert data to VID register.
    switch (info.vrm_version[page]) {
    case vr13:
    if (val >= 500)
    return 1 + DIV_ROUND_CLOSEST(val - 500, 10);
    return 0;
    case vr12:
    if (val >= 250)
    return 1 + DIV_ROUND_CLOSEST(val - 250, 5);
    return 0;
    case imvp9:
    if (val >= 200)
    return 1 + DIV_ROUND_CLOSEST(val - 200, 10);
    return 0;
    case amd625mv:
    if (val >= 200 && val <= 1550)
    return DIV_ROUND_CLOSEST((1550 - val) * 100,
    625);
    return 0;
    default:
    return -EINVAL;
    }
    default:
    return -ENODATA;
    }
    return 0;
    }
    static int xdpe122_identify(struct i2c_client *client,
    struct pmbus_driver_info *info)
    {
    u8 vout_params;
    int i, ret, vout_mode;
    vout_mode = pmbus_read_byte_data(client, 0, PMBUS_VOUT_MODE);
    if (vout_mode >= 0 && vout_mode != 0xff) {
    switch (vout_mode >> 5) {
    case 0:
    info.format[PSC_VOLTAGE_OUT] = linear;
    return 0;
    case 1:
    info.format[PSC_VOLTAGE_OUT] = vid;
    info.read_word_data = xdpe122_read_word_data;
    break;
    default:
    return -ENODEV;
    }
    }
    for (i = 0; i < XDPE122_PAGE_NUM; i++) {
// Read the register with VOUT scaling value.
    ret = pmbus_read_byte_data(client, i, PMBUS_VOUT_MODE);
    if (ret < 0)
    return ret;
    vout_params = ret & GENMASK(4, 0);
    switch (vout_params) {
    case XDPE122_PROT_VR12_5_10MV:
    info.vrm_version[i] = vr13;
    break;
    case XDPE122_PROT_VR12_5MV:
    info.vrm_version[i] = vr12;
    break;
    case XDPE122_PROT_IMVP9_10MV:
    info.vrm_version[i] = imvp9;
    break;
    case XDPE122_AMD_625MV:
    info.vrm_version[i] = amd625mv;
    break;
    default:
    return -EINVAL;
    }
    }
    return 0;
    }
    static const struct regulator_desc __maybe_unused xdpe122_reg_desc[] = {
    PMBUS_REGULATOR("vout", 0),
    PMBUS_REGULATOR("vout", 1),
    };
    static struct pmbus_driver_info xdpe122_info = {
    .pages = XDPE122_PAGE_NUM,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_POUT | PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT,
    .func[1] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_POUT | PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT,
    .identify = xdpe122_identify,

    .num_regulators = 2,
    .reg_desc = xdpe122_reg_desc,

    };
#[no_mangle]
unsafe extern "C" fn xdpe122_probe(client: *mut i2c_client) -> c_int {
    static int xdpe122_probe(struct i2c_client *client)
    {
    struct pmbus_driver_info *info;
    info = devm_kmemdup(&client.dev, &xdpe122_info, sizeof(*info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id xdpe122_id[] = {
    { .name = "xdpe11280" },
    { .name = "xdpe12254" },
    { .name = "xdpe12284" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, xdpe122_id);
    static const struct of_device_id __maybe_unused xdpe122_of_match[] = {
    {.compatible = "infineon,xdpe11280"},
    {.compatible = "infineon,xdpe12254"},
    {.compatible = "infineon,xdpe12284"},
    {}
    };
    MODULE_DEVICE_TABLE(of, xdpe122_of_match);
    static struct i2c_driver xdpe122_driver = {
    .driver = {
    .name = "xdpe12284",
    .of_match_table = of_match_ptr(xdpe122_of_match),
    },
    .probe = xdpe122_probe,
    .id_table = xdpe122_id,
    };
    module_i2c_driver(xdpe122_driver);
    MODULE_AUTHOR("Vadim Pasternak <vadimp@mellanox.com>");
    MODULE_DESCRIPTION("PMBus driver for Infineon XDPE122 family");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
