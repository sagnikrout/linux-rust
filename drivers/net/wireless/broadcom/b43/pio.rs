//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/pio.h
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

// Registers for PIO queues up to revision 7.
// TX queue.
pub const B43_PIO_TXCTL: c_uint = 0x00;
pub const B43_PIO_TXCTL_WRITELO: c_uint = 0x0001;
pub const B43_PIO_TXCTL_WRITEHI: c_uint = 0x0002;
pub const B43_PIO_TXCTL_EOF: c_uint = 0x0004;
pub const B43_PIO_TXCTL_FREADY: c_uint = 0x0008;
pub const B43_PIO_TXCTL_FLUSHREQ: c_uint = 0x0020;
pub const B43_PIO_TXCTL_FLUSHPEND: c_uint = 0x0040;
pub const B43_PIO_TXCTL_SUSPREQ: c_uint = 0x0080;
pub const B43_PIO_TXCTL_QSUSP: c_uint = 0x0100;
pub const B43_PIO_TXCTL_COMMCNT: c_uint = 0xFC00;
pub const B43_PIO_TXCTL_COMMCNT_SHIFT: c_int = 10;
pub const B43_PIO_TXDATA: c_uint = 0x02;
pub const B43_PIO_TXQBUFSIZE: c_uint = 0x04;
// RX queue.
pub const B43_PIO_RXCTL: c_uint = 0x00;
pub const B43_PIO_RXCTL_FRAMERDY: c_uint = 0x0001;
pub const B43_PIO_RXCTL_DATARDY: c_uint = 0x0002;
pub const B43_PIO_RXDATA: c_uint = 0x02;
// Registers for PIO queues revision 8 and later.
// TX queue
pub const B43_PIO8_TXCTL: c_uint = 0x00;
pub const B43_PIO8_TXCTL_0_7: c_uint = 0x00000001;
pub const B43_PIO8_TXCTL_8_15: c_uint = 0x00000002;
pub const B43_PIO8_TXCTL_16_23: c_uint = 0x00000004;
pub const B43_PIO8_TXCTL_24_31: c_uint = 0x00000008;
pub const B43_PIO8_TXCTL_EOF: c_uint = 0x00000010;
pub const B43_PIO8_TXCTL_FREADY: c_uint = 0x00000080;
pub const B43_PIO8_TXCTL_SUSPREQ: c_uint = 0x00000100;
pub const B43_PIO8_TXCTL_QSUSP: c_uint = 0x00000200;
pub const B43_PIO8_TXCTL_FLUSHREQ: c_uint = 0x00000400;
pub const B43_PIO8_TXCTL_FLUSHPEND: c_uint = 0x00000800;
pub const B43_PIO8_TXDATA: c_uint = 0x04;
// RX queue
pub const B43_PIO8_RXCTL: c_uint = 0x00;
pub const B43_PIO8_RXCTL_FRAMERDY: c_uint = 0x00000001;
pub const B43_PIO8_RXCTL_DATARDY: c_uint = 0x00000002;
pub const B43_PIO8_RXDATA: c_uint = 0x04;
// The maximum number of TX-packets the HW can handle.
pub const B43_PIO_MAX_NR_TXPACKETS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_pio_txpacket {
// Pointer to the TX queue we belong to.
    pub queue: *mut b43_pio_txqueue,
// The TX data packet.
    pub skb: *mut sk_buff,
// Index in the (struct b43_pio_txqueue)->packets array.
    pub index: u8,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_pio_txqueue {
    pub dev: *mut b43_wldev,
    pub mmio_base: u16,
// The device queue buffer size in bytes.
    pub buffer_size: u16,
// The number of used bytes in the device queue buffer.
    pub buffer_used: u16,
// The number of packets that can still get queued.
// This is decremented on queueing a packet and incremented
// after receiving the transmit status.
    pub free_packet_slots: u16,
// True, if the mac80211 queue was stopped due to overflow at TX.
    pub stopped: bool,
// Our b43 queue index number
    pub index: u8,
// The mac80211 QoS queue priority.
    pub queue_prio: u8,
// Buffer for TX packet meta data.
    pub packets: [b43_pio_txpacket; B43_PIO_MAX_NR_TXPACKETS],
    pub packets_list: list_head,
// Shortcut to the 802.11 core revision. This is to
// avoid horrible pointer dereferencing in the fastpaths.
    pub rev: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_pio_rxqueue {
    pub dev: *mut b43_wldev,
    pub mmio_base: u16,
// Shortcut to the 802.11 core revision. This is to
// avoid horrible pointer dereferencing in the fastpaths.
    pub rev: u8,
}

extern "C" {
    pub fn b43_read16(_arg: q->dev, offset: q->mmio_base +) -> return;
}
extern "C" {
    pub fn b43_read32(_arg: q->dev, offset: q->mmio_base +) -> return;
}
extern "C" {
    pub fn b43_read16(_arg: q->dev, offset: q->mmio_base +) -> return;
}
extern "C" {
    pub fn b43_read32(_arg: q->dev, offset: q->mmio_base +) -> return;
}
extern "C" {
    pub fn b43_pio_init(dev: *mut b43_wldev) -> c_int;
}
extern "C" {
    pub fn b43_pio_free(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_pio_tx(dev: *mut b43_wldev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn b43_pio_rx(q: *mut b43_pio_rxqueue);
}
extern "C" {
    pub fn b43_pio_tx_suspend(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_pio_tx_resume(dev: *mut b43_wldev);
}
