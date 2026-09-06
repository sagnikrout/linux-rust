//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd71828.h
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
// Copyright (C) 2019 ROHM Semiconductors

// Regulator IDs
pub const BD71828_BUCK1267_VOLTS: c_uint = 0x100;
pub const BD71828_BUCK3_VOLTS: c_uint = 0x20;
pub const BD71828_BUCK4_VOLTS: c_uint = 0x40;
pub const BD71828_BUCK5_VOLTS: c_uint = 0x20;
pub const BD71828_LDO_VOLTS: c_uint = 0x40;
// LDO6 is fixed 1.8V voltage
pub const BD71828_LDO_6_VOLTAGE: c_int = 1800000;
// Registers and masks
// MODE control
pub const BD71828_REG_PS_CTRL_1: c_uint = 0x04;
pub const BD71828_REG_PS_CTRL_2: c_uint = 0x05;
pub const BD71828_REG_PS_CTRL_3: c_uint = 0x06;

pub const BD71828_MASK_RUN_LVL_CTRL: c_uint = 0x30;
// Regulator control masks
pub const BD71828_MASK_RAMP_DELAY: c_uint = 0x6;
pub const BD71828_MASK_RUN_EN: c_uint = 0x08;
pub const BD71828_MASK_SUSP_EN: c_uint = 0x04;
pub const BD71828_MASK_IDLE_EN: c_uint = 0x02;
pub const BD71828_MASK_LPSR_EN: c_uint = 0x01;
pub const BD71828_MASK_RUN0_EN: c_uint = 0x01;
pub const BD71828_MASK_RUN1_EN: c_uint = 0x02;
pub const BD71828_MASK_RUN2_EN: c_uint = 0x04;
pub const BD71828_MASK_RUN3_EN: c_uint = 0x08;
pub const BD71828_MASK_DVS_BUCK1_CTRL: c_uint = 0x10;
pub const BD71828_DVS_BUCK1_CTRL_I2C: c_int = 0;
pub const BD71828_DVS_BUCK1_USE_RUNLVL: c_uint = 0x10;
pub const BD71828_MASK_DVS_BUCK2_CTRL: c_uint = 0x20;
pub const BD71828_DVS_BUCK2_CTRL_I2C: c_int = 0;
pub const BD71828_DVS_BUCK2_USE_RUNLVL: c_uint = 0x20;
pub const BD71828_MASK_DVS_BUCK6_CTRL: c_uint = 0x40;
pub const BD71828_DVS_BUCK6_CTRL_I2C: c_int = 0;
pub const BD71828_DVS_BUCK6_USE_RUNLVL: c_uint = 0x40;
pub const BD71828_MASK_DVS_BUCK7_CTRL: c_uint = 0x80;
pub const BD71828_DVS_BUCK7_CTRL_I2C: c_int = 0;
pub const BD71828_DVS_BUCK7_USE_RUNLVL: c_uint = 0x80;
pub const BD71828_MASK_BUCK1267_VOLT: c_uint = 0xff;
pub const BD71828_MASK_BUCK3_VOLT: c_uint = 0x1f;
pub const BD71828_MASK_BUCK4_VOLT: c_uint = 0x3f;
pub const BD71828_MASK_BUCK5_VOLT: c_uint = 0x1f;
pub const BD71828_MASK_LDO_VOLT: c_uint = 0x3f;
// Regulator control regs
pub const BD71828_REG_BUCK1_EN: c_uint = 0x08;
pub const BD71828_REG_BUCK1_CTRL: c_uint = 0x09;
pub const BD71828_REG_BUCK1_MODE: c_uint = 0x0a;
pub const BD71828_REG_BUCK1_IDLE_VOLT: c_uint = 0x0b;
pub const BD71828_REG_BUCK1_SUSP_VOLT: c_uint = 0x0c;
pub const BD71828_REG_BUCK1_VOLT: c_uint = 0x0d;
pub const BD71828_REG_BUCK2_EN: c_uint = 0x12;
pub const BD71828_REG_BUCK2_CTRL: c_uint = 0x13;
pub const BD71828_REG_BUCK2_MODE: c_uint = 0x14;
pub const BD71828_REG_BUCK2_IDLE_VOLT: c_uint = 0x15;
pub const BD71828_REG_BUCK2_SUSP_VOLT: c_uint = 0x16;
pub const BD71828_REG_BUCK2_VOLT: c_uint = 0x17;
pub const BD71828_REG_BUCK3_EN: c_uint = 0x1c;
pub const BD71828_REG_BUCK3_MODE: c_uint = 0x1d;
pub const BD71828_REG_BUCK3_VOLT: c_uint = 0x1e;
pub const BD71828_REG_BUCK4_EN: c_uint = 0x1f;
pub const BD71828_REG_BUCK4_MODE: c_uint = 0x20;
pub const BD71828_REG_BUCK4_VOLT: c_uint = 0x21;
pub const BD71828_REG_BUCK5_EN: c_uint = 0x22;
pub const BD71828_REG_BUCK5_MODE: c_uint = 0x23;
pub const BD71828_REG_BUCK5_VOLT: c_uint = 0x24;
pub const BD71828_REG_BUCK6_EN: c_uint = 0x25;
pub const BD71828_REG_BUCK6_CTRL: c_uint = 0x26;
pub const BD71828_REG_BUCK6_MODE: c_uint = 0x27;
pub const BD71828_REG_BUCK6_IDLE_VOLT: c_uint = 0x28;
pub const BD71828_REG_BUCK6_SUSP_VOLT: c_uint = 0x29;
pub const BD71828_REG_BUCK6_VOLT: c_uint = 0x2a;
pub const BD71828_REG_BUCK7_EN: c_uint = 0x2f;
pub const BD71828_REG_BUCK7_CTRL: c_uint = 0x30;
pub const BD71828_REG_BUCK7_MODE: c_uint = 0x31;
pub const BD71828_REG_BUCK7_IDLE_VOLT: c_uint = 0x32;
pub const BD71828_REG_BUCK7_SUSP_VOLT: c_uint = 0x33;
pub const BD71828_REG_BUCK7_VOLT: c_uint = 0x34;
pub const BD71828_REG_LDO1_EN: c_uint = 0x39;
pub const BD71828_REG_LDO1_VOLT: c_uint = 0x3a;
pub const BD71828_REG_LDO2_EN: c_uint = 0x3b;
pub const BD71828_REG_LDO2_VOLT: c_uint = 0x3c;
pub const BD71828_REG_LDO3_EN: c_uint = 0x3d;
pub const BD71828_REG_LDO3_VOLT: c_uint = 0x3e;
pub const BD71828_REG_LDO4_EN: c_uint = 0x3f;
pub const BD71828_REG_LDO4_VOLT: c_uint = 0x40;
pub const BD71828_REG_LDO5_EN: c_uint = 0x41;
pub const BD71828_REG_LDO5_VOLT: c_uint = 0x43;
pub const BD71828_REG_LDO5_VOLT_OPT: c_uint = 0x42;
pub const BD71828_REG_LDO6_EN: c_uint = 0x44;
pub const BD71828_REG_LDO7_EN: c_uint = 0x45;
pub const BD71828_REG_LDO7_VOLT: c_uint = 0x46;
// GPIO
pub const BD71828_GPIO_DRIVE_MASK: c_uint = 0x2;
pub const BD71828_GPIO_OPEN_DRAIN: c_uint = 0x0;
pub const BD71828_GPIO_PUSH_PULL: c_uint = 0x2;
pub const BD71828_GPIO_OUT_HI: c_uint = 0x1;
pub const BD71828_GPIO_OUT_LO: c_uint = 0x0;
pub const BD71828_GPIO_OUT_MASK: c_uint = 0x1;
pub const BD71828_REG_GPIO_CTRL1: c_uint = 0x47;
pub const BD71828_REG_GPIO_CTRL2: c_uint = 0x48;
pub const BD71828_REG_GPIO_CTRL3: c_uint = 0x49;
pub const BD71828_REG_IO_STAT: c_uint = 0xed;
// clk
pub const BD71828_REG_OUT32K: c_uint = 0x4b;
// RTC
pub const BD71828_REG_RTC_SEC: c_uint = 0x4c;
pub const BD71828_REG_RTC_MINUTE: c_uint = 0x4d;
pub const BD71828_REG_RTC_HOUR: c_uint = 0x4e;
pub const BD71828_REG_RTC_WEEK: c_uint = 0x4f;
pub const BD71828_REG_RTC_DAY: c_uint = 0x50;
pub const BD71828_REG_RTC_MONTH: c_uint = 0x51;
pub const BD71828_REG_RTC_YEAR: c_uint = 0x52;
pub const BD71828_REG_RTC_ALM0_SEC: c_uint = 0x53;

pub const BD71828_REG_RTC_ALM0_MINUTE: c_uint = 0x54;
pub const BD71828_REG_RTC_ALM0_HOUR: c_uint = 0x55;
pub const BD71828_REG_RTC_ALM0_WEEK: c_uint = 0x56;
pub const BD71828_REG_RTC_ALM0_DAY: c_uint = 0x57;
pub const BD71828_REG_RTC_ALM0_MONTH: c_uint = 0x58;
pub const BD71828_REG_RTC_ALM0_YEAR: c_uint = 0x59;
pub const BD71828_REG_RTC_ALM0_MASK: c_uint = 0x61;
pub const BD71828_REG_RTC_ALM1_SEC: c_uint = 0x5a;
pub const BD71828_REG_RTC_ALM1_MINUTE: c_uint = 0x5b;
pub const BD71828_REG_RTC_ALM1_HOUR: c_uint = 0x5c;
pub const BD71828_REG_RTC_ALM1_WEEK: c_uint = 0x5d;
pub const BD71828_REG_RTC_ALM1_DAY: c_uint = 0x5e;
pub const BD71828_REG_RTC_ALM1_MONTH: c_uint = 0x5f;
pub const BD71828_REG_RTC_ALM1_YEAR: c_uint = 0x60;
pub const BD71828_REG_RTC_ALM1_MASK: c_uint = 0x62;
pub const BD71828_REG_RTC_ALM2: c_uint = 0x63;

// Charger/Battey
pub const BD71828_REG_CHG_STATE: c_uint = 0x65;
pub const BD71828_REG_CHG_FULL: c_uint = 0xd2;
pub const BD71828_REG_CHG_EN: c_uint = 0x6F;
pub const BD71828_REG_DCIN_STAT: c_uint = 0x68;
pub const BD71828_MASK_DCIN_DET: c_uint = 0x01;
pub const BD71828_REG_VDCIN_U: c_uint = 0x9c;
pub const BD71828_MASK_CHG_EN: c_uint = 0x01;
pub const BD71828_CHG_MASK_DCIN_U: c_uint = 0x0f;
pub const BD71828_REG_BAT_STAT: c_uint = 0x67;
pub const BD71828_REG_BAT_TEMP: c_uint = 0x6c;
pub const BD71828_MASK_BAT_TEMP: c_uint = 0x07;
pub const BD71828_BAT_TEMP_OPEN: c_uint = 0x07;
pub const BD71828_MASK_BAT_DET: c_uint = 0x20;
pub const BD71828_MASK_BAT_DET_DONE: c_uint = 0x10;
pub const BD71828_REG_CHG_STATE: c_uint = 0x65;
pub const BD71828_REG_VBAT_U: c_uint = 0x8c;
pub const BD71828_MASK_VBAT_U: c_uint = 0x0f;
pub const BD71828_REG_VBAT_REX_AVG_U: c_uint = 0x92;
pub const BD71828_REG_OCV_PWRON_U: c_uint = 0x8A;
pub const BD71828_REG_VBAT_MIN_AVG_U: c_uint = 0x8e;
pub const BD71828_REG_VBAT_MIN_AVG_L: c_uint = 0x8f;
pub const BD71828_REG_CC_CNT3: c_uint = 0xb5;
pub const BD71828_REG_CC_CNT2: c_uint = 0xb6;
pub const BD71828_REG_CC_CNT1: c_uint = 0xb7;
pub const BD71828_REG_CC_CNT0: c_uint = 0xb8;
pub const BD71828_REG_CC_CURCD_AVG_U: c_uint = 0xb2;
pub const BD71828_MASK_CC_CURCD_AVG_U: c_uint = 0x3f;
pub const BD71828_MASK_CC_CUR_DIR: c_uint = 0x80;
pub const BD71828_REG_VM_BTMP_U: c_uint = 0xa1;
pub const BD71828_REG_VM_BTMP_L: c_uint = 0xa2;
pub const BD71828_MASK_VM_BTMP_U: c_uint = 0x0f;
pub const BD71828_REG_COULOMB_CTRL: c_uint = 0xc4;
pub const BD71828_REG_COULOMB_CTRL2: c_uint = 0xd2;
pub const BD71828_MASK_REX_CC_CLR: c_uint = 0x01;
pub const BD71828_MASK_FULL_CC_CLR: c_uint = 0x10;
pub const BD71828_REG_CC_CNT_FULL3: c_uint = 0xbd;
pub const BD71828_REG_CC_CNT_CHG3: c_uint = 0xc1;
pub const BD71828_REG_VBAT_INITIAL1_U: c_uint = 0x86;
pub const BD71828_REG_VBAT_INITIAL1_L: c_uint = 0x87;
pub const BD71828_REG_VBAT_INITIAL2_U: c_uint = 0x88;
pub const BD71828_REG_VBAT_INITIAL2_L: c_uint = 0x89;
pub const BD71828_REG_IBAT_U: c_uint = 0xb0;
pub const BD71828_REG_IBAT_L: c_uint = 0xb1;
pub const BD71828_REG_IBAT_AVG_U: c_uint = 0xb2;
pub const BD71828_REG_IBAT_AVG_L: c_uint = 0xb3;
pub const BD71828_REG_VSYS_AVG_U: c_uint = 0x96;
pub const BD71828_REG_VSYS_AVG_L: c_uint = 0x97;
pub const BD71828_REG_VSYS_MIN_AVG_U: c_uint = 0x98;
pub const BD71828_REG_VSYS_MIN_AVG_L: c_uint = 0x99;
pub const BD71828_REG_CHG_SET1: c_uint = 0x75;
pub const BD71828_REG_ALM_VBAT_LIMIT_U: c_uint = 0xaa;
pub const BD71828_REG_BATCAP_MON_LIMIT_U: c_uint = 0xcc;
pub const BD71828_REG_CONF: c_uint = 0x64;
pub const BD71828_REG_ILIM_STAT: c_uint = 0x6d;
pub const BD71828_REG_DCIN_SET: c_uint = 0x70;
pub const BD71828_REG_DCIN_CLPS: c_uint = 0x71;
pub const BD71828_REG_MEAS_CLEAR: c_uint = 0xaf;
// LEDs
pub const BD71828_REG_LED_CTRL: c_uint = 0x4A;
pub const BD71828_MASK_LED_AMBER: c_uint = 0x80;
pub const BD71828_MASK_LED_GREEN: c_uint = 0x40;
pub const BD71828_LED_ON: c_uint = 0xff;
pub const BD71828_LED_OFF: c_uint = 0x0;
// IRQ registers
pub const BD71828_REG_INT_MASK_BUCK: c_uint = 0xd3;
pub const BD71828_REG_INT_MASK_DCIN1: c_uint = 0xd4;
pub const BD71828_REG_INT_MASK_DCIN2: c_uint = 0xd5;
pub const BD71828_REG_INT_MASK_VSYS: c_uint = 0xd6;
pub const BD71828_REG_INT_MASK_CHG: c_uint = 0xd7;
pub const BD71828_REG_INT_MASK_BAT: c_uint = 0xd8;
pub const BD71828_REG_INT_MASK_BAT_MON1: c_uint = 0xd9;
pub const BD71828_REG_INT_MASK_BAT_MON2: c_uint = 0xda;
pub const BD71828_REG_INT_MASK_BAT_MON3: c_uint = 0xdb;
pub const BD71828_REG_INT_MASK_BAT_MON4: c_uint = 0xdc;
pub const BD71828_REG_INT_MASK_TEMP: c_uint = 0xdd;
pub const BD71828_REG_INT_MASK_RTC: c_uint = 0xde;
pub const BD71828_REG_INT_MAIN: c_uint = 0xdf;
pub const BD71828_REG_INT_BUCK: c_uint = 0xe0;
pub const BD71828_REG_INT_DCIN1: c_uint = 0xe1;
pub const BD71828_REG_INT_DCIN2: c_uint = 0xe2;
pub const BD71828_REG_INT_VSYS: c_uint = 0xe3;
pub const BD71828_REG_INT_CHG: c_uint = 0xe4;
pub const BD71828_REG_INT_BAT: c_uint = 0xe5;
pub const BD71828_REG_INT_BAT_MON1: c_uint = 0xe6;
pub const BD71828_REG_INT_BAT_MON2: c_uint = 0xe7;
pub const BD71828_REG_INT_BAT_MON3: c_uint = 0xe8;
pub const BD71828_REG_INT_BAT_MON4: c_uint = 0xe9;
pub const BD71828_REG_INT_TEMP: c_uint = 0xea;
pub const BD71828_REG_INT_RTC: c_uint = 0xeb;
pub const BD71828_REG_INT_UPDATE: c_uint = 0xec;

// Masks for main IRQ register bits

// Interrupts
// BUCK reg interrupts
// DCIN1 interrupts
// DCIN2 interrupts
// Vsys
// Charger
// Battery
// Battery Mon 1
// Battery Mon 2
// Battery Mon 3 (Coulomb counter)
// Battery Mon 4
// Temperature
// RTC Alarm
pub const BD71828_INT_BUCK1_OCP_MASK: c_uint = 0x1;
pub const BD71828_INT_BUCK2_OCP_MASK: c_uint = 0x2;
pub const BD71828_INT_BUCK3_OCP_MASK: c_uint = 0x4;
pub const BD71828_INT_BUCK4_OCP_MASK: c_uint = 0x8;
pub const BD71828_INT_BUCK5_OCP_MASK: c_uint = 0x10;
pub const BD71828_INT_BUCK6_OCP_MASK: c_uint = 0x20;
pub const BD71828_INT_BUCK7_OCP_MASK: c_uint = 0x40;
pub const BD71828_INT_PGFAULT_MASK: c_uint = 0x80;
pub const BD71828_INT_DCIN_DET_MASK: c_uint = 0x1;
pub const BD71828_INT_DCIN_RMV_MASK: c_uint = 0x2;
pub const BD71828_INT_CLPS_OUT_MASK: c_uint = 0x4;
pub const BD71828_INT_CLPS_IN_MASK: c_uint = 0x8;
// DCIN2 interrupts
pub const BD71828_INT_DCIN_MON_RES_MASK: c_uint = 0x1;
pub const BD71828_INT_DCIN_MON_DET_MASK: c_uint = 0x2;
pub const BD71828_INT_LONGPUSH_MASK: c_uint = 0x4;
pub const BD71828_INT_MIDPUSH_MASK: c_uint = 0x8;
pub const BD71828_INT_SHORTPUSH_MASK: c_uint = 0x10;
pub const BD71828_INT_PUSH_MASK: c_uint = 0x20;
pub const BD71828_INT_WDOG_MASK: c_uint = 0x40;
pub const BD71828_INT_SWRESET_MASK: c_uint = 0x80;
// Vsys
pub const BD71828_INT_VSYS_UV_RES_MASK: c_uint = 0x1;
pub const BD71828_INT_VSYS_UV_DET_MASK: c_uint = 0x2;
pub const BD71828_INT_VSYS_LOW_RES_MASK: c_uint = 0x4;
pub const BD71828_INT_VSYS_LOW_DET_MASK: c_uint = 0x8;
pub const BD71828_INT_VSYS_HALL_IN_MASK: c_uint = 0x10;
pub const BD71828_INT_VSYS_HALL_TOGGLE_MASK: c_uint = 0x20;
pub const BD71828_INT_VSYS_MON_RES_MASK: c_uint = 0x40;
pub const BD71828_INT_VSYS_MON_DET_MASK: c_uint = 0x80;
// Charger
pub const BD71828_INT_CHG_DCIN_ILIM_MASK: c_uint = 0x1;
pub const BD71828_INT_CHG_TOPOFF_TO_DONE_MASK: c_uint = 0x2;
pub const BD71828_INT_CHG_WDG_TEMP_MASK: c_uint = 0x4;
pub const BD71828_INT_CHG_WDG_TIME_MASK: c_uint = 0x8;
pub const BD71828_INT_CHG_RECHARGE_RES_MASK: c_uint = 0x10;
pub const BD71828_INT_CHG_RECHARGE_DET_MASK: c_uint = 0x20;
pub const BD71828_INT_CHG_RANGED_TEMP_TRANSITION_MASK: c_uint = 0x40;
pub const BD71828_INT_CHG_STATE_TRANSITION_MASK: c_uint = 0x80;
// Battery
pub const BD71828_INT_BAT_TEMP_NORMAL_MASK: c_uint = 0x1;
pub const BD71828_INT_BAT_TEMP_ERANGE_MASK: c_uint = 0x2;
pub const BD71828_INT_BAT_TEMP_WARN_MASK: c_uint = 0x4;
pub const BD71828_INT_BAT_REMOVED_MASK: c_uint = 0x10;
pub const BD71828_INT_BAT_DETECTED_MASK: c_uint = 0x20;
pub const BD71828_INT_THERM_REMOVED_MASK: c_uint = 0x40;
pub const BD71828_INT_THERM_DETECTED_MASK: c_uint = 0x80;
// Battery Mon 1
pub const BD71828_INT_BAT_DEAD_MASK: c_uint = 0x2;
pub const BD71828_INT_BAT_SHORTC_RES_MASK: c_uint = 0x4;
pub const BD71828_INT_BAT_SHORTC_DET_MASK: c_uint = 0x8;
pub const BD71828_INT_BAT_LOW_VOLT_RES_MASK: c_uint = 0x10;
pub const BD71828_INT_BAT_LOW_VOLT_DET_MASK: c_uint = 0x20;
pub const BD71828_INT_BAT_OVER_VOLT_RES_MASK: c_uint = 0x40;
pub const BD71828_INT_BAT_OVER_VOLT_DET_MASK: c_uint = 0x80;
// Battery Mon 2
pub const BD71828_INT_BAT_MON_RES_MASK: c_uint = 0x1;
pub const BD71828_INT_BAT_MON_DET_MASK: c_uint = 0x2;
// Battery Mon 3 (Coulomb counter)
pub const BD71828_INT_BAT_CC_MON1_MASK: c_uint = 0x1;
pub const BD71828_INT_BAT_CC_MON2_MASK: c_uint = 0x2;
pub const BD71828_INT_BAT_CC_MON3_MASK: c_uint = 0x4;
// Battery Mon 4
pub const BD71828_INT_BAT_OVER_CURR_1_RES_MASK: c_uint = 0x1;
pub const BD71828_INT_BAT_OVER_CURR_1_DET_MASK: c_uint = 0x2;
pub const BD71828_INT_BAT_OVER_CURR_2_RES_MASK: c_uint = 0x4;
pub const BD71828_INT_BAT_OVER_CURR_2_DET_MASK: c_uint = 0x8;
pub const BD71828_INT_BAT_OVER_CURR_3_RES_MASK: c_uint = 0x10;
pub const BD71828_INT_BAT_OVER_CURR_3_DET_MASK: c_uint = 0x20;
// Temperature
pub const BD71828_INT_TEMP_BAT_LOW_RES_MASK: c_uint = 0x1;
pub const BD71828_INT_TEMP_BAT_LOW_DET_MASK: c_uint = 0x2;
pub const BD71828_INT_TEMP_BAT_HI_RES_MASK: c_uint = 0x4;
pub const BD71828_INT_TEMP_BAT_HI_DET_MASK: c_uint = 0x8;
pub const BD71828_INT_TEMP_CHIP_OVER_125_RES_MASK: c_uint = 0x10;
pub const BD71828_INT_TEMP_CHIP_OVER_125_DET_MASK: c_uint = 0x20;
pub const BD71828_INT_TEMP_CHIP_OVER_VF_RES_MASK: c_uint = 0x40;
pub const BD71828_INT_TEMP_CHIP_OVER_VF_DET_MASK: c_uint = 0x80;
// RTC Alarm
pub const BD71828_INT_RTC0_MASK: c_uint = 0x1;
pub const BD71828_INT_RTC1_MASK: c_uint = 0x2;
pub const BD71828_INT_RTC2_MASK: c_uint = 0x4;
pub const BD71828_OUT_TYPE_MASK: c_uint = 0x2;
pub const BD71828_OUT_TYPE_OPEN_DRAIN: c_uint = 0x0;
pub const BD71828_OUT_TYPE_CMOS: c_uint = 0x2;
