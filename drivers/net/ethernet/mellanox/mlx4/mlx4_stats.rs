//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx4/mlx4_stats.h
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
pub const NUM_PRIORITIES: c_int = 9;
pub const NUM_PRIORITY_STATS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_pkt_stats {
    pub rx_multicast_packets: c_ulong,
    pub rx_broadcast_packets: c_ulong,
    pub rx_jabbers: c_ulong,
    pub rx_in_range_length_error: c_ulong,
    pub rx_out_range_length_error: c_ulong,
    pub tx_multicast_packets: c_ulong,
    pub tx_broadcast_packets: c_ulong,
    pub rx_prio: [c_ulong; NUM_PRIORITIES][NUM_PRIORITY_STATS],
    pub tx_prio: [c_ulong; NUM_PRIORITIES][NUM_PRIORITY_STATS],
pub const NUM_PKT_STATS: c_int = 43;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_counter_stats {
    pub rx_packets: c_ulong,
    pub rx_bytes: c_ulong,
    pub tx_packets: c_ulong,
    pub tx_bytes: c_ulong,
pub const NUM_PF_STATS: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_port_stats {
    pub tso_packets: c_ulong,
    pub xmit_more: c_ulong,
    pub queue_stopped: c_ulong,
    pub wake_queue: c_ulong,
    pub tx_timeout: c_ulong,
    pub rx_alloc_pages: c_ulong,
    pub rx_chksum_good: c_ulong,
    pub rx_chksum_none: c_ulong,
    pub rx_chksum_complete: c_ulong,
    pub tx_chksum_offload: c_ulong,
pub const NUM_PORT_STATS: c_int = 10;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_xdp_stats {
    pub rx_xdp_drop: c_ulong,
    pub rx_xdp_redirect: c_ulong,
    pub rx_xdp_redirect_fail: c_ulong,
    pub rx_xdp_tx: c_ulong,
    pub rx_xdp_tx_full: c_ulong,
pub const NUM_XDP_STATS: c_int = 5;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_phy_stats {
    pub rx_packets_phy: c_ulong,
    pub rx_bytes_phy: c_ulong,
    pub tx_packets_phy: c_ulong,
    pub tx_bytes_phy: c_ulong,
pub const NUM_PHY_STATS: c_int = 4;
}

pub const NUM_MAIN_STATS: c_int = 21;
pub const MLX4_NUM_PRIORITIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_flow_stats_rx {
    pub rx_pause: u64,
    pub rx_pause_duration: u64,
    pub rx_pause_transition: u64,
pub const NUM_FLOW_STATS_RX: c_int = 3;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_flow_stats_tx {
    pub tx_pause: u64,
    pub tx_pause_duration: u64,
    pub tx_pause_transition: u64,
pub const NUM_FLOW_STATS_TX: c_int = 3;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_stat_out_flow_control_mbox {
// Total number of PAUSE frames received from the far-end port
    pub rx_pause: __be64,
// Total number of microseconds that far-end port requested to pause
// transmission of packets
//
    pub rx_pause_duration: __be64,
// Number of received transmission from XOFF state to XON state
    pub rx_pause_transition: __be64,
// Total number of PAUSE frames sent from the far-end port
    pub tx_pause: __be64,
// Total time in microseconds that transmission of packets has been
// paused
//
    pub tx_pause_duration: __be64,
// Number of transmitter transitions from XOFF state to XON state
    pub tx_pause_transition: __be64,
// Reserved
    pub reserved: [__be64; 2],
}

