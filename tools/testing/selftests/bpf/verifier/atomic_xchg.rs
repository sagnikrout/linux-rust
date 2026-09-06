//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/atomic_xchg.c
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
    "atomic exchange smoketest - 64bit",
    .insns = {
// val = 3;
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 3),
// old = atomic_xchg(&val, 4);
    BPF_MOV64_IMM(BPF_REG_1, 4),
    BPF_ATOMIC_OP(BPF_DW, BPF_XCHG, BPF_REG_10, BPF_REG_1, -8),
// if (old != 3) exit(1);
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 3, 2),
    BPF_MOV64_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
// if (val != 4) exit(2);
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_10, -8),
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 4, 2),
    BPF_MOV64_IMM(BPF_REG_0, 2),
    BPF_EXIT_INSN(),
// exit(0);
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
    {
    "atomic exchange smoketest - 32bit",
    .insns = {
// val = 3;
    BPF_ST_MEM(BPF_W, BPF_REG_10, -4, 3),
// old = atomic_xchg(&val, 4);
    BPF_MOV32_IMM(BPF_REG_1, 4),
    BPF_ATOMIC_OP(BPF_W, BPF_XCHG, BPF_REG_10, BPF_REG_1, -4),
// if (old != 3) exit(1);
    BPF_JMP32_IMM(BPF_JEQ, BPF_REG_1, 3, 2),
    BPF_MOV32_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
// if (val != 4) exit(2);
    BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_10, -4),
    BPF_JMP32_IMM(BPF_JEQ, BPF_REG_0, 4, 2),
    BPF_MOV32_IMM(BPF_REG_0, 2),
    BPF_EXIT_INSN(),
// exit(0);
    BPF_MOV32_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
