//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/wide_access.c
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


    { \
    "wide store to bpf_sock_addr." #field "[" #off "]", \
    .insns = { \
    BPF_MOV64_IMM(BPF_REG_0, 1), \
    BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, \
    offsetof(struct bpf_sock_addr, field[off])), \
    BPF_EXIT_INSN(), \
    }, \
    .result = res, \
    .prog_type = BPF_PROG_TYPE_CGROUP_SOCK_ADDR, \
    .expected_attach_type = BPF_CGROUP_UDP6_SENDMSG, \
    .errstr = err, \
    .flags = flgs, \
    }
// user_ip6[0] is u64 aligned
    BPF_SOCK_ADDR_STORE(user_ip6, 0, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_STORE(user_ip6, 1, REJECT,
    "invalid bpf_context access off=12 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_STORE(user_ip6, 2, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_STORE(user_ip6, 3, REJECT,
    "invalid bpf_context access off=20 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
// msg_src_ip6[0] is _not_ u64 aligned
    BPF_SOCK_ADDR_STORE(msg_src_ip6, 0, REJECT,
    "invalid bpf_context access off=44 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_STORE(msg_src_ip6, 1, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_STORE(msg_src_ip6, 2, REJECT,
    "invalid bpf_context access off=52 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_STORE(msg_src_ip6, 3, REJECT,
    "invalid bpf_context access off=56 size=8", 0),

    { \
    "wide load from bpf_sock_addr." #field "[" #off "]", \
    .insns = { \
    BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_1, \
    offsetof(struct bpf_sock_addr, field[off])), \
    BPF_MOV64_IMM(BPF_REG_0, 1), \
    BPF_EXIT_INSN(), \
    }, \
    .result = res, \
    .prog_type = BPF_PROG_TYPE_CGROUP_SOCK_ADDR, \
    .expected_attach_type = BPF_CGROUP_UDP6_SENDMSG, \
    .errstr = err, \
    .flags = flgs, \
    }
// user_ip6[0] is u64 aligned
    BPF_SOCK_ADDR_LOAD(user_ip6, 0, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_LOAD(user_ip6, 1, REJECT,
    "invalid bpf_context access off=12 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_LOAD(user_ip6, 2, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_LOAD(user_ip6, 3, REJECT,
    "invalid bpf_context access off=20 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
// msg_src_ip6[0] is _not_ u64 aligned
    BPF_SOCK_ADDR_LOAD(msg_src_ip6, 0, REJECT,
    "invalid bpf_context access off=44 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_LOAD(msg_src_ip6, 1, ACCEPT,
    core::ptr::null_mut(), 0),
    BPF_SOCK_ADDR_LOAD(msg_src_ip6, 2, REJECT,
    "invalid bpf_context access off=52 size=8",
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS),
    BPF_SOCK_ADDR_LOAD(msg_src_ip6, 3, REJECT,
    "invalid bpf_context access off=56 size=8", 0),
