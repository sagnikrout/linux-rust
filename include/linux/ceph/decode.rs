//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/decode.h
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
// in all cases,
// void **p     pointer to position pointer
// void *end    pointer to end of buffer (last byte + 1)
//
// p += sizeof(u64);
// p += sizeof(u32);
// p += sizeof(u16);
// p += n;
//
// bounds check input.
//

//
// Allocate a buffer big enough to hold the wire-encoded string, and
// decode the string into it.  The resulting string will always be
// terminated with '\0'.  If successful, *p will be advanced
// past the decoded data.  Also, if lenp is not a null pointer, the
// length (not including the terminating '\0') will be recorded in
// *lenp.  Note that a zero-length string is a valid return value.
//
// Returns a pointer to the newly-allocated string buffer, or a
// pointer-coded errno if an error occurs.  Neither *p nor *lenp
// will have been updated if an error is returned.
//
// There are two possible failures:
// - converting the string would require accessing memory at or
// beyond the "end" pointer provided (-ERANGE)
// - memory could not be allocated for the result (-ENOMEM)
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
// p = (char *) *p + sizeof (u32) + len;
// lenp = (size_t) len;
extern "C" {
    pub fn ERR_PTR(_arg: -ERANGE) -> return;
}
//
// skip helpers
//

// p += n;					\

//
// struct ceph_timespec <-> struct timespec64
//
// This will still overflow in year 2106.  We could extend
// the protocol to steal two more bits from tv_nsec to
// add three more 136 year epochs after that the way ext4
// does if necessary.
//
// sockaddr_storage <-> ceph_sockaddr
//
pub const CEPH_ENTITY_ADDR_TYPE_NONE: c_int = 0;

// Banner addresses require TYPE_NONE
extern "C" {
    pub fn ceph_entity_addr_encoding_len(addr: *const ceph_entity_addr) -> c_int;
}
extern "C" {
    pub fn ceph_encode_entity_addr(p: *mut c_void, addr: *const ceph_entity_addr);
}
//
// encoders
//
// p += sizeof(u64);
// p += sizeof(u32);
// p += sizeof(u16);
// (u8 *)*p = v;
// p += len;
//
// filepath, string encoders
//
// p += len;
//
// version and length starting block encoders/decoders
//
// current code version (u8) + compat code version (u8) + len of struct (u32)
pub const CEPH_ENCODING_START_BLK_LEN: c_int = 6;
//
// ceph_start_encoding - start encoding block
// @struct_v: current (code) version of the encoding
// @struct_compat: oldest code version that can decode it
// @struct_len: length of struct encoding
//
// ceph_start_decoding - start decoding block
// @v: current version of the encoding that the code supports
// @name: name of the struct (free-form)
// @struct_v: out param for the encoding version
// @struct_len: out param for the length of struct encoding
//
// Validates the length of struct encoding, so unsafe ceph_decode_
// variants can be used for decoding.
//
// struct_v = ceph_decode_8(p);
// struct_v, struct_compat, v, name);
// struct_len = ceph_decode_32(p);

