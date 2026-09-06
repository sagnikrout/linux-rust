//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/xdr.h
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
// XDR standard data types and function declarations
//
// Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
//
// Based on:
// RFC 4506 "XDR: External Data Representation Standard", May 2006
//

//
// Size of an XDR encoding unit in bytes, i.e. 32 bits,
// as defined in Section 3 of RFC 4506. All encoded
// XDR data items are aligned on a boundary of 32 bits.
//

//
// Buffer adjustment
//

//
// Generic opaque `network object.'
//
pub const XDR_MAX_NETOBJ: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdr_netobj {
    pub len: c_uint,
    pub data: *mut *mut u8,
}

//
// Basic structure for transmission/reception of a client XDR message.
// Features a header (for a linear buffer containing RPC headers
// and the data payload for short messages), and then an array of
// pages.
// The tail iovec allows you to append data after the page array. Its
// main interest is for appending padding to the pages in order to
// satisfy the int_32-alignment requirements in RFC1832.
//
// For the future, we might want to string several of these together
// in a list if anybody wants to make use of NFSv4 COMPOUND
// operations and/or has a need for scatter/gather involving pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdr_buf {
    pub /: *mut *mut tail[1]; / Appended after page data,
    pub bvec: *mut bio_vec,
    pub /: *mut *mut *mut *mut page  pages; / Array of pages,
    pub /: *mut *mut flags; / Flags for data disposition,
pub const XDRBUF_READ: c_uint = 0x01		/* target of file read */;
pub const XDRBUF_WRITE: c_uint = 0x02		/* source of file write */;
pub const XDRBUF_SPARSE_PAGES: c_uint = 0x04		/* Page array is sparse */;
    pub /: *mut *mut len; / Length of XDR encoded message,
}

//
// pre-xdr'ed macros.
//

//
// Miscellaneous XDR helper functions
//
extern "C" {
    pub fn xdr_terminate_string(: *const xdr_buf, u32: const);
}
extern "C" {
    pub fn xdr_buf_pagecount(buf: *const xdr_buf) -> usize;
}
extern "C" {
    pub fn xdr_alloc_bvec(buf: *mut xdr_buf, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn xdr_free_bvec(buf: *mut xdr_buf);
}
//
// Inline scatterlist entries for xdr_buf_to_sg_alloc().  Sized to cover the
// head kvec, tail kvec, and a few page fragments without any heap allocation.
//
extern "C" {
    pub fn xdr_encode_opaque(_arg: p, _arg: s, _arg: len) -> return;
}
//
// Decode 64bit quantities (NFSv3 support)
//
// valp = get_unaligned_be64(p);
//
// Adjust kvec to reflect end of xdr'ed data (RPC client XDR)
//
// XDR buffer helper functions
//
extern "C" {
    pub fn xdr_buf_from_iov(: *const kvec, : *mut xdr_buf);
}
extern "C" {
    pub fn xdr_buf_subsegment(: *const xdr_buf, : *mut xdr_buf, int: unsigned, int: unsigned) -> c_int;
}
extern "C" {
    pub fn xdr_buf_trim(: *mut xdr_buf, int: unsigned);
}
extern "C" {
    pub fn read_bytes_from_xdr_buf(: *const xdr_buf, int: unsigned, : *mut c_void, int: unsigned) -> c_int;
}
extern "C" {
    pub fn write_bytes_to_xdr_buf(: *const xdr_buf, int: unsigned, : *mut c_void, int: unsigned) -> c_int;
}
extern "C" {
    pub fn xdr_encode_word(: *const xdr_buf, int: unsigned, _arg: u32) -> c_int;
}
extern "C" {
    pub fn xdr_decode_word(: *const xdr_buf, int: unsigned, : *mut u32) -> c_int;
}
extern "C" {
    pub fn int(desc: *mut *mut xdr_xcode_elem_t)(struct xdr_array2_desc, elem: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdr_array2_desc {
    pub elem_size: c_uint,
    pub array_len: c_uint,
    pub array_maxlen: c_uint,
    pub xcode: xdr_xcode_elem_t,
}

//
// Provide some simple tools for XDR buffer overflow-checking etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdr_stream {
    pub /: *mut *mut *mut __be32 p; / start of available buffer,
    pub /: *mut *mut *mut xdr_buf buf; / XDR buffer to read/write,
    pub /: *mut *mut *mut __be32 end; / end of available buffer space,
    pub /: *mut *mut *mut kvec iov; / pointer to the current kvec,
    pub /: *mut *mut kvec scratch; / Scratch buffer,
    pub /: *mut *mut *mut *mut page page_ptr; / pointer to the current page,
    pub /: *mut *mut *mut void page_kaddr; / kmapped address of the current page,
    pub /: *mut *mut unsigned int nwords; / Remaining decode buffer length,
    pub /: *mut *mut *mut rpc_rqst rqst; / For debugging,
}

//
// These are the xdr_stream style generic XDR encode and decode functions.
//
extern "C" {
    pub fn xdr_init_encode_pages(xdr: *mut xdr_stream, buf: *mut xdr_buf);
}
extern "C" {
    pub fn xdr_reserve_space_vec(xdr: *mut xdr_stream, nbytes: usize) -> c_int;
}
extern "C" {
    pub fn __xdr_commit_encode(xdr: *mut xdr_stream);
}
extern "C" {
    pub fn xdr_truncate_encode(xdr: *mut xdr_stream, len: usize);
}
extern "C" {
    pub fn xdr_truncate_decode(xdr: *mut xdr_stream, len: usize);
}
extern "C" {
    pub fn xdr_restrict_buflen(xdr: *mut xdr_stream, newbuflen: c_int) -> c_int;
}
extern "C" {
    pub fn xdr_stream_pos(xdr: *const xdr_stream) -> c_uint;
}
extern "C" {
    pub fn xdr_page_pos(xdr: *const xdr_stream) -> c_uint;
}
extern "C" {
    pub fn xdr_finish_decode(xdr: *mut xdr_stream);
}
extern "C" {
    pub fn xdr_read_pages(xdr: *mut xdr_stream, len: c_uint) -> c_uint;
}
extern "C" {
    pub fn xdr_enter_page(xdr: *mut xdr_stream, len: c_uint);
}
extern "C" {
    pub fn xdr_set_pagelen(: *mut xdr_stream, len: c_uint);
}
//
// xdr_set_scratch_buffer - Attach a scratch buffer for decoding data.
// @xdr: pointer to xdr_stream struct
// @buf: pointer to an empty buffer
// @buflen: size of 'buf'
//
// The scratch buffer is used when decoding from an array of pages.
// If an xdr_inline_decode() call spans across page boundaries, then
// we copy the data into the scratch buffer in order to allow linear
// access.
//
// xdr_set_scratch_folio - Attach a scratch buffer for decoding data
// @xdr: pointer to xdr_stream struct
// @folio: an anonymous folio
//
// See xdr_set_scratch_buffer().
//
// xdr_reset_scratch_buffer - Clear scratch buffer information
// @xdr: pointer to xdr_stream struct
//
// See xdr_set_scratch_buffer().
//
// xdr_commit_encode - Ensure all data is written to xdr->buf
// @xdr: pointer to xdr_stream
//
// Handle encoding across page boundaries by giving the caller a
// temporary location to write to, then later copying the data into
// place. __xdr_commit_encode() does that copying.
//
// xdr_stream_remaining - Return the number of bytes remaining in the stream
// @xdr: pointer to struct xdr_stream
//
// Returns:
// Number of bytes remaining in @xdr before xdr->end
//
// xdr_align_size - Calculate padded size of an object
// @n: Size of an object being XDR encoded (in bytes)
//
// Returns:
// Size (in bytes) of the object including xdr padding
//
// xdr_pad_size - Calculate size of an object's pad
// @n: Size of an object being XDR encoded (in bytes)
//
// This implementation avoids the need for conditional
// branches or modulo division.
//
// Returns:
// Size (in bytes) of the needed XDR pad
//
// xdr_stream_encode_item_present - Encode a "present" list item
// @xdr: pointer to xdr_stream
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// p = xdr_one;
//
// xdr_stream_encode_item_absent - Encode a "not present" list item
// @xdr: pointer to xdr_stream
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// p = xdr_zero;
//
// xdr_encode_bool - Encode a boolean item
// @p: address in a buffer into which to encode
// @n: boolean value to encode
//
// Returns:
// Address of item following the encoded boolean
//
// p++ = n ? xdr_one : xdr_zero;
//
// xdr_stream_encode_bool - Encode a boolean item
// @xdr: pointer to xdr_stream
// @n: boolean value to encode
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// xdr_stream_encode_u32 - Encode a 32-bit integer
// @xdr: pointer to xdr_stream
// @n: integer to encode
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// p = cpu_to_be32(n);
//
// xdr_stream_encode_be32 - Encode a big-endian 32-bit integer
// @xdr: pointer to xdr_stream
// @n: integer to encode
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// p = n;
//
// xdr_stream_encode_u64 - Encode a 64-bit integer
// @xdr: pointer to xdr_stream
// @n: 64-bit integer to encode
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// xdr_stream_encode_opaque_inline - Encode opaque xdr data
// @xdr: pointer to xdr_stream
// @ptr: pointer to void pointer
// @len: size of object
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// ptr = NULL;
// ptr = ++p;
//
// xdr_stream_encode_opaque_fixed - Encode fixed length opaque xdr data
// @xdr: pointer to xdr_stream
// @ptr: pointer to opaque data object
// @len: size of object pointed to by @ptr
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
extern "C" {
    pub fn xdr_align_size(_arg: len) -> return;
}
//
// xdr_stream_encode_opaque - Encode variable length opaque xdr data
// @xdr: pointer to xdr_stream
// @ptr: pointer to opaque data object
// @len: size of object pointed to by @ptr
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// xdr_stream_encode_uint32_array - Encode variable length array of integers
// @xdr: pointer to xdr_stream
// @array: array of integers
// @array_size: number of elements in @array
//
// Returns:
// On success, returns length in bytes of XDR buffer consumed
// %-EMSGSIZE on XDR buffer overflow
//
// p++ = cpu_to_be32(array_size);
// p = cpu_to_be32p(array);
//
// xdr_item_is_absent - symbolically handle XDR discriminators
// @p: pointer to undecoded discriminator
//
// Returns:
// %true if the following XDR item is absent
// %false if the following XDR item is present
//
// xdr_item_is_present - symbolically handle XDR discriminators
// @p: pointer to undecoded discriminator
//
// Returns:
// %true if the following XDR item is present
// %false if the following XDR item is absent
//
// xdr_stream_decode_bool - Decode a boolean
// @xdr: pointer to xdr_stream
// @ptr: pointer to a u32 in which to store the result
//
// Returns:
// %0 on success
// %-EBADMSG on XDR buffer overflow
//
// ptr = (*p != xdr_zero);
//
// xdr_stream_decode_u32 - Decode a 32-bit integer
// @xdr: pointer to xdr_stream
// @ptr: location to store integer
//
// Returns:
// %0 on success
// %-EBADMSG on XDR buffer overflow
//
// ptr = be32_to_cpup(p);
//
// xdr_stream_decode_be32 - Decode a big-endian 32-bit integer
// @xdr: pointer to xdr_stream
// @ptr: location to store integer
//
// Returns:
// %0 on success
// %-EBADMSG on XDR buffer overflow
//
// ptr = *p;
//
// xdr_stream_decode_u64 - Decode a 64-bit integer
// @xdr: pointer to xdr_stream
// @ptr: location to store 64-bit integer
//
// Returns:
// %0 on success
// %-EBADMSG on XDR buffer overflow
//
// xdr_stream_decode_opaque_fixed - Decode fixed length opaque xdr data
// @xdr: pointer to xdr_stream
// @ptr: location to store data
// @len: size of buffer pointed to by @ptr
//
// Returns:
// %0 on success
// %-EBADMSG on XDR buffer overflow
//
// xdr_stream_decode_opaque_inline - Decode variable length opaque xdr data
// @xdr: pointer to xdr_stream
// @ptr: location to store pointer to opaque data
// @maxlen: maximum acceptable object size
//
// Note: the pointer stored in @ptr cannot be assumed valid after the XDR
// buffer has been destroyed, or even after calling xdr_inline_decode()
// on @xdr. It is therefore expected that the object it points to should
// be processed immediately.
//
// Returns:
// On success, returns size of object stored in *@ptr
// %-EBADMSG on XDR buffer overflow
// %-EMSGSIZE if the size of the object would exceed @maxlen
//
// ptr = NULL;
// ptr = p;
//
// xdr_stream_decode_uint32_array - Decode variable length array of integers
// @xdr: pointer to xdr_stream
// @array: location to store the integer array or NULL
// @array_size: number of elements to store
//
// Returns:
// On success, returns number of elements stored in @array
// %-EBADMSG on XDR buffer overflow
// %-EMSGSIZE if the size of the array exceeds @array_size
//
// array = be32_to_cpup(p);
