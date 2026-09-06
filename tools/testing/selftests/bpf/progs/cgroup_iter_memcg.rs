//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_iter_memcg.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// The latest values read are stored here.
    struct memcg_query memcg_query SEC(".data.query");
    SEC("iter.s/cgroup")
#[no_mangle]
pub unsafe extern "C" fn cgroup_memcg_query(ctx: *mut bpf_iter__cgroup) -> c_int {
    int cgroup_memcg_query(struct bpf_iter__cgroup *ctx)
    {
    struct cgroup *cgrp = ctx.cgroup;
    struct cgroup_subsys_state *css;
    struct mem_cgroup *memcg;
    if (!cgrp)
    return 1;
    css = &cgrp.self;
    memcg = bpf_get_mem_cgroup(css);
    if (!memcg)
    return 1;
    bpf_mem_cgroup_flush_stats(memcg);
    memcg_query.nr_anon_mapped = bpf_mem_cgroup_page_state(
    memcg,
    bpf_core_enum_value(enum node_stat_item, NR_ANON_MAPPED));
    memcg_query.nr_shmem = bpf_mem_cgroup_page_state(
    memcg, bpf_core_enum_value(enum node_stat_item, NR_SHMEM));
    memcg_query.nr_file_pages = bpf_mem_cgroup_page_state(
    memcg, bpf_core_enum_value(enum node_stat_item, NR_FILE_PAGES));
    memcg_query.nr_file_mapped = bpf_mem_cgroup_page_state(
    memcg,
    bpf_core_enum_value(enum node_stat_item, NR_FILE_MAPPED));
    memcg_query.pgfault = bpf_mem_cgroup_vm_events(
    memcg, bpf_core_enum_value(enum vm_event_item, PGFAULT));
    bpf_put_mem_cgroup(memcg);
    return 0;
    }
