//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/ovs.c
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

    static void ovs_print_datapath(struct __test_metadata *_metadata,
    struct ovs_datapath_get_rsp *dp)
    {
    EXPECT_TRUE((bool)dp._len.name);
    if (!dp._len.name)
    return;
    EXPECT_TRUE((bool)dp._hdr.dp_ifindex);
    ksft_print_msg("%s(%d): pid:%u cache:%u\n",
    dp.name, dp._hdr.dp_ifindex,
    dp.upcall_pid, dp.masks_cache_size);
    }
    FIXTURE(ovs)
    {
    struct ynl_sock *ys;
    char *dp_name;
    };
    FIXTURE_SETUP(ovs)
    {
    self.ys = ynl_sock_create(&ynl_ovs_datapath_family, core::ptr::null_mut());
    ASSERT_NE(core::ptr::null_mut(), self.ys)
    TH_LOG("failed to create OVS datapath socket");
    }
    FIXTURE_TEARDOWN(ovs)
    {
    if (self.dp_name) {
    struct ovs_datapath_del_req *req;
    req = ovs_datapath_del_req_alloc();
    if (req) {
    ovs_datapath_del_req_set_name(req, self.dp_name);
    ovs_datapath_del(self.ys, req);
    ovs_datapath_del_req_free(req);
    }
    }
    ynl_sock_destroy(self.ys);
    }
    TEST_F(ovs, crud)
    {
    struct ovs_datapath_get_req_dump *dreq;
    struct ovs_datapath_new_req *new_req;
    struct ovs_datapath_get_list *dps;
    struct ovs_datapath_get_rsp *dp;
    struct ovs_datapath_get_req *req;
    let mut found: bool = false;
    int err;
    new_req = ovs_datapath_new_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), new_req);
    ovs_datapath_new_req_set_upcall_pid(new_req, 1);
    ovs_datapath_new_req_set_name(new_req, "ynl-test");
    err = ovs_datapath_new(self.ys, new_req);
    ovs_datapath_new_req_free(new_req);
    ASSERT_EQ(0, err) {
    TH_LOG("new failed: %s", self.ys.err.msg);
    }
    self.dp_name = "ynl-test";
    ksft_print_msg("get:\n");
    req = ovs_datapath_get_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    ovs_datapath_get_req_set_name(req, "ynl-test");
    dp = ovs_datapath_get(self.ys, req);
    ovs_datapath_get_req_free(req);
    ASSERT_NE(core::ptr::null_mut(), dp) {
    TH_LOG("get failed: %s", self.ys.err.msg);
    }
    ovs_print_datapath(_metadata, dp);
    EXPECT_STREQ("ynl-test", dp.name);
    ovs_datapath_get_rsp_free(dp);
    ksft_print_msg("dump:\n");
    dreq = ovs_datapath_get_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), dreq);
    dps = ovs_datapath_get_dump(self.ys, dreq);
    ovs_datapath_get_req_dump_free(dreq);
    ASSERT_NE(core::ptr::null_mut(), dps) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ynl_dump_foreach(dps, d) {
    ovs_print_datapath(_metadata, d);
    if (d.name && !strcmp(d.name, "ynl-test"))
    found = true;
    }
    ovs_datapath_get_list_free(dps);
    EXPECT_TRUE(found);
    }
    TEST_HARNESS_MAIN
