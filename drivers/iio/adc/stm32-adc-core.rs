//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/stm32-adc-core.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This file is part of STM32 ADC driver
//
// Copyright (C) 2016, STMicroelectronics - All Rights Reserved
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>.
//

//
// STM32 - ADC global register map
// ________________________________________________________
// | Offset |                 Register                    |
// --------------------------------------------------------
// | 0x000  |                Master ADC1                  |
// --------------------------------------------------------
// | 0x100  |                Slave ADC2                   |
// --------------------------------------------------------
// | 0x200  |                Slave ADC3                   |
// --------------------------------------------------------
// | 0x300  |         Master & Slave common regs          |
// --------------------------------------------------------
//
// Maximum ADC instances number per ADC block for all supported SoCs
pub const STM32_ADC_MAX_ADCS: c_int = 3;
pub const STM32_ADC_OFFSET: c_uint = 0x100;
pub const STM32_ADCX_COMN_OFFSET: c_uint = 0x300;
// STM32F4 - Registers for each ADC instance
pub const STM32F4_ADC_SR: c_uint = 0x00;
pub const STM32F4_ADC_CR1: c_uint = 0x04;
pub const STM32F4_ADC_CR2: c_uint = 0x08;
pub const STM32F4_ADC_SMPR1: c_uint = 0x0C;
pub const STM32F4_ADC_SMPR2: c_uint = 0x10;
pub const STM32F4_ADC_HTR: c_uint = 0x24;
pub const STM32F4_ADC_LTR: c_uint = 0x28;
pub const STM32F4_ADC_SQR1: c_uint = 0x2C;
pub const STM32F4_ADC_SQR2: c_uint = 0x30;
pub const STM32F4_ADC_SQR3: c_uint = 0x34;
pub const STM32F4_ADC_JSQR: c_uint = 0x38;
pub const STM32F4_ADC_JDR1: c_uint = 0x3C;
pub const STM32F4_ADC_JDR2: c_uint = 0x40;
pub const STM32F4_ADC_JDR3: c_uint = 0x44;
pub const STM32F4_ADC_JDR4: c_uint = 0x48;
pub const STM32F4_ADC_DR: c_uint = 0x4C;
// STM32F4 - common registers for all ADC instances: 1, 2 & 3

// STM32F4_ADC_SR - bit fields

// STM32F4_ADC_CR1 - bit fields

pub const STM32F4_RES_SHIFT: c_int = 24;

// STM32F4_ADC_CR2 - bit fields

pub const STM32F4_EXTEN_SHIFT: c_int = 28;

pub const STM32F4_EXTSEL_SHIFT: c_int = 24;

// STM32F4_ADC_CSR - bit fields

// STM32F4_ADC_CCR - bit fields
pub const STM32F4_ADC_ADCPRE_SHIFT: c_int = 16;

// STM32H7 - Registers for each ADC instance
pub const STM32H7_ADC_ISR: c_uint = 0x00;
pub const STM32H7_ADC_IER: c_uint = 0x04;
pub const STM32H7_ADC_CR: c_uint = 0x08;
pub const STM32H7_ADC_CFGR: c_uint = 0x0C;
pub const STM32H7_ADC_CFGR2: c_uint = 0x10;
pub const STM32H7_ADC_SMPR1: c_uint = 0x14;
pub const STM32H7_ADC_SMPR2: c_uint = 0x18;
pub const STM32H7_ADC_PCSEL: c_uint = 0x1C;
pub const STM32H7_ADC_SQR1: c_uint = 0x30;
pub const STM32H7_ADC_SQR2: c_uint = 0x34;
pub const STM32H7_ADC_SQR3: c_uint = 0x38;
pub const STM32H7_ADC_SQR4: c_uint = 0x3C;
pub const STM32H7_ADC_DR: c_uint = 0x40;
pub const STM32H7_ADC_DIFSEL: c_uint = 0xC0;
pub const STM32H7_ADC_CALFACT: c_uint = 0xC4;
pub const STM32H7_ADC_CALFACT2: c_uint = 0xC8;
// STM32MP1 - ADC2 instance option register
pub const STM32MP1_ADC2_OR: c_uint = 0xD0;
// STM32MP1 - Identification registers
pub const STM32MP1_ADC_HWCFGR0: c_uint = 0x3F0;
pub const STM32MP1_ADC_VERR: c_uint = 0x3F4;
pub const STM32MP1_ADC_IPDR: c_uint = 0x3F8;
pub const STM32MP1_ADC_SIDR: c_uint = 0x3FC;
// STM32MP13 - Registers for each ADC instance
pub const STM32MP13_ADC_DIFSEL: c_uint = 0xB0;
pub const STM32MP13_ADC_CALFACT: c_uint = 0xB4;
pub const STM32MP13_ADC2_OR: c_uint = 0xC8;
// STM32H7 - common registers for all ADC instances

// STM32H7_ADC_ISR - bit fields

// STM32H7_ADC_IER - bit fields

// STM32H7_ADC_CR - bit fields

// STM32H7_ADC_CFGR bit fields
pub const STM32H7_EXTEN_SHIFT: c_int = 10;

pub const STM32H7_EXTSEL_SHIFT: c_int = 5;

pub const STM32H7_RES_SHIFT: c_int = 2;

pub const STM32H7_DMNGT_SHIFT: c_int = 0;

// STM32H7_ADC_CFGR2 bit fields

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32h7_adc_dmngt {
    STM32H7_DMNGT_DR_ONLY,		/* Regular data in DR only */
    STM32H7_DMNGT_DMA_ONESHOT,	/* DMA one shot mode */
    STM32H7_DMNGT_DFSDM,		/* DFSDM mode */
    STM32H7_DMNGT_DMA_CIRC,		/* DMA circular mode */
}

// STM32H7_ADC_DIFSEL - bit fields

// STM32H7_ADC_CALFACT - bit fields
pub const STM32H7_CALFACT_D_SHIFT: c_int = 16;

pub const STM32H7_CALFACT_S_SHIFT: c_int = 0;

// STM32H7_ADC_CALFACT2 - bit fields
pub const STM32H7_LINCALFACT_SHIFT: c_int = 0;

// STM32H7_ADC_CSR - bit fields

// STM32H7_ADC_CCR - bit fields

pub const STM32H7_PRESC_SHIFT: c_int = 18;

pub const STM32H7_CKMODE_SHIFT: c_int = 16;

// STM32MP1_ADC2_OR - bit fields

// STM32MP1_ADC_HWCFGR0 - bit fields
pub const STM32MP1_ADCNUM_SHIFT: c_int = 0;

pub const STM32MP1_MULPIPE_SHIFT: c_int = 4;

pub const STM32MP1_OPBITS_SHIFT: c_int = 8;

pub const STM32MP1_IDLEVALUE_SHIFT: c_int = 12;

// STM32MP1_ADC_VERR - bit fields
pub const STM32MP1_MINREV_SHIFT: c_int = 0;

pub const STM32MP1_MAJREV_SHIFT: c_int = 4;

// STM32MP1_ADC_IPDR - bit fields

// STM32MP1_ADC_SIDR - bit fields

// STM32MP13_ADC_CFGR specific bit fields

pub const STM32MP13_RES_SHIFT: c_int = 3;

// STM32MP13_ADC_CFGR2 bit fields

// STM32MP13_ADC_DIFSEL - bit fields

// STM32MP13_ADC_CALFACT - bit fields
pub const STM32MP13_CALFACT_D_SHIFT: c_int = 16;

pub const STM32MP13_CALFACT_S_SHIFT: c_int = 0;

// STM32MP13_ADC2_OR - bit fields

pub const STM32MP15_IPIDR_NUMBER: c_uint = 0x00110005;
pub const STM32MP13_IPIDR_NUMBER: c_uint = 0x00110006;
//
// struct stm32_adc_common - stm32 ADC driver common data (for all instances)
// @base:		control registers base cpu addr
// @phys_base:		control registers base physical addr
// @rate:		clock rate used for analog circuitry
// @vref_mv:		vref voltage (mv)
// @lock:		spinlock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_adc_common {
    pub base: *mut void __iomem,
    pub phys_base: phys_addr_t,
    pub rate: c_ulong,
    pub vref_mv: c_int,
    pub /: *mut *mut spinlock_t lock; / lock for common register,
}
