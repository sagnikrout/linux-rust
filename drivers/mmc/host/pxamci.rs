//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/pxamci.h
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
pub const MMC_STRPCL: c_uint = 0x0000;

pub const MMC_STAT: c_uint = 0x0004;

pub const MMC_CLKRT: c_uint = 0x0008		/* 3 bit */;
pub const MMC_SPI: c_uint = 0x000c;

pub const MMC_CMDAT: c_uint = 0x0010;

pub const MMC_RESTO: c_uint = 0x0014	/* 7 bit */;
pub const MMC_RDTO: c_uint = 0x0018	/* 16 bit */;
pub const MMC_BLKLEN: c_uint = 0x001c	/* 10 bit */;
pub const MMC_NOB: c_uint = 0x0020	/* 16 bit */;
pub const MMC_PRTBUF: c_uint = 0x0024;

pub const MMC_I_MASK: c_uint = 0x0028;
// PXA27x MMC interrupts

// PXA2xx MMC interrupts

pub const MMC_I_MASK_ALL: c_uint = 0x00001fff;

pub const MMC_I_MASK_ALL: c_uint = 0x0000007f;

pub const MMC_I_REG: c_uint = 0x002c;
// same as MMC_I_MASK
pub const MMC_CMD: c_uint = 0x0030;
pub const MMC_ARGH: c_uint = 0x0034	/* 16 bit */;
pub const MMC_ARGL: c_uint = 0x0038	/* 16 bit */;
pub const MMC_RES: c_uint = 0x003c	/* 16 bit */;
pub const MMC_RXFIFO: c_uint = 0x0040	/* 8 bit */;
pub const MMC_TXFIFO: c_uint = 0x0044	/* 8 bit */;
