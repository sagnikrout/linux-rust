//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8995.h
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
// wm8995.h  --  WM8995 ALSA SoC Audio driver
//
// Copyright 2010 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
//

//
// Register values.
//
pub const WM8995_SOFTWARE_RESET: c_uint = 0x00;
pub const WM8995_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8995_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8995_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8995_POWER_MANAGEMENT_4: c_uint = 0x04;
pub const WM8995_POWER_MANAGEMENT_5: c_uint = 0x05;
pub const WM8995_LEFT_LINE_INPUT_1_VOLUME: c_uint = 0x10;
pub const WM8995_RIGHT_LINE_INPUT_1_VOLUME: c_uint = 0x11;
pub const WM8995_LEFT_LINE_INPUT_CONTROL: c_uint = 0x12;
pub const WM8995_DAC1_LEFT_VOLUME: c_uint = 0x18;
pub const WM8995_DAC1_RIGHT_VOLUME: c_uint = 0x19;
pub const WM8995_DAC2_LEFT_VOLUME: c_uint = 0x1A;
pub const WM8995_DAC2_RIGHT_VOLUME: c_uint = 0x1B;
pub const WM8995_OUTPUT_VOLUME_ZC_1: c_uint = 0x1C;
pub const WM8995_MICBIAS_1: c_uint = 0x20;
pub const WM8995_MICBIAS_2: c_uint = 0x21;
pub const WM8995_LDO_1: c_uint = 0x28;
pub const WM8995_LDO_2: c_uint = 0x29;
pub const WM8995_ACCESSORY_DETECT_MODE1: c_uint = 0x30;
pub const WM8995_ACCESSORY_DETECT_MODE2: c_uint = 0x31;
pub const WM8995_HEADPHONE_DETECT1: c_uint = 0x34;
pub const WM8995_HEADPHONE_DETECT2: c_uint = 0x35;
pub const WM8995_MIC_DETECT_1: c_uint = 0x38;
pub const WM8995_MIC_DETECT_2: c_uint = 0x39;
pub const WM8995_CHARGE_PUMP_1: c_uint = 0x40;
pub const WM8995_CLASS_W_1: c_uint = 0x45;
pub const WM8995_DC_SERVO_1: c_uint = 0x50;
pub const WM8995_DC_SERVO_2: c_uint = 0x51;
pub const WM8995_DC_SERVO_3: c_uint = 0x52;
pub const WM8995_DC_SERVO_5: c_uint = 0x54;
pub const WM8995_DC_SERVO_6: c_uint = 0x55;
pub const WM8995_DC_SERVO_7: c_uint = 0x56;
pub const WM8995_DC_SERVO_READBACK_0: c_uint = 0x57;
pub const WM8995_ANALOGUE_HP_1: c_uint = 0x60;
pub const WM8995_ANALOGUE_HP_2: c_uint = 0x61;
pub const WM8995_CHIP_REVISION: c_uint = 0x100;
pub const WM8995_CONTROL_INTERFACE_1: c_uint = 0x101;
pub const WM8995_CONTROL_INTERFACE_2: c_uint = 0x102;
pub const WM8995_WRITE_SEQUENCER_CTRL_1: c_uint = 0x110;
pub const WM8995_WRITE_SEQUENCER_CTRL_2: c_uint = 0x111;
pub const WM8995_AIF1_CLOCKING_1: c_uint = 0x200;
pub const WM8995_AIF1_CLOCKING_2: c_uint = 0x201;
pub const WM8995_AIF2_CLOCKING_1: c_uint = 0x204;
pub const WM8995_AIF2_CLOCKING_2: c_uint = 0x205;
pub const WM8995_CLOCKING_1: c_uint = 0x208;
pub const WM8995_CLOCKING_2: c_uint = 0x209;
pub const WM8995_AIF1_RATE: c_uint = 0x210;
pub const WM8995_AIF2_RATE: c_uint = 0x211;
pub const WM8995_RATE_STATUS: c_uint = 0x212;
pub const WM8995_FLL1_CONTROL_1: c_uint = 0x220;
pub const WM8995_FLL1_CONTROL_2: c_uint = 0x221;
pub const WM8995_FLL1_CONTROL_3: c_uint = 0x222;
pub const WM8995_FLL1_CONTROL_4: c_uint = 0x223;
pub const WM8995_FLL1_CONTROL_5: c_uint = 0x224;
pub const WM8995_FLL2_CONTROL_1: c_uint = 0x240;
pub const WM8995_FLL2_CONTROL_2: c_uint = 0x241;
pub const WM8995_FLL2_CONTROL_3: c_uint = 0x242;
pub const WM8995_FLL2_CONTROL_4: c_uint = 0x243;
pub const WM8995_FLL2_CONTROL_5: c_uint = 0x244;
pub const WM8995_AIF1_CONTROL_1: c_uint = 0x300;
pub const WM8995_AIF1_CONTROL_2: c_uint = 0x301;
pub const WM8995_AIF1_MASTER_SLAVE: c_uint = 0x302;
pub const WM8995_AIF1_BCLK: c_uint = 0x303;
pub const WM8995_AIF1ADC_LRCLK: c_uint = 0x304;
pub const WM8995_AIF1DAC_LRCLK: c_uint = 0x305;
pub const WM8995_AIF1DAC_DATA: c_uint = 0x306;
pub const WM8995_AIF1ADC_DATA: c_uint = 0x307;
pub const WM8995_AIF2_CONTROL_1: c_uint = 0x310;
pub const WM8995_AIF2_CONTROL_2: c_uint = 0x311;
pub const WM8995_AIF2_MASTER_SLAVE: c_uint = 0x312;
pub const WM8995_AIF2_BCLK: c_uint = 0x313;
pub const WM8995_AIF2ADC_LRCLK: c_uint = 0x314;
pub const WM8995_AIF2DAC_LRCLK: c_uint = 0x315;
pub const WM8995_AIF2DAC_DATA: c_uint = 0x316;
pub const WM8995_AIF2ADC_DATA: c_uint = 0x317;
pub const WM8995_AIF1_ADC1_LEFT_VOLUME: c_uint = 0x400;
pub const WM8995_AIF1_ADC1_RIGHT_VOLUME: c_uint = 0x401;
pub const WM8995_AIF1_DAC1_LEFT_VOLUME: c_uint = 0x402;
pub const WM8995_AIF1_DAC1_RIGHT_VOLUME: c_uint = 0x403;
pub const WM8995_AIF1_ADC2_LEFT_VOLUME: c_uint = 0x404;
pub const WM8995_AIF1_ADC2_RIGHT_VOLUME: c_uint = 0x405;
pub const WM8995_AIF1_DAC2_LEFT_VOLUME: c_uint = 0x406;
pub const WM8995_AIF1_DAC2_RIGHT_VOLUME: c_uint = 0x407;
pub const WM8995_AIF1_ADC1_FILTERS: c_uint = 0x410;
pub const WM8995_AIF1_ADC2_FILTERS: c_uint = 0x411;
pub const WM8995_AIF1_DAC1_FILTERS_1: c_uint = 0x420;
pub const WM8995_AIF1_DAC1_FILTERS_2: c_uint = 0x421;
pub const WM8995_AIF1_DAC2_FILTERS_1: c_uint = 0x422;
pub const WM8995_AIF1_DAC2_FILTERS_2: c_uint = 0x423;
pub const WM8995_AIF1_DRC1_1: c_uint = 0x440;
pub const WM8995_AIF1_DRC1_2: c_uint = 0x441;
pub const WM8995_AIF1_DRC1_3: c_uint = 0x442;
pub const WM8995_AIF1_DRC1_4: c_uint = 0x443;
pub const WM8995_AIF1_DRC1_5: c_uint = 0x444;
pub const WM8995_AIF1_DRC2_1: c_uint = 0x450;
pub const WM8995_AIF1_DRC2_2: c_uint = 0x451;
pub const WM8995_AIF1_DRC2_3: c_uint = 0x452;
pub const WM8995_AIF1_DRC2_4: c_uint = 0x453;
pub const WM8995_AIF1_DRC2_5: c_uint = 0x454;
pub const WM8995_AIF1_DAC1_EQ_GAINS_1: c_uint = 0x480;
pub const WM8995_AIF1_DAC1_EQ_GAINS_2: c_uint = 0x481;
pub const WM8995_AIF1_DAC1_EQ_BAND_1_A: c_uint = 0x482;
pub const WM8995_AIF1_DAC1_EQ_BAND_1_B: c_uint = 0x483;
pub const WM8995_AIF1_DAC1_EQ_BAND_1_PG: c_uint = 0x484;
pub const WM8995_AIF1_DAC1_EQ_BAND_2_A: c_uint = 0x485;
pub const WM8995_AIF1_DAC1_EQ_BAND_2_B: c_uint = 0x486;
pub const WM8995_AIF1_DAC1_EQ_BAND_2_C: c_uint = 0x487;
pub const WM8995_AIF1_DAC1_EQ_BAND_2_PG: c_uint = 0x488;
pub const WM8995_AIF1_DAC1_EQ_BAND_3_A: c_uint = 0x489;
pub const WM8995_AIF1_DAC1_EQ_BAND_3_B: c_uint = 0x48A;
pub const WM8995_AIF1_DAC1_EQ_BAND_3_C: c_uint = 0x48B;
pub const WM8995_AIF1_DAC1_EQ_BAND_3_PG: c_uint = 0x48C;
pub const WM8995_AIF1_DAC1_EQ_BAND_4_A: c_uint = 0x48D;
pub const WM8995_AIF1_DAC1_EQ_BAND_4_B: c_uint = 0x48E;
pub const WM8995_AIF1_DAC1_EQ_BAND_4_C: c_uint = 0x48F;
pub const WM8995_AIF1_DAC1_EQ_BAND_4_PG: c_uint = 0x490;
pub const WM8995_AIF1_DAC1_EQ_BAND_5_A: c_uint = 0x491;
pub const WM8995_AIF1_DAC1_EQ_BAND_5_B: c_uint = 0x492;
pub const WM8995_AIF1_DAC1_EQ_BAND_5_PG: c_uint = 0x493;
pub const WM8995_AIF1_DAC2_EQ_GAINS_1: c_uint = 0x4A0;
pub const WM8995_AIF1_DAC2_EQ_GAINS_2: c_uint = 0x4A1;
pub const WM8995_AIF1_DAC2_EQ_BAND_1_A: c_uint = 0x4A2;
pub const WM8995_AIF1_DAC2_EQ_BAND_1_B: c_uint = 0x4A3;
pub const WM8995_AIF1_DAC2_EQ_BAND_1_PG: c_uint = 0x4A4;
pub const WM8995_AIF1_DAC2_EQ_BAND_2_A: c_uint = 0x4A5;
pub const WM8995_AIF1_DAC2_EQ_BAND_2_B: c_uint = 0x4A6;
pub const WM8995_AIF1_DAC2_EQ_BAND_2_C: c_uint = 0x4A7;
pub const WM8995_AIF1_DAC2_EQ_BAND_2_PG: c_uint = 0x4A8;
pub const WM8995_AIF1_DAC2_EQ_BAND_3_A: c_uint = 0x4A9;
pub const WM8995_AIF1_DAC2_EQ_BAND_3_B: c_uint = 0x4AA;
pub const WM8995_AIF1_DAC2_EQ_BAND_3_C: c_uint = 0x4AB;
pub const WM8995_AIF1_DAC2_EQ_BAND_3_PG: c_uint = 0x4AC;
pub const WM8995_AIF1_DAC2_EQ_BAND_4_A: c_uint = 0x4AD;
pub const WM8995_AIF1_DAC2_EQ_BAND_4_B: c_uint = 0x4AE;
pub const WM8995_AIF1_DAC2_EQ_BAND_4_C: c_uint = 0x4AF;
pub const WM8995_AIF1_DAC2_EQ_BAND_4_PG: c_uint = 0x4B0;
pub const WM8995_AIF1_DAC2_EQ_BAND_5_A: c_uint = 0x4B1;
pub const WM8995_AIF1_DAC2_EQ_BAND_5_B: c_uint = 0x4B2;
pub const WM8995_AIF1_DAC2_EQ_BAND_5_PG: c_uint = 0x4B3;
pub const WM8995_AIF2_ADC_LEFT_VOLUME: c_uint = 0x500;
pub const WM8995_AIF2_ADC_RIGHT_VOLUME: c_uint = 0x501;
pub const WM8995_AIF2_DAC_LEFT_VOLUME: c_uint = 0x502;
pub const WM8995_AIF2_DAC_RIGHT_VOLUME: c_uint = 0x503;
pub const WM8995_AIF2_ADC_FILTERS: c_uint = 0x510;
pub const WM8995_AIF2_DAC_FILTERS_1: c_uint = 0x520;
pub const WM8995_AIF2_DAC_FILTERS_2: c_uint = 0x521;
pub const WM8995_AIF2_DRC_1: c_uint = 0x540;
pub const WM8995_AIF2_DRC_2: c_uint = 0x541;
pub const WM8995_AIF2_DRC_3: c_uint = 0x542;
pub const WM8995_AIF2_DRC_4: c_uint = 0x543;
pub const WM8995_AIF2_DRC_5: c_uint = 0x544;
pub const WM8995_AIF2_EQ_GAINS_1: c_uint = 0x580;
pub const WM8995_AIF2_EQ_GAINS_2: c_uint = 0x581;
pub const WM8995_AIF2_EQ_BAND_1_A: c_uint = 0x582;
pub const WM8995_AIF2_EQ_BAND_1_B: c_uint = 0x583;
pub const WM8995_AIF2_EQ_BAND_1_PG: c_uint = 0x584;
pub const WM8995_AIF2_EQ_BAND_2_A: c_uint = 0x585;
pub const WM8995_AIF2_EQ_BAND_2_B: c_uint = 0x586;
pub const WM8995_AIF2_EQ_BAND_2_C: c_uint = 0x587;
pub const WM8995_AIF2_EQ_BAND_2_PG: c_uint = 0x588;
pub const WM8995_AIF2_EQ_BAND_3_A: c_uint = 0x589;
pub const WM8995_AIF2_EQ_BAND_3_B: c_uint = 0x58A;
pub const WM8995_AIF2_EQ_BAND_3_C: c_uint = 0x58B;
pub const WM8995_AIF2_EQ_BAND_3_PG: c_uint = 0x58C;
pub const WM8995_AIF2_EQ_BAND_4_A: c_uint = 0x58D;
pub const WM8995_AIF2_EQ_BAND_4_B: c_uint = 0x58E;
pub const WM8995_AIF2_EQ_BAND_4_C: c_uint = 0x58F;
pub const WM8995_AIF2_EQ_BAND_4_PG: c_uint = 0x590;
pub const WM8995_AIF2_EQ_BAND_5_A: c_uint = 0x591;
pub const WM8995_AIF2_EQ_BAND_5_B: c_uint = 0x592;
pub const WM8995_AIF2_EQ_BAND_5_PG: c_uint = 0x593;
pub const WM8995_DAC1_MIXER_VOLUMES: c_uint = 0x600;
pub const WM8995_DAC1_LEFT_MIXER_ROUTING: c_uint = 0x601;
pub const WM8995_DAC1_RIGHT_MIXER_ROUTING: c_uint = 0x602;
pub const WM8995_DAC2_MIXER_VOLUMES: c_uint = 0x603;
pub const WM8995_DAC2_LEFT_MIXER_ROUTING: c_uint = 0x604;
pub const WM8995_DAC2_RIGHT_MIXER_ROUTING: c_uint = 0x605;
pub const WM8995_AIF1_ADC1_LEFT_MIXER_ROUTING: c_uint = 0x606;
pub const WM8995_AIF1_ADC1_RIGHT_MIXER_ROUTING: c_uint = 0x607;
pub const WM8995_AIF1_ADC2_LEFT_MIXER_ROUTING: c_uint = 0x608;
pub const WM8995_AIF1_ADC2_RIGHT_MIXER_ROUTING: c_uint = 0x609;
pub const WM8995_DAC_SOFTMUTE: c_uint = 0x610;
pub const WM8995_OVERSAMPLING: c_uint = 0x620;
pub const WM8995_SIDETONE: c_uint = 0x621;
pub const WM8995_GPIO_1: c_uint = 0x700;
pub const WM8995_GPIO_2: c_uint = 0x701;
pub const WM8995_GPIO_3: c_uint = 0x702;
pub const WM8995_GPIO_4: c_uint = 0x703;
pub const WM8995_GPIO_5: c_uint = 0x704;
pub const WM8995_GPIO_6: c_uint = 0x705;
pub const WM8995_GPIO_7: c_uint = 0x706;
pub const WM8995_GPIO_8: c_uint = 0x707;
pub const WM8995_GPIO_9: c_uint = 0x708;
pub const WM8995_GPIO_10: c_uint = 0x709;
pub const WM8995_GPIO_11: c_uint = 0x70A;
pub const WM8995_GPIO_12: c_uint = 0x70B;
pub const WM8995_GPIO_13: c_uint = 0x70C;
pub const WM8995_GPIO_14: c_uint = 0x70D;
pub const WM8995_PULL_CONTROL_1: c_uint = 0x720;
pub const WM8995_PULL_CONTROL_2: c_uint = 0x721;
pub const WM8995_INTERRUPT_STATUS_1: c_uint = 0x730;
pub const WM8995_INTERRUPT_STATUS_2: c_uint = 0x731;
pub const WM8995_INTERRUPT_RAW_STATUS_2: c_uint = 0x732;
pub const WM8995_INTERRUPT_STATUS_1_MASK: c_uint = 0x738;
pub const WM8995_INTERRUPT_STATUS_2_MASK: c_uint = 0x739;
pub const WM8995_INTERRUPT_CONTROL: c_uint = 0x740;
pub const WM8995_LEFT_PDM_SPEAKER_1: c_uint = 0x800;
pub const WM8995_RIGHT_PDM_SPEAKER_1: c_uint = 0x801;
pub const WM8995_PDM_SPEAKER_1_MUTE_SEQUENCE: c_uint = 0x802;
pub const WM8995_LEFT_PDM_SPEAKER_2: c_uint = 0x808;
pub const WM8995_RIGHT_PDM_SPEAKER_2: c_uint = 0x809;
pub const WM8995_PDM_SPEAKER_2_MUTE_SEQUENCE: c_uint = 0x80A;
pub const WM8995_WRITE_SEQUENCER_0: c_uint = 0x3000;
pub const WM8995_WRITE_SEQUENCER_1: c_uint = 0x3001;
pub const WM8995_WRITE_SEQUENCER_2: c_uint = 0x3002;
pub const WM8995_WRITE_SEQUENCER_3: c_uint = 0x3003;
pub const WM8995_WRITE_SEQUENCER_4: c_uint = 0x3004;
pub const WM8995_WRITE_SEQUENCER_5: c_uint = 0x3005;
pub const WM8995_WRITE_SEQUENCER_6: c_uint = 0x3006;
pub const WM8995_WRITE_SEQUENCER_7: c_uint = 0x3007;
pub const WM8995_WRITE_SEQUENCER_8: c_uint = 0x3008;
pub const WM8995_WRITE_SEQUENCER_9: c_uint = 0x3009;
pub const WM8995_WRITE_SEQUENCER_10: c_uint = 0x300A;
pub const WM8995_WRITE_SEQUENCER_11: c_uint = 0x300B;
pub const WM8995_WRITE_SEQUENCER_12: c_uint = 0x300C;
pub const WM8995_WRITE_SEQUENCER_13: c_uint = 0x300D;
pub const WM8995_WRITE_SEQUENCER_14: c_uint = 0x300E;
pub const WM8995_WRITE_SEQUENCER_15: c_uint = 0x300F;
pub const WM8995_WRITE_SEQUENCER_16: c_uint = 0x3010;
pub const WM8995_WRITE_SEQUENCER_17: c_uint = 0x3011;
pub const WM8995_WRITE_SEQUENCER_18: c_uint = 0x3012;
pub const WM8995_WRITE_SEQUENCER_19: c_uint = 0x3013;
pub const WM8995_WRITE_SEQUENCER_20: c_uint = 0x3014;
pub const WM8995_WRITE_SEQUENCER_21: c_uint = 0x3015;
pub const WM8995_WRITE_SEQUENCER_22: c_uint = 0x3016;
pub const WM8995_WRITE_SEQUENCER_23: c_uint = 0x3017;
pub const WM8995_WRITE_SEQUENCER_24: c_uint = 0x3018;
pub const WM8995_WRITE_SEQUENCER_25: c_uint = 0x3019;
pub const WM8995_WRITE_SEQUENCER_26: c_uint = 0x301A;
pub const WM8995_WRITE_SEQUENCER_27: c_uint = 0x301B;
pub const WM8995_WRITE_SEQUENCER_28: c_uint = 0x301C;
pub const WM8995_WRITE_SEQUENCER_29: c_uint = 0x301D;
pub const WM8995_WRITE_SEQUENCER_30: c_uint = 0x301E;
pub const WM8995_WRITE_SEQUENCER_31: c_uint = 0x301F;
pub const WM8995_WRITE_SEQUENCER_32: c_uint = 0x3020;
pub const WM8995_WRITE_SEQUENCER_33: c_uint = 0x3021;
pub const WM8995_WRITE_SEQUENCER_34: c_uint = 0x3022;
pub const WM8995_WRITE_SEQUENCER_35: c_uint = 0x3023;
pub const WM8995_WRITE_SEQUENCER_36: c_uint = 0x3024;
pub const WM8995_WRITE_SEQUENCER_37: c_uint = 0x3025;
pub const WM8995_WRITE_SEQUENCER_38: c_uint = 0x3026;
pub const WM8995_WRITE_SEQUENCER_39: c_uint = 0x3027;
pub const WM8995_WRITE_SEQUENCER_40: c_uint = 0x3028;
pub const WM8995_WRITE_SEQUENCER_41: c_uint = 0x3029;
pub const WM8995_WRITE_SEQUENCER_42: c_uint = 0x302A;
pub const WM8995_WRITE_SEQUENCER_43: c_uint = 0x302B;
pub const WM8995_WRITE_SEQUENCER_44: c_uint = 0x302C;
pub const WM8995_WRITE_SEQUENCER_45: c_uint = 0x302D;
pub const WM8995_WRITE_SEQUENCER_46: c_uint = 0x302E;
pub const WM8995_WRITE_SEQUENCER_47: c_uint = 0x302F;
pub const WM8995_WRITE_SEQUENCER_48: c_uint = 0x3030;
pub const WM8995_WRITE_SEQUENCER_49: c_uint = 0x3031;
pub const WM8995_WRITE_SEQUENCER_50: c_uint = 0x3032;
pub const WM8995_WRITE_SEQUENCER_51: c_uint = 0x3033;
pub const WM8995_WRITE_SEQUENCER_52: c_uint = 0x3034;
pub const WM8995_WRITE_SEQUENCER_53: c_uint = 0x3035;
pub const WM8995_WRITE_SEQUENCER_54: c_uint = 0x3036;
pub const WM8995_WRITE_SEQUENCER_55: c_uint = 0x3037;
pub const WM8995_WRITE_SEQUENCER_56: c_uint = 0x3038;
pub const WM8995_WRITE_SEQUENCER_57: c_uint = 0x3039;
pub const WM8995_WRITE_SEQUENCER_58: c_uint = 0x303A;
pub const WM8995_WRITE_SEQUENCER_59: c_uint = 0x303B;
pub const WM8995_WRITE_SEQUENCER_60: c_uint = 0x303C;
pub const WM8995_WRITE_SEQUENCER_61: c_uint = 0x303D;
pub const WM8995_WRITE_SEQUENCER_62: c_uint = 0x303E;
pub const WM8995_WRITE_SEQUENCER_63: c_uint = 0x303F;
pub const WM8995_WRITE_SEQUENCER_64: c_uint = 0x3040;
pub const WM8995_WRITE_SEQUENCER_65: c_uint = 0x3041;
pub const WM8995_WRITE_SEQUENCER_66: c_uint = 0x3042;
pub const WM8995_WRITE_SEQUENCER_67: c_uint = 0x3043;
pub const WM8995_WRITE_SEQUENCER_68: c_uint = 0x3044;
pub const WM8995_WRITE_SEQUENCER_69: c_uint = 0x3045;
pub const WM8995_WRITE_SEQUENCER_70: c_uint = 0x3046;
pub const WM8995_WRITE_SEQUENCER_71: c_uint = 0x3047;
pub const WM8995_WRITE_SEQUENCER_72: c_uint = 0x3048;
pub const WM8995_WRITE_SEQUENCER_73: c_uint = 0x3049;
pub const WM8995_WRITE_SEQUENCER_74: c_uint = 0x304A;
pub const WM8995_WRITE_SEQUENCER_75: c_uint = 0x304B;
pub const WM8995_WRITE_SEQUENCER_76: c_uint = 0x304C;
pub const WM8995_WRITE_SEQUENCER_77: c_uint = 0x304D;
pub const WM8995_WRITE_SEQUENCER_78: c_uint = 0x304E;
pub const WM8995_WRITE_SEQUENCER_79: c_uint = 0x304F;
pub const WM8995_WRITE_SEQUENCER_80: c_uint = 0x3050;
pub const WM8995_WRITE_SEQUENCER_81: c_uint = 0x3051;
pub const WM8995_WRITE_SEQUENCER_82: c_uint = 0x3052;
pub const WM8995_WRITE_SEQUENCER_83: c_uint = 0x3053;
pub const WM8995_WRITE_SEQUENCER_84: c_uint = 0x3054;
pub const WM8995_WRITE_SEQUENCER_85: c_uint = 0x3055;
pub const WM8995_WRITE_SEQUENCER_86: c_uint = 0x3056;
pub const WM8995_WRITE_SEQUENCER_87: c_uint = 0x3057;
pub const WM8995_WRITE_SEQUENCER_88: c_uint = 0x3058;
pub const WM8995_WRITE_SEQUENCER_89: c_uint = 0x3059;
pub const WM8995_WRITE_SEQUENCER_90: c_uint = 0x305A;
pub const WM8995_WRITE_SEQUENCER_91: c_uint = 0x305B;
pub const WM8995_WRITE_SEQUENCER_92: c_uint = 0x305C;
pub const WM8995_WRITE_SEQUENCER_93: c_uint = 0x305D;
pub const WM8995_WRITE_SEQUENCER_94: c_uint = 0x305E;
pub const WM8995_WRITE_SEQUENCER_95: c_uint = 0x305F;
pub const WM8995_WRITE_SEQUENCER_96: c_uint = 0x3060;
pub const WM8995_WRITE_SEQUENCER_97: c_uint = 0x3061;
pub const WM8995_WRITE_SEQUENCER_98: c_uint = 0x3062;
pub const WM8995_WRITE_SEQUENCER_99: c_uint = 0x3063;
pub const WM8995_WRITE_SEQUENCER_100: c_uint = 0x3064;
pub const WM8995_WRITE_SEQUENCER_101: c_uint = 0x3065;
pub const WM8995_WRITE_SEQUENCER_102: c_uint = 0x3066;
pub const WM8995_WRITE_SEQUENCER_103: c_uint = 0x3067;
pub const WM8995_WRITE_SEQUENCER_104: c_uint = 0x3068;
pub const WM8995_WRITE_SEQUENCER_105: c_uint = 0x3069;
pub const WM8995_WRITE_SEQUENCER_106: c_uint = 0x306A;
pub const WM8995_WRITE_SEQUENCER_107: c_uint = 0x306B;
pub const WM8995_WRITE_SEQUENCER_108: c_uint = 0x306C;
pub const WM8995_WRITE_SEQUENCER_109: c_uint = 0x306D;
pub const WM8995_WRITE_SEQUENCER_110: c_uint = 0x306E;
pub const WM8995_WRITE_SEQUENCER_111: c_uint = 0x306F;
pub const WM8995_WRITE_SEQUENCER_112: c_uint = 0x3070;
pub const WM8995_WRITE_SEQUENCER_113: c_uint = 0x3071;
pub const WM8995_WRITE_SEQUENCER_114: c_uint = 0x3072;
pub const WM8995_WRITE_SEQUENCER_115: c_uint = 0x3073;
pub const WM8995_WRITE_SEQUENCER_116: c_uint = 0x3074;
pub const WM8995_WRITE_SEQUENCER_117: c_uint = 0x3075;
pub const WM8995_WRITE_SEQUENCER_118: c_uint = 0x3076;
pub const WM8995_WRITE_SEQUENCER_119: c_uint = 0x3077;
pub const WM8995_WRITE_SEQUENCER_120: c_uint = 0x3078;
pub const WM8995_WRITE_SEQUENCER_121: c_uint = 0x3079;
pub const WM8995_WRITE_SEQUENCER_122: c_uint = 0x307A;
pub const WM8995_WRITE_SEQUENCER_123: c_uint = 0x307B;
pub const WM8995_WRITE_SEQUENCER_124: c_uint = 0x307C;
pub const WM8995_WRITE_SEQUENCER_125: c_uint = 0x307D;
pub const WM8995_WRITE_SEQUENCER_126: c_uint = 0x307E;
pub const WM8995_WRITE_SEQUENCER_127: c_uint = 0x307F;
pub const WM8995_WRITE_SEQUENCER_128: c_uint = 0x3080;
pub const WM8995_WRITE_SEQUENCER_129: c_uint = 0x3081;
pub const WM8995_WRITE_SEQUENCER_130: c_uint = 0x3082;
pub const WM8995_WRITE_SEQUENCER_131: c_uint = 0x3083;
pub const WM8995_WRITE_SEQUENCER_132: c_uint = 0x3084;
pub const WM8995_WRITE_SEQUENCER_133: c_uint = 0x3085;
pub const WM8995_WRITE_SEQUENCER_134: c_uint = 0x3086;
pub const WM8995_WRITE_SEQUENCER_135: c_uint = 0x3087;
pub const WM8995_WRITE_SEQUENCER_136: c_uint = 0x3088;
pub const WM8995_WRITE_SEQUENCER_137: c_uint = 0x3089;
pub const WM8995_WRITE_SEQUENCER_138: c_uint = 0x308A;
pub const WM8995_WRITE_SEQUENCER_139: c_uint = 0x308B;
pub const WM8995_WRITE_SEQUENCER_140: c_uint = 0x308C;
pub const WM8995_WRITE_SEQUENCER_141: c_uint = 0x308D;
pub const WM8995_WRITE_SEQUENCER_142: c_uint = 0x308E;
pub const WM8995_WRITE_SEQUENCER_143: c_uint = 0x308F;
pub const WM8995_WRITE_SEQUENCER_144: c_uint = 0x3090;
pub const WM8995_WRITE_SEQUENCER_145: c_uint = 0x3091;
pub const WM8995_WRITE_SEQUENCER_146: c_uint = 0x3092;
pub const WM8995_WRITE_SEQUENCER_147: c_uint = 0x3093;
pub const WM8995_WRITE_SEQUENCER_148: c_uint = 0x3094;
pub const WM8995_WRITE_SEQUENCER_149: c_uint = 0x3095;
pub const WM8995_WRITE_SEQUENCER_150: c_uint = 0x3096;
pub const WM8995_WRITE_SEQUENCER_151: c_uint = 0x3097;
pub const WM8995_WRITE_SEQUENCER_152: c_uint = 0x3098;
pub const WM8995_WRITE_SEQUENCER_153: c_uint = 0x3099;
pub const WM8995_WRITE_SEQUENCER_154: c_uint = 0x309A;
pub const WM8995_WRITE_SEQUENCER_155: c_uint = 0x309B;
pub const WM8995_WRITE_SEQUENCER_156: c_uint = 0x309C;
pub const WM8995_WRITE_SEQUENCER_157: c_uint = 0x309D;
pub const WM8995_WRITE_SEQUENCER_158: c_uint = 0x309E;
pub const WM8995_WRITE_SEQUENCER_159: c_uint = 0x309F;
pub const WM8995_WRITE_SEQUENCER_160: c_uint = 0x30A0;
pub const WM8995_WRITE_SEQUENCER_161: c_uint = 0x30A1;
pub const WM8995_WRITE_SEQUENCER_162: c_uint = 0x30A2;
pub const WM8995_WRITE_SEQUENCER_163: c_uint = 0x30A3;
pub const WM8995_WRITE_SEQUENCER_164: c_uint = 0x30A4;
pub const WM8995_WRITE_SEQUENCER_165: c_uint = 0x30A5;
pub const WM8995_WRITE_SEQUENCER_166: c_uint = 0x30A6;
pub const WM8995_WRITE_SEQUENCER_167: c_uint = 0x30A7;
pub const WM8995_WRITE_SEQUENCER_168: c_uint = 0x30A8;
pub const WM8995_WRITE_SEQUENCER_169: c_uint = 0x30A9;
pub const WM8995_WRITE_SEQUENCER_170: c_uint = 0x30AA;
pub const WM8995_WRITE_SEQUENCER_171: c_uint = 0x30AB;
pub const WM8995_WRITE_SEQUENCER_172: c_uint = 0x30AC;
pub const WM8995_WRITE_SEQUENCER_173: c_uint = 0x30AD;
pub const WM8995_WRITE_SEQUENCER_174: c_uint = 0x30AE;
pub const WM8995_WRITE_SEQUENCER_175: c_uint = 0x30AF;
pub const WM8995_WRITE_SEQUENCER_176: c_uint = 0x30B0;
pub const WM8995_WRITE_SEQUENCER_177: c_uint = 0x30B1;
pub const WM8995_WRITE_SEQUENCER_178: c_uint = 0x30B2;
pub const WM8995_WRITE_SEQUENCER_179: c_uint = 0x30B3;
pub const WM8995_WRITE_SEQUENCER_180: c_uint = 0x30B4;
pub const WM8995_WRITE_SEQUENCER_181: c_uint = 0x30B5;
pub const WM8995_WRITE_SEQUENCER_182: c_uint = 0x30B6;
pub const WM8995_WRITE_SEQUENCER_183: c_uint = 0x30B7;
pub const WM8995_WRITE_SEQUENCER_184: c_uint = 0x30B8;
pub const WM8995_WRITE_SEQUENCER_185: c_uint = 0x30B9;
pub const WM8995_WRITE_SEQUENCER_186: c_uint = 0x30BA;
pub const WM8995_WRITE_SEQUENCER_187: c_uint = 0x30BB;
pub const WM8995_WRITE_SEQUENCER_188: c_uint = 0x30BC;
pub const WM8995_WRITE_SEQUENCER_189: c_uint = 0x30BD;
pub const WM8995_WRITE_SEQUENCER_190: c_uint = 0x30BE;
pub const WM8995_WRITE_SEQUENCER_191: c_uint = 0x30BF;
pub const WM8995_WRITE_SEQUENCER_192: c_uint = 0x30C0;
pub const WM8995_WRITE_SEQUENCER_193: c_uint = 0x30C1;
pub const WM8995_WRITE_SEQUENCER_194: c_uint = 0x30C2;
pub const WM8995_WRITE_SEQUENCER_195: c_uint = 0x30C3;
pub const WM8995_WRITE_SEQUENCER_196: c_uint = 0x30C4;
pub const WM8995_WRITE_SEQUENCER_197: c_uint = 0x30C5;
pub const WM8995_WRITE_SEQUENCER_198: c_uint = 0x30C6;
pub const WM8995_WRITE_SEQUENCER_199: c_uint = 0x30C7;
pub const WM8995_WRITE_SEQUENCER_200: c_uint = 0x30C8;
pub const WM8995_WRITE_SEQUENCER_201: c_uint = 0x30C9;
pub const WM8995_WRITE_SEQUENCER_202: c_uint = 0x30CA;
pub const WM8995_WRITE_SEQUENCER_203: c_uint = 0x30CB;
pub const WM8995_WRITE_SEQUENCER_204: c_uint = 0x30CC;
pub const WM8995_WRITE_SEQUENCER_205: c_uint = 0x30CD;
pub const WM8995_WRITE_SEQUENCER_206: c_uint = 0x30CE;
pub const WM8995_WRITE_SEQUENCER_207: c_uint = 0x30CF;
pub const WM8995_WRITE_SEQUENCER_208: c_uint = 0x30D0;
pub const WM8995_WRITE_SEQUENCER_209: c_uint = 0x30D1;
pub const WM8995_WRITE_SEQUENCER_210: c_uint = 0x30D2;
pub const WM8995_WRITE_SEQUENCER_211: c_uint = 0x30D3;
pub const WM8995_WRITE_SEQUENCER_212: c_uint = 0x30D4;
pub const WM8995_WRITE_SEQUENCER_213: c_uint = 0x30D5;
pub const WM8995_WRITE_SEQUENCER_214: c_uint = 0x30D6;
pub const WM8995_WRITE_SEQUENCER_215: c_uint = 0x30D7;
pub const WM8995_WRITE_SEQUENCER_216: c_uint = 0x30D8;
pub const WM8995_WRITE_SEQUENCER_217: c_uint = 0x30D9;
pub const WM8995_WRITE_SEQUENCER_218: c_uint = 0x30DA;
pub const WM8995_WRITE_SEQUENCER_219: c_uint = 0x30DB;
pub const WM8995_WRITE_SEQUENCER_220: c_uint = 0x30DC;
pub const WM8995_WRITE_SEQUENCER_221: c_uint = 0x30DD;
pub const WM8995_WRITE_SEQUENCER_222: c_uint = 0x30DE;
pub const WM8995_WRITE_SEQUENCER_223: c_uint = 0x30DF;
pub const WM8995_WRITE_SEQUENCER_224: c_uint = 0x30E0;
pub const WM8995_WRITE_SEQUENCER_225: c_uint = 0x30E1;
pub const WM8995_WRITE_SEQUENCER_226: c_uint = 0x30E2;
pub const WM8995_WRITE_SEQUENCER_227: c_uint = 0x30E3;
pub const WM8995_WRITE_SEQUENCER_228: c_uint = 0x30E4;
pub const WM8995_WRITE_SEQUENCER_229: c_uint = 0x30E5;
pub const WM8995_WRITE_SEQUENCER_230: c_uint = 0x30E6;
pub const WM8995_WRITE_SEQUENCER_231: c_uint = 0x30E7;
pub const WM8995_WRITE_SEQUENCER_232: c_uint = 0x30E8;
pub const WM8995_WRITE_SEQUENCER_233: c_uint = 0x30E9;
pub const WM8995_WRITE_SEQUENCER_234: c_uint = 0x30EA;
pub const WM8995_WRITE_SEQUENCER_235: c_uint = 0x30EB;
pub const WM8995_WRITE_SEQUENCER_236: c_uint = 0x30EC;
pub const WM8995_WRITE_SEQUENCER_237: c_uint = 0x30ED;
pub const WM8995_WRITE_SEQUENCER_238: c_uint = 0x30EE;
pub const WM8995_WRITE_SEQUENCER_239: c_uint = 0x30EF;
pub const WM8995_WRITE_SEQUENCER_240: c_uint = 0x30F0;
pub const WM8995_WRITE_SEQUENCER_241: c_uint = 0x30F1;
pub const WM8995_WRITE_SEQUENCER_242: c_uint = 0x30F2;
pub const WM8995_WRITE_SEQUENCER_243: c_uint = 0x30F3;
pub const WM8995_WRITE_SEQUENCER_244: c_uint = 0x30F4;
pub const WM8995_WRITE_SEQUENCER_245: c_uint = 0x30F5;
pub const WM8995_WRITE_SEQUENCER_246: c_uint = 0x30F6;
pub const WM8995_WRITE_SEQUENCER_247: c_uint = 0x30F7;
pub const WM8995_WRITE_SEQUENCER_248: c_uint = 0x30F8;
pub const WM8995_WRITE_SEQUENCER_249: c_uint = 0x30F9;
pub const WM8995_WRITE_SEQUENCER_250: c_uint = 0x30FA;
pub const WM8995_WRITE_SEQUENCER_251: c_uint = 0x30FB;
pub const WM8995_WRITE_SEQUENCER_252: c_uint = 0x30FC;
pub const WM8995_WRITE_SEQUENCER_253: c_uint = 0x30FD;
pub const WM8995_WRITE_SEQUENCER_254: c_uint = 0x30FE;
pub const WM8995_WRITE_SEQUENCER_255: c_uint = 0x30FF;
pub const WM8995_WRITE_SEQUENCER_256: c_uint = 0x3100;
pub const WM8995_WRITE_SEQUENCER_257: c_uint = 0x3101;
pub const WM8995_WRITE_SEQUENCER_258: c_uint = 0x3102;
pub const WM8995_WRITE_SEQUENCER_259: c_uint = 0x3103;
pub const WM8995_WRITE_SEQUENCER_260: c_uint = 0x3104;
pub const WM8995_WRITE_SEQUENCER_261: c_uint = 0x3105;
pub const WM8995_WRITE_SEQUENCER_262: c_uint = 0x3106;
pub const WM8995_WRITE_SEQUENCER_263: c_uint = 0x3107;
pub const WM8995_WRITE_SEQUENCER_264: c_uint = 0x3108;
pub const WM8995_WRITE_SEQUENCER_265: c_uint = 0x3109;
pub const WM8995_WRITE_SEQUENCER_266: c_uint = 0x310A;
pub const WM8995_WRITE_SEQUENCER_267: c_uint = 0x310B;
pub const WM8995_WRITE_SEQUENCER_268: c_uint = 0x310C;
pub const WM8995_WRITE_SEQUENCER_269: c_uint = 0x310D;
pub const WM8995_WRITE_SEQUENCER_270: c_uint = 0x310E;
pub const WM8995_WRITE_SEQUENCER_271: c_uint = 0x310F;
pub const WM8995_WRITE_SEQUENCER_272: c_uint = 0x3110;
pub const WM8995_WRITE_SEQUENCER_273: c_uint = 0x3111;
pub const WM8995_WRITE_SEQUENCER_274: c_uint = 0x3112;
pub const WM8995_WRITE_SEQUENCER_275: c_uint = 0x3113;
pub const WM8995_WRITE_SEQUENCER_276: c_uint = 0x3114;
pub const WM8995_WRITE_SEQUENCER_277: c_uint = 0x3115;
pub const WM8995_WRITE_SEQUENCER_278: c_uint = 0x3116;
pub const WM8995_WRITE_SEQUENCER_279: c_uint = 0x3117;
pub const WM8995_WRITE_SEQUENCER_280: c_uint = 0x3118;
pub const WM8995_WRITE_SEQUENCER_281: c_uint = 0x3119;
pub const WM8995_WRITE_SEQUENCER_282: c_uint = 0x311A;
pub const WM8995_WRITE_SEQUENCER_283: c_uint = 0x311B;
pub const WM8995_WRITE_SEQUENCER_284: c_uint = 0x311C;
pub const WM8995_WRITE_SEQUENCER_285: c_uint = 0x311D;
pub const WM8995_WRITE_SEQUENCER_286: c_uint = 0x311E;
pub const WM8995_WRITE_SEQUENCER_287: c_uint = 0x311F;
pub const WM8995_WRITE_SEQUENCER_288: c_uint = 0x3120;
pub const WM8995_WRITE_SEQUENCER_289: c_uint = 0x3121;
pub const WM8995_WRITE_SEQUENCER_290: c_uint = 0x3122;
pub const WM8995_WRITE_SEQUENCER_291: c_uint = 0x3123;
pub const WM8995_WRITE_SEQUENCER_292: c_uint = 0x3124;
pub const WM8995_WRITE_SEQUENCER_293: c_uint = 0x3125;
pub const WM8995_WRITE_SEQUENCER_294: c_uint = 0x3126;
pub const WM8995_WRITE_SEQUENCER_295: c_uint = 0x3127;
pub const WM8995_WRITE_SEQUENCER_296: c_uint = 0x3128;
pub const WM8995_WRITE_SEQUENCER_297: c_uint = 0x3129;
pub const WM8995_WRITE_SEQUENCER_298: c_uint = 0x312A;
pub const WM8995_WRITE_SEQUENCER_299: c_uint = 0x312B;
pub const WM8995_WRITE_SEQUENCER_300: c_uint = 0x312C;
pub const WM8995_WRITE_SEQUENCER_301: c_uint = 0x312D;
pub const WM8995_WRITE_SEQUENCER_302: c_uint = 0x312E;
pub const WM8995_WRITE_SEQUENCER_303: c_uint = 0x312F;
pub const WM8995_WRITE_SEQUENCER_304: c_uint = 0x3130;
pub const WM8995_WRITE_SEQUENCER_305: c_uint = 0x3131;
pub const WM8995_WRITE_SEQUENCER_306: c_uint = 0x3132;
pub const WM8995_WRITE_SEQUENCER_307: c_uint = 0x3133;
pub const WM8995_WRITE_SEQUENCER_308: c_uint = 0x3134;
pub const WM8995_WRITE_SEQUENCER_309: c_uint = 0x3135;
pub const WM8995_WRITE_SEQUENCER_310: c_uint = 0x3136;
pub const WM8995_WRITE_SEQUENCER_311: c_uint = 0x3137;
pub const WM8995_WRITE_SEQUENCER_312: c_uint = 0x3138;
pub const WM8995_WRITE_SEQUENCER_313: c_uint = 0x3139;
pub const WM8995_WRITE_SEQUENCER_314: c_uint = 0x313A;
pub const WM8995_WRITE_SEQUENCER_315: c_uint = 0x313B;
pub const WM8995_WRITE_SEQUENCER_316: c_uint = 0x313C;
pub const WM8995_WRITE_SEQUENCER_317: c_uint = 0x313D;
pub const WM8995_WRITE_SEQUENCER_318: c_uint = 0x313E;
pub const WM8995_WRITE_SEQUENCER_319: c_uint = 0x313F;
pub const WM8995_WRITE_SEQUENCER_320: c_uint = 0x3140;
pub const WM8995_WRITE_SEQUENCER_321: c_uint = 0x3141;
pub const WM8995_WRITE_SEQUENCER_322: c_uint = 0x3142;
pub const WM8995_WRITE_SEQUENCER_323: c_uint = 0x3143;
pub const WM8995_WRITE_SEQUENCER_324: c_uint = 0x3144;
pub const WM8995_WRITE_SEQUENCER_325: c_uint = 0x3145;
pub const WM8995_WRITE_SEQUENCER_326: c_uint = 0x3146;
pub const WM8995_WRITE_SEQUENCER_327: c_uint = 0x3147;
pub const WM8995_WRITE_SEQUENCER_328: c_uint = 0x3148;
pub const WM8995_WRITE_SEQUENCER_329: c_uint = 0x3149;
pub const WM8995_WRITE_SEQUENCER_330: c_uint = 0x314A;
pub const WM8995_WRITE_SEQUENCER_331: c_uint = 0x314B;
pub const WM8995_WRITE_SEQUENCER_332: c_uint = 0x314C;
pub const WM8995_WRITE_SEQUENCER_333: c_uint = 0x314D;
pub const WM8995_WRITE_SEQUENCER_334: c_uint = 0x314E;
pub const WM8995_WRITE_SEQUENCER_335: c_uint = 0x314F;
pub const WM8995_WRITE_SEQUENCER_336: c_uint = 0x3150;
pub const WM8995_WRITE_SEQUENCER_337: c_uint = 0x3151;
pub const WM8995_WRITE_SEQUENCER_338: c_uint = 0x3152;
pub const WM8995_WRITE_SEQUENCER_339: c_uint = 0x3153;
pub const WM8995_WRITE_SEQUENCER_340: c_uint = 0x3154;
pub const WM8995_WRITE_SEQUENCER_341: c_uint = 0x3155;
pub const WM8995_WRITE_SEQUENCER_342: c_uint = 0x3156;
pub const WM8995_WRITE_SEQUENCER_343: c_uint = 0x3157;
pub const WM8995_WRITE_SEQUENCER_344: c_uint = 0x3158;
pub const WM8995_WRITE_SEQUENCER_345: c_uint = 0x3159;
pub const WM8995_WRITE_SEQUENCER_346: c_uint = 0x315A;
pub const WM8995_WRITE_SEQUENCER_347: c_uint = 0x315B;
pub const WM8995_WRITE_SEQUENCER_348: c_uint = 0x315C;
pub const WM8995_WRITE_SEQUENCER_349: c_uint = 0x315D;
pub const WM8995_WRITE_SEQUENCER_350: c_uint = 0x315E;
pub const WM8995_WRITE_SEQUENCER_351: c_uint = 0x315F;
pub const WM8995_WRITE_SEQUENCER_352: c_uint = 0x3160;
pub const WM8995_WRITE_SEQUENCER_353: c_uint = 0x3161;
pub const WM8995_WRITE_SEQUENCER_354: c_uint = 0x3162;
pub const WM8995_WRITE_SEQUENCER_355: c_uint = 0x3163;
pub const WM8995_WRITE_SEQUENCER_356: c_uint = 0x3164;
pub const WM8995_WRITE_SEQUENCER_357: c_uint = 0x3165;
pub const WM8995_WRITE_SEQUENCER_358: c_uint = 0x3166;
pub const WM8995_WRITE_SEQUENCER_359: c_uint = 0x3167;
pub const WM8995_WRITE_SEQUENCER_360: c_uint = 0x3168;
pub const WM8995_WRITE_SEQUENCER_361: c_uint = 0x3169;
pub const WM8995_WRITE_SEQUENCER_362: c_uint = 0x316A;
pub const WM8995_WRITE_SEQUENCER_363: c_uint = 0x316B;
pub const WM8995_WRITE_SEQUENCER_364: c_uint = 0x316C;
pub const WM8995_WRITE_SEQUENCER_365: c_uint = 0x316D;
pub const WM8995_WRITE_SEQUENCER_366: c_uint = 0x316E;
pub const WM8995_WRITE_SEQUENCER_367: c_uint = 0x316F;
pub const WM8995_WRITE_SEQUENCER_368: c_uint = 0x3170;
pub const WM8995_WRITE_SEQUENCER_369: c_uint = 0x3171;
pub const WM8995_WRITE_SEQUENCER_370: c_uint = 0x3172;
pub const WM8995_WRITE_SEQUENCER_371: c_uint = 0x3173;
pub const WM8995_WRITE_SEQUENCER_372: c_uint = 0x3174;
pub const WM8995_WRITE_SEQUENCER_373: c_uint = 0x3175;
pub const WM8995_WRITE_SEQUENCER_374: c_uint = 0x3176;
pub const WM8995_WRITE_SEQUENCER_375: c_uint = 0x3177;
pub const WM8995_WRITE_SEQUENCER_376: c_uint = 0x3178;
pub const WM8995_WRITE_SEQUENCER_377: c_uint = 0x3179;
pub const WM8995_WRITE_SEQUENCER_378: c_uint = 0x317A;
pub const WM8995_WRITE_SEQUENCER_379: c_uint = 0x317B;
pub const WM8995_WRITE_SEQUENCER_380: c_uint = 0x317C;
pub const WM8995_WRITE_SEQUENCER_381: c_uint = 0x317D;
pub const WM8995_WRITE_SEQUENCER_382: c_uint = 0x317E;
pub const WM8995_WRITE_SEQUENCER_383: c_uint = 0x317F;
pub const WM8995_WRITE_SEQUENCER_384: c_uint = 0x3180;
pub const WM8995_WRITE_SEQUENCER_385: c_uint = 0x3181;
pub const WM8995_WRITE_SEQUENCER_386: c_uint = 0x3182;
pub const WM8995_WRITE_SEQUENCER_387: c_uint = 0x3183;
pub const WM8995_WRITE_SEQUENCER_388: c_uint = 0x3184;
pub const WM8995_WRITE_SEQUENCER_389: c_uint = 0x3185;
pub const WM8995_WRITE_SEQUENCER_390: c_uint = 0x3186;
pub const WM8995_WRITE_SEQUENCER_391: c_uint = 0x3187;
pub const WM8995_WRITE_SEQUENCER_392: c_uint = 0x3188;
pub const WM8995_WRITE_SEQUENCER_393: c_uint = 0x3189;
pub const WM8995_WRITE_SEQUENCER_394: c_uint = 0x318A;
pub const WM8995_WRITE_SEQUENCER_395: c_uint = 0x318B;
pub const WM8995_WRITE_SEQUENCER_396: c_uint = 0x318C;
pub const WM8995_WRITE_SEQUENCER_397: c_uint = 0x318D;
pub const WM8995_WRITE_SEQUENCER_398: c_uint = 0x318E;
pub const WM8995_WRITE_SEQUENCER_399: c_uint = 0x318F;
pub const WM8995_WRITE_SEQUENCER_400: c_uint = 0x3190;
pub const WM8995_WRITE_SEQUENCER_401: c_uint = 0x3191;
pub const WM8995_WRITE_SEQUENCER_402: c_uint = 0x3192;
pub const WM8995_WRITE_SEQUENCER_403: c_uint = 0x3193;
pub const WM8995_WRITE_SEQUENCER_404: c_uint = 0x3194;
pub const WM8995_WRITE_SEQUENCER_405: c_uint = 0x3195;
pub const WM8995_WRITE_SEQUENCER_406: c_uint = 0x3196;
pub const WM8995_WRITE_SEQUENCER_407: c_uint = 0x3197;
pub const WM8995_WRITE_SEQUENCER_408: c_uint = 0x3198;
pub const WM8995_WRITE_SEQUENCER_409: c_uint = 0x3199;
pub const WM8995_WRITE_SEQUENCER_410: c_uint = 0x319A;
pub const WM8995_WRITE_SEQUENCER_411: c_uint = 0x319B;
pub const WM8995_WRITE_SEQUENCER_412: c_uint = 0x319C;
pub const WM8995_WRITE_SEQUENCER_413: c_uint = 0x319D;
pub const WM8995_WRITE_SEQUENCER_414: c_uint = 0x319E;
pub const WM8995_WRITE_SEQUENCER_415: c_uint = 0x319F;
pub const WM8995_WRITE_SEQUENCER_416: c_uint = 0x31A0;
pub const WM8995_WRITE_SEQUENCER_417: c_uint = 0x31A1;
pub const WM8995_WRITE_SEQUENCER_418: c_uint = 0x31A2;
pub const WM8995_WRITE_SEQUENCER_419: c_uint = 0x31A3;
pub const WM8995_WRITE_SEQUENCER_420: c_uint = 0x31A4;
pub const WM8995_WRITE_SEQUENCER_421: c_uint = 0x31A5;
pub const WM8995_WRITE_SEQUENCER_422: c_uint = 0x31A6;
pub const WM8995_WRITE_SEQUENCER_423: c_uint = 0x31A7;
pub const WM8995_WRITE_SEQUENCER_424: c_uint = 0x31A8;
pub const WM8995_WRITE_SEQUENCER_425: c_uint = 0x31A9;
pub const WM8995_WRITE_SEQUENCER_426: c_uint = 0x31AA;
pub const WM8995_WRITE_SEQUENCER_427: c_uint = 0x31AB;
pub const WM8995_WRITE_SEQUENCER_428: c_uint = 0x31AC;
pub const WM8995_WRITE_SEQUENCER_429: c_uint = 0x31AD;
pub const WM8995_WRITE_SEQUENCER_430: c_uint = 0x31AE;
pub const WM8995_WRITE_SEQUENCER_431: c_uint = 0x31AF;
pub const WM8995_WRITE_SEQUENCER_432: c_uint = 0x31B0;
pub const WM8995_WRITE_SEQUENCER_433: c_uint = 0x31B1;
pub const WM8995_WRITE_SEQUENCER_434: c_uint = 0x31B2;
pub const WM8995_WRITE_SEQUENCER_435: c_uint = 0x31B3;
pub const WM8995_WRITE_SEQUENCER_436: c_uint = 0x31B4;
pub const WM8995_WRITE_SEQUENCER_437: c_uint = 0x31B5;
pub const WM8995_WRITE_SEQUENCER_438: c_uint = 0x31B6;
pub const WM8995_WRITE_SEQUENCER_439: c_uint = 0x31B7;
pub const WM8995_WRITE_SEQUENCER_440: c_uint = 0x31B8;
pub const WM8995_WRITE_SEQUENCER_441: c_uint = 0x31B9;
pub const WM8995_WRITE_SEQUENCER_442: c_uint = 0x31BA;
pub const WM8995_WRITE_SEQUENCER_443: c_uint = 0x31BB;
pub const WM8995_WRITE_SEQUENCER_444: c_uint = 0x31BC;
pub const WM8995_WRITE_SEQUENCER_445: c_uint = 0x31BD;
pub const WM8995_WRITE_SEQUENCER_446: c_uint = 0x31BE;
pub const WM8995_WRITE_SEQUENCER_447: c_uint = 0x31BF;
pub const WM8995_WRITE_SEQUENCER_448: c_uint = 0x31C0;
pub const WM8995_WRITE_SEQUENCER_449: c_uint = 0x31C1;
pub const WM8995_WRITE_SEQUENCER_450: c_uint = 0x31C2;
pub const WM8995_WRITE_SEQUENCER_451: c_uint = 0x31C3;
pub const WM8995_WRITE_SEQUENCER_452: c_uint = 0x31C4;
pub const WM8995_WRITE_SEQUENCER_453: c_uint = 0x31C5;
pub const WM8995_WRITE_SEQUENCER_454: c_uint = 0x31C6;
pub const WM8995_WRITE_SEQUENCER_455: c_uint = 0x31C7;
pub const WM8995_WRITE_SEQUENCER_456: c_uint = 0x31C8;
pub const WM8995_WRITE_SEQUENCER_457: c_uint = 0x31C9;
pub const WM8995_WRITE_SEQUENCER_458: c_uint = 0x31CA;
pub const WM8995_WRITE_SEQUENCER_459: c_uint = 0x31CB;
pub const WM8995_WRITE_SEQUENCER_460: c_uint = 0x31CC;
pub const WM8995_WRITE_SEQUENCER_461: c_uint = 0x31CD;
pub const WM8995_WRITE_SEQUENCER_462: c_uint = 0x31CE;
pub const WM8995_WRITE_SEQUENCER_463: c_uint = 0x31CF;
pub const WM8995_WRITE_SEQUENCER_464: c_uint = 0x31D0;
pub const WM8995_WRITE_SEQUENCER_465: c_uint = 0x31D1;
pub const WM8995_WRITE_SEQUENCER_466: c_uint = 0x31D2;
pub const WM8995_WRITE_SEQUENCER_467: c_uint = 0x31D3;
pub const WM8995_WRITE_SEQUENCER_468: c_uint = 0x31D4;
pub const WM8995_WRITE_SEQUENCER_469: c_uint = 0x31D5;
pub const WM8995_WRITE_SEQUENCER_470: c_uint = 0x31D6;
pub const WM8995_WRITE_SEQUENCER_471: c_uint = 0x31D7;
pub const WM8995_WRITE_SEQUENCER_472: c_uint = 0x31D8;
pub const WM8995_WRITE_SEQUENCER_473: c_uint = 0x31D9;
pub const WM8995_WRITE_SEQUENCER_474: c_uint = 0x31DA;
pub const WM8995_WRITE_SEQUENCER_475: c_uint = 0x31DB;
pub const WM8995_WRITE_SEQUENCER_476: c_uint = 0x31DC;
pub const WM8995_WRITE_SEQUENCER_477: c_uint = 0x31DD;
pub const WM8995_WRITE_SEQUENCER_478: c_uint = 0x31DE;
pub const WM8995_WRITE_SEQUENCER_479: c_uint = 0x31DF;
pub const WM8995_WRITE_SEQUENCER_480: c_uint = 0x31E0;
pub const WM8995_WRITE_SEQUENCER_481: c_uint = 0x31E1;
pub const WM8995_WRITE_SEQUENCER_482: c_uint = 0x31E2;
pub const WM8995_WRITE_SEQUENCER_483: c_uint = 0x31E3;
pub const WM8995_WRITE_SEQUENCER_484: c_uint = 0x31E4;
pub const WM8995_WRITE_SEQUENCER_485: c_uint = 0x31E5;
pub const WM8995_WRITE_SEQUENCER_486: c_uint = 0x31E6;
pub const WM8995_WRITE_SEQUENCER_487: c_uint = 0x31E7;
pub const WM8995_WRITE_SEQUENCER_488: c_uint = 0x31E8;
pub const WM8995_WRITE_SEQUENCER_489: c_uint = 0x31E9;
pub const WM8995_WRITE_SEQUENCER_490: c_uint = 0x31EA;
pub const WM8995_WRITE_SEQUENCER_491: c_uint = 0x31EB;
pub const WM8995_WRITE_SEQUENCER_492: c_uint = 0x31EC;
pub const WM8995_WRITE_SEQUENCER_493: c_uint = 0x31ED;
pub const WM8995_WRITE_SEQUENCER_494: c_uint = 0x31EE;
pub const WM8995_WRITE_SEQUENCER_495: c_uint = 0x31EF;
pub const WM8995_WRITE_SEQUENCER_496: c_uint = 0x31F0;
pub const WM8995_WRITE_SEQUENCER_497: c_uint = 0x31F1;
pub const WM8995_WRITE_SEQUENCER_498: c_uint = 0x31F2;
pub const WM8995_WRITE_SEQUENCER_499: c_uint = 0x31F3;
pub const WM8995_WRITE_SEQUENCER_500: c_uint = 0x31F4;
pub const WM8995_WRITE_SEQUENCER_501: c_uint = 0x31F5;
pub const WM8995_WRITE_SEQUENCER_502: c_uint = 0x31F6;
pub const WM8995_WRITE_SEQUENCER_503: c_uint = 0x31F7;
pub const WM8995_WRITE_SEQUENCER_504: c_uint = 0x31F8;
pub const WM8995_WRITE_SEQUENCER_505: c_uint = 0x31F9;
pub const WM8995_WRITE_SEQUENCER_506: c_uint = 0x31FA;
pub const WM8995_WRITE_SEQUENCER_507: c_uint = 0x31FB;
pub const WM8995_WRITE_SEQUENCER_508: c_uint = 0x31FC;
pub const WM8995_WRITE_SEQUENCER_509: c_uint = 0x31FD;
pub const WM8995_WRITE_SEQUENCER_510: c_uint = 0x31FE;
pub const WM8995_WRITE_SEQUENCER_511: c_uint = 0x31FF;
pub const WM8995_REGISTER_COUNT: c_int = 725;
pub const WM8995_MAX_REGISTER: c_uint = 0x31FF;

//
// Field Definitions.
//
// R0 (0x00) - Software Reset
//
pub const WM8995_SW_RESET_MASK: c_uint = 0xFFFF	/* SW_RESET - [15:0] */;

//
// R1 (0x01) - Power Management (1)
//
pub const WM8995_MICB2_ENA: c_uint = 0x0200	/* MICB2_ENA */;
pub const WM8995_MICB2_ENA_MASK: c_uint = 0x0200	/* MICB2_ENA */;

pub const WM8995_MICB1_ENA: c_uint = 0x0100	/* MICB1_ENA */;
pub const WM8995_MICB1_ENA_MASK: c_uint = 0x0100	/* MICB1_ENA */;

pub const WM8995_HPOUT2L_ENA: c_uint = 0x0080	/* HPOUT2L_ENA */;
pub const WM8995_HPOUT2L_ENA_MASK: c_uint = 0x0080	/* HPOUT2L_ENA */;

pub const WM8995_HPOUT2R_ENA: c_uint = 0x0040	/* HPOUT2R_ENA */;
pub const WM8995_HPOUT2R_ENA_MASK: c_uint = 0x0040	/* HPOUT2R_ENA */;

pub const WM8995_HPOUT1L_ENA: c_uint = 0x0020	/* HPOUT1L_ENA */;
pub const WM8995_HPOUT1L_ENA_MASK: c_uint = 0x0020	/* HPOUT1L_ENA */;

pub const WM8995_HPOUT1R_ENA: c_uint = 0x0010	/* HPOUT1R_ENA */;
pub const WM8995_HPOUT1R_ENA_MASK: c_uint = 0x0010	/* HPOUT1R_ENA */;

pub const WM8995_BG_ENA: c_uint = 0x0001	/* BG_ENA */;
pub const WM8995_BG_ENA_MASK: c_uint = 0x0001	/* BG_ENA */;

//
// R2 (0x02) - Power Management (2)
//
pub const WM8995_OPCLK_ENA: c_uint = 0x0800	/* OPCLK_ENA */;
pub const WM8995_OPCLK_ENA_MASK: c_uint = 0x0800	/* OPCLK_ENA */;

pub const WM8995_IN1L_ENA: c_uint = 0x0020	/* IN1L_ENA */;
pub const WM8995_IN1L_ENA_MASK: c_uint = 0x0020	/* IN1L_ENA */;

pub const WM8995_IN1R_ENA: c_uint = 0x0010	/* IN1R_ENA */;
pub const WM8995_IN1R_ENA_MASK: c_uint = 0x0010	/* IN1R_ENA */;

pub const WM8995_LDO2_ENA: c_uint = 0x0002	/* LDO2_ENA */;
pub const WM8995_LDO2_ENA_MASK: c_uint = 0x0002	/* LDO2_ENA */;

//
// R3 (0x03) - Power Management (3)
//
pub const WM8995_AIF2ADCL_ENA: c_uint = 0x2000	/* AIF2ADCL_ENA */;
pub const WM8995_AIF2ADCL_ENA_MASK: c_uint = 0x2000	/* AIF2ADCL_ENA */;

pub const WM8995_AIF2ADCR_ENA: c_uint = 0x1000	/* AIF2ADCR_ENA */;
pub const WM8995_AIF2ADCR_ENA_MASK: c_uint = 0x1000	/* AIF2ADCR_ENA */;

pub const WM8995_AIF1ADC2L_ENA: c_uint = 0x0800	/* AIF1ADC2L_ENA */;
pub const WM8995_AIF1ADC2L_ENA_MASK: c_uint = 0x0800	/* AIF1ADC2L_ENA */;

pub const WM8995_AIF1ADC2R_ENA: c_uint = 0x0400	/* AIF1ADC2R_ENA */;
pub const WM8995_AIF1ADC2R_ENA_MASK: c_uint = 0x0400	/* AIF1ADC2R_ENA */;

pub const WM8995_AIF1ADC1L_ENA: c_uint = 0x0200	/* AIF1ADC1L_ENA */;
pub const WM8995_AIF1ADC1L_ENA_MASK: c_uint = 0x0200	/* AIF1ADC1L_ENA */;

pub const WM8995_AIF1ADC1R_ENA: c_uint = 0x0100	/* AIF1ADC1R_ENA */;
pub const WM8995_AIF1ADC1R_ENA_MASK: c_uint = 0x0100	/* AIF1ADC1R_ENA */;

pub const WM8995_DMIC3L_ENA: c_uint = 0x0080	/* DMIC3L_ENA */;
pub const WM8995_DMIC3L_ENA_MASK: c_uint = 0x0080	/* DMIC3L_ENA */;

pub const WM8995_DMIC3R_ENA: c_uint = 0x0040	/* DMIC3R_ENA */;
pub const WM8995_DMIC3R_ENA_MASK: c_uint = 0x0040	/* DMIC3R_ENA */;

pub const WM8995_DMIC2L_ENA: c_uint = 0x0020	/* DMIC2L_ENA */;
pub const WM8995_DMIC2L_ENA_MASK: c_uint = 0x0020	/* DMIC2L_ENA */;

pub const WM8995_DMIC2R_ENA: c_uint = 0x0010	/* DMIC2R_ENA */;
pub const WM8995_DMIC2R_ENA_MASK: c_uint = 0x0010	/* DMIC2R_ENA */;

pub const WM8995_DMIC1L_ENA: c_uint = 0x0008	/* DMIC1L_ENA */;
pub const WM8995_DMIC1L_ENA_MASK: c_uint = 0x0008	/* DMIC1L_ENA */;

pub const WM8995_DMIC1R_ENA: c_uint = 0x0004	/* DMIC1R_ENA */;
pub const WM8995_DMIC1R_ENA_MASK: c_uint = 0x0004	/* DMIC1R_ENA */;

pub const WM8995_ADCL_ENA: c_uint = 0x0002	/* ADCL_ENA */;
pub const WM8995_ADCL_ENA_MASK: c_uint = 0x0002	/* ADCL_ENA */;

pub const WM8995_ADCR_ENA: c_uint = 0x0001	/* ADCR_ENA */;
pub const WM8995_ADCR_ENA_MASK: c_uint = 0x0001	/* ADCR_ENA */;

//
// R4 (0x04) - Power Management (4)
//
pub const WM8995_AIF2DACL_ENA: c_uint = 0x2000	/* AIF2DACL_ENA */;
pub const WM8995_AIF2DACL_ENA_MASK: c_uint = 0x2000	/* AIF2DACL_ENA */;

pub const WM8995_AIF2DACR_ENA: c_uint = 0x1000	/* AIF2DACR_ENA */;
pub const WM8995_AIF2DACR_ENA_MASK: c_uint = 0x1000	/* AIF2DACR_ENA */;

pub const WM8995_AIF1DAC2L_ENA: c_uint = 0x0800	/* AIF1DAC2L_ENA */;
pub const WM8995_AIF1DAC2L_ENA_MASK: c_uint = 0x0800	/* AIF1DAC2L_ENA */;

pub const WM8995_AIF1DAC2R_ENA: c_uint = 0x0400	/* AIF1DAC2R_ENA */;
pub const WM8995_AIF1DAC2R_ENA_MASK: c_uint = 0x0400	/* AIF1DAC2R_ENA */;

pub const WM8995_AIF1DAC1L_ENA: c_uint = 0x0200	/* AIF1DAC1L_ENA */;
pub const WM8995_AIF1DAC1L_ENA_MASK: c_uint = 0x0200	/* AIF1DAC1L_ENA */;

pub const WM8995_AIF1DAC1R_ENA: c_uint = 0x0100	/* AIF1DAC1R_ENA */;
pub const WM8995_AIF1DAC1R_ENA_MASK: c_uint = 0x0100	/* AIF1DAC1R_ENA */;

pub const WM8995_DAC2L_ENA: c_uint = 0x0008	/* DAC2L_ENA */;
pub const WM8995_DAC2L_ENA_MASK: c_uint = 0x0008	/* DAC2L_ENA */;

pub const WM8995_DAC2R_ENA: c_uint = 0x0004	/* DAC2R_ENA */;
pub const WM8995_DAC2R_ENA_MASK: c_uint = 0x0004	/* DAC2R_ENA */;

pub const WM8995_DAC1L_ENA: c_uint = 0x0002	/* DAC1L_ENA */;
pub const WM8995_DAC1L_ENA_MASK: c_uint = 0x0002	/* DAC1L_ENA */;

pub const WM8995_DAC1R_ENA: c_uint = 0x0001	/* DAC1R_ENA */;
pub const WM8995_DAC1R_ENA_MASK: c_uint = 0x0001	/* DAC1R_ENA */;

//
// R5 (0x05) - Power Management (5)
//
pub const WM8995_DMIC_SRC2_MASK: c_uint = 0x0300	/* DMIC_SRC2 - [9:8] */;

pub const WM8995_DMIC_SRC1_MASK: c_uint = 0x00C0	/* DMIC_SRC1 - [7:6] */;

pub const WM8995_AIF3_TRI: c_uint = 0x0020	/* AIF3_TRI */;
pub const WM8995_AIF3_TRI_MASK: c_uint = 0x0020	/* AIF3_TRI */;

pub const WM8995_AIF3_ADCDAT_SRC_MASK: c_uint = 0x0018	/* AIF3_ADCDAT_SRC - [4:3] */;

pub const WM8995_AIF2_ADCDAT_SRC: c_uint = 0x0004	/* AIF2_ADCDAT_SRC */;
pub const WM8995_AIF2_ADCDAT_SRC_MASK: c_uint = 0x0004	/* AIF2_ADCDAT_SRC */;

pub const WM8995_AIF2_DACDAT_SRC: c_uint = 0x0002	/* AIF2_DACDAT_SRC */;
pub const WM8995_AIF2_DACDAT_SRC_MASK: c_uint = 0x0002	/* AIF2_DACDAT_SRC */;

pub const WM8995_AIF1_DACDAT_SRC: c_uint = 0x0001	/* AIF1_DACDAT_SRC */;
pub const WM8995_AIF1_DACDAT_SRC_MASK: c_uint = 0x0001	/* AIF1_DACDAT_SRC */;

//
// R16 (0x10) - Left Line Input 1 Volume
//
pub const WM8995_IN1_VU: c_uint = 0x0080	/* IN1_VU */;
pub const WM8995_IN1_VU_MASK: c_uint = 0x0080	/* IN1_VU */;

pub const WM8995_IN1L_ZC: c_uint = 0x0020	/* IN1L_ZC */;
pub const WM8995_IN1L_ZC_MASK: c_uint = 0x0020	/* IN1L_ZC */;

pub const WM8995_IN1L_VOL_MASK: c_uint = 0x001F	/* IN1L_VOL - [4:0] */;

//
// R17 (0x11) - Right Line Input 1 Volume
//
pub const WM8995_IN1_VU: c_uint = 0x0080	/* IN1_VU */;
pub const WM8995_IN1_VU_MASK: c_uint = 0x0080	/* IN1_VU */;

pub const WM8995_IN1R_ZC: c_uint = 0x0020	/* IN1R_ZC */;
pub const WM8995_IN1R_ZC_MASK: c_uint = 0x0020	/* IN1R_ZC */;

pub const WM8995_IN1R_VOL_MASK: c_uint = 0x001F	/* IN1R_VOL - [4:0] */;

//
// R18 (0x12) - Left Line Input Control
//
pub const WM8995_IN1L_BOOST_MASK: c_uint = 0x0030	/* IN1L_BOOST - [5:4] */;

pub const WM8995_IN1L_MODE_MASK: c_uint = 0x000C	/* IN1L_MODE - [3:2] */;

pub const WM8995_IN1R_MODE_MASK: c_uint = 0x0003	/* IN1R_MODE - [1:0] */;

//
// R24 (0x18) - DAC1 Left Volume
//
pub const WM8995_DAC1L_MUTE: c_uint = 0x0200	/* DAC1L_MUTE */;
pub const WM8995_DAC1L_MUTE_MASK: c_uint = 0x0200	/* DAC1L_MUTE */;

pub const WM8995_DAC1_VU: c_uint = 0x0100	/* DAC1_VU */;
pub const WM8995_DAC1_VU_MASK: c_uint = 0x0100	/* DAC1_VU */;

pub const WM8995_DAC1L_VOL_MASK: c_uint = 0x00FF	/* DAC1L_VOL - [7:0] */;

//
// R25 (0x19) - DAC1 Right Volume
//
pub const WM8995_DAC1R_MUTE: c_uint = 0x0200	/* DAC1R_MUTE */;
pub const WM8995_DAC1R_MUTE_MASK: c_uint = 0x0200	/* DAC1R_MUTE */;

pub const WM8995_DAC1_VU: c_uint = 0x0100	/* DAC1_VU */;
pub const WM8995_DAC1_VU_MASK: c_uint = 0x0100	/* DAC1_VU */;

pub const WM8995_DAC1R_VOL_MASK: c_uint = 0x00FF	/* DAC1R_VOL - [7:0] */;

//
// R26 (0x1A) - DAC2 Left Volume
//
pub const WM8995_DAC2L_MUTE: c_uint = 0x0200	/* DAC2L_MUTE */;
pub const WM8995_DAC2L_MUTE_MASK: c_uint = 0x0200	/* DAC2L_MUTE */;

pub const WM8995_DAC2_VU: c_uint = 0x0100	/* DAC2_VU */;
pub const WM8995_DAC2_VU_MASK: c_uint = 0x0100	/* DAC2_VU */;

pub const WM8995_DAC2L_VOL_MASK: c_uint = 0x00FF	/* DAC2L_VOL - [7:0] */;

//
// R27 (0x1B) - DAC2 Right Volume
//
pub const WM8995_DAC2R_MUTE: c_uint = 0x0200	/* DAC2R_MUTE */;
pub const WM8995_DAC2R_MUTE_MASK: c_uint = 0x0200	/* DAC2R_MUTE */;

pub const WM8995_DAC2_VU: c_uint = 0x0100	/* DAC2_VU */;
pub const WM8995_DAC2_VU_MASK: c_uint = 0x0100	/* DAC2_VU */;

pub const WM8995_DAC2R_VOL_MASK: c_uint = 0x00FF	/* DAC2R_VOL - [7:0] */;

//
// R28 (0x1C) - Output Volume ZC (1)
//
pub const WM8995_HPOUT2L_ZC: c_uint = 0x0008	/* HPOUT2L_ZC */;
pub const WM8995_HPOUT2L_ZC_MASK: c_uint = 0x0008	/* HPOUT2L_ZC */;

pub const WM8995_HPOUT2R_ZC: c_uint = 0x0004	/* HPOUT2R_ZC */;
pub const WM8995_HPOUT2R_ZC_MASK: c_uint = 0x0004	/* HPOUT2R_ZC */;

pub const WM8995_HPOUT1L_ZC: c_uint = 0x0002	/* HPOUT1L_ZC */;
pub const WM8995_HPOUT1L_ZC_MASK: c_uint = 0x0002	/* HPOUT1L_ZC */;

pub const WM8995_HPOUT1R_ZC: c_uint = 0x0001	/* HPOUT1R_ZC */;
pub const WM8995_HPOUT1R_ZC_MASK: c_uint = 0x0001	/* HPOUT1R_ZC */;

//
// R32 (0x20) - MICBIAS (1)
//
pub const WM8995_MICB1_MODE: c_uint = 0x0008	/* MICB1_MODE */;
pub const WM8995_MICB1_MODE_MASK: c_uint = 0x0008	/* MICB1_MODE */;

pub const WM8995_MICB1_LVL_MASK: c_uint = 0x0006	/* MICB1_LVL - [2:1] */;

pub const WM8995_MICB1_DISCH: c_uint = 0x0001	/* MICB1_DISCH */;
pub const WM8995_MICB1_DISCH_MASK: c_uint = 0x0001	/* MICB1_DISCH */;

//
// R33 (0x21) - MICBIAS (2)
//
pub const WM8995_MICB2_MODE: c_uint = 0x0008	/* MICB2_MODE */;
pub const WM8995_MICB2_MODE_MASK: c_uint = 0x0008	/* MICB2_MODE */;

pub const WM8995_MICB2_LVL_MASK: c_uint = 0x0006	/* MICB2_LVL - [2:1] */;

pub const WM8995_MICB2_DISCH: c_uint = 0x0001	/* MICB2_DISCH */;
pub const WM8995_MICB2_DISCH_MASK: c_uint = 0x0001	/* MICB2_DISCH */;

//
// R40 (0x28) - LDO 1
//
pub const WM8995_LDO1_MODE: c_uint = 0x0020	/* LDO1_MODE */;
pub const WM8995_LDO1_MODE_MASK: c_uint = 0x0020	/* LDO1_MODE */;

pub const WM8995_LDO1_VSEL_MASK: c_uint = 0x0006	/* LDO1_VSEL - [2:1] */;

pub const WM8995_LDO1_DISCH: c_uint = 0x0001	/* LDO1_DISCH */;
pub const WM8995_LDO1_DISCH_MASK: c_uint = 0x0001	/* LDO1_DISCH */;

//
// R41 (0x29) - LDO 2
//
pub const WM8995_LDO2_MODE: c_uint = 0x0020	/* LDO2_MODE */;
pub const WM8995_LDO2_MODE_MASK: c_uint = 0x0020	/* LDO2_MODE */;

pub const WM8995_LDO2_VSEL_MASK: c_uint = 0x001E	/* LDO2_VSEL - [4:1] */;

pub const WM8995_LDO2_DISCH: c_uint = 0x0001	/* LDO2_DISCH */;
pub const WM8995_LDO2_DISCH_MASK: c_uint = 0x0001	/* LDO2_DISCH */;

//
// R48 (0x30) - Accessory Detect Mode1
//
pub const WM8995_JD_MODE_MASK: c_uint = 0x0003	/* JD_MODE - [1:0] */;

//
// R49 (0x31) - Accessory Detect Mode2
//
pub const WM8995_VID_ENA: c_uint = 0x0001	/* VID_ENA */;
pub const WM8995_VID_ENA_MASK: c_uint = 0x0001	/* VID_ENA */;

//
// R52 (0x34) - Headphone Detect1
//
pub const WM8995_HP_RAMPRATE: c_uint = 0x0002	/* HP_RAMPRATE */;
pub const WM8995_HP_RAMPRATE_MASK: c_uint = 0x0002	/* HP_RAMPRATE */;

pub const WM8995_HP_POLL: c_uint = 0x0001	/* HP_POLL */;
pub const WM8995_HP_POLL_MASK: c_uint = 0x0001	/* HP_POLL */;

//
// R53 (0x35) - Headphone Detect2
//
pub const WM8995_HP_DONE: c_uint = 0x0080	/* HP_DONE */;
pub const WM8995_HP_DONE_MASK: c_uint = 0x0080	/* HP_DONE */;

pub const WM8995_HP_LVL_MASK: c_uint = 0x007F	/* HP_LVL - [6:0] */;

//
// R56 (0x38) - Mic Detect (1)
//
pub const WM8995_MICD_RATE_MASK: c_uint = 0x7800	/* MICD_RATE - [14:11] */;

pub const WM8995_MICD_LVL_SEL_MASK: c_uint = 0x01F8	/* MICD_LVL_SEL - [8:3] */;

pub const WM8995_MICD_DBTIME: c_uint = 0x0002	/* MICD_DBTIME */;
pub const WM8995_MICD_DBTIME_MASK: c_uint = 0x0002	/* MICD_DBTIME */;

pub const WM8995_MICD_ENA: c_uint = 0x0001	/* MICD_ENA */;
pub const WM8995_MICD_ENA_MASK: c_uint = 0x0001	/* MICD_ENA */;

//
// R57 (0x39) - Mic Detect (2)
//
pub const WM8995_MICD_LVL_MASK: c_uint = 0x01FC	/* MICD_LVL - [8:2] */;

pub const WM8995_MICD_VALID: c_uint = 0x0002	/* MICD_VALID */;
pub const WM8995_MICD_VALID_MASK: c_uint = 0x0002	/* MICD_VALID */;

pub const WM8995_MICD_STS: c_uint = 0x0001	/* MICD_STS */;
pub const WM8995_MICD_STS_MASK: c_uint = 0x0001	/* MICD_STS */;

//
// R64 (0x40) - Charge Pump (1)
//
pub const WM8995_CP_ENA: c_uint = 0x8000	/* CP_ENA */;
pub const WM8995_CP_ENA_MASK: c_uint = 0x8000	/* CP_ENA */;

//
// R69 (0x45) - Class W (1)
//
pub const WM8995_CP_DYN_SRC_SEL_MASK: c_uint = 0x0300	/* CP_DYN_SRC_SEL - [9:8] */;

pub const WM8995_CP_DYN_PWR: c_uint = 0x0001	/* CP_DYN_PWR */;
pub const WM8995_CP_DYN_PWR_MASK: c_uint = 0x0001	/* CP_DYN_PWR */;

//
// R80 (0x50) - DC Servo (1)
//
pub const WM8995_DCS_ENA_CHAN_3: c_uint = 0x0008	/* DCS_ENA_CHAN_3 */;
pub const WM8995_DCS_ENA_CHAN_3_MASK: c_uint = 0x0008	/* DCS_ENA_CHAN_3 */;

pub const WM8995_DCS_ENA_CHAN_2: c_uint = 0x0004	/* DCS_ENA_CHAN_2 */;
pub const WM8995_DCS_ENA_CHAN_2_MASK: c_uint = 0x0004	/* DCS_ENA_CHAN_2 */;

pub const WM8995_DCS_ENA_CHAN_1: c_uint = 0x0002	/* DCS_ENA_CHAN_1 */;
pub const WM8995_DCS_ENA_CHAN_1_MASK: c_uint = 0x0002	/* DCS_ENA_CHAN_1 */;

pub const WM8995_DCS_ENA_CHAN_0: c_uint = 0x0001	/* DCS_ENA_CHAN_0 */;
pub const WM8995_DCS_ENA_CHAN_0_MASK: c_uint = 0x0001	/* DCS_ENA_CHAN_0 */;

//
// R81 (0x51) - DC Servo (2)
//
pub const WM8995_DCS_TRIG_SINGLE_3: c_uint = 0x8000	/* DCS_TRIG_SINGLE_3 */;
pub const WM8995_DCS_TRIG_SINGLE_3_MASK: c_uint = 0x8000	/* DCS_TRIG_SINGLE_3 */;

pub const WM8995_DCS_TRIG_SINGLE_2: c_uint = 0x4000	/* DCS_TRIG_SINGLE_2 */;
pub const WM8995_DCS_TRIG_SINGLE_2_MASK: c_uint = 0x4000	/* DCS_TRIG_SINGLE_2 */;

pub const WM8995_DCS_TRIG_SINGLE_1: c_uint = 0x2000	/* DCS_TRIG_SINGLE_1 */;
pub const WM8995_DCS_TRIG_SINGLE_1_MASK: c_uint = 0x2000	/* DCS_TRIG_SINGLE_1 */;

pub const WM8995_DCS_TRIG_SINGLE_0: c_uint = 0x1000	/* DCS_TRIG_SINGLE_0 */;
pub const WM8995_DCS_TRIG_SINGLE_0_MASK: c_uint = 0x1000	/* DCS_TRIG_SINGLE_0 */;

pub const WM8995_DCS_TRIG_SERIES_3: c_uint = 0x0800	/* DCS_TRIG_SERIES_3 */;
pub const WM8995_DCS_TRIG_SERIES_3_MASK: c_uint = 0x0800	/* DCS_TRIG_SERIES_3 */;

pub const WM8995_DCS_TRIG_SERIES_2: c_uint = 0x0400	/* DCS_TRIG_SERIES_2 */;
pub const WM8995_DCS_TRIG_SERIES_2_MASK: c_uint = 0x0400	/* DCS_TRIG_SERIES_2 */;

pub const WM8995_DCS_TRIG_SERIES_1: c_uint = 0x0200	/* DCS_TRIG_SERIES_1 */;
pub const WM8995_DCS_TRIG_SERIES_1_MASK: c_uint = 0x0200	/* DCS_TRIG_SERIES_1 */;

pub const WM8995_DCS_TRIG_SERIES_0: c_uint = 0x0100	/* DCS_TRIG_SERIES_0 */;
pub const WM8995_DCS_TRIG_SERIES_0_MASK: c_uint = 0x0100	/* DCS_TRIG_SERIES_0 */;

pub const WM8995_DCS_TRIG_STARTUP_3: c_uint = 0x0080	/* DCS_TRIG_STARTUP_3 */;
pub const WM8995_DCS_TRIG_STARTUP_3_MASK: c_uint = 0x0080	/* DCS_TRIG_STARTUP_3 */;

pub const WM8995_DCS_TRIG_STARTUP_2: c_uint = 0x0040	/* DCS_TRIG_STARTUP_2 */;
pub const WM8995_DCS_TRIG_STARTUP_2_MASK: c_uint = 0x0040	/* DCS_TRIG_STARTUP_2 */;

pub const WM8995_DCS_TRIG_STARTUP_1: c_uint = 0x0020	/* DCS_TRIG_STARTUP_1 */;
pub const WM8995_DCS_TRIG_STARTUP_1_MASK: c_uint = 0x0020	/* DCS_TRIG_STARTUP_1 */;

pub const WM8995_DCS_TRIG_STARTUP_0: c_uint = 0x0010	/* DCS_TRIG_STARTUP_0 */;
pub const WM8995_DCS_TRIG_STARTUP_0_MASK: c_uint = 0x0010	/* DCS_TRIG_STARTUP_0 */;

pub const WM8995_DCS_TRIG_DAC_WR_3: c_uint = 0x0008	/* DCS_TRIG_DAC_WR_3 */;
pub const WM8995_DCS_TRIG_DAC_WR_3_MASK: c_uint = 0x0008	/* DCS_TRIG_DAC_WR_3 */;

pub const WM8995_DCS_TRIG_DAC_WR_2: c_uint = 0x0004	/* DCS_TRIG_DAC_WR_2 */;
pub const WM8995_DCS_TRIG_DAC_WR_2_MASK: c_uint = 0x0004	/* DCS_TRIG_DAC_WR_2 */;

pub const WM8995_DCS_TRIG_DAC_WR_1: c_uint = 0x0002	/* DCS_TRIG_DAC_WR_1 */;
pub const WM8995_DCS_TRIG_DAC_WR_1_MASK: c_uint = 0x0002	/* DCS_TRIG_DAC_WR_1 */;

pub const WM8995_DCS_TRIG_DAC_WR_0: c_uint = 0x0001	/* DCS_TRIG_DAC_WR_0 */;
pub const WM8995_DCS_TRIG_DAC_WR_0_MASK: c_uint = 0x0001	/* DCS_TRIG_DAC_WR_0 */;

//
// R82 (0x52) - DC Servo (3)
//
pub const WM8995_DCS_TIMER_PERIOD_23_MASK: c_uint = 0x0F00	/* DCS_TIMER_PERIOD_23 - [11:8] */;

pub const WM8995_DCS_TIMER_PERIOD_01_MASK: c_uint = 0x000F	/* DCS_TIMER_PERIOD_01 - [3:0] */;

//
// R84 (0x54) - DC Servo (5)
//
pub const WM8995_DCS_SERIES_NO_23_MASK: c_uint = 0x7F00	/* DCS_SERIES_NO_23 - [14:8] */;

pub const WM8995_DCS_SERIES_NO_01_MASK: c_uint = 0x007F	/* DCS_SERIES_NO_01 - [6:0] */;

//
// R85 (0x55) - DC Servo (6)
//
pub const WM8995_DCS_DAC_WR_VAL_3_MASK: c_uint = 0xFF00	/* DCS_DAC_WR_VAL_3 - [15:8] */;

pub const WM8995_DCS_DAC_WR_VAL_2_MASK: c_uint = 0x00FF	/* DCS_DAC_WR_VAL_2 - [7:0] */;

//
// R86 (0x56) - DC Servo (7)
//
pub const WM8995_DCS_DAC_WR_VAL_1_MASK: c_uint = 0xFF00	/* DCS_DAC_WR_VAL_1 - [15:8] */;

pub const WM8995_DCS_DAC_WR_VAL_0_MASK: c_uint = 0x00FF	/* DCS_DAC_WR_VAL_0 - [7:0] */;

//
// R87 (0x57) - DC Servo Readback 0
//
pub const WM8995_DCS_CAL_COMPLETE_MASK: c_uint = 0x0F00	/* DCS_CAL_COMPLETE - [11:8] */;

pub const WM8995_DCS_DAC_WR_COMPLETE_MASK: c_uint = 0x00F0	/* DCS_DAC_WR_COMPLETE - [7:4] */;

pub const WM8995_DCS_STARTUP_COMPLETE_MASK: c_uint = 0x000F	/* DCS_STARTUP_COMPLETE - [3:0] */;

//
// R96 (0x60) - Analogue HP (1)
//
pub const WM8995_HPOUT1L_RMV_SHORT: c_uint = 0x0080	/* HPOUT1L_RMV_SHORT */;
pub const WM8995_HPOUT1L_RMV_SHORT_MASK: c_uint = 0x0080	/* HPOUT1L_RMV_SHORT */;

pub const WM8995_HPOUT1L_OUTP: c_uint = 0x0040	/* HPOUT1L_OUTP */;
pub const WM8995_HPOUT1L_OUTP_MASK: c_uint = 0x0040	/* HPOUT1L_OUTP */;

pub const WM8995_HPOUT1L_DLY: c_uint = 0x0020	/* HPOUT1L_DLY */;
pub const WM8995_HPOUT1L_DLY_MASK: c_uint = 0x0020	/* HPOUT1L_DLY */;

pub const WM8995_HPOUT1R_RMV_SHORT: c_uint = 0x0008	/* HPOUT1R_RMV_SHORT */;
pub const WM8995_HPOUT1R_RMV_SHORT_MASK: c_uint = 0x0008	/* HPOUT1R_RMV_SHORT */;

pub const WM8995_HPOUT1R_OUTP: c_uint = 0x0004	/* HPOUT1R_OUTP */;
pub const WM8995_HPOUT1R_OUTP_MASK: c_uint = 0x0004	/* HPOUT1R_OUTP */;

pub const WM8995_HPOUT1R_DLY: c_uint = 0x0002	/* HPOUT1R_DLY */;
pub const WM8995_HPOUT1R_DLY_MASK: c_uint = 0x0002	/* HPOUT1R_DLY */;

//
// R97 (0x61) - Analogue HP (2)
//
pub const WM8995_HPOUT2L_RMV_SHORT: c_uint = 0x0080	/* HPOUT2L_RMV_SHORT */;
pub const WM8995_HPOUT2L_RMV_SHORT_MASK: c_uint = 0x0080	/* HPOUT2L_RMV_SHORT */;

pub const WM8995_HPOUT2L_OUTP: c_uint = 0x0040	/* HPOUT2L_OUTP */;
pub const WM8995_HPOUT2L_OUTP_MASK: c_uint = 0x0040	/* HPOUT2L_OUTP */;

pub const WM8995_HPOUT2L_DLY: c_uint = 0x0020	/* HPOUT2L_DLY */;
pub const WM8995_HPOUT2L_DLY_MASK: c_uint = 0x0020	/* HPOUT2L_DLY */;

pub const WM8995_HPOUT2R_RMV_SHORT: c_uint = 0x0008	/* HPOUT2R_RMV_SHORT */;
pub const WM8995_HPOUT2R_RMV_SHORT_MASK: c_uint = 0x0008	/* HPOUT2R_RMV_SHORT */;

pub const WM8995_HPOUT2R_OUTP: c_uint = 0x0004	/* HPOUT2R_OUTP */;
pub const WM8995_HPOUT2R_OUTP_MASK: c_uint = 0x0004	/* HPOUT2R_OUTP */;

pub const WM8995_HPOUT2R_DLY: c_uint = 0x0002	/* HPOUT2R_DLY */;
pub const WM8995_HPOUT2R_DLY_MASK: c_uint = 0x0002	/* HPOUT2R_DLY */;

//
// R256 (0x100) - Chip Revision
//
pub const WM8995_CHIP_REV_MASK: c_uint = 0x000F	/* CHIP_REV - [3:0] */;

//
// R257 (0x101) - Control Interface (1)
//
pub const WM8995_REG_SYNC: c_uint = 0x8000	/* REG_SYNC */;
pub const WM8995_REG_SYNC_MASK: c_uint = 0x8000	/* REG_SYNC */;

pub const WM8995_SPI_CONTRD: c_uint = 0x0040	/* SPI_CONTRD */;
pub const WM8995_SPI_CONTRD_MASK: c_uint = 0x0040	/* SPI_CONTRD */;

pub const WM8995_SPI_4WIRE: c_uint = 0x0020	/* SPI_4WIRE */;
pub const WM8995_SPI_4WIRE_MASK: c_uint = 0x0020	/* SPI_4WIRE */;

pub const WM8995_SPI_CFG: c_uint = 0x0010	/* SPI_CFG */;
pub const WM8995_SPI_CFG_MASK: c_uint = 0x0010	/* SPI_CFG */;

pub const WM8995_AUTO_INC: c_uint = 0x0004	/* AUTO_INC */;
pub const WM8995_AUTO_INC_MASK: c_uint = 0x0004	/* AUTO_INC */;

//
// R258 (0x102) - Control Interface (2)
//
pub const WM8995_CTRL_IF_SRC: c_uint = 0x0001	/* CTRL_IF_SRC */;
pub const WM8995_CTRL_IF_SRC_MASK: c_uint = 0x0001	/* CTRL_IF_SRC */;

//
// R272 (0x110) - Write Sequencer Ctrl (1)
//
pub const WM8995_WSEQ_ENA: c_uint = 0x8000	/* WSEQ_ENA */;
pub const WM8995_WSEQ_ENA_MASK: c_uint = 0x8000	/* WSEQ_ENA */;

pub const WM8995_WSEQ_ABORT: c_uint = 0x0200	/* WSEQ_ABORT */;
pub const WM8995_WSEQ_ABORT_MASK: c_uint = 0x0200	/* WSEQ_ABORT */;

pub const WM8995_WSEQ_START: c_uint = 0x0100	/* WSEQ_START */;
pub const WM8995_WSEQ_START_MASK: c_uint = 0x0100	/* WSEQ_START */;

pub const WM8995_WSEQ_START_INDEX_MASK: c_uint = 0x007F	/* WSEQ_START_INDEX - [6:0] */;

//
// R273 (0x111) - Write Sequencer Ctrl (2)
//
pub const WM8995_WSEQ_BUSY: c_uint = 0x0100	/* WSEQ_BUSY */;
pub const WM8995_WSEQ_BUSY_MASK: c_uint = 0x0100	/* WSEQ_BUSY */;

pub const WM8995_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x007F	/* WSEQ_CURRENT_INDEX - [6:0] */;

//
// R512 (0x200) - AIF1 Clocking (1)
//
pub const WM8995_AIF1CLK_SRC_MASK: c_uint = 0x0018	/* AIF1CLK_SRC - [4:3] */;

pub const WM8995_AIF1CLK_INV: c_uint = 0x0004	/* AIF1CLK_INV */;
pub const WM8995_AIF1CLK_INV_MASK: c_uint = 0x0004	/* AIF1CLK_INV */;

pub const WM8995_AIF1CLK_DIV: c_uint = 0x0002	/* AIF1CLK_DIV */;
pub const WM8995_AIF1CLK_DIV_MASK: c_uint = 0x0002	/* AIF1CLK_DIV */;

pub const WM8995_AIF1CLK_ENA: c_uint = 0x0001	/* AIF1CLK_ENA */;
pub const WM8995_AIF1CLK_ENA_MASK: c_uint = 0x0001	/* AIF1CLK_ENA */;

//
// R513 (0x201) - AIF1 Clocking (2)
//
pub const WM8995_AIF1DAC_DIV_MASK: c_uint = 0x0038	/* AIF1DAC_DIV - [5:3] */;

pub const WM8995_AIF1ADC_DIV_MASK: c_uint = 0x0007	/* AIF1ADC_DIV - [2:0] */;

//
// R516 (0x204) - AIF2 Clocking (1)
//
pub const WM8995_AIF2CLK_SRC_MASK: c_uint = 0x0018	/* AIF2CLK_SRC - [4:3] */;

pub const WM8995_AIF2CLK_INV: c_uint = 0x0004	/* AIF2CLK_INV */;
pub const WM8995_AIF2CLK_INV_MASK: c_uint = 0x0004	/* AIF2CLK_INV */;

pub const WM8995_AIF2CLK_DIV: c_uint = 0x0002	/* AIF2CLK_DIV */;
pub const WM8995_AIF2CLK_DIV_MASK: c_uint = 0x0002	/* AIF2CLK_DIV */;

pub const WM8995_AIF2CLK_ENA: c_uint = 0x0001	/* AIF2CLK_ENA */;
pub const WM8995_AIF2CLK_ENA_MASK: c_uint = 0x0001	/* AIF2CLK_ENA */;

//
// R517 (0x205) - AIF2 Clocking (2)
//
pub const WM8995_AIF2DAC_DIV_MASK: c_uint = 0x0038	/* AIF2DAC_DIV - [5:3] */;

pub const WM8995_AIF2ADC_DIV_MASK: c_uint = 0x0007	/* AIF2ADC_DIV - [2:0] */;

//
// R520 (0x208) - Clocking (1)
//
pub const WM8995_LFCLK_ENA: c_uint = 0x0020	/* LFCLK_ENA */;
pub const WM8995_LFCLK_ENA_MASK: c_uint = 0x0020	/* LFCLK_ENA */;

pub const WM8995_TOCLK_ENA: c_uint = 0x0010	/* TOCLK_ENA */;
pub const WM8995_TOCLK_ENA_MASK: c_uint = 0x0010	/* TOCLK_ENA */;

pub const WM8995_AIF1DSPCLK_ENA: c_uint = 0x0008	/* AIF1DSPCLK_ENA */;
pub const WM8995_AIF1DSPCLK_ENA_MASK: c_uint = 0x0008	/* AIF1DSPCLK_ENA */;

pub const WM8995_AIF2DSPCLK_ENA: c_uint = 0x0004	/* AIF2DSPCLK_ENA */;
pub const WM8995_AIF2DSPCLK_ENA_MASK: c_uint = 0x0004	/* AIF2DSPCLK_ENA */;

pub const WM8995_SYSDSPCLK_ENA: c_uint = 0x0002	/* SYSDSPCLK_ENA */;
pub const WM8995_SYSDSPCLK_ENA_MASK: c_uint = 0x0002	/* SYSDSPCLK_ENA */;

pub const WM8995_SYSCLK_SRC: c_uint = 0x0001	/* SYSCLK_SRC */;
pub const WM8995_SYSCLK_SRC_MASK: c_uint = 0x0001	/* SYSCLK_SRC */;

//
// R521 (0x209) - Clocking (2)
//
pub const WM8995_TOCLK_DIV_MASK: c_uint = 0x0700	/* TOCLK_DIV - [10:8] */;

pub const WM8995_DBCLK_DIV_MASK: c_uint = 0x00F0	/* DBCLK_DIV - [7:4] */;

pub const WM8995_OPCLK_DIV_MASK: c_uint = 0x0007	/* OPCLK_DIV - [2:0] */;

//
// R528 (0x210) - AIF1 Rate
//
pub const WM8995_AIF1_SR_MASK: c_uint = 0x00F0	/* AIF1_SR - [7:4] */;

pub const WM8995_AIF1CLK_RATE_MASK: c_uint = 0x000F	/* AIF1CLK_RATE - [3:0] */;

//
// R529 (0x211) - AIF2 Rate
//
pub const WM8995_AIF2_SR_MASK: c_uint = 0x00F0	/* AIF2_SR - [7:4] */;

pub const WM8995_AIF2CLK_RATE_MASK: c_uint = 0x000F	/* AIF2CLK_RATE - [3:0] */;

//
// R530 (0x212) - Rate Status
//
pub const WM8995_SR_ERROR_MASK: c_uint = 0x000F	/* SR_ERROR - [3:0] */;

//
// R544 (0x220) - FLL1 Control (1)
//
pub const WM8995_FLL1_OSC_ENA: c_uint = 0x0002	/* FLL1_OSC_ENA */;
pub const WM8995_FLL1_OSC_ENA_MASK: c_uint = 0x0002	/* FLL1_OSC_ENA */;

pub const WM8995_FLL1_ENA: c_uint = 0x0001	/* FLL1_ENA */;
pub const WM8995_FLL1_ENA_MASK: c_uint = 0x0001	/* FLL1_ENA */;

//
// R545 (0x221) - FLL1 Control (2)
//
pub const WM8995_FLL1_OUTDIV_MASK: c_uint = 0x3F00	/* FLL1_OUTDIV - [13:8] */;

pub const WM8995_FLL1_CTRL_RATE_MASK: c_uint = 0x0070	/* FLL1_CTRL_RATE - [6:4] */;

pub const WM8995_FLL1_FRATIO_MASK: c_uint = 0x0007	/* FLL1_FRATIO - [2:0] */;

//
// R546 (0x222) - FLL1 Control (3)
//
pub const WM8995_FLL1_K_MASK: c_uint = 0xFFFF	/* FLL1_K - [15:0] */;

//
// R547 (0x223) - FLL1 Control (4)
//
pub const WM8995_FLL1_N_MASK: c_uint = 0x7FE0	/* FLL1_N - [14:5] */;

pub const WM8995_FLL1_LOOP_GAIN_MASK: c_uint = 0x000F	/* FLL1_LOOP_GAIN - [3:0] */;

//
// R548 (0x224) - FLL1 Control (5)
//
pub const WM8995_FLL1_FRC_NCO_VAL_MASK: c_uint = 0x1F80	/* FLL1_FRC_NCO_VAL - [12:7] */;

pub const WM8995_FLL1_FRC_NCO: c_uint = 0x0040	/* FLL1_FRC_NCO */;
pub const WM8995_FLL1_FRC_NCO_MASK: c_uint = 0x0040	/* FLL1_FRC_NCO */;

pub const WM8995_FLL1_REFCLK_DIV_MASK: c_uint = 0x0018	/* FLL1_REFCLK_DIV - [4:3] */;

pub const WM8995_FLL1_REFCLK_SRC_MASK: c_uint = 0x0003	/* FLL1_REFCLK_SRC - [1:0] */;

//
// R576 (0x240) - FLL2 Control (1)
//
pub const WM8995_FLL2_OSC_ENA: c_uint = 0x0002	/* FLL2_OSC_ENA */;
pub const WM8995_FLL2_OSC_ENA_MASK: c_uint = 0x0002	/* FLL2_OSC_ENA */;

pub const WM8995_FLL2_ENA: c_uint = 0x0001	/* FLL2_ENA */;
pub const WM8995_FLL2_ENA_MASK: c_uint = 0x0001	/* FLL2_ENA */;

//
// R577 (0x241) - FLL2 Control (2)
//
pub const WM8995_FLL2_OUTDIV_MASK: c_uint = 0x3F00	/* FLL2_OUTDIV - [13:8] */;

pub const WM8995_FLL2_CTRL_RATE_MASK: c_uint = 0x0070	/* FLL2_CTRL_RATE - [6:4] */;

pub const WM8995_FLL2_FRATIO_MASK: c_uint = 0x0007	/* FLL2_FRATIO - [2:0] */;

//
// R578 (0x242) - FLL2 Control (3)
//
pub const WM8995_FLL2_K_MASK: c_uint = 0xFFFF	/* FLL2_K - [15:0] */;

//
// R579 (0x243) - FLL2 Control (4)
//
pub const WM8995_FLL2_N_MASK: c_uint = 0x7FE0	/* FLL2_N - [14:5] */;

pub const WM8995_FLL2_LOOP_GAIN_MASK: c_uint = 0x000F	/* FLL2_LOOP_GAIN - [3:0] */;

//
// R580 (0x244) - FLL2 Control (5)
//
pub const WM8995_FLL2_FRC_NCO_VAL_MASK: c_uint = 0x1F80	/* FLL2_FRC_NCO_VAL - [12:7] */;

pub const WM8995_FLL2_FRC_NCO: c_uint = 0x0040	/* FLL2_FRC_NCO */;
pub const WM8995_FLL2_FRC_NCO_MASK: c_uint = 0x0040	/* FLL2_FRC_NCO */;

pub const WM8995_FLL2_REFCLK_DIV_MASK: c_uint = 0x0018	/* FLL2_REFCLK_DIV - [4:3] */;

pub const WM8995_FLL2_REFCLK_SRC_MASK: c_uint = 0x0003	/* FLL2_REFCLK_SRC - [1:0] */;

//
// R768 (0x300) - AIF1 Control (1)
//
pub const WM8995_AIF1ADCL_SRC: c_uint = 0x8000	/* AIF1ADCL_SRC */;
pub const WM8995_AIF1ADCL_SRC_MASK: c_uint = 0x8000	/* AIF1ADCL_SRC */;

pub const WM8995_AIF1ADCR_SRC: c_uint = 0x4000	/* AIF1ADCR_SRC */;
pub const WM8995_AIF1ADCR_SRC_MASK: c_uint = 0x4000	/* AIF1ADCR_SRC */;

pub const WM8995_AIF1ADC_TDM: c_uint = 0x2000	/* AIF1ADC_TDM */;
pub const WM8995_AIF1ADC_TDM_MASK: c_uint = 0x2000	/* AIF1ADC_TDM */;

pub const WM8995_AIF1_BCLK_INV: c_uint = 0x0100	/* AIF1_BCLK_INV */;
pub const WM8995_AIF1_BCLK_INV_MASK: c_uint = 0x0100	/* AIF1_BCLK_INV */;

pub const WM8995_AIF1_LRCLK_INV: c_uint = 0x0080	/* AIF1_LRCLK_INV */;
pub const WM8995_AIF1_LRCLK_INV_MASK: c_uint = 0x0080	/* AIF1_LRCLK_INV */;

pub const WM8995_AIF1_WL_MASK: c_uint = 0x0060	/* AIF1_WL - [6:5] */;

pub const WM8995_AIF1_FMT_MASK: c_uint = 0x0018	/* AIF1_FMT - [4:3] */;

//
// R769 (0x301) - AIF1 Control (2)
//
pub const WM8995_AIF1DACL_SRC: c_uint = 0x8000	/* AIF1DACL_SRC */;
pub const WM8995_AIF1DACL_SRC_MASK: c_uint = 0x8000	/* AIF1DACL_SRC */;

pub const WM8995_AIF1DACR_SRC: c_uint = 0x4000	/* AIF1DACR_SRC */;
pub const WM8995_AIF1DACR_SRC_MASK: c_uint = 0x4000	/* AIF1DACR_SRC */;

pub const WM8995_AIF1DAC_BOOST_MASK: c_uint = 0x0C00	/* AIF1DAC_BOOST - [11:10] */;

pub const WM8995_AIF1DAC_COMP: c_uint = 0x0010	/* AIF1DAC_COMP */;
pub const WM8995_AIF1DAC_COMP_MASK: c_uint = 0x0010	/* AIF1DAC_COMP */;

pub const WM8995_AIF1DAC_COMPMODE: c_uint = 0x0008	/* AIF1DAC_COMPMODE */;
pub const WM8995_AIF1DAC_COMPMODE_MASK: c_uint = 0x0008	/* AIF1DAC_COMPMODE */;

pub const WM8995_AIF1ADC_COMP: c_uint = 0x0004	/* AIF1ADC_COMP */;
pub const WM8995_AIF1ADC_COMP_MASK: c_uint = 0x0004	/* AIF1ADC_COMP */;

pub const WM8995_AIF1ADC_COMPMODE: c_uint = 0x0002	/* AIF1ADC_COMPMODE */;
pub const WM8995_AIF1ADC_COMPMODE_MASK: c_uint = 0x0002	/* AIF1ADC_COMPMODE */;

pub const WM8995_AIF1_LOOPBACK: c_uint = 0x0001	/* AIF1_LOOPBACK */;
pub const WM8995_AIF1_LOOPBACK_MASK: c_uint = 0x0001	/* AIF1_LOOPBACK */;

//
// R770 (0x302) - AIF1 Master/Slave
//
pub const WM8995_AIF1_TRI: c_uint = 0x8000	/* AIF1_TRI */;
pub const WM8995_AIF1_TRI_MASK: c_uint = 0x8000	/* AIF1_TRI */;

pub const WM8995_AIF1_MSTR: c_uint = 0x4000	/* AIF1_MSTR */;
pub const WM8995_AIF1_MSTR_MASK: c_uint = 0x4000	/* AIF1_MSTR */;

pub const WM8995_AIF1_CLK_FRC: c_uint = 0x2000	/* AIF1_CLK_FRC */;
pub const WM8995_AIF1_CLK_FRC_MASK: c_uint = 0x2000	/* AIF1_CLK_FRC */;

pub const WM8995_AIF1_LRCLK_FRC: c_uint = 0x1000	/* AIF1_LRCLK_FRC */;
pub const WM8995_AIF1_LRCLK_FRC_MASK: c_uint = 0x1000	/* AIF1_LRCLK_FRC */;

//
// R771 (0x303) - AIF1 BCLK
//
pub const WM8995_AIF1_BCLK_DIV_MASK: c_uint = 0x00F0	/* AIF1_BCLK_DIV - [7:4] */;

//
// R772 (0x304) - AIF1ADC LRCLK
//
pub const WM8995_AIF1ADC_LRCLK_DIR: c_uint = 0x0800	/* AIF1ADC_LRCLK_DIR */;
pub const WM8995_AIF1ADC_LRCLK_DIR_MASK: c_uint = 0x0800	/* AIF1ADC_LRCLK_DIR */;

pub const WM8995_AIF1ADC_RATE_MASK: c_uint = 0x07FF	/* AIF1ADC_RATE - [10:0] */;

//
// R773 (0x305) - AIF1DAC LRCLK
//
pub const WM8995_AIF1DAC_LRCLK_DIR: c_uint = 0x0800	/* AIF1DAC_LRCLK_DIR */;
pub const WM8995_AIF1DAC_LRCLK_DIR_MASK: c_uint = 0x0800	/* AIF1DAC_LRCLK_DIR */;

pub const WM8995_AIF1DAC_RATE_MASK: c_uint = 0x07FF	/* AIF1DAC_RATE - [10:0] */;

//
// R774 (0x306) - AIF1DAC Data
//
pub const WM8995_AIF1DACL_DAT_INV: c_uint = 0x0002	/* AIF1DACL_DAT_INV */;
pub const WM8995_AIF1DACL_DAT_INV_MASK: c_uint = 0x0002	/* AIF1DACL_DAT_INV */;

pub const WM8995_AIF1DACR_DAT_INV: c_uint = 0x0001	/* AIF1DACR_DAT_INV */;
pub const WM8995_AIF1DACR_DAT_INV_MASK: c_uint = 0x0001	/* AIF1DACR_DAT_INV */;

//
// R775 (0x307) - AIF1ADC Data
//
pub const WM8995_AIF1ADCL_DAT_INV: c_uint = 0x0002	/* AIF1ADCL_DAT_INV */;
pub const WM8995_AIF1ADCL_DAT_INV_MASK: c_uint = 0x0002	/* AIF1ADCL_DAT_INV */;

pub const WM8995_AIF1ADCR_DAT_INV: c_uint = 0x0001	/* AIF1ADCR_DAT_INV */;
pub const WM8995_AIF1ADCR_DAT_INV_MASK: c_uint = 0x0001	/* AIF1ADCR_DAT_INV */;

//
// R784 (0x310) - AIF2 Control (1)
//
pub const WM8995_AIF2ADCL_SRC: c_uint = 0x8000	/* AIF2ADCL_SRC */;
pub const WM8995_AIF2ADCL_SRC_MASK: c_uint = 0x8000	/* AIF2ADCL_SRC */;

pub const WM8995_AIF2ADCR_SRC: c_uint = 0x4000	/* AIF2ADCR_SRC */;
pub const WM8995_AIF2ADCR_SRC_MASK: c_uint = 0x4000	/* AIF2ADCR_SRC */;

pub const WM8995_AIF2ADC_TDM: c_uint = 0x2000	/* AIF2ADC_TDM */;
pub const WM8995_AIF2ADC_TDM_MASK: c_uint = 0x2000	/* AIF2ADC_TDM */;

pub const WM8995_AIF2ADC_TDM_CHAN: c_uint = 0x1000	/* AIF2ADC_TDM_CHAN */;
pub const WM8995_AIF2ADC_TDM_CHAN_MASK: c_uint = 0x1000	/* AIF2ADC_TDM_CHAN */;

pub const WM8995_AIF2_BCLK_INV: c_uint = 0x0100	/* AIF2_BCLK_INV */;
pub const WM8995_AIF2_BCLK_INV_MASK: c_uint = 0x0100	/* AIF2_BCLK_INV */;

pub const WM8995_AIF2_LRCLK_INV: c_uint = 0x0080	/* AIF2_LRCLK_INV */;
pub const WM8995_AIF2_LRCLK_INV_MASK: c_uint = 0x0080	/* AIF2_LRCLK_INV */;

pub const WM8995_AIF2_WL_MASK: c_uint = 0x0060	/* AIF2_WL - [6:5] */;

pub const WM8995_AIF2_FMT_MASK: c_uint = 0x0018	/* AIF2_FMT - [4:3] */;

//
// R785 (0x311) - AIF2 Control (2)
//
pub const WM8995_AIF2DACL_SRC: c_uint = 0x8000	/* AIF2DACL_SRC */;
pub const WM8995_AIF2DACL_SRC_MASK: c_uint = 0x8000	/* AIF2DACL_SRC */;

pub const WM8995_AIF2DACR_SRC: c_uint = 0x4000	/* AIF2DACR_SRC */;
pub const WM8995_AIF2DACR_SRC_MASK: c_uint = 0x4000	/* AIF2DACR_SRC */;

pub const WM8995_AIF2DAC_TDM: c_uint = 0x2000	/* AIF2DAC_TDM */;
pub const WM8995_AIF2DAC_TDM_MASK: c_uint = 0x2000	/* AIF2DAC_TDM */;

pub const WM8995_AIF2DAC_TDM_CHAN: c_uint = 0x1000	/* AIF2DAC_TDM_CHAN */;
pub const WM8995_AIF2DAC_TDM_CHAN_MASK: c_uint = 0x1000	/* AIF2DAC_TDM_CHAN */;

pub const WM8995_AIF2DAC_BOOST_MASK: c_uint = 0x0C00	/* AIF2DAC_BOOST - [11:10] */;

pub const WM8995_AIF2DAC_COMP: c_uint = 0x0010	/* AIF2DAC_COMP */;
pub const WM8995_AIF2DAC_COMP_MASK: c_uint = 0x0010	/* AIF2DAC_COMP */;

pub const WM8995_AIF2DAC_COMPMODE: c_uint = 0x0008	/* AIF2DAC_COMPMODE */;
pub const WM8995_AIF2DAC_COMPMODE_MASK: c_uint = 0x0008	/* AIF2DAC_COMPMODE */;

pub const WM8995_AIF2ADC_COMP: c_uint = 0x0004	/* AIF2ADC_COMP */;
pub const WM8995_AIF2ADC_COMP_MASK: c_uint = 0x0004	/* AIF2ADC_COMP */;

pub const WM8995_AIF2ADC_COMPMODE: c_uint = 0x0002	/* AIF2ADC_COMPMODE */;
pub const WM8995_AIF2ADC_COMPMODE_MASK: c_uint = 0x0002	/* AIF2ADC_COMPMODE */;

pub const WM8995_AIF2_LOOPBACK: c_uint = 0x0001	/* AIF2_LOOPBACK */;
pub const WM8995_AIF2_LOOPBACK_MASK: c_uint = 0x0001	/* AIF2_LOOPBACK */;

//
// R786 (0x312) - AIF2 Master/Slave
//
pub const WM8995_AIF2_TRI: c_uint = 0x8000	/* AIF2_TRI */;
pub const WM8995_AIF2_TRI_MASK: c_uint = 0x8000	/* AIF2_TRI */;

pub const WM8995_AIF2_MSTR: c_uint = 0x4000	/* AIF2_MSTR */;
pub const WM8995_AIF2_MSTR_MASK: c_uint = 0x4000	/* AIF2_MSTR */;

pub const WM8995_AIF2_CLK_FRC: c_uint = 0x2000	/* AIF2_CLK_FRC */;
pub const WM8995_AIF2_CLK_FRC_MASK: c_uint = 0x2000	/* AIF2_CLK_FRC */;

pub const WM8995_AIF2_LRCLK_FRC: c_uint = 0x1000	/* AIF2_LRCLK_FRC */;
pub const WM8995_AIF2_LRCLK_FRC_MASK: c_uint = 0x1000	/* AIF2_LRCLK_FRC */;

//
// R787 (0x313) - AIF2 BCLK
//
pub const WM8995_AIF2_BCLK_DIV_MASK: c_uint = 0x00F0	/* AIF2_BCLK_DIV - [7:4] */;

//
// R788 (0x314) - AIF2ADC LRCLK
//
pub const WM8995_AIF2ADC_LRCLK_DIR: c_uint = 0x0800	/* AIF2ADC_LRCLK_DIR */;
pub const WM8995_AIF2ADC_LRCLK_DIR_MASK: c_uint = 0x0800	/* AIF2ADC_LRCLK_DIR */;

pub const WM8995_AIF2ADC_RATE_MASK: c_uint = 0x07FF	/* AIF2ADC_RATE - [10:0] */;

//
// R789 (0x315) - AIF2DAC LRCLK
//
pub const WM8995_AIF2DAC_LRCLK_DIR: c_uint = 0x0800	/* AIF2DAC_LRCLK_DIR */;
pub const WM8995_AIF2DAC_LRCLK_DIR_MASK: c_uint = 0x0800	/* AIF2DAC_LRCLK_DIR */;

pub const WM8995_AIF2DAC_RATE_MASK: c_uint = 0x07FF	/* AIF2DAC_RATE - [10:0] */;

//
// R790 (0x316) - AIF2DAC Data
//
pub const WM8995_AIF2DACL_DAT_INV: c_uint = 0x0002	/* AIF2DACL_DAT_INV */;
pub const WM8995_AIF2DACL_DAT_INV_MASK: c_uint = 0x0002	/* AIF2DACL_DAT_INV */;

pub const WM8995_AIF2DACR_DAT_INV: c_uint = 0x0001	/* AIF2DACR_DAT_INV */;
pub const WM8995_AIF2DACR_DAT_INV_MASK: c_uint = 0x0001	/* AIF2DACR_DAT_INV */;

//
// R791 (0x317) - AIF2ADC Data
//
pub const WM8995_AIF2ADCL_DAT_INV: c_uint = 0x0002	/* AIF2ADCL_DAT_INV */;
pub const WM8995_AIF2ADCL_DAT_INV_MASK: c_uint = 0x0002	/* AIF2ADCL_DAT_INV */;

pub const WM8995_AIF2ADCR_DAT_INV: c_uint = 0x0001	/* AIF2ADCR_DAT_INV */;
pub const WM8995_AIF2ADCR_DAT_INV_MASK: c_uint = 0x0001	/* AIF2ADCR_DAT_INV */;

//
// R1024 (0x400) - AIF1 ADC1 Left Volume
//
pub const WM8995_AIF1ADC1_VU: c_uint = 0x0100	/* AIF1ADC1_VU */;
pub const WM8995_AIF1ADC1_VU_MASK: c_uint = 0x0100	/* AIF1ADC1_VU */;

pub const WM8995_AIF1ADC1L_VOL_MASK: c_uint = 0x00FF	/* AIF1ADC1L_VOL - [7:0] */;

//
// R1025 (0x401) - AIF1 ADC1 Right Volume
//
pub const WM8995_AIF1ADC1_VU: c_uint = 0x0100	/* AIF1ADC1_VU */;
pub const WM8995_AIF1ADC1_VU_MASK: c_uint = 0x0100	/* AIF1ADC1_VU */;

pub const WM8995_AIF1ADC1R_VOL_MASK: c_uint = 0x00FF	/* AIF1ADC1R_VOL - [7:0] */;

//
// R1026 (0x402) - AIF1 DAC1 Left Volume
//
pub const WM8995_AIF1DAC1_VU: c_uint = 0x0100	/* AIF1DAC1_VU */;
pub const WM8995_AIF1DAC1_VU_MASK: c_uint = 0x0100	/* AIF1DAC1_VU */;

pub const WM8995_AIF1DAC1L_VOL_MASK: c_uint = 0x00FF	/* AIF1DAC1L_VOL - [7:0] */;

//
// R1027 (0x403) - AIF1 DAC1 Right Volume
//
pub const WM8995_AIF1DAC1_VU: c_uint = 0x0100	/* AIF1DAC1_VU */;
pub const WM8995_AIF1DAC1_VU_MASK: c_uint = 0x0100	/* AIF1DAC1_VU */;

pub const WM8995_AIF1DAC1R_VOL_MASK: c_uint = 0x00FF	/* AIF1DAC1R_VOL - [7:0] */;

//
// R1028 (0x404) - AIF1 ADC2 Left Volume
//
pub const WM8995_AIF1ADC2_VU: c_uint = 0x0100	/* AIF1ADC2_VU */;
pub const WM8995_AIF1ADC2_VU_MASK: c_uint = 0x0100	/* AIF1ADC2_VU */;

pub const WM8995_AIF1ADC2L_VOL_MASK: c_uint = 0x00FF	/* AIF1ADC2L_VOL - [7:0] */;

//
// R1029 (0x405) - AIF1 ADC2 Right Volume
//
pub const WM8995_AIF1ADC2_VU: c_uint = 0x0100	/* AIF1ADC2_VU */;
pub const WM8995_AIF1ADC2_VU_MASK: c_uint = 0x0100	/* AIF1ADC2_VU */;

pub const WM8995_AIF1ADC2R_VOL_MASK: c_uint = 0x00FF	/* AIF1ADC2R_VOL - [7:0] */;

//
// R1030 (0x406) - AIF1 DAC2 Left Volume
//
pub const WM8995_AIF1DAC2_VU: c_uint = 0x0100	/* AIF1DAC2_VU */;
pub const WM8995_AIF1DAC2_VU_MASK: c_uint = 0x0100	/* AIF1DAC2_VU */;

pub const WM8995_AIF1DAC2L_VOL_MASK: c_uint = 0x00FF	/* AIF1DAC2L_VOL - [7:0] */;

//
// R1031 (0x407) - AIF1 DAC2 Right Volume
//
pub const WM8995_AIF1DAC2_VU: c_uint = 0x0100	/* AIF1DAC2_VU */;
pub const WM8995_AIF1DAC2_VU_MASK: c_uint = 0x0100	/* AIF1DAC2_VU */;

pub const WM8995_AIF1DAC2R_VOL_MASK: c_uint = 0x00FF	/* AIF1DAC2R_VOL - [7:0] */;

//
// R1040 (0x410) - AIF1 ADC1 Filters
//
pub const WM8995_AIF1ADC_4FS: c_uint = 0x8000	/* AIF1ADC_4FS */;
pub const WM8995_AIF1ADC_4FS_MASK: c_uint = 0x8000	/* AIF1ADC_4FS */;

pub const WM8995_AIF1ADC1L_HPF: c_uint = 0x1000	/* AIF1ADC1L_HPF */;
pub const WM8995_AIF1ADC1L_HPF_MASK: c_uint = 0x1000	/* AIF1ADC1L_HPF */;

pub const WM8995_AIF1ADC1R_HPF: c_uint = 0x0800	/* AIF1ADC1R_HPF */;
pub const WM8995_AIF1ADC1R_HPF_MASK: c_uint = 0x0800	/* AIF1ADC1R_HPF */;

pub const WM8995_AIF1ADC1_HPF_MODE: c_uint = 0x0008	/* AIF1ADC1_HPF_MODE */;
pub const WM8995_AIF1ADC1_HPF_MODE_MASK: c_uint = 0x0008	/* AIF1ADC1_HPF_MODE */;

pub const WM8995_AIF1ADC1_HPF_CUT_MASK: c_uint = 0x0007	/* AIF1ADC1_HPF_CUT - [2:0] */;

//
// R1041 (0x411) - AIF1 ADC2 Filters
//
pub const WM8995_AIF1ADC2L_HPF: c_uint = 0x1000	/* AIF1ADC2L_HPF */;
pub const WM8995_AIF1ADC2L_HPF_MASK: c_uint = 0x1000	/* AIF1ADC2L_HPF */;

pub const WM8995_AIF1ADC2R_HPF: c_uint = 0x0800	/* AIF1ADC2R_HPF */;
pub const WM8995_AIF1ADC2R_HPF_MASK: c_uint = 0x0800	/* AIF1ADC2R_HPF */;

pub const WM8995_AIF1ADC2_HPF_MODE: c_uint = 0x0008	/* AIF1ADC2_HPF_MODE */;
pub const WM8995_AIF1ADC2_HPF_MODE_MASK: c_uint = 0x0008	/* AIF1ADC2_HPF_MODE */;

pub const WM8995_AIF1ADC2_HPF_CUT_MASK: c_uint = 0x0007	/* AIF1ADC2_HPF_CUT - [2:0] */;

//
// R1056 (0x420) - AIF1 DAC1 Filters (1)
//
pub const WM8995_AIF1DAC1_MUTE: c_uint = 0x0200	/* AIF1DAC1_MUTE */;
pub const WM8995_AIF1DAC1_MUTE_MASK: c_uint = 0x0200	/* AIF1DAC1_MUTE */;

pub const WM8995_AIF1DAC1_MONO: c_uint = 0x0080	/* AIF1DAC1_MONO */;
pub const WM8995_AIF1DAC1_MONO_MASK: c_uint = 0x0080	/* AIF1DAC1_MONO */;

pub const WM8995_AIF1DAC1_MUTERATE: c_uint = 0x0020	/* AIF1DAC1_MUTERATE */;
pub const WM8995_AIF1DAC1_MUTERATE_MASK: c_uint = 0x0020	/* AIF1DAC1_MUTERATE */;

pub const WM8995_AIF1DAC1_UNMUTE_RAMP: c_uint = 0x0010	/* AIF1DAC1_UNMUTE_RAMP */;
pub const WM8995_AIF1DAC1_UNMUTE_RAMP_MASK: c_uint = 0x0010	/* AIF1DAC1_UNMUTE_RAMP */;

pub const WM8995_AIF1DAC1_DEEMP_MASK: c_uint = 0x0006	/* AIF1DAC1_DEEMP - [2:1] */;

//
// R1057 (0x421) - AIF1 DAC1 Filters (2)
//
pub const WM8995_AIF1DAC1_3D_GAIN_MASK: c_uint = 0x3E00	/* AIF1DAC1_3D_GAIN - [13:9] */;

pub const WM8995_AIF1DAC1_3D_ENA: c_uint = 0x0100	/* AIF1DAC1_3D_ENA */;
pub const WM8995_AIF1DAC1_3D_ENA_MASK: c_uint = 0x0100	/* AIF1DAC1_3D_ENA */;

//
// R1058 (0x422) - AIF1 DAC2 Filters (1)
//
pub const WM8995_AIF1DAC2_MUTE: c_uint = 0x0200	/* AIF1DAC2_MUTE */;
pub const WM8995_AIF1DAC2_MUTE_MASK: c_uint = 0x0200	/* AIF1DAC2_MUTE */;

pub const WM8995_AIF1DAC2_MONO: c_uint = 0x0080	/* AIF1DAC2_MONO */;
pub const WM8995_AIF1DAC2_MONO_MASK: c_uint = 0x0080	/* AIF1DAC2_MONO */;

pub const WM8995_AIF1DAC2_MUTERATE: c_uint = 0x0020	/* AIF1DAC2_MUTERATE */;
pub const WM8995_AIF1DAC2_MUTERATE_MASK: c_uint = 0x0020	/* AIF1DAC2_MUTERATE */;

pub const WM8995_AIF1DAC2_UNMUTE_RAMP: c_uint = 0x0010	/* AIF1DAC2_UNMUTE_RAMP */;
pub const WM8995_AIF1DAC2_UNMUTE_RAMP_MASK: c_uint = 0x0010	/* AIF1DAC2_UNMUTE_RAMP */;

pub const WM8995_AIF1DAC2_DEEMP_MASK: c_uint = 0x0006	/* AIF1DAC2_DEEMP - [2:1] */;

//
// R1059 (0x423) - AIF1 DAC2 Filters (2)
//
pub const WM8995_AIF1DAC2_3D_GAIN_MASK: c_uint = 0x3E00	/* AIF1DAC2_3D_GAIN - [13:9] */;

pub const WM8995_AIF1DAC2_3D_ENA: c_uint = 0x0100	/* AIF1DAC2_3D_ENA */;
pub const WM8995_AIF1DAC2_3D_ENA_MASK: c_uint = 0x0100	/* AIF1DAC2_3D_ENA */;

//
// R1088 (0x440) - AIF1 DRC1 (1)
//
pub const WM8995_AIF1DRC1_SIG_DET_RMS_MASK: c_uint = 0xF800	/* AIF1DRC1_SIG_DET_RMS - [15:11] */;

pub const WM8995_AIF1DRC1_SIG_DET_PK_MASK: c_uint = 0x0600	/* AIF1DRC1_SIG_DET_PK - [10:9] */;

pub const WM8995_AIF1DRC1_NG_ENA: c_uint = 0x0100	/* AIF1DRC1_NG_ENA */;
pub const WM8995_AIF1DRC1_NG_ENA_MASK: c_uint = 0x0100	/* AIF1DRC1_NG_ENA */;

pub const WM8995_AIF1DRC1_SIG_DET_MODE: c_uint = 0x0080	/* AIF1DRC1_SIG_DET_MODE */;
pub const WM8995_AIF1DRC1_SIG_DET_MODE_MASK: c_uint = 0x0080	/* AIF1DRC1_SIG_DET_MODE */;

pub const WM8995_AIF1DRC1_SIG_DET: c_uint = 0x0040	/* AIF1DRC1_SIG_DET */;
pub const WM8995_AIF1DRC1_SIG_DET_MASK: c_uint = 0x0040	/* AIF1DRC1_SIG_DET */;

pub const WM8995_AIF1DRC1_KNEE2_OP_ENA: c_uint = 0x0020	/* AIF1DRC1_KNEE2_OP_ENA */;
pub const WM8995_AIF1DRC1_KNEE2_OP_ENA_MASK: c_uint = 0x0020	/* AIF1DRC1_KNEE2_OP_ENA */;

pub const WM8995_AIF1DRC1_QR: c_uint = 0x0010	/* AIF1DRC1_QR */;
pub const WM8995_AIF1DRC1_QR_MASK: c_uint = 0x0010	/* AIF1DRC1_QR */;

pub const WM8995_AIF1DRC1_ANTICLIP: c_uint = 0x0008	/* AIF1DRC1_ANTICLIP */;
pub const WM8995_AIF1DRC1_ANTICLIP_MASK: c_uint = 0x0008	/* AIF1DRC1_ANTICLIP */;

pub const WM8995_AIF1DAC1_DRC_ENA: c_uint = 0x0004	/* AIF1DAC1_DRC_ENA */;
pub const WM8995_AIF1DAC1_DRC_ENA_MASK: c_uint = 0x0004	/* AIF1DAC1_DRC_ENA */;

pub const WM8995_AIF1ADC1L_DRC_ENA: c_uint = 0x0002	/* AIF1ADC1L_DRC_ENA */;
pub const WM8995_AIF1ADC1L_DRC_ENA_MASK: c_uint = 0x0002	/* AIF1ADC1L_DRC_ENA */;

pub const WM8995_AIF1ADC1R_DRC_ENA: c_uint = 0x0001	/* AIF1ADC1R_DRC_ENA */;
pub const WM8995_AIF1ADC1R_DRC_ENA_MASK: c_uint = 0x0001	/* AIF1ADC1R_DRC_ENA */;

//
// R1089 (0x441) - AIF1 DRC1 (2)
//
pub const WM8995_AIF1DRC1_ATK_MASK: c_uint = 0x1E00	/* AIF1DRC1_ATK - [12:9] */;

pub const WM8995_AIF1DRC1_DCY_MASK: c_uint = 0x01E0	/* AIF1DRC1_DCY - [8:5] */;

pub const WM8995_AIF1DRC1_MINGAIN_MASK: c_uint = 0x001C	/* AIF1DRC1_MINGAIN - [4:2] */;

pub const WM8995_AIF1DRC1_MAXGAIN_MASK: c_uint = 0x0003	/* AIF1DRC1_MAXGAIN - [1:0] */;

//
// R1090 (0x442) - AIF1 DRC1 (3)
//
pub const WM8995_AIF1DRC1_NG_MINGAIN_MASK: c_uint = 0xF000	/* AIF1DRC1_NG_MINGAIN - [15:12] */;

pub const WM8995_AIF1DRC1_NG_EXP_MASK: c_uint = 0x0C00	/* AIF1DRC1_NG_EXP - [11:10] */;

pub const WM8995_AIF1DRC1_QR_THR_MASK: c_uint = 0x0300	/* AIF1DRC1_QR_THR - [9:8] */;

pub const WM8995_AIF1DRC1_QR_DCY_MASK: c_uint = 0x00C0	/* AIF1DRC1_QR_DCY - [7:6] */;

pub const WM8995_AIF1DRC1_HI_COMP_MASK: c_uint = 0x0038	/* AIF1DRC1_HI_COMP - [5:3] */;

pub const WM8995_AIF1DRC1_LO_COMP_MASK: c_uint = 0x0007	/* AIF1DRC1_LO_COMP - [2:0] */;

//
// R1091 (0x443) - AIF1 DRC1 (4)
//
pub const WM8995_AIF1DRC1_KNEE_IP_MASK: c_uint = 0x07E0	/* AIF1DRC1_KNEE_IP - [10:5] */;

pub const WM8995_AIF1DRC1_KNEE_OP_MASK: c_uint = 0x001F	/* AIF1DRC1_KNEE_OP - [4:0] */;

//
// R1092 (0x444) - AIF1 DRC1 (5)
//
pub const WM8995_AIF1DRC1_KNEE2_IP_MASK: c_uint = 0x03E0	/* AIF1DRC1_KNEE2_IP - [9:5] */;

pub const WM8995_AIF1DRC1_KNEE2_OP_MASK: c_uint = 0x001F	/* AIF1DRC1_KNEE2_OP - [4:0] */;

//
// R1104 (0x450) - AIF1 DRC2 (1)
//
pub const WM8995_AIF1DRC2_SIG_DET_RMS_MASK: c_uint = 0xF800	/* AIF1DRC2_SIG_DET_RMS - [15:11] */;

pub const WM8995_AIF1DRC2_SIG_DET_PK_MASK: c_uint = 0x0600	/* AIF1DRC2_SIG_DET_PK - [10:9] */;

pub const WM8995_AIF1DRC2_NG_ENA: c_uint = 0x0100	/* AIF1DRC2_NG_ENA */;
pub const WM8995_AIF1DRC2_NG_ENA_MASK: c_uint = 0x0100	/* AIF1DRC2_NG_ENA */;

pub const WM8995_AIF1DRC2_SIG_DET_MODE: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_MODE */;
pub const WM8995_AIF1DRC2_SIG_DET_MODE_MASK: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_MODE */;

pub const WM8995_AIF1DRC2_SIG_DET: c_uint = 0x0040	/* AIF1DRC2_SIG_DET */;
pub const WM8995_AIF1DRC2_SIG_DET_MASK: c_uint = 0x0040	/* AIF1DRC2_SIG_DET */;

pub const WM8995_AIF1DRC2_KNEE2_OP_ENA: c_uint = 0x0020	/* AIF1DRC2_KNEE2_OP_ENA */;
pub const WM8995_AIF1DRC2_KNEE2_OP_ENA_MASK: c_uint = 0x0020	/* AIF1DRC2_KNEE2_OP_ENA */;

pub const WM8995_AIF1DRC2_QR: c_uint = 0x0010	/* AIF1DRC2_QR */;
pub const WM8995_AIF1DRC2_QR_MASK: c_uint = 0x0010	/* AIF1DRC2_QR */;

pub const WM8995_AIF1DRC2_ANTICLIP: c_uint = 0x0008	/* AIF1DRC2_ANTICLIP */;
pub const WM8995_AIF1DRC2_ANTICLIP_MASK: c_uint = 0x0008	/* AIF1DRC2_ANTICLIP */;

pub const WM8995_AIF1DAC2_DRC_ENA: c_uint = 0x0004	/* AIF1DAC2_DRC_ENA */;
pub const WM8995_AIF1DAC2_DRC_ENA_MASK: c_uint = 0x0004	/* AIF1DAC2_DRC_ENA */;

pub const WM8995_AIF1ADC2L_DRC_ENA: c_uint = 0x0002	/* AIF1ADC2L_DRC_ENA */;
pub const WM8995_AIF1ADC2L_DRC_ENA_MASK: c_uint = 0x0002	/* AIF1ADC2L_DRC_ENA */;

pub const WM8995_AIF1ADC2R_DRC_ENA: c_uint = 0x0001	/* AIF1ADC2R_DRC_ENA */;
pub const WM8995_AIF1ADC2R_DRC_ENA_MASK: c_uint = 0x0001	/* AIF1ADC2R_DRC_ENA */;

//
// R1105 (0x451) - AIF1 DRC2 (2)
//
pub const WM8995_AIF1DRC2_ATK_MASK: c_uint = 0x1E00	/* AIF1DRC2_ATK - [12:9] */;

pub const WM8995_AIF1DRC2_DCY_MASK: c_uint = 0x01E0	/* AIF1DRC2_DCY - [8:5] */;

pub const WM8995_AIF1DRC2_MINGAIN_MASK: c_uint = 0x001C	/* AIF1DRC2_MINGAIN - [4:2] */;

pub const WM8995_AIF1DRC2_MAXGAIN_MASK: c_uint = 0x0003	/* AIF1DRC2_MAXGAIN - [1:0] */;

//
// R1106 (0x452) - AIF1 DRC2 (3)
//
pub const WM8995_AIF1DRC2_NG_MINGAIN_MASK: c_uint = 0xF000	/* AIF1DRC2_NG_MINGAIN - [15:12] */;

pub const WM8995_AIF1DRC2_NG_EXP_MASK: c_uint = 0x0C00	/* AIF1DRC2_NG_EXP - [11:10] */;

pub const WM8995_AIF1DRC2_QR_THR_MASK: c_uint = 0x0300	/* AIF1DRC2_QR_THR - [9:8] */;

pub const WM8995_AIF1DRC2_QR_DCY_MASK: c_uint = 0x00C0	/* AIF1DRC2_QR_DCY - [7:6] */;

pub const WM8995_AIF1DRC2_HI_COMP_MASK: c_uint = 0x0038	/* AIF1DRC2_HI_COMP - [5:3] */;

pub const WM8995_AIF1DRC2_LO_COMP_MASK: c_uint = 0x0007	/* AIF1DRC2_LO_COMP - [2:0] */;

//
// R1107 (0x453) - AIF1 DRC2 (4)
//
pub const WM8995_AIF1DRC2_KNEE_IP_MASK: c_uint = 0x07E0	/* AIF1DRC2_KNEE_IP - [10:5] */;

pub const WM8995_AIF1DRC2_KNEE_OP_MASK: c_uint = 0x001F	/* AIF1DRC2_KNEE_OP - [4:0] */;

//
// R1108 (0x454) - AIF1 DRC2 (5)
//
pub const WM8995_AIF1DRC2_KNEE2_IP_MASK: c_uint = 0x03E0	/* AIF1DRC2_KNEE2_IP - [9:5] */;

pub const WM8995_AIF1DRC2_KNEE2_OP_MASK: c_uint = 0x001F	/* AIF1DRC2_KNEE2_OP - [4:0] */;

//
// R1152 (0x480) - AIF1 DAC1 EQ Gains (1)
//
pub const WM8995_AIF1DAC1_EQ_B1_GAIN_MASK: c_uint = 0xF800	/* AIF1DAC1_EQ_B1_GAIN - [15:11] */;

pub const WM8995_AIF1DAC1_EQ_B2_GAIN_MASK: c_uint = 0x07C0	/* AIF1DAC1_EQ_B2_GAIN - [10:6] */;

pub const WM8995_AIF1DAC1_EQ_B3_GAIN_MASK: c_uint = 0x003E	/* AIF1DAC1_EQ_B3_GAIN - [5:1] */;

pub const WM8995_AIF1DAC1_EQ_ENA: c_uint = 0x0001	/* AIF1DAC1_EQ_ENA */;
pub const WM8995_AIF1DAC1_EQ_ENA_MASK: c_uint = 0x0001	/* AIF1DAC1_EQ_ENA */;

//
// R1153 (0x481) - AIF1 DAC1 EQ Gains (2)
//
pub const WM8995_AIF1DAC1_EQ_B4_GAIN_MASK: c_uint = 0xF800	/* AIF1DAC1_EQ_B4_GAIN - [15:11] */;

pub const WM8995_AIF1DAC1_EQ_B5_GAIN_MASK: c_uint = 0x07C0	/* AIF1DAC1_EQ_B5_GAIN - [10:6] */;

//
// R1154 (0x482) - AIF1 DAC1 EQ Band 1 A
//
pub const WM8995_AIF1DAC1_EQ_B1_A_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B1_A - [15:0] */;

//
// R1155 (0x483) - AIF1 DAC1 EQ Band 1 B
//
pub const WM8995_AIF1DAC1_EQ_B1_B_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B1_B - [15:0] */;

//
// R1156 (0x484) - AIF1 DAC1 EQ Band 1 PG
//
pub const WM8995_AIF1DAC1_EQ_B1_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B1_PG - [15:0] */;

//
// R1157 (0x485) - AIF1 DAC1 EQ Band 2 A
//
pub const WM8995_AIF1DAC1_EQ_B2_A_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B2_A - [15:0] */;

//
// R1158 (0x486) - AIF1 DAC1 EQ Band 2 B
//
pub const WM8995_AIF1DAC1_EQ_B2_B_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B2_B - [15:0] */;

//
// R1159 (0x487) - AIF1 DAC1 EQ Band 2 C
//
pub const WM8995_AIF1DAC1_EQ_B2_C_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B2_C - [15:0] */;

//
// R1160 (0x488) - AIF1 DAC1 EQ Band 2 PG
//
pub const WM8995_AIF1DAC1_EQ_B2_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B2_PG - [15:0] */;

//
// R1161 (0x489) - AIF1 DAC1 EQ Band 3 A
//
pub const WM8995_AIF1DAC1_EQ_B3_A_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B3_A - [15:0] */;

//
// R1162 (0x48A) - AIF1 DAC1 EQ Band 3 B
//
pub const WM8995_AIF1DAC1_EQ_B3_B_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B3_B - [15:0] */;

//
// R1163 (0x48B) - AIF1 DAC1 EQ Band 3 C
//
pub const WM8995_AIF1DAC1_EQ_B3_C_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B3_C - [15:0] */;

//
// R1164 (0x48C) - AIF1 DAC1 EQ Band 3 PG
//
pub const WM8995_AIF1DAC1_EQ_B3_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B3_PG - [15:0] */;

//
// R1165 (0x48D) - AIF1 DAC1 EQ Band 4 A
//
pub const WM8995_AIF1DAC1_EQ_B4_A_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B4_A - [15:0] */;

//
// R1166 (0x48E) - AIF1 DAC1 EQ Band 4 B
//
pub const WM8995_AIF1DAC1_EQ_B4_B_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B4_B - [15:0] */;

//
// R1167 (0x48F) - AIF1 DAC1 EQ Band 4 C
//
pub const WM8995_AIF1DAC1_EQ_B4_C_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B4_C - [15:0] */;

//
// R1168 (0x490) - AIF1 DAC1 EQ Band 4 PG
//
pub const WM8995_AIF1DAC1_EQ_B4_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B4_PG - [15:0] */;

//
// R1169 (0x491) - AIF1 DAC1 EQ Band 5 A
//
pub const WM8995_AIF1DAC1_EQ_B5_A_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B5_A - [15:0] */;

//
// R1170 (0x492) - AIF1 DAC1 EQ Band 5 B
//
pub const WM8995_AIF1DAC1_EQ_B5_B_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B5_B - [15:0] */;

//
// R1171 (0x493) - AIF1 DAC1 EQ Band 5 PG
//
pub const WM8995_AIF1DAC1_EQ_B5_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC1_EQ_B5_PG - [15:0] */;

//
// R1184 (0x4A0) - AIF1 DAC2 EQ Gains (1)
//
pub const WM8995_AIF1DAC2_EQ_B1_GAIN_MASK: c_uint = 0xF800	/* AIF1DAC2_EQ_B1_GAIN - [15:11] */;

pub const WM8995_AIF1DAC2_EQ_B2_GAIN_MASK: c_uint = 0x07C0	/* AIF1DAC2_EQ_B2_GAIN - [10:6] */;

pub const WM8995_AIF1DAC2_EQ_B3_GAIN_MASK: c_uint = 0x003E	/* AIF1DAC2_EQ_B3_GAIN - [5:1] */;

pub const WM8995_AIF1DAC2_EQ_ENA: c_uint = 0x0001	/* AIF1DAC2_EQ_ENA */;
pub const WM8995_AIF1DAC2_EQ_ENA_MASK: c_uint = 0x0001	/* AIF1DAC2_EQ_ENA */;

//
// R1185 (0x4A1) - AIF1 DAC2 EQ Gains (2)
//
pub const WM8995_AIF1DAC2_EQ_B4_GAIN_MASK: c_uint = 0xF800	/* AIF1DAC2_EQ_B4_GAIN - [15:11] */;

pub const WM8995_AIF1DAC2_EQ_B5_GAIN_MASK: c_uint = 0x07C0	/* AIF1DAC2_EQ_B5_GAIN - [10:6] */;

//
// R1186 (0x4A2) - AIF1 DAC2 EQ Band 1 A
//
pub const WM8995_AIF1DAC2_EQ_B1_A_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B1_A - [15:0] */;

//
// R1187 (0x4A3) - AIF1 DAC2 EQ Band 1 B
//
pub const WM8995_AIF1DAC2_EQ_B1_B_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B1_B - [15:0] */;

//
// R1188 (0x4A4) - AIF1 DAC2 EQ Band 1 PG
//
pub const WM8995_AIF1DAC2_EQ_B1_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B1_PG - [15:0] */;

//
// R1189 (0x4A5) - AIF1 DAC2 EQ Band 2 A
//
pub const WM8995_AIF1DAC2_EQ_B2_A_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B2_A - [15:0] */;

//
// R1190 (0x4A6) - AIF1 DAC2 EQ Band 2 B
//
pub const WM8995_AIF1DAC2_EQ_B2_B_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B2_B - [15:0] */;

//
// R1191 (0x4A7) - AIF1 DAC2 EQ Band 2 C
//
pub const WM8995_AIF1DAC2_EQ_B2_C_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B2_C - [15:0] */;

//
// R1192 (0x4A8) - AIF1 DAC2 EQ Band 2 PG
//
pub const WM8995_AIF1DAC2_EQ_B2_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B2_PG - [15:0] */;

//
// R1193 (0x4A9) - AIF1 DAC2 EQ Band 3 A
//
pub const WM8995_AIF1DAC2_EQ_B3_A_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B3_A - [15:0] */;

//
// R1194 (0x4AA) - AIF1 DAC2 EQ Band 3 B
//
pub const WM8995_AIF1DAC2_EQ_B3_B_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B3_B - [15:0] */;

//
// R1195 (0x4AB) - AIF1 DAC2 EQ Band 3 C
//
pub const WM8995_AIF1DAC2_EQ_B3_C_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B3_C - [15:0] */;

//
// R1196 (0x4AC) - AIF1 DAC2 EQ Band 3 PG
//
pub const WM8995_AIF1DAC2_EQ_B3_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B3_PG - [15:0] */;

//
// R1197 (0x4AD) - AIF1 DAC2 EQ Band 4 A
//
pub const WM8995_AIF1DAC2_EQ_B4_A_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B4_A - [15:0] */;

//
// R1198 (0x4AE) - AIF1 DAC2 EQ Band 4 B
//
pub const WM8995_AIF1DAC2_EQ_B4_B_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B4_B - [15:0] */;

//
// R1199 (0x4AF) - AIF1 DAC2 EQ Band 4 C
//
pub const WM8995_AIF1DAC2_EQ_B4_C_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B4_C - [15:0] */;

//
// R1200 (0x4B0) - AIF1 DAC2 EQ Band 4 PG
//
pub const WM8995_AIF1DAC2_EQ_B4_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B4_PG - [15:0] */;

//
// R1201 (0x4B1) - AIF1 DAC2 EQ Band 5 A
//
pub const WM8995_AIF1DAC2_EQ_B5_A_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B5_A - [15:0] */;

//
// R1202 (0x4B2) - AIF1 DAC2 EQ Band 5 B
//
pub const WM8995_AIF1DAC2_EQ_B5_B_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B5_B - [15:0] */;

//
// R1203 (0x4B3) - AIF1 DAC2 EQ Band 5 PG
//
pub const WM8995_AIF1DAC2_EQ_B5_PG_MASK: c_uint = 0xFFFF	/* AIF1DAC2_EQ_B5_PG - [15:0] */;

//
// R1280 (0x500) - AIF2 ADC Left Volume
//
pub const WM8995_AIF2ADC_VU: c_uint = 0x0100	/* AIF2ADC_VU */;
pub const WM8995_AIF2ADC_VU_MASK: c_uint = 0x0100	/* AIF2ADC_VU */;

pub const WM8995_AIF2ADCL_VOL_MASK: c_uint = 0x00FF	/* AIF2ADCL_VOL - [7:0] */;

//
// R1281 (0x501) - AIF2 ADC Right Volume
//
pub const WM8995_AIF2ADC_VU: c_uint = 0x0100	/* AIF2ADC_VU */;
pub const WM8995_AIF2ADC_VU_MASK: c_uint = 0x0100	/* AIF2ADC_VU */;

pub const WM8995_AIF2ADCR_VOL_MASK: c_uint = 0x00FF	/* AIF2ADCR_VOL - [7:0] */;

//
// R1282 (0x502) - AIF2 DAC Left Volume
//
pub const WM8995_AIF2DAC_VU: c_uint = 0x0100	/* AIF2DAC_VU */;
pub const WM8995_AIF2DAC_VU_MASK: c_uint = 0x0100	/* AIF2DAC_VU */;

pub const WM8995_AIF2DACL_VOL_MASK: c_uint = 0x00FF	/* AIF2DACL_VOL - [7:0] */;

//
// R1283 (0x503) - AIF2 DAC Right Volume
//
pub const WM8995_AIF2DAC_VU: c_uint = 0x0100	/* AIF2DAC_VU */;
pub const WM8995_AIF2DAC_VU_MASK: c_uint = 0x0100	/* AIF2DAC_VU */;

pub const WM8995_AIF2DACR_VOL_MASK: c_uint = 0x00FF	/* AIF2DACR_VOL - [7:0] */;

//
// R1296 (0x510) - AIF2 ADC Filters
//
pub const WM8995_AIF2ADC_4FS: c_uint = 0x8000	/* AIF2ADC_4FS */;
pub const WM8995_AIF2ADC_4FS_MASK: c_uint = 0x8000	/* AIF2ADC_4FS */;

pub const WM8995_AIF2ADCL_HPF: c_uint = 0x1000	/* AIF2ADCL_HPF */;
pub const WM8995_AIF2ADCL_HPF_MASK: c_uint = 0x1000	/* AIF2ADCL_HPF */;

pub const WM8995_AIF2ADCR_HPF: c_uint = 0x0800	/* AIF2ADCR_HPF */;
pub const WM8995_AIF2ADCR_HPF_MASK: c_uint = 0x0800	/* AIF2ADCR_HPF */;

pub const WM8995_AIF2ADC_HPF_MODE: c_uint = 0x0008	/* AIF2ADC_HPF_MODE */;
pub const WM8995_AIF2ADC_HPF_MODE_MASK: c_uint = 0x0008	/* AIF2ADC_HPF_MODE */;

pub const WM8995_AIF2ADC_HPF_CUT_MASK: c_uint = 0x0007	/* AIF2ADC_HPF_CUT - [2:0] */;

//
// R1312 (0x520) - AIF2 DAC Filters (1)
//
pub const WM8995_AIF2DAC_MUTE: c_uint = 0x0200	/* AIF2DAC_MUTE */;
pub const WM8995_AIF2DAC_MUTE_MASK: c_uint = 0x0200	/* AIF2DAC_MUTE */;

pub const WM8995_AIF2DAC_MONO: c_uint = 0x0080	/* AIF2DAC_MONO */;
pub const WM8995_AIF2DAC_MONO_MASK: c_uint = 0x0080	/* AIF2DAC_MONO */;

pub const WM8995_AIF2DAC_MUTERATE: c_uint = 0x0020	/* AIF2DAC_MUTERATE */;
pub const WM8995_AIF2DAC_MUTERATE_MASK: c_uint = 0x0020	/* AIF2DAC_MUTERATE */;

pub const WM8995_AIF2DAC_UNMUTE_RAMP: c_uint = 0x0010	/* AIF2DAC_UNMUTE_RAMP */;
pub const WM8995_AIF2DAC_UNMUTE_RAMP_MASK: c_uint = 0x0010	/* AIF2DAC_UNMUTE_RAMP */;

pub const WM8995_AIF2DAC_DEEMP_MASK: c_uint = 0x0006	/* AIF2DAC_DEEMP - [2:1] */;

//
// R1313 (0x521) - AIF2 DAC Filters (2)
//
pub const WM8995_AIF2DAC_3D_GAIN_MASK: c_uint = 0x3E00	/* AIF2DAC_3D_GAIN - [13:9] */;

pub const WM8995_AIF2DAC_3D_ENA: c_uint = 0x0100	/* AIF2DAC_3D_ENA */;
pub const WM8995_AIF2DAC_3D_ENA_MASK: c_uint = 0x0100	/* AIF2DAC_3D_ENA */;

//
// R1344 (0x540) - AIF2 DRC (1)
//
pub const WM8995_AIF2DRC_SIG_DET_RMS_MASK: c_uint = 0xF800	/* AIF2DRC_SIG_DET_RMS - [15:11] */;

pub const WM8995_AIF2DRC_SIG_DET_PK_MASK: c_uint = 0x0600	/* AIF2DRC_SIG_DET_PK - [10:9] */;

pub const WM8995_AIF2DRC_NG_ENA: c_uint = 0x0100	/* AIF2DRC_NG_ENA */;
pub const WM8995_AIF2DRC_NG_ENA_MASK: c_uint = 0x0100	/* AIF2DRC_NG_ENA */;

pub const WM8995_AIF2DRC_SIG_DET_MODE: c_uint = 0x0080	/* AIF2DRC_SIG_DET_MODE */;
pub const WM8995_AIF2DRC_SIG_DET_MODE_MASK: c_uint = 0x0080	/* AIF2DRC_SIG_DET_MODE */;

pub const WM8995_AIF2DRC_SIG_DET: c_uint = 0x0040	/* AIF2DRC_SIG_DET */;
pub const WM8995_AIF2DRC_SIG_DET_MASK: c_uint = 0x0040	/* AIF2DRC_SIG_DET */;

pub const WM8995_AIF2DRC_KNEE2_OP_ENA: c_uint = 0x0020	/* AIF2DRC_KNEE2_OP_ENA */;
pub const WM8995_AIF2DRC_KNEE2_OP_ENA_MASK: c_uint = 0x0020	/* AIF2DRC_KNEE2_OP_ENA */;

pub const WM8995_AIF2DRC_QR: c_uint = 0x0010	/* AIF2DRC_QR */;
pub const WM8995_AIF2DRC_QR_MASK: c_uint = 0x0010	/* AIF2DRC_QR */;

pub const WM8995_AIF2DRC_ANTICLIP: c_uint = 0x0008	/* AIF2DRC_ANTICLIP */;
pub const WM8995_AIF2DRC_ANTICLIP_MASK: c_uint = 0x0008	/* AIF2DRC_ANTICLIP */;

pub const WM8995_AIF2DAC_DRC_ENA: c_uint = 0x0004	/* AIF2DAC_DRC_ENA */;
pub const WM8995_AIF2DAC_DRC_ENA_MASK: c_uint = 0x0004	/* AIF2DAC_DRC_ENA */;

pub const WM8995_AIF2ADCL_DRC_ENA: c_uint = 0x0002	/* AIF2ADCL_DRC_ENA */;
pub const WM8995_AIF2ADCL_DRC_ENA_MASK: c_uint = 0x0002	/* AIF2ADCL_DRC_ENA */;

pub const WM8995_AIF2ADCR_DRC_ENA: c_uint = 0x0001	/* AIF2ADCR_DRC_ENA */;
pub const WM8995_AIF2ADCR_DRC_ENA_MASK: c_uint = 0x0001	/* AIF2ADCR_DRC_ENA */;

//
// R1345 (0x541) - AIF2 DRC (2)
//
pub const WM8995_AIF2DRC_ATK_MASK: c_uint = 0x1E00	/* AIF2DRC_ATK - [12:9] */;

pub const WM8995_AIF2DRC_DCY_MASK: c_uint = 0x01E0	/* AIF2DRC_DCY - [8:5] */;

pub const WM8995_AIF2DRC_MINGAIN_MASK: c_uint = 0x001C	/* AIF2DRC_MINGAIN - [4:2] */;

pub const WM8995_AIF2DRC_MAXGAIN_MASK: c_uint = 0x0003	/* AIF2DRC_MAXGAIN - [1:0] */;

//
// R1346 (0x542) - AIF2 DRC (3)
//
pub const WM8995_AIF2DRC_NG_MINGAIN_MASK: c_uint = 0xF000	/* AIF2DRC_NG_MINGAIN - [15:12] */;

pub const WM8995_AIF2DRC_NG_EXP_MASK: c_uint = 0x0C00	/* AIF2DRC_NG_EXP - [11:10] */;

pub const WM8995_AIF2DRC_QR_THR_MASK: c_uint = 0x0300	/* AIF2DRC_QR_THR - [9:8] */;

pub const WM8995_AIF2DRC_QR_DCY_MASK: c_uint = 0x00C0	/* AIF2DRC_QR_DCY - [7:6] */;

pub const WM8995_AIF2DRC_HI_COMP_MASK: c_uint = 0x0038	/* AIF2DRC_HI_COMP - [5:3] */;

pub const WM8995_AIF2DRC_LO_COMP_MASK: c_uint = 0x0007	/* AIF2DRC_LO_COMP - [2:0] */;

//
// R1347 (0x543) - AIF2 DRC (4)
//
pub const WM8995_AIF2DRC_KNEE_IP_MASK: c_uint = 0x07E0	/* AIF2DRC_KNEE_IP - [10:5] */;

pub const WM8995_AIF2DRC_KNEE_OP_MASK: c_uint = 0x001F	/* AIF2DRC_KNEE_OP - [4:0] */;

//
// R1348 (0x544) - AIF2 DRC (5)
//
pub const WM8995_AIF2DRC_KNEE2_IP_MASK: c_uint = 0x03E0	/* AIF2DRC_KNEE2_IP - [9:5] */;

pub const WM8995_AIF2DRC_KNEE2_OP_MASK: c_uint = 0x001F	/* AIF2DRC_KNEE2_OP - [4:0] */;

//
// R1408 (0x580) - AIF2 EQ Gains (1)
//
pub const WM8995_AIF2DAC_EQ_B1_GAIN_MASK: c_uint = 0xF800	/* AIF2DAC_EQ_B1_GAIN - [15:11] */;

pub const WM8995_AIF2DAC_EQ_B2_GAIN_MASK: c_uint = 0x07C0	/* AIF2DAC_EQ_B2_GAIN - [10:6] */;

pub const WM8995_AIF2DAC_EQ_B3_GAIN_MASK: c_uint = 0x003E	/* AIF2DAC_EQ_B3_GAIN - [5:1] */;

pub const WM8995_AIF2DAC_EQ_ENA: c_uint = 0x0001	/* AIF2DAC_EQ_ENA */;
pub const WM8995_AIF2DAC_EQ_ENA_MASK: c_uint = 0x0001	/* AIF2DAC_EQ_ENA */;

//
// R1409 (0x581) - AIF2 EQ Gains (2)
//
pub const WM8995_AIF2DAC_EQ_B4_GAIN_MASK: c_uint = 0xF800	/* AIF2DAC_EQ_B4_GAIN - [15:11] */;

pub const WM8995_AIF2DAC_EQ_B5_GAIN_MASK: c_uint = 0x07C0	/* AIF2DAC_EQ_B5_GAIN - [10:6] */;

//
// R1410 (0x582) - AIF2 EQ Band 1 A
//
pub const WM8995_AIF2DAC_EQ_B1_A_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B1_A - [15:0] */;

//
// R1411 (0x583) - AIF2 EQ Band 1 B
//
pub const WM8995_AIF2DAC_EQ_B1_B_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B1_B - [15:0] */;

//
// R1412 (0x584) - AIF2 EQ Band 1 PG
//
pub const WM8995_AIF2DAC_EQ_B1_PG_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B1_PG - [15:0] */;

//
// R1413 (0x585) - AIF2 EQ Band 2 A
//
pub const WM8995_AIF2DAC_EQ_B2_A_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B2_A - [15:0] */;

//
// R1414 (0x586) - AIF2 EQ Band 2 B
//
pub const WM8995_AIF2DAC_EQ_B2_B_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B2_B - [15:0] */;

//
// R1415 (0x587) - AIF2 EQ Band 2 C
//
pub const WM8995_AIF2DAC_EQ_B2_C_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B2_C - [15:0] */;

//
// R1416 (0x588) - AIF2 EQ Band 2 PG
//
pub const WM8995_AIF2DAC_EQ_B2_PG_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B2_PG - [15:0] */;

//
// R1417 (0x589) - AIF2 EQ Band 3 A
//
pub const WM8995_AIF2DAC_EQ_B3_A_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B3_A - [15:0] */;

//
// R1418 (0x58A) - AIF2 EQ Band 3 B
//
pub const WM8995_AIF2DAC_EQ_B3_B_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B3_B - [15:0] */;

//
// R1419 (0x58B) - AIF2 EQ Band 3 C
//
pub const WM8995_AIF2DAC_EQ_B3_C_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B3_C - [15:0] */;

//
// R1420 (0x58C) - AIF2 EQ Band 3 PG
//
pub const WM8995_AIF2DAC_EQ_B3_PG_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B3_PG - [15:0] */;

//
// R1421 (0x58D) - AIF2 EQ Band 4 A
//
pub const WM8995_AIF2DAC_EQ_B4_A_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B4_A - [15:0] */;

//
// R1422 (0x58E) - AIF2 EQ Band 4 B
//
pub const WM8995_AIF2DAC_EQ_B4_B_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B4_B - [15:0] */;

//
// R1423 (0x58F) - AIF2 EQ Band 4 C
//
pub const WM8995_AIF2DAC_EQ_B4_C_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B4_C - [15:0] */;

//
// R1424 (0x590) - AIF2 EQ Band 4 PG
//
pub const WM8995_AIF2DAC_EQ_B4_PG_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B4_PG - [15:0] */;

//
// R1425 (0x591) - AIF2 EQ Band 5 A
//
pub const WM8995_AIF2DAC_EQ_B5_A_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B5_A - [15:0] */;

//
// R1426 (0x592) - AIF2 EQ Band 5 B
//
pub const WM8995_AIF2DAC_EQ_B5_B_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B5_B - [15:0] */;

//
// R1427 (0x593) - AIF2 EQ Band 5 PG
//
pub const WM8995_AIF2DAC_EQ_B5_PG_MASK: c_uint = 0xFFFF	/* AIF2DAC_EQ_B5_PG - [15:0] */;

//
// R1536 (0x600) - DAC1 Mixer Volumes
//
pub const WM8995_ADCR_DAC1_VOL_MASK: c_uint = 0x03E0	/* ADCR_DAC1_VOL - [9:5] */;

pub const WM8995_ADCL_DAC1_VOL_MASK: c_uint = 0x001F	/* ADCL_DAC1_VOL - [4:0] */;

//
// R1537 (0x601) - DAC1 Left Mixer Routing
//
pub const WM8995_ADCR_TO_DAC1L: c_uint = 0x0020	/* ADCR_TO_DAC1L */;
pub const WM8995_ADCR_TO_DAC1L_MASK: c_uint = 0x0020	/* ADCR_TO_DAC1L */;

pub const WM8995_ADCL_TO_DAC1L: c_uint = 0x0010	/* ADCL_TO_DAC1L */;
pub const WM8995_ADCL_TO_DAC1L_MASK: c_uint = 0x0010	/* ADCL_TO_DAC1L */;

pub const WM8995_AIF2DACL_TO_DAC1L: c_uint = 0x0004	/* AIF2DACL_TO_DAC1L */;
pub const WM8995_AIF2DACL_TO_DAC1L_MASK: c_uint = 0x0004	/* AIF2DACL_TO_DAC1L */;

pub const WM8995_AIF1DAC2L_TO_DAC1L: c_uint = 0x0002	/* AIF1DAC2L_TO_DAC1L */;
pub const WM8995_AIF1DAC2L_TO_DAC1L_MASK: c_uint = 0x0002	/* AIF1DAC2L_TO_DAC1L */;

pub const WM8995_AIF1DAC1L_TO_DAC1L: c_uint = 0x0001	/* AIF1DAC1L_TO_DAC1L */;
pub const WM8995_AIF1DAC1L_TO_DAC1L_MASK: c_uint = 0x0001	/* AIF1DAC1L_TO_DAC1L */;

//
// R1538 (0x602) - DAC1 Right Mixer Routing
//
pub const WM8995_ADCR_TO_DAC1R: c_uint = 0x0020	/* ADCR_TO_DAC1R */;
pub const WM8995_ADCR_TO_DAC1R_MASK: c_uint = 0x0020	/* ADCR_TO_DAC1R */;

pub const WM8995_ADCL_TO_DAC1R: c_uint = 0x0010	/* ADCL_TO_DAC1R */;
pub const WM8995_ADCL_TO_DAC1R_MASK: c_uint = 0x0010	/* ADCL_TO_DAC1R */;

pub const WM8995_AIF2DACR_TO_DAC1R: c_uint = 0x0004	/* AIF2DACR_TO_DAC1R */;
pub const WM8995_AIF2DACR_TO_DAC1R_MASK: c_uint = 0x0004	/* AIF2DACR_TO_DAC1R */;

pub const WM8995_AIF1DAC2R_TO_DAC1R: c_uint = 0x0002	/* AIF1DAC2R_TO_DAC1R */;
pub const WM8995_AIF1DAC2R_TO_DAC1R_MASK: c_uint = 0x0002	/* AIF1DAC2R_TO_DAC1R */;

pub const WM8995_AIF1DAC1R_TO_DAC1R: c_uint = 0x0001	/* AIF1DAC1R_TO_DAC1R */;
pub const WM8995_AIF1DAC1R_TO_DAC1R_MASK: c_uint = 0x0001	/* AIF1DAC1R_TO_DAC1R */;

//
// R1539 (0x603) - DAC2 Mixer Volumes
//
pub const WM8995_ADCR_DAC2_VOL_MASK: c_uint = 0x03E0	/* ADCR_DAC2_VOL - [9:5] */;

pub const WM8995_ADCL_DAC2_VOL_MASK: c_uint = 0x001F	/* ADCL_DAC2_VOL - [4:0] */;

//
// R1540 (0x604) - DAC2 Left Mixer Routing
//
pub const WM8995_ADCR_TO_DAC2L: c_uint = 0x0020	/* ADCR_TO_DAC2L */;
pub const WM8995_ADCR_TO_DAC2L_MASK: c_uint = 0x0020	/* ADCR_TO_DAC2L */;

pub const WM8995_ADCL_TO_DAC2L: c_uint = 0x0010	/* ADCL_TO_DAC2L */;
pub const WM8995_ADCL_TO_DAC2L_MASK: c_uint = 0x0010	/* ADCL_TO_DAC2L */;

pub const WM8995_AIF2DACL_TO_DAC2L: c_uint = 0x0004	/* AIF2DACL_TO_DAC2L */;
pub const WM8995_AIF2DACL_TO_DAC2L_MASK: c_uint = 0x0004	/* AIF2DACL_TO_DAC2L */;

pub const WM8995_AIF1DAC2L_TO_DAC2L: c_uint = 0x0002	/* AIF1DAC2L_TO_DAC2L */;
pub const WM8995_AIF1DAC2L_TO_DAC2L_MASK: c_uint = 0x0002	/* AIF1DAC2L_TO_DAC2L */;

pub const WM8995_AIF1DAC1L_TO_DAC2L: c_uint = 0x0001	/* AIF1DAC1L_TO_DAC2L */;
pub const WM8995_AIF1DAC1L_TO_DAC2L_MASK: c_uint = 0x0001	/* AIF1DAC1L_TO_DAC2L */;

//
// R1541 (0x605) - DAC2 Right Mixer Routing
//
pub const WM8995_ADCR_TO_DAC2R: c_uint = 0x0020	/* ADCR_TO_DAC2R */;
pub const WM8995_ADCR_TO_DAC2R_MASK: c_uint = 0x0020	/* ADCR_TO_DAC2R */;

pub const WM8995_ADCL_TO_DAC2R: c_uint = 0x0010	/* ADCL_TO_DAC2R */;
pub const WM8995_ADCL_TO_DAC2R_MASK: c_uint = 0x0010	/* ADCL_TO_DAC2R */;

pub const WM8995_AIF2DACR_TO_DAC2R: c_uint = 0x0004	/* AIF2DACR_TO_DAC2R */;
pub const WM8995_AIF2DACR_TO_DAC2R_MASK: c_uint = 0x0004	/* AIF2DACR_TO_DAC2R */;

pub const WM8995_AIF1DAC2R_TO_DAC2R: c_uint = 0x0002	/* AIF1DAC2R_TO_DAC2R */;
pub const WM8995_AIF1DAC2R_TO_DAC2R_MASK: c_uint = 0x0002	/* AIF1DAC2R_TO_DAC2R */;

pub const WM8995_AIF1DAC1R_TO_DAC2R: c_uint = 0x0001	/* AIF1DAC1R_TO_DAC2R */;
pub const WM8995_AIF1DAC1R_TO_DAC2R_MASK: c_uint = 0x0001	/* AIF1DAC1R_TO_DAC2R */;

//
// R1542 (0x606) - AIF1 ADC1 Left Mixer Routing
//
pub const WM8995_ADC1L_TO_AIF1ADC1L: c_uint = 0x0002	/* ADC1L_TO_AIF1ADC1L */;
pub const WM8995_ADC1L_TO_AIF1ADC1L_MASK: c_uint = 0x0002	/* ADC1L_TO_AIF1ADC1L */;

pub const WM8995_AIF2DACL_TO_AIF1ADC1L: c_uint = 0x0001	/* AIF2DACL_TO_AIF1ADC1L */;
pub const WM8995_AIF2DACL_TO_AIF1ADC1L_MASK: c_uint = 0x0001	/* AIF2DACL_TO_AIF1ADC1L */;

//
// R1543 (0x607) - AIF1 ADC1 Right Mixer Routing
//
pub const WM8995_ADC1R_TO_AIF1ADC1R: c_uint = 0x0002	/* ADC1R_TO_AIF1ADC1R */;
pub const WM8995_ADC1R_TO_AIF1ADC1R_MASK: c_uint = 0x0002	/* ADC1R_TO_AIF1ADC1R */;

pub const WM8995_AIF2DACR_TO_AIF1ADC1R: c_uint = 0x0001	/* AIF2DACR_TO_AIF1ADC1R */;
pub const WM8995_AIF2DACR_TO_AIF1ADC1R_MASK: c_uint = 0x0001	/* AIF2DACR_TO_AIF1ADC1R */;

//
// R1544 (0x608) - AIF1 ADC2 Left Mixer Routing
//
pub const WM8995_ADC2L_TO_AIF1ADC2L: c_uint = 0x0002	/* ADC2L_TO_AIF1ADC2L */;
pub const WM8995_ADC2L_TO_AIF1ADC2L_MASK: c_uint = 0x0002	/* ADC2L_TO_AIF1ADC2L */;

pub const WM8995_AIF2DACL_TO_AIF1ADC2L: c_uint = 0x0001	/* AIF2DACL_TO_AIF1ADC2L */;
pub const WM8995_AIF2DACL_TO_AIF1ADC2L_MASK: c_uint = 0x0001	/* AIF2DACL_TO_AIF1ADC2L */;

//
// R1545 (0x609) - AIF1 ADC2 Right mixer Routing
//
pub const WM8995_ADC2R_TO_AIF1ADC2R: c_uint = 0x0002	/* ADC2R_TO_AIF1ADC2R */;
pub const WM8995_ADC2R_TO_AIF1ADC2R_MASK: c_uint = 0x0002	/* ADC2R_TO_AIF1ADC2R */;

pub const WM8995_AIF2DACR_TO_AIF1ADC2R: c_uint = 0x0001	/* AIF2DACR_TO_AIF1ADC2R */;
pub const WM8995_AIF2DACR_TO_AIF1ADC2R_MASK: c_uint = 0x0001	/* AIF2DACR_TO_AIF1ADC2R */;

//
// R1552 (0x610) - DAC Softmute
//
pub const WM8995_DAC_SOFTMUTEMODE: c_uint = 0x0002	/* DAC_SOFTMUTEMODE */;
pub const WM8995_DAC_SOFTMUTEMODE_MASK: c_uint = 0x0002	/* DAC_SOFTMUTEMODE */;

pub const WM8995_DAC_MUTERATE: c_uint = 0x0001	/* DAC_MUTERATE */;
pub const WM8995_DAC_MUTERATE_MASK: c_uint = 0x0001	/* DAC_MUTERATE */;

//
// R1568 (0x620) - Oversampling
//
pub const WM8995_ADC_OSR128: c_uint = 0x0002	/* ADC_OSR128 */;
pub const WM8995_ADC_OSR128_MASK: c_uint = 0x0002	/* ADC_OSR128 */;

pub const WM8995_DAC_OSR128: c_uint = 0x0001	/* DAC_OSR128 */;
pub const WM8995_DAC_OSR128_MASK: c_uint = 0x0001	/* DAC_OSR128 */;

//
// R1569 (0x621) - Sidetone
//
pub const WM8995_ST_LPF: c_uint = 0x1000	/* ST_LPF */;
pub const WM8995_ST_LPF_MASK: c_uint = 0x1000	/* ST_LPF */;

pub const WM8995_ST_HPF_CUT_MASK: c_uint = 0x0380	/* ST_HPF_CUT - [9:7] */;

pub const WM8995_ST_HPF: c_uint = 0x0040	/* ST_HPF */;
pub const WM8995_ST_HPF_MASK: c_uint = 0x0040	/* ST_HPF */;

pub const WM8995_STR_SEL: c_uint = 0x0002	/* STR_SEL */;
pub const WM8995_STR_SEL_MASK: c_uint = 0x0002	/* STR_SEL */;

pub const WM8995_STL_SEL: c_uint = 0x0001	/* STL_SEL */;
pub const WM8995_STL_SEL_MASK: c_uint = 0x0001	/* STL_SEL */;

//
// R1792 (0x700) - GPIO 1
//
pub const WM8995_GP1_DIR: c_uint = 0x8000	/* GP1_DIR */;
pub const WM8995_GP1_DIR_MASK: c_uint = 0x8000	/* GP1_DIR */;

pub const WM8995_GP1_PU: c_uint = 0x4000	/* GP1_PU */;
pub const WM8995_GP1_PU_MASK: c_uint = 0x4000	/* GP1_PU */;

pub const WM8995_GP1_PD: c_uint = 0x2000	/* GP1_PD */;
pub const WM8995_GP1_PD_MASK: c_uint = 0x2000	/* GP1_PD */;

pub const WM8995_GP1_POL: c_uint = 0x0400	/* GP1_POL */;
pub const WM8995_GP1_POL_MASK: c_uint = 0x0400	/* GP1_POL */;

pub const WM8995_GP1_OP_CFG: c_uint = 0x0200	/* GP1_OP_CFG */;
pub const WM8995_GP1_OP_CFG_MASK: c_uint = 0x0200	/* GP1_OP_CFG */;

pub const WM8995_GP1_DB: c_uint = 0x0100	/* GP1_DB */;
pub const WM8995_GP1_DB_MASK: c_uint = 0x0100	/* GP1_DB */;

pub const WM8995_GP1_LVL: c_uint = 0x0040	/* GP1_LVL */;
pub const WM8995_GP1_LVL_MASK: c_uint = 0x0040	/* GP1_LVL */;

pub const WM8995_GP1_FN_MASK: c_uint = 0x001F	/* GP1_FN - [4:0] */;

//
// R1793 (0x701) - GPIO 2
//
pub const WM8995_GP2_DIR: c_uint = 0x8000	/* GP2_DIR */;
pub const WM8995_GP2_DIR_MASK: c_uint = 0x8000	/* GP2_DIR */;

pub const WM8995_GP2_PU: c_uint = 0x4000	/* GP2_PU */;
pub const WM8995_GP2_PU_MASK: c_uint = 0x4000	/* GP2_PU */;

pub const WM8995_GP2_PD: c_uint = 0x2000	/* GP2_PD */;
pub const WM8995_GP2_PD_MASK: c_uint = 0x2000	/* GP2_PD */;

pub const WM8995_GP2_POL: c_uint = 0x0400	/* GP2_POL */;
pub const WM8995_GP2_POL_MASK: c_uint = 0x0400	/* GP2_POL */;

pub const WM8995_GP2_OP_CFG: c_uint = 0x0200	/* GP2_OP_CFG */;
pub const WM8995_GP2_OP_CFG_MASK: c_uint = 0x0200	/* GP2_OP_CFG */;

pub const WM8995_GP2_DB: c_uint = 0x0100	/* GP2_DB */;
pub const WM8995_GP2_DB_MASK: c_uint = 0x0100	/* GP2_DB */;

pub const WM8995_GP2_LVL: c_uint = 0x0040	/* GP2_LVL */;
pub const WM8995_GP2_LVL_MASK: c_uint = 0x0040	/* GP2_LVL */;

pub const WM8995_GP2_FN_MASK: c_uint = 0x001F	/* GP2_FN - [4:0] */;

//
// R1794 (0x702) - GPIO 3
//
pub const WM8995_GP3_DIR: c_uint = 0x8000	/* GP3_DIR */;
pub const WM8995_GP3_DIR_MASK: c_uint = 0x8000	/* GP3_DIR */;

pub const WM8995_GP3_PU: c_uint = 0x4000	/* GP3_PU */;
pub const WM8995_GP3_PU_MASK: c_uint = 0x4000	/* GP3_PU */;

pub const WM8995_GP3_PD: c_uint = 0x2000	/* GP3_PD */;
pub const WM8995_GP3_PD_MASK: c_uint = 0x2000	/* GP3_PD */;

pub const WM8995_GP3_POL: c_uint = 0x0400	/* GP3_POL */;
pub const WM8995_GP3_POL_MASK: c_uint = 0x0400	/* GP3_POL */;

pub const WM8995_GP3_OP_CFG: c_uint = 0x0200	/* GP3_OP_CFG */;
pub const WM8995_GP3_OP_CFG_MASK: c_uint = 0x0200	/* GP3_OP_CFG */;

pub const WM8995_GP3_DB: c_uint = 0x0100	/* GP3_DB */;
pub const WM8995_GP3_DB_MASK: c_uint = 0x0100	/* GP3_DB */;

pub const WM8995_GP3_LVL: c_uint = 0x0040	/* GP3_LVL */;
pub const WM8995_GP3_LVL_MASK: c_uint = 0x0040	/* GP3_LVL */;

pub const WM8995_GP3_FN_MASK: c_uint = 0x001F	/* GP3_FN - [4:0] */;

//
// R1795 (0x703) - GPIO 4
//
pub const WM8995_GP4_DIR: c_uint = 0x8000	/* GP4_DIR */;
pub const WM8995_GP4_DIR_MASK: c_uint = 0x8000	/* GP4_DIR */;

pub const WM8995_GP4_PU: c_uint = 0x4000	/* GP4_PU */;
pub const WM8995_GP4_PU_MASK: c_uint = 0x4000	/* GP4_PU */;

pub const WM8995_GP4_PD: c_uint = 0x2000	/* GP4_PD */;
pub const WM8995_GP4_PD_MASK: c_uint = 0x2000	/* GP4_PD */;

pub const WM8995_GP4_POL: c_uint = 0x0400	/* GP4_POL */;
pub const WM8995_GP4_POL_MASK: c_uint = 0x0400	/* GP4_POL */;

pub const WM8995_GP4_OP_CFG: c_uint = 0x0200	/* GP4_OP_CFG */;
pub const WM8995_GP4_OP_CFG_MASK: c_uint = 0x0200	/* GP4_OP_CFG */;

pub const WM8995_GP4_DB: c_uint = 0x0100	/* GP4_DB */;
pub const WM8995_GP4_DB_MASK: c_uint = 0x0100	/* GP4_DB */;

pub const WM8995_GP4_LVL: c_uint = 0x0040	/* GP4_LVL */;
pub const WM8995_GP4_LVL_MASK: c_uint = 0x0040	/* GP4_LVL */;

pub const WM8995_GP4_FN_MASK: c_uint = 0x001F	/* GP4_FN - [4:0] */;

//
// R1796 (0x704) - GPIO 5
//
pub const WM8995_GP5_DIR: c_uint = 0x8000	/* GP5_DIR */;
pub const WM8995_GP5_DIR_MASK: c_uint = 0x8000	/* GP5_DIR */;

pub const WM8995_GP5_PU: c_uint = 0x4000	/* GP5_PU */;
pub const WM8995_GP5_PU_MASK: c_uint = 0x4000	/* GP5_PU */;

pub const WM8995_GP5_PD: c_uint = 0x2000	/* GP5_PD */;
pub const WM8995_GP5_PD_MASK: c_uint = 0x2000	/* GP5_PD */;

pub const WM8995_GP5_POL: c_uint = 0x0400	/* GP5_POL */;
pub const WM8995_GP5_POL_MASK: c_uint = 0x0400	/* GP5_POL */;

pub const WM8995_GP5_OP_CFG: c_uint = 0x0200	/* GP5_OP_CFG */;
pub const WM8995_GP5_OP_CFG_MASK: c_uint = 0x0200	/* GP5_OP_CFG */;

pub const WM8995_GP5_DB: c_uint = 0x0100	/* GP5_DB */;
pub const WM8995_GP5_DB_MASK: c_uint = 0x0100	/* GP5_DB */;

pub const WM8995_GP5_LVL: c_uint = 0x0040	/* GP5_LVL */;
pub const WM8995_GP5_LVL_MASK: c_uint = 0x0040	/* GP5_LVL */;

pub const WM8995_GP5_FN_MASK: c_uint = 0x001F	/* GP5_FN - [4:0] */;

//
// R1797 (0x705) - GPIO 6
//
pub const WM8995_GP6_DIR: c_uint = 0x8000	/* GP6_DIR */;
pub const WM8995_GP6_DIR_MASK: c_uint = 0x8000	/* GP6_DIR */;

pub const WM8995_GP6_PU: c_uint = 0x4000	/* GP6_PU */;
pub const WM8995_GP6_PU_MASK: c_uint = 0x4000	/* GP6_PU */;

pub const WM8995_GP6_PD: c_uint = 0x2000	/* GP6_PD */;
pub const WM8995_GP6_PD_MASK: c_uint = 0x2000	/* GP6_PD */;

pub const WM8995_GP6_POL: c_uint = 0x0400	/* GP6_POL */;
pub const WM8995_GP6_POL_MASK: c_uint = 0x0400	/* GP6_POL */;

pub const WM8995_GP6_OP_CFG: c_uint = 0x0200	/* GP6_OP_CFG */;
pub const WM8995_GP6_OP_CFG_MASK: c_uint = 0x0200	/* GP6_OP_CFG */;

pub const WM8995_GP6_DB: c_uint = 0x0100	/* GP6_DB */;
pub const WM8995_GP6_DB_MASK: c_uint = 0x0100	/* GP6_DB */;

pub const WM8995_GP6_LVL: c_uint = 0x0040	/* GP6_LVL */;
pub const WM8995_GP6_LVL_MASK: c_uint = 0x0040	/* GP6_LVL */;

pub const WM8995_GP6_FN_MASK: c_uint = 0x001F	/* GP6_FN - [4:0] */;

//
// R1798 (0x706) - GPIO 7
//
pub const WM8995_GP7_DIR: c_uint = 0x8000	/* GP7_DIR */;
pub const WM8995_GP7_DIR_MASK: c_uint = 0x8000	/* GP7_DIR */;

pub const WM8995_GP7_PU: c_uint = 0x4000	/* GP7_PU */;
pub const WM8995_GP7_PU_MASK: c_uint = 0x4000	/* GP7_PU */;

pub const WM8995_GP7_PD: c_uint = 0x2000	/* GP7_PD */;
pub const WM8995_GP7_PD_MASK: c_uint = 0x2000	/* GP7_PD */;

pub const WM8995_GP7_POL: c_uint = 0x0400	/* GP7_POL */;
pub const WM8995_GP7_POL_MASK: c_uint = 0x0400	/* GP7_POL */;

pub const WM8995_GP7_OP_CFG: c_uint = 0x0200	/* GP7_OP_CFG */;
pub const WM8995_GP7_OP_CFG_MASK: c_uint = 0x0200	/* GP7_OP_CFG */;

pub const WM8995_GP7_DB: c_uint = 0x0100	/* GP7_DB */;
pub const WM8995_GP7_DB_MASK: c_uint = 0x0100	/* GP7_DB */;

pub const WM8995_GP7_LVL: c_uint = 0x0040	/* GP7_LVL */;
pub const WM8995_GP7_LVL_MASK: c_uint = 0x0040	/* GP7_LVL */;

pub const WM8995_GP7_FN_MASK: c_uint = 0x001F	/* GP7_FN - [4:0] */;

//
// R1799 (0x707) - GPIO 8
//
pub const WM8995_GP8_DIR: c_uint = 0x8000	/* GP8_DIR */;
pub const WM8995_GP8_DIR_MASK: c_uint = 0x8000	/* GP8_DIR */;

pub const WM8995_GP8_PU: c_uint = 0x4000	/* GP8_PU */;
pub const WM8995_GP8_PU_MASK: c_uint = 0x4000	/* GP8_PU */;

pub const WM8995_GP8_PD: c_uint = 0x2000	/* GP8_PD */;
pub const WM8995_GP8_PD_MASK: c_uint = 0x2000	/* GP8_PD */;

pub const WM8995_GP8_POL: c_uint = 0x0400	/* GP8_POL */;
pub const WM8995_GP8_POL_MASK: c_uint = 0x0400	/* GP8_POL */;

pub const WM8995_GP8_OP_CFG: c_uint = 0x0200	/* GP8_OP_CFG */;
pub const WM8995_GP8_OP_CFG_MASK: c_uint = 0x0200	/* GP8_OP_CFG */;

pub const WM8995_GP8_DB: c_uint = 0x0100	/* GP8_DB */;
pub const WM8995_GP8_DB_MASK: c_uint = 0x0100	/* GP8_DB */;

pub const WM8995_GP8_LVL: c_uint = 0x0040	/* GP8_LVL */;
pub const WM8995_GP8_LVL_MASK: c_uint = 0x0040	/* GP8_LVL */;

pub const WM8995_GP8_FN_MASK: c_uint = 0x001F	/* GP8_FN - [4:0] */;

//
// R1800 (0x708) - GPIO 9
//
pub const WM8995_GP9_DIR: c_uint = 0x8000	/* GP9_DIR */;
pub const WM8995_GP9_DIR_MASK: c_uint = 0x8000	/* GP9_DIR */;

pub const WM8995_GP9_PU: c_uint = 0x4000	/* GP9_PU */;
pub const WM8995_GP9_PU_MASK: c_uint = 0x4000	/* GP9_PU */;

pub const WM8995_GP9_PD: c_uint = 0x2000	/* GP9_PD */;
pub const WM8995_GP9_PD_MASK: c_uint = 0x2000	/* GP9_PD */;

pub const WM8995_GP9_POL: c_uint = 0x0400	/* GP9_POL */;
pub const WM8995_GP9_POL_MASK: c_uint = 0x0400	/* GP9_POL */;

pub const WM8995_GP9_OP_CFG: c_uint = 0x0200	/* GP9_OP_CFG */;
pub const WM8995_GP9_OP_CFG_MASK: c_uint = 0x0200	/* GP9_OP_CFG */;

pub const WM8995_GP9_DB: c_uint = 0x0100	/* GP9_DB */;
pub const WM8995_GP9_DB_MASK: c_uint = 0x0100	/* GP9_DB */;

pub const WM8995_GP9_LVL: c_uint = 0x0040	/* GP9_LVL */;
pub const WM8995_GP9_LVL_MASK: c_uint = 0x0040	/* GP9_LVL */;

pub const WM8995_GP9_FN_MASK: c_uint = 0x001F	/* GP9_FN - [4:0] */;

//
// R1801 (0x709) - GPIO 10
//
pub const WM8995_GP10_DIR: c_uint = 0x8000	/* GP10_DIR */;
pub const WM8995_GP10_DIR_MASK: c_uint = 0x8000	/* GP10_DIR */;

pub const WM8995_GP10_PU: c_uint = 0x4000	/* GP10_PU */;
pub const WM8995_GP10_PU_MASK: c_uint = 0x4000	/* GP10_PU */;

pub const WM8995_GP10_PD: c_uint = 0x2000	/* GP10_PD */;
pub const WM8995_GP10_PD_MASK: c_uint = 0x2000	/* GP10_PD */;

pub const WM8995_GP10_POL: c_uint = 0x0400	/* GP10_POL */;
pub const WM8995_GP10_POL_MASK: c_uint = 0x0400	/* GP10_POL */;

pub const WM8995_GP10_OP_CFG: c_uint = 0x0200	/* GP10_OP_CFG */;
pub const WM8995_GP10_OP_CFG_MASK: c_uint = 0x0200	/* GP10_OP_CFG */;

pub const WM8995_GP10_DB: c_uint = 0x0100	/* GP10_DB */;
pub const WM8995_GP10_DB_MASK: c_uint = 0x0100	/* GP10_DB */;

pub const WM8995_GP10_LVL: c_uint = 0x0040	/* GP10_LVL */;
pub const WM8995_GP10_LVL_MASK: c_uint = 0x0040	/* GP10_LVL */;

pub const WM8995_GP10_FN_MASK: c_uint = 0x001F	/* GP10_FN - [4:0] */;

//
// R1802 (0x70A) - GPIO 11
//
pub const WM8995_GP11_DIR: c_uint = 0x8000	/* GP11_DIR */;
pub const WM8995_GP11_DIR_MASK: c_uint = 0x8000	/* GP11_DIR */;

pub const WM8995_GP11_PU: c_uint = 0x4000	/* GP11_PU */;
pub const WM8995_GP11_PU_MASK: c_uint = 0x4000	/* GP11_PU */;

pub const WM8995_GP11_PD: c_uint = 0x2000	/* GP11_PD */;
pub const WM8995_GP11_PD_MASK: c_uint = 0x2000	/* GP11_PD */;

pub const WM8995_GP11_POL: c_uint = 0x0400	/* GP11_POL */;
pub const WM8995_GP11_POL_MASK: c_uint = 0x0400	/* GP11_POL */;

pub const WM8995_GP11_OP_CFG: c_uint = 0x0200	/* GP11_OP_CFG */;
pub const WM8995_GP11_OP_CFG_MASK: c_uint = 0x0200	/* GP11_OP_CFG */;

pub const WM8995_GP11_DB: c_uint = 0x0100	/* GP11_DB */;
pub const WM8995_GP11_DB_MASK: c_uint = 0x0100	/* GP11_DB */;

pub const WM8995_GP11_LVL: c_uint = 0x0040	/* GP11_LVL */;
pub const WM8995_GP11_LVL_MASK: c_uint = 0x0040	/* GP11_LVL */;

pub const WM8995_GP11_FN_MASK: c_uint = 0x001F	/* GP11_FN - [4:0] */;

//
// R1803 (0x70B) - GPIO 12
//
pub const WM8995_GP12_DIR: c_uint = 0x8000	/* GP12_DIR */;
pub const WM8995_GP12_DIR_MASK: c_uint = 0x8000	/* GP12_DIR */;

pub const WM8995_GP12_PU: c_uint = 0x4000	/* GP12_PU */;
pub const WM8995_GP12_PU_MASK: c_uint = 0x4000	/* GP12_PU */;

pub const WM8995_GP12_PD: c_uint = 0x2000	/* GP12_PD */;
pub const WM8995_GP12_PD_MASK: c_uint = 0x2000	/* GP12_PD */;

pub const WM8995_GP12_POL: c_uint = 0x0400	/* GP12_POL */;
pub const WM8995_GP12_POL_MASK: c_uint = 0x0400	/* GP12_POL */;

pub const WM8995_GP12_OP_CFG: c_uint = 0x0200	/* GP12_OP_CFG */;
pub const WM8995_GP12_OP_CFG_MASK: c_uint = 0x0200	/* GP12_OP_CFG */;

pub const WM8995_GP12_DB: c_uint = 0x0100	/* GP12_DB */;
pub const WM8995_GP12_DB_MASK: c_uint = 0x0100	/* GP12_DB */;

pub const WM8995_GP12_LVL: c_uint = 0x0040	/* GP12_LVL */;
pub const WM8995_GP12_LVL_MASK: c_uint = 0x0040	/* GP12_LVL */;

pub const WM8995_GP12_FN_MASK: c_uint = 0x001F	/* GP12_FN - [4:0] */;

//
// R1804 (0x70C) - GPIO 13
//
pub const WM8995_GP13_DIR: c_uint = 0x8000	/* GP13_DIR */;
pub const WM8995_GP13_DIR_MASK: c_uint = 0x8000	/* GP13_DIR */;

pub const WM8995_GP13_PU: c_uint = 0x4000	/* GP13_PU */;
pub const WM8995_GP13_PU_MASK: c_uint = 0x4000	/* GP13_PU */;

pub const WM8995_GP13_PD: c_uint = 0x2000	/* GP13_PD */;
pub const WM8995_GP13_PD_MASK: c_uint = 0x2000	/* GP13_PD */;

pub const WM8995_GP13_POL: c_uint = 0x0400	/* GP13_POL */;
pub const WM8995_GP13_POL_MASK: c_uint = 0x0400	/* GP13_POL */;

pub const WM8995_GP13_OP_CFG: c_uint = 0x0200	/* GP13_OP_CFG */;
pub const WM8995_GP13_OP_CFG_MASK: c_uint = 0x0200	/* GP13_OP_CFG */;

pub const WM8995_GP13_DB: c_uint = 0x0100	/* GP13_DB */;
pub const WM8995_GP13_DB_MASK: c_uint = 0x0100	/* GP13_DB */;

pub const WM8995_GP13_LVL: c_uint = 0x0040	/* GP13_LVL */;
pub const WM8995_GP13_LVL_MASK: c_uint = 0x0040	/* GP13_LVL */;

pub const WM8995_GP13_FN_MASK: c_uint = 0x001F	/* GP13_FN - [4:0] */;

//
// R1805 (0x70D) - GPIO 14
//
pub const WM8995_GP14_DIR: c_uint = 0x8000	/* GP14_DIR */;
pub const WM8995_GP14_DIR_MASK: c_uint = 0x8000	/* GP14_DIR */;

pub const WM8995_GP14_PU: c_uint = 0x4000	/* GP14_PU */;
pub const WM8995_GP14_PU_MASK: c_uint = 0x4000	/* GP14_PU */;

pub const WM8995_GP14_PD: c_uint = 0x2000	/* GP14_PD */;
pub const WM8995_GP14_PD_MASK: c_uint = 0x2000	/* GP14_PD */;

pub const WM8995_GP14_POL: c_uint = 0x0400	/* GP14_POL */;
pub const WM8995_GP14_POL_MASK: c_uint = 0x0400	/* GP14_POL */;

pub const WM8995_GP14_OP_CFG: c_uint = 0x0200	/* GP14_OP_CFG */;
pub const WM8995_GP14_OP_CFG_MASK: c_uint = 0x0200	/* GP14_OP_CFG */;

pub const WM8995_GP14_DB: c_uint = 0x0100	/* GP14_DB */;
pub const WM8995_GP14_DB_MASK: c_uint = 0x0100	/* GP14_DB */;

pub const WM8995_GP14_LVL: c_uint = 0x0040	/* GP14_LVL */;
pub const WM8995_GP14_LVL_MASK: c_uint = 0x0040	/* GP14_LVL */;

pub const WM8995_GP14_FN_MASK: c_uint = 0x001F	/* GP14_FN - [4:0] */;

//
// R1824 (0x720) - Pull Control (1)
//
pub const WM8995_DMICDAT3_PD: c_uint = 0x4000	/* DMICDAT3_PD */;
pub const WM8995_DMICDAT3_PD_MASK: c_uint = 0x4000	/* DMICDAT3_PD */;

pub const WM8995_DMICDAT2_PD: c_uint = 0x1000	/* DMICDAT2_PD */;
pub const WM8995_DMICDAT2_PD_MASK: c_uint = 0x1000	/* DMICDAT2_PD */;

pub const WM8995_DMICDAT1_PD: c_uint = 0x0400	/* DMICDAT1_PD */;
pub const WM8995_DMICDAT1_PD_MASK: c_uint = 0x0400	/* DMICDAT1_PD */;

pub const WM8995_MCLK2_PU: c_uint = 0x0200	/* MCLK2_PU */;
pub const WM8995_MCLK2_PU_MASK: c_uint = 0x0200	/* MCLK2_PU */;

pub const WM8995_MCLK2_PD: c_uint = 0x0100	/* MCLK2_PD */;
pub const WM8995_MCLK2_PD_MASK: c_uint = 0x0100	/* MCLK2_PD */;

pub const WM8995_MCLK1_PU: c_uint = 0x0080	/* MCLK1_PU */;
pub const WM8995_MCLK1_PU_MASK: c_uint = 0x0080	/* MCLK1_PU */;

pub const WM8995_MCLK1_PD: c_uint = 0x0040	/* MCLK1_PD */;
pub const WM8995_MCLK1_PD_MASK: c_uint = 0x0040	/* MCLK1_PD */;

pub const WM8995_DACDAT1_PU: c_uint = 0x0020	/* DACDAT1_PU */;
pub const WM8995_DACDAT1_PU_MASK: c_uint = 0x0020	/* DACDAT1_PU */;

pub const WM8995_DACDAT1_PD: c_uint = 0x0010	/* DACDAT1_PD */;
pub const WM8995_DACDAT1_PD_MASK: c_uint = 0x0010	/* DACDAT1_PD */;

pub const WM8995_DACLRCLK1_PU: c_uint = 0x0008	/* DACLRCLK1_PU */;
pub const WM8995_DACLRCLK1_PU_MASK: c_uint = 0x0008	/* DACLRCLK1_PU */;

pub const WM8995_DACLRCLK1_PD: c_uint = 0x0004	/* DACLRCLK1_PD */;
pub const WM8995_DACLRCLK1_PD_MASK: c_uint = 0x0004	/* DACLRCLK1_PD */;

pub const WM8995_BCLK1_PU: c_uint = 0x0002	/* BCLK1_PU */;
pub const WM8995_BCLK1_PU_MASK: c_uint = 0x0002	/* BCLK1_PU */;

pub const WM8995_BCLK1_PD: c_uint = 0x0001	/* BCLK1_PD */;
pub const WM8995_BCLK1_PD_MASK: c_uint = 0x0001	/* BCLK1_PD */;

//
// R1825 (0x721) - Pull Control (2)
//
pub const WM8995_LDO1ENA_PD: c_uint = 0x0010	/* LDO1ENA_PD */;
pub const WM8995_LDO1ENA_PD_MASK: c_uint = 0x0010	/* LDO1ENA_PD */;

pub const WM8995_MODE_PD: c_uint = 0x0004	/* MODE_PD */;
pub const WM8995_MODE_PD_MASK: c_uint = 0x0004	/* MODE_PD */;

pub const WM8995_CSNADDR_PD: c_uint = 0x0001	/* CSNADDR_PD */;
pub const WM8995_CSNADDR_PD_MASK: c_uint = 0x0001	/* CSNADDR_PD */;

//
// R1840 (0x730) - Interrupt Status 1
//
pub const WM8995_GP14_EINT: c_uint = 0x2000	/* GP14_EINT */;
pub const WM8995_GP14_EINT_MASK: c_uint = 0x2000	/* GP14_EINT */;

pub const WM8995_GP13_EINT: c_uint = 0x1000	/* GP13_EINT */;
pub const WM8995_GP13_EINT_MASK: c_uint = 0x1000	/* GP13_EINT */;

pub const WM8995_GP12_EINT: c_uint = 0x0800	/* GP12_EINT */;
pub const WM8995_GP12_EINT_MASK: c_uint = 0x0800	/* GP12_EINT */;

pub const WM8995_GP11_EINT: c_uint = 0x0400	/* GP11_EINT */;
pub const WM8995_GP11_EINT_MASK: c_uint = 0x0400	/* GP11_EINT */;

pub const WM8995_GP10_EINT: c_uint = 0x0200	/* GP10_EINT */;
pub const WM8995_GP10_EINT_MASK: c_uint = 0x0200	/* GP10_EINT */;

pub const WM8995_GP9_EINT: c_uint = 0x0100	/* GP9_EINT */;
pub const WM8995_GP9_EINT_MASK: c_uint = 0x0100	/* GP9_EINT */;

pub const WM8995_GP8_EINT: c_uint = 0x0080	/* GP8_EINT */;
pub const WM8995_GP8_EINT_MASK: c_uint = 0x0080	/* GP8_EINT */;

pub const WM8995_GP7_EINT: c_uint = 0x0040	/* GP7_EINT */;
pub const WM8995_GP7_EINT_MASK: c_uint = 0x0040	/* GP7_EINT */;

pub const WM8995_GP6_EINT: c_uint = 0x0020	/* GP6_EINT */;
pub const WM8995_GP6_EINT_MASK: c_uint = 0x0020	/* GP6_EINT */;

pub const WM8995_GP5_EINT: c_uint = 0x0010	/* GP5_EINT */;
pub const WM8995_GP5_EINT_MASK: c_uint = 0x0010	/* GP5_EINT */;

pub const WM8995_GP4_EINT: c_uint = 0x0008	/* GP4_EINT */;
pub const WM8995_GP4_EINT_MASK: c_uint = 0x0008	/* GP4_EINT */;

pub const WM8995_GP3_EINT: c_uint = 0x0004	/* GP3_EINT */;
pub const WM8995_GP3_EINT_MASK: c_uint = 0x0004	/* GP3_EINT */;

pub const WM8995_GP2_EINT: c_uint = 0x0002	/* GP2_EINT */;
pub const WM8995_GP2_EINT_MASK: c_uint = 0x0002	/* GP2_EINT */;

pub const WM8995_GP1_EINT: c_uint = 0x0001	/* GP1_EINT */;
pub const WM8995_GP1_EINT_MASK: c_uint = 0x0001	/* GP1_EINT */;

//
// R1841 (0x731) - Interrupt Status 2
//
pub const WM8995_DCS_DONE_23_EINT: c_uint = 0x1000	/* DCS_DONE_23_EINT */;
pub const WM8995_DCS_DONE_23_EINT_MASK: c_uint = 0x1000	/* DCS_DONE_23_EINT */;

pub const WM8995_DCS_DONE_01_EINT: c_uint = 0x0800	/* DCS_DONE_01_EINT */;
pub const WM8995_DCS_DONE_01_EINT_MASK: c_uint = 0x0800	/* DCS_DONE_01_EINT */;

pub const WM8995_WSEQ_DONE_EINT: c_uint = 0x0400	/* WSEQ_DONE_EINT */;
pub const WM8995_WSEQ_DONE_EINT_MASK: c_uint = 0x0400	/* WSEQ_DONE_EINT */;

pub const WM8995_FIFOS_ERR_EINT: c_uint = 0x0200	/* FIFOS_ERR_EINT */;
pub const WM8995_FIFOS_ERR_EINT_MASK: c_uint = 0x0200	/* FIFOS_ERR_EINT */;

pub const WM8995_AIF2DRC_SIG_DET_EINT: c_uint = 0x0100	/* AIF2DRC_SIG_DET_EINT */;
pub const WM8995_AIF2DRC_SIG_DET_EINT_MASK: c_uint = 0x0100	/* AIF2DRC_SIG_DET_EINT */;

pub const WM8995_AIF1DRC2_SIG_DET_EINT: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_EINT */;
pub const WM8995_AIF1DRC2_SIG_DET_EINT_MASK: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_EINT */;

pub const WM8995_AIF1DRC1_SIG_DET_EINT: c_uint = 0x0040	/* AIF1DRC1_SIG_DET_EINT */;
pub const WM8995_AIF1DRC1_SIG_DET_EINT_MASK: c_uint = 0x0040	/* AIF1DRC1_SIG_DET_EINT */;

pub const WM8995_SRC2_LOCK_EINT: c_uint = 0x0020	/* SRC2_LOCK_EINT */;
pub const WM8995_SRC2_LOCK_EINT_MASK: c_uint = 0x0020	/* SRC2_LOCK_EINT */;

pub const WM8995_SRC1_LOCK_EINT: c_uint = 0x0010	/* SRC1_LOCK_EINT */;
pub const WM8995_SRC1_LOCK_EINT_MASK: c_uint = 0x0010	/* SRC1_LOCK_EINT */;

pub const WM8995_FLL2_LOCK_EINT: c_uint = 0x0008	/* FLL2_LOCK_EINT */;
pub const WM8995_FLL2_LOCK_EINT_MASK: c_uint = 0x0008	/* FLL2_LOCK_EINT */;

pub const WM8995_FLL1_LOCK_EINT: c_uint = 0x0004	/* FLL1_LOCK_EINT */;
pub const WM8995_FLL1_LOCK_EINT_MASK: c_uint = 0x0004	/* FLL1_LOCK_EINT */;

pub const WM8995_HP_DONE_EINT: c_uint = 0x0002	/* HP_DONE_EINT */;
pub const WM8995_HP_DONE_EINT_MASK: c_uint = 0x0002	/* HP_DONE_EINT */;

pub const WM8995_MICD_EINT: c_uint = 0x0001	/* MICD_EINT */;
pub const WM8995_MICD_EINT_MASK: c_uint = 0x0001	/* MICD_EINT */;

//
// R1842 (0x732) - Interrupt Raw Status 2
//
pub const WM8995_DCS_DONE_23_STS: c_uint = 0x1000	/* DCS_DONE_23_STS */;
pub const WM8995_DCS_DONE_23_STS_MASK: c_uint = 0x1000	/* DCS_DONE_23_STS */;

pub const WM8995_DCS_DONE_01_STS: c_uint = 0x0800	/* DCS_DONE_01_STS */;
pub const WM8995_DCS_DONE_01_STS_MASK: c_uint = 0x0800	/* DCS_DONE_01_STS */;

pub const WM8995_WSEQ_DONE_STS: c_uint = 0x0400	/* WSEQ_DONE_STS */;
pub const WM8995_WSEQ_DONE_STS_MASK: c_uint = 0x0400	/* WSEQ_DONE_STS */;

pub const WM8995_FIFOS_ERR_STS: c_uint = 0x0200	/* FIFOS_ERR_STS */;
pub const WM8995_FIFOS_ERR_STS_MASK: c_uint = 0x0200	/* FIFOS_ERR_STS */;

pub const WM8995_AIF2DRC_SIG_DET_STS: c_uint = 0x0100	/* AIF2DRC_SIG_DET_STS */;
pub const WM8995_AIF2DRC_SIG_DET_STS_MASK: c_uint = 0x0100	/* AIF2DRC_SIG_DET_STS */;

pub const WM8995_AIF1DRC2_SIG_DET_STS: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_STS */;
pub const WM8995_AIF1DRC2_SIG_DET_STS_MASK: c_uint = 0x0080	/* AIF1DRC2_SIG_DET_STS */;

pub const WM8995_AIF1DRC1_SIG_DET_STS: c_uint = 0x0040	/* AIF1DRC1_SIG_DET_STS */;
pub const WM8995_AIF1DRC1_SIG_DET_STS_MASK: c_uint = 0x0040	/* AIF1DRC1_SIG_DET_STS */;

pub const WM8995_SRC2_LOCK_STS: c_uint = 0x0020	/* SRC2_LOCK_STS */;
pub const WM8995_SRC2_LOCK_STS_MASK: c_uint = 0x0020	/* SRC2_LOCK_STS */;

pub const WM8995_SRC1_LOCK_STS: c_uint = 0x0010	/* SRC1_LOCK_STS */;
pub const WM8995_SRC1_LOCK_STS_MASK: c_uint = 0x0010	/* SRC1_LOCK_STS */;

pub const WM8995_FLL2_LOCK_STS: c_uint = 0x0008	/* FLL2_LOCK_STS */;
pub const WM8995_FLL2_LOCK_STS_MASK: c_uint = 0x0008	/* FLL2_LOCK_STS */;

pub const WM8995_FLL1_LOCK_STS: c_uint = 0x0004	/* FLL1_LOCK_STS */;
pub const WM8995_FLL1_LOCK_STS_MASK: c_uint = 0x0004	/* FLL1_LOCK_STS */;

//
// R1848 (0x738) - Interrupt Status 1 Mask
//
pub const WM8995_IM_GP14_EINT: c_uint = 0x2000	/* IM_GP14_EINT */;
pub const WM8995_IM_GP14_EINT_MASK: c_uint = 0x2000	/* IM_GP14_EINT */;

pub const WM8995_IM_GP13_EINT: c_uint = 0x1000	/* IM_GP13_EINT */;
pub const WM8995_IM_GP13_EINT_MASK: c_uint = 0x1000	/* IM_GP13_EINT */;

pub const WM8995_IM_GP12_EINT: c_uint = 0x0800	/* IM_GP12_EINT */;
pub const WM8995_IM_GP12_EINT_MASK: c_uint = 0x0800	/* IM_GP12_EINT */;

pub const WM8995_IM_GP11_EINT: c_uint = 0x0400	/* IM_GP11_EINT */;
pub const WM8995_IM_GP11_EINT_MASK: c_uint = 0x0400	/* IM_GP11_EINT */;

pub const WM8995_IM_GP10_EINT: c_uint = 0x0200	/* IM_GP10_EINT */;
pub const WM8995_IM_GP10_EINT_MASK: c_uint = 0x0200	/* IM_GP10_EINT */;

pub const WM8995_IM_GP9_EINT: c_uint = 0x0100	/* IM_GP9_EINT */;
pub const WM8995_IM_GP9_EINT_MASK: c_uint = 0x0100	/* IM_GP9_EINT */;

pub const WM8995_IM_GP8_EINT: c_uint = 0x0080	/* IM_GP8_EINT */;
pub const WM8995_IM_GP8_EINT_MASK: c_uint = 0x0080	/* IM_GP8_EINT */;

pub const WM8995_IM_GP7_EINT: c_uint = 0x0040	/* IM_GP7_EINT */;
pub const WM8995_IM_GP7_EINT_MASK: c_uint = 0x0040	/* IM_GP7_EINT */;

pub const WM8995_IM_GP6_EINT: c_uint = 0x0020	/* IM_GP6_EINT */;
pub const WM8995_IM_GP6_EINT_MASK: c_uint = 0x0020	/* IM_GP6_EINT */;

pub const WM8995_IM_GP5_EINT: c_uint = 0x0010	/* IM_GP5_EINT */;
pub const WM8995_IM_GP5_EINT_MASK: c_uint = 0x0010	/* IM_GP5_EINT */;

pub const WM8995_IM_GP4_EINT: c_uint = 0x0008	/* IM_GP4_EINT */;
pub const WM8995_IM_GP4_EINT_MASK: c_uint = 0x0008	/* IM_GP4_EINT */;

pub const WM8995_IM_GP3_EINT: c_uint = 0x0004	/* IM_GP3_EINT */;
pub const WM8995_IM_GP3_EINT_MASK: c_uint = 0x0004	/* IM_GP3_EINT */;

pub const WM8995_IM_GP2_EINT: c_uint = 0x0002	/* IM_GP2_EINT */;
pub const WM8995_IM_GP2_EINT_MASK: c_uint = 0x0002	/* IM_GP2_EINT */;

pub const WM8995_IM_GP1_EINT: c_uint = 0x0001	/* IM_GP1_EINT */;
pub const WM8995_IM_GP1_EINT_MASK: c_uint = 0x0001	/* IM_GP1_EINT */;

//
// R1849 (0x739) - Interrupt Status 2 Mask
//
pub const WM8995_IM_DCS_DONE_23_EINT: c_uint = 0x1000	/* IM_DCS_DONE_23_EINT */;
pub const WM8995_IM_DCS_DONE_23_EINT_MASK: c_uint = 0x1000	/* IM_DCS_DONE_23_EINT */;

pub const WM8995_IM_DCS_DONE_01_EINT: c_uint = 0x0800	/* IM_DCS_DONE_01_EINT */;
pub const WM8995_IM_DCS_DONE_01_EINT_MASK: c_uint = 0x0800	/* IM_DCS_DONE_01_EINT */;

pub const WM8995_IM_WSEQ_DONE_EINT: c_uint = 0x0400	/* IM_WSEQ_DONE_EINT */;
pub const WM8995_IM_WSEQ_DONE_EINT_MASK: c_uint = 0x0400	/* IM_WSEQ_DONE_EINT */;

pub const WM8995_IM_FIFOS_ERR_EINT: c_uint = 0x0200	/* IM_FIFOS_ERR_EINT */;
pub const WM8995_IM_FIFOS_ERR_EINT_MASK: c_uint = 0x0200	/* IM_FIFOS_ERR_EINT */;

pub const WM8995_IM_AIF2DRC_SIG_DET_EINT: c_uint = 0x0100	/* IM_AIF2DRC_SIG_DET_EINT */;
pub const WM8995_IM_AIF2DRC_SIG_DET_EINT_MASK: c_uint = 0x0100	/* IM_AIF2DRC_SIG_DET_EINT */;

pub const WM8995_IM_AIF1DRC2_SIG_DET_EINT: c_uint = 0x0080	/* IM_AIF1DRC2_SIG_DET_EINT */;
pub const WM8995_IM_AIF1DRC2_SIG_DET_EINT_MASK: c_uint = 0x0080	/* IM_AIF1DRC2_SIG_DET_EINT */;

pub const WM8995_IM_AIF1DRC1_SIG_DET_EINT: c_uint = 0x0040	/* IM_AIF1DRC1_SIG_DET_EINT */;
pub const WM8995_IM_AIF1DRC1_SIG_DET_EINT_MASK: c_uint = 0x0040	/* IM_AIF1DRC1_SIG_DET_EINT */;

pub const WM8995_IM_SRC2_LOCK_EINT: c_uint = 0x0020	/* IM_SRC2_LOCK_EINT */;
pub const WM8995_IM_SRC2_LOCK_EINT_MASK: c_uint = 0x0020	/* IM_SRC2_LOCK_EINT */;

pub const WM8995_IM_SRC1_LOCK_EINT: c_uint = 0x0010	/* IM_SRC1_LOCK_EINT */;
pub const WM8995_IM_SRC1_LOCK_EINT_MASK: c_uint = 0x0010	/* IM_SRC1_LOCK_EINT */;

pub const WM8995_IM_FLL2_LOCK_EINT: c_uint = 0x0008	/* IM_FLL2_LOCK_EINT */;
pub const WM8995_IM_FLL2_LOCK_EINT_MASK: c_uint = 0x0008	/* IM_FLL2_LOCK_EINT */;

pub const WM8995_IM_FLL1_LOCK_EINT: c_uint = 0x0004	/* IM_FLL1_LOCK_EINT */;
pub const WM8995_IM_FLL1_LOCK_EINT_MASK: c_uint = 0x0004	/* IM_FLL1_LOCK_EINT */;

pub const WM8995_IM_HP_DONE_EINT: c_uint = 0x0002	/* IM_HP_DONE_EINT */;
pub const WM8995_IM_HP_DONE_EINT_MASK: c_uint = 0x0002	/* IM_HP_DONE_EINT */;

pub const WM8995_IM_MICD_EINT: c_uint = 0x0001	/* IM_MICD_EINT */;
pub const WM8995_IM_MICD_EINT_MASK: c_uint = 0x0001	/* IM_MICD_EINT */;

//
// R1856 (0x740) - Interrupt Control
//
pub const WM8995_IM_IRQ: c_uint = 0x0001	/* IM_IRQ */;
pub const WM8995_IM_IRQ_MASK: c_uint = 0x0001	/* IM_IRQ */;

//
// R2048 (0x800) - Left PDM Speaker 1
//
pub const WM8995_SPK1L_ENA: c_uint = 0x0010	/* SPK1L_ENA */;
pub const WM8995_SPK1L_ENA_MASK: c_uint = 0x0010	/* SPK1L_ENA */;

pub const WM8995_SPK1L_MUTE: c_uint = 0x0008	/* SPK1L_MUTE */;
pub const WM8995_SPK1L_MUTE_MASK: c_uint = 0x0008	/* SPK1L_MUTE */;

pub const WM8995_SPK1L_MUTE_ZC: c_uint = 0x0004	/* SPK1L_MUTE_ZC */;
pub const WM8995_SPK1L_MUTE_ZC_MASK: c_uint = 0x0004	/* SPK1L_MUTE_ZC */;

pub const WM8995_SPK1L_SRC_MASK: c_uint = 0x0003	/* SPK1L_SRC - [1:0] */;

//
// R2049 (0x801) - Right PDM Speaker 1
//
pub const WM8995_SPK1R_ENA: c_uint = 0x0010	/* SPK1R_ENA */;
pub const WM8995_SPK1R_ENA_MASK: c_uint = 0x0010	/* SPK1R_ENA */;

pub const WM8995_SPK1R_MUTE: c_uint = 0x0008	/* SPK1R_MUTE */;
pub const WM8995_SPK1R_MUTE_MASK: c_uint = 0x0008	/* SPK1R_MUTE */;

pub const WM8995_SPK1R_MUTE_ZC: c_uint = 0x0004	/* SPK1R_MUTE_ZC */;
pub const WM8995_SPK1R_MUTE_ZC_MASK: c_uint = 0x0004	/* SPK1R_MUTE_ZC */;

pub const WM8995_SPK1R_SRC_MASK: c_uint = 0x0003	/* SPK1R_SRC - [1:0] */;

//
// R2050 (0x802) - PDM Speaker 1 Mute Sequence
//
pub const WM8995_SPK1_MUTE_SEQ1_MASK: c_uint = 0x00FF	/* SPK1_MUTE_SEQ1 - [7:0] */;

//
// R2056 (0x808) - Left PDM Speaker 2
//
pub const WM8995_SPK2L_ENA: c_uint = 0x0010	/* SPK2L_ENA */;
pub const WM8995_SPK2L_ENA_MASK: c_uint = 0x0010	/* SPK2L_ENA */;

pub const WM8995_SPK2L_MUTE: c_uint = 0x0008	/* SPK2L_MUTE */;
pub const WM8995_SPK2L_MUTE_MASK: c_uint = 0x0008	/* SPK2L_MUTE */;

pub const WM8995_SPK2L_MUTE_ZC: c_uint = 0x0004	/* SPK2L_MUTE_ZC */;
pub const WM8995_SPK2L_MUTE_ZC_MASK: c_uint = 0x0004	/* SPK2L_MUTE_ZC */;

pub const WM8995_SPK2L_SRC_MASK: c_uint = 0x0003	/* SPK2L_SRC - [1:0] */;

//
// R2057 (0x809) - Right PDM Speaker 2
//
pub const WM8995_SPK2R_ENA: c_uint = 0x0010	/* SPK2R_ENA */;
pub const WM8995_SPK2R_ENA_MASK: c_uint = 0x0010	/* SPK2R_ENA */;

pub const WM8995_SPK2R_MUTE: c_uint = 0x0008	/* SPK2R_MUTE */;
pub const WM8995_SPK2R_MUTE_MASK: c_uint = 0x0008	/* SPK2R_MUTE */;

pub const WM8995_SPK2R_MUTE_ZC: c_uint = 0x0004	/* SPK2R_MUTE_ZC */;
pub const WM8995_SPK2R_MUTE_ZC_MASK: c_uint = 0x0004	/* SPK2R_MUTE_ZC */;

pub const WM8995_SPK2R_SRC_MASK: c_uint = 0x0003	/* SPK2R_SRC - [1:0] */;

//
// R2058 (0x80A) - PDM Speaker 2 Mute Sequence
//
pub const WM8995_SPK2_MUTE_SEQ1_MASK: c_uint = 0x00FF	/* SPK2_MUTE_SEQ1 - [7:0] */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8995_reg_access {
    pub read: u16,
    pub write: u16,
    pub vol: u16,
}

// Sources for AIF1/2 SYSCLK - use with set_dai_sysclk()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_src {
    WM8995_SYSCLK_MCLK1 = 1,
    WM8995_SYSCLK_MCLK2,
    WM8995_SYSCLK_FLL1,
    WM8995_SYSCLK_FLL2,
    WM8995_SYSCLK_OPCLK
}

pub const WM8995_FLL1: c_int = 1;
pub const WM8995_FLL2: c_int = 2;
pub const WM8995_FLL_SRC_MCLK1: c_int = 1;
pub const WM8995_FLL_SRC_MCLK2: c_int = 2;
pub const WM8995_FLL_SRC_LRCLK: c_int = 3;
pub const WM8995_FLL_SRC_BCLK: c_int = 4;
