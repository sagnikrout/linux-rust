//! Automatically rewritten from C to Rust
//! Source: drivers/of/overlay_test.c
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
// KUnit tests for device tree overlays
//

    let mut kunit_node_name: *const static char  const = "kunit-test";
    let mut kunit_compatible: *const static char  const = "test,empty";
// Test that of_overlay_apply_kunit() adds a node to the live tree
#[no_mangle]
unsafe extern "C" fn of_overlay_apply_kunit_apply(test: *mut kunit) {
    static void of_overlay_apply_kunit_apply(struct kunit *test)
    {
    struct device_node *np;
    KUNIT_ASSERT_EQ(test, 0,
    of_overlay_apply_kunit(test, kunit_overlay_test));
    np = of_find_node_by_name(core::ptr::null_mut(), kunit_node_name);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, np);
    of_node_put(np);
    }
//
// Test that of_overlay_apply_kunit() creates platform devices with the
// expected device_node
//
#[no_mangle]
unsafe extern "C" fn of_overlay_apply_kunit_platform_device(test: *mut kunit) {
    static void of_overlay_apply_kunit_platform_device(struct kunit *test)
    {
    struct platform_device *pdev;
    struct device_node *np;
    KUNIT_ASSERT_EQ(test, 0,
    of_overlay_apply_kunit(test, kunit_overlay_test));
    np = of_find_node_by_name(core::ptr::null_mut(), kunit_node_name);
    of_node_put_kunit(test, np);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, np);
    pdev = of_find_device_by_node(np);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, pdev);
    if (pdev)
    put_device(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn of_overlay_bus_match_compatible(dev: *mut device, data: *const c_void) -> c_int {
    static int of_overlay_bus_match_compatible(struct device *dev, const void *data)
    {
    return of_device_is_compatible(dev.of_node, data);
    }
// Test that of_overlay_apply_kunit() cleans up after the test is finished
#[no_mangle]
unsafe extern "C" fn of_overlay_apply_kunit_cleanup(test: *mut kunit) {
    static void of_overlay_apply_kunit_cleanup(struct kunit *test)
    {
    struct kunit fake;
    struct platform_device *pdev;
    struct device *dev;
    struct device_node *np;
    of_root_kunit_skip(test);
    if (!IS_ENABLED(CONFIG_OF_OVERLAY))
    kunit_skip(test, "requires CONFIG_OF_OVERLAY to apply overlay");
    if (!IS_ENABLED(CONFIG_OF_EARLY_FLATTREE))
    kunit_skip(test, "requires CONFIG_OF_EARLY_FLATTREE for root node");
    kunit_init_test(&fake, "fake test", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, fake.status, KUNIT_SUCCESS);
    KUNIT_ASSERT_EQ(test, 0,
    of_overlay_apply_kunit(&fake, kunit_overlay_test));
    np = of_find_node_by_name(core::ptr::null_mut(), kunit_node_name);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, np);
    of_node_put_kunit(&fake, np);
    pdev = of_find_device_by_node(np);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    put_device(&pdev.dev); /* Not derefing 'pdev' after this */
// Remove overlay
    kunit_cleanup(&fake);
// The node and device should be removed
    np = of_find_node_by_name(core::ptr::null_mut(), kunit_node_name);
    KUNIT_EXPECT_PTR_EQ(test, core::ptr::null_mut(), np);
    of_node_put(np);
    dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), kunit_compatible,
    of_overlay_bus_match_compatible);
    KUNIT_EXPECT_PTR_EQ(test, core::ptr::null_mut(), dev);
    put_device(dev);
    }
    static struct kunit_case of_overlay_apply_kunit_test_cases[] = {
    KUNIT_CASE(of_overlay_apply_kunit_apply),
    KUNIT_CASE(of_overlay_apply_kunit_platform_device),
    KUNIT_CASE(of_overlay_apply_kunit_cleanup),
    {}
    };
//
// Test suite for test managed device tree overlays.
//
    static struct kunit_suite of_overlay_apply_kunit_suite = {
    .name = "of_overlay_apply_kunit",
    .test_cases = of_overlay_apply_kunit_test_cases,
    };
    kunit_test_suites(
    &of_overlay_apply_kunit_suite,
    );
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("KUnit tests for device tree overlays");
