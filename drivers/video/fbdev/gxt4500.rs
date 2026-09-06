//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/gxt4500.c
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
//
// Frame buffer device for IBM GXT4500P/6500P and GXT4000P/6000P
// display adaptors
//
// Copyright (C) 2006 Paul Mackerras, IBM Corp. <paulus@samba.org>
//

pub const PCI_DEVICE_ID_IBM_GXT4500P: c_uint = 0x21c;
pub const PCI_DEVICE_ID_IBM_GXT6500P: c_uint = 0x21b;
pub const PCI_DEVICE_ID_IBM_GXT4000P: c_uint = 0x16e;
pub const PCI_DEVICE_ID_IBM_GXT6000P: c_uint = 0x170;
// GXT4500P registers
// Registers in PCI config space
pub const CFG_ENDIAN0: c_uint = 0x40;
// Misc control/status registers
pub const STATUS: c_uint = 0x1000;
pub const CTRL_REG0: c_uint = 0x1004;
pub const CR0_HALT_DMA: c_uint = 0x4;
pub const CR0_RASTER_RESET: c_uint = 0x8;
pub const CR0_GEOM_RESET: c_uint = 0x10;
pub const CR0_MEM_CTRLER_RESET: c_uint = 0x20;
// Framebuffer control registers
pub const FB_AB_CTRL: c_uint = 0x1100;
pub const FB_CD_CTRL: c_uint = 0x1104;
pub const FB_WID_CTRL: c_uint = 0x1108;
pub const FB_Z_CTRL: c_uint = 0x110c;
pub const FB_VGA_CTRL: c_uint = 0x1110;
pub const REFRESH_AB_CTRL: c_uint = 0x1114;
pub const REFRESH_CD_CTRL: c_uint = 0x1118;
pub const FB_OVL_CTRL: c_uint = 0x111c;
pub const FB_CTRL_TYPE: c_uint = 0x80000000;
pub const FB_CTRL_WIDTH_MASK: c_uint = 0x007f0000;
pub const FB_CTRL_WIDTH_SHIFT: c_int = 16;
pub const FB_CTRL_START_SEG_MASK: c_uint = 0x00003fff;
pub const REFRESH_START: c_uint = 0x1098;
pub const REFRESH_SIZE: c_uint = 0x109c;
// "Direct" framebuffer access registers
pub const DFA_FB_A: c_uint = 0x11e0;
pub const DFA_FB_B: c_uint = 0x11e4;
pub const DFA_FB_C: c_uint = 0x11e8;
pub const DFA_FB_D: c_uint = 0x11ec;
pub const DFA_FB_ENABLE: c_uint = 0x80000000;
pub const DFA_FB_BASE_MASK: c_uint = 0x03f00000;
pub const DFA_FB_STRIDE_1k: c_uint = 0x00000000;
pub const DFA_FB_STRIDE_2k: c_uint = 0x00000010;
pub const DFA_FB_STRIDE_4k: c_uint = 0x00000020;
pub const DFA_PIX_8BIT: c_uint = 0x00000000;
pub const DFA_PIX_16BIT_565: c_uint = 0x00000001;
pub const DFA_PIX_16BIT_1555: c_uint = 0x00000002;
pub const DFA_PIX_24BIT: c_uint = 0x00000004;
pub const DFA_PIX_32BIT: c_uint = 0x00000005;
// maps DFA_PIX_* to pixel size in bytes
    static const unsigned char pixsize[] = {
    1, 2, 2, 2, 4, 4
    };
// Display timing generator registers
pub const DTG_CONTROL: c_uint = 0x1900;
pub const DTG_CTL_SCREEN_REFRESH: c_int = 2;
pub const DTG_CTL_ENABLE: c_int = 1;
pub const DTG_HORIZ_EXTENT: c_uint = 0x1904;
pub const DTG_HORIZ_DISPLAY: c_uint = 0x1908;
pub const DTG_HSYNC_START: c_uint = 0x190c;
pub const DTG_HSYNC_END: c_uint = 0x1910;
pub const DTG_HSYNC_END_COMP: c_uint = 0x1914;
pub const DTG_VERT_EXTENT: c_uint = 0x1918;
pub const DTG_VERT_DISPLAY: c_uint = 0x191c;
pub const DTG_VSYNC_START: c_uint = 0x1920;
pub const DTG_VSYNC_END: c_uint = 0x1924;
pub const DTG_VERT_SHORT: c_uint = 0x1928;
// PLL/RAMDAC registers
pub const DISP_CTL: c_uint = 0x402c;
pub const DISP_CTL_OFF: c_int = 2;
pub const SYNC_CTL: c_uint = 0x4034;
pub const SYNC_CTL_SYNC_ON_RGB: c_int = 1;
pub const SYNC_CTL_SYNC_OFF: c_int = 2;
pub const SYNC_CTL_HSYNC_INV: c_int = 8;
pub const SYNC_CTL_VSYNC_INV: c_uint = 0x10;
pub const SYNC_CTL_HSYNC_OFF: c_uint = 0x20;
pub const SYNC_CTL_VSYNC_OFF: c_uint = 0x40;
pub const PLL_M: c_uint = 0x4040;
pub const PLL_N: c_uint = 0x4044;
pub const PLL_POSTDIV: c_uint = 0x4048;
pub const PLL_C: c_uint = 0x404c;
// Hardware cursor
pub const CURSOR_X: c_uint = 0x4078;
pub const CURSOR_Y: c_uint = 0x407c;
pub const CURSOR_HOTSPOT: c_uint = 0x4080;
pub const CURSOR_MODE: c_uint = 0x4084;
pub const CURSOR_MODE_OFF: c_int = 0;
pub const CURSOR_MODE_4BPP: c_int = 1;
pub const CURSOR_PIXMAP: c_uint = 0x5000;
pub const CURSOR_CMAP: c_uint = 0x7400;
// Window attribute table
pub const WAT_FMT: c_uint = 0x4100;
pub const WAT_FMT_24BIT: c_int = 0;
pub const WAT_FMT_16BIT_565: c_int = 1;
pub const WAT_FMT_16BIT_1555: c_int = 2;

pub const WAT_FMT_8BIT_332: c_int = 9;
pub const WAT_FMT_8BIT: c_uint = 0xa;

pub const WAT_CMAP_OFFSET: c_uint = 0x4104		/* 4-bit value gets << 6 */;
pub const WAT_CTRL: c_uint = 0x4108;

pub const WAT_CTRL_NO_INC: c_int = 2;
pub const WAT_GAMMA_CTRL: c_uint = 0x410c;

pub const WAT_OVL_CTRL: c_uint = 0x430c		/* controls overlay */;
// Indexed by DFA_PIX_* values
    static const unsigned char watfmt[] = {
    WAT_FMT_8BIT, WAT_FMT_16BIT_565, WAT_FMT_16BIT_1555, 0,
    WAT_FMT_24BIT, WAT_FMT_32BIT
    };
// Colormap array; 1k entries of 4 bytes each
pub const CMAP: c_uint = 0x6000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxt4500_par {
    pub regs: *mut void __iomem,
    pub wc_cookie: c_int,
    pub /: *mut *mut *mut int pixfmt; / pixel format, see DFA_PIX_ values,
// PLL parameters
    pub /: *mut *mut int refclk_ps; / ref clock period in picoseconds,
    pub /: *mut *mut int pll_m; / ref clock divisor,
    pub /: *mut *mut int pll_n; / VCO divisor,
    pub /: *mut *mut int pll_pd1; / first post-divisor,
    pub /: *mut *mut int pll_pd2; / second post-divisor,
    pub /: *mut *mut u32 pseudo_palette[16]; / used in color blits,
}

// mode requested by user
    static char *mode_option;
// default mode: 1280x1024 @ 60 Hz, 8 bpp
    static const struct fb_videomode defaultmode = {
    .refresh = 60,
    .xres = 1280,
    .yres = 1024,
    .pixclock = 9295,
    .left_margin = 248,
    .right_margin = 48,
    .upper_margin = 38,
    .lower_margin = 1,
    .hsync_len = 112,
    .vsync_len = 3,
    .vmode = FB_VMODE_NONINTERLACED
    };
// List of supported cards
    enum gxt_cards {
    GXT4500P,
    GXT6500P,
    GXT4000P,
    GXT6000P
    };
// Card-specific information
    static const struct cardinfo {
    int	refclk_ps;	/* period of PLL reference clock in ps */
    const char *cardname;
    } cardinfo[] = {
    [GXT4500P] = { .refclk_ps = 9259, .cardname = "IBM GXT4500P" },
    [GXT6500P] = { .refclk_ps = 9259, .cardname = "IBM GXT6500P" },
    [GXT4000P] = { .refclk_ps = 40000, .cardname = "IBM GXT4000P" },
    [GXT6000P] = { .refclk_ps = 40000, .cardname = "IBM GXT6000P" },
    };
//
// The refclk and VCO dividers appear to use a linear feedback shift
// register, which gets reloaded when it reaches a terminal value, at
// which point the divider output is toggled.  Thus one can obtain
// whatever divisor is required by putting the appropriate value into
// the reload register.  For a divisor of N, one puts the value from
// the LFSR sequence that comes N-1 places before the terminal value
// into the reload register.
//
    static const unsigned char mdivtab[] = {
// 1 */		      0x3f, 0x00, 0x20, 0x10, 0x28, 0x14, 0x2a, 0x15, 0x0a,
// 10 */	0x25, 0x32, 0x19, 0x0c, 0x26, 0x13, 0x09, 0x04, 0x22, 0x11,
// 20 */	0x08, 0x24, 0x12, 0x29, 0x34, 0x1a, 0x2d, 0x36, 0x1b, 0x0d,
// 30 */	0x06, 0x23, 0x31, 0x38, 0x1c, 0x2e, 0x17, 0x0b, 0x05, 0x02,
// 40 */	0x21, 0x30, 0x18, 0x2c, 0x16, 0x2b, 0x35, 0x3a, 0x1d, 0x0e,
// 50 */	0x27, 0x33, 0x39, 0x3c, 0x1e, 0x2f, 0x37, 0x3b, 0x3d, 0x3e,
// 60 */	0x1f, 0x0f, 0x07, 0x03, 0x01,
    };
    static const unsigned char ndivtab[] = {
// 2 */		            0x00, 0x80, 0xc0, 0xe0, 0xf0, 0x78, 0xbc, 0x5e,
// 10 */	0x2f, 0x17, 0x0b, 0x85, 0xc2, 0xe1, 0x70, 0x38, 0x9c, 0x4e,
// 20 */	0xa7, 0xd3, 0xe9, 0xf4, 0xfa, 0xfd, 0xfe, 0x7f, 0xbf, 0xdf,
// 30 */	0xef, 0x77, 0x3b, 0x1d, 0x8e, 0xc7, 0xe3, 0x71, 0xb8, 0xdc,
// 40 */	0x6e, 0xb7, 0x5b, 0x2d, 0x16, 0x8b, 0xc5, 0xe2, 0xf1, 0xf8,
// 50 */	0xfc, 0x7e, 0x3f, 0x9f, 0xcf, 0x67, 0xb3, 0xd9, 0x6c, 0xb6,
// 60 */	0xdb, 0x6d, 0x36, 0x9b, 0x4d, 0x26, 0x13, 0x89, 0xc4, 0x62,
// 70 */	0xb1, 0xd8, 0xec, 0xf6, 0xfb, 0x7d, 0xbe, 0x5f, 0xaf, 0x57,
// 80 */	0x2b, 0x95, 0x4a, 0x25, 0x92, 0x49, 0xa4, 0x52, 0x29, 0x94,
// 90 */	0xca, 0x65, 0xb2, 0x59, 0x2c, 0x96, 0xcb, 0xe5, 0xf2, 0x79,
// 100 */	0x3c, 0x1e, 0x0f, 0x07, 0x83, 0x41, 0x20, 0x90, 0x48, 0x24,
// 110 */	0x12, 0x09, 0x84, 0x42, 0xa1, 0x50, 0x28, 0x14, 0x8a, 0x45,
// 120 */	0xa2, 0xd1, 0xe8, 0x74, 0xba, 0xdd, 0xee, 0xf7, 0x7b, 0x3d,
// 130 */	0x9e, 0x4f, 0x27, 0x93, 0xc9, 0xe4, 0x72, 0x39, 0x1c, 0x0e,
// 140 */	0x87, 0xc3, 0x61, 0x30, 0x18, 0x8c, 0xc6, 0x63, 0x31, 0x98,
// 150 */	0xcc, 0xe6, 0x73, 0xb9, 0x5c, 0x2e, 0x97, 0x4b, 0xa5, 0xd2,
// 160 */	0x69,
    };
#[no_mangle]
unsafe extern "C" fn calc_pll(period_ps: c_int, par: *mut gxt4500_par) -> c_int {
    static int calc_pll(int period_ps, struct gxt4500_par *par)
    {
    int m, n, pdiv1, pdiv2, postdiv;
    int pll_period, best_error, t, intf;
// only deal with range 5MHz - 300MHz
    if (period_ps < 3333 || period_ps > 200000)
    return -1;
    best_error = 1000000;
    for (pdiv1 = 1; pdiv1 <= 8; ++pdiv1) {
    for (pdiv2 = 1; pdiv2 <= pdiv1; ++pdiv2) {
    postdiv = pdiv1 * pdiv2;
    pll_period = DIV_ROUND_UP(period_ps, postdiv);
// keep pll in range 350..600 MHz
    if (pll_period < 1666 || pll_period > 2857)
    continue;
    for (m = 1; m <= 64; ++m) {
    intf = m * par.refclk_ps;
    if (intf > 500000)
    break;
    n = intf * postdiv / period_ps;
    if (n < 3 || n > 160)
    continue;
    t = par.refclk_ps * m * postdiv / n;
    t -= period_ps;
    if (t >= 0 && t < best_error) {
    par.pll_m = m;
    par.pll_n = n;
    par.pll_pd1 = pdiv1;
    par.pll_pd2 = pdiv2;
    best_error = t;
    }
    }
    }
    }
    if (best_error == 1000000)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn calc_pixclock(par: *mut gxt4500_par) -> c_int {
    static int calc_pixclock(struct gxt4500_par *par)
    {
    return par.refclk_ps * par.pll_m * par.pll_pd1 * par.pll_pd2
    / par.pll_n;
    }
    static int gxt4500_var_to_par(struct fb_var_screeninfo *var,
    struct gxt4500_par *par)
    {
    if (var.xres + var.xoffset > var.xres_virtual ||
    var.yres + var.yoffset > var.yres_virtual ||
    var.xres_virtual > 4096)
    return -EINVAL;
    if ((var.vmode & FB_VMODE_MASK) != FB_VMODE_NONINTERLACED)
    return -EINVAL;
    if (calc_pll(var.pixclock, par) < 0)
    return -EINVAL;
    switch (var.bits_per_pixel) {
    case 32:
    if (var.transp.length)
    par.pixfmt = DFA_PIX_32BIT;
    else
    par.pixfmt = DFA_PIX_24BIT;
    break;
    case 24:
    par.pixfmt = DFA_PIX_24BIT;
    break;
    case 16:
    if (var.green.length == 5)
    par.pixfmt = DFA_PIX_16BIT_1555;
    else
    par.pixfmt = DFA_PIX_16BIT_565;
    break;
    case 8:
    par.pixfmt = DFA_PIX_8BIT;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    let mut eightbits: static struct fb_bitfield = {0, 8};
    let mut nobits: static struct fb_bitfield = {0, 0};
    static void gxt4500_unpack_pixfmt(struct fb_var_screeninfo *var,
    int pixfmt)
    {
    var.bits_per_pixel = pixsize[pixfmt] * 8;
    var.red = eightbits;
    var.green = eightbits;
    var.blue = eightbits;
    var.transp = nobits;
    switch (pixfmt) {
    case DFA_PIX_16BIT_565:
    var.red.length = 5;
    var.green.length = 6;
    var.blue.length = 5;
    break;
    case DFA_PIX_16BIT_1555:
    var.red.length = 5;
    var.green.length = 5;
    var.blue.length = 5;
    var.transp.length = 1;
    break;
    case DFA_PIX_32BIT:
    var.transp.length = 8;
    break;
    }
    if (pixfmt != DFA_PIX_8BIT) {
    var.blue.offset = 0;
    var.green.offset = var.blue.length;
    var.red.offset = var.green.offset + var.green.length;
    if (var.transp.length)
    var.transp.offset =
    var.red.offset + var.red.length;
    }
    }
    static int gxt4500_check_var(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    struct gxt4500_par par;
    int err;
    par = *(struct gxt4500_par *)info.par;
    err = gxt4500_var_to_par(var, &par);
    if (!err) {
    var.pixclock = calc_pixclock(&par);
    gxt4500_unpack_pixfmt(var, par.pixfmt);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn gxt4500_set_par(info: *mut fb_info) -> c_int {
    static int gxt4500_set_par(struct fb_info *info)
    {
    struct gxt4500_par *par = info.par;
    struct fb_var_screeninfo *var = &info.var;
    int err;
    u32 ctrlreg, tmp;
    unsigned int dfa_ctl, pixfmt, stride;
    unsigned int wid_tiles, i;
    unsigned int prefetch_pix, htot;
    struct gxt4500_par save_par;
    save_par = *par;
    err = gxt4500_var_to_par(var, par);
    if (err) {
// par = save_par;
    return err;
    }
// turn off DTG for now
    ctrlreg = readreg(par, DTG_CONTROL);
    ctrlreg &= ~(DTG_CTL_ENABLE | DTG_CTL_SCREEN_REFRESH);
    writereg(par, DTG_CONTROL, ctrlreg);
// set PLL registers
    tmp = readreg(par, PLL_C) & ~0x7f;
    if (par.pll_n < 38)
    tmp |= 0x29;
    if (par.pll_n < 69)
    tmp |= 0x35;
#[no_mangle]
pub unsafe extern "C" fn if(100: par->pll_n <) -> else {
    else if (par.pll_n < 100)
    tmp |= 0x76;
    else
    tmp |= 0x7e;
    writereg(par, PLL_C, tmp);
    writereg(par, PLL_M, mdivtab[par.pll_m - 1]);
    writereg(par, PLL_N, ndivtab[par.pll_n - 2]);
    tmp = ((8 - par.pll_pd2) << 3) | (8 - par.pll_pd1);
    if (par.pll_pd1 == 8 || par.pll_pd2 == 8) {
// work around erratum
    writereg(par, PLL_POSTDIV, tmp | 0x9);
    udelay(1);
    }
    writereg(par, PLL_POSTDIV, tmp);
    msleep(20);
// turn off hardware cursor
    writereg(par, CURSOR_MODE, CURSOR_MODE_OFF);
// reset raster engine
    writereg(par, CTRL_REG0, CR0_RASTER_RESET | (CR0_RASTER_RESET << 16));
    udelay(10);
    writereg(par, CTRL_REG0, CR0_RASTER_RESET << 16);
// set display timing generator registers
    htot = var.xres + var.left_margin + var.right_margin +
    var.hsync_len;
    writereg(par, DTG_HORIZ_EXTENT, htot - 1);
    writereg(par, DTG_HORIZ_DISPLAY, var.xres - 1);
    writereg(par, DTG_HSYNC_START, var.xres + var.right_margin - 1);
    writereg(par, DTG_HSYNC_END,
    var.xres + var.right_margin + var.hsync_len - 1);
    writereg(par, DTG_HSYNC_END_COMP,
    var.xres + var.right_margin + var.hsync_len - 1);
    writereg(par, DTG_VERT_EXTENT,
    var.yres + var.upper_margin + var.lower_margin +
    var.vsync_len - 1);
    writereg(par, DTG_VERT_DISPLAY, var.yres - 1);
    writereg(par, DTG_VSYNC_START, var.yres + var.lower_margin - 1);
    writereg(par, DTG_VSYNC_END,
    var.yres + var.lower_margin + var.vsync_len - 1);
    prefetch_pix = 3300000 / var.pixclock;
    if (prefetch_pix >= htot)
    prefetch_pix = htot - 1;
    writereg(par, DTG_VERT_SHORT, htot - prefetch_pix - 1);
    ctrlreg |= DTG_CTL_ENABLE | DTG_CTL_SCREEN_REFRESH;
    writereg(par, DTG_CONTROL, ctrlreg);
// calculate stride in DFA aperture
    if (var.xres_virtual > 2048) {
    stride = 4096;
    dfa_ctl = DFA_FB_STRIDE_4k;
    } else if (var.xres_virtual > 1024) {
    stride = 2048;
    dfa_ctl = DFA_FB_STRIDE_2k;
    } else {
    stride = 1024;
    dfa_ctl = DFA_FB_STRIDE_1k;
    }
// Set up framebuffer definition
    wid_tiles = (var.xres_virtual + 63) >> 6;
// XXX add proper FB allocation here someday
    writereg(par, FB_AB_CTRL, FB_CTRL_TYPE | (wid_tiles << 16) | 0);
    writereg(par, REFRESH_AB_CTRL, FB_CTRL_TYPE | (wid_tiles << 16) | 0);
    writereg(par, FB_CD_CTRL, FB_CTRL_TYPE | (wid_tiles << 16) | 0);
    writereg(par, REFRESH_CD_CTRL, FB_CTRL_TYPE | (wid_tiles << 16) | 0);
    writereg(par, REFRESH_START, (var.xoffset << 16) | var.yoffset);
    writereg(par, REFRESH_SIZE, (var.xres << 16) | var.yres);
// Set up framebuffer access by CPU
    pixfmt = par.pixfmt;
    dfa_ctl |= DFA_FB_ENABLE | pixfmt;
    writereg(par, DFA_FB_A, dfa_ctl);
//
// Set up window attribute table.
// We set all WAT entries the same so it doesn't matter what the
// window ID (WID) plane contains.
//
    for (i = 0; i < 32; ++i) {
    writereg(par, WAT_FMT + (i << 4), watfmt[pixfmt]);
    writereg(par, WAT_CMAP_OFFSET + (i << 4), 0);
    writereg(par, WAT_CTRL + (i << 4), 0);
    writereg(par, WAT_GAMMA_CTRL + (i << 4), WAT_GAMMA_DISABLE);
    }
// Set sync polarity etc.
    ctrlreg = readreg(par, SYNC_CTL) &
    ~(SYNC_CTL_SYNC_ON_RGB | SYNC_CTL_HSYNC_INV |
    SYNC_CTL_VSYNC_INV);
    if (var.sync & FB_SYNC_ON_GREEN)
    ctrlreg |= SYNC_CTL_SYNC_ON_RGB;
    if (!(var.sync & FB_SYNC_HOR_HIGH_ACT))
    ctrlreg |= SYNC_CTL_HSYNC_INV;
    if (!(var.sync & FB_SYNC_VERT_HIGH_ACT))
    ctrlreg |= SYNC_CTL_VSYNC_INV;
    writereg(par, SYNC_CTL, ctrlreg);
    info.fix.line_length = stride * pixsize[pixfmt];
    info.fix.visual = (pixfmt == DFA_PIX_8BIT)? FB_VISUAL_PSEUDOCOLOR:
    FB_VISUAL_DIRECTCOLOR;
    return 0;
    }
    static int gxt4500_setcolreg(unsigned int reg, unsigned int red,
    unsigned int green, unsigned int blue,
    unsigned int transp, struct fb_info *info)
    {
    u32 cmap_entry;
    struct gxt4500_par *par = info.par;
    if (reg > 1023)
    return 1;
    cmap_entry = ((transp & 0xff00) << 16) | ((red & 0xff00) << 8) |
    (green & 0xff00) | (blue >> 8);
    writereg(par, CMAP + reg * 4, cmap_entry);
    if (reg < 16 && par.pixfmt != DFA_PIX_8BIT) {
    u32 *pal = info.pseudo_palette;
    let mut val: u32 = reg;
    switch (par.pixfmt) {
    case DFA_PIX_16BIT_565:
    val |= (reg << 11) | (reg << 5);
    break;
    case DFA_PIX_16BIT_1555:
    val |= (reg << 10) | (reg << 5);
    break;
    case DFA_PIX_32BIT:
    val |= (reg << 24);
    fallthrough;
    case DFA_PIX_24BIT:
    val |= (reg << 16) | (reg << 8);
    break;
    }
    pal[reg] = val;
    }
    return 0;
    }
    static int gxt4500_pan_display(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    struct gxt4500_par *par = info.par;
    if (var.xoffset & 7)
    return -EINVAL;
    if (var.xoffset + info.var.xres > info.var.xres_virtual ||
    var.yoffset + info.var.yres > info.var.yres_virtual)
    return -EINVAL;
    writereg(par, REFRESH_START, (var.xoffset << 16) | var.yoffset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxt4500_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int gxt4500_blank(int blank, struct fb_info *info)
    {
    struct gxt4500_par *par = info.par;
    int ctrl, dctl;
    ctrl = readreg(par, SYNC_CTL);
    ctrl &= ~(SYNC_CTL_SYNC_OFF | SYNC_CTL_HSYNC_OFF | SYNC_CTL_VSYNC_OFF);
    dctl = readreg(par, DISP_CTL);
    dctl |= DISP_CTL_OFF;
    switch (blank) {
    case FB_BLANK_UNBLANK:
    dctl &= ~DISP_CTL_OFF;
    break;
    case FB_BLANK_POWERDOWN:
    ctrl |= SYNC_CTL_SYNC_OFF;
    break;
    case FB_BLANK_HSYNC_SUSPEND:
    ctrl |= SYNC_CTL_HSYNC_OFF;
    break;
    case FB_BLANK_VSYNC_SUSPEND:
    ctrl |= SYNC_CTL_VSYNC_OFF;
    break;
    default: ;
    }
    writereg(par, SYNC_CTL, ctrl);
    writereg(par, DISP_CTL, dctl);
    return 0;
    }
    static const struct fb_fix_screeninfo gxt4500_fix = {
    .id = "IBM GXT4500P",
    .type = FB_TYPE_PACKED_PIXELS,
    .visual = FB_VISUAL_PSEUDOCOLOR,
    .xpanstep = 8,
    .ypanstep = 1,
    .mmio_len = 0x20000,
    };
    static const struct fb_ops gxt4500_ops = {
    .owner = THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_check_var = gxt4500_check_var,
    .fb_set_par = gxt4500_set_par,
    .fb_setcolreg = gxt4500_setcolreg,
    .fb_pan_display = gxt4500_pan_display,
    .fb_blank = gxt4500_blank,
    };
// PCI functions
#[no_mangle]
unsafe extern "C" fn gxt4500_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int gxt4500_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    int err;
    unsigned long reg_phys, fb_phys;
    struct gxt4500_par *par;
    struct fb_info *info;
    struct fb_var_screeninfo var;
    enum gxt_cards cardtype;
    err = aperture_remove_conflicting_pci_devices(pdev, "gxt4500fb");
    if (err)
    return err;
    err = pci_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "gxt4500: cannot enable PCI device: %d\n",
    err);
    return err;
    }
    reg_phys = pci_resource_start(pdev, 0);
    if (!request_mem_region(reg_phys, pci_resource_len(pdev, 0),
    "gxt4500 regs")) {
    dev_err(&pdev.dev, "gxt4500: cannot get registers\n");
    goto err_nodev;
    }
    fb_phys = pci_resource_start(pdev, 1);
    if (!request_mem_region(fb_phys, pci_resource_len(pdev, 1),
    "gxt4500 FB")) {
    dev_err(&pdev.dev, "gxt4500: cannot get framebuffer\n");
    goto err_free_regs;
    }
    info = framebuffer_alloc(sizeof(struct gxt4500_par), &pdev.dev);
    if (!info)
    goto err_free_fb;
    par = info.par;
    cardtype = ent.driver_data;
    par.refclk_ps = cardinfo[cardtype].refclk_ps;
    info.fix = gxt4500_fix;
    strscpy(info.fix.id, cardinfo[cardtype].cardname,
    sizeof(info.fix.id));
    info.pseudo_palette = par.pseudo_palette;
    info.fix.mmio_start = reg_phys;
    par.regs = pci_ioremap_bar(pdev, 0);
    if (!par.regs) {
    dev_err(&pdev.dev, "gxt4500: cannot map registers\n");
    goto err_free_all;
    }
    info.fix.smem_start = fb_phys;
    info.fix.smem_len = pci_resource_len(pdev, 1);
    info.screen_base = pci_ioremap_wc_bar(pdev, 1);
    if (!info.screen_base) {
    dev_err(&pdev.dev, "gxt4500: cannot map framebuffer\n");
    goto err_unmap_regs;
    }
    pci_set_drvdata(pdev, info);
    par.wc_cookie = arch_phys_wc_add(info.fix.smem_start,
    info.fix.smem_len);

// Set byte-swapping for DFA aperture for all pixel sizes
    pci_write_config_dword(pdev, CFG_ENDIAN0, 0x333300);

// not sure what this means but fgl23 driver does that
    pci_write_config_dword(pdev, CFG_ENDIAN0, 0x2300);
// pci_write_config_dword(pdev, CFG_ENDIAN0 + 4, 0x400000);
    pci_write_config_dword(pdev, CFG_ENDIAN0 + 8, 0x98530000);

    info.fbops = &gxt4500_ops;
    info.flags = FBINFO_HWACCEL_XPAN | FBINFO_HWACCEL_YPAN;
    err = fb_alloc_cmap(&info.cmap, 256, 0);
    if (err) {
    dev_err(&pdev.dev, "gxt4500: cannot allocate cmap\n");
    goto err_unmap_all;
    }
    gxt4500_blank(FB_BLANK_UNBLANK, info);
    if (!fb_find_mode(&var, info, mode_option, core::ptr::null_mut(), 0, &defaultmode, 8)) {
    dev_err(&pdev.dev, "gxt4500: cannot find valid video mode\n");
    goto err_free_cmap;
    }
    info.var = var;
    if (gxt4500_set_par(info)) {
    dev_err(&pdev.dev, "cannot set video mode\n");
    goto err_free_cmap;
    }
    if (register_framebuffer(info) < 0) {
    dev_err(&pdev.dev, "gxt4500: cannot register framebuffer\n");
    goto err_free_cmap;
    }
    fb_info(info, "%s frame buffer device\n", info.fix.id);
    return 0;
    err_free_cmap:
    fb_dealloc_cmap(&info.cmap);
    err_unmap_all:
    iounmap(info.screen_base);
    err_unmap_regs:
    iounmap(par.regs);
    err_free_all:
    framebuffer_release(info);
    err_free_fb:
    release_mem_region(fb_phys, pci_resource_len(pdev, 1));
    err_free_regs:
    release_mem_region(reg_phys, pci_resource_len(pdev, 0));
    err_nodev:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn gxt4500_remove(pdev: *mut pci_dev) {
    static void gxt4500_remove(struct pci_dev *pdev)
    {
    struct fb_info *info = pci_get_drvdata(pdev);
    struct gxt4500_par *par;
    if (!info)
    return;
    par = info.par;
    unregister_framebuffer(info);
    arch_phys_wc_del(par.wc_cookie);
    fb_dealloc_cmap(&info.cmap);
    iounmap(par.regs);
    iounmap(info.screen_base);
    release_mem_region(pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0));
    release_mem_region(pci_resource_start(pdev, 1),
    pci_resource_len(pdev, 1));
    framebuffer_release(info);
    }
// supported chipsets
    static const struct pci_device_id gxt4500_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_IBM, PCI_DEVICE_ID_IBM_GXT4500P),
    .driver_data = GXT4500P },
    { PCI_DEVICE(PCI_VENDOR_ID_IBM, PCI_DEVICE_ID_IBM_GXT6500P),
    .driver_data = GXT6500P },
    { PCI_DEVICE(PCI_VENDOR_ID_IBM, PCI_DEVICE_ID_IBM_GXT4000P),
    .driver_data = GXT4000P },
    { PCI_DEVICE(PCI_VENDOR_ID_IBM, PCI_DEVICE_ID_IBM_GXT6000P),
    .driver_data = GXT6000P },
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, gxt4500_pci_tbl);
    static struct pci_driver gxt4500_driver = {
    .name = "gxt4500",
    .id_table = gxt4500_pci_tbl,
    .probe = gxt4500_probe,
    .remove = gxt4500_remove,
    };
#[no_mangle]
unsafe extern "C" fn gxt4500_init() -> c_int {
    static int gxt4500_init(void)
    {
    if (fb_modesetting_disabled("gxt4500"))
    return -ENODEV;

    if (fb_get_options("gxt4500", &mode_option))
    return -ENODEV;

    return pci_register_driver(&gxt4500_driver);
    }
    module_init(gxt4500_init);
#[no_mangle]
unsafe extern "C" fn gxt4500_exit() -> void __exit {
    static void __exit gxt4500_exit(void)
    {
    pci_unregister_driver(&gxt4500_driver);
    }
    module_exit(gxt4500_exit);
    MODULE_AUTHOR("Paul Mackerras <paulus@samba.org>");
    MODULE_DESCRIPTION("FBDev driver for IBM GXT4500P/6500P and GXT4000P/6000P");
    MODULE_LICENSE("GPL");
    module_param(mode_option, charp, 0);
    MODULE_PARM_DESC(mode_option, "Specify resolution as \"<xres>x<yres>[-<bpp>][@<refresh>]\"");
