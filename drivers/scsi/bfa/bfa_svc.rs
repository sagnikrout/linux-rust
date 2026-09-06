//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_svc.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

//
// Scatter-gather DMA related defines
//

//
// Alignment macro for SG page allocation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sgpg_wqe_s {
    pub /: *mut *mut list_head qe; / queue sg page element,
    pub /: *mut *mut int nsgpg; / pages to be allocated,
    pub /: *mut *mut int nsgpg_total; / total pages required,
    pub /: *mut *mut *mut *mut void (cbfn) (void cbarg); / callback function,
    pub /: *mut *mut *mut void cbarg; / callback arg,
    pub /: *mut *mut list_head sgpg_q; / queue of alloced sgpgs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sgpg_s {
    pub /: *mut *mut list_head qe; / queue sg page element,
    pub /: *mut *mut *mut bfi_sgpg_s sgpg; / va of SG page,
    pub /: *mut *mut bfi_addr_u sgpg_pa; / pa of SG page,
}

//
// Given number of SG elements, BFA_SGPG_NPAGE() returns the number of
// SG pages required.
//

// Max SGPG dma segs required

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sgpg_mod_s {
    pub bfa: *mut bfa_s,
    pub /: *mut *mut int num_sgpgs; / number of SG pages,
    pub /: *mut *mut int free_sgpgs; / number of free SG pages,
    pub /: *mut *mut list_head sgpg_q; / queue of free SG pages,
    pub /: *mut *mut list_head sgpg_wait_q; / wait queue for SG pages,
    pub dma_seg: [bfa_mem_dma_s; BFA_SGPG_DMA_SEGS],
    pub kva_seg: bfa_mem_kva_s,
}

extern "C" {
    pub fn bfa_sgpg_mfree(bfa: *mut bfa_s, sgpg_q: *mut list_head, nsgpgs: c_int);
}
extern "C" {
    pub fn bfa_sgpg_wait(bfa: *mut bfa_s, wqe: *mut bfa_sgpg_wqe_s, nsgpgs: c_int);
}
extern "C" {
    pub fn bfa_sgpg_wcancel(bfa: *mut bfa_s, wqe: *mut bfa_sgpg_wqe_s);
}
//
// FCXP related defines
//

// Max FCXP dma segs required

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcxp_mod_s {
    pub /: *mut *mut *mut bfa_s bfa; / backpointer to BFA,
    pub /: *mut *mut *mut bfa_fcxp_s fcxp_list; / array of FCXPs,
    pub /: *mut *mut u16 num_fcxps; / max num FCXP requests,
    pub /: *mut *mut list_head fcxp_req_free_q; / free FCXPs used for sending req,
    pub /: *mut *mut list_head fcxp_rsp_free_q; / free FCXPs used for sending req,
    pub /: *mut *mut list_head fcxp_active_q; / active FCXPs,
    pub /: *mut *mut list_head req_wait_q; / wait queue for free req_fcxp,
    pub /: *mut *mut list_head rsp_wait_q; / wait queue for free rsp_fcxp,
    pub /: *mut *mut list_head fcxp_req_unused_q; / unused req_fcxps,
    pub /: *mut *mut list_head fcxp_rsp_unused_q; / unused rsp_fcxps,
    pub req_pld_sz: u32,
    pub rsp_pld_sz: u32,
    pub dma_seg: [bfa_mem_dma_s; BFA_FCXP_DMA_SEGS],
    pub kva_seg: bfa_mem_kva_s,
}

extern "C" {
    pub fn u64(bfad_fcxp: *mut *mut bfa_fcxp_get_sgaddr_t) (void, sgeid: c_int) -> typedef;
}
extern "C" {
    pub fn u32(bfad_fcxp: *mut *mut bfa_fcxp_get_sglen_t) (void, sgeid: c_int) -> typedef;
}
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_fcxp_alloc_cbfn_t) (void, fcxp: *mut bfa_fcxp_s) -> typedef;
}
//
// Information needed for a FCXP request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcxp_req_info_s {
    pub bfa_rport: *mut bfa_rport_s,
// Pointer to the bfa rport that was
// returned from bfa_rport_create().
// This could be left NULL for WKA or
// for FCXP interactions before the
// rport nexus is established
//
    pub /: *mut *mut fchs_s fchs; / request FC header structure,
    pub /: *mut *mut u8 cts; / continuous sequence,
    pub /: *mut *mut u8 class; / FC class for the request/response,
    pub /: *mut *mut u16 max_frmsz; / max send frame size,
    pub /: *mut *mut u16 vf_id; / vsan tag if applicable,
    pub /: *mut *mut u8 lp_tag; / lport tag,
    pub /: *mut *mut u32 req_tot_len; / request payload total length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcxp_rsp_info_s {
    pub rsp_fchs: fchs_s,
// Response frame's FC header will
// be sent back in this field
    pub rsp_timeout: u8,
// timeout in seconds, 0-no response
    pub rsvd2: [u8; 3],
    pub /: *mut *mut u32 rsp_maxlen; / max response length expected,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcxp_s {
    pub /: *mut *mut list_head qe; / fcxp queue element,
    pub /: *mut *mut bfa_sm_t sm; / state machine,
    pub /: *mut *mut *mut void caller; / driver or fcs,
    pub fcxp_mod: *mut bfa_fcxp_mod_s,
// back pointer to fcxp mod
    pub /: *mut *mut u16 fcxp_tag; / internal tag,
    pub req_info: bfa_fcxp_req_info_s,
// request info
    pub rsp_info: bfa_fcxp_rsp_info_s,
// response info
    pub /: *mut *mut u8 use_ireqbuf; / use internal req buf,
    pub /: *mut *mut u8 use_irspbuf; / use internal rsp buf,
    pub /: *mut *mut u32 nreq_sgles; / num request SGLEs,
    pub /: *mut *mut u32 nrsp_sgles; / num response SGLEs,
    pub /: *mut *mut list_head req_sgpg_q; / SG pages for request buf,
    pub /: *mut *mut list_head req_sgpg_wqe; / wait queue for req SG page,
    pub /: *mut *mut list_head rsp_sgpg_q; / SG pages for response buf,
    pub /: *mut *mut list_head rsp_sgpg_wqe; / wait queue for rsp SG page,
    pub req_sga_cbfn: bfa_fcxp_get_sgaddr_t,
// SG elem addr user function
    pub req_sglen_cbfn: bfa_fcxp_get_sglen_t,
// SG elem len user function
    pub rsp_sga_cbfn: bfa_fcxp_get_sgaddr_t,
// SG elem addr user function
    pub rsp_sglen_cbfn: bfa_fcxp_get_sglen_t,
// SG elem len user function
    pub /: *mut *mut bfa_cb_fcxp_send_t send_cbfn; / send completion callback,
    pub /: *mut *mut *mut void send_cbarg; / callback arg,
    pub req_sge: [bfa_sge_s; BFA_FCXP_MAX_SGES],
// req SG elems
    pub rsp_sge: [bfa_sge_s; BFA_FCXP_MAX_SGES],
// rsp SG elems
    pub /: *mut *mut u8 rsp_status; / comp: rsp status,
    pub /: *mut *mut u32 rsp_len; / comp: actual response len,
    pub /: *mut *mut u32 residue_len; / comp: residual rsp length,
    pub /: *mut *mut fchs_s rsp_fchs; / comp: response fchs,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: callback qelem,
    pub reqq_wqe: bfa_reqq_wait_s,
    pub reqq_waiting: bfa_boolean_t,
    pub /: *mut *mut bfa_boolean_t req_rsp; / Used to track req/rsp fcxp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcxp_wqe_s {
    pub qe: list_head,
    pub alloc_cbfn: bfa_fcxp_alloc_cbfn_t,
    pub alloc_cbarg: *mut c_void,
    pub caller: *mut c_void,
    pub bfa: *mut bfa_s,
    pub nreq_sgles: c_int,
    pub nrsp_sgles: c_int,
    pub req_sga_cbfn: bfa_fcxp_get_sgaddr_t,
    pub req_sglen_cbfn: bfa_fcxp_get_sglen_t,
    pub rsp_sga_cbfn: bfa_fcxp_get_sgaddr_t,
    pub rsp_sglen_cbfn: bfa_fcxp_get_sglen_t,
}

// fcxp_buf = req_buf + rsp_buf :- add req_buf_sz to get to rsp_buf

extern "C" {
    pub fn bfa_fcxp_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
pub const BFA_RPORT_MIN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_mod_s {
    pub /: *mut *mut *mut bfa_rport_s rps_list; / list of rports,
    pub /: *mut *mut list_head rp_free_q; / free bfa_rports,
    pub /: *mut *mut list_head rp_active_q; / free bfa_rports,
    pub /: *mut *mut list_head rp_unused_q; / unused bfa rports,
    pub /: *mut *mut u16 num_rports; / number of rports,
    pub kva_seg: bfa_mem_kva_s,
}

//
// Convert rport tag to RPORT
//

//
// protected functions
//
extern "C" {
    pub fn bfa_rport_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_rport_res_recfg(bfa: *mut bfa_s, num_rport_fw: u16);
}
//
// BFA rport information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_info_s {
    pub /: *mut *mut u16 max_frmsz; / max rcv pdu size,
    pub /: *mut *mut lp_tag:8; / tag,
    pub /: *mut *mut cisc:8; / CIRO supported,
    pub /: *mut *mut u8 fc_class; / supported FC classes. enum fc_cos,
    pub /: *mut *mut u8 vf_en; / virtual fabric enable,
    pub /: *mut *mut u16 vf_id; / virtual fabric ID,
    pub /: *mut *mut bfa_port_speed speed; / Rport's current speed,
}

//
// RPORT related defines
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_rport_event {
    BFA_RPORT_SM_CREATE	= 1,	/*  rport create event          */
    BFA_RPORT_SM_DELETE	= 2,	/*  deleting an existing rport  */
    BFA_RPORT_SM_ONLINE	= 3,	/*  rport is online             */
    BFA_RPORT_SM_OFFLINE	= 4,	/*  rport is offline            */
    BFA_RPORT_SM_FWRSP	= 5,	/*  firmware response           */
    BFA_RPORT_SM_HWFAIL	= 6,	/*  IOC h/w failure             */
    BFA_RPORT_SM_QOS_SCN	= 7,	/*  QoS SCN from firmware       */
    BFA_RPORT_SM_SET_SPEED	= 8,	/*  Set Rport Speed             */
    BFA_RPORT_SM_QRESUME	= 9,	/*  space in requeue queue      */
}

extern "C" {
    pub fn void(: *mut *mut bfa_rport_sm_t)(struct bfa_rport_s, bfa_rport_event: enum) -> typedef;
}
//
// BFA rport data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut bfa_rport_sm_t sm; / state machine,
    pub /: *mut *mut *mut bfa_s bfa; / backpointer to BFA,
    pub /: *mut *mut *mut void rport_drv; / fcs/driver rport object,
    pub /: *mut *mut u16 fw_handle; / firmware rport handle,
    pub /: *mut *mut u16 rport_tag; / BFA rport tag,
    pub /: *mut *mut u8 lun_mask; / LUN mask flag,
    pub /: *mut *mut bfa_rport_info_s rport_info; / rport info from fcs/driver,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / BFA callback qelem,
    pub /: *mut *mut bfa_rport_hal_stats_s stats; / BFA rport statistics,
    pub qos_attr: bfa_rport_qos_attr_s,
#[repr(C)]
#[derive(Copy, Clone)]
pub union a {
    pub /: *mut *mut bfa_status_t status; / f/w status,
    pub /: *mut *mut *mut void fw_msg; / QoS scn event,
    pub event_arg: },
}

//
// UF - unsolicited receive related defines
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_uf_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut *mut bfa_s bfa; / bfa instance,
    pub /: *mut *mut u16 uf_tag; / identifying tag fw msgs,
    pub vf_id: u16,
    pub src_rport_handle: u16,
    pub rsvd: u16,
    pub data_ptr: *mut u8,
    pub /: *mut *mut u16 data_len; / actual receive length,
    pub /: *mut *mut u16 pb_len; / posted buffer length,
    pub /: *mut *mut *mut void buf_kva; / buffer virtual address,
    pub /: *mut *mut u64 buf_pa; / buffer physical address,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: BFA comp qelem,
    pub sges: [bfa_sge_s; BFI_SGE_INLINE_MAX],
}

//
// Callback prototype for unsolicited frame receive handler.
//
// @param[in]           cbarg           callback arg for receive handler
// @param[in]           uf              unsolicited frame descriptor
//
// @return None
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_uf_recv_t) (void, uf: *mut bfa_uf_s) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_uf_buf_s {
    pub d: [u8; BFA_UF_BUFSZ],
}

// Max UF dma segs required

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_uf_mod_s {
    pub /: *mut *mut *mut bfa_s bfa; / back pointer to BFA,
    pub /: *mut *mut *mut bfa_uf_s uf_list; / array of UFs,
    pub /: *mut *mut u16 num_ufs; / num unsolicited rx frames,
    pub /: *mut *mut list_head uf_free_q; / free UFs,
    pub /: *mut *mut list_head uf_posted_q; / UFs posted to IOC,
    pub /: *mut *mut list_head uf_unused_q; / unused UF's,
    pub uf_buf_posts: *mut bfi_uf_buf_post_s,
// pre-built UF post msgs
    pub /: *mut *mut bfa_cb_uf_recv_t ufrecv; / uf recv handler function,
    pub /: *mut *mut *mut void cbarg; / uf receive handler arg,
    pub dma_seg: [bfa_mem_dma_s; BFA_UF_DMA_SEGS],
    pub kva_seg: bfa_mem_kva_s,
}

extern "C" {
    pub fn bfa_uf_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_uf_res_recfg(bfa: *mut bfa_s, num_uf_fw: u16);
}
//
// lps_pvt BFA LPS private functions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lps_event {
    BFA_LPS_SM_LOGIN	= 1,	/* login request from user      */
    BFA_LPS_SM_LOGOUT	= 2,	/* logout request from user     */
    BFA_LPS_SM_FWRSP	= 3,	/* f/w response to login/logout */
    BFA_LPS_SM_RESUME	= 4,	/* space present in reqq queue  */
    BFA_LPS_SM_DELETE	= 5,	/* lps delete from user         */
    BFA_LPS_SM_OFFLINE	= 6,	/* Link is offline              */
    BFA_LPS_SM_RX_CVL	= 7,	/* Rx clear virtual link        */
    BFA_LPS_SM_SET_N2N_PID  = 8,	/* Set assigned PID for n2n */
}

extern "C" {
    pub fn void(: *mut *mut bfa_lps_sm_t)(struct bfa_lps_s, bfa_lps_event: enum) -> typedef;
}
//
// LPS - bfa lport login/logout service interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lps_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut *mut bfa_s bfa; / parent bfa instance,
    pub /: *mut *mut bfa_lps_sm_t sm; / finite state machine,
    pub /: *mut *mut u8 bfa_tag; / lport tag,
    pub /: *mut *mut u8 fw_tag; / lport fw tag,
    pub /: *mut *mut u8 reqq; / lport request queue,
    pub /: *mut *mut u8 alpa; / ALPA for loop topologies,
    pub /: *mut *mut u32 lp_pid; / lport port ID,
    pub /: *mut *mut bfa_boolean_t fdisc; / snd FDISC instead of FLOGI,
    pub /: *mut *mut bfa_boolean_t auth_en; / enable authentication,
    pub /: *mut *mut bfa_boolean_t auth_req; / authentication required,
    pub /: *mut *mut bfa_boolean_t npiv_en; / NPIV is allowed by peer,
    pub /: *mut *mut bfa_boolean_t fport; / attached peer is F_PORT,
    pub /: *mut *mut bfa_boolean_t brcd_switch; / attached peer is brcd sw,
    pub /: *mut *mut bfa_status_t status; / login status,
    pub /: *mut *mut u16 pdusz; / max receive PDU size,
    pub /: *mut *mut u16 pr_bbcred; / BB_CREDIT from peer,
    pub /: *mut *mut u8 lsrjt_rsn; / LSRJT reason,
    pub /: *mut *mut u8 lsrjt_expl; / LSRJT explanation,
    pub /: *mut *mut u8 lun_mask; / LUN mask flag,
    pub /: *mut *mut wwn_t pwwn; / port wwn of lport,
    pub /: *mut *mut wwn_t nwwn; / node wwn of lport,
    pub /: *mut *mut wwn_t pr_pwwn; / port wwn of lport peer,
    pub /: *mut *mut wwn_t pr_nwwn; / node wwn of lport peer,
    pub /: *mut *mut mac_t lp_mac; / fpma/spma MAC for lport,
    pub /: *mut *mut mac_t fcf_mac; / FCF MAC of lport,
    pub /: *mut *mut bfa_reqq_wait_s wqe; / request wait queue element,
    pub /: *mut *mut *mut void uarg; / user callback arg,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: callback qelem,
    pub loginrsp: *mut bfi_lps_login_rsp_s,
    pub ext_status: bfa_eproto_status_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lps_mod_s {
    pub lps_free_q: list_head,
    pub lps_active_q: list_head,
    pub lps_login_q: list_head,
    pub lps_arr: *mut bfa_lps_s,
    pub num_lps: c_int,
    pub kva_seg: bfa_mem_kva_s,
}

//
// external functions
//
extern "C" {
    pub fn bfa_lps_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
//
// FCPORT related defines
//

//
// BFA port link notification state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcport_ln_sm_event {
    BFA_FCPORT_LN_SM_LINKUP		= 1,	/*  linkup event	*/
    BFA_FCPORT_LN_SM_LINKDOWN	= 2,	/*  linkdown event	*/
    BFA_FCPORT_LN_SM_NOTIFICATION	= 3	/*  done notification	*/
}

extern "C" {
    pub fn void(: *mut *mut bfa_fcport_ln_sm_t)(struct bfa_fcport_ln_s, bfa_fcport_ln_sm_event: enum) -> typedef;
}
//
// Link notification data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcport_ln_s {
    pub fcport: *mut bfa_fcport_s,
    pub sm: bfa_fcport_ln_sm_t,
    pub /: *mut *mut bfa_cb_qe_s ln_qe; / BFA callback queue elem for ln,
    pub /: *mut *mut bfa_port_linkstate ln_event; / ln event for callback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcport_trunk_s {
    pub attr: bfa_trunk_attr_s,
}

//
// BFA port state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcport_sm_event {
    BFA_FCPORT_SM_START	= 1,	/*  start port state machine	*/
    BFA_FCPORT_SM_STOP	= 2,	/*  stop port state machine	*/
    BFA_FCPORT_SM_ENABLE	= 3,	/*  enable port		*/
    BFA_FCPORT_SM_DISABLE	= 4,	/*  disable port state machine */
    BFA_FCPORT_SM_FWRSP	= 5,	/*  firmware enable/disable rsp */
    BFA_FCPORT_SM_LINKUP	= 6,	/*  firmware linkup event	*/
    BFA_FCPORT_SM_LINKDOWN	= 7,	/*  firmware linkup down	*/
    BFA_FCPORT_SM_QRESUME	= 8,	/*  CQ space available	*/
    BFA_FCPORT_SM_HWFAIL	= 9,	/*  IOC h/w failure		*/
    BFA_FCPORT_SM_DPORTENABLE = 10, /*  enable dport      */
    BFA_FCPORT_SM_DPORTDISABLE = 11,/*  disable dport     */
    BFA_FCPORT_SM_FAA_MISCONFIG = 12,	/* FAA misconfiguratin */
    BFA_FCPORT_SM_DDPORTENABLE  = 13,	/* enable ddport	*/
    BFA_FCPORT_SM_DDPORTDISABLE = 14,	/* disable ddport	*/
}

extern "C" {
    pub fn void(: *mut *mut bfa_fcport_sm_t)(struct bfa_fcport_s, bfa_fcport_sm_event: enum) -> typedef;
}
//
// BFA FC port data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcport_s {
    pub /: *mut *mut *mut bfa_s bfa; / parent BFA instance,
    pub /: *mut *mut bfa_fcport_sm_t sm; / port state machine,
    pub /: *mut *mut wwn_t nwwn; / node wwn of physical port,
    pub /: *mut *mut wwn_t pwwn; / port wwn of physical oprt,
    pub speed_sup: bfa_port_speed,
// supported speeds
    pub /: *mut *mut bfa_port_speed speed; / current speed,
    pub /: *mut *mut bfa_port_topology topology; / current topology,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u8 myalpa; / my ALPA in LOOP topology,
    pub /: *mut *mut u8 alpabm_valid; / alpa bitmap valid or not,
    pub /: *mut *mut fc_alpabm_s alpabm; / alpa bitmap,
    pub /: *mut *mut bfa_port_cfg_s cfg; / current port configuration,
    pub /: *mut *mut bfa_boolean_t use_flash_cfg; / get port cfg from flash,
    pub /: *mut *mut bfa_qos_attr_s qos_attr; / QoS Attributes,
    pub /: *mut *mut bfa_qos_vc_attr_s qos_vc_attr; / VC info from ELP,
    pub reqq_wait: bfa_reqq_wait_s,
// to wait for room in reqq
    pub svcreq_wait: bfa_reqq_wait_s,
// to wait for room in reqq
    pub stats_reqq_wait: bfa_reqq_wait_s,
// to wait for room in reqq (stats)
    pub event_cbarg: *mut c_void,
    pub event): bfa_port_linkstate,
    pub i2hmsg: bfi_fcport_i2h_msg_u,
    pub event_arg: },
    pub /: *mut *mut *mut void bfad; / BFA driver handle,
    pub /: *mut *mut bfa_fcport_ln_s ln; / Link Notification,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / BFA callback queue elem,
    pub /: *mut *mut bfa_timer_s timer; / timer,
    pub /: *mut *mut u32 msgtag; / fimrware msg tag for reply,
    pub stats_kva: *mut u8,
    pub stats_pa: u64,
    pub stats: *mut bfa_fcport_stats_u,
    pub /: *mut *mut bfa_status_t stats_status; / stats/statsclr status,
    pub stats_pending_q: list_head,
    pub statsclr_pending_q: list_head,
    pub stats_qfull: bfa_boolean_t,
    pub /: *mut *mut time64_t stats_reset_time; / stats reset time stamp,
    pub /: *mut *mut bfa_boolean_t diag_busy; / diag busy status,
    pub /: *mut *mut bfa_boolean_t beacon; / port beacon status,
    pub /: *mut *mut bfa_boolean_t link_e2e_beacon; / link beacon status,
    pub trunk: bfa_fcport_trunk_s,
    pub fcoe_vlan: u16,
    pub fcport_dma: bfa_mem_dma_s,
    pub stats_dma_ready: bfa_boolean_t,
    pub bbcr_attr: bfa_bbcr_attr_s,
    pub fec_state: bfa_fec_state_s,
}

//
// protected functions
//
extern "C" {
    pub fn bfa_fcport_init(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcport_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
//
// bfa fcport API functions
//
extern "C" {
    pub fn bfa_fcport_enable(bfa: *mut bfa_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcport_disable(bfa: *mut bfa_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcport_get_speed(bfa: *mut bfa_s) -> bfa_port_speed;
}
extern "C" {
    pub fn bfa_fcport_get_topology(bfa: *mut bfa_s) -> bfa_port_topology;
}
extern "C" {
    pub fn bfa_fcport_get_cfg_topology(bfa: *mut bfa_s) -> bfa_port_topology;
}
extern "C" {
    pub fn bfa_fcport_cfg_hardalpa(bfa: *mut bfa_s, alpa: u8) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcport_get_myalpa(bfa: *mut bfa_s) -> u8;
}
extern "C" {
    pub fn bfa_fcport_clr_hardalpa(bfa: *mut bfa_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcport_cfg_maxfrsize(bfa: *mut bfa_s, maxsize: u16) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcport_get_maxfrsize(bfa: *mut bfa_s) -> u16;
}
extern "C" {
    pub fn bfa_fcport_get_rx_bbcredit(bfa: *mut bfa_s) -> u8;
}
extern "C" {
    pub fn bfa_fcport_get_attr(bfa: *mut bfa_s, attr: *mut bfa_port_attr_s);
}
extern "C" {
    pub fn bfa_fcport_is_disabled(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_is_dport(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_is_ddport(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_get_ratelim_speed(bfa: *mut bfa_s) -> bfa_port_speed;
}
extern "C" {
    pub fn bfa_fcport_set_tx_bbcredit(bfa: *mut bfa_s, tx_bbcredit: u16);
}
extern "C" {
    pub fn bfa_fcport_is_ratelim(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_is_linkup(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_is_qos_enabled(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_is_trunk_enabled(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcport_dportenable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcport_dportdisable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcport_is_pbcdisabled(bfa: *mut bfa_s) -> bfa_status_t;
}
//
// bfa rport API functions
//
extern "C" {
    pub fn bfa_rport_speed(rport: *mut bfa_rport_s, speed: bfa_port_speed);
}
extern "C" {
    pub fn bfa_cb_rport_online(rport: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_rport_offline(rport: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_rport_scn_online(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_cb_rport_scn_offline(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_cb_rport_scn_no_dev(rp: *mut c_void);
}
//
// Rport LUN masking related
//
pub const BFA_RPORT_TAG_INVALID: c_uint = 0xffff;
pub const BFA_LP_TAG_INVALID: c_uint = 0xff;
extern "C" {
    pub fn bfa_rport_set_lunmask(bfa: *mut bfa_s, rp: *mut bfa_rport_s);
}
extern "C" {
    pub fn bfa_rport_unset_lunmask(bfa: *mut bfa_s, rp: *mut bfa_rport_s);
}
//
// bfa fcxp API functions
//
extern "C" {
    pub fn bfa_fcxp_discard(fcxp: *mut bfa_fcxp_s);
}
extern "C" {
    pub fn bfa_fcxp_free(fcxp: *mut bfa_fcxp_s);
}
extern "C" {
    pub fn bfa_fcxp_get_maxrsp(bfa: *mut bfa_s) -> u32;
}
extern "C" {
    pub fn bfa_fcxp_res_recfg(bfa: *mut bfa_s, num_fcxp_fw: u16);
}
//
// bfa uf API functions
//
extern "C" {
    pub fn bfa_uf_free(uf: *mut bfa_uf_s);
}
//
// bfa lport service api
//
extern "C" {
    pub fn bfa_lps_get_max_vport(bfa: *mut bfa_s) -> u32;
}
extern "C" {
    pub fn bfa_lps_delete(lps: *mut bfa_lps_s);
}
extern "C" {
    pub fn bfa_lps_fdisclogo(lps: *mut bfa_lps_s);
}
extern "C" {
    pub fn bfa_lps_set_n2n_pid(lps: *mut bfa_lps_s, n2n_pid: u32);
}
extern "C" {
    pub fn bfa_lps_get_fwtag(bfa: *mut bfa_s, lp_tag: u8) -> u8;
}
extern "C" {
    pub fn bfa_lps_get_base_pid(bfa: *mut bfa_s) -> u32;
}
extern "C" {
    pub fn bfa_lps_get_tag_from_pid(bfa: *mut bfa_s, pid: u32) -> u8;
}
extern "C" {
    pub fn bfa_cb_lps_flogi_comp(bfad: *mut c_void, uarg: *mut c_void, status: bfa_status_t);
}
extern "C" {
    pub fn bfa_cb_lps_flogo_comp(bfad: *mut c_void, uarg: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_lps_fdisc_comp(bfad: *mut c_void, uarg: *mut c_void, status: bfa_status_t);
}
extern "C" {
    pub fn bfa_cb_lps_fdisclogo_comp(bfad: *mut c_void, uarg: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_lps_cvl_event(bfad: *mut c_void, uarg: *mut c_void);
}
// FAA specific APIs
//
// FC DIAG data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcdiag_qtest_s {
    pub result: *mut bfa_diag_qtest_result_s,
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub timer: bfa_timer_s,
    pub status: u32,
    pub count: u32,
    pub lock: u8,
    pub queue: u8,
    pub all: u8,
    pub timer_active: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcdiag_lb_s {
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub result: *mut c_void,
    pub lock: bfa_boolean_t,
    pub status: u32,
}

//
// BFA DPORT state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_dport_sm_event {
    BFA_DPORT_SM_ENABLE	= 1,	/* dport enable event         */
    BFA_DPORT_SM_DISABLE    = 2,    /* dport disable event        */
    BFA_DPORT_SM_FWRSP      = 3,    /* fw enable/disable rsp      */
    BFA_DPORT_SM_QRESUME    = 4,    /* CQ space available         */
    BFA_DPORT_SM_HWFAIL     = 5,    /* IOC h/w failure            */
    BFA_DPORT_SM_START	= 6,	/* re-start dport test        */
    BFA_DPORT_SM_REQFAIL	= 7,	/* request failure            */
    BFA_DPORT_SM_SCN	= 8,	/* state change notify frm fw */
}

extern "C" {
    pub fn void(: *mut *mut bfa_dport_sm_t)(struct bfa_dport_s, bfa_dport_sm_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dport_s {
    pub /: *mut *mut *mut bfa_s bfa; / Back pointer to BFA,
    pub /: *mut *mut bfa_dport_sm_t sm; / finite state machine,
    pub reqq_wait: bfa_reqq_wait_s,
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub i2hmsg: bfi_diag_dport_msg_u,
    pub /: *mut *mut u8 test_state; / enum dport_test_state,
    pub /: *mut *mut u8 dynamic; / boolean_t,
    pub rsvd: [u8; 2],
    pub lpcnt: u32,
    pub /: *mut *mut u32 payload; / user defined payload pattern,
    pub rp_pwwn: wwn_t,
    pub rp_nwwn: wwn_t,
    pub result: bfa_diag_dport_result_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcdiag_s {
    pub /: *mut *mut *mut bfa_s bfa; / Back pointer to BFA,
    pub trcmod: *mut bfa_trc_mod_s,
    pub lb: bfa_fcdiag_lb_s,
    pub qtest: bfa_fcdiag_qtest_s,
    pub dport: bfa_dport_s,
}

extern "C" {
    pub fn bfa_fcdiag_intr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_fcdiag_lb_is_running(bfa: *mut bfa_s) -> bfa_status_t;
}
