//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/xe_sriov_vfio.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

    struct xe_device *xe_sriov_vfio_get_pf(struct pci_dev *pdev)
    {
    return xe_pci_to_pf_device(pdev);
    }
    EXPORT_SYMBOL_FOR_MODULES(xe_sriov_vfio_get_pf, "xe-vfio-pci");
#[no_mangle]
pub unsafe extern "C" fn xe_sriov_vfio_migration_supported(xe: *mut xe_device) -> bool {
    bool xe_sriov_vfio_migration_supported(struct xe_device *xe)
    {
    if (!IS_SRIOV_PF(xe))
    return false;
    return xe_sriov_pf_migration_supported(xe);
    }
    EXPORT_SYMBOL_FOR_MODULES(xe_sriov_vfio_migration_supported, "xe-vfio-pci");

    _type xe_sriov_vfio_##_func(struct xe_device *xe, unsigned int vfid)		\
    {										\
    if (!IS_SRIOV_PF(xe))							\
    return -EPERM;							\
    if (vfid == PFID || vfid > xe_sriov_pf_num_vfs(xe))			\
    return -EINVAL;							\
    \
    guard(xe_pm_runtime_noresume)(xe);					\
    \
    return xe_sriov_pf_##_impl(xe, vfid);					\
    }										\
    EXPORT_SYMBOL_FOR_MODULES(xe_sriov_vfio_##_func, "xe-vfio-pci")
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, wait_flr_done, control_wait_flr);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, flr_prepare, control_prepare_flr);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, suspend_device, control_pause_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, resume_device, control_resume_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, stop_copy_enter, control_trigger_save_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, stop_copy_exit, control_finish_save_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, resume_data_enter, control_trigger_restore_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, resume_data_exit, control_finish_restore_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(int, error, control_stop_vf);
    DEFINE_XE_SRIOV_VFIO_FUNCTION(ssize_t, stop_copy_size, migration_size);
    ssize_t xe_sriov_vfio_data_read(struct xe_device *xe, unsigned int vfid,
    char __user *buf, size_t len)
    {
    if (!IS_SRIOV_PF(xe))
    return -EPERM;
    if (vfid == PFID || vfid > xe_sriov_pf_num_vfs(xe))
    return -EINVAL;
    guard(xe_pm_runtime_noresume)(xe);
    return xe_sriov_pf_migration_read(xe, vfid, buf, len);
    }
    EXPORT_SYMBOL_FOR_MODULES(xe_sriov_vfio_data_read, "xe-vfio-pci");
    ssize_t xe_sriov_vfio_data_write(struct xe_device *xe, unsigned int vfid,
    const char __user *buf, size_t len)
    {
    if (!IS_SRIOV_PF(xe))
    return -EPERM;
    if (vfid == PFID || vfid > xe_sriov_pf_num_vfs(xe))
    return -EINVAL;
    guard(xe_pm_runtime_noresume)(xe);
    return xe_sriov_pf_migration_write(xe, vfid, buf, len);
    }
    EXPORT_SYMBOL_FOR_MODULES(xe_sriov_vfio_data_write, "xe-vfio-pci");
