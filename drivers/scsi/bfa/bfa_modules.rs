//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_modules.h
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
// bfa_modules.h BFA modules
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_modules_s {
    pub /: *mut *mut bfa_fcdiag_s fcdiag; / fcdiag module,
    pub /: *mut *mut bfa_fcport_s fcport; / fc port module,
    pub /: *mut *mut bfa_fcxp_mod_s fcxp_mod; / fcxp module,
    pub /: *mut *mut bfa_lps_mod_s lps_mod; / fcxp module,
    pub /: *mut *mut bfa_uf_mod_s uf_mod; / unsolicited frame module,
    pub /: *mut *mut bfa_rport_mod_s rport_mod; / remote port module,
    pub /: *mut *mut bfa_fcp_mod_s fcp_mod; / FCP initiator module,
    pub /: *mut *mut bfa_sgpg_mod_s sgpg_mod; / SG page module,
    pub /: *mut *mut bfa_port_s port; / Physical port module,
    pub /: *mut *mut bfa_ablk_s ablk; / ASIC block config module,
    pub /: *mut *mut bfa_cee_s cee; / CEE Module,
    pub /: *mut *mut bfa_sfp_s sfp; / SFP module,
    pub /: *mut *mut bfa_flash_s flash; / flash module,
    pub /: *mut *mut bfa_diag_s diag_mod; / diagnostics module,
    pub /: *mut *mut bfa_phy_s phy; / phy module,
    pub /: *mut *mut bfa_dconf_mod_s dconf_mod; / DCONF common module,
    pub /: *mut *mut bfa_fru_s fru; / fru module,
}

//
// !!! Only append to the enums defined here to avoid any versioning
// !!! needed between trace utility and driver version
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_s {
    pub /: *mut *mut *mut void bfad; / BFA driver instance,
    pub /: *mut *mut *mut bfa_plog_s plog; / portlog buffer,
    pub /: *mut *mut *mut bfa_trc_mod_s trcmod; / driver tracing,
    pub /: *mut *mut bfa_ioc_s ioc; / IOC module,
    pub /: *mut *mut bfa_iocfc_s iocfc; / IOCFC module,
    pub /: *mut *mut bfa_timer_mod_s timer_mod; / timer module,
    pub /: *mut *mut bfa_modules_s modules; / BFA modules,
    pub /: *mut *mut list_head comp_q; / pending completions,
    pub /: *mut *mut bfa_boolean_t queue_process; / queue processing enabled,
    pub reqq_waitq: [list_head; BFI_IOC_MAX_CQS],
    pub /: *mut *mut bfa_boolean_t fcs; / FCS is attached to BFA,
    pub msix: bfa_msix_s,
    pub bfa_aen_seq: c_int,
    pub /: *mut *mut bfa_boolean_t intr_enabled; / Status of interrupts,
}

extern "C" {
    pub fn bfa_dconf_attach(: *mut bfa_s, : *mut c_void, : *mut bfa_iocfc_cfg_s);
}
extern "C" {
    pub fn bfa_dconf_iocdisable(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcp_iocdisable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcpim_iocdisable(: *mut bfa_fcp_mod_s);
}
extern "C" {
    pub fn bfa_fcport_start(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcport_iocdisable(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcxp_iocdisable(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_fcdiag_iocdisable(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_ioim_lm_init(: *mut bfa_s);
}
extern "C" {
    pub fn bfa_lps_iocdisable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_rport_iocdisable(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_uf_start(: *mut bfa_s);
}
