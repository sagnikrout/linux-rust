//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/rt-link.c
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

    static void rt_link_print(struct __test_metadata *_metadata,
    struct rt_link_getlink_rsp *r)
    {
    unsigned int i;
    EXPECT_TRUE((bool)r._hdr.ifi_index);
    ksft_print_msg("%3d: ", r._hdr.ifi_index);
    EXPECT_TRUE((bool)r._len.ifname);
    if (r._len.ifname)
    printf("%6s: ", r.ifname);
    if (r._present.mtu)
    printf("mtu %5d  ", r.mtu);
    if (r.linkinfo._len.kind)
    printf("kind %-8s  ", r.linkinfo.kind);
    else
    printf("     %8s  ", "");
    if (r.prop_list._count.alt_ifname) {
    printf("altname ");
    for (i = 0; i < r.prop_list._count.alt_ifname; i++)
    printf("%s ", r.prop_list.alt_ifname[i].str);
    printf(" ");
    }
    if (r.linkinfo._present.data && r.linkinfo.data._present.netkit) {
    struct rt_link_linkinfo_netkit_attrs *netkit;
    const char *name;
    netkit = &r.linkinfo.data.netkit;
    printf("primary %d  ", netkit.primary);
    name = core::ptr::null_mut();
    if (netkit._present.policy)
    name = rt_link_netkit_policy_str(netkit.policy);
    if (name)
    printf("policy %s  ", name);
    }
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn netkit_create(ys: *mut ynl_sock) -> c_int {
    static int netkit_create(struct ynl_sock *ys)
    {
    struct rt_link_getlink_ntf *ntf_gl;
    struct rt_link_newlink_req *req;
    struct ynl_ntf_base_type *ntf;
    int ret;
    req = rt_link_newlink_req_alloc();
    if (!req)
    return -1;
    rt_link_newlink_req_set_nlflags(req, NLM_F_CREATE | NLM_F_ECHO);
    rt_link_newlink_req_set_linkinfo_kind(req, "netkit");
    rt_link_newlink_req_set_linkinfo_data_netkit_policy(req, NETKIT_DROP);
    ret = rt_link_newlink(ys, req);
    rt_link_newlink_req_free(req);
    if (ret)
    return -1;
    if (!ynl_has_ntf(ys))
    return 0;
    ntf = ynl_ntf_dequeue(ys);
    if (!ntf || ntf.cmd != RTM_NEWLINK) {
    ynl_ntf_free(ntf);
    return 0;
    }
    ntf_gl = (void *)ntf;
    ret = ntf_gl.obj._hdr.ifi_index;
    ynl_ntf_free(ntf);
    return ret;
    }
    static void netkit_delete(struct __test_metadata *_metadata,
    struct ynl_sock *ys, int ifindex)
    {
    struct rt_link_dellink_req *req;
    req = rt_link_dellink_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    req._hdr.ifi_index = ifindex;
    EXPECT_EQ(0, rt_link_dellink(ys, req));
    rt_link_dellink_req_free(req);
    }
    FIXTURE(rt_link)
    {
    struct ynl_sock *ys;
    };
    FIXTURE_SETUP(rt_link)
    {
    struct ynl_error yerr;
    self.ys = ynl_sock_create(&ynl_rt_link_family, &yerr);
    ASSERT_NE(core::ptr::null_mut(), self.ys) {
    TH_LOG("failed to create rt-link socket: %s", yerr.msg);
    }
    }
    FIXTURE_TEARDOWN(rt_link)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(rt_link, dump)
    {
    struct rt_link_getlink_req_dump *req;
    struct rt_link_getlink_list *rsp;
    req = rt_link_getlink_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    rsp = rt_link_getlink_dump(self.ys, req);
    rt_link_getlink_req_dump_free(req);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ASSERT_FALSE(ynl_dump_empty(rsp));
    ynl_dump_foreach(rsp, link)
    rt_link_print(_metadata, link);
    rt_link_getlink_list_free(rsp);
    }
    TEST_F(rt_link, netkit)
    {
    struct rt_link_getlink_req_dump *dreq;
    struct rt_link_getlink_list *rsp;
    let mut found: bool = false;
    int netkit_ifindex;
// Create netkit with valid policy
    netkit_ifindex = netkit_create(self.ys);
    ASSERT_GT(netkit_ifindex, 0)
    TH_LOG("failed to create netkit: %s", self.ys.err.msg);
// Verify it appears in a dump
    dreq = rt_link_getlink_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), dreq);
    rsp = rt_link_getlink_dump(self.ys, dreq);
    rt_link_getlink_req_dump_free(dreq);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ynl_dump_foreach(rsp, link) {
    if (link._hdr.ifi_index == netkit_ifindex) {
    rt_link_print(_metadata, link);
    found = true;
    }
    }
    rt_link_getlink_list_free(rsp);
    EXPECT_TRUE(found);
    netkit_delete(_metadata, self.ys, netkit_ifindex);
    }
    TEST_F(rt_link, netkit_err_msg)
    {
    struct rt_link_newlink_req *req;
    int ret;
// Test creating netkit with bad policy - should fail
    req = rt_link_newlink_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    rt_link_newlink_req_set_nlflags(req, NLM_F_CREATE);
    rt_link_newlink_req_set_linkinfo_kind(req, "netkit");
    rt_link_newlink_req_set_linkinfo_data_netkit_policy(req, 10);
    ret = rt_link_newlink(self.ys, req);
    rt_link_newlink_req_free(req);
    EXPECT_NE(0, ret) {
    TH_LOG("creating netkit with bad policy should fail");
    }
// Expect:
// Kernel error: 'Provided default xmit policy not supported' (bad attribute: .linkinfo.data(netkit).policy)
//
    EXPECT_NE(core::ptr::null_mut(), strstr(self.ys.err.msg, "bad attribute: .linkinfo.data(netkit).policy")) {
    TH_LOG("expected extack msg not found: %s",
    self.ys.err.msg);
    }
    }
    TEST_HARNESS_MAIN
