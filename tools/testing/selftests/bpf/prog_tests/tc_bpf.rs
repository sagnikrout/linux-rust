//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tc_bpf.c
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

pub const LO_IFINDEX: c_int = 1;

    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_h, .handle = 1);                                     \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_p, .priority = 1);                                   \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_f, .prog_fd = __fd);                                 \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hp, .handle = 1, .priority = 1);                     \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hf, .handle = 1, .prog_fd = __fd);                   \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_pf, .priority = 1, .prog_fd = __fd);                 \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hpf, .handle = 1, .priority = 1, .prog_fd = __fd);   \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hpi, .handle = 1, .priority = 1, .prog_id = 42);     \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hpr, .handle = 1, .priority = 1,                     \
    .flags = BPF_TC_F_REPLACE);                                            \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_hpfi, .handle = 1, .priority = 1, .prog_fd = __fd,   \
    .prog_id = 42);                                                        \
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts_prio_max, .handle = 1, .priority = UINT16_MAX + 1);
#[no_mangle]
unsafe extern "C" fn test_tc_bpf_basic(hook: *const bpf_tc_hook, fd: c_int) -> c_int {
    static int test_tc_bpf_basic(const struct bpf_tc_hook *hook, int fd)
    {
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts, .handle = 1, .priority = 1, .prog_fd = fd);
    let mut info: bpf_prog_info = {};
    let mut info_len: __u32 = sizeof(info);
    int ret;
    ret = bpf_prog_get_info_by_fd(fd, &info, &info_len);
    if (!ASSERT_OK(ret, "bpf_prog_get_info_by_fd"))
    return ret;
    ret = bpf_tc_attach(hook, &opts);
    if (!ASSERT_OK(ret, "bpf_tc_attach"))
    return ret;
    if (!ASSERT_EQ(opts.handle, 1, "handle set") ||
    !ASSERT_EQ(opts.priority, 1, "priority set") ||
    !ASSERT_EQ(opts.prog_id, info.id, "prog_id set"))
    goto end;
    opts.prog_id = 0;
    opts.flags = BPF_TC_F_REPLACE;
    ret = bpf_tc_attach(hook, &opts);
    if (!ASSERT_OK(ret, "bpf_tc_attach replace mode"))
    goto end;
    opts.flags = opts.prog_fd = opts.prog_id = 0;
    ret = bpf_tc_query(hook, &opts);
    if (!ASSERT_OK(ret, "bpf_tc_query"))
    goto end;
    if (!ASSERT_EQ(opts.handle, 1, "handle set") ||
    !ASSERT_EQ(opts.priority, 1, "priority set") ||
    !ASSERT_EQ(opts.prog_id, info.id, "prog_id set"))
    goto end;
    end:
    opts.flags = opts.prog_fd = opts.prog_id = 0;
    ret = bpf_tc_detach(hook, &opts);
    ASSERT_OK(ret, "bpf_tc_detach");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_tc_bpf_api(hook: *mut bpf_tc_hook, fd: c_int) -> c_int {
    static int test_tc_bpf_api(struct bpf_tc_hook *hook, int fd)
    {
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, attach_opts, .handle = 1, .priority = 1, .prog_fd = fd);
    DECLARE_LIBBPF_OPTS(bpf_tc_hook, inv_hook, .attach_point = BPF_TC_INGRESS);
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts, .handle = 1, .priority = 1);
    int ret;
    ret = bpf_tc_hook_create(core::ptr::null_mut());
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_create invalid hook = core::ptr::null_mut()"))
    return -EINVAL;
// hook ifindex = 0
    ret = bpf_tc_hook_create(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_create invalid hook ifindex == 0"))
    return -EINVAL;
    ret = bpf_tc_hook_destroy(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_destroy invalid hook ifindex == 0"))
    return -EINVAL;
    ret = bpf_tc_attach(&inv_hook, &attach_opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook ifindex == 0"))
    return -EINVAL;
    attach_opts.prog_id = 0;
    ret = bpf_tc_detach(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook ifindex == 0"))
    return -EINVAL;
    ret = bpf_tc_query(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook ifindex == 0"))
    return -EINVAL;
// hook ifindex < 0
    inv_hook.ifindex = -1;
    ret = bpf_tc_hook_create(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_create invalid hook ifindex < 0"))
    return -EINVAL;
    ret = bpf_tc_hook_destroy(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_destroy invalid hook ifindex < 0"))
    return -EINVAL;
    ret = bpf_tc_attach(&inv_hook, &attach_opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook ifindex < 0"))
    return -EINVAL;
    attach_opts.prog_id = 0;
    ret = bpf_tc_detach(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook ifindex < 0"))
    return -EINVAL;
    ret = bpf_tc_query(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook ifindex < 0"))
    return -EINVAL;
    inv_hook.ifindex = LO_IFINDEX;
// hook.attach_point invalid
    inv_hook.attach_point = 0xabcd;
    ret = bpf_tc_hook_create(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_create invalid hook.attach_point"))
    return -EINVAL;
    ret = bpf_tc_hook_destroy(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_destroy invalid hook.attach_point"))
    return -EINVAL;
    ret = bpf_tc_attach(&inv_hook, &attach_opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook.attach_point"))
    return -EINVAL;
    ret = bpf_tc_detach(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook.attach_point"))
    return -EINVAL;
    ret = bpf_tc_query(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook.attach_point"))
    return -EINVAL;
    inv_hook.attach_point = BPF_TC_INGRESS;
// hook.attach_point valid, but parent invalid
    inv_hook.parent = TC_H_MAKE(1UL << 16, 10);
    ret = bpf_tc_hook_create(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_create invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_hook_destroy(&inv_hook);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_hook_destroy invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_attach(&inv_hook, &attach_opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_detach(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_query(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook parent"))
    return -EINVAL;
    inv_hook.attach_point = BPF_TC_CUSTOM;
    inv_hook.parent = 0;
// These return EOPNOTSUPP instead of EINVAL as parent is checked after
// attach_point of the hook.
//
    ret = bpf_tc_hook_create(&inv_hook);
    if (!ASSERT_EQ(ret, -EOPNOTSUPP, "bpf_tc_hook_create invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_hook_destroy(&inv_hook);
    if (!ASSERT_EQ(ret, -EOPNOTSUPP, "bpf_tc_hook_destroy invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_attach(&inv_hook, &attach_opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_detach(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook parent"))
    return -EINVAL;
    ret = bpf_tc_query(&inv_hook, &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook parent"))
    return -EINVAL;
    inv_hook.attach_point = BPF_TC_INGRESS;
// detach
    {
    TEST_DECLARE_OPTS(fd);
    ret = bpf_tc_detach(core::ptr::null_mut(), &opts_hp);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid hook = core::ptr::null_mut()"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, core::ptr::null_mut());
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid opts = core::ptr::null_mut()"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_hpr);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid flags set"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_hpf);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid prog_fd set"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_hpi);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid prog_id set"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_p);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid handle unset"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_h);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid priority unset"))
    return -EINVAL;
    ret = bpf_tc_detach(hook, &opts_prio_max);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_detach invalid priority > UINT16_MAX"))
    return -EINVAL;
    }
// query
    {
    TEST_DECLARE_OPTS(fd);
    ret = bpf_tc_query(core::ptr::null_mut(), &opts);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid hook = core::ptr::null_mut()"))
    return -EINVAL;
    ret = bpf_tc_query(hook, core::ptr::null_mut());
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid opts = core::ptr::null_mut()"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_hpr);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid flags set"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_hpf);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid prog_fd set"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_hpi);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid prog_id set"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_p);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid handle unset"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_h);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid priority unset"))
    return -EINVAL;
    ret = bpf_tc_query(hook, &opts_prio_max);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query invalid priority > UINT16_MAX"))
    return -EINVAL;
// when chain is not present, kernel returns -EINVAL
    ret = bpf_tc_query(hook, &opts_hp);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_query valid handle, priority set"))
    return -EINVAL;
    }
// attach
    {
    TEST_DECLARE_OPTS(fd);
    ret = bpf_tc_attach(core::ptr::null_mut(), &opts_hp);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid hook = core::ptr::null_mut()"))
    return -EINVAL;
    ret = bpf_tc_attach(hook, core::ptr::null_mut());
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid opts = core::ptr::null_mut()"))
    return -EINVAL;
    opts_hp.flags = 42;
    ret = bpf_tc_attach(hook, &opts_hp);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid flags"))
    return -EINVAL;
    ret = bpf_tc_attach(hook, core::ptr::null_mut());
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid prog_fd unset"))
    return -EINVAL;
    ret = bpf_tc_attach(hook, &opts_hpi);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid prog_id set"))
    return -EINVAL;
    ret = bpf_tc_attach(hook, &opts_pf);
    if (!ASSERT_OK(ret, "bpf_tc_attach valid handle unset"))
    return -EINVAL;
    opts_pf.prog_fd = opts_pf.prog_id = 0;
    ASSERT_OK(bpf_tc_detach(hook, &opts_pf), "bpf_tc_detach");
    ret = bpf_tc_attach(hook, &opts_hf);
    if (!ASSERT_OK(ret, "bpf_tc_attach valid priority unset"))
    return -EINVAL;
    opts_hf.prog_fd = opts_hf.prog_id = 0;
    ASSERT_OK(bpf_tc_detach(hook, &opts_hf), "bpf_tc_detach");
    ret = bpf_tc_attach(hook, &opts_prio_max);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_tc_attach invalid priority > UINT16_MAX"))
    return -EINVAL;
    ret = bpf_tc_attach(hook, &opts_f);
    if (!ASSERT_OK(ret, "bpf_tc_attach valid both handle and priority unset"))
    return -EINVAL;
    opts_f.prog_fd = opts_f.prog_id = 0;
    ASSERT_OK(bpf_tc_detach(hook, &opts_f), "bpf_tc_detach");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tc_bpf_root() {
    void tc_bpf_root(void)
    {
    DECLARE_LIBBPF_OPTS(bpf_tc_hook, hook, .ifindex = LO_IFINDEX,
    .attach_point = BPF_TC_INGRESS);
    struct test_tc_bpf *skel = core::ptr::null_mut();
    let mut hook_created: bool = false;
    int cls_fd, ret;
    skel = test_tc_bpf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_tc_bpf__open_and_load"))
    return;
    cls_fd = bpf_program__fd(skel.progs.cls);
    ret = bpf_tc_hook_create(&hook);
    if (ret == 0)
    hook_created = true;
    ret = ret == -EEXIST ? 0 : ret;
    if (!ASSERT_OK(ret, "bpf_tc_hook_create(BPF_TC_INGRESS)"))
    goto end;
    hook.attach_point = BPF_TC_CUSTOM;
    hook.parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    ret = bpf_tc_hook_create(&hook);
    if (!ASSERT_EQ(ret, -EOPNOTSUPP, "bpf_tc_hook_create invalid hook.attach_point"))
    goto end;
    ret = test_tc_bpf_basic(&hook, cls_fd);
    if (!ASSERT_OK(ret, "test_tc_internal ingress"))
    goto end;
    ret = bpf_tc_hook_destroy(&hook);
    if (!ASSERT_EQ(ret, -EOPNOTSUPP, "bpf_tc_hook_destroy invalid hook.attach_point"))
    goto end;
    hook.attach_point = BPF_TC_INGRESS;
    hook.parent = 0;
    bpf_tc_hook_destroy(&hook);
    ret = test_tc_bpf_basic(&hook, cls_fd);
    if (!ASSERT_OK(ret, "test_tc_internal ingress"))
    goto end;
    bpf_tc_hook_destroy(&hook);
    hook.attach_point = BPF_TC_EGRESS;
    ret = test_tc_bpf_basic(&hook, cls_fd);
    if (!ASSERT_OK(ret, "test_tc_internal egress"))
    goto end;
    bpf_tc_hook_destroy(&hook);
    ret = test_tc_bpf_api(&hook, cls_fd);
    if (!ASSERT_OK(ret, "test_tc_bpf_api"))
    goto end;
    bpf_tc_hook_destroy(&hook);
    end:
    if (hook_created) {
    hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
    bpf_tc_hook_destroy(&hook);
    }
    test_tc_bpf__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn tc_bpf_non_root() {
    void tc_bpf_non_root(void)
    {
    struct test_tc_bpf *skel = core::ptr::null_mut();
    let mut caps: __u64 = 0;
    int ret;
// In case CAP_BPF and CAP_PERFMON is not set
    ret = cap_enable_effective(1ULL << CAP_BPF | 1ULL << CAP_NET_ADMIN, &caps);
    if (!ASSERT_OK(ret, "set_cap_bpf_cap_net_admin"))
    return;
    ret = cap_disable_effective(1ULL << CAP_SYS_ADMIN | 1ULL << CAP_PERFMON, core::ptr::null_mut());
    if (!ASSERT_OK(ret, "disable_cap_sys_admin"))
    goto restore_cap;
    skel = test_tc_bpf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_tc_bpf__open_and_load"))
    goto restore_cap;
    test_tc_bpf__destroy(skel);
    restore_cap:
    if (caps)
    cap_enable_effective(caps, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn test_tc_bpf() {
    void test_tc_bpf(void)
    {
    if (test__start_subtest("tc_bpf_root"))
    tc_bpf_root();
    if (test__start_subtest("tc_bpf_non_root"))
    tc_bpf_non_root();
    }
