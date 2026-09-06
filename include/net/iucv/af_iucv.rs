//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/iucv/af_iucv.h
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
// Copyright 2006 IBM Corporation
// IUCV protocol stack for Linux on zSeries
// Version 1.0
// Author(s): Jennifer Hunt <jenhunt@us.ibm.com>
//

pub const AF_IUCV: c_int = 32;

// Connection and socket states
pub const IUCV_QUEUELEN_DEFAULT: c_int = 65535;
pub const IUCV_HIPER_MSGLIM_DEFAULT: c_int = 128;

pub const IUCV_BUFSIZE_DEFAULT: c_int = 32768;
// IUCV socket address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_iucv {
    pub siucv_family: sa_family_t,
    pub /: *mut *mut unsigned short siucv_port; / Reserved,
    pub /: *mut *mut unsigned int siucv_addr; / Reserved,
    pub /: *mut *mut char siucv_nodeid[8]; / Reserved,
    pub /: *mut *mut char siucv_user_id[8]; / Guest User Id,
    pub /: *mut *mut char siucv_name[8]; / Application Name,
}

// Common socket structures and functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_msg_q {
    pub path: *mut iucv_path,
    pub msg: iucv_message,
    pub list: list_head,
    pub lock: spinlock_t,
}

pub const AF_IUCV_FLAG_ACK: c_uint = 0x1;
pub const AF_IUCV_FLAG_SYN: c_uint = 0x2;
pub const AF_IUCV_FLAG_FIN: c_uint = 0x4;
pub const AF_IUCV_FLAG_WIN: c_uint = 0x8;
pub const AF_IUCV_FLAG_SHT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct af_iucv_trans_hdr {
    pub magic: u16,
    pub version: u8,
    pub flags: u8,
    pub window: u16,
    pub destNodeID: [c_char; 8],
    pub destUserID: [c_char; 8],
    pub destAppName: [c_char; 16],
    pub srcNodeID: [c_char; 8],
    pub srcUserID: [c_char; 8],
    pub /: *mut *mut char srcAppName[16]; / => 70 bytes,
    pub /: *mut *mut iucv_message iucv_hdr; / => 33 bytes,
    pub /: *mut *mut u8 pad; / total 104 bytes,
    pub __packed: },
    pub )skb_network_header(skb): *mut return (struct af_iucv_trans_hdr,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iucv_tx_notify {
// transmission of skb is completed and was successful
    TX_NOTIFY_OK = 0,
// target is unreachable
    TX_NOTIFY_UNREACHABLE = 1,
// transfer pending queue full
    TX_NOTIFY_TPQFULL = 2,
// general error
    TX_NOTIFY_GENERALERROR = 3,
// transmission of skb is pending - may interleave
// with TX_NOTIFY_DELAYED_*
    TX_NOTIFY_PENDING = 4,
// transmission of skb was done successfully (delayed)
    TX_NOTIFY_DELAYED_OK = 5,
// target unreachable (detected delayed)
    TX_NOTIFY_DELAYED_UNREACHABLE = 6,
// general error (detected delayed)
    TX_NOTIFY_DELAYED_GENERALERROR = 7,
}

pub const AF_IUCV_TRANS_IUCV: c_int = 0;
pub const AF_IUCV_TRANS_HIPER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_sock {
    pub sk: sock,
    pub src_user_id: [c_char; 8],
    pub src_name: [c_char; 8],
    pub dst_user_id: [c_char; 8],
    pub dst_name: [c_char; 8],
    pub accept_q: list_head,
    pub accept_q_lock: spinlock_t,
    pub parent: *mut sock,
    pub path: *mut iucv_path,
    pub hs_dev: *mut net_device,
    pub send_skb_q: sk_buff_head,
    pub backlog_skb_q: sk_buff_head,
    pub message_q: sock_msg_q,
    pub send_tag: c_uint,
    pub flags: u8,
    pub msglimit: u16,
    pub msglimit_peer: u16,
    pub skbs_in_xmit: core::sync::atomic::AtomicI32,
    pub msg_sent: core::sync::atomic::AtomicI32,
    pub msg_recv: core::sync::atomic::AtomicI32,
    pub pendings: core::sync::atomic::AtomicI32,
    pub transport: c_int,
    pub n): iucv_tx_notify,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_skb_cb {
    pub /: *mut *mut u32 class; / target class of message,
    pub /: *mut *mut u32 tag; / tag associated with message,
    pub /: *mut *mut u32 offset; / offset for skb receival,
}

// iucv socket options (SOL_IUCV)
pub const SO_IPRMDATA_MSG: c_uint = 0x0080		/* send/recv IPRM_DATA msgs */;
pub const SO_MSGLIMIT: c_uint = 0x1000		/* get/set IUCV MSGLIMIT */;
pub const SO_MSGSIZE: c_uint = 0x0800		/* get maximum msgsize */;
// iucv related control messages (scm)
pub const SCM_IUCV_TRGCLS: c_uint = 0x0001		/* target class control message */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,
    pub autobind_name: core::sync::atomic::AtomicI32,
}
