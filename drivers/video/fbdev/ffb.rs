//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/ffb.c
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
// ffb.c: Creator/Elite3D frame buffer driver
//
// Copyright (C) 2003, 2006 David S. Miller (davem@davemloft.net)
// Copyright (C) 1997,1998,1999 Jakub Jelinek (jj@ultra.linux.cz)
//
// Driver layout based loosely on tgafb.c, see that file for credits.
//

//
// Local functions.
//
    static int ffb_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int ffb_blank(int, struct fb_info *);
    static void ffb_imageblit(struct fb_info *, const struct fb_image *);
    static void ffb_fillrect(struct fb_info *, const struct fb_fillrect *);
    static void ffb_copyarea(struct fb_info *, const struct fb_copyarea *);
    static int ffb_sync(struct fb_info *);
    static int ffb_pan_display(struct fb_var_screeninfo *, struct fb_info *);
    static int ffb_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int ffb_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops ffb_ops = {
    .owner			= THIS_MODULE,
    __FB_DEFAULT_SBUS_OPS_RDWR(ffb),
    .fb_setcolreg		= ffb_setcolreg,
    .fb_blank		= ffb_blank,
    .fb_pan_display		= ffb_pan_display,
    .fb_fillrect		= ffb_fillrect,
    .fb_copyarea		= ffb_copyarea,
    .fb_imageblit		= ffb_imageblit,
    .fb_sync		= ffb_sync,
    __FB_DEFAULT_SBUS_OPS_IOCTL(ffb),
    __FB_DEFAULT_SBUS_OPS_MMAP(ffb),
    };
// Register layout and definitions
pub const FFB_SFB8R_VOFF: c_uint = 0x00000000;
pub const FFB_SFB8G_VOFF: c_uint = 0x00400000;
pub const FFB_SFB8B_VOFF: c_uint = 0x00800000;
pub const FFB_SFB8X_VOFF: c_uint = 0x00c00000;
pub const FFB_SFB32_VOFF: c_uint = 0x01000000;
pub const FFB_SFB64_VOFF: c_uint = 0x02000000;
pub const FFB_FBC_REGS_VOFF: c_uint = 0x04000000;
pub const FFB_BM_FBC_REGS_VOFF: c_uint = 0x04002000;
pub const FFB_DFB8R_VOFF: c_uint = 0x04004000;
pub const FFB_DFB8G_VOFF: c_uint = 0x04404000;
pub const FFB_DFB8B_VOFF: c_uint = 0x04804000;
pub const FFB_DFB8X_VOFF: c_uint = 0x04c04000;
pub const FFB_DFB24_VOFF: c_uint = 0x05004000;
pub const FFB_DFB32_VOFF: c_uint = 0x06004000;
pub const FFB_DFB422A_VOFF: c_uint = 0x07004000	/* DFB 422 mode write to A */;
pub const FFB_DFB422AD_VOFF: c_uint = 0x07804000	/* DFB 422 mode with line doubling */;
pub const FFB_DFB24B_VOFF: c_uint = 0x08004000	/* DFB 24bit mode write to B */;
pub const FFB_DFB422B_VOFF: c_uint = 0x09004000	/* DFB 422 mode write to B */;
pub const FFB_DFB422BD_VOFF: c_uint = 0x09804000	/* DFB 422 mode with line doubling */;
pub const FFB_SFB16Z_VOFF: c_uint = 0x0a004000	/* 16bit mode Z planes */;
pub const FFB_SFB8Z_VOFF: c_uint = 0x0a404000	/* 8bit mode Z planes */;
pub const FFB_SFB422_VOFF: c_uint = 0x0ac04000	/* SFB 422 mode write to A/B */;
pub const FFB_SFB422D_VOFF: c_uint = 0x0b404000	/* SFB 422 mode with line doubling */;
pub const FFB_FBC_KREGS_VOFF: c_uint = 0x0bc04000;
pub const FFB_DAC_VOFF: c_uint = 0x0bc06000;
pub const FFB_PROM_VOFF: c_uint = 0x0bc08000;
pub const FFB_EXP_VOFF: c_uint = 0x0bc18000;
pub const FFB_SFB8R_POFF: c_uint = 0x04000000UL;
pub const FFB_SFB8G_POFF: c_uint = 0x04400000UL;
pub const FFB_SFB8B_POFF: c_uint = 0x04800000UL;
pub const FFB_SFB8X_POFF: c_uint = 0x04c00000UL;
pub const FFB_SFB32_POFF: c_uint = 0x05000000UL;
pub const FFB_SFB64_POFF: c_uint = 0x06000000UL;
pub const FFB_FBC_REGS_POFF: c_uint = 0x00600000UL;
pub const FFB_BM_FBC_REGS_POFF: c_uint = 0x00600000UL;
pub const FFB_DFB8R_POFF: c_uint = 0x01000000UL;
pub const FFB_DFB8G_POFF: c_uint = 0x01400000UL;
pub const FFB_DFB8B_POFF: c_uint = 0x01800000UL;
pub const FFB_DFB8X_POFF: c_uint = 0x01c00000UL;
pub const FFB_DFB24_POFF: c_uint = 0x02000000UL;
pub const FFB_DFB32_POFF: c_uint = 0x03000000UL;
pub const FFB_FBC_KREGS_POFF: c_uint = 0x00610000UL;
pub const FFB_DAC_POFF: c_uint = 0x00400000UL;
pub const FFB_PROM_POFF: c_uint = 0x00000000UL;
pub const FFB_EXP_POFF: c_uint = 0x00200000UL;
pub const FFB_DFB422A_POFF: c_uint = 0x09000000UL;
pub const FFB_DFB422AD_POFF: c_uint = 0x09800000UL;
pub const FFB_DFB24B_POFF: c_uint = 0x0a000000UL;
pub const FFB_DFB422B_POFF: c_uint = 0x0b000000UL;
pub const FFB_DFB422BD_POFF: c_uint = 0x0b800000UL;
pub const FFB_SFB16Z_POFF: c_uint = 0x0c800000UL;
pub const FFB_SFB8Z_POFF: c_uint = 0x0c000000UL;
pub const FFB_SFB422_POFF: c_uint = 0x0d000000UL;
pub const FFB_SFB422D_POFF: c_uint = 0x0d800000UL;
// Draw operations
pub const FFB_DRAWOP_DOT: c_uint = 0x00;
pub const FFB_DRAWOP_AADOT: c_uint = 0x01;
pub const FFB_DRAWOP_BRLINECAP: c_uint = 0x02;
pub const FFB_DRAWOP_BRLINEOPEN: c_uint = 0x03;
pub const FFB_DRAWOP_DDLINE: c_uint = 0x04;
pub const FFB_DRAWOP_AALINE: c_uint = 0x05;
pub const FFB_DRAWOP_TRIANGLE: c_uint = 0x06;
pub const FFB_DRAWOP_POLYGON: c_uint = 0x07;
pub const FFB_DRAWOP_RECTANGLE: c_uint = 0x08;
pub const FFB_DRAWOP_FASTFILL: c_uint = 0x09;
pub const FFB_DRAWOP_BCOPY: c_uint = 0x0a;
pub const FFB_DRAWOP_VSCROLL: c_uint = 0x0b;
// Pixel processor control
// Force WID
pub const FFB_PPC_FW_DISABLE: c_uint = 0x800000;
pub const FFB_PPC_FW_ENABLE: c_uint = 0xc00000;
// Auxiliary clip
pub const FFB_PPC_ACE_DISABLE: c_uint = 0x040000;
pub const FFB_PPC_ACE_AUX_SUB: c_uint = 0x080000;
pub const FFB_PPC_ACE_AUX_ADD: c_uint = 0x0c0000;
// Depth cue
pub const FFB_PPC_DCE_DISABLE: c_uint = 0x020000;
pub const FFB_PPC_DCE_ENABLE: c_uint = 0x030000;
// Alpha blend
pub const FFB_PPC_ABE_DISABLE: c_uint = 0x008000;
pub const FFB_PPC_ABE_ENABLE: c_uint = 0x00c000;
// View clip
pub const FFB_PPC_VCE_DISABLE: c_uint = 0x001000;
pub const FFB_PPC_VCE_2D: c_uint = 0x002000;
pub const FFB_PPC_VCE_3D: c_uint = 0x003000;
// Area pattern
pub const FFB_PPC_APE_DISABLE: c_uint = 0x000800;
pub const FFB_PPC_APE_ENABLE: c_uint = 0x000c00;
// Transparent background
pub const FFB_PPC_TBE_OPAQUE: c_uint = 0x000200;
pub const FFB_PPC_TBE_TRANSPARENT: c_uint = 0x000300;
// Z source
pub const FFB_PPC_ZS_VAR: c_uint = 0x000080;
pub const FFB_PPC_ZS_CONST: c_uint = 0x0000c0;
// Y source
pub const FFB_PPC_YS_VAR: c_uint = 0x000020;
pub const FFB_PPC_YS_CONST: c_uint = 0x000030;
// X source
pub const FFB_PPC_XS_WID: c_uint = 0x000004;
pub const FFB_PPC_XS_VAR: c_uint = 0x000008;
pub const FFB_PPC_XS_CONST: c_uint = 0x00000c;
// Color (BGR) source
pub const FFB_PPC_CS_VAR: c_uint = 0x000002;
pub const FFB_PPC_CS_CONST: c_uint = 0x000003;
pub const FFB_ROP_NEW: c_uint = 0x83;
pub const FFB_ROP_OLD: c_uint = 0x85;
pub const FFB_ROP_NEW_XOR_OLD: c_uint = 0x86;
pub const FFB_UCSR_FIFO_MASK: c_uint = 0x00000fff;
pub const FFB_UCSR_FB_BUSY: c_uint = 0x01000000;
pub const FFB_UCSR_RP_BUSY: c_uint = 0x02000000;

pub const FFB_UCSR_READ_ERR: c_uint = 0x40000000;
pub const FFB_UCSR_FIFO_OVFL: c_uint = 0x80000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffb_fbc {
// Next vertex registers
    pub xxx1: [u32; 3],
    pub alpha: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub depth: u32,
    pub y: u32,
    pub x: u32,
    pub xxx2: [u32; 2],
    pub ryf: u32,
    pub rxf: u32,
    pub xxx3: [u32; 2],
    pub dmyf: u32,
    pub dmxf: u32,
    pub xxx4: [u32; 2],
    pub ebyi: u32,
    pub ebxi: u32,
    pub xxx5: [u32; 2],
    pub by: u32,
    pub bx: u32,
    pub dy: u32,
    pub dx: u32,
    pub bh: u32,
    pub bw: u32,
    pub xxx6: [u32; 2],
    pub xxx7: [u32; 32],
// Setup unit vertex state register
    pub suvtx: u32,
    pub xxx8: [u32; 63],
// Control registers
    pub ppc: u32,
    pub wid: u32,
    pub fg: u32,
    pub bg: u32,
    pub consty: u32,
    pub constz: u32,
    pub xclip: u32,
    pub dcss: u32,
    pub vclipmin: u32,
    pub vclipmax: u32,
    pub vclipzmin: u32,
    pub vclipzmax: u32,
    pub dcsf: u32,
    pub dcsb: u32,
    pub dczf: u32,
    pub dczb: u32,
    pub xxx9: u32,
    pub blendc: u32,
    pub blendc1: u32,
    pub blendc2: u32,
    pub fbramitc: u32,
    pub fbc: u32,
    pub rop: u32,
    pub cmp: u32,
    pub matchab: u32,
    pub matchc: u32,
    pub magnab: u32,
    pub magnc: u32,
    pub fbcfg0: u32,
    pub fbcfg1: u32,
    pub fbcfg2: u32,
    pub fbcfg3: u32,
    pub ppcfg: u32,
    pub pick: u32,
    pub fillmode: u32,
    pub fbramwac: u32,
    pub pmask: u32,
    pub xpmask: u32,
    pub ypmask: u32,
    pub zpmask: u32,
    pub clip0min: u32,
    pub clip0max: u32,
    pub clip1min: u32,
    pub clip1max: u32,
    pub clip2min: u32,
    pub clip2max: u32,
    pub clip3min: u32,
    pub clip3max: u32,
// New 3dRAM III support regs
    pub rawblend2: u32,
    pub rawpreblend: u32,
    pub rawstencil: u32,
    pub rawstencilctl: u32,
    pub threedram1: u32,
    pub threedram2: u32,
    pub passin: u32,
    pub rawclrdepth: u32,
    pub rawpmask: u32,
    pub rawcsrc: u32,
    pub rawmatch: u32,
    pub rawmagn: u32,
    pub rawropblend: u32,
    pub rawcmp: u32,
    pub rawwac: u32,
    pub fbramid: u32,
    pub drawop: u32,
    pub xxx10: [u32; 2],
    pub fontlpat: u32,
    pub xxx11: u32,
    pub fontxy: u32,
    pub fontw: u32,
    pub fontinc: u32,
    pub font: u32,
    pub xxx12: [u32; 3],
    pub blend2: u32,
    pub preblend: u32,
    pub stencil: u32,
    pub stencilctl: u32,
    pub xxx13: [u32; 4],
    pub dcss1: u32,
    pub dcss2: u32,
    pub dcss3: u32,
    pub widpmask: u32,
    pub dcs2: u32,
    pub dcs3: u32,
    pub dcs4: u32,
    pub xxx14: u32,
    pub dcd2: u32,
    pub dcd3: u32,
    pub dcd4: u32,
    pub xxx15: u32,
    pub pattern: [u32; 32],
    pub xxx16: [u32; 256],
    pub devid: u32,
    pub xxx17: [u32; 63],
    pub ucsr: u32,
    pub xxx18: [u32; 31],
    pub mer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffb_dac {
    pub type: u32,
    pub value: u32,
    pub type2: u32,
    pub value2: u32,
}

pub const FFB_DAC_UCTRL: c_uint = 0x1001 /* User Control */;
pub const FFB_DAC_UCTRL_OVENAB: c_uint = 0x00000008 /* Overlay Enable */;
pub const FFB_DAC_UCTRL_WMODE: c_uint = 0x00000030 /* Window Mode */;
pub const FFB_DAC_UCTRL_WM_COMB: c_uint = 0x00000000 /* Window Mode = Combined */;
pub const FFB_DAC_UCTRL_MANREV: c_uint = 0x00000f00 /* 4-bit Manufacturing Revision */;
pub const FFB_DAC_UCTRL_MANREV_SHIFT: c_int = 8;
pub const FFB_DAC_TGEN: c_uint = 0x6000 /* Timing Generator */;
pub const FFB_DAC_TGEN_VIDE: c_uint = 0x00000001 /* Video Enable */;
pub const FFB_DAC_DID: c_uint = 0x8000 /* Device Identification */;
pub const FFB_DAC_DID_PNUM: c_uint = 0x0ffff000 /* Device Part Number */;
pub const FFB_DAC_DID_PNUM_SHIFT: c_int = 12;
pub const FFB_DAC_DID_REV: c_uint = 0xf0000000 /* Device Revision */;
pub const FFB_DAC_DID_REV_SHIFT: c_int = 28;
pub const FFB_DAC_CUR_CTRL: c_uint = 0x100;
pub const FFB_DAC_CUR_CTRL_P0: c_uint = 0x00000001;
pub const FFB_DAC_CUR_CTRL_P1: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffb_par {
    pub lock: spinlock_t,
    pub fbc: *mut ffb_fbc __iomem,
    pub dac: *mut ffb_dac __iomem,
    pub flags: u32,
pub const FFB_FLAG_AFB: c_uint = 0x00000001 /* AFB m3 or m6 */;
pub const FFB_FLAG_BLANKED: c_uint = 0x00000002 /* screen is blanked */;
pub const FFB_FLAG_INVCURSOR: c_uint = 0x00000004 /* DAC has inverted cursor logic */;
// C attribute field omitted
    pub bg_cache: u32,
    pub rop_cache: u32,
    pub fifo_cache: c_int,
    pub physbase: c_ulong,
    pub fbsize: c_ulong,
    pub board_type: c_int,
    pub pseudo_palette: [u32; 16],
}

#[no_mangle]
unsafe extern "C" fn FFBFifo(par: *mut ffb_par, n: c_int) {
    static void FFBFifo(struct ffb_par *par, int n)
    {
    struct ffb_fbc __iomem *fbc;
    let mut cache: c_int = par.fifo_cache;
    if (cache - n < 0) {
    fbc = par.fbc;
    do {
    cache = (upa_readl(&fbc.ucsr) & FFB_UCSR_FIFO_MASK);
    cache -= 8;
    } while (cache - n < 0);
    }
    par.fifo_cache = cache - n;
    }
#[no_mangle]
unsafe extern "C" fn FFBWait(par: *mut ffb_par) {
    static void FFBWait(struct ffb_par *par)
    {
    struct ffb_fbc __iomem *fbc;
    let mut limit: c_int = 10000;
    fbc = par.fbc;
    do {
    if ((upa_readl(&fbc.ucsr) & FFB_UCSR_ALL_BUSY) == 0)
    break;
    if ((upa_readl(&fbc.ucsr) & FFB_UCSR_ALL_ERRORS) != 0) {
    upa_writel(FFB_UCSR_ALL_ERRORS, &fbc.ucsr);
    }
    udelay(10);
    } while (--limit > 0);
    }
#[no_mangle]
unsafe extern "C" fn ffb_sync(p: *mut fb_info) -> c_int {
    static int ffb_sync(struct fb_info *p)
    {
    struct ffb_par *par = (struct ffb_par *)p.par;
    FFBWait(par);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ffb_rop(par: *mut ffb_par, rop: u32) -> __inline__ void {
    static __inline__ void ffb_rop(struct ffb_par *par, u32 rop)
    {
    if (par.rop_cache != rop) {
    FFBFifo(par, 1);
    upa_writel(rop, &par.fbc.rop);
    par.rop_cache = rop;
    }
    }
#[no_mangle]
unsafe extern "C" fn ffb_switch_from_graph(par: *mut ffb_par) {
    static void ffb_switch_from_graph(struct ffb_par *par)
    {
    struct ffb_fbc __iomem *fbc = par.fbc;
    struct ffb_dac __iomem *dac = par.dac;
    unsigned long flags, uctrl;
    spin_lock_irqsave(&par.lock, flags);
    FFBWait(par);
    par.fifo_cache = 0;
    FFBFifo(par, 7);
    upa_writel(FFB_PPC_VCE_DISABLE | FFB_PPC_TBE_OPAQUE |
    FFB_PPC_APE_DISABLE | FFB_PPC_CS_CONST,
    &fbc.ppc);
    upa_writel(0x2000707f, &fbc.fbc);
    upa_writel(par.rop_cache, &fbc.rop);
    upa_writel(0xffffffff, &fbc.pmask);
    upa_writel((1 << 16) | (0 << 0), &fbc.fontinc);
    upa_writel(par.fg_cache, &fbc.fg);
    upa_writel(par.bg_cache, &fbc.bg);
    FFBWait(par);
// Disable cursor.
    upa_writel(FFB_DAC_CUR_CTRL, &dac.type2);
    if (par.flags & FFB_FLAG_INVCURSOR)
    upa_writel(0, &dac.value2);
    else
    upa_writel((FFB_DAC_CUR_CTRL_P0 |
    FFB_DAC_CUR_CTRL_P1), &dac.value2);
// Disable overlay and window modes.
    upa_writel(FFB_DAC_UCTRL, &dac.type);
    uctrl = upa_readl(&dac.value);
    uctrl &= ~FFB_DAC_UCTRL_WMODE;
    uctrl |= FFB_DAC_UCTRL_WM_COMB;
    uctrl &= ~FFB_DAC_UCTRL_OVENAB;
    upa_writel(FFB_DAC_UCTRL, &dac.type);
    upa_writel(uctrl, &dac.value);
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ffb_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int ffb_pan_display(struct fb_var_screeninfo *var, struct fb_info *info)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
// We just use this to catch switches out of
// graphics mode.
//
    ffb_switch_from_graph(par);
    if (var.xoffset || var.yoffset || var.vmode)
    return -EINVAL;
    return 0;
    }
//
// ffb_fillrect - Draws a rectangle on the screen.
//
// @info: frame buffer structure that represents a single frame buffer
// @rect: structure defining the rectagle and operation.
//
#[no_mangle]
unsafe extern "C" fn ffb_fillrect(info: *mut fb_info, rect: *const fb_fillrect) {
    static void ffb_fillrect(struct fb_info *info, const struct fb_fillrect *rect)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    struct ffb_fbc __iomem *fbc = par.fbc;
    unsigned long flags;
    u32 fg;
    BUG_ON(rect.rop != ROP_COPY && rect.rop != ROP_XOR);
    fg = ((u32 *)info.pseudo_palette)[rect.color];
    spin_lock_irqsave(&par.lock, flags);
    if (fg != par.fg_cache) {
    FFBFifo(par, 1);
    upa_writel(fg, &fbc.fg);
    par.fg_cache = fg;
    }
    ffb_rop(par, rect.rop == ROP_COPY ?
    FFB_ROP_NEW :
    FFB_ROP_NEW_XOR_OLD);
    FFBFifo(par, 5);
    upa_writel(FFB_DRAWOP_RECTANGLE, &fbc.drawop);
    upa_writel(rect.dy, &fbc.by);
    upa_writel(rect.dx, &fbc.bx);
    upa_writel(rect.height, &fbc.bh);
    upa_writel(rect.width, &fbc.bw);
    spin_unlock_irqrestore(&par.lock, flags);
    }
//
// ffb_copyarea - Copies on area of the screen to another area.
//
// @info: frame buffer structure that represents a single frame buffer
// @area: structure defining the source and destination.
//
#[no_mangle]
unsafe extern "C" fn ffb_copyarea(info: *mut fb_info, area: *const fb_copyarea) {
    static void ffb_copyarea(struct fb_info *info, const struct fb_copyarea *area)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    struct ffb_fbc __iomem *fbc = par.fbc;
    unsigned long flags;
    if (area.dx != area.sx ||
    area.dy == area.sy) {
    cfb_copyarea(info, area);
    return;
    }
    spin_lock_irqsave(&par.lock, flags);
    ffb_rop(par, FFB_ROP_OLD);
    FFBFifo(par, 7);
    upa_writel(FFB_DRAWOP_VSCROLL, &fbc.drawop);
    upa_writel(area.sy, &fbc.by);
    upa_writel(area.sx, &fbc.bx);
    upa_writel(area.dy, &fbc.dy);
    upa_writel(area.dx, &fbc.dx);
    upa_writel(area.height, &fbc.bh);
    upa_writel(area.width, &fbc.bw);
    spin_unlock_irqrestore(&par.lock, flags);
    }
//
// ffb_imageblit - Copies a image from system memory to the screen.
//
// @info: frame buffer structure that represents a single frame buffer
// @image: structure defining the image.
//
#[no_mangle]
unsafe extern "C" fn ffb_imageblit(info: *mut fb_info, image: *const fb_image) {
    static void ffb_imageblit(struct fb_info *info, const struct fb_image *image)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    struct ffb_fbc __iomem *fbc = par.fbc;
    const u8 *data = image.data;
    unsigned long flags;
    u32 fg, bg, xy;
    u64 fgbg;
    int i, width, stride;
    if (image.depth > 1) {
    cfb_imageblit(info, image);
    return;
    }
    fg = ((u32 *)info.pseudo_palette)[image.fg_color];
    bg = ((u32 *)info.pseudo_palette)[image.bg_color];
    fgbg = ((u64) fg << 32) | (u64) bg;
    xy = (image.dy << 16) | image.dx;
    width = image.width;
    stride = ((width + 7) >> 3);
    spin_lock_irqsave(&par.lock, flags);
    if (fgbg != *(u64 *)&par.fg_cache) {
    FFBFifo(par, 2);
    upa_writeq(fgbg, &fbc.fg);
// (u64 *)&par->fg_cache = fgbg;
    }
    if (width >= 32) {
    FFBFifo(par, 1);
    upa_writel(32, &fbc.fontw);
    }
    while (width >= 32) {
    const u8 *next_data = data + 4;
    FFBFifo(par, 1);
    upa_writel(xy, &fbc.fontxy);
    xy += (32 << 0);
    for (i = 0; i < image.height; i++) {
    u32 val = (((u32)data[0] << 24) |
    ((u32)data[1] << 16) |
    ((u32)data[2] <<  8) |
    ((u32)data[3] <<  0));
    FFBFifo(par, 1);
    upa_writel(val, &fbc.font);
    data += stride;
    }
    data = next_data;
    width -= 32;
    }
    if (width) {
    FFBFifo(par, 2);
    upa_writel(width, &fbc.fontw);
    upa_writel(xy, &fbc.fontxy);
    for (i = 0; i < image.height; i++) {
    u32 val = (((u32)data[0] << 24) |
    ((u32)data[1] << 16) |
    ((u32)data[2] <<  8) |
    ((u32)data[3] <<  0));
    FFBFifo(par, 1);
    upa_writel(val, &fbc.font);
    data += stride;
    }
    }
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ffb_fixup_var_rgb(var: *mut fb_var_screeninfo) {
    static void ffb_fixup_var_rgb(struct fb_var_screeninfo *var)
    {
    var.red.offset = 0;
    var.red.length = 8;
    var.green.offset = 8;
    var.green.length = 8;
    var.blue.offset = 16;
    var.blue.length = 8;
    var.transp.offset = 0;
    var.transp.length = 0;
    }
//
// ffb_setcolreg - Sets a color register.
//
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
    static int ffb_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    u32 value;
    if (regno >= 16)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    value = (blue << 16) | (green << 8) | red;
    ((u32 *)info.pseudo_palette)[regno] = value;
    return 0;
    }
//
// ffb_blank - Optional function.  Blanks the display.
// @blank: the blank mode we want.
// @info: frame buffer structure that represents a single frame buffer
//
#[no_mangle]
unsafe extern "C" fn ffb_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int ffb_blank(int blank, struct fb_info *info)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    struct ffb_dac __iomem *dac = par.dac;
    unsigned long flags;
    u32 val;
    int i;
    spin_lock_irqsave(&par.lock, flags);
    FFBWait(par);
    upa_writel(FFB_DAC_TGEN, &dac.type);
    val = upa_readl(&dac.value);
    switch (blank) {
    case FB_BLANK_UNBLANK: /* Unblanking */
    val |= FFB_DAC_TGEN_VIDE;
    par.flags &= ~FFB_FLAG_BLANKED;
    break;
    case FB_BLANK_NORMAL: /* Normal blanking */
    case FB_BLANK_VSYNC_SUSPEND: /* VESA blank (vsync off) */
    case FB_BLANK_HSYNC_SUSPEND: /* VESA blank (hsync off) */
    case FB_BLANK_POWERDOWN: /* Poweroff */
    val &= ~FFB_DAC_TGEN_VIDE;
    par.flags |= FFB_FLAG_BLANKED;
    break;
    }
    upa_writel(FFB_DAC_TGEN, &dac.type);
    upa_writel(val, &dac.value);
    for (i = 0; i < 10; i++) {
    upa_writel(FFB_DAC_TGEN, &dac.type);
    upa_readl(&dac.value);
    }
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
    static const struct sbus_mmap_map ffb_mmap_map[] = {
    {
    .voff	= FFB_SFB8R_VOFF,
    .poff	= FFB_SFB8R_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_SFB8G_VOFF,
    .poff	= FFB_SFB8G_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_SFB8B_VOFF,
    .poff	= FFB_SFB8B_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_SFB8X_VOFF,
    .poff	= FFB_SFB8X_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_SFB32_VOFF,
    .poff	= FFB_SFB32_POFF,
    .size	= 0x1000000
    },
    {
    .voff	= FFB_SFB64_VOFF,
    .poff	= FFB_SFB64_POFF,
    .size	= 0x2000000
    },
    {
    .voff	= FFB_FBC_REGS_VOFF,
    .poff	= FFB_FBC_REGS_POFF,
    .size	= 0x0002000
    },
    {
    .voff	= FFB_BM_FBC_REGS_VOFF,
    .poff	= FFB_BM_FBC_REGS_POFF,
    .size	= 0x0002000
    },
    {
    .voff	= FFB_DFB8R_VOFF,
    .poff	= FFB_DFB8R_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_DFB8G_VOFF,
    .poff	= FFB_DFB8G_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_DFB8B_VOFF,
    .poff	= FFB_DFB8B_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_DFB8X_VOFF,
    .poff	= FFB_DFB8X_POFF,
    .size	= 0x0400000
    },
    {
    .voff	= FFB_DFB24_VOFF,
    .poff	= FFB_DFB24_POFF,
    .size	= 0x1000000
    },
    {
    .voff	= FFB_DFB32_VOFF,
    .poff	= FFB_DFB32_POFF,
    .size	= 0x1000000
    },
    {
    .voff	= FFB_FBC_KREGS_VOFF,
    .poff	= FFB_FBC_KREGS_POFF,
    .size	= 0x0002000
    },
    {
    .voff	= FFB_DAC_VOFF,
    .poff	= FFB_DAC_POFF,
    .size	= 0x0002000
    },
    {
    .voff	= FFB_PROM_VOFF,
    .poff	= FFB_PROM_POFF,
    .size	= 0x0010000
    },
    {
    .voff	= FFB_EXP_VOFF,
    .poff	= FFB_EXP_POFF,
    .size	= 0x0002000
    },
    {
    .voff	= FFB_DFB422A_VOFF,
    .poff	= FFB_DFB422A_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_DFB422AD_VOFF,
    .poff	= FFB_DFB422AD_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_DFB24B_VOFF,
    .poff	= FFB_DFB24B_POFF,
    .size	= 0x1000000
    },
    {
    .voff	= FFB_DFB422B_VOFF,
    .poff	= FFB_DFB422B_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_DFB422BD_VOFF,
    .poff	= FFB_DFB422BD_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_SFB16Z_VOFF,
    .poff	= FFB_SFB16Z_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_SFB8Z_VOFF,
    .poff	= FFB_SFB8Z_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_SFB422_VOFF,
    .poff	= FFB_SFB422_POFF,
    .size	= 0x0800000
    },
    {
    .voff	= FFB_SFB422D_VOFF,
    .poff	= FFB_SFB422D_POFF,
    .size	= 0x0800000
    },
    { .size = 0 }
    };
#[no_mangle]
unsafe extern "C" fn ffb_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int ffb_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    return sbusfb_mmap_helper(ffb_mmap_map,
    par.physbase, par.fbsize,
    0, vma);
    }
#[no_mangle]
unsafe extern "C" fn ffb_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int ffb_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    return sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_CREATOR, 24, par.fbsize);
    }
//
// Initialisation
//
#[no_mangle]
unsafe extern "C" fn ffb_init_fix(info: *mut fb_info) {
    static void ffb_init_fix(struct fb_info *info)
    {
    struct ffb_par *par = (struct ffb_par *)info.par;
    const char *ffb_type_name;
    if (!(par.flags & FFB_FLAG_AFB)) {
    if ((par.board_type & 0x7) == 0x3)
    ffb_type_name = "Creator 3D";
    else
    ffb_type_name = "Creator";
    } else
    ffb_type_name = "Elite 3D";
    strscpy(info.fix.id, ffb_type_name, sizeof(info.fix.id));
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_TRUECOLOR;
// Framebuffer length is the same regardless of resolution.
    info.fix.line_length = 8192;
    info.fix.accel = FB_ACCEL_SUN_CREATOR;
    }
#[no_mangle]
unsafe extern "C" fn ffb_probe(op: *mut platform_device) -> c_int {
    static int ffb_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct ffb_fbc __iomem *fbc;
    struct ffb_dac __iomem *dac;
    struct fb_info *info;
    struct ffb_par *par;
    u32 dac_pnum, dac_rev, dac_mrev;
    int err;
    info = framebuffer_alloc(sizeof(struct ffb_par), &op.dev);
    err = -ENOMEM;
    if (!info)
    goto out_err;
    par = info.par;
    spin_lock_init(&par.lock);
    par.fbc = of_ioremap(&op.resource[2], 0,
    sizeof(struct ffb_fbc), "ffb fbc");
    if (!par.fbc)
    goto out_release_fb;
    par.dac = of_ioremap(&op.resource[1], 0,
    sizeof(struct ffb_dac), "ffb dac");
    if (!par.dac)
    goto out_unmap_fbc;
    par.rop_cache = FFB_ROP_NEW;
    par.physbase = op.resource[0].start;
// Don't mention copyarea, so SCROLL_REDRAW is always
// used.  It is the fastest on this chip.
//
    info.flags = (/* FBINFO_HWACCEL_COPYAREA | */
    FBINFO_HWACCEL_FILLRECT |
    FBINFO_HWACCEL_IMAGEBLIT);
    info.fbops = &ffb_ops;
    info.screen_base = (char *) par.physbase + FFB_DFB24_POFF;
    info.pseudo_palette = par.pseudo_palette;
    sbusfb_fill_var(&info.var, dp, 32);
    par.fbsize = PAGE_ALIGN(info.var.xres * info.var.yres * 4);
    ffb_fixup_var_rgb(&info.var);
    info.var.accel_flags = FB_ACCELF_TEXT;
    if (of_node_name_eq(dp, "SUNW,afb"))
    par.flags |= FFB_FLAG_AFB;
    par.board_type = of_getintprop_default(dp, "board_type", 0);
    fbc = par.fbc;
    if ((upa_readl(&fbc.ucsr) & FFB_UCSR_ALL_ERRORS) != 0)
    upa_writel(FFB_UCSR_ALL_ERRORS, &fbc.ucsr);
    dac = par.dac;
    upa_writel(FFB_DAC_DID, &dac.type);
    dac_pnum = upa_readl(&dac.value);
    dac_rev = (dac_pnum & FFB_DAC_DID_REV) >> FFB_DAC_DID_REV_SHIFT;
    dac_pnum = (dac_pnum & FFB_DAC_DID_PNUM) >> FFB_DAC_DID_PNUM_SHIFT;
    upa_writel(FFB_DAC_UCTRL, &dac.type);
    dac_mrev = upa_readl(&dac.value);
    dac_mrev = (dac_mrev & FFB_DAC_UCTRL_MANREV) >>
    FFB_DAC_UCTRL_MANREV_SHIFT;
// Elite3D has different DAC revision numbering, and no DAC revisions
// have the reversed meaning of cursor enable.  Otherwise, Pacifica 1
// ramdacs with manufacturing revision less than 3 have inverted
// cursor logic.  We identify Pacifica 1 as not Pacifica 2, the
// latter having a part number value of 0x236e.
//
    if ((par.flags & FFB_FLAG_AFB) || dac_pnum == 0x236e) {
    par.flags &= ~FFB_FLAG_INVCURSOR;
    } else {
    if (dac_mrev < 3)
    par.flags |= FFB_FLAG_INVCURSOR;
    }
    ffb_switch_from_graph(par);
// Unblank it just to be sure.  When there are multiple
// FFB/AFB cards in the system, or it is not the OBP
// chosen console, it will have video outputs off in
// the DAC.
//
    ffb_blank(FB_BLANK_UNBLANK, info);
    if (fb_alloc_cmap(&info.cmap, 256, 0))
    goto out_unmap_dac;
    ffb_init_fix(info);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: %s at %016lx, type %d, "
    "DAC pnum[%x] rev[%d] manuf_rev[%d]\n",
    dp,
    ((par.flags & FFB_FLAG_AFB) ? "AFB" : "FFB"),
    par.physbase, par.board_type,
    dac_pnum, dac_rev, dac_mrev);
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_dac:
    of_iounmap(&op.resource[1], par.dac, sizeof(struct ffb_dac));
    out_unmap_fbc:
    of_iounmap(&op.resource[2], par.fbc, sizeof(struct ffb_fbc));
    out_release_fb:
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ffb_remove(op: *mut platform_device) {
    static void ffb_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct ffb_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    of_iounmap(&op.resource[2], par.fbc, sizeof(struct ffb_fbc));
    of_iounmap(&op.resource[1], par.dac, sizeof(struct ffb_dac));
    framebuffer_release(info);
    }
    static const struct of_device_id ffb_match[] = {
    {
    .name = "SUNW,ffb",
    },
    {
    .name = "SUNW,afb",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ffb_match);
    static struct platform_driver ffb_driver = {
    .driver = {
    .name = "ffb",
    .of_match_table = ffb_match,
    },
    .probe		= ffb_probe,
    .remove		= ffb_remove,
    };
#[no_mangle]
unsafe extern "C" fn ffb_init() -> int __init {
    static int __init ffb_init(void)
    {
    if (fb_get_options("ffb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&ffb_driver);
    }
#[no_mangle]
unsafe extern "C" fn ffb_exit() -> void __exit {
    static void __exit ffb_exit(void)
    {
    platform_driver_unregister(&ffb_driver);
    }
    module_init(ffb_init);
    module_exit(ffb_exit);
    MODULE_DESCRIPTION("framebuffer driver for Creator/Elite3D chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
