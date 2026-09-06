//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/pse-pd/realtek-pse-mcu.h
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

//
// Time the MCU itself needs between accepting a request and having a
// response ready. These are properties of the MCU firmware, not of the
// underlying transport: the core paces transactions by RTPSE_MCU_RESPONSE_MS
// and both transports size their per-transaction recv ceiling from
// RTPSE_MCU_RESPONSE_MAX_MS, since some commands are documented as
// needing up to ~1s to produce a reply.
//
pub const RTPSE_MCU_RESPONSE_MS: c_int = 25;
pub const RTPSE_MCU_RESPONSE_MAX_MS: c_int = 1000;
//
// Total time to keep retrying the first MCU read at probe, and the pause
// between attempts. Right after reset-gpios is deasserted the MCU may not
// answer on the bus yet; give it a bounded window to come up before
// declaring the probe failed.
//
pub const RTPSE_MCU_BOOT_TIMEOUT_MS: c_int = 3000;
pub const RTPSE_MCU_BOOT_RETRY_MS: c_int = 100;
pub const RTPSE_MCU_MSG_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtpse_mcu_msg {
    pub opcode: u8,
    pub seq_num: u8,
    pub payload: [u8; 9],
    pub checksum: u8,
    pub __packed: },
//
// MCU status opcodes (seen on the Gen1 dialect; Gen2 never emits them).
// INCOMPLETE/BAD_CSUM are terminal; NOT_READY is transient.
//
pub const RTPSE_MCU_OPCODE_INCOMPLETE: c_uint = 0xfd	/* -EBADE   */;
pub const RTPSE_MCU_OPCODE_BAD_CSUM: c_uint = 0xfe	/* -EBADMSG */;
pub const RTPSE_MCU_OPCODE_NOT_READY: c_uint = 0xff	/* -EAGAIN  */;
//
// A polling transport can stop here: the reply to this request (opcode and
// seq_num) or a terminal error. The seq_num rejects a stale normal reply; the
// terminal errors match unconditionally, as a request the MCU couldn't parse
// carries no seq_num to correlate against.
//
    pub RTPSE_MCU_OPCODE_BAD_CSUM: resp->opcode ==,
// Opaque to transports; defined in realtek-pse-mcu-core.c.
    pub rtpse_mcu_dialect: struct,
    pub rtpse_mcu_chip_info: struct,
    pub rtpse_mcu_ctrl: struct,
// Per-compatible match data (the of_match .data).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtpse_mcu_match_data {
    pub dialect: *const rtpse_mcu_dialect,
    pub /: *mut *mut bool native_i2c; / raw-I2C framing (vs SMBus); I2C transport only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtpse_mcu_transport_ops {
    pub req): *const *const *const int (send)(struct rtpse_mcu_ctrl pse, struct rtpse_mcu_msg,
    pub resp): *mut rtpse_mcu_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtpse_mcu_ctrl {
    pub dev: *mut device,
    pub pcdev: pse_controller_dev,
    pub /: *mut *mut mutex mutex; / serializes MCU request/response transactions,
    pub dialect: *const rtpse_mcu_dialect,
    pub chip: *const rtpse_mcu_chip_info,
    pub transport: *const rtpse_mcu_transport_ops,
    pub /: *mut *mut u8 seq; / rolling request seq_num, echoed by the MCU,
}

extern "C" {
    pub fn rtpse_mcu_register(pse: *mut rtpse_mcu_ctrl) -> c_int;
}
