//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fman/fman_mac.h
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
// Copyright 2008-2015 Freescale Semiconductor Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// FM MAC ...

// Ethernet Address

// defaults

// PFC defines
pub const FSL_FM_PAUSE_TIME_ENABLE: c_uint = 0xf000;
pub const FSL_FM_PAUSE_TIME_DISABLE: c_int = 0;
pub const FSL_FM_PAUSE_THRESH_DEFAULT: c_int = 0;
pub const FM_MAC_NO_PFC: c_uint = 0xff;
// HASH defines

// FM MAC Exceptions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_mac_exceptions {
    FM_MAC_EX_10G_MDIO_SCAN_EVENT = 0
// 10GEC MDIO scan event interrupt
    , FM_MAC_EX_10G_MDIO_CMD_CMPL
// 10GEC MDIO command completion interrupt
    , FM_MAC_EX_10G_REM_FAULT
// 10GEC, mEMAC Remote fault interrupt
    , FM_MAC_EX_10G_LOC_FAULT
// 10GEC, mEMAC Local fault interrupt
    , FM_MAC_EX_10G_TX_ECC_ER
// 10GEC, mEMAC Transmit frame ECC error interrupt
    , FM_MAC_EX_10G_TX_FIFO_UNFL
// 10GEC, mEMAC Transmit FIFO underflow interrupt
    , FM_MAC_EX_10G_TX_FIFO_OVFL
// 10GEC, mEMAC Transmit FIFO overflow interrupt
    , FM_MAC_EX_10G_TX_ER
// 10GEC Transmit frame error interrupt
    , FM_MAC_EX_10G_RX_FIFO_OVFL
// 10GEC, mEMAC Receive FIFO overflow interrupt
    , FM_MAC_EX_10G_RX_ECC_ER
// 10GEC, mEMAC Receive frame ECC error interrupt
    , FM_MAC_EX_10G_RX_JAB_FRM
// 10GEC Receive jabber frame interrupt
    , FM_MAC_EX_10G_RX_OVRSZ_FRM
// 10GEC Receive oversized frame interrupt
    , FM_MAC_EX_10G_RX_RUNT_FRM
// 10GEC Receive runt frame interrupt
    , FM_MAC_EX_10G_RX_FRAG_FRM
// 10GEC Receive fragment frame interrupt
    , FM_MAC_EX_10G_RX_LEN_ER
// 10GEC Receive payload length error interrupt
    , FM_MAC_EX_10G_RX_CRC_ER
// 10GEC Receive CRC error interrupt
    , FM_MAC_EX_10G_RX_ALIGN_ER
// 10GEC Receive alignment error interrupt
    , FM_MAC_EX_1G_BAB_RX
// dTSEC Babbling receive error
    , FM_MAC_EX_1G_RX_CTL
// dTSEC Receive control (pause frame) interrupt
    , FM_MAC_EX_1G_GRATEFUL_TX_STP_COMPLET
// dTSEC Graceful transmit stop complete
    , FM_MAC_EX_1G_BAB_TX
// dTSEC Babbling transmit error
    , FM_MAC_EX_1G_TX_CTL
// dTSEC Transmit control (pause frame) interrupt
    , FM_MAC_EX_1G_TX_ERR
// dTSEC Transmit error
    , FM_MAC_EX_1G_LATE_COL
// dTSEC Late collision
    , FM_MAC_EX_1G_COL_RET_LMT
// dTSEC Collision retry limit
    , FM_MAC_EX_1G_TX_FIFO_UNDRN
// dTSEC Transmit FIFO underrun
    , FM_MAC_EX_1G_MAG_PCKT
// dTSEC Magic Packet detection
    , FM_MAC_EX_1G_MII_MNG_RD_COMPLET
// dTSEC MII management read completion
    , FM_MAC_EX_1G_MII_MNG_WR_COMPLET
// dTSEC MII management write completion
    , FM_MAC_EX_1G_GRATEFUL_RX_STP_COMPLET
// dTSEC Graceful receive stop complete
    , FM_MAC_EX_1G_DATA_ERR
// dTSEC Internal data error on transmit
    , FM_MAC_1G_RX_DATA_ERR
// dTSEC Internal data error on receive
    , FM_MAC_EX_1G_1588_TS_RX_ERR
// dTSEC Time-Stamp Receive Error
    , FM_MAC_EX_1G_RX_MIB_CNT_OVFL
// dTSEC MIB counter overflow
    , FM_MAC_EX_TS_FIFO_ECC_ERR
// mEMAC Time-stamp FIFO ECC error interrupt;
// not supported on T4240/B4860 rev1 chips
//
    , FM_MAC_EX_MAGIC_PACKET_INDICATION = FM_MAC_EX_1G_MAG_PCKT
// mEMAC Magic Packet Indication Interrupt
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_hash_entry {
    pub /: *mut *mut u64 addr; / Ethernet Address,
    pub node: list_head,
}

// FMan MAC config input
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_mac_params {
// MAC ID; numbering of dTSEC and 1G-mEMAC:
// 0 - FM_MAX_NUM_OF_1G_MACS;
// numbering of 10G-MAC (TGEC) and 10G-mEMAC:
// 0 - FM_MAX_NUM_OF_10G_MACS
//
    pub mac_id: u8,
// A handle to the FM object this port related to
    pub fm: *mut c_void,
    pub /: *mut *mut *mut fman_mac_exception_cb event_cb; / MDIO Events Callback Routine,
    pub /: *mut *mut *mut fman_mac_exception_cb exception_cb;/ Exception Callback Routine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_hash_t {
    pub size: u16,
    pub lsts: *mut list_head,
}

// dequeue_addr_from_hash_entry(struct list_head *addr_lst)
// Allocate address hash table
