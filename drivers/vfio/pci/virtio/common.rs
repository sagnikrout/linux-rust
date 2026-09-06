//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/virtio/common.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtiovf_migf_state {
    VIRTIOVF_MIGF_STATE_ERROR = 1,
    VIRTIOVF_MIGF_STATE_PRECOPY = 2,
    VIRTIOVF_MIGF_STATE_COMPLETE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtiovf_load_state {
    VIRTIOVF_LOAD_STATE_READ_HEADER,
    VIRTIOVF_LOAD_STATE_PREP_HEADER_DATA,
    VIRTIOVF_LOAD_STATE_READ_HEADER_DATA,
    VIRTIOVF_LOAD_STATE_PREP_CHUNK,
    VIRTIOVF_LOAD_STATE_READ_CHUNK,
    VIRTIOVF_LOAD_STATE_LOAD_CHUNK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtiovf_data_buffer {
    pub table: sg_append_table,
    pub start_pos: loff_t,
    pub length: u64,
    pub allocated_length: u64,
    pub buf_elm: list_head,
    pub include_header_object:1: u8,
    pub migf: *mut virtiovf_migration_file,
// Optimize virtiovf_get_migration_page() for sequential access
    pub last_offset_sg: *mut scatterlist,
    pub sg_last_entry: c_uint,
    pub last_offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtiovf_migf_header_flags {
    VIRTIOVF_MIGF_HEADER_FLAGS_TAG_MANDATORY = 0,
    VIRTIOVF_MIGF_HEADER_FLAGS_TAG_OPTIONAL = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtiovf_migf_header_tag {
    VIRTIOVF_MIGF_HEADER_TAG_DEVICE_DATA = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtiovf_migration_header {
    pub record_size: __le64,
// For future use in case we may need to change the kernel protocol
    pub /: *mut *mut __le32 flags; / Use virtiovf_migf_header_flags,
    pub /: *mut *mut __le32 tag; / Use virtiovf_migf_header_tag,
    pub /: *mut *mut __u8 data[]; / Its size is given in the record_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtiovf_migration_file {
    pub filp: *mut file,
// synchronize access to the file state
    pub lock: mutex,
    pub max_pos: loff_t,
    pub pre_copy_initial_bytes: u64,
    pub pre_copy_rl_state: ratelimit_state,
    pub record_size: u64,
    pub record_tag: u32,
    pub has_obj_id:1: u8,
    pub obj_id: u32,
    pub state: virtiovf_migf_state,
    pub load_state: virtiovf_load_state,
// synchronize access to the lists
    pub list_lock: mutex,
    pub buf_list: list_head,
    pub avail_list: list_head,
    pub buf: *mut virtiovf_data_buffer,
    pub buf_header: *mut virtiovf_data_buffer,
    pub virtvdev: *mut virtiovf_pci_core_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtiovf_pci_core_device {
    pub core_device: vfio_pci_core_device,

    pub bar0_virtual_buf: *mut u8,
// synchronize access to the virtual buf
    pub bar_mutex: mutex,
    pub notify_addr: *mut void __iomem,
    pub notify_offset: u64,
    pub pci_base_addr_0: __le32,
    pub pci_cmd: __le16,
    pub bar0_virtual_buf_size: u8,
    pub notify_bar: u8,

// LM related
    pub migrate_cap:1: u8,
    pub deferred_reset:1: u8,
// protect migration state
    pub state_mutex: mutex,
    pub mig_state: vfio_device_mig_state,
// protect the reset_done flow
    pub reset_lock: spinlock_t,
    pub resuming_migf: *mut virtiovf_migration_file,
    pub saving_migf: *mut virtiovf_migration_file,
}

extern "C" {
    pub fn virtiovf_set_migratable(virtvdev: *mut virtiovf_pci_core_device);
}
extern "C" {
    pub fn virtiovf_open_migration(virtvdev: *mut virtiovf_pci_core_device);
}
extern "C" {
    pub fn virtiovf_close_migration(virtvdev: *mut virtiovf_pci_core_device);
}
extern "C" {
    pub fn virtiovf_migration_reset_done(pdev: *mut pci_dev);
}

extern "C" {
    pub fn virtiovf_open_legacy_io(virtvdev: *mut virtiovf_pci_core_device) -> c_int;
}
extern "C" {
    pub fn virtiovf_support_legacy_io(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn virtiovf_init_legacy_io(virtvdev: *mut virtiovf_pci_core_device) -> c_int;
}
extern "C" {
    pub fn virtiovf_release_legacy_io(virtvdev: *mut virtiovf_pci_core_device);
}
extern "C" {
    pub fn virtiovf_legacy_io_reset_done(pdev: *mut pci_dev);
}

