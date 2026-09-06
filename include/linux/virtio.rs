//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio.h
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
// Everything a virtio driver needs to work with any particular virtio
// implementation.

//
// struct virtqueue - a queue to register buffers for sending or receiving.
// @list: the chain of virtqueues for this device
// @callback: the function to call when buffers are consumed (can be NULL).
// @name: the name of this virtqueue (mainly for debugging)
// @vdev: the virtio device this queue was created for.
// @priv: a pointer for the virtqueue implementation to use.
// @index: the zero-based ordinal number for this queue.
// @num_free: number of elements we expect to be able to fit.
// @num_max: the maximum number of elements supported by the device.
// @reset: vq is in reset state or not.
//
// A note on @num_free: with indirect buffers, each buffer needs one
// element in the queue, otherwise a buffer will need one element per
// sg element.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtqueue {
    pub list: list_head,
    pub vq): *mut *mut void (callback)(struct virtqueue,
    pub name: *const c_char,
    pub vdev: *mut virtio_device,
    pub index: c_uint,
    pub num_free: c_uint,
    pub num_max: c_uint,
    pub reset: bool,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union virtio_map {
// Device that performs DMA
    pub dma_dev: *mut device,
// VDUSE specific virtqueue group for doing map
    pub group: *mut vduse_vq_group,
}

extern "C" {
    pub fn virtqueue_kick(vq: *mut virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_kick_prepare(vq: *mut virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_notify(vq: *mut virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_disable_cb(vq: *mut virtqueue);
}
extern "C" {
    pub fn virtqueue_enable_cb(vq: *mut virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_enable_cb_prepare(vq: *mut virtqueue) -> unsigned;
}
extern "C" {
    pub fn virtqueue_poll(vq: *mut virtqueue, _arg: unsigned) -> bool;
}
extern "C" {
    pub fn virtqueue_enable_cb_delayed(vq: *mut virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_get_vring_size(vq: *const virtqueue) -> c_uint;
}
extern "C" {
    pub fn virtqueue_is_broken(vq: *const virtqueue) -> bool;
}
extern "C" {
    pub fn virtqueue_get_desc_addr(vq: *const virtqueue) -> dma_addr_t;
}
extern "C" {
    pub fn virtqueue_get_avail_addr(vq: *const virtqueue) -> dma_addr_t;
}
extern "C" {
    pub fn virtqueue_get_used_addr(vq: *const virtqueue) -> dma_addr_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd {
    pub opcode: __le16,
    pub group_type: __le16,
    pub group_member_id: __le64,
    pub data_sg: *mut scatterlist,
    pub result_sg: *mut scatterlist,
    pub completion: completion,
    pub result_sg_size: u32,
    pub ret: c_int,
}

//
// struct virtio_device - representation of a device using virtio
// @index: unique position on the virtio bus
// @failed: saved value for VIRTIO_CONFIG_S_FAILED bit (for restore)
// @config_core_enabled: configuration change reporting enabled by core
// @config_driver_disabled: configuration change reporting disabled by
// a driver
// @config_change_pending: configuration change reported while disabled
// @config_lock: protects configuration change reporting
// @vqs_list_lock: protects @vqs.
// @dev: underlying device.
// @id: the device type identification (used to match it with a driver).
// @config: the configuration ops for this device.
// @vringh_config: configuration ops for host vrings.
// @map: the map operations for mapping virtio device memory.
// @vqs: the list of virtqueues for this device.
// @features: the 64 lower features supported by both driver and device.
// @features_array: the full features space supported by both driver and
// device.
// @priv: private pointer for the driver's use.
// @vmap: the map container with transport- or device-specific metadata.
// @debugfs_dir: debugfs directory entry.
// @debugfs_filter_features: features to be filtered set by debugfs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_device {
    pub index: c_int,
    pub failed: bool,
    pub config_core_enabled: bool,
    pub config_driver_disabled: bool,
    pub config_change_pending: bool,
    pub config_lock: spinlock_t,
    pub vqs_list_lock: spinlock_t,
    pub dev: device,
    pub id: virtio_device_id,
    pub config: *const virtio_config_ops,
    pub vringh_config: *const vringh_config_ops,
    pub map: *const virtio_map_ops,
    pub vqs: list_head,
    pub priv: *mut c_void,
    pub vmap: virtio_map,

    pub debugfs_dir: *mut dentry,
    pub debugfs_filter_features: [u64; VIRTIO_FEATURES_U64S],
}

extern "C" {
    pub fn virtio_add_status(dev: *mut virtio_device, status: c_uint);
}
extern "C" {
    pub fn register_virtio_device(dev: *mut virtio_device) -> c_int;
}
extern "C" {
    pub fn unregister_virtio_device(dev: *mut virtio_device);
}
extern "C" {
    pub fn is_virtio_device(dev: *mut device) -> bool;
}
extern "C" {
    pub fn virtio_break_device(dev: *mut virtio_device);
}
extern "C" {
    pub fn __virtio_unbreak_device(dev: *mut virtio_device);
}
extern "C" {
    pub fn __virtqueue_break(_vq: *mut virtqueue);
}
extern "C" {
    pub fn __virtqueue_unbreak(_vq: *mut virtqueue);
}
extern "C" {
    pub fn virtio_config_changed(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_config_driver_disable(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_config_driver_enable(dev: *mut virtio_device);
}

extern "C" {
    pub fn virtio_device_freeze(dev: *mut virtio_device) -> c_int;
}
extern "C" {
    pub fn virtio_device_restore(dev: *mut virtio_device) -> c_int;
}

extern "C" {
    pub fn virtio_reset_device(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_device_shutdown(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_device_reset_prepare(dev: *mut virtio_device) -> c_int;
}
extern "C" {
    pub fn virtio_device_reset_done(dev: *mut virtio_device) -> c_int;
}
extern "C" {
    pub fn virtio_max_dma_size(vdev: *const virtio_device) -> usize;
}

//
// struct virtio_driver - operations for a virtio I/O driver
// @driver: underlying device driver (populate name).
// @id_table: the ids serviced by this driver.
// @feature_table: an array of feature numbers supported by this driver.
// @feature_table_size: number of entries in the feature table array.
// @feature_table_legacy: same as feature_table but when working in legacy mode.
// @feature_table_size_legacy: number of entries in feature table legacy array.
// @validate: the function to call to validate features and config space.
// Returns 0 or -errno.
// @probe: the function to call when a device is found.  Returns 0 or -errno.
// @scan: optional function to call after successful probe; intended
// for virtio-scsi to invoke a scan.
// @remove: the function to call when a device is removed.
// @config_changed: optional function to call when the device configuration
// changes; may be called in interrupt context.
// @freeze: optional function to call during suspend/hibernation.
// @restore: optional function to call on resume.
// @reset_prepare: optional function to call when a transport specific reset
// occurs.
// @reset_done: optional function to call after transport specific reset
// operation has finished.
// @shutdown: synchronize with the device on shutdown. If provided, replaces
// the virtio core implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_driver {
    pub driver: device_driver,
    pub id_table: *const virtio_device_id,
    pub feature_table: *const c_uint,
    pub feature_table_size: c_uint,
    pub feature_table_legacy: *const c_uint,
    pub feature_table_size_legacy: c_uint,
    pub dev): *mut *mut int (validate)(struct virtio_device,
    pub dev): *mut *mut int (probe)(struct virtio_device,
    pub dev): *mut *mut void (scan)(struct virtio_device,
    pub dev): *mut *mut void (remove)(struct virtio_device,
    pub dev): *mut *mut void (config_changed)(struct virtio_device,
    pub dev): *mut *mut int (freeze)(struct virtio_device,
    pub dev): *mut *mut int (restore)(struct virtio_device,
    pub dev): *mut *mut int (reset_prepare)(struct virtio_device,
    pub dev): *mut *mut int (reset_done)(struct virtio_device,
    pub dev): *mut *mut void (shutdown)(struct virtio_device,
}

// use a macro to avoid include chaining to get THIS_MODULE

extern "C" {
    pub fn __register_virtio_driver(drv: *mut virtio_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn unregister_virtio_driver(drv: *mut virtio_driver);
}
// module_virtio_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn virtqueue_map_mapping_error(_vq: *const virtqueue, addr: dma_addr_t) -> c_int;
}
extern "C" {
    pub fn virtqueue_map_need_sync(_vq: *const virtqueue, addr: dma_addr_t) -> bool;
}

extern "C" {
    pub fn virtio_debug_device_init(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_debug_device_exit(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_debug_device_filter_features(dev: *mut virtio_device);
}
extern "C" {
    pub fn virtio_debug_init();
}
extern "C" {
    pub fn virtio_debug_exit();
}

