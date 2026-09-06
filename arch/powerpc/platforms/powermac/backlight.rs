//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/backlight.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Miscellaneous procedures for dealing with the PowerMac hardware.
// Contains support for the backlight.
//
// Copyright (C) 2000 Benjamin Herrenschmidt
// Copyright (C) 2006 Michael Hanselmann <linux-kernel@hansmi.ch>
//

pub const OLD_BACKLIGHT_MAX: c_int = 15;
    static void pmac_backlight_key_worker(struct work_struct *work);
    static void pmac_backlight_set_legacy_worker(struct work_struct *work);
    static DECLARE_WORK(pmac_backlight_key_work, pmac_backlight_key_worker);
    static DECLARE_WORK(pmac_backlight_set_legacy_work, pmac_backlight_set_legacy_worker);
// Although these variables are used in interrupt context, it makes no sense to
// protect them. No user is able to produce enough key events per second and
// notice the errors that might happen.
//
    static int pmac_backlight_key_queued;
    static int pmac_backlight_set_legacy_queued;
// The via-pmu code allows the backlight to be grabbed, in which case the
// in-kernel control of the brightness needs to be disabled. This should
// only be used by really old PowerBooks.
//
    let mut kernel_backlight_disabled: static atomic_t = ATOMIC_INIT(0);
// Protect the pmac_backlight variable below.
    You should hold this lock when using the pmac_backlight pointer to
    prevent its potential removal. */
    DEFINE_MUTEX(pmac_backlight_mutex);
// Main backlight storage
//
// Backlight drivers in this variable are required to have the "ops"
// attribute set and to have an update_status function.
//
// We can only store one backlight here, but since Apple laptops have only one
// internal display, it doesn't matter. Other backlight drivers can be used
// independently.
//
    struct backlight_device *pmac_backlight;
#[no_mangle]
pub unsafe extern "C" fn pmac_has_backlight_type(type: *const c_char) -> c_int {
    int pmac_has_backlight_type(const char *type)
    {
    let mut bk_node: *mut device_node = of_find_node_by_name(core::ptr::null_mut(), "backlight");
    let mut i: c_int = of_property_match_string(bk_node, "backlight-control", type);
    of_node_put(bk_node);
    return i >= 0;
    }
#[no_mangle]
unsafe extern "C" fn pmac_backlight_key_worker(work: *mut work_struct) {
    static void pmac_backlight_key_worker(struct work_struct *work)
    {
    if (atomic_read(&kernel_backlight_disabled))
    return;
    mutex_lock(&pmac_backlight_mutex);
    if (pmac_backlight) {
    struct backlight_properties *props;
    int brightness;
    props = &pmac_backlight.props;
    brightness = props.brightness +
    ((pmac_backlight_key_queued?-1:1) *
    (props.max_brightness / 15));
    if (brightness < 0)
    brightness = 0;
#[no_mangle]
pub unsafe extern "C" fn if(props->max_brightness: brightness >) -> else {
    else if (brightness > props.max_brightness)
    brightness = props.max_brightness;
    props.brightness = brightness;
    backlight_update_status(pmac_backlight);
    }
    mutex_unlock(&pmac_backlight_mutex);
    }
// This function is called in interrupt context
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_key(direction: c_int) {
    void pmac_backlight_key(int direction)
    {
    if (atomic_read(&kernel_backlight_disabled))
    return;
// we can receive multiple interrupts here, but the scheduled work
// will run only once, with the last value
//
    pmac_backlight_key_queued = direction;
    schedule_work(&pmac_backlight_key_work);
    }
#[no_mangle]
unsafe extern "C" fn __pmac_backlight_set_legacy_brightness(brightness: c_int) -> c_int {
    static int __pmac_backlight_set_legacy_brightness(int brightness)
    {
    let mut error: c_int = -ENXIO;
    mutex_lock(&pmac_backlight_mutex);
    if (pmac_backlight) {
    struct backlight_properties *props;
    props = &pmac_backlight.props;
    props.brightness = brightness *
    (props.max_brightness + 1) /
    (OLD_BACKLIGHT_MAX + 1);
    if (props.brightness > props.max_brightness)
    props.brightness = props.max_brightness;
#[no_mangle]
pub unsafe extern "C" fn if(0: props->brightness <) -> else {
    else if (props.brightness < 0)
    props.brightness = 0;
    backlight_update_status(pmac_backlight);
    error = 0;
    }
    mutex_unlock(&pmac_backlight_mutex);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pmac_backlight_set_legacy_worker(work: *mut work_struct) {
    static void pmac_backlight_set_legacy_worker(struct work_struct *work)
    {
    if (atomic_read(&kernel_backlight_disabled))
    return;
    __pmac_backlight_set_legacy_brightness(pmac_backlight_set_legacy_queued);
    }
// This function is called in interrupt context
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_set_legacy_brightness_pmu(brightness: c_int) {
    if (atomic_read(&kernel_backlight_disabled))
    return;
    pmac_backlight_set_legacy_queued = brightness;
    schedule_work(&pmac_backlight_set_legacy_work);
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_set_legacy_brightness(brightness: c_int) -> c_int {
    int pmac_backlight_set_legacy_brightness(int brightness)
    {
    return __pmac_backlight_set_legacy_brightness(brightness);
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_get_legacy_brightness() -> c_int {
    int pmac_backlight_get_legacy_brightness(void)
    {
    let mut result: c_int = -ENXIO;
    mutex_lock(&pmac_backlight_mutex);
    if (pmac_backlight) {
    struct backlight_properties *props;
    props = &pmac_backlight.props;
    result = props.brightness *
    (OLD_BACKLIGHT_MAX + 1) /
    (props.max_brightness + 1);
    }
    mutex_unlock(&pmac_backlight_mutex);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_disable() {
    void pmac_backlight_disable(void)
    {
    atomic_inc(&kernel_backlight_disabled);
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_enable() {
    void pmac_backlight_enable(void)
    {
    atomic_dec(&kernel_backlight_disabled);
    }
    EXPORT_SYMBOL_GPL(pmac_backlight);
    EXPORT_SYMBOL_GPL(pmac_backlight_mutex);
    EXPORT_SYMBOL_GPL(pmac_has_backlight_type);
