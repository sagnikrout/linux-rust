//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/idt8a340_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Based on 5.2.0, Family Programming Guide (Sept 30, 2020)
//
// Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
//

// Macro flag: #define HAVE_IDT8A340_REG
pub const PAGE_ADDR_BASE: c_uint = 0x0000;
pub const PAGE_ADDR: c_uint = 0x00fc;
pub const HW_REVISION: c_uint = 0x8180;
pub const REV_ID: c_uint = 0x007a;

pub const SYNC_SOURCE_DPLL0_TOD_PPS: c_uint = 0x14;
pub const SYNC_SOURCE_DPLL1_TOD_PPS: c_uint = 0x15;
pub const SYNC_SOURCE_DPLL2_TOD_PPS: c_uint = 0x16;
pub const SYNC_SOURCE_DPLL3_TOD_PPS: c_uint = 0x17;

//
// Select FOD5 as sync_trigger for Q8 divider.
// Transition from logic zero to one
// sets trigger to sync Q8 divider.
//
// Unused when FOD4 is driving Q8 divider (normal operation).
//

//
// Enable FOD5 as driver for clock and sync for Q8 divider.
// Enable fanout buffer for FOD5.
//
// Unused when FOD4 is driving Q8 divider (normal operation).
//

//
// Select FOD6 as sync_trigger for Q11 divider.
// Transition from logic zero to one
// sets trigger to sync Q11 divider.
//
// Unused when FOD7 is driving Q11 divider (normal operation).
//

//
// Enable FOD6 as driver for clock and sync for Q11 divider.
// Enable fanout buffer for FOD6.
//
// Unused when FOD7 is driving Q11 divider (normal operation).
//

pub const RESET_CTRL: c_uint = 0xc000;
pub const SM_RESET: c_uint = 0x0012;
pub const SM_RESET_V520: c_uint = 0x0013;
pub const SM_RESET_CMD: c_uint = 0x5A;
pub const GENERAL_STATUS: c_uint = 0xc014;
pub const BOOT_STATUS: c_uint = 0x0000;
pub const HW_REV_ID: c_uint = 0x000A;
pub const BOND_ID: c_uint = 0x000B;
pub const HW_CSR_ID: c_uint = 0x000C;
pub const HW_IRQ_ID: c_uint = 0x000E;
pub const MAJ_REL: c_uint = 0x0010;
pub const MIN_REL: c_uint = 0x0011;
pub const HOTFIX_REL: c_uint = 0x0012;
pub const PIPELINE_ID: c_uint = 0x0014;
pub const BUILD_ID: c_uint = 0x0018;
pub const JTAG_DEVICE_ID: c_uint = 0x001c;
pub const PRODUCT_ID: c_uint = 0x001e;
pub const OTP_SCSR_CONFIG_SELECT: c_uint = 0x0022;
pub const STATUS: c_uint = 0xc03c;
pub const DPLL0_STATUS: c_uint = 0x0018;
pub const DPLL1_STATUS: c_uint = 0x0019;
pub const DPLL2_STATUS: c_uint = 0x001a;
pub const DPLL3_STATUS: c_uint = 0x001b;
pub const DPLL4_STATUS: c_uint = 0x001c;
pub const DPLL5_STATUS: c_uint = 0x001d;
pub const DPLL6_STATUS: c_uint = 0x001e;
pub const DPLL7_STATUS: c_uint = 0x001f;
pub const DPLL_SYS_STATUS: c_uint = 0x0020;
pub const DPLL_SYS_APLL_STATUS: c_uint = 0x0021;
pub const DPLL0_FILTER_STATUS: c_uint = 0x0044;
pub const DPLL1_FILTER_STATUS: c_uint = 0x004c;
pub const DPLL2_FILTER_STATUS: c_uint = 0x0054;
pub const DPLL3_FILTER_STATUS: c_uint = 0x005c;
pub const DPLL4_FILTER_STATUS: c_uint = 0x0064;
pub const DPLL5_FILTER_STATUS: c_uint = 0x006c;
pub const DPLL6_FILTER_STATUS: c_uint = 0x0074;
pub const DPLL7_FILTER_STATUS: c_uint = 0x007c;
pub const DPLLSYS_FILTER_STATUS: c_uint = 0x0084;
pub const USER_GPIO0_TO_7_STATUS: c_uint = 0x008a;
pub const USER_GPIO8_TO_15_STATUS: c_uint = 0x008b;
pub const GPIO_USER_CONTROL: c_uint = 0xc160;
pub const GPIO0_TO_7_OUT: c_uint = 0x0000;
pub const GPIO8_TO_15_OUT: c_uint = 0x0001;
pub const GPIO0_TO_7_OUT_V520: c_uint = 0x0002;
pub const GPIO8_TO_15_OUT_V520: c_uint = 0x0003;
pub const STICKY_STATUS_CLEAR: c_uint = 0xc164;
pub const GPIO_TOD_NOTIFICATION_CLEAR: c_uint = 0xc16c;
pub const ALERT_CFG: c_uint = 0xc188;
pub const SYS_DPLL_XO: c_uint = 0xc194;
pub const SYS_APLL: c_uint = 0xc19c;
pub const INPUT_0: c_uint = 0xc1b0;
pub const INPUT_1: c_uint = 0xc1c0;
pub const INPUT_2: c_uint = 0xc1d0;
pub const INPUT_3: c_uint = 0xc200;
pub const INPUT_4: c_uint = 0xc210;
pub const INPUT_5: c_uint = 0xc220;
pub const INPUT_6: c_uint = 0xc230;
pub const INPUT_7: c_uint = 0xc240;
pub const INPUT_8: c_uint = 0xc250;
pub const INPUT_9: c_uint = 0xc260;
pub const INPUT_10: c_uint = 0xc280;
pub const INPUT_11: c_uint = 0xc290;
pub const INPUT_12: c_uint = 0xc2a0;
pub const INPUT_13: c_uint = 0xc2b0;
pub const INPUT_14: c_uint = 0xc2c0;
pub const INPUT_15: c_uint = 0xc2d0;
pub const REF_MON_0: c_uint = 0xc2e0;
pub const REF_MON_1: c_uint = 0xc2ec;
pub const REF_MON_2: c_uint = 0xc300;
pub const REF_MON_3: c_uint = 0xc30c;
pub const REF_MON_4: c_uint = 0xc318;
pub const REF_MON_5: c_uint = 0xc324;
pub const REF_MON_6: c_uint = 0xc330;
pub const REF_MON_7: c_uint = 0xc33c;
pub const REF_MON_8: c_uint = 0xc348;
pub const REF_MON_9: c_uint = 0xc354;
pub const REF_MON_10: c_uint = 0xc360;
pub const REF_MON_11: c_uint = 0xc36c;
pub const REF_MON_12: c_uint = 0xc380;
pub const REF_MON_13: c_uint = 0xc38c;
pub const REF_MON_14: c_uint = 0xc398;
pub const REF_MON_15: c_uint = 0xc3a4;
pub const DPLL_0: c_uint = 0xc3b0;
pub const DPLL_CTRL_REG_0: c_uint = 0x0002;
pub const DPLL_CTRL_REG_1: c_uint = 0x0003;
pub const DPLL_CTRL_REG_2: c_uint = 0x0004;
pub const DPLL_TOD_SYNC_CFG: c_uint = 0x0031;
pub const DPLL_COMBO_SLAVE_CFG_0: c_uint = 0x0032;
pub const DPLL_COMBO_SLAVE_CFG_1: c_uint = 0x0033;
pub const DPLL_SLAVE_REF_CFG: c_uint = 0x0034;
pub const DPLL_REF_MODE: c_uint = 0x0035;
pub const DPLL_PHASE_MEASUREMENT_CFG: c_uint = 0x0036;
pub const DPLL_MODE: c_uint = 0x0037;
pub const DPLL_MODE_V520: c_uint = 0x003B;
pub const DPLL_1: c_uint = 0xc400;
pub const DPLL_2: c_uint = 0xc438;
pub const DPLL_2_V520: c_uint = 0xc43c;
pub const DPLL_3: c_uint = 0xc480;
pub const DPLL_4: c_uint = 0xc4b8;
pub const DPLL_4_V520: c_uint = 0xc4bc;
pub const DPLL_5: c_uint = 0xc500;
pub const DPLL_6: c_uint = 0xc538;
pub const DPLL_6_V520: c_uint = 0xc53c;
pub const DPLL_7: c_uint = 0xc580;
pub const SYS_DPLL: c_uint = 0xc5b8;
pub const SYS_DPLL_V520: c_uint = 0xc5bc;
pub const DPLL_CTRL_0: c_uint = 0xc600;
pub const DPLL_CTRL_DPLL_MANU_REF_CFG: c_uint = 0x0001;
pub const DPLL_CTRL_DPLL_FOD_FREQ: c_uint = 0x001c;
pub const DPLL_CTRL_COMBO_MASTER_CFG: c_uint = 0x003a;
pub const DPLL_CTRL_1: c_uint = 0xc63c;
pub const DPLL_CTRL_2: c_uint = 0xc680;
pub const DPLL_CTRL_3: c_uint = 0xc6bc;
pub const DPLL_CTRL_4: c_uint = 0xc700;
pub const DPLL_CTRL_5: c_uint = 0xc73c;
pub const DPLL_CTRL_6: c_uint = 0xc780;
pub const DPLL_CTRL_7: c_uint = 0xc7bc;
pub const SYS_DPLL_CTRL: c_uint = 0xc800;
pub const DPLL_PHASE_0: c_uint = 0xc818;
// Signed 42-bit FFO in units of 2^(-53)
pub const DPLL_WR_PHASE: c_uint = 0x0000;
pub const DPLL_PHASE_1: c_uint = 0xc81c;
pub const DPLL_PHASE_2: c_uint = 0xc820;
pub const DPLL_PHASE_3: c_uint = 0xc824;
pub const DPLL_PHASE_4: c_uint = 0xc828;
pub const DPLL_PHASE_5: c_uint = 0xc82c;
pub const DPLL_PHASE_6: c_uint = 0xc830;
pub const DPLL_PHASE_7: c_uint = 0xc834;
pub const DPLL_FREQ_0: c_uint = 0xc838;
// Signed 42-bit FFO in units of 2^(-53)
pub const DPLL_WR_FREQ: c_uint = 0x0000;
pub const DPLL_FREQ_1: c_uint = 0xc840;
pub const DPLL_FREQ_2: c_uint = 0xc848;
pub const DPLL_FREQ_3: c_uint = 0xc850;
pub const DPLL_FREQ_4: c_uint = 0xc858;
pub const DPLL_FREQ_5: c_uint = 0xc860;
pub const DPLL_FREQ_6: c_uint = 0xc868;
pub const DPLL_FREQ_7: c_uint = 0xc870;
pub const DPLL_PHASE_PULL_IN_0: c_uint = 0xc880;
pub const PULL_IN_OFFSET: c_uint = 0x0000 /* Signed 32 bit */;
pub const PULL_IN_SLOPE_LIMIT: c_uint = 0x0004 /* Unsigned 24 bit */;
pub const PULL_IN_CTRL: c_uint = 0x0007;
pub const DPLL_PHASE_PULL_IN_1: c_uint = 0xc888;
pub const DPLL_PHASE_PULL_IN_2: c_uint = 0xc890;
pub const DPLL_PHASE_PULL_IN_3: c_uint = 0xc898;
pub const DPLL_PHASE_PULL_IN_4: c_uint = 0xc8a0;
pub const DPLL_PHASE_PULL_IN_5: c_uint = 0xc8a8;
pub const DPLL_PHASE_PULL_IN_6: c_uint = 0xc8b0;
pub const DPLL_PHASE_PULL_IN_7: c_uint = 0xc8b8;
pub const GPIO_CFG: c_uint = 0xc8c0;
pub const GPIO_CFG_GBL: c_uint = 0x0000;
pub const GPIO_0: c_uint = 0xc8c2;
pub const GPIO_DCO_INC_DEC: c_uint = 0x0000;
pub const GPIO_OUT_CTRL_0: c_uint = 0x0001;
pub const GPIO_OUT_CTRL_1: c_uint = 0x0002;
pub const GPIO_TOD_TRIG: c_uint = 0x0003;
pub const GPIO_DPLL_INDICATOR: c_uint = 0x0004;
pub const GPIO_LOS_INDICATOR: c_uint = 0x0005;
pub const GPIO_REF_INPUT_DSQ_0: c_uint = 0x0006;
pub const GPIO_REF_INPUT_DSQ_1: c_uint = 0x0007;
pub const GPIO_REF_INPUT_DSQ_2: c_uint = 0x0008;
pub const GPIO_REF_INPUT_DSQ_3: c_uint = 0x0009;
pub const GPIO_MAN_CLK_SEL_0: c_uint = 0x000a;
pub const GPIO_MAN_CLK_SEL_1: c_uint = 0x000b;
pub const GPIO_MAN_CLK_SEL_2: c_uint = 0x000c;
pub const GPIO_SLAVE: c_uint = 0x000d;
pub const GPIO_ALERT_OUT_CFG: c_uint = 0x000e;
pub const GPIO_TOD_NOTIFICATION_CFG: c_uint = 0x000f;
pub const GPIO_CTRL: c_uint = 0x0010;
pub const GPIO_CTRL_V520: c_uint = 0x0011;
pub const GPIO_1: c_uint = 0xc8d4;
pub const GPIO_2: c_uint = 0xc8e6;
pub const GPIO_3: c_uint = 0xc900;
pub const GPIO_4: c_uint = 0xc912;
pub const GPIO_5: c_uint = 0xc924;
pub const GPIO_6: c_uint = 0xc936;
pub const GPIO_7: c_uint = 0xc948;
pub const GPIO_8: c_uint = 0xc95a;
pub const GPIO_9: c_uint = 0xc980;
pub const GPIO_10: c_uint = 0xc992;
pub const GPIO_11: c_uint = 0xc9a4;
pub const GPIO_12: c_uint = 0xc9b6;
pub const GPIO_13: c_uint = 0xc9c8;
pub const GPIO_14: c_uint = 0xc9da;
pub const GPIO_15: c_uint = 0xca00;
pub const OUT_DIV_MUX: c_uint = 0xca12;
pub const OUTPUT_0: c_uint = 0xca14;
pub const OUTPUT_0_V520: c_uint = 0xca20;
// FOD frequency output divider value
pub const OUT_DIV: c_uint = 0x0000;
pub const OUT_DUTY_CYCLE_HIGH: c_uint = 0x0004;
pub const OUT_CTRL_0: c_uint = 0x0008;
pub const OUT_CTRL_1: c_uint = 0x0009;
// Phase adjustment in FOD cycles
pub const OUT_PHASE_ADJ: c_uint = 0x000c;
pub const OUTPUT_1: c_uint = 0xca24;
pub const OUTPUT_1_V520: c_uint = 0xca30;
pub const OUTPUT_2: c_uint = 0xca34;
pub const OUTPUT_2_V520: c_uint = 0xca40;
pub const OUTPUT_3: c_uint = 0xca44;
pub const OUTPUT_3_V520: c_uint = 0xca50;
pub const OUTPUT_4: c_uint = 0xca54;
pub const OUTPUT_4_V520: c_uint = 0xca60;
pub const OUTPUT_5: c_uint = 0xca64;
pub const OUTPUT_5_V520: c_uint = 0xca80;
pub const OUTPUT_6: c_uint = 0xca80;
pub const OUTPUT_6_V520: c_uint = 0xca90;
pub const OUTPUT_7: c_uint = 0xca90;
pub const OUTPUT_7_V520: c_uint = 0xcaa0;
pub const OUTPUT_8: c_uint = 0xcaa0;
pub const OUTPUT_8_V520: c_uint = 0xcab0;
pub const OUTPUT_9: c_uint = 0xcab0;
pub const OUTPUT_9_V520: c_uint = 0xcac0;
pub const OUTPUT_10: c_uint = 0xcac0;
pub const OUTPUT_10_V520: c_uint = 0xcad0;
pub const OUTPUT_11: c_uint = 0xcad0;
pub const OUTPUT_11_V520: c_uint = 0xcae0;
pub const SERIAL: c_uint = 0xcae0;
pub const SERIAL_V520: c_uint = 0xcaf0;
pub const PWM_ENCODER_0: c_uint = 0xcb00;
pub const PWM_ENCODER_1: c_uint = 0xcb08;
pub const PWM_ENCODER_2: c_uint = 0xcb10;
pub const PWM_ENCODER_3: c_uint = 0xcb18;
pub const PWM_ENCODER_4: c_uint = 0xcb20;
pub const PWM_ENCODER_5: c_uint = 0xcb28;
pub const PWM_ENCODER_6: c_uint = 0xcb30;
pub const PWM_ENCODER_7: c_uint = 0xcb38;
pub const PWM_DECODER_0: c_uint = 0xcb40;
pub const PWM_DECODER_1: c_uint = 0xcb48;
pub const PWM_DECODER_1_V520: c_uint = 0xcb4a;
pub const PWM_DECODER_2: c_uint = 0xcb50;
pub const PWM_DECODER_2_V520: c_uint = 0xcb54;
pub const PWM_DECODER_3: c_uint = 0xcb58;
pub const PWM_DECODER_3_V520: c_uint = 0xcb5e;
pub const PWM_DECODER_4: c_uint = 0xcb60;
pub const PWM_DECODER_4_V520: c_uint = 0xcb68;
pub const PWM_DECODER_5: c_uint = 0xcb68;
pub const PWM_DECODER_5_V520: c_uint = 0xcb80;
pub const PWM_DECODER_6: c_uint = 0xcb70;
pub const PWM_DECODER_6_V520: c_uint = 0xcb8a;
pub const PWM_DECODER_7: c_uint = 0xcb80;
pub const PWM_DECODER_7_V520: c_uint = 0xcb94;
pub const PWM_DECODER_8: c_uint = 0xcb88;
pub const PWM_DECODER_8_V520: c_uint = 0xcb9e;
pub const PWM_DECODER_9: c_uint = 0xcb90;
pub const PWM_DECODER_9_V520: c_uint = 0xcba8;
pub const PWM_DECODER_10: c_uint = 0xcb98;
pub const PWM_DECODER_10_V520: c_uint = 0xcbb2;
pub const PWM_DECODER_11: c_uint = 0xcba0;
pub const PWM_DECODER_11_V520: c_uint = 0xcbbc;
pub const PWM_DECODER_12: c_uint = 0xcba8;
pub const PWM_DECODER_12_V520: c_uint = 0xcbc6;
pub const PWM_DECODER_13: c_uint = 0xcbb0;
pub const PWM_DECODER_13_V520: c_uint = 0xcbd0;
pub const PWM_DECODER_14: c_uint = 0xcbb8;
pub const PWM_DECODER_14_V520: c_uint = 0xcbda;
pub const PWM_DECODER_15: c_uint = 0xcbc0;
pub const PWM_DECODER_15_V520: c_uint = 0xcbe4;
pub const PWM_USER_DATA: c_uint = 0xcbc8;
pub const PWM_USER_DATA_V520: c_uint = 0xcbf0;
pub const TOD_0: c_uint = 0xcbcc;
pub const TOD_0_V520: c_uint = 0xcc00;
// Enable TOD counter, output channel sync and even-PPS mode
pub const TOD_CFG: c_uint = 0x0000;
pub const TOD_CFG_V520: c_uint = 0x0001;
pub const TOD_1: c_uint = 0xcbce;
pub const TOD_1_V520: c_uint = 0xcc02;
pub const TOD_2: c_uint = 0xcbd0;
pub const TOD_2_V520: c_uint = 0xcc04;
pub const TOD_3: c_uint = 0xcbd2;
pub const TOD_3_V520: c_uint = 0xcc06;
pub const TOD_WRITE_0: c_uint = 0xcc00;
pub const TOD_WRITE_0_V520: c_uint = 0xcc10;
// 8-bit subns, 32-bit ns, 48-bit seconds
pub const TOD_WRITE: c_uint = 0x0000;
// Counter increments after TOD write is completed
pub const TOD_WRITE_COUNTER: c_uint = 0x000c;
// TOD write trigger configuration
pub const TOD_WRITE_SELECT_CFG_0: c_uint = 0x000d;
// TOD write trigger selection
pub const TOD_WRITE_CMD: c_uint = 0x000f;
pub const TOD_WRITE_1: c_uint = 0xcc10;
pub const TOD_WRITE_1_V520: c_uint = 0xcc20;
pub const TOD_WRITE_2: c_uint = 0xcc20;
pub const TOD_WRITE_2_V520: c_uint = 0xcc30;
pub const TOD_WRITE_3: c_uint = 0xcc30;
pub const TOD_WRITE_3_V520: c_uint = 0xcc40;
pub const TOD_READ_PRIMARY_0: c_uint = 0xcc40;
pub const TOD_READ_PRIMARY_0_V520: c_uint = 0xcc50;
// 8-bit subns, 32-bit ns, 48-bit seconds
pub const TOD_READ_PRIMARY_BASE: c_uint = 0x0000;
// Counter increments after TOD write is completed
pub const TOD_READ_PRIMARY_COUNTER: c_uint = 0x000b;
// Read trigger configuration
pub const TOD_READ_PRIMARY_SEL_CFG_0: c_uint = 0x000c;
// Read trigger selection
pub const TOD_READ_PRIMARY_CMD: c_uint = 0x000e;
pub const TOD_READ_PRIMARY_CMD_V520: c_uint = 0x000f;
pub const TOD_READ_PRIMARY_1: c_uint = 0xcc50;
pub const TOD_READ_PRIMARY_1_V520: c_uint = 0xcc60;
pub const TOD_READ_PRIMARY_2: c_uint = 0xcc60;
pub const TOD_READ_PRIMARY_2_V520: c_uint = 0xcc80;
pub const TOD_READ_PRIMARY_3: c_uint = 0xcc80;
pub const TOD_READ_PRIMARY_3_V520: c_uint = 0xcc90;
pub const TOD_READ_SECONDARY_0: c_uint = 0xcc90;
pub const TOD_READ_SECONDARY_0_V520: c_uint = 0xcca0;
// 8-bit subns, 32-bit ns, 48-bit seconds
pub const TOD_READ_SECONDARY_BASE: c_uint = 0x0000;
// Counter increments after TOD write is completed
pub const TOD_READ_SECONDARY_COUNTER: c_uint = 0x000b;
// Read trigger configuration
pub const TOD_READ_SECONDARY_SEL_CFG_0: c_uint = 0x000c;
// Read trigger selection
pub const TOD_READ_SECONDARY_CMD: c_uint = 0x000e;
pub const TOD_READ_SECONDARY_CMD_V520: c_uint = 0x000f;
pub const TOD_READ_SECONDARY_1: c_uint = 0xcca0;
pub const TOD_READ_SECONDARY_1_V520: c_uint = 0xccb0;
pub const TOD_READ_SECONDARY_2: c_uint = 0xccb0;
pub const TOD_READ_SECONDARY_2_V520: c_uint = 0xccc0;
pub const TOD_READ_SECONDARY_3: c_uint = 0xccc0;
pub const TOD_READ_SECONDARY_3_V520: c_uint = 0xccd0;
pub const OUTPUT_TDC_CFG: c_uint = 0xccd0;
pub const OUTPUT_TDC_CFG_V520: c_uint = 0xcce0;
pub const OUTPUT_TDC_0: c_uint = 0xcd00;
pub const OUTPUT_TDC_1: c_uint = 0xcd08;
pub const OUTPUT_TDC_2: c_uint = 0xcd10;
pub const OUTPUT_TDC_3: c_uint = 0xcd18;
pub const INPUT_TDC: c_uint = 0xcd20;
pub const SCRATCH: c_uint = 0xcf50;
pub const SCRATCH_V520: c_uint = 0xcf4c;
pub const EEPROM: c_uint = 0xcf68;
pub const EEPROM_V520: c_uint = 0xcf64;
pub const OTP: c_uint = 0xcf70;
pub const BYTE: c_uint = 0xcf80;
// Bit definitions for the MAJ_REL register

// Bit definitions for the USER_GPIO0_TO_7_STATUS register

// Bit definitions for the USER_GPIO8_TO_15_STATUS register

// Bit definitions for the GPIO0_TO_7_OUT register

// Bit definitions for the GPIO8_TO_15_OUT register

// Bit definitions for the DPLL_TOD_SYNC_CFG register

// Bit definitions for the DPLL_MODE register

// Bit definitions for the DPLL_MANU_REF_CFG register

// Bit definitions for the GPIO_CFG_GBL register

// Bit definitions for the GPIO_DCO_INC_DEC register

// Bit definitions for the GPIO_OUT_CTRL_0 register

// Bit definitions for the GPIO_OUT_CTRL_1 register

// Bit definitions for the GPIO_TOD_TRIG register

// Bit definitions for the GPIO_DPLL_INDICATOR register

// Bit definitions for the GPIO_LOS_INDICATOR register

// Active level of LOS indicator, 0=low 1=high

// Bit definitions for the GPIO_REF_INPUT_DSQ_0 register

// Bit definitions for the GPIO_REF_INPUT_DSQ_1 register

// Bit definitions for the GPIO_REF_INPUT_DSQ_2 register

// Bit definitions for the GPIO_REF_INPUT_DSQ_3 register

// Bit definitions for the GPIO_TOD_NOTIFICATION_CFG register

// Bit definitions for the GPIO_CTRL register

// Bit definitions for the OUT_CTRL_1 register

// Bit definitions for the TOD_CFG register

// Bit definitions for the TOD_WRITE_SELECT_CFG_0 register

// Bit definitions for the TOD_WRITE_CMD register

// 4.8.7

// Bit definitions for the TOD_READ_PRIMARY_SEL_CFG_0 register

// Bit definitions for the TOD_READ_PRIMARY_CMD register

// Bit definitions for the DPLL_CTRL_COMBO_MASTER_CFG register

// Bit definitions for DPLL_SYS_STATUS register

// Bit definitions for SYS_APLL_STATUS register

pub const SYS_APLL_LOSS_LOCK_LIVE_LOCKED: c_int = 0;
pub const SYS_APLL_LOSS_LOCK_LIVE_UNLOCKED: c_int = 1;
// Bit definitions for the DPLL0_STATUS register

// Values of DPLL_N.DPLL_MODE.PLL_MODE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pll_mode {
    PLL_MODE_MIN = 0,
    PLL_MODE_PLL = PLL_MODE_MIN,
    PLL_MODE_WRITE_PHASE = 1,
    PLL_MODE_WRITE_FREQUENCY = 2,
    PLL_MODE_GPIO_INC_DEC = 3,
    PLL_MODE_SYNTHESIS = 4,
    PLL_MODE_PHASE_MEASUREMENT = 5,
    PLL_MODE_DISABLED = 6,
    PLL_MODE_MAX = PLL_MODE_DISABLED,
}

// Values of DPLL_CTRL_n.DPLL_MANU_REF_CFG.MANUAL_REFERENCE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum manual_reference {
    MANU_REF_MIN = 0,
    MANU_REF_CLK0 = MANU_REF_MIN,
    MANU_REF_CLK1,
    MANU_REF_CLK2,
    MANU_REF_CLK3,
    MANU_REF_CLK4,
    MANU_REF_CLK5,
    MANU_REF_CLK6,
    MANU_REF_CLK7,
    MANU_REF_CLK8,
    MANU_REF_CLK9,
    MANU_REF_CLK10,
    MANU_REF_CLK11,
    MANU_REF_CLK12,
    MANU_REF_CLK13,
    MANU_REF_CLK14,
    MANU_REF_CLK15,
    MANU_REF_WRITE_PHASE,
    MANU_REF_WRITE_FREQUENCY,
    MANU_REF_XO_DPLL,
    MANU_REF_MAX = MANU_REF_XO_DPLL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_tod_write_trig_sel {
    HW_TOD_WR_TRIG_SEL_MIN = 0,
    HW_TOD_WR_TRIG_SEL_MSB = HW_TOD_WR_TRIG_SEL_MIN,
    HW_TOD_WR_TRIG_SEL_RESERVED = 1,
    HW_TOD_WR_TRIG_SEL_TOD_PPS = 2,
    HW_TOD_WR_TRIG_SEL_IRIGB_PPS = 3,
    HW_TOD_WR_TRIG_SEL_PWM_PPS = 4,
    HW_TOD_WR_TRIG_SEL_GPIO = 5,
    HW_TOD_WR_TRIG_SEL_FOD_SYNC = 6,
    WR_TRIG_SEL_MAX = HW_TOD_WR_TRIG_SEL_FOD_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsr_read_trig_sel {
// CANCEL CURRENT TOD READ; MODULE BECOMES IDLE - NO TRIGGER OCCURS
    SCSR_TOD_READ_TRIG_SEL_DISABLE = 0,
// TRIGGER IMMEDIATELY
    SCSR_TOD_READ_TRIG_SEL_IMMEDIATE = 1,
// TRIGGER ON RISING EDGE OF INTERNAL TOD PPS SIGNAL
    SCSR_TOD_READ_TRIG_SEL_TODPPS = 2,
// TRGGER ON RISING EDGE OF SELECTED REFERENCE INPUT
    SCSR_TOD_READ_TRIG_SEL_REFCLK = 3,
// TRIGGER ON RISING EDGE OF SELECTED PWM DECODER 1PPS OUTPUT
    SCSR_TOD_READ_TRIG_SEL_PWMPPS = 4,
    SCSR_TOD_READ_TRIG_SEL_RESERVED = 5,
// TRIGGER WHEN WRITE FREQUENCY EVENT OCCURS
    SCSR_TOD_READ_TRIG_SEL_WRITEFREQUENCYEVENT = 6,
// TRIGGER ON SELECTED GPIO
    SCSR_TOD_READ_TRIG_SEL_GPIO = 7,
    SCSR_TOD_READ_TRIG_SEL_MAX = SCSR_TOD_READ_TRIG_SEL_GPIO,
}

// Values STATUS.DPLL_SYS_STATUS.DPLL_SYS_STATE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpll_state {
    DPLL_STATE_MIN = 0,
    DPLL_STATE_FREERUN = DPLL_STATE_MIN,
    DPLL_STATE_LOCKACQ = 1,
    DPLL_STATE_LOCKREC = 2,
    DPLL_STATE_LOCKED = 3,
    DPLL_STATE_HOLDOVER = 4,
    DPLL_STATE_OPEN_LOOP = 5,
    DPLL_STATE_MAX = DPLL_STATE_OPEN_LOOP,
}

// 4.8.7 only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsr_tod_write_trig_sel {
    SCSR_TOD_WR_TRIG_SEL_DISABLE = 0,
    SCSR_TOD_WR_TRIG_SEL_IMMEDIATE = 1,
    SCSR_TOD_WR_TRIG_SEL_REFCLK = 2,
    SCSR_TOD_WR_TRIG_SEL_PWMPPS = 3,
    SCSR_TOD_WR_TRIG_SEL_TODPPS = 4,
    SCSR_TOD_WR_TRIG_SEL_SYNCFOD = 5,
    SCSR_TOD_WR_TRIG_SEL_GPIO = 6,
    SCSR_TOD_WR_TRIG_SEL_MAX = SCSR_TOD_WR_TRIG_SEL_GPIO,
}

// 4.8.7 only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsr_tod_write_type_sel {
    SCSR_TOD_WR_TYPE_SEL_ABSOLUTE = 0,
    SCSR_TOD_WR_TYPE_SEL_DELTA_PLUS = 1,
    SCSR_TOD_WR_TYPE_SEL_DELTA_MINUS = 2,
    SCSR_TOD_WR_TYPE_SEL_MAX = SCSR_TOD_WR_TYPE_SEL_DELTA_MINUS,
}
