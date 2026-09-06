//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cptpf.h
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
// Copyright (C) 2020 Marvell.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptvf_info {
    pub /: *mut *mut *mut otx2_cptpf_dev cptpf; / PF pointer this VF belongs to,
    pub vfpf_mbox_work: work_struct,
    pub vf_dev: *mut pci_dev,
    pub vf_id: c_int,
    pub intr_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cptpf_flr_work {
    pub work: work_struct,
    pub pf: *mut otx2_cptpf_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptpf_dev {
    pub /: *mut *mut *mut void __iomem reg_base; / CPT PF registers start address,
    pub /: *mut *mut *mut void __iomem afpf_mbox_base; / PF-AF mbox start address,
    pub /: *mut *mut *mut void __iomem vfpf_mbox_base; / VF-PF mbox start address,
    pub /: *mut *mut *mut pci_dev pdev; / PCI device handle,
    pub vf: [otx2_cptvf_info; OTX2_CPT_MAX_VFS_NUM],
    pub /: *mut *mut otx2_cpt_eng_grps eng_grps;/ Engine groups information,
    pub /: *mut *mut otx2_cptlfs_info lfs; / CPT LFs attached to this PF,
    pub /: *mut *mut otx2_cptlfs_info cpt1_lfs; / CPT1 LFs attached to this PF,
// HW capabilities for each engine type
    pub eng_caps: [otx2_cpt_eng_caps; OTX2_CPT_MAX_ENG_TYPES],
    pub is_eng_caps_discovered: bool,
// AF <=> PF mbox
    pub afpf_mbox: otx2_mbox,
    pub afpf_mbox_work: work_struct,
    pub afpf_mbox_wq: *mut workqueue_struct,
    pub afpf_mbox_up: otx2_mbox,
    pub afpf_mbox_up_work: work_struct,
// VF <=> PF mbox
    pub vfpf_mbox: otx2_mbox,
    pub vfpf_mbox_wq: *mut workqueue_struct,
    pub flr_wq: *mut workqueue_struct,
    pub flr_work: *mut cptpf_flr_work,
    pub /: *mut *mut mutex lock; / serialize mailbox access,
    pub cap_flag: c_ulong,
    pub /: *mut *mut u8 pf_id; / RVU PF number,
    pub /: *mut *mut u8 max_vfs; / Maximum number of VFs supported by CPT,
    pub /: *mut *mut u8 enabled_vfs; / Number of enabled VFs,
    pub /: *mut *mut u8 sso_pf_func_ovrd; / SSO PF_FUNC override bit,
    pub /: *mut *mut u8 kvf_limits; / Kernel crypto limits,
    pub has_cpt1: bool,
    pub rsrc_req_blkaddr: u8,
// Devlink
    pub dl: *mut devlink,
}

extern "C" {
    pub fn otx2_cptpf_afpf_mbox_intr(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn otx2_cptpf_afpf_mbox_handler(work: *mut work_struct);
}
extern "C" {
    pub fn otx2_cptpf_afpf_mbox_up_handler(work: *mut work_struct);
}
extern "C" {
    pub fn otx2_cptpf_vfpf_mbox_intr(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn otx2_cptpf_vfpf_mbox_handler(work: *mut work_struct);
}
extern "C" {
    pub fn otx2_inline_cptlf_cleanup(lfs: *mut otx2_cptlfs_info);
}
