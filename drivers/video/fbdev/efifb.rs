//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/efifb.c
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
// Framebuffer driver for EFI/UEFI based system
//
// (c) 2006 Edgar Hucek <gimli@dark-green.com>
// Original efi driver written by Gerd Knorr <kraxel@goldbach.in-berlin.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_file_header {
    pub id: u16,
    pub file_size: u32,
    pub reserved: u32,
    pub bitmap_offset: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_dib_header {
    pub dib_header_size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bpp: u16,
    pub compression: u32,
    pub bitmap_size: u32,
    pub horz_resolution: u32,
    pub vert_resolution: u32,
    pub colors_used: u32,
    pub colors_important: u32,
    pub __packed: },
    pub true: static bool use_bgrt =,
    pub false: static bool request_mem_succeeded =,
    pub EFI_MEMORY_UC: static u64 mem_flags = EFI_MEMORY_WC |,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efifb_par {
    pub pseudo_palette: [u32; 16],
    pub base: resource_size_t,
    pub size: resource_size_t,
}

    static struct fb_var_screeninfo efifb_defined = {
    .activate		= FB_ACTIVATE_NOW,
    .height			= -1,
    .width			= -1,
    .right_margin		= 32,
    .upper_margin		= 16,
    .lower_margin		= 4,
    .vsync_len		= 4,
    .vmode			= FB_VMODE_NONINTERLACED,
    };
    static struct fb_fix_screeninfo efifb_fix = {
    .id			= "EFI VGA",
    .type			= FB_TYPE_PACKED_PIXELS,
    .accel			= FB_ACCEL_NONE,
    .visual			= FB_VISUAL_TRUECOLOR,
    };
    static int efifb_setcolreg(unsigned regno, unsigned red, unsigned green,
    unsigned blue, unsigned transp,
    struct fb_info *info)
    {
//
// Set a single color register. The values supplied are
// already rounded down to the hardware's capabilities
// (according to the entries in the `var' structure). Return
// != 0 for invalid regno.
//
    if (regno >= info.cmap.len)
    return 1;
    if (regno < 16) {
    red   >>= 16 - info.var.red.length;
    green >>= 16 - info.var.green.length;
    blue  >>= 16 - info.var.blue.length;
    ((u32 *)(info.pseudo_palette))[regno] =
    (red   << info.var.red.offset)   |
    (green << info.var.green.offset) |
    (blue  << info.var.blue.offset);
    }
    return 0;
    }
//
// If fbcon deffered console takeover is configured, the intent is for the
// framebuffer to show the boot graphics (e.g. vendor logo) until there is some
// (error) message to display. But the boot graphics may have been destroyed by
// e.g. option ROM output, detect this and restore the boot graphics.
//

    defined CONFIG_ACPI_BGRT
#[no_mangle]
unsafe extern "C" fn efifb_copy_bmp(src: *mut u8, dst: *mut u32, width: c_int, si: *const screen_info) {
    static void efifb_copy_bmp(u8 *src, u32 *dst, int width, const struct screen_info *si)
    {
    u8 r, g, b;
    while (width--) {
    b = *src++;
    g = *src++;
    r = *src++;
// dst++ = (r << si->red_pos)   |
    (g << si.green_pos) |
    (b << si.blue_pos);
    }
    }

//
// On x86 some firmwares use a low non native resolution for the display when
// they have shown some text messages. While keeping the bgrt filled with info
// for the native resolution. If the bgrt image intended for the native
// resolution still fits, it will be displayed very close to the right edge of
// the display looking quite bad. This function checks for this.
//
#[no_mangle]
unsafe extern "C" fn efifb_bgrt_sanity_check(si: *const screen_info, bmp_width: u32) -> bool {
    static bool efifb_bgrt_sanity_check(const struct screen_info *si, u32 bmp_width)
    {
//
// All x86 firmwares horizontally center the image (the yoffset
// calculations differ between boards, but xoffset is predictable).
//
    let mut expected_xoffset: u32 = (si.lfb_width - bmp_width) / 2;
    return bgrt_tab.image_offset_x == expected_xoffset;
    }

#[no_mangle]
unsafe extern "C" fn efifb_bgrt_sanity_check(si: *const screen_info, bmp_width: u32) -> bool {
    static bool efifb_bgrt_sanity_check(const struct screen_info *si, u32 bmp_width)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn efifb_show_boot_graphics(info: *mut fb_info, si: *const screen_info) {
    static void efifb_show_boot_graphics(struct fb_info *info, const struct screen_info *si)
    {
    u32 bmp_width, bmp_height, bmp_pitch, dst_x, y, src_y;
    struct bmp_file_header *file_header;
    struct bmp_dib_header *dib_header;
    void *bgrt_image = core::ptr::null_mut();
    u8 *dst = info.screen_base;
    if (!use_bgrt)
    return;
    if (!bgrt_tab.image_address) {
    pr_info("efifb: No BGRT, not showing boot graphics\n");
    return;
    }
    if (bgrt_tab.status & 0x06) {
    pr_info("efifb: BGRT rotation bits set, not showing boot graphics\n");
    return;
    }
// Avoid flashing the logo if we're going to print std probe messages
    if (console_loglevel > CONSOLE_LOGLEVEL_QUIET)
    return;
// bgrt_tab.status is unreliable, so we don't check it
    if (si.lfb_depth != 32) {
    pr_info("efifb: not 32 bits, not showing boot graphics\n");
    return;
    }
    bgrt_image = memremap(bgrt_tab.image_address, bgrt_image_size,
    MEMREMAP_WB);
    if (!bgrt_image) {
    pr_warn("efifb: Ignoring BGRT: failed to map image memory\n");
    return;
    }
    if (bgrt_image_size < (sizeof(*file_header) + sizeof(*dib_header)))
    goto error;
    file_header = bgrt_image;
    if (file_header.id != 0x4d42 || file_header.reserved != 0)
    goto error;
    dib_header = bgrt_image + sizeof(*file_header);
    if (dib_header.dib_header_size != 40 || dib_header.width < 0 ||
    dib_header.planes != 1 || dib_header.bpp != 24 ||
    dib_header.compression != 0)
    goto error;
    bmp_width = dib_header.width;
    bmp_height = abs(dib_header.height);
    bmp_pitch = round_up(3 * bmp_width, 4);
    if ((file_header.bitmap_offset + bmp_pitch * bmp_height) >
    bgrt_image_size)
    goto error;
    if ((bgrt_tab.image_offset_x + bmp_width) > si.lfb_width ||
    (bgrt_tab.image_offset_y + bmp_height) > si.lfb_height)
    goto error;
    if (!efifb_bgrt_sanity_check(si, bmp_width))
    goto error;
    pr_info("efifb: showing boot graphics\n");
    for (y = 0; y < si.lfb_height; y++, dst += si.lfb_linelength) {
// Only background?
    if (y < bgrt_tab.image_offset_y ||
    y >= (bgrt_tab.image_offset_y + bmp_height)) {
    memset(dst, 0, 4 * si.lfb_width);
    continue;
    }
    src_y = y - bgrt_tab.image_offset_y;
// Positive header height means upside down row order
    if (dib_header.height > 0)
    src_y = (bmp_height - 1) - src_y;
    memset(dst, 0, bgrt_tab.image_offset_x * 4);
    dst_x = bgrt_tab.image_offset_x;
    efifb_copy_bmp(bgrt_image + file_header.bitmap_offset +
    src_y * bmp_pitch,
    (u32 *)dst + dst_x, bmp_width, si);
    dst_x += bmp_width;
    memset((u32 *)dst + dst_x, 0, (si.lfb_width - dst_x) * 4);
    }
    memunmap(bgrt_image);
    return;
    error:
    memunmap(bgrt_image);
    pr_warn("efifb: Ignoring BGRT: unexpected or invalid BMP data\n");
    }

#[no_mangle]
pub unsafe extern "C" fn efifb_show_boot_graphics(info: *mut fb_info, si: *const screen_info) {
    static inline void efifb_show_boot_graphics(struct fb_info *info, const struct screen_info *si)
    { }

//
// fb_ops.fb_destroy is called by the last put_fb_info() call at the end
// of unregister_framebuffer() or fb_release(). Do any cleanup here.
//
#[no_mangle]
unsafe extern "C" fn efifb_destroy(info: *mut fb_info) {
    static void efifb_destroy(struct fb_info *info)
    {
    struct efifb_par *par = info.par;
    if (info.screen_base) {
    if (mem_flags & (EFI_MEMORY_UC | EFI_MEMORY_WC))
    iounmap(info.screen_base);
    else
    memunmap(info.screen_base);
    }
    if (request_mem_succeeded)
    release_mem_region(par.base, par.size);
    fb_dealloc_cmap(&info.cmap);
    framebuffer_release(info);
    }
    static const struct fb_ops efifb_ops = {
    .owner		= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    .fb_destroy	= efifb_destroy,
    .fb_setcolreg	= efifb_setcolreg,
    };
#[no_mangle]
unsafe extern "C" fn efifb_setup(si: *mut screen_info, options: *mut c_char) {
    static void efifb_setup(struct screen_info *si, char *options)
    {
    char *this_opt;
    if (options && *options) {
    while ((this_opt = strsep(&options, ",")) != core::ptr::null_mut()) {
    if (!*this_opt) continue;
    efifb_setup_from_dmi(si, this_opt);
    if (!strncmp(this_opt, "base:", 5))
    si.lfb_base = simple_strtoul(this_opt+5, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(this_opt, _arg: "stride:", _arg: 7)) -> else {
    else if (!strncmp(this_opt, "stride:", 7))
    si.lfb_linelength = simple_strtoul(this_opt+7, core::ptr::null_mut(), 0) * 4;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(this_opt, _arg: "height:", _arg: 7)) -> else {
    else if (!strncmp(this_opt, "height:", 7))
    si.lfb_height = simple_strtoul(this_opt+7, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(this_opt, _arg: "width:", _arg: 6)) -> else {
    else if (!strncmp(this_opt, "width:", 6))
    si.lfb_width = simple_strtoul(this_opt+6, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(this_opt, _arg: "nowc")) -> else {
    else if (!strcmp(this_opt, "nowc"))
    mem_flags &= ~EFI_MEMORY_WC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(this_opt, _arg: "nobgrt")) -> else {
    else if (!strcmp(this_opt, "nobgrt"))
    use_bgrt = false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fb_base_is_valid(si: *mut screen_info) -> bool {
    static inline bool fb_base_is_valid(struct screen_info *si)
    {
    if (si.lfb_base)
    return true;
    if (!(si.capabilities & VIDEO_CAPABILITY_64BIT_BASE))
    return false;
    if (si.ext_lfb_base)
    return true;
    return false;
    }

    static ssize_t name##_show(struct device *dev,				\
    struct device_attribute *attr,		\
    char *buf)					\
    {									\
    struct screen_info *si = dev_get_drvdata(dev);			\
    if (!si)							\
    return -ENODEV;						\
    return sprintf(buf, fmt "\n", (si.lfb_##name));		\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    efifb_attr_decl(base, "0x%x");
    efifb_attr_decl(linelength, "%u");
    efifb_attr_decl(height, "%u");
    efifb_attr_decl(width, "%u");
    efifb_attr_decl(depth, "%u");
    static struct attribute *efifb_attrs[] = {
    &dev_attr_base.attr,
    &dev_attr_linelength.attr,
    &dev_attr_width.attr,
    &dev_attr_height.attr,
    &dev_attr_depth.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(efifb);
#[no_mangle]
unsafe extern "C" fn efifb_probe(dev: *mut platform_device) -> c_int {
    static int efifb_probe(struct platform_device *dev)
    {
    struct sysfb_display_info *dpy;
    struct screen_info *si;
    struct fb_info *info;
    struct efifb_par *par;
    int err, orientation;
    unsigned int size_vmode;
    unsigned int size_remap;
    unsigned int size_total;
    char *option = core::ptr::null_mut();
    efi_memory_desc_t md;
//
// If we fail probing the device, the kernel might try a different
// driver. We get a copy of the attached screen_info, so that we can
// modify its values without affecting later drivers.
//
    dpy = dev_get_platdata(&dev.dev);
    if (!dpy)
    return -ENODEV;
    si = devm_kmemdup(&dev.dev, &dpy.screen, sizeof(*si), GFP_KERNEL);
    if (!si)
    return -ENOMEM;
    dev_set_drvdata(&dev.dev, si);
    if (si.orig_video_isVGA != VIDEO_TYPE_EFI)
    return -ENODEV;
    if (fb_get_options("efifb", &option))
    return -ENODEV;
    efifb_setup(si, option);
    kfree(option);
// We don't get linelength from UGA Draw Protocol, only from
// EFI Graphics Protocol.  So if it's not in DMI, and it's not
// passed in from the user, we really can't use the framebuffer.
//
    if (!si.lfb_linelength)
    return -ENODEV;
    if (!si.lfb_depth)
    si.lfb_depth = 32;
    if (!si.pages)
    si.pages = 1;
    if (!fb_base_is_valid(si)) {
    printk(KERN_DEBUG "efifb: invalid framebuffer address\n");
    return -ENODEV;
    }
    printk(KERN_INFO "efifb: probing for efifb\n");
// just assume they're all unset if any are
    if (!si.blue_size) {
    si.blue_size = 8;
    si.blue_pos = 0;
    si.green_size = 8;
    si.green_pos = 8;
    si.red_size = 8;
    si.red_pos = 16;
    si.rsvd_size = 8;
    si.rsvd_pos = 24;
    }
    efifb_fix.smem_start = __screen_info_lfb_base(si);
    efifb_defined.bits_per_pixel = si.lfb_depth;
    efifb_defined.xres = si.lfb_width;
    efifb_defined.yres = si.lfb_height;
    efifb_fix.line_length = si.lfb_linelength;
// size_vmode -- that is the amount of memory needed for the
// used video mode, i.e. the minimum amount of
// memory we need.
    size_vmode = efifb_defined.yres * efifb_fix.line_length;
// size_total -- all video memory we have. Used for
// entries, ressource allocation and bounds
// checking.
    size_total = si.lfb_size;
    if (size_total < size_vmode)
    size_total = size_vmode;
// size_remap -- the amount of video memory we are going to
// use for efifb.  With modern cards it is no
// option to simply use size_total as that
// wastes plenty of kernel address space.
    size_remap  = size_vmode * 2;
    if (size_remap > size_total)
    size_remap = size_total;
    if (size_remap % PAGE_SIZE)
    size_remap += PAGE_SIZE - (size_remap % PAGE_SIZE);
    efifb_fix.smem_len = size_remap;
    if (request_mem_region(efifb_fix.smem_start, size_remap, "efifb")) {
    request_mem_succeeded = true;
    } else {
// We cannot make this fatal. Sometimes this comes from magic
    spaces our resource handlers simply don't know about */
    pr_warn("efifb: cannot reserve video memory at 0x%lx\n",
    efifb_fix.smem_start);
    }
    info = framebuffer_alloc(sizeof(*par), &dev.dev);
    if (!info) {
    err = -ENOMEM;
    goto err_release_mem;
    }
    par = info.par;
    info.pseudo_palette = par.pseudo_palette;
    par.base = efifb_fix.smem_start;
    par.size = size_remap;
    if (efi_enabled(EFI_MEMMAP) &&
    !efi_mem_desc_lookup(efifb_fix.smem_start, &md)) {
    if ((efifb_fix.smem_start + efifb_fix.smem_len) >
    (md.phys_addr + (md.num_pages << EFI_PAGE_SHIFT))) {
    pr_err("efifb: video memory @ 0x%lx spans multiple EFI memory regions\n",
    efifb_fix.smem_start);
    err = -EIO;
    goto err_release_fb;
    }
//
// If the UEFI memory map covers the efifb region, we may only
// remap it using the attributes the memory map prescribes.
//
    md.attribute &= EFI_MEMORY_UC | EFI_MEMORY_WC |
    EFI_MEMORY_WT | EFI_MEMORY_WB;
    if (md.attribute) {
    mem_flags |= EFI_MEMORY_WT | EFI_MEMORY_WB;
    mem_flags &= md.attribute;
    }
    }
    if (mem_flags & EFI_MEMORY_WC)
    info.screen_base = ioremap_wc(efifb_fix.smem_start,
    efifb_fix.smem_len);
#[no_mangle]
pub unsafe extern "C" fn if(EFI_MEMORY_UC: mem_flags &) -> else {
    else if (mem_flags & EFI_MEMORY_UC)
    info.screen_base = ioremap(efifb_fix.smem_start,
    efifb_fix.smem_len);
#[no_mangle]
pub unsafe extern "C" fn if(EFI_MEMORY_WT: mem_flags &) -> else {
    else if (mem_flags & EFI_MEMORY_WT)
    info.screen_base = memremap(efifb_fix.smem_start,
    efifb_fix.smem_len, MEMREMAP_WT);
#[no_mangle]
pub unsafe extern "C" fn if(EFI_MEMORY_WB: mem_flags &) -> else {
    else if (mem_flags & EFI_MEMORY_WB)
    info.screen_base = memremap(efifb_fix.smem_start,
    efifb_fix.smem_len, MEMREMAP_WB);
    if (!info.screen_base) {
    pr_err("efifb: abort, cannot remap video memory 0x%x @ 0x%lx\n",
    efifb_fix.smem_len, efifb_fix.smem_start);
    err = -EIO;
    goto err_release_fb;
    }
    efifb_show_boot_graphics(info, si);
    pr_info("efifb: framebuffer at 0x%lx, using %dk, total %dk\n",
    efifb_fix.smem_start, size_remap/1024, size_total/1024);
    pr_info("efifb: mode is %dx%dx%d, linelength=%d, pages=%d\n",
    efifb_defined.xres, efifb_defined.yres,
    efifb_defined.bits_per_pixel, efifb_fix.line_length,
    si.pages);
    efifb_defined.xres_virtual = efifb_defined.xres;
    efifb_defined.yres_virtual = efifb_fix.smem_len /
    efifb_fix.line_length;
    pr_info("efifb: scrolling: redraw\n");
    efifb_defined.yres_virtual = efifb_defined.yres;
// some dummy values for timing to make fbset happy
    efifb_defined.pixclock     = 10000000 / efifb_defined.xres *
    1000 / efifb_defined.yres;
    efifb_defined.left_margin  = (efifb_defined.xres / 8) & 0xf8;
    efifb_defined.hsync_len    = (efifb_defined.xres / 8) & 0xf8;
    efifb_defined.red.offset    = si.red_pos;
    efifb_defined.red.length    = si.red_size;
    efifb_defined.green.offset  = si.green_pos;
    efifb_defined.green.length  = si.green_size;
    efifb_defined.blue.offset   = si.blue_pos;
    efifb_defined.blue.length   = si.blue_size;
    efifb_defined.transp.offset = si.rsvd_pos;
    efifb_defined.transp.length = si.rsvd_size;
    pr_info("efifb: %s: "
    "size=%d:%d:%d:%d, shift=%d:%d:%d:%d\n",
    "Truecolor",
    si.rsvd_size,
    si.red_size,
    si.green_size,
    si.blue_size,
    si.rsvd_pos,
    si.red_pos,
    si.green_pos,
    si.blue_pos);
    efifb_fix.ypanstep  = 0;
    efifb_fix.ywrapstep = 0;
    info.fbops = &efifb_ops;
    info.var = efifb_defined;
    info.fix = efifb_fix;
    orientation = drm_get_panel_orientation_quirk(efifb_defined.xres,
    efifb_defined.yres);
    switch (orientation) {
    default:
    info.fbcon_rotate_hint = FB_ROTATE_UR;
    break;
    case DRM_MODE_PANEL_ORIENTATION_BOTTOM_UP:
    info.fbcon_rotate_hint = FB_ROTATE_UD;
    break;
    case DRM_MODE_PANEL_ORIENTATION_LEFT_UP:
    info.fbcon_rotate_hint = FB_ROTATE_CCW;
    break;
    case DRM_MODE_PANEL_ORIENTATION_RIGHT_UP:
    info.fbcon_rotate_hint = FB_ROTATE_CW;
    break;
    }
    err = fb_alloc_cmap(&info.cmap, 256, 0);
    if (err < 0) {
    pr_err("efifb: cannot allocate colormap\n");
    goto err_unmap;
    }
    err = devm_aperture_acquire_for_platform_device(dev, par.base, par.size);
    if (err) {
    pr_err("efifb: cannot acquire aperture\n");
    goto err_fb_dealloc_cmap;
    }
    err = devm_register_framebuffer(&dev.dev, info);
    if (err < 0) {
    pr_err("efifb: cannot register framebuffer\n");
    goto err_fb_dealloc_cmap;
    }
    fb_info(info, "%s frame buffer device\n", info.fix.id);
    return 0;
    err_fb_dealloc_cmap:
    fb_dealloc_cmap(&info.cmap);
    err_unmap:
    if (mem_flags & (EFI_MEMORY_UC | EFI_MEMORY_WC))
    iounmap(info.screen_base);
    else
    memunmap(info.screen_base);
    err_release_fb:
    framebuffer_release(info);
    err_release_mem:
    if (request_mem_succeeded)
    release_mem_region(efifb_fix.smem_start, size_total);
    return err;
    }
    static struct platform_driver efifb_driver = {
    .driver = {
    .name = "efi-framebuffer",
    .dev_groups = efifb_groups,
    },
    .probe = efifb_probe,
    };
    builtin_platform_driver(efifb_driver);
