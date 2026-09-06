//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/dma.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// map/unmap direction

// DMA structure:
// support two DMA engines: 32 bits address or 64 bit addressing
// basic DMA register set is per channel(transmit or receive)
// a pair of channels is defined for convenience
//
// 32 bits addressing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma32diag {
    pub /: *mut *mut u32 fifoaddr; / diag address,
    pub /: *mut *mut u32 fifodatalow; / low 32bits of data,
    pub /: *mut *mut u32 fifodatahigh; / high 32bits of data,
    pub /: *mut *mut u32 pad; / reserved,
}

// 64 bits addressing
// dma registers per channel(xmt or rcv)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma64regs {
    pub /: *mut *mut u32 control; / enable, et al,
    pub /: *mut *mut u32 ptr; / last descriptor posted to chip,
    pub /: *mut *mut u32 addrlow; / desc ring base address low 32-bits (8K aligned),
    pub /: *mut *mut u32 addrhigh; / desc ring base address bits 63:32 (8K aligned),
    pub /: *mut *mut u32 status0; / current descriptor, xmt state,
    pub /: *mut *mut u32 status1; / active descriptor, xmt error,
}

// range param for dma_getnexttxp() and dma_txreclaim
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txd_range {
    DMA_RANGE_ALL = 1,
    DMA_RANGE_TRANSMITTED,
    DMA_RANGE_TRANSFERED
}

//
// Exported data structure (read-only)
//
// export structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pub {
    pub /: *mut *mut uint txavail; / # free tx descriptors,
    pub /: *mut *mut uint dmactrlflags; / dma control flags,
// rx error counters
    pub /: *mut *mut uint rxgiants; / rx giant frames,
    pub /: *mut *mut uint rxnobuf; / rx out of dma descriptors,
// tx error counters
    pub /: *mut *mut uint txnobuf; / tx out of dma descriptors,
}

extern "C" {
    pub fn dma_rxinit(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_rx(pub: *mut dma_pub, skb_list: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn dma_rxfill(pub: *mut dma_pub) -> bool;
}
extern "C" {
    pub fn dma_rxreset(pub: *mut dma_pub) -> bool;
}
extern "C" {
    pub fn dma_txreset(pub: *mut dma_pub) -> bool;
}
extern "C" {
    pub fn dma_txinit(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_txpending(pub: *mut dma_pub) -> c_int;
}
extern "C" {
    pub fn dma_kick_tx(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_txsuspend(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_txsuspended(pub: *mut dma_pub) -> bool;
}
extern "C" {
    pub fn dma_txresume(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_txreclaim(pub: *mut dma_pub, range: txd_range);
}
extern "C" {
    pub fn dma_rxreclaim(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_detach(pub: *mut dma_pub);
}
extern "C" {
    pub fn dma_getvar(pub: *mut dma_pub, name: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn dma_counterreset(pub: *mut dma_pub);
}
//
// DMA(Bug) on bcm47xx chips seems to declare that the packet is ready, but
// the packet length is not updated yet (by DMA) on the expected time.
// Workaround is to hold processor till DMA updates the length, and stay off
// the bus to allow DMA update the length in buffer
//

// (u16 *) (head->data) = cpu_to_le16((u16) len);

