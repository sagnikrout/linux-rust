//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_drv.h
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
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// Copyright (c) 2009-2010, Code Aurora Forum.
// Copyright 2016 Intel Corp.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// enum drm_driver_feature - feature flags
//
// See &drm_driver.driver_features, drm_device.driver_features and
// drm_core_check_feature().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_driver_feature {
//
// @DRIVER_GEM:
//
// Driver use the GEM memory manager. This should be set for all modern
// drivers.
//
    DRIVER_GEM			= BIT(0),
//
// @DRIVER_MODESET:
//
// Driver supports mode setting interfaces (KMS).
//
    DRIVER_MODESET			= BIT(1),
//
// @DRIVER_RENDER:
//
// Driver supports dedicated render nodes. See also the :ref:`section on
// render nodes <drm_render_node>` for details.
//
    DRIVER_RENDER			= BIT(3),
//
// @DRIVER_ATOMIC:
//
// Driver supports the full atomic modesetting userspace API. Drivers
// which only use atomic internally, but do not support the full
// userspace API (e.g. not all properties converted to atomic, or
// multi-plane updates are not guaranteed to be tear-free) should not
// set this flag.
//
    DRIVER_ATOMIC			= BIT(4),
//
// @DRIVER_SYNCOBJ:
//
// Driver supports &drm_syncobj for explicit synchronization of command
// submission.
//
    DRIVER_SYNCOBJ                  = BIT(5),
//
// @DRIVER_SYNCOBJ_TIMELINE:
//
// Driver supports the timeline flavor of &drm_syncobj for explicit
// synchronization of command submission.
//
    DRIVER_SYNCOBJ_TIMELINE         = BIT(6),
//
// @DRIVER_COMPUTE_ACCEL:
//
// Driver supports compute acceleration devices. This flag is mutually exclusive with
// @DRIVER_RENDER and @DRIVER_MODESET. Devices that support both graphics and compute
// acceleration should be handled by two drivers that are connected using auxiliary bus.
//
    DRIVER_COMPUTE_ACCEL            = BIT(7),
//
// @DRIVER_CURSOR_HOTSPOT:
//
// Driver supports and requires cursor hotspot information in the
// cursor plane (e.g. cursor plane has to actually track the mouse
// cursor and the clients are required to set hotspot in order for
// the cursor planes to work correctly).
//
    DRIVER_CURSOR_HOTSPOT           = BIT(9),

// IMPORTANT: Below are all the legacy flags, add new ones above.

//
// @DRIVER_USE_AGP:
//
// Set up DRM AGP support, see drm_agp_init(), the DRM core will manage
// AGP resources. New drivers don't need this.
//
    DRIVER_USE_AGP			= BIT(25),
//
// @DRIVER_LEGACY:
//
// Denote a legacy driver using shadow attach. Do not use.
//
    DRIVER_LEGACY			= BIT(26),
//
// @DRIVER_PCI_DMA:
//
// Driver is capable of PCI DMA, mapping of PCI DMA buffers to userspace
// will be enabled. Only for legacy drivers. Do not use.
//
    DRIVER_PCI_DMA			= BIT(27),
//
// @DRIVER_SG:
//
// Driver can perform scatter/gather DMA, allocation and mapping of
// scatter/gather buffers will be enabled. Only for legacy drivers. Do
// not use.
//
    DRIVER_SG			= BIT(28),

//
// @DRIVER_HAVE_DMA:
//
// Driver supports DMA, the userspace DMA API will be supported. Only
// for legacy drivers. Do not use.
//
    DRIVER_HAVE_DMA			= BIT(29),
//
// @DRIVER_HAVE_IRQ:
//
// Legacy irq support. Only for legacy drivers. Do not use.
//
    DRIVER_HAVE_IRQ			= BIT(30),
}

//
// struct drm_driver - DRM driver structure
//
// This structure represent the common code for a family of cards. There will be
// one &struct drm_device for each card present in this family. It contains lots
// of vfunc entries, and a pile of those probably should be moved to more
// appropriate places like &drm_mode_config_funcs or into a new operations
// structure for GEM drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_driver {
//
// @load:
//
// Backward-compatible driver callback to complete initialization steps
// after the driver is registered.  For this reason, may suffer from
// race conditions and its use is deprecated for new drivers.  It is
// therefore only supported for existing drivers not yet converted to
// the new scheme.  See devm_drm_dev_alloc() and drm_dev_register() for
// proper and race-free way to set up a &struct drm_device.
//
// This is deprecated, do not use!
//
// Returns:
//
// Zero on success, non-zero value on failure.
//
    pub flags): *mut *mut *mut int (load) (struct drm_device , unsigned long,
//
// @open:
//
// Driver callback when a new &struct drm_file is opened. Useful for
// setting up driver-private data structures like buffer allocators,
// execution contexts or similar things. Such driver-private resources
// must be released again in @postclose.
//
// Since the display/modeset side of DRM can only be owned by exactly
// one &struct drm_file (see &drm_file.is_master and &drm_device.master)
// there should never be a need to set up any modeset related resources
// in this callback. Doing so would be a driver design bug.
//
// Returns:
//
// 0 on success, a negative error code on failure, which will be
// promoted to userspace as the result of the open() system call.
//
    pub ): *mut *mut *mut int (open) (struct drm_device , struct drm_file,
//
// @postclose:
//
// One of the driver callbacks when a new &struct drm_file is closed.
// Useful for tearing down driver-private data structures allocated in
// @open like buffer allocators, execution contexts or similar things.
//
// Since the display/modeset side of DRM can only be owned by exactly
// one &struct drm_file (see &drm_file.is_master and &drm_device.master)
// there should never be a need to tear down any modeset related
// resources in this callback. Doing so would be a driver design bug.
//
    pub ): *mut *mut *mut void (postclose) (struct drm_device , struct drm_file,
//
// @unload:
//
// Reverse the effects of the driver load callback.  Ideally,
// the clean up performed by the driver should happen in the
// reverse order of the initialization.  Similarly to the load
// hook, this handler is deprecated and its usage should be
// dropped in favor of an open-coded teardown function at the
// driver layer.  See drm_dev_unregister() and drm_dev_put()
// for the proper way to remove a &struct drm_device.
//
// The unload() hook is called right after unregistering
// the device.
//
    pub ): *mut *mut void (unload) (struct drm_device,
//
// @release:
//
// Optional callback for destroying device data after the final
// reference is released, i.e. the device is being destroyed.
//
// This is deprecated, clean up all memory allocations associated with a
// &drm_device using drmm_add_action(), drmm_kmalloc() and related
// managed resources functions.
//
    pub ): *mut *mut void (release) (struct drm_device,
//
// @master_set:
//
// Called whenever the minor master is set. Only used by vmwgfx.
//
    pub from_open): bool,
//
// @master_drop:
//
// Called whenever the minor master is dropped. Only used by vmwgfx.
//
    pub file_priv): *mut *mut *mut void (master_drop)(struct drm_device dev, struct drm_file,
//
// @debugfs_init:
//
// Allows drivers to create driver-specific debugfs files.
//
    pub minor): *mut *mut void (debugfs_init)(struct drm_minor,
//
// @gem_create_object: constructor for gem objects
//
// Hook for allocating the GEM object struct, for use by the CMA
// and SHMEM GEM helpers. Returns a GEM object on success, or an
// ERR_PTR()-encoded error code otherwise.
//
    pub size): usize,
//
// @prime_handle_to_fd:
//
// PRIME export function. Only used by vmwgfx.
//
    pub prime_fd): *mut uint32_t handle, uint32_t flags, int,
//
// @prime_fd_to_handle:
//
// PRIME import function. Only used by vmwgfx.
//
    pub handle): *mut int prime_fd, uint32_t,
//
// @gem_prime_import:
//
// Import hook for GEM drivers.
//
// This defaults to drm_gem_prime_import() if not set.
//
    pub dma_buf): *mut dma_buf,
//
// @gem_prime_import_sg_table:
//
// Optional hook used by the PRIME helper functions
// drm_gem_prime_import() respectively drm_gem_prime_import_dev().
//
    pub sgt): *mut sg_table,
//
// @dumb_create:
//
// This creates a new dumb buffer in the driver's backing storage manager (GEM,
// TTM or something else entirely) and returns the resulting buffer handle. This
// handle can then be wrapped up into a framebuffer modeset object.
//
// Note that userspace is not allowed to use such objects for render
// acceleration - drivers must create their own private ioctls for such a use
// case.
//
// Width, height and depth are specified in the &drm_mode_create_dumb
// argument. The callback needs to fill the handle, pitch and size for
// the created buffer.
//
// Called by the user via ioctl.
//
// Returns:
//
// Zero on success, negative errno on failure.
//
    pub args): *mut drm_mode_create_dumb,
//
// @dumb_map_offset:
//
// Allocate an offset in the drm device node's address space to be able to
// memory map a dumb buffer.
//
// The default implementation is drm_gem_create_mmap_offset(). GEM based
// drivers must not overwrite this.
//
// Called by the user via ioctl.
//
// Returns:
//
// Zero on success, negative errno on failure.
//
    pub offset): *mut u64,
//
// @fbdev_probe:
//
// Allocates and initialize the fb_info structure for fbdev emulation.
// Furthermore it also needs to allocate the DRM framebuffer used to
// back the fbdev.
//
// This callback is mandatory for fbdev support.
//
// Returns:
//
// 0 on success ot a negative error code otherwise.
//
    pub sizes): *mut drm_fb_helper_surface_size,
//
// @show_fdinfo:
//
// Print device specific fdinfo.  See Documentation/gpu/drm-usage-stats.rst.
//
    pub f): *mut *mut *mut void (show_fdinfo)(struct drm_printer p, struct drm_file,
// @major: driver major number
    pub major: c_int,
// @minor: driver minor number
    pub minor: c_int,
// @patchlevel: driver patch level
    pub patchlevel: c_int,
// @name: driver name
    pub name: *mut c_char,
// @desc: driver description
    pub desc: *mut c_char,
//
// @driver_features:
// Driver features, see &enum drm_driver_feature. Drivers can disable
// some features on a per-instance basis using
// &drm_device.driver_features.
//
    pub driver_features: u32,
//
// @ioctls:
//
// Array of driver-private IOCTL description entries. See the chapter on
// :ref:`IOCTL support in the userland interfaces
// chapter<drm_driver_ioctl>` for the full details.
//
    pub ioctls: *const drm_ioctl_desc,
// @num_ioctls: Number of entries in @ioctls.
    pub num_ioctls: c_int,
//
// @fops:
//
// File operations for the DRM device node. See the discussion in
// :ref:`file operations<drm_driver_fops>` for in-depth coverage and
// some examples.
//
    pub fops: *const file_operations,
}

//
// devm_drm_dev_alloc - Resource managed allocation of a &drm_device instance
// @parent: Parent device object
// @driver: DRM driver
// @type: the type of the struct which contains struct &drm_device
// @member: the name of the &drm_device within @type.
//
// This allocates and initialize a new DRM device. No device registration is done.
// Call drm_dev_register() to advertice the device to user space and register it
// with other core subsystems. This should be done last in the device
// initialization sequence to make sure userspace can't access an inconsistent
// state.
//
// The initial ref-count of the object is 1. Use drm_dev_get() and
// drm_dev_put() to take and drop further ref-counts.
//
// It is recommended that drivers embed &struct drm_device into their own device
// structure.
//
// Note that this manages the lifetime of the resulting &drm_device
// automatically using devres. The DRM device initialized with this function is
// automatically put on driver detach using drm_dev_put().
//
// RETURNS:
// Pointer to new DRM device, or ERR_PTR on failure.
//

extern "C" {
    pub fn drm_dev_register(dev: *mut drm_device, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn drm_dev_unregister(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_dev_get(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_dev_put(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_put_dev(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_dev_enter(dev: *mut drm_device, idx: *mut c_int) -> bool;
}
extern "C" {
    pub fn drm_dev_exit(idx: c_int);
}
extern "C" {
    pub fn drm_dev_unplug(dev: *mut drm_device);
}
//
// drm_dev_is_unplugged - is a DRM device unplugged
// @dev: DRM device
//
// This function can be called to check whether a hotpluggable is unplugged.
// Unplugging itself is singalled through drm_dev_unplug(). If a device is
// unplugged, these two functions guarantee that any store before calling
// drm_dev_unplug() is visible to callers of this function after it completes
//
// WARNING: This function fundamentally races against drm_dev_unplug(). It is
// recommended that drivers instead use the underlying drm_dev_enter() and
// drm_dev_exit() function pairs.
//
// drm_core_check_all_features - check driver feature flags mask
// @dev: DRM device to check
// @features: feature flag(s) mask
//
// This checks @dev for driver features, see &drm_driver.driver_features,
// &drm_device.driver_features, and the various &enum drm_driver_feature flags.
//
// Returns true if all features in the @features mask are supported, false
// otherwise.
//
// drm_core_check_feature - check driver feature flags
// @dev: DRM device to check
// @feature: feature flag
//
// This checks @dev for driver features, see &drm_driver.driver_features,
// &drm_device.driver_features, and the various &enum drm_driver_feature flags.
//
// Returns true if the @feature is supported, false otherwise.
//
extern "C" {
    pub fn drm_core_check_all_features(_arg: dev, _arg: feature) -> return;
}
//
// drm_drv_uses_atomic_modeset - check if the driver implements
// atomic_commit()
// @dev: DRM device
//
// This check is useful if drivers do not have DRIVER_ATOMIC set but
// have atomic modesetting internally implemented.
//
// TODO: Inline drm_firmware_drivers_only() in all its callers.
extern "C" {
    pub fn video_firmware_drivers_only() -> return;
}

extern "C" {
    pub fn drm_debugfs_dev_init(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_debugfs_init_root();
}
extern "C" {
    pub fn drm_debugfs_remove_root();
}
extern "C" {
    pub fn drm_debugfs_bridge_params();
}

