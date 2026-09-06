//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/falcon/nic.h
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
    pub fn ef4_farch_fpga_ver(efx: *mut ef4_nic) -> u32;
}
// NIC has two interlinked PCI functions for the same port.
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
// Get partner of a TX queue, seen as part of the same net core queue
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
pub const EF4_PAGE_SIZE: c_int = 4096;
// Size and alignment of buffer table entries (same)

// NIC-generic software stats
//
// struct falcon_board_type - board operations and type information
// @id: Board type id, as found in NVRAM
// @init: Allocate resources and initialise peripheral hardware
// @init_phy: Do board-specific PHY initialisation
// @fini: Shut down hardware and free resources
// @set_id_led: Set state of identifying LED or revert to automatic function
// @monitor: Board-specific health check function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_board_type {
    pub id: u8,
    pub nic): *mut *mut int (init) (struct ef4_nic,
    pub efx): *mut *mut void (init_phy) (struct ef4_nic,
    pub nic): *mut *mut void (fini) (struct ef4_nic,
    pub mode): *mut *mut *mut void (set_id_led) (struct ef4_nic efx, enum ef4_led_mode,
    pub nic): *mut *mut int (monitor) (struct ef4_nic,
}

//
// struct falcon_board - board information
// @type: Type of board
// @major: Major rev. ('A', 'B' ...)
// @minor: Minor rev. (0, 1, ...)
// @i2c_adap: I2C adapter for on-board peripherals
// @i2c_data: Data for bit-banging algorithm
// @hwmon_client: I2C client for hardware monitor
// @ioexp_client: I2C client for power/port control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_board {
    pub type: *const falcon_board_type,
    pub major: c_int,
    pub minor: c_int,
    pub i2c_adap: i2c_adapter,
    pub i2c_data: i2c_algo_bit_data,
    pub ioexp_client: *mut *mut i2c_client hwmon_client,,
}

//
// struct falcon_spi_device - a Falcon SPI (Serial Peripheral Interface) device
// @device_id:		Controller's id for the device
// @size:		Size (in bytes)
// @addr_len:		Number of address bytes in read/write commands
// @munge_address:	Flag whether addresses should be munged.
// Some devices with 9-bit addresses (e.g. AT25040A EEPROM)
// use bit 3 of the command byte as address bit A8, rather
// than having a two-byte address.  If this flag is set, then
// commands should be munged in this way.
// @erase_command:	Erase command (or 0 if sector erase not needed).
// @erase_size:		Erase sector size (in bytes)
// Erase commands affect sectors with this size and alignment.
// This must be a power of two.
// @block_size:		Write block size (in bytes).
// Write commands are limited to blocks with this size and alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_spi_device {
    pub device_id: c_int,
    pub size: c_uint,
    pub addr_len: c_uint,
    pub munge_address:1: c_uint,
    pub erase_command: u8,
    pub erase_size: c_uint,
    pub block_size: c_uint,
}

//
// struct falcon_nic_data - Falcon NIC state
// @pci_dev2: Secondary function of Falcon A
// @efx: ef4_nic pointer
// @board: Board state and functions
// @stats: Hardware statistics
// @stats_disable_count: Nest count for disabling statistics fetches
// @stats_pending: Is there a pending DMA of MAC statistics.
// @stats_timer: A timer for regularly fetching MAC statistics.
// @spi_flash: SPI flash device
// @spi_eeprom: SPI EEPROM device
// @spi_lock: SPI bus lock
// @mdio_lock: MDIO bus lock
// @xmac_poll_required: XMAC link state needs polling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_nic_data {
    pub pci_dev2: *mut pci_dev,
    pub efx: *mut ef4_nic,
    pub board: falcon_board,
    pub stats: [u64; FALCON_STAT_COUNT],
    pub stats_disable_count: c_uint,
    pub stats_pending: bool,
    pub stats_timer: timer_list,
    pub spi_flash: falcon_spi_device,
    pub spi_eeprom: falcon_spi_device,
    pub spi_lock: mutex,
    pub mdio_lock: mutex,
    pub xmac_poll_required: bool,
}

//
// Externs
//
extern "C" {
    pub fn falcon_probe_board(efx: *mut ef4_nic, revision_info: u16) -> c_int;
}
// TX data path
// RX data path
// Event data path
extern "C" {
    pub fn ef4_nic_event_test_start(channel: *mut ef4_channel);
}
// queue operations
extern "C" {
    pub fn ef4_farch_tx_probe(tx_queue: *mut ef4_tx_queue) -> c_int;
}
extern "C" {
    pub fn ef4_farch_tx_init(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_farch_tx_fini(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_farch_tx_remove(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_farch_tx_write(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_farch_rx_probe(rx_queue: *mut ef4_rx_queue) -> c_int;
}
extern "C" {
    pub fn ef4_farch_rx_init(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_farch_rx_fini(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_farch_rx_remove(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_farch_rx_write(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_farch_rx_defer_refill(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_farch_ev_probe(channel: *mut ef4_channel) -> c_int;
}
extern "C" {
    pub fn ef4_farch_ev_init(channel: *mut ef4_channel) -> c_int;
}
extern "C" {
    pub fn ef4_farch_ev_fini(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_farch_ev_remove(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_farch_ev_process(channel: *mut ef4_channel, quota: c_int) -> c_int;
}
extern "C" {
    pub fn ef4_farch_ev_read_ack(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_farch_ev_test_generate(channel: *mut ef4_channel);
}
// filter operations
extern "C" {
    pub fn ef4_farch_filter_table_probe(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_farch_filter_table_restore(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_filter_table_remove(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_filter_update_rx_scatter(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_filter_get_rx_id_limit(efx: *mut ef4_nic) -> u32;
}

extern "C" {
    pub fn ef4_farch_filter_sync_rx_mode(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_nic_event_present(channel: *mut ef4_channel) -> bool;
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
    pub fn ef4_nic_init_interrupt(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_nic_irq_test_start(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_nic_fini_interrupt(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_irq_enable_master(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_irq_test_generate(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_farch_irq_disable_master(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_msi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ef4_farch_legacy_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ef4_farch_fatal_interrupt(efx: *mut ef4_nic) -> irqreturn_t;
}
extern "C" {
    pub fn READ_ONCE(_arg: channel->event_test_cpu) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: efx->last_irq_cpu) -> return;
}
// Global Resources
extern "C" {
    pub fn ef4_nic_flush_queues(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_farch_fini_dmaq(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_farch_finish_flr(efx: *mut ef4_nic);
}
extern "C" {
    pub fn falcon_start_nic_stats(efx: *mut ef4_nic);
}
extern "C" {
    pub fn falcon_stop_nic_stats(efx: *mut ef4_nic);
}
extern "C" {
    pub fn falcon_reset_xaui(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_farch_init_common(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_farch_rx_push_indir_table(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_nic_free_buffer(efx: *mut ef4_nic, buffer: *mut ef4_buffer);
}
// Tests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef4_farch_register_test {
    pub address: unsigned,
    pub mask: ef4_oword_t,
}

extern "C" {
    pub fn ef4_nic_get_regs_len(efx: *mut ef4_nic) -> usize;
}
extern "C" {
    pub fn ef4_nic_get_regs(efx: *mut ef4_nic, buf: *mut c_void);
}
