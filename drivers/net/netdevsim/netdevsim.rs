//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/netdevsim/netdevsim.h
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


//
// Copyright (C) 2017 Netronome Systems, Inc.
//
// This software is licensed under the GNU General License Version 2,
// June 1991 as shown in the file COPYING in the top-level directory of this
// source tree.
//
// THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM "AS IS"
// WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING,
// BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE. THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE
// OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME
// THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.
//

pub const NSIM_XDP_MAX_MTU: c_int = 4000;

pub const NSIM_IPSEC_MAX_SA_COUNT: c_int = 33;

pub const NSIM_UDP_TUNNEL_N_PORTS: c_int = 4;
pub const NSIM_HDS_THRESHOLD_MAX: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_sa {
    pub xs: *mut xfrm_state,
    pub ipaddr: [__be32; 4],
    pub key: [u32; 4],
    pub salt: u32,
    pub used: bool,
    pub crypt: bool,
    pub rx: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_ipsec {
    pub sa: [nsim_sa; NSIM_IPSEC_MAX_SA_COUNT],
    pub pfile: *mut dentry,
    pub count: u32,
    pub tx: u32,
}

pub const NSIM_MACSEC_MAX_SECY_COUNT: c_int = 3;
pub const NSIM_MACSEC_MAX_RXSC_COUNT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_rxsc {
    pub sci: sci_t,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_secy {
    pub sci: sci_t,
    pub nsim_rxsc: [nsim_rxsc; NSIM_MACSEC_MAX_RXSC_COUNT],
    pub nsim_rxsc_count: u8,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_macsec {
    pub nsim_secy: [nsim_secy; NSIM_MACSEC_MAX_SECY_COUNT],
    pub nsim_secy_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_vlan {
    pub VLAN_N_VID): DECLARE_BITMAP(ctag,,
    pub VLAN_N_VID): DECLARE_BITMAP(stag,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_ethtool_pauseparam {
    pub rx: bool,
    pub tx: bool,
    pub report_stats_rx: bool,
    pub report_stats_tx: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_ethtool {
    pub get_err: u32,
    pub set_err: u32,
    pub channels: u32,
    pub pauseparam: nsim_ethtool_pauseparam,
    pub coalesce: ethtool_coalesce,
    pub ring: ethtool_ringparam,
    pub fec: ethtool_fecparam,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_rq {
    pub napi: napi_struct,
    pub skb_queue: sk_buff_head,
    pub page_pool: *mut page_pool,
    pub napi_timer: hrtimer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdevsim {
    pub netdev: *mut net_device,
    pub nsim_dev: *mut nsim_dev,
    pub nsim_dev_port: *mut nsim_dev_port,
    pub phc: *mut mock_phc,
    pub rq: *mut nsim_rq,
    pub rq_reset_mode: c_int,
    pub rx_packets: core::sync::atomic::AtomicI64,
    pub rx_bytes: core::sync::atomic::AtomicI64,
    pub tx_packets: core::sync::atomic::AtomicI64,
    pub tx_bytes: core::sync::atomic::AtomicI64,
    pub dev: *mut psp_dev __rcu,
    pub rereg: *mut dentry,
    pub rereg_lock: mutex,
    pub spi: u32,
    pub assoc_cnt: u32,
    pub psp: },
    pub nsim_bus_dev: *mut nsim_bus_dev,
    pub bpf_offloaded: *mut bpf_prog,
    pub bpf_offloaded_id: u32,
    pub xdp: xdp_attachment_info,
    pub xdp_hw: xdp_attachment_info,
    pub bpf_tc_accept: bool,
    pub bpf_tc_non_bound_accept: bool,
    pub bpf_xdpdrv_accept: bool,
    pub bpf_xdpoffload_accept: bool,
    pub bpf_map_accept: bool,
    pub ipsec: nsim_ipsec,
    pub macsec: nsim_macsec,
    pub vlan: nsim_vlan,
    pub inject_error: u32,
    pub __ports: [u32; 2][NSIM_UDP_TUNNEL_N_PORTS],
    pub (*ports)[NSIM_UDP_TUNNEL_N_PORTS]: *mut u32,
    pub ddir: *mut dentry,
    pub dfs_ports: [debugfs_u32_array; 2],
    pub udp_ports: },
    pub page: *mut page,
    pub pp_dfs: *mut dentry,
    pub qr_dfs: *mut dentry,
    pub vlan_dfs: *mut dentry,
    pub ethtool_ddir: *mut dentry,
    pub ethtool: nsim_ethtool,
    pub peer: *mut netdevsim __rcu,
    pub nb: notifier_block,
    pub nn: netdev_net_notifier,
}

extern "C" {
    pub fn nsim_destroy(ns: *mut netdevsim);
}
extern "C" {
    pub fn netdev_is_nsim(dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn nsim_ethtool_init(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_ethtool_fini(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_udp_tunnels_debugfs_create(nsim_dev: *mut nsim_dev);
}
extern "C" {
    pub fn nsim_udp_tunnels_info_destroy(dev: *mut net_device);
}

extern "C" {
    pub fn nsim_bpf_dev_init(nsim_dev: *mut nsim_dev) -> c_int;
}
extern "C" {
    pub fn nsim_bpf_dev_exit(nsim_dev: *mut nsim_dev);
}
extern "C" {
    pub fn nsim_bpf_init(ns: *mut netdevsim) -> c_int;
}
extern "C" {
    pub fn nsim_bpf_uninit(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_bpf(dev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn nsim_bpf_disable_tc(ns: *mut netdevsim) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nsim_resource_id {
    NSIM_RESOURCE_NONE,   /* DEVLINK_RESOURCE_ID_PARENT_TOP */
    NSIM_RESOURCE_IPV4,
    NSIM_RESOURCE_IPV4_FIB,
    NSIM_RESOURCE_IPV4_FIB_RULES,
    NSIM_RESOURCE_IPV6,
    NSIM_RESOURCE_IPV6_FIB,
    NSIM_RESOURCE_IPV6_FIB_RULES,
    NSIM_RESOURCE_NEXTHOPS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nsim_port_resource_id {
    NSIM_PORT_RESOURCE_TEST = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_dev_health {
    pub empty_reporter: *mut devlink_health_reporter,
    pub dummy_reporter: *mut devlink_health_reporter,
    pub ddir: *mut dentry,
    pub recovered_break_msg: *mut c_char,
    pub binary_len: u32,
    pub fail_recover: bool,
}

extern "C" {
    pub fn nsim_dev_health_init(nsim_dev: *mut nsim_dev, devlink: *mut devlink) -> c_int;
}
extern "C" {
    pub fn nsim_dev_health_exit(nsim_dev: *mut nsim_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_dev_hwstats_netdev {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub stats: rtnl_hw_stats64,
    pub enabled: bool,
    pub fail_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_dev_hwstats {
    pub ddir: *mut dentry,
    pub l3_ddir: *mut dentry,
    pub /: *mut *mut mutex hwsdev_list_lock; / protects hwsdev list(s),
    pub l3_list: list_head,
    pub netdevice_nb: notifier_block,
    pub traffic_dw: delayed_work,
}

extern "C" {
    pub fn nsim_dev_hwstats_init(nsim_dev: *mut nsim_dev) -> c_int;
}
extern "C" {
    pub fn nsim_dev_hwstats_exit(nsim_dev: *mut nsim_dev);
}

extern "C" {
    pub fn nsim_dev_psample_init(nsim_dev: *mut nsim_dev) -> c_int;
}
extern "C" {
    pub fn nsim_dev_psample_exit(nsim_dev: *mut nsim_dev);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nsim_dev_port_type {
    NSIM_DEV_PORT_TYPE_PF,
    NSIM_DEV_PORT_TYPE_VF,
}

pub const NSIM_DEV_VF_PORT_INDEX_BASE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_dev_port {
    pub list: list_head,
    pub devlink_port: devlink_port,
    pub port_index: c_uint,
    pub port_type: nsim_dev_port_type,
    pub ddir: *mut dentry,
    pub rate_parent: *mut dentry,
    pub parent_name: *mut c_char,
    pub tc_bw: [u32; DEVLINK_RATE_TCS_MAX],
    pub ns: *mut netdevsim,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_vf_config {
    pub link_state: c_int,
    pub min_tx_rate: u16,
    pub max_tx_rate: u16,
    pub vlan: u16,
    pub vlan_proto: __be16,
    pub qos: u16,
    pub vf_mac: [u8; ETH_ALEN],
    pub spoofchk_enabled: bool,
    pub trusted: bool,
    pub rss_query_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_dev {
    pub nsim_bus_dev: *mut nsim_bus_dev,
    pub fib_data: *mut nsim_fib_data,
    pub trap_data: *mut nsim_trap_data,
    pub ddir: *mut dentry,
    pub ports_ddir: *mut dentry,
    pub take_snapshot: *mut dentry,
    pub nodes_ddir: *mut dentry,
    pub vfconfigs: *mut nsim_vf_config,
    pub bpf_dev: *mut bpf_offload_dev,
    pub bpf_bind_accept: bool,
    pub bpf_bind_verifier_accept: bool,
    pub bpf_bind_verifier_delay: u32,
    pub ddir_bpf_bound_progs: *mut dentry,
    pub prog_id_gen: u32,
    pub bpf_bound_progs: list_head,
    pub bpf_bound_maps: list_head,
    pub progs_list_lock: mutex,
    pub switch_id: netdev_phys_item_id,
    pub port_list: list_head,
    pub fw_update_status: bool,
    pub fw_update_overwrite_mask: u32,
    pub fw_update_flash_chunk_time_ms: u32,
    pub max_macs: u32,
    pub test1: bool,
    pub test2: u32,
    pub dont_allow_reload: bool,
    pub fail_reload: bool,
    pub dummy_region: *mut devlink_region,
    pub health: nsim_dev_health,
    pub hwstats: nsim_dev_hwstats,
    pub fa_cookie: *mut flow_action_cookie,
    pub /: *mut *mut spinlock_t fa_cookie_lock; / protects fa_cookie,
    pub fail_trap_group_set: bool,
    pub fail_trap_policer_set: bool,
    pub fail_trap_policer_counter_get: bool,
    pub fail_trap_drop_counter_get: bool,
    pub utn_shared: udp_tunnel_nic_shared,
    pub __ports: [u32; 2][NSIM_UDP_TUNNEL_N_PORTS],
    pub sync_all: bool,
    pub open_only: bool,
    pub ipv4_only: bool,
    pub shared: bool,
    pub static_iana_vxlan: bool,
    pub udp_ports: },
    pub psample: *mut nsim_dev_psample,
    pub esw_mode: u16,
}

extern "C" {
    pub fn devlink_net(_arg: priv_to_devlink(nsim_dev)) -> return;
}
extern "C" {
    pub fn nsim_dev_init() -> c_int;
}
extern "C" {
    pub fn nsim_dev_exit();
}
extern "C" {
    pub fn nsim_drv_probe(nsim_bus_dev: *mut nsim_bus_dev) -> c_int;
}
extern "C" {
    pub fn nsim_drv_remove(nsim_bus_dev: *mut nsim_bus_dev);
}
extern "C" {
    pub fn nsim_dev_get_vfs(nsim_dev: *mut nsim_dev) -> c_uint;
}
extern "C" {
    pub fn nsim_fib_destroy(devlink: *mut devlink, fib_data: *mut nsim_fib_data);
}

extern "C" {
    pub fn nsim_ipsec_init(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_ipsec_teardown(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_ipsec_tx(ns: *mut netdevsim, skb: *mut sk_buff) -> bool;
}

extern "C" {
    pub fn nsim_macsec_init(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_macsec_teardown(ns: *mut netdevsim);
}

extern "C" {
    pub fn nsim_psp_init(ns: *mut netdevsim) -> c_int;
}
extern "C" {
    pub fn nsim_psp_uninit(ns: *mut netdevsim);
}
extern "C" {
    pub fn nsim_psp_handle_ext(skb: *mut sk_buff, psp_ext: *mut skb_ext);
}

pub const NSIM_BUS_DEV_MAX_VFS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsim_bus_dev {
    pub dev: device,
    pub list: list_head,
    pub port_count: c_uint,
    pub /: *mut *mut unsigned int num_queues; / Number of queues for each port on this bus,
    pub pointer: *mut *mut *mut net initial_net; / Purpose of this is to carry net,
// during the probe time only.
//
    pub num_vfs: c_uint,
    pub init: bool,
}

extern "C" {
    pub fn nsim_bus_init() -> c_int;
}
extern "C" {
    pub fn nsim_bus_exit();
}
