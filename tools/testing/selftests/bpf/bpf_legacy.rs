//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_legacy.h
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

// Functions to emit BPF_LD_ABS and BPF_LD_IND instructions.  We
// provide the "standard" names as synonyms of the corresponding GCC
// builtins. Note how the SKB argument is ignored.
//

// llvm builtin functions that eBPF C program may use to
// emit BPF_LD_ABS and BPF_LD_IND instructions
//
extern "C" {
    pub fn load_byte(skb: *mut c_void, asm("llvm.bpf.load.byte": unsigned long long off)) -> c_ulonglong;
}
extern "C" {
    pub fn load_half(skb: *mut c_void, asm("llvm.bpf.load.half": unsigned long long off)) -> c_ulonglong;
}
extern "C" {
    pub fn load_word(skb: *mut c_void, asm("llvm.bpf.load.word": unsigned long long off)) -> c_ulonglong;
}

