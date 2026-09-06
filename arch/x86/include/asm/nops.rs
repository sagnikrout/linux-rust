//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/nops.h
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
// Define nops for use with alternative() and for tracing.
//

//
// Generic 32bit nops from GAS:
//
// 1: nop
// 2: movl %esi,%esi
// 3: leal 0x0(%esi),%esi
// 4: leal 0x0(%esi,%eiz,1),%esi
// 5: leal %ds:0x0(%esi,%eiz,1),%esi
// 6: leal 0x0(%esi),%esi
// 7: leal 0x0(%esi,%eiz,1),%esi
// 8: leal %ds:0x0(%esi,%eiz,1),%esi
//
// Except 5 and 8, which are DS prefixed 4 and 7 resp, where GAS would emit 2
// nop instructions.
//
pub const BYTES_NOP1: c_uint = 0x90;
pub const BYTES_NOP2: c_uint = 0x89,0xf6;
pub const BYTES_NOP3: c_uint = 0x8d,0x76,0x00;
pub const BYTES_NOP4: c_uint = 0x8d,0x74,0x26,0x00;
pub const BYTES_NOP5: c_uint = 0x3e,BYTES_NOP4;
pub const BYTES_NOP6: c_uint = 0x8d,0xb6,0x00,0x00,0x00,0x00;
pub const BYTES_NOP7: c_uint = 0x8d,0xb4,0x26,0x00,0x00,0x00,0x00;
pub const BYTES_NOP8: c_uint = 0x3e,BYTES_NOP7;
pub const ASM_NOP_MAX: c_int = 8;

//
// Generic 64bit nops from GAS:
//
// 1: nop
// 2: osp nop
// 3: nopl (%eax)
// 4: nopl 0x00(%eax)
// 5: nopl 0x00(%eax,%eax,1)
// 6: osp nopl 0x00(%eax,%eax,1)
// 7: nopl 0x00000000(%eax)
// 8: nopl 0x00000000(%eax,%eax,1)
// 9: cs nopl 0x00000000(%eax,%eax,1)
// 10: osp cs nopl 0x00000000(%eax,%eax,1)
// 11: osp osp cs nopl 0x00000000(%eax,%eax,1)
//
pub const BYTES_NOP1: c_uint = 0x90;
pub const BYTES_NOP2: c_uint = 0x66,BYTES_NOP1;
pub const BYTES_NOP3: c_uint = 0x0f,0x1f,0x00;
pub const BYTES_NOP4: c_uint = 0x0f,0x1f,0x40,0x00;
pub const BYTES_NOP5: c_uint = 0x0f,0x1f,0x44,0x00,0x00;
pub const BYTES_NOP6: c_uint = 0x66,BYTES_NOP5;
pub const BYTES_NOP7: c_uint = 0x0f,0x1f,0x80,0x00,0x00,0x00,0x00;
pub const BYTES_NOP8: c_uint = 0x0f,0x1f,0x84,0x00,0x00,0x00,0x00,0x00;
pub const BYTES_NOP9: c_uint = 0x2e,BYTES_NOP8;
pub const BYTES_NOP10: c_uint = 0x66,BYTES_NOP9;
pub const BYTES_NOP11: c_uint = 0x66,BYTES_NOP10;

pub const ASM_NOP_MAX: c_int = 11;

