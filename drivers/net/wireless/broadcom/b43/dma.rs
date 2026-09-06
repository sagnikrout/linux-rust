//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/dma.h
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

// DMA-Interrupt reasons.

// 32-bit DMA Engine.
// 32-bit DMA controller registers.
pub const B43_DMA32_TXCTL: c_uint = 0x00;
pub const B43_DMA32_TXENABLE: c_uint = 0x00000001;
pub const B43_DMA32_TXSUSPEND: c_uint = 0x00000002;
pub const B43_DMA32_TXLOOPBACK: c_uint = 0x00000004;
pub const B43_DMA32_TXFLUSH: c_uint = 0x00000010;
pub const B43_DMA32_TXPARITYDISABLE: c_uint = 0x00000800;
pub const B43_DMA32_TXADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA32_TXADDREXT_SHIFT: c_int = 16;
pub const B43_DMA32_TXRING: c_uint = 0x04;
pub const B43_DMA32_TXINDEX: c_uint = 0x08;
pub const B43_DMA32_TXSTATUS: c_uint = 0x0C;
pub const B43_DMA32_TXDPTR: c_uint = 0x00000FFF;
pub const B43_DMA32_TXSTATE: c_uint = 0x0000F000;
pub const B43_DMA32_TXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43_DMA32_TXSTAT_ACTIVE: c_uint = 0x00001000;
pub const B43_DMA32_TXSTAT_IDLEWAIT: c_uint = 0x00002000;
pub const B43_DMA32_TXSTAT_STOPPED: c_uint = 0x00003000;
pub const B43_DMA32_TXSTAT_SUSP: c_uint = 0x00004000;
pub const B43_DMA32_TXERROR: c_uint = 0x000F0000;
pub const B43_DMA32_TXERR_NOERR: c_uint = 0x00000000;
pub const B43_DMA32_TXERR_PROT: c_uint = 0x00010000;
pub const B43_DMA32_TXERR_UNDERRUN: c_uint = 0x00020000;
pub const B43_DMA32_TXERR_BUFREAD: c_uint = 0x00030000;
pub const B43_DMA32_TXERR_DESCREAD: c_uint = 0x00040000;
pub const B43_DMA32_TXACTIVE: c_uint = 0xFFF00000;
pub const B43_DMA32_RXCTL: c_uint = 0x10;
pub const B43_DMA32_RXENABLE: c_uint = 0x00000001;
pub const B43_DMA32_RXFROFF_MASK: c_uint = 0x000000FE;
pub const B43_DMA32_RXFROFF_SHIFT: c_int = 1;
pub const B43_DMA32_RXDIRECTFIFO: c_uint = 0x00000100;
pub const B43_DMA32_RXPARITYDISABLE: c_uint = 0x00000800;
pub const B43_DMA32_RXADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA32_RXADDREXT_SHIFT: c_int = 16;
pub const B43_DMA32_RXRING: c_uint = 0x14;
pub const B43_DMA32_RXINDEX: c_uint = 0x18;
pub const B43_DMA32_RXSTATUS: c_uint = 0x1C;
pub const B43_DMA32_RXDPTR: c_uint = 0x00000FFF;
pub const B43_DMA32_RXSTATE: c_uint = 0x0000F000;
pub const B43_DMA32_RXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43_DMA32_RXSTAT_ACTIVE: c_uint = 0x00001000;
pub const B43_DMA32_RXSTAT_IDLEWAIT: c_uint = 0x00002000;
pub const B43_DMA32_RXSTAT_STOPPED: c_uint = 0x00003000;
pub const B43_DMA32_RXERROR: c_uint = 0x000F0000;
pub const B43_DMA32_RXERR_NOERR: c_uint = 0x00000000;
pub const B43_DMA32_RXERR_PROT: c_uint = 0x00010000;
pub const B43_DMA32_RXERR_OVERFLOW: c_uint = 0x00020000;
pub const B43_DMA32_RXERR_BUFWRITE: c_uint = 0x00030000;
pub const B43_DMA32_RXERR_DESCREAD: c_uint = 0x00040000;
pub const B43_DMA32_RXACTIVE: c_uint = 0xFFF00000;
// 32-bit DMA descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dmadesc32 {
    pub control: __le32,
    pub address: __le32,
    pub __packed: },
pub const B43_DMA32_DCTL_BYTECNT: c_uint = 0x00001FFF;
pub const B43_DMA32_DCTL_ADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA32_DCTL_ADDREXT_SHIFT: c_int = 16;
pub const B43_DMA32_DCTL_DTABLEEND: c_uint = 0x10000000;
pub const B43_DMA32_DCTL_IRQ: c_uint = 0x20000000;
pub const B43_DMA32_DCTL_FRAMEEND: c_uint = 0x40000000;
pub const B43_DMA32_DCTL_FRAMESTART: c_uint = 0x80000000;
// 64-bit DMA Engine.
// 64-bit DMA controller registers.
pub const B43_DMA64_TXCTL: c_uint = 0x00;
pub const B43_DMA64_TXENABLE: c_uint = 0x00000001;
pub const B43_DMA64_TXSUSPEND: c_uint = 0x00000002;
pub const B43_DMA64_TXLOOPBACK: c_uint = 0x00000004;
pub const B43_DMA64_TXFLUSH: c_uint = 0x00000010;
pub const B43_DMA64_TXPARITYDISABLE: c_uint = 0x00000800;
pub const B43_DMA64_TXADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA64_TXADDREXT_SHIFT: c_int = 16;
pub const B43_DMA64_TXINDEX: c_uint = 0x04;
pub const B43_DMA64_TXRINGLO: c_uint = 0x08;
pub const B43_DMA64_TXRINGHI: c_uint = 0x0C;
pub const B43_DMA64_TXSTATUS: c_uint = 0x10;
pub const B43_DMA64_TXSTATDPTR: c_uint = 0x00001FFF;
pub const B43_DMA64_TXSTAT: c_uint = 0xF0000000;
pub const B43_DMA64_TXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43_DMA64_TXSTAT_ACTIVE: c_uint = 0x10000000;
pub const B43_DMA64_TXSTAT_IDLEWAIT: c_uint = 0x20000000;
pub const B43_DMA64_TXSTAT_STOPPED: c_uint = 0x30000000;
pub const B43_DMA64_TXSTAT_SUSP: c_uint = 0x40000000;
pub const B43_DMA64_TXERROR: c_uint = 0x14;
pub const B43_DMA64_TXERRDPTR: c_uint = 0x0001FFFF;
pub const B43_DMA64_TXERR: c_uint = 0xF0000000;
pub const B43_DMA64_TXERR_NOERR: c_uint = 0x00000000;
pub const B43_DMA64_TXERR_PROT: c_uint = 0x10000000;
pub const B43_DMA64_TXERR_UNDERRUN: c_uint = 0x20000000;
pub const B43_DMA64_TXERR_TRANSFER: c_uint = 0x30000000;
pub const B43_DMA64_TXERR_DESCREAD: c_uint = 0x40000000;
pub const B43_DMA64_TXERR_CORE: c_uint = 0x50000000;
pub const B43_DMA64_RXCTL: c_uint = 0x20;
pub const B43_DMA64_RXENABLE: c_uint = 0x00000001;
pub const B43_DMA64_RXFROFF_MASK: c_uint = 0x000000FE;
pub const B43_DMA64_RXFROFF_SHIFT: c_int = 1;
pub const B43_DMA64_RXDIRECTFIFO: c_uint = 0x00000100;
pub const B43_DMA64_RXPARITYDISABLE: c_uint = 0x00000800;
pub const B43_DMA64_RXADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA64_RXADDREXT_SHIFT: c_int = 16;
pub const B43_DMA64_RXINDEX: c_uint = 0x24;
pub const B43_DMA64_RXRINGLO: c_uint = 0x28;
pub const B43_DMA64_RXRINGHI: c_uint = 0x2C;
pub const B43_DMA64_RXSTATUS: c_uint = 0x30;
pub const B43_DMA64_RXSTATDPTR: c_uint = 0x00001FFF;
pub const B43_DMA64_RXSTAT: c_uint = 0xF0000000;
pub const B43_DMA64_RXSTAT_DISABLED: c_uint = 0x00000000;
pub const B43_DMA64_RXSTAT_ACTIVE: c_uint = 0x10000000;
pub const B43_DMA64_RXSTAT_IDLEWAIT: c_uint = 0x20000000;
pub const B43_DMA64_RXSTAT_STOPPED: c_uint = 0x30000000;
pub const B43_DMA64_RXSTAT_SUSP: c_uint = 0x40000000;
pub const B43_DMA64_RXERROR: c_uint = 0x34;
pub const B43_DMA64_RXERRDPTR: c_uint = 0x0001FFFF;
pub const B43_DMA64_RXERR: c_uint = 0xF0000000;
pub const B43_DMA64_RXERR_NOERR: c_uint = 0x00000000;
pub const B43_DMA64_RXERR_PROT: c_uint = 0x10000000;
pub const B43_DMA64_RXERR_UNDERRUN: c_uint = 0x20000000;
pub const B43_DMA64_RXERR_TRANSFER: c_uint = 0x30000000;
pub const B43_DMA64_RXERR_DESCREAD: c_uint = 0x40000000;
pub const B43_DMA64_RXERR_CORE: c_uint = 0x50000000;
// 64-bit DMA descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dmadesc64 {
    pub control0: __le32,
    pub control1: __le32,
    pub address_low: __le32,
    pub address_high: __le32,
    pub __packed: },
pub const B43_DMA64_DCTL0_DTABLEEND: c_uint = 0x10000000;
pub const B43_DMA64_DCTL0_IRQ: c_uint = 0x20000000;
pub const B43_DMA64_DCTL0_FRAMEEND: c_uint = 0x40000000;
pub const B43_DMA64_DCTL0_FRAMESTART: c_uint = 0x80000000;
pub const B43_DMA64_DCTL1_BYTECNT: c_uint = 0x00001FFF;
pub const B43_DMA64_DCTL1_ADDREXT_MASK: c_uint = 0x00030000;
pub const B43_DMA64_DCTL1_ADDREXT_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dmadesc_generic {
    pub dma32: b43_dmadesc32,
    pub dma64: b43_dmadesc64,
    pub __packed: },
    pub __packed: },
// Misc DMA constants
pub const B43_DMA32_RINGMEMSIZE: c_int = 4096;
pub const B43_DMA64_RINGMEMSIZE: c_int = 8192;
// Offset of frame with actual data
pub const B43_DMA0_RX_FW598_FO: c_int = 38;
pub const B43_DMA0_RX_FW351_FO: c_int = 30;
// DMA engine tuning knobs
pub const B43_TXRING_SLOTS: c_int = 256;
pub const B43_RXRING_SLOTS: c_int = 256;

// Pointer poison

    pub sk_buff: struct,
    pub b43_private: struct,
    pub b43_txstatus: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dmadesc_meta {
// The kernel DMA-able buffer.
    pub skb: *mut sk_buff,
// DMA base bus-address of the descriptor buffer.
    pub dmaaddr: dma_addr_t,
// ieee80211 TX status. Only used once per 802.11 frag.
    pub is_last_fragment: bool,
}

// Lowlevel DMA operations that differ between 32bit and 64bit DMA.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dma_ops {
    pub irq): int end, int,
    pub slot): *mut *mut *mut void (poke_tx) (struct b43_dmaring  ring, int,
    pub ring): *mut *mut *mut void (tx_suspend) (struct b43_dmaring,
    pub ring): *mut *mut *mut void (tx_resume) (struct b43_dmaring,
    pub ring): *mut *mut *mut int (get_current_rxslot) (struct b43_dmaring,
    pub slot): *mut *mut *mut void (set_current_rxslot) (struct b43_dmaring  ring, int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_dmatype {
    B43_DMA_30BIT	= 30,
    B43_DMA_32BIT	= 32,
    B43_DMA_64BIT	= 64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_addrtype {
    B43_DMA_ADDR_LOW,
    B43_DMA_ADDR_HIGH,
    B43_DMA_ADDR_EXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dmaring {
// Lowlevel DMA ops.
    pub ops: *const b43_dma_ops,
// Kernel virtual base address of the ring memory.
    pub descbase: *mut c_void,
// Cache of TX headers for each TX frame.
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
    pub type: b43_dmatype,
// Boolean. Is this ring stopped at ieee80211 level?
    pub stopped: bool,
// The QOS priority assigned to this ring. Only used for TX rings.
// This is the mac80211 "queue" value.
    pub queue_prio: u8,
    pub dev: *mut b43_wldev,

// Maximum number of used slots.
    pub max_used_slots: c_int,
// Last time we injected a ring overflow.
    pub last_injected_overflow: c_ulong,
// Statistics: Number of successfully transmitted packets
    pub nr_succeed_tx_packets: u64,
// Statistics: Number of failed TX packets
    pub nr_failed_tx_packets: u64,
// Statistics: Total number of TX plus all retries.
    pub nr_total_packet_tries: u64,

// Meta data about all descriptors.
    pub __counted_by(nr_slots): b43_dmadesc_meta meta[],
}

extern "C" {
    pub fn b43_read32(_arg: ring->dev, offset: ring->mmio_base +) -> return;
}
extern "C" {
    pub fn b43_dma_init(dev: *mut b43_wldev) -> c_int;
}
extern "C" {
    pub fn b43_dma_free(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_dma_tx_suspend(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_dma_tx_resume(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_dma_handle_rx_overflow(ring: *mut b43_dmaring);
}
extern "C" {
    pub fn b43_dma_rx(ring: *mut b43_dmaring);
}
