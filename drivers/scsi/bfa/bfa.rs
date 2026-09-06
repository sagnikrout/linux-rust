//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa.h
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

extern "C" {
    pub fn void(bfa: *mut *mut bfa_isr_func_t) (struct bfa_s, m: *mut bfi_msg_s) -> typedef;
}
//
// Interrupt message handlers
//
extern "C" {
    pub fn bfa_isr_unhandled(bfa: *mut bfa_s, m: *mut bfi_msg_s);
}
//
// Request and response queue related defines
//

//
// Circular queue usage assignments
//

//
// static inline void
// bfa_reqq_wait(struct bfa_s *bfa, int reqq, struct bfa_reqq_wait_s *wqe)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_regs_s {
    pub intr_status: *mut void __iomem,
    pub intr_mask: *mut void __iomem,
    pub cpe_q_pi: [*mut void __iomem; BFI_IOC_MAX_CQS],
    pub cpe_q_ci: [*mut void __iomem; BFI_IOC_MAX_CQS],
    pub cpe_q_ctrl: [*mut void __iomem; BFI_IOC_MAX_CQS],
    pub rme_q_ci: [*mut void __iomem; BFI_IOC_MAX_CQS],
    pub rme_q_pi: [*mut void __iomem; BFI_IOC_MAX_CQS],
    pub rme_q_ctrl: [*mut void __iomem; BFI_IOC_MAX_CQS],
}

//
// MSIX vector handlers
//
pub const BFA_MSIX_MAX_VECTORS: c_int = 22;
extern "C" {
    pub fn void(bfa: *mut *mut bfa_msix_handler_t)(struct bfa_s, vec: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_msix_s {
    pub nvecs: c_int,
    pub handler: [bfa_msix_handler_t; BFA_MSIX_MAX_VECTORS],
}

//
// Chip specific interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_hwif_s {
    pub bfa): *mut *mut void (hw_reginit)(struct bfa_s,
    pub reqq): *mut *mut *mut void (hw_reqq_ack)(struct bfa_s bfa, int,
    pub ci): *mut *mut *mut void (hw_rspq_ack)(struct bfa_s bfa, int rspq, u32,
    pub nvecs): *mut *mut *mut void (hw_msix_init)(struct bfa_s bfa, int,
    pub bfa): *mut *mut void (hw_msix_ctrl_install)(struct bfa_s,
    pub bfa): *mut *mut void (hw_msix_queue_install)(struct bfa_s,
    pub bfa): *mut *mut void (hw_msix_uninstall)(struct bfa_s,
    pub msix): *mut *mut *mut void (hw_isr_mode_set)(struct bfa_s bfa, bfa_boolean_t,
    pub maxvec): *mut *mut u32 nvecs, u32,
    pub end): *mut u32,
    pub cpe_vec_q0: c_int,
    pub rme_vec_q0: c_int,
}

extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_iocfc_t) (void, status: bfa_status) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_faa_cbfn_s {
    pub faa_cbfn: bfa_cb_iocfc_t,
    pub faa_cbarg: *mut c_void,
}

pub const BFA_FAA_ENABLED: c_int = 1;
pub const BFA_FAA_DISABLED: c_int = 2;
//
// FAA attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_faa_attr_s {
    pub faa: wwn_t,
    pub faa_state: u8,
    pub pwwn_source: u8,
    pub rsvd: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_faa_args_s {
    pub faa_attr: *mut bfa_faa_attr_s,
    pub faa_cb: bfa_faa_cbfn_s,
    pub faa_state: u8,
    pub busy: bfa_boolean_t,
}

//
// IOCFC state machine definitions/declarations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iocfc_event {
    IOCFC_E_INIT		= 1,	/* IOCFC init request		*/
    IOCFC_E_START		= 2,	/* IOCFC mod start request	*/
    IOCFC_E_STOP		= 3,	/* IOCFC stop request		*/
    IOCFC_E_ENABLE		= 4,	/* IOCFC enable request		*/
    IOCFC_E_DISABLE		= 5,	/* IOCFC disable request	*/
    IOCFC_E_IOC_ENABLED	= 6,	/* IOC enabled message		*/
    IOCFC_E_IOC_DISABLED	= 7,	/* IOC disabled message		*/
    IOCFC_E_IOC_FAILED	= 8,	/* failure notice by IOC sm	*/
    IOCFC_E_DCONF_DONE	= 9,	/* dconf read/write done	*/
    IOCFC_E_CFG_DONE	= 10,	/* IOCFC config complete	*/
}

extern "C" {
    pub fn void(: *mut *mut bfa_iocfs_fsm_t)(struct bfa_iocfc_s, iocfc_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_s {
    pub fsm: bfa_iocfs_fsm_t,
    pub bfa: *mut bfa_s,
    pub cfg: bfa_iocfc_cfg_s,
    pub req_cq_pi: [u32; BFI_IOC_MAX_CQS],
    pub rsp_cq_ci: [u32; BFI_IOC_MAX_CQS],
    pub hw_qid: [u8; BFI_IOC_MAX_CQS],
    pub init_hcb_qe: bfa_cb_qe_s,
    pub stop_hcb_qe: bfa_cb_qe_s,
    pub dis_hcb_qe: bfa_cb_qe_s,
    pub en_hcb_qe: bfa_cb_qe_s,
    pub stats_hcb_qe: bfa_cb_qe_s,
    pub submod_enabled: bfa_boolean_t,
    pub /: *mut *mut bfa_boolean_t cb_reqd; / Driver call back reqd,
    pub /: *mut *mut bfa_status_t op_status; / Status of bfa iocfc op,
    pub cfg_info: bfa_dma_s,
    pub cfginfo: *mut bfi_iocfc_cfg_s,
    pub cfgrsp_dma: bfa_dma_s,
    pub cfgrsp: *mut bfi_iocfc_cfgrsp_s,
    pub req_cq_ba: [bfa_dma_s; BFI_IOC_MAX_CQS],
    pub req_cq_shadow_ci: [bfa_dma_s; BFI_IOC_MAX_CQS],
    pub rsp_cq_ba: [bfa_dma_s; BFI_IOC_MAX_CQS],
    pub rsp_cq_shadow_pi: [bfa_dma_s; BFI_IOC_MAX_CQS],
    pub /: *mut *mut bfa_iocfc_regs_s bfa_regs; / BFA device registers,
    pub hwif: bfa_hwif_s,
    pub /: *mut *mut bfa_cb_iocfc_t updateq_cbfn; / bios callback function,
    pub /: *mut *mut *mut void updateq_cbarg; / bios callback arg,
    pub intr_mask: u32,
    pub faa_args: bfa_faa_args_s,
    pub ioc_dma: bfa_mem_dma_s,
    pub iocfc_dma: bfa_mem_dma_s,
    pub reqq_dma: [bfa_mem_dma_s; BFI_IOC_MAX_CQS],
    pub rspq_dma: [bfa_mem_dma_s; BFI_IOC_MAX_CQS],
    pub kva_seg: bfa_mem_kva_s,
}

//
// FC specific IOC functions.
//
extern "C" {
    pub fn bfa_iocfc_init(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_iocfc_start(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_iocfc_stop(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_iocfc_isr(bfa: *mut c_void, msg: *mut bfi_mbmsg_s);
}
extern "C" {
    pub fn bfa_iocfc_set_snsbase(bfa: *mut bfa_s, seg_no: c_int, snsbase_pa: u64);
}
extern "C" {
    pub fn bfa_iocfc_is_operational(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_iocfc_reset_queues(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_msix_all(bfa: *mut bfa_s, vec: c_int);
}
extern "C" {
    pub fn bfa_msix_reqq(bfa: *mut bfa_s, vec: c_int);
}
extern "C" {
    pub fn bfa_msix_rspq(bfa: *mut bfa_s, vec: c_int);
}
extern "C" {
    pub fn bfa_msix_lpu_err(bfa: *mut bfa_s, vec: c_int);
}
extern "C" {
    pub fn bfa_hwcb_reginit(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwcb_rspq_ack(bfa: *mut bfa_s, rspq: c_int, ci: u32);
}
extern "C" {
    pub fn bfa_hwcb_msix_init(bfa: *mut bfa_s, nvecs: c_int);
}
extern "C" {
    pub fn bfa_hwcb_msix_ctrl_install(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwcb_msix_queue_install(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwcb_msix_uninstall(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwcb_isr_mode_set(bfa: *mut bfa_s, msix: bfa_boolean_t);
}
extern "C" {
    pub fn bfa_hwct_reginit(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwct2_reginit(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwct_reqq_ack(bfa: *mut bfa_s, rspq: c_int);
}
extern "C" {
    pub fn bfa_hwct_rspq_ack(bfa: *mut bfa_s, rspq: c_int, ci: u32);
}
extern "C" {
    pub fn bfa_hwct2_rspq_ack(bfa: *mut bfa_s, rspq: c_int, ci: u32);
}
extern "C" {
    pub fn bfa_hwct_msix_init(bfa: *mut bfa_s, nvecs: c_int);
}
extern "C" {
    pub fn bfa_hwct_msix_ctrl_install(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwct_msix_queue_install(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwct_msix_uninstall(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_hwct_isr_mode_set(bfa: *mut bfa_s, msix: bfa_boolean_t);
}
extern "C" {
    pub fn bfa_iocfc_get_bootwwns(bfa: *mut bfa_s, nwwns: *mut u8, wwns: *mut wwn_t);
}
//
// ----------------------------------------------------------------------
// BFA public interfaces
// ----------------------------------------------------------------------
//

//
// lun mask macros return NULL when min cfg is enabled and there is
// no memory allocated for lunmask.
//

extern "C" {
    pub fn bfa_cfg_get_default(cfg: *mut bfa_iocfc_cfg_s);
}
extern "C" {
    pub fn bfa_detach(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_cb_init(bfad: *mut c_void, status: bfa_status_t);
}
extern "C" {
    pub fn bfa_cb_updateq(bfad: *mut c_void, status: bfa_status_t);
}
extern "C" {
    pub fn bfa_intx(bfa: *mut bfa_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_isr_enable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_isr_disable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_comp_deq(bfa: *mut bfa_s, comp_q: *mut list_head);
}
extern "C" {
    pub fn bfa_comp_process(bfa: *mut bfa_s, comp_q: *mut list_head);
}
extern "C" {
    pub fn bfa_comp_free(bfa: *mut bfa_s, comp_q: *mut list_head);
}
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_ioc_t) (void, status: bfa_status) -> typedef;
}
extern "C" {
    pub fn bfa_iocfc_get_attr(bfa: *mut bfa_s, attr: *mut bfa_iocfc_attr_s);
}
extern "C" {
    pub fn bfa_iocfc_enable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_iocfc_disable(bfa: *mut bfa_s);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cb_pending_q_s {
    pub hcb_qe: bfa_cb_qe_s,
    pub /: *mut *mut *mut void data; / Driver buffer,
}

// Common macros to operate on pending stats/attr apis

