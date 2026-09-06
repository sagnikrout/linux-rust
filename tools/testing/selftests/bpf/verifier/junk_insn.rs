//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/junk_insn.c
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
    "junk insn",
    .insns = {
    BPF_RAW_INSN(0, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "unknown opcode 00",
    .result = REJECT,
    },
    {
    "junk insn2",
    .insns = {
    BPF_RAW_INSN(BPF_LDX | BPF_MEM | BPF_W, 0, 0, 0, 1),
    BPF_EXIT_INSN(),
    },
    .errstr = "BPF_LDX uses reserved fields",
    .result = REJECT,
    },
    {
    "junk insn3",
    .insns = {
    BPF_RAW_INSN(-1, 0, 0, 0, 0),
    BPF_EXIT_INSN(),
    },
    .errstr = "unknown opcode ff",
    .result = REJECT,
    },
    {
    "junk insn4",
    .insns = {
    BPF_RAW_INSN(-1, 0, 0, -1, -1),
    BPF_EXIT_INSN(),
    },
    .errstr = "unknown opcode ff",
    .result = REJECT,
    },
    {
    "junk insn5",
    .insns = {
    BPF_RAW_INSN(0x7f, 0, 0, -1, -1),
    BPF_EXIT_INSN(),
    },
    .errstr = "BPF_ALU uses reserved fields",
    .result = REJECT,
    },
