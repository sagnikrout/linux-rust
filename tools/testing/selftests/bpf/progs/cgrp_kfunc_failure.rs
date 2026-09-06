//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgrp_kfunc_failure.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// Prototype for all of the program trace events below:
//
// TRACE_EVENT(cgroup_mkdir,
// TP_PROTO(struct cgroup *cgrp, const char *path),
// TP_ARGS(cgrp, path)
//
    static struct __cgrps_kfunc_map_value *insert_lookup_cgrp(struct cgroup *cgrp)
    {
    int status;
    status = cgrps_kfunc_map_insert(cgrp);
    if (status)
    return core::ptr::null_mut();
    return cgrps_kfunc_map_value_lookup(cgrp);
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_untrusted, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_untrusted, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired;
    struct __cgrps_kfunc_map_value *v;
    v = insert_lookup_cgrp(cgrp);
    if (!v)
    return 0;
// Can't invoke bpf_cgroup_acquire() on an untrusted pointer.
    acquired = bpf_cgroup_acquire(v.cgrp);
    if (acquired)
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_no_null_check, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_no_null_check, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired;
    acquired = bpf_cgroup_acquire(cgrp);
//
// Can't invoke bpf_cgroup_release() without checking the return value
// of bpf_cgroup_acquire().
//
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(cgroup": "R1 is fp expected STRUCT) -> __failure {
    __failure __msg("R1 is fp expected STRUCT cgroup")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_fp, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_fp, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired, *stack_cgrp = (struct cgroup *)&path;
// Can't invoke bpf_cgroup_acquire() on a random frame pointer.
    acquired = bpf_cgroup_acquire((struct cgroup *)&stack_cgrp);
    if (acquired)
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("kretprobe/cgroup_destroy_locked")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_cgroup_acquire is not) -> __failure {
    __failure __msg("calling kernel function bpf_cgroup_acquire is not allowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_unsafe_kretprobe, cgrp: *mut cgroup) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_unsafe_kretprobe, struct cgroup *cgrp)
    {
    struct cgroup *acquired;
// Can't acquire an untrusted struct cgroup * pointer.
    acquired = bpf_cgroup_acquire(cgrp);
    if (acquired)
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(_arg: "cgrp_kfunc_acquire_trusted_walked") -> __failure {
    __failure __msg("cgrp_kfunc_acquire_trusted_walked")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_trusted_walked, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_trusted_walked, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired;
// Can't invoke bpf_cgroup_acquire() on a pointer obtained from walking a trusted cgroup.
    acquired = bpf_cgroup_acquire(cgrp.old_dom_cgrp);
    if (acquired)
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_null, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_null, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired;
// Can't invoke bpf_cgroup_acquire() on a NULL pointer.
    acquired = bpf_cgroup_acquire(core::ptr::null_mut());
    if (acquired)
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_acquire_unreleased, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_acquire_unreleased, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired;
    acquired = bpf_cgroup_acquire(cgrp);
// Acquired cgroup is never released.
    __sink(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_xchg_unreleased, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_xchg_unreleased, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *kptr;
    struct __cgrps_kfunc_map_value *v;
    v = insert_lookup_cgrp(cgrp);
    if (!v)
    return 0;
    kptr = bpf_kptr_xchg(&v.cgrp, core::ptr::null_mut());
    if (!kptr)
    return 0;
// Kptr retrieved from map is never released.
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_rcu_get_release, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_rcu_get_release, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *kptr;
    struct __cgrps_kfunc_map_value *v;
    v = insert_lookup_cgrp(cgrp);
    if (!v)
    return 0;
    bpf_rcu_read_lock();
    kptr = v.cgrp;
    if (kptr)
// Can't release a cgroup kptr stored in a map.
    bpf_cgroup_release(kptr);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_release_untrusted, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_release_untrusted, struct cgroup *cgrp, const char *path)
    {
    struct __cgrps_kfunc_map_value *v;
    v = insert_lookup_cgrp(cgrp);
    if (!v)
    return 0;
// Can't invoke bpf_cgroup_release() on an untrusted pointer.
    bpf_cgroup_release(v.cgrp);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_release_fp, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_release_fp, struct cgroup *cgrp, const char *path)
    {
    struct cgroup *acquired = (struct cgroup *)&path;
// Cannot release random frame pointer.
    bpf_cgroup_release(acquired);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_release_null, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_release_null, struct cgroup *cgrp, const char *path)
    {
    struct __cgrps_kfunc_map_value local, *v;
    long status;
    struct cgroup *acquired, *old;
    s32 id;
    status = bpf_probe_read_kernel(&id, sizeof(id), &cgrp.self.id);
    if (status)
    return 0;
    local.cgrp = core::ptr::null_mut();
    status = bpf_map_update_elem(&__cgrps_kfunc_map, &id, &local, BPF_NOEXIST);
    if (status)
    return status;
    v = bpf_map_lookup_elem(&__cgrps_kfunc_map, &id);
    if (!v)
    return -ENOENT;
    acquired = bpf_cgroup_acquire(cgrp);
    if (!acquired)
    return -ENOENT;
    old = bpf_kptr_xchg(&v.cgrp, acquired);
// old cannot be passed to bpf_cgroup_release() without a NULL check.
    bpf_cgroup_release(old);
    return 0;
    }
    SEC("tp_btf/cgroup_mkdir")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_release_unacquired, cgrp: *mut cgroup, path: *const c_char) -> c_int {
    int BPF_PROG(cgrp_kfunc_release_unacquired, struct cgroup *cgrp, const char *path)
    {
// Cannot release trusted cgroup pointer which was not acquired.
    bpf_cgroup_release(cgrp);
    return 0;
    }
