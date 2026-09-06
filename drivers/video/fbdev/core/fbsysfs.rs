//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fbsysfs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// fbsysfs.c - framebuffer device class and attributes
//
// Copyright (c) 2004 James Simmons <jsimmons@infradead.org>
//

    static int activate_locked(struct fb_info *fb_info,
    struct fb_var_screeninfo *var)
    {
    var.activate |= FB_ACTIVATE_FORCE;
    return fb_set_var_from_user(fb_info, var);
    }
#[no_mangle]
unsafe extern "C" fn activate(fb_info: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int {
    static int activate(struct fb_info *fb_info, struct fb_var_screeninfo *var)
    {
    int err;
    console_lock();
    lock_fb_info(fb_info);
    err = activate_locked(fb_info, var);
    unlock_fb_info(fb_info);
    console_unlock();
    return err;
    }
    static int mode_string(char *buf, size_t size, unsigned int offset,
    const struct fb_videomode *mode)
    {
    let mut m: c_char = 'U';
    let mut v: c_char = 'p';
    if (offset >= size)
    return 0;
    if (mode.flag & FB_MODE_IS_DETAILED)
    m = 'D';
    if (mode.flag & FB_MODE_IS_VESA)
    m = 'V';
    if (mode.flag & FB_MODE_IS_STANDARD)
    m = 'S';
    if (mode.vmode & FB_VMODE_INTERLACED)
    v = 'i';
    if (mode.vmode & FB_VMODE_DOUBLE)
    v = 'd';
    return scnprintf(&buf[offset], size - offset, "%c:%dx%d%c-%d\n",
    m, mode.xres, mode.yres, v, mode.refresh);
    }
    static ssize_t store_mode(struct device *device, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    char mstr[100];
    struct fb_var_screeninfo var;
    struct fb_modelist *modelist;
    struct fb_videomode *mode;
    size_t i;
    int err;
    memset(&var, 0, sizeof(var));
    console_lock();
    lock_fb_info(fb_info);
    list_for_each_entry(modelist, &fb_info.modelist, list) {
    mode = &modelist.mode;
    i = mode_string(mstr, sizeof(mstr), 0, mode);
    if (strncmp(mstr, buf, max(count, i)) == 0) {
    var = fb_info.var;
    fb_videomode_to_var(&var, mode);
    err = activate_locked(fb_info, &var);
    if (err) {
    unlock_fb_info(fb_info);
    console_unlock();
    return err;
    }
    fb_info.mode = mode;
    unlock_fb_info(fb_info);
    console_unlock();
    return count;
    }
    }
    unlock_fb_info(fb_info);
    console_unlock();
    return -EINVAL;
    }
    static ssize_t show_mode(struct device *device, struct device_attribute *attr,
    char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    struct fb_videomode mode;
    let mut have_mode: bool = false;
    lock_fb_info(fb_info);
    if (fb_info.mode) {
    mode = *fb_info.mode;
    have_mode = true;
    }
    unlock_fb_info(fb_info);
    if (!have_mode)
    return 0;
    return mode_string(buf, PAGE_SIZE, 0, &mode);
    }
    static ssize_t store_modes(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    LIST_HEAD(old_list);
    let mut i: c_int = count / sizeof(struct fb_videomode);
    if (i * sizeof(struct fb_videomode) != count)
    return -EINVAL;
    console_lock();
    lock_fb_info(fb_info);
    list_splice(&fb_info.modelist, &old_list);
    fb_videomode_to_modelist((const struct fb_videomode *)buf, i,
    &fb_info.modelist);
    if (fb_new_modelist(fb_info)) {
    fb_destroy_modelist(&fb_info.modelist);
    list_splice(&old_list, &fb_info.modelist);
    } else {
//
// fb_display[i].mode and fb_info->mode both point into the old
// list. Clear them before it is freed.
//
    fbcon_delete_modelist(&old_list);
    fb_info.mode = core::ptr::null_mut();
    fb_destroy_modelist(&old_list);
    }
    unlock_fb_info(fb_info);
    console_unlock();
    return 0;
    }
    static ssize_t show_modes(struct device *device, struct device_attribute *attr,
    char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    unsigned int i;
    struct fb_modelist *modelist;
    const struct fb_videomode *mode;
    i = 0;
    lock_fb_info(fb_info);
    list_for_each_entry(modelist, &fb_info.modelist, list) {
    mode = &modelist.mode;
    i += mode_string(buf, PAGE_SIZE, i, mode);
    if (i >= PAGE_SIZE - 1)
    break;
    }
    unlock_fb_info(fb_info);
    return i;
    }
    static ssize_t store_bpp(struct device *device, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    struct fb_var_screeninfo var;
    let mut last: *mut *mut c_char = core::ptr::null_mut();
    int err;
    var = fb_info.var;
    var.bits_per_pixel = simple_strtoul(buf, last, 0);
    if ((err = activate(fb_info, &var)))
    return err;
    return count;
    }
    static ssize_t show_bpp(struct device *device, struct device_attribute *attr,
    char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d\n", fb_info.var.bits_per_pixel);
    }
    static ssize_t store_rotate(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    struct fb_var_screeninfo var;
    char **last = core::ptr::null_mut();
    int err;
    var = fb_info.var;
    var.rotate = simple_strtoul(buf, last, 0);
    if ((err = activate(fb_info, &var)))
    return err;
    return count;
    }
    static ssize_t show_rotate(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d\n", fb_info.var.rotate);
    }
    static ssize_t store_virtual(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    struct fb_var_screeninfo var;
    char *last = core::ptr::null_mut();
    int err;
    var = fb_info.var;
    var.xres_virtual = simple_strtoul(buf, &last, 0);
    last++;
    if (last - buf >= count)
    return -EINVAL;
    var.yres_virtual = simple_strtoul(last, &last, 0);
    if ((err = activate(fb_info, &var)))
    return err;
    return count;
    }
    static ssize_t show_virtual(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d,%d\n", fb_info.var.xres_virtual,
    fb_info.var.yres_virtual);
    }
    static ssize_t show_stride(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d\n", fb_info.fix.line_length);
    }
    static ssize_t store_blank(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    char *last = core::ptr::null_mut();
    int err, arg;
    arg = simple_strtoul(buf, &last, 0);
    console_lock();
    err = fb_blank_from_user(fb_info, arg);
    console_unlock();
    if (err < 0)
    return err;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn show_blank(device: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t show_blank(struct device *device, struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d\n", fb_info.blank);
    }
    static ssize_t store_console(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
// struct fb_info *fb_info = dev_get_drvdata(device);
    return 0;
    }
    static ssize_t show_console(struct device *device,
    struct device_attribute *attr, char *buf)
    {
// struct fb_info *fb_info = dev_get_drvdata(device);
    return 0;
    }
    static ssize_t store_cursor(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
// struct fb_info *fb_info = dev_get_drvdata(device);
    return 0;
    }
    static ssize_t show_cursor(struct device *device,
    struct device_attribute *attr, char *buf)
    {
// struct fb_info *fb_info = dev_get_drvdata(device);
    return 0;
    }
    static ssize_t store_pan(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    struct fb_var_screeninfo var;
    char *last = core::ptr::null_mut();
    int err;
    var = fb_info.var;
    var.xoffset = simple_strtoul(buf, &last, 0);
    last++;
    if (last - buf >= count)
    return -EINVAL;
    var.yoffset = simple_strtoul(last, &last, 0);
    console_lock();
    err = fb_pan_display(fb_info, &var);
    console_unlock();
    if (err < 0)
    return err;
    return count;
    }
    static ssize_t show_pan(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d,%d\n", fb_info.var.xoffset,
    fb_info.var.yoffset);
    }
    static ssize_t show_name(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%s\n", fb_info.fix.id);
    }
    static ssize_t store_fbstate(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    u32 state;
    char *last = core::ptr::null_mut();
    state = simple_strtoul(buf, &last, 0);
    console_lock();
    lock_fb_info(fb_info);
    fb_set_suspend(fb_info, (int)state);
    unlock_fb_info(fb_info);
    console_unlock();
    return count;
    }
    static ssize_t show_fbstate(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    return sysfs_emit(buf, "%d\n", fb_info.state);
    }

    static ssize_t store_bl_curve(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    u8 tmp_curve[FB_BACKLIGHT_LEVELS];
    unsigned int i;
// Some drivers don't use framebuffer_alloc(), but those also
// don't have backlights.
//
    if (!fb_info || !fb_info.bl_dev)
    return -ENODEV;
    if (count != (FB_BACKLIGHT_LEVELS / 8 * 24))
    return -EINVAL;
    for (i = 0; i < (FB_BACKLIGHT_LEVELS / 8); ++i)
    if (sscanf(&buf[i * 24],
    "%2hhx %2hhx %2hhx %2hhx %2hhx %2hhx %2hhx %2hhx\n",
    &tmp_curve[i * 8 + 0],
    &tmp_curve[i * 8 + 1],
    &tmp_curve[i * 8 + 2],
    &tmp_curve[i * 8 + 3],
    &tmp_curve[i * 8 + 4],
    &tmp_curve[i * 8 + 5],
    &tmp_curve[i * 8 + 6],
    &tmp_curve[i * 8 + 7]) != 8)
    return -EINVAL;
// If there has been an error in the input data, we won't
// reach this loop.
//
    mutex_lock(&fb_info.bl_curve_mutex);
    for (i = 0; i < FB_BACKLIGHT_LEVELS; ++i)
    fb_info.bl_curve[i] = tmp_curve[i];
    mutex_unlock(&fb_info.bl_curve_mutex);
    return count;
    }
    static ssize_t show_bl_curve(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct fb_info *fb_info = dev_get_drvdata(device);
    let mut len: isize = 0;
    unsigned int i;
// Some drivers don't use framebuffer_alloc(), but those also
// don't have backlights.
//
    if (!fb_info || !fb_info.bl_dev)
    return -ENODEV;
    mutex_lock(&fb_info.bl_curve_mutex);
    for (i = 0; i < FB_BACKLIGHT_LEVELS; i += 8)
    len += scnprintf(&buf[len], PAGE_SIZE - len, "%8ph\n",
    fb_info.bl_curve + i);
    mutex_unlock(&fb_info.bl_curve_mutex);
    return len;
    }

// When cmap is added back in it should be a binary attribute
// not a text one. Consideration should also be given to converting
// fbdev to use configfs instead of sysfs
    static DEVICE_ATTR(bits_per_pixel, 0644, show_bpp, store_bpp);
    static DEVICE_ATTR(blank, 0644, show_blank, store_blank);
    static DEVICE_ATTR(console, 0644, show_console, store_console);
    static DEVICE_ATTR(cursor, 0644, show_cursor, store_cursor);
    static DEVICE_ATTR(mode, 0644, show_mode, store_mode);
    static DEVICE_ATTR(modes, 0644, show_modes, store_modes);
    static DEVICE_ATTR(pan, 0644, show_pan, store_pan);
    static DEVICE_ATTR(virtual_size, 0644, show_virtual, store_virtual);
    static DEVICE_ATTR(name, 0444, show_name, core::ptr::null_mut());
    static DEVICE_ATTR(stride, 0444, show_stride, core::ptr::null_mut());
    static DEVICE_ATTR(rotate, 0644, show_rotate, store_rotate);
    static DEVICE_ATTR(state, 0644, show_fbstate, store_fbstate);

    static DEVICE_ATTR(bl_curve, 0644, show_bl_curve, store_bl_curve);

    static struct attribute *fb_device_attrs[] = {
    &dev_attr_bits_per_pixel.attr,
    &dev_attr_blank.attr,
    &dev_attr_console.attr,
    &dev_attr_cursor.attr,
    &dev_attr_mode.attr,
    &dev_attr_modes.attr,
    &dev_attr_pan.attr,
    &dev_attr_virtual_size.attr,
    &dev_attr_name.attr,
    &dev_attr_stride.attr,
    &dev_attr_rotate.attr,
    &dev_attr_state.attr,

    &dev_attr_bl_curve.attr,

    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(fb_device);
#[no_mangle]
pub unsafe extern "C" fn fb_device_create(fb_info: *mut fb_info) -> c_int {
    int fb_device_create(struct fb_info *fb_info)
    {
    let mut node: c_int = fb_info.node;
    let mut devt: dev_t = MKDEV(FB_MAJOR, node);
    int ret;
    fb_info.dev = device_create_with_groups(fb_class, fb_info.device, devt, fb_info,
    fb_device_groups, "fb%d", node);
    if (IS_ERR(fb_info.dev)) {
// Not fatal
    ret = PTR_ERR(fb_info.dev);
    pr_warn("Unable to create device for framebuffer %d; error %d\n", node, ret);
    fb_info.dev = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fb_device_destroy(fb_info: *mut fb_info) {
    void fb_device_destroy(struct fb_info *fb_info)
    {
    let mut devt: dev_t = MKDEV(FB_MAJOR, fb_info.node);
    if (!fb_info.dev)
    return;
    device_destroy(fb_class, devt);
    fb_info.dev = core::ptr::null_mut();
    }
