//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/au1100fb.c
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
// BRIEF MODULE DESCRIPTION
// Au1100 LCD Driver.
//
// Rewritten for 2.6 by Embedded Alley Solutions
// <source@embeddedalley.com>, based on submissions by
// Karl Lessard <klessard@sunrisetelecom.com>
// <c.pellegrin@exadron.com>
//
// PM support added by Rodolfo Giometti <giometti@linux.it>
// Cursor enable/disable by Rodolfo Giometti <giometti@linux.it>
//
// Copyright 2002 MontaVista Software
// Author: MontaVista Software, Inc.
// ppopov@mvista.com or source@mvista.com
//
// Copyright 2002 Alchemy Semiconductor
// Author: Alchemy Semiconductor
//
// Based on:
// linux/drivers/video/skeletonfb.c -- Skeleton for a frame buffer device
// Created 28 Dec 1997 by Geert Uytterhoeven
//

//
// LCD controller restrictions
pub const AU1100_LCD_MAX_XRES: c_int = 800;
pub const AU1100_LCD_MAX_YRES: c_int = 600;
pub const AU1100_LCD_MAX_BPP: c_int = 16;
pub const AU1100_LCD_MAX_CLK: c_int = 48000000;
pub const AU1100_LCD_NBR_PALETTE_ENTRIES: c_int = 256;
// Default number of visible screen buffer to allocate
pub const AU1100FB_NBR_VIDEO_BUFFERS: c_int = 4;
//
    struct au1100fb_panel
    {
    const char name[25];		/* Full name <vendor>_<model> */
    u32   	control_base;		/* Mode-independent control values */
    u32	clkcontrol_base;	/* Panel pixclock preferences */
    u32	horztiming;
    u32	verttiming;
    u32	xres;		/* Maximum horizontal resolution */
    u32 	yres;		/* Maximum vertical resolution */
    u32 	bpp;		/* Maximum depth supported */
    };
    struct au1100fb_regs
    {
    u32  lcd_control;
    u32  lcd_intstatus;
    u32  lcd_intenable;
    u32  lcd_horztiming;
    u32  lcd_verttiming;
    u32  lcd_clkcontrol;
    u32  lcd_dmaaddr0;
    u32  lcd_dmaaddr1;
    u32  lcd_words;
    u32  lcd_pwmdiv;
    u32  lcd_pwmhi;
    u32  reserved[(0x0400-0x002C)/4];
    u32  lcd_palettebase[256];
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct au1100fb_device {
    pub /: *mut *mut fb_info info; / FB driver info record,
    pub /: *mut *mut *mut au1100fb_panel panel; / Panel connected to this device,
    pub /: *mut *mut *mut au1100fb_regs regs; / Registers memory map,
    pub regs_len: usize,
    pub regs_phys: c_uint,

// stores the register values during suspend
    pub pm_regs: au1100fb_regs,

    pub /: *mut *mut *mut unsigned char fb_mem; / FrameBuffer memory map,
    pub fb_len: usize,
    pub fb_phys: dma_addr_t,
    pub panel_idx: c_int,
    pub lcdclk: *mut clk,
    pub dev: *mut device,
}

//

pub const LCD_CONTROL_SBB_BIT: c_int = 21;

pub const LCD_CONTROL_SBPPF_BIT: c_int = 18;

pub const LCD_CONTROL_SM_BIT: c_int = 13;

pub const LCD_CONTROL_PO_BIT: c_int = 8;

pub const LCD_CONTROL_BPP_BIT: c_int = 1;

pub const LCD_HORZTIMING_HN2_BIT: c_int = 24;

pub const LCD_HORZTIMING_HN1_BIT: c_int = 16;

pub const LCD_HORZTIMING_HPW_BIT: c_int = 10;

pub const LCD_HORZTIMING_PPL_BIT: c_int = 0;

pub const LCD_VERTTIMING_VN2_BIT: c_int = 24;

pub const LCD_VERTTIMING_VN1_BIT: c_int = 16;

pub const LCD_VERTTIMING_VPW_BIT: c_int = 10;

pub const LCD_VERTTIMING_LPP_BIT: c_int = 0;

pub const LCD_CLKCONTROL_BF_BIT: c_int = 10;

pub const LCD_CLKCONTROL_PCD_BIT: c_int = 0;

pub const LCD_DMA_SA_BIT: c_int = 5;

pub const LCD_WRD_WRDS_BIT: c_int = 0;

pub const LCD_PWMDIV_PWMDIV_BIT: c_int = 0;

pub const LCD_PWMHI_PWMHI1_BIT: c_int = 12;

pub const LCD_PWMHI_PWMHI0_BIT: c_int = 0;

pub const LCD_PALLETTE_MONO_MI_BIT: c_int = 0;

pub const LCD_PALLETTE_COLOR_RI_BIT: c_int = 8;

pub const LCD_PALLETTE_COLOR_GI_BIT: c_int = 4;

pub const LCD_PALLETTE_COLOR_BI_BIT: c_int = 0;

pub const LCD_PALLETTE_TFT_DC_BIT: c_int = 0;

//
// List of panels known to work with the AU1100 LCD controller.
// To add a new panel, enter the same specifications as the
// Generic_TFT one, and MAKE SURE that it doesn't conflicts
// with the controller restrictions. Restrictions are:
//
// STN color panels: max_bpp <= 12
// STN mono panels: max_bpp <= 4
// TFT panels: max_bpp <= 16
// max_xres <= 800
// max_yres <= 600
//
    static struct au1100fb_panel known_lcd_panels[] =
    {
// 800x600x16bpp CRT
    [0] = {
    .name = "CRT_800x600_16",
    .xres = 800,
    .yres = 600,
    .bpp = 16,
    .control_base =	0x0004886A |
    LCD_CONTROL_DEFAULT_PO | LCD_CONTROL_DEFAULT_SBPPF |
    LCD_CONTROL_BPP_16 | LCD_CONTROL_SBB_4,
    .clkcontrol_base = 0x00020000,
    .horztiming = 0x005aff1f,
    .verttiming = 0x16000e57,
    },
// just the standard LCD
    [1] = {
    .name = "WWPC LCD",
    .xres = 240,
    .yres = 320,
    .bpp = 16,
    .control_base = 0x0006806A,
    .horztiming = 0x0A1010EF,
    .verttiming = 0x0301013F,
    .clkcontrol_base = 0x00018001,
    },
// Sharp 320x240 TFT panel
    [2] = {
    .name = "Sharp_LQ038Q5DR01",
    .xres = 320,
    .yres = 240,
    .bpp = 16,
    .control_base =
    ( LCD_CONTROL_SBPPF_565
    | LCD_CONTROL_C
    | LCD_CONTROL_SM_0
    | LCD_CONTROL_DEFAULT_PO
    | LCD_CONTROL_PT
    | LCD_CONTROL_PC
    | LCD_CONTROL_BPP_16 ),
    .horztiming =
    ( LCD_HORZTIMING_HN2_N(8)
    | LCD_HORZTIMING_HN1_N(60)
    | LCD_HORZTIMING_HPW_N(12)
    | LCD_HORZTIMING_PPL_N(320) ),
    .verttiming =
    ( LCD_VERTTIMING_VN2_N(5)
    | LCD_VERTTIMING_VN1_N(17)
    | LCD_VERTTIMING_VPW_N(1)
    | LCD_VERTTIMING_LPP_N(240) ),
    .clkcontrol_base = LCD_CLKCONTROL_PCD_N(1),
    },
// Hitachi SP14Q005 and possibly others
    [3] = {
    .name = "Hitachi_SP14Qxxx",
    .xres = 320,
    .yres = 240,
    .bpp = 4,
    .control_base =
    ( LCD_CONTROL_C
    | LCD_CONTROL_BPP_4 ),
    .horztiming =
    ( LCD_HORZTIMING_HN2_N(1)
    | LCD_HORZTIMING_HN1_N(1)
    | LCD_HORZTIMING_HPW_N(1)
    | LCD_HORZTIMING_PPL_N(320) ),
    .verttiming =
    ( LCD_VERTTIMING_VN2_N(1)
    | LCD_VERTTIMING_VN1_N(1)
    | LCD_VERTTIMING_VPW_N(1)
    | LCD_VERTTIMING_LPP_N(240) ),
    .clkcontrol_base = LCD_CLKCONTROL_PCD_N(4),
    },
// Generic 640x480 TFT panel
    [4] = {
    .name = "TFT_640x480_16",
    .xres = 640,
    .yres = 480,
    .bpp = 16,
    .control_base = 0x004806a | LCD_CONTROL_DEFAULT_PO,
    .horztiming = 0x3434d67f,
    .verttiming = 0x0e0e39df,
    .clkcontrol_base = LCD_CLKCONTROL_PCD_N(1),
    },
// Pb1100 LCDB 640x480 PrimeView TFT panel
    [5] = {
    .name = "PrimeView_640x480_16",
    .xres = 640,
    .yres = 480,
    .bpp = 16,
    .control_base = 0x0004886a | LCD_CONTROL_DEFAULT_PO,
    .horztiming = 0x0e4bfe7f,
    .verttiming = 0x210805df,
    .clkcontrol_base = 0x00038001,
    },
    };
//
// Inline helpers

//
// KSEG1ADDR() is defined in arch/mips/include/asm/addrspace.h
// for 32 bit configurations. Provide a stub for compile testing
// on other platforms.
//

    (_info ? container_of(_info, struct au1100fb_device, info) : core::ptr::null_mut())
// Bitfields format supported by the controller. Note that the order of formats
// SHOULD be the same as in the LCD_CONTROL_SBPPF field, so we can retrieve the
// right pixel format by doing rgb_bitfields[LCD_CONTROL_SBPPF_XXX >> LCD_CONTROL_SBPPF]
//
    struct fb_bitfield rgb_bitfields[][4] =
    {
// Red, 	   Green, 	 Blue, 	     Transp
    { { 10, 6, 0 }, { 5, 5, 0 }, { 0, 5, 0 }, { 0, 0, 0 } },
    { { 11, 5, 0 }, { 5, 6, 0 }, { 0, 5, 0 }, { 0, 0, 0 } },
    { { 11, 5, 0 }, { 6, 5, 0 }, { 0, 6, 0 }, { 0, 0, 0 } },
    { { 10, 5, 0 }, { 5, 5, 0 }, { 0, 5, 0 }, { 15, 1, 0 } },
    { { 11, 5, 0 }, { 6, 5, 0 }, { 1, 5, 0 }, { 0, 1, 0 } },
// The last is used to describe 12bpp format
    { { 8, 4, 0 },  { 4, 4, 0 }, { 0, 4, 0 }, { 0, 0, 0 } },
    };
// fb_blank
// Blank the screen. Depending on the mode, the screen will be
// activated with the backlight color, or desactivated
//
#[no_mangle]
unsafe extern "C" fn au1100fb_fb_blank(blank_mode: c_int, fbi: *mut fb_info) -> c_int {
    static int au1100fb_fb_blank(int blank_mode, struct fb_info *fbi)
    {
    struct au1100fb_device *fbdev = to_au1100fb_device(fbi);
    pr_devel("fb_blank %d %p", blank_mode, fbi);
    switch (blank_mode) {
    case VESA_NO_BLANKING:
// Turn on panel
    fbdev.regs.lcd_control |= LCD_CONTROL_GO;
    wmb(); /* drain writebuffer */
    break;
    case VESA_VSYNC_SUSPEND:
    case VESA_HSYNC_SUSPEND:
    case VESA_POWERDOWN:
// Turn off panel
    fbdev.regs.lcd_control &= ~LCD_CONTROL_GO;
    wmb(); /* drain writebuffer */
    break;
    default:
    break;
    }
    return 0;
    }
//
// Set hardware with var settings. This will enable the controller with a specific
// mode, normally validated with the fb_check_var method
//
#[no_mangle]
unsafe extern "C" fn au1100fb_setmode(fbdev: *mut au1100fb_device) -> c_int {
    static int au1100fb_setmode(struct au1100fb_device *fbdev)
    {
    struct fb_info *info;
    u32 words;
    int index;
    if (!fbdev)
    return -EINVAL;
    info = &fbdev.info;
// Update var-dependent FB info
    if (panel_is_active(fbdev.panel) || panel_is_color(fbdev.panel)) {
    if (info.var.bits_per_pixel <= 8) {
// palettized
    info.var.red.offset    = 0;
    info.var.red.length    = info.var.bits_per_pixel;
    info.var.red.msb_right = 0;
    info.var.green.offset  = 0;
    info.var.green.length  = info.var.bits_per_pixel;
    info.var.green.msb_right = 0;
    info.var.blue.offset   = 0;
    info.var.blue.length   = info.var.bits_per_pixel;
    info.var.blue.msb_right = 0;
    info.var.transp.offset = 0;
    info.var.transp.length = 0;
    info.var.transp.msb_right = 0;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.fix.line_length = info.var.xres_virtual /
    (8/info.var.bits_per_pixel);
    } else {
// non-palettized
    index = (fbdev.panel.control_base & LCD_CONTROL_SBPPF_MASK) >> LCD_CONTROL_SBPPF_BIT;
    info.var.red = rgb_bitfields[index][0];
    info.var.green = rgb_bitfields[index][1];
    info.var.blue = rgb_bitfields[index][2];
    info.var.transp = rgb_bitfields[index][3];
    info.fix.visual = FB_VISUAL_TRUECOLOR;
    info.fix.line_length = info.var.xres_virtual << 1; /* depth=16 */
    }
    } else {
// mono
    info.fix.visual = FB_VISUAL_MONO10;
    info.fix.line_length = info.var.xres_virtual / 8;
    }
    info.screen_size = info.fix.line_length * info.var.yres_virtual;
    info.var.rotate = ((fbdev.panel.control_base&LCD_CONTROL_SM_MASK) \
    >> LCD_CONTROL_SM_BIT) * 90;
// Determine BPP mode and format
    fbdev.regs.lcd_control = fbdev.panel.control_base;
    fbdev.regs.lcd_horztiming = fbdev.panel.horztiming;
    fbdev.regs.lcd_verttiming = fbdev.panel.verttiming;
    fbdev.regs.lcd_clkcontrol = fbdev.panel.clkcontrol_base;
    fbdev.regs.lcd_intenable = 0;
    fbdev.regs.lcd_intstatus = 0;
    fbdev.regs.lcd_dmaaddr0 = LCD_DMA_SA_N(fbdev.fb_phys);
    if (panel_is_dual(fbdev.panel)) {
// Second panel display seconf half of screen if possible,
// otherwise display the same as the first panel
    if (info.var.yres_virtual >= (info.var.yres << 1)) {
    fbdev.regs.lcd_dmaaddr1 = LCD_DMA_SA_N(fbdev.fb_phys +
    (info.fix.line_length *
    (info.var.yres_virtual >> 1)));
    } else {
    fbdev.regs.lcd_dmaaddr1 = LCD_DMA_SA_N(fbdev.fb_phys);
    }
    }
    words = info.fix.line_length / sizeof(u32);
    if (!info.var.rotate || (info.var.rotate == 180)) {
    words *= info.var.yres_virtual;
    if (info.var.rotate /* 180 */) {
    words -= (words % 8); /* should be divisable by 8 */
    }
    }
    fbdev.regs.lcd_words = LCD_WRD_WRDS_N(words);
    fbdev.regs.lcd_pwmdiv = 0;
    fbdev.regs.lcd_pwmhi = 0;
// Resume controller
    fbdev.regs.lcd_control |= LCD_CONTROL_GO;
    mdelay(10);
    au1100fb_fb_blank(VESA_NO_BLANKING, info);
    return 0;
    }
// fb_setcolreg
// Set color in LCD palette.
//
    static int au1100fb_fb_setcolreg(unsigned regno, unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *fbi)
    {
    struct au1100fb_device *fbdev;
    u32 *palette;
    u32 value;
    fbdev = to_au1100fb_device(fbi);
    palette = fbdev.regs.lcd_palettebase;
    if (regno > (AU1100_LCD_NBR_PALETTE_ENTRIES - 1))
    return -EINVAL;
    if (fbi.var.grayscale) {
// Convert color to grayscale
    red = green = blue =
    (19595 * red + 38470 * green + 7471 * blue) >> 16;
    }
    if (fbi.fix.visual == FB_VISUAL_TRUECOLOR) {
// Place color in the pseudopalette
    if (regno > 16)
    return -EINVAL;
    palette = (u32*)fbi.pseudo_palette;
    red   >>= (16 - fbi.var.red.length);
    green >>= (16 - fbi.var.green.length);
    blue  >>= (16 - fbi.var.blue.length);
    value = (red   << fbi.var.red.offset) 	|
    (green << fbi.var.green.offset)|
    (blue  << fbi.var.blue.offset);
    value &= 0xFFFF;
    } else if (panel_is_active(fbdev.panel)) {
// COLOR TFT PALLETTIZED (use RGB 565)
    value = (red & 0xF800)|((green >> 5) & 0x07E0)|((blue >> 11) & 0x001F);
    value &= 0xFFFF;
    } else if (panel_is_color(fbdev.panel)) {
// COLOR STN MODE
    value = (((panel_swap_rgb(fbdev.panel) ? blue : red) >> 12) & 0x000F) |
    ((green >> 8) & 0x00F0) |
    (((panel_swap_rgb(fbdev.panel) ? red : blue) >> 4) & 0x0F00);
    value &= 0xFFF;
    } else {
// MONOCHROME MODE
    value = (green >> 12) & 0x000F;
    value &= 0xF;
    }
    palette[regno] = value;
    return 0;
    }
// fb_pan_display
// Pan display in x and/or y as specified
//
#[no_mangle]
unsafe extern "C" fn au1100fb_fb_pan_display(var: *mut fb_var_screeninfo, fbi: *mut fb_info) -> c_int {
    static int au1100fb_fb_pan_display(struct fb_var_screeninfo *var, struct fb_info *fbi)
    {
    struct au1100fb_device *fbdev;
    int dy;
    fbdev = to_au1100fb_device(fbi);
    pr_devel("fb_pan_display %p %p", var, fbi);
    if (!var || !fbdev) {
    return -EINVAL;
    }
    if (var.xoffset - fbi.var.xoffset) {
// No support for X panning for now!
    return -EINVAL;
    }
    pr_devel("fb_pan_display 2 %p %p", var, fbi);
    dy = var.yoffset - fbi.var.yoffset;
    if (dy) {
    u32 dmaaddr;
    pr_devel("Panning screen of %d lines", dy);
    dmaaddr = fbdev.regs.lcd_dmaaddr0;
    dmaaddr += (fbi.fix.line_length * dy);
// TODO: Wait for current frame to finished
    fbdev.regs.lcd_dmaaddr0 = LCD_DMA_SA_N(dmaaddr);
    if (panel_is_dual(fbdev.panel)) {
    dmaaddr = fbdev.regs.lcd_dmaaddr1;
    dmaaddr += (fbi.fix.line_length * dy);
    fbdev.regs.lcd_dmaaddr0 = LCD_DMA_SA_N(dmaaddr);
    }
    }
    pr_devel("fb_pan_display 3 %p %p", var, fbi);
    return 0;
    }
// fb_mmap
// Map video memory in user space. We don't use the generic fb_mmap method mainly
// to allow the use of the TLB streaming flag (CCA=6)
//
#[no_mangle]
unsafe extern "C" fn au1100fb_fb_mmap(fbi: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int au1100fb_fb_mmap(struct fb_info *fbi, struct vm_area_struct *vma)
    {
    struct au1100fb_device *fbdev = to_au1100fb_device(fbi);
    vma.vm_page_prot = pgprot_decrypted(vma.vm_page_prot);

// On s390 pgprot_val() is a function and thus not a lvalue
    pgprot_val(vma.vm_page_prot) |= (6 << 9); //CCA=6

    return dma_mmap_coherent(fbdev.dev, vma, fbdev.fb_mem, fbdev.fb_phys,
    fbdev.fb_len);
    }
    static const struct fb_ops au1100fb_ops = {
    .owner			= THIS_MODULE,
    __FB_DEFAULT_IOMEM_OPS_RDWR,
    .fb_setcolreg		= au1100fb_fb_setcolreg,
    .fb_blank		= au1100fb_fb_blank,
    .fb_pan_display		= au1100fb_fb_pan_display,
    __FB_DEFAULT_IOMEM_OPS_DRAW,
    .fb_mmap		= au1100fb_fb_mmap,
    };
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn au1100fb_setup(fbdev: *mut au1100fb_device) -> c_int {
    static int au1100fb_setup(struct au1100fb_device *fbdev)
    {
    char *this_opt, *options;
    let mut num_panels: c_int = ARRAY_SIZE(known_lcd_panels);
    if (num_panels <= 0) {
    pr_err("No LCD panels supported by driver!");
    return -ENODEV;
    }
    if (fb_get_options(DRIVER_NAME, &options))
    return -ENODEV;
    if (!options)
    return -ENODEV;
    while ((this_opt = strsep(&options, ",")) != core::ptr::null_mut()) {
// Panel option
    if (!strncmp(this_opt, "panel:", 6)) {
    int i;
    this_opt += 6;
    for (i = 0; i < num_panels; i++) {
    if (!strncmp(this_opt, known_lcd_panels[i].name,
    strlen(this_opt))) {
    fbdev.panel = &known_lcd_panels[i];
    fbdev.panel_idx = i;
    break;
    }
    }
    if (i >= num_panels) {
    pr_warn("Panel '%s' not supported!", this_opt);
    return -ENODEV;
    }
    }
// Unsupported option
    else
    pr_warn("Unsupported option \"%s\"", this_opt);
    }
    pr_info("Panel=%s", fbdev.panel.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1100fb_drv_probe(dev: *mut platform_device) -> c_int {
    static int au1100fb_drv_probe(struct platform_device *dev)
    {
    struct au1100fb_device *fbdev;
    struct resource *regs_res;
    struct clk *c;
// Allocate new device private
    fbdev = devm_kzalloc(&dev.dev, sizeof(*fbdev), GFP_KERNEL);
    if (!fbdev)
    return -ENOMEM;
    if (au1100fb_setup(fbdev))
    goto failed;
    platform_set_drvdata(dev, (void *)fbdev);
    fbdev.dev = &dev.dev;
// Allocate region for our registers and map them
    regs_res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (!regs_res) {
    pr_err("fail to retrieve registers resource");
    return -EFAULT;
    }
    fbdev.info.fix = (struct fb_fix_screeninfo) {
    .mmio_start = regs_res.start,
    .mmio_len = resource_size(regs_res),
    .id = "AU1100 FB",
    .xpanstep = 1,
    .ypanstep = 1,
    .type = FB_TYPE_PACKED_PIXELS,
    .accel = FB_ACCEL_NONE,
    };
    if (!devm_request_mem_region(&dev.dev,
    fbdev.info.fix.mmio_start,
    fbdev.info.fix.mmio_len,
    DRIVER_NAME)) {
    pr_err("fail to lock memory region at 0x%08lx",
    fbdev.info.fix.mmio_start);
    return -EBUSY;
    }
    fbdev.regs = (struct au1100fb_regs*)KSEG1ADDR(fbdev.info.fix.mmio_start);
    pr_devel("Register memory map at %p", fbdev.regs);
    pr_devel("phys=0x%08x, size=%zu", fbdev.regs_phys, fbdev.regs_len);
    c = clk_get(core::ptr::null_mut(), "lcd_intclk");
    if (!IS_ERR(c)) {
    fbdev.lcdclk = c;
    clk_set_rate(c, 48000000);
    clk_prepare_enable(c);
    }
// Allocate the framebuffer to the maximum screen size * nbr of video buffers
    fbdev.fb_len = fbdev.panel.xres * fbdev.panel.yres *
    (fbdev.panel.bpp >> 3) * AU1100FB_NBR_VIDEO_BUFFERS;
    fbdev.fb_mem = dmam_alloc_coherent(&dev.dev,
    PAGE_ALIGN(fbdev.fb_len),
    &fbdev.fb_phys, GFP_KERNEL);
    if (!fbdev.fb_mem) {
    pr_err("fail to allocate framebuffer (size: %zuK))",
    fbdev.fb_len / 1024);
    return -ENOMEM;
    }
    fbdev.info.fix.smem_start = fbdev.fb_phys;
    fbdev.info.fix.smem_len = fbdev.fb_len;
    pr_devel("Framebuffer memory map at %p", fbdev.fb_mem);
    pr_devel("phys=0x%pad, size=%zuK", &fbdev.fb_phys, fbdev.fb_len / 1024);
// load the panel info into the var struct
    fbdev.info.var = (struct fb_var_screeninfo) {
    .activate = FB_ACTIVATE_NOW,
    .height = -1,
    .width = -1,
    .vmode = FB_VMODE_NONINTERLACED,
    .bits_per_pixel = fbdev.panel.bpp,
    .xres = fbdev.panel.xres,
    .xres_virtual = fbdev.panel.xres,
    .yres = fbdev.panel.yres,
    .yres_virtual = fbdev.panel.yres,
    };
    fbdev.info.screen_base = fbdev.fb_mem;
    fbdev.info.fbops = &au1100fb_ops;
    fbdev.info.pseudo_palette =
    devm_kcalloc(&dev.dev, 16, sizeof(u32), GFP_KERNEL);
    if (!fbdev.info.pseudo_palette)
    return -ENOMEM;
    if (fb_alloc_cmap(&fbdev.info.cmap, AU1100_LCD_NBR_PALETTE_ENTRIES, 0) < 0) {
    pr_err("Fail to allocate colormap (%d entries)",
    AU1100_LCD_NBR_PALETTE_ENTRIES);
    return -EFAULT;
    }
// Set h/w registers
    au1100fb_setmode(fbdev);
// Register new framebuffer
    if (register_framebuffer(&fbdev.info) < 0) {
    pr_err("cannot register new framebuffer");
    goto failed;
    }
    return 0;
    failed:
    if (fbdev.lcdclk) {
    clk_disable_unprepare(fbdev.lcdclk);
    clk_put(fbdev.lcdclk);
    }
    if (fbdev.info.cmap.len != 0) {
    fb_dealloc_cmap(&fbdev.info.cmap);
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn au1100fb_drv_remove(dev: *mut platform_device) {
    static void au1100fb_drv_remove(struct platform_device *dev)
    {
    struct au1100fb_device *fbdev = core::ptr::null_mut();
    fbdev = platform_get_drvdata(dev);

    au1100fb_fb_blank(VESA_POWERDOWN, &fbdev.info);

    fbdev.regs.lcd_control &= ~LCD_CONTROL_GO;
// Clean up all probe data
    unregister_framebuffer(&fbdev.info);
    fb_dealloc_cmap(&fbdev.info.cmap);
    if (fbdev.lcdclk) {
    clk_disable_unprepare(fbdev.lcdclk);
    clk_put(fbdev.lcdclk);
    }
    }

#[no_mangle]
unsafe extern "C" fn au1100fb_drv_suspend(dev: *mut platform_device, state: pm_message_t) -> c_int {
    static int au1100fb_drv_suspend(struct platform_device *dev, pm_message_t state)
    {
    struct au1100fb_device *fbdev = platform_get_drvdata(dev);
    if (!fbdev)
    return 0;
// Blank the LCD
    au1100fb_fb_blank(VESA_POWERDOWN, &fbdev.info);
    clk_disable(fbdev.lcdclk);
    memcpy(&fbdev.pm_regs, fbdev.regs, sizeof(struct au1100fb_regs));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1100fb_drv_resume(dev: *mut platform_device) -> c_int {
    static int au1100fb_drv_resume(struct platform_device *dev)
    {
    struct au1100fb_device *fbdev = platform_get_drvdata(dev);
    int ret;
    if (!fbdev)
    return 0;
    memcpy(fbdev.regs, &fbdev.pm_regs, sizeof(struct au1100fb_regs));
    ret = clk_enable(fbdev.lcdclk);
    if (ret)
    return ret;
// Unblank the LCD
    au1100fb_fb_blank(VESA_NO_BLANKING, &fbdev.info);
    return 0;
    }

    static struct platform_driver au1100fb_driver = {
    .driver = {
    .name		= "au1100-lcd",
    },
    .probe		= au1100fb_drv_probe,
    .remove		= au1100fb_drv_remove,
    .suspend	= au1100fb_drv_suspend,
    .resume		= au1100fb_drv_resume,
    };
    module_platform_driver(au1100fb_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
