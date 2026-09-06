//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/tests/fpga-bridge-test.c
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
//
// KUnit test for the FPGA Bridge
//
// Copyright (C) 2023 Red Hat, Inc.
//
// Author: Marco Pagani <marpagan@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_stats {
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_ctx {
    pub bridge: *mut fpga_bridge,
    pub dev: *mut device,
    pub stats: bridge_stats,
}

//
// Wrapper to avoid a cast warning when passing the action function directly
// to kunit_add_action().
//
    KUNIT_DEFINE_ACTION_WRAPPER(fpga_bridge_unregister_wrapper, fpga_bridge_unregister,
    struct fpga_bridge *);
#[no_mangle]
unsafe extern "C" fn op_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    static int op_enable_set(struct fpga_bridge *bridge, bool enable)
    {
    struct bridge_stats *stats = bridge.priv;
    stats.enable = enable;
    return 0;
    }
//
// Fake FPGA bridge that implements only the enable_set op to track
// the state.
//
    static const struct fpga_bridge_ops fake_bridge_ops = {
    .enable_set = op_enable_set,
    };
//
// register_test_bridge() - Register a fake FPGA bridge for testing.
// @test: KUnit test context object.
// @dev_name: name of the kunit device to be registered
//
// Return: Context of the newly registered FPGA bridge.
//
    static struct bridge_ctx *register_test_bridge(struct kunit *test, const char *dev_name)
    {
    struct bridge_ctx *ctx;
    int ret;
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    ctx.dev = kunit_device_register(test, dev_name);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx.dev);
    ctx.bridge = fpga_bridge_register(ctx.dev, "Fake FPGA bridge", &fake_bridge_ops,
    &ctx.stats);
    KUNIT_ASSERT_FALSE(test, IS_ERR_OR_NULL(ctx.bridge));
    ret = kunit_add_action_or_reset(test, fpga_bridge_unregister_wrapper, ctx.bridge);
    KUNIT_ASSERT_EQ(test, ret, 0);
    return ctx;
    }
#[no_mangle]
unsafe extern "C" fn fpga_bridge_test_get(test: *mut kunit) {
    static void fpga_bridge_test_get(struct kunit *test)
    {
    struct bridge_ctx *ctx = test.priv;
    struct fpga_bridge *bridge;
    bridge = fpga_bridge_get(ctx.dev, core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bridge, ctx.bridge);
    bridge = fpga_bridge_get(ctx.dev, core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, PTR_ERR(bridge), -EBUSY);
    fpga_bridge_put(ctx.bridge);
    }
#[no_mangle]
unsafe extern "C" fn fpga_bridge_test_toggle(test: *mut kunit) {
    static void fpga_bridge_test_toggle(struct kunit *test)
    {
    struct bridge_ctx *ctx = test.priv;
    int ret;
    ret = fpga_bridge_disable(ctx.bridge);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_FALSE(test, ctx.stats.enable);
    ret = fpga_bridge_enable(ctx.bridge);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, ctx.stats.enable);
    }
// Test the functions for getting and controlling a list of bridges
#[no_mangle]
unsafe extern "C" fn fpga_bridge_test_get_put_list(test: *mut kunit) {
    static void fpga_bridge_test_get_put_list(struct kunit *test)
    {
    struct list_head bridge_list;
    struct bridge_ctx *ctx_0, *ctx_1;
    int ret;
    ctx_0 = test.priv;
    ctx_1 = register_test_bridge(test, "fpga-bridge-test-dev-1");
    INIT_LIST_HEAD(&bridge_list);
// Get bridge 0 and add it to the list
    ret = fpga_bridge_get_to_list(ctx_0.dev, core::ptr::null_mut(), &bridge_list);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_PTR_EQ(test, ctx_0.bridge,
    list_first_entry_or_null(&bridge_list, struct fpga_bridge, node));
// Get bridge 1 and add it to the list
    ret = fpga_bridge_get_to_list(ctx_1.dev, core::ptr::null_mut(), &bridge_list);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_PTR_EQ(test, ctx_1.bridge,
    list_first_entry_or_null(&bridge_list, struct fpga_bridge, node));
// Disable an then enable both bridges from the list
    ret = fpga_bridges_disable(&bridge_list);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_FALSE(test, ctx_0.stats.enable);
    KUNIT_EXPECT_FALSE(test, ctx_1.stats.enable);
    ret = fpga_bridges_enable(&bridge_list);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, ctx_0.stats.enable);
    KUNIT_EXPECT_TRUE(test, ctx_1.stats.enable);
// Put and remove both bridges from the list
    fpga_bridges_put(&bridge_list);
    KUNIT_EXPECT_TRUE(test, list_empty(&bridge_list));
    }
#[no_mangle]
unsafe extern "C" fn fpga_bridge_test_init(test: *mut kunit) -> c_int {
    static int fpga_bridge_test_init(struct kunit *test)
    {
    test.priv = register_test_bridge(test, "fpga-bridge-test-dev-0");
    return 0;
    }
    static struct kunit_case fpga_bridge_test_cases[] = {
    KUNIT_CASE(fpga_bridge_test_get),
    KUNIT_CASE(fpga_bridge_test_toggle),
    KUNIT_CASE(fpga_bridge_test_get_put_list),
    {}
    };
    static struct kunit_suite fpga_bridge_suite = {
    .name = "fpga_bridge",
    .init = fpga_bridge_test_init,
    .test_cases = fpga_bridge_test_cases,
    };
    kunit_test_suite(fpga_bridge_suite);
    MODULE_DESCRIPTION("KUnit test for the FPGA Bridge");
    MODULE_LICENSE("GPL");
