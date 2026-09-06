//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77541.h
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

// REGISTERS
pub const MAX77541_REG_INT_SRC: c_uint = 0x00;
pub const MAX77541_REG_INT_SRC_M: c_uint = 0x01;

pub const MAX77541_REG_TOPSYS_INT: c_uint = 0x02;
pub const MAX77541_REG_TOPSYS_INT_M: c_uint = 0x03;

// REGULATORS
pub const MAX77541_REG_BUCK_INT: c_uint = 0x20;
pub const MAX77541_REG_BUCK_INT_M: c_uint = 0x21;

pub const MAX77541_REG_EN_CTRL: c_uint = 0x0B;

pub const MAX77541_REG_M1_VOUT: c_uint = 0x23;
pub const MAX77541_REG_M2_VOUT: c_uint = 0x33;

pub const MAX77541_REG_M1_CFG1: c_uint = 0x25;
pub const MAX77541_REG_M2_CFG1: c_uint = 0x35;

// ADC
pub const MAX77541_REG_ADC_INT: c_uint = 0x70;
pub const MAX77541_REG_ADC_INT_M: c_uint = 0x71;

pub const MAX77541_REG_ADC_DATA_CH1: c_uint = 0x72;
pub const MAX77541_REG_ADC_DATA_CH2: c_uint = 0x73;
pub const MAX77541_REG_ADC_DATA_CH3: c_uint = 0x74;
pub const MAX77541_REG_ADC_DATA_CH6: c_uint = 0x77;
// INTERRUPT MASKS
pub const MAX77541_REG_INT_SRC_MASK: c_uint = 0x00;
pub const MAX77541_REG_TOPSYS_INT_MASK: c_uint = 0x00;
pub const MAX77541_REG_BUCK_INT_MASK: c_uint = 0x00;
pub const MAX77541_MAX_REGULATORS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max7754x_ids {
    MAX77540 = 1,
    MAX77541,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77541 {
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub id: max7754x_ids,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irq_buck: *mut regmap_irq_chip_data,
    pub irq_topsys: *mut regmap_irq_chip_data,
    pub irq_adc: *mut regmap_irq_chip_data,
}
