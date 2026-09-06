//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/text-patching.h
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
// Copyright 2008, Michael Ellerman, IBM Corporation.
//

// Flags for create_branch:
// "b"   == create_branch(addr, target, 0);
// "ba"  == create_branch(addr, target, BRANCH_ABSOLUTE);
// "bl"  == create_branch(addr, target, BRANCH_SET_LINK);
// "bla" == create_branch(addr, target, BRANCH_ABSOLUTE | BRANCH_SET_LINK);
//
pub const BRANCH_SET_LINK: c_uint = 0x1;
pub const BRANCH_ABSOLUTE: c_uint = 0x2;
//
// Powerpc branch instruction is :
//
// 0         6                 30   31
// +---------+----------------+---+---+
// | opcode  |     LI         |AA |LK |
// +---------+----------------+---+---+
// Where AA = 0 and LK = 0
//
// LI is a signed 24 bits integer. The real branch offset is computed
// by: imm32 = SignExtend(LI:'0b00', 32);
//
// So the maximum forward branch should be:
// (0x007fffff << 2) = 0x01fffffc =  0x1fffffc
// The maximum backward branch should be:
// (0xff800000 << 2) = 0xfe000000 = -0x2000000
//
// instr = ppc_inst(0);
// Check we can represent the target in the instruction format
// Mask out the flags and target, so they don't step on each other.
// instr = ppc_inst(0x48000000 | (flags & 0x3) | (offset & 0x03FFFFFC));
extern "C" {
    pub fn patch_branch(addr: *mut u32, target: c_ulong, flags: c_int) -> c_int;
}
extern "C" {
    pub fn patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int;
}
extern "C" {
    pub fn raw_patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int;
}
extern "C" {
    pub fn patch_instructions(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int;
}
//
// The data patching functions patch_uint() and patch_ulong(), etc., must be
// called on aligned addresses.
//
// The instruction patching functions patch_instruction() and similar must be
// called on addresses satisfying instruction alignment requirements.
//

extern "C" {
    pub fn patch_uint(addr: *mut c_void, val: c_uint) -> c_int;
}
extern "C" {
    pub fn patch_ulong(addr: *mut c_void, val: c_ulong) -> c_int;
}

extern "C" {
    pub fn patch_instruction(_arg: addr, _arg: ppc_inst(val)) -> return;
}
extern "C" {
    pub fn patch_instruction(_arg: addr, _arg: ppc_inst(val)) -> return;
}

extern "C" {
    pub fn patch_instruction()patch_site_addr(site): *mut (u32, _arg: instr) -> return;
}
extern "C" {
    pub fn patch_branch()patch_site_addr(site): *mut (u32, _arg: target, _arg: flags) -> return;
}
extern "C" {
    pub fn patch_instruction(_arg: addr, set): *mut *mut ppc_inst((addr & ~clr) |) -> return;
}
extern "C" {
    pub fn modify_instruction()patch_site_addr(site): *mut (unsigned int, _arg: clr, _arg: set) -> return;
}
extern "C" {
    pub fn instr_is_relative_branch(instr: ppc_inst_t) -> c_int;
}
extern "C" {
    pub fn instr_is_relative_link_branch(instr: ppc_inst_t) -> c_int;
}
extern "C" {
    pub fn branch_target(instr: *const u32) -> c_ulong;
}
extern "C" {
    pub fn translate_branch(instr: *mut ppc_inst_t, dest: *const u32, src: *const u32) -> c_int;
}
extern "C" {
    pub fn is_conditional_branch(instr: ppc_inst_t) -> bool;
}
pub const OP_RT_RA_MASK: c_uint = 0xffff0000UL;

//
// A PPC64 ABIv2 function may have a local and a global entry
// point. We need to use the local entry point when patching
// functions, so identify and step over the global entry point
// sequence.
//
// The global entry point sequence is always of the form:
//
// addis r2,r12,XXXX
// addi  r2,r2,XXXX
//
// A linker optimisation may convert the addis to lis:
//
// lis   r2,XXXX
// addi  r2,r2,XXXX
//

//
// On PPC64 ABIv1 the function pointer actually points to the
// function's descriptor. The first entry in the descriptor is the
// address of the function text.
//

// PPC64 ABIv2 the global entry point is at the address

// All other cases there is no change vs ppc_function_entry()
extern "C" {
    pub fn ppc_function_entry(_arg: func) -> return;
}

//
// Wrapper around kallsyms_lookup() to return function entry address:
// - For ABIv1, we lookup the dot variant.
// - For ABIv2, we return the local entry point.
//

// check for dot variant
// Let's try the original non-dot symbol lookup

//
// Some instruction encodings commonly used in dynamic ftracing
// and function live patching.
//
// This must match the definition of STK_GOT in <asm/ppc_asm.h>

pub const R2_STACK_OFFSET: c_int = 24;

pub const R2_STACK_OFFSET: c_int = 40;

// usually preceded by a mflr r0

