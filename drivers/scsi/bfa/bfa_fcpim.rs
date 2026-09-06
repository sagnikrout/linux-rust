//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_fcpim.h
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

// FCP module related definitions

pub const BFA_FWTIO_MAX: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iotag_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut u16 tag; / FW IO tag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itn_s {
    pub isr: bfa_isr_func_t,
}

extern "C" {
    pub fn bfa_itn_isr(bfa: *mut bfa_s, m: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_iotag_attach(fcp: *mut bfa_fcp_mod_s);
}
extern "C" {
    pub fn bfa_fcp_res_recfg(bfa: *mut bfa_s, num_ioim_fw: u16, max_ioim_fw: u16);
}

pub const BFA_ITNIM_MIN: c_int = 32;
pub const BFA_ITNIM_MAX: c_int = 1024;
pub const BFA_IOIM_MIN: c_int = 8;
pub const BFA_IOIM_MAX: c_int = 2000;
pub const BFA_TSKIM_MIN: c_int = 4;
pub const BFA_TSKIM_MAX: c_int = 512;

pub const BFA_IOIM_RETRY_TAG_OFFSET: c_int = 11;
pub const BFA_IOIM_IOTAG_MASK: c_uint = 0x07ff /* 2K IOs */;
pub const BFA_IOIM_RETRY_MAX: c_int = 7;
// Buckets are are 512 bytes to 2MB
//
// forward declarations
//
extern "C" {
    pub fn void(ioim: *mut *mut bfa_fcpim_profile_t) (struct bfa_ioim_s) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcpim_s {
    pub bfa: *mut bfa_s,
    pub fcp: *mut bfa_fcp_mod_s,
    pub itnim_arr: *mut bfa_itnim_s,
    pub ioim_arr: *mut bfa_ioim_s,
    pub ioim_sp_arr: *mut bfa_ioim_sp_s,
    pub tskim_arr: *mut bfa_tskim_s,
    pub num_itnims: c_int,
    pub num_tskim_reqs: c_int,
    pub path_tov: u32,
    pub q_depth: u16,
    pub /: *mut *mut u8 reqq; / Request queue to be used,
    pub /: *mut *mut list_head itnim_q; / queue of active itnim,
    pub /: *mut *mut list_head ioim_resfree_q; / IOs waiting for f/w,
    pub /: *mut *mut list_head ioim_comp_q; / IO global comp Q,
    pub tskim_free_q: list_head,
    pub /: *mut *mut list_head tskim_unused_q; / Unused tskim Q,
    pub /: *mut *mut u32 ios_active; / current active IOs,
    pub delay_comp: u32,
    pub del_itn_stats: bfa_fcpim_del_itn_stats_s,
    pub ioredirect: bfa_boolean_t,
    pub io_profile: bfa_boolean_t,
    pub io_profile_start_time: time64_t,
    pub profile_comp: bfa_fcpim_profile_t,
    pub profile_start: bfa_fcpim_profile_t,
}

// Max FCP dma segs required

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcp_mod_s {
    pub bfa: *mut bfa_s,
    pub /: *mut *mut list_head iotag_ioim_free_q; / free IO resources,
    pub /: *mut *mut list_head iotag_tio_free_q; / free IO resources,
    pub resources*/: *mut *mut list_head iotag_unused_q; / unused IO,
    pub iotag_arr: *mut bfa_iotag_s,
    pub itn_arr: *mut bfa_itn_s,
    pub max_ioim_reqs: c_int,
    pub num_ioim_reqs: c_int,
    pub num_fwtio_reqs: c_int,
    pub num_itns: c_int,
    pub snsbase: [bfa_dma_s; BFA_FCP_DMA_SEGS],
    pub fcpim: bfa_fcpim_s,
    pub dma_seg: [bfa_mem_dma_s; BFA_FCP_DMA_SEGS],
    pub kva_seg: bfa_mem_kva_s,
    pub throttle_update_required: c_int,
}

//
// IO state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioim_event {
    BFA_IOIM_SM_START	= 1,	/*  io start request from host */
    BFA_IOIM_SM_COMP_GOOD	= 2,	/*  io good comp, resource free */
    BFA_IOIM_SM_COMP	= 3,	/*  io comp, resource is free */
    BFA_IOIM_SM_COMP_UTAG	= 4,	/*  io comp, resource is free */
    BFA_IOIM_SM_DONE	= 5,	/*  io comp, resource not free */
    BFA_IOIM_SM_FREE	= 6,	/*  io resource is freed */
    BFA_IOIM_SM_ABORT	= 7,	/*  abort request from scsi stack */
    BFA_IOIM_SM_ABORT_COMP	= 8,	/*  abort from f/w */
    BFA_IOIM_SM_ABORT_DONE	= 9,	/*  abort completion from f/w */
    BFA_IOIM_SM_QRESUME	= 10,	/*  CQ space available to queue IO */
    BFA_IOIM_SM_SGALLOCED	= 11,	/*  SG page allocation successful */
    BFA_IOIM_SM_SQRETRY	= 12,	/*  sequence recovery retry */
    BFA_IOIM_SM_HCB		= 13,	/*  bfa callback complete */
    BFA_IOIM_SM_CLEANUP	= 14,	/*  IO cleanup from itnim */
    BFA_IOIM_SM_TMSTART	= 15,	/*  IO cleanup from tskim */
    BFA_IOIM_SM_TMDONE	= 16,	/*  IO cleanup from tskim */
    BFA_IOIM_SM_HWFAIL	= 17,	/*  IOC h/w failure event */
    BFA_IOIM_SM_IOTOV	= 18,	/*  ITN offline TOV */
}

extern "C" {
    pub fn void(: *mut *mut bfa_ioim_sm_t)(struct bfa_ioim_s, bfa_ioim_event: enum) -> typedef;
}
//
// BFA IO (initiator mode)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioim_s {
    pub /: *mut *mut list_head qe; / queue elememt,
    pub /: *mut *mut bfa_ioim_sm_t sm; / BFA ioim state machine,
    pub /: *mut *mut *mut bfa_s bfa; / BFA module,
    pub /: *mut *mut *mut bfa_fcpim_s fcpim; / parent fcpim module,
    pub /: *mut *mut *mut bfa_itnim_s itnim; / i-t-n nexus for this IO,
    pub /: *mut *mut *mut bfad_ioim_s dio; / driver IO handle,
    pub /: *mut *mut u16 iotag; / FWI IO tag,
    pub /: *mut *mut u16 abort_tag; / unqiue abort request tag,
    pub /: *mut *mut u16 nsges; / number of SG elements,
    pub /: *mut *mut u16 nsgpgs; / number of SG pages,
    pub /: *mut *mut *mut bfa_sgpg_s sgpg; / first SG page,
    pub /: *mut *mut list_head sgpg_q; / allocated SG pages,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / bfa callback qelem,
    pub /: *mut *mut bfa_cb_cbfn_t io_cbfn; / IO completion handler,
    pub /: *mut *mut *mut bfa_ioim_sp_s iosp; / slow-path IO handling,
    pub /: *mut *mut u8 reqq; / Request queue for I/O,
    pub /: *mut *mut u8 mode; / IO is passthrough or not,
    pub /: *mut *mut u64 start_time; / IO's Profile start val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioim_sp_s {
    pub /: *mut *mut bfi_msg_s comp_rspmsg; / IO comp f/w response,
    pub /: *mut *mut bfa_sgpg_wqe_s sgpg_wqe; / waitq elem for sgpg,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut bfa_boolean_t abort_explicit; / aborted by OS,
    pub /: *mut *mut *mut bfa_tskim_s tskim; / Relevant TM cmd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_tskim_event {
    BFA_TSKIM_SM_START	= 1,	/*  TM command start		*/
    BFA_TSKIM_SM_DONE	= 2,	/*  TM completion		*/
    BFA_TSKIM_SM_QRESUME	= 3,	/*  resume after qfull		*/
    BFA_TSKIM_SM_HWFAIL	= 5,	/*  IOC h/w failure event	*/
    BFA_TSKIM_SM_HCB	= 6,	/*  BFA callback completion	*/
    BFA_TSKIM_SM_IOS_DONE	= 7,	/*  IO and sub TM completions	*/
    BFA_TSKIM_SM_CLEANUP	= 8,	/*  TM cleanup on ITN offline	*/
    BFA_TSKIM_SM_CLEANUP_DONE = 9,	/*  TM abort completion	*/
    BFA_TSKIM_SM_UTAG	= 10,	/*  TM completion unknown tag  */
}

extern "C" {
    pub fn void(: *mut *mut bfa_tskim_sm_t)(struct bfa_tskim_s, bfa_tskim_event: enum) -> typedef;
}
//
// BFA Task management command (initiator mode)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_tskim_s {
    pub qe: list_head,
    pub sm: bfa_tskim_sm_t,
    pub /: *mut *mut *mut bfa_s bfa; / BFA module,
    pub /: *mut *mut *mut bfa_fcpim_s fcpim; / parent fcpim module,
    pub /: *mut *mut *mut bfa_itnim_s itnim; / i-t-n nexus for this IO,
    pub /: *mut *mut *mut bfad_tskim_s dtsk; / driver task mgmt cmnd,
    pub /: *mut *mut bfa_boolean_t notify; / notify itnim on TM comp,
    pub /: *mut *mut scsi_lun lun; / lun if applicable,
    pub /: *mut *mut fcp_tm_cmnd tm_cmnd; / task management command,
    pub /: *mut *mut u16 tsk_tag; / FWI IO tag,
    pub /: *mut *mut u8 tsecs; / timeout in seconds,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut list_head io_q; / queue of affected IOs,
    pub /: *mut *mut bfa_wc_s wc; / waiting counter,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / bfa callback qelem,
    pub /: *mut *mut bfi_tskim_status tsk_status; / TM status,
}

//
// itnim state machine event
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_itnim_event {
    BFA_ITNIM_SM_CREATE = 1,	/*  itnim is created */
    BFA_ITNIM_SM_ONLINE = 2,	/*  itnim is online */
    BFA_ITNIM_SM_OFFLINE = 3,	/*  itnim is offline */
    BFA_ITNIM_SM_FWRSP = 4,		/*  firmware response */
    BFA_ITNIM_SM_DELETE = 5,	/*  deleting an existing itnim */
    BFA_ITNIM_SM_CLEANUP = 6,	/*  IO cleanup completion */
    BFA_ITNIM_SM_SLER = 7,		/*  second level error recovery */
    BFA_ITNIM_SM_HWFAIL = 8,	/*  IOC h/w failure event */
    BFA_ITNIM_SM_QRESUME = 9,	/*  queue space available */
}

extern "C" {
    pub fn void(: *mut *mut bfa_itnim_sm_t)(struct bfa_itnim_s, bfa_itnim_event: enum) -> typedef;
}
//
// BFA i-t-n (initiator mode)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut bfa_itnim_sm_t sm; / i-t-n im BFA state machine,
    pub /: *mut *mut *mut bfa_s bfa; / bfa instance,
    pub /: *mut *mut *mut bfa_rport_s rport; / bfa rport,
    pub /: *mut *mut *mut void ditn; / driver i-t-n structure,
    pub /: *mut *mut bfi_mhdr_s mhdr; / pre-built mhdr,
    pub /: *mut *mut u8 msg_no; / itnim/rport firmware handle,
    pub /: *mut *mut u8 reqq; / CQ for requests,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / bfa callback qelem,
    pub /: *mut *mut list_head pending_q; / queue of pending IO requests,
    pub /: *mut *mut list_head io_q; / queue of active IO requests,
    pub /: *mut *mut list_head io_cleanup_q; / IO being cleaned up,
    pub /: *mut *mut list_head tsk_q; / queue of active TM commands,
    pub /: *mut *mut list_head delay_comp_q; / queue of failed inflight cmds,
    pub /: *mut *mut bfa_boolean_t seq_rec; / SQER supported,
    pub /: *mut *mut bfa_boolean_t is_online; / itnim is ONLINE for IO,
    pub /: *mut *mut bfa_boolean_t iotov_active; / IO TOV timer is active,
    pub /: *mut *mut bfa_wc_s wc; / waiting counter,
    pub /: *mut *mut bfa_timer_s timer; / pending IO TOV,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut *mut bfa_fcpim_s fcpim; / fcpim module,
    pub stats: bfa_itnim_iostats_s,
    pub ioprofile: bfa_itnim_ioprofile_s,
}

//
// function prototypes
//
extern "C" {
    pub fn bfa_ioim_attach(fcpim: *mut bfa_fcpim_s);
}
extern "C" {
    pub fn bfa_ioim_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_ioim_cleanup(ioim: *mut bfa_ioim_s);
}
extern "C" {
    pub fn bfa_ioim_iocdisable(ioim: *mut bfa_ioim_s);
}
extern "C" {
    pub fn bfa_ioim_tov(ioim: *mut bfa_ioim_s);
}
extern "C" {
    pub fn bfa_tskim_attach(fcpim: *mut bfa_fcpim_s);
}
extern "C" {
    pub fn bfa_tskim_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_tskim_iocdisable(tskim: *mut bfa_tskim_s);
}
extern "C" {
    pub fn bfa_tskim_cleanup(tskim: *mut bfa_tskim_s);
}
extern "C" {
    pub fn bfa_tskim_res_recfg(bfa: *mut bfa_s, num_tskim_fw: u16);
}
extern "C" {
    pub fn bfa_itnim_meminfo(cfg: *mut bfa_iocfc_cfg_s, km_len: *mut u32);
}
extern "C" {
    pub fn bfa_itnim_attach(fcpim: *mut bfa_fcpim_s);
}
extern "C" {
    pub fn bfa_itnim_iocdisable(itnim: *mut bfa_itnim_s);
}
extern "C" {
    pub fn bfa_itnim_isr(bfa: *mut bfa_s, msg: *mut bfi_msg_s);
}
extern "C" {
    pub fn bfa_itnim_iodone(itnim: *mut bfa_itnim_s);
}
extern "C" {
    pub fn bfa_itnim_tskdone(itnim: *mut bfa_itnim_s);
}
extern "C" {
    pub fn bfa_itnim_hold_io(itnim: *mut bfa_itnim_s) -> bfa_boolean_t;
}
//
// bfa fcpim module API functions
//
extern "C" {
    pub fn bfa_fcpim_path_tov_set(bfa: *mut bfa_s, path_tov: u16);
}
extern "C" {
    pub fn bfa_fcpim_path_tov_get(bfa: *mut bfa_s) -> u16;
}
extern "C" {
    pub fn bfa_fcpim_qdepth_get(bfa: *mut bfa_s) -> u16;
}
extern "C" {
    pub fn bfa_fcpim_profile_on(bfa: *mut bfa_s, time: time64_t) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_profile_off(bfa: *mut bfa_s) -> bfa_status_t;
}

// (__qid) = __fcpim->reqq;					\

// (__qid) = (u8)((__msg) & (BFI_IOC_MAX_CQS - 1));
//
// bfa itnim API functions
//
extern "C" {
    pub fn bfa_itnim_delete(itnim: *mut bfa_itnim_s);
}
extern "C" {
    pub fn bfa_itnim_online(itnim: *mut bfa_itnim_s, seq_rec: bfa_boolean_t);
}
extern "C" {
    pub fn bfa_itnim_offline(itnim: *mut bfa_itnim_s);
}
extern "C" {
    pub fn bfa_itnim_clear_stats(itnim: *mut bfa_itnim_s);
}

//
// BFA completion callback for bfa_itnim_online().
//
extern "C" {
    pub fn bfa_cb_itnim_online(itnim: *mut c_void);
}
//
// BFA completion callback for bfa_itnim_offline().
//
extern "C" {
    pub fn bfa_cb_itnim_offline(itnim: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_itnim_tov_begin(itnim: *mut c_void);
}
extern "C" {
    pub fn bfa_cb_itnim_tov(itnim: *mut c_void);
}
//
// BFA notification to FCS/driver for second level error recovery.
// Atleast one I/O request has timedout and target is unresponsive to
// repeated abort requests. Second level error recovery should be initiated
// by starting implicit logout and recovery procedures.
//
extern "C" {
    pub fn bfa_cb_itnim_sler(itnim: *mut c_void);
}
//
// bfa ioim API functions
//
extern "C" {
    pub fn bfa_ioim_free(ioim: *mut bfa_ioim_s);
}
extern "C" {
    pub fn bfa_ioim_start(ioim: *mut bfa_ioim_s);
}
extern "C" {
    pub fn bfa_ioim_abort(ioim: *mut bfa_ioim_s) -> bfa_status_t;
}
//
// I/O completion notification.
//
// @param[in]		dio			driver IO structure
// @param[in]		io_status		IO completion status
// @param[in]		scsi_status		SCSI status returned by target
// @param[in]		sns_len			SCSI sense length, 0 if none
// @param[in]		sns_info		SCSI sense data, if any
// @param[in]		residue			Residual length
//
// @return None
//
// I/O good completion notification.
//
extern "C" {
    pub fn bfa_cb_ioim_good_comp(bfad: *mut c_void, dio: *mut bfad_ioim_s);
}
//
// I/O abort completion notification
//
extern "C" {
    pub fn bfa_cb_ioim_abort(bfad: *mut c_void, dio: *mut bfad_ioim_s);
}
//
// bfa tskim API functions
//
extern "C" {
    pub fn bfa_tskim_free(tskim: *mut bfa_tskim_s);
}
extern "C" {
    pub fn bfa_fcpim_lunmask_update(bfa: *mut bfa_s, on_off: u32) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_lunmask_query(bfa: *mut bfa_s, buf: *mut c_void) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_lunmask_clear(bfa: *mut bfa_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_read_throttle(bfa: *mut bfa_s) -> u16;
}
extern "C" {
    pub fn bfa_fcpim_write_throttle(bfa: *mut bfa_s, value: u16) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_throttle_set(bfa: *mut bfa_s, value: u16) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_throttle_get(bfa: *mut bfa_s, buf: *mut c_void) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcpim_get_throttle_cfg(bfa: *mut bfa_s, drv_cfg_param: u16) -> u16;
}
