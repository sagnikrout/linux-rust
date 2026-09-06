//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/pn533/pn533.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for NXP PN533 NFC Chip
//
// Copyright (C) 2011 Instituto Nokia de Tecnologia
// Copyright (C) 2012-2013 Tieto Poland
//
pub const PN533_DEVICE_STD: c_uint = 0x1;
pub const PN533_DEVICE_PASORI: c_uint = 0x2;
pub const PN533_DEVICE_ACR122U: c_uint = 0x3;
pub const PN533_DEVICE_PN532: c_uint = 0x4;
pub const PN533_DEVICE_PN532_AUTOPOLL: c_uint = 0x5;

// Standard pn533 frame definitions (standard and extended)

pub const PN533_CMD_DATAEXCH_HEAD_LEN: c_int = 1;
pub const PN533_CMD_DATAEXCH_DATA_MAXLEN: c_int = 262;

//
// Max extended frame payload len, excluding TFI and CC
// which are already in PN533_FRAME_HEADER_LEN.
//
pub const PN533_STD_FRAME_MAX_PAYLOAD_LEN: c_int = 263;
// Preamble (1), SoPC (2), ACK Code (2), Postamble (1)
pub const PN533_STD_FRAME_ACK_SIZE: c_int = 6;
//
// Preamble (1), SoPC (2), Packet Length (1), Packet Length Checksum (1),
// Specific Application Level Error Code (1) , Postamble (1)
//
pub const PN533_STD_ERROR_FRAME_SIZE: c_int = 8;

// Half start code (3), LEN (4) should be 0xffff for extended frame

// start of frame
pub const PN533_STD_FRAME_SOF: c_uint = 0x00FF;
// standard frame identifier: in/out/error

pub const PN533_STD_FRAME_DIR_OUT: c_uint = 0xD4;
pub const PN533_STD_FRAME_DIR_IN: c_uint = 0xD5;
// PN533 Commands

pub const PN533_CMD_GET_FIRMWARE_VERSION: c_uint = 0x02;
pub const PN533_CMD_SAM_CONFIGURATION: c_uint = 0x14;
pub const PN533_CMD_RF_CONFIGURATION: c_uint = 0x32;
pub const PN533_CMD_IN_DATA_EXCHANGE: c_uint = 0x40;
pub const PN533_CMD_IN_COMM_THRU: c_uint = 0x42;
pub const PN533_CMD_IN_LIST_PASSIVE_TARGET: c_uint = 0x4A;
pub const PN533_CMD_IN_ATR: c_uint = 0x50;
pub const PN533_CMD_IN_RELEASE: c_uint = 0x52;
pub const PN533_CMD_IN_JUMP_FOR_DEP: c_uint = 0x56;
pub const PN533_CMD_IN_AUTOPOLL: c_uint = 0x60;
pub const PN533_CMD_TG_INIT_AS_TARGET: c_uint = 0x8c;
pub const PN533_CMD_TG_GET_DATA: c_uint = 0x86;
pub const PN533_CMD_TG_SET_DATA: c_uint = 0x8e;
pub const PN533_CMD_TG_SET_META_DATA: c_uint = 0x94;
pub const PN533_CMD_UNDEF: c_uint = 0xff;

// PN533 Return codes
pub const PN533_CMD_RET_MASK: c_uint = 0x3F;
pub const PN533_CMD_MI_MASK: c_uint = 0x40;
pub const PN533_CMD_RET_SUCCESS: c_uint = 0x00;
pub const PN533_FRAME_DATALEN_ACK: c_uint = 0x00;
pub const PN533_FRAME_DATALEN_ERROR: c_uint = 0x01;
pub const PN533_FRAME_DATALEN_EXTENDED: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pn533_protocol_type {
    PN533_PROTO_REQ_ACK_RESP = 0,
    PN533_PROTO_REQ_RESP
}

// Poll modulations

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_std_frame {
    pub preamble: u8,
    pub start_frame: __be16,
    pub datalen: u8,
    pub datalen_checksum: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_ext_frame {
    pub preamble: u8,
    pub start_frame: __be16,
    pub /: *mut *mut __be16 eif_flag; / fixed to 0xFFFF,
    pub datalen: __be16,
    pub datalen_checksum: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533 {
    pub nfc_dev: *mut nfc_dev,
    pub device_type: u32,
    pub protocol_type: pn533_protocol_type,
    pub resp_q: sk_buff_head,
    pub fragment_skb: sk_buff_head,
    pub wq: *mut workqueue_struct,
    pub cmd_work: work_struct,
    pub cmd_complete_work: work_struct,
    pub poll_work: delayed_work,
    pub mi_rx_work: work_struct,
    pub mi_tx_work: work_struct,
    pub mi_tm_rx_work: work_struct,
    pub mi_tm_tx_work: work_struct,
    pub tg_work: work_struct,
    pub rf_work: work_struct,
    pub cmd_queue: list_head,
    pub cmd: *mut pn533_cmd,
    pub cmd_pending: u8,
    pub /: *mut *mut mutex cmd_lock; / protects cmd queue,
    pub cmd_complete_mi_arg: *mut c_void,
    pub cmd_complete_dep_arg: *mut c_void,
    pub 1]: *mut *mut pn533_poll_modulations poll_mod_active[PN533_POLL_MOD_MAX +,
    pub poll_mod_count: u8,
    pub poll_mod_curr: u8,
    pub poll_dep: u8,
    pub poll_protocols: u32,
    pub listen_protocols: u32,
    pub listen_timer: timer_list,
    pub cancel_listen: c_int,
    pub gb: *mut u8,
    pub gb_len: usize,
    pub tgt_available_prots: u8,
    pub tgt_active_prot: u8,
    pub tgt_mode: u8,
    pub ops: *mut pn533_frame_ops,
    pub dev: *mut device,
    pub phy: *mut c_void,
    pub phy_ops: *const pn533_phy_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_cmd {
    pub queue: list_head,
    pub code: u8,
    pub status: c_int,
    pub req: *mut sk_buff,
    pub resp: *mut sk_buff,
    pub complete_cb: pn533_send_async_complete_t,
    pub complete_cb_context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_frame_ops {
    pub cmd_code): *mut *mut *mut void (tx_frame_init)(void frame, u8,
    pub frame): *mut *mut void (tx_frame_finish)(void,
    pub len): *mut *mut *mut void (tx_update_payload_len)(void frame, int,
    pub tx_header_len: c_int,
    pub tx_tail_len: c_int,
    pub dev): *mut *mut *mut bool (rx_is_frame_valid)(void frame, struct pn533,
    pub frame): *mut *mut bool (rx_frame_is_ack)(void,
    pub frame): *mut *mut int (rx_frame_size)(void,
    pub rx_header_len: c_int,
    pub rx_tail_len: c_int,
    pub max_payload_len: c_int,
    pub frame): *mut *mut u8 (get_cmd_code)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn533_phy_ops {
    pub out): *mut sk_buff,
    pub flags): *mut *mut *mut int (send_ack)(struct pn533 dev, gfp_t,
    pub flags): *mut *mut *mut void (abort_cmd)(struct pn533 priv, gfp_t,
//
// dev_up and dev_down are optional.
// They are used to inform the phy layer that the nfc chip
// is going to be really used very soon. The phy layer can then
// bring up it's interface to the chip and have it suspended for power
// saving reasons otherwise.
//
    pub priv): *mut *mut int (dev_up)(struct pn533,
    pub priv): *mut *mut int (dev_down)(struct pn533,
}

extern "C" {
    pub fn pn533_finalize_setup(dev: *mut pn533) -> c_int;
}
extern "C" {
    pub fn pn53x_common_clean(priv: *mut pn533);
}
extern "C" {
    pub fn pn533_recv_frame(dev: *mut pn533, skb: *mut sk_buff, status: c_int);
}
extern "C" {
    pub fn pn53x_unregister_nfc(priv: *mut pn533);
}
extern "C" {
    pub fn pn533_rx_frame_is_cmd_response(dev: *mut pn533, frame: *mut c_void) -> bool;
}
extern "C" {
    pub fn pn533_rx_frame_is_ack(_frame: *mut c_void) -> bool;
}
