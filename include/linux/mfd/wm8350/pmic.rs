//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/pmic.h
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
// pmic.h  --  Power Management Driver for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//

//
// Register values.
//
pub const WM8350_CURRENT_SINK_DRIVER_A: c_uint = 0xAC;
pub const WM8350_CSA_FLASH_CONTROL: c_uint = 0xAD;
pub const WM8350_CURRENT_SINK_DRIVER_B: c_uint = 0xAE;
pub const WM8350_CSB_FLASH_CONTROL: c_uint = 0xAF;
pub const WM8350_DCDC_LDO_REQUESTED: c_uint = 0xB0;
pub const WM8350_DCDC_ACTIVE_OPTIONS: c_uint = 0xB1;
pub const WM8350_DCDC_SLEEP_OPTIONS: c_uint = 0xB2;
pub const WM8350_POWER_CHECK_COMPARATOR: c_uint = 0xB3;
pub const WM8350_DCDC1_CONTROL: c_uint = 0xB4;
pub const WM8350_DCDC1_TIMEOUTS: c_uint = 0xB5;
pub const WM8350_DCDC1_LOW_POWER: c_uint = 0xB6;
pub const WM8350_DCDC2_CONTROL: c_uint = 0xB7;
pub const WM8350_DCDC2_TIMEOUTS: c_uint = 0xB8;
pub const WM8350_DCDC3_CONTROL: c_uint = 0xBA;
pub const WM8350_DCDC3_TIMEOUTS: c_uint = 0xBB;
pub const WM8350_DCDC3_LOW_POWER: c_uint = 0xBC;
pub const WM8350_DCDC4_CONTROL: c_uint = 0xBD;
pub const WM8350_DCDC4_TIMEOUTS: c_uint = 0xBE;
pub const WM8350_DCDC4_LOW_POWER: c_uint = 0xBF;
pub const WM8350_DCDC5_CONTROL: c_uint = 0xC0;
pub const WM8350_DCDC5_TIMEOUTS: c_uint = 0xC1;
pub const WM8350_DCDC6_CONTROL: c_uint = 0xC3;
pub const WM8350_DCDC6_TIMEOUTS: c_uint = 0xC4;
pub const WM8350_DCDC6_LOW_POWER: c_uint = 0xC5;
pub const WM8350_LIMIT_SWITCH_CONTROL: c_uint = 0xC7;
pub const WM8350_LDO1_CONTROL: c_uint = 0xC8;
pub const WM8350_LDO1_TIMEOUTS: c_uint = 0xC9;
pub const WM8350_LDO1_LOW_POWER: c_uint = 0xCA;
pub const WM8350_LDO2_CONTROL: c_uint = 0xCB;
pub const WM8350_LDO2_TIMEOUTS: c_uint = 0xCC;
pub const WM8350_LDO2_LOW_POWER: c_uint = 0xCD;
pub const WM8350_LDO3_CONTROL: c_uint = 0xCE;
pub const WM8350_LDO3_TIMEOUTS: c_uint = 0xCF;
pub const WM8350_LDO3_LOW_POWER: c_uint = 0xD0;
pub const WM8350_LDO4_CONTROL: c_uint = 0xD1;
pub const WM8350_LDO4_TIMEOUTS: c_uint = 0xD2;
pub const WM8350_LDO4_LOW_POWER: c_uint = 0xD3;
pub const WM8350_VCC_FAULT_MASKS: c_uint = 0xD7;
pub const WM8350_MAIN_BANDGAP_CONTROL: c_uint = 0xD8;
pub const WM8350_OSC_CONTROL: c_uint = 0xD9;
pub const WM8350_RTC_TICK_CONTROL: c_uint = 0xDA;
pub const WM8350_SECURITY: c_uint = 0xDB;
pub const WM8350_RAM_BIST_1: c_uint = 0xDC;
pub const WM8350_DCDC_LDO_STATUS: c_uint = 0xE1;
pub const WM8350_GPIO_PIN_STATUS: c_uint = 0xE6;
pub const WM8350_DCDC1_FORCE_PWM: c_uint = 0xF8;
pub const WM8350_DCDC3_FORCE_PWM: c_uint = 0xFA;
pub const WM8350_DCDC4_FORCE_PWM: c_uint = 0xFB;
pub const WM8350_DCDC6_FORCE_PWM: c_uint = 0xFD;
//
// R172 (0xAC) - Current Sink Driver A
//
pub const WM8350_CS1_HIB_MODE: c_uint = 0x1000;
pub const WM8350_CS1_HIB_MODE_MASK: c_uint = 0x1000;
pub const WM8350_CS1_HIB_MODE_SHIFT: c_int = 12;
pub const WM8350_CS1_ISEL_MASK: c_uint = 0x003F;
pub const WM8350_CS1_ISEL_SHIFT: c_int = 0;
// Bit values for R172 (0xAC)
pub const WM8350_CS1_HIB_MODE_DISABLE: c_int = 0;
pub const WM8350_CS1_HIB_MODE_LEAVE: c_int = 1;
pub const WM8350_CS1_ISEL_220M: c_uint = 0x3F;
//
// R173 (0xAD) - CSA Flash control
//
pub const WM8350_CS1_FLASH_MODE: c_uint = 0x8000;
pub const WM8350_CS1_TRIGSRC: c_uint = 0x4000;
pub const WM8350_CS1_DRIVE: c_uint = 0x2000;
pub const WM8350_CS1_FLASH_DUR_MASK: c_uint = 0x0300;
pub const WM8350_CS1_OFF_RAMP_MASK: c_uint = 0x0030;
pub const WM8350_CS1_ON_RAMP_MASK: c_uint = 0x0003;
//
// R174 (0xAE) - Current Sink Driver B
//
pub const WM8350_CS2_HIB_MODE: c_uint = 0x1000;
pub const WM8350_CS2_ISEL_MASK: c_uint = 0x003F;
//
// R175 (0xAF) - CSB Flash control
//
pub const WM8350_CS2_FLASH_MODE: c_uint = 0x8000;
pub const WM8350_CS2_TRIGSRC: c_uint = 0x4000;
pub const WM8350_CS2_DRIVE: c_uint = 0x2000;
pub const WM8350_CS2_FLASH_DUR_MASK: c_uint = 0x0300;
pub const WM8350_CS2_OFF_RAMP_MASK: c_uint = 0x0030;
pub const WM8350_CS2_ON_RAMP_MASK: c_uint = 0x0003;
//
// R176 (0xB0) - DCDC/LDO requested
//
pub const WM8350_LS_ENA: c_uint = 0x8000;
pub const WM8350_LDO4_ENA: c_uint = 0x0800;
pub const WM8350_LDO3_ENA: c_uint = 0x0400;
pub const WM8350_LDO2_ENA: c_uint = 0x0200;
pub const WM8350_LDO1_ENA: c_uint = 0x0100;
pub const WM8350_DC6_ENA: c_uint = 0x0020;
pub const WM8350_DC5_ENA: c_uint = 0x0010;
pub const WM8350_DC4_ENA: c_uint = 0x0008;
pub const WM8350_DC3_ENA: c_uint = 0x0004;
pub const WM8350_DC2_ENA: c_uint = 0x0002;
pub const WM8350_DC1_ENA: c_uint = 0x0001;
//
// R177 (0xB1) - DCDC Active options
//
pub const WM8350_PUTO_MASK: c_uint = 0x3000;
pub const WM8350_PWRUP_DELAY_MASK: c_uint = 0x0300;
pub const WM8350_DC6_ACTIVE: c_uint = 0x0020;
pub const WM8350_DC4_ACTIVE: c_uint = 0x0008;
pub const WM8350_DC3_ACTIVE: c_uint = 0x0004;
pub const WM8350_DC1_ACTIVE: c_uint = 0x0001;
//
// R178 (0xB2) - DCDC Sleep options
//
pub const WM8350_DC6_SLEEP: c_uint = 0x0020;
pub const WM8350_DC4_SLEEP: c_uint = 0x0008;
pub const WM8350_DC3_SLEEP: c_uint = 0x0004;
pub const WM8350_DC1_SLEEP: c_uint = 0x0001;
//
// R179 (0xB3) - Power-check comparator
//
pub const WM8350_PCCMP_ERRACT: c_uint = 0x4000;
pub const WM8350_PCCMP_RAIL: c_uint = 0x0100;
pub const WM8350_PCCMP_OFF_THR_MASK: c_uint = 0x0070;
pub const WM8350_PCCMP_ON_THR_MASK: c_uint = 0x0007;
//
// R180 (0xB4) - DCDC1 Control
//
pub const WM8350_DC1_OPFLT: c_uint = 0x0400;
pub const WM8350_DC1_VSEL_MASK: c_uint = 0x007F;
pub const WM8350_DC1_VSEL_SHIFT: c_int = 0;
//
// R181 (0xB5) - DCDC1 Timeouts
//
pub const WM8350_DC1_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC1_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC1_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC1_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC1_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC1_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_DC1_SDSLOT_SHIFT: c_int = 6;
// Bit values for R181 (0xB5)
pub const WM8350_DC1_ERRACT_NONE: c_int = 0;
pub const WM8350_DC1_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC1_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R182 (0xB6) - DCDC1 Low Power
//
pub const WM8350_DC1_HIB_MODE_MASK: c_uint = 0x7000;
pub const WM8350_DC1_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC1_VIMG_MASK: c_uint = 0x007F;
//
// R183 (0xB7) - DCDC2 Control
//
pub const WM8350_DC2_MODE: c_uint = 0x4000;
pub const WM8350_DC2_MODE_MASK: c_uint = 0x4000;
pub const WM8350_DC2_MODE_SHIFT: c_int = 14;
pub const WM8350_DC2_HIB_MODE: c_uint = 0x1000;
pub const WM8350_DC2_HIB_MODE_MASK: c_uint = 0x1000;
pub const WM8350_DC2_HIB_MODE_SHIFT: c_int = 12;
pub const WM8350_DC2_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC2_HIB_TRIG_SHIFT: c_int = 8;
pub const WM8350_DC2_ILIM: c_uint = 0x0040;
pub const WM8350_DC2_ILIM_MASK: c_uint = 0x0040;
pub const WM8350_DC2_ILIM_SHIFT: c_int = 6;
pub const WM8350_DC2_RMP_MASK: c_uint = 0x0018;
pub const WM8350_DC2_RMP_SHIFT: c_int = 3;
pub const WM8350_DC2_FBSRC_MASK: c_uint = 0x0003;
pub const WM8350_DC2_FBSRC_SHIFT: c_int = 0;
// Bit values for R183 (0xB7)
pub const WM8350_DC2_MODE_BOOST: c_int = 0;
pub const WM8350_DC2_MODE_SWITCH: c_int = 1;
pub const WM8350_DC2_HIB_MODE_ACTIVE: c_int = 1;
pub const WM8350_DC2_HIB_MODE_DISABLE: c_int = 0;
pub const WM8350_DC2_HIB_TRIG_NONE: c_int = 0;
pub const WM8350_DC2_HIB_TRIG_LPWR1: c_int = 1;
pub const WM8350_DC2_HIB_TRIG_LPWR2: c_int = 2;
pub const WM8350_DC2_HIB_TRIG_LPWR3: c_int = 3;
pub const WM8350_DC2_ILIM_HIGH: c_int = 0;
pub const WM8350_DC2_ILIM_LOW: c_int = 1;
pub const WM8350_DC2_RMP_30V: c_int = 0;
pub const WM8350_DC2_RMP_20V: c_int = 1;
pub const WM8350_DC2_RMP_10V: c_int = 2;
pub const WM8350_DC2_RMP_5V: c_int = 3;
pub const WM8350_DC2_FBSRC_FB2: c_int = 0;
pub const WM8350_DC2_FBSRC_ISINKA: c_int = 1;
pub const WM8350_DC2_FBSRC_ISINKB: c_int = 2;
pub const WM8350_DC2_FBSRC_USB: c_int = 3;
//
// R184 (0xB8) - DCDC2 Timeouts
//
pub const WM8350_DC2_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC2_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC2_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC2_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC2_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC2_UVTO_MASK: c_uint = 0x0030;
// Bit values for R184 (0xB8)
pub const WM8350_DC2_ERRACT_NONE: c_int = 0;
pub const WM8350_DC2_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC2_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R186 (0xBA) - DCDC3 Control
//
pub const WM8350_DC3_OPFLT: c_uint = 0x0400;
pub const WM8350_DC3_VSEL_MASK: c_uint = 0x007F;
pub const WM8350_DC3_VSEL_SHIFT: c_int = 0;
//
// R187 (0xBB) - DCDC3 Timeouts
//
pub const WM8350_DC3_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC3_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC3_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC3_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC3_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC3_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_DC3_SDSLOT_SHIFT: c_int = 6;
// Bit values for R187 (0xBB)
pub const WM8350_DC3_ERRACT_NONE: c_int = 0;
pub const WM8350_DC3_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC3_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R188 (0xBC) - DCDC3 Low Power
//
pub const WM8350_DC3_HIB_MODE_MASK: c_uint = 0x7000;
pub const WM8350_DC3_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC3_VIMG_MASK: c_uint = 0x007F;
//
// R189 (0xBD) - DCDC4 Control
//
pub const WM8350_DC4_OPFLT: c_uint = 0x0400;
pub const WM8350_DC4_VSEL_MASK: c_uint = 0x007F;
pub const WM8350_DC4_VSEL_SHIFT: c_int = 0;
//
// R190 (0xBE) - DCDC4 Timeouts
//
pub const WM8350_DC4_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC4_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC4_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC4_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC4_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC4_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_DC4_SDSLOT_SHIFT: c_int = 6;
// Bit values for R190 (0xBE)
pub const WM8350_DC4_ERRACT_NONE: c_int = 0;
pub const WM8350_DC4_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC4_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R191 (0xBF) - DCDC4 Low Power
//
pub const WM8350_DC4_HIB_MODE_MASK: c_uint = 0x7000;
pub const WM8350_DC4_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC4_VIMG_MASK: c_uint = 0x007F;
//
// R192 (0xC0) - DCDC5 Control
//
pub const WM8350_DC5_MODE: c_uint = 0x4000;
pub const WM8350_DC5_MODE_MASK: c_uint = 0x4000;
pub const WM8350_DC5_MODE_SHIFT: c_int = 14;
pub const WM8350_DC5_HIB_MODE: c_uint = 0x1000;
pub const WM8350_DC5_HIB_MODE_MASK: c_uint = 0x1000;
pub const WM8350_DC5_HIB_MODE_SHIFT: c_int = 12;
pub const WM8350_DC5_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC5_HIB_TRIG_SHIFT: c_int = 8;
pub const WM8350_DC5_ILIM: c_uint = 0x0040;
pub const WM8350_DC5_ILIM_MASK: c_uint = 0x0040;
pub const WM8350_DC5_ILIM_SHIFT: c_int = 6;
pub const WM8350_DC5_RMP_MASK: c_uint = 0x0018;
pub const WM8350_DC5_RMP_SHIFT: c_int = 3;
pub const WM8350_DC5_FBSRC_MASK: c_uint = 0x0003;
pub const WM8350_DC5_FBSRC_SHIFT: c_int = 0;
// Bit values for R192 (0xC0)
pub const WM8350_DC5_MODE_BOOST: c_int = 0;
pub const WM8350_DC5_MODE_SWITCH: c_int = 1;
pub const WM8350_DC5_HIB_MODE_ACTIVE: c_int = 1;
pub const WM8350_DC5_HIB_MODE_DISABLE: c_int = 0;
pub const WM8350_DC5_HIB_TRIG_NONE: c_int = 0;
pub const WM8350_DC5_HIB_TRIG_LPWR1: c_int = 1;
pub const WM8350_DC5_HIB_TRIG_LPWR2: c_int = 2;
pub const WM8350_DC5_HIB_TRIG_LPWR3: c_int = 3;
pub const WM8350_DC5_ILIM_HIGH: c_int = 0;
pub const WM8350_DC5_ILIM_LOW: c_int = 1;
pub const WM8350_DC5_RMP_30V: c_int = 0;
pub const WM8350_DC5_RMP_20V: c_int = 1;
pub const WM8350_DC5_RMP_10V: c_int = 2;
pub const WM8350_DC5_RMP_5V: c_int = 3;
pub const WM8350_DC5_FBSRC_FB2: c_int = 0;
pub const WM8350_DC5_FBSRC_ISINKA: c_int = 1;
pub const WM8350_DC5_FBSRC_ISINKB: c_int = 2;
pub const WM8350_DC5_FBSRC_USB: c_int = 3;
//
// R193 (0xC1) - DCDC5 Timeouts
//
pub const WM8350_DC5_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC5_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC5_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC5_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC5_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC5_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_DC5_SDSLOT_SHIFT: c_int = 6;
// Bit values for R193 (0xC1)
pub const WM8350_DC5_ERRACT_NONE: c_int = 0;
pub const WM8350_DC5_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC5_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R195 (0xC3) - DCDC6 Control
//
pub const WM8350_DC6_OPFLT: c_uint = 0x0400;
pub const WM8350_DC6_VSEL_MASK: c_uint = 0x007F;
pub const WM8350_DC6_VSEL_SHIFT: c_int = 0;
//
// R196 (0xC4) - DCDC6 Timeouts
//
pub const WM8350_DC6_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_DC6_ERRACT_SHIFT: c_int = 14;
pub const WM8350_DC6_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_DC6_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_DC6_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_DC6_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_DC6_SDSLOT_SHIFT: c_int = 6;
// Bit values for R196 (0xC4)
pub const WM8350_DC6_ERRACT_NONE: c_int = 0;
pub const WM8350_DC6_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_DC6_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R197 (0xC5) - DCDC6 Low Power
//
pub const WM8350_DC6_HIB_MODE_MASK: c_uint = 0x7000;
pub const WM8350_DC6_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_DC6_VIMG_MASK: c_uint = 0x007F;
//
// R199 (0xC7) - Limit Switch Control
//
pub const WM8350_LS_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_LS_ERRACT_SHIFT: c_int = 14;
pub const WM8350_LS_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_LS_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_LS_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_LS_SDSLOT_SHIFT: c_int = 6;
pub const WM8350_LS_HIB_MODE: c_uint = 0x0010;
pub const WM8350_LS_HIB_MODE_MASK: c_uint = 0x0010;
pub const WM8350_LS_HIB_MODE_SHIFT: c_int = 4;
pub const WM8350_LS_HIB_PROT: c_uint = 0x0002;
pub const WM8350_LS_HIB_PROT_MASK: c_uint = 0x0002;
pub const WM8350_LS_HIB_PROT_SHIFT: c_int = 1;
pub const WM8350_LS_PROT: c_uint = 0x0001;
pub const WM8350_LS_PROT_MASK: c_uint = 0x0001;
pub const WM8350_LS_PROT_SHIFT: c_int = 0;
// Bit values for R199 (0xC7)
pub const WM8350_LS_ERRACT_NONE: c_int = 0;
pub const WM8350_LS_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_LS_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R200 (0xC8) - LDO1 Control
//
pub const WM8350_LDO1_SWI: c_uint = 0x4000;
pub const WM8350_LDO1_OPFLT: c_uint = 0x0400;
pub const WM8350_LDO1_VSEL_MASK: c_uint = 0x001F;
pub const WM8350_LDO1_VSEL_SHIFT: c_int = 0;
//
// R201 (0xC9) - LDO1 Timeouts
//
pub const WM8350_LDO1_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_LDO1_ERRACT_SHIFT: c_int = 14;
pub const WM8350_LDO1_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_LDO1_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_LDO1_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_LDO1_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_LDO1_SDSLOT_SHIFT: c_int = 6;
// Bit values for R201 (0xC9)
pub const WM8350_LDO1_ERRACT_NONE: c_int = 0;
pub const WM8350_LDO1_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_LDO1_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R202 (0xCA) - LDO1 Low Power
//
pub const WM8350_LDO1_HIB_MODE_MASK: c_uint = 0x3000;
pub const WM8350_LDO1_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_LDO1_VIMG_MASK: c_uint = 0x001F;

//
// R203 (0xCB) - LDO2 Control
//
pub const WM8350_LDO2_SWI: c_uint = 0x4000;
pub const WM8350_LDO2_OPFLT: c_uint = 0x0400;
pub const WM8350_LDO2_VSEL_MASK: c_uint = 0x001F;
pub const WM8350_LDO2_VSEL_SHIFT: c_int = 0;
//
// R204 (0xCC) - LDO2 Timeouts
//
pub const WM8350_LDO2_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_LDO2_ERRACT_SHIFT: c_int = 14;
pub const WM8350_LDO2_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_LDO2_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_LDO2_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_LDO2_SDSLOT_SHIFT: c_int = 6;
// Bit values for R204 (0xCC)
pub const WM8350_LDO2_ERRACT_NONE: c_int = 0;
pub const WM8350_LDO2_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_LDO2_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R205 (0xCD) - LDO2 Low Power
//
pub const WM8350_LDO2_HIB_MODE_MASK: c_uint = 0x3000;
pub const WM8350_LDO2_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_LDO2_VIMG_MASK: c_uint = 0x001F;
//
// R206 (0xCE) - LDO3 Control
//
pub const WM8350_LDO3_SWI: c_uint = 0x4000;
pub const WM8350_LDO3_OPFLT: c_uint = 0x0400;
pub const WM8350_LDO3_VSEL_MASK: c_uint = 0x001F;
pub const WM8350_LDO3_VSEL_SHIFT: c_int = 0;
//
// R207 (0xCF) - LDO3 Timeouts
//
pub const WM8350_LDO3_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_LDO3_ERRACT_SHIFT: c_int = 14;
pub const WM8350_LDO3_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_LDO3_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_LDO3_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_LDO3_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_LDO3_SDSLOT_SHIFT: c_int = 6;
// Bit values for R207 (0xCF)
pub const WM8350_LDO3_ERRACT_NONE: c_int = 0;
pub const WM8350_LDO3_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_LDO3_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R208 (0xD0) - LDO3 Low Power
//
pub const WM8350_LDO3_HIB_MODE_MASK: c_uint = 0x3000;
pub const WM8350_LDO3_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_LDO3_VIMG_MASK: c_uint = 0x001F;
//
// R209 (0xD1) - LDO4 Control
//
pub const WM8350_LDO4_SWI: c_uint = 0x4000;
pub const WM8350_LDO4_OPFLT: c_uint = 0x0400;
pub const WM8350_LDO4_VSEL_MASK: c_uint = 0x001F;
pub const WM8350_LDO4_VSEL_SHIFT: c_int = 0;
//
// R210 (0xD2) - LDO4 Timeouts
//
pub const WM8350_LDO4_ERRACT_MASK: c_uint = 0xC000;
pub const WM8350_LDO4_ERRACT_SHIFT: c_int = 14;
pub const WM8350_LDO4_ENSLOT_MASK: c_uint = 0x3C00;
pub const WM8350_LDO4_ENSLOT_SHIFT: c_int = 10;
pub const WM8350_LDO4_SDSLOT_MASK: c_uint = 0x03C0;
pub const WM8350_LDO4_UVTO_MASK: c_uint = 0x0030;
pub const WM8350_LDO4_SDSLOT_SHIFT: c_int = 6;
// Bit values for R210 (0xD2)
pub const WM8350_LDO4_ERRACT_NONE: c_int = 0;
pub const WM8350_LDO4_ERRACT_SHUTDOWN_CONV: c_int = 1;
pub const WM8350_LDO4_ERRACT_SHUTDOWN_SYS: c_int = 2;
//
// R211 (0xD3) - LDO4 Low Power
//
pub const WM8350_LDO4_HIB_MODE_MASK: c_uint = 0x3000;
pub const WM8350_LDO4_HIB_TRIG_MASK: c_uint = 0x0300;
pub const WM8350_LDO4_VIMG_MASK: c_uint = 0x001F;
//
// R215 (0xD7) - VCC_FAULT Masks
//
pub const WM8350_LS_FAULT: c_uint = 0x8000;
pub const WM8350_LDO4_FAULT: c_uint = 0x0800;
pub const WM8350_LDO3_FAULT: c_uint = 0x0400;
pub const WM8350_LDO2_FAULT: c_uint = 0x0200;
pub const WM8350_LDO1_FAULT: c_uint = 0x0100;
pub const WM8350_DC6_FAULT: c_uint = 0x0020;
pub const WM8350_DC5_FAULT: c_uint = 0x0010;
pub const WM8350_DC4_FAULT: c_uint = 0x0008;
pub const WM8350_DC3_FAULT: c_uint = 0x0004;
pub const WM8350_DC2_FAULT: c_uint = 0x0002;
pub const WM8350_DC1_FAULT: c_uint = 0x0001;
//
// R216 (0xD8) - Main Bandgap Control
//
pub const WM8350_MBG_LOAD_FUSES: c_uint = 0x8000;
pub const WM8350_MBG_FUSE_WPREP: c_uint = 0x4000;
pub const WM8350_MBG_FUSE_WRITE: c_uint = 0x2000;
pub const WM8350_MBG_FUSE_TRIM_MASK: c_uint = 0x1F00;
pub const WM8350_MBG_TRIM_SRC: c_uint = 0x0020;
pub const WM8350_MBG_USER_TRIM_MASK: c_uint = 0x001F;
//
// R217 (0xD9) - OSC Control
//
pub const WM8350_OSC_LOAD_FUSES: c_uint = 0x8000;
pub const WM8350_OSC_FUSE_WPREP: c_uint = 0x4000;
pub const WM8350_OSC_FUSE_WRITE: c_uint = 0x2000;
pub const WM8350_OSC_FUSE_TRIM_MASK: c_uint = 0x0F00;
pub const WM8350_OSC_TRIM_SRC: c_uint = 0x0020;
pub const WM8350_OSC_USER_TRIM_MASK: c_uint = 0x000F;
//
// R248 (0xF8) - DCDC1 Force PWM
//
pub const WM8350_DCDC1_FORCE_PWM_ENA: c_uint = 0x0010;
//
// R250 (0xFA) - DCDC3 Force PWM
//
pub const WM8350_DCDC3_FORCE_PWM_ENA: c_uint = 0x0010;
//
// R251 (0xFB) - DCDC4 Force PWM
//
pub const WM8350_DCDC4_FORCE_PWM_ENA: c_uint = 0x0010;
//
// R253 (0xFD) - DCDC1 Force PWM
//
pub const WM8350_DCDC6_FORCE_PWM_ENA: c_uint = 0x0010;
//
// DCDC's
//
pub const WM8350_DCDC_1: c_int = 0;
pub const WM8350_DCDC_2: c_int = 1;
pub const WM8350_DCDC_3: c_int = 2;
pub const WM8350_DCDC_4: c_int = 3;
pub const WM8350_DCDC_5: c_int = 4;
pub const WM8350_DCDC_6: c_int = 5;
// DCDC modes
pub const WM8350_DCDC_ACTIVE_STANDBY: c_int = 0;
pub const WM8350_DCDC_ACTIVE_PULSE: c_int = 1;
pub const WM8350_DCDC_SLEEP_NORMAL: c_int = 0;
pub const WM8350_DCDC_SLEEP_LOW: c_int = 1;
// DCDC Low power (Hibernate) mode

// DCDC Low Power (Hibernate) signal

// LDO Low power (Hibernate) mode

// LDO Low Power (Hibernate) signal

//
// LDOs
//
pub const WM8350_LDO_1: c_int = 6;
pub const WM8350_LDO_2: c_int = 7;
pub const WM8350_LDO_3: c_int = 8;
pub const WM8350_LDO_4: c_int = 9;
//
// ISINKs
//
pub const WM8350_ISINK_A: c_int = 10;
pub const WM8350_ISINK_B: c_int = 11;
pub const WM8350_ISINK_MODE_BOOST: c_int = 0;
pub const WM8350_ISINK_MODE_SWITCH: c_int = 1;
pub const WM8350_ISINK_ILIM_NORMAL: c_int = 0;
pub const WM8350_ISINK_ILIM_LOW: c_int = 1;
pub const WM8350_ISINK_FLASH_DISABLE: c_int = 0;
pub const WM8350_ISINK_FLASH_ENABLE: c_int = 1;
pub const WM8350_ISINK_FLASH_TRIG_BIT: c_int = 0;
pub const WM8350_ISINK_FLASH_TRIG_GPIO: c_int = 1;

//
// Regulator Interrupts.
//
pub const WM8350_IRQ_CS1: c_int = 13;
pub const WM8350_IRQ_CS2: c_int = 14;
pub const WM8350_IRQ_UV_LDO4: c_int = 25;
pub const WM8350_IRQ_UV_LDO3: c_int = 26;
pub const WM8350_IRQ_UV_LDO2: c_int = 27;
pub const WM8350_IRQ_UV_LDO1: c_int = 28;
pub const WM8350_IRQ_UV_DC6: c_int = 29;
pub const WM8350_IRQ_UV_DC5: c_int = 30;
pub const WM8350_IRQ_UV_DC4: c_int = 31;
pub const WM8350_IRQ_UV_DC3: c_int = 32;
pub const WM8350_IRQ_UV_DC2: c_int = 33;
pub const WM8350_IRQ_UV_DC1: c_int = 34;
pub const WM8350_IRQ_OC_LS: c_int = 35;
pub const NUM_WM8350_REGULATORS: c_int = 12;
//
// WM8350 LED platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_led_platform_data {
    pub name: *const c_char,
    pub default_trigger: *const c_char,
    pub max_uA: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_led {
    pub pdev: *mut platform_device,
    pub work: work_struct,
    pub value_lock: spinlock_t,
    pub value: led_brightness,
    pub cdev: led_classdev,
    pub max_uA_index: c_int,
    pub enabled: c_int,
    pub isink: *mut regulator,
    pub isink_consumer: regulator_consumer_supply,
    pub isink_init: regulator_init_data,
    pub dcdc: *mut regulator,
    pub dcdc_consumer: regulator_consumer_supply,
    pub dcdc_init: regulator_init_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_pmic {
// Number of regulators of each type on this device
    pub max_dcdc: c_int,
    pub max_isink: c_int,
// ISINK to DCDC mapping
    pub isink_A_dcdc: c_int,
    pub isink_B_dcdc: c_int,
// hibernate configs
    pub dcdc1_hib_mode: u16,
    pub dcdc3_hib_mode: u16,
    pub dcdc4_hib_mode: u16,
    pub dcdc6_hib_mode: u16,
// regulator devices
    pub pdev: [*mut platform_device; NUM_WM8350_REGULATORS],
// LED devices
    pub led: [wm8350_led; 2],
}

//
// Additional DCDC control not supported via regulator API
//
// Additional LDO control not supported via regulator API
//
extern "C" {
    pub fn wm8350_ldo_set_slot(wm8350: *mut wm8350, ldo: c_int, start: u16, stop: u16) -> c_int;
}
//
// Additional ISINK control not supported via regulator API
//
