//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/gsi.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//

// Maximum number of channels and event rings supported by the driver
pub const GSI_CHANNEL_COUNT_MAX: c_int = 28;
pub const GSI_EVT_RING_COUNT_MAX: c_int = 28;
// Maximum TLV FIFO size for a channel; 64 here is arbitrary (and high)
pub const GSI_TLV_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_ring {
    pub /: *mut *mut *mut void virt; / ring array base address,
    pub /: *mut *mut dma_addr_t addr; / primarily low 32 bits used,
    pub /: *mut *mut u32 count; / number of elements in ring,
// The ring index value indicates the next "open" entry in the ring.
//
// A channel ring consists of TRE entries filled by the AP and passed
// to the hardware for processing.  For a channel ring, the ring index
// identifies the next unused entry to be filled by the AP.  In this
// case the initial value is assumed by hardware to be 0.
//
// An event ring consists of event structures filled by the hardware
// and passed to the AP.  For event rings, the ring index identifies
// the next ring entry that is not known to have been filled by the
// hardware.  The initial value used is arbitrary (so we use 0).
//
    pub index: u32,
}

// Transactions use several resources that can be allocated dynamically
// but taken from a fixed-size pool.  The number of elements required for
// the pool is limited by the total number of TREs that can be outstanding.
//
// If sufficient TREs are available to reserve for a transaction,
// allocation from these pools is guaranteed to succeed.  Furthermore,
// these resources are implicitly freed whenever the TREs in the
// transaction they're associated with are released.
//
// The result of a pool allocation of multiple elements is always
// contiguous.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_trans_pool {
    pub /: *mut *mut *mut void base; / base address of element pool,
    pub /: *mut *mut u32 count; / # elements in the pool,
    pub /: *mut *mut u32 free; / next free element in pool (modulo),
    pub /: *mut *mut u32 size; / size (bytes) of an element,
    pub /: *mut *mut u32 max_alloc; / max allocation request,
    pub /: *mut *mut dma_addr_t addr; / DMA address if DMA pool (or 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_trans_info {
    pub /: *mut *mut atomic_t tre_avail; / TREs available for allocation,
    pub /: *mut *mut u16 free_id; / first free trans in array,
    pub /: *mut *mut u16 allocated_id; / first allocated transaction,
    pub /: *mut *mut u16 committed_id; / first committed transaction,
    pub /: *mut *mut u16 pending_id; / first pending transaction,
    pub /: *mut *mut u16 completed_id; / first completed transaction,
    pub /: *mut *mut u16 polled_id; / first polled transaction,
    pub /: *mut *mut *mut gsi_trans trans; / transaction array,
    pub /: *mut *mut *mut *mut gsi_trans map; / TRE -> transaction map,
    pub /: *mut *mut gsi_trans_pool sg_pool; / scatterlist pool,
    pub /: *mut *mut gsi_trans_pool cmd_pool; / command payload DMA pool,
}

// Hardware values signifying the state of a channel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsi_channel_state {
    GSI_CHANNEL_STATE_NOT_ALLOCATED		= 0x0,
    GSI_CHANNEL_STATE_ALLOCATED		= 0x1,
    GSI_CHANNEL_STATE_STARTED		= 0x2,
    GSI_CHANNEL_STATE_STOPPED		= 0x3,
    GSI_CHANNEL_STATE_STOP_IN_PROC		= 0x4,
    GSI_CHANNEL_STATE_FLOW_CONTROLLED	= 0x5,	/* IPA v4.2-v4.9 */
    GSI_CHANNEL_STATE_ERROR			= 0xf,
}

// We only care about channels between IPA and AP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_channel {
    pub gsi: *mut gsi,
    pub toward_ipa: bool,
    pub /: *mut *mut bool command; / AP command TX channel or not,
    pub /: *mut *mut u8 trans_tre_max; / max TREs in a transaction,
    pub tre_count: u16,
    pub event_count: u16,
    pub tre_ring: gsi_ring,
    pub evt_ring_id: u32,
// The following counts are used only for TX endpoints
    pub /: *mut *mut u64 byte_count; / total # bytes transferred,
    pub /: *mut *mut u64 trans_count; / total # transactions,
    pub /: *mut *mut u64 queued_byte_count; / last reported queued byte count,
    pub /: *mut *mut u64 queued_trans_count; / ...and queued trans count,
    pub /: *mut *mut u64 compl_byte_count; / last reported completed byte count,
    pub /: *mut *mut u64 compl_trans_count; / ...and completed trans count,
    pub trans_info: gsi_trans_info,
    pub napi: napi_struct,
}

// Hardware values signifying the state of an event ring
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsi_evt_ring_state {
    GSI_EVT_RING_STATE_NOT_ALLOCATED	= 0x0,
    GSI_EVT_RING_STATE_ALLOCATED		= 0x1,
    GSI_EVT_RING_STATE_ERROR		= 0xf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_evt_ring {
    pub channel: *mut gsi_channel,
    pub ring: gsi_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi {
    pub /: *mut *mut *mut device dev; / Same as IPA device,
    pub version: ipa_version,
    pub /: *mut *mut *mut void __iomem virt; / I/O mapped registers,
    pub regs: *const regs,
    pub irq: u32,
    pub channel_count: u32,
    pub evt_ring_count: u32,
    pub /: *mut *mut u32 event_bitmap; / allocated event rings,
    pub /: *mut *mut u32 modem_channel_bitmap; / modem channels to allocate,
    pub /: *mut *mut u32 type_enabled_bitmap; / GSI IRQ types enabled,
    pub /: *mut *mut u32 ieob_enabled_bitmap; / IEOB IRQ enabled (event rings),
    pub /: *mut *mut int result; / Negative errno (generic commands),
    pub /: *mut *mut completion completion; / Signals GSI command completion,
    pub /: *mut *mut mutex mutex; / protects commands, programming,
    pub channel: [gsi_channel; GSI_CHANNEL_COUNT_MAX],
    pub evt_ring: [gsi_evt_ring; GSI_EVT_RING_COUNT_MAX],
    pub /: *mut *mut *mut net_device dummy_dev; / needed for NAPI,
}

//
// gsi_setup() - Set up the GSI subsystem
// @gsi:	Address of GSI structure embedded in an IPA structure
//
// Return:	0 if successful, or a negative error code
//
// Performs initialization that must wait until the GSI hardware is
// ready (including firmware loaded).
//
extern "C" {
    pub fn gsi_setup(gsi: *mut gsi) -> c_int;
}
//
// gsi_teardown() - Tear down GSI subsystem
// @gsi:	GSI address previously passed to a successful gsi_setup() call
//
extern "C" {
    pub fn gsi_teardown(gsi: *mut gsi);
}
//
// gsi_channel_tre_max() - Channel maximum number of in-flight TREs
// @gsi:	GSI pointer
// @channel_id:	Channel whose limit is to be returned
//
// Return:	 The maximum number of TREs outstanding on the channel
//
extern "C" {
    pub fn gsi_channel_tre_max(gsi: *mut gsi, channel_id: u32) -> u32;
}
//
// gsi_channel_start() - Start an allocated GSI channel
// @gsi:	GSI pointer
// @channel_id:	Channel to start
//
// Return:	0 if successful, or a negative error code
//
extern "C" {
    pub fn gsi_channel_start(gsi: *mut gsi, channel_id: u32) -> c_int;
}
//
// gsi_channel_stop() - Stop a started GSI channel
// @gsi:	GSI pointer returned by gsi_setup()
// @channel_id:	Channel to stop
//
// Return:	0 if successful, or a negative error code
//
extern "C" {
    pub fn gsi_channel_stop(gsi: *mut gsi, channel_id: u32) -> c_int;
}
//
// gsi_modem_channel_flow_control() - Set channel flow control state (IPA v4.2+)
// @gsi:	GSI pointer returned by gsi_setup()
// @channel_id:	Modem TX channel to control
// @enable:	Whether to enable flow control (i.e., prevent flow)
//
// gsi_channel_reset() - Reset an allocated GSI channel
// @gsi:	GSI pointer
// @channel_id:	Channel to be reset
// @doorbell:	Whether to (possibly) enable the doorbell engine
//
// Reset a channel and reconfigure it.  The @doorbell flag indicates
// that the doorbell engine should be enabled if needed.
//
// GSI hardware relinquishes ownership of all pending receive buffer
// transactions and they will complete with their cancelled flag set.
//
extern "C" {
    pub fn gsi_channel_reset(gsi: *mut gsi, channel_id: u32, doorbell: bool);
}
//
// gsi_suspend() - Prepare the GSI subsystem for suspend
// @gsi:	GSI pointer
//
extern "C" {
    pub fn gsi_suspend(gsi: *mut gsi);
}
//
// gsi_resume() - Resume the GSI subsystem following suspend
// @gsi:	GSI pointer
//
extern "C" {
    pub fn gsi_resume(gsi: *mut gsi);
}
//
// gsi_channel_suspend() - Suspend a GSI channel
// @gsi:	GSI pointer
// @channel_id:	Channel to suspend
//
// For IPA v4.0+, suspend is implemented by stopping the channel.
//
extern "C" {
    pub fn gsi_channel_suspend(gsi: *mut gsi, channel_id: u32) -> c_int;
}
//
// gsi_channel_resume() - Resume a suspended GSI channel
// @gsi:	GSI pointer
// @channel_id:	Channel to resume
//
// For IPA v4.0+, the stopped channel is started again.
//
extern "C" {
    pub fn gsi_channel_resume(gsi: *mut gsi, channel_id: u32) -> c_int;
}
//
// gsi_init() - Initialize the GSI subsystem
// @gsi:	Address of GSI structure embedded in an IPA structure
// @pdev:	IPA platform device
// @version:	IPA hardware version (implies GSI version)
// @count:	Number of entries in the configuration data array
// @data:	Endpoint and channel configuration data
//
// Return:	0 if successful, or a negative error code
//
// Early stage initialization of the GSI subsystem, performing tasks
// that can be done before the GSI hardware is ready to use.
//
// gsi_exit() - Exit the GSI subsystem
// @gsi:	GSI address previously passed to a successful gsi_init() call
//
extern "C" {
    pub fn gsi_exit(gsi: *mut gsi);
}
