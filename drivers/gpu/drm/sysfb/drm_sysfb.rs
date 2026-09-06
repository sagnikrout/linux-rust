//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sysfb/drm_sysfb.c
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

    int drm_sysfb_get_validated_int(struct drm_device *dev, const char *name,
    u64 value, u32 max)
    {
    if (value > min(max, INT_MAX)) {
    drm_warn(dev, "%s of %llu exceeds maximum of %u\n", name, value, max);
    return -EINVAL;
    }
    return value;
    }
    EXPORT_SYMBOL(drm_sysfb_get_validated_int);
    int drm_sysfb_get_validated_int0(struct drm_device *dev, const char *name,
    u64 value, u32 max)
    {
    if (!value) {
    drm_warn(dev, "%s of 0 not allowed\n", name);
    return -EINVAL;
    }
    return drm_sysfb_get_validated_int(dev, name, value, max);
    }
    EXPORT_SYMBOL(drm_sysfb_get_validated_int0);
    const struct drm_format_info *drm_sysfb_get_format(struct drm_device *dev,
    const struct drm_sysfb_format *formats,
    size_t nformats,
    const struct pixel_format *pixel)
    {
    const struct drm_format_info *format = core::ptr::null_mut();
    size_t i;
    for (i = 0; i < nformats; ++i) {
    const struct drm_sysfb_format *f = &formats[i];
    if (pixel_format_equal(pixel, &f.pixel)) {
    format = drm_format_info(f.fourcc);
    break;
    }
    }
    if (!format)
    drm_warn(dev, "No compatible color format found\n");
    return format;
    }
    EXPORT_SYMBOL(drm_sysfb_get_format);
    MODULE_DESCRIPTION("Helpers for DRM sysfb drivers");
    MODULE_LICENSE("GPL");
