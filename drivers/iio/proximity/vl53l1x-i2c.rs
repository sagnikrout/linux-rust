//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/vl53l1x-i2c.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Support for ST VL53L1X FlightSense ToF Ranging Sensor on a i2c bus.
//
// Copyright (C) 2026 Siratul Islam <siratul.islam@linux.dev>
//
// Datasheet available at
// <https://www.st.com/resource/en/datasheet/vl53l1x.pdf>
//
// Default 7-bit i2c slave address 0x29.
//
// The VL53L1X requires a firmware configuration blob to be loaded at boot.
// Register values for the default configuration are taken from
// ST's VL53L1X Ultra Lite Driver (STSW-IMG009).
//

pub const VL53L1X_REG_SOFT_RESET: c_uint = 0x0000;
pub const VL53L1X_REG_VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND: c_uint = 0x0008;
pub const VL53L1X_REG_VHV_CONFIG__INIT: c_uint = 0x000B;
pub const VL53L1X_REG_DEFAULT_CONFIG: c_uint = 0x002D;
pub const VL53L1X_REG_GPIO_HV_MUX__CTRL: c_uint = 0x0030;
pub const VL53L1X_REG_GPIO__TIO_HV_STATUS: c_uint = 0x0031;
pub const VL53L1X_REG_SYSTEM__INTERRUPT_CONFIG_GPIO: c_uint = 0x0046;
pub const VL53L1X_REG_PHASECAL_CONFIG__TIMEOUT_MACROP: c_uint = 0x004B;
pub const VL53L1X_REG_RANGE_CONFIG__TIMEOUT_MACROP_A: c_uint = 0x005E;
pub const VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_A: c_uint = 0x0060;
pub const VL53L1X_REG_RANGE_CONFIG__TIMEOUT_MACROP_B: c_uint = 0x0061;
pub const VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_B: c_uint = 0x0063;
pub const VL53L1X_REG_RANGE_CONFIG__VALID_PHASE_HIGH: c_uint = 0x0069;
pub const VL53L1X_REG_SYSTEM__INTERMEASUREMENT_PERIOD: c_uint = 0x006C;
pub const VL53L1X_REG_SD_CONFIG__WOI_SD0: c_uint = 0x0078;
pub const VL53L1X_REG_SD_CONFIG__WOI_SD1: c_uint = 0x0079;
pub const VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD0: c_uint = 0x007A;
pub const VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD1: c_uint = 0x007B;
pub const VL53L1X_REG_SYSTEM__INTERRUPT_CLEAR: c_uint = 0x0086;
pub const VL53L1X_REG_SYSTEM__MODE_START: c_uint = 0x0087;
pub const VL53L1X_REG_RESULT__RANGE_STATUS: c_uint = 0x0089;
pub const VL53L1X_REG_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0: c_uint = 0x0096;
pub const VL53L1X_REG_RESULT__OSC_CALIBRATE_VAL: c_uint = 0x00DE;
pub const VL53L1X_REG_FIRMWARE__SYSTEM_STATUS: c_uint = 0x00E5;
pub const VL53L1X_REG_IDENTIFICATION__MODEL_ID: c_uint = 0x010F;
pub const VL53L1X_MODEL_ID_VAL: c_uint = 0xEACC;
pub const VL53L1X_MODE_START_TIMED: c_uint = 0x40;
pub const VL53L1X_MODE_START_STOP: c_uint = 0x00;
pub const VL53L1X_INT_NEW_SAMPLE_READY: c_uint = 0x02;

pub const VL53L1X_VHV_LOOP_BOUND_TWO: c_uint = 0x09;

pub const VL53L1X_RANGE_STATUS_VALID: c_int = 9;

// Inter-measurement period uses PLL divider with 1.075 oscillator correction
    static const struct u32_fract vl53l1x_osc_correction = {
    .numerator = 1075,
    .denominator = 1000,
    };
    enum vl53l1x_distance_mode {
    VL53L1X_SHORT,
    VL53L1X_LONG,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl53l1x_data {
    pub regmap: *mut regmap,
    pub completion: completion,
    pub xshut_reset: *mut reset_control,
    pub distance_mode: enum vl53l1x_distance_mode,
    pub gpio_polarity: u8,
    pub irq: c_int,
}

    static const struct regmap_range vl53l1x_volatile_ranges[] = {
    regmap_reg_range(VL53L1X_REG_GPIO__TIO_HV_STATUS,
    VL53L1X_REG_GPIO__TIO_HV_STATUS),
    regmap_reg_range(VL53L1X_REG_RESULT__RANGE_STATUS,
    VL53L1X_REG_RESULT__RANGE_STATUS),
    regmap_reg_range(VL53L1X_REG_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
    VL53L1X_REG_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 + 1),
    regmap_reg_range(VL53L1X_REG_RESULT__OSC_CALIBRATE_VAL,
    VL53L1X_REG_RESULT__OSC_CALIBRATE_VAL + 1),
    regmap_reg_range(VL53L1X_REG_FIRMWARE__SYSTEM_STATUS,
    VL53L1X_REG_FIRMWARE__SYSTEM_STATUS),
    };
    static const struct regmap_access_table vl53l1x_volatile_table = {
    .yes_ranges = vl53l1x_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(vl53l1x_volatile_ranges),
    };
    static const struct regmap_range vl53l1x_write_only_ranges[] = {
    regmap_reg_range(VL53L1X_REG_SOFT_RESET, VL53L1X_REG_SOFT_RESET),
    regmap_reg_range(VL53L1X_REG_SYSTEM__INTERRUPT_CLEAR,
    VL53L1X_REG_SYSTEM__MODE_START),
    };
    static const struct regmap_access_table vl53l1x_readable_table = {
    .no_ranges = vl53l1x_write_only_ranges,
    .n_no_ranges = ARRAY_SIZE(vl53l1x_write_only_ranges),
    };
    static const struct regmap_config vl53l1x_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
// MODEL_ID is 16-bit. +1 covers the second byte at 0x0110
    .max_register = VL53L1X_REG_IDENTIFICATION__MODEL_ID + 1,
    .cache_type = REGCACHE_MAPLE,
    .volatile_table = &vl53l1x_volatile_table,
    .rd_table = &vl53l1x_readable_table,
    };
#[no_mangle]
unsafe extern "C" fn vl53l1x_read_u16(data: *mut vl53l1x_data, reg: u16, val: *mut u16) -> c_int {
    static int vl53l1x_read_u16(struct vl53l1x_data *data, u16 reg, u16 *val)
    {
    __be16 buf;
    int ret;
    ret = regmap_bulk_read(data.regmap, reg, &buf, sizeof(buf));
    if (ret)
    return ret;
// val = be16_to_cpu(buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_write_u16(data: *mut vl53l1x_data, reg: u16, val: u16) -> c_int {
    static int vl53l1x_write_u16(struct vl53l1x_data *data, u16 reg, u16 val)
    {
    let mut buf: __be16 = cpu_to_be16(val);
    return regmap_bulk_write(data.regmap, reg, &buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_write_u32(data: *mut vl53l1x_data, reg: u16, val: u32) -> c_int {
    static int vl53l1x_write_u32(struct vl53l1x_data *data, u16 reg, u32 val)
    {
    let mut buf: __be32 = cpu_to_be32(val);
    return regmap_bulk_write(data.regmap, reg, &buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_clear_irq(data: *mut vl53l1x_data) -> c_int {
    static int vl53l1x_clear_irq(struct vl53l1x_data *data)
    {
    return regmap_write(data.regmap, VL53L1X_REG_SYSTEM__INTERRUPT_CLEAR, 0x01);
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_start_ranging(data: *mut vl53l1x_data) -> c_int {
    static int vl53l1x_start_ranging(struct vl53l1x_data *data)
    {
    int ret;
    ret = vl53l1x_clear_irq(data);
    if (ret)
    return ret;
    return regmap_write(data.regmap, VL53L1X_REG_SYSTEM__MODE_START,
    VL53L1X_MODE_START_TIMED);
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_stop_ranging(data: *mut vl53l1x_data) -> c_int {
    static int vl53l1x_stop_ranging(struct vl53l1x_data *data)
    {
    return regmap_write(data.regmap, VL53L1X_REG_SYSTEM__MODE_START,
    VL53L1X_MODE_START_STOP);
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_wait_data_ready(data: *mut vl53l1x_data) -> c_int {
    static int vl53l1x_wait_data_ready(struct vl53l1x_data *data)
    {
    unsigned int val;
// 1ms poll, 1s timeout covers max timing budgets (per ST Ultra Lite Driver)
    return regmap_read_poll_timeout(data.regmap,
    VL53L1X_REG_GPIO__TIO_HV_STATUS, val,
    (val & VL53L1X_GPIO__TIO_HV_STATUS_DATA_READY) != data.gpio_polarity,
    1 * USEC_PER_MSEC, 1 * USEC_PER_SEC);
    }
//
// Default configuration blob from ST's VL53L1X Ultra Lite Driver
// (STSW-IMG009).
//
    static const u8 vl53l1x_default_config[] = {
    0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02, 0x08,	/* reg 0x2d..0x34 */
    0x00, 0x08, 0x10, 0x01, 0x01, 0x00, 0x00, 0x00,	/* reg 0x35..0x3c */
    0x00, 0xFF, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00,	/* reg 0x3d..0x44 */
    0x00, 0x20, 0x0B, 0x00, 0x00, 0x02, 0x0A, 0x21,	/* reg 0x45..0x4c */
    0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0xC8,	/* reg 0x4d..0x54 */
    0x00, 0x00, 0x38, 0xFF, 0x01, 0x00, 0x08, 0x00,	/* reg 0x55..0x5c */
    0x00, 0x01, 0xCC, 0x0F, 0x01, 0xF1, 0x0D, 0x01,	/* reg 0x5d..0x64 */
    0x68, 0x00, 0x80, 0x08, 0xB8, 0x00, 0x00, 0x00,	/* reg 0x65..0x6c */
    0x00, 0x0F, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00,	/* reg 0x6d..0x74 */
    0x00, 0x00, 0x01, 0x0F, 0x0D, 0x0E, 0x0E, 0x00,	/* reg 0x75..0x7c */
    0x00, 0x02, 0xC7, 0xFF, 0x9B, 0x00, 0x00, 0x00,	/* reg 0x7d..0x84 */
    0x01, 0x00, 0x00,				/* reg 0x85..0x87 */
    };
#[no_mangle]
unsafe extern "C" fn vl53l1x_chip_init(data: *mut vl53l1x_data) -> c_int {
    static int vl53l1x_chip_init(struct vl53l1x_data *data)
    {
    struct device *dev = regmap_get_device(data.regmap);
    unsigned int val;
    u16 model_id;
    int ret;
    if (!data.xshut_reset) {
    ret = regmap_write(data.regmap, VL53L1X_REG_SOFT_RESET, 0x00);
    if (ret)
    return ret;
    fsleep(100); /* conservative reset pulse, no spec */
    ret = regmap_write(data.regmap, VL53L1X_REG_SOFT_RESET, 0x01);
    if (ret)
    return ret;
    fsleep(1000); /* conservative boot wait, no spec */
    }
    ret = regmap_read_poll_timeout(data.regmap,
    VL53L1X_REG_FIRMWARE__SYSTEM_STATUS,
    val, val & VL53L1X_FIRMWARE__SYSTEM_STATUS_BOOTED,
    1 * USEC_PER_MSEC, 100 * USEC_PER_MSEC);
    if (ret)
    return dev_err_probe(dev, ret, "firmware boot timeout\n");
    ret = vl53l1x_read_u16(data, VL53L1X_REG_IDENTIFICATION__MODEL_ID,
    &model_id);
    if (ret)
    return ret;
    if (model_id != VL53L1X_MODEL_ID_VAL)
    dev_info(dev, "unknown model id: 0x%04x, continuing\n", model_id);
    ret = regmap_bulk_write(data.regmap, VL53L1X_REG_DEFAULT_CONFIG,
    vl53l1x_default_config,
    sizeof(vl53l1x_default_config));
    if (ret)
    return ret;
    ret = regmap_read(data.regmap, VL53L1X_REG_GPIO_HV_MUX__CTRL, &val);
    if (ret)
    return ret;
    data.gpio_polarity = !!(val & VL53L1X_GPIO_HV_MUX_POLARITY);
// Initial ranging cycle for VHV calibration
    ret = vl53l1x_start_ranging(data);
    if (ret)
    return ret;
    ret = vl53l1x_wait_data_ready(data);
    if (ret)
    return ret;
    ret = vl53l1x_clear_irq(data);
    if (ret)
    return ret;
    ret = vl53l1x_stop_ranging(data);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap,
    VL53L1X_REG_VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND,
    VL53L1X_VHV_LOOP_BOUND_TWO);
    if (ret)
    return ret;
    return regmap_write(data.regmap, VL53L1X_REG_VHV_CONFIG__INIT, 0x00);
    }
    static const struct reg_sequence vl53l1x_mode_short[] = {
    { VL53L1X_REG_PHASECAL_CONFIG__TIMEOUT_MACROP,		0x14 },
    { VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_A,		0x07 },
    { VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_B,		0x05 },
    { VL53L1X_REG_RANGE_CONFIG__VALID_PHASE_HIGH,		0x38 },
    { VL53L1X_REG_SD_CONFIG__WOI_SD0,			0x07 },
    { VL53L1X_REG_SD_CONFIG__WOI_SD1,			0x05 },
    { VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD0,		0x06 },
    { VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD1,		0x06 },
    };
    static const struct reg_sequence vl53l1x_mode_long[] = {
    { VL53L1X_REG_PHASECAL_CONFIG__TIMEOUT_MACROP,		0x0A },
    { VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_A,		0x0F },
    { VL53L1X_REG_RANGE_CONFIG__VCSEL_PERIOD_B,		0x0D },
    { VL53L1X_REG_RANGE_CONFIG__VALID_PHASE_HIGH,		0xB8 },
    { VL53L1X_REG_SD_CONFIG__WOI_SD0,			0x0F },
    { VL53L1X_REG_SD_CONFIG__WOI_SD1,			0x0D },
    { VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD0,		0x0E },
    { VL53L1X_REG_SD_CONFIG__INITIAL_PHASE_SD1,		0x0E },
    };
    static const struct {
    const struct reg_sequence *regs;
    size_t num_regs;
    } vl53l1x_mode_configs[] = {
    [VL53L1X_SHORT] = { vl53l1x_mode_short, ARRAY_SIZE(vl53l1x_mode_short) },
    [VL53L1X_LONG]  = { vl53l1x_mode_long, ARRAY_SIZE(vl53l1x_mode_long) },
    };
    static int vl53l1x_set_distance_mode(struct vl53l1x_data *data,
    enum vl53l1x_distance_mode mode)
    {
    int ret;
    if (mode >= ARRAY_SIZE(vl53l1x_mode_configs))
    return -EINVAL;
    ret = regmap_multi_reg_write(data.regmap,
    vl53l1x_mode_configs[mode].regs,
    vl53l1x_mode_configs[mode].num_regs);
    if (ret)
    return ret;
    data.distance_mode = mode;
    return 0;
    }
//
// The timing budget controls how long the sensor spends collecting
// a single range measurement. Pre-computed TIMEOUT_MACROP register
// values from ST's VL53L1X Ultra Lite Driver.
//
#[no_mangle]
unsafe extern "C" fn vl53l1x_set_timing_budget(data: *mut vl53l1x_data, budget_ms: u16) -> c_int {
    static int vl53l1x_set_timing_budget(struct vl53l1x_data *data, u16 budget_ms)
    {
    u16 timeout_a, timeout_b;
    int ret;
    switch (data.distance_mode) {
    case VL53L1X_SHORT:
    switch (budget_ms) {
    case 15:
    timeout_a = 0x001D;
    timeout_b = 0x0027;
    break;
    case 20:
    timeout_a = 0x0051;
    timeout_b = 0x006E;
    break;
    case 33:
    timeout_a = 0x00D6;
    timeout_b = 0x006E;
    break;
    case 50:
    timeout_a = 0x01AE;
    timeout_b = 0x01E8;
    break;
    case 100:
    timeout_a = 0x02E1;
    timeout_b = 0x0388;
    break;
    case 200:
    timeout_a = 0x03E1;
    timeout_b = 0x0496;
    break;
    case 500:
    timeout_a = 0x0591;
    timeout_b = 0x05C1;
    break;
    default:
    return -EINVAL;
    }
    break;
    case VL53L1X_LONG:
    switch (budget_ms) {
    case 20:
    timeout_a = 0x001E;
    timeout_b = 0x0022;
    break;
    case 33:
    timeout_a = 0x0060;
    timeout_b = 0x006E;
    break;
    case 50:
    timeout_a = 0x00AD;
    timeout_b = 0x00C6;
    break;
    case 100:
    timeout_a = 0x01CC;
    timeout_b = 0x01EA;
    break;
    case 200:
    timeout_a = 0x02D9;
    timeout_b = 0x02F8;
    break;
    case 500:
    timeout_a = 0x048F;
    timeout_b = 0x04A4;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    ret = vl53l1x_write_u16(data, VL53L1X_REG_RANGE_CONFIG__TIMEOUT_MACROP_A,
    timeout_a);
    if (ret)
    return ret;
    return vl53l1x_write_u16(data, VL53L1X_REG_RANGE_CONFIG__TIMEOUT_MACROP_B,
    timeout_b);
    }
    static int vl53l1x_set_inter_measurement_ms(struct vl53l1x_data *data,
    u16 period_ms)
    {
    u16 osc_calibrate_val;
    u16 clock_pll;
    u32 inter_meas;
    int ret;
    ret = vl53l1x_read_u16(data, VL53L1X_REG_RESULT__OSC_CALIBRATE_VAL,
    &osc_calibrate_val);
    if (ret)
    return ret;
    clock_pll = osc_calibrate_val & VL53L1X_OSC_CALIBRATE_MASK;
    inter_meas = (clock_pll * period_ms * vl53l1x_osc_correction.numerator) /
    vl53l1x_osc_correction.denominator;
    return vl53l1x_write_u32(data,
    VL53L1X_REG_SYSTEM__INTERMEASUREMENT_PERIOD,
    inter_meas);
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_read_proximity(data: *mut vl53l1x_data, val: *mut c_int) -> c_int {
    static int vl53l1x_read_proximity(struct vl53l1x_data *data, int *val)
    {
    unsigned int range_status;
    u16 distance;
    int ret;
    if (data.irq) {
    reinit_completion(&data.completion);
    ret = vl53l1x_clear_irq(data);
    if (ret)
    return ret;
    if (!wait_for_completion_timeout(&data.completion, HZ))
    return -ETIMEDOUT;
    } else {
    ret = vl53l1x_wait_data_ready(data);
    if (ret)
    return ret;
    }
    ret = regmap_read(data.regmap, VL53L1X_REG_RESULT__RANGE_STATUS,
    &range_status);
    if (ret)
    goto clear_irq;
    if (FIELD_GET(VL53L1X_RANGE_STATUS_MASK, range_status) !=
    VL53L1X_RANGE_STATUS_VALID) {
    ret = -EIO;
    goto clear_irq;
    }
    ret = vl53l1x_read_u16(data,
    VL53L1X_REG_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
    &distance);
    if (ret)
    goto clear_irq;
// val = distance;
    clear_irq:
    vl53l1x_clear_irq(data);
    return ret;
    }
    static const struct iio_chan_spec vl53l1x_channels[] = {
    {
    .type = IIO_DISTANCE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 'u',
    .realbits = 16,
    .storagebits = 16,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    };
    static int vl53l1x_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct vl53l1x_data *data = iio_priv(indio_dev);
    int ret;
    if (chan.type != IIO_DISTANCE)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = vl53l1x_read_proximity(data, val);
    iio_device_release_direct(indio_dev);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = 1000;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info vl53l1x_info = {
    .read_raw = vl53l1x_read_raw,
    .validate_trigger = iio_validate_own_trigger,
    };
#[no_mangle]
unsafe extern "C" fn vl53l1x_trigger_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t vl53l1x_trigger_handler(int irq, void *priv)
    {
    struct iio_poll_func *pf = priv;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct vl53l1x_data *data = iio_priv(indio_dev);
    struct {
    u16 distance;
    aligned_s64 timestamp;
    } scan = { };
    unsigned int range_status;
    int ret;
    ret = regmap_read(data.regmap, VL53L1X_REG_RESULT__RANGE_STATUS,
    &range_status);
    if (ret)
    goto notify_and_clear_irq;
    if (FIELD_GET(VL53L1X_RANGE_STATUS_MASK, range_status) !=
    VL53L1X_RANGE_STATUS_VALID)
    goto notify_and_clear_irq;
    ret = vl53l1x_read_u16(data,
    VL53L1X_REG_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
    &scan.distance);
    if (ret)
    goto notify_and_clear_irq;
    iio_push_to_buffers_with_ts(indio_dev, &scan, sizeof(scan),
    iio_get_time_ns(indio_dev));
    notify_and_clear_irq:
    iio_trigger_notify_done(indio_dev.trig);
    vl53l1x_clear_irq(data);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t vl53l1x_irq_handler(int irq, void *priv)
    {
    struct iio_dev *indio_dev = priv;
    struct vl53l1x_data *data = iio_priv(indio_dev);
    if (iio_buffer_enabled(indio_dev))
    iio_trigger_poll(indio_dev.trig);
    else
    complete(&data.completion);
    return IRQ_HANDLED;
    }
    static const struct iio_trigger_ops vl53l1x_trigger_ops = {
    .validate_device = iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn vl53l1x_stop_ranging_action(priv: *mut c_void) {
    static void vl53l1x_stop_ranging_action(void *priv)
    {
    vl53l1x_stop_ranging(priv);
    }
    static int vl53l1x_configure_irq(struct device *dev, int irq,
    struct iio_dev *indio_dev)
    {
    struct vl53l1x_data *data = iio_priv(indio_dev);
    int ret;
    ret = devm_request_irq(dev, irq, vl53l1x_irq_handler, IRQF_NO_THREAD,
    indio_dev.name, indio_dev);
    if (ret)
    return ret;
    ret = regmap_write(data.regmap, VL53L1X_REG_SYSTEM__INTERRUPT_CONFIG_GPIO,
    VL53L1X_INT_NEW_SAMPLE_READY);
    if (ret)
    return dev_err_probe(dev, ret, "failed to configure IRQ\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vl53l1x_probe(client: *mut i2c_client) -> c_int {
    static int vl53l1x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct vl53l1x_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.irq = client.irq;
    data.regmap = devm_regmap_init_i2c(client, &vl53l1x_regmap_config);
    if (IS_ERR(data.regmap))
    return dev_err_probe(dev, PTR_ERR(data.regmap),
    "regmap initialization failed\n");
    ret = devm_regulator_get_enable(dev, "vdd");
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable VDD regulator\n");
//
// XSHUT held low puts the chip in hardware standby. All register
// state is lost on de-assert so this is functionally a reset.
//
    data.xshut_reset = devm_reset_control_get_optional_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(data.xshut_reset))
    return dev_err_probe(dev, PTR_ERR(data.xshut_reset),
    "Cannot get reset control\n");
//
// 1.2 ms max boot duration.
// Datasheet Section 3.6 "Power up and boot sequence".
//
    fsleep(1200);
    ret = vl53l1x_chip_init(data);
    if (ret)
    return ret;
    ret = vl53l1x_set_distance_mode(data, VL53L1X_LONG);
    if (ret)
    return ret;
// 50 ms timing budget (per ST Ultra Lite Driver)
    ret = vl53l1x_set_timing_budget(data, 50);
    if (ret)
    return ret;
// 50 ms inter-measurement period (per ST Ultra Lite Driver)
    ret = vl53l1x_set_inter_measurement_ms(data, 50);
    if (ret)
    return ret;
//
// The hardware only supports "autonomous" continuous ranging mode.
// Start ranging here and leave it running for the lifetime of
// the device. Both direct reads and the buffer path rely on this.
//
    ret = vl53l1x_start_ranging(data);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, vl53l1x_stop_ranging_action, data);
    if (ret)
    return ret;
    indio_dev.name = "vl53l1x";
    indio_dev.info = &vl53l1x_info;
    indio_dev.channels = vl53l1x_channels;
    indio_dev.num_channels = ARRAY_SIZE(vl53l1x_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    if (client.irq) {
    struct iio_trigger *trig;
    init_completion(&data.completion);
    trig = devm_iio_trigger_alloc(dev, "%s-dev%d", indio_dev.name,
    iio_device_id(indio_dev));
    if (!trig)
    return -ENOMEM;
    trig.ops = &vl53l1x_trigger_ops;
    iio_trigger_set_drvdata(trig, indio_dev);
    ret = devm_iio_trigger_register(dev, trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(trig);
    ret = vl53l1x_configure_irq(dev, client.irq, indio_dev);
    if (ret)
    return ret;
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev, core::ptr::null_mut(),
    &vl53l1x_trigger_handler,
    core::ptr::null_mut());
    if (ret)
    return ret;
    }
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct i2c_device_id vl53l1x_id[] = {
    { .name = "vl53l1x" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, vl53l1x_id);
    static const struct of_device_id st_vl53l1x_dt_match[] = {
    { .compatible = "st,vl53l1x" },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_vl53l1x_dt_match);
    static struct i2c_driver vl53l1x_driver = {
    .driver = {
    .name = "vl53l1x-i2c",
    .of_match_table = st_vl53l1x_dt_match,
    },
    .probe = vl53l1x_probe,
    .id_table = vl53l1x_id,
    };
    module_i2c_driver(vl53l1x_driver);
    MODULE_AUTHOR("Siratul Islam <siratul.islam@linux.dev>");
    MODULE_DESCRIPTION("ST VL53L1X ToF ranging sensor driver");
    MODULE_LICENSE("Dual BSD/GPL");
