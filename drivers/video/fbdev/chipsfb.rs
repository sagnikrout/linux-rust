//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/chipsfb.c
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


//
// drivers/video/chipsfb.c -- frame buffer device for
// Chips & Technologies 65550 chip.
//
// Copyright (C) 1998-2002 Paul Mackerras
//
// This file is derived from the Powermac "chips" driver:
// Copyright (C) 1997 Fabio Riccardi.
// And from the frame buffer device for Open Firmware-initialized devices:
// Copyright (C) 1997 Geert Uytterhoeven.
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//

//
// Since we access the display with inb/outb to fixed port numbers,
// we can only handle one 6555x chip.  -- paulus
//

    outb((num), (ap)); outb((val), (dp)); \
    } while (0)

    outb((num), (ap)); var = inb((dp)); \
    } while (0)
// extension registers

// flat panel registers

// CRTC registers

// graphics registers

// sequencer registers

// attribute registers - slightly strange

    inb(0x3da); write_ind(num, val, 0x3c0, 0x3c0); \
    } while (0)

    inb(0x3da); read_ind(num, var, 0x3c0, 0x3c1); \
    } while (0)
//
// Exported functions
//
    int chips_init(void);
    static int chipsfb_pci_init(struct pci_dev *dp, const struct pci_device_id *);
    static int chipsfb_check_var(struct fb_var_screeninfo *var,
    struct fb_info *info);
    static int chipsfb_set_par(struct fb_info *info);
    static int chipsfb_setcolreg(u_int regno, u_int red, u_int green, u_int blue,
    u_int transp, struct fb_info *info);
    static int chipsfb_blank(int blank, struct fb_info *info);
    static const struct fb_ops chipsfb_ops = {
    .owner		= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_check_var	= chipsfb_check_var,
    .fb_set_par	= chipsfb_set_par,
    .fb_setcolreg	= chipsfb_setcolreg,
    .fb_blank	= chipsfb_blank,
    };
    static int chipsfb_check_var(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    if (var.xres > 800 || var.yres > 600
    || var.xres_virtual > 800 || var.yres_virtual > 600
    || (var.bits_per_pixel != 8 && var.bits_per_pixel != 16)
    || var.nonstd
    || (var.vmode & FB_VMODE_MASK) != FB_VMODE_NONINTERLACED)
    return -EINVAL;
    var.xres = var.xres_virtual = 800;
    var.yres = var.yres_virtual = 600;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chipsfb_set_par(info: *mut fb_info) -> c_int {
    static int chipsfb_set_par(struct fb_info *info)
    {
    if (info.var.bits_per_pixel == 16) {
    write_cr(0x13, 200);		// Set line length (doublewords)
    write_xr(0x81, 0x14);		// 15 bit (555) color mode
    write_xr(0x82, 0x00);		// Disable palettes
    write_xr(0x20, 0x10);		// 16 bit blitter mode
    info.fix.line_length = 800*2;
    info.fix.visual = FB_VISUAL_TRUECOLOR;
    info.var.red.offset = 10;
    info.var.green.offset = 5;
    info.var.blue.offset = 0;
    info.var.red.length = info.var.green.length =
    info.var.blue.length = 5;
    } else {
// p->var.bits_per_pixel == 8
    write_cr(0x13, 100);		// Set line length (doublewords)
    write_xr(0x81, 0x12);		// 8 bit color mode
    write_xr(0x82, 0x08);		// Graphics gamma enable
    write_xr(0x20, 0x00);		// 8 bit blitter mode
    info.fix.line_length = 800;
    info.fix.visual = FB_VISUAL_PSEUDOCOLOR;
    info.var.red.offset = info.var.green.offset =
    info.var.blue.offset = 0;
    info.var.red.length = info.var.green.length =
    info.var.blue.length = 8;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chipsfb_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int chipsfb_blank(int blank, struct fb_info *info)
    {
    return 1;	/* get fb_blank to set the colormap to all black */
    }
    static int chipsfb_setcolreg(u_int regno, u_int red, u_int green, u_int blue,
    u_int transp, struct fb_info *info)
    {
    if (regno > 255)
    return 1;
    red >>= 8;
    green >>= 8;
    blue >>= 8;
    outb(regno, 0x3c8);
    udelay(1);
    outb(red, 0x3c9);
    outb(green, 0x3c9);
    outb(blue, 0x3c9);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chips_init_reg {
    pub addr: c_uchar,
    pub data: c_uchar,
}

    static struct chips_init_reg chips_init_sr[] = {
    { 0x00, 0x03 },
    { 0x01, 0x01 },
    { 0x02, 0x0f },
    { 0x04, 0x0e }
    };
    static struct chips_init_reg chips_init_gr[] = {
    { 0x05, 0x00 },
    { 0x06, 0x0d },
    { 0x08, 0xff }
    };
    static struct chips_init_reg chips_init_ar[] = {
    { 0x10, 0x01 },
    { 0x12, 0x0f },
    { 0x13, 0x00 }
    };
    static struct chips_init_reg chips_init_cr[] = {
    { 0x00, 0x7f },
    { 0x01, 0x63 },
    { 0x02, 0x63 },
    { 0x03, 0x83 },
    { 0x04, 0x66 },
    { 0x05, 0x10 },
    { 0x06, 0x72 },
    { 0x07, 0x3e },
    { 0x08, 0x00 },
    { 0x09, 0x40 },
    { 0x0c, 0x00 },
    { 0x0d, 0x00 },
    { 0x10, 0x59 },
    { 0x11, 0x0d },
    { 0x12, 0x57 },
    { 0x13, 0x64 },
    { 0x14, 0x00 },
    { 0x15, 0x57 },
    { 0x16, 0x73 },
    { 0x17, 0xe3 },
    { 0x18, 0xff },
    { 0x30, 0x02 },
    { 0x31, 0x02 },
    { 0x32, 0x02 },
    { 0x33, 0x02 },
    { 0x40, 0x00 },
    { 0x41, 0x00 },
    { 0x40, 0x80 }
    };
    static struct chips_init_reg chips_init_fr[] = {
    { 0x01, 0x02 },
    { 0x03, 0x08 },
    { 0x04, 0x81 },
    { 0x05, 0x21 },
    { 0x08, 0x0c },
    { 0x0a, 0x74 },
    { 0x0b, 0x11 },
    { 0x10, 0x0c },
    { 0x11, 0xe0 },
// { 0x12, 0x40 }, -- 3400 needs 40, 2400 needs 48, no way to tell
    { 0x20, 0x63 },
    { 0x21, 0x68 },
    { 0x22, 0x19 },
    { 0x23, 0x7f },
    { 0x24, 0x68 },
    { 0x26, 0x00 },
    { 0x27, 0x0f },
    { 0x30, 0x57 },
    { 0x31, 0x58 },
    { 0x32, 0x0d },
    { 0x33, 0x72 },
    { 0x34, 0x02 },
    { 0x35, 0x22 },
    { 0x36, 0x02 },
    { 0x37, 0x00 }
    };
    static struct chips_init_reg chips_init_xr[] = {
    { 0xce, 0x00 },		/* set default memory clock */
    { 0xcc, 0x43 },		/* memory clock ratio */
    { 0xcd, 0x18 },
    { 0xce, 0xa1 },
    { 0xc8, 0x84 },
    { 0xc9, 0x0a },
    { 0xca, 0x00 },
    { 0xcb, 0x20 },
    { 0xcf, 0x06 },
    { 0xd0, 0x0e },
    { 0x09, 0x01 },
    { 0x0a, 0x02 },
    { 0x0b, 0x01 },
    { 0x20, 0x00 },
    { 0x40, 0x03 },
    { 0x41, 0x01 },
    { 0x42, 0x00 },
    { 0x80, 0x82 },
    { 0x81, 0x12 },
    { 0x82, 0x08 },
    { 0xa0, 0x00 },
    { 0xa8, 0x00 }
    };
#[no_mangle]
unsafe extern "C" fn chips_hw_init() {
    static void chips_hw_init(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(chips_init_xr); ++i)
    write_xr(chips_init_xr[i].addr, chips_init_xr[i].data);
    outb(0x29, 0x3c2); /* set misc output reg */
    for (i = 0; i < ARRAY_SIZE(chips_init_sr); ++i)
    write_sr(chips_init_sr[i].addr, chips_init_sr[i].data);
    for (i = 0; i < ARRAY_SIZE(chips_init_gr); ++i)
    write_gr(chips_init_gr[i].addr, chips_init_gr[i].data);
    for (i = 0; i < ARRAY_SIZE(chips_init_ar); ++i)
    write_ar(chips_init_ar[i].addr, chips_init_ar[i].data);
    for (i = 0; i < ARRAY_SIZE(chips_init_cr); ++i)
    write_cr(chips_init_cr[i].addr, chips_init_cr[i].data);
    for (i = 0; i < ARRAY_SIZE(chips_init_fr); ++i)
    write_fr(chips_init_fr[i].addr, chips_init_fr[i].data);
    }
    static const struct fb_fix_screeninfo chipsfb_fix = {
    .id =		"C&T 65550",
    .type =		FB_TYPE_PACKED_PIXELS,
    .visual =	FB_VISUAL_PSEUDOCOLOR,
    .accel =	FB_ACCEL_NONE,
    .line_length =	800,
// FIXME: Assumes 1MB frame buffer, but 65550 supports 1MB or 2MB.
// * "3500" PowerBook G3 (the original PB G3) has 2MB.
// * 2400 has 1MB composed of 2 Mitsubishi M5M4V4265CTP DRAM chips.
// Motherboard actually supports 2MB -- there are two blank locations
// for a second pair of DRAMs.  (Thanks, Apple!)
// * 3400 has 1MB (I think).  Don't know if it's expandable.
// -- Tim Seufert
    .smem_len =	0x100000,	/* 1MB */
    };
    static const struct fb_var_screeninfo chipsfb_var = {
    .xres = 800,
    .yres = 600,
    .xres_virtual = 800,
    .yres_virtual = 600,
    .bits_per_pixel = 8,
    .red = { .length = 8 },
    .green = { .length = 8 },
    .blue = { .length = 8 },
    .height = -1,
    .width = -1,
    .vmode = FB_VMODE_NONINTERLACED,
    .pixclock = 10000,
    .left_margin = 16,
    .right_margin = 16,
    .upper_margin = 16,
    .lower_margin = 16,
    .hsync_len = 8,
    .vsync_len = 8,
    };
#[no_mangle]
unsafe extern "C" fn init_chips(p: *mut fb_info, addr: c_ulong) {
    static void init_chips(struct fb_info *p, unsigned long addr)
    {
    fb_memset_io(p.screen_base, 0, 0x100000);
    p.fix = chipsfb_fix;
    p.fix.smem_start = addr;
    p.var = chipsfb_var;
    p.fbops = &chipsfb_ops;
    fb_alloc_cmap(&p.cmap, 256, 0);
    chips_hw_init();
    }
#[no_mangle]
unsafe extern "C" fn chipsfb_pci_init(dp: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int chipsfb_pci_init(struct pci_dev *dp, const struct pci_device_id *ent)
    {
    struct fb_info *p;
    unsigned long addr;
    unsigned short cmd;
    int rc;
    rc = aperture_remove_conflicting_pci_devices(dp, "chipsfb");
    if (rc)
    return rc;
    rc = pci_enable_device(dp);
    if (rc < 0) {
    dev_err(&dp.dev, "Cannot enable PCI device\n");
    goto err_out;
    }
    if ((dp.resource[0].flags & IORESOURCE_MEM) == 0) {
    rc = -ENODEV;
    goto err_disable;
    }
    addr = pci_resource_start(dp, 0);
    if (addr == 0) {
    rc = -ENODEV;
    goto err_disable;
    }
    p = framebuffer_alloc(0, &dp.dev);
    if (p == core::ptr::null_mut()) {
    rc = -ENOMEM;
    goto err_disable;
    }
    if (pci_request_region(dp, 0, "chipsfb") != 0) {
    dev_err(&dp.dev, "Cannot request framebuffer\n");
    rc = -EBUSY;
    goto err_release_fb;
    }

    addr += 0x800000;	// Use big-endian aperture

// we should use pci_enable_device here, but,
    the device doesn't declare its I/O ports in its BARs
    so pci_enable_device won't turn on I/O responses */
    pci_read_config_word(dp, PCI_COMMAND, &cmd);
    cmd |= 3;	/* enable memory and IO space */
    pci_write_config_word(dp, PCI_COMMAND, cmd);

// turn on the backlight
    mutex_lock(&pmac_backlight_mutex);
    if (pmac_backlight) {
    pmac_backlight.props.power = BACKLIGHT_POWER_ON;
    backlight_update_status(pmac_backlight);
    }
    mutex_unlock(&pmac_backlight_mutex);

    p.screen_base = ioremap_wc(addr, 0x200000);

    p.screen_base = ioremap(addr, 0x200000);

    if (p.screen_base == core::ptr::null_mut()) {
    dev_err(&dp.dev, "Cannot map framebuffer\n");
    rc = -ENOMEM;
    goto err_release_pci;
    }
    pci_set_drvdata(dp, p);
    init_chips(p, addr);
    rc = register_framebuffer(p);
    if (rc < 0) {
    dev_err(&dp.dev,"C&T 65550 framebuffer failed to register\n");
    goto err_unmap;
    }
    dev_info(&dp.dev,"fb%d: Chips 65550 frame buffer"
    " (%dK RAM detected)\n",
    p.node, p.fix.smem_len / 1024);
    return 0;
    err_unmap:
    iounmap(p.screen_base);
    err_release_pci:
    pci_release_region(dp, 0);
    err_release_fb:
    framebuffer_release(p);
    err_disable:
    pci_disable_device(dp);
    err_out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn chipsfb_remove(dp: *mut pci_dev) {
    static void chipsfb_remove(struct pci_dev *dp)
    {
    struct fb_info *p = pci_get_drvdata(dp);
    if (p.screen_base == core::ptr::null_mut())
    return;
    unregister_framebuffer(p);
    iounmap(p.screen_base);
    p.screen_base = core::ptr::null_mut();
    pci_release_region(dp, 0);
    }

#[no_mangle]
unsafe extern "C" fn chipsfb_pci_suspend(pdev: *mut pci_dev, state: pm_message_t) -> c_int {
    static int chipsfb_pci_suspend(struct pci_dev *pdev, pm_message_t state)
    {
    struct fb_info *p = pci_get_drvdata(pdev);
    if (state.event == pdev.dev.power.power_state.event)
    return 0;
    if (!(state.event & PM_EVENT_SLEEP))
    goto done;
    console_lock();
    chipsfb_blank(1, p);
    fb_set_suspend(p, 1);
    console_unlock();
    done:
    pdev.dev.power.power_state = state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chipsfb_pci_resume(pdev: *mut pci_dev) -> c_int {
    static int chipsfb_pci_resume(struct pci_dev *pdev)
    {
    struct fb_info *p = pci_get_drvdata(pdev);
    console_lock();
    fb_set_suspend(p, 0);
    chipsfb_blank(0, p);
    console_unlock();
    pdev.dev.power.power_state = PMSG_ON;
    return 0;
    }

    static struct pci_device_id chipsfb_pci_tbl[] = {
    { PCI_VENDOR_ID_CT, PCI_DEVICE_ID_CT_65550, PCI_ANY_ID, PCI_ANY_ID },
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, chipsfb_pci_tbl);
    static struct pci_driver chipsfb_driver = {
    .name =		"chipsfb",
    .id_table =	chipsfb_pci_tbl,
    .probe =	chipsfb_pci_init,
    .remove =	chipsfb_remove,

    .suspend =	chipsfb_pci_suspend,
    .resume =	chipsfb_pci_resume,

    };
#[no_mangle]
pub unsafe extern "C" fn chips_init() -> int __init {
    int __init chips_init(void)
    {
    if (fb_modesetting_disabled("chipsfb"))
    return -ENODEV;
    if (fb_get_options("chipsfb", core::ptr::null_mut()))
    return -ENODEV;
    return pci_register_driver(&chipsfb_driver);
    }
    module_init(chips_init);
#[no_mangle]
unsafe extern "C" fn chipsfb_exit() -> void __exit {
    static void __exit chipsfb_exit(void)
    {
    pci_unregister_driver(&chipsfb_driver);
    }
    MODULE_DESCRIPTION("Chips & Technologies 65550 frame buffer driver");
    MODULE_LICENSE("GPL");
