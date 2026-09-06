//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_d_path_check_rdonly_mem.c
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
// Copyright (c) 2022 Google

    extern const int bpf_prog_active __ksym;
    SEC("fentry/security_inode_getattr")
    int BPF_PROG(d_path_check_rdonly_mem, struct path *path, struct kstat *stat,
    __u32 request_mask, unsigned int query_flags)
    {
    void *active;
    __u32 cpu;
    cpu = bpf_get_smp_processor_id();
    active = (void *)bpf_per_cpu_ptr(&bpf_prog_active, cpu);
    if (active) {
// FAIL here! 'active' points to readonly memory. bpf helpers
// that update its arguments can not write into it.
//
    bpf_d_path(path, active, sizeof(int));
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
