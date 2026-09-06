//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/xe_sriov_vfio.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

//
// xe_sriov_vfio_get_pf() - Get PF &xe_device.
// @pdev: the VF &pci_dev device
//
// Return: pointer to PF &xe_device, NULL otherwise.
//
// xe_sriov_vfio_migration_supported() - Check if migration is supported.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
//
// Return: true if migration is supported, false otherwise.
//
extern "C" {
    pub fn xe_sriov_vfio_migration_supported(xe: *mut xe_device) -> bool;
}
//
// xe_sriov_vfio_flr_prepare() - Notify PF that VF FLR prepare has started.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// This function marks VF FLR as pending before PF receives GuC FLR event.
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_flr_prepare(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_wait_flr_done() - Wait for VF FLR completion.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// This function will wait until VF FLR is processed by PF on all tiles (or
// until timeout occurs).
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_wait_flr_done(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_suspend_device() - Suspend VF.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// This function will pause VF on all tiles/GTs.
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_suspend_device(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_resume_device() - Resume VF.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// This function will resume VF on all tiles.
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_resume_device(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_stop_copy_enter() - Initiate a VF device migration data save.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_stop_copy_enter(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_stop_copy_exit() - Finish a VF device migration data save.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_stop_copy_exit(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_resume_data_enter() - Initiate a VF device migration data restore.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_resume_data_enter(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_resume_data_exit() - Finish a VF device migration data restore.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_resume_data_exit(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_error() - Move VF device to error state.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Reset is needed to move it out of error state.
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_error(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
//
// xe_sriov_vfio_data_read() - Read migration data from the VF device.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
// @buf: start address of userspace buffer
// @len: requested read size from userspace
//
// Return: number of bytes that has been successfully read,
// 0 if no more migration data is available, -errno on failure.
//
// xe_sriov_vfio_data_write() - Write migration data to the VF device.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
// @buf: start address of userspace buffer
// @len: requested write size from userspace
//
// Return: number of bytes that has been successfully written, -errno on failure.
//
// xe_sriov_vfio_stop_copy_size() - Get a size estimate of VF device migration data.
// @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
// @vfid: the VF identifier (can't be 0)
//
// Return: migration data size in bytes or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_vfio_stop_copy_size(xe: *mut xe_device, vfid: c_uint) -> isize;
}
