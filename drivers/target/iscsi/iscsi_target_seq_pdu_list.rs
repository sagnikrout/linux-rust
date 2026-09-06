//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_seq_pdu_list.h
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

// struct iscsi_pdu->status
pub const DATAOUT_PDU_SENT: c_int = 1;
// struct iscsi_seq->type
pub const SEQTYPE_IMMEDIATE: c_int = 1;
pub const SEQTYPE_UNSOLICITED: c_int = 2;
pub const SEQTYPE_NORMAL: c_int = 3;
// struct iscsi_seq->status
pub const DATAOUT_SEQUENCE_GOT_R2T: c_int = 1;
pub const DATAOUT_SEQUENCE_WITHIN_COMMAND_RECOVERY: c_int = 2;
pub const DATAOUT_SEQUENCE_COMPLETE: c_int = 3;
// iscsi_determine_counts_for_list() type
pub const PDULIST_NORMAL: c_int = 1;
pub const PDULIST_IMMEDIATE: c_int = 2;
pub const PDULIST_UNSOLICITED: c_int = 3;
pub const PDULIST_IMMEDIATE_AND_UNSOLICITED: c_int = 4;
// struct iscsi_pdu->type
pub const PDUTYPE_IMMEDIATE: c_int = 1;
pub const PDUTYPE_UNSOLICITED: c_int = 2;
pub const PDUTYPE_NORMAL: c_int = 3;
// struct iscsi_pdu->status
pub const ISCSI_PDU_NOT_RECEIVED: c_int = 0;
pub const ISCSI_PDU_RECEIVED_OK: c_int = 1;
pub const ISCSI_PDU_CRC_FAILED: c_int = 2;
pub const ISCSI_PDU_TIMED_OUT: c_int = 3;
// struct iscsi_build_list->randomize
pub const RANDOM_DATAIN_PDU_OFFSETS: c_uint = 0x01;
pub const RANDOM_DATAIN_SEQ_OFFSETS: c_uint = 0x02;
pub const RANDOM_DATAOUT_PDU_OFFSETS: c_uint = 0x04;
pub const RANDOM_R2T_OFFSETS: c_uint = 0x08;
// struct iscsi_build_list->data_direction
pub const ISCSI_PDU_READ: c_uint = 0x01;
pub const ISCSI_PDU_WRITE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_build_list {
    pub data_direction: c_int,
    pub randomize: c_int,
    pub type: c_int,
    pub immediate_data_length: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_pdu {
    pub status: c_int,
    pub type: c_int,
    pub flags: u8,
    pub data_sn: u32,
    pub length: u32,
    pub offset: u32,
    pub pdu_send_order: u32,
    pub seq_no: u32,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_seq {
    pub sent: c_int,
    pub status: c_int,
    pub type: c_int,
    pub data_sn: u32,
    pub first_datasn: u32,
    pub last_datasn: u32,
    pub next_burst_len: u32,
    pub pdu_start: u32,
    pub pdu_count: u32,
    pub offset: u32,
    pub orig_offset: u32,
    pub pdu_send_order: u32,
    pub r2t_sn: u32,
    pub seq_send_order: u32,
    pub seq_no: u32,
    pub xfer_len: u32,
    pub ____cacheline_aligned: },
    pub iscsit_cmd: struct,
    pub u32): *mut *mut extern int iscsit_build_pdu_and_seq_lists(struct iscsit_cmd ,,
    pub u32): *mut *mut *mut extern struct iscsi_pdu iscsit_get_pdu_holder(struct iscsit_cmd , u32,,
    pub ): *mut *mut *mut extern struct iscsi_pdu iscsit_get_pdu_holder_for_seq(struct iscsit_cmd , struct iscsi_seq,
    pub u32): *mut *mut *mut extern struct iscsi_seq iscsit_get_seq_holder(struct iscsit_cmd , u32,,
