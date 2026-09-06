//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_draw.c
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


// SPDX-License-Identifier: GPL-2.0 or MIT
//
// Copyright (c) 2023 Red Hat.
// Author: Jocelyn Falempe <jfalempe@redhat.com>
//

//
// drm_draw_can_convert_from_xrgb8888 - check if xrgb8888 can be converted to the desired format
// @format: format
//
// Returns:
// True if XRGB8888 can be converted to the specified format, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn drm_draw_can_convert_from_xrgb8888(format: u32) -> bool {
    bool drm_draw_can_convert_from_xrgb8888(u32 format)
    {
    switch (format) {
    case DRM_FORMAT_RGB565:
    case DRM_FORMAT_RGBA5551:
    case DRM_FORMAT_XRGB1555:
    case DRM_FORMAT_ARGB1555:
    case DRM_FORMAT_RGB888:
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_ARGB8888:
    case DRM_FORMAT_XBGR8888:
    case DRM_FORMAT_ABGR8888:
    case DRM_FORMAT_XRGB2101010:
    case DRM_FORMAT_ARGB2101010:
    case DRM_FORMAT_ABGR2101010:
    return true;
    default:
    return false;
    }
    }
    EXPORT_SYMBOL(drm_draw_can_convert_from_xrgb8888);
//
// drm_draw_color_from_xrgb8888 - convert one pixel from xrgb8888 to the desired format
// @color: input color, in xrgb8888 format
// @format: output format
//
// Returns:
// Color in the format specified, casted to u32.
// Or 0 if the format is not supported.
//
#[no_mangle]
pub unsafe extern "C" fn drm_draw_color_from_xrgb8888(color: u32, format: u32) -> u32 {
    u32 drm_draw_color_from_xrgb8888(u32 color, u32 format)
    {
    switch (format) {
    case DRM_FORMAT_RGB565:
    return drm_pixel_xrgb8888_to_rgb565(color);
    case DRM_FORMAT_RGBA5551:
    return drm_pixel_xrgb8888_to_rgba5551(color);
    case DRM_FORMAT_XRGB1555:
    return drm_pixel_xrgb8888_to_xrgb1555(color);
    case DRM_FORMAT_ARGB1555:
    return drm_pixel_xrgb8888_to_argb1555(color);
    case DRM_FORMAT_RGB888:
    case DRM_FORMAT_XRGB8888:
    return color;
    case DRM_FORMAT_ARGB8888:
    return drm_pixel_xrgb8888_to_argb8888(color);
    case DRM_FORMAT_XBGR8888:
    return drm_pixel_xrgb8888_to_xbgr8888(color);
    case DRM_FORMAT_ABGR8888:
    return drm_pixel_xrgb8888_to_abgr8888(color);
    case DRM_FORMAT_XRGB2101010:
    return drm_pixel_xrgb8888_to_xrgb2101010(color);
    case DRM_FORMAT_ARGB2101010:
    return drm_pixel_xrgb8888_to_argb2101010(color);
    case DRM_FORMAT_ABGR2101010:
    return drm_pixel_xrgb8888_to_abgr2101010(color);
    default:
    WARN_ONCE(1, "Can't convert to %p4cc\n", &format);
    return 0;
    }
    }
    EXPORT_SYMBOL(drm_draw_color_from_xrgb8888);
//
// Blit functions
//
    void drm_draw_blit16(struct iosys_map *dmap, unsigned int dpitch,
    const u8 *sbuf8, unsigned int spitch,
    unsigned int height, unsigned int width,
    unsigned int scale, u16 fg16)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++)
    for (x = 0; x < width; x++)
    if (drm_draw_is_pixel_fg(sbuf8, spitch, x / scale, y / scale))
    iosys_map_wr(dmap, y * dpitch + x * sizeof(u16), u16, fg16);
    }
    EXPORT_SYMBOL(drm_draw_blit16);
    void drm_draw_blit24(struct iosys_map *dmap, unsigned int dpitch,
    const u8 *sbuf8, unsigned int spitch,
    unsigned int height, unsigned int width,
    unsigned int scale, u32 fg32)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++) {
    for (x = 0; x < width; x++) {
    let mut off: u32 = y * dpitch + x * 3;
    if (drm_draw_is_pixel_fg(sbuf8, spitch, x / scale, y / scale)) {
// write blue-green-red to output in little endianness
    iosys_map_wr(dmap, off, u8, (fg32 & 0x000000FF) >> 0);
    iosys_map_wr(dmap, off + 1, u8, (fg32 & 0x0000FF00) >> 8);
    iosys_map_wr(dmap, off + 2, u8, (fg32 & 0x00FF0000) >> 16);
    }
    }
    }
    }
    EXPORT_SYMBOL(drm_draw_blit24);
    void drm_draw_blit32(struct iosys_map *dmap, unsigned int dpitch,
    const u8 *sbuf8, unsigned int spitch,
    unsigned int height, unsigned int width,
    unsigned int scale, u32 fg32)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++)
    for (x = 0; x < width; x++)
    if (drm_draw_is_pixel_fg(sbuf8, spitch, x / scale, y / scale))
    iosys_map_wr(dmap, y * dpitch + x * sizeof(u32), u32, fg32);
    }
    EXPORT_SYMBOL(drm_draw_blit32);
//
// Fill functions
//
    void drm_draw_fill16(struct iosys_map *dmap, unsigned int dpitch,
    unsigned int height, unsigned int width,
    u16 color)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++)
    for (x = 0; x < width; x++)
    iosys_map_wr(dmap, y * dpitch + x * sizeof(u16), u16, color);
    }
    EXPORT_SYMBOL(drm_draw_fill16);
    void drm_draw_fill24(struct iosys_map *dmap, unsigned int dpitch,
    unsigned int height, unsigned int width,
    u32 color)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++) {
    for (x = 0; x < width; x++) {
    let mut off: c_uint = y * dpitch + x * 3;
// write blue-green-red to output in little endianness
    iosys_map_wr(dmap, off, u8, (color & 0x000000FF) >> 0);
    iosys_map_wr(dmap, off + 1, u8, (color & 0x0000FF00) >> 8);
    iosys_map_wr(dmap, off + 2, u8, (color & 0x00FF0000) >> 16);
    }
    }
    }
    EXPORT_SYMBOL(drm_draw_fill24);
    void drm_draw_fill32(struct iosys_map *dmap, unsigned int dpitch,
    unsigned int height, unsigned int width,
    u32 color)
    {
    unsigned int y, x;
    for (y = 0; y < height; y++)
    for (x = 0; x < width; x++)
    iosys_map_wr(dmap, y * dpitch + x * sizeof(u32), u32, color);
    }
    EXPORT_SYMBOL(drm_draw_fill32);
