//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/atomic_bounds.c
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
    "BPF_ATOMIC bounds propagation, mem.reg",
    .insns = {
// a = 0;
//
// Note this is implemented with two separate instructions,
// where you might think one would suffice:
//
// BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
//
// This is because BPF_ST_MEM doesn't seem to set the stack slot
// type to 0 when storing an immediate.
//
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_STX_MEM(BPF_DW, BPF_REG_10, BPF_REG_0, -8),
// b = atomic_fetch_add(&a, 1);
    BPF_MOV64_IMM(BPF_REG_1, 1),
    BPF_ATOMIC_OP(BPF_DW, BPF_ADD | BPF_FETCH, BPF_REG_10, BPF_REG_1, -8),
// Verifier should be able to tell that this infinite loop isn't reachable.
// if (b) while (true) continue;
    BPF_JMP_IMM(BPF_JNE, BPF_REG_1, 0, -1),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .result_unpriv = REJECT,
    .errstr_unpriv = "back-edge",
    },
