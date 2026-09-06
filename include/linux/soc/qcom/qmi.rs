//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/qmi.h
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
// Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
// Copyright (c) 2017, Linaro Ltd.
//

//
// struct qmi_header - wireformat header of QMI messages
// @type:	type of message
// @txn_id:	transaction id
// @msg_id:	message id
// @msg_len:	length of message payload following header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_header {
    pub type: u8,
    pub txn_id: __le16,
    pub msg_id: __le16,
    pub msg_len: __le16,
    pub __packed: },
pub const QMI_REQUEST: c_int = 0;
pub const QMI_RESPONSE: c_int = 2;
pub const QMI_INDICATION: c_int = 4;
pub const QMI_COMMON_TLV_TYPE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmi_elem_type {
    QMI_EOTI,
    QMI_OPT_FLAG,
    QMI_DATA_LEN,
    QMI_UNSIGNED_1_BYTE,
    QMI_UNSIGNED_2_BYTE,
    QMI_UNSIGNED_4_BYTE,
    QMI_UNSIGNED_8_BYTE,
    QMI_SIGNED_2_BYTE_ENUM,
    QMI_SIGNED_4_BYTE_ENUM,
    QMI_STRUCT,
    QMI_STRING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmi_array_type {
    NO_ARRAY,
    STATIC_ARRAY,
    VAR_LEN_ARRAY,
}

//
// struct qmi_elem_info - describes how to encode a single QMI element
// @data_type:	Data type of this element.
// @elem_len:	Array length of this element, if an array.
// @elem_size:	Size of a single instance of this data type.
// @array_type:	Array type of this element.
// @tlv_type:	QMI message specific type to identify which element
// is present in an incoming message.
// @offset:	Specifies the offset of the first instance of this
// element in the data structure.
// @ei_array:	Null-terminated array of @qmi_elem_info to describe nested
// structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_elem_info {
    pub data_type: qmi_elem_type,
    pub elem_len: u32,
    pub elem_size: u32,
    pub array_type: qmi_array_type,
    pub tlv_type: u8,
    pub offset: u32,
    pub ei_array: *const qmi_elem_info,
}

pub const QMI_RESULT_SUCCESS_V01: c_int = 0;
pub const QMI_RESULT_FAILURE_V01: c_int = 1;
pub const QMI_ERR_NONE_V01: c_int = 0;
pub const QMI_ERR_MALFORMED_MSG_V01: c_int = 1;
pub const QMI_ERR_NO_MEMORY_V01: c_int = 2;
pub const QMI_ERR_INTERNAL_V01: c_int = 3;
pub const QMI_ERR_CLIENT_IDS_EXHAUSTED_V01: c_int = 5;
pub const QMI_ERR_INVALID_ID_V01: c_int = 41;
pub const QMI_ERR_ENCODING_V01: c_int = 58;
pub const QMI_ERR_DISABLED_V01: c_int = 69;
pub const QMI_ERR_INCOMPATIBLE_STATE_V01: c_int = 90;
pub const QMI_ERR_NOT_SUPPORTED_V01: c_int = 94;
//
// Enumerate the IDs of the QMI services
//
pub const QMI_SERVICE_ID_TEST: c_uint = 0x0f	/*   15 */;
pub const QMI_SERVICE_ID_SSCTL: c_uint = 0x2b	/*   43 */;
pub const QMI_SERVICE_ID_IPA: c_uint = 0x31	/*   49 */;
pub const QMI_SERVICE_ID_SERVREG_LOC: c_uint = 0x40	/*   64 */;
pub const QMI_SERVICE_ID_SERVREG_NOTIF: c_uint = 0x42	/*   66 */;
pub const QMI_SERVICE_ID_WLFW: c_uint = 0x45	/*   69 */;
pub const QMI_SERVICE_ID_SLIMBUS: c_uint = 0x301	/*  769 */;
pub const QMI_SERVICE_ID_USB_AUDIO_STREAM: c_uint = 0x41d	/* 1053 */;
//
// struct qmi_response_type_v01 - common response header (decoded)
// @result:	result of the transaction
// @error:	error value, when @result is QMI_RESULT_FAILURE_V01
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_response_type_v01 {
    pub result: u16,
    pub error: u16,
}

//
// struct qmi_service - context to track lookup-results
// @service:	service type
// @version:	version of the @service
// @instance:	instance id of the @service
// @node:	node of the service
// @port:	port of the service
// @priv:	handle for client's use
// @list_node:	list_head for house keeping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_service {
    pub service: c_uint,
    pub version: c_uint,
    pub instance: c_uint,
    pub node: c_uint,
    pub port: c_uint,
    pub priv: *mut c_void,
    pub list_node: list_head,
}

//
// struct qmi_ops - callbacks for qmi_handle
// @new_server:		inform client of a new_server lookup-result, returning
// successfully from this call causes the library to call
// @del_server as the service is removed from the
// lookup-result. @priv of the qmi_service can be used by
// the client
// @del_server:		inform client of a del_server lookup-result
// @net_reset:		inform client that the name service was restarted and
// that and any state needs to be released
// @msg_handler:	invoked for incoming messages, allows a client to
// override the usual QMI message handler
// @bye:                inform a client that all clients from a node are gone
// @del_client:         inform a client that a particular client is gone
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_ops {
    pub svc): *mut *mut *mut int (new_server)(struct qmi_handle qmi, struct qmi_service,
    pub svc): *mut *mut *mut void (del_server)(struct qmi_handle qmi, struct qmi_service,
    pub qmi): *mut *mut void (net_reset)(struct qmi_handle,
    pub count): *const *const void data, size_t,
    pub node): *mut *mut *mut void (bye)(struct qmi_handle qmi, unsigned int,
    pub port): unsigned int node, unsigned int,
}

//
// struct qmi_txn - transaction context
// @qmi:	QMI handle this transaction is associated with
// @id:		transaction id
// @lock:	for synchronization between handler and waiter of messages
// @completion:	completion object as the transaction receives a response
// @result:	result code for the completed transaction
// @ei:		description of the QMI encoded response (optional)
// @dest:	destination buffer to decode message into (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_txn {
    pub qmi: *mut qmi_handle,
    pub id: u16,
    pub lock: mutex,
    pub completion: completion,
    pub result: c_int,
    pub ei: *const qmi_elem_info,
    pub dest: *mut c_void,
}

//
// struct qmi_msg_handler - description of QMI message handler
// @type:	type of message
// @msg_id:	message id
// @ei:		description of the QMI encoded message
// @decoded_size:	size of the decoded object
// @fn:		function to invoke as the message is decoded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_msg_handler {
    pub type: c_uint,
    pub msg_id: c_uint,
    pub ei: *const qmi_elem_info,
    pub decoded_size: usize,
    pub decoded): *const *const qmi_txn txn, void,
}

//
// struct qmi_handle - QMI context
// @sock:	socket handle
// @sock_lock:	synchronization of @sock modifications
// @sq:		sockaddr of @sock
// @work:	work for handling incoming messages
// @wq:		workqueue to post @work on
// @recv_buf:	scratch buffer for handling incoming messages
// @recv_buf_size:	size of @recv_buf
// @lookups:		list of registered lookup requests
// @lookup_results:	list of lookup-results advertised to the client
// @services:		list of registered services (by this client)
// @ops:	reference to callbacks
// @txns:	outstanding transactions
// @txn_lock:	lock for modifications of @txns
// @handlers:	list of handlers for incoming messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_handle {
    pub sock: *mut socket,
    pub sock_lock: mutex,
    pub sq: sockaddr_qrtr,
    pub work: work_struct,
    pub wq: *mut workqueue_struct,
    pub recv_buf: *mut c_void,
    pub recv_buf_size: usize,
    pub lookups: list_head,
    pub lookup_results: list_head,
    pub services: list_head,
    pub ops: qmi_ops,
    pub txns: idr,
    pub txn_lock: mutex,
    pub handlers: *const qmi_msg_handler,
}

extern "C" {
    pub fn qmi_handle_release(qmi: *mut qmi_handle);
}
extern "C" {
    pub fn qmi_txn_wait(txn: *mut qmi_txn, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn qmi_txn_cancel(txn: *mut qmi_txn);
}
