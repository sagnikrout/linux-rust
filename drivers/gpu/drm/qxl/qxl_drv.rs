//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/qxl/qxl_drv.h
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


//
// Copyright 2013 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Dave Airlie
// Alon Levy
//
// Definitions taken from spice-protocol, plus kernel driver specific bits.
//

pub const DRIVER_MAJOR: c_int = 0;
pub const DRIVER_MINOR: c_int = 1;
pub const DRIVER_PATCHLEVEL: c_int = 0;
pub const QXL_DEBUGFS_MAX_COMPONENTS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_bo {
    pub tbo: ttm_buffer_object,
// Protected by gem.mutex
    pub list: list_head,
// Protected by tbo.reserved
    pub placements: [ttm_place; 3],
    pub placement: ttm_placement,
    pub map: iosys_map,
    pub kptr: *mut c_void,
    pub map_count: c_uint,
    pub type: c_int,
// Constant after initialization
    pub /: *mut *mut unsigned int is_primary:1; / is this now a primary surface,
    pub is_dumb:1: c_uint,
    pub shadow: *mut qxl_bo,
    pub hw_surf_alloc:1: c_uint,
    pub surf: qxl_surface,
    pub surface_id: u32,
    pub surf_create: *mut qxl_release,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_gem {
    pub mutex: mutex,
    pub objects: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_bo_list {
    pub bo: *mut qxl_bo,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_crtc {
    pub base: drm_crtc,
    pub index: c_int,
    pub cursor_bo: *mut qxl_bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_output {
    pub index: c_int,
    pub base: drm_connector,
    pub enc: drm_encoder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_mman {
    pub bdev: ttm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_memslot {
    pub index: c_int,
    pub name: *const c_char,
    pub generation: u8,
    pub start_phys_addr: u64,
    pub size: u64,
    pub high_bits: u64,
}

// drm_ prefix to differentiate from qxl_release_info in
// spice-protocol/qxl_dev.h
pub const QXL_MAX_RES: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_release {
    pub base: dma_fence,
    pub id: c_int,
    pub type: c_int,
    pub release_bo: *mut qxl_bo,
    pub release_offset: u32,
    pub surface_release_id: u32,
    pub exec: drm_exec,
    pub bos: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_drm_chunk {
    pub head: list_head,
    pub bo: *mut qxl_bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_drm_image {
    pub bo: *mut qxl_bo,
    pub chunk_list: list_head,
}

//
// Debugfs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_debugfs {
    pub files: *mut drm_info_list,
    pub num_files: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_device {
    pub ddev: drm_device,
    pub vram_size: resource_size_t vram_base,,
    pub surfaceram_size: resource_size_t surfaceram_base,,
    pub rom_size: resource_size_t rom_base,,
    pub rom: *mut qxl_rom,
    pub modes: *mut qxl_mode,
    pub monitors_config_bo: *mut qxl_bo,
    pub monitors_config: *mut qxl_monitors_config,
// last received client_monitors_config
    pub client_monitors_config: *mut qxl_monitors_config,
    pub io_base: c_int,
    pub ram: *mut c_void,
    pub mman: qxl_mman,
    pub gem: qxl_gem,
    pub ram_physical: *mut c_void,
    pub release_ring: *mut qxl_ring,
    pub command_ring: *mut qxl_ring,
    pub cursor_ring: *mut qxl_ring,
    pub ram_header: *mut qxl_ram_header,
    pub primary_bo: *mut qxl_bo,
    pub dumb_shadow_bo: *mut qxl_bo,
    pub dumb_heads: *mut qxl_head,
    pub main_slot: qxl_memslot,
    pub surfaces_slot: qxl_memslot,
    pub release_lock: spinlock_t,
    pub release_idr: idr,
    pub release_seqno: u32,
    pub release_count: core::sync::atomic::AtomicI32,
    pub release_event: wait_queue_head_t,
    pub release_idr_lock: spinlock_t,
    pub async_io_mutex: mutex,
    pub last_sent_io_cmd: c_uint,
// interrupt handling
    pub irq_received: core::sync::atomic::AtomicI32,
    pub irq_received_display: core::sync::atomic::AtomicI32,
    pub irq_received_cursor: core::sync::atomic::AtomicI32,
    pub irq_received_io_cmd: core::sync::atomic::AtomicI32,
    pub irq_received_error: c_uint,
    pub display_event: wait_queue_head_t,
    pub cursor_event: wait_queue_head_t,
    pub io_cmd_event: wait_queue_head_t,
    pub client_monitors_config_work: work_struct,
// debugfs
    pub debugfs: [qxl_debugfs; QXL_DEBUGFS_MAX_COMPONENTS],
    pub debugfs_count: c_uint,
    pub update_area_mutex: mutex,
    pub surf_id_idr: idr,
    pub surf_id_idr_lock: spinlock_t,
    pub last_alloced_surf_id: c_int,
    pub surf_evict_mutex: mutex,
    pub vram_mapping: *mut io_mapping,
    pub surface_mapping: *mut io_mapping,
//
    pub release_mutex: mutex,
    pub current_release_bo: [*mut qxl_bo; 3],
    pub current_release_bo_offset: [c_int; 3],
    pub gc_work: work_struct,
    pub hotplug_mode_update_property: *mut drm_property,
    pub monitors_config_width: c_int,
    pub monitors_config_height: c_int,
}

extern "C" {
    pub fn qxl_device_init(qdev: *mut qxl_device, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn qxl_device_fini(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_modeset_init(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_modeset_fini(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_bo_init(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_bo_fini(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_reinit_memslots(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_surf_evict(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_vram_evict(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_ring_free(ring: *mut qxl_ring);
}
extern "C" {
    pub fn qxl_check_idle(ring: *mut qxl_ring) -> c_int;
}
// TODO - need to hold one of the locks to read bo->tbo.resource->start
// qxl_display.c
extern "C" {
    pub fn qxl_display_read_client_monitors_config(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_create_monitors_object(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_destroy_monitors_object(qdev: *mut qxl_device) -> c_int;
}
// qxl_gem.c
extern "C" {
    pub fn qxl_gem_init(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_gem_fini(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_gem_object_free(gobj: *mut drm_gem_object);
}
extern "C" {
    pub fn qxl_gem_object_open(obj: *mut drm_gem_object, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_bo_force_delete(qdev: *mut qxl_device);
}
// qxl_dumb.c
// qxl ttm
extern "C" {
    pub fn qxl_ttm_init(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_ttm_fini(qdev: *mut qxl_device);
}
// qxl image
extern "C" {
    pub fn qxl_image_free_objects(qdev: *mut qxl_device, dimage: *mut qxl_drm_image);
}
// qxl io operations (qxl_cmd.c)
extern "C" {
    pub fn qxl_io_destroy_primary(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_io_memslot_add(qdev: *mut qxl_device, id: u8);
}
extern "C" {
    pub fn qxl_io_notify_oom(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_io_reset(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_io_monitors_config(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_ring_push(ring: *mut qxl_ring, new_elt: *const c_void, interruptible: bool) -> c_int;
}
extern "C" {
    pub fn qxl_io_flush_release(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_io_flush_surfaces(qdev: *mut qxl_device);
}
extern "C" {
    pub fn qxl_release_list_add(release: *mut qxl_release, bo: *mut qxl_bo) -> c_int;
}
extern "C" {
    pub fn qxl_release_reserve_list(release: *mut qxl_release, no_intr: bool) -> c_int;
}
extern "C" {
    pub fn qxl_release_backoff_reserve_list(release: *mut qxl_release);
}
extern "C" {
    pub fn qxl_release_fence_buffer_objects(release: *mut qxl_release);
}
// qxl drawing commands
// used by qxl_debugfs_release
extern "C" {
    pub fn qxl_queue_garbage_collect(qdev: *mut qxl_device, flush: bool) -> bool;
}
extern "C" {
    pub fn qxl_garbage_collect(qdev: *mut qxl_device) -> c_int;
}
// debugfs
extern "C" {
    pub fn qxl_debugfs_init(minor: *mut drm_minor);
}
extern "C" {
    pub fn qxl_ttm_debugfs_init(qdev: *mut qxl_device);
}
// qxl_prime.c
extern "C" {
    pub fn qxl_gem_prime_pin(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn qxl_gem_prime_unpin(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn qxl_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
// qxl_irq.c
extern "C" {
    pub fn qxl_irq_init(qdev: *mut qxl_device) -> c_int;
}
extern "C" {
    pub fn qxl_bo_check_id(qdev: *mut qxl_device, bo: *mut qxl_bo) -> c_int;
}
extern "C" {
    pub fn qxl_surface_evict(qdev: *mut qxl_device, surf: *mut qxl_bo, freeing: bool);
}
// qxl_ioctl.c
extern "C" {
    pub fn qxl_alloc_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_map_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_execbuffer_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_update_area_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_getparam_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_clientcap_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qxl_alloc_surf_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
