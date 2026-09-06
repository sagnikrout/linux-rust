//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_type_tag_percpu.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_btf_type_tag_1 {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_btf_type_tag_2 {
    pub p: *mut bpf_testmod_btf_type_tag_1,
}

    __u64 g;
    SEC("fentry/bpf_testmod_test_btf_type_tag_percpu_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_percpu1, arg: *mut bpf_testmod_btf_type_tag_1) -> c_int {
    int BPF_PROG(test_percpu1, struct bpf_testmod_btf_type_tag_1 *arg)
    {
    g = arg.a;
    return 0;
    }
    SEC("fentry/bpf_testmod_test_btf_type_tag_percpu_2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_percpu2, arg: *mut bpf_testmod_btf_type_tag_2) -> c_int {
    int BPF_PROG(test_percpu2, struct bpf_testmod_btf_type_tag_2 *arg)
    {
    g = arg.p.a;
    return 0;
    }
// trace_cgroup_mkdir(struct cgroup *cgrp, const char *path)
//
// struct css_rstat_cpu {
// ...
// struct cgroup_subsys_state *updated_children;
// ...
// };
//
// struct cgroup_subsys_state {
// ...
// struct css_rstat_cpu __percpu *rstat_cpu;
// ...
// };
//
// struct cgroup {
// struct cgroup_subsys_state self;
// ...
// };
//
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_percpu_load, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(test_percpu_load, struct cgroup *cgrp, const char *path)
    {
    g = (__u64)cgrp.self.rstat_cpu.updated_children;
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_percpu_helper, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(test_percpu_helper, struct cgroup *cgrp, const char *path)
    {
    struct css_rstat_cpu *rstat;
    __u32 cpu;
    cpu = bpf_get_smp_processor_id();
    rstat = (struct css_rstat_cpu *)bpf_per_cpu_ptr(
    cgrp.self.rstat_cpu, cpu);
    if (rstat) {
// READ_ONCE
// (volatile long *)rstat;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
