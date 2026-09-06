//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/overlay-sysfs.c
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
//
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
// Some code and ideas taken from drivers/video/omap/ driver
// by Imre Deak.
//

#[no_mangle]
unsafe extern "C" fn overlay_name_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_name_show(struct omap_overlay *ovl, char *buf)
    {
    return sysfs_emit(buf, "%s\n", ovl.name);
    }
#[no_mangle]
unsafe extern "C" fn overlay_manager_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_manager_show(struct omap_overlay *ovl, char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    ovl.manager ? ovl.manager.name : "<none>");
    }
    static ssize_t overlay_manager_store(struct omap_overlay *ovl, const char *buf,
    size_t size)
    {
    int i, r;
    struct omap_overlay_manager *mgr = core::ptr::null_mut();
    struct omap_overlay_manager *old_mgr;
    let mut len: c_int = size;
    if (buf[size-1] == '\n')
    --len;
    if (len > 0) {
    for (i = 0; i < omap_dss_get_num_overlay_managers(); ++i) {
    mgr = omap_dss_get_overlay_manager(i);
    if (sysfs_streq(buf, mgr.name))
    break;
    mgr = core::ptr::null_mut();
    }
    }
    if (len > 0 && mgr == core::ptr::null_mut())
    return -EINVAL;
    if (mgr)
    DSSDBG("manager %s found\n", mgr.name);
    if (mgr == ovl.manager)
    return size;
    old_mgr = ovl.manager;
    r = dispc_runtime_get();
    if (r)
    return r;
// detach old manager
    if (old_mgr) {
    r = ovl.unset_manager(ovl);
    if (r) {
    DSSERR("detach failed\n");
    goto err;
    }
    r = old_mgr.apply(old_mgr);
    if (r)
    goto err;
    }
    if (mgr) {
    r = ovl.set_manager(ovl, mgr);
    if (r) {
    DSSERR("Failed to attach overlay\n");
    goto err;
    }
    r = mgr.apply(mgr);
    if (r)
    goto err;
    }
    dispc_runtime_put();
    return size;
    err:
    dispc_runtime_put();
    return r;
    }
#[no_mangle]
unsafe extern "C" fn overlay_input_size_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_input_size_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d,%d\n",
    info.width, info.height);
    }
#[no_mangle]
unsafe extern "C" fn overlay_screen_width_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_screen_width_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d\n", info.screen_width);
    }
#[no_mangle]
unsafe extern "C" fn overlay_position_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_position_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d,%d\n",
    info.pos_x, info.pos_y);
    }
    static ssize_t overlay_position_store(struct omap_overlay *ovl,
    const char *buf, size_t size)
    {
    int r;
    char *last;
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    info.pos_x = simple_strtoul(buf, &last, 10);
    ++last;
    if (last - buf >= size)
    return -EINVAL;
    info.pos_y = simple_strtoul(last, &last, 10);
    r = ovl.set_overlay_info(ovl, &info);
    if (r)
    return r;
    if (ovl.manager) {
    r = ovl.manager.apply(ovl.manager);
    if (r)
    return r;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn overlay_output_size_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_output_size_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d,%d\n",
    info.out_width, info.out_height);
    }
    static ssize_t overlay_output_size_store(struct omap_overlay *ovl,
    const char *buf, size_t size)
    {
    int r;
    char *last;
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    info.out_width = simple_strtoul(buf, &last, 10);
    ++last;
    if (last - buf >= size)
    return -EINVAL;
    info.out_height = simple_strtoul(last, &last, 10);
    r = ovl.set_overlay_info(ovl, &info);
    if (r)
    return r;
    if (ovl.manager) {
    r = ovl.manager.apply(ovl.manager);
    if (r)
    return r;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn overlay_enabled_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_enabled_show(struct omap_overlay *ovl, char *buf)
    {
    return sysfs_emit(buf, "%d\n", ovl.is_enabled(ovl));
    }
    static ssize_t overlay_enabled_store(struct omap_overlay *ovl, const char *buf,
    size_t size)
    {
    int r;
    bool enable;
    r = kstrtobool(buf, &enable);
    if (r)
    return r;
    if (enable)
    r = ovl.enable(ovl);
    else
    r = ovl.disable(ovl);
    if (r)
    return r;
    return size;
    }
#[no_mangle]
unsafe extern "C" fn overlay_global_alpha_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_global_alpha_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d\n",
    info.global_alpha);
    }
    static ssize_t overlay_global_alpha_store(struct omap_overlay *ovl,
    const char *buf, size_t size)
    {
    int r;
    u8 alpha;
    struct omap_overlay_info info;
    if ((ovl.caps & OMAP_DSS_OVL_CAP_GLOBAL_ALPHA) == 0)
    return -ENODEV;
    r = kstrtou8(buf, 0, &alpha);
    if (r)
    return r;
    ovl.get_overlay_info(ovl, &info);
    info.global_alpha = alpha;
    r = ovl.set_overlay_info(ovl, &info);
    if (r)
    return r;
    if (ovl.manager) {
    r = ovl.manager.apply(ovl.manager);
    if (r)
    return r;
    }
    return size;
    }
    static ssize_t overlay_pre_mult_alpha_show(struct omap_overlay *ovl,
    char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d\n",
    info.pre_mult_alpha);
    }
    static ssize_t overlay_pre_mult_alpha_store(struct omap_overlay *ovl,
    const char *buf, size_t size)
    {
    int r;
    u8 alpha;
    struct omap_overlay_info info;
    if ((ovl.caps & OMAP_DSS_OVL_CAP_PRE_MULT_ALPHA) == 0)
    return -ENODEV;
    r = kstrtou8(buf, 0, &alpha);
    if (r)
    return r;
    ovl.get_overlay_info(ovl, &info);
    info.pre_mult_alpha = alpha;
    r = ovl.set_overlay_info(ovl, &info);
    if (r)
    return r;
    if (ovl.manager) {
    r = ovl.manager.apply(ovl.manager);
    if (r)
    return r;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn overlay_zorder_show(ovl: *mut omap_overlay, buf: *mut c_char) -> isize {
    static ssize_t overlay_zorder_show(struct omap_overlay *ovl, char *buf)
    {
    struct omap_overlay_info info;
    ovl.get_overlay_info(ovl, &info);
    return sysfs_emit(buf, "%d\n", info.zorder);
    }
    static ssize_t overlay_zorder_store(struct omap_overlay *ovl,
    const char *buf, size_t size)
    {
    int r;
    u8 zorder;
    struct omap_overlay_info info;
    if ((ovl.caps & OMAP_DSS_OVL_CAP_ZORDER) == 0)
    return -ENODEV;
    r = kstrtou8(buf, 0, &zorder);
    if (r)
    return r;
    ovl.get_overlay_info(ovl, &info);
    info.zorder = zorder;
    r = ovl.set_overlay_info(ovl, &info);
    if (r)
    return r;
    if (ovl.manager) {
    r = ovl.manager.apply(ovl.manager);
    if (r)
    return r;
    }
    return size;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct overlay_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct omap_overlay , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct omap_overlay , char ,,
}

    struct overlay_attribute overlay_attr_##_name = \
    __ATTR(_name, _mode, _show, _store)
    static OVERLAY_ATTR(name, S_IRUGO, overlay_name_show, core::ptr::null_mut());
    static OVERLAY_ATTR(manager, S_IRUGO|S_IWUSR,
    overlay_manager_show, overlay_manager_store);
    static OVERLAY_ATTR(input_size, S_IRUGO, overlay_input_size_show, core::ptr::null_mut());
    static OVERLAY_ATTR(screen_width, S_IRUGO, overlay_screen_width_show, core::ptr::null_mut());
    static OVERLAY_ATTR(position, S_IRUGO|S_IWUSR,
    overlay_position_show, overlay_position_store);
    static OVERLAY_ATTR(output_size, S_IRUGO|S_IWUSR,
    overlay_output_size_show, overlay_output_size_store);
    static OVERLAY_ATTR(enabled, S_IRUGO|S_IWUSR,
    overlay_enabled_show, overlay_enabled_store);
    static OVERLAY_ATTR(global_alpha, S_IRUGO|S_IWUSR,
    overlay_global_alpha_show, overlay_global_alpha_store);
    static OVERLAY_ATTR(pre_mult_alpha, S_IRUGO|S_IWUSR,
    overlay_pre_mult_alpha_show,
    overlay_pre_mult_alpha_store);
    static OVERLAY_ATTR(zorder, S_IRUGO|S_IWUSR,
    overlay_zorder_show, overlay_zorder_store);
    static struct attribute *overlay_sysfs_attrs[] = {
    &overlay_attr_name.attr,
    &overlay_attr_manager.attr,
    &overlay_attr_input_size.attr,
    &overlay_attr_screen_width.attr,
    &overlay_attr_position.attr,
    &overlay_attr_output_size.attr,
    &overlay_attr_enabled.attr,
    &overlay_attr_global_alpha.attr,
    &overlay_attr_pre_mult_alpha.attr,
    &overlay_attr_zorder.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(overlay_sysfs);
    static ssize_t overlay_attr_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    struct omap_overlay *overlay;
    struct overlay_attribute *overlay_attr;
    overlay = container_of(kobj, struct omap_overlay, kobj);
    overlay_attr = container_of(attr, struct overlay_attribute, attr);
    if (!overlay_attr.show)
    return -ENOENT;
    return overlay_attr.show(overlay, buf);
    }
    static ssize_t overlay_attr_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t size)
    {
    struct omap_overlay *overlay;
    struct overlay_attribute *overlay_attr;
    overlay = container_of(kobj, struct omap_overlay, kobj);
    overlay_attr = container_of(attr, struct overlay_attribute, attr);
    if (!overlay_attr.store)
    return -ENOENT;
    return overlay_attr.store(overlay, buf, size);
    }
    static const struct sysfs_ops overlay_sysfs_ops = {
    .show = overlay_attr_show,
    .store = overlay_attr_store,
    };
    static struct kobj_type overlay_ktype = {
    .sysfs_ops = &overlay_sysfs_ops,
    .default_groups = overlay_sysfs_groups,
    };
    int dss_overlay_kobj_init(struct omap_overlay *ovl,
    struct platform_device *pdev)
    {
    return kobject_init_and_add(&ovl.kobj, &overlay_ktype,
    &pdev.dev.kobj, "overlay%d", ovl.id);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_overlay_kobj_uninit(ovl: *mut omap_overlay) {
    void dss_overlay_kobj_uninit(struct omap_overlay *ovl)
    {
    kobject_del(&ovl.kobj);
    kobject_put(&ovl.kobj);
    }
