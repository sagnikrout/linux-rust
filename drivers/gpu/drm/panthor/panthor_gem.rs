//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_gem.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>
// Copyright 2023 Collabora ltd.
// Copyright 2025 ARM Limited. All rights reserved.

pub const PANTHOR_BO_LABEL_MAXLEN: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_debugfs_gem_state_flags {
    PANTHOR_DEBUGFS_GEM_STATE_IMPORTED_BIT = 0,
    PANTHOR_DEBUGFS_GEM_STATE_EXPORTED_BIT = 1,
    PANTHOR_DEBUGFS_GEM_STATE_EVICTED_BIT = 2,

// @PANTHOR_DEBUGFS_GEM_STATE_FLAG_IMPORTED: GEM BO is PRIME imported.
    PANTHOR_DEBUGFS_GEM_STATE_FLAG_IMPORTED = BIT(PANTHOR_DEBUGFS_GEM_STATE_IMPORTED_BIT),

// @PANTHOR_DEBUGFS_GEM_STATE_FLAG_EXPORTED: GEM BO is PRIME exported.
    PANTHOR_DEBUGFS_GEM_STATE_FLAG_EXPORTED = BIT(PANTHOR_DEBUGFS_GEM_STATE_EXPORTED_BIT),

// @PANTHOR_DEBUGFS_GEM_STATE_FLAG_EVICTED: GEM BO is evicted to swap.
    PANTHOR_DEBUGFS_GEM_STATE_FLAG_EVICTED = BIT(PANTHOR_DEBUGFS_GEM_STATE_EVICTED_BIT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_debugfs_gem_usage_flags {
    PANTHOR_DEBUGFS_GEM_USAGE_KERNEL_BIT = 0,
    PANTHOR_DEBUGFS_GEM_USAGE_FW_MAPPED_BIT = 1,

// @PANTHOR_DEBUGFS_GEM_USAGE_FLAG_KERNEL: BO is for kernel use only.
    PANTHOR_DEBUGFS_GEM_USAGE_FLAG_KERNEL = BIT(PANTHOR_DEBUGFS_GEM_USAGE_KERNEL_BIT),

// @PANTHOR_DEBUGFS_GEM_USAGE_FLAG_FW_MAPPED: BO is mapped on the FW VM.
    PANTHOR_DEBUGFS_GEM_USAGE_FLAG_FW_MAPPED = BIT(PANTHOR_DEBUGFS_GEM_USAGE_FW_MAPPED_BIT),
}

//
// struct panthor_gem_debugfs - GEM object's DebugFS list information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gem_debugfs {
//
// @node: Node used to insert the object in the device-wide list of
// GEM objects, to display information about it through a DebugFS file.
//
    pub node: list_head,
// @creator: Information about the UM process which created the GEM.
// @creator.process_name: Group leader name in owning thread's process
    pub process_name: [c_char; TASK_COMM_LEN],
// @creator.tgid: PID of the thread's group leader within its process
    pub tgid: pid_t,
    pub creator: },
// @flags: Combination of panthor_debugfs_gem_usage_flags flags
    pub flags: u32,
}

//
// struct panthor_gem_backing - GEM memory backing related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gem_backing {
// @pages: Pages requested with drm_gem_get_pages()
    pub pages: *mut page,
// @pin_count: Number of active pin requests on this GEM
    pub pin_count: refcount_t,
}

//
// struct panthor_gem_cpu_map - GEM CPU mapping related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gem_cpu_map {
// @vaddr: Address returned by vmap()
    pub vaddr: *mut c_void,
// @vaddr_use_count: Number of active vmap() requests on this GEM
    pub vaddr_use_count: refcount_t,
// @mmap_count: Number of active mmap() requests on this GEM
    pub mmap_count: refcount_t,
}

//
// struct panthor_gem_dev_map - GEM device mapping related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gem_dev_map {
// @sgt: Device mapped sg_table for this GEM
    pub sgt: *mut sg_table,
}

//
// enum panthor_gem_reclaim_state - Reclaim state of a GEM object
//
// This is defined in descending reclaimability order and some part
// of the code depends on that.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_gem_reclaim_state {
//
// @PANTHOR_GEM_UNUSED: GEM is currently unused
//
// This can happen when the GEM was previously vmap-ed, mmap-ed,
// and/or GPU mapped and got unmapped. Because pages are lazily
// returned to the shmem layer, we want to keep a list of such
// BOs, because they should be fairly easy to reclaim (no need
// to wait for GPU to be done, and no need to tear down user
// mappings either).
//
    PANTHOR_GEM_UNUSED,

//
// @PANTHOR_GEM_MMAPPED: GEM is currently mmap-ed
//
// When a GEM has pages allocated and the mmap_count is > 0, the
// GEM is placed in the mmapped list. This comes right after
// unused because we can relatively easily tear down user mappings.
//
    PANTHOR_GEM_MMAPPED,

//
// @PANTHOR_GEM_GPU_MAPPED_SINGLE_VM: GEM is GPU mapped to only one VM
//
// When a GEM is mapped to a single VM, reclaim requests have more
// chances to succeed, because we only need to synchronize against
// a single GPU context. This is more annoying than reclaiming
// mmap-ed pages still, because we have to wait for in-flight jobs
// to land, and we might not be able to acquire all necessary locks
// at reclaim time either.
//
    PANTHOR_GEM_GPU_MAPPED_SINGLE_VM,

//
// @PANTHOR_GEM_GPU_MAPPED_MULTI_VM: GEM is GPU mapped to multiple VMs
//
// Like PANTHOR_GEM_GPU_MAPPED_SINGLE_VM, but the synchronization across
// VMs makes such BOs harder to reclaim.
//
    PANTHOR_GEM_GPU_MAPPED_MULTI_VM,

//
// @PANTHOR_GEM_UNRECLAIMABLE: GEM can't be reclaimed
//
// Happens when the GEM memory is pinned. It's also the state all GEM
// objects start in, because no memory is allocated until explicitly
// requested by a CPU or GPU map, meaning there's nothing to reclaim
// until such an allocation happens.
//
    PANTHOR_GEM_UNRECLAIMABLE,
}

//
// struct panthor_gem_object - Driver specific GEM object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gem_object {
// @base: Inherit from drm_gem_object.
    pub base: drm_gem_object,
// @backing: Memory backing state
    pub backing: panthor_gem_backing,
// @cmap: CPU mapping state
    pub cmap: panthor_gem_cpu_map,
// @dmap: Device mapping state
    pub dmap: panthor_gem_dev_map,
// @reclaim_state: Cached reclaim state
    pub reclaim_state: panthor_gem_reclaim_state,
//
// @reclaimed_count: How many times object has been evicted to swap.
// The count saturates at %INT_MAX and will never wrap around to 0.
//
    pub reclaimed_count: core::sync::atomic::AtomicI32,
//
// @exclusive_vm_root_gem: Root GEM of the exclusive VM this GEM object
// is attached to.
//
// If @exclusive_vm_root_gem != NULL, any attempt to bind the GEM to a
// different VM will fail.
//
// All FW memory objects have this field set to the root GEM of the MCU
// VM.
//
    pub exclusive_vm_root_gem: *mut drm_gem_object,
// @flags: Combination of drm_panthor_bo_flags flags.
    pub flags: u32,
//
// @label: BO tagging fields. The label can be assigned within the
// driver itself or through a specific IOCTL.
//
// @label.str: Pointer to NULL-terminated string,
//
    pub str: *const c_char,
// @lock.str: Protects access to the @label.str field.
    pub lock: mutex,
    pub label: },

    pub debugfs: panthor_gem_debugfs,

}

//
// struct panthor_kernel_bo - Kernel buffer object.
//
// These objects are only manipulated by the kernel driver and not
// directly exposed to the userspace. The GPU address of a kernel
// BO might be passed to userspace though.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_kernel_bo {
//
// @obj: The GEM object backing this kernel buffer object.
//
    pub obj: *mut drm_gem_object,
//
// @vm: VM this private buffer is attached to.
//
    pub vm: *mut panthor_vm,
//
// @va_node: VA space allocated to this GEM.
//
    pub va_node: drm_mm_node,
//
// @kmap: Kernel CPU mapping of @gem.
//
    pub kmap: *mut c_void,
}

extern "C" {
    pub fn panthor_gem_init(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gem_pin(bo: *mut panthor_gem_object) -> c_int;
}
extern "C" {
    pub fn panthor_gem_unpin(bo: *mut panthor_gem_object);
}
extern "C" {
    pub fn panthor_gem_swapin_locked(bo: *mut panthor_gem_object) -> c_int;
}
extern "C" {
    pub fn panthor_gem_shrinker_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_gem_shrinker_unplug(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gem_bo_set_label(obj: *mut drm_gem_object, label: *const c_char);
}
extern "C" {
    pub fn panthor_gem_kernel_bo_set_label(bo: *mut panthor_kernel_bo, label: *const c_char);
}
extern "C" {
    pub fn panthor_kernel_bo_destroy(bo: *mut panthor_kernel_bo);
}

extern "C" {
    pub fn panthor_gem_debugfs_init(minor: *mut drm_minor);
}

