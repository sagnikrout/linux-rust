//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport_srp.h
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

pub const SRP_RPORT_ROLE_INITIATOR: c_int = 0;
pub const SRP_RPORT_ROLE_TARGET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_rport_identifiers {
    pub port_id: [u8; 16],
    pub roles: u8,
}

//
// enum srp_rport_state - SRP transport layer state
// @SRP_RPORT_RUNNING:   Transport layer operational.
// @SRP_RPORT_BLOCKED:   Transport layer not operational; fast I/O fail timer
// is running and I/O has been blocked.
// @SRP_RPORT_FAIL_FAST: Fast I/O fail timer has expired; fail I/O fast.
// @SRP_RPORT_LOST:      Port is being removed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_rport_state {
    SRP_RPORT_RUNNING,
    SRP_RPORT_BLOCKED,
    SRP_RPORT_FAIL_FAST,
    SRP_RPORT_LOST,
}

//
// struct srp_rport - SRP initiator or target port
//
// Fields that are relevant for SRP initiator and SRP target drivers:
// @dev:               Device associated with this rport.
// @port_id:           16-byte port identifier.
// @roles:             Role of this port - initiator or target.
//
// Fields that are only relevant for SRP initiator drivers:
// @lld_data:          LLD private data.
// @mutex:             Protects against concurrent rport reconnect
// fast_io_fail / dev_loss_tmo activity.
// @state:             rport state.
// @reconnect_delay:   Reconnect delay in seconds.
// @failed_reconnects: Number of failed reconnect attempts.
// @reconnect_work:    Work structure used for scheduling reconnect attempts.
// @fast_io_fail_tmo:  Fast I/O fail timeout in seconds.
// @dev_loss_tmo:      Device loss timeout in seconds.
// @fast_io_fail_work: Work structure used for scheduling fast I/O fail work.
// @dev_loss_work:     Work structure used for scheduling device loss work.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_rport {
// for initiator and target drivers
    pub dev: device,
    pub port_id: [u8; 16],
    pub roles: u8,
// for initiator drivers
    pub lld_data: *mut c_void,
    pub mutex: mutex,
    pub state: srp_rport_state,
    pub reconnect_delay: c_int,
    pub failed_reconnects: c_int,
    pub reconnect_work: delayed_work,
    pub fast_io_fail_tmo: c_int,
    pub dev_loss_tmo: c_int,
    pub fast_io_fail_work: delayed_work,
    pub dev_loss_work: delayed_work,
}

//
// struct srp_function_template - template for SRP initiator drivers
//
// Fields that are only relevant for SRP initiator drivers:
// @has_rport_state: Whether or not to create the state, fast_io_fail_tmo and
// dev_loss_tmo sysfs attribute for an rport.
// @reset_timer_if_blocked: Whether or srp_timed_out() should reset the command
// timer if the device on which it has been queued is blocked.
// @reconnect_delay: If not NULL, points to the default reconnect_delay value.
// @fast_io_fail_tmo: If not NULL, points to the default fast_io_fail_tmo value.
// @dev_loss_tmo: If not NULL, points to the default dev_loss_tmo value.
// @reconnect: Callback function for reconnecting to the target. See also
// srp_reconnect_rport().
// @terminate_rport_io: Callback function for terminating all outstanding I/O
// requests for an rport.
// @rport_delete: Callback function that deletes an rport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_function_template {
// for initiator drivers
    pub has_rport_state: bool,
    pub reset_timer_if_blocked: bool,
    pub reconnect_delay: *mut c_int,
    pub fast_io_fail_tmo: *mut c_int,
    pub dev_loss_tmo: *mut c_int,
    pub rport): *mut *mut int (reconnect)(struct srp_rport,
    pub rport): *mut *mut void (terminate_rport_io)(struct srp_rport,
    pub rport): *mut *mut void (rport_delete)(struct srp_rport,
}

extern "C" {
    pub fn srp_release_transport(: *mut scsi_transport_template);
}
extern "C" {
    pub fn srp_rport_get(rport: *mut srp_rport);
}
extern "C" {
    pub fn srp_rport_put(rport: *mut srp_rport);
}
extern "C" {
    pub fn srp_rport_del(: *mut srp_rport);
}
extern "C" {
    pub fn srp_parse_tmo(tmo: *mut c_int, buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn srp_reconnect_rport(rport: *mut srp_rport) -> c_int;
}
extern "C" {
    pub fn srp_start_tl_fail_timers(rport: *mut srp_rport);
}
extern "C" {
    pub fn srp_remove_host(: *mut Scsi_Host);
}
extern "C" {
    pub fn srp_stop_rport_timers(rport: *mut srp_rport);
}
extern "C" {
    pub fn srp_timed_out(scmd: *mut scsi_cmnd) -> scsi_timeout_action;
}
//
// srp_chkready() - evaluate the transport layer state before I/O
// @rport: SRP target port pointer.
//
// Returns: a SCSI result code that can be returned by the LLD queuecommand()
// implementation. The role of this function is similar to that of
// fc_remote_port_chkready().
//
