//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl2/hw_atl2_utils.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

// Hardware tx launch time descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_txts_s {
    pub ts: __le64,
    pub ctrl: __le32,
    pub reserved: u32,
}

// F W    A P I
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_options_s {
    pub link_up:1: u8,
    pub link_renegotiate:1: u8,
    pub minimal_link_speed:1: u8,
    pub internal_loopback:1: u8,
    pub external_loopback:1: u8,
    pub rate_10M_hd:1: u8,
    pub rate_100M_hd:1: u8,
    pub rate_1G_hd:1: u8,
    pub rate_10M:1: u8,
    pub rate_100M:1: u8,
    pub rate_1G:1: u8,
    pub rate_2P5G:1: u8,
    pub rate_N2P5G:1: u8,
    pub rate_5G:1: u8,
    pub rate_N5G:1: u8,
    pub rate_10G:1: u8,
    pub eee_100M:1: u8,
    pub eee_1G:1: u8,
    pub eee_2P5G:1: u8,
    pub eee_5G:1: u8,
    pub eee_10G:1: u8,
    pub rsvd3:3: u8,
    pub pause_rx:1: u8,
    pub pause_tx:1: u8,
    pub rsvd4:1: u8,
    pub downshift:1: u8,
    pub downshift_retry:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_control_s {
    pub mode:4: u8,
    pub disable_crc_corruption:1: u8,
    pub discard_short_frames:1: u8,
    pub flow_control_mode:1: u8,
    pub disable_length_check:1: u8,
    pub discard_errored_frames:1: u8,
    pub control_frame_enable:1: u8,
    pub enable_tx_padding:1: u8,
    pub enable_crc_forwarding:1: u8,
    pub 1: u8 enable_frame_padding_removal_rx:,
    pub 1: u8 promiscuous_mode:,
    pub rsvd:2: u8,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_shutdown_s {
    pub enable:1: u8,
    pub warning_enable:1: u8,
    pub rsvd:6: u8,
    pub shutdown_temperature: u8,
    pub cold_temperature: u8,
    pub warning_temperature: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_address_s {
    pub mac_address: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_address_aligned_s {
    pub aligned: mac_address_s,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_proxy_s {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_on_lan_s {
    pub wake_on_magic_packet:1: u8,
    pub wake_on_pattern:1: u8,
    pub wake_on_link_up:1: u8,
    pub wake_on_link_down:1: u8,
    pub wake_on_ping:1: u8,
    pub wake_on_timer:1: u8,
    pub rsvd:2: u8,
    pub rsvd2: u8,
    pub rsvd3: u16,
    pub link_up_timeout: u32,
    pub link_down_timeout: u32,
    pub timer: u32,
    pub wake_on_lan: },
    pub mask: [u32; 4],
    pub crc32: u32,
    pub wake_up_pattern: [}; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub arp_responder:1: u8,
    pub echo_responder:1: u8,
    pub igmp_client:1: u8,
    pub echo_truncate:1: u8,
    pub address_guard:1: u8,
    pub ignore_fragmented:1: u8,
    pub rsvd:2: u8,
    pub echo_max_len: u16,
    pub rsvd2: u8,
    pub ipv4_offload: },
    pub ipv4_offload_addr: [u32; 8],
    pub reserved: [u32; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub ns_responder:1: u8,
    pub echo_responder:1: u8,
    pub mld_client:1: u8,
    pub echo_truncate:1: u8,
    pub address_guard:1: u8,
    pub rsvd:3: u8,
    pub echo_max_len: u16,
    pub rsvd2: u8,
    pub ipv6_offload: },
    pub ipv6_offload_addr: [u32; 16][4],
    pub port: [u16; 16],
    pub tcp_port_offload: },
    pub port: [u16; 16],
    pub udp_port_offload: },
    pub retry_count: u32,
    pub retry_interval: u32,
    pub ka4_offload: },
    pub timeout: u32,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_mac_addr: [u8; 6],
    pub rsvd: u16,
    pub rsvd2: u32,
    pub rsvd3: u32,
    pub rsvd4: u16,
    pub win_size: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub local_ip: u32,
    pub remote_ip: u32,
    pub ka4_connection: [}; 16],
    pub retry_count: u32,
    pub retry_interval: u32,
    pub ka6_offload: },
    pub timeout: u32,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_mac_addr: [u8; 6],
    pub rsvd: u16,
    pub rsvd2: u32,
    pub rsvd3: u32,
    pub rsvd4: u16,
    pub win_size: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub local_ip: [u32; 4],
    pub remote_ip: [u32; 4],
    pub ka6_connection: [}; 16],
    pub rr_count: u32,
    pub rr_buf_len: u32,
    pub idx_offset: u32,
    pub rr__offset: u32,
    pub mdns_offload: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pause_quanta_s {
    pub quanta_10M: u16,
    pub threshold_10M: u16,
    pub quanta_100M: u16,
    pub threshold_100M: u16,
    pub quanta_1G: u16,
    pub threshold_1G: u16,
    pub quanta_2P5G: u16,
    pub threshold_2P5G: u16,
    pub quanta_5G: u16,
    pub threshold_5G: u16,
    pub quanta_10G: u16,
    pub threshold_10G: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_buffer_status_s {
    pub data_offset: u32,
    pub data_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_caps_s {
    pub finite_flashless:1: u8,
    pub cable_diag:1: u8,
    pub ncsi:1: u8,
    pub avb:1: u8,
    pub rsvd:4: u8,
    pub rsvd2: u8,
    pub rsvd3: u16,
    pub rsvd4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_s {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bundle_version_t {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub bundle: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_version_t {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub mac: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_version_t {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub phy: },
    pub drv_iface_ver:4: u32,
    pub rsvd:28: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_status_s {
    pub link_state:4: u8,
    pub link_rate:4: u8,
    pub pause_tx:1: u8,
    pub pause_rx:1: u8,
    pub eee:1: u8,
    pub duplex:1: u8,
    pub rsvd:4: u8,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wol_status_s {
    pub wake_count: u8,
    pub wake_reason: u8,
    pub :12: u16 wake_up_packet_length,
    pub :3: u16 wake_up_pattern_number,
    pub rsvd:1: u16,
    pub wake_up_packet: [u32; 379],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_health_monitor_s {
    pub mac_ready:1: u8,
    pub mac_fault:1: u8,
    pub mac_flashless_finished:1: u8,
    pub rsvd:5: u8,
    pub mac_temperature: u8,
    pub mac_heart_beat: u16,
    pub mac_fault_code: u16,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_health_monitor_s {
    pub phy_ready:1: u8,
    pub phy_fault:1: u8,
    pub phy_hot_warning:1: u8,
    pub rsvd:5: u8,
    pub phy_temperature: u8,
    pub phy_heart_beat: u16,
    pub phy_fault_code: u16,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_link_caps_s {
    pub rsvd:3: u8,
    pub internal_loopback:1: u8,
    pub external_loopback:1: u8,
    pub rate_10M_hd:1: u8,
    pub rate_100M_hd:1: u8,
    pub rate_1G_hd:1: u8,
    pub rate_10M:1: u8,
    pub rate_100M:1: u8,
    pub rate_1G:1: u8,
    pub rate_2P5G:1: u8,
    pub rate_N2P5G:1: u8,
    pub rate_5G:1: u8,
    pub rate_N5G:1: u8,
    pub rate_10G:1: u8,
    pub rsvd3:1: u8,
    pub eee_100M:1: u8,
    pub eee_1G:1: u8,
    pub eee_2P5G:1: u8,
    pub rsvd4:1: u8,
    pub eee_5G:1: u8,
    pub rsvd5:1: u8,
    pub eee_10G:1: u8,
    pub pause_rx:1: u8,
    pub pause_tx:1: u8,
    pub pfc:1: u8,
    pub downshift:1: u8,
    pub downshift_retry:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_proxy_caps_s {
    pub ipv4_offload:1: u8,
    pub ipv6_offload:1: u8,
    pub tcp_port_offload:1: u8,
    pub udp_port_offload:1: u8,
    pub ka4_offload:1: u8,
    pub ka6_offload:1: u8,
    pub mdns_offload:1: u8,
    pub wake_on_ping:1: u8,
    pub wake_on_magic_packet:1: u8,
    pub wake_on_pattern:1: u8,
    pub wake_on_timer:1: u8,
    pub wake_on_link:1: u8,
    pub wake_patterns_count:4: u8,
    pub ipv4_count: u8,
    pub ipv6_count: u8,
    pub tcp_port_offload_count: u8,
    pub udp_port_offload_count: u8,
    pub tcp4_ka_count: u8,
    pub tcp6_ka_count: u8,
    pub igmp_offload:1: u8,
    pub mld_offload:1: u8,
    pub rsvd:6: u8,
    pub rsvd2: u8,
    pub rsvd3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lkp_link_caps_s {
    pub rsvd:5: u8,
    pub rate_10M_hd:1: u8,
    pub rate_100M_hd:1: u8,
    pub rate_1G_hd:1: u8,
    pub rate_10M:1: u8,
    pub rate_100M:1: u8,
    pub rate_1G:1: u8,
    pub rate_2P5G:1: u8,
    pub rate_N2P5G:1: u8,
    pub rate_5G:1: u8,
    pub rate_N5G:1: u8,
    pub rate_10G:1: u8,
    pub rsvd2:1: u8,
    pub eee_100M:1: u8,
    pub eee_1G:1: u8,
    pub eee_2P5G:1: u8,
    pub rsvd3:1: u8,
    pub eee_5G:1: u8,
    pub rsvd4:1: u8,
    pub eee_10G:1: u8,
    pub pause_rx:1: u8,
    pub pause_tx:1: u8,
    pub rsvd5:6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_dump_s {
    pub reg0: u32,
    pub reg1: u32,
    pub reg2: u32,
    pub hi: u32,
    pub lo: u32,
    pub regs: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_s {
    pub sync_counter: u32,
    pub mem_buffer: [u32; 0x1ff],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cable_diag_control_s {
    pub :1: u8 toggle,
    pub rsvd:7: u8,
    pub wait_timeout_sec: u8,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cable_diag_lane_data_s {
    pub result_code: u8,
    pub dist: u8,
    pub far_dist: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cable_diag_status_s {
    pub lane_data: [cable_diag_lane_data_s; 4],
    pub transact_id: u8,
    pub status:4: u8,
    pub rsvd:4: u8,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_a0_s {
    pub link_up: u32,
    pub link_down: u32,
    pub link: },
    pub tx_unicast_octets: u64,
    pub tx_multicast_octets: u64,
    pub tx_broadcast_octets: u64,
    pub rx_unicast_octets: u64,
    pub rx_multicast_octets: u64,
    pub rx_broadcast_octets: u64,
    pub tx_unicast_frames: u32,
    pub tx_multicast_frames: u32,
    pub tx_broadcast_frames: u32,
    pub tx_errors: u32,
    pub rx_unicast_frames: u32,
    pub rx_multicast_frames: u32,
    pub rx_broadcast_frames: u32,
    pub rx_dropped_frames: u32,
    pub rx_error_frames: u32,
    pub tx_good_frames: u32,
    pub rx_good_frames: u32,
    pub reserve_fw_gap: u32,
    pub msm: },
    pub main_loop_cycles: u32,
    pub reserve_fw_gap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __packed {
    pub a0: statistics_a0_s,
    pub b0: statistics_b0_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_caps_s {
    pub l2_filters_base_index:6: u8,
    pub flexible_filter_mask:2: u8,
    pub l2_filter_count: u8,
    pub ethertype_filter_base_index: u8,
    pub ethertype_filter_count: u8,
    pub vlan_filter_base_index: u8,
    pub vlan_filter_count: u8,
    pub l3_ip4_filter_base_index:4: u8,
    pub l3_ip4_filter_count:4: u8,
    pub l3_ip6_filter_base_index:4: u8,
    pub l3_ip6_filter_count:4: u8,
    pub l4_filter_base_index:4: u8,
    pub l4_filter_count:4: u8,
    pub l4_flex_filter_base_index:4: u8,
    pub l4_flex_filter_count:4: u8,
    pub rslv_tbl_base_index: u8,
    pub rslv_tbl_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_policy_s {
    pub all:1: u8,
    pub mcast:1: u8,
    pub rx_queue_tc_index:5: u8,
    pub queue_or_tc:1: u8,
    pub promisc: },
    pub accept:1: u8,
    pub rsvd:1: u8,
    pub rx_queue_tc_index:5: u8,
    pub queue_or_tc:1: u8,
    pub bcast: },
    pub accept:1: u8,
    pub rsvd:1: u8,
    pub rx_queue_tc_index:5: u8,
    pub queue_or_tc:1: u8,
    pub mcast: },
    pub rsvd:8: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_interface_in {
    pub mtu: u32,
    pub rsvd1: u32,
    pub mac_address: mac_address_aligned_s,
    pub link_control: link_control_s,
    pub rsvd2: u32,
    pub link_options: link_options_s,
    pub rsvd3: u32,
    pub thermal_shutdown: thermal_shutdown_s,
    pub rsvd4: u32,
    pub sleep_proxy: sleep_proxy_s,
    pub rsvd5: u32,
    pub pause_quanta: [pause_quanta_s; 8],
    pub cable_diag_control: cable_diag_control_s,
    pub rsvd6: u32,
    pub data_buffer_status: data_buffer_status_s,
    pub rsvd7: u32,
    pub request_policy: request_policy_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transaction_counter_s {
    pub transaction_cnt_a: u16,
    pub transaction_cnt_b: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct management_status_s {
    pub mac_address: mac_address_s,
    pub vlan: u16,
    pub 1: u32 enable :,
    pub rsvd:31: u32,
    pub flags: },
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub rsvd3: u32,
    pub rsvd4: u32,
    pub rsvd5: u32,
}

pub const AQ_A2_FW_LINK_RATE_INVALID: c_int = 0;
pub const AQ_A2_FW_LINK_RATE_10M: c_int = 1;
pub const AQ_A2_FW_LINK_RATE_100M: c_int = 2;
pub const AQ_A2_FW_LINK_RATE_1G: c_int = 3;
pub const AQ_A2_FW_LINK_RATE_2G5: c_int = 4;
pub const AQ_A2_FW_LINK_RATE_5G: c_int = 5;
pub const AQ_A2_FW_LINK_RATE_10G: c_int = 6;

pub const AQ_A2_FW_INTERFACE_A0: c_int = 0;
pub const AQ_A2_FW_INTERFACE_B0: c_int = 1;
extern "C" {
    pub fn hw_atl2_utils_initfw(self: *mut aq_hw_s, fw_ops: *const aq_fw_ops) -> c_int;
}
extern "C" {
    pub fn hw_atl2_utils_soft_reset(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl2_utils_get_fw_version(self: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl2_utils_get_filter_caps(self: *mut aq_hw_s) -> c_int;
}
