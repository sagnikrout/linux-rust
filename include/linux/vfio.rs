//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vfio.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// VFIO API definition
//
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//

//
// VFIO devices can be placed in a set, this allows all devices to share this
// structure and the VFIO core will provide a lock that is held around
// open_device()/close_device() for all devices in the set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_set {
    pub set_id: *mut c_void,
    pub lock: mutex,
    pub device_list: list_head,
    pub device_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device {
    pub dev: *mut device,
    pub ops: *const vfio_device_ops,
//
// mig_ops/log_ops is a static property of the vfio_device which must
// be set prior to registering the vfio_device.
//
    pub mig_ops: *const vfio_migration_ops,
    pub log_ops: *const vfio_log_ops,

    pub group: *mut vfio_group,
    pub group_next: list_head,
    pub iommu_entry: list_head,

    pub dev_set: *mut vfio_device_set,
    pub dev_set_list: list_head,
    pub migration_flags: c_uint,
    pub precopy_info_v2: u8,
    pub kvm: *mut kvm,
// Members below here are private, not for driver use
    pub index: c_uint,
    pub /: *mut *mut device device; / device.kref covers object life circle,

    pub cdev: cdev,

    pub device*/: *mut *mut refcount_t refcount; / user count on registered,
    pub open_count: c_uint,
    pub comp: completion,
    pub iommufd_access: *mut iommufd_access,
    pub kvm): *mut *mut void (put_kvm)(struct kvm,
    pub inode: *mut inode,

    pub iommufd_device: *mut iommufd_device,
    pub pasids: ida,
    pub iommufd_attached:1: u8,

    pub cdev_opened:1: u8,
    pub noiommu:1: u8,
//
// debug_root is a static property of the vfio_device
// which must be set prior to registering the vfio_device.
//
    pub debug_root: *mut dentry,
}

//
// struct vfio_device_ops - VFIO bus driver device callbacks
//
// @name: Name of the device driver.
// @init: initialize private fields in device structure
// @release: Reclaim private fields in device structure
// @bind_iommufd: Called when binding the device to an iommufd
// @unbind_iommufd: Opposite of bind_iommufd
// @attach_ioas: Called when attaching device to an IOAS/HWPT managed by the
// bound iommufd. Undo in unbind_iommufd if @detach_ioas is not
// called.
// @detach_ioas: Opposite of attach_ioas
// @pasid_attach_ioas: The pasid variation of attach_ioas
// @pasid_detach_ioas: Opposite of pasid_attach_ioas
// @open_device: Called when the first file descriptor is opened for this device
// @close_device: Opposite of open_device
// @read: Perform read(2) on device file descriptor
// @write: Perform write(2) on device file descriptor
// @ioctl: Perform ioctl(2) on device file descriptor, supporting VFIO_DEVICE_
// operations documented below
// @mmap: Perform mmap(2) on a region of the device file descriptor
// @request: Request for the bus driver to release the device
// @match: Optional device name match callback (return: 0 for no-match, >0 for
// match, -errno for abort (ex. match with insufficient or incorrect
// additional args)
// @match_token_uuid: Optional device token match/validation. Return 0
// if the uuid is valid for the device, -errno otherwise. uuid is NULL
// if none was provided.
// @dma_unmap: Called when userspace unmaps IOVA from the container
// this device is attached to.
// @device_feature: Optional, fill in the VFIO_DEVICE_FEATURE ioctl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_ops {
    pub name: *mut c_char,
    pub vdev): *mut *mut int (init)(struct vfio_device,
    pub vdev): *mut *mut void (release)(struct vfio_device,
    pub out_device_id): *mut *mut iommufd_ctx ictx, u32,
    pub vdev): *mut *mut void (unbind_iommufd)(struct vfio_device,
    pub pt_id): *mut *mut *mut int (attach_ioas)(struct vfio_device vdev, u32,
    pub vdev): *mut *mut void (detach_ioas)(struct vfio_device,
    pub pt_id): *mut u32,
    pub pasid): *mut *mut *mut void (pasid_detach_ioas)(struct vfio_device vdev, u32,
    pub vdev): *mut *mut int (open_device)(struct vfio_device,
    pub vdev): *mut *mut void (close_device)(struct vfio_device,
    pub ppos): *mut size_t count, loff_t,
    pub size): *mut size_t count, loff_t,
    pub arg): c_ulong,
    pub caps): *mut vfio_info_cap,
    pub vma): *mut *mut *mut int (mmap)(struct vfio_device vdev, struct vm_area_struct,
    pub count): *mut *mut *mut void (request)(struct vfio_device vdev, unsigned int,
    pub buf): *mut *mut *mut int (match)(struct vfio_device vdev, char,
    pub uuid): *const *const *const int (match_token_uuid)(struct vfio_device vdev, uuid_t,
    pub length): *mut *mut *mut void (dma_unmap)(struct vfio_device vdev, u64 iova, u64,
    pub argsz): *mut *mut void __user arg, size_t,
}

extern "C" {
    pub fn vfio_iommufd_get_dev_id(vdev: *mut vfio_device, ictx: *mut iommufd_ctx) -> c_int;
}
extern "C" {
    pub fn vfio_iommufd_physical_unbind(vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_iommufd_physical_attach_ioas(vdev: *mut vfio_device, pt_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn vfio_iommufd_physical_detach_ioas(vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_iommufd_emulated_unbind(vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_iommufd_emulated_attach_ioas(vdev: *mut vfio_device, pt_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn vfio_iommufd_emulated_detach_ioas(vdev: *mut vfio_device);
}

//
// struct vfio_migration_ops - VFIO bus device driver migration callbacks
//
// @migration_set_state: Optional callback to change the migration state for
// devices that support migration. It's mandatory for
// VFIO_DEVICE_FEATURE_MIGRATION migration support.
// The returned FD is used for data transfer according to the FSM
// definition. The driver is responsible to ensure that FD reaches end
// of stream or error whenever the migration FSM leaves a data transfer
// state or before close_device() returns.
// @migration_get_state: Optional callback to get the migration state for
// devices that support migration. It's mandatory for
// VFIO_DEVICE_FEATURE_MIGRATION migration support.
// @migration_get_data_size: Optional callback to get the estimated data
// length that will be required to complete stop copy. It's mandatory for
// VFIO_DEVICE_FEATURE_MIGRATION migration support.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_migration_ops {
    pub new_state): vfio_device_mig_state,
    pub curr_state): *mut vfio_device_mig_state,
    pub stop_copy_length): *mut c_ulong,
}

//
// struct vfio_log_ops - VFIO bus device driver logging callbacks
//
// @log_start: Optional callback to ask the device start DMA logging.
// @log_stop: Optional callback to ask the device stop DMA logging.
// @log_read_and_clear: Optional callback to ask the device read
// and clear the dirty DMAs in some given range.
//
// The vfio core implementation of the DEVICE_FEATURE_DMA_LOGGING_ set
// of features does not track logging state relative to the device,
// therefore the device implementation of vfio_log_ops must handle
// arbitrary user requests. This includes rejecting subsequent calls
// to log_start without an intervening log_stop, as well as graceful
// handling of log_stop and log_read_and_clear from invalid states.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_log_ops {
    pub page_size): *mut *mut rb_root_cached ranges, u32 nnodes, u64,
    pub device): *mut *mut int (log_stop)(struct vfio_device,
    pub dirty): *mut iova_bitmap,
}

//
// vfio_check_feature - Validate user input for the VFIO_DEVICE_FEATURE ioctl
// @flags: Arg from the device_feature op
// @argsz: Arg from the device_feature op
// @supported_ops: Combination of VFIO_DEVICE_FEATURE_GET and SET the driver
// supports
// @minsz: Minimum data size the driver accepts
//
// For use in a driver's device_feature op. Checks that the inputs to the
// VFIO_DEVICE_FEATURE ioctl are correct for the driver's feature. Returns 1 if
// the driver should execute the get or set, otherwise the relevant
// value should be returned.
//
// Without PROBE one of GET or SET must be requested
//
// vfio_check_precopy_ioctl - Validate user input for the VFIO_MIG_GET_PRECOPY_INFO ioctl
// @vdev: The vfio device
// @cmd: Cmd from the ioctl
// @arg: Arg from the ioctl
// @info: Driver pointer to hold the userspace input to the ioctl
//
// For use in a driver's get_precopy_info. Checks that the inputs to the
// VFIO_MIG_GET_PRECOPY_INFO ioctl are correct.
// Returns 0 on success, otherwise errno.
//
// keep v1 behaviour as is for compatibility reasons
// flags are output, set its initial value to 0

extern "C" {
    pub fn vfio_register_group_dev(device: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_register_emulated_iommu_dev(device: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_unregister_group_dev(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_try_get_registration(device: *mut vfio_device) -> bool;
}
extern "C" {
    pub fn vfio_device_put_registration(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_assign_device_set(device: *mut vfio_device, set_id: *mut c_void) -> c_int;
}
extern "C" {
    pub fn vfio_device_set_open_count(dev_set: *mut vfio_device_set) -> c_uint;
}
//
// External user API
//

extern "C" {
    pub fn vfio_file_is_group(file: *mut file) -> bool;
}
extern "C" {
    pub fn vfio_file_has_dev(file: *mut file, device: *mut vfio_device) -> bool;
}

extern "C" {
    pub fn vfio_file_is_valid(file: *mut file) -> bool;
}
extern "C" {
    pub fn vfio_file_enforced_coherent(file: *mut file) -> bool;
}
extern "C" {
    pub fn vfio_file_set_kvm(file: *mut file, kvm: *mut kvm);
}

extern "C" {
    pub fn vfio_unpin_pages(device: *mut vfio_device, iova: dma_addr_t, npage: c_int);
}
//
// Sub-module helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_info_cap {
    pub buf: *mut vfio_info_cap_header,
    pub size: usize,
}

extern "C" {
    pub fn vfio_info_cap_shift(caps: *mut vfio_info_cap, offset: usize);
}
//
// IRQfd - generic
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virqfd {
    pub opaque: *mut c_void,
    pub eventfd: *mut eventfd_ctx,
    pub ): *mut *mut *mut int (handler)(void , void,
    pub ): *mut *mut *mut void (thread)(void , void,
    pub data: *mut c_void,
    pub inject: work_struct,
    pub wait: wait_queue_entry_t,
    pub pt: poll_table,
    pub shutdown: work_struct,
    pub flush_inject: work_struct,
    pub pvirqfd: *mut virqfd,
}

extern "C" {
    pub fn vfio_virqfd_disable(pvirqfd: *mut virqfd);
}
extern "C" {
    pub fn vfio_virqfd_flush_thread(pvirqfd: *mut virqfd);
}
