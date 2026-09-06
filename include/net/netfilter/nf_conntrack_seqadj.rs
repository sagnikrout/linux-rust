//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_seqadj.h
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
// struct nf_ct_seqadj - sequence number adjustment information
//
// @correction_pos: position of the last TCP sequence number modification
// @offset_before: sequence number offset before last modification
// @offset_after: sequence number offset after last modification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_seqadj {
    pub correction_pos: u32,
    pub offset_before: i32,
    pub offset_after: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_seqadj {
    pub seq: [nf_ct_seqadj; IP_CT_DIR_MAX],
}

extern "C" {
    pub fn nf_ct_ext_find(_arg: ct, _arg: NF_CT_EXT_SEQADJ) -> return;
}
extern "C" {
    pub fn nf_ct_ext_add(_arg: ct, _arg: NF_CT_EXT_SEQADJ, _arg: GFP_ATOMIC) -> return;
}
extern "C" {
    pub fn nf_ct_seq_offset(ct: *const nf_conn, ip_conntrack_dir: enum, seq: u32) -> i32;
}
