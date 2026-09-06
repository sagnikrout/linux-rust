//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/virtio/virtgpu_drv.h
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
// Copyright (C) 2015 Red Hat, Inc.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const DRIVER_MAJOR: c_int = 0;
pub const DRIVER_MINOR: c_int = 1;
pub const DRIVER_PATCHLEVEL: c_int = 0;
pub const STATE_INITIALIZING: c_int = 0;
pub const STATE_OK: c_int = 1;
pub const STATE_ERR: c_int = 2;
pub const MAX_CAPSET_ID: c_int = 63;
pub const MAX_RINGS: c_int = 64;
// See virtio_gpu_ctx_create. One additional character for NULL terminator.
pub const DEBUG_NAME_MAX_LEN: c_int = 65;
//
// Whether the host must be told about resource backing pages by DMA address
// rather than guest-physical address.
//
// This mirrors vring_use_map_api() in drivers/virtio/virtio_ring.c, including
// its xen_domain() case.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_object_params {
    pub size: c_ulong,
    pub dumb: bool,
// 3d
    pub virgl: bool,
    pub blob: bool,
// classic resources only
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub target: u32,
    pub bind: u32,
    pub depth: u32,
    pub array_size: u32,
    pub last_level: u32,
    pub nr_samples: u32,
    pub flags: u32,
// blob resources only
    pub ctx_id: u32,
    pub blob_mem: u32,
    pub blob_flags: u32,
    pub blob_id: u64,
    pub blob_hints: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_object {
    pub base: drm_gem_shmem_object,
    pub sgt: *mut sg_table,
    pub hw_res_handle: u32,
    pub dumb: bool,
    pub created: bool,
    pub attached: bool,
    pub guest_blob: bool host3d_blob,,
    pub blob_flags: uint32_t blob_mem,,
    pub uuid_state: c_int,
    pub uuid: uuid_t,
// for restoration of objects after hibernation
    pub params: virtio_gpu_object_params,
    pub restore_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_object_shmem {
    pub base: virtio_gpu_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_object_vram {
    pub base: virtio_gpu_object,
    pub map_state: u32,
    pub map_info: u32,
    pub vram_node: drm_mm_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_object_array {
    pub ticket: ww_acquire_ctx,
    pub next: list_head,
    pub total: u32 nents,,
    pub __counted_by(total): *mut *mut drm_gem_object objs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_fence_driver {
    pub last_fence_id: core::sync::atomic::AtomicI64,
    pub current_fence_id: u64,
    pub context: u64,
    pub fences: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_fence_event {
    pub base: drm_pending_event,
    pub event: drm_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_fence {
    pub f: dma_fence,
    pub ring_idx: u32,
    pub fence_id: u64,
    pub emit_fence_info: bool,
    pub e: *mut virtio_gpu_fence_event,
    pub drv: *mut virtio_gpu_fence_driver,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_vbuffer {
    pub buf: *mut c_char,
    pub size: c_int,
    pub data_buf: *mut c_void,
    pub data_size: u32,
    pub resp_buf: *mut c_char,
    pub resp_size: c_int,
    pub resp_cb: virtio_gpu_resp_cb,
    pub resp_cb_data: *mut c_void,
    pub objs: *mut virtio_gpu_object_array,
    pub list: list_head,
    pub seqno: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_output {
    pub index: c_int,
    pub crtc: drm_crtc,
    pub conn: drm_connector,
    pub enc: drm_encoder,
    pub info: virtio_gpu_display_one,
    pub cursor: virtio_gpu_update_cursor,
    pub drm_edid: *const drm_edid,
    pub cur_x: c_int,
    pub cur_y: c_int,
    pub needs_modeset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_framebuffer {
    pub base: drm_framebuffer,
    pub fence: *mut virtio_gpu_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_plane_state {
    pub base: drm_plane_state,
    pub fence: *mut virtio_gpu_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_queue {
    pub vq: *mut virtqueue,
    pub qlock: spinlock_t,
    pub ack_queue: wait_queue_head_t,
    pub dequeue_work: work_struct,
    pub seqno: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_drv_capset {
    pub id: u32,
    pub max_version: u32,
    pub max_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_drv_cap_cache {
    pub head: list_head,
    pub caps_cache: *mut c_void,
    pub id: u32,
    pub version: u32,
    pub size: u32,
    pub is_valid: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_device {
    pub ddev: *mut drm_device,
    pub vdev: *mut virtio_device,
    pub outputs: [virtio_gpu_output; VIRTIO_GPU_MAX_SCANOUTS],
    pub num_scanouts: u32,
    pub ctrlq: virtio_gpu_queue,
    pub cursorq: virtio_gpu_queue,
    pub vqs_released: bool,
    pub vbufs: *mut kmem_cache,
    pub pending_commands: core::sync::atomic::AtomicI32,
    pub resource_ida: ida,
    pub resp_wq: wait_queue_head_t,
// current display info
    pub display_info_lock: spinlock_t,
    pub display_info_pending: bool,
    pub fence_drv: virtio_gpu_fence_driver,
    pub ctx_id_ida: ida,
    pub has_virgl_3d: bool,
    pub has_edid: bool,
    pub has_indirect: bool,
    pub has_resource_assign_uuid: bool,
    pub has_resource_blob: bool,
    pub has_host_visible: bool,
    pub has_context_init: bool,
    pub has_blob_alignment: bool,
    pub hibernated: bool,
    pub host_visible_region: virtio_shm_region,
    pub host_visible_mm: drm_mm,
    pub config_changed_work: work_struct,
    pub obj_free_work: work_struct,
    pub obj_free_lock: spinlock_t,
    pub obj_free_list: list_head,
    pub obj_restore_lock: mutex,
    pub obj_restore_list: list_head,
    pub capsets: *mut virtio_gpu_drv_capset,
    pub num_capsets: u32,
    pub capset_id_mask: u64,
    pub cap_cache: list_head,
    pub blob_alignment: u32,
    pub pm_nb: notifier_block,
// protects uuid state when exporting
    pub resource_export_lock: spinlock_t,
// protects map state and host_visible_mm
    pub host_visible_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_fpriv {
    pub ctx_id: u32,
    pub context_init: u32,
    pub context_created: bool,
    pub num_rings: u32,
    pub base_fence_ctx: u64,
    pub ring_idx_mask: u64,
    pub context_lock: mutex,
    pub debug_name: [c_char; DEBUG_NAME_MAX_LEN],
    pub explicit_debug_name: bool,
}

// virtgpu_ioctl.c
pub const DRM_VIRTIO_NUM_IOCTLS: c_int = 12;
extern "C" {
    pub fn virtio_gpu_create_context(dev: *mut drm_device, file: *mut drm_file);
}
// virtgpu_kms.c
extern "C" {
    pub fn virtio_gpu_init(vdev: *mut virtio_device, dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_deinit(dev: *mut drm_device);
}
extern "C" {
    pub fn virtio_gpu_release(dev: *mut drm_device);
}
extern "C" {
    pub fn virtio_gpu_driver_open(dev: *mut drm_device, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_driver_postclose(dev: *mut drm_device, file: *mut drm_file);
}
extern "C" {
    pub fn virtio_gpu_find_vqs(vgdev: *mut virtio_gpu_device) -> c_int;
}
// virtgpu_gem.c
extern "C" {
    pub fn virtio_gpu_array_lock_resv(objs: *mut virtio_gpu_object_array) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_lock_one_resv_uninterruptible(objs: *mut virtio_gpu_object_array) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_array_unlock_resv(objs: *mut virtio_gpu_object_array);
}
extern "C" {
    pub fn virtio_gpu_array_put_free(objs: *mut virtio_gpu_object_array);
}
extern "C" {
    pub fn virtio_gpu_array_put_free_work(work: *mut work_struct);
}
// virtgpu_vq.c
extern "C" {
    pub fn virtio_gpu_alloc_vbufs(vgdev: *mut virtio_gpu_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_free_vbufs(vgdev: *mut virtio_gpu_device);
}
extern "C" {
    pub fn virtio_gpu_reclaim_vbufs(vgdev: *mut virtio_gpu_device);
}
extern "C" {
    pub fn virtio_gpu_detach_object_fenced(bo: *mut virtio_gpu_object) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_cmd_get_display_info(vgdev: *mut virtio_gpu_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_cmd_get_capset_info(vgdev: *mut virtio_gpu_device, idx: c_int) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_cmd_get_edids(vgdev: *mut virtio_gpu_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_ctrl_ack(vq: *mut virtqueue);
}
extern "C" {
    pub fn virtio_gpu_cursor_ack(vq: *mut virtqueue);
}
extern "C" {
    pub fn virtio_gpu_dequeue_ctrl_func(work: *mut work_struct);
}
extern "C" {
    pub fn virtio_gpu_dequeue_cursor_func(work: *mut work_struct);
}
extern "C" {
    pub fn virtio_gpu_panic_notify(vgdev: *mut virtio_gpu_device);
}
extern "C" {
    pub fn virtio_gpu_notify(vgdev: *mut virtio_gpu_device);
}
extern "C" {
    pub fn virtio_gpu_wait_queue(vgvq: *mut virtio_gpu_queue, num_elem: c_uint) -> c_int;
}
// virtgpu_display.c
extern "C" {
    pub fn virtio_gpu_modeset_init(vgdev: *mut virtio_gpu_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_modeset_fini(vgdev: *mut virtio_gpu_device);
}
// virtgpu_plane.c
extern "C" {
    pub fn virtio_gpu_translate_format(drm_fourcc: u32) -> u32;
}
// virtgpu_fence.c
// virtgpu_object.c
extern "C" {
    pub fn virtio_gpu_remove_from_restore_list(bo: *mut virtio_gpu_object);
}
extern "C" {
    pub fn virtio_gpu_cleanup_object(bo: *mut virtio_gpu_object);
}
extern "C" {
    pub fn virtio_gpu_is_shmem(bo: *mut virtio_gpu_object) -> bool;
}
extern "C" {
    pub fn virtio_gpu_object_restore_all(vgdev: *mut virtio_gpu_device) -> c_int;
}
extern "C" {
    pub fn virtio_gpu_object_unref_all(vgdev: *mut virtio_gpu_device);
}
// virtgpu_prime.c
// virtgpu_debugfs.c
extern "C" {
    pub fn virtio_gpu_debugfs_init(minor: *mut drm_minor);
}
// virtgpu_vram.c
extern "C" {
    pub fn virtio_gpu_is_vram(bo: *mut virtio_gpu_object) -> bool;
}
extern "C" {
    pub fn virtio_gpu_vram_map_deferred(vram: *mut virtio_gpu_object_vram);
}
// virtgpu_submit.c
