//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/macsmc.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SMC (System Management Controller) core definitions
//
// Copyright (C) The Asahi Linux Contributors
//

//
// typedef smc_key - Alias for u32 to be used for SMC keys
//
// SMC keys are 32bit integers containing packed ASCII characters in natural
// integer order, i.e. 0xAABBCCDD, which represent the FourCC ABCD.
// The SMC driver is designed with this assumption and ensures the right
// endianness is used when these are stored to memory and sent to or received
// from the actual SMC firmware (which can be done in either shared memory or
// as 64bit mailbox message on Apple Silicon).
// Internally, SMC stores these keys in a table sorted lexicographically and
// allows resolving an index into this table to the corresponding SMC key.
// Thus, storing keys as u32 is very convenient as it allows to e.g. use
// normal comparison operators which directly map to the natural order used
// by SMC firmware.
//
// This simple type alias is introduced to allow easy recognition of SMC key
// variables and arguments.
//
pub type smc_key = u32;
//
// SMC_KEY - Convert FourCC SMC keys in source code to smc_key
//
// This macro can be used to easily define FourCC SMC keys in source code
// and convert these to u32 / smc_key, e.g. SMC_KEY(NTAP) will expand to
// 0x4e544150.
//
// @s: FourCC SMC key to be converted
//

//
// struct apple_smc_key_info - Information for a SMC key as returned by SMC
// @type_code: FourCC code indicating the type for this key.
// Known types:
// ch8*: ASCII string
// flag: Boolean, 1 or 0
// flt: 32-bit single-precision IEEE 754 float
// hex: Binary data
// ioft: 64bit Unsigned fixed-point intger (48.16)
// {si,ui}{8,16,32,64}: Signed/Unsigned 8-/16-/32-/64-bit integer
// @size: Size of the buffer associated with this key
// @flags: Bitfield encoding flags (APPLE_SMC_{READABLE,WRITABLE,FUNCTION})
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_smc_key_info {
    pub type_code: u32,
    pub size: u8,
    pub flags: u8,
}

//
// enum apple_smc_boot_stage - SMC boot stage
// @APPLE_SMC_BOOTING: SMC is booting
// @APPLE_SMC_INITIALIZED: SMC is initialized and ready to use
// @APPLE_SMC_ERROR_NO_SHMEM: Shared memory could not be initialized during boot
// @APPLE_SMC_ERROR_CRASHED: SMC has crashed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apple_smc_boot_stage {
    APPLE_SMC_BOOTING,
    APPLE_SMC_INITIALIZED,
    APPLE_SMC_ERROR_NO_SHMEM,
    APPLE_SMC_ERROR_CRASHED
}

//
// struct apple_smc
// @dev: Underlying device struct for the physical backend device
// @key_count: Number of available SMC keys
// @first_key: First valid SMC key
// @last_key: Last valid SMC key
// @event_handlers: Notifier call chain for events received from SMC
// @rtk: Pointer to Apple RTKit instance
// @init_done: Completion for initialization
// @boot_stage: Current boot stage of SMC
// @sram: Pointer to SRAM resource
// @sram_base: SRAM base address
// @shmem: RTKit shared memory structure for SRAM
// @msg_id: Current message id for commands, will be incremented for each command
// @atomic_mode: Flag set when atomic mode is entered
// @atomic_pending: Flag indicating pending atomic command
// @cmd_done: Completion for command execution in non-atomic mode
// @cmd_ret: Return value from SMC for last command
// @mutex: Mutex for non-atomic mode
// @lock: Spinlock for atomic mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_smc {
    pub dev: *mut device,
    pub key_count: u32,
    pub first_key: smc_key,
    pub last_key: smc_key,
    pub event_handlers: blocking_notifier_head,
    pub rtk: *mut apple_rtkit,
    pub init_done: completion,
    pub boot_stage: apple_smc_boot_stage,
    pub sram: *mut resource,
    pub sram_base: *mut void __iomem,
    pub shmem: apple_rtkit_shmem,
    pub msg_id: c_uint,
    pub atomic_mode: bool,
    pub atomic_pending: bool,
    pub cmd_done: completion,
    pub cmd_ret: u64,
    pub mutex: mutex,
    pub lock: spinlock_t,
}

//
// apple_smc_read - Read size bytes from given SMC key into buf
// @smc: Pointer to apple_smc struct
// @key: smc_key to be read
// @buf: Buffer into which size bytes of data will be read from SMC
// @size: Number of bytes to be read into buf
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_read(smc: *mut apple_smc, key: smc_key, buf: *mut c_void, size: usize) -> c_int;
}
//
// apple_smc_write - Write size bytes into given SMC key from buf
// @smc: Pointer to apple_smc struct
// @key: smc_key data will be written to
// @buf: Buffer from which size bytes of data will be written to SMC
// @size: Number of bytes to be written
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_write(smc: *mut apple_smc, key: smc_key, buf: *const c_void, size: usize) -> c_int;
}
//
// apple_smc_enter_atomic - Enter atomic mode to be able to use apple_smc_write_atomic
// @smc: Pointer to apple_smc struct
//
// This function switches the SMC backend to atomic mode which allows the
// use of apple_smc_write_atomic while disabling *all* other functions.
// This is only used for shutdown/reboot which requires writing to a SMC
// key from atomic context.
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_enter_atomic(smc: *mut apple_smc) -> c_int;
}
//
// apple_smc_write_atomic - Write size bytes into given SMC key from buf without sleeping
// @smc: Pointer to apple_smc struct
// @key: smc_key data will be written to
// @buf: Buffer from which size bytes of data will be written to SMC
// @size: Number of bytes to be written
//
// Note that this function will fail if apple_smc_enter_atomic hasn't been
// called before.
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_write_atomic(smc: *mut apple_smc, key: smc_key, buf: *const c_void, size: usize) -> c_int;
}
//
// apple_smc_rw - Write and then read using the given SMC key
// @smc: Pointer to apple_smc struct
// @key: smc_key data will be written to
// @wbuf: Buffer from which size bytes of data will be written to SMC
// @wsize: Number of bytes to be written
// @rbuf: Buffer to which size bytes of data will be read from SMC
// @rsize: Number of bytes to be read
//
// Return: Zero on success, negative errno on error
//
// apple_smc_get_key_by_index - Given an index return the corresponding SMC key
// @smc: Pointer to apple_smc struct
// @index: Index to be resolved
// @key: Buffer for SMC key to be returned
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_get_key_by_index(smc: *mut apple_smc, index: c_int, key: *mut smc_key) -> c_int;
}
//
// apple_smc_get_key_info - Get key information from SMC
// @smc: Pointer to apple_smc struct
// @key: Key to acquire information for
// @info: Pointer to struct apple_smc_key_info which will be filled
//
// Return: Zero on success, negative errno on error
//
extern "C" {
    pub fn apple_smc_get_key_info(smc: *mut apple_smc, key: smc_key, info: *mut apple_smc_key_info) -> c_int;
}
//
// apple_smc_key_exists - Check if the given SMC key exists
// @smc: Pointer to apple_smc struct
// @key: smc_key to be checked
//
// Return: True if the key exists, false otherwise
//

// flag = val ? true : false;
extern "C" {
    pub fn apple_smc_write_u8(_arg: smc, _arg: key, 0: state ? 1 :) -> return;
}
extern "C" {
    pub fn apple_smc_write_u8_atomic(_arg: smc, _arg: key, 0: state ? 1 :) -> return;
}
