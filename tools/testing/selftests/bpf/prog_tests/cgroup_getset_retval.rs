//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_getset_retval.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2021 Google LLC.
//

pub const SOL_CUSTOM: c_uint = 0xdeadbeef;
    static int zero;
#[no_mangle]
unsafe extern "C" fn test_setsockopt_set(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_set(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_set_eunatch = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that sets EUNATCH, assert that
// we actually get that error when we run setsockopt()
//
    link_set_eunatch = bpf_program__attach_cgroup(obj.progs.set_eunatch,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eunatch, "cg-attach-set_eunatch"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EUNATCH, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 1, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eunatch);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_set_and_get(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_set_and_get(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_set_eunatch = core::ptr::null_mut(), *link_get_retval = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that sets EUNATCH, and one that gets the
// previously set errno. Assert that we get the same errno back.
//
    link_set_eunatch = bpf_program__attach_cgroup(obj.progs.set_eunatch,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eunatch, "cg-attach-set_eunatch"))
    goto close_bpf_object;
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EUNATCH, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 2, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, -EUNATCH, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eunatch);
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_default_zero(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_default_zero(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_get_retval = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that gets the previously set errno.
// Assert that, without anything setting one, we get 0.
//
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_OK(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 1, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, 0, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_default_zero_and_set(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_default_zero_and_set(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_get_retval = core::ptr::null_mut(), *link_set_eunatch = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that gets the previously set errno, and then
// one that sets the errno to EUNATCH. Assert that the get does not
// see EUNATCH set later, and does not prevent EUNATCH from being set.
//
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    link_set_eunatch = bpf_program__attach_cgroup(obj.progs.set_eunatch,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eunatch, "cg-attach-set_eunatch"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EUNATCH, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 2, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, 0, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_get_retval);
    bpf_link__destroy(link_set_eunatch);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_override(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_override(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_set_eunatch = core::ptr::null_mut(), *link_set_eisconn = core::ptr::null_mut();
    struct bpf_link *link_get_retval = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that sets EUNATCH, then one that sets EISCONN,
// and then one that gets the exported errno. Assert both the syscall
// and the helper sees the last set errno.
//
    link_set_eunatch = bpf_program__attach_cgroup(obj.progs.set_eunatch,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eunatch, "cg-attach-set_eunatch"))
    goto close_bpf_object;
    link_set_eisconn = bpf_program__attach_cgroup(obj.progs.set_eisconn,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eisconn, "cg-attach-set_eisconn"))
    goto close_bpf_object;
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EISCONN, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 3, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, -EISCONN, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eunatch);
    bpf_link__destroy(link_set_eisconn);
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_legacy_eperm(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_legacy_eperm(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_legacy_eperm = core::ptr::null_mut(), *link_get_retval = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that return a reject without setting errno
// (legacy reject), and one that gets the errno. Assert that for
// backward compatibility the syscall result in EPERM, and this
// is also visible to the helper.
//
    link_legacy_eperm = bpf_program__attach_cgroup(obj.progs.legacy_eperm,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_legacy_eperm, "cg-attach-legacy_eperm"))
    goto close_bpf_object;
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EPERM, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 2, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, -EPERM, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_legacy_eperm);
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_setsockopt_legacy_no_override(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_setsockopt_legacy_no_override(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_setsockopt *obj;
    struct bpf_link *link_set_eunatch = core::ptr::null_mut(), *link_legacy_eperm = core::ptr::null_mut();
    struct bpf_link *link_get_retval = core::ptr::null_mut();
    obj = cgroup_getset_retval_setsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach setsockopt that sets EUNATCH, then one that return a reject
// without setting errno, and then one that gets the exported errno.
// Assert both the syscall and the helper's errno are unaffected by
// the second prog (i.e. legacy rejects does not override the errno
// to EPERM).
//
    link_set_eunatch = bpf_program__attach_cgroup(obj.progs.set_eunatch,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eunatch, "cg-attach-set_eunatch"))
    goto close_bpf_object;
    link_legacy_eperm = bpf_program__attach_cgroup(obj.progs.legacy_eperm,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_legacy_eperm, "cg-attach-legacy_eperm"))
    goto close_bpf_object;
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_ERR(setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR,
    &zero, sizeof(int)), "setsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EUNATCH, "setsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 3, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, -EUNATCH, "retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eunatch);
    bpf_link__destroy(link_legacy_eperm);
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_setsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_getsockopt_get(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_getsockopt_get(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_getsockopt *obj;
    struct bpf_link *link_get_retval = core::ptr::null_mut();
    int buf;
    let mut optlen: socklen_t = sizeof(buf);
    obj = cgroup_getset_retval_getsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach getsockopt that gets previously set errno. Assert that the
// error from kernel is in both ctx_retval_value and retval_value.
//
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_ERR(getsockopt(sock_fd, SOL_CUSTOM, 0,
    &buf, &optlen), "getsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EOPNOTSUPP, "getsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 1, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, -EOPNOTSUPP, "retval_value"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.ctx_retval_value, -EOPNOTSUPP, "ctx_retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_getsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_getsockopt_override(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_getsockopt_override(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_getsockopt *obj;
    struct bpf_link *link_set_eisconn = core::ptr::null_mut();
    int buf;
    let mut optlen: socklen_t = sizeof(buf);
    obj = cgroup_getset_retval_getsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach getsockopt that sets retval to -EISCONN. Assert that this
// overrides the value from kernel.
//
    link_set_eisconn = bpf_program__attach_cgroup(obj.progs.set_eisconn,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eisconn, "cg-attach-set_eisconn"))
    goto close_bpf_object;
    if (!ASSERT_ERR(getsockopt(sock_fd, SOL_CUSTOM, 0,
    &buf, &optlen), "getsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(errno, EISCONN, "getsockopt-errno"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 1, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eisconn);
    cgroup_getset_retval_getsockopt__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_getsockopt_retval_sync(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_getsockopt_retval_sync(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_getsockopt *obj;
    struct bpf_link *link_set_eisconn = core::ptr::null_mut(), *link_clear_retval = core::ptr::null_mut();
    struct bpf_link *link_get_retval = core::ptr::null_mut();
    int buf;
    let mut optlen: socklen_t = sizeof(buf);
    obj = cgroup_getset_retval_getsockopt__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
// Attach getsockopt that sets retval to -EISCONN, and one that clears
// ctx retval. Assert that the clearing ctx retval is synced to helper
// and clears any errors both from kernel and BPF..
//
    link_set_eisconn = bpf_program__attach_cgroup(obj.progs.set_eisconn,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_set_eisconn, "cg-attach-set_eisconn"))
    goto close_bpf_object;
    link_clear_retval = bpf_program__attach_cgroup(obj.progs.clear_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_clear_retval, "cg-attach-clear_retval"))
    goto close_bpf_object;
    link_get_retval = bpf_program__attach_cgroup(obj.progs.get_retval,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_get_retval, "cg-attach-get_retval"))
    goto close_bpf_object;
    if (!ASSERT_OK(getsockopt(sock_fd, SOL_CUSTOM, 0,
    &buf, &optlen), "getsockopt"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.invocations, 3, "invocations"))
    goto close_bpf_object;
    if (!ASSERT_FALSE(obj.bss.assertion_error, "assertion_error"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.retval_value, 0, "retval_value"))
    goto close_bpf_object;
    if (!ASSERT_EQ(obj.bss.ctx_retval_value, 0, "ctx_retval_value"))
    goto close_bpf_object;
    close_bpf_object:
    bpf_link__destroy(link_set_eisconn);
    bpf_link__destroy(link_clear_retval);
    bpf_link__destroy(link_get_retval);
    cgroup_getset_retval_getsockopt__destroy(obj);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exposed_hook {
    pub name: *const c_char,
    pub expected_err: c_int,
    } exposed_hooks[] = {

    { \
    .name = #NAME, \
    .expected_err = EXPECTED_ERR, \
    },

}

#[no_mangle]
unsafe extern "C" fn test_exposed_hooks(cgroup_fd: c_int, sock_fd: c_int) {
    static void test_exposed_hooks(int cgroup_fd, int sock_fd)
    {
    struct cgroup_getset_retval_hooks *skel;
    struct bpf_program *prog;
    int err;
    int i;
    for (i = 0; i < ARRAY_SIZE(exposed_hooks); i++) {
    skel = cgroup_getset_retval_hooks__open();
    if (!ASSERT_OK_PTR(skel, "cgroup_getset_retval_hooks__open"))
    continue;
    prog = bpf_object__find_program_by_name(skel.obj, exposed_hooks[i].name);
    if (!ASSERT_NEQ(prog, core::ptr::null_mut(), "bpf_object__find_program_by_name"))
    goto close_skel;
    err = bpf_program__set_autoload(prog, true);
    if (!ASSERT_OK(err, "bpf_program__set_autoload"))
    goto close_skel;
    err = cgroup_getset_retval_hooks__load(skel);
    ASSERT_EQ(err, exposed_hooks[i].expected_err, "expected_err");
    close_skel:
    cgroup_getset_retval_hooks__destroy(skel);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_getset_retval() {
    void test_cgroup_getset_retval(void)
    {
    let mut cgroup_fd: c_int = -1;
    let mut sock_fd: c_int = -1;
    cgroup_fd = test__join_cgroup("/cgroup_getset_retval");
    if (!ASSERT_GE(cgroup_fd, 0, "cg-create"))
    goto close_fd;
    sock_fd = start_server(AF_INET, SOCK_DGRAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(sock_fd, 0, "start-server"))
    goto close_fd;
    if (test__start_subtest("setsockopt-set"))
    test_setsockopt_set(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-set_and_get"))
    test_setsockopt_set_and_get(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-default_zero"))
    test_setsockopt_default_zero(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-default_zero_and_set"))
    test_setsockopt_default_zero_and_set(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-override"))
    test_setsockopt_override(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-legacy_eperm"))
    test_setsockopt_legacy_eperm(cgroup_fd, sock_fd);
    if (test__start_subtest("setsockopt-legacy_no_override"))
    test_setsockopt_legacy_no_override(cgroup_fd, sock_fd);
    if (test__start_subtest("getsockopt-get"))
    test_getsockopt_get(cgroup_fd, sock_fd);
    if (test__start_subtest("getsockopt-override"))
    test_getsockopt_override(cgroup_fd, sock_fd);
    if (test__start_subtest("getsockopt-retval_sync"))
    test_getsockopt_retval_sync(cgroup_fd, sock_fd);
    if (test__start_subtest("exposed_hooks"))
    test_exposed_hooks(cgroup_fd, sock_fd);
    close_fd:
    close(cgroup_fd);
    }
