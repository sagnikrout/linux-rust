//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_signed_loader_lsm.c
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


// SPDX-License-Identifier: GPL-2.0

    char _license[] SEC("license") = "GPL";
    __u32 monitored_tid;
    int sig_keyring_serial;
    int sig_keyring_type;
    int sig_verdict;
    int seen;
    SEC("lsm/bpf_prog_load")
    int BPF_PROG(inspect_prog_load, struct bpf_prog *prog, union bpf_attr *attr,
    struct bpf_token *token, bool kernel)
    {
    let mut tid: __u32 = bpf_get_current_pid_tgid() & 0xffffffff;
    if (!monitored_tid || tid != monitored_tid)
    return 0;
    seen++;
    sig_keyring_serial = prog.aux.sig.keyring_serial;
    sig_keyring_type = prog.aux.sig.keyring_type;
    sig_verdict = prog.aux.sig.verdict;
    return 0;
    }
