//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_fddi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for the ANSI FDDI interface.
//
// Version:	@(#)if_fddi.h	1.0.2	Sep 29 2004
//
// Author:	Lawrence V. Stefani, <stefani@lkg.dec.com>
//
// if_fddi.h is based on previous if_ether.h and if_tr.h work by
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
// Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
//

// Define FDDI statistics structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fddi_statistics {
// Generic statistics.
    pub gen: net_device_stats,
// Detailed FDDI statistics.  Adopted from RFC 1512
    pub smt_station_id: [__u8; 8],
    pub smt_op_version_id: __u32,
    pub smt_hi_version_id: __u32,
    pub smt_lo_version_id: __u32,
    pub smt_user_data: [__u8; 32],
    pub smt_mib_version_id: __u32,
    pub smt_mac_cts: __u32,
    pub smt_non_master_cts: __u32,
    pub smt_master_cts: __u32,
    pub smt_available_paths: __u32,
    pub smt_config_capabilities: __u32,
    pub smt_config_policy: __u32,
    pub smt_connection_policy: __u32,
    pub smt_t_notify: __u32,
    pub smt_stat_rpt_policy: __u32,
    pub smt_trace_max_expiration: __u32,
    pub smt_bypass_present: __u32,
    pub smt_ecm_state: __u32,
    pub smt_cf_state: __u32,
    pub smt_remote_disconnect_flag: __u32,
    pub smt_station_status: __u32,
    pub smt_peer_wrap_flag: __u32,
    pub smt_time_stamp: __u32,
    pub smt_transition_time_stamp: __u32,
    pub mac_frame_status_functions: __u32,
    pub mac_t_max_capability: __u32,
    pub mac_tvx_capability: __u32,
    pub mac_available_paths: __u32,
    pub mac_current_path: __u32,
    pub mac_upstream_nbr: [__u8; FDDI_K_ALEN],
    pub mac_downstream_nbr: [__u8; FDDI_K_ALEN],
    pub mac_old_upstream_nbr: [__u8; FDDI_K_ALEN],
    pub mac_old_downstream_nbr: [__u8; FDDI_K_ALEN],
    pub mac_dup_address_test: __u32,
    pub mac_requested_paths: __u32,
    pub mac_downstream_port_type: __u32,
    pub mac_smt_address: [__u8; FDDI_K_ALEN],
    pub mac_t_req: __u32,
    pub mac_t_neg: __u32,
    pub mac_t_max: __u32,
    pub mac_tvx_value: __u32,
    pub mac_frame_cts: __u32,
    pub mac_copied_cts: __u32,
    pub mac_transmit_cts: __u32,
    pub mac_error_cts: __u32,
    pub mac_lost_cts: __u32,
    pub mac_frame_error_threshold: __u32,
    pub mac_frame_error_ratio: __u32,
    pub mac_rmt_state: __u32,
    pub mac_da_flag: __u32,
    pub mac_una_da_flag: __u32,
    pub mac_frame_error_flag: __u32,
    pub mac_ma_unitdata_available: __u32,
    pub mac_hardware_present: __u32,
    pub mac_ma_unitdata_enable: __u32,
    pub path_tvx_lower_bound: __u32,
    pub path_t_max_lower_bound: __u32,
    pub path_max_t_req: __u32,
    pub path_configuration: [__u32; 8],
    pub port_my_type: [__u32; 2],
    pub port_neighbor_type: [__u32; 2],
    pub port_connection_policies: [__u32; 2],
    pub port_mac_indicated: [__u32; 2],
    pub port_current_path: [__u32; 2],
    pub port_requested_paths: [*mut __u8; 3*2],
    pub port_mac_placement: [__u32; 2],
    pub port_available_paths: [__u32; 2],
    pub port_pmd_class: [__u32; 2],
    pub port_connection_capabilities: [__u32; 2],
    pub port_bs_flag: [__u32; 2],
    pub port_lct_fail_cts: [__u32; 2],
    pub port_ler_estimate: [__u32; 2],
    pub port_lem_reject_cts: [__u32; 2],
    pub port_lem_cts: [__u32; 2],
    pub port_ler_cutoff: [__u32; 2],
    pub port_ler_alarm: [__u32; 2],
    pub port_connect_state: [__u32; 2],
    pub port_pcm_state: [__u32; 2],
    pub port_pc_withhold: [__u32; 2],
    pub port_ler_flag: [__u32; 2],
    pub port_hardware_present: [__u32; 2],
}
