//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs48l32_registers.h
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
// Register definitions for Cirrus Logic CS48L32
//
// Copyright (C) 2017-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Register Addresses.
pub const CS48L32_DEVID: c_uint = 0x0;
pub const CS48L32_REVID: c_uint = 0x4;
pub const CS48L32_OTPID: c_uint = 0x10;
pub const CS48L32_SFT_RESET: c_uint = 0x20;
pub const CS48L32_CTRL_IF_DEBUG3: c_uint = 0xA8;
pub const CS48L32_MCU_CTRL1: c_uint = 0x804;
pub const CS48L32_GPIO1_CTRL1: c_uint = 0xc08;
pub const CS48L32_GPIO3_CTRL1: c_uint = 0xc10;
pub const CS48L32_GPIO7_CTRL1: c_uint = 0xc20;
pub const CS48L32_GPIO16_CTRL1: c_uint = 0xc44;
pub const CS48L32_OUTPUT_SYS_CLK: c_uint = 0x1020;
pub const CS48L32_AUXPDM_CTRL: c_uint = 0x1044;
pub const CS48L32_AUXPDM_CTRL2: c_uint = 0x105c;
pub const CS48L32_CLOCK32K: c_uint = 0x1400;
pub const CS48L32_SYSTEM_CLOCK1: c_uint = 0x1404;
pub const CS48L32_SYSTEM_CLOCK2: c_uint = 0x1408;
pub const CS48L32_SAMPLE_RATE1: c_uint = 0x1420;
pub const CS48L32_SAMPLE_RATE2: c_uint = 0x1424;
pub const CS48L32_SAMPLE_RATE3: c_uint = 0x1428;
pub const CS48L32_SAMPLE_RATE4: c_uint = 0x142c;
pub const CS48L32_DSP_CLOCK1: c_uint = 0x1510;
pub const CS48L32_FLL1_CONTROL1: c_uint = 0x1c00;
pub const CS48L32_FLL1_CONTROL5: c_uint = 0x1c10;
pub const CS48L32_FLL1_CONTROL6: c_uint = 0x1c14;
pub const CS48L32_FLL1_GPIO_CLOCK: c_uint = 0x1ca0;
pub const CS48L32_CHARGE_PUMP1: c_uint = 0x2000;
pub const CS48L32_LDO2_CTRL1: c_uint = 0x2408;
pub const CS48L32_MICBIAS_CTRL1: c_uint = 0x2410;
pub const CS48L32_MICBIAS_CTRL5: c_uint = 0x2418;
pub const CS48L32_IRQ1_CTRL_AOD: c_uint = 0x2710;
pub const CS48L32_AOD_PAD_CTRL: c_uint = 0x2718;
pub const CS48L32_INPUT_CONTROL: c_uint = 0x4000;
pub const CS48L32_INPUT_STATUS: c_uint = 0x4004;
pub const CS48L32_INPUT_RATE_CONTROL: c_uint = 0x4008;
pub const CS48L32_INPUT_CONTROL2: c_uint = 0x400c;
pub const CS48L32_INPUT_CONTROL3: c_uint = 0x4014;
pub const CS48L32_INPUT1_CONTROL1: c_uint = 0x4020;
pub const CS48L32_IN1L_CONTROL1: c_uint = 0x4024;
pub const CS48L32_IN1L_CONTROL2: c_uint = 0x4028;
pub const CS48L32_IN1R_CONTROL1: c_uint = 0x4044;
pub const CS48L32_IN1R_CONTROL2: c_uint = 0x4048;
pub const CS48L32_INPUT2_CONTROL1: c_uint = 0x4060;
pub const CS48L32_IN2L_CONTROL1: c_uint = 0x4064;
pub const CS48L32_IN2L_CONTROL2: c_uint = 0x4068;
pub const CS48L32_IN2R_CONTROL1: c_uint = 0x4084;
pub const CS48L32_IN2R_CONTROL2: c_uint = 0x4088;
pub const CS48L32_INPUT_HPF_CONTROL: c_uint = 0x4244;
pub const CS48L32_INPUT_VOL_CONTROL: c_uint = 0x4248;
pub const CS48L32_AUXPDM_CONTROL1: c_uint = 0x4300;
pub const CS48L32_AUXPDM_CONTROL2: c_uint = 0x4304;
pub const CS48L32_AUXPDM1_CONTROL1: c_uint = 0x4308;
pub const CS48L32_AUXPDM2_CONTROL1: c_uint = 0x4310;
pub const CS48L32_ADC1L_ANA_CONTROL1: c_uint = 0x4688;
pub const CS48L32_ADC1R_ANA_CONTROL1: c_uint = 0x468c;
pub const CS48L32_ASP1_ENABLES1: c_uint = 0x6000;
pub const CS48L32_ASP1_CONTROL3: c_uint = 0x600C;
pub const CS48L32_ASP1_DATA_CONTROL5: c_uint = 0x6040;
pub const CS48L32_ASP2_ENABLES1: c_uint = 0x6080;
pub const CS48L32_ASP2_CONTROL3: c_uint = 0x608C;
pub const CS48L32_ASP2_DATA_CONTROL5: c_uint = 0x60c0;
pub const CS48L32_ASP1TX1_INPUT1: c_uint = 0x8200;
pub const CS48L32_ASP1TX2_INPUT1: c_uint = 0x8210;
pub const CS48L32_ASP1TX3_INPUT1: c_uint = 0x8220;
pub const CS48L32_ASP1TX4_INPUT1: c_uint = 0x8230;
pub const CS48L32_ASP1TX5_INPUT1: c_uint = 0x8240;
pub const CS48L32_ASP1TX6_INPUT1: c_uint = 0x8250;
pub const CS48L32_ASP1TX7_INPUT1: c_uint = 0x8260;
pub const CS48L32_ASP1TX8_INPUT1: c_uint = 0x8270;
pub const CS48L32_ASP1TX8_INPUT4: c_uint = 0x827c;
pub const CS48L32_ASP2TX1_INPUT1: c_uint = 0x8300;
pub const CS48L32_ASP2TX2_INPUT1: c_uint = 0x8310;
pub const CS48L32_ASP2TX3_INPUT1: c_uint = 0x8320;
pub const CS48L32_ASP2TX4_INPUT1: c_uint = 0x8330;
pub const CS48L32_ASP2TX4_INPUT4: c_uint = 0x833c;
pub const CS48L32_ISRC1INT1_INPUT1: c_uint = 0x8980;
pub const CS48L32_ISRC1INT2_INPUT1: c_uint = 0x8990;
pub const CS48L32_ISRC1INT3_INPUT1: c_uint = 0x89a0;
pub const CS48L32_ISRC1INT4_INPUT1: c_uint = 0x89b0;
pub const CS48L32_ISRC1DEC1_INPUT1: c_uint = 0x89c0;
pub const CS48L32_ISRC1DEC2_INPUT1: c_uint = 0x89d0;
pub const CS48L32_ISRC1DEC3_INPUT1: c_uint = 0x89e0;
pub const CS48L32_ISRC1DEC4_INPUT1: c_uint = 0x89f0;
pub const CS48L32_ISRC2INT1_INPUT1: c_uint = 0x8a00;
pub const CS48L32_ISRC2INT2_INPUT1: c_uint = 0x8a10;
pub const CS48L32_ISRC2DEC1_INPUT1: c_uint = 0x8a40;
pub const CS48L32_ISRC2DEC2_INPUT1: c_uint = 0x8a50;
pub const CS48L32_ISRC3INT1_INPUT1: c_uint = 0x8a80;
pub const CS48L32_ISRC3INT2_INPUT1: c_uint = 0x8a90;
pub const CS48L32_ISRC3DEC1_INPUT1: c_uint = 0x8ac0;
pub const CS48L32_ISRC3DEC2_INPUT1: c_uint = 0x8ad0;
pub const CS48L32_EQ1_INPUT1: c_uint = 0x8b80;
pub const CS48L32_EQ2_INPUT1: c_uint = 0x8b90;
pub const CS48L32_EQ3_INPUT1: c_uint = 0x8ba0;
pub const CS48L32_EQ4_INPUT1: c_uint = 0x8bb0;
pub const CS48L32_EQ4_INPUT4: c_uint = 0x8bbc;
pub const CS48L32_DRC1L_INPUT1: c_uint = 0x8c00;
pub const CS48L32_DRC1R_INPUT1: c_uint = 0x8c10;
pub const CS48L32_DRC1R_INPUT4: c_uint = 0x8c1c;
pub const CS48L32_DRC2L_INPUT1: c_uint = 0x8c20;
pub const CS48L32_DRC2R_INPUT1: c_uint = 0x8c30;
pub const CS48L32_DRC2R_INPUT4: c_uint = 0x8c3c;
pub const CS48L32_LHPF1_INPUT1: c_uint = 0x8c80;
pub const CS48L32_LHPF1_INPUT4: c_uint = 0x8c8c;
pub const CS48L32_LHPF2_INPUT1: c_uint = 0x8c90;
pub const CS48L32_LHPF2_INPUT4: c_uint = 0x8c9c;
pub const CS48L32_LHPF3_INPUT1: c_uint = 0x8ca0;
pub const CS48L32_LHPF3_INPUT4: c_uint = 0x8cac;
pub const CS48L32_LHPF4_INPUT1: c_uint = 0x8cb0;
pub const CS48L32_LHPF4_INPUT4: c_uint = 0x8cbc;
pub const CS48L32_DSP1RX1_INPUT1: c_uint = 0x9000;
pub const CS48L32_DSP1RX2_INPUT1: c_uint = 0x9010;
pub const CS48L32_DSP1RX3_INPUT1: c_uint = 0x9020;
pub const CS48L32_DSP1RX4_INPUT1: c_uint = 0x9030;
pub const CS48L32_DSP1RX5_INPUT1: c_uint = 0x9040;
pub const CS48L32_DSP1RX6_INPUT1: c_uint = 0x9050;
pub const CS48L32_DSP1RX7_INPUT1: c_uint = 0x9060;
pub const CS48L32_DSP1RX8_INPUT1: c_uint = 0x9070;
pub const CS48L32_DSP1RX8_INPUT4: c_uint = 0x907c;
pub const CS48L32_ISRC1_CONTROL1: c_uint = 0xa400;
pub const CS48L32_ISRC1_CONTROL2: c_uint = 0xa404;
pub const CS48L32_ISRC2_CONTROL1: c_uint = 0xa510;
pub const CS48L32_ISRC2_CONTROL2: c_uint = 0xa514;
pub const CS48L32_ISRC3_CONTROL1: c_uint = 0xa620;
pub const CS48L32_ISRC3_CONTROL2: c_uint = 0xa624;
pub const CS48L32_FX_SAMPLE_RATE: c_uint = 0xa800;
pub const CS48L32_EQ_CONTROL1: c_uint = 0xa808;
pub const CS48L32_EQ_CONTROL2: c_uint = 0xa80c;
pub const CS48L32_EQ1_GAIN1: c_uint = 0xa810;
pub const CS48L32_EQ1_GAIN2: c_uint = 0xa814;
pub const CS48L32_EQ1_BAND1_COEFF1: c_uint = 0xa818;
pub const CS48L32_EQ1_BAND1_COEFF2: c_uint = 0xa81c;
pub const CS48L32_EQ1_BAND1_PG: c_uint = 0xa820;
pub const CS48L32_EQ1_BAND2_COEFF1: c_uint = 0xa824;
pub const CS48L32_EQ1_BAND2_COEFF2: c_uint = 0xa828;
pub const CS48L32_EQ1_BAND2_PG: c_uint = 0xa82c;
pub const CS48L32_EQ1_BAND3_COEFF1: c_uint = 0xa830;
pub const CS48L32_EQ1_BAND3_COEFF2: c_uint = 0xa834;
pub const CS48L32_EQ1_BAND3_PG: c_uint = 0xa838;
pub const CS48L32_EQ1_BAND4_COEFF1: c_uint = 0xa83c;
pub const CS48L32_EQ1_BAND4_COEFF2: c_uint = 0xa840;
pub const CS48L32_EQ1_BAND4_PG: c_uint = 0xa844;
pub const CS48L32_EQ1_BAND5_COEFF1: c_uint = 0xa848;
pub const CS48L32_EQ1_BAND5_PG: c_uint = 0xa850;
pub const CS48L32_EQ2_GAIN1: c_uint = 0xa854;
pub const CS48L32_EQ2_GAIN2: c_uint = 0xa858;
pub const CS48L32_EQ2_BAND1_COEFF1: c_uint = 0xa85c;
pub const CS48L32_EQ2_BAND1_COEFF2: c_uint = 0xa860;
pub const CS48L32_EQ2_BAND1_PG: c_uint = 0xa864;
pub const CS48L32_EQ2_BAND2_COEFF1: c_uint = 0xa868;
pub const CS48L32_EQ2_BAND2_COEFF2: c_uint = 0xa86c;
pub const CS48L32_EQ2_BAND2_PG: c_uint = 0xa870;
pub const CS48L32_EQ2_BAND3_COEFF1: c_uint = 0xa874;
pub const CS48L32_EQ2_BAND3_COEFF2: c_uint = 0xa878;
pub const CS48L32_EQ2_BAND3_PG: c_uint = 0xa87c;
pub const CS48L32_EQ2_BAND4_COEFF1: c_uint = 0xa880;
pub const CS48L32_EQ2_BAND4_COEFF2: c_uint = 0xa884;
pub const CS48L32_EQ2_BAND4_PG: c_uint = 0xa888;
pub const CS48L32_EQ2_BAND5_COEFF1: c_uint = 0xa88c;
pub const CS48L32_EQ2_BAND5_PG: c_uint = 0xa894;
pub const CS48L32_EQ3_GAIN1: c_uint = 0xa898;
pub const CS48L32_EQ3_GAIN2: c_uint = 0xa89c;
pub const CS48L32_EQ3_BAND1_COEFF1: c_uint = 0xa8a0;
pub const CS48L32_EQ3_BAND1_COEFF2: c_uint = 0xa8a4;
pub const CS48L32_EQ3_BAND1_PG: c_uint = 0xa8a8;
pub const CS48L32_EQ3_BAND2_COEFF1: c_uint = 0xa8ac;
pub const CS48L32_EQ3_BAND2_COEFF2: c_uint = 0xa8b0;
pub const CS48L32_EQ3_BAND2_PG: c_uint = 0xa8b4;
pub const CS48L32_EQ3_BAND3_COEFF1: c_uint = 0xa8b8;
pub const CS48L32_EQ3_BAND3_COEFF2: c_uint = 0xa8bc;
pub const CS48L32_EQ3_BAND3_PG: c_uint = 0xa8c0;
pub const CS48L32_EQ3_BAND4_COEFF1: c_uint = 0xa8c4;
pub const CS48L32_EQ3_BAND4_COEFF2: c_uint = 0xa8c8;
pub const CS48L32_EQ3_BAND4_PG: c_uint = 0xa8cc;
pub const CS48L32_EQ3_BAND5_COEFF1: c_uint = 0xa8d0;
pub const CS48L32_EQ3_BAND5_PG: c_uint = 0xa8d8;
pub const CS48L32_EQ4_GAIN1: c_uint = 0xa8dc;
pub const CS48L32_EQ4_GAIN2: c_uint = 0xa8e0;
pub const CS48L32_EQ4_BAND1_COEFF1: c_uint = 0xa8e4;
pub const CS48L32_EQ4_BAND1_COEFF2: c_uint = 0xa8e8;
pub const CS48L32_EQ4_BAND1_PG: c_uint = 0xa8ec;
pub const CS48L32_EQ4_BAND2_COEFF1: c_uint = 0xa8f0;
pub const CS48L32_EQ4_BAND2_COEFF2: c_uint = 0xa8f4;
pub const CS48L32_EQ4_BAND2_PG: c_uint = 0xa8f8;
pub const CS48L32_EQ4_BAND3_COEFF1: c_uint = 0xa8fc;
pub const CS48L32_EQ4_BAND3_COEFF2: c_uint = 0xa900;
pub const CS48L32_EQ4_BAND3_PG: c_uint = 0xa904;
pub const CS48L32_EQ4_BAND4_COEFF1: c_uint = 0xa908;
pub const CS48L32_EQ4_BAND4_COEFF2: c_uint = 0xa90c;
pub const CS48L32_EQ4_BAND4_PG: c_uint = 0xa910;
pub const CS48L32_EQ4_BAND5_COEFF1: c_uint = 0xa914;
pub const CS48L32_EQ4_BAND5_PG: c_uint = 0xa91c;
pub const CS48L32_LHPF_CONTROL1: c_uint = 0xaa30;
pub const CS48L32_LHPF_CONTROL2: c_uint = 0xaa34;
pub const CS48L32_LHPF1_COEFF: c_uint = 0xaa38;
pub const CS48L32_LHPF2_COEFF: c_uint = 0xaa3c;
pub const CS48L32_LHPF3_COEFF: c_uint = 0xaa40;
pub const CS48L32_LHPF4_COEFF: c_uint = 0xaa44;
pub const CS48L32_DRC1_CONTROL1: c_uint = 0xab00;
pub const CS48L32_DRC1_CONTROL4: c_uint = 0xab0c;
pub const CS48L32_DRC2_CONTROL1: c_uint = 0xab14;
pub const CS48L32_DRC2_CONTROL4: c_uint = 0xab20;
pub const CS48L32_TONE_GENERATOR1: c_uint = 0xb000;
pub const CS48L32_TONE_GENERATOR2: c_uint = 0xb004;
pub const CS48L32_COMFORT_NOISE_GENERATOR: c_uint = 0xb400;
pub const CS48L32_US_CONTROL: c_uint = 0xb800;
pub const CS48L32_US1_CONTROL: c_uint = 0xb804;
pub const CS48L32_US1_DET_CONTROL: c_uint = 0xb808;
pub const CS48L32_US2_CONTROL: c_uint = 0xb814;
pub const CS48L32_US2_DET_CONTROL: c_uint = 0xb818;
pub const CS48L32_DSP1_XM_SRAM_IBUS_SETUP_0: c_uint = 0x1700c;
pub const CS48L32_DSP1_XM_SRAM_IBUS_SETUP_1: c_uint = 0x17010;
pub const CS48L32_DSP1_XM_SRAM_IBUS_SETUP_24: c_uint = 0x1706c;
pub const CS48L32_DSP1_YM_SRAM_IBUS_SETUP_0: c_uint = 0x17070;
pub const CS48L32_DSP1_YM_SRAM_IBUS_SETUP_1: c_uint = 0x17074;
pub const CS48L32_DSP1_YM_SRAM_IBUS_SETUP_8: c_uint = 0x17090;
pub const CS48L32_DSP1_PM_SRAM_IBUS_SETUP_0: c_uint = 0x17094;
pub const CS48L32_DSP1_PM_SRAM_IBUS_SETUP_1: c_uint = 0x17098;
pub const CS48L32_DSP1_PM_SRAM_IBUS_SETUP_7: c_uint = 0x170b0;
pub const CS48L32_IRQ1_STATUS: c_uint = 0x18004;
pub const CS48L32_IRQ1_EINT_1: c_uint = 0x18010;
pub const CS48L32_IRQ1_EINT_2: c_uint = 0x18014;
pub const CS48L32_IRQ1_EINT_7: c_uint = 0x18028;
pub const CS48L32_IRQ1_EINT_9: c_uint = 0x18030;
pub const CS48L32_IRQ1_EINT_11: c_uint = 0x18038;
pub const CS48L32_IRQ1_STS_1: c_uint = 0x18090;
pub const CS48L32_IRQ1_STS_6: c_uint = 0x180a4;
pub const CS48L32_IRQ1_STS_11: c_uint = 0x180b8;
pub const CS48L32_IRQ1_MASK_1: c_uint = 0x18110;
pub const CS48L32_IRQ1_MASK_2: c_uint = 0x18114;
pub const CS48L32_IRQ1_MASK_7: c_uint = 0x18128;
pub const CS48L32_IRQ1_MASK_9: c_uint = 0x18130;
pub const CS48L32_IRQ1_MASK_11: c_uint = 0x18138;
pub const CS48L32_DSP1_XMEM_PACKED_0: c_uint = 0x2000000;
pub const CS48L32_DSP1_XMEM_PACKED_LAST: c_uint = 0x208fff0;
pub const CS48L32_DSP1_SYS_INFO_ID: c_uint = 0x25e0000;
pub const CS48L32_DSP1_AHBM_WINDOW_DEBUG_1: c_uint = 0x25e2044;
pub const CS48L32_DSP1_XMEM_UNPACKED24_0: c_uint = 0x2800000;
pub const CS48L32_DSP1_XMEM_UNPACKED24_LAST: c_uint = 0x28bfff4;
pub const CS48L32_DSP1_CLOCK_FREQ: c_uint = 0x2b80000;
pub const CS48L32_DSP1_SAMPLE_RATE_TX8: c_uint = 0x2b802b8;
pub const CS48L32_DSP1_SCRATCH1: c_uint = 0x2b805c0;
pub const CS48L32_DSP1_SCRATCH4: c_uint = 0x2b805d8;
pub const CS48L32_DSP1_CCM_CORE_CONTROL: c_uint = 0x2bc1000;
pub const CS48L32_DSP1_STREAM_ARB_RESYNC_MSK1: c_uint = 0x2bc5a00;
pub const CS48L32_DSP1_YMEM_PACKED_0: c_uint = 0x2c00000;
pub const CS48L32_DSP1_YMEM_PACKED_LAST: c_uint = 0x2c2fff0;
pub const CS48L32_DSP1_YMEM_UNPACKED24_0: c_uint = 0x3400000;
pub const CS48L32_DSP1_YMEM_UNPACKED24_LAST: c_uint = 0x343fff4;
pub const CS48L32_DSP1_PMEM_0: c_uint = 0x3800000;
pub const CS48L32_DSP1_PMEM_LAST: c_uint = 0x3845fe8;
// (0x0) DEVID
pub const CS48L32_DEVID_MASK: c_uint = 0x00ffffff;
pub const CS48L32_DEVID_SHIFT: c_int = 0;
// (0x4) REVID
pub const CS48L32_AREVID_MASK: c_uint = 0x000000f0;
pub const CS48L32_AREVID_SHIFT: c_int = 4;
pub const CS48L32_MTLREVID_MASK: c_uint = 0x0000000f;
pub const CS48L32_MTLREVID_SHIFT: c_int = 0;
// (0x10) OTPID
pub const CS48L32_OTPID_MASK: c_uint = 0x0000000f;
// (0x0804) MCU_CTRL1
pub const CS48L32_MCU_STS_MASK: c_uint = 0x0000ff00;
pub const CS48L32_MCU_STS_SHIFT: c_int = 8;
// (0xc08) GPIO1_CTRL1
pub const CS48L32_GPIOX_CTRL1_FN_MASK: c_uint = 0x000003ff;
// (0x1020) OUTPUT_SYS_CLK
pub const CS48L32_OPCLK_EN_SHIFT: c_int = 15;
pub const CS48L32_OPCLK_DIV_MASK: c_uint = 0x000000f8;
pub const CS48L32_OPCLK_DIV_SHIFT: c_int = 3;
pub const CS48L32_OPCLK_SEL_MASK: c_uint = 0x00000007;
// (0x105c) AUXPDM_CTRL2
pub const CS48L32_AUXPDMDAT2_SRC_SHIFT: c_int = 4;
pub const CS48L32_AUXPDMDAT1_SRC_SHIFT: c_int = 0;
// (0x1400) CLOCK32K
pub const CS48L32_CLK_32K_EN_MASK: c_uint = 0x00000040;
pub const CS48L32_CLK_32K_SRC_MASK: c_uint = 0x00000003;
// (0x1404) SYSTEM_CLOCK1
pub const CS48L32_SYSCLK_FRAC_MASK: c_uint = 0x00008000;
pub const CS48L32_SYSCLK_FREQ_MASK: c_uint = 0x00000700;
pub const CS48L32_SYSCLK_FREQ_SHIFT: c_int = 8;
pub const CS48L32_SYSCLK_EN_SHIFT: c_int = 6;
pub const CS48L32_SYSCLK_SRC_MASK: c_uint = 0x0000001f;
pub const CS48L32_SYSCLK_SRC_SHIFT: c_int = 0;
// (0x1408) SYSTEM_CLOCK2
pub const CS48L32_SYSCLK_FREQ_STS_MASK: c_uint = 0x00000700;
pub const CS48L32_SYSCLK_FREQ_STS_SHIFT: c_int = 8;
// (0x1420) SAMPLE_RATE1
pub const CS48L32_SAMPLE_RATE_1_MASK: c_uint = 0x0000001f;
pub const CS48L32_SAMPLE_RATE_1_SHIFT: c_int = 0;
// (0x1510) DSP_CLOCK1
pub const CS48L32_DSP_CLK_FREQ_MASK: c_uint = 0xffff0000;
pub const CS48L32_DSP_CLK_FREQ_SHIFT: c_int = 16;
// (0x1c00) FLL_CONTROL1
pub const CS48L32_FLL_CTRL_UPD_MASK: c_uint = 0x00000004;
pub const CS48L32_FLL_HOLD_MASK: c_uint = 0x00000002;
pub const CS48L32_FLL_EN_MASK: c_uint = 0x00000001;
// (0x1c04) FLL_CONTROL2
pub const CS48L32_FLL_LOCKDET_THR_MASK: c_uint = 0xf0000000;
pub const CS48L32_FLL_LOCKDET_THR_SHIFT: c_int = 28;
pub const CS48L32_FLL_LOCKDET_MASK: c_uint = 0x08000000;
pub const CS48L32_FLL_PHASEDET_MASK: c_uint = 0x00400000;
pub const CS48L32_FLL_PHASEDET_SHIFT: c_int = 22;
pub const CS48L32_FLL_REFCLK_DIV_MASK: c_uint = 0x00030000;
pub const CS48L32_FLL_REFCLK_DIV_SHIFT: c_int = 16;
pub const CS48L32_FLL_REFCLK_SRC_MASK: c_uint = 0x0000f000;
pub const CS48L32_FLL_REFCLK_SRC_SHIFT: c_int = 12;
pub const CS48L32_FLL_N_MASK: c_uint = 0x000003ff;
pub const CS48L32_FLL_N_SHIFT: c_int = 0;
// (0x1c08) FLL_CONTROL3
pub const CS48L32_FLL_LAMBDA_MASK: c_uint = 0xffff0000;
pub const CS48L32_FLL_LAMBDA_SHIFT: c_int = 16;
pub const CS48L32_FLL_THETA_MASK: c_uint = 0x0000ffff;
pub const CS48L32_FLL_THETA_SHIFT: c_int = 0;
// (0x1c0c) FLL_CONTROL4
pub const CS48L32_FLL_FD_GAIN_COARSE_SHIFT: c_int = 16;
pub const CS48L32_FLL_HP_MASK: c_uint = 0x00003000;
pub const CS48L32_FLL_HP_SHIFT: c_int = 12;
pub const CS48L32_FLL_FB_DIV_MASK: c_uint = 0x000003ff;
pub const CS48L32_FLL_FB_DIV_SHIFT: c_int = 0;
// (0x1c10) FLL_CONTROL5
pub const CS48L32_FLL_FRC_INTEG_UPD_MASK: c_uint = 0x00008000;
// (0x2000) CHARGE_PUMP1
pub const CS48L32_CP2_BYPASS_SHIFT: c_int = 1;
pub const CS48L32_CP2_EN_SHIFT: c_int = 0;
// (0x2408) LDO2_CTRL1
pub const CS48L32_LDO2_VSEL_MASK: c_uint = 0x000007e0;
pub const CS48L32_LDO2_VSEL_SHIFT: c_int = 5;
// (0x2410) MICBIAS_CTRL1
pub const CS48L32_MICB1_LVL_MASK: c_uint = 0x000001e0;
pub const CS48L32_MICB1_LVL_SHIFT: c_int = 5;
pub const CS48L32_MICB1_EN_SHIFT: c_int = 0;
// (0x2418) MICBIAS_CTRL5
pub const CS48L32_MICB1C_EN_SHIFT: c_int = 8;
pub const CS48L32_MICB1B_EN_SHIFT: c_int = 4;
pub const CS48L32_MICB1A_EN_SHIFT: c_int = 0;
// (0x2710) IRQ1_CTRL_AOD
pub const CS48L32_IRQ_POL_MASK: c_uint = 0x00000400;
// (0x4000) INPUT_CONTROL
pub const CS48L32_IN2L_EN_SHIFT: c_int = 3;
pub const CS48L32_IN2R_EN_SHIFT: c_int = 2;
pub const CS48L32_IN1L_EN_SHIFT: c_int = 1;
pub const CS48L32_IN1R_EN_SHIFT: c_int = 0;
// (0x400c) INPUT_CONTROL2
pub const CS48L32_PDM_FLLCLK_SRC_MASK: c_uint = 0x0000000f;
pub const CS48L32_PDM_FLLCLK_SRC_SHIFT: c_int = 0;
// (0x4014) INPUT_CONTROL3
pub const CS48L32_IN_VU: c_uint = 0x20000000;
pub const CS48L32_IN_VU_MASK: c_uint = 0x20000000;
pub const CS48L32_IN_VU_SHIFT: c_int = 29;
pub const CS48L32_IN_VU_WIDTH: c_int = 1;
// (0x4020) INPUT1_CONTROL1
pub const CS48L32_IN1_OSR_SHIFT: c_int = 16;
pub const CS48L32_IN1_PDM_SUP_MASK: c_uint = 0x00000300;
pub const CS48L32_IN1_PDM_SUP_SHIFT: c_int = 8;
pub const CS48L32_IN1_MODE_SHIFT: c_int = 0;
//
// (0x4024) IN1L_CONTROL1
// (0x4044) IN1R_CONTROL1
//
pub const CS48L32_INx_SRC_MASK: c_uint = 0x30000000;
pub const CS48L32_INx_SRC_SHIFT: c_int = 28;
pub const CS48L32_INx_RATE_MASK: c_uint = 0x0000f800;
pub const CS48L32_INx_RATE_SHIFT: c_int = 11;
pub const CS48L32_INx_HPF_SHIFT: c_int = 2;
pub const CS48L32_INx_LP_MODE_SHIFT: c_int = 0;
//
// (0x4028) IN1L_CONTROL2
// (0x4048) IN1R_CONTROL2
//
pub const CS48L32_INx_MUTE_MASK: c_uint = 0x10000000;
pub const CS48L32_INx_VOL_SHIFT: c_int = 16;
pub const CS48L32_INx_PGA_VOL_SHIFT: c_int = 1;
// (0x4244) INPUT_HPF_CONTROL
pub const CS48L32_IN_HPF_CUT_SHIFT: c_int = 0;
// (0x4248) INPUT_VOL_CONTROL
pub const CS48L32_IN_VD_RAMP_SHIFT: c_int = 4;
pub const CS48L32_IN_VI_RAMP_SHIFT: c_int = 0;
// (0x4308) AUXPDM1_CONTROL1
pub const CS48L32_AUXPDM1_FREQ_SHIFT: c_int = 16;
pub const CS48L32_AUXPDM1_SRC_MASK: c_uint = 0x00000f00;
pub const CS48L32_AUXPDM1_SRC_SHIFT: c_int = 8;
// (0x4688) ADC1L_ANA_CONTROL1
// (0x468c) ADC1R_ANA_CONTROL1
pub const CS48L32_ADC1x_INT_ENA_FRC_MASK: c_uint = 0x00000002;
// (0x6004) ASPn_CONTROL1
pub const CS48L32_ASP_RATE_MASK: c_uint = 0x00001f00;
pub const CS48L32_ASP_RATE_SHIFT: c_int = 8;
pub const CS48L32_ASP_BCLK_FREQ_MASK: c_uint = 0x0000003f;
// (0x6008) ASPn_CONTROL2
pub const CS48L32_ASP_RX_WIDTH_MASK: c_uint = 0xff000000;
pub const CS48L32_ASP_RX_WIDTH_SHIFT: c_int = 24;
pub const CS48L32_ASP_TX_WIDTH_MASK: c_uint = 0x00ff0000;
pub const CS48L32_ASP_TX_WIDTH_SHIFT: c_int = 16;
pub const CS48L32_ASP_FMT_MASK: c_uint = 0x00000700;
pub const CS48L32_ASP_FMT_SHIFT: c_int = 8;
pub const CS48L32_ASP_BCLK_INV_MASK: c_uint = 0x00000040;
pub const CS48L32_ASP_BCLK_MSTR_MASK: c_uint = 0x00000010;
pub const CS48L32_ASP_FSYNC_INV_MASK: c_uint = 0x00000004;
pub const CS48L32_ASP_FSYNC_MSTR_MASK: c_uint = 0x00000001;
// (0x6010) ASPn_CONTROL3
pub const CS48L32_ASP_DOUT_HIZ_MASK: c_uint = 0x00000003;
// (0x6030) ASPn_DATA_CONTROL1
pub const CS48L32_ASP_TX_WL_MASK: c_uint = 0x0000003f;
// (0x6040) ASPn_DATA_CONTROL5
pub const CS48L32_ASP_RX_WL_MASK: c_uint = 0x0000003f;
// (0x82xx - 0x90xx)  *_INPUT[1-4]
pub const CS48L32_MIXER_VOL_MASK: c_uint = 0x00FE0000;
pub const CS48L32_MIXER_VOL_SHIFT: c_int = 17;
pub const CS48L32_MIXER_VOL_WIDTH: c_int = 7;
pub const CS48L32_MIXER_SRC_MASK: c_uint = 0x000001ff;
pub const CS48L32_MIXER_SRC_SHIFT: c_int = 0;
pub const CS48L32_MIXER_SRC_WIDTH: c_int = 9;
// (0xa400) ISRC1_CONTROL1
pub const CS48L32_ISRC1_FSL_MASK: c_uint = 0xf8000000;
pub const CS48L32_ISRC1_FSL_SHIFT: c_int = 27;
pub const CS48L32_ISRC1_FSH_MASK: c_uint = 0x0000f800;
pub const CS48L32_ISRC1_FSH_SHIFT: c_int = 11;
// (0xa404) ISRC1_CONTROL2
pub const CS48L32_ISRC1_INT4_EN_SHIFT: c_int = 11;
pub const CS48L32_ISRC1_INT3_EN_SHIFT: c_int = 10;
pub const CS48L32_ISRC1_INT2_EN_SHIFT: c_int = 9;
pub const CS48L32_ISRC1_INT1_EN_SHIFT: c_int = 8;
pub const CS48L32_ISRC1_DEC4_EN_SHIFT: c_int = 3;
pub const CS48L32_ISRC1_DEC3_EN_SHIFT: c_int = 2;
pub const CS48L32_ISRC1_DEC2_EN_SHIFT: c_int = 1;
pub const CS48L32_ISRC1_DEC1_EN_SHIFT: c_int = 0;
// (0xa800) FX_SAMPLE_RATE
pub const CS48L32_FX_RATE_MASK: c_uint = 0x0000f800;
pub const CS48L32_FX_RATE_SHIFT: c_int = 11;
// (0xab00) DRC1_CONTROL1
pub const CS48L32_DRC1L_EN_SHIFT: c_int = 1;
pub const CS48L32_DRC1R_EN_SHIFT: c_int = 0;
// (0xb400) Comfort_Noise_Generator
pub const CS48L32_NOISE_GEN_RATE_MASK: c_uint = 0x0000f800;
pub const CS48L32_NOISE_GEN_RATE_SHIFT: c_int = 11;
pub const CS48L32_NOISE_GEN_EN_SHIFT: c_int = 5;
pub const CS48L32_NOISE_GEN_GAIN_SHIFT: c_int = 0;
// (0xb800) US_CONTROL
pub const CS48L32_US1_DET_EN_SHIFT: c_int = 8;
// (0xb804) US1_CONTROL
pub const CS48L32_US1_RATE_MASK: c_uint = 0xf8000000;
pub const CS48L32_US1_RATE_SHIFT: c_int = 27;
pub const CS48L32_US1_GAIN_SHIFT: c_int = 12;
pub const CS48L32_US1_SRC_MASK: c_uint = 0x00000f00;
pub const CS48L32_US1_SRC_SHIFT: c_int = 8;
pub const CS48L32_US1_FREQ_MASK: c_uint = 0x00000070;
pub const CS48L32_US1_FREQ_SHIFT: c_int = 4;
// (0xb808) US1_DET_CONTROL
pub const CS48L32_US1_DET_DCY_SHIFT: c_int = 28;
pub const CS48L32_US1_DET_HOLD_SHIFT: c_int = 24;
pub const CS48L32_US1_DET_NUM_SHIFT: c_int = 20;
pub const CS48L32_US1_DET_THR_SHIFT: c_int = 16;
pub const CS48L32_US1_DET_LPF_CUT_SHIFT: c_int = 5;
pub const CS48L32_US1_DET_LPF_SHIFT: c_int = 4;
// (0x18004) IRQ1_STATUS
pub const CS48L32_IRQ1_STS_MASK: c_uint = 0x00000001;
// (0x18014) IRQ1_EINT_2
pub const CS48L32_BOOT_DONE_EINT1_MASK: c_uint = 0x00000008;
// (0x18028) IRQ1_EINT_7
pub const CS48L32_DSP1_MPU_ERR_EINT1_MASK: c_uint = 0x00200000;
pub const CS48L32_DSP1_WDT_EXPIRE_EINT1_MASK: c_uint = 0x00100000;
// (0x18030) IRQ1_EINT_9
pub const CS48L32_DSP1_IRQ0_EINT1_MASK: c_uint = 0x00000001;
// (0x180a4) IRQ1_STS_6
pub const CS48L32_FLL1_LOCK_STS1_MASK: c_uint = 0x00000001;
