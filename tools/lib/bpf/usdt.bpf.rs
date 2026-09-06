//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/usdt.bpf.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

// Below types and maps are internal implementation details of libbpf's USDT
// support and are subjects to change. Also, bpf_usdt_xxx() API helpers should
// be considered an unstable API as well and might be adjusted based on user
// feedback from using libbpf's USDT support in production.
//
// User can override BPF_USDT_MAX_SPEC_CNT to change default size of internal
// map that keeps track of USDT argument specifications. This might be
// necessary if there are a lot of USDT attachments.
//

pub const BPF_USDT_MAX_SPEC_CNT: c_int = 256;

// User can override BPF_USDT_MAX_IP_CNT to change default size of internal
// map that keeps track of IP (memory address) mapping to USDT argument
// specification.
// Note, if kernel supports BPF cookies, this map is not used and could be
// resized all the way to 1 to save a bit of memory.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __bpf_usdt_arg_type {
    BPF_USDT_ARG_CONST,
    BPF_USDT_ARG_REG,
    BPF_USDT_ARG_REG_DEREF,
    BPF_USDT_ARG_SIB,
}

//
// This struct layout is designed specifically to be backwards/forward
// compatible between libbpf versions for ARG_CONST, ARG_REG, and
// ARG_REG_DEREF modes. ARG_SIB requires libbpf v1.7+.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bpf_usdt_arg_spec {
// u64 scalar interpreted depending on arg_type, see below
    pub val_off: __u64,

// arg location case, see bpf_usdt_arg() for details
    pub 8: __bpf_usdt_arg_type arg_type:,
// index register offset within struct pt_regs
    pub 12: __u16 idx_reg_off:,
// scale factor for index register (1, 2, 4, or 8)
    pub 4: __u16 scale_bitshift:,
// reserved for future use, keeps reg_off offset stable
    pub 8: __u8 __reserved:,

    pub 8: __u8 __reserved:,
    pub 12: __u16 idx_reg_off:,
    pub 4: __u16 scale_bitshift:,
    pub 8: __bpf_usdt_arg_type arg_type:,

// offset of referenced register within struct pt_regs
    pub reg_off: c_short,
// whether arg should be interpreted as signed value
    pub arg_signed: bool,
// number of bits that need to be cleared and, optionally,
// sign-extended to cast arguments that are 1, 2, or 4 bytes
// long into final 8-byte u64/s64 value returned to user
//
    pub arg_bitshift: c_char,
}

// should match USDT_MAX_ARG_CNT in usdt.c exactly
pub const BPF_USDT_MAX_ARG_CNT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bpf_usdt_spec {
    pub args: [__bpf_usdt_arg_spec; BPF_USDT_MAX_ARG_CNT],
    pub usdt_cookie: __u64,
    pub arg_cnt: c_short,
}

extern "C" {
    pub fn bpf_get_attach_cookie(_arg: ctx) -> return;
}
// Return number of USDT arguments defined for currently traced USDT.
// Returns the size in bytes of the #*arg_num* (zero-indexed) USDT argument.
// Returns negative error if argument is not found or arg_num is invalid.
//
// arg_spec->arg_bitshift = 64 - arg_sz * 8
// so: arg_sz = (64 - arg_spec->arg_bitshift) / 8
//
// Fetch USDT argument #*arg_num* (zero-indexed) and put its value into *res.
// Returns 0 on success; negative error, otherwise.
// On error *res is guaranteed to be set to zero.
//
// res = 0;
// Arg is just a constant ("-4@$-9" in USDT arg spec).
// value is recorded in arg_spec->val_off directly.
//
// Arg is in a register (e.g, "8@%rax" in USDT arg spec),
// so we read the contents of that register directly from
// struct pt_regs. To keep things simple user-space parts
// record offsetof(struct pt_regs, <regname>) in arg_spec->reg_off.
//
// Arg is in memory addressed by register, plus some offset
// (e.g., "-4@-1204(%rbp)" in USDT arg spec). Register is
// identified like with BPF_USDT_ARG_REG case, and the offset
// is in arg_spec->val_off. We first fetch register contents
// from pt_regs, then do another user-space probe read to
// fetch argument value itself.
//

// Arg is in memory addressed by SIB (Scale-Index-Base) mode
// (e.g., "-1@-96(%rbp,%rax,8)" in USDT arg spec). We first
// fetch the base register contents and the index register
// contents from pt_regs. Then we calculate the final address
// as base + (index * scale) + offset, and do a user-space
// probe read to fetch the argument value.
//

// cast arg from 1, 2, or 4 bytes to final 8 byte size clearing
// necessary upper arg_bitshift bits, with sign extension if argument
// is signed
//
// res = val;
// Retrieve user-specified cookie value provided during attach as
// bpf_usdt_opts.usdt_cookie. This serves the same purpose as BPF cookie
// returned by bpf_get_attach_cookie(). Libbpf's support for USDT is itself
// utilizing BPF cookies internally, so user can't use BPF cookie directly
// for USDT programs and has to use bpf_usdt_cookie() API instead.
//
// we rely on ___bpf_apply() and ___bpf_narg() macros already defined in bpf_tracing.h

//
// BPF_USDT serves the same purpose for USDT handlers as BPF_PROG for
// tp_btf/fentry/fexit BPF programs and BPF_KPROBE for kprobes.
// Original struct pt_regs * context is preserved as 'ctx' argument.
//

