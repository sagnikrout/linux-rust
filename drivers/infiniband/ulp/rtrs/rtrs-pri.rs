//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/rtrs/rtrs-pri.h
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
// RDMA Transport Layer
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

pub const RTRS_PROTO_VER_MAJOR: c_int = 2;
pub const RTRS_PROTO_VER_MINOR: c_int = 0;

//
// Max IB immediate data size is 2^28 (MAX_IMM_PAYL_BITS)
// and the minimum chunk size is 4096 (2^12).
// So the maximum sess_queue_depth is 65535 (2^16 - 1) in theory
// since queue_depth in rtrs_msg_conn_rsp is defined as le16.
// Therefore the pratical max value of sess_queue_depth is
// somewhere between 1 and 65535 and it depends on the system.
//
pub const MAX_SESS_QUEUE_DEPTH: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_imm_const {
    MAX_IMM_TYPE_BITS = 4,
    MAX_IMM_TYPE_MASK = ((1 << MAX_IMM_TYPE_BITS) - 1),
    MAX_IMM_PAYL_BITS = 28,
    MAX_IMM_PAYL_MASK = ((1 << MAX_IMM_PAYL_BITS) - 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_imm_type {
    RTRS_IO_REQ_IMM       = 0, /* client to server */
    RTRS_IO_RSP_IMM       = 1, /* server to client */
    RTRS_IO_RSP_W_INV_IMM = 2, /* server to client */

    RTRS_HB_MSG_IMM = 8, /* HB: HeartBeat */
    RTRS_HB_ACK_IMM = 9,

    RTRS_LAST_IMM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_rdma_dev_pd_ops {
    pub dev): *mut *mut int (init)(struct rtrs_ib_dev,
    pub dev): *mut *mut void (deinit)(struct rtrs_ib_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_rdma_dev_pd {
    pub mutex: mutex,
    pub list: list_head,
    pub pd_flags: ib_pd_flags,
    pub ops: *const rtrs_rdma_dev_pd_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_ib_dev {
    pub ib_dev: *mut ib_device,
    pub ib_pd: *mut ib_pd,
    pub ref: kref,
    pub entry: list_head,
    pub pool: *mut rtrs_rdma_dev_pd,
    pub event_handler: ib_event_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_con {
    pub path: *mut rtrs_path,
    pub qp: *mut ib_qp,
    pub cq: *mut ib_cq,
    pub cm_id: *mut rdma_cm_id,
    pub cid: c_uint,
    pub nr_cqe: c_int,
    pub wr_cnt: core::sync::atomic::AtomicI32,
    pub sq_wr_avail: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_path {
    pub entry: list_head,
    pub dst_addr: sockaddr_storage,
    pub src_addr: sockaddr_storage,
    pub sessname: [c_char; NAME_MAX],
    pub uuid: uuid_t,
    pub con: *mut rtrs_con,
    pub con_num: c_uint,
    pub irq_con_num: c_uint,
    pub recon_cnt: c_uint,
    pub signal_interval: c_uint,
    pub dev: *mut rtrs_ib_dev,
    pub dev_ref: c_int,
    pub hb_cqe: *mut ib_cqe,
    pub con): *mut *mut void (hb_err_handler)(struct rtrs_con,
    pub hb_wq: *mut workqueue_struct,
    pub hb_dwork: delayed_work,
    pub hb_interval_ms: c_uint,
    pub hb_missed_cnt: c_uint,
    pub hb_missed_max: c_uint,
    pub hb_last_sent: ktime_t,
    pub hb_cur_latency: ktime_t,
}

// rtrs information unit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_iu {
    pub cqe: ib_cqe,
    pub dma_addr: dma_addr_t,
    pub buf: *mut c_void,
    pub size: usize,
    pub direction: dma_data_direction,
}

//
// enum rtrs_msg_types - RTRS message types, see also rtrs/README
// @RTRS_MSG_INFO_REQ:		Client additional info request to the server
// @RTRS_MSG_INFO_RSP:		Server additional info response to the client
// @RTRS_MSG_WRITE:		Client writes data per RDMA to server
// @RTRS_MSG_READ:		Client requests data transfer from server
// @RTRS_MSG_RKEY_RSP:		Server refreshed rkey for rbuf
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_msg_types {
    RTRS_MSG_INFO_REQ,
    RTRS_MSG_INFO_RSP,
    RTRS_MSG_WRITE,
    RTRS_MSG_READ,
    RTRS_MSG_RKEY_RSP,
}

//
// enum rtrs_msg_flags - RTRS message flags.
// @RTRS_MSG_NEED_INVAL_F: Send invalidation in response.
// @RTRS_MSG_NEW_RKEY_F: Send refreshed rkey in response.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_msg_flags {
    RTRS_MSG_NEED_INVAL_F = 1 << 0,
    RTRS_MSG_NEW_RKEY_F = 1 << 1,
}

//
// struct rtrs_sg_desc - RDMA-Buffer entry description
// @addr:	Address of RDMA destination buffer
// @key:	Authorization rkey to write to the buffer
// @len:	Size of the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_sg_desc {
    pub addr: __le64,
    pub key: __le32,
    pub len: __le32,
}

//
// struct rtrs_msg_conn_req - Client connection request to the server
// @magic:	   RTRS magic
// @version:	   RTRS protocol version
// @cid:	   Current connection id
// @cid_num:	   Number of connections per session
// @recon_cnt:	   Reconnections counter
// @sess_uuid:	   UUID of a session (path)
// @paths_uuid:	   UUID of a group of sessions (paths)
// @first_conn:    %1 if the connection request is the first for that session,
// otherwise %0
// NOTE: max size 56 bytes, see man rdma_connect().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_conn_req {
//
// @__cma_version: Is set to 0 by cma.c in case of AF_IB, do not touch
// that. See https://www.spinics.net/lists/linux-rdma/msg22397.html
//
    pub __cma_version: u8,
//
// @__ip_version: On sender side that should be set to 0, or
// cma_save_ip_info() extract garbage and will fail.
//
    pub __ip_version: u8,
    pub magic: __le16,
    pub version: __le16,
    pub cid: __le16,
    pub cid_num: __le16,
    pub recon_cnt: __le16,
    pub sess_uuid: uuid_t,
    pub paths_uuid: uuid_t,
    pub 1: u8 first_conn :,
// private:
    pub 7: u8 reserved_bits :,
    pub reserved: [u8; 11],
}

//
// struct rtrs_msg_conn_rsp - Server connection response to the client
// @magic:	   RTRS magic
// @version:	   RTRS protocol version
// @errno:	   If rdma_accept() then 0, if rdma_reject() indicates error
// @queue_depth:   max inflight messages (queue-depth) in this session
// @max_io_size:   max io size server supports
// @max_hdr_size:  max msg header size server supports
// @flags:	   RTRS message flags for this message
//
// NOTE: size is 56 bytes, max possible is 136 bytes, see man rdma_accept().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_conn_rsp {
    pub magic: __le16,
    pub version: __le16,
    pub errno: __le16,
    pub queue_depth: __le16,
    pub max_io_size: __le32,
    pub max_hdr_size: __le32,
    pub flags: __le32,
// private:
    pub reserved: [u8; 36],
}

//
// struct rtrs_msg_info_req - client additional info request
// @type:		@RTRS_MSG_INFO_REQ
// @pathname:		Path name chosen by client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_info_req {
    pub type: __le16,
    pub pathname: [u8; NAME_MAX],
// private:
    pub reserved: [u8; 15],
}

//
// struct rtrs_msg_info_rsp - server additional info response
// @type:		@RTRS_MSG_INFO_RSP
// @sg_cnt:		Number of @desc entries
// @desc:		RDMA buffers where the client can write to server
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_info_rsp {
    pub type: __le16,
    pub sg_cnt: __le16,
// private:
    pub reserved: [u8; 4],
// public:
    pub desc: [rtrs_sg_desc; ],
}

//
// struct rtrs_msg_rkey_rsp - server refreshed rkey response
// @type:		@RTRS_MSG_RKEY_RSP
// @buf_id:		RDMA buf_id of the new rkey
// @rkey:		new remote key for RDMA buffers id from server
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_rkey_rsp {
    pub type: __le16,
    pub buf_id: __le16,
    pub rkey: __le32,
}

//
// struct rtrs_msg_rdma_read - RDMA data transfer request from client
// @type:		always @RTRS_MSG_READ
// @flags:		RTRS message flags (enum rtrs_msg_flags)
// @usr_len:		length of user payload
// @sg_cnt:		number of @desc entries
// @desc:		RDMA buffers where the server can write the result to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_rdma_read {
    pub type: __le16,
    pub usr_len: __le16,
    pub flags: __le16,
    pub sg_cnt: __le16,
    pub desc: [rtrs_sg_desc; ],
}

//
// struct rtrs_msg_rdma_write - Message transferred to server with RDMA-Write
// @type:		always @RTRS_MSG_WRITE
// @usr_len:		length of user payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_rdma_write {
    pub type: __le16,
    pub usr_len: __le16,
}

//
// struct rtrs_msg_rdma_hdr - header for read or write request
// @type:		@RTRS_MSG_WRITE | @RTRS_MSG_READ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_msg_rdma_hdr {
    pub type: __le16,
}

// rtrs.c
extern "C" {
    pub fn rtrs_iu_free(iu: *mut rtrs_iu, dev: *mut ib_device, queue_num: u32);
}
extern "C" {
    pub fn rtrs_iu_post_recv(con: *mut rtrs_con, iu: *mut rtrs_iu) -> c_int;
}
extern "C" {
    pub fn rtrs_post_recv_empty(con: *mut rtrs_con, cqe: *mut ib_cqe) -> c_int;
}
extern "C" {
    pub fn rtrs_cq_qp_destroy(con: *mut rtrs_con);
}
extern "C" {
    pub fn rtrs_start_hb(path: *mut rtrs_path);
}
extern "C" {
    pub fn rtrs_stop_hb(path: *mut rtrs_path);
}
extern "C" {
    pub fn rtrs_send_hb_ack(path: *mut rtrs_path);
}
extern "C" {
    pub fn rtrs_rdma_dev_pd_deinit(pool: *mut rtrs_rdma_dev_pd);
}
extern "C" {
    pub fn rtrs_ib_dev_put(dev: *mut rtrs_ib_dev) -> c_int;
}
// payload = imm & MAX_IMM_PAYL_MASK;
// type = imm >> MAX_IMM_PAYL_BITS;
extern "C" {
    pub fn rtrs_to_imm(_arg: RTRS_IO_REQ_IMM, _arg: addr) -> return;
}
// 9 bits for errno, 19 bits for msg_id
extern "C" {
    pub fn rtrs_to_imm(_arg: type, _arg: payload) -> return;
}
// 9 bits for errno, 19 bits for msg_id
// msg_id = payload & 0x7ffff;
// errno = -(int)((payload >> 19) & 0x1ff);

