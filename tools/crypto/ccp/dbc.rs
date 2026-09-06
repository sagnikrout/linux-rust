//! Automatically rewritten from C to Rust
//! Source: tools/crypto/ccp/dbc.c
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
// AMD Secure Processor Dynamic Boost Control sample library
//
// Copyright (C) 2023 Advanced Micro Devices, Inc.
//
// Author: Mario Limonciello <mario.limonciello@amd.com>
//

// if uapi header isn't installed, this might not yet exist

#[no_mangle]
pub unsafe extern "C" fn get_nonce(fd: c_int, nonce_out: *mut c_void, signature: *mut c_void) -> c_int {
    int get_nonce(int fd, void *nonce_out, void *signature)
    {
    struct dbc_user_nonce tmp = {
    .auth_needed = !!signature,
    };
    assert(nonce_out);
    if (signature)
    memcpy(tmp.signature, signature, sizeof(tmp.signature));
    if (ioctl(fd, DBCIOCNONCE, &tmp))
    return errno;
    memcpy(nonce_out, tmp.nonce, sizeof(tmp.nonce));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_uid(fd: c_int, uid: *mut __u8, signature: *mut __u8) -> c_int {
    int set_uid(int fd, __u8 *uid, __u8 *signature)
    {
    struct dbc_user_setuid tmp;
    assert(uid);
    assert(signature);
    memcpy(tmp.uid, uid, sizeof(tmp.uid));
    memcpy(tmp.signature, signature, sizeof(tmp.signature));
    if (ioctl(fd, DBCIOCUID, &tmp))
    return errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn process_param(fd: c_int, msg_index: c_int, signature: *mut __u8, data: *mut c_int) -> c_int {
    int process_param(int fd, int msg_index, __u8 *signature, int *data)
    {
    struct dbc_user_param tmp = {
    .msg_index = msg_index,
    .param = *data,
    };
    assert(signature);
    assert(data);
    memcpy(tmp.signature, signature, sizeof(tmp.signature));
    if (ioctl(fd, DBCIOCPARAM, &tmp))
    return errno;
// data = tmp.param;
    memcpy(signature, tmp.signature, sizeof(tmp.signature));
    return 0;
    }
