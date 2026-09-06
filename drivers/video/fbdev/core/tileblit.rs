//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/tileblit.c
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
// linux/drivers/video/console/tileblit.c -- Tile Blitting Operation
//
// Copyright (C) 2004 Antonino Daplas <adaplas @pol.net>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

    static void tile_bmove(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int dy, int dx, int height, int width)
    {
    struct fb_tilearea area;
    area.sx = sx;
    area.sy = sy;
    area.dx = dx;
    area.dy = dy;
    area.height = height;
    area.width = width;
    info.tileops.fb_tilecopy(info, &area);
    }
    static void tile_clear(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int height, int width, int fg, int bg)
    {
    struct fb_tilerect rect;
    rect.index = vc.vc_video_erase_char &
    ((vc.vc_hi_font_mask) ? 0x1ff : 0xff);
    rect.fg = fg;
    rect.bg = bg;
    rect.sx = sx;
    rect.sy = sy;
    rect.width = width;
    rect.height = height;
    rect.rop = ROP_COPY;
    info.tileops.fb_tilefill(info, &rect);
    }
    static void tile_putcs(struct vc_data *vc, struct fb_info *info,
    const unsigned short *s, int count, int yy, int xx,
    int fg, int bg)
    {
    struct fb_tileblit blit;
    let mut charmask: c_ushort = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut size: c_int = sizeof(u32) * count, i;
    blit.sx = xx;
    blit.sy = yy;
    blit.width = count;
    blit.height = 1;
    blit.fg = fg;
    blit.bg = bg;
    blit.length = count;
    blit.indices = (u32 *) fb_get_buffer_offset(info, &info.pixmap, size);
    for (i = 0; i < count; i++)
    blit.indices[i] = (u32)(scr_readw(s++) & charmask);
    info.tileops.fb_tileblit(info, &blit);
    }
    static void tile_clear_margins(struct vc_data *vc, struct fb_info *info,
    int color, int bottom_only)
    {
    let mut cw: c_uint = vc.vc_font.width;
    let mut ch: c_uint = vc.vc_font.height;
    let mut rw: c_uint = info.var.xres - (vc.vc_cols*cw);
    let mut bh: c_uint = info.var.yres - (vc.vc_rows*ch);
    let mut rs: c_uint = info.var.xres - rw;
    let mut bs: c_uint = info.var.yres - bh;
    let mut vwt: c_uint = info.var.xres_virtual / cw;
    let mut vht: c_uint = info.var.yres_virtual / ch;
    struct fb_tilerect rect;
    rect.index = vc.vc_video_erase_char &
    ((vc.vc_hi_font_mask) ? 0x1ff : 0xff);
    rect.fg = color;
    rect.bg = color;
    if ((int) rw > 0 && !bottom_only) {
    rect.sx = (info.var.xoffset + rs + cw - 1) / cw;
    rect.sy = 0;
    rect.width = (rw + cw - 1) / cw;
    rect.height = vht;
    if (rect.width + rect.sx > vwt)
    rect.width = vwt - rect.sx;
    if (rect.sx < vwt)
    info.tileops.fb_tilefill(info, &rect);
    }
    if ((int) bh > 0) {
    rect.sx = info.var.xoffset / cw;
    rect.sy = (info.var.yoffset + bs) / ch;
    rect.width = rs / cw;
    rect.height = (bh + ch - 1) / ch;
    if (rect.height + rect.sy > vht)
    rect.height = vht - rect.sy;
    if (rect.sy < vht)
    info.tileops.fb_tilefill(info, &rect);
    }
    }
    static void tile_cursor(struct vc_data *vc, struct fb_info *info, bool enable,
    int fg, int bg)
    {
    struct fb_tilecursor cursor;
    let mut use_sw: c_int = vc.vc_cursor_type & CUR_SW;
    cursor.sx = vc.state.x;
    cursor.sy = vc.state.y;
    cursor.mode = enable && !use_sw;
    cursor.fg = fg;
    cursor.bg = bg;
    switch (vc.vc_cursor_type & 0x0f) {
    case CUR_NONE:
    cursor.shape = FB_TILE_CURSOR_NONE;
    break;
    case CUR_UNDERLINE:
    cursor.shape = FB_TILE_CURSOR_UNDERLINE;
    break;
    case CUR_LOWER_THIRD:
    cursor.shape = FB_TILE_CURSOR_LOWER_THIRD;
    break;
    case CUR_LOWER_HALF:
    cursor.shape = FB_TILE_CURSOR_LOWER_HALF;
    break;
    case CUR_TWO_THIRDS:
    cursor.shape = FB_TILE_CURSOR_TWO_THIRDS;
    break;
    case CUR_BLOCK:
    default:
    cursor.shape = FB_TILE_CURSOR_BLOCK;
    break;
    }
    info.tileops.fb_tilecursor(info, &cursor);
    }
#[no_mangle]
unsafe extern "C" fn tile_update_start(info: *mut fb_info) -> c_int {
    static int tile_update_start(struct fb_info *info)
    {
    struct fbcon_par *par = info.fbcon_par;
    int err;
    err = fb_pan_display(info, &par.var);
    par.var.xoffset = info.var.xoffset;
    par.var.yoffset = info.var.yoffset;
    par.var.vmode = info.var.vmode;
    return err;
    }
    static const struct fbcon_bitops tile_fbcon_bitops = {
    .bmove = tile_bmove,
    .clear = tile_clear,
    .putcs = tile_putcs,
    .clear_margins = tile_clear_margins,
    .cursor = tile_cursor,
    .update_start = tile_update_start,
    };
#[no_mangle]
pub unsafe extern "C" fn fbcon_set_tileops(vc: *mut vc_data, info: *mut fb_info) {
    void fbcon_set_tileops(struct vc_data *vc, struct fb_info *info)
    {
    struct fb_tilemap map;
    struct fbcon_par *par = info.fbcon_par;
    par.bitops = &tile_fbcon_bitops;
    if (par.p) {
    map.width = vc.vc_font.width;
    map.height = vc.vc_font.height;
    map.depth = 1;
    map.length = vc.vc_font.charcount;
    map.data = par.p.fontdata;
    info.tileops.fb_settile(info, &map);
    }
    }
