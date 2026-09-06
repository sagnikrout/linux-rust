//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_gem.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from "pvr_device.h".
//
// DOC: Flags for DRM_IOCTL_PVR_CREATE_BO (kernel-only)
//
// Kernel-only values allowed in &pvr_gem_object->flags. The majority of options
// for this field are specified in the UAPI header "pvr_drm.h" with a
// DRM_PVR_BO_ prefix. To distinguish these internal options (which must exist
// in ranges marked as "reserved" in the UAPI header), we drop the DRM prefix.
// The public options should be used directly, DRM prefix and all.
//
// To avoid potentially confusing gaps in the UAPI options, these kernel-only
// options are specified "in reverse", starting at bit 63.
//
// We use "reserved" to refer to bits defined here and not exposed in the UAPI.
// Bits not defined anywhere are "undefined".
//
// CPU mapping options
// :PVR_BO_CPU_CACHED: By default, all GEM objects are mapped write-combined on the CPU. Set
// this flag to override this behaviour and map the object cached. If the dma_coherent
// property is present in devicetree, all allocations will be mapped as if this flag was set.
// This does not require any additional consideration at allocation time.
//
// Firmware options
// :PVR_BO_FW_NO_CLEAR_ON_RESET: By default, all FW objects are cleared and reinitialised on hard
// reset. Set this flag to override this behaviour and preserve buffer contents on reset.
//

// Bits 61..3 are undefined.
// Bits 2..0 are defined in the UAPI.
// Other utilities.

//
// All firmware-mapped memory uses (mostly) the same flags. Specifically,
// firmware-mapped memory should be:
// * Read/write on the device,
// * Read/write on the CPU, and
// * Write-combined on the CPU.
//
// The only variation is in caching on the device.
//

//
// struct pvr_gem_object - powervr-specific wrapper for &struct drm_gem_object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_gem_object {
//
// @base: The underlying &struct drm_gem_shmem_object.
//
// .. note::
//
// This member should not be accessed directly, but instead by
// calling shmem_gem_from_pvr_gem().
//
    pub base: drm_gem_shmem_object,
//
// @flags: Options set at creation-time. Some of these options apply to
// the creation operation itself (which are stored here for reference)
// with the remainder used for mapping options to both the device and
// CPU. These are used every time this object is mapped, but may be
// changed after creation.
//
// Must be a combination of DRM_PVR_BO_* and/or PVR_BO_* flags.
//
    pub flags: u64,
}

// Functions defined in pvr_gem.c
extern "C" {
    pub fn drm_gem_shmem_get_pages_sgt(_arg: shmem_gem_from_pvr_gem(pvr_obj)) -> return;
}
extern "C" {
    pub fn pvr_gem_object_vunmap(pvr_obj: *mut pvr_gem_object);
}
//
// pvr_gem_object_get() - Acquire reference on pvr_gem_object
// @pvr_obj: Pointer to object to acquire reference on.
//
// pvr_gem_object_put() - Release reference on pvr_gem_object
// @pvr_obj: Pointer to object to release reference on.
//
