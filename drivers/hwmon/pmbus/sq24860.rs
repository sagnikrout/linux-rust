//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/sq24860.c
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
// Author: Ziming Zhu <ziming.zhu@silergycorp.com>
//

pub const SQ24860_IIN_CAL_GAIN: c_uint = 0x38;
pub const SQ24860_READ_VAUX: c_uint = 0xd0;
pub const SQ24860_READ_VIN_MIN: c_uint = 0xd1;
pub const SQ24860_READ_VIN_PEAK: c_uint = 0xd2;
pub const SQ24860_READ_IIN_PEAK: c_uint = 0xd4;
pub const SQ24860_READ_PIN_PEAK: c_uint = 0xd5;
pub const SQ24860_READ_TEMP_AVG: c_uint = 0xd6;
pub const SQ24860_READ_TEMP_PEAK: c_uint = 0xd7;
pub const SQ24860_READ_VOUT_MIN: c_uint = 0xda;
pub const SQ24860_READ_VIN_AVG: c_uint = 0xdc;
pub const SQ24860_READ_VOUT_AVG: c_uint = 0xdd;
pub const SQ24860_READ_IIN_AVG: c_uint = 0xde;
pub const SQ24860_READ_PIN_AVG: c_uint = 0xdf;
pub const SQ24860_VIREF: c_uint = 0xe0;
pub const SQ24860_PK_MIN_AVG: c_uint = 0xea;

pub const SQ24860_MFR_WRITE_PROTECT: c_uint = 0xf8;

pub const SQ24860_8B_SHIFT: c_int = 2;
pub const SQ24860_IIN_OCF_NUM: c_int = 1000000;
pub const SQ24860_IIN_OCF_DIV: c_int = 129278;
pub const SQ24860_IIN_OCF_OFF: c_int = 165;

    PK_MIN_AVG_RST_AVG  | \
    PK_MIN_AVG_RST_MIN)

//
// Arbitrary default Rimon value: 1.6kOhm
//
pub const SQ24860_DEFAULT_RIMON: c_int = 1600000000;
pub const SQ24860_GIMON: c_int = 18180;
pub const SQ24860_VAUX_DIV: c_int = 20;
#[no_mangle]
unsafe extern "C" fn sq24860_write_iin_cal_gain(client: *mut i2c_client, rimon: u32) -> c_int {
    static int sq24860_write_iin_cal_gain(struct i2c_client *client, u32 rimon)
    {
    let mut temp: u64 = 6400ULL * 1000000000ULL * 1000ULL;
    u64 denom;
    u64 word;
    if (!rimon)
    return -EINVAL;
    denom = (u64)rimon * SQ24860_GIMON;
    word = div64_u64(temp, denom);
    if (!word || word > U16_MAX)
    return -EINVAL;
    return i2c_smbus_write_word_data(client, SQ24860_IIN_CAL_GAIN,
    (u16)word);
    }
    static int sq24860_mfr_write_protect_set(struct i2c_client *client,
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
    return pmbus_write_byte_data(client, -1, SQ24860_MFR_WRITE_PROTECT,
    val);
    }
#[no_mangle]
unsafe extern "C" fn sq24860_mfr_write_protect_get(client: *mut i2c_client) -> c_int {
    static int sq24860_mfr_write_protect_get(struct i2c_client *client)
    {
    let mut ret: c_int = pmbus_read_byte_data(client, -1, SQ24860_MFR_WRITE_PROTECT);
    if (ret < 0)
    return ret;
    return (ret & SQ24860_UNLOCKED) ? 0 : PB_WP_ALL;
    }
    static int sq24860_read_word_data(struct i2c_client *client,
    int page, int phase, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VIN_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VIN_MIN);
    break;
    case PMBUS_VIRT_READ_VIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VIN_AVG);
    break;
    case PMBUS_VIRT_READ_VOUT_MIN:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VOUT_MIN);
    break;
    case PMBUS_VIRT_READ_VOUT_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VOUT_AVG);
    break;
    case PMBUS_VIRT_READ_IIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_IIN_AVG);
    break;
    case PMBUS_VIRT_READ_IIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_IIN_PEAK);
    break;
    case PMBUS_VIRT_READ_TEMP_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_TEMP_AVG);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_TEMP_PEAK);
    break;
    case PMBUS_VIRT_READ_PIN_AVG:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_PIN_AVG);
    break;
    case PMBUS_VIRT_READ_PIN_MAX:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_PIN_PEAK);
    break;
    case PMBUS_VIRT_READ_VMON:
    ret = pmbus_read_word_data(client, page, phase,
    SQ24860_READ_VAUX);
    if (ret < 0)
    break;
    ret = DIV_ROUND_CLOSEST(ret, SQ24860_VAUX_DIV);
    break;
    case PMBUS_VIN_UV_WARN_LIMIT:
    case PMBUS_VIN_UV_FAULT_LIMIT:
    case PMBUS_VIN_OV_WARN_LIMIT:
    case PMBUS_VIN_OV_FAULT_LIMIT:
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
    ret <<= SQ24860_8B_SHIFT;
    break;
    case PMBUS_IIN_OC_FAULT_LIMIT:
//
// VIREF directly sets the over-current limit at which the eFuse
// will turn the FET off and trigger a fault. Expose it through
// this generic property instead of a manufacturer specific one.
//
    ret = pmbus_read_byte_data(client, page, SQ24860_VIREF);
    if (ret < 0)
    break;
    ret = DIV_ROUND_CLOSEST(ret * SQ24860_IIN_OCF_NUM,
    SQ24860_IIN_OCF_DIV);
    ret += SQ24860_IIN_OCF_OFF;
    break;
    case PMBUS_VIRT_SAMPLES:
    ret = pmbus_read_byte_data(client, page, SQ24860_PK_MIN_AVG);
    if (ret < 0)
    break;
    ret = BIT(FIELD_GET(PK_MIN_AVG_AVG_CNT, ret));
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
    static int sq24860_write_word_data(struct i2c_client *client,
    int page, int reg, u16 value)
    {
    int ret;
    switch (reg) {
    case PMBUS_VIN_UV_WARN_LIMIT:
    case PMBUS_VIN_UV_FAULT_LIMIT:
    case PMBUS_VIN_OV_WARN_LIMIT:
    case PMBUS_VIN_OV_FAULT_LIMIT:
    case PMBUS_VOUT_UV_WARN_LIMIT:
    case PMBUS_IIN_OC_WARN_LIMIT:
    case PMBUS_OT_WARN_LIMIT:
    case PMBUS_OT_FAULT_LIMIT:
    case PMBUS_PIN_OP_WARN_LIMIT:
    value = max_t(s16, (s16)value, 0);
    value >>= SQ24860_8B_SHIFT;
    value = clamp_val(value, 0, 0xff);
    ret = pmbus_write_word_data(client, page, reg, value);
    break;
    case PMBUS_IIN_OC_FAULT_LIMIT:
    value = max_t(s16, (s16)value, SQ24860_IIN_OCF_OFF);
    value -= SQ24860_IIN_OCF_OFF;
    value = DIV_ROUND_CLOSEST(((unsigned int)value) * SQ24860_IIN_OCF_DIV,
    SQ24860_IIN_OCF_NUM);
    value = clamp_val(value, 0, 0x3f);
    ret = pmbus_write_byte_data(client, page, SQ24860_VIREF, value);
    break;
    case PMBUS_VIRT_SAMPLES:
    value = clamp_val(value, 1, SQ24860_MAX_SAMPLES);
    value = ilog2(value);
    ret = pmbus_update_byte_data(client, page, SQ24860_PK_MIN_AVG,
    PK_MIN_AVG_AVG_CNT,
    FIELD_PREP(PK_MIN_AVG_AVG_CNT, value));
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
//
// SQ24860 has history resets based on MIN/AVG/PEAK instead of per
// sensor type. Exposing this quirk in hwmon is not desirable so
// reset MIN, AVG and PEAK together. Even is there effectively only
// one reset, which resets everything, expose the 5 entries so
// userspace is not required map a sensor type to another to trigger
// a reset
//
    ret = pmbus_update_byte_data(client, 0, SQ24860_PK_MIN_AVG,
    PK_MIN_AVG_RST_MASK,
    PK_MIN_AVG_RST_MASK);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int sq24860_read_byte_data(struct i2c_client *client,
    int page, int reg)
    {
    int ret;
    switch (reg) {
    case PMBUS_WRITE_PROTECT:
    ret = sq24860_mfr_write_protect_get(client);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int sq24860_write_byte_data(struct i2c_client *client,
    int page, int reg, u8 byte)
    {
    int ret;
    switch (reg) {
    case PMBUS_WRITE_PROTECT:
    ret = sq24860_mfr_write_protect_set(client, byte);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }

    static const struct regulator_desc sq24860_reg_desc[] = {
    PMBUS_REGULATOR_ONE_NODE("vout"),
    };

    static const struct pmbus_driver_info sq24860_base_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 64,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = 0,
    .format[PSC_VOLTAGE_OUT] = direct,
    .m[PSC_VOLTAGE_OUT] = 64,
    .b[PSC_VOLTAGE_OUT] = 0,
    .R[PSC_VOLTAGE_OUT] = 0,
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 1,
    .b[PSC_TEMPERATURE] = 0,
    .R[PSC_TEMPERATURE] = 0,
//
// Current and power measurements depend on the calibration gain
// programmed from the board-specific IMON resistor value.
//
    .format[PSC_CURRENT_IN] = direct,
    .m[PSC_CURRENT_IN] = 16,
    .b[PSC_CURRENT_IN] = 0,
    .R[PSC_CURRENT_IN] = 0,
    .format[PSC_POWER] = direct,
    .m[PSC_POWER] = 2,
    .b[PSC_POWER] = 0,
    .R[PSC_POWER] = 0,
    .func[0] = PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT |
    PMBUS_HAVE_VMON |
    PMBUS_HAVE_IIN |
    PMBUS_HAVE_PIN |
    PMBUS_HAVE_TEMP |
    PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_SAMPLES,
    .read_word_data = sq24860_read_word_data,
    .write_word_data = sq24860_write_word_data,
    .read_byte_data = sq24860_read_byte_data,
    .write_byte_data = sq24860_write_byte_data,

    .reg_desc = sq24860_reg_desc,
    .num_regulators = ARRAY_SIZE(sq24860_reg_desc),

    };
    static const struct i2c_device_id sq24860_i2c_id[] = {
    { "sq24860" },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, sq24860_i2c_id);
    static const struct of_device_id sq24860_of_match[] = {
    { .compatible = "silergy,sq24860" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sq24860_of_match);
#[no_mangle]
unsafe extern "C" fn sq24860_probe(client: *mut i2c_client) -> c_int {
    static int sq24860_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct pmbus_driver_info *info;
    u32 rimon;
    int ret;
    if (device_property_read_u32(dev, "silergy,rimon-micro-ohms", &rimon))
    rimon = SQ24860_DEFAULT_RIMON;
    ret = sq24860_write_iin_cal_gain(client, rimon);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to set gain\n");
    info = devm_kmemdup(dev, &sq24860_base_info, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    return pmbus_do_probe(client, info);
    }
    static struct i2c_driver sq24860_driver = {
    .driver = {
    .name = "sq24860",
    .of_match_table = sq24860_of_match,
    },
    .probe = sq24860_probe,
    .id_table = sq24860_i2c_id,
    };
    module_i2c_driver(sq24860_driver);
    MODULE_AUTHOR("Ziming Zhu <ziming.zhu@silergycorp.com>");
    MODULE_DESCRIPTION("PMBUS driver for SQ24860 eFuse");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
