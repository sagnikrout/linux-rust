//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/optee/optee_smc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Copyright (c) 2015-2021, Linaro Limited
//

//
// Function specified by SMC Calling convention.
//
pub const OPTEE_SMC_FUNCID_CALLS_COUNT: c_uint = 0xFF00;

//
// Normal cached memory (write-back), shareable for SMP systems and not
// shareable for UP systems.
//
pub const OPTEE_SMC_SHM_CACHED: c_int = 1;
//
// a0..a7 is used as register names in the descriptions below, on arm32
// that translates to r0..r7 and on arm64 to w0..w7. In both cases it's
// 32-bit registers.
//
// Function specified by SMC Calling convention
//
// Return the following UID if using API specified in this file
// without further extensions:
// 384fb3e0-e7f8-11e3-af63-0002a5d5c51b.
// see also OPTEE_MSG_UID_* in optee_msg.h
//

//
// Function specified by SMC Calling convention
//
// Returns 2.0 if using API specified in this file without further extensions.
// see also OPTEE_MSG_REVISION_* in optee_msg.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_calls_revision_result {
    pub major: c_ulong,
    pub minor: c_ulong,
    pub reserved0: c_ulong,
    pub reserved1: c_ulong,
}

//
// Get UUID of Trusted OS.
//
// Used by non-secure world to figure out which Trusted OS is installed.
// Note that returned UUID is the UUID of the Trusted OS, not of the API.
//
// Returns UUID in a0-4 in the same way as OPTEE_SMC_CALLS_UID
// described above.
//

//
// Get revision of Trusted OS.
//
// Used by non-secure world to figure out which version of the Trusted OS
// is installed. Note that the returned revision is the revision of the
// Trusted OS, not of the API.
//
// Returns revision in a0-1 in the same way as OPTEE_SMC_CALLS_REVISION
// described above. May optionally return a 32-bit build identifier in a2,
// with zero meaning unspecified.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_call_get_os_revision_result {
    pub major: c_ulong,
    pub minor: c_ulong,
    pub build_id: c_ulong,
    pub reserved1: c_ulong,
}

//
// Load Trusted OS from optee/tee.bin in the Linux firmware.
//
// WARNING: Use this cautiously as it could lead to insecure loading of the
// Trusted OS.
// This SMC instructs EL3 to load a binary and execute it as the Trusted OS.
//
// Call register usage:
// a0 SMC Function ID, OPTEE_SMC_CALL_LOAD_IMAGE
// a1 Upper 32bit of a 64bit size for the payload
// a2 Lower 32bit of a 64bit size for the payload
// a3 Upper 32bit of the physical address for the payload
// a4 Lower 32bit of the physical address for the payload
//
// The payload is in the OP-TEE image format.
//
// Returns result in a0, 0 on success and an error code otherwise.
//
pub const OPTEE_SMC_FUNCID_LOAD_IMAGE: c_int = 2;

//
// Call with struct optee_msg_arg as argument
//
// When called with OPTEE_SMC_CALL_WITH_RPC_ARG or
// OPTEE_SMC_CALL_WITH_REGD_ARG in a0 there is one RPC struct optee_msg_arg
// following after the first struct optee_msg_arg. The RPC struct
// optee_msg_arg has reserved space for the number of RPC parameters as
// returned by OPTEE_SMC_EXCHANGE_CAPABILITIES.
//
// When calling these functions, normal world has a few responsibilities:
// 1. It must be able to handle eventual RPCs
// 2. Non-secure interrupts should not be masked
// 3. If asynchronous notifications has been negotiated successfully, then
// the interrupt for asynchronous notifications should be unmasked
// during this call.
//
// Call register usage, OPTEE_SMC_CALL_WITH_ARG and
// OPTEE_SMC_CALL_WITH_RPC_ARG:
// a0	SMC Function ID, OPTEE_SMC_CALL_WITH_ARG or OPTEE_SMC_CALL_WITH_RPC_ARG
// a1	Upper 32 bits of a 64-bit physical pointer to a struct optee_msg_arg
// a2	Lower 32 bits of a 64-bit physical pointer to a struct optee_msg_arg
// a3	Cache settings, not used if physical pointer is in a predefined shared
// memory area else per OPTEE_SMC_SHM_
// a4-6	Not used
// a7	Hypervisor Client ID register
//
// Call register usage, OPTEE_SMC_CALL_WITH_REGD_ARG:
// a0	SMC Function ID, OPTEE_SMC_CALL_WITH_REGD_ARG
// a1	Upper 32 bits of a 64-bit shared memory cookie
// a2	Lower 32 bits of a 64-bit shared memory cookie
// a3	Offset of the struct optee_msg_arg in the shared memory with the
// supplied cookie
// a4-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	Return value, OPTEE_SMC_RETURN_
// a1-3	Not used
// a4-7	Preserved
//
// OPTEE_SMC_RETURN_ETHREAD_LIMIT return register usage:
// a0	Return value, OPTEE_SMC_RETURN_ETHREAD_LIMIT
// a1-3	Preserved
// a4-7	Preserved
//
// RPC return register usage:
// a0	Return value, OPTEE_SMC_RETURN_IS_RPC(val)
// a1-2	RPC parameters
// a3-7	Resume information, must be preserved
//
// Possible return values:
// OPTEE_SMC_RETURN_UNKNOWN_FUNCTION	Trusted OS does not recognize this
// function.
// OPTEE_SMC_RETURN_OK			Call completed, result updated in
// the previously supplied struct
// optee_msg_arg.
// OPTEE_SMC_RETURN_ETHREAD_LIMIT	Number of Trusted OS threads exceeded,
// try again later.
// OPTEE_SMC_RETURN_EBADADDR		Bad physical pointer to struct
// optee_msg_arg.
// OPTEE_SMC_RETURN_EBADCMD		Bad/unknown cmd in struct optee_msg_arg
// OPTEE_SMC_RETURN_IS_RPC()		Call suspended by RPC call to normal
// world.
//

//
// Get Shared Memory Config
//
// Returns the Secure/Non-secure shared memory config.
//
// Call register usage:
// a0	SMC Function ID, OPTEE_SMC_GET_SHM_CONFIG
// a1-6	Not used
// a7	Hypervisor Client ID register
//
// Have config return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	Physical address of start of SHM
// a2	Size of SHM
// a3	Cache settings of memory, as defined by the
// OPTEE_SMC_SHM_* values above
// a4-7	Preserved
//
// Not available register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL
// a1-3 Not used
// a4-7	Preserved
//
pub const OPTEE_SMC_FUNCID_GET_SHM_CONFIG: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_get_shm_config_result {
    pub status: c_ulong,
    pub start: c_ulong,
    pub size: c_ulong,
    pub settings: c_ulong,
}

//
// Exchanges capabilities between normal world and secure world
//
// Call register usage:
// a0	SMC Function ID, OPTEE_SMC_EXCHANGE_CAPABILITIES
// a1	bitfield of normal world capabilities OPTEE_SMC_NSEC_CAP_
// a2-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	bitfield of secure world capabilities OPTEE_SMC_SEC_CAP_
// a2	The maximum secure world notification number
// a3	Bit[7:0]: Number of parameters needed for RPC to be supplied
// as the second MSG arg struct for
// OPTEE_SMC_CALL_WITH_ARG
// Bit[31:8]: Reserved (MBZ)
// a4-7	Preserved
//
// Error return register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL, can't use the capabilities from normal world
// a1	bitfield of secure world capabilities OPTEE_SMC_SEC_CAP_
// a2-7 Preserved
//
// Normal world works as a uniprocessor system

// Secure world has reserved shared memory for normal world to use

// Secure world can communicate via previously unregistered shared memory

//
// Secure world supports commands "register/unregister shared memory",
// secure world accepts command buffers located in any parts of non-secure RAM
//

// Secure world is built with virtualization support

// Secure world supports Shared Memory with a NULL reference

// Secure world supports asynchronous notification of normal world

// Secure world supports pre-allocating RPC arg struct

// Secure world supports probing for RPMB device if needed

// Secure world supports protected memory

// Secure world supports dynamic protected memory

pub const OPTEE_SMC_FUNCID_EXCHANGE_CAPABILITIES: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_exchange_capabilities_result {
    pub status: c_ulong,
    pub capabilities: c_ulong,
    pub max_notif_value: c_ulong,
    pub data: c_ulong,
}

//
// Disable and empties cache of shared memory objects
//
// Secure world can cache frequently used shared memory objects, for
// example objects used as RPC arguments. When secure world is idle this
// function returns one shared memory reference to free. To disable the
// cache and free all cached objects this function has to be called until
// it returns OPTEE_SMC_RETURN_ENOTAVAIL.
//
// Call register usage:
// a0	SMC Function ID, OPTEE_SMC_DISABLE_SHM_CACHE
// a1-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	Upper 32 bits of a 64-bit Shared memory cookie
// a2	Lower 32 bits of a 64-bit Shared memory cookie
// a3-7	Preserved
//
// Cache empty return register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL
// a1-7	Preserved
//
// Not idle return register usage:
// a0	OPTEE_SMC_RETURN_EBUSY
// a1-7	Preserved
//
pub const OPTEE_SMC_FUNCID_DISABLE_SHM_CACHE: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_disable_shm_cache_result {
    pub status: c_ulong,
    pub shm_upper32: c_ulong,
    pub shm_lower32: c_ulong,
    pub reserved0: c_ulong,
}

//
// Enable cache of shared memory objects
//
// Secure world can cache frequently used shared memory objects, for
// example objects used as RPC arguments. When secure world is idle this
// function returns OPTEE_SMC_RETURN_OK and the cache is enabled. If
// secure world isn't idle OPTEE_SMC_RETURN_EBUSY is returned.
//
// Call register usage:
// a0	SMC Function ID, OPTEE_SMC_ENABLE_SHM_CACHE
// a1-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1-7	Preserved
//
// Not idle return register usage:
// a0	OPTEE_SMC_RETURN_EBUSY
// a1-7	Preserved
//
pub const OPTEE_SMC_FUNCID_ENABLE_SHM_CACHE: c_int = 11;

//
// Query OP-TEE about number of supported threads
//
// Normal World OS or Hypervisor issues this call to find out how many
// threads OP-TEE supports. That is how many standard calls can be issued
// in parallel before OP-TEE will return OPTEE_SMC_RETURN_ETHREAD_LIMIT.
//
// Call requests usage:
// a0	SMC Function ID, OPTEE_SMC_GET_THREAD_COUNT
// a1-6 Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	Number of threads
// a2-7 Preserved
//
// Error return:
// a0	OPTEE_SMC_RETURN_UNKNOWN_FUNCTION   Requested call is not implemented
// a1-7	Preserved
//
pub const OPTEE_SMC_FUNCID_GET_THREAD_COUNT: c_int = 15;

//
// Inform OP-TEE that normal world is able to receive asynchronous
// notifications.
//
// Call requests usage:
// a0	SMC Function ID, OPTEE_SMC_ENABLE_ASYNC_NOTIF
// a1-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1-7	Preserved
//
// Not supported return register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL
// a1-7	Preserved
//
pub const OPTEE_SMC_FUNCID_ENABLE_ASYNC_NOTIF: c_int = 16;

//
// Retrieve a value of notifications pending since the last call of this
// function.
//
// OP-TEE keeps a record of all posted values. When an interrupt is
// received which indicates that there are posted values this function
// should be called until all pended values have been retrieved. When a
// value is retrieved, it's cleared from the record in secure world.
//
// It is expected that this function is called from an interrupt handler
// in normal world.
//
// Call requests usage:
// a0	SMC Function ID, OPTEE_SMC_GET_ASYNC_NOTIF_VALUE
// a1-6	Not used
// a7	Hypervisor Client ID register
//
// Normal return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	value
// a2	Bit[0]: OPTEE_SMC_ASYNC_NOTIF_VALUE_VALID if the value in a1 is
// valid, else 0 if no values where pending
// a2	Bit[1]: OPTEE_SMC_ASYNC_NOTIF_VALUE_PENDING if another value is
// pending, else 0.
// Bit[31:2]: MBZ
// a3-7	Preserved
//
// Not supported return register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL
// a1-7	Preserved
//

//
// Notification that OP-TEE expects a yielding call to do some bottom half
// work in a driver.
//
pub const OPTEE_SMC_ASYNC_NOTIF_VALUE_DO_BOTTOM_HALF: c_int = 0;
pub const OPTEE_SMC_FUNCID_GET_ASYNC_NOTIF_VALUE: c_int = 17;

// See OPTEE_SMC_CALL_WITH_RPC_ARG above
pub const OPTEE_SMC_FUNCID_CALL_WITH_RPC_ARG: c_int = 18;
// See OPTEE_SMC_CALL_WITH_REGD_ARG above
pub const OPTEE_SMC_FUNCID_CALL_WITH_REGD_ARG: c_int = 19;
//
// Get protected memory config
//
// Returns the protected memory config.
//
// Call register usage:
// a0   SMC Function ID, OPTEE_SMC_GET_PROTMEM_CONFIG
// a2-6	Not used, must be zero
// a7	Hypervisor Client ID register
//
// Have config return register usage:
// a0	OPTEE_SMC_RETURN_OK
// a1	Physical address of start of protected memory
// a2	Size of protected memory
// a3	PA width, max 64
// a4-7	Preserved
//
// Not available register usage:
// a0	OPTEE_SMC_RETURN_ENOTAVAIL
// a1-3 Not used
// a4-7	Preserved
//
pub const OPTEE_SMC_FUNCID_GET_PROTMEM_CONFIG: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc_get_protmem_config_result {
    pub status: c_ulong,
    pub start: c_ulong,
    pub size: c_ulong,
    pub pa_width: c_ulong,
}

//
// Resume from RPC (for example after processing a foreign interrupt)
//
// Call register usage:
// a0	SMC Function ID, OPTEE_SMC_CALL_RETURN_FROM_RPC
// a1-3	Value of a1-3 when OPTEE_SMC_CALL_WITH_ARG returned
// OPTEE_SMC_RETURN_RPC in a0
//
// Return register usage is the same as for OPTEE_SMC_*CALL_WITH_ARG above.
//
// Possible return values
// OPTEE_SMC_RETURN_UNKNOWN_FUNCTION	Trusted OS does not recognize this
// function.
// OPTEE_SMC_RETURN_OK			Original call completed, result
// updated in the previously supplied.
// struct optee_msg_arg
// OPTEE_SMC_RETURN_RPC			Call suspended by RPC call to normal
// world.
// OPTEE_SMC_RETURN_ERESUME		Resume failed, the opaque resume
// information was corrupt.
//
pub const OPTEE_SMC_FUNCID_RETURN_FROM_RPC: c_int = 3;

pub const OPTEE_SMC_RETURN_RPC_PREFIX_MASK: c_uint = 0xFFFF0000;
pub const OPTEE_SMC_RETURN_RPC_PREFIX: c_uint = 0xFFFF0000;
pub const OPTEE_SMC_RETURN_RPC_FUNC_MASK: c_uint = 0x0000FFFF;

//
// Allocate memory for RPC parameter passing. The memory is used to hold a
// struct optee_msg_arg.
//
// "Call" register usage:
// a0	This value, OPTEE_SMC_RETURN_RPC_ALLOC
// a1	Size in bytes of required argument memory
// a2	Not used
// a3	Resume information, must be preserved
// a4-5	Not used
// a6-7	Resume information, must be preserved
//
// "Return" register usage:
// a0	SMC Function ID, OPTEE_SMC_CALL_RETURN_FROM_RPC.
// a1	Upper 32 bits of 64-bit physical pointer to allocated
// memory, (a1 == 0 && a2 == 0) if size was 0 or if memory can't
// be allocated.
// a2	Lower 32 bits of 64-bit physical pointer to allocated
// memory, (a1 == 0 && a2 == 0) if size was 0 or if memory can't
// be allocated
// a3	Preserved
// a4	Upper 32 bits of 64-bit Shared memory cookie used when freeing
// the memory or doing an RPC
// a5	Lower 32 bits of 64-bit Shared memory cookie used when freeing
// the memory or doing an RPC
// a6-7	Preserved
//
pub const OPTEE_SMC_RPC_FUNC_ALLOC: c_int = 0;

//
// Free memory previously allocated by OPTEE_SMC_RETURN_RPC_ALLOC
//
// "Call" register usage:
// a0	This value, OPTEE_SMC_RETURN_RPC_FREE
// a1	Upper 32 bits of 64-bit shared memory cookie belonging to this
// argument memory
// a2	Lower 32 bits of 64-bit shared memory cookie belonging to this
// argument memory
// a3-7	Resume information, must be preserved
//
// "Return" register usage:
// a0	SMC Function ID, OPTEE_SMC_CALL_RETURN_FROM_RPC.
// a1-2	Not used
// a3-7	Preserved
//
pub const OPTEE_SMC_RPC_FUNC_FREE: c_int = 2;

//
// Deliver a foreign interrupt in normal world.
//
// "Call" register usage:
// a0	OPTEE_SMC_RETURN_RPC_FOREIGN_INTR
// a1-7	Resume information, must be preserved
//
// "Return" register usage:
// a0	SMC Function ID, OPTEE_SMC_CALL_RETURN_FROM_RPC.
// a1-7	Preserved
//
pub const OPTEE_SMC_RPC_FUNC_FOREIGN_INTR: c_int = 4;

//
// Do an RPC request. The supplied struct optee_msg_arg tells which
// request to do and the parameters for the request. The following fields
// are used (the rest are unused):
// - cmd		the Request ID
// - ret		return value of the request, filled in by normal world
// - num_params		number of parameters for the request
// - params		the parameters
// - param_attrs	attributes of the parameters
//
// "Call" register usage:
// a0	OPTEE_SMC_RETURN_RPC_CMD
// a1	Upper 32 bits of a 64-bit Shared memory cookie holding a
// struct optee_msg_arg, must be preserved, only the data should
// be updated
// a2	Lower 32 bits of a 64-bit Shared memory cookie holding a
// struct optee_msg_arg, must be preserved, only the data should
// be updated
// a3-7	Resume information, must be preserved
//
// "Return" register usage:
// a0	SMC Function ID, OPTEE_SMC_CALL_RETURN_FROM_RPC.
// a1-2	Not used
// a3-7	Preserved
//
pub const OPTEE_SMC_RPC_FUNC_CMD: c_int = 5;

// Returned in a0
pub const OPTEE_SMC_RETURN_UNKNOWN_FUNCTION: c_uint = 0xFFFFFFFF;
// Returned in a0 only from Trusted OS functions
pub const OPTEE_SMC_RETURN_OK: c_uint = 0x0;
pub const OPTEE_SMC_RETURN_ETHREAD_LIMIT: c_uint = 0x1;
pub const OPTEE_SMC_RETURN_EBUSY: c_uint = 0x2;
pub const OPTEE_SMC_RETURN_ERESUME: c_uint = 0x3;
pub const OPTEE_SMC_RETURN_EBADADDR: c_uint = 0x4;
pub const OPTEE_SMC_RETURN_EBADCMD: c_uint = 0x5;
pub const OPTEE_SMC_RETURN_ENOMEM: c_uint = 0x6;
pub const OPTEE_SMC_RETURN_ENOTAVAIL: c_uint = 0x7;

