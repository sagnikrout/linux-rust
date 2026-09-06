//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/gpib_cmd.h
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

// Command byte definitions tests and functions
// mask of bits that actually matter in a command byte
// Possible GPIB command messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_byte {
    GTL = 0x1,	/* go to local			*/
    SDC = 0x4,	/* selected device clear	*/
    PP_CONFIG = 0x5,
    GET = 0x8,	/* group execute trigger	*/
    TCT = 0x9,	/* take control			*/
    LLO = 0x11,	/* local lockout		*/
    DCL = 0x14,	/* device clear			*/
    PPU = 0x15,	/* parallel poll unconfigure	*/
    SPE = 0x18,	/* serial poll enable		*/
    SPD = 0x19,	/* serial poll disable		*/
    CFE = 0x1f,     /* configure enable */
    LAD = 0x20,	/* value to be 'ored' in to obtain listen address */
    UNL = 0x3F,	/* unlisten			*/
    TAD = 0x40,	/* value to be 'ored' in to obtain talk address	  */
    UNT = 0x5F,	/* untalk			*/
    SAD = 0x60,	/* my secondary address (base) */
    PPE = 0x60,	/* parallel poll enable (base)	*/
    PPD = 0x70	/* parallel poll disable	*/
}

// confine address to range 0 to 30.
