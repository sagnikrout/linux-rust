//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/cg3.c
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
// cg3.c: CGTHREE frame buffer driver
//
// Copyright (C) 2003, 2006 David S. Miller (davem@davemloft.net)
// Copyright (C) 1996,1998 Jakub Jelinek (jj@ultra.linux.cz)
// Copyright (C) 1996 Miguel de Icaza (miguel@nuclecu.unam.mx)
// Copyright (C) 1997 Eddie C. Dost (ecd@skynet.be)
//
// Driver layout based loosely on tgafb.c, see that file for credits.
//

//
// Local functions.
//
    static int cg3_setcolreg(unsigned, unsigned, unsigned, unsigned,
    unsigned, struct fb_info *);
    static int cg3_blank(int, struct fb_info *);
    static int cg3_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma);
    static int cg3_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg);
//
// Frame buffer operations
//
    static const struct fb_ops cg3_ops = {
    .owner			= THIS_MODULE,
    FB_DEFAULT_SBUS_OPS(cg3),
    .fb_setcolreg		= cg3_setcolreg,
    .fb_blank		= cg3_blank,
    };
// Control Register Constants
pub const CG3_CR_ENABLE_INTS: c_uint = 0x80;
pub const CG3_CR_ENABLE_VIDEO: c_uint = 0x40;
pub const CG3_CR_ENABLE_TIMING: c_uint = 0x20;
pub const CG3_CR_ENABLE_CURCMP: c_uint = 0x10;
pub const CG3_CR_XTAL_MASK: c_uint = 0x0c;
pub const CG3_CR_DIVISOR_MASK: c_uint = 0x03;
// Status Register Constants
pub const CG3_SR_PENDING_INT: c_uint = 0x80;
pub const CG3_SR_RES_MASK: c_uint = 0x70;
pub const CG3_SR_1152_900_76_A: c_uint = 0x40;
pub const CG3_SR_1152_900_76_B: c_uint = 0x60;
pub const CG3_SR_ID_MASK: c_uint = 0x0f;
pub const CG3_SR_ID_COLOR: c_uint = 0x01;
pub const CG3_SR_ID_MONO: c_uint = 0x02;
pub const CG3_SR_ID_MONO_ECL: c_uint = 0x03;
    enum cg3_type {
    CG3_AT_66HZ = 0,
    CG3_AT_76HZ,
    CG3_RDI
    };
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
pub struct cg3_regs {
    pub cmap: bt_regs,
    pub control: u8,
    pub status: u8,
    pub cursor_start: u8,
    pub cursor_end: u8,
    pub h_blank_start: u8,
    pub h_blank_end: u8,
    pub h_sync_start: u8,
    pub h_sync_end: u8,
    pub comp_sync_end: u8,
    pub v_blank_start_high: u8,
    pub v_blank_start_low: u8,
    pub v_blank_end: u8,
    pub v_sync_start: u8,
    pub v_sync_end: u8,
    pub xfer_holdoff_start: u8,
    pub xfer_holdoff_end: u8,
}

// Offset of interesting structures in the OBIO space
pub const CG3_REGS_OFFSET: c_uint = 0x400000UL;
pub const CG3_RAM_OFFSET: c_uint = 0x800000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg3_par {
    pub lock: spinlock_t,
    pub regs: *mut cg3_regs __iomem,
    pub 4]: *mut *mut u32 sw_cmap[((256  3) + 3) /,
    pub flags: u32,
pub const CG3_FLAG_BLANKED: c_uint = 0x00000001;
pub const CG3_FLAG_RDI: c_uint = 0x00000002;
    pub which_io: c_ulong,
}

//
// cg3_setcolreg - Optional function. Sets a color register.
// @regno: boolean, 0 copy local, 1 get_user() function
// @red: frame buffer colormap structure
// @green: The green value which can be up to 16 bits wide
// @blue:  The blue value which can be up to 16 bits wide.
// @transp: If supported the alpha value which can be up to 16 bits wide.
// @info: frame buffer info structure
//
// The cg3 palette is loaded with 4 color values at each time
// so you end up with: (rgb)(r), (gb)(rg), (b)(rgb), and so on.
// We keep a sw copy of the hw cmap to assist us in this esoteric
// loading procedure.
//
    static int cg3_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    struct cg3_par *par = (struct cg3_par *) info.par;
    struct bt_regs __iomem *bt = &par.regs.cmap;
    unsigned long flags;
    u32 *p32;
    u8 *p8;
    int count;
    if (regno >= 256)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    spin_lock_irqsave(&par.lock, flags);
    p8 = (u8 *)par.sw_cmap + (regno * 3);
    p8[0] = red;
    p8[1] = green;
    p8[2] = blue;

    count = 3;
    p32 = &par.sw_cmap[D4M3(regno)];
    sbus_writel(D4M4(regno), &bt.addr);
    while (count--)
    sbus_writel(*p32++, &bt.color_map);

    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
//
// cg3_blank - Optional function.  Blanks the display.
// @blank: the blank mode we want.
// @info: frame buffer structure that represents a single frame buffer
//
#[no_mangle]
unsafe extern "C" fn cg3_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int cg3_blank(int blank, struct fb_info *info)
    {
    struct cg3_par *par = (struct cg3_par *) info.par;
    struct cg3_regs __iomem *regs = par.regs;
    unsigned long flags;
    u8 val;
    spin_lock_irqsave(&par.lock, flags);
    switch (blank) {
    case FB_BLANK_UNBLANK: /* Unblanking */
    val = sbus_readb(&regs.control);
    val |= CG3_CR_ENABLE_VIDEO;
    sbus_writeb(val, &regs.control);
    par.flags &= ~CG3_FLAG_BLANKED;
    break;
    case FB_BLANK_NORMAL: /* Normal blanking */
    case FB_BLANK_VSYNC_SUSPEND: /* VESA blank (vsync off) */
    case FB_BLANK_HSYNC_SUSPEND: /* VESA blank (hsync off) */
    case FB_BLANK_POWERDOWN: /* Poweroff */
    val = sbus_readb(&regs.control);
    val &= ~CG3_CR_ENABLE_VIDEO;
    sbus_writeb(val, &regs.control);
    par.flags |= CG3_FLAG_BLANKED;
    break;
    }
    spin_unlock_irqrestore(&par.lock, flags);
    return 0;
    }
    static const struct sbus_mmap_map cg3_mmap_map[] = {
    {
    .voff	= CG3_MMAP_OFFSET,
    .poff	= CG3_RAM_OFFSET,
    .size	= SBUS_MMAP_FBSIZE(1)
    },
    { .size = 0 }
    };
#[no_mangle]
unsafe extern "C" fn cg3_sbusfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int cg3_sbusfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct cg3_par *par = (struct cg3_par *)info.par;
    return sbusfb_mmap_helper(cg3_mmap_map,
    info.fix.smem_start, info.fix.smem_len,
    par.which_io,
    vma);
    }
#[no_mangle]
unsafe extern "C" fn cg3_sbusfb_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int {
    static int cg3_sbusfb_ioctl(struct fb_info *info, unsigned int cmd, unsigned long arg)
    {
    return sbusfb_ioctl_helper(cmd, arg, info,
    FBTYPE_SUN3COLOR, 8, info.fix.smem_len);
    }
//
// Initialisation
//
    static void cg3_init_fix(struct fb_info *info, int linebytes,
    struct device_node *dp)
    {
    snprintf(info.fix.id, sizeof(info.fix.id), "%pOFn", dp);
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.fix.line_length = linebytes;
    info.fix.accel = FB_ACCEL_SUN_CGTHREE;
    }
    static void cg3_rdi_maybe_fixup_var(struct fb_var_screeninfo *var,
    struct device_node *dp)
    {
    const char *params;
    char *p;
    int ww, hh;
    params = of_get_property(dp, "params", core::ptr::null_mut());
    if (params) {
    ww = simple_strtoul(params, &p, 10);
    if (ww && *p == 'x') {
    hh = simple_strtoul(p + 1, &p, 10);
    if (hh && *p == '-') {
    if (var.xres != ww ||
    var.yres != hh) {
    var.xres = var.xres_virtual = ww;
    var.yres = var.yres_virtual = hh;
    }
    }
    }
    }
    }
    static u8 cg3regvals_66hz[] = {	/* 1152 x 900, 66 Hz */
    0x14, 0xbb,	0x15, 0x2b,	0x16, 0x04,	0x17, 0x14,
    0x18, 0xae,	0x19, 0x03,	0x1a, 0xa8,	0x1b, 0x24,
    0x1c, 0x01,	0x1d, 0x05,	0x1e, 0xff,	0x1f, 0x01,
    0x10, 0x20,	0
    };
    static u8 cg3regvals_76hz[] = {	/* 1152 x 900, 76 Hz */
    0x14, 0xb7,	0x15, 0x27,	0x16, 0x03,	0x17, 0x0f,
    0x18, 0xae,	0x19, 0x03,	0x1a, 0xae,	0x1b, 0x2a,
    0x1c, 0x01,	0x1d, 0x09,	0x1e, 0xff,	0x1f, 0x01,
    0x10, 0x24,	0
    };
    static u8 cg3regvals_rdi[] = {	/* 640 x 480, cgRDI */
    0x14, 0x70,	0x15, 0x20,	0x16, 0x08,	0x17, 0x10,
    0x18, 0x06,	0x19, 0x02,	0x1a, 0x31,	0x1b, 0x51,
    0x1c, 0x06,	0x1d, 0x0c,	0x1e, 0xff,	0x1f, 0x01,
    0x10, 0x22,	0
    };
    static u8 *cg3_regvals[] = {
    cg3regvals_66hz, cg3regvals_76hz, cg3regvals_rdi
    };
    static u_char cg3_dacvals[] = {
    4, 0xff,	5, 0x00,	6, 0x70,	7, 0x00,	0
    };
#[no_mangle]
unsafe extern "C" fn cg3_do_default_mode(par: *mut cg3_par) -> c_int {
    static int cg3_do_default_mode(struct cg3_par *par)
    {
    enum cg3_type type;
    u8 *p;
    if (par.flags & CG3_FLAG_RDI)
    type = CG3_RDI;
    else {
    let mut status: u8 = sbus_readb(&par.regs.status), mon;
    if ((status & CG3_SR_ID_MASK) == CG3_SR_ID_COLOR) {
    mon = status & CG3_SR_RES_MASK;
    if (mon == CG3_SR_1152_900_76_A ||
    mon == CG3_SR_1152_900_76_B)
    type = CG3_AT_76HZ;
    else
    type = CG3_AT_66HZ;
    } else {
    printk(KERN_ERR "cgthree: can't handle SR %02x\n",
    status);
    return -EINVAL;
    }
    }
    for (p = cg3_regvals[type]; *p; p += 2) {
    u8 __iomem *regp = &((u8 __iomem *)par.regs)[p[0]];
    sbus_writeb(p[1], regp);
    }
    for (p = cg3_dacvals; *p; p += 2) {
    u8 __iomem *regp;
    regp = (u8 __iomem *)&par.regs.cmap.addr;
    sbus_writeb(p[0], regp);
    regp = (u8 __iomem *)&par.regs.cmap.control;
    sbus_writeb(p[1], regp);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cg3_probe(op: *mut platform_device) -> c_int {
    static int cg3_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
    struct fb_info *info;
    struct cg3_par *par;
    int linebytes, err;
    info = framebuffer_alloc(sizeof(struct cg3_par), &op.dev);
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
    if (of_node_name_eq(dp, "cgRDI"))
    par.flags |= CG3_FLAG_RDI;
    if (par.flags & CG3_FLAG_RDI)
    cg3_rdi_maybe_fixup_var(&info.var, dp);
    linebytes = of_getintprop_default(dp, "linebytes",
    info.var.xres);
    info.fix.smem_len = PAGE_ALIGN(linebytes * info.var.yres);
    par.regs = of_ioremap(&op.resource[0], CG3_REGS_OFFSET,
    sizeof(struct cg3_regs), "cg3 regs");
    if (!par.regs)
    goto out_release_fb;
    info.fbops = &cg3_ops;
    info.screen_base = of_ioremap(&op.resource[0], CG3_RAM_OFFSET,
    info.fix.smem_len, "cg3 ram");
    if (!info.screen_base)
    goto out_unmap_regs;
    cg3_blank(FB_BLANK_UNBLANK, info);
    if (!of_property_present(dp, "width")) {
    err = cg3_do_default_mode(par);
    if (err)
    goto out_unmap_screen;
    }
    err = fb_alloc_cmap(&info.cmap, 256, 0);
    if (err)
    goto out_unmap_screen;
    fb_set_cmap(&info.cmap, info);
    cg3_init_fix(info, linebytes, dp);
    err = register_framebuffer(info);
    if (err < 0)
    goto out_dealloc_cmap;
    dev_set_drvdata(&op.dev, info);
    printk(KERN_INFO "%pOF: cg3 at %lx:%lx\n",
    dp, par.which_io, info.fix.smem_start);
    return 0;
    out_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    out_unmap_screen:
    of_iounmap(&op.resource[0], info.screen_base, info.fix.smem_len);
    out_unmap_regs:
    of_iounmap(&op.resource[0], par.regs, sizeof(struct cg3_regs));
    out_release_fb:
    framebuffer_release(info);
    out_err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cg3_remove(op: *mut platform_device) {
    static void cg3_remove(struct platform_device *op)
    {
    struct fb_info *info = dev_get_drvdata(&op.dev);
    struct cg3_par *par = info.par;
    unregister_framebuffer(info);
    fb_dealloc_cmap(&info.cmap);
    of_iounmap(&op.resource[0], par.regs, sizeof(struct cg3_regs));
    of_iounmap(&op.resource[0], info.screen_base, info.fix.smem_len);
    framebuffer_release(info);
    }
    static const struct of_device_id cg3_match[] = {
    {
    .name = "cgthree",
    },
    {
    .name = "cgRDI",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cg3_match);
    static struct platform_driver cg3_driver = {
    .driver = {
    .name = "cg3",
    .of_match_table = cg3_match,
    },
    .probe		= cg3_probe,
    .remove		= cg3_remove,
    };
#[no_mangle]
unsafe extern "C" fn cg3_init() -> int __init {
    static int __init cg3_init(void)
    {
    if (fb_get_options("cg3fb", core::ptr::null_mut()))
    return -ENODEV;
    return platform_driver_register(&cg3_driver);
    }
#[no_mangle]
unsafe extern "C" fn cg3_exit() -> void __exit {
    static void __exit cg3_exit(void)
    {
    platform_driver_unregister(&cg3_driver);
    }
    module_init(cg3_init);
    module_exit(cg3_exit);
    MODULE_DESCRIPTION("framebuffer driver for CGthree chipsets");
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
