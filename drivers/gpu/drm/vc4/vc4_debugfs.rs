//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/vc4_debugfs.c
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
// Copyright © 2014 Broadcom
//

//
// Called at drm_dev_register() time on each of the minors registered
// by the DRM device, to attach the debugfs files.
//
    void
    vc4_debugfs_init(struct drm_minor *minor)
    {
    struct vc4_dev *vc4 = to_vc4_dev(minor.dev);
    struct drm_device *drm = &vc4.base;
    drm_WARN_ON(drm, vc4_hvs_debugfs_init(minor));
    if (vc4.v3d) {
    drm_WARN_ON(drm, vc4_bo_debugfs_init(minor));
    drm_WARN_ON(drm, vc4_v3d_debugfs_init(minor));
    }
    }
#[no_mangle]
unsafe extern "C" fn vc4_debugfs_regset32(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int vc4_debugfs_regset32(struct seq_file *m, void *unused)
    {
    struct drm_debugfs_entry *entry = m.private;
    struct drm_device *drm = entry.dev;
    struct debugfs_regset32 *regset = entry.file.data;
    let mut p: drm_printer = drm_seq_file_printer(m);
    int idx;
    if (!drm_dev_enter(drm, &idx))
    return -ENODEV;
    drm_print_regset32(&p, regset);
    drm_dev_exit(idx);
    return 0;
    }
    void vc4_debugfs_add_regset32(struct drm_device *drm,
    const char *name,
    struct debugfs_regset32 *regset)
    {
    drm_debugfs_add_file(drm, name, vc4_debugfs_regset32, regset);
    }
