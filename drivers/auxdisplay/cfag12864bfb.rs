//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/cfag12864bfb.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Filename: cfag12864bfb.c
// Version: 0.1.0
// Description: cfag12864b LCD framebuffer driver
// Depends: cfag12864b
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-31
//

    static const struct fb_fix_screeninfo cfag12864bfb_fix = {
    .id = "cfag12864b",
    .type = FB_TYPE_PACKED_PIXELS,
    .visual = FB_VISUAL_MONO10,
    .xpanstep = 0,
    .ypanstep = 0,
    .ywrapstep = 0,
    .line_length = CFAG12864B_WIDTH / 8,
    .accel = FB_ACCEL_NONE,
    };
    static const struct fb_var_screeninfo cfag12864bfb_var = {
    .xres = CFAG12864B_WIDTH,
    .yres = CFAG12864B_HEIGHT,
    .xres_virtual = CFAG12864B_WIDTH,
    .yres_virtual = CFAG12864B_HEIGHT,
    .bits_per_pixel = 1,
    .red = { 0, 1, 0 },
    .green = { 0, 1, 0 },
    .blue = { 0, 1, 0 },
    .left_margin = 0,
    .right_margin = 0,
    .upper_margin = 0,
    .lower_margin = 0,
    .vmode = FB_VMODE_NONINTERLACED,
    };
#[no_mangle]
unsafe extern "C" fn cfag12864bfb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int cfag12864bfb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct page *pages = virt_to_page(cfag12864b_buffer);
    vma.vm_page_prot = pgprot_decrypted(vma.vm_page_prot);
    return vm_map_pages_zero(vma, &pages, 1);
    }
    static const struct fb_ops cfag12864bfb_ops = {
    .owner = THIS_MODULE,
    __FB_DEFAULT_SYSMEM_OPS_RDWR,
    __FB_DEFAULT_SYSMEM_OPS_DRAW,
    .fb_mmap = cfag12864bfb_mmap,
    };
#[no_mangle]
unsafe extern "C" fn cfag12864bfb_probe(device: *mut platform_device) -> c_int {
    static int cfag12864bfb_probe(struct platform_device *device)
    {
    let mut ret: c_int = -EINVAL;
    struct fb_info *info = framebuffer_alloc(0, &device.dev);
    if (!info)
    goto none;
    info.flags = FBINFO_VIRTFB;
    info.screen_buffer = cfag12864b_buffer;
    info.screen_size = CFAG12864B_SIZE;
    info.fbops = &cfag12864bfb_ops;
    info.fix = cfag12864bfb_fix;
    info.var = cfag12864bfb_var;
    info.pseudo_palette = core::ptr::null_mut();
    info.par = core::ptr::null_mut();
    if (register_framebuffer(info) < 0)
    goto fballoced;
    platform_set_drvdata(device, info);
    fb_info(info, "%s frame buffer device\n", info.fix.id);
    return 0;
    fballoced:
    framebuffer_release(info);
    none:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cfag12864bfb_remove(device: *mut platform_device) {
    static void cfag12864bfb_remove(struct platform_device *device)
    {
    struct fb_info *info = platform_get_drvdata(device);
    if (info) {
    unregister_framebuffer(info);
    framebuffer_release(info);
    }
    }
    static struct platform_driver cfag12864bfb_driver = {
    .probe	= cfag12864bfb_probe,
    .remove = cfag12864bfb_remove,
    .driver = {
    .name	= CFAG12864BFB_NAME,
    },
    };
    static struct platform_device *cfag12864bfb_device;
#[no_mangle]
unsafe extern "C" fn cfag12864bfb_init() -> int __init {
    static int __init cfag12864bfb_init(void)
    {
    let mut ret: c_int = -EINVAL;
// cfag12864b_init() must be called first
    if (!cfag12864b_isinited()) {
    printk(KERN_ERR CFAG12864BFB_NAME ": ERROR: "
    "cfag12864b is not initialized\n");
    goto none;
    }
    if (cfag12864b_enable()) {
    printk(KERN_ERR CFAG12864BFB_NAME ": ERROR: "
    "can't enable cfag12864b refreshing (being used)\n");
    return -ENODEV;
    }
    ret = platform_driver_register(&cfag12864bfb_driver);
    if (!ret) {
    cfag12864bfb_device =
    platform_device_alloc(CFAG12864BFB_NAME, 0);
    if (cfag12864bfb_device)
    ret = platform_device_add(cfag12864bfb_device);
    else
    ret = -ENOMEM;
    if (ret) {
    platform_device_put(cfag12864bfb_device);
    platform_driver_unregister(&cfag12864bfb_driver);
    }
    }
    none:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cfag12864bfb_exit() -> void __exit {
    static void __exit cfag12864bfb_exit(void)
    {
    platform_device_unregister(cfag12864bfb_device);
    platform_driver_unregister(&cfag12864bfb_driver);
    cfag12864b_disable();
    }
    module_init(cfag12864bfb_init);
    module_exit(cfag12864bfb_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Miguel Ojeda <ojeda@kernel.org>");
    MODULE_DESCRIPTION("cfag12864b LCD framebuffer driver");
