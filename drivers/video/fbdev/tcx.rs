//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/tcx.c
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
// tcx.c: TCX frame buffer driver
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
    static int tcx_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int tcx_blank(int, struct fb_info *);
    static int tcx_pan_display(struct fb_var_screeninfo *, struct fb_info *);
    static int tcx_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int tcx_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops tcx_ops = {
    .owner			= THIS_MODULE,
    FB_DEFAULT_SBUS_OPS(tcx),
    .fb_setcolreg		= tcx_setcolreg,
    .fb_blank		= tcx_blank,
    .fb_pan_display		= tcx_pan_display,
    };
// THC definitions
pub const TCX_THC_MISC_REV_SHIFT: c_int = 16;
pub const TCX_THC_MISC_REV_MASK: c_int = 15;

pub const TCX_THC_MISC_INIT: c_uint = 0x9f;
pub const TCX_THC_REV_REV_SHIFT: c_int = 20;
pub const TCX_THC_REV_REV_MASK: c_int = 15;
pub const TCX_THC_REV_MINREV_SHIFT: c_int = 28;
pub const TCX_THC_REV_MINREV_MASK: c_int = 15;
// The contents are unknown
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcx_tec {
    pub tec_matrix: u32,
    pub tec_clip: u32,
    pub tec_vdc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcx_thc {
    pub thc_rev: u32,
    pub thc_pad0: [u32; 511],
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
pub struct bt_regs {
    pub addr: u32,
    pub color_map: u32,
    pub control: u32,
    pub cursor: u32,
}

pub const TCX_MMAP_ENTRIES: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcx_par {
    pub lock: spinlock_t,
    pub bt: *mut bt_regs __iomem,
    pub thc: *mut tcx_thc __iomem,
    pub tec: *mut tcx_tec __iomem,
    pub cplane: *mut u32 __iomem,
    pub flags: u32,
pub const TCX_FLAG_BLANKED: c_uint = 0x00000001;
    pub which_io: c_ulong,
    pub mmap_map: [sbus_mmap_map; TCX_MMAP_ENTRIES],
    pub lowdepth: c_int,
}

// Reset control plane so that WID is 8-bit plane.
#[no_mangle]
unsafe extern "C" fn __tcx_set_control_plane(info: *mut fb_info) {
    static void __tcx_set_control_plane(struct fb_info *info)
    {
    struct tcx_par *par = info.par;
    u32 __iomem *p, *pend;
    if (par.lowdepth)
    return;
    p = par.cplane;
    if (p == core::ptr::null_mut())
    return;
    for (pend = p + info.fix.smem_len; p < pend; p++) {
    let mut tmp: u32 = sbus_readl(p);
    tmp &= 0xffffff;
    sbus_writel(tmp, p);
    }
    }
#[no_mangle]
unsafe extern "C" fn tcx_reset(info: *mut fb_info) {
    static void tcx_reset(struct fb_info *info)
    {
    struct tcx_par *par = (struct tcx_par *) info.par;
    unsigned long flags;
    spin_lock_irqsave(&par.lock, flags);
    __tcx_set_control_plane(info);
    spin_unlock_irqrestore(&par.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn tcx_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int tcx_pan_display(struct fb_var_screeninfo *var, struct fb_info *info)
    {
    tcx_reset(info);
    return 0;
    }
//
// tcx_setcolreg - Optional function. Sets a color register.
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
    static int tcx_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    struct tcx_par *par = (struct tcx_par *) info.par;
    struct bt_regs __iomem *bt = par.bt;
    unsigned long flags;
    if (regno >= 256)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    spin_lock_irqsave(&par.lock, flags);
    sbus_writel(regno << 24, &bt.addr);
    sbus_writel(red << 24, &bt.color_map);
    sbus_writel(green << 24, &bt.color_map);
    sbus_writel(blue << 24, &bt.color_map);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
//
// tcx_blank - Optional function.  Blanks the display.
// @blank: the blank mode we want.
// @info: frame buffer structure that represents a single frame buffer
//
    static int
    tcx_blank(int blank, struct fb_info *info)
    {
    struct tcx_par *par = (struct tcx_par *) info.par;
    struct tcx_thc __iomem *thc = par.thc;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&par.lock, flags);
    val = sbus_readl(&thc.thc_misc);
    switch (blank) {
    case FB_BLANK_UNBLANK: /* Unblanking */
    val &= ~(TCX_THC_MISC_VSYNC_DIS |
    TCX_THC_MISC_HSYNC_DIS);
    val |= TCX_THC_MISC_VIDEO;
    par.flags &= ~TCX_FLAG_BLANKED;
    break;
    case FB_BLANK_NORMAL: /* Normal blanking */
    val &= ~TCX_THC_MISC_VIDEO;
    par.flags |= TCX_FLAG_BLANKED;
    break;
    case FB_BLANK_VSYNC_SUSPEND: /* VESA blank (vsync off) */
    val |= TCX_THC_MISC_VSYNC_DIS;
    break;
    case FB_BLANK_HSYNC_SUSPEND: /* VESA blank (hsync off) */
    val |= TCX_THC_MISC_HSYNC_DIS;
    break;
    case FB_BLANK_POWERDOWN: /* Poweroff */
    break;
    }
    sbus_writel(val, &thc.thc_misc);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
    static const struct sbus_mmap_map __tcx_mmap_map[TCX_MMAP_ENTRIES] = {
    {
    .voff	= TCX_RAM8BIT,
    .size	= SBUS_MMAP_FBSIZE(1)
    },
    {
    .voff	= TCX_RAM24BIT,
    .size	= SBUS_MMAP_FBSIZE(4)
    },
    {
    .voff	= TCX_UNK3,
    .size	= SBUS_MMAP_FBSIZE(8)
    },
    {
    .voff	= TCX_UNK4,
    .size	= SBUS_MMAP_FBSIZE(8)
    },
    {
    .voff	= TCX_CONTROLPLANE,
    .size	= SBUS_MMAP_FBSIZE(4)
    },
    {
    .voff	= TCX_UNK6,
    .size	= SBUS_MMAP_FBSIZE(8)
    },
    {
    .voff	= TCX_UNK7,
    .size	= SBUS_MMAP_FBSIZE(8)
    },
    {
    .voff	= TCX_TEC,
    .size	= PAGE_SIZE
    },
    {
    .voff	= TCX_BTREGS,
    .size	= PAGE_SIZE
    },
    {
    .voff	= TCX_THC,
    .size	= PAGE_SIZE
    },
    {
    .voff	= TCX_DHC,
    .size	= PAGE_SIZE
    },
    {
    .voff	= TCX_ALT,
    .size	= PAGE_SIZE
    },
    {
    .voff	= TCX_UNK2,
    .size	= 0x20000
    },
    { .size = 0 }
    };
#[no_mangle]
unsafe extern "C" fn tcx_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int tcx_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct tcx_par *par = (struct tcx_par *)info.par;
    return sbusfb_mmap_helper(par.mmap_map,
    info.fix.smem_start, info.fix.smem_len,
    par.which_io, vma);
    }
#[no_mangle]
unsafe extern "C" fn tcx_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int tcx_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    struct tcx_par *par = (struct tcx_par *) info.par;
    return sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_TCXCOLOR,
    (par.lowdepth ? 8 : 24),
    info.fix.smem_len);
    }
//
// Initialisation
//
    static void
    tcx_init_fix(struct fb_info *info, int linebytes)
    {
    struct tcx_par *par = (struct tcx_par *)info.par;
    const char *tcx_name;
    if (par.lowdepth)
    tcx_name = "TCX8";
    else
    tcx_name = "TCX24";
    strscpy(info.fix.id, tcx_name, sizeof(info.fix.id));
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.fix.line_length = linebytes;
    info.fix.accel = FB_ACCEL_SUN_TCX;
    }
    static void tcx_unmap_regs(struct platform_device *op, struct fb_info *info,
    struct tcx_par *par)
    {
    if (par.tec)
    of_iounmap(&op.resource[7],
    par.tec, sizeof(struct tcx_tec));
    if (par.thc)
    of_iounmap(&op.resource[9],
    par.thc, sizeof(struct tcx_thc));
    if (par.bt)
    of_iounmap(&op.resource[8],
    par.bt, sizeof(struct bt_regs));
    if (par.cplane)
    of_iounmap(&op.resource[4],
    par.cplane, info.fix.smem_len * sizeof(u32));
    if (info.screen_base)
    of_iounmap(&op.resource[0],
    info.screen_base, info.fix.smem_len);
    }
#[no_mangle]
unsafe extern "C" fn tcx_probe(op: *mut platform_device) -> c_int {
    static int tcx_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct fb_info *info;
    struct tcx_par *par;
    int linebytes, i, err;
    info = framebuffer_alloc(sizeof(struct tcx_par), &op.dev);
    err = -ENOMEM;
    if (!info)
    goto out_err;
    par = info.par;
    spin_lock_init(&par.lock);
    par.lowdepth = of_property_read_bool(dp, "tcx-8-bit");
    sbusfb_fill_var(&info.var, dp, 8);
    info.var.red.length = 8;
    info.var.green.length = 8;
    info.var.blue.length = 8;
    linebytes = of_getintprop_default(dp, "linebytes",
    info.var.xres);
    info.fix.smem_len = PAGE_ALIGN(linebytes * info.var.yres);
    par.tec = of_ioremap(&op.resource[7], 0,
    sizeof(struct tcx_tec), "tcx tec");
    par.thc = of_ioremap(&op.resource[9], 0,
    sizeof(struct tcx_thc), "tcx thc");
    par.bt = of_ioremap(&op.resource[8], 0,
    sizeof(struct bt_regs), "tcx dac");
    info.screen_base = of_ioremap(&op.resource[0], 0,
    info.fix.smem_len, "tcx ram");
    if (!par.tec || !par.thc ||
    !par.bt || !info.screen_base)
    goto out_unmap_regs;
    memcpy(&par.mmap_map, &__tcx_mmap_map, sizeof(par.mmap_map));
    if (!par.lowdepth) {
    par.cplane = of_ioremap(&op.resource[4], 0,
    info.fix.smem_len * sizeof(u32),
    "tcx cplane");
    if (!par.cplane)
    goto out_unmap_regs;
    } else {
    par.mmap_map[1].size = SBUS_MMAP_EMPTY;
    par.mmap_map[4].size = SBUS_MMAP_EMPTY;
    par.mmap_map[5].size = SBUS_MMAP_EMPTY;
    par.mmap_map[6].size = SBUS_MMAP_EMPTY;
    }
    info.fix.smem_start = op.resource[0].start;
    par.which_io = op.resource[0].flags & IORESOURCE_BITS;
    for (i = 0; i < TCX_MMAP_ENTRIES; i++) {
    int j;
    switch (i) {
    case 10:
    j = 12;
    break;
    case 11: case 12:
    j = i - 1;
    break;
    default:
    j = i;
    break;
    }
    par.mmap_map[i].poff = op.resource[j].start - info.fix.smem_start;
    }
    info.fbops = &tcx_ops;
// Initialize brooktree DAC.
    sbus_writel(0x04 << 24, &par.bt.addr);         /* color planes */
    sbus_writel(0xff << 24, &par.bt.control);
    sbus_writel(0x05 << 24, &par.bt.addr);
    sbus_writel(0x00 << 24, &par.bt.control);
    sbus_writel(0x06 << 24, &par.bt.addr);         /* overlay plane */
    sbus_writel(0x73 << 24, &par.bt.control);
    sbus_writel(0x07 << 24, &par.bt.addr);
    sbus_writel(0x00 << 24, &par.bt.control);
    tcx_reset(info);
    tcx_blank(FB_BLANK_UNBLANK, info);
    if (fb_alloc_cmap(&info.cmap, 256, 0))
    goto out_unmap_regs;
    fb_set_cmap(&info.cmap, info);
    tcx_init_fix(info, linebytes);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: TCX at %lx:%lx, %s\n",
    dp,
    par.which_io,
    info.fix.smem_start,
    par.lowdepth ? "8-bit only" : "24-bit depth");
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_regs:
    tcx_unmap_regs(op, info, par);
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tcx_remove(op: *mut platform_device) {
    static void tcx_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct tcx_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    tcx_unmap_regs(op, info, par);
    framebuffer_release(info);
    }
    static const struct of_device_id tcx_match[] = {
    {
    .name = "SUNW,tcx",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, tcx_match);
    static struct platform_driver tcx_driver = {
    .driver = {
    .name = "tcx",
    .of_match_table = tcx_match,
    },
    .probe		= tcx_probe,
    .remove		= tcx_remove,
    };
#[no_mangle]
unsafe extern "C" fn tcx_init() -> int __init {
    static int __init tcx_init(void)
    {
    if (fb_get_options("tcxfb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&tcx_driver);
    }
#[no_mangle]
unsafe extern "C" fn tcx_exit() -> void __exit {
    static void __exit tcx_exit(void)
    {
    platform_driver_unregister(&tcx_driver);
    }
    module_init(tcx_init);
    module_exit(tcx_exit);
    MODULE_DESCRIPTION("framebuffer driver for TCX chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
