//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/leo.c
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
// leo.c: LEO frame buffer driver
//
// Copyright (C) 2003, 2006 David S. Miller (davem@davemloft.net)
// Copyright (C) 1996-1999 Jakub Jelinek (jj@ultra.linux.cz)
// Copyright (C) 1997 Michal Rehacek (Michal.Rehacek@st.mff.cuni.cz)
//
// Driver layout based loosely on tgafb.c, see that file for credits.
//

//
// Local functions.
//
    static int leo_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int leo_blank(int, struct fb_info *);
    static int leo_pan_display(struct fb_var_screeninfo *, struct fb_info *);
    static int leo_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int leo_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops leo_ops = {
    .owner			= THIS_MODULE,
    FB_DEFAULT_SBUS_OPS(leo),
    .fb_setcolreg		= leo_setcolreg,
    .fb_blank		= leo_blank,
    .fb_pan_display		= leo_pan_display,
    };
pub const LEO_OFF_LC_SS0_KRN: c_uint = 0x00200000UL;
pub const LEO_OFF_LC_SS0_USR: c_uint = 0x00201000UL;
pub const LEO_OFF_LC_SS1_KRN: c_uint = 0x01200000UL;
pub const LEO_OFF_LC_SS1_USR: c_uint = 0x01201000UL;
pub const LEO_OFF_LD_SS0: c_uint = 0x00400000UL;
pub const LEO_OFF_LD_SS1: c_uint = 0x01400000UL;
pub const LEO_OFF_LD_GBL: c_uint = 0x00401000UL;
pub const LEO_OFF_LX_KRN: c_uint = 0x00600000UL;
pub const LEO_OFF_LX_CURSOR: c_uint = 0x00601000UL;
pub const LEO_OFF_SS0: c_uint = 0x00800000UL;
pub const LEO_OFF_SS1: c_uint = 0x01800000UL;
pub const LEO_OFF_UNK: c_uint = 0x00602000UL;
pub const LEO_OFF_UNK2: c_uint = 0x00000000UL;
pub const LEO_CUR_ENABLE: c_uint = 0x00000080;
pub const LEO_CUR_UPDATE: c_uint = 0x00000030;
pub const LEO_CUR_PROGRESS: c_uint = 0x00000006;
pub const LEO_CUR_UPDATECMAP: c_uint = 0x00000003;
pub const LEO_CUR_TYPE_MASK: c_uint = 0x00000000;
pub const LEO_CUR_TYPE_IMAGE: c_uint = 0x00000020;
pub const LEO_CUR_TYPE_CMAP: c_uint = 0x00000050;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_cursor {
    pub xxx0: [u8; 16],
    pub cur_type: u32,
    pub cur_misc: u32,
    pub cur_cursxy: u32,
    pub cur_data: u32,
}

pub const LEO_KRN_TYPE_CLUT0: c_uint = 0x00001000;
pub const LEO_KRN_TYPE_CLUT1: c_uint = 0x00001001;
pub const LEO_KRN_TYPE_CLUT2: c_uint = 0x00001002;
pub const LEO_KRN_TYPE_WID: c_uint = 0x00001003;
pub const LEO_KRN_TYPE_UNK: c_uint = 0x00001006;
pub const LEO_KRN_TYPE_VIDEO: c_uint = 0x00002003;
pub const LEO_KRN_TYPE_CLUTDATA: c_uint = 0x00004000;
pub const LEO_KRN_CSR_ENABLE: c_uint = 0x00000008;
pub const LEO_KRN_CSR_PROGRESS: c_uint = 0x00000004;
pub const LEO_KRN_CSR_UNK: c_uint = 0x00000002;
pub const LEO_KRN_CSR_UNK2: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_lx_krn {
    pub krn_type: u32,
    pub krn_csr: u32,
    pub krn_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_lc_ss0_krn {
    pub misc: u32,
    pub xxx0: [u8; 0x800-4],
    pub rev: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_lc_ss0_usr {
    pub csr: u32,
    pub addrspace: u32,
    pub fontmsk: u32,
    pub fontt: u32,
    pub extent: u32,
    pub src: u32,
    pub dst: u32,
    pub copy: u32,
    pub fill: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_lc_ss1_krn {
    pub unknown: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_lc_ss1_usr {
    pub unknown: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_ld_ss0 {
    pub xxx0: [u8; 0xe00],
    pub csr: u32,
    pub wid: u32,
    pub wmask: u32,
    pub widclip: u32,
    pub vclipmin: u32,
    pub vclipmax: u32,
    pub /: *mut *mut u32 pickmin; / SS1 only,
    pub /: *mut *mut u32 pickmax; / SS1 only,
    pub fg: u32,
    pub bg: u32,
    pub /: *mut *mut u32 src; / Copy/Scroll (SS0 only),
    pub /: *mut *mut u32 dst; / Copy/Scroll/Fill (SS0 only),
    pub /: *mut *mut u32 extent; / Copy/Scroll/Fill size (SS0 only),
    pub xxx1: [u32; 3],
    pub /: *mut *mut u32 setsem; / SS1 only,
    pub /: *mut *mut u32 clrsem; / SS1 only,
    pub /: *mut *mut u32 clrpick; / SS1 only,
    pub /: *mut *mut u32 clrdat; / SS1 only,
    pub /: *mut *mut u32 alpha; / SS1 only,
    pub xxx2: [u8; 0x2c],
    pub winbg: u32,
    pub planemask: u32,
    pub rop: u32,
    pub z: u32,
    pub /: *mut *mut u32 dczf; / SS1 only,
    pub /: *mut *mut u32 dczb; / SS1 only,
    pub /: *mut *mut u32 dcs; / SS1 only,
    pub /: *mut *mut u32 dczs; / SS1 only,
    pub /: *mut *mut u32 pickfb; / SS1 only,
    pub /: *mut *mut u32 pickbb; / SS1 only,
    pub /: *mut *mut u32 dcfc; / SS1 only,
    pub /: *mut *mut u32 forcecol; / SS1 only,
    pub /: *mut *mut u32 door[8]; / SS1 only,
    pub /: *mut *mut u32 pick[5]; / SS1 only,
}

pub const LEO_SS1_MISC_ENABLE: c_uint = 0x00000001;
pub const LEO_SS1_MISC_STEREO: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_ld_ss1 {
    pub xxx0: [u8; 0xef4],
    pub ss1_misc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_ld_gbl {
    pub unknown: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leo_par {
    pub lock: spinlock_t,
    pub lx_krn: *mut leo_lx_krn __iomem,
    pub lc_ss0_usr: *mut leo_lc_ss0_usr __iomem,
    pub ld_ss0: *mut leo_ld_ss0 __iomem,
    pub ld_ss1: *mut leo_ld_ss1 __iomem,
    pub cursor: *mut leo_cursor __iomem,
    pub extent: u32,
    pub clut_data: [u32; 256],
    pub flags: u32,
pub const LEO_FLAG_BLANKED: c_uint = 0x00000001;
    pub which_io: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn leo_wait(lx_krn: *mut leo_lx_krn __iomem) {
    static void leo_wait(struct leo_lx_krn __iomem *lx_krn)
    {
    int i;
    for (i = 0;
    (sbus_readl(&lx_krn.krn_csr) & LEO_KRN_CSR_PROGRESS) &&
    i < 300000;
    i++)
    udelay(1); /* Busy wait at most 0.3 sec */
    return;
    }
#[no_mangle]
unsafe extern "C" fn leo_switch_from_graph(info: *mut fb_info) {
    static void leo_switch_from_graph(struct fb_info *info)
    {
    struct leo_par *par = (struct leo_par *) info.par;
    struct leo_ld_ss0 __iomem *ss = par.ld_ss0;
    struct leo_cursor __iomem *cursor = par.cursor;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&par.lock, flags);
    par.extent = ((info.var.xres - 1) |
    ((info.var.yres - 1) << 16));
    sbus_writel(0xffffffff, &ss.wid);
    sbus_writel(0xffff, &ss.wmask);
    sbus_writel(0, &ss.vclipmin);
    sbus_writel(par.extent, &ss.vclipmax);
    sbus_writel(0, &ss.fg);
    sbus_writel(0xff000000, &ss.planemask);
    sbus_writel(0x310850, &ss.rop);
    sbus_writel(0, &ss.widclip);
    sbus_writel((info.var.xres-1) | ((info.var.yres-1) << 11),
    &par.lc_ss0_usr.extent);
    sbus_writel(4, &par.lc_ss0_usr.addrspace);
    sbus_writel(0x80000000, &par.lc_ss0_usr.fill);
    sbus_writel(0, &par.lc_ss0_usr.fontt);
    do {
    val = sbus_readl(&par.lc_ss0_usr.csr);
    } while (val & 0x20000000);
// setup screen buffer for cfb_* functions
    sbus_writel(1, &ss.wid);
    sbus_writel(0x00ffffff, &ss.planemask);
    sbus_writel(0x310b90, &ss.rop);
    sbus_writel(0, &par.lc_ss0_usr.addrspace);
// hide cursor
    sbus_writel(sbus_readl(&cursor.cur_misc) & ~LEO_CUR_ENABLE, &cursor.cur_misc);
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn leo_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int leo_pan_display(struct fb_var_screeninfo *var, struct fb_info *info)
    {
// We just use this to catch switches out of
// graphics mode.
//
    leo_switch_from_graph(info);
    if (var.xoffset || var.yoffset || var.vmode)
    return -EINVAL;
    return 0;
    }
//
// leo_setcolreg - Optional function. Sets a color register.
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
    static int leo_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    struct leo_par *par = (struct leo_par *) info.par;
    struct leo_lx_krn __iomem *lx_krn = par.lx_krn;
    unsigned long flags;
    u32 val;
    int i;
    if (regno >= 256)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    par.clut_data[regno] = red | (green << 8) | (blue << 16);
    spin_lock_irqsave(&par.lock, flags);
    leo_wait(lx_krn);
    sbus_writel(LEO_KRN_TYPE_CLUTDATA, &lx_krn.krn_type);
    for (i = 0; i < 256; i++)
    sbus_writel(par.clut_data[i], &lx_krn.krn_value);
    sbus_writel(LEO_KRN_TYPE_CLUT0, &lx_krn.krn_type);
    val = sbus_readl(&lx_krn.krn_csr);
    val |= (LEO_KRN_CSR_UNK | LEO_KRN_CSR_UNK2);
    sbus_writel(val, &lx_krn.krn_csr);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
//
// leo_blank - Optional function.  Blanks the display.
// @blank: the blank mode we want.
// @info: frame buffer structure that represents a single frame buffer
//
#[no_mangle]
unsafe extern "C" fn leo_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int leo_blank(int blank, struct fb_info *info)
    {
    struct leo_par *par = (struct leo_par *) info.par;
    struct leo_lx_krn __iomem *lx_krn = par.lx_krn;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&par.lock, flags);
    switch (blank) {
    case FB_BLANK_UNBLANK: /* Unblanking */
    val = sbus_readl(&lx_krn.krn_csr);
    val |= LEO_KRN_CSR_ENABLE;
    sbus_writel(val, &lx_krn.krn_csr);
    par.flags &= ~LEO_FLAG_BLANKED;
    break;
    case FB_BLANK_NORMAL: /* Normal blanking */
    case FB_BLANK_VSYNC_SUSPEND: /* VESA blank (vsync off) */
    case FB_BLANK_HSYNC_SUSPEND: /* VESA blank (hsync off) */
    case FB_BLANK_POWERDOWN: /* Poweroff */
    val = sbus_readl(&lx_krn.krn_csr);
    val &= ~LEO_KRN_CSR_ENABLE;
    sbus_writel(val, &lx_krn.krn_csr);
    par.flags |= LEO_FLAG_BLANKED;
    break;
    }
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
    static const struct sbus_mmap_map leo_mmap_map[] = {
    {
    .voff	= LEO_SS0_MAP,
    .poff	= LEO_OFF_SS0,
    .size	= 0x800000
    },
    {
    .voff	= LEO_LC_SS0_USR_MAP,
    .poff	= LEO_OFF_LC_SS0_USR,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LD_SS0_MAP,
    .poff	= LEO_OFF_LD_SS0,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LX_CURSOR_MAP,
    .poff	= LEO_OFF_LX_CURSOR,
    .size	= 0x1000
    },
    {
    .voff	= LEO_SS1_MAP,
    .poff	= LEO_OFF_SS1,
    .size	= 0x800000
    },
    {
    .voff	= LEO_LC_SS1_USR_MAP,
    .poff	= LEO_OFF_LC_SS1_USR,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LD_SS1_MAP,
    .poff	= LEO_OFF_LD_SS1,
    .size	= 0x1000
    },
    {
    .voff	= LEO_UNK_MAP,
    .poff	= LEO_OFF_UNK,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LX_KRN_MAP,
    .poff	= LEO_OFF_LX_KRN,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LC_SS0_KRN_MAP,
    .poff	= LEO_OFF_LC_SS0_KRN,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LC_SS1_KRN_MAP,
    .poff	= LEO_OFF_LC_SS1_KRN,
    .size	= 0x1000
    },
    {
    .voff	= LEO_LD_GBL_MAP,
    .poff	= LEO_OFF_LD_GBL,
    .size	= 0x1000
    },
    {
    .voff	= LEO_UNK2_MAP,
    .poff	= LEO_OFF_UNK2,
    .size	= 0x100000
    },
    { .size = 0 }
    };
#[no_mangle]
unsafe extern "C" fn leo_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int leo_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct leo_par *par = (struct leo_par *)info.par;
    return sbusfb_mmap_helper(leo_mmap_map,
    info.fix.smem_start, info.fix.smem_len,
    par.which_io, vma);
    }
#[no_mangle]
unsafe extern "C" fn leo_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int leo_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    return sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_SUNLEO, 32, info.fix.smem_len);
    }
//
// Initialisation
//
    static void
    leo_init_fix(struct fb_info *info, struct device_node *dp)
    {
    snprintf(info.fix.id, sizeof(info.fix.id), "%pOFn", dp);
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_TRUECOLOR;
    info.fix.line_length = 8192;
    info.fix.accel = FB_ACCEL_SUN_LEO;
    }
#[no_mangle]
unsafe extern "C" fn leo_wid_put(info: *mut fb_info, wl: *mut fb_wid_list) {
    static void leo_wid_put(struct fb_info *info, struct fb_wid_list *wl)
    {
    struct leo_par *par = (struct leo_par *) info.par;
    struct leo_lx_krn __iomem *lx_krn = par.lx_krn;
    struct fb_wid_item *wi;
    unsigned long flags;
    u32 val;
    int i, j;
    spin_lock_irqsave(&par.lock, flags);
    leo_wait(lx_krn);
    for (i = 0, wi = wl.wl_list; i < wl.wl_count; i++, wi++) {
    switch (wi.wi_type) {
    case FB_WID_DBL_8:
    j = (wi.wi_index & 0xf) + 0x40;
    break;
    case FB_WID_DBL_24:
    j = wi.wi_index & 0x3f;
    break;
    default:
    continue;
    }
    sbus_writel(0x5800 + j, &lx_krn.krn_type);
    sbus_writel(wi.wi_values[0], &lx_krn.krn_value);
    }
    sbus_writel(LEO_KRN_TYPE_WID, &lx_krn.krn_type);
    val = sbus_readl(&lx_krn.krn_csr);
    val |= (LEO_KRN_CSR_UNK | LEO_KRN_CSR_UNK2);
    sbus_writel(val, &lx_krn.krn_csr);
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn leo_init_wids(info: *mut fb_info) {
    static void leo_init_wids(struct fb_info *info)
    {
    struct fb_wid_item wi;
    struct fb_wid_list wl;
    wl.wl_count = 1;
    wl.wl_list = &wi;
    wi.wi_type = FB_WID_DBL_8;
    wi.wi_index = 0;
    wi.wi_values [0] = 0x2c0;
    leo_wid_put(info, &wl);
    wi.wi_index = 1;
    wi.wi_values [0] = 0x30;
    leo_wid_put(info, &wl);
    wi.wi_index = 2;
    wi.wi_values [0] = 0x20;
    leo_wid_put(info, &wl);
    wi.wi_type = FB_WID_DBL_24;
    wi.wi_index = 1;
    wi.wi_values [0] = 0x30;
    leo_wid_put(info, &wl);
    }
#[no_mangle]
unsafe extern "C" fn leo_init_hw(info: *mut fb_info) {
    static void leo_init_hw(struct fb_info *info)
    {
    struct leo_par *par = (struct leo_par *) info.par;
    u32 val;
    val = sbus_readl(&par.ld_ss1.ss1_misc);
    val |= LEO_SS1_MISC_ENABLE;
    sbus_writel(val, &par.ld_ss1.ss1_misc);
    leo_switch_from_graph(info);
    }
#[no_mangle]
unsafe extern "C" fn leo_fixup_var_rgb(var: *mut fb_var_screeninfo) {
    static void leo_fixup_var_rgb(struct fb_var_screeninfo *var)
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
    static void leo_unmap_regs(struct platform_device *op, struct fb_info *info,
    struct leo_par *par)
    {
    if (par.lc_ss0_usr)
    of_iounmap(&op.resource[0], par.lc_ss0_usr, 0x1000);
    if (par.ld_ss0)
    of_iounmap(&op.resource[0], par.ld_ss0, 0x1000);
    if (par.ld_ss1)
    of_iounmap(&op.resource[0], par.ld_ss1, 0x1000);
    if (par.lx_krn)
    of_iounmap(&op.resource[0], par.lx_krn, 0x1000);
    if (par.cursor)
    of_iounmap(&op.resource[0],
    par.cursor, sizeof(struct leo_cursor));
    if (info.screen_base)
    of_iounmap(&op.resource[0], info.screen_base, 0x800000);
    }
#[no_mangle]
unsafe extern "C" fn leo_probe(op: *mut platform_device) -> c_int {
    static int leo_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct fb_info *info;
    struct leo_par *par;
    int linebytes, err;
    info = framebuffer_alloc(sizeof(struct leo_par), &op.dev);
    err = -ENOMEM;
    if (!info)
    goto out_err;
    par = info.par;
    spin_lock_init(&par.lock);
    info.fix.smem_start = op.resource[0].start;
    par.which_io = op.resource[0].flags & IORESOURCE_BITS;
    sbusfb_fill_var(&info.var, dp, 32);
    leo_fixup_var_rgb(&info.var);
    linebytes = of_getintprop_default(dp, "linebytes",
    info.var.xres);
    info.fix.smem_len = PAGE_ALIGN(linebytes * info.var.yres);
    par.lc_ss0_usr =
    of_ioremap(&op.resource[0], LEO_OFF_LC_SS0_USR,
    0x1000, "leolc ss0usr");
    par.ld_ss0 =
    of_ioremap(&op.resource[0], LEO_OFF_LD_SS0,
    0x1000, "leold ss0");
    par.ld_ss1 =
    of_ioremap(&op.resource[0], LEO_OFF_LD_SS1,
    0x1000, "leold ss1");
    par.lx_krn =
    of_ioremap(&op.resource[0], LEO_OFF_LX_KRN,
    0x1000, "leolx krn");
    par.cursor =
    of_ioremap(&op.resource[0], LEO_OFF_LX_CURSOR,
    sizeof(struct leo_cursor), "leolx cursor");
    info.screen_base =
    of_ioremap(&op.resource[0], LEO_OFF_SS0,
    0x800000, "leo ram");
    if (!par.lc_ss0_usr ||
    !par.ld_ss0 ||
    !par.ld_ss1 ||
    !par.lx_krn ||
    !par.cursor ||
    !info.screen_base)
    goto out_unmap_regs;
    info.fbops = &leo_ops;
    info.pseudo_palette = par.clut_data;
    leo_init_wids(info);
    leo_init_hw(info);
    leo_blank(FB_BLANK_UNBLANK, info);
    if (fb_alloc_cmap(&info.cmap, 256, 0))
    goto out_unmap_regs;
    leo_init_fix(info, dp);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: leo at %lx:%lx\n",
    dp,
    par.which_io, info.fix.smem_start);
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_regs:
    leo_unmap_regs(op, info, par);
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn leo_remove(op: *mut platform_device) {
    static void leo_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct leo_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    leo_unmap_regs(op, info, par);
    framebuffer_release(info);
    }
    static const struct of_device_id leo_match[] = {
    {
    .name = "SUNW,leo",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, leo_match);
    static struct platform_driver leo_driver = {
    .driver = {
    .name = "leo",
    .of_match_table = leo_match,
    },
    .probe		= leo_probe,
    .remove		= leo_remove,
    };
#[no_mangle]
unsafe extern "C" fn leo_init() -> int __init {
    static int __init leo_init(void)
    {
    if (fb_get_options("leofb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&leo_driver);
    }
#[no_mangle]
unsafe extern "C" fn leo_exit() -> void __exit {
    static void __exit leo_exit(void)
    {
    platform_driver_unregister(&leo_driver);
    }
    module_init(leo_init);
    module_exit(leo_exit);
    MODULE_DESCRIPTION("framebuffer driver for LEO chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
