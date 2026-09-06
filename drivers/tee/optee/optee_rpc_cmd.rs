//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/optee/optee_rpc_cmd.h
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


// SPDX-License-Identifier: BSD-2-Clause
//
// Copyright (c) 2016-2021, Linaro Limited
//
// All RPC is done with a struct optee_msg_arg as bearer of information,
// struct optee_msg_arg::arg holds values defined by OPTEE_RPC_CMD_* below.
// Only the commands handled by the kernel driver are defined here.
//
// RPC communication with tee-supplicant is reversed compared to normal
// client communication described above. The supplicant receives requests
// and sends responses.
//
// Get time
//
// Returns number of seconds and nano seconds since the Epoch,
// 1970-01-01 00:00:00 +0000 (UTC).
//
// [out]    value[0].a	    Number of seconds
// [out]    value[0].b	    Number of nano seconds.
//
pub const OPTEE_RPC_CMD_GET_TIME: c_int = 3;
//
// Notification from/to secure world.
//
// If secure world needs to wait for something, for instance a mutex, it
// does a notification wait request instead of spinning in secure world.
// Conversely can a synchronous notification can be sent when a secure
// world mutex with a thread waiting thread is unlocked.
//
// This interface can also be used to wait for a asynchronous notification
// which instead is sent via a non-secure interrupt.
//
// Waiting on notification
// [in]    value[0].a	    OPTEE_RPC_NOTIFICATION_WAIT
// [in]    value[0].b	    notification value
// [in]    value[0].c	    timeout in milliseconds or 0 if no timeout
//
// Sending a synchronous notification
// [in]    value[0].a	    OPTEE_RPC_NOTIFICATION_SEND
// [in]    value[0].b	    notification value
//
pub const OPTEE_RPC_CMD_NOTIFICATION: c_int = 4;
pub const OPTEE_RPC_NOTIFICATION_WAIT: c_int = 0;
pub const OPTEE_RPC_NOTIFICATION_SEND: c_int = 1;
//
// Suspend execution
//
// [in]    value[0].a	Number of milliseconds to suspend
//
pub const OPTEE_RPC_CMD_SUSPEND: c_int = 5;
//
// Allocate a piece of shared memory
//
// [in]    value[0].a	    Type of memory one of
// OPTEE_RPC_SHM_TYPE_* below
// [in]    value[0].b	    Requested size
// [in]    value[0].c	    Required alignment
// [out]   memref[0]	    Buffer
//
pub const OPTEE_RPC_CMD_SHM_ALLOC: c_int = 6;
// Memory that can be shared with a non-secure user space application
pub const OPTEE_RPC_SHM_TYPE_APPL: c_int = 0;
// Memory only shared with non-secure kernel
pub const OPTEE_RPC_SHM_TYPE_KERNEL: c_int = 1;
//
// Free shared memory previously allocated with OPTEE_RPC_CMD_SHM_ALLOC
//
// [in]     value[0].a	    Type of memory one of
// OPTEE_RPC_SHM_TYPE_* above
// [in]     value[0].b	    Value of shared memory reference or cookie
//
pub const OPTEE_RPC_CMD_SHM_FREE: c_int = 7;
//
// Issue master requests (read and write operations) to an I2C chip.
//
// [in]     value[0].a	    Transfer mode (OPTEE_RPC_I2C_TRANSFER_*)
// [in]     value[0].b	    The I2C bus (a.k.a adapter).
// 16 bit field.
// [in]     value[0].c	    The I2C chip (a.k.a address).
// 16 bit field (either 7 or 10 bit effective).
// [in]     value[1].a	    The I2C master control flags (ie, 10 bit address).
// 16 bit field.
// [in/out] memref[2]	    Buffer used for data transfers.
// [out]    value[3].a	    Number of bytes transferred by the REE.
//
pub const OPTEE_RPC_CMD_I2C_TRANSFER: c_int = 21;
// I2C master transfer modes
pub const OPTEE_RPC_I2C_TRANSFER_RD: c_int = 0;
pub const OPTEE_RPC_I2C_TRANSFER_WR: c_int = 1;
// I2C master control flags

//
// Reset RPMB probing
//
// Releases an eventually already used RPMB devices and starts over searching
// for RPMB devices. Returns the kind of shared memory to use in subsequent
// OPTEE_RPC_CMD_RPMB_PROBE_NEXT and OPTEE_RPC_CMD_RPMB calls.
//
// [out]    value[0].a	    OPTEE_RPC_SHM_TYPE_*, the parameter for
// OPTEE_RPC_CMD_SHM_ALLOC
//
pub const OPTEE_RPC_CMD_RPMB_PROBE_RESET: c_int = 22;
//
// Probe next RPMB device
//
// [out]    value[0].a	    Type of RPMB device, OPTEE_RPC_RPMB_
// [out]    value[0].b	    EXT CSD-slice 168 "RPMB Size"
// [out]    value[0].c	    EXT CSD-slice 222 "Reliable Write Sector Count"
// [out]    memref[1]       Buffer with the raw CID
//
pub const OPTEE_RPC_CMD_RPMB_PROBE_NEXT: c_int = 23;
// Type of RPMB device
pub const OPTEE_RPC_RPMB_EMMC: c_int = 0;
pub const OPTEE_RPC_RPMB_UFS: c_int = 1;
pub const OPTEE_RPC_RPMB_NVME: c_int = 2;
//
// Replay Protected Memory Block access
//
// [in]     memref[0]	    Frames to device
// [out]    memref[1]	    Frames from device
//
pub const OPTEE_RPC_CMD_RPMB_FRAMES: c_int = 24;
