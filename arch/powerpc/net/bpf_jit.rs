//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/net/bpf_jit.h
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
// bpf_jit.h: BPF JIT compiler for PPC
//
// Copyright 2011 Matt Evans <matt@ozlabs.org>, IBM Corporation
// 2016 Naveen N. Rao <naveen.n.rao@linux.vnet.ibm.com>
//

pub const FUNCTION_DESCR_SIZE: c_int = 24;

pub const FUNCTION_DESCR_SIZE: c_int = 0;

pub const BPF_INSN_SAFETY: c_int = 64;
pub const BPF_PPC_TAILCALL: c_int = 8;

// Long jump; (unconditional 'branch')

// "cond" here covers BO:BI fields.

// When constant jump offset is known prior

//
// Sign-extended 32-bit immediate load
//
// If this is a dummy pass (!image), account for
// maximum possible instructions.
//

// If dummy pass (!image), account for maximum possible instructions

//
// The fly in the ointment of code size changing from pass to pass is
// avoided by padding the short branch case with a NOP.	 If code size differs
// with different branch reaches we will have the issue of code moving from
// one pass to the next and will need a few passes to converge on a stable
// state.
//

// Flip the 'T or F' bit to invert comparison */      \
// To create a branch condition, select a bit of cr0...
pub const CR0_LT: c_int = 0;
pub const CR0_GT: c_int = 1;
pub const CR0_EQ: c_int = 2;
// ...and modify BO[3]
pub const COND_CMP_TRUE: c_uint = 0x100;
pub const COND_CMP_FALSE: c_uint = 0x000;
// Together, they make all required comparisons:

pub const SEEN_FUNC: c_uint = 0x20000000 /* might call external helpers */;
pub const SEEN_TAILCALL: c_uint = 0x40000000 /* uses tail calls */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codegen_context {
//
// This is used to track register usage as well
// as calls to external helpers.
// - register usage is tracked with corresponding
// bits (r3-r31)
// - rest of the bits can be used to track other
// things -- for now, we use bits 0 to 2
// encoded in SEEN_* macros above
//
    pub seen: c_uint,
    pub idx: c_uint,
    pub stack_size: c_uint,
    pub 3]: int b2p[MAX_BPF_JIT_REG +,
    pub exentry_idx: c_uint,
    pub alt_exit_addr: c_uint,
    pub arena_vm_start: u64,
    pub user_vm_start: u64,
    pub is_subprog: bool,
    pub exception_boundary: bool,
    pub exception_cb: bool,
    pub priv_sp: *mut void __percpu,
    pub priv_stack_size: c_uint,
}

// Memory size & magic-value to detect private stack overflow/underflow
pub const PRIV_STACK_GUARD_SZ: c_int = 16;
pub const PRIV_STACK_GUARD_VAL: c_uint = 0xEB9F12345678eb9fULL;

extern "C" {
    pub fn bpf_jit_init_reg_mapping(ctx: *mut codegen_context);
}
extern "C" {
    pub fn bpf_jit_emit_func_call_rel(image: *mut u32, fimage: *mut u32, ctx: *mut codegen_context, func: u64) -> c_int;
}
extern "C" {
    pub fn bpf_jit_build_prologue(image: *mut u32, ctx: *mut codegen_context);
}
extern "C" {
    pub fn bpf_jit_build_epilogue(image: *mut u32, ctx: *mut codegen_context);
}
extern "C" {
    pub fn bpf_jit_build_fentry_stubs(image: *mut u32, ctx: *mut codegen_context);
}
extern "C" {
    pub fn bpf_jit_realloc_regs(ctx: *mut codegen_context);
}
extern "C" {
    pub fn bpf_jit_emit_exit_insn(image: *mut u32, ctx: *mut codegen_context, tmp_reg: c_int, exit_addr: c_long) -> c_int;
}
extern "C" {
    pub fn store_func_meta(image: *mut u32, ctx: *mut codegen_context, func_meta: u64, func_meta_off: c_int);
}

