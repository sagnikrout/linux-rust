//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/maxinefb.c
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
// linux/drivers/video/maxinefb.c
//
// DECstation 5000/xx onboard framebuffer support ... derived from:
// "HP300 Topcat framebuffer support (derived from macfb of all things)
// Phil Blundell <philb@gnu.org> 1998", the original code can be
// found in the file hpfb.c in the same directory.
//
// DECstation related code Copyright (C) 1999,2000,2001 by
// Michael Engel <engel@unix-ag.org> and
// Karsten Merker <merker@linuxtag.org>.
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//
// Changes:
// 2001/01/27 removed debugging and testing code, fixed fb_ops
// initialization which had caused a crash before,
// general cleanup, first official release (KM)
//

// bootinfo.h defines the machine type values, needed when checking
// whether are really running on a maxine, KM

    static struct fb_info fb_info;
    static const struct fb_var_screeninfo maxinefb_defined = {
    .xres =		1024,
    .yres =		768,
    .xres_virtual =	1024,
    .yres_virtual =	768,
    .bits_per_pixel =8,
    .activate =	FB_ACTIVATE_NOW,
    .height =	-1,
    .width =	-1,
    .vmode =	FB_VMODE_NONINTERLACED,
    };
    static struct fb_fix_screeninfo maxinefb_fix __initdata = {
    .id =		"Maxine",
    .smem_len =	(1024*768),
    .type =		FB_TYPE_PACKED_PIXELS,
    .visual =	FB_VISUAL_PSEUDOCOLOR,
    .line_length =	1024,
    };
// Handle the funny Inmos RamDAC/video controller ...
#[no_mangle]
unsafe extern "C" fn maxinefb_ims332_write_register(regno: c_int, val: register unsigned int) {
    static void maxinefb_ims332_write_register(int regno, register unsigned int val)
    {
    register unsigned char *regs = (char *) MAXINEFB_IMS332_ADDRESS;
    unsigned char *wptr;
    wptr = regs + 0xa0000 + (regno << 4);
// ((volatile unsigned int *) (regs)) = (val >> 8) & 0xff00;
// ((volatile unsigned short *) (wptr)) = val;
    }

// dead code: leave here for hardware interface documentation
#[no_mangle]
unsafe extern "C" fn maxinefb_ims332_read_register(regno: c_int) -> c_uint {
    static unsigned int maxinefb_ims332_read_register(int regno)
    {
    register unsigned char *regs = (char *) MAXINEFB_IMS332_ADDRESS;
    unsigned char *rptr;
    register unsigned int j, k;
    rptr = regs + 0x80000 + (regno << 4);
    j = *((volatile unsigned short *) rptr);
    k = *((volatile unsigned short *) regs);
    return (j & 0xffff) | ((k & 0xff00) << 8);
    }

// Set the palette
    static int maxinefb_setcolreg(unsigned regno, unsigned red, unsigned green,
    unsigned blue, unsigned transp, struct fb_info *info)
    {
// value to be written into the palette reg.
    let mut hw_colorvalue: c_ulong = 0;
    if (regno > 255)
    return 1;
    red   >>= 8;    /* The cmap fields are 16 bits    */
    green >>= 8;    /* wide, but the harware colormap */
    blue  >>= 8;    /* registers are only 8 bits wide */
    hw_colorvalue = (blue << 16) + (green << 8) + (red);
    maxinefb_ims332_write_register(IMS332_REG_COLOR_PALETTE + regno,
    hw_colorvalue);
    return 0;
    }
    static const struct fb_ops maxinefb_ops = {
    .owner		= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_setcolreg	= maxinefb_setcolreg,
    };
#[no_mangle]
unsafe extern "C" fn maxinefb_init() -> int __init {
    static int __init maxinefb_init(void)
    {
    unsigned long fboff;
    unsigned long fb_start;
    int i;
    if (fb_get_options("maxinefb", core::ptr::null_mut()))
    return -ENODEV;
// Validate we're on the proper machine type
    if (mips_machtype != MACH_DS5000_XX) {
    return -EINVAL;
    }
    printk(KERN_INFO "Maxinefb: Personal DECstation detected\n");
    printk(KERN_INFO "Maxinefb: initializing onboard framebuffer\n");
// Framebuffer display memory base address
    fb_start = DS5000_xx_ONBOARD_FBMEM_START;
// Clear screen
    for (fboff = fb_start; fboff < fb_start + 0x1ffff; fboff++)
// (volatile unsigned char *)fboff = 0x0;
    maxinefb_fix.smem_start = fb_start;
// erase hardware cursor
    for (i = 0; i < 512; i++) {
    maxinefb_ims332_write_register(IMS332_REG_CURSOR_RAM + i,
    0);
//
    if (i&0x8 == 0)
    maxinefb_ims332_write_register (IMS332_REG_CURSOR_RAM + i, 0x0f);
    else
    maxinefb_ims332_write_register (IMS332_REG_CURSOR_RAM + i, 0xf0);
//
    }
    fb_info.fbops = &maxinefb_ops;
    fb_info.screen_base = (char *)maxinefb_fix.smem_start;
    fb_info.var = maxinefb_defined;
    fb_info.fix = maxinefb_fix;
    fb_alloc_cmap(&fb_info.cmap, 256, 0);
    if (register_framebuffer(&fb_info) < 0)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn maxinefb_exit() -> void __exit {
    static void __exit maxinefb_exit(void)
    {
    unregister_framebuffer(&fb_info);
    }
    module_init(maxinefb_init);
    module_exit(maxinefb_exit);
    MODULE_LICENSE("GPL");
