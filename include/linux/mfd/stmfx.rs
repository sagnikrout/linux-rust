//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stmfx.h
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
// Copyright (C) 2019 STMicroelectronics
// Author(s): Amelie Delaunay <amelie.delaunay@st.com>.
//

// General
pub const STMFX_REG_CHIP_ID: c_uint = 0x00 /* R */;
pub const STMFX_REG_FW_VERSION_MSB: c_uint = 0x01 /* R */;
pub const STMFX_REG_FW_VERSION_LSB: c_uint = 0x02 /* R */;
pub const STMFX_REG_SYS_CTRL: c_uint = 0x40 /* RW */;
// IRQ output management
pub const STMFX_REG_IRQ_OUT_PIN: c_uint = 0x41 /* RW */;
pub const STMFX_REG_IRQ_SRC_EN: c_uint = 0x42 /* RW */;
pub const STMFX_REG_IRQ_PENDING: c_uint = 0x08 /* R */;
pub const STMFX_REG_IRQ_ACK: c_uint = 0x44 /* RW */;
// GPIO management
pub const STMFX_REG_IRQ_GPI_PENDING1: c_uint = 0x0C /* R */;
pub const STMFX_REG_IRQ_GPI_PENDING2: c_uint = 0x0D /* R */;
pub const STMFX_REG_IRQ_GPI_PENDING3: c_uint = 0x0E /* R */;
pub const STMFX_REG_GPIO_STATE1: c_uint = 0x10 /* R */;
pub const STMFX_REG_GPIO_STATE2: c_uint = 0x11 /* R */;
pub const STMFX_REG_GPIO_STATE3: c_uint = 0x12 /* R */;
pub const STMFX_REG_IRQ_GPI_SRC1: c_uint = 0x48 /* RW */;
pub const STMFX_REG_IRQ_GPI_SRC2: c_uint = 0x49 /* RW */;
pub const STMFX_REG_IRQ_GPI_SRC3: c_uint = 0x4A /* RW */;
pub const STMFX_REG_IRQ_GPI_EVT1: c_uint = 0x4C /* RW */;
pub const STMFX_REG_IRQ_GPI_EVT2: c_uint = 0x4D /* RW */;
pub const STMFX_REG_IRQ_GPI_EVT3: c_uint = 0x4E /* RW */;
pub const STMFX_REG_IRQ_GPI_TYPE1: c_uint = 0x50 /* RW */;
pub const STMFX_REG_IRQ_GPI_TYPE2: c_uint = 0x51 /* RW */;
pub const STMFX_REG_IRQ_GPI_TYPE3: c_uint = 0x52 /* RW */;
pub const STMFX_REG_IRQ_GPI_ACK1: c_uint = 0x54 /* RW */;
pub const STMFX_REG_IRQ_GPI_ACK2: c_uint = 0x55 /* RW */;
pub const STMFX_REG_IRQ_GPI_ACK3: c_uint = 0x56 /* RW */;
pub const STMFX_REG_GPIO_DIR1: c_uint = 0x60 /* RW */;
pub const STMFX_REG_GPIO_DIR2: c_uint = 0x61 /* RW */;
pub const STMFX_REG_GPIO_DIR3: c_uint = 0x62 /* RW */;
pub const STMFX_REG_GPIO_TYPE1: c_uint = 0x64 /* RW */;
pub const STMFX_REG_GPIO_TYPE2: c_uint = 0x65 /* RW */;
pub const STMFX_REG_GPIO_TYPE3: c_uint = 0x66 /* RW */;
pub const STMFX_REG_GPIO_PUPD1: c_uint = 0x68 /* RW */;
pub const STMFX_REG_GPIO_PUPD2: c_uint = 0x69 /* RW */;
pub const STMFX_REG_GPIO_PUPD3: c_uint = 0x6A /* RW */;
pub const STMFX_REG_GPO_SET1: c_uint = 0x6C /* RW */;
pub const STMFX_REG_GPO_SET2: c_uint = 0x6D /* RW */;
pub const STMFX_REG_GPO_SET3: c_uint = 0x6E /* RW */;
pub const STMFX_REG_GPO_CLR1: c_uint = 0x70 /* RW */;
pub const STMFX_REG_GPO_CLR2: c_uint = 0x71 /* RW */;
pub const STMFX_REG_GPO_CLR3: c_uint = 0x72 /* RW */;
pub const STMFX_REG_MAX: c_uint = 0xB0;
// MFX boot time is around 10ms, so after reset, we have to wait this delay
pub const STMFX_BOOT_TIME_MS: c_int = 10;
// STMFX_REG_CHIP_ID bitfields

// STMFX_REG_SYS_CTRL bitfields

// STMFX_REG_IRQ_OUT_PIN bitfields

// STMFX_REG_IRQ_(SRC_EN/PENDING/ACK) bit shift
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmfx_irqs {
    STMFX_REG_IRQ_SRC_EN_GPIO = 0,
    STMFX_REG_IRQ_SRC_EN_IDD,
    STMFX_REG_IRQ_SRC_EN_ERROR,
    STMFX_REG_IRQ_SRC_EN_TS_DET,
    STMFX_REG_IRQ_SRC_EN_TS_NE,
    STMFX_REG_IRQ_SRC_EN_TS_TH,
    STMFX_REG_IRQ_SRC_EN_TS_FULL,
    STMFX_REG_IRQ_SRC_EN_TS_OVF,
    STMFX_REG_IRQ_SRC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmfx_functions {
    STMFX_FUNC_GPIO		= BIT(0), /* GPIO[15:0] */
    STMFX_FUNC_ALTGPIO_LOW	= BIT(1), /* aGPIO[3:0] */
    STMFX_FUNC_ALTGPIO_HIGH = BIT(2), /* aGPIO[7:4] */
    STMFX_FUNC_TS		= BIT(3),
    STMFX_FUNC_IDD		= BIT(4),
}

//
// struct stmfx_ddata - STMFX MFD structure
// @device:		device reference used for logs
// @map:		register map
// @vdd:		STMFX power supply
// @irq_domain:		IRQ domain
// @lock:		IRQ bus lock
// @irq_src:		cache of IRQ_SRC_EN register for bus_lock
// @bkp_sysctrl:	backup of SYS_CTRL register for suspend/resume
// @bkp_irqoutpin:	backup of IRQ_OUT_PIN register for suspend/resume
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmfx {
    pub dev: *mut device,
    pub map: *mut regmap,
    pub vdd: *mut regulator,
    pub irq: c_int,
    pub irq_domain: *mut irq_domain,
    pub /: *mut *mut mutex lock; / IRQ bus lock,
    pub irq_src: u8,
    pub bkp_sysctrl: u8,
    pub bkp_irqoutpin: u8,
}

extern "C" {
    pub fn stmfx_function_enable(stmfx: *mut stmfx, func: u32) -> c_int;
}
extern "C" {
    pub fn stmfx_function_disable(stmfx: *mut stmfx, func: u32) -> c_int;
}
