//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/dma.h
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

// Macro flag: #define B43legacy_DMA_H_

// DMA-Interrupt reasons.

// 32-bit DMA Engine.
// 32-bit DMA controller registers.
pub const B43legacy_DMA32_TXCTL: c_uint = 0x00;
pub const B43legacy_DMA32_TXENABLE: c_uint = 0x00000001;
pub const B43legacy_DMA32_TXSUSPEND: c_uint = 0x00000002;
pub const B43legacy_DMA32_TXLOOPBACK: c_uint = 0x00000004;
pub const B43legacy_DMA32_TXFLUSH: c_uint = 0x00000010;
pub const B43legacy_DMA32_TXADDREXT_MASK: c_uint = 0x00030000;
pub const B43legacy_DMA32_TXADDREXT_SHIFT: c_int = 16;
pub const B43legacy_DMA32_TXRING: c_uint = 0x04;
pub const B43legacy_DMA32_TXINDEX: c_uint = 0x08;
pub const B43legacy_DMA32_TXSTATUS: c_uint = 0x0C;
pub const B43legacy_DMA32_TXDPTR: c_uint = 0x00000FFF;
pub const B43legacy_DMA32_TXSTATE: c_uint = 0x0000F000;
pub const B43legacy_DMA32_TXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43legacy_DMA32_TXSTAT_ACTIVE: c_uint = 0x00001000;
pub const B43legacy_DMA32_TXSTAT_IDLEWAIT: c_uint = 0x00002000;
pub const B43legacy_DMA32_TXSTAT_STOPPED: c_uint = 0x00003000;
pub const B43legacy_DMA32_TXSTAT_SUSP: c_uint = 0x00004000;
pub const B43legacy_DMA32_TXERROR: c_uint = 0x000F0000;
pub const B43legacy_DMA32_TXERR_NOERR: c_uint = 0x00000000;
pub const B43legacy_DMA32_TXERR_PROT: c_uint = 0x00010000;
pub const B43legacy_DMA32_TXERR_UNDERRUN: c_uint = 0x00020000;
pub const B43legacy_DMA32_TXERR_BUFREAD: c_uint = 0x00030000;
pub const B43legacy_DMA32_TXERR_DESCREAD: c_uint = 0x00040000;
pub const B43legacy_DMA32_TXACTIVE: c_uint = 0xFFF00000;
pub const B43legacy_DMA32_RXCTL: c_uint = 0x10;
pub const B43legacy_DMA32_RXENABLE: c_uint = 0x00000001;
pub const B43legacy_DMA32_RXFROFF_MASK: c_uint = 0x000000FE;
pub const B43legacy_DMA32_RXFROFF_SHIFT: c_int = 1;
pub const B43legacy_DMA32_RXDIRECTFIFO: c_uint = 0x00000100;
pub const B43legacy_DMA32_RXADDREXT_MASK: c_uint = 0x00030000;
pub const B43legacy_DMA32_RXADDREXT_SHIFT: c_int = 16;
pub const B43legacy_DMA32_RXRING: c_uint = 0x14;
pub const B43legacy_DMA32_RXINDEX: c_uint = 0x18;
pub const B43legacy_DMA32_RXSTATUS: c_uint = 0x1C;
pub const B43legacy_DMA32_RXDPTR: c_uint = 0x00000FFF;
pub const B43legacy_DMA32_RXSTATE: c_uint = 0x0000F000;
pub const B43legacy_DMA32_RXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43legacy_DMA32_RXSTAT_ACTIVE: c_uint = 0x00001000;
pub const B43legacy_DMA32_RXSTAT_IDLEWAIT: c_uint = 0x00002000;
pub const B43legacy_DMA32_RXSTAT_STOPPED: c_uint = 0x00003000;
pub const B43legacy_DMA32_RXERROR: c_uint = 0x000F0000;
pub const B43legacy_DMA32_RXERR_NOERR: c_uint = 0x00000000;
pub const B43legacy_DMA32_RXERR_PROT: c_uint = 0x00010000;
pub const B43legacy_DMA32_RXERR_OVERFLOW: c_uint = 0x00020000;
pub const B43legacy_DMA32_RXERR_BUFWRITE: c_uint = 0x00030000;
pub const B43legacy_DMA32_RXERR_DESCREAD: c_uint = 0x00040000;
pub const B43legacy_DMA32_RXACTIVE: c_uint = 0xFFF00000;
// 32-bit DMA descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dmadesc32 {
    pub control: __le32,
    pub address: __le32,
    pub __packed: },
pub const B43legacy_DMA32_DCTL_BYTECNT: c_uint = 0x00001FFF;
pub const B43legacy_DMA32_DCTL_ADDREXT_MASK: c_uint = 0x00030000;
pub const B43legacy_DMA32_DCTL_ADDREXT_SHIFT: c_int = 16;
pub const B43legacy_DMA32_DCTL_DTABLEEND: c_uint = 0x10000000;
pub const B43legacy_DMA32_DCTL_IRQ: c_uint = 0x20000000;
pub const B43legacy_DMA32_DCTL_FRAMEEND: c_uint = 0x40000000;
pub const B43legacy_DMA32_DCTL_FRAMESTART: c_uint = 0x80000000;
// Misc DMA constants

pub const B43legacy_DMA0_RX_FRAMEOFFSET: c_int = 30;
pub const B43legacy_DMA3_RX_FRAMEOFFSET: c_int = 0;
// DMA engine tuning knobs
pub const B43legacy_TXRING_SLOTS: c_int = 128;
pub const B43legacy_RXRING_SLOTS: c_int = 64;

pub const B43legacy_DMA3_RX_BUFFERSIZE: c_int = 16;

    pub sk_buff: struct,
    pub b43legacy_private: struct,
    pub b43legacy_txstatus: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dmadesc_meta {
// The kernel DMA-able buffer.
    pub skb: *mut sk_buff,
// DMA base bus-address of the descriptor buffer.
    pub dmaaddr: dma_addr_t,
// ieee80211 TX status. Only used once per 802.11 frag.
    pub is_last_fragment: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43legacy_dmatype {
    B43legacy_DMA_30BIT = 30,
    B43legacy_DMA_32BIT = 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dmaring {
// Kernel virtual base address of the ring memory.
    pub descbase: *mut c_void,
// Meta data about all descriptors.
    pub meta: *mut b43legacy_dmadesc_meta,
// Cache of TX headers for each slot.
// This is to avoid an allocation on each TX.
// This is NULL for an RX ring.
//
    pub txhdr_cache: *mut u8,
// (Unadjusted) DMA base bus-address of the ring memory.
    pub dmabase: dma_addr_t,
// Number of descriptor slots in the ring.
    pub nr_slots: c_int,
// Number of used descriptor slots.
    pub used_slots: c_int,
// Currently used slot in the ring.
    pub current_slot: c_int,
// Frameoffset in octets.
    pub frameoffset: u32,
// Descriptor buffer size.
    pub rx_buffersize: u16,
// The MMIO base register of the DMA controller.
    pub mmio_base: u16,
// DMA controller index number (0-5).
    pub index: c_int,
// Boolean. Is this a TX ring?
    pub tx: bool,
// The type of DMA engine used.
    pub type: b43legacy_dmatype,
// Boolean. Is this ring stopped at ieee80211 level?
    pub stopped: bool,
// The QOS priority assigned to this ring. Only used for TX rings.
// This is the mac80211 "queue" value.
    pub queue_prio: u8,
    pub dev: *mut b43legacy_wldev,

// Maximum number of used slots.
    pub max_used_slots: c_int,
// Last time we injected a ring overflow.
    pub last_injected_overflow: c_ulong,

}

extern "C" {
    pub fn b43legacy_read32(_arg: ring->dev, offset: ring->mmio_base +) -> return;
}
extern "C" {
    pub fn b43legacy_dma_init(dev: *mut b43legacy_wldev) -> c_int;
}
extern "C" {
    pub fn b43legacy_dma_free(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_dma_tx_suspend(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_dma_tx_resume(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_dma_rx(ring: *mut b43legacy_dmaring);
}

