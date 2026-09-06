//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ar9003_mac.h
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


//
// Copyright (c) 2010-2011 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
pub const AR_DescId: c_uint = 0xffff0000;
pub const AR_DescId_S: c_int = 16;
pub const AR_CtrlStat: c_uint = 0x00004000;
pub const AR_CtrlStat_S: c_int = 14;
pub const AR_TxRxDesc: c_uint = 0x00008000;
pub const AR_TxRxDesc_S: c_int = 15;
pub const AR_TxQcuNum: c_uint = 0x00000f00;
pub const AR_TxQcuNum_S: c_int = 8;
pub const AR_BufLen: c_uint = 0x0fff0000;
pub const AR_BufLen_S: c_int = 16;
pub const AR_TxDescId: c_uint = 0xffff0000;
pub const AR_TxDescId_S: c_int = 16;
pub const AR_TxPtrChkSum: c_uint = 0x0000ffff;
pub const AR_LowRxChain: c_uint = 0x00004000;
pub const AR_Not_Sounding: c_uint = 0x20000000;
// ctl 12
pub const AR_PAPRDChainMask: c_uint = 0x00000e00;
pub const AR_PAPRDChainMask_S: c_int = 9;
pub const MAP_ISR_S2_CST: c_int = 6;
pub const MAP_ISR_S2_GTT: c_int = 6;
pub const MAP_ISR_S2_TIM: c_int = 3;
pub const MAP_ISR_S2_CABEND: c_int = 0;
pub const MAP_ISR_S2_DTIMSYNC: c_int = 7;
pub const MAP_ISR_S2_DTIM: c_int = 7;
pub const MAP_ISR_S2_TSFOOR: c_int = 4;
pub const MAP_ISR_S2_BB_WATCHDOG: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9003_rxs {
    pub ds_info: u32,
    pub status1: u32,
    pub status2: u32,
    pub status3: u32,
    pub status4: u32,
    pub status5: u32,
    pub status6: u32,
    pub status7: u32,
    pub status8: u32,
    pub status9: u32,
    pub status10: u32,
    pub status11: u32,
    pub __aligned(4): } __packed,
// Transmit Control Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9003_txc {
    pub /: *mut *mut u32 info; / descriptor information,
    pub /: *mut *mut u32 link; / link pointer,
    pub /: *mut *mut u32 data0; / data pointer to 1st buffer,
    pub /: *mut *mut u32 ctl3; / DMA control 3,
    pub /: *mut *mut u32 data1; / data pointer to 2nd buffer,
    pub /: *mut *mut u32 ctl5; / DMA control 5,
    pub /: *mut *mut u32 data2; / data pointer to 3rd buffer,
    pub /: *mut *mut u32 ctl7; / DMA control 7,
    pub /: *mut *mut u32 data3; / data pointer to 4th buffer,
    pub /: *mut *mut u32 ctl9; / DMA control 9,
    pub /: *mut *mut u32 ctl10; / DMA control 10,
    pub /: *mut *mut u32 ctl11; / DMA control 11,
    pub /: *mut *mut u32 ctl12; / DMA control 12,
    pub /: *mut *mut u32 ctl13; / DMA control 13,
    pub /: *mut *mut u32 ctl14; / DMA control 14,
    pub /: *mut *mut u32 ctl15; / DMA control 15,
    pub /: *mut *mut u32 ctl16; / DMA control 16,
    pub /: *mut *mut u32 ctl17; / DMA control 17,
    pub /: *mut *mut u32 ctl18; / DMA control 18,
    pub /: *mut *mut u32 ctl19; / DMA control 19,
    pub /: *mut *mut u32 ctl20; / DMA control 20,
    pub /: *mut *mut u32 ctl21; / DMA control 21,
    pub /: *mut *mut u32 ctl22; / DMA control 22,
    pub /: *mut *mut u32 ctl23; / DMA control 23,
    pub /: *mut *mut u32 pad[8]; / pad to cache line (128 bytes/32 dwords),
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9003_txs {
    pub ds_info: u32,
    pub status1: u32,
    pub status2: u32,
    pub status3: u32,
    pub status4: u32,
    pub status5: u32,
    pub status6: u32,
    pub status7: u32,
    pub status8: u32,
    pub __aligned(4): } __packed,
    pub hw): *mut void ar9003_hw_attach_mac_ops(struct ath_hw,
    pub buf_size): *mut *mut void ath9k_hw_set_rx_bufsize(struct ath_hw ah, u16,
    pub qtype): ath9k_rx_qtype,
    pub buf_addr): *mut c_void,
    pub ah): *mut void ath9k_hw_reset_txstatus_ring(struct ath_hw,
    pub size): u16,
