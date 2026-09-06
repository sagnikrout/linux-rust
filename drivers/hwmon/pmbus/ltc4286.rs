//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ltc4286.c
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

// LTC4286 register
pub const LTC4286_MFR_CONFIG1: c_uint = 0xF2;
// LTC4286 configuration

pub const LTC4286_MFR_ID_SIZE: c_int = 3;
//
// Initialize the MBR as default settings which is referred to LTC4286 datasheet
// (March 22, 2022 version) table 3 page 16
//
    static struct pmbus_driver_info ltc4286_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_CURRENT_OUT] = direct,
    .format[PSC_POWER] = direct,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_VOLTAGE_IN] = 32,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = 1,
    .m[PSC_VOLTAGE_OUT] = 32,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = 1,
    .m[PSC_CURRENT_OUT] = 1024,
    .b[PSC_CURRENT_OUT] = 0,
//
// The rsense value used in MBR formula in LTC4286 datasheet should be ohm unit.
// However, the rsense value that user input is micro ohm.
// Thus, the MBR setting which involves rsense should be shifted by 6 digits.
//
    .R[PSC_CURRENT_OUT] = 3 - 6,
    .m[PSC_POWER] = 1,
    .b[PSC_POWER] = 0,
//
// The rsense value used in MBR formula in LTC4286 datasheet should be ohm unit.
// However, the rsense value that user input is micro ohm.
// Thus, the MBR setting which involves rsense should be shifted by 6 digits.
//
    .R[PSC_POWER] = 4 - 6,
    .m[PSC_TEMPERATURE] = 1,
    .b[PSC_TEMPERATURE] = 273,
    .R[PSC_TEMPERATURE] = 0,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT |
    PMBUS_HAVE_PIN | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_STATUS_IOUT | PMBUS_HAVE_STATUS_TEMP,
    };
    static const struct i2c_device_id ltc4286_id[] = {
    { "ltc4286", },
    { "ltc4287", },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, ltc4286_id);
#[no_mangle]
unsafe extern "C" fn ltc4286_probe(client: *mut i2c_client) -> c_int {
    static int ltc4286_probe(struct i2c_client *client)
    {
    int ret;
    const struct i2c_device_id *mid;
    u8 block_buffer[I2C_SMBUS_BLOCK_MAX + 1];
    struct pmbus_driver_info *info;
    u32 rsense;
    int vrange_nval, vrange_oval;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, block_buffer);
    if (ret < 0) {
    return dev_err_probe(&client.dev, ret,
    "Failed to read manufacturer id\n");
    }
//
// Refer to ltc4286 datasheet page 20
// the manufacturer id is LTC
//
    if (ret != LTC4286_MFR_ID_SIZE ||
    strncmp(block_buffer, "LTC", LTC4286_MFR_ID_SIZE)) {
    return dev_err_probe(&client.dev, -ENODEV,
    "Manufacturer id mismatch\n");
    }
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, block_buffer);
    if (ret < 0) {
    return dev_err_probe(&client.dev, ret,
    "Failed to read manufacturer model\n");
    }
    for (mid = ltc4286_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, block_buffer, strlen(mid.name)))
    break;
    }
    if (!mid.name[0])
    return dev_err_probe(&client.dev, -ENODEV,
    "Unsupported device\n");
    if (of_property_read_u32(client.dev.of_node,
    "shunt-resistor-micro-ohms", &rsense))
    rsense = 300; /* 0.3 mOhm if not set via DT */
    if (rsense == 0)
    return -EINVAL;
// Check for the latter MBR value won't overflow
    if (rsense > (INT_MAX / 1024))
    return -EINVAL;
    info = devm_kmemdup(&client.dev, &ltc4286_info, sizeof(*info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
// Check MFR1 CONFIG register bit 1 VRANGE_SELECT before driver loading
    vrange_oval = i2c_smbus_read_word_data(client, LTC4286_MFR_CONFIG1);
    if (vrange_oval < 0)
    return dev_err_probe(&client.dev, vrange_oval,
    "Failed to read manufacturer configuration one\n");
    vrange_nval = vrange_oval;
    if (device_property_read_bool(&client.dev, "adi,vrange-low-enable")) {
    vrange_nval &=
    ~VRANGE_SELECT_BIT; /* VRANGE_SELECT = 0, 25.6 volts */
    info.m[PSC_VOLTAGE_IN] = 128;
    info.m[PSC_VOLTAGE_OUT] = 128;
    info.m[PSC_POWER] = 4 * rsense;
    } else {
    vrange_nval |=
    VRANGE_SELECT_BIT; /* VRANGE_SELECT = 1, 102.4 volts */
    info.m[PSC_POWER] = rsense;
    }
    if (vrange_nval != vrange_oval) {
// Set MFR1 CONFIG register bit 1 VRANGE_SELECT
    ret = i2c_smbus_write_word_data(client, LTC4286_MFR_CONFIG1,
    vrange_nval);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to set vrange\n");
    }
    info.m[PSC_CURRENT_OUT] = 1024 * rsense;
    return pmbus_do_probe(client, info);
    }
    static const struct of_device_id ltc4286_of_match[] = {
    { .compatible = "lltc,ltc4286" },
    { .compatible = "lltc,ltc4287" },
    {}
    };
    static struct i2c_driver ltc4286_driver = {
    .driver = {
    .name = "ltc4286",
    .of_match_table = ltc4286_of_match,
    },
    .probe = ltc4286_probe,
    .id_table = ltc4286_id,
    };
    module_i2c_driver(ltc4286_driver);
    MODULE_AUTHOR("Delphine CC Chiu <Delphine_CC_Chiu@wiwynn.com>");
    MODULE_DESCRIPTION("PMBUS driver for LTC4286 and compatibles");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
