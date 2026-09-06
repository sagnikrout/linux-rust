//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_cmd.c
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
// Copyright IBM Corp. 2007,2012
//
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

// CPU configuration related functions
pub const SCLP_CMDW_CONFIGURE_CPU: c_uint = 0x00110001;
pub const SCLP_CMDW_DECONFIGURE_CPU: c_uint = 0x00100001;
// Channel path configuration related functions
pub const SCLP_CMDW_CONFIGURE_CHPATH: c_uint = 0x000f0001;
pub const SCLP_CMDW_DECONFIGURE_CHPATH: c_uint = 0x000e0001;
pub const SCLP_CMDW_READ_CHPATH_INFORMATION: c_uint = 0x00030001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_configure_sccb {
    pub header: sccb_header,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chp_cfg_sccb {
    pub header: sccb_header,
    pub ccm: u8,
    pub reserved: [u8; 6],
    pub cssid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chp_info_sccb {
    pub header: sccb_header,
    pub recognized: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub standby: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub configured: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub ccm: u8,
    pub reserved: [u8; 6],
    pub cssid: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn sclp_sync_callback(req: *mut sclp_req, data: *mut c_void) {
    static void sclp_sync_callback(struct sclp_req *req, void *data)
    {
    pub data: *mut *mut completion completion =,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_sync_request(cmd: sclp_cmdw_t, sccb: *mut c_void) -> c_int {
    int sclp_sync_request(sclp_cmdw_t cmd, void *sccb)
    {
    pub 0): return sclp_sync_request_timeout(cmd, sccb,,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_sync_request_timeout(cmd: sclp_cmdw_t, sccb: *mut c_void, timeout: c_int) -> c_int {
    int sclp_sync_request_timeout(sclp_cmdw_t cmd, void *sccb, int timeout)
    {
    pub completion: completion,
    pub request: *mut sclp_req,
    pub rc: c_int,
    pub kzalloc_obj(*request): *mut request =,
    if (!request)
    pub -ENOMEM: return,
    if (timeout)
    pub timeout: request->queue_timeout =,
    pub cmd: request->command =,
    pub sccb: request->sccb =,
    pub SCLP_REQ_FILLED: request->status =,
    pub sclp_sync_callback: request->callback =,
    pub &completion: request->callback_data =,
    pub sclp_add_request(request): rc =,
    if (rc)
    pub out: goto,
    if (request.status != SCLP_REQ_DONE) {
    pr_warn("sync request failed (cmd=0x%08x, status=0x%02x)\n",
    pub request->status): cmd,,
    pub -EIO: rc =,
    }
    out:
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn _sclp_get_core_info(info: *mut sclp_core_info) -> c_int {
    int _sclp_get_core_info(struct sclp_core_info *info)
    {
    pub sccb: *mut read_cpu_info_sccb,
    pub length: int rc,,
    if (!SCLP_HAS_CPU_INFO)
    pub -EOPNOTSUPP: return,
    pub PAGE_SIZE: length = test_facility(140) ? EXT_SCCB_READ_CPU :,
    pub get_order(length)): *mut *mut sccb = (void )__get_free_pages(GFP_KERNEL | GFP_DMA | __GFP_ZERO,,
    if (!sccb)
    pub -ENOMEM: return,
    pub length: sccb->header.length =,
    pub 0x80: sccb->header.control_mask[2] =,
    rc = sclp_sync_request_timeout(SCLP_CMDW_READ_CPU_INFO, sccb,
    if (rc)
    pub out: goto,
    if (sccb.header.response_code != 0x0010) {
    pr_warn("readcpuinfo failed (response=0x%04x)\n",
    pub -EIO: rc =,
    pub out: goto,
    }
    pub sccb): sclp_fill_core_info(info,,
    out:
    pub get_order(length)): free_pages((unsigned long)sccb,,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn do_core_configure(cmd: sclp_cmdw_t) -> c_int {
    static int do_core_configure(sclp_cmdw_t cmd)
    {
    pub sccb: *mut cpu_configure_sccb,
    pub rc: c_int,
    if (!SCLP_HAS_CPU_RECONFIG)
    pub -EOPNOTSUPP: return,
//
// Use kmalloc to have a minimum alignment of 8 bytes and ensure sccb
// is not going to cross a page boundary.
//
    pub GFP_DMA): *mut *mut sccb = kzalloc_obj(sccb, GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub sizeof(*sccb): *mut sccb->header.length =,
    pub SCLP_QUEUE_INTERVAL): rc = sclp_sync_request_timeout(cmd, sccb,,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020:
    case 0x0120:
    default:
    pr_warn("configure cpu failed (cmd=0x%08x, response=0x%04x)\n",
    pub sccb->header.response_code): cmd,,
    pub -EIO: rc =,
    }
    out:
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_core_configure(core: u8) -> c_int {
    int sclp_core_configure(u8 core)
    {
    pub 8): return do_core_configure(SCLP_CMDW_CONFIGURE_CPU | core <<,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_core_deconfigure(core: u8) -> c_int {
    int sclp_core_deconfigure(u8 core)
    {
    pub 8): return do_core_configure(SCLP_CMDW_DECONFIGURE_CPU | core <<,
    }
#[no_mangle]
unsafe extern "C" fn do_chp_configure(cmd: sclp_cmdw_t) -> c_int {
    static int do_chp_configure(sclp_cmdw_t cmd)
    {
    pub sccb: *mut chp_cfg_sccb,
    pub rc: c_int,
    if (!SCLP_HAS_CHP_RECONFIG)
    pub -EOPNOTSUPP: return,
    pub GFP_DMA): *mut *mut sccb = (struct chp_cfg_sccb )get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub sizeof(*sccb): *mut sccb->header.length =,
    pub sccb): rc = sclp_sync_request(cmd,,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020:
    case 0x0120:
    case 0x0440:
    case 0x0450:
    default:
    pr_warn("configure channel-path failed (cmd=0x%08x, response=0x%04x)\n",
    pub sccb->header.response_code): cmd,,
    pub -EIO: rc =,
    }
    out:
    pub long)sccb): free_page((unsigned,
    pub rc: return,
    }
//
// sclp_chp_configure - perform configure channel-path sclp command
// @chpid: channel-path ID
//
// Perform configure channel-path command sclp command for specified chpid.
// Return 0 after command successfully finished, non-zero otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sclp_chp_configure(chpid: chp_id) -> c_int {
    int sclp_chp_configure(struct chp_id chpid)
    {
    pub 8): return do_chp_configure(SCLP_CMDW_CONFIGURE_CHPATH | chpid.id <<,
    }
//
// sclp_chp_deconfigure - perform deconfigure channel-path sclp command
// @chpid: channel-path ID
//
// Perform deconfigure channel-path command sclp command for specified chpid
// and wait for completion. On success return 0. Return non-zero otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sclp_chp_deconfigure(chpid: chp_id) -> c_int {
    int sclp_chp_deconfigure(struct chp_id chpid)
    {
    pub 8): return do_chp_configure(SCLP_CMDW_DECONFIGURE_CHPATH | chpid.id <<,
    }
//
// sclp_chp_read_info - perform read channel-path information sclp command
// @info: resulting channel-path information data
//
// Perform read channel-path information sclp command and wait for completion.
// On success, store channel-path information in @info and return 0. Return
// non-zero otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sclp_chp_read_info(info: *mut sclp_chp_info) -> c_int {
    int sclp_chp_read_info(struct sclp_chp_info *info)
    {
    pub sccb: *mut chp_info_sccb,
    pub rc: c_int,
    if (!SCLP_HAS_CHP_INFO)
    pub -EOPNOTSUPP: return,
    pub GFP_DMA): *mut *mut sccb = (struct chp_info_sccb )get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub sizeof(*sccb): *mut sccb->header.length =,
    pub sccb): rc = sclp_sync_request(SCLP_CMDW_READ_CHPATH_INFORMATION,,
    if (rc)
    pub out: goto,
    if (sccb.header.response_code != 0x0010) {
    pr_warn("read channel-path info failed (response=0x%04x)\n",
    pub -EIO: rc =,
    pub out: goto,
    }
    pub SCLP_CHP_INFO_MASK_SIZE): memcpy(info->recognized, sccb->recognized,,
    pub SCLP_CHP_INFO_MASK_SIZE): memcpy(info->standby, sccb->standby,,
    pub SCLP_CHP_INFO_MASK_SIZE): memcpy(info->configured, sccb->configured,,
    out:
    pub long)sccb): free_page((unsigned,
    pub rc: return,
    }
