//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fbmem.c
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
// linux/drivers/video/fbmem.c
//
// Copyright (C) 1994 Martin Schaller
//
// 2001 - Documented with DocBook
// - Brad Douglas <brad@neruo.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Frame buffer device initialization and setup routines
//

    struct class *fb_class;
    DEFINE_MUTEX(registration_lock);
    struct fb_info *registered_fb[FB_MAX] __read_mostly;
    int num_registered_fb __read_mostly;

    for (i = 0; i < FB_MAX; i++)		\
    if (!registered_fb[i]) {} else
    struct fb_info *get_fb_info(unsigned int idx)
    {
    struct fb_info *fb_info;
    if (idx >= FB_MAX)
    return ERR_PTR(-ENODEV);
    mutex_lock(&registration_lock);
    fb_info = registered_fb[idx];
    if (fb_info)
    refcount_inc(&fb_info.count);
    mutex_unlock(&registration_lock);
    return fb_info;
    }
#[no_mangle]
pub unsafe extern "C" fn put_fb_info(fb_info: *mut fb_info) {
    void put_fb_info(struct fb_info *fb_info)
    {
    if (!refcount_dec_and_test(&fb_info.count))
    return;
    if (fb_info.fbops.fb_destroy)
    fb_info.fbops.fb_destroy(fb_info);
    }
//
// Helpers
//
    int fb_get_color_depth(struct fb_var_screeninfo *var,
    struct fb_fix_screeninfo *fix)
    {
    let mut depth: c_int = 0;
    if (fix.visual == FB_VISUAL_MONO01 ||
    fix.visual == FB_VISUAL_MONO10)
    depth = 1;
    else {
    if (var.green.length == var.blue.length &&
    var.green.length == var.red.length &&
    var.green.offset == var.blue.offset &&
    var.green.offset == var.red.offset)
    depth = var.green.length;
    else
    depth = var.green.length + var.red.length +
    var.blue.length;
    }
    return depth;
    }
    EXPORT_SYMBOL(fb_get_color_depth);
//
// Data padding functions.
//
#[no_mangle]
pub unsafe extern "C" fn fb_pad_aligned_buffer(dst: *mut u8, d_pitch: u32, src: *const u8, s_pitch: u32, height: u32) {
    void fb_pad_aligned_buffer(u8 *dst, u32 d_pitch, const u8 *src, u32 s_pitch, u32 height)
    {
    __fb_pad_aligned_buffer(dst, d_pitch, src, s_pitch, height);
    }
    EXPORT_SYMBOL(fb_pad_aligned_buffer);
    void fb_pad_unaligned_buffer(u8 *dst, u32 d_pitch, const u8 *src, u32 idx, u32 height,
    u32 shift_high, u32 shift_low, u32 mod)
    {
    let mut mask: u8 = (u8) (0xff << shift_high), tmp;
    int i, j;
    for (i = height; i--; ) {
    for (j = 0; j < idx; j++) {
    tmp = dst[j];
    tmp &= mask;
    tmp |= *src >> shift_low;
    dst[j] = tmp;
    tmp = *src << shift_high;
    dst[j+1] = tmp;
    src++;
    }
    tmp = dst[idx];
    tmp &= mask;
    tmp |= *src >> shift_low;
    dst[idx] = tmp;
    if (shift_high < mod) {
    tmp = *src << shift_high;
    dst[idx+1] = tmp;
    }
    src++;
    dst += d_pitch;
    }
    }
    EXPORT_SYMBOL(fb_pad_unaligned_buffer);
//
// we need to lock this section since fb_cursor
// may use fb_imageblit()
//
#[no_mangle]
pub unsafe extern "C" fn fb_get_buffer_offset(info: *mut fb_info, buf: *mut fb_pixmap, size: u32) -> *mut c_char {
    char* fb_get_buffer_offset(struct fb_info *info, struct fb_pixmap *buf, u32 size)
    {
    let mut align: u32 = buf.buf_align - 1, offset;
    char *addr = buf.addr;
// If IO mapped, we need to sync before access, no sharing of
// the pixmap is done
//
    if (buf.flags & FB_PIXMAP_IO) {
    if (info.fbops.fb_sync && (buf.flags & FB_PIXMAP_SYNC))
    info.fbops.fb_sync(info);
    return addr;
    }
// See if we fit in the remaining pixmap space
    offset = buf.offset + align;
    offset &= ~align;
    if (offset + size > buf.size) {
// We do not fit. In order to be able to re-use the buffer,
// we must ensure no asynchronous DMA'ing or whatever operation
// is in progress, we sync for that.
//
    if (info.fbops.fb_sync && (buf.flags & FB_PIXMAP_SYNC))
    info.fbops.fb_sync(info);
    offset = 0;
    }
    buf.offset = offset + size;
    addr += offset;
    return addr;
    }
    EXPORT_SYMBOL(fb_get_buffer_offset);
    int
    fb_pan_display(struct fb_info *info, struct fb_var_screeninfo *var)
    {
    struct fb_fix_screeninfo *fix = &info.fix;
    let mut yres: c_uint = info.var.yres;
    let mut err: c_int = 0;
    if (var.yoffset > 0) {
    if (var.vmode & FB_VMODE_YWRAP) {
    if (!fix.ywrapstep || (var.yoffset % fix.ywrapstep))
    err = -EINVAL;
    else
    yres = 0;
    } else if (!fix.ypanstep || (var.yoffset % fix.ypanstep))
    err = -EINVAL;
    }
    if (var.xoffset > 0 && (!fix.xpanstep ||
    (var.xoffset % fix.xpanstep)))
    err = -EINVAL;
    if (err || !info.fbops.fb_pan_display ||
    var.yoffset > info.var.yres_virtual - yres ||
    var.xoffset > info.var.xres_virtual - info.var.xres)
    return -EINVAL;
    if ((err = info.fbops.fb_pan_display(var, info)))
    return err;
    info.var.xoffset = var.xoffset;
    info.var.yoffset = var.yoffset;
    if (var.vmode & FB_VMODE_YWRAP)
    info.var.vmode |= FB_VMODE_YWRAP;
    else
    info.var.vmode &= ~FB_VMODE_YWRAP;
    return 0;
    }
    EXPORT_SYMBOL(fb_pan_display);
    static int fb_check_caps(struct fb_info *info, struct fb_var_screeninfo *var,
    u32 activate)
    {
    struct fb_blit_caps caps, fbcaps;
    let mut err: c_int = 0;
    memset(&caps, 0, sizeof(caps));
    memset(&fbcaps, 0, sizeof(fbcaps));
    caps.flags = (activate & FB_ACTIVATE_ALL) ? 1 : 0;
    fbcon_get_requirement(info, &caps);
    info.fbops.fb_get_caps(info, &fbcaps, var);
    if (!bitmap_subset(caps.x, fbcaps.x, FB_MAX_BLIT_WIDTH) ||
    !bitmap_subset(caps.y, fbcaps.y, FB_MAX_BLIT_HEIGHT) ||
    (fbcaps.len < caps.len))
    err = -EINVAL;
    return err;
    }
    static void fb_lcd_notify_mode_change(struct fb_info *info,
    struct fb_videomode *mode)
    {
    lcd_notify_mode_change_all(info.device, mode.xres, mode.yres);
    }
    int
    fb_set_var(struct fb_info *info, struct fb_var_screeninfo *var)
    {
    let mut ret: c_int = 0;
    u32 activate;
    struct fb_var_screeninfo old_var;
    struct fb_videomode mode;
    u32 unused;
    if (var.activate & FB_ACTIVATE_INV_MODE) {
    struct fb_videomode mode1, mode2;
    fb_var_to_videomode(&mode1, var);
    fb_var_to_videomode(&mode2, &info.var);
// make sure we don't delete the videomode of current var
    ret = fb_mode_is_equal(&mode1, &mode2);
    if (!ret) {
    ret = fbcon_mode_deleted(info, &mode1);
    if (!ret) {
    if (info.mode && fb_mode_is_equal(info.mode, &mode1))
    info.mode = core::ptr::null_mut();
    fb_delete_videomode(&mode1, &info.modelist);
    }
    }
    return ret ? -EINVAL : 0;
    }
    if (!(var.activate & FB_ACTIVATE_FORCE) &&
    !memcmp(&info.var, var, sizeof(struct fb_var_screeninfo)))
    return 0;
    activate = var.activate;
// When using FOURCC mode, make sure the red, green, blue and
// transp fields are set to 0.
//
    if ((info.fix.capabilities & FB_CAP_FOURCC) &&
    var.grayscale > 1) {
    if (var.red.offset     || var.green.offset    ||
    var.blue.offset    || var.transp.offset   ||
    var.red.length     || var.green.length    ||
    var.blue.length    || var.transp.length   ||
    var.red.msb_right  || var.green.msb_right ||
    var.blue.msb_right || var.transp.msb_right)
    return -EINVAL;
    }
    if (!info.fbops.fb_check_var) {
// var = info->var;
    return 0;
    }
// bitfill_aligned() assumes that it's at least 8x8
    if (var.xres < 8 || var.yres < 8)
    return -EINVAL;
// Too huge resolution causes multiplication overflow.
    if (check_mul_overflow(var.xres, var.yres, &unused) ||
    check_mul_overflow(var.xres_virtual, var.yres_virtual, &unused))
    return -EINVAL;
    ret = info.fbops.fb_check_var(var, info);
    if (ret)
    return ret;
// verify that virtual resolution >= physical resolution
    if (var.xres_virtual < var.xres ||
    var.yres_virtual < var.yres) {
    pr_warn("WARNING: fbcon: Driver '%s' missed to adjust virtual screen size (%ux%u vs. %ux%u)\n",
    info.fix.id,
    var.xres_virtual, var.yres_virtual,
    var.xres, var.yres);
    return -EINVAL;
    }
    if ((var.activate & FB_ACTIVATE_MASK) != FB_ACTIVATE_NOW)
    return 0;
    if (info.fbops.fb_get_caps) {
    ret = fb_check_caps(info, var, activate);
    if (ret)
    return ret;
    }
    old_var = info.var;
    info.var = *var;
    if (info.fbops.fb_set_par) {
    ret = info.fbops.fb_set_par(info);
    if (ret) {
    info.var = old_var;
    printk(KERN_WARNING "detected "
    "fb_set_par error, "
    "error code: %d\n", ret);
    return ret;
    }
    }
    fb_pan_display(info, &info.var);
    fb_set_cmap(&info.cmap, info);
    fb_var_to_videomode(&mode, &info.var);
    if (info.modelist.prev && info.modelist.next &&
    !list_empty(&info.modelist))
    ret = fb_add_videomode(&mode, &info.modelist);
    if (ret) {
    info.var = old_var;
    return ret;
    }
    fb_lcd_notify_mode_change(info, &mode);
    return 0;
    }
    EXPORT_SYMBOL(fb_set_var);
#[no_mangle]
pub unsafe extern "C" fn fb_set_var_from_user(info: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int {
    int fb_set_var_from_user(struct fb_info *info, struct fb_var_screeninfo *var)
    {
    let mut ret: c_int = fbcon_modechange_possible(info, var);
    if (!ret)
    ret = fb_set_var(info, var);
    if (!ret)
    fbcon_update_vcs(info, var.activate & FB_ACTIVATE_ALL);
    return ret;
    }
    EXPORT_SYMBOL(fb_set_var_from_user);
#[no_mangle]
unsafe extern "C" fn fb_lcd_notify_blank(info: *mut fb_info) {
    static void fb_lcd_notify_blank(struct fb_info *info)
    {
    int power;
    switch (info.blank) {
    case FB_BLANK_UNBLANK:
    power = LCD_POWER_ON;
    break;
// deprecated; TODO: should become 'off'
    case FB_BLANK_NORMAL:
    power = LCD_POWER_REDUCED;
    break;
    case FB_BLANK_VSYNC_SUSPEND:
    power = LCD_POWER_REDUCED_VSYNC_SUSPEND;
    break;
// 'off'
    case FB_BLANK_HSYNC_SUSPEND:
    case FB_BLANK_POWERDOWN:
    default:
    power = LCD_POWER_OFF;
    break;
    }
    lcd_notify_blank_all(info.device, power);
    }
#[no_mangle]
unsafe extern "C" fn fb_ledtrig_backlight_notify_blank(info: *mut fb_info) {
    static void fb_ledtrig_backlight_notify_blank(struct fb_info *info)
    {
    if (info.blank == FB_BLANK_UNBLANK)
    ledtrig_backlight_blank(false);
    else
    ledtrig_backlight_blank(true);
    }
#[no_mangle]
pub unsafe extern "C" fn fb_blank(info: *mut fb_info, blank: c_int) -> c_int {
    int fb_blank(struct fb_info *info, int blank)
    {
    let mut old_blank: c_int = info.blank;
    int ret;
    if (!info.fbops.fb_blank)
    return -EINVAL;
    if (blank > FB_BLANK_POWERDOWN)
    blank = FB_BLANK_POWERDOWN;
    info.blank = blank;
    ret = info.fbops.fb_blank(blank, info);
    if (ret)
    goto err;
    fb_bl_notify_blank(info, old_blank);
    fb_lcd_notify_blank(info);
    fb_ledtrig_backlight_notify_blank(info);
    return 0;
    err:
    info.blank = old_blank;
    return ret;
    }
    EXPORT_SYMBOL(fb_blank);
#[no_mangle]
pub unsafe extern "C" fn fb_blank_from_user(info: *mut fb_info, blank: c_int) -> c_int {
    int fb_blank_from_user(struct fb_info *info, int blank)
    {
    let mut ret: c_int = fb_blank(info, blank);
// might again call into fb_blank
    fbcon_fb_blanked(info, blank);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fb_check_foreignness(fi: *mut fb_info) -> c_int {
    static int fb_check_foreignness(struct fb_info *fi)
    {
    let mut foreign_endian: bool = fi.flags & FBINFO_FOREIGN_ENDIAN;
    fi.flags &= ~FBINFO_FOREIGN_ENDIAN;

    fi.flags |= foreign_endian ? 0 : FBINFO_BE_MATH;

    fi.flags |= foreign_endian ? FBINFO_BE_MATH : 0;

    if (fi.flags & FBINFO_BE_MATH && !fb_be_math(fi)) {
    pr_err("%s: enable CONFIG_FB_BIG_ENDIAN to "
    "support this framebuffer\n", fi.fix.id);
    return -ENOSYS;
    } else if (!(fi.flags & FBINFO_BE_MATH) && fb_be_math(fi)) {
    pr_err("%s: enable CONFIG_FB_LITTLE_ENDIAN to "
    "support this framebuffer\n", fi.fix.id);
    return -ENOSYS;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_register_framebuffer(fb_info: *mut fb_info) -> c_int {
    static int do_register_framebuffer(struct fb_info *fb_info)
    {
    int i, err = 0;
    struct fb_videomode mode;
    if (fb_check_foreignness(fb_info))
    return -ENOSYS;
    if (num_registered_fb == FB_MAX)
    return -ENXIO;
    for (i = 0 ; i < FB_MAX; i++)
    if (!registered_fb[i])
    break;
    if (i >= FB_MAX)
    return -ENXIO;
    if (!fb_info.modelist.prev || !fb_info.modelist.next)
    INIT_LIST_HEAD(&fb_info.modelist);
    fb_var_to_videomode(&mode, &fb_info.var);
    err = fb_add_videomode(&mode, &fb_info.modelist);
    if (err < 0)
    return err;
    fb_info.node = i;
    refcount_set(&fb_info.count, 1);
    mutex_init(&fb_info.lock);
    mutex_init(&fb_info.mm_lock);
//
// With an fb_blank callback present, we assume that the
// display is blank, so that fb_blank() enables it on the
// first modeset.
//
    if (fb_info.fbops.fb_blank)
    fb_info.blank = FB_BLANK_POWERDOWN;
    fb_device_create(fb_info);
    if (fb_info.pixmap.addr == core::ptr::null_mut()) {
    fb_info.pixmap.addr = kmalloc(FBPIXMAPSIZE, GFP_KERNEL);
    if (fb_info.pixmap.addr) {
    fb_info.pixmap.size = FBPIXMAPSIZE;
    fb_info.pixmap.buf_align = 1;
    fb_info.pixmap.scan_align = 1;
    fb_info.pixmap.access_align = 32;
    fb_info.pixmap.flags = FB_PIXMAP_DEFAULT;
    }
    }
    fb_info.pixmap.offset = 0;
    if (bitmap_empty(fb_info.pixmap.blit_x, FB_MAX_BLIT_WIDTH))
    bitmap_fill(fb_info.pixmap.blit_x, FB_MAX_BLIT_WIDTH);
    if (bitmap_empty(fb_info.pixmap.blit_y, FB_MAX_BLIT_HEIGHT))
    bitmap_fill(fb_info.pixmap.blit_y, FB_MAX_BLIT_HEIGHT);
    if (fb_info.skip_vt_switch)
    pm_vt_switch_required(fb_info.device, false);
    else
    pm_vt_switch_required(fb_info.device, true);
    num_registered_fb++;
    registered_fb[i] = fb_info;

    {
    struct fb_event event;
    event.info = fb_info;
    fb_notifier_call_chain(FB_EVENT_FB_REGISTERED, &event);
    }

    return fbcon_fb_registered(fb_info);
    }
#[no_mangle]
unsafe extern "C" fn unbind_console(fb_info: *mut fb_info) {
    static void unbind_console(struct fb_info *fb_info)
    {
    let mut i: c_int = fb_info.node;
    if (WARN_ON(i < 0 || i >= FB_MAX || registered_fb[i] != fb_info))
    return;
    fbcon_fb_unbind(fb_info);
    }
#[no_mangle]
unsafe extern "C" fn unlink_framebuffer(fb_info: *mut fb_info) {
    static void unlink_framebuffer(struct fb_info *fb_info)
    {
    int i;
    i = fb_info.node;
    if (WARN_ON(i < 0 || i >= FB_MAX || registered_fb[i] != fb_info))
    return;
    fb_device_destroy(fb_info);
    pm_vt_switch_unregister(fb_info.device);
    unbind_console(fb_info);
    }
#[no_mangle]
unsafe extern "C" fn do_unregister_framebuffer(fb_info: *mut fb_info) {
    static void do_unregister_framebuffer(struct fb_info *fb_info)
    {
    unlink_framebuffer(fb_info);
    if (fb_info.pixmap.addr &&
    (fb_info.pixmap.flags & FB_PIXMAP_DEFAULT)) {
    kfree(fb_info.pixmap.addr);
    fb_info.pixmap.addr = core::ptr::null_mut();
    }
    fbcon_delete_modelist(&fb_info.modelist);
    fb_destroy_modelist(&fb_info.modelist);
    registered_fb[fb_info.node] = core::ptr::null_mut();
    num_registered_fb--;

    {
    struct fb_event event;
    event.info = fb_info;
    fb_notifier_call_chain(FB_EVENT_FB_UNREGISTERED, &event);
    }

    fbcon_fb_unregistered(fb_info);
// this may free fb info
    put_fb_info(fb_info);
    }
//
// register_framebuffer - registers a frame buffer device
// @fb_info: frame buffer info structure
//
// Registers a frame buffer device @fb_info.
//
// Returns negative errno on error, or zero for success.
//
    int
    register_framebuffer(struct fb_info *fb_info)
    {
    int ret;
    mutex_lock(&registration_lock);
    ret = do_register_framebuffer(fb_info);
    mutex_unlock(&registration_lock);
    return ret;
    }
    EXPORT_SYMBOL(register_framebuffer);
//
// unregister_framebuffer - releases a frame buffer device
// @fb_info: frame buffer info structure
//
// Unregisters a frame buffer device @fb_info.
//
// Returns negative errno on error, or zero for success.
//
// This function will also notify the framebuffer console
// to release the driver.
//
// This is meant to be called within a driver's module_exit()
// function. If this is called outside module_exit(), ensure
// that the driver implements fb_open() and fb_release() to
// check that no processes are using the device.
//
    void
    unregister_framebuffer(struct fb_info *fb_info)
    {
    mutex_lock(&registration_lock);
    do_unregister_framebuffer(fb_info);
    mutex_unlock(&registration_lock);
    }
    EXPORT_SYMBOL(unregister_framebuffer);
#[no_mangle]
unsafe extern "C" fn devm_unregister_framebuffer(data: *mut c_void) {
    static void devm_unregister_framebuffer(void *data)
    {
    struct fb_info *info = data;
    unregister_framebuffer(info);
    }
//
// devm_register_framebuffer - resource-managed frame buffer device registration
// @dev: device the framebuffer belongs to
// @fb_info: frame buffer info structure
//
// Registers a frame buffer device @fb_info to device @dev.
//
// Returns negative errno on error, or zero for success.
//
    int
    devm_register_framebuffer(struct device *dev, struct fb_info *fb_info)
    {
    int ret;
    ret = register_framebuffer(fb_info);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, devm_unregister_framebuffer, fb_info);
    }
    EXPORT_SYMBOL(devm_register_framebuffer);
//
// fb_set_suspend - low level driver signals suspend
// @info: framebuffer affected
// @state: 0 = resuming, !=0 = suspending
//
// This is meant to be used by low level drivers to
// signal suspend/resume to the core & clients.
// It must be called with the console semaphore held
//
#[no_mangle]
pub unsafe extern "C" fn fb_set_suspend(info: *mut fb_info, state: c_int) {
    void fb_set_suspend(struct fb_info *info, int state)
    {
    WARN_CONSOLE_UNLOCKED();
    if (state) {
    fbcon_suspended(info);
    info.state = FBINFO_STATE_SUSPENDED;
    } else {
    info.state = FBINFO_STATE_RUNNING;
    fbcon_resumed(info);
    }
    }
    EXPORT_SYMBOL(fb_set_suspend);
//
// fb_switch_outputs - framebuffer got the outputs from vga-switcheroo
// @info: framebuffer
//
#[no_mangle]
pub unsafe extern "C" fn fb_switch_outputs(info: *mut fb_info) {
    void fb_switch_outputs(struct fb_info *info)
    {
    fbcon_remap_all(info);
    }
    EXPORT_SYMBOL(fb_switch_outputs);
#[no_mangle]
unsafe extern "C" fn fbmem_init() -> int __init {
    static int __init fbmem_init(void)
    {
    int ret;
    fb_class = class_create("graphics");
    if (IS_ERR(fb_class)) {
    ret = PTR_ERR(fb_class);
    pr_err("Unable to create fb class; errno = %d\n", ret);
    goto err_fb_class;
    }
    ret = fb_init_procfs();
    if (ret)
    goto err_class_destroy;
    ret = fb_register_chrdev();
    if (ret)
    goto err_fb_cleanup_procfs;
    fb_console_init();
    return 0;
    err_fb_cleanup_procfs:
    fb_cleanup_procfs();
    err_class_destroy:
    class_destroy(fb_class);
    err_fb_class:
    fb_class = core::ptr::null_mut();
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn fbmem_exit() -> void __exit {
    static void __exit fbmem_exit(void)
    {
    fb_console_exit();
    fb_unregister_chrdev();
    fb_cleanup_procfs();
    class_destroy(fb_class);
    }
    module_init(fbmem_init);
    module_exit(fbmem_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Framebuffer base");

    subsys_initcall(fbmem_init);

#[no_mangle]
pub unsafe extern "C" fn fb_new_modelist(info: *mut fb_info) -> c_int {
    int fb_new_modelist(struct fb_info *info)
    {
    let mut var: fb_var_screeninfo = info.var;
    struct list_head *pos, *n;
    struct fb_modelist *modelist;
    struct fb_videomode *m, mode;
    int err;
    list_for_each_safe(pos, n, &info.modelist) {
    modelist = list_entry(pos, struct fb_modelist, list);
    m = &modelist.mode;
    fb_videomode_to_var(&var, m);
    var.activate = FB_ACTIVATE_TEST;
    err = fb_set_var(info, &var);
    fb_var_to_videomode(&mode, &var);
    if (err || !fb_mode_is_equal(m, &mode)) {
    list_del(pos);
    kfree(pos);
    }
    }
    if (list_empty(&info.modelist))
    return 1;
//
// The new modelist may not contain the current mode (info->var), and
// fbcon_new_modelist() below only re-points consoles mapped to this
// framebuffer. Add the current mode here so info->var keeps a match
// even when fbcon is unbound.
//
    if (!fb_match_mode(&info.var, &info.modelist)) {
    fb_var_to_videomode(&mode, &info.var);
    if (fb_add_videomode(&mode, &info.modelist))
    return 1;
    }
    fbcon_new_modelist(info);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fb_modesetting_disabled(drvname: *const c_char) -> bool {
    bool fb_modesetting_disabled(const char *drvname)
    {
    let mut fwonly: bool = video_firmware_drivers_only();
    if (fwonly)
    pr_warn("Driver %s not loading because of nomodeset parameter\n",
    drvname);
    return fwonly;
    }
    EXPORT_SYMBOL(fb_modesetting_disabled);
    MODULE_LICENSE("GPL");
