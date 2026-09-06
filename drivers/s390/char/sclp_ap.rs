//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_ap.c
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
//
// s390 crypto adapter related sclp functions.
//
// Copyright IBM Corp. 2020
//

pub const SCLP_CMDW_CONFIGURE_AP: c_uint = 0x001f0001;
pub const SCLP_CMDW_DECONFIGURE_AP: c_uint = 0x001e0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_cfg_sccb {
    pub header: sccb_header,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn do_ap_configure(cmd: sclp_cmdw_t, apid: u32) -> c_int {
    static int do_ap_configure(sclp_cmdw_t cmd, u32 apid)
    {
    pub sccb: *mut ap_cfg_sccb,
    pub rc: c_int,
    if (!SCLP_HAS_AP_RECONFIG)
    pub -EOPNOTSUPP: return,
    pub GFP_DMA): *mut *mut sccb = (struct ap_cfg_sccb ) get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub PAGE_SIZE: sccb->header.length =,
    pub 8: cmd |= (apid & 0xFF) <<,
    pub sccb): rc = sclp_sync_request(cmd,,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020: case 0x0120: case 0x0440: case 0x0450:
    default:
    pr_warn("configure AP adapter %u failed: cmd=0x%08x response=0x%04x\n",
    pub sccb->header.response_code): apid, cmd,,
    pub -EIO: rc =,
    }
    out:
    pub sccb): free_page((unsigned long),
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_ap_configure(apid: u32) -> c_int {
    int sclp_ap_configure(u32 apid)
    {
    pub apid): return do_ap_configure(SCLP_CMDW_CONFIGURE_AP,,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_ap_deconfigure(apid: u32) -> c_int {
    int sclp_ap_deconfigure(u32 apid)
    {
    pub apid): return do_ap_configure(SCLP_CMDW_DECONFIGURE_AP,,
    }
