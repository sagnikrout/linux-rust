//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/iw_cm.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2005 Network Appliance, Inc. All rights reserved.
// Copyright (c) 2005 Open Grid Computing, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iw_cm_event_type {
    IW_CM_EVENT_CONNECT_REQUEST = 1, /* connect request received */
    IW_CM_EVENT_CONNECT_REPLY,	 /* reply from active connect request */
    IW_CM_EVENT_ESTABLISHED,	 /* passive side accept successful */
    IW_CM_EVENT_DISCONNECT,		 /* orderly shutdown */
    IW_CM_EVENT_CLOSE		 /* close complete */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_cm_event {
    pub event: iw_cm_event_type,
    pub status: c_int,
    pub local_addr: sockaddr_storage,
    pub remote_addr: sockaddr_storage,
    pub private_data: *mut c_void,
    pub provider_data: *mut c_void,
    pub private_data_len: u8,
    pub ord: u8,
    pub ird: u8,
}

//
// typedef iw_cm_handler - Function to be called by the IW CM when delivering
// events to the client.
//
// @cm_id: The IW CM identifier associated with the event.
// @event: Pointer to the event structure.
//
// typedef iw_event_handler - Function called by the provider when delivering
// provider events to the IW CM.  Returns either 0 indicating the event was
// processed or -errno if the event could not be processed.
//
// @cm_id: The IW CM identifier associated with the event.
// @event: Pointer to the event structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_cm_id {
    pub /: *mut *mut iw_cm_handler cm_handler; / client callback function,
    pub /: *mut *mut *mut void context; / client cb context,
    pub device: *mut ib_device,
    pub /: *mut *mut sockaddr_storage local_addr; / local addr,
    pub remote_addr: sockaddr_storage,
    pub /: *mut *mut sockaddr_storage m_local_addr; / nmapped local addr,
    pub /: *mut *mut sockaddr_storage m_remote_addr; / nmapped rem addr,
    pub /: *mut *mut *mut void provider_data; / provider private data,
    pub provider: *mut *mut iw_event_handler event_handler; / cb for,
// Used by provider to add and remove refs on IW cm_id
    pub ): *mut *mut void (add_ref)(struct iw_cm_id,
    pub ): *mut *mut void (rem_ref)(struct iw_cm_id,
    pub tos: u8,
    pub tos_set:1: bool,
    pub mapped:1: bool,
    pub afonly:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_cm_conn_param {
    pub private_data: *const c_void,
    pub private_data_len: u16,
    pub ord: u32,
    pub ird: u32,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iw_flags {

//
// This flag allows the iwcm and iwpmd to still advertise
// mappings but the real and mapped port numbers are the
// same.  Further, iwpmd will not bind any user socket to
// reserve the port.  This is required for soft iwarp
// to play in the port mapped iwarp space.
//
    IW_F_NO_PORT_MAP = (1 << 0),
}

//
// iw_create_cm_id - Create an IW CM identifier.
//
// @device: The IB device on which to create the IW CM identier.
// @cm_handler: User callback invoked to report events associated with the
// returned IW CM identifier.
// @context: User specified context associated with the id.
//
// iw_destroy_cm_id - Destroy an IW CM identifier.
//
// @cm_id: The previously created IW CM identifier to destroy.
//
// The client can assume that no events will be delivered for the CM ID after
// this function returns.
//
extern "C" {
    pub fn iw_destroy_cm_id(cm_id: *mut iw_cm_id);
}
//
// iw_cm_listen - Listen for incoming connection requests on the
// specified IW CM id.
//
// @cm_id: The IW CM identifier.
// @backlog: The maximum number of outstanding un-accepted inbound listen
// requests to queue.
//
// The source address and port number are specified in the IW CM identifier
// structure.
//
extern "C" {
    pub fn iw_cm_listen(cm_id: *mut iw_cm_id, backlog: c_int) -> c_int;
}
//
// iw_cm_accept - Called to accept an incoming connect request.
//
// @cm_id: The IW CM identifier associated with the connection request.
// @iw_param: Pointer to a structure containing connection establishment
// parameters.
//
// The specified cm_id will have been provided in the event data for a
// CONNECT_REQUEST event. Subsequent events related to this connection will be
// delivered to the specified IW CM identifier prior and may occur prior to
// the return of this function. If this function returns a non-zero value, the
// client can assume that no events will be delivered to the specified IW CM
// identifier.
//
extern "C" {
    pub fn iw_cm_accept(cm_id: *mut iw_cm_id, iw_param: *mut iw_cm_conn_param) -> c_int;
}
//
// iw_cm_reject - Reject an incoming connection request.
//
// @cm_id: Connection identifier associated with the request.
// @private_data: Pointer to data to deliver to the remote peer as part of the
// reject message.
// @private_data_len: The number of bytes in the private_data parameter.
//
// The client can assume that no events will be delivered to the specified IW
// CM identifier following the return of this function. The private_data
// buffer is available for reuse when this function returns.
//
// iw_cm_connect - Called to request a connection to a remote peer.
//
// @cm_id: The IW CM identifier for the connection.
// @iw_param: Pointer to a structure containing connection  establishment
// parameters.
//
// Events may be delivered to the specified IW CM identifier prior to the
// return of this function. If this function returns a non-zero value, the
// client can assume that no events will be delivered to the specified IW CM
// identifier.
//
extern "C" {
    pub fn iw_cm_connect(cm_id: *mut iw_cm_id, iw_param: *mut iw_cm_conn_param) -> c_int;
}
//
// iw_cm_disconnect - Close the specified connection.
//
// @cm_id: The IW CM identifier to close.
// @abrupt: If 0, the connection will be closed gracefully, otherwise, the
// connection will be reset.
//
// The IW CM identifier is still active until the IW_CM_EVENT_CLOSE event is
// delivered.
//
extern "C" {
    pub fn iw_cm_disconnect(cm_id: *mut iw_cm_id, abrupt: c_int) -> c_int;
}
//
// iw_cm_init_qp_attr - Called to initialize the attributes of the QP
// associated with a IW CM identifier.
//
// @cm_id: The IW CM identifier associated with the QP
// @qp_attr: Pointer to the QP attributes structure.
// @qp_attr_mask: Pointer to a bit vector specifying which QP attributes are
// valid.
//
// iwcm_reject_msg - return a pointer to a reject message string.
// @reason: Value returned in the REJECT event status field.
//
extern "C" {
    pub fn iwcm_reject_msg(reason: c_int) -> *const char __attribute_const__;
}
