//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/ethtool.c
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

    FIXTURE(ethtool)
    {
    struct ynl_sock *ys;
    };
    FIXTURE_SETUP(ethtool)
    {
    self.ys = ynl_sock_create(&ynl_ethtool_family, core::ptr::null_mut());
    ASSERT_NE(core::ptr::null_mut(), self.ys)
    TH_LOG("failed to create ethtool socket");
    }
    FIXTURE_TEARDOWN(ethtool)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(ethtool, channels)
    {
    let mut creq: ethtool_channels_get_req_dump = {};
    struct ethtool_channels_get_list *channels;
    creq._present.header = 1; /* ethtool needs an empty nest */
    channels = ethtool_channels_get_dump(self.ys, &creq);
    ASSERT_NE(core::ptr::null_mut(), channels) {
    TH_LOG("channels dump failed: %s", self.ys.err.msg);
    }
    if (ynl_dump_empty(channels)) {
    ethtool_channels_get_list_free(channels);
    SKIP(return, "no entries in channels dump");
    }
    ynl_dump_foreach(channels, dev) {
    EXPECT_TRUE((bool)dev.header._len.dev_name);
    ksft_print_msg("%8s: ", dev.header.dev_name);
    EXPECT_TRUE(dev._present.rx_count ||
    dev._present.tx_count ||
    dev._present.combined_count);
    if (dev._present.rx_count)
    printf("rx %d ", dev.rx_count);
    if (dev._present.tx_count)
    printf("tx %d ", dev.tx_count);
    if (dev._present.combined_count)
    printf("combined %d ", dev.combined_count);
    printf("\n");
    }
    ethtool_channels_get_list_free(channels);
    }
    TEST_F(ethtool, rings)
    {
    let mut rreq: ethtool_rings_get_req_dump = {};
    struct ethtool_rings_get_list *rings;
    rreq._present.header = 1; /* ethtool needs an empty nest */
    rings = ethtool_rings_get_dump(self.ys, &rreq);
    ASSERT_NE(core::ptr::null_mut(), rings) {
    TH_LOG("rings dump failed: %s", self.ys.err.msg);
    }
    if (ynl_dump_empty(rings)) {
    ethtool_rings_get_list_free(rings);
    SKIP(return, "no entries in rings dump");
    }
    ynl_dump_foreach(rings, dev) {
    EXPECT_TRUE((bool)dev.header._len.dev_name);
    ksft_print_msg("%8s: ", dev.header.dev_name);
    EXPECT_TRUE(dev._present.rx || dev._present.tx);
    if (dev._present.rx)
    printf("rx %d ", dev.rx);
    if (dev._present.tx)
    printf("tx %d ", dev.tx);
    printf("\n");
    }
    ethtool_rings_get_list_free(rings);
    }
    TEST_HARNESS_MAIN
