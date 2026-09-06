//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_config.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_shm_region {
    pub addr: u64,
    pub len: u64,
}

extern "C" {
    pub fn vq_callback_t(: *mut virtqueue) -> typedef void;
}
//
// struct virtqueue_info - Info for a virtqueue passed to find_vqs().
// @name: virtqueue description. Used mainly for debugging, NULL for
// a virtqueue unused by the driver.
// @callback: A callback to invoke on a used buffer notification.
// NULL for a virtqueue that does not need a callback.
// @ctx: whether to maintain an extra context per virtqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtqueue_info {
    pub name: *const c_char,
    pub callback: *mut vq_callback_t,
    pub ctx: bool,
}

//
// struct virtio_config_ops - operations for configuring a virtio device
// Note: Do not assume that a transport implements all of the operations
// getting/setting a value as a simple read/write! Generally speaking,
// any of @get/@set, @get_status/@set_status, or @get_features
// @finalize_features are NOT safe to be called from an atomic
// context.
// @get: read the value of a configuration field
// vdev: the virtio_device
// offset: the offset of the configuration field
// buf: the buffer to write the field value into.
// len: the length of the buffer
// @set: write the value of a configuration field
// vdev: the virtio_device
// offset: the offset of the configuration field
// buf: the buffer to read the field value from.
// len: the length of the buffer
// @generation: config generation counter (optional)
// vdev: the virtio_device
// Returns the config generation counter
// @get_status: read the status byte
// vdev: the virtio_device
// Returns the status byte
// @set_status: write the status byte
// vdev: the virtio_device
// status: the new status byte
// @reset: reset the device
// vdev: the virtio device
// After this, status and feature negotiation must be done again
// Device must not be reset from its vq/config callbacks, or in
// parallel with being added/removed.
// @find_vqs: find virtqueues and instantiate them.
// vdev: the virtio_device
// nvqs: the number of virtqueues to find
// vqs: on success, includes new virtqueues
// vqs_info: array of virtqueue info structures
// Returns 0 on success or error status
// @del_vqs: free virtqueues found by find_vqs().
// @synchronize_cbs: synchronize with the virtqueue callbacks (optional)
// The function guarantees that all memory operations on the
// queue before it are visible to the vring_interrupt() that is
// called after it.
// vdev: the virtio_device
// @get_features: get the array of feature bits for this device.
// vdev: the virtio_device
// Returns the first 64 feature bits.
// @get_extended_features:
// vdev: the virtio_device
// Returns the first VIRTIO_FEATURES_BITS feature bits (all we currently
// need).
// @finalize_features: confirm what device features we'll be using.
// vdev: the virtio_device
// This sends the driver feature bits to the device: it can change
// the dev->feature bits if it wants.
// Note that despite the name this can be called any number of
// times.
// Returns 0 on success or error status
// @bus_name: return the bus name associated with the device (optional)
// vdev: the virtio_device
// This returns a pointer to the bus name a la pci_name from which
// the caller can then copy.
// @set_vq_affinity: set the affinity for a virtqueue (optional).
// @get_vq_affinity: get the affinity for a virtqueue (optional).
// @get_shm_region: get a shared memory region based on the index.
// @disable_vq_and_reset: reset a queue individually (optional).
// vq: the virtqueue
// Returns 0 on success or error status
// disable_vq_and_reset will guarantee that the callbacks are disabled and
// synchronized.
// Except for the callback, the caller should guarantee that the vring is
// not accessed by any functions of virtqueue.
// @enable_vq_after_reset: enable a reset queue
// vq: the virtqueue
// Returns 0 on success or error status
// If disable_vq_and_reset is set, then enable_vq_after_reset must also be
// set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_config_ops {
    pub len): *mut *mut void buf, unsigned,
    pub len): *const *const void buf, unsigned,
    pub vdev): *mut *mut u32 (generation)(struct virtio_device,
    pub vdev): *mut *mut u8 (get_status)(struct virtio_device,
    pub status): *mut *mut *mut void (set_status)(struct virtio_device vdev, u8,
    pub vdev): *mut *mut void (reset)(struct virtio_device,
    pub desc): *mut irq_affinity,
    pub ): *mut *mut void (del_vqs)(struct virtio_device,
    pub ): *mut *mut void (synchronize_cbs)(struct virtio_device,
    pub vdev): *mut *mut u64 (get_features)(struct virtio_device,
    pub features): *mut u64,
    pub vdev): *mut *mut int (finalize_features)(struct virtio_device,
    pub vdev): *const *const *const char (bus_name)(struct virtio_device,
    pub cpu_mask): *const cpumask,
    pub index): c_int,
    pub id): *mut *mut virtio_shm_region region, u8,
    pub vq): *mut *mut int (disable_vq_and_reset)(struct virtqueue,
    pub vq): *mut *mut int (enable_vq_after_reset)(struct virtqueue,
}

//
// struct virtio_map_ops - operations for mapping buffer for a virtio device
// Note: For a transport that has its own mapping logic it must
// implement all of the operations
// @map_page: map a buffer to the device
// map: metadata for performing mapping
// page: the page that will be mapped by the device
// offset: the offset in the page for a buffer
// size: the buffer size
// dir: mapping direction
// attrs: mapping attributes
// Returns the mapped address
// @unmap_page: unmap a buffer from the device
// map: device specific mapping map
// map_handle: the mapped address
// size: the buffer size
// dir: mapping direction
// attrs: unmapping attributes
// @sync_single_for_cpu: sync a single buffer from device to cpu
// map: metadata for performing mapping
// map_handle: the mapping address to sync
// size: the size of the buffer
// dir: synchronization direction
// @sync_single_for_device: sync a single buffer from cpu to device
// map: metadata for performing mapping
// map_handle: the mapping address to sync
// size: the size of the buffer
// dir: synchronization direction
// @alloc: alloc a coherent buffer mapping
// map: metadata for performing mapping
// size: the size of the buffer
// map_handle: the mapping address to sync
// gfp: allocation flag (GFP_XXX)
// Returns virtual address of the allocated buffer
// @free: free a coherent buffer mapping
// map: metadata for performing mapping
// size: the size of the buffer
// vaddr: virtual address of the buffer
// map_handle: the mapping address that needs to be freed
// attrs: unmapping attributes
// @need_sync: if the buffer needs synchronization
// map: metadata for performing mapping
// map_handle: the mapped address
// Returns whether the buffer needs synchronization
// @mapping_error: if the mapping address is error
// map: metadata for performing mapping
// map_handle: the mapped address
// @max_mapping_size: get the maximum buffer size that can be mapped
// map: metadata for performing mapping
// Returns the maximum buffer size that can be mapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_map_ops {
    pub attrs): dma_data_direction dir, unsigned long,
    pub attrs): c_ulong,
    pub dir): size_t size, enum dma_data_direction,
    pub dir): dma_data_direction,
    pub gfp): *mut *mut dma_addr_t map_handle, gfp_t,
    pub attrs): dma_addr_t map_handle, unsigned long,
    pub map_handle): *mut *mut bool (need_sync)(union virtio_map map, dma_addr_t,
    pub map_handle): *mut *mut int (mapping_error)(union virtio_map map, dma_addr_t,
    pub map): *mut *mut size_t (max_mapping_size)(union virtio_map,
}

// If driver didn't advertise the feature, it will never appear.
//
// __virtio_test_bit - helper to test feature bits. For use by transports.
// Devices should normally use virtio_has_feature,
// which includes more checks.
// @vdev: the device
// @fbit: the feature bit
//
extern "C" {
    pub fn virtio_features_test_bit(_arg: vdev->features_array, _arg: fbit) -> return;
}
//
// __virtio_set_bit - helper to set feature bits. For use by transports.
// @vdev: the device
// @fbit: the feature bit
//
// __virtio_clear_bit - helper to clear feature bits. For use by transports.
// @vdev: the device
// @fbit: the feature bit
//
// virtio_has_feature - helper to determine if this device has this feature.
// @vdev: the device
// @fbit: the feature bit
//
extern "C" {
    pub fn __virtio_test_bit(_arg: vdev, _arg: fbit) -> return;
}
//
// virtio_has_dma_quirk - determine whether this device has the DMA quirk
// @vdev: the device
//
// Note the reverse polarity of the quirk feature (compared to most
// other features), this is for compatibility with legacy systems.
//
extern "C" {
    pub fn ERR_PTR(_arg: err) -> return;
}
//
// virtio_synchronize_cbs - synchronize with virtqueue callbacks
// @dev: the virtio device
//
// A best effort fallback to synchronize with
// interrupts, preemption and softirq disabled
// regions. See comment above synchronize_rcu().
//
// virtio_device_ready - enable vq use in probe function
// @dev: the virtio device
//
// Driver must call this to use vqs in the probe function.
//
// Note: vqs are enabled automatically after probe returns.
//

//
// The virtio_synchronize_cbs() makes sure vring_interrupt()
// will see the driver specific setup if it sees vq->broken
// as false (even if the notifications come before DRIVER_OK).
//

//
// The transport should ensure the visibility of vq->broken
// before setting DRIVER_OK. See the comments for the transport
// specific set_status() method.
//
// A well behaved device will only notify a virtqueue after
// DRIVER_OK, this means the device should "see" the coherent
// memory write that set vq->broken as false which is done by
// the driver when it sees DRIVER_OK, then the following
// driver's vring_interrupt() will see vq->broken as false so
// we won't lose any notification.
//
// virtqueue_set_affinity - setting affinity for a virtqueue
// @vq: the virtqueue
// @cpu_mask: the cpu mask
//
// Note that this function is best-effort: the affinity hint may not be set
// due to config support, irq type and sharing.
//
// Memory accessors
extern "C" {
    pub fn __virtio16_to_cpu(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio16(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio32_to_cpu(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio32(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio64_to_cpu(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio64(_arg: virtio_is_little_endian(vdev), _arg: val) -> return;
}

// Config space accessors.

// Sanity check: must match the member's type */	\
// (ptr) = virtio_to_cpu(vdev, virtio_cread_v);		\
// Config space accessors.

// Sanity check: must match the member's type */	\
//
// Nothing virtio-specific about these, but let's worry about generalizing
// these later.
//

// LE (e.g. modern) Config space accessors.

// Sanity check: must match the member's type */	\
// (ptr) = virtio_le_to_cpu(virtio_cread_v);		\

// Sanity check: must match the member's type */	\
// Read @count fields, @bytes each.
extern "C" {
    pub fn virtio16_to_cpu(_arg: vdev, _arg: ret) -> return;
}
extern "C" {
    pub fn virtio32_to_cpu(_arg: vdev, _arg: ret) -> return;
}
extern "C" {
    pub fn virtio64_to_cpu(_arg: vdev, _arg: ret) -> return;
}
// Conditional config space accessors.

// Conditional config space accessors.

