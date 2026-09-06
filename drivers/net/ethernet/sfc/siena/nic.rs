//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/nic.h
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
//

extern "C" {
    pub fn efx_farch_fpga_ver(efx: *mut efx_nic) -> u32;
}
//
// struct siena_nic_data - Siena NIC state
// @efx: Pointer back to main interface structure
// @wol_filter_id: Wake-on-LAN packet filter id
// @stats: Hardware statistics
// @vf: Array of &struct siena_vf objects
// @vf_buftbl_base: The zeroth buffer table index used to back VF queues.
// @vfdi_status: Common VFDI status page to be dmad to VF address space.
// @local_addr_list: List of local addresses. Protected by %local_lock.
// @local_page_list: List of DMA addressable pages used to broadcast
// %local_addr_list. Protected by %local_lock.
// @local_lock: Mutex protecting %local_addr_list and %local_page_list.
// @peer_work: Work item to broadcast peer addresses to VMs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siena_nic_data {
    pub efx: *mut efx_nic,
    pub wol_filter_id: c_int,
    pub stats: [u64; SIENA_STAT_COUNT],
    pub vf: *mut siena_vf,
    pub vfdi_channel: *mut efx_channel,
    pub vf_buftbl_base: unsigned,
    pub vfdi_status: efx_buffer,
    pub local_addr_list: list_head,
    pub local_page_list: list_head,
    pub local_lock: mutex,
    pub peer_work: work_struct,

}

extern "C" {
    pub fn falcon_probe_board(efx: *mut efx_nic, revision_info: u16) -> c_int;
}
// Falcon/Siena queue operations
extern "C" {
    pub fn efx_farch_tx_probe(tx_queue: *mut efx_tx_queue) -> c_int;
}
extern "C" {
    pub fn efx_farch_tx_init(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_farch_tx_fini(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_farch_tx_remove(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_farch_tx_write(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_farch_rx_probe(rx_queue: *mut efx_rx_queue) -> c_int;
}
extern "C" {
    pub fn efx_farch_rx_init(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_farch_rx_fini(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_farch_rx_remove(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_farch_rx_write(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_farch_rx_defer_refill(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_farch_ev_probe(channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_farch_ev_init(channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_farch_ev_fini(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_farch_ev_remove(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_farch_ev_process(channel: *mut efx_channel, quota: c_int) -> c_int;
}
extern "C" {
    pub fn efx_farch_ev_read_ack(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_farch_ev_test_generate(channel: *mut efx_channel);
}
// Falcon/Siena filter operations
extern "C" {
    pub fn efx_farch_filter_table_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_farch_filter_table_restore(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_filter_table_remove(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_filter_update_rx_scatter(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_filter_get_rx_id_limit(efx: *mut efx_nic) -> u32;
}

extern "C" {
    pub fn efx_farch_filter_sync_rx_mode(efx: *mut efx_nic);
}
// Falcon/Siena interrupts
extern "C" {
    pub fn efx_farch_irq_enable_master(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_irq_test_generate(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_farch_irq_disable_master(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_msi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn efx_farch_legacy_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn efx_farch_fatal_interrupt(efx: *mut efx_nic) -> irqreturn_t;
}
// Global Resources
extern "C" {
    pub fn efx_siena_prepare_flush(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_fini_dmaq(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_farch_finish_flr(efx: *mut efx_nic);
}
extern "C" {
    pub fn siena_finish_flush(efx: *mut efx_nic);
}
extern "C" {
    pub fn falcon_start_nic_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn falcon_stop_nic_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn falcon_reset_xaui(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_farch_dimension_resources(efx: *mut efx_nic, sram_lim_qw: unsigned);
}
extern "C" {
    pub fn efx_farch_init_common(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_rx_push_indir_table(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_farch_rx_pull_indir_table(efx: *mut efx_nic);
}
// Tests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_farch_register_test {
    pub address: unsigned,
    pub mask: efx_oword_t,
}
