//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/ep93xx-fb.c
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
// linux/drivers/video/ep93xx-fb.c
//
// Framebuffer support for the EP93xx series.
//
// Copyright (C) 2007 Bluewater Systems Ltd
// Author: Ryan Mallon
//
// Copyright (c) 2009 H Hartley Sweeten <hsweeten@visionengravers.com>
//
// Based on the Cirrus Logic ep93xxfb driver, and various other ep93xxfb
// drivers.
//

// Vertical Frame Timing Registers
pub const EP93XXFB_VLINES_TOTAL: c_uint = 0x0000	/* SW locked */;
pub const EP93XXFB_VSYNC: c_uint = 0x0004	/* SW locked */;
pub const EP93XXFB_VACTIVE: c_uint = 0x0008	/* SW locked */;
pub const EP93XXFB_VBLANK: c_uint = 0x0228	/* SW locked */;
pub const EP93XXFB_VCLK: c_uint = 0x000c	/* SW locked */;
// Horizontal Frame Timing Registers
pub const EP93XXFB_HCLKS_TOTAL: c_uint = 0x0010	/* SW locked */;
pub const EP93XXFB_HSYNC: c_uint = 0x0014	/* SW locked */;
pub const EP93XXFB_HACTIVE: c_uint = 0x0018	/* SW locked */;
pub const EP93XXFB_HBLANK: c_uint = 0x022c	/* SW locked */;
pub const EP93XXFB_HCLK: c_uint = 0x001c	/* SW locked */;
// Frame Buffer Memory Configuration Registers
pub const EP93XXFB_SCREEN_PAGE: c_uint = 0x0028;
pub const EP93XXFB_SCREEN_HPAGE: c_uint = 0x002c;
pub const EP93XXFB_SCREEN_LINES: c_uint = 0x0030;
pub const EP93XXFB_LINE_LENGTH: c_uint = 0x0034;
pub const EP93XXFB_VLINE_STEP: c_uint = 0x0038;
pub const EP93XXFB_LINE_CARRY: c_uint = 0x003c	/* SW locked */;
pub const EP93XXFB_EOL_OFFSET: c_uint = 0x0230;
// Other Video Registers
pub const EP93XXFB_BRIGHTNESS: c_uint = 0x0020;
pub const EP93XXFB_ATTRIBS: c_uint = 0x0024	/* SW locked */;
pub const EP93XXFB_SWLOCK: c_uint = 0x007c	/* SW locked */;
pub const EP93XXFB_AC_RATE: c_uint = 0x0214;
pub const EP93XXFB_FIFO_LEVEL: c_uint = 0x0234;
pub const EP93XXFB_PIXELMODE: c_uint = 0x0054;

pub const EP93XXFB_PARL_IF_OUT: c_uint = 0x0058;
pub const EP93XXFB_PARL_IF_IN: c_uint = 0x005c;
// Blink Control Registers
pub const EP93XXFB_BLINK_RATE: c_uint = 0x0040;
pub const EP93XXFB_BLINK_MASK: c_uint = 0x0044;
pub const EP93XXFB_BLINK_PATTRN: c_uint = 0x0048;
pub const EP93XXFB_PATTRN_MASK: c_uint = 0x004c;
pub const EP93XXFB_BKGRND_OFFSET: c_uint = 0x0050;
// Hardware Cursor Registers
pub const EP93XXFB_CURSOR_ADR_START: c_uint = 0x0060;
pub const EP93XXFB_CURSOR_ADR_RESET: c_uint = 0x0064;
pub const EP93XXFB_CURSOR_SIZE: c_uint = 0x0068;
pub const EP93XXFB_CURSOR_COLOR1: c_uint = 0x006c;
pub const EP93XXFB_CURSOR_COLOR2: c_uint = 0x0070;
pub const EP93XXFB_CURSOR_BLINK_COLOR1: c_uint = 0x021c;
pub const EP93XXFB_CURSOR_BLINK_COLOR2: c_uint = 0x0220;
pub const EP93XXFB_CURSOR_XY_LOC: c_uint = 0x0074;
pub const EP93XXFB_CURSOR_DSCAN_HY_LOC: c_uint = 0x0078;
pub const EP93XXFB_CURSOR_BLINK_RATE_CTRL: c_uint = 0x0224;
// LUT Registers
pub const EP93XXFB_GRY_SCL_LUTR: c_uint = 0x0080;
pub const EP93XXFB_GRY_SCL_LUTG: c_uint = 0x0280;
pub const EP93XXFB_GRY_SCL_LUTB: c_uint = 0x0300;
pub const EP93XXFB_LUT_SW_CONTROL: c_uint = 0x0218;

pub const EP93XXFB_COLOR_LUT: c_uint = 0x0400;
// Video Signature Registers
pub const EP93XXFB_VID_SIG_RSLT_VAL: c_uint = 0x0200;
pub const EP93XXFB_VID_SIG_CTRL: c_uint = 0x0204;
pub const EP93XXFB_VSIG: c_uint = 0x0208;
pub const EP93XXFB_HSIG: c_uint = 0x020c;
pub const EP93XXFB_SIG_CLR_STR: c_uint = 0x0210;
// Minimum / Maximum resolutions supported
pub const EP93XXFB_MIN_XRES: c_int = 64;
pub const EP93XXFB_MIN_YRES: c_int = 64;
pub const EP93XXFB_MAX_XRES: c_int = 1024;
pub const EP93XXFB_MAX_YRES: c_int = 768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_fbi {
    pub mach_info: *mut ep93xxfb_mach_info,
    pub clk: *mut clk,
    pub res: *mut resource,
    pub mmio_base: *mut void __iomem,
    pub pseudo_palette: [c_uint; 256],
}

    let mut check_screenpage_bug: static int = 1;
    module_param(check_screenpage_bug, int, 0644);
    MODULE_PARM_DESC(check_screenpage_bug,
    "Check for bit 27 screen page bug. Default = 1");
    static inline unsigned int ep93xxfb_readl(struct ep93xx_fbi *fbi,
    unsigned int off)
    {
    return __raw_readl(fbi.mmio_base + off);
    }
    static inline void ep93xxfb_writel(struct ep93xx_fbi *fbi,
    unsigned int val, unsigned int off)
    {
    __raw_writel(val, fbi.mmio_base + off);
    }
//
// Write to one of the locked raster registers.
//
    static inline void ep93xxfb_out_locked(struct ep93xx_fbi *fbi,
    unsigned int val, unsigned int reg)
    {
//
// We don't need a lock or delay here since the raster register
// block will remain unlocked until the next access.
//
    ep93xxfb_writel(fbi, 0xaa, EP93XXFB_SWLOCK);
    ep93xxfb_writel(fbi, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_set_video_attribs(info: *mut fb_info) {
    static void ep93xxfb_set_video_attribs(struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    unsigned int attribs;
    attribs = EP93XXFB_ENABLE;
    attribs |= fbi.mach_info.flags;
    ep93xxfb_out_locked(fbi, attribs, EP93XXFB_ATTRIBS);
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_set_pixelmode(info: *mut fb_info) -> c_int {
    static int ep93xxfb_set_pixelmode(struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    unsigned int val;
    info.var.transp.offset = 0;
    info.var.transp.length = 0;
    switch (info.var.bits_per_pixel) {
    case 8:
    val = EP93XXFB_PIXELMODE_8BPP | EP93XXFB_PIXELMODE_COLOR_LUT |
    EP93XXFB_PIXELMODE_SHIFT_1P_18B;
    info.var.red.offset	= 0;
    info.var.red.length	= 8;
    info.var.green.offset	= 0;
    info.var.green.length	= 8;
    info.var.blue.offset	= 0;
    info.var.blue.length	= 8;
    info.fix.visual 	= FB_VISUAL_PSEUDOCOLOR;
    break;
    case 16:
    val = EP93XXFB_PIXELMODE_16BPP | EP93XXFB_PIXELMODE_COLOR_555 |
    EP93XXFB_PIXELMODE_SHIFT_1P_18B;
    info.var.red.offset	= 11;
    info.var.red.length	= 5;
    info.var.green.offset	= 5;
    info.var.green.length	= 6;
    info.var.blue.offset	= 0;
    info.var.blue.length	= 5;
    info.fix.visual 	= FB_VISUAL_TRUECOLOR;
    break;
    case 24:
    val = EP93XXFB_PIXELMODE_24BPP | EP93XXFB_PIXELMODE_COLOR_888 |
    EP93XXFB_PIXELMODE_SHIFT_1P_24B;
    info.var.red.offset	= 16;
    info.var.red.length	= 8;
    info.var.green.offset	= 8;
    info.var.green.length	= 8;
    info.var.blue.offset	= 0;
    info.var.blue.length	= 8;
    info.fix.visual 	= FB_VISUAL_TRUECOLOR;
    break;
    case 32:
    val = EP93XXFB_PIXELMODE_32BPP | EP93XXFB_PIXELMODE_COLOR_888 |
    EP93XXFB_PIXELMODE_SHIFT_1P_24B;
    info.var.red.offset	= 16;
    info.var.red.length	= 8;
    info.var.green.offset	= 8;
    info.var.green.length	= 8;
    info.var.blue.offset	= 0;
    info.var.blue.length	= 8;
    info.fix.visual 	= FB_VISUAL_TRUECOLOR;
    break;
    default:
    return -EINVAL;
    }
    ep93xxfb_writel(fbi, val, EP93XXFB_PIXELMODE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_set_timing(info: *mut fb_info) {
    static void ep93xxfb_set_timing(struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    unsigned int vlines_total, hclks_total, start, stop;
    vlines_total = info.var.yres + info.var.upper_margin +
    info.var.lower_margin + info.var.vsync_len - 1;
    hclks_total = info.var.xres + info.var.left_margin +
    info.var.right_margin + info.var.hsync_len - 1;
    ep93xxfb_out_locked(fbi, vlines_total, EP93XXFB_VLINES_TOTAL);
    ep93xxfb_out_locked(fbi, hclks_total, EP93XXFB_HCLKS_TOTAL);
    start = vlines_total;
    stop = vlines_total - info.var.vsync_len;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_VSYNC);
    start = vlines_total - info.var.vsync_len - info.var.upper_margin;
    stop = info.var.lower_margin - 1;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_VBLANK);
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_VACTIVE);
    start = vlines_total;
    stop = vlines_total + 1;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_VCLK);
    start = hclks_total;
    stop = hclks_total - info.var.hsync_len;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_HSYNC);
    start = hclks_total - info.var.hsync_len - info.var.left_margin;
    stop = info.var.right_margin - 1;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_HBLANK);
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_HACTIVE);
    start = hclks_total;
    stop = hclks_total;
    ep93xxfb_out_locked(fbi, start | (stop << 16), EP93XXFB_HCLK);
    ep93xxfb_out_locked(fbi, 0x0, EP93XXFB_LINE_CARRY);
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_set_par(info: *mut fb_info) -> c_int {
    static int ep93xxfb_set_par(struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    clk_set_rate(fbi.clk, 1000 * PICOS2KHZ(info.var.pixclock));
    ep93xxfb_set_timing(info);
    info.fix.line_length = info.var.xres_virtual *
    info.var.bits_per_pixel / 8;
    ep93xxfb_writel(fbi, info.fix.smem_start, EP93XXFB_SCREEN_PAGE);
    ep93xxfb_writel(fbi, info.var.yres - 1, EP93XXFB_SCREEN_LINES);
    ep93xxfb_writel(fbi, ((info.var.xres * info.var.bits_per_pixel)
    / 32) - 1, EP93XXFB_LINE_LENGTH);
    ep93xxfb_writel(fbi, info.fix.line_length / 4, EP93XXFB_VLINE_STEP);
    ep93xxfb_set_video_attribs(info);
    return 0;
    }
    static int ep93xxfb_check_var(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    int err;
    err = ep93xxfb_set_pixelmode(info);
    if (err)
    return err;
    var.xres = max_t(unsigned int, var.xres, EP93XXFB_MIN_XRES);
    var.xres = min_t(unsigned int, var.xres, EP93XXFB_MAX_XRES);
    var.xres_virtual = max(var.xres_virtual, var.xres);
    var.yres = max_t(unsigned int, var.yres, EP93XXFB_MIN_YRES);
    var.yres = min_t(unsigned int, var.yres, EP93XXFB_MAX_YRES);
    var.yres_virtual = max(var.yres_virtual, var.yres);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int ep93xxfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    let mut offset: c_uint = vma.vm_pgoff << PAGE_SHIFT;
    vma.vm_page_prot = pgprot_decrypted(vma.vm_page_prot);
    if (offset < info.fix.smem_len) {
    return dma_mmap_wc(info.device, vma, info.screen_base,
    info.fix.smem_start, info.fix.smem_len);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_blank(blank_mode: c_int, info: *mut fb_info) -> c_int {
    static int ep93xxfb_blank(int blank_mode, struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    let mut attribs: c_uint = ep93xxfb_readl(fbi, EP93XXFB_ATTRIBS);
    if (blank_mode) {
    if (fbi.mach_info.blank)
    fbi.mach_info.blank(blank_mode, info);
    ep93xxfb_out_locked(fbi, attribs & ~EP93XXFB_ENABLE,
    EP93XXFB_ATTRIBS);
    clk_disable(fbi.clk);
    } else {
    clk_enable(fbi.clk);
    ep93xxfb_out_locked(fbi, attribs | EP93XXFB_ENABLE,
    EP93XXFB_ATTRIBS);
    if (fbi.mach_info.blank)
    fbi.mach_info.blank(blank_mode, info);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ep93xxfb_convert_color(val: c_int, width: c_int) -> c_int {
    static inline int ep93xxfb_convert_color(int val, int width)
    {
    return ((val << width) + 0x7fff - val) >> 16;
    }
    static int ep93xxfb_setcolreg(unsigned int regno, unsigned int red,
    unsigned int green, unsigned int blue,
    unsigned int transp, struct fb_info *info)
    {
    struct ep93xx_fbi *fbi = info.par;
    unsigned int *pal = info.pseudo_palette;
    unsigned int ctrl, i, rgb, lut_current, lut_stat;
    switch (info.fix.visual) {
    case FB_VISUAL_PSEUDOCOLOR:
    if (regno > 255)
    return 1;
    rgb = ((red & 0xff00) << 8) | (green & 0xff00) |
    ((blue & 0xff00) >> 8);
    pal[regno] = rgb;
    ep93xxfb_writel(fbi, rgb, (EP93XXFB_COLOR_LUT + (regno << 2)));
    ctrl = ep93xxfb_readl(fbi, EP93XXFB_LUT_SW_CONTROL);
    lut_stat = !!(ctrl & EP93XXFB_LUT_SW_CONTROL_SSTAT);
    lut_current = !!(ctrl & EP93XXFB_LUT_SW_CONTROL_SWTCH);
    if (lut_stat == lut_current) {
    for (i = 0; i < 256; i++) {
    ep93xxfb_writel(fbi, pal[i],
    EP93XXFB_COLOR_LUT + (i << 2));
    }
    ep93xxfb_writel(fbi,
    ctrl ^ EP93XXFB_LUT_SW_CONTROL_SWTCH,
    EP93XXFB_LUT_SW_CONTROL);
    }
    break;
    case FB_VISUAL_TRUECOLOR:
    if (regno > 16)
    return 1;
    red = ep93xxfb_convert_color(red, info.var.red.length);
    green = ep93xxfb_convert_color(green, info.var.green.length);
    blue = ep93xxfb_convert_color(blue, info.var.blue.length);
    transp = ep93xxfb_convert_color(transp,
    info.var.transp.length);
    pal[regno] = (red << info.var.red.offset) |
    (green << info.var.green.offset) |
    (blue << info.var.blue.offset) |
    (transp << info.var.transp.offset);
    break;
    default:
    return 1;
    }
    return 0;
    }
    static const struct fb_ops ep93xxfb_ops = {
    .owner		= THIS_MODULE,
    __FB_DEFAULT_IOMEM_OPS_RDWR,
    .fb_check_var	= ep93xxfb_check_var,
    .fb_set_par	= ep93xxfb_set_par,
    .fb_blank	= ep93xxfb_blank,
    __FB_DEFAULT_IOMEM_OPS_DRAW,
    .fb_setcolreg	= ep93xxfb_setcolreg,
    .fb_mmap	= ep93xxfb_mmap,
    };
#[no_mangle]
unsafe extern "C" fn ep93xxfb_alloc_videomem(info: *mut fb_info) -> c_int {
    static int ep93xxfb_alloc_videomem(struct fb_info *info)
    {
    char __iomem *virt_addr;
    dma_addr_t phys_addr;
    unsigned int fb_size;
// Maximum 16bpp -> used memory is maximum x*y*2 bytes
    fb_size = EP93XXFB_MAX_XRES * EP93XXFB_MAX_YRES * 2;
    virt_addr = dma_alloc_wc(info.device, fb_size, &phys_addr, GFP_KERNEL);
    if (!virt_addr)
    return -ENOMEM;
//
// There is a bug in the ep93xx framebuffer which causes problems
// if bit 27 of the physical address is set.
// See: https://marc.info/?l=linux-arm-kernel&m=110061245502000&w=2
// There does not seem to be any official errata for this, but I
// have confirmed the problem exists on my hardware (ep9315) at
// least.
//
    if (check_screenpage_bug && phys_addr & (1 << 27)) {
    fb_err(info, "ep93xx framebuffer bug. phys addr (0x%x) "
    "has bit 27 set: cannot init framebuffer\n",
    phys_addr);
    dma_free_coherent(info.device, fb_size, virt_addr, phys_addr);
    return -ENOMEM;
    }
    info.fix.smem_start = phys_addr;
    info.fix.smem_len = fb_size;
    info.screen_base = virt_addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_dealloc_videomem(info: *mut fb_info) {
    static void ep93xxfb_dealloc_videomem(struct fb_info *info)
    {
    if (info.screen_base)
    dma_free_coherent(info.device, info.fix.smem_len,
    info.screen_base, info.fix.smem_start);
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xxfb_probe(struct platform_device *pdev)
    {
    struct ep93xxfb_mach_info *mach_info = dev_get_platdata(&pdev.dev);
    struct fb_info *info;
    struct ep93xx_fbi *fbi;
    struct resource *res;
    char *video_mode;
    int err;
    if (!mach_info)
    return -EINVAL;
    info = framebuffer_alloc(sizeof(struct ep93xx_fbi), &pdev.dev);
    if (!info)
    return -ENOMEM;
    platform_set_drvdata(pdev, info);
    fbi = info.par;
    fbi.mach_info = mach_info;
    err = fb_alloc_cmap(&info.cmap, 256, 0);
    if (err)
    goto failed_cmap;
    err = ep93xxfb_alloc_videomem(info);
    if (err)
    goto failed_videomem;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    err = -ENXIO;
    goto failed_resource;
    }
//
// FIXME - We don't do a request_mem_region here because we are
// sharing the register space with the backlight driver (see
// drivers/video/backlight/ep93xx_bl.c) and doing so will cause
// the second loaded driver to return -EBUSY.
//
// NOTE: No locking is required; the backlight does not touch
// any of the framebuffer registers.
//
    fbi.res = res;
    fbi.mmio_base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!fbi.mmio_base) {
    err = -ENXIO;
    goto failed_resource;
    }
    strcpy(info.fix.id, pdev.name);
    info.fbops		= &ep93xxfb_ops;
    info.fix.type		= FB_TYPE_PACKED_PIXELS;
    info.fix.accel		= FB_ACCEL_NONE;
    info.var.activate	= FB_ACTIVATE_NOW;
    info.var.vmode		= FB_VMODE_NONINTERLACED;
    info.node		= -1;
    info.state		= FBINFO_STATE_RUNNING;
    info.pseudo_palette	= &fbi.pseudo_palette;
    fb_get_options("ep93xx-fb", &video_mode);
    err = fb_find_mode(&info.var, info, video_mode,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 16);
    if (err == 0) {
    fb_err(info, "No suitable video mode found\n");
    err = -EINVAL;
    goto failed_resource;
    }
    if (mach_info.setup) {
    err = mach_info.setup(pdev);
    if (err)
    goto failed_resource;
    }
    err = ep93xxfb_check_var(&info.var, info);
    if (err)
    goto failed_check;
    fbi.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(fbi.clk)) {
    err = PTR_ERR(fbi.clk);
    fbi.clk = core::ptr::null_mut();
    goto failed_check;
    }
    ep93xxfb_set_par(info);
    err = clk_prepare_enable(fbi.clk);
    if (err)
    goto failed_check;
    err = register_framebuffer(info);
    if (err)
    goto failed_framebuffer;
    fb_info(info, "registered. Mode = %dx%d-%d\n",
    info.var.xres, info.var.yres, info.var.bits_per_pixel);
    return 0;
    failed_framebuffer:
    clk_disable_unprepare(fbi.clk);
    failed_check:
    if (fbi.mach_info.teardown)
    fbi.mach_info.teardown(pdev);
    failed_resource:
    ep93xxfb_dealloc_videomem(info);
    failed_videomem:
    fb_dealloc_cmap(&info.cmap);
    failed_cmap:
    kfree(info);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxfb_remove(pdev: *mut platform_device) {
    static void ep93xxfb_remove(struct platform_device *pdev)
    {
    struct fb_info *info = platform_get_drvdata(pdev);
    struct ep93xx_fbi *fbi = info.par;
    unregister_framebuffer(info);
    clk_disable_unprepare(fbi.clk);
    ep93xxfb_dealloc_videomem(info);
    fb_dealloc_cmap(&info.cmap);
    if (fbi.mach_info.teardown)
    fbi.mach_info.teardown(pdev);
    kfree(info);
    }
    static struct platform_driver ep93xxfb_driver = {
    .probe		= ep93xxfb_probe,
    .remove		= ep93xxfb_remove,
    .driver = {
    .name	= "ep93xx-fb",
    },
    };
    module_platform_driver(ep93xxfb_driver);
    MODULE_DESCRIPTION("EP93XX Framebuffer Driver");
    MODULE_ALIAS("platform:ep93xx-fb");
    MODULE_AUTHOR("Ryan Mallon, "
    "H Hartley Sweeten <hsweeten@visionengravers.com");
    MODULE_LICENSE("GPL");
