//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/bitblit.c
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
// linux/drivers/video/console/bitblit.c -- BitBlitting Operation
//
// Originally from the 'accel_*' routines in drivers/video/console/fbcon.c
//
// Copyright (C) 2004 Antonino Daplas <adaplas @pol.net>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

//
// Accelerated handlers.
//
#[no_mangle]
unsafe extern "C" fn update_attr(dst: *mut u8, src: *const u8, attribute: c_int, vc: *mut vc_data) {
    static void update_attr(u8 *dst, const u8 *src, int attribute, struct vc_data *vc)
    {
    int i, offset = (vc.vc_font.height < 10) ? 1 : 2;
    let mut width: c_int = DIV_ROUND_UP(vc.vc_font.width, 8);
    let mut cellsize: c_uint = vc.vc_font.height * width;
    u8 c;
    offset = cellsize - (offset * width);
    for (i = 0; i < cellsize; i++) {
    c = src[i];
    if (attribute & FBCON_ATTRIBUTE_UNDERLINE && i >= offset)
    c = 0xff;
    if (attribute & FBCON_ATTRIBUTE_BOLD)
    c |= c >> 1;
    if (attribute & FBCON_ATTRIBUTE_REVERSE)
    c = ~c;
    dst[i] = c;
    }
    }
    static void bit_bmove(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int dy, int dx, int height, int width)
    {
    struct fb_copyarea area;
    area.sx = sx * vc.vc_font.width;
    area.sy = sy * vc.vc_font.height;
    area.dx = dx * vc.vc_font.width;
    area.dy = dy * vc.vc_font.height;
    area.height = height * vc.vc_font.height;
    area.width = width * vc.vc_font.width;
    info.fbops.fb_copyarea(info, &area);
    }
    static void bit_clear(struct vc_data *vc, struct fb_info *info, int sy,
    int sx, int height, int width, int fg, int bg)
    {
    struct fb_fillrect region;
    region.color = bg;
    region.dx = sx * vc.vc_font.width;
    region.dy = sy * vc.vc_font.height;
    region.width = width * vc.vc_font.width;
    region.height = height * vc.vc_font.height;
    region.rop = ROP_COPY;
    info.fbops.fb_fillrect(info, &region);
    }
    static inline void bit_putcs_aligned(struct vc_data *vc, struct fb_info *info,
    const u16 *s, u32 attr, u32 cnt,
    u32 d_pitch, u32 s_pitch, u32 cellsize,
    struct fb_image *image, u8 *buf, u8 *dst)
    {
    let mut charmask: u16 = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut charcnt: c_uint = vc.vc_font.charcount;
    let mut idx: u32 = vc.vc_font.width >> 3;
    const u8 *src;
    while (cnt--) {
    let mut ch: u16 = scr_readw(s++) & charmask;
    if (ch >= charcnt)
    ch = 0;
    src = vc.vc_font.data + (unsigned int)ch * cellsize;
    if (attr) {
    update_attr(buf, src, attr, vc);
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
    static inline void bit_putcs_unaligned(struct vc_data *vc,
    struct fb_info *info, const u16 *s,
    u32 attr, u32 cnt, u32 d_pitch,
    u32 s_pitch, u32 cellsize,
    struct fb_image *image, u8 *buf,
    u8 *dst)
    {
    let mut charmask: u16 = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut charcnt: c_uint = vc.vc_font.charcount;
    let mut shift_low: u32 = 0, mod = vc.vc_font.width % 8;
    let mut shift_high: u32 = 8;
    let mut idx: u32 = vc.vc_font.width >> 3;
    const u8 *src;
    while (cnt--) {
    let mut ch: u16 = scr_readw(s++) & charmask;
    if (ch >= charcnt)
    ch = 0;
    src = vc.vc_font.data + (unsigned int)ch * cellsize;
    if (attr) {
    update_attr(buf, src, attr, vc);
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
    static void bit_putcs(struct vc_data *vc, struct fb_info *info,
    const unsigned short *s, int count, int yy, int xx,
    int fg, int bg)
    {
    struct fb_image image;
    let mut width: u32 = DIV_ROUND_UP(vc.vc_font.width, 8);
    let mut cellsize: u32 = width * vc.vc_font.height;
    let mut maxcnt: u32 = info.pixmap.size/cellsize;
    let mut scan_align: u32 = info.pixmap.scan_align - 1;
    let mut buf_align: u32 = info.pixmap.buf_align - 1;
    let mut mod: u32 = vc.vc_font.width % 8, cnt, pitch, size;
    let mut attribute: u32 = get_attribute(info, scr_readw(s));
    u8 *dst, *buf = core::ptr::null_mut();
    image.fg_color = fg;
    image.bg_color = bg;
    image.dx = xx * vc.vc_font.width;
    image.dy = yy * vc.vc_font.height;
    image.height = vc.vc_font.height;
    image.depth = 1;
    if (image.dy >= info.var.yres)
    return;
    image.height = min(image.height, info.var.yres - image.dy);
    if (attribute) {
    buf = kmalloc(cellsize, GFP_ATOMIC);
    if (!buf)
    return;
    }
    while (count) {
    if (count > maxcnt)
    cnt = maxcnt;
    else
    cnt = count;
    image.width = vc.vc_font.width * cnt;
    if (image.dx >= info.var.xres)
    break;
    if (image.dx + image.width > info.var.xres) {
    image.width = info.var.xres - image.dx;
    cnt = image.width / vc.vc_font.width;
    if (cnt == 0)
    break;
    image.width = cnt * vc.vc_font.width;
    }
    pitch = DIV_ROUND_UP(image.width, 8) + scan_align;
    pitch &= ~scan_align;
    size = pitch * image.height + buf_align;
    size &= ~buf_align;
    dst = fb_get_buffer_offset(info, &info.pixmap, size);
    image.data = dst;
    if (!mod)
    bit_putcs_aligned(vc, info, s, attribute, cnt, pitch,
    width, cellsize, &image, buf, dst);
    else
    bit_putcs_unaligned(vc, info, s, attribute, cnt,
    pitch, width, cellsize, &image,
    buf, dst);
    image.dx += cnt * vc.vc_font.width;
    count -= cnt;
    s += cnt;
    }
// buf is always NULL except when in monochrome mode, so in this case
    it's a gain to check buf against core::ptr::null_mut() even though kfree() handles
    core::ptr::null_mut() pointers just fine */
    if (unlikely(buf))
    kfree(buf);
    }
    static void bit_clear_margins(struct vc_data *vc, struct fb_info *info,
    int color, int bottom_only)
    {
    let mut cw: c_uint = vc.vc_font.width;
    let mut ch: c_uint = vc.vc_font.height;
    let mut rw: c_uint = info.var.xres - (vc.vc_cols*cw);
    let mut bh: c_uint = info.var.yres - (vc.vc_rows*ch);
    let mut rs: c_uint = info.var.xres - rw;
    let mut bs: c_uint = info.var.yres - bh;
    struct fb_fillrect region;
    region.color = color;
    region.rop = ROP_COPY;
    if ((int) rw > 0 && !bottom_only) {
    region.dx = info.var.xoffset + rs;
    region.dy = 0;
    region.width = rw;
    region.height = info.var.yres_virtual;
    info.fbops.fb_fillrect(info, &region);
    }
    if ((int) bh > 0) {
    region.dx = info.var.xoffset;
    region.dy = info.var.yoffset + bs;
    region.width = rs;
    region.height = bh;
    info.fbops.fb_fillrect(info, &region);
    }
    }
    static void bit_cursor(struct vc_data *vc, struct fb_info *info, bool enable,
    int fg, int bg)
    {
    struct fb_cursor cursor;
    struct fbcon_par *par = info.fbcon_par;
    let mut charmask: c_ushort = vc.vc_hi_font_mask ? 0x1ff : 0xff;
    let mut w: c_int = DIV_ROUND_UP(vc.vc_font.width, 8), c;
    let mut y: c_int = real_y(par.p, vc.state.y);
    int attribute, use_sw = vc.vc_cursor_type & CUR_SW;
    let mut err: c_int = 1;
    const u8 *src;
    cursor.set = 0;
    if (!vc.vc_font.data)
    return;
    c = scr_readw((u16 *) vc.vc_pos);
    attribute = get_attribute(info, c);
    c &= charmask;
// Clamp to font size, same as bit_putcs_aligned()
    if (c >= vc.vc_font.charcount)
    c = 0;
    src = vc.vc_font.data + (c * (w * vc.vc_font.height));
    if (par.cursor_state.image.data != (const char *)src ||
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
    update_attr(dst, src, attribute, vc);
    src = dst;
    }
    if (par.cursor_state.image.fg_color != fg ||
    par.cursor_state.image.bg_color != bg ||
    par.cursor_reset) {
    par.cursor_state.image.fg_color = fg;
    par.cursor_state.image.bg_color = bg;
    cursor.set |= FB_CUR_SETCMAP;
    }
    if ((par.cursor_state.image.dx != (vc.vc_font.width * vc.state.x)) ||
    (par.cursor_state.image.dy != (vc.vc_font.height * y)) ||
    par.cursor_reset) {
    par.cursor_state.image.dx = vc.vc_font.width * vc.state.x;
    par.cursor_state.image.dy = vc.vc_font.height * y;
    cursor.set |= FB_CUR_SETPOS;
    }
    if (par.cursor_state.image.height != vc.vc_font.height ||
    par.cursor_state.image.width != vc.vc_font.width ||
    par.cursor_reset) {
    par.cursor_state.image.height = vc.vc_font.height;
    par.cursor_state.image.width = vc.vc_font.width;
    cursor.set |= FB_CUR_SETSIZE;
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
    unsigned char *mask = kmalloc_array(vc.vc_font.height, w, GFP_ATOMIC);
    if (!mask)
    return;
    fbcon_fill_cursor_mask(par, vc, mask);
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
unsafe extern "C" fn bit_update_start(info: *mut fb_info) -> c_int {
    static int bit_update_start(struct fb_info *info)
    {
    struct fbcon_par *par = info.fbcon_par;
    int err;
    err = fb_pan_display(info, &par.var);
    par.var.xoffset = info.var.xoffset;
    par.var.yoffset = info.var.yoffset;
    par.var.vmode = info.var.vmode;
    return err;
    }
    static const struct fbcon_bitops bit_fbcon_bitops = {
    .bmove = bit_bmove,
    .clear = bit_clear,
    .putcs = bit_putcs,
    .clear_margins = bit_clear_margins,
    .cursor = bit_cursor,
    .update_start = bit_update_start,
    };
#[no_mangle]
pub unsafe extern "C" fn fbcon_set_bitops_ur(par: *mut fbcon_par) {
    void fbcon_set_bitops_ur(struct fbcon_par *par)
    {
    par.bitops = &bit_fbcon_bitops;
    }
