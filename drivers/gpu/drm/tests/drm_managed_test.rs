//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_managed_test.c
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

// Ought to be enough for anybody
pub const TEST_TIMEOUT_MS: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct managed_test_priv {
    pub drm: *mut drm_device,
    pub action_done: bool,
    pub action_wq: wait_queue_head_t,
}

#[no_mangle]
unsafe extern "C" fn drm_action(drm: *mut drm_device, ptr: *mut c_void) {
    static void drm_action(struct drm_device *drm, void *ptr)
    {
    struct managed_test_priv *priv = ptr;
    priv.action_done = true;
    wake_up_interruptible(&priv.action_wq);
    }
//
// The test verifies that the release action is called when
// drmm_release_action is called.
//
#[no_mangle]
unsafe extern "C" fn drm_test_managed_release_action(test: *mut kunit) {
    static void drm_test_managed_release_action(struct kunit *test)
    {
    struct managed_test_priv *priv = test.priv;
    int ret;
    ret = drmm_add_action_or_reset(priv.drm, drm_action, priv);
    KUNIT_EXPECT_EQ(test, ret, 0);
    ret = drm_dev_register(priv.drm, 0);
    KUNIT_ASSERT_EQ(test, ret, 0);
    drmm_release_action(priv.drm, drm_action, priv);
    ret = wait_event_interruptible_timeout(priv.action_wq, priv.action_done,
    msecs_to_jiffies(TEST_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    drm_dev_unregister(priv.drm);
    drm_kunit_helper_free_device(test, priv.drm.dev);
    }
//
// The test verifies that the release action is called automatically when the
// device is released.
//
#[no_mangle]
unsafe extern "C" fn drm_test_managed_run_action(test: *mut kunit) {
    static void drm_test_managed_run_action(struct kunit *test)
    {
    struct managed_test_priv *priv = test.priv;
    int ret;
    ret = drmm_add_action_or_reset(priv.drm, drm_action, priv);
    KUNIT_EXPECT_EQ(test, ret, 0);
    ret = drm_dev_register(priv.drm, 0);
    KUNIT_ASSERT_EQ(test, ret, 0);
    drm_dev_unregister(priv.drm);
    drm_kunit_helper_free_device(test, priv.drm.dev);
    ret = wait_event_interruptible_timeout(priv.action_wq, priv.action_done,
    msecs_to_jiffies(TEST_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    }
#[no_mangle]
unsafe extern "C" fn drm_managed_test_init(test: *mut kunit) -> c_int {
    static int drm_managed_test_init(struct kunit *test)
    {
    struct managed_test_priv *priv;
    struct device *dev;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
    init_waitqueue_head(&priv.action_wq);
    dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
//
// DRM device can't be embedded in priv, since priv->action_done needs
// to remain allocated beyond both parent device and drm_device
// lifetime.
//
    priv.drm = __drm_kunit_helper_alloc_drm_device(test, dev, sizeof(*priv.drm), 0,
    DRIVER_MODESET);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.drm);
    test.priv = priv;
    return 0;
    }
    static struct kunit_case drm_managed_tests[] = {
    KUNIT_CASE(drm_test_managed_release_action),
    KUNIT_CASE(drm_test_managed_run_action),
    {}
    };
    static struct kunit_suite drm_managed_test_suite = {
    .name = "drm_managed",
    .init = drm_managed_test_init,
    .test_cases = drm_managed_tests
    };
    kunit_test_suite(drm_managed_test_suite);
    MODULE_AUTHOR("Maxime Ripard <maxime@cerno.tech>");
    MODULE_DESCRIPTION("KUnit DRM managed test suite");
    MODULE_LICENSE("GPL");
