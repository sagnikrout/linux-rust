//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_modes_test.c
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
// Kunit test for drm_modes functions
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_test_modes_priv {
    pub drm: *mut drm_device,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn drm_test_modes_init(test: *mut kunit) -> c_int {
    static int drm_test_modes_init(struct kunit *test)
    {
    struct drm_test_modes_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, priv);
    priv.dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.dev);
    priv.drm = __drm_kunit_helper_alloc_drm_device(test, priv.dev,
    sizeof(*priv.drm), 0,
    DRIVER_MODESET);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.drm);
    test.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_test_modes_analog_tv_ntsc_480i(test: *mut kunit) {
    static void drm_test_modes_analog_tv_ntsc_480i(struct kunit *test)
    {
    struct drm_test_modes_priv *priv = test.priv;
    struct drm_display_mode *mode;
    int ret;
    mode = drm_analog_tv_mode(priv.drm,
    DRM_MODE_TV_MODE_NTSC,
    13500 * HZ_PER_KHZ, 720, 480,
    true);
    KUNIT_ASSERT_NOT_NULL(test, mode);
    ret = drm_kunit_add_mode_destroy_action(test, mode);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, drm_mode_vrefresh(mode), 60);
    KUNIT_EXPECT_EQ(test, mode.hdisplay, 720);
// BT.601 defines hsync_start at 736 for 480i
    KUNIT_EXPECT_EQ(test, mode.hsync_start, 736);
//
// The NTSC standard expects a line to take 63.556us. With a
// pixel clock of 13.5 MHz, a pixel takes around 74ns, so we
// need to have 63556ns / 74ns = 858.
//
// This is also mandated by BT.601.
//
    KUNIT_EXPECT_EQ(test, mode.htotal, 858);
    KUNIT_EXPECT_EQ(test, mode.vdisplay, 480);
    KUNIT_EXPECT_EQ(test, mode.vtotal, 525);
    }
#[no_mangle]
unsafe extern "C" fn drm_test_modes_analog_tv_ntsc_480i_inlined(test: *mut kunit) {
    static void drm_test_modes_analog_tv_ntsc_480i_inlined(struct kunit *test)
    {
    struct drm_test_modes_priv *priv = test.priv;
    struct drm_display_mode *expected, *mode;
    int ret;
    expected = drm_analog_tv_mode(priv.drm,
    DRM_MODE_TV_MODE_NTSC,
    13500 * HZ_PER_KHZ, 720, 480,
    true);
    KUNIT_ASSERT_NOT_NULL(test, expected);
    ret = drm_kunit_add_mode_destroy_action(test, expected);
    KUNIT_ASSERT_EQ(test, ret, 0);
    mode = drm_mode_analog_ntsc_480i(priv.drm);
    KUNIT_ASSERT_NOT_NULL(test, mode);
    ret = drm_kunit_add_mode_destroy_action(test, mode);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, drm_mode_equal(expected, mode));
    }
#[no_mangle]
unsafe extern "C" fn drm_test_modes_analog_tv_pal_576i(test: *mut kunit) {
    static void drm_test_modes_analog_tv_pal_576i(struct kunit *test)
    {
    struct drm_test_modes_priv *priv = test.priv;
    struct drm_display_mode *mode;
    int ret;
    mode = drm_analog_tv_mode(priv.drm,
    DRM_MODE_TV_MODE_PAL,
    13500 * HZ_PER_KHZ, 720, 576,
    true);
    KUNIT_ASSERT_NOT_NULL(test, mode);
    ret = drm_kunit_add_mode_destroy_action(test, mode);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, drm_mode_vrefresh(mode), 50);
    KUNIT_EXPECT_EQ(test, mode.hdisplay, 720);
// BT.601 defines hsync_start at 732 for 576i
    KUNIT_EXPECT_EQ(test, mode.hsync_start, 732);
//
// The PAL standard expects a line to take 64us. With a pixel
// clock of 13.5 MHz, a pixel takes around 74ns, so we need to
// have 64000ns / 74ns = 864.
//
// This is also mandated by BT.601.
//
    KUNIT_EXPECT_EQ(test, mode.htotal, 864);
    KUNIT_EXPECT_EQ(test, mode.vdisplay, 576);
    KUNIT_EXPECT_EQ(test, mode.vtotal, 625);
    }
#[no_mangle]
unsafe extern "C" fn drm_test_modes_analog_tv_pal_576i_inlined(test: *mut kunit) {
    static void drm_test_modes_analog_tv_pal_576i_inlined(struct kunit *test)
    {
    struct drm_test_modes_priv *priv = test.priv;
    struct drm_display_mode *expected, *mode;
    int ret;
    expected = drm_analog_tv_mode(priv.drm,
    DRM_MODE_TV_MODE_PAL,
    13500 * HZ_PER_KHZ, 720, 576,
    true);
    KUNIT_ASSERT_NOT_NULL(test, expected);
    ret = drm_kunit_add_mode_destroy_action(test, expected);
    KUNIT_ASSERT_EQ(test, ret, 0);
    mode = drm_mode_analog_pal_576i(priv.drm);
    KUNIT_ASSERT_NOT_NULL(test, mode);
    ret = drm_kunit_add_mode_destroy_action(test, mode);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, drm_mode_equal(expected, mode));
    }
#[no_mangle]
unsafe extern "C" fn drm_test_modes_analog_tv_mono_576i(test: *mut kunit) {
    static void drm_test_modes_analog_tv_mono_576i(struct kunit *test)
    {
    struct drm_test_modes_priv *priv = test.priv;
    struct drm_display_mode *mode;
    int ret;
    mode = drm_analog_tv_mode(priv.drm,
    DRM_MODE_TV_MODE_MONOCHROME,
    13500 * HZ_PER_KHZ, 720, 576,
    true);
    KUNIT_ASSERT_NOT_NULL(test, mode);
    ret = drm_kunit_add_mode_destroy_action(test, mode);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, drm_mode_vrefresh(mode), 50);
    KUNIT_EXPECT_EQ(test, mode.hdisplay, 720);
// BT.601 defines hsync_start at 732 for 576i
    KUNIT_EXPECT_EQ(test, mode.hsync_start, 732);
//
// The PAL standard expects a line to take 64us. With a pixel
// clock of 13.5 MHz, a pixel takes around 74ns, so we need to
// have 64000ns / 74ns = 864.
//
// This is also mandated by BT.601.
//
    KUNIT_EXPECT_EQ(test, mode.htotal, 864);
    KUNIT_EXPECT_EQ(test, mode.vdisplay, 576);
    KUNIT_EXPECT_EQ(test, mode.vtotal, 625);
    }
    static struct kunit_case drm_modes_analog_tv_tests[] = {
    KUNIT_CASE(drm_test_modes_analog_tv_mono_576i),
    KUNIT_CASE(drm_test_modes_analog_tv_ntsc_480i),
    KUNIT_CASE(drm_test_modes_analog_tv_ntsc_480i_inlined),
    KUNIT_CASE(drm_test_modes_analog_tv_pal_576i),
    KUNIT_CASE(drm_test_modes_analog_tv_pal_576i_inlined),
    { }
    };
    static struct kunit_suite drm_modes_analog_tv_test_suite = {
    .name = "drm_modes_analog_tv",
    .init = drm_test_modes_init,
    .test_cases = drm_modes_analog_tv_tests,
    };
    kunit_test_suite(drm_modes_analog_tv_test_suite);
    MODULE_AUTHOR("Maxime Ripard <maxime@cerno.tech>");
    MODULE_DESCRIPTION("Kunit test for drm_modes functions");
    MODULE_LICENSE("GPL");
