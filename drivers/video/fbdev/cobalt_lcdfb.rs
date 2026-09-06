//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/cobalt_lcdfb.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Cobalt/SEAD3 LCD frame buffer driver.
//
// Copyright (C) 2008  Yoichi Yuasa <yuasa@linux-mips.org>
// Copyright (C) 2012  MIPS Technologies, Inc.
//

//
// Cursor position address
// \X  0    1    2  ...  14   15
// Y+----+----+----+---+----+----+
// 0|0x00|0x01|0x02|...|0x0e|0x0f|
// +----+----+----+---+----+----+
// 1|0x40|0x41|0x42|...|0x4e|0x4f|
// +----+----+----+---+----+----+
//
pub const LCD_DATA_REG_OFFSET: c_uint = 0x10;
pub const LCD_XRES_MAX: c_int = 16;
pub const LCD_YRES_MAX: c_int = 2;
pub const LCD_CHARS_MAX: c_int = 32;
pub const LCD_CLEAR: c_uint = 0x01;
pub const LCD_CURSOR_MOVE_HOME: c_uint = 0x02;
pub const LCD_RESET: c_uint = 0x06;
pub const LCD_OFF: c_uint = 0x08;
pub const LCD_CURSOR_OFF: c_uint = 0x0c;
pub const LCD_CURSOR_BLINK_OFF: c_uint = 0x0e;
pub const LCD_CURSOR_ON: c_uint = 0x0f;

pub const LCD_CURSOR_MOVE_LEFT: c_uint = 0x10;
pub const LCD_CURSOR_MOVE_RIGHT: c_uint = 0x14;
pub const LCD_DISPLAY_LEFT: c_uint = 0x18;
pub const LCD_DISPLAY_RIGHT: c_uint = 0x1c;
pub const LCD_PRERESET: c_uint = 0x3f	/* execute 4 times continuously */;
pub const LCD_BUSY: c_uint = 0x80;
pub const LCD_GRAPHIC_MODE: c_uint = 0x40;
pub const LCD_TEXT_MODE: c_uint = 0x80;
pub const LCD_CUR_POS_MASK: c_uint = 0x7f;

#[no_mangle]
pub unsafe extern "C" fn lcd_write_control(info: *mut fb_info, control: u8) {
    static inline void lcd_write_control(struct fb_info *info, u8 control)
    {
    writel((u32)control << 24, info.screen_base);
    }
#[no_mangle]
pub unsafe extern "C" fn lcd_read_control(info: *mut fb_info) -> u8 {
    static inline u8 lcd_read_control(struct fb_info *info)
    {
    return readl(info.screen_base) >> 24;
    }
#[no_mangle]
pub unsafe extern "C" fn lcd_write_data(info: *mut fb_info, data: u8) {
    static inline void lcd_write_data(struct fb_info *info, u8 data)
    {
    writel((u32)data << 24, info.screen_base + LCD_DATA_REG_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn lcd_read_data(info: *mut fb_info) -> u8 {
    static inline u8 lcd_read_data(struct fb_info *info)
    {
    return readl(info.screen_base + LCD_DATA_REG_OFFSET) >> 24;
    }
#[no_mangle]
unsafe extern "C" fn lcd_busy_wait(info: *mut fb_info) -> c_int {
    static int lcd_busy_wait(struct fb_info *info)
    {
    let mut val: u8 = 0;
    let mut timeout: c_int = 10, retval = 0;
    do {
    val = lcd_read_control(info);
    val &= LCD_BUSY;
    if (val != LCD_BUSY)
    break;
    if (msleep_interruptible(1))
    return -EINTR;
    timeout--;
    } while (timeout);
    if (val == LCD_BUSY)
    retval = -EBUSY;
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lcd_clear(info: *mut fb_info) {
    static void lcd_clear(struct fb_info *info)
    {
    int i;
    for (i = 0; i < 4; i++) {
    udelay(150);
    lcd_write_control(info, LCD_PRERESET);
    }
    udelay(150);
    lcd_write_control(info, LCD_CLEAR);
    udelay(150);
    lcd_write_control(info, LCD_RESET);
    }
    static const struct fb_fix_screeninfo cobalt_lcdfb_fix = {
    .id		= "cobalt-lcd",
    .type		= FB_TYPE_TEXT,
    .type_aux	= FB_AUX_TEXT_MDA,
    .visual		= FB_VISUAL_MONO01,
    .line_length	= LCD_XRES_MAX,
    .accel		= FB_ACCEL_NONE,
    };
    static ssize_t cobalt_lcdfb_read(struct fb_info *info, char __user *buf,
    size_t count, loff_t *ppos)
    {
    char src[LCD_CHARS_MAX];
    unsigned long pos;
    int len, retval = 0;
    if (!info.screen_base)
    return -ENODEV;
    pos = *ppos;
    if (pos >= LCD_CHARS_MAX || count == 0)
    return 0;
    if (count > LCD_CHARS_MAX)
    count = LCD_CHARS_MAX;
    if (pos + count > LCD_CHARS_MAX)
    count = LCD_CHARS_MAX - pos;
    for (len = 0; len < count; len++) {
    retval = lcd_busy_wait(info);
    if (retval < 0)
    break;
    lcd_write_control(info, LCD_TEXT_POS(pos));
    retval = lcd_busy_wait(info);
    if (retval < 0)
    break;
    src[len] = lcd_read_data(info);
    if (pos == 0x0f)
    pos = 0x40;
    else
    pos++;
    }
    if (retval < 0 && signal_pending(current))
    return -ERESTARTSYS;
    if (copy_to_user(buf, src, len))
    return -EFAULT;
// ppos += len;
    return len;
    }
    static ssize_t cobalt_lcdfb_write(struct fb_info *info, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    char dst[LCD_CHARS_MAX];
    unsigned long pos;
    int len, retval = 0;
    if (!info.screen_base)
    return -ENODEV;
    pos = *ppos;
    if (pos >= LCD_CHARS_MAX || count == 0)
    return 0;
    if (count > LCD_CHARS_MAX)
    count = LCD_CHARS_MAX;
    if (pos + count > LCD_CHARS_MAX)
    count = LCD_CHARS_MAX - pos;
    if (copy_from_user(dst, buf, count))
    return -EFAULT;
    for (len = 0; len < count; len++) {
    retval = lcd_busy_wait(info);
    if (retval < 0)
    break;
    lcd_write_control(info, LCD_TEXT_POS(pos));
    retval = lcd_busy_wait(info);
    if (retval < 0)
    break;
    lcd_write_data(info, dst[len]);
    if (pos == 0x0f)
    pos = 0x40;
    else
    pos++;
    }
    if (retval < 0 && signal_pending(current))
    return -ERESTARTSYS;
// ppos += len;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn cobalt_lcdfb_blank(blank_mode: c_int, info: *mut fb_info) -> c_int {
    static int cobalt_lcdfb_blank(int blank_mode, struct fb_info *info)
    {
    int retval;
    retval = lcd_busy_wait(info);
    if (retval < 0)
    return retval;
    switch (blank_mode) {
    case FB_BLANK_UNBLANK:
    lcd_write_control(info, LCD_ON);
    break;
    default:
    lcd_write_control(info, LCD_OFF);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cobalt_lcdfb_cursor(info: *mut fb_info, cursor: *mut fb_cursor) -> c_int {
    static int cobalt_lcdfb_cursor(struct fb_info *info, struct fb_cursor *cursor)
    {
    u32 x, y;
    int retval;
    switch (cursor.set) {
    case FB_CUR_SETPOS:
    x = cursor.image.dx;
    y = cursor.image.dy;
    if (x >= LCD_XRES_MAX || y >= LCD_YRES_MAX)
    return -EINVAL;
    retval = lcd_busy_wait(info);
    if (retval < 0)
    return retval;
    lcd_write_control(info,
    LCD_TEXT_POS(info.fix.line_length * y + x));
    break;
    default:
    return -EINVAL;
    }
    retval = lcd_busy_wait(info);
    if (retval < 0)
    return retval;
    if (cursor.enable)
    lcd_write_control(info, LCD_CURSOR_ON);
    else
    lcd_write_control(info, LCD_CURSOR_OFF);
    return 0;
    }
    static const struct fb_ops cobalt_lcd_fbops = {
    .owner		= THIS_MODULE,
    .fb_read	= cobalt_lcdfb_read,
    .fb_write	= cobalt_lcdfb_write,
    .fb_blank	= cobalt_lcdfb_blank,
    __FB_DEFAULT_IOMEM_OPS_DRAW,
    .fb_cursor	= cobalt_lcdfb_cursor,
    __FB_DEFAULT_IOMEM_OPS_MMAP,
    };
#[no_mangle]
unsafe extern "C" fn cobalt_lcdfb_probe(dev: *mut platform_device) -> c_int {
    static int cobalt_lcdfb_probe(struct platform_device *dev)
    {
    struct fb_info *info;
    struct resource *res;
    int retval;
    info = framebuffer_alloc(0, &dev.dev);
    if (!info)
    return -ENOMEM;
    info.screen_base = devm_platform_get_and_ioremap_resource(dev, 0, &res);
    if (IS_ERR(info.screen_base)) {
    framebuffer_release(info);
    return PTR_ERR(info.screen_base);
    }
    info.screen_size = resource_size(res);
    info.fbops = &cobalt_lcd_fbops;
    info.fix = cobalt_lcdfb_fix;
    info.fix.smem_start = res.start;
    info.fix.smem_len = info.screen_size;
    info.pseudo_palette = core::ptr::null_mut();
    info.par = core::ptr::null_mut();
    retval = register_framebuffer(info);
    if (retval < 0) {
    framebuffer_release(info);
    return retval;
    }
    platform_set_drvdata(dev, info);
    lcd_clear(info);
    fb_info(info, "Cobalt server LCD frame buffer device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cobalt_lcdfb_remove(dev: *mut platform_device) {
    static void cobalt_lcdfb_remove(struct platform_device *dev)
    {
    struct fb_info *info;
    info = platform_get_drvdata(dev);
    if (info) {
    unregister_framebuffer(info);
    framebuffer_release(info);
    }
    }
    static struct platform_driver cobalt_lcdfb_driver = {
    .probe	= cobalt_lcdfb_probe,
    .remove	= cobalt_lcdfb_remove,
    .driver	= {
    .name	= "cobalt-lcd",
    },
    };
    module_platform_driver(cobalt_lcdfb_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Yoichi Yuasa");
    MODULE_DESCRIPTION("Cobalt server LCD frame buffer driver");
