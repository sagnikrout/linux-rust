//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_vblank_helper.c
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


// SPDX-License-Identifier: MIT

//
// DOC: overview
//
// The vblank helper library provides functions for supporting vertical
// blanking in DRM drivers.
//
// For vblank timers, several callback implementations are available.
// Drivers enable support for vblank timers by setting the vblank callbacks
// in struct &drm_crtc_funcs to the helpers provided by this library. The
// initializer macro DRM_CRTC_VBLANK_TIMER_FUNCS does this conveniently.
// The driver further has to send the VBLANK event from its atomic_flush
// callback and control vblank from the CRTC's atomic_enable and atomic_disable
// callbacks. The callbacks are located in struct &drm_crtc_helper_funcs.
// The vblank helper library provides implementations of these callbacks
// for drivers without further requirements. The initializer macro
// DRM_CRTC_HELPER_VBLANK_FUNCS sets them coveniently.
//
// Once the driver enables vblank support with drm_vblank_init(), each
// CRTC's vblank timer fires according to the programmed display mode. By
// default, the vblank timer invokes drm_crtc_handle_vblank(). Drivers with
// more specific requirements can set their own handler function in
// struct &drm_crtc_helper_funcs.handle_vblank_timeout.
//
// VBLANK helpers
//
// drm_crtc_vblank_atomic_flush -
// Implements struct &drm_crtc_helper_funcs.atomic_flush
// @crtc: The CRTC
// @state: The atomic state to apply
//
// The helper drm_crtc_vblank_atomic_flush() implements atomic_flush of
// struct drm_crtc_helper_funcs for CRTCs that only need to send out a
// VBLANK event.
//
// See also struct &drm_crtc_helper_funcs.atomic_flush.
//
    void drm_crtc_vblank_atomic_flush(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_device *dev = crtc.dev;
    struct drm_crtc_state *crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    struct drm_pending_vblank_event *event;
    spin_lock_irq(&dev.event_lock);
    event = crtc_state.event;
    crtc_state.event = core::ptr::null_mut();
    if (event) {
    if (drm_crtc_vblank_get(crtc) == 0)
    drm_crtc_arm_vblank_event(crtc, event);
    else
    drm_crtc_send_vblank_event(crtc, event);
    }
    spin_unlock_irq(&dev.event_lock);
    }
    EXPORT_SYMBOL(drm_crtc_vblank_atomic_flush);
//
// drm_crtc_vblank_atomic_enable - Implements struct &drm_crtc_helper_funcs.atomic_enable
// @crtc: The CRTC
// @state: The atomic state
//
// The helper drm_crtc_vblank_atomic_enable() implements atomic_enable
// of struct drm_crtc_helper_funcs for CRTCs the only need to enable VBLANKs.
//
// See also struct &drm_crtc_helper_funcs.atomic_enable.
//
    void drm_crtc_vblank_atomic_enable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    drm_crtc_vblank_on(crtc);
    }
    EXPORT_SYMBOL(drm_crtc_vblank_atomic_enable);
//
// drm_crtc_vblank_atomic_disable - Implements struct &drm_crtc_helper_funcs.atomic_disable
// @crtc: The CRTC
// @state: The atomic state
//
// The helper drm_crtc_vblank_atomic_disable() implements atomic_disable
// of struct drm_crtc_helper_funcs for CRTCs the only need to disable VBLANKs.
//
// See also struct &drm_crtc_funcs.atomic_disable.
//
    void drm_crtc_vblank_atomic_disable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    drm_crtc_vblank_off(crtc);
    }
    EXPORT_SYMBOL(drm_crtc_vblank_atomic_disable);
//
// VBLANK timer
//
// drm_crtc_vblank_helper_enable_vblank_timer - Implements struct &drm_crtc_funcs.enable_vblank
// @crtc: The CRTC
//
// The helper drm_crtc_vblank_helper_enable_vblank_timer() implements
// enable_vblank of struct drm_crtc_helper_funcs for CRTCs that require
// a VBLANK timer. It sets up the timer on the first invocation. The
// started timer expires after the current frame duration. See struct
// &drm_vblank_crtc.framedur_ns.
//
// See also struct &drm_crtc_helper_funcs.enable_vblank.
//
// Returns:
// 0 on success, or a negative errno code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn drm_crtc_vblank_helper_enable_vblank_timer(crtc: *mut drm_crtc) -> c_int {
    int drm_crtc_vblank_helper_enable_vblank_timer(struct drm_crtc *crtc)
    {
    return drm_crtc_vblank_start_timer(crtc);
    }
    EXPORT_SYMBOL(drm_crtc_vblank_helper_enable_vblank_timer);
//
// drm_crtc_vblank_helper_disable_vblank_timer - Implements struct &drm_crtc_funcs.disable_vblank
// @crtc: The CRTC
//
// The helper drm_crtc_vblank_helper_disable_vblank_timer() implements
// disable_vblank of struct drm_crtc_funcs for CRTCs that require a
// VBLANK timer.
//
// See also struct &drm_crtc_helper_funcs.disable_vblank.
//
#[no_mangle]
pub unsafe extern "C" fn drm_crtc_vblank_helper_disable_vblank_timer(crtc: *mut drm_crtc) {
    void drm_crtc_vblank_helper_disable_vblank_timer(struct drm_crtc *crtc)
    {
    drm_crtc_vblank_cancel_timer(crtc);
    }
    EXPORT_SYMBOL(drm_crtc_vblank_helper_disable_vblank_timer);
//
// drm_crtc_vblank_helper_get_vblank_timestamp_from_timer -
// Implements struct &drm_crtc_funcs.get_vblank_timestamp
// @crtc: The CRTC
// @max_error: Maximum acceptable error
// @vblank_time: Returns the next vblank timestamp
// @in_vblank_irq: True is called from drm_crtc_handle_vblank()
//
// The helper drm_crtc_helper_get_vblank_timestamp_from_timer() implements
// get_vblank_timestamp of struct drm_crtc_funcs for CRTCs that require a
// VBLANK timer. It returns the timestamp according to the timer's expiry
// time.
//
// See also struct &drm_crtc_funcs.get_vblank_timestamp.
//
// Returns:
// True on success, or false otherwise.
//
    bool drm_crtc_vblank_helper_get_vblank_timestamp_from_timer(struct drm_crtc *crtc,
    int *max_error,
    ktime_t *vblank_time,
    bool in_vblank_irq)
    {
    drm_crtc_vblank_get_vblank_timeout(crtc, vblank_time);
    return true;
    }
    EXPORT_SYMBOL(drm_crtc_vblank_helper_get_vblank_timestamp_from_timer);
