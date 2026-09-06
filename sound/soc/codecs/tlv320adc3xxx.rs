//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320adc3xxx.c
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
// Based on sound/soc/codecs/tlv320aic3x.c by  Vladimir Barinov
//
// Copyright (C) 2010 Mistral Solutions Pvt Ltd.
// Author: Shahina Shaik <shahina.s@mistralsolutions.com>
//
// Copyright (C) 2014-2018, Ambarella, Inc.
// Author: Dongge wu <dgwu@ambarella.com>
//
// Copyright (C) 2021 Axis Communications AB
// Author: Ricard Wanderlof <ricardw@axis.com>
//

//
// General definitions defining exported functionality.
//
pub const ADC3XXX_MICBIAS_PINS: c_int = 2;
pub const ADC3XXX_GPIO_PINS: c_int = 2;
// Number of GPIO pins exposed via the gpiolib interface

    SNDRV_PCM_FMTBIT_S20_3LE | \
    SNDRV_PCM_FMTBIT_S24_3LE | \
    SNDRV_PCM_FMTBIT_S32_LE)
//
// PLL modes, to be used for clk_id for set_sysclk callback.
//
// The default behavior (AUTO) is to take the first matching entry in the clock
// table, which is intended to be the PLL based one if there is more than one.
//
// Setting the clock source using simple-card (clocks or
// system-clock-frequency property) sets clk_id = 0 = ADC3XXX_PLL_AUTO.
//

// Register definitions.
pub const ADC3XXX_PAGE_SIZE: c_int = 128;

//
// Page 0 registers.
//

// 2-3 Reserved

// 9-17 Reserved

// 23-24 Reserved

// 35 Reserved

// 39-41 Reserved

// 44 Reserved

// 46 Reserved

// 50 Reserved

// 54-56 Reserved

// 60 Reserved

// 63-79 Reserved

// 102-127 Reserved
//
// Page 1 registers.
//
// 1-25 Reserved

// 27-50 Reserved

// 53 Reserved

// 63-127 Reserved
//
// Page 4 registers. First page of coefficient memory for the miniDSP.
//

//
// Register bits.
//
// PLL Enable bits
pub const ADC3XXX_ENABLE_PLL_SHIFT: c_int = 7;

pub const ADC3XXX_ENABLE_NADC_SHIFT: c_int = 7;

pub const ADC3XXX_ENABLE_MADC_SHIFT: c_int = 7;

pub const ADC3XXX_ENABLE_BCLK_SHIFT: c_int = 7;

// Power bits
pub const ADC3XXX_LADC_PWR_ON: c_uint = 0x80;
pub const ADC3XXX_RADC_PWR_ON: c_uint = 0x40;
pub const ADC3XXX_SOFT_RESET: c_uint = 0x01;
pub const ADC3XXX_BCLK_MASTER: c_uint = 0x08;
pub const ADC3XXX_WCLK_MASTER: c_uint = 0x04;
// Interface register masks
pub const ADC3XXX_FORMAT_MASK: c_uint = 0xc0;
pub const ADC3XXX_FORMAT_SHIFT: c_int = 6;
pub const ADC3XXX_WLENGTH_MASK: c_uint = 0x30;
pub const ADC3XXX_WLENGTH_SHIFT: c_int = 4;
pub const ADC3XXX_CLKDIR_MASK: c_uint = 0x0c;
pub const ADC3XXX_CLKDIR_SHIFT: c_int = 2;
// Interface register bit patterns

// PLL P/R bit offsets
pub const ADC3XXX_PLLP_SHIFT: c_int = 4;
pub const ADC3XXX_PLLR_SHIFT: c_int = 0;
pub const ADC3XXX_PLL_PR_MASK: c_uint = 0x7f;
pub const ADC3XXX_PLLJ_MASK: c_uint = 0x3f;
pub const ADC3XXX_PLLD_MSB_MASK: c_uint = 0x3f;
pub const ADC3XXX_PLLD_LSB_MASK: c_uint = 0xff;
pub const ADC3XXX_NADC_MASK: c_uint = 0x7f;
pub const ADC3XXX_MADC_MASK: c_uint = 0x7f;
pub const ADC3XXX_AOSR_MASK: c_uint = 0xff;
pub const ADC3XXX_IADC_MASK: c_uint = 0xff;
pub const ADC3XXX_BDIV_MASK: c_uint = 0x7f;
// PLL_CLKIN bits
pub const ADC3XXX_PLL_CLKIN_SHIFT: c_int = 2;
pub const ADC3XXX_PLL_CLKIN_MCLK: c_uint = 0x0;
pub const ADC3XXX_PLL_CLKIN_BCLK: c_uint = 0x1;
pub const ADC3XXX_PLL_CLKIN_ZERO: c_uint = 0x3;
// CODEC_CLKIN bits
pub const ADC3XXX_CODEC_CLKIN_SHIFT: c_int = 0;
pub const ADC3XXX_CODEC_CLKIN_MCLK: c_uint = 0x0;
pub const ADC3XXX_CODEC_CLKIN_BCLK: c_uint = 0x1;
pub const ADC3XXX_CODEC_CLKIN_PLL_CLK: c_uint = 0x3;

    (ADC3XXX_CODEC_CLKIN_PLL_CLK << ADC3XXX_CODEC_CLKIN_SHIFT))

    (ADC3XXX_CODEC_CLKIN_MCLK << ADC3XXX_CODEC_CLKIN_SHIFT))
// Analog PGA control bits
pub const ADC3XXX_LPGA_MUTE: c_uint = 0x80;
pub const ADC3XXX_RPGA_MUTE: c_uint = 0x80;
pub const ADC3XXX_LPGA_GAIN_MASK: c_uint = 0x7f;
pub const ADC3XXX_RPGA_GAIN_MASK: c_uint = 0x7f;
// ADC current modes
pub const ADC3XXX_ADC_LOW_CURR_MODE: c_uint = 0x01;
// Left ADC Input selection bits
pub const ADC3XXX_LCH_SEL1_SHIFT: c_int = 0;
pub const ADC3XXX_LCH_SEL2_SHIFT: c_int = 2;
pub const ADC3XXX_LCH_SEL3_SHIFT: c_int = 4;
pub const ADC3XXX_LCH_SEL4_SHIFT: c_int = 6;
pub const ADC3XXX_LCH_SEL1X_SHIFT: c_int = 0;
pub const ADC3XXX_LCH_SEL2X_SHIFT: c_int = 2;
pub const ADC3XXX_LCH_SEL3X_SHIFT: c_int = 4;
pub const ADC3XXX_LCH_COMMON_MODE: c_uint = 0x40;
pub const ADC3XXX_BYPASS_LPGA: c_uint = 0x80;
// Right ADC Input selection bits
pub const ADC3XXX_RCH_SEL1_SHIFT: c_int = 0;
pub const ADC3XXX_RCH_SEL2_SHIFT: c_int = 2;
pub const ADC3XXX_RCH_SEL3_SHIFT: c_int = 4;
pub const ADC3XXX_RCH_SEL4_SHIFT: c_int = 6;
pub const ADC3XXX_RCH_SEL1X_SHIFT: c_int = 0;
pub const ADC3XXX_RCH_SEL2X_SHIFT: c_int = 2;
pub const ADC3XXX_RCH_SEL3X_SHIFT: c_int = 4;
pub const ADC3XXX_RCH_COMMON_MODE: c_uint = 0x40;
pub const ADC3XXX_BYPASS_RPGA: c_uint = 0x80;
// MICBIAS control bits
pub const ADC3XXX_MICBIAS_MASK: c_uint = 0x3;
pub const ADC3XXX_MICBIAS1_SHIFT: c_int = 5;
pub const ADC3XXX_MICBIAS2_SHIFT: c_int = 3;
pub const ADC3XXX_ADC_MAX_VOLUME: c_int = 64;
pub const ADC3XXX_ADC_POS_VOL: c_int = 24;
// GPIO control bits (GPIO1_CTRL and GPIO2_CTRL)
pub const ADC3XXX_GPIO_CTRL_CFG_MASK: c_uint = 0x3c;
pub const ADC3XXX_GPIO_CTRL_CFG_SHIFT: c_int = 2;
pub const ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK: c_uint = 0x01;
pub const ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_SHIFT: c_int = 0;
pub const ADC3XXX_GPIO_CTRL_INPUT_VALUE_MASK: c_uint = 0x02;
pub const ADC3XXX_GPIO_CTRL_INPUT_VALUE_SHIFT: c_int = 1;
    enum adc3xxx_type {
    ADC3001 = 0,
    ADC3101
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc3xxx {
    pub dev: *mut device,
    pub type: enum adc3xxx_type,
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub rst_pin: *mut gpio_desc,
    pub pll_mode: c_uint,
    pub sysclk: c_uint,
    pub /: *mut *mut unsigned int gpio_cfg[ADC3XXX_GPIO_PINS]; / value+1 (0 => not set),
    pub /: *mut *mut unsigned int micbias_gpo[ADC3XXX_MICBIAS_PINS]; / 1 => pin is GPO,
    pub micbias_vg: [c_uint; ADC3XXX_MICBIAS_PINS],
    pub master: c_int,
    pub page_no: u8,
    pub use_pll: c_int,
    pub gpio_chip: gpio_chip,
}

    static const unsigned int adc3xxx_gpio_ctrl_reg[ADC3XXX_GPIO_PINS] = {
    ADC3XXX_GPIO1_CTRL,
    ADC3XXX_GPIO2_CTRL
    };
    static const unsigned int adc3xxx_micbias_shift[ADC3XXX_MICBIAS_PINS] = {
    ADC3XXX_MICBIAS1_SHIFT,
    ADC3XXX_MICBIAS2_SHIFT
    };
    static const struct reg_default adc3xxx_defaults[] = {
// Page 0
    { 0, 0x00 },    { 1, 0x00 },    { 2, 0x00 },    { 3, 0x00 },
    { 4, 0x00 },    { 5, 0x11 },    { 6, 0x04 },    { 7, 0x00 },
    { 8, 0x00 },    { 9, 0x00 },    { 10, 0x00 },   { 11, 0x00 },
    { 12, 0x00 },   { 13, 0x00 },   { 14, 0x00 },   { 15, 0x00 },
    { 16, 0x00 },   { 17, 0x00 },   { 18, 0x01 },   { 19, 0x01 },
    { 20, 0x80 },   { 21, 0x80 },   { 22, 0x04 },   { 23, 0x00 },
    { 24, 0x00 },   { 25, 0x00 },   { 26, 0x01 },   { 27, 0x00 },
    { 28, 0x00 },   { 29, 0x02 },   { 30, 0x01 },   { 31, 0x00 },
    { 32, 0x00 },   { 33, 0x10 },   { 34, 0x00 },   { 35, 0x00 },
    { 36, 0x00 },   { 37, 0x00 },   { 38, 0x02 },   { 39, 0x00 },
    { 40, 0x00 },   { 41, 0x00 },   { 42, 0x00 },   { 43, 0x00 },
    { 44, 0x00 },   { 45, 0x00 },   { 46, 0x00 },   { 47, 0x00 },
    { 48, 0x00 },   { 49, 0x00 },   { 50, 0x00 },   { 51, 0x00 },
    { 52, 0x00 },   { 53, 0x12 },   { 54, 0x00 },   { 55, 0x00 },
    { 56, 0x00 },   { 57, 0x00 },   { 58, 0x00 },   { 59, 0x44 },
    { 60, 0x00 },   { 61, 0x01 },   { 62, 0x00 },   { 63, 0x00 },
    { 64, 0x00 },   { 65, 0x00 },   { 66, 0x00 },   { 67, 0x00 },
    { 68, 0x00 },   { 69, 0x00 },   { 70, 0x00 },   { 71, 0x00 },
    { 72, 0x00 },   { 73, 0x00 },   { 74, 0x00 },   { 75, 0x00 },
    { 76, 0x00 },   { 77, 0x00 },   { 78, 0x00 },   { 79, 0x00 },
    { 80, 0x00 },   { 81, 0x00 },   { 82, 0x88 },   { 83, 0x00 },
    { 84, 0x00 },   { 85, 0x00 },   { 86, 0x00 },   { 87, 0x00 },
    { 88, 0x7f },   { 89, 0x00 },   { 90, 0x00 },   { 91, 0x00 },
    { 92, 0x00 },   { 93, 0x00 },   { 94, 0x00 },   { 95, 0x00 },
    { 96, 0x7f },   { 97, 0x00 },   { 98, 0x00 },   { 99, 0x00 },
    { 100, 0x00 },  { 101, 0x00 },  { 102, 0x00 },  { 103, 0x00 },
    { 104, 0x00 },  { 105, 0x00 },  { 106, 0x00 },  { 107, 0x00 },
    { 108, 0x00 },  { 109, 0x00 },  { 110, 0x00 },  { 111, 0x00 },
    { 112, 0x00 },  { 113, 0x00 },  { 114, 0x00 },  { 115, 0x00 },
    { 116, 0x00 },  { 117, 0x00 },  { 118, 0x00 },  { 119, 0x00 },
    { 120, 0x00 },  { 121, 0x00 },  { 122, 0x00 },  { 123, 0x00 },
    { 124, 0x00 },  { 125, 0x00 },  { 126, 0x00 },  { 127, 0x00 },
// Page 1
    { 128, 0x00 },  { 129, 0x00 },  { 130, 0x00 },  { 131, 0x00 },
    { 132, 0x00 },  { 133, 0x00 },  { 134, 0x00 },  { 135, 0x00 },
    { 136, 0x00 },  { 137, 0x00 },  { 138, 0x00 },  { 139, 0x00 },
    { 140, 0x00 },  { 141, 0x00 },  { 142, 0x00 },  { 143, 0x00 },
    { 144, 0x00 },  { 145, 0x00 },  { 146, 0x00 },  { 147, 0x00 },
    { 148, 0x00 },  { 149, 0x00 },  { 150, 0x00 },  { 151, 0x00 },
    { 152, 0x00 },  { 153, 0x00 },  { 154, 0x00 },  { 155, 0x00 },
    { 156, 0x00 },  { 157, 0x00 },  { 158, 0x00 },  { 159, 0x00 },
    { 160, 0x00 },  { 161, 0x00 },  { 162, 0x00 },  { 163, 0x00 },
    { 164, 0x00 },  { 165, 0x00 },  { 166, 0x00 },  { 167, 0x00 },
    { 168, 0x00 },  { 169, 0x00 },  { 170, 0x00 },  { 171, 0x00 },
    { 172, 0x00 },  { 173, 0x00 },  { 174, 0x00 },  { 175, 0x00 },
    { 176, 0x00 },  { 177, 0x00 },  { 178, 0x00 },  { 179, 0x00 },
    { 180, 0xff },  { 181, 0x00 },  { 182, 0x3f },  { 183, 0xff },
    { 184, 0x00 },  { 185, 0x3f },  { 186, 0x00 },  { 187, 0x80 },
    { 188, 0x80 },  { 189, 0x00 },  { 190, 0x00 },  { 191, 0x00 },
// Page 4
    { 1024, 0x00 },			{ 1026, 0x01 },	{ 1027, 0x17 },
    { 1028, 0x01 }, { 1029, 0x17 }, { 1030, 0x7d }, { 1031, 0xd3 },
    { 1032, 0x7f }, { 1033, 0xff }, { 1034, 0x00 }, { 1035, 0x00 },
    { 1036, 0x00 }, { 1037, 0x00 }, { 1038, 0x7f }, { 1039, 0xff },
    { 1040, 0x00 }, { 1041, 0x00 }, { 1042, 0x00 }, { 1043, 0x00 },
    { 1044, 0x00 }, { 1045, 0x00 }, { 1046, 0x00 }, { 1047, 0x00 },
    { 1048, 0x7f }, { 1049, 0xff }, { 1050, 0x00 }, { 1051, 0x00 },
    { 1052, 0x00 }, { 1053, 0x00 }, { 1054, 0x00 }, { 1055, 0x00 },
    { 1056, 0x00 }, { 1057, 0x00 }, { 1058, 0x7f }, { 1059, 0xff },
    { 1060, 0x00 }, { 1061, 0x00 }, { 1062, 0x00 }, { 1063, 0x00 },
    { 1064, 0x00 }, { 1065, 0x00 }, { 1066, 0x00 }, { 1067, 0x00 },
    { 1068, 0x7f }, { 1069, 0xff }, { 1070, 0x00 }, { 1071, 0x00 },
    { 1072, 0x00 }, { 1073, 0x00 }, { 1074, 0x00 }, { 1075, 0x00 },
    { 1076, 0x00 }, { 1077, 0x00 }, { 1078, 0x7f }, { 1079, 0xff },
    { 1080, 0x00 }, { 1081, 0x00 }, { 1082, 0x00 }, { 1083, 0x00 },
    { 1084, 0x00 }, { 1085, 0x00 }, { 1086, 0x00 }, { 1087, 0x00 },
    { 1088, 0x00 }, { 1089, 0x00 }, { 1090, 0x00 }, { 1091, 0x00 },
    { 1092, 0x00 }, { 1093, 0x00 }, { 1094, 0x00 }, { 1095, 0x00 },
    { 1096, 0x00 }, { 1097, 0x00 }, { 1098, 0x00 }, { 1099, 0x00 },
    { 1100, 0x00 }, { 1101, 0x00 }, { 1102, 0x00 }, { 1103, 0x00 },
    { 1104, 0x00 }, { 1105, 0x00 }, { 1106, 0x00 }, { 1107, 0x00 },
    { 1108, 0x00 }, { 1109, 0x00 }, { 1110, 0x00 }, { 1111, 0x00 },
    { 1112, 0x00 }, { 1113, 0x00 }, { 1114, 0x00 }, { 1115, 0x00 },
    { 1116, 0x00 }, { 1117, 0x00 }, { 1118, 0x00 }, { 1119, 0x00 },
    { 1120, 0x00 }, { 1121, 0x00 }, { 1122, 0x00 }, { 1123, 0x00 },
    { 1124, 0x00 }, { 1125, 0x00 }, { 1126, 0x00 }, { 1127, 0x00 },
    { 1128, 0x00 }, { 1129, 0x00 }, { 1130, 0x00 }, { 1131, 0x00 },
    { 1132, 0x00 }, { 1133, 0x00 }, { 1134, 0x00 }, { 1135, 0x00 },
    { 1136, 0x00 }, { 1137, 0x00 }, { 1138, 0x00 }, { 1139, 0x00 },
    { 1140, 0x00 }, { 1141, 0x00 }, { 1142, 0x00 }, { 1143, 0x00 },
    { 1144, 0x00 }, { 1145, 0x00 }, { 1146, 0x00 }, { 1147, 0x00 },
    { 1148, 0x00 }, { 1149, 0x00 }, { 1150, 0x00 }, { 1151, 0x00 },
    };
#[no_mangle]
unsafe extern "C" fn adc3xxx_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool adc3xxx_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ADC3XXX_RESET:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_range_cfg adc3xxx_ranges[] = {
    {
    .range_min = 0,
    .range_max = 5 * ADC3XXX_PAGE_SIZE,
    .selector_reg = ADC3XXX_PAGE_SELECT,
    .selector_mask = 0xff,
    .selector_shift = 0,
    .window_start = 0,
    .window_len = ADC3XXX_PAGE_SIZE,
    }
    };
    static const struct regmap_config adc3xxx_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .reg_defaults = adc3xxx_defaults,
    .num_reg_defaults = ARRAY_SIZE(adc3xxx_defaults),
    .volatile_reg = adc3xxx_volatile_reg,
    .cache_type = REGCACHE_RBTREE,
    .ranges = adc3xxx_ranges,
    .num_ranges = ARRAY_SIZE(adc3xxx_ranges),
    .max_register = 5 * ADC3XXX_PAGE_SIZE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc3xxx_rate_divs {
    pub mclk: u32,
    pub rate: u32,
    pub pll_p: u8,
    pub pll_r: u8,
    pub pll_j: u8,
    pub pll_d: u16,
    pub nadc: u8,
    pub madc: u8,
    pub aosr: u8,
}

//
// PLL and Clock settings.
// If p member is 0, PLL is not used.
// The order of the entries in this table have the PLL entries before
// the non-PLL entries, so that the PLL modes are preferred unless
// the PLL mode setting says otherwise.
//
    static const struct adc3xxx_rate_divs adc3xxx_divs[] = {
// mclk, rate, p, r, j, d, nadc, madc, aosr
// 8k rate
    { 12000000, 8000, 1, 1, 7, 1680, 42, 2, 128 },
    { 12288000, 8000, 1, 1, 7, 0000, 42, 2, 128 },
// 11.025k rate
    { 12000000, 11025, 1, 1, 6, 8208, 29, 2, 128 },
// 16k rate
    { 12000000, 16000, 1, 1, 7, 1680, 21, 2, 128 },
    { 12288000, 16000, 1, 1, 7, 0000, 21, 2, 128 },
// 22.05k rate
    { 12000000, 22050, 1, 1, 7, 560, 15, 2, 128 },
// 32k rate
    { 12000000, 32000, 1, 1, 8, 1920, 12, 2, 128 },
    { 12288000, 32000, 1, 1, 8, 0000, 12, 2, 128 },
// 44.1k rate
    { 12000000, 44100, 1, 1, 7, 5264, 8, 2, 128 },
// 48k rate
    { 12000000, 48000, 1, 1, 7, 1680, 7, 2, 128 },
    { 12288000, 48000, 1, 1, 7, 0000, 7, 2, 128 },
    { 24576000, 48000, 1, 1, 3, 5000, 7, 2, 128 }, /* With PLL */
    { 24576000, 48000, 0, 0, 0, 0000, 2, 2, 128 }, /* Without PLL */
// 88.2k rate
    { 12000000, 88200, 1, 1, 7, 5264, 4, 4, 64 },
// 96k rate
    { 12000000, 96000, 1, 1, 8, 1920, 4, 4, 64 },
    };
#[no_mangle]
unsafe extern "C" fn adc3xxx_get_divs(dev: *mut device, mclk: c_int, rate: c_int, pll_mode: c_int) -> c_int {
    static int adc3xxx_get_divs(struct device *dev, int mclk, int rate, int pll_mode)
    {
    int i;
    dev_dbg(dev, "mclk = %d, rate = %d, clock mode %u\n",
    mclk, rate, pll_mode);
    for (i = 0; i < ARRAY_SIZE(adc3xxx_divs); i++) {
    const struct adc3xxx_rate_divs *mode = &adc3xxx_divs[i];
// Skip this entry if it doesn't fulfill the intended clock
// mode requirement. We consider anything besides the two
// modes below to be the same as ADC3XXX_PLL_AUTO.
//
    if ((pll_mode == ADC3XXX_PLL_BYPASS && mode.pll_p) ||
    (pll_mode == ADC3XXX_PLL_ENABLE && !mode.pll_p))
    continue;
    if (mode.rate == rate && mode.mclk == mclk)
    return i;
    }
    dev_info(dev, "Master clock rate %d and sample rate %d is not supported\n",
    mclk, rate);
    return -EINVAL;
    }
    static int adc3xxx_pll_delay(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
// 10msec delay needed after PLL power-up to allow
// PLL and dividers to stabilize (datasheet p13).
//
    usleep_range(10000, 20000);
    return 0;
    }
    static int adc3xxx_coefficient_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    let mut numcoeff: c_int = kcontrol.private_value >> 16;
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = numcoeff;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 0xffff; /* all coefficients are 16 bit */
    return 0;
    }
    static int adc3xxx_coefficient_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    let mut numcoeff: c_int = kcontrol.private_value >> 16;
    let mut reg: c_int = kcontrol.private_value & 0xffff;
    let mut index: c_int = 0;
    for (index = 0; index < numcoeff; index++) {
    unsigned int value_msb, value_lsb, value;
    value_msb = snd_soc_component_read(component, reg++);
    if ((int)value_msb < 0)
    return (int)value_msb;
    value_lsb = snd_soc_component_read(component, reg++);
    if ((int)value_lsb < 0)
    return (int)value_lsb;
    value = (value_msb << 8) | value_lsb;
    ucontrol.value.integer.value[index] = value;
    }
    return 0;
    }
    static int adc3xxx_coefficient_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    let mut numcoeff: c_int = kcontrol.private_value >> 16;
    let mut reg: c_int = kcontrol.private_value & 0xffff;
    let mut index: c_int = 0;
    int ret;
    for (index = 0; index < numcoeff; index++) {
    let mut value: c_uint = ucontrol.value.integer.value[index];
    let mut value_msb: c_uint = (value >> 8) & 0xff;
    let mut value_lsb: c_uint = value & 0xff;
    ret = snd_soc_component_write(component, reg++, value_msb);
    if (ret)
    return ret;
    ret = snd_soc_component_write(component, reg++, value_lsb);
    if (ret)
    return ret;
    }
    return 0;
    }
// All on-chip filters have coefficients which are expressed in terms of
// 16 bit values, so represent them as strings of 16-bit integers.
//

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name = xname, \
    .info = adc3xxx_coefficient_info, \
    .get = adc3xxx_coefficient_get,\
    .put = adc3xxx_coefficient_put, \
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE, \
    .private_value = reg | (numcoeffs << 16) \
    }
    static const char * const adc_softstepping_text[] = { "1 step", "2 step", "off" };
    static SOC_ENUM_SINGLE_DECL(adc_softstepping_enum, ADC3XXX_ADC_DIGITAL, 0,
    adc_softstepping_text);
    static const char * const multiplier_text[] = { "1", "2", "4", "8", "16", "32", "64", "128" };
    static SOC_ENUM_SINGLE_DECL(left_agc_attack_mult_enum,
    ADC3XXX_LEFT_CHN_AGC_4, 0, multiplier_text);
    static SOC_ENUM_SINGLE_DECL(right_agc_attack_mult_enum,
    ADC3XXX_RIGHT_CHN_AGC_4, 0, multiplier_text);
    static SOC_ENUM_SINGLE_DECL(left_agc_decay_mult_enum,
    ADC3XXX_LEFT_CHN_AGC_5, 0, multiplier_text);
    static SOC_ENUM_SINGLE_DECL(right_agc_decay_mult_enum,
    ADC3XXX_RIGHT_CHN_AGC_5, 0, multiplier_text);
    static const char * const dither_dc_offset_text[] = {
    "0mV", "15mV", "30mV", "45mV", "60mV", "75mV", "90mV", "105mV",
    "-15mV", "-30mV", "-45mV", "-60mV", "-75mV", "-90mV", "-105mV"
    };
    static const unsigned int dither_dc_offset_values[] = {
    0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15
    };
    static SOC_VALUE_ENUM_DOUBLE_DECL(dither_dc_offset_enum,
    ADC3XXX_DITHER_CTRL,
    4, 0, 0xf, dither_dc_offset_text,
    dither_dc_offset_values);
    static const DECLARE_TLV_DB_SCALE(pga_tlv, 0, 50, 0);
    static const DECLARE_TLV_DB_SCALE(adc_tlv, -1200, 50, 0);
    static const DECLARE_TLV_DB_SCALE(adc_fine_tlv, -40, 10, 0);
// AGC target: 8 values: -5.5, -8, -10, -12, -14, -17, -20, -24 dB
// It would be nice to declare these in the order above, but empirically
// TLV_DB_SCALE_ITEM doesn't take lightly to the increment (second) parameter
// being negative, despite there being examples to the contrary in other
// drivers. So declare these in the order from lowest to highest, and
// set the invert flag in the SOC_DOUBLE_R_TLV declaration instead.
//
    static const DECLARE_TLV_DB_RANGE(agc_target_tlv,
    0, 0, TLV_DB_SCALE_ITEM(-2400, 0, 0),
    1, 3, TLV_DB_SCALE_ITEM(-2000, 300, 0),
    4, 6, TLV_DB_SCALE_ITEM(-1200, 200, 0),
    7, 7, TLV_DB_SCALE_ITEM(-550, 0, 0));
// Since the 'disabled' value (mute) is at the highest value in the dB
// range (i.e. just before -32 dB) rather than the lowest, we need to resort
// to using a TLV_DB_RANGE in order to get the mute value in the right place.
//
    static const DECLARE_TLV_DB_RANGE(agc_thresh_tlv,
    0, 30, TLV_DB_SCALE_ITEM(-9000, 200, 0),
    31, 31, TLV_DB_SCALE_ITEM(0, 0, 1)); /* disabled = mute */
// AGC hysteresis: 4 values: 1, 2, 4 dB, disabled (= mute)
    static const DECLARE_TLV_DB_RANGE(agc_hysteresis_tlv,
    0, 1, TLV_DB_SCALE_ITEM(100, 100, 0),
    2, 2, TLV_DB_SCALE_ITEM(400, 0, 0),
    3, 3, TLV_DB_SCALE_ITEM(0, 0, 1)); /* disabled = mute */
    static const DECLARE_TLV_DB_SCALE(agc_max_tlv, 0, 50, 0);
// Input attenuation: -6 dB or 0 dB
    static const DECLARE_TLV_DB_SCALE(input_attenuation_tlv, -600, 600, 0);
    static const struct snd_kcontrol_new adc3xxx_snd_controls[] = {
    SOC_DOUBLE_R_TLV("PGA Capture Volume", ADC3XXX_LEFT_APGA_CTRL,
    ADC3XXX_RIGHT_APGA_CTRL, 0, 80, 0, pga_tlv),
    SOC_DOUBLE("PGA Capture Switch", ADC3XXX_ADC_FGA, 7, 3, 1, 1),
    SOC_DOUBLE_R("AGC Capture Switch", ADC3XXX_LEFT_CHN_AGC_1,
    ADC3XXX_RIGHT_CHN_AGC_1, 7, 1, 0),
    SOC_DOUBLE_R_TLV("AGC Target Level Capture Volume", ADC3XXX_LEFT_CHN_AGC_1,
    ADC3XXX_RIGHT_CHN_AGC_2, 4, 0x07, 1, agc_target_tlv),
    SOC_DOUBLE_R_TLV("AGC Noise Threshold Capture Volume", ADC3XXX_LEFT_CHN_AGC_2,
    ADC3XXX_RIGHT_CHN_AGC_2, 1, 0x1f, 1, agc_thresh_tlv),
    SOC_DOUBLE_R_TLV("AGC Hysteresis Capture Volume", ADC3XXX_LEFT_CHN_AGC_2,
    ADC3XXX_RIGHT_CHN_AGC_2, 6, 3, 0, agc_hysteresis_tlv),
    SOC_DOUBLE_R("AGC Clip Stepping Capture Switch", ADC3XXX_LEFT_CHN_AGC_2,
    ADC3XXX_RIGHT_CHN_AGC_2, 0, 1, 0),
//
// Oddly enough, the data sheet says the default value
// for the left/right AGC maximum gain register field
// (ADC3XXX_LEFT/RIGHT_CHN_AGC_3 bits 0..6) is 0x7f = 127
// (verified empirically) even though this value (indeed, above
// 0x50) is specified as 'Reserved. Do not use.' in the accompanying
// table in the data sheet.
//
    SOC_DOUBLE_R_TLV("AGC Maximum Capture Volume", ADC3XXX_LEFT_CHN_AGC_3,
    ADC3XXX_RIGHT_CHN_AGC_3, 0, 0x50, 0, agc_max_tlv),
    SOC_DOUBLE_R("AGC Attack Time", ADC3XXX_LEFT_CHN_AGC_4,
    ADC3XXX_RIGHT_CHN_AGC_4, 3, 0x1f, 0),
// Would like to have the multipliers as LR pairs, but there is
// no SOC_ENUM_foo which accepts two values in separate registers.
//
    SOC_ENUM("AGC Left Attack Time Multiplier", left_agc_attack_mult_enum),
    SOC_ENUM("AGC Right Attack Time Multiplier", right_agc_attack_mult_enum),
    SOC_DOUBLE_R("AGC Decay Time", ADC3XXX_LEFT_CHN_AGC_5,
    ADC3XXX_RIGHT_CHN_AGC_5, 3, 0x1f, 0),
    SOC_ENUM("AGC Left Decay Time Multiplier", left_agc_decay_mult_enum),
    SOC_ENUM("AGC Right Decay Time Multiplier", right_agc_decay_mult_enum),
    SOC_DOUBLE_R("AGC Noise Debounce", ADC3XXX_LEFT_CHN_AGC_6,
    ADC3XXX_RIGHT_CHN_AGC_6, 0, 0x1f, 0),
    SOC_DOUBLE_R("AGC Signal Debounce", ADC3XXX_LEFT_CHN_AGC_7,
    ADC3XXX_RIGHT_CHN_AGC_7, 0, 0x0f, 0),
// Read only register
    SOC_DOUBLE_R_S_TLV("AGC Applied Capture Volume", ADC3XXX_LEFT_AGC_GAIN,
    ADC3XXX_RIGHT_AGC_GAIN, 0, -24, 40, 6, 0, adc_tlv),
// ADC soft stepping
    SOC_ENUM("ADC Soft Stepping", adc_softstepping_enum),
// Left/Right Input attenuation
    SOC_SINGLE_TLV("Left Input IN_1L Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_1, 0, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input IN_2L Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_1, 2, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input IN_3L Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_1, 4, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input IN_1R Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_2, 0, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input DIF_2L_3L Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_1, 6, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input DIF_1L_1R Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_2, 4, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Left Input DIF_2R_3R Capture Volume",
    ADC3XXX_LEFT_PGA_SEL_2, 2, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input IN_1R Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_1, 0, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input IN_2R Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_1, 2, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input IN_3R Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_1, 4, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input IN_1L Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_2, 0, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input DIF_2R_3R Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_1, 6, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input DIF_1L_1R Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_2, 4, 1, 1, input_attenuation_tlv),
    SOC_SINGLE_TLV("Right Input DIF_2L_3L Capture Volume",
    ADC3XXX_RIGHT_PGA_SEL_2, 2, 1, 1, input_attenuation_tlv),
    SOC_DOUBLE_R_S_TLV("ADC Volume Control Capture Volume", ADC3XXX_LADC_VOL,
    ADC3XXX_RADC_VOL, 0, -24, 40, 6, 0, adc_tlv),
// Empirically, the following doesn't work the way it's supposed
// to. Values 0, -0.1, -0.2 and -0.3 dB result in the same level, and
// -0.4 dB drops about 0.12 dB on a specific chip.
//
    SOC_DOUBLE_TLV("ADC Fine Volume Control Capture Volume", ADC3XXX_ADC_FGA,
    4, 0, 4, 1, adc_fine_tlv),
    SOC_SINGLE("Left ADC Unselected CM Bias Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_2, 6, 1, 0),
    SOC_SINGLE("Right ADC Unselected CM Bias Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_2, 6, 1, 0),
    SOC_ENUM("Dither Control DC Offset", dither_dc_offset_enum),
// Coefficient memory for miniDSP.
// For the default PRB_R1 processing block, the only available
// filter is the first order IIR.
//
    TI_COEFFICIENTS("Left ADC IIR Coefficients N0 N1 D1",
    ADC3XXX_LEFT_ADC_IIR_COEFF_N0_MSB, 3),
    TI_COEFFICIENTS("Right ADC IIR Coefficients N0 N1 D1",
    ADC3XXX_RIGHT_ADC_IIR_COEFF_N0_MSB, 3),
    };
// Left input selection, Single Ended inputs and Differential inputs
    static const struct snd_kcontrol_new left_input_mixer_controls[] = {
    SOC_DAPM_SINGLE("IN_1L Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_1, 1, 0x1, 1),
    SOC_DAPM_SINGLE("IN_2L Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_1, 3, 0x1, 1),
    SOC_DAPM_SINGLE("IN_3L Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_1, 5, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_2L_3L Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_1, 7, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_1L_1R Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_2, 5, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_2R_3R Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_2, 3, 0x1, 1),
    SOC_DAPM_SINGLE("IN_1R Capture Switch",
    ADC3XXX_LEFT_PGA_SEL_2, 1, 0x1, 1),
    };
// Right input selection, Single Ended inputs and Differential inputs
    static const struct snd_kcontrol_new right_input_mixer_controls[] = {
    SOC_DAPM_SINGLE("IN_1R Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_1, 1, 0x1, 1),
    SOC_DAPM_SINGLE("IN_2R Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_1, 3, 0x1, 1),
    SOC_DAPM_SINGLE("IN_3R Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_1, 5, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_2R_3R Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_1, 7, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_1L_1R Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_2, 5, 0x1, 1),
    SOC_DAPM_SINGLE("DIF_2L_3L Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_2, 3, 0x1, 1),
    SOC_DAPM_SINGLE("IN_1L Capture Switch",
    ADC3XXX_RIGHT_PGA_SEL_2, 1, 0x1, 1),
    };
// Left Digital Mic input for left ADC
    static const struct snd_kcontrol_new left_input_dmic_controls[] = {
    SOC_DAPM_SINGLE("Left ADC Capture Switch",
    ADC3XXX_ADC_DIGITAL, 3, 0x1, 0),
    };
// Right Digital Mic input for Right ADC
    static const struct snd_kcontrol_new right_input_dmic_controls[] = {
    SOC_DAPM_SINGLE("Right ADC Capture Switch",
    ADC3XXX_ADC_DIGITAL, 2, 0x1, 0),
    };
// DAPM widgets
    static const struct snd_soc_dapm_widget adc3xxx_dapm_widgets[] = {
// Left Input Selection
    SND_SOC_DAPM_MIXER("Left Input", SND_SOC_NOPM, 0, 0,
    &left_input_mixer_controls[0],
    ARRAY_SIZE(left_input_mixer_controls)),
// Right Input Selection
    SND_SOC_DAPM_MIXER("Right Input", SND_SOC_NOPM, 0, 0,
    &right_input_mixer_controls[0],
    ARRAY_SIZE(right_input_mixer_controls)),
// PGA selection
    SND_SOC_DAPM_PGA("Left PGA", ADC3XXX_LEFT_APGA_CTRL, 7, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Right PGA", ADC3XXX_RIGHT_APGA_CTRL, 7, 1, core::ptr::null_mut(), 0),
// Digital Microphone Input Control for Left/Right ADC
    SND_SOC_DAPM_MIXER("Left DMic Input", SND_SOC_NOPM, 0, 0,
    &left_input_dmic_controls[0],
    ARRAY_SIZE(left_input_dmic_controls)),
    SND_SOC_DAPM_MIXER("Right DMic Input", SND_SOC_NOPM, 0, 0,
    &right_input_dmic_controls[0],
    ARRAY_SIZE(right_input_dmic_controls)),
// Left/Right ADC
    SND_SOC_DAPM_ADC("Left ADC", "Left Capture", ADC3XXX_ADC_DIGITAL, 7, 0),
    SND_SOC_DAPM_ADC("Right ADC", "Right Capture", ADC3XXX_ADC_DIGITAL, 6, 0),
// Inputs
    SND_SOC_DAPM_INPUT("IN_1L"),
    SND_SOC_DAPM_INPUT("IN_1R"),
    SND_SOC_DAPM_INPUT("IN_2L"),
    SND_SOC_DAPM_INPUT("IN_2R"),
    SND_SOC_DAPM_INPUT("IN_3L"),
    SND_SOC_DAPM_INPUT("IN_3R"),
    SND_SOC_DAPM_INPUT("DIFL_1L_1R"),
    SND_SOC_DAPM_INPUT("DIFL_2L_3L"),
    SND_SOC_DAPM_INPUT("DIFL_2R_3R"),
    SND_SOC_DAPM_INPUT("DIFR_1L_1R"),
    SND_SOC_DAPM_INPUT("DIFR_2L_3L"),
    SND_SOC_DAPM_INPUT("DIFR_2R_3R"),
    SND_SOC_DAPM_INPUT("DMic_L"),
    SND_SOC_DAPM_INPUT("DMic_R"),
// Digital audio interface output
    SND_SOC_DAPM_AIF_OUT("AIF_OUT", "Capture", 0, SND_SOC_NOPM, 0, 0),
// Clocks
    SND_SOC_DAPM_SUPPLY("PLL_CLK", ADC3XXX_PLL_PROG_PR, ADC3XXX_ENABLE_PLL_SHIFT,
    0, adc3xxx_pll_delay, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SUPPLY("ADC_CLK", ADC3XXX_ADC_NADC, ADC3XXX_ENABLE_NADC_SHIFT,
    0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("ADC_MOD_CLK", ADC3XXX_ADC_MADC, ADC3XXX_ENABLE_MADC_SHIFT,
    0, core::ptr::null_mut(), 0),
// This refers to the generated BCLK in master mode.
    SND_SOC_DAPM_SUPPLY("BCLK", ADC3XXX_BCLK_N_DIV, ADC3XXX_ENABLE_BCLK_SHIFT,
    0, core::ptr::null_mut(), 0),
    };
    static const struct snd_soc_dapm_route adc3xxx_intercon[] = {
// Left input selection from switches
    { "Left Input", "IN_1L Capture Switch", "IN_1L" },
    { "Left Input", "IN_2L Capture Switch", "IN_2L" },
    { "Left Input", "IN_3L Capture Switch", "IN_3L" },
    { "Left Input", "DIF_2L_3L Capture Switch", "DIFL_2L_3L" },
    { "Left Input", "DIF_1L_1R Capture Switch", "DIFL_1L_1R" },
    { "Left Input", "DIF_2R_3R Capture Switch", "DIFL_2R_3R" },
    { "Left Input", "IN_1R Capture Switch", "IN_1R" },
// Left input selection to left PGA
    { "Left PGA", core::ptr::null_mut(), "Left Input" },
// Left PGA to left ADC
    { "Left ADC", core::ptr::null_mut(), "Left PGA" },
// Right input selection from switches
    { "Right Input", "IN_1R Capture Switch", "IN_1R" },
    { "Right Input", "IN_2R Capture Switch", "IN_2R" },
    { "Right Input", "IN_3R Capture Switch", "IN_3R" },
    { "Right Input", "DIF_2R_3R Capture Switch", "DIFR_2R_3R" },
    { "Right Input", "DIF_1L_1R Capture Switch", "DIFR_1L_1R" },
    { "Right Input", "DIF_2L_3L Capture Switch", "DIFR_2L_3L" },
    { "Right Input", "IN_1L Capture Switch", "IN_1L" },
// Right input selection to right PGA
    { "Right PGA", core::ptr::null_mut(), "Right Input" },
// Right PGA to right ADC
    { "Right ADC", core::ptr::null_mut(), "Right PGA" },
// Left DMic Input selection from switch
    { "Left DMic Input", "Left ADC Capture Switch", "DMic_L" },
// Left DMic to left ADC
    { "Left ADC", core::ptr::null_mut(), "Left DMic Input" },
// Right DMic Input selection from switch
    { "Right DMic Input", "Right ADC Capture Switch", "DMic_R" },
// Right DMic to right ADC
    { "Right ADC", core::ptr::null_mut(), "Right DMic Input" },
// ADC to AIF output
    { "AIF_OUT", core::ptr::null_mut(), "Left ADC" },
    { "AIF_OUT", core::ptr::null_mut(), "Right ADC" },
// Clocking
    { "ADC_MOD_CLK", core::ptr::null_mut(), "ADC_CLK" },
    { "Left ADC", core::ptr::null_mut(), "ADC_MOD_CLK" },
    { "Right ADC", core::ptr::null_mut(), "ADC_MOD_CLK" },
    { "BCLK", core::ptr::null_mut(), "ADC_CLK" },
    };
    static const struct snd_soc_dapm_route adc3xxx_pll_intercon[] = {
    { "ADC_CLK", core::ptr::null_mut(), "PLL_CLK" },
    };
    static const struct snd_soc_dapm_route adc3xxx_bclk_out_intercon[] = {
    { "AIF_OUT", core::ptr::null_mut(), "BCLK" }
    };
#[no_mangle]
unsafe extern "C" fn adc3xxx_gpio_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int adc3xxx_gpio_request(struct gpio_chip *chip, unsigned int offset)
    {
    struct adc3xxx *adc3xxx = gpiochip_get_data(chip);
    if (offset >= ADC3XXX_GPIOS_MAX)
    return -EINVAL;
    if (offset < ADC3XXX_GPIO_PINS) {
// GPIO1 is offset 0, GPIO2 is offset 1
// We check here that the GPIO pins are either not configured
// in the DT, or that they purposely are set as outputs.
// (Input mode not yet implemented).
//
    if (adc3xxx.gpio_cfg[offset] != 0 &&
    adc3xxx.gpio_cfg[offset] != ADC3XXX_GPIO_GPO + 1)
    return -EINVAL;
    } else if (offset >= ADC3XXX_GPIO_PINS && offset < ADC3XXX_GPIOS_MAX) {
// MICBIAS1 is offset 2, MICBIAS2 is offset 3
// We check here if the MICBIAS pins are in fact configured
// as GPOs.
//
    if (!adc3xxx.micbias_gpo[offset - ADC3XXX_GPIO_PINS])
    return -EINVAL;
    }
    return 0;
    }
    static int adc3xxx_gpio_direction_out(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    struct adc3xxx *adc3xxx = gpiochip_get_data(chip);
// For the MICBIAS pins, they are by definition outputs.
    if (offset >= ADC3XXX_GPIO_PINS) {
    unsigned int vg;
    let mut micbias: c_uint = offset - ADC3XXX_GPIO_PINS;
    if (value)
    vg = adc3xxx.micbias_vg[micbias];
    else
    vg = ADC3XXX_MICBIAS_OFF;
    return regmap_update_bits(adc3xxx.regmap,
    ADC3XXX_MICBIAS_CTRL,
    ADC3XXX_MICBIAS_MASK << adc3xxx_micbias_shift[micbias],
    vg << adc3xxx_micbias_shift[micbias]);
    }
// Set GPIO output function.
    return regmap_update_bits(adc3xxx.regmap,
    adc3xxx_gpio_ctrl_reg[offset],
    ADC3XXX_GPIO_CTRL_CFG_MASK |
    ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK,
    ADC3XXX_GPIO_GPO << ADC3XXX_GPIO_CTRL_CFG_SHIFT |
    !!value << ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_SHIFT);
    }
// With only GPIO outputs configured, we never get the .direction_out call,
// so we set the output mode and output value in the same call. Hence
// .set in practice does the same thing as .direction_out .
//
    static int adc3xxx_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    return adc3xxx_gpio_direction_out(chip, offset, value);
    }
// Even though we only support GPIO output for now, some GPIO clients
// want to read the current pin state using the .get callback.
//
#[no_mangle]
unsafe extern "C" fn adc3xxx_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int adc3xxx_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct adc3xxx *adc3xxx = gpiochip_get_data(chip);
    unsigned int regval;
    int ret;
// We only allow output pins, so just read the value prevously set.
    if (offset >= ADC3XXX_GPIO_PINS) {
// MICBIAS pins
    let mut micbias: c_uint = offset - ADC3XXX_GPIO_PINS;
    ret = regmap_read(adc3xxx.regmap, ADC3XXX_MICBIAS_CTRL, &regval);
    if (ret)
    return ret;
    return ((regval >> adc3xxx_micbias_shift[micbias]) & ADC3XXX_MICBIAS_MASK) !=
    ADC3XXX_MICBIAS_OFF;
    }
    ret = regmap_read(adc3xxx.regmap, adc3xxx_gpio_ctrl_reg[offset], &regval);
    if (ret)
    return ret;
    return !!(regval & ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK);
    }
    static const struct gpio_chip adc3xxx_gpio_chip = {
    .label			= "adc3xxx",
    .owner			= THIS_MODULE,
    .request		= adc3xxx_gpio_request,
    .direction_output	= adc3xxx_gpio_direction_out,
    .set			= adc3xxx_gpio_set,
    .get			= adc3xxx_gpio_get,
    .can_sleep		= 1,
    };
#[no_mangle]
unsafe extern "C" fn adc3xxx_free_gpio(adc3xxx: *mut adc3xxx) {
    static void adc3xxx_free_gpio(struct adc3xxx *adc3xxx)
    {

    gpiochip_remove(&adc3xxx.gpio_chip);

    }
#[no_mangle]
unsafe extern "C" fn adc3xxx_init_gpio(adc3xxx: *mut adc3xxx) {
    static void adc3xxx_init_gpio(struct adc3xxx *adc3xxx)
    {
    int gpio, micbias;
    int ret;
    adc3xxx.gpio_chip = adc3xxx_gpio_chip;
    adc3xxx.gpio_chip.ngpio = ADC3XXX_GPIOS_MAX;
    adc3xxx.gpio_chip.parent = adc3xxx.dev;
    adc3xxx.gpio_chip.base = -1;
    ret = gpiochip_add_data(&adc3xxx.gpio_chip, adc3xxx);
    if (ret)
    dev_err(adc3xxx.dev, "Failed to add gpios: %d\n", ret);
// Set up potential GPIO configuration from the devicetree.
// This allows us to set up things which are not software
// controllable GPIOs, such as PDM microphone I/O,
//
    for (gpio = 0; gpio < ADC3XXX_GPIO_PINS; gpio++) {
    let mut cfg: c_uint = adc3xxx.gpio_cfg[gpio];
    if (cfg) {
    cfg--; /* actual value to use is stored +1 */
    regmap_update_bits(adc3xxx.regmap,
    adc3xxx_gpio_ctrl_reg[gpio],
    ADC3XXX_GPIO_CTRL_CFG_MASK,
    cfg << ADC3XXX_GPIO_CTRL_CFG_SHIFT);
    }
    }
// Set up micbias voltage.
// If pin is configured as GPO, set off initially.
    for (micbias = 0; micbias < ADC3XXX_MICBIAS_PINS; micbias++) {
    unsigned int vg;
    if (adc3xxx.micbias_gpo[micbias])
    vg = ADC3XXX_MICBIAS_OFF;
    else
    vg = adc3xxx.micbias_vg[micbias];
    regmap_update_bits(adc3xxx.regmap,
    ADC3XXX_MICBIAS_CTRL,
    ADC3XXX_MICBIAS_MASK << adc3xxx_micbias_shift[micbias],
    vg << adc3xxx_micbias_shift[micbias]);
    }
    }
    static int adc3xxx_parse_dt_gpio(struct adc3xxx *adc3xxx,
    const char *propname, unsigned int *cfg)
    {
    struct device *dev = adc3xxx.dev;
    struct device_node *np = dev.of_node;
    unsigned int val;
    if (!of_property_read_u32(np, propname, &val)) {
    if (val & ~15 || val == 7 || val >= 11) {
    dev_err(dev, "Invalid property value for '%s'\n", propname);
    return -EINVAL;
    }
    if (val == ADC3XXX_GPIO_GPI)
    dev_warn(dev, "GPIO Input read not yet implemented\n");
// cfg = val + 1; /* 0 => not set up, all others shifted +1
    }
    return 0;
    }
    static int adc3xxx_parse_dt_micbias_gpo(struct adc3xxx *adc3xxx,
    const char *propname,
    unsigned int *cfg)
    {
    struct device *dev = adc3xxx.dev;
    struct device_node *np = dev.of_node;
// cfg = of_property_read_bool(np, propname);
    return 0;
    }
    static int adc3xxx_parse_dt_micbias_vg(struct adc3xxx *adc3xxx,
    const char *propname, unsigned int *vg)
    {
    struct device *dev = adc3xxx.dev;
    struct device_node *np = dev.of_node;
    unsigned int val;
    if (!of_property_read_u32(np, propname, &val)) {
    if (val > ADC3XXX_MICBIAS_AVDD) {
    dev_err(dev, "Invalid property value for '%s'\n", propname);
    return -EINVAL;
    }
// vg = val;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc3xxx_parse_pll_mode(val: u32, pll_mode: *mut c_uint) -> c_int {
    static int adc3xxx_parse_pll_mode(uint32_t val, unsigned int *pll_mode)
    {
    if (val != ADC3XXX_PLL_ENABLE && val != ADC3XXX_PLL_BYPASS &&
    val != ADC3XXX_PLL_AUTO)
    return -EINVAL;
// pll_mode = val;
    return 0;
    }
    static void adc3xxx_setup_pll(struct snd_soc_component *component,
    int div_entry)
    {
    let mut i: c_int = div_entry;
// P & R values
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_PR,
    (adc3xxx_divs[i].pll_p << ADC3XXX_PLLP_SHIFT) |
    (adc3xxx_divs[i].pll_r << ADC3XXX_PLLR_SHIFT));
// J value
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_J,
    adc3xxx_divs[i].pll_j & ADC3XXX_PLLJ_MASK);
// D value
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_D_LSB,
    adc3xxx_divs[i].pll_d & ADC3XXX_PLLD_LSB_MASK);
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_D_MSB,
    (adc3xxx_divs[i].pll_d >> 8) & ADC3XXX_PLLD_MSB_MASK);
    }
    static int adc3xxx_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(dai.component);
    struct adc3xxx *adc3xxx = snd_soc_component_get_drvdata(component);
    int i, width = 16;
    u8 iface_len, bdiv;
    i = adc3xxx_get_divs(component.dev, adc3xxx.sysclk,
    params_rate(params), adc3xxx.pll_mode);
    if (i < 0)
    return i;
// select data word length
    switch (params_width(params)) {
    case 16:
    iface_len = ADC3XXX_IFACE_16BITS;
    width = 16;
    break;
    case 20:
    iface_len = ADC3XXX_IFACE_20BITS;
    width = 20;
    break;
    case 24:
    iface_len = ADC3XXX_IFACE_24BITS;
    width = 24;
    break;
    case 32:
    iface_len = ADC3XXX_IFACE_32BITS;
    width = 32;
    break;
    default:
    dev_err(component.dev, "Unsupported serial data format\n");
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, ADC3XXX_INTERFACE_CTRL_1,
    ADC3XXX_WLENGTH_MASK, iface_len);
    if (adc3xxx_divs[i].pll_p) { /* If PLL used for this mode */
    adc3xxx_setup_pll(component, i);
    snd_soc_component_write(component, ADC3XXX_CLKGEN_MUX, ADC3XXX_USE_PLL);
    if (!adc3xxx.use_pll) {
    snd_soc_dapm_add_routes(dapm, adc3xxx_pll_intercon,
    ARRAY_SIZE(adc3xxx_pll_intercon));
    adc3xxx.use_pll = 1;
    }
    } else {
    snd_soc_component_write(component, ADC3XXX_CLKGEN_MUX, ADC3XXX_NO_PLL);
    if (adc3xxx.use_pll) {
    snd_soc_dapm_del_routes(dapm, adc3xxx_pll_intercon,
    ARRAY_SIZE(adc3xxx_pll_intercon));
    adc3xxx.use_pll = 0;
    }
    }
// NADC
    snd_soc_component_update_bits(component, ADC3XXX_ADC_NADC,
    ADC3XXX_NADC_MASK, adc3xxx_divs[i].nadc);
// MADC
    snd_soc_component_update_bits(component, ADC3XXX_ADC_MADC,
    ADC3XXX_MADC_MASK, adc3xxx_divs[i].madc);
// AOSR
    snd_soc_component_update_bits(component, ADC3XXX_ADC_AOSR,
    ADC3XXX_AOSR_MASK, adc3xxx_divs[i].aosr);
// BDIV N Value
// BCLK is (by default) set up to be derived from ADC_CLK
    bdiv = (adc3xxx_divs[i].aosr * adc3xxx_divs[i].madc) / (2 * width);
    snd_soc_component_update_bits(component, ADC3XXX_BCLK_N_DIV,
    ADC3XXX_BDIV_MASK, bdiv);
    return 0;
    }
    static const char *adc3xxx_pll_mode_text(int pll_mode)
    {
    switch (pll_mode) {
    case ADC3XXX_PLL_AUTO:
    return "PLL auto";
    case ADC3XXX_PLL_ENABLE:
    return "PLL enable";
    case ADC3XXX_PLL_BYPASS:
    return "PLL bypass";
    default:
    break;
    }
    return "PLL unknown";
    }
    static int adc3xxx_set_dai_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct adc3xxx *adc3xxx = snd_soc_component_get_drvdata(component);
    int ret;
    ret = adc3xxx_parse_pll_mode(clk_id, &adc3xxx.pll_mode);
    if (ret < 0)
    return ret;
    adc3xxx.sysclk = freq;
    dev_dbg(component.dev, "Set sysclk to %u Hz, %s\n",
    freq, adc3xxx_pll_mode_text(adc3xxx.pll_mode));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc3xxx_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int adc3xxx_set_dai_fmt(struct snd_soc_dai *codec_dai, unsigned int fmt)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    struct adc3xxx *adc3xxx = snd_soc_component_get_drvdata(component);
    let mut clkdir: u8 = 0, format = 0;
    let mut master: c_int = 0;
    int ret;
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    master = 1;
    clkdir = ADC3XXX_BCLK_MASTER | ADC3XXX_WCLK_MASTER;
    break;
    case SND_SOC_DAIFMT_CBC_CFC:
    master = 0;
    break;
    default:
    dev_err(component.dev, "Invalid DAI clock setup\n");
    return -EINVAL;
    }
//
// match both interface format and signal polarities since they
// are fixed
//
    switch (fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK)) {
    case SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF:
    format = ADC3XXX_FORMAT_I2S;
    break;
    case SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF:
    format = ADC3XXX_FORMAT_DSP;
    break;
    case SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF:
    format = ADC3XXX_FORMAT_DSP;
    break;
    case SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_NB_NF:
    format = ADC3XXX_FORMAT_RJF;
    break;
    case SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF:
    format = ADC3XXX_FORMAT_LJF;
    break;
    default:
    dev_err(component.dev, "Invalid DAI format\n");
    return -EINVAL;
    }
// Add/del route enabling BCLK output as applicable
    if (master && !adc3xxx.master)
    snd_soc_dapm_add_routes(dapm, adc3xxx_bclk_out_intercon,
    ARRAY_SIZE(adc3xxx_bclk_out_intercon));
#[no_mangle]
pub unsafe extern "C" fn if(adc3xxx->master: !master &&) -> else {
    else if (!master && adc3xxx.master)
    snd_soc_dapm_del_routes(dapm, adc3xxx_bclk_out_intercon,
    ARRAY_SIZE(adc3xxx_bclk_out_intercon));
    adc3xxx.master = master;
// set clock direction and format
    ret = snd_soc_component_update_bits(component,
    ADC3XXX_INTERFACE_CTRL_1,
    ADC3XXX_CLKDIR_MASK | ADC3XXX_FORMAT_MASK,
    clkdir | format);
    if (ret < 0)
    return ret;
    return 0;
    }
    static const struct snd_soc_dai_ops adc3xxx_dai_ops = {
    .hw_params	= adc3xxx_hw_params,
    .set_sysclk	= adc3xxx_set_dai_sysclk,
    .set_fmt	= adc3xxx_set_dai_fmt,
    };
    static struct snd_soc_dai_driver adc3xxx_dai = {
    .name = "tlv320adc3xxx-hifi",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = ADC3XXX_RATES,
    .formats = ADC3XXX_FORMATS,
    },
    .ops = &adc3xxx_dai_ops,
    };
    static const struct snd_soc_component_driver soc_component_dev_adc3xxx = {
    .controls		= adc3xxx_snd_controls,
    .num_controls		= ARRAY_SIZE(adc3xxx_snd_controls),
    .dapm_widgets		= adc3xxx_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(adc3xxx_dapm_widgets),
    .dapm_routes		= adc3xxx_intercon,
    .num_dapm_routes	= ARRAY_SIZE(adc3xxx_intercon),
    .endianness		= 1,
    };
    static const struct i2c_device_id adc3xxx_i2c_id[] = {
    { .name = "tlv320adc3001", .driver_data = ADC3001 },
    { .name = "tlv320adc3101", .driver_data = ADC3101 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adc3xxx_i2c_id);
#[no_mangle]
unsafe extern "C" fn adc3xxx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int adc3xxx_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct adc3xxx *adc3xxx = core::ptr::null_mut();
    int ret;
    adc3xxx = devm_kzalloc(dev, sizeof(struct adc3xxx), GFP_KERNEL);
    if (!adc3xxx)
    return -ENOMEM;
    adc3xxx.dev = dev;
    adc3xxx.rst_pin = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(adc3xxx.rst_pin)) {
    return dev_err_probe(dev, PTR_ERR(adc3xxx.rst_pin),
    "Failed to request rst_pin\n");
    }
    adc3xxx.mclk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(adc3xxx.mclk)) {
//
// The chip itself supports running off the BCLK either
// directly or via the PLL, but the driver does not (yet), so
// having a specified mclk is required. Otherwise, we could
// use the lack of a clocks property to indicate when BCLK is
// intended as the clock source.
//
    return dev_err_probe(dev, PTR_ERR(adc3xxx.mclk),
    "Failed to acquire MCLK\n");
    } else if (adc3xxx.mclk) {
    ret = clk_prepare_enable(adc3xxx.mclk);
    if (ret < 0)
    return ret;
    dev_dbg(dev, "Enabled MCLK, freq %lu Hz\n", clk_get_rate(adc3xxx.mclk));
    }
// Configure mode for DMDIN/GPIO1 pin
    ret = adc3xxx_parse_dt_gpio(adc3xxx, "ti,dmdin-gpio1", &adc3xxx.gpio_cfg[0]);
    if (ret < 0)
    goto err_unprepare_mclk;
// Configure mode for DMCLK/GPIO2 pin
    ret = adc3xxx_parse_dt_gpio(adc3xxx, "ti,dmclk-gpio2", &adc3xxx.gpio_cfg[1]);
    if (ret < 0)
    goto err_unprepare_mclk;
// Configure mode for MICBIAS1: as Mic Bias output or GPO
    ret = adc3xxx_parse_dt_micbias_gpo(adc3xxx, "ti,micbias1-gpo", &adc3xxx.micbias_gpo[0]);
    if (ret < 0)
    goto err_unprepare_mclk;
// Configure mode for MICBIAS2: as Mic Bias output or GPO
    ret = adc3xxx_parse_dt_micbias_gpo(adc3xxx, "ti,micbias2-gpo", &adc3xxx.micbias_gpo[1]);
    if (ret < 0)
    goto err_unprepare_mclk;
// Configure voltage for MICBIAS1 pin (ON voltage when used as GPO)
    ret = adc3xxx_parse_dt_micbias_vg(adc3xxx, "ti,micbias1-vg", &adc3xxx.micbias_vg[0]);
    if (ret < 0)
    goto err_unprepare_mclk;
// Configure voltage for MICBIAS2 pin (ON voltage when used as GPO)
    ret = adc3xxx_parse_dt_micbias_vg(adc3xxx, "ti,micbias2-vg", &adc3xxx.micbias_vg[1]);
    if (ret < 0)
    goto err_unprepare_mclk;
    adc3xxx.regmap = devm_regmap_init_i2c(i2c, &adc3xxx_regmap);
    if (IS_ERR(adc3xxx.regmap)) {
    ret = PTR_ERR(adc3xxx.regmap);
    goto err_unprepare_mclk;
    }
    i2c_set_clientdata(i2c, adc3xxx);
    adc3xxx.type = (uintptr_t)i2c_get_match_data(i2c);
// Reset codec chip
    gpiod_set_value_cansleep(adc3xxx.rst_pin, 1);
    usleep_range(2000, 100000); /* Requirement: > 10 ns (datasheet p13) */
    gpiod_set_value_cansleep(adc3xxx.rst_pin, 0);
// Potentially set up pins used as GPIOs
    adc3xxx_init_gpio(adc3xxx);
    ret = snd_soc_register_component(dev,
    &soc_component_dev_adc3xxx, &adc3xxx_dai, 1);
    if (ret < 0) {
    dev_err(dev, "Failed to register codec: %d\n", ret);
    goto err_unprepare_mclk;
    }
    return 0;
    err_unprepare_mclk:
    clk_disable_unprepare(adc3xxx.mclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adc3xxx_i2c_remove(client: *mut i2c_client) {
    static void adc3xxx_i2c_remove(struct i2c_client *client)
    {
    struct adc3xxx *adc3xxx = i2c_get_clientdata(client);
    clk_disable_unprepare(adc3xxx.mclk);
    adc3xxx_free_gpio(adc3xxx);
    snd_soc_unregister_component(&client.dev);
    }
    static const struct of_device_id tlv320adc3xxx_of_match[] = {
    { .compatible = "ti,tlv320adc3001", },
    { .compatible = "ti,tlv320adc3101", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tlv320adc3xxx_of_match);
    static struct i2c_driver adc3xxx_i2c_driver = {
    .driver = {
    .name = "tlv320adc3xxx-codec",
    .of_match_table = tlv320adc3xxx_of_match,
    },
    .probe = adc3xxx_i2c_probe,
    .remove = adc3xxx_i2c_remove,
    .id_table = adc3xxx_i2c_id,
    };
    module_i2c_driver(adc3xxx_i2c_driver);
    MODULE_DESCRIPTION("ASoC TLV320ADC3xxx codec driver");
    MODULE_AUTHOR("shahina.s@mistralsolutions.com");
    MODULE_LICENSE("GPL v2");
