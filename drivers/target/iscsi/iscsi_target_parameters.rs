//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_parameters.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_extra_response {
    pub key: [c_char; KEY_MAXLEN],
    pub value: [c_char; 32],
    pub er_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_param {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub set_param: u8,
    pub phase: u8,
    pub scope: u8,
    pub sender: u8,
    pub type: u8,
    pub use: u8,
    pub type_range: u16,
    pub state: u32,
    pub p_list: list_head,
    pub ____cacheline_aligned: },
    pub iscsit_conn: struct,
    pub iscsi_conn_ops: struct,
    pub iscsi_param_list: struct,
    pub iscsi_sess_ops: struct,
    pub int): *mut *mut *mut extern int iscsi_login_rx_data(struct iscsit_conn , char ,,
    pub int): *mut *mut *mut *mut extern int iscsi_login_tx_data(struct iscsit_conn , char , char ,,
    pub ): *mut extern int iscsi_create_default_params(struct iscsi_param_list,
    pub bool): *mut *mut extern int iscsi_set_keys_to_negotiate(struct iscsi_param_list ,,
    pub ): *mut extern int iscsi_set_keys_irrelevant_for_discovery(struct iscsi_param_list,
    pub int): *mut *mut iscsi_param_list ,,
    pub int): *mut *mut *mut extern int iscsi_change_param_value(char , struct iscsi_param_list ,,
    pub ): *mut extern void iscsi_release_param_list(struct iscsi_param_list,
    pub ): *mut *mut *mut extern struct iscsi_param iscsi_find_param_from_key(char , struct iscsi_param_list,
    pub ): *mut *mut *mut *mut extern int iscsi_extract_key_value(char , char , char,
    pub ): *mut *mut extern int iscsi_update_param_value(struct iscsi_param , char,
    pub ): *mut *mut extern int iscsi_decode_text_input(u8, u8, char , u32, struct iscsit_conn,
    pub bool): *mut *mut iscsi_param_list ,,
    pub ): *mut extern int iscsi_check_negotiated_keys(struct iscsi_param_list,
    pub ): *mut iscsi_param_list,
    pub int): *mut *mut iscsi_param_list ,,

//
// The Parameter Names.
//

//
// Parameter names of iSCSI Extentions for RDMA (iSER).  See RFC-5046
//

//
// For AuthMethod.
//

//
// Initial values for Parameter Negotiation.
//

//
// Match outgoing MXDSL default to incoming Open-iSCSI default
//

//
// Initial values for iSER parameters following RFC-5046 Section 6
//

//
// For [Header,Data]Digests.
//

//
// For SessionType.
//

//
// struct iscsi_param->use
//
pub const USE_LEADING_ONLY: c_uint = 0x01;
pub const USE_INITIAL_ONLY: c_uint = 0x02;
pub const USE_ALL: c_uint = 0x04;

//
// struct iscsi_param->sender
//
pub const SENDER_INITIATOR: c_uint = 0x01;
pub const SENDER_TARGET: c_uint = 0x02;
pub const SENDER_BOTH: c_uint = 0x03;
// Used in iscsi_check_key()
pub const SENDER_RECEIVER: c_uint = 0x04;

//
// struct iscsi_param->scope
//
pub const SCOPE_CONNECTION_ONLY: c_uint = 0x01;
pub const SCOPE_SESSION_WIDE: c_uint = 0x02;

//
// struct iscsi_param->phase
//
pub const PHASE_SECURITY: c_uint = 0x01;
pub const PHASE_OPERATIONAL: c_uint = 0x02;
pub const PHASE_DECLARATIVE: c_uint = 0x04;
pub const PHASE_FFP0: c_uint = 0x08;

//
// struct iscsi_param->type
//
pub const TYPE_BOOL_AND: c_uint = 0x01;
pub const TYPE_BOOL_OR: c_uint = 0x02;
pub const TYPE_NUMBER: c_uint = 0x04;
pub const TYPE_NUMBER_RANGE: c_uint = 0x08;
pub const TYPE_STRING: c_uint = 0x10;
pub const TYPE_VALUE_LIST: c_uint = 0x20;

//
// struct iscsi_param->type_range
//
pub const TYPERANGE_BOOL_AND: c_uint = 0x0001;
pub const TYPERANGE_BOOL_OR: c_uint = 0x0002;
pub const TYPERANGE_0_TO_2: c_uint = 0x0004;
pub const TYPERANGE_0_TO_3600: c_uint = 0x0008;
pub const TYPERANGE_0_TO_32767: c_uint = 0x0010;
pub const TYPERANGE_0_TO_65535: c_uint = 0x0020;
pub const TYPERANGE_1_TO_65535: c_uint = 0x0040;
pub const TYPERANGE_2_TO_3600: c_uint = 0x0080;
pub const TYPERANGE_512_TO_16777215: c_uint = 0x0100;
pub const TYPERANGE_AUTH: c_uint = 0x0200;
pub const TYPERANGE_DIGEST: c_uint = 0x0400;
pub const TYPERANGE_ISCSINAME: c_uint = 0x0800;
pub const TYPERANGE_SESSIONTYPE: c_uint = 0x1000;
pub const TYPERANGE_TARGETADDRESS: c_uint = 0x2000;
pub const TYPERANGE_UTF8: c_uint = 0x4000;

//
// struct iscsi_param->state
//
pub const PSTATE_ACCEPTOR: c_uint = 0x01;
pub const PSTATE_NEGOTIATE: c_uint = 0x02;
pub const PSTATE_PROPOSER: c_uint = 0x04;
pub const PSTATE_IRRELEVANT: c_uint = 0x08;
pub const PSTATE_REJECT: c_uint = 0x10;
pub const PSTATE_REPLY_OPTIONAL: c_uint = 0x20;
pub const PSTATE_RESPONSE_GOT: c_uint = 0x40;
pub const PSTATE_RESPONSE_SENT: c_uint = 0x80;

