//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/rt-addr.c
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

    static void rt_addr_print(struct __test_metadata *_metadata,
    struct rt_addr_getaddr_rsp *a)
    {
    char ifname[IF_NAMESIZE];
    char addr_str[64];
    const char *addr;
    const char *name;
    name = if_indextoname(a._hdr.ifa_index, ifname);
    EXPECT_NE(core::ptr::null_mut(), name);
    if (name)
    ksft_print_msg("%16s: ", name);
    EXPECT_TRUE(a._len.address == 4 || a._len.address == 16);
    switch (a._len.address) {
    case 4:
    addr = inet_ntop(AF_INET, a.address,
    addr_str, sizeof(addr_str));
    break;
    case 16:
    addr = inet_ntop(AF_INET6, a.address,
    addr_str, sizeof(addr_str));
    break;
    default:
    addr = core::ptr::null_mut();
    break;
    }
    if (addr)
    printf("%s", addr);
    else
    printf("[%d]", a._len.address);
    printf("\n");
    }
    FIXTURE(rt_addr)
    {
    struct ynl_sock *ys;
    };
    FIXTURE_SETUP(rt_addr)
    {
    struct ynl_error yerr;
    self.ys = ynl_sock_create(&ynl_rt_addr_family, &yerr);
    ASSERT_NE(core::ptr::null_mut(), self.ys)
    TH_LOG("failed to create rt-addr socket: %s", yerr.msg);
    }
    FIXTURE_TEARDOWN(rt_addr)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(rt_addr, dump)
    {
    struct rt_addr_getaddr_list *rsp;
    struct rt_addr_getaddr_req *req;
    struct in6_addr v6_expected;
    struct in_addr v4_expected;
    let mut found_v4: bool = false;
    let mut found_v6: bool = false;
// The bash wrapper for this test adds these addresses on nsim0,
// make sure we can find them in the dump.
//
    inet_pton(AF_INET, "192.168.1.1", &v4_expected);
    inet_pton(AF_INET6, "2001:db8::1", &v6_expected);
    req = rt_addr_getaddr_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), req);
    rsp = rt_addr_getaddr_dump(self.ys, req);
    rt_addr_getaddr_req_free(req);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ASSERT_FALSE(ynl_dump_empty(rsp)) {
    rt_addr_getaddr_list_free(rsp);
    TH_LOG("no addresses reported");
    }
    ynl_dump_foreach(rsp, addr) {
    rt_addr_print(_metadata, addr);
    found_v4 |= addr._len.address == 4 &&
    !memcmp(addr.address, &v4_expected, 4);
    found_v6 |= addr._len.address == 16 &&
    !memcmp(addr.address, &v6_expected, 16);
    }
    rt_addr_getaddr_list_free(rsp);
    EXPECT_TRUE(found_v4);
    EXPECT_TRUE(found_v6);
    }
    TEST_HARNESS_MAIN
