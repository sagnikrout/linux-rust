//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/asm/insn.h
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
// x86 instruction analysis
//
// Copyright (C) IBM Corporation, 2009
//

// insn_attr_t is defined in inat.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_field {
    pub value: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

// !0 if we've run insn_get_xxx() for this field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_field {
    pub value: insn_value_t,
    pub little: insn_value_t,
    pub bytes: [insn_byte_t; 4],
}

// !0 if we've run insn_get_xxx() for this field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn {
    pub /*: *mut insn_field prefixes;,
// Prefixes
// prefixes.bytes[3]: last prefix
//
    pub /: *mut *mut insn_field rex_prefix; / REX prefix,
    pub /: *mut *mut insn_field vex_prefix; / VEX prefix,
    pub /: *mut *mut insn_field xop_prefix; / XOP prefix,
}

// opcode.bytes[0]: opcode1
// opcode.bytes[1]: opcode2
// opcode.bytes[2]: opcode3
//
pub const MAX_INSN_SIZE: c_int = 15;

// VEX bit flags

// VEX bit fields

pub const X86_VEX_M_MAX: c_uint = 0x1f			/* VEX3.M Maximum value */;
// XOP bit fields

pub const X86_XOP_M_MIN: c_uint = 0x08	/* Min of XOP.M */;
pub const X86_XOP_M_MAX: c_uint = 0x1f	/* Max of XOP.M */;
extern "C" {
    pub fn insn_init(insn: *mut insn, kaddr: *const c_void, buf_len: c_int, x86_64: c_int);
}
extern "C" {
    pub fn insn_get_prefixes(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_opcode(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_modrm(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_sib(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_displacement(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_immediate(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn insn_get_length(insn: *mut insn) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum insn_mode {
    INSN_MODE_32,
    INSN_MODE_64,
// Mode is determined by the current kernel build.
    INSN_MODE_KERN,
    INSN_NUM_MODES,
}

extern "C" {
    pub fn insn_decode(insn: *mut insn, kaddr: *const c_void, buf_len: c_int, m: insn_mode) -> c_int;
}

// Attribute will be determined after getting ModRM (for opcode groups)
// Instruction uses RIP-relative addressing
extern "C" {
    pub fn insn_rip_relative(insn: *mut insn) -> c_int;
}
extern "C" {
    pub fn X86_REX2_M(_arg: insn->rex_prefix.bytes[1]) -> return;
}
// If we already know this is AVX/XOP encoded
extern "C" {
    pub fn inat_is_xop_prefix(_arg: attr) -> return;
}
extern "C" {
    pub fn avx_insn_is_xop(_arg: insn) -> return;
}
extern "C" {
    pub fn X86_VEX3_M(_arg: insn->vex_prefix.bytes[1]) -> return;
}
extern "C" {
    pub fn X86_EVEX_M(_arg: insn->vex_prefix.bytes[1]) -> return;
}
extern "C" {
    pub fn X86_VEX_P(_arg: insn->vex_prefix.bytes[1]) -> return;
}
extern "C" {
    pub fn X86_VEX_P(_arg: insn->vex_prefix.bytes[2]) -> return;
}
extern "C" {
    pub fn X86_VEX_W(_arg: insn->vex_prefix.bytes[2]) -> return;
}
extern "C" {
    pub fn X86_XOP_M(_arg: insn->xop_prefix.bytes[1]) -> return;
}
extern "C" {
    pub fn X86_XOP_P(_arg: insn->vex_prefix.bytes[2]) -> return;
}
// Get the last prefix id from last prefix or VEX prefix
extern "C" {
    pub fn insn_xop_p_bits(_arg: insn) -> return;
}
extern "C" {
    pub fn inat_get_last_prefix_id(_arg: insn->prefixes.bytes[3]) -> return;
}
// Offset of each field from kaddr
//
// for_each_insn_prefix() -- Iterate prefixes in the instruction
// @insn: Pointer to struct insn.
// @prefix: Prefix byte.
//
// Iterate prefix bytes of given @insn. Each prefix byte is stored in @prefix
// and the index is stored in @idx (note that this @idx is just for a cursor,
// do not change it.)
// Since prefixes.nbytes can be bigger than 4 if some prefixes
// are repeated, it cannot be used for looping over the prefixes.
//

pub const POP_SS_OPCODE: c_uint = 0x1f;
pub const MOV_SREG_OPCODE: c_uint = 0x8e;
//
// Intel SDM Vol.3A 6.8.3 states;
// "Any single-step trap that would be delivered following the MOV to SS
// instruction or POP to SS instruction (because EFLAGS.TF is 1) is
// suppressed."
// This function returns true if @insn is MOV SS or POP SS. On these
// instructions, single stepping is suppressed.
//
