//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/iscsi/iscsi_transport.h
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
pub struct iscsit_transport {
pub const ISCSIT_TRANSPORT_NAME: c_int = 16;
    pub name: [c_char; ISCSIT_TRANSPORT_NAME],
    pub transport_type: c_int,
    pub rdma_shutdown: bool,
    pub priv_size: c_int,
    pub owner: *mut module,
    pub t_node: list_head,
    pub ): *mut *mut *mut int (iscsit_setup_np)(struct iscsi_np , struct sockaddr_storage,
    pub ): *mut *mut *mut int (iscsit_accept_np)(struct iscsi_np , struct iscsit_conn,
    pub ): *mut *mut void (iscsit_free_np)(struct iscsi_np,
    pub ): *mut *mut void (iscsit_wait_conn)(struct iscsit_conn,
    pub ): *mut *mut void (iscsit_free_conn)(struct iscsit_conn,
    pub ): *mut *mut *mut int (iscsit_get_login_rx)(struct iscsit_conn , struct iscsi_login,
    pub u32): *mut *mut *mut *mut int (iscsit_put_login_tx)(struct iscsit_conn , struct iscsi_login ,,
    pub int): *mut *mut *mut *mut int (iscsit_immediate_queue)(struct iscsit_conn , struct iscsit_cmd ,,
    pub int): *mut *mut *mut *mut int (iscsit_response_queue)(struct iscsit_conn , struct iscsit_cmd ,,
    pub bool): *mut *mut *mut *mut int (iscsit_get_dataout)(struct iscsit_conn , struct iscsit_cmd ,,
    pub ): *mut *mut *mut int (iscsit_queue_data_in)(struct iscsit_conn , struct iscsit_cmd,
    pub ): *mut *mut *mut int (iscsit_queue_status)(struct iscsit_conn , struct iscsit_cmd,
    pub ): *mut *mut *mut void (iscsit_aborted_task)(struct iscsit_conn , struct iscsit_cmd,
    pub u32): *const *const *const iscsi_datain_req , void ,,
    pub ): *mut *mut *mut void (iscsit_unmap_cmd)(struct iscsit_conn , struct iscsit_cmd,
    pub ): *mut *mut void (iscsit_get_rx_pdu)(struct iscsit_conn,
    pub ): *mut *mut int (iscsit_validate_params)(struct iscsit_conn,
    pub ): *mut iscsi_r2t,
    pub ): *mut *mut target_prot_op (iscsit_get_sup_prot_ops)(struct iscsit_conn,
}

//
// From iscsi_target_transport.c
//
extern "C" {
    pub fn iscsit_register_transport(: *mut iscsit_transport);
}
extern "C" {
    pub fn iscsit_unregister_transport(: *mut iscsit_transport);
}
extern "C" {
    pub fn iscsit_put_transport(: *mut iscsit_transport);
}
//
// From iscsi_target.c
//
extern "C" {
    pub fn iscsit_set_unsolicited_dataout(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_logout_post_handler(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_queue_rsp(: *mut iscsit_conn, : *mut iscsit_cmd) -> c_int;
}
extern "C" {
    pub fn iscsit_aborted_task(: *mut iscsit_conn, : *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_add_reject(: *mut iscsit_conn, _arg: u8, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn iscsit_reject_cmd(: *mut iscsit_cmd, _arg: u8, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn iscsit_handle_snack(: *mut iscsit_conn, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn iscsit_immediate_queue(: *mut iscsit_conn, : *mut iscsit_cmd, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_response_queue(: *mut iscsit_conn, : *mut iscsit_cmd, _arg: c_int) -> c_int;
}
//
// From iscsi_target_device.c
//
extern "C" {
    pub fn iscsit_increment_maxcmdsn(: *mut iscsit_cmd, : *mut iscsit_session);
}
//
// From iscsi_target_erl0.c
//
extern "C" {
    pub fn iscsit_cause_connection_reinstatement(: *mut iscsit_conn, _arg: c_int);
}
//
// From iscsi_target_erl1.c
//
extern "C" {
    pub fn iscsit_stop_dataout_timer(: *mut iscsit_cmd);
}
//
// From iscsi_target_tmr.c
//
extern "C" {
    pub fn iscsit_tmr_post_handler(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
//
// From iscsi_target_util.c
//
extern "C" {
    pub fn iscsit_release_cmd(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_free_cmd(: *mut iscsit_cmd, _arg: bool);
}
//
// From iscsi_target_nego.c
//
// From iscsi_target_login.c
//
// From iscsi_target_parameters.c
//
