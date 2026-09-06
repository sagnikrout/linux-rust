//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mc6821.h
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
// This file describes the memery mapping of the MC6821 PIA.
// The unions describe overlayed registers. Which of them is used is
// determined by bit 2 of the corresponding control register.
// this files expects the PIA_REG_PADWIDTH to be defined the numeric
// value of the register spacing.
//
// Data came from MFC-31-Developer Kit (from Ralph Seidel,
// zodiac@darkness.gun.de) and Motorola Data Sheet (from
// Richard Hirst, srh@gpt.co.uk)
//
// 6.11.95 copyright Joerg Dorchain (dorchain@mpi-sb.mpg.de)
//

pub const PIA_REG_PADWIDTH: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pia {
    pub pra: volatile u_char,
    pub ddra: volatile u_char,
    pub ua: },
    pub pad1: [u_char; PIA_REG_PADWIDTH],
    pub cra: volatile u_char,
    pub pad2: [u_char; PIA_REG_PADWIDTH],
    pub prb: volatile u_char,
    pub ddrb: volatile u_char,
    pub ub: },
    pub pad3: [u_char; PIA_REG_PADWIDTH],
    pub crb: volatile u_char,
    pub pad4: [u_char; PIA_REG_PADWIDTH],
}

