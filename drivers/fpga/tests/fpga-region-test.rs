//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/tests/fpga-region-test.c
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
// KUnit test for the FPGA Region
//
// Copyright (C) 2023 Red Hat, Inc.
//
// Author: Marco Pagani <marpagan@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgr_stats {
    pub write_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_stats {
    pub enable: bool,
    pub cycles_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_ctx {
    pub mgr: *mut fpga_manager,
    pub mgr_dev: *mut device,
    pub bridge: *mut fpga_bridge,
    pub bridge_dev: *mut device,
    pub region: *mut fpga_region,
    pub region_dev: *mut device,
    pub bridge_stats: bridge_stats,
    pub mgr_stats: mgr_stats,
}

//
// Wrappers to avoid cast warnings when passing action functions directly
// to kunit_add_action().
//
    KUNIT_DEFINE_ACTION_WRAPPER(fpga_image_info_free_wrapper, fpga_image_info_free,
    struct fpga_image_info *);
    KUNIT_DEFINE_ACTION_WRAPPER(fpga_bridge_unregister_wrapper, fpga_bridge_unregister,
    struct fpga_bridge *);
    KUNIT_DEFINE_ACTION_WRAPPER(fpga_region_unregister_wrapper, fpga_region_unregister,
    struct fpga_region *);
#[no_mangle]
unsafe extern "C" fn op_write(mgr: *mut fpga_manager, buf: *const c_char, count: usize) -> c_int {
    static int op_write(struct fpga_manager *mgr, const char *buf, size_t count)
    {
    struct mgr_stats *stats = mgr.priv;
    stats.write_count++;
    return 0;
    }
//
// Fake FPGA manager that implements only the write op to count the number
// of programming cycles. The internals of the programming sequence are
// tested in the Manager suite since they are outside the responsibility
// of the Region.
//
    static const struct fpga_manager_ops fake_mgr_ops = {
    .write = op_write,
    };
#[no_mangle]
unsafe extern "C" fn op_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    static int op_enable_set(struct fpga_bridge *bridge, bool enable)
    {
    struct bridge_stats *stats = bridge.priv;
    if (!stats.enable && enable)
    stats.cycles_count++;
    stats.enable = enable;
    return 0;
    }
//
// Fake FPGA bridge that implements only enable_set op to count the number
// of activation cycles.
//
    static const struct fpga_bridge_ops fake_bridge_ops = {
    .enable_set = op_enable_set,
    };
#[no_mangle]
unsafe extern "C" fn fake_region_get_bridges(region: *mut fpga_region) -> c_int {
    static int fake_region_get_bridges(struct fpga_region *region)
    {
    struct fpga_bridge *bridge = region.priv;
    return fpga_bridge_get_to_list(bridge.dev.parent, region.info, &region.bridge_list);
    }
#[no_mangle]
unsafe extern "C" fn fake_region_match(dev: *mut device, data: *const c_void) -> c_int {
    static int fake_region_match(struct device *dev, const void *data)
    {
    return dev.parent == data;
    }
#[no_mangle]
unsafe extern "C" fn fpga_region_test_class_find(test: *mut kunit) {
    static void fpga_region_test_class_find(struct kunit *test)
    {
    struct test_ctx *ctx = test.priv;
    struct fpga_region *region;
    region = fpga_region_class_find(core::ptr::null_mut(), ctx.region_dev, fake_region_match);
    KUNIT_EXPECT_PTR_EQ(test, region, ctx.region);
    put_device(&region.dev);
    }
//
// FPGA Region programming test. The Region must call get_bridges() to get
// and control the bridges, and then the Manager for the actual programming.
//
#[no_mangle]
unsafe extern "C" fn fpga_region_test_program_fpga(test: *mut kunit) {
    static void fpga_region_test_program_fpga(struct kunit *test)
    {
    struct test_ctx *ctx = test.priv;
    struct fpga_image_info *img_info;
    char img_buf[4];
    int ret;
    img_info = fpga_image_info_alloc(ctx.mgr_dev);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, img_info);
    ret = kunit_add_action_or_reset(test, fpga_image_info_free_wrapper, img_info);
    KUNIT_ASSERT_EQ(test, ret, 0);
    img_info.buf = img_buf;
    img_info.count = sizeof(img_buf);
    ctx.region.info = img_info;
    ret = fpga_region_program_fpga(ctx.region);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, 1, ctx.mgr_stats.write_count);
    KUNIT_EXPECT_EQ(test, 1, ctx.bridge_stats.cycles_count);
    fpga_bridges_put(&ctx.region.bridge_list);
    ret = fpga_region_program_fpga(ctx.region);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, 2, ctx.mgr_stats.write_count);
    KUNIT_EXPECT_EQ(test, 2, ctx.bridge_stats.cycles_count);
    fpga_bridges_put(&ctx.region.bridge_list);
    }
//
// The configuration used in this test suite uses a single bridge to
// limit the code under test to a single unit. The functions used by the
// Region for getting and controlling bridges are tested (with a list of
// multiple bridges) in the Bridge suite.
//
#[no_mangle]
unsafe extern "C" fn fpga_region_test_init(test: *mut kunit) -> c_int {
    static int fpga_region_test_init(struct kunit *test)
    {
    struct test_ctx *ctx;
    let mut region_info: fpga_region_info = { 0 };
    int ret;
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    ctx.mgr_dev = kunit_device_register(test, "fpga-manager-test-dev");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx.mgr_dev);
    ctx.mgr = devm_fpga_mgr_register(ctx.mgr_dev, "Fake FPGA Manager",
    &fake_mgr_ops, &ctx.mgr_stats);
    KUNIT_ASSERT_FALSE(test, IS_ERR_OR_NULL(ctx.mgr));
    ctx.bridge_dev = kunit_device_register(test, "fpga-bridge-test-dev");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx.bridge_dev);
    ctx.bridge = fpga_bridge_register(ctx.bridge_dev, "Fake FPGA Bridge",
    &fake_bridge_ops, &ctx.bridge_stats);
    KUNIT_ASSERT_FALSE(test, IS_ERR_OR_NULL(ctx.bridge));
    ctx.bridge_stats.enable = true;
    ret = kunit_add_action_or_reset(test, fpga_bridge_unregister_wrapper, ctx.bridge);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ctx.region_dev = kunit_device_register(test, "fpga-region-test-dev");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx.region_dev);
    region_info.mgr = ctx.mgr;
    region_info.priv = ctx.bridge;
    region_info.get_bridges = fake_region_get_bridges;
    ctx.region = fpga_region_register_full(ctx.region_dev, &region_info);
    KUNIT_ASSERT_FALSE(test, IS_ERR_OR_NULL(ctx.region));
    ret = kunit_add_action_or_reset(test, fpga_region_unregister_wrapper, ctx.region);
    KUNIT_ASSERT_EQ(test, ret, 0);
    test.priv = ctx;
    return 0;
    }
    static struct kunit_case fpga_region_test_cases[] = {
    KUNIT_CASE(fpga_region_test_class_find),
    KUNIT_CASE(fpga_region_test_program_fpga),
    {}
    };
    static struct kunit_suite fpga_region_suite = {
    .name = "fpga_region",
    .init = fpga_region_test_init,
    .test_cases = fpga_region_test_cases,
    };
    kunit_test_suite(fpga_region_suite);
    MODULE_DESCRIPTION("KUnit test for the FPGA Region");
    MODULE_LICENSE("GPL");
