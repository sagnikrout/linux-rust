//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/cg14.c
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
// cg14.c: CGFOURTEEN frame buffer driver
//
// Copyright (C) 2003, 2006 David S. Miller (davem@davemloft.net)
// Copyright (C) 1996,1998 Jakub Jelinek (jj@ultra.linux.cz)
// Copyright (C) 1995 Miguel de Icaza (miguel@nuclecu.unam.mx)
//
// Driver layout based loosely on tgafb.c, see that file for credits.
//

//
// Local functions.
//
    static int cg14_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int cg14_pan_display(struct fb_var_screeninfo *, struct fb_info *);
    static int cg14_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int cg14_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops cg14_ops = {
    .owner			= THIS_MODULE,
    FB_DEFAULT_SBUS_OPS(cg14),
    .fb_setcolreg		= cg14_setcolreg,
    .fb_pan_display		= cg14_pan_display,
    };
pub const CG14_MCR_INTENABLE_SHIFT: c_int = 7;
pub const CG14_MCR_INTENABLE_MASK: c_uint = 0x80;
pub const CG14_MCR_VIDENABLE_SHIFT: c_int = 6;
pub const CG14_MCR_VIDENABLE_MASK: c_uint = 0x40;
pub const CG14_MCR_PIXMODE_SHIFT: c_int = 4;
pub const CG14_MCR_PIXMODE_MASK: c_uint = 0x30;
pub const CG14_MCR_TMR_SHIFT: c_int = 2;
pub const CG14_MCR_TMR_MASK: c_uint = 0x0c;
pub const CG14_MCR_TMENABLE_SHIFT: c_int = 1;
pub const CG14_MCR_TMENABLE_MASK: c_uint = 0x02;
pub const CG14_MCR_RESET_SHIFT: c_int = 0;
pub const CG14_MCR_RESET_MASK: c_uint = 0x01;
pub const CG14_REV_REVISION_SHIFT: c_int = 4;
pub const CG14_REV_REVISION_MASK: c_uint = 0xf0;
pub const CG14_REV_IMPL_SHIFT: c_int = 0;
pub const CG14_REV_IMPL_MASK: c_uint = 0x0f;
pub const CG14_VBR_FRAMEBASE_SHIFT: c_int = 12;
pub const CG14_VBR_FRAMEBASE_MASK: c_uint = 0x00fff000;
pub const CG14_VMCR1_SETUP_SHIFT: c_int = 0;
pub const CG14_VMCR1_SETUP_MASK: c_uint = 0x000001ff;
pub const CG14_VMCR1_VCONFIG_SHIFT: c_int = 9;
pub const CG14_VMCR1_VCONFIG_MASK: c_uint = 0x00000e00;
pub const CG14_VMCR2_REFRESH_SHIFT: c_int = 0;
pub const CG14_VMCR2_REFRESH_MASK: c_uint = 0x00000001;
pub const CG14_VMCR2_TESTROWCNT_SHIFT: c_int = 1;
pub const CG14_VMCR2_TESTROWCNT_MASK: c_uint = 0x00000002;
pub const CG14_VMCR2_FBCONFIG_SHIFT: c_int = 2;
pub const CG14_VMCR2_FBCONFIG_MASK: c_uint = 0x0000000c;
pub const CG14_VCR_REFRESHREQ_SHIFT: c_int = 0;
pub const CG14_VCR_REFRESHREQ_MASK: c_uint = 0x000003ff;
pub const CG14_VCR1_REFRESHENA_SHIFT: c_int = 10;
pub const CG14_VCR1_REFRESHENA_MASK: c_uint = 0x00000400;
pub const CG14_VCA_CAD_SHIFT: c_int = 0;
pub const CG14_VCA_CAD_MASK: c_uint = 0x000003ff;
pub const CG14_VCA_VERS_SHIFT: c_int = 10;
pub const CG14_VCA_VERS_MASK: c_uint = 0x00000c00;
pub const CG14_VCA_RAMSPEED_SHIFT: c_int = 12;
pub const CG14_VCA_RAMSPEED_MASK: c_uint = 0x00001000;
pub const CG14_VCA_8MB_SHIFT: c_int = 13;
pub const CG14_VCA_8MB_MASK: c_uint = 0x00002000;
pub const CG14_MCR_PIXMODE_8: c_int = 0;
pub const CG14_MCR_PIXMODE_16: c_int = 2;
pub const CG14_MCR_PIXMODE_32: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_regs {
    pub /: *mut *mut u8 mcr; / Master Control Reg,
    pub /: *mut *mut u8 ppr; / Packed Pixel Reg,
    pub /: *mut *mut u8 tms[2]; / Test Mode Status Regs,
    pub /: *mut *mut u8 msr; / Master Status Reg,
    pub /: *mut *mut u8 fsr; / Fault Status Reg,
    pub /: *mut *mut u8 rev; / Revision & Impl,
    pub /: *mut *mut u8 ccr; / Clock Control Reg,
    pub /: *mut *mut u32 tmr; / Test Mode Read Back,
    pub /: *mut *mut u8 mod; / Monitor Operation Data Reg,
    pub /: *mut *mut u8 acr; / Aux Control,
    pub xxx0: [u8; 6],
    pub /: *mut *mut u16 hct; / Hor Counter,
    pub /: *mut *mut u16 vct; / Vert Counter,
    pub /: *mut *mut u16 hbs; / Hor Blank Start,
    pub /: *mut *mut u16 hbc; / Hor Blank Clear,
    pub /: *mut *mut u16 hss; / Hor Sync Start,
    pub /: *mut *mut u16 hsc; / Hor Sync Clear,
    pub /: *mut *mut u16 csc; / Composite Sync Clear,
    pub /: *mut *mut u16 vbs; / Vert Blank Start,
    pub /: *mut *mut u16 vbc; / Vert Blank Clear,
    pub /: *mut *mut u16 vss; / Vert Sync Start,
    pub /: *mut *mut u16 vsc; / Vert Sync Clear,
    pub xcs: u16,
    pub xcc: u16,
    pub /: *mut *mut u16 fsa; / Fault Status Address,
    pub /: *mut *mut u16 adr; / Address Registers,
    pub xxx1: [u8; 0xce],
    pub /: *mut *mut u8 pcg[0x100]; / Pixel Clock Generator,
    pub /: *mut *mut u32 vbr; / Frame Base Row,
    pub /: *mut *mut u32 vmcr; / VBC Master Control,
    pub /: *mut *mut u32 vcr; / VBC refresh,
    pub /: *mut *mut u32 vca; / VBC Config,
}

pub const CG14_CCR_ENABLE: c_uint = 0x04;
pub const CG14_CCR_SELECT: c_uint = 0x02	/* HW/Full screen */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_cursor {
    pub /: *mut *mut u32 cpl0[32]; / Enable plane 0,
    pub /: *mut *mut u32 cpl1[32]; / Color selection plane,
    pub /: *mut *mut u8 ccr; / Cursor Control Reg,
    pub xxx0: [u8; 3],
    pub /: *mut *mut u16 cursx; / Cursor x,y position,
    pub /: *mut *mut u16 cursy; / Cursor x,y position,
    pub color0: u32,
    pub color1: u32,
    pub xxx1: [u32; 0x1bc],
    pub /: *mut *mut u32 cpl0i[32]; / Enable plane 0 autoinc,
    pub /: *mut *mut u32 cpl1i[32]; / Color selection autoinc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_dac {
    pub /: *mut *mut u8 addr; / Address Register,
    pub xxx0: [u8; 255],
    pub /: *mut *mut u8 glut; / Gamma table,
    pub xxx1: [u8; 255],
    pub /: *mut *mut u8 select; / Register Select,
    pub xxx2: [u8; 255],
    pub /: *mut *mut u8 mode; / Mode Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_xlut {
    pub [256]: u8 x_xlut,
    pub [256]: u8 x_xlutd,
    pub xxx0: [u8; 0x600],
    pub [256]: u8 x_xlut_inc,
    pub [256]: u8 x_xlutd_inc,
}

// Color look up table (clut)
// Each one of these arrays hold the color lookup table (for 256
// colors) for each MDI page (I assume then there should be 4 MDI
// pages, I still wonder what they are.  I have seen NeXTStep split
// the screen in four parts, while operating in 24 bits mode.  Each
// integer holds 4 values: alpha value (transparency channel, thanks
// go to John Stone (johns@umr.edu) from OpenBSD), red, green and blue
//
// I currently use the clut instead of the Xlut
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_clut {
    pub [256]: u32 c_clut,
    pub /: *mut *mut u32 c_clutd [256]; / i wonder what the 'd' is for,
    pub [256]: u32 c_clut_inc,
    pub [256]: u32 c_clutd_inc,
}

pub const CG14_MMAP_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg14_par {
    pub lock: spinlock_t,
    pub regs: *mut cg14_regs __iomem,
    pub clut: *mut cg14_clut __iomem,
    pub cursor: *mut cg14_cursor __iomem,
    pub flags: u32,
pub const CG14_FLAG_BLANKED: c_uint = 0x00000001;
    pub iospace: c_ulong,
    pub mmap_map: [sbus_mmap_map; CG14_MMAP_ENTRIES],
    pub mode: c_int,
    pub ramsize: c_int,
}

#[no_mangle]
unsafe extern "C" fn __cg14_reset(par: *mut cg14_par) {
    static void __cg14_reset(struct cg14_par *par)
    {
    struct cg14_regs __iomem *regs = par.regs;
    u8 val;
    val = sbus_readb(&regs.mcr);
    val &= ~(CG14_MCR_PIXMODE_MASK);
    sbus_writeb(val, &regs.mcr);
    }
#[no_mangle]
unsafe extern "C" fn cg14_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int cg14_pan_display(struct fb_var_screeninfo *var, struct fb_info *info)
    {
    struct cg14_par *par = (struct cg14_par *) info.par;
    unsigned long flags;
// We just use this to catch switches out of
// graphics mode.
//
    spin_lock_irqsave(&par.lock, flags);
    __cg14_reset(par);
    spin_unlock_irqrestore(&par.lock, flags);
    if (var.xoffset || var.yoffset || var.vmode)
    return -EINVAL;
    return 0;
    }
//
// cg14_setcolreg - Optional function. Sets a color register.
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
    static int cg14_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    struct cg14_par *par = (struct cg14_par *) info.par;
    struct cg14_clut __iomem *clut = par.clut;
    unsigned long flags;
    u32 val;
    if (regno >= 256)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    val = (red | (green << 8) | (blue << 16));
    spin_lock_irqsave(&par.lock, flags);
    sbus_writel(val, &clut.c_clut[regno]);
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cg14_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int cg14_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct cg14_par *par = (struct cg14_par *) info.par;
    return sbusfb_mmap_helper(par.mmap_map,
    info.fix.smem_start, info.fix.smem_len,
    par.iospace, vma);
    }
#[no_mangle]
unsafe extern "C" fn cg14_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int cg14_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    struct cg14_par *par = (struct cg14_par *) info.par;
    struct cg14_regs __iomem *regs = par.regs;
    struct mdi_cfginfo kmdi, __user *mdii;
    unsigned long flags;
    int cur_mode, mode, ret = 0;
    switch (cmd) {
    case MDI_RESET:
    spin_lock_irqsave(&par.lock, flags);
    __cg14_reset(par);
    spin_unlock_irqrestore(&par.lock, flags);
    break;
    case MDI_GET_CFGINFO:
    memset(&kmdi, 0, sizeof(kmdi));
    spin_lock_irqsave(&par.lock, flags);
    kmdi.mdi_type = FBTYPE_MDICOLOR;
    kmdi.mdi_height = info.var.yres;
    kmdi.mdi_width = info.var.xres;
    kmdi.mdi_mode = par.mode;
    kmdi.mdi_pixfreq = 72; /* FIXME */
    kmdi.mdi_size = par.ramsize;
    spin_unlock_irqrestore(&par.lock, flags);
    mdii = (struct mdi_cfginfo __user *) arg;
    if (copy_to_user(mdii, &kmdi, sizeof(kmdi)))
    ret = -EFAULT;
    break;
    case MDI_SET_PIXELMODE:
    if (get_user(mode, (int __user *) arg)) {
    ret = -EFAULT;
    break;
    }
    spin_lock_irqsave(&par.lock, flags);
    cur_mode = sbus_readb(&regs.mcr);
    cur_mode &= ~CG14_MCR_PIXMODE_MASK;
    switch(mode) {
    case MDI_32_PIX:
    cur_mode |= (CG14_MCR_PIXMODE_32 <<
    CG14_MCR_PIXMODE_SHIFT);
    break;
    case MDI_16_PIX:
    cur_mode |= (CG14_MCR_PIXMODE_16 <<
    CG14_MCR_PIXMODE_SHIFT);
    break;
    case MDI_8_PIX:
    break;
    default:
    ret = -ENOSYS;
    break;
    }
    if (!ret) {
    sbus_writeb(cur_mode, &regs.mcr);
    par.mode = mode;
    }
    spin_unlock_irqrestore(&par.lock, flags);
    break;
    default:
    ret = sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_MDICOLOR, 8,
    info.fix.smem_len);
    break;
    }
    return ret;
    }
//
// Initialisation
//
    static void cg14_init_fix(struct fb_info *info, int linebytes,
    struct device_node *dp)
    {
    snprintf(info.fix.id, sizeof(info.fix.id), "%pOFn", dp);
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.fix.line_length = linebytes;
    info.fix.accel = FB_ACCEL_SUN_CG14;
    }
    static const struct sbus_mmap_map __cg14_mmap_map[CG14_MMAP_ENTRIES] = {
    {
    .voff	= CG14_REGS,
    .poff	= 0x80000000,
    .size	= 0x1000
    },
    {
    .voff	= CG14_XLUT,
    .poff	= 0x80003000,
    .size	= 0x1000
    },
    {
    .voff	= CG14_CLUT1,
    .poff	= 0x80004000,
    .size	= 0x1000
    },
    {
    .voff	= CG14_CLUT2,
    .poff	= 0x80005000,
    .size	= 0x1000
    },
    {
    .voff	= CG14_CLUT3,
    .poff	= 0x80006000,
    .size	= 0x1000
    },
    {
    .voff	= CG3_MMAP_OFFSET - 0x7000,
    .poff	= 0x80000000,
    .size	= 0x7000
    },
    {
    .voff	= CG3_MMAP_OFFSET,
    .poff	= 0x00000000,
    .size	= SBUS_MMAP_FBSIZE(1)
    },
    {
    .voff	= MDI_CURSOR_MAP,
    .poff	= 0x80001000,
    .size	= 0x1000
    },
    {
    .voff	= MDI_CHUNKY_BGR_MAP,
    .poff	= 0x01000000,
    .size	= 0x400000
    },
    {
    .voff	= MDI_PLANAR_X16_MAP,
    .poff	= 0x02000000,
    .size	= 0x200000
    },
    {
    .voff	= MDI_PLANAR_C16_MAP,
    .poff	= 0x02800000,
    .size	= 0x200000
    },
    {
    .voff	= MDI_PLANAR_X32_MAP,
    .poff	= 0x03000000,
    .size	= 0x100000
    },
    {
    .voff	= MDI_PLANAR_B32_MAP,
    .poff	= 0x03400000,
    .size	= 0x100000
    },
    {
    .voff	= MDI_PLANAR_G32_MAP,
    .poff	= 0x03800000,
    .size	= 0x100000
    },
    {
    .voff	= MDI_PLANAR_R32_MAP,
    .poff	= 0x03c00000,
    .size	= 0x100000
    },
    { .size = 0 }
    };
    static void cg14_unmap_regs(struct platform_device *op, struct fb_info *info,
    struct cg14_par *par)
    {
    if (par.regs)
    of_iounmap(&op.resource[0],
    par.regs, sizeof(struct cg14_regs));
    if (par.clut)
    of_iounmap(&op.resource[0],
    par.clut, sizeof(struct cg14_clut));
    if (par.cursor)
    of_iounmap(&op.resource[0],
    par.cursor, sizeof(struct cg14_cursor));
    if (info.screen_base)
    of_iounmap(&op.resource[1],
    info.screen_base, info.fix.smem_len);
    }
#[no_mangle]
unsafe extern "C" fn cg14_probe(op: *mut platform_device) -> c_int {
    static int cg14_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct fb_info *info;
    struct cg14_par *par;
    int is_8mb, linebytes, i, err;
    info = framebuffer_alloc(sizeof(struct cg14_par), &op.dev);
    err = -ENOMEM;
    if (!info)
    goto out_err;
    par = info.par;
    spin_lock_init(&par.lock);
    sbusfb_fill_var(&info.var, dp, 8);
    info.var.red.length = 8;
    info.var.green.length = 8;
    info.var.blue.length = 8;
    linebytes = of_getintprop_default(dp, "linebytes",
    info.var.xres);
    info.fix.smem_len = PAGE_ALIGN(linebytes * info.var.yres);
    if (of_node_name_eq(dp.parent, "sbus") ||
    of_node_name_eq(dp.parent, "sbi")) {
    info.fix.smem_start = op.resource[0].start;
    par.iospace = op.resource[0].flags & IORESOURCE_BITS;
    } else {
    info.fix.smem_start = op.resource[1].start;
    par.iospace = op.resource[0].flags & IORESOURCE_BITS;
    }
    par.regs = of_ioremap(&op.resource[0], 0,
    sizeof(struct cg14_regs), "cg14 regs");
    par.clut = of_ioremap(&op.resource[0], CG14_CLUT1,
    sizeof(struct cg14_clut), "cg14 clut");
    par.cursor = of_ioremap(&op.resource[0], CG14_CURSORREGS,
    sizeof(struct cg14_cursor), "cg14 cursor");
    info.screen_base = of_ioremap(&op.resource[1], 0,
    info.fix.smem_len, "cg14 ram");
    if (!par.regs || !par.clut || !par.cursor || !info.screen_base)
    goto out_unmap_regs;
    is_8mb = (resource_size(&op.resource[1]) == (8 * 1024 * 1024));
    BUILD_BUG_ON(sizeof(par.mmap_map) != sizeof(__cg14_mmap_map));
    memcpy(&par.mmap_map, &__cg14_mmap_map, sizeof(par.mmap_map));
    for (i = 0; i < CG14_MMAP_ENTRIES; i++) {
    struct sbus_mmap_map *map = &par.mmap_map[i];
    if (!map.size)
    break;
    if (map.poff & 0x80000000)
    map.poff = (map.poff & 0x7fffffff) +
    (op.resource[0].start -
    op.resource[1].start);
    if (is_8mb &&
    map.size >= 0x100000 &&
    map.size <= 0x400000)
    map.size *= 2;
    }
    par.mode = MDI_8_PIX;
    par.ramsize = (is_8mb ? 0x800000 : 0x400000);
    info.flags = FBINFO_HWACCEL_YPAN;
    info.fbops = &cg14_ops;
    __cg14_reset(par);
    if (fb_alloc_cmap(&info.cmap, 256, 0))
    goto out_unmap_regs;
    fb_set_cmap(&info.cmap, info);
    cg14_init_fix(info, linebytes, dp);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: cgfourteen at %lx:%lx, %dMB\n",
    dp,
    par.iospace, info.fix.smem_start,
    par.ramsize >> 20);
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_regs:
    cg14_unmap_regs(op, info, par);
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cg14_remove(op: *mut platform_device) {
    static void cg14_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct cg14_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    cg14_unmap_regs(op, info, par);
    framebuffer_release(info);
    }
    static const struct of_device_id cg14_match[] = {
    {
    .name = "cgfourteen",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cg14_match);
    static struct platform_driver cg14_driver = {
    .driver = {
    .name = "cg14",
    .of_match_table = cg14_match,
    },
    .probe		= cg14_probe,
    .remove		= cg14_remove,
    };
#[no_mangle]
unsafe extern "C" fn cg14_init() -> int __init {
    static int __init cg14_init(void)
    {
    if (fb_get_options("cg14fb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&cg14_driver);
    }
#[no_mangle]
unsafe extern "C" fn cg14_exit() -> void __exit {
    static void __exit cg14_exit(void)
    {
    platform_driver_unregister(&cg14_driver);
    }
    module_init(cg14_init);
    module_exit(cg14_exit);
    MODULE_DESCRIPTION("framebuffer driver for CGfourteen chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
