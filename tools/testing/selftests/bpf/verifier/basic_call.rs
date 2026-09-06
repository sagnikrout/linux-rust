//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/basic_call.c
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
    "invalid call insn1",
    .insns = {
    BPF_RAW_INSN(BPF_JMP | BPF_CALL | BPF_X, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "unknown opcode 8d",
    .result = REJECT,
    },
    {
    "invalid call insn2",
    .insns = {
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 1, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "BPF_CALL uses reserved",
    .result = REJECT,
    },
    {
    "invalid function call",
    .insns = {
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, 1234567),
    BPF_EXIT_INSN(),
    },
    .errstr = "invalid func unknown#1234567",
    .result = REJECT,
    },
    {
    "invalid argument register",
    .insns = {
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_cgroup_classid),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_cgroup_classid),
    BPF_EXIT_INSN(),
    },
    .errstr = "R1 !read_ok",
    .result = REJECT,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    },
    {
    "non-invalid argument register",
    .insns = {
    BPF_ALU64_REG(BPF_MOV, BPF_REG_6, BPF_REG_1),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_cgroup_classid),
    BPF_ALU64_REG(BPF_MOV, BPF_REG_1, BPF_REG_6),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_cgroup_classid),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    },
