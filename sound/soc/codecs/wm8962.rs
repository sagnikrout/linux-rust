//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8962.h
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
// wm8962.h  --  WM8962 ASoC driver
//
// Copyright 2010 Wolfson Microelectronics, plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

pub const WM8962_SYSCLK_MCLK: c_int = 0;
pub const WM8962_SYSCLK_FLL: c_int = 1;
pub const WM8962_SYSCLK_PLL3: c_int = 2;
pub const WM8962_FLL: c_int = 1;
pub const WM8962_FLL_MCLK: c_int = 1;
pub const WM8962_FLL_BCLK: c_int = 2;
pub const WM8962_FLL_OSC: c_int = 3;
pub const WM8962_FLL_INT: c_int = 4;
//
// Register values.
//
pub const WM8962_LEFT_INPUT_VOLUME: c_uint = 0x00;
pub const WM8962_RIGHT_INPUT_VOLUME: c_uint = 0x01;
pub const WM8962_HPOUTL_VOLUME: c_uint = 0x02;
pub const WM8962_HPOUTR_VOLUME: c_uint = 0x03;
pub const WM8962_CLOCKING1: c_uint = 0x04;
pub const WM8962_ADC_DAC_CONTROL_1: c_uint = 0x05;
pub const WM8962_ADC_DAC_CONTROL_2: c_uint = 0x06;
pub const WM8962_AUDIO_INTERFACE_0: c_uint = 0x07;
pub const WM8962_CLOCKING2: c_uint = 0x08;
pub const WM8962_AUDIO_INTERFACE_1: c_uint = 0x09;
pub const WM8962_LEFT_DAC_VOLUME: c_uint = 0x0A;
pub const WM8962_RIGHT_DAC_VOLUME: c_uint = 0x0B;
pub const WM8962_AUDIO_INTERFACE_2: c_uint = 0x0E;
pub const WM8962_SOFTWARE_RESET: c_uint = 0x0F;
pub const WM8962_ALC1: c_uint = 0x11;
pub const WM8962_ALC2: c_uint = 0x12;
pub const WM8962_ALC3: c_uint = 0x13;
pub const WM8962_NOISE_GATE: c_uint = 0x14;
pub const WM8962_LEFT_ADC_VOLUME: c_uint = 0x15;
pub const WM8962_RIGHT_ADC_VOLUME: c_uint = 0x16;
pub const WM8962_ADDITIONAL_CONTROL_1: c_uint = 0x17;
pub const WM8962_ADDITIONAL_CONTROL_2: c_uint = 0x18;
pub const WM8962_PWR_MGMT_1: c_uint = 0x19;
pub const WM8962_PWR_MGMT_2: c_uint = 0x1A;
pub const WM8962_ADDITIONAL_CONTROL_3: c_uint = 0x1B;
pub const WM8962_ANTI_POP: c_uint = 0x1C;
pub const WM8962_CLOCKING_3: c_uint = 0x1E;
pub const WM8962_INPUT_MIXER_CONTROL_1: c_uint = 0x1F;
pub const WM8962_LEFT_INPUT_MIXER_VOLUME: c_uint = 0x20;
pub const WM8962_RIGHT_INPUT_MIXER_VOLUME: c_uint = 0x21;
pub const WM8962_INPUT_MIXER_CONTROL_2: c_uint = 0x22;
pub const WM8962_INPUT_BIAS_CONTROL: c_uint = 0x23;
pub const WM8962_LEFT_INPUT_PGA_CONTROL: c_uint = 0x25;
pub const WM8962_RIGHT_INPUT_PGA_CONTROL: c_uint = 0x26;
pub const WM8962_SPKOUTL_VOLUME: c_uint = 0x28;
pub const WM8962_SPKOUTR_VOLUME: c_uint = 0x29;
pub const WM8962_THERMAL_SHUTDOWN_STATUS: c_uint = 0x2F;
pub const WM8962_ADDITIONAL_CONTROL_4: c_uint = 0x30;
pub const WM8962_CLASS_D_CONTROL_1: c_uint = 0x31;
pub const WM8962_CLASS_D_CONTROL_2: c_uint = 0x33;
pub const WM8962_CLOCKING_4: c_uint = 0x38;
pub const WM8962_DAC_DSP_MIXING_1: c_uint = 0x39;
pub const WM8962_DAC_DSP_MIXING_2: c_uint = 0x3A;
pub const WM8962_DC_SERVO_0: c_uint = 0x3C;
pub const WM8962_DC_SERVO_1: c_uint = 0x3D;
pub const WM8962_DC_SERVO_4: c_uint = 0x40;
pub const WM8962_DC_SERVO_6: c_uint = 0x42;
pub const WM8962_ANALOGUE_PGA_BIAS: c_uint = 0x44;
pub const WM8962_ANALOGUE_HP_0: c_uint = 0x45;
pub const WM8962_ANALOGUE_HP_2: c_uint = 0x47;
pub const WM8962_CHARGE_PUMP_1: c_uint = 0x48;
pub const WM8962_CHARGE_PUMP_B: c_uint = 0x52;
pub const WM8962_WRITE_SEQUENCER_CONTROL_1: c_uint = 0x57;
pub const WM8962_WRITE_SEQUENCER_CONTROL_2: c_uint = 0x5A;
pub const WM8962_WRITE_SEQUENCER_CONTROL_3: c_uint = 0x5D;
pub const WM8962_CONTROL_INTERFACE: c_uint = 0x5E;
pub const WM8962_MIXER_ENABLES: c_uint = 0x63;
pub const WM8962_HEADPHONE_MIXER_1: c_uint = 0x64;
pub const WM8962_HEADPHONE_MIXER_2: c_uint = 0x65;
pub const WM8962_HEADPHONE_MIXER_3: c_uint = 0x66;
pub const WM8962_HEADPHONE_MIXER_4: c_uint = 0x67;
pub const WM8962_SPEAKER_MIXER_1: c_uint = 0x69;
pub const WM8962_SPEAKER_MIXER_2: c_uint = 0x6A;
pub const WM8962_SPEAKER_MIXER_3: c_uint = 0x6B;
pub const WM8962_SPEAKER_MIXER_4: c_uint = 0x6C;
pub const WM8962_SPEAKER_MIXER_5: c_uint = 0x6D;
pub const WM8962_BEEP_GENERATOR_1: c_uint = 0x6E;
pub const WM8962_OSCILLATOR_TRIM_3: c_uint = 0x73;
pub const WM8962_OSCILLATOR_TRIM_4: c_uint = 0x74;
pub const WM8962_OSCILLATOR_TRIM_7: c_uint = 0x77;
pub const WM8962_ANALOGUE_CLOCKING1: c_uint = 0x7C;
pub const WM8962_ANALOGUE_CLOCKING2: c_uint = 0x7D;
pub const WM8962_ANALOGUE_CLOCKING3: c_uint = 0x7E;
pub const WM8962_PLL_SOFTWARE_RESET: c_uint = 0x7F;
pub const WM8962_PLL2: c_uint = 0x81;
pub const WM8962_PLL_4: c_uint = 0x83;
pub const WM8962_PLL_9: c_uint = 0x88;
pub const WM8962_PLL_10: c_uint = 0x89;
pub const WM8962_PLL_11: c_uint = 0x8A;
pub const WM8962_PLL_12: c_uint = 0x8B;
pub const WM8962_PLL_13: c_uint = 0x8C;
pub const WM8962_PLL_14: c_uint = 0x8D;
pub const WM8962_PLL_15: c_uint = 0x8E;
pub const WM8962_PLL_16: c_uint = 0x8F;
pub const WM8962_FLL_CONTROL_1: c_uint = 0x9B;
pub const WM8962_FLL_CONTROL_2: c_uint = 0x9C;
pub const WM8962_FLL_CONTROL_3: c_uint = 0x9D;
pub const WM8962_FLL_CONTROL_5: c_uint = 0x9F;
pub const WM8962_FLL_CONTROL_6: c_uint = 0xA0;
pub const WM8962_FLL_CONTROL_7: c_uint = 0xA1;
pub const WM8962_FLL_CONTROL_8: c_uint = 0xA2;
pub const WM8962_GENERAL_TEST_1: c_uint = 0xFC;
pub const WM8962_DF1: c_uint = 0x100;
pub const WM8962_DF2: c_uint = 0x101;
pub const WM8962_DF3: c_uint = 0x102;
pub const WM8962_DF4: c_uint = 0x103;
pub const WM8962_DF5: c_uint = 0x104;
pub const WM8962_DF6: c_uint = 0x105;
pub const WM8962_DF7: c_uint = 0x106;
pub const WM8962_LHPF1: c_uint = 0x108;
pub const WM8962_LHPF2: c_uint = 0x109;
pub const WM8962_THREED1: c_uint = 0x10C;
pub const WM8962_THREED2: c_uint = 0x10D;
pub const WM8962_THREED3: c_uint = 0x10E;
pub const WM8962_THREED4: c_uint = 0x10F;
pub const WM8962_DRC_1: c_uint = 0x114;
pub const WM8962_DRC_2: c_uint = 0x115;
pub const WM8962_DRC_3: c_uint = 0x116;
pub const WM8962_DRC_4: c_uint = 0x117;
pub const WM8962_DRC_5: c_uint = 0x118;
pub const WM8962_TLOOPBACK: c_uint = 0x11D;
pub const WM8962_EQ1: c_uint = 0x14F;
pub const WM8962_EQ2: c_uint = 0x150;
pub const WM8962_EQ3: c_uint = 0x151;
pub const WM8962_EQ4: c_uint = 0x152;
pub const WM8962_EQ5: c_uint = 0x153;
pub const WM8962_EQ6: c_uint = 0x154;
pub const WM8962_EQ7: c_uint = 0x155;
pub const WM8962_EQ8: c_uint = 0x156;
pub const WM8962_EQ9: c_uint = 0x157;
pub const WM8962_EQ10: c_uint = 0x158;
pub const WM8962_EQ11: c_uint = 0x159;
pub const WM8962_EQ12: c_uint = 0x15A;
pub const WM8962_EQ13: c_uint = 0x15B;
pub const WM8962_EQ14: c_uint = 0x15C;
pub const WM8962_EQ15: c_uint = 0x15D;
pub const WM8962_EQ16: c_uint = 0x15E;
pub const WM8962_EQ17: c_uint = 0x15F;
pub const WM8962_EQ18: c_uint = 0x160;
pub const WM8962_EQ19: c_uint = 0x161;
pub const WM8962_EQ20: c_uint = 0x162;
pub const WM8962_EQ21: c_uint = 0x163;
pub const WM8962_EQ22: c_uint = 0x164;
pub const WM8962_EQ23: c_uint = 0x165;
pub const WM8962_EQ24: c_uint = 0x166;
pub const WM8962_EQ25: c_uint = 0x167;
pub const WM8962_EQ26: c_uint = 0x168;
pub const WM8962_EQ27: c_uint = 0x169;
pub const WM8962_EQ28: c_uint = 0x16A;
pub const WM8962_EQ29: c_uint = 0x16B;
pub const WM8962_EQ30: c_uint = 0x16C;
pub const WM8962_EQ31: c_uint = 0x16D;
pub const WM8962_EQ32: c_uint = 0x16E;
pub const WM8962_EQ33: c_uint = 0x16F;
pub const WM8962_EQ34: c_uint = 0x170;
pub const WM8962_EQ35: c_uint = 0x171;
pub const WM8962_EQ36: c_uint = 0x172;
pub const WM8962_EQ37: c_uint = 0x173;
pub const WM8962_EQ38: c_uint = 0x174;
pub const WM8962_EQ39: c_uint = 0x175;
pub const WM8962_EQ40: c_uint = 0x176;
pub const WM8962_EQ41: c_uint = 0x177;
pub const WM8962_GPIO_BASE: c_uint = 0x200;
pub const WM8962_GPIO_2: c_uint = 0x201;
pub const WM8962_GPIO_3: c_uint = 0x202;
pub const WM8962_GPIO_5: c_uint = 0x204;
pub const WM8962_GPIO_6: c_uint = 0x205;
pub const WM8962_INTERRUPT_STATUS_1: c_uint = 0x230;
pub const WM8962_INTERRUPT_STATUS_2: c_uint = 0x231;
pub const WM8962_INTERRUPT_STATUS_1_MASK: c_uint = 0x238;
pub const WM8962_INTERRUPT_STATUS_2_MASK: c_uint = 0x239;
pub const WM8962_INTERRUPT_CONTROL: c_uint = 0x240;
pub const WM8962_IRQ_DEBOUNCE: c_uint = 0x248;
pub const WM8962_MICINT_SOURCE_POL: c_uint = 0x24A;
pub const WM8962_DSP2_POWER_MANAGEMENT: c_uint = 0x300;
pub const WM8962_DSP2_EXECCONTROL: c_uint = 0x40D;
pub const WM8962_WRITE_SEQUENCER_0: c_uint = 0x1000;
pub const WM8962_WRITE_SEQUENCER_1: c_uint = 0x1001;
pub const WM8962_WRITE_SEQUENCER_2: c_uint = 0x1002;
pub const WM8962_WRITE_SEQUENCER_3: c_uint = 0x1003;
pub const WM8962_WRITE_SEQUENCER_4: c_uint = 0x1004;
pub const WM8962_WRITE_SEQUENCER_5: c_uint = 0x1005;
pub const WM8962_WRITE_SEQUENCER_6: c_uint = 0x1006;
pub const WM8962_WRITE_SEQUENCER_7: c_uint = 0x1007;
pub const WM8962_WRITE_SEQUENCER_8: c_uint = 0x1008;
pub const WM8962_WRITE_SEQUENCER_9: c_uint = 0x1009;
pub const WM8962_WRITE_SEQUENCER_10: c_uint = 0x100A;
pub const WM8962_WRITE_SEQUENCER_11: c_uint = 0x100B;
pub const WM8962_WRITE_SEQUENCER_12: c_uint = 0x100C;
pub const WM8962_WRITE_SEQUENCER_13: c_uint = 0x100D;
pub const WM8962_WRITE_SEQUENCER_14: c_uint = 0x100E;
pub const WM8962_WRITE_SEQUENCER_15: c_uint = 0x100F;
pub const WM8962_WRITE_SEQUENCER_16: c_uint = 0x1010;
pub const WM8962_WRITE_SEQUENCER_17: c_uint = 0x1011;
pub const WM8962_WRITE_SEQUENCER_18: c_uint = 0x1012;
pub const WM8962_WRITE_SEQUENCER_19: c_uint = 0x1013;
pub const WM8962_WRITE_SEQUENCER_20: c_uint = 0x1014;
pub const WM8962_WRITE_SEQUENCER_21: c_uint = 0x1015;
pub const WM8962_WRITE_SEQUENCER_22: c_uint = 0x1016;
pub const WM8962_WRITE_SEQUENCER_23: c_uint = 0x1017;
pub const WM8962_WRITE_SEQUENCER_24: c_uint = 0x1018;
pub const WM8962_WRITE_SEQUENCER_25: c_uint = 0x1019;
pub const WM8962_WRITE_SEQUENCER_26: c_uint = 0x101A;
pub const WM8962_WRITE_SEQUENCER_27: c_uint = 0x101B;
pub const WM8962_WRITE_SEQUENCER_28: c_uint = 0x101C;
pub const WM8962_WRITE_SEQUENCER_29: c_uint = 0x101D;
pub const WM8962_WRITE_SEQUENCER_30: c_uint = 0x101E;
pub const WM8962_WRITE_SEQUENCER_31: c_uint = 0x101F;
pub const WM8962_WRITE_SEQUENCER_32: c_uint = 0x1020;
pub const WM8962_WRITE_SEQUENCER_33: c_uint = 0x1021;
pub const WM8962_WRITE_SEQUENCER_34: c_uint = 0x1022;
pub const WM8962_WRITE_SEQUENCER_35: c_uint = 0x1023;
pub const WM8962_WRITE_SEQUENCER_36: c_uint = 0x1024;
pub const WM8962_WRITE_SEQUENCER_37: c_uint = 0x1025;
pub const WM8962_WRITE_SEQUENCER_38: c_uint = 0x1026;
pub const WM8962_WRITE_SEQUENCER_39: c_uint = 0x1027;
pub const WM8962_WRITE_SEQUENCER_40: c_uint = 0x1028;
pub const WM8962_WRITE_SEQUENCER_41: c_uint = 0x1029;
pub const WM8962_WRITE_SEQUENCER_42: c_uint = 0x102A;
pub const WM8962_WRITE_SEQUENCER_43: c_uint = 0x102B;
pub const WM8962_WRITE_SEQUENCER_44: c_uint = 0x102C;
pub const WM8962_WRITE_SEQUENCER_45: c_uint = 0x102D;
pub const WM8962_WRITE_SEQUENCER_46: c_uint = 0x102E;
pub const WM8962_WRITE_SEQUENCER_47: c_uint = 0x102F;
pub const WM8962_WRITE_SEQUENCER_48: c_uint = 0x1030;
pub const WM8962_WRITE_SEQUENCER_49: c_uint = 0x1031;
pub const WM8962_WRITE_SEQUENCER_50: c_uint = 0x1032;
pub const WM8962_WRITE_SEQUENCER_51: c_uint = 0x1033;
pub const WM8962_WRITE_SEQUENCER_52: c_uint = 0x1034;
pub const WM8962_WRITE_SEQUENCER_53: c_uint = 0x1035;
pub const WM8962_WRITE_SEQUENCER_54: c_uint = 0x1036;
pub const WM8962_WRITE_SEQUENCER_55: c_uint = 0x1037;
pub const WM8962_WRITE_SEQUENCER_56: c_uint = 0x1038;
pub const WM8962_WRITE_SEQUENCER_57: c_uint = 0x1039;
pub const WM8962_WRITE_SEQUENCER_58: c_uint = 0x103A;
pub const WM8962_WRITE_SEQUENCER_59: c_uint = 0x103B;
pub const WM8962_WRITE_SEQUENCER_60: c_uint = 0x103C;
pub const WM8962_WRITE_SEQUENCER_61: c_uint = 0x103D;
pub const WM8962_WRITE_SEQUENCER_62: c_uint = 0x103E;
pub const WM8962_WRITE_SEQUENCER_63: c_uint = 0x103F;
pub const WM8962_WRITE_SEQUENCER_64: c_uint = 0x1040;
pub const WM8962_WRITE_SEQUENCER_65: c_uint = 0x1041;
pub const WM8962_WRITE_SEQUENCER_66: c_uint = 0x1042;
pub const WM8962_WRITE_SEQUENCER_67: c_uint = 0x1043;
pub const WM8962_WRITE_SEQUENCER_68: c_uint = 0x1044;
pub const WM8962_WRITE_SEQUENCER_69: c_uint = 0x1045;
pub const WM8962_WRITE_SEQUENCER_70: c_uint = 0x1046;
pub const WM8962_WRITE_SEQUENCER_71: c_uint = 0x1047;
pub const WM8962_WRITE_SEQUENCER_72: c_uint = 0x1048;
pub const WM8962_WRITE_SEQUENCER_73: c_uint = 0x1049;
pub const WM8962_WRITE_SEQUENCER_74: c_uint = 0x104A;
pub const WM8962_WRITE_SEQUENCER_75: c_uint = 0x104B;
pub const WM8962_WRITE_SEQUENCER_76: c_uint = 0x104C;
pub const WM8962_WRITE_SEQUENCER_77: c_uint = 0x104D;
pub const WM8962_WRITE_SEQUENCER_78: c_uint = 0x104E;
pub const WM8962_WRITE_SEQUENCER_79: c_uint = 0x104F;
pub const WM8962_WRITE_SEQUENCER_80: c_uint = 0x1050;
pub const WM8962_WRITE_SEQUENCER_81: c_uint = 0x1051;
pub const WM8962_WRITE_SEQUENCER_82: c_uint = 0x1052;
pub const WM8962_WRITE_SEQUENCER_83: c_uint = 0x1053;
pub const WM8962_WRITE_SEQUENCER_84: c_uint = 0x1054;
pub const WM8962_WRITE_SEQUENCER_85: c_uint = 0x1055;
pub const WM8962_WRITE_SEQUENCER_86: c_uint = 0x1056;
pub const WM8962_WRITE_SEQUENCER_87: c_uint = 0x1057;
pub const WM8962_WRITE_SEQUENCER_88: c_uint = 0x1058;
pub const WM8962_WRITE_SEQUENCER_89: c_uint = 0x1059;
pub const WM8962_WRITE_SEQUENCER_90: c_uint = 0x105A;
pub const WM8962_WRITE_SEQUENCER_91: c_uint = 0x105B;
pub const WM8962_WRITE_SEQUENCER_92: c_uint = 0x105C;
pub const WM8962_WRITE_SEQUENCER_93: c_uint = 0x105D;
pub const WM8962_WRITE_SEQUENCER_94: c_uint = 0x105E;
pub const WM8962_WRITE_SEQUENCER_95: c_uint = 0x105F;
pub const WM8962_WRITE_SEQUENCER_96: c_uint = 0x1060;
pub const WM8962_WRITE_SEQUENCER_97: c_uint = 0x1061;
pub const WM8962_WRITE_SEQUENCER_98: c_uint = 0x1062;
pub const WM8962_WRITE_SEQUENCER_99: c_uint = 0x1063;
pub const WM8962_WRITE_SEQUENCER_100: c_uint = 0x1064;
pub const WM8962_WRITE_SEQUENCER_101: c_uint = 0x1065;
pub const WM8962_WRITE_SEQUENCER_102: c_uint = 0x1066;
pub const WM8962_WRITE_SEQUENCER_103: c_uint = 0x1067;
pub const WM8962_WRITE_SEQUENCER_104: c_uint = 0x1068;
pub const WM8962_WRITE_SEQUENCER_105: c_uint = 0x1069;
pub const WM8962_WRITE_SEQUENCER_106: c_uint = 0x106A;
pub const WM8962_WRITE_SEQUENCER_107: c_uint = 0x106B;
pub const WM8962_WRITE_SEQUENCER_108: c_uint = 0x106C;
pub const WM8962_WRITE_SEQUENCER_109: c_uint = 0x106D;
pub const WM8962_WRITE_SEQUENCER_110: c_uint = 0x106E;
pub const WM8962_WRITE_SEQUENCER_111: c_uint = 0x106F;
pub const WM8962_WRITE_SEQUENCER_112: c_uint = 0x1070;
pub const WM8962_WRITE_SEQUENCER_113: c_uint = 0x1071;
pub const WM8962_WRITE_SEQUENCER_114: c_uint = 0x1072;
pub const WM8962_WRITE_SEQUENCER_115: c_uint = 0x1073;
pub const WM8962_WRITE_SEQUENCER_116: c_uint = 0x1074;
pub const WM8962_WRITE_SEQUENCER_117: c_uint = 0x1075;
pub const WM8962_WRITE_SEQUENCER_118: c_uint = 0x1076;
pub const WM8962_WRITE_SEQUENCER_119: c_uint = 0x1077;
pub const WM8962_WRITE_SEQUENCER_120: c_uint = 0x1078;
pub const WM8962_WRITE_SEQUENCER_121: c_uint = 0x1079;
pub const WM8962_WRITE_SEQUENCER_122: c_uint = 0x107A;
pub const WM8962_WRITE_SEQUENCER_123: c_uint = 0x107B;
pub const WM8962_WRITE_SEQUENCER_124: c_uint = 0x107C;
pub const WM8962_WRITE_SEQUENCER_125: c_uint = 0x107D;
pub const WM8962_WRITE_SEQUENCER_126: c_uint = 0x107E;
pub const WM8962_WRITE_SEQUENCER_127: c_uint = 0x107F;
pub const WM8962_WRITE_SEQUENCER_128: c_uint = 0x1080;
pub const WM8962_WRITE_SEQUENCER_129: c_uint = 0x1081;
pub const WM8962_WRITE_SEQUENCER_130: c_uint = 0x1082;
pub const WM8962_WRITE_SEQUENCER_131: c_uint = 0x1083;
pub const WM8962_WRITE_SEQUENCER_132: c_uint = 0x1084;
pub const WM8962_WRITE_SEQUENCER_133: c_uint = 0x1085;
pub const WM8962_WRITE_SEQUENCER_134: c_uint = 0x1086;
pub const WM8962_WRITE_SEQUENCER_135: c_uint = 0x1087;
pub const WM8962_WRITE_SEQUENCER_136: c_uint = 0x1088;
pub const WM8962_WRITE_SEQUENCER_137: c_uint = 0x1089;
pub const WM8962_WRITE_SEQUENCER_138: c_uint = 0x108A;
pub const WM8962_WRITE_SEQUENCER_139: c_uint = 0x108B;
pub const WM8962_WRITE_SEQUENCER_140: c_uint = 0x108C;
pub const WM8962_WRITE_SEQUENCER_141: c_uint = 0x108D;
pub const WM8962_WRITE_SEQUENCER_142: c_uint = 0x108E;
pub const WM8962_WRITE_SEQUENCER_143: c_uint = 0x108F;
pub const WM8962_WRITE_SEQUENCER_144: c_uint = 0x1090;
pub const WM8962_WRITE_SEQUENCER_145: c_uint = 0x1091;
pub const WM8962_WRITE_SEQUENCER_146: c_uint = 0x1092;
pub const WM8962_WRITE_SEQUENCER_147: c_uint = 0x1093;
pub const WM8962_WRITE_SEQUENCER_148: c_uint = 0x1094;
pub const WM8962_WRITE_SEQUENCER_149: c_uint = 0x1095;
pub const WM8962_WRITE_SEQUENCER_150: c_uint = 0x1096;
pub const WM8962_WRITE_SEQUENCER_151: c_uint = 0x1097;
pub const WM8962_WRITE_SEQUENCER_152: c_uint = 0x1098;
pub const WM8962_WRITE_SEQUENCER_153: c_uint = 0x1099;
pub const WM8962_WRITE_SEQUENCER_154: c_uint = 0x109A;
pub const WM8962_WRITE_SEQUENCER_155: c_uint = 0x109B;
pub const WM8962_WRITE_SEQUENCER_156: c_uint = 0x109C;
pub const WM8962_WRITE_SEQUENCER_157: c_uint = 0x109D;
pub const WM8962_WRITE_SEQUENCER_158: c_uint = 0x109E;
pub const WM8962_WRITE_SEQUENCER_159: c_uint = 0x109F;
pub const WM8962_WRITE_SEQUENCER_160: c_uint = 0x10A0;
pub const WM8962_WRITE_SEQUENCER_161: c_uint = 0x10A1;
pub const WM8962_WRITE_SEQUENCER_162: c_uint = 0x10A2;
pub const WM8962_WRITE_SEQUENCER_163: c_uint = 0x10A3;
pub const WM8962_WRITE_SEQUENCER_164: c_uint = 0x10A4;
pub const WM8962_WRITE_SEQUENCER_165: c_uint = 0x10A5;
pub const WM8962_WRITE_SEQUENCER_166: c_uint = 0x10A6;
pub const WM8962_WRITE_SEQUENCER_167: c_uint = 0x10A7;
pub const WM8962_WRITE_SEQUENCER_168: c_uint = 0x10A8;
pub const WM8962_WRITE_SEQUENCER_169: c_uint = 0x10A9;
pub const WM8962_WRITE_SEQUENCER_170: c_uint = 0x10AA;
pub const WM8962_WRITE_SEQUENCER_171: c_uint = 0x10AB;
pub const WM8962_WRITE_SEQUENCER_172: c_uint = 0x10AC;
pub const WM8962_WRITE_SEQUENCER_173: c_uint = 0x10AD;
pub const WM8962_WRITE_SEQUENCER_174: c_uint = 0x10AE;
pub const WM8962_WRITE_SEQUENCER_175: c_uint = 0x10AF;
pub const WM8962_WRITE_SEQUENCER_176: c_uint = 0x10B0;
pub const WM8962_WRITE_SEQUENCER_177: c_uint = 0x10B1;
pub const WM8962_WRITE_SEQUENCER_178: c_uint = 0x10B2;
pub const WM8962_WRITE_SEQUENCER_179: c_uint = 0x10B3;
pub const WM8962_WRITE_SEQUENCER_180: c_uint = 0x10B4;
pub const WM8962_WRITE_SEQUENCER_181: c_uint = 0x10B5;
pub const WM8962_WRITE_SEQUENCER_182: c_uint = 0x10B6;
pub const WM8962_WRITE_SEQUENCER_183: c_uint = 0x10B7;
pub const WM8962_WRITE_SEQUENCER_184: c_uint = 0x10B8;
pub const WM8962_WRITE_SEQUENCER_185: c_uint = 0x10B9;
pub const WM8962_WRITE_SEQUENCER_186: c_uint = 0x10BA;
pub const WM8962_WRITE_SEQUENCER_187: c_uint = 0x10BB;
pub const WM8962_WRITE_SEQUENCER_188: c_uint = 0x10BC;
pub const WM8962_WRITE_SEQUENCER_189: c_uint = 0x10BD;
pub const WM8962_WRITE_SEQUENCER_190: c_uint = 0x10BE;
pub const WM8962_WRITE_SEQUENCER_191: c_uint = 0x10BF;
pub const WM8962_WRITE_SEQUENCER_192: c_uint = 0x10C0;
pub const WM8962_WRITE_SEQUENCER_193: c_uint = 0x10C1;
pub const WM8962_WRITE_SEQUENCER_194: c_uint = 0x10C2;
pub const WM8962_WRITE_SEQUENCER_195: c_uint = 0x10C3;
pub const WM8962_WRITE_SEQUENCER_196: c_uint = 0x10C4;
pub const WM8962_WRITE_SEQUENCER_197: c_uint = 0x10C5;
pub const WM8962_WRITE_SEQUENCER_198: c_uint = 0x10C6;
pub const WM8962_WRITE_SEQUENCER_199: c_uint = 0x10C7;
pub const WM8962_WRITE_SEQUENCER_200: c_uint = 0x10C8;
pub const WM8962_WRITE_SEQUENCER_201: c_uint = 0x10C9;
pub const WM8962_WRITE_SEQUENCER_202: c_uint = 0x10CA;
pub const WM8962_WRITE_SEQUENCER_203: c_uint = 0x10CB;
pub const WM8962_WRITE_SEQUENCER_204: c_uint = 0x10CC;
pub const WM8962_WRITE_SEQUENCER_205: c_uint = 0x10CD;
pub const WM8962_WRITE_SEQUENCER_206: c_uint = 0x10CE;
pub const WM8962_WRITE_SEQUENCER_207: c_uint = 0x10CF;
pub const WM8962_WRITE_SEQUENCER_208: c_uint = 0x10D0;
pub const WM8962_WRITE_SEQUENCER_209: c_uint = 0x10D1;
pub const WM8962_WRITE_SEQUENCER_210: c_uint = 0x10D2;
pub const WM8962_WRITE_SEQUENCER_211: c_uint = 0x10D3;
pub const WM8962_WRITE_SEQUENCER_212: c_uint = 0x10D4;
pub const WM8962_WRITE_SEQUENCER_213: c_uint = 0x10D5;
pub const WM8962_WRITE_SEQUENCER_214: c_uint = 0x10D6;
pub const WM8962_WRITE_SEQUENCER_215: c_uint = 0x10D7;
pub const WM8962_WRITE_SEQUENCER_216: c_uint = 0x10D8;
pub const WM8962_WRITE_SEQUENCER_217: c_uint = 0x10D9;
pub const WM8962_WRITE_SEQUENCER_218: c_uint = 0x10DA;
pub const WM8962_WRITE_SEQUENCER_219: c_uint = 0x10DB;
pub const WM8962_WRITE_SEQUENCER_220: c_uint = 0x10DC;
pub const WM8962_WRITE_SEQUENCER_221: c_uint = 0x10DD;
pub const WM8962_WRITE_SEQUENCER_222: c_uint = 0x10DE;
pub const WM8962_WRITE_SEQUENCER_223: c_uint = 0x10DF;
pub const WM8962_WRITE_SEQUENCER_224: c_uint = 0x10E0;
pub const WM8962_WRITE_SEQUENCER_225: c_uint = 0x10E1;
pub const WM8962_WRITE_SEQUENCER_226: c_uint = 0x10E2;
pub const WM8962_WRITE_SEQUENCER_227: c_uint = 0x10E3;
pub const WM8962_WRITE_SEQUENCER_228: c_uint = 0x10E4;
pub const WM8962_WRITE_SEQUENCER_229: c_uint = 0x10E5;
pub const WM8962_WRITE_SEQUENCER_230: c_uint = 0x10E6;
pub const WM8962_WRITE_SEQUENCER_231: c_uint = 0x10E7;
pub const WM8962_WRITE_SEQUENCER_232: c_uint = 0x10E8;
pub const WM8962_WRITE_SEQUENCER_233: c_uint = 0x10E9;
pub const WM8962_WRITE_SEQUENCER_234: c_uint = 0x10EA;
pub const WM8962_WRITE_SEQUENCER_235: c_uint = 0x10EB;
pub const WM8962_WRITE_SEQUENCER_236: c_uint = 0x10EC;
pub const WM8962_WRITE_SEQUENCER_237: c_uint = 0x10ED;
pub const WM8962_WRITE_SEQUENCER_238: c_uint = 0x10EE;
pub const WM8962_WRITE_SEQUENCER_239: c_uint = 0x10EF;
pub const WM8962_WRITE_SEQUENCER_240: c_uint = 0x10F0;
pub const WM8962_WRITE_SEQUENCER_241: c_uint = 0x10F1;
pub const WM8962_WRITE_SEQUENCER_242: c_uint = 0x10F2;
pub const WM8962_WRITE_SEQUENCER_243: c_uint = 0x10F3;
pub const WM8962_WRITE_SEQUENCER_244: c_uint = 0x10F4;
pub const WM8962_WRITE_SEQUENCER_245: c_uint = 0x10F5;
pub const WM8962_WRITE_SEQUENCER_246: c_uint = 0x10F6;
pub const WM8962_WRITE_SEQUENCER_247: c_uint = 0x10F7;
pub const WM8962_WRITE_SEQUENCER_248: c_uint = 0x10F8;
pub const WM8962_WRITE_SEQUENCER_249: c_uint = 0x10F9;
pub const WM8962_WRITE_SEQUENCER_250: c_uint = 0x10FA;
pub const WM8962_WRITE_SEQUENCER_251: c_uint = 0x10FB;
pub const WM8962_WRITE_SEQUENCER_252: c_uint = 0x10FC;
pub const WM8962_WRITE_SEQUENCER_253: c_uint = 0x10FD;
pub const WM8962_WRITE_SEQUENCER_254: c_uint = 0x10FE;
pub const WM8962_WRITE_SEQUENCER_255: c_uint = 0x10FF;
pub const WM8962_WRITE_SEQUENCER_256: c_uint = 0x1100;
pub const WM8962_WRITE_SEQUENCER_257: c_uint = 0x1101;
pub const WM8962_WRITE_SEQUENCER_258: c_uint = 0x1102;
pub const WM8962_WRITE_SEQUENCER_259: c_uint = 0x1103;
pub const WM8962_WRITE_SEQUENCER_260: c_uint = 0x1104;
pub const WM8962_WRITE_SEQUENCER_261: c_uint = 0x1105;
pub const WM8962_WRITE_SEQUENCER_262: c_uint = 0x1106;
pub const WM8962_WRITE_SEQUENCER_263: c_uint = 0x1107;
pub const WM8962_WRITE_SEQUENCER_264: c_uint = 0x1108;
pub const WM8962_WRITE_SEQUENCER_265: c_uint = 0x1109;
pub const WM8962_WRITE_SEQUENCER_266: c_uint = 0x110A;
pub const WM8962_WRITE_SEQUENCER_267: c_uint = 0x110B;
pub const WM8962_WRITE_SEQUENCER_268: c_uint = 0x110C;
pub const WM8962_WRITE_SEQUENCER_269: c_uint = 0x110D;
pub const WM8962_WRITE_SEQUENCER_270: c_uint = 0x110E;
pub const WM8962_WRITE_SEQUENCER_271: c_uint = 0x110F;
pub const WM8962_WRITE_SEQUENCER_272: c_uint = 0x1110;
pub const WM8962_WRITE_SEQUENCER_273: c_uint = 0x1111;
pub const WM8962_WRITE_SEQUENCER_274: c_uint = 0x1112;
pub const WM8962_WRITE_SEQUENCER_275: c_uint = 0x1113;
pub const WM8962_WRITE_SEQUENCER_276: c_uint = 0x1114;
pub const WM8962_WRITE_SEQUENCER_277: c_uint = 0x1115;
pub const WM8962_WRITE_SEQUENCER_278: c_uint = 0x1116;
pub const WM8962_WRITE_SEQUENCER_279: c_uint = 0x1117;
pub const WM8962_WRITE_SEQUENCER_280: c_uint = 0x1118;
pub const WM8962_WRITE_SEQUENCER_281: c_uint = 0x1119;
pub const WM8962_WRITE_SEQUENCER_282: c_uint = 0x111A;
pub const WM8962_WRITE_SEQUENCER_283: c_uint = 0x111B;
pub const WM8962_WRITE_SEQUENCER_284: c_uint = 0x111C;
pub const WM8962_WRITE_SEQUENCER_285: c_uint = 0x111D;
pub const WM8962_WRITE_SEQUENCER_286: c_uint = 0x111E;
pub const WM8962_WRITE_SEQUENCER_287: c_uint = 0x111F;
pub const WM8962_WRITE_SEQUENCER_288: c_uint = 0x1120;
pub const WM8962_WRITE_SEQUENCER_289: c_uint = 0x1121;
pub const WM8962_WRITE_SEQUENCER_290: c_uint = 0x1122;
pub const WM8962_WRITE_SEQUENCER_291: c_uint = 0x1123;
pub const WM8962_WRITE_SEQUENCER_292: c_uint = 0x1124;
pub const WM8962_WRITE_SEQUENCER_293: c_uint = 0x1125;
pub const WM8962_WRITE_SEQUENCER_294: c_uint = 0x1126;
pub const WM8962_WRITE_SEQUENCER_295: c_uint = 0x1127;
pub const WM8962_WRITE_SEQUENCER_296: c_uint = 0x1128;
pub const WM8962_WRITE_SEQUENCER_297: c_uint = 0x1129;
pub const WM8962_WRITE_SEQUENCER_298: c_uint = 0x112A;
pub const WM8962_WRITE_SEQUENCER_299: c_uint = 0x112B;
pub const WM8962_WRITE_SEQUENCER_300: c_uint = 0x112C;
pub const WM8962_WRITE_SEQUENCER_301: c_uint = 0x112D;
pub const WM8962_WRITE_SEQUENCER_302: c_uint = 0x112E;
pub const WM8962_WRITE_SEQUENCER_303: c_uint = 0x112F;
pub const WM8962_WRITE_SEQUENCER_304: c_uint = 0x1130;
pub const WM8962_WRITE_SEQUENCER_305: c_uint = 0x1131;
pub const WM8962_WRITE_SEQUENCER_306: c_uint = 0x1132;
pub const WM8962_WRITE_SEQUENCER_307: c_uint = 0x1133;
pub const WM8962_WRITE_SEQUENCER_308: c_uint = 0x1134;
pub const WM8962_WRITE_SEQUENCER_309: c_uint = 0x1135;
pub const WM8962_WRITE_SEQUENCER_310: c_uint = 0x1136;
pub const WM8962_WRITE_SEQUENCER_311: c_uint = 0x1137;
pub const WM8962_WRITE_SEQUENCER_312: c_uint = 0x1138;
pub const WM8962_WRITE_SEQUENCER_313: c_uint = 0x1139;
pub const WM8962_WRITE_SEQUENCER_314: c_uint = 0x113A;
pub const WM8962_WRITE_SEQUENCER_315: c_uint = 0x113B;
pub const WM8962_WRITE_SEQUENCER_316: c_uint = 0x113C;
pub const WM8962_WRITE_SEQUENCER_317: c_uint = 0x113D;
pub const WM8962_WRITE_SEQUENCER_318: c_uint = 0x113E;
pub const WM8962_WRITE_SEQUENCER_319: c_uint = 0x113F;
pub const WM8962_WRITE_SEQUENCER_320: c_uint = 0x1140;
pub const WM8962_WRITE_SEQUENCER_321: c_uint = 0x1141;
pub const WM8962_WRITE_SEQUENCER_322: c_uint = 0x1142;
pub const WM8962_WRITE_SEQUENCER_323: c_uint = 0x1143;
pub const WM8962_WRITE_SEQUENCER_324: c_uint = 0x1144;
pub const WM8962_WRITE_SEQUENCER_325: c_uint = 0x1145;
pub const WM8962_WRITE_SEQUENCER_326: c_uint = 0x1146;
pub const WM8962_WRITE_SEQUENCER_327: c_uint = 0x1147;
pub const WM8962_WRITE_SEQUENCER_328: c_uint = 0x1148;
pub const WM8962_WRITE_SEQUENCER_329: c_uint = 0x1149;
pub const WM8962_WRITE_SEQUENCER_330: c_uint = 0x114A;
pub const WM8962_WRITE_SEQUENCER_331: c_uint = 0x114B;
pub const WM8962_WRITE_SEQUENCER_332: c_uint = 0x114C;
pub const WM8962_WRITE_SEQUENCER_333: c_uint = 0x114D;
pub const WM8962_WRITE_SEQUENCER_334: c_uint = 0x114E;
pub const WM8962_WRITE_SEQUENCER_335: c_uint = 0x114F;
pub const WM8962_WRITE_SEQUENCER_336: c_uint = 0x1150;
pub const WM8962_WRITE_SEQUENCER_337: c_uint = 0x1151;
pub const WM8962_WRITE_SEQUENCER_338: c_uint = 0x1152;
pub const WM8962_WRITE_SEQUENCER_339: c_uint = 0x1153;
pub const WM8962_WRITE_SEQUENCER_340: c_uint = 0x1154;
pub const WM8962_WRITE_SEQUENCER_341: c_uint = 0x1155;
pub const WM8962_WRITE_SEQUENCER_342: c_uint = 0x1156;
pub const WM8962_WRITE_SEQUENCER_343: c_uint = 0x1157;
pub const WM8962_WRITE_SEQUENCER_344: c_uint = 0x1158;
pub const WM8962_WRITE_SEQUENCER_345: c_uint = 0x1159;
pub const WM8962_WRITE_SEQUENCER_346: c_uint = 0x115A;
pub const WM8962_WRITE_SEQUENCER_347: c_uint = 0x115B;
pub const WM8962_WRITE_SEQUENCER_348: c_uint = 0x115C;
pub const WM8962_WRITE_SEQUENCER_349: c_uint = 0x115D;
pub const WM8962_WRITE_SEQUENCER_350: c_uint = 0x115E;
pub const WM8962_WRITE_SEQUENCER_351: c_uint = 0x115F;
pub const WM8962_WRITE_SEQUENCER_352: c_uint = 0x1160;
pub const WM8962_WRITE_SEQUENCER_353: c_uint = 0x1161;
pub const WM8962_WRITE_SEQUENCER_354: c_uint = 0x1162;
pub const WM8962_WRITE_SEQUENCER_355: c_uint = 0x1163;
pub const WM8962_WRITE_SEQUENCER_356: c_uint = 0x1164;
pub const WM8962_WRITE_SEQUENCER_357: c_uint = 0x1165;
pub const WM8962_WRITE_SEQUENCER_358: c_uint = 0x1166;
pub const WM8962_WRITE_SEQUENCER_359: c_uint = 0x1167;
pub const WM8962_WRITE_SEQUENCER_360: c_uint = 0x1168;
pub const WM8962_WRITE_SEQUENCER_361: c_uint = 0x1169;
pub const WM8962_WRITE_SEQUENCER_362: c_uint = 0x116A;
pub const WM8962_WRITE_SEQUENCER_363: c_uint = 0x116B;
pub const WM8962_WRITE_SEQUENCER_364: c_uint = 0x116C;
pub const WM8962_WRITE_SEQUENCER_365: c_uint = 0x116D;
pub const WM8962_WRITE_SEQUENCER_366: c_uint = 0x116E;
pub const WM8962_WRITE_SEQUENCER_367: c_uint = 0x116F;
pub const WM8962_WRITE_SEQUENCER_368: c_uint = 0x1170;
pub const WM8962_WRITE_SEQUENCER_369: c_uint = 0x1171;
pub const WM8962_WRITE_SEQUENCER_370: c_uint = 0x1172;
pub const WM8962_WRITE_SEQUENCER_371: c_uint = 0x1173;
pub const WM8962_WRITE_SEQUENCER_372: c_uint = 0x1174;
pub const WM8962_WRITE_SEQUENCER_373: c_uint = 0x1175;
pub const WM8962_WRITE_SEQUENCER_374: c_uint = 0x1176;
pub const WM8962_WRITE_SEQUENCER_375: c_uint = 0x1177;
pub const WM8962_WRITE_SEQUENCER_376: c_uint = 0x1178;
pub const WM8962_WRITE_SEQUENCER_377: c_uint = 0x1179;
pub const WM8962_WRITE_SEQUENCER_378: c_uint = 0x117A;
pub const WM8962_WRITE_SEQUENCER_379: c_uint = 0x117B;
pub const WM8962_WRITE_SEQUENCER_380: c_uint = 0x117C;
pub const WM8962_WRITE_SEQUENCER_381: c_uint = 0x117D;
pub const WM8962_WRITE_SEQUENCER_382: c_uint = 0x117E;
pub const WM8962_WRITE_SEQUENCER_383: c_uint = 0x117F;
pub const WM8962_WRITE_SEQUENCER_384: c_uint = 0x1180;
pub const WM8962_WRITE_SEQUENCER_385: c_uint = 0x1181;
pub const WM8962_WRITE_SEQUENCER_386: c_uint = 0x1182;
pub const WM8962_WRITE_SEQUENCER_387: c_uint = 0x1183;
pub const WM8962_WRITE_SEQUENCER_388: c_uint = 0x1184;
pub const WM8962_WRITE_SEQUENCER_389: c_uint = 0x1185;
pub const WM8962_WRITE_SEQUENCER_390: c_uint = 0x1186;
pub const WM8962_WRITE_SEQUENCER_391: c_uint = 0x1187;
pub const WM8962_WRITE_SEQUENCER_392: c_uint = 0x1188;
pub const WM8962_WRITE_SEQUENCER_393: c_uint = 0x1189;
pub const WM8962_WRITE_SEQUENCER_394: c_uint = 0x118A;
pub const WM8962_WRITE_SEQUENCER_395: c_uint = 0x118B;
pub const WM8962_WRITE_SEQUENCER_396: c_uint = 0x118C;
pub const WM8962_WRITE_SEQUENCER_397: c_uint = 0x118D;
pub const WM8962_WRITE_SEQUENCER_398: c_uint = 0x118E;
pub const WM8962_WRITE_SEQUENCER_399: c_uint = 0x118F;
pub const WM8962_WRITE_SEQUENCER_400: c_uint = 0x1190;
pub const WM8962_WRITE_SEQUENCER_401: c_uint = 0x1191;
pub const WM8962_WRITE_SEQUENCER_402: c_uint = 0x1192;
pub const WM8962_WRITE_SEQUENCER_403: c_uint = 0x1193;
pub const WM8962_WRITE_SEQUENCER_404: c_uint = 0x1194;
pub const WM8962_WRITE_SEQUENCER_405: c_uint = 0x1195;
pub const WM8962_WRITE_SEQUENCER_406: c_uint = 0x1196;
pub const WM8962_WRITE_SEQUENCER_407: c_uint = 0x1197;
pub const WM8962_WRITE_SEQUENCER_408: c_uint = 0x1198;
pub const WM8962_WRITE_SEQUENCER_409: c_uint = 0x1199;
pub const WM8962_WRITE_SEQUENCER_410: c_uint = 0x119A;
pub const WM8962_WRITE_SEQUENCER_411: c_uint = 0x119B;
pub const WM8962_WRITE_SEQUENCER_412: c_uint = 0x119C;
pub const WM8962_WRITE_SEQUENCER_413: c_uint = 0x119D;
pub const WM8962_WRITE_SEQUENCER_414: c_uint = 0x119E;
pub const WM8962_WRITE_SEQUENCER_415: c_uint = 0x119F;
pub const WM8962_WRITE_SEQUENCER_416: c_uint = 0x11A0;
pub const WM8962_WRITE_SEQUENCER_417: c_uint = 0x11A1;
pub const WM8962_WRITE_SEQUENCER_418: c_uint = 0x11A2;
pub const WM8962_WRITE_SEQUENCER_419: c_uint = 0x11A3;
pub const WM8962_WRITE_SEQUENCER_420: c_uint = 0x11A4;
pub const WM8962_WRITE_SEQUENCER_421: c_uint = 0x11A5;
pub const WM8962_WRITE_SEQUENCER_422: c_uint = 0x11A6;
pub const WM8962_WRITE_SEQUENCER_423: c_uint = 0x11A7;
pub const WM8962_WRITE_SEQUENCER_424: c_uint = 0x11A8;
pub const WM8962_WRITE_SEQUENCER_425: c_uint = 0x11A9;
pub const WM8962_WRITE_SEQUENCER_426: c_uint = 0x11AA;
pub const WM8962_WRITE_SEQUENCER_427: c_uint = 0x11AB;
pub const WM8962_WRITE_SEQUENCER_428: c_uint = 0x11AC;
pub const WM8962_WRITE_SEQUENCER_429: c_uint = 0x11AD;
pub const WM8962_WRITE_SEQUENCER_430: c_uint = 0x11AE;
pub const WM8962_WRITE_SEQUENCER_431: c_uint = 0x11AF;
pub const WM8962_WRITE_SEQUENCER_432: c_uint = 0x11B0;
pub const WM8962_WRITE_SEQUENCER_433: c_uint = 0x11B1;
pub const WM8962_WRITE_SEQUENCER_434: c_uint = 0x11B2;
pub const WM8962_WRITE_SEQUENCER_435: c_uint = 0x11B3;
pub const WM8962_WRITE_SEQUENCER_436: c_uint = 0x11B4;
pub const WM8962_WRITE_SEQUENCER_437: c_uint = 0x11B5;
pub const WM8962_WRITE_SEQUENCER_438: c_uint = 0x11B6;
pub const WM8962_WRITE_SEQUENCER_439: c_uint = 0x11B7;
pub const WM8962_WRITE_SEQUENCER_440: c_uint = 0x11B8;
pub const WM8962_WRITE_SEQUENCER_441: c_uint = 0x11B9;
pub const WM8962_WRITE_SEQUENCER_442: c_uint = 0x11BA;
pub const WM8962_WRITE_SEQUENCER_443: c_uint = 0x11BB;
pub const WM8962_WRITE_SEQUENCER_444: c_uint = 0x11BC;
pub const WM8962_WRITE_SEQUENCER_445: c_uint = 0x11BD;
pub const WM8962_WRITE_SEQUENCER_446: c_uint = 0x11BE;
pub const WM8962_WRITE_SEQUENCER_447: c_uint = 0x11BF;
pub const WM8962_WRITE_SEQUENCER_448: c_uint = 0x11C0;
pub const WM8962_WRITE_SEQUENCER_449: c_uint = 0x11C1;
pub const WM8962_WRITE_SEQUENCER_450: c_uint = 0x11C2;
pub const WM8962_WRITE_SEQUENCER_451: c_uint = 0x11C3;
pub const WM8962_WRITE_SEQUENCER_452: c_uint = 0x11C4;
pub const WM8962_WRITE_SEQUENCER_453: c_uint = 0x11C5;
pub const WM8962_WRITE_SEQUENCER_454: c_uint = 0x11C6;
pub const WM8962_WRITE_SEQUENCER_455: c_uint = 0x11C7;
pub const WM8962_WRITE_SEQUENCER_456: c_uint = 0x11C8;
pub const WM8962_WRITE_SEQUENCER_457: c_uint = 0x11C9;
pub const WM8962_WRITE_SEQUENCER_458: c_uint = 0x11CA;
pub const WM8962_WRITE_SEQUENCER_459: c_uint = 0x11CB;
pub const WM8962_WRITE_SEQUENCER_460: c_uint = 0x11CC;
pub const WM8962_WRITE_SEQUENCER_461: c_uint = 0x11CD;
pub const WM8962_WRITE_SEQUENCER_462: c_uint = 0x11CE;
pub const WM8962_WRITE_SEQUENCER_463: c_uint = 0x11CF;
pub const WM8962_WRITE_SEQUENCER_464: c_uint = 0x11D0;
pub const WM8962_WRITE_SEQUENCER_465: c_uint = 0x11D1;
pub const WM8962_WRITE_SEQUENCER_466: c_uint = 0x11D2;
pub const WM8962_WRITE_SEQUENCER_467: c_uint = 0x11D3;
pub const WM8962_WRITE_SEQUENCER_468: c_uint = 0x11D4;
pub const WM8962_WRITE_SEQUENCER_469: c_uint = 0x11D5;
pub const WM8962_WRITE_SEQUENCER_470: c_uint = 0x11D6;
pub const WM8962_WRITE_SEQUENCER_471: c_uint = 0x11D7;
pub const WM8962_WRITE_SEQUENCER_472: c_uint = 0x11D8;
pub const WM8962_WRITE_SEQUENCER_473: c_uint = 0x11D9;
pub const WM8962_WRITE_SEQUENCER_474: c_uint = 0x11DA;
pub const WM8962_WRITE_SEQUENCER_475: c_uint = 0x11DB;
pub const WM8962_WRITE_SEQUENCER_476: c_uint = 0x11DC;
pub const WM8962_WRITE_SEQUENCER_477: c_uint = 0x11DD;
pub const WM8962_WRITE_SEQUENCER_478: c_uint = 0x11DE;
pub const WM8962_WRITE_SEQUENCER_479: c_uint = 0x11DF;
pub const WM8962_WRITE_SEQUENCER_480: c_uint = 0x11E0;
pub const WM8962_WRITE_SEQUENCER_481: c_uint = 0x11E1;
pub const WM8962_WRITE_SEQUENCER_482: c_uint = 0x11E2;
pub const WM8962_WRITE_SEQUENCER_483: c_uint = 0x11E3;
pub const WM8962_WRITE_SEQUENCER_484: c_uint = 0x11E4;
pub const WM8962_WRITE_SEQUENCER_485: c_uint = 0x11E5;
pub const WM8962_WRITE_SEQUENCER_486: c_uint = 0x11E6;
pub const WM8962_WRITE_SEQUENCER_487: c_uint = 0x11E7;
pub const WM8962_WRITE_SEQUENCER_488: c_uint = 0x11E8;
pub const WM8962_WRITE_SEQUENCER_489: c_uint = 0x11E9;
pub const WM8962_WRITE_SEQUENCER_490: c_uint = 0x11EA;
pub const WM8962_WRITE_SEQUENCER_491: c_uint = 0x11EB;
pub const WM8962_WRITE_SEQUENCER_492: c_uint = 0x11EC;
pub const WM8962_WRITE_SEQUENCER_493: c_uint = 0x11ED;
pub const WM8962_WRITE_SEQUENCER_494: c_uint = 0x11EE;
pub const WM8962_WRITE_SEQUENCER_495: c_uint = 0x11EF;
pub const WM8962_WRITE_SEQUENCER_496: c_uint = 0x11F0;
pub const WM8962_WRITE_SEQUENCER_497: c_uint = 0x11F1;
pub const WM8962_WRITE_SEQUENCER_498: c_uint = 0x11F2;
pub const WM8962_WRITE_SEQUENCER_499: c_uint = 0x11F3;
pub const WM8962_WRITE_SEQUENCER_500: c_uint = 0x11F4;
pub const WM8962_WRITE_SEQUENCER_501: c_uint = 0x11F5;
pub const WM8962_WRITE_SEQUENCER_502: c_uint = 0x11F6;
pub const WM8962_WRITE_SEQUENCER_503: c_uint = 0x11F7;
pub const WM8962_WRITE_SEQUENCER_504: c_uint = 0x11F8;
pub const WM8962_WRITE_SEQUENCER_505: c_uint = 0x11F9;
pub const WM8962_WRITE_SEQUENCER_506: c_uint = 0x11FA;
pub const WM8962_WRITE_SEQUENCER_507: c_uint = 0x11FB;
pub const WM8962_WRITE_SEQUENCER_508: c_uint = 0x11FC;
pub const WM8962_WRITE_SEQUENCER_509: c_uint = 0x11FD;
pub const WM8962_WRITE_SEQUENCER_510: c_uint = 0x11FE;
pub const WM8962_WRITE_SEQUENCER_511: c_uint = 0x11FF;
pub const WM8962_DSP2_INSTRUCTION_RAM_0: c_uint = 0x2000;
pub const WM8962_DSP2_ADDRESS_RAM_2: c_uint = 0x2400;
pub const WM8962_DSP2_ADDRESS_RAM_1: c_uint = 0x2401;
pub const WM8962_DSP2_ADDRESS_RAM_0: c_uint = 0x2402;
pub const WM8962_DSP2_DATA1_RAM_1: c_uint = 0x3000;
pub const WM8962_DSP2_DATA1_RAM_0: c_uint = 0x3001;
pub const WM8962_DSP2_DATA2_RAM_1: c_uint = 0x3400;
pub const WM8962_DSP2_DATA2_RAM_0: c_uint = 0x3401;
pub const WM8962_DSP2_DATA3_RAM_1: c_uint = 0x3800;
pub const WM8962_DSP2_DATA3_RAM_0: c_uint = 0x3801;
pub const WM8962_DSP2_COEFF_RAM_0: c_uint = 0x3C00;
pub const WM8962_RETUNEADC_SHARED_COEFF_1: c_uint = 0x4000;
pub const WM8962_RETUNEADC_SHARED_COEFF_0: c_uint = 0x4001;
pub const WM8962_RETUNEDAC_SHARED_COEFF_1: c_uint = 0x4002;
pub const WM8962_RETUNEDAC_SHARED_COEFF_0: c_uint = 0x4003;
pub const WM8962_SOUNDSTAGE_ENABLES_1: c_uint = 0x4004;
pub const WM8962_SOUNDSTAGE_ENABLES_0: c_uint = 0x4005;
pub const WM8962_HDBASS_AI_1: c_uint = 0x4200;
pub const WM8962_HDBASS_AI_0: c_uint = 0x4201;
pub const WM8962_HDBASS_AR_1: c_uint = 0x4202;
pub const WM8962_HDBASS_AR_0: c_uint = 0x4203;
pub const WM8962_HDBASS_B_1: c_uint = 0x4204;
pub const WM8962_HDBASS_B_0: c_uint = 0x4205;
pub const WM8962_HDBASS_K_1: c_uint = 0x4206;
pub const WM8962_HDBASS_K_0: c_uint = 0x4207;
pub const WM8962_HDBASS_N1_1: c_uint = 0x4208;
pub const WM8962_HDBASS_N1_0: c_uint = 0x4209;
pub const WM8962_HDBASS_N2_1: c_uint = 0x420A;
pub const WM8962_HDBASS_N2_0: c_uint = 0x420B;
pub const WM8962_HDBASS_N3_1: c_uint = 0x420C;
pub const WM8962_HDBASS_N3_0: c_uint = 0x420D;
pub const WM8962_HDBASS_N4_1: c_uint = 0x420E;
pub const WM8962_HDBASS_N4_0: c_uint = 0x420F;
pub const WM8962_HDBASS_N5_1: c_uint = 0x4210;
pub const WM8962_HDBASS_N5_0: c_uint = 0x4211;
pub const WM8962_HDBASS_X1_1: c_uint = 0x4212;
pub const WM8962_HDBASS_X1_0: c_uint = 0x4213;
pub const WM8962_HDBASS_X2_1: c_uint = 0x4214;
pub const WM8962_HDBASS_X2_0: c_uint = 0x4215;
pub const WM8962_HDBASS_X3_1: c_uint = 0x4216;
pub const WM8962_HDBASS_X3_0: c_uint = 0x4217;
pub const WM8962_HDBASS_ATK_1: c_uint = 0x4218;
pub const WM8962_HDBASS_ATK_0: c_uint = 0x4219;
pub const WM8962_HDBASS_DCY_1: c_uint = 0x421A;
pub const WM8962_HDBASS_DCY_0: c_uint = 0x421B;
pub const WM8962_HDBASS_PG_1: c_uint = 0x421C;
pub const WM8962_HDBASS_PG_0: c_uint = 0x421D;
pub const WM8962_HPF_C_1: c_uint = 0x4400;
pub const WM8962_HPF_C_0: c_uint = 0x4401;
pub const WM8962_ADCL_RETUNE_C1_1: c_uint = 0x4600;
pub const WM8962_ADCL_RETUNE_C1_0: c_uint = 0x4601;
pub const WM8962_ADCL_RETUNE_C2_1: c_uint = 0x4602;
pub const WM8962_ADCL_RETUNE_C2_0: c_uint = 0x4603;
pub const WM8962_ADCL_RETUNE_C3_1: c_uint = 0x4604;
pub const WM8962_ADCL_RETUNE_C3_0: c_uint = 0x4605;
pub const WM8962_ADCL_RETUNE_C4_1: c_uint = 0x4606;
pub const WM8962_ADCL_RETUNE_C4_0: c_uint = 0x4607;
pub const WM8962_ADCL_RETUNE_C5_1: c_uint = 0x4608;
pub const WM8962_ADCL_RETUNE_C5_0: c_uint = 0x4609;
pub const WM8962_ADCL_RETUNE_C6_1: c_uint = 0x460A;
pub const WM8962_ADCL_RETUNE_C6_0: c_uint = 0x460B;
pub const WM8962_ADCL_RETUNE_C7_1: c_uint = 0x460C;
pub const WM8962_ADCL_RETUNE_C7_0: c_uint = 0x460D;
pub const WM8962_ADCL_RETUNE_C8_1: c_uint = 0x460E;
pub const WM8962_ADCL_RETUNE_C8_0: c_uint = 0x460F;
pub const WM8962_ADCL_RETUNE_C9_1: c_uint = 0x4610;
pub const WM8962_ADCL_RETUNE_C9_0: c_uint = 0x4611;
pub const WM8962_ADCL_RETUNE_C10_1: c_uint = 0x4612;
pub const WM8962_ADCL_RETUNE_C10_0: c_uint = 0x4613;
pub const WM8962_ADCL_RETUNE_C11_1: c_uint = 0x4614;
pub const WM8962_ADCL_RETUNE_C11_0: c_uint = 0x4615;
pub const WM8962_ADCL_RETUNE_C12_1: c_uint = 0x4616;
pub const WM8962_ADCL_RETUNE_C12_0: c_uint = 0x4617;
pub const WM8962_ADCL_RETUNE_C13_1: c_uint = 0x4618;
pub const WM8962_ADCL_RETUNE_C13_0: c_uint = 0x4619;
pub const WM8962_ADCL_RETUNE_C14_1: c_uint = 0x461A;
pub const WM8962_ADCL_RETUNE_C14_0: c_uint = 0x461B;
pub const WM8962_ADCL_RETUNE_C15_1: c_uint = 0x461C;
pub const WM8962_ADCL_RETUNE_C15_0: c_uint = 0x461D;
pub const WM8962_ADCL_RETUNE_C16_1: c_uint = 0x461E;
pub const WM8962_ADCL_RETUNE_C16_0: c_uint = 0x461F;
pub const WM8962_ADCL_RETUNE_C17_1: c_uint = 0x4620;
pub const WM8962_ADCL_RETUNE_C17_0: c_uint = 0x4621;
pub const WM8962_ADCL_RETUNE_C18_1: c_uint = 0x4622;
pub const WM8962_ADCL_RETUNE_C18_0: c_uint = 0x4623;
pub const WM8962_ADCL_RETUNE_C19_1: c_uint = 0x4624;
pub const WM8962_ADCL_RETUNE_C19_0: c_uint = 0x4625;
pub const WM8962_ADCL_RETUNE_C20_1: c_uint = 0x4626;
pub const WM8962_ADCL_RETUNE_C20_0: c_uint = 0x4627;
pub const WM8962_ADCL_RETUNE_C21_1: c_uint = 0x4628;
pub const WM8962_ADCL_RETUNE_C21_0: c_uint = 0x4629;
pub const WM8962_ADCL_RETUNE_C22_1: c_uint = 0x462A;
pub const WM8962_ADCL_RETUNE_C22_0: c_uint = 0x462B;
pub const WM8962_ADCL_RETUNE_C23_1: c_uint = 0x462C;
pub const WM8962_ADCL_RETUNE_C23_0: c_uint = 0x462D;
pub const WM8962_ADCL_RETUNE_C24_1: c_uint = 0x462E;
pub const WM8962_ADCL_RETUNE_C24_0: c_uint = 0x462F;
pub const WM8962_ADCL_RETUNE_C25_1: c_uint = 0x4630;
pub const WM8962_ADCL_RETUNE_C25_0: c_uint = 0x4631;
pub const WM8962_ADCL_RETUNE_C26_1: c_uint = 0x4632;
pub const WM8962_ADCL_RETUNE_C26_0: c_uint = 0x4633;
pub const WM8962_ADCL_RETUNE_C27_1: c_uint = 0x4634;
pub const WM8962_ADCL_RETUNE_C27_0: c_uint = 0x4635;
pub const WM8962_ADCL_RETUNE_C28_1: c_uint = 0x4636;
pub const WM8962_ADCL_RETUNE_C28_0: c_uint = 0x4637;
pub const WM8962_ADCL_RETUNE_C29_1: c_uint = 0x4638;
pub const WM8962_ADCL_RETUNE_C29_0: c_uint = 0x4639;
pub const WM8962_ADCL_RETUNE_C30_1: c_uint = 0x463A;
pub const WM8962_ADCL_RETUNE_C30_0: c_uint = 0x463B;
pub const WM8962_ADCL_RETUNE_C31_1: c_uint = 0x463C;
pub const WM8962_ADCL_RETUNE_C31_0: c_uint = 0x463D;
pub const WM8962_ADCL_RETUNE_C32_1: c_uint = 0x463E;
pub const WM8962_ADCL_RETUNE_C32_0: c_uint = 0x463F;
pub const WM8962_RETUNEADC_PG2_1: c_uint = 0x4800;
pub const WM8962_RETUNEADC_PG2_0: c_uint = 0x4801;
pub const WM8962_RETUNEADC_PG_1: c_uint = 0x4802;
pub const WM8962_RETUNEADC_PG_0: c_uint = 0x4803;
pub const WM8962_ADCR_RETUNE_C1_1: c_uint = 0x4A00;
pub const WM8962_ADCR_RETUNE_C1_0: c_uint = 0x4A01;
pub const WM8962_ADCR_RETUNE_C2_1: c_uint = 0x4A02;
pub const WM8962_ADCR_RETUNE_C2_0: c_uint = 0x4A03;
pub const WM8962_ADCR_RETUNE_C3_1: c_uint = 0x4A04;
pub const WM8962_ADCR_RETUNE_C3_0: c_uint = 0x4A05;
pub const WM8962_ADCR_RETUNE_C4_1: c_uint = 0x4A06;
pub const WM8962_ADCR_RETUNE_C4_0: c_uint = 0x4A07;
pub const WM8962_ADCR_RETUNE_C5_1: c_uint = 0x4A08;
pub const WM8962_ADCR_RETUNE_C5_0: c_uint = 0x4A09;
pub const WM8962_ADCR_RETUNE_C6_1: c_uint = 0x4A0A;
pub const WM8962_ADCR_RETUNE_C6_0: c_uint = 0x4A0B;
pub const WM8962_ADCR_RETUNE_C7_1: c_uint = 0x4A0C;
pub const WM8962_ADCR_RETUNE_C7_0: c_uint = 0x4A0D;
pub const WM8962_ADCR_RETUNE_C8_1: c_uint = 0x4A0E;
pub const WM8962_ADCR_RETUNE_C8_0: c_uint = 0x4A0F;
pub const WM8962_ADCR_RETUNE_C9_1: c_uint = 0x4A10;
pub const WM8962_ADCR_RETUNE_C9_0: c_uint = 0x4A11;
pub const WM8962_ADCR_RETUNE_C10_1: c_uint = 0x4A12;
pub const WM8962_ADCR_RETUNE_C10_0: c_uint = 0x4A13;
pub const WM8962_ADCR_RETUNE_C11_1: c_uint = 0x4A14;
pub const WM8962_ADCR_RETUNE_C11_0: c_uint = 0x4A15;
pub const WM8962_ADCR_RETUNE_C12_1: c_uint = 0x4A16;
pub const WM8962_ADCR_RETUNE_C12_0: c_uint = 0x4A17;
pub const WM8962_ADCR_RETUNE_C13_1: c_uint = 0x4A18;
pub const WM8962_ADCR_RETUNE_C13_0: c_uint = 0x4A19;
pub const WM8962_ADCR_RETUNE_C14_1: c_uint = 0x4A1A;
pub const WM8962_ADCR_RETUNE_C14_0: c_uint = 0x4A1B;
pub const WM8962_ADCR_RETUNE_C15_1: c_uint = 0x4A1C;
pub const WM8962_ADCR_RETUNE_C15_0: c_uint = 0x4A1D;
pub const WM8962_ADCR_RETUNE_C16_1: c_uint = 0x4A1E;
pub const WM8962_ADCR_RETUNE_C16_0: c_uint = 0x4A1F;
pub const WM8962_ADCR_RETUNE_C17_1: c_uint = 0x4A20;
pub const WM8962_ADCR_RETUNE_C17_0: c_uint = 0x4A21;
pub const WM8962_ADCR_RETUNE_C18_1: c_uint = 0x4A22;
pub const WM8962_ADCR_RETUNE_C18_0: c_uint = 0x4A23;
pub const WM8962_ADCR_RETUNE_C19_1: c_uint = 0x4A24;
pub const WM8962_ADCR_RETUNE_C19_0: c_uint = 0x4A25;
pub const WM8962_ADCR_RETUNE_C20_1: c_uint = 0x4A26;
pub const WM8962_ADCR_RETUNE_C20_0: c_uint = 0x4A27;
pub const WM8962_ADCR_RETUNE_C21_1: c_uint = 0x4A28;
pub const WM8962_ADCR_RETUNE_C21_0: c_uint = 0x4A29;
pub const WM8962_ADCR_RETUNE_C22_1: c_uint = 0x4A2A;
pub const WM8962_ADCR_RETUNE_C22_0: c_uint = 0x4A2B;
pub const WM8962_ADCR_RETUNE_C23_1: c_uint = 0x4A2C;
pub const WM8962_ADCR_RETUNE_C23_0: c_uint = 0x4A2D;
pub const WM8962_ADCR_RETUNE_C24_1: c_uint = 0x4A2E;
pub const WM8962_ADCR_RETUNE_C24_0: c_uint = 0x4A2F;
pub const WM8962_ADCR_RETUNE_C25_1: c_uint = 0x4A30;
pub const WM8962_ADCR_RETUNE_C25_0: c_uint = 0x4A31;
pub const WM8962_ADCR_RETUNE_C26_1: c_uint = 0x4A32;
pub const WM8962_ADCR_RETUNE_C26_0: c_uint = 0x4A33;
pub const WM8962_ADCR_RETUNE_C27_1: c_uint = 0x4A34;
pub const WM8962_ADCR_RETUNE_C27_0: c_uint = 0x4A35;
pub const WM8962_ADCR_RETUNE_C28_1: c_uint = 0x4A36;
pub const WM8962_ADCR_RETUNE_C28_0: c_uint = 0x4A37;
pub const WM8962_ADCR_RETUNE_C29_1: c_uint = 0x4A38;
pub const WM8962_ADCR_RETUNE_C29_0: c_uint = 0x4A39;
pub const WM8962_ADCR_RETUNE_C30_1: c_uint = 0x4A3A;
pub const WM8962_ADCR_RETUNE_C30_0: c_uint = 0x4A3B;
pub const WM8962_ADCR_RETUNE_C31_1: c_uint = 0x4A3C;
pub const WM8962_ADCR_RETUNE_C31_0: c_uint = 0x4A3D;
pub const WM8962_ADCR_RETUNE_C32_1: c_uint = 0x4A3E;
pub const WM8962_ADCR_RETUNE_C32_0: c_uint = 0x4A3F;
pub const WM8962_DACL_RETUNE_C1_1: c_uint = 0x4C00;
pub const WM8962_DACL_RETUNE_C1_0: c_uint = 0x4C01;
pub const WM8962_DACL_RETUNE_C2_1: c_uint = 0x4C02;
pub const WM8962_DACL_RETUNE_C2_0: c_uint = 0x4C03;
pub const WM8962_DACL_RETUNE_C3_1: c_uint = 0x4C04;
pub const WM8962_DACL_RETUNE_C3_0: c_uint = 0x4C05;
pub const WM8962_DACL_RETUNE_C4_1: c_uint = 0x4C06;
pub const WM8962_DACL_RETUNE_C4_0: c_uint = 0x4C07;
pub const WM8962_DACL_RETUNE_C5_1: c_uint = 0x4C08;
pub const WM8962_DACL_RETUNE_C5_0: c_uint = 0x4C09;
pub const WM8962_DACL_RETUNE_C6_1: c_uint = 0x4C0A;
pub const WM8962_DACL_RETUNE_C6_0: c_uint = 0x4C0B;
pub const WM8962_DACL_RETUNE_C7_1: c_uint = 0x4C0C;
pub const WM8962_DACL_RETUNE_C7_0: c_uint = 0x4C0D;
pub const WM8962_DACL_RETUNE_C8_1: c_uint = 0x4C0E;
pub const WM8962_DACL_RETUNE_C8_0: c_uint = 0x4C0F;
pub const WM8962_DACL_RETUNE_C9_1: c_uint = 0x4C10;
pub const WM8962_DACL_RETUNE_C9_0: c_uint = 0x4C11;
pub const WM8962_DACL_RETUNE_C10_1: c_uint = 0x4C12;
pub const WM8962_DACL_RETUNE_C10_0: c_uint = 0x4C13;
pub const WM8962_DACL_RETUNE_C11_1: c_uint = 0x4C14;
pub const WM8962_DACL_RETUNE_C11_0: c_uint = 0x4C15;
pub const WM8962_DACL_RETUNE_C12_1: c_uint = 0x4C16;
pub const WM8962_DACL_RETUNE_C12_0: c_uint = 0x4C17;
pub const WM8962_DACL_RETUNE_C13_1: c_uint = 0x4C18;
pub const WM8962_DACL_RETUNE_C13_0: c_uint = 0x4C19;
pub const WM8962_DACL_RETUNE_C14_1: c_uint = 0x4C1A;
pub const WM8962_DACL_RETUNE_C14_0: c_uint = 0x4C1B;
pub const WM8962_DACL_RETUNE_C15_1: c_uint = 0x4C1C;
pub const WM8962_DACL_RETUNE_C15_0: c_uint = 0x4C1D;
pub const WM8962_DACL_RETUNE_C16_1: c_uint = 0x4C1E;
pub const WM8962_DACL_RETUNE_C16_0: c_uint = 0x4C1F;
pub const WM8962_DACL_RETUNE_C17_1: c_uint = 0x4C20;
pub const WM8962_DACL_RETUNE_C17_0: c_uint = 0x4C21;
pub const WM8962_DACL_RETUNE_C18_1: c_uint = 0x4C22;
pub const WM8962_DACL_RETUNE_C18_0: c_uint = 0x4C23;
pub const WM8962_DACL_RETUNE_C19_1: c_uint = 0x4C24;
pub const WM8962_DACL_RETUNE_C19_0: c_uint = 0x4C25;
pub const WM8962_DACL_RETUNE_C20_1: c_uint = 0x4C26;
pub const WM8962_DACL_RETUNE_C20_0: c_uint = 0x4C27;
pub const WM8962_DACL_RETUNE_C21_1: c_uint = 0x4C28;
pub const WM8962_DACL_RETUNE_C21_0: c_uint = 0x4C29;
pub const WM8962_DACL_RETUNE_C22_1: c_uint = 0x4C2A;
pub const WM8962_DACL_RETUNE_C22_0: c_uint = 0x4C2B;
pub const WM8962_DACL_RETUNE_C23_1: c_uint = 0x4C2C;
pub const WM8962_DACL_RETUNE_C23_0: c_uint = 0x4C2D;
pub const WM8962_DACL_RETUNE_C24_1: c_uint = 0x4C2E;
pub const WM8962_DACL_RETUNE_C24_0: c_uint = 0x4C2F;
pub const WM8962_DACL_RETUNE_C25_1: c_uint = 0x4C30;
pub const WM8962_DACL_RETUNE_C25_0: c_uint = 0x4C31;
pub const WM8962_DACL_RETUNE_C26_1: c_uint = 0x4C32;
pub const WM8962_DACL_RETUNE_C26_0: c_uint = 0x4C33;
pub const WM8962_DACL_RETUNE_C27_1: c_uint = 0x4C34;
pub const WM8962_DACL_RETUNE_C27_0: c_uint = 0x4C35;
pub const WM8962_DACL_RETUNE_C28_1: c_uint = 0x4C36;
pub const WM8962_DACL_RETUNE_C28_0: c_uint = 0x4C37;
pub const WM8962_DACL_RETUNE_C29_1: c_uint = 0x4C38;
pub const WM8962_DACL_RETUNE_C29_0: c_uint = 0x4C39;
pub const WM8962_DACL_RETUNE_C30_1: c_uint = 0x4C3A;
pub const WM8962_DACL_RETUNE_C30_0: c_uint = 0x4C3B;
pub const WM8962_DACL_RETUNE_C31_1: c_uint = 0x4C3C;
pub const WM8962_DACL_RETUNE_C31_0: c_uint = 0x4C3D;
pub const WM8962_DACL_RETUNE_C32_1: c_uint = 0x4C3E;
pub const WM8962_DACL_RETUNE_C32_0: c_uint = 0x4C3F;
pub const WM8962_RETUNEDAC_PG2_1: c_uint = 0x4E00;
pub const WM8962_RETUNEDAC_PG2_0: c_uint = 0x4E01;
pub const WM8962_RETUNEDAC_PG_1: c_uint = 0x4E02;
pub const WM8962_RETUNEDAC_PG_0: c_uint = 0x4E03;
pub const WM8962_DACR_RETUNE_C1_1: c_uint = 0x5000;
pub const WM8962_DACR_RETUNE_C1_0: c_uint = 0x5001;
pub const WM8962_DACR_RETUNE_C2_1: c_uint = 0x5002;
pub const WM8962_DACR_RETUNE_C2_0: c_uint = 0x5003;
pub const WM8962_DACR_RETUNE_C3_1: c_uint = 0x5004;
pub const WM8962_DACR_RETUNE_C3_0: c_uint = 0x5005;
pub const WM8962_DACR_RETUNE_C4_1: c_uint = 0x5006;
pub const WM8962_DACR_RETUNE_C4_0: c_uint = 0x5007;
pub const WM8962_DACR_RETUNE_C5_1: c_uint = 0x5008;
pub const WM8962_DACR_RETUNE_C5_0: c_uint = 0x5009;
pub const WM8962_DACR_RETUNE_C6_1: c_uint = 0x500A;
pub const WM8962_DACR_RETUNE_C6_0: c_uint = 0x500B;
pub const WM8962_DACR_RETUNE_C7_1: c_uint = 0x500C;
pub const WM8962_DACR_RETUNE_C7_0: c_uint = 0x500D;
pub const WM8962_DACR_RETUNE_C8_1: c_uint = 0x500E;
pub const WM8962_DACR_RETUNE_C8_0: c_uint = 0x500F;
pub const WM8962_DACR_RETUNE_C9_1: c_uint = 0x5010;
pub const WM8962_DACR_RETUNE_C9_0: c_uint = 0x5011;
pub const WM8962_DACR_RETUNE_C10_1: c_uint = 0x5012;
pub const WM8962_DACR_RETUNE_C10_0: c_uint = 0x5013;
pub const WM8962_DACR_RETUNE_C11_1: c_uint = 0x5014;
pub const WM8962_DACR_RETUNE_C11_0: c_uint = 0x5015;
pub const WM8962_DACR_RETUNE_C12_1: c_uint = 0x5016;
pub const WM8962_DACR_RETUNE_C12_0: c_uint = 0x5017;
pub const WM8962_DACR_RETUNE_C13_1: c_uint = 0x5018;
pub const WM8962_DACR_RETUNE_C13_0: c_uint = 0x5019;
pub const WM8962_DACR_RETUNE_C14_1: c_uint = 0x501A;
pub const WM8962_DACR_RETUNE_C14_0: c_uint = 0x501B;
pub const WM8962_DACR_RETUNE_C15_1: c_uint = 0x501C;
pub const WM8962_DACR_RETUNE_C15_0: c_uint = 0x501D;
pub const WM8962_DACR_RETUNE_C16_1: c_uint = 0x501E;
pub const WM8962_DACR_RETUNE_C16_0: c_uint = 0x501F;
pub const WM8962_DACR_RETUNE_C17_1: c_uint = 0x5020;
pub const WM8962_DACR_RETUNE_C17_0: c_uint = 0x5021;
pub const WM8962_DACR_RETUNE_C18_1: c_uint = 0x5022;
pub const WM8962_DACR_RETUNE_C18_0: c_uint = 0x5023;
pub const WM8962_DACR_RETUNE_C19_1: c_uint = 0x5024;
pub const WM8962_DACR_RETUNE_C19_0: c_uint = 0x5025;
pub const WM8962_DACR_RETUNE_C20_1: c_uint = 0x5026;
pub const WM8962_DACR_RETUNE_C20_0: c_uint = 0x5027;
pub const WM8962_DACR_RETUNE_C21_1: c_uint = 0x5028;
pub const WM8962_DACR_RETUNE_C21_0: c_uint = 0x5029;
pub const WM8962_DACR_RETUNE_C22_1: c_uint = 0x502A;
pub const WM8962_DACR_RETUNE_C22_0: c_uint = 0x502B;
pub const WM8962_DACR_RETUNE_C23_1: c_uint = 0x502C;
pub const WM8962_DACR_RETUNE_C23_0: c_uint = 0x502D;
pub const WM8962_DACR_RETUNE_C24_1: c_uint = 0x502E;
pub const WM8962_DACR_RETUNE_C24_0: c_uint = 0x502F;
pub const WM8962_DACR_RETUNE_C25_1: c_uint = 0x5030;
pub const WM8962_DACR_RETUNE_C25_0: c_uint = 0x5031;
pub const WM8962_DACR_RETUNE_C26_1: c_uint = 0x5032;
pub const WM8962_DACR_RETUNE_C26_0: c_uint = 0x5033;
pub const WM8962_DACR_RETUNE_C27_1: c_uint = 0x5034;
pub const WM8962_DACR_RETUNE_C27_0: c_uint = 0x5035;
pub const WM8962_DACR_RETUNE_C28_1: c_uint = 0x5036;
pub const WM8962_DACR_RETUNE_C28_0: c_uint = 0x5037;
pub const WM8962_DACR_RETUNE_C29_1: c_uint = 0x5038;
pub const WM8962_DACR_RETUNE_C29_0: c_uint = 0x5039;
pub const WM8962_DACR_RETUNE_C30_1: c_uint = 0x503A;
pub const WM8962_DACR_RETUNE_C30_0: c_uint = 0x503B;
pub const WM8962_DACR_RETUNE_C31_1: c_uint = 0x503C;
pub const WM8962_DACR_RETUNE_C31_0: c_uint = 0x503D;
pub const WM8962_DACR_RETUNE_C32_1: c_uint = 0x503E;
pub const WM8962_DACR_RETUNE_C32_0: c_uint = 0x503F;
pub const WM8962_VSS_XHD2_1: c_uint = 0x5200;
pub const WM8962_VSS_XHD2_0: c_uint = 0x5201;
pub const WM8962_VSS_XHD3_1: c_uint = 0x5202;
pub const WM8962_VSS_XHD3_0: c_uint = 0x5203;
pub const WM8962_VSS_XHN1_1: c_uint = 0x5204;
pub const WM8962_VSS_XHN1_0: c_uint = 0x5205;
pub const WM8962_VSS_XHN2_1: c_uint = 0x5206;
pub const WM8962_VSS_XHN2_0: c_uint = 0x5207;
pub const WM8962_VSS_XHN3_1: c_uint = 0x5208;
pub const WM8962_VSS_XHN3_0: c_uint = 0x5209;
pub const WM8962_VSS_XLA_1: c_uint = 0x520A;
pub const WM8962_VSS_XLA_0: c_uint = 0x520B;
pub const WM8962_VSS_XLB_1: c_uint = 0x520C;
pub const WM8962_VSS_XLB_0: c_uint = 0x520D;
pub const WM8962_VSS_XLG_1: c_uint = 0x520E;
pub const WM8962_VSS_XLG_0: c_uint = 0x520F;
pub const WM8962_VSS_PG2_1: c_uint = 0x5210;
pub const WM8962_VSS_PG2_0: c_uint = 0x5211;
pub const WM8962_VSS_PG_1: c_uint = 0x5212;
pub const WM8962_VSS_PG_0: c_uint = 0x5213;
pub const WM8962_VSS_XTD1_1: c_uint = 0x5214;
pub const WM8962_VSS_XTD1_0: c_uint = 0x5215;
pub const WM8962_VSS_XTD2_1: c_uint = 0x5216;
pub const WM8962_VSS_XTD2_0: c_uint = 0x5217;
pub const WM8962_VSS_XTD3_1: c_uint = 0x5218;
pub const WM8962_VSS_XTD3_0: c_uint = 0x5219;
pub const WM8962_VSS_XTD4_1: c_uint = 0x521A;
pub const WM8962_VSS_XTD4_0: c_uint = 0x521B;
pub const WM8962_VSS_XTD5_1: c_uint = 0x521C;
pub const WM8962_VSS_XTD5_0: c_uint = 0x521D;
pub const WM8962_VSS_XTD6_1: c_uint = 0x521E;
pub const WM8962_VSS_XTD6_0: c_uint = 0x521F;
pub const WM8962_VSS_XTD7_1: c_uint = 0x5220;
pub const WM8962_VSS_XTD7_0: c_uint = 0x5221;
pub const WM8962_VSS_XTD8_1: c_uint = 0x5222;
pub const WM8962_VSS_XTD8_0: c_uint = 0x5223;
pub const WM8962_VSS_XTD9_1: c_uint = 0x5224;
pub const WM8962_VSS_XTD9_0: c_uint = 0x5225;
pub const WM8962_VSS_XTD10_1: c_uint = 0x5226;
pub const WM8962_VSS_XTD10_0: c_uint = 0x5227;
pub const WM8962_VSS_XTD11_1: c_uint = 0x5228;
pub const WM8962_VSS_XTD11_0: c_uint = 0x5229;
pub const WM8962_VSS_XTD12_1: c_uint = 0x522A;
pub const WM8962_VSS_XTD12_0: c_uint = 0x522B;
pub const WM8962_VSS_XTD13_1: c_uint = 0x522C;
pub const WM8962_VSS_XTD13_0: c_uint = 0x522D;
pub const WM8962_VSS_XTD14_1: c_uint = 0x522E;
pub const WM8962_VSS_XTD14_0: c_uint = 0x522F;
pub const WM8962_VSS_XTD15_1: c_uint = 0x5230;
pub const WM8962_VSS_XTD15_0: c_uint = 0x5231;
pub const WM8962_VSS_XTD16_1: c_uint = 0x5232;
pub const WM8962_VSS_XTD16_0: c_uint = 0x5233;
pub const WM8962_VSS_XTD17_1: c_uint = 0x5234;
pub const WM8962_VSS_XTD17_0: c_uint = 0x5235;
pub const WM8962_VSS_XTD18_1: c_uint = 0x5236;
pub const WM8962_VSS_XTD18_0: c_uint = 0x5237;
pub const WM8962_VSS_XTD19_1: c_uint = 0x5238;
pub const WM8962_VSS_XTD19_0: c_uint = 0x5239;
pub const WM8962_VSS_XTD20_1: c_uint = 0x523A;
pub const WM8962_VSS_XTD20_0: c_uint = 0x523B;
pub const WM8962_VSS_XTD21_1: c_uint = 0x523C;
pub const WM8962_VSS_XTD21_0: c_uint = 0x523D;
pub const WM8962_VSS_XTD22_1: c_uint = 0x523E;
pub const WM8962_VSS_XTD22_0: c_uint = 0x523F;
pub const WM8962_VSS_XTD23_1: c_uint = 0x5240;
pub const WM8962_VSS_XTD23_0: c_uint = 0x5241;
pub const WM8962_VSS_XTD24_1: c_uint = 0x5242;
pub const WM8962_VSS_XTD24_0: c_uint = 0x5243;
pub const WM8962_VSS_XTD25_1: c_uint = 0x5244;
pub const WM8962_VSS_XTD25_0: c_uint = 0x5245;
pub const WM8962_VSS_XTD26_1: c_uint = 0x5246;
pub const WM8962_VSS_XTD26_0: c_uint = 0x5247;
pub const WM8962_VSS_XTD27_1: c_uint = 0x5248;
pub const WM8962_VSS_XTD27_0: c_uint = 0x5249;
pub const WM8962_VSS_XTD28_1: c_uint = 0x524A;
pub const WM8962_VSS_XTD28_0: c_uint = 0x524B;
pub const WM8962_VSS_XTD29_1: c_uint = 0x524C;
pub const WM8962_VSS_XTD29_0: c_uint = 0x524D;
pub const WM8962_VSS_XTD30_1: c_uint = 0x524E;
pub const WM8962_VSS_XTD30_0: c_uint = 0x524F;
pub const WM8962_VSS_XTD31_1: c_uint = 0x5250;
pub const WM8962_VSS_XTD31_0: c_uint = 0x5251;
pub const WM8962_VSS_XTD32_1: c_uint = 0x5252;
pub const WM8962_VSS_XTD32_0: c_uint = 0x5253;
pub const WM8962_VSS_XTS1_1: c_uint = 0x5254;
pub const WM8962_VSS_XTS1_0: c_uint = 0x5255;
pub const WM8962_VSS_XTS2_1: c_uint = 0x5256;
pub const WM8962_VSS_XTS2_0: c_uint = 0x5257;
pub const WM8962_VSS_XTS3_1: c_uint = 0x5258;
pub const WM8962_VSS_XTS3_0: c_uint = 0x5259;
pub const WM8962_VSS_XTS4_1: c_uint = 0x525A;
pub const WM8962_VSS_XTS4_0: c_uint = 0x525B;
pub const WM8962_VSS_XTS5_1: c_uint = 0x525C;
pub const WM8962_VSS_XTS5_0: c_uint = 0x525D;
pub const WM8962_VSS_XTS6_1: c_uint = 0x525E;
pub const WM8962_VSS_XTS6_0: c_uint = 0x525F;
pub const WM8962_VSS_XTS7_1: c_uint = 0x5260;
pub const WM8962_VSS_XTS7_0: c_uint = 0x5261;
pub const WM8962_VSS_XTS8_1: c_uint = 0x5262;
pub const WM8962_VSS_XTS8_0: c_uint = 0x5263;
pub const WM8962_VSS_XTS9_1: c_uint = 0x5264;
pub const WM8962_VSS_XTS9_0: c_uint = 0x5265;
pub const WM8962_VSS_XTS10_1: c_uint = 0x5266;
pub const WM8962_VSS_XTS10_0: c_uint = 0x5267;
pub const WM8962_VSS_XTS11_1: c_uint = 0x5268;
pub const WM8962_VSS_XTS11_0: c_uint = 0x5269;
pub const WM8962_VSS_XTS12_1: c_uint = 0x526A;
pub const WM8962_VSS_XTS12_0: c_uint = 0x526B;
pub const WM8962_VSS_XTS13_1: c_uint = 0x526C;
pub const WM8962_VSS_XTS13_0: c_uint = 0x526D;
pub const WM8962_VSS_XTS14_1: c_uint = 0x526E;
pub const WM8962_VSS_XTS14_0: c_uint = 0x526F;
pub const WM8962_VSS_XTS15_1: c_uint = 0x5270;
pub const WM8962_VSS_XTS15_0: c_uint = 0x5271;
pub const WM8962_VSS_XTS16_1: c_uint = 0x5272;
pub const WM8962_VSS_XTS16_0: c_uint = 0x5273;
pub const WM8962_VSS_XTS17_1: c_uint = 0x5274;
pub const WM8962_VSS_XTS17_0: c_uint = 0x5275;
pub const WM8962_VSS_XTS18_1: c_uint = 0x5276;
pub const WM8962_VSS_XTS18_0: c_uint = 0x5277;
pub const WM8962_VSS_XTS19_1: c_uint = 0x5278;
pub const WM8962_VSS_XTS19_0: c_uint = 0x5279;
pub const WM8962_VSS_XTS20_1: c_uint = 0x527A;
pub const WM8962_VSS_XTS20_0: c_uint = 0x527B;
pub const WM8962_VSS_XTS21_1: c_uint = 0x527C;
pub const WM8962_VSS_XTS21_0: c_uint = 0x527D;
pub const WM8962_VSS_XTS22_1: c_uint = 0x527E;
pub const WM8962_VSS_XTS22_0: c_uint = 0x527F;
pub const WM8962_VSS_XTS23_1: c_uint = 0x5280;
pub const WM8962_VSS_XTS23_0: c_uint = 0x5281;
pub const WM8962_VSS_XTS24_1: c_uint = 0x5282;
pub const WM8962_VSS_XTS24_0: c_uint = 0x5283;
pub const WM8962_VSS_XTS25_1: c_uint = 0x5284;
pub const WM8962_VSS_XTS25_0: c_uint = 0x5285;
pub const WM8962_VSS_XTS26_1: c_uint = 0x5286;
pub const WM8962_VSS_XTS26_0: c_uint = 0x5287;
pub const WM8962_VSS_XTS27_1: c_uint = 0x5288;
pub const WM8962_VSS_XTS27_0: c_uint = 0x5289;
pub const WM8962_VSS_XTS28_1: c_uint = 0x528A;
pub const WM8962_VSS_XTS28_0: c_uint = 0x528B;
pub const WM8962_VSS_XTS29_1: c_uint = 0x528C;
pub const WM8962_VSS_XTS29_0: c_uint = 0x528D;
pub const WM8962_VSS_XTS30_1: c_uint = 0x528E;
pub const WM8962_VSS_XTS30_0: c_uint = 0x528F;
pub const WM8962_VSS_XTS31_1: c_uint = 0x5290;
pub const WM8962_VSS_XTS31_0: c_uint = 0x5291;
pub const WM8962_VSS_XTS32_1: c_uint = 0x5292;
pub const WM8962_VSS_XTS32_0: c_uint = 0x5293;
pub const WM8962_REGISTER_COUNT: c_int = 1138;
pub const WM8962_MAX_REGISTER: c_uint = 0x5293;
//
// Field Definitions.
//
// R0 (0x00) - Left Input volume
//
pub const WM8962_IN_VU: c_uint = 0x0100  /* IN_VU */;
pub const WM8962_IN_VU_MASK: c_uint = 0x0100  /* IN_VU */;

pub const WM8962_INPGAL_MUTE: c_uint = 0x0080  /* INPGAL_MUTE */;
pub const WM8962_INPGAL_MUTE_MASK: c_uint = 0x0080  /* INPGAL_MUTE */;

pub const WM8962_INL_ZC: c_uint = 0x0040  /* INL_ZC */;
pub const WM8962_INL_ZC_MASK: c_uint = 0x0040  /* INL_ZC */;

pub const WM8962_INL_VOL_MASK: c_uint = 0x003F  /* INL_VOL - [5:0] */;

//
// R1 (0x01) - Right Input volume
//
pub const WM8962_CUST_ID_MASK: c_uint = 0xF000  /* CUST_ID - [15:12] */;

pub const WM8962_CHIP_REV_MASK: c_uint = 0x0E00  /* CHIP_REV - [11:9] */;

pub const WM8962_IN_VU: c_uint = 0x0100  /* IN_VU */;
pub const WM8962_IN_VU_MASK: c_uint = 0x0100  /* IN_VU */;

pub const WM8962_INPGAR_MUTE: c_uint = 0x0080  /* INPGAR_MUTE */;
pub const WM8962_INPGAR_MUTE_MASK: c_uint = 0x0080  /* INPGAR_MUTE */;

pub const WM8962_INR_ZC: c_uint = 0x0040  /* INR_ZC */;
pub const WM8962_INR_ZC_MASK: c_uint = 0x0040  /* INR_ZC */;

pub const WM8962_INR_VOL_MASK: c_uint = 0x003F  /* INR_VOL - [5:0] */;

//
// R2 (0x02) - HPOUTL volume
//
pub const WM8962_HPOUT_VU: c_uint = 0x0100  /* HPOUT_VU */;
pub const WM8962_HPOUT_VU_MASK: c_uint = 0x0100  /* HPOUT_VU */;

pub const WM8962_HPOUTL_ZC: c_uint = 0x0080  /* HPOUTL_ZC */;
pub const WM8962_HPOUTL_ZC_MASK: c_uint = 0x0080  /* HPOUTL_ZC */;

pub const WM8962_HPOUTL_VOL_MASK: c_uint = 0x007F  /* HPOUTL_VOL - [6:0] */;

//
// R3 (0x03) - HPOUTR volume
//
pub const WM8962_HPOUT_VU: c_uint = 0x0100  /* HPOUT_VU */;
pub const WM8962_HPOUT_VU_MASK: c_uint = 0x0100  /* HPOUT_VU */;

pub const WM8962_HPOUTR_ZC: c_uint = 0x0080  /* HPOUTR_ZC */;
pub const WM8962_HPOUTR_ZC_MASK: c_uint = 0x0080  /* HPOUTR_ZC */;

pub const WM8962_HPOUTR_VOL_MASK: c_uint = 0x007F  /* HPOUTR_VOL - [6:0] */;

//
// R4 (0x04) - Clocking1
//
pub const WM8962_DSPCLK_DIV_MASK: c_uint = 0x0600  /* DSPCLK_DIV - [10:9] */;

pub const WM8962_ADCSYS_CLK_DIV_MASK: c_uint = 0x01C0  /* ADCSYS_CLK_DIV - [8:6] */;

pub const WM8962_DACSYS_CLK_DIV_MASK: c_uint = 0x0038  /* DACSYS_CLK_DIV - [5:3] */;

pub const WM8962_MCLKDIV_MASK: c_uint = 0x0006  /* MCLKDIV - [2:1] */;

//
// R5 (0x05) - ADC & DAC Control 1
//
pub const WM8962_ADCR_DAT_INV: c_uint = 0x0040  /* ADCR_DAT_INV */;
pub const WM8962_ADCR_DAT_INV_MASK: c_uint = 0x0040  /* ADCR_DAT_INV */;

pub const WM8962_ADCL_DAT_INV: c_uint = 0x0020  /* ADCL_DAT_INV */;
pub const WM8962_ADCL_DAT_INV_MASK: c_uint = 0x0020  /* ADCL_DAT_INV */;

pub const WM8962_DAC_MUTE_RAMP: c_uint = 0x0010  /* DAC_MUTE_RAMP */;
pub const WM8962_DAC_MUTE_RAMP_MASK: c_uint = 0x0010  /* DAC_MUTE_RAMP */;

pub const WM8962_DAC_MUTE: c_uint = 0x0008  /* DAC_MUTE */;
pub const WM8962_DAC_MUTE_MASK: c_uint = 0x0008  /* DAC_MUTE */;

pub const WM8962_DAC_DEEMP_MASK: c_uint = 0x0006  /* DAC_DEEMP - [2:1] */;

pub const WM8962_ADC_HPF_DIS: c_uint = 0x0001  /* ADC_HPF_DIS */;
pub const WM8962_ADC_HPF_DIS_MASK: c_uint = 0x0001  /* ADC_HPF_DIS */;

//
// R6 (0x06) - ADC & DAC Control 2
//
pub const WM8962_ADC_HPF_SR_MASK: c_uint = 0x3000  /* ADC_HPF_SR - [13:12] */;

pub const WM8962_ADC_HPF_MODE: c_uint = 0x0400  /* ADC_HPF_MODE */;
pub const WM8962_ADC_HPF_MODE_MASK: c_uint = 0x0400  /* ADC_HPF_MODE */;

pub const WM8962_ADC_HPF_CUT_MASK: c_uint = 0x0380  /* ADC_HPF_CUT - [9:7] */;

pub const WM8962_DACR_DAT_INV: c_uint = 0x0040  /* DACR_DAT_INV */;
pub const WM8962_DACR_DAT_INV_MASK: c_uint = 0x0040  /* DACR_DAT_INV */;

pub const WM8962_DACL_DAT_INV: c_uint = 0x0020  /* DACL_DAT_INV */;
pub const WM8962_DACL_DAT_INV_MASK: c_uint = 0x0020  /* DACL_DAT_INV */;

pub const WM8962_DAC_UNMUTE_RAMP: c_uint = 0x0008  /* DAC_UNMUTE_RAMP */;
pub const WM8962_DAC_UNMUTE_RAMP_MASK: c_uint = 0x0008  /* DAC_UNMUTE_RAMP */;

pub const WM8962_DAC_MUTERATE: c_uint = 0x0004  /* DAC_MUTERATE */;
pub const WM8962_DAC_MUTERATE_MASK: c_uint = 0x0004  /* DAC_MUTERATE */;

pub const WM8962_DAC_HP: c_uint = 0x0001  /* DAC_HP */;
pub const WM8962_DAC_HP_MASK: c_uint = 0x0001  /* DAC_HP */;

//
// R7 (0x07) - Audio Interface 0
//
pub const WM8962_AIFDAC_TDM_MODE: c_uint = 0x1000  /* AIFDAC_TDM_MODE */;
pub const WM8962_AIFDAC_TDM_MODE_MASK: c_uint = 0x1000  /* AIFDAC_TDM_MODE */;

pub const WM8962_AIFDAC_TDM_SLOT: c_uint = 0x0800  /* AIFDAC_TDM_SLOT */;
pub const WM8962_AIFDAC_TDM_SLOT_MASK: c_uint = 0x0800  /* AIFDAC_TDM_SLOT */;

pub const WM8962_AIFADC_TDM_MODE: c_uint = 0x0400  /* AIFADC_TDM_MODE */;
pub const WM8962_AIFADC_TDM_MODE_MASK: c_uint = 0x0400  /* AIFADC_TDM_MODE */;

pub const WM8962_AIFADC_TDM_SLOT: c_uint = 0x0200  /* AIFADC_TDM_SLOT */;
pub const WM8962_AIFADC_TDM_SLOT_MASK: c_uint = 0x0200  /* AIFADC_TDM_SLOT */;

pub const WM8962_ADC_LRSWAP: c_uint = 0x0100  /* ADC_LRSWAP */;
pub const WM8962_ADC_LRSWAP_MASK: c_uint = 0x0100  /* ADC_LRSWAP */;

pub const WM8962_BCLK_INV: c_uint = 0x0080  /* BCLK_INV */;
pub const WM8962_BCLK_INV_MASK: c_uint = 0x0080  /* BCLK_INV */;

pub const WM8962_MSTR: c_uint = 0x0040  /* MSTR */;
pub const WM8962_MSTR_MASK: c_uint = 0x0040  /* MSTR */;

pub const WM8962_DAC_LRSWAP: c_uint = 0x0020  /* DAC_LRSWAP */;
pub const WM8962_DAC_LRSWAP_MASK: c_uint = 0x0020  /* DAC_LRSWAP */;

pub const WM8962_LRCLK_INV: c_uint = 0x0010  /* LRCLK_INV */;
pub const WM8962_LRCLK_INV_MASK: c_uint = 0x0010  /* LRCLK_INV */;

pub const WM8962_WL_MASK: c_uint = 0x000C  /* WL - [3:2] */;

pub const WM8962_FMT_MASK: c_uint = 0x0003  /* FMT - [1:0] */;

//
// R8 (0x08) - Clocking2
//
pub const WM8962_CLKREG_OVD: c_uint = 0x0800  /* CLKREG_OVD */;
pub const WM8962_CLKREG_OVD_MASK: c_uint = 0x0800  /* CLKREG_OVD */;

pub const WM8962_SYSCLK_SRC_MASK: c_uint = 0x0600  /* SYSCLK_SRC - [10:9] */;

pub const WM8962_CLASSD_CLK_DIV_MASK: c_uint = 0x01C0  /* CLASSD_CLK_DIV - [8:6] */;

pub const WM8962_SYSCLK_ENA: c_uint = 0x0020  /* SYSCLK_ENA */;
pub const WM8962_SYSCLK_ENA_MASK: c_uint = 0x0020  /* SYSCLK_ENA */;

pub const WM8962_BCLK_DIV_MASK: c_uint = 0x000F  /* BCLK_DIV - [3:0] */;

//
// R9 (0x09) - Audio Interface 1
//
pub const WM8962_AUTOMUTE_STS: c_uint = 0x0800  /* AUTOMUTE_STS */;
pub const WM8962_AUTOMUTE_STS_MASK: c_uint = 0x0800  /* AUTOMUTE_STS */;

pub const WM8962_DAC_AUTOMUTE_SAMPLES_MASK: c_uint = 0x0300  /* DAC_AUTOMUTE_SAMPLES - [9:8] */;

pub const WM8962_DAC_AUTOMUTE: c_uint = 0x0080  /* DAC_AUTOMUTE */;
pub const WM8962_DAC_AUTOMUTE_MASK: c_uint = 0x0080  /* DAC_AUTOMUTE */;

pub const WM8962_DAC_COMP: c_uint = 0x0010  /* DAC_COMP */;
pub const WM8962_DAC_COMP_MASK: c_uint = 0x0010  /* DAC_COMP */;

pub const WM8962_DAC_COMPMODE: c_uint = 0x0008  /* DAC_COMPMODE */;
pub const WM8962_DAC_COMPMODE_MASK: c_uint = 0x0008  /* DAC_COMPMODE */;

pub const WM8962_ADC_COMP: c_uint = 0x0004  /* ADC_COMP */;
pub const WM8962_ADC_COMP_MASK: c_uint = 0x0004  /* ADC_COMP */;

pub const WM8962_ADC_COMPMODE: c_uint = 0x0002  /* ADC_COMPMODE */;
pub const WM8962_ADC_COMPMODE_MASK: c_uint = 0x0002  /* ADC_COMPMODE */;

pub const WM8962_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
pub const WM8962_LOOPBACK_MASK: c_uint = 0x0001  /* LOOPBACK */;

//
// R10 (0x0A) - Left DAC volume
//
pub const WM8962_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8962_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8962_DACL_VOL_MASK: c_uint = 0x00FF  /* DACL_VOL - [7:0] */;

//
// R11 (0x0B) - Right DAC volume
//
pub const WM8962_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8962_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8962_DACR_VOL_MASK: c_uint = 0x00FF  /* DACR_VOL - [7:0] */;

//
// R14 (0x0E) - Audio Interface 2
//
pub const WM8962_AIF_RATE_MASK: c_uint = 0x07FF  /* AIF_RATE - [10:0] */;

//
// R15 (0x0F) - Software Reset
//
pub const WM8962_SW_RESET_MASK: c_uint = 0xFFFF  /* SW_RESET - [15:0] */;

//
// R17 (0x11) - ALC1
//
pub const WM8962_ALC_INACTIVE_ENA: c_uint = 0x0400  /* ALC_INACTIVE_ENA */;
pub const WM8962_ALC_INACTIVE_ENA_MASK: c_uint = 0x0400  /* ALC_INACTIVE_ENA */;

pub const WM8962_ALC_LVL_MODE: c_uint = 0x0200  /* ALC_LVL_MODE */;
pub const WM8962_ALC_LVL_MODE_MASK: c_uint = 0x0200  /* ALC_LVL_MODE */;

pub const WM8962_ALCL_ENA: c_uint = 0x0100  /* ALCL_ENA */;
pub const WM8962_ALCL_ENA_MASK: c_uint = 0x0100  /* ALCL_ENA */;

pub const WM8962_ALCR_ENA: c_uint = 0x0080  /* ALCR_ENA */;
pub const WM8962_ALCR_ENA_MASK: c_uint = 0x0080  /* ALCR_ENA */;

pub const WM8962_ALC_MAXGAIN_MASK: c_uint = 0x0070  /* ALC_MAXGAIN - [6:4] */;

pub const WM8962_ALC_LVL_MASK: c_uint = 0x000F  /* ALC_LVL - [3:0] */;

//
// R18 (0x12) - ALC2
//
pub const WM8962_ALC_LOCK_STS: c_uint = 0x8000  /* ALC_LOCK_STS */;
pub const WM8962_ALC_LOCK_STS_MASK: c_uint = 0x8000  /* ALC_LOCK_STS */;

pub const WM8962_ALC_THRESH_STS: c_uint = 0x4000  /* ALC_THRESH_STS */;
pub const WM8962_ALC_THRESH_STS_MASK: c_uint = 0x4000  /* ALC_THRESH_STS */;

pub const WM8962_ALC_SAT_STS: c_uint = 0x2000  /* ALC_SAT_STS */;
pub const WM8962_ALC_SAT_STS_MASK: c_uint = 0x2000  /* ALC_SAT_STS */;

pub const WM8962_ALC_PKOVR_STS: c_uint = 0x1000  /* ALC_PKOVR_STS */;
pub const WM8962_ALC_PKOVR_STS_MASK: c_uint = 0x1000  /* ALC_PKOVR_STS */;

pub const WM8962_ALC_NGATE_STS: c_uint = 0x0800  /* ALC_NGATE_STS */;
pub const WM8962_ALC_NGATE_STS_MASK: c_uint = 0x0800  /* ALC_NGATE_STS */;

pub const WM8962_ALC_ZC: c_uint = 0x0080  /* ALC_ZC */;
pub const WM8962_ALC_ZC_MASK: c_uint = 0x0080  /* ALC_ZC */;

pub const WM8962_ALC_MINGAIN_MASK: c_uint = 0x0070  /* ALC_MINGAIN - [6:4] */;

pub const WM8962_ALC_HLD_MASK: c_uint = 0x000F  /* ALC_HLD - [3:0] */;

//
// R19 (0x13) - ALC3
//
pub const WM8962_ALC_NGATE_GAIN_MASK: c_uint = 0x1C00  /* ALC_NGATE_GAIN - [12:10] */;

pub const WM8962_ALC_MODE: c_uint = 0x0100  /* ALC_MODE */;
pub const WM8962_ALC_MODE_MASK: c_uint = 0x0100  /* ALC_MODE */;

pub const WM8962_ALC_DCY_MASK: c_uint = 0x00F0  /* ALC_DCY - [7:4] */;

pub const WM8962_ALC_ATK_MASK: c_uint = 0x000F  /* ALC_ATK - [3:0] */;

//
// R20 (0x14) - Noise Gate
//
pub const WM8962_ALC_NGATE_DCY_MASK: c_uint = 0xF000  /* ALC_NGATE_DCY - [15:12] */;

pub const WM8962_ALC_NGATE_ATK_MASK: c_uint = 0x0F00  /* ALC_NGATE_ATK - [11:8] */;

pub const WM8962_ALC_NGATE_THR_MASK: c_uint = 0x00F8  /* ALC_NGATE_THR - [7:3] */;

pub const WM8962_ALC_NGATE_MODE_MASK: c_uint = 0x0006  /* ALC_NGATE_MODE - [2:1] */;

pub const WM8962_ALC_NGATE_ENA: c_uint = 0x0001  /* ALC_NGATE_ENA */;
pub const WM8962_ALC_NGATE_ENA_MASK: c_uint = 0x0001  /* ALC_NGATE_ENA */;

//
// R21 (0x15) - Left ADC volume
//
pub const WM8962_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8962_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8962_ADCL_VOL_MASK: c_uint = 0x00FF  /* ADCL_VOL - [7:0] */;

//
// R22 (0x16) - Right ADC volume
//
pub const WM8962_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8962_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8962_ADCR_VOL_MASK: c_uint = 0x00FF  /* ADCR_VOL - [7:0] */;

//
// R23 (0x17) - Additional control(1)
//
pub const WM8962_THERR_ACT: c_uint = 0x0100  /* THERR_ACT */;
pub const WM8962_THERR_ACT_MASK: c_uint = 0x0100  /* THERR_ACT */;

pub const WM8962_ADC_BIAS: c_uint = 0x0040  /* ADC_BIAS */;
pub const WM8962_ADC_BIAS_MASK: c_uint = 0x0040  /* ADC_BIAS */;

pub const WM8962_ADC_HP: c_uint = 0x0020  /* ADC_HP */;
pub const WM8962_ADC_HP_MASK: c_uint = 0x0020  /* ADC_HP */;

pub const WM8962_TOCLK_ENA: c_uint = 0x0001  /* TOCLK_ENA */;
pub const WM8962_TOCLK_ENA_MASK: c_uint = 0x0001  /* TOCLK_ENA */;

//
// R24 (0x18) - Additional control(2)
//
pub const WM8962_AIF_TRI: c_uint = 0x0008  /* AIF_TRI */;
pub const WM8962_AIF_TRI_MASK: c_uint = 0x0008  /* AIF_TRI */;

//
// R25 (0x19) - Pwr Mgmt (1)
//
pub const WM8962_DMIC_ENA: c_uint = 0x0400  /* DMIC_ENA */;
pub const WM8962_DMIC_ENA_MASK: c_uint = 0x0400  /* DMIC_ENA */;

pub const WM8962_OPCLK_ENA: c_uint = 0x0200  /* OPCLK_ENA */;
pub const WM8962_OPCLK_ENA_MASK: c_uint = 0x0200  /* OPCLK_ENA */;

pub const WM8962_VMID_SEL_MASK: c_uint = 0x0180  /* VMID_SEL - [8:7] */;

pub const WM8962_BIAS_ENA: c_uint = 0x0040  /* BIAS_ENA */;
pub const WM8962_BIAS_ENA_MASK: c_uint = 0x0040  /* BIAS_ENA */;

pub const WM8962_INL_ENA: c_uint = 0x0020  /* INL_ENA */;
pub const WM8962_INL_ENA_MASK: c_uint = 0x0020  /* INL_ENA */;

pub const WM8962_INR_ENA: c_uint = 0x0010  /* INR_ENA */;
pub const WM8962_INR_ENA_MASK: c_uint = 0x0010  /* INR_ENA */;

pub const WM8962_ADCL_ENA: c_uint = 0x0008  /* ADCL_ENA */;
pub const WM8962_ADCL_ENA_MASK: c_uint = 0x0008  /* ADCL_ENA */;

pub const WM8962_ADCR_ENA: c_uint = 0x0004  /* ADCR_ENA */;
pub const WM8962_ADCR_ENA_MASK: c_uint = 0x0004  /* ADCR_ENA */;

pub const WM8962_MICBIAS_ENA: c_uint = 0x0002  /* MICBIAS_ENA */;
pub const WM8962_MICBIAS_ENA_MASK: c_uint = 0x0002  /* MICBIAS_ENA */;

//
// R26 (0x1A) - Pwr Mgmt (2)
//
pub const WM8962_DACL_ENA: c_uint = 0x0100  /* DACL_ENA */;
pub const WM8962_DACL_ENA_MASK: c_uint = 0x0100  /* DACL_ENA */;

pub const WM8962_DACR_ENA: c_uint = 0x0080  /* DACR_ENA */;
pub const WM8962_DACR_ENA_MASK: c_uint = 0x0080  /* DACR_ENA */;

pub const WM8962_HPOUTL_PGA_ENA: c_uint = 0x0040  /* HPOUTL_PGA_ENA */;
pub const WM8962_HPOUTL_PGA_ENA_MASK: c_uint = 0x0040  /* HPOUTL_PGA_ENA */;

pub const WM8962_HPOUTR_PGA_ENA: c_uint = 0x0020  /* HPOUTR_PGA_ENA */;
pub const WM8962_HPOUTR_PGA_ENA_MASK: c_uint = 0x0020  /* HPOUTR_PGA_ENA */;

pub const WM8962_SPKOUTL_PGA_ENA: c_uint = 0x0010  /* SPKOUTL_PGA_ENA */;
pub const WM8962_SPKOUTL_PGA_ENA_MASK: c_uint = 0x0010  /* SPKOUTL_PGA_ENA */;

pub const WM8962_SPKOUTR_PGA_ENA: c_uint = 0x0008  /* SPKOUTR_PGA_ENA */;
pub const WM8962_SPKOUTR_PGA_ENA_MASK: c_uint = 0x0008  /* SPKOUTR_PGA_ENA */;

pub const WM8962_HPOUTL_PGA_MUTE: c_uint = 0x0002  /* HPOUTL_PGA_MUTE */;
pub const WM8962_HPOUTL_PGA_MUTE_MASK: c_uint = 0x0002  /* HPOUTL_PGA_MUTE */;

pub const WM8962_HPOUTR_PGA_MUTE: c_uint = 0x0001  /* HPOUTR_PGA_MUTE */;
pub const WM8962_HPOUTR_PGA_MUTE_MASK: c_uint = 0x0001  /* HPOUTR_PGA_MUTE */;

//
// R27 (0x1B) - Additional Control (3)
//
pub const WM8962_SAMPLE_RATE_INT_MODE: c_uint = 0x0010  /* SAMPLE_RATE_INT_MODE */;
pub const WM8962_SAMPLE_RATE_INT_MODE_MASK: c_uint = 0x0010  /* SAMPLE_RATE_INT_MODE */;

pub const WM8962_SAMPLE_RATE_MASK: c_uint = 0x0007  /* SAMPLE_RATE - [2:0] */;

//
// R28 (0x1C) - Anti-pop
//
pub const WM8962_STARTUP_BIAS_ENA: c_uint = 0x0010  /* STARTUP_BIAS_ENA */;
pub const WM8962_STARTUP_BIAS_ENA_MASK: c_uint = 0x0010  /* STARTUP_BIAS_ENA */;

pub const WM8962_VMID_BUF_ENA: c_uint = 0x0008  /* VMID_BUF_ENA */;
pub const WM8962_VMID_BUF_ENA_MASK: c_uint = 0x0008  /* VMID_BUF_ENA */;

pub const WM8962_VMID_RAMP: c_uint = 0x0004  /* VMID_RAMP */;
pub const WM8962_VMID_RAMP_MASK: c_uint = 0x0004  /* VMID_RAMP */;

//
// R30 (0x1E) - Clocking 3
//
pub const WM8962_DBCLK_DIV_MASK: c_uint = 0xE000  /* DBCLK_DIV - [15:13] */;

pub const WM8962_OPCLK_DIV_MASK: c_uint = 0x1C00  /* OPCLK_DIV - [12:10] */;

pub const WM8962_TOCLK_DIV_MASK: c_uint = 0x0380  /* TOCLK_DIV - [9:7] */;

pub const WM8962_F256KCLK_DIV_MASK: c_uint = 0x007E  /* F256KCLK_DIV - [6:1] */;

//
// R31 (0x1F) - Input mixer control (1)
//
pub const WM8962_MIXINL_MUTE: c_uint = 0x0008  /* MIXINL_MUTE */;
pub const WM8962_MIXINL_MUTE_MASK: c_uint = 0x0008  /* MIXINL_MUTE */;

pub const WM8962_MIXINR_MUTE: c_uint = 0x0004  /* MIXINR_MUTE */;
pub const WM8962_MIXINR_MUTE_MASK: c_uint = 0x0004  /* MIXINR_MUTE */;

pub const WM8962_MIXINL_ENA: c_uint = 0x0002  /* MIXINL_ENA */;
pub const WM8962_MIXINL_ENA_MASK: c_uint = 0x0002  /* MIXINL_ENA */;

pub const WM8962_MIXINR_ENA: c_uint = 0x0001  /* MIXINR_ENA */;
pub const WM8962_MIXINR_ENA_MASK: c_uint = 0x0001  /* MIXINR_ENA */;

//
// R32 (0x20) - Left input mixer volume
//
pub const WM8962_IN2L_MIXINL_VOL_MASK: c_uint = 0x01C0  /* IN2L_MIXINL_VOL - [8:6] */;

pub const WM8962_INPGAL_MIXINL_VOL_MASK: c_uint = 0x0038  /* INPGAL_MIXINL_VOL - [5:3] */;

pub const WM8962_IN3L_MIXINL_VOL_MASK: c_uint = 0x0007  /* IN3L_MIXINL_VOL - [2:0] */;

//
// R33 (0x21) - Right input mixer volume
//
pub const WM8962_IN2R_MIXINR_VOL_MASK: c_uint = 0x01C0  /* IN2R_MIXINR_VOL - [8:6] */;

pub const WM8962_INPGAR_MIXINR_VOL_MASK: c_uint = 0x0038  /* INPGAR_MIXINR_VOL - [5:3] */;

pub const WM8962_IN3R_MIXINR_VOL_MASK: c_uint = 0x0007  /* IN3R_MIXINR_VOL - [2:0] */;

//
// R34 (0x22) - Input mixer control (2)
//
pub const WM8962_IN2L_TO_MIXINL: c_uint = 0x0020  /* IN2L_TO_MIXINL */;
pub const WM8962_IN2L_TO_MIXINL_MASK: c_uint = 0x0020  /* IN2L_TO_MIXINL */;

pub const WM8962_IN3L_TO_MIXINL: c_uint = 0x0010  /* IN3L_TO_MIXINL */;
pub const WM8962_IN3L_TO_MIXINL_MASK: c_uint = 0x0010  /* IN3L_TO_MIXINL */;

pub const WM8962_INPGAL_TO_MIXINL: c_uint = 0x0008  /* INPGAL_TO_MIXINL */;
pub const WM8962_INPGAL_TO_MIXINL_MASK: c_uint = 0x0008  /* INPGAL_TO_MIXINL */;

pub const WM8962_IN2R_TO_MIXINR: c_uint = 0x0004  /* IN2R_TO_MIXINR */;
pub const WM8962_IN2R_TO_MIXINR_MASK: c_uint = 0x0004  /* IN2R_TO_MIXINR */;

pub const WM8962_IN3R_TO_MIXINR: c_uint = 0x0002  /* IN3R_TO_MIXINR */;
pub const WM8962_IN3R_TO_MIXINR_MASK: c_uint = 0x0002  /* IN3R_TO_MIXINR */;

pub const WM8962_INPGAR_TO_MIXINR: c_uint = 0x0001  /* INPGAR_TO_MIXINR */;
pub const WM8962_INPGAR_TO_MIXINR_MASK: c_uint = 0x0001  /* INPGAR_TO_MIXINR */;

//
// R35 (0x23) - Input bias control
//
pub const WM8962_MIXIN_BIAS_MASK: c_uint = 0x0038  /* MIXIN_BIAS - [5:3] */;

pub const WM8962_INPGA_BIAS_MASK: c_uint = 0x0007  /* INPGA_BIAS - [2:0] */;

//
// R37 (0x25) - Left input PGA control
//
pub const WM8962_INPGAL_ENA: c_uint = 0x0010  /* INPGAL_ENA */;
pub const WM8962_INPGAL_ENA_MASK: c_uint = 0x0010  /* INPGAL_ENA */;

pub const WM8962_IN1L_TO_INPGAL: c_uint = 0x0008  /* IN1L_TO_INPGAL */;
pub const WM8962_IN1L_TO_INPGAL_MASK: c_uint = 0x0008  /* IN1L_TO_INPGAL */;

pub const WM8962_IN2L_TO_INPGAL: c_uint = 0x0004  /* IN2L_TO_INPGAL */;
pub const WM8962_IN2L_TO_INPGAL_MASK: c_uint = 0x0004  /* IN2L_TO_INPGAL */;

pub const WM8962_IN3L_TO_INPGAL: c_uint = 0x0002  /* IN3L_TO_INPGAL */;
pub const WM8962_IN3L_TO_INPGAL_MASK: c_uint = 0x0002  /* IN3L_TO_INPGAL */;

pub const WM8962_IN4L_TO_INPGAL: c_uint = 0x0001  /* IN4L_TO_INPGAL */;
pub const WM8962_IN4L_TO_INPGAL_MASK: c_uint = 0x0001  /* IN4L_TO_INPGAL */;

//
// R38 (0x26) - Right input PGA control
//
pub const WM8962_INPGAR_ENA: c_uint = 0x0010  /* INPGAR_ENA */;
pub const WM8962_INPGAR_ENA_MASK: c_uint = 0x0010  /* INPGAR_ENA */;

pub const WM8962_IN1R_TO_INPGAR: c_uint = 0x0008  /* IN1R_TO_INPGAR */;
pub const WM8962_IN1R_TO_INPGAR_MASK: c_uint = 0x0008  /* IN1R_TO_INPGAR */;

pub const WM8962_IN2R_TO_INPGAR: c_uint = 0x0004  /* IN2R_TO_INPGAR */;
pub const WM8962_IN2R_TO_INPGAR_MASK: c_uint = 0x0004  /* IN2R_TO_INPGAR */;

pub const WM8962_IN3R_TO_INPGAR: c_uint = 0x0002  /* IN3R_TO_INPGAR */;
pub const WM8962_IN3R_TO_INPGAR_MASK: c_uint = 0x0002  /* IN3R_TO_INPGAR */;

pub const WM8962_IN4R_TO_INPGAR: c_uint = 0x0001  /* IN4R_TO_INPGAR */;
pub const WM8962_IN4R_TO_INPGAR_MASK: c_uint = 0x0001  /* IN4R_TO_INPGAR */;

//
// R40 (0x28) - SPKOUTL volume
//
pub const WM8962_SPKOUT_VU: c_uint = 0x0100  /* SPKOUT_VU */;
pub const WM8962_SPKOUT_VU_MASK: c_uint = 0x0100  /* SPKOUT_VU */;

pub const WM8962_SPKOUTL_ZC: c_uint = 0x0080  /* SPKOUTL_ZC */;
pub const WM8962_SPKOUTL_ZC_MASK: c_uint = 0x0080  /* SPKOUTL_ZC */;

pub const WM8962_SPKOUTL_VOL_MASK: c_uint = 0x007F  /* SPKOUTL_VOL - [6:0] */;

//
// R41 (0x29) - SPKOUTR volume
//
pub const WM8962_SPKOUTR_ZC: c_uint = 0x0080  /* SPKOUTR_ZC */;
pub const WM8962_SPKOUTR_ZC_MASK: c_uint = 0x0080  /* SPKOUTR_ZC */;

pub const WM8962_SPKOUTR_VOL_MASK: c_uint = 0x007F  /* SPKOUTR_VOL - [6:0] */;

//
// R47 (0x2F) - Thermal Shutdown Status
//
pub const WM8962_TEMP_ERR_HP: c_uint = 0x0008  /* TEMP_ERR_HP */;
pub const WM8962_TEMP_ERR_HP_MASK: c_uint = 0x0008  /* TEMP_ERR_HP */;

pub const WM8962_TEMP_WARN_HP: c_uint = 0x0004  /* TEMP_WARN_HP */;
pub const WM8962_TEMP_WARN_HP_MASK: c_uint = 0x0004  /* TEMP_WARN_HP */;

pub const WM8962_TEMP_ERR_SPK: c_uint = 0x0002  /* TEMP_ERR_SPK */;
pub const WM8962_TEMP_ERR_SPK_MASK: c_uint = 0x0002  /* TEMP_ERR_SPK */;

pub const WM8962_TEMP_WARN_SPK: c_uint = 0x0001  /* TEMP_WARN_SPK */;
pub const WM8962_TEMP_WARN_SPK_MASK: c_uint = 0x0001  /* TEMP_WARN_SPK */;

//
// R48 (0x30) - Additional Control (4)
//
pub const WM8962_MICDET_THR_MASK: c_uint = 0x7000  /* MICDET_THR - [14:12] */;

pub const WM8962_MICSHORT_THR_MASK: c_uint = 0x0C00  /* MICSHORT_THR - [11:10] */;

pub const WM8962_MICDET_ENA: c_uint = 0x0200  /* MICDET_ENA */;
pub const WM8962_MICDET_ENA_MASK: c_uint = 0x0200  /* MICDET_ENA */;

pub const WM8962_MICDET_STS: c_uint = 0x0080  /* MICDET_STS */;
pub const WM8962_MICDET_STS_MASK: c_uint = 0x0080  /* MICDET_STS */;

pub const WM8962_MICSHORT_STS: c_uint = 0x0040  /* MICSHORT_STS */;
pub const WM8962_MICSHORT_STS_MASK: c_uint = 0x0040  /* MICSHORT_STS */;

pub const WM8962_TEMP_ENA_HP: c_uint = 0x0004  /* TEMP_ENA_HP */;
pub const WM8962_TEMP_ENA_HP_MASK: c_uint = 0x0004  /* TEMP_ENA_HP */;

pub const WM8962_TEMP_ENA_SPK: c_uint = 0x0002  /* TEMP_ENA_SPK */;
pub const WM8962_TEMP_ENA_SPK_MASK: c_uint = 0x0002  /* TEMP_ENA_SPK */;

pub const WM8962_MICBIAS_LVL: c_uint = 0x0001  /* MICBIAS_LVL */;
pub const WM8962_MICBIAS_LVL_MASK: c_uint = 0x0001  /* MICBIAS_LVL */;

//
// R49 (0x31) - Class D Control 1
//
pub const WM8962_SPKOUTR_ENA: c_uint = 0x0080  /* SPKOUTR_ENA */;
pub const WM8962_SPKOUTR_ENA_MASK: c_uint = 0x0080  /* SPKOUTR_ENA */;

pub const WM8962_SPKOUTL_ENA: c_uint = 0x0040  /* SPKOUTL_ENA */;
pub const WM8962_SPKOUTL_ENA_MASK: c_uint = 0x0040  /* SPKOUTL_ENA */;

pub const WM8962_DAC_MUTE_ALT: c_uint = 0x0010  /* DAC_MUTE */;
pub const WM8962_DAC_MUTE_ALT_MASK: c_uint = 0x0010  /* DAC_MUTE */;

pub const WM8962_SPKOUTL_PGA_MUTE: c_uint = 0x0002  /* SPKOUTL_PGA_MUTE */;
pub const WM8962_SPKOUTL_PGA_MUTE_MASK: c_uint = 0x0002  /* SPKOUTL_PGA_MUTE */;

pub const WM8962_SPKOUTR_PGA_MUTE: c_uint = 0x0001  /* SPKOUTR_PGA_MUTE */;
pub const WM8962_SPKOUTR_PGA_MUTE_MASK: c_uint = 0x0001  /* SPKOUTR_PGA_MUTE */;

//
// R51 (0x33) - Class D Control 2
//
pub const WM8962_SPK_MONO: c_uint = 0x0040  /* SPK_MONO */;
pub const WM8962_SPK_MONO_MASK: c_uint = 0x0040  /* SPK_MONO */;

pub const WM8962_CLASSD_VOL_MASK: c_uint = 0x0007  /* CLASSD_VOL - [2:0] */;

//
// R56 (0x38) - Clocking 4
//
pub const WM8962_SYSCLK_RATE_MASK: c_uint = 0x001E  /* SYSCLK_RATE - [4:1] */;

//
// R57 (0x39) - DAC DSP Mixing (1)
//
pub const WM8962_DAC_MONOMIX: c_uint = 0x0200  /* DAC_MONOMIX */;
pub const WM8962_DAC_MONOMIX_MASK: c_uint = 0x0200  /* DAC_MONOMIX */;

pub const WM8962_ADCR_DAC_SVOL_MASK: c_uint = 0x00F0  /* ADCR_DAC_SVOL - [7:4] */;

pub const WM8962_ADC_TO_DACR_MASK: c_uint = 0x000C  /* ADC_TO_DACR - [3:2] */;

//
// R58 (0x3A) - DAC DSP Mixing (2)
//
pub const WM8962_ADCL_DAC_SVOL_MASK: c_uint = 0x00F0  /* ADCL_DAC_SVOL - [7:4] */;

pub const WM8962_ADC_TO_DACL_MASK: c_uint = 0x000C  /* ADC_TO_DACL - [3:2] */;

//
// R60 (0x3C) - DC Servo 0
//
pub const WM8962_INL_DCS_ENA: c_uint = 0x0080  /* INL_DCS_ENA */;
pub const WM8962_INL_DCS_ENA_MASK: c_uint = 0x0080  /* INL_DCS_ENA */;

pub const WM8962_INL_DCS_STARTUP: c_uint = 0x0040  /* INL_DCS_STARTUP */;
pub const WM8962_INL_DCS_STARTUP_MASK: c_uint = 0x0040  /* INL_DCS_STARTUP */;

pub const WM8962_INR_DCS_ENA: c_uint = 0x0008  /* INR_DCS_ENA */;
pub const WM8962_INR_DCS_ENA_MASK: c_uint = 0x0008  /* INR_DCS_ENA */;

pub const WM8962_INR_DCS_STARTUP: c_uint = 0x0004  /* INR_DCS_STARTUP */;
pub const WM8962_INR_DCS_STARTUP_MASK: c_uint = 0x0004  /* INR_DCS_STARTUP */;

//
// R61 (0x3D) - DC Servo 1
//
pub const WM8962_HP1L_DCS_ENA: c_uint = 0x0080  /* HP1L_DCS_ENA */;
pub const WM8962_HP1L_DCS_ENA_MASK: c_uint = 0x0080  /* HP1L_DCS_ENA */;

pub const WM8962_HP1L_DCS_STARTUP: c_uint = 0x0040  /* HP1L_DCS_STARTUP */;
pub const WM8962_HP1L_DCS_STARTUP_MASK: c_uint = 0x0040  /* HP1L_DCS_STARTUP */;

pub const WM8962_HP1L_DCS_SYNC: c_uint = 0x0010  /* HP1L_DCS_SYNC */;
pub const WM8962_HP1L_DCS_SYNC_MASK: c_uint = 0x0010  /* HP1L_DCS_SYNC */;

pub const WM8962_HP1R_DCS_ENA: c_uint = 0x0008  /* HP1R_DCS_ENA */;
pub const WM8962_HP1R_DCS_ENA_MASK: c_uint = 0x0008  /* HP1R_DCS_ENA */;

pub const WM8962_HP1R_DCS_STARTUP: c_uint = 0x0004  /* HP1R_DCS_STARTUP */;
pub const WM8962_HP1R_DCS_STARTUP_MASK: c_uint = 0x0004  /* HP1R_DCS_STARTUP */;

pub const WM8962_HP1R_DCS_SYNC: c_uint = 0x0001  /* HP1R_DCS_SYNC */;
pub const WM8962_HP1R_DCS_SYNC_MASK: c_uint = 0x0001  /* HP1R_DCS_SYNC */;

//
// R64 (0x40) - DC Servo 4
//
pub const WM8962_HP1_DCS_SYNC_STEPS_MASK: c_uint = 0x3F80  /* HP1_DCS_SYNC_STEPS - [13:7] */;

//
// R66 (0x42) - DC Servo 6
//
pub const WM8962_DCS_STARTUP_DONE_INL: c_uint = 0x0400  /* DCS_STARTUP_DONE_INL */;
pub const WM8962_DCS_STARTUP_DONE_INL_MASK: c_uint = 0x0400  /* DCS_STARTUP_DONE_INL */;

pub const WM8962_DCS_STARTUP_DONE_INR: c_uint = 0x0200  /* DCS_STARTUP_DONE_INR */;
pub const WM8962_DCS_STARTUP_DONE_INR_MASK: c_uint = 0x0200  /* DCS_STARTUP_DONE_INR */;

pub const WM8962_DCS_STARTUP_DONE_HP1L: c_uint = 0x0100  /* DCS_STARTUP_DONE_HP1L */;
pub const WM8962_DCS_STARTUP_DONE_HP1L_MASK: c_uint = 0x0100  /* DCS_STARTUP_DONE_HP1L */;

pub const WM8962_DCS_STARTUP_DONE_HP1R: c_uint = 0x0080  /* DCS_STARTUP_DONE_HP1R */;
pub const WM8962_DCS_STARTUP_DONE_HP1R_MASK: c_uint = 0x0080  /* DCS_STARTUP_DONE_HP1R */;

//
// R68 (0x44) - Analogue PGA Bias
//
pub const WM8962_HP_PGAS_BIAS_MASK: c_uint = 0x0007  /* HP_PGAS_BIAS - [2:0] */;

//
// R69 (0x45) - Analogue HP 0
//
pub const WM8962_HP1L_RMV_SHORT: c_uint = 0x0080  /* HP1L_RMV_SHORT */;
pub const WM8962_HP1L_RMV_SHORT_MASK: c_uint = 0x0080  /* HP1L_RMV_SHORT */;

pub const WM8962_HP1L_ENA_OUTP: c_uint = 0x0040  /* HP1L_ENA_OUTP */;
pub const WM8962_HP1L_ENA_OUTP_MASK: c_uint = 0x0040  /* HP1L_ENA_OUTP */;

pub const WM8962_HP1L_ENA_DLY: c_uint = 0x0020  /* HP1L_ENA_DLY */;
pub const WM8962_HP1L_ENA_DLY_MASK: c_uint = 0x0020  /* HP1L_ENA_DLY */;

pub const WM8962_HP1L_ENA: c_uint = 0x0010  /* HP1L_ENA */;
pub const WM8962_HP1L_ENA_MASK: c_uint = 0x0010  /* HP1L_ENA */;

pub const WM8962_HP1R_RMV_SHORT: c_uint = 0x0008  /* HP1R_RMV_SHORT */;
pub const WM8962_HP1R_RMV_SHORT_MASK: c_uint = 0x0008  /* HP1R_RMV_SHORT */;

pub const WM8962_HP1R_ENA_OUTP: c_uint = 0x0004  /* HP1R_ENA_OUTP */;
pub const WM8962_HP1R_ENA_OUTP_MASK: c_uint = 0x0004  /* HP1R_ENA_OUTP */;

pub const WM8962_HP1R_ENA_DLY: c_uint = 0x0002  /* HP1R_ENA_DLY */;
pub const WM8962_HP1R_ENA_DLY_MASK: c_uint = 0x0002  /* HP1R_ENA_DLY */;

pub const WM8962_HP1R_ENA: c_uint = 0x0001  /* HP1R_ENA */;
pub const WM8962_HP1R_ENA_MASK: c_uint = 0x0001  /* HP1R_ENA */;

//
// R71 (0x47) - Analogue HP 2
//
pub const WM8962_HP1L_VOL_MASK: c_uint = 0x01C0  /* HP1L_VOL - [8:6] */;

pub const WM8962_HP1R_VOL_MASK: c_uint = 0x0038  /* HP1R_VOL - [5:3] */;

pub const WM8962_HP_BIAS_BOOST_MASK: c_uint = 0x0007  /* HP_BIAS_BOOST - [2:0] */;

//
// R72 (0x48) - Charge Pump 1
//
pub const WM8962_CP_ENA: c_uint = 0x0001  /* CP_ENA */;
pub const WM8962_CP_ENA_MASK: c_uint = 0x0001  /* CP_ENA */;

//
// R82 (0x52) - Charge Pump B
//
pub const WM8962_CP_DYN_PWR: c_uint = 0x0001  /* CP_DYN_PWR */;
pub const WM8962_CP_DYN_PWR_MASK: c_uint = 0x0001  /* CP_DYN_PWR */;

//
// R87 (0x57) - Write Sequencer Control 1
//
pub const WM8962_WSEQ_AUTOSEQ_ENA: c_uint = 0x0080  /* WSEQ_AUTOSEQ_ENA */;
pub const WM8962_WSEQ_AUTOSEQ_ENA_MASK: c_uint = 0x0080  /* WSEQ_AUTOSEQ_ENA */;

pub const WM8962_WSEQ_ENA: c_uint = 0x0020  /* WSEQ_ENA */;
pub const WM8962_WSEQ_ENA_MASK: c_uint = 0x0020  /* WSEQ_ENA */;

//
// R90 (0x5A) - Write Sequencer Control 2
//
pub const WM8962_WSEQ_ABORT: c_uint = 0x0100  /* WSEQ_ABORT */;
pub const WM8962_WSEQ_ABORT_MASK: c_uint = 0x0100  /* WSEQ_ABORT */;

pub const WM8962_WSEQ_START: c_uint = 0x0080  /* WSEQ_START */;
pub const WM8962_WSEQ_START_MASK: c_uint = 0x0080  /* WSEQ_START */;

pub const WM8962_WSEQ_START_INDEX_MASK: c_uint = 0x007F  /* WSEQ_START_INDEX - [6:0] */;

//
// R93 (0x5D) - Write Sequencer Control 3
//
pub const WM8962_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x03F8  /* WSEQ_CURRENT_INDEX - [9:3] */;

pub const WM8962_WSEQ_BUSY: c_uint = 0x0001  /* WSEQ_BUSY */;
pub const WM8962_WSEQ_BUSY_MASK: c_uint = 0x0001  /* WSEQ_BUSY */;

//
// R94 (0x5E) - Control Interface
//
pub const WM8962_SPI_CONTRD: c_uint = 0x0040  /* SPI_CONTRD */;
pub const WM8962_SPI_CONTRD_MASK: c_uint = 0x0040  /* SPI_CONTRD */;

pub const WM8962_SPI_4WIRE: c_uint = 0x0020  /* SPI_4WIRE */;
pub const WM8962_SPI_4WIRE_MASK: c_uint = 0x0020  /* SPI_4WIRE */;

pub const WM8962_SPI_CFG: c_uint = 0x0010  /* SPI_CFG */;
pub const WM8962_SPI_CFG_MASK: c_uint = 0x0010  /* SPI_CFG */;

//
// R99 (0x63) - Mixer Enables
//
pub const WM8962_HPMIXL_ENA: c_uint = 0x0008  /* HPMIXL_ENA */;
pub const WM8962_HPMIXL_ENA_MASK: c_uint = 0x0008  /* HPMIXL_ENA */;

pub const WM8962_HPMIXR_ENA: c_uint = 0x0004  /* HPMIXR_ENA */;
pub const WM8962_HPMIXR_ENA_MASK: c_uint = 0x0004  /* HPMIXR_ENA */;

pub const WM8962_SPKMIXL_ENA: c_uint = 0x0002  /* SPKMIXL_ENA */;
pub const WM8962_SPKMIXL_ENA_MASK: c_uint = 0x0002  /* SPKMIXL_ENA */;

pub const WM8962_SPKMIXR_ENA: c_uint = 0x0001  /* SPKMIXR_ENA */;
pub const WM8962_SPKMIXR_ENA_MASK: c_uint = 0x0001  /* SPKMIXR_ENA */;

//
// R100 (0x64) - Headphone Mixer (1)
//
pub const WM8962_HPMIXL_TO_HPOUTL_PGA: c_uint = 0x0080  /* HPMIXL_TO_HPOUTL_PGA */;
pub const WM8962_HPMIXL_TO_HPOUTL_PGA_MASK: c_uint = 0x0080  /* HPMIXL_TO_HPOUTL_PGA */;

pub const WM8962_DACL_TO_HPMIXL: c_uint = 0x0020  /* DACL_TO_HPMIXL */;
pub const WM8962_DACL_TO_HPMIXL_MASK: c_uint = 0x0020  /* DACL_TO_HPMIXL */;

pub const WM8962_DACR_TO_HPMIXL: c_uint = 0x0010  /* DACR_TO_HPMIXL */;
pub const WM8962_DACR_TO_HPMIXL_MASK: c_uint = 0x0010  /* DACR_TO_HPMIXL */;

pub const WM8962_MIXINL_TO_HPMIXL: c_uint = 0x0008  /* MIXINL_TO_HPMIXL */;
pub const WM8962_MIXINL_TO_HPMIXL_MASK: c_uint = 0x0008  /* MIXINL_TO_HPMIXL */;

pub const WM8962_MIXINR_TO_HPMIXL: c_uint = 0x0004  /* MIXINR_TO_HPMIXL */;
pub const WM8962_MIXINR_TO_HPMIXL_MASK: c_uint = 0x0004  /* MIXINR_TO_HPMIXL */;

pub const WM8962_IN4L_TO_HPMIXL: c_uint = 0x0002  /* IN4L_TO_HPMIXL */;
pub const WM8962_IN4L_TO_HPMIXL_MASK: c_uint = 0x0002  /* IN4L_TO_HPMIXL */;

pub const WM8962_IN4R_TO_HPMIXL: c_uint = 0x0001  /* IN4R_TO_HPMIXL */;
pub const WM8962_IN4R_TO_HPMIXL_MASK: c_uint = 0x0001  /* IN4R_TO_HPMIXL */;

//
// R101 (0x65) - Headphone Mixer (2)
//
pub const WM8962_HPMIXR_TO_HPOUTR_PGA: c_uint = 0x0080  /* HPMIXR_TO_HPOUTR_PGA */;
pub const WM8962_HPMIXR_TO_HPOUTR_PGA_MASK: c_uint = 0x0080  /* HPMIXR_TO_HPOUTR_PGA */;

pub const WM8962_DACL_TO_HPMIXR: c_uint = 0x0020  /* DACL_TO_HPMIXR */;
pub const WM8962_DACL_TO_HPMIXR_MASK: c_uint = 0x0020  /* DACL_TO_HPMIXR */;

pub const WM8962_DACR_TO_HPMIXR: c_uint = 0x0010  /* DACR_TO_HPMIXR */;
pub const WM8962_DACR_TO_HPMIXR_MASK: c_uint = 0x0010  /* DACR_TO_HPMIXR */;

pub const WM8962_MIXINL_TO_HPMIXR: c_uint = 0x0008  /* MIXINL_TO_HPMIXR */;
pub const WM8962_MIXINL_TO_HPMIXR_MASK: c_uint = 0x0008  /* MIXINL_TO_HPMIXR */;

pub const WM8962_MIXINR_TO_HPMIXR: c_uint = 0x0004  /* MIXINR_TO_HPMIXR */;
pub const WM8962_MIXINR_TO_HPMIXR_MASK: c_uint = 0x0004  /* MIXINR_TO_HPMIXR */;

pub const WM8962_IN4L_TO_HPMIXR: c_uint = 0x0002  /* IN4L_TO_HPMIXR */;
pub const WM8962_IN4L_TO_HPMIXR_MASK: c_uint = 0x0002  /* IN4L_TO_HPMIXR */;

pub const WM8962_IN4R_TO_HPMIXR: c_uint = 0x0001  /* IN4R_TO_HPMIXR */;
pub const WM8962_IN4R_TO_HPMIXR_MASK: c_uint = 0x0001  /* IN4R_TO_HPMIXR */;

//
// R102 (0x66) - Headphone Mixer (3)
//
pub const WM8962_HPMIXL_MUTE: c_uint = 0x0100  /* HPMIXL_MUTE */;
pub const WM8962_HPMIXL_MUTE_MASK: c_uint = 0x0100  /* HPMIXL_MUTE */;

pub const WM8962_MIXINL_HPMIXL_VOL: c_uint = 0x0080  /* MIXINL_HPMIXL_VOL */;
pub const WM8962_MIXINL_HPMIXL_VOL_MASK: c_uint = 0x0080  /* MIXINL_HPMIXL_VOL */;

pub const WM8962_MIXINR_HPMIXL_VOL: c_uint = 0x0040  /* MIXINR_HPMIXL_VOL */;
pub const WM8962_MIXINR_HPMIXL_VOL_MASK: c_uint = 0x0040  /* MIXINR_HPMIXL_VOL */;

pub const WM8962_IN4L_HPMIXL_VOL_MASK: c_uint = 0x0038  /* IN4L_HPMIXL_VOL - [5:3] */;

pub const WM8962_IN4R_HPMIXL_VOL_MASK: c_uint = 0x0007  /* IN4R_HPMIXL_VOL - [2:0] */;

//
// R103 (0x67) - Headphone Mixer (4)
//
pub const WM8962_HPMIXR_MUTE: c_uint = 0x0100  /* HPMIXR_MUTE */;
pub const WM8962_HPMIXR_MUTE_MASK: c_uint = 0x0100  /* HPMIXR_MUTE */;

pub const WM8962_MIXINL_HPMIXR_VOL: c_uint = 0x0080  /* MIXINL_HPMIXR_VOL */;
pub const WM8962_MIXINL_HPMIXR_VOL_MASK: c_uint = 0x0080  /* MIXINL_HPMIXR_VOL */;

pub const WM8962_MIXINR_HPMIXR_VOL: c_uint = 0x0040  /* MIXINR_HPMIXR_VOL */;
pub const WM8962_MIXINR_HPMIXR_VOL_MASK: c_uint = 0x0040  /* MIXINR_HPMIXR_VOL */;

pub const WM8962_IN4L_HPMIXR_VOL_MASK: c_uint = 0x0038  /* IN4L_HPMIXR_VOL - [5:3] */;

pub const WM8962_IN4R_HPMIXR_VOL_MASK: c_uint = 0x0007  /* IN4R_HPMIXR_VOL - [2:0] */;

//
// R105 (0x69) - Speaker Mixer (1)
//
pub const WM8962_SPKMIXL_TO_SPKOUTL_PGA: c_uint = 0x0080  /* SPKMIXL_TO_SPKOUTL_PGA */;
pub const WM8962_SPKMIXL_TO_SPKOUTL_PGA_MASK: c_uint = 0x0080  /* SPKMIXL_TO_SPKOUTL_PGA */;

pub const WM8962_DACL_TO_SPKMIXL: c_uint = 0x0020  /* DACL_TO_SPKMIXL */;
pub const WM8962_DACL_TO_SPKMIXL_MASK: c_uint = 0x0020  /* DACL_TO_SPKMIXL */;

pub const WM8962_DACR_TO_SPKMIXL: c_uint = 0x0010  /* DACR_TO_SPKMIXL */;
pub const WM8962_DACR_TO_SPKMIXL_MASK: c_uint = 0x0010  /* DACR_TO_SPKMIXL */;

pub const WM8962_MIXINL_TO_SPKMIXL: c_uint = 0x0008  /* MIXINL_TO_SPKMIXL */;
pub const WM8962_MIXINL_TO_SPKMIXL_MASK: c_uint = 0x0008  /* MIXINL_TO_SPKMIXL */;

pub const WM8962_MIXINR_TO_SPKMIXL: c_uint = 0x0004  /* MIXINR_TO_SPKMIXL */;
pub const WM8962_MIXINR_TO_SPKMIXL_MASK: c_uint = 0x0004  /* MIXINR_TO_SPKMIXL */;

pub const WM8962_IN4L_TO_SPKMIXL: c_uint = 0x0002  /* IN4L_TO_SPKMIXL */;
pub const WM8962_IN4L_TO_SPKMIXL_MASK: c_uint = 0x0002  /* IN4L_TO_SPKMIXL */;

pub const WM8962_IN4R_TO_SPKMIXL: c_uint = 0x0001  /* IN4R_TO_SPKMIXL */;
pub const WM8962_IN4R_TO_SPKMIXL_MASK: c_uint = 0x0001  /* IN4R_TO_SPKMIXL */;

//
// R106 (0x6A) - Speaker Mixer (2)
//
pub const WM8962_SPKMIXR_TO_SPKOUTR_PGA: c_uint = 0x0080  /* SPKMIXR_TO_SPKOUTR_PGA */;
pub const WM8962_SPKMIXR_TO_SPKOUTR_PGA_MASK: c_uint = 0x0080  /* SPKMIXR_TO_SPKOUTR_PGA */;

pub const WM8962_DACL_TO_SPKMIXR: c_uint = 0x0020  /* DACL_TO_SPKMIXR */;
pub const WM8962_DACL_TO_SPKMIXR_MASK: c_uint = 0x0020  /* DACL_TO_SPKMIXR */;

pub const WM8962_DACR_TO_SPKMIXR: c_uint = 0x0010  /* DACR_TO_SPKMIXR */;
pub const WM8962_DACR_TO_SPKMIXR_MASK: c_uint = 0x0010  /* DACR_TO_SPKMIXR */;

pub const WM8962_MIXINL_TO_SPKMIXR: c_uint = 0x0008  /* MIXINL_TO_SPKMIXR */;
pub const WM8962_MIXINL_TO_SPKMIXR_MASK: c_uint = 0x0008  /* MIXINL_TO_SPKMIXR */;

pub const WM8962_MIXINR_TO_SPKMIXR: c_uint = 0x0004  /* MIXINR_TO_SPKMIXR */;
pub const WM8962_MIXINR_TO_SPKMIXR_MASK: c_uint = 0x0004  /* MIXINR_TO_SPKMIXR */;

pub const WM8962_IN4L_TO_SPKMIXR: c_uint = 0x0002  /* IN4L_TO_SPKMIXR */;
pub const WM8962_IN4L_TO_SPKMIXR_MASK: c_uint = 0x0002  /* IN4L_TO_SPKMIXR */;

pub const WM8962_IN4R_TO_SPKMIXR: c_uint = 0x0001  /* IN4R_TO_SPKMIXR */;
pub const WM8962_IN4R_TO_SPKMIXR_MASK: c_uint = 0x0001  /* IN4R_TO_SPKMIXR */;

//
// R107 (0x6B) - Speaker Mixer (3)
//
pub const WM8962_SPKMIXL_MUTE: c_uint = 0x0100  /* SPKMIXL_MUTE */;
pub const WM8962_SPKMIXL_MUTE_MASK: c_uint = 0x0100  /* SPKMIXL_MUTE */;

pub const WM8962_MIXINL_SPKMIXL_VOL: c_uint = 0x0080  /* MIXINL_SPKMIXL_VOL */;
pub const WM8962_MIXINL_SPKMIXL_VOL_MASK: c_uint = 0x0080  /* MIXINL_SPKMIXL_VOL */;

pub const WM8962_MIXINR_SPKMIXL_VOL: c_uint = 0x0040  /* MIXINR_SPKMIXL_VOL */;
pub const WM8962_MIXINR_SPKMIXL_VOL_MASK: c_uint = 0x0040  /* MIXINR_SPKMIXL_VOL */;

pub const WM8962_IN4L_SPKMIXL_VOL_MASK: c_uint = 0x0038  /* IN4L_SPKMIXL_VOL - [5:3] */;

pub const WM8962_IN4R_SPKMIXL_VOL_MASK: c_uint = 0x0007  /* IN4R_SPKMIXL_VOL - [2:0] */;

//
// R108 (0x6C) - Speaker Mixer (4)
//
pub const WM8962_SPKMIXR_MUTE: c_uint = 0x0100  /* SPKMIXR_MUTE */;
pub const WM8962_SPKMIXR_MUTE_MASK: c_uint = 0x0100  /* SPKMIXR_MUTE */;

pub const WM8962_MIXINL_SPKMIXR_VOL: c_uint = 0x0080  /* MIXINL_SPKMIXR_VOL */;
pub const WM8962_MIXINL_SPKMIXR_VOL_MASK: c_uint = 0x0080  /* MIXINL_SPKMIXR_VOL */;

pub const WM8962_MIXINR_SPKMIXR_VOL: c_uint = 0x0040  /* MIXINR_SPKMIXR_VOL */;
pub const WM8962_MIXINR_SPKMIXR_VOL_MASK: c_uint = 0x0040  /* MIXINR_SPKMIXR_VOL */;

pub const WM8962_IN4L_SPKMIXR_VOL_MASK: c_uint = 0x0038  /* IN4L_SPKMIXR_VOL - [5:3] */;

pub const WM8962_IN4R_SPKMIXR_VOL_MASK: c_uint = 0x0007  /* IN4R_SPKMIXR_VOL - [2:0] */;

//
// R109 (0x6D) - Speaker Mixer (5)
//
pub const WM8962_DACL_SPKMIXL_VOL: c_uint = 0x0080  /* DACL_SPKMIXL_VOL */;
pub const WM8962_DACL_SPKMIXL_VOL_MASK: c_uint = 0x0080  /* DACL_SPKMIXL_VOL */;

pub const WM8962_DACR_SPKMIXL_VOL: c_uint = 0x0040  /* DACR_SPKMIXL_VOL */;
pub const WM8962_DACR_SPKMIXL_VOL_MASK: c_uint = 0x0040  /* DACR_SPKMIXL_VOL */;

pub const WM8962_DACL_SPKMIXR_VOL: c_uint = 0x0020  /* DACL_SPKMIXR_VOL */;
pub const WM8962_DACL_SPKMIXR_VOL_MASK: c_uint = 0x0020  /* DACL_SPKMIXR_VOL */;

pub const WM8962_DACR_SPKMIXR_VOL: c_uint = 0x0010  /* DACR_SPKMIXR_VOL */;
pub const WM8962_DACR_SPKMIXR_VOL_MASK: c_uint = 0x0010  /* DACR_SPKMIXR_VOL */;

//
// R110 (0x6E) - Beep Generator (1)
//
pub const WM8962_BEEP_GAIN_MASK: c_uint = 0x00F0  /* BEEP_GAIN - [7:4] */;

pub const WM8962_BEEP_RATE_MASK: c_uint = 0x0006  /* BEEP_RATE - [2:1] */;

pub const WM8962_BEEP_ENA: c_uint = 0x0001  /* BEEP_ENA */;
pub const WM8962_BEEP_ENA_MASK: c_uint = 0x0001  /* BEEP_ENA */;

//
// R115 (0x73) - Oscillator Trim (3)
//
pub const WM8962_OSC_TRIM_XTI_MASK: c_uint = 0x001F  /* OSC_TRIM_XTI - [4:0] */;

//
// R116 (0x74) - Oscillator Trim (4)
//
pub const WM8962_OSC_TRIM_XTO_MASK: c_uint = 0x001F  /* OSC_TRIM_XTO - [4:0] */;

//
// R119 (0x77) - Oscillator Trim (7)
//
pub const WM8962_XTO_CAP_SEL_MASK: c_uint = 0x00F0  /* XTO_CAP_SEL - [7:4] */;

pub const WM8962_XTI_CAP_SEL_MASK: c_uint = 0x000F  /* XTI_CAP_SEL - [3:0] */;

//
// R124 (0x7C) - Analogue Clocking1
//
pub const WM8962_CLKOUT2_SEL_MASK: c_uint = 0x0060  /* CLKOUT2_SEL - [6:5] */;

pub const WM8962_CLKOUT3_SEL_MASK: c_uint = 0x0018  /* CLKOUT3_SEL - [4:3] */;

pub const WM8962_CLKOUT5_SEL: c_uint = 0x0001  /* CLKOUT5_SEL */;
pub const WM8962_CLKOUT5_SEL_MASK: c_uint = 0x0001  /* CLKOUT5_SEL */;

//
// R125 (0x7D) - Analogue Clocking2
//
pub const WM8962_PLL2_OUTDIV: c_uint = 0x0080  /* PLL2_OUTDIV */;
pub const WM8962_PLL2_OUTDIV_MASK: c_uint = 0x0080  /* PLL2_OUTDIV */;

pub const WM8962_PLL3_OUTDIV: c_uint = 0x0040  /* PLL3_OUTDIV */;
pub const WM8962_PLL3_OUTDIV_MASK: c_uint = 0x0040  /* PLL3_OUTDIV */;

pub const WM8962_PLL_SYSCLK_DIV_MASK: c_uint = 0x0018  /* PLL_SYSCLK_DIV - [4:3] */;

pub const WM8962_CLKOUT3_DIV: c_uint = 0x0004  /* CLKOUT3_DIV */;
pub const WM8962_CLKOUT3_DIV_MASK: c_uint = 0x0004  /* CLKOUT3_DIV */;

pub const WM8962_CLKOUT2_DIV: c_uint = 0x0002  /* CLKOUT2_DIV */;
pub const WM8962_CLKOUT2_DIV_MASK: c_uint = 0x0002  /* CLKOUT2_DIV */;

pub const WM8962_CLKOUT5_DIV: c_uint = 0x0001  /* CLKOUT5_DIV */;
pub const WM8962_CLKOUT5_DIV_MASK: c_uint = 0x0001  /* CLKOUT5_DIV */;

//
// R126 (0x7E) - Analogue Clocking3
//
pub const WM8962_CLKOUT2_OE: c_uint = 0x0008  /* CLKOUT2_OE */;
pub const WM8962_CLKOUT2_OE_MASK: c_uint = 0x0008  /* CLKOUT2_OE */;

pub const WM8962_CLKOUT3_OE: c_uint = 0x0004  /* CLKOUT3_OE */;
pub const WM8962_CLKOUT3_OE_MASK: c_uint = 0x0004  /* CLKOUT3_OE */;

pub const WM8962_CLKOUT5_OE: c_uint = 0x0001  /* CLKOUT5_OE */;
pub const WM8962_CLKOUT5_OE_MASK: c_uint = 0x0001  /* CLKOUT5_OE */;

//
// R127 (0x7F) - PLL Software Reset
//
pub const WM8962_SW_RESET_PLL_MASK: c_uint = 0xFFFF  /* SW_RESET_PLL - [15:0] */;

//
// R129 (0x81) - PLL2
//
pub const WM8962_OSC_ENA: c_uint = 0x0080  /* OSC_ENA */;
pub const WM8962_OSC_ENA_MASK: c_uint = 0x0080  /* OSC_ENA */;

pub const WM8962_PLL2_ENA: c_uint = 0x0020  /* PLL2_ENA */;
pub const WM8962_PLL2_ENA_MASK: c_uint = 0x0020  /* PLL2_ENA */;

pub const WM8962_PLL3_ENA: c_uint = 0x0010  /* PLL3_ENA */;
pub const WM8962_PLL3_ENA_MASK: c_uint = 0x0010  /* PLL3_ENA */;

//
// R131 (0x83) - PLL 4
//
pub const WM8962_PLL_CLK_SRC: c_uint = 0x0002  /* PLL_CLK_SRC */;
pub const WM8962_PLL_CLK_SRC_MASK: c_uint = 0x0002  /* PLL_CLK_SRC */;

pub const WM8962_FLL_TO_PLL3: c_uint = 0x0001  /* FLL_TO_PLL3 */;
pub const WM8962_FLL_TO_PLL3_MASK: c_uint = 0x0001  /* FLL_TO_PLL3 */;

//
// R136 (0x88) - PLL 9
//
pub const WM8962_PLL2_FRAC: c_uint = 0x0040  /* PLL2_FRAC */;
pub const WM8962_PLL2_FRAC_MASK: c_uint = 0x0040  /* PLL2_FRAC */;

pub const WM8962_PLL2_N_MASK: c_uint = 0x001F  /* PLL2_N - [4:0] */;

//
// R137 (0x89) - PLL 10
//
pub const WM8962_PLL2_K_MASK: c_uint = 0x00FF  /* PLL2_K - [7:0] */;

//
// R138 (0x8A) - PLL 11
//
pub const WM8962_PLL2_K_MASK: c_uint = 0x00FF  /* PLL2_K - [7:0] */;

//
// R139 (0x8B) - PLL 12
//
pub const WM8962_PLL2_K_MASK: c_uint = 0x00FF  /* PLL2_K - [7:0] */;

//
// R140 (0x8C) - PLL 13
//
pub const WM8962_PLL3_FRAC: c_uint = 0x0040  /* PLL3_FRAC */;
pub const WM8962_PLL3_FRAC_MASK: c_uint = 0x0040  /* PLL3_FRAC */;

pub const WM8962_PLL3_N_MASK: c_uint = 0x001F  /* PLL3_N - [4:0] */;

//
// R141 (0x8D) - PLL 14
//
pub const WM8962_PLL3_K_MASK: c_uint = 0x00FF  /* PLL3_K - [7:0] */;

//
// R142 (0x8E) - PLL 15
//
pub const WM8962_PLL3_K_MASK: c_uint = 0x00FF  /* PLL3_K - [7:0] */;

//
// R143 (0x8F) - PLL 16
//
pub const WM8962_PLL3_K_MASK: c_uint = 0x00FF  /* PLL3_K - [7:0] */;

//
// R155 (0x9B) - FLL Control (1)
//
pub const WM8962_FLL_REFCLK_SRC_MASK: c_uint = 0x0060  /* FLL_REFCLK_SRC - [6:5] */;

pub const WM8962_FLL_FRAC: c_uint = 0x0004  /* FLL_FRAC */;
pub const WM8962_FLL_FRAC_MASK: c_uint = 0x0004  /* FLL_FRAC */;

pub const WM8962_FLL_OSC_ENA: c_uint = 0x0002  /* FLL_OSC_ENA */;
pub const WM8962_FLL_OSC_ENA_MASK: c_uint = 0x0002  /* FLL_OSC_ENA */;

pub const WM8962_FLL_ENA: c_uint = 0x0001  /* FLL_ENA */;
pub const WM8962_FLL_ENA_MASK: c_uint = 0x0001  /* FLL_ENA */;

//
// R156 (0x9C) - FLL Control (2)
//
pub const WM8962_FLL_OUTDIV_MASK: c_uint = 0x01F8  /* FLL_OUTDIV - [8:3] */;

pub const WM8962_FLL_REFCLK_DIV_MASK: c_uint = 0x0003  /* FLL_REFCLK_DIV - [1:0] */;

//
// R157 (0x9D) - FLL Control (3)
//
pub const WM8962_FLL_FRATIO_MASK: c_uint = 0x0007  /* FLL_FRATIO - [2:0] */;

//
// R159 (0x9F) - FLL Control (5)
//
pub const WM8962_FLL_FRC_NCO_VAL_MASK: c_uint = 0x007E  /* FLL_FRC_NCO_VAL - [6:1] */;

pub const WM8962_FLL_FRC_NCO: c_uint = 0x0001  /* FLL_FRC_NCO */;
pub const WM8962_FLL_FRC_NCO_MASK: c_uint = 0x0001  /* FLL_FRC_NCO */;

//
// R160 (0xA0) - FLL Control (6)
//
pub const WM8962_FLL_THETA_MASK: c_uint = 0xFFFF  /* FLL_THETA - [15:0] */;

//
// R161 (0xA1) - FLL Control (7)
//
pub const WM8962_FLL_LAMBDA_MASK: c_uint = 0xFFFF  /* FLL_LAMBDA - [15:0] */;

//
// R162 (0xA2) - FLL Control (8)
//
pub const WM8962_FLL_N_MASK: c_uint = 0x03FF  /* FLL_N - [9:0] */;

//
// R252 (0xFC) - General test 1
//
pub const WM8962_REG_SYNC: c_uint = 0x0004  /* REG_SYNC */;
pub const WM8962_REG_SYNC_MASK: c_uint = 0x0004  /* REG_SYNC */;

pub const WM8962_AUTO_INC: c_uint = 0x0001  /* AUTO_INC */;
pub const WM8962_AUTO_INC_MASK: c_uint = 0x0001  /* AUTO_INC */;

//
// R256 (0x100) - DF1
//
pub const WM8962_DRC_DF1_ENA: c_uint = 0x0008  /* DRC_DF1_ENA */;
pub const WM8962_DRC_DF1_ENA_MASK: c_uint = 0x0008  /* DRC_DF1_ENA */;

pub const WM8962_DF1_SHARED_COEFF: c_uint = 0x0004  /* DF1_SHARED_COEFF */;
pub const WM8962_DF1_SHARED_COEFF_MASK: c_uint = 0x0004  /* DF1_SHARED_COEFF */;

pub const WM8962_DF1_SHARED_COEFF_SEL: c_uint = 0x0002  /* DF1_SHARED_COEFF_SEL */;
pub const WM8962_DF1_SHARED_COEFF_SEL_MASK: c_uint = 0x0002  /* DF1_SHARED_COEFF_SEL */;

pub const WM8962_DF1_ENA: c_uint = 0x0001  /* DF1_ENA */;
pub const WM8962_DF1_ENA_MASK: c_uint = 0x0001  /* DF1_ENA */;

//
// R257 (0x101) - DF2
//
pub const WM8962_DF1_COEFF_L0_MASK: c_uint = 0xFFFF  /* DF1_COEFF_L0 - [15:0] */;

//
// R258 (0x102) - DF3
//
pub const WM8962_DF1_COEFF_L1_MASK: c_uint = 0xFFFF  /* DF1_COEFF_L1 - [15:0] */;

//
// R259 (0x103) - DF4
//
pub const WM8962_DF1_COEFF_L2_MASK: c_uint = 0xFFFF  /* DF1_COEFF_L2 - [15:0] */;

//
// R260 (0x104) - DF5
//
pub const WM8962_DF1_COEFF_R0_MASK: c_uint = 0xFFFF  /* DF1_COEFF_R0 - [15:0] */;

//
// R261 (0x105) - DF6
//
pub const WM8962_DF1_COEFF_R1_MASK: c_uint = 0xFFFF  /* DF1_COEFF_R1 - [15:0] */;

//
// R262 (0x106) - DF7
//
pub const WM8962_DF1_COEFF_R2_MASK: c_uint = 0xFFFF  /* DF1_COEFF_R2 - [15:0] */;

//
// R264 (0x108) - LHPF1
//
pub const WM8962_LHPF_MODE: c_uint = 0x0002  /* LHPF_MODE */;
pub const WM8962_LHPF_MODE_MASK: c_uint = 0x0002  /* LHPF_MODE */;

pub const WM8962_LHPF_ENA: c_uint = 0x0001  /* LHPF_ENA */;
pub const WM8962_LHPF_ENA_MASK: c_uint = 0x0001  /* LHPF_ENA */;

//
// R265 (0x109) - LHPF2
//
pub const WM8962_LHPF_COEFF_MASK: c_uint = 0xFFFF  /* LHPF_COEFF - [15:0] */;

//
// R268 (0x10C) - THREED1
//
pub const WM8962_ADC_MONOMIX: c_uint = 0x0040  /* ADC_MONOMIX */;
pub const WM8962_ADC_MONOMIX_MASK: c_uint = 0x0040  /* ADC_MONOMIX */;

pub const WM8962_THREED_SIGN_L: c_uint = 0x0020  /* THREED_SIGN_L */;
pub const WM8962_THREED_SIGN_L_MASK: c_uint = 0x0020  /* THREED_SIGN_L */;

pub const WM8962_THREED_SIGN_R: c_uint = 0x0010  /* THREED_SIGN_R */;
pub const WM8962_THREED_SIGN_R_MASK: c_uint = 0x0010  /* THREED_SIGN_R */;

pub const WM8962_THREED_LHPF_MODE: c_uint = 0x0004  /* THREED_LHPF_MODE */;
pub const WM8962_THREED_LHPF_MODE_MASK: c_uint = 0x0004  /* THREED_LHPF_MODE */;

pub const WM8962_THREED_LHPF_ENA: c_uint = 0x0002  /* THREED_LHPF_ENA */;
pub const WM8962_THREED_LHPF_ENA_MASK: c_uint = 0x0002  /* THREED_LHPF_ENA */;

pub const WM8962_THREED_ENA: c_uint = 0x0001  /* THREED_ENA */;
pub const WM8962_THREED_ENA_MASK: c_uint = 0x0001  /* THREED_ENA */;

//
// R269 (0x10D) - THREED2
//
pub const WM8962_THREED_FGAINL_MASK: c_uint = 0xF800  /* THREED_FGAINL - [15:11] */;

pub const WM8962_THREED_CGAINL_MASK: c_uint = 0x07C0  /* THREED_CGAINL - [10:6] */;

pub const WM8962_THREED_DELAYL_MASK: c_uint = 0x003C  /* THREED_DELAYL - [5:2] */;

//
// R270 (0x10E) - THREED3
//
pub const WM8962_THREED_LHPF_COEFF_MASK: c_uint = 0xFFFF  /* THREED_LHPF_COEFF - [15:0] */;

//
// R271 (0x10F) - THREED4
//
pub const WM8962_THREED_FGAINR_MASK: c_uint = 0xF800  /* THREED_FGAINR - [15:11] */;

pub const WM8962_THREED_CGAINR_MASK: c_uint = 0x07C0  /* THREED_CGAINR - [10:6] */;

pub const WM8962_THREED_DELAYR_MASK: c_uint = 0x003C  /* THREED_DELAYR - [5:2] */;

//
// R276 (0x114) - DRC 1
//
pub const WM8962_DRC_SIG_DET_RMS_MASK: c_uint = 0x7C00  /* DRC_SIG_DET_RMS - [14:10] */;

pub const WM8962_DRC_SIG_DET_PK_MASK: c_uint = 0x0300  /* DRC_SIG_DET_PK - [9:8] */;

pub const WM8962_DRC_NG_ENA: c_uint = 0x0080  /* DRC_NG_ENA */;
pub const WM8962_DRC_NG_ENA_MASK: c_uint = 0x0080  /* DRC_NG_ENA */;

pub const WM8962_DRC_SIG_DET_MODE: c_uint = 0x0040  /* DRC_SIG_DET_MODE */;
pub const WM8962_DRC_SIG_DET_MODE_MASK: c_uint = 0x0040  /* DRC_SIG_DET_MODE */;

pub const WM8962_DRC_SIG_DET: c_uint = 0x0020  /* DRC_SIG_DET */;
pub const WM8962_DRC_SIG_DET_MASK: c_uint = 0x0020  /* DRC_SIG_DET */;

pub const WM8962_DRC_KNEE2_OP_ENA: c_uint = 0x0010  /* DRC_KNEE2_OP_ENA */;
pub const WM8962_DRC_KNEE2_OP_ENA_MASK: c_uint = 0x0010  /* DRC_KNEE2_OP_ENA */;

pub const WM8962_DRC_QR: c_uint = 0x0008  /* DRC_QR */;
pub const WM8962_DRC_QR_MASK: c_uint = 0x0008  /* DRC_QR */;

pub const WM8962_DRC_ANTICLIP: c_uint = 0x0004  /* DRC_ANTICLIP */;
pub const WM8962_DRC_ANTICLIP_MASK: c_uint = 0x0004  /* DRC_ANTICLIP */;

pub const WM8962_DRC_MODE: c_uint = 0x0002  /* DRC_MODE */;
pub const WM8962_DRC_MODE_MASK: c_uint = 0x0002  /* DRC_MODE */;

pub const WM8962_DRC_ENA: c_uint = 0x0001  /* DRC_ENA */;
pub const WM8962_DRC_ENA_MASK: c_uint = 0x0001  /* DRC_ENA */;

//
// R277 (0x115) - DRC 2
//
pub const WM8962_DRC_ATK_MASK: c_uint = 0x1E00  /* DRC_ATK - [12:9] */;

pub const WM8962_DRC_DCY_MASK: c_uint = 0x01E0  /* DRC_DCY - [8:5] */;

pub const WM8962_DRC_MINGAIN_MASK: c_uint = 0x001C  /* DRC_MINGAIN - [4:2] */;

pub const WM8962_DRC_MAXGAIN_MASK: c_uint = 0x0003  /* DRC_MAXGAIN - [1:0] */;

//
// R278 (0x116) - DRC 3
//
pub const WM8962_DRC_NG_MINGAIN_MASK: c_uint = 0xF000  /* DRC_NG_MINGAIN - [15:12] */;

pub const WM8962_DRC_QR_THR_MASK: c_uint = 0x0C00  /* DRC_QR_THR - [11:10] */;

pub const WM8962_DRC_QR_DCY_MASK: c_uint = 0x0300  /* DRC_QR_DCY - [9:8] */;

pub const WM8962_DRC_NG_EXP_MASK: c_uint = 0x00C0  /* DRC_NG_EXP - [7:6] */;

pub const WM8962_DRC_HI_COMP_MASK: c_uint = 0x0038  /* DRC_HI_COMP - [5:3] */;

pub const WM8962_DRC_LO_COMP_MASK: c_uint = 0x0007  /* DRC_LO_COMP - [2:0] */;

//
// R279 (0x117) - DRC 4
//
pub const WM8962_DRC_KNEE_IP_MASK: c_uint = 0x07E0  /* DRC_KNEE_IP - [10:5] */;

pub const WM8962_DRC_KNEE_OP_MASK: c_uint = 0x001F  /* DRC_KNEE_OP - [4:0] */;

//
// R280 (0x118) - DRC 5
//
pub const WM8962_DRC_KNEE2_IP_MASK: c_uint = 0x03E0  /* DRC_KNEE2_IP - [9:5] */;

pub const WM8962_DRC_KNEE2_OP_MASK: c_uint = 0x001F  /* DRC_KNEE2_OP - [4:0] */;

//
// R285 (0x11D) - Tloopback
//
pub const WM8962_TLB_ENA: c_uint = 0x0002  /* TLB_ENA */;
pub const WM8962_TLB_ENA_MASK: c_uint = 0x0002  /* TLB_ENA */;

pub const WM8962_TLB_MODE: c_uint = 0x0001  /* TLB_MODE */;
pub const WM8962_TLB_MODE_MASK: c_uint = 0x0001  /* TLB_MODE */;

//
// R335 (0x14F) - EQ1
//
pub const WM8962_EQ_SHARED_COEFF: c_uint = 0x0004  /* EQ_SHARED_COEFF */;
pub const WM8962_EQ_SHARED_COEFF_MASK: c_uint = 0x0004  /* EQ_SHARED_COEFF */;

pub const WM8962_EQ_SHARED_COEFF_SEL: c_uint = 0x0002  /* EQ_SHARED_COEFF_SEL */;
pub const WM8962_EQ_SHARED_COEFF_SEL_MASK: c_uint = 0x0002  /* EQ_SHARED_COEFF_SEL */;

pub const WM8962_EQ_ENA: c_uint = 0x0001  /* EQ_ENA */;
pub const WM8962_EQ_ENA_MASK: c_uint = 0x0001  /* EQ_ENA */;

//
// R336 (0x150) - EQ2
//
pub const WM8962_EQL_B1_GAIN_MASK: c_uint = 0xF800  /* EQL_B1_GAIN - [15:11] */;

pub const WM8962_EQL_B2_GAIN_MASK: c_uint = 0x07C0  /* EQL_B2_GAIN - [10:6] */;

pub const WM8962_EQL_B3_GAIN_MASK: c_uint = 0x003E  /* EQL_B3_GAIN - [5:1] */;

//
// R337 (0x151) - EQ3
//
pub const WM8962_EQL_B4_GAIN_MASK: c_uint = 0xF800  /* EQL_B4_GAIN - [15:11] */;

pub const WM8962_EQL_B5_GAIN_MASK: c_uint = 0x07C0  /* EQL_B5_GAIN - [10:6] */;

//
// R338 (0x152) - EQ4
//
pub const WM8962_EQL_B1_A_MASK: c_uint = 0xFFFF  /* EQL_B1_A - [15:0] */;

//
// R339 (0x153) - EQ5
//
pub const WM8962_EQL_B1_B_MASK: c_uint = 0xFFFF  /* EQL_B1_B - [15:0] */;

//
// R340 (0x154) - EQ6
//
pub const WM8962_EQL_B1_PG_MASK: c_uint = 0xFFFF  /* EQL_B1_PG - [15:0] */;

//
// R341 (0x155) - EQ7
//
pub const WM8962_EQL_B2_A_MASK: c_uint = 0xFFFF  /* EQL_B2_A - [15:0] */;

//
// R342 (0x156) - EQ8
//
pub const WM8962_EQL_B2_B_MASK: c_uint = 0xFFFF  /* EQL_B2_B - [15:0] */;

//
// R343 (0x157) - EQ9
//
pub const WM8962_EQL_B2_C_MASK: c_uint = 0xFFFF  /* EQL_B2_C - [15:0] */;

//
// R344 (0x158) - EQ10
//
pub const WM8962_EQL_B2_PG_MASK: c_uint = 0xFFFF  /* EQL_B2_PG - [15:0] */;

//
// R345 (0x159) - EQ11
//
pub const WM8962_EQL_B3_A_MASK: c_uint = 0xFFFF  /* EQL_B3_A - [15:0] */;

//
// R346 (0x15A) - EQ12
//
pub const WM8962_EQL_B3_B_MASK: c_uint = 0xFFFF  /* EQL_B3_B - [15:0] */;

//
// R347 (0x15B) - EQ13
//
pub const WM8962_EQL_B3_C_MASK: c_uint = 0xFFFF  /* EQL_B3_C - [15:0] */;

//
// R348 (0x15C) - EQ14
//
pub const WM8962_EQL_B3_PG_MASK: c_uint = 0xFFFF  /* EQL_B3_PG - [15:0] */;

//
// R349 (0x15D) - EQ15
//
pub const WM8962_EQL_B4_A_MASK: c_uint = 0xFFFF  /* EQL_B4_A - [15:0] */;

//
// R350 (0x15E) - EQ16
//
pub const WM8962_EQL_B4_B_MASK: c_uint = 0xFFFF  /* EQL_B4_B - [15:0] */;

//
// R351 (0x15F) - EQ17
//
pub const WM8962_EQL_B4_C_MASK: c_uint = 0xFFFF  /* EQL_B4_C - [15:0] */;

//
// R352 (0x160) - EQ18
//
pub const WM8962_EQL_B4_PG_MASK: c_uint = 0xFFFF  /* EQL_B4_PG - [15:0] */;

//
// R353 (0x161) - EQ19
//
pub const WM8962_EQL_B5_A_MASK: c_uint = 0xFFFF  /* EQL_B5_A - [15:0] */;

//
// R354 (0x162) - EQ20
//
pub const WM8962_EQL_B5_B_MASK: c_uint = 0xFFFF  /* EQL_B5_B - [15:0] */;

//
// R355 (0x163) - EQ21
//
pub const WM8962_EQL_B5_PG_MASK: c_uint = 0xFFFF  /* EQL_B5_PG - [15:0] */;

//
// R356 (0x164) - EQ22
//
pub const WM8962_EQR_B1_GAIN_MASK: c_uint = 0xF800  /* EQR_B1_GAIN - [15:11] */;

pub const WM8962_EQR_B2_GAIN_MASK: c_uint = 0x07C0  /* EQR_B2_GAIN - [10:6] */;

pub const WM8962_EQR_B3_GAIN_MASK: c_uint = 0x003E  /* EQR_B3_GAIN - [5:1] */;

//
// R357 (0x165) - EQ23
//
pub const WM8962_EQR_B4_GAIN_MASK: c_uint = 0xF800  /* EQR_B4_GAIN - [15:11] */;

pub const WM8962_EQR_B5_GAIN_MASK: c_uint = 0x07C0  /* EQR_B5_GAIN - [10:6] */;

//
// R358 (0x166) - EQ24
//
pub const WM8962_EQR_B1_A_MASK: c_uint = 0xFFFF  /* EQR_B1_A - [15:0] */;

//
// R359 (0x167) - EQ25
//
pub const WM8962_EQR_B1_B_MASK: c_uint = 0xFFFF  /* EQR_B1_B - [15:0] */;

//
// R360 (0x168) - EQ26
//
pub const WM8962_EQR_B1_PG_MASK: c_uint = 0xFFFF  /* EQR_B1_PG - [15:0] */;

//
// R361 (0x169) - EQ27
//
pub const WM8962_EQR_B2_A_MASK: c_uint = 0xFFFF  /* EQR_B2_A - [15:0] */;

//
// R362 (0x16A) - EQ28
//
pub const WM8962_EQR_B2_B_MASK: c_uint = 0xFFFF  /* EQR_B2_B - [15:0] */;

//
// R363 (0x16B) - EQ29
//
pub const WM8962_EQR_B2_C_MASK: c_uint = 0xFFFF  /* EQR_B2_C - [15:0] */;

//
// R364 (0x16C) - EQ30
//
pub const WM8962_EQR_B2_PG_MASK: c_uint = 0xFFFF  /* EQR_B2_PG - [15:0] */;

//
// R365 (0x16D) - EQ31
//
pub const WM8962_EQR_B3_A_MASK: c_uint = 0xFFFF  /* EQR_B3_A - [15:0] */;

//
// R366 (0x16E) - EQ32
//
pub const WM8962_EQR_B3_B_MASK: c_uint = 0xFFFF  /* EQR_B3_B - [15:0] */;

//
// R367 (0x16F) - EQ33
//
pub const WM8962_EQR_B3_C_MASK: c_uint = 0xFFFF  /* EQR_B3_C - [15:0] */;

//
// R368 (0x170) - EQ34
//
pub const WM8962_EQR_B3_PG_MASK: c_uint = 0xFFFF  /* EQR_B3_PG - [15:0] */;

//
// R369 (0x171) - EQ35
//
pub const WM8962_EQR_B4_A_MASK: c_uint = 0xFFFF  /* EQR_B4_A - [15:0] */;

//
// R370 (0x172) - EQ36
//
pub const WM8962_EQR_B4_B_MASK: c_uint = 0xFFFF  /* EQR_B4_B - [15:0] */;

//
// R371 (0x173) - EQ37
//
pub const WM8962_EQR_B4_C_MASK: c_uint = 0xFFFF  /* EQR_B4_C - [15:0] */;

//
// R372 (0x174) - EQ38
//
pub const WM8962_EQR_B4_PG_MASK: c_uint = 0xFFFF  /* EQR_B4_PG - [15:0] */;

//
// R373 (0x175) - EQ39
//
pub const WM8962_EQR_B5_A_MASK: c_uint = 0xFFFF  /* EQR_B5_A - [15:0] */;

//
// R374 (0x176) - EQ40
//
pub const WM8962_EQR_B5_B_MASK: c_uint = 0xFFFF  /* EQR_B5_B - [15:0] */;

//
// R375 (0x177) - EQ41
//
pub const WM8962_EQR_B5_PG_MASK: c_uint = 0xFFFF  /* EQR_B5_PG - [15:0] */;

//
// R513 (0x201) - GPIO 2
//
pub const WM8962_GP2_POL: c_uint = 0x0400  /* GP2_POL */;
pub const WM8962_GP2_POL_MASK: c_uint = 0x0400  /* GP2_POL */;

pub const WM8962_GP2_LVL: c_uint = 0x0040  /* GP2_LVL */;
pub const WM8962_GP2_LVL_MASK: c_uint = 0x0040  /* GP2_LVL */;

pub const WM8962_GP2_FN_MASK: c_uint = 0x001F  /* GP2_FN - [4:0] */;

//
// R514 (0x202) - GPIO 3
//
pub const WM8962_GP3_POL: c_uint = 0x0400  /* GP3_POL */;
pub const WM8962_GP3_POL_MASK: c_uint = 0x0400  /* GP3_POL */;

pub const WM8962_GP3_LVL: c_uint = 0x0040  /* GP3_LVL */;
pub const WM8962_GP3_LVL_MASK: c_uint = 0x0040  /* GP3_LVL */;

pub const WM8962_GP3_FN_MASK: c_uint = 0x001F  /* GP3_FN - [4:0] */;

//
// R516 (0x204) - GPIO 5
//
pub const WM8962_GP5_DIR: c_uint = 0x8000  /* GP5_DIR */;
pub const WM8962_GP5_DIR_MASK: c_uint = 0x8000  /* GP5_DIR */;

pub const WM8962_GP5_PU: c_uint = 0x4000  /* GP5_PU */;
pub const WM8962_GP5_PU_MASK: c_uint = 0x4000  /* GP5_PU */;

pub const WM8962_GP5_PD: c_uint = 0x2000  /* GP5_PD */;
pub const WM8962_GP5_PD_MASK: c_uint = 0x2000  /* GP5_PD */;

pub const WM8962_GP5_POL: c_uint = 0x0400  /* GP5_POL */;
pub const WM8962_GP5_POL_MASK: c_uint = 0x0400  /* GP5_POL */;

pub const WM8962_GP5_OP_CFG: c_uint = 0x0200  /* GP5_OP_CFG */;
pub const WM8962_GP5_OP_CFG_MASK: c_uint = 0x0200  /* GP5_OP_CFG */;

pub const WM8962_GP5_DB: c_uint = 0x0100  /* GP5_DB */;
pub const WM8962_GP5_DB_MASK: c_uint = 0x0100  /* GP5_DB */;

pub const WM8962_GP5_LVL: c_uint = 0x0040  /* GP5_LVL */;
pub const WM8962_GP5_LVL_MASK: c_uint = 0x0040  /* GP5_LVL */;

pub const WM8962_GP5_FN_MASK: c_uint = 0x001F  /* GP5_FN - [4:0] */;

//
// R517 (0x205) - GPIO 6
//
pub const WM8962_GP6_DIR: c_uint = 0x8000  /* GP6_DIR */;
pub const WM8962_GP6_DIR_MASK: c_uint = 0x8000  /* GP6_DIR */;

pub const WM8962_GP6_PU: c_uint = 0x4000  /* GP6_PU */;
pub const WM8962_GP6_PU_MASK: c_uint = 0x4000  /* GP6_PU */;

pub const WM8962_GP6_PD: c_uint = 0x2000  /* GP6_PD */;
pub const WM8962_GP6_PD_MASK: c_uint = 0x2000  /* GP6_PD */;

pub const WM8962_GP6_POL: c_uint = 0x0400  /* GP6_POL */;
pub const WM8962_GP6_POL_MASK: c_uint = 0x0400  /* GP6_POL */;

pub const WM8962_GP6_OP_CFG: c_uint = 0x0200  /* GP6_OP_CFG */;
pub const WM8962_GP6_OP_CFG_MASK: c_uint = 0x0200  /* GP6_OP_CFG */;

pub const WM8962_GP6_DB: c_uint = 0x0100  /* GP6_DB */;
pub const WM8962_GP6_DB_MASK: c_uint = 0x0100  /* GP6_DB */;

pub const WM8962_GP6_LVL: c_uint = 0x0040  /* GP6_LVL */;
pub const WM8962_GP6_LVL_MASK: c_uint = 0x0040  /* GP6_LVL */;

pub const WM8962_GP6_FN_MASK: c_uint = 0x001F  /* GP6_FN - [4:0] */;

//
// R560 (0x230) - Interrupt Status 1
//
pub const WM8962_GP6_EINT: c_uint = 0x0020  /* GP6_EINT */;
pub const WM8962_GP6_EINT_MASK: c_uint = 0x0020  /* GP6_EINT */;

pub const WM8962_GP5_EINT: c_uint = 0x0010  /* GP5_EINT */;
pub const WM8962_GP5_EINT_MASK: c_uint = 0x0010  /* GP5_EINT */;

//
// R561 (0x231) - Interrupt Status 2
//
pub const WM8962_MICSCD_EINT: c_uint = 0x8000  /* MICSCD_EINT */;
pub const WM8962_MICSCD_EINT_MASK: c_uint = 0x8000  /* MICSCD_EINT */;

pub const WM8962_MICD_EINT: c_uint = 0x4000  /* MICD_EINT */;
pub const WM8962_MICD_EINT_MASK: c_uint = 0x4000  /* MICD_EINT */;

pub const WM8962_FIFOS_ERR_EINT: c_uint = 0x2000  /* FIFOS_ERR_EINT */;
pub const WM8962_FIFOS_ERR_EINT_MASK: c_uint = 0x2000  /* FIFOS_ERR_EINT */;

pub const WM8962_ALC_LOCK_EINT: c_uint = 0x1000  /* ALC_LOCK_EINT */;
pub const WM8962_ALC_LOCK_EINT_MASK: c_uint = 0x1000  /* ALC_LOCK_EINT */;

pub const WM8962_ALC_THRESH_EINT: c_uint = 0x0800  /* ALC_THRESH_EINT */;
pub const WM8962_ALC_THRESH_EINT_MASK: c_uint = 0x0800  /* ALC_THRESH_EINT */;

pub const WM8962_ALC_SAT_EINT: c_uint = 0x0400  /* ALC_SAT_EINT */;
pub const WM8962_ALC_SAT_EINT_MASK: c_uint = 0x0400  /* ALC_SAT_EINT */;

pub const WM8962_ALC_PKOVR_EINT: c_uint = 0x0200  /* ALC_PKOVR_EINT */;
pub const WM8962_ALC_PKOVR_EINT_MASK: c_uint = 0x0200  /* ALC_PKOVR_EINT */;

pub const WM8962_ALC_NGATE_EINT: c_uint = 0x0100  /* ALC_NGATE_EINT */;
pub const WM8962_ALC_NGATE_EINT_MASK: c_uint = 0x0100  /* ALC_NGATE_EINT */;

pub const WM8962_WSEQ_DONE_EINT: c_uint = 0x0080  /* WSEQ_DONE_EINT */;
pub const WM8962_WSEQ_DONE_EINT_MASK: c_uint = 0x0080  /* WSEQ_DONE_EINT */;

pub const WM8962_DRC_ACTDET_EINT: c_uint = 0x0040  /* DRC_ACTDET_EINT */;
pub const WM8962_DRC_ACTDET_EINT_MASK: c_uint = 0x0040  /* DRC_ACTDET_EINT */;

pub const WM8962_FLL_LOCK_EINT: c_uint = 0x0020  /* FLL_LOCK_EINT */;
pub const WM8962_FLL_LOCK_EINT_MASK: c_uint = 0x0020  /* FLL_LOCK_EINT */;

pub const WM8962_PLL3_LOCK_EINT: c_uint = 0x0008  /* PLL3_LOCK_EINT */;
pub const WM8962_PLL3_LOCK_EINT_MASK: c_uint = 0x0008  /* PLL3_LOCK_EINT */;

pub const WM8962_PLL2_LOCK_EINT: c_uint = 0x0004  /* PLL2_LOCK_EINT */;
pub const WM8962_PLL2_LOCK_EINT_MASK: c_uint = 0x0004  /* PLL2_LOCK_EINT */;

pub const WM8962_TEMP_SHUT_EINT: c_uint = 0x0001  /* TEMP_SHUT_EINT */;
pub const WM8962_TEMP_SHUT_EINT_MASK: c_uint = 0x0001  /* TEMP_SHUT_EINT */;

//
// R568 (0x238) - Interrupt Status 1 Mask
//
pub const WM8962_IM_GP6_EINT: c_uint = 0x0020  /* IM_GP6_EINT */;
pub const WM8962_IM_GP6_EINT_MASK: c_uint = 0x0020  /* IM_GP6_EINT */;

pub const WM8962_IM_GP5_EINT: c_uint = 0x0010  /* IM_GP5_EINT */;
pub const WM8962_IM_GP5_EINT_MASK: c_uint = 0x0010  /* IM_GP5_EINT */;

//
// R569 (0x239) - Interrupt Status 2 Mask
//
pub const WM8962_IM_MICSCD_EINT: c_uint = 0x8000  /* IM_MICSCD_EINT */;
pub const WM8962_IM_MICSCD_EINT_MASK: c_uint = 0x8000  /* IM_MICSCD_EINT */;

pub const WM8962_IM_MICD_EINT: c_uint = 0x4000  /* IM_MICD_EINT */;
pub const WM8962_IM_MICD_EINT_MASK: c_uint = 0x4000  /* IM_MICD_EINT */;

pub const WM8962_IM_FIFOS_ERR_EINT: c_uint = 0x2000  /* IM_FIFOS_ERR_EINT */;
pub const WM8962_IM_FIFOS_ERR_EINT_MASK: c_uint = 0x2000  /* IM_FIFOS_ERR_EINT */;

pub const WM8962_IM_ALC_LOCK_EINT: c_uint = 0x1000  /* IM_ALC_LOCK_EINT */;
pub const WM8962_IM_ALC_LOCK_EINT_MASK: c_uint = 0x1000  /* IM_ALC_LOCK_EINT */;

pub const WM8962_IM_ALC_THRESH_EINT: c_uint = 0x0800  /* IM_ALC_THRESH_EINT */;
pub const WM8962_IM_ALC_THRESH_EINT_MASK: c_uint = 0x0800  /* IM_ALC_THRESH_EINT */;

pub const WM8962_IM_ALC_SAT_EINT: c_uint = 0x0400  /* IM_ALC_SAT_EINT */;
pub const WM8962_IM_ALC_SAT_EINT_MASK: c_uint = 0x0400  /* IM_ALC_SAT_EINT */;

pub const WM8962_IM_ALC_PKOVR_EINT: c_uint = 0x0200  /* IM_ALC_PKOVR_EINT */;
pub const WM8962_IM_ALC_PKOVR_EINT_MASK: c_uint = 0x0200  /* IM_ALC_PKOVR_EINT */;

pub const WM8962_IM_ALC_NGATE_EINT: c_uint = 0x0100  /* IM_ALC_NGATE_EINT */;
pub const WM8962_IM_ALC_NGATE_EINT_MASK: c_uint = 0x0100  /* IM_ALC_NGATE_EINT */;

pub const WM8962_IM_WSEQ_DONE_EINT: c_uint = 0x0080  /* IM_WSEQ_DONE_EINT */;
pub const WM8962_IM_WSEQ_DONE_EINT_MASK: c_uint = 0x0080  /* IM_WSEQ_DONE_EINT */;

pub const WM8962_IM_DRC_ACTDET_EINT: c_uint = 0x0040  /* IM_DRC_ACTDET_EINT */;
pub const WM8962_IM_DRC_ACTDET_EINT_MASK: c_uint = 0x0040  /* IM_DRC_ACTDET_EINT */;

pub const WM8962_IM_FLL_LOCK_EINT: c_uint = 0x0020  /* IM_FLL_LOCK_EINT */;
pub const WM8962_IM_FLL_LOCK_EINT_MASK: c_uint = 0x0020  /* IM_FLL_LOCK_EINT */;

pub const WM8962_IM_PLL3_LOCK_EINT: c_uint = 0x0008  /* IM_PLL3_LOCK_EINT */;
pub const WM8962_IM_PLL3_LOCK_EINT_MASK: c_uint = 0x0008  /* IM_PLL3_LOCK_EINT */;

pub const WM8962_IM_PLL2_LOCK_EINT: c_uint = 0x0004  /* IM_PLL2_LOCK_EINT */;
pub const WM8962_IM_PLL2_LOCK_EINT_MASK: c_uint = 0x0004  /* IM_PLL2_LOCK_EINT */;

pub const WM8962_IM_TEMP_SHUT_EINT: c_uint = 0x0001  /* IM_TEMP_SHUT_EINT */;
pub const WM8962_IM_TEMP_SHUT_EINT_MASK: c_uint = 0x0001  /* IM_TEMP_SHUT_EINT */;

//
// R576 (0x240) - Interrupt Control
//
pub const WM8962_IRQ_POL: c_uint = 0x0001  /* IRQ_POL */;
pub const WM8962_IRQ_POL_MASK: c_uint = 0x0001  /* IRQ_POL */;

//
// R584 (0x248) - IRQ Debounce
//
pub const WM8962_FLL_LOCK_DB: c_uint = 0x0020  /* FLL_LOCK_DB */;
pub const WM8962_FLL_LOCK_DB_MASK: c_uint = 0x0020  /* FLL_LOCK_DB */;

pub const WM8962_PLL3_LOCK_DB: c_uint = 0x0008  /* PLL3_LOCK_DB */;
pub const WM8962_PLL3_LOCK_DB_MASK: c_uint = 0x0008  /* PLL3_LOCK_DB */;

pub const WM8962_PLL2_LOCK_DB: c_uint = 0x0004  /* PLL2_LOCK_DB */;
pub const WM8962_PLL2_LOCK_DB_MASK: c_uint = 0x0004  /* PLL2_LOCK_DB */;

pub const WM8962_TEMP_SHUT_DB: c_uint = 0x0001  /* TEMP_SHUT_DB */;
pub const WM8962_TEMP_SHUT_DB_MASK: c_uint = 0x0001  /* TEMP_SHUT_DB */;

//
// R586 (0x24A) -  MICINT Source Pol
//
pub const WM8962_MICSCD_IRQ_POL: c_uint = 0x8000  /* MICSCD_IRQ_POL */;
pub const WM8962_MICSCD_IRQ_POL_MASK: c_uint = 0x8000  /* MICSCD_IRQ_POL */;

pub const WM8962_MICD_IRQ_POL: c_uint = 0x4000  /* MICD_IRQ_POL */;
pub const WM8962_MICD_IRQ_POL_MASK: c_uint = 0x4000  /* MICD_IRQ_POL */;

//
// R768 (0x300) - DSP2 Power Management
//
pub const WM8962_DSP2_ENA: c_uint = 0x0001  /* DSP2_ENA */;
pub const WM8962_DSP2_ENA_MASK: c_uint = 0x0001  /* DSP2_ENA */;

//
// R1037 (0x40D) - DSP2_ExecControl
//
pub const WM8962_DSP2_STOPC: c_uint = 0x0020  /* DSP2_STOPC */;
pub const WM8962_DSP2_STOPC_MASK: c_uint = 0x0020  /* DSP2_STOPC */;

pub const WM8962_DSP2_STOPS: c_uint = 0x0010  /* DSP2_STOPS */;
pub const WM8962_DSP2_STOPS_MASK: c_uint = 0x0010  /* DSP2_STOPS */;

pub const WM8962_DSP2_STOPI: c_uint = 0x0008  /* DSP2_STOPI */;
pub const WM8962_DSP2_STOPI_MASK: c_uint = 0x0008  /* DSP2_STOPI */;

pub const WM8962_DSP2_STOP: c_uint = 0x0004  /* DSP2_STOP */;
pub const WM8962_DSP2_STOP_MASK: c_uint = 0x0004  /* DSP2_STOP */;

pub const WM8962_DSP2_RUNR: c_uint = 0x0002  /* DSP2_RUNR */;
pub const WM8962_DSP2_RUNR_MASK: c_uint = 0x0002  /* DSP2_RUNR */;

pub const WM8962_DSP2_RUN: c_uint = 0x0001  /* DSP2_RUN */;
pub const WM8962_DSP2_RUN_MASK: c_uint = 0x0001  /* DSP2_RUN */;

//
// R8192 (0x2000) - DSP2 Instruction RAM 0
//
pub const WM8962_DSP2_INSTR_RAM_1024_10_9_0_MASK: c_uint = 0x03FF  /* DSP2_INSTR_RAM_1024_10_9_0 - [9:0] */;

//
// R9216 (0x2400) - DSP2 Address RAM 2
//
pub const WM8962_DSP2_ADDR_RAM_1024_38_37_32_MASK: c_uint = 0x003F  /* DSP2_ADDR_RAM_1024_38_37_32 - [5:0] */;

//
// R9217 (0x2401) - DSP2 Address RAM 1
//
pub const WM8962_DSP2_ADDR_RAM_1024_38_31_16_MASK: c_uint = 0xFFFF  /* DSP2_ADDR_RAM_1024_38_31_16 - [15:0] */;

//
// R9218 (0x2402) - DSP2 Address RAM 0
//
pub const WM8962_DSP2_ADDR_RAM_1024_38_15_0_MASK: c_uint = 0xFFFF  /* DSP2_ADDR_RAM_1024_38_15_0 - [15:0] */;

//
// R12288 (0x3000) - DSP2 Data1 RAM 1
//
pub const WM8962_DSP2_DATA1_RAM_384_24_23_16_MASK: c_uint = 0x00FF  /* DSP2_DATA1_RAM_384_24_23_16 - [7:0] */;

//
// R12289 (0x3001) - DSP2 Data1 RAM 0
//
pub const WM8962_DSP2_DATA1_RAM_384_24_15_0_MASK: c_uint = 0xFFFF  /* DSP2_DATA1_RAM_384_24_15_0 - [15:0] */;

//
// R13312 (0x3400) - DSP2 Data2 RAM 1
//
pub const WM8962_DSP2_DATA2_RAM_384_24_23_16_MASK: c_uint = 0x00FF  /* DSP2_DATA2_RAM_384_24_23_16 - [7:0] */;

//
// R13313 (0x3401) - DSP2 Data2 RAM 0
//
pub const WM8962_DSP2_DATA2_RAM_384_24_15_0_MASK: c_uint = 0xFFFF  /* DSP2_DATA2_RAM_384_24_15_0 - [15:0] */;

//
// R14336 (0x3800) - DSP2 Data3 RAM 1
//
pub const WM8962_DSP2_DATA3_RAM_384_24_23_16_MASK: c_uint = 0x00FF  /* DSP2_DATA3_RAM_384_24_23_16 - [7:0] */;

//
// R14337 (0x3801) - DSP2 Data3 RAM 0
//
pub const WM8962_DSP2_DATA3_RAM_384_24_15_0_MASK: c_uint = 0xFFFF  /* DSP2_DATA3_RAM_384_24_15_0 - [15:0] */;

//
// R15360 (0x3C00) - DSP2 Coeff RAM 0
//
pub const WM8962_DSP2_CMAP_RAM_384_11_10_0_MASK: c_uint = 0x07FF  /* DSP2_CMAP_RAM_384_11_10_0 - [10:0] */;

//
// R16384 (0x4000) - RETUNEADC_SHARED_COEFF_1
//
pub const WM8962_ADC_RETUNE_SCV: c_uint = 0x0080  /* ADC_RETUNE_SCV */;
pub const WM8962_ADC_RETUNE_SCV_MASK: c_uint = 0x0080  /* ADC_RETUNE_SCV */;

pub const WM8962_RETUNEADC_SHARED_COEFF_22_16_MASK: c_uint = 0x007F  /* RETUNEADC_SHARED_COEFF_22_16 - [6:0] */;

//
// R16385 (0x4001) - RETUNEADC_SHARED_COEFF_0
//
pub const WM8962_RETUNEADC_SHARED_COEFF_15_00_MASK: c_uint = 0xFFFF  /* RETUNEADC_SHARED_COEFF_15_00 - [15:0] */;

//
// R16386 (0x4002) - RETUNEDAC_SHARED_COEFF_1
//
pub const WM8962_DAC_RETUNE_SCV: c_uint = 0x0080  /* DAC_RETUNE_SCV */;
pub const WM8962_DAC_RETUNE_SCV_MASK: c_uint = 0x0080  /* DAC_RETUNE_SCV */;

pub const WM8962_RETUNEDAC_SHARED_COEFF_23_16_MASK: c_uint = 0x007F  /* RETUNEDAC_SHARED_COEFF_23_16 - [6:0] */;

//
// R16387 (0x4003) - RETUNEDAC_SHARED_COEFF_0
//
pub const WM8962_RETUNEDAC_SHARED_COEFF_15_00_MASK: c_uint = 0xFFFF  /* RETUNEDAC_SHARED_COEFF_15_00 - [15:0] */;

//
// R16388 (0x4004) - SOUNDSTAGE_ENABLES_1
//
pub const WM8962_SOUNDSTAGE_ENABLES_23_16_MASK: c_uint = 0x00FF  /* SOUNDSTAGE_ENABLES_23_16 - [7:0] */;

//
// R16389 (0x4005) - SOUNDSTAGE_ENABLES_0
//
pub const WM8962_SOUNDSTAGE_ENABLES_15_06_MASK: c_uint = 0xFFC0  /* SOUNDSTAGE_ENABLES_15_06 - [15:6] */;

pub const WM8962_RTN_ADC_ENA: c_uint = 0x0020  /* RTN_ADC_ENA */;
pub const WM8962_RTN_ADC_ENA_MASK: c_uint = 0x0020  /* RTN_ADC_ENA */;

pub const WM8962_RTN_DAC_ENA: c_uint = 0x0010  /* RTN_DAC_ENA */;
pub const WM8962_RTN_DAC_ENA_MASK: c_uint = 0x0010  /* RTN_DAC_ENA */;

pub const WM8962_HDBASS_ENA: c_uint = 0x0008  /* HDBASS_ENA */;
pub const WM8962_HDBASS_ENA_MASK: c_uint = 0x0008  /* HDBASS_ENA */;

pub const WM8962_HPF2_ENA: c_uint = 0x0004  /* HPF2_ENA */;
pub const WM8962_HPF2_ENA_MASK: c_uint = 0x0004  /* HPF2_ENA */;

pub const WM8962_HPF1_ENA: c_uint = 0x0002  /* HPF1_ENA */;
pub const WM8962_HPF1_ENA_MASK: c_uint = 0x0002  /* HPF1_ENA */;

pub const WM8962_VSS_ENA: c_uint = 0x0001  /* VSS_ENA */;
pub const WM8962_VSS_ENA_MASK: c_uint = 0x0001  /* VSS_ENA */;

extern "C" {
    pub fn wm8962_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int;
}
