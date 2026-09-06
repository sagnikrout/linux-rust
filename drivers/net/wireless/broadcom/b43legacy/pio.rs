//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/pio.h
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

// Macro flag: #define B43legacy_PIO_H_

pub const B43legacy_PIO_TXCTL: c_uint = 0x00;
pub const B43legacy_PIO_TXDATA: c_uint = 0x02;
pub const B43legacy_PIO_TXQBUFSIZE: c_uint = 0x04;
pub const B43legacy_PIO_RXCTL: c_uint = 0x08;
pub const B43legacy_PIO_RXDATA: c_uint = 0x0A;

// PIO constants
pub const B43legacy_PIO_MAXTXDEVQPACKETS: c_int = 31;
pub const B43legacy_PIO_TXQADJUST: c_int = 80;
// PIO tuning knobs
pub const B43legacy_PIO_MAXTXPACKETS: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_pio_txpacket {
    pub queue: *mut b43legacy_pioqueue,
    pub skb: *mut sk_buff,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_pioqueue {
    pub dev: *mut b43legacy_wldev,
    pub mmio_base: u16,
    pub tx_suspended: bool,
    pub tx_frozen: bool,
    pub /: *mut *mut bool need_workarounds; / Workarounds needed for core.rev < 3,
// Adjusted size of the device internal TX buffer.
    pub tx_devq_size: u16,
// Used octets of the device internal TX buffer.
    pub tx_devq_used: u16,
// Used packet slots in the device internal TX buffer.
    pub tx_devq_packets: u8,
// Packets from the txfree list can
// be taken on incoming TX requests.
//
    pub txfree: list_head,
    pub nr_txfree: c_uint,
// Packets on the txqueue are queued,
// but not completely written to the chip, yet.
//
    pub txqueue: list_head,
// Packets on the txrunning queue are completely
// posted to the device. We are waiting for the txstatus.
//
    pub txrunning: list_head,
    pub txtask: tasklet_struct,
}

extern "C" {
    pub fn b43legacy_read16(_arg: queue->dev, offset: queue->mmio_base +) -> return;
}
extern "C" {
    pub fn b43legacy_pio_init(dev: *mut b43legacy_wldev) -> c_int;
}
extern "C" {
    pub fn b43legacy_pio_free(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_pio_rx(queue: *mut b43legacy_pioqueue);
}
// Suspend TX queue in hardware.
extern "C" {
    pub fn b43legacy_pio_tx_suspend(queue: *mut b43legacy_pioqueue);
}
extern "C" {
    pub fn b43legacy_pio_tx_resume(queue: *mut b43legacy_pioqueue);
}
// Suspend (freeze) the TX tasklet (software level).
extern "C" {
    pub fn b43legacy_pio_freeze_txqueues(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_pio_thaw_txqueues(dev: *mut b43legacy_wldev);
}

