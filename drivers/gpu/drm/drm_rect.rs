//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_rect.c
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


//
// Copyright (C) 2011-2013 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// drm_rect_intersect - intersect two rectangles
// @r1: first rectangle
// @r2: second rectangle
//
// Calculate the intersection of rectangles @r1 and @r2.
// @r1 will be overwritten with the intersection.
//
// RETURNS:
// %true if rectangle @r1 is still visible after the operation,
// %false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn drm_rect_intersect(r1: *mut drm_rect, r2: *const drm_rect) -> bool {
    bool drm_rect_intersect(struct drm_rect *r1, const struct drm_rect *r2)
    {
    r1.x1 = max(r1.x1, r2.x1);
    r1.y1 = max(r1.y1, r2.y1);
    r1.x2 = min(r1.x2, r2.x2);
    r1.y2 = min(r1.y2, r2.y2);
    return drm_rect_visible(r1);
    }
    EXPORT_SYMBOL(drm_rect_intersect);
#[no_mangle]
unsafe extern "C" fn clip_scaled(src: c_int, dst: c_int, clip: *mut c_int) -> u32 {
    static u32 clip_scaled(int src, int dst, int *clip)
    {
    u64 tmp;
    if (dst == 0)
    return 0;
// Only clip what we have. Keeps the result bounded.
// clip = min(*clip, dst);
    tmp = mul_u32_u32(src, dst - *clip);
//
// Round toward 1.0 when clipping so that we don't accidentally
// change upscaling to downscaling or vice versa.
//
    if (src < (dst << 16))
    return DIV_ROUND_UP_ULL(tmp, dst);
    else
    return DIV_ROUND_DOWN_ULL(tmp, dst);
    }
//
// drm_rect_clip_scaled - perform a scaled clip operation
// @src: source window rectangle
// @dst: destination window rectangle
// @clip: clip rectangle
//
// Clip rectangle @dst by rectangle @clip. Clip rectangle @src by
// the corresponding amounts, retaining the vertical and horizontal scaling
// factors from @src to @dst.
//
// RETURNS:
// %true if rectangle @dst is still visible after being clipped,
// %false otherwise.
//
    bool drm_rect_clip_scaled(struct drm_rect *src, struct drm_rect *dst,
    const struct drm_rect *clip)
    {
    int diff;
    diff = clip.x1 - dst.x1;
    if (diff > 0) {
    u32 new_src_w = clip_scaled(drm_rect_width(src),
    drm_rect_width(dst), &diff);
    src.x1 = src.x2 - new_src_w;
    dst.x1 += diff;
    }
    diff = clip.y1 - dst.y1;
    if (diff > 0) {
    u32 new_src_h = clip_scaled(drm_rect_height(src),
    drm_rect_height(dst), &diff);
    src.y1 = src.y2 - new_src_h;
    dst.y1 += diff;
    }
    diff = dst.x2 - clip.x2;
    if (diff > 0) {
    u32 new_src_w = clip_scaled(drm_rect_width(src),
    drm_rect_width(dst), &diff);
    src.x2 = src.x1 + new_src_w;
    dst.x2 -= diff;
    }
    diff = dst.y2 - clip.y2;
    if (diff > 0) {
    u32 new_src_h = clip_scaled(drm_rect_height(src),
    drm_rect_height(dst), &diff);
    src.y2 = src.y1 + new_src_h;
    dst.y2 -= diff;
    }
    return drm_rect_visible(dst);
    }
    EXPORT_SYMBOL(drm_rect_clip_scaled);
#[no_mangle]
unsafe extern "C" fn drm_calc_scale(src: c_int, dst: c_int) -> c_int {
    static int drm_calc_scale(int src, int dst)
    {
    let mut scale: c_int = 0;
    if (WARN_ON(src < 0 || dst < 0))
    return -EINVAL;
    if (dst == 0)
    return 0;
    if (src > (dst << 16))
    return DIV_ROUND_UP(src, dst);
    else
    scale = src / dst;
    return scale;
    }
//
// drm_rect_calc_hscale - calculate the horizontal scaling factor
// @src: source window rectangle
// @dst: destination window rectangle
// @min_hscale: minimum allowed horizontal scaling factor
// @max_hscale: maximum allowed horizontal scaling factor
//
// Calculate the horizontal scaling factor as
// (@src width) / (@dst width).
//
// If the scale is below 1 << 16, round down. If the scale is above
// 1 << 16, round up. This will calculate the scale with the most
// pessimistic limit calculation.
//
// RETURNS:
// The horizontal scaling factor, or errno of out of limits.
//
    int drm_rect_calc_hscale(const struct drm_rect *src,
    const struct drm_rect *dst,
    int min_hscale, int max_hscale)
    {
    let mut src_w: c_int = drm_rect_width(src);
    let mut dst_w: c_int = drm_rect_width(dst);
    let mut hscale: c_int = drm_calc_scale(src_w, dst_w);
    if (hscale < 0 || dst_w == 0)
    return hscale;
    if (hscale < min_hscale || hscale > max_hscale)
    return -ERANGE;
    return hscale;
    }
    EXPORT_SYMBOL(drm_rect_calc_hscale);
//
// drm_rect_calc_vscale - calculate the vertical scaling factor
// @src: source window rectangle
// @dst: destination window rectangle
// @min_vscale: minimum allowed vertical scaling factor
// @max_vscale: maximum allowed vertical scaling factor
//
// Calculate the vertical scaling factor as
// (@src height) / (@dst height).
//
// If the scale is below 1 << 16, round down. If the scale is above
// 1 << 16, round up. This will calculate the scale with the most
// pessimistic limit calculation.
//
// RETURNS:
// The vertical scaling factor, or errno of out of limits.
//
    int drm_rect_calc_vscale(const struct drm_rect *src,
    const struct drm_rect *dst,
    int min_vscale, int max_vscale)
    {
    let mut src_h: c_int = drm_rect_height(src);
    let mut dst_h: c_int = drm_rect_height(dst);
    let mut vscale: c_int = drm_calc_scale(src_h, dst_h);
    if (vscale < 0 || dst_h == 0)
    return vscale;
    if (vscale < min_vscale || vscale > max_vscale)
    return -ERANGE;
    return vscale;
    }
    EXPORT_SYMBOL(drm_rect_calc_vscale);
//
// drm_rect_debug_print - print the rectangle information
// @prefix: prefix string
// @r: rectangle to print
// @fixed_point: rectangle is in 16.16 fixed point format
//
#[no_mangle]
pub unsafe extern "C" fn drm_rect_debug_print(prefix: *const c_char, r: *const drm_rect, fixed_point: bool) {
    void drm_rect_debug_print(const char *prefix, const struct drm_rect *r, bool fixed_point)
    {
    if (fixed_point)
    DRM_DEBUG_KMS("%s" DRM_RECT_FP_FMT "\n", prefix, DRM_RECT_FP_ARG(r));
    else
    DRM_DEBUG_KMS("%s" DRM_RECT_FMT "\n", prefix, DRM_RECT_ARG(r));
    }
    EXPORT_SYMBOL(drm_rect_debug_print);
//
// drm_rect_rotate - Rotate the rectangle
// @r: rectangle to be rotated
// @width: Width of the coordinate space
// @height: Height of the coordinate space
// @rotation: Transformation to be applied
//
// Apply @rotation to the coordinates of rectangle @r.
//
// @width and @height combined with @rotation define
// the location of the new origin.
//
// @width correcsponds to the horizontal and @height
// to the vertical axis of the untransformed coordinate
// space.
//
    void drm_rect_rotate(struct drm_rect *r,
    int width, int height,
    unsigned int rotation)
    {
    struct drm_rect tmp;
    if (rotation & (DRM_MODE_REFLECT_X | DRM_MODE_REFLECT_Y)) {
    tmp = *r;
    if (rotation & DRM_MODE_REFLECT_X) {
    r.x1 = width - tmp.x2;
    r.x2 = width - tmp.x1;
    }
    if (rotation & DRM_MODE_REFLECT_Y) {
    r.y1 = height - tmp.y2;
    r.y2 = height - tmp.y1;
    }
    }
    switch (rotation & DRM_MODE_ROTATE_MASK) {
    case DRM_MODE_ROTATE_0:
    break;
    case DRM_MODE_ROTATE_90:
    tmp = *r;
    r.x1 = tmp.y1;
    r.x2 = tmp.y2;
    r.y1 = width - tmp.x2;
    r.y2 = width - tmp.x1;
    break;
    case DRM_MODE_ROTATE_180:
    tmp = *r;
    r.x1 = width - tmp.x2;
    r.x2 = width - tmp.x1;
    r.y1 = height - tmp.y2;
    r.y2 = height - tmp.y1;
    break;
    case DRM_MODE_ROTATE_270:
    tmp = *r;
    r.x1 = height - tmp.y2;
    r.x2 = height - tmp.y1;
    r.y1 = tmp.x1;
    r.y2 = tmp.x2;
    break;
    default:
    break;
    }
    }
    EXPORT_SYMBOL(drm_rect_rotate);
//
// drm_rect_rotate_inv - Inverse rotate the rectangle
// @r: rectangle to be rotated
// @width: Width of the coordinate space
// @height: Height of the coordinate space
// @rotation: Transformation whose inverse is to be applied
//
// Apply the inverse of @rotation to the coordinates
// of rectangle @r.
//
// @width and @height combined with @rotation define
// the location of the new origin.
//
// @width correcsponds to the horizontal and @height
// to the vertical axis of the original untransformed
// coordinate space, so that you never have to flip
// them when doing a rotatation and its inverse.
// That is, if you do ::
//
// drm_rect_rotate(&r, width, height, rotation);
// drm_rect_rotate_inv(&r, width, height, rotation);
//
// you will always get back the original rectangle.
//
    void drm_rect_rotate_inv(struct drm_rect *r,
    int width, int height,
    unsigned int rotation)
    {
    struct drm_rect tmp;
    switch (rotation & DRM_MODE_ROTATE_MASK) {
    case DRM_MODE_ROTATE_0:
    break;
    case DRM_MODE_ROTATE_90:
    tmp = *r;
    r.x1 = width - tmp.y2;
    r.x2 = width - tmp.y1;
    r.y1 = tmp.x1;
    r.y2 = tmp.x2;
    break;
    case DRM_MODE_ROTATE_180:
    tmp = *r;
    r.x1 = width - tmp.x2;
    r.x2 = width - tmp.x1;
    r.y1 = height - tmp.y2;
    r.y2 = height - tmp.y1;
    break;
    case DRM_MODE_ROTATE_270:
    tmp = *r;
    r.x1 = tmp.y1;
    r.x2 = tmp.y2;
    r.y1 = height - tmp.x2;
    r.y2 = height - tmp.x1;
    break;
    default:
    break;
    }
    if (rotation & (DRM_MODE_REFLECT_X | DRM_MODE_REFLECT_Y)) {
    tmp = *r;
    if (rotation & DRM_MODE_REFLECT_X) {
    r.x1 = width - tmp.x2;
    r.x2 = width - tmp.x1;
    }
    if (rotation & DRM_MODE_REFLECT_Y) {
    r.y1 = height - tmp.y2;
    r.y2 = height - tmp.y1;
    }
    }
    }
    EXPORT_SYMBOL(drm_rect_rotate_inv);
