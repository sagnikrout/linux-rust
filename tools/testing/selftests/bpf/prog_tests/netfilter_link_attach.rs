//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/netfilter_link_attach.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_link_test {
    pub pf: __u32,
    pub hooknum: __u32,
    pub priority: __s32,
    pub flags: __u32,
    pub expect_success: bool,
    pub name: *const *const c_char,
}

    static const struct nf_link_test nf_hook_link_tests[] = {
    { .name = "allzero", },
    { .pf = NFPROTO_NUMPROTO, .name = "invalid-pf", },
    { .pf = NFPROTO_IPV4, .hooknum = 42, .name = "invalid-hooknum", },
    { .pf = NFPROTO_IPV4, .priority = INT_MIN, .name = "invalid-priority-min", },
    { .pf = NFPROTO_IPV4, .priority = INT_MAX, .name = "invalid-priority-max", },
    { .pf = NFPROTO_IPV4, .flags = UINT_MAX, .name = "invalid-flags", },
    { .pf = NFPROTO_INET, .priority = 1, .name = "invalid-inet-not-supported", },
    {
    .pf = NFPROTO_IPV4,
    .hooknum = NF_INET_POST_ROUTING,
    .priority = -10000,
    .flags = 0,
    .expect_success = true,
    .name = "attach ipv4",
    },
    {
    .pf = NFPROTO_IPV6,
    .hooknum = NF_INET_FORWARD,
    .priority =  10001,
    .flags = BPF_F_NETFILTER_IP_DEFRAG,
    .expect_success = true,
    .name = "attach ipv6",
    },
    };
#[no_mangle]
unsafe extern "C" fn verify_netfilter_link_info(link: *mut bpf_link, nf_expected: nf_link_test) {
    static void verify_netfilter_link_info(struct bpf_link *link, const struct nf_link_test nf_expected)
    {
    struct bpf_link_info info;
    let mut len: __u32 = sizeof(info);
    int err, fd;
    memset(&info, 0, len);
    fd = bpf_link__fd(link);
    err = bpf_link_get_info_by_fd(fd, &info, &len);
    ASSERT_OK(err, "get_link_info");
    ASSERT_EQ(info.type, BPF_LINK_TYPE_NETFILTER, "info link type");
    ASSERT_EQ(info.netfilter.pf, nf_expected.pf, "info nf protocol family");
    ASSERT_EQ(info.netfilter.hooknum, nf_expected.hooknum, "info nf hooknum");
    ASSERT_EQ(info.netfilter.priority, nf_expected.priority, "info nf priority");
    ASSERT_EQ(info.netfilter.flags, nf_expected.flags, "info nf flags");
    }
#[no_mangle]
pub unsafe extern "C" fn test_netfilter_link_attach() {
    void test_netfilter_link_attach(void)
    {
    struct test_netfilter_link_attach *skel;
    struct bpf_program *prog;
    LIBBPF_OPTS(bpf_netfilter_opts, opts);
    int i;
    skel = test_netfilter_link_attach__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_netfilter_link_attach__open_and_load"))
    goto out;
    prog = skel.progs.nf_link_attach_test;
    if (!ASSERT_OK_PTR(prog, "attach program"))
    goto out;
    for (i = 0; i < ARRAY_SIZE(nf_hook_link_tests); i++) {
    struct bpf_link *link;
    if (!test__start_subtest(nf_hook_link_tests[i].name))
    continue;

    X(opts, pf, i);
    X(opts, hooknum, i);
    X(opts, priority, i);
    X(opts, flags, i);

    link = bpf_program__attach_netfilter(prog, &opts);
    if (nf_hook_link_tests[i].expect_success) {
    struct bpf_link *link2;
    if (!ASSERT_OK_PTR(link, "program attach successful"))
    continue;
    verify_netfilter_link_info(link, nf_hook_link_tests[i]);
    link2 = bpf_program__attach_netfilter(prog, &opts);
    ASSERT_ERR_PTR(link2, "attach program with same pf/hook/priority");
    if (!ASSERT_OK(bpf_link__destroy(link), "link destroy"))
    break;
    link2 = bpf_program__attach_netfilter(prog, &opts);
    if (!ASSERT_OK_PTR(link2, "program reattach successful"))
    continue;
    verify_netfilter_link_info(link2, nf_hook_link_tests[i]);
    if (!ASSERT_OK(bpf_link__destroy(link2), "link destroy"))
    break;
    } else {
    ASSERT_ERR_PTR(link, "program load failure");
    }
    }
    out:
    test_netfilter_link_attach__destroy(skel);
    }
