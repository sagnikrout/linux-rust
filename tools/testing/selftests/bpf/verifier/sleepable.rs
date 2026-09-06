//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/sleepable.c
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
    "sleepable fentry accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_TRACE_FENTRY,
    .kfunc = "bpf_fentry_test1",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable fexit accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_TRACE_FENTRY,
    .kfunc = "bpf_fentry_test1",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable fmod_ret accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_MODIFY_RETURN,
    .kfunc = "bpf_fentry_test1",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable iter accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_TRACE_ITER,
    .kfunc = "task",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable lsm accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_LSM,
    .kfunc = "bpf",
    .expected_attach_type = BPF_LSM_MAC,
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable uprobe accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_KPROBE,
    .kfunc = "bpf_fentry_test1",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable raw tracepoint accept",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_TRACE_RAW_TP,
    .kfunc = "sys_enter",
    .result = ACCEPT,
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
    {
    "sleepable raw tracepoint reject non-faultable",
    .insns = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    },
    .prog_type = BPF_PROG_TYPE_TRACING,
    .expected_attach_type = BPF_TRACE_RAW_TP,
    .kfunc = "sched_switch",
    .result = REJECT,
    .errstr = "Sleepable program cannot attach to non-faultable tracepoint",
    .flags = BPF_F_SLEEPABLE,
    .runs = -1,
    },
