//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/rtrs/rtrs.h
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

//
// RDMA transport (RTRS) client API
//
// enum rtrs_clt_link_ev - Events about connectivity state of a client
// @RTRS_CLT_LINK_EV_RECONNECTED:	Client was reconnected.
// @RTRS_CLT_LINK_EV_DISCONNECTED:	Client was disconnected.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_clt_link_ev {
    RTRS_CLT_LINK_EV_RECONNECTED,
    RTRS_CLT_LINK_EV_DISCONNECTED,
}

//
// struct rtrs_addr - Source and destination address of a path to be established
// @src:	source address
// @dst:	destination address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_addr {
    pub src: *mut sockaddr_storage,
    pub dst: *mut sockaddr_storage,
}

//
// struct rtrs_clt_ops - it holds the link event callback and private pointer.
// @priv: User supplied private data.
// @link_ev: Event notification callback function for connection state changes
// @priv: User supplied data that was passed to rtrs_clt_open()
// @ev: Occurred event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_ops {
    pub priv: *mut c_void,
    pub ev): *mut *mut *mut void (link_ev)(void priv, enum rtrs_clt_link_ev,
}

extern "C" {
    pub fn rtrs_clt_close(clt: *mut rtrs_clt_sess);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wait_type {
    RTRS_PERMIT_NOWAIT = 0,
    RTRS_PERMIT_WAIT   = 1
}

//
// enum rtrs_clt_con_type - type of ib connection to use with a given
// rtrs_permit
// @RTRS_ADMIN_CON: use connection reserved for "service" messages
// @RTRS_IO_CON: use a connection reserved for IO
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_clt_con_type {
    RTRS_ADMIN_CON,
    RTRS_IO_CON
}

//
// struct rtrs_clt_req_ops - it holds the request confirmation callback
// and a private pointer.
// @priv: User supplied private data.
// @conf_fn:	callback function to be called as confirmation
// @priv:	User provided data, passed back with corresponding
// @(conf) confirmation.
// @errno: error number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_req_ops {
    pub priv: *mut c_void,
    pub errno): *mut *mut *mut void (conf_fn)(void priv, int,
}

extern "C" {
    pub fn rtrs_clt_rdma_cq_direct(clt: *mut rtrs_clt_sess, index: c_uint) -> c_int;
}
//
// struct rtrs_attrs - RTRS session attributes
// @queue_depth:	queue_depth saved from rtrs_clt_sess message
// @max_io_size:	max_io_size from rtrs_clt_sess message, capped to
// @max_segments * %SZ_4K
// @max_segments:	max_segments saved from rtrs_clt_sess message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_attrs {
    pub queue_depth: u32,
    pub max_io_size: u32,
    pub max_segments: u32,
}

extern "C" {
    pub fn rtrs_clt_query(sess: *mut rtrs_clt_sess, attr: *mut rtrs_attrs) -> c_int;
}
//
// Here goes RTRS server API
//
// enum rtrs_srv_link_ev - Server link events
// @RTRS_SRV_LINK_EV_CONNECTED:	Connection from client established
// @RTRS_SRV_LINK_EV_DISCONNECTED:	Connection was disconnected, all
// connection RTRS resources were freed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_srv_link_ev {
    RTRS_SRV_LINK_EV_CONNECTED,
    RTRS_SRV_LINK_EV_DISCONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_ops {
//
// rdma_ev():		Event notification for RDMA operations
// If the callback returns a value != 0, an error
// message for the data transfer will be sent to
// the client.
// @priv:		Private data set by rtrs_srv_set_sess_priv()
// @id:		internal RTRS operation id
// @data:		Pointer to (bidirectional) rdma memory area:
// - in case of %RTRS_SRV_RDMA_EV_RECV contains
// data sent by the client
// - in case of %RTRS_SRV_RDMA_EV_WRITE_REQ points
// to the memory area where the response is to be
// written to
// @datalen:	Size of the memory area in @data
// @usr:		The extra user message sent by the client (%vec)
// @usrlen:	Size of the user message
//
    pub usrlen): usize,
//
// link_ev():		Events about connectivity state changes
// If the callback returns != 0 and the event
// %RTRS_SRV_LINK_EV_CONNECTED the corresponding
// session will be destroyed.
// @sess:		Session
// @ev:		event
// @priv:		Private data from user if previously set with
// rtrs_srv_set_sess_priv()
//
    pub priv): *mut c_void,
}

extern "C" {
    pub fn rtrs_srv_close(ctx: *mut rtrs_srv_ctx);
}
extern "C" {
    pub fn rtrs_srv_resp_rdma(id: *mut rtrs_srv_op, errno: c_int) -> bool;
}
extern "C" {
    pub fn rtrs_srv_set_sess_priv(sess: *mut rtrs_srv_sess, priv: *mut c_void);
}
extern "C" {
    pub fn rtrs_srv_get_queue_depth(sess: *mut rtrs_srv_sess) -> c_int;
}
extern "C" {
    pub fn sockaddr_to_str(addr: *const sockaddr, buf: *mut c_char, len: usize) -> c_int;
}
extern "C" {
    pub fn rtrs_addr_to_str(addr: *const rtrs_addr, buf: *mut c_char, len: usize) -> c_int;
}
