//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/disasm.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    pub e_flags: u32,
    pub e_machine: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch {
// @name: name such as "x86" or "powerpc".
    pub name: *const c_char,
    pub instructions: *const ins,
    pub nr_instructions: usize,
    pub nr_instructions_allocated: usize,
    pub insn_suffix: *const c_char,
    pub model: c_uint,
    pub family: c_uint,
// @id: ELF machine and flags associated with arch.
    pub id: e_machine_and_e_flags,
    pub sorted_instructions: bool,
    pub comment_char: c_char,
    pub skip_functions_char: c_char,
    pub register_char: c_char,
    pub memory_ref_char: c_char,
    pub imm_char: c_char,
    pub objdump: },
    pub ins2): *const c_char,
    pub name): *const *const *const *const ins_ops (associate_instruction_ops)(arch arch, char,

    pub dl): *mut disasm_line,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *const ins_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub raw: *mut c_char,
    pub name: *mut c_char,
    pub sym: *mut symbol,
    pub addr: u64,
    pub offset: i64,
    pub offset_avail: bool,
    pub outside: bool,
    pub multi_regs: bool,
    pub mem_ref: bool,
    pub target: },
    pub raw: *mut c_char,
    pub name: *mut c_char,
    pub addr: u64,
    pub multi_regs: bool,
    pub mem_ref: bool,
    pub source: },
    pub ins: ins,
    pub ops: *mut ins_operands,
    pub locked: },
    pub raw_comment: *mut c_char,
    pub raw_func_start: *mut c_char,
    pub jump: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_ops {
    pub ops): *mut *mut void (free)(struct ins_operands,
    pub dl): *mut disasm_line,
    pub max_ins_name): *mut *mut ins_operands ops, int,
    pub is_jump: bool,
    pub is_call: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotate_args {
    pub arch: *const arch,
    pub ms: *mut map_symbol,
    pub options: *mut annotation_options,
    pub offset: i64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
}

extern "C" {
    pub fn arch__is_x86(arch: *const arch) -> bool;
}
extern "C" {
    pub fn arch__is_powerpc(arch: *const arch) -> bool;
}
extern "C" {
    pub fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops) -> c_int;
}
extern "C" {
    pub fn ins__is_call(ins: *const ins) -> bool;
}
extern "C" {
    pub fn ins__is_jump(ins: *const ins) -> bool;
}
extern "C" {
    pub fn ins__is_fused(arch: *const arch, ins1: *const c_char, ins2: *const c_char) -> bool;
}
extern "C" {
    pub fn ins__is_ret(ins: *const ins) -> bool;
}
extern "C" {
    pub fn ins__is_lock(ins: *const ins) -> bool;
}
extern "C" {
    pub fn disasm_line__free(dl: *mut disasm_line);
}
extern "C" {
    pub fn jump__delete(ops: *mut ins_operands);
}
extern "C" {
    pub fn symbol__disassemble(sym: *mut symbol, args: *mut annotate_args) -> c_int;
}
