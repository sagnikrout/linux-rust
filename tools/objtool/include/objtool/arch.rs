//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/arch.h
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
// Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum insn_type {
    INSN_JUMP_CONDITIONAL,
    INSN_JUMP_UNCONDITIONAL,
    INSN_JUMP_DYNAMIC,
    INSN_JUMP_DYNAMIC_CONDITIONAL,
    INSN_CALL,
    INSN_CALL_DYNAMIC,
    INSN_RETURN,
    INSN_SYSCALL,
    INSN_SYSRET,
    INSN_BUG,
    INSN_NOP,
    INSN_STAC,
    INSN_CLAC,
    INSN_STD,
    INSN_CLD,
    INSN_TRAP,
    INSN_ENDBR,
    INSN_LEA_RIP,
    INSN_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum op_dest_type {
    OP_DEST_REG,
    OP_DEST_REG_INDIRECT,
    OP_DEST_MEM,
    OP_DEST_PUSH,
    OP_DEST_PUSHF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_dest {
    pub type: op_dest_type,
    pub reg: c_uchar,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum op_src_type {
    OP_SRC_REG,
    OP_SRC_REG_INDIRECT,
    OP_SRC_CONST,
    OP_SRC_POP,
    OP_SRC_POPF,
    OP_SRC_ADD,
    OP_SRC_AND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_src {
    pub type: op_src_type,
    pub reg: c_uchar,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_op {
    pub next: *mut stack_op,
    pub dest: op_dest,
    pub src: op_src,
}

extern "C" {
    pub fn arch_ftrace_match(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn arch_initial_func_cfi_state(state: *mut cfi_init_state);
}
extern "C" {
    pub fn arch_callee_saved_reg(reg: c_uchar) -> bool;
}
extern "C" {
    pub fn arch_jump_destination(insn: *mut instruction) -> c_ulong;
}
extern "C" {
    pub fn arch_insn_adjusted_addend(insn: *mut instruction, reloc: *mut reloc) -> i64;
}
extern "C" {
    pub fn arch_adjusted_addend(reloc: *mut reloc) -> u64;
}
extern "C" {
    pub fn arch_decode_hint_reg(sp_reg: u8, base: *mut c_int) -> c_int;
}
extern "C" {
    pub fn arch_is_retpoline(sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn arch_is_rethunk(sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn arch_is_embedded_insn(sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn arch_rewrite_retpolines(file: *mut objtool_file) -> c_int;
}
extern "C" {
    pub fn arch_pc_relative_reloc(reloc: *mut reloc) -> bool;
}
extern "C" {
    pub fn arch_absolute_reloc(elf: *mut elf, reloc: *mut reloc) -> bool;
}
extern "C" {
    pub fn arch_reloc_size(reloc: *mut reloc) -> c_uint;
}
extern "C" {
    pub fn arch_jump_table_sym_offset(reloc: *mut reloc, table: *mut reloc) -> c_ulong;
}

extern "C" {
    pub fn arch_disas_info_init(dinfo: *mut disassemble_info) -> c_int;
}

