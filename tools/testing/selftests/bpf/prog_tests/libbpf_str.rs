//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/libbpf_str.c
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

//
// Utility function uppercasing an entire string.
//
#[no_mangle]
unsafe extern "C" fn uppercase(s: *mut c_char) {
    static void uppercase(char *s)
    {
    for (; *s != '\0'; s++)
// s = toupper(*s);
    }
//
// Test case to check that all bpf_attach_type variants are covered by
// libbpf_bpf_attach_type_str.
//
#[no_mangle]
unsafe extern "C" fn test_libbpf_bpf_attach_type_str() {
    static void test_libbpf_bpf_attach_type_str(void)
    {
    struct btf *btf;
    const struct btf_type *t;
    const struct btf_enum *e;
    int i, n, id;
    btf = btf__parse("/sys/kernel/btf/vmlinux", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(btf, "btf_parse"))
    return;
// find enum bpf_attach_type and enumerate each value
    id = btf__find_by_name_kind(btf, "bpf_attach_type", BTF_KIND_ENUM);
    if (!ASSERT_GT(id, 0, "bpf_attach_type_id"))
    goto cleanup;
    t = btf__type_by_id(btf, id);
    e = btf_enum(t);
    n = btf_vlen(t);
    for (i = 0; i < n; e++, i++) {
    let mut attach_type: enum bpf_attach_type = (enum bpf_attach_type)e.val;
    const char *attach_type_name;
    const char *attach_type_str;
    char buf[256];
    if (attach_type == __MAX_BPF_ATTACH_TYPE)
    continue;
    attach_type_name = btf__str_by_offset(btf, e.name_off);
    attach_type_str = libbpf_bpf_attach_type_str(attach_type);
    ASSERT_OK_PTR(attach_type_str, attach_type_name);
    snprintf(buf, sizeof(buf), "BPF_%s", attach_type_str);
    uppercase(buf);
    ASSERT_STREQ(buf, attach_type_name, "exp_str_value");
    }
    cleanup:
    btf__free(btf);
    }
//
// Test case to check that all bpf_link_type variants are covered by
// libbpf_bpf_link_type_str.
//
#[no_mangle]
unsafe extern "C" fn test_libbpf_bpf_link_type_str() {
    static void test_libbpf_bpf_link_type_str(void)
    {
    struct btf *btf;
    const struct btf_type *t;
    const struct btf_enum *e;
    int i, n, id;
    btf = btf__parse("/sys/kernel/btf/vmlinux", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(btf, "btf_parse"))
    return;
// find enum bpf_link_type and enumerate each value
    id = btf__find_by_name_kind(btf, "bpf_link_type", BTF_KIND_ENUM);
    if (!ASSERT_GT(id, 0, "bpf_link_type_id"))
    goto cleanup;
    t = btf__type_by_id(btf, id);
    e = btf_enum(t);
    n = btf_vlen(t);
    for (i = 0; i < n; e++, i++) {
    let mut link_type: enum bpf_link_type = (enum bpf_link_type)e.val;
    const char *link_type_name;
    const char *link_type_str;
    char buf[256];
    if (link_type == __MAX_BPF_LINK_TYPE)
    continue;
    link_type_name = btf__str_by_offset(btf, e.name_off);
    link_type_str = libbpf_bpf_link_type_str(link_type);
    ASSERT_OK_PTR(link_type_str, link_type_name);
    snprintf(buf, sizeof(buf), "BPF_LINK_TYPE_%s", link_type_str);
    uppercase(buf);
    ASSERT_STREQ(buf, link_type_name, "exp_str_value");
    }
    cleanup:
    btf__free(btf);
    }
//
// Test case to check that all bpf_map_type variants are covered by
// libbpf_bpf_map_type_str.
//
#[no_mangle]
unsafe extern "C" fn test_libbpf_bpf_map_type_str() {
    static void test_libbpf_bpf_map_type_str(void)
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
    e = btf_enum(t);
    n = btf_vlen(t);
    for (i = 0; i < n; e++, i++) {
    let mut map_type: enum bpf_map_type = (enum bpf_map_type)e.val;
    const char *map_type_name;
    const char *map_type_str;
    char buf[256];
    if (map_type == __MAX_BPF_MAP_TYPE)
    continue;
    map_type_name = btf__str_by_offset(btf, e.name_off);
    map_type_str = libbpf_bpf_map_type_str(map_type);
    ASSERT_OK_PTR(map_type_str, map_type_name);
    snprintf(buf, sizeof(buf), "BPF_MAP_TYPE_%s", map_type_str);
    uppercase(buf);
// Special case for map_type_name BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED
// where it and BPF_MAP_TYPE_CGROUP_STORAGE have the same enum value
// (map_type). For this enum value, libbpf_bpf_map_type_str() picks
// BPF_MAP_TYPE_CGROUP_STORAGE. The same for
// BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED and
// BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE.
//
    if (strcmp(map_type_name, "BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED") == 0)
    continue;
    if (strcmp(map_type_name, "BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED") == 0)
    continue;
    ASSERT_STREQ(buf, map_type_name, "exp_str_value");
    }
    cleanup:
    btf__free(btf);
    }
//
// Test case to check that all bpf_prog_type variants are covered by
// libbpf_bpf_prog_type_str.
//
#[no_mangle]
unsafe extern "C" fn test_libbpf_bpf_prog_type_str() {
    static void test_libbpf_bpf_prog_type_str(void)
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
    e = btf_enum(t);
    n = btf_vlen(t);
    for (i = 0; i < n; e++, i++) {
    let mut prog_type: enum bpf_prog_type = (enum bpf_prog_type)e.val;
    const char *prog_type_name;
    const char *prog_type_str;
    char buf[256];
    if (prog_type == __MAX_BPF_PROG_TYPE)
    continue;
    prog_type_name = btf__str_by_offset(btf, e.name_off);
    prog_type_str = libbpf_bpf_prog_type_str(prog_type);
    ASSERT_OK_PTR(prog_type_str, prog_type_name);
    snprintf(buf, sizeof(buf), "BPF_PROG_TYPE_%s", prog_type_str);
    uppercase(buf);
    ASSERT_STREQ(buf, prog_type_name, "exp_str_value");
    }
    cleanup:
    btf__free(btf);
    }
//
// Run all libbpf str conversion tests.
//
#[no_mangle]
pub unsafe extern "C" fn test_libbpf_str() {
    void test_libbpf_str(void)
    {
    if (test__start_subtest("bpf_attach_type_str"))
    test_libbpf_bpf_attach_type_str();
    if (test__start_subtest("bpf_link_type_str"))
    test_libbpf_bpf_link_type_str();
    if (test__start_subtest("bpf_map_type_str"))
    test_libbpf_bpf_map_type_str();
    if (test__start_subtest("bpf_prog_type_str"))
    test_libbpf_bpf_prog_type_str();
    }
