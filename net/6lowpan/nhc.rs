//! Automatically rewritten from C Header to Rust Module
//! Source: net/6lowpan/nhc.h
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
// LOWPAN_NHC - helper macro to generate nh id fields and lowpan_nhc struct
//
// @__nhc: variable name of the lowpan_nhc struct.
// @_name: const char * of common header compression name.
// @_nexthdr: ipv6 nexthdr field for the header compression.
// @_nexthdrlen: ipv6 nexthdr len for the reserved space.
// @_id: one byte nhc id value.
// @_idmask: one byte nhc id mask value.
// @_uncompress: callback for uncompression call.
// @_compress: callback for compression call.
//

//
// struct lowpan_nhc - hold 6lowpan next hdr compression ifnformation
//
// @name: name of the specific next header compression
// @nexthdr: next header value of the protocol which should be compressed.
// @nexthdrlen: ipv6 nexthdr len for the reserved space.
// @id: one byte nhc id value.
// @idmask: one byte nhc id mask value.
// @compress: callback to do the header compression.
// @uncompress: callback to do the header uncompression.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_nhc {
    pub name: *const c_char,
    pub nexthdr: u8,
    pub nexthdrlen: usize,
    pub id: u8,
    pub idmask: u8,
    pub needed): *mut *mut *mut int (uncompress)(struct sk_buff skb, size_t,
    pub hc_ptr): *mut *mut *mut int (compress)(struct sk_buff skb, u8,
}

//
// lowpan_nhc_by_nexthdr - return the 6lowpan nhc by ipv6 nexthdr.
//
// @nexthdr: ipv6 nexthdr value.
//
// lowpan_nhc_check_compression - checks if we support compression format. If
// we support the nhc by nexthdr field, the function will return 0. If we
// don't support the nhc by nexthdr this function will return -ENOENT.
//
// @skb: skb of 6LoWPAN header to read nhc and replace header.
// @hdr: ipv6hdr to check the nexthdr value
// @hc_ptr: pointer for 6LoWPAN header which should increment at the end of
// replaced header.
//
// lowpan_nhc_do_compression - calling compress callback for nhc
//
// @skb: skb of 6LoWPAN header to read nhc and replace header.
// @hdr: ipv6hdr to set the nexthdr value
// @hc_ptr: pointer for 6LoWPAN header which should increment at the end of
// replaced header.
//
// lowpan_nhc_do_uncompression - calling uncompress callback for nhc
//
// @nhc: 6LoWPAN nhc context, get by lowpan_nhc_by_ functions.
// @skb: skb of 6LoWPAN header, skb->data should be pointed to nhc id value.
// @dev: netdevice for print logging information.
// @hdr: ipv6hdr for setting nexthdr value.
//
// lowpan_nhc_add - register a next header compression to framework
//
// @nhc: nhc which should be add.
//
extern "C" {
    pub fn lowpan_nhc_add(nhc: *const lowpan_nhc) -> c_int;
}
//
// lowpan_nhc_del - delete a next header compression from framework
//
// @nhc: nhc which should be delete.
//
extern "C" {
    pub fn lowpan_nhc_del(nhc: *const lowpan_nhc);
}
//
// lowpan_nhc_init - adding all default nhcs
//
extern "C" {
    pub fn lowpan_nhc_init();
}
