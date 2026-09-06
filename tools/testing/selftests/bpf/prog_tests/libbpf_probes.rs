//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/libbpf_probes.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_libbpf_probe_prog_types() {
    void test_libbpf_probe_prog_types(void)
    {
    struct btf *btf;
    const struct btf_type *t;
    const struct btf_enum *e;
    int i, n, id;
    btf = btf__parse("/sys/kernel/btf/vmlinux", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(btf, "btf_parse"))
    return;
// find enum bpf_prog_type and enumerate each value
    id = btf__find_by_name_kind(btf, "bpf_prog_type", BTF_KIND_ENUM);
    if (!ASSERT_GT(id, 0, "bpf_prog_type_id"))
    goto cleanup;
    t = btf__type_by_id(btf, id);
    if (!ASSERT_OK_PTR(t, "bpf_prog_type_enum"))
    goto cleanup;
    for (e = btf_enum(t), i = 0, n = btf_vlen(t); i < n; e++, i++) {
    const char *prog_type_name = btf__str_by_offset(btf, e.name_off);
    let mut prog_type: enum bpf_prog_type = (enum bpf_prog_type)e.val;
    int res;
    if (prog_type == BPF_PROG_TYPE_UNSPEC)
    continue;
    if (strcmp(prog_type_name, "__MAX_BPF_PROG_TYPE") == 0)
    continue;
    if (!test__start_subtest(prog_type_name))
    continue;
    res = libbpf_probe_bpf_prog_type(prog_type, core::ptr::null_mut());
    ASSERT_EQ(res, 1, prog_type_name);
    }
    cleanup:
    btf__free(btf);
    }
#[no_mangle]
pub unsafe extern "C" fn test_libbpf_probe_map_types() {
    void test_libbpf_probe_map_types(void)
    {
    struct btf *btf;
    const struct btf_type *t;
    const struct btf_enum *e;
    int i, n, id;
    btf = btf__parse("/sys/kernel/btf/vmlinux", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(btf, "btf_parse"))
    return;
// find enum bpf_map_type and enumerate each value
    id = btf__find_by_name_kind(btf, "bpf_map_type", BTF_KIND_ENUM);
    if (!ASSERT_GT(id, 0, "bpf_map_type_id"))
    goto cleanup;
    t = btf__type_by_id(btf, id);
    if (!ASSERT_OK_PTR(t, "bpf_map_type_enum"))
    goto cleanup;
    for (e = btf_enum(t), i = 0, n = btf_vlen(t); i < n; e++, i++) {
    const char *map_type_name = btf__str_by_offset(btf, e.name_off);
    let mut map_type: enum bpf_map_type = (enum bpf_map_type)e.val;
    int res;
    if (map_type == BPF_MAP_TYPE_UNSPEC)
    continue;
    if (strcmp(map_type_name, "__MAX_BPF_MAP_TYPE") == 0)
    continue;
    if (!test__start_subtest(map_type_name))
    continue;
    res = libbpf_probe_bpf_map_type(map_type, core::ptr::null_mut());
    ASSERT_EQ(res, 1, map_type_name);
    }
    cleanup:
    btf__free(btf);
    }
#[no_mangle]
pub unsafe extern "C" fn test_libbpf_probe_helpers() {
    void test_libbpf_probe_helpers(void)
    {

    .prog_type_name = "BPF_PROG_TYPE_" # prog,	\
    .helper_name = "bpf_" # helper,			\
    .prog_type = BPF_PROG_TYPE_ ## prog,		\
    .helper_id = BPF_FUNC_ ## helper,		\
    .supported = supp,				\
    }
    const struct case_def {
    const char *prog_type_name;
    const char *helper_name;
    enum bpf_prog_type prog_type;
    enum bpf_func_id helper_id;
    bool supported;
    } cases[] = {
    CASE(KPROBE, unspec, false),
    CASE(KPROBE, map_lookup_elem, true),
    CASE(KPROBE, loop, true),
    CASE(KPROBE, ktime_get_coarse_ns, false),
    CASE(SOCKET_FILTER, ktime_get_coarse_ns, true),
    CASE(KPROBE, sys_bpf, false),
    CASE(SYSCALL, sys_bpf, true),
    };
    let mut case_cnt: usize = ARRAY_SIZE(cases), i;
    char buf[128];
    for (i = 0; i < case_cnt; i++) {
    const struct case_def *d = &cases[i];
    int res;
    snprintf(buf, sizeof(buf), "%s+%s", d.prog_type_name, d.helper_name);
    if (!test__start_subtest(buf))
    continue;
    res = libbpf_probe_bpf_helper(d.prog_type, d.helper_id, core::ptr::null_mut());
    ASSERT_EQ(res, d.supported, buf);
    }
    }
