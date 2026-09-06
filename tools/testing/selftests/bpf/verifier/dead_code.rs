//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/dead_code.c
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
    "dead code: start",
    .insns = {
    BPF_JMP_IMM(BPF_JA, 0, 0, 2),
// unpriv: nospec (inserted to prevent "R9 !read_ok")
    BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
    BPF_JMP_IMM(BPF_JA, 0, 0, 2),
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 10, -4),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: mid 1",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 0, 1),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 10, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: mid 2",
    .insns = {
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32),
    BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 4),
    BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 1),
    BPF_JMP_IMM(BPF_JA, 0, 0, 2),
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "dead code: end 1",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 10, 1),
    BPF_EXIT_INSN(),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: end 2",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 10, 1),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 12),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: end 3",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 8, 1),
    BPF_EXIT_INSN(),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 10, 1),
    BPF_JMP_IMM(BPF_JA, 0, 0, 1),
    BPF_MOV64_IMM(BPF_REG_0, 12),
    BPF_JMP_IMM(BPF_JA, 0, 0, -5),
    },
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: tail of main + func",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 8, 1),
    BPF_EXIT_INSN(),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 1),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 12),
    BPF_EXIT_INSN(),
    },
    .errstr_unpriv = "loading/calling other bpf or kernel functions are allowed for",
    .result_unpriv = REJECT,
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: tail of main + two functions",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 8, 1),
    BPF_EXIT_INSN(),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 1),
    BPF_EXIT_INSN(),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 1),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 12),
    BPF_EXIT_INSN(),
    },
    .errstr_unpriv = "loading/calling other bpf or kernel functions are allowed for",
    .result_unpriv = REJECT,
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: function in the middle and mid of another func",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_1, 7),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 3),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 12),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 7),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_1, 7, 1),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, -5),
    BPF_EXIT_INSN(),
    },
    .errstr_unpriv = "loading/calling other bpf or kernel functions are allowed for",
    .result_unpriv = REJECT,
    .result = ACCEPT,
    .retval = 7,
    },
    {
    "dead code: middle of main before call",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_1, 2),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_1, 2, 1),
    BPF_MOV64_IMM(BPF_REG_1, 5),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 1),
    BPF_EXIT_INSN(),
    BPF_MOV64_REG(BPF_REG_0, BPF_REG_1),
    BPF_EXIT_INSN(),
    },
    .errstr_unpriv = "loading/calling other bpf or kernel functions are allowed for",
    .result_unpriv = REJECT,
    .result = ACCEPT,
    .retval = 2,
    },
    {
    "dead code: start of a function",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_1, 2),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 1),
    BPF_EXIT_INSN(),
    BPF_JMP_IMM(BPF_JA, 0, 0, 0),
    BPF_MOV64_REG(BPF_REG_0, BPF_REG_1),
    BPF_EXIT_INSN(),
    },
    .errstr_unpriv = "loading/calling other bpf or kernel functions are allowed for",
    .result_unpriv = REJECT,
    .result = ACCEPT,
    .retval = 2,
    },
    {
    "dead code: zero extension",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4),
    BPF_JMP_IMM(BPF_JGE, BPF_REG_0, 0, 1),
    BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_10, -4),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 0,
    },
