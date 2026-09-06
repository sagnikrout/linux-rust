//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/basic.c
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


    {
    "empty prog",
    .insns = {
    },
    .errstr = "last insn is not an exit or jmp",
    .result = REJECT,
    },
    {
    "only exit insn",
    .insns = {
    BPF_EXIT_INSN(),
    },
    .errstr = "R0 !read_ok",
    .result = REJECT,
    },
    {
    "no bpf_exit",
    .insns = {
    BPF_ALU64_REG(BPF_MOV, BPF_REG_0, BPF_REG_2),
    },
    .errstr = "not an exit",
    .result = REJECT,
    },
