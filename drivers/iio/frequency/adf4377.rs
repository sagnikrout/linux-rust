//! Automatically rewritten from C to Rust
//! Source: drivers/iio/frequency/adf4377.c
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
// ADF4377 driver
//
// Copyright 2022 Analog Devices Inc.
//

// ADF4377 REG0000 Map

// ADF4377 REG0000 Bit Definition
pub const ADF4377_0000_SDO_ACTIVE_SPI_3W: c_uint = 0x0;
pub const ADF4377_0000_SDO_ACTIVE_SPI_4W: c_uint = 0x1;
pub const ADF4377_0000_ADDR_ASC_AUTO_DECR: c_uint = 0x0;
pub const ADF4377_0000_ADDR_ASC_AUTO_INCR: c_uint = 0x1;
pub const ADF4377_0000_LSB_FIRST_MSB: c_uint = 0x0;
pub const ADF4377_0000_LSB_FIRST_LSB: c_uint = 0x1;
pub const ADF4377_0000_SOFT_RESET_N_OP: c_uint = 0x0;
pub const ADF4377_0000_SOFT_RESET_EN: c_uint = 0x1;
// ADF4377 REG0001 Map

// ADF4377 REG0003 Bit Definition
pub const ADF4377_0003_CHIP_TYPE: c_uint = 0x06;
// ADF4377 REG0004 Bit Definition
pub const ADF4377_0004_PRODUCT_ID_LSB: c_uint = 0x0005;
// ADF4377 REG0005 Bit Definition
pub const ADF4377_0005_PRODUCT_ID_MSB: c_uint = 0x0005;
// ADF4377 REG000A Map

// ADF4377 REG000C Bit Definition
pub const ADF4377_000C_VENDOR_ID_LSB: c_uint = 0x56;
// ADF4377 REG000D Bit Definition
pub const ADF4377_000D_VENDOR_ID_MSB: c_uint = 0x04;
// ADF4377 REG000F Bit Definition

// ADF4377 REG0010 Map

// ADF4377 REG0011 Map

// ADF4377 REG0011 Bit Definition
pub const ADF4377_0011_DCLK_DIV2_1: c_uint = 0x0;
pub const ADF4377_0011_DCLK_DIV2_2: c_uint = 0x1;
pub const ADF4377_0011_DCLK_DIV2_4: c_uint = 0x2;
pub const ADF4377_0011_DCLK_DIV2_8: c_uint = 0x3;
// ADF4377 REG0012 Map

// ADF4377 REG0012 Bit Definition
pub const ADF4377_0012_CLKOUT_DIV_1: c_uint = 0x0;
pub const ADF4377_0012_CLKOUT_DIV_2: c_uint = 0x1;
pub const ADF4377_0012_CLKOUT_DIV_4: c_uint = 0x2;
pub const ADF4377_0012_CLKOUT_DIV_8: c_uint = 0x3;
// ADF4377 REG0013 Map

// ADF4377 REG0013 Bit Definition
pub const ADF4377_0013_M_VCO_0: c_uint = 0x0;
pub const ADF4377_0013_M_VCO_1: c_uint = 0x1;
pub const ADF4377_0013_M_VCO_2: c_uint = 0x2;
pub const ADF4377_0013_M_VCO_3: c_uint = 0x3;
// ADF4377 REG0014 Map

// ADF4377 REG0015 Map

// ADF4377 REG0015 Bit Definition
pub const ADF4377_CURRENT_SINK: c_uint = 0x0;
pub const ADF4377_CURRENT_SOURCE: c_uint = 0x1;
pub const ADF4377_0015_CP_0MA7: c_uint = 0x0;
pub const ADF4377_0015_CP_0MA9: c_uint = 0x1;
pub const ADF4377_0015_CP_1MA1: c_uint = 0x2;
pub const ADF4377_0015_CP_1MA3: c_uint = 0x3;
pub const ADF4377_0015_CP_1MA4: c_uint = 0x4;
pub const ADF4377_0015_CP_1MA8: c_uint = 0x5;
pub const ADF4377_0015_CP_2MA2: c_uint = 0x6;
pub const ADF4377_0015_CP_2MA5: c_uint = 0x7;
pub const ADF4377_0015_CP_2MA9: c_uint = 0x8;
pub const ADF4377_0015_CP_3MA6: c_uint = 0x9;
pub const ADF4377_0015_CP_4MA3: c_uint = 0xA;
pub const ADF4377_0015_CP_5MA0: c_uint = 0xB;
pub const ADF4377_0015_CP_5MA7: c_uint = 0xC;
pub const ADF4377_0015_CP_7MA2: c_uint = 0xD;
pub const ADF4377_0015_CP_8MA6: c_uint = 0xE;
pub const ADF4377_0015_CP_10MA1: c_uint = 0xF;
// ADF4377 REG0016 Map

// ADF4377 REG0017 Map

// ADF4377 REG0018 Map

// ADF4377 REG0018 Bit Definition
pub const ADF4377_0018_1V8_LOGIC: c_uint = 0x0;
pub const ADF4377_0018_3V3_LOGIC: c_uint = 0x1;
// ADF4377 REG0019 Map

// ADF4377 REG0019 Bit Definition
pub const ADF4377_0019_CLKOUT_320MV: c_uint = 0x0;
pub const ADF4377_0019_CLKOUT_420MV: c_uint = 0x1;
pub const ADF4377_0019_CLKOUT_530MV: c_uint = 0x2;
pub const ADF4377_0019_CLKOUT_640MV: c_uint = 0x3;
// ADF4377 REG001A Map

// ADF4377 REG001B Map

// ADF4377 REG001B Bit Definition
pub const ADF4377_001B_LDWIN_PW_NARROW: c_uint = 0x0;
pub const ADF4377_001B_LDWIN_PW_WIDE: c_uint = 0x1;
// ADF4377 REG001C Map

// ADF4377 REG001C Bit Definition
pub const ADF4377_001C_RST_LD_INACTIVE: c_uint = 0x0;
pub const ADF4377_001C_RST_LD_ACTIVE: c_uint = 0x1;
pub const ADF4377_001C_R01C_RSV1: c_uint = 0x1;
// ADF4377 REG001D Map

pub const ADF4377_001D_EN_CPTEST_OFF: c_uint = 0x0;
pub const ADF4377_001D_EN_CPTEST_ON: c_uint = 0x1;
pub const ADF4377_001D_CP_DOWN_OFF: c_uint = 0x0;
pub const ADF4377_001D_CP_DOWN_ON: c_uint = 0x1;
pub const ADF4377_001D_CP_UP_OFF: c_uint = 0x0;
pub const ADF4377_001D_CP_UP_ON: c_uint = 0x1;
// ADF4377 REG001F Map

// ADF4377 REG001F Bit Definition
pub const ADF4377_001F_BST_LARGE_REF_IN: c_uint = 0x0;
pub const ADF4377_001F_BST_SMALL_REF_IN: c_uint = 0x1;
pub const ADF4377_001F_FILT_REF_OFF: c_uint = 0x0;
pub const ADF4377_001F_FILT_REF_ON: c_uint = 0x1;
pub const ADF4377_001F_REF_SEL_DMA: c_uint = 0x0;
pub const ADF4377_001F_REF_SEL_LNA: c_uint = 0x1;
pub const ADF4377_001F_R01F_RSV1: c_uint = 0x7;
// ADF4377 REG0020 Map

// ADF4377 REG0021 Bit Definition
pub const ADF4377_0021_R021_RSV1: c_uint = 0xD3;
// ADF4377 REG0022 Bit Definition
pub const ADF4377_0022_R022_RSV1: c_uint = 0x32;
// ADF4377 REG0023 Map

// ADF4377 REG0023 Bit Definition
pub const ADF4377_0023_R023_RSV1: c_uint = 0x18;
// ADF4377 REG0024 Map

// ADF4377 REG0025 Map

// ADF4377 REG0025 Bit Definition
pub const ADF4377_0025_R025_RSV1: c_uint = 0x16;
// ADF4377 REG0026 Map

// ADF4377 REG0027 Map

// ADF4377 REG0028 Map

// ADF4377 REG0029 Map

// ADF4377 REG002A Map

// ADF4377 REG002C Map
pub const ADF4377_002C_R02C_RSV1: c_uint = 0xC0;
// ADF4377 REG002D Map

// ADF4377 REG002E Map

// ADF4377 REG002E Bit Definition
pub const ADF4377_002E_ADC_A_CONV_ADC_ST_CNV: c_uint = 0x0;
pub const ADF4377_002E_ADC_A_CONV_VCO_CALIB: c_uint = 0x1;
// ADF4377 REG002F Map

// ADF4377 REG002F Bit Definition
pub const ADF4377_002F_DCLK_DIV1_1: c_uint = 0x0;
pub const ADF4377_002F_DCLK_DIV1_2: c_uint = 0x1;
pub const ADF4377_002F_DCLK_DIV1_8: c_uint = 0x2;
pub const ADF4377_002F_DCLK_DIV1_32: c_uint = 0x3;
// ADF4377 REG0031 Bit Definition
pub const ADF4377_0031_R031_RSV1: c_uint = 0x09;
// ADF4377 REG0032 Map

// ADF4377 REG0032 Bit Definition
pub const ADF4377_0032_ADC_CLK_SEL_N_OP: c_uint = 0x0;
pub const ADF4377_0032_ADC_CLK_SEL_SPI_CLK: c_uint = 0x1;
pub const ADF4377_0032_R032_RSV1: c_uint = 0x9;
// ADF4377 REG0033 Bit Definition
pub const ADF4377_0033_R033_RSV1: c_uint = 0x18;
// ADF4377 REG0034 Bit Definition
pub const ADF4377_0034_R034_RSV1: c_uint = 0x08;
// ADF4377 REG003A Bit Definition
pub const ADF4377_003A_R03A_RSV1: c_uint = 0x5D;
// ADF4377 REG003B Bit Definition
pub const ADF4377_003B_R03B_RSV1: c_uint = 0x2B;
// ADF4377 REG003D Map

// ADF4377 REG003D Bit Definition
pub const ADF4377_003D_O_VCO_BAND_VCO_CALIB: c_uint = 0x0;
pub const ADF4377_003D_O_VCO_BAND_M_VCO: c_uint = 0x1;
pub const ADF4377_003D_O_VCO_CORE_VCO_CALIB: c_uint = 0x0;
pub const ADF4377_003D_O_VCO_CORE_M_VCO: c_uint = 0x1;
pub const ADF4377_003D_O_VCO_BIAS_VCO_CALIB: c_uint = 0x0;
pub const ADF4377_003D_O_VCO_BIAS_M_VCO: c_uint = 0x1;
// ADF4377 REG0042 Map
pub const ADF4377_0042_R042_RSV1: c_uint = 0x05;
// ADF4377 REG0045 Map

// ADF4377 REG0049 Map

// ADF4377 REG004B Map

// ADF4377 REG004C Map

// ADF4377 REG004D Map

// ADF4377 REG004F Map

// ADF4377 REG0051 Map

// ADF4377 REG0054 Map

// Specifications

    enum {
    ADF4377_FREQ,
    };
    enum muxout_select_mode {
    ADF4377_MUXOUT_HIGH_Z = 0x0,
    ADF4377_MUXOUT_LKDET = 0x1,
    ADF4377_MUXOUT_LOW = 0x2,
    ADF4377_MUXOUT_DIV_RCLK_2 = 0x4,
    ADF4377_MUXOUT_DIV_NCLK_2 = 0x5,
    ADF4377_MUXOUT_HIGH = 0x8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4377_chip_info {
    pub name: *const c_char,
    pub has_gpio_enclk2: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4377_state {
    pub chip_info: *const adf4377_chip_info,
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub clkin: *mut clk,
// Protect against concurrent accesses to the device and data content
    pub lock: mutex,
    pub nb: notifier_block,
// Reference Divider
    pub ref_div_factor: c_uint,
// PFD Frequency
    pub f_pfd: c_uint,
// Input Reference Clock
    pub clkin_freq: c_uint,
// CLKOUT Divider
    pub clkout_div_sel: u8,
// Feedback Divider (N)
    pub n_int: u16,
    pub synth_lock_timeout: u16,
    pub vco_alc_timeout: u16,
    pub adc_clk_div: u16,
    pub vco_band_div: u16,
    pub dclk_div1: u8,
    pub dclk_div2: u8,
    pub dclk_mode: u8,
    pub f_div_rclk: c_uint,
    pub muxout_select: enum muxout_select_mode,
    pub gpio_ce: *mut gpio_desc,
    pub gpio_enclk1: *mut gpio_desc,
    pub gpio_enclk2: *mut gpio_desc,
    pub clk: *mut clk,
    pub clkout: *mut clk,
    pub hw: clk_hw,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[2],
}

    static const char * const adf4377_muxout_modes[] = {
    [ADF4377_MUXOUT_HIGH_Z] = "high_z",
    [ADF4377_MUXOUT_LKDET] = "lock_detect",
    [ADF4377_MUXOUT_LOW] = "muxout_low",
    [ADF4377_MUXOUT_DIV_RCLK_2] = "f_div_rclk_2",
    [ADF4377_MUXOUT_DIV_NCLK_2] = "f_div_nclk_2",
    [ADF4377_MUXOUT_HIGH] = "muxout_high",
    };
    static const struct reg_sequence adf4377_reg_defaults[] = {
    { 0x42,  ADF4377_0042_R042_RSV1 },
    { 0x3B,  ADF4377_003B_R03B_RSV1 },
    { 0x3A,  ADF4377_003A_R03A_RSV1 },
    { 0x34,  ADF4377_0034_R034_RSV1 },
    { 0x33,  ADF4377_0033_R033_RSV1 },
    { 0x32,  ADF4377_0032_R032_RSV1 },
    { 0x31,  ADF4377_0031_R031_RSV1 },
    { 0x2C,  ADF4377_002C_R02C_RSV1 },
    { 0x25,  ADF4377_0025_R025_RSV1 },
    { 0x23,  ADF4377_0023_R023_RSV1 },
    { 0x22,  ADF4377_0022_R022_RSV1 },
    { 0x21,  ADF4377_0021_R021_RSV1 },
    { 0x1f,  ADF4377_001F_R01F_RSV1 },
    { 0x1c,  ADF4377_001C_R01C_RSV1 },
    };
    static const struct regmap_config adf4377_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    .max_register = 0x54,
    };
    static int adf4377_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int write_val,
    unsigned int *read_val)
    {
    struct adf4377_state *st = iio_priv(indio_dev);
    if (read_val)
    return regmap_read(st.regmap, reg, read_val);
    return regmap_write(st.regmap, reg, write_val);
    }
    static const struct iio_info adf4377_info = {
    .debugfs_reg_access = &adf4377_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn adf4377_soft_reset(st: *mut adf4377_state) -> c_int {
    static int adf4377_soft_reset(struct adf4377_state *st)
    {
    unsigned int read_val;
    int ret;
    ret = regmap_update_bits(st.regmap, 0x0, ADF4377_0000_SOFT_RESET_MSK |
    ADF4377_0000_SOFT_RESET_R_MSK,
    FIELD_PREP(ADF4377_0000_SOFT_RESET_MSK, 1) |
    FIELD_PREP(ADF4377_0000_SOFT_RESET_R_MSK, 1));
    if (ret)
    return ret;
    return regmap_read_poll_timeout(st.regmap, 0x0, read_val,
    !(read_val & (ADF4377_0000_SOFT_RESET_MSK |
    ADF4377_0000_SOFT_RESET_R_MSK)), 200, 200 * 100);
    }
#[no_mangle]
unsafe extern "C" fn adf4377_get_freq(st: *mut adf4377_state, freq: *mut u64) -> c_int {
    static int adf4377_get_freq(struct adf4377_state *st, u64 *freq)
    {
    unsigned int ref_div_factor, n_int;
    u64 clkin_freq;
    int ret;
    guard(mutex)(&st.lock);
    ret = regmap_read(st.regmap, 0x12, &ref_div_factor);
    if (ret)
    return ret;
    ret = regmap_bulk_read(st.regmap, 0x10, st.buf, sizeof(st.buf));
    if (ret)
    return ret;
    clkin_freq = clk_get_rate(st.clkin);
    ref_div_factor = FIELD_GET(ADF4377_0012_R_DIV_MSK, ref_div_factor);
    n_int = FIELD_GET(ADF4377_0010_N_INT_LSB_MSK | ADF4377_0011_N_INT_MSB_MSK,
    get_unaligned_le16(&st.buf));
// freq = div_u64(clkin_freq, ref_div_factor) * n_int;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf4377_set_freq(st: *mut adf4377_state, freq: u64) -> c_int {
    static int adf4377_set_freq(struct adf4377_state *st, u64 freq)
    {
    unsigned int read_val;
    u64 f_vco;
    int ret;
    guard(mutex)(&st.lock);
    if (freq > ADF4377_MAX_CLKPN_FREQ || freq < ADF4377_MIN_CLKPN_FREQ)
    return -EINVAL;
    ret = regmap_update_bits(st.regmap, 0x1C, ADF4377_001C_EN_DNCLK_MSK |
    ADF4377_001C_EN_DRCLK_MSK,
    FIELD_PREP(ADF4377_001C_EN_DNCLK_MSK, 1) |
    FIELD_PREP(ADF4377_001C_EN_DRCLK_MSK, 1));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x11, ADF4377_0011_EN_AUTOCAL_MSK |
    ADF4377_0011_DCLK_DIV2_MSK,
    FIELD_PREP(ADF4377_0011_EN_AUTOCAL_MSK, 1) |
    FIELD_PREP(ADF4377_0011_DCLK_DIV2_MSK, st.dclk_div2));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x2E, ADF4377_002E_EN_ADC_CNV_MSK |
    ADF4377_002E_EN_ADC_MSK |
    ADF4377_002E_ADC_A_CONV_MSK,
    FIELD_PREP(ADF4377_002E_EN_ADC_CNV_MSK, 1) |
    FIELD_PREP(ADF4377_002E_EN_ADC_MSK, 1) |
    FIELD_PREP(ADF4377_002E_ADC_A_CONV_MSK,
    ADF4377_002E_ADC_A_CONV_VCO_CALIB));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x20, ADF4377_0020_EN_ADC_CLK_MSK,
    FIELD_PREP(ADF4377_0020_EN_ADC_CLK_MSK, 1));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x2F, ADF4377_002F_DCLK_DIV1_MSK,
    FIELD_PREP(ADF4377_002F_DCLK_DIV1_MSK, st.dclk_div1));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x24, ADF4377_0024_DCLK_MODE_MSK,
    FIELD_PREP(ADF4377_0024_DCLK_MODE_MSK, st.dclk_mode));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, 0x27,
    FIELD_PREP(ADF4377_0027_SYNTH_LOCK_TO_LSB_MSK,
    st.synth_lock_timeout));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x28, ADF4377_0028_SYNTH_LOCK_TO_MSB_MSK,
    FIELD_PREP(ADF4377_0028_SYNTH_LOCK_TO_MSB_MSK,
    st.synth_lock_timeout >> 8));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, 0x29,
    FIELD_PREP(ADF4377_0029_VCO_ALC_TO_LSB_MSK,
    st.vco_alc_timeout));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x2A, ADF4377_002A_VCO_ALC_TO_MSB_MSK,
    FIELD_PREP(ADF4377_002A_VCO_ALC_TO_MSB_MSK,
    st.vco_alc_timeout >> 8));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, 0x26,
    FIELD_PREP(ADF4377_0026_VCO_BAND_DIV_MSK, st.vco_band_div));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, 0x2D,
    FIELD_PREP(ADF4377_002D_ADC_CLK_DIV_MSK, st.adc_clk_div));
    if (ret)
    return ret;
    st.clkout_div_sel = 0;
    f_vco = freq;
    while (f_vco < ADF4377_MIN_VCO_FREQ) {
    f_vco <<= 1;
    st.clkout_div_sel++;
    }
    st.n_int = div_u64(freq, st.f_pfd);
    ret = regmap_update_bits(st.regmap, 0x11, ADF4377_0011_EN_RDBLR_MSK |
    ADF4377_0011_N_INT_MSB_MSK,
    FIELD_PREP(ADF4377_0011_EN_RDBLR_MSK, 0) |
    FIELD_PREP(ADF4377_0011_N_INT_MSB_MSK, st.n_int >> 8));
    if (ret)
    return ret;
    ret = regmap_update_bits(st.regmap, 0x12, ADF4377_0012_R_DIV_MSK |
    ADF4377_0012_CLKOUT_DIV_MSK,
    FIELD_PREP(ADF4377_0012_CLKOUT_DIV_MSK, st.clkout_div_sel) |
    FIELD_PREP(ADF4377_0012_R_DIV_MSK, st.ref_div_factor));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, 0x10,
    FIELD_PREP(ADF4377_0010_N_INT_LSB_MSK, st.n_int));
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(st.regmap, 0x49, read_val,
    !(read_val & (ADF4377_0049_FSM_BUSY_MSK)), 200, 200 * 100);
    if (ret)
    return ret;
// Disable EN_DNCLK, EN_DRCLK
    ret = regmap_update_bits(st.regmap, 0x1C, ADF4377_001C_EN_DNCLK_MSK |
    ADF4377_001C_EN_DRCLK_MSK,
    FIELD_PREP(ADF4377_001C_EN_DNCLK_MSK, 0) |
    FIELD_PREP(ADF4377_001C_EN_DRCLK_MSK, 0));
    if (ret)
    return ret;
// Disable EN_ADC_CLK
    ret = regmap_update_bits(st.regmap, 0x20, ADF4377_0020_EN_ADC_CLK_MSK,
    FIELD_PREP(ADF4377_0020_EN_ADC_CLK_MSK, 0));
    if (ret)
    return ret;
// Set output Amplitude
    return regmap_update_bits(st.regmap, 0x19, ADF4377_0019_CLKOUT2_OP_MSK |
    ADF4377_0019_CLKOUT1_OP_MSK,
    FIELD_PREP(ADF4377_0019_CLKOUT1_OP_MSK,
    ADF4377_0019_CLKOUT_420MV) |
    FIELD_PREP(ADF4377_0019_CLKOUT2_OP_MSK,
    ADF4377_0019_CLKOUT_420MV));
    }
#[no_mangle]
unsafe extern "C" fn adf4377_gpio_init(st: *mut adf4377_state) {
    static void adf4377_gpio_init(struct adf4377_state *st)
    {
    if (st.gpio_ce) {
    gpiod_set_value(st.gpio_ce, 1);
// Delay for SPI register bits to settle to their power-on reset state
    fsleep(200);
    }
    if (st.gpio_enclk1)
    gpiod_set_value(st.gpio_enclk1, 1);
    if (st.gpio_enclk2)
    gpiod_set_value(st.gpio_enclk2, 1);
    }
#[no_mangle]
unsafe extern "C" fn adf4377_init(st: *mut adf4377_state) -> c_int {
    static int adf4377_init(struct adf4377_state *st)
    {
    struct device *dev = &st.spi.dev;
    int ret;
    adf4377_gpio_init(st);
    ret = adf4377_soft_reset(st);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to soft reset.\n");
    ret = regmap_multi_reg_write(st.regmap, adf4377_reg_defaults,
    ARRAY_SIZE(adf4377_reg_defaults));
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to set default registers.\n");
    ret = regmap_update_bits(st.regmap, 0x00,
    ADF4377_0000_SDO_ACTIVE_MSK | ADF4377_0000_SDO_ACTIVE_R_MSK,
    FIELD_PREP(ADF4377_0000_SDO_ACTIVE_MSK,
    ADF4377_0000_SDO_ACTIVE_SPI_4W) |
    FIELD_PREP(ADF4377_0000_SDO_ACTIVE_R_MSK,
    ADF4377_0000_SDO_ACTIVE_SPI_4W));
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to set 4-Wire Operation.\n");
    st.clkin_freq = clk_get_rate(st.clkin);
// Power Up
    ret = regmap_write(st.regmap, 0x1a,
    FIELD_PREP(ADF4377_001A_PD_ALL_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_RDIV_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_NDIV_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_VCO_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_LD_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_PFDCP_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_CLKOUT1_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_CLKOUT2_MSK, 0));
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to set power down registers.\n");
// Set Mux Output
    ret = regmap_update_bits(st.regmap, 0x1D,
    ADF4377_001D_MUXOUT_MSK,
    FIELD_PREP(ADF4377_001D_MUXOUT_MSK, st.muxout_select));
    if (ret)
    return ret;
// Compute PFD
    st.ref_div_factor = 0;
    do {
    st.ref_div_factor++;
    st.f_pfd = st.clkin_freq / st.ref_div_factor;
    } while (st.f_pfd > ADF4377_MAX_FREQ_PFD);
    if (st.f_pfd > ADF4377_MAX_FREQ_PFD || st.f_pfd < ADF4377_MIN_FREQ_PFD)
    return -EINVAL;
    st.f_div_rclk = st.f_pfd;
    if (st.f_pfd <= ADF4377_FREQ_PFD_80MHZ) {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_1;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_1;
    st.dclk_mode = 0;
    } else if (st.f_pfd <= ADF4377_FREQ_PFD_125MHZ) {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_1;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_1;
    st.dclk_mode = 1;
    } else if (st.f_pfd <= ADF4377_FREQ_PFD_160MHZ) {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_2;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_1;
    st.dclk_mode = 0;
    st.f_div_rclk /= 2;
    } else if (st.f_pfd <= ADF4377_FREQ_PFD_250MHZ) {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_2;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_1;
    st.dclk_mode = 1;
    st.f_div_rclk /= 2;
    } else if (st.f_pfd <= ADF4377_FREQ_PFD_320MHZ) {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_2;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_2;
    st.dclk_mode = 0;
    st.f_div_rclk /= 4;
    } else {
    st.dclk_div1 = ADF4377_002F_DCLK_DIV1_2;
    st.dclk_div2 = ADF4377_0011_DCLK_DIV2_2;
    st.dclk_mode = 1;
    st.f_div_rclk /= 4;
    }
    st.synth_lock_timeout = DIV_ROUND_UP(st.f_div_rclk, 50000);
    st.vco_alc_timeout = DIV_ROUND_UP(st.f_div_rclk, 20000);
    st.vco_band_div = DIV_ROUND_UP(st.f_div_rclk, 150000 * 16 * (1 << st.dclk_mode));
    st.adc_clk_div = DIV_ROUND_UP((st.f_div_rclk / 400000 - 2), 4);
    return 0;
    }
    static ssize_t adf4377_read(struct iio_dev *indio_dev, uintptr_t private,
    const struct iio_chan_spec *chan, char *buf)
    {
    struct adf4377_state *st = iio_priv(indio_dev);
    let mut val: u64 = 0;
    int ret;
    switch ((u32)private) {
    case ADF4377_FREQ:
    ret = adf4377_get_freq(st, &val);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%llu\n", val);
    default:
    return -EINVAL;
    }
    }
    static ssize_t adf4377_write(struct iio_dev *indio_dev, uintptr_t private,
    const struct iio_chan_spec *chan, const char *buf,
    size_t len)
    {
    struct adf4377_state *st = iio_priv(indio_dev);
    unsigned long long freq;
    int ret;
    switch ((u32)private) {
    case ADF4377_FREQ:
    ret = kstrtoull(buf, 10, &freq);
    if (ret)
    return ret;
    ret = adf4377_set_freq(st, freq);
    if (ret)
    return ret;
    return len;
    default:
    return -EINVAL;
    }
    }

    .name = _name, \
    .read = adf4377_read, \
    .write = adf4377_write, \
    .private = _ident, \
    .shared = _shared, \
    }
    static const struct iio_chan_spec_ext_info adf4377_ext_info[] = {
//
// Usually we use IIO_CHAN_INFO_FREQUENCY, but there are
// values > 2^32 in order to support the entire frequency range
// in Hz.
//
    _ADF4377_EXT_INFO("frequency", IIO_SEPARATE, ADF4377_FREQ),
    { }
    };
    static const struct iio_chan_spec adf4377_channels[] = {
    {
    .type = IIO_ALTVOLTAGE,
    .indexed = 1,
    .output = 1,
    .channel = 0,
    .ext_info = adf4377_ext_info,
    },
    };
#[no_mangle]
unsafe extern "C" fn adf4377_properties_parse(st: *mut adf4377_state) -> c_int {
    static int adf4377_properties_parse(struct adf4377_state *st)
    {
    struct device *dev = &st.spi.dev;
    int ret;
    st.clkin = devm_clk_get_enabled(dev, "ref_in");
    if (IS_ERR(st.clkin))
    return dev_err_probe(dev, PTR_ERR(st.clkin),
    "failed to get the reference input clock\n");
    st.gpio_ce = devm_gpiod_get_optional(dev, "chip-enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(st.gpio_ce))
    return dev_err_probe(dev, PTR_ERR(st.gpio_ce),
    "failed to get the CE GPIO\n");
    st.gpio_enclk1 = devm_gpiod_get_optional(dev, "clk1-enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(st.gpio_enclk1))
    return dev_err_probe(dev, PTR_ERR(st.gpio_enclk1),
    "failed to get the CE GPIO\n");
    if (st.chip_info.has_gpio_enclk2) {
    st.gpio_enclk2 = devm_gpiod_get_optional(dev, "clk2-enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(st.gpio_enclk2))
    return dev_err_probe(dev, PTR_ERR(st.gpio_enclk2),
    "failed to get the CE GPIO\n");
    }
    ret = device_property_match_property_string(dev, "adi,muxout-select",
    adf4377_muxout_modes,
    ARRAY_SIZE(adf4377_muxout_modes));
    if (ret >= 0)
    st.muxout_select = ret;
    else
    st.muxout_select = ADF4377_MUXOUT_HIGH_Z;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf4377_freq_change(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    static int adf4377_freq_change(struct notifier_block *nb, unsigned long action, void *data)
    {
    struct adf4377_state *st = container_of(nb, struct adf4377_state, nb);
    if (action == POST_RATE_CHANGE) {
    guard(mutex)(&st.lock);
    return notifier_from_errno(adf4377_init(st));
    }
    return NOTIFY_OK;
    }
    static unsigned long adf4377_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct adf4377_state *st = to_adf4377_state(hw);
    u64 freq;
    int ret;
    ret = adf4377_get_freq(st, &freq);
    if (ret)
    return 0;
    return freq;
    }
    static int adf4377_clk_set_rate(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct adf4377_state *st = to_adf4377_state(hw);
    return adf4377_set_freq(st, rate);
    }
#[no_mangle]
unsafe extern "C" fn adf4377_clk_prepare(hw: *mut clk_hw) -> c_int {
    static int adf4377_clk_prepare(struct clk_hw *hw)
    {
    struct adf4377_state *st = to_adf4377_state(hw);
    return regmap_update_bits(st.regmap, 0x1a, ADF4377_001A_PD_CLKOUT1_MSK |
    ADF4377_001A_PD_CLKOUT2_MSK,
    FIELD_PREP(ADF4377_001A_PD_CLKOUT1_MSK, 0) |
    FIELD_PREP(ADF4377_001A_PD_CLKOUT2_MSK, 0));
    }
#[no_mangle]
unsafe extern "C" fn adf4377_clk_unprepare(hw: *mut clk_hw) {
    static void adf4377_clk_unprepare(struct clk_hw *hw)
    {
    struct adf4377_state *st = to_adf4377_state(hw);
    regmap_update_bits(st.regmap, 0x1a, ADF4377_001A_PD_CLKOUT1_MSK |
    ADF4377_001A_PD_CLKOUT2_MSK,
    FIELD_PREP(ADF4377_001A_PD_CLKOUT1_MSK, 1) |
    FIELD_PREP(ADF4377_001A_PD_CLKOUT2_MSK, 1));
    }
#[no_mangle]
unsafe extern "C" fn adf4377_clk_is_prepared(hw: *mut clk_hw) -> c_int {
    static int adf4377_clk_is_prepared(struct clk_hw *hw)
    {
    struct adf4377_state *st = to_adf4377_state(hw);
    unsigned int readval;
    int ret;
    ret = regmap_read(st.regmap, 0x1a, &readval);
    if (ret)
    return ret;
    return !(readval & (ADF4377_001A_PD_CLKOUT1_MSK | ADF4377_001A_PD_CLKOUT2_MSK));
    }
    static const struct clk_ops adf4377_clk_ops = {
    .recalc_rate = adf4377_clk_recalc_rate,
    .set_rate = adf4377_clk_set_rate,
    .prepare = adf4377_clk_prepare,
    .unprepare = adf4377_clk_unprepare,
    .is_prepared = adf4377_clk_is_prepared,
    };
#[no_mangle]
unsafe extern "C" fn adf4377_clk_register(st: *mut adf4377_state) -> c_int {
    static int adf4377_clk_register(struct adf4377_state *st)
    {
    struct spi_device *spi = st.spi;
    struct device *dev = &spi.dev;
    struct clk_init_data init;
    struct clk_parent_data parent_data;
    int ret;
    if (!device_property_present(dev, "#clock-cells"))
    return 0;
    ret = device_property_read_string(dev, "clock-output-names", &init.name);
    if (ret) {
    init.name = devm_kasprintf(dev, GFP_KERNEL, "%pfw-clk",
    dev_fwnode(dev));
    if (!init.name)
    return -ENOMEM;
    }
    parent_data.fw_name = "ref_in";
    init.ops = &adf4377_clk_ops;
    init.parent_data = &parent_data;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_PARENT;
    st.hw.init = &init;
    ret = devm_clk_hw_register(dev, &st.hw);
    if (ret)
    return ret;
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &st.hw);
    if (ret)
    return ret;
    st.clkout = st.hw.clk;
    return 0;
    }
    static const struct adf4377_chip_info adf4377_chip_info = {
    .name = "adf4377",
    .has_gpio_enclk2 = true,
    };
    static const struct adf4377_chip_info adf4378_chip_info = {
    .name = "adf4378",
    .has_gpio_enclk2 = false,
    };
#[no_mangle]
unsafe extern "C" fn adf4377_probe(spi: *mut spi_device) -> c_int {
    static int adf4377_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct regmap *regmap;
    struct adf4377_state *st;
    struct device *dev = &spi.dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    regmap = devm_regmap_init_spi(spi, &adf4377_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    st = iio_priv(indio_dev);
    indio_dev.info = &adf4377_info;
    indio_dev.name = "adf4377";
    st.regmap = regmap;
    st.spi = spi;
    st.chip_info = spi_get_device_match_data(spi);
    mutex_init(&st.lock);
    ret = adf4377_properties_parse(st);
    if (ret)
    return ret;
    st.nb.notifier_call = adf4377_freq_change;
    ret = devm_clk_notifier_register(dev, st.clkin, &st.nb);
    if (ret)
    return ret;
    ret = adf4377_init(st);
    if (ret)
    return ret;
    ret = adf4377_clk_register(st);
    if (ret)
    return ret;
    if (!st.clkout) {
    indio_dev.channels = adf4377_channels;
    indio_dev.num_channels = ARRAY_SIZE(adf4377_channels);
    }
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct spi_device_id adf4377_id[] = {
    { .name = "adf4377", .driver_data = (kernel_ulong_t)&adf4377_chip_info },
    { .name = "adf4378", .driver_data = (kernel_ulong_t)&adf4378_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adf4377_id);
    static const struct of_device_id adf4377_of_match[] = {
    { .compatible = "adi,adf4377", .data = &adf4377_chip_info },
    { .compatible = "adi,adf4378", .data = &adf4378_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, adf4377_of_match);
    static struct spi_driver adf4377_driver = {
    .driver = {
    .name = "adf4377",
    .of_match_table = adf4377_of_match,
    },
    .probe = adf4377_probe,
    .id_table = adf4377_id,
    };
    module_spi_driver(adf4377_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADF4377");
    MODULE_LICENSE("GPL");
