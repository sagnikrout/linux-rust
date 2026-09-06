//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/check.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_state {
    pub cfi: cfi_state,
    pub uaccess_stack: c_uint,
    pub uaccess: bool,
    pub df: bool,
    pub noinstr: bool,
    pub instr: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_group {
//
// Pointer from a replacement group to the original group.  NULL if it
// *is* the original group.
//
    pub orig_group: *mut alt_group,
// First and last instructions in the group
    pub nop: *mut *mut *mut instruction first_insn, last_insn,,
//
// Byte-offset-addressed len-sized array of pointers to CFI structs.
// This is shared with the other alt_groups in the same alternative.
//
    pub cfi: *mut cfi_state,
    pub ignore: bool,
    pub feature: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alternative_type {
    ALT_TYPE_INSTRUCTIONS,
    ALT_TYPE_JUMP_TABLE,
    ALT_TYPE_EX_TABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alternative {
    pub next: *mut alternative,
    pub insn: *mut instruction,
    pub type: alternative_type,
}

pub const INSN_CHUNK_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct instruction {
    pub hash: hlist_node,
    pub call_node: list_head,
    pub sec: *mut section,
    pub offset: c_ulong,
    pub immediate: c_ulong,
    pub len: u8,
    pub prev_len: u8,
    pub type: u8,
    pub instr: i8,
    pub 1: trace :,
// 4 bit hole
    pub alt_group: *mut alt_group,
    pub jump_dest: *mut instruction,
    pub first_jump_src: *mut instruction,
    pub _call_dest: *mut symbol,
    pub _jump_table: *mut reloc,
    pub _jump_table_size: c_ulong,
}

//
// Return the symbol associated with an instruction.  For alternative
// replacements, return the symbol of the original code being replaced rather
// than NULL.  insn->_sym reflects the actual location in the ELF file.
//
pub const VISITED_BRANCH: c_uint = 0x01;
pub const VISITED_BRANCH_UACCESS: c_uint = 0x02;
pub const VISITED_BRANCH_MASK: c_uint = 0x03;
pub const VISITED_UNRET: c_uint = 0x04;
extern "C" {
    pub fn is_static_jump(is_dynamic_jump(insn: insn) ||) -> return;
}

extern "C" {
    pub fn decode_file(file: *mut objtool_file) -> c_int;
}
extern "C" {
    pub fn free_insns(file: *mut objtool_file);
}
extern "C" {
    pub fn pv_ops_idx_off(symname: *const c_char) -> c_int;
}
