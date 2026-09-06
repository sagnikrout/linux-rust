//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8994/registers.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/linux/mfd/wm8994/registers.h -- Register definitions for WM8994
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Register values.
//
pub const WM8994_SOFTWARE_RESET: c_uint = 0x00;
pub const WM8994_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8994_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8994_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8994_POWER_MANAGEMENT_4: c_uint = 0x04;
pub const WM8994_POWER_MANAGEMENT_5: c_uint = 0x05;
pub const WM8994_POWER_MANAGEMENT_6: c_uint = 0x06;
pub const WM8994_INPUT_MIXER_1: c_uint = 0x15;
pub const WM8994_LEFT_LINE_INPUT_1_2_VOLUME: c_uint = 0x18;
pub const WM8994_LEFT_LINE_INPUT_3_4_VOLUME: c_uint = 0x19;
pub const WM8994_RIGHT_LINE_INPUT_1_2_VOLUME: c_uint = 0x1A;
pub const WM8994_RIGHT_LINE_INPUT_3_4_VOLUME: c_uint = 0x1B;
pub const WM8994_LEFT_OUTPUT_VOLUME: c_uint = 0x1C;
pub const WM8994_RIGHT_OUTPUT_VOLUME: c_uint = 0x1D;
pub const WM8994_LINE_OUTPUTS_VOLUME: c_uint = 0x1E;
pub const WM8994_HPOUT2_VOLUME: c_uint = 0x1F;
pub const WM8994_LEFT_OPGA_VOLUME: c_uint = 0x20;
pub const WM8994_RIGHT_OPGA_VOLUME: c_uint = 0x21;
pub const WM8994_SPKMIXL_ATTENUATION: c_uint = 0x22;
pub const WM8994_SPKMIXR_ATTENUATION: c_uint = 0x23;
pub const WM8994_SPKOUT_MIXERS: c_uint = 0x24;
pub const WM8994_CLASSD: c_uint = 0x25;
pub const WM8994_SPEAKER_VOLUME_LEFT: c_uint = 0x26;
pub const WM8994_SPEAKER_VOLUME_RIGHT: c_uint = 0x27;
pub const WM8994_INPUT_MIXER_2: c_uint = 0x28;
pub const WM8994_INPUT_MIXER_3: c_uint = 0x29;
pub const WM8994_INPUT_MIXER_4: c_uint = 0x2A;
pub const WM8994_INPUT_MIXER_5: c_uint = 0x2B;
pub const WM8994_INPUT_MIXER_6: c_uint = 0x2C;
pub const WM8994_OUTPUT_MIXER_1: c_uint = 0x2D;
pub const WM8994_OUTPUT_MIXER_2: c_uint = 0x2E;
pub const WM8994_OUTPUT_MIXER_3: c_uint = 0x2F;
pub const WM8994_OUTPUT_MIXER_4: c_uint = 0x30;
pub const WM8994_OUTPUT_MIXER_5: c_uint = 0x31;
pub const WM8994_OUTPUT_MIXER_6: c_uint = 0x32;
pub const WM8994_HPOUT2_MIXER: c_uint = 0x33;
pub const WM8994_LINE_MIXER_1: c_uint = 0x34;
pub const WM8994_LINE_MIXER_2: c_uint = 0x35;
pub const WM8994_SPEAKER_MIXER: c_uint = 0x36;
pub const WM8994_ADDITIONAL_CONTROL: c_uint = 0x37;
pub const WM8994_ANTIPOP_1: c_uint = 0x38;
pub const WM8994_ANTIPOP_2: c_uint = 0x39;
pub const WM8994_MICBIAS: c_uint = 0x3A;
pub const WM8994_LDO_1: c_uint = 0x3B;
pub const WM8994_LDO_2: c_uint = 0x3C;
pub const WM8958_MICBIAS1: c_uint = 0x3D;
pub const WM8958_MICBIAS2: c_uint = 0x3E;
pub const WM8994_CHARGE_PUMP_1: c_uint = 0x4C;
pub const WM8958_CHARGE_PUMP_2: c_uint = 0x4D;
pub const WM8994_CLASS_W_1: c_uint = 0x51;
pub const WM8994_DC_SERVO_1: c_uint = 0x54;
pub const WM8994_DC_SERVO_2: c_uint = 0x55;
pub const WM8994_DC_SERVO_4: c_uint = 0x57;
pub const WM8994_DC_SERVO_READBACK: c_uint = 0x58;
pub const WM8994_DC_SERVO_4E: c_uint = 0x59;
pub const WM8994_ANALOGUE_HP_1: c_uint = 0x60;
pub const WM8958_MIC_DETECT_1: c_uint = 0xD0;
pub const WM8958_MIC_DETECT_2: c_uint = 0xD1;
pub const WM8958_MIC_DETECT_3: c_uint = 0xD2;
pub const WM8994_CHIP_REVISION: c_uint = 0x100;
pub const WM8994_CONTROL_INTERFACE: c_uint = 0x101;
pub const WM8994_WRITE_SEQUENCER_CTRL_1: c_uint = 0x110;
pub const WM8994_WRITE_SEQUENCER_CTRL_2: c_uint = 0x111;
pub const WM8994_AIF1_CLOCKING_1: c_uint = 0x200;
pub const WM8994_AIF1_CLOCKING_2: c_uint = 0x201;
pub const WM8994_AIF2_CLOCKING_1: c_uint = 0x204;
pub const WM8994_AIF2_CLOCKING_2: c_uint = 0x205;
pub const WM8994_CLOCKING_1: c_uint = 0x208;
pub const WM8994_CLOCKING_2: c_uint = 0x209;
pub const WM8994_AIF1_RATE: c_uint = 0x210;
pub const WM8994_AIF2_RATE: c_uint = 0x211;
pub const WM8994_RATE_STATUS: c_uint = 0x212;
pub const WM8994_FLL1_CONTROL_1: c_uint = 0x220;
pub const WM8994_FLL1_CONTROL_2: c_uint = 0x221;
pub const WM8994_FLL1_CONTROL_3: c_uint = 0x222;
pub const WM8994_FLL1_CONTROL_4: c_uint = 0x223;
pub const WM8994_FLL1_CONTROL_5: c_uint = 0x224;
pub const WM8958_FLL1_EFS_1: c_uint = 0x226;
pub const WM8958_FLL1_EFS_2: c_uint = 0x227;
pub const WM8994_FLL2_CONTROL_1: c_uint = 0x240;
pub const WM8994_FLL2_CONTROL_2: c_uint = 0x241;
pub const WM8994_FLL2_CONTROL_3: c_uint = 0x242;
pub const WM8994_FLL2_CONTROL_4: c_uint = 0x243;
pub const WM8994_FLL2_CONTROL_5: c_uint = 0x244;
pub const WM8958_FLL2_EFS_1: c_uint = 0x246;
pub const WM8958_FLL2_EFS_2: c_uint = 0x247;
pub const WM8994_AIF1_CONTROL_1: c_uint = 0x300;
pub const WM8994_AIF1_CONTROL_2: c_uint = 0x301;
pub const WM8994_AIF1_MASTER_SLAVE: c_uint = 0x302;
pub const WM8994_AIF1_BCLK: c_uint = 0x303;
pub const WM8994_AIF1ADC_LRCLK: c_uint = 0x304;
pub const WM8994_AIF1DAC_LRCLK: c_uint = 0x305;
pub const WM8994_AIF1DAC_DATA: c_uint = 0x306;
pub const WM8994_AIF1ADC_DATA: c_uint = 0x307;
pub const WM8994_AIF2_CONTROL_1: c_uint = 0x310;
pub const WM8994_AIF2_CONTROL_2: c_uint = 0x311;
pub const WM8994_AIF2_MASTER_SLAVE: c_uint = 0x312;
pub const WM8994_AIF2_BCLK: c_uint = 0x313;
pub const WM8994_AIF2ADC_LRCLK: c_uint = 0x314;
pub const WM8994_AIF2DAC_LRCLK: c_uint = 0x315;
pub const WM8994_AIF2DAC_DATA: c_uint = 0x316;
pub const WM8994_AIF2ADC_DATA: c_uint = 0x317;
pub const WM1811_AIF2TX_CONTROL: c_uint = 0x318;
pub const WM8958_AIF3_CONTROL_1: c_uint = 0x320;
pub const WM8958_AIF3_CONTROL_2: c_uint = 0x321;
pub const WM8958_AIF3DAC_DATA: c_uint = 0x322;
pub const WM8958_AIF3ADC_DATA: c_uint = 0x323;
pub const WM8994_AIF1_ADC1_LEFT_VOLUME: c_uint = 0x400;
pub const WM8994_AIF1_ADC1_RIGHT_VOLUME: c_uint = 0x401;
pub const WM8994_AIF1_DAC1_LEFT_VOLUME: c_uint = 0x402;
pub const WM8994_AIF1_DAC1_RIGHT_VOLUME: c_uint = 0x403;
pub const WM8994_AIF1_ADC2_LEFT_VOLUME: c_uint = 0x404;
pub const WM8994_AIF1_ADC2_RIGHT_VOLUME: c_uint = 0x405;
pub const WM8994_AIF1_DAC2_LEFT_VOLUME: c_uint = 0x406;
pub const WM8994_AIF1_DAC2_RIGHT_VOLUME: c_uint = 0x407;
pub const WM8994_AIF1_ADC1_FILTERS: c_uint = 0x410;
pub const WM8994_AIF1_ADC2_FILTERS: c_uint = 0x411;
pub const WM8994_AIF1_DAC1_FILTERS_1: c_uint = 0x420;
pub const WM8994_AIF1_DAC1_FILTERS_2: c_uint = 0x421;
pub const WM8994_AIF1_DAC2_FILTERS_1: c_uint = 0x422;
pub const WM8994_AIF1_DAC2_FILTERS_2: c_uint = 0x423;
pub const WM8958_AIF1_DAC1_NOISE_GATE: c_uint = 0x430;
pub const WM8958_AIF1_DAC2_NOISE_GATE: c_uint = 0x431;
pub const WM8994_AIF1_DRC1_1: c_uint = 0x440;
pub const WM8994_AIF1_DRC1_2: c_uint = 0x441;
pub const WM8994_AIF1_DRC1_3: c_uint = 0x442;
pub const WM8994_AIF1_DRC1_4: c_uint = 0x443;
pub const WM8994_AIF1_DRC1_5: c_uint = 0x444;
pub const WM8994_AIF1_DRC2_1: c_uint = 0x450;
pub const WM8994_AIF1_DRC2_2: c_uint = 0x451;
pub const WM8994_AIF1_DRC2_3: c_uint = 0x452;
pub const WM8994_AIF1_DRC2_4: c_uint = 0x453;
pub const WM8994_AIF1_DRC2_5: c_uint = 0x454;
pub const WM8994_AIF1_DAC1_EQ_GAINS_1: c_uint = 0x480;
pub const WM8994_AIF1_DAC1_EQ_GAINS_2: c_uint = 0x481;
pub const WM8994_AIF1_DAC1_EQ_BAND_1_A: c_uint = 0x482;
pub const WM8994_AIF1_DAC1_EQ_BAND_1_B: c_uint = 0x483;
pub const WM8994_AIF1_DAC1_EQ_BAND_1_PG: c_uint = 0x484;
pub const WM8994_AIF1_DAC1_EQ_BAND_2_A: c_uint = 0x485;
pub const WM8994_AIF1_DAC1_EQ_BAND_2_B: c_uint = 0x486;
pub const WM8994_AIF1_DAC1_EQ_BAND_2_C: c_uint = 0x487;
pub const WM8994_AIF1_DAC1_EQ_BAND_2_PG: c_uint = 0x488;
pub const WM8994_AIF1_DAC1_EQ_BAND_3_A: c_uint = 0x489;
pub const WM8994_AIF1_DAC1_EQ_BAND_3_B: c_uint = 0x48A;
pub const WM8994_AIF1_DAC1_EQ_BAND_3_C: c_uint = 0x48B;
pub const WM8994_AIF1_DAC1_EQ_BAND_3_PG: c_uint = 0x48C;
pub const WM8994_AIF1_DAC1_EQ_BAND_4_A: c_uint = 0x48D;
pub const WM8994_AIF1_DAC1_EQ_BAND_4_B: c_uint = 0x48E;
pub const WM8994_AIF1_DAC1_EQ_BAND_4_C: c_uint = 0x48F;
pub const WM8994_AIF1_DAC1_EQ_BAND_4_PG: c_uint = 0x490;
pub const WM8994_AIF1_DAC1_EQ_BAND_5_A: c_uint = 0x491;
pub const WM8994_AIF1_DAC1_EQ_BAND_5_B: c_uint = 0x492;
pub const WM8994_AIF1_DAC1_EQ_BAND_5_PG: c_uint = 0x493;
pub const WM8994_AIF1_DAC1_EQ_BAND_1_C: c_uint = 0x494;
pub const WM8994_AIF1_DAC2_EQ_GAINS_1: c_uint = 0x4A0;
pub const WM8994_AIF1_DAC2_EQ_GAINS_2: c_uint = 0x4A1;
pub const WM8994_AIF1_DAC2_EQ_BAND_1_A: c_uint = 0x4A2;
pub const WM8994_AIF1_DAC2_EQ_BAND_1_B: c_uint = 0x4A3;
pub const WM8994_AIF1_DAC2_EQ_BAND_1_PG: c_uint = 0x4A4;
pub const WM8994_AIF1_DAC2_EQ_BAND_2_A: c_uint = 0x4A5;
pub const WM8994_AIF1_DAC2_EQ_BAND_2_B: c_uint = 0x4A6;
pub const WM8994_AIF1_DAC2_EQ_BAND_2_C: c_uint = 0x4A7;
pub const WM8994_AIF1_DAC2_EQ_BAND_2_PG: c_uint = 0x4A8;
pub const WM8994_AIF1_DAC2_EQ_BAND_3_A: c_uint = 0x4A9;
pub const WM8994_AIF1_DAC2_EQ_BAND_3_B: c_uint = 0x4AA;
pub const WM8994_AIF1_DAC2_EQ_BAND_3_C: c_uint = 0x4AB;
pub const WM8994_AIF1_DAC2_EQ_BAND_3_PG: c_uint = 0x4AC;
pub const WM8994_AIF1_DAC2_EQ_BAND_4_A: c_uint = 0x4AD;
pub const WM8994_AIF1_DAC2_EQ_BAND_4_B: c_uint = 0x4AE;
pub const WM8994_AIF1_DAC2_EQ_BAND_4_C: c_uint = 0x4AF;
pub const WM8994_AIF1_DAC2_EQ_BAND_4_PG: c_uint = 0x4B0;
pub const WM8994_AIF1_DAC2_EQ_BAND_5_A: c_uint = 0x4B1;
pub const WM8994_AIF1_DAC2_EQ_BAND_5_B: c_uint = 0x4B2;
pub const WM8994_AIF1_DAC2_EQ_BAND_5_PG: c_uint = 0x4B3;
pub const WM8994_AIF1_DAC2_EQ_BAND_1_C: c_uint = 0x4B4;
pub const WM8994_AIF2_ADC_LEFT_VOLUME: c_uint = 0x500;
pub const WM8994_AIF2_ADC_RIGHT_VOLUME: c_uint = 0x501;
pub const WM8994_AIF2_DAC_LEFT_VOLUME: c_uint = 0x502;
pub const WM8994_AIF2_DAC_RIGHT_VOLUME: c_uint = 0x503;
pub const WM8994_AIF2_ADC_FILTERS: c_uint = 0x510;
pub const WM8994_AIF2_DAC_FILTERS_1: c_uint = 0x520;
pub const WM8994_AIF2_DAC_FILTERS_2: c_uint = 0x521;
pub const WM8958_AIF2_DAC_NOISE_GATE: c_uint = 0x530;
pub const WM8994_AIF2_DRC_1: c_uint = 0x540;
pub const WM8994_AIF2_DRC_2: c_uint = 0x541;
pub const WM8994_AIF2_DRC_3: c_uint = 0x542;
pub const WM8994_AIF2_DRC_4: c_uint = 0x543;
pub const WM8994_AIF2_DRC_5: c_uint = 0x544;
pub const WM8994_AIF2_EQ_GAINS_1: c_uint = 0x580;
pub const WM8994_AIF2_EQ_GAINS_2: c_uint = 0x581;
pub const WM8994_AIF2_EQ_BAND_1_A: c_uint = 0x582;
pub const WM8994_AIF2_EQ_BAND_1_B: c_uint = 0x583;
pub const WM8994_AIF2_EQ_BAND_1_PG: c_uint = 0x584;
pub const WM8994_AIF2_EQ_BAND_2_A: c_uint = 0x585;
pub const WM8994_AIF2_EQ_BAND_2_B: c_uint = 0x586;
pub const WM8994_AIF2_EQ_BAND_2_C: c_uint = 0x587;
pub const WM8994_AIF2_EQ_BAND_2_PG: c_uint = 0x588;
pub const WM8994_AIF2_EQ_BAND_3_A: c_uint = 0x589;
pub const WM8994_AIF2_EQ_BAND_3_B: c_uint = 0x58A;
pub const WM8994_AIF2_EQ_BAND_3_C: c_uint = 0x58B;
pub const WM8994_AIF2_EQ_BAND_3_PG: c_uint = 0x58C;
pub const WM8994_AIF2_EQ_BAND_4_A: c_uint = 0x58D;
pub const WM8994_AIF2_EQ_BAND_4_B: c_uint = 0x58E;
pub const WM8994_AIF2_EQ_BAND_4_C: c_uint = 0x58F;
pub const WM8994_AIF2_EQ_BAND_4_PG: c_uint = 0x590;
pub const WM8994_AIF2_EQ_BAND_5_A: c_uint = 0x591;
pub const WM8994_AIF2_EQ_BAND_5_B: c_uint = 0x592;
pub const WM8994_AIF2_EQ_BAND_5_PG: c_uint = 0x593;
pub const WM8994_AIF2_EQ_BAND_1_C: c_uint = 0x594;
pub const WM8994_DAC1_MIXER_VOLUMES: c_uint = 0x600;
pub const WM8994_DAC1_LEFT_MIXER_ROUTING: c_uint = 0x601;
pub const WM8994_DAC1_RIGHT_MIXER_ROUTING: c_uint = 0x602;
pub const WM8994_DAC2_MIXER_VOLUMES: c_uint = 0x603;
pub const WM8994_DAC2_LEFT_MIXER_ROUTING: c_uint = 0x604;
pub const WM8994_DAC2_RIGHT_MIXER_ROUTING: c_uint = 0x605;
pub const WM8994_AIF1_ADC1_LEFT_MIXER_ROUTING: c_uint = 0x606;
pub const WM8994_AIF1_ADC1_RIGHT_MIXER_ROUTING: c_uint = 0x607;
pub const WM8994_AIF1_ADC2_LEFT_MIXER_ROUTING: c_uint = 0x608;
pub const WM8994_AIF1_ADC2_RIGHT_MIXER_ROUTING: c_uint = 0x609;
pub const WM8994_DAC1_LEFT_VOLUME: c_uint = 0x610;
pub const WM8994_DAC1_RIGHT_VOLUME: c_uint = 0x611;
pub const WM8994_DAC2_LEFT_VOLUME: c_uint = 0x612;
pub const WM8994_DAC2_RIGHT_VOLUME: c_uint = 0x613;
pub const WM8994_DAC_SOFTMUTE: c_uint = 0x614;
pub const WM8994_OVERSAMPLING: c_uint = 0x620;
pub const WM8994_SIDETONE: c_uint = 0x621;
pub const WM8994_GPIO_1: c_uint = 0x700;
pub const WM8994_GPIO_2: c_uint = 0x701;
pub const WM8994_GPIO_3: c_uint = 0x702;
pub const WM8994_GPIO_4: c_uint = 0x703;
pub const WM8994_GPIO_5: c_uint = 0x704;
pub const WM8994_GPIO_6: c_uint = 0x705;
pub const WM1811_JACKDET_CTRL: c_uint = 0x705;
pub const WM8994_GPIO_7: c_uint = 0x706;
pub const WM8994_GPIO_8: c_uint = 0x707;
pub const WM8994_GPIO_9: c_uint = 0x708;
pub const WM8994_GPIO_10: c_uint = 0x709;
pub const WM8994_GPIO_11: c_uint = 0x70A;
pub const WM8994_PULL_CONTROL_1: c_uint = 0x720;
pub const WM8994_PULL_CONTROL_2: c_uint = 0x721;
pub const WM8994_INTERRUPT_STATUS_1: c_uint = 0x730;
pub const WM8994_INTERRUPT_STATUS_2: c_uint = 0x731;
pub const WM8994_INTERRUPT_RAW_STATUS_2: c_uint = 0x732;
pub const WM8994_INTERRUPT_STATUS_1_MASK: c_uint = 0x738;
pub const WM8994_INTERRUPT_STATUS_2_MASK: c_uint = 0x739;
pub const WM8994_INTERRUPT_CONTROL: c_uint = 0x740;
pub const WM8994_IRQ_DEBOUNCE: c_uint = 0x748;
pub const WM8958_DSP2_PROGRAM: c_uint = 0x900;
pub const WM8958_DSP2_CONFIG: c_uint = 0x901;
pub const WM8958_DSP2_MAGICNUM: c_uint = 0xA00;
pub const WM8958_DSP2_RELEASEYEAR: c_uint = 0xA01;
pub const WM8958_DSP2_RELEASEMONTHDAY: c_uint = 0xA02;
pub const WM8958_DSP2_RELEASETIME: c_uint = 0xA03;
pub const WM8958_DSP2_VERMAJMIN: c_uint = 0xA04;
pub const WM8958_DSP2_VERBUILD: c_uint = 0xA05;
pub const WM8958_DSP2_TESTREG: c_uint = 0xA06;
pub const WM8958_DSP2_XORREG: c_uint = 0xA07;
pub const WM8958_DSP2_SHIFTMAXX: c_uint = 0xA08;
pub const WM8958_DSP2_SHIFTMAXY: c_uint = 0xA09;
pub const WM8958_DSP2_SHIFTMAXZ: c_uint = 0xA0A;
pub const WM8958_DSP2_SHIFTMAXEXTLO: c_uint = 0xA0B;
pub const WM8958_DSP2_AESSELECT: c_uint = 0xA0C;
pub const WM8958_DSP2_EXECCONTROL: c_uint = 0xA0D;
pub const WM8958_DSP2_SAMPLEBREAK: c_uint = 0xA0E;
pub const WM8958_DSP2_COUNTBREAK: c_uint = 0xA0F;
pub const WM8958_DSP2_INTSTATUS: c_uint = 0xA10;
pub const WM8958_DSP2_EVENTSTATUS: c_uint = 0xA11;
pub const WM8958_DSP2_INTMASK: c_uint = 0xA12;
pub const WM8958_DSP2_CONFIGDWIDTH: c_uint = 0xA13;
pub const WM8958_DSP2_CONFIGINSTR: c_uint = 0xA14;
pub const WM8958_DSP2_CONFIGDMEM: c_uint = 0xA15;
pub const WM8958_DSP2_CONFIGDELAYS: c_uint = 0xA16;
pub const WM8958_DSP2_CONFIGNUMIO: c_uint = 0xA17;
pub const WM8958_DSP2_CONFIGEXTDEPTH: c_uint = 0xA18;
pub const WM8958_DSP2_CONFIGMULTIPLIER: c_uint = 0xA19;
pub const WM8958_DSP2_CONFIGCTRLDWIDTH: c_uint = 0xA1A;
pub const WM8958_DSP2_CONFIGPIPELINE: c_uint = 0xA1B;
pub const WM8958_DSP2_SHIFTMAXEXTHI: c_uint = 0xA1C;
pub const WM8958_DSP2_SWVERSIONREG: c_uint = 0xA1D;
pub const WM8958_DSP2_CONFIGXMEM: c_uint = 0xA1E;
pub const WM8958_DSP2_CONFIGYMEM: c_uint = 0xA1F;
pub const WM8958_DSP2_CONFIGZMEM: c_uint = 0xA20;
pub const WM8958_FW_BUILD_1: c_uint = 0x2000;
pub const WM8958_FW_BUILD_0: c_uint = 0x2001;
pub const WM8958_FW_ID_1: c_uint = 0x2002;
pub const WM8958_FW_ID_0: c_uint = 0x2003;
pub const WM8958_FW_MAJOR_1: c_uint = 0x2004;
pub const WM8958_FW_MAJOR_0: c_uint = 0x2005;
pub const WM8958_FW_MINOR_1: c_uint = 0x2006;
pub const WM8958_FW_MINOR_0: c_uint = 0x2007;
pub const WM8958_FW_PATCH_1: c_uint = 0x2008;
pub const WM8958_FW_PATCH_0: c_uint = 0x2009;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C1_1: c_uint = 0x2200;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C1_2: c_uint = 0x2201;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C2_1: c_uint = 0x2202;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C2_2: c_uint = 0x2203;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C3_1: c_uint = 0x2204;
pub const WM8958_MBC_BAND_2_LOWER_CUTOFF_C3_2: c_uint = 0x2205;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C2_1: c_uint = 0x2206;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C2_2: c_uint = 0x2207;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C3_1: c_uint = 0x2208;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C3_2: c_uint = 0x2209;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C1_1: c_uint = 0x220A;
pub const WM8958_MBC_BAND_2_UPPER_CUTOFF_C1_2: c_uint = 0x220B;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C1_1: c_uint = 0x220C;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C1_2: c_uint = 0x220D;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C2_1: c_uint = 0x220E;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C2_2: c_uint = 0x220F;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C3_1: c_uint = 0x2210;
pub const WM8958_MBC_BAND_1_UPPER_CUTOFF_C3_2: c_uint = 0x2211;
pub const WM8958_MBC_BAND_1_LOWER_CUTOFF_1: c_uint = 0x2212;
pub const WM8958_MBC_BAND_1_LOWER_CUTOFF_2: c_uint = 0x2213;
pub const WM8958_MBC_BAND_1_K_1: c_uint = 0x2400;
pub const WM8958_MBC_BAND_1_K_2: c_uint = 0x2401;
pub const WM8958_MBC_BAND_1_N1_1: c_uint = 0x2402;
pub const WM8958_MBC_BAND_1_N1_2: c_uint = 0x2403;
pub const WM8958_MBC_BAND_1_N2_1: c_uint = 0x2404;
pub const WM8958_MBC_BAND_1_N2_2: c_uint = 0x2405;
pub const WM8958_MBC_BAND_1_N3_1: c_uint = 0x2406;
pub const WM8958_MBC_BAND_1_N3_2: c_uint = 0x2407;
pub const WM8958_MBC_BAND_1_N4_1: c_uint = 0x2408;
pub const WM8958_MBC_BAND_1_N4_2: c_uint = 0x2409;
pub const WM8958_MBC_BAND_1_N5_1: c_uint = 0x240A;
pub const WM8958_MBC_BAND_1_N5_2: c_uint = 0x240B;
pub const WM8958_MBC_BAND_1_X1_1: c_uint = 0x240C;
pub const WM8958_MBC_BAND_1_X1_2: c_uint = 0x240D;
pub const WM8958_MBC_BAND_1_X2_1: c_uint = 0x240E;
pub const WM8958_MBC_BAND_1_X2_2: c_uint = 0x240F;
pub const WM8958_MBC_BAND_1_X3_1: c_uint = 0x2410;
pub const WM8958_MBC_BAND_1_X3_2: c_uint = 0x2411;
pub const WM8958_MBC_BAND_1_ATTACK_1: c_uint = 0x2412;
pub const WM8958_MBC_BAND_1_ATTACK_2: c_uint = 0x2413;
pub const WM8958_MBC_BAND_1_DECAY_1: c_uint = 0x2414;
pub const WM8958_MBC_BAND_1_DECAY_2: c_uint = 0x2415;
pub const WM8958_MBC_BAND_2_K_1: c_uint = 0x2416;
pub const WM8958_MBC_BAND_2_K_2: c_uint = 0x2417;
pub const WM8958_MBC_BAND_2_N1_1: c_uint = 0x2418;
pub const WM8958_MBC_BAND_2_N1_2: c_uint = 0x2419;
pub const WM8958_MBC_BAND_2_N2_1: c_uint = 0x241A;
pub const WM8958_MBC_BAND_2_N2_2: c_uint = 0x241B;
pub const WM8958_MBC_BAND_2_N3_1: c_uint = 0x241C;
pub const WM8958_MBC_BAND_2_N3_2: c_uint = 0x241D;
pub const WM8958_MBC_BAND_2_N4_1: c_uint = 0x241E;
pub const WM8958_MBC_BAND_2_N4_2: c_uint = 0x241F;
pub const WM8958_MBC_BAND_2_N5_1: c_uint = 0x2420;
pub const WM8958_MBC_BAND_2_N5_2: c_uint = 0x2421;
pub const WM8958_MBC_BAND_2_X1_1: c_uint = 0x2422;
pub const WM8958_MBC_BAND_2_X1_2: c_uint = 0x2423;
pub const WM8958_MBC_BAND_2_X2_1: c_uint = 0x2424;
pub const WM8958_MBC_BAND_2_X2_2: c_uint = 0x2425;
pub const WM8958_MBC_BAND_2_X3_1: c_uint = 0x2426;
pub const WM8958_MBC_BAND_2_X3_2: c_uint = 0x2427;
pub const WM8958_MBC_BAND_2_ATTACK_1: c_uint = 0x2428;
pub const WM8958_MBC_BAND_2_ATTACK_2: c_uint = 0x2429;
pub const WM8958_MBC_BAND_2_DECAY_1: c_uint = 0x242A;
pub const WM8958_MBC_BAND_2_DECAY_2: c_uint = 0x242B;
pub const WM8958_MBC_B2_PG2_1: c_uint = 0x242C;
pub const WM8958_MBC_B2_PG2_2: c_uint = 0x242D;
pub const WM8958_MBC_B1_PG2_1: c_uint = 0x242E;
pub const WM8958_MBC_B1_PG2_2: c_uint = 0x242F;
pub const WM8958_MBC_CROSSOVER_1: c_uint = 0x2600;
pub const WM8958_MBC_CROSSOVER_2: c_uint = 0x2601;
pub const WM8958_MBC_HPF_1: c_uint = 0x2602;
pub const WM8958_MBC_HPF_2: c_uint = 0x2603;
pub const WM8958_MBC_LPF_1: c_uint = 0x2606;
pub const WM8958_MBC_LPF_2: c_uint = 0x2607;
pub const WM8958_MBC_RMS_LIMIT_1: c_uint = 0x260A;
pub const WM8958_MBC_RMS_LIMIT_2: c_uint = 0x260B;
pub const WM8994_WRITE_SEQUENCER_0: c_uint = 0x3000;
pub const WM8994_WRITE_SEQUENCER_1: c_uint = 0x3001;
pub const WM8994_WRITE_SEQUENCER_2: c_uint = 0x3002;
pub const WM8994_WRITE_SEQUENCER_3: c_uint = 0x3003;
pub const WM8994_WRITE_SEQUENCER_4: c_uint = 0x3004;
pub const WM8994_WRITE_SEQUENCER_5: c_uint = 0x3005;
pub const WM8994_WRITE_SEQUENCER_6: c_uint = 0x3006;
pub const WM8994_WRITE_SEQUENCER_7: c_uint = 0x3007;
pub const WM8994_WRITE_SEQUENCER_8: c_uint = 0x3008;
pub const WM8994_WRITE_SEQUENCER_9: c_uint = 0x3009;
pub const WM8994_WRITE_SEQUENCER_10: c_uint = 0x300A;
pub const WM8994_WRITE_SEQUENCER_11: c_uint = 0x300B;
pub const WM8994_WRITE_SEQUENCER_12: c_uint = 0x300C;
pub const WM8994_WRITE_SEQUENCER_13: c_uint = 0x300D;
pub const WM8994_WRITE_SEQUENCER_14: c_uint = 0x300E;
pub const WM8994_WRITE_SEQUENCER_15: c_uint = 0x300F;
pub const WM8994_WRITE_SEQUENCER_16: c_uint = 0x3010;
pub const WM8994_WRITE_SEQUENCER_17: c_uint = 0x3011;
pub const WM8994_WRITE_SEQUENCER_18: c_uint = 0x3012;
pub const WM8994_WRITE_SEQUENCER_19: c_uint = 0x3013;
pub const WM8994_WRITE_SEQUENCER_20: c_uint = 0x3014;
pub const WM8994_WRITE_SEQUENCER_21: c_uint = 0x3015;
pub const WM8994_WRITE_SEQUENCER_22: c_uint = 0x3016;
pub const WM8994_WRITE_SEQUENCER_23: c_uint = 0x3017;
pub const WM8994_WRITE_SEQUENCER_24: c_uint = 0x3018;
pub const WM8994_WRITE_SEQUENCER_25: c_uint = 0x3019;
pub const WM8994_WRITE_SEQUENCER_26: c_uint = 0x301A;
pub const WM8994_WRITE_SEQUENCER_27: c_uint = 0x301B;
pub const WM8994_WRITE_SEQUENCER_28: c_uint = 0x301C;
pub const WM8994_WRITE_SEQUENCER_29: c_uint = 0x301D;
pub const WM8994_WRITE_SEQUENCER_30: c_uint = 0x301E;
pub const WM8994_WRITE_SEQUENCER_31: c_uint = 0x301F;
pub const WM8994_WRITE_SEQUENCER_32: c_uint = 0x3020;
pub const WM8994_WRITE_SEQUENCER_33: c_uint = 0x3021;
pub const WM8994_WRITE_SEQUENCER_34: c_uint = 0x3022;
pub const WM8994_WRITE_SEQUENCER_35: c_uint = 0x3023;
pub const WM8994_WRITE_SEQUENCER_36: c_uint = 0x3024;
pub const WM8994_WRITE_SEQUENCER_37: c_uint = 0x3025;
pub const WM8994_WRITE_SEQUENCER_38: c_uint = 0x3026;
pub const WM8994_WRITE_SEQUENCER_39: c_uint = 0x3027;
pub const WM8994_WRITE_SEQUENCER_40: c_uint = 0x3028;
pub const WM8994_WRITE_SEQUENCER_41: c_uint = 0x3029;
pub const WM8994_WRITE_SEQUENCER_42: c_uint = 0x302A;
pub const WM8994_WRITE_SEQUENCER_43: c_uint = 0x302B;
pub const WM8994_WRITE_SEQUENCER_44: c_uint = 0x302C;
pub const WM8994_WRITE_SEQUENCER_45: c_uint = 0x302D;
pub const WM8994_WRITE_SEQUENCER_46: c_uint = 0x302E;
pub const WM8994_WRITE_SEQUENCER_47: c_uint = 0x302F;
pub const WM8994_WRITE_SEQUENCER_48: c_uint = 0x3030;
pub const WM8994_WRITE_SEQUENCER_49: c_uint = 0x3031;
pub const WM8994_WRITE_SEQUENCER_50: c_uint = 0x3032;
pub const WM8994_WRITE_SEQUENCER_51: c_uint = 0x3033;
pub const WM8994_WRITE_SEQUENCER_52: c_uint = 0x3034;
pub const WM8994_WRITE_SEQUENCER_53: c_uint = 0x3035;
pub const WM8994_WRITE_SEQUENCER_54: c_uint = 0x3036;
pub const WM8994_WRITE_SEQUENCER_55: c_uint = 0x3037;
pub const WM8994_WRITE_SEQUENCER_56: c_uint = 0x3038;
pub const WM8994_WRITE_SEQUENCER_57: c_uint = 0x3039;
pub const WM8994_WRITE_SEQUENCER_58: c_uint = 0x303A;
pub const WM8994_WRITE_SEQUENCER_59: c_uint = 0x303B;
pub const WM8994_WRITE_SEQUENCER_60: c_uint = 0x303C;
pub const WM8994_WRITE_SEQUENCER_61: c_uint = 0x303D;
pub const WM8994_WRITE_SEQUENCER_62: c_uint = 0x303E;
pub const WM8994_WRITE_SEQUENCER_63: c_uint = 0x303F;
pub const WM8994_WRITE_SEQUENCER_64: c_uint = 0x3040;
pub const WM8994_WRITE_SEQUENCER_65: c_uint = 0x3041;
pub const WM8994_WRITE_SEQUENCER_66: c_uint = 0x3042;
pub const WM8994_WRITE_SEQUENCER_67: c_uint = 0x3043;
pub const WM8994_WRITE_SEQUENCER_68: c_uint = 0x3044;
pub const WM8994_WRITE_SEQUENCER_69: c_uint = 0x3045;
pub const WM8994_WRITE_SEQUENCER_70: c_uint = 0x3046;
pub const WM8994_WRITE_SEQUENCER_71: c_uint = 0x3047;
pub const WM8994_WRITE_SEQUENCER_72: c_uint = 0x3048;
pub const WM8994_WRITE_SEQUENCER_73: c_uint = 0x3049;
pub const WM8994_WRITE_SEQUENCER_74: c_uint = 0x304A;
pub const WM8994_WRITE_SEQUENCER_75: c_uint = 0x304B;
pub const WM8994_WRITE_SEQUENCER_76: c_uint = 0x304C;
pub const WM8994_WRITE_SEQUENCER_77: c_uint = 0x304D;
pub const WM8994_WRITE_SEQUENCER_78: c_uint = 0x304E;
pub const WM8994_WRITE_SEQUENCER_79: c_uint = 0x304F;
pub const WM8994_WRITE_SEQUENCER_80: c_uint = 0x3050;
pub const WM8994_WRITE_SEQUENCER_81: c_uint = 0x3051;
pub const WM8994_WRITE_SEQUENCER_82: c_uint = 0x3052;
pub const WM8994_WRITE_SEQUENCER_83: c_uint = 0x3053;
pub const WM8994_WRITE_SEQUENCER_84: c_uint = 0x3054;
pub const WM8994_WRITE_SEQUENCER_85: c_uint = 0x3055;
pub const WM8994_WRITE_SEQUENCER_86: c_uint = 0x3056;
pub const WM8994_WRITE_SEQUENCER_87: c_uint = 0x3057;
pub const WM8994_WRITE_SEQUENCER_88: c_uint = 0x3058;
pub const WM8994_WRITE_SEQUENCER_89: c_uint = 0x3059;
pub const WM8994_WRITE_SEQUENCER_90: c_uint = 0x305A;
pub const WM8994_WRITE_SEQUENCER_91: c_uint = 0x305B;
pub const WM8994_WRITE_SEQUENCER_92: c_uint = 0x305C;
pub const WM8994_WRITE_SEQUENCER_93: c_uint = 0x305D;
pub const WM8994_WRITE_SEQUENCER_94: c_uint = 0x305E;
pub const WM8994_WRITE_SEQUENCER_95: c_uint = 0x305F;
pub const WM8994_WRITE_SEQUENCER_96: c_uint = 0x3060;
pub const WM8994_WRITE_SEQUENCER_97: c_uint = 0x3061;
pub const WM8994_WRITE_SEQUENCER_98: c_uint = 0x3062;
pub const WM8994_WRITE_SEQUENCER_99: c_uint = 0x3063;
pub const WM8994_WRITE_SEQUENCER_100: c_uint = 0x3064;
pub const WM8994_WRITE_SEQUENCER_101: c_uint = 0x3065;
pub const WM8994_WRITE_SEQUENCER_102: c_uint = 0x3066;
pub const WM8994_WRITE_SEQUENCER_103: c_uint = 0x3067;
pub const WM8994_WRITE_SEQUENCER_104: c_uint = 0x3068;
pub const WM8994_WRITE_SEQUENCER_105: c_uint = 0x3069;
pub const WM8994_WRITE_SEQUENCER_106: c_uint = 0x306A;
pub const WM8994_WRITE_SEQUENCER_107: c_uint = 0x306B;
pub const WM8994_WRITE_SEQUENCER_108: c_uint = 0x306C;
pub const WM8994_WRITE_SEQUENCER_109: c_uint = 0x306D;
pub const WM8994_WRITE_SEQUENCER_110: c_uint = 0x306E;
pub const WM8994_WRITE_SEQUENCER_111: c_uint = 0x306F;
pub const WM8994_WRITE_SEQUENCER_112: c_uint = 0x3070;
pub const WM8994_WRITE_SEQUENCER_113: c_uint = 0x3071;
pub const WM8994_WRITE_SEQUENCER_114: c_uint = 0x3072;
pub const WM8994_WRITE_SEQUENCER_115: c_uint = 0x3073;
pub const WM8994_WRITE_SEQUENCER_116: c_uint = 0x3074;
pub const WM8994_WRITE_SEQUENCER_117: c_uint = 0x3075;
pub const WM8994_WRITE_SEQUENCER_118: c_uint = 0x3076;
pub const WM8994_WRITE_SEQUENCER_119: c_uint = 0x3077;
pub const WM8994_WRITE_SEQUENCER_120: c_uint = 0x3078;
pub const WM8994_WRITE_SEQUENCER_121: c_uint = 0x3079;
pub const WM8994_WRITE_SEQUENCER_122: c_uint = 0x307A;
pub const WM8994_WRITE_SEQUENCER_123: c_uint = 0x307B;
pub const WM8994_WRITE_SEQUENCER_124: c_uint = 0x307C;
pub const WM8994_WRITE_SEQUENCER_125: c_uint = 0x307D;
pub const WM8994_WRITE_SEQUENCER_126: c_uint = 0x307E;
pub const WM8994_WRITE_SEQUENCER_127: c_uint = 0x307F;
pub const WM8994_WRITE_SEQUENCER_128: c_uint = 0x3080;
pub const WM8994_WRITE_SEQUENCER_129: c_uint = 0x3081;
pub const WM8994_WRITE_SEQUENCER_130: c_uint = 0x3082;
pub const WM8994_WRITE_SEQUENCER_131: c_uint = 0x3083;
pub const WM8994_WRITE_SEQUENCER_132: c_uint = 0x3084;
pub const WM8994_WRITE_SEQUENCER_133: c_uint = 0x3085;
pub const WM8994_WRITE_SEQUENCER_134: c_uint = 0x3086;
pub const WM8994_WRITE_SEQUENCER_135: c_uint = 0x3087;
pub const WM8994_WRITE_SEQUENCER_136: c_uint = 0x3088;
pub const WM8994_WRITE_SEQUENCER_137: c_uint = 0x3089;
pub const WM8994_WRITE_SEQUENCER_138: c_uint = 0x308A;
pub const WM8994_WRITE_SEQUENCER_139: c_uint = 0x308B;
pub const WM8994_WRITE_SEQUENCER_140: c_uint = 0x308C;
pub const WM8994_WRITE_SEQUENCER_141: c_uint = 0x308D;
pub const WM8994_WRITE_SEQUENCER_142: c_uint = 0x308E;
pub const WM8994_WRITE_SEQUENCER_143: c_uint = 0x308F;
pub const WM8994_WRITE_SEQUENCER_144: c_uint = 0x3090;
pub const WM8994_WRITE_SEQUENCER_145: c_uint = 0x3091;
pub const WM8994_WRITE_SEQUENCER_146: c_uint = 0x3092;
pub const WM8994_WRITE_SEQUENCER_147: c_uint = 0x3093;
pub const WM8994_WRITE_SEQUENCER_148: c_uint = 0x3094;
pub const WM8994_WRITE_SEQUENCER_149: c_uint = 0x3095;
pub const WM8994_WRITE_SEQUENCER_150: c_uint = 0x3096;
pub const WM8994_WRITE_SEQUENCER_151: c_uint = 0x3097;
pub const WM8994_WRITE_SEQUENCER_152: c_uint = 0x3098;
pub const WM8994_WRITE_SEQUENCER_153: c_uint = 0x3099;
pub const WM8994_WRITE_SEQUENCER_154: c_uint = 0x309A;
pub const WM8994_WRITE_SEQUENCER_155: c_uint = 0x309B;
pub const WM8994_WRITE_SEQUENCER_156: c_uint = 0x309C;
pub const WM8994_WRITE_SEQUENCER_157: c_uint = 0x309D;
pub const WM8994_WRITE_SEQUENCER_158: c_uint = 0x309E;
pub const WM8994_WRITE_SEQUENCER_159: c_uint = 0x309F;
pub const WM8994_WRITE_SEQUENCER_160: c_uint = 0x30A0;
pub const WM8994_WRITE_SEQUENCER_161: c_uint = 0x30A1;
pub const WM8994_WRITE_SEQUENCER_162: c_uint = 0x30A2;
pub const WM8994_WRITE_SEQUENCER_163: c_uint = 0x30A3;
pub const WM8994_WRITE_SEQUENCER_164: c_uint = 0x30A4;
pub const WM8994_WRITE_SEQUENCER_165: c_uint = 0x30A5;
pub const WM8994_WRITE_SEQUENCER_166: c_uint = 0x30A6;
pub const WM8994_WRITE_SEQUENCER_167: c_uint = 0x30A7;
pub const WM8994_WRITE_SEQUENCER_168: c_uint = 0x30A8;
pub const WM8994_WRITE_SEQUENCER_169: c_uint = 0x30A9;
pub const WM8994_WRITE_SEQUENCER_170: c_uint = 0x30AA;
pub const WM8994_WRITE_SEQUENCER_171: c_uint = 0x30AB;
pub const WM8994_WRITE_SEQUENCER_172: c_uint = 0x30AC;
pub const WM8994_WRITE_SEQUENCER_173: c_uint = 0x30AD;
pub const WM8994_WRITE_SEQUENCER_174: c_uint = 0x30AE;
pub const WM8994_WRITE_SEQUENCER_175: c_uint = 0x30AF;
pub const WM8994_WRITE_SEQUENCER_176: c_uint = 0x30B0;
pub const WM8994_WRITE_SEQUENCER_177: c_uint = 0x30B1;
pub const WM8994_WRITE_SEQUENCER_178: c_uint = 0x30B2;
pub const WM8994_WRITE_SEQUENCER_179: c_uint = 0x30B3;
pub const WM8994_WRITE_SEQUENCER_180: c_uint = 0x30B4;
pub const WM8994_WRITE_SEQUENCER_181: c_uint = 0x30B5;
pub const WM8994_WRITE_SEQUENCER_182: c_uint = 0x30B6;
pub const WM8994_WRITE_SEQUENCER_183: c_uint = 0x30B7;
pub const WM8994_WRITE_SEQUENCER_184: c_uint = 0x30B8;
pub const WM8994_WRITE_SEQUENCER_185: c_uint = 0x30B9;
pub const WM8994_WRITE_SEQUENCER_186: c_uint = 0x30BA;
pub const WM8994_WRITE_SEQUENCER_187: c_uint = 0x30BB;
pub const WM8994_WRITE_SEQUENCER_188: c_uint = 0x30BC;
pub const WM8994_WRITE_SEQUENCER_189: c_uint = 0x30BD;
pub const WM8994_WRITE_SEQUENCER_190: c_uint = 0x30BE;
pub const WM8994_WRITE_SEQUENCER_191: c_uint = 0x30BF;
pub const WM8994_WRITE_SEQUENCER_192: c_uint = 0x30C0;
pub const WM8994_WRITE_SEQUENCER_193: c_uint = 0x30C1;
pub const WM8994_WRITE_SEQUENCER_194: c_uint = 0x30C2;
pub const WM8994_WRITE_SEQUENCER_195: c_uint = 0x30C3;
pub const WM8994_WRITE_SEQUENCER_196: c_uint = 0x30C4;
pub const WM8994_WRITE_SEQUENCER_197: c_uint = 0x30C5;
pub const WM8994_WRITE_SEQUENCER_198: c_uint = 0x30C6;
pub const WM8994_WRITE_SEQUENCER_199: c_uint = 0x30C7;
pub const WM8994_WRITE_SEQUENCER_200: c_uint = 0x30C8;
pub const WM8994_WRITE_SEQUENCER_201: c_uint = 0x30C9;
pub const WM8994_WRITE_SEQUENCER_202: c_uint = 0x30CA;
pub const WM8994_WRITE_SEQUENCER_203: c_uint = 0x30CB;
pub const WM8994_WRITE_SEQUENCER_204: c_uint = 0x30CC;
pub const WM8994_WRITE_SEQUENCER_205: c_uint = 0x30CD;
pub const WM8994_WRITE_SEQUENCER_206: c_uint = 0x30CE;
pub const WM8994_WRITE_SEQUENCER_207: c_uint = 0x30CF;
pub const WM8994_WRITE_SEQUENCER_208: c_uint = 0x30D0;
pub const WM8994_WRITE_SEQUENCER_209: c_uint = 0x30D1;
pub const WM8994_WRITE_SEQUENCER_210: c_uint = 0x30D2;
pub const WM8994_WRITE_SEQUENCER_211: c_uint = 0x30D3;
pub const WM8994_WRITE_SEQUENCER_212: c_uint = 0x30D4;
pub const WM8994_WRITE_SEQUENCER_213: c_uint = 0x30D5;
pub const WM8994_WRITE_SEQUENCER_214: c_uint = 0x30D6;
pub const WM8994_WRITE_SEQUENCER_215: c_uint = 0x30D7;
pub const WM8994_WRITE_SEQUENCER_216: c_uint = 0x30D8;
pub const WM8994_WRITE_SEQUENCER_217: c_uint = 0x30D9;
pub const WM8994_WRITE_SEQUENCER_218: c_uint = 0x30DA;
pub const WM8994_WRITE_SEQUENCER_219: c_uint = 0x30DB;
pub const WM8994_WRITE_SEQUENCER_220: c_uint = 0x30DC;
pub const WM8994_WRITE_SEQUENCER_221: c_uint = 0x30DD;
pub const WM8994_WRITE_SEQUENCER_222: c_uint = 0x30DE;
pub const WM8994_WRITE_SEQUENCER_223: c_uint = 0x30DF;
pub const WM8994_WRITE_SEQUENCER_224: c_uint = 0x30E0;
pub const WM8994_WRITE_SEQUENCER_225: c_uint = 0x30E1;
pub const WM8994_WRITE_SEQUENCER_226: c_uint = 0x30E2;
pub const WM8994_WRITE_SEQUENCER_227: c_uint = 0x30E3;
pub const WM8994_WRITE_SEQUENCER_228: c_uint = 0x30E4;
pub const WM8994_WRITE_SEQUENCER_229: c_uint = 0x30E5;
pub const WM8994_WRITE_SEQUENCER_230: c_uint = 0x30E6;
pub const WM8994_WRITE_SEQUENCER_231: c_uint = 0x30E7;
pub const WM8994_WRITE_SEQUENCER_232: c_uint = 0x30E8;
pub const WM8994_WRITE_SEQUENCER_233: c_uint = 0x30E9;
pub const WM8994_WRITE_SEQUENCER_234: c_uint = 0x30EA;
pub const WM8994_WRITE_SEQUENCER_235: c_uint = 0x30EB;
pub const WM8994_WRITE_SEQUENCER_236: c_uint = 0x30EC;
pub const WM8994_WRITE_SEQUENCER_237: c_uint = 0x30ED;
pub const WM8994_WRITE_SEQUENCER_238: c_uint = 0x30EE;
pub const WM8994_WRITE_SEQUENCER_239: c_uint = 0x30EF;
pub const WM8994_WRITE_SEQUENCER_240: c_uint = 0x30F0;
pub const WM8994_WRITE_SEQUENCER_241: c_uint = 0x30F1;
pub const WM8994_WRITE_SEQUENCER_242: c_uint = 0x30F2;
pub const WM8994_WRITE_SEQUENCER_243: c_uint = 0x30F3;
pub const WM8994_WRITE_SEQUENCER_244: c_uint = 0x30F4;
pub const WM8994_WRITE_SEQUENCER_245: c_uint = 0x30F5;
pub const WM8994_WRITE_SEQUENCER_246: c_uint = 0x30F6;
pub const WM8994_WRITE_SEQUENCER_247: c_uint = 0x30F7;
pub const WM8994_WRITE_SEQUENCER_248: c_uint = 0x30F8;
pub const WM8994_WRITE_SEQUENCER_249: c_uint = 0x30F9;
pub const WM8994_WRITE_SEQUENCER_250: c_uint = 0x30FA;
pub const WM8994_WRITE_SEQUENCER_251: c_uint = 0x30FB;
pub const WM8994_WRITE_SEQUENCER_252: c_uint = 0x30FC;
pub const WM8994_WRITE_SEQUENCER_253: c_uint = 0x30FD;
pub const WM8994_WRITE_SEQUENCER_254: c_uint = 0x30FE;
pub const WM8994_WRITE_SEQUENCER_255: c_uint = 0x30FF;
pub const WM8994_WRITE_SEQUENCER_256: c_uint = 0x3100;
pub const WM8994_WRITE_SEQUENCER_257: c_uint = 0x3101;
pub const WM8994_WRITE_SEQUENCER_258: c_uint = 0x3102;
pub const WM8994_WRITE_SEQUENCER_259: c_uint = 0x3103;
pub const WM8994_WRITE_SEQUENCER_260: c_uint = 0x3104;
pub const WM8994_WRITE_SEQUENCER_261: c_uint = 0x3105;
pub const WM8994_WRITE_SEQUENCER_262: c_uint = 0x3106;
pub const WM8994_WRITE_SEQUENCER_263: c_uint = 0x3107;
pub const WM8994_WRITE_SEQUENCER_264: c_uint = 0x3108;
pub const WM8994_WRITE_SEQUENCER_265: c_uint = 0x3109;
pub const WM8994_WRITE_SEQUENCER_266: c_uint = 0x310A;
pub const WM8994_WRITE_SEQUENCER_267: c_uint = 0x310B;
pub const WM8994_WRITE_SEQUENCER_268: c_uint = 0x310C;
pub const WM8994_WRITE_SEQUENCER_269: c_uint = 0x310D;
pub const WM8994_WRITE_SEQUENCER_270: c_uint = 0x310E;
pub const WM8994_WRITE_SEQUENCER_271: c_uint = 0x310F;
pub const WM8994_WRITE_SEQUENCER_272: c_uint = 0x3110;
pub const WM8994_WRITE_SEQUENCER_273: c_uint = 0x3111;
pub const WM8994_WRITE_SEQUENCER_274: c_uint = 0x3112;
pub const WM8994_WRITE_SEQUENCER_275: c_uint = 0x3113;
pub const WM8994_WRITE_SEQUENCER_276: c_uint = 0x3114;
pub const WM8994_WRITE_SEQUENCER_277: c_uint = 0x3115;
pub const WM8994_WRITE_SEQUENCER_278: c_uint = 0x3116;
pub const WM8994_WRITE_SEQUENCER_279: c_uint = 0x3117;
pub const WM8994_WRITE_SEQUENCER_280: c_uint = 0x3118;
pub const WM8994_WRITE_SEQUENCER_281: c_uint = 0x3119;
pub const WM8994_WRITE_SEQUENCER_282: c_uint = 0x311A;
pub const WM8994_WRITE_SEQUENCER_283: c_uint = 0x311B;
pub const WM8994_WRITE_SEQUENCER_284: c_uint = 0x311C;
pub const WM8994_WRITE_SEQUENCER_285: c_uint = 0x311D;
pub const WM8994_WRITE_SEQUENCER_286: c_uint = 0x311E;
pub const WM8994_WRITE_SEQUENCER_287: c_uint = 0x311F;
pub const WM8994_WRITE_SEQUENCER_288: c_uint = 0x3120;
pub const WM8994_WRITE_SEQUENCER_289: c_uint = 0x3121;
pub const WM8994_WRITE_SEQUENCER_290: c_uint = 0x3122;
pub const WM8994_WRITE_SEQUENCER_291: c_uint = 0x3123;
pub const WM8994_WRITE_SEQUENCER_292: c_uint = 0x3124;
pub const WM8994_WRITE_SEQUENCER_293: c_uint = 0x3125;
pub const WM8994_WRITE_SEQUENCER_294: c_uint = 0x3126;
pub const WM8994_WRITE_SEQUENCER_295: c_uint = 0x3127;
pub const WM8994_WRITE_SEQUENCER_296: c_uint = 0x3128;
pub const WM8994_WRITE_SEQUENCER_297: c_uint = 0x3129;
pub const WM8994_WRITE_SEQUENCER_298: c_uint = 0x312A;
pub const WM8994_WRITE_SEQUENCER_299: c_uint = 0x312B;
pub const WM8994_WRITE_SEQUENCER_300: c_uint = 0x312C;
pub const WM8994_WRITE_SEQUENCER_301: c_uint = 0x312D;
pub const WM8994_WRITE_SEQUENCER_302: c_uint = 0x312E;
pub const WM8994_WRITE_SEQUENCER_303: c_uint = 0x312F;
pub const WM8994_WRITE_SEQUENCER_304: c_uint = 0x3130;
pub const WM8994_WRITE_SEQUENCER_305: c_uint = 0x3131;
pub const WM8994_WRITE_SEQUENCER_306: c_uint = 0x3132;
pub const WM8994_WRITE_SEQUENCER_307: c_uint = 0x3133;
pub const WM8994_WRITE_SEQUENCER_308: c_uint = 0x3134;
pub const WM8994_WRITE_SEQUENCER_309: c_uint = 0x3135;
pub const WM8994_WRITE_SEQUENCER_310: c_uint = 0x3136;
pub const WM8994_WRITE_SEQUENCER_311: c_uint = 0x3137;
pub const WM8994_WRITE_SEQUENCER_312: c_uint = 0x3138;
pub const WM8994_WRITE_SEQUENCER_313: c_uint = 0x3139;
pub const WM8994_WRITE_SEQUENCER_314: c_uint = 0x313A;
pub const WM8994_WRITE_SEQUENCER_315: c_uint = 0x313B;
pub const WM8994_WRITE_SEQUENCER_316: c_uint = 0x313C;
pub const WM8994_WRITE_SEQUENCER_317: c_uint = 0x313D;
pub const WM8994_WRITE_SEQUENCER_318: c_uint = 0x313E;
pub const WM8994_WRITE_SEQUENCER_319: c_uint = 0x313F;
pub const WM8994_WRITE_SEQUENCER_320: c_uint = 0x3140;
pub const WM8994_WRITE_SEQUENCER_321: c_uint = 0x3141;
pub const WM8994_WRITE_SEQUENCER_322: c_uint = 0x3142;
pub const WM8994_WRITE_SEQUENCER_323: c_uint = 0x3143;
pub const WM8994_WRITE_SEQUENCER_324: c_uint = 0x3144;
pub const WM8994_WRITE_SEQUENCER_325: c_uint = 0x3145;
pub const WM8994_WRITE_SEQUENCER_326: c_uint = 0x3146;
pub const WM8994_WRITE_SEQUENCER_327: c_uint = 0x3147;
pub const WM8994_WRITE_SEQUENCER_328: c_uint = 0x3148;
pub const WM8994_WRITE_SEQUENCER_329: c_uint = 0x3149;
pub const WM8994_WRITE_SEQUENCER_330: c_uint = 0x314A;
pub const WM8994_WRITE_SEQUENCER_331: c_uint = 0x314B;
pub const WM8994_WRITE_SEQUENCER_332: c_uint = 0x314C;
pub const WM8994_WRITE_SEQUENCER_333: c_uint = 0x314D;
pub const WM8994_WRITE_SEQUENCER_334: c_uint = 0x314E;
pub const WM8994_WRITE_SEQUENCER_335: c_uint = 0x314F;
pub const WM8994_WRITE_SEQUENCER_336: c_uint = 0x3150;
pub const WM8994_WRITE_SEQUENCER_337: c_uint = 0x3151;
pub const WM8994_WRITE_SEQUENCER_338: c_uint = 0x3152;
pub const WM8994_WRITE_SEQUENCER_339: c_uint = 0x3153;
pub const WM8994_WRITE_SEQUENCER_340: c_uint = 0x3154;
pub const WM8994_WRITE_SEQUENCER_341: c_uint = 0x3155;
pub const WM8994_WRITE_SEQUENCER_342: c_uint = 0x3156;
pub const WM8994_WRITE_SEQUENCER_343: c_uint = 0x3157;
pub const WM8994_WRITE_SEQUENCER_344: c_uint = 0x3158;
pub const WM8994_WRITE_SEQUENCER_345: c_uint = 0x3159;
pub const WM8994_WRITE_SEQUENCER_346: c_uint = 0x315A;
pub const WM8994_WRITE_SEQUENCER_347: c_uint = 0x315B;
pub const WM8994_WRITE_SEQUENCER_348: c_uint = 0x315C;
pub const WM8994_WRITE_SEQUENCER_349: c_uint = 0x315D;
pub const WM8994_WRITE_SEQUENCER_350: c_uint = 0x315E;
pub const WM8994_WRITE_SEQUENCER_351: c_uint = 0x315F;
pub const WM8994_WRITE_SEQUENCER_352: c_uint = 0x3160;
pub const WM8994_WRITE_SEQUENCER_353: c_uint = 0x3161;
pub const WM8994_WRITE_SEQUENCER_354: c_uint = 0x3162;
pub const WM8994_WRITE_SEQUENCER_355: c_uint = 0x3163;
pub const WM8994_WRITE_SEQUENCER_356: c_uint = 0x3164;
pub const WM8994_WRITE_SEQUENCER_357: c_uint = 0x3165;
pub const WM8994_WRITE_SEQUENCER_358: c_uint = 0x3166;
pub const WM8994_WRITE_SEQUENCER_359: c_uint = 0x3167;
pub const WM8994_WRITE_SEQUENCER_360: c_uint = 0x3168;
pub const WM8994_WRITE_SEQUENCER_361: c_uint = 0x3169;
pub const WM8994_WRITE_SEQUENCER_362: c_uint = 0x316A;
pub const WM8994_WRITE_SEQUENCER_363: c_uint = 0x316B;
pub const WM8994_WRITE_SEQUENCER_364: c_uint = 0x316C;
pub const WM8994_WRITE_SEQUENCER_365: c_uint = 0x316D;
pub const WM8994_WRITE_SEQUENCER_366: c_uint = 0x316E;
pub const WM8994_WRITE_SEQUENCER_367: c_uint = 0x316F;
pub const WM8994_WRITE_SEQUENCER_368: c_uint = 0x3170;
pub const WM8994_WRITE_SEQUENCER_369: c_uint = 0x3171;
pub const WM8994_WRITE_SEQUENCER_370: c_uint = 0x3172;
pub const WM8994_WRITE_SEQUENCER_371: c_uint = 0x3173;
pub const WM8994_WRITE_SEQUENCER_372: c_uint = 0x3174;
pub const WM8994_WRITE_SEQUENCER_373: c_uint = 0x3175;
pub const WM8994_WRITE_SEQUENCER_374: c_uint = 0x3176;
pub const WM8994_WRITE_SEQUENCER_375: c_uint = 0x3177;
pub const WM8994_WRITE_SEQUENCER_376: c_uint = 0x3178;
pub const WM8994_WRITE_SEQUENCER_377: c_uint = 0x3179;
pub const WM8994_WRITE_SEQUENCER_378: c_uint = 0x317A;
pub const WM8994_WRITE_SEQUENCER_379: c_uint = 0x317B;
pub const WM8994_WRITE_SEQUENCER_380: c_uint = 0x317C;
pub const WM8994_WRITE_SEQUENCER_381: c_uint = 0x317D;
pub const WM8994_WRITE_SEQUENCER_382: c_uint = 0x317E;
pub const WM8994_WRITE_SEQUENCER_383: c_uint = 0x317F;
pub const WM8994_WRITE_SEQUENCER_384: c_uint = 0x3180;
pub const WM8994_WRITE_SEQUENCER_385: c_uint = 0x3181;
pub const WM8994_WRITE_SEQUENCER_386: c_uint = 0x3182;
pub const WM8994_WRITE_SEQUENCER_387: c_uint = 0x3183;
pub const WM8994_WRITE_SEQUENCER_388: c_uint = 0x3184;
pub const WM8994_WRITE_SEQUENCER_389: c_uint = 0x3185;
pub const WM8994_WRITE_SEQUENCER_390: c_uint = 0x3186;
pub const WM8994_WRITE_SEQUENCER_391: c_uint = 0x3187;
pub const WM8994_WRITE_SEQUENCER_392: c_uint = 0x3188;
pub const WM8994_WRITE_SEQUENCER_393: c_uint = 0x3189;
pub const WM8994_WRITE_SEQUENCER_394: c_uint = 0x318A;
pub const WM8994_WRITE_SEQUENCER_395: c_uint = 0x318B;
pub const WM8994_WRITE_SEQUENCER_396: c_uint = 0x318C;
pub const WM8994_WRITE_SEQUENCER_397: c_uint = 0x318D;
pub const WM8994_WRITE_SEQUENCER_398: c_uint = 0x318E;
pub const WM8994_WRITE_SEQUENCER_399: c_uint = 0x318F;
pub const WM8994_WRITE_SEQUENCER_400: c_uint = 0x3190;
pub const WM8994_WRITE_SEQUENCER_401: c_uint = 0x3191;
pub const WM8994_WRITE_SEQUENCER_402: c_uint = 0x3192;
pub const WM8994_WRITE_SEQUENCER_403: c_uint = 0x3193;
pub const WM8994_WRITE_SEQUENCER_404: c_uint = 0x3194;
pub const WM8994_WRITE_SEQUENCER_405: c_uint = 0x3195;
pub const WM8994_WRITE_SEQUENCER_406: c_uint = 0x3196;
pub const WM8994_WRITE_SEQUENCER_407: c_uint = 0x3197;
pub const WM8994_WRITE_SEQUENCER_408: c_uint = 0x3198;
pub const WM8994_WRITE_SEQUENCER_409: c_uint = 0x3199;
pub const WM8994_WRITE_SEQUENCER_410: c_uint = 0x319A;
pub const WM8994_WRITE_SEQUENCER_411: c_uint = 0x319B;
pub const WM8994_WRITE_SEQUENCER_412: c_uint = 0x319C;
pub const WM8994_WRITE_SEQUENCER_413: c_uint = 0x319D;
pub const WM8994_WRITE_SEQUENCER_414: c_uint = 0x319E;
pub const WM8994_WRITE_SEQUENCER_415: c_uint = 0x319F;
pub const WM8994_WRITE_SEQUENCER_416: c_uint = 0x31A0;
pub const WM8994_WRITE_SEQUENCER_417: c_uint = 0x31A1;
pub const WM8994_WRITE_SEQUENCER_418: c_uint = 0x31A2;
pub const WM8994_WRITE_SEQUENCER_419: c_uint = 0x31A3;
pub const WM8994_WRITE_SEQUENCER_420: c_uint = 0x31A4;
pub const WM8994_WRITE_SEQUENCER_421: c_uint = 0x31A5;
pub const WM8994_WRITE_SEQUENCER_422: c_uint = 0x31A6;
pub const WM8994_WRITE_SEQUENCER_423: c_uint = 0x31A7;
pub const WM8994_WRITE_SEQUENCER_424: c_uint = 0x31A8;
pub const WM8994_WRITE_SEQUENCER_425: c_uint = 0x31A9;
pub const WM8994_WRITE_SEQUENCER_426: c_uint = 0x31AA;
pub const WM8994_WRITE_SEQUENCER_427: c_uint = 0x31AB;
pub const WM8994_WRITE_SEQUENCER_428: c_uint = 0x31AC;
pub const WM8994_WRITE_SEQUENCER_429: c_uint = 0x31AD;
pub const WM8994_WRITE_SEQUENCER_430: c_uint = 0x31AE;
pub const WM8994_WRITE_SEQUENCER_431: c_uint = 0x31AF;
pub const WM8994_WRITE_SEQUENCER_432: c_uint = 0x31B0;
pub const WM8994_WRITE_SEQUENCER_433: c_uint = 0x31B1;
pub const WM8994_WRITE_SEQUENCER_434: c_uint = 0x31B2;
pub const WM8994_WRITE_SEQUENCER_435: c_uint = 0x31B3;
pub const WM8994_WRITE_SEQUENCER_436: c_uint = 0x31B4;
pub const WM8994_WRITE_SEQUENCER_437: c_uint = 0x31B5;
pub const WM8994_WRITE_SEQUENCER_438: c_uint = 0x31B6;
pub const WM8994_WRITE_SEQUENCER_439: c_uint = 0x31B7;
pub const WM8994_WRITE_SEQUENCER_440: c_uint = 0x31B8;
pub const WM8994_WRITE_SEQUENCER_441: c_uint = 0x31B9;
pub const WM8994_WRITE_SEQUENCER_442: c_uint = 0x31BA;
pub const WM8994_WRITE_SEQUENCER_443: c_uint = 0x31BB;
pub const WM8994_WRITE_SEQUENCER_444: c_uint = 0x31BC;
pub const WM8994_WRITE_SEQUENCER_445: c_uint = 0x31BD;
pub const WM8994_WRITE_SEQUENCER_446: c_uint = 0x31BE;
pub const WM8994_WRITE_SEQUENCER_447: c_uint = 0x31BF;
pub const WM8994_WRITE_SEQUENCER_448: c_uint = 0x31C0;
pub const WM8994_WRITE_SEQUENCER_449: c_uint = 0x31C1;
pub const WM8994_WRITE_SEQUENCER_450: c_uint = 0x31C2;
pub const WM8994_WRITE_SEQUENCER_451: c_uint = 0x31C3;
pub const WM8994_WRITE_SEQUENCER_452: c_uint = 0x31C4;
pub const WM8994_WRITE_SEQUENCER_453: c_uint = 0x31C5;
pub const WM8994_WRITE_SEQUENCER_454: c_uint = 0x31C6;
pub const WM8994_WRITE_SEQUENCER_455: c_uint = 0x31C7;
pub const WM8994_WRITE_SEQUENCER_456: c_uint = 0x31C8;
pub const WM8994_WRITE_SEQUENCER_457: c_uint = 0x31C9;
pub const WM8994_WRITE_SEQUENCER_458: c_uint = 0x31CA;
pub const WM8994_WRITE_SEQUENCER_459: c_uint = 0x31CB;
pub const WM8994_WRITE_SEQUENCER_460: c_uint = 0x31CC;
pub const WM8994_WRITE_SEQUENCER_461: c_uint = 0x31CD;
pub const WM8994_WRITE_SEQUENCER_462: c_uint = 0x31CE;
pub const WM8994_WRITE_SEQUENCER_463: c_uint = 0x31CF;
pub const WM8994_WRITE_SEQUENCER_464: c_uint = 0x31D0;
pub const WM8994_WRITE_SEQUENCER_465: c_uint = 0x31D1;
pub const WM8994_WRITE_SEQUENCER_466: c_uint = 0x31D2;
pub const WM8994_WRITE_SEQUENCER_467: c_uint = 0x31D3;
pub const WM8994_WRITE_SEQUENCER_468: c_uint = 0x31D4;
pub const WM8994_WRITE_SEQUENCER_469: c_uint = 0x31D5;
pub const WM8994_WRITE_SEQUENCER_470: c_uint = 0x31D6;
pub const WM8994_WRITE_SEQUENCER_471: c_uint = 0x31D7;
pub const WM8994_WRITE_SEQUENCER_472: c_uint = 0x31D8;
pub const WM8994_WRITE_SEQUENCER_473: c_uint = 0x31D9;
pub const WM8994_WRITE_SEQUENCER_474: c_uint = 0x31DA;
pub const WM8994_WRITE_SEQUENCER_475: c_uint = 0x31DB;
pub const WM8994_WRITE_SEQUENCER_476: c_uint = 0x31DC;
pub const WM8994_WRITE_SEQUENCER_477: c_uint = 0x31DD;
pub const WM8994_WRITE_SEQUENCER_478: c_uint = 0x31DE;
pub const WM8994_WRITE_SEQUENCER_479: c_uint = 0x31DF;
pub const WM8994_WRITE_SEQUENCER_480: c_uint = 0x31E0;
pub const WM8994_WRITE_SEQUENCER_481: c_uint = 0x31E1;
pub const WM8994_WRITE_SEQUENCER_482: c_uint = 0x31E2;
pub const WM8994_WRITE_SEQUENCER_483: c_uint = 0x31E3;
pub const WM8994_WRITE_SEQUENCER_484: c_uint = 0x31E4;
pub const WM8994_WRITE_SEQUENCER_485: c_uint = 0x31E5;
pub const WM8994_WRITE_SEQUENCER_486: c_uint = 0x31E6;
pub const WM8994_WRITE_SEQUENCER_487: c_uint = 0x31E7;
pub const WM8994_WRITE_SEQUENCER_488: c_uint = 0x31E8;
pub const WM8994_WRITE_SEQUENCER_489: c_uint = 0x31E9;
pub const WM8994_WRITE_SEQUENCER_490: c_uint = 0x31EA;
pub const WM8994_WRITE_SEQUENCER_491: c_uint = 0x31EB;
pub const WM8994_WRITE_SEQUENCER_492: c_uint = 0x31EC;
pub const WM8994_WRITE_SEQUENCER_493: c_uint = 0x31ED;
pub const WM8994_WRITE_SEQUENCER_494: c_uint = 0x31EE;
pub const WM8994_WRITE_SEQUENCER_495: c_uint = 0x31EF;
pub const WM8994_WRITE_SEQUENCER_496: c_uint = 0x31F0;
pub const WM8994_WRITE_SEQUENCER_497: c_uint = 0x31F1;
pub const WM8994_WRITE_SEQUENCER_498: c_uint = 0x31F2;
pub const WM8994_WRITE_SEQUENCER_499: c_uint = 0x31F3;
pub const WM8994_WRITE_SEQUENCER_500: c_uint = 0x31F4;
pub const WM8994_WRITE_SEQUENCER_501: c_uint = 0x31F5;
pub const WM8994_WRITE_SEQUENCER_502: c_uint = 0x31F6;
pub const WM8994_WRITE_SEQUENCER_503: c_uint = 0x31F7;
pub const WM8994_WRITE_SEQUENCER_504: c_uint = 0x31F8;
pub const WM8994_WRITE_SEQUENCER_505: c_uint = 0x31F9;
pub const WM8994_WRITE_SEQUENCER_506: c_uint = 0x31FA;
pub const WM8994_WRITE_SEQUENCER_507: c_uint = 0x31FB;
pub const WM8994_WRITE_SEQUENCER_508: c_uint = 0x31FC;
pub const WM8994_WRITE_SEQUENCER_509: c_uint = 0x31FD;
pub const WM8994_WRITE_SEQUENCER_510: c_uint = 0x31FE;
pub const WM8994_WRITE_SEQUENCER_511: c_uint = 0x31FF;
pub const WM8994_REGISTER_COUNT: c_int = 736;
pub const WM8994_MAX_REGISTER: c_uint = 0x31FF;
pub const WM8994_MAX_CACHED_REGISTER: c_uint = 0x749;
//
// Field Definitions.
//
// R0 (0x00) - Software Reset
//
pub const WM8994_SW_RESET_MASK: c_uint = 0xFFFF  /* SW_RESET - [15:0] */;

//
// R1 (0x01) - Power Management (1)
//
pub const WM8994_SPKOUTR_ENA: c_uint = 0x2000  /* SPKOUTR_ENA */;
pub const WM8994_SPKOUTR_ENA_MASK: c_uint = 0x2000  /* SPKOUTR_ENA */;

pub const WM8994_SPKOUTL_ENA: c_uint = 0x1000  /* SPKOUTL_ENA */;
pub const WM8994_SPKOUTL_ENA_MASK: c_uint = 0x1000  /* SPKOUTL_ENA */;

pub const WM8994_HPOUT2_ENA: c_uint = 0x0800  /* HPOUT2_ENA */;
pub const WM8994_HPOUT2_ENA_MASK: c_uint = 0x0800  /* HPOUT2_ENA */;

pub const WM8994_HPOUT1L_ENA: c_uint = 0x0200  /* HPOUT1L_ENA */;
pub const WM8994_HPOUT1L_ENA_MASK: c_uint = 0x0200  /* HPOUT1L_ENA */;

pub const WM8994_HPOUT1R_ENA: c_uint = 0x0100  /* HPOUT1R_ENA */;
pub const WM8994_HPOUT1R_ENA_MASK: c_uint = 0x0100  /* HPOUT1R_ENA */;

pub const WM8994_MICB2_ENA: c_uint = 0x0020  /* MICB2_ENA */;
pub const WM8994_MICB2_ENA_MASK: c_uint = 0x0020  /* MICB2_ENA */;

pub const WM8994_MICB1_ENA: c_uint = 0x0010  /* MICB1_ENA */;
pub const WM8994_MICB1_ENA_MASK: c_uint = 0x0010  /* MICB1_ENA */;

pub const WM8994_VMID_SEL_MASK: c_uint = 0x0006  /* VMID_SEL - [2:1] */;

pub const WM8994_BIAS_ENA: c_uint = 0x0001  /* BIAS_ENA */;
pub const WM8994_BIAS_ENA_MASK: c_uint = 0x0001  /* BIAS_ENA */;

//
// R2 (0x02) - Power Management (2)
//
pub const WM8994_TSHUT_ENA: c_uint = 0x4000  /* TSHUT_ENA */;
pub const WM8994_TSHUT_ENA_MASK: c_uint = 0x4000  /* TSHUT_ENA */;

pub const WM8994_TSHUT_OPDIS: c_uint = 0x2000  /* TSHUT_OPDIS */;
pub const WM8994_TSHUT_OPDIS_MASK: c_uint = 0x2000  /* TSHUT_OPDIS */;

pub const WM8994_OPCLK_ENA: c_uint = 0x0800  /* OPCLK_ENA */;
pub const WM8994_OPCLK_ENA_MASK: c_uint = 0x0800  /* OPCLK_ENA */;

pub const WM8994_MIXINL_ENA: c_uint = 0x0200  /* MIXINL_ENA */;
pub const WM8994_MIXINL_ENA_MASK: c_uint = 0x0200  /* MIXINL_ENA */;

pub const WM8994_MIXINR_ENA: c_uint = 0x0100  /* MIXINR_ENA */;
pub const WM8994_MIXINR_ENA_MASK: c_uint = 0x0100  /* MIXINR_ENA */;

pub const WM8994_IN2L_ENA: c_uint = 0x0080  /* IN2L_ENA */;
pub const WM8994_IN2L_ENA_MASK: c_uint = 0x0080  /* IN2L_ENA */;

pub const WM8994_IN1L_ENA: c_uint = 0x0040  /* IN1L_ENA */;
pub const WM8994_IN1L_ENA_MASK: c_uint = 0x0040  /* IN1L_ENA */;

pub const WM8994_IN2R_ENA: c_uint = 0x0020  /* IN2R_ENA */;
pub const WM8994_IN2R_ENA_MASK: c_uint = 0x0020  /* IN2R_ENA */;

pub const WM8994_IN1R_ENA: c_uint = 0x0010  /* IN1R_ENA */;
pub const WM8994_IN1R_ENA_MASK: c_uint = 0x0010  /* IN1R_ENA */;

//
// R3 (0x03) - Power Management (3)
//
pub const WM8994_LINEOUT1N_ENA: c_uint = 0x2000  /* LINEOUT1N_ENA */;
pub const WM8994_LINEOUT1N_ENA_MASK: c_uint = 0x2000  /* LINEOUT1N_ENA */;

pub const WM8994_LINEOUT1P_ENA: c_uint = 0x1000  /* LINEOUT1P_ENA */;
pub const WM8994_LINEOUT1P_ENA_MASK: c_uint = 0x1000  /* LINEOUT1P_ENA */;

pub const WM8994_LINEOUT2N_ENA: c_uint = 0x0800  /* LINEOUT2N_ENA */;
pub const WM8994_LINEOUT2N_ENA_MASK: c_uint = 0x0800  /* LINEOUT2N_ENA */;

pub const WM8994_LINEOUT2P_ENA: c_uint = 0x0400  /* LINEOUT2P_ENA */;
pub const WM8994_LINEOUT2P_ENA_MASK: c_uint = 0x0400  /* LINEOUT2P_ENA */;

pub const WM8994_SPKRVOL_ENA: c_uint = 0x0200  /* SPKRVOL_ENA */;
pub const WM8994_SPKRVOL_ENA_MASK: c_uint = 0x0200  /* SPKRVOL_ENA */;

pub const WM8994_SPKLVOL_ENA: c_uint = 0x0100  /* SPKLVOL_ENA */;
pub const WM8994_SPKLVOL_ENA_MASK: c_uint = 0x0100  /* SPKLVOL_ENA */;

pub const WM8994_MIXOUTLVOL_ENA: c_uint = 0x0080  /* MIXOUTLVOL_ENA */;
pub const WM8994_MIXOUTLVOL_ENA_MASK: c_uint = 0x0080  /* MIXOUTLVOL_ENA */;

pub const WM8994_MIXOUTRVOL_ENA: c_uint = 0x0040  /* MIXOUTRVOL_ENA */;
pub const WM8994_MIXOUTRVOL_ENA_MASK: c_uint = 0x0040  /* MIXOUTRVOL_ENA */;

pub const WM8994_MIXOUTL_ENA: c_uint = 0x0020  /* MIXOUTL_ENA */;
pub const WM8994_MIXOUTL_ENA_MASK: c_uint = 0x0020  /* MIXOUTL_ENA */;

pub const WM8994_MIXOUTR_ENA: c_uint = 0x0010  /* MIXOUTR_ENA */;
pub const WM8994_MIXOUTR_ENA_MASK: c_uint = 0x0010  /* MIXOUTR_ENA */;

//
// R4 (0x04) - Power Management (4)
//
pub const WM8994_AIF2ADCL_ENA: c_uint = 0x2000  /* AIF2ADCL_ENA */;
pub const WM8994_AIF2ADCL_ENA_MASK: c_uint = 0x2000  /* AIF2ADCL_ENA */;

pub const WM8994_AIF2ADCR_ENA: c_uint = 0x1000  /* AIF2ADCR_ENA */;
pub const WM8994_AIF2ADCR_ENA_MASK: c_uint = 0x1000  /* AIF2ADCR_ENA */;

pub const WM8994_AIF1ADC2L_ENA: c_uint = 0x0800  /* AIF1ADC2L_ENA */;
pub const WM8994_AIF1ADC2L_ENA_MASK: c_uint = 0x0800  /* AIF1ADC2L_ENA */;

pub const WM8994_AIF1ADC2R_ENA: c_uint = 0x0400  /* AIF1ADC2R_ENA */;
pub const WM8994_AIF1ADC2R_ENA_MASK: c_uint = 0x0400  /* AIF1ADC2R_ENA */;

pub const WM8994_AIF1ADC1L_ENA: c_uint = 0x0200  /* AIF1ADC1L_ENA */;
pub const WM8994_AIF1ADC1L_ENA_MASK: c_uint = 0x0200  /* AIF1ADC1L_ENA */;

pub const WM8994_AIF1ADC1R_ENA: c_uint = 0x0100  /* AIF1ADC1R_ENA */;
pub const WM8994_AIF1ADC1R_ENA_MASK: c_uint = 0x0100  /* AIF1ADC1R_ENA */;

pub const WM8994_DMIC2L_ENA: c_uint = 0x0020  /* DMIC2L_ENA */;
pub const WM8994_DMIC2L_ENA_MASK: c_uint = 0x0020  /* DMIC2L_ENA */;

pub const WM8994_DMIC2R_ENA: c_uint = 0x0010  /* DMIC2R_ENA */;
pub const WM8994_DMIC2R_ENA_MASK: c_uint = 0x0010  /* DMIC2R_ENA */;

pub const WM8994_DMIC1L_ENA: c_uint = 0x0008  /* DMIC1L_ENA */;
pub const WM8994_DMIC1L_ENA_MASK: c_uint = 0x0008  /* DMIC1L_ENA */;

pub const WM8994_DMIC1R_ENA: c_uint = 0x0004  /* DMIC1R_ENA */;
pub const WM8994_DMIC1R_ENA_MASK: c_uint = 0x0004  /* DMIC1R_ENA */;

pub const WM8994_ADCL_ENA: c_uint = 0x0002  /* ADCL_ENA */;
pub const WM8994_ADCL_ENA_MASK: c_uint = 0x0002  /* ADCL_ENA */;

pub const WM8994_ADCR_ENA: c_uint = 0x0001  /* ADCR_ENA */;
pub const WM8994_ADCR_ENA_MASK: c_uint = 0x0001  /* ADCR_ENA */;

//
// R5 (0x05) - Power Management (5)
//
pub const WM8994_AIF2DACL_ENA: c_uint = 0x2000  /* AIF2DACL_ENA */;
pub const WM8994_AIF2DACL_ENA_MASK: c_uint = 0x2000  /* AIF2DACL_ENA */;

pub const WM8994_AIF2DACR_ENA: c_uint = 0x1000  /* AIF2DACR_ENA */;
pub const WM8994_AIF2DACR_ENA_MASK: c_uint = 0x1000  /* AIF2DACR_ENA */;

pub const WM8994_AIF1DAC2L_ENA: c_uint = 0x0800  /* AIF1DAC2L_ENA */;
pub const WM8994_AIF1DAC2L_ENA_MASK: c_uint = 0x0800  /* AIF1DAC2L_ENA */;

pub const WM8994_AIF1DAC2R_ENA: c_uint = 0x0400  /* AIF1DAC2R_ENA */;
pub const WM8994_AIF1DAC2R_ENA_MASK: c_uint = 0x0400  /* AIF1DAC2R_ENA */;

pub const WM8994_AIF1DAC1L_ENA: c_uint = 0x0200  /* AIF1DAC1L_ENA */;
pub const WM8994_AIF1DAC1L_ENA_MASK: c_uint = 0x0200  /* AIF1DAC1L_ENA */;

pub const WM8994_AIF1DAC1R_ENA: c_uint = 0x0100  /* AIF1DAC1R_ENA */;
pub const WM8994_AIF1DAC1R_ENA_MASK: c_uint = 0x0100  /* AIF1DAC1R_ENA */;

pub const WM8994_DAC2L_ENA: c_uint = 0x0008  /* DAC2L_ENA */;
pub const WM8994_DAC2L_ENA_MASK: c_uint = 0x0008  /* DAC2L_ENA */;

pub const WM8994_DAC2R_ENA: c_uint = 0x0004  /* DAC2R_ENA */;
pub const WM8994_DAC2R_ENA_MASK: c_uint = 0x0004  /* DAC2R_ENA */;

pub const WM8994_DAC1L_ENA: c_uint = 0x0002  /* DAC1L_ENA */;
pub const WM8994_DAC1L_ENA_MASK: c_uint = 0x0002  /* DAC1L_ENA */;

pub const WM8994_DAC1R_ENA: c_uint = 0x0001  /* DAC1R_ENA */;
pub const WM8994_DAC1R_ENA_MASK: c_uint = 0x0001  /* DAC1R_ENA */;

//
// R6 (0x06) - Power Management (6)
//
pub const WM8958_AIF3ADC_SRC_MASK: c_uint = 0x0600  /* AIF3ADC_SRC - [10:9] */;

pub const WM8958_AIF2DAC_SRC_MASK: c_uint = 0x0180  /* AIF2DAC_SRC - [8:7] */;

pub const WM8994_AIF3_TRI: c_uint = 0x0020  /* AIF3_TRI */;
pub const WM8994_AIF3_TRI_MASK: c_uint = 0x0020  /* AIF3_TRI */;

pub const WM8994_AIF3_ADCDAT_SRC_MASK: c_uint = 0x0018  /* AIF3_ADCDAT_SRC - [4:3] */;

pub const WM8994_AIF2_ADCDAT_SRC: c_uint = 0x0004  /* AIF2_ADCDAT_SRC */;
pub const WM8994_AIF2_ADCDAT_SRC_MASK: c_uint = 0x0004  /* AIF2_ADCDAT_SRC */;

pub const WM8994_AIF2_DACDAT_SRC: c_uint = 0x0002  /* AIF2_DACDAT_SRC */;
pub const WM8994_AIF2_DACDAT_SRC_MASK: c_uint = 0x0002  /* AIF2_DACDAT_SRC */;

pub const WM8994_AIF1_DACDAT_SRC: c_uint = 0x0001  /* AIF1_DACDAT_SRC */;
pub const WM8994_AIF1_DACDAT_SRC_MASK: c_uint = 0x0001  /* AIF1_DACDAT_SRC */;

//
// R21 (0x15) - Input Mixer (1)
//
pub const WM8994_IN1RP_MIXINR_BOOST: c_uint = 0x0100  /* IN1RP_MIXINR_BOOST */;
pub const WM8994_IN1RP_MIXINR_BOOST_MASK: c_uint = 0x0100  /* IN1RP_MIXINR_BOOST */;

pub const WM8994_IN1LP_MIXINL_BOOST: c_uint = 0x0080  /* IN1LP_MIXINL_BOOST */;
pub const WM8994_IN1LP_MIXINL_BOOST_MASK: c_uint = 0x0080  /* IN1LP_MIXINL_BOOST */;

pub const WM8994_INPUTS_CLAMP: c_uint = 0x0040  /* INPUTS_CLAMP */;
pub const WM8994_INPUTS_CLAMP_MASK: c_uint = 0x0040  /* INPUTS_CLAMP */;

//
// R24 (0x18) - Left Line Input 1&2 Volume
//
pub const WM8994_IN1_VU: c_uint = 0x0100  /* IN1_VU */;
pub const WM8994_IN1_VU_MASK: c_uint = 0x0100  /* IN1_VU */;

pub const WM8994_IN1L_MUTE: c_uint = 0x0080  /* IN1L_MUTE */;
pub const WM8994_IN1L_MUTE_MASK: c_uint = 0x0080  /* IN1L_MUTE */;

pub const WM8994_IN1L_ZC: c_uint = 0x0040  /* IN1L_ZC */;
pub const WM8994_IN1L_ZC_MASK: c_uint = 0x0040  /* IN1L_ZC */;

pub const WM8994_IN1L_VOL_MASK: c_uint = 0x001F  /* IN1L_VOL - [4:0] */;

//
// R25 (0x19) - Left Line Input 3&4 Volume
//
pub const WM8994_IN2_VU: c_uint = 0x0100  /* IN2_VU */;
pub const WM8994_IN2_VU_MASK: c_uint = 0x0100  /* IN2_VU */;

pub const WM8994_IN2L_MUTE: c_uint = 0x0080  /* IN2L_MUTE */;
pub const WM8994_IN2L_MUTE_MASK: c_uint = 0x0080  /* IN2L_MUTE */;

pub const WM8994_IN2L_ZC: c_uint = 0x0040  /* IN2L_ZC */;
pub const WM8994_IN2L_ZC_MASK: c_uint = 0x0040  /* IN2L_ZC */;

pub const WM8994_IN2L_VOL_MASK: c_uint = 0x001F  /* IN2L_VOL - [4:0] */;

//
// R26 (0x1A) - Right Line Input 1&2 Volume
//
pub const WM8994_IN1_VU: c_uint = 0x0100  /* IN1_VU */;
pub const WM8994_IN1_VU_MASK: c_uint = 0x0100  /* IN1_VU */;

pub const WM8994_IN1R_MUTE: c_uint = 0x0080  /* IN1R_MUTE */;
pub const WM8994_IN1R_MUTE_MASK: c_uint = 0x0080  /* IN1R_MUTE */;

pub const WM8994_IN1R_ZC: c_uint = 0x0040  /* IN1R_ZC */;
pub const WM8994_IN1R_ZC_MASK: c_uint = 0x0040  /* IN1R_ZC */;

pub const WM8994_IN1R_VOL_MASK: c_uint = 0x001F  /* IN1R_VOL - [4:0] */;

//
// R27 (0x1B) - Right Line Input 3&4 Volume
//
pub const WM8994_IN2_VU: c_uint = 0x0100  /* IN2_VU */;
pub const WM8994_IN2_VU_MASK: c_uint = 0x0100  /* IN2_VU */;

pub const WM8994_IN2R_MUTE: c_uint = 0x0080  /* IN2R_MUTE */;
pub const WM8994_IN2R_MUTE_MASK: c_uint = 0x0080  /* IN2R_MUTE */;

pub const WM8994_IN2R_ZC: c_uint = 0x0040  /* IN2R_ZC */;
pub const WM8994_IN2R_ZC_MASK: c_uint = 0x0040  /* IN2R_ZC */;

pub const WM8994_IN2R_VOL_MASK: c_uint = 0x001F  /* IN2R_VOL - [4:0] */;

//
// R28 (0x1C) - Left Output Volume
//
pub const WM8994_HPOUT1_VU: c_uint = 0x0100  /* HPOUT1_VU */;
pub const WM8994_HPOUT1_VU_MASK: c_uint = 0x0100  /* HPOUT1_VU */;

pub const WM8994_HPOUT1L_ZC: c_uint = 0x0080  /* HPOUT1L_ZC */;
pub const WM8994_HPOUT1L_ZC_MASK: c_uint = 0x0080  /* HPOUT1L_ZC */;

pub const WM8994_HPOUT1L_MUTE_N: c_uint = 0x0040  /* HPOUT1L_MUTE_N */;
pub const WM8994_HPOUT1L_MUTE_N_MASK: c_uint = 0x0040  /* HPOUT1L_MUTE_N */;

pub const WM8994_HPOUT1L_VOL_MASK: c_uint = 0x003F  /* HPOUT1L_VOL - [5:0] */;

//
// R29 (0x1D) - Right Output Volume
//
pub const WM8994_HPOUT1_VU: c_uint = 0x0100  /* HPOUT1_VU */;
pub const WM8994_HPOUT1_VU_MASK: c_uint = 0x0100  /* HPOUT1_VU */;

pub const WM8994_HPOUT1R_ZC: c_uint = 0x0080  /* HPOUT1R_ZC */;
pub const WM8994_HPOUT1R_ZC_MASK: c_uint = 0x0080  /* HPOUT1R_ZC */;

pub const WM8994_HPOUT1R_MUTE_N: c_uint = 0x0040  /* HPOUT1R_MUTE_N */;
pub const WM8994_HPOUT1R_MUTE_N_MASK: c_uint = 0x0040  /* HPOUT1R_MUTE_N */;

pub const WM8994_HPOUT1R_VOL_MASK: c_uint = 0x003F  /* HPOUT1R_VOL - [5:0] */;

//
// R30 (0x1E) - Line Outputs Volume
//
pub const WM8994_LINEOUT1N_MUTE: c_uint = 0x0040  /* LINEOUT1N_MUTE */;
pub const WM8994_LINEOUT1N_MUTE_MASK: c_uint = 0x0040  /* LINEOUT1N_MUTE */;

pub const WM8994_LINEOUT1P_MUTE: c_uint = 0x0020  /* LINEOUT1P_MUTE */;
pub const WM8994_LINEOUT1P_MUTE_MASK: c_uint = 0x0020  /* LINEOUT1P_MUTE */;

pub const WM8994_LINEOUT1_VOL: c_uint = 0x0010  /* LINEOUT1_VOL */;
pub const WM8994_LINEOUT1_VOL_MASK: c_uint = 0x0010  /* LINEOUT1_VOL */;

pub const WM8994_LINEOUT2N_MUTE: c_uint = 0x0004  /* LINEOUT2N_MUTE */;
pub const WM8994_LINEOUT2N_MUTE_MASK: c_uint = 0x0004  /* LINEOUT2N_MUTE */;

pub const WM8994_LINEOUT2P_MUTE: c_uint = 0x0002  /* LINEOUT2P_MUTE */;
pub const WM8994_LINEOUT2P_MUTE_MASK: c_uint = 0x0002  /* LINEOUT2P_MUTE */;

pub const WM8994_LINEOUT2_VOL: c_uint = 0x0001  /* LINEOUT2_VOL */;
pub const WM8994_LINEOUT2_VOL_MASK: c_uint = 0x0001  /* LINEOUT2_VOL */;

//
// R31 (0x1F) - HPOUT2 Volume
//
pub const WM8994_HPOUT2_MUTE: c_uint = 0x0020  /* HPOUT2_MUTE */;
pub const WM8994_HPOUT2_MUTE_MASK: c_uint = 0x0020  /* HPOUT2_MUTE */;

pub const WM8994_HPOUT2_VOL: c_uint = 0x0010  /* HPOUT2_VOL */;
pub const WM8994_HPOUT2_VOL_MASK: c_uint = 0x0010  /* HPOUT2_VOL */;

//
// R32 (0x20) - Left OPGA Volume
//
pub const WM8994_MIXOUT_VU: c_uint = 0x0100  /* MIXOUT_VU */;
pub const WM8994_MIXOUT_VU_MASK: c_uint = 0x0100  /* MIXOUT_VU */;

pub const WM8994_MIXOUTL_ZC: c_uint = 0x0080  /* MIXOUTL_ZC */;
pub const WM8994_MIXOUTL_ZC_MASK: c_uint = 0x0080  /* MIXOUTL_ZC */;

pub const WM8994_MIXOUTL_MUTE_N: c_uint = 0x0040  /* MIXOUTL_MUTE_N */;
pub const WM8994_MIXOUTL_MUTE_N_MASK: c_uint = 0x0040  /* MIXOUTL_MUTE_N */;

pub const WM8994_MIXOUTL_VOL_MASK: c_uint = 0x003F  /* MIXOUTL_VOL - [5:0] */;

//
// R33 (0x21) - Right OPGA Volume
//
pub const WM8994_MIXOUT_VU: c_uint = 0x0100  /* MIXOUT_VU */;
pub const WM8994_MIXOUT_VU_MASK: c_uint = 0x0100  /* MIXOUT_VU */;

pub const WM8994_MIXOUTR_ZC: c_uint = 0x0080  /* MIXOUTR_ZC */;
pub const WM8994_MIXOUTR_ZC_MASK: c_uint = 0x0080  /* MIXOUTR_ZC */;

pub const WM8994_MIXOUTR_MUTE_N: c_uint = 0x0040  /* MIXOUTR_MUTE_N */;
pub const WM8994_MIXOUTR_MUTE_N_MASK: c_uint = 0x0040  /* MIXOUTR_MUTE_N */;

pub const WM8994_MIXOUTR_VOL_MASK: c_uint = 0x003F  /* MIXOUTR_VOL - [5:0] */;

//
// R34 (0x22) - SPKMIXL Attenuation
//
pub const WM8994_DAC2L_SPKMIXL_VOL: c_uint = 0x0040  /* DAC2L_SPKMIXL_VOL */;
pub const WM8994_DAC2L_SPKMIXL_VOL_MASK: c_uint = 0x0040  /* DAC2L_SPKMIXL_VOL */;

pub const WM8994_MIXINL_SPKMIXL_VOL: c_uint = 0x0020  /* MIXINL_SPKMIXL_VOL */;
pub const WM8994_MIXINL_SPKMIXL_VOL_MASK: c_uint = 0x0020  /* MIXINL_SPKMIXL_VOL */;

pub const WM8994_IN1LP_SPKMIXL_VOL: c_uint = 0x0010  /* IN1LP_SPKMIXL_VOL */;
pub const WM8994_IN1LP_SPKMIXL_VOL_MASK: c_uint = 0x0010  /* IN1LP_SPKMIXL_VOL */;

pub const WM8994_MIXOUTL_SPKMIXL_VOL: c_uint = 0x0008  /* MIXOUTL_SPKMIXL_VOL */;
pub const WM8994_MIXOUTL_SPKMIXL_VOL_MASK: c_uint = 0x0008  /* MIXOUTL_SPKMIXL_VOL */;

pub const WM8994_DAC1L_SPKMIXL_VOL: c_uint = 0x0004  /* DAC1L_SPKMIXL_VOL */;
pub const WM8994_DAC1L_SPKMIXL_VOL_MASK: c_uint = 0x0004  /* DAC1L_SPKMIXL_VOL */;

pub const WM8994_SPKMIXL_VOL_MASK: c_uint = 0x0003  /* SPKMIXL_VOL - [1:0] */;

//
// R35 (0x23) - SPKMIXR Attenuation
//
pub const WM8994_SPKOUT_CLASSAB: c_uint = 0x0100  /* SPKOUT_CLASSAB */;
pub const WM8994_SPKOUT_CLASSAB_MASK: c_uint = 0x0100  /* SPKOUT_CLASSAB */;

pub const WM8994_DAC2R_SPKMIXR_VOL: c_uint = 0x0040  /* DAC2R_SPKMIXR_VOL */;
pub const WM8994_DAC2R_SPKMIXR_VOL_MASK: c_uint = 0x0040  /* DAC2R_SPKMIXR_VOL */;

pub const WM8994_MIXINR_SPKMIXR_VOL: c_uint = 0x0020  /* MIXINR_SPKMIXR_VOL */;
pub const WM8994_MIXINR_SPKMIXR_VOL_MASK: c_uint = 0x0020  /* MIXINR_SPKMIXR_VOL */;

pub const WM8994_IN1RP_SPKMIXR_VOL: c_uint = 0x0010  /* IN1RP_SPKMIXR_VOL */;
pub const WM8994_IN1RP_SPKMIXR_VOL_MASK: c_uint = 0x0010  /* IN1RP_SPKMIXR_VOL */;

pub const WM8994_MIXOUTR_SPKMIXR_VOL: c_uint = 0x0008  /* MIXOUTR_SPKMIXR_VOL */;
pub const WM8994_MIXOUTR_SPKMIXR_VOL_MASK: c_uint = 0x0008  /* MIXOUTR_SPKMIXR_VOL */;

pub const WM8994_DAC1R_SPKMIXR_VOL: c_uint = 0x0004  /* DAC1R_SPKMIXR_VOL */;
pub const WM8994_DAC1R_SPKMIXR_VOL_MASK: c_uint = 0x0004  /* DAC1R_SPKMIXR_VOL */;

pub const WM8994_SPKMIXR_VOL_MASK: c_uint = 0x0003  /* SPKMIXR_VOL - [1:0] */;

//
// R36 (0x24) - SPKOUT Mixers
//
pub const WM8994_IN2LRP_TO_SPKOUTL: c_uint = 0x0020  /* IN2LRP_TO_SPKOUTL */;
pub const WM8994_IN2LRP_TO_SPKOUTL_MASK: c_uint = 0x0020  /* IN2LRP_TO_SPKOUTL */;

pub const WM8994_SPKMIXL_TO_SPKOUTL: c_uint = 0x0010  /* SPKMIXL_TO_SPKOUTL */;
pub const WM8994_SPKMIXL_TO_SPKOUTL_MASK: c_uint = 0x0010  /* SPKMIXL_TO_SPKOUTL */;

pub const WM8994_SPKMIXR_TO_SPKOUTL: c_uint = 0x0008  /* SPKMIXR_TO_SPKOUTL */;
pub const WM8994_SPKMIXR_TO_SPKOUTL_MASK: c_uint = 0x0008  /* SPKMIXR_TO_SPKOUTL */;

pub const WM8994_IN2LRP_TO_SPKOUTR: c_uint = 0x0004  /* IN2LRP_TO_SPKOUTR */;
pub const WM8994_IN2LRP_TO_SPKOUTR_MASK: c_uint = 0x0004  /* IN2LRP_TO_SPKOUTR */;

pub const WM8994_SPKMIXL_TO_SPKOUTR: c_uint = 0x0002  /* SPKMIXL_TO_SPKOUTR */;
pub const WM8994_SPKMIXL_TO_SPKOUTR_MASK: c_uint = 0x0002  /* SPKMIXL_TO_SPKOUTR */;

pub const WM8994_SPKMIXR_TO_SPKOUTR: c_uint = 0x0001  /* SPKMIXR_TO_SPKOUTR */;
pub const WM8994_SPKMIXR_TO_SPKOUTR_MASK: c_uint = 0x0001  /* SPKMIXR_TO_SPKOUTR */;

//
// R37 (0x25) - ClassD
//
pub const WM8994_SPKOUTL_BOOST_MASK: c_uint = 0x0038  /* SPKOUTL_BOOST - [5:3] */;

pub const WM8994_SPKOUTR_BOOST_MASK: c_uint = 0x0007  /* SPKOUTR_BOOST - [2:0] */;

//
// R38 (0x26) - Speaker Volume Left
//
pub const WM8994_SPKOUT_VU: c_uint = 0x0100  /* SPKOUT_VU */;
pub const WM8994_SPKOUT_VU_MASK: c_uint = 0x0100  /* SPKOUT_VU */;

pub const WM8994_SPKOUTL_ZC: c_uint = 0x0080  /* SPKOUTL_ZC */;
pub const WM8994_SPKOUTL_ZC_MASK: c_uint = 0x0080  /* SPKOUTL_ZC */;

pub const WM8994_SPKOUTL_MUTE_N: c_uint = 0x0040  /* SPKOUTL_MUTE_N */;
pub const WM8994_SPKOUTL_MUTE_N_MASK: c_uint = 0x0040  /* SPKOUTL_MUTE_N */;

pub const WM8994_SPKOUTL_VOL_MASK: c_uint = 0x003F  /* SPKOUTL_VOL - [5:0] */;

//
// R39 (0x27) - Speaker Volume Right
//
pub const WM8994_SPKOUT_VU: c_uint = 0x0100  /* SPKOUT_VU */;
pub const WM8994_SPKOUT_VU_MASK: c_uint = 0x0100  /* SPKOUT_VU */;

pub const WM8994_SPKOUTR_ZC: c_uint = 0x0080  /* SPKOUTR_ZC */;
pub const WM8994_SPKOUTR_ZC_MASK: c_uint = 0x0080  /* SPKOUTR_ZC */;

pub const WM8994_SPKOUTR_MUTE_N: c_uint = 0x0040  /* SPKOUTR_MUTE_N */;
pub const WM8994_SPKOUTR_MUTE_N_MASK: c_uint = 0x0040  /* SPKOUTR_MUTE_N */;

pub const WM8994_SPKOUTR_VOL_MASK: c_uint = 0x003F  /* SPKOUTR_VOL - [5:0] */;

//
// R40 (0x28) - Input Mixer (2)
//
pub const WM8994_IN2LP_TO_IN2L: c_uint = 0x0080  /* IN2LP_TO_IN2L */;
pub const WM8994_IN2LP_TO_IN2L_MASK: c_uint = 0x0080  /* IN2LP_TO_IN2L */;

pub const WM8994_IN2LN_TO_IN2L: c_uint = 0x0040  /* IN2LN_TO_IN2L */;
pub const WM8994_IN2LN_TO_IN2L_MASK: c_uint = 0x0040  /* IN2LN_TO_IN2L */;

pub const WM8994_IN1LP_TO_IN1L: c_uint = 0x0020  /* IN1LP_TO_IN1L */;
pub const WM8994_IN1LP_TO_IN1L_MASK: c_uint = 0x0020  /* IN1LP_TO_IN1L */;

pub const WM8994_IN1LN_TO_IN1L: c_uint = 0x0010  /* IN1LN_TO_IN1L */;
pub const WM8994_IN1LN_TO_IN1L_MASK: c_uint = 0x0010  /* IN1LN_TO_IN1L */;

pub const WM8994_IN2RP_TO_IN2R: c_uint = 0x0008  /* IN2RP_TO_IN2R */;
pub const WM8994_IN2RP_TO_IN2R_MASK: c_uint = 0x0008  /* IN2RP_TO_IN2R */;

pub const WM8994_IN2RN_TO_IN2R: c_uint = 0x0004  /* IN2RN_TO_IN2R */;
pub const WM8994_IN2RN_TO_IN2R_MASK: c_uint = 0x0004  /* IN2RN_TO_IN2R */;

pub const WM8994_IN1RP_TO_IN1R: c_uint = 0x0002  /* IN1RP_TO_IN1R */;
pub const WM8994_IN1RP_TO_IN1R_MASK: c_uint = 0x0002  /* IN1RP_TO_IN1R */;

pub const WM8994_IN1RN_TO_IN1R: c_uint = 0x0001  /* IN1RN_TO_IN1R */;
pub const WM8994_IN1RN_TO_IN1R_MASK: c_uint = 0x0001  /* IN1RN_TO_IN1R */;

//
// R41 (0x29) - Input Mixer (3)
//
pub const WM8994_IN2L_TO_MIXINL: c_uint = 0x0100  /* IN2L_TO_MIXINL */;
pub const WM8994_IN2L_TO_MIXINL_MASK: c_uint = 0x0100  /* IN2L_TO_MIXINL */;

pub const WM8994_IN2L_MIXINL_VOL: c_uint = 0x0080  /* IN2L_MIXINL_VOL */;
pub const WM8994_IN2L_MIXINL_VOL_MASK: c_uint = 0x0080  /* IN2L_MIXINL_VOL */;

pub const WM8994_IN1L_TO_MIXINL: c_uint = 0x0020  /* IN1L_TO_MIXINL */;
pub const WM8994_IN1L_TO_MIXINL_MASK: c_uint = 0x0020  /* IN1L_TO_MIXINL */;

pub const WM8994_IN1L_MIXINL_VOL: c_uint = 0x0010  /* IN1L_MIXINL_VOL */;
pub const WM8994_IN1L_MIXINL_VOL_MASK: c_uint = 0x0010  /* IN1L_MIXINL_VOL */;

pub const WM8994_MIXOUTL_MIXINL_VOL_MASK: c_uint = 0x0007  /* MIXOUTL_MIXINL_VOL - [2:0] */;

//
// R42 (0x2A) - Input Mixer (4)
//
pub const WM8994_IN2R_TO_MIXINR: c_uint = 0x0100  /* IN2R_TO_MIXINR */;
pub const WM8994_IN2R_TO_MIXINR_MASK: c_uint = 0x0100  /* IN2R_TO_MIXINR */;

pub const WM8994_IN2R_MIXINR_VOL: c_uint = 0x0080  /* IN2R_MIXINR_VOL */;
pub const WM8994_IN2R_MIXINR_VOL_MASK: c_uint = 0x0080  /* IN2R_MIXINR_VOL */;

pub const WM8994_IN1R_TO_MIXINR: c_uint = 0x0020  /* IN1R_TO_MIXINR */;
pub const WM8994_IN1R_TO_MIXINR_MASK: c_uint = 0x0020  /* IN1R_TO_MIXINR */;

pub const WM8994_IN1R_MIXINR_VOL: c_uint = 0x0010  /* IN1R_MIXINR_VOL */;
pub const WM8994_IN1R_MIXINR_VOL_MASK: c_uint = 0x0010  /* IN1R_MIXINR_VOL */;

pub const WM8994_MIXOUTR_MIXINR_VOL_MASK: c_uint = 0x0007  /* MIXOUTR_MIXINR_VOL - [2:0] */;

//
// R43 (0x2B) - Input Mixer (5)
//
pub const WM8994_IN1LP_MIXINL_VOL_MASK: c_uint = 0x01C0  /* IN1LP_MIXINL_VOL - [8:6] */;

pub const WM8994_IN2LRP_MIXINL_VOL_MASK: c_uint = 0x0007  /* IN2LRP_MIXINL_VOL - [2:0] */;

//
// R44 (0x2C) - Input Mixer (6)
//
pub const WM8994_IN1RP_MIXINR_VOL_MASK: c_uint = 0x01C0  /* IN1RP_MIXINR_VOL - [8:6] */;

pub const WM8994_IN2LRP_MIXINR_VOL_MASK: c_uint = 0x0007  /* IN2LRP_MIXINR_VOL - [2:0] */;

//
// R45 (0x2D) - Output Mixer (1)
//
pub const WM8994_DAC1L_TO_HPOUT1L: c_uint = 0x0100  /* DAC1L_TO_HPOUT1L */;
pub const WM8994_DAC1L_TO_HPOUT1L_MASK: c_uint = 0x0100  /* DAC1L_TO_HPOUT1L */;

pub const WM8994_MIXINR_TO_MIXOUTL: c_uint = 0x0080  /* MIXINR_TO_MIXOUTL */;
pub const WM8994_MIXINR_TO_MIXOUTL_MASK: c_uint = 0x0080  /* MIXINR_TO_MIXOUTL */;

pub const WM8994_MIXINL_TO_MIXOUTL: c_uint = 0x0040  /* MIXINL_TO_MIXOUTL */;
pub const WM8994_MIXINL_TO_MIXOUTL_MASK: c_uint = 0x0040  /* MIXINL_TO_MIXOUTL */;

pub const WM8994_IN2RN_TO_MIXOUTL: c_uint = 0x0020  /* IN2RN_TO_MIXOUTL */;
pub const WM8994_IN2RN_TO_MIXOUTL_MASK: c_uint = 0x0020  /* IN2RN_TO_MIXOUTL */;

pub const WM8994_IN2LN_TO_MIXOUTL: c_uint = 0x0010  /* IN2LN_TO_MIXOUTL */;
pub const WM8994_IN2LN_TO_MIXOUTL_MASK: c_uint = 0x0010  /* IN2LN_TO_MIXOUTL */;

pub const WM8994_IN1R_TO_MIXOUTL: c_uint = 0x0008  /* IN1R_TO_MIXOUTL */;
pub const WM8994_IN1R_TO_MIXOUTL_MASK: c_uint = 0x0008  /* IN1R_TO_MIXOUTL */;

pub const WM8994_IN1L_TO_MIXOUTL: c_uint = 0x0004  /* IN1L_TO_MIXOUTL */;
pub const WM8994_IN1L_TO_MIXOUTL_MASK: c_uint = 0x0004  /* IN1L_TO_MIXOUTL */;

pub const WM8994_IN2LP_TO_MIXOUTL: c_uint = 0x0002  /* IN2LP_TO_MIXOUTL */;
pub const WM8994_IN2LP_TO_MIXOUTL_MASK: c_uint = 0x0002  /* IN2LP_TO_MIXOUTL */;

pub const WM8994_DAC1L_TO_MIXOUTL: c_uint = 0x0001  /* DAC1L_TO_MIXOUTL */;
pub const WM8994_DAC1L_TO_MIXOUTL_MASK: c_uint = 0x0001  /* DAC1L_TO_MIXOUTL */;

//
// R46 (0x2E) - Output Mixer (2)
//
pub const WM8994_DAC1R_TO_HPOUT1R: c_uint = 0x0100  /* DAC1R_TO_HPOUT1R */;
pub const WM8994_DAC1R_TO_HPOUT1R_MASK: c_uint = 0x0100  /* DAC1R_TO_HPOUT1R */;

pub const WM8994_MIXINL_TO_MIXOUTR: c_uint = 0x0080  /* MIXINL_TO_MIXOUTR */;
pub const WM8994_MIXINL_TO_MIXOUTR_MASK: c_uint = 0x0080  /* MIXINL_TO_MIXOUTR */;

pub const WM8994_MIXINR_TO_MIXOUTR: c_uint = 0x0040  /* MIXINR_TO_MIXOUTR */;
pub const WM8994_MIXINR_TO_MIXOUTR_MASK: c_uint = 0x0040  /* MIXINR_TO_MIXOUTR */;

pub const WM8994_IN2LN_TO_MIXOUTR: c_uint = 0x0020  /* IN2LN_TO_MIXOUTR */;
pub const WM8994_IN2LN_TO_MIXOUTR_MASK: c_uint = 0x0020  /* IN2LN_TO_MIXOUTR */;

pub const WM8994_IN2RN_TO_MIXOUTR: c_uint = 0x0010  /* IN2RN_TO_MIXOUTR */;
pub const WM8994_IN2RN_TO_MIXOUTR_MASK: c_uint = 0x0010  /* IN2RN_TO_MIXOUTR */;

pub const WM8994_IN1L_TO_MIXOUTR: c_uint = 0x0008  /* IN1L_TO_MIXOUTR */;
pub const WM8994_IN1L_TO_MIXOUTR_MASK: c_uint = 0x0008  /* IN1L_TO_MIXOUTR */;

pub const WM8994_IN1R_TO_MIXOUTR: c_uint = 0x0004  /* IN1R_TO_MIXOUTR */;
pub const WM8994_IN1R_TO_MIXOUTR_MASK: c_uint = 0x0004  /* IN1R_TO_MIXOUTR */;

pub const WM8994_IN2RP_TO_MIXOUTR: c_uint = 0x0002  /* IN2RP_TO_MIXOUTR */;
pub const WM8994_IN2RP_TO_MIXOUTR_MASK: c_uint = 0x0002  /* IN2RP_TO_MIXOUTR */;

pub const WM8994_DAC1R_TO_MIXOUTR: c_uint = 0x0001  /* DAC1R_TO_MIXOUTR */;
pub const WM8994_DAC1R_TO_MIXOUTR_MASK: c_uint = 0x0001  /* DAC1R_TO_MIXOUTR */;

//
// R47 (0x2F) - Output Mixer (3)
//
pub const WM8994_IN2LP_MIXOUTL_VOL_MASK: c_uint = 0x0E00  /* IN2LP_MIXOUTL_VOL - [11:9] */;

pub const WM8994_IN2LN_MIXOUTL_VOL_MASK: c_uint = 0x01C0  /* IN2LN_MIXOUTL_VOL - [8:6] */;

pub const WM8994_IN1R_MIXOUTL_VOL_MASK: c_uint = 0x0038  /* IN1R_MIXOUTL_VOL - [5:3] */;

pub const WM8994_IN1L_MIXOUTL_VOL_MASK: c_uint = 0x0007  /* IN1L_MIXOUTL_VOL - [2:0] */;

//
// R48 (0x30) - Output Mixer (4)
//
pub const WM8994_IN2RP_MIXOUTR_VOL_MASK: c_uint = 0x0E00  /* IN2RP_MIXOUTR_VOL - [11:9] */;

pub const WM8994_IN2RN_MIXOUTR_VOL_MASK: c_uint = 0x01C0  /* IN2RN_MIXOUTR_VOL - [8:6] */;

pub const WM8994_IN1L_MIXOUTR_VOL_MASK: c_uint = 0x0038  /* IN1L_MIXOUTR_VOL - [5:3] */;

pub const WM8994_IN1R_MIXOUTR_VOL_MASK: c_uint = 0x0007  /* IN1R_MIXOUTR_VOL - [2:0] */;

//
// R49 (0x31) - Output Mixer (5)
//
pub const WM8994_DAC1L_MIXOUTL_VOL_MASK: c_uint = 0x0E00  /* DAC1L_MIXOUTL_VOL - [11:9] */;

pub const WM8994_IN2RN_MIXOUTL_VOL_MASK: c_uint = 0x01C0  /* IN2RN_MIXOUTL_VOL - [8:6] */;

pub const WM8994_MIXINR_MIXOUTL_VOL_MASK: c_uint = 0x0038  /* MIXINR_MIXOUTL_VOL - [5:3] */;

pub const WM8994_MIXINL_MIXOUTL_VOL_MASK: c_uint = 0x0007  /* MIXINL_MIXOUTL_VOL - [2:0] */;

//
// R50 (0x32) - Output Mixer (6)
//
pub const WM8994_DAC1R_MIXOUTR_VOL_MASK: c_uint = 0x0E00  /* DAC1R_MIXOUTR_VOL - [11:9] */;

pub const WM8994_IN2LN_MIXOUTR_VOL_MASK: c_uint = 0x01C0  /* IN2LN_MIXOUTR_VOL - [8:6] */;

pub const WM8994_MIXINL_MIXOUTR_VOL_MASK: c_uint = 0x0038  /* MIXINL_MIXOUTR_VOL - [5:3] */;

pub const WM8994_MIXINR_MIXOUTR_VOL_MASK: c_uint = 0x0007  /* MIXINR_MIXOUTR_VOL - [2:0] */;

//
// R51 (0x33) - HPOUT2 Mixer
//
pub const WM8994_IN2LRP_TO_HPOUT2: c_uint = 0x0020  /* IN2LRP_TO_HPOUT2 */;
pub const WM8994_IN2LRP_TO_HPOUT2_MASK: c_uint = 0x0020  /* IN2LRP_TO_HPOUT2 */;

pub const WM8994_MIXOUTLVOL_TO_HPOUT2: c_uint = 0x0010  /* MIXOUTLVOL_TO_HPOUT2 */;
pub const WM8994_MIXOUTLVOL_TO_HPOUT2_MASK: c_uint = 0x0010  /* MIXOUTLVOL_TO_HPOUT2 */;

pub const WM8994_MIXOUTRVOL_TO_HPOUT2: c_uint = 0x0008  /* MIXOUTRVOL_TO_HPOUT2 */;
pub const WM8994_MIXOUTRVOL_TO_HPOUT2_MASK: c_uint = 0x0008  /* MIXOUTRVOL_TO_HPOUT2 */;

//
// R52 (0x34) - Line Mixer (1)
//
pub const WM8994_MIXOUTL_TO_LINEOUT1N: c_uint = 0x0040  /* MIXOUTL_TO_LINEOUT1N */;
pub const WM8994_MIXOUTL_TO_LINEOUT1N_MASK: c_uint = 0x0040  /* MIXOUTL_TO_LINEOUT1N */;

pub const WM8994_MIXOUTR_TO_LINEOUT1N: c_uint = 0x0020  /* MIXOUTR_TO_LINEOUT1N */;
pub const WM8994_MIXOUTR_TO_LINEOUT1N_MASK: c_uint = 0x0020  /* MIXOUTR_TO_LINEOUT1N */;

pub const WM8994_LINEOUT1_MODE: c_uint = 0x0010  /* LINEOUT1_MODE */;
pub const WM8994_LINEOUT1_MODE_MASK: c_uint = 0x0010  /* LINEOUT1_MODE */;

pub const WM8994_IN1R_TO_LINEOUT1P: c_uint = 0x0004  /* IN1R_TO_LINEOUT1P */;
pub const WM8994_IN1R_TO_LINEOUT1P_MASK: c_uint = 0x0004  /* IN1R_TO_LINEOUT1P */;

pub const WM8994_IN1L_TO_LINEOUT1P: c_uint = 0x0002  /* IN1L_TO_LINEOUT1P */;
pub const WM8994_IN1L_TO_LINEOUT1P_MASK: c_uint = 0x0002  /* IN1L_TO_LINEOUT1P */;

pub const WM8994_MIXOUTL_TO_LINEOUT1P: c_uint = 0x0001  /* MIXOUTL_TO_LINEOUT1P */;
pub const WM8994_MIXOUTL_TO_LINEOUT1P_MASK: c_uint = 0x0001  /* MIXOUTL_TO_LINEOUT1P */;

//
// R53 (0x35) - Line Mixer (2)
//
pub const WM8994_MIXOUTR_TO_LINEOUT2N: c_uint = 0x0040  /* MIXOUTR_TO_LINEOUT2N */;
pub const WM8994_MIXOUTR_TO_LINEOUT2N_MASK: c_uint = 0x0040  /* MIXOUTR_TO_LINEOUT2N */;

pub const WM8994_MIXOUTL_TO_LINEOUT2N: c_uint = 0x0020  /* MIXOUTL_TO_LINEOUT2N */;
pub const WM8994_MIXOUTL_TO_LINEOUT2N_MASK: c_uint = 0x0020  /* MIXOUTL_TO_LINEOUT2N */;

pub const WM8994_LINEOUT2_MODE: c_uint = 0x0010  /* LINEOUT2_MODE */;
pub const WM8994_LINEOUT2_MODE_MASK: c_uint = 0x0010  /* LINEOUT2_MODE */;

pub const WM8994_IN1L_TO_LINEOUT2P: c_uint = 0x0004  /* IN1L_TO_LINEOUT2P */;
pub const WM8994_IN1L_TO_LINEOUT2P_MASK: c_uint = 0x0004  /* IN1L_TO_LINEOUT2P */;

pub const WM8994_IN1R_TO_LINEOUT2P: c_uint = 0x0002  /* IN1R_TO_LINEOUT2P */;
pub const WM8994_IN1R_TO_LINEOUT2P_MASK: c_uint = 0x0002  /* IN1R_TO_LINEOUT2P */;

pub const WM8994_MIXOUTR_TO_LINEOUT2P: c_uint = 0x0001  /* MIXOUTR_TO_LINEOUT2P */;
pub const WM8994_MIXOUTR_TO_LINEOUT2P_MASK: c_uint = 0x0001  /* MIXOUTR_TO_LINEOUT2P */;

//
// R54 (0x36) - Speaker Mixer
//
pub const WM8994_DAC2L_TO_SPKMIXL: c_uint = 0x0200  /* DAC2L_TO_SPKMIXL */;
pub const WM8994_DAC2L_TO_SPKMIXL_MASK: c_uint = 0x0200  /* DAC2L_TO_SPKMIXL */;

pub const WM8994_DAC2R_TO_SPKMIXR: c_uint = 0x0100  /* DAC2R_TO_SPKMIXR */;
pub const WM8994_DAC2R_TO_SPKMIXR_MASK: c_uint = 0x0100  /* DAC2R_TO_SPKMIXR */;

pub const WM8994_MIXINL_TO_SPKMIXL: c_uint = 0x0080  /* MIXINL_TO_SPKMIXL */;
pub const WM8994_MIXINL_TO_SPKMIXL_MASK: c_uint = 0x0080  /* MIXINL_TO_SPKMIXL */;

pub const WM8994_MIXINR_TO_SPKMIXR: c_uint = 0x0040  /* MIXINR_TO_SPKMIXR */;
pub const WM8994_MIXINR_TO_SPKMIXR_MASK: c_uint = 0x0040  /* MIXINR_TO_SPKMIXR */;

pub const WM8994_IN1LP_TO_SPKMIXL: c_uint = 0x0020  /* IN1LP_TO_SPKMIXL */;
pub const WM8994_IN1LP_TO_SPKMIXL_MASK: c_uint = 0x0020  /* IN1LP_TO_SPKMIXL */;

pub const WM8994_IN1RP_TO_SPKMIXR: c_uint = 0x0010  /* IN1RP_TO_SPKMIXR */;
pub const WM8994_IN1RP_TO_SPKMIXR_MASK: c_uint = 0x0010  /* IN1RP_TO_SPKMIXR */;

pub const WM8994_MIXOUTL_TO_SPKMIXL: c_uint = 0x0008  /* MIXOUTL_TO_SPKMIXL */;
pub const WM8994_MIXOUTL_TO_SPKMIXL_MASK: c_uint = 0x0008  /* MIXOUTL_TO_SPKMIXL */;

pub const WM8994_MIXOUTR_TO_SPKMIXR: c_uint = 0x0004  /* MIXOUTR_TO_SPKMIXR */;
pub const WM8994_MIXOUTR_TO_SPKMIXR_MASK: c_uint = 0x0004  /* MIXOUTR_TO_SPKMIXR */;

pub const WM8994_DAC1L_TO_SPKMIXL: c_uint = 0x0002  /* DAC1L_TO_SPKMIXL */;
pub const WM8994_DAC1L_TO_SPKMIXL_MASK: c_uint = 0x0002  /* DAC1L_TO_SPKMIXL */;

pub const WM8994_DAC1R_TO_SPKMIXR: c_uint = 0x0001  /* DAC1R_TO_SPKMIXR */;
pub const WM8994_DAC1R_TO_SPKMIXR_MASK: c_uint = 0x0001  /* DAC1R_TO_SPKMIXR */;

//
// R55 (0x37) - Additional Control
//
pub const WM8994_LINEOUT1_FB: c_uint = 0x0080  /* LINEOUT1_FB */;
pub const WM8994_LINEOUT1_FB_MASK: c_uint = 0x0080  /* LINEOUT1_FB */;

pub const WM8994_LINEOUT2_FB: c_uint = 0x0040  /* LINEOUT2_FB */;
pub const WM8994_LINEOUT2_FB_MASK: c_uint = 0x0040  /* LINEOUT2_FB */;

pub const WM8994_VROI: c_uint = 0x0001  /* VROI */;
pub const WM8994_VROI_MASK: c_uint = 0x0001  /* VROI */;

//
// R56 (0x38) - AntiPOP (1)
//
pub const WM8994_LINEOUT_VMID_BUF_ENA: c_uint = 0x0080  /* LINEOUT_VMID_BUF_ENA */;
pub const WM8994_LINEOUT_VMID_BUF_ENA_MASK: c_uint = 0x0080  /* LINEOUT_VMID_BUF_ENA */;

pub const WM8994_HPOUT2_IN_ENA: c_uint = 0x0040  /* HPOUT2_IN_ENA */;
pub const WM8994_HPOUT2_IN_ENA_MASK: c_uint = 0x0040  /* HPOUT2_IN_ENA */;

pub const WM8994_LINEOUT1_DISCH: c_uint = 0x0020  /* LINEOUT1_DISCH */;
pub const WM8994_LINEOUT1_DISCH_MASK: c_uint = 0x0020  /* LINEOUT1_DISCH */;

pub const WM8994_LINEOUT2_DISCH: c_uint = 0x0010  /* LINEOUT2_DISCH */;
pub const WM8994_LINEOUT2_DISCH_MASK: c_uint = 0x0010  /* LINEOUT2_DISCH */;

//
// R57 (0x39) - AntiPOP (2)
//
pub const WM1811_JACKDET_MODE_MASK: c_uint = 0x0180  /* JACKDET_MODE - [8:7] */;

pub const WM8994_MICB2_DISCH: c_uint = 0x0100  /* MICB2_DISCH */;
pub const WM8994_MICB2_DISCH_MASK: c_uint = 0x0100  /* MICB2_DISCH */;

pub const WM8994_MICB1_DISCH: c_uint = 0x0080  /* MICB1_DISCH */;
pub const WM8994_MICB1_DISCH_MASK: c_uint = 0x0080  /* MICB1_DISCH */;

pub const WM8994_VMID_RAMP_MASK: c_uint = 0x0060  /* VMID_RAMP - [6:5] */;

pub const WM8994_VMID_BUF_ENA: c_uint = 0x0008  /* VMID_BUF_ENA */;
pub const WM8994_VMID_BUF_ENA_MASK: c_uint = 0x0008  /* VMID_BUF_ENA */;

pub const WM8994_STARTUP_BIAS_ENA: c_uint = 0x0004  /* STARTUP_BIAS_ENA */;
pub const WM8994_STARTUP_BIAS_ENA_MASK: c_uint = 0x0004  /* STARTUP_BIAS_ENA */;

pub const WM8994_BIAS_SRC: c_uint = 0x0002  /* BIAS_SRC */;
pub const WM8994_BIAS_SRC_MASK: c_uint = 0x0002  /* BIAS_SRC */;

pub const WM8994_VMID_DISCH: c_uint = 0x0001  /* VMID_DISCH */;
pub const WM8994_VMID_DISCH_MASK: c_uint = 0x0001  /* VMID_DISCH */;

//
// R58 (0x3A) - MICBIAS
//
pub const WM8994_MICD_SCTHR_MASK: c_uint = 0x00C0  /* MICD_SCTHR - [7:6] */;

pub const WM8994_MICD_THR_MASK: c_uint = 0x0038  /* MICD_THR - [5:3] */;

pub const WM8994_MICD_ENA: c_uint = 0x0004  /* MICD_ENA */;
pub const WM8994_MICD_ENA_MASK: c_uint = 0x0004  /* MICD_ENA */;

pub const WM8994_MICB2_LVL: c_uint = 0x0002  /* MICB2_LVL */;
pub const WM8994_MICB2_LVL_MASK: c_uint = 0x0002  /* MICB2_LVL */;

pub const WM8994_MICB1_LVL: c_uint = 0x0001  /* MICB1_LVL */;
pub const WM8994_MICB1_LVL_MASK: c_uint = 0x0001  /* MICB1_LVL */;

//
// R59 (0x3B) - LDO 1
//
pub const WM8994_LDO1_VSEL_MASK: c_uint = 0x000E  /* LDO1_VSEL - [3:1] */;

pub const WM8994_LDO1_DISCH: c_uint = 0x0001  /* LDO1_DISCH */;
pub const WM8994_LDO1_DISCH_MASK: c_uint = 0x0001  /* LDO1_DISCH */;

//
// R60 (0x3C) - LDO 2
//
pub const WM8994_LDO2_VSEL_MASK: c_uint = 0x0006  /* LDO2_VSEL - [2:1] */;

pub const WM8994_LDO2_DISCH: c_uint = 0x0001  /* LDO2_DISCH */;
pub const WM8994_LDO2_DISCH_MASK: c_uint = 0x0001  /* LDO2_DISCH */;

//
// R61 (0x3D) - MICBIAS1
//
pub const WM8958_MICB1_RATE: c_uint = 0x0020  /* MICB1_RATE */;
pub const WM8958_MICB1_RATE_MASK: c_uint = 0x0020  /* MICB1_RATE */;

pub const WM8958_MICB1_MODE: c_uint = 0x0010  /* MICB1_MODE */;
pub const WM8958_MICB1_MODE_MASK: c_uint = 0x0010  /* MICB1_MODE */;

pub const WM8958_MICB1_LVL_MASK: c_uint = 0x000E  /* MICB1_LVL - [3:1] */;

pub const WM8958_MICB1_DISCH: c_uint = 0x0001  /* MICB1_DISCH */;
pub const WM8958_MICB1_DISCH_MASK: c_uint = 0x0001  /* MICB1_DISCH */;

//
// R62 (0x3E) - MICBIAS2
//
pub const WM8958_MICB2_RATE: c_uint = 0x0020  /* MICB2_RATE */;
pub const WM8958_MICB2_RATE_MASK: c_uint = 0x0020  /* MICB2_RATE */;

pub const WM8958_MICB2_MODE: c_uint = 0x0010  /* MICB2_MODE */;
pub const WM8958_MICB2_MODE_MASK: c_uint = 0x0010  /* MICB2_MODE */;

pub const WM8958_MICB2_LVL_MASK: c_uint = 0x000E  /* MICB2_LVL - [3:1] */;

pub const WM8958_MICB2_DISCH: c_uint = 0x0001  /* MICB2_DISCH */;
pub const WM8958_MICB2_DISCH_MASK: c_uint = 0x0001  /* MICB2_DISCH */;

//
// R210 (0xD2) - Mic Detect 3
//
pub const WM8958_MICD_LVL_MASK: c_uint = 0x07FC  /* MICD_LVL - [10:2] */;

pub const WM8958_MICD_VALID: c_uint = 0x0002  /* MICD_VALID */;
pub const WM8958_MICD_VALID_MASK: c_uint = 0x0002  /* MICD_VALID */;

pub const WM8958_MICD_STS: c_uint = 0x0001  /* MICD_STS */;
pub const WM8958_MICD_STS_MASK: c_uint = 0x0001  /* MICD_STS */;

//
// R76 (0x4C) - Charge Pump (1)
//
pub const WM8994_CP_ENA: c_uint = 0x8000  /* CP_ENA */;
pub const WM8994_CP_ENA_MASK: c_uint = 0x8000  /* CP_ENA */;

//
// R77 (0x4D) - Charge Pump (2)
//
pub const WM8958_CP_DISCH: c_uint = 0x8000  /* CP_DISCH */;
pub const WM8958_CP_DISCH_MASK: c_uint = 0x8000  /* CP_DISCH */;

//
// R81 (0x51) - Class W (1)
//
pub const WM8994_CP_DYN_SRC_SEL_MASK: c_uint = 0x0300  /* CP_DYN_SRC_SEL - [9:8] */;

pub const WM8994_CP_DYN_PWR: c_uint = 0x0001  /* CP_DYN_PWR */;
pub const WM8994_CP_DYN_PWR_MASK: c_uint = 0x0001  /* CP_DYN_PWR */;

//
// R84 (0x54) - DC Servo (1)
//
pub const WM8994_DCS_TRIG_SINGLE_1: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;
pub const WM8994_DCS_TRIG_SINGLE_1_MASK: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;

pub const WM8994_DCS_TRIG_SINGLE_0: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;
pub const WM8994_DCS_TRIG_SINGLE_0_MASK: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;

pub const WM8994_DCS_TRIG_SERIES_1: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;
pub const WM8994_DCS_TRIG_SERIES_1_MASK: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;

pub const WM8994_DCS_TRIG_SERIES_0: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;
pub const WM8994_DCS_TRIG_SERIES_0_MASK: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;

pub const WM8994_DCS_TRIG_STARTUP_1: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;
pub const WM8994_DCS_TRIG_STARTUP_1_MASK: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;

pub const WM8994_DCS_TRIG_STARTUP_0: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;
pub const WM8994_DCS_TRIG_STARTUP_0_MASK: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;

pub const WM8994_DCS_TRIG_DAC_WR_1: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_1 */;
pub const WM8994_DCS_TRIG_DAC_WR_1_MASK: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_1 */;

pub const WM8994_DCS_TRIG_DAC_WR_0: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_0 */;
pub const WM8994_DCS_TRIG_DAC_WR_0_MASK: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_0 */;

pub const WM8994_DCS_ENA_CHAN_1: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;
pub const WM8994_DCS_ENA_CHAN_1_MASK: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;

pub const WM8994_DCS_ENA_CHAN_0: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;
pub const WM8994_DCS_ENA_CHAN_0_MASK: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;

//
// R85 (0x55) - DC Servo (2)
//
pub const WM8994_DCS_SERIES_NO_01_MASK: c_uint = 0x0FE0  /* DCS_SERIES_NO_01 - [11:5] */;

pub const WM8994_DCS_TIMER_PERIOD_01_MASK: c_uint = 0x000F  /* DCS_TIMER_PERIOD_01 - [3:0] */;

//
// R87 (0x57) - DC Servo (4)
//
pub const WM8994_DCS_DAC_WR_VAL_1_MASK: c_uint = 0xFF00  /* DCS_DAC_WR_VAL_1 - [15:8] */;

pub const WM8994_DCS_DAC_WR_VAL_0_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_0 - [7:0] */;

//
// R88 (0x58) - DC Servo Readback
//
pub const WM8994_DCS_CAL_COMPLETE_MASK: c_uint = 0x0300  /* DCS_CAL_COMPLETE - [9:8] */;

pub const WM8994_DCS_DAC_WR_COMPLETE_MASK: c_uint = 0x0030  /* DCS_DAC_WR_COMPLETE - [5:4] */;

pub const WM8994_DCS_STARTUP_COMPLETE_MASK: c_uint = 0x0003  /* DCS_STARTUP_COMPLETE - [1:0] */;

//
// R96 (0x60) - Analogue HP (1)
//
pub const WM1811_HPOUT1_ATTN: c_uint = 0x0100  /* HPOUT1_ATTN */;
pub const WM1811_HPOUT1_ATTN_MASK: c_uint = 0x0100  /* HPOUT1_ATTN */;

pub const WM8994_HPOUT1L_RMV_SHORT: c_uint = 0x0080  /* HPOUT1L_RMV_SHORT */;
pub const WM8994_HPOUT1L_RMV_SHORT_MASK: c_uint = 0x0080  /* HPOUT1L_RMV_SHORT */;

pub const WM8994_HPOUT1L_OUTP: c_uint = 0x0040  /* HPOUT1L_OUTP */;
pub const WM8994_HPOUT1L_OUTP_MASK: c_uint = 0x0040  /* HPOUT1L_OUTP */;

pub const WM8994_HPOUT1L_DLY: c_uint = 0x0020  /* HPOUT1L_DLY */;
pub const WM8994_HPOUT1L_DLY_MASK: c_uint = 0x0020  /* HPOUT1L_DLY */;

pub const WM8994_HPOUT1R_RMV_SHORT: c_uint = 0x0008  /* HPOUT1R_RMV_SHORT */;
pub const WM8994_HPOUT1R_RMV_SHORT_MASK: c_uint = 0x0008  /* HPOUT1R_RMV_SHORT */;

pub const WM8994_HPOUT1R_OUTP: c_uint = 0x0004  /* HPOUT1R_OUTP */;
pub const WM8994_HPOUT1R_OUTP_MASK: c_uint = 0x0004  /* HPOUT1R_OUTP */;

pub const WM8994_HPOUT1R_DLY: c_uint = 0x0002  /* HPOUT1R_DLY */;
pub const WM8994_HPOUT1R_DLY_MASK: c_uint = 0x0002  /* HPOUT1R_DLY */;

//
// R208 (0xD0) - Mic Detect 1
//
pub const WM8958_MICD_BIAS_STARTTIME_MASK: c_uint = 0xF000  /* MICD_BIAS_STARTTIME - [15:12] */;

pub const WM8958_MICD_RATE_MASK: c_uint = 0x0F00  /* MICD_RATE - [11:8] */;

pub const WM8958_MICD_DBTIME: c_uint = 0x0002  /* MICD_DBTIME */;
pub const WM8958_MICD_DBTIME_MASK: c_uint = 0x0002  /* MICD_DBTIME */;

pub const WM8958_MICD_ENA: c_uint = 0x0001  /* MICD_ENA */;
pub const WM8958_MICD_ENA_MASK: c_uint = 0x0001  /* MICD_ENA */;

//
// R209 (0xD1) - Mic Detect 2
//
pub const WM8958_MICD_LVL_SEL_MASK: c_uint = 0x00FF  /* MICD_LVL_SEL - [7:0] */;

//
// R210 (0xD2) - Mic Detect 3
//
pub const WM8958_MICD_LVL_MASK: c_uint = 0x07FC  /* MICD_LVL - [10:2] */;

pub const WM8958_MICD_VALID: c_uint = 0x0002  /* MICD_VALID */;
pub const WM8958_MICD_VALID_MASK: c_uint = 0x0002  /* MICD_VALID */;

pub const WM8958_MICD_STS: c_uint = 0x0001  /* MICD_STS */;
pub const WM8958_MICD_STS_MASK: c_uint = 0x0001  /* MICD_STS */;

//
// R256 (0x100) - Chip Revision
//
pub const WM8994_CUST_ID_MASK: c_uint = 0xFF00  /* CUST_ID - [15:8] */;

pub const WM8994_CHIP_REV_MASK: c_uint = 0x000F  /* CHIP_REV - [3:0] */;

//
// R257 (0x101) - Control Interface
//
pub const WM8994_SPI_CONTRD: c_uint = 0x0040  /* SPI_CONTRD */;
pub const WM8994_SPI_CONTRD_MASK: c_uint = 0x0040  /* SPI_CONTRD */;

pub const WM8994_SPI_4WIRE: c_uint = 0x0020  /* SPI_4WIRE */;
pub const WM8994_SPI_4WIRE_MASK: c_uint = 0x0020  /* SPI_4WIRE */;

pub const WM8994_SPI_CFG: c_uint = 0x0010  /* SPI_CFG */;
pub const WM8994_SPI_CFG_MASK: c_uint = 0x0010  /* SPI_CFG */;

pub const WM8994_AUTO_INC: c_uint = 0x0004  /* AUTO_INC */;
pub const WM8994_AUTO_INC_MASK: c_uint = 0x0004  /* AUTO_INC */;

//
// R272 (0x110) - Write Sequencer Ctrl (1)
//
pub const WM8994_WSEQ_ENA: c_uint = 0x8000  /* WSEQ_ENA */;
pub const WM8994_WSEQ_ENA_MASK: c_uint = 0x8000  /* WSEQ_ENA */;

pub const WM8994_WSEQ_ABORT: c_uint = 0x0200  /* WSEQ_ABORT */;
pub const WM8994_WSEQ_ABORT_MASK: c_uint = 0x0200  /* WSEQ_ABORT */;

pub const WM8994_WSEQ_START: c_uint = 0x0100  /* WSEQ_START */;
pub const WM8994_WSEQ_START_MASK: c_uint = 0x0100  /* WSEQ_START */;

pub const WM8994_WSEQ_START_INDEX_MASK: c_uint = 0x007F  /* WSEQ_START_INDEX - [6:0] */;

//
// R273 (0x111) - Write Sequencer Ctrl (2)
//
pub const WM8994_WSEQ_BUSY: c_uint = 0x0100  /* WSEQ_BUSY */;
pub const WM8994_WSEQ_BUSY_MASK: c_uint = 0x0100  /* WSEQ_BUSY */;

pub const WM8994_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x007F  /* WSEQ_CURRENT_INDEX - [6:0] */;

//
// R512 (0x200) - AIF1 Clocking (1)
//
pub const WM8994_AIF1CLK_SRC_MASK: c_uint = 0x0018  /* AIF1CLK_SRC - [4:3] */;

pub const WM8994_AIF1CLK_INV: c_uint = 0x0004  /* AIF1CLK_INV */;
pub const WM8994_AIF1CLK_INV_MASK: c_uint = 0x0004  /* AIF1CLK_INV */;

pub const WM8994_AIF1CLK_DIV: c_uint = 0x0002  /* AIF1CLK_DIV */;
pub const WM8994_AIF1CLK_DIV_MASK: c_uint = 0x0002  /* AIF1CLK_DIV */;

pub const WM8994_AIF1CLK_ENA: c_uint = 0x0001  /* AIF1CLK_ENA */;
pub const WM8994_AIF1CLK_ENA_MASK: c_uint = 0x0001  /* AIF1CLK_ENA */;

//
// R513 (0x201) - AIF1 Clocking (2)
//
pub const WM8994_AIF1DAC_DIV_MASK: c_uint = 0x0038  /* AIF1DAC_DIV - [5:3] */;

pub const WM8994_AIF1ADC_DIV_MASK: c_uint = 0x0007  /* AIF1ADC_DIV - [2:0] */;

//
// R516 (0x204) - AIF2 Clocking (1)
//
pub const WM8994_AIF2CLK_SRC_MASK: c_uint = 0x0018  /* AIF2CLK_SRC - [4:3] */;

pub const WM8994_AIF2CLK_INV: c_uint = 0x0004  /* AIF2CLK_INV */;
pub const WM8994_AIF2CLK_INV_MASK: c_uint = 0x0004  /* AIF2CLK_INV */;

pub const WM8994_AIF2CLK_DIV: c_uint = 0x0002  /* AIF2CLK_DIV */;
pub const WM8994_AIF2CLK_DIV_MASK: c_uint = 0x0002  /* AIF2CLK_DIV */;

pub const WM8994_AIF2CLK_ENA: c_uint = 0x0001  /* AIF2CLK_ENA */;
pub const WM8994_AIF2CLK_ENA_MASK: c_uint = 0x0001  /* AIF2CLK_ENA */;

//
// R517 (0x205) - AIF2 Clocking (2)
//
pub const WM8994_AIF2DAC_DIV_MASK: c_uint = 0x0038  /* AIF2DAC_DIV - [5:3] */;

pub const WM8994_AIF2ADC_DIV_MASK: c_uint = 0x0007  /* AIF2ADC_DIV - [2:0] */;

//
// R520 (0x208) - Clocking (1)
//
pub const WM8958_DSP2CLK_ENA: c_uint = 0x4000  /* DSP2CLK_ENA */;
pub const WM8958_DSP2CLK_ENA_MASK: c_uint = 0x4000  /* DSP2CLK_ENA */;

pub const WM8958_DSP2CLK_SRC: c_uint = 0x1000  /* DSP2CLK_SRC */;
pub const WM8958_DSP2CLK_SRC_MASK: c_uint = 0x1000  /* DSP2CLK_SRC */;

pub const WM8994_TOCLK_ENA: c_uint = 0x0010  /* TOCLK_ENA */;
pub const WM8994_TOCLK_ENA_MASK: c_uint = 0x0010  /* TOCLK_ENA */;

pub const WM8994_AIF1DSPCLK_ENA: c_uint = 0x0008  /* AIF1DSPCLK_ENA */;
pub const WM8994_AIF1DSPCLK_ENA_MASK: c_uint = 0x0008  /* AIF1DSPCLK_ENA */;

pub const WM8994_AIF2DSPCLK_ENA: c_uint = 0x0004  /* AIF2DSPCLK_ENA */;
pub const WM8994_AIF2DSPCLK_ENA_MASK: c_uint = 0x0004  /* AIF2DSPCLK_ENA */;

pub const WM8994_SYSDSPCLK_ENA: c_uint = 0x0002  /* SYSDSPCLK_ENA */;
pub const WM8994_SYSDSPCLK_ENA_MASK: c_uint = 0x0002  /* SYSDSPCLK_ENA */;

pub const WM8994_SYSCLK_SRC: c_uint = 0x0001  /* SYSCLK_SRC */;
pub const WM8994_SYSCLK_SRC_MASK: c_uint = 0x0001  /* SYSCLK_SRC */;

//
// R521 (0x209) - Clocking (2)
//
pub const WM8994_TOCLK_DIV_MASK: c_uint = 0x0700  /* TOCLK_DIV - [10:8] */;

pub const WM8994_DBCLK_DIV_MASK: c_uint = 0x0070  /* DBCLK_DIV - [6:4] */;

pub const WM8994_OPCLK_DIV_MASK: c_uint = 0x0007  /* OPCLK_DIV - [2:0] */;

//
// R528 (0x210) - AIF1 Rate
//
pub const WM8994_AIF1_SR_MASK: c_uint = 0x00F0  /* AIF1_SR - [7:4] */;

pub const WM8994_AIF1CLK_RATE_MASK: c_uint = 0x000F  /* AIF1CLK_RATE - [3:0] */;

//
// R529 (0x211) - AIF2 Rate
//
pub const WM8994_AIF2_SR_MASK: c_uint = 0x00F0  /* AIF2_SR - [7:4] */;

pub const WM8994_AIF2CLK_RATE_MASK: c_uint = 0x000F  /* AIF2CLK_RATE - [3:0] */;

//
// R530 (0x212) - Rate Status
//
pub const WM8994_SR_ERROR_MASK: c_uint = 0x000F  /* SR_ERROR - [3:0] */;

//
// R544 (0x220) - FLL1 Control (1)
//
pub const WM8994_FLL1_FRAC: c_uint = 0x0004  /* FLL1_FRAC */;
pub const WM8994_FLL1_FRAC_MASK: c_uint = 0x0004  /* FLL1_FRAC */;

pub const WM8994_FLL1_OSC_ENA: c_uint = 0x0002  /* FLL1_OSC_ENA */;
pub const WM8994_FLL1_OSC_ENA_MASK: c_uint = 0x0002  /* FLL1_OSC_ENA */;

pub const WM8994_FLL1_ENA: c_uint = 0x0001  /* FLL1_ENA */;
pub const WM8994_FLL1_ENA_MASK: c_uint = 0x0001  /* FLL1_ENA */;

//
// R545 (0x221) - FLL1 Control (2)
//
pub const WM8994_FLL1_OUTDIV_MASK: c_uint = 0x3F00  /* FLL1_OUTDIV - [13:8] */;

pub const WM8994_FLL1_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL1_CTRL_RATE - [6:4] */;

pub const WM8994_FLL1_FRATIO_MASK: c_uint = 0x0007  /* FLL1_FRATIO - [2:0] */;

//
// R546 (0x222) - FLL1 Control (3)
//
pub const WM8994_FLL1_K_MASK: c_uint = 0xFFFF  /* FLL1_K - [15:0] */;

//
// R547 (0x223) - FLL1 Control (4)
//
pub const WM8994_FLL1_N_MASK: c_uint = 0x7FE0  /* FLL1_N - [14:5] */;

pub const WM8994_FLL1_LOOP_GAIN_MASK: c_uint = 0x000F  /* FLL1_LOOP_GAIN - [3:0] */;

//
// R548 (0x224) - FLL1 Control (5)
//
pub const WM8958_FLL1_BYP: c_uint = 0x8000  /* FLL1_BYP */;
pub const WM8958_FLL1_BYP_MASK: c_uint = 0x8000  /* FLL1_BYP */;

pub const WM8994_FLL1_FRC_NCO_VAL_MASK: c_uint = 0x1F80  /* FLL1_FRC_NCO_VAL - [12:7] */;

pub const WM8994_FLL1_FRC_NCO: c_uint = 0x0040  /* FLL1_FRC_NCO */;
pub const WM8994_FLL1_FRC_NCO_MASK: c_uint = 0x0040  /* FLL1_FRC_NCO */;

pub const WM8994_FLL1_REFCLK_DIV_MASK: c_uint = 0x0018  /* FLL1_REFCLK_DIV - [4:3] */;

pub const WM8994_FLL1_REFCLK_SRC_MASK: c_uint = 0x0003  /* FLL1_REFCLK_SRC - [1:0] */;

//
// R550 (0x226) - FLL1 EFS 1
//
pub const WM8958_FLL1_LAMBDA_MASK: c_uint = 0xFFFF  /* FLL1_LAMBDA - [15:0] */;

//
// R551 (0x227) - FLL1 EFS 2
//
pub const WM8958_FLL1_LFSR_SEL_MASK: c_uint = 0x0006  /* FLL1_LFSR_SEL - [2:1] */;

pub const WM8958_FLL1_EFS_ENA: c_uint = 0x0001  /* FLL1_EFS_ENA */;
pub const WM8958_FLL1_EFS_ENA_MASK: c_uint = 0x0001  /* FLL1_EFS_ENA */;

//
// R576 (0x240) - FLL2 Control (1)
//
pub const WM8994_FLL2_FRAC: c_uint = 0x0004  /* FLL2_FRAC */;
pub const WM8994_FLL2_FRAC_MASK: c_uint = 0x0004  /* FLL2_FRAC */;

pub const WM8994_FLL2_OSC_ENA: c_uint = 0x0002  /* FLL2_OSC_ENA */;
pub const WM8994_FLL2_OSC_ENA_MASK: c_uint = 0x0002  /* FLL2_OSC_ENA */;

pub const WM8994_FLL2_ENA: c_uint = 0x0001  /* FLL2_ENA */;
pub const WM8994_FLL2_ENA_MASK: c_uint = 0x0001  /* FLL2_ENA */;

//
// R577 (0x241) - FLL2 Control (2)
//
pub const WM8994_FLL2_OUTDIV_MASK: c_uint = 0x3F00  /* FLL2_OUTDIV - [13:8] */;

pub const WM8994_FLL2_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL2_CTRL_RATE - [6:4] */;

pub const WM8994_FLL2_FRATIO_MASK: c_uint = 0x0007  /* FLL2_FRATIO - [2:0] */;

//
// R578 (0x242) - FLL2 Control (3)
//
pub const WM8994_FLL2_K_MASK: c_uint = 0xFFFF  /* FLL2_K - [15:0] */;

//
// R579 (0x243) - FLL2 Control (4)
//
pub const WM8994_FLL2_N_MASK: c_uint = 0x7FE0  /* FLL2_N - [14:5] */;

pub const WM8994_FLL2_LOOP_GAIN_MASK: c_uint = 0x000F  /* FLL2_LOOP_GAIN - [3:0] */;

//
// R580 (0x244) - FLL2 Control (5)
//
pub const WM8958_FLL2_BYP: c_uint = 0x8000  /* FLL2_BYP */;
pub const WM8958_FLL2_BYP_MASK: c_uint = 0x8000  /* FLL2_BYP */;

pub const WM8994_FLL2_FRC_NCO_VAL_MASK: c_uint = 0x1F80  /* FLL2_FRC_NCO_VAL - [12:7] */;

pub const WM8994_FLL2_FRC_NCO: c_uint = 0x0040  /* FLL2_FRC_NCO */;
pub const WM8994_FLL2_FRC_NCO_MASK: c_uint = 0x0040  /* FLL2_FRC_NCO */;

pub const WM8994_FLL2_REFCLK_DIV_MASK: c_uint = 0x0018  /* FLL2_REFCLK_DIV - [4:3] */;

pub const WM8994_FLL2_REFCLK_SRC_MASK: c_uint = 0x0003  /* FLL2_REFCLK_SRC - [1:0] */;

//
// R582 (0x246) - FLL2 EFS 1
//
pub const WM8958_FLL2_LAMBDA_MASK: c_uint = 0xFFFF  /* FLL2_LAMBDA - [15:0] */;

//
// R583 (0x247) - FLL2 EFS 2
//
pub const WM8958_FLL2_LFSR_SEL_MASK: c_uint = 0x0006  /* FLL2_LFSR_SEL - [2:1] */;

pub const WM8958_FLL2_EFS_ENA: c_uint = 0x0001  /* FLL2_EFS_ENA */;
pub const WM8958_FLL2_EFS_ENA_MASK: c_uint = 0x0001  /* FLL2_EFS_ENA */;

//
// R768 (0x300) - AIF1 Control (1)
//
pub const WM8994_AIF1ADCL_SRC: c_uint = 0x8000  /* AIF1ADCL_SRC */;
pub const WM8994_AIF1ADCL_SRC_MASK: c_uint = 0x8000  /* AIF1ADCL_SRC */;

pub const WM8994_AIF1ADCR_SRC: c_uint = 0x4000  /* AIF1ADCR_SRC */;
pub const WM8994_AIF1ADCR_SRC_MASK: c_uint = 0x4000  /* AIF1ADCR_SRC */;

pub const WM8994_AIF1ADC_TDM: c_uint = 0x2000  /* AIF1ADC_TDM */;
pub const WM8994_AIF1ADC_TDM_MASK: c_uint = 0x2000  /* AIF1ADC_TDM */;

pub const WM8994_AIF1_BCLK_INV: c_uint = 0x0100  /* AIF1_BCLK_INV */;
pub const WM8994_AIF1_BCLK_INV_MASK: c_uint = 0x0100  /* AIF1_BCLK_INV */;

pub const WM8994_AIF1_LRCLK_INV: c_uint = 0x0080  /* AIF1_LRCLK_INV */;
pub const WM8994_AIF1_LRCLK_INV_MASK: c_uint = 0x0080  /* AIF1_LRCLK_INV */;

pub const WM8994_AIF1_WL_MASK: c_uint = 0x0060  /* AIF1_WL - [6:5] */;

pub const WM8994_AIF1_FMT_MASK: c_uint = 0x0018  /* AIF1_FMT - [4:3] */;

//
// R769 (0x301) - AIF1 Control (2)
//
pub const WM8994_AIF1DACL_SRC: c_uint = 0x8000  /* AIF1DACL_SRC */;
pub const WM8994_AIF1DACL_SRC_MASK: c_uint = 0x8000  /* AIF1DACL_SRC */;

pub const WM8994_AIF1DACR_SRC: c_uint = 0x4000  /* AIF1DACR_SRC */;
pub const WM8994_AIF1DACR_SRC_MASK: c_uint = 0x4000  /* AIF1DACR_SRC */;

pub const WM8994_AIF1DAC_BOOST_MASK: c_uint = 0x0C00  /* AIF1DAC_BOOST - [11:10] */;

pub const WM8994_AIF1_MONO: c_uint = 0x0100  /* AIF1_MONO */;
pub const WM8994_AIF1_MONO_MASK: c_uint = 0x0100  /* AIF1_MONO */;

pub const WM8994_AIF1DAC_COMP: c_uint = 0x0010  /* AIF1DAC_COMP */;
pub const WM8994_AIF1DAC_COMP_MASK: c_uint = 0x0010  /* AIF1DAC_COMP */;

pub const WM8994_AIF1DAC_COMPMODE: c_uint = 0x0008  /* AIF1DAC_COMPMODE */;
pub const WM8994_AIF1DAC_COMPMODE_MASK: c_uint = 0x0008  /* AIF1DAC_COMPMODE */;

pub const WM8994_AIF1ADC_COMP: c_uint = 0x0004  /* AIF1ADC_COMP */;
pub const WM8994_AIF1ADC_COMP_MASK: c_uint = 0x0004  /* AIF1ADC_COMP */;

pub const WM8994_AIF1ADC_COMPMODE: c_uint = 0x0002  /* AIF1ADC_COMPMODE */;
pub const WM8994_AIF1ADC_COMPMODE_MASK: c_uint = 0x0002  /* AIF1ADC_COMPMODE */;

pub const WM8994_AIF1_LOOPBACK: c_uint = 0x0001  /* AIF1_LOOPBACK */;
pub const WM8994_AIF1_LOOPBACK_MASK: c_uint = 0x0001  /* AIF1_LOOPBACK */;

//
// R770 (0x302) - AIF1 Master/Slave
//
pub const WM8994_AIF1_TRI: c_uint = 0x8000  /* AIF1_TRI */;
pub const WM8994_AIF1_TRI_MASK: c_uint = 0x8000  /* AIF1_TRI */;

pub const WM8994_AIF1_MSTR: c_uint = 0x4000  /* AIF1_MSTR */;
pub const WM8994_AIF1_MSTR_MASK: c_uint = 0x4000  /* AIF1_MSTR */;

pub const WM8994_AIF1_CLK_FRC: c_uint = 0x2000  /* AIF1_CLK_FRC */;
pub const WM8994_AIF1_CLK_FRC_MASK: c_uint = 0x2000  /* AIF1_CLK_FRC */;

pub const WM8994_AIF1_LRCLK_FRC: c_uint = 0x1000  /* AIF1_LRCLK_FRC */;
pub const WM8994_AIF1_LRCLK_FRC_MASK: c_uint = 0x1000  /* AIF1_LRCLK_FRC */;

//
// R771 (0x303) - AIF1 BCLK
//
pub const WM8994_AIF1_BCLK_DIV_MASK: c_uint = 0x01F0  /* AIF1_BCLK_DIV - [8:4] */;

//
// R772 (0x304) - AIF1ADC LRCLK
//
pub const WM8958_AIF1_LRCLK_INV: c_uint = 0x1000  /* AIF1_LRCLK_INV */;
pub const WM8958_AIF1_LRCLK_INV_MASK: c_uint = 0x1000  /* AIF1_LRCLK_INV */;

pub const WM8994_AIF1ADC_LRCLK_DIR: c_uint = 0x0800  /* AIF1ADC_LRCLK_DIR */;
pub const WM8994_AIF1ADC_LRCLK_DIR_MASK: c_uint = 0x0800  /* AIF1ADC_LRCLK_DIR */;

pub const WM8994_AIF1ADC_RATE_MASK: c_uint = 0x07FF  /* AIF1ADC_RATE - [10:0] */;

//
// R773 (0x305) - AIF1DAC LRCLK
//
pub const WM8958_AIF1_LRCLK_INV: c_uint = 0x1000  /* AIF1_LRCLK_INV */;
pub const WM8958_AIF1_LRCLK_INV_MASK: c_uint = 0x1000  /* AIF1_LRCLK_INV */;

pub const WM8994_AIF1DAC_LRCLK_DIR: c_uint = 0x0800  /* AIF1DAC_LRCLK_DIR */;
pub const WM8994_AIF1DAC_LRCLK_DIR_MASK: c_uint = 0x0800  /* AIF1DAC_LRCLK_DIR */;

pub const WM8994_AIF1DAC_RATE_MASK: c_uint = 0x07FF  /* AIF1DAC_RATE - [10:0] */;

//
// R774 (0x306) - AIF1DAC Data
//
pub const WM8994_AIF1DACL_DAT_INV: c_uint = 0x0002  /* AIF1DACL_DAT_INV */;
pub const WM8994_AIF1DACL_DAT_INV_MASK: c_uint = 0x0002  /* AIF1DACL_DAT_INV */;

pub const WM8994_AIF1DACR_DAT_INV: c_uint = 0x0001  /* AIF1DACR_DAT_INV */;
pub const WM8994_AIF1DACR_DAT_INV_MASK: c_uint = 0x0001  /* AIF1DACR_DAT_INV */;

//
// R775 (0x307) - AIF1ADC Data
//
pub const WM8994_AIF1ADCL_DAT_INV: c_uint = 0x0002  /* AIF1ADCL_DAT_INV */;
pub const WM8994_AIF1ADCL_DAT_INV_MASK: c_uint = 0x0002  /* AIF1ADCL_DAT_INV */;

pub const WM8994_AIF1ADCR_DAT_INV: c_uint = 0x0001  /* AIF1ADCR_DAT_INV */;
pub const WM8994_AIF1ADCR_DAT_INV_MASK: c_uint = 0x0001  /* AIF1ADCR_DAT_INV */;

//
// R784 (0x310) - AIF2 Control (1)
//
pub const WM8994_AIF2ADCL_SRC: c_uint = 0x8000  /* AIF2ADCL_SRC */;
pub const WM8994_AIF2ADCL_SRC_MASK: c_uint = 0x8000  /* AIF2ADCL_SRC */;

pub const WM8994_AIF2ADCR_SRC: c_uint = 0x4000  /* AIF2ADCR_SRC */;
pub const WM8994_AIF2ADCR_SRC_MASK: c_uint = 0x4000  /* AIF2ADCR_SRC */;

pub const WM8994_AIF2ADC_TDM: c_uint = 0x2000  /* AIF2ADC_TDM */;
pub const WM8994_AIF2ADC_TDM_MASK: c_uint = 0x2000  /* AIF2ADC_TDM */;

pub const WM8994_AIF2ADC_TDM_CHAN: c_uint = 0x1000  /* AIF2ADC_TDM_CHAN */;
pub const WM8994_AIF2ADC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIF2ADC_TDM_CHAN */;

pub const WM8994_AIF2_BCLK_INV: c_uint = 0x0100  /* AIF2_BCLK_INV */;
pub const WM8994_AIF2_BCLK_INV_MASK: c_uint = 0x0100  /* AIF2_BCLK_INV */;

pub const WM8994_AIF2_LRCLK_INV: c_uint = 0x0080  /* AIF2_LRCLK_INV */;
pub const WM8994_AIF2_LRCLK_INV_MASK: c_uint = 0x0080  /* AIF2_LRCLK_INV */;

pub const WM8994_AIF2_WL_MASK: c_uint = 0x0060  /* AIF2_WL - [6:5] */;

pub const WM8994_AIF2_FMT_MASK: c_uint = 0x0018  /* AIF2_FMT - [4:3] */;

//
// R785 (0x311) - AIF2 Control (2)
//
pub const WM8994_AIF2DACL_SRC: c_uint = 0x8000  /* AIF2DACL_SRC */;
pub const WM8994_AIF2DACL_SRC_MASK: c_uint = 0x8000  /* AIF2DACL_SRC */;

pub const WM8994_AIF2DACR_SRC: c_uint = 0x4000  /* AIF2DACR_SRC */;
pub const WM8994_AIF2DACR_SRC_MASK: c_uint = 0x4000  /* AIF2DACR_SRC */;

pub const WM8994_AIF2DAC_TDM: c_uint = 0x2000  /* AIF2DAC_TDM */;
pub const WM8994_AIF2DAC_TDM_MASK: c_uint = 0x2000  /* AIF2DAC_TDM */;

pub const WM8994_AIF2DAC_TDM_CHAN: c_uint = 0x1000  /* AIF2DAC_TDM_CHAN */;
pub const WM8994_AIF2DAC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIF2DAC_TDM_CHAN */;

pub const WM8994_AIF2DAC_BOOST_MASK: c_uint = 0x0C00  /* AIF2DAC_BOOST - [11:10] */;

pub const WM8994_AIF2_MONO: c_uint = 0x0100  /* AIF2_MONO */;
pub const WM8994_AIF2_MONO_MASK: c_uint = 0x0100  /* AIF2_MONO */;

pub const WM8994_AIF2DAC_COMP: c_uint = 0x0010  /* AIF2DAC_COMP */;
pub const WM8994_AIF2DAC_COMP_MASK: c_uint = 0x0010  /* AIF2DAC_COMP */;

pub const WM8994_AIF2DAC_COMPMODE: c_uint = 0x0008  /* AIF2DAC_COMPMODE */;
pub const WM8994_AIF2DAC_COMPMODE_MASK: c_uint = 0x0008  /* AIF2DAC_COMPMODE */;

pub const WM8994_AIF2ADC_COMP: c_uint = 0x0004  /* AIF2ADC_COMP */;
pub const WM8994_AIF2ADC_COMP_MASK: c_uint = 0x0004  /* AIF2ADC_COMP */;

pub const WM8994_AIF2ADC_COMPMODE: c_uint = 0x0002  /* AIF2ADC_COMPMODE */;
pub const WM8994_AIF2ADC_COMPMODE_MASK: c_uint = 0x0002  /* AIF2ADC_COMPMODE */;

pub const WM8994_AIF2_LOOPBACK: c_uint = 0x0001  /* AIF2_LOOPBACK */;
pub const WM8994_AIF2_LOOPBACK_MASK: c_uint = 0x0001  /* AIF2_LOOPBACK */;

//
// R786 (0x312) - AIF2 Master/Slave
//
pub const WM8994_AIF2_TRI: c_uint = 0x8000  /* AIF2_TRI */;
pub const WM8994_AIF2_TRI_MASK: c_uint = 0x8000  /* AIF2_TRI */;

pub const WM8994_AIF2_MSTR: c_uint = 0x4000  /* AIF2_MSTR */;
pub const WM8994_AIF2_MSTR_MASK: c_uint = 0x4000  /* AIF2_MSTR */;

pub const WM8994_AIF2_CLK_FRC: c_uint = 0x2000  /* AIF2_CLK_FRC */;
pub const WM8994_AIF2_CLK_FRC_MASK: c_uint = 0x2000  /* AIF2_CLK_FRC */;

pub const WM8994_AIF2_LRCLK_FRC: c_uint = 0x1000  /* AIF2_LRCLK_FRC */;
pub const WM8994_AIF2_LRCLK_FRC_MASK: c_uint = 0x1000  /* AIF2_LRCLK_FRC */;

//
// R787 (0x313) - AIF2 BCLK
//
pub const WM8994_AIF2_BCLK_DIV_MASK: c_uint = 0x01F0  /* AIF2_BCLK_DIV - [8:4] */;

//
// R788 (0x314) - AIF2ADC LRCLK
//
pub const WM8994_AIF2ADC_LRCLK_DIR: c_uint = 0x0800  /* AIF2ADC_LRCLK_DIR */;
pub const WM8994_AIF2ADC_LRCLK_DIR_MASK: c_uint = 0x0800  /* AIF2ADC_LRCLK_DIR */;

pub const WM8994_AIF2ADC_RATE_MASK: c_uint = 0x07FF  /* AIF2ADC_RATE - [10:0] */;

//
// R789 (0x315) - AIF2DAC LRCLK
//
pub const WM8994_AIF2DAC_LRCLK_DIR: c_uint = 0x0800  /* AIF2DAC_LRCLK_DIR */;
pub const WM8994_AIF2DAC_LRCLK_DIR_MASK: c_uint = 0x0800  /* AIF2DAC_LRCLK_DIR */;

pub const WM8994_AIF2DAC_RATE_MASK: c_uint = 0x07FF  /* AIF2DAC_RATE - [10:0] */;

//
// R790 (0x316) - AIF2DAC Data
//
pub const WM8994_AIF2DACL_DAT_INV: c_uint = 0x0002  /* AIF2DACL_DAT_INV */;
pub const WM8994_AIF2DACL_DAT_INV_MASK: c_uint = 0x0002  /* AIF2DACL_DAT_INV */;

pub const WM8994_AIF2DACR_DAT_INV: c_uint = 0x0001  /* AIF2DACR_DAT_INV */;
pub const WM8994_AIF2DACR_DAT_INV_MASK: c_uint = 0x0001  /* AIF2DACR_DAT_INV */;

//
// R791 (0x317) - AIF2ADC Data
//
pub const WM8994_AIF2ADCL_DAT_INV: c_uint = 0x0002  /* AIF2ADCL_DAT_INV */;
pub const WM8994_AIF2ADCL_DAT_INV_MASK: c_uint = 0x0002  /* AIF2ADCL_DAT_INV */;

pub const WM8994_AIF2ADCR_DAT_INV: c_uint = 0x0001  /* AIF2ADCR_DAT_INV */;
pub const WM8994_AIF2ADCR_DAT_INV_MASK: c_uint = 0x0001  /* AIF2ADCR_DAT_INV */;

//
// R800 (0x320) - AIF3 Control (1)
//
pub const WM8958_AIF3_LRCLK_INV: c_uint = 0x0080  /* AIF3_LRCLK_INV */;
pub const WM8958_AIF3_LRCLK_INV_MASK: c_uint = 0x0080  /* AIF3_LRCLK_INV */;

pub const WM8958_AIF3_WL_MASK: c_uint = 0x0060  /* AIF3_WL - [6:5] */;

pub const WM8958_AIF3_FMT_MASK: c_uint = 0x0018  /* AIF3_FMT - [4:3] */;

//
// R801 (0x321) - AIF3 Control (2)
//
pub const WM8958_AIF3DAC_BOOST_MASK: c_uint = 0x0C00  /* AIF3DAC_BOOST - [11:10] */;

pub const WM8958_AIF3DAC_COMP: c_uint = 0x0010  /* AIF3DAC_COMP */;
pub const WM8958_AIF3DAC_COMP_MASK: c_uint = 0x0010  /* AIF3DAC_COMP */;

pub const WM8958_AIF3DAC_COMPMODE: c_uint = 0x0008  /* AIF3DAC_COMPMODE */;
pub const WM8958_AIF3DAC_COMPMODE_MASK: c_uint = 0x0008  /* AIF3DAC_COMPMODE */;

pub const WM8958_AIF3ADC_COMP: c_uint = 0x0004  /* AIF3ADC_COMP */;
pub const WM8958_AIF3ADC_COMP_MASK: c_uint = 0x0004  /* AIF3ADC_COMP */;

pub const WM8958_AIF3ADC_COMPMODE: c_uint = 0x0002  /* AIF3ADC_COMPMODE */;
pub const WM8958_AIF3ADC_COMPMODE_MASK: c_uint = 0x0002  /* AIF3ADC_COMPMODE */;

pub const WM8958_AIF3_LOOPBACK: c_uint = 0x0001  /* AIF3_LOOPBACK */;
pub const WM8958_AIF3_LOOPBACK_MASK: c_uint = 0x0001  /* AIF3_LOOPBACK */;

//
// R802 (0x322) - AIF3DAC Data
//
pub const WM8958_AIF3DAC_DAT_INV: c_uint = 0x0001  /* AIF3DAC_DAT_INV */;
pub const WM8958_AIF3DAC_DAT_INV_MASK: c_uint = 0x0001  /* AIF3DAC_DAT_INV */;

//
// R803 (0x323) - AIF3ADC Data
//
pub const WM8958_AIF3ADC_DAT_INV: c_uint = 0x0001  /* AIF3ADC_DAT_INV */;
pub const WM8958_AIF3ADC_DAT_INV_MASK: c_uint = 0x0001  /* AIF3ADC_DAT_INV */;

//
// R1024 (0x400) - AIF1 ADC1 Left Volume
//
pub const WM8994_AIF1ADC1_VU: c_uint = 0x0100  /* AIF1ADC1_VU */;
pub const WM8994_AIF1ADC1_VU_MASK: c_uint = 0x0100  /* AIF1ADC1_VU */;

pub const WM8994_AIF1ADC1L_VOL_MASK: c_uint = 0x00FF  /* AIF1ADC1L_VOL - [7:0] */;

//
// R1025 (0x401) - AIF1 ADC1 Right Volume
//
pub const WM8994_AIF1ADC1_VU: c_uint = 0x0100  /* AIF1ADC1_VU */;
pub const WM8994_AIF1ADC1_VU_MASK: c_uint = 0x0100  /* AIF1ADC1_VU */;

pub const WM8994_AIF1ADC1R_VOL_MASK: c_uint = 0x00FF  /* AIF1ADC1R_VOL - [7:0] */;

//
// R1026 (0x402) - AIF1 DAC1 Left Volume
//
pub const WM8994_AIF1DAC1_VU: c_uint = 0x0100  /* AIF1DAC1_VU */;
pub const WM8994_AIF1DAC1_VU_MASK: c_uint = 0x0100  /* AIF1DAC1_VU */;

pub const WM8994_AIF1DAC1L_VOL_MASK: c_uint = 0x00FF  /* AIF1DAC1L_VOL - [7:0] */;

//
// R1027 (0x403) - AIF1 DAC1 Right Volume
//
pub const WM8994_AIF1DAC1_VU: c_uint = 0x0100  /* AIF1DAC1_VU */;
pub const WM8994_AIF1DAC1_VU_MASK: c_uint = 0x0100  /* AIF1DAC1_VU */;

pub const WM8994_AIF1DAC1R_VOL_MASK: c_uint = 0x00FF  /* AIF1DAC1R_VOL - [7:0] */;

//
// R1028 (0x404) - AIF1 ADC2 Left Volume
//
pub const WM8994_AIF1ADC2_VU: c_uint = 0x0100  /* AIF1ADC2_VU */;
pub const WM8994_AIF1ADC2_VU_MASK: c_uint = 0x0100  /* AIF1ADC2_VU */;

pub const WM8994_AIF1ADC2L_VOL_MASK: c_uint = 0x00FF  /* AIF1ADC2L_VOL - [7:0] */;

//
// R1029 (0x405) - AIF1 ADC2 Right Volume
//
pub const WM8994_AIF1ADC2_VU: c_uint = 0x0100  /* AIF1ADC2_VU */;
pub const WM8994_AIF1ADC2_VU_MASK: c_uint = 0x0100  /* AIF1ADC2_VU */;

pub const WM8994_AIF1ADC2R_VOL_MASK: c_uint = 0x00FF  /* AIF1ADC2R_VOL - [7:0] */;

//
// R1030 (0x406) - AIF1 DAC2 Left Volume
//
pub const WM8994_AIF1DAC2_VU: c_uint = 0x0100  /* AIF1DAC2_VU */;
pub const WM8994_AIF1DAC2_VU_MASK: c_uint = 0x0100  /* AIF1DAC2_VU */;

pub const WM8994_AIF1DAC2L_VOL_MASK: c_uint = 0x00FF  /* AIF1DAC2L_VOL - [7:0] */;

//
// R1031 (0x407) - AIF1 DAC2 Right Volume
//
pub const WM8994_AIF1DAC2_VU: c_uint = 0x0100  /* AIF1DAC2_VU */;
pub const WM8994_AIF1DAC2_VU_MASK: c_uint = 0x0100  /* AIF1DAC2_VU */;

pub const WM8994_AIF1DAC2R_VOL_MASK: c_uint = 0x00FF  /* AIF1DAC2R_VOL - [7:0] */;

//
// R1040 (0x410) - AIF1 ADC1 Filters
//
pub const WM8994_AIF1ADC_4FS: c_uint = 0x8000  /* AIF1ADC_4FS */;
pub const WM8994_AIF1ADC_4FS_MASK: c_uint = 0x8000  /* AIF1ADC_4FS */;

pub const WM8994_AIF1ADC1_HPF_CUT_MASK: c_uint = 0x6000  /* AIF1ADC1_HPF_CUT - [14:13] */;

pub const WM8994_AIF1ADC1L_HPF: c_uint = 0x1000  /* AIF1ADC1L_HPF */;
pub const WM8994_AIF1ADC1L_HPF_MASK: c_uint = 0x1000  /* AIF1ADC1L_HPF */;

pub const WM8994_AIF1ADC1R_HPF: c_uint = 0x0800  /* AIF1ADC1R_HPF */;
pub const WM8994_AIF1ADC1R_HPF_MASK: c_uint = 0x0800  /* AIF1ADC1R_HPF */;

//
// R1041 (0x411) - AIF1 ADC2 Filters
//
pub const WM8994_AIF1ADC2_HPF_CUT_MASK: c_uint = 0x6000  /* AIF1ADC2_HPF_CUT - [14:13] */;

pub const WM8994_AIF1ADC2L_HPF: c_uint = 0x1000  /* AIF1ADC2L_HPF */;
pub const WM8994_AIF1ADC2L_HPF_MASK: c_uint = 0x1000  /* AIF1ADC2L_HPF */;

pub const WM8994_AIF1ADC2R_HPF: c_uint = 0x0800  /* AIF1ADC2R_HPF */;
pub const WM8994_AIF1ADC2R_HPF_MASK: c_uint = 0x0800  /* AIF1ADC2R_HPF */;

//
// R1056 (0x420) - AIF1 DAC1 Filters (1)
//
pub const WM8994_AIF1DAC1_MUTE: c_uint = 0x0200  /* AIF1DAC1_MUTE */;
pub const WM8994_AIF1DAC1_MUTE_MASK: c_uint = 0x0200  /* AIF1DAC1_MUTE */;

pub const WM8994_AIF1DAC1_MONO: c_uint = 0x0080  /* AIF1DAC1_MONO */;
pub const WM8994_AIF1DAC1_MONO_MASK: c_uint = 0x0080  /* AIF1DAC1_MONO */;

pub const WM8994_AIF1DAC1_MUTERATE: c_uint = 0x0020  /* AIF1DAC1_MUTERATE */;
pub const WM8994_AIF1DAC1_MUTERATE_MASK: c_uint = 0x0020  /* AIF1DAC1_MUTERATE */;

pub const WM8994_AIF1DAC1_UNMUTE_RAMP: c_uint = 0x0010  /* AIF1DAC1_UNMUTE_RAMP */;
pub const WM8994_AIF1DAC1_UNMUTE_RAMP_MASK: c_uint = 0x0010  /* AIF1DAC1_UNMUTE_RAMP */;

pub const WM8994_AIF1DAC1_DEEMP_MASK: c_uint = 0x0006  /* AIF1DAC1_DEEMP - [2:1] */;

//
// R1057 (0x421) - AIF1 DAC1 Filters (2)
//
pub const WM8994_AIF1DAC1_3D_GAIN_MASK: c_uint = 0x3E00  /* AIF1DAC1_3D_GAIN - [13:9] */;

pub const WM8994_AIF1DAC1_3D_ENA: c_uint = 0x0100  /* AIF1DAC1_3D_ENA */;
pub const WM8994_AIF1DAC1_3D_ENA_MASK: c_uint = 0x0100  /* AIF1DAC1_3D_ENA */;

//
// R1058 (0x422) - AIF1 DAC2 Filters (1)
//
pub const WM8994_AIF1DAC2_MUTE: c_uint = 0x0200  /* AIF1DAC2_MUTE */;
pub const WM8994_AIF1DAC2_MUTE_MASK: c_uint = 0x0200  /* AIF1DAC2_MUTE */;

pub const WM8994_AIF1DAC2_MONO: c_uint = 0x0080  /* AIF1DAC2_MONO */;
pub const WM8994_AIF1DAC2_MONO_MASK: c_uint = 0x0080  /* AIF1DAC2_MONO */;

pub const WM8994_AIF1DAC2_MUTERATE: c_uint = 0x0020  /* AIF1DAC2_MUTERATE */;
pub const WM8994_AIF1DAC2_MUTERATE_MASK: c_uint = 0x0020  /* AIF1DAC2_MUTERATE */;

pub const WM8994_AIF1DAC2_UNMUTE_RAMP: c_uint = 0x0010  /* AIF1DAC2_UNMUTE_RAMP */;
pub const WM8994_AIF1DAC2_UNMUTE_RAMP_MASK: c_uint = 0x0010  /* AIF1DAC2_UNMUTE_RAMP */;

pub const WM8994_AIF1DAC2_DEEMP_MASK: c_uint = 0x0006  /* AIF1DAC2_DEEMP - [2:1] */;

//
// R1059 (0x423) - AIF1 DAC2 Filters (2)
//
pub const WM8994_AIF1DAC2_3D_GAIN_MASK: c_uint = 0x3E00  /* AIF1DAC2_3D_GAIN - [13:9] */;

pub const WM8994_AIF1DAC2_3D_ENA: c_uint = 0x0100  /* AIF1DAC2_3D_ENA */;
pub const WM8994_AIF1DAC2_3D_ENA_MASK: c_uint = 0x0100  /* AIF1DAC2_3D_ENA */;

//
// R1072 (0x430) - AIF1 DAC1 Noise Gate
//
pub const WM8958_AIF1DAC1_NG_HLD_MASK: c_uint = 0x0060  /* AIF1DAC1_NG_HLD - [6:5] */;

pub const WM8958_AIF1DAC1_NG_THR_MASK: c_uint = 0x000E  /* AIF1DAC1_NG_THR - [3:1] */;

pub const WM8958_AIF1DAC1_NG_ENA: c_uint = 0x0001  /* AIF1DAC1_NG_ENA */;
pub const WM8958_AIF1DAC1_NG_ENA_MASK: c_uint = 0x0001  /* AIF1DAC1_NG_ENA */;

//
// R1073 (0x431) - AIF1 DAC2 Noise Gate
//
pub const WM8958_AIF1DAC2_NG_HLD_MASK: c_uint = 0x0060  /* AIF1DAC2_NG_HLD - [6:5] */;

pub const WM8958_AIF1DAC2_NG_THR_MASK: c_uint = 0x000E  /* AIF1DAC2_NG_THR - [3:1] */;

pub const WM8958_AIF1DAC2_NG_ENA: c_uint = 0x0001  /* AIF1DAC2_NG_ENA */;
pub const WM8958_AIF1DAC2_NG_ENA_MASK: c_uint = 0x0001  /* AIF1DAC2_NG_ENA */;

//
// R1088 (0x440) - AIF1 DRC1 (1)
//
pub const WM8994_AIF1DRC1_SIG_DET_RMS_MASK: c_uint = 0xF800  /* AIF1DRC1_SIG_DET_RMS - [15:11] */;

pub const WM8994_AIF1DRC1_SIG_DET_PK_MASK: c_uint = 0x0600  /* AIF1DRC1_SIG_DET_PK - [10:9] */;

pub const WM8994_AIF1DRC1_NG_ENA: c_uint = 0x0100  /* AIF1DRC1_NG_ENA */;
pub const WM8994_AIF1DRC1_NG_ENA_MASK: c_uint = 0x0100  /* AIF1DRC1_NG_ENA */;

pub const WM8994_AIF1DRC1_SIG_DET_MODE: c_uint = 0x0080  /* AIF1DRC1_SIG_DET_MODE */;
pub const WM8994_AIF1DRC1_SIG_DET_MODE_MASK: c_uint = 0x0080  /* AIF1DRC1_SIG_DET_MODE */;

pub const WM8994_AIF1DRC1_SIG_DET: c_uint = 0x0040  /* AIF1DRC1_SIG_DET */;
pub const WM8994_AIF1DRC1_SIG_DET_MASK: c_uint = 0x0040  /* AIF1DRC1_SIG_DET */;

pub const WM8994_AIF1DRC1_KNEE2_OP_ENA: c_uint = 0x0020  /* AIF1DRC1_KNEE2_OP_ENA */;
pub const WM8994_AIF1DRC1_KNEE2_OP_ENA_MASK: c_uint = 0x0020  /* AIF1DRC1_KNEE2_OP_ENA */;

pub const WM8994_AIF1DRC1_QR: c_uint = 0x0010  /* AIF1DRC1_QR */;
pub const WM8994_AIF1DRC1_QR_MASK: c_uint = 0x0010  /* AIF1DRC1_QR */;

pub const WM8994_AIF1DRC1_ANTICLIP: c_uint = 0x0008  /* AIF1DRC1_ANTICLIP */;
pub const WM8994_AIF1DRC1_ANTICLIP_MASK: c_uint = 0x0008  /* AIF1DRC1_ANTICLIP */;

pub const WM8994_AIF1DAC1_DRC_ENA: c_uint = 0x0004  /* AIF1DAC1_DRC_ENA */;
pub const WM8994_AIF1DAC1_DRC_ENA_MASK: c_uint = 0x0004  /* AIF1DAC1_DRC_ENA */;

pub const WM8994_AIF1ADC1L_DRC_ENA: c_uint = 0x0002  /* AIF1ADC1L_DRC_ENA */;
pub const WM8994_AIF1ADC1L_DRC_ENA_MASK: c_uint = 0x0002  /* AIF1ADC1L_DRC_ENA */;

pub const WM8994_AIF1ADC1R_DRC_ENA: c_uint = 0x0001  /* AIF1ADC1R_DRC_ENA */;
pub const WM8994_AIF1ADC1R_DRC_ENA_MASK: c_uint = 0x0001  /* AIF1ADC1R_DRC_ENA */;

//
// R1089 (0x441) - AIF1 DRC1 (2)
//
pub const WM8994_AIF1DRC1_ATK_MASK: c_uint = 0x1E00  /* AIF1DRC1_ATK - [12:9] */;

pub const WM8994_AIF1DRC1_DCY_MASK: c_uint = 0x01E0  /* AIF1DRC1_DCY - [8:5] */;

pub const WM8994_AIF1DRC1_MINGAIN_MASK: c_uint = 0x001C  /* AIF1DRC1_MINGAIN - [4:2] */;

pub const WM8994_AIF1DRC1_MAXGAIN_MASK: c_uint = 0x0003  /* AIF1DRC1_MAXGAIN - [1:0] */;

//
// R1090 (0x442) - AIF1 DRC1 (3)
//
pub const WM8994_AIF1DRC1_NG_MINGAIN_MASK: c_uint = 0xF000  /* AIF1DRC1_NG_MINGAIN - [15:12] */;

pub const WM8994_AIF1DRC1_NG_EXP_MASK: c_uint = 0x0C00  /* AIF1DRC1_NG_EXP - [11:10] */;

pub const WM8994_AIF1DRC1_QR_THR_MASK: c_uint = 0x0300  /* AIF1DRC1_QR_THR - [9:8] */;

pub const WM8994_AIF1DRC1_QR_DCY_MASK: c_uint = 0x00C0  /* AIF1DRC1_QR_DCY - [7:6] */;

pub const WM8994_AIF1DRC1_HI_COMP_MASK: c_uint = 0x0038  /* AIF1DRC1_HI_COMP - [5:3] */;

pub const WM8994_AIF1DRC1_LO_COMP_MASK: c_uint = 0x0007  /* AIF1DRC1_LO_COMP - [2:0] */;

//
// R1091 (0x443) - AIF1 DRC1 (4)
//
pub const WM8994_AIF1DRC1_KNEE_IP_MASK: c_uint = 0x07E0  /* AIF1DRC1_KNEE_IP - [10:5] */;

pub const WM8994_AIF1DRC1_KNEE_OP_MASK: c_uint = 0x001F  /* AIF1DRC1_KNEE_OP - [4:0] */;

//
// R1092 (0x444) - AIF1 DRC1 (5)
//
pub const WM8994_AIF1DRC1_KNEE2_IP_MASK: c_uint = 0x03E0  /* AIF1DRC1_KNEE2_IP - [9:5] */;

pub const WM8994_AIF1DRC1_KNEE2_OP_MASK: c_uint = 0x001F  /* AIF1DRC1_KNEE2_OP - [4:0] */;

//
// R1104 (0x450) - AIF1 DRC2 (1)
//
pub const WM8994_AIF1DRC2_SIG_DET_RMS_MASK: c_uint = 0xF800  /* AIF1DRC2_SIG_DET_RMS - [15:11] */;

pub const WM8994_AIF1DRC2_SIG_DET_PK_MASK: c_uint = 0x0600  /* AIF1DRC2_SIG_DET_PK - [10:9] */;

pub const WM8994_AIF1DRC2_NG_ENA: c_uint = 0x0100  /* AIF1DRC2_NG_ENA */;
pub const WM8994_AIF1DRC2_NG_ENA_MASK: c_uint = 0x0100  /* AIF1DRC2_NG_ENA */;

pub const WM8994_AIF1DRC2_SIG_DET_MODE: c_uint = 0x0080  /* AIF1DRC2_SIG_DET_MODE */;
pub const WM8994_AIF1DRC2_SIG_DET_MODE_MASK: c_uint = 0x0080  /* AIF1DRC2_SIG_DET_MODE */;

pub const WM8994_AIF1DRC2_SIG_DET: c_uint = 0x0040  /* AIF1DRC2_SIG_DET */;
pub const WM8994_AIF1DRC2_SIG_DET_MASK: c_uint = 0x0040  /* AIF1DRC2_SIG_DET */;

pub const WM8994_AIF1DRC2_KNEE2_OP_ENA: c_uint = 0x0020  /* AIF1DRC2_KNEE2_OP_ENA */;
pub const WM8994_AIF1DRC2_KNEE2_OP_ENA_MASK: c_uint = 0x0020  /* AIF1DRC2_KNEE2_OP_ENA */;

pub const WM8994_AIF1DRC2_QR: c_uint = 0x0010  /* AIF1DRC2_QR */;
pub const WM8994_AIF1DRC2_QR_MASK: c_uint = 0x0010  /* AIF1DRC2_QR */;

pub const WM8994_AIF1DRC2_ANTICLIP: c_uint = 0x0008  /* AIF1DRC2_ANTICLIP */;
pub const WM8994_AIF1DRC2_ANTICLIP_MASK: c_uint = 0x0008  /* AIF1DRC2_ANTICLIP */;

pub const WM8994_AIF1DAC2_DRC_ENA: c_uint = 0x0004  /* AIF1DAC2_DRC_ENA */;
pub const WM8994_AIF1DAC2_DRC_ENA_MASK: c_uint = 0x0004  /* AIF1DAC2_DRC_ENA */;

pub const WM8994_AIF1ADC2L_DRC_ENA: c_uint = 0x0002  /* AIF1ADC2L_DRC_ENA */;
pub const WM8994_AIF1ADC2L_DRC_ENA_MASK: c_uint = 0x0002  /* AIF1ADC2L_DRC_ENA */;

pub const WM8994_AIF1ADC2R_DRC_ENA: c_uint = 0x0001  /* AIF1ADC2R_DRC_ENA */;
pub const WM8994_AIF1ADC2R_DRC_ENA_MASK: c_uint = 0x0001  /* AIF1ADC2R_DRC_ENA */;

//
// R1105 (0x451) - AIF1 DRC2 (2)
//
pub const WM8994_AIF1DRC2_ATK_MASK: c_uint = 0x1E00  /* AIF1DRC2_ATK - [12:9] */;

pub const WM8994_AIF1DRC2_DCY_MASK: c_uint = 0x01E0  /* AIF1DRC2_DCY - [8:5] */;

pub const WM8994_AIF1DRC2_MINGAIN_MASK: c_uint = 0x001C  /* AIF1DRC2_MINGAIN - [4:2] */;

pub const WM8994_AIF1DRC2_MAXGAIN_MASK: c_uint = 0x0003  /* AIF1DRC2_MAXGAIN - [1:0] */;

//
// R1106 (0x452) - AIF1 DRC2 (3)
//
pub const WM8994_AIF1DRC2_NG_MINGAIN_MASK: c_uint = 0xF000  /* AIF1DRC2_NG_MINGAIN - [15:12] */;

pub const WM8994_AIF1DRC2_NG_EXP_MASK: c_uint = 0x0C00  /* AIF1DRC2_NG_EXP - [11:10] */;

pub const WM8994_AIF1DRC2_QR_THR_MASK: c_uint = 0x0300  /* AIF1DRC2_QR_THR - [9:8] */;

pub const WM8994_AIF1DRC2_QR_DCY_MASK: c_uint = 0x00C0  /* AIF1DRC2_QR_DCY - [7:6] */;

pub const WM8994_AIF1DRC2_HI_COMP_MASK: c_uint = 0x0038  /* AIF1DRC2_HI_COMP - [5:3] */;

pub const WM8994_AIF1DRC2_LO_COMP_MASK: c_uint = 0x0007  /* AIF1DRC2_LO_COMP - [2:0] */;

//
// R1107 (0x453) - AIF1 DRC2 (4)
//
pub const WM8994_AIF1DRC2_KNEE_IP_MASK: c_uint = 0x07E0  /* AIF1DRC2_KNEE_IP - [10:5] */;

pub const WM8994_AIF1DRC2_KNEE_OP_MASK: c_uint = 0x001F  /* AIF1DRC2_KNEE_OP - [4:0] */;

//
// R1108 (0x454) - AIF1 DRC2 (5)
//
pub const WM8994_AIF1DRC2_KNEE2_IP_MASK: c_uint = 0x03E0  /* AIF1DRC2_KNEE2_IP - [9:5] */;

pub const WM8994_AIF1DRC2_KNEE2_OP_MASK: c_uint = 0x001F  /* AIF1DRC2_KNEE2_OP - [4:0] */;

//
// R1152 (0x480) - AIF1 DAC1 EQ Gains (1)
//
pub const WM8994_AIF1DAC1_EQ_B1_GAIN_MASK: c_uint = 0xF800  /* AIF1DAC1_EQ_B1_GAIN - [15:11] */;

pub const WM8994_AIF1DAC1_EQ_B2_GAIN_MASK: c_uint = 0x07C0  /* AIF1DAC1_EQ_B2_GAIN - [10:6] */;

pub const WM8994_AIF1DAC1_EQ_B3_GAIN_MASK: c_uint = 0x003E  /* AIF1DAC1_EQ_B3_GAIN - [5:1] */;

pub const WM8994_AIF1DAC1_EQ_ENA: c_uint = 0x0001  /* AIF1DAC1_EQ_ENA */;
pub const WM8994_AIF1DAC1_EQ_ENA_MASK: c_uint = 0x0001  /* AIF1DAC1_EQ_ENA */;

//
// R1153 (0x481) - AIF1 DAC1 EQ Gains (2)
//
pub const WM8994_AIF1DAC1_EQ_B4_GAIN_MASK: c_uint = 0xF800  /* AIF1DAC1_EQ_B4_GAIN - [15:11] */;

pub const WM8994_AIF1DAC1_EQ_B5_GAIN_MASK: c_uint = 0x07C0  /* AIF1DAC1_EQ_B5_GAIN - [10:6] */;

//
// R1154 (0x482) - AIF1 DAC1 EQ Band 1 A
//
pub const WM8994_AIF1DAC1_EQ_B1_A_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B1_A - [15:0] */;

//
// R1155 (0x483) - AIF1 DAC1 EQ Band 1 B
//
pub const WM8994_AIF1DAC1_EQ_B1_B_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B1_B - [15:0] */;

//
// R1156 (0x484) - AIF1 DAC1 EQ Band 1 PG
//
pub const WM8994_AIF1DAC1_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B1_PG - [15:0] */;

//
// R1157 (0x485) - AIF1 DAC1 EQ Band 2 A
//
pub const WM8994_AIF1DAC1_EQ_B2_A_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B2_A - [15:0] */;

//
// R1158 (0x486) - AIF1 DAC1 EQ Band 2 B
//
pub const WM8994_AIF1DAC1_EQ_B2_B_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B2_B - [15:0] */;

//
// R1159 (0x487) - AIF1 DAC1 EQ Band 2 C
//
pub const WM8994_AIF1DAC1_EQ_B2_C_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B2_C - [15:0] */;

//
// R1160 (0x488) - AIF1 DAC1 EQ Band 2 PG
//
pub const WM8994_AIF1DAC1_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B2_PG - [15:0] */;

//
// R1161 (0x489) - AIF1 DAC1 EQ Band 3 A
//
pub const WM8994_AIF1DAC1_EQ_B3_A_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B3_A - [15:0] */;

//
// R1162 (0x48A) - AIF1 DAC1 EQ Band 3 B
//
pub const WM8994_AIF1DAC1_EQ_B3_B_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B3_B - [15:0] */;

//
// R1163 (0x48B) - AIF1 DAC1 EQ Band 3 C
//
pub const WM8994_AIF1DAC1_EQ_B3_C_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B3_C - [15:0] */;

//
// R1164 (0x48C) - AIF1 DAC1 EQ Band 3 PG
//
pub const WM8994_AIF1DAC1_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B3_PG - [15:0] */;

//
// R1165 (0x48D) - AIF1 DAC1 EQ Band 4 A
//
pub const WM8994_AIF1DAC1_EQ_B4_A_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B4_A - [15:0] */;

//
// R1166 (0x48E) - AIF1 DAC1 EQ Band 4 B
//
pub const WM8994_AIF1DAC1_EQ_B4_B_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B4_B - [15:0] */;

//
// R1167 (0x48F) - AIF1 DAC1 EQ Band 4 C
//
pub const WM8994_AIF1DAC1_EQ_B4_C_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B4_C - [15:0] */;

//
// R1168 (0x490) - AIF1 DAC1 EQ Band 4 PG
//
pub const WM8994_AIF1DAC1_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B4_PG - [15:0] */;

//
// R1169 (0x491) - AIF1 DAC1 EQ Band 5 A
//
pub const WM8994_AIF1DAC1_EQ_B5_A_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B5_A - [15:0] */;

//
// R1170 (0x492) - AIF1 DAC1 EQ Band 5 B
//
pub const WM8994_AIF1DAC1_EQ_B5_B_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B5_B - [15:0] */;

//
// R1171 (0x493) - AIF1 DAC1 EQ Band 5 PG
//
pub const WM8994_AIF1DAC1_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC1_EQ_B5_PG - [15:0] */;

//
// R1184 (0x4A0) - AIF1 DAC2 EQ Gains (1)
//
pub const WM8994_AIF1DAC2_EQ_B1_GAIN_MASK: c_uint = 0xF800  /* AIF1DAC2_EQ_B1_GAIN - [15:11] */;

pub const WM8994_AIF1DAC2_EQ_B2_GAIN_MASK: c_uint = 0x07C0  /* AIF1DAC2_EQ_B2_GAIN - [10:6] */;

pub const WM8994_AIF1DAC2_EQ_B3_GAIN_MASK: c_uint = 0x003E  /* AIF1DAC2_EQ_B3_GAIN - [5:1] */;

pub const WM8994_AIF1DAC2_EQ_ENA: c_uint = 0x0001  /* AIF1DAC2_EQ_ENA */;
pub const WM8994_AIF1DAC2_EQ_ENA_MASK: c_uint = 0x0001  /* AIF1DAC2_EQ_ENA */;

//
// R1185 (0x4A1) - AIF1 DAC2 EQ Gains (2)
//
pub const WM8994_AIF1DAC2_EQ_B4_GAIN_MASK: c_uint = 0xF800  /* AIF1DAC2_EQ_B4_GAIN - [15:11] */;

pub const WM8994_AIF1DAC2_EQ_B5_GAIN_MASK: c_uint = 0x07C0  /* AIF1DAC2_EQ_B5_GAIN - [10:6] */;

//
// R1186 (0x4A2) - AIF1 DAC2 EQ Band 1 A
//
pub const WM8994_AIF1DAC2_EQ_B1_A_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B1_A - [15:0] */;

//
// R1187 (0x4A3) - AIF1 DAC2 EQ Band 1 B
//
pub const WM8994_AIF1DAC2_EQ_B1_B_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B1_B - [15:0] */;

//
// R1188 (0x4A4) - AIF1 DAC2 EQ Band 1 PG
//
pub const WM8994_AIF1DAC2_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B1_PG - [15:0] */;

//
// R1189 (0x4A5) - AIF1 DAC2 EQ Band 2 A
//
pub const WM8994_AIF1DAC2_EQ_B2_A_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B2_A - [15:0] */;

//
// R1190 (0x4A6) - AIF1 DAC2 EQ Band 2 B
//
pub const WM8994_AIF1DAC2_EQ_B2_B_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B2_B - [15:0] */;

//
// R1191 (0x4A7) - AIF1 DAC2 EQ Band 2 C
//
pub const WM8994_AIF1DAC2_EQ_B2_C_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B2_C - [15:0] */;

//
// R1192 (0x4A8) - AIF1 DAC2 EQ Band 2 PG
//
pub const WM8994_AIF1DAC2_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B2_PG - [15:0] */;

//
// R1193 (0x4A9) - AIF1 DAC2 EQ Band 3 A
//
pub const WM8994_AIF1DAC2_EQ_B3_A_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B3_A - [15:0] */;

//
// R1194 (0x4AA) - AIF1 DAC2 EQ Band 3 B
//
pub const WM8994_AIF1DAC2_EQ_B3_B_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B3_B - [15:0] */;

//
// R1195 (0x4AB) - AIF1 DAC2 EQ Band 3 C
//
pub const WM8994_AIF1DAC2_EQ_B3_C_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B3_C - [15:0] */;

//
// R1196 (0x4AC) - AIF1 DAC2 EQ Band 3 PG
//
pub const WM8994_AIF1DAC2_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B3_PG - [15:0] */;

//
// R1197 (0x4AD) - AIF1 DAC2 EQ Band 4 A
//
pub const WM8994_AIF1DAC2_EQ_B4_A_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B4_A - [15:0] */;

//
// R1198 (0x4AE) - AIF1 DAC2 EQ Band 4 B
//
pub const WM8994_AIF1DAC2_EQ_B4_B_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B4_B - [15:0] */;

//
// R1199 (0x4AF) - AIF1 DAC2 EQ Band 4 C
//
pub const WM8994_AIF1DAC2_EQ_B4_C_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B4_C - [15:0] */;

//
// R1200 (0x4B0) - AIF1 DAC2 EQ Band 4 PG
//
pub const WM8994_AIF1DAC2_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B4_PG - [15:0] */;

//
// R1201 (0x4B1) - AIF1 DAC2 EQ Band 5 A
//
pub const WM8994_AIF1DAC2_EQ_B5_A_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B5_A - [15:0] */;

//
// R1202 (0x4B2) - AIF1 DAC2 EQ Band 5 B
//
pub const WM8994_AIF1DAC2_EQ_B5_B_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B5_B - [15:0] */;

//
// R1203 (0x4B3) - AIF1 DAC2 EQ Band 5 PG
//
pub const WM8994_AIF1DAC2_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* AIF1DAC2_EQ_B5_PG - [15:0] */;

//
// R1280 (0x500) - AIF2 ADC Left Volume
//
pub const WM8994_AIF2ADC_VU: c_uint = 0x0100  /* AIF2ADC_VU */;
pub const WM8994_AIF2ADC_VU_MASK: c_uint = 0x0100  /* AIF2ADC_VU */;

pub const WM8994_AIF2ADCL_VOL_MASK: c_uint = 0x00FF  /* AIF2ADCL_VOL - [7:0] */;

//
// R1281 (0x501) - AIF2 ADC Right Volume
//
pub const WM8994_AIF2ADC_VU: c_uint = 0x0100  /* AIF2ADC_VU */;
pub const WM8994_AIF2ADC_VU_MASK: c_uint = 0x0100  /* AIF2ADC_VU */;

pub const WM8994_AIF2ADCR_VOL_MASK: c_uint = 0x00FF  /* AIF2ADCR_VOL - [7:0] */;

//
// R1282 (0x502) - AIF2 DAC Left Volume
//
pub const WM8994_AIF2DAC_VU: c_uint = 0x0100  /* AIF2DAC_VU */;
pub const WM8994_AIF2DAC_VU_MASK: c_uint = 0x0100  /* AIF2DAC_VU */;

pub const WM8994_AIF2DACL_VOL_MASK: c_uint = 0x00FF  /* AIF2DACL_VOL - [7:0] */;

//
// R1283 (0x503) - AIF2 DAC Right Volume
//
pub const WM8994_AIF2DAC_VU: c_uint = 0x0100  /* AIF2DAC_VU */;
pub const WM8994_AIF2DAC_VU_MASK: c_uint = 0x0100  /* AIF2DAC_VU */;

pub const WM8994_AIF2DACR_VOL_MASK: c_uint = 0x00FF  /* AIF2DACR_VOL - [7:0] */;

//
// R1296 (0x510) - AIF2 ADC Filters
//
pub const WM8994_AIF2ADC_4FS: c_uint = 0x8000  /* AIF2ADC_4FS */;
pub const WM8994_AIF2ADC_4FS_MASK: c_uint = 0x8000  /* AIF2ADC_4FS */;

pub const WM8994_AIF2ADC_HPF_CUT_MASK: c_uint = 0x6000  /* AIF2ADC_HPF_CUT - [14:13] */;

pub const WM8994_AIF2ADCL_HPF: c_uint = 0x1000  /* AIF2ADCL_HPF */;
pub const WM8994_AIF2ADCL_HPF_MASK: c_uint = 0x1000  /* AIF2ADCL_HPF */;

pub const WM8994_AIF2ADCR_HPF: c_uint = 0x0800  /* AIF2ADCR_HPF */;
pub const WM8994_AIF2ADCR_HPF_MASK: c_uint = 0x0800  /* AIF2ADCR_HPF */;

//
// R1312 (0x520) - AIF2 DAC Filters (1)
//
pub const WM8994_AIF2DAC_MUTE: c_uint = 0x0200  /* AIF2DAC_MUTE */;
pub const WM8994_AIF2DAC_MUTE_MASK: c_uint = 0x0200  /* AIF2DAC_MUTE */;

pub const WM8994_AIF2DAC_MONO: c_uint = 0x0080  /* AIF2DAC_MONO */;
pub const WM8994_AIF2DAC_MONO_MASK: c_uint = 0x0080  /* AIF2DAC_MONO */;

pub const WM8994_AIF2DAC_MUTERATE: c_uint = 0x0020  /* AIF2DAC_MUTERATE */;
pub const WM8994_AIF2DAC_MUTERATE_MASK: c_uint = 0x0020  /* AIF2DAC_MUTERATE */;

pub const WM8994_AIF2DAC_UNMUTE_RAMP: c_uint = 0x0010  /* AIF2DAC_UNMUTE_RAMP */;
pub const WM8994_AIF2DAC_UNMUTE_RAMP_MASK: c_uint = 0x0010  /* AIF2DAC_UNMUTE_RAMP */;

pub const WM8994_AIF2DAC_DEEMP_MASK: c_uint = 0x0006  /* AIF2DAC_DEEMP - [2:1] */;

//
// R1313 (0x521) - AIF2 DAC Filters (2)
//
pub const WM8994_AIF2DAC_3D_GAIN_MASK: c_uint = 0x3E00  /* AIF2DAC_3D_GAIN - [13:9] */;

pub const WM8994_AIF2DAC_3D_ENA: c_uint = 0x0100  /* AIF2DAC_3D_ENA */;
pub const WM8994_AIF2DAC_3D_ENA_MASK: c_uint = 0x0100  /* AIF2DAC_3D_ENA */;

//
// R1328 (0x530) - AIF2 DAC Noise Gate
//
pub const WM8958_AIF2DAC_NG_HLD_MASK: c_uint = 0x0060  /* AIF2DAC_NG_HLD - [6:5] */;

pub const WM8958_AIF2DAC_NG_THR_MASK: c_uint = 0x000E  /* AIF2DAC_NG_THR - [3:1] */;

pub const WM8958_AIF2DAC_NG_ENA: c_uint = 0x0001  /* AIF2DAC_NG_ENA */;
pub const WM8958_AIF2DAC_NG_ENA_MASK: c_uint = 0x0001  /* AIF2DAC_NG_ENA */;

//
// R1344 (0x540) - AIF2 DRC (1)
//
pub const WM8994_AIF2DRC_SIG_DET_RMS_MASK: c_uint = 0xF800  /* AIF2DRC_SIG_DET_RMS - [15:11] */;

pub const WM8994_AIF2DRC_SIG_DET_PK_MASK: c_uint = 0x0600  /* AIF2DRC_SIG_DET_PK - [10:9] */;

pub const WM8994_AIF2DRC_NG_ENA: c_uint = 0x0100  /* AIF2DRC_NG_ENA */;
pub const WM8994_AIF2DRC_NG_ENA_MASK: c_uint = 0x0100  /* AIF2DRC_NG_ENA */;

pub const WM8994_AIF2DRC_SIG_DET_MODE: c_uint = 0x0080  /* AIF2DRC_SIG_DET_MODE */;
pub const WM8994_AIF2DRC_SIG_DET_MODE_MASK: c_uint = 0x0080  /* AIF2DRC_SIG_DET_MODE */;

pub const WM8994_AIF2DRC_SIG_DET: c_uint = 0x0040  /* AIF2DRC_SIG_DET */;
pub const WM8994_AIF2DRC_SIG_DET_MASK: c_uint = 0x0040  /* AIF2DRC_SIG_DET */;

pub const WM8994_AIF2DRC_KNEE2_OP_ENA: c_uint = 0x0020  /* AIF2DRC_KNEE2_OP_ENA */;
pub const WM8994_AIF2DRC_KNEE2_OP_ENA_MASK: c_uint = 0x0020  /* AIF2DRC_KNEE2_OP_ENA */;

pub const WM8994_AIF2DRC_QR: c_uint = 0x0010  /* AIF2DRC_QR */;
pub const WM8994_AIF2DRC_QR_MASK: c_uint = 0x0010  /* AIF2DRC_QR */;

pub const WM8994_AIF2DRC_ANTICLIP: c_uint = 0x0008  /* AIF2DRC_ANTICLIP */;
pub const WM8994_AIF2DRC_ANTICLIP_MASK: c_uint = 0x0008  /* AIF2DRC_ANTICLIP */;

pub const WM8994_AIF2DAC_DRC_ENA: c_uint = 0x0004  /* AIF2DAC_DRC_ENA */;
pub const WM8994_AIF2DAC_DRC_ENA_MASK: c_uint = 0x0004  /* AIF2DAC_DRC_ENA */;

pub const WM8994_AIF2ADCL_DRC_ENA: c_uint = 0x0002  /* AIF2ADCL_DRC_ENA */;
pub const WM8994_AIF2ADCL_DRC_ENA_MASK: c_uint = 0x0002  /* AIF2ADCL_DRC_ENA */;

pub const WM8994_AIF2ADCR_DRC_ENA: c_uint = 0x0001  /* AIF2ADCR_DRC_ENA */;
pub const WM8994_AIF2ADCR_DRC_ENA_MASK: c_uint = 0x0001  /* AIF2ADCR_DRC_ENA */;

//
// R1345 (0x541) - AIF2 DRC (2)
//
pub const WM8994_AIF2DRC_ATK_MASK: c_uint = 0x1E00  /* AIF2DRC_ATK - [12:9] */;

pub const WM8994_AIF2DRC_DCY_MASK: c_uint = 0x01E0  /* AIF2DRC_DCY - [8:5] */;

pub const WM8994_AIF2DRC_MINGAIN_MASK: c_uint = 0x001C  /* AIF2DRC_MINGAIN - [4:2] */;

pub const WM8994_AIF2DRC_MAXGAIN_MASK: c_uint = 0x0003  /* AIF2DRC_MAXGAIN - [1:0] */;

//
// R1346 (0x542) - AIF2 DRC (3)
//
pub const WM8994_AIF2DRC_NG_MINGAIN_MASK: c_uint = 0xF000  /* AIF2DRC_NG_MINGAIN - [15:12] */;

pub const WM8994_AIF2DRC_NG_EXP_MASK: c_uint = 0x0C00  /* AIF2DRC_NG_EXP - [11:10] */;

pub const WM8994_AIF2DRC_QR_THR_MASK: c_uint = 0x0300  /* AIF2DRC_QR_THR - [9:8] */;

pub const WM8994_AIF2DRC_QR_DCY_MASK: c_uint = 0x00C0  /* AIF2DRC_QR_DCY - [7:6] */;

pub const WM8994_AIF2DRC_HI_COMP_MASK: c_uint = 0x0038  /* AIF2DRC_HI_COMP - [5:3] */;

pub const WM8994_AIF2DRC_LO_COMP_MASK: c_uint = 0x0007  /* AIF2DRC_LO_COMP - [2:0] */;

//
// R1347 (0x543) - AIF2 DRC (4)
//
pub const WM8994_AIF2DRC_KNEE_IP_MASK: c_uint = 0x07E0  /* AIF2DRC_KNEE_IP - [10:5] */;

pub const WM8994_AIF2DRC_KNEE_OP_MASK: c_uint = 0x001F  /* AIF2DRC_KNEE_OP - [4:0] */;

//
// R1348 (0x544) - AIF2 DRC (5)
//
pub const WM8994_AIF2DRC_KNEE2_IP_MASK: c_uint = 0x03E0  /* AIF2DRC_KNEE2_IP - [9:5] */;

pub const WM8994_AIF2DRC_KNEE2_OP_MASK: c_uint = 0x001F  /* AIF2DRC_KNEE2_OP - [4:0] */;

//
// R1408 (0x580) - AIF2 EQ Gains (1)
//
pub const WM8994_AIF2DAC_EQ_B1_GAIN_MASK: c_uint = 0xF800  /* AIF2DAC_EQ_B1_GAIN - [15:11] */;

pub const WM8994_AIF2DAC_EQ_B2_GAIN_MASK: c_uint = 0x07C0  /* AIF2DAC_EQ_B2_GAIN - [10:6] */;

pub const WM8994_AIF2DAC_EQ_B3_GAIN_MASK: c_uint = 0x003E  /* AIF2DAC_EQ_B3_GAIN - [5:1] */;

pub const WM8994_AIF2DAC_EQ_ENA: c_uint = 0x0001  /* AIF2DAC_EQ_ENA */;
pub const WM8994_AIF2DAC_EQ_ENA_MASK: c_uint = 0x0001  /* AIF2DAC_EQ_ENA */;

//
// R1409 (0x581) - AIF2 EQ Gains (2)
//
pub const WM8994_AIF2DAC_EQ_B4_GAIN_MASK: c_uint = 0xF800  /* AIF2DAC_EQ_B4_GAIN - [15:11] */;

pub const WM8994_AIF2DAC_EQ_B5_GAIN_MASK: c_uint = 0x07C0  /* AIF2DAC_EQ_B5_GAIN - [10:6] */;

//
// R1410 (0x582) - AIF2 EQ Band 1 A
//
pub const WM8994_AIF2DAC_EQ_B1_A_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B1_A - [15:0] */;

//
// R1411 (0x583) - AIF2 EQ Band 1 B
//
pub const WM8994_AIF2DAC_EQ_B1_B_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B1_B - [15:0] */;

//
// R1412 (0x584) - AIF2 EQ Band 1 PG
//
pub const WM8994_AIF2DAC_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B1_PG - [15:0] */;

//
// R1413 (0x585) - AIF2 EQ Band 2 A
//
pub const WM8994_AIF2DAC_EQ_B2_A_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B2_A - [15:0] */;

//
// R1414 (0x586) - AIF2 EQ Band 2 B
//
pub const WM8994_AIF2DAC_EQ_B2_B_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B2_B - [15:0] */;

//
// R1415 (0x587) - AIF2 EQ Band 2 C
//
pub const WM8994_AIF2DAC_EQ_B2_C_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B2_C - [15:0] */;

//
// R1416 (0x588) - AIF2 EQ Band 2 PG
//
pub const WM8994_AIF2DAC_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B2_PG - [15:0] */;

//
// R1417 (0x589) - AIF2 EQ Band 3 A
//
pub const WM8994_AIF2DAC_EQ_B3_A_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B3_A - [15:0] */;

//
// R1418 (0x58A) - AIF2 EQ Band 3 B
//
pub const WM8994_AIF2DAC_EQ_B3_B_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B3_B - [15:0] */;

//
// R1419 (0x58B) - AIF2 EQ Band 3 C
//
pub const WM8994_AIF2DAC_EQ_B3_C_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B3_C - [15:0] */;

//
// R1420 (0x58C) - AIF2 EQ Band 3 PG
//
pub const WM8994_AIF2DAC_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B3_PG - [15:0] */;

//
// R1421 (0x58D) - AIF2 EQ Band 4 A
//
pub const WM8994_AIF2DAC_EQ_B4_A_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B4_A - [15:0] */;

//
// R1422 (0x58E) - AIF2 EQ Band 4 B
//
pub const WM8994_AIF2DAC_EQ_B4_B_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B4_B - [15:0] */;

//
// R1423 (0x58F) - AIF2 EQ Band 4 C
//
pub const WM8994_AIF2DAC_EQ_B4_C_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B4_C - [15:0] */;

//
// R1424 (0x590) - AIF2 EQ Band 4 PG
//
pub const WM8994_AIF2DAC_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B4_PG - [15:0] */;

//
// R1425 (0x591) - AIF2 EQ Band 5 A
//
pub const WM8994_AIF2DAC_EQ_B5_A_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B5_A - [15:0] */;

//
// R1426 (0x592) - AIF2 EQ Band 5 B
//
pub const WM8994_AIF2DAC_EQ_B5_B_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B5_B - [15:0] */;

//
// R1427 (0x593) - AIF2 EQ Band 5 PG
//
pub const WM8994_AIF2DAC_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* AIF2DAC_EQ_B5_PG - [15:0] */;

//
// R1536 (0x600) - DAC1 Mixer Volumes
//
pub const WM8994_ADCR_DAC1_VOL_MASK: c_uint = 0x01E0  /* ADCR_DAC1_VOL - [8:5] */;

pub const WM8994_ADCL_DAC1_VOL_MASK: c_uint = 0x000F  /* ADCL_DAC1_VOL - [3:0] */;

//
// R1537 (0x601) - DAC1 Left Mixer Routing
//
pub const WM8994_ADCR_TO_DAC1L: c_uint = 0x0020  /* ADCR_TO_DAC1L */;
pub const WM8994_ADCR_TO_DAC1L_MASK: c_uint = 0x0020  /* ADCR_TO_DAC1L */;

pub const WM8994_ADCL_TO_DAC1L: c_uint = 0x0010  /* ADCL_TO_DAC1L */;
pub const WM8994_ADCL_TO_DAC1L_MASK: c_uint = 0x0010  /* ADCL_TO_DAC1L */;

pub const WM8994_AIF2DACL_TO_DAC1L: c_uint = 0x0004  /* AIF2DACL_TO_DAC1L */;
pub const WM8994_AIF2DACL_TO_DAC1L_MASK: c_uint = 0x0004  /* AIF2DACL_TO_DAC1L */;

pub const WM8994_AIF1DAC2L_TO_DAC1L: c_uint = 0x0002  /* AIF1DAC2L_TO_DAC1L */;
pub const WM8994_AIF1DAC2L_TO_DAC1L_MASK: c_uint = 0x0002  /* AIF1DAC2L_TO_DAC1L */;

pub const WM8994_AIF1DAC1L_TO_DAC1L: c_uint = 0x0001  /* AIF1DAC1L_TO_DAC1L */;
pub const WM8994_AIF1DAC1L_TO_DAC1L_MASK: c_uint = 0x0001  /* AIF1DAC1L_TO_DAC1L */;

//
// R1538 (0x602) - DAC1 Right Mixer Routing
//
pub const WM8994_ADCR_TO_DAC1R: c_uint = 0x0020  /* ADCR_TO_DAC1R */;
pub const WM8994_ADCR_TO_DAC1R_MASK: c_uint = 0x0020  /* ADCR_TO_DAC1R */;

pub const WM8994_ADCL_TO_DAC1R: c_uint = 0x0010  /* ADCL_TO_DAC1R */;
pub const WM8994_ADCL_TO_DAC1R_MASK: c_uint = 0x0010  /* ADCL_TO_DAC1R */;

pub const WM8994_AIF2DACR_TO_DAC1R: c_uint = 0x0004  /* AIF2DACR_TO_DAC1R */;
pub const WM8994_AIF2DACR_TO_DAC1R_MASK: c_uint = 0x0004  /* AIF2DACR_TO_DAC1R */;

pub const WM8994_AIF1DAC2R_TO_DAC1R: c_uint = 0x0002  /* AIF1DAC2R_TO_DAC1R */;
pub const WM8994_AIF1DAC2R_TO_DAC1R_MASK: c_uint = 0x0002  /* AIF1DAC2R_TO_DAC1R */;

pub const WM8994_AIF1DAC1R_TO_DAC1R: c_uint = 0x0001  /* AIF1DAC1R_TO_DAC1R */;
pub const WM8994_AIF1DAC1R_TO_DAC1R_MASK: c_uint = 0x0001  /* AIF1DAC1R_TO_DAC1R */;

//
// R1539 (0x603) - DAC2 Mixer Volumes
//
pub const WM8994_ADCR_DAC2_VOL_MASK: c_uint = 0x01E0  /* ADCR_DAC2_VOL - [8:5] */;

pub const WM8994_ADCL_DAC2_VOL_MASK: c_uint = 0x000F  /* ADCL_DAC2_VOL - [3:0] */;

//
// R1540 (0x604) - DAC2 Left Mixer Routing
//
pub const WM8994_ADCR_TO_DAC2L: c_uint = 0x0020  /* ADCR_TO_DAC2L */;
pub const WM8994_ADCR_TO_DAC2L_MASK: c_uint = 0x0020  /* ADCR_TO_DAC2L */;

pub const WM8994_ADCL_TO_DAC2L: c_uint = 0x0010  /* ADCL_TO_DAC2L */;
pub const WM8994_ADCL_TO_DAC2L_MASK: c_uint = 0x0010  /* ADCL_TO_DAC2L */;

pub const WM8994_AIF2DACL_TO_DAC2L: c_uint = 0x0004  /* AIF2DACL_TO_DAC2L */;
pub const WM8994_AIF2DACL_TO_DAC2L_MASK: c_uint = 0x0004  /* AIF2DACL_TO_DAC2L */;

pub const WM8994_AIF1DAC2L_TO_DAC2L: c_uint = 0x0002  /* AIF1DAC2L_TO_DAC2L */;
pub const WM8994_AIF1DAC2L_TO_DAC2L_MASK: c_uint = 0x0002  /* AIF1DAC2L_TO_DAC2L */;

pub const WM8994_AIF1DAC1L_TO_DAC2L: c_uint = 0x0001  /* AIF1DAC1L_TO_DAC2L */;
pub const WM8994_AIF1DAC1L_TO_DAC2L_MASK: c_uint = 0x0001  /* AIF1DAC1L_TO_DAC2L */;

//
// R1541 (0x605) - DAC2 Right Mixer Routing
//
pub const WM8994_ADCR_TO_DAC2R: c_uint = 0x0020  /* ADCR_TO_DAC2R */;
pub const WM8994_ADCR_TO_DAC2R_MASK: c_uint = 0x0020  /* ADCR_TO_DAC2R */;

pub const WM8994_ADCL_TO_DAC2R: c_uint = 0x0010  /* ADCL_TO_DAC2R */;
pub const WM8994_ADCL_TO_DAC2R_MASK: c_uint = 0x0010  /* ADCL_TO_DAC2R */;

pub const WM8994_AIF2DACR_TO_DAC2R: c_uint = 0x0004  /* AIF2DACR_TO_DAC2R */;
pub const WM8994_AIF2DACR_TO_DAC2R_MASK: c_uint = 0x0004  /* AIF2DACR_TO_DAC2R */;

pub const WM8994_AIF1DAC2R_TO_DAC2R: c_uint = 0x0002  /* AIF1DAC2R_TO_DAC2R */;
pub const WM8994_AIF1DAC2R_TO_DAC2R_MASK: c_uint = 0x0002  /* AIF1DAC2R_TO_DAC2R */;

pub const WM8994_AIF1DAC1R_TO_DAC2R: c_uint = 0x0001  /* AIF1DAC1R_TO_DAC2R */;
pub const WM8994_AIF1DAC1R_TO_DAC2R_MASK: c_uint = 0x0001  /* AIF1DAC1R_TO_DAC2R */;

//
// R1542 (0x606) - AIF1 ADC1 Left Mixer Routing
//
pub const WM8994_ADC1L_TO_AIF1ADC1L: c_uint = 0x0002  /* ADC1L_TO_AIF1ADC1L */;
pub const WM8994_ADC1L_TO_AIF1ADC1L_MASK: c_uint = 0x0002  /* ADC1L_TO_AIF1ADC1L */;

pub const WM8994_AIF2DACL_TO_AIF1ADC1L: c_uint = 0x0001  /* AIF2DACL_TO_AIF1ADC1L */;
pub const WM8994_AIF2DACL_TO_AIF1ADC1L_MASK: c_uint = 0x0001  /* AIF2DACL_TO_AIF1ADC1L */;

//
// R1543 (0x607) - AIF1 ADC1 Right Mixer Routing
//
pub const WM8994_ADC1R_TO_AIF1ADC1R: c_uint = 0x0002  /* ADC1R_TO_AIF1ADC1R */;
pub const WM8994_ADC1R_TO_AIF1ADC1R_MASK: c_uint = 0x0002  /* ADC1R_TO_AIF1ADC1R */;

pub const WM8994_AIF2DACR_TO_AIF1ADC1R: c_uint = 0x0001  /* AIF2DACR_TO_AIF1ADC1R */;
pub const WM8994_AIF2DACR_TO_AIF1ADC1R_MASK: c_uint = 0x0001  /* AIF2DACR_TO_AIF1ADC1R */;

//
// R1544 (0x608) - AIF1 ADC2 Left Mixer Routing
//
pub const WM8994_ADC2L_TO_AIF1ADC2L: c_uint = 0x0002  /* ADC2L_TO_AIF1ADC2L */;
pub const WM8994_ADC2L_TO_AIF1ADC2L_MASK: c_uint = 0x0002  /* ADC2L_TO_AIF1ADC2L */;

pub const WM8994_AIF2DACL_TO_AIF1ADC2L: c_uint = 0x0001  /* AIF2DACL_TO_AIF1ADC2L */;
pub const WM8994_AIF2DACL_TO_AIF1ADC2L_MASK: c_uint = 0x0001  /* AIF2DACL_TO_AIF1ADC2L */;

//
// R1545 (0x609) - AIF1 ADC2 Right mixer Routing
//
pub const WM8994_ADC2R_TO_AIF1ADC2R: c_uint = 0x0002  /* ADC2R_TO_AIF1ADC2R */;
pub const WM8994_ADC2R_TO_AIF1ADC2R_MASK: c_uint = 0x0002  /* ADC2R_TO_AIF1ADC2R */;

pub const WM8994_AIF2DACR_TO_AIF1ADC2R: c_uint = 0x0001  /* AIF2DACR_TO_AIF1ADC2R */;
pub const WM8994_AIF2DACR_TO_AIF1ADC2R_MASK: c_uint = 0x0001  /* AIF2DACR_TO_AIF1ADC2R */;

//
// R1552 (0x610) - DAC1 Left Volume
//
pub const WM8994_DAC1L_MUTE: c_uint = 0x0200  /* DAC1L_MUTE */;
pub const WM8994_DAC1L_MUTE_MASK: c_uint = 0x0200  /* DAC1L_MUTE */;

pub const WM8994_DAC1_VU: c_uint = 0x0100  /* DAC1_VU */;
pub const WM8994_DAC1_VU_MASK: c_uint = 0x0100  /* DAC1_VU */;

pub const WM8994_DAC1L_VOL_MASK: c_uint = 0x00FF  /* DAC1L_VOL - [7:0] */;

//
// R1553 (0x611) - DAC1 Right Volume
//
pub const WM8994_DAC1R_MUTE: c_uint = 0x0200  /* DAC1R_MUTE */;
pub const WM8994_DAC1R_MUTE_MASK: c_uint = 0x0200  /* DAC1R_MUTE */;

pub const WM8994_DAC1_VU: c_uint = 0x0100  /* DAC1_VU */;
pub const WM8994_DAC1_VU_MASK: c_uint = 0x0100  /* DAC1_VU */;

pub const WM8994_DAC1R_VOL_MASK: c_uint = 0x00FF  /* DAC1R_VOL - [7:0] */;

//
// R1554 (0x612) - DAC2 Left Volume
//
pub const WM8994_DAC2L_MUTE: c_uint = 0x0200  /* DAC2L_MUTE */;
pub const WM8994_DAC2L_MUTE_MASK: c_uint = 0x0200  /* DAC2L_MUTE */;

pub const WM8994_DAC2_VU: c_uint = 0x0100  /* DAC2_VU */;
pub const WM8994_DAC2_VU_MASK: c_uint = 0x0100  /* DAC2_VU */;

pub const WM8994_DAC2L_VOL_MASK: c_uint = 0x00FF  /* DAC2L_VOL - [7:0] */;

//
// R1555 (0x613) - DAC2 Right Volume
//
pub const WM8994_DAC2R_MUTE: c_uint = 0x0200  /* DAC2R_MUTE */;
pub const WM8994_DAC2R_MUTE_MASK: c_uint = 0x0200  /* DAC2R_MUTE */;

pub const WM8994_DAC2_VU: c_uint = 0x0100  /* DAC2_VU */;
pub const WM8994_DAC2_VU_MASK: c_uint = 0x0100  /* DAC2_VU */;

pub const WM8994_DAC2R_VOL_MASK: c_uint = 0x00FF  /* DAC2R_VOL - [7:0] */;

//
// R1556 (0x614) - DAC Softmute
//
pub const WM8994_DAC_SOFTMUTEMODE: c_uint = 0x0002  /* DAC_SOFTMUTEMODE */;
pub const WM8994_DAC_SOFTMUTEMODE_MASK: c_uint = 0x0002  /* DAC_SOFTMUTEMODE */;

pub const WM8994_DAC_MUTERATE: c_uint = 0x0001  /* DAC_MUTERATE */;
pub const WM8994_DAC_MUTERATE_MASK: c_uint = 0x0001  /* DAC_MUTERATE */;

//
// R1568 (0x620) - Oversampling
//
pub const WM8994_ADC_OSR128: c_uint = 0x0002  /* ADC_OSR128 */;
pub const WM8994_ADC_OSR128_MASK: c_uint = 0x0002  /* ADC_OSR128 */;

pub const WM8994_DAC_OSR128: c_uint = 0x0001  /* DAC_OSR128 */;
pub const WM8994_DAC_OSR128_MASK: c_uint = 0x0001  /* DAC_OSR128 */;

//
// R1569 (0x621) - Sidetone
//
pub const WM8994_ST_HPF_CUT_MASK: c_uint = 0x0380  /* ST_HPF_CUT - [9:7] */;

pub const WM8994_ST_HPF: c_uint = 0x0040  /* ST_HPF */;
pub const WM8994_ST_HPF_MASK: c_uint = 0x0040  /* ST_HPF */;

pub const WM8994_STR_SEL: c_uint = 0x0002  /* STR_SEL */;
pub const WM8994_STR_SEL_MASK: c_uint = 0x0002  /* STR_SEL */;

pub const WM8994_STL_SEL: c_uint = 0x0001  /* STL_SEL */;
pub const WM8994_STL_SEL_MASK: c_uint = 0x0001  /* STL_SEL */;

//
// R1797 (0x705) - JACKDET Ctrl
//
pub const WM1811_JACKDET_DB: c_uint = 0x0100  /* JACKDET_DB */;
pub const WM1811_JACKDET_DB_MASK: c_uint = 0x0100  /* JACKDET_DB */;

pub const WM1811_JACKDET_LVL: c_uint = 0x0040  /* JACKDET_LVL */;
pub const WM1811_JACKDET_LVL_MASK: c_uint = 0x0040  /* JACKDET_LVL */;

//
// R1824 (0x720) - Pull Control (1)
//
pub const WM8994_DMICDAT2_PU: c_uint = 0x0800  /* DMICDAT2_PU */;
pub const WM8994_DMICDAT2_PU_MASK: c_uint = 0x0800  /* DMICDAT2_PU */;

pub const WM8994_DMICDAT2_PD: c_uint = 0x0400  /* DMICDAT2_PD */;
pub const WM8994_DMICDAT2_PD_MASK: c_uint = 0x0400  /* DMICDAT2_PD */;

pub const WM8994_DMICDAT1_PU: c_uint = 0x0200  /* DMICDAT1_PU */;
pub const WM8994_DMICDAT1_PU_MASK: c_uint = 0x0200  /* DMICDAT1_PU */;

pub const WM8994_DMICDAT1_PD: c_uint = 0x0100  /* DMICDAT1_PD */;
pub const WM8994_DMICDAT1_PD_MASK: c_uint = 0x0100  /* DMICDAT1_PD */;

pub const WM8994_MCLK1_PU: c_uint = 0x0080  /* MCLK1_PU */;
pub const WM8994_MCLK1_PU_MASK: c_uint = 0x0080  /* MCLK1_PU */;

pub const WM8994_MCLK1_PD: c_uint = 0x0040  /* MCLK1_PD */;
pub const WM8994_MCLK1_PD_MASK: c_uint = 0x0040  /* MCLK1_PD */;

pub const WM8994_DACDAT1_PU: c_uint = 0x0020  /* DACDAT1_PU */;
pub const WM8994_DACDAT1_PU_MASK: c_uint = 0x0020  /* DACDAT1_PU */;

pub const WM8994_DACDAT1_PD: c_uint = 0x0010  /* DACDAT1_PD */;
pub const WM8994_DACDAT1_PD_MASK: c_uint = 0x0010  /* DACDAT1_PD */;

pub const WM8994_DACLRCLK1_PU: c_uint = 0x0008  /* DACLRCLK1_PU */;
pub const WM8994_DACLRCLK1_PU_MASK: c_uint = 0x0008  /* DACLRCLK1_PU */;

pub const WM8994_DACLRCLK1_PD: c_uint = 0x0004  /* DACLRCLK1_PD */;
pub const WM8994_DACLRCLK1_PD_MASK: c_uint = 0x0004  /* DACLRCLK1_PD */;

pub const WM8994_BCLK1_PU: c_uint = 0x0002  /* BCLK1_PU */;
pub const WM8994_BCLK1_PU_MASK: c_uint = 0x0002  /* BCLK1_PU */;

pub const WM8994_BCLK1_PD: c_uint = 0x0001  /* BCLK1_PD */;
pub const WM8994_BCLK1_PD_MASK: c_uint = 0x0001  /* BCLK1_PD */;

//
// R1825 (0x721) - Pull Control (2)
//
pub const WM8994_CSNADDR_PD: c_uint = 0x0100  /* CSNADDR_PD */;
pub const WM8994_CSNADDR_PD_MASK: c_uint = 0x0100  /* CSNADDR_PD */;

pub const WM8994_LDO2ENA_PD: c_uint = 0x0040  /* LDO2ENA_PD */;
pub const WM8994_LDO2ENA_PD_MASK: c_uint = 0x0040  /* LDO2ENA_PD */;

pub const WM8994_LDO1ENA_PD: c_uint = 0x0010  /* LDO1ENA_PD */;
pub const WM8994_LDO1ENA_PD_MASK: c_uint = 0x0010  /* LDO1ENA_PD */;

pub const WM8994_CIFMODE_PD: c_uint = 0x0004  /* CIFMODE_PD */;
pub const WM8994_CIFMODE_PD_MASK: c_uint = 0x0004  /* CIFMODE_PD */;

pub const WM8994_SPKMODE_PU: c_uint = 0x0002  /* SPKMODE_PU */;
pub const WM8994_SPKMODE_PU_MASK: c_uint = 0x0002  /* SPKMODE_PU */;

//
// R1840 (0x730) - Interrupt Status 1
//
pub const WM8994_GP11_EINT: c_uint = 0x0400  /* GP11_EINT */;
pub const WM8994_GP11_EINT_MASK: c_uint = 0x0400  /* GP11_EINT */;

pub const WM8994_GP10_EINT: c_uint = 0x0200  /* GP10_EINT */;
pub const WM8994_GP10_EINT_MASK: c_uint = 0x0200  /* GP10_EINT */;

pub const WM8994_GP9_EINT: c_uint = 0x0100  /* GP9_EINT */;
pub const WM8994_GP9_EINT_MASK: c_uint = 0x0100  /* GP9_EINT */;

pub const WM8994_GP8_EINT: c_uint = 0x0080  /* GP8_EINT */;
pub const WM8994_GP8_EINT_MASK: c_uint = 0x0080  /* GP8_EINT */;

pub const WM8994_GP7_EINT: c_uint = 0x0040  /* GP7_EINT */;
pub const WM8994_GP7_EINT_MASK: c_uint = 0x0040  /* GP7_EINT */;

pub const WM8994_GP6_EINT: c_uint = 0x0020  /* GP6_EINT */;
pub const WM8994_GP6_EINT_MASK: c_uint = 0x0020  /* GP6_EINT */;

pub const WM8994_GP5_EINT: c_uint = 0x0010  /* GP5_EINT */;
pub const WM8994_GP5_EINT_MASK: c_uint = 0x0010  /* GP5_EINT */;

pub const WM8994_GP4_EINT: c_uint = 0x0008  /* GP4_EINT */;
pub const WM8994_GP4_EINT_MASK: c_uint = 0x0008  /* GP4_EINT */;

pub const WM8994_GP3_EINT: c_uint = 0x0004  /* GP3_EINT */;
pub const WM8994_GP3_EINT_MASK: c_uint = 0x0004  /* GP3_EINT */;

pub const WM8994_GP2_EINT: c_uint = 0x0002  /* GP2_EINT */;
pub const WM8994_GP2_EINT_MASK: c_uint = 0x0002  /* GP2_EINT */;

pub const WM8994_GP1_EINT: c_uint = 0x0001  /* GP1_EINT */;
pub const WM8994_GP1_EINT_MASK: c_uint = 0x0001  /* GP1_EINT */;

//
// R1841 (0x731) - Interrupt Status 2
//
pub const WM8994_TEMP_WARN_EINT: c_uint = 0x8000  /* TEMP_WARN_EINT */;
pub const WM8994_TEMP_WARN_EINT_MASK: c_uint = 0x8000  /* TEMP_WARN_EINT */;

pub const WM8994_DCS_DONE_EINT: c_uint = 0x4000  /* DCS_DONE_EINT */;
pub const WM8994_DCS_DONE_EINT_MASK: c_uint = 0x4000  /* DCS_DONE_EINT */;

pub const WM8994_WSEQ_DONE_EINT: c_uint = 0x2000  /* WSEQ_DONE_EINT */;
pub const WM8994_WSEQ_DONE_EINT_MASK: c_uint = 0x2000  /* WSEQ_DONE_EINT */;

pub const WM8994_FIFOS_ERR_EINT: c_uint = 0x1000  /* FIFOS_ERR_EINT */;
pub const WM8994_FIFOS_ERR_EINT_MASK: c_uint = 0x1000  /* FIFOS_ERR_EINT */;

pub const WM8994_AIF2DRC_SIG_DET_EINT: c_uint = 0x0800  /* AIF2DRC_SIG_DET_EINT */;
pub const WM8994_AIF2DRC_SIG_DET_EINT_MASK: c_uint = 0x0800  /* AIF2DRC_SIG_DET_EINT */;

pub const WM8994_AIF1DRC2_SIG_DET_EINT: c_uint = 0x0400  /* AIF1DRC2_SIG_DET_EINT */;
pub const WM8994_AIF1DRC2_SIG_DET_EINT_MASK: c_uint = 0x0400  /* AIF1DRC2_SIG_DET_EINT */;

pub const WM8994_AIF1DRC1_SIG_DET_EINT: c_uint = 0x0200  /* AIF1DRC1_SIG_DET_EINT */;
pub const WM8994_AIF1DRC1_SIG_DET_EINT_MASK: c_uint = 0x0200  /* AIF1DRC1_SIG_DET_EINT */;

pub const WM8994_SRC2_LOCK_EINT: c_uint = 0x0100  /* SRC2_LOCK_EINT */;
pub const WM8994_SRC2_LOCK_EINT_MASK: c_uint = 0x0100  /* SRC2_LOCK_EINT */;

pub const WM8994_SRC1_LOCK_EINT: c_uint = 0x0080  /* SRC1_LOCK_EINT */;
pub const WM8994_SRC1_LOCK_EINT_MASK: c_uint = 0x0080  /* SRC1_LOCK_EINT */;

pub const WM8994_FLL2_LOCK_EINT: c_uint = 0x0040  /* FLL2_LOCK_EINT */;
pub const WM8994_FLL2_LOCK_EINT_MASK: c_uint = 0x0040  /* FLL2_LOCK_EINT */;

pub const WM8994_FLL1_LOCK_EINT: c_uint = 0x0020  /* FLL1_LOCK_EINT */;
pub const WM8994_FLL1_LOCK_EINT_MASK: c_uint = 0x0020  /* FLL1_LOCK_EINT */;

pub const WM8994_MIC2_SHRT_EINT: c_uint = 0x0010  /* MIC2_SHRT_EINT */;
pub const WM8994_MIC2_SHRT_EINT_MASK: c_uint = 0x0010  /* MIC2_SHRT_EINT */;

pub const WM8994_MIC2_DET_EINT: c_uint = 0x0008  /* MIC2_DET_EINT */;
pub const WM8994_MIC2_DET_EINT_MASK: c_uint = 0x0008  /* MIC2_DET_EINT */;

pub const WM8994_MIC1_SHRT_EINT: c_uint = 0x0004  /* MIC1_SHRT_EINT */;
pub const WM8994_MIC1_SHRT_EINT_MASK: c_uint = 0x0004  /* MIC1_SHRT_EINT */;

pub const WM8994_MIC1_DET_EINT: c_uint = 0x0002  /* MIC1_DET_EINT */;
pub const WM8994_MIC1_DET_EINT_MASK: c_uint = 0x0002  /* MIC1_DET_EINT */;

pub const WM8994_TEMP_SHUT_EINT: c_uint = 0x0001  /* TEMP_SHUT_EINT */;
pub const WM8994_TEMP_SHUT_EINT_MASK: c_uint = 0x0001  /* TEMP_SHUT_EINT */;

//
// R1842 (0x732) - Interrupt Raw Status 2
//
pub const WM8994_TEMP_WARN_STS: c_uint = 0x8000  /* TEMP_WARN_STS */;
pub const WM8994_TEMP_WARN_STS_MASK: c_uint = 0x8000  /* TEMP_WARN_STS */;

pub const WM8994_DCS_DONE_STS: c_uint = 0x4000  /* DCS_DONE_STS */;
pub const WM8994_DCS_DONE_STS_MASK: c_uint = 0x4000  /* DCS_DONE_STS */;

pub const WM8994_WSEQ_DONE_STS: c_uint = 0x2000  /* WSEQ_DONE_STS */;
pub const WM8994_WSEQ_DONE_STS_MASK: c_uint = 0x2000  /* WSEQ_DONE_STS */;

pub const WM8994_FIFOS_ERR_STS: c_uint = 0x1000  /* FIFOS_ERR_STS */;
pub const WM8994_FIFOS_ERR_STS_MASK: c_uint = 0x1000  /* FIFOS_ERR_STS */;

pub const WM8994_AIF2DRC_SIG_DET_STS: c_uint = 0x0800  /* AIF2DRC_SIG_DET_STS */;
pub const WM8994_AIF2DRC_SIG_DET_STS_MASK: c_uint = 0x0800  /* AIF2DRC_SIG_DET_STS */;

pub const WM8994_AIF1DRC2_SIG_DET_STS: c_uint = 0x0400  /* AIF1DRC2_SIG_DET_STS */;
pub const WM8994_AIF1DRC2_SIG_DET_STS_MASK: c_uint = 0x0400  /* AIF1DRC2_SIG_DET_STS */;

pub const WM8994_AIF1DRC1_SIG_DET_STS: c_uint = 0x0200  /* AIF1DRC1_SIG_DET_STS */;
pub const WM8994_AIF1DRC1_SIG_DET_STS_MASK: c_uint = 0x0200  /* AIF1DRC1_SIG_DET_STS */;

pub const WM8994_SRC2_LOCK_STS: c_uint = 0x0100  /* SRC2_LOCK_STS */;
pub const WM8994_SRC2_LOCK_STS_MASK: c_uint = 0x0100  /* SRC2_LOCK_STS */;

pub const WM8994_SRC1_LOCK_STS: c_uint = 0x0080  /* SRC1_LOCK_STS */;
pub const WM8994_SRC1_LOCK_STS_MASK: c_uint = 0x0080  /* SRC1_LOCK_STS */;

pub const WM8994_FLL2_LOCK_STS: c_uint = 0x0040  /* FLL2_LOCK_STS */;
pub const WM8994_FLL2_LOCK_STS_MASK: c_uint = 0x0040  /* FLL2_LOCK_STS */;

pub const WM8994_FLL1_LOCK_STS: c_uint = 0x0020  /* FLL1_LOCK_STS */;
pub const WM8994_FLL1_LOCK_STS_MASK: c_uint = 0x0020  /* FLL1_LOCK_STS */;

pub const WM8994_MIC2_SHRT_STS: c_uint = 0x0010  /* MIC2_SHRT_STS */;
pub const WM8994_MIC2_SHRT_STS_MASK: c_uint = 0x0010  /* MIC2_SHRT_STS */;

pub const WM8994_MIC2_DET_STS: c_uint = 0x0008  /* MIC2_DET_STS */;
pub const WM8994_MIC2_DET_STS_MASK: c_uint = 0x0008  /* MIC2_DET_STS */;

pub const WM8994_MIC1_SHRT_STS: c_uint = 0x0004  /* MIC1_SHRT_STS */;
pub const WM8994_MIC1_SHRT_STS_MASK: c_uint = 0x0004  /* MIC1_SHRT_STS */;

pub const WM8994_MIC1_DET_STS: c_uint = 0x0002  /* MIC1_DET_STS */;
pub const WM8994_MIC1_DET_STS_MASK: c_uint = 0x0002  /* MIC1_DET_STS */;

pub const WM8994_TEMP_SHUT_STS: c_uint = 0x0001  /* TEMP_SHUT_STS */;
pub const WM8994_TEMP_SHUT_STS_MASK: c_uint = 0x0001  /* TEMP_SHUT_STS */;

//
// R1848 (0x738) - Interrupt Status 1 Mask
//
pub const WM8994_IM_GP11_EINT: c_uint = 0x0400  /* IM_GP11_EINT */;
pub const WM8994_IM_GP11_EINT_MASK: c_uint = 0x0400  /* IM_GP11_EINT */;

pub const WM8994_IM_GP10_EINT: c_uint = 0x0200  /* IM_GP10_EINT */;
pub const WM8994_IM_GP10_EINT_MASK: c_uint = 0x0200  /* IM_GP10_EINT */;

pub const WM8994_IM_GP9_EINT: c_uint = 0x0100  /* IM_GP9_EINT */;
pub const WM8994_IM_GP9_EINT_MASK: c_uint = 0x0100  /* IM_GP9_EINT */;

pub const WM8994_IM_GP8_EINT: c_uint = 0x0080  /* IM_GP8_EINT */;
pub const WM8994_IM_GP8_EINT_MASK: c_uint = 0x0080  /* IM_GP8_EINT */;

pub const WM8994_IM_GP7_EINT: c_uint = 0x0040  /* IM_GP7_EINT */;
pub const WM8994_IM_GP7_EINT_MASK: c_uint = 0x0040  /* IM_GP7_EINT */;

pub const WM8994_IM_GP6_EINT: c_uint = 0x0020  /* IM_GP6_EINT */;
pub const WM8994_IM_GP6_EINT_MASK: c_uint = 0x0020  /* IM_GP6_EINT */;

pub const WM8994_IM_GP5_EINT: c_uint = 0x0010  /* IM_GP5_EINT */;
pub const WM8994_IM_GP5_EINT_MASK: c_uint = 0x0010  /* IM_GP5_EINT */;

pub const WM8994_IM_GP4_EINT: c_uint = 0x0008  /* IM_GP4_EINT */;
pub const WM8994_IM_GP4_EINT_MASK: c_uint = 0x0008  /* IM_GP4_EINT */;

pub const WM8994_IM_GP3_EINT: c_uint = 0x0004  /* IM_GP3_EINT */;
pub const WM8994_IM_GP3_EINT_MASK: c_uint = 0x0004  /* IM_GP3_EINT */;

pub const WM8994_IM_GP2_EINT: c_uint = 0x0002  /* IM_GP2_EINT */;
pub const WM8994_IM_GP2_EINT_MASK: c_uint = 0x0002  /* IM_GP2_EINT */;

pub const WM8994_IM_GP1_EINT: c_uint = 0x0001  /* IM_GP1_EINT */;
pub const WM8994_IM_GP1_EINT_MASK: c_uint = 0x0001  /* IM_GP1_EINT */;

//
// R1849 (0x739) - Interrupt Status 2 Mask
//
pub const WM8994_IM_TEMP_WARN_EINT: c_uint = 0x8000  /* IM_TEMP_WARN_EINT */;
pub const WM8994_IM_TEMP_WARN_EINT_MASK: c_uint = 0x8000  /* IM_TEMP_WARN_EINT */;

pub const WM8994_IM_DCS_DONE_EINT: c_uint = 0x4000  /* IM_DCS_DONE_EINT */;
pub const WM8994_IM_DCS_DONE_EINT_MASK: c_uint = 0x4000  /* IM_DCS_DONE_EINT */;

pub const WM8994_IM_WSEQ_DONE_EINT: c_uint = 0x2000  /* IM_WSEQ_DONE_EINT */;
pub const WM8994_IM_WSEQ_DONE_EINT_MASK: c_uint = 0x2000  /* IM_WSEQ_DONE_EINT */;

pub const WM8994_IM_FIFOS_ERR_EINT: c_uint = 0x1000  /* IM_FIFOS_ERR_EINT */;
pub const WM8994_IM_FIFOS_ERR_EINT_MASK: c_uint = 0x1000  /* IM_FIFOS_ERR_EINT */;

pub const WM8994_IM_AIF2DRC_SIG_DET_EINT: c_uint = 0x0800  /* IM_AIF2DRC_SIG_DET_EINT */;
pub const WM8994_IM_AIF2DRC_SIG_DET_EINT_MASK: c_uint = 0x0800  /* IM_AIF2DRC_SIG_DET_EINT */;

pub const WM8994_IM_AIF1DRC2_SIG_DET_EINT: c_uint = 0x0400  /* IM_AIF1DRC2_SIG_DET_EINT */;
pub const WM8994_IM_AIF1DRC2_SIG_DET_EINT_MASK: c_uint = 0x0400  /* IM_AIF1DRC2_SIG_DET_EINT */;

pub const WM8994_IM_AIF1DRC1_SIG_DET_EINT: c_uint = 0x0200  /* IM_AIF1DRC1_SIG_DET_EINT */;
pub const WM8994_IM_AIF1DRC1_SIG_DET_EINT_MASK: c_uint = 0x0200  /* IM_AIF1DRC1_SIG_DET_EINT */;

pub const WM8994_IM_SRC2_LOCK_EINT: c_uint = 0x0100  /* IM_SRC2_LOCK_EINT */;
pub const WM8994_IM_SRC2_LOCK_EINT_MASK: c_uint = 0x0100  /* IM_SRC2_LOCK_EINT */;

pub const WM8994_IM_SRC1_LOCK_EINT: c_uint = 0x0080  /* IM_SRC1_LOCK_EINT */;
pub const WM8994_IM_SRC1_LOCK_EINT_MASK: c_uint = 0x0080  /* IM_SRC1_LOCK_EINT */;

pub const WM8994_IM_FLL2_LOCK_EINT: c_uint = 0x0040  /* IM_FLL2_LOCK_EINT */;
pub const WM8994_IM_FLL2_LOCK_EINT_MASK: c_uint = 0x0040  /* IM_FLL2_LOCK_EINT */;

pub const WM8994_IM_FLL1_LOCK_EINT: c_uint = 0x0020  /* IM_FLL1_LOCK_EINT */;
pub const WM8994_IM_FLL1_LOCK_EINT_MASK: c_uint = 0x0020  /* IM_FLL1_LOCK_EINT */;

pub const WM8994_IM_MIC2_SHRT_EINT: c_uint = 0x0010  /* IM_MIC2_SHRT_EINT */;
pub const WM8994_IM_MIC2_SHRT_EINT_MASK: c_uint = 0x0010  /* IM_MIC2_SHRT_EINT */;

pub const WM8994_IM_MIC2_DET_EINT: c_uint = 0x0008  /* IM_MIC2_DET_EINT */;
pub const WM8994_IM_MIC2_DET_EINT_MASK: c_uint = 0x0008  /* IM_MIC2_DET_EINT */;

pub const WM8994_IM_MIC1_SHRT_EINT: c_uint = 0x0004  /* IM_MIC1_SHRT_EINT */;
pub const WM8994_IM_MIC1_SHRT_EINT_MASK: c_uint = 0x0004  /* IM_MIC1_SHRT_EINT */;

pub const WM8994_IM_MIC1_DET_EINT: c_uint = 0x0002  /* IM_MIC1_DET_EINT */;
pub const WM8994_IM_MIC1_DET_EINT_MASK: c_uint = 0x0002  /* IM_MIC1_DET_EINT */;

pub const WM8994_IM_TEMP_SHUT_EINT: c_uint = 0x0001  /* IM_TEMP_SHUT_EINT */;
pub const WM8994_IM_TEMP_SHUT_EINT_MASK: c_uint = 0x0001  /* IM_TEMP_SHUT_EINT */;

//
// R1856 (0x740) - Interrupt Control
//
pub const WM8994_IM_IRQ: c_uint = 0x0001  /* IM_IRQ */;
pub const WM8994_IM_IRQ_MASK: c_uint = 0x0001  /* IM_IRQ */;

//
// R1864 (0x748) - IRQ Debounce
//
pub const WM8994_TEMP_WARN_DB: c_uint = 0x0020  /* TEMP_WARN_DB */;
pub const WM8994_TEMP_WARN_DB_MASK: c_uint = 0x0020  /* TEMP_WARN_DB */;

pub const WM8994_MIC2_SHRT_DB: c_uint = 0x0010  /* MIC2_SHRT_DB */;
pub const WM8994_MIC2_SHRT_DB_MASK: c_uint = 0x0010  /* MIC2_SHRT_DB */;

pub const WM8994_MIC2_DET_DB: c_uint = 0x0008  /* MIC2_DET_DB */;
pub const WM8994_MIC2_DET_DB_MASK: c_uint = 0x0008  /* MIC2_DET_DB */;

pub const WM8994_MIC1_SHRT_DB: c_uint = 0x0004  /* MIC1_SHRT_DB */;
pub const WM8994_MIC1_SHRT_DB_MASK: c_uint = 0x0004  /* MIC1_SHRT_DB */;

pub const WM8994_MIC1_DET_DB: c_uint = 0x0002  /* MIC1_DET_DB */;
pub const WM8994_MIC1_DET_DB_MASK: c_uint = 0x0002  /* MIC1_DET_DB */;

pub const WM8994_TEMP_SHUT_DB: c_uint = 0x0001  /* TEMP_SHUT_DB */;
pub const WM8994_TEMP_SHUT_DB_MASK: c_uint = 0x0001  /* TEMP_SHUT_DB */;

//
// R2304 (0x900) - DSP2_Program
//
pub const WM8958_DSP2_ENA: c_uint = 0x0001  /* DSP2_ENA */;
pub const WM8958_DSP2_ENA_MASK: c_uint = 0x0001  /* DSP2_ENA */;

//
// R2305 (0x901) - DSP2_Config
//
pub const WM8958_MBC_SEL_MASK: c_uint = 0x0030  /* MBC_SEL - [5:4] */;

pub const WM8958_MBC_ENA: c_uint = 0x0001  /* MBC_ENA */;
pub const WM8958_MBC_ENA_MASK: c_uint = 0x0001  /* MBC_ENA */;

//
// R2560 (0xA00) - DSP2_MagicNum
//
pub const WM8958_DSP2_MAGIC_NUM_MASK: c_uint = 0xFFFF  /* DSP2_MAGIC_NUM - [15:0] */;

//
// R2561 (0xA01) - DSP2_ReleaseYear
//
pub const WM8958_DSP2_RELEASE_YEAR_MASK: c_uint = 0xFFFF  /* DSP2_RELEASE_YEAR - [15:0] */;

//
// R2562 (0xA02) - DSP2_ReleaseMonthDay
//
pub const WM8958_DSP2_RELEASE_MONTH_MASK: c_uint = 0xFF00  /* DSP2_RELEASE_MONTH - [15:8] */;

pub const WM8958_DSP2_RELEASE_DAY_MASK: c_uint = 0x00FF  /* DSP2_RELEASE_DAY - [7:0] */;

//
// R2563 (0xA03) - DSP2_ReleaseTime
//
pub const WM8958_DSP2_RELEASE_HOURS_MASK: c_uint = 0xFF00  /* DSP2_RELEASE_HOURS - [15:8] */;

pub const WM8958_DSP2_RELEASE_MINS_MASK: c_uint = 0x00FF  /* DSP2_RELEASE_MINS - [7:0] */;

//
// R2564 (0xA04) - DSP2_VerMajMin
//
pub const WM8958_DSP2_MAJOR_VER_MASK: c_uint = 0xFF00  /* DSP2_MAJOR_VER - [15:8] */;

pub const WM8958_DSP2_MINOR_VER_MASK: c_uint = 0x00FF  /* DSP2_MINOR_VER - [7:0] */;

//
// R2565 (0xA05) - DSP2_VerBuild
//
pub const WM8958_DSP2_BUILD_VER_MASK: c_uint = 0xFFFF  /* DSP2_BUILD_VER - [15:0] */;

//
// R2573 (0xA0D) - DSP2_ExecControl
//
pub const WM8958_DSP2_STOPC: c_uint = 0x0020  /* DSP2_STOPC */;
pub const WM8958_DSP2_STOPC_MASK: c_uint = 0x0020  /* DSP2_STOPC */;

pub const WM8958_DSP2_STOPS: c_uint = 0x0010  /* DSP2_STOPS */;
pub const WM8958_DSP2_STOPS_MASK: c_uint = 0x0010  /* DSP2_STOPS */;

pub const WM8958_DSP2_STOPI: c_uint = 0x0008  /* DSP2_STOPI */;
pub const WM8958_DSP2_STOPI_MASK: c_uint = 0x0008  /* DSP2_STOPI */;

pub const WM8958_DSP2_STOP: c_uint = 0x0004  /* DSP2_STOP */;
pub const WM8958_DSP2_STOP_MASK: c_uint = 0x0004  /* DSP2_STOP */;

pub const WM8958_DSP2_RUNR: c_uint = 0x0002  /* DSP2_RUNR */;
pub const WM8958_DSP2_RUNR_MASK: c_uint = 0x0002  /* DSP2_RUNR */;

pub const WM8958_DSP2_RUN: c_uint = 0x0001  /* DSP2_RUN */;
pub const WM8958_DSP2_RUN_MASK: c_uint = 0x0001  /* DSP2_RUN */;

