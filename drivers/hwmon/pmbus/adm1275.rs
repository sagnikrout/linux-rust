//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/adm1275.c
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
// Hardware monitoring driver for Analog Devices ADM1275 Hot-Swap Controller
// and Digital Power Monitor
//
// Copyright (c) 2011 Ericsson AB.
// Copyright (c) 2018 Guenter Roeck
//

    enum chips { adm1075, adm1272, adm1273, adm1275, adm1276, adm1278, adm1281,
    adm1293, adm1294, bd12780, bd12790, sq24905c };

pub const ADM1275_PEAK_IOUT: c_uint = 0xd0;
pub const ADM1275_PEAK_VIN: c_uint = 0xd1;
pub const ADM1275_PEAK_VOUT: c_uint = 0xd2;
pub const ADM1275_PMON_CONTROL: c_uint = 0xd3;
pub const ADM1275_PMON_CONFIG: c_uint = 0xd4;

// The BD127[89]0 data sheets mark TSFILT bit as reserved.

pub const ADM1293_IRANGE_25: c_int = 0;

pub const ADM1278_PEAK_TEMP: c_uint = 0xd7;
pub const ADM1275_IOUT_WARN2_LIMIT: c_uint = 0xd7;
pub const ADM1275_DEVICE_CONFIG: c_uint = 0xd8;

pub const ADM1276_PEAK_PIN: c_uint = 0xda;
pub const ADM1075_READ_VAUX: c_uint = 0xdd;
pub const ADM1075_VAUX_OV_WARN_LIMIT: c_uint = 0xde;
pub const ADM1075_VAUX_UV_WARN_LIMIT: c_uint = 0xdf;
pub const ADM1293_IOUT_MIN: c_uint = 0xe3;
pub const ADM1293_PIN_MIN: c_uint = 0xe4;
pub const ADM1075_VAUX_STATUS: c_uint = 0xf6;

pub const ADM1275_VI_AVG_SHIFT: c_int = 0;

    ADM1275_VI_AVG_SHIFT)
pub const ADM1275_SAMPLES_AVG_MAX: c_int = 128;
pub const ADM1278_PWR_AVG_SHIFT: c_int = 11;

    ADM1278_PWR_AVG_SHIFT)
pub const ADM1278_VI_AVG_SHIFT: c_int = 8;

    ADM1278_VI_AVG_SHIFT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm1275_data {
    pub id: c_int,
    pub have_oc_fault: bool,
    pub have_uc_fault: bool,
    pub have_vout: bool,
    pub have_vaux_status: bool,
    pub have_mfr_vaux_status: bool,
    pub have_iout_min: bool,
    pub have_pin_min: bool,
    pub have_pin_max: bool,
    pub have_temp_max: bool,
    pub have_power_sampling: bool,
    pub info: pmbus_driver_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coefficients {
    pub m: i16,
    pub b: i16,
    pub R: i16,
}

    static const struct coefficients adm1075_coefficients[] = {
    [0] = { 27169, 0, -1 },		/* voltage */
    [1] = { 806, 20475, -1 },	/* current, irange25 */
    [2] = { 404, 20475, -1 },	/* current, irange50 */
    [3] = { 8549, 0, -1 },		/* power, irange25 */
    [4] = { 4279, 0, -1 },		/* power, irange50 */
    };
    static const struct coefficients adm1272_coefficients[] = {
    [0] = { 6770, 0, -2 },		/* voltage, vrange 60V */
    [1] = { 4062, 0, -2 },		/* voltage, vrange 100V */
    [2] = { 1326, 20480, -1 },	/* current, vsense range 15mV */
    [3] = { 663, 20480, -1 },	/* current, vsense range 30mV */
    [4] = { 3512, 0, -2 },		/* power, vrange 60V, irange 15mV */
    [5] = { 21071, 0, -3 },		/* power, vrange 100V, irange 15mV */
    [6] = { 17561, 0, -3 },		/* power, vrange 60V, irange 30mV */
    [7] = { 10535, 0, -3 },		/* power, vrange 100V, irange 30mV */
    [8] = { 42, 31871, -1 },	/* temperature */
    };
//
// BD12790 coefficients derived from preliminary datasheet, Table 1 (p.18)
// and the PMBus direct-format relationship X = (Y * 10^(-R) - b) / m.
//
// Voltage: V[V] = 14.77e-3 * code (60V) / 24.62e-3 * code (100V)
// -> m = 6770, R=-2 / m = 4062, R=-2
// Current: code = I[A] * RS * 132802.1 + 2048 (15mV) / * 66401.06 + 2048 (30mV)
// -> m = 1328, b = 2048 * 10^(-R) = 20480, R=-1 / m = 664, same b and R
// Power: code = k * RS * PIN, k = 35119.94 / 17559.97 / 21071.44 / 10535.72
// -> m = round(k * 10^(-3-R)), R=-2 for 60V/15mV, R=-3 for the other three
// Temperature: code = 4.2 * T + 3188 -> m = 42, b = 3188 * 10 = 31880, R=-1
//
    static const struct coefficients bd12790_coefficients[] = {
    [0] = { 6770, 0, -2 },		/* voltage, vrange 60V */
    [1] = { 4062, 0, -2 },		/* voltage, vrange 100V */
    [2] = { 1328, 20480, -1 },	/* current, vsense range 15mV */
    [3] = { 664, 20480, -1 },	/* current, vsense range 30mV */
    [4] = { 3512, 0, -2 },		/* power, vrange 60V, irange 15mV */
    [5] = { 21071, 0, -3 },		/* power, vrange 100V, irange 15mV */
    [6] = { 17560, 0, -3 },		/* power, vrange 60V, irange 30mV */
    [7] = { 10536, 0, -3 },		/* power, vrange 100V, irange 30mV */
    [8] = { 42, 31880, -1 },	/* temperature */
    };
    static const struct coefficients adm1275_coefficients[] = {
    [0] = { 19199, 0, -2 },		/* voltage, vrange set */
    [1] = { 6720, 0, -1 },		/* voltage, vrange not set */
    [2] = { 807, 20475, -1 },	/* current */
    };
    static const struct coefficients adm1276_coefficients[] = {
    [0] = { 19199, 0, -2 },		/* voltage, vrange set */
    [1] = { 6720, 0, -1 },		/* voltage, vrange not set */
    [2] = { 807, 20475, -1 },	/* current */
    [3] = { 6043, 0, -2 },		/* power, vrange set */
    [4] = { 2115, 0, -1 },		/* power, vrange not set */
    };
    static const struct coefficients adm1278_coefficients[] = {
    [0] = { 19599, 0, -2 },		/* voltage */
    [1] = { 800, 20475, -1 },	/* current */
    [2] = { 6123, 0, -2 },		/* power */
    [3] = { 42, 31880, -1 },	/* temperature */
    };
    static const struct coefficients adm1293_coefficients[] = {
    [0] = { 3333, -1, 0 },		/* voltage, vrange 1.2V */
    [1] = { 5552, -5, -1 },		/* voltage, vrange 7.4V */
    [2] = { 19604, -50, -2 },	/* voltage, vrange 21V */
    [3] = { 8000, -100, -2 },	/* current, irange25 */
    [4] = { 4000, -100, -2 },	/* current, irange50 */
    [5] = { 20000, -1000, -3 },	/* current, irange100 */
    [6] = { 10000, -1000, -3 },	/* current, irange200 */
    [7] = { 10417, 0, -1 },		/* power, 1.2V, irange25 */
    [8] = { 5208, 0, -1 },		/* power, 1.2V, irange50 */
    [9] = { 26042, 0, -2 },		/* power, 1.2V, irange100 */
    [10] = { 13021, 0, -2 },	/* power, 1.2V, irange200 */
    [11] = { 17351, 0, -2 },	/* power, 7.4V, irange25 */
    [12] = { 8676, 0, -2 },		/* power, 7.4V, irange50 */
    [13] = { 4338, 0, -2 },		/* power, 7.4V, irange100 */
    [14] = { 21689, 0, -3 },	/* power, 7.4V, irange200 */
    [15] = { 6126, 0, -2 },		/* power, 21V, irange25 */
    [16] = { 30631, 0, -3 },	/* power, 21V, irange50 */
    [17] = { 15316, 0, -3 },	/* power, 21V, irange100 */
    [18] = { 7658, 0, -3 },		/* power, 21V, irange200 */
    };
    static int adm1275_read_samples(const struct adm1275_data *data,
    struct i2c_client *client, bool is_power)
    {
    int shift, ret;
    u16 mask;
//
// The PMON configuration register is a 16-bit register only on chips
// supporting power average sampling. On other chips it is an 8-bit
// register.
//
    if (data.have_power_sampling) {
    ret = i2c_smbus_read_word_data(client, ADM1275_PMON_CONFIG);
    mask = is_power ? ADM1278_PWR_AVG_MASK : ADM1278_VI_AVG_MASK;
    shift = is_power ? ADM1278_PWR_AVG_SHIFT : ADM1278_VI_AVG_SHIFT;
    } else {
    ret = i2c_smbus_read_byte_data(client, ADM1275_PMON_CONFIG);
    mask = ADM1275_VI_AVG_MASK;
    shift = ADM1275_VI_AVG_SHIFT;
    }
    if (ret < 0)
    return ret;
    return (ret & mask) >> shift;
    }
    static int adm1275_write_pmon_config(const struct adm1275_data *data,
    struct i2c_client *client, u16 word)
    {
    int ret, ret2;
    ret = i2c_smbus_write_byte_data(client, ADM1275_PMON_CONTROL, 0);
    if (ret)
    return ret;
    if (data.have_power_sampling)
    ret = i2c_smbus_write_word_data(client, ADM1275_PMON_CONFIG,
    word);
    else
    ret = i2c_smbus_write_byte_data(client, ADM1275_PMON_CONFIG,
    word);
//
// We still want to re-enable conversions if writing into
// ADM1275_PMON_CONFIG failed.
//
    ret2 = i2c_smbus_write_byte_data(client, ADM1275_PMON_CONTROL,
    ADM1275_CONVERT_EN);
    if (!ret)
    ret = ret2;
    return ret;
    }
    static int adm1275_write_samples(const struct adm1275_data *data,
    struct i2c_client *client,
    bool is_power, u16 word)
    {
    int shift, ret;
    u16 mask;
    if (data.have_power_sampling) {
    ret = i2c_smbus_read_word_data(client, ADM1275_PMON_CONFIG);
    mask = is_power ? ADM1278_PWR_AVG_MASK : ADM1278_VI_AVG_MASK;
    shift = is_power ? ADM1278_PWR_AVG_SHIFT : ADM1278_VI_AVG_SHIFT;
    } else {
    ret = i2c_smbus_read_byte_data(client, ADM1275_PMON_CONFIG);
    mask = ADM1275_VI_AVG_MASK;
    shift = ADM1275_VI_AVG_SHIFT;
    }
    if (ret < 0)
    return ret;
    word = (ret & ~mask) | ((word << shift) & mask);
    return adm1275_write_pmon_config(data, client, word);
    }
    static int adm1275_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    const struct adm1275_data *data = to_adm1275_data(info);
    let mut ret: c_int = 0;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
    case PMBUS_IOUT_UC_FAULT_LIMIT:
    if (!data.have_uc_fault)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1275_IOUT_WARN2_LIMIT);
    break;
    case PMBUS_IOUT_OC_FAULT_LIMIT:
    if (!data.have_oc_fault)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1275_IOUT_WARN2_LIMIT);
    break;
    case PMBUS_VOUT_OV_WARN_LIMIT:
    if (data.have_vout)
    return -ENODATA;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1075_VAUX_OV_WARN_LIMIT);
    break;
    case PMBUS_VOUT_UV_WARN_LIMIT:
    if (data.have_vout)
    return -ENODATA;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1075_VAUX_UV_WARN_LIMIT);
    break;
    case PMBUS_READ_VOUT:
    if (data.have_vout)
    return -ENODATA;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1075_READ_VAUX);
    break;
    case PMBUS_VIRT_READ_IOUT_MIN:
    if (!data.have_iout_min)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1293_IOUT_MIN);
    break;
    case PMBUS_VIRT_READ_IOUT_MAX:
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1275_PEAK_IOUT);
    break;
    case PMBUS_VIRT_READ_VOUT_MAX:
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1275_PEAK_VOUT);
    break;
    case PMBUS_VIRT_READ_VIN_MAX:
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1275_PEAK_VIN);
    break;
    case PMBUS_VIRT_READ_PIN_MIN:
    if (!data.have_pin_min)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1293_PIN_MIN);
    break;
    case PMBUS_VIRT_READ_PIN_MAX:
    if (!data.have_pin_max)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1276_PEAK_PIN);
    break;
    case PMBUS_VIRT_READ_TEMP_MAX:
    if (!data.have_temp_max)
    return -ENXIO;
    ret = pmbus_read_word_data(client, 0, 0xff,
    ADM1278_PEAK_TEMP);
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    break;
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    if (!data.have_pin_max)
    return -ENXIO;
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    if (!data.have_temp_max)
    return -ENXIO;
    break;
    case PMBUS_VIRT_POWER_SAMPLES:
    if (!data.have_power_sampling)
    return -ENXIO;
    ret = adm1275_read_samples(data, client, true);
    if (ret < 0)
    break;
    ret = BIT(ret);
    break;
    case PMBUS_VIRT_IN_SAMPLES:
    case PMBUS_VIRT_CURR_SAMPLES:
    ret = adm1275_read_samples(data, client, false);
    if (ret < 0)
    break;
    ret = BIT(ret);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int adm1275_write_word_data(struct i2c_client *client, int page, int reg,
    u16 word)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    const struct adm1275_data *data = to_adm1275_data(info);
    int ret;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
    case PMBUS_IOUT_UC_FAULT_LIMIT:
    case PMBUS_IOUT_OC_FAULT_LIMIT:
    ret = pmbus_write_word_data(client, 0, ADM1275_IOUT_WARN2_LIMIT,
    word);
    break;
    case PMBUS_VIRT_RESET_IOUT_HISTORY:
    ret = pmbus_write_word_data(client, 0, ADM1275_PEAK_IOUT, 0);
    if (!ret && data.have_iout_min)
    ret = pmbus_write_word_data(client, 0,
    ADM1293_IOUT_MIN, 0);
    break;
    case PMBUS_VIRT_RESET_VOUT_HISTORY:
    ret = pmbus_write_word_data(client, 0, ADM1275_PEAK_VOUT, 0);
    break;
    case PMBUS_VIRT_RESET_VIN_HISTORY:
    ret = pmbus_write_word_data(client, 0, ADM1275_PEAK_VIN, 0);
    break;
    case PMBUS_VIRT_RESET_PIN_HISTORY:
    ret = pmbus_write_word_data(client, 0, ADM1276_PEAK_PIN, 0);
    if (!ret && data.have_pin_min)
    ret = pmbus_write_word_data(client, 0,
    ADM1293_PIN_MIN, 0);
    break;
    case PMBUS_VIRT_RESET_TEMP_HISTORY:
    ret = pmbus_write_word_data(client, 0, ADM1278_PEAK_TEMP, 0);
    break;
    case PMBUS_VIRT_POWER_SAMPLES:
    if (!data.have_power_sampling)
    return -ENXIO;
    word = clamp_val(word, 1, ADM1275_SAMPLES_AVG_MAX);
    ret = adm1275_write_samples(data, client, true, ilog2(word));
    break;
    case PMBUS_VIRT_IN_SAMPLES:
    case PMBUS_VIRT_CURR_SAMPLES:
    word = clamp_val(word, 1, ADM1275_SAMPLES_AVG_MAX);
    ret = adm1275_write_samples(data, client, false, ilog2(word));
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adm1275_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int adm1275_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    const struct adm1275_data *data = to_adm1275_data(info);
    int mfr_status, ret;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
    case PMBUS_STATUS_IOUT:
    ret = pmbus_read_byte_data(client, page, PMBUS_STATUS_IOUT);
    if (ret < 0)
    break;
    if (!data.have_oc_fault && !data.have_uc_fault)
    break;
    mfr_status = pmbus_read_byte_data(client, page,
    PMBUS_STATUS_MFR_SPECIFIC);
    if (mfr_status < 0)
    return mfr_status;
    if (mfr_status & ADM1275_MFR_STATUS_IOUT_WARN2) {
    ret |= data.have_oc_fault ?
    PB_IOUT_OC_FAULT : PB_IOUT_UC_FAULT;
    }
    break;
    case PMBUS_STATUS_VOUT:
    if (data.have_vout)
    return -ENODATA;
    ret = 0;
    if (data.have_vaux_status) {
    mfr_status = pmbus_read_byte_data(client, 0,
    ADM1075_VAUX_STATUS);
    if (mfr_status < 0)
    return mfr_status;
    if (mfr_status & ADM1075_VAUX_OV_WARN)
    ret |= PB_VOLTAGE_OV_WARNING;
    if (mfr_status & ADM1075_VAUX_UV_WARN)
    ret |= PB_VOLTAGE_UV_WARNING;
    } else if (data.have_mfr_vaux_status) {
    mfr_status = pmbus_read_byte_data(client, page,
    PMBUS_STATUS_MFR_SPECIFIC);
    if (mfr_status < 0)
    return mfr_status;
    if (mfr_status & ADM1293_MFR_STATUS_VAUX_OV_WARN)
    ret |= PB_VOLTAGE_OV_WARNING;
    if (mfr_status & ADM1293_MFR_STATUS_VAUX_UV_WARN)
    ret |= PB_VOLTAGE_UV_WARNING;
    }
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static const struct i2c_device_id adm1275_id[] = {
    { .name = "adm1075", .driver_data = adm1075 },
    { .name = "adm1272", .driver_data = adm1272 },
    { .name = "adm1273", .driver_data = adm1273 },
    { .name = "adm1275", .driver_data = adm1275 },
    { .name = "adm1276", .driver_data = adm1276 },
    { .name = "adm1278", .driver_data = adm1278 },
    { .name = "adm1281", .driver_data = adm1281 },
    { .name = "adm1293", .driver_data = adm1293 },
    { .name = "adm1294", .driver_data = adm1294 },
    { .name = "bd12780", .driver_data = bd12780 },
    { .name = "bd12790", .driver_data = bd12790 },
    { .name = "mc09c", .driver_data = sq24905c },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adm1275_id);
// Enable VOUT & TEMP1 if not enabled (disabled by default)
    static int adm1275_enable_vout_temp(struct adm1275_data *data,
    struct i2c_client *client, int config,
    u16 defconfig)
    {
    int ret;
    if ((config & defconfig) != defconfig) {
    config |= defconfig;
    ret = adm1275_write_pmon_config(data, client, config);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to enable VOUT/TEMP1 monitoring\n");
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm1275_probe(client: *mut i2c_client) -> c_int {
    static int adm1275_probe(struct i2c_client *client)
    {
    s32 (*config_read_fn)(const struct i2c_client *client, u8 reg);
    u8 block_buffer[I2C_SMBUS_BLOCK_MAX + 1] = {0};
    int config, device_config;
    int ret;
    struct pmbus_driver_info *info;
    struct adm1275_data *data;
    const struct i2c_device_id *mid;
    const struct coefficients *coefficients;
    let mut vindex: c_int = -1, voindex = -1, cindex = -1, pindex = -1;
    let mut tindex: c_int = -1;
    u32 shunt;
    u32 avg;
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_MFR_ID, block_buffer);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer ID\n");
    return ret;
    }
    if ((ret != 3 || strncmp(block_buffer, "ADI", 3)) &&
    (ret != 2 || strncmp(block_buffer, "SY", 2)) &&
    (ret != 4 || strncmp(block_buffer, "ROHM", 4))) {
    dev_err(&client.dev, "Unsupported Manufacturer ID\n");
    return -ENODEV;
    }
    ret = pmbus_read_smbus_i2c_block_data(client, PMBUS_MFR_MODEL, block_buffer);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer Model\n");
    return ret;
    }
    for (mid = adm1275_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, block_buffer, strlen(mid.name)))
    break;
    }
    if (!mid.name[0]) {
    dev_err(&client.dev, "Unsupported device\n");
    return -ENODEV;
    }
    if (strcmp(client.name, mid.name) != 0)
    dev_notice(&client.dev,
    "Device mismatch: Configured %s, detected %s\n",
    client.name, mid.name);
    if (mid.driver_data == adm1272 || mid.driver_data == adm1273 ||
    mid.driver_data == adm1278 || mid.driver_data == adm1281 ||
    mid.driver_data == adm1293 || mid.driver_data == adm1294 ||
    mid.driver_data == bd12780 || mid.driver_data == bd12790 ||
    mid.driver_data == sq24905c)
    config_read_fn = i2c_smbus_read_word_data;
    else
    config_read_fn = i2c_smbus_read_byte_data;
    config = config_read_fn(client, ADM1275_PMON_CONFIG);
    if (config < 0)
    return config;
    device_config = config_read_fn(client, ADM1275_DEVICE_CONFIG);
    if (device_config < 0)
    return device_config;
    data = devm_kzalloc(&client.dev, sizeof(struct adm1275_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    if (of_property_read_u32(client.dev.of_node,
    "shunt-resistor-micro-ohms", &shunt))
    shunt = 1000; /* 1 mOhm if not set via DT */
    if (shunt == 0)
    return -EINVAL;
    data.id = mid.driver_data;
    info = &data.info;
    info.pages = 1;
    info.format[PSC_VOLTAGE_IN] = direct;
    info.format[PSC_VOLTAGE_OUT] = direct;
    info.format[PSC_CURRENT_OUT] = direct;
    info.format[PSC_POWER] = direct;
    info.format[PSC_TEMPERATURE] = direct;
    info.func[0] = PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_SAMPLES;
    info.read_word_data = adm1275_read_word_data;
    info.read_byte_data = adm1275_read_byte_data;
    info.write_word_data = adm1275_write_word_data;
    switch (data.id) {
    case adm1075:
    if (device_config & ADM1275_IOUT_WARN2_SELECT)
    data.have_oc_fault = true;
    else
    data.have_uc_fault = true;
    data.have_pin_max = true;
    data.have_vaux_status = true;
    coefficients = adm1075_coefficients;
    vindex = 0;
    switch (config & ADM1075_IRANGE_MASK) {
    case ADM1075_IRANGE_25:
    cindex = 1;
    pindex = 3;
    break;
    case ADM1075_IRANGE_50:
    cindex = 2;
    pindex = 4;
    break;
    default:
    dev_err(&client.dev, "Invalid input current range");
    break;
    }
    info.func[0] |= PMBUS_HAVE_VIN | PMBUS_HAVE_PIN
    | PMBUS_HAVE_STATUS_INPUT;
    if (config & ADM1275_VIN_VOUT_SELECT)
    info.func[0] |=
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT;
    break;
    case adm1272:
    case adm1273:
    data.have_vout = true;
    data.have_pin_max = true;
    data.have_temp_max = true;
    data.have_power_sampling = true;
    coefficients = adm1272_coefficients;
    vindex = (config & ADM1275_VRANGE) ? 1 : 0;
    cindex = (config & ADM1272_IRANGE) ? 3 : 2;
// pindex depends on the combination of the above
    switch (config & (ADM1275_VRANGE | ADM1272_IRANGE)) {
    case 0:
    default:
    pindex = 4;
    break;
    case ADM1275_VRANGE:
    pindex = 5;
    break;
    case ADM1272_IRANGE:
    pindex = 6;
    break;
    case ADM1275_VRANGE | ADM1272_IRANGE:
    pindex = 7;
    break;
    }
    tindex = 8;
    info.func[0] |= PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    ret = adm1275_enable_vout_temp(data, client, config,
    ADM1278_PMON_DEFCONFIG);
    if (ret)
    return ret;
    if (config & ADM1278_VIN_EN)
    info.func[0] |= PMBUS_HAVE_VIN;
    break;
//
// The BD12790 is almost identical to the adm1272. Only the defconfig
// and coefficients have minor differences.
//
    case bd12790:
    data.have_vout = true;
    data.have_pin_max = true;
    data.have_temp_max = true;
    data.have_power_sampling = true;
    coefficients = bd12790_coefficients;
    vindex = (config & ADM1275_VRANGE) ? 1 : 0;
    cindex = (config & ADM1272_IRANGE) ? 3 : 2;
// pindex depends on the combination of the above
    switch (config & (ADM1275_VRANGE | ADM1272_IRANGE)) {
    case 0:
    default:
    pindex = 4;
    break;
    case ADM1275_VRANGE:
    pindex = 5;
    break;
    case ADM1272_IRANGE:
    pindex = 6;
    break;
    case ADM1275_VRANGE | ADM1272_IRANGE:
    pindex = 7;
    break;
    }
    tindex = 8;
    info.func[0] |= PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    ret = adm1275_enable_vout_temp(data, client, config,
    BD12780_PMON_DEFCONFIG);
    if (ret)
    return ret;
    if (config & ADM1278_VIN_EN)
    info.func[0] |= PMBUS_HAVE_VIN;
    break;
    case adm1275:
    if (device_config & ADM1275_IOUT_WARN2_SELECT)
    data.have_oc_fault = true;
    else
    data.have_uc_fault = true;
    data.have_vout = true;
    coefficients = adm1275_coefficients;
    vindex = (config & ADM1275_VRANGE) ? 0 : 1;
    cindex = 2;
    if (config & ADM1275_VIN_VOUT_SELECT)
    info.func[0] |=
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT;
    else
    info.func[0] |=
    PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT;
    break;
    case adm1276:
    if (device_config & ADM1275_IOUT_WARN2_SELECT)
    data.have_oc_fault = true;
    else
    data.have_uc_fault = true;
    data.have_vout = true;
    data.have_pin_max = true;
    coefficients = adm1276_coefficients;
    vindex = (config & ADM1275_VRANGE) ? 0 : 1;
    cindex = 2;
    pindex = (config & ADM1275_VRANGE) ? 3 : 4;
    info.func[0] |= PMBUS_HAVE_VIN | PMBUS_HAVE_PIN
    | PMBUS_HAVE_STATUS_INPUT;
    if (config & ADM1275_VIN_VOUT_SELECT)
    info.func[0] |=
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT;
    break;
    case adm1278:
    case adm1281:
    case sq24905c:
    data.have_vout = true;
    data.have_pin_max = true;
    data.have_temp_max = true;
    data.have_power_sampling = true;
    coefficients = adm1278_coefficients;
    vindex = 0;
    cindex = 1;
    pindex = 2;
    tindex = 3;
    info.func[0] |= PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    ret = adm1275_enable_vout_temp(data, client, config,
    ADM1278_PMON_DEFCONFIG);
    if (ret)
    return ret;
    if (config & ADM1278_VIN_EN)
    info.func[0] |= PMBUS_HAVE_VIN;
    break;
//
// The BD12780 is almost functionally identical with the adm1278 above.
// Only differences visible to the driver are lack of TSFILT bits and
// different identification register contents.
//
    case bd12780:
    data.have_vout = true;
    data.have_pin_max = true;
    data.have_temp_max = true;
    data.have_power_sampling = true;
    coefficients = adm1278_coefficients;
    vindex = 0;
    cindex = 1;
    pindex = 2;
    tindex = 3;
    info.func[0] |= PMBUS_HAVE_PIN | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP;
    ret = adm1275_enable_vout_temp(data, client, config,
    BD12780_PMON_DEFCONFIG);
    if (ret)
    return ret;
    if (config & ADM1278_VIN_EN)
    info.func[0] |= PMBUS_HAVE_VIN;
    break;
    case adm1293:
    case adm1294:
    data.have_iout_min = true;
    data.have_pin_min = true;
    data.have_pin_max = true;
    data.have_mfr_vaux_status = true;
    data.have_power_sampling = true;
    coefficients = adm1293_coefficients;
    voindex = 0;
    switch (config & ADM1293_VIN_SEL_MASK) {
    case ADM1293_VIN_SEL_012:	/* 1.2V */
    vindex = 0;
    break;
    case ADM1293_VIN_SEL_074:	/* 7.4V */
    vindex = 1;
    break;
    case ADM1293_VIN_SEL_210:	/* 21V */
    vindex = 2;
    break;
    default:			/* disabled */
    break;
    }
    switch (config & ADM1293_IRANGE_MASK) {
    case ADM1293_IRANGE_25:
    cindex = 3;
    break;
    case ADM1293_IRANGE_50:
    cindex = 4;
    break;
    case ADM1293_IRANGE_100:
    cindex = 5;
    break;
    case ADM1293_IRANGE_200:
    cindex = 6;
    break;
    }
    if (vindex >= 0)
    pindex = 7 + vindex * 4 + (cindex - 3);
    if (config & ADM1293_VAUX_EN)
    info.func[0] |=
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT;
    info.func[0] |= PMBUS_HAVE_PIN |
    PMBUS_HAVE_VIN | PMBUS_HAVE_STATUS_INPUT;
    break;
    default:
    dev_err(&client.dev, "Unsupported device\n");
    return -ENODEV;
    }
    if (data.have_power_sampling &&
    of_property_read_u32(client.dev.of_node,
    "adi,power-sample-average", &avg) == 0) {
    if (!avg || avg > ADM1275_SAMPLES_AVG_MAX ||
    BIT(__fls(avg)) != avg) {
    dev_err(&client.dev,
    "Invalid number of power samples");
    return -EINVAL;
    }
    ret = adm1275_write_samples(data, client, true, ilog2(avg));
    if (ret < 0) {
    dev_err(&client.dev,
    "Setting power sample averaging failed with error %d",
    ret);
    return ret;
    }
    }
    if (of_property_read_u32(client.dev.of_node,
    "adi,volt-curr-sample-average", &avg) == 0) {
    if (!avg || avg > ADM1275_SAMPLES_AVG_MAX ||
    BIT(__fls(avg)) != avg) {
    dev_err(&client.dev,
    "Invalid number of voltage/current samples");
    return -EINVAL;
    }
    ret = adm1275_write_samples(data, client, false, ilog2(avg));
    if (ret < 0) {
    dev_err(&client.dev,
    "Setting voltage and current sample averaging failed with error %d",
    ret);
    return ret;
    }
    }
    if (voindex < 0)
    voindex = vindex;
    if (vindex >= 0) {
    info.m[PSC_VOLTAGE_IN] = coefficients[vindex].m;
    info.b[PSC_VOLTAGE_IN] = coefficients[vindex].b;
    info.R[PSC_VOLTAGE_IN] = coefficients[vindex].R;
    }
    if (voindex >= 0) {
    info.m[PSC_VOLTAGE_OUT] = coefficients[voindex].m;
    info.b[PSC_VOLTAGE_OUT] = coefficients[voindex].b;
    info.R[PSC_VOLTAGE_OUT] = coefficients[voindex].R;
    }
    if (cindex >= 0) {
    u32 m;
// Scale current with sense resistor value
    if (unlikely(check_mul_overflow(coefficients[cindex].m, shunt, &m))) {
    dev_err(&client.dev, "Current coefficient overflow\n");
    return -EOVERFLOW;
    }
    info.m[PSC_CURRENT_OUT] = m / 1000;
    info.b[PSC_CURRENT_OUT] = coefficients[cindex].b;
    info.R[PSC_CURRENT_OUT] = coefficients[cindex].R;
    }
    if (pindex >= 0) {
    u32 m;
    if (unlikely(check_mul_overflow(coefficients[pindex].m, shunt, &m))) {
    dev_err(&client.dev, "Power coefficient overflow\n");
    return -EOVERFLOW;
    }
    info.m[PSC_POWER] = m / 1000;
    info.b[PSC_POWER] = coefficients[pindex].b;
    info.R[PSC_POWER] = coefficients[pindex].R;
    }
    if (tindex >= 0) {
    info.m[PSC_TEMPERATURE] = coefficients[tindex].m;
    info.b[PSC_TEMPERATURE] = coefficients[tindex].b;
    info.R[PSC_TEMPERATURE] = coefficients[tindex].R;
    }
    return pmbus_do_probe(client, info);
    }
    static const struct of_device_id adm1275_of_match[] = {
    { .compatible = "adi,adm1075", },
    { .compatible = "adi,adm1272", },
    { .compatible = "adi,adm1273", },
    { .compatible = "adi,adm1275", },
    { .compatible = "adi,adm1276", },
    { .compatible = "adi,adm1278", },
    { .compatible = "adi,adm1281", },
    { .compatible = "adi,adm1293", },
    { .compatible = "adi,adm1294", },
    { .compatible = "rohm,bd12780", },
    { .compatible = "rohm,bd12790", },
    { .compatible = "silergy,mc09c", },
    { }
    };
    MODULE_DEVICE_TABLE(of, adm1275_of_match);
    static struct i2c_driver adm1275_driver = {
    .driver = {
    .name = "adm1275",
    .of_match_table = adm1275_of_match,
    },
    .probe = adm1275_probe,
    .id_table = adm1275_id,
    };
    module_i2c_driver(adm1275_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("PMBus driver for Analog Devices ADM1275 and compatibles");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
