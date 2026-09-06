//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi.h
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
// Copyright 2008-2013 Solarflare Communications Inc.
//
// enum efx_mcdi_state - MCDI request handling state
// @MCDI_STATE_QUIESCENT: No pending MCDI requests. If the caller holds the
// mcdi @iface_lock then they are able to move to %MCDI_STATE_RUNNING
// @MCDI_STATE_RUNNING_SYNC: There is a synchronous MCDI request pending.
// Only the thread that moved into this state is allowed to move out of it.
// @MCDI_STATE_RUNNING_ASYNC: There is an asynchronous MCDI request pending.
// @MCDI_STATE_PROXY_WAIT: An MCDI request has completed with a response that
// indicates we must wait for a proxy try again message.
// @MCDI_STATE_COMPLETED: An MCDI request has completed, but the owning thread
// has not yet consumed the result. For all other threads, equivalent to
// %MCDI_STATE_RUNNING.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_mcdi_state {
    MCDI_STATE_QUIESCENT,
    MCDI_STATE_RUNNING_SYNC,
    MCDI_STATE_RUNNING_ASYNC,
    MCDI_STATE_PROXY_WAIT,
    MCDI_STATE_COMPLETED,
}

//
// enum efx_mcdi_mode - MCDI transaction mode
// @MCDI_MODE_POLL: poll for MCDI completion, until timeout
// @MCDI_MODE_EVENTS: wait for an mcdi_event.  On timeout, poll once
// @MCDI_MODE_FAIL: we think MCDI is dead, so fail-fast all calls
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_mcdi_mode {
    MCDI_MODE_POLL,
    MCDI_MODE_EVENTS,
    MCDI_MODE_FAIL,
}

//
// struct efx_mcdi_iface - MCDI protocol context
// @efx: The associated NIC.
// @state: Request handling state. Waited for by @wq.
// @mode: Poll for mcdi completion, or wait for an mcdi_event.
// @wq: Wait queue for threads waiting for @state != %MCDI_STATE_RUNNING
// @new_epoch: Indicates start of day or start of MC reboot recovery
// @iface_lock: Serialises access to @seqno, @credits and response metadata
// @seqno: The next sequence number to use for mcdi requests.
// @credits: Number of spurious MCDI completion events allowed before we
// trigger a fatal error
// @resprc: Response error/success code (Linux numbering)
// @resp_hdr_len: Response header length
// @resp_data_len: Response data (SDU or error) length
// @async_lock: Serialises access to @async_list while event processing is
// enabled
// @async_list: Queue of asynchronous requests
// @async_timer: Timer for asynchronous request timeout
// @logging_buffer: buffer that may be used to build MCDI tracing messages
// @logging_enabled: whether to trace MCDI
// @proxy_rx_handle: Most recently received proxy authorisation handle
// @proxy_rx_status: Status of most recent proxy authorisation
// @proxy_rx_wq: Wait queue for updates to proxy_rx_handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_iface {
    pub efx: *mut efx_nic,
    pub state: efx_mcdi_state,
    pub mode: efx_mcdi_mode,
    pub wq: wait_queue_head_t,
    pub iface_lock: spinlock_t,
    pub new_epoch: bool,
    pub credits: c_uint,
    pub seqno: c_uint,
    pub resprc: c_int,
    pub resprc_raw: c_int,
    pub resp_hdr_len: usize,
    pub resp_data_len: usize,
    pub async_lock: spinlock_t,
    pub async_list: list_head,
    pub async_timer: timer_list,

    pub logging_buffer: *mut c_char,
    pub logging_enabled: bool,

    pub proxy_rx_handle: c_uint,
    pub proxy_rx_status: c_int,
    pub proxy_rx_wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_mon {
    pub dma_buf: efx_buffer,
    pub update_lock: mutex,
    pub last_update: c_ulong,
    pub device: *mut device,
    pub attrs: *mut efx_mcdi_mon_attribute,
    pub group: attribute_group,
    pub groups: [*const attribute_group; 2],
    pub n_attrs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_mtd_partition {
    pub common: efx_mtd_partition,
    pub updating: bool,
    pub nvram_type: u16,
    pub fw_subtype: u16,
}

//
// struct efx_mcdi_data - extra state for NICs that implement MCDI
// @iface: Interface/protocol state
// @hwmon: Hardware monitor state
// @fn_flags: Flags for this function, as returned by %MC_CMD_DRV_ATTACH.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_data {
    pub iface: efx_mcdi_iface,

    pub hwmon: efx_mcdi_mon,

    pub fn_flags: u32,
}

extern "C" {
    pub fn efx_mcdi_init(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_detach(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_fini(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_poll_reboot(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mode_poll(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_mode_event(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_flush_async(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_process_event(channel: *mut efx_channel, event: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_mcdi_sensor_event(efx: *mut efx_nic, ev: *mut efx_qword_t);
}
// We expect that 16- and 32-bit fields in MCDI requests and responses
// are appropriately aligned, but 64-bit fields are only
// 32-bit-aligned.  Also, on Siena we must copy to the MC shared
// memory strictly 32 bits at a time, so add any necessary padding.
//

// Use MCDI_STRUCT_ functions to access members of MCDI structuredefs.
// _buf should point to the start of the structure, typically obtained with
// MCDI_DECLARE_STRUCT_PTR(structure) = _MCDI_DWORD(mcdi_buf, FIELD_WHICH_IS_STRUCT);
//

// (u8 *)MCDI_STRUCT_PTR(_buf, _field) = _value;			\

// MCDI_PTR(_buf, _field))

// MCDI_STRUCT_PTR(_buf, _field))

// ( __le16 *)MCDI_PTR(_buf, _field) = cpu_to_le16(_value);\

// ( __le16 *)MCDI_STRUCT_PTR(_buf, _field) = cpu_to_le16(_value);\

// Write a 16-bit field defined in the protocol as being big-endian.

// ( __be16 *)MCDI_PTR(_buf, _field) = (_value);		\

// ( __be16 *)MCDI_STRUCT_PTR(_buf, _field) = (_value);	\

// Write a 32-bit field defined in the protocol as being big-endian.

// ( __be32 *)MCDI_STRUCT_PTR(_buf, _field) = (_value);	\

// (efx_dword_t *)					\

extern "C" {
    pub fn efx_mcdi_print_fwver(efx: *mut efx_nic, buf: *mut c_char, len: usize);
}
extern "C" {
    pub fn efx_mcdi_log_ctrl(efx: *mut efx_nic, evq: bool, uart: bool, dest_evq: u32) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_nvram_types(efx: *mut efx_nic, nvram_types_out: *mut u32) -> c_int;
}
extern "C" {
    pub fn efx_new_mcdi_nvram_test_all(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_nvram_test_all(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_handle_assertion(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_set_id_led(efx: *mut efx_nic, mode: efx_led_mode) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_wol_filter_remove(efx: *mut efx_nic, id: c_int) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_wol_filter_reset(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_process_link_change(efx: *mut efx_nic, ev: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_mcdi_mac_start_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_mac_stop_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_mac_pull_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_map_reset_reason(reason: reset_type) -> reset_type;
}
extern "C" {
    pub fn efx_mcdi_reset(efx: *mut efx_nic, method: reset_type) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_get_privilege_mask(efx: *mut efx_nic, mask: *mut u32) -> c_int;
}

extern "C" {
    pub fn efx_mcdi_mon_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mon_remove(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_mcdi_nvram_update_start(efx: *mut efx_nic, type: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_update_finish_mode {
    EFX_UPDATE_FINISH_WAIT,
    EFX_UPDATE_FINISH_BACKGROUND,
    EFX_UPDATE_FINISH_POLL,
    EFX_UPDATE_FINISH_ABORT,
}

extern "C" {
    pub fn efx_mcdi_nvram_update_finish_polled(efx: *mut efx_nic, type: c_uint) -> c_int;
}

extern "C" {
    pub fn efx_mcdi_mtd_erase(mtd: *mut mtd_info, start: loff_t, len: usize) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mtd_sync(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mtd_rename(part: *mut efx_mtd_partition);
}

