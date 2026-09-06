//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/cg6.c
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
// cg6.c: CGSIX (GX, GXplus, TGX) frame buffer driver
//
// Copyright (C) 2003, 2006 David S. Miller (davem@davemloft.net)
// Copyright (C) 1996,1998 Jakub Jelinek (jj@ultra.linux.cz)
// Copyright (C) 1996 Miguel de Icaza (miguel@nuclecu.unam.mx)
// Copyright (C) 1996 Eddie C. Dost (ecd@skynet.be)
//
// Driver layout based loosely on tgafb.c, see that file for credits.
//

//
// Local functions.
//
    static int cg6_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int cg6_blank(int, struct fb_info *);
    static void cg6_imageblit(struct fb_info *, const struct fb_image *);
    static void cg6_fillrect(struct fb_info *, const struct fb_fillrect *);
    static void cg6_copyarea(struct fb_info *info, const struct fb_copyarea *area);
    static int cg6_sync(struct fb_info *);
    static int cg6_pan_display(struct fb_var_screeninfo *, struct fb_info *);
    static int cg6_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int cg6_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops cg6_ops = {
    .owner			= THIS_MODULE,
    __FB_DEFAULT_SBUS_OPS_RDWR(cg6),
    .fb_setcolreg		= cg6_setcolreg,
    .fb_blank		= cg6_blank,
    .fb_pan_display		= cg6_pan_display,
    .fb_fillrect		= cg6_fillrect,
    .fb_copyarea		= cg6_copyarea,
    .fb_imageblit		= cg6_imageblit,
    .fb_sync		= cg6_sync,
    __FB_DEFAULT_SBUS_OPS_IOCTL(cg6),
    __FB_DEFAULT_SBUS_OPS_MMAP(cg6),
    };
// Offset of interesting structures in the OBIO space
//
// Brooktree is the video dac and is funny to program on the cg6.
// (it's even funnier on the cg3)
// The FBC could be the frame buffer control
// The FHC could is the frame buffer hardware control.
//
pub const CG6_ROM_OFFSET: c_uint = 0x0UL;
pub const CG6_BROOKTREE_OFFSET: c_uint = 0x200000UL;
pub const CG6_DHC_OFFSET: c_uint = 0x240000UL;
pub const CG6_ALT_OFFSET: c_uint = 0x280000UL;
pub const CG6_FHC_OFFSET: c_uint = 0x300000UL;
pub const CG6_THC_OFFSET: c_uint = 0x301000UL;
pub const CG6_FBC_OFFSET: c_uint = 0x700000UL;
pub const CG6_TEC_OFFSET: c_uint = 0x701000UL;
pub const CG6_RAM_OFFSET: c_uint = 0x800000UL;
// FHC definitions
pub const CG6_FHC_FBID_SHIFT: c_int = 24;
pub const CG6_FHC_FBID_MASK: c_int = 255;
pub const CG6_FHC_REV_SHIFT: c_int = 20;
pub const CG6_FHC_REV_MASK: c_int = 15;

pub const CG6_FHC_TEST_X_SHIFT: c_int = 4;
pub const CG6_FHC_TEST_X_MASK: c_int = 15;
pub const CG6_FHC_TEST_Y_SHIFT: c_int = 0;
pub const CG6_FHC_TEST_Y_MASK: c_int = 15;
// FBC mode definitions
pub const CG6_FBC_BLIT_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_BLIT_NOSRC: c_uint = 0x00100000;
pub const CG6_FBC_BLIT_SRC: c_uint = 0x00200000;
pub const CG6_FBC_BLIT_ILLEGAL: c_uint = 0x00300000;
pub const CG6_FBC_BLIT_MASK: c_uint = 0x00300000;
pub const CG6_FBC_VBLANK: c_uint = 0x00080000;
pub const CG6_FBC_MODE_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_MODE_COLOR8: c_uint = 0x00020000;
pub const CG6_FBC_MODE_COLOR1: c_uint = 0x00040000;
pub const CG6_FBC_MODE_HRMONO: c_uint = 0x00060000;
pub const CG6_FBC_MODE_MASK: c_uint = 0x00060000;
pub const CG6_FBC_DRAW_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_DRAW_RENDER: c_uint = 0x00008000;
pub const CG6_FBC_DRAW_PICK: c_uint = 0x00010000;
pub const CG6_FBC_DRAW_ILLEGAL: c_uint = 0x00018000;
pub const CG6_FBC_DRAW_MASK: c_uint = 0x00018000;
pub const CG6_FBC_BWRITE0_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_BWRITE0_ENABLE: c_uint = 0x00002000;
pub const CG6_FBC_BWRITE0_DISABLE: c_uint = 0x00004000;
pub const CG6_FBC_BWRITE0_ILLEGAL: c_uint = 0x00006000;
pub const CG6_FBC_BWRITE0_MASK: c_uint = 0x00006000;
pub const CG6_FBC_BWRITE1_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_BWRITE1_ENABLE: c_uint = 0x00000800;
pub const CG6_FBC_BWRITE1_DISABLE: c_uint = 0x00001000;
pub const CG6_FBC_BWRITE1_ILLEGAL: c_uint = 0x00001800;
pub const CG6_FBC_BWRITE1_MASK: c_uint = 0x00001800;
pub const CG6_FBC_BREAD_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_BREAD_0: c_uint = 0x00000200;
pub const CG6_FBC_BREAD_1: c_uint = 0x00000400;
pub const CG6_FBC_BREAD_ILLEGAL: c_uint = 0x00000600;
pub const CG6_FBC_BREAD_MASK: c_uint = 0x00000600;
pub const CG6_FBC_BDISP_IGNORE: c_uint = 0x00000000;
pub const CG6_FBC_BDISP_0: c_uint = 0x00000080;
pub const CG6_FBC_BDISP_1: c_uint = 0x00000100;
pub const CG6_FBC_BDISP_ILLEGAL: c_uint = 0x00000180;
pub const CG6_FBC_BDISP_MASK: c_uint = 0x00000180;
pub const CG6_FBC_INDEX_MOD: c_uint = 0x00000040;
pub const CG6_FBC_INDEX_MASK: c_uint = 0x00000030;
// THC definitions
pub const CG6_THC_MISC_REV_SHIFT: c_int = 16;
pub const CG6_THC_MISC_REV_MASK: c_int = 15;

pub const CG6_THC_MISC_INIT: c_uint = 0x9f;

// The contents are unknown
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg6_tec {
    pub tec_matrix: c_int,
    pub tec_clip: c_int,
    pub tec_vdc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg6_thc {
    pub thc_pad0: [u32; 512],
    pub /: *mut *mut u32 thc_hs; / hsync timing,
    pub thc_hsdvs: u32,
    pub thc_hd: u32,
    pub /: *mut *mut u32 thc_vs; / vsync timing,
    pub thc_vd: u32,
    pub thc_refresh: u32,
    pub thc_misc: u32,
    pub thc_pad1: [u32; 56],
    pub /: *mut *mut u32 thc_cursxy; / cursor x,y position (16 bits each),
    pub /: *mut *mut u32 thc_cursmask[32]; / cursor mask bits,
    pub /: *mut *mut u32 thc_cursbits[32]; / what to show where mask enabled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg6_fbc {
    pub xxx0: [u32; 1],
    pub mode: u32,
    pub clip: u32,
    pub xxx1: [u32; 1],
    pub s: u32,
    pub draw: u32,
    pub blit: u32,
    pub font: u32,
    pub xxx2: [u32; 24],
    pub color0: u32 x0, y0, z0,,
    pub color1: u32 x1, y1, z1,,
    pub color2: u32 x2, y2, z2,,
    pub color3: u32 x3, y3, z3,,
    pub offy: u32 offx,,
    pub xxx3: [u32; 2],
    pub incy: u32 incx,,
    pub xxx4: [u32; 2],
    pub clipminy: u32 clipminx,,
    pub xxx5: [u32; 2],
    pub clipmaxy: u32 clipmaxx,,
    pub xxx6: [u32; 2],
    pub fg: u32,
    pub bg: u32,
    pub alu: u32,
    pub pm: u32,
    pub pixelm: u32,
    pub xxx7: [u32; 2],
    pub patalign: u32,
    pub pattern: [u32; 8],
    pub xxx8: [u32; 432],
    pub apointz: u32 apointx, apointy,,
    pub xxx9: [u32; 1],
    pub rpointz: u32 rpointx, rpointy,,
    pub xxx10: [u32; 5],
    pub pointa: u32 pointr, pointg, pointb,,
    pub alinez: u32 alinex, aliney,,
    pub xxx11: [u32; 1],
    pub rlinez: u32 rlinex, rliney,,
    pub xxx12: [u32; 5],
    pub linea: u32 liner, lineg, lineb,,
    pub atriz: u32 atrix, atriy,,
    pub xxx13: [u32; 1],
    pub rtriz: u32 rtrix, rtriy,,
    pub xxx14: [u32; 5],
    pub tria: u32 trir, trig, trib,,
    pub aquadz: u32 aquadx, aquady,,
    pub xxx15: [u32; 1],
    pub rquadz: u32 rquadx, rquady,,
    pub xxx16: [u32; 5],
    pub quada: u32 quadr, quadg, quadb,,
    pub arectz: u32 arectx, arecty,,
    pub xxx17: [u32; 1],
    pub rrectz: u32 rrectx, rrecty,,
    pub xxx18: [u32; 5],
    pub recta: u32 rectr, rectg, rectb,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_regs {
    pub addr: u32,
    pub color_map: u32,
    pub control: u32,
    pub cursor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg6_par {
    pub lock: spinlock_t,
    pub bt: *mut bt_regs __iomem,
    pub fbc: *mut cg6_fbc __iomem,
    pub thc: *mut cg6_thc __iomem,
    pub tec: *mut cg6_tec __iomem,
    pub fhc: *mut u32 __iomem,
    pub flags: u32,
pub const CG6_FLAG_BLANKED: c_uint = 0x00000001;
    pub which_io: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn cg6_sync(info: *mut fb_info) -> c_int {
    static int cg6_sync(struct fb_info *info)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_fbc __iomem *fbc = par.fbc;
    let mut limit: c_int = 10000;
    do {
    if (!(sbus_readl(&fbc.s) & 0x10000000))
    break;
    udelay(10);
    } while (--limit > 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cg6_switch_from_graph(par: *mut cg6_par) {
    static void cg6_switch_from_graph(struct cg6_par *par)
    {
    struct cg6_thc __iomem *thc = par.thc;
    unsigned long flags;
    spin_lock_irqsave(&par.lock, flags);
// Hide the cursor.
    sbus_writel(CG6_THC_CURSOFF, &thc.thc_cursxy);
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn cg6_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int cg6_pan_display(struct fb_var_screeninfo *var, struct fb_info *info)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
// We just use this to catch switches out of
// graphics mode.
//
    cg6_switch_from_graph(par);
    if (var.xoffset || var.yoffset || var.vmode)
    return -EINVAL;
    return 0;
    }
//
// cg6_fillrect -	Draws a rectangle on the screen.
//
// @info: frame buffer structure that represents a single frame buffer
// @rect: structure defining the rectagle and operation.
//
#[no_mangle]
unsafe extern "C" fn cg6_fillrect(info: *mut fb_info, rect: *const fb_fillrect) {
    static void cg6_fillrect(struct fb_info *info, const struct fb_fillrect *rect)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_fbc __iomem *fbc = par.fbc;
    unsigned long flags;
    s32 val;
// CG6 doesn't handle ROP_XOR
    spin_lock_irqsave(&par.lock, flags);
    cg6_sync(info);
    sbus_writel(rect.color, &fbc.fg);
    sbus_writel(~(u32)0, &fbc.pixelm);
    sbus_writel(0xea80ff00, &fbc.alu);
    sbus_writel(0, &fbc.s);
    sbus_writel(0, &fbc.clip);
    sbus_writel(~(u32)0, &fbc.pm);
    sbus_writel(rect.dy, &fbc.arecty);
    sbus_writel(rect.dx, &fbc.arectx);
    sbus_writel(rect.dy + rect.height, &fbc.arecty);
    sbus_writel(rect.dx + rect.width, &fbc.arectx);
    do {
    val = sbus_readl(&fbc.draw);
    } while (val < 0 && (val & 0x20000000));
    spin_unlock_irqrestore(&par.lock, flags);
    }
//
// cg6_copyarea - Copies one area of the screen to another area.
//
// @info: frame buffer structure that represents a single frame buffer
// @area: Structure providing the data to copy the framebuffer contents
// from one region to another.
//
// This drawing operation copies a rectangular area from one area of the
// screen to another area.
//
#[no_mangle]
unsafe extern "C" fn cg6_copyarea(info: *mut fb_info, area: *const fb_copyarea) {
    static void cg6_copyarea(struct fb_info *info, const struct fb_copyarea *area)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_fbc __iomem *fbc = par.fbc;
    unsigned long flags;
    int i;
    spin_lock_irqsave(&par.lock, flags);
    cg6_sync(info);
    sbus_writel(0xff, &fbc.fg);
    sbus_writel(0x00, &fbc.bg);
    sbus_writel(~0, &fbc.pixelm);
    sbus_writel(0xe880cccc, &fbc.alu);
    sbus_writel(0, &fbc.s);
    sbus_writel(0, &fbc.clip);
    sbus_writel(area.sy, &fbc.y0);
    sbus_writel(area.sx, &fbc.x0);
    sbus_writel(area.sy + area.height - 1, &fbc.y1);
    sbus_writel(area.sx + area.width - 1, &fbc.x1);
    sbus_writel(area.dy, &fbc.y2);
    sbus_writel(area.dx, &fbc.x2);
    sbus_writel(area.dy + area.height - 1, &fbc.y3);
    sbus_writel(area.dx + area.width - 1, &fbc.x3);
    do {
    i = sbus_readl(&fbc.blit);
    } while (i < 0 && (i & 0x20000000));
    spin_unlock_irqrestore(&par.lock, flags);
    }
//
// cg6_imageblit -	Copies a image from system memory to the screen.
//
// @info: frame buffer structure that represents a single frame buffer
// @image: structure defining the image.
//
#[no_mangle]
unsafe extern "C" fn cg6_imageblit(info: *mut fb_info, image: *const fb_image) {
    static void cg6_imageblit(struct fb_info *info, const struct fb_image *image)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_fbc __iomem *fbc = par.fbc;
    const u8 *data = image.data;
    unsigned long flags;
    u32 x, y;
    int i, width;
    if (image.depth > 1) {
    cfb_imageblit(info, image);
    return;
    }
    spin_lock_irqsave(&par.lock, flags);
    cg6_sync(info);
    sbus_writel(image.fg_color, &fbc.fg);
    sbus_writel(image.bg_color, &fbc.bg);
    sbus_writel(0x140000, &fbc.mode);
    sbus_writel(0xe880fc30, &fbc.alu);
    sbus_writel(~(u32)0, &fbc.pixelm);
    sbus_writel(0, &fbc.s);
    sbus_writel(0, &fbc.clip);
    sbus_writel(0xff, &fbc.pm);
    sbus_writel(32, &fbc.incx);
    sbus_writel(0, &fbc.incy);
    x = image.dx;
    y = image.dy;
    for (i = 0; i < image.height; i++) {
    width = image.width;
    while (width >= 32) {
    u32 val;
    sbus_writel(y, &fbc.y0);
    sbus_writel(x, &fbc.x0);
    sbus_writel(x + 32 - 1, &fbc.x1);
    val = ((u32)data[0] << 24) |
    ((u32)data[1] << 16) |
    ((u32)data[2] <<  8) |
    ((u32)data[3] <<  0);
    sbus_writel(val, &fbc.font);
    data += 4;
    x += 32;
    width -= 32;
    }
    if (width) {
    u32 val;
    sbus_writel(y, &fbc.y0);
    sbus_writel(x, &fbc.x0);
    sbus_writel(x + width - 1, &fbc.x1);
    if (width <= 8) {
    val = (u32) data[0] << 24;
    data += 1;
    } else if (width <= 16) {
    val = ((u32) data[0] << 24) |
    ((u32) data[1] << 16);
    data += 2;
    } else {
    val = ((u32) data[0] << 24) |
    ((u32) data[1] << 16) |
    ((u32) data[2] <<  8);
    data += 3;
    }
    sbus_writel(val, &fbc.font);
    }
    y += 1;
    x = image.dx;
    }
    spin_unlock_irqrestore(&par.lock, flags);
    }
//
// cg6_setcolreg - Sets a color register.
//
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
    static int cg6_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct bt_regs __iomem *bt = par.bt;
    unsigned long flags;
    if (regno >= 256)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    spin_lock_irqsave(&par.lock, flags);
    sbus_writel((u32)regno << 24, &bt.addr);
    sbus_writel((u32)red << 24, &bt.color_map);
    sbus_writel((u32)green << 24, &bt.color_map);
    sbus_writel((u32)blue << 24, &bt.color_map);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
//
// cg6_blank - Blanks the display.
//
// @blank: the blank mode we want.
// @info: frame buffer structure that represents a single frame buffer
//
#[no_mangle]
unsafe extern "C" fn cg6_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int cg6_blank(int blank, struct fb_info *info)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_thc __iomem *thc = par.thc;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&par.lock, flags);
    val = sbus_readl(&thc.thc_misc);
    switch (blank) {
    case FB_BLANK_UNBLANK: /* Unblanking */
    val |= CG6_THC_MISC_VIDEO;
    par.flags &= ~CG6_FLAG_BLANKED;
    break;
    case FB_BLANK_NORMAL: /* Normal blanking */
    case FB_BLANK_VSYNC_SUSPEND: /* VESA blank (vsync off) */
    case FB_BLANK_HSYNC_SUSPEND: /* VESA blank (hsync off) */
    case FB_BLANK_POWERDOWN: /* Poweroff */
    val &= ~CG6_THC_MISC_VIDEO;
    par.flags |= CG6_FLAG_BLANKED;
    break;
    }
    sbus_writel(val, &thc.thc_misc);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
    static const struct sbus_mmap_map cg6_mmap_map[] = {
    {
    .voff	= CG6_FBC,
    .poff	= CG6_FBC_OFFSET,
    .size	= PAGE_SIZE
    },
    {
    .voff	= CG6_TEC,
    .poff	= CG6_TEC_OFFSET,
    .size	= PAGE_SIZE
    },
    {
    .voff	= CG6_BTREGS,
    .poff	= CG6_BROOKTREE_OFFSET,
    .size	= PAGE_SIZE
    },
    {
    .voff	= CG6_FHC,
    .poff	= CG6_FHC_OFFSET,
    .size	= PAGE_SIZE
    },
    {
    .voff	= CG6_THC,
    .poff	= CG6_THC_OFFSET,
    .size	= PAGE_SIZE
    },
    {
    .voff	= CG6_ROM,
    .poff	= CG6_ROM_OFFSET,
    .size	= 0x10000
    },
    {
    .voff	= CG6_RAM,
    .poff	= CG6_RAM_OFFSET,
    .size	= SBUS_MMAP_FBSIZE(1)
    },
    {
    .voff	= CG6_DHC,
    .poff	= CG6_DHC_OFFSET,
    .size	= 0x40000
    },
    { .size	= 0 }
    };
#[no_mangle]
unsafe extern "C" fn cg6_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int cg6_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    return sbusfb_mmap_helper(cg6_mmap_map,
    info.fix.smem_start, info.fix.smem_len,
    par.which_io, vma);
    }
#[no_mangle]
unsafe extern "C" fn cg6_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int cg6_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    return sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_SUNFAST_COLOR, 8, info.fix.smem_len);
    }
//
// Initialisation
//
#[no_mangle]
unsafe extern "C" fn cg6_init_fix(info: *mut fb_info, linebytes: c_int) {
    static void cg6_init_fix(struct fb_info *info, int linebytes)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    const char *cg6_cpu_name, *cg6_card_name;
    u32 conf;
    conf = sbus_readl(par.fhc);
    switch (conf & CG6_FHC_CPU_MASK) {
    case CG6_FHC_CPU_SPARC:
    cg6_cpu_name = "sparc";
    break;
    case CG6_FHC_CPU_68020:
    cg6_cpu_name = "68020";
    break;
    default:
    cg6_cpu_name = "i386";
    break;
    }
    if (((conf >> CG6_FHC_REV_SHIFT) & CG6_FHC_REV_MASK) >= 11) {
    if (info.fix.smem_len <= 0x100000)
    cg6_card_name = "TGX";
    else
    cg6_card_name = "TGX+";
    } else {
    if (info.fix.smem_len <= 0x100000)
    cg6_card_name = "GX";
    else
    cg6_card_name = "GX+";
    }
    sprintf(info.fix.id, "%s %s", cg6_card_name, cg6_cpu_name);
    info.fix.id[sizeof(info.fix.id) - 1] = 0;
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.fix.line_length = linebytes;
    info.fix.accel = FB_ACCEL_SUN_CGSIX;
    }
// Initialize Brooktree DAC
#[no_mangle]
unsafe extern "C" fn cg6_bt_init(par: *mut cg6_par) {
    static void cg6_bt_init(struct cg6_par *par)
    {
    struct bt_regs __iomem *bt = par.bt;
    sbus_writel(0x04 << 24, &bt.addr);	 /* color planes */
    sbus_writel(0xff << 24, &bt.control);
    sbus_writel(0x05 << 24, &bt.addr);
    sbus_writel(0x00 << 24, &bt.control);
    sbus_writel(0x06 << 24, &bt.addr);	 /* overlay plane */
    sbus_writel(0x73 << 24, &bt.control);
    sbus_writel(0x07 << 24, &bt.addr);
    sbus_writel(0x00 << 24, &bt.control);
    }
#[no_mangle]
unsafe extern "C" fn cg6_chip_init(info: *mut fb_info) {
    static void cg6_chip_init(struct fb_info *info)
    {
    struct cg6_par *par = (struct cg6_par *)info.par;
    struct cg6_tec __iomem *tec = par.tec;
    struct cg6_fbc __iomem *fbc = par.fbc;
    struct cg6_thc __iomem *thc = par.thc;
    u32 rev, conf, mode;
    int i;
// Hide the cursor.
    sbus_writel(CG6_THC_CURSOFF, &thc.thc_cursxy);
// Turn off stuff in the Transform Engine.
    sbus_writel(0, &tec.tec_matrix);
    sbus_writel(0, &tec.tec_clip);
    sbus_writel(0, &tec.tec_vdc);
// Take care of bugs in old revisions.
    rev = (sbus_readl(par.fhc) >> CG6_FHC_REV_SHIFT) & CG6_FHC_REV_MASK;
    if (rev < 5) {
    conf = (sbus_readl(par.fhc) & CG6_FHC_RES_MASK) |
    CG6_FHC_CPU_68020 | CG6_FHC_TEST |
    (11 << CG6_FHC_TEST_X_SHIFT) |
    (11 << CG6_FHC_TEST_Y_SHIFT);
    if (rev < 2)
    conf |= CG6_FHC_DST_DISABLE;
    sbus_writel(conf, par.fhc);
    }
// Set things in the FBC. Bad things appear to happen if we do
// back to back store/loads on the mode register, so copy it
// out instead.
    mode = sbus_readl(&fbc.mode);
    do {
    i = sbus_readl(&fbc.s);
    } while (i & 0x10000000);
    mode &= ~(CG6_FBC_BLIT_MASK | CG6_FBC_MODE_MASK |
    CG6_FBC_DRAW_MASK | CG6_FBC_BWRITE0_MASK |
    CG6_FBC_BWRITE1_MASK | CG6_FBC_BREAD_MASK |
    CG6_FBC_BDISP_MASK);
    mode |= (CG6_FBC_BLIT_SRC | CG6_FBC_MODE_COLOR8 |
    CG6_FBC_DRAW_RENDER | CG6_FBC_BWRITE0_ENABLE |
    CG6_FBC_BWRITE1_DISABLE | CG6_FBC_BREAD_0 |
    CG6_FBC_BDISP_0);
    sbus_writel(mode, &fbc.mode);
    sbus_writel(0, &fbc.clip);
    sbus_writel(0, &fbc.offx);
    sbus_writel(0, &fbc.offy);
    sbus_writel(0, &fbc.clipminx);
    sbus_writel(0, &fbc.clipminy);
    sbus_writel(info.var.xres - 1, &fbc.clipmaxx);
    sbus_writel(info.var.yres - 1, &fbc.clipmaxy);
    }
    static void cg6_unmap_regs(struct platform_device *op, struct fb_info *info,
    struct cg6_par *par)
    {
    if (par.fbc)
    of_iounmap(&op.resource[0], par.fbc, 4096);
    if (par.tec)
    of_iounmap(&op.resource[0], par.tec, sizeof(struct cg6_tec));
    if (par.thc)
    of_iounmap(&op.resource[0], par.thc, sizeof(struct cg6_thc));
    if (par.bt)
    of_iounmap(&op.resource[0], par.bt, sizeof(struct bt_regs));
    if (par.fhc)
    of_iounmap(&op.resource[0], par.fhc, sizeof(u32));
    if (info.screen_base)
    of_iounmap(&op.resource[0], info.screen_base,
    info.fix.smem_len);
    }
#[no_mangle]
unsafe extern "C" fn cg6_probe(op: *mut platform_device) -> c_int {
    static int cg6_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct fb_info *info;
    struct cg6_par *par;
    int linebytes, err;
    int dblbuf;
    info = framebuffer_alloc(sizeof(struct cg6_par), &op.dev);
    err = -ENOMEM;
    if (!info)
    goto out_err;
    par = info.par;
    spin_lock_init(&par.lock);
    info.fix.smem_start = op.resource[0].start;
    par.which_io = op.resource[0].flags & IORESOURCE_BITS;
    sbusfb_fill_var(&info.var, dp, 8);
    info.var.red.length = 8;
    info.var.green.length = 8;
    info.var.blue.length = 8;
    linebytes = of_getintprop_default(dp, "linebytes",
    info.var.xres);
    info.fix.smem_len = PAGE_ALIGN(linebytes * info.var.yres);
    dblbuf = of_getintprop_default(dp, "dblbuf", 0);
    if (dblbuf)
    info.fix.smem_len *= 4;
    par.fbc = of_ioremap(&op.resource[0], CG6_FBC_OFFSET,
    4096, "cgsix fbc");
    par.tec = of_ioremap(&op.resource[0], CG6_TEC_OFFSET,
    sizeof(struct cg6_tec), "cgsix tec");
    par.thc = of_ioremap(&op.resource[0], CG6_THC_OFFSET,
    sizeof(struct cg6_thc), "cgsix thc");
    par.bt = of_ioremap(&op.resource[0], CG6_BROOKTREE_OFFSET,
    sizeof(struct bt_regs), "cgsix dac");
    par.fhc = of_ioremap(&op.resource[0], CG6_FHC_OFFSET,
    sizeof(u32), "cgsix fhc");
    info.flags = FBINFO_HWACCEL_IMAGEBLIT |
    FBINFO_HWACCEL_COPYAREA | FBINFO_HWACCEL_FILLRECT |
    FBINFO_READS_FAST;
    info.fbops = &cg6_ops;
    info.screen_base = of_ioremap(&op.resource[0], CG6_RAM_OFFSET,
    info.fix.smem_len, "cgsix ram");
    if (!par.fbc || !par.tec || !par.thc ||
    !par.bt || !par.fhc || !info.screen_base)
    goto out_unmap_regs;
    info.var.accel_flags = FB_ACCELF_TEXT;
    cg6_bt_init(par);
    cg6_chip_init(info);
    cg6_blank(FB_BLANK_UNBLANK, info);
    if (fb_alloc_cmap(&info.cmap, 256, 0))
    goto out_unmap_regs;
    fb_set_cmap(&info.cmap, info);
    cg6_init_fix(info, linebytes);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: CGsix [%s] at %lx:%lx\n",
    dp, info.fix.id,
    par.which_io, info.fix.smem_start);
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_regs:
    cg6_unmap_regs(op, info, par);
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cg6_remove(op: *mut platform_device) {
    static void cg6_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct cg6_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    cg6_unmap_regs(op, info, par);
    framebuffer_release(info);
    }
    static const struct of_device_id cg6_match[] = {
    {
    .name = "cgsix",
    },
    {
    .name = "cgthree+",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cg6_match);
    static struct platform_driver cg6_driver = {
    .driver = {
    .name = "cg6",
    .of_match_table = cg6_match,
    },
    .probe		= cg6_probe,
    .remove		= cg6_remove,
    };
#[no_mangle]
unsafe extern "C" fn cg6_init() -> int __init {
    static int __init cg6_init(void)
    {
    if (fb_get_options("cg6fb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&cg6_driver);
    }
#[no_mangle]
unsafe extern "C" fn cg6_exit() -> void __exit {
    static void __exit cg6_exit(void)
    {
    platform_driver_unregister(&cg6_driver);
    }
    module_init(cg6_init);
    module_exit(cg6_exit);
    MODULE_DESCRIPTION("framebuffer driver for CGsix chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
