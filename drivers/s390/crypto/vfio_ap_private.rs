//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/vfio_ap_private.h
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
// Private data and functions for adjunct processor VFIO matrix driver.
//
// Author(s): Tony Krowiak <akrowiak@linux.ibm.com>
// Halil Pasic <pasic@linux.ibm.com>
// Pierre Morel <pmorel@linux.ibm.com>
//
// Copyright IBM Corp. 2018
//

//
// struct ap_matrix_dev - Contains the data for the matrix device.
//
// @device:	generic device structure associated with the AP matrix device
// @info:	the struct containing the output from the PQAP(QCI) instruction
// @mdev_list:	the list of mediated matrix devices created
// @mdevs_lock: mutex for locking the AP matrix device. This lock will be
// taken every time we fiddle with state managed by the vfio_ap
// driver, be it using @mdev_list or writing the state of a
// single ap_matrix_mdev device. It's quite coarse but we don't
// expect much contention.
// @vfio_ap_drv: the vfio_ap device driver
// @guests_lock: mutex for controlling access to a guest that is using AP
// devices passed through by the vfio_ap device driver. This lock
// will be taken when the AP devices are plugged into or unplugged
// from a guest, and when an ap_matrix_mdev device is added to or
// removed from @mdev_list or the list is iterated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_matrix_dev {
    pub device: device,
    pub info: ap_config_info,
    pub mdev_list: list_head,
    pub /: *mut *mut mutex mdevs_lock; / serializes access to each ap_matrix_mdev,
    pub vfio_ap_drv: *mut ap_driver,
    pub /: *mut *mut mutex guests_lock; / serializes access to each KVM guest,
    pub parent: mdev_parent,
    pub mdev_type: mdev_type,
    pub mdev_types: *mut mdev_type,
}

//
// struct ap_matrix - matrix of adapters, domains and control domains
//
// @apm_max: max adapter number in @apm
// @apm: identifies the AP adapters in the matrix
// @aqm_max: max domain number in @aqm
// @aqm: identifies the AP queues (domains) in the matrix
// @adm_max: max domain number in @adm
// @adm: identifies the AP control domains in the matrix
//
// The AP matrix is comprised of three bit masks identifying the adapters,
// queues (domains) and control domains that belong to an AP matrix. The bits in
// each mask, from left to right, correspond to IDs 0 to 255. When a bit is set
// the corresponding ID belongs to the matrix.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_matrix {
    pub apm_max: c_ulong,
    pub AP_DEVICES): DECLARE_BITMAP(apm,,
    pub aqm_max: c_ulong,
    pub AP_DOMAINS): DECLARE_BITMAP(aqm,,
    pub adm_max: c_ulong,
    pub AP_DOMAINS): DECLARE_BITMAP(adm,,
}

//
// struct ap_queue_table - a table of queue objects.
//
// @queues: a hashtable of queues (struct vfio_ap_queue).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_queue_table {
    pub 8): DECLARE_HASHTABLE(queues,,
}

//
// struct ap_matrix_mdev - Contains the data associated with a matrix mediated
// device.
// @vdev:	the vfio device
// @node:	allows the ap_matrix_mdev struct to be added to a list
// @matrix:	the adapters, usage domains and control domains assigned to the
// mediated matrix device.
// @shadow_apcb:    the shadow copy of the APCB field of the KVM guest's CRYCB
// @kvm:	the struct holding guest's state
// @pqap_hook:	the function pointer to the interception handler for the
// PQAP(AQIC) instruction.
// @mdev:	the mediated device
// @qtable:	table of queues (struct vfio_ap_queue) assigned to the mdev
// @req_trigger eventfd ctx for signaling userspace to return a device
// @cfg_chg_trigger eventfd ctx to signal AP config changed to userspace
// @apm_add:	bitmap of APIDs added to the host's AP configuration
// @aqm_add:	bitmap of APQIs added to the host's AP configuration
// @adm_add:	bitmap of control domain numbers added to the host's AP
// configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_matrix_mdev {
    pub vdev: vfio_device,
    pub node: list_head,
    pub matrix: ap_matrix,
    pub shadow_apcb: ap_matrix,
    pub kvm: *mut kvm,
    pub pqap_hook: crypto_hook,
    pub mdev: *mut mdev_device,
    pub qtable: ap_queue_table,
    pub req_trigger: *mut eventfd_ctx,
    pub cfg_chg_trigger: *mut eventfd_ctx,
    pub AP_DEVICES): DECLARE_BITMAP(apm_add,,
    pub AP_DOMAINS): DECLARE_BITMAP(aqm_add,,
    pub AP_DOMAINS): DECLARE_BITMAP(adm_add,,
}

//
// struct vfio_ap_queue - contains the data associated with a queue bound to the
// vfio_ap device driver
// @matrix_mdev: the matrix mediated device
// @saved_iova: the notification indicator byte (nib) address
// @apqn: the APQN of the AP queue device
// @saved_isc: the guest ISC registered with the GIB interface
// @mdev_qnode: allows the vfio_ap_queue struct to be added to a hashtable
// @reset_qnode: allows the vfio_ap_queue struct to be added to a list of queues
// that need to be reset
// @reset_status: the status from the last reset of the queue
// @reset_work: work to wait for queue reset to complete
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_ap_queue {
    pub matrix_mdev: *mut ap_matrix_mdev,
    pub saved_iova: dma_addr_t,
    pub apqn: c_int,
pub const VFIO_AP_ISC_INVALID: c_uint = 0xff;
    pub saved_isc: c_uchar,
    pub mdev_qnode: hlist_node,
    pub reset_qnode: list_head,
    pub reset_status: ap_queue_status,
    pub reset_work: work_struct,
}

extern "C" {
    pub fn vfio_ap_mdev_register() -> c_int;
}
extern "C" {
    pub fn vfio_ap_mdev_unregister();
}
extern "C" {
    pub fn vfio_ap_mdev_probe_queue(queue: *mut ap_device) -> c_int;
}
extern "C" {
    pub fn vfio_ap_mdev_remove_queue(queue: *mut ap_device);
}
extern "C" {
    pub fn vfio_ap_mdev_resource_in_use(apm: *mut c_ulong, aqm: *mut c_ulong) -> c_int;
}
