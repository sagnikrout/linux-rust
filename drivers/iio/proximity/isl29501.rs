//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/isl29501.c
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
// isl29501.c: ISL29501 Time of Flight sensor driver.
//
// Copyright (C) 2018
// Author: Mathieu Othacehe <m.othacehe@gmail.com>
//
// 7-bit I2C slave address: 0x57
//

// Control, setting and status registers
pub const ISL29501_DEVICE_ID: c_uint = 0x00;
pub const ISL29501_ID: c_uint = 0x0A;
// Sampling control registers
pub const ISL29501_INTEGRATION_PERIOD: c_uint = 0x10;
pub const ISL29501_SAMPLE_PERIOD: c_uint = 0x11;
// Closed loop calibration registers
pub const ISL29501_CROSSTALK_I_MSB: c_uint = 0x24;
pub const ISL29501_CROSSTALK_I_LSB: c_uint = 0x25;
pub const ISL29501_CROSSTALK_I_EXPONENT: c_uint = 0x26;
pub const ISL29501_CROSSTALK_Q_MSB: c_uint = 0x27;
pub const ISL29501_CROSSTALK_Q_LSB: c_uint = 0x28;
pub const ISL29501_CROSSTALK_Q_EXPONENT: c_uint = 0x29;
pub const ISL29501_CROSSTALK_GAIN_MSB: c_uint = 0x2A;
pub const ISL29501_CROSSTALK_GAIN_LSB: c_uint = 0x2B;
pub const ISL29501_MAGNITUDE_REF_EXP: c_uint = 0x2C;
pub const ISL29501_MAGNITUDE_REF_MSB: c_uint = 0x2D;
pub const ISL29501_MAGNITUDE_REF_LSB: c_uint = 0x2E;
pub const ISL29501_PHASE_OFFSET_MSB: c_uint = 0x2F;
pub const ISL29501_PHASE_OFFSET_LSB: c_uint = 0x30;
// Analog control registers
pub const ISL29501_DRIVER_RANGE: c_uint = 0x90;
pub const ISL29501_EMITTER_DAC: c_uint = 0x91;
pub const ISL29501_COMMAND_REGISTER: c_uint = 0xB0;
// Commands
pub const ISL29501_EMUL_SAMPLE_START_PIN: c_uint = 0x49;
pub const ISL29501_RESET_ALL_REGISTERS: c_uint = 0xD7;
pub const ISL29501_RESET_INT_SM: c_uint = 0xD1;
// Ambiant light and temperature corrections
pub const ISL29501_TEMP_REFERENCE: c_uint = 0x31;
pub const ISL29501_PHASE_EXPONENT: c_uint = 0x33;
pub const ISL29501_TEMP_COEFF_A: c_uint = 0x34;
pub const ISL29501_TEMP_COEFF_B: c_uint = 0x39;
pub const ISL29501_AMBIANT_COEFF_A: c_uint = 0x36;
pub const ISL29501_AMBIANT_COEFF_B: c_uint = 0x3B;
// Data output registers
pub const ISL29501_DISTANCE_MSB_DATA: c_uint = 0xD1;
pub const ISL29501_DISTANCE_LSB_DATA: c_uint = 0xD2;
pub const ISL29501_PRECISION_MSB: c_uint = 0xD3;
pub const ISL29501_PRECISION_LSB: c_uint = 0xD4;
pub const ISL29501_MAGNITUDE_EXPONENT: c_uint = 0xD5;
pub const ISL29501_MAGNITUDE_MSB: c_uint = 0xD6;
pub const ISL29501_MAGNITUDE_LSB: c_uint = 0xD7;
pub const ISL29501_PHASE_MSB: c_uint = 0xD8;
pub const ISL29501_PHASE_LSB: c_uint = 0xD9;
pub const ISL29501_I_RAW_EXPONENT: c_uint = 0xDA;
pub const ISL29501_I_RAW_MSB: c_uint = 0xDB;
pub const ISL29501_I_RAW_LSB: c_uint = 0xDC;
pub const ISL29501_Q_RAW_EXPONENT: c_uint = 0xDD;
pub const ISL29501_Q_RAW_MSB: c_uint = 0xDE;
pub const ISL29501_Q_RAW_LSB: c_uint = 0xDF;
pub const ISL29501_DIE_TEMPERATURE: c_uint = 0xE2;
pub const ISL29501_AMBIENT_LIGHT: c_uint = 0xE3;
pub const ISL29501_GAIN_MSB: c_uint = 0xE6;
pub const ISL29501_GAIN_LSB: c_uint = 0xE7;
pub const ISL29501_MAX_EXP_VAL: c_int = 15;

    "0.00007 0.00014 0.00028 0.00057 0.00114 " \
    "0.00228 0.00455 0.00910 0.01820 0.03640 " \
    "0.07281 0.14561"

    "0.0039 0.0078 0.0118 0.0157 0.0196 " \
    "0.0235 0.0275 0.0314 0.0352 0.0392 " \
    "0.0431 0.0471 0.0510 0.0549 0.0588"
    enum isl29501_correction_coeff {
    COEFF_TEMP_A,
    COEFF_TEMP_B,
    COEFF_LIGHT_A,
    COEFF_LIGHT_B,
    COEFF_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29501_private {
    pub client: *mut i2c_client,
    pub lock: mutex,
// Exact representation of correction coefficients.
    pub shadow_coeffs: [c_uint; COEFF_MAX],
}

    enum isl29501_register_name {
    REG_DISTANCE,
    REG_PHASE,
    REG_TEMPERATURE,
    REG_AMBIENT_LIGHT,
    REG_GAIN,
    REG_GAIN_BIAS,
    REG_PHASE_EXP,
    REG_CALIB_PHASE_TEMP_A,
    REG_CALIB_PHASE_TEMP_B,
    REG_CALIB_PHASE_LIGHT_A,
    REG_CALIB_PHASE_LIGHT_B,
    REG_DISTANCE_BIAS,
    REG_TEMPERATURE_BIAS,
    REG_INT_TIME,
    REG_SAMPLE_TIME,
    REG_DRIVER_RANGE,
    REG_EMITTER_DAC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl29501_register_desc {
    pub msb: u8,
    pub lsb: u8,
}

    static const struct isl29501_register_desc isl29501_registers[] = {
    [REG_DISTANCE] = {
    .msb = ISL29501_DISTANCE_MSB_DATA,
    .lsb = ISL29501_DISTANCE_LSB_DATA,
    },
    [REG_PHASE] = {
    .msb = ISL29501_PHASE_MSB,
    .lsb = ISL29501_PHASE_LSB,
    },
    [REG_TEMPERATURE] = {
    .lsb = ISL29501_DIE_TEMPERATURE,
    },
    [REG_AMBIENT_LIGHT] = {
    .lsb = ISL29501_AMBIENT_LIGHT,
    },
    [REG_GAIN] = {
    .msb = ISL29501_GAIN_MSB,
    .lsb = ISL29501_GAIN_LSB,
    },
    [REG_GAIN_BIAS] = {
    .msb = ISL29501_CROSSTALK_GAIN_MSB,
    .lsb = ISL29501_CROSSTALK_GAIN_LSB,
    },
    [REG_PHASE_EXP] = {
    .lsb = ISL29501_PHASE_EXPONENT,
    },
    [REG_CALIB_PHASE_TEMP_A] = {
    .lsb = ISL29501_TEMP_COEFF_A,
    },
    [REG_CALIB_PHASE_TEMP_B] = {
    .lsb = ISL29501_TEMP_COEFF_B,
    },
    [REG_CALIB_PHASE_LIGHT_A] = {
    .lsb = ISL29501_AMBIANT_COEFF_A,
    },
    [REG_CALIB_PHASE_LIGHT_B] = {
    .lsb = ISL29501_AMBIANT_COEFF_B,
    },
    [REG_DISTANCE_BIAS] = {
    .msb = ISL29501_PHASE_OFFSET_MSB,
    .lsb = ISL29501_PHASE_OFFSET_LSB,
    },
    [REG_TEMPERATURE_BIAS] = {
    .lsb = ISL29501_TEMP_REFERENCE,
    },
    [REG_INT_TIME] = {
    .lsb = ISL29501_INTEGRATION_PERIOD,
    },
    [REG_SAMPLE_TIME] = {
    .lsb = ISL29501_SAMPLE_PERIOD,
    },
    [REG_DRIVER_RANGE] = {
    .lsb = ISL29501_DRIVER_RANGE,
    },
    [REG_EMITTER_DAC] = {
    .lsb = ISL29501_EMITTER_DAC,
    },
    };
    static int isl29501_register_read(struct isl29501_private *isl29501,
    enum isl29501_register_name name,
    u32 *val)
    {
    const struct isl29501_register_desc *reg = &isl29501_registers[name];
    let mut msb: u8 = 0, lsb = 0;
    s32 ret;
    mutex_lock(&isl29501.lock);
    if (reg.msb) {
    ret = i2c_smbus_read_byte_data(isl29501.client, reg.msb);
    if (ret < 0)
    goto err;
    msb = ret;
    }
    if (reg.lsb) {
    ret = i2c_smbus_read_byte_data(isl29501.client, reg.lsb);
    if (ret < 0)
    goto err;
    lsb = ret;
    }
    mutex_unlock(&isl29501.lock);
// val = (msb << 8) + lsb;
    return 0;
    err:
    mutex_unlock(&isl29501.lock);
    return ret;
    }
    static u32 isl29501_register_write(struct isl29501_private *isl29501,
    enum isl29501_register_name name,
    u32 value)
    {
    const struct isl29501_register_desc *reg = &isl29501_registers[name];
    int ret;
    if (!reg.msb && value > U8_MAX)
    return -ERANGE;
    if (value > U16_MAX)
    return -ERANGE;
    mutex_lock(&isl29501.lock);
    if (reg.msb) {
    ret = i2c_smbus_write_byte_data(isl29501.client,
    reg.msb, value >> 8);
    if (ret < 0)
    goto err;
    }
    ret = i2c_smbus_write_byte_data(isl29501.client, reg.lsb, value);
    err:
    mutex_unlock(&isl29501.lock);
    return ret;
    }
    static ssize_t isl29501_read_ext(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct isl29501_private *isl29501 = iio_priv(indio_dev);
    let mut reg: enum isl29501_register_name = private;
    int ret;
    u32 value, gain, coeff, exp;
    switch (reg) {
    case REG_GAIN:
    case REG_GAIN_BIAS:
    ret = isl29501_register_read(isl29501, reg, &gain);
    if (ret < 0)
    return ret;
    value = gain;
    break;
    case REG_CALIB_PHASE_TEMP_A:
    case REG_CALIB_PHASE_TEMP_B:
    case REG_CALIB_PHASE_LIGHT_A:
    case REG_CALIB_PHASE_LIGHT_B:
    ret = isl29501_register_read(isl29501, REG_PHASE_EXP, &exp);
    if (ret < 0)
    return ret;
    ret = isl29501_register_read(isl29501, reg, &coeff);
    if (ret < 0)
    return ret;
    value = coeff << exp;
    break;
    default:
    return -EINVAL;
    }
    return sprintf(buf, "%u\n", value);
    }
    static int isl29501_set_shadow_coeff(struct isl29501_private *isl29501,
    enum isl29501_register_name reg,
    unsigned int val)
    {
    enum isl29501_correction_coeff coeff;
    switch (reg) {
    case REG_CALIB_PHASE_TEMP_A:
    coeff = COEFF_TEMP_A;
    break;
    case REG_CALIB_PHASE_TEMP_B:
    coeff = COEFF_TEMP_B;
    break;
    case REG_CALIB_PHASE_LIGHT_A:
    coeff = COEFF_LIGHT_A;
    break;
    case REG_CALIB_PHASE_LIGHT_B:
    coeff = COEFF_LIGHT_B;
    break;
    default:
    return -EINVAL;
    }
    isl29501.shadow_coeffs[coeff] = val;
    return 0;
    }
    static int isl29501_write_coeff(struct isl29501_private *isl29501,
    enum isl29501_correction_coeff coeff,
    int val)
    {
    enum isl29501_register_name reg;
    switch (coeff) {
    case COEFF_TEMP_A:
    reg = REG_CALIB_PHASE_TEMP_A;
    break;
    case COEFF_TEMP_B:
    reg = REG_CALIB_PHASE_TEMP_B;
    break;
    case COEFF_LIGHT_A:
    reg = REG_CALIB_PHASE_LIGHT_A;
    break;
    case COEFF_LIGHT_B:
    reg = REG_CALIB_PHASE_LIGHT_B;
    break;
    default:
    return -EINVAL;
    }
    return isl29501_register_write(isl29501, reg, val);
    }
    static unsigned int isl29501_find_corr_exp(unsigned int val,
    unsigned int max_exp,
    unsigned int max_mantissa)
    {
    let mut exp: c_uint = 1;
//
// Correction coefficients are represented under
// mantissa * 2^exponent form, where mantissa and exponent
// are stored in two separate registers of the sensor.
//
// Compute and return the lowest exponent such as:
// mantissa = value / 2^exponent
//
// where mantissa < max_mantissa.
//
    if (val <= max_mantissa)
    return 0;
    while ((val >> exp) > max_mantissa) {
    exp++;
    if (exp > max_exp)
    return max_exp;
    }
    return exp;
    }
    static ssize_t isl29501_write_ext(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct isl29501_private *isl29501 = iio_priv(indio_dev);
    let mut reg: enum isl29501_register_name = private;
    unsigned int val;
    let mut max_exp: c_int = 0;
    int ret;
    int i;
    ret = kstrtouint(buf, 10, &val);
    if (ret)
    return ret;
    switch (reg) {
    case REG_GAIN_BIAS:
    if (val > U16_MAX)
    return -ERANGE;
    ret = isl29501_register_write(isl29501, reg, val);
    if (ret < 0)
    return ret;
    break;
    case REG_CALIB_PHASE_TEMP_A:
    case REG_CALIB_PHASE_TEMP_B:
    case REG_CALIB_PHASE_LIGHT_A:
    case REG_CALIB_PHASE_LIGHT_B:
    if (val > (U8_MAX << ISL29501_MAX_EXP_VAL))
    return -ERANGE;
// Store the correction coefficient under its exact form.
    ret = isl29501_set_shadow_coeff(isl29501, reg, val);
    if (ret < 0)
    return ret;
//
// Find the highest exponent needed to represent
// correction coefficients.
//
    for (i = 0; i < COEFF_MAX; i++) {
    int corr;
    int corr_exp;
    corr = isl29501.shadow_coeffs[i];
    corr_exp = isl29501_find_corr_exp(corr,
    ISL29501_MAX_EXP_VAL,
    U8_MAX / 2);
    dev_dbg(&isl29501.client.dev,
    "found exp of corr(%d) = %d\n", corr, corr_exp);
    max_exp = max(max_exp, corr_exp);
    }
//
// Represent every correction coefficient under
// mantissa * 2^max_exponent form and force the
// writing of those coefficients on the sensor.
//
    for (i = 0; i < COEFF_MAX; i++) {
    int corr;
    int mantissa;
    corr = isl29501.shadow_coeffs[i];
    if (!corr)
    continue;
    mantissa = corr >> max_exp;
    ret = isl29501_write_coeff(isl29501, i, mantissa);
    if (ret < 0)
    return ret;
    }
    ret = isl29501_register_write(isl29501, REG_PHASE_EXP, max_exp);
    if (ret < 0)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    return len;
    }

    .name = _name, \
    .read = isl29501_read_ext, \
    .write = isl29501_write_ext, \
    .private = _ident, \
    .shared = IIO_SEPARATE, \
    }
    static const struct iio_chan_spec_ext_info isl29501_ext_info[] = {
    _ISL29501_EXT_INFO("agc_gain", REG_GAIN),
    _ISL29501_EXT_INFO("agc_gain_bias", REG_GAIN_BIAS),
    _ISL29501_EXT_INFO("calib_phase_temp_a", REG_CALIB_PHASE_TEMP_A),
    _ISL29501_EXT_INFO("calib_phase_temp_b", REG_CALIB_PHASE_TEMP_B),
    _ISL29501_EXT_INFO("calib_phase_light_a", REG_CALIB_PHASE_LIGHT_A),
    _ISL29501_EXT_INFO("calib_phase_light_b", REG_CALIB_PHASE_LIGHT_B),
    { }
    };
pub const ISL29501_DISTANCE_SCAN_INDEX: c_int = 0;
pub const ISL29501_TIMESTAMP_SCAN_INDEX: c_int = 1;
    static const struct iio_chan_spec isl29501_channels[] = {
    {
    .type = IIO_PROXIMITY,
    .scan_index = ISL29501_DISTANCE_SCAN_INDEX,
    .info_mask_separate =
    BIT(IIO_CHAN_INFO_RAW)   |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_CALIBBIAS),
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_CPU,
    },
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME) |
    BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .ext_info = isl29501_ext_info,
    },
    {
    .type = IIO_PHASE,
    .scan_index = -1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    {
    .type = IIO_CURRENT,
    .scan_index = -1,
    .output = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    {
    .type = IIO_TEMP,
    .scan_index = -1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE)     |
    BIT(IIO_CHAN_INFO_CALIBBIAS),
    },
    {
    .type = IIO_INTENSITY,
    .scan_index = -1,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_CLEAR,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    IIO_CHAN_SOFT_TIMESTAMP(ISL29501_TIMESTAMP_SCAN_INDEX),
    };
#[no_mangle]
unsafe extern "C" fn isl29501_reset_registers(isl29501: *mut isl29501_private) -> c_int {
    static int isl29501_reset_registers(struct isl29501_private *isl29501)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(isl29501.client,
    ISL29501_COMMAND_REGISTER,
    ISL29501_RESET_ALL_REGISTERS);
    if (ret < 0) {
    dev_err(&isl29501.client.dev,
    "cannot reset registers %d\n", ret);
    return ret;
    }
    ret = i2c_smbus_write_byte_data(isl29501.client,
    ISL29501_COMMAND_REGISTER,
    ISL29501_RESET_INT_SM);
    if (ret < 0)
    dev_err(&isl29501.client.dev,
    "cannot reset state machine %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn isl29501_begin_acquisition(isl29501: *mut isl29501_private) -> c_int {
    static int isl29501_begin_acquisition(struct isl29501_private *isl29501)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(isl29501.client,
    ISL29501_COMMAND_REGISTER,
    ISL29501_EMUL_SAMPLE_START_PIN);
    if (ret < 0)
    dev_err(&isl29501.client.dev,
    "cannot begin acquisition %d\n", ret);
    return ret;
    }
    static IIO_CONST_ATTR_INT_TIME_AVAIL(ISL29501_INT_TIME_AVAILABLE);
    static IIO_CONST_ATTR(out_current_scale_available,
    ISL29501_CURRENT_SCALE_AVAILABLE);
    static struct attribute *isl29501_attributes[] = {
    &iio_const_attr_integration_time_available.dev_attr.attr,
    &iio_const_attr_out_current_scale_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group isl29501_attribute_group = {
    .attrs = isl29501_attributes,
    };
    static const int isl29501_current_scale_table[][2] = {
    {0, 3900}, {0, 7800}, {0, 11800}, {0, 15700},
    {0, 19600}, {0, 23500}, {0, 27500}, {0, 31400},
    {0, 35200}, {0, 39200}, {0, 43100}, {0, 47100},
    {0, 51000}, {0, 54900}, {0, 58800},
    };
    static const int isl29501_int_time[][2] = {
    {0, 70},    /* 0.07 ms */
    {0, 140},   /* 0.14 ms */
    {0, 280},   /* 0.28 ms */
    {0, 570},   /* 0.57 ms */
    {0, 1140},  /* 1.14 ms */
    {0, 2280},  /* 2.28 ms */
    {0, 4550},  /* 4.55 ms */
    {0, 9100},  /* 9.11 ms */
    {0, 18200}, /* 18.2 ms */
    {0, 36400}, /* 36.4 ms */
    {0, 72810}, /* 72.81 ms */
    {0, 145610} /* 145.28 ms */
    };
    static int isl29501_get_raw(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int *raw)
    {
    int ret;
    switch (chan.type) {
    case IIO_PROXIMITY:
    ret = isl29501_register_read(isl29501, REG_DISTANCE, raw);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_INTENSITY:
    ret = isl29501_register_read(isl29501,
    REG_AMBIENT_LIGHT,
    raw);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_PHASE:
    ret = isl29501_register_read(isl29501, REG_PHASE, raw);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CURRENT:
    ret = isl29501_register_read(isl29501, REG_EMITTER_DAC, raw);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_TEMP:
    ret = isl29501_register_read(isl29501, REG_TEMPERATURE, raw);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int isl29501_get_scale(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int *val, int *val2)
    {
    int ret;
    u32 current_scale;
    switch (chan.type) {
    case IIO_PROXIMITY:
// distance = raw_distance * 33.31 / 65536 (m)
// val = 3331;
// val2 = 6553600;
    return IIO_VAL_FRACTIONAL;
    case IIO_PHASE:
// phase = raw_phase * 2pi / 65536 (rad)
// val = 0;
// val2 = 95874;
    return IIO_VAL_INT_PLUS_NANO;
    case IIO_INTENSITY:
// light = raw_light * 35 / 10000 (mA)
// val = 35;
// val2 = 10000;
    return IIO_VAL_FRACTIONAL;
    case IIO_CURRENT:
    ret = isl29501_register_read(isl29501,
    REG_DRIVER_RANGE,
    &current_scale);
    if (ret < 0)
    return ret;
    if (current_scale > ARRAY_SIZE(isl29501_current_scale_table))
    return -EINVAL;
    if (!current_scale) {
// val = 0;
// val2 = 0;
    return IIO_VAL_INT;
    }
// val = isl29501_current_scale_table[current_scale - 1][0];
// val2 = isl29501_current_scale_table[current_scale - 1][1];
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_TEMP:
// temperature = raw_temperature * 125 / 100000 (milli °C)
// val = 125;
// val2 = 100000;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    }
    static int isl29501_get_calibbias(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int *bias)
    {
    switch (chan.type) {
    case IIO_PROXIMITY:
    return isl29501_register_read(isl29501,
    REG_DISTANCE_BIAS,
    bias);
    case IIO_TEMP:
    return isl29501_register_read(isl29501,
    REG_TEMPERATURE_BIAS,
    bias);
    default:
    return -EINVAL;
    }
    }
    static int isl29501_get_inttime(struct isl29501_private *isl29501,
    int *val, int *val2)
    {
    int ret;
    u32 inttime;
    ret = isl29501_register_read(isl29501, REG_INT_TIME, &inttime);
    if (ret < 0)
    return ret;
    if (inttime >= ARRAY_SIZE(isl29501_int_time))
    return -EINVAL;
// val = isl29501_int_time[inttime][0];
// val2 = isl29501_int_time[inttime][1];
    return IIO_VAL_INT_PLUS_MICRO;
    }
    static int isl29501_get_freq(struct isl29501_private *isl29501,
    int *val, int *val2)
    {
    int ret;
    int sample_time;
    unsigned long long freq;
    u32 temp;
    ret = isl29501_register_read(isl29501, REG_SAMPLE_TIME, &sample_time);
    if (ret < 0)
    return ret;
// freq = 1 / (0.000450 * (sample_time + 1) * 10^-6)
    freq = 1000000ULL * 1000000ULL;
    do_div(freq, 450 * (sample_time + 1));
    temp = do_div(freq, 1000000);
// val = freq;
// val2 = temp;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    static int isl29501_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct isl29501_private *isl29501 = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return isl29501_get_raw(isl29501, chan, val);
    case IIO_CHAN_INFO_SCALE:
    return isl29501_get_scale(isl29501, chan, val, val2);
    case IIO_CHAN_INFO_INT_TIME:
    return isl29501_get_inttime(isl29501, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return isl29501_get_freq(isl29501, val, val2);
    case IIO_CHAN_INFO_CALIBBIAS:
    return isl29501_get_calibbias(isl29501, chan, val);
    default:
    return -EINVAL;
    }
    }
    static int isl29501_set_raw(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int raw)
    {
    switch (chan.type) {
    case IIO_CURRENT:
    return isl29501_register_write(isl29501, REG_EMITTER_DAC, raw);
    default:
    return -EINVAL;
    }
    }
    static int isl29501_set_inttime(struct isl29501_private *isl29501,
    int val, int val2)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(isl29501_int_time); i++) {
    if (isl29501_int_time[i][0] == val &&
    isl29501_int_time[i][1] == val2) {
    return isl29501_register_write(isl29501,
    REG_INT_TIME,
    i);
    }
    }
    return -EINVAL;
    }
    static int isl29501_set_scale(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int val, int val2)
    {
    int i;
    if (chan.type != IIO_CURRENT)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(isl29501_current_scale_table); i++) {
    if (isl29501_current_scale_table[i][0] == val &&
    isl29501_current_scale_table[i][1] == val2) {
    return isl29501_register_write(isl29501,
    REG_DRIVER_RANGE,
    i + 1);
    }
    }
    return -EINVAL;
    }
    static int isl29501_set_calibbias(struct isl29501_private *isl29501,
    const struct iio_chan_spec *chan,
    int bias)
    {
    switch (chan.type) {
    case IIO_PROXIMITY:
    return isl29501_register_write(isl29501,
    REG_DISTANCE_BIAS,
    bias);
    case IIO_TEMP:
    return isl29501_register_write(isl29501,
    REG_TEMPERATURE_BIAS,
    bias);
    default:
    return -EINVAL;
    }
    }
    static int isl29501_set_freq(struct isl29501_private *isl29501,
    int val, int val2)
    {
    int freq;
    unsigned long long sample_time;
// sample_freq = 1 / (0.000450 * (sample_time + 1) * 10^-6)
    freq = val * 1000000 + val2 % 1000000;
    sample_time = 2222ULL * 1000000ULL;
    do_div(sample_time, freq);
    sample_time -= 1;
    if (sample_time > 255)
    return -ERANGE;
    return isl29501_register_write(isl29501, REG_SAMPLE_TIME, sample_time);
    }
    static int isl29501_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct isl29501_private *isl29501 = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return isl29501_set_raw(isl29501, chan, val);
    case IIO_CHAN_INFO_INT_TIME:
    return isl29501_set_inttime(isl29501, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return isl29501_set_freq(isl29501, val, val2);
    case IIO_CHAN_INFO_SCALE:
    return isl29501_set_scale(isl29501, chan, val, val2);
    case IIO_CHAN_INFO_CALIBBIAS:
    return isl29501_set_calibbias(isl29501, chan, val);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info isl29501_info = {
    .read_raw = &isl29501_read_raw,
    .write_raw = &isl29501_write_raw,
    .attrs = &isl29501_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn isl29501_init_chip(isl29501: *mut isl29501_private) -> c_int {
    static int isl29501_init_chip(struct isl29501_private *isl29501)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(isl29501.client, ISL29501_DEVICE_ID);
    if (ret < 0) {
    dev_err(&isl29501.client.dev, "Error reading device id\n");
    return ret;
    }
    if (ret != ISL29501_ID) {
    dev_err(&isl29501.client.dev,
    "Wrong chip id, got %x expected %x\n",
    ret, ISL29501_DEVICE_ID);
    return -ENODEV;
    }
    ret = isl29501_reset_registers(isl29501);
    if (ret < 0)
    return ret;
    return isl29501_begin_acquisition(isl29501);
    }
#[no_mangle]
unsafe extern "C" fn isl29501_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t isl29501_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct isl29501_private *isl29501 = iio_priv(indio_dev);
    const unsigned long *active_mask = indio_dev.active_scan_mask;
    u32 value;
    struct {
    u16 data;
    aligned_s64 ts;
    } scan = { };
    if (test_bit(ISL29501_DISTANCE_SCAN_INDEX, active_mask)) {
    isl29501_register_read(isl29501, REG_DISTANCE, &value);
    scan.data = value;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, &scan, pf.timestamp);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn isl29501_probe(client: *mut i2c_client) -> c_int {
    static int isl29501_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct isl29501_private *isl29501;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*isl29501));
    if (!indio_dev)
    return -ENOMEM;
    isl29501 = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    isl29501.client = client;
    mutex_init(&isl29501.lock);
    ret = isl29501_init_chip(isl29501);
    if (ret < 0)
    return ret;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = isl29501_channels;
    indio_dev.num_channels = ARRAY_SIZE(isl29501_channels);
    indio_dev.name = client.name;
    indio_dev.info = &isl29501_info;
    ret = devm_iio_triggered_buffer_setup(&client.dev, indio_dev,
    iio_pollfunc_store_time,
    isl29501_trigger_handler,
    core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&client.dev, "unable to setup iio triggered buffer\n");
    return ret;
    }
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id isl29501_id[] = {
    { .name = "isl29501" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl29501_id);
    static const struct of_device_id isl29501_i2c_matches[] = {
    { .compatible = "renesas,isl29501" },
    { }
    };
    MODULE_DEVICE_TABLE(of, isl29501_i2c_matches);
    static struct i2c_driver isl29501_driver = {
    .driver = {
    .name	= "isl29501",
    .of_match_table = isl29501_i2c_matches,
    },
    .id_table	= isl29501_id,
    .probe		= isl29501_probe,
    };
    module_i2c_driver(isl29501_driver);
    MODULE_AUTHOR("Mathieu Othacehe <m.othacehe@gmail.com>");
    MODULE_DESCRIPTION("ISL29501 Time of Flight sensor driver");
    MODULE_LICENSE("GPL v2");
