//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/tests/vc4_test_pv_muxing.c
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
pub struct pv_muxing_priv {
    pub vc4: *mut vc4_dev,
}

    static bool check_fifo_conflict(struct kunit *test,
    const struct drm_atomic_commit *state)
    {
    struct vc4_hvs_state *hvs_state;
    let mut used_fifos: c_uint = 0;
    unsigned int i;
    hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hvs_state);
    for (i = 0; i < HVS_NUM_CHANNELS; i++) {
    if (!hvs_state.fifo_state[i].in_use)
    continue;
    KUNIT_EXPECT_FALSE(test, used_fifos & BIT(i));
    used_fifos |= BIT(i);
    }
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_constraint {
    pub type: enum vc4_encoder_type,
    pub channels: *mut c_uint,
    pub nchannels: usize,
}

    {								\
    .type = _type,						\
    .channels = (unsigned int[]) { __VA_ARGS__ },		\
    .nchannels = sizeof((unsigned int[]) { __VA_ARGS__ }) /	\
    sizeof(unsigned int),			\
    }
    static bool __check_encoder_constraints(const struct encoder_constraint *constraints,
    size_t nconstraints,
    enum vc4_encoder_type type,
    unsigned int channel)
    {
    unsigned int i;
    for (i = 0; i < nconstraints; i++) {
    const struct encoder_constraint *constraint = &constraints[i];
    unsigned int j;
    if (constraint.type != type)
    continue;
    for (j = 0; j < constraint.nchannels; j++) {
    let mut _channel: c_uint = constraint.channels[j];
    if (channel != _channel)
    continue;
    return true;
    }
    }
    return false;
    }
    static const struct encoder_constraint vc4_encoder_constraints[] = {
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DPI, 0),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DSI0, 0),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_HDMI0, 1),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_VEC, 1),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_TXP0, 2),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DSI1, 2),
    };
    static const struct encoder_constraint vc5_encoder_constraints[] = {
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DPI, 0),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DSI0, 0),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_VEC, 1),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_TXP0, 0, 2),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_DSI1, 0, 1, 2),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_HDMI0, 0, 1, 2),
    ENCODER_CONSTRAINT(VC4_ENCODER_TYPE_HDMI1, 0, 1, 2),
    };
#[no_mangle]
unsafe extern "C" fn check_vc4_encoder_constraints(type: enum vc4_encoder_type, channel: c_uint) -> bool {
    static bool check_vc4_encoder_constraints(enum vc4_encoder_type type, unsigned int channel)
    {
    return __check_encoder_constraints(vc4_encoder_constraints,
    ARRAY_SIZE(vc4_encoder_constraints),
    type, channel);
    }
#[no_mangle]
unsafe extern "C" fn check_vc5_encoder_constraints(type: enum vc4_encoder_type, channel: c_uint) -> bool {
    static bool check_vc5_encoder_constraints(enum vc4_encoder_type type, unsigned int channel)
    {
    return __check_encoder_constraints(vc5_encoder_constraints,
    ARRAY_SIZE(vc5_encoder_constraints),
    type, channel);
    }
    static struct vc4_crtc_state *
    get_vc4_crtc_state_for_encoder(struct kunit *test,
    const struct drm_atomic_commit *state,
    enum vc4_encoder_type type)
    {
    struct drm_device *drm = state.dev;
    struct drm_crtc_state *new_crtc_state;
    struct drm_encoder *encoder;
    struct drm_crtc *crtc;
    encoder = vc4_find_encoder_by_type(drm, type);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, encoder);
    crtc = vc4_find_crtc_for_encoder(test, drm, encoder);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, crtc);
    new_crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    if (!new_crtc_state)
    return core::ptr::null_mut();
    return to_vc4_crtc_state(new_crtc_state);
    }
    static bool check_channel_for_encoder(struct kunit *test,
    const struct drm_atomic_commit *state,
    enum vc4_encoder_type type,
    bool (*check_fn)(enum vc4_encoder_type type, unsigned int channel))
    {
    struct vc4_crtc_state *new_vc4_crtc_state;
    struct vc4_hvs_state *new_hvs_state;
    unsigned int channel;
    new_hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_hvs_state);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state, type);
    KUNIT_ASSERT_NOT_NULL(test, new_vc4_crtc_state);
    channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_EXPECT_NE(test, channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_EXPECT_TRUE(test, new_hvs_state.fifo_state[channel].in_use);
    KUNIT_EXPECT_TRUE(test, check_fn(type, channel));
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_muxing_param {
    pub name: *const c_char,
    pub test): *mut *mut *mut vc4_dev (mock_fn)(kunit,
    pub channel): *mut *mut bool (check_fn)(enum vc4_encoder_type type, unsigned int,
    pub encoders: *mut enum vc4_encoder_type,
    pub nencoders: usize,
}

#[no_mangle]
unsafe extern "C" fn vc4_test_pv_muxing_desc(t: *const pv_muxing_param, desc: *mut c_char) {
    static void vc4_test_pv_muxing_desc(const struct pv_muxing_param *t, char *desc)
    {
    strscpy(desc, t.name, KUNIT_PARAM_DESC_SIZE);
    }

    {										\
    .name = _name,								\
    .mock_fn = &_mock_fn,							\
    .check_fn = &_check_fn,							\
    .encoders = (enum vc4_encoder_type[]) { __VA_ARGS__ },			\
    .nencoders = sizeof((enum vc4_encoder_type[]) { __VA_ARGS__ }) /	\
    sizeof(enum vc4_encoder_type),				\
    }

    PV_MUXING_TEST(_name, vc4_mock_device, check_vc4_encoder_constraints, __VA_ARGS__)

    PV_MUXING_TEST(_name, vc5_mock_device, check_vc5_encoder_constraints, __VA_ARGS__)
    static const struct pv_muxing_param vc4_test_pv_muxing_params[] = {
    VC4_PV_MUXING_TEST("1 output: DSI0",
    VC4_ENCODER_TYPE_DSI0),
    VC4_PV_MUXING_TEST("1 output: DPI",
    VC4_ENCODER_TYPE_DPI),
    VC4_PV_MUXING_TEST("1 output: HDMI0",
    VC4_ENCODER_TYPE_HDMI0),
    VC4_PV_MUXING_TEST("1 output: VEC",
    VC4_ENCODER_TYPE_VEC),
    VC4_PV_MUXING_TEST("1 output: DSI1",
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("1 output: TXP",
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("2 outputs: DSI0, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0),
    VC4_PV_MUXING_TEST("2 outputs: DSI0, VEC",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC),
    VC4_PV_MUXING_TEST("2 outputs: DSI0, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("2 outputs: DSI0, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("2 outputs: DPI, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0),
    VC4_PV_MUXING_TEST("2 outputs: DPI, VEC",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC),
    VC4_PV_MUXING_TEST("2 outputs: DPI, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("2 outputs: DPI, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("2 outputs: HDMI0, DSI1",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("2 outputs: HDMI0, TXP",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("2 outputs: VEC, DSI1",
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("2 outputs: VEC, TXP",
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("3 outputs: DSI0, HDMI0, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("3 outputs: DSI0, HDMI0, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("3 outputs: DSI0, VEC, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("3 outputs: DSI0, VEC, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("3 outputs: DPI, HDMI0, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("3 outputs: DPI, HDMI0, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("3 outputs: DPI, VEC, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("3 outputs: DPI, VEC, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0),
    };
    KUNIT_ARRAY_PARAM(vc4_test_pv_muxing,
    vc4_test_pv_muxing_params,
    vc4_test_pv_muxing_desc);
    static const struct pv_muxing_param vc4_test_pv_muxing_invalid_params[] = {
    VC4_PV_MUXING_TEST("DPI/DSI0 Conflict",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI0),
    VC4_PV_MUXING_TEST("TXP/DSI1 Conflict",
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1),
    VC4_PV_MUXING_TEST("HDMI0/VEC Conflict",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_VEC),
    VC4_PV_MUXING_TEST("More than 3 outputs: DSI0, HDMI0, DSI1, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, DSI1, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("More than 3 outputs: DPI, HDMI0, DSI1, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_TXP0),
    VC4_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, DSI1, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_TXP0),
    };
    KUNIT_ARRAY_PARAM(vc4_test_pv_muxing_invalid,
    vc4_test_pv_muxing_invalid_params,
    vc4_test_pv_muxing_desc);
    static const struct pv_muxing_param vc5_test_pv_muxing_params[] = {
    VC5_PV_MUXING_TEST("1 output: DPI",
    VC4_ENCODER_TYPE_DPI),
    VC5_PV_MUXING_TEST("1 output: DSI0",
    VC4_ENCODER_TYPE_DSI0),
    VC5_PV_MUXING_TEST("1 output: DSI1",
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("1 output: HDMI0",
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("1 output: HDMI1",
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("1 output: VEC",
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: DPI, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("2 outputs: DPI, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("2 outputs: DPI, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("2 outputs: DPI, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("2 outputs: DPI, VEC",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: DPI, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, VEC",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: DSI0, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("2 outputs: DSI1, VEC",
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: DSI1, TXP",
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("2 outputs: DSI1, HDMI0",
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("2 outputs: DSI1, HDMI1",
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("2 outputs: HDMI0, VEC",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: HDMI0, TXP",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("2 outputs: HDMI0, HDMI1",
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("2 outputs: HDMI1, VEC",
    VC4_ENCODER_TYPE_HDMI1,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("2 outputs: HDMI1, TXP",
    VC4_ENCODER_TYPE_HDMI1,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("2 outputs: TXP, VEC",
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_VEC),
    VC5_PV_MUXING_TEST("3 outputs: DPI, VEC, TXP",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("3 outputs: DPI, VEC, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("3 outputs: DPI, VEC, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DPI, VEC, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DPI, TXP, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("3 outputs: DPI, TXP, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DPI, TXP, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DPI, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DPI, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DPI, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, VEC, TXP",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, VEC, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, VEC, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, VEC, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, TXP, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, TXP, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, TXP, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("3 outputs: DSI0, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    };
    KUNIT_ARRAY_PARAM(vc5_test_pv_muxing,
    vc5_test_pv_muxing_params,
    vc4_test_pv_muxing_desc);
    static const struct pv_muxing_param vc5_test_pv_muxing_invalid_params[] = {
    VC5_PV_MUXING_TEST("DPI/DSI0 Conflict",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, DSI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, TXP, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, TXP, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, TXP, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, TXP, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, DSI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, TXP, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, TXP, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, TXP, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, DSI1, HDMI0",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, DSI1, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, TXP, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: VEC, TXP, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DPI, VEC, TXP, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    VC5_PV_MUXING_TEST("More than 3 outputs: DSI0, VEC, TXP, DSI1, HDMI0, HDMI1",
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1),
    };
    KUNIT_ARRAY_PARAM(vc5_test_pv_muxing_invalid,
    vc5_test_pv_muxing_invalid_params,
    vc4_test_pv_muxing_desc);
#[no_mangle]
unsafe extern "C" fn drm_vc4_test_pv_muxing(test: *mut kunit) {
    static void drm_vc4_test_pv_muxing(struct kunit *test)
    {
    const struct pv_muxing_param *params = test.param_value;
    const struct pv_muxing_priv *priv = test.priv;
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_commit *state;
    struct drm_device *drm;
    struct vc4_dev *vc4;
    unsigned int i;
    int ret;
    drm_modeset_acquire_init(&ctx, 0);
    vc4 = priv.vc4;
    drm = &vc4.base;
    retry:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    for (i = 0; i < params.nencoders; i++) {
    let mut enc_type: enum vc4_encoder_type = params.encoders[i];
    ret = vc4_mock_atomic_add_output(test, state, enc_type);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    }
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry;
    }
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test,
    check_fifo_conflict(test, state));
    for (i = 0; i < params.nencoders; i++) {
    let mut enc_type: enum vc4_encoder_type = params.encoders[i];
    KUNIT_EXPECT_TRUE(test, check_channel_for_encoder(test, state, enc_type,
    params.check_fn));
    }
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
#[no_mangle]
unsafe extern "C" fn drm_vc4_test_pv_muxing_invalid(test: *mut kunit) {
    static void drm_vc4_test_pv_muxing_invalid(struct kunit *test)
    {
    const struct pv_muxing_param *params = test.param_value;
    const struct pv_muxing_priv *priv = test.priv;
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_commit *state;
    struct drm_device *drm;
    struct vc4_dev *vc4;
    unsigned int i;
    int ret;
    drm_modeset_acquire_init(&ctx, 0);
    vc4 = priv.vc4;
    drm = &vc4.base;
    retry:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    for (i = 0; i < params.nencoders; i++) {
    let mut enc_type: enum vc4_encoder_type = params.encoders[i];
    ret = vc4_mock_atomic_add_output(test, state, enc_type);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    }
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry;
    }
    KUNIT_EXPECT_LT(test, ret, 0);
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
#[no_mangle]
unsafe extern "C" fn vc4_pv_muxing_test_init(test: *mut kunit) -> c_int {
    static int vc4_pv_muxing_test_init(struct kunit *test)
    {
    const struct pv_muxing_param *params = test.param_value;
    struct pv_muxing_priv *priv;
    struct vc4_dev *vc4;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, priv);
    test.priv = priv;
    vc4 = params.mock_fn(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vc4);
    priv.vc4 = vc4;
    return 0;
    }
    static struct kunit_case vc4_pv_muxing_tests[] = {
    KUNIT_CASE_PARAM(drm_vc4_test_pv_muxing,
    vc4_test_pv_muxing_gen_params),
    KUNIT_CASE_PARAM(drm_vc4_test_pv_muxing_invalid,
    vc4_test_pv_muxing_invalid_gen_params),
    {}
    };
    static struct kunit_suite vc4_pv_muxing_test_suite = {
    .name = "vc4-pv-muxing-combinations",
    .init = vc4_pv_muxing_test_init,
    .test_cases = vc4_pv_muxing_tests,
    };
    static struct kunit_case vc5_pv_muxing_tests[] = {
    KUNIT_CASE_PARAM(drm_vc4_test_pv_muxing,
    vc5_test_pv_muxing_gen_params),
    KUNIT_CASE_PARAM(drm_vc4_test_pv_muxing_invalid,
    vc5_test_pv_muxing_invalid_gen_params),
    {}
    };
    static struct kunit_suite vc5_pv_muxing_test_suite = {
    .name = "vc5-pv-muxing-combinations",
    .init = vc4_pv_muxing_test_init,
    .test_cases = vc5_pv_muxing_tests,
    };
// See
// https://lore.kernel.org/all/3e113525-aa89-b1e2-56b7-ca55bd41d057@samsung.com
// and
// https://lore.kernel.org/dri-devel/20200917121623.42023-1-maxime@cerno.tech
//
#[no_mangle]
unsafe extern "C" fn drm_test_vc5_pv_muxing_bugs_subsequent_crtc_enable(test: *mut kunit) {
    static void drm_test_vc5_pv_muxing_bugs_subsequent_crtc_enable(struct kunit *test)
    {
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_commit *state;
    struct vc4_crtc_state *new_vc4_crtc_state;
    struct vc4_hvs_state *new_hvs_state;
    unsigned int hdmi0_channel;
    unsigned int hdmi1_channel;
    struct drm_device *drm;
    struct vc4_dev *vc4;
    int ret;
    vc4 = vc5_mock_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vc4);
    drm_modeset_acquire_init(&ctx, 0);
    drm = &vc4.base;
    retry_first:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI0);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    new_hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_hvs_state);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI0);
    KUNIT_ASSERT_NOT_NULL(test, new_vc4_crtc_state);
    hdmi0_channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_ASSERT_NE(test, hdmi0_channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_ASSERT_TRUE(test, new_hvs_state.fifo_state[hdmi0_channel].in_use);
    ret = drm_atomic_helper_swap_state(state, false);
    KUNIT_ASSERT_EQ(test, ret, 0);
    retry_second:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI1);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    new_hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_hvs_state);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI1);
    KUNIT_ASSERT_NOT_NULL(test, new_vc4_crtc_state);
    hdmi1_channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_ASSERT_NE(test, hdmi1_channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_ASSERT_TRUE(test, new_hvs_state.fifo_state[hdmi1_channel].in_use);
    KUNIT_EXPECT_NE(test, hdmi0_channel, hdmi1_channel);
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
//
// This test makes sure that we never change the FIFO of an active HVS
// channel if we disable a FIFO with a lower index.
//
// Doing so would result in a FIFO stall and would disrupt an output
// supposed to be unaffected by the commit.
//
#[no_mangle]
unsafe extern "C" fn drm_test_vc5_pv_muxing_bugs_stable_fifo(test: *mut kunit) {
    static void drm_test_vc5_pv_muxing_bugs_stable_fifo(struct kunit *test)
    {
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_commit *state;
    struct vc4_crtc_state *new_vc4_crtc_state;
    struct vc4_hvs_state *new_hvs_state;
    unsigned int old_hdmi0_channel;
    unsigned int old_hdmi1_channel;
    struct drm_device *drm;
    struct vc4_dev *vc4;
    int ret;
    vc4 = vc5_mock_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vc4);
    drm_modeset_acquire_init(&ctx, 0);
    drm = &vc4.base;
    retry_first:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI0);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI1);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    new_hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_hvs_state);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI0);
    KUNIT_ASSERT_NOT_NULL(test, new_vc4_crtc_state);
    old_hdmi0_channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_ASSERT_NE(test, old_hdmi0_channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_ASSERT_TRUE(test, new_hvs_state.fifo_state[old_hdmi0_channel].in_use);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI1);
    KUNIT_ASSERT_NOT_NULL(test, new_vc4_crtc_state);
    old_hdmi1_channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_ASSERT_NE(test, old_hdmi1_channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_ASSERT_TRUE(test, new_hvs_state.fifo_state[old_hdmi1_channel].in_use);
    ret = drm_atomic_helper_swap_state(state, false);
    KUNIT_ASSERT_EQ(test, ret, 0);
    retry_second:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_del_output(test, state, VC4_ENCODER_TYPE_HDMI0);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    new_hvs_state = vc4_hvs_get_new_global_state(state);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, new_hvs_state);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI1);
    if (new_vc4_crtc_state) {
    unsigned int hdmi1_channel;
    hdmi1_channel = new_vc4_crtc_state.assigned_channel;
    KUNIT_ASSERT_NE(test, hdmi1_channel, VC4_HVS_CHANNEL_DISABLED);
    KUNIT_ASSERT_TRUE(test, new_hvs_state.fifo_state[hdmi1_channel].in_use);
    KUNIT_EXPECT_EQ(test, old_hdmi1_channel, hdmi1_channel);
    }
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
//
// Test that if we affect a single output, only the CRTC state of that
// output will be pulled in the global atomic state.
//
// This is relevant for two things:
//
// - If we don't have that state at all, we are unlikely to affect the
// FIFO muxing. This is somewhat redundant with
// drm_test_vc5_pv_muxing_bugs_stable_fifo()
//
// - KMS waits for page flips to occur on all the CRTC found in the
// CRTC state. Since the CRTC is unaffected, we would over-wait, but
// most importantly run into corner cases like waiting on an
// inactive CRTC that never completes.
//
    static void
    drm_test_vc5_pv_muxing_bugs_subsequent_crtc_enable_too_many_crtc_state(struct kunit *test)
    {
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_commit *state;
    struct vc4_crtc_state *new_vc4_crtc_state;
    struct drm_device *drm;
    struct vc4_dev *vc4;
    int ret;
    vc4 = vc5_mock_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vc4);
    drm_modeset_acquire_init(&ctx, 0);
    drm = &vc4.base;
    retry_first:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI0);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_first;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_helper_swap_state(state, false);
    KUNIT_ASSERT_EQ(test, ret, 0);
    retry_second:
    state = drm_kunit_helper_atomic_state_alloc(test, drm, &ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, state);
    ret = vc4_mock_atomic_add_output(test, state, VC4_ENCODER_TYPE_HDMI1);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = drm_atomic_check_only(state);
    if (ret == -EDEADLK) {
    drm_atomic_commit_clear(state);
    ret = drm_modeset_backoff(&ctx);
    if (!ret)
    goto retry_second;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    new_vc4_crtc_state = get_vc4_crtc_state_for_encoder(test, state,
    VC4_ENCODER_TYPE_HDMI0);
    KUNIT_EXPECT_NULL(test, new_vc4_crtc_state);
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
    static struct kunit_case vc5_pv_muxing_bugs_tests[] = {
    KUNIT_CASE(drm_test_vc5_pv_muxing_bugs_subsequent_crtc_enable),
    KUNIT_CASE(drm_test_vc5_pv_muxing_bugs_subsequent_crtc_enable_too_many_crtc_state),
    KUNIT_CASE(drm_test_vc5_pv_muxing_bugs_stable_fifo),
    {}
    };
    static struct kunit_suite vc5_pv_muxing_bugs_test_suite = {
    .name = "vc5-pv-muxing-bugs",
    .test_cases = vc5_pv_muxing_bugs_tests,
    };
    kunit_test_suites(
    &vc4_pv_muxing_test_suite,
    &vc5_pv_muxing_test_suite,
    &vc5_pv_muxing_bugs_test_suite
    );
