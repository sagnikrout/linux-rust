//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rc5t583.h
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
// Core driver interface to access RICOH_RC5T583 power management chip.
//
// Copyright (c) 2011-2012, NVIDIA CORPORATION.  All rights reserved.
// Author: Laxman dewangan <ldewangan@nvidia.com>
//
// Based on code
// Copyright (C) 2011 RICOH COMPANY,LTD
//

// Maximum number of main interrupts
pub const MAX_MAIN_INTERRUPT: c_int = 5;
pub const RC5T583_MAX_GPEDGE_REG: c_int = 2;
pub const RC5T583_MAX_INTERRUPT_EN_REGS: c_int = 8;
pub const RC5T583_MAX_INTERRUPT_MASK_REGS: c_int = 9;
// Interrupt enable register
pub const RC5T583_INT_EN_SYS1: c_uint = 0x19;
pub const RC5T583_INT_EN_SYS2: c_uint = 0x1D;
pub const RC5T583_INT_EN_DCDC: c_uint = 0x41;
pub const RC5T583_INT_EN_RTC: c_uint = 0xED;
pub const RC5T583_INT_EN_ADC1: c_uint = 0x90;
pub const RC5T583_INT_EN_ADC2: c_uint = 0x91;
pub const RC5T583_INT_EN_ADC3: c_uint = 0x92;
// Interrupt status registers (monitor regs in Ricoh)
pub const RC5T583_INTC_INTPOL: c_uint = 0xAD;
pub const RC5T583_INTC_INTEN: c_uint = 0xAE;
pub const RC5T583_INTC_INTMON: c_uint = 0xAF;
pub const RC5T583_INT_MON_GRP: c_uint = 0xAF;
pub const RC5T583_INT_MON_SYS1: c_uint = 0x1B;
pub const RC5T583_INT_MON_SYS2: c_uint = 0x1F;
pub const RC5T583_INT_MON_DCDC: c_uint = 0x43;
pub const RC5T583_INT_MON_RTC: c_uint = 0xEE;
// Interrupt clearing registers
pub const RC5T583_INT_IR_SYS1: c_uint = 0x1A;
pub const RC5T583_INT_IR_SYS2: c_uint = 0x1E;
pub const RC5T583_INT_IR_DCDC: c_uint = 0x42;
pub const RC5T583_INT_IR_RTC: c_uint = 0xEE;
pub const RC5T583_INT_IR_ADCL: c_uint = 0x94;
pub const RC5T583_INT_IR_ADCH: c_uint = 0x95;
pub const RC5T583_INT_IR_ADCEND: c_uint = 0x96;
pub const RC5T583_INT_IR_GPIOR: c_uint = 0xA9;
pub const RC5T583_INT_IR_GPIOF: c_uint = 0xAA;
// Sleep sequence registers
pub const RC5T583_SLPSEQ1: c_uint = 0x21;
pub const RC5T583_SLPSEQ2: c_uint = 0x22;
pub const RC5T583_SLPSEQ3: c_uint = 0x23;
pub const RC5T583_SLPSEQ4: c_uint = 0x24;
pub const RC5T583_SLPSEQ5: c_uint = 0x25;
pub const RC5T583_SLPSEQ6: c_uint = 0x26;
pub const RC5T583_SLPSEQ7: c_uint = 0x27;
pub const RC5T583_SLPSEQ8: c_uint = 0x28;
pub const RC5T583_SLPSEQ9: c_uint = 0x29;
pub const RC5T583_SLPSEQ10: c_uint = 0x2A;
pub const RC5T583_SLPSEQ11: c_uint = 0x2B;
// Regulator registers
pub const RC5T583_REG_DC0CTL: c_uint = 0x30;
pub const RC5T583_REG_DC0DAC: c_uint = 0x31;
pub const RC5T583_REG_DC0LATCTL: c_uint = 0x32;
pub const RC5T583_REG_SR0CTL: c_uint = 0x33;
pub const RC5T583_REG_DC1CTL: c_uint = 0x34;
pub const RC5T583_REG_DC1DAC: c_uint = 0x35;
pub const RC5T583_REG_DC1LATCTL: c_uint = 0x36;
pub const RC5T583_REG_SR1CTL: c_uint = 0x37;
pub const RC5T583_REG_DC2CTL: c_uint = 0x38;
pub const RC5T583_REG_DC2DAC: c_uint = 0x39;
pub const RC5T583_REG_DC2LATCTL: c_uint = 0x3A;
pub const RC5T583_REG_SR2CTL: c_uint = 0x3B;
pub const RC5T583_REG_DC3CTL: c_uint = 0x3C;
pub const RC5T583_REG_DC3DAC: c_uint = 0x3D;
pub const RC5T583_REG_DC3LATCTL: c_uint = 0x3E;
pub const RC5T583_REG_SR3CTL: c_uint = 0x3F;
pub const RC5T583_REG_LDOEN1: c_uint = 0x50;
pub const RC5T583_REG_LDOEN2: c_uint = 0x51;
pub const RC5T583_REG_LDODIS1: c_uint = 0x52;
pub const RC5T583_REG_LDODIS2: c_uint = 0x53;
pub const RC5T583_REG_LDO0DAC: c_uint = 0x54;
pub const RC5T583_REG_LDO1DAC: c_uint = 0x55;
pub const RC5T583_REG_LDO2DAC: c_uint = 0x56;
pub const RC5T583_REG_LDO3DAC: c_uint = 0x57;
pub const RC5T583_REG_LDO4DAC: c_uint = 0x58;
pub const RC5T583_REG_LDO5DAC: c_uint = 0x59;
pub const RC5T583_REG_LDO6DAC: c_uint = 0x5A;
pub const RC5T583_REG_LDO7DAC: c_uint = 0x5B;
pub const RC5T583_REG_LDO8DAC: c_uint = 0x5C;
pub const RC5T583_REG_LDO9DAC: c_uint = 0x5D;
pub const RC5T583_REG_DC0DAC_DS: c_uint = 0x60;
pub const RC5T583_REG_DC1DAC_DS: c_uint = 0x61;
pub const RC5T583_REG_DC2DAC_DS: c_uint = 0x62;
pub const RC5T583_REG_DC3DAC_DS: c_uint = 0x63;
pub const RC5T583_REG_LDO0DAC_DS: c_uint = 0x64;
pub const RC5T583_REG_LDO1DAC_DS: c_uint = 0x65;
pub const RC5T583_REG_LDO2DAC_DS: c_uint = 0x66;
pub const RC5T583_REG_LDO3DAC_DS: c_uint = 0x67;
pub const RC5T583_REG_LDO4DAC_DS: c_uint = 0x68;
pub const RC5T583_REG_LDO5DAC_DS: c_uint = 0x69;
pub const RC5T583_REG_LDO6DAC_DS: c_uint = 0x6A;
pub const RC5T583_REG_LDO7DAC_DS: c_uint = 0x6B;
pub const RC5T583_REG_LDO8DAC_DS: c_uint = 0x6C;
pub const RC5T583_REG_LDO9DAC_DS: c_uint = 0x6D;
// GPIO register base address
pub const RC5T583_GPIO_IOSEL: c_uint = 0xA0;
pub const RC5T583_GPIO_PDEN: c_uint = 0xA1;
pub const RC5T583_GPIO_IOOUT: c_uint = 0xA2;
pub const RC5T583_GPIO_PGSEL: c_uint = 0xA3;
pub const RC5T583_GPIO_GPINV: c_uint = 0xA4;
pub const RC5T583_GPIO_GPDEB: c_uint = 0xA5;
pub const RC5T583_GPIO_GPEDGE1: c_uint = 0xA6;
pub const RC5T583_GPIO_GPEDGE2: c_uint = 0xA7;
pub const RC5T583_GPIO_EN_INT: c_uint = 0xA8;
pub const RC5T583_GPIO_MON_IOIN: c_uint = 0xAB;
pub const RC5T583_GPIO_GPOFUNC: c_uint = 0xAC;
// RTC registers
pub const RC5T583_RTC_SEC: c_uint = 0xE0;
pub const RC5T583_RTC_MIN: c_uint = 0xE1;
pub const RC5T583_RTC_HOUR: c_uint = 0xE2;
pub const RC5T583_RTC_WDAY: c_uint = 0xE3;
pub const RC5T583_RTC_DAY: c_uint = 0xE4;
pub const RC5T583_RTC_MONTH: c_uint = 0xE5;
pub const RC5T583_RTC_YEAR: c_uint = 0xE6;
pub const RC5T583_RTC_ADJ: c_uint = 0xE7;
pub const RC5T583_RTC_AW_MIN: c_uint = 0xE8;
pub const RC5T583_RTC_AW_HOUR: c_uint = 0xE9;
pub const RC5T583_RTC_AW_WEEK: c_uint = 0xEA;
pub const RC5T583_RTC_AD_MIN: c_uint = 0xEB;
pub const RC5T583_RTC_AD_HOUR: c_uint = 0xEC;
pub const RC5T583_RTC_CTL1: c_uint = 0xED;
pub const RC5T583_RTC_CTL2: c_uint = 0xEE;
pub const RC5T583_RTC_AY_MIN: c_uint = 0xF0;
pub const RC5T583_RTC_AY_HOUR: c_uint = 0xF1;
pub const RC5T583_RTC_AY_DAY: c_uint = 0xF2;
pub const RC5T583_RTC_AY_MONTH: c_uint = 0xF3;
pub const RC5T583_RTC_AY_YEAR: c_uint = 0xF4;
pub const RC5T583_MAX_REG: c_uint = 0xF7;

// RICOH_RC5T583 IRQ definitions
// Should be last entry
// Ricoh583 gpio definitions
// Should be last entry
//
// Ricoh pmic RC5T583 supports sleep through two external controls.
// The output of gpios and regulator can be enable/disable through
// this external signals.
//
// Should be last entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc5t583 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub chip_irq: c_int,
    pub irq_base: c_int,
    pub irq_lock: mutex,
    pub group_irq_en: [c_ulong; MAX_MAIN_INTERRUPT],
// For main interrupt bits in INTC
    pub intc_inten_reg: u8,
// For group interrupt bits and address
    pub irq_en_reg: [u8; RC5T583_MAX_INTERRUPT_EN_REGS],
// For gpio edge
    pub gpedge_reg: [u8; RC5T583_MAX_GPEDGE_REG],
}

//
// rc5t583_platform_data: Platform data for ricoh rc5t583 pmu.
// The board specific data is provided through this structure.
// @irq_base: Irq base number on which this device registers their interrupts.
// @gpio_base: GPIO base from which gpio of this device will start.
// @enable_shutdown: Enable shutdown through the input pin "shutdown".
// @regulator_deepsleep_slot: The slot number on which device goes to sleep
// in device sleep mode.
// @regulator_ext_pwr_control: External power request regulator control. The
// regulator output enable/disable is controlled by the external
// power request input state.
// @reg_init_data: Regulator init data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc5t583_platform_data {
    pub irq_base: c_int,
    pub gpio_base: c_int,
    pub enable_shutdown: bool,
    pub regulator_deepsleep_slot: [c_int; RC5T583_REGULATOR_MAX],
    pub regulator_ext_pwr_control: [c_ulong; RC5T583_REGULATOR_MAX],
    pub reg_init_data: [*mut regulator_init_data; RC5T583_REGULATOR_MAX],
}

extern "C" {
    pub fn regmap_write(_arg: rc5t583->regmap, _arg: reg, _arg: val) -> return;
}
// val = (uint8_t)ival;
extern "C" {
    pub fn regmap_update_bits(_arg: rc5t583->regmap, _arg: reg, _arg: bit_mask, _arg: bit_mask) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: rc5t583->regmap, _arg: reg, _arg: bit_mask, _arg: 0) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: rc5t583->regmap, _arg: reg, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn rc5t583_irq_init(rc5t583: *mut rc5t583, irq: c_int, irq_base: c_int) -> c_int;
}
extern "C" {
    pub fn rc5t583_irq_exit(rc5t583: *mut rc5t583) -> c_int;
}
