//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/sunxvr2500.c
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


// sunxvr2500.c: Sun 3DLABS XVR-2500 et al. fb driver for sparc64 systems
//
// License: GPL
//
// Copyright (C) 2007 David S. Miller (davem@davemloft.net)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3d_info {
    pub info: *mut fb_info,
    pub pdev: *mut pci_dev,
    pub fb_base: *mut char __iomem,
    pub fb_base_phys: c_ulong,
    pub of_node: *mut device_node,
    pub width: c_uint,
    pub height: c_uint,
    pub depth: c_uint,
    pub fb_size: c_uint,
    pub pseudo_palette: [u32; 16],
}

#[no_mangle]
unsafe extern "C" fn s3d_get_props(sp: *mut s3d_info) -> c_int {
    static int s3d_get_props(struct s3d_info *sp)
    {
    sp.width = of_getintprop_default(sp.of_node, "width", 0);
    sp.height = of_getintprop_default(sp.of_node, "height", 0);
    sp.depth = of_getintprop_default(sp.of_node, "depth", 8);
    if (!sp.width || !sp.height) {
    pci_err(sp.pdev, "Critical properties missing\n");
    return -EINVAL;
    }
    return 0;
    }
    static int s3d_setcolreg(unsigned regno,
    unsigned red, unsigned green, unsigned blue,
    unsigned transp, struct fb_info *info)
    {
    u32 value;
    if (regno < 16) {
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    value = (blue << 24) | (green << 16) | (red << 8);
    ((u32 *)info.pseudo_palette)[regno] = value;
    }
    return 0;
    }
    static const struct fb_ops s3d_ops = {
    .owner			= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_setcolreg		= s3d_setcolreg,
    };
#[no_mangle]
unsafe extern "C" fn s3d_set_fbinfo(sp: *mut s3d_info) -> c_int {
    static int s3d_set_fbinfo(struct s3d_info *sp)
    {
    struct fb_info *info = sp.info;
    struct fb_var_screeninfo *var = &info.var;
    info.fbops = &s3d_ops;
    info.screen_base = sp.fb_base;
    info.screen_size = sp.fb_size;
    info.pseudo_palette = sp.pseudo_palette;
// Fill fix common fields
    strscpy(info.fix.id, "s3d", sizeof(info.fix.id));
    info.fix.smem_start = sp.fb_base_phys;
    info.fix.smem_len = sp.fb_size;
    info.fix.type = FB_TYPE_PACKED_PIXELS;
    if (sp.depth == 32 || sp.depth == 24)
    info.fix.visual = FB_VISUAL_TRUECOLOR;
    else
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    var.xres = sp.width;
    var.yres = sp.height;
    var.xres_virtual = var.xres;
    var.yres_virtual = var.yres;
    var.bits_per_pixel = sp.depth;
    var.red.offset = 8;
    var.red.length = 8;
    var.green.offset = 16;
    var.green.length = 8;
    var.blue.offset = 24;
    var.blue.length = 8;
    var.transp.offset = 0;
    var.transp.length = 0;
    if (fb_alloc_cmap(&info.cmap, 256, 0)) {
    pci_err(sp.pdev, "Cannot allocate color map\n");
    return -ENOMEM;
    }
    return 0;
    }
    static int s3d_pci_register(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct fb_info *info;
    struct s3d_info *sp;
    int err;
    err = aperture_remove_conflicting_pci_devices(pdev, "s3dfb");
    if (err)
    return err;
    err = pci_enable_device(pdev);
    if (err < 0) {
    pci_err(pdev, "Cannot enable PCI device\n");
    goto err_out;
    }
    info = framebuffer_alloc(sizeof(struct s3d_info), &pdev.dev);
    if (!info) {
    err = -ENOMEM;
    goto err_disable;
    }
    sp = info.par;
    sp.info = info;
    sp.pdev = pdev;
    sp.of_node = pci_device_to_OF_node(pdev);
    if (!sp.of_node) {
    pci_err(pdev, "Cannot find OF node\n");
    err = -ENODEV;
    goto err_release_fb;
    }
    sp.fb_base_phys = pci_resource_start (pdev, 1);
    err = pci_request_region(pdev, 1, "s3d framebuffer");
    if (err < 0) {
    pci_err(pdev, "Cannot request region 1\n");
    goto err_release_fb;
    }
    err = s3d_get_props(sp);
    if (err)
    goto err_release_pci;
// XXX 'linebytes' is often wrong, it is equal to the width
// XXX with depth of 32 on my XVR-2500 which is clearly not
// XXX right.  So we don't try to use it.
//
    switch (sp.depth) {
    case 8:
    info.fix.line_length = sp.width;
    break;
    case 16:
    info.fix.line_length = sp.width * 2;
    break;
    case 24:
    info.fix.line_length = sp.width * 3;
    break;
    case 32:
    info.fix.line_length = sp.width * 4;
    break;
    }
    sp.fb_size = info.fix.line_length * sp.height;
    sp.fb_base = ioremap(sp.fb_base_phys, sp.fb_size);
    if (!sp.fb_base) {
    err = -ENOMEM;
    goto err_release_pci;
    }
    err = s3d_set_fbinfo(sp);
    if (err)
    goto err_unmap_fb;
    pci_set_drvdata(pdev, info);
    pci_info(pdev, "Found device\n");
    err = register_framebuffer(info);
    if (err < 0) {
    pci_err(pdev, "Could not register framebuffer\n");
    goto err_unmap_fb;
    }
    return 0;
    err_unmap_fb:
    iounmap(sp.fb_base);
    err_release_pci:
    pci_release_region(pdev, 1);
    err_release_fb:
    framebuffer_release(info);
    err_disable:
    pci_disable_device(pdev);
    err_out:
    return err;
    }
    static const struct pci_device_id s3d_pci_table[] = {
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x002c),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x002d),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x002e),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x002f),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x0030),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x0031),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x0032),	},
    {	PCI_DEVICE(PCI_VENDOR_ID_3DLABS, 0x0033),	},
    { 0, }
    };
    static struct pci_driver s3d_driver = {
    .driver = {
    .suppress_bind_attrs = true,
    },
    .name		= "s3d",
    .id_table	= s3d_pci_table,
    .probe		= s3d_pci_register,
    };
#[no_mangle]
unsafe extern "C" fn s3d_init() -> int __init {
    static int __init s3d_init(void)
    {
    if (fb_modesetting_disabled("s3d"))
    return -ENODEV;
    if (fb_get_options("s3d", core::ptr::null_mut()))
    return -ENODEV;
    return pci_register_driver(&s3d_driver);
    }
    device_initcall(s3d_init);
