//! Automatically rewritten from C to Rust
//! Source: drivers/iio/frequency/adf41513.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ADF41513 SPI PLL Frequency Synthesizer driver
//
// Copyright 2026 Analog Devices Inc.
//

// Registers
pub const ADF41513_REG0: c_int = 0;
pub const ADF41513_REG1: c_int = 1;
pub const ADF41513_REG2: c_int = 2;
pub const ADF41513_REG3: c_int = 3;
pub const ADF41513_REG4: c_int = 4;
pub const ADF41513_REG5: c_int = 5;
pub const ADF41513_REG6: c_int = 6;
pub const ADF41513_REG7: c_int = 7;
pub const ADF41513_REG8: c_int = 8;
pub const ADF41513_REG9: c_int = 9;
pub const ADF41513_REG10: c_int = 10;
pub const ADF41513_REG11: c_int = 11;
pub const ADF41513_REG12: c_int = 12;
pub const ADF41513_REG13: c_int = 13;
pub const ADF41513_REG_NUM: c_int = 14;

pub const ADF41513_SYNC_DIFF: c_int = 0;

// REG0 Bit Definitions

// REG1 Bit Definitions

// REG2 Bit Definitions

// REG3 Bit Definitions

// REG4 Bit Definitions

// REG5 Bit Definitions

// REG6 Bit Definitions

// REG7 Bit Definitions

// REG9 Bit Definitions

// REG11 Bit Definitions

// REG12 Bit Definitions

// MUXOUT Selection
pub const ADF41513_MUXOUT_TRISTATE: c_uint = 0x0;
pub const ADF41513_MUXOUT_DVDD: c_uint = 0x1;
pub const ADF41513_MUXOUT_DGND: c_uint = 0x2;
pub const ADF41513_MUXOUT_R_DIV: c_uint = 0x3;
pub const ADF41513_MUXOUT_N_DIV: c_uint = 0x4;
pub const ADF41513_MUXOUT_DIG_LD: c_uint = 0x6;
pub const ADF41513_MUXOUT_SDO: c_uint = 0x7;
pub const ADF41513_MUXOUT_READBACK: c_uint = 0x8;
pub const ADF41513_MUXOUT_CLK1_DIV: c_uint = 0xA;
pub const ADF41513_MUXOUT_R_DIV2: c_uint = 0xD;
pub const ADF41513_MUXOUT_N_DIV2: c_uint = 0xE;
// DLD Mode Selection
pub const ADF41513_DLD_TRISTATE: c_uint = 0x0;
pub const ADF41513_DLD_DIG_LD: c_uint = 0x1;
pub const ADF41513_DLD_LOW: c_uint = 0x2;
pub const ADF41513_DLD_HIGH: c_uint = 0x3;
// Prescaler Selection
pub const ADF41513_PRESCALER_4_5: c_int = 0;
pub const ADF41513_PRESCALER_8_9: c_int = 1;
pub const ADF41513_PRESCALER_AUTO: c_int = 2;
// CLK Divider mode
pub const ADF41513_CLK_DIV_MODE_OFF: c_int = 0;
pub const ADF41513_CLK_DIV_MODE_PHASE_RESYNC: c_int = 2;
// Specifications

pub const ADF41513_MIN_INT_4_5: c_int = 20;
pub const ADF41513_MAX_INT_4_5: c_int = 511;
pub const ADF41513_MIN_INT_8_9: c_int = 64;
pub const ADF41513_MAX_INT_8_9: c_int = 1023;
pub const ADF41513_MIN_INT_FRAC_4_5: c_int = 23;
pub const ADF41513_MIN_INT_FRAC_8_9: c_int = 75;
pub const ADF41513_MIN_R_CNT: c_int = 1;
pub const ADF41513_MAX_R_CNT: c_int = 32;
pub const ADF41513_MIN_R_SET: c_int = 1800;
pub const ADF41513_DEFAULT_R_SET: c_int = 2700;
pub const ADF41513_MAX_R_SET: c_int = 10000;
pub const ADF41513_MIN_CP_VOLTAGE_mV: c_int = 810;
pub const ADF41513_DEFAULT_CP_VOLTAGE_mV: c_int = 6480;
pub const ADF41513_MAX_CP_VOLTAGE_mV: c_int = 12960;
pub const ADF41513_MIN_CP_CURRENT_uA: c_int = 81;
pub const ADF41513_MAX_CP_CURRENT_uA: c_int = 7200;
pub const ADF41513_LD_COUNT_FAST_MIN: c_int = 2;
pub const ADF41513_LD_COUNT_FAST_LIMIT: c_int = 64;
pub const ADF41513_LD_COUNT_MIN: c_int = 64;
pub const ADF41513_LD_COUNT_MAX: c_int = 8192;

pub const ADF41513_HZ_DECIMAL_SCALE: c_int = 6;
pub const ADF41513_PS_BIAS_INIT: c_uint = 0x2;

    enum adf41513_pll_mode {
    ADF41513_MODE_INVALID,
    ADF41513_MODE_INTEGER_N,
    ADF41513_MODE_FIXED_MODULUS,
    ADF41513_MODE_VARIABLE_MODULUS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf41513_chip_info {
    pub name: *const c_char,
    pub max_rf_freq_hz: u64,
    pub has_prescaler_8_9: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf41513_data {
    pub power_up_frequency_hz: u64,
    pub freq_resolution_uhz: u64,
    pub phase_resync_period_ns: u32,
    pub charge_pump_voltage_mv: u32,
    pub lock_detect_count: u32,
    pub ref_div_factor: u8,
    pub ref_doubler_en: bool,
    pub ref_div2_en: bool,
    pub phase_detector_polarity: bool,
    pub logic_lvl_1v8_en: bool,
    pub le_sync_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf41513_pll_settings {
    pub mode: enum adf41513_pll_mode,
// reference path parameters
    pub r_counter: u8,
    pub ref_doubler: u8,
    pub ref_div2: u8,
    pub prescaler: u8,
// frequency parameters
    pub target_frequency_uhz: u64,
    pub actual_frequency_uhz: u64,
    pub pfd_frequency_uhz: u64,
// pll parameters
    pub frac1: u32,
    pub frac2: u32,
    pub mod2: u32,
    pub int_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf41513_state {
    pub chip_info: *const adf41513_chip_info,
    pub spi: *mut spi_device,
    pub lock_detect: *mut gpio_desc,
    pub ref_clk: *mut clk,
    pub ref_freq_hz: u32,
//
// Lock for accessing device registers. Some operations require
// multiple consecutive R/W operations, during which the device
// shouldn't be interrupted. The buffers are also shared across
// all operations so need to be protected on stand alone reads and
// writes.
//
    pub lock: mutex,
// Cached register values
    pub regs: [u32; ADF41513_REG_NUM],
    pub regs_hw: [u32; ADF41513_REG_NUM],
    pub data: adf41513_data,
    pub settings: adf41513_pll_settings,
    pub powerdown: bool,
}

    static const u16 adf41513_ld_window_x10_ns[] = {
    9, 12, 16, 17, 21, 28, 29, 35,			/* 0 - 7 */
    43, 47, 49, 52, 70, 79, 115,			/* 8 - 14 */
    };
    static const u8 adf41513_ldp_bias[] = {
    0xC, 0xD, 0xE, 0x8, 0x9, 0x4, 0xA, 0x5,		/* 0 - 7 */
    0x0, 0x6, 0xB, 0x1, 0x2, 0x7, 0x3,		/* 8 - 14 */
    };
    static const char * const adf41513_power_supplies[] = {
    "avdd1", "avdd2", "avdd3", "avdd4", "avdd5", "vp",
    };
#[no_mangle]
unsafe extern "C" fn adf41513_sync_config(st: *mut adf41513_state, sync_mask: u16) -> c_int {
    static int adf41513_sync_config(struct adf41513_state *st, u16 sync_mask)
    {
    __be32 d32;
    int ret;
// write registers in reverse order (R13 to R0)
    for (int i = ADF41513_REG13; i >= ADF41513_REG0; i--) {
    if (st.regs_hw[i] == st.regs[i] && !(sync_mask & BIT(i)))
    continue;
    d32 = cpu_to_be32(st.regs[i] | i);
    ret = spi_write_then_read(st.spi, &d32, sizeof(d32), core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    st.regs_hw[i] = st.regs[i];
    dev_dbg(&st.spi.dev, "REG%d <= 0x%08X\n", i, st.regs[i] | i);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf41513_pll_get_rate(st: *mut adf41513_state) -> u64 {
    static u64 adf41513_pll_get_rate(struct adf41513_state *st)
    {
    struct adf41513_pll_settings *cfg = &st.settings;
    if (cfg.mode != ADF41513_MODE_INVALID)
    return cfg.actual_frequency_uhz;
// get pll settings from regs_hw
    cfg.int_val = FIELD_GET(ADF41513_REG0_INT_MSK, st.regs_hw[ADF41513_REG0]);
    cfg.frac1 = FIELD_GET(ADF41513_REG1_FRAC1_MSK, st.regs_hw[ADF41513_REG1]);
    cfg.frac2 = FIELD_GET(ADF41513_REG3_FRAC2_MSK, st.regs_hw[ADF41513_REG3]);
    cfg.mod2 = FIELD_GET(ADF41513_REG4_MOD2_MSK, st.regs_hw[ADF41513_REG4]);
    cfg.r_counter = FIELD_GET(ADF41513_REG5_R_CNT_MSK, st.regs_hw[ADF41513_REG5]);
    cfg.ref_doubler = FIELD_GET(ADF41513_REG5_REF_DOUBLER_MSK, st.regs_hw[ADF41513_REG5]);
    cfg.ref_div2 = FIELD_GET(ADF41513_REG5_RDIV2_MSK, st.regs_hw[ADF41513_REG5]);
    cfg.prescaler = FIELD_GET(ADF41513_REG5_PRESCALER_MSK, st.regs_hw[ADF41513_REG5]);
    if (!cfg.mod2)
    cfg.mod2 = 1;
    if (!cfg.r_counter)
    cfg.r_counter = ADF41513_MAX_R_CNT;
// calculate pfd frequency
    cfg.pfd_frequency_uhz = (u64)st.ref_freq_hz * MICRO;
    if (cfg.ref_doubler)
    cfg.pfd_frequency_uhz <<= 1;
    if (cfg.ref_div2)
    cfg.pfd_frequency_uhz >>= 1;
    cfg.pfd_frequency_uhz = div_u64(cfg.pfd_frequency_uhz, cfg.r_counter);
    cfg.actual_frequency_uhz = (u64)cfg.int_val * cfg.pfd_frequency_uhz;
// check if int mode is selected
    if (FIELD_GET(ADF41513_REG6_INT_MODE_MSK, st.regs_hw[ADF41513_REG6])) {
    cfg.mode = ADF41513_MODE_INTEGER_N;
    } else {
    cfg.actual_frequency_uhz += mul_u64_u32_div(cfg.pfd_frequency_uhz,
    cfg.frac1,
    ADF41513_FIXED_MODULUS);
// check if variable modulus is selected
    if (FIELD_GET(ADF41513_REG0_VAR_MOD_MSK, st.regs_hw[ADF41513_REG0])) {
    cfg.actual_frequency_uhz +=
    mul_u64_u64_div_u64(cfg.frac2,
    cfg.pfd_frequency_uhz,
    (u64)cfg.mod2 * ADF41513_FIXED_MODULUS);
    cfg.mode = ADF41513_MODE_VARIABLE_MODULUS;
    } else {
// LSB_P1 offset
    if (!FIELD_GET(ADF41513_REG5_LSB_P1_MSK, st.regs_hw[ADF41513_REG5]))
    cfg.actual_frequency_uhz +=
    div_u64(cfg.pfd_frequency_uhz,
    2 * ADF41513_FIXED_MODULUS);
    cfg.mode = ADF41513_MODE_FIXED_MODULUS;
    }
    }
    cfg.target_frequency_uhz = cfg.actual_frequency_uhz;
    return cfg.actual_frequency_uhz;
    }
    static int adf41513_calc_pfd_frequency(struct adf41513_state *st,
    struct adf41513_pll_settings *result,
    u64 fpfd_limit_uhz)
    {
    result.ref_div2 = st.data.ref_div2_en;
    result.ref_doubler = st.data.ref_doubler_en;
    result.r_counter = st.data.ref_div_factor - 1;
    do {
    result.r_counter++;
// f_PFD = REF_IN × ((1 + D)/(R × (1 + T)))
    result.pfd_frequency_uhz = (u64)st.ref_freq_hz * MICRO;
    if (result.ref_doubler)
    result.pfd_frequency_uhz <<= 1;
    if (result.ref_div2)
    result.pfd_frequency_uhz >>= 1;
    result.pfd_frequency_uhz = div_u64(result.pfd_frequency_uhz,
    result.r_counter);
    } while (result.pfd_frequency_uhz > fpfd_limit_uhz);
    if (result.r_counter > ADF41513_MAX_R_CNT) {
    dev_err(&st.spi.dev, "Cannot optimize PFD frequency\n");
    return -ERANGE;
    }
    return 0;
    }
    static int adf41513_calc_integer_n(struct adf41513_state *st,
    struct adf41513_pll_settings *result)
    {
    u32 max_int = st.chip_info.has_prescaler_8_9 ?
    ADF41513_MAX_INT_8_9 : ADF41513_MAX_INT_4_5;
    u64 freq_error_uhz;
    u32 int_val = div64_u64_rem(result.target_frequency_uhz, result.pfd_frequency_uhz,
    &freq_error_uhz);
// check if freq error is within a tolerance of 1/2 resolution
    if (freq_error_uhz > (result.pfd_frequency_uhz >> 1) && int_val < max_int) {
    int_val++;
    freq_error_uhz = result.pfd_frequency_uhz - freq_error_uhz;
    }
    if (freq_error_uhz > st.data.freq_resolution_uhz)
    return -ERANGE;
// set prescaler
    if (st.chip_info.has_prescaler_8_9 && int_val >= ADF41513_MIN_INT_8_9 &&
    int_val <= ADF41513_MAX_INT_8_9)
    result.prescaler = 1;
#[no_mangle]
pub unsafe extern "C" fn if(ADF41513_MAX_INT_4_5: int_val >= ADF41513_MIN_INT_4_5 && int_val <=) -> else {
    else if (int_val >= ADF41513_MIN_INT_4_5 && int_val <= ADF41513_MAX_INT_4_5)
    result.prescaler = 0;
    else
    return -ERANGE;
    result.actual_frequency_uhz = (u64)int_val * result.pfd_frequency_uhz;
    result.mode = ADF41513_MODE_INTEGER_N;
    result.int_val = int_val;
    result.frac1 = 0;
    result.frac2 = 0;
    result.mod2 = 0;
    return 0;
    }
    static int adf41513_calc_fixed_mod(struct adf41513_state *st,
    struct adf41513_pll_settings *result)
    {
    let mut resolution_uhz: u64 = div_u64(result.pfd_frequency_uhz, ADF41513_FIXED_MODULUS);
    let mut target_frequency_uhz: u64 = result.target_frequency_uhz;
    u64 freq_error_uhz;
    u32 int_val, frac1;
    let mut lsb_p1_offset: bool = !FIELD_GET(ADF41513_REG5_LSB_P1_MSK, st.regs[ADF41513_REG5]);
// LSB_P1 adds a frequency offset of f_pfd/2^26
    if (lsb_p1_offset)
    target_frequency_uhz -= resolution_uhz >> 1;
    int_val = div64_u64_rem(target_frequency_uhz, result.pfd_frequency_uhz,
    &freq_error_uhz);
    if (st.chip_info.has_prescaler_8_9 && int_val >= ADF41513_MIN_INT_FRAC_8_9 &&
    int_val <= ADF41513_MAX_INT_8_9)
    result.prescaler = 1;
#[no_mangle]
pub unsafe extern "C" fn if(ADF41513_MAX_INT_4_5: int_val >= ADF41513_MIN_INT_FRAC_4_5 && int_val <=) -> else {
    else if (int_val >= ADF41513_MIN_INT_FRAC_4_5 && int_val <= ADF41513_MAX_INT_4_5)
    result.prescaler = 0;
    else
    return -ERANGE;
// compute frac1 and fixed modulus error
    frac1 = mul_u64_u64_div_u64(freq_error_uhz, ADF41513_FIXED_MODULUS,
    result.pfd_frequency_uhz);
    freq_error_uhz -= mul_u64_u32_div(result.pfd_frequency_uhz, frac1,
    ADF41513_FIXED_MODULUS);
// check if freq error is within a tolerance of 1/2 resolution
    if (freq_error_uhz > (resolution_uhz >> 1) && frac1 < (ADF41513_FIXED_MODULUS - 1)) {
    frac1++;
    freq_error_uhz = freq_error_uhz < resolution_uhz ?
    resolution_uhz - freq_error_uhz : 0;
    }
    if (freq_error_uhz > st.data.freq_resolution_uhz)
    return -ERANGE;
// integer part
    result.actual_frequency_uhz = (u64)int_val * result.pfd_frequency_uhz;
// fractional part
    if (lsb_p1_offset)
    result.actual_frequency_uhz +=	(resolution_uhz >> 1);
    result.actual_frequency_uhz += mul_u64_u32_div(result.pfd_frequency_uhz, frac1,
    ADF41513_FIXED_MODULUS);
    result.mode = ADF41513_MODE_FIXED_MODULUS;
    result.int_val = int_val;
    result.frac1 = frac1;
    result.frac2 = 0;
    result.mod2 = 0;
    return 0;
    }
    static int adf41513_calc_variable_mod(struct adf41513_state *st,
    struct adf41513_pll_settings *result)
    {
    u64 freq_error_uhz, mod2;
    u32 frac1, frac2;
    u32 int_val = div64_u64_rem(result.target_frequency_uhz,
    result.pfd_frequency_uhz, &freq_error_uhz);
    if (st.chip_info.has_prescaler_8_9 && int_val >= ADF41513_MIN_INT_FRAC_8_9 &&
    int_val <= ADF41513_MAX_INT_8_9)
    result.prescaler = 1;
#[no_mangle]
pub unsafe extern "C" fn if(ADF41513_MAX_INT_4_5: int_val >= ADF41513_MIN_INT_FRAC_4_5 && int_val <=) -> else {
    else if (int_val >= ADF41513_MIN_INT_FRAC_4_5 && int_val <= ADF41513_MAX_INT_4_5)
    result.prescaler = 0;
    else
    return -ERANGE;
// calculate required mod2 based on target resolution / 2
    mod2 = DIV64_U64_ROUND_CLOSEST(result.pfd_frequency_uhz << 1,
    st.data.freq_resolution_uhz * ADF41513_FIXED_MODULUS);
// ensure mod2 is at least 2 for meaningful operation
    mod2 = clamp(mod2, 2, ADF41513_MAX_MOD2);
// calculate frac1 and frac2
    frac1 = mul_u64_u64_div_u64(freq_error_uhz, ADF41513_FIXED_MODULUS,
    result.pfd_frequency_uhz);
    frac2 = mul_u64_u64_div_u64(freq_error_uhz, mod2 * ADF41513_FIXED_MODULUS,
    result.pfd_frequency_uhz) - mod2 * frac1;
// integer part
    result.actual_frequency_uhz = (u64)int_val * result.pfd_frequency_uhz;
// fractional part
    result.actual_frequency_uhz += mul_u64_u64_div_u64(mod2 * frac1 + frac2,
    result.pfd_frequency_uhz,
    mod2 * ADF41513_FIXED_MODULUS);
    result.mode = ADF41513_MODE_VARIABLE_MODULUS;
    result.int_val = int_val;
    result.frac1 = frac1;
    result.frac2 = frac2;
    result.mod2 = mod2;
    return 0;
    }
    static int adf41513_calc_pll_settings(struct adf41513_state *st,
    struct adf41513_pll_settings *result,
    u64 rf_out_uhz)
    {
    let mut max_rf_freq_uhz: u64 = st.chip_info.max_rf_freq_hz * MICRO;
    let mut min_rf_freq_uhz: u64 = ADF41513_MIN_RF_FREQ_HZ * MICRO;
    u64 pfd_freq_limit_uhz;
    int ret;
    if (rf_out_uhz < min_rf_freq_uhz || rf_out_uhz > max_rf_freq_uhz) {
    dev_err(&st.spi.dev, "RF frequency %llu uHz out of range [%llu, %llu] uHz\n",
    rf_out_uhz, min_rf_freq_uhz, max_rf_freq_uhz);
    return -EINVAL;
    }
    result.target_frequency_uhz = rf_out_uhz;
// try integer-N first (best phase noise performance)
    pfd_freq_limit_uhz = min(div_u64(rf_out_uhz, ADF41513_MIN_INT_4_5),
    ADF41513_MAX_PFD_FREQ_INT_N_UHZ);
    ret = adf41513_calc_pfd_frequency(st, result, pfd_freq_limit_uhz);
    if (ret)
    return ret;
    if (adf41513_calc_integer_n(st, result) == 0)
    return 0;
// try fractional-N: recompute pfd frequency if necessary
    pfd_freq_limit_uhz = min(div_u64(rf_out_uhz, ADF41513_MIN_INT_FRAC_4_5),
    ADF41513_MAX_PFD_FREQ_FRAC_N_UHZ);
    if (pfd_freq_limit_uhz < result.pfd_frequency_uhz) {
    ret = adf41513_calc_pfd_frequency(st, result, pfd_freq_limit_uhz);
    if (ret)
    return ret;
    }
// fixed-modulus attempt
    if (adf41513_calc_fixed_mod(st, result) == 0)
    return 0;
// variable-modulus attempt
    ret = adf41513_calc_variable_mod(st, result);
    if (ret) {
    dev_err(&st.spi.dev,
    "no valid PLL configuration found for %llu uHz\n",
    rf_out_uhz);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf41513_set_bleed_val(st: *mut adf41513_state) {
    static void adf41513_set_bleed_val(struct adf41513_state *st)
    {
    u32 bleed_value, cp_index;
    if (!(st.regs[ADF41513_REG6] & ADF41513_REG6_BLEED_ENABLE_MSK))
    return;
    if (st.data.phase_detector_polarity)
    bleed_value = 90;
    else
    bleed_value = 144;
    cp_index = 1 + FIELD_GET(ADF41513_REG5_CP_CURRENT_MSK,
    st.regs[ADF41513_REG5]);
    bleed_value = div64_u64(st.settings.pfd_frequency_uhz * cp_index * bleed_value,
    1600ULL * MEGA * MICROHZ_PER_HZ);
    FIELD_MODIFY(ADF41513_REG6_BLEED_CURRENT_MSK, &st.regs[ADF41513_REG6],
    bleed_value);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_set_ld_window(st: *mut adf41513_state) {
    static void adf41513_set_ld_window(struct adf41513_state *st)
    {
//
// The ideal lock detector window size is halfway between the max
// window, set by the phase comparison period t_PFD = (1 / f_PFD),
// and the minimum is set by (I_BLEED/I_CP) × t_PFD
//
    u16 ld_window_10x_ns = div64_u64(10ULL * NSEC_PER_SEC * MICROHZ_PER_HZ,
    st.settings.pfd_frequency_uhz << 1);
    u8 ld_idx, ldp, ld_bias;
    if (st.settings.mode != ADF41513_MODE_INTEGER_N) {
// account for bleed current (deduced from eq.6 and eq.7)
    if (st.data.phase_detector_polarity)
    ld_window_10x_ns += 4;
    else
    ld_window_10x_ns += 6;
    }
    ld_idx = find_closest(ld_window_10x_ns, adf41513_ld_window_x10_ns,
    ARRAY_SIZE(adf41513_ld_window_x10_ns));
    ldp = (adf41513_ldp_bias[ld_idx] >> 2) & 0x3;
    ld_bias = adf41513_ldp_bias[ld_idx] & 0x3;
    FIELD_MODIFY(ADF41513_REG6_LDP_MSK, &st.regs[ADF41513_REG6], ldp);
    FIELD_MODIFY(ADF41513_REG9_LD_BIAS_MSK, &st.regs[ADF41513_REG9], ld_bias);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_set_phase_resync(st: *mut adf41513_state) {
    static void adf41513_set_phase_resync(struct adf41513_state *st)
    {
    u32 total_div, clk1_div, clk2_div;
    if (!st.data.phase_resync_period_ns)
    return;
// assuming both clock dividers hold similar values
    total_div = mul_u64_u64_div_u64(st.settings.pfd_frequency_uhz,
    st.data.phase_resync_period_ns,
    1ULL * MICROHZ_PER_HZ * NSEC_PER_SEC);
    clk1_div = clamp(int_sqrt(total_div), 1,
    ADF41513_MAX_CLK_DIVIDER);
    clk2_div = clamp(DIV_ROUND_CLOSEST(total_div, clk1_div), 1,
    ADF41513_MAX_CLK_DIVIDER);
    FIELD_MODIFY(ADF41513_REG5_CLK1_DIV_MSK, &st.regs[ADF41513_REG5],
    clk1_div);
    FIELD_MODIFY(ADF41513_REG7_CLK2_DIV_MSK, &st.regs[ADF41513_REG7],
    clk2_div);
// enable phase resync
    FIELD_MODIFY(ADF41513_REG7_CLK_DIV_MODE_MSK, &st.regs[ADF41513_REG7],
    ADF41513_CLK_DIV_MODE_PHASE_RESYNC);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_set_frequency(st: *mut adf41513_state, freq_uhz: u64, sync_mask: u16) -> c_int {
    static int adf41513_set_frequency(struct adf41513_state *st, u64 freq_uhz, u16 sync_mask)
    {
    struct adf41513_pll_settings result;
    let mut pfd_change: bool = false;
    let mut mode_change: bool = false;
    int ret;
    ret = adf41513_calc_pll_settings(st, &result, freq_uhz);
    if (ret < 0)
    return ret;
// apply computed results to pll settings
    pfd_change = st.settings.pfd_frequency_uhz != result.pfd_frequency_uhz;
    mode_change = st.settings.mode != result.mode;
    st.settings = result;
    dev_dbg(&st.spi.dev,
    "%s mode: int=%u, frac1=%u, frac2=%u, mod2=%u, fpdf=%llu Hz, prescaler=%s\n",
    (result.mode == ADF41513_MODE_INTEGER_N) ? "integer-n" :
    (result.mode == ADF41513_MODE_FIXED_MODULUS) ? "fixed-modulus" : "variable-modulus",
    result.int_val, result.frac1, result.frac2, result.mod2,
    div64_u64(result.pfd_frequency_uhz, MICRO),
    result.prescaler ? "8/9" : "4/5");
    st.regs[ADF41513_REG0] = FIELD_PREP(ADF41513_REG0_INT_MSK,
    st.settings.int_val);
    if (st.settings.mode == ADF41513_MODE_VARIABLE_MODULUS)
    st.regs[ADF41513_REG0] |= ADF41513_REG0_VAR_MOD_MSK;
    st.regs[ADF41513_REG1] = FIELD_PREP(ADF41513_REG1_FRAC1_MSK,
    st.settings.frac1);
    if (st.settings.mode != ADF41513_MODE_INTEGER_N)
    st.regs[ADF41513_REG1] |= ADF41513_REG1_DITHER2_MSK;
    st.regs[ADF41513_REG3] = FIELD_PREP(ADF41513_REG3_FRAC2_MSK,
    st.settings.frac2);
    FIELD_MODIFY(ADF41513_REG4_MOD2_MSK, &st.regs[ADF41513_REG4],
    st.settings.mod2);
    FIELD_MODIFY(ADF41513_REG5_R_CNT_MSK, &st.regs[ADF41513_REG5],
    st.settings.r_counter % ADF41513_MAX_R_CNT);
    FIELD_MODIFY(ADF41513_REG5_REF_DOUBLER_MSK, &st.regs[ADF41513_REG5],
    st.settings.ref_doubler);
    FIELD_MODIFY(ADF41513_REG5_RDIV2_MSK, &st.regs[ADF41513_REG5],
    st.settings.ref_div2);
    FIELD_MODIFY(ADF41513_REG5_PRESCALER_MSK, &st.regs[ADF41513_REG5],
    st.settings.prescaler);
    if (st.settings.mode == ADF41513_MODE_INTEGER_N) {
    st.regs[ADF41513_REG6] |= ADF41513_REG6_INT_MODE_MSK;
    st.regs[ADF41513_REG6] &= ~ADF41513_REG6_BLEED_ENABLE_MSK;
    } else {
    st.regs[ADF41513_REG6] &= ~ADF41513_REG6_INT_MODE_MSK;
    st.regs[ADF41513_REG6] |= ADF41513_REG6_BLEED_ENABLE_MSK;
    }
    if (pfd_change)
    adf41513_set_phase_resync(st);
    if (pfd_change || mode_change) {
    adf41513_set_bleed_val(st);
    adf41513_set_ld_window(st);
    }
    return adf41513_sync_config(st, sync_mask | ADF41513_SYNC_REG0);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_suspend(st: *mut adf41513_state) -> c_int {
    static int adf41513_suspend(struct adf41513_state *st)
    {
    st.regs[ADF41513_REG6] |= FIELD_PREP(ADF41513_REG6_POWER_DOWN_MSK, 1);
    st.regs[ADF41513_REG12] &= ~ADF41513_REG12_LE_SELECT_MSK;
    return adf41513_sync_config(st, ADF41513_SYNC_DIFF);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_resume(st: *mut adf41513_state) -> c_int {
    static int adf41513_resume(struct adf41513_state *st)
    {
    int ret;
    st.regs[ADF41513_REG6] &= ~ADF41513_REG6_POWER_DOWN_MSK;
    st.regs[ADF41513_REG12] &= ~ADF41513_REG12_LE_SELECT_MSK;
    ret = adf41513_sync_config(st, ADF41513_SYNC_ALL);
    if (ret)
    return ret;
    if (st.data.le_sync_en) {
    st.regs[ADF41513_REG12] |= ADF41513_REG12_LE_SELECT_MSK;
    ret = adf41513_sync_config(st, ADF41513_SYNC_DIFF);
    if (ret)
    return ret;
    }
    return 0;
    }
    static ssize_t adf41513_read_resolution(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    int vals[2];
    guard(mutex)(&st.lock);
    iio_val_s64_decompose(st.data.freq_resolution_uhz, &vals[0], &vals[1]);
    return iio_format_value(buf, IIO_VAL_DECIMAL64_MICRO, ARRAY_SIZE(vals), vals);
    }
    static ssize_t adf41513_read_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    u32 val;
    guard(mutex)(&st.lock);
    val = FIELD_GET(ADF41513_REG6_POWER_DOWN_MSK, st.regs_hw[ADF41513_REG6]);
    return sysfs_emit(buf, "%u\n", val);
    }
    static ssize_t adf41513_write_resolution(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    u64 freq_uhz;
    int ret;
    ret = kstrtoudec64(buf, ADF41513_HZ_DECIMAL_SCALE, &freq_uhz);
    if (ret)
    return ret;
    if (freq_uhz == 0 || freq_uhz > ADF41513_MAX_FREQ_RESOLUTION_UHZ)
    return -EINVAL;
    guard(mutex)(&st.lock);
    st.data.freq_resolution_uhz = freq_uhz;
    return len;
    }
    static ssize_t adf41513_write_powerdown(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    bool val;
    int ret;
    ret = kstrtobool(buf, &val);
    if (ret)
    return ret;
    guard(mutex)(&st.lock);
    if (val)
    ret = adf41513_suspend(st);
    else
    ret = adf41513_resume(st);
    if (ret)
    return ret;
    st.powerdown = val;
    return len;
    }
    static const struct iio_chan_spec_ext_info adf41513_ext_info[] = {
    {
    .name = "frequency_resolution",
    .read = adf41513_read_resolution,
    .write = adf41513_write_resolution,
    .shared = IIO_SEPARATE,
    },
    {
    .name = "powerdown",
    .read = adf41513_read_powerdown,
    .write = adf41513_write_powerdown,
    .shared = IIO_SEPARATE,
    },
    { }
    };
    static const struct iio_chan_spec adf41513_chan = {
    .type = IIO_ALTVOLTAGE,
    .indexed = 1,
    .output = 1,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_FREQUENCY) |
    BIT(IIO_CHAN_INFO_PHASE),
    .ext_info = adf41513_ext_info,
    };
    static int adf41513_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    u64 tmp64;
    guard(mutex)(&st.lock);
    switch (info) {
    case IIO_CHAN_INFO_FREQUENCY:
    if (st.lock_detect &&
    !gpiod_get_value_cansleep(st.lock_detect)) {
    dev_dbg(&st.spi.dev, "PLL un-locked\n");
    return -EBUSY;
    }
    tmp64 = adf41513_pll_get_rate(st);
    iio_val_s64_decompose(tmp64, val, val2);
    return IIO_VAL_DECIMAL64_MICRO;
    case IIO_CHAN_INFO_PHASE:
    tmp64 = FIELD_GET(ADF41513_REG2_PHASE_VAL_MSK,
    st.regs_hw[ADF41513_REG2]);
    tmp64 = (tmp64 * ADF41513_MAX_PHASE_MICRORAD) >> 12;
    iio_val_s64_decompose(tmp64, val, val2);
    return IIO_VAL_DECIMAL64_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int adf41513_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long info)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    let mut tmp64: u64 = iio_val_s64_compose(val, val2);
    u16 phase_val;
    int ret;
    guard(mutex)(&st.lock);
    switch (info) {
    case IIO_CHAN_INFO_FREQUENCY:
    return adf41513_set_frequency(st, tmp64, ADF41513_SYNC_DIFF);
    case IIO_CHAN_INFO_PHASE:
    if (tmp64 >= ADF41513_MAX_PHASE_MICRORAD)
    return -EINVAL;
    phase_val = DIV_U64_ROUND_CLOSEST(tmp64 << 12,
    ADF41513_MAX_PHASE_MICRORAD);
    phase_val = min(phase_val, ADF41513_MAX_PHASE_VAL);
    st.regs[ADF41513_REG2] |= ADF41513_REG2_PHASE_ADJ_MSK;
    FIELD_MODIFY(ADF41513_REG2_PHASE_VAL_MSK,
    &st.regs[ADF41513_REG2], phase_val);
    ret = adf41513_sync_config(st, ADF41513_SYNC_REG0);
// clear phase adjust for the next sync
    st.regs[ADF41513_REG2] &= ~ADF41513_REG2_PHASE_ADJ_MSK;
    return ret;
    default:
    return -EINVAL;
    }
    }
    static int adf41513_write_raw_get_fmt(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    long mask)
    {
    switch (mask) {
    case IIO_CHAN_INFO_FREQUENCY:
    case IIO_CHAN_INFO_PHASE:
    return IIO_VAL_DECIMAL64_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int adf41513_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct adf41513_state *st = iio_priv(indio_dev);
    if (reg > ADF41513_REG13)
    return -EINVAL;
    guard(mutex)(&st.lock);
    if (!readval) {
    if (reg <= ADF41513_REG6)
    st.settings.mode = ADF41513_MODE_INVALID;
    st.regs[reg] = writeval & ~0xF; /* Clear control bits */
    return adf41513_sync_config(st, BIT(reg));
    }
// readval = st->regs_hw[reg];
    return 0;
    }
    static const struct iio_info adf41513_info = {
    .read_raw = adf41513_read_raw,
    .write_raw = adf41513_write_raw,
    .write_raw_get_fmt = adf41513_write_raw_get_fmt,
    .debugfs_reg_access = &adf41513_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn adf41513_parse_fw(st: *mut adf41513_state) -> c_int {
    static int adf41513_parse_fw(struct adf41513_state *st)
    {
    struct device *dev = &st.spi.dev;
    u32 tmp, cp_resistance, cp_current;
    int ret;
    tmp = ADF41510_MAX_RF_FREQ_HZ / MEGA;
    device_property_read_u32(dev, "adi,power-up-frequency-mhz", &tmp);
    st.data.power_up_frequency_hz = (u64)tmp * MEGA;
    if (st.data.power_up_frequency_hz < ADF41513_MIN_RF_FREQ_HZ ||
    st.data.power_up_frequency_hz > st.chip_info.max_rf_freq_hz)
    return dev_err_probe(dev, -ERANGE,
    "power-up frequency %llu Hz out of range\n",
    st.data.power_up_frequency_hz);
    tmp = ADF41513_MIN_R_CNT;
    device_property_read_u32(dev, "adi,reference-div-factor", &tmp);
    if (tmp < ADF41513_MIN_R_CNT || tmp > ADF41513_MAX_R_CNT)
    return dev_err_probe(dev, -ERANGE,
    "invalid reference div factor %u\n", tmp);
    st.data.ref_div_factor = tmp;
    st.data.ref_div2_en = device_property_read_bool(dev, "adi,reference-div2-enable");
    st.data.ref_doubler_en = device_property_read_bool(dev, "adi,reference-doubler-enable");
    if (st.data.ref_doubler_en &&
    st.ref_freq_hz > ADF41513_MAX_REF_FREQ_DOUBLER_HZ) {
    return dev_err_probe(dev, -ERANGE,
    "Ref frequency not supported with doubler enabled\n");
    }
    cp_resistance = ADF41513_DEFAULT_R_SET;
    device_property_read_u32(dev, "adi,charge-pump-resistor-ohms", &cp_resistance);
    if (cp_resistance < ADF41513_MIN_R_SET || cp_resistance > ADF41513_MAX_R_SET)
    return dev_err_probe(dev, -ERANGE, "R_SET %u Ohms out of range\n", cp_resistance);
    st.data.charge_pump_voltage_mv = ADF41513_DEFAULT_CP_VOLTAGE_mV;
    ret = device_property_read_u32(dev, "adi,charge-pump-current-microamp", &cp_current);
    if (!ret) {
    if (cp_current < ADF41513_MIN_CP_CURRENT_uA ||
    cp_current > ADF41513_MAX_CP_CURRENT_uA)
    return dev_err_probe(dev, -ERANGE,
    "I_CP %u uA out of range\n", cp_current);
    tmp = DIV_ROUND_CLOSEST(cp_current * cp_resistance, MILLI);
    if (tmp < ADF41513_MIN_CP_VOLTAGE_mV || tmp > ADF41513_MAX_CP_VOLTAGE_mV)
    return dev_err_probe(dev, -ERANGE, "I_CP %u uA (%u Ohms) out of range\n",
    cp_current, cp_resistance);
    st.data.charge_pump_voltage_mv = tmp;
    }
    st.data.phase_detector_polarity =
    device_property_read_bool(dev, "adi,phase-detector-polarity-positive-enable");
    st.data.phase_resync_period_ns = 0;
    ret = device_property_read_u32(dev, "adi,phase-resync-period-ns", &tmp);
    if (!ret)
    st.data.phase_resync_period_ns = tmp;
    st.data.logic_lvl_1v8_en = device_property_read_bool(dev, "adi,logic-level-1v8-enable");
    tmp = ADF41513_LD_COUNT_MIN;
    device_property_read_u32(dev, "adi,lock-detector-count", &tmp);
    if (tmp < ADF41513_LD_COUNT_FAST_MIN || tmp > ADF41513_LD_COUNT_MAX ||
    !is_power_of_2(tmp))
    return dev_err_probe(dev, -ERANGE,
    "invalid lock detect count: %u\n", tmp);
    st.data.lock_detect_count = tmp;
// load enable sync
    st.data.le_sync_en = device_property_read_bool(dev, "adi,le-sync-enable");
    st.data.freq_resolution_uhz = MICROHZ_PER_HZ;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf41513_chip_disable(data: *mut c_void) {
    static void adf41513_chip_disable(void *data)
    {
    gpiod_set_value_cansleep(data, 0);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_close(data: *mut c_void) {
    static void adf41513_close(void *data)
    {
    adf41513_suspend(data);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_setup(dev: *mut device, st: *mut adf41513_state) -> c_int {
    static int adf41513_setup(struct device *dev, struct adf41513_state *st)
    {
    u32 tmp;
    int ret;
    memset(st.regs_hw, 0xFF, sizeof(st.regs_hw));
// assuming DLD pin is used for lock detection
    st.regs[ADF41513_REG5] = FIELD_PREP(ADF41513_REG5_DLD_MODES_MSK,
    ADF41513_DLD_DIG_LD);
    tmp = DIV_ROUND_CLOSEST(st.data.charge_pump_voltage_mv, ADF41513_MIN_CP_VOLTAGE_mV);
    st.regs[ADF41513_REG5] |= FIELD_PREP(ADF41513_REG5_CP_CURRENT_MSK, tmp - 1);
    st.regs[ADF41513_REG6] = ADF41513_REG6_ABP_MSK |
    ADF41513_REG6_LOL_ENABLE_MSK |
    ADF41513_REG6_SD_RESET_MSK;
    if (st.data.phase_detector_polarity)
    st.regs[ADF41513_REG6] |= ADF41513_REG6_PD_POLARITY_MSK;
    st.regs[ADF41513_REG7] = FIELD_PREP(ADF41513_REG7_PS_BIAS_MSK,
    ADF41513_PS_BIAS_INIT);
    tmp = ilog2(st.data.lock_detect_count);
    if (st.data.lock_detect_count < ADF41513_LD_COUNT_FAST_LIMIT) {
    tmp -= const_ilog2(ADF41513_LD_COUNT_FAST_MIN);
    st.regs[ADF41513_REG7] |= ADF41513_REG7_LD_CLK_SEL_MSK;
    } else {
    tmp -= const_ilog2(ADF41513_LD_COUNT_MIN);
    }
    st.regs[ADF41513_REG7] |= FIELD_PREP(ADF41513_REG7_LD_COUNT_MSK, tmp);
    st.regs[ADF41513_REG11] = ADF41513_REG11_POWER_DOWN_SEL_MSK;
    st.regs[ADF41513_REG12] = FIELD_PREP(ADF41513_REG12_LOGIC_LEVEL_MSK,
    st.data.logic_lvl_1v8_en ? 0 : 1);
// perform initialization sequence with power-up frequency
    ret = adf41513_set_frequency(st, st.data.power_up_frequency_hz * MICRO,
    ADF41513_SYNC_ALL);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, adf41513_close, st);
    if (ret)
    return ret;
    if (st.data.le_sync_en) {
    st.regs[ADF41513_REG12] |= ADF41513_REG12_LE_SELECT_MSK;
    ret = adf41513_sync_config(st, ADF41513_SYNC_DIFF);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf41513_pm_suspend(dev: *mut device) -> c_int {
    static int adf41513_pm_suspend(struct device *dev)
    {
    struct adf41513_state *st = dev_get_drvdata(dev);
    guard(mutex)(&st.lock);
    return adf41513_suspend(st);
    }
#[no_mangle]
unsafe extern "C" fn adf41513_pm_resume(dev: *mut device) -> c_int {
    static int adf41513_pm_resume(struct device *dev)
    {
    struct adf41513_state *st = dev_get_drvdata(dev);
    guard(mutex)(&st.lock);
    if (st.powerdown)
    return 0; /* nothing to do */
    return adf41513_resume(st);
    }
    static const struct adf41513_chip_info adf41510_chip_info = {
    .name = "adf41510",
    .max_rf_freq_hz = ADF41510_MAX_RF_FREQ_HZ,
    .has_prescaler_8_9 = false,
    };
    static const struct adf41513_chip_info adf41513_chip_info = {
    .name = "adf41513",
    .max_rf_freq_hz = ADF41513_MAX_RF_FREQ_HZ,
    .has_prescaler_8_9 = true,
    };
#[no_mangle]
unsafe extern "C" fn adf41513_probe(spi: *mut spi_device) -> c_int {
    static int adf41513_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct gpio_desc *chip_enable;
    struct iio_dev *indio_dev;
    struct adf41513_state *st;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.chip_info = spi_get_device_match_data(spi);
    if (!st.chip_info)
    return -EINVAL;
    spi_set_drvdata(spi, st);
    st.ref_clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(st.ref_clk))
    return PTR_ERR(st.ref_clk);
    st.ref_freq_hz = clk_get_rate(st.ref_clk);
    if (st.ref_freq_hz < ADF41513_MIN_REF_FREQ_HZ ||
    st.ref_freq_hz > ADF41513_MAX_REF_FREQ_HZ)
    return dev_err_probe(dev, -ERANGE,
    "reference frequency %u Hz out of range\n",
    st.ref_freq_hz);
    ret = adf41513_parse_fw(st);
    if (ret)
    return ret;
    ret = devm_regulator_bulk_get_enable(dev,
    ARRAY_SIZE(adf41513_power_supplies),
    adf41513_power_supplies);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get and enable regulators\n");
    st.lock_detect = devm_gpiod_get_optional(dev, "lock-detect", GPIOD_IN);
    if (IS_ERR(st.lock_detect))
    return dev_err_probe(dev, PTR_ERR(st.lock_detect),
    "fail to request lock detect GPIO\n");
    chip_enable = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(chip_enable))
    return dev_err_probe(dev, PTR_ERR(chip_enable),
    "fail to request chip enable GPIO\n");
    ret = devm_add_action_or_reset(dev, adf41513_chip_disable, chip_enable);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add disable action\n");
    ret = devm_mutex_init(dev, &st.lock);
    if (ret)
    return ret;
    indio_dev.name = st.chip_info.name;
    indio_dev.info = &adf41513_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = &adf41513_chan;
    indio_dev.num_channels = 1;
    ret = adf41513_setup(dev, st);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to setup device\n");
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id adf41513_id[] = {
    { .name = "adf41510", .driver_data = (kernel_ulong_t)&adf41510_chip_info },
    { .name = "adf41513", .driver_data = (kernel_ulong_t)&adf41513_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adf41513_id);
    static const struct of_device_id adf41513_of_match[] = {
    { .compatible = "adi,adf41510", .data = &adf41510_chip_info },
    { .compatible = "adi,adf41513", .data = &adf41513_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, adf41513_of_match);
    static DEFINE_SIMPLE_DEV_PM_OPS(adf41513_pm_ops, adf41513_pm_suspend, adf41513_pm_resume);
    static struct spi_driver adf41513_driver = {
    .driver = {
    .name = "adf41513",
    .pm = pm_ptr(&adf41513_pm_ops),
    .of_match_table = adf41513_of_match,
    },
    .probe = adf41513_probe,
    .id_table = adf41513_id,
    };
    module_spi_driver(adf41513_driver);
    MODULE_AUTHOR("Rodrigo Alencar <rodrigo.alencar@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADF41513 PLL Frequency Synthesizer");
    MODULE_LICENSE("GPL");
