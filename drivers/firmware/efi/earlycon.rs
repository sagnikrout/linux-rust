//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/earlycon.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2013 Intel Corporation; author Matt Fleming
//

    static const struct console *earlycon_console __initdata;
    static const struct font_desc *font;
    static u16 cur_line_y, max_line_y;
    static u32 efi_x_array[1024];
    static u32 efi_x, efi_y;
    static u64 fb_base;
    static bool fb_wb;
    static void *efi_fb;
//
// EFI earlycon needs to use early_memremap() to map the framebuffer.
// But early_memremap() is not usable for 'earlycon=efifb keep_bootcon',
// memremap() should be used instead. memremap() will be available after
// paging_init() which is earlier than initcall callbacks. Thus adding this
// early initcall function early_efi_map_fb() to map the whole EFI framebuffer.
//
#[no_mangle]
unsafe extern "C" fn efi_earlycon_remap_fb() -> int __init {
    static int __init efi_earlycon_remap_fb(void)
    {
    const struct screen_info *si = &sysfb_primary_display.screen;
// bail if there is no bootconsole or it was unregistered already
    if (!earlycon_console || !console_is_registered(earlycon_console))
    return 0;
    efi_fb = memremap(fb_base, si.lfb_size, fb_wb ? MEMREMAP_WB : MEMREMAP_WC);
    return efi_fb ? 0 : -ENOMEM;
    }
    early_initcall(efi_earlycon_remap_fb);
#[no_mangle]
unsafe extern "C" fn efi_earlycon_unmap_fb() -> int __init {
    static int __init efi_earlycon_unmap_fb(void)
    {
// unmap the bootconsole fb unless keep_bootcon left it registered
    if (efi_fb && !console_is_registered(earlycon_console))
    memunmap(efi_fb);
    return 0;
    }
    late_initcall(efi_earlycon_unmap_fb);
    static __ref void *efi_earlycon_map(unsigned long start, unsigned long len)
    {
    pgprot_t fb_prot;
    if (efi_fb)
    return efi_fb + start;
    fb_prot = fb_wb ? PAGE_KERNEL : pgprot_writecombine(PAGE_KERNEL);
    return early_memremap_prot(fb_base + start, len, pgprot_val(fb_prot));
    }
#[no_mangle]
unsafe extern "C" fn efi_earlycon_unmap(addr: *mut c_void, len: c_ulong) -> __ref void {
    static __ref void efi_earlycon_unmap(void *addr, unsigned long len)
    {
    if (efi_fb)
    return;
    early_memunmap(addr, len);
    }
#[no_mangle]
unsafe extern "C" fn efi_earlycon_clear_scanline(y: c_uint, si: *const screen_info) {
    static void efi_earlycon_clear_scanline(unsigned int y, const struct screen_info *si)
    {
    unsigned long *dst;
    u16 len;
    len = si.lfb_linelength;
    dst = efi_earlycon_map(y*len, len);
    if (!dst)
    return;
    memset(dst, 0, len);
    efi_earlycon_unmap(dst, len);
    }
#[no_mangle]
unsafe extern "C" fn efi_earlycon_scroll_up(si: *const screen_info) {
    static void efi_earlycon_scroll_up(const struct screen_info *si)
    {
    unsigned long *dst, *src;
    let mut maxlen: u16 = 0;
    u16 len;
    u32 i, height;
// Find the cached maximum x coordinate
    for (i = 0; i < max_line_y; i++) {
    if (efi_x_array[i] > maxlen)
    maxlen = efi_x_array[i];
    }
    maxlen *= 4;
    len = si.lfb_linelength;
    height = si.lfb_height;
    for (i = 0; i < height - font.height; i++) {
    dst = efi_earlycon_map(i*len, len);
    if (!dst)
    return;
    src = efi_earlycon_map((i + font.height) * len, len);
    if (!src) {
    efi_earlycon_unmap(dst, len);
    return;
    }
    memmove(dst, src, maxlen);
    efi_earlycon_unmap(src, len);
    efi_earlycon_unmap(dst, len);
    }
    }
    static void efi_earlycon_write_char(u32 *dst, unsigned char c, unsigned int h,
    const struct screen_info *si)
    {
    let mut color_black: u32 = 0x00000000;
    let mut color_white: u32 = 0x00ffffff;
    const u8 *src;
    int m, n, bytes;
    u8 x;
    bytes = BITS_TO_BYTES(font.width);
    src = font.data + c * font.height * bytes + h * bytes;
    for (m = 0; m < font.width; m++) {
    n = m % 8;
    x = *(src + m / 8);
    if ((x >> (7 - n)) & 1)
// dst = color_white;
    else
// dst = color_black;
    dst++;
    }
    }
    static void
    efi_earlycon_write(struct console *con, const char *str, unsigned int num)
    {
    const struct screen_info *si = &sysfb_primary_display.screen;
    let mut cur_efi_x: u32 = efi_x;
    unsigned int len;
    const char *s;
    void *dst;
    len = si.lfb_linelength;
    while (num) {
    let mut linemax: c_uint = (si.lfb_width - efi_x) / font.width;
    unsigned int h, count;
    count = strnchrnul(str, num, '\n') - str;
    if (count > linemax)
    count = linemax;
    for (h = 0; h < font.height; h++) {
    unsigned int n, x;
    dst = efi_earlycon_map((efi_y + h) * len, len);
    if (!dst)
    return;
    s = str;
    n = count;
    x = efi_x;
    while (n-- > 0) {
    efi_earlycon_write_char(dst + x * 4, *s, h, si);
    x += font.width;
    s++;
    }
    efi_earlycon_unmap(dst, len);
    }
    num -= count;
    efi_x += count * font.width;
    str += count;
    if (num > 0 && *s == '\n') {
    cur_efi_x = efi_x;
    efi_x = 0;
    efi_y += font.height;
    str++;
    num--;
    }
    if (efi_x + font.width > si.lfb_width) {
    cur_efi_x = efi_x;
    efi_x = 0;
    efi_y += font.height;
    }
    if (efi_y + font.height > si.lfb_height) {
    u32 i;
    efi_x_array[cur_line_y] = cur_efi_x;
    cur_line_y = (cur_line_y + 1) % max_line_y;
    efi_y -= font.height;
    efi_earlycon_scroll_up(si);
    for (i = 0; i < font.height; i++)
    efi_earlycon_clear_scanline(efi_y + i, si);
    }
    }
    }
    static bool __initdata fb_probed;
#[no_mangle]
pub unsafe extern "C" fn efi_earlycon_reprobe() -> void __init {
    void __init efi_earlycon_reprobe(void)
    {
    if (fb_probed)
    setup_earlycon("efifb");
    }
    static int __init efi_earlycon_setup(struct earlycon_device *device,
    const char *opt)
    {
    const struct screen_info *si = &sysfb_primary_display.screen;
    u16 xres, yres;
    u32 i;
    fb_wb = opt && !strcmp(opt, "ram");
    if (si.orig_video_isVGA != VIDEO_TYPE_EFI) {
    fb_probed = true;
    return -ENODEV;
    }
    fb_base = si.lfb_base;
    if (si.capabilities & VIDEO_CAPABILITY_64BIT_BASE)
    fb_base |= (u64)si.ext_lfb_base << 32;
    xres = si.lfb_width;
    yres = si.lfb_height;
//
// efi_earlycon_write_char() implicitly assumes a framebuffer with
// 32 bits per pixel.
//
    if (si.lfb_depth != 32)
    return -ENODEV;
    font = get_default_font(xres, yres, core::ptr::null_mut(), core::ptr::null_mut());
    if (!font)
    return -ENODEV;
// Fill the cache with maximum possible value of x coordinate
    memset32(efi_x_array, rounddown(xres, font.width), ARRAY_SIZE(efi_x_array));
    efi_y = rounddown(yres, font.height);
// Make sure we have cache for the x coordinate for the full screen
    max_line_y = efi_y / font.height + 1;
    cur_line_y = 0;
    efi_y -= font.height;
    for (i = 0; i < (yres - efi_y) / font.height; i++)
    efi_earlycon_scroll_up(si);
    device.con.write = efi_earlycon_write;
    earlycon_console = device.con;
    return 0;
    }
    EARLYCON_DECLARE(efifb, efi_earlycon_setup);
