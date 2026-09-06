//! Automatically rewritten from C to Rust
//! Source: drivers/base/test/platform-device-test.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_priv {
    pub probe_done: bool,
    pub release_done: bool,
    pub probe_wq: wait_queue_head_t,
    pub release_wq: wait_queue_head_t,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn platform_device_devm_init(test: *mut kunit) -> c_int {
    static int platform_device_devm_init(struct kunit *test)
    {
    struct test_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
    init_waitqueue_head(&priv.probe_wq);
    init_waitqueue_head(&priv.release_wq);
    test.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn devm_device_action(ptr: *mut c_void) {
    static void devm_device_action(void *ptr)
    {
    struct test_priv *priv = ptr;
    priv.release_done = true;
    wake_up_interruptible(&priv.release_wq);
    }
#[no_mangle]
unsafe extern "C" fn devm_put_device_action(ptr: *mut c_void) {
    static void devm_put_device_action(void *ptr)
    {
    struct test_priv *priv = ptr;
    put_device(priv.dev);
    priv.release_done = true;
    wake_up_interruptible(&priv.release_wq);
    }
pub const RELEASE_TIMEOUT_MS: c_int = 100;
//
// Tests that a platform bus, non-probed device will run its
// device-managed actions when unregistered.
//
#[no_mangle]
unsafe extern "C" fn platform_device_devm_register_unregister_test(test: *mut kunit) {
    static void platform_device_devm_register_unregister_test(struct kunit *test)
    {
    struct platform_device *pdev;
    struct test_priv *priv = test.priv;
    int ret;
    pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    priv.dev = &pdev.dev;
    ret = devm_add_action_or_reset(priv.dev, devm_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    }
//
// Tests that a platform bus, non-probed device will run its
// device-managed actions when unregistered, even if someone still holds
// a reference to it.
//
#[no_mangle]
unsafe extern "C" fn platform_device_devm_register_get_unregister_with_devm_test(test: *mut kunit) {
    static void platform_device_devm_register_get_unregister_with_devm_test(struct kunit *test)
    {
    struct platform_device *pdev;
    struct test_priv *priv = test.priv;
    int ret;
    pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    priv.dev = &pdev.dev;
    get_device(priv.dev);
    ret = devm_add_action_or_reset(priv.dev, devm_put_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    }
#[no_mangle]
unsafe extern "C" fn fake_probe(pdev: *mut platform_device) -> c_int {
    static int fake_probe(struct platform_device *pdev)
    {
    struct test_priv *priv = platform_get_drvdata(pdev);
    priv.probe_done = true;
    wake_up_interruptible(&priv.probe_wq);
    return 0;
    }
    static struct platform_driver fake_driver = {
    .probe	= fake_probe,
    .driver = {
    .name = DEVICE_NAME,
    },
    };
//
// Tests that a platform bus, probed device will run its device-managed
// actions when unregistered.
//
#[no_mangle]
unsafe extern "C" fn probed_platform_device_devm_register_unregister_test(test: *mut kunit) {
    static void probed_platform_device_devm_register_unregister_test(struct kunit *test)
    {
    struct platform_device *pdev;
    struct test_priv *priv = test.priv;
    int ret;
    ret = platform_driver_register(&fake_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    priv.dev = &pdev.dev;
    platform_set_drvdata(pdev, priv);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = wait_event_interruptible_timeout(priv.probe_wq, priv.probe_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_ASSERT_GT(test, ret, 0);
    ret = devm_add_action_or_reset(priv.dev, devm_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    platform_driver_unregister(&fake_driver);
    }
//
// Tests that a platform bus, probed device will run its device-managed
// actions when unregistered, even if someone still holds a reference to
// it.
//
#[no_mangle]
unsafe extern "C" fn probed_platform_device_devm_register_get_unregister_with_devm_test(test: *mut kunit) {
    static void probed_platform_device_devm_register_get_unregister_with_devm_test(struct kunit *test)
    {
    struct platform_device *pdev;
    struct test_priv *priv = test.priv;
    int ret;
    ret = platform_driver_register(&fake_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    priv.dev = &pdev.dev;
    platform_set_drvdata(pdev, priv);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = wait_event_interruptible_timeout(priv.probe_wq, priv.probe_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_ASSERT_GT(test, ret, 0);
    get_device(priv.dev);
    ret = devm_add_action_or_reset(priv.dev, devm_put_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    platform_driver_unregister(&fake_driver);
    }
    static struct kunit_case platform_device_devm_tests[] = {
    KUNIT_CASE(platform_device_devm_register_unregister_test),
    KUNIT_CASE(platform_device_devm_register_get_unregister_with_devm_test),
    KUNIT_CASE(probed_platform_device_devm_register_unregister_test),
    KUNIT_CASE(probed_platform_device_devm_register_get_unregister_with_devm_test),
    {}
    };
    static struct kunit_suite platform_device_devm_test_suite = {
    .name = "platform-device-devm",
    .init = platform_device_devm_init,
    .test_cases = platform_device_devm_tests,
    };
#[no_mangle]
unsafe extern "C" fn platform_device_find_by_null_test(test: *mut kunit) {
    static void platform_device_find_by_null_test(struct kunit *test)
    {
    struct platform_device *pdev;
    int ret;
    pdev = kunit_platform_device_alloc(test, DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    ret = kunit_platform_device_add(test, pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_PTR_EQ(test, of_find_device_by_node(core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_of_node(&platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_fwnode(&platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_acpi_dev(&platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_FALSE(test, device_match_of_node(&pdev.dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_fwnode(&pdev.dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_acpi_dev(&pdev.dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_acpi_handle(&pdev.dev, core::ptr::null_mut()));
    }
    static struct kunit_case platform_device_match_tests[] = {
    KUNIT_CASE(platform_device_find_by_null_test),
    {}
    };
    static struct kunit_suite platform_device_match_test_suite = {
    .name = "platform-device-match",
    .test_cases = platform_device_match_tests,
    };
#[no_mangle]
unsafe extern "C" fn platform_device_swnode_test_probe(pdev: *mut platform_device) -> c_int {
    static int platform_device_swnode_test_probe(struct platform_device *pdev)
    {
    return 0;
    }
    static struct platform_driver platform_swnode_test_driver = {
    .probe = platform_device_swnode_test_probe,
    .driver = {
    .name = DEVICE_NAME,
    },
    };
    let mut platform_device_test_swnode: static struct software_node = { };
//
// Check that reusing a software node works correctly. If the call to
// platform_device_register_full() fails after adding the secondary firmware
// node, the software node must be unregistered in the device's release()
// callback or the subsequent call to platform_device_register_full() will fail
// with -EBUSY due to the software node already having been registered.
//
#[no_mangle]
unsafe extern "C" fn platform_device_swnode_add_twice(test: *mut kunit) {
    static void platform_device_swnode_add_twice(struct kunit *test)
    {
    struct platform_device_info pdevinfo;
    struct platform_device *pdev;
    struct fwnode_handle *fwnode;
    let mut bound: bool = false;
    int ret;
    fwnode = kunit_kzalloc(test, sizeof(*fwnode), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    ret = kunit_platform_driver_register(test, &platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    fwnode_init(fwnode, core::ptr::null_mut());
    pdevinfo = (struct platform_device_info){
    .name = DEVICE_NAME,
    .id = PLATFORM_DEVID_NONE,
    .fwnode = fwnode,
    .swnode = &platform_device_test_swnode,
    };
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    scoped_guard(device, &pdev.dev)
    bound = device_is_bound(&pdev.dev);
    KUNIT_ASSERT_TRUE(test, bound);
    platform_device_unregister(pdev);
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    scoped_guard(device, &pdev.dev)
    bound = device_is_bound(&pdev.dev);
    KUNIT_ASSERT_TRUE(test, bound);
    platform_device_unregister(pdev);
    }
//
// Check that passing a software node as the primary firmware node of the
// platform device does not result in it being unregistered by the call to
// device_remove_software_node() in its release path.
//
#[no_mangle]
unsafe extern "C" fn platform_device_swnode_as_primary(test: *mut kunit) {
    static void platform_device_swnode_as_primary(struct kunit *test)
    {
    struct platform_device_info pdevinfo;
    struct platform_device *pdev;
    struct fwnode_handle *fwnode;
    let mut bound: bool = false;
    int ret;
    ret = kunit_platform_driver_register(test, &platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    fwnode = kunit_software_node_register(test, &platform_device_test_swnode);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    pdevinfo = (struct platform_device_info){
    .name = DEVICE_NAME,
    .id = PLATFORM_DEVID_NONE,
    .fwnode = fwnode,
    };
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    scoped_guard(device, &pdev.dev)
    bound = device_is_bound(&pdev.dev);
    KUNIT_ASSERT_TRUE(test, bound);
    platform_device_unregister(pdev);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, software_node_fwnode(&platform_device_test_swnode));
    }
//
// Check that passing two software nodes to platform_device_register_full()
// fails.
//
#[no_mangle]
unsafe extern "C" fn platform_device_two_swnodes(test: *mut kunit) {
    static void platform_device_two_swnodes(struct kunit *test)
    {
    static const struct property_entry properties[] = {
    PROPERTY_ENTRY_U32("foo", 42),
    { }
    };
    struct platform_device_info pdevinfo;
    struct platform_device *pdev;
    struct fwnode_handle *fwnode;
    int ret;
    ret = kunit_platform_driver_register(test, &platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    fwnode = kunit_software_node_register(test, &platform_device_test_swnode);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    pdevinfo = (struct platform_device_info){
    .name = DEVICE_NAME,
    .id = PLATFORM_DEVID_NONE,
    .fwnode = fwnode,
    .swnode = &platform_device_test_swnode,
    };
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL,
    "Expected errno == -EINVAL, got: %pe", pdev);
    pdevinfo = (struct platform_device_info){
    .name = DEVICE_NAME,
    .id = PLATFORM_DEVID_NONE,
    .swnode = &platform_device_test_swnode,
    .properties = properties,
    };
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL,
    "Expected errno == -EINVAL, got: %pe", pdev);
    pdevinfo = (struct platform_device_info){
    .name = DEVICE_NAME,
    .id = PLATFORM_DEVID_NONE,
    .fwnode = fwnode,
    .properties = properties,
    };
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL,
    "Expected errno == -EINVAL, got: %pe", pdev);
    }
    static struct kunit_case platform_device_swnode_tests[] = {
    KUNIT_CASE(platform_device_swnode_add_twice),
    KUNIT_CASE(platform_device_swnode_as_primary),
    KUNIT_CASE(platform_device_two_swnodes),
    {}
    };
    static struct kunit_suite platform_device_swnode_test_suite = {
    .name = "platform-device-swnode",
    .test_cases = platform_device_swnode_tests,
    };
    kunit_test_suites(
    &platform_device_devm_test_suite,
    &platform_device_match_test_suite,
    &platform_device_swnode_test_suite,
    );
    MODULE_DESCRIPTION("Test module for platform devices");
    MODULE_AUTHOR("Maxime Ripard <mripard@kernel.org>");
    MODULE_LICENSE("GPL");
