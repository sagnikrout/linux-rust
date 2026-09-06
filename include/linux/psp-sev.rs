//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psp-sev.h
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
// AMD Secure Encrypted Virtualization (SEV) driver interface
//
// Copyright (C) 2016-2017 Advanced Micro Devices, Inc.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//
// SEV API spec is available at https://developer.amd.com/sev
//

// As defined by SEV API, under "Guest Policy".

// As defined by SEV-SNP Firmware ABI, under "Guest Policy".

// Base SEV-SNP policy bitmask for minimum supported SEV firmware version

pub const SEV_FW_BLOB_MAX_SIZE: c_uint = 0x4000	/* 16KB */;
//
// SEV platform state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sev_state {
    SEV_STATE_UNINIT		= 0x0,
    SEV_STATE_INIT			= 0x1,
    SEV_STATE_WORKING		= 0x2,

    SEV_STATE_MAX
}

//
// SEV platform and guest management commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sev_cmd {
// platform commands
    SEV_CMD_INIT			= 0x001,
    SEV_CMD_SHUTDOWN		= 0x002,
    SEV_CMD_FACTORY_RESET		= 0x003,
    SEV_CMD_PLATFORM_STATUS		= 0x004,
    SEV_CMD_PEK_GEN			= 0x005,
    SEV_CMD_PEK_CSR			= 0x006,
    SEV_CMD_PEK_CERT_IMPORT		= 0x007,
    SEV_CMD_PDH_CERT_EXPORT		= 0x008,
    SEV_CMD_PDH_GEN			= 0x009,
    SEV_CMD_DF_FLUSH		= 0x00A,
    SEV_CMD_DOWNLOAD_FIRMWARE	= 0x00B,
    SEV_CMD_GET_ID			= 0x00C,
    SEV_CMD_INIT_EX                 = 0x00D,

// Guest commands
    SEV_CMD_DECOMMISSION		= 0x020,
    SEV_CMD_ACTIVATE		= 0x021,
    SEV_CMD_DEACTIVATE		= 0x022,
    SEV_CMD_GUEST_STATUS		= 0x023,

// Guest launch commands
    SEV_CMD_LAUNCH_START		= 0x030,
    SEV_CMD_LAUNCH_UPDATE_DATA	= 0x031,
    SEV_CMD_LAUNCH_UPDATE_VMSA	= 0x032,
    SEV_CMD_LAUNCH_MEASURE		= 0x033,
    SEV_CMD_LAUNCH_UPDATE_SECRET	= 0x034,
    SEV_CMD_LAUNCH_FINISH		= 0x035,
    SEV_CMD_ATTESTATION_REPORT	= 0x036,

// Guest migration commands (outgoing)
    SEV_CMD_SEND_START		= 0x040,
    SEV_CMD_SEND_UPDATE_DATA	= 0x041,
    SEV_CMD_SEND_UPDATE_VMSA	= 0x042,
    SEV_CMD_SEND_FINISH		= 0x043,
    SEV_CMD_SEND_CANCEL		= 0x044,

// Guest migration commands (incoming)
    SEV_CMD_RECEIVE_START		= 0x050,
    SEV_CMD_RECEIVE_UPDATE_DATA	= 0x051,
    SEV_CMD_RECEIVE_UPDATE_VMSA	= 0x052,
    SEV_CMD_RECEIVE_FINISH		= 0x053,

// Guest debug commands
    SEV_CMD_DBG_DECRYPT		= 0x060,
    SEV_CMD_DBG_ENCRYPT		= 0x061,

// SNP specific commands
    SEV_CMD_SNP_INIT		= 0x081,
    SEV_CMD_SNP_SHUTDOWN		= 0x082,
    SEV_CMD_SNP_PLATFORM_STATUS	= 0x083,
    SEV_CMD_SNP_DF_FLUSH		= 0x084,
    SEV_CMD_SNP_INIT_EX		= 0x085,
    SEV_CMD_SNP_SHUTDOWN_EX		= 0x086,
    SEV_CMD_SNP_DECOMMISSION	= 0x090,
    SEV_CMD_SNP_ACTIVATE		= 0x091,
    SEV_CMD_SNP_GUEST_STATUS	= 0x092,
    SEV_CMD_SNP_GCTX_CREATE		= 0x093,
    SEV_CMD_SNP_GUEST_REQUEST	= 0x094,
    SEV_CMD_SNP_ACTIVATE_EX		= 0x095,
    SEV_CMD_SNP_LAUNCH_START	= 0x0A0,
    SEV_CMD_SNP_LAUNCH_UPDATE	= 0x0A1,
    SEV_CMD_SNP_LAUNCH_FINISH	= 0x0A2,
    SEV_CMD_SNP_DBG_DECRYPT		= 0x0B0,
    SEV_CMD_SNP_DBG_ENCRYPT		= 0x0B1,
    SEV_CMD_SNP_VERIFY_MITIGATION	= 0x0B2,
    SEV_CMD_SNP_PAGE_SWAP_OUT	= 0x0C0,
    SEV_CMD_SNP_PAGE_SWAP_IN	= 0x0C1,
    SEV_CMD_SNP_PAGE_MOVE		= 0x0C2,
    SEV_CMD_SNP_PAGE_MD_INIT	= 0x0C3,
    SEV_CMD_SNP_PAGE_SET_STATE	= 0x0C6,
    SEV_CMD_SNP_PAGE_RECLAIM	= 0x0C7,
    SEV_CMD_SNP_PAGE_UNSMASH	= 0x0C8,
    SEV_CMD_SNP_CONFIG		= 0x0C9,
    SEV_CMD_SNP_DOWNLOAD_FIRMWARE_EX = 0x0CA,
    SEV_CMD_SNP_COMMIT		= 0x0CB,
    SEV_CMD_SNP_VLEK_LOAD		= 0x0CD,
    SEV_CMD_SNP_FEATURE_INFO	= 0x0CE,

// SEV-TIO commands
    SEV_CMD_TIO_STATUS		= 0x0D0,
    SEV_CMD_TIO_INIT		= 0x0D1,
    SEV_CMD_TIO_DEV_CREATE		= 0x0D2,
    SEV_CMD_TIO_DEV_RECLAIM		= 0x0D3,
    SEV_CMD_TIO_DEV_CONNECT		= 0x0D4,
    SEV_CMD_TIO_DEV_DISCONNECT	= 0x0D5,
    SEV_CMD_MAX,
}

//
// struct sev_data_init - INIT command parameters
//
// @flags: processing flags
// @tmr_address: system physical address used for SEV-ES
// @tmr_len: len of tmr_address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_init {
    pub /: *mut *mut u32 flags; / In,
    pub /: *mut *mut u32 reserved; / In,
    pub /: *mut *mut u64 tmr_address; / In,
    pub /: *mut *mut u32 tmr_len; / In,
    pub __packed: },
//
// struct sev_data_init_ex - INIT_EX command parameters
//
// @length: len of the command buffer read by the PSP
// @flags: processing flags
// @tmr_address: system physical address used for SEV-ES
// @tmr_len: len of tmr_address
// @nv_address: system physical address used for PSP NV storage
// @nv_len: len of nv_address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_init_ex {
    pub /: *mut *mut u32 length; / In,
    pub /: *mut *mut u32 flags; / In,
    pub /: *mut *mut u64 tmr_address; / In,
    pub /: *mut *mut u32 tmr_len; / In,
    pub /: *mut *mut u32 reserved; / In,
    pub /: *mut *mut u64 nv_address; / In/Out,
    pub /: *mut *mut u32 nv_len; / In,
    pub __packed: },
pub const SEV_INIT_FLAGS_SEV_ES: c_uint = 0x01;
//
// struct sev_data_pek_csr - PEK_CSR command parameters
//
// @address: PEK certificate chain
// @len: len of certificate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_pek_csr {
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In/Out,
    pub __packed: },
//
// struct sev_data_cert_import - PEK_CERT_IMPORT command parameters
//
// @pek_address: PEK certificate chain
// @pek_len: len of PEK certificate
// @oca_address: OCA certificate chain
// @oca_len: len of OCA certificate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_pek_cert_import {
    pub /: *mut *mut u64 pek_cert_address; / In,
    pub /: *mut *mut u32 pek_cert_len; / In,
    pub /: *mut *mut u32 reserved; / In,
    pub /: *mut *mut u64 oca_cert_address; / In,
    pub /: *mut *mut u32 oca_cert_len; / In,
    pub __packed: },
//
// struct sev_data_download_firmware - DOWNLOAD_FIRMWARE command parameters
//
// @address: physical address of firmware image
// @len: len of the firmware image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_download_firmware {
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In,
    pub __packed: },
//
// struct sev_data_get_id - GET_ID command parameters
//
// @address: physical address of region to place unique CPU ID(s)
// @len: len of the region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_get_id {
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In/Out,
    pub __packed: },
//
// struct sev_data_pdh_cert_export - PDH_CERT_EXPORT command parameters
//
// @pdh_address: PDH certificate address
// @pdh_len: len of PDH certificate
// @cert_chain_address: PDH certificate chain
// @cert_chain_len: len of PDH certificate chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_pdh_cert_export {
    pub /: *mut *mut u64 pdh_cert_address; / In,
    pub /: *mut *mut u32 pdh_cert_len; / In/Out,
    pub /: *mut *mut u32 reserved; / In,
    pub /: *mut *mut u64 cert_chain_address; / In,
    pub /: *mut *mut u32 cert_chain_len; / In/Out,
    pub __packed: },
//
// struct sev_data_decommission - DECOMMISSION command parameters
//
// @handle: handle of the VM to decommission
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_decommission {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_activate - ACTIVATE command parameters
//
// @handle: handle of the VM to activate
// @asid: asid assigned to the VM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_activate {
    pub /: *mut *mut u32 handle; / In,
    pub /: *mut *mut u32 asid; / In,
    pub __packed: },
//
// struct sev_data_deactivate - DEACTIVATE command parameters
//
// @handle: handle of the VM to deactivate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_deactivate {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_guest_status - SEV GUEST_STATUS command parameters
//
// @handle: handle of the VM to retrieve status
// @policy: policy information for the VM
// @asid: current ASID of the VM
// @state: current state of the VM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_guest_status {
    pub /: *mut *mut u32 handle; / In,
    pub /: *mut *mut u32 policy; / Out,
    pub /: *mut *mut u32 asid; / Out,
    pub /: *mut *mut u8 state; / Out,
    pub __packed: },
//
// struct sev_data_launch_start - LAUNCH_START command parameters
//
// @handle: handle assigned to the VM
// @policy: guest launch policy
// @dh_cert_address: physical address of DH certificate blob
// @dh_cert_len: len of DH certificate blob
// @session_address: physical address of session parameters
// @session_len: len of session parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_start {
    pub /: *mut *mut u32 handle; / In/Out,
    pub /: *mut *mut u32 policy; / In,
    pub /: *mut *mut u64 dh_cert_address; / In,
    pub /: *mut *mut u32 dh_cert_len; / In,
    pub /: *mut *mut u32 reserved; / In,
    pub /: *mut *mut u64 session_address; / In,
    pub /: *mut *mut u32 session_len; / In,
    pub __packed: },
//
// struct sev_data_launch_update_data - LAUNCH_UPDATE_DATA command parameter
//
// @handle: handle of the VM to update
// @len: len of memory to be encrypted
// @address: physical address of memory region to encrypt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_update_data {
    pub /: *mut *mut u32 handle; / In,
    pub reserved: u32,
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In,
    pub __packed: },
//
// struct sev_data_launch_update_vmsa - LAUNCH_UPDATE_VMSA command
//
// @handle: handle of the VM
// @address: physical address of memory region to encrypt
// @len: len of memory region to encrypt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_update_vmsa {
    pub /: *mut *mut u32 handle; / In,
    pub reserved: u32,
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In,
    pub __packed: },
//
// struct sev_data_launch_measure - LAUNCH_MEASURE command parameters
//
// @handle: handle of the VM to process
// @address: physical address containing the measurement blob
// @len: len of measurement blob
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_measure {
    pub /: *mut *mut u32 handle; / In,
    pub reserved: u32,
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In/Out,
    pub __packed: },
//
// struct sev_data_launch_secret - LAUNCH_SECRET command parameters
//
// @handle: handle of the VM to process
// @hdr_address: physical address containing the packet header
// @hdr_len: len of packet header
// @guest_address: system physical address of guest memory region
// @guest_len: len of guest_paddr
// @trans_address: physical address of transport memory buffer
// @trans_len: len of transport memory buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_secret {
    pub /: *mut *mut u32 handle; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 hdr_address; / In,
    pub /: *mut *mut u32 hdr_len; / In,
    pub reserved2: u32,
    pub /: *mut *mut u64 guest_address; / In,
    pub /: *mut *mut u32 guest_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 trans_address; / In,
    pub /: *mut *mut u32 trans_len; / In,
    pub __packed: },
//
// struct sev_data_launch_finish - LAUNCH_FINISH command parameters
//
// @handle: handle of the VM to process
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_launch_finish {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_send_start - SEND_START command parameters
//
// @handle: handle of the VM to process
// @policy: policy information for the VM
// @pdh_cert_address: physical address containing PDH certificate
// @pdh_cert_len: len of PDH certificate
// @plat_certs_address: physical address containing platform certificate
// @plat_certs_len: len of platform certificate
// @amd_certs_address: physical address containing AMD certificate
// @amd_certs_len: len of AMD certificate
// @session_address: physical address containing Session data
// @session_len: len of session data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_send_start {
    pub /: *mut *mut u32 handle; / In,
    pub /: *mut *mut u32 policy; / Out,
    pub /: *mut *mut u64 pdh_cert_address; / In,
    pub /: *mut *mut u32 pdh_cert_len; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 plat_certs_address; / In,
    pub /: *mut *mut u32 plat_certs_len; / In,
    pub reserved2: u32,
    pub /: *mut *mut u64 amd_certs_address; / In,
    pub /: *mut *mut u32 amd_certs_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 session_address; / In,
    pub /: *mut *mut u32 session_len; / In/Out,
    pub __packed: },
//
// struct sev_data_send_update - SEND_UPDATE_DATA command
//
// @handle: handle of the VM to process
// @hdr_address: physical address containing packet header
// @hdr_len: len of packet header
// @guest_address: physical address of guest memory region to send
// @guest_len: len of guest memory region to send
// @trans_address: physical address of host memory region
// @trans_len: len of host memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_send_update_data {
    pub /: *mut *mut u32 handle; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 hdr_address; / In,
    pub /: *mut *mut u32 hdr_len; / In/Out,
    pub reserved2: u32,
    pub /: *mut *mut u64 guest_address; / In,
    pub /: *mut *mut u32 guest_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 trans_address; / In,
    pub /: *mut *mut u32 trans_len; / In,
    pub __packed: },
//
// struct sev_data_send_update - SEND_UPDATE_VMSA command
//
// @handle: handle of the VM to process
// @hdr_address: physical address containing packet header
// @hdr_len: len of packet header
// @guest_address: physical address of guest memory region to send
// @guest_len: len of guest memory region to send
// @trans_address: physical address of host memory region
// @trans_len: len of host memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_send_update_vmsa {
    pub /: *mut *mut u32 handle; / In,
    pub /: *mut *mut u64 hdr_address; / In,
    pub /: *mut *mut u32 hdr_len; / In/Out,
    pub reserved2: u32,
    pub /: *mut *mut u64 guest_address; / In,
    pub /: *mut *mut u32 guest_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 trans_address; / In,
    pub /: *mut *mut u32 trans_len; / In,
    pub __packed: },
//
// struct sev_data_send_finish - SEND_FINISH command parameters
//
// @handle: handle of the VM to process
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_send_finish {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_send_cancel - SEND_CANCEL command parameters
//
// @handle: handle of the VM to process
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_send_cancel {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_receive_start - RECEIVE_START command parameters
//
// @handle: handle of the VM to perform receive operation
// @pdh_cert_address: system physical address containing PDH certificate blob
// @pdh_cert_len: len of PDH certificate blob
// @session_address: system physical address containing session blob
// @session_len: len of session blob
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_receive_start {
    pub /: *mut *mut u32 handle; / In/Out,
    pub /: *mut *mut u32 policy; / In,
    pub /: *mut *mut u64 pdh_cert_address; / In,
    pub /: *mut *mut u32 pdh_cert_len; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 session_address; / In,
    pub /: *mut *mut u32 session_len; / In,
    pub __packed: },
//
// struct sev_data_receive_update_data - RECEIVE_UPDATE_DATA command parameters
//
// @handle: handle of the VM to update
// @hdr_address: physical address containing packet header blob
// @hdr_len: len of packet header
// @guest_address: system physical address of guest memory region
// @guest_len: len of guest memory region
// @trans_address: system physical address of transport buffer
// @trans_len: len of transport buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_receive_update_data {
    pub /: *mut *mut u32 handle; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 hdr_address; / In,
    pub /: *mut *mut u32 hdr_len; / In,
    pub reserved2: u32,
    pub /: *mut *mut u64 guest_address; / In,
    pub /: *mut *mut u32 guest_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 trans_address; / In,
    pub /: *mut *mut u32 trans_len; / In,
    pub __packed: },
//
// struct sev_data_receive_update_vmsa - RECEIVE_UPDATE_VMSA command parameters
//
// @handle: handle of the VM to update
// @hdr_address: physical address containing packet header blob
// @hdr_len: len of packet header
// @guest_address: system physical address of guest memory region
// @guest_len: len of guest memory region
// @trans_address: system physical address of transport buffer
// @trans_len: len of transport buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_receive_update_vmsa {
    pub /: *mut *mut u32 handle; / In,
    pub reserved1: u32,
    pub /: *mut *mut u64 hdr_address; / In,
    pub /: *mut *mut u32 hdr_len; / In,
    pub reserved2: u32,
    pub /: *mut *mut u64 guest_address; / In,
    pub /: *mut *mut u32 guest_len; / In,
    pub reserved3: u32,
    pub /: *mut *mut u64 trans_address; / In,
    pub /: *mut *mut u32 trans_len; / In,
    pub __packed: },
//
// struct sev_data_receive_finish - RECEIVE_FINISH command parameters
//
// @handle: handle of the VM to finish
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_receive_finish {
    pub /: *mut *mut u32 handle; / In,
    pub __packed: },
//
// struct sev_data_dbg - DBG_ENCRYPT/DBG_DECRYPT command parameters
//
// @handle: handle of the VM to perform debug operation
// @src_addr: source address of data to operate on
// @dst_addr: destination address of data to operate on
// @len: len of data to operate on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_dbg {
    pub /: *mut *mut u32 handle; / In,
    pub reserved: u32,
    pub /: *mut *mut u64 src_addr; / In,
    pub /: *mut *mut u64 dst_addr; / In,
    pub /: *mut *mut u32 len; / In,
    pub __packed: },
//
// struct sev_data_attestation_report - SEV_ATTESTATION_REPORT command parameters
//
// @handle: handle of the VM
// @mnonce: a random nonce that will be included in the report.
// @address: physical address where the report will be copied.
// @len: length of the physical buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_attestation_report {
    pub /: *mut *mut u32 handle; / In,
    pub reserved: u32,
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u8 mnonce[16]; / In,
    pub /: *mut *mut u32 len; / In/Out,
    pub __packed: },
//
// struct sev_data_snp_download_firmware - SNP_DOWNLOAD_FIRMWARE command params
//
// @address: physical address of firmware image
// @len: length of the firmware image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_download_firmware {
    pub /: *mut *mut u64 address; / In,
    pub /: *mut *mut u32 len; / In,
    pub __packed: },
//
// struct sev_data_snp_activate - SNP_ACTIVATE command params
//
// @gctx_paddr: system physical address guest context page
// @asid: ASID to bind to the guest
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_activate {
    pub /: *mut *mut u64 gctx_paddr; / In,
    pub /: *mut *mut u32 asid; / In,
    pub __packed: },
//
// struct sev_data_snp_addr - generic SNP command params
//
// @address: physical address of generic data param
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_addr {
    pub /: *mut *mut u64 address; / In/Out,
    pub __packed: },
//
// struct sev_data_snp_launch_start - SNP_LAUNCH_START command params
//
// @gctx_paddr: system physical address of guest context page
// @policy: guest policy
// @ma_gctx_paddr: system physical address of migration agent
// @ma_en: the guest is associated with a migration agent
// @imi_en: launch flow is launching an IMI (Incoming Migration Image) for the
// purpose of guest-assisted migration.
// @rsvd: reserved
// @desired_tsc_khz: hypervisor desired mean TSC freq in kHz of the guest
// @gosvw: guest OS-visible workarounds, as defined by hypervisor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_launch_start {
    pub /: *mut *mut u64 gctx_paddr; / In,
    pub /: *mut *mut u64 policy; / In,
    pub /: *mut *mut u64 ma_gctx_paddr; / In,
    pub /: *mut *mut u32 ma_en:1; / In,
    pub /: *mut *mut u32 imi_en:1; / In,
    pub rsvd:30: u32,
    pub /: *mut *mut u32 desired_tsc_khz; / In,
    pub /: *mut *mut u8 gosvw[16]; / In,
    pub __packed: },
// SNP support page type
}

//
// struct sev_data_snp_launch_update - SNP_LAUNCH_UPDATE command params
//
// @gctx_paddr: system physical address of guest context page
// @page_size: page size 0 indicates 4K and 1 indicates 2MB page
// @page_type: encoded page type
// @imi_page: indicates that this page is part of the IMI (Incoming Migration
// Image) of the guest
// @rsvd: reserved
// @rsvd2: reserved
// @address: system physical address of destination page to encrypt
// @rsvd3: reserved
// @vmpl1_perms: VMPL permission mask for VMPL1
// @vmpl2_perms: VMPL permission mask for VMPL2
// @vmpl3_perms: VMPL permission mask for VMPL3
// @rsvd4: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_launch_update {
    pub /: *mut *mut u64 gctx_paddr; / In,
    pub /: *mut *mut u32 page_size:1; / In,
    pub /: *mut *mut u32 page_type:3; / In,
    pub /: *mut *mut u32 imi_page:1; / In,
    pub rsvd:27: u32,
    pub rsvd2: u32,
    pub /: *mut *mut u64 address; / In,
    pub rsvd3:8: u32,
    pub /: *mut *mut u32 vmpl1_perms:8; / In,
    pub /: *mut *mut u32 vmpl2_perms:8; / In,
    pub /: *mut *mut u32 vmpl3_perms:8; / In,
    pub rsvd4: u32,
    pub __packed: },
//
// struct sev_data_snp_launch_finish - SNP_LAUNCH_FINISH command params
//
// @gctx_paddr: system physical address of guest context page
// @id_block_paddr: system physical address of ID block
// @id_auth_paddr: system physical address of ID block authentication structure
// @id_block_en: indicates whether ID block is present
// @auth_key_en: indicates whether author key is present in authentication structure
// @vcek_disabled: indicates whether use of VCEK is allowed for attestation reports
// @rsvd: reserved
// @host_data: host-supplied data for guest, not interpreted by firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_launch_finish {
    pub gctx_paddr: u64,
    pub id_block_paddr: u64,
    pub id_auth_paddr: u64,
    pub id_block_en:1: u8,
    pub auth_key_en:1: u8,
    pub vcek_disabled:1: u8,
    pub rsvd:61: u64,
    pub host_data: [u8; 32],
    pub __packed: },
//
// struct sev_data_snp_guest_status - SNP_GUEST_STATUS command params
//
// @gctx_paddr: system physical address of guest context page
// @address: system physical address of guest status page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_guest_status {
    pub gctx_paddr: u64,
    pub address: u64,
    pub __packed: },
//
// struct sev_data_snp_page_reclaim - SNP_PAGE_RECLAIM command params
//
// @paddr: system physical address of page to be claimed. The 0th bit in the
// address indicates the page size. 0h indicates 4KB and 1h indicates
// 2MB page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_page_reclaim {
    pub paddr: u64,
    pub __packed: },
//
// struct sev_data_snp_page_unsmash - SNP_PAGE_UNSMASH command params
//
// @paddr: system physical address of page to be unsmashed. The 0th bit in the
// address indicates the page size. 0h indicates 4 KB and 1h indicates
// 2 MB page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_page_unsmash {
    pub paddr: u64,
    pub __packed: },
//
// struct sev_data_snp_dbg - DBG_ENCRYPT/DBG_DECRYPT command parameters
//
// @gctx_paddr: system physical address of guest context page
// @src_addr: source address of data to operate on
// @dst_addr: destination address of data to operate on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_dbg {
    pub /: *mut *mut u64 gctx_paddr; / In,
    pub /: *mut *mut u64 src_addr; / In,
    pub /: *mut *mut u64 dst_addr; / In,
    pub __packed: },
//
// struct sev_data_snp_guest_request - SNP_GUEST_REQUEST command params
//
// @gctx_paddr: system physical address of guest context page
// @req_paddr: system physical address of request page
// @res_paddr: system physical address of response page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_guest_request {
    pub /: *mut *mut u64 gctx_paddr; / In,
    pub /: *mut *mut u64 req_paddr; / In,
    pub /: *mut *mut u64 res_paddr; / In,
    pub __packed: },
//
// struct sev_data_snp_init_ex - SNP_INIT_EX structure
//
// @init_rmp: indicate that the RMP should be initialized.
// @list_paddr_en: indicate that list_paddr is valid
// @rsvd: reserved
// @rsvd1: reserved
// @list_paddr: system physical address of range list
// @rsvd2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_init_ex {
    pub init_rmp:1: u32,
    pub list_paddr_en:1: u32,
    pub rapl_dis:1: u32,
    pub ciphertext_hiding_en:1: u32,
    pub tio_en:1: u32,
    pub rsvd:27: u32,
    pub rsvd1: u32,
    pub list_paddr: u64,
    pub max_snp_asid: u16,
    pub rsvd2: [u8; 46],
    pub __packed: },
//
// struct sev_data_range - RANGE structure
//
// @base: system physical address of first byte of range
// @page_count: number of 4KB pages in this range
// @rsvd: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_range {
    pub base: u64,
    pub page_count: u32,
    pub rsvd: u32,
    pub __packed: },
//
// struct sev_data_range_list - RANGE_LIST structure
//
// @num_elements: number of elements in RANGE_ARRAY
// @rsvd: reserved
// @ranges: array of num_elements of type RANGE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_range_list {
    pub num_elements: u32,
    pub rsvd: u32,
    pub ranges: [sev_data_range; ],
    pub __packed: },
//
// struct sev_data_snp_shutdown_ex - SNP_SHUTDOWN_EX structure
//
// @len: length of the command buffer read by the PSP
// @iommu_snp_shutdown: Disable enforcement of SNP in the IOMMU
// @x86_snp_shutdown: Disable SNP on all cores
// @rsvd1: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_shutdown_ex {
    pub len: u32,
    pub iommu_snp_shutdown:1: u32,
    pub x86_snp_shutdown:1: u32,
    pub rsvd1:30: u32,
    pub __packed: },
//
// struct sev_platform_init_args
//
// @error: SEV firmware error code
// @probe: True if this is being called as part of CCP module probe, which
// will defer SEV_INIT/SEV_INIT_EX firmware initialization until needed
// unless psp_init_on_probe module param is set
// @max_snp_asid: When non-zero, enable ciphertext hiding and specify the
// maximum ASID that can be used for an SEV-SNP guest.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_platform_init_args {
    pub error: c_int,
    pub probe: bool,
    pub max_snp_asid: c_uint,
}

//
// struct sev_data_snp_commit - SNP_COMMIT structure
//
// @len: length of the command buffer read by the PSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_commit {
    pub len: u32,
    pub __packed: },
//
// struct sev_data_snp_feature_info - SEV_SNP_FEATURE_INFO structure
//
// @length: len of the command buffer read by the PSP
// @ecx_in: subfunction index
// @feature_info_paddr : System Physical Address of the FEATURE_INFO structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_feature_info {
    pub length: u32,
    pub ecx_in: u32,
    pub feature_info_paddr: u64,
    pub __packed: },
//
// struct feature_info - FEATURE_INFO structure
//
// @eax: output of SNP_FEATURE_INFO command
// @ebx: output of SNP_FEATURE_INFO command
// @ecx: output of SNP_FEATURE_INFO command
// #edx: output of SNP_FEATURE_INFO command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_feature_info {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub __packed: },
// Feature bits in ECX

// Feature bits in EBX

pub const SNP_MIT_SUBCMD_REQ_STATUS: c_uint = 0x0;
pub const SNP_MIT_SUBCMD_REQ_VERIFY: c_uint = 0x1;
//
// struct sev_data_snp_verify_mitigation - SNP_VERIFY_MITIGATION command params
//
// @length: Length of the command buffer read by the PSP
// @subcommand: Mitigation sub-command for the firmware to execute.
// REQ_STATUS: 0x0 - Request status about currently supported and
// verified mitigations
// REQ_VERIFY: 0x1 - Request to initiate verification mitigation
// operation on a specific mitigation
// @rsvd: Reserved
// @vector: Bit specifying the vulnerability mitigation to process
// @dst_paddr_en: Destination paddr enabled
// @src_paddr_en: Source paddr enabled
// @rsvd1: Reserved
// @rsvd2: Reserved
// @src_paddr: Source address for optional input data
// @dst_paddr: Destination address to write the result
// @rsvd3: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_verify_mitigation {
    pub length: u32,
    pub subcommand: u16,
    pub rsvd: u16,
    pub vector: u64,
    pub 30: rsvd1 :,
    pub rsvd2: [u8; 4],
    pub src_paddr: u64,
    pub dst_paddr: u64,
    pub rsvd3: [u8; 24],
    pub __packed: },
//
// struct sev_data_snp_verify_mitigation_dst - mitigation result vectors
//
// @mit_verified_vector: Bit vector of vulnerability mitigations verified
// @mit_supported_vector: Bit vector of vulnerability mitigations supported
// @mit_failure_status: Status of the verification operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_data_snp_verify_mitigation_dst {
    pub /: *mut *mut u64 mit_verified_vector; / OUT,
    pub /: *mut *mut u64 mit_supported_vector; / OUT,
    pub /: *mut *mut u32 mit_failure_status; / OUT,
    pub __packed: },
//
// struct sev_snp_tcb_version_genoa_milan
//
// @boot_loader: SVN of PSP bootloader
// @tee: SVN of PSP operating system
// @reserved: reserved
// @snp: SVN of SNP firmware
// @microcode: Lowest current patch level of all cores
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_snp_tcb_version_genoa_milan {
    pub boot_loader: u8,
    pub tee: u8,
    pub reserved: [u8; 4],
    pub snp: u8,
    pub microcode: u8,
}

//
// struct sev_snp_tcb_version_turin
//
// @fmc: SVN of FMC firmware
// @boot_loader: SVN of PSP bootloader
// @tee: SVN of PSP operating system
// @snp: SVN of SNP firmware
// @reserved: reserved
// @microcode: Lowest current patch level of all cores
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_snp_tcb_version_turin {
    pub fmc: u8,
    pub boot_loader: u8,
    pub tee: u8,
    pub snp: u8,
    pub reserved: [u8; 3],
    pub microcode: u8,
}

//
// sev_module_init - perform PSP SEV module initialization
//
// Returns:
// 0 if the PSP module is successfully initialized
// negative value if the PSP module initialization fails
//
extern "C" {
    pub fn sev_module_init() -> c_int;
}
//
// sev_platform_init - perform SEV INIT command
//
// @args: struct sev_platform_init_args to pass in arguments
//
// Returns:
// 0 if the SEV successfully processed the command
// -%ENODEV    if the SEV device is not available
// -%ENOTSUPP  if the SEV does not support SEV
// -%ETIMEDOUT if the SEV command timed out
// -%EIO       if the SEV returned a non-zero return code
//
extern "C" {
    pub fn sev_platform_init(args: *mut sev_platform_init_args) -> c_int;
}
//
// sev_platform_status - perform SEV PLATFORM_STATUS command
//
// @status: sev_user_data_status structure to be processed
// @error: SEV command return code
//
// Returns:
// 0 if the SEV successfully processed the command
// -%ENODEV    if the SEV device is not available
// -%ENOTSUPP  if the SEV does not support SEV
// -%ETIMEDOUT if the SEV command timed out
// -%EIO       if the SEV returned a non-zero return code
//
extern "C" {
    pub fn sev_platform_status(status: *mut sev_user_data_status, error: *mut c_int) -> c_int;
}
//
// sev_issue_cmd_external_user - issue SEV command by other driver with a file
// handle.
//
// This function can be used by other drivers to issue a SEV command on
// behalf of userspace. The caller must pass a valid SEV file descriptor
// so that we know that it has access to SEV device.
//
// @filep - SEV device file pointer
// @cmd - command to issue
// @data - command buffer
// @error: SEV command return code
//
// Returns:
// 0 if the SEV successfully processed the command
// -%ENODEV    if the SEV device is not available
// -%ENOTSUPP  if the SEV does not support SEV
// -%ETIMEDOUT if the SEV command timed out
// -%EIO       if the SEV returned a non-zero return code
// -%EINVAL    if the SEV file descriptor is not valid
//
// sev_guest_deactivate - perform SEV DEACTIVATE command
//
// @deactivate: sev_data_deactivate structure to be processed
// @sev_ret: sev command return code
//
// Returns:
// 0 if the sev successfully processed the command
// -%ENODEV    if the sev device is not available
// -%ENOTSUPP  if the sev does not support SEV
// -%ETIMEDOUT if the sev command timed out
// -%EIO       if the sev returned a non-zero return code
//
extern "C" {
    pub fn sev_guest_deactivate(data: *mut sev_data_deactivate, error: *mut c_int) -> c_int;
}
//
// sev_guest_activate - perform SEV ACTIVATE command
//
// @activate: sev_data_activate structure to be processed
// @sev_ret: sev command return code
//
// Returns:
// 0 if the sev successfully processed the command
// -%ENODEV    if the sev device is not available
// -%ENOTSUPP  if the sev does not support SEV
// -%ETIMEDOUT if the sev command timed out
// -%EIO       if the sev returned a non-zero return code
//
extern "C" {
    pub fn sev_guest_activate(data: *mut sev_data_activate, error: *mut c_int) -> c_int;
}
//
// sev_guest_df_flush - perform SEV DF_FLUSH command
//
// @sev_ret: sev command return code
//
// Returns:
// 0 if the sev successfully processed the command
// -%ENODEV    if the sev device is not available
// -%ENOTSUPP  if the sev does not support SEV
// -%ETIMEDOUT if the sev command timed out
// -%EIO       if the sev returned a non-zero return code
//
extern "C" {
    pub fn sev_guest_df_flush(error: *mut c_int) -> c_int;
}
//
// sev_guest_decommission - perform SEV DECOMMISSION command
//
// @decommission: sev_data_decommission structure to be processed
// @sev_ret: sev command return code
//
// Returns:
// 0 if the sev successfully processed the command
// -%ENODEV    if the sev device is not available
// -%ENOTSUPP  if the sev does not support SEV
// -%ETIMEDOUT if the sev command timed out
// -%EIO       if the sev returned a non-zero return code
//
extern "C" {
    pub fn sev_guest_decommission(data: *mut sev_data_decommission, error: *mut c_int) -> c_int;
}
//
// sev_do_cmd - issue an SEV or an SEV-SNP command
//
// @cmd: SEV or SEV-SNP firmware command to issue
// @data: arguments for firmware command
// @psp_ret: SEV command return code
//
// Returns:
// 0 if the SEV device successfully processed the command
// -%ENODEV    if the PSP device is not available
// -%ENOTSUPP  if PSP device does not support SEV
// -%ETIMEDOUT if the SEV command timed out
// -%EIO       if PSP device returned a non-zero return code
//
extern "C" {
    pub fn sev_do_cmd(cmd: c_int, data: *mut c_void, psp_ret: *mut c_int) -> c_int;
}
extern "C" {
    pub fn snp_reclaim_pages(paddr: c_ulong, npages: c_uint, locked: bool) -> c_int;
}
extern "C" {
    pub fn snp_free_firmware_page(addr: *mut c_void);
}
extern "C" {
    pub fn sev_platform_shutdown();
}
extern "C" {
    pub fn sev_is_snp_ciphertext_hiding_supported() -> bool;
}
extern "C" {
    pub fn sev_get_snp_policy_bits() -> u64;
}
extern "C" {
    pub fn sev_firmware_supported_vm_types() -> c_int;
}

