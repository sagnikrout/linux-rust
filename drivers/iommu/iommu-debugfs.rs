//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/iommu-debugfs.c
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
// IOMMU debugfs core infrastructure
//
// Copyright (C) 2018 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <gary.hook@amd.com>
//

    struct dentry *iommu_debugfs_dir;
    EXPORT_SYMBOL_GPL(iommu_debugfs_dir);
//
// iommu_debugfs_setup - create the top-level iommu directory in debugfs
//
// Provide base enablement for using debugfs to expose internal data of an
// IOMMU driver. When called, this function creates the
// /sys/kernel/debug/iommu directory.
//
// Emit a strong warning at boot time to indicate that this feature is
// enabled.
//
// This function is called from iommu_init; drivers may then use
// iommu_debugfs_dir to instantiate a vendor-specific directory to be used
// to expose internal data.
//
#[no_mangle]
pub unsafe extern "C" fn iommu_debugfs_setup() {
    void iommu_debugfs_setup(void)
    {
    if (!iommu_debugfs_dir) {
    iommu_debugfs_dir = debugfs_create_dir("iommu", core::ptr::null_mut());
    pr_warn("\n");
    pr_warn("*************************************************************\n");
    pr_warn("**     NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE    **\n");
    pr_warn("**                                                         **\n");
    pr_warn("**  IOMMU DebugFS SUPPORT HAS BEEN ENABLED IN THIS KERNEL  **\n");
    pr_warn("**                                                         **\n");
    pr_warn("** This means that this kernel is built to expose internal **\n");
    pr_warn("** IOMMU data structures, which may compromise security on **\n");
    pr_warn("** your system.                                            **\n");
    pr_warn("**                                                         **\n");
    pr_warn("** If you see this message and you are not debugging the   **\n");
    pr_warn("** kernel, report this immediately to your vendor!         **\n");
    pr_warn("**                                                         **\n");
    pr_warn("**     NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE    **\n");
    pr_warn("*************************************************************\n");
    }
    }
