//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/backlight.c
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
// GMA500 Backlight Interface
//
// Copyright (c) 2009-2011, Intel Corporation.
//
// Authors: Eric Knopp
//

#[no_mangle]
pub unsafe extern "C" fn gma_backlight_enable(dev: *mut drm_device) {
    void gma_backlight_enable(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    dev_priv.backlight_enabled = true;
    dev_priv.ops.backlight_set(dev, dev_priv.backlight_level);
    }
#[no_mangle]
pub unsafe extern "C" fn gma_backlight_disable(dev: *mut drm_device) {
    void gma_backlight_disable(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    dev_priv.backlight_enabled = false;
    dev_priv.ops.backlight_set(dev, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn gma_backlight_set(dev: *mut drm_device, v: c_int) {
    void gma_backlight_set(struct drm_device *dev, int v)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    dev_priv.backlight_level = v;
    if (dev_priv.backlight_enabled)
    dev_priv.ops.backlight_set(dev, v);
    }
#[no_mangle]
unsafe extern "C" fn gma_backlight_get_brightness(bd: *mut backlight_device) -> c_int {
    static int gma_backlight_get_brightness(struct backlight_device *bd)
    {
    struct drm_device *dev = bl_get_data(bd);
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    if (dev_priv.ops.backlight_get)
    return dev_priv.ops.backlight_get(dev);
    return dev_priv.backlight_level;
    }
#[no_mangle]
unsafe extern "C" fn gma_backlight_update_status(bd: *mut backlight_device) -> c_int {
    static int gma_backlight_update_status(struct backlight_device *bd)
    {
    struct drm_device *dev = bl_get_data(bd);
    let mut level: c_int = backlight_get_brightness(bd);
// Percentage 1-100% being valid
    if (level < 1)
    level = 1;
    gma_backlight_set(dev, level);
    return 0;
    }
    static const struct backlight_ops gma_backlight_ops __maybe_unused = {
    .get_brightness = gma_backlight_get_brightness,
    .update_status  = gma_backlight_update_status,
    };
#[no_mangle]
pub unsafe extern "C" fn gma_backlight_init(dev: *mut drm_device) -> c_int {
    int gma_backlight_init(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    let mut __maybe_unused: backlight_properties props = {};
    int ret;
    dev_priv.backlight_enabled = true;
    dev_priv.backlight_level = 100;
    ret = dev_priv.ops.backlight_init(dev);
    if (ret)
    return ret;
    if (!acpi_video_backlight_use_native()) {
    drm_info(dev, "Skipping %s backlight registration\n",
    dev_priv.ops.backlight_name);
    return 0;
    }

    props.brightness = dev_priv.backlight_level;
    props.max_brightness = PSB_MAX_BRIGHTNESS;
    props.type = BACKLIGHT_RAW;
    dev_priv.backlight_device =
    backlight_device_register(dev_priv.ops.backlight_name,
    dev.dev, dev,
    &gma_backlight_ops, &props);
    if (IS_ERR(dev_priv.backlight_device))
    return PTR_ERR(dev_priv.backlight_device);

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gma_backlight_exit(dev: *mut drm_device) {
    void gma_backlight_exit(struct drm_device *dev)
    {

    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    if (dev_priv.backlight_device)
    backlight_device_unregister(dev_priv.backlight_device);

    }
