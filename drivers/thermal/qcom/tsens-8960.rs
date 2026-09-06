//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qcom/tsens-8960.c
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
//

pub const CONFIG_ADDR: c_uint = 0x3640;
pub const CONFIG_ADDR_8660: c_uint = 0x3620;
// CONFIG_ADDR bitmasks
pub const CONFIG: c_uint = 0x9b;
pub const CONFIG_MASK: c_uint = 0xf;
pub const CONFIG_8660: c_int = 1;
pub const CONFIG_SHIFT_8660: c_int = 28;

pub const CNTL_ADDR: c_uint = 0x3620;
// CNTL_ADDR bitmasks

pub const SENSOR0_SHIFT: c_int = 3;
pub const THRESHOLD_ADDR: c_uint = 0x3624;
pub const INT_STATUS_ADDR: c_uint = 0x363c;
pub const S0_STATUS_OFF: c_uint = 0x3628;
pub const S1_STATUS_OFF: c_uint = 0x362c;
pub const S2_STATUS_OFF: c_uint = 0x3630;
pub const S3_STATUS_OFF: c_uint = 0x3634;
pub const S4_STATUS_OFF: c_uint = 0x3638;
pub const S5_STATUS_OFF: c_uint = 0x3664  /* Sensors 5-10 found on apq8064/msm8960 */;
pub const S6_STATUS_OFF: c_uint = 0x3668;
pub const S7_STATUS_OFF: c_uint = 0x366c;
pub const S8_STATUS_OFF: c_uint = 0x3670;
pub const S9_STATUS_OFF: c_uint = 0x3674;
pub const S10_STATUS_OFF: c_uint = 0x3678;
// Original slope - 350 to compensate mC to C inaccuracy
    static u32 tsens_msm8960_slope[] = {
    826, 826, 804, 826,
    761, 782, 782, 849,
    782, 849, 782
    };
#[no_mangle]
unsafe extern "C" fn suspend_8960(priv: *mut tsens_priv) -> c_int {
    static int suspend_8960(struct tsens_priv *priv)
    {
    int ret;
    unsigned int mask;
    struct regmap *map = priv.tm_map;
    ret = regmap_read(map, THRESHOLD_ADDR, &priv.ctx.threshold);
    if (ret)
    return ret;
    ret = regmap_read(map, CNTL_ADDR, &priv.ctx.control);
    if (ret)
    return ret;
    if (priv.num_sensors > 1)
    mask = SLP_CLK_ENA | EN;
    else
    mask = SLP_CLK_ENA_8660 | EN;
    ret = regmap_update_bits(map, CNTL_ADDR, mask, 0);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn resume_8960(priv: *mut tsens_priv) -> c_int {
    static int resume_8960(struct tsens_priv *priv)
    {
    int ret;
    struct regmap *map = priv.tm_map;
    ret = regmap_update_bits(map, CNTL_ADDR, SW_RST, SW_RST);
    if (ret)
    return ret;
//
// Separate CONFIG restore is not needed only for 8660 as
// config is part of CTRL Addr and its restored as such
//
    if (priv.num_sensors > 1) {
    ret = regmap_update_bits(map, CONFIG_ADDR, CONFIG_MASK, CONFIG);
    if (ret)
    return ret;
    }
    ret = regmap_write(map, THRESHOLD_ADDR, priv.ctx.threshold);
    if (ret)
    return ret;
    ret = regmap_write(map, CNTL_ADDR, priv.ctx.control);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_8960(priv: *mut tsens_priv, id: c_int) -> c_int {
    static int enable_8960(struct tsens_priv *priv, int id)
    {
    int ret;
    u32 reg, mask = BIT(id);
    ret = regmap_read(priv.tm_map, CNTL_ADDR, &reg);
    if (ret)
    return ret;
// HARDWARE BUG:
// On platforms with more than 6 sensors, all remaining sensors
// must be enabled together, otherwise undefined results are expected.
// (Sensor 6-7 disabled, Sensor 3 disabled...) In the original driver,
// all the sensors are enabled in one step hence this bug is not
// triggered.
//
    if (id > 5)
    mask = GENMASK(10, 6);
    mask <<= SENSOR0_SHIFT;
// Sensors already enabled. Skip.
    if ((reg & mask) == mask)
    return 0;
    ret = regmap_write(priv.tm_map, CNTL_ADDR, reg | SW_RST);
    if (ret)
    return ret;
    reg |= MEASURE_PERIOD;
    if (priv.num_sensors > 1)
    reg |= mask | SLP_CLK_ENA | EN;
    else
    reg |= mask | SLP_CLK_ENA_8660 | EN;
    ret = regmap_write(priv.tm_map, CNTL_ADDR, reg);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn disable_8960(priv: *mut tsens_priv) {
    static void disable_8960(struct tsens_priv *priv)
    {
    int ret;
    u32 reg_cntl;
    u32 mask;
    mask = GENMASK(priv.num_sensors - 1, 0);
    mask <<= SENSOR0_SHIFT;
    mask |= EN;
    ret = regmap_read(priv.tm_map, CNTL_ADDR, &reg_cntl);
    if (ret)
    return;
    reg_cntl &= ~mask;
    if (priv.num_sensors > 1)
    reg_cntl &= ~SLP_CLK_ENA;
    else
    reg_cntl &= ~SLP_CLK_ENA_8660;
    regmap_write(priv.tm_map, CNTL_ADDR, reg_cntl);
    }
#[no_mangle]
unsafe extern "C" fn calibrate_8960(priv: *mut tsens_priv) -> c_int {
    static int calibrate_8960(struct tsens_priv *priv)
    {
    int i;
    char *data;
    u32 p1[11];
    data = qfprom_read(priv.dev, "calib");
    if (IS_ERR(data))
    data = qfprom_read(priv.dev, "calib_backup");
    if (IS_ERR(data))
    return PTR_ERR(data);
    for (i = 0; i < priv.num_sensors; i++) {
    p1[i] = data[i];
    priv.sensor[i].slope = tsens_msm8960_slope[i];
    }
    compute_intercept_slope(priv, p1, core::ptr::null_mut(), ONE_PT_CALIB);
    kfree(data);
    return 0;
    }
    static const struct reg_field tsens_8960_regfields[MAX_REGFIELDS] = {
// ----- SROT ------
// No VERSION information
// CNTL
    [TSENS_EN]     = REG_FIELD(CNTL_ADDR,  0, 0),
    [TSENS_SW_RST] = REG_FIELD(CNTL_ADDR,  1, 1),
// 8960 has 5 sensors, 8660 has 11, we only handle 5
    [SENSOR_EN]    = REG_FIELD(CNTL_ADDR,  3, 7),
// ----- TM ------
// INTERRUPT ENABLE
// NO INTERRUPT ENABLE
// Single UPPER/LOWER TEMPERATURE THRESHOLD for all sensors
    [LOW_THRESH_0]   = REG_FIELD(THRESHOLD_ADDR,  0,  7),
    [UP_THRESH_0]    = REG_FIELD(THRESHOLD_ADDR,  8, 15),
// MIN_THRESH_0 and MAX_THRESH_0 are not present in the regfield
// Recycle CRIT_THRESH_0 and 1 to set the required regs to hardcoded temp
// MIN_THRESH_0 -> CRIT_THRESH_1
// MAX_THRESH_0 -> CRIT_THRESH_0
//
    [CRIT_THRESH_1]   = REG_FIELD(THRESHOLD_ADDR, 16, 23),
    [CRIT_THRESH_0]   = REG_FIELD(THRESHOLD_ADDR, 24, 31),
// UPPER/LOWER INTERRUPT [CLEAR/STATUS]
// 1 == clear, 0 == normal operation
    [LOW_INT_CLEAR_0]   = REG_FIELD(CNTL_ADDR,  9,  9),
    [UP_INT_CLEAR_0]    = REG_FIELD(CNTL_ADDR, 10, 10),
// NO CRITICAL INTERRUPT SUPPORT on 8960
// Sn_STATUS
    [LAST_TEMP_0]  = REG_FIELD(S0_STATUS_OFF,  0,  7),
    [LAST_TEMP_1]  = REG_FIELD(S1_STATUS_OFF,  0,  7),
    [LAST_TEMP_2]  = REG_FIELD(S2_STATUS_OFF,  0,  7),
    [LAST_TEMP_3]  = REG_FIELD(S3_STATUS_OFF,  0,  7),
    [LAST_TEMP_4]  = REG_FIELD(S4_STATUS_OFF,  0,  7),
    [LAST_TEMP_5]  = REG_FIELD(S5_STATUS_OFF,  0,  7),
    [LAST_TEMP_6]  = REG_FIELD(S6_STATUS_OFF,  0,  7),
    [LAST_TEMP_7]  = REG_FIELD(S7_STATUS_OFF,  0,  7),
    [LAST_TEMP_8]  = REG_FIELD(S8_STATUS_OFF,  0,  7),
    [LAST_TEMP_9]  = REG_FIELD(S9_STATUS_OFF,  0,  7),
    [LAST_TEMP_10] = REG_FIELD(S10_STATUS_OFF, 0,  7),
// No VALID field on 8960
// TSENS_INT_STATUS bits: 1 == threshold violated
    [MIN_STATUS_0] = REG_FIELD(INT_STATUS_ADDR, 0, 0),
    [LOWER_STATUS_0] = REG_FIELD(INT_STATUS_ADDR, 1, 1),
    [UPPER_STATUS_0] = REG_FIELD(INT_STATUS_ADDR, 2, 2),
// No CRITICAL field on 8960
    [MAX_STATUS_0] = REG_FIELD(INT_STATUS_ADDR, 3, 3),
// TRDY: 1=ready, 0=in progress
    [TRDY] = REG_FIELD(INT_STATUS_ADDR, 7, 7),
    };
    static const struct tsens_ops ops_8960 = {
    .init		= init_common,
    .calibrate	= calibrate_8960,
    .get_temp	= get_temp_common,
    .enable		= enable_8960,
    .disable	= disable_8960,
    .suspend	= suspend_8960,
    .resume		= resume_8960,
    };
    static struct tsens_features tsens_8960_feat = {
    .ver_major	= VER_0,
    .crit_int	= 0,
    .combo_int	= 0,
    .adc		= 1,
    .srot_split	= 0,
    .max_sensors	= 11,
    .trip_min_temp	= -40000,
    .trip_max_temp	= 120000,
    };
    struct tsens_plat_data data_8960 = {
    .num_sensors	= 11,
    .ops		= &ops_8960,
    .feat		= &tsens_8960_feat,
    .fields		= tsens_8960_regfields,
    };
