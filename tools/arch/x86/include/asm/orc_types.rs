//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/asm/orc_types.h
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
// Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
//

//
// The ORC_REG_* registers are base registers which are used to find other
// registers on the stack.
//
// ORC_REG_PREV_SP, also known as DWARF Call Frame Address (CFA), is the
// address of the previous frame: the caller's SP before it called the current
// function.
//
// ORC_REG_UNDEFINED means the corresponding register's value didn't change in
// the current frame.
//
// The most commonly used base registers are SP and BP -- which the previous SP
// is usually based on -- and PREV_SP and UNDEFINED -- which the previous BP is
// usually based on.
//
// The rest of the base registers are needed for special cases like entry code
// and GCC realigned stacks.
//
pub const ORC_REG_UNDEFINED: c_int = 0;
pub const ORC_REG_AX: c_int = 1;
pub const ORC_REG_DX: c_int = 2;
pub const ORC_REG_SP: c_int = 3;
pub const ORC_REG_BP: c_int = 4;
pub const ORC_REG_DI: c_int = 5;
pub const ORC_REG_R10: c_int = 6;
pub const ORC_REG_R13: c_int = 7;
pub const ORC_REG_PREV_SP: c_int = 8;
pub const ORC_REG_SP_INDIRECT: c_int = 9;
pub const ORC_REG_BP_INDIRECT: c_int = 10;
pub const ORC_REG_MAX: c_int = 15;
pub const ORC_TYPE_UNDEFINED: c_int = 0;
pub const ORC_TYPE_END_OF_STACK: c_int = 1;
pub const ORC_TYPE_CALL: c_int = 2;
pub const ORC_TYPE_REGS: c_int = 3;
pub const ORC_TYPE_REGS_PARTIAL: c_int = 4;

//
// This struct is more or less a vastly simplified version of the DWARF Call
// Frame Information standard.  It contains only the necessary parts of DWARF
// CFI, simplified for ease of access by the in-kernel unwinder.  It tells the
// unwinder how to find the previous SP and BP (and sometimes entry regs) on
// the stack for a given code address.  Each instance of the struct corresponds
// to one or more code locations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_entry {
    pub sp_offset: i16,
    pub bp_offset: i16,

    pub sp_reg:4: unsigned,
    pub bp_reg:4: unsigned,
    pub type:3: unsigned,
    pub signal:1: unsigned,

    pub bp_reg:4: unsigned,
    pub sp_reg:4: unsigned,
    pub unused:4: unsigned,
    pub signal:1: unsigned,
    pub type:3: unsigned,

    pub __packed: },

