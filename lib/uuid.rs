//! Automatically rewritten from C to Rust
//! Source: lib/uuid.c
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
// Unified UUID/GUID definition
//
// Copyright (C) 2009, 2016 Intel Corp.
// Huang Ying <ying.huang@intel.com>
//

    const guid_t guid_null;
    EXPORT_SYMBOL(guid_null);
    const uuid_t uuid_null;
    EXPORT_SYMBOL(uuid_null);
    const u8 guid_index[16] = {3,2,1,0,5,4,7,6,8,9,10,11,12,13,14,15};
    const u8 uuid_index[16] = {0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15};
//
// generate_random_uuid - generate a random UUID
// @uuid: where to put the generated UUID
//
// Random UUID interface
//
// Used to create a Boot ID or a filesystem UUID/GUID, but can be
// useful for other kernel drivers.
//
#[no_mangle]
pub unsafe extern "C" fn generate_random_uuid(uuid[16]: c_uchar) {
    void generate_random_uuid(unsigned char uuid[16])
    {
    get_random_bytes(uuid, 16);
// Set UUID version to 4 --- truly random generation
    uuid[6] = (uuid[6] & 0x0F) | 0x40;
// Set the UUID variant to DCE
    uuid[8] = (uuid[8] & 0x3F) | 0x80;
    }
    EXPORT_SYMBOL(generate_random_uuid);
#[no_mangle]
pub unsafe extern "C" fn generate_random_guid(guid[16]: c_uchar) {
    void generate_random_guid(unsigned char guid[16])
    {
    get_random_bytes(guid, 16);
// Set GUID version to 4 --- truly random generation
    guid[7] = (guid[7] & 0x0F) | 0x40;
// Set the GUID variant to DCE
    guid[8] = (guid[8] & 0x3F) | 0x80;
    }
    EXPORT_SYMBOL(generate_random_guid);
#[no_mangle]
unsafe extern "C" fn __uuid_gen_common(b[16]: __u8) {
    static void __uuid_gen_common(__u8 b[16])
    {
    get_random_bytes(b, 16);
// revision 0b10
    b[8] = (b[8] & 0x3F) | 0x80;
    }
#[no_mangle]
pub unsafe extern "C" fn guid_gen(lu: *mut guid_t) {
    void guid_gen(guid_t *lu)
    {
    __uuid_gen_common(lu.b);
// version 4 : random generation
    lu.b[7] = (lu.b[7] & 0x0F) | 0x40;
    }
    EXPORT_SYMBOL_GPL(guid_gen);
#[no_mangle]
pub unsafe extern "C" fn uuid_gen(bu: *mut uuid_t) {
    void uuid_gen(uuid_t *bu)
    {
    __uuid_gen_common(bu.b);
// version 4 : random generation
    bu.b[6] = (bu.b[6] & 0x0F) | 0x40;
    }
    EXPORT_SYMBOL_GPL(uuid_gen);
//
// uuid_is_valid - checks if a UUID string is valid
// @uuid:	UUID string to check
//
// Description:
// It checks if the UUID string is following the format:
// xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
//
// where x is a hex digit.
//
// Return: true if input is valid UUID string.
//
#[no_mangle]
pub unsafe extern "C" fn uuid_is_valid(uuid: *const c_char) -> bool {
    bool uuid_is_valid(const char *uuid)
    {
    unsigned int i;
    for (i = 0; i < UUID_STRING_LEN; i++) {
    if (i == 8 || i == 13 || i == 18 || i == 23) {
    if (uuid[i] != '-')
    return false;
    } else if (!isxdigit(uuid[i])) {
    return false;
    }
    }
    return true;
    }
    EXPORT_SYMBOL(uuid_is_valid);
#[no_mangle]
unsafe extern "C" fn __uuid_parse(uuid: *const c_char, b[16]: __u8, ei[16]: u8) -> c_int {
    static int __uuid_parse(const char *uuid, __u8 b[16], const u8 ei[16])
    {
    static const u8 si[16] = {0,2,4,6,9,11,14,16,19,21,24,26,28,30,32,34};
    unsigned int i;
    if (!uuid_is_valid(uuid))
    return -EINVAL;
    for (i = 0; i < 16; i++) {
    let mut hi: c_int = hex_to_bin(uuid[si[i] + 0]);
    let mut lo: c_int = hex_to_bin(uuid[si[i] + 1]);
    b[ei[i]] = (hi << 4) | lo;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn guid_parse(uuid: *const c_char, u: *mut guid_t) -> c_int {
    int guid_parse(const char *uuid, guid_t *u)
    {
    return __uuid_parse(uuid, u.b, guid_index);
    }
    EXPORT_SYMBOL(guid_parse);
#[no_mangle]
pub unsafe extern "C" fn uuid_parse(uuid: *const c_char, u: *mut uuid_t) -> c_int {
    int uuid_parse(const char *uuid, uuid_t *u)
    {
    return __uuid_parse(uuid, u.b, uuid_index);
    }
    EXPORT_SYMBOL(uuid_parse);
