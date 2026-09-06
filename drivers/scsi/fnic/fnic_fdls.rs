//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic_fdls.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

// FDLS - Fabric discovery and login services
// -> VLAN discovery
// -> retry every retry delay seconds until it succeeds.
// <- List of VLANs
//
// -> Solicitation
// <- Solicitation response (Advertisement)
//
// -> FCF selection & FLOGI ( FLOGI timeout - 2 * E_D_TOV)
// <- FLOGI response
//
// -> FCF keep alive
// <- FCF keep alive
//
// -> PLOGI to FFFFFC (DNS) (PLOGI timeout - 2 * R_A_TOV)
// -> ABTS if timeout (ABTS tomeout - 2 * R_A_TOV)
// <- PLOGI response
// -> Retry PLOGI to FFFFFC (DNS) - Number of retries from vnic.cfg
//
// -> SCR to FFFFFC (DNS) (SCR timeout - 2 * R_A_TOV)
// -> ABTS if timeout (ABTS tomeout - 2 * R_A_TOV)
// <- SCR response
// -> Retry SCR - Number of retries 2
//
// -> GPN_FT to FFFFFC (GPN_FT timeout - 2 * R_A_TOV)a
// -> Retry on BUSY until it succeeds
// -> 2 retries on timeout
//
// -> RFT_ID to FFFFFC (DNS)        (RFT_ID timeout - 3 * R_A_TOV)
// -> ABTS if timeout (ABTS tomeout - 2 * R_A_TOV)
// -> Retry RFT_ID to FFFFFC (DNS) (Number of retries 2 )
// -> Ignore if both retires fail.
//
// Session establishment with targets
// For each PWWN
// -> PLOGI to FCID of that PWWN (PLOGI timeout 2 * R_A_TOV)
// -> ABTS if timeout (ABTS tomeout - 2 * R_A_TOV)
// <- PLOGI response
// -> Retry PLOGI. Num retries using vnic.cfg
//
// -> PRLI to FCID of that PWWN (PRLI timeout 2 * R_A_TOV)
// -> ABTS if timeout (ABTS tomeout - 2 * R_A_TOV)
// <- PRLI response
// -> Retry PRLI. Num retries using vnic.cfg
//
pub const FDLS_RETRY_COUNT: c_int = 2;
//
// OXID encoding:
// bits 0-8: oxid idx - allocated from poool
// bits 9-13: oxid frame code from fnic_oxid_frame_type_e
// bits 14-15: all zeros
//

pub const FNIC_FRAME_MASK: c_uint = 0xFE00;

pub const FNIC_FDLS_FABRIC_ABORT_ISSUED: c_uint = 0x1;
pub const FNIC_FDLS_FPMA_LEARNT: c_uint = 0x2;
// tport flags
pub const FNIC_FDLS_TPORT_IN_GPN_FT_LIST: c_uint = 0x1;
pub const FNIC_FDLS_TGT_ABORT_ISSUED: c_uint = 0x2;
pub const FNIC_FDLS_TPORT_SEND_ADISC: c_uint = 0x4;
pub const FNIC_FDLS_RETRY_FRAME: c_uint = 0x8;
pub const FNIC_FDLS_TPORT_BUSY: c_uint = 0x10;
pub const FNIC_FDLS_TPORT_TERMINATING: c_uint = 0x20;
pub const FNIC_FDLS_TPORT_DELETED: c_uint = 0x40;
pub const FNIC_FDLS_NVME_REGISTERED: c_uint = 0x80;
pub const FNIC_FDLS_NVME_TPORT_CLEANUP_PENDING: c_uint = 0x100;
pub const FNIC_FDLS_SCSI_REGISTERED: c_uint = 0x200;
pub const FNIC_TPORT_CAN_BE_FREED: c_uint = 0x400;
// Retry supported by rport(returned by prli service parameters)
pub const FDLS_FC_RP_FLAGS_RETRY: c_uint = 0x1;

pub const FNIC_FDMI_ACTIVE: c_uint = 0x8;
pub const FNIC_LPORT_NVME_REGISTERED: c_uint = 0x4;
pub const FNIC_FIRST_LINK_UP: c_uint = 0x2;

pub const FNIC_PORTSPEED_10GBIT: c_int = 1;

pub const FNIC_FRAME_TYPE_FABRIC_FLOGI: c_uint = 0x1000;
pub const FNIC_FRAME_TYPE_FABRIC_PLOGI: c_uint = 0x1200;
pub const FNIC_FRAME_TYPE_FABRIC_RPN: c_uint = 0x1400;
pub const FNIC_FRAME_TYPE_FABRIC_RFT: c_uint = 0x1600;
pub const FNIC_FRAME_TYPE_FABRIC_RFF: c_uint = 0x1800;
pub const FNIC_FRAME_TYPE_FABRIC_SCR: c_uint = 0x1A00;
pub const FNIC_FRAME_TYPE_FABRIC_GPN_FT: c_uint = 0x1C00;
pub const FNIC_FRAME_TYPE_FABRIC_LOGO: c_uint = 0x1E00;
pub const FNIC_FRAME_TYPE_FDMI_PLOGI: c_uint = 0x2000;
pub const FNIC_FRAME_TYPE_FDMI_RHBA: c_uint = 0x2200;
pub const FNIC_FRAME_TYPE_FDMI_RPA: c_uint = 0x2400;
pub const FNIC_FRAME_TYPE_TGT_PLOGI: c_uint = 0x2600;
pub const FNIC_FRAME_TYPE_TGT_PRLI: c_uint = 0x2800;
pub const FNIC_FRAME_TYPE_TGT_ADISC: c_uint = 0x2A00;
pub const FNIC_FRAME_TYPE_TGT_LOGO: c_uint = 0x2C00;
pub const FNIC_FRAME_TYPE_NVME_LS: c_uint = 0x3000;

pub const FNIC_LPORT_NVME_REGISTERED: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_fip_fcf_s {
    pub vlan_id: u16,
    pub fcf_mac: [u8; 6],
    pub fcf_priority: u8,
    pub fka_adv_period: u32,
    pub ka_disabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_fdls_state_e {
    FDLS_STATE_INIT = 0,
    FDLS_STATE_LINKDOWN,
    FDLS_STATE_FABRIC_LOGO,
    FDLS_STATE_FLOGO_DONE,
    FDLS_STATE_FABRIC_FLOGI,
    FDLS_STATE_FABRIC_PLOGI,
    FDLS_STATE_RPN_ID,
    FDLS_STATE_REGISTER_FC4_TYPES,
    FDLS_STATE_REGISTER_FC4_FEATURES,
    FDLS_STATE_SCR,
    FDLS_STATE_GPN_FT,
    FDLS_STATE_TGT_DISCOVERY,
    FDLS_STATE_RSCN_GPN_FT,
    FDLS_STATE_SEND_GPNFT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_fdls_fabric_s {
    pub state: fnic_fdls_state_e,
    pub flags: u32,
    pub /: *mut *mut list_head tport_list; / List of discovered tports,
    pub retry_timer: timer_list,
    pub del_timer_inprogress: c_int,
    pub del_fdmi_timer_inprogress: c_int,
    pub retry_counter: c_int,
    pub timer_pending: c_int,
    pub fdmi_retry: c_int,
    pub fdmi_timer: timer_list,
    pub fdmi_pending: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_fdls_fip_s {
    pub state: u32,
    pub flogi_retry: u32,
}

// Message to tport_event_handler
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_tgt_msg_id {
    TGT_EV_NONE = 0,
    TGT_EV_RPORT_ADD,
    TGT_EV_RPORT_DEL,
    TGT_EV_TPORT_DELETE,
    TGT_EV_REMOVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_tport_event_s {
    pub links: list_head,
    pub event: fnic_tgt_msg_id,
    pub arg1: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdls_tgt_state_e {
    FDLS_TGT_STATE_INIT = 0,
    FDLS_TGT_STATE_PLOGI,
    FDLS_TGT_STATE_PRLI,
    FDLS_TGT_STATE_READY,
    FDLS_TGT_STATE_LOGO_RECEIVED,
    FDLS_TGT_STATE_ADISC,
    FDL_TGT_STATE_PLOGO,
    FDLS_TGT_STATE_OFFLINING,
    FDLS_TGT_STATE_OFFLINE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_tport_s {
    pub /: *mut *mut list_head links; / To link the tports,
    pub state: fdls_tgt_state_e,
    pub flags: u32,
    pub fcid: u32,
    pub wwpn: u64,
    pub wwnn: u64,
    pub active_oxid: u16,
    pub tgt_flags: u16,
    pub /: *mut *mut atomic_t in_flight; / io counter,
    pub max_payload_size: u16,
    pub r_a_tov: u16,
    pub e_d_tov: u16,
    pub lun0_delay: u16,
    pub max_concur_seqs: c_int,
    pub fcp_csp: u32,
    pub retry_timer: timer_list,
    pub del_timer_inprogress: c_int,
    pub retry_counter: c_int,
    pub timer_pending: c_int,
    pub num_pending_cmds: c_uint,
    pub nexus_restart_count: c_int,
    pub exch_reset_in_progress: c_int,
    pub iport: *mut c_void,
    pub tport_del_work: work_struct,
    pub tport_del_done: *mut completion,
    pub rport: *mut fc_rport,
    pub str_wwpn: [c_char; 20],
    pub str_wwnn: [c_char; 20],
    pub ls_req_list: list_head,
    pub nv_rport: *mut nvme_fc_remote_port,
}

// OXID pool related structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reclaim_entry_s {
    pub links: list_head,
// oxid that needs to be freed after 2*r_a_tov
    pub oxid_idx: u16,
// in jiffies. Use this to waiting time
    pub expires: c_ulong,
    pub bitmap: *mut c_ulong,
}

// used for allocating oxids for fabric and fdmi requests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_oxid_pool_s {
    pub FNIC_OXID_POOL_SZ): DECLARE_BITMAP(bitmap,,
    pub /: *mut *mut int sz; / size of the pool or block,
    pub /: *mut *mut int next_idx; / used for cycling through the oxid pool,
// retry schedule free
    pub FNIC_OXID_POOL_SZ): DECLARE_BITMAP(pending_schedule_free,,
    pub schedule_oxid_free_retry: delayed_work,
// List of oxids that need to be freed and reclaimed.
// This list is shared by all the oxid pools
//
    pub oxid_reclaim_list: list_head,
// Work associated with reclaim list
    pub oxid_reclaim_work: delayed_work,
}

// iport
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_iport_state_e {
    FNIC_IPORT_STATE_INIT = 0,
    FNIC_IPORT_STATE_LINK_WAIT,
    FNIC_IPORT_STATE_FIP,
    FNIC_IPORT_STATE_FABRIC_DISC,
    FNIC_IPORT_STATE_READY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_iport_s {
    pub state: fnic_iport_state_e,
    pub fnic: *mut fnic,
    pub boot_time: u64,
    pub flags: u32,
    pub usefip: c_int,
    pub /: *mut *mut uint8_t hwmac[6]; / HW MAC Addr,
    pub /: *mut *mut uint8_t fpma[6]; / Fabric Provided MA,
    pub /: *mut *mut uint8_t fcfmac[6]; / MAC addr of Fabric,
    pub vlan_id: u16,
    pub fcid: u32,
// oxid pool
    pub oxid_pool: fnic_oxid_pool_s,
//
// fabric reqs are serialized and only one req at a time.
// Tracking the oxid for sending abort
//
    pub active_oxid_fabric_req: u16,
// fdmi only
    pub active_oxid_fdmi_plogi: u16,
    pub active_oxid_fdmi_rhba: u16,
    pub active_oxid_fdmi_rpa: u16,
    pub selected_fcf: fnic_fip_fcf_s,
    pub fip: fnic_fdls_fip_s,
    pub fabric: fnic_fdls_fabric_s,
    pub tport_list: list_head,
// list of tports for which we are yet to send PLOGO
    pub inprocess_tport_list: list_head,
    pub deleted_tport_list: list_head,
    pub tport_event_work: work_struct,
    pub /: *mut *mut uint32_t e_d_tov; / msec,
    pub /: *mut *mut uint32_t r_a_tov; / msec,
    pub link_supported_speeds: u32,
    pub max_flogi_retries: u32,
    pub max_plogi_retries: u32,
    pub plogi_timeout: u32,
    pub service_params: u32,
    pub wwpn: u64,
    pub wwnn: u64,
    pub max_payload_size: u16,
    pub deleted_tport_lst_lock: spinlock_t,
    pub flogi_reg_done: *mut completion,
    pub iport_stats: fnic_iport_stats,
    pub str_wwpn: [c_char; 20],
    pub str_wwnn: [c_char; 20],
// nvme */;
    pub nvfnic_fcpio_tag: [*mut c_void; NVFNIC_FCPIO_TAG_POOL_SZ],
    pub nv_lport: *mut nvme_fc_local_port,
    pub nv_tmpl: *mut nvme_fc_port_template,
    pub ls_req_oxid_pool: fnic_oxid_pool_s,
    pub fcpio_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rport_dd_data_s {
    pub tport: *mut fnic_tport_s,
    pub iport: *mut fnic_iport_s,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_recv_frame_type_e {
    FNIC_FABRIC_FLOGI_RSP = 1,
    FNIC_FABRIC_PLOGI_RSP,
    FNIC_FABRIC_RPN_RSP,
    FNIC_FABRIC_RFT_RSP,
    FNIC_FABRIC_RFF_RSP,
    FNIC_FABRIC_SCR_RSP,
    FNIC_FABRIC_GPN_FT_RSP,
    FNIC_FABRIC_BLS_ABTS_RSP,
    FNIC_FDMI_PLOGI_RSP,
    FNIC_FDMI_REG_HBA_RSP,
    FNIC_FDMI_RPA_RSP,
    FNIC_FDMI_BLS_ABTS_RSP,
    FNIC_FABRIC_LOGO_RSP,

// responses to target requests
    FNIC_TPORT_PLOGI_RSP,
    FNIC_TPORT_PRLI_RSP,
    FNIC_TPORT_ADISC_RSP,
    FNIC_TPORT_BLS_ABTS_RSP,
    FNIC_TPORT_LOGO_RSP,
    FNIC_LS_REQ_ABTS_RSP,

// unsolicited requests
    FNIC_BLS_ABTS_REQ,
    FNIC_ELS_PLOGI_REQ,
    FNIC_ELS_RSCN_REQ,
    FNIC_ELS_LOGO_REQ,
    FNIC_ELS_ECHO_REQ,
    FNIC_ELS_ADISC,
    FNIC_ELS_RLS,
    FNIC_ELS_RRQ,
    FNIC_ELS_UNSUPPORTED_REQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_port_speeds {
    DCEM_PORTSPEED_NONE = 0,
    DCEM_PORTSPEED_1G = 1000,
    DCEM_PORTSPEED_2G = 2000,
    DCEM_PORTSPEED_4G = 4000,
    DCEM_PORTSPEED_8G = 8000,
    DCEM_PORTSPEED_10G = 10000,
    DCEM_PORTSPEED_16G = 16000,
    DCEM_PORTSPEED_20G = 20000,
    DCEM_PORTSPEED_25G = 25000,
    DCEM_PORTSPEED_32G = 32000,
    DCEM_PORTSPEED_40G = 40000,
    DCEM_PORTSPEED_4x10G = 41000,
    DCEM_PORTSPEED_50G = 50000,
    DCEM_PORTSPEED_64G = 64000,
    DCEM_PORTSPEED_100G = 100000,
    DCEM_PORTSPEED_128G = 128000,
}

// Function Declarations
// fdls_disc.c
extern "C" {
    pub fn fnic_fdls_disc_init(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn fnic_fdls_disc_start(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn fnic_fdls_link_down(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn fdls_init_frame_pool(iport: *mut fnic_iport_s) -> c_int;
}
extern "C" {
    pub fn fnic_del_fabric_timer_sync(fnic: *mut fnic);
}
extern "C" {
    pub fn fdls_send_fabric_logo(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn fdls_fdmi_timer_callback(t: *mut timer_list);
}
extern "C" {
    pub fn fdls_fdmi_retry_plogi(iport: *mut fnic_iport_s);
}
// fnic_fcs.c
extern "C" {
    pub fn fnic_fdls_init(fnic: *mut fnic, usefip: c_int);
}
extern "C" {
    pub fn fnic_fcoe_send_vlan_req(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_fcpio_reset(fnic: *mut fnic);
}
// fip.c
extern "C" {
    pub fn fnic_fcoe_send_vlan_req(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_common_fip_cleanup(fnic: *mut fnic);
}
extern "C" {
    pub fn fdls_fip_recv_frame(fnic: *mut fnic, frame: *mut c_void) -> c_int;
}
extern "C" {
    pub fn fnic_handle_fcs_ka_timer(t: *mut timer_list);
}
extern "C" {
    pub fn fnic_handle_enode_ka_timer(t: *mut timer_list);
}
extern "C" {
    pub fn fnic_handle_vn_ka_timer(t: *mut timer_list);
}
extern "C" {
    pub fn fnic_handle_fip_timer(t: *mut timer_list);
}
extern "C" {
    pub fn fdls_fabric_timer_callback(t: *mut timer_list);
}
// fnic_scsi.c
extern "C" {
    pub fn fdls_fabric_timer_callback(t: *mut timer_list);
}
extern "C" {
    pub fn fnic_rport_exch_reset(fnic: *mut fnic, fcid: u32);
}
