//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mantis/mantis_reg.h
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
// Interrupts
pub const MANTIS_INT_STAT: c_uint = 0x00;
pub const MANTIS_INT_MASK: c_uint = 0x04;

// #define MANTIS_INT_GPIF			(0xff << 12)

// DMA
pub const MANTIS_DMA_CTL: c_uint = 0x08;

// DEBUG
pub const MANTIS_DEBUGREG: c_uint = 0x0c;

pub const MANTIS_RISC_START: c_uint = 0x10;
pub const MANTIS_RISC_PC: c_uint = 0x14;
// I2C
pub const MANTIS_I2CDATA_CTL: c_uint = 0x18;

// DATA
pub const MANTIS_CMD_DATA_R1: c_uint = 0x20;

pub const MANTIS_CMD_DATA_R2: c_uint = 0x24;

pub const MANTIS_CONTROL: c_uint = 0x28;

pub const MANTIS_GPIF_CFGSLA: c_uint = 0x84;

pub const MANTIS_GPIF_WSTOPER: c_uint = 0x90;

pub const MANTIS_GPIF_CS2RW: c_uint = 0x94;

pub const MANTIS_GPIF_IRQCFG: c_uint = 0x98;

pub const MANTIS_GPIF_STATUS: c_uint = 0x9c;

pub const MANTIS_GPIF_BRADDR: c_uint = 0xa0;

pub const MANTIS_GPIF_BRBYTES: c_uint = 0xa4;

pub const MANTIS_PCMCIA_RESET: c_uint = 0xa8;

pub const MANTIS_CARD_RESET: c_uint = 0xac;
pub const MANTIS_GPIF_ADDR: c_uint = 0xb0;

pub const MANTIS_GPIF_DOUT: c_uint = 0xb4;

pub const MANTIS_GPIF_DIN: c_uint = 0xb8;

pub const MANTIS_GPIF_SPARE: c_uint = 0xbc;

