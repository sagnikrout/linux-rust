//! Automatically rewritten from C to Rust
//! Source: net/nfc/nci/lib.c
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
// The NFC Controller Interface is the communication protocol between an
// NFC Controller (NFCC) and a Device Host (DH).
//
// Copyright (C) 2011 Texas Instruments, Inc.
//
// Written by Ilan Elias <ilane@ti.com>
//
// Acknowledgements:
// This file is based on lib.c, which was written
// by Maxim Krasnyansky.
//

// NCI status codes to Unix errno mapping
#[no_mangle]
pub unsafe extern "C" fn nci_to_errno(code: __u8) -> c_int {
    int nci_to_errno(__u8 code)
    {
    switch (code) {
    case NCI_STATUS_OK:
    return 0;
    case NCI_STATUS_REJECTED:
    return -EBUSY;
    case NCI_STATUS_RF_FRAME_CORRUPTED:
    return -EBADMSG;
    case NCI_STATUS_NOT_INITIALIZED:
    return -EHOSTDOWN;
    case NCI_STATUS_SYNTAX_ERROR:
    case NCI_STATUS_SEMANTIC_ERROR:
    case NCI_STATUS_INVALID_PARAM:
    case NCI_STATUS_RF_PROTOCOL_ERROR:
    case NCI_STATUS_NFCEE_PROTOCOL_ERROR:
    return -EPROTO;
    case NCI_STATUS_UNKNOWN_GID:
    case NCI_STATUS_UNKNOWN_OID:
    return -EBADRQC;
    case NCI_STATUS_MESSAGE_SIZE_EXCEEDED:
    return -EMSGSIZE;
    case NCI_STATUS_DISCOVERY_ALREADY_STARTED:
    return -EALREADY;
    case NCI_STATUS_DISCOVERY_TARGET_ACTIVATION_FAILED:
    case NCI_STATUS_NFCEE_INTERFACE_ACTIVATION_FAILED:
    return -ECONNREFUSED;
    case NCI_STATUS_RF_TRANSMISSION_ERROR:
    case NCI_STATUS_NFCEE_TRANSMISSION_ERROR:
    return -ECOMM;
    case NCI_STATUS_RF_TIMEOUT_ERROR:
    case NCI_STATUS_NFCEE_TIMEOUT_ERROR:
    return -ETIMEDOUT;
    case NCI_STATUS_FAILED:
    default:
    return -ENOSYS;
    }
    }
    EXPORT_SYMBOL(nci_to_errno);
