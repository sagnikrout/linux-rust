//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/loopback/tcm_loop.h
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

pub const TL_WWN_ADDR_LEN: c_int = 256;
pub const TL_TPGS_PER_HBA: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_loop_cmd {
// State of Linux/SCSI CDB+Data descriptor
    pub sc_cmd_state: u32,
// Tagged command queueing
    pub sc_cmd_tag: u32,
// Pointer to the CDB+Data descriptor from Linux/SCSI subsystem
    pub sc: *mut scsi_cmnd,
// The TCM I/O descriptor that is accessed via container_of()
    pub tl_se_cmd: se_cmd,
    pub tmr_done: completion,
// Sense buffer that will be mapped into outgoing status
    pub tl_sense_buf: [c_uchar; TRANSPORT_SENSE_BUFFER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_loop_nexus {
//
// Pointer to TCM session for I_T Nexus
//
    pub se_sess: *mut se_session,
}

pub const TCM_TRANSPORT_ONLINE: c_int = 0;
pub const TCM_TRANSPORT_OFFLINE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_loop_tpg {
    pub tl_tpgt: c_ushort,
    pub tl_transport_status: c_ushort,
    pub tl_fabric_prot_type: target_prot_type,
    pub tl_tpg_port_count: core::sync::atomic::AtomicI32,
    pub tl_se_tpg: se_portal_group,
    pub tl_hba: *mut tcm_loop_hba,
    pub tl_nexus: *mut tcm_loop_nexus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_loop_hba {
    pub tl_proto_id: u8,
    pub tl_wwn_address: [c_uchar; TL_WWN_ADDR_LEN],
    pub se_hba: *mut se_hba_s,
    pub tl_hba_lun: *mut se_lun,
    pub tl_hba_lun_sep: *mut se_port,
    pub dev: device,
    pub sh: *mut Scsi_Host,
    pub tl_hba_tpgs: [tcm_loop_tpg; TL_TPGS_PER_HBA],
    pub tl_hba_wwn: se_wwn,
}
