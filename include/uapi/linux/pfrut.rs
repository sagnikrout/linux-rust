//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pfrut.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Platform Firmware Runtime Update header
//
// Copyright(c) 2021 Intel Corporation. All rights reserved.
//

pub const PFRUT_IOCTL_MAGIC: c_uint = 0xEE;
//
// PFRU_IOC_SET_REV - _IOW(PFRUT_IOCTL_MAGIC, 0x01, unsigned int)
//
// Return:
// * 0			- success
// * -EFAULT		- fail to read the revision id
// * -EINVAL		- user provides an invalid revision id
//
// Set the Revision ID for Platform Firmware Runtime Update.
//

//
// PFRU_IOC_STAGE - _IOW(PFRUT_IOCTL_MAGIC, 0x02, unsigned int)
//
// Return:
// * 0			- success
// * -EINVAL		- stage phase returns invalid result
//
// Stage a capsule image from communication buffer and perform authentication.
//

//
// PFRU_IOC_ACTIVATE - _IOW(PFRUT_IOCTL_MAGIC, 0x03, unsigned int)
//
// Return:
// * 0			- success
// * -EINVAL		- activate phase returns invalid result
//
// Activate a previously staged capsule image.
//

//
// PFRU_IOC_STAGE_ACTIVATE - _IOW(PFRUT_IOCTL_MAGIC, 0x04, unsigned int)
//
// Return:
// * 0			- success
// * -EINVAL		- stage/activate phase returns invalid result.
//
// Perform both stage and activation action.
//

//
// PFRU_IOC_QUERY_CAP - _IOR(PFRUT_IOCTL_MAGIC, 0x05,
// struct pfru_update_cap_info)
//
// Return:
// * 0			- success
// * -EINVAL		- query phase returns invalid result
// * -EFAULT		- the result fails to be copied to userspace
//
// Retrieve information on the Platform Firmware Runtime Update capability.
// The information is a struct pfru_update_cap_info.
//

//
// struct pfru_payload_hdr - Capsule file payload header.
//
// @sig: Signature of this capsule file.
// @hdr_version: Revision of this header structure.
// @hdr_size: Size of this header, including the OemHeader bytes.
// @hw_ver: The supported firmware version.
// @rt_ver: Version of the code injection image.
// @platform_id: A platform specific GUID to specify the platform what
// this capsule image support.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfru_payload_hdr {
    pub sig: __u32,
    pub hdr_version: __u32,
    pub hdr_size: __u32,
    pub hw_ver: __u32,
    pub rt_ver: __u32,
    pub platform_id: [__u8; 16],
    pub svn_ver: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pfru_dsm_status {
    DSM_SUCCEED = 0,
    DSM_FUNC_NOT_SUPPORT = 1,
    DSM_INVAL_INPUT = 2,
    DSM_HARDWARE_ERR = 3,
    DSM_RETRY_SUGGESTED = 4,
    DSM_UNKNOWN = 5,
    DSM_FUNC_SPEC_ERR = 6,
}

//
// struct pfru_update_cap_info - Runtime update capability information.
//
// @status: Indicator of whether this query succeed.
// @update_cap: Bitmap to indicate whether the feature is supported.
// @code_type: A buffer containing an image type GUID.
// @fw_version: Platform firmware version.
// @code_rt_version: Code injection runtime version for anti-rollback.
// @drv_type: A buffer containing an image type GUID.
// @drv_rt_version: The version of the driver update runtime code.
// @drv_svn: The secure version number(SVN) of the driver update runtime code.
// @platform_id: A buffer containing a platform ID GUID.
// @oem_id: A buffer containing an OEM ID GUID.
// @oem_info_len: Length of the buffer containing the vendor specific information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfru_update_cap_info {
    pub status: __u32,
    pub update_cap: __u32,
    pub code_type: [__u8; 16],
    pub fw_version: __u32,
    pub code_rt_version: __u32,
    pub drv_type: [__u8; 16],
    pub drv_rt_version: __u32,
    pub drv_svn: __u32,
    pub platform_id: [__u8; 16],
    pub oem_id: [__u8; 16],
    pub oem_info_len: __u32,
}

//
// struct pfru_com_buf_info - Communication buffer information.
//
// @status: Indicator of whether this query succeed.
// @ext_status: Implementation specific query result.
// @addr_lo: Low 32bit physical address of the communication buffer to hold
// a runtime update package.
// @addr_hi: High 32bit physical address of the communication buffer to hold
// a runtime update package.
// @buf_size: Maximum size in bytes of the communication buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfru_com_buf_info {
    pub status: __u32,
    pub ext_status: __u32,
    pub addr_lo: __u64,
    pub addr_hi: __u64,
    pub buf_size: __u32,
}

//
// struct pfru_updated_result - Platform firmware runtime update result information.
// @status: Indicator of whether this update succeed.
// @ext_status: Implementation specific update result.
// @low_auth_time: Low 32bit value of image authentication time in nanosecond.
// @high_auth_time: High 32bit value of image authentication time in nanosecond.
// @low_exec_time: Low 32bit value of image execution time in nanosecond.
// @high_exec_time: High 32bit value of image execution time in nanosecond.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfru_updated_result {
    pub status: __u32,
    pub ext_status: __u32,
    pub low_auth_time: __u64,
    pub high_auth_time: __u64,
    pub low_exec_time: __u64,
    pub high_exec_time: __u64,
}

//
// struct pfrt_log_data_info - Log Data from telemetry service.
// @status: Indicator of whether this update succeed.
// @ext_status: Implementation specific update result.
// @chunk1_addr_lo: Low 32bit physical address of the telemetry data chunk1
// starting address.
// @chunk1_addr_hi: High 32bit physical address of the telemetry data chunk1
// starting address.
// @chunk2_addr_lo: Low 32bit physical address of the telemetry data chunk2
// starting address.
// @chunk2_addr_hi: High 32bit physical address of the telemetry data chunk2
// starting address.
// @max_data_size: Maximum supported size of data of all data chunks combined.
// @chunk1_size: Data size in bytes of the telemetry data chunk1 buffer.
// @chunk2_size: Data size in bytes of the telemetry data chunk2 buffer.
// @rollover_cnt: Number of times telemetry data buffer is overwritten
// since telemetry buffer reset.
// @reset_cnt: Number of times telemetry services resets that results in
// rollover count and data chunk buffers are reset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfrt_log_data_info {
    pub status: __u32,
    pub ext_status: __u32,
    pub chunk1_addr_lo: __u64,
    pub chunk1_addr_hi: __u64,
    pub chunk2_addr_lo: __u64,
    pub chunk2_addr_hi: __u64,
    pub max_data_size: __u32,
    pub chunk1_size: __u32,
    pub chunk2_size: __u32,
    pub rollover_cnt: __u32,
    pub reset_cnt: __u32,
}

//
// struct pfrt_log_info - Telemetry log information.
// @log_level: The telemetry log level.
// @log_type: The telemetry log type(history and execution).
// @log_revid: The telemetry log revision id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfrt_log_info {
    pub log_level: __u32,
    pub log_type: __u32,
    pub log_revid: __u32,
}

//
// PFRT_LOG_IOC_SET_INFO - _IOW(PFRUT_IOCTL_MAGIC, 0x06,
// struct pfrt_log_info)
//
// Return:
// * 0			- success
// * -EFAULT		- fail to get the setting parameter
// * -EINVAL		- fail to set the log level
//
// Set the PFRT log level and log type. The input information is
// a struct pfrt_log_info.
//

//
// PFRT_LOG_IOC_GET_INFO - _IOR(PFRUT_IOCTL_MAGIC, 0x07,
// struct pfrt_log_info)
//
// Return:
// * 0			- success
// * -EINVAL		- fail to get the log level
// * -EFAULT		- fail to copy the result back to userspace
//
// Retrieve log level and log type of the telemetry. The information is
// a struct pfrt_log_info.
//

//
// PFRT_LOG_IOC_GET_DATA_INFO - _IOR(PFRUT_IOCTL_MAGIC, 0x08,
// struct pfrt_log_data_info)
//
// Return:
// * 0			- success
// * -EINVAL		- fail to get the log buffer information
// * -EFAULT		- fail to copy the log buffer information to userspace
//
// Retrieve data information about the telemetry. The information
// is a struct pfrt_log_data_info.
//

