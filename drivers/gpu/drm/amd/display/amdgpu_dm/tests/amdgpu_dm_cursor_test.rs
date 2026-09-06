//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_cursor_test.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// KUnit tests for amdgpu_dm_cursor.c
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

// Tests for amdgpu_dm_should_update_native_cursor()
//
// dm_test_should_update_native_cursor_without_crtc - Test NULL crtc cases update native cursor
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_update_native_cursor_without_crtc(test: *mut kunit) {
    static void dm_test_should_update_native_cursor_without_crtc(struct kunit *test)
    {
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_should_update_native_cursor(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), false));
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_should_update_native_cursor(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), true));
    }
//
// dm_test_should_update_native_cursor_disable_native - Test disable path reads old crtc cursor mode
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_update_native_cursor_disable_native(test: *mut kunit) {
    static void dm_test_should_update_native_cursor_disable_native(struct kunit *test)
    {
    struct dm_crtc_state *dm_crtc_state;
    struct drm_atomic_commit *state;
    struct drm_crtc *crtc;
    state = kunit_kzalloc(test, sizeof(*state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state);
    crtc = kunit_kzalloc(test, sizeof(*crtc), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, crtc);
    dm_crtc_state = kunit_kzalloc(test, sizeof(*dm_crtc_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dm_crtc_state);
    state.crtcs = kunit_kzalloc(test, sizeof(*state.crtcs), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state.crtcs);
    crtc.index = 0;
    dm_crtc_state.cursor_mode = DM_CURSOR_NATIVE_MODE;
    state.crtcs[0].old_state = &dm_crtc_state.base;
    KUNIT_EXPECT_TRUE(test,
    amdgpu_dm_should_update_native_cursor(state, crtc, core::ptr::null_mut(), false));
    }
//
// dm_test_should_update_native_cursor_enable_overlay - Test enable path reads new crtc cursor mode
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_update_native_cursor_enable_overlay(test: *mut kunit) {
    static void dm_test_should_update_native_cursor_enable_overlay(struct kunit *test)
    {
    struct dm_crtc_state *dm_crtc_state;
    struct drm_atomic_commit *state;
    struct drm_crtc *crtc;
    state = kunit_kzalloc(test, sizeof(*state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state);
    crtc = kunit_kzalloc(test, sizeof(*crtc), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, crtc);
    dm_crtc_state = kunit_kzalloc(test, sizeof(*dm_crtc_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dm_crtc_state);
    state.crtcs = kunit_kzalloc(test, sizeof(*state.crtcs), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state.crtcs);
    crtc.index = 0;
    dm_crtc_state.cursor_mode = DM_CURSOR_OVERLAY_MODE;
    state.crtcs[0].new_state = &dm_crtc_state.base;
    KUNIT_EXPECT_FALSE(test,
    amdgpu_dm_should_update_native_cursor(state, core::ptr::null_mut(), crtc, true));
    }
// Tests for dm_get_oriented_plane_size()
//
// dm_test_oriented_plane_size_rotate_0 - Test Oriented plane size rotate 0
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_oriented_plane_size_rotate_0(test: *mut kunit) {
    static void dm_test_oriented_plane_size_rotate_0(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut src_w: c_int = 0;
    let mut src_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_0;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    dm_get_oriented_plane_size(&plane_state, &src_w, &src_h);
    KUNIT_EXPECT_EQ(test, src_w, 1920);
    KUNIT_EXPECT_EQ(test, src_h, 1080);
    }
//
// dm_test_oriented_plane_size_rotate_90 - Test Oriented plane size rotate 90
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_oriented_plane_size_rotate_90(test: *mut kunit) {
    static void dm_test_oriented_plane_size_rotate_90(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut src_w: c_int = 0;
    let mut src_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_90;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    dm_get_oriented_plane_size(&plane_state, &src_w, &src_h);
    KUNIT_EXPECT_EQ(test, src_w, 1080);
    KUNIT_EXPECT_EQ(test, src_h, 1920);
    }
//
// dm_test_oriented_plane_size_rotate_180 - Test Oriented plane size rotate 180
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_oriented_plane_size_rotate_180(test: *mut kunit) {
    static void dm_test_oriented_plane_size_rotate_180(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut src_w: c_int = 0;
    let mut src_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_180;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    dm_get_oriented_plane_size(&plane_state, &src_w, &src_h);
    KUNIT_EXPECT_EQ(test, src_w, 1920);
    KUNIT_EXPECT_EQ(test, src_h, 1080);
    }
//
// dm_test_oriented_plane_size_rotate_270 - Test Oriented plane size rotate 270
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_oriented_plane_size_rotate_270(test: *mut kunit) {
    static void dm_test_oriented_plane_size_rotate_270(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut src_w: c_int = 0;
    let mut src_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_270;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    dm_get_oriented_plane_size(&plane_state, &src_w, &src_h);
    KUNIT_EXPECT_EQ(test, src_w, 1080);
    KUNIT_EXPECT_EQ(test, src_h, 1920);
    }
// Tests for dm_get_plane_scale()
//
// dm_test_get_plane_scale_identity - Test Get plane scale identity
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_plane_scale_identity(test: *mut kunit) {
    static void dm_test_get_plane_scale_identity(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut scale_w: c_int = 0;
    let mut scale_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_0;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    plane_state.crtc_w = 1920;
    plane_state.crtc_h = 1080;
    dm_get_plane_scale(&plane_state, &scale_w, &scale_h);
    KUNIT_EXPECT_EQ(test, scale_w, 1000);
    KUNIT_EXPECT_EQ(test, scale_h, 1000);
    }
//
// dm_test_get_plane_scale_rotate_90_identity - Test Get plane scale rotate 90 identity
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_plane_scale_rotate_90_identity(test: *mut kunit) {
    static void dm_test_get_plane_scale_rotate_90_identity(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut scale_w: c_int = 0;
    let mut scale_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_90;
    plane_state.src_w = 1920 << 16;
    plane_state.src_h = 1080 << 16;
    plane_state.crtc_w = 1080;
    plane_state.crtc_h = 1920;
    dm_get_plane_scale(&plane_state, &scale_w, &scale_h);
    KUNIT_EXPECT_EQ(test, scale_w, 1000);
    KUNIT_EXPECT_EQ(test, scale_h, 1000);
    }
//
// dm_test_get_plane_scale_zero_src_width - Test Get plane scale zero src width
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_plane_scale_zero_src_width(test: *mut kunit) {
    static void dm_test_get_plane_scale_zero_src_width(struct kunit *test)
    {
    let mut plane_state: drm_plane_state = { 0 };
    let mut scale_w: c_int = 0;
    let mut scale_h: c_int = 0;
    plane_state.rotation = DRM_MODE_ROTATE_0;
    plane_state.src_w = 0;
    plane_state.src_h = 1080 << 16;
    plane_state.crtc_w = 100;
    plane_state.crtc_h = 200;
    dm_get_plane_scale(&plane_state, &scale_w, &scale_h);
    KUNIT_EXPECT_EQ(test, scale_w, 0);
    KUNIT_EXPECT_EQ(test, scale_h, 185);
    }
    static struct kunit_case amdgpu_dm_cursor_tests[] = {
// amdgpu_dm_should_update_native_cursor
    KUNIT_CASE(dm_test_should_update_native_cursor_without_crtc),
    KUNIT_CASE(dm_test_should_update_native_cursor_disable_native),
    KUNIT_CASE(dm_test_should_update_native_cursor_enable_overlay),
// dm_get_oriented_plane_size
    KUNIT_CASE(dm_test_oriented_plane_size_rotate_0),
    KUNIT_CASE(dm_test_oriented_plane_size_rotate_90),
    KUNIT_CASE(dm_test_oriented_plane_size_rotate_180),
    KUNIT_CASE(dm_test_oriented_plane_size_rotate_270),
// dm_get_plane_scale
    KUNIT_CASE(dm_test_get_plane_scale_identity),
    KUNIT_CASE(dm_test_get_plane_scale_rotate_90_identity),
    KUNIT_CASE(dm_test_get_plane_scale_zero_src_width),
    {}
    };
    static struct kunit_suite amdgpu_dm_cursor_test_suite = {
    .name = "amdgpu_dm_cursor",
    .test_cases = amdgpu_dm_cursor_tests,
    };
    kunit_test_suite(amdgpu_dm_cursor_test_suite);
    MODULE_AUTHOR("AMD");
    MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_cursor");
    MODULE_LICENSE("Dual MIT/GPL");
