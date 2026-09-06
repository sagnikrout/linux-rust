//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/text-patching.h
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
// Currently, the max observed size in the kernel code is
// JUMP_LABEL_NOP_SIZE/RELATIVEJUMP_SIZE, which are 5.
// Raise it if needed.
//
pub const TEXT_POKE_MAX_OPCODE_SIZE: c_int = 5;
extern "C" {
    pub fn text_poke_early(addr: *mut c_void, opcode: *const c_void, len: usize);
}
extern "C" {
    pub fn text_poke_apply_relocation(buf: *mut u8, instr: *const *const u8, instrlen: usize, repl: *mut u8, repl_len: usize);
}
//
// Clear and restore the kernel write-protection flag on the local CPU.
// Allows the kernel to edit read-only pages.
// Side-effect: any interrupt handler running between save and restore will have
// the ability to write to read-only pages.
//
// Warning:
// Code patching in the UP case is safe if NMIs and MCE handlers are stopped and
// no thread can be preempted in the instructions being modified (no iret to an
// invalid instruction possible) or if the instructions are changed from a
// consistent state to another consistent state atomically.
// On the local CPU you need to be protected against NMI or MCE handlers seeing
// an inconsistent instruction while you patch.
//
extern "C" {
    pub fn smp_text_poke_sync_each_cpu();
}

extern "C" {
    pub fn smp_text_poke_int3_handler(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn smp_text_poke_single(addr: *mut c_void, opcode: *const c_void, len: usize, emulate: *const c_void);
}
extern "C" {
    pub fn smp_text_poke_batch_add(addr: *mut c_void, opcode: *const c_void, len: usize, emulate: *const c_void);
}
extern "C" {
    pub fn smp_text_poke_batch_finish();
}
pub const INT3_INSN_SIZE: c_int = 1;
pub const INT3_INSN_OPCODE: c_uint = 0xCC;
pub const RET_INSN_SIZE: c_int = 1;
pub const RET_INSN_OPCODE: c_uint = 0xC3;
pub const CALL_INSN_SIZE: c_int = 5;
pub const CALL_INSN_OPCODE: c_uint = 0xE8;
pub const JMP32_INSN_SIZE: c_int = 5;
pub const JMP32_INSN_OPCODE: c_uint = 0xE9;
pub const JMP8_INSN_SIZE: c_int = 2;
pub const JMP8_INSN_OPCODE: c_uint = 0xEB;
pub const DISP32_SIZE: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub union text_poke_insn {
    pub text: [u8; TEXT_POKE_MAX_OPCODE_SIZE],
    pub opcode: u8,
    pub disp: i32,
    pub __attribute__((packed)): },
}

//
// Hide the addresses to avoid the compiler folding in constants when
// referencing code, these can mess up annotations like
// ANNOTATE_NOENDBR.
//
// Ensure that for JMP8 the displacement
// actually fits the signed byte.
//

//
// The INT3 handler in entry_64.S adds a gap between the
// stack where the break point happened, and the saving of
// pt_regs. We can extend the original stack because of
// this gap. See the idtentry macro's X86_TRAP_BP logic.
//
// Similarly, entry_32.S will have a gap on the stack for
// (any) hardware exception and pt_regs; see the
// FIXUP_FRAME macro.
//
// (unsigned long *)regs->sp = val;

