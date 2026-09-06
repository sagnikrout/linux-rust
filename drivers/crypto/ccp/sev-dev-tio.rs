//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/sev-dev-tio.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sla_addr_t {
    pub sla: u64,
    pub :12: reserved2,
}

pub const SEV_TIO_MAX_COMMAND_LENGTH: c_int = 128;
// SPDM control structure for DOE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_spdm {
    pub req_len: c_ulong,
    pub req: *mut c_void,
    pub rsp_len: c_ulong,
    pub rsp: *mut c_void,
}

// Describes TIO device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_dsm_tio {
    pub cert_slot: u8,
    pub dev_ctx: sla_addr_t,
    pub req: sla_addr_t,
    pub resp: sla_addr_t,
    pub scratch: sla_addr_t,
    pub output: sla_addr_t,
    pub output_len: usize,
    pub scratch_len: usize,
    pub spdm: tsm_spdm,
    pub /: *mut *mut *mut sla_buffer_hdr reqbuf; / vmap'ed @req for DOE,
    pub /: *mut *mut *mut sla_buffer_hdr respbuf; / vmap'ed @resp for DOE,
    pub cmd: c_int,
    pub psp_ret: c_int,
    pub cmd_data: [u8; SEV_TIO_MAX_COMMAND_LENGTH],
    pub /: *mut *mut *mut void data_pg; / Data page for DEV_STATUS/TDI_STATUS/TDI_INFO/ASID_FENCE,
pub const TIO_IDE_MAX_TC: c_int = 8;
    pub ide: [*mut pci_ide; TIO_IDE_MAX_TC],
}

// Describes TSM structure for PF0 pointed by pci_dev->tsm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tio_dsm {
    pub tsm: pci_tsm_pf0,
    pub data: tsm_dsm_tio,
    pub sev: *mut sev_device,
}

// Data object IDs
pub const SPDM_DOBJ_ID_NONE: c_int = 0;
pub const SPDM_DOBJ_ID_REQ: c_int = 1;
pub const SPDM_DOBJ_ID_RESP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spdm_dobj_hdr {
    pub /: *mut *mut u32 id; / Data object type identifier,
    pub /: *mut *mut u32 length; / Length of the data object, INCLUDING THIS HEADER,
    pub minor: u8,
    pub major: u8,
    pub version: },
    pub __packed: },
//
// struct sev_tio_status - TIO_STATUS command's info_paddr buffer
//
// @length: Length of this structure in bytes
// @tio_en: Indicates that SNP_INIT_EX initialized the RMP for SEV-TIO
// @tio_init_done: Indicates TIO_INIT has been invoked
// @spdm_req_size_min: Minimum SPDM request buffer size in bytes
// @spdm_req_size_max: Maximum SPDM request buffer size in bytes
// @spdm_scratch_size_min: Minimum SPDM scratch buffer size in bytes
// @spdm_scratch_size_max: Maximum SPDM scratch buffer size in bytes
// @spdm_out_size_min: Minimum SPDM output buffer size in bytes
// @spdm_out_size_max: Maximum for the SPDM output buffer size in bytes
// @spdm_rsp_size_min: Minimum SPDM response buffer size in bytes
// @spdm_rsp_size_max: Maximum SPDM response buffer size in bytes
// @devctx_size: Size of a device context buffer in bytes
// @tdictx_size: Size of a TDI context buffer in bytes
// @tio_crypto_alg: TIO crypto algorithms supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_tio_status {
    pub length: u32,
    pub :30: reserved,
    pub spdm_req_size_min: u32,
    pub spdm_req_size_max: u32,
    pub spdm_scratch_size_min: u32,
    pub spdm_scratch_size_max: u32,
    pub spdm_out_size_min: u32,
    pub spdm_out_size_max: u32,
    pub spdm_rsp_size_min: u32,
    pub spdm_rsp_size_max: u32,
    pub devctx_size: u32,
    pub tdictx_size: u32,
    pub tio_crypto_alg: u32,
    pub reserved2: [u8; 12],
    pub __packed: },
    pub tio_status_page): *mut int sev_tio_init_locked(void,
    pub dev_data): *mut int sev_tio_continue(struct tsm_dsm_tio,
    pub segment_id): u8,
    pub cert_slot): *mut *mut int sev_tio_dev_connect(struct tsm_dsm_tio dev_data, u8 tc_mask, u8 ids[8], u8,
    pub force): *mut *mut int sev_tio_dev_disconnect(struct tsm_dsm_tio dev_data, bool,
    pub dev_data): *mut int sev_tio_dev_reclaim(struct tsm_dsm_tio,
