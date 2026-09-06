//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/tps25990.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2024 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

pub const TPS25990_READ_VAUX: c_uint = 0xd0;
pub const TPS25990_READ_VIN_MIN: c_uint = 0xd1;
pub const TPS25990_READ_VIN_PEAK: c_uint = 0xd2;
pub const TPS25990_READ_IIN_PEAK: c_uint = 0xd4;
pub const TPS25990_READ_PIN_PEAK: c_uint = 0xd5;
pub const TPS25990_READ_TEMP_AVG: c_uint = 0xd6;
pub const TPS25990_READ_TEMP_PEAK: c_uint = 0xd7;
pub const TPS25990_READ_VOUT_MIN: c_uint = 0xda;
pub const TPS25990_READ_VIN_AVG: c_uint = 0xdc;
pub const TPS25990_READ_VOUT_AVG: c_uint = 0xdd;
pub const TPS25990_READ_IIN_AVG: c_uint = 0xde;
pub const TPS25990_READ_PIN_AVG: c_uint = 0xdf;
pub const TPS25990_VIREF: c_uint = 0xe0;
pub const TPS25990_PK_MIN_AVG: c_uint = 0xea;

pub const TPS25990_MFR_WRITE_PROTECT: c_uint = 0xf8;

pub const TPS25990_8B_SHIFT: c_int = 2;
pub const TPS25990_VIN_OVF_NUM: c_int = 525100;
pub const TPS25990_VIN_OVF_DIV: c_int = 10163;
pub const TPS25990_VIN_OVF_OFF: c_int = 155;
pub const TPS25990_IIN_OCF_NUM: c_int = 953800;
pub const TPS25990_IIN_OCF_DIV: c_int = 129278;
pub const TPS25990_IIN_OCF_OFF: c_int = 157;

    PK_MIN_AVG_RST_AVG  | \
    PK_MIN_AVG_RST_MIN)
//
// Arbitrary default Rimon value: 1kOhm
// This correspond to an overcurrent limit of 55A, close to the specified limit
// of un-stacked TPS25990 and makes further calculation easier to setup in
// sensor.conf, if necessary
//
pub const TPS25990_DEFAULT_RIMON: c_int = 1000000000;
#[no_mangle]
unsafe extern "C" fn tps25990_set_m(m: *mut c_int, rimon: u32) {
    static void tps25990_set_m(int *m, u32 rimon)
    {
    let mut val: u64 = ((u64)*m) * rimon;
// Make sure m fits the s32 type
// m = DIV_ROUND_CLOSEST_ULL(val, 1000000);
    }
    static int tps25990_mfr_write_protect_set(struct i2c_client *client,
    u8 protect)
    {
    u8 val;
    switch (protect) {
    case 0:
    val = 0xa2;
    break;
    case PB_WP_ALL:
    val = 0x0;
    break;
    default:
    return -EINVAL;
    }
    return pmbus_write_byte_data(client, -1, TPS25990_MFR_WRITE_PROTECT,
    val);
    }
#[no_mangle]
unsafe extern "C" fn tps25990_mfr_write_protect_get(client: *mut i2c_client) -> c_int {
    static int tps25990_mfr_write_protect_get(struct i2c_client *client)
    {
    let mut ret: c_int = pmbus_read_byte_data(client, -1, TPS25990_MFR_WRITE_PROTECT);
    if (ret < 0)
    return ret;
    return (ret & TPS25990_UNLOCKED) ? 0 : PB_WP_ALL;
    }
    static int tps25990_read_word_data(struct i2c_client *client,
    int page, int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VIN_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VIN_MIN);
    break;
    case PMBUS_VIRT_READ_VIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VIN_AVG);
    break;
    case PMBUS_VIRT_READ_VOUT_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VOUT_MIN);
    break;
    case PMBUS_VIRT_READ_VOUT_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VOUT_AVG);
    break;
    case PMBUS_VIRT_READ_IIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_IIN_AVG);
    break;
    case PMBUS_VIRT_READ_IIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_IIN_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_TEMP_AVG);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_TEMP_PEAK);
    break;
    case PMBUS_VIRT_READ_PIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_PIN_AVG);
    break;
    case PMBUS_VIRT_READ_PIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_PIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VMON:
    ret = pmbus_read_word_data(client, page, phase,
    TPS25990_READ_VAUX);
    break;
    case PMBUS_VIN_UV_WARN_LIMIT:
    case PMBUS_VIN_UV_FAULT_LIMIT:
    case PMBUS_VIN_OV_WARN_LIMIT:
    case PMBUS_VOUT_UV_WARN_LIMIT:
    case PMBUS_IIN_OC_WARN_LIMIT:
    case PMBUS_OT_WARN_LIMIT:
    case PMBUS_OT_FAULT_LIMIT:
    case PMBUS_PIN_OP_WARN_LIMIT:
//
// These registers provide an 8 bits value instead of a
// 10bits one. Just shifting twice the register value is
// enough to make the sensor type conversion work, even
// if the datasheet provides different m, b and R for
// those.
//
    ret = pmbus_read_word_data(client, page, phase, reg);
    if (ret < 0)
    break;
    ret <<= TPS25990_8B_SHIFT;
    break;
    case PMBUS_VIN_OV_FAULT_LIMIT:
    ret = pmbus_read_word_data(client, page, phase, reg);
    if (ret < 0)
    break;
    ret = DIV_ROUND_CLOSEST(ret * TPS25990_VIN_OVF_NUM,
    TPS25990_VIN_OVF_DIV);
    ret += TPS25990_VIN_OVF_OFF;
    break;
    case PMBUS_IIN_OC_FAULT_LIMIT:
//
// VIREF directly sets the over-current limit at which the eFuse
// will turn the FET off and trigger a fault. Expose it through
// this generic property instead of a manufacturer specific one.
//
    ret = pmbus_read_byte_data(client, page, TPS25990_VIREF);
    if (ret < 0)
    break;
    ret = DIV_ROUND_CLOSEST(ret * TPS25990_IIN_OCF_NUM,
    TPS25990_IIN_OCF_DIV);
    ret += TPS25990_IIN_OCF_OFF;
    break;
    case PMBUS_VIRT_SAMPLES:
    ret = pmbus_read_byte_data(client, page, TPS25990_PK_MIN_AVG);
    if (ret < 0)
    break;
    ret = 1 << FIELD_GET(PK_MIN_AVG_AVG_CNT, ret);
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    ret = 0;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int tps25990_write_word_data(struct i2c_client *client,
    int page, int reg, u16 value)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIN_UV_WARN_LIMIT:
    case PMBUS_VIN_UV_FAULT_LIMIT:
    case PMBUS_VIN_OV_WARN_LIMIT:
    case PMBUS_VOUT_UV_WARN_LIMIT:
    case PMBUS_IIN_OC_WARN_LIMIT:
    case PMBUS_OT_WARN_LIMIT:
    case PMBUS_OT_FAULT_LIMIT:
    case PMBUS_PIN_OP_WARN_LIMIT:
    value >>= TPS25990_8B_SHIFT;
    value = clamp_val(value, 0, 0xff);
    ret = pmbus_write_word_data(client, page, reg, value);
    break;
    case PMBUS_VIN_OV_FAULT_LIMIT:
    value -= TPS25990_VIN_OVF_OFF;
    value = DIV_ROUND_CLOSEST(((unsigned int)value) * TPS25990_VIN_OVF_DIV,
    TPS25990_VIN_OVF_NUM);
    value = clamp_val(value, 0, 0xf);
    ret = pmbus_write_word_data(client, page, reg, value);
    break;
    case PMBUS_IIN_OC_FAULT_LIMIT:
    value -= TPS25990_IIN_OCF_OFF;
    value = DIV_ROUND_CLOSEST(((unsigned int)value) * TPS25990_IIN_OCF_DIV,
    TPS25990_IIN_OCF_NUM);
    value = clamp_val(value, 0, 0x3f);
    ret = pmbus_write_byte_data(client, page, TPS25990_VIREF, value);
    break;
    case PMBUS_VIRT_SAMPLES:
    value = clamp_val(value, 1, 1 << PK_MIN_AVG_AVG_CNT);
    value = ilog2(value);
    ret = pmbus_update_byte_data(client, page, TPS25990_PK_MIN_AVG,
    PK_MIN_AVG_AVG_CNT,
    FIELD_PREP(PK_MIN_AVG_AVG_CNT, value));
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
//
// TPS25990 has history resets based on MIN/AVG/PEAK instead of per
// sensor type. Exposing this quirk in hwmon is not desirable so
// reset MIN, AVG and PEAK together. Even is there effectively only
// one reset, which resets everything, expose the 5 entries so
// userspace is not required map a sensor type to another to trigger
// a reset
//
    ret = pmbus_update_byte_data(client, 0, TPS25990_PK_MIN_AVG,
    PK_MIN_AVG_RST_MASK,
    PK_MIN_AVG_RST_MASK);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int tps25990_read_byte_data(struct i2c_client *client,
    int page, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_WRITE_PROTECT:
    ret = tps25990_mfr_write_protect_get(client);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int tps25990_write_byte_data(struct i2c_client *client,
    int page, int reg, u8 byte)
    {
    int ret;
    switch (reg) {
    case PMBUS_WRITE_PROTECT:
    ret = tps25990_mfr_write_protect_set(client, byte);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }

    static const struct regulator_desc tps25990_reg_desc[] = {
    PMBUS_REGULATOR_ONE_NODE("vout"),
    };

    static const struct pmbus_driver_info tps25990_base_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 5251,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -2,
    .format[PSC_VOLTAGE_OUT] = direct,
    .m[PSC_VOLTAGE_OUT] = 5251,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = -2,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 140,
    .b[PSC_TEMPERATURE] = 32100,
    .R[PSC_TEMPERATURE] = -2,
//
// Current and Power measurement depends on the ohm value
// of Rimon. m is multiplied by 1000 below to have an integer
// and -3 is added to R to compensate.
//
    .format[PSC_CURRENT_IN] = direct,
    .m[PSC_CURRENT_IN] = 9538,
    .b[PSC_CURRENT_IN] = 0,
    .R[PSC_CURRENT_IN] = -6,
    .format[PSC_POWER] = direct,
    .m[PSC_POWER] = 4901,
    .b[PSC_POWER] = 0,
    .R[PSC_POWER] = -7,
    .func[0] = (PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT |
    PMBUS_HAVE_VMON |
    PMBUS_HAVE_IIN |
    PMBUS_HAVE_PIN |
    PMBUS_HAVE_TEMP |
    PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_SAMPLES),
    .read_word_data = tps25990_read_word_data,
    .write_word_data = tps25990_write_word_data,
    .read_byte_data = tps25990_read_byte_data,
    .write_byte_data = tps25990_write_byte_data,

    .reg_desc = tps25990_reg_desc,
    .num_regulators = ARRAY_SIZE(tps25990_reg_desc),

    };
    static const struct i2c_device_id tps25990_i2c_id[] = {
    { .name = "tps25990" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps25990_i2c_id);
    static const struct of_device_id tps25990_of_match[] = {
    { .compatible = "ti,tps25990" },
    {}
    };
    MODULE_DEVICE_TABLE(of, tps25990_of_match);
#[no_mangle]
unsafe extern "C" fn tps25990_probe(client: *mut i2c_client) -> c_int {
    static int tps25990_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct pmbus_driver_info *info;
    const char *propname;
    u32 rimon;
    int ret;
    propname = "ti,rimon-micro-ohms";
    if (device_property_present(dev, propname)) {
    ret = device_property_read_u32(dev, propname, &rimon);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get %s\n", propname);
    } else {
    rimon = TPS25990_DEFAULT_RIMON;
    }
    info = devm_kmemdup(dev, &tps25990_base_info, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
// Adapt the current and power scale for each instance
    tps25990_set_m(&info.m[PSC_CURRENT_IN], rimon);
    tps25990_set_m(&info.m[PSC_POWER], rimon);
    return pmbus_do_probe(client, info);
    }
    static struct i2c_driver tps25990_driver = {
    .driver = {
    .name = "tps25990",
    .of_match_table = tps25990_of_match,
    },
    .probe = tps25990_probe,
    .id_table = tps25990_i2c_id,
    };
    module_i2c_driver(tps25990_driver);
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_DESCRIPTION("PMBUS driver for TPS25990 eFuse");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
