//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_gem.h
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
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>

pub const PANFROST_BO_LABEL_MAXLEN: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_debugfs_gem_state_flags {
// @PANFROST_DEBUGFS_GEM_STATE_FLAG_IMPORTED: GEM BO is PRIME imported.
    PANFROST_DEBUGFS_GEM_STATE_FLAG_IMPORTED = BIT(0),

// @PANFROST_DEBUGFS_GEM_STATE_FLAG_EXPORTED: GEM BO is PRIME exported.
    PANFROST_DEBUGFS_GEM_STATE_FLAG_EXPORTED = BIT(1),

// @PANFROST_DEBUGFS_GEM_STATE_FLAG_PURGED: GEM BO was reclaimed by the shrinker.
    PANFROST_DEBUGFS_GEM_STATE_FLAG_PURGED = BIT(2),

//
// @PANFROST_DEBUGFS_GEM_STATE_FLAG_PURGEABLE: GEM BO pages were marked as no longer
// needed by UM and can be reclaimed by the shrinker.
//
    PANFROST_DEBUGFS_GEM_STATE_FLAG_PURGEABLE = BIT(3),
}

//
// struct panfrost_gem_debugfs - GEM object's DebugFS list information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_gem_debugfs {
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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_gem_object {
    pub base: drm_gem_shmem_object,
    pub sgts: *mut sg_table,
//
// Use a list for now. If searching a mapping ever becomes the
// bottleneck, we should consider using an RB-tree, or even better,
// let the core store drm_gem_object_mapping entries (where we
// could place driver specific data) instead of drm_gem_object ones
// in its drm_file->object_idr table.
//
// struct drm_gem_object_mapping {
// struct drm_gem_object *obj;
// void *driver_priv;
// };
//
    pub list: list_head,
    pub lock: mutex,
    pub mappings: },
//
// Count the number of jobs referencing this BO so we don't let the
// shrinker reclaim this object prematurely.
//
    pub gpu_usecount: core::sync::atomic::AtomicI32,
//
// Object chunk size currently mapped onto physical memory
//
    pub heap_rss_size: usize,
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
    pub :1: bool noexec,
    pub :1: bool is_heap,
// On coherent devices, this reflects the creation flags, not the true
// cacheability attribute of the mapping.
//
    pub :1: bool wb_mmap,

    pub debugfs: panfrost_gem_debugfs,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_gem_mapping {
    pub node: list_head,
    pub refcount: kref,
    pub obj: *mut panfrost_gem_object,
    pub mmnode: drm_mm_node,
    pub mmu: *mut panfrost_mmu,
    pub :1: bool active,
}

extern "C" {
    pub fn container_of(_arg: to_drm_gem_shmem_obj(obj), panfrost_gem_object: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: node, panfrost_gem_mapping: struct, _arg: mmnode) -> return;
}
extern "C" {
    pub fn panfrost_gem_init(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_gem_open(obj: *mut drm_gem_object, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn panfrost_gem_mapping_put(mapping: *mut panfrost_gem_mapping);
}
extern "C" {
    pub fn panfrost_gem_teardown_mappings_locked(bo: *mut panfrost_gem_object);
}
extern "C" {
    pub fn panfrost_gem_shrinker_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn panfrost_gem_shrinker_cleanup(dev: *mut drm_device);
}
extern "C" {
    pub fn panfrost_gem_set_label(obj: *mut drm_gem_object, label: *const c_char);
}
extern "C" {
    pub fn panfrost_gem_internal_set_label(obj: *mut drm_gem_object, label: *const c_char);
}

