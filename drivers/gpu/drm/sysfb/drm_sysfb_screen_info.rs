//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sysfb/drm_sysfb_screen_info.c
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

    static s64 drm_sysfb_get_validated_size0(struct drm_device *dev, const char *name,
    u64 value, u64 max)
    {
    if (!value) {
    drm_warn(dev, "%s of 0 not allowed\n", name);
    return -EINVAL;
    } else if (value > min(max, S64_MAX)) {
    drm_warn(dev, "%s of %llu exceeds maximum of %llu\n", name, value, max);
    return -EINVAL;
    }
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn drm_sysfb_get_width_si(dev: *mut drm_device, si: *const screen_info) -> c_int {
    int drm_sysfb_get_width_si(struct drm_device *dev, const struct screen_info *si)
    {
    return drm_sysfb_get_validated_int0(dev, "width", si.lfb_width, U16_MAX);
    }
    EXPORT_SYMBOL(drm_sysfb_get_width_si);
#[no_mangle]
pub unsafe extern "C" fn drm_sysfb_get_height_si(dev: *mut drm_device, si: *const screen_info) -> c_int {
    int drm_sysfb_get_height_si(struct drm_device *dev, const struct screen_info *si)
    {
    return drm_sysfb_get_validated_int0(dev, "height", si.lfb_height, U16_MAX);
    }
    EXPORT_SYMBOL(drm_sysfb_get_height_si);
    struct resource *drm_sysfb_get_memory_si(struct drm_device *dev,
    const struct screen_info *si,
    struct resource *res)
    {
    ssize_t	num;
    num = screen_info_resources(si, res, 1);
    if (!num) {
    drm_warn(dev, "memory resource not found\n");
    return core::ptr::null_mut();
    }
    return res;
    }
    EXPORT_SYMBOL(drm_sysfb_get_memory_si);
    int drm_sysfb_get_stride_si(struct drm_device *dev, const struct screen_info *si,
    const struct drm_format_info *format,
    unsigned int width, unsigned int height, u64 size)
    {
    let mut lfb_linelength: u64 = si.lfb_linelength;
    s64 stride;
    if (!lfb_linelength)
    lfb_linelength = drm_format_info_min_pitch(format, 0, width);
    stride = drm_sysfb_get_validated_size0(dev, "stride", lfb_linelength,
    div64_u64(size, height));
    if (stride < INT_MIN || stride > INT_MAX)
    return -EINVAL;
    return (int)stride; /* stride or negative errno code */
    }
    EXPORT_SYMBOL(drm_sysfb_get_stride_si);
    s64 drm_sysfb_get_visible_size_si(struct drm_device *dev, const struct screen_info *si,
    unsigned int height, unsigned int stride, u64 size)
    {
    let mut vsize: u64 = mul_u32_u32(height, stride);
    return drm_sysfb_get_validated_size0(dev, "visible size", vsize, size);
    }
    EXPORT_SYMBOL(drm_sysfb_get_visible_size_si);
