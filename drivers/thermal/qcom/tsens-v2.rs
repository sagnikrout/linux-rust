//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qcom/tsens-v2.c
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited
//

// ----- SROT ------
pub const SROT_HW_VER_OFF: c_uint = 0x0000;
pub const SROT_CTRL_OFF: c_uint = 0x0004;
pub const SROT_MEASURE_PERIOD: c_uint = 0x0008;
pub const SROT_Sn_CONVERSION: c_uint = 0x0060;
pub const V2_SHIFT_DEFAULT: c_uint = 0x0003;
pub const V2_SLOPE_DEFAULT: c_uint = 0x0cd0;
pub const V2_CZERO_DEFAULT: c_uint = 0x016a;
pub const ONE_PT_SLOPE: c_uint = 0x0cd0;
pub const TWO_PT_SHIFTED_GAIN: c_int = 921600;
pub const ONE_PT_CZERO_CONST: c_int = 94;
pub const SW_RST_DEASSERT: c_uint = 0x0;
pub const SW_RST_ASSERT: c_uint = 0x1;
pub const MEASURE_PERIOD_2mSEC: c_uint = 0x1;
pub const RESULT_FORMAT_TEMP: c_uint = 0x1;
pub const TSENS_ENABLE: c_uint = 0x1;

// ----- TM ------
pub const TM_INT_EN_OFF: c_uint = 0x0004;
pub const TM_UPPER_LOWER_INT_STATUS_OFF: c_uint = 0x0008;
pub const TM_UPPER_LOWER_INT_CLEAR_OFF: c_uint = 0x000c;
pub const TM_UPPER_LOWER_INT_MASK_OFF: c_uint = 0x0010;
pub const TM_CRITICAL_INT_STATUS_OFF: c_uint = 0x0014;
pub const TM_CRITICAL_INT_CLEAR_OFF: c_uint = 0x0018;
pub const TM_CRITICAL_INT_MASK_OFF: c_uint = 0x001c;
pub const TM_Sn_UPPER_LOWER_THRESHOLD_OFF: c_uint = 0x0020;
pub const TM_Sn_CRITICAL_THRESHOLD_OFF: c_uint = 0x0060;
pub const TM_Sn_STATUS_OFF: c_uint = 0x00a0;
pub const TM_TRDY_OFF: c_uint = 0x00e4;
pub const TM_WDOG_LOG_OFF: c_uint = 0x013c;
// v2.x: 8996, 8998, sdm845
    static struct tsens_features tsens_v2_feat = {
    .ver_major	= VER_2_X,
    .crit_int	= 1,
    .combo_int	= 0,
    .adc		= 0,
    .srot_split	= 1,
    .max_sensors	= 16,
    .trip_min_temp	= -40000,
    .trip_max_temp	= 120000,
    };
    static struct tsens_features ipq8074_feat = {
    .ver_major	= VER_2_X,
    .crit_int	= 1,
    .combo_int	= 1,
    .adc		= 0,
    .srot_split	= 1,
    .max_sensors	= 16,
    .trip_min_temp	= 0,
    .trip_max_temp	= 204000,
    };
    static struct tsens_features ipq5332_feat = {
    .ver_major	= VER_2_X_NO_RPM,
    .crit_int	= 1,
    .combo_int	= 1,
    .adc		= 0,
    .srot_split	= 1,
    .max_sensors	= 16,
    .trip_min_temp	= 0,
    .trip_max_temp	= 204000,
    };
    static const struct reg_field tsens_v2_regfields[MAX_REGFIELDS] = {
// ----- SROT ------
// VERSION
    [VER_MAJOR] = REG_FIELD(SROT_HW_VER_OFF, 28, 31),
    [VER_MINOR] = REG_FIELD(SROT_HW_VER_OFF, 16, 27),
    [VER_STEP]  = REG_FIELD(SROT_HW_VER_OFF,  0, 15),
// CTRL_OFF
    [TSENS_EN]     = REG_FIELD(SROT_CTRL_OFF,    0,  0),
    [TSENS_SW_RST] = REG_FIELD(SROT_CTRL_OFF,    1,  1),
    [SENSOR_EN]    = REG_FIELD(SROT_CTRL_OFF,    3,  18),
    [CODE_OR_TEMP] = REG_FIELD(SROT_CTRL_OFF,    21, 21),
    [MAIN_MEASURE_PERIOD] = REG_FIELD(SROT_MEASURE_PERIOD, 0, 7),
// ----- TM ------
// INTERRUPT ENABLE
// v2 has separate enables for UPPER/LOWER/CRITICAL interrupts
    [INT_EN]  = REG_FIELD(TM_INT_EN_OFF, 0, 2),
// TEMPERATURE THRESHOLDS
    REG_FIELD_FOR_EACH_SENSOR16(LOW_THRESH,  TM_Sn_UPPER_LOWER_THRESHOLD_OFF,  0,  11),
    REG_FIELD_FOR_EACH_SENSOR16(UP_THRESH,   TM_Sn_UPPER_LOWER_THRESHOLD_OFF, 12,  23),
    REG_FIELD_FOR_EACH_SENSOR16(CRIT_THRESH, TM_Sn_CRITICAL_THRESHOLD_OFF,     0,  11),
// INTERRUPTS [CLEAR/STATUS/MASK]
    REG_FIELD_SPLIT_BITS_0_15(LOW_INT_STATUS,  TM_UPPER_LOWER_INT_STATUS_OFF),
    REG_FIELD_SPLIT_BITS_0_15(LOW_INT_CLEAR,   TM_UPPER_LOWER_INT_CLEAR_OFF),
    REG_FIELD_SPLIT_BITS_0_15(LOW_INT_MASK,    TM_UPPER_LOWER_INT_MASK_OFF),
    REG_FIELD_SPLIT_BITS_16_31(UP_INT_STATUS,  TM_UPPER_LOWER_INT_STATUS_OFF),
    REG_FIELD_SPLIT_BITS_16_31(UP_INT_CLEAR,   TM_UPPER_LOWER_INT_CLEAR_OFF),
    REG_FIELD_SPLIT_BITS_16_31(UP_INT_MASK,    TM_UPPER_LOWER_INT_MASK_OFF),
    REG_FIELD_SPLIT_BITS_0_15(CRIT_INT_STATUS, TM_CRITICAL_INT_STATUS_OFF),
    REG_FIELD_SPLIT_BITS_0_15(CRIT_INT_CLEAR,  TM_CRITICAL_INT_CLEAR_OFF),
    REG_FIELD_SPLIT_BITS_0_15(CRIT_INT_MASK,   TM_CRITICAL_INT_MASK_OFF),
// WATCHDOG on v2.3 or later
    [WDOG_BARK_STATUS] = REG_FIELD(TM_CRITICAL_INT_STATUS_OFF, 31, 31),
    [WDOG_BARK_CLEAR]  = REG_FIELD(TM_CRITICAL_INT_CLEAR_OFF,  31, 31),
    [WDOG_BARK_MASK]   = REG_FIELD(TM_CRITICAL_INT_MASK_OFF,   31, 31),
    [CC_MON_STATUS]    = REG_FIELD(TM_CRITICAL_INT_STATUS_OFF, 30, 30),
    [CC_MON_CLEAR]     = REG_FIELD(TM_CRITICAL_INT_CLEAR_OFF,  30, 30),
    [CC_MON_MASK]      = REG_FIELD(TM_CRITICAL_INT_MASK_OFF,   30, 30),
    [WDOG_BARK_COUNT]  = REG_FIELD(TM_WDOG_LOG_OFF,             0,  7),
// Sn_STATUS
    REG_FIELD_FOR_EACH_SENSOR16(LAST_TEMP,       TM_Sn_STATUS_OFF,  0,  11),
    REG_FIELD_FOR_EACH_SENSOR16(VALID,           TM_Sn_STATUS_OFF, 21,  21),
// xxx_STATUS bits: 1 == threshold violated
    REG_FIELD_FOR_EACH_SENSOR16(MIN_STATUS,      TM_Sn_STATUS_OFF, 16,  16),
    REG_FIELD_FOR_EACH_SENSOR16(LOWER_STATUS,    TM_Sn_STATUS_OFF, 17,  17),
    REG_FIELD_FOR_EACH_SENSOR16(UPPER_STATUS,    TM_Sn_STATUS_OFF, 18,  18),
    REG_FIELD_FOR_EACH_SENSOR16(CRITICAL_STATUS, TM_Sn_STATUS_OFF, 19,  19),
    REG_FIELD_FOR_EACH_SENSOR16(MAX_STATUS,      TM_Sn_STATUS_OFF, 20,  20),
// TRDY: 1=ready, 0=in progress
    [TRDY] = REG_FIELD(TM_TRDY_OFF, 0, 0),
    };
    static int tsens_v2_calibrate_sensor(struct device *dev, struct tsens_sensor *sensor,
    struct regmap *map,  u32 mode, u32 base0, u32 base1)
    {
    let mut shift: u32 = V2_SHIFT_DEFAULT;
    let mut slope: u32 = V2_SLOPE_DEFAULT;
    let mut czero: u32 = V2_CZERO_DEFAULT;
    char	name[20];
    u32	val;
    int	ret;
// Read offset value
    ret = snprintf(name, sizeof(name), "tsens_sens%d_off", sensor.hw_id);
    if (ret < 0)
    return ret;
    ret = nvmem_cell_read_variable_le_u32(dev, name, &sensor.offset);
    if (ret)
    return ret;
// Based on calib mode, program SHIFT, SLOPE and CZERO
    switch (mode) {
    case TWO_PT_CALIB:
    slope = (TWO_PT_SHIFTED_GAIN / (base1 - base0));
    czero = (base0 + sensor.offset - ((base1 - base0) / 3));
    break;
    case ONE_PT_CALIB2:
    czero = base0 + sensor.offset - ONE_PT_CZERO_CONST;
    slope = ONE_PT_SLOPE;
    break;
    default:
    dev_dbg(dev, "calibrationless mode\n");
    }
    val = FIELD_PREP(CONVERSION_SHIFT_MASK, shift) |
    FIELD_PREP(CONVERSION_SLOPE_MASK, slope) |
    FIELD_PREP(CONVERSION_CZERO_MASK, czero);
    regmap_write(map, SENSOR_CONVERSION(sensor.hw_id), val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsens_v2_calibration(priv: *mut tsens_priv) -> c_int {
    static int tsens_v2_calibration(struct tsens_priv *priv)
    {
    struct device *dev = priv.dev;
    u32 mode, base0, base1;
    int i, ret;
    if (priv.num_sensors > MAX_SENSORS)
    return -EINVAL;
    ret = nvmem_cell_read_variable_le_u32(priv.dev, "mode", &mode);
    if (ret == -ENOENT)
    dev_warn(priv.dev, "Calibration data not present in DT\n");
    if (ret < 0)
    return ret;
    dev_dbg(priv.dev, "calibration mode is %d\n", mode);
    ret = nvmem_cell_read_variable_le_u32(priv.dev, "base0", &base0);
    if (ret < 0)
    return ret;
    ret = nvmem_cell_read_variable_le_u32(priv.dev, "base1", &base1);
    if (ret < 0)
    return ret;
// Calibrate each sensor
    for (i = 0; i < priv.num_sensors; i++) {
    ret = tsens_v2_calibrate_sensor(dev, &priv.sensor[i], priv.srot_map,
    mode, base0, base1);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_tsens_v2_no_rpm(priv: *mut tsens_priv) -> int __init {
    static int __init init_tsens_v2_no_rpm(struct tsens_priv *priv)
    {
    struct device *dev = priv.dev;
    int i, ret;
    let mut val: u32 = 0;
    ret = init_common(priv);
    if (ret < 0)
    return ret;
    priv.rf[CODE_OR_TEMP] = devm_regmap_field_alloc(dev, priv.srot_map,
    priv.fields[CODE_OR_TEMP]);
    if (IS_ERR(priv.rf[CODE_OR_TEMP]))
    return PTR_ERR(priv.rf[CODE_OR_TEMP]);
    priv.rf[MAIN_MEASURE_PERIOD] = devm_regmap_field_alloc(dev, priv.srot_map,
    priv.fields[MAIN_MEASURE_PERIOD]);
    if (IS_ERR(priv.rf[MAIN_MEASURE_PERIOD]))
    return PTR_ERR(priv.rf[MAIN_MEASURE_PERIOD]);
    regmap_field_write(priv.rf[TSENS_SW_RST], SW_RST_ASSERT);
    regmap_field_write(priv.rf[MAIN_MEASURE_PERIOD], MEASURE_PERIOD_2mSEC);
// Enable available sensors
    for (i = 0; i < priv.num_sensors; i++)
    val |= 1 << priv.sensor[i].hw_id;
    regmap_field_write(priv.rf[SENSOR_EN], val);
// Select temperature format, unit is deci-Celsius
    regmap_field_write(priv.rf[CODE_OR_TEMP], RESULT_FORMAT_TEMP);
    regmap_field_write(priv.rf[TSENS_SW_RST], SW_RST_DEASSERT);
    regmap_field_write(priv.rf[TSENS_EN], TSENS_ENABLE);
    return 0;
    }
    static const struct tsens_ops ops_generic_v2 = {
    .init		= init_common,
    .get_temp	= get_temp_tsens_valid,
    };
    struct tsens_plat_data data_tsens_v2 = {
    .ops		= &ops_generic_v2,
    .feat		= &tsens_v2_feat,
    .fields	= tsens_v2_regfields,
    };
    struct tsens_plat_data data_ipq8074 = {
    .ops		= &ops_generic_v2,
    .feat		= &ipq8074_feat,
    .fields	= tsens_v2_regfields,
    };
    static const struct tsens_ops ops_ipq5332 = {
    .init		= init_tsens_v2_no_rpm,
    .get_temp	= get_temp_tsens_valid,
    .calibrate	= tsens_v2_calibration,
    };
    const struct tsens_plat_data data_ipq5332 = {
    .num_sensors	= 5,
    .ops		= &ops_ipq5332,
    .hw_ids		= (unsigned int []){11, 12, 13, 14, 15},
    .feat		= &ipq5332_feat,
    .fields		= tsens_v2_regfields,
    };
    const struct tsens_plat_data data_ipq5424 = {
    .num_sensors	= 7,
    .ops		= &ops_ipq5332,
    .hw_ids		= (unsigned int []){9, 10, 11, 12, 13, 14, 15},
    .feat		= &ipq5332_feat,
    .fields		= tsens_v2_regfields,
    };
// Kept around for backward compatibility with old msm8996.dtsi
    struct tsens_plat_data data_8996 = {
    .num_sensors	= 13,
    .ops		= &ops_generic_v2,
    .feat		= &tsens_v2_feat,
    .fields	= tsens_v2_regfields,
    };
// Do not enable wakeup capable interrupts for automotive platforms
    struct tsens_plat_data data_automotive_v2 = {
    .ops		= &ops_generic_v2,
    .feat		= &tsens_v2_feat,
    .fields		= tsens_v2_regfields,
    .no_irq_wake = true,
    };
