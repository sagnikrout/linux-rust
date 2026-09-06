//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/devlink.c
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

    FIXTURE(devlink)
    {
    struct ynl_sock *ys;
    };
    FIXTURE_SETUP(devlink)
    {
    self.ys = ynl_sock_create(&ynl_devlink_family, core::ptr::null_mut());
    ASSERT_NE(core::ptr::null_mut(), self.ys)
    TH_LOG("failed to create devlink socket");
    }
    FIXTURE_TEARDOWN(devlink)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(devlink, dump)
    {
    struct devlink_get_list *devs;
    devs = devlink_get_dump(self.ys);
    ASSERT_NE(core::ptr::null_mut(), devs) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    if (ynl_dump_empty(devs)) {
    devlink_get_list_free(devs);
    SKIP(return, "no entries in dump");
    }
    ynl_dump_foreach(devs, d) {
    EXPECT_TRUE((bool)d._len.bus_name);
    EXPECT_TRUE((bool)d._len.dev_name);
    ksft_print_msg("%s/%s\n", d.bus_name, d.dev_name);
    }
    devlink_get_list_free(devs);
    }
    TEST_F(devlink, info)
    {
    struct devlink_get_list *devs;
    devs = devlink_get_dump(self.ys);
    ASSERT_NE(core::ptr::null_mut(), devs) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    if (ynl_dump_empty(devs)) {
    devlink_get_list_free(devs);
    SKIP(return, "no devices to query");
    }
    ynl_dump_foreach(devs, d) {
    struct devlink_info_get_req *info_req;
    struct devlink_info_get_rsp *info_rsp;
    unsigned int i;
    EXPECT_TRUE((bool)d._len.bus_name);
    EXPECT_TRUE((bool)d._len.dev_name);
    ksft_print_msg("%s/%s:\n", d.bus_name, d.dev_name);
    info_req = devlink_info_get_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), info_req);
    devlink_info_get_req_set_bus_name(info_req, d.bus_name);
    devlink_info_get_req_set_dev_name(info_req, d.dev_name);
    info_rsp = devlink_info_get(self.ys, info_req);
    devlink_info_get_req_free(info_req);
    ASSERT_NE(core::ptr::null_mut(), info_rsp) {
    devlink_get_list_free(devs);
    TH_LOG("info_get failed: %s", self.ys.err.msg);
    }
    EXPECT_TRUE((bool)info_rsp._len.info_driver_name);
    if (info_rsp._len.info_driver_name)
    ksft_print_msg("  driver: %s\n",
    info_rsp.info_driver_name);
    if (info_rsp._count.info_version_running)
    ksft_print_msg("  running fw:\n");
    for (i = 0; i < info_rsp._count.info_version_running; i++)
    ksft_print_msg("    %s: %s\n",
    info_rsp.info_version_running[i].info_version_name,
    info_rsp.info_version_running[i].info_version_value);
    devlink_info_get_rsp_free(info_rsp);
    }
    devlink_get_list_free(devs);
    }
    TEST_HARNESS_MAIN
