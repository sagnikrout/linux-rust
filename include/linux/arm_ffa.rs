//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/arm_ffa.h
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
// Copyright (C) 2021 ARM Ltd.
//

//
// For some calls it is necessary to use SMC64 to pass or return 64-bit values.
// For such calls FFA_FN_NATIVE(name) will choose the appropriate
// (native-width) function ID.
//

// FFA error codes.

// FFA version encoding

//
// FF-A specification mentions explicitly about '4K pages'. This should
// not be confused with the kernel PAGE_SIZE, which is the translation
// granule kernel is configured and may be one among 4K, 16K and 64K.
//

//
// Minimum buffer size/alignment encodings returned by an FFA_FEATURES
// query for FFA_RXTX_MAP.
//
pub const FFA_FEAT_RXTX_MIN_SZ_4K: c_int = 0;
pub const FFA_FEAT_RXTX_MIN_SZ_64K: c_int = 1;
pub const FFA_FEAT_RXTX_MIN_SZ_16K: c_int = 2;

// FFA Bus/Device/Driver related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_device {
    pub id: u32,
    pub properties: u32,
    pub vm_id: c_int,
    pub mode_32bit: bool,
    pub uuid: uuid_t,
    pub dev: device,
    pub ops: *const ffa_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_device_id {
    pub uuid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_driver {
    pub name: *const c_char,
    pub sdev): *mut *mut int (probe)(struct ffa_device,
    pub sdev): *mut *mut void (remove)(struct ffa_device,
    pub id_table: *const ffa_device_id,
    pub driver: device_driver,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &fdev->dev) -> return;
}

extern "C" {
    pub fn ffa_device_unregister(ffa_dev: *mut ffa_device);
}
extern "C" {
    pub fn ffa_driver_unregister(driver: *mut ffa_driver);
}
extern "C" {
    pub fn ffa_devices_unregister();
}
extern "C" {
    pub fn ffa_device_is_valid(ffa_dev: *mut ffa_device) -> bool;
}

//
// module_ffa_driver() - Helper macro for registering a psa_ffa driver
// @__ffa_driver: ffa_driver structure
//
// Helper macro for psa_ffa drivers to set up proper module init / exit
// functions.  Replaces module_init() and module_exit() and keeps people from
// printing pointless things to the kernel log when their driver is loaded.
//

// The FF-A 1.0 partition structure lacks the uuid[4]

// FFA transport related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_partition_info {
    pub id: u16,
    pub exec_ctxt: u16,
// partition supports receipt of direct requests

// partition can send direct requests.

// partition can send and receive indirect messages.

// partition can receive notifications

// partition runs in the AArch64 execution state.

// partition supports receipt of direct request2

// partition can send direct request2.

    pub properties: u32,
    pub uuid: uuid_t,
}

// For use with FFA_MSG_SEND_DIRECT_{REQ,RESP} which pass data via registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_send_direct_data {
    pub /: *mut *mut unsigned long data0; / w3/x3,
    pub /: *mut *mut unsigned long data1; / w4/x4,
    pub /: *mut *mut unsigned long data2; / w5/x5,
    pub /: *mut *mut unsigned long data3; / w6/x6,
    pub /: *mut *mut unsigned long data4; / w7/x7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_indirect_msg_hdr {
    pub flags: u32,
    pub res0: u32,
    pub offset: u32,
    pub send_recv_id: u32,
    pub size: u32,
    pub res1: u32,
    pub uuid: uuid_t,
}

// For use with FFA_MSG_SEND_DIRECT_{REQ,RESP}2 which pass data via registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_send_direct_data2 {
    pub /: *mut *mut unsigned long data[14]; / x4-x17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_mem_region_addr_range {
// The base IPA of the constituent memory region, aligned to 4 kiB
    pub address: u64,
// The number of 4 kiB pages in the constituent memory region.
    pub pg_cnt: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_composite_mem_region {
//
// The total number of 4 kiB pages included in this memory region. This
// must be equal to the sum of page counts specified in each
// `struct ffa_mem_region_addr_range`.
//
    pub total_pg_cnt: u32,
// The number of constituents included in this memory region range
    pub addr_range_cnt: u32,
    pub reserved: u64,
// An array of `addr_range_cnt` memory region constituents.
    pub constituents: [ffa_mem_region_addr_range; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_mem_region_attributes {
// The ID of the VM to which the memory is being given or shared.
    pub receiver: u16,
//
// The permissions with which the memory region should be mapped in the
// receiver's page table.
//

    pub attrs: u8,
//
// Flags used during FFA_MEM_RETRIEVE_REQ and FFA_MEM_RETRIEVE_RESP
// for memory regions with multiple borrowers.
//

    pub flag: u8,
//
// Offset in bytes from the start of the outer `ffa_memory_region` to
// an `struct ffa_mem_region_addr_range`.
//
    pub composite_off: u32,
    pub impdef_val: [u8; 16],
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_mem_region {
// The ID of the VM/owner which originally sent the memory region
    pub sender_id: u16,

// Memory region attributes, upper byte MBZ pre v1.1
    pub attributes: u16,
//
// Clear memory region contents after unmapping it from the sender and
// before mapping it for any receiver.
//

//
// Whether the hypervisor may time slice the memory sharing or retrieval
// operation.
//

// Flags to control behaviour of the transaction.
    pub flags: u32,

//
// A globally-unique ID assigned by the hypervisor for a region
// of memory being sent between VMs.
//
    pub handle: u64,
//
// An implementation defined value associated with the receiver and the
// memory region.
//
    pub tag: u64,
// Size of each endpoint memory access descriptor, MBZ pre v1.1
    pub ep_mem_size: u32,
//
// The number of `ffa_mem_region_attributes` entries included in this
// transaction.
//
    pub ep_count: u32,
//
// 16-byte aligned offset from the base address of this descriptor
// to the first element of the endpoint memory access descriptor array
// Valid only from v1.1
//
    pub ep_mem_offset: u32,
// MBZ, valid only from v1.1
    pub reserved: [u32; 3],
}

// The layout changed from FFA_VERSION_1_0 and the region includes an
// ep_mem_offset.
//

//
// Earlier to v1.1, the endpoint memory descriptor array started at
// offset 32(i.e. offset of ep_mem_offset in the current structure)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_mem_ops_args {
    pub use_txbuf: bool,
    pub nattrs: u32,
    pub flags: u32,
    pub tag: u64,
    pub g_handle: u64,
    pub sg: *mut scatterlist,
    pub attrs: *mut ffa_mem_region_attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_info_ops {
    pub (*api_version_get)(void): *mut u32,
    pub buffer): *mut ffa_partition_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_msg_ops {
    pub dev): *mut *mut void (mode_32bit_set)(struct ffa_device,
    pub data): *mut ffa_send_direct_data,
    pub sz): *mut *mut *mut *mut int (indirect_send)(struct ffa_device dev, void buf, size_t,
    pub data): *mut ffa_send_direct_data2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_mem_ops {
    pub flags): *mut *mut int (memory_reclaim)(u64 g_handle, u32,
    pub args): *mut *mut int (memory_share)(struct ffa_mem_ops_args,
    pub args): *mut *mut int (memory_lend)(struct ffa_mem_ops_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_cpu_ops {
    pub vcpu): *mut *mut *mut int (run)(struct ffa_device dev, u16,
}

extern "C" {
    pub fn void(vcpu: *mut *mut ffa_sched_recv_cb)(u16, is_per_vcpu: bool, cb_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(notify_id: *mut *mut ffa_notifier_cb)(int, cb_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(notify_id: *mut *mut ffa_fwk_notifier_cb)(int, cb_data: *mut c_void, buf: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_notifier_ops {
    pub cb_data): *mut ffa_sched_recv_cb cb, void,
    pub dev): *mut *mut int (sched_recv_cb_unregister)(struct ffa_device,
    pub notify_id): *mut *mut ffa_notifier_cb cb, void cb_data, int,
    pub notify_id): *mut *mut *mut int (notify_relinquish)(struct ffa_device dev, int,
    pub notify_id): c_int,
    pub notify_id): *mut *mut *mut int (fwk_notify_relinquish)(struct ffa_device dev, int,
    pub vcpu): u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffa_ops {
    pub info_ops: *const ffa_info_ops,
    pub msg_ops: *const ffa_msg_ops,
    pub mem_ops: *const ffa_mem_ops,
    pub cpu_ops: *const ffa_cpu_ops,
    pub notifier_ops: *const ffa_notifier_ops,
}
