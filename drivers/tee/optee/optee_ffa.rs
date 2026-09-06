//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/optee/optee_ffa.h
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
// Copyright (c) 2019-2021, 2023 Linaro Limited
//
// This file is exported by OP-TEE and is kept in sync between secure world
// and normal world drivers. We're using ARM FF-A 1.0 specification.
//

//
// Normal world sends requests with FFA_MSG_SEND_DIRECT_REQ and
// responses are returned with FFA_MSG_SEND_DIRECT_RESP for normal
// messages.
//
// All requests with FFA_MSG_SEND_DIRECT_REQ and FFA_MSG_SEND_DIRECT_RESP
// are using the AArch32 SMC calling convention with register usage as
// defined in FF-A specification:
// w0:    Function ID (0x8400006F or 0x84000070)
// w1:    Source/Destination IDs
// w2:    Reserved (MBZ)
// w3-w7: Implementation defined, free to be used below
//
pub const OPTEE_FFA_VERSION_MAJOR: c_int = 1;
pub const OPTEE_FFA_VERSION_MINOR: c_int = 0;

pub const OPTEE_FFA_YIELDING_CALL_BIT: c_int = 31;

//
// Returns the API version implemented, currently follows the FF-A version.
// Call register usage:
// w3:    Service ID, OPTEE_FFA_GET_API_VERSION
// w4-w7: Not used (MBZ)
//
// Return register usage:
// w3:    OPTEE_FFA_VERSION_MAJOR
// w4:    OPTEE_FFA_VERSION_MINOR
// w5-w7: Not used (MBZ)
//

//
// Returns the revision of OP-TEE.
//
// Used by non-secure world to figure out which version of the Trusted OS
// is installed. Note that the returned revision is the revision of the
// Trusted OS, not of the API.
//
// Call register usage:
// w3:    Service ID, OPTEE_FFA_GET_OS_VERSION
// w4-w7: Unused (MBZ)
//
// Return register usage:
// w3:    CFG_OPTEE_REVISION_MAJOR
// w4:    CFG_OPTEE_REVISION_MINOR
// w5:    TEE_IMPL_GIT_SHA1 (or zero if not supported)
//

//
// Exchange capabilities between normal world and secure world.
//
// Currently there are no defined capabilities. When features are added new
// capabilities may be added.
//
// Call register usage:
// w3:    Service ID, OPTEE_FFA_EXCHANGE_CAPABILITIES
// w4-w7: Not used (MBZ)
//
// Return register usage:
// w3:    Error code, 0 on success
// w4:    Bit[7:0]:  Number of parameters needed for RPC to be supplied
// as the second MSG arg struct for
// OPTEE_FFA_YIELDING_CALL_WITH_ARG.
// Bit[31:8]: Reserved (MBZ)
// w5:	  Bitfield of OP-TEE capabilities OPTEE_FFA_SEC_CAP_
// w6:	  The maximum secure world notification number
// w7:	  Not used (MBZ)
//
// Secure world supports giving an offset into the argument shared memory
// object, see also OPTEE_FFA_YIELDING_CALL_WITH_ARG
//

// OP-TEE supports asynchronous notification via FF-A

// OP-TEE supports probing for RPMB device if needed

// OP-TEE supports Protected Memory for secure data path

//
// Unregister shared memory
//
// Call register usage:
// w3:    Service ID, OPTEE_FFA_YIELDING_CALL_UNREGISTER_SHM
// w4:    Shared memory handle, lower bits
// w5:    Shared memory handle, higher bits
// w6-w7: Not used (MBZ)
//
// Return register usage:
// w3:    Error code, 0 on success
// w4-w7: Not used (MBZ)
//

//
// Inform OP-TEE that the normal world is able to receive asynchronous
// notifications.
//
// Call register usage:
// w3:    Service ID, OPTEE_FFA_ENABLE_ASYNC_NOTIF
// w4:	  Notification value to request bottom half processing, should be
// less than OPTEE_FFA_MAX_ASYNC_NOTIF_VALUE
// w5-w7: Not used (MBZ)
//
// Return register usage:
// w3:    Error code, 0 on success
// w4-w7: Not used (MBZ)
//

pub const OPTEE_FFA_MAX_ASYNC_NOTIF_VALUE: c_int = 64;
//
// Release Protected memory
//
// Call register usage:
// w3:    Service ID, OPTEE_FFA_RECLAIM_PROTMEM
// w4:    Shared memory handle, lower bits
// w5:    Shared memory handle, higher bits
// w6-w7: Not used (MBZ)
//
// Return register usage:
// w3:    Error code, 0 on success
// w4-w7: Note used (MBZ)
//

//
// Call with struct optee_msg_arg as argument in the supplied shared memory
// with a zero internal offset and normal cached memory attributes.
// Register usage:
// w3:    Service ID, OPTEE_FFA_YIELDING_CALL_WITH_ARG
// w4:    Lower 32 bits of a 64-bit Shared memory handle
// w5:    Upper 32 bits of a 64-bit Shared memory handle
// w6:    Offset into shared memory pointing to a struct optee_msg_arg
// right after the parameters of this struct (at offset
// OPTEE_MSG_GET_ARG_SIZE(num_params) follows a struct optee_msg_arg
// for RPC, this struct has reserved space for the number of RPC
// parameters as returned by OPTEE_FFA_EXCHANGE_CAPABILITIES.
// MBZ unless the bit OPTEE_FFA_SEC_CAP_ARG_OFFSET is received with
// OPTEE_FFA_EXCHANGE_CAPABILITIES.
// w7:    Not used (MBZ)
// Resume from RPC. Register usage:
// w3:    Service ID, OPTEE_FFA_YIELDING_CALL_RESUME
// w4-w6: Not used (MBZ)
// w7:    Resume info
//
// Normal return (yielding call is completed). Register usage:
// w3:    Error code, 0 on success
// w4:    OPTEE_FFA_YIELDING_CALL_RETURN_DONE
// w5-w7: Not used (MBZ)
//
// RPC interrupt return (RPC from secure world). Register usage:
// w3:    Error code == 0
// w4:    Any defined RPC code but OPTEE_FFA_YIELDING_CALL_RETURN_DONE
// w5-w6: Not used (MBZ)
// w7:    Resume info
//
// Possible error codes in register w3:
// 0:                       Success
// FFA_DENIED:              w4 isn't one of OPTEE_FFA_YIELDING_CALL_START
// OPTEE_FFA_YIELDING_CALL_RESUME
//
// Possible error codes for OPTEE_FFA_YIELDING_CALL_START,
// FFA_BUSY:               Number of OP-TEE OS threads exceeded,
// try again later
// FFA_DENIED:             RPC shared memory object not found
// FFA_INVALID_PARAMETER:  Bad shared memory handle or offset into the memory
//
// Possible error codes for OPTEE_FFA_YIELDING_CALL_RESUME
// FFA_INVALID_PARAMETER:  Bad resume info
//

pub const OPTEE_FFA_YIELDING_CALL_RETURN_DONE: c_int = 0;
pub const OPTEE_FFA_YIELDING_CALL_RETURN_RPC_CMD: c_int = 1;
pub const OPTEE_FFA_YIELDING_CALL_RETURN_INTERRUPT: c_int = 2;
