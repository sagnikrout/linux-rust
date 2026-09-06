//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_state.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// DOC: DRBD State macros
//
// These macros are used to express state changes in easily readable form.
//
// The NS macros expand to a mask and a value, that can be bit ored onto the
// current state as soon as the spinlock (req_lock) was taken.
//
// The _NS macros are used for state functions that get called with the
// spinlock. These macros expand directly to the new state value.
//
// Besides the basic forms NS() and _NS() additional _?NS[23] are defined
// to express state changes that affect more than one aspect of the state.
//
// E.g. NS2(conn, C_CONNECTED, peer, R_SECONDARY)
// Means that the network connection was established and that the peer
// is in secondary role.
//

pub const susp_MASK: c_int = 1;
pub const user_isp_MASK: c_int = 1;
pub const aftr_isp_MASK: c_int = 1;
pub const susp_nod_MASK: c_int = 1;
pub const susp_fen_MASK: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chg_state_flags {
    CS_HARD	         = 1 << 0,
    CS_VERBOSE       = 1 << 1,
    CS_WAIT_COMPLETE = 1 << 2,
    CS_SERIALIZE     = 1 << 3,
    CS_ORDERED       = CS_WAIT_COMPLETE + CS_SERIALIZE,
    CS_LOCAL_ONLY    = 1 << 4, /* Do not consider a device pair wide state change */
    CS_DC_ROLE       = 1 << 5, /* DC = display as connection state change */
    CS_DC_PEER       = 1 << 6,
    CS_DC_CONN       = 1 << 7,
    CS_DC_DISK       = 1 << 8,
    CS_DC_PDSK       = 1 << 9,
    CS_DC_SUSP       = 1 << 10,
    CS_DC_MASK       = CS_DC_ROLE + CS_DC_PEER + CS_DC_CONN + CS_DC_DISK + CS_DC_PDSK,
    CS_IGN_OUTD_FAIL = 1 << 11,

// Make sure no meta data IO is in flight, by calling
// drbd_md_get_buffer().  Used for graceful detach.
    CS_INHIBIT_MD_IO = 1 << 12,
}

// drbd_dev_state and drbd_state are different types. This is to stress the
#[repr(C)]
#[derive(Copy, Clone)]
pub union drbd_dev_state {

    pub /: *mut *mut unsigned role:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned peer:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned conn:5 ; / 17/32 cstates,
    pub /: *mut *mut unsigned disk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned pdsk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub _unused:1: unsigned,
    pub /: *mut *mut unsigned aftr_isp:1 ; / isp .. imposed sync pause,
    pub peer_isp:1: unsigned,
    pub user_isp:1: unsigned,
    pub /: *mut *mut unsigned _pad:11; / 0 unused,

    pub _pad:11: unsigned,
    pub user_isp:1: unsigned,
    pub peer_isp:1: unsigned,
    pub /: *mut *mut unsigned aftr_isp:1 ; / isp .. imposed sync pause,
    pub _unused:1: unsigned,
    pub /: *mut *mut unsigned pdsk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned disk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned conn:5 ; / 17/32 cstates,
    pub /: *mut *mut unsigned peer:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned role:2 ; / 3/4 primary/secondary/unknown,

}

extern "C" {
    pub fn drbd_resume_al(device: *mut drbd_device);
}
extern "C" {
    pub fn conn_all_vols_unconf(connection: *mut drbd_connection) -> bool;
}
//
// drbd_request_state() - Request a state change
// @device:	DRBD device.
// @mask:	mask of state bits to change.
// @val:	value of new state bits.
//
// This is the most graceful way of requesting a state change. It is verbose
// quite verbose in case the state change is not possible, and all those
// state changes are globally serialized.
//
extern "C" {
    pub fn _drbd_request_state(_arg: device, _arg: mask, _arg: val, CS_ORDERED: CS_VERBOSE +) -> return;
}
// for use in adm_detach() (drbd_adm_detach(), drbd_adm_down())
extern "C" {
    pub fn drbd_request_detach_interruptible(device: *mut drbd_device) -> c_int;
}
extern "C" {
    pub fn conn_highest_role(connection: *mut drbd_connection) -> drbd_role;
}
extern "C" {
    pub fn conn_highest_peer(connection: *mut drbd_connection) -> drbd_role;
}
extern "C" {
    pub fn conn_highest_disk(connection: *mut drbd_connection) -> drbd_disk_state;
}
extern "C" {
    pub fn conn_lowest_disk(connection: *mut drbd_connection) -> drbd_disk_state;
}
extern "C" {
    pub fn conn_highest_pdsk(connection: *mut drbd_connection) -> drbd_disk_state;
}
extern "C" {
    pub fn conn_lowest_conn(connection: *mut drbd_connection) -> drbd_conns;
}
