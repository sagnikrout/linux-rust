//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/g364fb.c
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


// $Id: g364fb.c,v 1.3 1998/08/28 22:43:00 tsbogend Exp $
//
// linux/drivers/video/g364fb.c -- Mips Magnum frame buffer device
//
// (C) 1998 Thomas Bogendoerfer
//
// This driver is based on tgafb.c
//
// Copyright (C) 1997 Geert Uytterhoeven
// Copyright (C) 1995  Jay Estabrook
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//

//
// Various defines for the G364
//
pub const G364_MEM_BASE: c_uint = 0xe4400000;
pub const G364_PORT_BASE: c_uint = 0xe4000000;
pub const ID_REG: c_uint = 0xe4000000	/* Read only */;
pub const BOOT_REG: c_uint = 0xe4080000;
pub const TIMING_REG: c_uint = 0xe4080108	/* to 0x080170 - DON'T TOUCH! */;
pub const DISPLAY_REG: c_uint = 0xe4080118;
pub const VDISPLAY_REG: c_uint = 0xe4080150;
pub const MASK_REG: c_uint = 0xe4080200;
pub const CTLA_REG: c_uint = 0xe4080300;
pub const CURS_TOGGLE: c_uint = 0x800000;
pub const BIT_PER_PIX: c_uint = 0x700000	/* bits 22 to 20 of Control A */;
pub const DELAY_SAMPLE: c_uint = 0x080000;
pub const PORT_INTER: c_uint = 0x040000;
pub const PIX_PIPE_DEL: c_uint = 0x030000	/* bits 17 and 16 of Control A */;
pub const PIX_PIPE_DEL2: c_uint = 0x008000	/* same as above - don't ask me why */;
pub const TR_CYCLE_TOG: c_uint = 0x004000;
pub const VRAM_ADR_INC: c_uint = 0x003000	/* bits 13 and 12 of Control A */;
pub const BLANK_OFF: c_uint = 0x000800;
pub const FORCE_BLANK: c_uint = 0x000400;
pub const BLK_FUN_SWTCH: c_uint = 0x000200;
pub const BLANK_IO: c_uint = 0x000100;
pub const BLANK_LEVEL: c_uint = 0x000080;
pub const A_VID_FORM: c_uint = 0x000040;
pub const D_SYNC_FORM: c_uint = 0x000020;
pub const FRAME_FLY_PAT: c_uint = 0x000010;
pub const OP_MODE: c_uint = 0x000008;
pub const INTL_STAND: c_uint = 0x000004;
pub const SCRN_FORM: c_uint = 0x000002;
pub const ENABLE_VTG: c_uint = 0x000001;
pub const TOP_REG: c_uint = 0xe4080400;
pub const CURS_PAL_REG: c_uint = 0xe4080508	/* to 0x080518 */;
pub const CHKSUM_REG: c_uint = 0xe4080600	/* to 0x080610 - unused */;
pub const CURS_POS_REG: c_uint = 0xe4080638;
pub const CLR_PAL_REG: c_uint = 0xe4080800	/* to 0x080ff8 */;
pub const CURS_PAT_REG: c_uint = 0xe4081000	/* to 0x081ff8 */;
pub const MON_ID_REG: c_uint = 0xe4100000	/* unused */;
pub const RESET_REG: c_uint = 0xe4180000	/* Write only */;
    static struct fb_info fb_info;
    static struct fb_fix_screeninfo fb_fix __initdata = {
    .id 		= "G364 8plane",
    .smem_start 	= 0x40000000,	/* physical address */
    .type 		= FB_TYPE_PACKED_PIXELS,
    .visual 	= FB_VISUAL_PSEUDOCOLOR,
    .ypanstep 	= 1,
    .accel 		= FB_ACCEL_NONE,
    };
    static struct fb_var_screeninfo fb_var __initdata = {
    .bits_per_pixel = 8,
    .red 		= { 0, 8, 0 },
    .green 		= { 0, 8, 0 },
    .blue		= { 0, 8, 0 },
    .activate	= FB_ACTIVATE_NOW,
    .height		= -1,
    .width		= -1,
    .pixclock	= 39722,
    .left_margin	= 40,
    .right_margin	= 24,
    .upper_margin	= 32,
    .lower_margin	= 11,
    .hsync_len 	= 96,
    .vsync_len 	= 2,
    .vmode		= FB_VMODE_NONINTERLACED,
    };
//
// Interface used by the world
//
    int g364fb_init(void);
    static int g364fb_pan_display(struct fb_var_screeninfo *var,
    struct fb_info *info);
    static int g364fb_setcolreg(u_int regno, u_int red, u_int green,
    u_int blue, u_int transp,
    struct fb_info *info);
    static int g364fb_blank(int blank, struct fb_info *info);
    static const struct fb_ops g364fb_ops = {
    .owner		= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_setcolreg	= g364fb_setcolreg,
    .fb_pan_display	= g364fb_pan_display,
    .fb_blank	= g364fb_blank,
    };
//
// Pan or Wrap the Display
//
// This call looks only at xoffset, yoffset and the FB_VMODE_YWRAP flag
//
    static int g364fb_pan_display(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    if (var.xoffset ||
    var.yoffset + info.var.yres > info.var.yres_virtual)
    return -EINVAL;
// (unsigned int *) TOP_REG = var->yoffset * info->var.xres;
    return 0;
    }
//
// Blank the display.
//
#[no_mangle]
unsafe extern "C" fn g364fb_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int g364fb_blank(int blank, struct fb_info *info)
    {
    if (blank)
// (unsigned int *) CTLA_REG |= FORCE_BLANK;
    else
// (unsigned int *) CTLA_REG &= ~FORCE_BLANK;
    return 0;
    }
//
// Set a single color register. Return != 0 for invalid regno.
//
    static int g364fb_setcolreg(u_int regno, u_int red, u_int green,
    u_int blue, u_int transp, struct fb_info *info)
    {
    volatile unsigned int *ptr = (volatile unsigned int *) CLR_PAL_REG;
    if (regno > 255)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    ptr[regno << 1] = (red << 16) | (green << 8) | blue;
    return 0;
    }
//
// Initialisation
//
#[no_mangle]
pub unsafe extern "C" fn g364fb_init() -> int __init {
    int __init g364fb_init(void)
    {
    volatile unsigned int *curs_pal_ptr =
    (volatile unsigned int *) CURS_PAL_REG;
    int mem, i;
    if (fb_get_options("g364fb", core::ptr::null_mut()))
    return -ENODEV;
// TBD: G364 detection
// get the resolution set by ARC console
// (volatile unsigned int *) CTLA_REG &= ~ENABLE_VTG;
    fb_var.xres =
    (*((volatile unsigned int *) DISPLAY_REG) & 0x00ffffff) * 4;
    fb_var.yres =
    (*((volatile unsigned int *) VDISPLAY_REG) & 0x00ffffff) / 2;
// (volatile unsigned int *) CTLA_REG |= ENABLE_VTG;
// setup cursor
    curs_pal_ptr[0] |= 0x00ffffff;
    curs_pal_ptr[2] |= 0x00ffffff;
    curs_pal_ptr[4] |= 0x00ffffff;
//
// first set the whole cursor to transparent
//
    for (i = 0; i < 512; i++)
// (unsigned short *) (CURS_PAT_REG + i * 8) = 0;
//
// switch the last two lines to cursor palette 3
// we assume here, that FONTSIZE_X is 8
//
// (unsigned short *) (CURS_PAT_REG + 14 * 64) = 0xffff;
// (unsigned short *) (CURS_PAT_REG + 15 * 64) = 0xffff;
    fb_var.xres_virtual = fb_var.xres;
    fb_fix.line_length = fb_var.xres_virtual * fb_var.bits_per_pixel / 8;
    fb_fix.smem_start = 0x40000000;	/* physical address */
// get size of video memory; this is special for the JAZZ hardware
    mem = (r4030_read_reg32(JAZZ_R4030_CONFIG) >> 8) & 3;
    fb_fix.smem_len = (1 << (mem * 2)) * 512 * 1024;
    fb_var.yres_virtual = fb_fix.smem_len / fb_var.xres;
    fb_info.fbops = &g364fb_ops;
    fb_info.screen_base = (char *) G364_MEM_BASE;	/* virtual kernel address */
    fb_info.var = fb_var;
    fb_info.fix = fb_fix;
    fb_info.flags = FBINFO_HWACCEL_YPAN;
    fb_alloc_cmap(&fb_info.cmap, 255, 0);
    if (register_framebuffer(&fb_info) < 0)
    return -EINVAL;
    return 0;
    }
    module_init(g364fb_init);
    MODULE_LICENSE("GPL");
