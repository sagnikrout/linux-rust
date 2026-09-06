//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2012 Regents of the University of California
//

pub const PTRACE_GETFDPIC: c_int = 33;
pub const PTRACE_GETFDPIC_EXEC: c_int = 0;
pub const PTRACE_GETFDPIC_INTERP: c_int = 1;
//
// User-mode register state for core dumps, ptrace, sigcontext
//
// This decouples struct pt_regs from the userspace ABI.
// struct user_regs_struct must form a prefix of struct pt_regs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regs_struct {
    pub pc: c_ulong,
    pub ra: c_ulong,
    pub sp: c_ulong,
    pub gp: c_ulong,
    pub tp: c_ulong,
    pub t0: c_ulong,
    pub t1: c_ulong,
    pub t2: c_ulong,
    pub s0: c_ulong,
    pub s1: c_ulong,
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
    pub a4: c_ulong,
    pub a5: c_ulong,
    pub a6: c_ulong,
    pub a7: c_ulong,
    pub s2: c_ulong,
    pub s3: c_ulong,
    pub s4: c_ulong,
    pub s5: c_ulong,
    pub s6: c_ulong,
    pub s7: c_ulong,
    pub s8: c_ulong,
    pub s9: c_ulong,
    pub s10: c_ulong,
    pub s11: c_ulong,
    pub t3: c_ulong,
    pub t4: c_ulong,
    pub t5: c_ulong,
    pub t6: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_f_ext_state {
    pub f: [__u32; 32],
    pub fcsr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_d_ext_state {
    pub f: [__u64; 32],
    pub fcsr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_q_ext_state {
    pub __attribute__((aligned(16))): __u64 f[64],
    pub fcsr: __u32,
//
// Reserved for expansion of sigcontext structure.  Currently zeroed
// upon signal, and must be zero upon sigreturn.
//
    pub reserved: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_ctx_hdr {
    pub magic: __u32,
    pub size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_extra_ext_header {
    pub __attribute__((aligned(16))): __u32 __padding[129],
//
// Reserved for expansion of sigcontext structure.  Currently zeroed
// upon signal, and must be zero upon sigreturn.
//
    pub reserved: __u32,
    pub hdr: __riscv_ctx_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __riscv_fp_state {
    pub f: __riscv_f_ext_state,
    pub d: __riscv_d_ext_state,
    pub q: __riscv_q_ext_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_v_ext_state {
    pub vstart: c_ulong,
    pub vl: c_ulong,
    pub vtype: c_ulong,
    pub vcsr: c_ulong,
    pub vlenb: c_ulong,
    pub datap: *mut c_void,
//
// In signal handler, datap will be set a correct user stack offset
// and vector registers will be copied to the address of datap
// pointer.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __riscv_v_regset_state {
    pub vstart: c_ulong,
    pub vl: c_ulong,
    pub vtype: c_ulong,
    pub vcsr: c_ulong,
    pub vlenb: c_ulong,
    pub vreg: [c_char; ],
}

//
// According to spec: The number of bits in a single vector register,
// VLEN >= ELEN, which must be a power of 2, and must be no greater than
// 2^16 = 65536bits = 8192bytes
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sc_riscv_cfi_state {
    pub /: *mut *mut unsigned long ss_ptr; / shadow stack pointer,
}

pub const PTRACE_CFI_BRANCH_LANDING_PAD_EN_BIT: c_int = 0;
pub const PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_BIT: c_int = 1;
pub const PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_BIT: c_int = 2;
pub const PTRACE_CFI_SHADOW_STACK_EN_BIT: c_int = 3;
pub const PTRACE_CFI_SHADOW_STACK_LOCK_BIT: c_int = 4;
pub const PTRACE_CFI_SHADOW_STACK_PTR_BIT: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __cfi_status {
    pub cfi_state: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_cfi_state {
    pub cfi_status: __cfi_status,
    pub shstk_ptr: __u64,
}

