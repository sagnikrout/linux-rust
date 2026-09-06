//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_cm.h
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
// Copyright (c) 2004, 2005 Intel Corporation.  All rights reserved.
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
// Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_state {
    IB_CM_IDLE,
    IB_CM_LISTEN,
    IB_CM_REQ_SENT,
    IB_CM_REQ_RCVD,
    IB_CM_MRA_REQ_SENT,
    IB_CM_MRA_REQ_RCVD,
    IB_CM_REP_SENT,
    IB_CM_REP_RCVD,
    IB_CM_MRA_REP_SENT,
    IB_CM_MRA_REP_RCVD,
    IB_CM_ESTABLISHED,
    IB_CM_DREQ_SENT,
    IB_CM_DREQ_RCVD,
    IB_CM_TIMEWAIT,
    IB_CM_SIDR_REQ_SENT,
    IB_CM_SIDR_REQ_RCVD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_lap_state {
    IB_CM_LAP_UNINIT,
    IB_CM_LAP_IDLE,
    IB_CM_LAP_SENT,
    IB_CM_LAP_RCVD,
    IB_CM_MRA_LAP_SENT,
    IB_CM_MRA_LAP_RCVD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_event_type {
    IB_CM_REQ_ERROR,
    IB_CM_REQ_RECEIVED,
    IB_CM_REP_ERROR,
    IB_CM_REP_RECEIVED,
    IB_CM_RTU_RECEIVED,
    IB_CM_USER_ESTABLISHED,
    IB_CM_DREQ_ERROR,
    IB_CM_DREQ_RECEIVED,
    IB_CM_DREP_RECEIVED,
    IB_CM_TIMEWAIT_EXIT,
    IB_CM_MRA_RECEIVED,
    IB_CM_REJ_RECEIVED,
    IB_CM_LAP_ERROR,
    IB_CM_LAP_RECEIVED,
    IB_CM_APR_RECEIVED,
    IB_CM_SIDR_REQ_ERROR,
    IB_CM_SIDR_REQ_RECEIVED,
    IB_CM_SIDR_REP_RECEIVED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_data_size {
    IB_CM_REQ_PRIVATE_DATA_SIZE	 = 92,
    IB_CM_MRA_PRIVATE_DATA_SIZE	 = 222,
    IB_CM_REJ_PRIVATE_DATA_SIZE	 = 148,
    IB_CM_REP_PRIVATE_DATA_SIZE	 = 196,
    IB_CM_RTU_PRIVATE_DATA_SIZE	 = 224,
    IB_CM_DREQ_PRIVATE_DATA_SIZE	 = 220,
    IB_CM_DREP_PRIVATE_DATA_SIZE	 = 224,
    IB_CM_REJ_ARI_LENGTH		 = 72,
    IB_CM_LAP_PRIVATE_DATA_SIZE	 = 168,
    IB_CM_APR_PRIVATE_DATA_SIZE	 = 148,
    IB_CM_APR_INFO_LENGTH		 = 72,
    IB_CM_SIDR_REQ_PRIVATE_DATA_SIZE = 216,
    IB_CM_SIDR_REP_PRIVATE_DATA_SIZE = 136,
    IB_CM_SIDR_REP_INFO_LENGTH	 = 72,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_req_event_param {
    pub listen_id: *mut ib_cm_id,
// P_Key that was used by the GMP's BTH header
    pub bth_pkey: u16,
    pub port: u8,
    pub primary_path: *mut sa_path_rec,
    pub alternate_path: *mut sa_path_rec,
//
// SGID attribute of the primary path. Currently only
// useful for RoCE. Alternate path GID attributes
// are not yet supported.
//
    pub ppath_sgid_attr: *const ib_gid_attr,
    pub remote_ca_guid: __be64,
    pub remote_qkey: u32,
    pub remote_qpn: u32,
    pub qp_type: ib_qp_type,
    pub starting_psn: u32,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub local_cm_response_timeout:5: c_uint,
    pub flow_control:1: c_uint,
    pub remote_cm_response_timeout:5: c_uint,
    pub retry_count:3: c_uint,
    pub rnr_retry_count:3: c_uint,
    pub srq:1: c_uint,
    pub ece: rdma_ucm_ece,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_rep_event_param {
    pub remote_ca_guid: __be64,
    pub remote_qkey: u32,
    pub remote_qpn: u32,
    pub starting_psn: u32,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub target_ack_delay:5: c_uint,
    pub failover_accepted:2: c_uint,
    pub flow_control:1: c_uint,
    pub rnr_retry_count:3: c_uint,
    pub srq:1: c_uint,
    pub ece: rdma_ucm_ece,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_rej_reason {
    IB_CM_REJ_NO_QP				= 1,
    IB_CM_REJ_NO_EEC			= 2,
    IB_CM_REJ_NO_RESOURCES			= 3,
    IB_CM_REJ_TIMEOUT			= 4,
    IB_CM_REJ_UNSUPPORTED			= 5,
    IB_CM_REJ_INVALID_COMM_ID		= 6,
    IB_CM_REJ_INVALID_COMM_INSTANCE		= 7,
    IB_CM_REJ_INVALID_SERVICE_ID		= 8,
    IB_CM_REJ_INVALID_TRANSPORT_TYPE	= 9,
    IB_CM_REJ_STALE_CONN			= 10,
    IB_CM_REJ_RDC_NOT_EXIST			= 11,
    IB_CM_REJ_INVALID_GID			= 12,
    IB_CM_REJ_INVALID_LID			= 13,
    IB_CM_REJ_INVALID_SL			= 14,
    IB_CM_REJ_INVALID_TRAFFIC_CLASS		= 15,
    IB_CM_REJ_INVALID_HOP_LIMIT		= 16,
    IB_CM_REJ_INVALID_PACKET_RATE		= 17,
    IB_CM_REJ_INVALID_ALT_GID		= 18,
    IB_CM_REJ_INVALID_ALT_LID		= 19,
    IB_CM_REJ_INVALID_ALT_SL		= 20,
    IB_CM_REJ_INVALID_ALT_TRAFFIC_CLASS	= 21,
    IB_CM_REJ_INVALID_ALT_HOP_LIMIT		= 22,
    IB_CM_REJ_INVALID_ALT_PACKET_RATE	= 23,
    IB_CM_REJ_PORT_CM_REDIRECT		= 24,
    IB_CM_REJ_PORT_REDIRECT			= 25,
    IB_CM_REJ_INVALID_MTU			= 26,
    IB_CM_REJ_INSUFFICIENT_RESP_RESOURCES	= 27,
    IB_CM_REJ_CONSUMER_DEFINED		= 28,
    IB_CM_REJ_INVALID_RNR_RETRY		= 29,
    IB_CM_REJ_DUPLICATE_LOCAL_COMM_ID	= 30,
    IB_CM_REJ_INVALID_CLASS_VERSION		= 31,
    IB_CM_REJ_INVALID_FLOW_LABEL		= 32,
    IB_CM_REJ_INVALID_ALT_FLOW_LABEL	= 33,
    IB_CM_REJ_VENDOR_OPTION_NOT_SUPPORTED	= 35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_rej_event_param {
    pub reason: ib_cm_rej_reason,
    pub ari: *mut c_void,
    pub ari_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_mra_event_param {
    pub service_timeout: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_lap_event_param {
    pub alternate_path: *mut sa_path_rec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_apr_status {
    IB_CM_APR_SUCCESS,
    IB_CM_APR_INVALID_COMM_ID,
    IB_CM_APR_UNSUPPORTED,
    IB_CM_APR_REJECT,
    IB_CM_APR_REDIRECT,
    IB_CM_APR_IS_CURRENT,
    IB_CM_APR_INVALID_QPN_EECN,
    IB_CM_APR_INVALID_LID,
    IB_CM_APR_INVALID_GID,
    IB_CM_APR_INVALID_FLOW_LABEL,
    IB_CM_APR_INVALID_TCLASS,
    IB_CM_APR_INVALID_HOP_LIMIT,
    IB_CM_APR_INVALID_PACKET_RATE,
    IB_CM_APR_INVALID_SL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_apr_event_param {
    pub ap_status: ib_cm_apr_status,
    pub apr_info: *mut c_void,
    pub info_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_sidr_req_event_param {
    pub listen_id: *mut ib_cm_id,
    pub service_id: __be64,
//
// SGID attribute of the request. Currently only
// useful for RoCE.
//
    pub sgid_attr: *const ib_gid_attr,
// P_Key that was used by the GMP's BTH header
    pub bth_pkey: u16,
    pub port: u8,
    pub pkey: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cm_sidr_status {
    IB_SIDR_SUCCESS,
    IB_SIDR_UNSUPPORTED,
    IB_SIDR_REJECT,
    IB_SIDR_NO_QP,
    IB_SIDR_REDIRECT,
    IB_SIDR_UNSUPPORTED_VERSION
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_sidr_rep_event_param {
    pub status: ib_cm_sidr_status,
    pub qkey: u32,
    pub qpn: u32,
    pub info: *mut c_void,
    pub sgid_attr: *const ib_gid_attr,
    pub info_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_event {
    pub event: ib_cm_event_type,
    pub req_rcvd: ib_cm_req_event_param,
    pub rep_rcvd: ib_cm_rep_event_param,
// No data for RTU received events.
    pub rej_rcvd: ib_cm_rej_event_param,
    pub mra_rcvd: ib_cm_mra_event_param,
    pub lap_rcvd: ib_cm_lap_event_param,
    pub apr_rcvd: ib_cm_apr_event_param,
// No data for DREQ/DREP received events.
    pub sidr_req_rcvd: ib_cm_sidr_req_event_param,
    pub sidr_rep_rcvd: ib_cm_sidr_rep_event_param,
    pub send_status: ib_wc_status,
    pub param: },
    pub private_data: *mut c_void,
}

//
// typedef ib_cm_handler - User-defined callback to process communication events.
// @cm_id: Communication identifier associated with the reported event.
// @event: Information about the communication event.
//
// IB_CM_REQ_RECEIVED and IB_CM_SIDR_REQ_RECEIVED communication events
// generated as a result of listen requests result in the allocation of a
// new @cm_id.  The new @cm_id is returned to the user through this callback.
// Clients are responsible for destroying the new @cm_id.  For peer-to-peer
// IB_CM_REQ_RECEIVED and all other events, the returned @cm_id corresponds
// to a user's existing communication identifier.
//
// Users may not call ib_destroy_cm_id while in the context of this callback;
// however, returning a non-zero value instructs the communication manager to
// destroy the @cm_id after the callback completes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_id {
    pub cm_handler: ib_cm_handler,
    pub context: *mut c_void,
    pub device: *mut ib_device,
    pub service_id: __be64,
    pub /: *mut *mut ib_cm_state state; / internal CM/debug use,
    pub /: *mut *mut ib_cm_lap_state lap_state; / internal CM/debug use,
    pub local_id: __be32,
    pub remote_id: __be32,
    pub /: *mut *mut u32 remote_cm_qpn; / 1 unless redirected,
}

//
// ib_create_cm_id - Allocate a communication identifier.
// @device: Device associated with the cm_id.  All related communication will
// be associated with the specified device.
// @cm_handler: Callback invoked to notify the user of CM events.
// @context: User specified context associated with the communication
// identifier.
//
// Communication identifiers are used to track connection states, service
// ID resolution requests, and listen requests.
//
// ib_destroy_cm_id - Destroy a connection identifier.
// @cm_id: Connection identifier to destroy.
//
// This call blocks until the connection identifier is destroyed.
//
extern "C" {
    pub fn ib_destroy_cm_id(cm_id: *mut ib_cm_id);
}

//
// ib_cm_listen - Initiates listening on the specified service ID for
// connection and service ID resolution requests.
// @cm_id: Connection identifier associated with the listen request.
// @service_id: Service identifier matched against incoming connection
// and service ID resolution requests.  The service ID should be specified
// network-byte order.  If set to IB_CM_ASSIGN_SERVICE_ID, the CM will
// assign a service ID to the caller.
//
extern "C" {
    pub fn ib_cm_listen(cm_id: *mut ib_cm_id, service_id: __be64) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_req_param {
    pub primary_path: *mut sa_path_rec,
    pub primary_path_inbound: *mut sa_path_rec,
    pub primary_path_outbound: *mut sa_path_rec,
    pub alternate_path: *mut sa_path_rec,
    pub ppath_sgid_attr: *const ib_gid_attr,
    pub service_id: __be64,
    pub qp_num: u32,
    pub qp_type: ib_qp_type,
    pub starting_psn: u32,
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub remote_cm_response_timeout: u8,
    pub flow_control: u8,
    pub local_cm_response_timeout: u8,
    pub retry_count: u8,
    pub rnr_retry_count: u8,
    pub max_cm_retries: u8,
    pub srq: u8,
    pub ece: rdma_ucm_ece,
}

//
// ib_send_cm_req - Sends a connection request to the remote node.
// @cm_id: Connection identifier that will be associated with the
// connection request.
// @param: Connection request information needed to establish the
// connection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_rep_param {
    pub qp_num: u32,
    pub starting_psn: u32,
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub failover_accepted: u8,
    pub flow_control: u8,
    pub rnr_retry_count: u8,
    pub srq: u8,
    pub ece: rdma_ucm_ece,
}

//
// ib_send_cm_rep - Sends a connection reply in response to a connection
// request.
// @cm_id: Connection identifier that will be associated with the
// connection request.
// @param: Connection reply information needed to establish the
// connection.
//
// ib_send_cm_rtu - Sends a connection ready to use message in response
// to a connection reply message.
// @cm_id: Connection identifier associated with the connection request.
// @private_data: Optional user-defined private data sent with the
// ready to use message.
// @private_data_len: Size of the private data buffer, in bytes.
//
// ib_send_cm_dreq - Sends a disconnection request for an existing
// connection.
// @cm_id: Connection identifier associated with the connection being
// released.
// @private_data: Optional user-defined private data sent with the
// disconnection request message.
// @private_data_len: Size of the private data buffer, in bytes.
//
// ib_send_cm_drep - Sends a disconnection reply to a disconnection request.
// @cm_id: Connection identifier associated with the connection being
// released.
// @private_data: Optional user-defined private data sent with the
// disconnection reply message.
// @private_data_len: Size of the private data buffer, in bytes.
//
// If the cm_id is in the correct state, the CM will transition the connection
// to the timewait state, even if an error occurs sending the DREP message.
//
// ib_cm_notify - Notifies the CM of an event reported to the consumer.
// @cm_id: Connection identifier to transition to established.
// @event: Type of event.
//
// This routine should be invoked by users to notify the CM of relevant
// communication events.  Events that should be reported to the CM and
// when to report them are:
//
// IB_EVENT_COMM_EST - Used when a message is received on a connected
// QP before an RTU has been received.
// IB_EVENT_PATH_MIG - Notifies the CM that the connection has failed over
// to the alternate path.
//
extern "C" {
    pub fn ib_cm_notify(cm_id: *mut ib_cm_id, event: ib_event_type) -> c_int;
}
//
// ib_send_cm_rej - Sends a connection rejection message to the
// remote node.
// @cm_id: Connection identifier associated with the connection being
// rejected.
// @reason: Reason for the connection request rejection.
// @ari: Optional additional rejection information.
// @ari_length: Size of the additional rejection information, in bytes.
// @private_data: Optional user-defined private data sent with the
// rejection message.
// @private_data_len: Size of the private data buffer, in bytes.
//
// ib_prepare_cm_mra - Prepares to send a message receipt acknowledgment to a
// connection message in case duplicates are received.
// @cm_id: Connection identifier associated with the connection message.
//
extern "C" {
    pub fn ib_prepare_cm_mra(cm_id: *mut ib_cm_id) -> c_int;
}
//
// ib_cm_init_qp_attr - Initializes the QP attributes for use in transitioning
// to a specified QP state.
// @cm_id: Communication identifier associated with the QP attributes to
// initialize.
// @qp_attr: On input, specifies the desired QP state.  On output, the
// mandatory and desired optional attributes will be set in order to
// modify the QP to the specified state.
// @qp_attr_mask: The QP attribute mask that may be used to transition the
// QP to the specified state.
//
// Users must set the @qp_attr->qp_state to the desired QP state.  This call
// will set all required attributes for the given transition, along with
// known optional attributes.  Users may override the attributes returned from
// this call before calling ib_modify_qp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_sidr_req_param {
    pub path: *mut sa_path_rec,
    pub sgid_attr: *const ib_gid_attr,
    pub service_id: __be64,
    pub timeout_ms: c_ulong,
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub max_cm_retries: u8,
}

//
// ib_send_cm_sidr_req - Sends a service ID resolution request to the
// remote node.
// @cm_id: Communication identifier that will be associated with the
// service ID resolution request.
// @param: Service ID resolution request information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm_sidr_rep_param {
    pub qp_num: u32,
    pub qkey: u32,
    pub status: ib_cm_sidr_status,
    pub info: *const c_void,
    pub info_length: u8,
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub ece: rdma_ucm_ece,
}

//
// ib_send_cm_sidr_rep - Sends a service ID resolution reply to the
// remote node.
// @cm_id: Communication identifier associated with the received service ID
// resolution request.
// @param: Service ID resolution reply information.
//
// ibcm_reject_msg - return a pointer to a reject message string.
// @reason: Value returned in the REJECT event status field.
//
extern "C" {
    pub fn ibcm_reject_msg(reason: c_int) -> *const char __attribute_const__;
}
