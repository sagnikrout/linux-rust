//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ppc4xx/xor.h
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
// 440SPe's XOR engines support header file
//
// 2006-2009 (C) DENX Software Engineering.
//
// Author: Yuri Tikhonov <yur@emcraft.com>
//

// Number of XOR engines available on the contoller
pub const XOR_ENGINES_NUM: c_int = 1;
// Number of operands supported in the h/w
pub const XOR_MAX_OPS: c_int = 16;
//
// XOR Command Block Control Register bits
//

//
// XORCore Status Register bits
//

//
// XORCore Control Set and Reset Register bits
//

//
// XORCore Interrupt Enable Register
//

//
// XOR Accelerator engine Command Block Type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xor_cb {
//
// Basic 64-bit format XOR CB (Table 19-1, p.463, 440spe_um_1_22.pdf)
//
    pub /: *mut *mut u32 cbc; / control,
    pub /: *mut *mut u32 cbbc; / byte count,
    pub /: *mut *mut u32 cbs; / status,
    pub /: *mut *mut u8 pad0[4]; / reserved,
    pub /: *mut *mut u32 cbtah; / target address high,
    pub /: *mut *mut u32 cbtal; / target address low,
    pub /: *mut *mut u32 cblah; / link address high,
    pub /: *mut *mut u32 cblal; / link address low,
    pub h: u32,
    pub l: u32,
    pub ops: [} __attribute__ ((packed)); 16],
// C attribute field omitted
//
// XOR hardware registers Table 19-3, UM 1.22
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xor_regs {
    pub /: *mut *mut u32 op_ar[16][2]; / operand address[0]-high,[1]-low registers,
    pub /: *mut *mut u8 pad0[352]; / reserved,
    pub /: *mut *mut u32 cbcr; / CB control register,
    pub /: *mut *mut u32 cbbcr; / CB byte count register,
    pub /: *mut *mut u32 cbsr; / CB status register,
    pub /: *mut *mut u8 pad1[4]; / reserved,
    pub /: *mut *mut u32 cbtahr; / operand target address high register,
    pub /: *mut *mut u32 cbtalr; / operand target address low register,
    pub /: *mut *mut u32 cblahr; / CB link address high register,
    pub /: *mut *mut u32 cblalr; / CB link address low register,
    pub /: *mut *mut u32 crsr; / control set register,
    pub /: *mut *mut u32 crrr; / control reset register,
    pub /: *mut *mut u32 ccbahr; / current CB address high register,
    pub /: *mut *mut u32 ccbalr; / current CB address low register,
    pub /: *mut *mut u32 plbr; / PLB configuration register,
    pub /: *mut *mut u32 ier; / interrupt enable register,
    pub /: *mut *mut u32 pecr; / parity error count register,
    pub /: *mut *mut u32 sr; / status register,
    pub /: *mut *mut u32 revidr; / revision ID register,
}
