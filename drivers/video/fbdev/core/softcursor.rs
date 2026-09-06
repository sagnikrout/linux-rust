//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/softcursor.c
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
// linux/drivers/video/console/softcursor.c
//
// Generic software cursor for frame buffer devices
//
// Created 14 Nov 2002 by James Simmons
//
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//

#[no_mangle]
pub unsafe extern "C" fn soft_cursor(info: *mut fb_info, cursor: *mut fb_cursor) -> c_int {
    int soft_cursor(struct fb_info *info, struct fb_cursor *cursor)
    {
    struct fbcon_par *par = info.fbcon_par;
    let mut scan_align: c_uint = info.pixmap.scan_align - 1;
    let mut buf_align: c_uint = info.pixmap.buf_align - 1;
    unsigned int i, size, dsize, s_pitch, d_pitch;
    struct fb_image *image;
    u8 *src, *dst;
    if (info.state != FBINFO_STATE_RUNNING)
    return 0;
    s_pitch = (cursor.image.width + 7) >> 3;
    dsize = s_pitch * cursor.image.height;
    if (dsize + sizeof(struct fb_image) != par.cursor_size) {
    kfree(par.cursor_src);
    par.cursor_size = dsize + sizeof(struct fb_image);
    par.cursor_src = kmalloc(par.cursor_size, GFP_ATOMIC);
    if (!par.cursor_src) {
    par.cursor_size = 0;
    return -ENOMEM;
    }
    }
    src = par.cursor_src + sizeof(struct fb_image);
    image = (struct fb_image *)par.cursor_src;
// image = cursor->image;
    d_pitch = (s_pitch + scan_align) & ~scan_align;
    size = d_pitch * image.height + buf_align;
    size &= ~buf_align;
    dst = fb_get_buffer_offset(info, &info.pixmap, size);
    if (cursor.enable) {
    switch (cursor.rop) {
    case ROP_XOR:
    for (i = 0; i < dsize; i++)
    src[i] = image.data[i] ^ cursor.mask[i];
    break;
    case ROP_COPY:
    default:
    for (i = 0; i < dsize; i++)
    src[i] = image.data[i] & cursor.mask[i];
    break;
    }
    } else
    memcpy(src, image.data, dsize);
    fb_pad_aligned_buffer(dst, d_pitch, src, s_pitch, image.height);
    image.data = dst;
    info.fbops.fb_imageblit(info, image);
    return 0;
    }
