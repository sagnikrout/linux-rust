//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ltc2978.c
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
// Hardware monitoring driver for LTC2978 and compatible chips.
//
// Copyright (c) 2011 Ericsson AB.
// Copyright (c) 2013, 2014, 2015 Guenter Roeck
// Copyright (c) 2015 Linear Technology
// Copyright (c) 2018 Analog Devices Inc.
//

    enum chips {
// Managers
    ltc2972, ltc2974, ltc2975, ltc2977, ltc2978, ltc2979, ltc2980,
// Controllers
    lt7170, lt7171, ltc3880, ltc3882, ltc3883, ltc3884, ltc3886, ltc3887,
    ltc3889, ltc7132, ltc7841, ltc7880,
// Modules
    ltm2987, ltm4664, ltm4673, ltm4675, ltm4676, ltm4677, ltm4678, ltm4680,
    ltm4686, ltm4700,
    };
// Common for all chips
pub const LTC2978_MFR_VOUT_PEAK: c_uint = 0xdd;
pub const LTC2978_MFR_VIN_PEAK: c_uint = 0xde;
pub const LTC2978_MFR_TEMPERATURE_PEAK: c_uint = 0xdf;
pub const LTC2978_MFR_SPECIAL_ID: c_uint = 0xe7	/* Undocumented on LTC3882 */;
pub const LTC2978_MFR_COMMON: c_uint = 0xef;
// LTC2974, LTC2975, LCT2977, LTC2980, LTC2978, and LTM2987
pub const LTC2978_MFR_VOUT_MIN: c_uint = 0xfb;
pub const LTC2978_MFR_VIN_MIN: c_uint = 0xfc;
pub const LTC2978_MFR_TEMPERATURE_MIN: c_uint = 0xfd;
// LTC2974, LTC2975
pub const LTC2974_MFR_IOUT_PEAK: c_uint = 0xd7;
pub const LTC2974_MFR_IOUT_MIN: c_uint = 0xd8;
// LTC3880, LTC3882, LTC3883, LTC3887, LTM4675, LTM4676, LTC7132
pub const LTC3880_MFR_IOUT_PEAK: c_uint = 0xd7;
pub const LTC3880_MFR_CLEAR_PEAKS: c_uint = 0xe3;
pub const LTC3880_MFR_TEMPERATURE2_PEAK: c_uint = 0xf4;
// LTC3883, LTC3884, LTC3886, LTC3889, LTC7132, LTC7841 and LTC7880 only
pub const LTC3883_MFR_IIN_PEAK: c_uint = 0xe1;
// LTC2975 only
pub const LTC2975_MFR_IIN_PEAK: c_uint = 0xc4;
pub const LTC2975_MFR_IIN_MIN: c_uint = 0xc5;
pub const LTC2975_MFR_PIN_PEAK: c_uint = 0xc6;
pub const LTC2975_MFR_PIN_MIN: c_uint = 0xc7;
pub const LTC2978_ID_MASK: c_uint = 0xfff0;
pub const LT7170_ID: c_uint = 0x1C10;
pub const LTC2972_ID: c_uint = 0x0310;
pub const LTC2974_ID: c_uint = 0x0210;
pub const LTC2975_ID: c_uint = 0x0220;
pub const LTC2977_ID: c_uint = 0x0130;
pub const LTC2978_ID_REV1: c_uint = 0x0110	/* Early revision */;
pub const LTC2978_ID_REV2: c_uint = 0x0120;
pub const LTC2979_ID_A: c_uint = 0x8060;
pub const LTC2979_ID_B: c_uint = 0x8070;
pub const LTC2980_ID_A: c_uint = 0x8030	/* A/B for two die IDs */;
pub const LTC2980_ID_B: c_uint = 0x8040;
pub const LTC3880_ID: c_uint = 0x4020;
pub const LTC3882_ID: c_uint = 0x4200;
pub const LTC3882_ID_D1: c_uint = 0x4240	/* Dash 1 */;
pub const LTC3883_ID: c_uint = 0x4300;
pub const LTC3884_ID: c_uint = 0x4C00;
pub const LTC3886_ID: c_uint = 0x4600;
pub const LTC3887_ID: c_uint = 0x4700;
pub const LTC3889_ID: c_uint = 0x4900;
pub const LTC7132_ID: c_uint = 0x4CE0;
pub const LTC7841_ID: c_uint = 0x40D0;
pub const LTC7880_ID: c_uint = 0x49E0;
pub const LTM2987_ID_A: c_uint = 0x8010	/* A/B for two die IDs */;
pub const LTM2987_ID_B: c_uint = 0x8020;
pub const LTM4664_ID: c_uint = 0x4120;
pub const LTM4673_ID_REV1: c_uint = 0x0230;
pub const LTM4673_ID: c_uint = 0x4480;
pub const LTM4675_ID: c_uint = 0x47a0;
pub const LTM4676_ID_REV1: c_uint = 0x4400;
pub const LTM4676_ID_REV2: c_uint = 0x4480;
pub const LTM4676A_ID: c_uint = 0x47e0;
pub const LTM4677_ID_REV1: c_uint = 0x47B0;
pub const LTM4677_ID_REV2: c_uint = 0x47D0;
pub const LTM4678_ID_REV1: c_uint = 0x4100;
pub const LTM4678_ID_REV2: c_uint = 0x4110;
pub const LTM4680_ID: c_uint = 0x4140;
pub const LTM4686_ID: c_uint = 0x4770;
pub const LTM4700_ID: c_uint = 0x4130;
pub const LTC2972_NUM_PAGES: c_int = 2;
pub const LTC2974_NUM_PAGES: c_int = 4;
pub const LTC2978_NUM_PAGES: c_int = 8;
pub const LTC3880_NUM_PAGES: c_int = 2;
pub const LTC3883_NUM_PAGES: c_int = 1;

//
// LTC2978 clears peak data whenever the CLEAR_FAULTS command is executed, which
// happens pretty much each time chip data is updated. Raw peak data therefore
// does not provide much value. To be able to provide useful peak data, keep an
// internal cache of measured peak data, which is only cleared if an explicit
// "clear peak" command is executed for the sensor in question.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2978_data {
    pub id: enum chips,
    pub vin_max: u16 vin_min,,
    pub temp_max: [u16 temp_min[LTC2974_NUM_PAGES],; LTC2974_NUM_PAGES],
    pub vout_max: [u16 vout_min[LTC2978_NUM_PAGES],; LTC2978_NUM_PAGES],
    pub iout_max: [u16 iout_min[LTC2974_NUM_PAGES],; LTC2974_NUM_PAGES],
    pub iin_max: u16 iin_min,,
    pub pin_max: u16 pin_min,,
    pub temp2_max: u16,
    pub info: pmbus_driver_info,
    pub features: u32,
}

#[no_mangle]
unsafe extern "C" fn ltc_wait_ready(client: *mut i2c_client) -> c_int {
    static int ltc_wait_ready(struct i2c_client *client)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(LTC_POLL_TIMEOUT);
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int status;
    u8 mask;
    if (!needs_polling(data))
    return 0;
//
// LTC3883 does not support LTC_NOT_PENDING, even though
// the datasheet claims that it does.
//
    mask = LTC_NOT_BUSY;
    if (data.id != ltc3883)
    mask |= LTC_NOT_PENDING;
    do {
    status = pmbus_read_byte_data(client, 0, LTC2978_MFR_COMMON);
    if (status == -EBADMSG || status == -ENXIO) {
// PEC error or NACK: chip may be busy, try again
    usleep_range(50, 100);
    continue;
    }
    if (status < 0)
    return status;
    if ((status & mask) == mask)
    return 0;
    usleep_range(50, 100);
    } while (time_before(jiffies, timeout));
    return -ETIMEDOUT;
    }
    static int ltc_read_word_data(struct i2c_client *client, int page, int phase,
    int reg)
    {
    int ret;
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    return pmbus_read_word_data(client, page, 0xff, reg);
    }
#[no_mangle]
unsafe extern "C" fn ltc_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int ltc_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    int ret;
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    return pmbus_read_byte_data(client, page, reg);
    }
#[no_mangle]
unsafe extern "C" fn ltc_write_byte_data(client: *mut i2c_client, page: c_int, reg: c_int, value: u8) -> c_int {
    static int ltc_write_byte_data(struct i2c_client *client, int page, int reg, u8 value)
    {
    int ret;
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    return pmbus_write_byte_data(client, page, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn ltc_write_byte(client: *mut i2c_client, page: c_int, byte: u8) -> c_int {
    static int ltc_write_byte(struct i2c_client *client, int page, u8 byte)
    {
    int ret;
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    return pmbus_write_byte(client, page, byte);
    }
#[no_mangle]
pub unsafe extern "C" fn lin11_to_val(data: c_int) -> c_int {
    static inline int lin11_to_val(int data)
    {
    let mut e: i16 = ((s16)data) >> 11;
    let mut m: i32 = (((s16)(data << 5)) >> 5);
//
// mantissa is 10 bit + sign, exponent adds up to 15 bit.
// Add 6 bit to exponent for maximum accuracy (10 + 15 + 6 = 31).
//
    e += 6;
    return (e < 0 ? m >> -e : m << e);
    }
    static int ltc_get_max(struct ltc2978_data *data, struct i2c_client *client,
    int page, int reg, u16 *pmax)
    {
    int ret;
    ret = ltc_read_word_data(client, page, 0xff, reg);
    if (ret >= 0) {
    if (lin11_to_val(ret) > lin11_to_val(*pmax))
// pmax = ret;
    ret = *pmax;
    }
    return ret;
    }
    static int ltc_get_min(struct ltc2978_data *data, struct i2c_client *client,
    int page, int reg, u16 *pmin)
    {
    int ret;
    ret = ltc_read_word_data(client, page, 0xff, reg);
    if (ret >= 0) {
    if (lin11_to_val(ret) < lin11_to_val(*pmin))
// pmin = ret;
    ret = *pmin;
    }
    return ret;
    }
    static int ltc2978_read_word_data_common(struct i2c_client *client, int page,
    int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = ltc_get_max(data, client, page, LTC2978_MFR_VIN_PEAK,
    &data.vin_max);
    break;
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = ltc_read_word_data(client, page, 0xff,
    LTC2978_MFR_VOUT_PEAK);
    if (ret >= 0) {
//
// VOUT is 16 bit unsigned with fixed exponent,
// so we can compare it directly
//
    if (ret > data.vout_max[page])
    data.vout_max[page] = ret;
    ret = data.vout_max[page];
    }
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    ret = ltc_get_max(data, client, page,
    LTC2978_MFR_TEMPERATURE_PEAK,
    &data.temp_max[page]);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    ret = 0;
    break;
    default:
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int ltc2978_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_VIN_MIN:
    ret = ltc_get_min(data, client, page, LTC2978_MFR_VIN_MIN,
    &data.vin_min);
    break;
    case PMBUS_VIRT_READ_VOUT_MIN:
    ret = ltc_read_word_data(client, page, phase,
    LTC2978_MFR_VOUT_MIN);
    if (ret >= 0) {
//
// VOUT_MIN is known to not be supported on some lots
// of LTC2978 revision 1, and will return the maximum
// possible voltage if read. If VOUT_MAX is valid and
// lower than the reading of VOUT_MIN, use it instead.
//
    if (data.vout_max[page] && ret > data.vout_max[page])
    ret = data.vout_max[page];
    if (ret < data.vout_min[page])
    data.vout_min[page] = ret;
    ret = data.vout_min[page];
    }
    break;
    case PMBUS_VIRT_READ_TEMP_MIN:
    ret = ltc_get_min(data, client, page,
    LTC2978_MFR_TEMPERATURE_MIN,
    &data.temp_min[page]);
    break;
    case PMBUS_VIRT_READ_IOUT_MAX:
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    case PMBUS_VIRT_READ_TEMP2_MAX:
    case PMBUS_VIRT_RESET_TEMP2_HISTORY:
    ret = -ENXIO;
    break;
    default:
    ret = ltc2978_read_word_data_common(client, page, reg);
    break;
    }
    return ret;
    }
    static int ltc2974_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = ltc_get_max(data, client, page, LTC2974_MFR_IOUT_PEAK,
    &data.iout_max[page]);
    break;
    case PMBUS_VIRT_READ_IOUT_MIN:
    ret = ltc_get_min(data, client, page, LTC2974_MFR_IOUT_MIN,
    &data.iout_min[page]);
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    ret = 0;
    break;
    default:
    ret = ltc2978_read_word_data(client, page, phase, reg);
    break;
    }
    return ret;
    }
    static int ltc2975_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_IIN_MAX:
    ret = ltc_get_max(data, client, page, LTC2975_MFR_IIN_PEAK,
    &data.iin_max);
    break;
    case PMBUS_VIRT_READ_IIN_MIN:
    ret = ltc_get_min(data, client, page, LTC2975_MFR_IIN_MIN,
    &data.iin_min);
    break;
    case PMBUS_VIRT_READ_PIN_MAX:
    ret = ltc_get_max(data, client, page, LTC2975_MFR_PIN_PEAK,
    &data.pin_max);
    break;
    case PMBUS_VIRT_READ_PIN_MIN:
    ret = ltc_get_min(data, client, page, LTC2975_MFR_PIN_MIN,
    &data.pin_min);
    break;
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    ret = 0;
    break;
    default:
    ret = ltc2978_read_word_data(client, page, phase, reg);
    break;
    }
    return ret;
    }
    static int ltc3880_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = ltc_get_max(data, client, page, LTC3880_MFR_IOUT_PEAK,
    &data.iout_max[page]);
    break;
    case PMBUS_VIRT_READ_TEMP2_MAX:
    ret = ltc_get_max(data, client, page,
    LTC3880_MFR_TEMPERATURE2_PEAK,
    &data.temp2_max);
    break;
    case PMBUS_VIRT_READ_VIN_MIN:
    case PMBUS_VIRT_READ_VOUT_MIN:
    case PMBUS_VIRT_READ_TEMP_MIN:
    ret = -ENXIO;
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    case PMBUS_VIRT_RESET_TEMP2_HISTORY:
    ret = 0;
    break;
    default:
    ret = ltc2978_read_word_data_common(client, page, reg);
    break;
    }
    return ret;
    }
    static int ltc3883_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_READ_IIN_MAX:
    ret = ltc_get_max(data, client, page, LTC3883_MFR_IIN_PEAK,
    &data.iin_max);
    break;
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    ret = 0;
    break;
    default:
    ret = ltc3880_read_word_data(client, page, phase, reg);
    break;
    }
    return ret;
    }
    static int ltc2978_clear_peaks(struct ltc2978_data *data,
    struct i2c_client *client, int page)
    {
    int ret;
    if (has_clear_peaks(data))
    ret = ltc_write_byte(client, 0, LTC3880_MFR_CLEAR_PEAKS);
    else
    ret = ltc_write_byte(client, page, PMBUS_CLEAR_FAULTS);
    return ret;
    }
    static int ltc2978_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct ltc2978_data *data = to_ltc2978_data(info);
    int ret;
    switch (reg) {
    case PMBUS_VIRT_RESET_IIN_HISTORY:
    data.iin_max = 0x7c00;
    data.iin_min = 0x7bff;
    ret = ltc2978_clear_peaks(data, client, 0);
    break;
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    data.pin_max = 0x7c00;
    data.pin_min = 0x7bff;
    ret = ltc2978_clear_peaks(data, client, 0);
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    data.iout_max[page] = 0x7c00;
    data.iout_min[page] = 0xfbff;
    ret = ltc2978_clear_peaks(data, client, page);
    break;
    case PMBUS_VIRT_RESET_TEMP2_HISTORY:
    data.temp2_max = 0x7c00;
    ret = ltc2978_clear_peaks(data, client, page);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    data.vout_min[page] = 0xffff;
    data.vout_max[page] = 0;
    ret = ltc2978_clear_peaks(data, client, page);
    break;
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    data.vin_min = 0x7bff;
    data.vin_max = 0x7c00;
    ret = ltc2978_clear_peaks(data, client, page);
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    data.temp_min[page] = 0x7bff;
    data.temp_max[page] = 0x7c00;
    ret = ltc2978_clear_peaks(data, client, page);
    break;
    default:
    ret = ltc_wait_ready(client);
    if (ret < 0)
    return ret;
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static const struct i2c_device_id ltc2978_id[] = {
    { .name = "lt7170", .driver_data = lt7170 },
    { .name = "lt7171", .driver_data = lt7171 },
    { .name = "ltc2972", .driver_data = ltc2972 },
    { .name = "ltc2974", .driver_data = ltc2974 },
    { .name = "ltc2975", .driver_data = ltc2975 },
    { .name = "ltc2977", .driver_data = ltc2977 },
    { .name = "ltc2978", .driver_data = ltc2978 },
    { .name = "ltc2979", .driver_data = ltc2979 },
    { .name = "ltc2980", .driver_data = ltc2980 },
    { .name = "ltc3880", .driver_data = ltc3880 },
    { .name = "ltc3882", .driver_data = ltc3882 },
    { .name = "ltc3883", .driver_data = ltc3883 },
    { .name = "ltc3884", .driver_data = ltc3884 },
    { .name = "ltc3886", .driver_data = ltc3886 },
    { .name = "ltc3887", .driver_data = ltc3887 },
    { .name = "ltc3889", .driver_data = ltc3889 },
    { .name = "ltc7132", .driver_data = ltc7132 },
    { .name = "ltc7841", .driver_data = ltc7841 },
    { .name = "ltc7880", .driver_data = ltc7880 },
    { .name = "ltm2987", .driver_data = ltm2987 },
    { .name = "ltm4664", .driver_data = ltm4664 },
    { .name = "ltm4673", .driver_data = ltm4673 },
    { .name = "ltm4675", .driver_data = ltm4675 },
    { .name = "ltm4676", .driver_data = ltm4676 },
    { .name = "ltm4677", .driver_data = ltm4677 },
    { .name = "ltm4678", .driver_data = ltm4678 },
    { .name = "ltm4680", .driver_data = ltm4680 },
    { .name = "ltm4686", .driver_data = ltm4686 },
    { .name = "ltm4700", .driver_data = ltm4700 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc2978_id);

pub const LTC2978_ADC_RES: c_uint = 0xFFFF;
pub const LTC2978_N_ADC: c_int = 122;

pub const LTC2978_UV_STEP: c_int = 1000;

    static const struct regulator_desc ltc2978_reg_desc[] = {
    PMBUS_REGULATOR_STEP("vout", 0, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 1, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 2, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 3, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 4, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 5, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 6, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    PMBUS_REGULATOR_STEP("vout", 7, LTC2978_N_VOLTAGES, LTC2978_UV_STEP, 0),
    };
    static const struct regulator_desc ltc2978_reg_desc_default[] = {
    PMBUS_REGULATOR("vout", 0),
    PMBUS_REGULATOR("vout", 1),
    PMBUS_REGULATOR("vout", 2),
    PMBUS_REGULATOR("vout", 3),
    PMBUS_REGULATOR("vout", 4),
    PMBUS_REGULATOR("vout", 5),
    PMBUS_REGULATOR("vout", 6),
    PMBUS_REGULATOR("vout", 7),
    };

#[no_mangle]
unsafe extern "C" fn ltc2978_get_id(client: *mut i2c_client) -> c_int {
    static int ltc2978_get_id(struct i2c_client *client)
    {
    int chip_id;
    chip_id = i2c_smbus_read_word_data(client, LTC2978_MFR_SPECIAL_ID);
    if (chip_id < 0) {
    const struct i2c_device_id *id;
    u8 buf[I2C_SMBUS_BLOCK_MAX];
    int ret;
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0)
    return ret;
    if (ret < 3 || (strncmp(buf, "LTC", 3) && strncmp(buf, "ADI", 3)))
    return -ENODEV;
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0)
    return ret;
    for (id = &ltc2978_id[0]; strlen(id.name); id++) {
    if (!strncasecmp(id.name, buf, strlen(id.name)))
    return (int)id.driver_data;
    }
    return -ENODEV;
    }
    chip_id &= LTC2978_ID_MASK;
    if (chip_id == LT7170_ID) {
    u8 buf[I2C_SMBUS_BLOCK_MAX];
    int ret;
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_IC_DEVICE_ID,
    buf);
    if (ret < 0)
    return ret;
    if (ret < 6)
    return -ENODEV;
    if (!strncmp(buf, "LT7170", 6))
    return lt7170;
    if (!strncmp(buf, "LT7171", 6))
    return lt7171;
    return -ENODEV;
    }
    if (chip_id == LTC2972_ID)
    return ltc2972;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2974_ID: chip_id ==) -> else {
    else if (chip_id == LTC2974_ID)
    return ltc2974;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2975_ID: chip_id ==) -> else {
    else if (chip_id == LTC2975_ID)
    return ltc2975;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2977_ID: chip_id ==) -> else {
    else if (chip_id == LTC2977_ID)
    return ltc2977;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2978_ID_REV2: chip_id == LTC2978_ID_REV1 || chip_id ==) -> else {
    else if (chip_id == LTC2978_ID_REV1 || chip_id == LTC2978_ID_REV2)
    return ltc2978;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2979_ID_B: chip_id == LTC2979_ID_A || chip_id ==) -> else {
    else if (chip_id == LTC2979_ID_A || chip_id == LTC2979_ID_B)
    return ltc2979;
#[no_mangle]
pub unsafe extern "C" fn if(LTC2980_ID_B: chip_id == LTC2980_ID_A || chip_id ==) -> else {
    else if (chip_id == LTC2980_ID_A || chip_id == LTC2980_ID_B)
    return ltc2980;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3880_ID: chip_id ==) -> else {
    else if (chip_id == LTC3880_ID)
    return ltc3880;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3882_ID_D1: chip_id == LTC3882_ID || chip_id ==) -> else {
    else if (chip_id == LTC3882_ID || chip_id == LTC3882_ID_D1)
    return ltc3882;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3883_ID: chip_id ==) -> else {
    else if (chip_id == LTC3883_ID)
    return ltc3883;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3884_ID: chip_id ==) -> else {
    else if (chip_id == LTC3884_ID)
    return ltc3884;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3886_ID: chip_id ==) -> else {
    else if (chip_id == LTC3886_ID)
    return ltc3886;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3887_ID: chip_id ==) -> else {
    else if (chip_id == LTC3887_ID)
    return ltc3887;
#[no_mangle]
pub unsafe extern "C" fn if(LTC3889_ID: chip_id ==) -> else {
    else if (chip_id == LTC3889_ID)
    return ltc3889;
#[no_mangle]
pub unsafe extern "C" fn if(LTC7132_ID: chip_id ==) -> else {
    else if (chip_id == LTC7132_ID)
    return ltc7132;
#[no_mangle]
pub unsafe extern "C" fn if(LTC7841_ID: chip_id ==) -> else {
    else if (chip_id == LTC7841_ID)
    return ltc7841;
#[no_mangle]
pub unsafe extern "C" fn if(LTC7880_ID: chip_id ==) -> else {
    else if (chip_id == LTC7880_ID)
    return ltc7880;
#[no_mangle]
pub unsafe extern "C" fn if(LTM2987_ID_B: chip_id == LTM2987_ID_A || chip_id ==) -> else {
    else if (chip_id == LTM2987_ID_A || chip_id == LTM2987_ID_B)
    return ltm2987;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4664_ID: chip_id ==) -> else {
    else if (chip_id == LTM4664_ID)
    return ltm4664;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4673_ID_REV1: chip_id == LTM4673_ID || chip_id ==) -> else {
    else if (chip_id == LTM4673_ID || chip_id == LTM4673_ID_REV1)
    return ltm4673;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4675_ID: chip_id ==) -> else {
    else if (chip_id == LTM4675_ID)
    return ltm4675;
    else if (chip_id == LTM4676_ID_REV1 || chip_id == LTM4676_ID_REV2 ||
    chip_id == LTM4676A_ID)
    return ltm4676;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4677_ID_REV2: chip_id == LTM4677_ID_REV1 || chip_id ==) -> else {
    else if (chip_id == LTM4677_ID_REV1 || chip_id == LTM4677_ID_REV2)
    return ltm4677;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4678_ID_REV2: chip_id == LTM4678_ID_REV1 || chip_id ==) -> else {
    else if (chip_id == LTM4678_ID_REV1 || chip_id == LTM4678_ID_REV2)
    return ltm4678;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4680_ID: chip_id ==) -> else {
    else if (chip_id == LTM4680_ID)
    return ltm4680;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4686_ID: chip_id ==) -> else {
    else if (chip_id == LTM4686_ID)
    return ltm4686;
#[no_mangle]
pub unsafe extern "C" fn if(LTM4700_ID: chip_id ==) -> else {
    else if (chip_id == LTM4700_ID)
    return ltm4700;
    dev_err(&client.dev, "Unsupported chip ID 0x%x\n", chip_id);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn ltc2978_probe(client: *mut i2c_client) -> c_int {
    static int ltc2978_probe(struct i2c_client *client)
    {
    int i, chip_id;
    struct ltc2978_data *data;
    struct pmbus_driver_info *info;
    const struct i2c_device_id *id;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_WORD_DATA))
    return -ENODEV;
    data = devm_kzalloc(&client.dev, sizeof(struct ltc2978_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    chip_id = ltc2978_get_id(client);
    if (chip_id < 0)
    return chip_id;
    data.id = chip_id;
    id = i2c_client_get_device_id(client);
    if (data.id != id.driver_data)
    dev_warn(&client.dev,
    "Device mismatch: Configured %s (%d), detected %d\n",
    id.name,
    (int) id.driver_data,
    chip_id);
    info = &data.info;
    info.write_word_data = ltc2978_write_word_data;
    info.write_byte = ltc_write_byte;
    info.write_byte_data = ltc_write_byte_data;
    info.read_word_data = ltc_read_word_data;
    info.read_byte_data = ltc_read_byte_data;
    data.vin_min = 0x7bff;
    data.vin_max = 0x7c00;
    for (i = 0; i < ARRAY_SIZE(data.vout_min); i++)
    data.vout_min[i] = 0xffff;
    for (i = 0; i < ARRAY_SIZE(data.iout_min); i++)
    data.iout_min[i] = 0xfbff;
    for (i = 0; i < ARRAY_SIZE(data.iout_max); i++)
    data.iout_max[i] = 0x7c00;
    for (i = 0; i < ARRAY_SIZE(data.temp_min); i++)
    data.temp_min[i] = 0x7bff;
    for (i = 0; i < ARRAY_SIZE(data.temp_max); i++)
    data.temp_max[i] = 0x7c00;
    data.temp2_max = 0x7c00;
    switch (data.id) {
    case lt7170:
    case lt7171:
    data.features |= FEAT_CLEAR_PEAKS | FEAT_NEEDS_POLLING;
    info.read_word_data = ltc3883_read_word_data;
    info.pages = LTC3883_NUM_PAGES;
    info.format[PSC_VOLTAGE_IN] = ieee754;
    info.format[PSC_VOLTAGE_OUT] = ieee754;
    info.format[PSC_CURRENT_OUT] = ieee754;
    info.format[PSC_TEMPERATURE] = ieee754;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltc2972:
    info.read_word_data = ltc2975_read_word_data;
    info.pages = LTC2972_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_IIN | PMBUS_HAVE_PIN
    | PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP2;
    for (i = 0; i < info.pages; i++) {
    info.func[i] |= PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT;
    }
    break;
    case ltc2974:
    info.read_word_data = ltc2974_read_word_data;
    info.pages = LTC2974_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP2;
    for (i = 0; i < info.pages; i++) {
    info.func[i] |= PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT;
    }
    break;
    case ltc2975:
    info.read_word_data = ltc2975_read_word_data;
    info.pages = LTC2974_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_IIN | PMBUS_HAVE_PIN
    | PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP2;
    for (i = 0; i < info.pages; i++) {
    info.func[i] |= PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT;
    }
    break;
    case ltc2977:
    case ltc2978:
    case ltc2979:
    case ltc2980:
    case ltm2987:
    info.read_word_data = ltc2978_read_word_data;
    info.pages = LTC2978_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    for (i = 1; i < LTC2978_NUM_PAGES; i++) {
    info.func[i] = PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_VOUT;
    }
    break;
    case ltc3880:
    case ltc3887:
    case ltm4675:
    case ltm4676:
    case ltm4677:
    case ltm4686:
    data.features |= FEAT_CLEAR_PEAKS | FEAT_NEEDS_POLLING;
    info.read_word_data = ltc3880_read_word_data;
    info.pages = LTC3880_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP;
    info.func[1] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltc3882:
    data.features |= FEAT_CLEAR_PEAKS | FEAT_NEEDS_POLLING;
    info.read_word_data = ltc3880_read_word_data;
    info.pages = LTC3880_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP;
    info.func[1] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltc3883:
    data.features |= FEAT_CLEAR_PEAKS | FEAT_NEEDS_POLLING;
    info.read_word_data = ltc3883_read_word_data;
    info.pages = LTC3883_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltc3884:
    case ltc3886:
    case ltc3889:
    case ltc7132:
    case ltc7880:
    case ltm4664:
    case ltm4678:
    case ltm4680:
    case ltm4700:
    data.features |= FEAT_CLEAR_PEAKS | FEAT_NEEDS_POLLING;
    info.read_word_data = ltc3883_read_word_data;
    info.pages = LTC3880_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_STATUS_TEMP;
    info.func[1] = PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltc7841:
    data.features |= FEAT_CLEAR_PEAKS;
    info.read_word_data = ltc3883_read_word_data;
    info.pages = LTC3883_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN
    | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    break;
    case ltm4673:
    data.features |= FEAT_NEEDS_POLLING;
    info.read_word_data = ltc2975_read_word_data;
    info.pages = LTC2974_NUM_PAGES;
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT
    | PMBUS_HAVE_TEMP2;
    for (i = 0; i < info.pages; i++) {
    info.func[i] |= PMBUS_HAVE_IIN
    | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_PIN
    | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    }
    break;
    default:
    return -ENODEV;
    }

    info.num_regulators = info.pages;
    switch (data.id) {
    case ltc2972:
    case ltc2974:
    case ltc2975:
    case ltc2977:
    case ltc2978:
    case ltc2979:
    case ltc2980:
    case ltm2987:
    info.reg_desc = ltc2978_reg_desc;
    if (info.num_regulators > ARRAY_SIZE(ltc2978_reg_desc)) {
    dev_warn(&client.dev, "num_regulators too large!");
    info.num_regulators = ARRAY_SIZE(ltc2978_reg_desc);
    }
    break;
    default:
    info.reg_desc = ltc2978_reg_desc_default;
    if (info.num_regulators > ARRAY_SIZE(ltc2978_reg_desc_default)) {
    dev_warn(&client.dev, "num_regulators too large!");
    info.num_regulators =
    ARRAY_SIZE(ltc2978_reg_desc_default);
    }
    break;
    }

    return pmbus_do_probe(client, info);
    }

    static const struct of_device_id ltc2978_of_match[] = {
    { .compatible = "lltc,lt7170" },
    { .compatible = "lltc,lt7171" },
    { .compatible = "lltc,ltc2972" },
    { .compatible = "lltc,ltc2974" },
    { .compatible = "lltc,ltc2975" },
    { .compatible = "lltc,ltc2977" },
    { .compatible = "lltc,ltc2978" },
    { .compatible = "lltc,ltc2979" },
    { .compatible = "lltc,ltc2980" },
    { .compatible = "lltc,ltc3880" },
    { .compatible = "lltc,ltc3882" },
    { .compatible = "lltc,ltc3883" },
    { .compatible = "lltc,ltc3884" },
    { .compatible = "lltc,ltc3886" },
    { .compatible = "lltc,ltc3887" },
    { .compatible = "lltc,ltc3889" },
    { .compatible = "lltc,ltc7132" },
    { .compatible = "lltc,ltc7841" },
    { .compatible = "lltc,ltc7880" },
    { .compatible = "lltc,ltm2987" },
    { .compatible = "lltc,ltm4664" },
    { .compatible = "lltc,ltm4673" },
    { .compatible = "lltc,ltm4675" },
    { .compatible = "lltc,ltm4676" },
    { .compatible = "lltc,ltm4677" },
    { .compatible = "lltc,ltm4678" },
    { .compatible = "lltc,ltm4680" },
    { .compatible = "lltc,ltm4686" },
    { .compatible = "lltc,ltm4700" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc2978_of_match);

    static struct i2c_driver ltc2978_driver = {
    .driver = {
    .name = "ltc2978",
    .of_match_table = of_match_ptr(ltc2978_of_match),
    },
    .probe = ltc2978_probe,
    .id_table = ltc2978_id,
    };
    module_i2c_driver(ltc2978_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("PMBus driver for LTC2978 and compatible chips");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
