//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/tcm_qla2xxx.h
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

// length of ASCII WWPNs including pad
pub const TCM_QLA2XXX_NAMELEN: c_int = 32;
//
// Number of pre-allocated per-session tags, based upon the worst-case
// per port number of iocbs
//
pub const TCM_QLA2XXX_DEFAULT_TAGS: c_int = 2088;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_qla2xxx_nacl {
    pub se_node_acl: se_node_acl,
// From libfc struct fc_rport->port_id
    pub nport_id: u32,
// Binary World Wide unique Node Name for remote FC Initiator Nport
    pub nport_wwnn: u64,
// ASCII formatted WWPN for FC Initiator Nport
    pub nport_name: [c_char; TCM_QLA2XXX_NAMELEN],
// Pointer to fc_port
    pub fc_port: *mut fc_port,
// Pointer to TCM FC nexus
    pub nport_nexus: *mut se_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_qla2xxx_tpg_attrib {
    pub generate_node_acls: c_int,
    pub cache_dynamic_acls: c_int,
    pub demo_mode_write_protect: c_int,
    pub prod_mode_write_protect: c_int,
    pub demo_mode_login_only: c_int,
    pub fabric_prot_type: c_int,
    pub jam_host: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_qla2xxx_tpg {
// FC lport target portal group tag for TCM
    pub lport_tpgt: u16,
// Atomic bit to determine TPG active status
    pub lport_tpg_enabled: core::sync::atomic::AtomicI32,
// Pointer back to tcm_qla2xxx_lport
    pub lport: *mut tcm_qla2xxx_lport,
// Used by tcm_qla2xxx_tpg_attrib_cit
    pub tpg_attrib: tcm_qla2xxx_tpg_attrib,
// Returned by tcm_qla2xxx_make_tpg()
    pub se_tpg: se_portal_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_qla2xxx_fc_loopid {
    pub se_nacl: *mut se_node_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_qla2xxx_lport {
// Binary World Wide unique Port Name for FC Target Lport
    pub lport_wwpn: u64,
// Binary World Wide unique Port Name for FC NPIV Target Lport
    pub lport_npiv_wwpn: u64,
// Binary World Wide unique Node Name for FC NPIV Target Lport
    pub lport_npiv_wwnn: u64,
// ASCII formatted WWPN for FC Target Lport
    pub lport_name: [c_char; TCM_QLA2XXX_NAMELEN],
// ASCII formatted naa WWPN for VPD page 83 etc
    pub lport_naa_name: [c_char; TCM_QLA2XXX_NAMELEN],
// map for fc_port pointers in 24-bit FC Port ID space
    pub lport_fcport_map: btree_head32,
// vmalloc-ed memory for fc_port pointers for 16-bit FC loop ID
    pub lport_loopid_map: *mut tcm_qla2xxx_fc_loopid,
// Pointer to struct scsi_qla_host from qla2xxx LLD
    pub qla_vha: *mut scsi_qla_host,
// Pointer to struct qla_tgt pointer
    pub lport_qla_tgt: qla_tgt,
// Pointer to TPG=1 for non NPIV mode
    pub tpg_1: *mut tcm_qla2xxx_tpg,
// Returned by tcm_qla2xxx_make_lport()
    pub lport_wwn: se_wwn,
}
