//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/atomic_invalid.c
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


    {								\
    "atomic " #op " access through non-pointer ",		\
    .insns = {						\
    BPF_MOV64_IMM(BPF_REG_0, 1),			\
    BPF_MOV64_IMM(BPF_REG_1, 0),			\
    BPF_ATOMIC_OP(BPF_DW, op, BPF_REG_1, BPF_REG_0, -8), \
    BPF_MOV64_IMM(BPF_REG_0, 0),			\
    BPF_EXIT_INSN(),				\
    },							\
    .result = REJECT,					\
    .errstr = "R1 invalid mem access 'scalar'"		\
    }
    __INVALID_ATOMIC_ACCESS_TEST(BPF_ADD),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_ADD | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_ADD),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_ADD | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_AND),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_AND | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_OR),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_OR | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_XOR),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_XOR | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_XCHG),
    __INVALID_ATOMIC_ACCESS_TEST(BPF_CMPXCHG),
