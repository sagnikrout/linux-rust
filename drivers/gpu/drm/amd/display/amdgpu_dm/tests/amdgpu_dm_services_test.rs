//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_services_test.c
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
// KUnit tests for amdgpu_dm_services.c
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

// Tests for dm_get_elapse_time_in_ns()
//
// dm_test_get_elapse_time_zero_delta - Test Get elapse time zero delta
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_elapse_time_zero_delta(test: *mut kunit) {
    static void dm_test_get_elapse_time_zero_delta(struct kunit *test)
    {
    let mut ts: c_ulonglong = 1000000ULL;
    KUNIT_EXPECT_EQ(test, dm_get_elapse_time_in_ns(core::ptr::null_mut(), ts, ts), 0ULL);
    }
//
// dm_test_get_elapse_time_positive_delta - Test Get elapse time positive delta
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_elapse_time_positive_delta(test: *mut kunit) {
    static void dm_test_get_elapse_time_positive_delta(struct kunit *test)
    {
    let mut current_ts: c_ulonglong = 5000000ULL;
    let mut last_ts: c_ulonglong = 1000000ULL;
    KUNIT_EXPECT_EQ(test, dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
    4000000ULL);
    }
//
// dm_test_get_elapse_time_large_delta - Test Get elapse time large delta
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_elapse_time_large_delta(test: *mut kunit) {
    static void dm_test_get_elapse_time_large_delta(struct kunit *test)
    {
    let mut current_ts: c_ulonglong = ULLONG_MAX;
    let mut last_ts: c_ulonglong = 0ULL;
    KUNIT_EXPECT_EQ(test, dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
    ULLONG_MAX);
    }
//
// dm_test_get_elapse_time_wraparound - Test Get elapse time wraparound
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_get_elapse_time_wraparound(test: *mut kunit) {
    static void dm_test_get_elapse_time_wraparound(struct kunit *test)
    {
// Unsigned wraparound: result = ULLONG_MAX - last + current + 1
    let mut current_ts: c_ulonglong = 5ULL;
    let mut last_ts: c_ulonglong = ULLONG_MAX - 4ULL;
    KUNIT_EXPECT_EQ(test, dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
    10ULL);
    }
// Tests for dm_perf_trace_timestamp()
//
// dm_test_perf_trace_timestamp_basic - Test Perf trace timestamp basic
// @test: The KUnit test context
//
// The tracepoint is a no-op without an attached probe, so this verifies the
// function dereferences ctx->perf_trace safely and does not crash.
//
#[no_mangle]
unsafe extern "C" fn dm_test_perf_trace_timestamp_basic(test: *mut kunit) {
    static void dm_test_perf_trace_timestamp_basic(struct kunit *test)
    {
    struct dc_context *ctx;
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx);
    ctx.perf_trace = kunit_kzalloc(test, sizeof(*ctx.perf_trace), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx.perf_trace);
    ctx.perf_trace.read_count = 10;
    ctx.perf_trace.write_count = 20;
    dm_perf_trace_timestamp(__func__, __LINE__, ctx);
    }
// Tests for dm_trace_smu_enter()
//
// dm_test_trace_smu_enter_null_ctx - Test Trace smu enter null ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_trace_smu_enter_null_ctx(test: *mut kunit) {
    static void dm_test_trace_smu_enter_null_ctx(struct kunit *test)
    {
// Empty stub — must not crash with NULL ctx
    dm_trace_smu_enter(0, 0, 0, core::ptr::null_mut());
    }
//
// dm_test_trace_smu_enter_with_params - Test Trace smu enter with params
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_trace_smu_enter_with_params(test: *mut kunit) {
    static void dm_test_trace_smu_enter_with_params(struct kunit *test)
    {
// Exercise non-zero msg_id, param_in, and delay
    dm_trace_smu_enter(0xFF, 0x12345678, 1000, core::ptr::null_mut());
    }
// Tests for dm_trace_smu_exit()
//
// dm_test_trace_smu_exit_success_null_ctx - Test Trace smu exit success null ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_trace_smu_exit_success_null_ctx(test: *mut kunit) {
    static void dm_test_trace_smu_exit_success_null_ctx(struct kunit *test)
    {
// Empty stub — must not crash on success path with NULL ctx
    dm_trace_smu_exit(true, 0x0, core::ptr::null_mut());
    }
//
// dm_test_trace_smu_exit_failure_null_ctx - Test Trace smu exit failure null ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_trace_smu_exit_failure_null_ctx(test: *mut kunit) {
    static void dm_test_trace_smu_exit_failure_null_ctx(struct kunit *test)
    {
// Empty stub — must not crash on failure path with NULL ctx
    dm_trace_smu_exit(false, 0x0, core::ptr::null_mut());
    }
//
// dm_test_trace_smu_exit_with_response - Test Trace smu exit with response
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_trace_smu_exit_with_response(test: *mut kunit) {
    static void dm_test_trace_smu_exit_with_response(struct kunit *test)
    {
// Exercise non-zero response value
    dm_trace_smu_exit(true, 0xDEADBEEF, core::ptr::null_mut());
    }
// Tests for dm_query_extended_brightness_caps()
//
// dm_test_query_brightness_caps_null_ctx - Test Query brightness caps null ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_null_ctx(test: *mut kunit) {
    static void dm_test_query_brightness_caps_null_ctx(struct kunit *test)
    {
    let mut caps: dm_acpi_atif_backlight_caps = {};
    KUNIT_EXPECT_FALSE(test,
    dm_query_extended_brightness_caps(core::ptr::null_mut(), AcpiDisplayType_LCD1, &caps));
    }
//
// dm_test_query_brightness_caps_null_caps - Test Query brightness caps null caps
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_null_caps(test: *mut kunit) {
    static void dm_test_query_brightness_caps_null_caps(struct kunit *test)
    {
    let mut ctx: dc_context = {};
    ctx.driver_context = (void *)0x1; /* non-core::ptr::null_mut() sentinel */
    KUNIT_EXPECT_FALSE(test,
    dm_query_extended_brightness_caps(&ctx, AcpiDisplayType_LCD1, core::ptr::null_mut()));
    }
//
// dm_test_query_brightness_caps_null_driver_ctx - Test Query brightness caps null driver ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_null_driver_ctx(test: *mut kunit) {
    static void dm_test_query_brightness_caps_null_driver_ctx(struct kunit *test)
    {
    let mut ctx: dc_context = {};
    let mut caps: dm_acpi_atif_backlight_caps = {};
    ctx.driver_context = core::ptr::null_mut();
    KUNIT_EXPECT_FALSE(test,
    dm_query_extended_brightness_caps(&ctx, AcpiDisplayType_LCD1, &caps));
    }
//
// dm_test_query_brightness_caps_lcd2_null_ctx - Test Query brightness caps lcd2 null ctx
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_lcd2_null_ctx(test: *mut kunit) {
    static void dm_test_query_brightness_caps_lcd2_null_ctx(struct kunit *test)
    {
    let mut caps: dm_acpi_atif_backlight_caps = {};
    KUNIT_EXPECT_FALSE(test,
    dm_query_extended_brightness_caps(core::ptr::null_mut(), AcpiDisplayType_LCD2, &caps));
    }
//
// dm_test_query_brightness_caps_lcd1_success - Test Query brightness caps lcd1 success
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_lcd1_success(test: *mut kunit) {
    static void dm_test_query_brightness_caps_lcd1_success(struct kunit *test)
    {
    struct amdgpu_device *adev;
    struct amdgpu_dm_backlight_caps *source_caps;
    let mut ctx: dc_context = {};
    let mut caps: dm_acpi_atif_backlight_caps = {};
    adev = kunit_kzalloc(test, sizeof(*adev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, adev);
    source_caps = &adev.dm.backlight_caps[0];
    source_caps.caps_valid = true;
    source_caps.min_input_signal = 12;
    source_caps.max_input_signal = 240;
    source_caps.ac_level = 80;
    source_caps.dc_level = 40;
    source_caps.data_points = 2;
    source_caps.luminance_data[0].luminance = 10;
    source_caps.luminance_data[0].input_signal = 22;
    source_caps.luminance_data[1].luminance = 90;
    source_caps.luminance_data[1].input_signal = 200;
    ctx.driver_context = adev;
    KUNIT_EXPECT_TRUE(test,
    dm_query_extended_brightness_caps(&ctx, AcpiDisplayType_LCD1, &caps));
    KUNIT_EXPECT_EQ(test, caps.num_data_points, 2);
    KUNIT_EXPECT_EQ(test, caps.max_input_signal, 240);
    KUNIT_EXPECT_EQ(test, caps.min_input_signal, 12);
    KUNIT_EXPECT_EQ(test, caps.ac_level_percentage, 80);
    KUNIT_EXPECT_EQ(test, caps.dc_level_percentage, 40);
    KUNIT_EXPECT_EQ(test, caps.data_points[0].luminance, 10);
    KUNIT_EXPECT_EQ(test, caps.data_points[0].signal_level, 22);
    KUNIT_EXPECT_EQ(test, caps.data_points[1].luminance, 90);
    KUNIT_EXPECT_EQ(test, caps.data_points[1].signal_level, 200);
    }
//
// dm_test_query_brightness_caps_non_lcd1_uses_second_slot - Test Query brightness caps non lcd1 uses second slot
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_query_brightness_caps_non_lcd1_uses_second_slot(test: *mut kunit) {
    static void dm_test_query_brightness_caps_non_lcd1_uses_second_slot(struct kunit *test)
    {
    struct amdgpu_device *adev;
    struct amdgpu_dm_backlight_caps *source_caps;
    let mut ctx: dc_context = {};
    let mut caps: dm_acpi_atif_backlight_caps = {};
    adev = kunit_kzalloc(test, sizeof(*adev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, adev);
    adev.dm.backlight_caps[0].caps_valid = true;
    adev.dm.backlight_caps[0].min_input_signal = 1;
    adev.dm.backlight_caps[0].max_input_signal = 2;
    source_caps = &adev.dm.backlight_caps[1];
    source_caps.caps_valid = true;
    source_caps.min_input_signal = 33;
    source_caps.max_input_signal = 199;
    source_caps.ac_level = 70;
    source_caps.dc_level = 30;
    source_caps.data_points = 0;
    ctx.driver_context = adev;
    KUNIT_EXPECT_TRUE(test,
    dm_query_extended_brightness_caps(&ctx, AcpiDisplayType_DFP1, &caps));
    KUNIT_EXPECT_EQ(test, caps.num_data_points, 0);
    KUNIT_EXPECT_EQ(test, caps.max_input_signal, 199);
    KUNIT_EXPECT_EQ(test, caps.min_input_signal, 33);
    KUNIT_EXPECT_EQ(test, caps.ac_level_percentage, 70);
    KUNIT_EXPECT_EQ(test, caps.dc_level_percentage, 30);
    KUNIT_EXPECT_EQ(test, caps.data_points[0].luminance, 0);
    KUNIT_EXPECT_EQ(test, caps.data_points[0].signal_level, 0);
    }
    static struct kunit_case amdgpu_dm_services_test_cases[] = {
// dm_get_elapse_time_in_ns
    KUNIT_CASE(dm_test_get_elapse_time_zero_delta),
    KUNIT_CASE(dm_test_get_elapse_time_positive_delta),
    KUNIT_CASE(dm_test_get_elapse_time_large_delta),
    KUNIT_CASE(dm_test_get_elapse_time_wraparound),
// dm_perf_trace_timestamp
    KUNIT_CASE(dm_test_perf_trace_timestamp_basic),
// dm_trace_smu_enter
    KUNIT_CASE(dm_test_trace_smu_enter_null_ctx),
    KUNIT_CASE(dm_test_trace_smu_enter_with_params),
// dm_trace_smu_exit
    KUNIT_CASE(dm_test_trace_smu_exit_success_null_ctx),
    KUNIT_CASE(dm_test_trace_smu_exit_failure_null_ctx),
    KUNIT_CASE(dm_test_trace_smu_exit_with_response),
// dm_query_extended_brightness_caps
    KUNIT_CASE(dm_test_query_brightness_caps_null_ctx),
    KUNIT_CASE(dm_test_query_brightness_caps_null_caps),
    KUNIT_CASE(dm_test_query_brightness_caps_null_driver_ctx),
    KUNIT_CASE(dm_test_query_brightness_caps_lcd2_null_ctx),
    KUNIT_CASE(dm_test_query_brightness_caps_lcd1_success),
    KUNIT_CASE(dm_test_query_brightness_caps_non_lcd1_uses_second_slot),
    {}
    };
    static struct kunit_suite amdgpu_dm_services_test_suite = {
    .name = "amdgpu_dm_services",
    .test_cases = amdgpu_dm_services_test_cases,
    };
    kunit_test_suite(amdgpu_dm_services_test_suite);
    MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_services");
    MODULE_LICENSE("Dual MIT/GPL");
