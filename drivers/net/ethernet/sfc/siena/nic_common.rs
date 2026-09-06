//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/nic_common.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
// Copyright 2019-2020 Xilinx Inc.
//

// Revisions 0-2 were Falcon A0, A1 and B0 respectively.
// They are not supported by this driver but these revision numbers
// form part of the ethtool API for register dumping.
//
// Read the current event from the event queue
// See if an event is present
//
// We check both the high and low dword of the event for all ones.  We
// wrote all ones when we cleared the event, and no valid event can
// have all ones in either its high or low dwords.  This approach is
// robust against reordering.
//
// Note that using a single 64-bit comparison is incorrect; even
// though the CPU read will be atomic, the DMA write may not be.
//
// Returns a pointer to the specified transmit descriptor in the TX
// descriptor queue belonging to the specified channel.
//
// Report whether this TX queue would be empty for the given write_count.
// May return false negative.
//
// Decide whether to push a TX descriptor to the NIC vs merely writing
// the doorbell.  This can reduce latency when we are adding a single
// descriptor to an empty queue, but is otherwise pointless.  Further,
// Falcon and Siena have hardware bugs (SF bug 33851) that may be
// triggered if we don't check this.
// We use the write_count used for the last doorbell push, to get the
// NIC's view of the tx queue.
//
// Returns a pointer to the specified descriptor in the RX descriptor queue
// Alignment of PCIe DMA boundaries (4KB)
pub const EFX_PAGE_SIZE: c_int = 4096;
// Size and alignment of buffer table entries (same)

// NIC-generic software stats

// TX data path
// RX data path
// Event data path
extern "C" {
    pub fn efx_siena_event_test_start(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_siena_event_present(channel: *mut efx_channel) -> bool;
}
// Some statistics are computed as A - B where A and B each increase
// linearly with some hardware counter(s) and the counters are read
// asynchronously.  If the counters contributing to B are always read
// after those contributing to A, the computed value may be lower than
// the true value by some variable amount, and may decrease between
// subsequent computations.
//
// We should never allow statistics to decrease or to exceed the true
// value.  Since the computed value will never be greater than the
// true value, we can achieve this by only storing the computed value
// when it increases.
//
// stat = diff;
// Interrupts
extern "C" {
    pub fn efx_siena_init_interrupt(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_irq_test_start(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_fini_interrupt(efx: *mut efx_nic);
}
extern "C" {
    pub fn READ_ONCE(_arg: channel->event_test_cpu) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: efx->last_irq_cpu) -> return;
}
// Global Resources
extern "C" {
    pub fn efx_siena_free_buffer(efx: *mut efx_nic, buffer: *mut efx_buffer);
}
extern "C" {
    pub fn efx_siena_get_regs_len(efx: *mut efx_nic) -> usize;
}
extern "C" {
    pub fn efx_siena_get_regs(efx: *mut efx_nic, buf: *mut c_void);
}

extern "C" {
    pub fn efx_siena_fix_nodesc_drop_stat(efx: *mut efx_nic, stat: *mut u64);
}
pub const EFX_MAX_FLUSH_TIME: c_int = 5000;
