//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/ctcm_main.h
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
// Copyright IBM Corp. 2001, 2007
// Authors:	Fritz Elfert (felfert@millenux.com)
// Peter Tiedemann (ptiedem@de.ibm.com)
//

pub const CHANNEL_FLAGS_READ: c_int = 0;
pub const CHANNEL_FLAGS_WRITE: c_int = 1;
pub const CHANNEL_FLAGS_INUSE: c_int = 2;
pub const CHANNEL_FLAGS_BUFSIZE_CHANGED: c_int = 4;
pub const CHANNEL_FLAGS_FAILED: c_int = 8;
pub const CHANNEL_FLAGS_WAITIRQ: c_int = 16;
pub const CHANNEL_FLAGS_RWMASK: c_int = 1;

pub const LOG_FLAG_ILLEGALPKT: c_int = 1;
pub const LOG_FLAG_ILLEGALSIZE: c_int = 2;
pub const LOG_FLAG_OVERRUN: c_int = 4;
pub const LOG_FLAG_NOMEM: c_int = 8;

//
// Enum for classifying detected devices
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctcm_channel_types {
// Device is not a channel
    ctcm_channel_type_none,

// Device is a CTC/A
    ctcm_channel_type_parallel,

// Device is a FICON channel
    ctcm_channel_type_ficon,

// Device is a ESCON channel
    ctcm_channel_type_escon
}

//
// CCW commands, used in this driver.
//
pub const CCW_CMD_WRITE: c_uint = 0x01;
pub const CCW_CMD_READ: c_uint = 0x02;
pub const CCW_CMD_NOOP: c_uint = 0x03;
pub const CCW_CMD_TIC: c_uint = 0x08;
pub const CCW_CMD_SENSE_CMD: c_uint = 0x14;
pub const CCW_CMD_WRITE_CTL: c_uint = 0x17;
pub const CCW_CMD_SET_EXTENDED: c_uint = 0xc3;
pub const CCW_CMD_PREPARE: c_uint = 0xe3;
pub const CTCM_PROTO_S390: c_int = 0;
pub const CTCM_PROTO_LINUX: c_int = 1;
pub const CTCM_PROTO_LINUX_TTY: c_int = 2;
pub const CTCM_PROTO_OS390: c_int = 3;
pub const CTCM_PROTO_MPC: c_int = 4;
pub const CTCM_PROTO_MAX: c_int = 4;
pub const CTCM_STATSIZE_LIMIT: c_int = 64;
pub const CTCM_BUFSIZE_LIMIT: c_int = 65535;
pub const CTCM_BUFSIZE_DEFAULT: c_int = 32768;

pub const CTCM_TIME_1_SEC: c_int = 1000;
pub const CTCM_TIME_5_SEC: c_int = 5000;
pub const CTCM_TIME_10_SEC: c_int = 10000;
pub const CTCM_INITIAL_BLOCKLEN: c_int = 2;
pub const CTCM_READ: c_int = 0;
pub const CTCM_WRITE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcm_profile {
    pub maxmulti: c_ulong,
    pub maxcqueue: c_ulong,
    pub doios_single: c_ulong,
    pub doios_multi: c_ulong,
    pub txlen: c_ulong,
    pub tx_time: c_ulong,
    pub send_stamp: c_ulong,
}

//
// Definition of one channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel {
    pub next: *mut channel,
    pub id: [c_char; CTCM_ID_SIZE],
    pub cdev: *mut ccw_device,
//
// Type of this channel.
// CTC/A or Escon for valid channels.
//
    pub type: ctcm_channel_types,
//
// Misc. flags. See CHANNEL_FLAGS_... below
//
    pub flags: __u32,
    pub /: *mut *mut __u16 protocol; / protocol of this channel (4 = MPC),
//
// I/O and irq related stuff
//
    pub ccw: *mut ccw1,
    pub irb: *mut irb,
//
// RX/TX buffer size
//
    pub max_bufsize: c_int,
    pub /: *mut *mut *mut sk_buff trans_skb; / transmit/receive buffer,
    pub /: *mut *mut sk_buff_head io_queue; / universal I/O queue,
    pub /: *mut *mut tasklet_ch_tasklet; / MPC ONLY,
//
// TX queue for collecting skb's during busy.
//
    pub collect_queue: sk_buff_head,
//
// Amount of data in collect_queue.
//
    pub collect_len: c_int,
//
// spinlock for collect_queue and collect_len
//
    pub collect_lock: spinlock_t,
//
// Timer for detecting unresposive
// I/O operations.
//
    pub timer: fsm_timer,
// MPC ONLY section begin
    pub /: *mut *mut __u32 th_seq_num; / SNA TH seq number,
    pub th_seg: __u8,
    pub pdu_seq: __u32,
    pub xid_skb: *mut sk_buff,
    pub xid_skb_data: *mut c_char,
    pub xid_th: *mut th_header,
    pub xid: *mut xid2,
    pub xid_id: *mut c_char,
    pub rcvd_xid_th: *mut th_header,
    pub rcvd_xid: *mut xid2,
    pub rcvd_xid_id: *mut c_char,
    pub in_mpcgroup: __u8,
    pub sweep_timer: fsm_timer,
    pub sweep_queue: sk_buff_head,
    pub discontact_th: *mut th_header,
    pub ch_disc_tasklet: tasklet_struct,
// MPC ONLY section end
    pub /: *mut *mut int retry; / retry counter for misc. operations,
    pub /: *mut *mut *mut fsm_instance fsm; / finite state machine of this channel,
    pub /: *mut *mut *mut net_device netdev; / corresponding net_device,
    pub prof: ctcm_profile,
    pub trans_skb_data: *mut __u8,
    pub logflags: __u16,
    pub /: *mut *mut __u8 sense_rc; / last unit check sense code report control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcm_priv {
    pub stats: net_device_stats,
    pub tbusy: c_ulong,
// The MPC group struct of this interface
    pub /: *mut *mut *mut mpc_group mpcg; / MPC only,
    pub /: *mut *mut *mut xid2 xid; / MPC only,
// The finite state machine of this interface
    pub fsm: *mut fsm_instance,
// The protocol of this device
    pub protocol: __u16,
// Timer for restarting after I/O Errors
    pub restart_timer: fsm_timer,
    pub /: *mut *mut int buffer_size; / ctc only,
    pub channel: [*mut channel; 2],
}

extern "C" {
    pub fn ctcm_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ctcm_close(dev: *mut net_device) -> c_int;
}
//
// Compatibility macros for busy handling
// of network devices.
//
extern "C" {
    pub fn ctcm_unpack_skb(ch: *mut channel, pskb: *mut sk_buff);
}
//
// Functions related to setup and device detection.
//
extern "C" {
    pub fn ctcm_ch_alloc_buffer(ch: *mut channel) -> c_int;
}
extern "C" {
    pub fn ctcm_ch_alloc_buffer(_arg: ch) -> return;
}
extern "C" {
    pub fn ctcm_ch_alloc_buffer(_arg: ch) -> return;
}
// test if protocol attribute (of struct ctcm_priv or struct channel)
// has MPC protocol setting. Type is not checked
//

// test if struct ctcm_priv of struct net_device has MPC protocol setting

//
// Definition of our link level header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ll_header {
    pub length: __u16,
    pub type: __u16,
    pub unused: __u16,
}

