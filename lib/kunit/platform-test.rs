//! Automatically rewritten from C to Rust
//! Source: lib/kunit/platform-test.c
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
// KUnit test for KUnit platform driver infrastructure.
//

//
// Test that kunit_platform_device_alloc() creates a platform device.
//
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_alloc_test(test: *mut kunit) {
    static void kunit_platform_device_alloc_test(struct kunit *test)
    {
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test,
    kunit_platform_device_alloc(test, "kunit-platform", 1));
    }
//
// Test that kunit_platform_device_add() registers a platform device on the
// platform bus with the proper name and id.
//
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_add_test(test: *mut kunit) {
    static void kunit_platform_device_add_test(struct kunit *test)
    {
    struct platform_device *pdev;
    const char *name = "kunit-platform-add";
    let mut id: c_int = -1;
    pdev = kunit_platform_device_alloc(test, name, id);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_EXPECT_EQ(test, 0, kunit_platform_device_add(test, pdev));
    KUNIT_EXPECT_TRUE(test, dev_is_platform(&pdev.dev));
    KUNIT_EXPECT_STREQ(test, pdev.name, name);
    KUNIT_EXPECT_EQ(test, pdev.id, id);
    }
//
// Test that kunit_platform_device_add() called twice with the same device name
// and id fails the second time and properly cleans up.
//
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_add_twice_fails_test(test: *mut kunit) {
    static void kunit_platform_device_add_twice_fails_test(struct kunit *test)
    {
    struct platform_device *pdev;
    const char *name = "kunit-platform-add-2";
    let mut id: c_int = -1;
    pdev = kunit_platform_device_alloc(test, name, id);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_add(test, pdev));
    pdev = kunit_platform_device_alloc(test, name, id);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_EXPECT_NE(test, 0, kunit_platform_device_add(test, pdev));
    }
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_find_by_name(dev: *mut device, data: *const c_void) -> c_int {
    static int kunit_platform_device_find_by_name(struct device *dev, const void *data)
    {
    return strcmp(dev_name(dev), data) == 0;
    }
//
// Test that kunit_platform_device_add() cleans up by removing the platform
// device when the test finishes.
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_add_cleans_up(test: *mut kunit) {
    static void kunit_platform_device_add_cleans_up(struct kunit *test)
    {
    struct platform_device *pdev;
    const char *name = "kunit-platform-clean";
    let mut id: c_int = -1;
    struct kunit fake;
    struct device *dev;
    kunit_init_test(&fake, "kunit_platform_device_add_fake_test", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, fake.status, KUNIT_SUCCESS);
    pdev = kunit_platform_device_alloc(&fake, name, id);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_add(&fake, pdev));
    dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), name,
    kunit_platform_device_find_by_name);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
    put_device(dev);
// Remove pdev
    kunit_cleanup(&fake);
//
// Failing to migrate the kunit_resource would lead to an extra
// put_device() call on the platform device. The best we can do here is
// make sure the device no longer exists on the bus, but if something
// is wrong we'll see a refcount underflow here. We can't test for a
// refcount underflow because the kref matches the lifetime of the
// device which should already be freed and could be used by something
// else.
//
    dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), name,
    kunit_platform_device_find_by_name);
    KUNIT_EXPECT_PTR_EQ(test, core::ptr::null_mut(), dev);
    put_device(dev);
    }
//
// Test suite for struct platform_device kunit APIs
//
    static struct kunit_case kunit_platform_device_test_cases[] = {
    KUNIT_CASE(kunit_platform_device_alloc_test),
    KUNIT_CASE(kunit_platform_device_add_test),
    KUNIT_CASE(kunit_platform_device_add_twice_fails_test),
    KUNIT_CASE(kunit_platform_device_add_cleans_up),
    {}
    };
    static struct kunit_suite kunit_platform_device_suite = {
    .name = "kunit_platform_device",
    .test_cases = kunit_platform_device_test_cases,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_platform_driver_test_context {
    pub pdrv: platform_driver,
    pub data: *const c_char,
}

    let mut test_data: *const static char  const = "test data";
    static inline struct kunit_platform_driver_test_context *
    to_test_context(struct platform_device *pdev)
    {
    return container_of(to_platform_driver(pdev.dev.driver),
    struct kunit_platform_driver_test_context,
    pdrv);
    }
#[no_mangle]
unsafe extern "C" fn kunit_platform_driver_probe(pdev: *mut platform_device) -> c_int {
    static int kunit_platform_driver_probe(struct platform_device *pdev)
    {
    struct kunit_platform_driver_test_context *ctx;
    ctx = to_test_context(pdev);
    ctx.data = test_data;
    return 0;
    }
// Test that kunit_platform_driver_register() registers a driver that probes.
#[no_mangle]
unsafe extern "C" fn kunit_platform_driver_register_test(test: *mut kunit) {
    static void kunit_platform_driver_register_test(struct kunit *test)
    {
    struct platform_device *pdev;
    struct kunit_platform_driver_test_context *ctx;
    DECLARE_COMPLETION_ONSTACK(comp);
    const char *name = "kunit-platform-register";
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    pdev = kunit_platform_device_alloc(test, name, -1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_add(test, pdev));
    ctx.pdrv.probe = kunit_platform_driver_probe;
    ctx.pdrv.driver.name = name;
    ctx.pdrv.driver.owner = THIS_MODULE;
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_prepare_wait_for_probe(test, pdev, &comp));
    KUNIT_EXPECT_EQ(test, 0, kunit_platform_driver_register(test, &ctx.pdrv));
    KUNIT_EXPECT_NE(test, 0, wait_for_completion_timeout(&comp, 3 * HZ));
    KUNIT_EXPECT_STREQ(test, ctx.data, test_data);
    }
//
// Test that kunit_platform_device_prepare_wait_for_probe() completes the completion
// when the device is already probed.
//
#[no_mangle]
unsafe extern "C" fn kunit_platform_device_prepare_wait_for_probe_completes_when_already_probed(test: *mut kunit) {
    static void kunit_platform_device_prepare_wait_for_probe_completes_when_already_probed(struct kunit *test)
    {
    struct platform_device *pdev;
    struct kunit_platform_driver_test_context *ctx;
    DECLARE_COMPLETION_ONSTACK(comp);
    const char *name = "kunit-platform-wait";
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    pdev = kunit_platform_device_alloc(test, name, -1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_add(test, pdev));
    ctx.pdrv.probe = kunit_platform_driver_probe;
    ctx.pdrv.driver.name = name;
    ctx.pdrv.driver.owner = THIS_MODULE;
// Make sure driver has actually probed
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_prepare_wait_for_probe(test, pdev, &comp));
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_driver_register(test, &ctx.pdrv));
    KUNIT_ASSERT_NE(test, 0, wait_for_completion_timeout(&comp, 3 * HZ));
    reinit_completion(&comp);
    KUNIT_ASSERT_EQ(test, 0, kunit_platform_device_prepare_wait_for_probe(test, pdev, &comp));
    KUNIT_EXPECT_NE(test, 0, wait_for_completion_timeout(&comp, HZ));
    }
    static struct kunit_case kunit_platform_driver_test_cases[] = {
    KUNIT_CASE(kunit_platform_driver_register_test),
    KUNIT_CASE(kunit_platform_device_prepare_wait_for_probe_completes_when_already_probed),
    {}
    };
//
// Test suite for struct platform_driver kunit APIs
//
    static struct kunit_suite kunit_platform_driver_suite = {
    .name = "kunit_platform_driver",
    .test_cases = kunit_platform_driver_test_cases,
    };
    kunit_test_suites(
    &kunit_platform_device_suite,
    &kunit_platform_driver_suite,
    );
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("KUnit test for KUnit platform driver infrastructure");
