//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fbcon_ud.c
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
// linux/drivers/video/console/fbcon_ud.c -- Software Rotation - 180 degrees
//
// Copyright (C) 2005 Antonino Daplas <adaplas @pol.net>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

//
// Rotation 180 degrees
//
    static void ud_update_attr(u8 *dst, u8 *src, int attribute,
    struct vc_data *vc)
    {
    int i, offset = (vc.vc_font.height < 10) ? 1 : 2;
    let mut width: c_int = font_glyph_pitch(vc.vc_font.width);
    let mut cellsize: c_uint = vc.vc_font.height * width;
    u8 c;
    offset = offset * width;
    for (i = 0; i < cellsize; i++) {
    c = src[i];
    if (attribute & FBCON_ATTRIBUTE_UNDERLINE && i < offset)
    c = 0xff;
    if (attribute & FBCON_ATTRIBUTE_BOLD)
    c |= c << 1;
    if (attribute & FBCON_ATTRIBUTE_REVERSE)
    c = ~c;
    dst[i] = c;
    }
    }
    static void ud_bmove(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int dy, int dx, int height, int width)
    {
    struct fbcon_par *par = info.fbcon_par;
    struct fb_copyarea area;
    let mut vyres: u32 = GETVYRES(par.p, info);
    let mut vxres: u32 = GETVXRES(par.p, info);
    area.sy = vyres - ((sy + height) * vc.vc_font.height);
    area.sx = vxres - ((sx + width) * vc.vc_font.width);
    area.dy = vyres - ((dy + height) * vc.vc_font.height);
    area.dx = vxres - ((dx + width) * vc.vc_font.width);
    area.height = height * vc.vc_font.height;
    area.width  = width * vc.vc_font.width;
    info.fbops.fb_copyarea(info, &area);
    }
    static void ud_clear(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int height, int width, int fg, int bg)
    {
    struct fbcon_par *par = info.fbcon_par;
    struct fb_fillrect region;
    let mut vyres: u32 = GETVYRES(par.p, info);
    let mut vxres: u32 = GETVXRES(par.p, info);
    region.color = bg;
    region.dy = vyres - ((sy + height) * vc.vc_font.height);
    region.dx = vxres - ((sx + width) *  vc.vc_font.width);
    region.width = width * vc.vc_font.width;
    region.height = height * vc.vc_font.height;
    region.rop = ROP_COPY;
    info.fbops.fb_fillrect(info, &region);
    }
    static inline void ud_putcs_aligned(struct vc_data *vc, struct fb_info *info,
    const u16 *s, u32 attr, u32 cnt,
    u32 d_pitch, u32 s_pitch, u32 cellsize,
    struct fb_image *image, u8 *buf, u8 *dst)
    {
    struct fbcon_par *par = info.fbcon_par;
    let mut charmask: u16 = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut idx: u32 = vc.vc_font.width >> 3;
    u8 *src;
    while (cnt--) {
    src = par.rotated.buf + (scr_readw(s--) & charmask) * cellsize;
    if (attr) {
    ud_update_attr(buf, src, attr, vc);
    src = buf;
    }
    if (likely(idx == 1))
    __fb_pad_aligned_buffer(dst, d_pitch, src, idx,
    image.height);
    else
    fb_pad_aligned_buffer(dst, d_pitch, src, idx,
    image.height);
    dst += s_pitch;
    }
    info.fbops.fb_imageblit(info, image);
    }
    static inline void ud_putcs_unaligned(struct vc_data *vc,
    struct fb_info *info, const u16 *s,
    u32 attr, u32 cnt, u32 d_pitch,
    u32 s_pitch, u32 cellsize,
    struct fb_image *image, u8 *buf,
    u8 *dst)
    {
    struct fbcon_par *par = info.fbcon_par;
    let mut charmask: u16 = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut shift_low: u32 = 0, mod = vc.vc_font.width % 8;
    let mut shift_high: u32 = 8;
    let mut idx: u32 = vc.vc_font.width >> 3;
    u8 *src;
    while (cnt--) {
    src = par.rotated.buf + (scr_readw(s--) & charmask) * cellsize;
    if (attr) {
    ud_update_attr(buf, src, attr, vc);
    src = buf;
    }
    fb_pad_unaligned_buffer(dst, d_pitch, src, idx,
    image.height, shift_high,
    shift_low, mod);
    shift_low += mod;
    dst += (shift_low >= 8) ? s_pitch : s_pitch - 1;
    shift_low &= 7;
    shift_high = 8 - shift_low;
    }
    info.fbops.fb_imageblit(info, image);
    }
    static void ud_putcs(struct vc_data *vc, struct fb_info *info,
    const unsigned short *s, int count, int yy, int xx,
    int fg, int bg)
    {
    struct fb_image image;
    struct fbcon_par *par = info.fbcon_par;
    let mut width: u32 = font_glyph_pitch(vc.vc_font.width);
    let mut cellsize: u32 = width * vc.vc_font.height;
    let mut maxcnt: u32 = info.pixmap.size/cellsize;
    let mut scan_align: u32 = info.pixmap.scan_align - 1;
    let mut buf_align: u32 = info.pixmap.buf_align - 1;
    let mut mod: u32 = vc.vc_font.width % 8, cnt, pitch, size;
    let mut attribute: u32 = get_attribute(info, scr_readw(s));
    u8 *dst, *buf = core::ptr::null_mut();
    let mut vyres: u32 = GETVYRES(par.p, info);
    let mut vxres: u32 = GETVXRES(par.p, info);
    if (!par.rotated.buf)
    return;
    image.fg_color = fg;
    image.bg_color = bg;
    image.dy = vyres - ((yy * vc.vc_font.height) + vc.vc_font.height);
    image.dx = vxres - ((xx + count) * vc.vc_font.width);
    image.height = vc.vc_font.height;
    image.depth = 1;
    if (attribute) {
    buf = kmalloc(cellsize, GFP_KERNEL);
    if (!buf)
    return;
    }
    s += count - 1;
    while (count) {
    if (count > maxcnt)
    cnt = maxcnt;
    else
    cnt = count;
    image.width = vc.vc_font.width * cnt;
    pitch = ((image.width + 7) >> 3) + scan_align;
    pitch &= ~scan_align;
    size = pitch * image.height + buf_align;
    size &= ~buf_align;
    dst = fb_get_buffer_offset(info, &info.pixmap, size);
    image.data = dst;
    if (!mod)
    ud_putcs_aligned(vc, info, s, attribute, cnt, pitch,
    width, cellsize, &image, buf, dst);
    else
    ud_putcs_unaligned(vc, info, s, attribute, cnt, pitch,
    width, cellsize, &image,
    buf, dst);
    image.dx += image.width;
    count -= cnt;
    s -= cnt;
    xx += cnt;
    }
// buf is always NULL except when in monochrome mode, so in this case
    it's a gain to check buf against core::ptr::null_mut() even though kfree() handles
    core::ptr::null_mut() pointers just fine */
    if (unlikely(buf))
    kfree(buf);
    }
    static void ud_clear_margins(struct vc_data *vc, struct fb_info *info,
    int color, int bottom_only)
    {
    let mut cw: c_uint = vc.vc_font.width;
    let mut ch: c_uint = vc.vc_font.height;
    let mut rw: c_uint = info.var.xres - (vc.vc_cols*cw);
    let mut bh: c_uint = info.var.yres - (vc.vc_rows*ch);
    struct fb_fillrect region;
    region.color = color;
    region.rop = ROP_COPY;
    if ((int) rw > 0 && !bottom_only) {
    region.dy = 0;
    region.dx = info.var.xoffset;
    region.width  = rw;
    region.height = info.var.yres_virtual;
    info.fbops.fb_fillrect(info, &region);
    }
    if ((int) bh > 0) {
    region.dy = info.var.yoffset;
    region.dx = info.var.xoffset;
    region.height  = bh;
    region.width = info.var.xres;
    info.fbops.fb_fillrect(info, &region);
    }
    }
    static void ud_cursor(struct vc_data *vc, struct fb_info *info, bool enable,
    int fg, int bg)
    {
    struct fb_cursor cursor;
    struct fbcon_par *par = info.fbcon_par;
    let mut charmask: c_ushort = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut w: c_int = font_glyph_pitch(vc.vc_font.width);
    int c;
    let mut y: c_int = real_y(par.p, vc.state.y);
    int attribute, use_sw = vc.vc_cursor_type & CUR_SW;
    let mut err: c_int = 1, dx, dy;
    char *src;
    let mut vyres: u32 = GETVYRES(par.p, info);
    let mut vxres: u32 = GETVXRES(par.p, info);
    if (!par.rotated.buf)
    return;
    cursor.set = 0;
    c = scr_readw((u16 *) vc.vc_pos);
    attribute = get_attribute(info, c);
    src = par.rotated.buf + ((c & charmask) * (w * vc.vc_font.height));
    if (par.cursor_state.image.data != src ||
    par.cursor_reset) {
    par.cursor_state.image.data = src;
    cursor.set |= FB_CUR_SETIMAGE;
    }
    if (attribute) {
    u8 *dst;
    dst = kmalloc_array(w, vc.vc_font.height, GFP_ATOMIC);
    if (!dst)
    return;
    kfree(par.cursor_data);
    par.cursor_data = dst;
    ud_update_attr(dst, src, attribute, vc);
    src = dst;
    }
    if (par.cursor_state.image.fg_color != fg ||
    par.cursor_state.image.bg_color != bg ||
    par.cursor_reset) {
    par.cursor_state.image.fg_color = fg;
    par.cursor_state.image.bg_color = bg;
    cursor.set |= FB_CUR_SETCMAP;
    }
    if (par.cursor_state.image.height != vc.vc_font.height ||
    par.cursor_state.image.width != vc.vc_font.width ||
    par.cursor_reset) {
    par.cursor_state.image.height = vc.vc_font.height;
    par.cursor_state.image.width = vc.vc_font.width;
    cursor.set |= FB_CUR_SETSIZE;
    }
    dy = vyres - ((y * vc.vc_font.height) + vc.vc_font.height);
    dx = vxres - ((vc.state.x * vc.vc_font.width) + vc.vc_font.width);
    if (par.cursor_state.image.dx != dx ||
    par.cursor_state.image.dy != dy ||
    par.cursor_reset) {
    par.cursor_state.image.dx = dx;
    par.cursor_state.image.dy = dy;
    cursor.set |= FB_CUR_SETPOS;
    }
    if (par.cursor_state.hot.x || par.cursor_state.hot.y ||
    par.cursor_reset) {
    par.cursor_state.hot.x = cursor.hot.y = 0;
    cursor.set |= FB_CUR_SETHOT;
    }
    if (cursor.set & FB_CUR_SETSIZE ||
    vc.vc_cursor_type != par.p.cursor_shape ||
    par.cursor_state.mask == core::ptr::null_mut() ||
    par.cursor_reset) {
    unsigned char *tmp, *mask;
    tmp = kmalloc_array(vc.vc_font.height, w, GFP_ATOMIC);
    if (!tmp)
    return;
    fbcon_fill_cursor_mask(par, vc, tmp);
    mask = kmalloc_array(vc.vc_font.height, w, GFP_ATOMIC);
    if (!mask) {
    kfree(tmp);
    return;
    }
    font_glyph_rotate_180(tmp, vc.vc_font.width, vc.vc_font.height, mask);
    kfree(tmp);
    kfree(par.cursor_state.mask);
    par.cursor_state.mask = (const char *)mask;
    par.p.cursor_shape = vc.vc_cursor_type;
    cursor.set |= FB_CUR_SETSHAPE;
    }
    par.cursor_state.enable = enable && !use_sw;
    cursor.image.data = src;
    cursor.image.fg_color = par.cursor_state.image.fg_color;
    cursor.image.bg_color = par.cursor_state.image.bg_color;
    cursor.image.dx = par.cursor_state.image.dx;
    cursor.image.dy = par.cursor_state.image.dy;
    cursor.image.height = par.cursor_state.image.height;
    cursor.image.width = par.cursor_state.image.width;
    cursor.hot.x = par.cursor_state.hot.x;
    cursor.hot.y = par.cursor_state.hot.y;
    cursor.mask = par.cursor_state.mask;
    cursor.enable = par.cursor_state.enable;
    cursor.image.depth = 1;
    cursor.rop = ROP_XOR;
    if (info.fbops.fb_cursor)
    err = info.fbops.fb_cursor(info, &cursor);
    if (err)
    soft_cursor(info, &cursor);
    par.cursor_reset = 0;
    }
#[no_mangle]
unsafe extern "C" fn ud_update_start(info: *mut fb_info) -> c_int {
    static int ud_update_start(struct fb_info *info)
    {
    struct fbcon_par *par = info.fbcon_par;
    int xoffset, yoffset;
    let mut vyres: u32 = GETVYRES(par.p, info);
    let mut vxres: u32 = GETVXRES(par.p, info);
    int err;
    xoffset = vxres - info.var.xres - par.var.xoffset;
    yoffset = vyres - info.var.yres - par.var.yoffset;
    if (yoffset < 0)
    yoffset += vyres;
    par.var.xoffset = xoffset;
    par.var.yoffset = yoffset;
    err = fb_pan_display(info, &par.var);
    par.var.xoffset = info.var.xoffset;
    par.var.yoffset = info.var.yoffset;
    par.var.vmode = info.var.vmode;
    return err;
    }
    static const struct fbcon_bitops ud_fbcon_bitops = {
    .bmove = ud_bmove,
    .clear = ud_clear,
    .putcs = ud_putcs,
    .clear_margins = ud_clear_margins,
    .cursor = ud_cursor,
    .update_start = ud_update_start,
    .rotate_font = fbcon_rotate_font,
    };
#[no_mangle]
pub unsafe extern "C" fn fbcon_set_bitops_ud(par: *mut fbcon_par) {
    void fbcon_set_bitops_ud(struct fbcon_par *par)
    {
    par.bitops = &ud_fbcon_bitops;
    }
