//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/core.h
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
// linux/include/linux/mmc/core.h
//

pub const UHS2_MAX_PAYLOAD_LEN: c_int = 2;
pub const UHS2_MAX_RESP_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhs2_command {
    pub header: u16,
    pub arg: u16,
    pub payload: [__be32; UHS2_MAX_PAYLOAD_LEN],
    pub payload_len: u8,
    pub packet_len: u8,
    pub tmode_half_duplex: u8,
    pub /: *mut *mut u8 uhs2_resp[UHS2_MAX_RESP_LEN]; / UHS2 native cmd resp,
    pub /: *mut *mut u8 uhs2_resp_len; / UHS2 native cmd resp len,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_command {
    pub opcode: u32,
    pub arg: u32,
    pub resp: [u32; 4],
    pub /: *mut *mut unsigned int flags; / expected response type,

//
// These are the native response types, and correspond to valid bit
// patterns of the above flags.  One additional valid pattern
// is all zeros, which means we don't expect a response.
//

//
// These are the SPI response types for MMC, SD, and SDIO cards.
// Commands return R1, with maybe more info.  Zero is an error type;
// callers must always provide the appropriate MMC_RSP_SPI_Rx flags.
//

//
// These are the command types.
//

    pub /: *mut *mut unsigned int retries; / max number of retries,
    pub /: *mut *mut int error; / command error,
//
// Standard errno values are used for errors, but some have specific
// meaning in the MMC layer:
//
// ETIMEDOUT    Card took too long to respond
// EILSEQ       Basic format problem with the received or sent data
// (e.g. CRC check failed, incorrect opcode in response
// or bad end bit)
// EINVAL       Request cannot be performed because of restrictions
// in hardware and/or the driver
// ENOMEDIUM    Host can determine that the slot is empty and is
// actively failing requests
//
    pub /: *mut *mut unsigned int busy_timeout; / busy detect timeout in ms,
    pub /: *mut *mut *mut mmc_data data; / data segment associated with cmd,
    pub /: *mut *mut *mut mmc_request mrq; / associated request,
    pub /: *mut *mut *mut uhs2_command uhs2_cmd; / UHS2 command,
// for SDUC
    pub has_ext_addr: bool,
    pub ext_addr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_data {
    pub /: *mut *mut unsigned int timeout_ns; / data timeout (in ns, max 80ms),
    pub /: *mut *mut unsigned int timeout_clks; / data timeout (in clocks),
    pub /: *mut *mut unsigned int blksz; / data block size,
    pub /: *mut *mut unsigned int blocks; / number of blocks,
    pub /: *mut *mut unsigned int blk_addr; / block address,
    pub /: *mut *mut int error; / data error,
    pub flags: c_uint,

// Extra flags used by CQE

    pub bytes_xfered: c_uint,
    pub /: *mut *mut *mut mmc_command stop; / stop command,
    pub /: *mut *mut *mut mmc_request mrq; / associated request,
    pub /: *mut *mut unsigned int sg_len; / size of scatter list,
    pub /: *mut *mut int sg_count; / mapped sg entries,
    pub /: *mut *mut *mut scatterlist sg; / I/O scatter list,
    pub /: *mut *mut s32 host_cookie; / host private data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_request {
    pub /: *mut *mut *mut mmc_command sbc; / SET_BLOCK_COUNT for multiblock,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub stop: *mut mmc_command,
    pub completion: completion,
    pub cmd_completion: completion,
    pub /: *mut *mut *mut *mut void (done)(struct mmc_request );/ completion function,
//
// Notify uppers layers (e.g. mmc block driver) that recovery is needed
// due to an error associated with the mmc_request. Currently used only
// by CQE.
//
    pub ): *mut *mut void (recovery_notifier)(struct mmc_request,
    pub host: *mut mmc_host,
// Allow other commands during this ongoing data transfer or busy wait
    pub cap_cmd_during_tfr: bool,
    pub tag: c_int,

    pub crypto_ctx: *const bio_crypt_ctx,
    pub crypto_key_slot: c_int,

    pub uhs2_cmd: uhs2_command,
}

extern "C" {
    pub fn mmc_wait_for_req(host: *mut mmc_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn mmc_hw_reset(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_sw_reset(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_set_data_timeout(data: *mut mmc_data, card: *const mmc_card);
}
