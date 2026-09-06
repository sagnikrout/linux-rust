//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_hsr.h
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

// used to differentiate various protocols
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsr_version {
    HSR_V0 = 0,
    HSR_V1,
    PRP_V1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsr_port_type {
    HSR_PT_NONE = 0,	/* Must be 0, used by framereg */
    HSR_PT_SLAVE_A,
    HSR_PT_SLAVE_B,
    HSR_PT_INTERLINK,
    HSR_PT_MASTER,
    HSR_PT_PORTS,	/* This must be the last item in the enum */
}

// HSR Tag.
// As defined in IEC-62439-3:2010, the HSR tag is really { ethertype = 0x88FB,
// path, LSDU_size, sequence Nr }. But we let eth_header() create { h_dest,
// h_source, h_proto = 0x88FB }, and add { path, LSDU_size, sequence Nr,
// encapsulated protocol } instead.
//
// Field names as defined in the IEC:2010 standard for HSR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_tag {
    pub path_and_LSDU_size: __be16,
    pub sequence_nr: __be16,
    pub encap_proto: __be16,
    pub __packed: },
pub const HSR_HLEN: c_int = 6;

    pub dev): *mut extern bool is_hsr_master(struct net_device,
    pub ver): *mut *mut extern int hsr_get_version(struct net_device dev, enum hsr_version,
    pub pt): hsr_port_type,
    pub type): *mut hsr_port_type,

    pub false: return,
    pub -EINVAL: return,
    pub ERR_PTR(-EINVAL): return,
    pub -EINVAL: return,

