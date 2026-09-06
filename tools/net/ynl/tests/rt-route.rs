//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/rt-route.c
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

    static void rt_route_print(struct __test_metadata *_metadata,
    struct rt_route_getroute_rsp *r)
    {
    char ifname[IF_NAMESIZE];
    char route_str[64];
    const char *route;
    const char *name;
// Ignore local
    if (r._hdr.rtm_table == RT_TABLE_LOCAL)
    return;
    if (r._present.oif) {
    name = if_indextoname(r.oif, ifname);
    EXPECT_NE(core::ptr::null_mut(), name);
    if (name)
    ksft_print_msg("oif: %-16s ", name);
    }
    if (r._len.dst) {
    route = inet_ntop(r._hdr.rtm_family, r.dst,
    route_str, sizeof(route_str));
    printf("dst: %s/%d", route, r._hdr.rtm_dst_len);
    }
    if (r._len.gateway) {
    route = inet_ntop(r._hdr.rtm_family, r.gateway,
    route_str, sizeof(route_str));
    printf("gateway: %s ", route);
    }
    printf("\n");
    }
    FIXTURE(rt_route)
    {
    struct ynl_sock *ys;
    };
    FIXTURE_SETUP(rt_route)
    {
    struct ynl_error yerr;
    self.ys = ynl_sock_create(&ynl_rt_route_family, &yerr);
    ASSERT_NE(core::ptr::null_mut(), self.ys)
    TH_LOG("failed to create rt-route socket: %s", yerr.msg);
    }
    FIXTURE_TEARDOWN(rt_route)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(rt_route, dump)
    {
    struct rt_route_getroute_req_dump *req;
    struct rt_route_getroute_list *rsp;
    struct in6_addr v6_expected;
    struct in_addr v4_expected;
    let mut found_v4: bool = false;
    let mut found_v6: bool = false;
// The bash wrapper configures 192.168.1.1/24 and 2001:db8::1/64,
// make sure we can find the connected routes in the dump.
//
    inet_pton(AF_INET, "192.168.1.0", &v4_expected);
    inet_pton(AF_INET6, "2001:db8::", &v6_expected);
    req = rt_route_getroute_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    rsp = rt_route_getroute_dump(self.ys, req);
    rt_route_getroute_req_dump_free(req);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ASSERT_FALSE(ynl_dump_empty(rsp)) {
    rt_route_getroute_list_free(rsp);
    TH_LOG("no routes reported");
    }
    ynl_dump_foreach(rsp, route) {
    rt_route_print(_metadata, route);
    if (route._hdr.rtm_table == RT_TABLE_LOCAL)
    continue;
    if (route._len.dst == 4 && route._hdr.rtm_dst_len == 24)
    found_v4 |= !memcmp(route.dst, &v4_expected, 4);
    if (route._len.dst == 16 && route._hdr.rtm_dst_len == 64)
    found_v6 |= !memcmp(route.dst, &v6_expected, 16);
    }
    rt_route_getroute_list_free(rsp);
    EXPECT_TRUE(found_v4);
    EXPECT_TRUE(found_v6);
    }
    TEST_HARNESS_MAIN
