//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max7360.h
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

pub const MAX7360_MAX_KEY_ROWS: c_int = 8;
pub const MAX7360_MAX_KEY_COLS: c_int = 8;

pub const MAX7360_ROW_SHIFT: c_int = 3;
pub const MAX7360_MAX_GPIO: c_int = 8;
pub const MAX7360_MAX_GPO: c_int = 6;
pub const MAX7360_PORT_PWM_COUNT: c_int = 8;

//
// MAX7360 registers
//
pub const MAX7360_REG_KEYFIFO: c_uint = 0x00;
pub const MAX7360_REG_CONFIG: c_uint = 0x01;
pub const MAX7360_REG_DEBOUNCE: c_uint = 0x02;
pub const MAX7360_REG_INTERRUPT: c_uint = 0x03;
pub const MAX7360_REG_PORTS: c_uint = 0x04;
pub const MAX7360_REG_KEYREP: c_uint = 0x05;
pub const MAX7360_REG_SLEEP: c_uint = 0x06;
//
// MAX7360 GPIO registers
//
// All these registers are reset together when writing bit 3 of
// MAX7360_REG_GPIOCFG.
//
pub const MAX7360_REG_GPIOCFG: c_uint = 0x40;
pub const MAX7360_REG_GPIOCTRL: c_uint = 0x41;
pub const MAX7360_REG_GPIODEB: c_uint = 0x42;
pub const MAX7360_REG_GPIOCURR: c_uint = 0x43;
pub const MAX7360_REG_GPIOOUTM: c_uint = 0x44;
pub const MAX7360_REG_PWMCOM: c_uint = 0x45;
pub const MAX7360_REG_RTRCFG: c_uint = 0x46;
pub const MAX7360_REG_I2C_TIMEOUT: c_uint = 0x48;
pub const MAX7360_REG_GPIOIN: c_uint = 0x49;
pub const MAX7360_REG_RTR_CNT: c_uint = 0x4A;
pub const MAX7360_REG_PWMBASE: c_uint = 0x50;
pub const MAX7360_REG_PWMCFGBASE: c_uint = 0x58;
pub const MAX7360_REG_GPIO_LAST: c_uint = 0x5F;

//
// Configuration register bits
//
pub const MAX7360_FIFO_EMPTY: c_uint = 0x3F;
pub const MAX7360_FIFO_OVERFLOW: c_uint = 0x7F;

pub const MAX7360_DEBOUNCE_MIN: c_int = 9;
pub const MAX7360_DEBOUNCE_MAX: c_int = 40;

//
// Autosleep register values
//
pub const MAX7360_AUTOSLEEP_8192MS: c_uint = 0x01;
pub const MAX7360_AUTOSLEEP_4096MS: c_uint = 0x02;
pub const MAX7360_AUTOSLEEP_2048MS: c_uint = 0x03;
pub const MAX7360_AUTOSLEEP_1024MS: c_uint = 0x04;
pub const MAX7360_AUTOSLEEP_512MS: c_uint = 0x05;
pub const MAX7360_AUTOSLEEP_256MS: c_uint = 0x06;

pub const MAX7360_ROT_DEBOUNCE_MIN: c_int = 0;
pub const MAX7360_ROT_DEBOUNCE_MAX: c_int = 15;

pub const MAX7360_INT_INTI: c_int = 0;
pub const MAX7360_INT_INTK: c_int = 1;
pub const MAX7360_INT_GPIO: c_int = 0;
pub const MAX7360_INT_KEYPAD: c_int = 1;
pub const MAX7360_INT_ROTARY: c_int = 2;
pub const MAX7360_NR_INTERNAL_IRQS: c_int = 3;
