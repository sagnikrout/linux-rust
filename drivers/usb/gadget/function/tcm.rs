//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/tcm.h
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

// #include <linux/usb/uas.h>

pub const USBG_NAMELEN: c_int = 32;

pub const UASP_SS_EP_COMP_LOG_STREAMS: c_int = 5;

pub const USB_G_ALT_INT_BBB: c_int = 0;
pub const USB_G_ALT_INT_UAS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_usbg_nexus {
    pub tvn_se_sess: *mut se_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbg_tpg {
    pub tpg_mutex: mutex,
// SAS port target portal group tag for TCM
    pub tport_tpgt: u16,
// Pointer back to usbg_tport
    pub tport: *mut usbg_tport,
    pub workqueue: *mut workqueue_struct,
// Returned by usbg_make_tpg()
    pub se_tpg: se_portal_group,
    pub gadget_connect: u32,
    pub tpg_nexus: *mut tcm_usbg_nexus,
    pub tpg_port_count: core::sync::atomic::AtomicI32,
    pub fi: *mut usb_function_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbg_tport {
// Binary World Wide unique Port Name for SAS Target port
    pub tport_wwpn: u64,
// ASCII formatted WWPN for SAS Target port
    pub tport_name: [c_char; USBG_NAMELEN],
// Returned by usbg_make_tport()
    pub tport_wwn: se_wwn,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uas_state {
    UASP_SEND_DATA,
    UASP_RECEIVE_DATA,
    UASP_SEND_STATUS,
    UASP_QUEUE_COMMAND,
}

pub const USBG_MAX_CMD: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbg_cmd {
// common
    pub cmd_buf: [u8; USBG_MAX_CMD],
    pub data_len: u32,
    pub work: work_struct,
    pub unpacked_lun: c_int,
    pub se_cmd: se_cmd,
    pub /: *mut *mut *mut void data_buf; / used if no sg support available,
    pub fu: *mut f_uas,
    pub ref: kref,
    pub req: *mut usb_request,
    pub flags: u32,

// UAS only
    pub tag: u16,
    pub prio_attr: u16,
    pub sense_iu: sense_iu,
    pub response_iu: response_iu,
    pub state: uas_state,
    pub tmr_func: c_int,
    pub tmr_rsp: c_int,
pub const RC_RESPONSE_UNKNOWN: c_uint = 0xff;
// BOT only
    pub bot_tag: __le32,
    pub csw_code: c_uint,
    pub is_read:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uas_stream {
    pub req_in: *mut usb_request,
    pub req_out: *mut usb_request,
    pub req_status: *mut usb_request,
    pub cmd_completion: completion,
    pub node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbg_cdb {
    pub req: *mut usb_request,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_status {
    pub req: *mut usb_request,
    pub csw: bulk_cs_wrap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_uas {
    pub tpg: *mut usbg_tpg,
    pub function: usb_function,
    pub iface: u16,
    pub flags: u32,

    pub delayed_set_alt: work_struct,
    pub /: *mut *mut *mut spinlock_t delayed_set_alt_lock; / protects delayed_set_alt_,
    pub delayed_alt: c_uint,
    pub delayed_set_alt_state: c_uint,
    pub delayed_set_alt_cancel: bool,
    pub cmd: [usbg_cdb; USBG_NUM_CMDS],
    pub ep_in: *mut usb_ep,
    pub ep_out: *mut usb_ep,
// UAS
    pub ep_status: *mut usb_ep,
    pub ep_cmd: *mut usb_ep,
    pub stream: [uas_stream; USBG_NUM_CMDS],
    pub UASP_SS_EP_COMP_LOG_STREAMS): DECLARE_HASHTABLE(stream_hash,,
// BOT
    pub bot_status: bot_status,
    pub bot_req_in: *mut usb_request,
    pub bot_req_out: *mut usb_request,
}
