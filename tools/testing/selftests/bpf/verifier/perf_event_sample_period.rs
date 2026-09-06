//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/perf_event_sample_period.c
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
    "check bpf_perf_event_data.sample_period byte load permitted",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),

    BPF_LDX_MEM(BPF_B, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period)),

    BPF_LDX_MEM(BPF_B, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period) + 7),

    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .prog_type = BPF_PROG_TYPE_PERF_EVENT,
    },
    {
    "check bpf_perf_event_data.sample_period half load permitted",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),

    BPF_LDX_MEM(BPF_H, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period)),

    BPF_LDX_MEM(BPF_H, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period) + 6),

    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .prog_type = BPF_PROG_TYPE_PERF_EVENT,
    },
    {
    "check bpf_perf_event_data.sample_period word load permitted",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),

    BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period)),

    BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period) + 4),

    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .prog_type = BPF_PROG_TYPE_PERF_EVENT,
    },
    {
    "check bpf_perf_event_data.sample_period dword load permitted",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_1,
    offsetof(struct bpf_perf_event_data, sample_period)),
    BPF_EXIT_INSN(),
    },
    .result = ACCEPT,
    .prog_type = BPF_PROG_TYPE_PERF_EVENT,
    },
