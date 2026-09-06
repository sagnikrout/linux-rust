//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uacce.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

pub const UACCE_MAX_REGION: c_int = 2;
pub const UACCE_MAX_NAME_SIZE: c_int = 64;
pub const UACCE_MAX_ERR_THRESHOLD: c_int = 65535;
//
// struct uacce_qfile_region - structure of queue file region
// @type: type of the region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uacce_qfile_region {
    pub type: uacce_qfrt,
}

//
// struct uacce_ops - uacce device operations
// @get_available_instances:  get available instances left of the device
// @get_queue: get a queue from the device
// @put_queue: free a queue to the device
// @start_queue: make the queue start work after get_queue
// @stop_queue: make the queue stop work before put_queue
// @is_q_updated: check whether the task is finished
// @mmap: mmap addresses of queue to user space
// @ioctl: ioctl for user space users of the queue
// @get_isolate_state: get the device state after set the isolate strategy
// @isolate_err_threshold_write: stored the isolate error threshold to the device
// @isolate_err_threshold_read: read the isolate error threshold value from the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uacce_ops {
    pub uacce): *mut *mut int (get_available_instances)(struct uacce_device,
    pub q): *mut uacce_queue,
    pub q): *mut *mut void (put_queue)(struct uacce_queue,
    pub q): *mut *mut int (start_queue)(struct uacce_queue,
    pub q): *mut *mut void (stop_queue)(struct uacce_queue,
    pub q): *mut *mut int (is_q_updated)(struct uacce_queue,
    pub qfr): *mut uacce_qfile_region,
    pub arg): c_ulong,
    pub uacce): *mut *mut uacce_dev_state (get_isolate_state)(struct uacce_device,
    pub num): *mut *mut *mut int (isolate_err_threshold_write)(struct uacce_device uacce, u32,
    pub uacce): *mut *mut u32 (isolate_err_threshold_read)(struct uacce_device,
}

//
// struct uacce_interface - interface required for uacce_register()
// @name: the uacce device name.  Will show up in sysfs
// @flags: uacce device attributes
// @ops: pointer to the struct uacce_ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uacce_interface {
    pub name: [c_char; UACCE_MAX_NAME_SIZE],
    pub flags: c_uint,
    pub ops: *const uacce_ops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uacce_dev_state {
    UACCE_DEV_NORMAL,
    UACCE_DEV_ISOLATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uacce_q_state {
    UACCE_Q_ZOMBIE = 0,
    UACCE_Q_INIT,
    UACCE_Q_STARTED,
}

//
// struct uacce_queue
// @uacce: pointer to uacce
// @priv: private pointer
// @wait: wait queue head
// @list: index into uacce queues list
// @qfrs: pointer of qfr regions
// @mutex: protects queue state
// @state: queue state machine
// @pasid: pasid associated to the mm
// @handle: iommu_sva handle returned by iommu_sva_bind_device()
// @mapping: user space mapping of the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uacce_queue {
    pub uacce: *mut uacce_device,
    pub priv: *mut c_void,
    pub wait: wait_queue_head_t,
    pub list: list_head,
    pub qfrs: [*mut uacce_qfile_region; UACCE_MAX_REGION],
    pub mutex: mutex,
    pub state: uacce_q_state,
    pub pasid: u32,
    pub handle: *mut iommu_sva,
    pub mapping: *mut address_space,
}

//
// struct uacce_device
// @algs: supported algorithms
// @api_ver: api version
// @ops: pointer to the struct uacce_ops
// @qf_pg_num: page numbers of the queue file regions
// @parent: pointer to the parent device
// @is_vf: whether virtual function
// @flags: uacce attributes
// @dev_id: id of the uacce device
// @cdev: cdev of the uacce
// @dev: dev of the uacce
// @mutex: protects uacce operation
// @priv: private pointer of the uacce
// @queues: list of queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uacce_device {
    pub algs: *const c_char,
    pub api_ver: *const c_char,
    pub ops: *const uacce_ops,
    pub qf_pg_num: [c_ulong; UACCE_MAX_REGION],
    pub parent: *mut device,
    pub is_vf: bool,
    pub flags: u32,
    pub dev_id: u32,
    pub cdev: *mut cdev,
    pub dev: device,
    pub mutex: mutex,
    pub priv: *mut c_void,
    pub queues: list_head,
}

extern "C" {
    pub fn uacce_register(uacce: *mut uacce_device) -> c_int;
}
extern "C" {
    pub fn uacce_remove(uacce: *mut uacce_device);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

