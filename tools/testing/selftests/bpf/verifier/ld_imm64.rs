//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/ld_imm64.c
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
    "test1 ld_imm64",
    .insns = {
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_MOV64_IMM(BPF_REG_0, 2),
    BPF_EXIT_INSN(),
    },
    .errstr = "jump into the middle of ldimm64 insn 1",
    .errstr_unpriv = "jump into the middle of ldimm64 insn 1",
    .result = REJECT,
    },
    {
    "test2 ld_imm64",
    .insns = {
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "jump into the middle of ldimm64 insn 1",
    .errstr_unpriv = "jump into the middle of ldimm64 insn 1",
    .result = REJECT,
    },
    {
    "test3 ld_imm64",
    .insns = {
    BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 0),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_LD_IMM64(BPF_REG_0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test4 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test6 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
    BPF_RAW_INSN(0, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    },
    {
    "test7 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
    BPF_RAW_INSN(0, 0, 0, 0, 1),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "test8 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 1, 1),
    BPF_RAW_INSN(0, 0, 0, 0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "uses reserved fields",
    .result = REJECT,
    },
    {
    "test9 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
    BPF_RAW_INSN(0, 0, 0, 1, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test10 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
    BPF_RAW_INSN(0, BPF_REG_1, 0, 0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test11 ld_imm64",
    .insns = {
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
    BPF_RAW_INSN(0, 0, BPF_REG_1, 0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test12 ld_imm64",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_1, 0),
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, BPF_REG_1, 0, 1),
    BPF_RAW_INSN(0, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "not pointing to valid bpf_map",
    .result = REJECT,
    },
    {
    "test13 ld_imm64",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_1, 0),
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, BPF_REG_1, 0, 1),
    BPF_RAW_INSN(0, 0, BPF_REG_1, 0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid bpf_ld_imm64 insn",
    .result = REJECT,
    },
    {
    "test14 ld_imm64: reject 2nd imm != 0",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, BPF_REG_1,
    BPF_PSEUDO_MAP_FD, 0, 0),
    BPF_RAW_INSN(0, 0, 0, 0, 0xfefefe),
    BPF_EXIT_INSN(),
    },
    .fixup_map_hash_48b = { 1 },
    .errstr = "unrecognized bpf_ld_imm64 insn",
    .result = REJECT,
    },
