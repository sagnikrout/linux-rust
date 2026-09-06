//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vdpa.h
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

//
// struct vdpa_callback - vDPA callback definition.
// @callback: interrupt callback function
// @private: the data passed to the callback function
// @trigger: the eventfd for the callback (Optional).
// When it is set, the vDPA driver must guarantee that
// signaling it is functional equivalent to triggering
// the callback. Then vDPA parent can signal it directly
// instead of triggering the callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_callback {
    pub data): *mut *mut irqreturn_t (callback)(void,
    pub private: *mut c_void,
    pub trigger: *mut eventfd_ctx,
}

//
// struct vdpa_notification_area - vDPA notification area
// @addr: base address of the notification area
// @size: size of the notification area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_notification_area {
    pub addr: resource_size_t,
    pub size: resource_size_t,
}

//
// struct vdpa_vq_state_split - vDPA split virtqueue state
// @avail_index: available index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_vq_state_split {
    pub avail_index: u16,
}

//
// struct vdpa_vq_state_packed - vDPA packed virtqueue state
// @last_avail_counter: last driver ring wrap counter observed by device
// @last_avail_idx: device available index
// @last_used_counter: device ring wrap counter
// @last_used_idx: used index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_vq_state_packed {
    pub last_avail_counter:1: u16,
    pub last_avail_idx:15: u16,
    pub last_used_counter:1: u16,
    pub last_used_idx:15: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_vq_state {
    pub split: vdpa_vq_state_split,
    pub packed: vdpa_vq_state_packed,
}

//
// struct vdpa_device - representation of a vDPA device
// @dev: underlying device
// @vmap: the metadata passed to upper layer to be used for mapping
// @config: the configuration ops for this device.
// @map: the map ops for this device
// @cf_lock: Protects get and set access to configuration layout.
// @index: device index
// @features_valid: were features initialized? for legacy guests
// @ngroups: the number of virtqueue groups
// @nas: the number of address spaces
// @use_va: indicate whether virtual address must be used by this device
// @nvqs: maximum number of supported virtqueues
// @mdev: management device pointer; caller must setup when registering device as part
// of dev_add() mgmtdev ops callback before invoking _vdpa_register_device().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_device {
    pub dev: device,
    pub vmap: virtio_map,
    pub config: *const vdpa_config_ops,
    pub map: *const virtio_map_ops,
    pub /: *mut *mut rw_semaphore cf_lock; / Protects get/set config,
    pub index: c_uint,
    pub features_valid: bool,
    pub use_va: bool,
    pub nvqs: u32,
    pub mdev: *mut vdpa_mgmt_dev,
    pub ngroups: c_uint,
    pub nas: c_uint,
}

//
// struct vdpa_iova_range - the IOVA range support by the device
// @first: start of the IOVA range
// @last: end of the IOVA range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_iova_range {
    pub first: u64,
    pub last: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_dev_set_config {
    pub device_features: u64,
    pub mac: [u8; ETH_ALEN],
    pub mtu: u16,
    pub max_vq_pairs: u16,
    pub net: },
    pub mask: u64,
}

//
// struct vdpa_map_file - file area for device memory mapping
// @file: vma->vm_file for the mapping
// @offset: mapping offset in the vm_file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_map_file {
    pub file: *mut file,
    pub offset: u64,
}

//
// struct vdpa_config_ops - operations for configuring a vDPA device.
// Note: vDPA device drivers are required to implement all of the
// operations unless it is mentioned to be optional in the following
// list.
//
// @set_vq_address:		Set the address of virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @desc_area: address of desc area
// @driver_area: address of driver area
// @device_area: address of device area
// Returns integer: success (0) or error (< 0)
// @set_vq_num:			Set the size of virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @num: the size of virtqueue
// @kick_vq:			Kick the virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @kick_vq_with_data:		Kick the virtqueue and supply extra data
// (only if VIRTIO_F_NOTIFICATION_DATA is negotiated)
// @vdev: vdpa device
// @data for split virtqueue:
// 16 bits vqn and 16 bits next available index.
// @data for packed virtqueue:
// 16 bits vqn, 15 least significant bits of
// next available index and 1 bit next_wrap.
// @set_vq_cb:			Set the interrupt callback function for
// a virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @cb: virtio-vdev interrupt callback structure
// @set_vq_ready:		Set ready status for a virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @ready: ready (true) not ready(false)
// @get_vq_ready:		Get ready status for a virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// Returns boolean: ready (true) or not (false)
// @set_vq_state:		Set the state for a virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @state: pointer to set virtqueue state (last_avail_idx)
// Returns integer: success (0) or error (< 0)
// @get_vq_state:		Get the state for a virtqueue
// @vdev: vdpa device
// @idx: virtqueue index
// @state: pointer to returned state (last_avail_idx)
// @get_vendor_vq_stats:	Get the vendor statistics of a device.
// @vdev: vdpa device
// @idx: virtqueue index
// @msg: socket buffer holding stats message
// @extack: extack for reporting error messages
// Returns integer: success (0) or error (< 0)
// @get_vq_notification:	Get the notification area for a virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns the notification area
// @get_vq_irq:			Get the irq number of a virtqueue (optional,
// but must implemented if require vq irq offloading)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns int: irq number of a virtqueue,
// negative number if no irq assigned.
// @get_vq_size:		Get the size of a specific virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Return u16: the size of the virtqueue
// @get_vq_align:		Get the virtqueue align requirement
// for the device
// @vdev: vdpa device
// Returns virtqueue algin requirement
// @get_vq_group:		Get the group id for a specific
// virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns u32: group id for this virtqueue
// @get_vq_desc_group:		Get the group id for the descriptor table of
// a specific virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns u32: group id for the descriptor table
// portion of this virtqueue. Could be different
// than the one from @get_vq_group, in which case
// the access to the descriptor table can be
// confined to a separate asid, isolating from
// the virtqueue's buffer address access.
// @get_device_features:	Get virtio features supported by the device
// @vdev: vdpa device
// Returns the virtio features support by the
// device
// @get_backend_features:	Get parent-specific backend features (optional)
// Returns the vdpa features supported by the
// device.
// @set_driver_features:	Set virtio features supported by the driver
// @vdev: vdpa device
// @features: feature support by the driver
// Returns integer: success (0) or error (< 0)
// @get_driver_features:	Get the virtio driver features in action
// @vdev: vdpa device
// Returns the virtio features accepted
// @set_config_cb:		Set the config interrupt callback
// @vdev: vdpa device
// @cb: virtio-vdev interrupt callback structure
// @get_vq_num_max:		Get the max size of virtqueue
// @vdev: vdpa device
// Returns u16: max size of virtqueue
// @get_vq_num_min:		Get the min size of virtqueue (optional)
// @vdev: vdpa device
// Returns u16: min size of virtqueue
// @get_device_id:		Get virtio device id
// @vdev: vdpa device
// Returns u32: virtio device id
// @get_vendor_id:		Get id for the vendor that provides this device
// @vdev: vdpa device
// Returns u32: virtio vendor id
// @get_status:			Get the device status
// @vdev: vdpa device
// Returns u8: virtio device status
// @set_status:			Set the device status
// @vdev: vdpa device
// @status: virtio device status
// @reset:			Reset device
// @vdev: vdpa device
// Returns integer: success (0) or error (< 0)
// @compat_reset:		Reset device with compatibility quirks to
// accommodate older userspace. Only needed by
// parent driver which used to have bogus reset
// behaviour, and has to maintain such behaviour
// for compatibility with older userspace.
// Historically compliant driver only has to
// implement .reset, Historically non-compliant
// driver should implement both.
// @vdev: vdpa device
// @flags: compatibility quirks for reset
// Returns integer: success (0) or error (< 0)
// @suspend:			Suspend the device (optional)
// @vdev: vdpa device
// Returns integer: success (0) or error (< 0)
// @resume:			Resume the device (optional)
// @vdev: vdpa device
// Returns integer: success (0) or error (< 0)
// @get_config_size:		Get the size of the configuration space includes
// fields that are conditional on feature bits.
// @vdev: vdpa device
// Returns size_t: configuration size
// @get_config:			Read from device specific configuration space
// @vdev: vdpa device
// @offset: offset from the beginning of
// configuration space
// @buf: buffer used to read to
// @len: the length to read from
// configuration space
// @set_config:			Write to device specific configuration space
// @vdev: vdpa device
// @offset: offset from the beginning of
// configuration space
// @buf: buffer used to write from
// @len: the length to write to
// configuration space
// @get_generation:		Get device config generation (optional)
// @vdev: vdpa device
// Returns u32: device generation
// @get_iova_range:		Get supported iova range (optional)
// @vdev: vdpa device
// Returns the iova range supported by
// the device.
// @set_vq_affinity:		Set the affinity of virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// @cpu_mask: the affinity mask
// Returns integer: success (0) or error (< 0)
// @get_vq_affinity:		Get the affinity of virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns the affinity mask
// @set_group_asid:		Set address space identifier for a
// virtqueue group (optional).  Caller must
// prevent this from being executed concurrently
// with set_status.
// @vdev: vdpa device
// @group: virtqueue group
// @asid: address space id for this group
// Returns integer: success (0) or error (< 0)
// @set_map:			Set device memory mapping (optional)
// Needed for device that using device
// specific DMA translation (on-chip IOMMU)
// @vdev: vdpa device
// @asid: address space identifier
// @iotlb: vhost memory mapping to be
// used by the vDPA
// Returns integer: success (0) or error (< 0)
// @dma_map:			Map an area of PA to IOVA (optional)
// Needed for device that using device
// specific DMA translation (on-chip IOMMU)
// and preferring incremental map.
// @vdev: vdpa device
// @asid: address space identifier
// @iova: iova to be mapped
// @size: size of the area
// @pa: physical address for the map
// @perm: device access permission (VHOST_MAP_XX)
// Returns integer: success (0) or error (< 0)
// @dma_unmap:			Unmap an area of IOVA (optional but
// must be implemented with dma_map)
// Needed for device that using device
// specific DMA translation (on-chip IOMMU)
// and preferring incremental unmap.
// @vdev: vdpa device
// @asid: address space identifier
// @iova: iova to be unmapped
// @size: size of the area
// Returns integer: success (0) or error (< 0)
// @reset_map:			Reset device memory mapping to the default
// state (optional)
// Needed for devices that are using device
// specific DMA translation and prefer mapping
// to be decoupled from the virtio life cycle,
// i.e. device .reset op does not reset mapping
// @vdev: vdpa device
// @asid: address space identifier
// Returns integer: success (0) or error (< 0)
// @get_vq_map:		Get the map metadata for a specific
// virtqueue (optional)
// @vdev: vdpa device
// @idx: virtqueue index
// Returns map token union error (NULL)
// @bind_mm:			Bind the device to a specific address space
// so the vDPA framework can use VA when this
// callback is implemented. (optional)
// @vdev: vdpa device
// @mm: address space to bind
// @unbind_mm:			Unbind the device from the address space
// bound using the bind_mm callback. (optional)
// @vdev: vdpa device
// @free:			Free resources that belongs to vDPA (optional)
// @vdev: vdpa device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_config_ops {
// Virtqueue ops
    pub device_area): u64,
    pub num): *mut *mut *mut void (set_vq_num)(struct vdpa_device vdev, u16 idx, u32,
    pub idx): *mut *mut *mut void (kick_vq)(struct vdpa_device vdev, u16,
    pub data): *mut *mut *mut void (kick_vq_with_data)(struct vdpa_device vdev, u32,
    pub cb): *mut vdpa_callback,
    pub ready): *mut *mut *mut void (set_vq_ready)(struct vdpa_device vdev, u16 idx, bool,
    pub idx): *mut *mut *mut bool (get_vq_ready)(struct vdpa_device vdev, u16,
    pub state): *const vdpa_vq_state,
    pub state): *mut vdpa_vq_state,
    pub extack): *mut netlink_ext_ack,
    pub idx): *mut *mut *mut (get_vq_notification)(struct vdpa_device vdev, u16,
// vq irq is not expected to be changed once DRIVER_OK is set
    pub idx): *mut *mut *mut int (get_vq_irq)(struct vdpa_device vdev, u16,
    pub idx): *mut *mut *mut u16 (get_vq_size)(struct vdpa_device vdev, u16,
// Device ops
    pub vdev): *mut *mut u32 (get_vq_align)(struct vdpa_device,
    pub idx): *mut *mut *mut u32 (get_vq_group)(struct vdpa_device vdev, u16,
    pub idx): *mut *mut *mut u32 (get_vq_desc_group)(struct vdpa_device vdev, u16,
    pub vdev): *mut *mut u64 (get_device_features)(struct vdpa_device,
    pub vdev): *const *const u64 (get_backend_features)(struct vdpa_device,
    pub features): *mut *mut *mut int (set_driver_features)(struct vdpa_device vdev, u64,
    pub vdev): *mut *mut u64 (get_driver_features)(struct vdpa_device,
    pub cb): *mut vdpa_callback,
    pub vdev): *mut *mut u16 (get_vq_num_max)(struct vdpa_device,
    pub vdev): *mut *mut u16 (get_vq_num_min)(struct vdpa_device,
    pub vdev): *mut *mut u32 (get_device_id)(struct vdpa_device,
    pub vdev): *mut *mut u32 (get_vendor_id)(struct vdpa_device,
    pub vdev): *mut *mut u8 (get_status)(struct vdpa_device,
    pub status): *mut *mut *mut void (set_status)(struct vdpa_device vdev, u8,
    pub vdev): *mut *mut int (reset)(struct vdpa_device,
    pub flags): *mut *mut *mut int (compat_reset)(struct vdpa_device vdev, u32,
pub const VDPA_RESET_F_CLEAN_MAP: c_int = 1;
    pub vdev): *mut *mut int (suspend)(struct vdpa_device,
    pub vdev): *mut *mut int (resume)(struct vdpa_device,
    pub vdev): *mut *mut size_t (get_config_size)(struct vdpa_device,
    pub len): *mut *mut void buf, unsigned int,
    pub len): *const *const void buf, unsigned int,
    pub vdev): *mut *mut u32 (get_generation)(struct vdpa_device,
    pub vdev): *mut *mut vdpa_iova_range (get_iova_range)(vdpa_device,
    pub cpu_mask): *const cpumask,
    pub idx): u16,
// DMA ops
    pub iotlb): *mut vhost_iotlb,
    pub opaque): *mut u64 iova, u64 size, u64 pa, u32 perm, void,
    pub size): u64 iova, u64,
    pub asid): *mut *mut *mut int (reset_map)(struct vdpa_device vdev, unsigned int,
    pub asid): c_uint,
    pub idx): *mut *mut *mut virtio_map (get_vq_map)(struct vdpa_device vdev, u16,
    pub mm): *mut *mut *mut int (bind_mm)(struct vdpa_device vdev, struct mm_struct,
    pub vdev): *mut *mut void (unbind_mm)(struct vdpa_device,
// Free device resources
    pub vdev): *mut *mut void (free)(struct vdpa_device,
}

//
// vdpa_alloc_device - allocate and initilaize a vDPA device
//
// @dev_struct: the type of the parent structure
// @member: the name of struct vdpa_device within the @dev_struct
// @parent: the parent device
// @config: the bus operations that is supported by this device
// @map: the map operations that is supported by this device
// @ngroups: the number of virtqueue groups supported by this device
// @nas: the number of address spaces
// @name: name of the vdpa device
// @use_va: indicate whether virtual address must be used by this device
//
// Return allocated data structure or ERR_PTR upon error
//

extern "C" {
    pub fn vdpa_register_device(vdev: *mut vdpa_device, nvqs: u32) -> c_int;
}
extern "C" {
    pub fn vdpa_unregister_device(vdev: *mut vdpa_device);
}
extern "C" {
    pub fn _vdpa_register_device(vdev: *mut vdpa_device, nvqs: u32) -> c_int;
}
extern "C" {
    pub fn _vdpa_unregister_device(vdev: *mut vdpa_device);
}
//
// struct vdpa_driver - operations for a vDPA driver
// @driver: underlying device driver
// @probe: the function to call when a device is found.  Returns 0 or -errno.
// @remove: the function to call when a device is removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_driver {
    pub driver: device_driver,
    pub vdev): *mut *mut int (probe)(struct vdpa_device,
    pub vdev): *mut *mut void (remove)(struct vdpa_device,
}

extern "C" {
    pub fn __vdpa_register_driver(drv: *mut vdpa_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn vdpa_unregister_driver(drv: *mut vdpa_driver);
}

extern "C" {
    pub fn container_of(_arg: driver, vdpa_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn container_of(_arg: _dev, vdpa_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &vdev->dev) -> return;
}
extern "C" {
    pub fn vdpa_set_status(vdev: *mut vdpa_device, status: u8);
}
//
// struct vdpa_mgmtdev_ops - vdpa device ops
// @dev_add: Add a vdpa device using alloc and register
// @mdev: parent device to use for device addition
// @name: name of the new vdpa device
// @config: config attributes to apply to the device under creation
// Driver need to add a new device using _vdpa_register_device()
// after fully initializing the vdpa device. Driver must return 0
// on success or appropriate error code.
// @dev_del: Remove a vdpa device using unregister
// @mdev: parent device to use for device removal
// @dev: vdpa device to remove
// Driver need to remove the specified device by calling
// _vdpa_unregister_device().
// @dev_set_attr: change a vdpa device's attr after it was create
// @mdev: parent device to use for device
// @dev: vdpa device structure
// @config:Attributes to be set for the device.
// The driver needs to check the mask of the structure and then set
// the related information to the vdpa device. The driver must return 0
// if set successfully.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_mgmtdev_ops {
    pub config): *const vdpa_dev_set_config,
    pub dev): *mut *mut *mut void (dev_del)(struct vdpa_mgmt_dev mdev, struct vdpa_device,
    pub config): *const vdpa_dev_set_config,
}

//
// struct vdpa_mgmt_dev - vdpa management device
// @device: Management parent device
// @ops: operations supported by management device
// @id_table: Pointer to device id table of supported ids
// @config_attr_mask: bit mask of attributes of type enum vdpa_attr that
// management device support during dev_add callback
// @list: list entry
// @supported_features: features supported by device
// @max_supported_vqs: maximum number of virtqueues supported by device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpa_mgmt_dev {
    pub device: *mut device,
    pub ops: *const vdpa_mgmtdev_ops,
    pub id_table: *mut virtio_device_id,
    pub config_attr_mask: u64,
    pub list: list_head,
    pub supported_features: u64,
    pub max_supported_vqs: u32,
}

extern "C" {
    pub fn vdpa_mgmtdev_register(mdev: *mut vdpa_mgmt_dev) -> c_int;
}
extern "C" {
    pub fn vdpa_mgmtdev_unregister(mdev: *mut vdpa_mgmt_dev);
}
