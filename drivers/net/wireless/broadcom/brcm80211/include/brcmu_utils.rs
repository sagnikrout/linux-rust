//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/brcmu_utils.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

//
// Spin at most 'us' microseconds while 'exp' is true.
// Caller should explicitly test 'exp' when this completes
// and take appropriate error action if 'exp' is still true.
//

// osl multi-precedence packet queue

// the largest reasonable packet buffer driver uses for ethernet MTU in bytes
pub const PKTBUFSZ: c_int = 2048;

// crc defines
pub const CRC16_INIT_VALUE: c_uint = 0xffff	/* Initial CRC16 checksum value */;
pub const CRC16_GOOD_VALUE: c_uint = 0xf0b8	/* Good final CRC16 checksum value */;
// 18-bytes of Ethernet address buffer length
pub const ETHER_ADDR_STR_LEN: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pktq_prec {
    pub skblist: sk_buff_head,
    pub /: *mut *mut u16 max; / maximum number of queued packets,
}

// multi-priority pkt queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pktq {
    pub /: *mut *mut u16 num_prec; / number of precedences in use,
    pub /: *mut *mut u16 hi_prec; / rapid dequeue hint (>= highest non-empty prec),
    pub /: *mut *mut u16 max; / total max packets,
    pub /: *mut *mut u16 len; / total number of packets,
//
// q array must be last since # of elements can be either
// PKTQ_MAX_PREC or 1
//
    pub q: [pktq_prec; PKTQ_MAX_PREC],
}

// operations on a specific precedence in packet queue
extern "C" {
    pub fn skb_queue_empty(_arg: &pq->q[prec].skblist) -> return;
}
extern "C" {
    pub fn skb_peek(_arg: &pq->q[prec].skblist) -> return;
}
extern "C" {
    pub fn skb_peek_tail(_arg: &pq->q[prec].skblist) -> return;
}
// packet primitives
extern "C" {
    pub fn brcmu_pkt_buf_free_skb(skb: *mut sk_buff);
}
// Empty the queue at particular precedence level
// callback function fn(pkt, arg) returns true if pkt belongs to if
// operations on a set of precedences in packet queue
extern "C" {
    pub fn brcmu_pktq_mlen(pq: *mut pktq, prec_bmp: c_uint) -> c_int;
}
// operations on packet queue as a whole
extern "C" {
    pub fn brcmu_pktq_init(pq: *mut pktq, num_prec: c_int, max_len: c_int);
}
// prec_out may be NULL if caller is not interested in return value
// externs
// ip address
//
// bitfield macros using masking and shift
//
// remark: the mask parameter should be a shifted mask.
//
// var = (*var & ~mask) | value;
// externs
// format/print

extern "C" {
    pub fn brcmu_prpkt(msg: *const c_char, p0: *mut sk_buff);
}

extern "C" {
    pub fn brcmu_dbg_hex_dump(data: *const c_void, size: usize, fmt: *const c_char, ...);
}

pub const BRCMU_BOARDREV_LEN: c_int = 8;
pub const BRCMU_DOTREV_LEN: c_int = 16;
