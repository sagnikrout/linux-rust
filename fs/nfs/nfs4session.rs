//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/nfs4session.h
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
// fs/nfs/nfs4session.h
//
// Copyright (c) 2012 Trond Myklebust <Trond.Myklebust@netapp.com>
//
// maximum number of slots to use

// Sessions slot seqid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_slot {
    pub table: *mut nfs4_slot_table,
    pub next: *mut nfs4_slot,
    pub generation: c_ulong,
    pub slot_nr: u32,
    pub seq_nr: u32,
    pub seq_nr_last_acked: u32,
    pub seq_nr_highest_sent: u32,
    pub 1: seq_done :,
}

// Sessions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_slot_tbl_state {
    NFS4_SLOT_TBL_DRAINING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_slot_table {
    pub /: *mut *mut *mut nfs4_session session; / Parent session,
    pub /: *mut *mut *mut nfs4_slot slots; / seqid per slot,
    pub /: *mut *mut unsigned long used_slots[SLOT_TABLE_SZ]; / used/unused bitmap,
    pub slot_tbl_lock: spinlock_t,
    pub /: *mut *mut rpc_wait_queue slot_tbl_waitq; / allocators may wait here,
    pub /: *mut *mut wait_queue_head_t slot_waitq; / Completion wait on slot,
    pub /: *mut *mut u32 max_slots; / # slots in table,
    pub /: *mut *mut u32 max_slotid; / Max allowed slotid value,
    pub SEQ.: *mut *mut u32 highest_used_slotid; / sent to server on each,
// op for dynamic resizing
    pub /: *mut *mut u32 target_highest_slotid; / Server max_slot target,
    pub /: *mut *mut u32 server_highest_slotid; / Server highest slotid,
    pub /: *mut *mut s32 d_target_highest_slotid; / Derivative,
    pub /: *mut *mut s32 d2_target_highest_slotid; / 2nd derivative,
    pub for: *mut *mut unsigned long generation; / Generation counter,
    pub complete: completion,
    pub slot_tbl_state: c_ulong,
}

//
// Session related parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_session {
    pub sess_id: nfs4_sessionid,
    pub flags: u32,
    pub session_state: c_ulong,
    pub hash_alg: u32,
    pub ssv_len: u32,
// The fore and back channel
    pub fc_attrs: nfs4_channel_attrs,
    pub fc_slot_table: nfs4_slot_table,
    pub bc_attrs: nfs4_channel_attrs,
    pub bc_slot_table: nfs4_slot_table,
    pub clp: *mut nfs_client,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_session_state {
    NFS4_SESSION_INITING,
    NFS4_SESSION_ESTABLISHED,
}

extern "C" {
    pub fn nfs4_shutdown_slot_table(tbl: *mut nfs4_slot_table);
}
extern "C" {
    pub fn nfs4_try_to_lock_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool;
}
extern "C" {
    pub fn nfs4_free_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot);
}
extern "C" {
    pub fn nfs4_slot_tbl_drain_complete(tbl: *mut nfs4_slot_table);
}
extern "C" {
    pub fn nfs41_wake_slot_table(tbl: *mut nfs4_slot_table);
}
extern "C" {
    pub fn nfs4_setup_session_slot_tables(ses: *mut nfs4_session) -> c_int;
}
extern "C" {
    pub fn nfs4_destroy_session(session: *mut nfs4_session);
}
extern "C" {
    pub fn nfs4_init_session(clp: *mut nfs_client) -> c_int;
}
//
// Determine if sessions are in use.
//
// nfs_session_id_hash - calculate the crc32 hash for the session id
// @session - pointer to session
//

