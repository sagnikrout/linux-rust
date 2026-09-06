//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_pci.c
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
// PCI I/O adapter configuration related functions.
//
// Copyright IBM Corp. 2016
//

pub const SCLP_CMDW_CONFIGURE_PCI: c_uint = 0x001a0001;
pub const SCLP_CMDW_DECONFIGURE_PCI: c_uint = 0x001b0001;
pub const SCLP_ATYPE_PCI: c_int = 2;
    static DEFINE_MUTEX(sclp_pci_mutex);
    static struct sclp_register sclp_pci_event = {
    .send_mask = EVTYP_ERRNOTIFY_MASK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_cfg_sccb {
    pub header: sccb_header,
    pub /: *mut *mut u8 atype; / adapter type,
    pub reserved1: u8,
    pub reserved2: u16,
    pub /: *mut *mut u32 aid; / adapter identifier,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn do_pci_configure(cmd: sclp_cmdw_t, fid: u32) -> c_int {
    static int do_pci_configure(sclp_cmdw_t cmd, u32 fid)
    {
    pub sccb: *mut pci_cfg_sccb,
    pub rc: c_int,
    if (!SCLP_HAS_PCI_RECONFIG)
    pub -EOPNOTSUPP: return,
    pub GFP_DMA): *mut *mut sccb = (struct pci_cfg_sccb ) get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub PAGE_SIZE: sccb->header.length =,
    pub SCLP_ATYPE_PCI: sccb->atype =,
    pub fid: sccb->aid =,
    pub sccb): rc = sclp_sync_request(cmd,,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020:
    case 0x0120:
    default:
    pr_warn("configure PCI I/O adapter failed: cmd=0x%08x  response=0x%04x\n",
    pub sccb->header.response_code): cmd,,
    pub -EIO: rc =,
    }
    out:
    pub sccb): free_page((unsigned long),
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_pci_configure(fid: u32) -> c_int {
    int sclp_pci_configure(u32 fid)
    {
    pub fid): return do_pci_configure(SCLP_CMDW_CONFIGURE_PCI,,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_pci_deconfigure(fid: u32) -> c_int {
    int sclp_pci_deconfigure(u32 fid)
    {
    pub fid): return do_pci_configure(SCLP_CMDW_DECONFIGURE_PCI,,
    }
#[no_mangle]
unsafe extern "C" fn sclp_pci_callback(req: *mut sclp_req, data: *mut c_void) {
    static void sclp_pci_callback(struct sclp_req *req, void *data)
    {
    pub data: *mut *mut completion completion =,
    }
#[no_mangle]
unsafe extern "C" fn sclp_pci_check_report(report: *mut zpci_report_error_header) -> c_int {
    static int sclp_pci_check_report(struct zpci_report_error_header *report)
    {
    if (report.version != 1)
    pub -EINVAL: return,
    switch (report.action) {
    case SCLP_ERRNOTIFY_AQ_RESET:
    case SCLP_ERRNOTIFY_AQ_REPAIR:
    case SCLP_ERRNOTIFY_AQ_INFO_LOG:
    case SCLP_ERRNOTIFY_AQ_OPTICS_DATA:
    case SCLP_ERRNOTIFY_AQ_NVME_SMART_LOG:
    case SCLP_ERRNOTIFY_AQ_ADAPTER_INITIALIZED:
    case SCLP_ERRNOTIFY_AQ_RECOVERABLE_ERROR:
    case SCLP_ERRNOTIFY_AQ_TELEMETRY_DATA:
    default:
    pub -EINVAL: return,
    }
    if (report.length > (PAGE_SIZE - sizeof(struct err_notify_sccb)))
    pub -EINVAL: return,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_pci_report(report: *mut zpci_report_error_header, fh: u32, fid: u32) -> c_int {
    int sclp_pci_report(struct zpci_report_error_header *report, u32 fh, u32 fid)
    {
    pub sccb: *mut err_notify_sccb,
    pub req: sclp_req,
    pub ret: c_int,
    pub sclp_pci_check_report(report): ret =,
    if (ret)
    pub ret: return,
    pub sclp_register(&sclp_pci_event): ret =,
    if (ret)
    pub out_unlock: goto,
    if (!(sclp_pci_event.sclp_receive_mask & EVTYP_ERRNOTIFY_MASK)) {
    pub -EOPNOTSUPP: ret =,
    pub out_unregister: goto,
    }
    pub GFP_DMA): *mut *mut sccb = (void ) get_zeroed_page(GFP_KERNEL |,
    if (!sccb) {
    pub -ENOMEM: ret =,
    pub out_unregister: goto,
    }
    pub sizeof(req)): memset(&req, 0,,
    pub &completion: req.callback_data =,
    pub sclp_pci_callback: req.callback =,
    pub SCLP_CMDW_WRITE_EVENT_DATA: req.command =,
    pub SCLP_REQ_FILLED: req.status =,
    pub sccb: req.sccb =,
    pub report->length: sccb->evbuf.header.length = sizeof(sccb->evbuf) +,
    pub EVTYP_ERRNOTIFY: sccb->evbuf.header.type =,
    pub sccb->evbuf.header.length: sccb->header.length = sizeof(sccb->header) +,
    pub report->action: sccb->evbuf.action =,
    pub SCLP_ATYPE_PCI: sccb->evbuf.atype =,
    pub fh: sccb->evbuf.fh =,
    pub fid: sccb->evbuf.fid =,
    pub report->length): memcpy(sccb->evbuf.data, report->data,,
    pub sclp_add_request(&req): ret =,
    if (ret)
    pub out_free_req: goto,
    if (req.status != SCLP_REQ_DONE) {
    pr_warn("request failed (status=0x%02x)\n",
    pub -EIO: ret =,
    pub out_free_req: goto,
    }
    if (sccb.header.response_code != 0x0020) {
    pr_warn("request failed with response code 0x%x\n",
    pub -EIO: ret =,
    }
    out_free_req:
    pub sccb): free_page((unsigned long),
    out_unregister:
    out_unlock:
    pub ret: return,
    }
