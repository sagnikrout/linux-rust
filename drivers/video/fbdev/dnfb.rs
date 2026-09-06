//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/dnfb.c
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

// apollo video HW definitions
//
// Control Registers.   IOBASE + $x
//
// Note: these are the Memory/IO BASE definitions for a mono card set to the
// alternate address
//
// Control 3A and 3B serve identical functions except that 3A
// deals with control 1 and 3b deals with Color LUT reg.
//
pub const AP_IOBASE: c_uint = 0x3b0	/* Base address of 1 plane board. */;

pub const FRAME_BUFFER_START: c_uint = 0x0FA0000;
pub const FRAME_BUFFER_LEN: c_uint = 0x40000;
// CREG 0
pub const VECTOR_MODE: c_uint = 0x40	/* 010x.xxxx */;
pub const DBLT_MODE: c_uint = 0x80	/* 100x.xxxx */;
pub const NORMAL_MODE: c_uint = 0xE0	/* 111x.xxxx */;
pub const SHIFT_BITS: c_uint = 0x1F	/* xxx1.1111 */;
// other bits are Shift value
// CREG 1
pub const AD_BLT: c_uint = 0x80	/* 1xxx.xxxx */;
pub const NORMAL: c_uint = 0x80 /* 1xxx.xxxx */	/* What is happening here ?? */;
pub const INVERSE: c_uint = 0x00 /* 0xxx.xxxx */	/* Clearing this reverses the screen */;
pub const PIX_BLT: c_uint = 0x00	/* 0xxx.xxxx */;
pub const AD_HIBIT: c_uint = 0x40	/* xIxx.xxxx */;
pub const ROP_EN: c_uint = 0x10	/* xxx1.xxxx */;
pub const DST_EQ_SRC: c_uint = 0x00	/* xxx0.xxxx */;
pub const nRESET_SYNC: c_uint = 0x08	/* xxxx.1xxx */;
pub const SYNC_ENAB: c_uint = 0x02	/* xxxx.xx1x */;
pub const BLANK_DISP: c_uint = 0x00	/* xxxx.xxx0 */;
pub const ENAB_DISP: c_uint = 0x01	/* xxxx.xxx1 */;

// CREG 2
//
// Following 3 defines are common to 1, 4 and 8 plane.
//
pub const S_DATA_1s: c_uint = 0x00 /* 00xx.xxxx */	/* set source to all 1's -- vector drawing */;
pub const S_DATA_PIX: c_uint = 0x40 /* 01xx.xxxx */	/* takes source from ls-bits and replicates over 16 bits */;
pub const S_DATA_PLN: c_uint = 0xC0 /* 11xx.xxxx */	/* normal, each data access =16-bits in;
    one plane of image mem */
// CREG 3A/CREG 3B

// ROP REG  -  all one nibble
// ********* NOTE : this is used r0,r1,r2,r3 ***********

pub const DEST_ZERO: c_uint = 0x0;
pub const SRC_AND_DEST: c_uint = 0x1;
pub const SRC_AND_nDEST: c_uint = 0x2;
pub const SRC: c_uint = 0x3;
pub const nSRC_AND_DEST: c_uint = 0x4;
pub const DEST: c_uint = 0x5;
pub const SRC_XOR_DEST: c_uint = 0x6;
pub const SRC_OR_DEST: c_uint = 0x7;
pub const SRC_NOR_DEST: c_uint = 0x8;
pub const SRC_XNOR_DEST: c_uint = 0x9;
pub const nDEST: c_uint = 0xA;
pub const SRC_OR_nDEST: c_uint = 0xB;
pub const nSRC: c_uint = 0xC;
pub const nSRC_OR_DEST: c_uint = 0xD;
pub const SRC_NAND_DEST: c_uint = 0xE;
pub const DEST_ONE: c_uint = 0xF;

// frame buffer operations
    static int dnfb_blank(int blank, struct fb_info *info);
    static void dnfb_copyarea(struct fb_info *info, const struct fb_copyarea *area);
    static const struct fb_ops dn_fb_ops = {
    .owner		= THIS_MODULE,
    __FB_DEFAULT_IOMEM_OPS_RDWR,
    .fb_blank	= dnfb_blank,
    .fb_fillrect	= cfb_fillrect,
    .fb_copyarea	= dnfb_copyarea,
    .fb_imageblit	= cfb_imageblit,
    __FB_DEFAULT_IOMEM_OPS_MMAP,
    };
    static const struct fb_var_screeninfo dnfb_var = {
    .xres		= 1280,
    .yres		= 1024,
    .xres_virtual	= 2048,
    .yres_virtual	= 1024,
    .bits_per_pixel	= 1,
    .height		= -1,
    .width		= -1,
    .vmode		= FB_VMODE_NONINTERLACED,
    };
    static const struct fb_fix_screeninfo dnfb_fix = {
    .id		= "Apollo Mono",
    .smem_start	= (FRAME_BUFFER_START + IO_BASE),
    .smem_len	= FRAME_BUFFER_LEN,
    .type		= FB_TYPE_PACKED_PIXELS,
    .visual		= FB_VISUAL_MONO10,
    .line_length	= 256,
    };
#[no_mangle]
unsafe extern "C" fn dnfb_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int dnfb_blank(int blank, struct fb_info *info)
    {
    if (blank)
    out_8(AP_CONTROL_3A, 0x0);
    else
    out_8(AP_CONTROL_3A, 0x1);
    return 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn dnfb_copyarea(info: *mut fb_info, area: *const fb_copyarea) {
    void dnfb_copyarea(struct fb_info *info, const struct fb_copyarea *area)
    {
    int incr, y_delta, pre_read = 0, x_end, x_word_count;
    uint start_mask, end_mask, dest;
    ushort *src, dummy;
    short i, j;
    incr = (area.dy <= area.sy) ? 1 : -1;
    src = (ushort *)(info.screen_base + area.sy * info.fix.line_length +
    (area.sx >> 4));
    dest = area.dy * (info.fix.line_length >> 1) + (area.dx >> 4);
    if (incr > 0) {
    y_delta = (info.fix.line_length * 8) - area.sx - area.width;
    x_end = area.dx + area.width - 1;
    x_word_count = (x_end >> 4) - (area.dx >> 4) + 1;
    start_mask = 0xffff0000 >> (area.dx & 0xf);
    end_mask = 0x7ffff >> (x_end & 0xf);
    out_8(AP_CONTROL_0,
    (((area.dx & 0xf) - (area.sx & 0xf)) % 16) | (0x4 << 5));
    if ((area.dx & 0xf) < (area.sx & 0xf))
    pre_read = 1;
    } else {
    y_delta = -((info.fix.line_length * 8) - area.sx - area.width);
    x_end = area.dx - area.width + 1;
    x_word_count = (area.dx >> 4) - (x_end >> 4) + 1;
    start_mask = 0x7ffff >> (area.dx & 0xf);
    end_mask = 0xffff0000 >> (x_end & 0xf);
    out_8(AP_CONTROL_0,
    ((-((area.sx & 0xf) - (area.dx & 0xf))) % 16) |
    (0x4 << 5));
    if ((area.dx & 0xf) > (area.sx & 0xf))
    pre_read = 1;
    }
    for (i = 0; i < area.height; i++) {
    out_8(AP_CONTROL_3A, 0xc | (dest >> 16));
    if (pre_read) {
    dummy = *src;
    src += incr;
    }
    if (x_word_count) {
    out_8(AP_WRITE_ENABLE, start_mask);
// src = dest;
    src += incr;
    dest += incr;
    out_8(AP_WRITE_ENABLE, 0);
    for (j = 1; j < (x_word_count - 1); j++) {
// src = dest;
    src += incr;
    dest += incr;
    }
    out_8(AP_WRITE_ENABLE, start_mask);
// src = dest;
    dest += incr;
    src += incr;
    } else {
    out_8(AP_WRITE_ENABLE, start_mask | end_mask);
// src = dest;
    dest += incr;
    src += incr;
    }
    src += (y_delta / 16);
    dest += (y_delta / 16);
    }
    out_8(AP_CONTROL_0, NORMAL_MODE);
    }
//
// Initialization
//
#[no_mangle]
unsafe extern "C" fn dnfb_probe(dev: *mut platform_device) -> c_int {
    static int dnfb_probe(struct platform_device *dev)
    {
    struct fb_info *info;
    let mut err: c_int = 0;
    info = framebuffer_alloc(0, &dev.dev);
    if (!info)
    return -ENOMEM;
    info.fbops = &dn_fb_ops;
    info.fix = dnfb_fix;
    info.var = dnfb_var;
    info.var.red.length = 1;
    info.var.red.offset = 0;
    info.var.green = info.var.blue = info.var.red;
    info.screen_base = (u_char *) info.fix.smem_start;
    err = fb_alloc_cmap(&info.cmap, 2, 0);
    if (err < 0)
    goto release_framebuffer;
    err = register_framebuffer(info);
    if (err < 0) {
    fb_dealloc_cmap(&info.cmap);
    goto release_framebuffer;
    }
    platform_set_drvdata(dev, info);
// now we have registered we can safely setup the hardware
    out_8(AP_CONTROL_3A, RESET_CREG);
    out_be16(AP_WRITE_ENABLE, 0x0);
    out_8(AP_CONTROL_0, NORMAL_MODE);
    out_8(AP_CONTROL_1, (AD_BLT | DST_EQ_SRC | NORM_CREG1));
    out_8(AP_CONTROL_2, S_DATA_PLN);
    out_be16(AP_ROP_1, SWAP(0x3));
    printk("apollo frame buffer alive and kicking !\n");
    return err;
    release_framebuffer:
    framebuffer_release(info);
    return err;
    }
    static struct platform_driver dnfb_driver = {
    .probe	= dnfb_probe,
    .driver	= {
    .name	= "dnfb",
    },
    };
    static struct platform_device dnfb_device = {
    .name	= "dnfb",
    };
#[no_mangle]
unsafe extern "C" fn dnfb_init() -> int __init {
    static int __init dnfb_init(void)
    {
    int ret;
    if (!MACH_IS_APOLLO)
    return -ENODEV;
    if (fb_get_options("dnfb", core::ptr::null_mut()))
    return -ENODEV;
    ret = platform_driver_register(&dnfb_driver);
    if (!ret) {
    ret = platform_device_register(&dnfb_device);
    if (ret)
    platform_driver_unregister(&dnfb_driver);
    }
    return ret;
    }
    module_init(dnfb_init);
    MODULE_LICENSE("GPL");
