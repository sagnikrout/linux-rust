//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vboxvideo/vbox_ttm.c
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
// Copyright (C) 2013-2017 Oracle Corporation
// This file is based on ast_ttm.c
// Copyright 2012 Red Hat Inc.
// Authors: Dave Airlie <airlied@redhat.com>
// Michael Thayer <michael.thayer@oracle.com>
//

#[no_mangle]
pub unsafe extern "C" fn vbox_mm_init(vbox: *mut vbox_private) -> c_int {
    int vbox_mm_init(struct vbox_private *vbox)
    {
    int ret;
    resource_size_t base, size;
    struct drm_device *dev = &vbox.ddev;
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    base = pci_resource_start(pdev, 0);
    size = pci_resource_len(pdev, 0);
// Don't fail on errors, but performance might be reduced.
    devm_arch_phys_wc_add(&pdev.dev, base, size);
    ret = drmm_vram_helper_init(dev, base, vbox.available_vram_size);
    if (ret) {
    DRM_ERROR("Error initializing VRAM MM; %d\n", ret);
    return ret;
    }
    return 0;
    }
