//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/alternative.h
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

pub const ALT_FLAGS_SHIFT: c_int = 16;

//
// The patching flags are part of the upper bits of the @ft_flags parameter when
// specifying them. The split is currently like this:
//
// [31... flags ...16][15... CPUID feature bit ...0]
//
// but since this is all hidden in the macros argument being split, those fields can be
// extended in the future to fit in a u64 or however the need arises.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_instr {
    pub /: *mut *mut s32 instr_offset; / original instruction,
    pub /: *mut *mut s32 repl_offset; / offset to replacement instruction,
    pub /: *mut *mut u32 cpuid: 16; / CPUID bit set for replacement,
    pub /: *mut *mut u32 flags: 16; / patching control flags,
}

//
// Debug flag that can be tested to see whether alternative
// instructions were patched in already:
//
extern "C" {
    pub fn alternative_instructions();
}
extern "C" {
    pub fn apply_alternatives(start: *mut alt_instr, end: *mut alt_instr);
}
extern "C" {
    pub fn apply_retpolines(start: *mut i32, end: *mut i32);
}
extern "C" {
    pub fn apply_returns(start: *mut i32, end: *mut i32);
}
extern "C" {
    pub fn apply_seal_endbr(start: *mut i32, end: *mut i32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callthunk_sites {
    pub call_end: *mut *mut s32 call_start,,
}

extern "C" {
    pub fn callthunks_patch_builtin_calls();
}
extern "C" {
    pub fn x86_call_depth_emit_accounting(pprog: *mut u8, func: *mut c_void, ip: *mut c_void) -> c_int;
}

extern "C" {
    pub fn its_init_mod(mod: *mut module);
}
extern "C" {
    pub fn its_fini_mod(mod: *mut module);
}
extern "C" {
    pub fn its_free_mod(mod: *mut module);
}

extern "C" {
    pub fn cpu_wants_rethunk() -> bool;
}
extern "C" {
    pub fn cpu_wants_rethunk_at(addr: *mut c_void) -> bool;
}

// alternative assembly primitive:

// If @feature is set, patch in @newinstr_yes, otherwise @newinstr_no.

//
// Alternative instructions for different CPU types or capabilities.
//
// This allows to use optimized instructions even on generic binary
// kernels.
//
// length of oldinstr must be longer or equal the length of newinstr
// It can be padded with nops as needed.
//
// For non barrier like inlines please define new variants
// without volatile and memory clobber.
//

//
// Alternative inline assembly with input.
//
// Peculiarities:
// No memory clobber here.
// Argument numbers start with 1.
// Leaving an unused argument 0 to keep API compatibility.
//

// Like alternative_input, but with a single output argument

//
// Like alternative_io, but for replacing a direct call with another one.
//
// Use the %c operand modifier which is the generic way to print a bare
// constant expression with all syntax-specific punctuation omitted. %P
// is the x86-specific variant which can handle constants too, for
// historical reasons, but it should be used primarily for PIC
// references: i.e., if used for a function, it would add the PLT
// suffix.
//

//
// Like alternative_call, but there are two features and respective functions.
// If CPU has feature2, function2 is used.
// Otherwise, if CPU has feature1, function1 is used.
// Otherwise, old function is used.
//

// Macro for creating assembler functions avoiding any C magic.

extern "C" {
    pub fn BUG_func();
}
extern "C" {
    pub fn nop_func();
}

//
// Issue one struct alt_instr descriptor entry (need to put it into
// the section .altinstructions, see below). This entry contains
// enough information for the alternatives patching code to patch an
// instruction. See apply_alternatives().
//
// Define an alternative between two instructions. If @feature is
// present, early code in apply_alternatives() replaces @oldinstr with
// @newinstr. ".skip" directive takes care of proper instruction padding
// in case @newinstr is longer than @oldinstr.
//

//
// Same as ALTERNATIVE macro above but for two alternatives. If CPU
// has @feature1, it replaces @oldinstr with @newinstr1. If CPU has
// @feature2, it replaces @oldinstr with @feature2.
//
// If @feature is set, patch in @newinstr_yes, otherwise @newinstr_no.

