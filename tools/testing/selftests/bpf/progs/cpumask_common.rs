//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/cpumask_common.h
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

// Should use BTF_FIELDS_MAX, but it is not always available in vmlinux.h,
// so use the hard-coded number as a workaround.
//
pub const CPUMASK_KPTR_FIELDS_MAX: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __cpumask_map_value {
    pub cpumask: *mut *mut bpf_cpumask __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub __cpumask_map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } __cpumask_map,
    pub __weak: *mut *mut bpf_cpumask bpf_cpumask_create(void) __ksym,
    pub __weak: *mut *mut void bpf_cpumask_release(struct bpf_cpumask cpumask) __ksym,
    pub __weak: *mut *mut *mut bpf_cpumask bpf_cpumask_acquire(bpf_cpumask cpumask) __ksym,
    pub __weak: *const *const u32 bpf_cpumask_first(struct cpumask cpumask) __ksym,
    pub __weak: *const *const u32 bpf_cpumask_first_zero(struct cpumask cpumask) __ksym,
    pub __weak: *const *const cpumask src2) __ksym,
    pub __weak: *mut *mut void bpf_cpumask_set_cpu(u32 cpu, struct bpf_cpumask cpumask) __ksym,
    pub __weak: *mut *mut void bpf_cpumask_clear_cpu(u32 cpu, struct bpf_cpumask cpumask) __ksym,
    pub __weak: *const *const bool bpf_cpumask_test_cpu(u32 cpu, struct cpumask cpumask) __ksym,
    pub __weak: *mut *mut bool bpf_cpumask_test_and_set_cpu(u32 cpu, struct bpf_cpumask cpumask) __ksym,
    pub __weak: *mut *mut bool bpf_cpumask_test_and_clear_cpu(u32 cpu, struct bpf_cpumask cpumask) __ksym,
    pub __weak: *mut *mut void bpf_cpumask_setall(struct bpf_cpumask cpumask) __ksym,
    pub __weak: *mut *mut void bpf_cpumask_clear(struct bpf_cpumask cpumask) __ksym,
    pub __weak: *const *const cpumask src2) __ksym,
    pub __weak: *const *const cpumask src2) __ksym,
    pub __weak: *const *const cpumask src2) __ksym,
    pub __weak: *const *const *const bool bpf_cpumask_equal(struct cpumask src1, struct cpumask src2) __ksym,
    pub __weak: *const *const *const bool bpf_cpumask_intersects(struct cpumask src1, struct cpumask src2) __ksym,
    pub __weak: *const *const *const bool bpf_cpumask_subset(struct cpumask src1, struct cpumask src2) __ksym,
    pub __weak: *const *const bool bpf_cpumask_empty(struct cpumask cpumask) __ksym,
    pub __weak: *const *const bool bpf_cpumask_full(struct cpumask cpumask) __ksym,
    pub __weak: *const *const *const void bpf_cpumask_copy(struct bpf_cpumask dst, struct cpumask src) __ksym,
    pub __weak: *const *const u32 bpf_cpumask_any_distribute(struct cpumask src) __ksym,
    pub __weak: *const *const cpumask src2) __ksym,
    pub __weak: *const *const u32 bpf_cpumask_weight(struct cpumask cpumask) __ksym,
    pub __weak: *mut *mut *mut int bpf_cpumask_populate(struct bpf_cpumask cpumask, void src, size_t src__sz) __ksym,
    pub __weak: void bpf_rcu_read_lock(void) __ksym,
    pub __weak: void bpf_rcu_read_unlock(void) __ksym,
    pub )cpumask: *const return (struct cpumask,
    pub cpumask: *mut bpf_cpumask,
    pub bpf_cpumask_create(): cpumask =,
    pub 1: err =,
    pub NULL: return,
    pub 2: err =,
    pub NULL: return,
    pub cpumask: return,
    pub 0: u32 key =,
    pub &key): return bpf_map_lookup_elem(&__cpumask_map,,
    pub v: *mut __cpumask_map_value local,,
    pub status: c_long,
    pub old: *mut bpf_cpumask,
    pub 0: u32 key =,
    pub NULL: local.cpumask =,
    pub 0): status = bpf_map_update_elem(&__cpumask_map, &key, &local,,
    pub status: return,
    pub &key): v = bpf_map_lookup_elem(&__cpumask_map,,
    pub -ENOENT: return,
    pub mask): old = bpf_kptr_xchg(&v->cpumask,,
    pub -EEXIST: return,
    pub 0: return,
