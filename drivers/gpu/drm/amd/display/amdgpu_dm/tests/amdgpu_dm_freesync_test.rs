//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_freesync_test.c
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
// KUnit tests for amdgpu_dm_freesync.c
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

// Tests for amdgpu_dm_is_timing_unchanged_for_freesync()
//
// dm_test_timing_unchanged_null_args - Test NULL crtc states return false
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_timing_unchanged_null_args(test: *mut kunit) {
    static void dm_test_timing_unchanged_null_args(struct kunit *test)
    {
    let mut crtc_state: drm_crtc_state = { 0 };
    KUNIT_EXPECT_FALSE(test,
    amdgpu_dm_is_timing_unchanged_for_freesync(core::ptr::null_mut(), &crtc_state));
    KUNIT_EXPECT_FALSE(test,
    amdgpu_dm_is_timing_unchanged_for_freesync(&crtc_state, core::ptr::null_mut()));
    }
//
// dm_test_timing_unchanged_identical_modes - Test identical modes are not "unchanged"
// @test: The KUnit test context
//
// The helper only returns true when vtotal/vsync shift (vrr) while the rest
// of the timing stays fixed, so identical modes must return false.
//
#[no_mangle]
unsafe extern "C" fn dm_test_timing_unchanged_identical_modes(test: *mut kunit) {
    static void dm_test_timing_unchanged_identical_modes(struct kunit *test)
    {
    let mut old_state: drm_crtc_state = { 0 };
    let mut new_state: drm_crtc_state = { 0 };
    old_state.mode.clock = 148500;
    old_state.mode.hdisplay = 1920;
    old_state.mode.vdisplay = 1080;
    old_state.mode.htotal = 2200;
    old_state.mode.vtotal = 1125;
    new_state.mode = old_state.mode;
    KUNIT_EXPECT_FALSE(test,
    amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
    }
//
// dm_test_timing_unchanged_vrr_shift - Test vrr-style vtotal/vsync shift is detected
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_timing_unchanged_vrr_shift(test: *mut kunit) {
    static void dm_test_timing_unchanged_vrr_shift(struct kunit *test)
    {
    let mut old_state: drm_crtc_state = { 0 };
    let mut new_state: drm_crtc_state = { 0 };
    old_state.mode.clock = 148500;
    old_state.mode.hdisplay = 1920;
    old_state.mode.vdisplay = 1080;
    old_state.mode.htotal = 2200;
    old_state.mode.vtotal = 1125;
    old_state.mode.hsync_start = 2008;
    old_state.mode.vsync_start = 1084;
    old_state.mode.hsync_end = 2052;
    old_state.mode.vsync_end = 1089;
// Same horizontal timing, vertical totals/sync shifted by 125 lines
    new_state.mode = old_state.mode;
    new_state.mode.vtotal = 1250;
    new_state.mode.vsync_start = 1209;
    new_state.mode.vsync_end = 1214;
    KUNIT_EXPECT_TRUE(test,
    amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
    }
//
// dm_test_timing_unchanged_clock_changed - Test pixel clock change returns false
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_timing_unchanged_clock_changed(test: *mut kunit) {
    static void dm_test_timing_unchanged_clock_changed(struct kunit *test)
    {
    let mut old_state: drm_crtc_state = { 0 };
    let mut new_state: drm_crtc_state = { 0 };
    old_state.mode.clock = 148500;
    old_state.mode.htotal = 2200;
    old_state.mode.vtotal = 1125;
    old_state.mode.vsync_start = 1084;
    old_state.mode.vsync_end = 1089;
    new_state.mode = old_state.mode;
    new_state.mode.clock = 297000;
    new_state.mode.vtotal = 1250;
    new_state.mode.vsync_start = 1209;
    new_state.mode.vsync_end = 1214;
    KUNIT_EXPECT_FALSE(test,
    amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
    }
// Tests for amdgpu_dm_set_freesync_fixed_config()
//
// dm_test_set_freesync_fixed_config_60hz - Test fixed refresh computed for 1080p60
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_set_freesync_fixed_config_60hz(test: *mut kunit) {
    static void dm_test_set_freesync_fixed_config_60hz(struct kunit *test)
    {
    let mut dm_crtc_state: dm_crtc_state = { 0 };
    dm_crtc_state.base.mode.clock = 148500;
    dm_crtc_state.base.mode.htotal = 2200;
    dm_crtc_state.base.mode.vtotal = 1125;
    amdgpu_dm_set_freesync_fixed_config(&dm_crtc_state);
    KUNIT_EXPECT_EQ(test, (int)dm_crtc_state.freesync_config.state,
    (int)VRR_STATE_ACTIVE_FIXED);
// 148500 kHz / (2200 * 1125) = 60 Hz = 60000000 uHz
    KUNIT_EXPECT_EQ(test, dm_crtc_state.freesync_config.fixed_refresh_in_uhz,
    60000000U);
    }
// Tests for amdgpu_dm_is_dc_timing_adjust_needed()
//
// dm_test_dc_timing_adjust_pending - Test a pending hw timing adjust forces true
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_dc_timing_adjust_pending(test: *mut kunit) {
    static void dm_test_dc_timing_adjust_pending(struct kunit *test)
    {
    struct dm_crtc_state *old_state, *new_state;
    struct dc_stream_state *stream;
    old_state = kunit_kzalloc(test, sizeof(*old_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, old_state);
    new_state = kunit_kzalloc(test, sizeof(*new_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_state);
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    new_state.stream = stream;
    stream.adjust.timing_adjust_pending = 1;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_is_dc_timing_adjust_needed(old_state, new_state));
    }
//
// dm_test_dc_timing_adjust_active_fixed - Test VRR active-fixed forces true
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_dc_timing_adjust_active_fixed(test: *mut kunit) {
    static void dm_test_dc_timing_adjust_active_fixed(struct kunit *test)
    {
    struct dm_crtc_state *old_state, *new_state;
    struct dc_stream_state *stream;
    old_state = kunit_kzalloc(test, sizeof(*old_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, old_state);
    new_state = kunit_kzalloc(test, sizeof(*new_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_state);
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    new_state.stream = stream;
    new_state.freesync_config.state = VRR_STATE_ACTIVE_FIXED;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_is_dc_timing_adjust_needed(old_state, new_state));
    }
//
// dm_test_dc_timing_adjust_vrr_toggle - Test a change in vrr active state forces true
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_dc_timing_adjust_vrr_toggle(test: *mut kunit) {
    static void dm_test_dc_timing_adjust_vrr_toggle(struct kunit *test)
    {
    struct dm_crtc_state *old_state, *new_state;
    struct dc_stream_state *stream;
    old_state = kunit_kzalloc(test, sizeof(*old_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, old_state);
    new_state = kunit_kzalloc(test, sizeof(*new_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_state);
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    new_state.stream = stream;
    old_state.freesync_config.state = VRR_STATE_ACTIVE_VARIABLE;
    new_state.freesync_config.state = VRR_STATE_INACTIVE;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_is_dc_timing_adjust_needed(old_state, new_state));
    }
//
// dm_test_dc_timing_adjust_not_needed - Test steady-state timing needs no adjust
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_dc_timing_adjust_not_needed(test: *mut kunit) {
    static void dm_test_dc_timing_adjust_not_needed(struct kunit *test)
    {
    struct dm_crtc_state *old_state, *new_state;
    struct dc_stream_state *stream;
    old_state = kunit_kzalloc(test, sizeof(*old_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, old_state);
    new_state = kunit_kzalloc(test, sizeof(*new_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_state);
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    new_state.stream = stream;
    old_state.freesync_config.state = VRR_STATE_INACTIVE;
    new_state.freesync_config.state = VRR_STATE_INACTIVE;
    KUNIT_EXPECT_FALSE(test, amdgpu_dm_is_dc_timing_adjust_needed(old_state, new_state));
    }
// Tests for amdgpu_dm_get_freesync_config_for_crtc()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_test_freesync_ctx {
    pub aconnector: *mut amdgpu_dm_connector,
    pub crtc_state: *mut dm_crtc_state,
    pub conn_state: *mut dm_connector_state,
    pub stream: *mut dc_stream_state,
}

    static struct dm_test_freesync_ctx *dm_test_freesync_ctx_alloc(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx;
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx);
    ctx.aconnector = kunit_kzalloc(test, sizeof(*ctx.aconnector), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx.aconnector);
    ctx.crtc_state = kunit_kzalloc(test, sizeof(*ctx.crtc_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx.crtc_state);
    ctx.conn_state = kunit_kzalloc(test, sizeof(*ctx.conn_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx.conn_state);
    ctx.stream = dm_kunit_alloc_stream(test, core::ptr::null_mut());
    ctx.conn_state.base.connector = &ctx.aconnector.base;
    ctx.aconnector.base.connector_type = DRM_MODE_CONNECTOR_DisplayPort;
    ctx.crtc_state.stream = ctx.stream;
// 1080p60 timing so drm_mode_vrefresh() == 60
    ctx.crtc_state.base.mode.clock = 148500;
    ctx.crtc_state.base.mode.htotal = 2200;
    ctx.crtc_state.base.mode.vtotal = 1125;
    return ctx;
    }
//
// dm_test_freesync_config_writeback - Test writeback connector is left untouched
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_writeback(test: *mut kunit) {
    static void dm_test_freesync_config_writeback(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.aconnector.base.connector_type = DRM_MODE_CONNECTOR_WRITEBACK;
    ctx.conn_state.freesync_capable = true;
    ctx.aconnector.min_vfreq = 48;
    ctx.aconnector.max_vfreq = 120;
    ctx.crtc_state.vrr_supported = true;	/* sentinel: must stay set */
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
// Writeback: early return leaves vrr_supported sentinel untouched
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.vrr_supported);
    }
//
// dm_test_freesync_config_not_capable - Test a non-freesync sink reports UNSUPPORTED
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_not_capable(test: *mut kunit) {
    static void dm_test_freesync_config_not_capable(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.conn_state.freesync_capable = false;
    ctx.aconnector.min_vfreq = 48;
    ctx.aconnector.max_vfreq = 120;
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
    KUNIT_EXPECT_FALSE(test, ctx.crtc_state.vrr_supported);
    KUNIT_EXPECT_EQ(test, (int)ctx.crtc_state.freesync_config.state,
    (int)VRR_STATE_UNSUPPORTED);
    }
//
// dm_test_freesync_config_out_of_range - Test a refresh outside the range is UNSUPPORTED
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_out_of_range(test: *mut kunit) {
    static void dm_test_freesync_config_out_of_range(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.conn_state.freesync_capable = true;
    ctx.aconnector.min_vfreq = 90;	/* 60 < 90 . out of range */
    ctx.aconnector.max_vfreq = 120;
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
    KUNIT_EXPECT_FALSE(test, ctx.crtc_state.vrr_supported);
    KUNIT_EXPECT_EQ(test, (int)ctx.crtc_state.freesync_config.state,
    (int)VRR_STATE_UNSUPPORTED);
    }
//
// dm_test_freesync_config_active_variable - Test vrr_enabled yields ACTIVE_VARIABLE
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_active_variable(test: *mut kunit) {
    static void dm_test_freesync_config_active_variable(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.conn_state.freesync_capable = true;
    ctx.aconnector.min_vfreq = 48;
    ctx.aconnector.max_vfreq = 120;
    ctx.crtc_state.base.vrr_enabled = true;
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.vrr_supported);
    KUNIT_EXPECT_TRUE(test, ctx.stream.ignore_msa_timing_param);
    KUNIT_EXPECT_EQ(test, (int)ctx.crtc_state.freesync_config.state,
    (int)VRR_STATE_ACTIVE_VARIABLE);
    KUNIT_EXPECT_EQ(test, ctx.crtc_state.freesync_config.min_refresh_in_uhz,
    48000000U);
    KUNIT_EXPECT_EQ(test, ctx.crtc_state.freesync_config.max_refresh_in_uhz,
    120000000U);
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.freesync_config.vsif_supported);
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.freesync_config.btr);
    }
//
// dm_test_freesync_config_inactive - Test supported-but-off yields INACTIVE
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_inactive(test: *mut kunit) {
    static void dm_test_freesync_config_inactive(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.conn_state.freesync_capable = true;
    ctx.aconnector.min_vfreq = 48;
    ctx.aconnector.max_vfreq = 120;
    ctx.crtc_state.base.vrr_enabled = false;
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.vrr_supported);
    KUNIT_EXPECT_EQ(test, (int)ctx.crtc_state.freesync_config.state,
    (int)VRR_STATE_INACTIVE);
    }
//
// dm_test_freesync_config_active_fixed - Test freesync-video mode yields ACTIVE_FIXED
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_freesync_config_active_fixed(test: *mut kunit) {
    static void dm_test_freesync_config_active_fixed(struct kunit *test)
    {
    struct dm_test_freesync_ctx *ctx = dm_test_freesync_ctx_alloc(test);
    ctx.conn_state.freesync_capable = true;
    ctx.aconnector.min_vfreq = 48;
    ctx.aconnector.max_vfreq = 120;
// Pre-set fixed state selects the freesync-video (fixed) path
    ctx.crtc_state.freesync_config.state = VRR_STATE_ACTIVE_FIXED;
    ctx.crtc_state.freesync_config.fixed_refresh_in_uhz = 60000000;
    ctx.crtc_state.base.vrr_enabled = true;	/* ignored on the fixed path */
    amdgpu_dm_get_freesync_config_for_crtc(ctx.crtc_state, ctx.conn_state);
    KUNIT_EXPECT_TRUE(test, ctx.crtc_state.vrr_supported);
    KUNIT_EXPECT_EQ(test, (int)ctx.crtc_state.freesync_config.state,
    (int)VRR_STATE_ACTIVE_FIXED);
    KUNIT_EXPECT_EQ(test, ctx.crtc_state.freesync_config.fixed_refresh_in_uhz,
    60000000U);
    }
// Tests for amdgpu_dm_reset_freesync_config_for_crtc()
//
// dm_test_reset_freesync_config - Test reset clears vrr support and info packet
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_reset_freesync_config(test: *mut kunit) {
    static void dm_test_reset_freesync_config(struct kunit *test)
    {
    struct dm_crtc_state *crtc_state;
    crtc_state = kunit_kzalloc(test, sizeof(*crtc_state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, crtc_state);
    crtc_state.vrr_supported = true;
    crtc_state.vrr_infopacket.valid = true;
    amdgpu_dm_reset_freesync_config_for_crtc(crtc_state);
    KUNIT_EXPECT_FALSE(test, crtc_state.vrr_supported);
    KUNIT_EXPECT_FALSE(test, crtc_state.vrr_infopacket.valid);
    }
    static struct kunit_case amdgpu_dm_freesync_tests[] = {
// amdgpu_dm_is_timing_unchanged_for_freesync
    KUNIT_CASE(dm_test_timing_unchanged_null_args),
    KUNIT_CASE(dm_test_timing_unchanged_identical_modes),
    KUNIT_CASE(dm_test_timing_unchanged_vrr_shift),
    KUNIT_CASE(dm_test_timing_unchanged_clock_changed),
// amdgpu_dm_set_freesync_fixed_config
    KUNIT_CASE(dm_test_set_freesync_fixed_config_60hz),
// amdgpu_dm_is_dc_timing_adjust_needed
    KUNIT_CASE(dm_test_dc_timing_adjust_pending),
    KUNIT_CASE(dm_test_dc_timing_adjust_active_fixed),
    KUNIT_CASE(dm_test_dc_timing_adjust_vrr_toggle),
    KUNIT_CASE(dm_test_dc_timing_adjust_not_needed),
// amdgpu_dm_get_freesync_config_for_crtc
    KUNIT_CASE(dm_test_freesync_config_writeback),
    KUNIT_CASE(dm_test_freesync_config_not_capable),
    KUNIT_CASE(dm_test_freesync_config_out_of_range),
    KUNIT_CASE(dm_test_freesync_config_active_variable),
    KUNIT_CASE(dm_test_freesync_config_inactive),
    KUNIT_CASE(dm_test_freesync_config_active_fixed),
// amdgpu_dm_reset_freesync_config_for_crtc
    KUNIT_CASE(dm_test_reset_freesync_config),
    {}
    };
    static struct kunit_suite amdgpu_dm_freesync_test_suite = {
    .name = "amdgpu_dm_freesync",
    .test_cases = amdgpu_dm_freesync_tests,
    };
    kunit_test_suite(amdgpu_dm_freesync_test_suite);
    MODULE_AUTHOR("AMD");
    MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_freesync");
    MODULE_LICENSE("Dual MIT/GPL");
