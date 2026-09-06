//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/atmel-i2c.h
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
// Copyright (c) 2017, Microchip Technology Inc.
// Author: Tudor Ambarus
//

pub const ATMEL_ECC_PRIORITY: c_int = 300;
pub const COMMAND: c_uint = 0x03 /* packet function */;
pub const SLEEP_TOKEN: c_uint = 0x01;
pub const WAKE_TOKEN_MAX_SIZE: c_int = 8;
// Definitions of Data and Command sizes
pub const WORD_ADDR_SIZE: c_int = 1;
pub const COUNT_SIZE: c_int = 1;
pub const CRC_SIZE: c_int = 2;

// size in bytes of the n prime
pub const ATMEL_ECC_NIST_P256_N_SIZE: c_int = 32;

pub const STATUS_RSP_SIZE: c_int = 4;

//
// atmel_i2c_cmd - structure used for communicating with the device.
// @word_addr: indicates the function of the packet sent to the device. This
// byte should have a value of COMMAND for normal operation.
// @count    : number of bytes to be transferred to (or from) the device.
// @opcode   : the command code.
// @param1   : the first parameter; always present.
// @param2   : the second parameter; always present.
// @data     : optional remaining input data. Includes a 2-byte CRC.
// @rxsize   : size of the data received from i2c client.
// @msecs    : command execution time in milliseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2c_cmd {
    pub word_addr: u8,
    pub count: u8,
    pub opcode: u8,
    pub param1: u8,
    pub param2: __le16,
    pub data: [u8; MAX_RSP_SIZE],
    pub msecs: u8,
    pub rxsize: u16,
    pub __packed: },
// Status/Error codes
pub const STATUS_SIZE: c_uint = 0x04;
pub const STATUS_NOERR: c_uint = 0x00;
pub const STATUS_WAKE_SUCCESSFUL: c_uint = 0x11;
// Definitions for eeprom organization
pub const CONFIGURATION_ZONE: c_int = 0;
pub const OTP_ZONE: c_int = 1;
// Definitions for eeprom zone sizes
pub const OTP_ZONE_SIZE: c_int = 64;
// Definitions for Indexes common to all commands

// Definitions for the device lock state
pub const DEVICE_LOCK_ADDR: c_uint = 0x15;

//
// Wake High delay to data communication (microseconds). SDA should be stable
// high for this entire duration.
//
pub const TWHI_MIN: c_int = 1500;
pub const TWHI_MAX: c_int = 1550;
// Wake Low duration
pub const TWLO_USEC: c_int = 60;
// Command execution time (milliseconds)
pub const MAX_EXEC_TIME_ECDH: c_int = 58;
pub const MAX_EXEC_TIME_GENKEY: c_int = 115;
pub const MAX_EXEC_TIME_READ: c_int = 1;
pub const MAX_EXEC_TIME_RANDOM: c_int = 50;
// Command opcode
pub const OPCODE_ECDH: c_uint = 0x43;
pub const OPCODE_GENKEY: c_uint = 0x40;
pub const OPCODE_READ: c_uint = 0x02;
pub const OPCODE_RANDOM: c_uint = 0x1b;
// Definitions for the READ Command
pub const READ_COUNT: c_int = 7;
// Definitions for the RANDOM Command
pub const RANDOM_COUNT: c_int = 7;
// Definitions for the GenKey Command
pub const GENKEY_COUNT: c_int = 7;
pub const GENKEY_MODE_PRIVATE: c_uint = 0x04;
// Definitions for the ECDH Command
pub const ECDH_COUNT: c_int = 71;
pub const ECDH_PREFIX_MODE: c_uint = 0x00;
// Used for binding tfm objects to i2c clients.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ecc_driver_data {
    pub i2c_client_list: list_head,
    pub i2c_list_lock: spinlock_t,
    pub ____cacheline_aligned: },
//
// atmel_i2c_client_priv - i2c_client private data
// @client              : pointer to i2c client device
// @i2c_client_list_node: part of i2c_client_list
// @lock                : lock for sending i2c commands
// @wake_token          : wake token array of zeros
// @wake_token_sz       : size in bytes of the wake_token
// @tfm_count           : number of active crypto transformations on i2c client
// @hwrng               : hold the hardware generated rng
//
// Reads and writes from/to the i2c client are sequential. The first byte
// transmitted to the device is treated as the byte size. Any attempt to send
// more than this number of bytes will cause the device to not ACK those bytes.
// After the host writes a single command byte to the input buffer, reads are
// prohibited until after the device completes command execution. Use a mutex
// when sending i2c commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2c_client_priv {
    pub client: *mut i2c_client,
    pub i2c_client_list_node: list_head,
    pub lock: mutex,
    pub wake_token: [u8; WAKE_TOKEN_MAX_SIZE],
    pub wake_token_sz: usize,
    pub ____cacheline_aligned: atomic_t tfm_count,
    pub hwrng: hwrng,
}

//
// atmel_i2c_work_data - data structure representing the work
// @ctx : transformation context.
// @cbk : pointer to a callback function to be invoked upon completion of this
// request. This has the form:
// callback(struct atmel_i2c_work_data *work_data, void *areq, u8 status)
// where:
// @work_data: data structure representing the work
// @areq     : optional pointer to an argument passed with the original
// request.
// @status   : status returned from the i2c client device or i2c error.
// @areq: optional pointer to a user argument for use at callback time.
// @work: describes the task to be executed.
// @cmd : structure used for communicating with the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2c_work_data {
    pub ctx: *mut c_void,
    pub client: *mut i2c_client,
    pub status): c_int,
    pub areq: *mut c_void,
    pub work: work_struct,
    pub cmd: atmel_i2c_cmd,
}

extern "C" {
    pub fn atmel_i2c_probe(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn atmel_i2c_flush_queue();
}
extern "C" {
    pub fn atmel_i2c_send_receive(client: *mut i2c_client, cmd: *mut atmel_i2c_cmd) -> c_int;
}
extern "C" {
    pub fn atmel_i2c_init_read_config_cmd(cmd: *mut atmel_i2c_cmd);
}
extern "C" {
    pub fn atmel_i2c_init_read_otp_cmd(cmd: *mut atmel_i2c_cmd, addr: u16) -> c_int;
}
extern "C" {
    pub fn atmel_i2c_init_random_cmd(cmd: *mut atmel_i2c_cmd);
}
extern "C" {
    pub fn atmel_i2c_init_genkey_cmd(cmd: *mut atmel_i2c_cmd, keyid: u16);
}
