//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/bpf_st_mem.c
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
    "BPF_ST_MEM stack imm non-zero",
    .insns = {
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 42),
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_10, -8),
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_0, -42),
// if value is tracked correctly R0 is zero
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
// Use prog type that requires return value in range [0, 1]
    .prog_type = BPF_PROG_TYPE_SK_LOOKUP,
    .expected_attach_type = BPF_SK_LOOKUP,
    .runs = -1,
    },
    {
    "BPF_ST_MEM stack imm zero",
    .insns = {
// mark stack 0000 0000
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
// read and sum a few bytes
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_LDX_MEM(BPF_B, BPF_REG_1, BPF_REG_10, -8),
    BPF_ALU64_REG(BPF_ADD, BPF_REG_0, BPF_REG_1),
    BPF_LDX_MEM(BPF_B, BPF_REG_1, BPF_REG_10, -4),
    BPF_ALU64_REG(BPF_ADD, BPF_REG_0, BPF_REG_1),
    BPF_LDX_MEM(BPF_B, BPF_REG_1, BPF_REG_10, -1),
    BPF_ALU64_REG(BPF_ADD, BPF_REG_0, BPF_REG_1),
// if value is tracked correctly R0 is zero
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
// Use prog type that requires return value in range [0, 1]
    .prog_type = BPF_PROG_TYPE_SK_LOOKUP,
    .expected_attach_type = BPF_SK_LOOKUP,
    .runs = -1,
    },
    {
    "BPF_ST_MEM stack imm zero, variable offset",
    .insns = {
// set fp[-16], fp[-24] to zeros
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -16, 0),
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -24, 0),
// r0 = random value in range [-32, -15]
    BPF_EMIT_CALL(BPF_FUNC_get_prandom_u32),
    BPF_JMP_IMM(BPF_JLE, BPF_REG_0, 16, 2),
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    BPF_ALU64_IMM(BPF_SUB, BPF_REG_0, 32),
// fp[r0] = 0, make a variable offset write of zero,
// this should preserve zero marks on stack.
//
    BPF_ALU64_REG(BPF_ADD, BPF_REG_0, BPF_REG_10),
    BPF_ST_MEM(BPF_B, BPF_REG_0, 0, 0),
// r0 = fp[-20], if variable offset write was tracked correctly
// r0 would be a known zero.
//
    BPF_LDX_MEM(BPF_B, BPF_REG_0, BPF_REG_10, -20),
// Would fail return code verification if r0 range is not tracked correctly.
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
// Use prog type that requires return value in range [0, 1]
    .prog_type = BPF_PROG_TYPE_SK_LOOKUP,
    .expected_attach_type = BPF_SK_LOOKUP,
    .runs = -1,
    },
    {
    "BPF_ST_MEM stack imm sign",
// Check if verifier correctly reasons about sign of an
// immediate spilled to stack by BPF_ST instruction.
//
// fp[-8] = -44;
// r0 = fp[-8];
// if r0 s< 0 goto ret0;
// r0 = -1;
// exit;
// ret0:
// r0 = 0;
// exit;
//
    .insns = {
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, -44),
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_10, -8),
    BPF_JMP_IMM(BPF_JSLT, BPF_REG_0, 0, 2),
    BPF_MOV64_IMM(BPF_REG_0, -1),
    BPF_EXIT_INSN(),
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
// Use prog type that requires return value in range [0, 1]
    .prog_type = BPF_PROG_TYPE_SK_LOOKUP,
    .expected_attach_type = BPF_SK_LOOKUP,
    .result = VERBOSE_ACCEPT,
    .runs = -1,
    .errstr = "0: (7a) *(u64 *)(r10 -8) = -44        ; R10=fp0 fp-8=-44\
    2: (c5) if r0 s< 0x0 goto pc+2\
    R0=-44",
    },
