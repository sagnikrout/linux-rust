//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/comparator.h
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
// comparator.h  --  Comparator Aux ADC for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//
// Registers
//
pub const WM8350_DIGITISER_CONTROL_1: c_uint = 0x90;
pub const WM8350_DIGITISER_CONTROL_2: c_uint = 0x91;
pub const WM8350_AUX1_READBACK: c_uint = 0x98;
pub const WM8350_AUX2_READBACK: c_uint = 0x99;
pub const WM8350_AUX3_READBACK: c_uint = 0x9A;
pub const WM8350_AUX4_READBACK: c_uint = 0x9B;
pub const WM8350_CHIP_TEMP_READBACK: c_uint = 0x9F;
pub const WM8350_GENERIC_COMPARATOR_CONTROL: c_uint = 0xA3;
pub const WM8350_GENERIC_COMPARATOR_1: c_uint = 0xA4;
pub const WM8350_GENERIC_COMPARATOR_2: c_uint = 0xA5;
pub const WM8350_GENERIC_COMPARATOR_3: c_uint = 0xA6;
pub const WM8350_GENERIC_COMPARATOR_4: c_uint = 0xA7;
//
// R144 (0x90) - Digitiser Control (1)
//
pub const WM8350_AUXADC_CTC: c_uint = 0x4000;
pub const WM8350_AUXADC_POLL: c_uint = 0x2000;
pub const WM8350_AUXADC_HIB_MODE: c_uint = 0x1000;
pub const WM8350_AUXADC_SEL8: c_uint = 0x0080;
pub const WM8350_AUXADC_SEL7: c_uint = 0x0040;
pub const WM8350_AUXADC_SEL6: c_uint = 0x0020;
pub const WM8350_AUXADC_SEL5: c_uint = 0x0010;
pub const WM8350_AUXADC_SEL4: c_uint = 0x0008;
pub const WM8350_AUXADC_SEL3: c_uint = 0x0004;
pub const WM8350_AUXADC_SEL2: c_uint = 0x0002;
pub const WM8350_AUXADC_SEL1: c_uint = 0x0001;
//
// R145 (0x91) - Digitiser Control (2)
//
pub const WM8350_AUXADC_MASKMODE_MASK: c_uint = 0x3000;
pub const WM8350_AUXADC_CRATE_MASK: c_uint = 0x0700;
pub const WM8350_AUXADC_CAL: c_uint = 0x0004;
pub const WM8350_AUX_RBMODE: c_uint = 0x0002;
pub const WM8350_AUXADC_WAIT: c_uint = 0x0001;
//
// R152 (0x98) - AUX1 Readback
//
pub const WM8350_AUXADC_SCALE1_MASK: c_uint = 0x6000;
pub const WM8350_AUXADC_REF1: c_uint = 0x1000;
pub const WM8350_AUXADC_DATA1_MASK: c_uint = 0x0FFF;
//
// R153 (0x99) - AUX2 Readback
//
pub const WM8350_AUXADC_SCALE2_MASK: c_uint = 0x6000;
pub const WM8350_AUXADC_REF2: c_uint = 0x1000;
pub const WM8350_AUXADC_DATA2_MASK: c_uint = 0x0FFF;
//
// R154 (0x9A) - AUX3 Readback
//
pub const WM8350_AUXADC_SCALE3_MASK: c_uint = 0x6000;
pub const WM8350_AUXADC_REF3: c_uint = 0x1000;
pub const WM8350_AUXADC_DATA3_MASK: c_uint = 0x0FFF;
//
// R155 (0x9B) - AUX4 Readback
//
pub const WM8350_AUXADC_SCALE4_MASK: c_uint = 0x6000;
pub const WM8350_AUXADC_REF4: c_uint = 0x1000;
pub const WM8350_AUXADC_DATA4_MASK: c_uint = 0x0FFF;
//
// R156 (0x9C) - USB Voltage Readback
//
pub const WM8350_AUXADC_DATA_USB_MASK: c_uint = 0x0FFF;
//
// R157 (0x9D) - LINE Voltage Readback
//
pub const WM8350_AUXADC_DATA_LINE_MASK: c_uint = 0x0FFF;
//
// R158 (0x9E) - BATT Voltage Readback
//
pub const WM8350_AUXADC_DATA_BATT_MASK: c_uint = 0x0FFF;
//
// R159 (0x9F) - Chip Temp Readback
//
pub const WM8350_AUXADC_DATA_CHIPTEMP_MASK: c_uint = 0x0FFF;
//
// R163 (0xA3) - Generic Comparator Control
//
pub const WM8350_DCMP4_ENA: c_uint = 0x0008;
pub const WM8350_DCMP3_ENA: c_uint = 0x0004;
pub const WM8350_DCMP2_ENA: c_uint = 0x0002;
pub const WM8350_DCMP1_ENA: c_uint = 0x0001;
//
// R164 (0xA4) - Generic comparator 1
//
pub const WM8350_DCMP1_SRCSEL_MASK: c_uint = 0xE000;
pub const WM8350_DCMP1_GT: c_uint = 0x1000;
pub const WM8350_DCMP1_THR_MASK: c_uint = 0x0FFF;
//
// R165 (0xA5) - Generic comparator 2
//
pub const WM8350_DCMP2_SRCSEL_MASK: c_uint = 0xE000;
pub const WM8350_DCMP2_GT: c_uint = 0x1000;
pub const WM8350_DCMP2_THR_MASK: c_uint = 0x0FFF;
//
// R166 (0xA6) - Generic comparator 3
//
pub const WM8350_DCMP3_SRCSEL_MASK: c_uint = 0xE000;
pub const WM8350_DCMP3_GT: c_uint = 0x1000;
pub const WM8350_DCMP3_THR_MASK: c_uint = 0x0FFF;
//
// R167 (0xA7) - Generic comparator 4
//
pub const WM8350_DCMP4_SRCSEL_MASK: c_uint = 0xE000;
pub const WM8350_DCMP4_GT: c_uint = 0x1000;
pub const WM8350_DCMP4_THR_MASK: c_uint = 0x0FFF;
//
// Interrupts.
//
pub const WM8350_IRQ_AUXADC_DATARDY: c_int = 16;
pub const WM8350_IRQ_AUXADC_DCOMP4: c_int = 17;
pub const WM8350_IRQ_AUXADC_DCOMP3: c_int = 18;
pub const WM8350_IRQ_AUXADC_DCOMP2: c_int = 19;
pub const WM8350_IRQ_AUXADC_DCOMP1: c_int = 20;
pub const WM8350_IRQ_SYS_HYST_COMP_FAIL: c_int = 21;
pub const WM8350_IRQ_SYS_CHIP_GT115: c_int = 22;
pub const WM8350_IRQ_SYS_CHIP_GT140: c_int = 23;
//
// USB/2, LINE & BATT = ((VRTC * 2) / 4095)) * 10e6 uV
// Where VRTC = 2.7 V
//
pub const WM8350_AUX_COEFF: c_int = 1319;
pub const WM8350_AUXADC_AUX1: c_int = 0;
pub const WM8350_AUXADC_AUX2: c_int = 1;
pub const WM8350_AUXADC_AUX3: c_int = 2;
pub const WM8350_AUXADC_AUX4: c_int = 3;
pub const WM8350_AUXADC_USB: c_int = 4;
pub const WM8350_AUXADC_LINE: c_int = 5;
pub const WM8350_AUXADC_BATT: c_int = 6;
pub const WM8350_AUXADC_TEMP: c_int = 7;
//
// AUX ADC Readback
//
