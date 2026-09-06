//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm5100.h
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
// wm5100.h  --  WM5100 ALSA SoC Audio driver
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

extern "C" {
    pub fn wm5100_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int;
}
pub const WM5100_CLK_AIF1: c_int = 1;
pub const WM5100_CLK_AIF2: c_int = 2;
pub const WM5100_CLK_AIF3: c_int = 3;
pub const WM5100_CLK_SYSCLK: c_int = 4;
pub const WM5100_CLK_ASYNCCLK: c_int = 5;
pub const WM5100_CLK_32KHZ: c_int = 6;
pub const WM5100_CLK_OPCLK: c_int = 7;
pub const WM5100_CLKSRC_MCLK1: c_int = 0;
pub const WM5100_CLKSRC_MCLK2: c_int = 1;
pub const WM5100_CLKSRC_SYSCLK: c_int = 2;
pub const WM5100_CLKSRC_FLL1: c_int = 4;
pub const WM5100_CLKSRC_FLL2: c_int = 5;
pub const WM5100_CLKSRC_AIF1BCLK: c_int = 8;
pub const WM5100_CLKSRC_AIF2BCLK: c_int = 9;
pub const WM5100_CLKSRC_AIF3BCLK: c_int = 10;
pub const WM5100_CLKSRC_ASYNCCLK: c_uint = 0x100;
pub const WM5100_FLL1: c_int = 1;
pub const WM5100_FLL2: c_int = 2;
pub const WM5100_FLL_SRC_MCLK1: c_uint = 0x0;
pub const WM5100_FLL_SRC_MCLK2: c_uint = 0x1;
pub const WM5100_FLL_SRC_FLL1: c_uint = 0x4;
pub const WM5100_FLL_SRC_FLL2: c_uint = 0x5;
pub const WM5100_FLL_SRC_AIF1BCLK: c_uint = 0x8;
pub const WM5100_FLL_SRC_AIF2BCLK: c_uint = 0x9;
pub const WM5100_FLL_SRC_AIF3BCLK: c_uint = 0xa;
//
// Register values.
//
pub const WM5100_SOFTWARE_RESET: c_uint = 0x00;
pub const WM5100_DEVICE_REVISION: c_uint = 0x01;
pub const WM5100_CTRL_IF_1: c_uint = 0x10;
pub const WM5100_TONE_GENERATOR_1: c_uint = 0x20;
pub const WM5100_PWM_DRIVE_1: c_uint = 0x30;
pub const WM5100_PWM_DRIVE_2: c_uint = 0x31;
pub const WM5100_PWM_DRIVE_3: c_uint = 0x32;
pub const WM5100_CLOCKING_1: c_uint = 0x100;
pub const WM5100_CLOCKING_3: c_uint = 0x101;
pub const WM5100_CLOCKING_4: c_uint = 0x102;
pub const WM5100_CLOCKING_5: c_uint = 0x103;
pub const WM5100_CLOCKING_6: c_uint = 0x104;
pub const WM5100_CLOCKING_7: c_uint = 0x107;
pub const WM5100_CLOCKING_8: c_uint = 0x108;
pub const WM5100_ASRC_ENABLE: c_uint = 0x120;
pub const WM5100_ASRC_STATUS: c_uint = 0x121;
pub const WM5100_ASRC_RATE1: c_uint = 0x122;
pub const WM5100_ISRC_1_CTRL_1: c_uint = 0x141;
pub const WM5100_ISRC_1_CTRL_2: c_uint = 0x142;
pub const WM5100_ISRC_2_CTRL1: c_uint = 0x143;
pub const WM5100_ISRC_2_CTRL_2: c_uint = 0x144;
pub const WM5100_FLL1_CONTROL_1: c_uint = 0x182;
pub const WM5100_FLL1_CONTROL_2: c_uint = 0x183;
pub const WM5100_FLL1_CONTROL_3: c_uint = 0x184;
pub const WM5100_FLL1_CONTROL_5: c_uint = 0x186;
pub const WM5100_FLL1_CONTROL_6: c_uint = 0x187;
pub const WM5100_FLL1_EFS_1: c_uint = 0x188;
pub const WM5100_FLL2_CONTROL_1: c_uint = 0x1A2;
pub const WM5100_FLL2_CONTROL_2: c_uint = 0x1A3;
pub const WM5100_FLL2_CONTROL_3: c_uint = 0x1A4;
pub const WM5100_FLL2_CONTROL_5: c_uint = 0x1A6;
pub const WM5100_FLL2_CONTROL_6: c_uint = 0x1A7;
pub const WM5100_FLL2_EFS_1: c_uint = 0x1A8;
pub const WM5100_MIC_CHARGE_PUMP_1: c_uint = 0x200;
pub const WM5100_MIC_CHARGE_PUMP_2: c_uint = 0x201;
pub const WM5100_HP_CHARGE_PUMP_1: c_uint = 0x202;
pub const WM5100_LDO1_CONTROL: c_uint = 0x211;
pub const WM5100_MIC_BIAS_CTRL_1: c_uint = 0x215;
pub const WM5100_MIC_BIAS_CTRL_2: c_uint = 0x216;
pub const WM5100_MIC_BIAS_CTRL_3: c_uint = 0x217;
pub const WM5100_ACCESSORY_DETECT_MODE_1: c_uint = 0x280;
pub const WM5100_HEADPHONE_DETECT_1: c_uint = 0x288;
pub const WM5100_HEADPHONE_DETECT_2: c_uint = 0x289;
pub const WM5100_MIC_DETECT_1: c_uint = 0x290;
pub const WM5100_MIC_DETECT_2: c_uint = 0x291;
pub const WM5100_MIC_DETECT_3: c_uint = 0x292;
pub const WM5100_MISC_CONTROL: c_uint = 0x2BB;
pub const WM5100_INPUT_ENABLES: c_uint = 0x301;
pub const WM5100_INPUT_ENABLES_STATUS: c_uint = 0x302;
pub const WM5100_IN1L_CONTROL: c_uint = 0x310;
pub const WM5100_IN1R_CONTROL: c_uint = 0x311;
pub const WM5100_IN2L_CONTROL: c_uint = 0x312;
pub const WM5100_IN2R_CONTROL: c_uint = 0x313;
pub const WM5100_IN3L_CONTROL: c_uint = 0x314;
pub const WM5100_IN3R_CONTROL: c_uint = 0x315;
pub const WM5100_IN4L_CONTROL: c_uint = 0x316;
pub const WM5100_IN4R_CONTROL: c_uint = 0x317;
pub const WM5100_RXANC_SRC: c_uint = 0x318;
pub const WM5100_INPUT_VOLUME_RAMP: c_uint = 0x319;
pub const WM5100_ADC_DIGITAL_VOLUME_1L: c_uint = 0x320;
pub const WM5100_ADC_DIGITAL_VOLUME_1R: c_uint = 0x321;
pub const WM5100_ADC_DIGITAL_VOLUME_2L: c_uint = 0x322;
pub const WM5100_ADC_DIGITAL_VOLUME_2R: c_uint = 0x323;
pub const WM5100_ADC_DIGITAL_VOLUME_3L: c_uint = 0x324;
pub const WM5100_ADC_DIGITAL_VOLUME_3R: c_uint = 0x325;
pub const WM5100_ADC_DIGITAL_VOLUME_4L: c_uint = 0x326;
pub const WM5100_ADC_DIGITAL_VOLUME_4R: c_uint = 0x327;
pub const WM5100_OUTPUT_ENABLES_2: c_uint = 0x401;
pub const WM5100_OUTPUT_STATUS_1: c_uint = 0x402;
pub const WM5100_OUTPUT_STATUS_2: c_uint = 0x403;
pub const WM5100_CHANNEL_ENABLES_1: c_uint = 0x408;
pub const WM5100_OUT_VOLUME_1L: c_uint = 0x410;
pub const WM5100_OUT_VOLUME_1R: c_uint = 0x411;
pub const WM5100_DAC_VOLUME_LIMIT_1L: c_uint = 0x412;
pub const WM5100_DAC_VOLUME_LIMIT_1R: c_uint = 0x413;
pub const WM5100_OUT_VOLUME_2L: c_uint = 0x414;
pub const WM5100_OUT_VOLUME_2R: c_uint = 0x415;
pub const WM5100_DAC_VOLUME_LIMIT_2L: c_uint = 0x416;
pub const WM5100_DAC_VOLUME_LIMIT_2R: c_uint = 0x417;
pub const WM5100_OUT_VOLUME_3L: c_uint = 0x418;
pub const WM5100_OUT_VOLUME_3R: c_uint = 0x419;
pub const WM5100_DAC_VOLUME_LIMIT_3L: c_uint = 0x41A;
pub const WM5100_DAC_VOLUME_LIMIT_3R: c_uint = 0x41B;
pub const WM5100_OUT_VOLUME_4L: c_uint = 0x41C;
pub const WM5100_OUT_VOLUME_4R: c_uint = 0x41D;
pub const WM5100_DAC_VOLUME_LIMIT_5L: c_uint = 0x41E;
pub const WM5100_DAC_VOLUME_LIMIT_5R: c_uint = 0x41F;
pub const WM5100_DAC_VOLUME_LIMIT_6L: c_uint = 0x420;
pub const WM5100_DAC_VOLUME_LIMIT_6R: c_uint = 0x421;
pub const WM5100_DAC_AEC_CONTROL_1: c_uint = 0x440;
pub const WM5100_OUTPUT_VOLUME_RAMP: c_uint = 0x441;
pub const WM5100_DAC_DIGITAL_VOLUME_1L: c_uint = 0x480;
pub const WM5100_DAC_DIGITAL_VOLUME_1R: c_uint = 0x481;
pub const WM5100_DAC_DIGITAL_VOLUME_2L: c_uint = 0x482;
pub const WM5100_DAC_DIGITAL_VOLUME_2R: c_uint = 0x483;
pub const WM5100_DAC_DIGITAL_VOLUME_3L: c_uint = 0x484;
pub const WM5100_DAC_DIGITAL_VOLUME_3R: c_uint = 0x485;
pub const WM5100_DAC_DIGITAL_VOLUME_4L: c_uint = 0x486;
pub const WM5100_DAC_DIGITAL_VOLUME_4R: c_uint = 0x487;
pub const WM5100_DAC_DIGITAL_VOLUME_5L: c_uint = 0x488;
pub const WM5100_DAC_DIGITAL_VOLUME_5R: c_uint = 0x489;
pub const WM5100_DAC_DIGITAL_VOLUME_6L: c_uint = 0x48A;
pub const WM5100_DAC_DIGITAL_VOLUME_6R: c_uint = 0x48B;
pub const WM5100_PDM_SPK1_CTRL_1: c_uint = 0x4C0;
pub const WM5100_PDM_SPK1_CTRL_2: c_uint = 0x4C1;
pub const WM5100_PDM_SPK2_CTRL_1: c_uint = 0x4C2;
pub const WM5100_PDM_SPK2_CTRL_2: c_uint = 0x4C3;
pub const WM5100_AUDIO_IF_1_1: c_uint = 0x500;
pub const WM5100_AUDIO_IF_1_2: c_uint = 0x501;
pub const WM5100_AUDIO_IF_1_3: c_uint = 0x502;
pub const WM5100_AUDIO_IF_1_4: c_uint = 0x503;
pub const WM5100_AUDIO_IF_1_5: c_uint = 0x504;
pub const WM5100_AUDIO_IF_1_6: c_uint = 0x505;
pub const WM5100_AUDIO_IF_1_7: c_uint = 0x506;
pub const WM5100_AUDIO_IF_1_8: c_uint = 0x507;
pub const WM5100_AUDIO_IF_1_9: c_uint = 0x508;
pub const WM5100_AUDIO_IF_1_10: c_uint = 0x509;
pub const WM5100_AUDIO_IF_1_11: c_uint = 0x50A;
pub const WM5100_AUDIO_IF_1_12: c_uint = 0x50B;
pub const WM5100_AUDIO_IF_1_13: c_uint = 0x50C;
pub const WM5100_AUDIO_IF_1_14: c_uint = 0x50D;
pub const WM5100_AUDIO_IF_1_15: c_uint = 0x50E;
pub const WM5100_AUDIO_IF_1_16: c_uint = 0x50F;
pub const WM5100_AUDIO_IF_1_17: c_uint = 0x510;
pub const WM5100_AUDIO_IF_1_18: c_uint = 0x511;
pub const WM5100_AUDIO_IF_1_19: c_uint = 0x512;
pub const WM5100_AUDIO_IF_1_20: c_uint = 0x513;
pub const WM5100_AUDIO_IF_1_21: c_uint = 0x514;
pub const WM5100_AUDIO_IF_1_22: c_uint = 0x515;
pub const WM5100_AUDIO_IF_1_23: c_uint = 0x516;
pub const WM5100_AUDIO_IF_1_24: c_uint = 0x517;
pub const WM5100_AUDIO_IF_1_25: c_uint = 0x518;
pub const WM5100_AUDIO_IF_1_26: c_uint = 0x519;
pub const WM5100_AUDIO_IF_1_27: c_uint = 0x51A;
pub const WM5100_AUDIO_IF_2_1: c_uint = 0x540;
pub const WM5100_AUDIO_IF_2_2: c_uint = 0x541;
pub const WM5100_AUDIO_IF_2_3: c_uint = 0x542;
pub const WM5100_AUDIO_IF_2_4: c_uint = 0x543;
pub const WM5100_AUDIO_IF_2_5: c_uint = 0x544;
pub const WM5100_AUDIO_IF_2_6: c_uint = 0x545;
pub const WM5100_AUDIO_IF_2_7: c_uint = 0x546;
pub const WM5100_AUDIO_IF_2_8: c_uint = 0x547;
pub const WM5100_AUDIO_IF_2_9: c_uint = 0x548;
pub const WM5100_AUDIO_IF_2_10: c_uint = 0x549;
pub const WM5100_AUDIO_IF_2_11: c_uint = 0x54A;
pub const WM5100_AUDIO_IF_2_18: c_uint = 0x551;
pub const WM5100_AUDIO_IF_2_19: c_uint = 0x552;
pub const WM5100_AUDIO_IF_2_26: c_uint = 0x559;
pub const WM5100_AUDIO_IF_2_27: c_uint = 0x55A;
pub const WM5100_AUDIO_IF_3_1: c_uint = 0x580;
pub const WM5100_AUDIO_IF_3_2: c_uint = 0x581;
pub const WM5100_AUDIO_IF_3_3: c_uint = 0x582;
pub const WM5100_AUDIO_IF_3_4: c_uint = 0x583;
pub const WM5100_AUDIO_IF_3_5: c_uint = 0x584;
pub const WM5100_AUDIO_IF_3_6: c_uint = 0x585;
pub const WM5100_AUDIO_IF_3_7: c_uint = 0x586;
pub const WM5100_AUDIO_IF_3_8: c_uint = 0x587;
pub const WM5100_AUDIO_IF_3_9: c_uint = 0x588;
pub const WM5100_AUDIO_IF_3_10: c_uint = 0x589;
pub const WM5100_AUDIO_IF_3_11: c_uint = 0x58A;
pub const WM5100_AUDIO_IF_3_18: c_uint = 0x591;
pub const WM5100_AUDIO_IF_3_19: c_uint = 0x592;
pub const WM5100_AUDIO_IF_3_26: c_uint = 0x599;
pub const WM5100_AUDIO_IF_3_27: c_uint = 0x59A;
pub const WM5100_PWM1MIX_INPUT_1_SOURCE: c_uint = 0x640;
pub const WM5100_PWM1MIX_INPUT_1_VOLUME: c_uint = 0x641;
pub const WM5100_PWM1MIX_INPUT_2_SOURCE: c_uint = 0x642;
pub const WM5100_PWM1MIX_INPUT_2_VOLUME: c_uint = 0x643;
pub const WM5100_PWM1MIX_INPUT_3_SOURCE: c_uint = 0x644;
pub const WM5100_PWM1MIX_INPUT_3_VOLUME: c_uint = 0x645;
pub const WM5100_PWM1MIX_INPUT_4_SOURCE: c_uint = 0x646;
pub const WM5100_PWM1MIX_INPUT_4_VOLUME: c_uint = 0x647;
pub const WM5100_PWM2MIX_INPUT_1_SOURCE: c_uint = 0x648;
pub const WM5100_PWM2MIX_INPUT_1_VOLUME: c_uint = 0x649;
pub const WM5100_PWM2MIX_INPUT_2_SOURCE: c_uint = 0x64A;
pub const WM5100_PWM2MIX_INPUT_2_VOLUME: c_uint = 0x64B;
pub const WM5100_PWM2MIX_INPUT_3_SOURCE: c_uint = 0x64C;
pub const WM5100_PWM2MIX_INPUT_3_VOLUME: c_uint = 0x64D;
pub const WM5100_PWM2MIX_INPUT_4_SOURCE: c_uint = 0x64E;
pub const WM5100_PWM2MIX_INPUT_4_VOLUME: c_uint = 0x64F;
pub const WM5100_OUT1LMIX_INPUT_1_SOURCE: c_uint = 0x680;
pub const WM5100_OUT1LMIX_INPUT_1_VOLUME: c_uint = 0x681;
pub const WM5100_OUT1LMIX_INPUT_2_SOURCE: c_uint = 0x682;
pub const WM5100_OUT1LMIX_INPUT_2_VOLUME: c_uint = 0x683;
pub const WM5100_OUT1LMIX_INPUT_3_SOURCE: c_uint = 0x684;
pub const WM5100_OUT1LMIX_INPUT_3_VOLUME: c_uint = 0x685;
pub const WM5100_OUT1LMIX_INPUT_4_SOURCE: c_uint = 0x686;
pub const WM5100_OUT1LMIX_INPUT_4_VOLUME: c_uint = 0x687;
pub const WM5100_OUT1RMIX_INPUT_1_SOURCE: c_uint = 0x688;
pub const WM5100_OUT1RMIX_INPUT_1_VOLUME: c_uint = 0x689;
pub const WM5100_OUT1RMIX_INPUT_2_SOURCE: c_uint = 0x68A;
pub const WM5100_OUT1RMIX_INPUT_2_VOLUME: c_uint = 0x68B;
pub const WM5100_OUT1RMIX_INPUT_3_SOURCE: c_uint = 0x68C;
pub const WM5100_OUT1RMIX_INPUT_3_VOLUME: c_uint = 0x68D;
pub const WM5100_OUT1RMIX_INPUT_4_SOURCE: c_uint = 0x68E;
pub const WM5100_OUT1RMIX_INPUT_4_VOLUME: c_uint = 0x68F;
pub const WM5100_OUT2LMIX_INPUT_1_SOURCE: c_uint = 0x690;
pub const WM5100_OUT2LMIX_INPUT_1_VOLUME: c_uint = 0x691;
pub const WM5100_OUT2LMIX_INPUT_2_SOURCE: c_uint = 0x692;
pub const WM5100_OUT2LMIX_INPUT_2_VOLUME: c_uint = 0x693;
pub const WM5100_OUT2LMIX_INPUT_3_SOURCE: c_uint = 0x694;
pub const WM5100_OUT2LMIX_INPUT_3_VOLUME: c_uint = 0x695;
pub const WM5100_OUT2LMIX_INPUT_4_SOURCE: c_uint = 0x696;
pub const WM5100_OUT2LMIX_INPUT_4_VOLUME: c_uint = 0x697;
pub const WM5100_OUT2RMIX_INPUT_1_SOURCE: c_uint = 0x698;
pub const WM5100_OUT2RMIX_INPUT_1_VOLUME: c_uint = 0x699;
pub const WM5100_OUT2RMIX_INPUT_2_SOURCE: c_uint = 0x69A;
pub const WM5100_OUT2RMIX_INPUT_2_VOLUME: c_uint = 0x69B;
pub const WM5100_OUT2RMIX_INPUT_3_SOURCE: c_uint = 0x69C;
pub const WM5100_OUT2RMIX_INPUT_3_VOLUME: c_uint = 0x69D;
pub const WM5100_OUT2RMIX_INPUT_4_SOURCE: c_uint = 0x69E;
pub const WM5100_OUT2RMIX_INPUT_4_VOLUME: c_uint = 0x69F;
pub const WM5100_OUT3LMIX_INPUT_1_SOURCE: c_uint = 0x6A0;
pub const WM5100_OUT3LMIX_INPUT_1_VOLUME: c_uint = 0x6A1;
pub const WM5100_OUT3LMIX_INPUT_2_SOURCE: c_uint = 0x6A2;
pub const WM5100_OUT3LMIX_INPUT_2_VOLUME: c_uint = 0x6A3;
pub const WM5100_OUT3LMIX_INPUT_3_SOURCE: c_uint = 0x6A4;
pub const WM5100_OUT3LMIX_INPUT_3_VOLUME: c_uint = 0x6A5;
pub const WM5100_OUT3LMIX_INPUT_4_SOURCE: c_uint = 0x6A6;
pub const WM5100_OUT3LMIX_INPUT_4_VOLUME: c_uint = 0x6A7;
pub const WM5100_OUT3RMIX_INPUT_1_SOURCE: c_uint = 0x6A8;
pub const WM5100_OUT3RMIX_INPUT_1_VOLUME: c_uint = 0x6A9;
pub const WM5100_OUT3RMIX_INPUT_2_SOURCE: c_uint = 0x6AA;
pub const WM5100_OUT3RMIX_INPUT_2_VOLUME: c_uint = 0x6AB;
pub const WM5100_OUT3RMIX_INPUT_3_SOURCE: c_uint = 0x6AC;
pub const WM5100_OUT3RMIX_INPUT_3_VOLUME: c_uint = 0x6AD;
pub const WM5100_OUT3RMIX_INPUT_4_SOURCE: c_uint = 0x6AE;
pub const WM5100_OUT3RMIX_INPUT_4_VOLUME: c_uint = 0x6AF;
pub const WM5100_OUT4LMIX_INPUT_1_SOURCE: c_uint = 0x6B0;
pub const WM5100_OUT4LMIX_INPUT_1_VOLUME: c_uint = 0x6B1;
pub const WM5100_OUT4LMIX_INPUT_2_SOURCE: c_uint = 0x6B2;
pub const WM5100_OUT4LMIX_INPUT_2_VOLUME: c_uint = 0x6B3;
pub const WM5100_OUT4LMIX_INPUT_3_SOURCE: c_uint = 0x6B4;
pub const WM5100_OUT4LMIX_INPUT_3_VOLUME: c_uint = 0x6B5;
pub const WM5100_OUT4LMIX_INPUT_4_SOURCE: c_uint = 0x6B6;
pub const WM5100_OUT4LMIX_INPUT_4_VOLUME: c_uint = 0x6B7;
pub const WM5100_OUT4RMIX_INPUT_1_SOURCE: c_uint = 0x6B8;
pub const WM5100_OUT4RMIX_INPUT_1_VOLUME: c_uint = 0x6B9;
pub const WM5100_OUT4RMIX_INPUT_2_SOURCE: c_uint = 0x6BA;
pub const WM5100_OUT4RMIX_INPUT_2_VOLUME: c_uint = 0x6BB;
pub const WM5100_OUT4RMIX_INPUT_3_SOURCE: c_uint = 0x6BC;
pub const WM5100_OUT4RMIX_INPUT_3_VOLUME: c_uint = 0x6BD;
pub const WM5100_OUT4RMIX_INPUT_4_SOURCE: c_uint = 0x6BE;
pub const WM5100_OUT4RMIX_INPUT_4_VOLUME: c_uint = 0x6BF;
pub const WM5100_OUT5LMIX_INPUT_1_SOURCE: c_uint = 0x6C0;
pub const WM5100_OUT5LMIX_INPUT_1_VOLUME: c_uint = 0x6C1;
pub const WM5100_OUT5LMIX_INPUT_2_SOURCE: c_uint = 0x6C2;
pub const WM5100_OUT5LMIX_INPUT_2_VOLUME: c_uint = 0x6C3;
pub const WM5100_OUT5LMIX_INPUT_3_SOURCE: c_uint = 0x6C4;
pub const WM5100_OUT5LMIX_INPUT_3_VOLUME: c_uint = 0x6C5;
pub const WM5100_OUT5LMIX_INPUT_4_SOURCE: c_uint = 0x6C6;
pub const WM5100_OUT5LMIX_INPUT_4_VOLUME: c_uint = 0x6C7;
pub const WM5100_OUT5RMIX_INPUT_1_SOURCE: c_uint = 0x6C8;
pub const WM5100_OUT5RMIX_INPUT_1_VOLUME: c_uint = 0x6C9;
pub const WM5100_OUT5RMIX_INPUT_2_SOURCE: c_uint = 0x6CA;
pub const WM5100_OUT5RMIX_INPUT_2_VOLUME: c_uint = 0x6CB;
pub const WM5100_OUT5RMIX_INPUT_3_SOURCE: c_uint = 0x6CC;
pub const WM5100_OUT5RMIX_INPUT_3_VOLUME: c_uint = 0x6CD;
pub const WM5100_OUT5RMIX_INPUT_4_SOURCE: c_uint = 0x6CE;
pub const WM5100_OUT5RMIX_INPUT_4_VOLUME: c_uint = 0x6CF;
pub const WM5100_OUT6LMIX_INPUT_1_SOURCE: c_uint = 0x6D0;
pub const WM5100_OUT6LMIX_INPUT_1_VOLUME: c_uint = 0x6D1;
pub const WM5100_OUT6LMIX_INPUT_2_SOURCE: c_uint = 0x6D2;
pub const WM5100_OUT6LMIX_INPUT_2_VOLUME: c_uint = 0x6D3;
pub const WM5100_OUT6LMIX_INPUT_3_SOURCE: c_uint = 0x6D4;
pub const WM5100_OUT6LMIX_INPUT_3_VOLUME: c_uint = 0x6D5;
pub const WM5100_OUT6LMIX_INPUT_4_SOURCE: c_uint = 0x6D6;
pub const WM5100_OUT6LMIX_INPUT_4_VOLUME: c_uint = 0x6D7;
pub const WM5100_OUT6RMIX_INPUT_1_SOURCE: c_uint = 0x6D8;
pub const WM5100_OUT6RMIX_INPUT_1_VOLUME: c_uint = 0x6D9;
pub const WM5100_OUT6RMIX_INPUT_2_SOURCE: c_uint = 0x6DA;
pub const WM5100_OUT6RMIX_INPUT_2_VOLUME: c_uint = 0x6DB;
pub const WM5100_OUT6RMIX_INPUT_3_SOURCE: c_uint = 0x6DC;
pub const WM5100_OUT6RMIX_INPUT_3_VOLUME: c_uint = 0x6DD;
pub const WM5100_OUT6RMIX_INPUT_4_SOURCE: c_uint = 0x6DE;
pub const WM5100_OUT6RMIX_INPUT_4_VOLUME: c_uint = 0x6DF;
pub const WM5100_AIF1TX1MIX_INPUT_1_SOURCE: c_uint = 0x700;
pub const WM5100_AIF1TX1MIX_INPUT_1_VOLUME: c_uint = 0x701;
pub const WM5100_AIF1TX1MIX_INPUT_2_SOURCE: c_uint = 0x702;
pub const WM5100_AIF1TX1MIX_INPUT_2_VOLUME: c_uint = 0x703;
pub const WM5100_AIF1TX1MIX_INPUT_3_SOURCE: c_uint = 0x704;
pub const WM5100_AIF1TX1MIX_INPUT_3_VOLUME: c_uint = 0x705;
pub const WM5100_AIF1TX1MIX_INPUT_4_SOURCE: c_uint = 0x706;
pub const WM5100_AIF1TX1MIX_INPUT_4_VOLUME: c_uint = 0x707;
pub const WM5100_AIF1TX2MIX_INPUT_1_SOURCE: c_uint = 0x708;
pub const WM5100_AIF1TX2MIX_INPUT_1_VOLUME: c_uint = 0x709;
pub const WM5100_AIF1TX2MIX_INPUT_2_SOURCE: c_uint = 0x70A;
pub const WM5100_AIF1TX2MIX_INPUT_2_VOLUME: c_uint = 0x70B;
pub const WM5100_AIF1TX2MIX_INPUT_3_SOURCE: c_uint = 0x70C;
pub const WM5100_AIF1TX2MIX_INPUT_3_VOLUME: c_uint = 0x70D;
pub const WM5100_AIF1TX2MIX_INPUT_4_SOURCE: c_uint = 0x70E;
pub const WM5100_AIF1TX2MIX_INPUT_4_VOLUME: c_uint = 0x70F;
pub const WM5100_AIF1TX3MIX_INPUT_1_SOURCE: c_uint = 0x710;
pub const WM5100_AIF1TX3MIX_INPUT_1_VOLUME: c_uint = 0x711;
pub const WM5100_AIF1TX3MIX_INPUT_2_SOURCE: c_uint = 0x712;
pub const WM5100_AIF1TX3MIX_INPUT_2_VOLUME: c_uint = 0x713;
pub const WM5100_AIF1TX3MIX_INPUT_3_SOURCE: c_uint = 0x714;
pub const WM5100_AIF1TX3MIX_INPUT_3_VOLUME: c_uint = 0x715;
pub const WM5100_AIF1TX3MIX_INPUT_4_SOURCE: c_uint = 0x716;
pub const WM5100_AIF1TX3MIX_INPUT_4_VOLUME: c_uint = 0x717;
pub const WM5100_AIF1TX4MIX_INPUT_1_SOURCE: c_uint = 0x718;
pub const WM5100_AIF1TX4MIX_INPUT_1_VOLUME: c_uint = 0x719;
pub const WM5100_AIF1TX4MIX_INPUT_2_SOURCE: c_uint = 0x71A;
pub const WM5100_AIF1TX4MIX_INPUT_2_VOLUME: c_uint = 0x71B;
pub const WM5100_AIF1TX4MIX_INPUT_3_SOURCE: c_uint = 0x71C;
pub const WM5100_AIF1TX4MIX_INPUT_3_VOLUME: c_uint = 0x71D;
pub const WM5100_AIF1TX4MIX_INPUT_4_SOURCE: c_uint = 0x71E;
pub const WM5100_AIF1TX4MIX_INPUT_4_VOLUME: c_uint = 0x71F;
pub const WM5100_AIF1TX5MIX_INPUT_1_SOURCE: c_uint = 0x720;
pub const WM5100_AIF1TX5MIX_INPUT_1_VOLUME: c_uint = 0x721;
pub const WM5100_AIF1TX5MIX_INPUT_2_SOURCE: c_uint = 0x722;
pub const WM5100_AIF1TX5MIX_INPUT_2_VOLUME: c_uint = 0x723;
pub const WM5100_AIF1TX5MIX_INPUT_3_SOURCE: c_uint = 0x724;
pub const WM5100_AIF1TX5MIX_INPUT_3_VOLUME: c_uint = 0x725;
pub const WM5100_AIF1TX5MIX_INPUT_4_SOURCE: c_uint = 0x726;
pub const WM5100_AIF1TX5MIX_INPUT_4_VOLUME: c_uint = 0x727;
pub const WM5100_AIF1TX6MIX_INPUT_1_SOURCE: c_uint = 0x728;
pub const WM5100_AIF1TX6MIX_INPUT_1_VOLUME: c_uint = 0x729;
pub const WM5100_AIF1TX6MIX_INPUT_2_SOURCE: c_uint = 0x72A;
pub const WM5100_AIF1TX6MIX_INPUT_2_VOLUME: c_uint = 0x72B;
pub const WM5100_AIF1TX6MIX_INPUT_3_SOURCE: c_uint = 0x72C;
pub const WM5100_AIF1TX6MIX_INPUT_3_VOLUME: c_uint = 0x72D;
pub const WM5100_AIF1TX6MIX_INPUT_4_SOURCE: c_uint = 0x72E;
pub const WM5100_AIF1TX6MIX_INPUT_4_VOLUME: c_uint = 0x72F;
pub const WM5100_AIF1TX7MIX_INPUT_1_SOURCE: c_uint = 0x730;
pub const WM5100_AIF1TX7MIX_INPUT_1_VOLUME: c_uint = 0x731;
pub const WM5100_AIF1TX7MIX_INPUT_2_SOURCE: c_uint = 0x732;
pub const WM5100_AIF1TX7MIX_INPUT_2_VOLUME: c_uint = 0x733;
pub const WM5100_AIF1TX7MIX_INPUT_3_SOURCE: c_uint = 0x734;
pub const WM5100_AIF1TX7MIX_INPUT_3_VOLUME: c_uint = 0x735;
pub const WM5100_AIF1TX7MIX_INPUT_4_SOURCE: c_uint = 0x736;
pub const WM5100_AIF1TX7MIX_INPUT_4_VOLUME: c_uint = 0x737;
pub const WM5100_AIF1TX8MIX_INPUT_1_SOURCE: c_uint = 0x738;
pub const WM5100_AIF1TX8MIX_INPUT_1_VOLUME: c_uint = 0x739;
pub const WM5100_AIF1TX8MIX_INPUT_2_SOURCE: c_uint = 0x73A;
pub const WM5100_AIF1TX8MIX_INPUT_2_VOLUME: c_uint = 0x73B;
pub const WM5100_AIF1TX8MIX_INPUT_3_SOURCE: c_uint = 0x73C;
pub const WM5100_AIF1TX8MIX_INPUT_3_VOLUME: c_uint = 0x73D;
pub const WM5100_AIF1TX8MIX_INPUT_4_SOURCE: c_uint = 0x73E;
pub const WM5100_AIF1TX8MIX_INPUT_4_VOLUME: c_uint = 0x73F;
pub const WM5100_AIF2TX1MIX_INPUT_1_SOURCE: c_uint = 0x740;
pub const WM5100_AIF2TX1MIX_INPUT_1_VOLUME: c_uint = 0x741;
pub const WM5100_AIF2TX1MIX_INPUT_2_SOURCE: c_uint = 0x742;
pub const WM5100_AIF2TX1MIX_INPUT_2_VOLUME: c_uint = 0x743;
pub const WM5100_AIF2TX1MIX_INPUT_3_SOURCE: c_uint = 0x744;
pub const WM5100_AIF2TX1MIX_INPUT_3_VOLUME: c_uint = 0x745;
pub const WM5100_AIF2TX1MIX_INPUT_4_SOURCE: c_uint = 0x746;
pub const WM5100_AIF2TX1MIX_INPUT_4_VOLUME: c_uint = 0x747;
pub const WM5100_AIF2TX2MIX_INPUT_1_SOURCE: c_uint = 0x748;
pub const WM5100_AIF2TX2MIX_INPUT_1_VOLUME: c_uint = 0x749;
pub const WM5100_AIF2TX2MIX_INPUT_2_SOURCE: c_uint = 0x74A;
pub const WM5100_AIF2TX2MIX_INPUT_2_VOLUME: c_uint = 0x74B;
pub const WM5100_AIF2TX2MIX_INPUT_3_SOURCE: c_uint = 0x74C;
pub const WM5100_AIF2TX2MIX_INPUT_3_VOLUME: c_uint = 0x74D;
pub const WM5100_AIF2TX2MIX_INPUT_4_SOURCE: c_uint = 0x74E;
pub const WM5100_AIF2TX2MIX_INPUT_4_VOLUME: c_uint = 0x74F;
pub const WM5100_AIF3TX1MIX_INPUT_1_SOURCE: c_uint = 0x780;
pub const WM5100_AIF3TX1MIX_INPUT_1_VOLUME: c_uint = 0x781;
pub const WM5100_AIF3TX1MIX_INPUT_2_SOURCE: c_uint = 0x782;
pub const WM5100_AIF3TX1MIX_INPUT_2_VOLUME: c_uint = 0x783;
pub const WM5100_AIF3TX1MIX_INPUT_3_SOURCE: c_uint = 0x784;
pub const WM5100_AIF3TX1MIX_INPUT_3_VOLUME: c_uint = 0x785;
pub const WM5100_AIF3TX1MIX_INPUT_4_SOURCE: c_uint = 0x786;
pub const WM5100_AIF3TX1MIX_INPUT_4_VOLUME: c_uint = 0x787;
pub const WM5100_AIF3TX2MIX_INPUT_1_SOURCE: c_uint = 0x788;
pub const WM5100_AIF3TX2MIX_INPUT_1_VOLUME: c_uint = 0x789;
pub const WM5100_AIF3TX2MIX_INPUT_2_SOURCE: c_uint = 0x78A;
pub const WM5100_AIF3TX2MIX_INPUT_2_VOLUME: c_uint = 0x78B;
pub const WM5100_AIF3TX2MIX_INPUT_3_SOURCE: c_uint = 0x78C;
pub const WM5100_AIF3TX2MIX_INPUT_3_VOLUME: c_uint = 0x78D;
pub const WM5100_AIF3TX2MIX_INPUT_4_SOURCE: c_uint = 0x78E;
pub const WM5100_AIF3TX2MIX_INPUT_4_VOLUME: c_uint = 0x78F;
pub const WM5100_EQ1MIX_INPUT_1_SOURCE: c_uint = 0x880;
pub const WM5100_EQ1MIX_INPUT_1_VOLUME: c_uint = 0x881;
pub const WM5100_EQ1MIX_INPUT_2_SOURCE: c_uint = 0x882;
pub const WM5100_EQ1MIX_INPUT_2_VOLUME: c_uint = 0x883;
pub const WM5100_EQ1MIX_INPUT_3_SOURCE: c_uint = 0x884;
pub const WM5100_EQ1MIX_INPUT_3_VOLUME: c_uint = 0x885;
pub const WM5100_EQ1MIX_INPUT_4_SOURCE: c_uint = 0x886;
pub const WM5100_EQ1MIX_INPUT_4_VOLUME: c_uint = 0x887;
pub const WM5100_EQ2MIX_INPUT_1_SOURCE: c_uint = 0x888;
pub const WM5100_EQ2MIX_INPUT_1_VOLUME: c_uint = 0x889;
pub const WM5100_EQ2MIX_INPUT_2_SOURCE: c_uint = 0x88A;
pub const WM5100_EQ2MIX_INPUT_2_VOLUME: c_uint = 0x88B;
pub const WM5100_EQ2MIX_INPUT_3_SOURCE: c_uint = 0x88C;
pub const WM5100_EQ2MIX_INPUT_3_VOLUME: c_uint = 0x88D;
pub const WM5100_EQ2MIX_INPUT_4_SOURCE: c_uint = 0x88E;
pub const WM5100_EQ2MIX_INPUT_4_VOLUME: c_uint = 0x88F;
pub const WM5100_EQ3MIX_INPUT_1_SOURCE: c_uint = 0x890;
pub const WM5100_EQ3MIX_INPUT_1_VOLUME: c_uint = 0x891;
pub const WM5100_EQ3MIX_INPUT_2_SOURCE: c_uint = 0x892;
pub const WM5100_EQ3MIX_INPUT_2_VOLUME: c_uint = 0x893;
pub const WM5100_EQ3MIX_INPUT_3_SOURCE: c_uint = 0x894;
pub const WM5100_EQ3MIX_INPUT_3_VOLUME: c_uint = 0x895;
pub const WM5100_EQ3MIX_INPUT_4_SOURCE: c_uint = 0x896;
pub const WM5100_EQ3MIX_INPUT_4_VOLUME: c_uint = 0x897;
pub const WM5100_EQ4MIX_INPUT_1_SOURCE: c_uint = 0x898;
pub const WM5100_EQ4MIX_INPUT_1_VOLUME: c_uint = 0x899;
pub const WM5100_EQ4MIX_INPUT_2_SOURCE: c_uint = 0x89A;
pub const WM5100_EQ4MIX_INPUT_2_VOLUME: c_uint = 0x89B;
pub const WM5100_EQ4MIX_INPUT_3_SOURCE: c_uint = 0x89C;
pub const WM5100_EQ4MIX_INPUT_3_VOLUME: c_uint = 0x89D;
pub const WM5100_EQ4MIX_INPUT_4_SOURCE: c_uint = 0x89E;
pub const WM5100_EQ4MIX_INPUT_4_VOLUME: c_uint = 0x89F;
pub const WM5100_DRC1LMIX_INPUT_1_SOURCE: c_uint = 0x8C0;
pub const WM5100_DRC1LMIX_INPUT_1_VOLUME: c_uint = 0x8C1;
pub const WM5100_DRC1LMIX_INPUT_2_SOURCE: c_uint = 0x8C2;
pub const WM5100_DRC1LMIX_INPUT_2_VOLUME: c_uint = 0x8C3;
pub const WM5100_DRC1LMIX_INPUT_3_SOURCE: c_uint = 0x8C4;
pub const WM5100_DRC1LMIX_INPUT_3_VOLUME: c_uint = 0x8C5;
pub const WM5100_DRC1LMIX_INPUT_4_SOURCE: c_uint = 0x8C6;
pub const WM5100_DRC1LMIX_INPUT_4_VOLUME: c_uint = 0x8C7;
pub const WM5100_DRC1RMIX_INPUT_1_SOURCE: c_uint = 0x8C8;
pub const WM5100_DRC1RMIX_INPUT_1_VOLUME: c_uint = 0x8C9;
pub const WM5100_DRC1RMIX_INPUT_2_SOURCE: c_uint = 0x8CA;
pub const WM5100_DRC1RMIX_INPUT_2_VOLUME: c_uint = 0x8CB;
pub const WM5100_DRC1RMIX_INPUT_3_SOURCE: c_uint = 0x8CC;
pub const WM5100_DRC1RMIX_INPUT_3_VOLUME: c_uint = 0x8CD;
pub const WM5100_DRC1RMIX_INPUT_4_SOURCE: c_uint = 0x8CE;
pub const WM5100_DRC1RMIX_INPUT_4_VOLUME: c_uint = 0x8CF;
pub const WM5100_HPLP1MIX_INPUT_1_SOURCE: c_uint = 0x900;
pub const WM5100_HPLP1MIX_INPUT_1_VOLUME: c_uint = 0x901;
pub const WM5100_HPLP1MIX_INPUT_2_SOURCE: c_uint = 0x902;
pub const WM5100_HPLP1MIX_INPUT_2_VOLUME: c_uint = 0x903;
pub const WM5100_HPLP1MIX_INPUT_3_SOURCE: c_uint = 0x904;
pub const WM5100_HPLP1MIX_INPUT_3_VOLUME: c_uint = 0x905;
pub const WM5100_HPLP1MIX_INPUT_4_SOURCE: c_uint = 0x906;
pub const WM5100_HPLP1MIX_INPUT_4_VOLUME: c_uint = 0x907;
pub const WM5100_HPLP2MIX_INPUT_1_SOURCE: c_uint = 0x908;
pub const WM5100_HPLP2MIX_INPUT_1_VOLUME: c_uint = 0x909;
pub const WM5100_HPLP2MIX_INPUT_2_SOURCE: c_uint = 0x90A;
pub const WM5100_HPLP2MIX_INPUT_2_VOLUME: c_uint = 0x90B;
pub const WM5100_HPLP2MIX_INPUT_3_SOURCE: c_uint = 0x90C;
pub const WM5100_HPLP2MIX_INPUT_3_VOLUME: c_uint = 0x90D;
pub const WM5100_HPLP2MIX_INPUT_4_SOURCE: c_uint = 0x90E;
pub const WM5100_HPLP2MIX_INPUT_4_VOLUME: c_uint = 0x90F;
pub const WM5100_HPLP3MIX_INPUT_1_SOURCE: c_uint = 0x910;
pub const WM5100_HPLP3MIX_INPUT_1_VOLUME: c_uint = 0x911;
pub const WM5100_HPLP3MIX_INPUT_2_SOURCE: c_uint = 0x912;
pub const WM5100_HPLP3MIX_INPUT_2_VOLUME: c_uint = 0x913;
pub const WM5100_HPLP3MIX_INPUT_3_SOURCE: c_uint = 0x914;
pub const WM5100_HPLP3MIX_INPUT_3_VOLUME: c_uint = 0x915;
pub const WM5100_HPLP3MIX_INPUT_4_SOURCE: c_uint = 0x916;
pub const WM5100_HPLP3MIX_INPUT_4_VOLUME: c_uint = 0x917;
pub const WM5100_HPLP4MIX_INPUT_1_SOURCE: c_uint = 0x918;
pub const WM5100_HPLP4MIX_INPUT_1_VOLUME: c_uint = 0x919;
pub const WM5100_HPLP4MIX_INPUT_2_SOURCE: c_uint = 0x91A;
pub const WM5100_HPLP4MIX_INPUT_2_VOLUME: c_uint = 0x91B;
pub const WM5100_HPLP4MIX_INPUT_3_SOURCE: c_uint = 0x91C;
pub const WM5100_HPLP4MIX_INPUT_3_VOLUME: c_uint = 0x91D;
pub const WM5100_HPLP4MIX_INPUT_4_SOURCE: c_uint = 0x91E;
pub const WM5100_HPLP4MIX_INPUT_4_VOLUME: c_uint = 0x91F;
pub const WM5100_DSP1LMIX_INPUT_1_SOURCE: c_uint = 0x940;
pub const WM5100_DSP1LMIX_INPUT_1_VOLUME: c_uint = 0x941;
pub const WM5100_DSP1LMIX_INPUT_2_SOURCE: c_uint = 0x942;
pub const WM5100_DSP1LMIX_INPUT_2_VOLUME: c_uint = 0x943;
pub const WM5100_DSP1LMIX_INPUT_3_SOURCE: c_uint = 0x944;
pub const WM5100_DSP1LMIX_INPUT_3_VOLUME: c_uint = 0x945;
pub const WM5100_DSP1LMIX_INPUT_4_SOURCE: c_uint = 0x946;
pub const WM5100_DSP1LMIX_INPUT_4_VOLUME: c_uint = 0x947;
pub const WM5100_DSP1RMIX_INPUT_1_SOURCE: c_uint = 0x948;
pub const WM5100_DSP1RMIX_INPUT_1_VOLUME: c_uint = 0x949;
pub const WM5100_DSP1RMIX_INPUT_2_SOURCE: c_uint = 0x94A;
pub const WM5100_DSP1RMIX_INPUT_2_VOLUME: c_uint = 0x94B;
pub const WM5100_DSP1RMIX_INPUT_3_SOURCE: c_uint = 0x94C;
pub const WM5100_DSP1RMIX_INPUT_3_VOLUME: c_uint = 0x94D;
pub const WM5100_DSP1RMIX_INPUT_4_SOURCE: c_uint = 0x94E;
pub const WM5100_DSP1RMIX_INPUT_4_VOLUME: c_uint = 0x94F;
pub const WM5100_DSP1AUX1MIX_INPUT_1_SOURCE: c_uint = 0x950;
pub const WM5100_DSP1AUX2MIX_INPUT_1_SOURCE: c_uint = 0x958;
pub const WM5100_DSP1AUX3MIX_INPUT_1_SOURCE: c_uint = 0x960;
pub const WM5100_DSP1AUX4MIX_INPUT_1_SOURCE: c_uint = 0x968;
pub const WM5100_DSP1AUX5MIX_INPUT_1_SOURCE: c_uint = 0x970;
pub const WM5100_DSP1AUX6MIX_INPUT_1_SOURCE: c_uint = 0x978;
pub const WM5100_DSP2LMIX_INPUT_1_SOURCE: c_uint = 0x980;
pub const WM5100_DSP2LMIX_INPUT_1_VOLUME: c_uint = 0x981;
pub const WM5100_DSP2LMIX_INPUT_2_SOURCE: c_uint = 0x982;
pub const WM5100_DSP2LMIX_INPUT_2_VOLUME: c_uint = 0x983;
pub const WM5100_DSP2LMIX_INPUT_3_SOURCE: c_uint = 0x984;
pub const WM5100_DSP2LMIX_INPUT_3_VOLUME: c_uint = 0x985;
pub const WM5100_DSP2LMIX_INPUT_4_SOURCE: c_uint = 0x986;
pub const WM5100_DSP2LMIX_INPUT_4_VOLUME: c_uint = 0x987;
pub const WM5100_DSP2RMIX_INPUT_1_SOURCE: c_uint = 0x988;
pub const WM5100_DSP2RMIX_INPUT_1_VOLUME: c_uint = 0x989;
pub const WM5100_DSP2RMIX_INPUT_2_SOURCE: c_uint = 0x98A;
pub const WM5100_DSP2RMIX_INPUT_2_VOLUME: c_uint = 0x98B;
pub const WM5100_DSP2RMIX_INPUT_3_SOURCE: c_uint = 0x98C;
pub const WM5100_DSP2RMIX_INPUT_3_VOLUME: c_uint = 0x98D;
pub const WM5100_DSP2RMIX_INPUT_4_SOURCE: c_uint = 0x98E;
pub const WM5100_DSP2RMIX_INPUT_4_VOLUME: c_uint = 0x98F;
pub const WM5100_DSP2AUX1MIX_INPUT_1_SOURCE: c_uint = 0x990;
pub const WM5100_DSP2AUX2MIX_INPUT_1_SOURCE: c_uint = 0x998;
pub const WM5100_DSP2AUX3MIX_INPUT_1_SOURCE: c_uint = 0x9A0;
pub const WM5100_DSP2AUX4MIX_INPUT_1_SOURCE: c_uint = 0x9A8;
pub const WM5100_DSP2AUX5MIX_INPUT_1_SOURCE: c_uint = 0x9B0;
pub const WM5100_DSP2AUX6MIX_INPUT_1_SOURCE: c_uint = 0x9B8;
pub const WM5100_DSP3LMIX_INPUT_1_SOURCE: c_uint = 0x9C0;
pub const WM5100_DSP3LMIX_INPUT_1_VOLUME: c_uint = 0x9C1;
pub const WM5100_DSP3LMIX_INPUT_2_SOURCE: c_uint = 0x9C2;
pub const WM5100_DSP3LMIX_INPUT_2_VOLUME: c_uint = 0x9C3;
pub const WM5100_DSP3LMIX_INPUT_3_SOURCE: c_uint = 0x9C4;
pub const WM5100_DSP3LMIX_INPUT_3_VOLUME: c_uint = 0x9C5;
pub const WM5100_DSP3LMIX_INPUT_4_SOURCE: c_uint = 0x9C6;
pub const WM5100_DSP3LMIX_INPUT_4_VOLUME: c_uint = 0x9C7;
pub const WM5100_DSP3RMIX_INPUT_1_SOURCE: c_uint = 0x9C8;
pub const WM5100_DSP3RMIX_INPUT_1_VOLUME: c_uint = 0x9C9;
pub const WM5100_DSP3RMIX_INPUT_2_SOURCE: c_uint = 0x9CA;
pub const WM5100_DSP3RMIX_INPUT_2_VOLUME: c_uint = 0x9CB;
pub const WM5100_DSP3RMIX_INPUT_3_SOURCE: c_uint = 0x9CC;
pub const WM5100_DSP3RMIX_INPUT_3_VOLUME: c_uint = 0x9CD;
pub const WM5100_DSP3RMIX_INPUT_4_SOURCE: c_uint = 0x9CE;
pub const WM5100_DSP3RMIX_INPUT_4_VOLUME: c_uint = 0x9CF;
pub const WM5100_DSP3AUX1MIX_INPUT_1_SOURCE: c_uint = 0x9D0;
pub const WM5100_DSP3AUX2MIX_INPUT_1_SOURCE: c_uint = 0x9D8;
pub const WM5100_DSP3AUX3MIX_INPUT_1_SOURCE: c_uint = 0x9E0;
pub const WM5100_DSP3AUX4MIX_INPUT_1_SOURCE: c_uint = 0x9E8;
pub const WM5100_DSP3AUX5MIX_INPUT_1_SOURCE: c_uint = 0x9F0;
pub const WM5100_DSP3AUX6MIX_INPUT_1_SOURCE: c_uint = 0x9F8;
pub const WM5100_ASRC1LMIX_INPUT_1_SOURCE: c_uint = 0xA80;
pub const WM5100_ASRC1RMIX_INPUT_1_SOURCE: c_uint = 0xA88;
pub const WM5100_ASRC2LMIX_INPUT_1_SOURCE: c_uint = 0xA90;
pub const WM5100_ASRC2RMIX_INPUT_1_SOURCE: c_uint = 0xA98;
pub const WM5100_ISRC1DEC1MIX_INPUT_1_SOURCE: c_uint = 0xB00;
pub const WM5100_ISRC1DEC2MIX_INPUT_1_SOURCE: c_uint = 0xB08;
pub const WM5100_ISRC1DEC3MIX_INPUT_1_SOURCE: c_uint = 0xB10;
pub const WM5100_ISRC1DEC4MIX_INPUT_1_SOURCE: c_uint = 0xB18;
pub const WM5100_ISRC1INT1MIX_INPUT_1_SOURCE: c_uint = 0xB20;
pub const WM5100_ISRC1INT2MIX_INPUT_1_SOURCE: c_uint = 0xB28;
pub const WM5100_ISRC1INT3MIX_INPUT_1_SOURCE: c_uint = 0xB30;
pub const WM5100_ISRC1INT4MIX_INPUT_1_SOURCE: c_uint = 0xB38;
pub const WM5100_ISRC2DEC1MIX_INPUT_1_SOURCE: c_uint = 0xB40;
pub const WM5100_ISRC2DEC2MIX_INPUT_1_SOURCE: c_uint = 0xB48;
pub const WM5100_ISRC2DEC3MIX_INPUT_1_SOURCE: c_uint = 0xB50;
pub const WM5100_ISRC2DEC4MIX_INPUT_1_SOURCE: c_uint = 0xB58;
pub const WM5100_ISRC2INT1MIX_INPUT_1_SOURCE: c_uint = 0xB60;
pub const WM5100_ISRC2INT2MIX_INPUT_1_SOURCE: c_uint = 0xB68;
pub const WM5100_ISRC2INT3MIX_INPUT_1_SOURCE: c_uint = 0xB70;
pub const WM5100_ISRC2INT4MIX_INPUT_1_SOURCE: c_uint = 0xB78;
pub const WM5100_GPIO_CTRL_1: c_uint = 0xC00;
pub const WM5100_GPIO_CTRL_2: c_uint = 0xC01;
pub const WM5100_GPIO_CTRL_3: c_uint = 0xC02;
pub const WM5100_GPIO_CTRL_4: c_uint = 0xC03;
pub const WM5100_GPIO_CTRL_5: c_uint = 0xC04;
pub const WM5100_GPIO_CTRL_6: c_uint = 0xC05;
pub const WM5100_MISC_PAD_CTRL_1: c_uint = 0xC23;
pub const WM5100_MISC_PAD_CTRL_2: c_uint = 0xC24;
pub const WM5100_MISC_PAD_CTRL_3: c_uint = 0xC25;
pub const WM5100_MISC_PAD_CTRL_4: c_uint = 0xC26;
pub const WM5100_MISC_PAD_CTRL_5: c_uint = 0xC27;
pub const WM5100_MISC_GPIO_1: c_uint = 0xC28;
pub const WM5100_INTERRUPT_STATUS_1: c_uint = 0xD00;
pub const WM5100_INTERRUPT_STATUS_2: c_uint = 0xD01;
pub const WM5100_INTERRUPT_STATUS_3: c_uint = 0xD02;
pub const WM5100_INTERRUPT_STATUS_4: c_uint = 0xD03;
pub const WM5100_INTERRUPT_RAW_STATUS_2: c_uint = 0xD04;
pub const WM5100_INTERRUPT_RAW_STATUS_3: c_uint = 0xD05;
pub const WM5100_INTERRUPT_RAW_STATUS_4: c_uint = 0xD06;
pub const WM5100_INTERRUPT_STATUS_1_MASK: c_uint = 0xD07;
pub const WM5100_INTERRUPT_STATUS_2_MASK: c_uint = 0xD08;
pub const WM5100_INTERRUPT_STATUS_3_MASK: c_uint = 0xD09;
pub const WM5100_INTERRUPT_STATUS_4_MASK: c_uint = 0xD0A;
pub const WM5100_INTERRUPT_CONTROL: c_uint = 0xD1F;
pub const WM5100_IRQ_DEBOUNCE_1: c_uint = 0xD20;
pub const WM5100_IRQ_DEBOUNCE_2: c_uint = 0xD21;
pub const WM5100_FX_CTRL: c_uint = 0xE00;
pub const WM5100_EQ1_1: c_uint = 0xE10;
pub const WM5100_EQ1_2: c_uint = 0xE11;
pub const WM5100_EQ1_3: c_uint = 0xE12;
pub const WM5100_EQ1_4: c_uint = 0xE13;
pub const WM5100_EQ1_5: c_uint = 0xE14;
pub const WM5100_EQ1_6: c_uint = 0xE15;
pub const WM5100_EQ1_7: c_uint = 0xE16;
pub const WM5100_EQ1_8: c_uint = 0xE17;
pub const WM5100_EQ1_9: c_uint = 0xE18;
pub const WM5100_EQ1_10: c_uint = 0xE19;
pub const WM5100_EQ1_11: c_uint = 0xE1A;
pub const WM5100_EQ1_12: c_uint = 0xE1B;
pub const WM5100_EQ1_13: c_uint = 0xE1C;
pub const WM5100_EQ1_14: c_uint = 0xE1D;
pub const WM5100_EQ1_15: c_uint = 0xE1E;
pub const WM5100_EQ1_16: c_uint = 0xE1F;
pub const WM5100_EQ1_17: c_uint = 0xE20;
pub const WM5100_EQ1_18: c_uint = 0xE21;
pub const WM5100_EQ1_19: c_uint = 0xE22;
pub const WM5100_EQ1_20: c_uint = 0xE23;
pub const WM5100_EQ2_1: c_uint = 0xE26;
pub const WM5100_EQ2_2: c_uint = 0xE27;
pub const WM5100_EQ2_3: c_uint = 0xE28;
pub const WM5100_EQ2_4: c_uint = 0xE29;
pub const WM5100_EQ2_5: c_uint = 0xE2A;
pub const WM5100_EQ2_6: c_uint = 0xE2B;
pub const WM5100_EQ2_7: c_uint = 0xE2C;
pub const WM5100_EQ2_8: c_uint = 0xE2D;
pub const WM5100_EQ2_9: c_uint = 0xE2E;
pub const WM5100_EQ2_10: c_uint = 0xE2F;
pub const WM5100_EQ2_11: c_uint = 0xE30;
pub const WM5100_EQ2_12: c_uint = 0xE31;
pub const WM5100_EQ2_13: c_uint = 0xE32;
pub const WM5100_EQ2_14: c_uint = 0xE33;
pub const WM5100_EQ2_15: c_uint = 0xE34;
pub const WM5100_EQ2_16: c_uint = 0xE35;
pub const WM5100_EQ2_17: c_uint = 0xE36;
pub const WM5100_EQ2_18: c_uint = 0xE37;
pub const WM5100_EQ2_19: c_uint = 0xE38;
pub const WM5100_EQ2_20: c_uint = 0xE39;
pub const WM5100_EQ3_1: c_uint = 0xE3C;
pub const WM5100_EQ3_2: c_uint = 0xE3D;
pub const WM5100_EQ3_3: c_uint = 0xE3E;
pub const WM5100_EQ3_4: c_uint = 0xE3F;
pub const WM5100_EQ3_5: c_uint = 0xE40;
pub const WM5100_EQ3_6: c_uint = 0xE41;
pub const WM5100_EQ3_7: c_uint = 0xE42;
pub const WM5100_EQ3_8: c_uint = 0xE43;
pub const WM5100_EQ3_9: c_uint = 0xE44;
pub const WM5100_EQ3_10: c_uint = 0xE45;
pub const WM5100_EQ3_11: c_uint = 0xE46;
pub const WM5100_EQ3_12: c_uint = 0xE47;
pub const WM5100_EQ3_13: c_uint = 0xE48;
pub const WM5100_EQ3_14: c_uint = 0xE49;
pub const WM5100_EQ3_15: c_uint = 0xE4A;
pub const WM5100_EQ3_16: c_uint = 0xE4B;
pub const WM5100_EQ3_17: c_uint = 0xE4C;
pub const WM5100_EQ3_18: c_uint = 0xE4D;
pub const WM5100_EQ3_19: c_uint = 0xE4E;
pub const WM5100_EQ3_20: c_uint = 0xE4F;
pub const WM5100_EQ4_1: c_uint = 0xE52;
pub const WM5100_EQ4_2: c_uint = 0xE53;
pub const WM5100_EQ4_3: c_uint = 0xE54;
pub const WM5100_EQ4_4: c_uint = 0xE55;
pub const WM5100_EQ4_5: c_uint = 0xE56;
pub const WM5100_EQ4_6: c_uint = 0xE57;
pub const WM5100_EQ4_7: c_uint = 0xE58;
pub const WM5100_EQ4_8: c_uint = 0xE59;
pub const WM5100_EQ4_9: c_uint = 0xE5A;
pub const WM5100_EQ4_10: c_uint = 0xE5B;
pub const WM5100_EQ4_11: c_uint = 0xE5C;
pub const WM5100_EQ4_12: c_uint = 0xE5D;
pub const WM5100_EQ4_13: c_uint = 0xE5E;
pub const WM5100_EQ4_14: c_uint = 0xE5F;
pub const WM5100_EQ4_15: c_uint = 0xE60;
pub const WM5100_EQ4_16: c_uint = 0xE61;
pub const WM5100_EQ4_17: c_uint = 0xE62;
pub const WM5100_EQ4_18: c_uint = 0xE63;
pub const WM5100_EQ4_19: c_uint = 0xE64;
pub const WM5100_EQ4_20: c_uint = 0xE65;
pub const WM5100_DRC1_CTRL1: c_uint = 0xE80;
pub const WM5100_DRC1_CTRL2: c_uint = 0xE81;
pub const WM5100_DRC1_CTRL3: c_uint = 0xE82;
pub const WM5100_DRC1_CTRL4: c_uint = 0xE83;
pub const WM5100_DRC1_CTRL5: c_uint = 0xE84;
pub const WM5100_HPLPF1_1: c_uint = 0xEC0;
pub const WM5100_HPLPF1_2: c_uint = 0xEC1;
pub const WM5100_HPLPF2_1: c_uint = 0xEC4;
pub const WM5100_HPLPF2_2: c_uint = 0xEC5;
pub const WM5100_HPLPF3_1: c_uint = 0xEC8;
pub const WM5100_HPLPF3_2: c_uint = 0xEC9;
pub const WM5100_HPLPF4_1: c_uint = 0xECC;
pub const WM5100_HPLPF4_2: c_uint = 0xECD;
pub const WM5100_DSP1_CONTROL_1: c_uint = 0xF00;
pub const WM5100_DSP1_CONTROL_2: c_uint = 0xF02;
pub const WM5100_DSP1_CONTROL_3: c_uint = 0xF03;
pub const WM5100_DSP1_CONTROL_4: c_uint = 0xF04;
pub const WM5100_DSP1_CONTROL_5: c_uint = 0xF06;
pub const WM5100_DSP1_CONTROL_6: c_uint = 0xF07;
pub const WM5100_DSP1_CONTROL_7: c_uint = 0xF08;
pub const WM5100_DSP1_CONTROL_8: c_uint = 0xF09;
pub const WM5100_DSP1_CONTROL_9: c_uint = 0xF0A;
pub const WM5100_DSP1_CONTROL_10: c_uint = 0xF0B;
pub const WM5100_DSP1_CONTROL_11: c_uint = 0xF0C;
pub const WM5100_DSP1_CONTROL_12: c_uint = 0xF0D;
pub const WM5100_DSP1_CONTROL_13: c_uint = 0xF0F;
pub const WM5100_DSP1_CONTROL_14: c_uint = 0xF10;
pub const WM5100_DSP1_CONTROL_15: c_uint = 0xF11;
pub const WM5100_DSP1_CONTROL_16: c_uint = 0xF12;
pub const WM5100_DSP1_CONTROL_17: c_uint = 0xF13;
pub const WM5100_DSP1_CONTROL_18: c_uint = 0xF14;
pub const WM5100_DSP1_CONTROL_19: c_uint = 0xF16;
pub const WM5100_DSP1_CONTROL_20: c_uint = 0xF17;
pub const WM5100_DSP1_CONTROL_21: c_uint = 0xF18;
pub const WM5100_DSP1_CONTROL_22: c_uint = 0xF1A;
pub const WM5100_DSP1_CONTROL_23: c_uint = 0xF1B;
pub const WM5100_DSP1_CONTROL_24: c_uint = 0xF1C;
pub const WM5100_DSP1_CONTROL_25: c_uint = 0xF1E;
pub const WM5100_DSP1_CONTROL_26: c_uint = 0xF20;
pub const WM5100_DSP1_CONTROL_27: c_uint = 0xF21;
pub const WM5100_DSP1_CONTROL_28: c_uint = 0xF22;
pub const WM5100_DSP1_CONTROL_29: c_uint = 0xF23;
pub const WM5100_DSP1_CONTROL_30: c_uint = 0xF24;
pub const WM5100_DSP2_CONTROL_1: c_uint = 0x1000;
pub const WM5100_DSP2_CONTROL_2: c_uint = 0x1002;
pub const WM5100_DSP2_CONTROL_3: c_uint = 0x1003;
pub const WM5100_DSP2_CONTROL_4: c_uint = 0x1004;
pub const WM5100_DSP2_CONTROL_5: c_uint = 0x1006;
pub const WM5100_DSP2_CONTROL_6: c_uint = 0x1007;
pub const WM5100_DSP2_CONTROL_7: c_uint = 0x1008;
pub const WM5100_DSP2_CONTROL_8: c_uint = 0x1009;
pub const WM5100_DSP2_CONTROL_9: c_uint = 0x100A;
pub const WM5100_DSP2_CONTROL_10: c_uint = 0x100B;
pub const WM5100_DSP2_CONTROL_11: c_uint = 0x100C;
pub const WM5100_DSP2_CONTROL_12: c_uint = 0x100D;
pub const WM5100_DSP2_CONTROL_13: c_uint = 0x100F;
pub const WM5100_DSP2_CONTROL_14: c_uint = 0x1010;
pub const WM5100_DSP2_CONTROL_15: c_uint = 0x1011;
pub const WM5100_DSP2_CONTROL_16: c_uint = 0x1012;
pub const WM5100_DSP2_CONTROL_17: c_uint = 0x1013;
pub const WM5100_DSP2_CONTROL_18: c_uint = 0x1014;
pub const WM5100_DSP2_CONTROL_19: c_uint = 0x1016;
pub const WM5100_DSP2_CONTROL_20: c_uint = 0x1017;
pub const WM5100_DSP2_CONTROL_21: c_uint = 0x1018;
pub const WM5100_DSP2_CONTROL_22: c_uint = 0x101A;
pub const WM5100_DSP2_CONTROL_23: c_uint = 0x101B;
pub const WM5100_DSP2_CONTROL_24: c_uint = 0x101C;
pub const WM5100_DSP2_CONTROL_25: c_uint = 0x101E;
pub const WM5100_DSP2_CONTROL_26: c_uint = 0x1020;
pub const WM5100_DSP2_CONTROL_27: c_uint = 0x1021;
pub const WM5100_DSP2_CONTROL_28: c_uint = 0x1022;
pub const WM5100_DSP2_CONTROL_29: c_uint = 0x1023;
pub const WM5100_DSP2_CONTROL_30: c_uint = 0x1024;
pub const WM5100_DSP3_CONTROL_1: c_uint = 0x1100;
pub const WM5100_DSP3_CONTROL_2: c_uint = 0x1102;
pub const WM5100_DSP3_CONTROL_3: c_uint = 0x1103;
pub const WM5100_DSP3_CONTROL_4: c_uint = 0x1104;
pub const WM5100_DSP3_CONTROL_5: c_uint = 0x1106;
pub const WM5100_DSP3_CONTROL_6: c_uint = 0x1107;
pub const WM5100_DSP3_CONTROL_7: c_uint = 0x1108;
pub const WM5100_DSP3_CONTROL_8: c_uint = 0x1109;
pub const WM5100_DSP3_CONTROL_9: c_uint = 0x110A;
pub const WM5100_DSP3_CONTROL_10: c_uint = 0x110B;
pub const WM5100_DSP3_CONTROL_11: c_uint = 0x110C;
pub const WM5100_DSP3_CONTROL_12: c_uint = 0x110D;
pub const WM5100_DSP3_CONTROL_13: c_uint = 0x110F;
pub const WM5100_DSP3_CONTROL_14: c_uint = 0x1110;
pub const WM5100_DSP3_CONTROL_15: c_uint = 0x1111;
pub const WM5100_DSP3_CONTROL_16: c_uint = 0x1112;
pub const WM5100_DSP3_CONTROL_17: c_uint = 0x1113;
pub const WM5100_DSP3_CONTROL_18: c_uint = 0x1114;
pub const WM5100_DSP3_CONTROL_19: c_uint = 0x1116;
pub const WM5100_DSP3_CONTROL_20: c_uint = 0x1117;
pub const WM5100_DSP3_CONTROL_21: c_uint = 0x1118;
pub const WM5100_DSP3_CONTROL_22: c_uint = 0x111A;
pub const WM5100_DSP3_CONTROL_23: c_uint = 0x111B;
pub const WM5100_DSP3_CONTROL_24: c_uint = 0x111C;
pub const WM5100_DSP3_CONTROL_25: c_uint = 0x111E;
pub const WM5100_DSP3_CONTROL_26: c_uint = 0x1120;
pub const WM5100_DSP3_CONTROL_27: c_uint = 0x1121;
pub const WM5100_DSP3_CONTROL_28: c_uint = 0x1122;
pub const WM5100_DSP3_CONTROL_29: c_uint = 0x1123;
pub const WM5100_DSP3_CONTROL_30: c_uint = 0x1124;
pub const WM5100_DSP1_DM_0: c_uint = 0x4000;
pub const WM5100_DSP1_DM_1: c_uint = 0x4001;
pub const WM5100_DSP1_DM_2: c_uint = 0x4002;
pub const WM5100_DSP1_DM_3: c_uint = 0x4003;
pub const WM5100_DSP1_DM_508: c_uint = 0x41FC;
pub const WM5100_DSP1_DM_509: c_uint = 0x41FD;
pub const WM5100_DSP1_DM_510: c_uint = 0x41FE;
pub const WM5100_DSP1_DM_511: c_uint = 0x41FF;
pub const WM5100_DSP1_PM_0: c_uint = 0x4800;
pub const WM5100_DSP1_PM_1: c_uint = 0x4801;
pub const WM5100_DSP1_PM_2: c_uint = 0x4802;
pub const WM5100_DSP1_PM_3: c_uint = 0x4803;
pub const WM5100_DSP1_PM_4: c_uint = 0x4804;
pub const WM5100_DSP1_PM_5: c_uint = 0x4805;
pub const WM5100_DSP1_PM_1530: c_uint = 0x4DFA;
pub const WM5100_DSP1_PM_1531: c_uint = 0x4DFB;
pub const WM5100_DSP1_PM_1532: c_uint = 0x4DFC;
pub const WM5100_DSP1_PM_1533: c_uint = 0x4DFD;
pub const WM5100_DSP1_PM_1534: c_uint = 0x4DFE;
pub const WM5100_DSP1_PM_1535: c_uint = 0x4DFF;
pub const WM5100_DSP1_ZM_0: c_uint = 0x5000;
pub const WM5100_DSP1_ZM_1: c_uint = 0x5001;
pub const WM5100_DSP1_ZM_2: c_uint = 0x5002;
pub const WM5100_DSP1_ZM_3: c_uint = 0x5003;
pub const WM5100_DSP1_ZM_2044: c_uint = 0x57FC;
pub const WM5100_DSP1_ZM_2045: c_uint = 0x57FD;
pub const WM5100_DSP1_ZM_2046: c_uint = 0x57FE;
pub const WM5100_DSP1_ZM_2047: c_uint = 0x57FF;
pub const WM5100_DSP2_DM_0: c_uint = 0x6000;
pub const WM5100_DSP2_DM_1: c_uint = 0x6001;
pub const WM5100_DSP2_DM_2: c_uint = 0x6002;
pub const WM5100_DSP2_DM_3: c_uint = 0x6003;
pub const WM5100_DSP2_DM_508: c_uint = 0x61FC;
pub const WM5100_DSP2_DM_509: c_uint = 0x61FD;
pub const WM5100_DSP2_DM_510: c_uint = 0x61FE;
pub const WM5100_DSP2_DM_511: c_uint = 0x61FF;
pub const WM5100_DSP2_PM_0: c_uint = 0x6800;
pub const WM5100_DSP2_PM_1: c_uint = 0x6801;
pub const WM5100_DSP2_PM_2: c_uint = 0x6802;
pub const WM5100_DSP2_PM_3: c_uint = 0x6803;
pub const WM5100_DSP2_PM_4: c_uint = 0x6804;
pub const WM5100_DSP2_PM_5: c_uint = 0x6805;
pub const WM5100_DSP2_PM_1530: c_uint = 0x6DFA;
pub const WM5100_DSP2_PM_1531: c_uint = 0x6DFB;
pub const WM5100_DSP2_PM_1532: c_uint = 0x6DFC;
pub const WM5100_DSP2_PM_1533: c_uint = 0x6DFD;
pub const WM5100_DSP2_PM_1534: c_uint = 0x6DFE;
pub const WM5100_DSP2_PM_1535: c_uint = 0x6DFF;
pub const WM5100_DSP2_ZM_0: c_uint = 0x7000;
pub const WM5100_DSP2_ZM_1: c_uint = 0x7001;
pub const WM5100_DSP2_ZM_2: c_uint = 0x7002;
pub const WM5100_DSP2_ZM_3: c_uint = 0x7003;
pub const WM5100_DSP2_ZM_2044: c_uint = 0x77FC;
pub const WM5100_DSP2_ZM_2045: c_uint = 0x77FD;
pub const WM5100_DSP2_ZM_2046: c_uint = 0x77FE;
pub const WM5100_DSP2_ZM_2047: c_uint = 0x77FF;
pub const WM5100_DSP3_DM_0: c_uint = 0x8000;
pub const WM5100_DSP3_DM_1: c_uint = 0x8001;
pub const WM5100_DSP3_DM_2: c_uint = 0x8002;
pub const WM5100_DSP3_DM_3: c_uint = 0x8003;
pub const WM5100_DSP3_DM_508: c_uint = 0x81FC;
pub const WM5100_DSP3_DM_509: c_uint = 0x81FD;
pub const WM5100_DSP3_DM_510: c_uint = 0x81FE;
pub const WM5100_DSP3_DM_511: c_uint = 0x81FF;
pub const WM5100_DSP3_PM_0: c_uint = 0x8800;
pub const WM5100_DSP3_PM_1: c_uint = 0x8801;
pub const WM5100_DSP3_PM_2: c_uint = 0x8802;
pub const WM5100_DSP3_PM_3: c_uint = 0x8803;
pub const WM5100_DSP3_PM_4: c_uint = 0x8804;
pub const WM5100_DSP3_PM_5: c_uint = 0x8805;
pub const WM5100_DSP3_PM_1530: c_uint = 0x8DFA;
pub const WM5100_DSP3_PM_1531: c_uint = 0x8DFB;
pub const WM5100_DSP3_PM_1532: c_uint = 0x8DFC;
pub const WM5100_DSP3_PM_1533: c_uint = 0x8DFD;
pub const WM5100_DSP3_PM_1534: c_uint = 0x8DFE;
pub const WM5100_DSP3_PM_1535: c_uint = 0x8DFF;
pub const WM5100_DSP3_ZM_0: c_uint = 0x9000;
pub const WM5100_DSP3_ZM_1: c_uint = 0x9001;
pub const WM5100_DSP3_ZM_2: c_uint = 0x9002;
pub const WM5100_DSP3_ZM_3: c_uint = 0x9003;
pub const WM5100_DSP3_ZM_2044: c_uint = 0x97FC;
pub const WM5100_DSP3_ZM_2045: c_uint = 0x97FD;
pub const WM5100_DSP3_ZM_2046: c_uint = 0x97FE;
pub const WM5100_DSP3_ZM_2047: c_uint = 0x97FF;
pub const WM5100_REGISTER_COUNT: c_int = 1435;
pub const WM5100_MAX_REGISTER: c_uint = 0x97FF;
//
// Field Definitions.
//
// R0 (0x00) - software reset
//
pub const WM5100_SW_RST_DEV_ID1_MASK: c_uint = 0xFFFF  /* SW_RST_DEV_ID1 - [15:0] */;

//
// R1 (0x01) - Device Revision
//
pub const WM5100_DEVICE_REVISION_MASK: c_uint = 0x000F  /* DEVICE_REVISION - [3:0] */;

//
// R16 (0x10) - Ctrl IF 1
//
pub const WM5100_AUTO_INC: c_uint = 0x0001  /* AUTO_INC */;
pub const WM5100_AUTO_INC_MASK: c_uint = 0x0001  /* AUTO_INC */;

//
// R32 (0x20) - Tone Generator 1
//
pub const WM5100_TONE_RATE_MASK: c_uint = 0x3000  /* TONE_RATE - [13:12] */;

pub const WM5100_TONE_OFFSET_MASK: c_uint = 0x0300  /* TONE_OFFSET - [9:8] */;

pub const WM5100_TONE2_ENA: c_uint = 0x0002  /* TONE2_ENA */;
pub const WM5100_TONE2_ENA_MASK: c_uint = 0x0002  /* TONE2_ENA */;

pub const WM5100_TONE1_ENA: c_uint = 0x0001  /* TONE1_ENA */;
pub const WM5100_TONE1_ENA_MASK: c_uint = 0x0001  /* TONE1_ENA */;

//
// R48 (0x30) - PWM Drive 1
//
pub const WM5100_PWM_RATE_MASK: c_uint = 0x3000  /* PWM_RATE - [13:12] */;

pub const WM5100_PWM_CLK_SEL_MASK: c_uint = 0x0300  /* PWM_CLK_SEL - [9:8] */;

pub const WM5100_PWM2_OVD: c_uint = 0x0020  /* PWM2_OVD */;
pub const WM5100_PWM2_OVD_MASK: c_uint = 0x0020  /* PWM2_OVD */;

pub const WM5100_PWM1_OVD: c_uint = 0x0010  /* PWM1_OVD */;
pub const WM5100_PWM1_OVD_MASK: c_uint = 0x0010  /* PWM1_OVD */;

pub const WM5100_PWM2_ENA: c_uint = 0x0002  /* PWM2_ENA */;
pub const WM5100_PWM2_ENA_MASK: c_uint = 0x0002  /* PWM2_ENA */;

pub const WM5100_PWM1_ENA: c_uint = 0x0001  /* PWM1_ENA */;
pub const WM5100_PWM1_ENA_MASK: c_uint = 0x0001  /* PWM1_ENA */;

//
// R49 (0x31) - PWM Drive 2
//
pub const WM5100_PWM1_LVL_MASK: c_uint = 0x03FF  /* PWM1_LVL - [9:0] */;

//
// R50 (0x32) - PWM Drive 3
//
pub const WM5100_PWM2_LVL_MASK: c_uint = 0x03FF  /* PWM2_LVL - [9:0] */;

//
// R256 (0x100) - Clocking 1
//
pub const WM5100_CLK_32K_SRC_MASK: c_uint = 0x000F  /* CLK_32K_SRC - [3:0] */;

//
// R257 (0x101) - Clocking 3
//
pub const WM5100_SYSCLK_FREQ_MASK: c_uint = 0x0700  /* SYSCLK_FREQ - [10:8] */;

pub const WM5100_SYSCLK_ENA: c_uint = 0x0040  /* SYSCLK_ENA */;
pub const WM5100_SYSCLK_ENA_MASK: c_uint = 0x0040  /* SYSCLK_ENA */;

pub const WM5100_SYSCLK_SRC_MASK: c_uint = 0x000F  /* SYSCLK_SRC - [3:0] */;

//
// R258 (0x102) - Clocking 4
//
pub const WM5100_SAMPLE_RATE_1_MASK: c_uint = 0x001F  /* SAMPLE_RATE_1 - [4:0] */;

//
// R259 (0x103) - Clocking 5
//
pub const WM5100_SAMPLE_RATE_2_MASK: c_uint = 0x001F  /* SAMPLE_RATE_2 - [4:0] */;

//
// R260 (0x104) - Clocking 6
//
pub const WM5100_SAMPLE_RATE_3_MASK: c_uint = 0x001F  /* SAMPLE_RATE_3 - [4:0] */;

//
// R263 (0x107) - Clocking 7
//
pub const WM5100_ASYNC_CLK_FREQ_MASK: c_uint = 0x0700  /* ASYNC_CLK_FREQ - [10:8] */;

pub const WM5100_ASYNC_CLK_ENA: c_uint = 0x0040  /* ASYNC_CLK_ENA */;
pub const WM5100_ASYNC_CLK_ENA_MASK: c_uint = 0x0040  /* ASYNC_CLK_ENA */;

pub const WM5100_ASYNC_CLK_SRC_MASK: c_uint = 0x000F  /* ASYNC_CLK_SRC - [3:0] */;

//
// R264 (0x108) - Clocking 8
//
pub const WM5100_ASYNC_SAMPLE_RATE_MASK: c_uint = 0x001F  /* ASYNC_SAMPLE_RATE - [4:0] */;

//
// R288 (0x120) - ASRC_ENABLE
//
pub const WM5100_ASRC2L_ENA: c_uint = 0x0008  /* ASRC2L_ENA */;
pub const WM5100_ASRC2L_ENA_MASK: c_uint = 0x0008  /* ASRC2L_ENA */;

pub const WM5100_ASRC2R_ENA: c_uint = 0x0004  /* ASRC2R_ENA */;
pub const WM5100_ASRC2R_ENA_MASK: c_uint = 0x0004  /* ASRC2R_ENA */;

pub const WM5100_ASRC1L_ENA: c_uint = 0x0002  /* ASRC1L_ENA */;
pub const WM5100_ASRC1L_ENA_MASK: c_uint = 0x0002  /* ASRC1L_ENA */;

pub const WM5100_ASRC1R_ENA: c_uint = 0x0001  /* ASRC1R_ENA */;
pub const WM5100_ASRC1R_ENA_MASK: c_uint = 0x0001  /* ASRC1R_ENA */;

//
// R289 (0x121) - ASRC_STATUS
//
pub const WM5100_ASRC2L_ENA_STS: c_uint = 0x0008  /* ASRC2L_ENA_STS */;
pub const WM5100_ASRC2L_ENA_STS_MASK: c_uint = 0x0008  /* ASRC2L_ENA_STS */;

pub const WM5100_ASRC2R_ENA_STS: c_uint = 0x0004  /* ASRC2R_ENA_STS */;
pub const WM5100_ASRC2R_ENA_STS_MASK: c_uint = 0x0004  /* ASRC2R_ENA_STS */;

pub const WM5100_ASRC1L_ENA_STS: c_uint = 0x0002  /* ASRC1L_ENA_STS */;
pub const WM5100_ASRC1L_ENA_STS_MASK: c_uint = 0x0002  /* ASRC1L_ENA_STS */;

pub const WM5100_ASRC1R_ENA_STS: c_uint = 0x0001  /* ASRC1R_ENA_STS */;
pub const WM5100_ASRC1R_ENA_STS_MASK: c_uint = 0x0001  /* ASRC1R_ENA_STS */;

//
// R290 (0x122) - ASRC_RATE1
//
pub const WM5100_ASRC_RATE1_MASK: c_uint = 0x0006  /* ASRC_RATE1 - [2:1] */;

//
// R321 (0x141) - ISRC 1 CTRL 1
//
pub const WM5100_ISRC1_DFS_ENA: c_uint = 0x2000  /* ISRC1_DFS_ENA */;
pub const WM5100_ISRC1_DFS_ENA_MASK: c_uint = 0x2000  /* ISRC1_DFS_ENA */;

pub const WM5100_ISRC1_CLK_SEL_MASK: c_uint = 0x0300  /* ISRC1_CLK_SEL - [9:8] */;

pub const WM5100_ISRC1_FSH_MASK: c_uint = 0x000C  /* ISRC1_FSH - [3:2] */;

pub const WM5100_ISRC1_FSL_MASK: c_uint = 0x0003  /* ISRC1_FSL - [1:0] */;

//
// R322 (0x142) - ISRC 1 CTRL 2
//
pub const WM5100_ISRC1_INT1_ENA: c_uint = 0x8000  /* ISRC1_INT1_ENA */;
pub const WM5100_ISRC1_INT1_ENA_MASK: c_uint = 0x8000  /* ISRC1_INT1_ENA */;

pub const WM5100_ISRC1_INT2_ENA: c_uint = 0x4000  /* ISRC1_INT2_ENA */;
pub const WM5100_ISRC1_INT2_ENA_MASK: c_uint = 0x4000  /* ISRC1_INT2_ENA */;

pub const WM5100_ISRC1_INT3_ENA: c_uint = 0x2000  /* ISRC1_INT3_ENA */;
pub const WM5100_ISRC1_INT3_ENA_MASK: c_uint = 0x2000  /* ISRC1_INT3_ENA */;

pub const WM5100_ISRC1_INT4_ENA: c_uint = 0x1000  /* ISRC1_INT4_ENA */;
pub const WM5100_ISRC1_INT4_ENA_MASK: c_uint = 0x1000  /* ISRC1_INT4_ENA */;

pub const WM5100_ISRC1_DEC1_ENA: c_uint = 0x0200  /* ISRC1_DEC1_ENA */;
pub const WM5100_ISRC1_DEC1_ENA_MASK: c_uint = 0x0200  /* ISRC1_DEC1_ENA */;

pub const WM5100_ISRC1_DEC2_ENA: c_uint = 0x0100  /* ISRC1_DEC2_ENA */;
pub const WM5100_ISRC1_DEC2_ENA_MASK: c_uint = 0x0100  /* ISRC1_DEC2_ENA */;

pub const WM5100_ISRC1_DEC3_ENA: c_uint = 0x0080  /* ISRC1_DEC3_ENA */;
pub const WM5100_ISRC1_DEC3_ENA_MASK: c_uint = 0x0080  /* ISRC1_DEC3_ENA */;

pub const WM5100_ISRC1_DEC4_ENA: c_uint = 0x0040  /* ISRC1_DEC4_ENA */;
pub const WM5100_ISRC1_DEC4_ENA_MASK: c_uint = 0x0040  /* ISRC1_DEC4_ENA */;

pub const WM5100_ISRC1_NOTCH_ENA: c_uint = 0x0001  /* ISRC1_NOTCH_ENA */;
pub const WM5100_ISRC1_NOTCH_ENA_MASK: c_uint = 0x0001  /* ISRC1_NOTCH_ENA */;

//
// R323 (0x143) - ISRC 2 CTRL1
//
pub const WM5100_ISRC2_DFS_ENA: c_uint = 0x2000  /* ISRC2_DFS_ENA */;
pub const WM5100_ISRC2_DFS_ENA_MASK: c_uint = 0x2000  /* ISRC2_DFS_ENA */;

pub const WM5100_ISRC2_CLK_SEL_MASK: c_uint = 0x0300  /* ISRC2_CLK_SEL - [9:8] */;

pub const WM5100_ISRC2_FSH_MASK: c_uint = 0x000C  /* ISRC2_FSH - [3:2] */;

pub const WM5100_ISRC2_FSL_MASK: c_uint = 0x0003  /* ISRC2_FSL - [1:0] */;

//
// R324 (0x144) - ISRC 2 CTRL 2
//
pub const WM5100_ISRC2_INT1_ENA: c_uint = 0x8000  /* ISRC2_INT1_ENA */;
pub const WM5100_ISRC2_INT1_ENA_MASK: c_uint = 0x8000  /* ISRC2_INT1_ENA */;

pub const WM5100_ISRC2_INT2_ENA: c_uint = 0x4000  /* ISRC2_INT2_ENA */;
pub const WM5100_ISRC2_INT2_ENA_MASK: c_uint = 0x4000  /* ISRC2_INT2_ENA */;

pub const WM5100_ISRC2_INT3_ENA: c_uint = 0x2000  /* ISRC2_INT3_ENA */;
pub const WM5100_ISRC2_INT3_ENA_MASK: c_uint = 0x2000  /* ISRC2_INT3_ENA */;

pub const WM5100_ISRC2_INT4_ENA: c_uint = 0x1000  /* ISRC2_INT4_ENA */;
pub const WM5100_ISRC2_INT4_ENA_MASK: c_uint = 0x1000  /* ISRC2_INT4_ENA */;

pub const WM5100_ISRC2_DEC1_ENA: c_uint = 0x0200  /* ISRC2_DEC1_ENA */;
pub const WM5100_ISRC2_DEC1_ENA_MASK: c_uint = 0x0200  /* ISRC2_DEC1_ENA */;

pub const WM5100_ISRC2_DEC2_ENA: c_uint = 0x0100  /* ISRC2_DEC2_ENA */;
pub const WM5100_ISRC2_DEC2_ENA_MASK: c_uint = 0x0100  /* ISRC2_DEC2_ENA */;

pub const WM5100_ISRC2_DEC3_ENA: c_uint = 0x0080  /* ISRC2_DEC3_ENA */;
pub const WM5100_ISRC2_DEC3_ENA_MASK: c_uint = 0x0080  /* ISRC2_DEC3_ENA */;

pub const WM5100_ISRC2_DEC4_ENA: c_uint = 0x0040  /* ISRC2_DEC4_ENA */;
pub const WM5100_ISRC2_DEC4_ENA_MASK: c_uint = 0x0040  /* ISRC2_DEC4_ENA */;

pub const WM5100_ISRC2_NOTCH_ENA: c_uint = 0x0001  /* ISRC2_NOTCH_ENA */;
pub const WM5100_ISRC2_NOTCH_ENA_MASK: c_uint = 0x0001  /* ISRC2_NOTCH_ENA */;

//
// R386 (0x182) - FLL1 Control 1
//
pub const WM5100_FLL1_ENA: c_uint = 0x0001  /* FLL1_ENA */;
pub const WM5100_FLL1_ENA_MASK: c_uint = 0x0001  /* FLL1_ENA */;

//
// R387 (0x183) - FLL1 Control 2
//
pub const WM5100_FLL1_OUTDIV_MASK: c_uint = 0x3F00  /* FLL1_OUTDIV - [13:8] */;

pub const WM5100_FLL1_FRATIO_MASK: c_uint = 0x0007  /* FLL1_FRATIO - [2:0] */;

//
// R388 (0x184) - FLL1 Control 3
//
pub const WM5100_FLL1_THETA_MASK: c_uint = 0xFFFF  /* FLL1_THETA - [15:0] */;

//
// R390 (0x186) - FLL1 Control 5
//
pub const WM5100_FLL1_N_MASK: c_uint = 0x03FF  /* FLL1_N - [9:0] */;

//
// R391 (0x187) - FLL1 Control 6
//
pub const WM5100_FLL1_REFCLK_DIV_MASK: c_uint = 0x00C0  /* FLL1_REFCLK_DIV - [7:6] */;

pub const WM5100_FLL1_REFCLK_SRC_MASK: c_uint = 0x000F  /* FLL1_REFCLK_SRC - [3:0] */;

//
// R392 (0x188) - FLL1 EFS 1
//
pub const WM5100_FLL1_LAMBDA_MASK: c_uint = 0xFFFF  /* FLL1_LAMBDA - [15:0] */;

//
// R418 (0x1A2) - FLL2 Control 1
//
pub const WM5100_FLL2_ENA: c_uint = 0x0001  /* FLL2_ENA */;
pub const WM5100_FLL2_ENA_MASK: c_uint = 0x0001  /* FLL2_ENA */;

//
// R419 (0x1A3) - FLL2 Control 2
//
pub const WM5100_FLL2_OUTDIV_MASK: c_uint = 0x3F00  /* FLL2_OUTDIV - [13:8] */;

pub const WM5100_FLL2_FRATIO_MASK: c_uint = 0x0007  /* FLL2_FRATIO - [2:0] */;

//
// R420 (0x1A4) - FLL2 Control 3
//
pub const WM5100_FLL2_THETA_MASK: c_uint = 0xFFFF  /* FLL2_THETA - [15:0] */;

//
// R422 (0x1A6) - FLL2 Control 5
//
pub const WM5100_FLL2_N_MASK: c_uint = 0x03FF  /* FLL2_N - [9:0] */;

//
// R423 (0x1A7) - FLL2 Control 6
//
pub const WM5100_FLL2_REFCLK_DIV_MASK: c_uint = 0x00C0  /* FLL2_REFCLK_DIV - [7:6] */;

pub const WM5100_FLL2_REFCLK_SRC_MASK: c_uint = 0x000F  /* FLL2_REFCLK_SRC - [3:0] */;

//
// R424 (0x1A8) - FLL2 EFS 1
//
pub const WM5100_FLL2_LAMBDA_MASK: c_uint = 0xFFFF  /* FLL2_LAMBDA - [15:0] */;

//
// R512 (0x200) - Mic Charge Pump 1
//
pub const WM5100_CP2_BYPASS: c_uint = 0x0020  /* CP2_BYPASS */;
pub const WM5100_CP2_BYPASS_MASK: c_uint = 0x0020  /* CP2_BYPASS */;

pub const WM5100_CP2_ENA: c_uint = 0x0001  /* CP2_ENA */;
pub const WM5100_CP2_ENA_MASK: c_uint = 0x0001  /* CP2_ENA */;

//
// R513 (0x201) - Mic Charge Pump 2
//
pub const WM5100_LDO2_VSEL_MASK: c_uint = 0xF800  /* LDO2_VSEL - [15:11] */;

//
// R514 (0x202) - HP Charge Pump 1
//
pub const WM5100_CP1_ENA: c_uint = 0x0001  /* CP1_ENA */;
pub const WM5100_CP1_ENA_MASK: c_uint = 0x0001  /* CP1_ENA */;

//
// R529 (0x211) - LDO1 Control
//
pub const WM5100_LDO1_BYPASS: c_uint = 0x0002  /* LDO1_BYPASS */;
pub const WM5100_LDO1_BYPASS_MASK: c_uint = 0x0002  /* LDO1_BYPASS */;

//
// R533 (0x215) - Mic Bias Ctrl 1
//
pub const WM5100_MICB1_DISCH: c_uint = 0x0040  /* MICB1_DISCH */;
pub const WM5100_MICB1_DISCH_MASK: c_uint = 0x0040  /* MICB1_DISCH */;

pub const WM5100_MICB1_RATE: c_uint = 0x0020  /* MICB1_RATE */;
pub const WM5100_MICB1_RATE_MASK: c_uint = 0x0020  /* MICB1_RATE */;

pub const WM5100_MICB1_LVL_MASK: c_uint = 0x001C  /* MICB1_LVL - [4:2] */;

pub const WM5100_MICB1_BYPASS: c_uint = 0x0002  /* MICB1_BYPASS */;
pub const WM5100_MICB1_BYPASS_MASK: c_uint = 0x0002  /* MICB1_BYPASS */;

pub const WM5100_MICB1_ENA: c_uint = 0x0001  /* MICB1_ENA */;
pub const WM5100_MICB1_ENA_MASK: c_uint = 0x0001  /* MICB1_ENA */;

//
// R534 (0x216) - Mic Bias Ctrl 2
//
pub const WM5100_MICB2_DISCH: c_uint = 0x0040  /* MICB2_DISCH */;
pub const WM5100_MICB2_DISCH_MASK: c_uint = 0x0040  /* MICB2_DISCH */;

pub const WM5100_MICB2_RATE: c_uint = 0x0020  /* MICB2_RATE */;
pub const WM5100_MICB2_RATE_MASK: c_uint = 0x0020  /* MICB2_RATE */;

pub const WM5100_MICB2_LVL_MASK: c_uint = 0x001C  /* MICB2_LVL - [4:2] */;

pub const WM5100_MICB2_BYPASS: c_uint = 0x0002  /* MICB2_BYPASS */;
pub const WM5100_MICB2_BYPASS_MASK: c_uint = 0x0002  /* MICB2_BYPASS */;

pub const WM5100_MICB2_ENA: c_uint = 0x0001  /* MICB2_ENA */;
pub const WM5100_MICB2_ENA_MASK: c_uint = 0x0001  /* MICB2_ENA */;

//
// R535 (0x217) - Mic Bias Ctrl 3
//
pub const WM5100_MICB3_DISCH: c_uint = 0x0040  /* MICB3_DISCH */;
pub const WM5100_MICB3_DISCH_MASK: c_uint = 0x0040  /* MICB3_DISCH */;

pub const WM5100_MICB3_RATE: c_uint = 0x0020  /* MICB3_RATE */;
pub const WM5100_MICB3_RATE_MASK: c_uint = 0x0020  /* MICB3_RATE */;

pub const WM5100_MICB3_LVL_MASK: c_uint = 0x001C  /* MICB3_LVL - [4:2] */;

pub const WM5100_MICB3_BYPASS: c_uint = 0x0002  /* MICB3_BYPASS */;
pub const WM5100_MICB3_BYPASS_MASK: c_uint = 0x0002  /* MICB3_BYPASS */;

pub const WM5100_MICB3_ENA: c_uint = 0x0001  /* MICB3_ENA */;
pub const WM5100_MICB3_ENA_MASK: c_uint = 0x0001  /* MICB3_ENA */;

//
// R640 (0x280) - Accessory Detect Mode 1
//
pub const WM5100_ACCDET_BIAS_SRC_MASK: c_uint = 0xC000  /* ACCDET_BIAS_SRC - [15:14] */;

pub const WM5100_ACCDET_SRC: c_uint = 0x2000  /* ACCDET_SRC */;
pub const WM5100_ACCDET_SRC_MASK: c_uint = 0x2000  /* ACCDET_SRC */;

pub const WM5100_ACCDET_MODE_MASK: c_uint = 0x0003  /* ACCDET_MODE - [1:0] */;

//
// R648 (0x288) - Headphone Detect 1
//
pub const WM5100_HP_HOLDTIME_MASK: c_uint = 0x00E0  /* HP_HOLDTIME - [7:5] */;

pub const WM5100_HP_CLK_DIV_MASK: c_uint = 0x0018  /* HP_CLK_DIV - [4:3] */;

pub const WM5100_HP_STEP_SIZE: c_uint = 0x0002  /* HP_STEP_SIZE */;
pub const WM5100_HP_STEP_SIZE_MASK: c_uint = 0x0002  /* HP_STEP_SIZE */;

pub const WM5100_HP_POLL: c_uint = 0x0001  /* HP_POLL */;
pub const WM5100_HP_POLL_MASK: c_uint = 0x0001  /* HP_POLL */;

//
// R649 (0x289) - Headphone Detect 2
//
pub const WM5100_HP_DONE: c_uint = 0x0080  /* HP_DONE */;
pub const WM5100_HP_DONE_MASK: c_uint = 0x0080  /* HP_DONE */;

pub const WM5100_HP_LVL_MASK: c_uint = 0x007F  /* HP_LVL - [6:0] */;

//
// R656 (0x290) - Mic Detect 1
//
pub const WM5100_ACCDET_BIAS_STARTTIME_MASK: c_uint = 0xF000  /* ACCDET_BIAS_STARTTIME - [15:12] */;

pub const WM5100_ACCDET_RATE_MASK: c_uint = 0x0F00  /* ACCDET_RATE - [11:8] */;

pub const WM5100_ACCDET_DBTIME: c_uint = 0x0002  /* ACCDET_DBTIME */;
pub const WM5100_ACCDET_DBTIME_MASK: c_uint = 0x0002  /* ACCDET_DBTIME */;

pub const WM5100_ACCDET_ENA: c_uint = 0x0001  /* ACCDET_ENA */;
pub const WM5100_ACCDET_ENA_MASK: c_uint = 0x0001  /* ACCDET_ENA */;

//
// R657 (0x291) - Mic Detect 2
//
pub const WM5100_ACCDET_LVL_SEL_MASK: c_uint = 0x00FF  /* ACCDET_LVL_SEL - [7:0] */;

//
// R658 (0x292) - Mic Detect 3
//
pub const WM5100_ACCDET_LVL_MASK: c_uint = 0x07FC  /* ACCDET_LVL - [10:2] */;

pub const WM5100_ACCDET_VALID: c_uint = 0x0002  /* ACCDET_VALID */;
pub const WM5100_ACCDET_VALID_MASK: c_uint = 0x0002  /* ACCDET_VALID */;

pub const WM5100_ACCDET_STS: c_uint = 0x0001  /* ACCDET_STS */;
pub const WM5100_ACCDET_STS_MASK: c_uint = 0x0001  /* ACCDET_STS */;

//
// R699 (0x2BB) - Misc Control
//
pub const WM5100_HPCOM_SRC: c_uint = 0x200  /* HPCOM_SRC */;

//
// R769 (0x301) - Input Enables
//
pub const WM5100_IN4L_ENA: c_uint = 0x0080  /* IN4L_ENA */;
pub const WM5100_IN4L_ENA_MASK: c_uint = 0x0080  /* IN4L_ENA */;

pub const WM5100_IN4R_ENA: c_uint = 0x0040  /* IN4R_ENA */;
pub const WM5100_IN4R_ENA_MASK: c_uint = 0x0040  /* IN4R_ENA */;

pub const WM5100_IN3L_ENA: c_uint = 0x0020  /* IN3L_ENA */;
pub const WM5100_IN3L_ENA_MASK: c_uint = 0x0020  /* IN3L_ENA */;

pub const WM5100_IN3R_ENA: c_uint = 0x0010  /* IN3R_ENA */;
pub const WM5100_IN3R_ENA_MASK: c_uint = 0x0010  /* IN3R_ENA */;

pub const WM5100_IN2L_ENA: c_uint = 0x0008  /* IN2L_ENA */;
pub const WM5100_IN2L_ENA_MASK: c_uint = 0x0008  /* IN2L_ENA */;

pub const WM5100_IN2R_ENA: c_uint = 0x0004  /* IN2R_ENA */;
pub const WM5100_IN2R_ENA_MASK: c_uint = 0x0004  /* IN2R_ENA */;

pub const WM5100_IN1L_ENA: c_uint = 0x0002  /* IN1L_ENA */;
pub const WM5100_IN1L_ENA_MASK: c_uint = 0x0002  /* IN1L_ENA */;

pub const WM5100_IN1R_ENA: c_uint = 0x0001  /* IN1R_ENA */;
pub const WM5100_IN1R_ENA_MASK: c_uint = 0x0001  /* IN1R_ENA */;

//
// R770 (0x302) - Input Enables Status
//
pub const WM5100_IN4L_ENA_STS: c_uint = 0x0080  /* IN4L_ENA_STS */;
pub const WM5100_IN4L_ENA_STS_MASK: c_uint = 0x0080  /* IN4L_ENA_STS */;

pub const WM5100_IN4R_ENA_STS: c_uint = 0x0040  /* IN4R_ENA_STS */;
pub const WM5100_IN4R_ENA_STS_MASK: c_uint = 0x0040  /* IN4R_ENA_STS */;

pub const WM5100_IN3L_ENA_STS: c_uint = 0x0020  /* IN3L_ENA_STS */;
pub const WM5100_IN3L_ENA_STS_MASK: c_uint = 0x0020  /* IN3L_ENA_STS */;

pub const WM5100_IN3R_ENA_STS: c_uint = 0x0010  /* IN3R_ENA_STS */;
pub const WM5100_IN3R_ENA_STS_MASK: c_uint = 0x0010  /* IN3R_ENA_STS */;

pub const WM5100_IN2L_ENA_STS: c_uint = 0x0008  /* IN2L_ENA_STS */;
pub const WM5100_IN2L_ENA_STS_MASK: c_uint = 0x0008  /* IN2L_ENA_STS */;

pub const WM5100_IN2R_ENA_STS: c_uint = 0x0004  /* IN2R_ENA_STS */;
pub const WM5100_IN2R_ENA_STS_MASK: c_uint = 0x0004  /* IN2R_ENA_STS */;

pub const WM5100_IN1L_ENA_STS: c_uint = 0x0002  /* IN1L_ENA_STS */;
pub const WM5100_IN1L_ENA_STS_MASK: c_uint = 0x0002  /* IN1L_ENA_STS */;

pub const WM5100_IN1R_ENA_STS: c_uint = 0x0001  /* IN1R_ENA_STS */;
pub const WM5100_IN1R_ENA_STS_MASK: c_uint = 0x0001  /* IN1R_ENA_STS */;

//
// R784 (0x310) - IN1L Control
//
pub const WM5100_IN_RATE_MASK: c_uint = 0xC000  /* IN_RATE - [15:14] */;

pub const WM5100_IN1_OSR: c_uint = 0x2000  /* IN1_OSR */;
pub const WM5100_IN1_OSR_MASK: c_uint = 0x2000  /* IN1_OSR */;

pub const WM5100_IN1_DMIC_SUP_MASK: c_uint = 0x1800  /* IN1_DMIC_SUP - [12:11] */;

pub const WM5100_IN1_MODE_MASK: c_uint = 0x0600  /* IN1_MODE - [10:9] */;

pub const WM5100_IN1L_PGA_VOL_MASK: c_uint = 0x00FE  /* IN1L_PGA_VOL - [7:1] */;

//
// R785 (0x311) - IN1R Control
//
pub const WM5100_IN1R_PGA_VOL_MASK: c_uint = 0x00FE  /* IN1R_PGA_VOL - [7:1] */;

//
// R786 (0x312) - IN2L Control
//
pub const WM5100_IN2_OSR: c_uint = 0x2000  /* IN2_OSR */;
pub const WM5100_IN2_OSR_MASK: c_uint = 0x2000  /* IN2_OSR */;

pub const WM5100_IN2_DMIC_SUP_MASK: c_uint = 0x1800  /* IN2_DMIC_SUP - [12:11] */;

pub const WM5100_IN2_MODE_MASK: c_uint = 0x0600  /* IN2_MODE - [10:9] */;

pub const WM5100_IN2L_PGA_VOL_MASK: c_uint = 0x00FE  /* IN2L_PGA_VOL - [7:1] */;

//
// R787 (0x313) - IN2R Control
//
pub const WM5100_IN2R_PGA_VOL_MASK: c_uint = 0x00FE  /* IN2R_PGA_VOL - [7:1] */;

//
// R788 (0x314) - IN3L Control
//
pub const WM5100_IN3_OSR: c_uint = 0x2000  /* IN3_OSR */;
pub const WM5100_IN3_OSR_MASK: c_uint = 0x2000  /* IN3_OSR */;

pub const WM5100_IN3_DMIC_SUP_MASK: c_uint = 0x1800  /* IN3_DMIC_SUP - [12:11] */;

pub const WM5100_IN3_MODE_MASK: c_uint = 0x0600  /* IN3_MODE - [10:9] */;

pub const WM5100_IN3L_PGA_VOL_MASK: c_uint = 0x00FE  /* IN3L_PGA_VOL - [7:1] */;

//
// R789 (0x315) - IN3R Control
//
pub const WM5100_IN3R_PGA_VOL_MASK: c_uint = 0x00FE  /* IN3R_PGA_VOL - [7:1] */;

//
// R790 (0x316) - IN4L Control
//
pub const WM5100_IN4_OSR: c_uint = 0x2000  /* IN4_OSR */;
pub const WM5100_IN4_OSR_MASK: c_uint = 0x2000  /* IN4_OSR */;

pub const WM5100_IN4_DMIC_SUP_MASK: c_uint = 0x1800  /* IN4_DMIC_SUP - [12:11] */;

pub const WM5100_IN4_MODE_MASK: c_uint = 0x0600  /* IN4_MODE - [10:9] */;

pub const WM5100_IN4L_PGA_VOL_MASK: c_uint = 0x00FE  /* IN4L_PGA_VOL - [7:1] */;

//
// R791 (0x317) - IN4R Control
//
pub const WM5100_IN4R_PGA_VOL_MASK: c_uint = 0x00FE  /* IN4R_PGA_VOL - [7:1] */;

//
// R792 (0x318) - RXANC_SRC
//
pub const WM5100_IN_RXANC_SEL_MASK: c_uint = 0x0007  /* IN_RXANC_SEL - [2:0] */;

//
// R793 (0x319) - Input Volume Ramp
//
pub const WM5100_IN_VD_RAMP_MASK: c_uint = 0x0070  /* IN_VD_RAMP - [6:4] */;

pub const WM5100_IN_VI_RAMP_MASK: c_uint = 0x0007  /* IN_VI_RAMP - [2:0] */;

//
// R800 (0x320) - ADC Digital Volume 1L
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN1L_MUTE: c_uint = 0x0100  /* IN1L_MUTE */;
pub const WM5100_IN1L_MUTE_MASK: c_uint = 0x0100  /* IN1L_MUTE */;

pub const WM5100_IN1L_VOL_MASK: c_uint = 0x00FF  /* IN1L_VOL - [7:0] */;

//
// R801 (0x321) - ADC Digital Volume 1R
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN1R_MUTE: c_uint = 0x0100  /* IN1R_MUTE */;
pub const WM5100_IN1R_MUTE_MASK: c_uint = 0x0100  /* IN1R_MUTE */;

pub const WM5100_IN1R_VOL_MASK: c_uint = 0x00FF  /* IN1R_VOL - [7:0] */;

//
// R802 (0x322) - ADC Digital Volume 2L
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN2L_MUTE: c_uint = 0x0100  /* IN2L_MUTE */;
pub const WM5100_IN2L_MUTE_MASK: c_uint = 0x0100  /* IN2L_MUTE */;

pub const WM5100_IN2L_VOL_MASK: c_uint = 0x00FF  /* IN2L_VOL - [7:0] */;

//
// R803 (0x323) - ADC Digital Volume 2R
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN2R_MUTE: c_uint = 0x0100  /* IN2R_MUTE */;
pub const WM5100_IN2R_MUTE_MASK: c_uint = 0x0100  /* IN2R_MUTE */;

pub const WM5100_IN2R_VOL_MASK: c_uint = 0x00FF  /* IN2R_VOL - [7:0] */;

//
// R804 (0x324) - ADC Digital Volume 3L
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN3L_MUTE: c_uint = 0x0100  /* IN3L_MUTE */;
pub const WM5100_IN3L_MUTE_MASK: c_uint = 0x0100  /* IN3L_MUTE */;

pub const WM5100_IN3L_VOL_MASK: c_uint = 0x00FF  /* IN3L_VOL - [7:0] */;

//
// R805 (0x325) - ADC Digital Volume 3R
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN3R_MUTE: c_uint = 0x0100  /* IN3R_MUTE */;
pub const WM5100_IN3R_MUTE_MASK: c_uint = 0x0100  /* IN3R_MUTE */;

pub const WM5100_IN3R_VOL_MASK: c_uint = 0x00FF  /* IN3R_VOL - [7:0] */;

//
// R806 (0x326) - ADC Digital Volume 4L
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN4L_MUTE: c_uint = 0x0100  /* IN4L_MUTE */;
pub const WM5100_IN4L_MUTE_MASK: c_uint = 0x0100  /* IN4L_MUTE */;

pub const WM5100_IN4L_VOL_MASK: c_uint = 0x00FF  /* IN4L_VOL - [7:0] */;

//
// R807 (0x327) - ADC Digital Volume 4R
//
pub const WM5100_IN_VU: c_uint = 0x0200  /* IN_VU */;
pub const WM5100_IN_VU_MASK: c_uint = 0x0200  /* IN_VU */;

pub const WM5100_IN4R_MUTE: c_uint = 0x0100  /* IN4R_MUTE */;
pub const WM5100_IN4R_MUTE_MASK: c_uint = 0x0100  /* IN4R_MUTE */;

pub const WM5100_IN4R_VOL_MASK: c_uint = 0x00FF  /* IN4R_VOL - [7:0] */;

//
// R1025 (0x401) - Output Enables 2
//
pub const WM5100_OUT6L_ENA: c_uint = 0x0800  /* OUT6L_ENA */;
pub const WM5100_OUT6L_ENA_MASK: c_uint = 0x0800  /* OUT6L_ENA */;

pub const WM5100_OUT6R_ENA: c_uint = 0x0400  /* OUT6R_ENA */;
pub const WM5100_OUT6R_ENA_MASK: c_uint = 0x0400  /* OUT6R_ENA */;

pub const WM5100_OUT5L_ENA: c_uint = 0x0200  /* OUT5L_ENA */;
pub const WM5100_OUT5L_ENA_MASK: c_uint = 0x0200  /* OUT5L_ENA */;

pub const WM5100_OUT5R_ENA: c_uint = 0x0100  /* OUT5R_ENA */;
pub const WM5100_OUT5R_ENA_MASK: c_uint = 0x0100  /* OUT5R_ENA */;

pub const WM5100_OUT4L_ENA: c_uint = 0x0080  /* OUT4L_ENA */;
pub const WM5100_OUT4L_ENA_MASK: c_uint = 0x0080  /* OUT4L_ENA */;

pub const WM5100_OUT4R_ENA: c_uint = 0x0040  /* OUT4R_ENA */;
pub const WM5100_OUT4R_ENA_MASK: c_uint = 0x0040  /* OUT4R_ENA */;

//
// R1026 (0x402) - Output Status 1
//
pub const WM5100_OUT3L_ENA_STS: c_uint = 0x0020  /* OUT3L_ENA_STS */;
pub const WM5100_OUT3L_ENA_STS_MASK: c_uint = 0x0020  /* OUT3L_ENA_STS */;

pub const WM5100_OUT3R_ENA_STS: c_uint = 0x0010  /* OUT3R_ENA_STS */;
pub const WM5100_OUT3R_ENA_STS_MASK: c_uint = 0x0010  /* OUT3R_ENA_STS */;

pub const WM5100_OUT2L_ENA_STS: c_uint = 0x0008  /* OUT2L_ENA_STS */;
pub const WM5100_OUT2L_ENA_STS_MASK: c_uint = 0x0008  /* OUT2L_ENA_STS */;

pub const WM5100_OUT2R_ENA_STS: c_uint = 0x0004  /* OUT2R_ENA_STS */;
pub const WM5100_OUT2R_ENA_STS_MASK: c_uint = 0x0004  /* OUT2R_ENA_STS */;

pub const WM5100_OUT1L_ENA_STS: c_uint = 0x0002  /* OUT1L_ENA_STS */;
pub const WM5100_OUT1L_ENA_STS_MASK: c_uint = 0x0002  /* OUT1L_ENA_STS */;

pub const WM5100_OUT1R_ENA_STS: c_uint = 0x0001  /* OUT1R_ENA_STS */;
pub const WM5100_OUT1R_ENA_STS_MASK: c_uint = 0x0001  /* OUT1R_ENA_STS */;

//
// R1027 (0x403) - Output Status 2
//
pub const WM5100_OUT6L_ENA_STS: c_uint = 0x0800  /* OUT6L_ENA_STS */;
pub const WM5100_OUT6L_ENA_STS_MASK: c_uint = 0x0800  /* OUT6L_ENA_STS */;

pub const WM5100_OUT6R_ENA_STS: c_uint = 0x0400  /* OUT6R_ENA_STS */;
pub const WM5100_OUT6R_ENA_STS_MASK: c_uint = 0x0400  /* OUT6R_ENA_STS */;

pub const WM5100_OUT5L_ENA_STS: c_uint = 0x0200  /* OUT5L_ENA_STS */;
pub const WM5100_OUT5L_ENA_STS_MASK: c_uint = 0x0200  /* OUT5L_ENA_STS */;

pub const WM5100_OUT5R_ENA_STS: c_uint = 0x0100  /* OUT5R_ENA_STS */;
pub const WM5100_OUT5R_ENA_STS_MASK: c_uint = 0x0100  /* OUT5R_ENA_STS */;

pub const WM5100_OUT4L_ENA_STS: c_uint = 0x0080  /* OUT4L_ENA_STS */;
pub const WM5100_OUT4L_ENA_STS_MASK: c_uint = 0x0080  /* OUT4L_ENA_STS */;

pub const WM5100_OUT4R_ENA_STS: c_uint = 0x0040  /* OUT4R_ENA_STS */;
pub const WM5100_OUT4R_ENA_STS_MASK: c_uint = 0x0040  /* OUT4R_ENA_STS */;

//
// R1032 (0x408) - Channel Enables 1
//
pub const WM5100_HP3L_ENA: c_uint = 0x0020  /* HP3L_ENA */;
pub const WM5100_HP3L_ENA_MASK: c_uint = 0x0020  /* HP3L_ENA */;

pub const WM5100_HP3R_ENA: c_uint = 0x0010  /* HP3R_ENA */;
pub const WM5100_HP3R_ENA_MASK: c_uint = 0x0010  /* HP3R_ENA */;

pub const WM5100_HP2L_ENA: c_uint = 0x0008  /* HP2L_ENA */;
pub const WM5100_HP2L_ENA_MASK: c_uint = 0x0008  /* HP2L_ENA */;

pub const WM5100_HP2R_ENA: c_uint = 0x0004  /* HP2R_ENA */;
pub const WM5100_HP2R_ENA_MASK: c_uint = 0x0004  /* HP2R_ENA */;

pub const WM5100_HP1L_ENA: c_uint = 0x0002  /* HP1L_ENA */;
pub const WM5100_HP1L_ENA_MASK: c_uint = 0x0002  /* HP1L_ENA */;

pub const WM5100_HP1R_ENA: c_uint = 0x0001  /* HP1R_ENA */;
pub const WM5100_HP1R_ENA_MASK: c_uint = 0x0001  /* HP1R_ENA */;

//
// R1040 (0x410) - Out Volume 1L
//
pub const WM5100_OUT_RATE_MASK: c_uint = 0xC000  /* OUT_RATE - [15:14] */;

pub const WM5100_OUT1_OSR: c_uint = 0x2000  /* OUT1_OSR */;
pub const WM5100_OUT1_OSR_MASK: c_uint = 0x2000  /* OUT1_OSR */;

pub const WM5100_OUT1_MONO: c_uint = 0x1000  /* OUT1_MONO */;
pub const WM5100_OUT1_MONO_MASK: c_uint = 0x1000  /* OUT1_MONO */;

pub const WM5100_OUT1L_ANC_SRC: c_uint = 0x0800  /* OUT1L_ANC_SRC */;
pub const WM5100_OUT1L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT1L_ANC_SRC */;

pub const WM5100_OUT1L_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT1L_PGA_VOL - [7:1] */;

//
// R1041 (0x411) - Out Volume 1R
//
pub const WM5100_OUT1R_ANC_SRC: c_uint = 0x0800  /* OUT1R_ANC_SRC */;
pub const WM5100_OUT1R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT1R_ANC_SRC */;

pub const WM5100_OUT1R_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT1R_PGA_VOL - [7:1] */;

//
// R1042 (0x412) - DAC Volume Limit 1L
//
pub const WM5100_OUT1L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT1L_VOL_LIM - [7:0] */;

//
// R1043 (0x413) - DAC Volume Limit 1R
//
pub const WM5100_OUT1R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT1R_VOL_LIM - [7:0] */;

//
// R1044 (0x414) - Out Volume 2L
//
pub const WM5100_OUT2_OSR: c_uint = 0x2000  /* OUT2_OSR */;
pub const WM5100_OUT2_OSR_MASK: c_uint = 0x2000  /* OUT2_OSR */;

pub const WM5100_OUT2_MONO: c_uint = 0x1000  /* OUT2_MONO */;
pub const WM5100_OUT2_MONO_MASK: c_uint = 0x1000  /* OUT2_MONO */;

pub const WM5100_OUT2L_ANC_SRC: c_uint = 0x0800  /* OUT2L_ANC_SRC */;
pub const WM5100_OUT2L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT2L_ANC_SRC */;

pub const WM5100_OUT2L_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT2L_PGA_VOL - [7:1] */;

//
// R1045 (0x415) - Out Volume 2R
//
pub const WM5100_OUT2R_ANC_SRC: c_uint = 0x0800  /* OUT2R_ANC_SRC */;
pub const WM5100_OUT2R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT2R_ANC_SRC */;

pub const WM5100_OUT2R_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT2R_PGA_VOL - [7:1] */;

//
// R1046 (0x416) - DAC Volume Limit 2L
//
pub const WM5100_OUT2L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT2L_VOL_LIM - [7:0] */;

//
// R1047 (0x417) - DAC Volume Limit 2R
//
pub const WM5100_OUT2R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT2R_VOL_LIM - [7:0] */;

//
// R1048 (0x418) - Out Volume 3L
//
pub const WM5100_OUT3_OSR: c_uint = 0x2000  /* OUT3_OSR */;
pub const WM5100_OUT3_OSR_MASK: c_uint = 0x2000  /* OUT3_OSR */;

pub const WM5100_OUT3_MONO: c_uint = 0x1000  /* OUT3_MONO */;
pub const WM5100_OUT3_MONO_MASK: c_uint = 0x1000  /* OUT3_MONO */;

pub const WM5100_OUT3L_ANC_SRC: c_uint = 0x0800  /* OUT3L_ANC_SRC */;
pub const WM5100_OUT3L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT3L_ANC_SRC */;

pub const WM5100_OUT3L_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT3L_PGA_VOL - [7:1] */;

//
// R1049 (0x419) - Out Volume 3R
//
pub const WM5100_OUT3R_ANC_SRC: c_uint = 0x0800  /* OUT3R_ANC_SRC */;
pub const WM5100_OUT3R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT3R_ANC_SRC */;

pub const WM5100_OUT3R_PGA_VOL_MASK: c_uint = 0x00FE  /* OUT3R_PGA_VOL - [7:1] */;

//
// R1050 (0x41A) - DAC Volume Limit 3L
//
pub const WM5100_OUT3L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT3L_VOL_LIM - [7:0] */;

//
// R1051 (0x41B) - DAC Volume Limit 3R
//
pub const WM5100_OUT3R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT3R_VOL_LIM - [7:0] */;

//
// R1052 (0x41C) - Out Volume 4L
//
pub const WM5100_OUT4_OSR: c_uint = 0x2000  /* OUT4_OSR */;
pub const WM5100_OUT4_OSR_MASK: c_uint = 0x2000  /* OUT4_OSR */;

pub const WM5100_OUT4L_ANC_SRC: c_uint = 0x0800  /* OUT4L_ANC_SRC */;
pub const WM5100_OUT4L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT4L_ANC_SRC */;

pub const WM5100_OUT4L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT4L_VOL_LIM - [7:0] */;

//
// R1053 (0x41D) - Out Volume 4R
//
pub const WM5100_OUT4R_ANC_SRC: c_uint = 0x0800  /* OUT4R_ANC_SRC */;
pub const WM5100_OUT4R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT4R_ANC_SRC */;

pub const WM5100_OUT4R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT4R_VOL_LIM - [7:0] */;

//
// R1054 (0x41E) - DAC Volume Limit 5L
//
pub const WM5100_OUT5_OSR: c_uint = 0x2000  /* OUT5_OSR */;
pub const WM5100_OUT5_OSR_MASK: c_uint = 0x2000  /* OUT5_OSR */;

pub const WM5100_OUT5L_ANC_SRC: c_uint = 0x0800  /* OUT5L_ANC_SRC */;
pub const WM5100_OUT5L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT5L_ANC_SRC */;

pub const WM5100_OUT5L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT5L_VOL_LIM - [7:0] */;

//
// R1055 (0x41F) - DAC Volume Limit 5R
//
pub const WM5100_OUT5R_ANC_SRC: c_uint = 0x0800  /* OUT5R_ANC_SRC */;
pub const WM5100_OUT5R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT5R_ANC_SRC */;

pub const WM5100_OUT5R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT5R_VOL_LIM - [7:0] */;

//
// R1056 (0x420) - DAC Volume Limit 6L
//
pub const WM5100_OUT6_OSR: c_uint = 0x2000  /* OUT6_OSR */;
pub const WM5100_OUT6_OSR_MASK: c_uint = 0x2000  /* OUT6_OSR */;

pub const WM5100_OUT6L_ANC_SRC: c_uint = 0x0800  /* OUT6L_ANC_SRC */;
pub const WM5100_OUT6L_ANC_SRC_MASK: c_uint = 0x0800  /* OUT6L_ANC_SRC */;

pub const WM5100_OUT6L_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT6L_VOL_LIM - [7:0] */;

//
// R1057 (0x421) - DAC Volume Limit 6R
//
pub const WM5100_OUT6R_ANC_SRC: c_uint = 0x0800  /* OUT6R_ANC_SRC */;
pub const WM5100_OUT6R_ANC_SRC_MASK: c_uint = 0x0800  /* OUT6R_ANC_SRC */;

pub const WM5100_OUT6R_VOL_LIM_MASK: c_uint = 0x00FF  /* OUT6R_VOL_LIM - [7:0] */;

//
// R1088 (0x440) - DAC AEC Control 1
//
pub const WM5100_AEC_LOOPBACK_SRC_MASK: c_uint = 0x003C  /* AEC_LOOPBACK_SRC - [5:2] */;

pub const WM5100_AEC_ENA_STS: c_uint = 0x0002  /* AEC_ENA_STS */;
pub const WM5100_AEC_ENA_STS_MASK: c_uint = 0x0002  /* AEC_ENA_STS */;

pub const WM5100_AEC_LOOPBACK_ENA: c_uint = 0x0001  /* AEC_LOOPBACK_ENA */;
pub const WM5100_AEC_LOOPBACK_ENA_MASK: c_uint = 0x0001  /* AEC_LOOPBACK_ENA */;

//
// R1089 (0x441) - Output Volume Ramp
//
pub const WM5100_OUT_VD_RAMP_MASK: c_uint = 0x0070  /* OUT_VD_RAMP - [6:4] */;

pub const WM5100_OUT_VI_RAMP_MASK: c_uint = 0x0007  /* OUT_VI_RAMP - [2:0] */;

//
// R1152 (0x480) - DAC Digital Volume 1L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT1L_MUTE: c_uint = 0x0100  /* OUT1L_MUTE */;
pub const WM5100_OUT1L_MUTE_MASK: c_uint = 0x0100  /* OUT1L_MUTE */;

pub const WM5100_OUT1L_VOL_MASK: c_uint = 0x00FF  /* OUT1L_VOL - [7:0] */;

//
// R1153 (0x481) - DAC Digital Volume 1R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT1R_MUTE: c_uint = 0x0100  /* OUT1R_MUTE */;
pub const WM5100_OUT1R_MUTE_MASK: c_uint = 0x0100  /* OUT1R_MUTE */;

pub const WM5100_OUT1R_VOL_MASK: c_uint = 0x00FF  /* OUT1R_VOL - [7:0] */;

//
// R1154 (0x482) - DAC Digital Volume 2L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT2L_MUTE: c_uint = 0x0100  /* OUT2L_MUTE */;
pub const WM5100_OUT2L_MUTE_MASK: c_uint = 0x0100  /* OUT2L_MUTE */;

pub const WM5100_OUT2L_VOL_MASK: c_uint = 0x00FF  /* OUT2L_VOL - [7:0] */;

//
// R1155 (0x483) - DAC Digital Volume 2R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT2R_MUTE: c_uint = 0x0100  /* OUT2R_MUTE */;
pub const WM5100_OUT2R_MUTE_MASK: c_uint = 0x0100  /* OUT2R_MUTE */;

pub const WM5100_OUT2R_VOL_MASK: c_uint = 0x00FF  /* OUT2R_VOL - [7:0] */;

//
// R1156 (0x484) - DAC Digital Volume 3L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT3L_MUTE: c_uint = 0x0100  /* OUT3L_MUTE */;
pub const WM5100_OUT3L_MUTE_MASK: c_uint = 0x0100  /* OUT3L_MUTE */;

pub const WM5100_OUT3L_VOL_MASK: c_uint = 0x00FF  /* OUT3L_VOL - [7:0] */;

//
// R1157 (0x485) - DAC Digital Volume 3R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT3R_MUTE: c_uint = 0x0100  /* OUT3R_MUTE */;
pub const WM5100_OUT3R_MUTE_MASK: c_uint = 0x0100  /* OUT3R_MUTE */;

pub const WM5100_OUT3R_VOL_MASK: c_uint = 0x00FF  /* OUT3R_VOL - [7:0] */;

//
// R1158 (0x486) - DAC Digital Volume 4L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT4L_MUTE: c_uint = 0x0100  /* OUT4L_MUTE */;
pub const WM5100_OUT4L_MUTE_MASK: c_uint = 0x0100  /* OUT4L_MUTE */;

pub const WM5100_OUT4L_VOL_MASK: c_uint = 0x00FF  /* OUT4L_VOL - [7:0] */;

//
// R1159 (0x487) - DAC Digital Volume 4R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT4R_MUTE: c_uint = 0x0100  /* OUT4R_MUTE */;
pub const WM5100_OUT4R_MUTE_MASK: c_uint = 0x0100  /* OUT4R_MUTE */;

pub const WM5100_OUT4R_VOL_MASK: c_uint = 0x00FF  /* OUT4R_VOL - [7:0] */;

//
// R1160 (0x488) - DAC Digital Volume 5L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT5L_MUTE: c_uint = 0x0100  /* OUT5L_MUTE */;
pub const WM5100_OUT5L_MUTE_MASK: c_uint = 0x0100  /* OUT5L_MUTE */;

pub const WM5100_OUT5L_VOL_MASK: c_uint = 0x00FF  /* OUT5L_VOL - [7:0] */;

//
// R1161 (0x489) - DAC Digital Volume 5R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT5R_MUTE: c_uint = 0x0100  /* OUT5R_MUTE */;
pub const WM5100_OUT5R_MUTE_MASK: c_uint = 0x0100  /* OUT5R_MUTE */;

pub const WM5100_OUT5R_VOL_MASK: c_uint = 0x00FF  /* OUT5R_VOL - [7:0] */;

//
// R1162 (0x48A) - DAC Digital Volume 6L
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT6L_MUTE: c_uint = 0x0100  /* OUT6L_MUTE */;
pub const WM5100_OUT6L_MUTE_MASK: c_uint = 0x0100  /* OUT6L_MUTE */;

pub const WM5100_OUT6L_VOL_MASK: c_uint = 0x00FF  /* OUT6L_VOL - [7:0] */;

//
// R1163 (0x48B) - DAC Digital Volume 6R
//
pub const WM5100_OUT_VU: c_uint = 0x0200  /* OUT_VU */;
pub const WM5100_OUT_VU_MASK: c_uint = 0x0200  /* OUT_VU */;

pub const WM5100_OUT6R_MUTE: c_uint = 0x0100  /* OUT6R_MUTE */;
pub const WM5100_OUT6R_MUTE_MASK: c_uint = 0x0100  /* OUT6R_MUTE */;

pub const WM5100_OUT6R_VOL_MASK: c_uint = 0x00FF  /* OUT6R_VOL - [7:0] */;

//
// R1216 (0x4C0) - PDM SPK1 CTRL 1
//
pub const WM5100_SPK1R_MUTE: c_uint = 0x2000  /* SPK1R_MUTE */;
pub const WM5100_SPK1R_MUTE_MASK: c_uint = 0x2000  /* SPK1R_MUTE */;

pub const WM5100_SPK1L_MUTE: c_uint = 0x1000  /* SPK1L_MUTE */;
pub const WM5100_SPK1L_MUTE_MASK: c_uint = 0x1000  /* SPK1L_MUTE */;

pub const WM5100_SPK1_MUTE_ENDIAN: c_uint = 0x0100  /* SPK1_MUTE_ENDIAN */;
pub const WM5100_SPK1_MUTE_ENDIAN_MASK: c_uint = 0x0100  /* SPK1_MUTE_ENDIAN */;

pub const WM5100_SPK1_MUTE_SEQ1_MASK: c_uint = 0x00FF  /* SPK1_MUTE_SEQ1 - [7:0] */;

//
// R1217 (0x4C1) - PDM SPK1 CTRL 2
//
pub const WM5100_SPK1_FMT: c_uint = 0x0001  /* SPK1_FMT */;
pub const WM5100_SPK1_FMT_MASK: c_uint = 0x0001  /* SPK1_FMT */;

//
// R1218 (0x4C2) - PDM SPK2 CTRL 1
//
pub const WM5100_SPK2R_MUTE: c_uint = 0x2000  /* SPK2R_MUTE */;
pub const WM5100_SPK2R_MUTE_MASK: c_uint = 0x2000  /* SPK2R_MUTE */;

pub const WM5100_SPK2L_MUTE: c_uint = 0x1000  /* SPK2L_MUTE */;
pub const WM5100_SPK2L_MUTE_MASK: c_uint = 0x1000  /* SPK2L_MUTE */;

pub const WM5100_SPK2_MUTE_ENDIAN: c_uint = 0x0100  /* SPK2_MUTE_ENDIAN */;
pub const WM5100_SPK2_MUTE_ENDIAN_MASK: c_uint = 0x0100  /* SPK2_MUTE_ENDIAN */;

pub const WM5100_SPK2_MUTE_SEQ1_MASK: c_uint = 0x00FF  /* SPK2_MUTE_SEQ1 - [7:0] */;

//
// R1219 (0x4C3) - PDM SPK2 CTRL 2
//
pub const WM5100_SPK2_FMT: c_uint = 0x0001  /* SPK2_FMT */;
pub const WM5100_SPK2_FMT_MASK: c_uint = 0x0001  /* SPK2_FMT */;

//
// R1280 (0x500) - Audio IF 1_1
//
pub const WM5100_AIF1_BCLK_INV: c_uint = 0x0080  /* AIF1_BCLK_INV */;
pub const WM5100_AIF1_BCLK_INV_MASK: c_uint = 0x0080  /* AIF1_BCLK_INV */;

pub const WM5100_AIF1_BCLK_FRC: c_uint = 0x0040  /* AIF1_BCLK_FRC */;
pub const WM5100_AIF1_BCLK_FRC_MASK: c_uint = 0x0040  /* AIF1_BCLK_FRC */;

pub const WM5100_AIF1_BCLK_MSTR: c_uint = 0x0020  /* AIF1_BCLK_MSTR */;
pub const WM5100_AIF1_BCLK_MSTR_MASK: c_uint = 0x0020  /* AIF1_BCLK_MSTR */;

pub const WM5100_AIF1_BCLK_FREQ_MASK: c_uint = 0x001F  /* AIF1_BCLK_FREQ - [4:0] */;

//
// R1281 (0x501) - Audio IF 1_2
//
pub const WM5100_AIF1TX_DAT_TRI: c_uint = 0x0020  /* AIF1TX_DAT_TRI */;
pub const WM5100_AIF1TX_DAT_TRI_MASK: c_uint = 0x0020  /* AIF1TX_DAT_TRI */;

pub const WM5100_AIF1TX_LRCLK_SRC: c_uint = 0x0008  /* AIF1TX_LRCLK_SRC */;
pub const WM5100_AIF1TX_LRCLK_SRC_MASK: c_uint = 0x0008  /* AIF1TX_LRCLK_SRC */;

pub const WM5100_AIF1TX_LRCLK_INV: c_uint = 0x0004  /* AIF1TX_LRCLK_INV */;
pub const WM5100_AIF1TX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF1TX_LRCLK_INV */;

pub const WM5100_AIF1TX_LRCLK_FRC: c_uint = 0x0002  /* AIF1TX_LRCLK_FRC */;
pub const WM5100_AIF1TX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF1TX_LRCLK_FRC */;

pub const WM5100_AIF1TX_LRCLK_MSTR: c_uint = 0x0001  /* AIF1TX_LRCLK_MSTR */;
pub const WM5100_AIF1TX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF1TX_LRCLK_MSTR */;

//
// R1282 (0x502) - Audio IF 1_3
//
pub const WM5100_AIF1RX_LRCLK_INV: c_uint = 0x0004  /* AIF1RX_LRCLK_INV */;
pub const WM5100_AIF1RX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF1RX_LRCLK_INV */;

pub const WM5100_AIF1RX_LRCLK_FRC: c_uint = 0x0002  /* AIF1RX_LRCLK_FRC */;
pub const WM5100_AIF1RX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF1RX_LRCLK_FRC */;

pub const WM5100_AIF1RX_LRCLK_MSTR: c_uint = 0x0001  /* AIF1RX_LRCLK_MSTR */;
pub const WM5100_AIF1RX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF1RX_LRCLK_MSTR */;

//
// R1283 (0x503) - Audio IF 1_4
//
pub const WM5100_AIF1_TRI: c_uint = 0x0040  /* AIF1_TRI */;
pub const WM5100_AIF1_TRI_MASK: c_uint = 0x0040  /* AIF1_TRI */;

pub const WM5100_AIF1_RATE_MASK: c_uint = 0x0003  /* AIF1_RATE - [1:0] */;

//
// R1284 (0x504) - Audio IF 1_5
//
pub const WM5100_AIF1_FMT_MASK: c_uint = 0x0007  /* AIF1_FMT - [2:0] */;

//
// R1285 (0x505) - Audio IF 1_6
//
pub const WM5100_AIF1TX_BCPF_MASK: c_uint = 0x1FFF  /* AIF1TX_BCPF - [12:0] */;

//
// R1286 (0x506) - Audio IF 1_7
//
pub const WM5100_AIF1RX_BCPF_MASK: c_uint = 0x1FFF  /* AIF1RX_BCPF - [12:0] */;

//
// R1287 (0x507) - Audio IF 1_8
//
pub const WM5100_AIF1TX_WL_MASK: c_uint = 0x3F00  /* AIF1TX_WL - [13:8] */;

pub const WM5100_AIF1TX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF1TX_SLOT_LEN - [7:0] */;

//
// R1288 (0x508) - Audio IF 1_9
//
pub const WM5100_AIF1RX_WL_MASK: c_uint = 0x3F00  /* AIF1RX_WL - [13:8] */;

pub const WM5100_AIF1RX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF1RX_SLOT_LEN - [7:0] */;

//
// R1289 (0x509) - Audio IF 1_10
//
pub const WM5100_AIF1TX1_SLOT_MASK: c_uint = 0x003F  /* AIF1TX1_SLOT - [5:0] */;

//
// R1290 (0x50A) - Audio IF 1_11
//
pub const WM5100_AIF1TX2_SLOT_MASK: c_uint = 0x003F  /* AIF1TX2_SLOT - [5:0] */;

//
// R1291 (0x50B) - Audio IF 1_12
//
pub const WM5100_AIF1TX3_SLOT_MASK: c_uint = 0x003F  /* AIF1TX3_SLOT - [5:0] */;

//
// R1292 (0x50C) - Audio IF 1_13
//
pub const WM5100_AIF1TX4_SLOT_MASK: c_uint = 0x003F  /* AIF1TX4_SLOT - [5:0] */;

//
// R1293 (0x50D) - Audio IF 1_14
//
pub const WM5100_AIF1TX5_SLOT_MASK: c_uint = 0x003F  /* AIF1TX5_SLOT - [5:0] */;

//
// R1294 (0x50E) - Audio IF 1_15
//
pub const WM5100_AIF1TX6_SLOT_MASK: c_uint = 0x003F  /* AIF1TX6_SLOT - [5:0] */;

//
// R1295 (0x50F) - Audio IF 1_16
//
pub const WM5100_AIF1TX7_SLOT_MASK: c_uint = 0x003F  /* AIF1TX7_SLOT - [5:0] */;

//
// R1296 (0x510) - Audio IF 1_17
//
pub const WM5100_AIF1TX8_SLOT_MASK: c_uint = 0x003F  /* AIF1TX8_SLOT - [5:0] */;

//
// R1297 (0x511) - Audio IF 1_18
//
pub const WM5100_AIF1RX1_SLOT_MASK: c_uint = 0x003F  /* AIF1RX1_SLOT - [5:0] */;

//
// R1298 (0x512) - Audio IF 1_19
//
pub const WM5100_AIF1RX2_SLOT_MASK: c_uint = 0x003F  /* AIF1RX2_SLOT - [5:0] */;

//
// R1299 (0x513) - Audio IF 1_20
//
pub const WM5100_AIF1RX3_SLOT_MASK: c_uint = 0x003F  /* AIF1RX3_SLOT - [5:0] */;

//
// R1300 (0x514) - Audio IF 1_21
//
pub const WM5100_AIF1RX4_SLOT_MASK: c_uint = 0x003F  /* AIF1RX4_SLOT - [5:0] */;

//
// R1301 (0x515) - Audio IF 1_22
//
pub const WM5100_AIF1RX5_SLOT_MASK: c_uint = 0x003F  /* AIF1RX5_SLOT - [5:0] */;

//
// R1302 (0x516) - Audio IF 1_23
//
pub const WM5100_AIF1RX6_SLOT_MASK: c_uint = 0x003F  /* AIF1RX6_SLOT - [5:0] */;

//
// R1303 (0x517) - Audio IF 1_24
//
pub const WM5100_AIF1RX7_SLOT_MASK: c_uint = 0x003F  /* AIF1RX7_SLOT - [5:0] */;

//
// R1304 (0x518) - Audio IF 1_25
//
pub const WM5100_AIF1RX8_SLOT_MASK: c_uint = 0x003F  /* AIF1RX8_SLOT - [5:0] */;

//
// R1305 (0x519) - Audio IF 1_26
//
pub const WM5100_AIF1TX8_ENA: c_uint = 0x0080  /* AIF1TX8_ENA */;
pub const WM5100_AIF1TX8_ENA_MASK: c_uint = 0x0080  /* AIF1TX8_ENA */;

pub const WM5100_AIF1TX7_ENA: c_uint = 0x0040  /* AIF1TX7_ENA */;
pub const WM5100_AIF1TX7_ENA_MASK: c_uint = 0x0040  /* AIF1TX7_ENA */;

pub const WM5100_AIF1TX6_ENA: c_uint = 0x0020  /* AIF1TX6_ENA */;
pub const WM5100_AIF1TX6_ENA_MASK: c_uint = 0x0020  /* AIF1TX6_ENA */;

pub const WM5100_AIF1TX5_ENA: c_uint = 0x0010  /* AIF1TX5_ENA */;
pub const WM5100_AIF1TX5_ENA_MASK: c_uint = 0x0010  /* AIF1TX5_ENA */;

pub const WM5100_AIF1TX4_ENA: c_uint = 0x0008  /* AIF1TX4_ENA */;
pub const WM5100_AIF1TX4_ENA_MASK: c_uint = 0x0008  /* AIF1TX4_ENA */;

pub const WM5100_AIF1TX3_ENA: c_uint = 0x0004  /* AIF1TX3_ENA */;
pub const WM5100_AIF1TX3_ENA_MASK: c_uint = 0x0004  /* AIF1TX3_ENA */;

pub const WM5100_AIF1TX2_ENA: c_uint = 0x0002  /* AIF1TX2_ENA */;
pub const WM5100_AIF1TX2_ENA_MASK: c_uint = 0x0002  /* AIF1TX2_ENA */;

pub const WM5100_AIF1TX1_ENA: c_uint = 0x0001  /* AIF1TX1_ENA */;
pub const WM5100_AIF1TX1_ENA_MASK: c_uint = 0x0001  /* AIF1TX1_ENA */;

//
// R1306 (0x51A) - Audio IF 1_27
//
pub const WM5100_AIF1RX8_ENA: c_uint = 0x0080  /* AIF1RX8_ENA */;
pub const WM5100_AIF1RX8_ENA_MASK: c_uint = 0x0080  /* AIF1RX8_ENA */;

pub const WM5100_AIF1RX7_ENA: c_uint = 0x0040  /* AIF1RX7_ENA */;
pub const WM5100_AIF1RX7_ENA_MASK: c_uint = 0x0040  /* AIF1RX7_ENA */;

pub const WM5100_AIF1RX6_ENA: c_uint = 0x0020  /* AIF1RX6_ENA */;
pub const WM5100_AIF1RX6_ENA_MASK: c_uint = 0x0020  /* AIF1RX6_ENA */;

pub const WM5100_AIF1RX5_ENA: c_uint = 0x0010  /* AIF1RX5_ENA */;
pub const WM5100_AIF1RX5_ENA_MASK: c_uint = 0x0010  /* AIF1RX5_ENA */;

pub const WM5100_AIF1RX4_ENA: c_uint = 0x0008  /* AIF1RX4_ENA */;
pub const WM5100_AIF1RX4_ENA_MASK: c_uint = 0x0008  /* AIF1RX4_ENA */;

pub const WM5100_AIF1RX3_ENA: c_uint = 0x0004  /* AIF1RX3_ENA */;
pub const WM5100_AIF1RX3_ENA_MASK: c_uint = 0x0004  /* AIF1RX3_ENA */;

pub const WM5100_AIF1RX2_ENA: c_uint = 0x0002  /* AIF1RX2_ENA */;
pub const WM5100_AIF1RX2_ENA_MASK: c_uint = 0x0002  /* AIF1RX2_ENA */;

pub const WM5100_AIF1RX1_ENA: c_uint = 0x0001  /* AIF1RX1_ENA */;
pub const WM5100_AIF1RX1_ENA_MASK: c_uint = 0x0001  /* AIF1RX1_ENA */;

//
// R1344 (0x540) - Audio IF 2_1
//
pub const WM5100_AIF2_BCLK_INV: c_uint = 0x0080  /* AIF2_BCLK_INV */;
pub const WM5100_AIF2_BCLK_INV_MASK: c_uint = 0x0080  /* AIF2_BCLK_INV */;

pub const WM5100_AIF2_BCLK_FRC: c_uint = 0x0040  /* AIF2_BCLK_FRC */;
pub const WM5100_AIF2_BCLK_FRC_MASK: c_uint = 0x0040  /* AIF2_BCLK_FRC */;

pub const WM5100_AIF2_BCLK_MSTR: c_uint = 0x0020  /* AIF2_BCLK_MSTR */;
pub const WM5100_AIF2_BCLK_MSTR_MASK: c_uint = 0x0020  /* AIF2_BCLK_MSTR */;

pub const WM5100_AIF2_BCLK_FREQ_MASK: c_uint = 0x001F  /* AIF2_BCLK_FREQ - [4:0] */;

//
// R1345 (0x541) - Audio IF 2_2
//
pub const WM5100_AIF2TX_DAT_TRI: c_uint = 0x0020  /* AIF2TX_DAT_TRI */;
pub const WM5100_AIF2TX_DAT_TRI_MASK: c_uint = 0x0020  /* AIF2TX_DAT_TRI */;

pub const WM5100_AIF2TX_LRCLK_SRC: c_uint = 0x0008  /* AIF2TX_LRCLK_SRC */;
pub const WM5100_AIF2TX_LRCLK_SRC_MASK: c_uint = 0x0008  /* AIF2TX_LRCLK_SRC */;

pub const WM5100_AIF2TX_LRCLK_INV: c_uint = 0x0004  /* AIF2TX_LRCLK_INV */;
pub const WM5100_AIF2TX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF2TX_LRCLK_INV */;

pub const WM5100_AIF2TX_LRCLK_FRC: c_uint = 0x0002  /* AIF2TX_LRCLK_FRC */;
pub const WM5100_AIF2TX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF2TX_LRCLK_FRC */;

pub const WM5100_AIF2TX_LRCLK_MSTR: c_uint = 0x0001  /* AIF2TX_LRCLK_MSTR */;
pub const WM5100_AIF2TX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF2TX_LRCLK_MSTR */;

//
// R1346 (0x542) - Audio IF 2_3
//
pub const WM5100_AIF2RX_LRCLK_INV: c_uint = 0x0004  /* AIF2RX_LRCLK_INV */;
pub const WM5100_AIF2RX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF2RX_LRCLK_INV */;

pub const WM5100_AIF2RX_LRCLK_FRC: c_uint = 0x0002  /* AIF2RX_LRCLK_FRC */;
pub const WM5100_AIF2RX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF2RX_LRCLK_FRC */;

pub const WM5100_AIF2RX_LRCLK_MSTR: c_uint = 0x0001  /* AIF2RX_LRCLK_MSTR */;
pub const WM5100_AIF2RX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF2RX_LRCLK_MSTR */;

//
// R1347 (0x543) - Audio IF 2_4
//
pub const WM5100_AIF2_TRI: c_uint = 0x0040  /* AIF2_TRI */;
pub const WM5100_AIF2_TRI_MASK: c_uint = 0x0040  /* AIF2_TRI */;

pub const WM5100_AIF2_RATE_MASK: c_uint = 0x0003  /* AIF2_RATE - [1:0] */;

//
// R1348 (0x544) - Audio IF 2_5
//
pub const WM5100_AIF2_FMT_MASK: c_uint = 0x0007  /* AIF2_FMT - [2:0] */;

//
// R1349 (0x545) - Audio IF 2_6
//
pub const WM5100_AIF2TX_BCPF_MASK: c_uint = 0x1FFF  /* AIF2TX_BCPF - [12:0] */;

//
// R1350 (0x546) - Audio IF 2_7
//
pub const WM5100_AIF2RX_BCPF_MASK: c_uint = 0x1FFF  /* AIF2RX_BCPF - [12:0] */;

//
// R1351 (0x547) - Audio IF 2_8
//
pub const WM5100_AIF2TX_WL_MASK: c_uint = 0x3F00  /* AIF2TX_WL - [13:8] */;

pub const WM5100_AIF2TX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF2TX_SLOT_LEN - [7:0] */;

//
// R1352 (0x548) - Audio IF 2_9
//
pub const WM5100_AIF2RX_WL_MASK: c_uint = 0x3F00  /* AIF2RX_WL - [13:8] */;

pub const WM5100_AIF2RX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF2RX_SLOT_LEN - [7:0] */;

//
// R1353 (0x549) - Audio IF 2_10
//
pub const WM5100_AIF2TX1_SLOT_MASK: c_uint = 0x003F  /* AIF2TX1_SLOT - [5:0] */;

//
// R1354 (0x54A) - Audio IF 2_11
//
pub const WM5100_AIF2TX2_SLOT_MASK: c_uint = 0x003F  /* AIF2TX2_SLOT - [5:0] */;

//
// R1361 (0x551) - Audio IF 2_18
//
pub const WM5100_AIF2RX1_SLOT_MASK: c_uint = 0x003F  /* AIF2RX1_SLOT - [5:0] */;

//
// R1362 (0x552) - Audio IF 2_19
//
pub const WM5100_AIF2RX2_SLOT_MASK: c_uint = 0x003F  /* AIF2RX2_SLOT - [5:0] */;

//
// R1369 (0x559) - Audio IF 2_26
//
pub const WM5100_AIF2TX2_ENA: c_uint = 0x0002  /* AIF2TX2_ENA */;
pub const WM5100_AIF2TX2_ENA_MASK: c_uint = 0x0002  /* AIF2TX2_ENA */;

pub const WM5100_AIF2TX1_ENA: c_uint = 0x0001  /* AIF2TX1_ENA */;
pub const WM5100_AIF2TX1_ENA_MASK: c_uint = 0x0001  /* AIF2TX1_ENA */;

//
// R1370 (0x55A) - Audio IF 2_27
//
pub const WM5100_AIF2RX2_ENA: c_uint = 0x0002  /* AIF2RX2_ENA */;
pub const WM5100_AIF2RX2_ENA_MASK: c_uint = 0x0002  /* AIF2RX2_ENA */;

pub const WM5100_AIF2RX1_ENA: c_uint = 0x0001  /* AIF2RX1_ENA */;
pub const WM5100_AIF2RX1_ENA_MASK: c_uint = 0x0001  /* AIF2RX1_ENA */;

//
// R1408 (0x580) - Audio IF 3_1
//
pub const WM5100_AIF3_BCLK_INV: c_uint = 0x0080  /* AIF3_BCLK_INV */;
pub const WM5100_AIF3_BCLK_INV_MASK: c_uint = 0x0080  /* AIF3_BCLK_INV */;

pub const WM5100_AIF3_BCLK_FRC: c_uint = 0x0040  /* AIF3_BCLK_FRC */;
pub const WM5100_AIF3_BCLK_FRC_MASK: c_uint = 0x0040  /* AIF3_BCLK_FRC */;

pub const WM5100_AIF3_BCLK_MSTR: c_uint = 0x0020  /* AIF3_BCLK_MSTR */;
pub const WM5100_AIF3_BCLK_MSTR_MASK: c_uint = 0x0020  /* AIF3_BCLK_MSTR */;

pub const WM5100_AIF3_BCLK_FREQ_MASK: c_uint = 0x001F  /* AIF3_BCLK_FREQ - [4:0] */;

//
// R1409 (0x581) - Audio IF 3_2
//
pub const WM5100_AIF3TX_DAT_TRI: c_uint = 0x0020  /* AIF3TX_DAT_TRI */;
pub const WM5100_AIF3TX_DAT_TRI_MASK: c_uint = 0x0020  /* AIF3TX_DAT_TRI */;

pub const WM5100_AIF3TX_LRCLK_SRC: c_uint = 0x0008  /* AIF3TX_LRCLK_SRC */;
pub const WM5100_AIF3TX_LRCLK_SRC_MASK: c_uint = 0x0008  /* AIF3TX_LRCLK_SRC */;

pub const WM5100_AIF3TX_LRCLK_INV: c_uint = 0x0004  /* AIF3TX_LRCLK_INV */;
pub const WM5100_AIF3TX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF3TX_LRCLK_INV */;

pub const WM5100_AIF3TX_LRCLK_FRC: c_uint = 0x0002  /* AIF3TX_LRCLK_FRC */;
pub const WM5100_AIF3TX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF3TX_LRCLK_FRC */;

pub const WM5100_AIF3TX_LRCLK_MSTR: c_uint = 0x0001  /* AIF3TX_LRCLK_MSTR */;
pub const WM5100_AIF3TX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF3TX_LRCLK_MSTR */;

//
// R1410 (0x582) - Audio IF 3_3
//
pub const WM5100_AIF3RX_LRCLK_INV: c_uint = 0x0004  /* AIF3RX_LRCLK_INV */;
pub const WM5100_AIF3RX_LRCLK_INV_MASK: c_uint = 0x0004  /* AIF3RX_LRCLK_INV */;

pub const WM5100_AIF3RX_LRCLK_FRC: c_uint = 0x0002  /* AIF3RX_LRCLK_FRC */;
pub const WM5100_AIF3RX_LRCLK_FRC_MASK: c_uint = 0x0002  /* AIF3RX_LRCLK_FRC */;

pub const WM5100_AIF3RX_LRCLK_MSTR: c_uint = 0x0001  /* AIF3RX_LRCLK_MSTR */;
pub const WM5100_AIF3RX_LRCLK_MSTR_MASK: c_uint = 0x0001  /* AIF3RX_LRCLK_MSTR */;

//
// R1411 (0x583) - Audio IF 3_4
//
pub const WM5100_AIF3_TRI: c_uint = 0x0040  /* AIF3_TRI */;
pub const WM5100_AIF3_TRI_MASK: c_uint = 0x0040  /* AIF3_TRI */;

pub const WM5100_AIF3_RATE_MASK: c_uint = 0x0003  /* AIF3_RATE - [1:0] */;

//
// R1412 (0x584) - Audio IF 3_5
//
pub const WM5100_AIF3_FMT_MASK: c_uint = 0x0007  /* AIF3_FMT - [2:0] */;

//
// R1413 (0x585) - Audio IF 3_6
//
pub const WM5100_AIF3TX_BCPF_MASK: c_uint = 0x1FFF  /* AIF3TX_BCPF - [12:0] */;

//
// R1414 (0x586) - Audio IF 3_7
//
pub const WM5100_AIF3RX_BCPF_MASK: c_uint = 0x1FFF  /* AIF3RX_BCPF - [12:0] */;

//
// R1415 (0x587) - Audio IF 3_8
//
pub const WM5100_AIF3TX_WL_MASK: c_uint = 0x3F00  /* AIF3TX_WL - [13:8] */;

pub const WM5100_AIF3TX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF3TX_SLOT_LEN - [7:0] */;

//
// R1416 (0x588) - Audio IF 3_9
//
pub const WM5100_AIF3RX_WL_MASK: c_uint = 0x3F00  /* AIF3RX_WL - [13:8] */;

pub const WM5100_AIF3RX_SLOT_LEN_MASK: c_uint = 0x00FF  /* AIF3RX_SLOT_LEN - [7:0] */;

//
// R1417 (0x589) - Audio IF 3_10
//
pub const WM5100_AIF3TX1_SLOT_MASK: c_uint = 0x003F  /* AIF3TX1_SLOT - [5:0] */;

//
// R1418 (0x58A) - Audio IF 3_11
//
pub const WM5100_AIF3TX2_SLOT_MASK: c_uint = 0x003F  /* AIF3TX2_SLOT - [5:0] */;

//
// R1425 (0x591) - Audio IF 3_18
//
pub const WM5100_AIF3RX1_SLOT_MASK: c_uint = 0x003F  /* AIF3RX1_SLOT - [5:0] */;

//
// R1426 (0x592) - Audio IF 3_19
//
pub const WM5100_AIF3RX2_SLOT_MASK: c_uint = 0x003F  /* AIF3RX2_SLOT - [5:0] */;

//
// R1433 (0x599) - Audio IF 3_26
//
pub const WM5100_AIF3TX2_ENA: c_uint = 0x0002  /* AIF3TX2_ENA */;
pub const WM5100_AIF3TX2_ENA_MASK: c_uint = 0x0002  /* AIF3TX2_ENA */;

pub const WM5100_AIF3TX1_ENA: c_uint = 0x0001  /* AIF3TX1_ENA */;
pub const WM5100_AIF3TX1_ENA_MASK: c_uint = 0x0001  /* AIF3TX1_ENA */;

//
// R1434 (0x59A) - Audio IF 3_27
//
pub const WM5100_AIF3RX2_ENA: c_uint = 0x0002  /* AIF3RX2_ENA */;
pub const WM5100_AIF3RX2_ENA_MASK: c_uint = 0x0002  /* AIF3RX2_ENA */;

pub const WM5100_AIF3RX1_ENA: c_uint = 0x0001  /* AIF3RX1_ENA */;
pub const WM5100_AIF3RX1_ENA_MASK: c_uint = 0x0001  /* AIF3RX1_ENA */;

pub const WM5100_MIXER_VOL_MASK: c_uint = 0x00FE  /* MIXER_VOL - [7:1] */;

//
// R3072 (0xC00) - GPIO CTRL 1
//
pub const WM5100_GP1_DIR: c_uint = 0x8000  /* GP1_DIR */;
pub const WM5100_GP1_DIR_MASK: c_uint = 0x8000  /* GP1_DIR */;

pub const WM5100_GP1_PU: c_uint = 0x4000  /* GP1_PU */;
pub const WM5100_GP1_PU_MASK: c_uint = 0x4000  /* GP1_PU */;

pub const WM5100_GP1_PD: c_uint = 0x2000  /* GP1_PD */;
pub const WM5100_GP1_PD_MASK: c_uint = 0x2000  /* GP1_PD */;

pub const WM5100_GP1_POL: c_uint = 0x0400  /* GP1_POL */;
pub const WM5100_GP1_POL_MASK: c_uint = 0x0400  /* GP1_POL */;

pub const WM5100_GP1_OP_CFG: c_uint = 0x0200  /* GP1_OP_CFG */;
pub const WM5100_GP1_OP_CFG_MASK: c_uint = 0x0200  /* GP1_OP_CFG */;

pub const WM5100_GP1_DB: c_uint = 0x0100  /* GP1_DB */;
pub const WM5100_GP1_DB_MASK: c_uint = 0x0100  /* GP1_DB */;

pub const WM5100_GP1_LVL: c_uint = 0x0040  /* GP1_LVL */;
pub const WM5100_GP1_LVL_MASK: c_uint = 0x0040  /* GP1_LVL */;

pub const WM5100_GP1_FN_MASK: c_uint = 0x003F  /* GP1_FN - [5:0] */;

//
// R3073 (0xC01) - GPIO CTRL 2
//
pub const WM5100_GP2_DIR: c_uint = 0x8000  /* GP2_DIR */;
pub const WM5100_GP2_DIR_MASK: c_uint = 0x8000  /* GP2_DIR */;

pub const WM5100_GP2_PU: c_uint = 0x4000  /* GP2_PU */;
pub const WM5100_GP2_PU_MASK: c_uint = 0x4000  /* GP2_PU */;

pub const WM5100_GP2_PD: c_uint = 0x2000  /* GP2_PD */;
pub const WM5100_GP2_PD_MASK: c_uint = 0x2000  /* GP2_PD */;

pub const WM5100_GP2_POL: c_uint = 0x0400  /* GP2_POL */;
pub const WM5100_GP2_POL_MASK: c_uint = 0x0400  /* GP2_POL */;

pub const WM5100_GP2_OP_CFG: c_uint = 0x0200  /* GP2_OP_CFG */;
pub const WM5100_GP2_OP_CFG_MASK: c_uint = 0x0200  /* GP2_OP_CFG */;

pub const WM5100_GP2_DB: c_uint = 0x0100  /* GP2_DB */;
pub const WM5100_GP2_DB_MASK: c_uint = 0x0100  /* GP2_DB */;

pub const WM5100_GP2_LVL: c_uint = 0x0040  /* GP2_LVL */;
pub const WM5100_GP2_LVL_MASK: c_uint = 0x0040  /* GP2_LVL */;

pub const WM5100_GP2_FN_MASK: c_uint = 0x003F  /* GP2_FN - [5:0] */;

//
// R3074 (0xC02) - GPIO CTRL 3
//
pub const WM5100_GP3_DIR: c_uint = 0x8000  /* GP3_DIR */;
pub const WM5100_GP3_DIR_MASK: c_uint = 0x8000  /* GP3_DIR */;

pub const WM5100_GP3_PU: c_uint = 0x4000  /* GP3_PU */;
pub const WM5100_GP3_PU_MASK: c_uint = 0x4000  /* GP3_PU */;

pub const WM5100_GP3_PD: c_uint = 0x2000  /* GP3_PD */;
pub const WM5100_GP3_PD_MASK: c_uint = 0x2000  /* GP3_PD */;

pub const WM5100_GP3_POL: c_uint = 0x0400  /* GP3_POL */;
pub const WM5100_GP3_POL_MASK: c_uint = 0x0400  /* GP3_POL */;

pub const WM5100_GP3_OP_CFG: c_uint = 0x0200  /* GP3_OP_CFG */;
pub const WM5100_GP3_OP_CFG_MASK: c_uint = 0x0200  /* GP3_OP_CFG */;

pub const WM5100_GP3_DB: c_uint = 0x0100  /* GP3_DB */;
pub const WM5100_GP3_DB_MASK: c_uint = 0x0100  /* GP3_DB */;

pub const WM5100_GP3_LVL: c_uint = 0x0040  /* GP3_LVL */;
pub const WM5100_GP3_LVL_MASK: c_uint = 0x0040  /* GP3_LVL */;

pub const WM5100_GP3_FN_MASK: c_uint = 0x003F  /* GP3_FN - [5:0] */;

//
// R3075 (0xC03) - GPIO CTRL 4
//
pub const WM5100_GP4_DIR: c_uint = 0x8000  /* GP4_DIR */;
pub const WM5100_GP4_DIR_MASK: c_uint = 0x8000  /* GP4_DIR */;

pub const WM5100_GP4_PU: c_uint = 0x4000  /* GP4_PU */;
pub const WM5100_GP4_PU_MASK: c_uint = 0x4000  /* GP4_PU */;

pub const WM5100_GP4_PD: c_uint = 0x2000  /* GP4_PD */;
pub const WM5100_GP4_PD_MASK: c_uint = 0x2000  /* GP4_PD */;

pub const WM5100_GP4_POL: c_uint = 0x0400  /* GP4_POL */;
pub const WM5100_GP4_POL_MASK: c_uint = 0x0400  /* GP4_POL */;

pub const WM5100_GP4_OP_CFG: c_uint = 0x0200  /* GP4_OP_CFG */;
pub const WM5100_GP4_OP_CFG_MASK: c_uint = 0x0200  /* GP4_OP_CFG */;

pub const WM5100_GP4_DB: c_uint = 0x0100  /* GP4_DB */;
pub const WM5100_GP4_DB_MASK: c_uint = 0x0100  /* GP4_DB */;

pub const WM5100_GP4_LVL: c_uint = 0x0040  /* GP4_LVL */;
pub const WM5100_GP4_LVL_MASK: c_uint = 0x0040  /* GP4_LVL */;

pub const WM5100_GP4_FN_MASK: c_uint = 0x003F  /* GP4_FN - [5:0] */;

//
// R3076 (0xC04) - GPIO CTRL 5
//
pub const WM5100_GP5_DIR: c_uint = 0x8000  /* GP5_DIR */;
pub const WM5100_GP5_DIR_MASK: c_uint = 0x8000  /* GP5_DIR */;

pub const WM5100_GP5_PU: c_uint = 0x4000  /* GP5_PU */;
pub const WM5100_GP5_PU_MASK: c_uint = 0x4000  /* GP5_PU */;

pub const WM5100_GP5_PD: c_uint = 0x2000  /* GP5_PD */;
pub const WM5100_GP5_PD_MASK: c_uint = 0x2000  /* GP5_PD */;

pub const WM5100_GP5_POL: c_uint = 0x0400  /* GP5_POL */;
pub const WM5100_GP5_POL_MASK: c_uint = 0x0400  /* GP5_POL */;

pub const WM5100_GP5_OP_CFG: c_uint = 0x0200  /* GP5_OP_CFG */;
pub const WM5100_GP5_OP_CFG_MASK: c_uint = 0x0200  /* GP5_OP_CFG */;

pub const WM5100_GP5_DB: c_uint = 0x0100  /* GP5_DB */;
pub const WM5100_GP5_DB_MASK: c_uint = 0x0100  /* GP5_DB */;

pub const WM5100_GP5_LVL: c_uint = 0x0040  /* GP5_LVL */;
pub const WM5100_GP5_LVL_MASK: c_uint = 0x0040  /* GP5_LVL */;

pub const WM5100_GP5_FN_MASK: c_uint = 0x003F  /* GP5_FN - [5:0] */;

//
// R3077 (0xC05) - GPIO CTRL 6
//
pub const WM5100_GP6_DIR: c_uint = 0x8000  /* GP6_DIR */;
pub const WM5100_GP6_DIR_MASK: c_uint = 0x8000  /* GP6_DIR */;

pub const WM5100_GP6_PU: c_uint = 0x4000  /* GP6_PU */;
pub const WM5100_GP6_PU_MASK: c_uint = 0x4000  /* GP6_PU */;

pub const WM5100_GP6_PD: c_uint = 0x2000  /* GP6_PD */;
pub const WM5100_GP6_PD_MASK: c_uint = 0x2000  /* GP6_PD */;

pub const WM5100_GP6_POL: c_uint = 0x0400  /* GP6_POL */;
pub const WM5100_GP6_POL_MASK: c_uint = 0x0400  /* GP6_POL */;

pub const WM5100_GP6_OP_CFG: c_uint = 0x0200  /* GP6_OP_CFG */;
pub const WM5100_GP6_OP_CFG_MASK: c_uint = 0x0200  /* GP6_OP_CFG */;

pub const WM5100_GP6_DB: c_uint = 0x0100  /* GP6_DB */;
pub const WM5100_GP6_DB_MASK: c_uint = 0x0100  /* GP6_DB */;

pub const WM5100_GP6_LVL: c_uint = 0x0040  /* GP6_LVL */;
pub const WM5100_GP6_LVL_MASK: c_uint = 0x0040  /* GP6_LVL */;

pub const WM5100_GP6_FN_MASK: c_uint = 0x003F  /* GP6_FN - [5:0] */;

//
// R3107 (0xC23) - Misc Pad Ctrl 1
//
pub const WM5100_LDO1ENA_PD: c_uint = 0x8000  /* LDO1ENA_PD */;
pub const WM5100_LDO1ENA_PD_MASK: c_uint = 0x8000  /* LDO1ENA_PD */;

pub const WM5100_MCLK2_PD: c_uint = 0x2000  /* MCLK2_PD */;
pub const WM5100_MCLK2_PD_MASK: c_uint = 0x2000  /* MCLK2_PD */;

pub const WM5100_MCLK1_PD: c_uint = 0x1000  /* MCLK1_PD */;
pub const WM5100_MCLK1_PD_MASK: c_uint = 0x1000  /* MCLK1_PD */;

pub const WM5100_RESET_PU: c_uint = 0x0002  /* RESET_PU */;
pub const WM5100_RESET_PU_MASK: c_uint = 0x0002  /* RESET_PU */;

pub const WM5100_ADDR_PD: c_uint = 0x0001  /* ADDR_PD */;
pub const WM5100_ADDR_PD_MASK: c_uint = 0x0001  /* ADDR_PD */;

//
// R3108 (0xC24) - Misc Pad Ctrl 2
//
pub const WM5100_DMICDAT4_PD: c_uint = 0x0008  /* DMICDAT4_PD */;
pub const WM5100_DMICDAT4_PD_MASK: c_uint = 0x0008  /* DMICDAT4_PD */;

pub const WM5100_DMICDAT3_PD: c_uint = 0x0004  /* DMICDAT3_PD */;
pub const WM5100_DMICDAT3_PD_MASK: c_uint = 0x0004  /* DMICDAT3_PD */;

pub const WM5100_DMICDAT2_PD: c_uint = 0x0002  /* DMICDAT2_PD */;
pub const WM5100_DMICDAT2_PD_MASK: c_uint = 0x0002  /* DMICDAT2_PD */;

pub const WM5100_DMICDAT1_PD: c_uint = 0x0001  /* DMICDAT1_PD */;
pub const WM5100_DMICDAT1_PD_MASK: c_uint = 0x0001  /* DMICDAT1_PD */;

//
// R3109 (0xC25) - Misc Pad Ctrl 3
//
pub const WM5100_AIF1RXLRCLK_PU: c_uint = 0x0020  /* AIF1RXLRCLK_PU */;
pub const WM5100_AIF1RXLRCLK_PU_MASK: c_uint = 0x0020  /* AIF1RXLRCLK_PU */;

pub const WM5100_AIF1RXLRCLK_PD: c_uint = 0x0010  /* AIF1RXLRCLK_PD */;
pub const WM5100_AIF1RXLRCLK_PD_MASK: c_uint = 0x0010  /* AIF1RXLRCLK_PD */;

pub const WM5100_AIF1BCLK_PU: c_uint = 0x0008  /* AIF1BCLK_PU */;
pub const WM5100_AIF1BCLK_PU_MASK: c_uint = 0x0008  /* AIF1BCLK_PU */;

pub const WM5100_AIF1BCLK_PD: c_uint = 0x0004  /* AIF1BCLK_PD */;
pub const WM5100_AIF1BCLK_PD_MASK: c_uint = 0x0004  /* AIF1BCLK_PD */;

pub const WM5100_AIF1RXDAT_PU: c_uint = 0x0002  /* AIF1RXDAT_PU */;
pub const WM5100_AIF1RXDAT_PU_MASK: c_uint = 0x0002  /* AIF1RXDAT_PU */;

pub const WM5100_AIF1RXDAT_PD: c_uint = 0x0001  /* AIF1RXDAT_PD */;
pub const WM5100_AIF1RXDAT_PD_MASK: c_uint = 0x0001  /* AIF1RXDAT_PD */;

//
// R3110 (0xC26) - Misc Pad Ctrl 4
//
pub const WM5100_AIF2RXLRCLK_PU: c_uint = 0x0020  /* AIF2RXLRCLK_PU */;
pub const WM5100_AIF2RXLRCLK_PU_MASK: c_uint = 0x0020  /* AIF2RXLRCLK_PU */;

pub const WM5100_AIF2RXLRCLK_PD: c_uint = 0x0010  /* AIF2RXLRCLK_PD */;
pub const WM5100_AIF2RXLRCLK_PD_MASK: c_uint = 0x0010  /* AIF2RXLRCLK_PD */;

pub const WM5100_AIF2BCLK_PU: c_uint = 0x0008  /* AIF2BCLK_PU */;
pub const WM5100_AIF2BCLK_PU_MASK: c_uint = 0x0008  /* AIF2BCLK_PU */;

pub const WM5100_AIF2BCLK_PD: c_uint = 0x0004  /* AIF2BCLK_PD */;
pub const WM5100_AIF2BCLK_PD_MASK: c_uint = 0x0004  /* AIF2BCLK_PD */;

pub const WM5100_AIF2RXDAT_PU: c_uint = 0x0002  /* AIF2RXDAT_PU */;
pub const WM5100_AIF2RXDAT_PU_MASK: c_uint = 0x0002  /* AIF2RXDAT_PU */;

pub const WM5100_AIF2RXDAT_PD: c_uint = 0x0001  /* AIF2RXDAT_PD */;
pub const WM5100_AIF2RXDAT_PD_MASK: c_uint = 0x0001  /* AIF2RXDAT_PD */;

//
// R3111 (0xC27) - Misc Pad Ctrl 5
//
pub const WM5100_AIF3RXLRCLK_PU: c_uint = 0x0020  /* AIF3RXLRCLK_PU */;
pub const WM5100_AIF3RXLRCLK_PU_MASK: c_uint = 0x0020  /* AIF3RXLRCLK_PU */;

pub const WM5100_AIF3RXLRCLK_PD: c_uint = 0x0010  /* AIF3RXLRCLK_PD */;
pub const WM5100_AIF3RXLRCLK_PD_MASK: c_uint = 0x0010  /* AIF3RXLRCLK_PD */;

pub const WM5100_AIF3BCLK_PU: c_uint = 0x0008  /* AIF3BCLK_PU */;
pub const WM5100_AIF3BCLK_PU_MASK: c_uint = 0x0008  /* AIF3BCLK_PU */;

pub const WM5100_AIF3BCLK_PD: c_uint = 0x0004  /* AIF3BCLK_PD */;
pub const WM5100_AIF3BCLK_PD_MASK: c_uint = 0x0004  /* AIF3BCLK_PD */;

pub const WM5100_AIF3RXDAT_PU: c_uint = 0x0002  /* AIF3RXDAT_PU */;
pub const WM5100_AIF3RXDAT_PU_MASK: c_uint = 0x0002  /* AIF3RXDAT_PU */;

pub const WM5100_AIF3RXDAT_PD: c_uint = 0x0001  /* AIF3RXDAT_PD */;
pub const WM5100_AIF3RXDAT_PD_MASK: c_uint = 0x0001  /* AIF3RXDAT_PD */;

//
// R3112 (0xC28) - Misc GPIO 1
//
pub const WM5100_OPCLK_SEL_MASK: c_uint = 0x0003  /* OPCLK_SEL - [1:0] */;

//
// R3328 (0xD00) - Interrupt Status 1
//
pub const WM5100_GP6_EINT: c_uint = 0x0020  /* GP6_EINT */;
pub const WM5100_GP6_EINT_MASK: c_uint = 0x0020  /* GP6_EINT */;

pub const WM5100_GP5_EINT: c_uint = 0x0010  /* GP5_EINT */;
pub const WM5100_GP5_EINT_MASK: c_uint = 0x0010  /* GP5_EINT */;

pub const WM5100_GP4_EINT: c_uint = 0x0008  /* GP4_EINT */;
pub const WM5100_GP4_EINT_MASK: c_uint = 0x0008  /* GP4_EINT */;

pub const WM5100_GP3_EINT: c_uint = 0x0004  /* GP3_EINT */;
pub const WM5100_GP3_EINT_MASK: c_uint = 0x0004  /* GP3_EINT */;

pub const WM5100_GP2_EINT: c_uint = 0x0002  /* GP2_EINT */;
pub const WM5100_GP2_EINT_MASK: c_uint = 0x0002  /* GP2_EINT */;

pub const WM5100_GP1_EINT: c_uint = 0x0001  /* GP1_EINT */;
pub const WM5100_GP1_EINT_MASK: c_uint = 0x0001  /* GP1_EINT */;

//
// R3329 (0xD01) - Interrupt Status 2
//
pub const WM5100_DSP_IRQ6_EINT: c_uint = 0x0020  /* DSP_IRQ6_EINT */;
pub const WM5100_DSP_IRQ6_EINT_MASK: c_uint = 0x0020  /* DSP_IRQ6_EINT */;

pub const WM5100_DSP_IRQ5_EINT: c_uint = 0x0010  /* DSP_IRQ5_EINT */;
pub const WM5100_DSP_IRQ5_EINT_MASK: c_uint = 0x0010  /* DSP_IRQ5_EINT */;

pub const WM5100_DSP_IRQ4_EINT: c_uint = 0x0008  /* DSP_IRQ4_EINT */;
pub const WM5100_DSP_IRQ4_EINT_MASK: c_uint = 0x0008  /* DSP_IRQ4_EINT */;

pub const WM5100_DSP_IRQ3_EINT: c_uint = 0x0004  /* DSP_IRQ3_EINT */;
pub const WM5100_DSP_IRQ3_EINT_MASK: c_uint = 0x0004  /* DSP_IRQ3_EINT */;

pub const WM5100_DSP_IRQ2_EINT: c_uint = 0x0002  /* DSP_IRQ2_EINT */;
pub const WM5100_DSP_IRQ2_EINT_MASK: c_uint = 0x0002  /* DSP_IRQ2_EINT */;

pub const WM5100_DSP_IRQ1_EINT: c_uint = 0x0001  /* DSP_IRQ1_EINT */;
pub const WM5100_DSP_IRQ1_EINT_MASK: c_uint = 0x0001  /* DSP_IRQ1_EINT */;

//
// R3330 (0xD02) - Interrupt Status 3
//
pub const WM5100_SPK_SHUTDOWN_WARN_EINT: c_uint = 0x8000  /* SPK_SHUTDOWN_WARN_EINT */;
pub const WM5100_SPK_SHUTDOWN_WARN_EINT_MASK: c_uint = 0x8000  /* SPK_SHUTDOWN_WARN_EINT */;

pub const WM5100_SPK_SHUTDOWN_EINT: c_uint = 0x4000  /* SPK_SHUTDOWN_EINT */;
pub const WM5100_SPK_SHUTDOWN_EINT_MASK: c_uint = 0x4000  /* SPK_SHUTDOWN_EINT */;

pub const WM5100_HPDET_EINT: c_uint = 0x2000  /* HPDET_EINT */;
pub const WM5100_HPDET_EINT_MASK: c_uint = 0x2000  /* HPDET_EINT */;

pub const WM5100_ACCDET_EINT: c_uint = 0x1000  /* ACCDET_EINT */;
pub const WM5100_ACCDET_EINT_MASK: c_uint = 0x1000  /* ACCDET_EINT */;

pub const WM5100_DRC_SIG_DET_EINT: c_uint = 0x0200  /* DRC_SIG_DET_EINT */;
pub const WM5100_DRC_SIG_DET_EINT_MASK: c_uint = 0x0200  /* DRC_SIG_DET_EINT */;

pub const WM5100_ASRC2_LOCK_EINT: c_uint = 0x0100  /* ASRC2_LOCK_EINT */;
pub const WM5100_ASRC2_LOCK_EINT_MASK: c_uint = 0x0100  /* ASRC2_LOCK_EINT */;

pub const WM5100_ASRC1_LOCK_EINT: c_uint = 0x0080  /* ASRC1_LOCK_EINT */;
pub const WM5100_ASRC1_LOCK_EINT_MASK: c_uint = 0x0080  /* ASRC1_LOCK_EINT */;

pub const WM5100_FLL2_LOCK_EINT: c_uint = 0x0008  /* FLL2_LOCK_EINT */;
pub const WM5100_FLL2_LOCK_EINT_MASK: c_uint = 0x0008  /* FLL2_LOCK_EINT */;

pub const WM5100_FLL1_LOCK_EINT: c_uint = 0x0004  /* FLL1_LOCK_EINT */;
pub const WM5100_FLL1_LOCK_EINT_MASK: c_uint = 0x0004  /* FLL1_LOCK_EINT */;

pub const WM5100_CLKGEN_ERR_EINT: c_uint = 0x0002  /* CLKGEN_ERR_EINT */;
pub const WM5100_CLKGEN_ERR_EINT_MASK: c_uint = 0x0002  /* CLKGEN_ERR_EINT */;

pub const WM5100_CLKGEN_ERR_ASYNC_EINT: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_EINT */;
pub const WM5100_CLKGEN_ERR_ASYNC_EINT_MASK: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_EINT */;

//
// R3331 (0xD03) - Interrupt Status 4
//
pub const WM5100_AIF3_ERR_EINT: c_uint = 0x2000  /* AIF3_ERR_EINT */;
pub const WM5100_AIF3_ERR_EINT_MASK: c_uint = 0x2000  /* AIF3_ERR_EINT */;

pub const WM5100_AIF2_ERR_EINT: c_uint = 0x1000  /* AIF2_ERR_EINT */;
pub const WM5100_AIF2_ERR_EINT_MASK: c_uint = 0x1000  /* AIF2_ERR_EINT */;

pub const WM5100_AIF1_ERR_EINT: c_uint = 0x0800  /* AIF1_ERR_EINT */;
pub const WM5100_AIF1_ERR_EINT_MASK: c_uint = 0x0800  /* AIF1_ERR_EINT */;

pub const WM5100_CTRLIF_ERR_EINT: c_uint = 0x0400  /* CTRLIF_ERR_EINT */;
pub const WM5100_CTRLIF_ERR_EINT_MASK: c_uint = 0x0400  /* CTRLIF_ERR_EINT */;

pub const WM5100_ISRC2_UNDERCLOCKED_EINT: c_uint = 0x0200  /* ISRC2_UNDERCLOCKED_EINT */;
pub const WM5100_ISRC2_UNDERCLOCKED_EINT_MASK: c_uint = 0x0200  /* ISRC2_UNDERCLOCKED_EINT */;

pub const WM5100_ISRC1_UNDERCLOCKED_EINT: c_uint = 0x0100  /* ISRC1_UNDERCLOCKED_EINT */;
pub const WM5100_ISRC1_UNDERCLOCKED_EINT_MASK: c_uint = 0x0100  /* ISRC1_UNDERCLOCKED_EINT */;

pub const WM5100_FX_UNDERCLOCKED_EINT: c_uint = 0x0080  /* FX_UNDERCLOCKED_EINT */;
pub const WM5100_FX_UNDERCLOCKED_EINT_MASK: c_uint = 0x0080  /* FX_UNDERCLOCKED_EINT */;

pub const WM5100_AIF3_UNDERCLOCKED_EINT: c_uint = 0x0040  /* AIF3_UNDERCLOCKED_EINT */;
pub const WM5100_AIF3_UNDERCLOCKED_EINT_MASK: c_uint = 0x0040  /* AIF3_UNDERCLOCKED_EINT */;

pub const WM5100_AIF2_UNDERCLOCKED_EINT: c_uint = 0x0020  /* AIF2_UNDERCLOCKED_EINT */;
pub const WM5100_AIF2_UNDERCLOCKED_EINT_MASK: c_uint = 0x0020  /* AIF2_UNDERCLOCKED_EINT */;

pub const WM5100_AIF1_UNDERCLOCKED_EINT: c_uint = 0x0010  /* AIF1_UNDERCLOCKED_EINT */;
pub const WM5100_AIF1_UNDERCLOCKED_EINT_MASK: c_uint = 0x0010  /* AIF1_UNDERCLOCKED_EINT */;

pub const WM5100_ASRC_UNDERCLOCKED_EINT: c_uint = 0x0008  /* ASRC_UNDERCLOCKED_EINT */;
pub const WM5100_ASRC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0008  /* ASRC_UNDERCLOCKED_EINT */;

pub const WM5100_DAC_UNDERCLOCKED_EINT: c_uint = 0x0004  /* DAC_UNDERCLOCKED_EINT */;
pub const WM5100_DAC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0004  /* DAC_UNDERCLOCKED_EINT */;

pub const WM5100_ADC_UNDERCLOCKED_EINT: c_uint = 0x0002  /* ADC_UNDERCLOCKED_EINT */;
pub const WM5100_ADC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0002  /* ADC_UNDERCLOCKED_EINT */;

pub const WM5100_MIXER_UNDERCLOCKED_EINT: c_uint = 0x0001  /* MIXER_UNDERCLOCKED_EINT */;
pub const WM5100_MIXER_UNDERCLOCKED_EINT_MASK: c_uint = 0x0001  /* MIXER_UNDERCLOCKED_EINT */;

//
// R3332 (0xD04) - Interrupt Raw Status 2
//
pub const WM5100_DSP_IRQ6_STS: c_uint = 0x0020  /* DSP_IRQ6_STS */;
pub const WM5100_DSP_IRQ6_STS_MASK: c_uint = 0x0020  /* DSP_IRQ6_STS */;

pub const WM5100_DSP_IRQ5_STS: c_uint = 0x0010  /* DSP_IRQ5_STS */;
pub const WM5100_DSP_IRQ5_STS_MASK: c_uint = 0x0010  /* DSP_IRQ5_STS */;

pub const WM5100_DSP_IRQ4_STS: c_uint = 0x0008  /* DSP_IRQ4_STS */;
pub const WM5100_DSP_IRQ4_STS_MASK: c_uint = 0x0008  /* DSP_IRQ4_STS */;

pub const WM5100_DSP_IRQ3_STS: c_uint = 0x0004  /* DSP_IRQ3_STS */;
pub const WM5100_DSP_IRQ3_STS_MASK: c_uint = 0x0004  /* DSP_IRQ3_STS */;

pub const WM5100_DSP_IRQ2_STS: c_uint = 0x0002  /* DSP_IRQ2_STS */;
pub const WM5100_DSP_IRQ2_STS_MASK: c_uint = 0x0002  /* DSP_IRQ2_STS */;

pub const WM5100_DSP_IRQ1_STS: c_uint = 0x0001  /* DSP_IRQ1_STS */;
pub const WM5100_DSP_IRQ1_STS_MASK: c_uint = 0x0001  /* DSP_IRQ1_STS */;

//
// R3333 (0xD05) - Interrupt Raw Status 3
//
pub const WM5100_SPK_SHUTDOWN_WARN_STS: c_uint = 0x8000  /* SPK_SHUTDOWN_WARN_STS */;
pub const WM5100_SPK_SHUTDOWN_WARN_STS_MASK: c_uint = 0x8000  /* SPK_SHUTDOWN_WARN_STS */;

pub const WM5100_SPK_SHUTDOWN_STS: c_uint = 0x4000  /* SPK_SHUTDOWN_STS */;
pub const WM5100_SPK_SHUTDOWN_STS_MASK: c_uint = 0x4000  /* SPK_SHUTDOWN_STS */;

pub const WM5100_HPDET_STS: c_uint = 0x2000  /* HPDET_STS */;
pub const WM5100_HPDET_STS_MASK: c_uint = 0x2000  /* HPDET_STS */;

pub const WM5100_DRC_SID_DET_STS: c_uint = 0x0200  /* DRC_SID_DET_STS */;
pub const WM5100_DRC_SID_DET_STS_MASK: c_uint = 0x0200  /* DRC_SID_DET_STS */;

pub const WM5100_ASRC2_LOCK_STS: c_uint = 0x0100  /* ASRC2_LOCK_STS */;
pub const WM5100_ASRC2_LOCK_STS_MASK: c_uint = 0x0100  /* ASRC2_LOCK_STS */;

pub const WM5100_ASRC1_LOCK_STS: c_uint = 0x0080  /* ASRC1_LOCK_STS */;
pub const WM5100_ASRC1_LOCK_STS_MASK: c_uint = 0x0080  /* ASRC1_LOCK_STS */;

pub const WM5100_FLL2_LOCK_STS: c_uint = 0x0008  /* FLL2_LOCK_STS */;
pub const WM5100_FLL2_LOCK_STS_MASK: c_uint = 0x0008  /* FLL2_LOCK_STS */;

pub const WM5100_FLL1_LOCK_STS: c_uint = 0x0004  /* FLL1_LOCK_STS */;
pub const WM5100_FLL1_LOCK_STS_MASK: c_uint = 0x0004  /* FLL1_LOCK_STS */;

pub const WM5100_CLKGEN_ERR_STS: c_uint = 0x0002  /* CLKGEN_ERR_STS */;
pub const WM5100_CLKGEN_ERR_STS_MASK: c_uint = 0x0002  /* CLKGEN_ERR_STS */;

pub const WM5100_CLKGEN_ERR_ASYNC_STS: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_STS */;
pub const WM5100_CLKGEN_ERR_ASYNC_STS_MASK: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_STS */;

//
// R3334 (0xD06) - Interrupt Raw Status 4
//
pub const WM5100_AIF3_ERR_STS: c_uint = 0x2000  /* AIF3_ERR_STS */;
pub const WM5100_AIF3_ERR_STS_MASK: c_uint = 0x2000  /* AIF3_ERR_STS */;

pub const WM5100_AIF2_ERR_STS: c_uint = 0x1000  /* AIF2_ERR_STS */;
pub const WM5100_AIF2_ERR_STS_MASK: c_uint = 0x1000  /* AIF2_ERR_STS */;

pub const WM5100_AIF1_ERR_STS: c_uint = 0x0800  /* AIF1_ERR_STS */;
pub const WM5100_AIF1_ERR_STS_MASK: c_uint = 0x0800  /* AIF1_ERR_STS */;

pub const WM5100_CTRLIF_ERR_STS: c_uint = 0x0400  /* CTRLIF_ERR_STS */;
pub const WM5100_CTRLIF_ERR_STS_MASK: c_uint = 0x0400  /* CTRLIF_ERR_STS */;

pub const WM5100_ISRC2_UNDERCLOCKED_STS: c_uint = 0x0200  /* ISRC2_UNDERCLOCKED_STS */;
pub const WM5100_ISRC2_UNDERCLOCKED_STS_MASK: c_uint = 0x0200  /* ISRC2_UNDERCLOCKED_STS */;

pub const WM5100_ISRC1_UNDERCLOCKED_STS: c_uint = 0x0100  /* ISRC1_UNDERCLOCKED_STS */;
pub const WM5100_ISRC1_UNDERCLOCKED_STS_MASK: c_uint = 0x0100  /* ISRC1_UNDERCLOCKED_STS */;

pub const WM5100_FX_UNDERCLOCKED_STS: c_uint = 0x0080  /* FX_UNDERCLOCKED_STS */;
pub const WM5100_FX_UNDERCLOCKED_STS_MASK: c_uint = 0x0080  /* FX_UNDERCLOCKED_STS */;

pub const WM5100_AIF3_UNDERCLOCKED_STS: c_uint = 0x0040  /* AIF3_UNDERCLOCKED_STS */;
pub const WM5100_AIF3_UNDERCLOCKED_STS_MASK: c_uint = 0x0040  /* AIF3_UNDERCLOCKED_STS */;

pub const WM5100_AIF2_UNDERCLOCKED_STS: c_uint = 0x0020  /* AIF2_UNDERCLOCKED_STS */;
pub const WM5100_AIF2_UNDERCLOCKED_STS_MASK: c_uint = 0x0020  /* AIF2_UNDERCLOCKED_STS */;

pub const WM5100_AIF1_UNDERCLOCKED_STS: c_uint = 0x0010  /* AIF1_UNDERCLOCKED_STS */;
pub const WM5100_AIF1_UNDERCLOCKED_STS_MASK: c_uint = 0x0010  /* AIF1_UNDERCLOCKED_STS */;

pub const WM5100_ASRC_UNDERCLOCKED_STS: c_uint = 0x0008  /* ASRC_UNDERCLOCKED_STS */;
pub const WM5100_ASRC_UNDERCLOCKED_STS_MASK: c_uint = 0x0008  /* ASRC_UNDERCLOCKED_STS */;

pub const WM5100_DAC_UNDERCLOCKED_STS: c_uint = 0x0004  /* DAC_UNDERCLOCKED_STS */;
pub const WM5100_DAC_UNDERCLOCKED_STS_MASK: c_uint = 0x0004  /* DAC_UNDERCLOCKED_STS */;

pub const WM5100_ADC_UNDERCLOCKED_STS: c_uint = 0x0002  /* ADC_UNDERCLOCKED_STS */;
pub const WM5100_ADC_UNDERCLOCKED_STS_MASK: c_uint = 0x0002  /* ADC_UNDERCLOCKED_STS */;

pub const WM5100_MIXER_UNDERCLOCKED_STS: c_uint = 0x0001  /* MIXER_UNDERCLOCKED_STS */;
pub const WM5100_MIXER_UNDERCLOCKED_STS_MASK: c_uint = 0x0001  /* MIXER_UNDERCLOCKED_STS */;

//
// R3335 (0xD07) - Interrupt Status 1 Mask
//
pub const WM5100_IM_GP6_EINT: c_uint = 0x0020  /* IM_GP6_EINT */;
pub const WM5100_IM_GP6_EINT_MASK: c_uint = 0x0020  /* IM_GP6_EINT */;

pub const WM5100_IM_GP5_EINT: c_uint = 0x0010  /* IM_GP5_EINT */;
pub const WM5100_IM_GP5_EINT_MASK: c_uint = 0x0010  /* IM_GP5_EINT */;

pub const WM5100_IM_GP4_EINT: c_uint = 0x0008  /* IM_GP4_EINT */;
pub const WM5100_IM_GP4_EINT_MASK: c_uint = 0x0008  /* IM_GP4_EINT */;

pub const WM5100_IM_GP3_EINT: c_uint = 0x0004  /* IM_GP3_EINT */;
pub const WM5100_IM_GP3_EINT_MASK: c_uint = 0x0004  /* IM_GP3_EINT */;

pub const WM5100_IM_GP2_EINT: c_uint = 0x0002  /* IM_GP2_EINT */;
pub const WM5100_IM_GP2_EINT_MASK: c_uint = 0x0002  /* IM_GP2_EINT */;

pub const WM5100_IM_GP1_EINT: c_uint = 0x0001  /* IM_GP1_EINT */;
pub const WM5100_IM_GP1_EINT_MASK: c_uint = 0x0001  /* IM_GP1_EINT */;

//
// R3336 (0xD08) - Interrupt Status 2 Mask
//
pub const WM5100_IM_DSP_IRQ6_EINT: c_uint = 0x0020  /* IM_DSP_IRQ6_EINT */;
pub const WM5100_IM_DSP_IRQ6_EINT_MASK: c_uint = 0x0020  /* IM_DSP_IRQ6_EINT */;

pub const WM5100_IM_DSP_IRQ5_EINT: c_uint = 0x0010  /* IM_DSP_IRQ5_EINT */;
pub const WM5100_IM_DSP_IRQ5_EINT_MASK: c_uint = 0x0010  /* IM_DSP_IRQ5_EINT */;

pub const WM5100_IM_DSP_IRQ4_EINT: c_uint = 0x0008  /* IM_DSP_IRQ4_EINT */;
pub const WM5100_IM_DSP_IRQ4_EINT_MASK: c_uint = 0x0008  /* IM_DSP_IRQ4_EINT */;

pub const WM5100_IM_DSP_IRQ3_EINT: c_uint = 0x0004  /* IM_DSP_IRQ3_EINT */;
pub const WM5100_IM_DSP_IRQ3_EINT_MASK: c_uint = 0x0004  /* IM_DSP_IRQ3_EINT */;

pub const WM5100_IM_DSP_IRQ2_EINT: c_uint = 0x0002  /* IM_DSP_IRQ2_EINT */;
pub const WM5100_IM_DSP_IRQ2_EINT_MASK: c_uint = 0x0002  /* IM_DSP_IRQ2_EINT */;

pub const WM5100_IM_DSP_IRQ1_EINT: c_uint = 0x0001  /* IM_DSP_IRQ1_EINT */;
pub const WM5100_IM_DSP_IRQ1_EINT_MASK: c_uint = 0x0001  /* IM_DSP_IRQ1_EINT */;

//
// R3337 (0xD09) - Interrupt Status 3 Mask
//
pub const WM5100_IM_SPK_SHUTDOWN_WARN_EINT: c_uint = 0x8000  /* IM_SPK_SHUTDOWN_WARN_EINT */;
pub const WM5100_IM_SPK_SHUTDOWN_WARN_EINT_MASK: c_uint = 0x8000  /* IM_SPK_SHUTDOWN_WARN_EINT */;

pub const WM5100_IM_SPK_SHUTDOWN_EINT: c_uint = 0x4000  /* IM_SPK_SHUTDOWN_EINT */;
pub const WM5100_IM_SPK_SHUTDOWN_EINT_MASK: c_uint = 0x4000  /* IM_SPK_SHUTDOWN_EINT */;

pub const WM5100_IM_HPDET_EINT: c_uint = 0x2000  /* IM_HPDET_EINT */;
pub const WM5100_IM_HPDET_EINT_MASK: c_uint = 0x2000  /* IM_HPDET_EINT */;

pub const WM5100_IM_ACCDET_EINT: c_uint = 0x1000  /* IM_ACCDET_EINT */;
pub const WM5100_IM_ACCDET_EINT_MASK: c_uint = 0x1000  /* IM_ACCDET_EINT */;

pub const WM5100_IM_DRC_SIG_DET_EINT: c_uint = 0x0200  /* IM_DRC_SIG_DET_EINT */;
pub const WM5100_IM_DRC_SIG_DET_EINT_MASK: c_uint = 0x0200  /* IM_DRC_SIG_DET_EINT */;

pub const WM5100_IM_ASRC2_LOCK_EINT: c_uint = 0x0100  /* IM_ASRC2_LOCK_EINT */;
pub const WM5100_IM_ASRC2_LOCK_EINT_MASK: c_uint = 0x0100  /* IM_ASRC2_LOCK_EINT */;

pub const WM5100_IM_ASRC1_LOCK_EINT: c_uint = 0x0080  /* IM_ASRC1_LOCK_EINT */;
pub const WM5100_IM_ASRC1_LOCK_EINT_MASK: c_uint = 0x0080  /* IM_ASRC1_LOCK_EINT */;

pub const WM5100_IM_FLL2_LOCK_EINT: c_uint = 0x0008  /* IM_FLL2_LOCK_EINT */;
pub const WM5100_IM_FLL2_LOCK_EINT_MASK: c_uint = 0x0008  /* IM_FLL2_LOCK_EINT */;

pub const WM5100_IM_FLL1_LOCK_EINT: c_uint = 0x0004  /* IM_FLL1_LOCK_EINT */;
pub const WM5100_IM_FLL1_LOCK_EINT_MASK: c_uint = 0x0004  /* IM_FLL1_LOCK_EINT */;

pub const WM5100_IM_CLKGEN_ERR_EINT: c_uint = 0x0002  /* IM_CLKGEN_ERR_EINT */;
pub const WM5100_IM_CLKGEN_ERR_EINT_MASK: c_uint = 0x0002  /* IM_CLKGEN_ERR_EINT */;

pub const WM5100_IM_CLKGEN_ERR_ASYNC_EINT: c_uint = 0x0001  /* IM_CLKGEN_ERR_ASYNC_EINT */;
pub const WM5100_IM_CLKGEN_ERR_ASYNC_EINT_MASK: c_uint = 0x0001  /* IM_CLKGEN_ERR_ASYNC_EINT */;

//
// R3338 (0xD0A) - Interrupt Status 4 Mask
//
pub const WM5100_IM_AIF3_ERR_EINT: c_uint = 0x2000  /* IM_AIF3_ERR_EINT */;
pub const WM5100_IM_AIF3_ERR_EINT_MASK: c_uint = 0x2000  /* IM_AIF3_ERR_EINT */;

pub const WM5100_IM_AIF2_ERR_EINT: c_uint = 0x1000  /* IM_AIF2_ERR_EINT */;
pub const WM5100_IM_AIF2_ERR_EINT_MASK: c_uint = 0x1000  /* IM_AIF2_ERR_EINT */;

pub const WM5100_IM_AIF1_ERR_EINT: c_uint = 0x0800  /* IM_AIF1_ERR_EINT */;
pub const WM5100_IM_AIF1_ERR_EINT_MASK: c_uint = 0x0800  /* IM_AIF1_ERR_EINT */;

pub const WM5100_IM_CTRLIF_ERR_EINT: c_uint = 0x0400  /* IM_CTRLIF_ERR_EINT */;
pub const WM5100_IM_CTRLIF_ERR_EINT_MASK: c_uint = 0x0400  /* IM_CTRLIF_ERR_EINT */;

pub const WM5100_IM_ISRC2_UNDERCLOCKED_EINT: c_uint = 0x0200  /* IM_ISRC2_UNDERCLOCKED_EINT */;
pub const WM5100_IM_ISRC2_UNDERCLOCKED_EINT_MASK: c_uint = 0x0200  /* IM_ISRC2_UNDERCLOCKED_EINT */;

pub const WM5100_IM_ISRC1_UNDERCLOCKED_EINT: c_uint = 0x0100  /* IM_ISRC1_UNDERCLOCKED_EINT */;
pub const WM5100_IM_ISRC1_UNDERCLOCKED_EINT_MASK: c_uint = 0x0100  /* IM_ISRC1_UNDERCLOCKED_EINT */;

pub const WM5100_IM_FX_UNDERCLOCKED_EINT: c_uint = 0x0080  /* IM_FX_UNDERCLOCKED_EINT */;
pub const WM5100_IM_FX_UNDERCLOCKED_EINT_MASK: c_uint = 0x0080  /* IM_FX_UNDERCLOCKED_EINT */;

pub const WM5100_IM_AIF3_UNDERCLOCKED_EINT: c_uint = 0x0040  /* IM_AIF3_UNDERCLOCKED_EINT */;
pub const WM5100_IM_AIF3_UNDERCLOCKED_EINT_MASK: c_uint = 0x0040  /* IM_AIF3_UNDERCLOCKED_EINT */;

pub const WM5100_IM_AIF2_UNDERCLOCKED_EINT: c_uint = 0x0020  /* IM_AIF2_UNDERCLOCKED_EINT */;
pub const WM5100_IM_AIF2_UNDERCLOCKED_EINT_MASK: c_uint = 0x0020  /* IM_AIF2_UNDERCLOCKED_EINT */;

pub const WM5100_IM_AIF1_UNDERCLOCKED_EINT: c_uint = 0x0010  /* IM_AIF1_UNDERCLOCKED_EINT */;
pub const WM5100_IM_AIF1_UNDERCLOCKED_EINT_MASK: c_uint = 0x0010  /* IM_AIF1_UNDERCLOCKED_EINT */;

pub const WM5100_IM_ASRC_UNDERCLOCKED_EINT: c_uint = 0x0008  /* IM_ASRC_UNDERCLOCKED_EINT */;
pub const WM5100_IM_ASRC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0008  /* IM_ASRC_UNDERCLOCKED_EINT */;

pub const WM5100_IM_DAC_UNDERCLOCKED_EINT: c_uint = 0x0004  /* IM_DAC_UNDERCLOCKED_EINT */;
pub const WM5100_IM_DAC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0004  /* IM_DAC_UNDERCLOCKED_EINT */;

pub const WM5100_IM_ADC_UNDERCLOCKED_EINT: c_uint = 0x0002  /* IM_ADC_UNDERCLOCKED_EINT */;
pub const WM5100_IM_ADC_UNDERCLOCKED_EINT_MASK: c_uint = 0x0002  /* IM_ADC_UNDERCLOCKED_EINT */;

pub const WM5100_IM_MIXER_UNDERCLOCKED_EINT: c_uint = 0x0001  /* IM_MIXER_UNDERCLOCKED_EINT */;
pub const WM5100_IM_MIXER_UNDERCLOCKED_EINT_MASK: c_uint = 0x0001  /* IM_MIXER_UNDERCLOCKED_EINT */;

//
// R3359 (0xD1F) - Interrupt Control
//
pub const WM5100_IM_IRQ: c_uint = 0x0001  /* IM_IRQ */;
pub const WM5100_IM_IRQ_MASK: c_uint = 0x0001  /* IM_IRQ */;

//
// R3360 (0xD20) - IRQ Debounce 1
//
pub const WM5100_SPK_SHUTDOWN_WARN_DB: c_uint = 0x0200  /* SPK_SHUTDOWN_WARN_DB */;
pub const WM5100_SPK_SHUTDOWN_WARN_DB_MASK: c_uint = 0x0200  /* SPK_SHUTDOWN_WARN_DB */;

pub const WM5100_SPK_SHUTDOWN_DB: c_uint = 0x0100  /* SPK_SHUTDOWN_DB */;
pub const WM5100_SPK_SHUTDOWN_DB_MASK: c_uint = 0x0100  /* SPK_SHUTDOWN_DB */;

pub const WM5100_FLL1_LOCK_IRQ_DB: c_uint = 0x0008  /* FLL1_LOCK_IRQ_DB */;
pub const WM5100_FLL1_LOCK_IRQ_DB_MASK: c_uint = 0x0008  /* FLL1_LOCK_IRQ_DB */;

pub const WM5100_FLL2_LOCK_IRQ_DB: c_uint = 0x0004  /* FLL2_LOCK_IRQ_DB */;
pub const WM5100_FLL2_LOCK_IRQ_DB_MASK: c_uint = 0x0004  /* FLL2_LOCK_IRQ_DB */;

pub const WM5100_CLKGEN_ERR_IRQ_DB: c_uint = 0x0002  /* CLKGEN_ERR_IRQ_DB */;
pub const WM5100_CLKGEN_ERR_IRQ_DB_MASK: c_uint = 0x0002  /* CLKGEN_ERR_IRQ_DB */;

pub const WM5100_CLKGEN_ERR_ASYNC_IRQ_DB: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_IRQ_DB */;
pub const WM5100_CLKGEN_ERR_ASYNC_IRQ_DB_MASK: c_uint = 0x0001  /* CLKGEN_ERR_ASYNC_IRQ_DB */;

//
// R3361 (0xD21) - IRQ Debounce 2
//
pub const WM5100_AIF_ERR_DB: c_uint = 0x0001  /* AIF_ERR_DB */;
pub const WM5100_AIF_ERR_DB_MASK: c_uint = 0x0001  /* AIF_ERR_DB */;

//
// R3584 (0xE00) - FX_Ctrl
//
pub const WM5100_FX_STS_MASK: c_uint = 0xFFC0  /* FX_STS - [15:6] */;

pub const WM5100_FX_RATE_MASK: c_uint = 0x0003  /* FX_RATE - [1:0] */;

//
// R3600 (0xE10) - EQ1_1
//
pub const WM5100_EQ1_B1_GAIN_MASK: c_uint = 0xF800  /* EQ1_B1_GAIN - [15:11] */;

pub const WM5100_EQ1_B2_GAIN_MASK: c_uint = 0x07C0  /* EQ1_B2_GAIN - [10:6] */;

pub const WM5100_EQ1_B3_GAIN_MASK: c_uint = 0x003E  /* EQ1_B3_GAIN - [5:1] */;

pub const WM5100_EQ1_ENA: c_uint = 0x0001  /* EQ1_ENA */;
pub const WM5100_EQ1_ENA_MASK: c_uint = 0x0001  /* EQ1_ENA */;

//
// R3601 (0xE11) - EQ1_2
//
pub const WM5100_EQ1_B4_GAIN_MASK: c_uint = 0xF800  /* EQ1_B4_GAIN - [15:11] */;

pub const WM5100_EQ1_B5_GAIN_MASK: c_uint = 0x07C0  /* EQ1_B5_GAIN - [10:6] */;

//
// R3602 (0xE12) - EQ1_3
//
pub const WM5100_EQ1_B1_A_MASK: c_uint = 0xFFFF  /* EQ1_B1_A - [15:0] */;

//
// R3603 (0xE13) - EQ1_4
//
pub const WM5100_EQ1_B1_B_MASK: c_uint = 0xFFFF  /* EQ1_B1_B - [15:0] */;

//
// R3604 (0xE14) - EQ1_5
//
pub const WM5100_EQ1_B1_PG_MASK: c_uint = 0xFFFF  /* EQ1_B1_PG - [15:0] */;

//
// R3605 (0xE15) - EQ1_6
//
pub const WM5100_EQ1_B2_A_MASK: c_uint = 0xFFFF  /* EQ1_B2_A - [15:0] */;

//
// R3606 (0xE16) - EQ1_7
//
pub const WM5100_EQ1_B2_B_MASK: c_uint = 0xFFFF  /* EQ1_B2_B - [15:0] */;

//
// R3607 (0xE17) - EQ1_8
//
pub const WM5100_EQ1_B2_C_MASK: c_uint = 0xFFFF  /* EQ1_B2_C - [15:0] */;

//
// R3608 (0xE18) - EQ1_9
//
pub const WM5100_EQ1_B2_PG_MASK: c_uint = 0xFFFF  /* EQ1_B2_PG - [15:0] */;

//
// R3609 (0xE19) - EQ1_10
//
pub const WM5100_EQ1_B3_A_MASK: c_uint = 0xFFFF  /* EQ1_B3_A - [15:0] */;

//
// R3610 (0xE1A) - EQ1_11
//
pub const WM5100_EQ1_B3_B_MASK: c_uint = 0xFFFF  /* EQ1_B3_B - [15:0] */;

//
// R3611 (0xE1B) - EQ1_12
//
pub const WM5100_EQ1_B3_C_MASK: c_uint = 0xFFFF  /* EQ1_B3_C - [15:0] */;

//
// R3612 (0xE1C) - EQ1_13
//
pub const WM5100_EQ1_B3_PG_MASK: c_uint = 0xFFFF  /* EQ1_B3_PG - [15:0] */;

//
// R3613 (0xE1D) - EQ1_14
//
pub const WM5100_EQ1_B4_A_MASK: c_uint = 0xFFFF  /* EQ1_B4_A - [15:0] */;

//
// R3614 (0xE1E) - EQ1_15
//
pub const WM5100_EQ1_B4_B_MASK: c_uint = 0xFFFF  /* EQ1_B4_B - [15:0] */;

//
// R3615 (0xE1F) - EQ1_16
//
pub const WM5100_EQ1_B4_C_MASK: c_uint = 0xFFFF  /* EQ1_B4_C - [15:0] */;

//
// R3616 (0xE20) - EQ1_17
//
pub const WM5100_EQ1_B4_PG_MASK: c_uint = 0xFFFF  /* EQ1_B4_PG - [15:0] */;

//
// R3617 (0xE21) - EQ1_18
//
pub const WM5100_EQ1_B5_A_MASK: c_uint = 0xFFFF  /* EQ1_B5_A - [15:0] */;

//
// R3618 (0xE22) - EQ1_19
//
pub const WM5100_EQ1_B5_B_MASK: c_uint = 0xFFFF  /* EQ1_B5_B - [15:0] */;

//
// R3619 (0xE23) - EQ1_20
//
pub const WM5100_EQ1_B5_PG_MASK: c_uint = 0xFFFF  /* EQ1_B5_PG - [15:0] */;

//
// R3622 (0xE26) - EQ2_1
//
pub const WM5100_EQ2_B1_GAIN_MASK: c_uint = 0xF800  /* EQ2_B1_GAIN - [15:11] */;

pub const WM5100_EQ2_B2_GAIN_MASK: c_uint = 0x07C0  /* EQ2_B2_GAIN - [10:6] */;

pub const WM5100_EQ2_B3_GAIN_MASK: c_uint = 0x003E  /* EQ2_B3_GAIN - [5:1] */;

pub const WM5100_EQ2_ENA: c_uint = 0x0001  /* EQ2_ENA */;
pub const WM5100_EQ2_ENA_MASK: c_uint = 0x0001  /* EQ2_ENA */;

//
// R3623 (0xE27) - EQ2_2
//
pub const WM5100_EQ2_B4_GAIN_MASK: c_uint = 0xF800  /* EQ2_B4_GAIN - [15:11] */;

pub const WM5100_EQ2_B5_GAIN_MASK: c_uint = 0x07C0  /* EQ2_B5_GAIN - [10:6] */;

//
// R3624 (0xE28) - EQ2_3
//
pub const WM5100_EQ2_B1_A_MASK: c_uint = 0xFFFF  /* EQ2_B1_A - [15:0] */;

//
// R3625 (0xE29) - EQ2_4
//
pub const WM5100_EQ2_B1_B_MASK: c_uint = 0xFFFF  /* EQ2_B1_B - [15:0] */;

//
// R3626 (0xE2A) - EQ2_5
//
pub const WM5100_EQ2_B1_PG_MASK: c_uint = 0xFFFF  /* EQ2_B1_PG - [15:0] */;

//
// R3627 (0xE2B) - EQ2_6
//
pub const WM5100_EQ2_B2_A_MASK: c_uint = 0xFFFF  /* EQ2_B2_A - [15:0] */;

//
// R3628 (0xE2C) - EQ2_7
//
pub const WM5100_EQ2_B2_B_MASK: c_uint = 0xFFFF  /* EQ2_B2_B - [15:0] */;

//
// R3629 (0xE2D) - EQ2_8
//
pub const WM5100_EQ2_B2_C_MASK: c_uint = 0xFFFF  /* EQ2_B2_C - [15:0] */;

//
// R3630 (0xE2E) - EQ2_9
//
pub const WM5100_EQ2_B2_PG_MASK: c_uint = 0xFFFF  /* EQ2_B2_PG - [15:0] */;

//
// R3631 (0xE2F) - EQ2_10
//
pub const WM5100_EQ2_B3_A_MASK: c_uint = 0xFFFF  /* EQ2_B3_A - [15:0] */;

//
// R3632 (0xE30) - EQ2_11
//
pub const WM5100_EQ2_B3_B_MASK: c_uint = 0xFFFF  /* EQ2_B3_B - [15:0] */;

//
// R3633 (0xE31) - EQ2_12
//
pub const WM5100_EQ2_B3_C_MASK: c_uint = 0xFFFF  /* EQ2_B3_C - [15:0] */;

//
// R3634 (0xE32) - EQ2_13
//
pub const WM5100_EQ2_B3_PG_MASK: c_uint = 0xFFFF  /* EQ2_B3_PG - [15:0] */;

//
// R3635 (0xE33) - EQ2_14
//
pub const WM5100_EQ2_B4_A_MASK: c_uint = 0xFFFF  /* EQ2_B4_A - [15:0] */;

//
// R3636 (0xE34) - EQ2_15
//
pub const WM5100_EQ2_B4_B_MASK: c_uint = 0xFFFF  /* EQ2_B4_B - [15:0] */;

//
// R3637 (0xE35) - EQ2_16
//
pub const WM5100_EQ2_B4_C_MASK: c_uint = 0xFFFF  /* EQ2_B4_C - [15:0] */;

//
// R3638 (0xE36) - EQ2_17
//
pub const WM5100_EQ2_B4_PG_MASK: c_uint = 0xFFFF  /* EQ2_B4_PG - [15:0] */;

//
// R3639 (0xE37) - EQ2_18
//
pub const WM5100_EQ2_B5_A_MASK: c_uint = 0xFFFF  /* EQ2_B5_A - [15:0] */;

//
// R3640 (0xE38) - EQ2_19
//
pub const WM5100_EQ2_B5_B_MASK: c_uint = 0xFFFF  /* EQ2_B5_B - [15:0] */;

//
// R3641 (0xE39) - EQ2_20
//
pub const WM5100_EQ2_B5_PG_MASK: c_uint = 0xFFFF  /* EQ2_B5_PG - [15:0] */;

//
// R3644 (0xE3C) - EQ3_1
//
pub const WM5100_EQ3_B1_GAIN_MASK: c_uint = 0xF800  /* EQ3_B1_GAIN - [15:11] */;

pub const WM5100_EQ3_B2_GAIN_MASK: c_uint = 0x07C0  /* EQ3_B2_GAIN - [10:6] */;

pub const WM5100_EQ3_B3_GAIN_MASK: c_uint = 0x003E  /* EQ3_B3_GAIN - [5:1] */;

pub const WM5100_EQ3_ENA: c_uint = 0x0001  /* EQ3_ENA */;
pub const WM5100_EQ3_ENA_MASK: c_uint = 0x0001  /* EQ3_ENA */;

//
// R3645 (0xE3D) - EQ3_2
//
pub const WM5100_EQ3_B4_GAIN_MASK: c_uint = 0xF800  /* EQ3_B4_GAIN - [15:11] */;

pub const WM5100_EQ3_B5_GAIN_MASK: c_uint = 0x07C0  /* EQ3_B5_GAIN - [10:6] */;

//
// R3646 (0xE3E) - EQ3_3
//
pub const WM5100_EQ3_B1_A_MASK: c_uint = 0xFFFF  /* EQ3_B1_A - [15:0] */;

//
// R3647 (0xE3F) - EQ3_4
//
pub const WM5100_EQ3_B1_B_MASK: c_uint = 0xFFFF  /* EQ3_B1_B - [15:0] */;

//
// R3648 (0xE40) - EQ3_5
//
pub const WM5100_EQ3_B1_PG_MASK: c_uint = 0xFFFF  /* EQ3_B1_PG - [15:0] */;

//
// R3649 (0xE41) - EQ3_6
//
pub const WM5100_EQ3_B2_A_MASK: c_uint = 0xFFFF  /* EQ3_B2_A - [15:0] */;

//
// R3650 (0xE42) - EQ3_7
//
pub const WM5100_EQ3_B2_B_MASK: c_uint = 0xFFFF  /* EQ3_B2_B - [15:0] */;

//
// R3651 (0xE43) - EQ3_8
//
pub const WM5100_EQ3_B2_C_MASK: c_uint = 0xFFFF  /* EQ3_B2_C - [15:0] */;

//
// R3652 (0xE44) - EQ3_9
//
pub const WM5100_EQ3_B2_PG_MASK: c_uint = 0xFFFF  /* EQ3_B2_PG - [15:0] */;

//
// R3653 (0xE45) - EQ3_10
//
pub const WM5100_EQ3_B3_A_MASK: c_uint = 0xFFFF  /* EQ3_B3_A - [15:0] */;

//
// R3654 (0xE46) - EQ3_11
//
pub const WM5100_EQ3_B3_B_MASK: c_uint = 0xFFFF  /* EQ3_B3_B - [15:0] */;

//
// R3655 (0xE47) - EQ3_12
//
pub const WM5100_EQ3_B3_C_MASK: c_uint = 0xFFFF  /* EQ3_B3_C - [15:0] */;

//
// R3656 (0xE48) - EQ3_13
//
pub const WM5100_EQ3_B3_PG_MASK: c_uint = 0xFFFF  /* EQ3_B3_PG - [15:0] */;

//
// R3657 (0xE49) - EQ3_14
//
pub const WM5100_EQ3_B4_A_MASK: c_uint = 0xFFFF  /* EQ3_B4_A - [15:0] */;

//
// R3658 (0xE4A) - EQ3_15
//
pub const WM5100_EQ3_B4_B_MASK: c_uint = 0xFFFF  /* EQ3_B4_B - [15:0] */;

//
// R3659 (0xE4B) - EQ3_16
//
pub const WM5100_EQ3_B4_C_MASK: c_uint = 0xFFFF  /* EQ3_B4_C - [15:0] */;

//
// R3660 (0xE4C) - EQ3_17
//
pub const WM5100_EQ3_B4_PG_MASK: c_uint = 0xFFFF  /* EQ3_B4_PG - [15:0] */;

//
// R3661 (0xE4D) - EQ3_18
//
pub const WM5100_EQ3_B5_A_MASK: c_uint = 0xFFFF  /* EQ3_B5_A - [15:0] */;

//
// R3662 (0xE4E) - EQ3_19
//
pub const WM5100_EQ3_B5_B_MASK: c_uint = 0xFFFF  /* EQ3_B5_B - [15:0] */;

//
// R3663 (0xE4F) - EQ3_20
//
pub const WM5100_EQ3_B5_PG_MASK: c_uint = 0xFFFF  /* EQ3_B5_PG - [15:0] */;

//
// R3666 (0xE52) - EQ4_1
//
pub const WM5100_EQ4_B1_GAIN_MASK: c_uint = 0xF800  /* EQ4_B1_GAIN - [15:11] */;

pub const WM5100_EQ4_B2_GAIN_MASK: c_uint = 0x07C0  /* EQ4_B2_GAIN - [10:6] */;

pub const WM5100_EQ4_B3_GAIN_MASK: c_uint = 0x003E  /* EQ4_B3_GAIN - [5:1] */;

pub const WM5100_EQ4_ENA: c_uint = 0x0001  /* EQ4_ENA */;
pub const WM5100_EQ4_ENA_MASK: c_uint = 0x0001  /* EQ4_ENA */;

//
// R3667 (0xE53) - EQ4_2
//
pub const WM5100_EQ4_B4_GAIN_MASK: c_uint = 0xF800  /* EQ4_B4_GAIN - [15:11] */;

pub const WM5100_EQ4_B5_GAIN_MASK: c_uint = 0x07C0  /* EQ4_B5_GAIN - [10:6] */;

//
// R3668 (0xE54) - EQ4_3
//
pub const WM5100_EQ4_B1_A_MASK: c_uint = 0xFFFF  /* EQ4_B1_A - [15:0] */;

//
// R3669 (0xE55) - EQ4_4
//
pub const WM5100_EQ4_B1_B_MASK: c_uint = 0xFFFF  /* EQ4_B1_B - [15:0] */;

//
// R3670 (0xE56) - EQ4_5
//
pub const WM5100_EQ4_B1_PG_MASK: c_uint = 0xFFFF  /* EQ4_B1_PG - [15:0] */;

//
// R3671 (0xE57) - EQ4_6
//
pub const WM5100_EQ4_B2_A_MASK: c_uint = 0xFFFF  /* EQ4_B2_A - [15:0] */;

//
// R3672 (0xE58) - EQ4_7
//
pub const WM5100_EQ4_B2_B_MASK: c_uint = 0xFFFF  /* EQ4_B2_B - [15:0] */;

//
// R3673 (0xE59) - EQ4_8
//
pub const WM5100_EQ4_B2_C_MASK: c_uint = 0xFFFF  /* EQ4_B2_C - [15:0] */;

//
// R3674 (0xE5A) - EQ4_9
//
pub const WM5100_EQ4_B2_PG_MASK: c_uint = 0xFFFF  /* EQ4_B2_PG - [15:0] */;

//
// R3675 (0xE5B) - EQ4_10
//
pub const WM5100_EQ4_B3_A_MASK: c_uint = 0xFFFF  /* EQ4_B3_A - [15:0] */;

//
// R3676 (0xE5C) - EQ4_11
//
pub const WM5100_EQ4_B3_B_MASK: c_uint = 0xFFFF  /* EQ4_B3_B - [15:0] */;

//
// R3677 (0xE5D) - EQ4_12
//
pub const WM5100_EQ4_B3_C_MASK: c_uint = 0xFFFF  /* EQ4_B3_C - [15:0] */;

//
// R3678 (0xE5E) - EQ4_13
//
pub const WM5100_EQ4_B3_PG_MASK: c_uint = 0xFFFF  /* EQ4_B3_PG - [15:0] */;

//
// R3679 (0xE5F) - EQ4_14
//
pub const WM5100_EQ4_B4_A_MASK: c_uint = 0xFFFF  /* EQ4_B4_A - [15:0] */;

//
// R3680 (0xE60) - EQ4_15
//
pub const WM5100_EQ4_B4_B_MASK: c_uint = 0xFFFF  /* EQ4_B4_B - [15:0] */;

//
// R3681 (0xE61) - EQ4_16
//
pub const WM5100_EQ4_B4_C_MASK: c_uint = 0xFFFF  /* EQ4_B4_C - [15:0] */;

//
// R3682 (0xE62) - EQ4_17
//
pub const WM5100_EQ4_B4_PG_MASK: c_uint = 0xFFFF  /* EQ4_B4_PG - [15:0] */;

//
// R3683 (0xE63) - EQ4_18
//
pub const WM5100_EQ4_B5_A_MASK: c_uint = 0xFFFF  /* EQ4_B5_A - [15:0] */;

//
// R3684 (0xE64) - EQ4_19
//
pub const WM5100_EQ4_B5_B_MASK: c_uint = 0xFFFF  /* EQ4_B5_B - [15:0] */;

//
// R3685 (0xE65) - EQ4_20
//
pub const WM5100_EQ4_B5_PG_MASK: c_uint = 0xFFFF  /* EQ4_B5_PG - [15:0] */;

//
// R3712 (0xE80) - DRC1 ctrl1
//
pub const WM5100_DRC_SIG_DET_RMS_MASK: c_uint = 0xF800  /* DRC_SIG_DET_RMS - [15:11] */;

pub const WM5100_DRC_SIG_DET_PK_MASK: c_uint = 0x0600  /* DRC_SIG_DET_PK - [10:9] */;

pub const WM5100_DRC_NG_ENA: c_uint = 0x0100  /* DRC_NG_ENA */;
pub const WM5100_DRC_NG_ENA_MASK: c_uint = 0x0100  /* DRC_NG_ENA */;

pub const WM5100_DRC_SIG_DET_MODE: c_uint = 0x0080  /* DRC_SIG_DET_MODE */;
pub const WM5100_DRC_SIG_DET_MODE_MASK: c_uint = 0x0080  /* DRC_SIG_DET_MODE */;

pub const WM5100_DRC_SIG_DET: c_uint = 0x0040  /* DRC_SIG_DET */;
pub const WM5100_DRC_SIG_DET_MASK: c_uint = 0x0040  /* DRC_SIG_DET */;

pub const WM5100_DRC_KNEE2_OP_ENA: c_uint = 0x0020  /* DRC_KNEE2_OP_ENA */;
pub const WM5100_DRC_KNEE2_OP_ENA_MASK: c_uint = 0x0020  /* DRC_KNEE2_OP_ENA */;

pub const WM5100_DRC_QR: c_uint = 0x0010  /* DRC_QR */;
pub const WM5100_DRC_QR_MASK: c_uint = 0x0010  /* DRC_QR */;

pub const WM5100_DRC_ANTICLIP: c_uint = 0x0008  /* DRC_ANTICLIP */;
pub const WM5100_DRC_ANTICLIP_MASK: c_uint = 0x0008  /* DRC_ANTICLIP */;

pub const WM5100_DRCL_ENA: c_uint = 0x0002  /* DRCL_ENA */;
pub const WM5100_DRCL_ENA_MASK: c_uint = 0x0002  /* DRCL_ENA */;

pub const WM5100_DRCR_ENA: c_uint = 0x0001  /* DRCR_ENA */;
pub const WM5100_DRCR_ENA_MASK: c_uint = 0x0001  /* DRCR_ENA */;

//
// R3713 (0xE81) - DRC1 ctrl2
//
pub const WM5100_DRC_ATK_MASK: c_uint = 0x1E00  /* DRC_ATK - [12:9] */;

pub const WM5100_DRC_DCY_MASK: c_uint = 0x01E0  /* DRC_DCY - [8:5] */;

pub const WM5100_DRC_MINGAIN_MASK: c_uint = 0x001C  /* DRC_MINGAIN - [4:2] */;

pub const WM5100_DRC_MAXGAIN_MASK: c_uint = 0x0003  /* DRC_MAXGAIN - [1:0] */;

//
// R3714 (0xE82) - DRC1 ctrl3
//
pub const WM5100_DRC_NG_MINGAIN_MASK: c_uint = 0xF000  /* DRC_NG_MINGAIN - [15:12] */;

pub const WM5100_DRC_NG_EXP_MASK: c_uint = 0x0C00  /* DRC_NG_EXP - [11:10] */;

pub const WM5100_DRC_QR_THR_MASK: c_uint = 0x0300  /* DRC_QR_THR - [9:8] */;

pub const WM5100_DRC_QR_DCY_MASK: c_uint = 0x00C0  /* DRC_QR_DCY - [7:6] */;

pub const WM5100_DRC_HI_COMP_MASK: c_uint = 0x0038  /* DRC_HI_COMP - [5:3] */;

pub const WM5100_DRC_LO_COMP_MASK: c_uint = 0x0007  /* DRC_LO_COMP - [2:0] */;

//
// R3715 (0xE83) - DRC1 ctrl4
//
pub const WM5100_DRC_KNEE_IP_MASK: c_uint = 0x07E0  /* DRC_KNEE_IP - [10:5] */;

pub const WM5100_DRC_KNEE_OP_MASK: c_uint = 0x001F  /* DRC_KNEE_OP - [4:0] */;

//
// R3716 (0xE84) - DRC1 ctrl5
//
pub const WM5100_DRC_KNEE2_IP_MASK: c_uint = 0x03E0  /* DRC_KNEE2_IP - [9:5] */;

pub const WM5100_DRC_KNEE2_OP_MASK: c_uint = 0x001F  /* DRC_KNEE2_OP - [4:0] */;

//
// R3776 (0xEC0) - HPLPF1_1
//
pub const WM5100_LHPF1_MODE: c_uint = 0x0002  /* LHPF1_MODE */;
pub const WM5100_LHPF1_MODE_MASK: c_uint = 0x0002  /* LHPF1_MODE */;

pub const WM5100_LHPF1_ENA: c_uint = 0x0001  /* LHPF1_ENA */;
pub const WM5100_LHPF1_ENA_MASK: c_uint = 0x0001  /* LHPF1_ENA */;

//
// R3777 (0xEC1) - HPLPF1_2
//
pub const WM5100_LHPF1_COEFF_MASK: c_uint = 0xFFFF  /* LHPF1_COEFF - [15:0] */;

//
// R3780 (0xEC4) - HPLPF2_1
//
pub const WM5100_LHPF2_MODE: c_uint = 0x0002  /* LHPF2_MODE */;
pub const WM5100_LHPF2_MODE_MASK: c_uint = 0x0002  /* LHPF2_MODE */;

pub const WM5100_LHPF2_ENA: c_uint = 0x0001  /* LHPF2_ENA */;
pub const WM5100_LHPF2_ENA_MASK: c_uint = 0x0001  /* LHPF2_ENA */;

//
// R3781 (0xEC5) - HPLPF2_2
//
pub const WM5100_LHPF2_COEFF_MASK: c_uint = 0xFFFF  /* LHPF2_COEFF - [15:0] */;

//
// R3784 (0xEC8) - HPLPF3_1
//
pub const WM5100_LHPF3_MODE: c_uint = 0x0002  /* LHPF3_MODE */;
pub const WM5100_LHPF3_MODE_MASK: c_uint = 0x0002  /* LHPF3_MODE */;

pub const WM5100_LHPF3_ENA: c_uint = 0x0001  /* LHPF3_ENA */;
pub const WM5100_LHPF3_ENA_MASK: c_uint = 0x0001  /* LHPF3_ENA */;

//
// R3785 (0xEC9) - HPLPF3_2
//
pub const WM5100_LHPF3_COEFF_MASK: c_uint = 0xFFFF  /* LHPF3_COEFF - [15:0] */;

//
// R3788 (0xECC) - HPLPF4_1
//
pub const WM5100_LHPF4_MODE: c_uint = 0x0002  /* LHPF4_MODE */;
pub const WM5100_LHPF4_MODE_MASK: c_uint = 0x0002  /* LHPF4_MODE */;

pub const WM5100_LHPF4_ENA: c_uint = 0x0001  /* LHPF4_ENA */;
pub const WM5100_LHPF4_ENA_MASK: c_uint = 0x0001  /* LHPF4_ENA */;

//
// R3789 (0xECD) - HPLPF4_2
//
pub const WM5100_LHPF4_COEFF_MASK: c_uint = 0xFFFF  /* LHPF4_COEFF - [15:0] */;

//
// R4132 (0x1024) - DSP2 Control 30
//
pub const WM5100_DSP2_RATE_MASK: c_uint = 0xC000  /* DSP2_RATE - [15:14] */;

pub const WM5100_DSP2_DBG_CLK_ENA: c_uint = 0x0008  /* DSP2_DBG_CLK_ENA */;
pub const WM5100_DSP2_DBG_CLK_ENA_MASK: c_uint = 0x0008  /* DSP2_DBG_CLK_ENA */;

pub const WM5100_DSP2_SYS_ENA: c_uint = 0x0004  /* DSP2_SYS_ENA */;
pub const WM5100_DSP2_SYS_ENA_MASK: c_uint = 0x0004  /* DSP2_SYS_ENA */;

pub const WM5100_DSP2_CORE_ENA: c_uint = 0x0002  /* DSP2_CORE_ENA */;
pub const WM5100_DSP2_CORE_ENA_MASK: c_uint = 0x0002  /* DSP2_CORE_ENA */;

pub const WM5100_DSP2_START: c_uint = 0x0001  /* DSP2_START */;
pub const WM5100_DSP2_START_MASK: c_uint = 0x0001  /* DSP2_START */;

//
// R3876 (0xF24) - DSP1 Control 30
//
pub const WM5100_DSP1_RATE_MASK: c_uint = 0xC000  /* DSP1_RATE - [15:14] */;

pub const WM5100_DSP1_DBG_CLK_ENA: c_uint = 0x0008  /* DSP1_DBG_CLK_ENA */;
pub const WM5100_DSP1_DBG_CLK_ENA_MASK: c_uint = 0x0008  /* DSP1_DBG_CLK_ENA */;

pub const WM5100_DSP1_SYS_ENA: c_uint = 0x0004  /* DSP1_SYS_ENA */;
pub const WM5100_DSP1_SYS_ENA_MASK: c_uint = 0x0004  /* DSP1_SYS_ENA */;

pub const WM5100_DSP1_CORE_ENA: c_uint = 0x0002  /* DSP1_CORE_ENA */;
pub const WM5100_DSP1_CORE_ENA_MASK: c_uint = 0x0002  /* DSP1_CORE_ENA */;

pub const WM5100_DSP1_START: c_uint = 0x0001  /* DSP1_START */;
pub const WM5100_DSP1_START_MASK: c_uint = 0x0001  /* DSP1_START */;

//
// R4388 (0x1124) - DSP3 Control 30
//
pub const WM5100_DSP3_RATE_MASK: c_uint = 0xC000  /* DSP3_RATE - [15:14] */;

pub const WM5100_DSP3_DBG_CLK_ENA: c_uint = 0x0008  /* DSP3_DBG_CLK_ENA */;
pub const WM5100_DSP3_DBG_CLK_ENA_MASK: c_uint = 0x0008  /* DSP3_DBG_CLK_ENA */;

pub const WM5100_DSP3_SYS_ENA: c_uint = 0x0004  /* DSP3_SYS_ENA */;
pub const WM5100_DSP3_SYS_ENA_MASK: c_uint = 0x0004  /* DSP3_SYS_ENA */;

pub const WM5100_DSP3_CORE_ENA: c_uint = 0x0002  /* DSP3_CORE_ENA */;
pub const WM5100_DSP3_CORE_ENA_MASK: c_uint = 0x0002  /* DSP3_CORE_ENA */;

pub const WM5100_DSP3_START: c_uint = 0x0001  /* DSP3_START */;
pub const WM5100_DSP3_START_MASK: c_uint = 0x0001  /* DSP3_START */;

//
// R16384 (0x4000) - DSP1 DM 0
//
pub const WM5100_DSP1_DM_START_1_MASK: c_uint = 0x00FF  /* DSP1_DM_START - [7:0] */;

//
// R16385 (0x4001) - DSP1 DM 1
//
pub const WM5100_DSP1_DM_START_MASK: c_uint = 0xFFFF  /* DSP1_DM_START - [15:0] */;

//
// R16386 (0x4002) - DSP1 DM 2
//
pub const WM5100_DSP1_DM_1_1_MASK: c_uint = 0x00FF  /* DSP1_DM_1 - [7:0] */;

//
// R16387 (0x4003) - DSP1 DM 3
//
pub const WM5100_DSP1_DM_1_MASK: c_uint = 0xFFFF  /* DSP1_DM_1 - [15:0] */;

//
// R16892 (0x41FC) - DSP1 DM 508
//
pub const WM5100_DSP1_DM_254_1_MASK: c_uint = 0x00FF  /* DSP1_DM_254 - [7:0] */;

//
// R16893 (0x41FD) - DSP1 DM 509
//
pub const WM5100_DSP1_DM_254_MASK: c_uint = 0xFFFF  /* DSP1_DM_254 - [15:0] */;

//
// R16894 (0x41FE) - DSP1 DM 510
//
pub const WM5100_DSP1_DM_END_1_MASK: c_uint = 0x00FF  /* DSP1_DM_END - [7:0] */;

//
// R16895 (0x41FF) - DSP1 DM 511
//
pub const WM5100_DSP1_DM_END_MASK: c_uint = 0xFFFF  /* DSP1_DM_END - [15:0] */;

//
// R18432 (0x4800) - DSP1 PM 0
//
pub const WM5100_DSP1_PM_START_2_MASK: c_uint = 0x00FF  /* DSP1_PM_START - [7:0] */;

//
// R18433 (0x4801) - DSP1 PM 1
//
pub const WM5100_DSP1_PM_START_1_MASK: c_uint = 0xFFFF  /* DSP1_PM_START - [15:0] */;

//
// R18434 (0x4802) - DSP1 PM 2
//
pub const WM5100_DSP1_PM_START_MASK: c_uint = 0xFFFF  /* DSP1_PM_START - [15:0] */;

//
// R18435 (0x4803) - DSP1 PM 3
//
pub const WM5100_DSP1_PM_1_2_MASK: c_uint = 0x00FF  /* DSP1_PM_1 - [7:0] */;

//
// R18436 (0x4804) - DSP1 PM 4
//
pub const WM5100_DSP1_PM_1_1_MASK: c_uint = 0xFFFF  /* DSP1_PM_1 - [15:0] */;

//
// R18437 (0x4805) - DSP1 PM 5
//
pub const WM5100_DSP1_PM_1_MASK: c_uint = 0xFFFF  /* DSP1_PM_1 - [15:0] */;

//
// R19962 (0x4DFA) - DSP1 PM 1530
//
pub const WM5100_DSP1_PM_510_2_MASK: c_uint = 0x00FF  /* DSP1_PM_510 - [7:0] */;

//
// R19963 (0x4DFB) - DSP1 PM 1531
//
pub const WM5100_DSP1_PM_510_1_MASK: c_uint = 0xFFFF  /* DSP1_PM_510 - [15:0] */;

//
// R19964 (0x4DFC) - DSP1 PM 1532
//
pub const WM5100_DSP1_PM_510_MASK: c_uint = 0xFFFF  /* DSP1_PM_510 - [15:0] */;

//
// R19965 (0x4DFD) - DSP1 PM 1533
//
pub const WM5100_DSP1_PM_END_2_MASK: c_uint = 0x00FF  /* DSP1_PM_END - [7:0] */;

//
// R19966 (0x4DFE) - DSP1 PM 1534
//
pub const WM5100_DSP1_PM_END_1_MASK: c_uint = 0xFFFF  /* DSP1_PM_END - [15:0] */;

//
// R19967 (0x4DFF) - DSP1 PM 1535
//
pub const WM5100_DSP1_PM_END_MASK: c_uint = 0xFFFF  /* DSP1_PM_END - [15:0] */;

//
// R20480 (0x5000) - DSP1 ZM 0
//
pub const WM5100_DSP1_ZM_START_1_MASK: c_uint = 0x00FF  /* DSP1_ZM_START - [7:0] */;

//
// R20481 (0x5001) - DSP1 ZM 1
//
pub const WM5100_DSP1_ZM_START_MASK: c_uint = 0xFFFF  /* DSP1_ZM_START - [15:0] */;

//
// R20482 (0x5002) - DSP1 ZM 2
//
pub const WM5100_DSP1_ZM_1_1_MASK: c_uint = 0x00FF  /* DSP1_ZM_1 - [7:0] */;

//
// R20483 (0x5003) - DSP1 ZM 3
//
pub const WM5100_DSP1_ZM_1_MASK: c_uint = 0xFFFF  /* DSP1_ZM_1 - [15:0] */;

//
// R22524 (0x57FC) - DSP1 ZM 2044
//
pub const WM5100_DSP1_ZM_1022_1_MASK: c_uint = 0x00FF  /* DSP1_ZM_1022 - [7:0] */;

//
// R22525 (0x57FD) - DSP1 ZM 2045
//
pub const WM5100_DSP1_ZM_1022_MASK: c_uint = 0xFFFF  /* DSP1_ZM_1022 - [15:0] */;

//
// R22526 (0x57FE) - DSP1 ZM 2046
//
pub const WM5100_DSP1_ZM_END_1_MASK: c_uint = 0x00FF  /* DSP1_ZM_END - [7:0] */;

//
// R22527 (0x57FF) - DSP1 ZM 2047
//
pub const WM5100_DSP1_ZM_END_MASK: c_uint = 0xFFFF  /* DSP1_ZM_END - [15:0] */;

//
// R24576 (0x6000) - DSP2 DM 0
//
pub const WM5100_DSP2_DM_START_1_MASK: c_uint = 0x00FF  /* DSP2_DM_START - [7:0] */;

//
// R24577 (0x6001) - DSP2 DM 1
//
pub const WM5100_DSP2_DM_START_MASK: c_uint = 0xFFFF  /* DSP2_DM_START - [15:0] */;

//
// R24578 (0x6002) - DSP2 DM 2
//
pub const WM5100_DSP2_DM_1_1_MASK: c_uint = 0x00FF  /* DSP2_DM_1 - [7:0] */;

//
// R24579 (0x6003) - DSP2 DM 3
//
pub const WM5100_DSP2_DM_1_MASK: c_uint = 0xFFFF  /* DSP2_DM_1 - [15:0] */;

//
// R25084 (0x61FC) - DSP2 DM 508
//
pub const WM5100_DSP2_DM_254_1_MASK: c_uint = 0x00FF  /* DSP2_DM_254 - [7:0] */;

//
// R25085 (0x61FD) - DSP2 DM 509
//
pub const WM5100_DSP2_DM_254_MASK: c_uint = 0xFFFF  /* DSP2_DM_254 - [15:0] */;

//
// R25086 (0x61FE) - DSP2 DM 510
//
pub const WM5100_DSP2_DM_END_1_MASK: c_uint = 0x00FF  /* DSP2_DM_END - [7:0] */;

//
// R25087 (0x61FF) - DSP2 DM 511
//
pub const WM5100_DSP2_DM_END_MASK: c_uint = 0xFFFF  /* DSP2_DM_END - [15:0] */;

//
// R26624 (0x6800) - DSP2 PM 0
//
pub const WM5100_DSP2_PM_START_2_MASK: c_uint = 0x00FF  /* DSP2_PM_START - [7:0] */;

//
// R26625 (0x6801) - DSP2 PM 1
//
pub const WM5100_DSP2_PM_START_1_MASK: c_uint = 0xFFFF  /* DSP2_PM_START - [15:0] */;

//
// R26626 (0x6802) - DSP2 PM 2
//
pub const WM5100_DSP2_PM_START_MASK: c_uint = 0xFFFF  /* DSP2_PM_START - [15:0] */;

//
// R26627 (0x6803) - DSP2 PM 3
//
pub const WM5100_DSP2_PM_1_2_MASK: c_uint = 0x00FF  /* DSP2_PM_1 - [7:0] */;

//
// R26628 (0x6804) - DSP2 PM 4
//
pub const WM5100_DSP2_PM_1_1_MASK: c_uint = 0xFFFF  /* DSP2_PM_1 - [15:0] */;

//
// R26629 (0x6805) - DSP2 PM 5
//
pub const WM5100_DSP2_PM_1_MASK: c_uint = 0xFFFF  /* DSP2_PM_1 - [15:0] */;

//
// R28154 (0x6DFA) - DSP2 PM 1530
//
pub const WM5100_DSP2_PM_510_2_MASK: c_uint = 0x00FF  /* DSP2_PM_510 - [7:0] */;

//
// R28155 (0x6DFB) - DSP2 PM 1531
//
pub const WM5100_DSP2_PM_510_1_MASK: c_uint = 0xFFFF  /* DSP2_PM_510 - [15:0] */;

//
// R28156 (0x6DFC) - DSP2 PM 1532
//
pub const WM5100_DSP2_PM_510_MASK: c_uint = 0xFFFF  /* DSP2_PM_510 - [15:0] */;

//
// R28157 (0x6DFD) - DSP2 PM 1533
//
pub const WM5100_DSP2_PM_END_2_MASK: c_uint = 0x00FF  /* DSP2_PM_END - [7:0] */;

//
// R28158 (0x6DFE) - DSP2 PM 1534
//
pub const WM5100_DSP2_PM_END_1_MASK: c_uint = 0xFFFF  /* DSP2_PM_END - [15:0] */;

//
// R28159 (0x6DFF) - DSP2 PM 1535
//
pub const WM5100_DSP2_PM_END_MASK: c_uint = 0xFFFF  /* DSP2_PM_END - [15:0] */;

//
// R28672 (0x7000) - DSP2 ZM 0
//
pub const WM5100_DSP2_ZM_START_1_MASK: c_uint = 0x00FF  /* DSP2_ZM_START - [7:0] */;

//
// R28673 (0x7001) - DSP2 ZM 1
//
pub const WM5100_DSP2_ZM_START_MASK: c_uint = 0xFFFF  /* DSP2_ZM_START - [15:0] */;

//
// R28674 (0x7002) - DSP2 ZM 2
//
pub const WM5100_DSP2_ZM_1_1_MASK: c_uint = 0x00FF  /* DSP2_ZM_1 - [7:0] */;

//
// R28675 (0x7003) - DSP2 ZM 3
//
pub const WM5100_DSP2_ZM_1_MASK: c_uint = 0xFFFF  /* DSP2_ZM_1 - [15:0] */;

//
// R30716 (0x77FC) - DSP2 ZM 2044
//
pub const WM5100_DSP2_ZM_1022_1_MASK: c_uint = 0x00FF  /* DSP2_ZM_1022 - [7:0] */;

//
// R30717 (0x77FD) - DSP2 ZM 2045
//
pub const WM5100_DSP2_ZM_1022_MASK: c_uint = 0xFFFF  /* DSP2_ZM_1022 - [15:0] */;

//
// R30718 (0x77FE) - DSP2 ZM 2046
//
pub const WM5100_DSP2_ZM_END_1_MASK: c_uint = 0x00FF  /* DSP2_ZM_END - [7:0] */;

//
// R30719 (0x77FF) - DSP2 ZM 2047
//
pub const WM5100_DSP2_ZM_END_MASK: c_uint = 0xFFFF  /* DSP2_ZM_END - [15:0] */;

//
// R32768 (0x8000) - DSP3 DM 0
//
pub const WM5100_DSP3_DM_START_1_MASK: c_uint = 0x00FF  /* DSP3_DM_START - [7:0] */;

//
// R32769 (0x8001) - DSP3 DM 1
//
pub const WM5100_DSP3_DM_START_MASK: c_uint = 0xFFFF  /* DSP3_DM_START - [15:0] */;

//
// R32770 (0x8002) - DSP3 DM 2
//
pub const WM5100_DSP3_DM_1_1_MASK: c_uint = 0x00FF  /* DSP3_DM_1 - [7:0] */;

//
// R32771 (0x8003) - DSP3 DM 3
//
pub const WM5100_DSP3_DM_1_MASK: c_uint = 0xFFFF  /* DSP3_DM_1 - [15:0] */;

//
// R33276 (0x81FC) - DSP3 DM 508
//
pub const WM5100_DSP3_DM_254_1_MASK: c_uint = 0x00FF  /* DSP3_DM_254 - [7:0] */;

//
// R33277 (0x81FD) - DSP3 DM 509
//
pub const WM5100_DSP3_DM_254_MASK: c_uint = 0xFFFF  /* DSP3_DM_254 - [15:0] */;

//
// R33278 (0x81FE) - DSP3 DM 510
//
pub const WM5100_DSP3_DM_END_1_MASK: c_uint = 0x00FF  /* DSP3_DM_END - [7:0] */;

//
// R33279 (0x81FF) - DSP3 DM 511
//
pub const WM5100_DSP3_DM_END_MASK: c_uint = 0xFFFF  /* DSP3_DM_END - [15:0] */;

//
// R34816 (0x8800) - DSP3 PM 0
//
pub const WM5100_DSP3_PM_START_2_MASK: c_uint = 0x00FF  /* DSP3_PM_START - [7:0] */;

//
// R34817 (0x8801) - DSP3 PM 1
//
pub const WM5100_DSP3_PM_START_1_MASK: c_uint = 0xFFFF  /* DSP3_PM_START - [15:0] */;

//
// R34818 (0x8802) - DSP3 PM 2
//
pub const WM5100_DSP3_PM_START_MASK: c_uint = 0xFFFF  /* DSP3_PM_START - [15:0] */;

//
// R34819 (0x8803) - DSP3 PM 3
//
pub const WM5100_DSP3_PM_1_2_MASK: c_uint = 0x00FF  /* DSP3_PM_1 - [7:0] */;

//
// R34820 (0x8804) - DSP3 PM 4
//
pub const WM5100_DSP3_PM_1_1_MASK: c_uint = 0xFFFF  /* DSP3_PM_1 - [15:0] */;

//
// R34821 (0x8805) - DSP3 PM 5
//
pub const WM5100_DSP3_PM_1_MASK: c_uint = 0xFFFF  /* DSP3_PM_1 - [15:0] */;

//
// R36346 (0x8DFA) - DSP3 PM 1530
//
pub const WM5100_DSP3_PM_510_2_MASK: c_uint = 0x00FF  /* DSP3_PM_510 - [7:0] */;

//
// R36347 (0x8DFB) - DSP3 PM 1531
//
pub const WM5100_DSP3_PM_510_1_MASK: c_uint = 0xFFFF  /* DSP3_PM_510 - [15:0] */;

//
// R36348 (0x8DFC) - DSP3 PM 1532
//
pub const WM5100_DSP3_PM_510_MASK: c_uint = 0xFFFF  /* DSP3_PM_510 - [15:0] */;

//
// R36349 (0x8DFD) - DSP3 PM 1533
//
pub const WM5100_DSP3_PM_END_2_MASK: c_uint = 0x00FF  /* DSP3_PM_END - [7:0] */;

//
// R36350 (0x8DFE) - DSP3 PM 1534
//
pub const WM5100_DSP3_PM_END_1_MASK: c_uint = 0xFFFF  /* DSP3_PM_END - [15:0] */;

//
// R36351 (0x8DFF) - DSP3 PM 1535
//
pub const WM5100_DSP3_PM_END_MASK: c_uint = 0xFFFF  /* DSP3_PM_END - [15:0] */;

//
// R36864 (0x9000) - DSP3 ZM 0
//
pub const WM5100_DSP3_ZM_START_1_MASK: c_uint = 0x00FF  /* DSP3_ZM_START - [7:0] */;

//
// R36865 (0x9001) - DSP3 ZM 1
//
pub const WM5100_DSP3_ZM_START_MASK: c_uint = 0xFFFF  /* DSP3_ZM_START - [15:0] */;

//
// R36866 (0x9002) - DSP3 ZM 2
//
pub const WM5100_DSP3_ZM_1_1_MASK: c_uint = 0x00FF  /* DSP3_ZM_1 - [7:0] */;

//
// R36867 (0x9003) - DSP3 ZM 3
//
pub const WM5100_DSP3_ZM_1_MASK: c_uint = 0xFFFF  /* DSP3_ZM_1 - [15:0] */;

//
// R38908 (0x97FC) - DSP3 ZM 2044
//
pub const WM5100_DSP3_ZM_1022_1_MASK: c_uint = 0x00FF  /* DSP3_ZM_1022 - [7:0] */;

//
// R38909 (0x97FD) - DSP3 ZM 2045
//
pub const WM5100_DSP3_ZM_1022_MASK: c_uint = 0xFFFF  /* DSP3_ZM_1022 - [15:0] */;

//
// R38910 (0x97FE) - DSP3 ZM 2046
//
pub const WM5100_DSP3_ZM_END_1_MASK: c_uint = 0x00FF  /* DSP3_ZM_END - [7:0] */;

//
// R38911 (0x97FF) - DSP3 ZM 2047
//
pub const WM5100_DSP3_ZM_END_MASK: c_uint = 0xFFFF  /* DSP3_ZM_END - [15:0] */;

extern "C" {
    pub fn wm5100_readable_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn wm5100_volatile_register(dev: *mut device, reg: c_uint) -> bool;
}
