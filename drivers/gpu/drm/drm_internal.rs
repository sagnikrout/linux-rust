//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/drm_internal.h
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
// Copyright © 2014 Intel Corporation
// Daniel Vetter <daniel.vetter@ffwll.ch>
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

pub const DRM_IF_MAJOR: c_int = 1;
pub const DRM_IF_MINOR: c_int = 4;

// drm_client_event.c

extern "C" {
    pub fn drm_client_debugfs_init(dev: *mut drm_device);
}

// drm_client_sysrq.c

extern "C" {
    pub fn drm_client_sysrq_register(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_client_sysrq_unregister(dev: *mut drm_device);
}

// drm_file.c
extern "C" {
    pub fn drm_dev_needs_global_mutex(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn drm_file_free(file: *mut drm_file);
}

// drm_pci.c
extern "C" {
    pub fn drm_pci_set_busid(dev: *mut drm_device, master: *mut drm_master) -> c_int;
}

// drm_prime.c
extern "C" {
    pub fn drm_prime_init_file_private(prime_fpriv: *mut drm_prime_file_private);
}
extern "C" {
    pub fn drm_prime_destroy_file_private(prime_fpriv: *mut drm_prime_file_private);
}
// drm_managed.c
extern "C" {
    pub fn drm_managed_release(dev: *mut drm_device);
}
extern "C" {
    pub fn drmm_add_final_kfree(dev: *mut drm_device, container: *mut c_void);
}
// drm_vblank.c
extern "C" {
    pub fn drm_vblank_disable_and_save(dev: *mut drm_device, pipe: c_uint);
}
extern "C" {
    pub fn drm_vblank_get(dev: *mut drm_device, pipe: c_uint) -> c_int;
}
extern "C" {
    pub fn drm_vblank_put(dev: *mut drm_device, pipe: c_uint);
}
extern "C" {
    pub fn drm_vblank_count(dev: *mut drm_device, pipe: c_uint) -> u64;
}
// drm_vblank_work.c
extern "C" {
    pub fn drm_vblank_worker_init(vblank: *mut drm_vblank_crtc) -> c_int;
}
extern "C" {
    pub fn drm_vblank_cancel_pending_works(vblank: *mut drm_vblank_crtc);
}
extern "C" {
    pub fn drm_handle_vblank_works(vblank: *mut drm_vblank_crtc);
}
// IOCTLS
// drm_irq.c
// IOCTLS
// drm_auth.c
extern "C" {
    pub fn drm_master_open(file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn drm_master_release(file_priv: *mut drm_file);
}
extern "C" {
    pub fn drm_master_internal_acquire(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn drm_master_internal_release(dev: *mut drm_device);
}
// drm_sysfs.c
extern "C" {
    pub fn drm_sysfs_init() -> c_int;
}
extern "C" {
    pub fn drm_sysfs_destroy();
}
extern "C" {
    pub fn drm_sysfs_connector_add(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_sysfs_connector_add_late(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_sysfs_connector_remove_early(connector: *mut drm_connector);
}
extern "C" {
    pub fn drm_sysfs_connector_remove(connector: *mut drm_connector);
}
extern "C" {
    pub fn drm_sysfs_lease_event(dev: *mut drm_device);
}
// drm_gem.c
extern "C" {
    pub fn drm_gem_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_gem_object_handle_get_if_exists_unlocked(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn drm_gem_object_handle_put_unlocked(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_open(dev: *mut drm_device, file_private: *mut drm_file);
}
extern "C" {
    pub fn drm_gem_release(dev: *mut drm_device, file_private: *mut drm_file);
}
extern "C" {
    pub fn drm_gem_vmap_locked(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn drm_gem_vunmap_locked(obj: *mut drm_gem_object, map: *mut iosys_map);
}
// drm_debugfs.c drm_debugfs_crc.c

extern "C" {
    pub fn drm_debugfs_dev_fini(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_debugfs_dev_register(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_debugfs_register(minor: *mut drm_minor, minor_id: c_int) -> c_int;
}
extern "C" {
    pub fn drm_debugfs_unregister(minor: *mut drm_minor);
}
extern "C" {
    pub fn drm_debugfs_connector_add(connector: *mut drm_connector);
}
extern "C" {
    pub fn drm_debugfs_connector_remove(connector: *mut drm_connector);
}
extern "C" {
    pub fn drm_debugfs_crtc_add(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn drm_debugfs_crtc_remove(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn drm_debugfs_crtc_crc_add(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn drm_debugfs_encoder_add(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn drm_debugfs_encoder_remove(encoder: *mut drm_encoder);
}

// drm_syncobj.c
extern "C" {
    pub fn drm_syncobj_open(file_private: *mut drm_file);
}
extern "C" {
    pub fn drm_syncobj_release(file_private: *mut drm_file);
}
// drm_framebuffer.c
extern "C" {
    pub fn drm_framebuffer_debugfs_init(dev: *mut drm_device);
}
