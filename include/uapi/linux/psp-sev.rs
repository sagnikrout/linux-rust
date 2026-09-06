//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/psp-sev.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Userspace interface for AMD Secure Encrypted Virtualization (SEV)
// platform management commands.
//
// Copyright (C) 2016-2017 Advanced Micro Devices, Inc.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//
// SEV API specification is available at: https://developer.amd.com/sev
//

//
// SEV platform commands
//
// SEV Firmware status code
//
// This error code is not in the SEV spec. Its purpose is to convey that
// there was an error that prevented the SEV firmware from being called.
// The SEV API error codes are 16 bits, so the -1 value will not overlap
// with possible values from the specification.
//
// struct sev_user_data_status - PLATFORM_STATUS command parameters
//
// @major: major API version
// @minor: minor API version
// @state: platform state
// @flags: platform config flags
// @build: firmware build id for API version
// @guest_count: number of active guests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_status {
    pub /: *mut *mut __u8 api_major; / Out,
    pub /: *mut *mut __u8 api_minor; / Out,
    pub /: *mut *mut __u8 state; / Out,
    pub /: *mut *mut __u32 flags; / Out,
    pub /: *mut *mut __u8 build; / Out,
    pub /: *mut *mut __u32 guest_count; / Out,
    pub __packed: },
pub const SEV_STATUS_FLAGS_CONFIG_ES: c_uint = 0x0100;
//
// struct sev_user_data_pek_csr - PEK_CSR command parameters
//
// @address: PEK certificate chain
// @length: length of certificate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_pek_csr {
    pub /: *mut *mut __u64 address; / In,
    pub /: *mut *mut __u32 length; / In/Out,
    pub __packed: },
//
// struct sev_user_data_cert_import - PEK_CERT_IMPORT command parameters
//
// @pek_address: PEK certificate chain
// @pek_len: length of PEK certificate
// @oca_address: OCA certificate chain
// @oca_len: length of OCA certificate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_pek_cert_import {
    pub /: *mut *mut __u64 pek_cert_address; / In,
    pub /: *mut *mut __u32 pek_cert_len; / In,
    pub /: *mut *mut __u64 oca_cert_address; / In,
    pub /: *mut *mut __u32 oca_cert_len; / In,
    pub __packed: },
//
// struct sev_user_data_pdh_cert_export - PDH_CERT_EXPORT command parameters
//
// @pdh_address: PDH certificate address
// @pdh_len: length of PDH certificate
// @cert_chain_address: PDH certificate chain
// @cert_chain_len: length of PDH certificate chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_pdh_cert_export {
    pub /: *mut *mut __u64 pdh_cert_address; / In,
    pub /: *mut *mut __u32 pdh_cert_len; / In/Out,
    pub /: *mut *mut __u64 cert_chain_address; / In,
    pub /: *mut *mut __u32 cert_chain_len; / In/Out,
    pub __packed: },
//
// struct sev_user_data_get_id - GET_ID command parameters (deprecated)
//
// @socket1: Buffer to pass unique ID of first socket
// @socket2: Buffer to pass unique ID of second socket
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_get_id {
    pub /: *mut *mut __u8 socket1[64]; / Out,
    pub /: *mut *mut __u8 socket2[64]; / Out,
    pub __packed: },
//
// struct sev_user_data_get_id2 - GET_ID command parameters
// @address: Buffer to store unique ID
// @length: length of the unique ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_get_id2 {
    pub /: *mut *mut __u64 address; / In,
    pub /: *mut *mut __u32 length; / In/Out,
    pub __packed: },
//
// struct sev_user_data_snp_status - SNP status
//
// @api_major: API major version
// @api_minor: API minor version
// @state: current platform state
// @is_rmp_initialized: whether RMP is initialized or not
// @rsvd: reserved
// @build_id: firmware build id for the API version
// @mask_chip_id: whether chip id is present in attestation reports or not
// @mask_chip_key: whether attestation reports are signed or not
// @vlek_en: VLEK (Version Loaded Endorsement Key) hashstick is loaded
// @feature_info: whether SNP_FEATURE_INFO command is available
// @rapl_dis: whether RAPL is disabled
// @ciphertext_hiding_cap: whether platform has ciphertext hiding capability
// @ciphertext_hiding_en: whether ciphertext hiding is enabled
// @rsvd1: reserved
// @guest_count: the number of guest currently managed by the firmware
// @current_tcb_version: current TCB version
// @reported_tcb_version: reported TCB version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_snp_status {
    pub /: *mut *mut __u8 api_major; / Out,
    pub /: *mut *mut __u8 api_minor; / Out,
    pub /: *mut *mut __u8 state; / Out,
    pub /: *mut *mut __u8 is_rmp_initialized:1; / Out,
    pub rsvd:7: __u8,
    pub /: *mut *mut __u32 build_id; / Out,
    pub /: *mut *mut __u32 mask_chip_id:1; / Out,
    pub /: *mut *mut __u32 mask_chip_key:1; / Out,
    pub /: *mut *mut __u32 vlek_en:1; / Out,
    pub /: *mut *mut __u32 feature_info:1; / Out,
    pub /: *mut *mut __u32 rapl_dis:1; / Out,
    pub /: *mut *mut __u32 ciphertext_hiding_cap:1; / Out,
    pub /: *mut *mut __u32 ciphertext_hiding_en:1; / Out,
    pub rsvd1:25: __u32,
    pub /: *mut *mut __u32 guest_count; / Out,
    pub /: *mut *mut __u64 current_tcb_version; / Out,
    pub /: *mut *mut __u64 reported_tcb_version; / Out,
    pub __packed: },
//
// struct sev_user_data_snp_config - system wide configuration value for SNP.
//
// @reported_tcb: the TCB version to report in the guest attestation report.
// @mask_chip_id: whether chip id is present in attestation reports or not
// @mask_chip_key: whether attestation reports are signed or not
// @rsvd: reserved
// @rsvd1: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_snp_config {
    pub /: *mut *mut __u64 reported_tcb ; / In,
    pub /: *mut *mut __u32 mask_chip_id:1; / In,
    pub /: *mut *mut __u32 mask_chip_key:1; / In,
    pub /: *mut *mut __u32 rsvd:30; / In,
    pub rsvd1: [__u8; 52],
    pub __packed: },
//
// struct sev_data_snp_vlek_load - SNP_VLEK_LOAD structure
//
// @len: length of the command buffer read by the PSP
// @vlek_wrapped_version: version of wrapped VLEK hashstick (Must be 0h)
// @rsvd: reserved
// @vlek_wrapped_address: address of a wrapped VLEK hashstick
// (struct sev_user_data_snp_wrapped_vlek_hashstick)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_snp_vlek_load {
    pub /: *mut *mut __u32 len; / In,
    pub /: *mut *mut __u8 vlek_wrapped_version; / In,
    pub /: *mut *mut __u8 rsvd[3]; / In,
    pub /: *mut *mut __u64 vlek_wrapped_address; / In,
    pub __packed: },
//
// struct sev_user_data_snp_vlek_wrapped_vlek_hashstick - Wrapped VLEK data
//
// @data: Opaque data provided by AMD KDS (as described in SEV-SNP Firmware ABI
// 1.54, SNP_VLEK_LOAD)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_user_data_snp_wrapped_vlek_hashstick {
    pub /: *mut *mut __u8 data[432]; / In,
    pub __packed: },
//
// struct sev_issue_cmd - SEV ioctl parameters
//
// @cmd: SEV commands to execute
// @data: pointer to the command structure
// @error: SEV FW return code on failure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_issue_cmd {
    pub /: *mut *mut __u32 cmd; / In,
    pub /: *mut *mut __u64 data; / In,
    pub /: *mut *mut __u32 error; / Out,
    pub __packed: },

