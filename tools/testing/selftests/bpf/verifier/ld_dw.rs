//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/ld_dw.c
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
    "ld_dw: xor semi-random 64 bit imms, test 1",
    .insns = { },
    .data = { },
    .fill_helper = bpf_fill_rand_ld_dw,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .result = ACCEPT,
    .retval = 4090,
    },
    {
    "ld_dw: xor semi-random 64 bit imms, test 2",
    .insns = { },
    .data = { },
    .fill_helper = bpf_fill_rand_ld_dw,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .result = ACCEPT,
    .retval = 2047,
    },
    {
    "ld_dw: xor semi-random 64 bit imms, test 3",
    .insns = { },
    .data = { },
    .fill_helper = bpf_fill_rand_ld_dw,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .result = ACCEPT,
    .retval = 511,
    },
    {
    "ld_dw: xor semi-random 64 bit imms, test 4",
    .insns = { },
    .data = { },
    .fill_helper = bpf_fill_rand_ld_dw,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .result = ACCEPT,
    .retval = 5,
    },
    {
    "ld_dw: xor semi-random 64 bit imms, test 5",
    .insns = { },
    .data = { },
    .fill_helper = bpf_fill_rand_ld_dw,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .result = ACCEPT,
    .retval = 1000000 - 6,
    },
