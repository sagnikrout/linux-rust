//! Automatically rewritten from C to Rust
//! Source: lib/oid_registry.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// ASN.1 Object identifier (OID) registry
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    MODULE_DESCRIPTION("OID Registry");
    MODULE_AUTHOR("Red Hat, Inc.");
    MODULE_LICENSE("GPL");
//
// look_up_OID - Find an OID registration for the specified data
// @data: Binary representation of the OID
// @datasize: Size of the binary representation
//
#[no_mangle]
pub unsafe extern "C" fn look_up_OID(data: *const c_void, datasize: usize) -> enum OID {
    enum OID look_up_OID(const void *data, size_t datasize)
    {
    const unsigned char *octets = data;
    enum OID oid;
    unsigned char xhash;
    unsigned i, j, k, hash;
    size_t len;
// Hash the OID data
    hash = datasize - 1;
    for (i = 0; i < datasize; i++)
    hash += octets[i] * 33;
    hash = (hash >> 24) ^ (hash >> 16) ^ (hash >> 8) ^ hash;
    hash &= 0xff;
// Binary search the OID registry.  OIDs are stored in ascending order
// of hash value then ascending order of size and then in ascending
// order of reverse value.
//
    i = 0;
    k = OID__NR;
    while (i < k) {
    j = (i + k) / 2;
    xhash = oid_search_table[j].hash;
    if (xhash > hash) {
    k = j;
    continue;
    }
    if (xhash < hash) {
    i = j + 1;
    continue;
    }
    oid = oid_search_table[j].oid;
    len = oid_index[oid + 1] - oid_index[oid];
    if (len > datasize) {
    k = j;
    continue;
    }
    if (len < datasize) {
    i = j + 1;
    continue;
    }
// Variation is most likely to be at the tail end of the
// OID, so do the comparison in reverse.
//
    while (len > 0) {
    let mut a: c_uchar = oid_data[oid_index[oid] + --len];
    let mut b: c_uchar = octets[len];
    if (a > b) {
    k = j;
    goto next;
    }
    if (a < b) {
    i = j + 1;
    goto next;
    }
    }
    return oid;
    next:
    ;
    }
    return OID__NR;
    }
    EXPORT_SYMBOL_GPL(look_up_OID);
//
// parse_OID - Parse an OID from a bytestream
// @data: Binary representation of the header + OID
// @datasize: Size of the binary representation
// @oid: Pointer to oid to return result
//
// Parse an OID from a bytestream that holds the OID in the format
// ASN1_OID | length | oid. The length indicator must equal to datasize - 2.
// -EBADMSG is returned if the bytestream is too short.
//
#[no_mangle]
pub unsafe extern "C" fn parse_OID(data: *const c_void, datasize: usize, oid: *mut enum OID) -> c_int {
    int parse_OID(const void *data, size_t datasize, enum OID *oid)
    {
    const unsigned char *v = data;
// we need 2 bytes of header and at least 1 byte for oid
    if (datasize < 3 || v[0] != ASN1_OID || v[1] != datasize - 2)
    return -EBADMSG;
// oid = look_up_OID(data + 2, datasize - 2);
    return 0;
    }
    EXPORT_SYMBOL_GPL(parse_OID);
//
// sprint_oid - Print an Object Identifier into a buffer
// @data: The encoded OID to print
// @datasize: The size of the encoded OID
// @buffer: The buffer to render into
// @bufsize: The size of the buffer
//
// The OID is rendered into the buffer in "a.b.c.d" format and the number of
// bytes is returned.  -EBADMSG is returned if the data could not be interpreted
// and -ENOBUFS if the buffer was too small.
//
#[no_mangle]
pub unsafe extern "C" fn sprint_oid(data: *const c_void, datasize: usize, buffer: *mut c_char, bufsize: usize) -> c_int {
    int sprint_oid(const void *data, size_t datasize, char *buffer, size_t bufsize)
    {
    const unsigned char *v = data, *end = v + datasize;
    unsigned long num;
    unsigned char n;
    size_t ret;
    int count;
    if (v >= end)
    goto bad;
    n = *v++;
    ret = count = snprintf(buffer, bufsize, "%u.%u", n / 40, n % 40);
    if (count >= bufsize)
    return -ENOBUFS;
    buffer += count;
    bufsize -= count;
    while (v < end) {
    n = *v++;
    if (!(n & 0x80)) {
    num = n;
    } else {
    num = n & 0x7f;
    do {
    if (v >= end)
    goto bad;
    n = *v++;
    num <<= 7;
    num |= n & 0x7f;
    } while (n & 0x80);
    }
    ret += count = snprintf(buffer, bufsize, ".%lu", num);
    if (count >= bufsize)
    return -ENOBUFS;
    buffer += count;
    bufsize -= count;
    }
    return ret;
    bad:
    snprintf(buffer, bufsize, "(bad)");
    return -EBADMSG;
    }
    EXPORT_SYMBOL_GPL(sprint_oid);
