//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/raw3270.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Local Channel Commands
pub const TC_WRITE: c_uint = 0x01		/* Write */;
pub const TC_RDBUF: c_uint = 0x02		/* Read Buffer */;
pub const TC_EWRITE: c_uint = 0x05		/* Erase write */;
pub const TC_READMOD: c_uint = 0x06		/* Read modified */;
pub const TC_EWRITEA: c_uint = 0x0d		/* Erase write alternate */;
pub const TC_WRITESF: c_uint = 0x11		/* Write structured field */;
// Buffer Control Orders
pub const TO_GE: c_uint = 0x08		/* Graphics Escape */;
pub const TO_SF: c_uint = 0x1d		/* Start field */;
pub const TO_SBA: c_uint = 0x11		/* Set buffer address */;
pub const TO_IC: c_uint = 0x13		/* Insert cursor */;
pub const TO_PT: c_uint = 0x05		/* Program tab */;
pub const TO_RA: c_uint = 0x3c		/* Repeat to address */;
pub const TO_SFE: c_uint = 0x29		/* Start field extended */;
pub const TO_EUA: c_uint = 0x12		/* Erase unprotected to address */;
pub const TO_MF: c_uint = 0x2c		/* Modify field */;
pub const TO_SA: c_uint = 0x28		/* Set attribute */;
// Field Attribute Bytes
pub const TF_INPUT: c_uint = 0x40		/* Visible input */;
pub const TF_INPUTN: c_uint = 0x4c		/* Invisible input */;
pub const TF_INMDT: c_uint = 0xc1		/* Visible, Set-MDT */;
pub const TF_LOG: c_uint = 0x60;
// Character Attribute Bytes
pub const TAT_RESET: c_uint = 0x00;
pub const TAT_FIELD: c_uint = 0xc0;
pub const TAT_EXTHI: c_uint = 0x41;
pub const TAT_FGCOLOR: c_uint = 0x42;
pub const TAT_CHARS: c_uint = 0x43;
pub const TAT_BGCOLOR: c_uint = 0x45;
pub const TAT_TRANS: c_uint = 0x46;
// Extended-Highlighting Bytes
pub const TAX_RESET: c_uint = 0x00;
pub const TAX_BLINK: c_uint = 0xf1;
pub const TAX_REVER: c_uint = 0xf2;
pub const TAX_UNDER: c_uint = 0xf4;
// Reset value
pub const TAR_RESET: c_uint = 0x00;
// Color values
pub const TAC_RESET: c_uint = 0x00;
pub const TAC_BLUE: c_uint = 0xf1;
pub const TAC_RED: c_uint = 0xf2;
pub const TAC_PINK: c_uint = 0xf3;
pub const TAC_GREEN: c_uint = 0xf4;
pub const TAC_TURQ: c_uint = 0xf5;
pub const TAC_YELLOW: c_uint = 0xf6;
pub const TAC_WHITE: c_uint = 0xf7;
pub const TAC_DEFAULT: c_uint = 0x00;
// Write Control Characters
pub const TW_NONE: c_uint = 0x40		/* No particular action */;
pub const TW_KR: c_uint = 0xc2		/* Keyboard restore */;
pub const TW_PLUSALARM: c_uint = 0x04		/* Add this bit for alarm */;

pub const AID_CLEAR: c_uint = 0x6d;
pub const AID_ENTER: c_uint = 0x7d;
pub const AID_PF3: c_uint = 0xf3;
pub const AID_PF7: c_uint = 0xf7;
pub const AID_PF8: c_uint = 0xf8;
pub const AID_READ_PARTITION: c_uint = 0x88;
