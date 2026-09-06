//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/lowcore.h
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
// Copyright IBM Corp. 1999, 2012
// Author(s): Hartmut Penner <hp@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Denis Joseph Barrow,
//

pub const LC_ORDER: c_int = 1;
pub const LC_PAGES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgm_tdb {
    pub data: [u64; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowcore {
    pub /: *mut *mut __u8 pad_0x0000[0x0014-0x0000]; / 0x0000,
    pub /: *mut *mut __u32 ipl_parmblock_ptr; / 0x0014,
    pub /: *mut *mut __u8 pad_0x0018[0x0080-0x0018]; / 0x0018,
    pub /: *mut *mut __u32 ext_params; / 0x0080,
    pub /: *mut *mut __u16 ext_cpu_addr; / 0x0084,
    pub /: *mut *mut __u16 ext_int_code; / 0x0086,
}

// Save areas.
// Return psws.
// CPU accounting and timing values.
// Current process.
// Interrupt, DAT-off and restartstack.
// Restart function and parameter.
// Address space pointer.
//
// The lpp and current_pid fields form a
// 64-bit value that is set as program
// parameter with the LPP instruction.
//
// SMP info area
//
// 0xe00 contains the address of the IPL Parameter Information
// block. Dump tools need IPIB for IPL after dump.
// Note: do not change the position of any fields in 0x0e00-0x0f00
//
// Pointer to the machine check extended save area
// 64 bit extparam used for pfault/diag 250: defined by architecture
// CPU register save area: defined by architecture
// Cryptography-counter designation
// AI-extension counter designation
// Transaction abort diagnostic block
extern "C" {
    pub fn volatile("memory": "spx %0" : : "Q" (address) :) -> asm;
}

