//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/ucc_slow.h
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
// Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors: 	Shlomi Gridish <gridish@freescale.com>
// Li Yang <leoli@freescale.com>
//
// Description:
// Internal header file for UCC SLOW unit routines.
//

// transmit BD's status
pub const T_R: c_uint = 0x80000000	/* ready bit */;
pub const T_PAD: c_uint = 0x40000000	/* add pads to short frames */;
pub const T_W: c_uint = 0x20000000	/* wrap bit */;
pub const T_I: c_uint = 0x10000000	/* interrupt on completion */;
pub const T_L: c_uint = 0x08000000	/* last */;
pub const T_A: c_uint = 0x04000000	/* Address - the data transmitted as address;
pub const T_TC: c_uint = 0x04000000	/* transmit CRC */;
pub const T_CM: c_uint = 0x02000000	/* continuous mode */;
pub const T_DEF: c_uint = 0x02000000	/* collision on previous attempt to transmit */;
pub const T_P: c_uint = 0x01000000	/* Preamble - send Preamble sequence before;
pub const T_HB: c_uint = 0x01000000	/* heartbeat */;
pub const T_NS: c_uint = 0x00800000	/* No Stop */;
pub const T_LC: c_uint = 0x00800000	/* late collision */;
pub const T_RL: c_uint = 0x00400000	/* retransmission limit */;
pub const T_UN: c_uint = 0x00020000	/* underrun */;
pub const T_CT: c_uint = 0x00010000	/* CTS lost */;
pub const T_CSL: c_uint = 0x00010000	/* carrier sense lost */;
pub const T_RC: c_uint = 0x003c0000	/* retry count */;
// Receive BD's status
pub const R_E: c_uint = 0x80000000	/* buffer empty */;
pub const R_W: c_uint = 0x20000000	/* wrap bit */;
pub const R_I: c_uint = 0x10000000	/* interrupt on reception */;
pub const R_L: c_uint = 0x08000000	/* last */;
pub const R_C: c_uint = 0x08000000	/* the last byte in this buffer is a cntl;
pub const R_F: c_uint = 0x04000000	/* first */;
pub const R_A: c_uint = 0x04000000	/* the first byte in this buffer is address;
pub const R_CM: c_uint = 0x02000000	/* continuous mode */;
pub const R_ID: c_uint = 0x01000000	/* buffer close on reception of idles */;
pub const R_M: c_uint = 0x01000000	/* Frame received because of promiscuous;
pub const R_AM: c_uint = 0x00800000	/* Address match */;
pub const R_DE: c_uint = 0x00800000	/* Address match */;
pub const R_LG: c_uint = 0x00200000	/* Break received */;
pub const R_BR: c_uint = 0x00200000	/* Frame length violation */;
pub const R_NO: c_uint = 0x00100000	/* Rx Non Octet Aligned Packet */;
pub const R_FR: c_uint = 0x00100000	/* Framing Error (no stop bit) character;
pub const R_PR: c_uint = 0x00080000	/* Parity Error character received */;
pub const R_AB: c_uint = 0x00080000	/* Frame Aborted */;
pub const R_SH: c_uint = 0x00080000	/* frame is too short */;
pub const R_CR: c_uint = 0x00040000	/* CRC Error */;
pub const R_OV: c_uint = 0x00020000	/* Overrun */;
pub const R_CD: c_uint = 0x00010000	/* CD lost */;
pub const R_CL: c_uint = 0x00010000	/* this frame is closed because of a;
// Rx Data buffer must be 4 bytes aligned in most cases.
pub const UCC_SLOW_RX_ALIGN: c_int = 4;
pub const UCC_SLOW_MRBLR_ALIGNMENT: c_int = 4;
pub const UCC_SLOW_PRAM_SIZE: c_uint = 0x100;
pub const ALIGNMENT_OF_UCC_SLOW_PRAM: c_int = 64;
// UCC Slow Channel Protocol Mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_channel_protocol_mode {
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_QMC = 0x00000002,
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_UART = 0x00000004,
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_BISYNC = 0x00000008,
}

// UCC Slow Transparent Transmit CRC (TCRC)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_transparent_tcrc {
// 16-bit CCITT CRC (HDLC).  (X16 + X12 + X5 + 1)
    UCC_SLOW_TRANSPARENT_TCRC_CCITT_CRC16 = 0x00000000,
// CRC16 (BISYNC).  (X16 + X15 + X2 + 1)
    UCC_SLOW_TRANSPARENT_TCRC_CRC16 = 0x00004000,
// 32-bit CCITT CRC (Ethernet and HDLC)
    UCC_SLOW_TRANSPARENT_TCRC_CCITT_CRC32 = 0x00008000,
}

// UCC Slow oversampling rate for transmitter (TDCR)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_tx_oversampling_rate {
// 1x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_1 = 0x00000000,
// 8x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_8 = 0x00010000,
// 16x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_16 = 0x00020000,
// 32x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_32 = 0x00030000,
}

// UCC Slow Oversampling rate for receiver (RDCR)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_rx_oversampling_rate {
// 1x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_1 = 0x00000000,
// 8x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_8 = 0x00004000,
// 16x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_16 = 0x00008000,
// 32x clock mode
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_32 = 0x0000c000,
}

// UCC Slow Transmitter encoding method (TENC)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_tx_encoding_method {
    UCC_SLOW_TRANSMITTER_ENCODING_METHOD_TENC_NRZ = 0x00000000,
    UCC_SLOW_TRANSMITTER_ENCODING_METHOD_TENC_NRZI = 0x00000100
}

// UCC Slow Receiver decoding method (RENC)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_rx_decoding_method {
    UCC_SLOW_RECEIVER_DECODING_METHOD_RENC_NRZ = 0x00000000,
    UCC_SLOW_RECEIVER_DECODING_METHOD_RENC_NRZI = 0x00000800
}

// UCC Slow Diagnostic mode (DIAG)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_slow_diag_mode {
    UCC_SLOW_DIAG_MODE_NORMAL = 0x00000000,
    UCC_SLOW_DIAG_MODE_LOOPBACK = 0x00000040,
    UCC_SLOW_DIAG_MODE_ECHO = 0x00000080,
    UCC_SLOW_DIAG_MODE_LOOPBACK_ECHO = 0x000000c0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_slow_info {
    pub ucc_num: c_int,
    pub /: *mut *mut int protocol; / QE_CR_PROTOCOL_xxx,
    pub rx_clock: qe_clock,
    pub tx_clock: qe_clock,
    pub regs: phys_addr_t,
    pub irq: c_int,
    pub uccm_mask: u16,
    pub data_mem_part: c_int,
    pub init_tx: c_int,
    pub init_rx: c_int,
    pub tx_bd_ring_len: u32,
    pub rx_bd_ring_len: u32,
    pub rx_interrupts: c_int,
    pub brkpt_support: c_int,
    pub grant_support: c_int,
    pub tsa: c_int,
    pub cdp: c_int,
    pub cds: c_int,
    pub ctsp: c_int,
    pub ctss: c_int,
    pub rinv: c_int,
    pub tinv: c_int,
    pub rtsm: c_int,
    pub rfw: c_int,
    pub tci: c_int,
    pub tend: c_int,
    pub tfl: c_int,
    pub txsy: c_int,
    pub max_rx_buf_length: u16,
    pub tcrc: ucc_slow_transparent_tcrc,
    pub mode: ucc_slow_channel_protocol_mode,
    pub diag: ucc_slow_diag_mode,
    pub tdcr: ucc_slow_tx_oversampling_rate,
    pub rdcr: ucc_slow_rx_oversampling_rate,
    pub tenc: ucc_slow_tx_encoding_method,
    pub renc: ucc_slow_rx_decoding_method,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_slow_private {
    pub us_info: *mut ucc_slow_info,
    pub /: *mut *mut *mut ucc_slow __iomem us_regs; / Ptr to memory map of UCC regs,
    pub /: *mut *mut *mut ucc_slow_pram __iomem us_pram; / a pointer to the parameter RAM,
    pub us_pram_offset: i32,
    pub /: *mut *mut int enabled_tx; / Whether channel is enabled for Tx (ENT),
    pub /: *mut *mut int enabled_rx; / Whether channel is enabled for Rx (ENR),
    pub Tx: *mut *mut int stopped_tx; / Whether channel has been stopped for,
    pub /: *mut *mut int stopped_rx; / Whether channel has been stopped for Rx,
    pub /: *mut *mut list_head confQ; / frames passed to chip waiting for tx,
    pub status: *mut *mut u32 first_tx_bd_mask; / mask is used in Tx routine to save,
    pub /: *mut *mut s32 tx_base_offset; / first BD in Tx BD table offset (In MURAM),
    pub /: *mut *mut s32 rx_base_offset; / first BD in Rx BD table offset (In MURAM),
    pub /: *mut *mut *mut qe_bd __iomem confBd; / next BD for confirm after Tx,
    pub /: *mut *mut *mut qe_bd __iomem tx_bd; / next BD for new Tx request,
    pub /: *mut *mut *mut qe_bd __iomem rx_bd; / next BD to collect after Rx,
    pub /: *mut *mut *mut void p_rx_frame; / accumulating receive frame,
    pub /: *mut *mut *mut __be16 __iomem p_ucce; / a pointer to the event register in memory,
    pub /: *mut *mut *mut __be16 __iomem p_uccm; / a pointer to the mask register in memory,
    pub /: *mut *mut u16 saved_uccm; / a saved mask for the RX Interrupt bits,

    pub /: *mut *mut u32 tx_frames; / Transmitted frames counters,
    pub frames: *mut *mut u32 rx_frames; / Received frames counters (only,
    pub that: *mut *mut u32 rx_discarded; / Discarded frames counters (frames,

}

// ucc_slow_init
// Initializes Slow UCC according to provided parameters.
//
// us_info  - (In) pointer to the slow UCC info structure.
// uccs_ret - (Out) pointer to the slow UCC structure.
//
extern "C" {
    pub fn ucc_slow_init(us_info: *mut *mut ucc_slow_info, uccs_ret: *mut *mut *mut ucc_slow_private) -> c_int;
}
// ucc_slow_free
// Frees all resources for slow UCC.
//
// uccs - (In) pointer to the slow UCC structure.
//
extern "C" {
    pub fn ucc_slow_free(uccs: *mut *mut ucc_slow_private);
}
// ucc_slow_enable
// Enables a fast UCC port.
// This routine enables Tx and/or Rx through the General UCC Mode Register.
//
// uccs - (In) pointer to the slow UCC structure.
// mode - (In) TX, RX, or both.
//
extern "C" {
    pub fn ucc_slow_enable(uccs: *mut *mut ucc_slow_private, mode: comm_dir);
}
// ucc_slow_disable
// Disables a fast UCC port.
// This routine disables Tx and/or Rx through the General UCC Mode Register.
//
// uccs - (In) pointer to the slow UCC structure.
// mode - (In) TX, RX, or both.
//
extern "C" {
    pub fn ucc_slow_disable(uccs: *mut *mut ucc_slow_private, mode: comm_dir);
}
// ucc_slow_graceful_stop_tx
// Smoothly stops transmission on a specified slow UCC.
//
// uccs - (In) pointer to the slow UCC structure.
//
extern "C" {
    pub fn ucc_slow_graceful_stop_tx(uccs: *mut *mut ucc_slow_private);
}
// ucc_slow_stop_tx
// Stops transmission on a specified slow UCC.
//
// uccs - (In) pointer to the slow UCC structure.
//
extern "C" {
    pub fn ucc_slow_stop_tx(uccs: *mut *mut ucc_slow_private);
}
// ucc_slow_restart_tx
// Restarts transmitting on a specified slow UCC.
//
// uccs - (In) pointer to the slow UCC structure.
//
extern "C" {
    pub fn ucc_slow_restart_tx(uccs: *mut ucc_slow_private);
}
extern "C" {
    pub fn ucc_slow_get_qe_cr_subblock(uccs_num: c_int) -> u32;
}
