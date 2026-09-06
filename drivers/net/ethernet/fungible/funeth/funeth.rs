//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/fungible/funeth/funeth.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)

pub const FUN_MAX_MTU: c_int = 9024;

pub const CQ_INTCOAL_USEC: c_int = 10;
pub const CQ_INTCOAL_NPKT: c_int = 16;
pub const SQ_INTCOAL_USEC: c_int = 10;
pub const SQ_INTCOAL_NPKT: c_int = 16;
pub const INVALID_LPORT: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_vport_info {
    pub mac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub vlan_proto: __be16,
    pub qos: u8,
    pub spoofchk:1: u8,
    pub trusted:1: u8,
    pub max_rate: c_uint,
}

// "subclass" of fun_dev for Ethernet functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_ethdev {
    pub fdev: fun_dev,
// the function's network ports
    pub netdevs: *mut net_device,
    pub num_ports: c_uint,
// configuration for the function's virtual ports
    pub num_vports: c_uint,
    pub vport_info: *mut fun_vport_info,
    pub /: *mut *mut mutex state_mutex; / nests inside RTNL if both taken,
    pub nsqs_per_port: c_uint,
}

extern "C" {
    pub fn container_of(_arg: p, fun_ethdev: struct, _arg: fdev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_qset {
    pub rxqs: *mut funeth_rxq,
    pub txqs: *mut funeth_txq,
    pub xdpqs: *mut funeth_txq,
    pub nrxqs: c_uint,
    pub ntxqs: c_uint,
    pub nxdpqs: c_uint,
    pub rxq_start: c_uint,
    pub txq_start: c_uint,
    pub xdpq_start: c_uint,
    pub cq_depth: c_uint,
    pub rq_depth: c_uint,
    pub sq_depth: c_uint,
    pub state: c_int,
}

// Per netdevice driver state, i.e., netdev_priv.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct funeth_priv {
    pub fdev: *mut fun_dev,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub rxqs: *mut *mut funeth_rxq  __rcu,
    pub txqs: *mut funeth_txq,
    pub xdpqs: *mut *mut funeth_txq  __rcu,
    pub irqs: xarray,
    pub num_tx_irqs: c_uint,
    pub num_rx_irqs: c_uint,
    pub rx_irq_ofst: c_uint,
    pub lane_attrs: c_uint,
    pub lport: u16,
// link settings
    pub port_caps: u64,
    pub advertising: u64,
    pub lp_advertising: u64,
    pub link_speed: c_uint,
    pub xcvr_type: u8,
    pub active_fc: u8,
    pub active_fec: u8,
    pub link_down_reason: u8,
    pub link_seq: seqcount_t,
    pub msg_enable: u32,
    pub num_xdpqs: c_uint,
// ethtool, etc. config parameters
    pub sq_depth: c_uint,
    pub rq_depth: c_uint,
    pub cq_depth: c_uint,
    pub cq_irq_db: c_uint,
    pub tx_coal_usec: u8,
    pub tx_coal_count: u8,
    pub rx_coal_usec: u8,
    pub rx_coal_count: u8,
    pub hwtstamp_cfg: kernel_hwtstamp_config,
// cumulative queue stats from earlier queue instances
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_dropped: u64,
// RSS
    pub rss_hw_id: c_uint,
    pub hash_algo: fun_eth_hash_alg,
    pub rss_key: [u8; FUN_ETH_RSS_MAX_KEY_SIZE],
    pub indir_table_nentries: c_uint,
    pub indir_table: [u32; FUN_ETH_RSS_MAX_INDIR_ENT],
    pub rss_dma_addr: dma_addr_t,
    pub rss_cfg: *mut c_void,
// DMA area for port stats
    pub stats_dma_addr: dma_addr_t,
    pub stats: *mut __be64,
    pub xdp_prog: *mut bpf_prog,
    pub dl_port: devlink_port,
// kTLS state
    pub ktls_id: c_uint,
    pub tx_tls_add: core::sync::atomic::AtomicI64,
    pub tx_tls_del: core::sync::atomic::AtomicI64,
    pub tx_tls_resync: core::sync::atomic::AtomicI64,
}

extern "C" {
    pub fn fun_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn fun_port_write_cmd(fp: *mut funeth_priv, key: c_int, data: u64) -> c_int;
}
extern "C" {
    pub fn fun_port_read_cmd(fp: *mut funeth_priv, key: c_int, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn fun_create_and_bind_tx(fp: *mut funeth_priv, sqid: u32) -> c_int;
}
