//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/rfcomm.h
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

pub const RFCOMM_DEFAULT_MTU: c_int = 127;
pub const RFCOMM_DEFAULT_CREDITS: c_int = 7;
pub const RFCOMM_MAX_CREDITS: c_int = 40;
pub const RFCOMM_SKB_HEAD_RESERVE: c_int = 8;
pub const RFCOMM_SKB_TAIL_RESERVE: c_int = 2;

pub const RFCOMM_SABM: c_uint = 0x2f;
pub const RFCOMM_DISC: c_uint = 0x43;
pub const RFCOMM_UA: c_uint = 0x63;
pub const RFCOMM_DM: c_uint = 0x0f;
pub const RFCOMM_UIH: c_uint = 0xef;
pub const RFCOMM_TEST: c_uint = 0x08;
pub const RFCOMM_FCON: c_uint = 0x28;
pub const RFCOMM_FCOFF: c_uint = 0x18;
pub const RFCOMM_MSC: c_uint = 0x38;
pub const RFCOMM_RPN: c_uint = 0x24;
pub const RFCOMM_RLS: c_uint = 0x14;
pub const RFCOMM_PN: c_uint = 0x20;
pub const RFCOMM_NSC: c_uint = 0x04;
pub const RFCOMM_V24_FC: c_uint = 0x02;
pub const RFCOMM_V24_RTC: c_uint = 0x04;
pub const RFCOMM_V24_RTR: c_uint = 0x08;
pub const RFCOMM_V24_IC: c_uint = 0x40;
pub const RFCOMM_V24_DV: c_uint = 0x80;
pub const RFCOMM_RPN_BR_2400: c_uint = 0x0;
pub const RFCOMM_RPN_BR_4800: c_uint = 0x1;
pub const RFCOMM_RPN_BR_7200: c_uint = 0x2;
pub const RFCOMM_RPN_BR_9600: c_uint = 0x3;
pub const RFCOMM_RPN_BR_19200: c_uint = 0x4;
pub const RFCOMM_RPN_BR_38400: c_uint = 0x5;
pub const RFCOMM_RPN_BR_57600: c_uint = 0x6;
pub const RFCOMM_RPN_BR_115200: c_uint = 0x7;
pub const RFCOMM_RPN_BR_230400: c_uint = 0x8;
pub const RFCOMM_RPN_DATA_5: c_uint = 0x0;
pub const RFCOMM_RPN_DATA_6: c_uint = 0x1;
pub const RFCOMM_RPN_DATA_7: c_uint = 0x2;
pub const RFCOMM_RPN_DATA_8: c_uint = 0x3;
pub const RFCOMM_RPN_STOP_1: c_int = 0;
pub const RFCOMM_RPN_STOP_15: c_int = 1;
pub const RFCOMM_RPN_PARITY_NONE: c_uint = 0x0;
pub const RFCOMM_RPN_PARITY_ODD: c_uint = 0x1;
pub const RFCOMM_RPN_PARITY_EVEN: c_uint = 0x3;
pub const RFCOMM_RPN_PARITY_MARK: c_uint = 0x5;
pub const RFCOMM_RPN_PARITY_SPACE: c_uint = 0x7;
pub const RFCOMM_RPN_FLOW_NONE: c_uint = 0x00;
pub const RFCOMM_RPN_XON_CHAR: c_uint = 0x11;
pub const RFCOMM_RPN_XOFF_CHAR: c_uint = 0x13;
pub const RFCOMM_RPN_PM_BITRATE: c_uint = 0x0001;
pub const RFCOMM_RPN_PM_DATA: c_uint = 0x0002;
pub const RFCOMM_RPN_PM_STOP: c_uint = 0x0004;
pub const RFCOMM_RPN_PM_PARITY: c_uint = 0x0008;
pub const RFCOMM_RPN_PM_PARITY_TYPE: c_uint = 0x0010;
pub const RFCOMM_RPN_PM_XON: c_uint = 0x0020;
pub const RFCOMM_RPN_PM_XOFF: c_uint = 0x0040;
pub const RFCOMM_RPN_PM_FLOW: c_uint = 0x3F00;
pub const RFCOMM_RPN_PM_ALL: c_uint = 0x3F7F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_hdr {
    pub addr: u8,
    pub ctrl: u8,
    pub /: *mut *mut u8 len; / Actual size can be 2 bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_cmd {
    pub addr: u8,
    pub ctrl: u8,
    pub len: u8,
    pub fcs: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_mcc {
    pub type: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_pn {
    pub dlci: u8,
    pub flow_ctrl: u8,
    pub priority: u8,
    pub ack_timer: u8,
    pub mtu: __le16,
    pub max_retrans: u8,
    pub credits: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_rpn {
    pub dlci: u8,
    pub bit_rate: u8,
    pub line_settings: u8,
    pub flow_ctrl: u8,
    pub xon_char: u8,
    pub xoff_char: u8,
    pub param_mask: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_rls {
    pub dlci: u8,
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_msc {
    pub dlci: u8,
    pub v24_sig: u8,
    pub __packed: },
// ---- Core structures, flags etc ----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_session {
    pub list: list_head,
    pub sock: *mut socket,
    pub timer: timer_list,
    pub state: c_ulong,
    pub flags: c_ulong,
    pub initiator: c_int,
// Default DLC parameters
    pub cfc: c_int,
    pub mtu: c_uint,
    pub dlcs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_dlc {
    pub list: list_head,
    pub session: *mut rfcomm_session,
    pub tx_queue: sk_buff_head,
    pub timer: timer_list,
    pub lock: mutex,
    pub state: c_ulong,
    pub flags: c_ulong,
    pub refcnt: refcount_t,
    pub dlci: u8,
    pub addr: u8,
    pub priority: u8,
    pub v24_sig: u8,
    pub remote_v24_sig: u8,
    pub mscex: u8,
    pub out: u8,
    pub sec_level: u8,
    pub role_switch: u8,
    pub defer_setup: u32,
    pub mtu: c_uint,
    pub cfc: c_uint,
    pub rx_credits: c_uint,
    pub tx_credits: c_uint,
    pub owner: *mut c_void,
    pub skb): *mut *mut *mut void (data_ready)(struct rfcomm_dlc d, struct sk_buff,
    pub err): *mut *mut *mut void (state_change)(struct rfcomm_dlc d, int,
    pub v24_sig): *mut *mut *mut void (modem_status)(struct rfcomm_dlc d, u8,
}

// DLC and session flags
pub const RFCOMM_RX_THROTTLED: c_int = 0;
pub const RFCOMM_TX_THROTTLED: c_int = 1;
pub const RFCOMM_TIMED_OUT: c_int = 2;
pub const RFCOMM_MSC_PENDING: c_int = 3;
pub const RFCOMM_SEC_PENDING: c_int = 4;
pub const RFCOMM_AUTH_PENDING: c_int = 5;
pub const RFCOMM_AUTH_ACCEPT: c_int = 6;
pub const RFCOMM_AUTH_REJECT: c_int = 7;
pub const RFCOMM_DEFER_SETUP: c_int = 8;
pub const RFCOMM_ENC_DROP: c_int = 9;
// Scheduling flags and events
pub const RFCOMM_SCHED_WAKEUP: c_int = 31;
// MSC exchange flags
pub const RFCOMM_MSCEX_TX: c_int = 1;
pub const RFCOMM_MSCEX_RX: c_int = 2;

// CFC states

pub const RFCOMM_CFC_DISABLED: c_int = 0;

// ---- RFCOMM SEND RPN ----
// ---- RFCOMM DLCs (channels) ----
extern "C" {
    pub fn rfcomm_dlc_free(d: *mut rfcomm_dlc);
}
extern "C" {
    pub fn rfcomm_dlc_close(d: *mut rfcomm_dlc, reason: c_int) -> c_int;
}
extern "C" {
    pub fn rfcomm_dlc_send(d: *mut rfcomm_dlc, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rfcomm_dlc_send_noerror(d: *mut rfcomm_dlc, skb: *mut sk_buff);
}
extern "C" {
    pub fn rfcomm_dlc_set_modem_status(d: *mut rfcomm_dlc, v24_sig: u8) -> c_int;
}
extern "C" {
    pub fn rfcomm_dlc_get_modem_status(d: *mut rfcomm_dlc, v24_sig: *mut u8) -> c_int;
}
extern "C" {
    pub fn rfcomm_dlc_accept(d: *mut rfcomm_dlc);
}

extern "C" {
    pub fn __rfcomm_dlc_throttle(d: *mut rfcomm_dlc);
}
extern "C" {
    pub fn __rfcomm_dlc_unthrottle(d: *mut rfcomm_dlc);
}
// ---- RFCOMM sessions ----
// ---- RFCOMM sockets ----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_rc {
    pub rc_family: sa_family_t,
    pub rc_bdaddr: bdaddr_t,
    pub rc_channel: u8,
}

pub const RFCOMM_CONNINFO: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_conninfo {
    pub hci_handle: __u16,
    pub dev_class: [__u8; 3],
}

pub const RFCOMM_LM: c_uint = 0x03;
pub const RFCOMM_LM_MASTER: c_uint = 0x0001;
pub const RFCOMM_LM_AUTH: c_uint = 0x0002;
pub const RFCOMM_LM_ENCRYPT: c_uint = 0x0004;
pub const RFCOMM_LM_TRUSTED: c_uint = 0x0008;
pub const RFCOMM_LM_RELIABLE: c_uint = 0x0010;
pub const RFCOMM_LM_SECURE: c_uint = 0x0020;
pub const RFCOMM_LM_FIPS: c_uint = 0x0040;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_pinfo {
    pub bt: bt_sock,
    pub src: bdaddr_t,
    pub dst: bdaddr_t,
    pub dlc: *mut rfcomm_dlc,
    pub channel: u8,
    pub sec_level: u8,
    pub role_switch: u8,
}

extern "C" {
    pub fn rfcomm_init_sockets() -> c_int;
}
extern "C" {
    pub fn rfcomm_cleanup_sockets();
}
// ---- RFCOMM TTY ----
pub const RFCOMM_MAX_DEV: c_int = 256;

// rfcomm_dev.flags bit definitions
pub const RFCOMM_REUSE_DLC: c_int = 0;
pub const RFCOMM_RELEASE_ONHUP: c_int = 1;
pub const RFCOMM_HANGUP_NOW: c_int = 2;
pub const RFCOMM_TTY_ATTACHED: c_int = 3;

// rfcomm_dev.status bit definitions
pub const RFCOMM_DEV_RELEASED: c_int = 0;
pub const RFCOMM_TTY_OWNED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_dev_req {
    pub dev_id: i16,
    pub flags: u32,
    pub src: bdaddr_t,
    pub dst: bdaddr_t,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_dev_info {
    pub id: i16,
    pub flags: u32,
    pub state: u16,
    pub src: bdaddr_t,
    pub dst: bdaddr_t,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfcomm_dev_list_req {
    pub dev_num: u16,
    pub __counted_by(dev_num): rfcomm_dev_info dev_info[],
}

extern "C" {
    pub fn rfcomm_dev_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut void __user) -> c_int;
}

extern "C" {
    pub fn rfcomm_init_ttys() -> c_int;
}
extern "C" {
    pub fn rfcomm_cleanup_ttys();
}

