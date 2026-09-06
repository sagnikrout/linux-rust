//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/sclp.h
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
// Copyright IBM Corp. 1999,2012
//
// Author(s): Martin Peschke <mpeschke@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
//

// maximum number of pages concerning our own memory management

pub const SCLP_CONSOLE_PAGES: c_int = 6;

pub const EVTYP_OPCMD: c_uint = 0x01;
pub const EVTYP_MSG: c_uint = 0x02;
pub const EVTYP_CONFMGMDATA: c_uint = 0x04;
pub const EVTYP_DIAG_TEST: c_uint = 0x07;
pub const EVTYP_STATECHANGE: c_uint = 0x08;
pub const EVTYP_PMSGCMD: c_uint = 0x09;
pub const EVTYP_ASYNC: c_uint = 0x0A;
pub const EVTYP_CTLPROGIDENT: c_uint = 0x0B;
pub const EVTYP_STORE_DATA: c_uint = 0x0C;
pub const EVTYP_ERRNOTIFY: c_uint = 0x18;
pub const EVTYP_VT220MSG: c_uint = 0x1A;
pub const EVTYP_SDIAS: c_uint = 0x1C;
pub const EVTYP_SIGQUIESCE: c_uint = 0x1D;
pub const EVTYP_OCF: c_uint = 0x1E;

pub const GNRLMSGFLGS_DOM: c_uint = 0x8000;
pub const GNRLMSGFLGS_SNDALRM: c_uint = 0x4000;
pub const GNRLMSGFLGS_HOLDMSG: c_uint = 0x2000;
pub const LNTPFLGS_CNTLTEXT: c_uint = 0x8000;
pub const LNTPFLGS_LABELTEXT: c_uint = 0x4000;
pub const LNTPFLGS_DATATEXT: c_uint = 0x2000;
pub const LNTPFLGS_ENDTEXT: c_uint = 0x1000;
pub const LNTPFLGS_PROMPTTEXT: c_uint = 0x0800;
pub type sclp_cmdw_t = c_uint;
pub const SCLP_CMDW_READ_CPU_INFO: c_uint = 0x00010001;
pub const SCLP_CMDW_READ_SCP_INFO: c_uint = 0x00020001;
pub const SCLP_CMDW_READ_STORAGE_INFO: c_uint = 0x00040001;
pub const SCLP_CMDW_READ_SCP_INFO_FORCED: c_uint = 0x00120001;
pub const SCLP_CMDW_READ_EVENT_DATA: c_uint = 0x00770005;
pub const SCLP_CMDW_WRITE_EVENT_DATA: c_uint = 0x00760005;
pub const SCLP_CMDW_WRITE_EVENT_MASK: c_uint = 0x00780005;
pub const GDS_ID_MDSMU: c_uint = 0x1310;
pub const GDS_ID_MDSROUTEINFO: c_uint = 0x1311;
pub const GDS_ID_AGUNWRKCORR: c_uint = 0x1549;
pub const GDS_ID_SNACONDREPORT: c_uint = 0x1532;
pub const GDS_ID_CPMSU: c_uint = 0x1212;
pub const GDS_ID_ROUTTARGINSTR: c_uint = 0x154D;
pub const GDS_ID_OPREQ: c_uint = 0x8070;
pub const GDS_ID_TEXTCMD: c_uint = 0x1320;
pub const GDS_KEY_SELFDEFTEXTMSG: c_uint = 0x31;
pub type sccb_mask_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_sccb {
    pub header: sccb_header,
    pub _reserved: u16,
    pub mask_length: u16,
    pub /: *mut *mut *mut u8 masks[4  1021]; / variable length,
//
// u8 receive_mask[mask_length];
// u8 send_mask[mask_length];
// u8 sclp_receive_mask[mask_length];
// u8 sclp_send_mask[mask_length];
//
    pub __attribute__((packed)): },
pub const SCLP_MASK_SIZE_COMPAT: c_int = 4;
    pub 0: sccb_mask_t res =,
    pub len)): *mut *mut memcpy(&res, masks + i  len, min(sizeof(res),,
    pub res: return,
    pub len): *mut *mut memset(masks + i  len, 0,,
    pub len)): *mut *mut memcpy(masks + i  len, &val, min(sizeof(val),,

    pub \: __typeof__(sccb) __sccb = sccb;,
    pub \: sccb_get_mask(__sccb->masks, __sccb->mask_length, i);,

    pub \: __typeof__(sccb) __sccb = sccb;,
    pub \: sccb_set_mask(__sccb->masks, __sccb->mask_length, i, val);,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_cpu_info_sccb {
    pub header: sccb_header,
    pub nr_configured: u16,
    pub offset_configured: u16,
    pub nr_standby: u16,
    pub offset_standby: u16,
//
// Without ext sccb, struct size is PAGE_SIZE.
// With ext sccb, struct size is EXT_SCCB_READ_CPU.
//
    pub reserved: [u8; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_info_sccb {
    pub /: *mut *mut sccb_header header; / 0-7,
    pub /: *mut *mut u16 rnmax; / 8-9,
    pub /: *mut *mut u8 rnsize; / 10,
    pub /: *mut *mut u8 _pad_11[16 - 11]; / 11-15,
    pub /: *mut *mut u16 ncpurl; / 16-17,
    pub /: *mut *mut u16 cpuoff; / 18-19,
    pub /: *mut *mut u8 _pad_20[24 - 20]; / 20-23,
    pub /: *mut *mut u8 loadparm[8]; / 24-31,
    pub /: *mut *mut u8 _pad_32[42 - 32]; / 32-41,
    pub /: *mut *mut u8 fac42; / 42,
    pub /: *mut *mut u8 fac43; / 43,
    pub /: *mut *mut u8 _pad_44[48 - 44]; / 44-47,
    pub /: *mut *mut u64 facilities; / 48-55,
    pub /: *mut *mut u8 _pad_56[66 - 56]; / 56-65,
    pub /: *mut *mut u8 fac66; / 66,
    pub /: *mut *mut u8 _pad_67[76 - 67]; / 67-83,
    pub /: *mut *mut u32 ibc; / 76-79,
    pub /: *mut *mut u8 _pad80[84 - 80]; / 80-83,
    pub /: *mut *mut u8 fac84; / 84,
    pub /: *mut *mut u8 fac85; / 85,
    pub /: *mut *mut u8 _pad_86[91 - 86]; / 86-90,
    pub /: *mut *mut u8 fac91; / 91,
    pub /: *mut *mut u8 _pad_92[98 - 92]; / 92-97,
    pub /: *mut *mut u8 fac98; / 98,
    pub /: *mut *mut u8 hamaxpow; / 99,
    pub /: *mut *mut u32 rnsize2; / 100-103,
    pub /: *mut *mut u64 rnmax2; / 104-111,
    pub /: *mut *mut u32 hsa_size; / 112-115,
    pub /: *mut *mut u8 fac116; / 116,
    pub /: *mut *mut u8 fac117; / 117,
    pub /: *mut *mut u8 fac118; / 118,
    pub /: *mut *mut u8 fac119; / 119,
    pub /: *mut *mut u16 hcpua; / 120-121,
    pub /: *mut *mut u8 _pad_122[124 - 122]; / 122-123,
    pub /: *mut *mut u32 hmfai; / 124-127,
    pub /: *mut *mut u8 _pad_128[134 - 128]; / 128-133,
    pub /: *mut *mut u8 byte_134; / 134,
    pub /: *mut *mut u8 cpudirq; / 135,
    pub /: *mut *mut u16 cbl; / 136-137,
    pub /: *mut *mut u8 byte_138; / 138,
    pub /: *mut *mut u8 byte_139; / 139,
    pub 140]: u8 _pad_140[EXT_SCCB_READ_SCP -,
    pub __aligned(PAGE_SIZE): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_storage_sccb {
    pub header: sccb_header,
    pub max_id: u16,
    pub assigned: u16,
    pub standby: u16,
    pub :16: u16,
    pub entries: [u32; ],
    pub __packed: },
    pub sccb: *mut *mut *mut char page = (char ),
    pub sizeof(*info)): *mut memset(info, 0,,
    pub sccb->nr_configured: info->configured =,
    pub sccb->nr_standby: info->standby =,
    pub sccb->nr_standby: info->combined = sccb->nr_configured +,
    pub sclp_core_entry)): *mut *mut info->combined  sizeof(struct,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gds_subvector {
    pub length: u8,
    pub key: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gds_vector {
    pub length: u16,
    pub gds_id: u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_req {
    pub /: *mut *mut list_head list; / list_head for request queueing.,
    pub /: *mut *mut sclp_cmdw_t command; / sclp command to execute,
    pub /: *mut *mut *mut void sccb; / pointer to the sccb to execute,
    pub /: *mut *mut char status; / status of this request,
    pub /: *mut *mut int start_count; / number of SVCs done for this req,
// Callback that is called after reaching final status.
    pub data): *mut *mut *mut void (callback)(struct sclp_req , void,
    pub callback_data: *mut c_void,
    pub by: *mut *mut int queue_timeout; / request queue timeout (sec), set,
// Internal fields
    pub /: *mut *mut unsigned long queue_expires; / request queue timeout (jiffies),
}

pub const SCLP_REQ_FILLED: c_uint = 0x00	/* request is ready to be processed */;
pub const SCLP_REQ_QUEUED: c_uint = 0x01	/* request is queued to be processed */;
pub const SCLP_REQ_RUNNING: c_uint = 0x02	/* request is currently running */;
pub const SCLP_REQ_DONE: c_uint = 0x03	/* request is completed successfully */;
pub const SCLP_REQ_FAILED: c_uint = 0x05	/* request is finally failed */;
pub const SCLP_REQ_QUEUED_TIMEOUT: c_uint = 0x06	/* request on queue timed out */;

// function pointers that a high level driver has to use for registration
// of some routines it wants to be called from the low level driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_register {
    pub list: list_head,
// User wants to receive:
    pub receive_mask: sccb_mask_t,
// User wants to send:
    pub send_mask: sccb_mask_t,
// H/W can receive:
    pub sclp_receive_mask: sccb_mask_t,
// H/W can send:
    pub sclp_send_mask: sccb_mask_t,
// called if event type availability changes
    pub ): *mut *mut void (state_change_fn)(struct sclp_register,
// called for events in cp_receive_mask/sclp_receive_mask
    pub ): *mut *mut void (receiver_fn)(struct evbuf_header,
}

// externals from sclp.c
extern "C" {
    pub fn sclp_add_request(req: *mut sclp_req) -> c_int;
}
extern "C" {
    pub fn sclp_sync_wait();
}
extern "C" {
    pub fn sclp_register(reg: *mut sclp_register) -> c_int;
}
extern "C" {
    pub fn sclp_unregister(reg: *mut sclp_register);
}
extern "C" {
    pub fn sclp_remove_processed(sccb: *mut sccb_header) -> c_int;
}
extern "C" {
    pub fn sclp_deactivate() -> c_int;
}
extern "C" {
    pub fn sclp_reactivate() -> c_int;
}
extern "C" {
    pub fn sclp_sync_request(command: sclp_cmdw_t, sccb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sclp_sync_request_timeout(command: sclp_cmdw_t, sccb: *mut c_void, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn sclp_sdias_init() -> c_int;
}
extern "C" {
    pub fn sclp_early_wait_irq();
}
extern "C" {
    pub fn sclp_early_cmd(cmd: sclp_cmdw_t, sccb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sclp_early_con_check_linemode(sccb: *mut init_sccb) -> c_uint;
}
extern "C" {
    pub fn sclp_early_con_check_vt220(sccb: *mut init_sccb) -> c_uint;
}
extern "C" {
    pub fn sclp_early_get_info() -> *mut read_info_sccb  __init;
}
// useful inlines
// Perform service call. Return 0 on success, non-zero otherwise.
// VM uses EBCDIC 037, LPAR+native(SE+HMC) use EBCDIC 500
// translate single character from ASCII to EBCDIC
// translate string from EBCDIC to ASCII
// translate string from ASCII to EBCDIC
