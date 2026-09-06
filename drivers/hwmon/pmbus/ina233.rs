//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ina233.c
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
// Hardware monitoring driver for ina233
//
// Copyright (c) 2025 Leo Yang
//

pub const MFR_READ_VSHUNT: c_uint = 0xd1;
pub const MFR_CALIBRATION: c_uint = 0xd4;

pub const MAX_M_VAL: c_int = 32767;
#[no_mangle]
unsafe extern "C" fn calculate_coef(m: *mut c_int, R: *mut c_int, current_lsb: u32, power_coef: c_int) {
    static void calculate_coef(int *m, int *R, u32 current_lsb, int power_coef)
    {
    u64 scaled_m;
    let mut scale_factor: c_int = 0;
    let mut scale_coef: c_int = 1;
//
// 1000000 from Current_LSB A->uA .
// scale_coef is for scaling up to minimize rounding errors,
// If there is no decimal information, no need to scale.
//
    if (1000000 % current_lsb) {
// Scaling to keep integer precision
    scale_factor = -3;
    scale_coef = 1000;
    }
//
// Unit Conversion (Current_LSB A->uA) and use scaling(scale_factor)
// to keep integer precision.
// Formulae referenced from spec.
//
    scaled_m = div64_u64(1000000 * scale_coef, (u64)current_lsb * power_coef);
// Maximize while keeping it bounded.
    while (scaled_m > MAX_M_VAL) {
    scaled_m = div_u64(scaled_m, 10);
    scale_factor++;
    }
// Scale up only if fractional part exists.
    while (scaled_m * 10 < MAX_M_VAL && scale_coef != 1) {
    scaled_m *= 10;
    scale_factor--;
    }
// m = scaled_m;
// R = scale_factor;
    }
    static int ina233_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VMON:
    ret = pmbus_read_word_data(client, 0, 0xff, MFR_READ_VSHUNT);
    if (ret < 0)
    return ret;
// Adjust returned value to match VIN coefficients
// VIN: 1.25 mV VSHUNT: 2.5 uV LSB
    ret = clamp_val(DIV_ROUND_CLOSEST((s16)ret * 25, 12500),
    S16_MIN, S16_MAX) & 0xffff;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ina233_probe(client: *mut i2c_client) -> c_int {
    static int ina233_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    const char *propname;
    int ret, m, R;
    u32 rshunt;
    u32 max_current;
    u32 current_lsb;
    u16 calibration;
    struct pmbus_driver_info *info;
    info = devm_kzalloc(dev, sizeof(struct pmbus_driver_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.pages = 1;
    info.format[PSC_VOLTAGE_IN] = direct;
    info.format[PSC_VOLTAGE_OUT] = direct;
    info.format[PSC_CURRENT_OUT] = direct;
    info.format[PSC_POWER] = direct;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_VMON | PMBUS_HAVE_STATUS_VMON;
    info.m[PSC_VOLTAGE_IN] = 8;
    info.R[PSC_VOLTAGE_IN] = 2;
    info.m[PSC_VOLTAGE_OUT] = 8;
    info.R[PSC_VOLTAGE_OUT] = 2;
    info.read_word_data = ina233_read_word_data;
// If INA233 skips current/power, shunt-resistor and current-lsb aren't needed.
// read rshunt value (uOhm)
    propname = "shunt-resistor";
    if (device_property_present(dev, propname)) {
    ret = device_property_read_u32(dev, propname, &rshunt);
    if (ret)
    return dev_err_probe(dev, ret, "%s property read fail.\n", propname);
    } else {
    rshunt = INA233_RSHUNT_DEFAULT;
    }
    if (!rshunt)
    return dev_err_probe(dev, -EINVAL, "%s cannot be zero.\n", propname);
// read Maximum expected current value (uA)
    propname = "ti,maximum-expected-current-microamp";
    if (device_property_present(dev, propname)) {
    ret = device_property_read_u32(dev, propname, &max_current);
    if (ret)
    return dev_err_probe(dev, ret, "%s property read fail.\n", propname);
    } else {
    max_current = INA233_MAX_CURRENT_DEFAULT;
    }
    if (max_current < 32768)
    return dev_err_probe(dev, -EINVAL, "%s cannot be less than 32768.\n", propname);
// Calculate Current_LSB according to the spec formula
    current_lsb = max_current / 32768;
// calculate current coefficient
    calculate_coef(&m, &R, current_lsb, 1);
    info.m[PSC_CURRENT_OUT] = m;
    info.R[PSC_CURRENT_OUT] = R;
// calculate power coefficient
    calculate_coef(&m, &R, current_lsb, 25);
    info.m[PSC_POWER] = m;
    info.R[PSC_POWER] = R;
// write MFR_CALIBRATION register, Apply formula from spec with unit scaling.
    calibration = div64_u64(5120000000ULL, (u64)rshunt * current_lsb);
    if (calibration > 0x7FFF)
    return dev_err_probe(dev, -EINVAL,
    "Product of Current_LSB %u and shunt resistor %u too small, MFR_CALIBRATION reg exceeds 0x7FFF.\n",
    current_lsb, rshunt);
    ret = i2c_smbus_write_word_data(client, MFR_CALIBRATION, calibration);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Unable to write calibration.\n");
    dev_dbg(dev, "power monitor %s (Rshunt = %u uOhm, Current_LSB = %u uA/bit)\n",
    client.name, rshunt, current_lsb);
    return pmbus_do_probe(client, info);
    }
    static const struct i2c_device_id ina233_id[] = {
    { .name = "ina233" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ina233_id);
    static const struct of_device_id __maybe_unused ina233_of_match[] = {
    { .compatible = "ti,ina233" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ina233_of_match);
    static struct i2c_driver ina233_driver = {
    .driver = {
    .name = "ina233",
    .of_match_table = of_match_ptr(ina233_of_match),
    },
    .probe = ina233_probe,
    .id_table = ina233_id,
    };
    module_i2c_driver(ina233_driver);
    MODULE_AUTHOR("Leo Yang <leo.yang.sy0@gmail.com>");
    MODULE_DESCRIPTION("PMBus driver for INA233 and compatible chips");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
