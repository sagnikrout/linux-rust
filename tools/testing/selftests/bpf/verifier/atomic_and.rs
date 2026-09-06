//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/atomic_and.c
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
    "BPF_ATOMIC_AND without fetch",
    .insns = {
// val = 0x110;
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0x110),
// atomic_and(&val, 0x011);
    BPF_MOV64_IMM(BPF_REG_1, 0x011),
    BPF_ATOMIC_OP(BPF_DW, BPF_AND, BPF_REG_10, BPF_REG_1, -8),
// if (val != 0x010) exit(2);
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_10, -8),
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0x010, 2),
    BPF_MOV64_IMM(BPF_REG_0, 2),
    BPF_EXIT_INSN(),
// r1 should not be clobbered, no BPF_FETCH flag
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0x011, 1),
    BPF_MOV64_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
    {
    "BPF_ATOMIC_AND with fetch",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 123),
// val = 0x110;
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0x110),
// old = atomic_fetch_and(&val, 0x011);
    BPF_MOV64_IMM(BPF_REG_1, 0x011),
    BPF_ATOMIC_OP(BPF_DW, BPF_AND | BPF_FETCH, BPF_REG_10, BPF_REG_1, -8),
// if (old != 0x110) exit(3);
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0x110, 2),
    BPF_MOV64_IMM(BPF_REG_0, 3),
    BPF_EXIT_INSN(),
// if (val != 0x010) exit(2);
    BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_10, -8),
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0x010, 2),
    BPF_MOV64_IMM(BPF_REG_1, 2),
    BPF_EXIT_INSN(),
// Check R0 wasn't clobbered (for fear of x86 JIT bug)
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 123, 2),
    BPF_MOV64_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
// exit(0);
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
    {
    "BPF_ATOMIC_AND with fetch 32bit",
    .insns = {
// r0 = (s64) -1
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_ALU64_IMM(BPF_SUB, BPF_REG_0, 1),
// val = 0x110;
    BPF_ST_MEM(BPF_W, BPF_REG_10, -4, 0x110),
// old = atomic_fetch_and(&val, 0x011);
    BPF_MOV32_IMM(BPF_REG_1, 0x011),
    BPF_ATOMIC_OP(BPF_W, BPF_AND | BPF_FETCH, BPF_REG_10, BPF_REG_1, -4),
// if (old != 0x110) exit(3);
    BPF_JMP32_IMM(BPF_JEQ, BPF_REG_1, 0x110, 2),
    BPF_MOV32_IMM(BPF_REG_0, 3),
    BPF_EXIT_INSN(),
// if (val != 0x010) exit(2);
    BPF_LDX_MEM(BPF_W, BPF_REG_1, BPF_REG_10, -4),
    BPF_JMP32_IMM(BPF_JEQ, BPF_REG_1, 0x010, 2),
    BPF_MOV32_IMM(BPF_REG_1, 2),
    BPF_EXIT_INSN(),
// Check R0 wasn't clobbered (for fear of x86 JIT bug)
// It should be -1 so add 1 to get exit code.
// 
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
    {
    "BPF_ATOMIC_AND with fetch - r0 as source reg",
    .insns = {
// val = 0x110;
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0x110),
// old = atomic_fetch_and(&val, 0x011);
    BPF_MOV64_IMM(BPF_REG_0, 0x011),
    BPF_ATOMIC_OP(BPF_DW, BPF_AND | BPF_FETCH, BPF_REG_10, BPF_REG_0, -8),
// if (old != 0x110) exit(3);
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0x110, 2),
    BPF_MOV64_IMM(BPF_REG_0, 3),
    BPF_EXIT_INSN(),
// if (val != 0x010) exit(2);
    BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_10, -8),
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0x010, 2),
    BPF_MOV64_IMM(BPF_REG_1, 2),
    BPF_EXIT_INSN(),
// exit(0);
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },