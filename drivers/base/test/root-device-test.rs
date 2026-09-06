//! Automatically rewritten from C to Rust
//! Source: drivers/base/test/root-device-test.c
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
// Copyright 2023 Maxime Ripard <mripard@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_priv {
    pub probe_done: bool,
    pub release_done: bool,
    pub release_wq: wait_queue_head_t,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn root_device_devm_init(test: *mut kunit) -> c_int {
    static int root_device_devm_init(struct kunit *test)
    {
    struct test_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
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
pub const RELEASE_TIMEOUT_MS: c_int = 100;
//
// Tests that a bus-less, non-probed device will run its device-managed
// actions when unregistered.
//
#[no_mangle]
unsafe extern "C" fn root_device_devm_register_unregister_test(test: *mut kunit) {
    static void root_device_devm_register_unregister_test(struct kunit *test)
    {
    struct test_priv *priv = test.priv;
    int ret;
    priv.dev = root_device_register(DEVICE_NAME);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.dev);
    ret = devm_add_action_or_reset(priv.dev, devm_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    root_device_unregister(priv.dev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
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
//
// Tests that a bus-less, non-probed device will run its device-managed
// actions when unregistered, even if someone still holds a reference to
// it.
//
#[no_mangle]
unsafe extern "C" fn root_device_devm_register_get_unregister_with_devm_test(test: *mut kunit) {
    static void root_device_devm_register_get_unregister_with_devm_test(struct kunit *test)
    {
    struct test_priv *priv = test.priv;
    int ret;
    priv.dev = root_device_register(DEVICE_NAME);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.dev);
    get_device(priv.dev);
    ret = devm_add_action_or_reset(priv.dev, devm_put_device_action, priv);
    KUNIT_ASSERT_EQ(test, ret, 0);
    root_device_unregister(priv.dev);
    ret = wait_event_interruptible_timeout(priv.release_wq, priv.release_done,
    msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    }
    static struct kunit_case root_device_devm_tests[] = {
    KUNIT_CASE(root_device_devm_register_unregister_test),
    KUNIT_CASE(root_device_devm_register_get_unregister_with_devm_test),
    {}
    };
    static struct kunit_suite root_device_devm_test_suite = {
    .name = "root-device-devm",
    .init = root_device_devm_init,
    .test_cases = root_device_devm_tests,
    };
    kunit_test_suite(root_device_devm_test_suite);
    MODULE_DESCRIPTION("Test module for root devices");
    MODULE_AUTHOR("Maxime Ripard <mripard@kernel.org>");
    MODULE_LICENSE("GPL");
