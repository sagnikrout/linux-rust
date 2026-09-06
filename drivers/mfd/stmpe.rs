//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/stmpe.h
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

//
// struct stmpe_variant_block - information about block
// @cell:	base mfd cell
// @irq:	interrupt number to be added to each IORESOURCE_IRQ
// in the cell
// @block:	block id; used for identification with platform data and for
// enable and altfunc callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_variant_block {
    pub cell: *const mfd_cell,
    pub irq: c_int,
    pub block: stmpe_block,
}

//
// struct stmpe_variant_info - variant-specific information
// @name:	part name
// @id_val:	content of CHIPID register
// @id_mask:	bits valid in CHIPID register for comparison with id_val
// @num_gpios:	number of GPIOS
// @af_bits:	number of bits used to specify the alternate function
// @regs: variant specific registers.
// @blocks:	list of blocks present on this device
// @num_blocks:	number of blocks present on this device
// @num_irqs:	number of internal IRQs available on this device
// @enable:	callback to enable the specified blocks.
// Called with the I/O lock held.
// @get_altfunc: callback to get the alternate function number for the
// specific block
// @enable_autosleep: callback to configure autosleep with specified timeout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_variant_info {
    pub name: *const c_char,
    pub id_val: u16,
    pub id_mask: u16,
    pub num_gpios: c_int,
    pub af_bits: c_int,
    pub regs: *const u8,
    pub blocks: *mut stmpe_variant_block,
    pub num_blocks: c_int,
    pub num_irqs: c_int,
    pub enable): *mut *mut *mut int (enable)(struct stmpe stmpe, unsigned int blocks, bool,
    pub block): *mut *mut *mut int (get_altfunc)(struct stmpe stmpe, enum stmpe_block,
    pub autosleep_timeout): *mut *mut *mut int (enable_autosleep)(struct stmpe stmpe, int,
}

//
// struct stmpe_client_info - i2c or spi specific routines/info
// @data: client specific data
// @read_byte: read single byte
// @write_byte: write single byte
// @read_block: read block or multiple bytes
// @write_block: write block or multiple bytes
// @init: client init routine, called during probe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_client_info {
    pub data: *mut c_void,
    pub irq: c_int,
    pub client: *mut c_void,
    pub dev: *mut device,
    pub reg): *mut *mut *mut int (read_byte)(struct stmpe stmpe, u8,
    pub val): *mut *mut *mut int (write_byte)(struct stmpe stmpe, u8 reg, u8,
    pub values): *mut *mut *mut int (read_block)(struct stmpe stmpe, u8 reg, u8 len, u8,
    pub values): *const u8,
    pub stmpe): *mut *mut void (init)(struct stmpe,
}

extern "C" {
    pub fn stmpe_probe(ci: *mut stmpe_client_info, partnum: stmpe_partnum) -> c_int;
}
extern "C" {
    pub fn stmpe_remove(stmpe: *mut stmpe);
}

//
// STMPE801
//
pub const STMPE801_ID: c_uint = 0x0108;
pub const STMPE801_NR_INTERNAL_IRQS: c_int = 1;
pub const STMPE801_REG_CHIP_ID: c_uint = 0x00;
pub const STMPE801_REG_VERSION_ID: c_uint = 0x02;
pub const STMPE801_REG_SYS_CTRL: c_uint = 0x04;
pub const STMPE801_REG_GPIO_INT_EN: c_uint = 0x08;
pub const STMPE801_REG_GPIO_INT_STA: c_uint = 0x09;
pub const STMPE801_REG_GPIO_MP_STA: c_uint = 0x10;
pub const STMPE801_REG_GPIO_SET_PIN: c_uint = 0x11;
pub const STMPE801_REG_GPIO_DIR: c_uint = 0x12;
//
// STMPE811
//
pub const STMPE811_ID: c_uint = 0x0811;
pub const STMPE811_IRQ_TOUCH_DET: c_int = 0;
pub const STMPE811_IRQ_FIFO_TH: c_int = 1;
pub const STMPE811_IRQ_FIFO_OFLOW: c_int = 2;
pub const STMPE811_IRQ_FIFO_FULL: c_int = 3;
pub const STMPE811_IRQ_FIFO_EMPTY: c_int = 4;
pub const STMPE811_IRQ_TEMP_SENS: c_int = 5;
pub const STMPE811_IRQ_ADC: c_int = 6;
pub const STMPE811_IRQ_GPIOC: c_int = 7;
pub const STMPE811_NR_INTERNAL_IRQS: c_int = 8;
pub const STMPE811_REG_CHIP_ID: c_uint = 0x00;
pub const STMPE811_REG_SYS_CTRL: c_uint = 0x03;
pub const STMPE811_REG_SYS_CTRL2: c_uint = 0x04;
pub const STMPE811_REG_SPI_CFG: c_uint = 0x08;
pub const STMPE811_REG_INT_CTRL: c_uint = 0x09;
pub const STMPE811_REG_INT_EN: c_uint = 0x0A;
pub const STMPE811_REG_INT_STA: c_uint = 0x0B;
pub const STMPE811_REG_GPIO_INT_EN: c_uint = 0x0C;
pub const STMPE811_REG_GPIO_INT_STA: c_uint = 0x0D;
pub const STMPE811_REG_GPIO_SET_PIN: c_uint = 0x10;
pub const STMPE811_REG_GPIO_CLR_PIN: c_uint = 0x11;
pub const STMPE811_REG_GPIO_MP_STA: c_uint = 0x12;
pub const STMPE811_REG_GPIO_DIR: c_uint = 0x13;
pub const STMPE811_REG_GPIO_ED: c_uint = 0x14;
pub const STMPE811_REG_GPIO_RE: c_uint = 0x15;
pub const STMPE811_REG_GPIO_FE: c_uint = 0x16;
pub const STMPE811_REG_GPIO_AF: c_uint = 0x17;

//
// STMPE1600
//
pub const STMPE1600_ID: c_uint = 0x0016;
pub const STMPE1600_NR_INTERNAL_IRQS: c_int = 16;
pub const STMPE1600_REG_CHIP_ID: c_uint = 0x00;
pub const STMPE1600_REG_SYS_CTRL: c_uint = 0x03;
pub const STMPE1600_REG_IEGPIOR_LSB: c_uint = 0x08;
pub const STMPE1600_REG_IEGPIOR_MSB: c_uint = 0x09;
pub const STMPE1600_REG_ISGPIOR_LSB: c_uint = 0x0A;
pub const STMPE1600_REG_ISGPIOR_MSB: c_uint = 0x0B;
pub const STMPE1600_REG_GPMR_LSB: c_uint = 0x10;
pub const STMPE1600_REG_GPMR_MSB: c_uint = 0x11;
pub const STMPE1600_REG_GPSR_LSB: c_uint = 0x12;
pub const STMPE1600_REG_GPSR_MSB: c_uint = 0x13;
pub const STMPE1600_REG_GPDR_LSB: c_uint = 0x14;
pub const STMPE1600_REG_GPDR_MSB: c_uint = 0x15;
pub const STMPE1600_REG_GPPIR_LSB: c_uint = 0x16;
pub const STMPE1600_REG_GPPIR_MSB: c_uint = 0x17;
//
// STMPE1601
//
pub const STMPE1601_IRQ_GPIOC: c_int = 8;
pub const STMPE1601_IRQ_PWM3: c_int = 7;
pub const STMPE1601_IRQ_PWM2: c_int = 6;
pub const STMPE1601_IRQ_PWM1: c_int = 5;
pub const STMPE1601_IRQ_PWM0: c_int = 4;
pub const STMPE1601_IRQ_KEYPAD_OVER: c_int = 2;
pub const STMPE1601_IRQ_KEYPAD: c_int = 1;
pub const STMPE1601_IRQ_WAKEUP: c_int = 0;
pub const STMPE1601_NR_INTERNAL_IRQS: c_int = 9;
pub const STMPE1601_REG_SYS_CTRL: c_uint = 0x02;
pub const STMPE1601_REG_SYS_CTRL2: c_uint = 0x03;
pub const STMPE1601_REG_ICR_MSB: c_uint = 0x10;
pub const STMPE1601_REG_ICR_LSB: c_uint = 0x11;
pub const STMPE1601_REG_IER_MSB: c_uint = 0x12;
pub const STMPE1601_REG_IER_LSB: c_uint = 0x13;
pub const STMPE1601_REG_ISR_MSB: c_uint = 0x14;
pub const STMPE1601_REG_ISR_LSB: c_uint = 0x15;
pub const STMPE1601_REG_INT_EN_GPIO_MASK_MSB: c_uint = 0x16;
pub const STMPE1601_REG_INT_EN_GPIO_MASK_LSB: c_uint = 0x17;
pub const STMPE1601_REG_INT_STA_GPIO_MSB: c_uint = 0x18;
pub const STMPE1601_REG_INT_STA_GPIO_LSB: c_uint = 0x19;
pub const STMPE1601_REG_CHIP_ID: c_uint = 0x80;
pub const STMPE1601_REG_GPIO_SET_MSB: c_uint = 0x82;
pub const STMPE1601_REG_GPIO_SET_LSB: c_uint = 0x83;
pub const STMPE1601_REG_GPIO_CLR_MSB: c_uint = 0x84;
pub const STMPE1601_REG_GPIO_CLR_LSB: c_uint = 0x85;
pub const STMPE1601_REG_GPIO_MP_MSB: c_uint = 0x86;
pub const STMPE1601_REG_GPIO_MP_LSB: c_uint = 0x87;
pub const STMPE1601_REG_GPIO_SET_DIR_MSB: c_uint = 0x88;
pub const STMPE1601_REG_GPIO_SET_DIR_LSB: c_uint = 0x89;
pub const STMPE1601_REG_GPIO_ED_MSB: c_uint = 0x8A;
pub const STMPE1601_REG_GPIO_ED_LSB: c_uint = 0x8B;
pub const STMPE1601_REG_GPIO_RE_MSB: c_uint = 0x8C;
pub const STMPE1601_REG_GPIO_RE_LSB: c_uint = 0x8D;
pub const STMPE1601_REG_GPIO_FE_MSB: c_uint = 0x8E;
pub const STMPE1601_REG_GPIO_FE_LSB: c_uint = 0x8F;
pub const STMPE1601_REG_GPIO_PU_MSB: c_uint = 0x90;
pub const STMPE1601_REG_GPIO_PU_LSB: c_uint = 0x91;
pub const STMPE1601_REG_GPIO_AF_U_MSB: c_uint = 0x92;

// The 1601/2403 share the same masks

//
// STMPE1801
//
pub const STMPE1801_ID: c_uint = 0xc110;
pub const STMPE1801_NR_INTERNAL_IRQS: c_int = 5;
pub const STMPE1801_IRQ_KEYPAD_COMBI: c_int = 4;
pub const STMPE1801_IRQ_GPIOC: c_int = 3;
pub const STMPE1801_IRQ_KEYPAD_OVER: c_int = 2;
pub const STMPE1801_IRQ_KEYPAD: c_int = 1;
pub const STMPE1801_IRQ_WAKEUP: c_int = 0;
pub const STMPE1801_REG_CHIP_ID: c_uint = 0x00;
pub const STMPE1801_REG_SYS_CTRL: c_uint = 0x02;
pub const STMPE1801_REG_INT_CTRL_LOW: c_uint = 0x04;
pub const STMPE1801_REG_INT_EN_MASK_LOW: c_uint = 0x06;
pub const STMPE1801_REG_INT_STA_LOW: c_uint = 0x08;
pub const STMPE1801_REG_INT_EN_GPIO_MASK_LOW: c_uint = 0x0A;
pub const STMPE1801_REG_INT_EN_GPIO_MASK_MID: c_uint = 0x0B;
pub const STMPE1801_REG_INT_EN_GPIO_MASK_HIGH: c_uint = 0x0C;
pub const STMPE1801_REG_INT_STA_GPIO_LOW: c_uint = 0x0D;
pub const STMPE1801_REG_INT_STA_GPIO_MID: c_uint = 0x0E;
pub const STMPE1801_REG_INT_STA_GPIO_HIGH: c_uint = 0x0F;
pub const STMPE1801_REG_GPIO_SET_LOW: c_uint = 0x10;
pub const STMPE1801_REG_GPIO_SET_MID: c_uint = 0x11;
pub const STMPE1801_REG_GPIO_SET_HIGH: c_uint = 0x12;
pub const STMPE1801_REG_GPIO_CLR_LOW: c_uint = 0x13;
pub const STMPE1801_REG_GPIO_CLR_MID: c_uint = 0x14;
pub const STMPE1801_REG_GPIO_CLR_HIGH: c_uint = 0x15;
pub const STMPE1801_REG_GPIO_MP_LOW: c_uint = 0x16;
pub const STMPE1801_REG_GPIO_MP_MID: c_uint = 0x17;
pub const STMPE1801_REG_GPIO_MP_HIGH: c_uint = 0x18;
pub const STMPE1801_REG_GPIO_SET_DIR_LOW: c_uint = 0x19;
pub const STMPE1801_REG_GPIO_SET_DIR_MID: c_uint = 0x1A;
pub const STMPE1801_REG_GPIO_SET_DIR_HIGH: c_uint = 0x1B;
pub const STMPE1801_REG_GPIO_RE_LOW: c_uint = 0x1C;
pub const STMPE1801_REG_GPIO_RE_MID: c_uint = 0x1D;
pub const STMPE1801_REG_GPIO_RE_HIGH: c_uint = 0x1E;
pub const STMPE1801_REG_GPIO_FE_LOW: c_uint = 0x1F;
pub const STMPE1801_REG_GPIO_FE_MID: c_uint = 0x20;
pub const STMPE1801_REG_GPIO_FE_HIGH: c_uint = 0x21;
pub const STMPE1801_REG_GPIO_PULL_UP_LOW: c_uint = 0x22;
pub const STMPE1801_REG_GPIO_PULL_UP_MID: c_uint = 0x23;
pub const STMPE1801_REG_GPIO_PULL_UP_HIGH: c_uint = 0x24;

//
// STMPE24xx
//
pub const STMPE24XX_IRQ_GPIOC: c_int = 8;
pub const STMPE24XX_IRQ_PWM2: c_int = 7;
pub const STMPE24XX_IRQ_PWM1: c_int = 6;
pub const STMPE24XX_IRQ_PWM0: c_int = 5;
pub const STMPE24XX_IRQ_ROT_OVER: c_int = 4;
pub const STMPE24XX_IRQ_ROT: c_int = 3;
pub const STMPE24XX_IRQ_KEYPAD_OVER: c_int = 2;
pub const STMPE24XX_IRQ_KEYPAD: c_int = 1;
pub const STMPE24XX_IRQ_WAKEUP: c_int = 0;
pub const STMPE24XX_NR_INTERNAL_IRQS: c_int = 9;
pub const STMPE24XX_REG_SYS_CTRL: c_uint = 0x02;
pub const STMPE24XX_REG_SYS_CTRL2: c_uint = 0x03;
pub const STMPE24XX_REG_ICR_MSB: c_uint = 0x10;
pub const STMPE24XX_REG_ICR_LSB: c_uint = 0x11;
pub const STMPE24XX_REG_IER_MSB: c_uint = 0x12;
pub const STMPE24XX_REG_IER_LSB: c_uint = 0x13;
pub const STMPE24XX_REG_ISR_MSB: c_uint = 0x14;
pub const STMPE24XX_REG_ISR_LSB: c_uint = 0x15;
pub const STMPE24XX_REG_IEGPIOR_MSB: c_uint = 0x16;
pub const STMPE24XX_REG_IEGPIOR_CSB: c_uint = 0x17;
pub const STMPE24XX_REG_IEGPIOR_LSB: c_uint = 0x18;
pub const STMPE24XX_REG_ISGPIOR_MSB: c_uint = 0x19;
pub const STMPE24XX_REG_ISGPIOR_CSB: c_uint = 0x1A;
pub const STMPE24XX_REG_ISGPIOR_LSB: c_uint = 0x1B;
pub const STMPE24XX_REG_CHIP_ID: c_uint = 0x80;
pub const STMPE24XX_REG_GPSR_MSB: c_uint = 0x83;
pub const STMPE24XX_REG_GPSR_CSB: c_uint = 0x84;
pub const STMPE24XX_REG_GPSR_LSB: c_uint = 0x85;
pub const STMPE24XX_REG_GPCR_MSB: c_uint = 0x86;
pub const STMPE24XX_REG_GPCR_CSB: c_uint = 0x87;
pub const STMPE24XX_REG_GPCR_LSB: c_uint = 0x88;
pub const STMPE24XX_REG_GPDR_MSB: c_uint = 0x89;
pub const STMPE24XX_REG_GPDR_CSB: c_uint = 0x8A;
pub const STMPE24XX_REG_GPDR_LSB: c_uint = 0x8B;
pub const STMPE24XX_REG_GPEDR_MSB: c_uint = 0x8C;
pub const STMPE24XX_REG_GPEDR_CSB: c_uint = 0x8D;
pub const STMPE24XX_REG_GPEDR_LSB: c_uint = 0x8E;
pub const STMPE24XX_REG_GPRER_MSB: c_uint = 0x8F;
pub const STMPE24XX_REG_GPRER_CSB: c_uint = 0x90;
pub const STMPE24XX_REG_GPRER_LSB: c_uint = 0x91;
pub const STMPE24XX_REG_GPFER_MSB: c_uint = 0x92;
pub const STMPE24XX_REG_GPFER_CSB: c_uint = 0x93;
pub const STMPE24XX_REG_GPFER_LSB: c_uint = 0x94;
pub const STMPE24XX_REG_GPPUR_MSB: c_uint = 0x95;
pub const STMPE24XX_REG_GPPUR_CSB: c_uint = 0x96;
pub const STMPE24XX_REG_GPPUR_LSB: c_uint = 0x97;
pub const STMPE24XX_REG_GPPDR_MSB: c_uint = 0x98;
pub const STMPE24XX_REG_GPPDR_CSB: c_uint = 0x99;
pub const STMPE24XX_REG_GPPDR_LSB: c_uint = 0x9A;
pub const STMPE24XX_REG_GPAFR_U_MSB: c_uint = 0x9B;
pub const STMPE24XX_REG_GPMR_MSB: c_uint = 0xA2;
pub const STMPE24XX_REG_GPMR_CSB: c_uint = 0xA3;
pub const STMPE24XX_REG_GPMR_LSB: c_uint = 0xA4;

