//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_drv.h
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


// i915_drv.h -- Private header for the I915 driver -*- linux-c -*-
//
// Copyright 2003 Tungsten Graphics, Inc., Cedar Park, Texas.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
// IN NO EVENT SHALL TUNGSTEN GRAPHICS AND/OR ITS SUPPLIERS BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Data Stolen Memory (DSM) aka "i915 stolen memory"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_dsm {
//
// The start and end of DSM which we can optionally use to create GEM
// objects backed by stolen memory.
//
// Note that usable_size tells us exactly how much of this we are
// actually allowed to use, given that some portion of it is in fact
// reserved for use by hardware functions.
//
    pub stolen: resource,
//
// Reserved portion of DSM.
//
    pub reserved: resource,
//
// Total size minus reserved ranges.
//
// DSM is segmented in hardware with different portions offlimits to
// certain functions.
//
// The drm_mm is initialised to the total accessible range, as found
// from the PCI config. On Broadwell+, this is further restricted to
// avoid the first page! The upper end of DSM is reserved for hardware
// functions and similarly removed from the accessible range.
//
    pub usable_size: resource_size_t,
}

pub const MAX_L3_SLICES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_l3_parity {
    pub remap_info: [*mut u32; MAX_L3_SLICES],
    pub error_work: work_struct,
    pub which_slice: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_mm {
//
// Shortcut for the stolen region. This points to either
// INTEL_REGION_STOLEN_SMEM for integrated platforms, or
// INTEL_REGION_STOLEN_LMEM for discrete, or NULL if the device doesn't
// support stolen.
//
    pub stolen_region: *mut intel_memory_region,
// Memory allocator for GTT stolen memory
    pub stolen: drm_mm,
// Protects the usage of the GTT stolen memory allocator
    pub stolen_lock: mutex,
// Protects bound_list/unbound_list and #drm_i915_gem_object.mm.link
    pub obj_lock: spinlock_t,
//
// List of objects which are purgeable.
//
    pub purge_list: list_head,
//
// List of objects which have allocated pages and are shrinkable.
//
    pub shrink_list: list_head,
//
// List of objects which are pending destruction.
//
    pub free_list: llist_head,
    pub free_work: work_struct,
//
// Count of objects pending destructions. Used to skip needlessly
// waiting on an RCU barrier if no objects are waiting to be freed.
//
    pub free_count: core::sync::atomic::AtomicI32,
    pub regions: [*mut intel_memory_region; INTEL_REGION_UNKNOWN],
    pub oom_notifier: notifier_block,
    pub vmap_notifier: notifier_block,
    pub shrinker: *mut shrinker,
// shrinker accounting, also useful for userland debugging
    pub shrink_memory: u64,
    pub shrink_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_virtual_gpu {
    pub /: *mut *mut mutex lock; / serialises sending of g2v_notify command pkts,
    pub active: bool,
    pub caps: u32,
    pub initial_mmio: *mut u32,
    pub initial_cfg_space: *mut u8,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_selftest_stash {
    pub counter: core::sync::atomic::AtomicI32,
    pub mock_region_instances: ida,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_private {
    pub drm: drm_device,
// display device data, must be placed after drm device member
    pub display: *mut intel_display,
// FIXME: Device release actions should all be moved to drmm_
    pub do_release: bool,
// i915 device parameters
    pub params: i915_params,
    pub /: *const *const *const intel_device_info __info; / Use INTEL_INFO() to access.,
    pub /: *mut *mut intel_runtime_info __runtime; / Use RUNTIME_INFO() to access.,
    pub caps: intel_driver_caps,
    pub dsm: i915_dsm,
    pub uncore: intel_uncore,
    pub mmio_debug: intel_uncore_mmio_debug,
    pub vgpu: i915_virtual_gpu,
    pub gvt: *mut intel_gvt,
    pub pdev: *mut pci_dev,
    pub mch_res: resource,
    pub mchbar_need_disable: bool,
    pub gmch: },
//
// Chaining user engines happens in multiple stages, starting with a
// simple lock-less linked list created by intel_engine_add_user(),
// which later gets sorted and converted to an intermediate regular
// list, just to be converted once again to its final rb tree structure
// in intel_engines_driver_register().
//
// Make sure to use the right iterator helper, depending on if the code
// in question runs before or after intel_engines_driver_register() --
// for_each_uabi_engine() can only be used afterwards!
//
    pub uabi_engines_llist: llist_head,
    pub uabi_engines_list: list_head,
    pub uabi_engines: rb_root,
}

// LPT/WPT IOSF sideband protection
// VLV/CHV IOSF sideband
// Sideband mailbox protection
// Cached value of gen 2-4 IMR to avoid reads in updating the bitfield
//
// wq - Driver workqueue for GEM.
//
// NOTE: Work items scheduled here are not allowed to grab any modeset
// locks, for otherwise the flushing done in the pageflip code will
// result in deadlocks.
//
// unordered_wq - internal workqueue for unordered work
//
// This workqueue should be used for all unordered work
// scheduling within i915, which used to be scheduled on the
// system_percpu_wq before moving to a driver instance due
// deprecation of flush_scheduled_work().
//
// pm private clock gating functions
//
// edram size in MB.
// Cannot be determined by PCIID. You must always read a register.
//
// Quick lookup of media GT (current platforms only have one)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_contexts {
    pub /: *mut *mut spinlock_t lock; / locks list,
    pub list: list_head,
    pub contexts: },
//
// We replace the local file with a global mappings as the
// backing storage for the mmap is on the device and not
// on the struct file, and we do not want to prolong the
// lifetime of the local fd. To minimise the number of
// anonymous inodes we create, we use a global singleton to
// share the global mapping.
//
    pub mmap_singleton: *mut file,
    pub gem: },
    pub /: *mut *mut spinlock_t frontbuffer_lock; / protects obj->frontbuffer (write-side),
    pub pxp: *mut intel_pxp,
    pub overlay: *mut i915_overlay,
    pub pmu: i915_pmu,
// The TTM device structure.
    pub bdev: ttm_device,
    pub pending_fb_pin: core::sync::atomic::AtomicI32,
    pub selftest;): I915_SELFTEST_DECLARE(struct i915_selftest_stash,
//
// NOTE: This is the dri1/ums dungeon, don't add stuff here. Your patch
// will be rejected. Instead look for a better place.
//
}

extern "C" {
    pub fn container_of(_arg: dev, drm_i915_private: struct, _arg: drm) -> return;
}

// Expand the platform_mask array if this fails.
extern "C" {
    pub fn ARRAY_SIZE(_arg: info->platform_mask)) -> *mut pbits;
}
// Shift and test on the MSB position so sign flag can be used.

// ULX machines are also considered ULT.

//
// The Gen7 cmdparser copies the scanned buffer to the ggtt for execution
// All later gens can run the final buffer from the ppgtt
//

// WaRsDisableCoarsePowerGating:skl,cnl

// With the 945 and later, Y tiling got adjusted so that it was 32 128-byte
// rows, which changed the alignment requirements and fence programming.
//

//
// Set this flag, when platform requires 64K GTT page sizes or larger for
// device local memory access.
//

//
// Platform has the dedicated compression control state for each lmem surfaces
// stored in lmem to support the 3D and media compression formats.
//

// DPF == dynamic parity feature

