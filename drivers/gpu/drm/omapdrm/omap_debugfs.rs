//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/omapdrm/omap_debugfs.c
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
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
// Author: Rob Clark <rob.clark@linaro.org>
//

#[no_mangle]
unsafe extern "C" fn gem_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int gem_show(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct drm_device *dev = node.minor.dev;
    struct omap_drm_private *priv = dev.dev_private;
    seq_printf(m, "All Objects:\n");
    mutex_lock(&priv.list_lock);
    omap_gem_describe_objects(&priv.obj_list, m);
    mutex_unlock(&priv.list_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mm_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int mm_show(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct drm_device *dev = node.minor.dev;
    let mut p: drm_printer = drm_seq_file_printer(m);
    drm_mm_print(&dev.vma_offset_manager.vm_addr_space_mm, &p);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn fb_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int fb_show(struct seq_file *m, void *arg)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct drm_device *dev = node.minor.dev;
    struct drm_fb_helper *helper = dev.fb_helper;
    struct drm_framebuffer *fb;
    seq_printf(m, "fbcon ");
    omap_framebuffer_describe(helper.fb, m);
    mutex_lock(&dev.mode_config.fb_lock);
    list_for_each_entry(fb, &dev.mode_config.fb_list, head) {
    if (fb == helper.fb)
    continue;
    seq_printf(m, "user ");
    omap_framebuffer_describe(fb, m);
    }
    mutex_unlock(&dev.mode_config.fb_lock);
    return 0;
    }

// list of debufs files that are applicable to all devices
    static struct drm_info_list omap_debugfs_list[] = {
    {"gem", gem_show, 0},
    {"mm", mm_show, 0},

    {"fb", fb_show, 0},

    };
// list of debugfs files that are specific to devices with dmm/tiler
    static struct drm_info_list omap_dmm_debugfs_list[] = {
    {"tiler_map", tiler_map_show, 0},
    };
#[no_mangle]
pub unsafe extern "C" fn omap_debugfs_init(minor: *mut drm_minor) {
    void omap_debugfs_init(struct drm_minor *minor)
    {
    drm_debugfs_create_files(omap_debugfs_list,
    ARRAY_SIZE(omap_debugfs_list),
    minor.debugfs_root, minor);
    if (dmm_is_available())
    drm_debugfs_create_files(omap_dmm_debugfs_list,
    ARRAY_SIZE(omap_dmm_debugfs_list),
    minor.debugfs_root, minor);
    }
