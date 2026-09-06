//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cptvf.h
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
pub struct otx2_cptvf_dev {
    pub /: *mut *mut *mut void __iomem reg_base; / Register start address,
    pub /: *mut *mut *mut void __iomem pfvf_mbox_base; / PF-VF mbox start address,
    pub /: *mut *mut *mut pci_dev pdev; / PCI device handle,
    pub /: *mut *mut otx2_cptlfs_info lfs; / CPT LFs attached to this VF,
    pub /: *mut *mut u8 vf_id; / Virtual function index,
// PF <=> VF mbox
    pub pfvf_mbox: otx2_mbox,
    pub pfvf_mbox_work: work_struct,
    pub pfvf_mbox_wq: *mut workqueue_struct,
    pub blkaddr: c_int,
    pub bbuf_base: *mut c_void,
    pub cap_flag: c_ulong,
    pub eng_caps: [u64; OTX2_CPT_MAX_ENG_TYPES],
}

extern "C" {
    pub fn otx2_cptvf_pfvf_mbox_intr(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn otx2_cptvf_pfvf_mbox_handler(work: *mut work_struct);
}
extern "C" {
    pub fn otx2_cptvf_send_eng_grp_num_msg(cptvf: *mut otx2_cptvf_dev, eng_type: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_cptvf_send_kvf_limits_msg(cptvf: *mut otx2_cptvf_dev) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_mbox_bbuf_init(cptvf: *mut otx2_cptvf_dev, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn otx2_cptvf_send_caps_msg(cptvf: *mut otx2_cptvf_dev) -> c_int;
}
