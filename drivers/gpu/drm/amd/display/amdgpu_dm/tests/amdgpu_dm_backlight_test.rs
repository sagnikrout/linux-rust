//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_backlight_test.c
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
// KUnit tests for amdgpu_dm_backlight.c
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_backlight_connector_fixture {
    pub adev: *mut amdgpu_device,
    pub aconnector: *mut amdgpu_dm_connector,
    pub link: *mut dc_link,
}

    static const struct drm_connector_funcs dm_backlight_test_connector_funcs = {
    .reset = drm_atomic_helper_connector_reset,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
    static void setup_test_connector(struct kunit *test,
    struct dm_backlight_connector_fixture *fixture,
    int bl_idx, enum signal_type signal)
    {
    fixture.adev = kunit_kzalloc(test, sizeof(*fixture.adev), GFP_KERNEL);
    fixture.aconnector = kunit_kzalloc(test, sizeof(*fixture.aconnector), GFP_KERNEL);
    fixture.link = kunit_kzalloc(test, sizeof(*fixture.link), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fixture.adev);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fixture.aconnector);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fixture.link);
    fixture.aconnector.bl_idx = bl_idx;
    fixture.aconnector.dc_link = fixture.link;
    fixture.aconnector.base.dev = &fixture.adev.ddev;
    fixture.link.connector_signal = signal;
    }
#[no_mangle]
unsafe extern "C" fn setup_test_dm_ddev(test: *mut kunit, dm: *mut amdgpu_display_manager) {
    static void setup_test_dm_ddev(struct kunit *test, struct amdgpu_display_manager *dm)
    {
    dm.ddev = dm_kunit_alloc_drm_with_connector_list(test);
    }
// Tests for dm_find_stream_with_link()
//
// dm_test_find_stream_with_link_returns_match - Test matching stream lookup
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_find_stream_with_link_returns_match(test: *mut kunit) {
    static void dm_test_find_stream_with_link_returns_match(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *other_link = dm_kunit_alloc_link(test);
    struct dc_link *target_link = dm_kunit_alloc_link(test);
    struct dc_stream_state *stream;
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 0, other_link);
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 1, target_link);
    stream = dm_find_stream_with_link(dm, target_link);
    KUNIT_ASSERT_NOT_NULL(test, stream);
    KUNIT_EXPECT_PTR_EQ(test, stream.link, target_link);
    }
//
// dm_test_find_stream_with_link_missing - Test missing stream lookup
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_find_stream_with_link_missing(test: *mut kunit) {
    static void dm_test_find_stream_with_link_missing(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *stream_link = dm_kunit_alloc_link(test);
    struct dc_link *missing_link = dm_kunit_alloc_link(test);
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 0, stream_link);
    KUNIT_EXPECT_NULL(test, dm_find_stream_with_link(dm, missing_link));
    }
// Tests for amdgpu_dm_backlight_set_level()
//
// dm_test_backlight_set_level_connector_off - Test connector-off cache path
// @test: The KUnit test context
//
// If the matching connector has no encoder, set_level() must cache the
// requested brightness and return before touching DC or backlight hardware.
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_set_level_connector_off(test: *mut kunit) {
    static void dm_test_backlight_set_level_connector_off(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct amdgpu_dm_connector *aconnector;
    setup_test_dm_ddev(test, dm);
    aconnector = kunit_kzalloc(test, sizeof(*aconnector), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, aconnector);
    INIT_LIST_HEAD(&aconnector.base.head);
    aconnector.bl_idx = 1;
    aconnector.base.encoder = core::ptr::null_mut();
    list_add_tail(&aconnector.base.head, &dm.ddev.mode_config.connector_list);
    amdgpu_dm_backlight_set_level(dm, 1, 1234);
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 1234U);
    KUNIT_EXPECT_EQ(test, dm.actual_brightness[1], 0U);
    }
//
// dm_test_backlight_set_level_no_stream - Test no-stream early return
// @test: The KUnit test context
//
// With no stream for the backlight link, set_level() records the requested
// brightness and exits before calling the power-module programming path.
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_set_level_no_stream(test: *mut kunit) {
    static void dm_test_backlight_set_level_no_stream(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    setup_test_dm_ddev(test, dm);
    dm.backlight_caps[1].caps_valid = true;
    dm.backlight_caps[1].min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    dm.backlight_caps[1].max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    dm.backlight_link[1] = link;
    amdgpu_dm_backlight_set_level(dm, 1, 2000);
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 2000U);
    KUNIT_EXPECT_EQ(test, dm.actual_brightness[1], 0U);
    }
//
// dm_test_backlight_set_level_aux_programs_power_module - Test AUX programming path
// @test: The KUnit test context
//
// With a matching stream present, set_level() walks into the DC programming
// path. A NULL power_module makes mod_power_set_backlight_nits() a safe
// early-false, and ips_support disabled leaves idle optimizations untouched.
// A non-matching connector exercises the connector-list skip, and a non-zero
// brightness_mask exercises the quirk-OR path.
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_set_level_aux_programs_power_module(test: *mut kunit) {
    static void dm_test_backlight_set_level_aux_programs_power_module(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct amdgpu_dm_connector *other;
    setup_test_dm_ddev(test, dm);
    mutex_init(&dm.dc_lock);
    dm.power_module = core::ptr::null_mut();
// Non-matching connector exercises the bl_idx skip (continue).
    other = kunit_kzalloc(test, sizeof(*other), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, other);
    INIT_LIST_HEAD(&other.base.head);
    other.bl_idx = 0;
    list_add_tail(&other.base.head, &dm.ddev.mode_config.connector_list);
    dm.backlight_caps[1].caps_valid = true;
    dm.backlight_caps[1].aux_support = true;
    dm.backlight_caps[1].brightness_mask = 0x3;
    dm.backlight_caps[1].aux_min_input_signal = 1;
    dm.backlight_caps[1].aux_max_input_signal = 512;
    dm.backlight_caps[1].min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    dm.backlight_caps[1].max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    dm.backlight_link[1] = link;
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 0, link);
    amdgpu_dm_backlight_set_level(dm, 1, 2000);
// power_module is NULL so programming fails; actual stays unchanged.
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 2000U);
    KUNIT_EXPECT_EQ(test, dm.actual_brightness[1], 0U);
    }
//
// dm_test_backlight_set_level_pwm_programs_power_module - Test PWM programming path
// @test: The KUnit test context
//
// With aux_support cleared, set_level() takes the millipercent branch:
// get_brightness_range() + mod_power_set_backlight_percent(). A NULL
// power_module keeps the call a safe early-false.
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_set_level_pwm_programs_power_module(test: *mut kunit) {
    static void dm_test_backlight_set_level_pwm_programs_power_module(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    setup_test_dm_ddev(test, dm);
    mutex_init(&dm.dc_lock);
    dm.power_module = core::ptr::null_mut();
    dm.backlight_caps[1].caps_valid = true;
    dm.backlight_caps[1].aux_support = false;
    dm.backlight_caps[1].min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    dm.backlight_caps[1].max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    dm.backlight_link[1] = link;
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 0, link);
    amdgpu_dm_backlight_set_level(dm, 1, 2000);
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 2000U);
    KUNIT_EXPECT_EQ(test, dm.actual_brightness[1], 0U);
    }
//
// dm_test_backlight_set_level_reallows_idle - Test idle-optimization toggle path
// @test: The KUnit test context
//
// When ips_support is set and dmub idle is allowed, set_level() disables idle
// optimizations around the programming call and re-enables them afterwards.
// disable_idle_power_optimizations keeps dc_allow_idle_optimizations() a safe
// early return, and ctx->logger is wired because DC_LOG_* dereferences it.
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_set_level_reallows_idle(test: *mut kunit) {
    static void dm_test_backlight_set_level_reallows_idle(struct kunit *test)
    {
    struct amdgpu_device *adev = dm_kunit_alloc_adev(test);
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct dc_dmub_srv *dmub_srv;
    struct dal_logger *logger;
    struct dc_context *ctx;
    setup_test_dm_ddev(test, dm);
    mutex_init(&dm.dc_lock);
    dm.power_module = core::ptr::null_mut();
// dm_kunit_alloc_dm() leaves dc->ctx NULL; the idle path dereferences it.
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    dm.dc.ctx = ctx;
    logger = kunit_kzalloc(test, sizeof(*logger), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, logger);
    logger.dev = &adev.ddev;
    dm.dc.ctx.logger = logger;
    dmub_srv = kunit_kzalloc(test, sizeof(*dmub_srv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dmub_srv);
    dmub_srv.idle_allowed = true;
    dm.dc.ctx.dmub_srv = dmub_srv;
    dm.dc.caps.ips_support = true;
// Keep dc_allow_idle_optimizations() a safe early return.
    dm.dc.debug.disable_idle_power_optimizations = true;
    dm.backlight_caps[1].caps_valid = true;
    dm.backlight_caps[1].aux_support = true;
    dm.backlight_caps[1].aux_min_input_signal = 1;
    dm.backlight_caps[1].aux_max_input_signal = 512;
    dm.backlight_caps[1].min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    dm.backlight_caps[1].max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    dm.backlight_link[1] = link;
    dm_kunit_add_stream_to_state(test, dm.dc.current_state, 0, link);
    amdgpu_dm_backlight_set_level(dm, 1, 2000);
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 2000U);
    }
//
// dm_test_backlight_update_status_no_stream - Test update_status wrapper
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_update_status_no_stream(test: *mut kunit) {
    static void dm_test_backlight_update_status_no_stream(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct backlight_device *bd;
    setup_test_dm_ddev(test, dm);
    bd = kunit_kzalloc(test, sizeof(*bd), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bd);
    dev_set_drvdata(&bd.dev, dm);
    bd.props.brightness = 3456;
    dm.num_of_edps = 2;
    dm.backlight_dev[1] = bd;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_update_status(bd), 0);
    KUNIT_EXPECT_EQ(test, dm.brightness[1], 3456U);
    }
#[no_mangle]
unsafe extern "C" fn setup_test_link_service(test: *mut kunit, link: *mut dc_link) {
    static void setup_test_link_service(struct kunit *test, struct dc_link *link)
    {
    struct link_service *link_srv;
    struct dc_context *ctx;
    struct dc *dc;
    dc = kunit_kzalloc(test, sizeof(*dc), GFP_KERNEL);
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    link_srv = kunit_kzalloc(test, sizeof(*link_srv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dc);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, link_srv);
    dc.ctx = ctx;
    dc.link_srv = link_srv;
    ctx.dc = dc;
    link.dc = dc;
    link.ctx = ctx;
    }
#[no_mangle]
unsafe extern "C" fn dm_test_get_backlight_level_mid(link: *const dc_link) -> c_int {
    static int dm_test_get_backlight_level_mid(const struct dc_link *link)
    {
    return (0x101 * AMDGPU_DM_DEFAULT_MIN_BACKLIGHT) + 1000;
    }
#[no_mangle]
unsafe extern "C" fn dm_test_get_backlight_level_error(link: *const dc_link) -> c_int {
    static int dm_test_get_backlight_level_error(const struct dc_link *link)
    {
    return DC_ERROR_UNEXPECTED;
    }
#[no_mangle]
unsafe extern "C" fn dm_test_unregister_backlight_device(data: *mut c_void) {
    static void dm_test_unregister_backlight_device(void *data)
    {
    backlight_device_unregister(data);
    }
    static bool dm_test_get_backlight_level_nits(struct dc_link *link,
    uint32_t *avg,
    uint32_t *peak)
    {
// avg = 250000;
// peak = 300000;
    return true;
    }
    static bool dm_test_get_backlight_level_nits_fail(struct dc_link *link,
    uint32_t *avg,
    uint32_t *peak)
    {
    return false;
    }
// Tests for amdgpu_dm_backlight_get_level()/get_brightness()
//
// dm_test_backlight_get_level_pwm_success - Test PWM brightness readback
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_get_level_pwm_success(test: *mut kunit) {
    static void dm_test_backlight_get_level_pwm_success(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    let mut hw_level: u32 = dm_test_get_backlight_level_mid(link);
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level = dm_test_get_backlight_level_mid;
    dm.backlight_link[0] = link;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_level(dm, 0),
    convert_brightness_to_user(&dm.backlight_caps[0], hw_level));
    }
//
// dm_test_backlight_get_level_pwm_error - Test PWM readback fallback
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_get_level_pwm_error(test: *mut kunit) {
    static void dm_test_backlight_get_level_pwm_error(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level = dm_test_get_backlight_level_error;
    dm.brightness[0] = 4321;
    dm.backlight_link[0] = link;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_level(dm, 0), 4321U);
    }
//
// dm_test_backlight_get_level_aux_success - Test AUX brightness readback
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_get_level_aux_success(test: *mut kunit) {
    static void dm_test_backlight_get_level_aux_success(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct amdgpu_dm_backlight_caps *caps = &dm.backlight_caps[0];
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level_nits = dm_test_get_backlight_level_nits;
    dm.backlight_link[0] = link;
    caps.caps_valid = true;
    caps.aux_support = true;
    caps.aux_min_input_signal = 1;
    caps.aux_max_input_signal = 512;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_level(dm, 0),
    convert_brightness_to_user(caps, 250000));
    }
//
// dm_test_backlight_get_level_aux_error - Test AUX readback fallback
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_get_level_aux_error(test: *mut kunit) {
    static void dm_test_backlight_get_level_aux_error(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct amdgpu_dm_backlight_caps *caps = &dm.backlight_caps[0];
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level_nits = dm_test_get_backlight_level_nits_fail;
    dm.brightness[0] = 6789;
    dm.backlight_link[0] = link;
    caps.caps_valid = true;
    caps.aux_support = true;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_level(dm, 0), 6789U);
    }
//
// dm_test_backlight_get_brightness_uses_device_index - Test get_brightness wrapper
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_get_brightness_uses_device_index(test: *mut kunit) {
    static void dm_test_backlight_get_brightness_uses_device_index(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct backlight_device *bd;
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level = dm_test_get_backlight_level_error;
    bd = kunit_kzalloc(test, sizeof(*bd), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bd);
    dev_set_drvdata(&bd.dev, dm);
    dm.num_of_edps = 2;
    dm.backlight_dev[1] = bd;
    dm.brightness[1] = 2468;
    dm.backlight_link[1] = link;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_brightness(bd), 2468);
    }
// Tests for amdgpu_dm_register_backlight_device()
//
// dm_test_register_backlight_device_negative_index - Test invalid index no-op
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_register_backlight_device_negative_index(test: *mut kunit) {
    static void dm_test_register_backlight_device_negative_index(struct kunit *test)
    {
    struct amdgpu_device *adev = dm_kunit_alloc_adev(test);
    struct amdgpu_dm_connector *aconnector;
    aconnector = dm_kunit_alloc_connector(test, adev, core::ptr::null_mut());
    aconnector.bl_idx = -1;
    amdgpu_dm_register_backlight_device(aconnector);
    KUNIT_EXPECT_NULL(test, adev.dm.backlight_dev[0]);
    }
//
// dm_test_register_backlight_device_success - Test native backlight registration
// @test: The KUnit test context
//
// A native backlight device must be registered with the calculated
// properties, and its initial readback must retain the initial brightness.
//
#[no_mangle]
unsafe extern "C" fn dm_test_register_backlight_device_success(test: *mut kunit) {
    static void dm_test_register_backlight_device_success(struct kunit *test)
    {
    struct amdgpu_device *adev = dm_kunit_alloc_adev(test);
    struct amdgpu_dm_connector *aconnector;
    struct amdgpu_dm_backlight_caps *caps;
    struct dc_link *link = dm_kunit_alloc_link(test);
    struct drm_minor *primary;
    unsigned int max;
    setup_test_link_service(test, link);
    link.dc.link_srv.edp_get_backlight_level = dm_test_get_backlight_level_error;
    primary = kunit_kzalloc(test, sizeof(*primary), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, primary);
    adev.ddev.primary = primary;
    adev.dm.backlight_link[0] = link;
    aconnector = dm_kunit_alloc_connector(test, adev, link);
    aconnector.bl_idx = 0;
    aconnector.base.kdev = adev.ddev.dev;
    caps = &adev.dm.backlight_caps[0];
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.ac_level = 50;
    caps.dc_level = 25;
    max = 0x101 * AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    amdgpu_dm_register_backlight_device(aconnector);
    KUNIT_ASSERT_NOT_NULL(test, adev.dm.backlight_dev[0]);
    KUNIT_EXPECT_EQ(test, adev.dm.backlight_dev[0].props.max_brightness, max);
    KUNIT_EXPECT_EQ(test, adev.dm.backlight_dev[0].props.brightness,
    DIV_ROUND_CLOSEST(max * caps.ac_level, 100));
    KUNIT_EXPECT_EQ(test, adev.dm.brightness[0],
    adev.dm.backlight_dev[0].props.brightness);
    KUNIT_ASSERT_EQ(test, kunit_add_action_or_reset(test,
    dm_test_unregister_backlight_device,
    adev.dm.backlight_dev[0]), 0);
    }
    static struct drm_connector *setup_panel_power_savings_connector(struct kunit *test,
    struct device **device_out,
    struct dm_connector_state **state_out)
    {
    struct dm_connector_state *state;
    struct drm_connector *connector;
    struct amdgpu_device *adev;
    struct device *device;
    int ret;
    adev = dm_kunit_alloc_adev(test);
    ret = drmm_mode_config_init(&adev.ddev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    connector = kunit_kzalloc(test, sizeof(*connector), GFP_KERNEL);
    device = kunit_kzalloc(test, sizeof(*device), GFP_KERNEL);
    state = kunit_kzalloc(test, sizeof(*state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, connector);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, device);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    connector.dev = &adev.ddev;
    connector.state = &state.base;
    dev_set_drvdata(device, connector);
// device_out = device;
// state_out = state;
    return connector;
    }
#[no_mangle]
unsafe extern "C" fn dm_test_free_sysfs_buf(data: *mut c_void) {
    static void dm_test_free_sysfs_buf(void *data)
    {
    free_page((unsigned long)data);
    }
    static char *dm_test_alloc_sysfs_buf(struct kunit *test)
    {
    char *buf;
    buf = (char *)get_zeroed_page(GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, buf);
    KUNIT_ASSERT_EQ(test, kunit_add_action_or_reset(test, dm_test_free_sysfs_buf, buf), 0);
    return buf;
    }
// Tests for panel_power_savings_show()/panel_power_savings_store()
//
// dm_test_panel_power_savings_show_maps_disable_to_zero - Test show output
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_show_maps_disable_to_zero(test: *mut kunit) {
    static void dm_test_panel_power_savings_show_maps_disable_to_zero(struct kunit *test)
    {
    struct dm_connector_state *state;
    struct device *device;
    char *buf;
    setup_panel_power_savings_connector(test, &device, &state);
    buf = dm_test_alloc_sysfs_buf(test);
    state.abm_level = ABM_LEVEL_IMMEDIATE_DISABLE;
    KUNIT_EXPECT_EQ(test, panel_power_savings_show(device, core::ptr::null_mut(), buf), 2);
    KUNIT_EXPECT_STREQ(test, buf, "0\n");
    }
//
// dm_test_panel_power_savings_show_reports_level - Test show output for active level
// @test: The KUnit test context
//
// When abm_level is not the immediate-disable sentinel, show() reports the
// raw level value.
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_show_reports_level(test: *mut kunit) {
    static void dm_test_panel_power_savings_show_reports_level(struct kunit *test)
    {
    struct dm_connector_state *state;
    struct device *device;
    char *buf;
    setup_panel_power_savings_connector(test, &device, &state);
    buf = dm_test_alloc_sysfs_buf(test);
    state.abm_level = 3;
    KUNIT_EXPECT_EQ(test, panel_power_savings_show(device, core::ptr::null_mut(), buf), 2);
    KUNIT_EXPECT_STREQ(test, buf, "3\n");
    }
//
// dm_test_panel_power_savings_store_sets_disable - Test zero maps to disable
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_store_sets_disable(test: *mut kunit) {
    static void dm_test_panel_power_savings_store_sets_disable(struct kunit *test)
    {
    struct dm_connector_state *state;
    struct device *device;
    let mut count: usize = strlen("0");
    setup_panel_power_savings_connector(test, &device, &state);
    KUNIT_EXPECT_EQ(test, panel_power_savings_store(device, core::ptr::null_mut(), "0", count),
    (ssize_t)count);
    KUNIT_EXPECT_EQ(test, state.abm_level, ABM_LEVEL_IMMEDIATE_DISABLE);
    }
//
// dm_test_panel_power_savings_store_forbidden - Test forbidden update
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_store_forbidden(test: *mut kunit) {
    static void dm_test_panel_power_savings_store_forbidden(struct kunit *test)
    {
    struct dm_connector_state *state;
    struct device *device;
    setup_panel_power_savings_connector(test, &device, &state);
    state.abm_sysfs_forbidden = true;
    KUNIT_EXPECT_EQ(test, panel_power_savings_store(device, core::ptr::null_mut(), "1", 1), -EBUSY);
    }
//
// dm_test_panel_power_savings_store_rejects_invalid_text - Test parse failure
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_store_rejects_invalid_text(test: *mut kunit) {
    static void dm_test_panel_power_savings_store_rejects_invalid_text(struct kunit *test)
    {
    struct drm_connector *connector;
    struct drm_device *drm;
    struct device *device;
    connector = kunit_kzalloc(test, sizeof(*connector), GFP_KERNEL);
    drm = kunit_kzalloc(test, sizeof(*drm), GFP_KERNEL);
    device = kunit_kzalloc(test, sizeof(*device), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, connector);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, drm);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, device);
    connector.dev = drm;
    dev_set_drvdata(device, connector);
    KUNIT_EXPECT_LT(test, panel_power_savings_store(device, core::ptr::null_mut(), "bad", 3), 0);
    }
//
// dm_test_panel_power_savings_store_rejects_out_of_range - Test range failure
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_panel_power_savings_store_rejects_out_of_range(test: *mut kunit) {
    static void dm_test_panel_power_savings_store_rejects_out_of_range(struct kunit *test)
    {
    struct drm_connector *connector;
    struct drm_device *drm;
    struct device *device;
    connector = kunit_kzalloc(test, sizeof(*connector), GFP_KERNEL);
    drm = kunit_kzalloc(test, sizeof(*drm), GFP_KERNEL);
    device = kunit_kzalloc(test, sizeof(*device), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, connector);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, drm);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, device);
    connector.dev = drm;
    dev_set_drvdata(device, connector);
    KUNIT_EXPECT_EQ(test, panel_power_savings_store(device, core::ptr::null_mut(), "5", 1), -EINVAL);
    }
// Tests for amdgpu_dm_backlight_get_device_index()
//
// dm_test_backlight_device_index_matches_second - Test matching second backlight device
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_device_index_matches_second(test: *mut kunit) {
    static void dm_test_backlight_device_index_matches_second(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct backlight_device *bd0;
    struct backlight_device *bd1;
    bd0 = kunit_kzalloc(test, sizeof(*bd0), GFP_KERNEL);
    bd1 = kunit_kzalloc(test, sizeof(*bd1), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, bd0);
    KUNIT_ASSERT_NOT_NULL(test, bd1);
    dm.num_of_edps = 2;
    dm.backlight_dev[0] = bd0;
    dm.backlight_dev[1] = bd1;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_device_index(dm, bd1), 1);
    }
//
// dm_test_backlight_device_index_missing_fallback - Test missing backlight device fallback
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_device_index_missing_fallback(test: *mut kunit) {
    static void dm_test_backlight_device_index_missing_fallback(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct backlight_device *known_bd;
    struct backlight_device *unknown_bd;
    known_bd = kunit_kzalloc(test, sizeof(*known_bd), GFP_KERNEL);
    unknown_bd = kunit_kzalloc(test, sizeof(*unknown_bd), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, known_bd);
    KUNIT_ASSERT_NOT_NULL(test, unknown_bd);
    dm.num_of_edps = 1;
    dm.backlight_dev[0] = known_bd;
    KUNIT_EXPECT_EQ(test, amdgpu_dm_backlight_get_device_index(dm, unknown_bd), 0);
    }
// Tests for amdgpu_dm_update_backlight_caps()
//
// dm_test_backlight_caps_valid_short_circuit - Test Backlight caps valid short circuit
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_caps_valid_short_circuit(test: *mut kunit) {
    static void dm_test_backlight_caps_valid_short_circuit(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct amdgpu_dm_backlight_caps *caps = &dm.backlight_caps[0];
    caps.caps_valid = true;
    caps.aux_support = false;
    caps.min_input_signal = 42;
    caps.max_input_signal = 199;
    amdgpu_dm_update_backlight_caps(dm, 0);
    KUNIT_EXPECT_TRUE(test, caps.caps_valid);
    KUNIT_EXPECT_EQ(test, caps.min_input_signal, 42);
    KUNIT_EXPECT_EQ(test, caps.max_input_signal, 199);
    }

//
// dm_test_backlight_caps_aux_support_noop - Test Backlight caps aux support noop
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_caps_aux_support_noop(test: *mut kunit) {
    static void dm_test_backlight_caps_aux_support_noop(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct amdgpu_dm_backlight_caps *caps = &dm.backlight_caps[0];
    caps.caps_valid = false;
    caps.aux_support = true;
    caps.min_input_signal = 11;
    caps.max_input_signal = 222;
    amdgpu_dm_update_backlight_caps(dm, 0);
    KUNIT_EXPECT_FALSE(test, caps.caps_valid);
    KUNIT_EXPECT_EQ(test, caps.min_input_signal, 11);
    KUNIT_EXPECT_EQ(test, caps.max_input_signal, 222);
    }
//
// dm_test_backlight_caps_non_aux_sets_defaults - Test Backlight caps non aux sets defaults
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_caps_non_aux_sets_defaults(test: *mut kunit) {
    static void dm_test_backlight_caps_non_aux_sets_defaults(struct kunit *test)
    {
    struct amdgpu_display_manager *dm = dm_kunit_alloc_dm(test);
    struct amdgpu_dm_backlight_caps *caps = &dm.backlight_caps[0];
    caps.caps_valid = false;
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = 0;
    amdgpu_dm_update_backlight_caps(dm, 0);
    KUNIT_EXPECT_TRUE(test, caps.caps_valid);
    KUNIT_EXPECT_EQ(test, caps.min_input_signal, AMDGPU_DM_DEFAULT_MIN_BACKLIGHT);
    KUNIT_EXPECT_EQ(test, caps.max_input_signal, AMDGPU_DM_DEFAULT_MAX_BACKLIGHT);
    }

// Tests for get_brightness_range()
//
// dm_test_brightness_range_null_caps - Test Brightness range null caps
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_range_null_caps(test: *mut kunit) {
    static void dm_test_brightness_range_null_caps(struct kunit *test)
    {
    let mut min: c_uint = 99, max = 99;
    KUNIT_EXPECT_EQ(test, get_brightness_range(core::ptr::null_mut(), &min, &max), 0);
// min/max should remain untouched
    KUNIT_EXPECT_EQ(test, min, 99U);
    KUNIT_EXPECT_EQ(test, max, 99U);
    }
//
// dm_test_brightness_range_pwm - Test Brightness range pwm
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_range_pwm(test: *mut kunit) {
    static void dm_test_brightness_range_pwm(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    KUNIT_EXPECT_EQ(test, get_brightness_range(&caps, &min, &max), 1);
// 0x101 * AMDGPU_DM_DEFAULT_MIN_BACKLIGHT, 0x101 * AMDGPU_DM_DEFAULT_MAX_BACKLIGHT
    KUNIT_EXPECT_EQ(test, min, 0x101U * AMDGPU_DM_DEFAULT_MIN_BACKLIGHT);
    KUNIT_EXPECT_EQ(test, max, 0x101U * AMDGPU_DM_DEFAULT_MAX_BACKLIGHT);
    }
//
// dm_test_brightness_range_aux - Test Brightness range aux
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_range_aux(test: *mut kunit) {
    static void dm_test_brightness_range_aux(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = true;
    caps.aux_min_input_signal = 1;
    caps.aux_max_input_signal = 512;
    KUNIT_EXPECT_EQ(test, get_brightness_range(&caps, &min, &max), 1);
// millinits: 1000 * value
    KUNIT_EXPECT_EQ(test, min, 1000U);
    KUNIT_EXPECT_EQ(test, max, 512000U);
    }
// Tests for convert_brightness_to_user()
//
// dm_test_brightness_to_user_null_caps - Test Brightness to user null caps
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_null_caps(test: *mut kunit) {
    static void dm_test_brightness_to_user_null_caps(struct kunit *test)
    {
//
// With NULL caps, get_brightness_range fails → passthrough.
// We simulate this by passing a zeroed caps struct where
// max_input_signal=0 makes max=0 and the function hits
// get_brightness_range returning 0 since caps is NULL.
//
    KUNIT_EXPECT_EQ(test, convert_brightness_to_user(core::ptr::null_mut(), 42), 42U);
    }
//
// dm_test_brightness_to_user_below_min - Test Brightness to user below min
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_below_min(test: *mut kunit) {
    static void dm_test_brightness_to_user_below_min(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
// brightness < min (0x101*AMDGPU_DM_DEFAULT_MIN_BACKLIGHT), should return 0
    KUNIT_EXPECT_EQ(test, convert_brightness_to_user(&caps, 100), 0U);
    }
//
// dm_test_brightness_to_user_at_max - Test Brightness to user at max
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_at_max(test: *mut kunit) {
    static void dm_test_brightness_to_user_at_max(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    get_brightness_range(&caps, &min, &max);
// At max → should return max
    KUNIT_EXPECT_EQ(test, convert_brightness_to_user(&caps, max), max);
    }
//
// dm_test_brightness_to_user_at_min - Test Brightness to user at min
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_at_min(test: *mut kunit) {
    static void dm_test_brightness_to_user_at_min(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    get_brightness_range(&caps, &min, &max);
// At min → should return 0
    KUNIT_EXPECT_EQ(test, convert_brightness_to_user(&caps, min), 0U);
    }
//
// dm_test_brightness_to_user_midpoint_pwm - Test Brightness to user midpoint pwm
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_midpoint_pwm(test: *mut kunit) {
    static void dm_test_brightness_to_user_midpoint_pwm(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max, mid_hw, result;
    u64 expected;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    get_brightness_range(&caps, &min, &max);
// midpoint of hw range
    mid_hw = min + (max - min) / 2;
// expected = DIV_ROUND_CLOSEST_ULL((u64)max * (mid_hw - min), max - min)
    expected = DIV_ROUND_CLOSEST_ULL((u64)max * (mid_hw - min), max - min);
    result = convert_brightness_to_user(&caps, mid_hw);
    KUNIT_EXPECT_EQ(test, result, (u32)expected);
    }
// Tests for convert_brightness_from_user() — no custom curve
//
// dm_test_brightness_from_user_null_caps - Test Brightness from user null caps
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_null_caps(test: *mut kunit) {
    static void dm_test_brightness_from_user_null_caps(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, convert_brightness_from_user(core::ptr::null_mut(), 100), 100U);
    }
//
// dm_test_brightness_from_user_zero - Test Brightness from user zero
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_zero(test: *mut kunit) {
    static void dm_test_brightness_from_user_zero(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
// no custom curve
    caps.data_points = 0;
    get_brightness_range(&caps, &min, &max);
// brightness=0 → min + 0 = min
    KUNIT_EXPECT_EQ(test, convert_brightness_from_user(&caps, 0), (u32)min);
    }
//
// dm_test_brightness_from_user_max - Test Brightness from user max
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_max(test: *mut kunit) {
    static void dm_test_brightness_from_user_max(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 0;
    get_brightness_range(&caps, &min, &max);
//
// brightness=max → min + DIV_ROUND_CLOSEST((max-min)*max, max)
// = min + (max - min) = max
//
    KUNIT_EXPECT_EQ(test, convert_brightness_from_user(&caps, max), (u32)max);
    }
//
// dm_test_brightness_from_user_aux - Test Brightness from user aux
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_aux(test: *mut kunit) {
    static void dm_test_brightness_from_user_aux(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.aux_support = true;
    caps.aux_min_input_signal = 1;
    caps.aux_max_input_signal = 512;
    caps.data_points = 0;
    get_brightness_range(&caps, &min, &max);
// brightness=0 → min
    KUNIT_EXPECT_EQ(test, convert_brightness_from_user(&caps, 0), (u32)min);
// brightness=max → max
    KUNIT_EXPECT_EQ(test, convert_brightness_from_user(&caps, max), (u32)max);
    }
// Tests for convert_custom_brightness()
//
// dm_test_custom_brightness_no_data_points - Test Custom brightness no data points
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_no_data_points(test: *mut kunit) {
    static void dm_test_custom_brightness_no_data_points(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    let mut brightness: u32 = 128;
    let mut saved: u32 = brightness;
    caps.data_points = 0;
    convert_custom_brightness(&caps, 3084, 65535, &brightness);
// No data points → no-op
    KUNIT_EXPECT_EQ(test, brightness, saved);
    }
//
// dm_test_custom_brightness_debug_mask_disables - Test Custom brightness debug mask disables
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_debug_mask_disables(test: *mut kunit) {
    static void dm_test_custom_brightness_debug_mask_disables(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    let mut brightness: u32 = 128;
    let mut saved: u32 = brightness;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    caps.data_points = 3;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 10;
// Set the disable flag
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() | DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    convert_custom_brightness(&caps, 3084, 65535, &brightness);
// Should be no-op due to debug mask
    KUNIT_EXPECT_EQ(test, brightness, saved);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_exact_match - Test Custom brightness exact match
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_exact_match(test: *mut kunit) {
    static void dm_test_custom_brightness_exact_match(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 3;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 20;
    caps.luminance_data[1].input_signal = 128;
    caps.luminance_data[1].luminance = 50;
    caps.luminance_data[2].input_signal = 200;
    caps.luminance_data[2].luminance = 90;
    get_brightness_range(&caps, &min, &max);
//
// Set brightness so that scale_input_to_fw yields exactly 128.
// scale_input_to_fw(min, max, x) = DIV_ROUND_CLOSEST(x * 255, max - min)
// With min=0, max=0x101*255=65535:
// We need x such that DIV_ROUND_CLOSEST(x * 255, 65535) = 128
// → x = 128 * 65535 / 255 = 32896
//
    brightness = 32896;
    convert_custom_brightness(&caps, min, max, &brightness);
//
// Exact match: lum=50, brightness_scaled=128
// result = scale_fw_to_input(min, max, DIV_ROUND_CLOSEST(50*128, 101))
// = scale_fw_to_input(0, 65535, DIV_ROUND_CLOSEST(6400, 101))
// = scale_fw_to_input(0, 65535, 63)
// = 0 + DIV_ROUND_CLOSEST(63 * 65535, 255) = 16191 (approx)
//
    KUNIT_EXPECT_TRUE(test, brightness != 32896);
    KUNIT_EXPECT_TRUE(test, brightness < 32896);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_below_first - Test Custom brightness below first
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_below_first(test: *mut kunit) {
    static void dm_test_custom_brightness_below_first(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 2;
    caps.luminance_data[0].input_signal = 100;
    caps.luminance_data[0].luminance = 40;
    caps.luminance_data[1].input_signal = 200;
    caps.luminance_data[1].luminance = 80;
    get_brightness_range(&caps, &min, &max);
//
// Set brightness low enough that scaled value < 100.
// scale_input_to_fw(0, 65535, x) = DIV_ROUND_CLOSEST(x*255, 65535)
// For result=50: x = 50*65535/255 = 12850
//
    brightness = 12850;
    convert_custom_brightness(&caps, min, max, &brightness);
//
// Below first data point: lum = DIV_ROUND_CLOSEST(40 * 50, 100) = 20
// Then: scale_fw_to_input(0, 65535, DIV_ROUND_CLOSEST(20 * 50, 101))
// = scale_fw_to_input(0, 65535, DIV_ROUND_CLOSEST(1000, 101))
// = scale_fw_to_input(0, 65535, 10)
// The output should be significantly less than input.
//
    KUNIT_EXPECT_TRUE(test, brightness < 12850);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_interpolation - Test Custom brightness interpolation
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_interpolation(test: *mut kunit) {
    static void dm_test_custom_brightness_interpolation(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 2;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 20;
    caps.luminance_data[1].input_signal = 200;
    caps.luminance_data[1].luminance = 80;
    get_brightness_range(&caps, &min, &max);
//
// Choose a value between data points 50 and 200.
// scale_input_to_fw(0, 65535, x) = 125 when x = 125*65535/255 = 32125
//
    brightness = 32125;
    convert_custom_brightness(&caps, min, max, &brightness);
//
// The function should interpolate between data points and produce
// a remapped value different from the input.
//
    KUNIT_EXPECT_TRUE(test, brightness != 32125);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_above_last - Test Custom brightness above last data point
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_above_last(test: *mut kunit) {
    static void dm_test_custom_brightness_above_last(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 2;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 20;
    caps.luminance_data[1].input_signal = 150;
    caps.luminance_data[1].luminance = 60;
    get_brightness_range(&caps, &min, &max);
//
// Choose brightness above the last data point (150).
// scale_input_to_fw(0, 65535, x) = 220 when x = 220*65535/255 = 56533
// After binary search, left >= data_points, clamped → right==left,
// so lum = upper_lum = 60.
//
    brightness = 56533;
    convert_custom_brightness(&caps, min, max, &brightness);
// Output should differ from input (remapped via curve)
    KUNIT_EXPECT_TRUE(test, brightness != 56533);
    KUNIT_EXPECT_TRUE(test, brightness < 56533);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_single_data_point - Test Custom brightness with single data point
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_single_data_point(test: *mut kunit) {
    static void dm_test_custom_brightness_single_data_point(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 1;
    caps.luminance_data[0].input_signal = 128;
    caps.luminance_data[0].luminance = 50;
    get_brightness_range(&caps, &min, &max);
//
// Brightness below the single data point triggers the
// "below first" path: lum = DIV_ROUND_CLOSEST(50 * scaled, 128).
// scale_input_to_fw(0, 65535, x) = 64 when x = 64*65535/255 = 16448
//
    brightness = 16448;
    convert_custom_brightness(&caps, min, max, &brightness);
    KUNIT_EXPECT_TRUE(test, brightness < 16448);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_custom_brightness_lower_lum_zero - Test Custom brightness with zero lower luminance
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_custom_brightness_lower_lum_zero(test: *mut kunit) {
    static void dm_test_custom_brightness_lower_lum_zero(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    uint32_t brightness;
    unsigned int min, max;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 2;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 0;	/* zero lower luminance */
    caps.luminance_data[1].input_signal = 200;
    caps.luminance_data[1].luminance = 80;
    get_brightness_range(&caps, &min, &max);
//
// Choose brightness between data points to trigger interpolation.
// scale_input_to_fw(0, 65535, x) = 125 when x = 125*65535/255 = 32125
// With lower_lum == 0, code takes shortcut: lum = upper_lum = 80.
//
    brightness = 32125;
    convert_custom_brightness(&caps, min, max, &brightness);
// Should remap; result should differ from input
    KUNIT_EXPECT_TRUE(test, brightness != 32125);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_brightness_to_user_above_max - Test Brightness to user above max
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_to_user_above_max(test: *mut kunit) {
    static void dm_test_brightness_to_user_above_max(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max, result;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    get_brightness_range(&caps, &min, &max);
// brightness above max → result > max (linear extrapolation)
    result = convert_brightness_to_user(&caps, max + 1000);
    KUNIT_EXPECT_GT(test, result, max);
    }
//
// dm_test_brightness_from_user_midrange - Test Brightness from user mid-range value
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_midrange(test: *mut kunit) {
    static void dm_test_brightness_from_user_midrange(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    u32 result;
    caps.aux_support = false;
    caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 0;
    get_brightness_range(&caps, &min, &max);
// Mid-range brightness should map to between min and max
    result = convert_brightness_from_user(&caps, max / 2);
    KUNIT_EXPECT_GE(test, result, min);
    KUNIT_EXPECT_LE(test, result, max);
    }
//
// dm_test_brightness_from_user_with_curve - Test Brightness from user with custom curve active
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_from_user_with_curve(test: *mut kunit) {
    static void dm_test_brightness_from_user_with_curve(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    u32 with_curve, without_curve;
    let mut saved_mask: c_uint = amdgpu_dm_get_dc_debug_mask();
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() & ~DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
    caps.data_points = 2;
    caps.luminance_data[0].input_signal = 50;
    caps.luminance_data[0].luminance = 20;
    caps.luminance_data[1].input_signal = 200;
    caps.luminance_data[1].luminance = 80;
    get_brightness_range(&caps, &min, &max);
    with_curve = convert_brightness_from_user(&caps, max / 2);
// Now disable the curve and compare
    amdgpu_dm_set_dc_debug_mask(amdgpu_dm_get_dc_debug_mask() | DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE);
    without_curve = convert_brightness_from_user(&caps, max / 2);
// Custom curve should produce a different mapping
    KUNIT_EXPECT_NE(test, with_curve, without_curve);
    amdgpu_dm_set_dc_debug_mask(saved_mask);
    }
//
// dm_test_brightness_range_zero_signals - Test Brightness range with zero min and max signals
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_brightness_range_zero_signals(test: *mut kunit) {
    static void dm_test_brightness_range_zero_signals(struct kunit *test)
    {
    let mut caps: amdgpu_dm_backlight_caps = {};
    let mut min: c_uint = 99, max = 99;
    caps.aux_support = false;
    caps.min_input_signal = 0;
    caps.max_input_signal = 0;
// Both signals zero → min=max=0
    KUNIT_EXPECT_EQ(test, get_brightness_range(&caps, &min, &max), 1);
    KUNIT_EXPECT_EQ(test, min, 0U);
    KUNIT_EXPECT_EQ(test, max, 0U);
    }
// Tests for amdgpu_dm_backlight_fill_props()
//
// dm_test_backlight_fill_props_ac_linear - Test AC brightness and linear scale
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_fill_props_ac_linear(test: *mut kunit) {
    static void dm_test_backlight_fill_props_ac_linear(struct kunit *test)
    {
    let mut props: backlight_properties = {};
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.min_input_signal = 12;
    caps.max_input_signal = 255;
    caps.ac_level = 40;
    caps.dc_level = 20;
    get_brightness_range(&caps, &min, &max);
    amdgpu_dm_backlight_fill_props(&caps, true, false, &props);
    KUNIT_EXPECT_EQ(test, props.brightness,
    DIV_ROUND_CLOSEST(max * caps.ac_level, 100));
    KUNIT_EXPECT_EQ(test, props.max_brightness, max);
    KUNIT_EXPECT_EQ(test, props.scale, BACKLIGHT_SCALE_LINEAR);
    KUNIT_EXPECT_EQ(test, props.type, BACKLIGHT_RAW);
    }
//
// dm_test_backlight_fill_props_dc_nonlinear - Test DC brightness and non-linear scale
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_fill_props_dc_nonlinear(test: *mut kunit) {
    static void dm_test_backlight_fill_props_dc_nonlinear(struct kunit *test)
    {
    let mut props: backlight_properties = {};
    let mut caps: amdgpu_dm_backlight_caps = {};
    unsigned int min, max;
    caps.min_input_signal = 12;
    caps.max_input_signal = 255;
    caps.ac_level = 40;
    caps.dc_level = 20;
    caps.data_points = 2;
    get_brightness_range(&caps, &min, &max);
    amdgpu_dm_backlight_fill_props(&caps, false, true, &props);
    KUNIT_EXPECT_EQ(test, props.brightness,
    DIV_ROUND_CLOSEST(max * caps.dc_level, 100));
    KUNIT_EXPECT_EQ(test, props.max_brightness, max);
    KUNIT_EXPECT_EQ(test, props.scale, BACKLIGHT_SCALE_NON_LINEAR);
    KUNIT_EXPECT_EQ(test, props.type, BACKLIGHT_RAW);
    }
//
// dm_test_backlight_fill_props_default_range - Test default properties without caps
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_backlight_fill_props_default_range(test: *mut kunit) {
    static void dm_test_backlight_fill_props_default_range(struct kunit *test)
    {
    let mut props: backlight_properties = {};
    amdgpu_dm_backlight_fill_props(core::ptr::null_mut(), false, true, &props);
    KUNIT_EXPECT_EQ(test, props.brightness, MAX_BACKLIGHT_LEVEL);
    KUNIT_EXPECT_EQ(test, props.max_brightness, MAX_BACKLIGHT_LEVEL);
    KUNIT_EXPECT_EQ(test, props.scale, BACKLIGHT_SCALE_LINEAR);
    KUNIT_EXPECT_EQ(test, props.type, BACKLIGHT_RAW);
    }
// Tests for amdgpu_dm_update_connector_ext_caps()
//
// dm_test_update_connector_ext_caps_negative_bl_idx - Test negative backlight index early return
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_negative_bl_idx(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_negative_bl_idx(struct kunit *test)
    {
    struct amdgpu_dm_connector *aconnector;
    aconnector = kunit_kzalloc(test, sizeof(*aconnector), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, aconnector);
    aconnector.bl_idx = -1;
    amdgpu_dm_update_connector_ext_caps(aconnector);
    KUNIT_SUCCEED(test);
    }
//
// dm_test_update_connector_ext_caps_non_edp - Test non-eDP connector early return
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_non_edp(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_non_edp(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_HDMI_TYPE_A);
    fixture.adev.dm.backlight_caps[0].aux_support = true;
    amdgpu_dm_update_connector_ext_caps(fixture.aconnector);
    KUNIT_EXPECT_TRUE(test, fixture.adev.dm.backlight_caps[0].aux_support);
    KUNIT_EXPECT_PTR_EQ(test, fixture.adev.dm.backlight_caps[0].ext_caps, core::ptr::null_mut());
    }
//
// dm_test_update_connector_ext_caps_oled_defaults - Test OLED eDP defaults to AUX backlight
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_oled_defaults(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_oled_defaults(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    amdgpu_dm_set_backlight_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.link.dpcd_sink_ext_caps.bits.oled = 1;
    amdgpu_dm_update_connector_ext_caps(fixture.aconnector);
    KUNIT_EXPECT_PTR_EQ(test, fixture.adev.dm.backlight_caps[0].ext_caps,
    &fixture.link.dpcd_sink_ext_caps);
    KUNIT_EXPECT_TRUE(test, fixture.adev.dm.backlight_caps[0].aux_support);
    KUNIT_EXPECT_EQ(test, fixture.link.backlight_control_type,
    BACKLIGHT_CONTROL_AMD_AUX);
    KUNIT_EXPECT_EQ(test, fixture.adev.dm.backlight_caps[0].aux_max_input_signal, 512);
    KUNIT_EXPECT_EQ(test, fixture.adev.dm.backlight_caps[0].aux_min_input_signal, 1);
    amdgpu_dm_set_backlight_param(saved_backlight);
    }
//
// dm_test_update_connector_ext_caps_luminance_values - Test luminance range copy
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_luminance_values(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_luminance_values(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    amdgpu_dm_set_backlight_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.display_info.luminance_range.min_luminance = 2;
    fixture.aconnector.base.display_info.luminance_range.max_luminance = 400;
    amdgpu_dm_update_connector_ext_caps(fixture.aconnector);
    KUNIT_EXPECT_FALSE(test, fixture.adev.dm.backlight_caps[0].aux_support);
    KUNIT_EXPECT_EQ(test, fixture.adev.dm.backlight_caps[0].aux_max_input_signal, 400);
    KUNIT_EXPECT_EQ(test, fixture.adev.dm.backlight_caps[0].aux_min_input_signal, 2);
    amdgpu_dm_set_backlight_param(saved_backlight);
    }
//
// dm_test_update_connector_ext_caps_force_aux - Test module parameter forces AUX backlight
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_force_aux(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_force_aux(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    amdgpu_dm_set_backlight_param(1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    amdgpu_dm_update_connector_ext_caps(fixture.aconnector);
    KUNIT_EXPECT_TRUE(test, fixture.adev.dm.backlight_caps[0].aux_support);
    KUNIT_EXPECT_EQ(test, fixture.link.backlight_control_type,
    BACKLIGHT_CONTROL_AMD_AUX);
    amdgpu_dm_set_backlight_param(saved_backlight);
    }
//
// dm_test_update_connector_ext_caps_force_pwm - Test module parameter forces PWM backlight
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_update_connector_ext_caps_force_pwm(test: *mut kunit) {
    static void dm_test_update_connector_ext_caps_force_pwm(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    amdgpu_dm_set_backlight_param(0);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.link.dpcd_sink_ext_caps.bits.oled = 1;
    amdgpu_dm_update_connector_ext_caps(fixture.aconnector);
    KUNIT_EXPECT_FALSE(test, fixture.adev.dm.backlight_caps[0].aux_support);
    KUNIT_EXPECT_NE(test, fixture.link.backlight_control_type,
    BACKLIGHT_CONTROL_AMD_AUX);
    amdgpu_dm_set_backlight_param(saved_backlight);
    }
// Tests for amdgpu_dm_should_create_sysfs()
//
// dm_test_should_create_sysfs_abm_forced - Test forced ABM disables sysfs
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_abm_forced(test: *mut kunit) {
    static void dm_test_should_create_sysfs_abm_forced(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_eDP;
    KUNIT_EXPECT_FALSE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_should_create_sysfs_non_edp - Test non-eDP connector disables sysfs
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_non_edp(test: *mut kunit) {
    static void dm_test_should_create_sysfs_non_edp(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_HDMI_TYPE_A);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_HDMIA;
    KUNIT_EXPECT_FALSE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_should_create_sysfs_no_backlight_index - Test eDP without backlight index enables sysfs
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_no_backlight_index(test: *mut kunit) {
    static void dm_test_should_create_sysfs_no_backlight_index(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(-1);
    setup_test_connector(test, &fixture, -1, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_eDP;
    fixture.link.panel_type = PANEL_TYPE_LCD;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_should_create_sysfs_oled_no_cacp - Test OLED without CACP disables sysfs
// @test: The KUnit test context
//
// A non-LCD panel that does not support CACP must not expose the sysfs
// backlight interface.
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_oled_no_cacp(test: *mut kunit) {
    static void dm_test_should_create_sysfs_oled_no_cacp(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_eDP;
    fixture.link.panel_type = PANEL_TYPE_OLED;
    fixture.link.panel_config.cacp.cacp_supported = false;
    KUNIT_EXPECT_FALSE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_should_create_sysfs_oled_cacp - Test OLED with CACP enables sysfs
// @test: The KUnit test context
//
// An OLED panel that supports CACP must expose the sysfs backlight
// interface so the ABM/CACP level can be controlled.
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_oled_cacp(test: *mut kunit) {
    static void dm_test_should_create_sysfs_oled_cacp(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_eDP;
    fixture.link.panel_type = PANEL_TYPE_OLED;
    fixture.link.panel_config.cacp.cacp_supported = true;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_should_create_sysfs_lcd_panel - Test LCD eDP panel enables sysfs
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_create_sysfs_lcd_panel(test: *mut kunit) {
    static void dm_test_should_create_sysfs_lcd_panel(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_abm_level_param(-1);
    setup_test_connector(test, &fixture, 0, SIGNAL_TYPE_EDP);
    fixture.aconnector.base.connector_type = DRM_MODE_CONNECTOR_eDP;
    fixture.link.panel_type = PANEL_TYPE_LCD;
    KUNIT_EXPECT_TRUE(test, amdgpu_dm_should_create_sysfs(fixture.aconnector));
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
// Tests for amdgpu_dm_setup_backlight_device()
//
// dm_test_setup_backlight_device_non_edp - Test non-eDP/LVDS link is skipped
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_setup_backlight_device_non_edp(test: *mut kunit) {
    static void dm_test_setup_backlight_device_non_edp(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    struct amdgpu_display_manager *dm;
    setup_test_connector(test, &fixture, -1, SIGNAL_TYPE_HDMI_TYPE_A);
    fixture.link.type = dc_connection_single;
    dm = &fixture.adev.dm;
    dm.adev = fixture.adev;
    dm.num_of_edps = 0;
    amdgpu_dm_setup_backlight_device(dm, fixture.aconnector);
// Non-eDP/LVDS signal → no backlight setup
    KUNIT_EXPECT_EQ(test, dm.num_of_edps, 0);
    KUNIT_EXPECT_EQ(test, fixture.aconnector.bl_idx, -1);
    }
//
// dm_test_setup_backlight_device_connection_none - Test disconnected link is skipped
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_setup_backlight_device_connection_none(test: *mut kunit) {
    static void dm_test_setup_backlight_device_connection_none(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    struct amdgpu_display_manager *dm;
    setup_test_connector(test, &fixture, -1, SIGNAL_TYPE_EDP);
    fixture.link.type = dc_connection_none;
    dm = &fixture.adev.dm;
    dm.adev = fixture.adev;
    dm.num_of_edps = 0;
    amdgpu_dm_setup_backlight_device(dm, fixture.aconnector);
// Disconnected link → no backlight setup
    KUNIT_EXPECT_EQ(test, dm.num_of_edps, 0);
    KUNIT_EXPECT_EQ(test, fixture.aconnector.bl_idx, -1);
    }
//
// dm_test_setup_backlight_device_max_edps - Test setup is skipped when at eDP limit
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_setup_backlight_device_max_edps(test: *mut kunit) {
    static void dm_test_setup_backlight_device_max_edps(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    struct amdgpu_display_manager *dm;
    setup_test_connector(test, &fixture, -1, SIGNAL_TYPE_EDP);
    fixture.link.type = dc_connection_single;
    dm = &fixture.adev.dm;
    dm.adev = fixture.adev;
    dm.num_of_edps = AMDGPU_DM_MAX_NUM_EDP;
    amdgpu_dm_setup_backlight_device(dm, fixture.aconnector);
// Already at the eDP limit → no additional setup
    KUNIT_EXPECT_EQ(test, dm.num_of_edps, AMDGPU_DM_MAX_NUM_EDP);
    KUNIT_EXPECT_EQ(test, fixture.aconnector.bl_idx, -1);
    }
//
// dm_test_setup_backlight_device_oled_success - Test successful eDP backlight setup
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_setup_backlight_device_oled_success(test: *mut kunit) {
    static void dm_test_setup_backlight_device_oled_success(struct kunit *test)
    {
    let mut fixture: dm_backlight_connector_fixture = {};
    struct amdgpu_display_manager *dm;
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    amdgpu_dm_set_backlight_param(-1);
// Skip ABM property attach (requires full DRM object setup)
    amdgpu_dm_set_abm_level_param(0);
    setup_test_connector(test, &fixture, -1, SIGNAL_TYPE_EDP);
    fixture.link.type = dc_connection_single;
    fixture.link.dpcd_sink_ext_caps.bits.oled = 1;
    dm = &fixture.adev.dm;
    dm.adev = fixture.adev;
    dm.num_of_edps = 0;
    amdgpu_dm_setup_backlight_device(dm, fixture.aconnector);
    KUNIT_EXPECT_EQ(test, dm.num_of_edps, 1);
    KUNIT_EXPECT_EQ(test, fixture.aconnector.bl_idx, 0);
    KUNIT_EXPECT_PTR_EQ(test, (void *)dm.backlight_link[0],
    (void *)fixture.link);
    KUNIT_EXPECT_TRUE(test, dm.backlight_caps[0].aux_support);
    amdgpu_dm_set_backlight_param(saved_backlight);
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
//
// dm_test_setup_backlight_device_attaches_abm_property - Test ABM property path
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_setup_backlight_device_attaches_abm_property(test: *mut kunit) {
    static void dm_test_setup_backlight_device_attaches_abm_property(struct kunit *test)
    {
    struct amdgpu_dm_connector *aconnector;
    struct amdgpu_display_manager *dm;
    struct amdgpu_device *adev;
    struct drm_property *prop;
    struct dc_link *link;
    let mut saved_abm_level: c_int = amdgpu_dm_get_abm_level_param();
    let mut saved_backlight: c_int = amdgpu_dm_get_backlight_param();
    int old_count;
    int ret;
    amdgpu_dm_set_abm_level_param(-1);
    amdgpu_dm_set_backlight_param(-1);
    adev = dm_kunit_alloc_adev(test);
    ret = drmm_mode_config_init(&adev.ddev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    prop = drm_property_create_range(&adev.ddev, 0, "abm level", 0, 4);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, prop);
    adev.mode_info.abm_level_property = prop;
    aconnector = dm_kunit_alloc_connector(test, adev, core::ptr::null_mut());
    ret = drmm_connector_init(&adev.ddev, &aconnector.base,
    &dm_backlight_test_connector_funcs,
    DRM_MODE_CONNECTOR_eDP, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    link = dm_kunit_alloc_link(test);
    link.connector_signal = SIGNAL_TYPE_EDP;
    link.type = dc_connection_single;
    aconnector.dc_link = link;
    aconnector.bl_idx = -1;
    dm = &adev.dm;
    dm.adev = adev;
    dm.ddev = &adev.ddev;
    old_count = aconnector.base.base.properties.count;
    amdgpu_dm_setup_backlight_device(dm, aconnector);
    KUNIT_EXPECT_EQ(test, dm.num_of_edps, 1);
    KUNIT_EXPECT_EQ(test, aconnector.bl_idx, 0);
    KUNIT_EXPECT_EQ(test, aconnector.base.base.properties.count, old_count + 1);
    KUNIT_EXPECT_PTR_EQ(test, aconnector.base.base.properties.properties[old_count], prop);
    KUNIT_EXPECT_EQ(test, aconnector.base.base.properties.values[old_count],
    (uint64_t)ABM_SYSFS_CONTROL);
    amdgpu_dm_set_backlight_param(saved_backlight);
    amdgpu_dm_set_abm_level_param(saved_abm_level);
    }
    static struct kunit_case dm_backlight_test_cases[] = {
// dm_find_stream_with_link
    KUNIT_CASE(dm_test_find_stream_with_link_returns_match),
    KUNIT_CASE(dm_test_find_stream_with_link_missing),
// amdgpu_dm_backlight_set_level / update_status
    KUNIT_CASE(dm_test_backlight_set_level_connector_off),
    KUNIT_CASE(dm_test_backlight_set_level_no_stream),
    KUNIT_CASE(dm_test_backlight_set_level_aux_programs_power_module),
    KUNIT_CASE(dm_test_backlight_set_level_pwm_programs_power_module),
    KUNIT_CASE(dm_test_backlight_set_level_reallows_idle),
    KUNIT_CASE(dm_test_backlight_update_status_no_stream),
// amdgpu_dm_backlight_get_level / get_brightness
    KUNIT_CASE(dm_test_backlight_get_level_pwm_success),
    KUNIT_CASE(dm_test_backlight_get_level_pwm_error),
    KUNIT_CASE(dm_test_backlight_get_level_aux_success),
    KUNIT_CASE(dm_test_backlight_get_level_aux_error),
    KUNIT_CASE(dm_test_backlight_get_brightness_uses_device_index),
// amdgpu_dm_register_backlight_device
    KUNIT_CASE(dm_test_register_backlight_device_negative_index),
    KUNIT_CASE(dm_test_register_backlight_device_success),
// panel_power_savings_show / store
    KUNIT_CASE(dm_test_panel_power_savings_show_maps_disable_to_zero),
    KUNIT_CASE(dm_test_panel_power_savings_show_reports_level),
    KUNIT_CASE(dm_test_panel_power_savings_store_sets_disable),
    KUNIT_CASE(dm_test_panel_power_savings_store_forbidden),
    KUNIT_CASE(dm_test_panel_power_savings_store_rejects_invalid_text),
    KUNIT_CASE(dm_test_panel_power_savings_store_rejects_out_of_range),
// amdgpu_dm_backlight_get_device_index
    KUNIT_CASE(dm_test_backlight_device_index_matches_second),
    KUNIT_CASE(dm_test_backlight_device_index_missing_fallback),
    KUNIT_CASE(dm_test_backlight_caps_valid_short_circuit),

    KUNIT_CASE(dm_test_backlight_caps_aux_support_noop),
    KUNIT_CASE(dm_test_backlight_caps_non_aux_sets_defaults),

// get_brightness_range
    KUNIT_CASE(dm_test_brightness_range_null_caps),
    KUNIT_CASE(dm_test_brightness_range_pwm),
    KUNIT_CASE(dm_test_brightness_range_aux),
// convert_brightness_to_user
    KUNIT_CASE(dm_test_brightness_to_user_null_caps),
    KUNIT_CASE(dm_test_brightness_to_user_below_min),
    KUNIT_CASE(dm_test_brightness_to_user_at_max),
    KUNIT_CASE(dm_test_brightness_to_user_at_min),
    KUNIT_CASE(dm_test_brightness_to_user_midpoint_pwm),
// convert_brightness_from_user
    KUNIT_CASE(dm_test_brightness_from_user_null_caps),
    KUNIT_CASE(dm_test_brightness_from_user_zero),
    KUNIT_CASE(dm_test_brightness_from_user_max),
    KUNIT_CASE(dm_test_brightness_from_user_aux),
// convert_custom_brightness
    KUNIT_CASE(dm_test_custom_brightness_no_data_points),
    KUNIT_CASE(dm_test_custom_brightness_debug_mask_disables),
    KUNIT_CASE(dm_test_custom_brightness_exact_match),
    KUNIT_CASE(dm_test_custom_brightness_below_first),
    KUNIT_CASE(dm_test_custom_brightness_interpolation),
    KUNIT_CASE(dm_test_custom_brightness_above_last),
    KUNIT_CASE(dm_test_custom_brightness_single_data_point),
    KUNIT_CASE(dm_test_custom_brightness_lower_lum_zero),
    KUNIT_CASE(dm_test_brightness_to_user_above_max),
    KUNIT_CASE(dm_test_brightness_from_user_midrange),
    KUNIT_CASE(dm_test_brightness_from_user_with_curve),
    KUNIT_CASE(dm_test_brightness_range_zero_signals),
// amdgpu_dm_backlight_fill_props
    KUNIT_CASE(dm_test_backlight_fill_props_ac_linear),
    KUNIT_CASE(dm_test_backlight_fill_props_dc_nonlinear),
    KUNIT_CASE(dm_test_backlight_fill_props_default_range),
// amdgpu_dm_update_connector_ext_caps
    KUNIT_CASE(dm_test_update_connector_ext_caps_negative_bl_idx),
    KUNIT_CASE(dm_test_update_connector_ext_caps_non_edp),
    KUNIT_CASE(dm_test_update_connector_ext_caps_oled_defaults),
    KUNIT_CASE(dm_test_update_connector_ext_caps_luminance_values),
    KUNIT_CASE(dm_test_update_connector_ext_caps_force_aux),
    KUNIT_CASE(dm_test_update_connector_ext_caps_force_pwm),
// amdgpu_dm_should_create_sysfs
    KUNIT_CASE(dm_test_should_create_sysfs_abm_forced),
    KUNIT_CASE(dm_test_should_create_sysfs_non_edp),
    KUNIT_CASE(dm_test_should_create_sysfs_no_backlight_index),
    KUNIT_CASE(dm_test_should_create_sysfs_oled_no_cacp),
    KUNIT_CASE(dm_test_should_create_sysfs_oled_cacp),
    KUNIT_CASE(dm_test_should_create_sysfs_lcd_panel),
// amdgpu_dm_setup_backlight_device
    KUNIT_CASE(dm_test_setup_backlight_device_non_edp),
    KUNIT_CASE(dm_test_setup_backlight_device_connection_none),
    KUNIT_CASE(dm_test_setup_backlight_device_max_edps),
    KUNIT_CASE(dm_test_setup_backlight_device_oled_success),
    KUNIT_CASE(dm_test_setup_backlight_device_attaches_abm_property),
    {}
    };
    static struct kunit_suite dm_backlight_test_suite = {
    .name = "amdgpu_dm_backlight",
    .test_cases = dm_backlight_test_cases,
    };
    kunit_test_suite(dm_backlight_test_suite);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_backlight");
    MODULE_AUTHOR("AMD");
