//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/mmc.h
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
// MMC control register
// When set, all counter are reset
pub const MMC_CNTRL_COUNTER_RESET: c_uint = 0x1;
// When set, do not roll over zero after reaching the max value
pub const MMC_CNTRL_COUNTER_STOP_ROLLOVER: c_uint = 0x2;
pub const MMC_CNTRL_RESET_ON_READ: c_uint = 0x4	/* Reset after reading */;
pub const MMC_CNTRL_COUNTER_FREEZER: c_uint = 0x8	/* Freeze counter values to the;
// current value.
pub const MMC_CNTRL_PRESET: c_uint = 0x10;
pub const MMC_CNTRL_FULL_HALF_PRESET: c_uint = 0x20;
pub const MMC_GMAC4_OFFSET: c_uint = 0x700;
pub const MMC_GMAC3_X_OFFSET: c_uint = 0x100;
pub const MMC_XGMAC_OFFSET: c_uint = 0x800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_counters {
    pub mmc_tx_octetcount_gb: c_uint,
    pub mmc_tx_framecount_gb: c_uint,
    pub mmc_tx_broadcastframe_g: c_uint,
    pub mmc_tx_multicastframe_g: c_uint,
    pub mmc_tx_64_octets_gb: c_uint,
    pub mmc_tx_65_to_127_octets_gb: c_uint,
    pub mmc_tx_128_to_255_octets_gb: c_uint,
    pub mmc_tx_256_to_511_octets_gb: c_uint,
    pub mmc_tx_512_to_1023_octets_gb: c_uint,
    pub mmc_tx_1024_to_max_octets_gb: c_uint,
    pub mmc_tx_unicast_gb: c_uint,
    pub mmc_tx_multicast_gb: c_uint,
    pub mmc_tx_broadcast_gb: c_uint,
    pub mmc_tx_underflow_error: c_uint,
    pub mmc_tx_singlecol_g: c_uint,
    pub mmc_tx_multicol_g: c_uint,
    pub mmc_tx_deferred: c_uint,
    pub mmc_tx_latecol: c_uint,
    pub mmc_tx_exesscol: c_uint,
    pub mmc_tx_carrier_error: c_uint,
    pub mmc_tx_octetcount_g: c_uint,
    pub mmc_tx_framecount_g: c_uint,
    pub mmc_tx_excessdef: c_uint,
    pub mmc_tx_pause_frame: c_uint,
    pub mmc_tx_vlan_frame_g: c_uint,
    pub mmc_tx_oversize_g: c_uint,
    pub mmc_tx_lpi_usec: c_uint,
    pub mmc_tx_lpi_tran: c_uint,
// MMC RX counter registers
    pub mmc_rx_framecount_gb: c_uint,
    pub mmc_rx_octetcount_gb: c_uint,
    pub mmc_rx_octetcount_g: c_uint,
    pub mmc_rx_broadcastframe_g: c_uint,
    pub mmc_rx_multicastframe_g: c_uint,
    pub mmc_rx_crc_error: c_uint,
    pub mmc_rx_align_error: c_uint,
    pub mmc_rx_run_error: c_uint,
    pub mmc_rx_jabber_error: c_uint,
    pub mmc_rx_undersize_g: c_uint,
    pub mmc_rx_oversize_g: c_uint,
    pub mmc_rx_64_octets_gb: c_uint,
    pub mmc_rx_65_to_127_octets_gb: c_uint,
    pub mmc_rx_128_to_255_octets_gb: c_uint,
    pub mmc_rx_256_to_511_octets_gb: c_uint,
    pub mmc_rx_512_to_1023_octets_gb: c_uint,
    pub mmc_rx_1024_to_max_octets_gb: c_uint,
    pub mmc_rx_unicast_g: c_uint,
    pub mmc_rx_length_error: c_uint,
    pub mmc_rx_autofrangetype: c_uint,
    pub mmc_rx_pause_frames: c_uint,
    pub mmc_rx_fifo_overflow: c_uint,
    pub mmc_rx_vlan_frames_gb: c_uint,
    pub mmc_rx_watchdog_error: c_uint,
    pub mmc_rx_error: c_uint,
    pub mmc_rx_lpi_usec: c_uint,
    pub mmc_rx_lpi_tran: c_uint,
    pub mmc_rx_discard_frames_gb: c_uint,
    pub mmc_rx_discard_octets_gb: c_uint,
    pub mmc_rx_align_err_frames: c_uint,
// IPv4
    pub mmc_rx_ipv4_gd: c_uint,
    pub mmc_rx_ipv4_hderr: c_uint,
    pub mmc_rx_ipv4_nopay: c_uint,
    pub mmc_rx_ipv4_frag: c_uint,
    pub mmc_rx_ipv4_udsbl: c_uint,
    pub mmc_rx_ipv4_gd_octets: c_uint,
    pub mmc_rx_ipv4_hderr_octets: c_uint,
    pub mmc_rx_ipv4_nopay_octets: c_uint,
    pub mmc_rx_ipv4_frag_octets: c_uint,
    pub mmc_rx_ipv4_udsbl_octets: c_uint,
// IPV6
    pub mmc_rx_ipv6_gd_octets: c_uint,
    pub mmc_rx_ipv6_hderr_octets: c_uint,
    pub mmc_rx_ipv6_nopay_octets: c_uint,
    pub mmc_rx_ipv6_gd: c_uint,
    pub mmc_rx_ipv6_hderr: c_uint,
    pub mmc_rx_ipv6_nopay: c_uint,
// Protocols
    pub mmc_rx_udp_gd: c_uint,
    pub mmc_rx_udp_err: c_uint,
    pub mmc_rx_tcp_gd: c_uint,
    pub mmc_rx_tcp_err: c_uint,
    pub mmc_rx_icmp_gd: c_uint,
    pub mmc_rx_icmp_err: c_uint,
    pub mmc_rx_udp_gd_octets: c_uint,
    pub mmc_rx_udp_err_octets: c_uint,
    pub mmc_rx_tcp_gd_octets: c_uint,
    pub mmc_rx_tcp_err_octets: c_uint,
    pub mmc_rx_icmp_gd_octets: c_uint,
    pub mmc_rx_icmp_err_octets: c_uint,
// Stream-Gate Filter
    pub mmc_sgf_pass_fragment_cntr: c_uint,
    pub mmc_sgf_fail_fragment_cntr: c_uint,
// FPE
    pub mmc_tx_fpe_fragment_cntr: c_uint,
    pub mmc_tx_hold_req_cntr: c_uint,
    pub mmc_tx_gate_overrun_cntr: c_uint,
    pub mmc_rx_packet_assembly_err_cntr: c_uint,
    pub mmc_rx_packet_smd_err_cntr: c_uint,
    pub mmc_rx_packet_assembly_ok_cntr: c_uint,
    pub mmc_rx_fpe_fragment_cntr: c_uint,
}
