//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/common.h
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
// Copyright (c) 2005-2008 Chelsio, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// More powerful macro that selectively prints messages based on msg_enable.
// For info and debugging messages.
//

// Additional NETIF_MSG_* categories
pub const NETIF_MSG_MMIO: c_uint = 0x8000000;

pub const TP_VERSION_MAJOR: c_int = 1;
pub const TP_VERSION_MINOR: c_int = 1;
pub const TP_VERSION_MICRO: c_int = 0;
pub const S_TP_VERSION_MAJOR: c_int = 16;
pub const M_TP_VERSION_MAJOR: c_uint = 0xFF;

pub const S_TP_VERSION_MINOR: c_int = 8;
pub const M_TP_VERSION_MINOR: c_uint = 0xFF;

pub const S_TP_VERSION_MICRO: c_int = 0;
pub const M_TP_VERSION_MICRO: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sge_context_type {
    SGE_CNTXT_RDMA = 0,
    SGE_CNTXT_ETH = 2,
    SGE_CNTXT_OFLD = 4,
    SGE_CNTXT_CTRL = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_ent {
    pub len: [__be32; 2],
    pub addr: [__be64; 2],
}

// Must be 1 or 2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_ops {
    pub reg_addr): u16,
    pub val): u16 reg_addr, u16,
    pub mode_support: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_info {
    pub /: *mut *mut unsigned char nports0; / # of ports on channel 0,
    pub /: *mut *mut unsigned char nports1; / # of ports on channel 1,
    pub /: *mut *mut unsigned char phy_base_addr; / MDIO PHY base address,
    pub /: *mut *mut unsigned int gpio_out; / GPIO output settings,
    pub /: *mut *mut unsigned char gpio_intr[MAX_NPORTS]; / GPIO PHY IRQ pins,
    pub /: *mut *mut unsigned long caps; / adapter capabilities,
    pub /: *const *const *const mdio_ops mdio_ops; / MDIO operations,
    pub /: *const *const *const char desc; / product description,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc5_stats {
    pub parity_err: c_ulong,
    pub active_rgn_full: c_ulong,
    pub nfa_srch_err: c_ulong,
    pub unknown_cmd: c_ulong,
    pub reqq_parity_err: c_ulong,
    pub dispq_parity_err: c_ulong,
    pub del_act_empty: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc7_stats {
    pub corr_err: c_ulong,
    pub uncorr_err: c_ulong,
    pub parity_err: c_ulong,
    pub addr_err: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_stats {
    pub /: *mut *mut u64 tx_octets; / total # of octets in good frames,
    pub /: *mut *mut u64 tx_octets_bad; / total # of octets in error frames,
    pub /: *mut *mut u64 tx_frames; / all good frames,
    pub /: *mut *mut u64 tx_mcast_frames; / good multicast frames,
    pub /: *mut *mut u64 tx_bcast_frames; / good broadcast frames,
    pub /: *mut *mut u64 tx_pause; / # of transmitted pause frames,
    pub /: *mut *mut u64 tx_deferred; / frames with deferred transmissions,
    pub /: *mut *mut u64 tx_late_collisions; / # of late collisions,
    pub /: *mut *mut u64 tx_total_collisions; / # of total collisions,
    pub /: *mut *mut u64 tx_excess_collisions; / frame errors from excessive collissions,
    pub /: *mut *mut u64 tx_underrun; / # of Tx FIFO underruns,
    pub /: *mut *mut u64 tx_len_errs; / # of Tx length errors,
    pub /: *mut *mut u64 tx_mac_internal_errs; / # of internal MAC errors on Tx,
    pub /: *mut *mut u64 tx_excess_deferral; / # of frames with excessive deferral,
    pub /: *mut *mut u64 tx_fcs_errs; / # of frames with bad FCS,
    pub /: *mut *mut u64 tx_frames_64; / # of Tx frames in a particular range,
    pub tx_frames_65_127: u64,
    pub tx_frames_128_255: u64,
    pub tx_frames_256_511: u64,
    pub tx_frames_512_1023: u64,
    pub tx_frames_1024_1518: u64,
    pub tx_frames_1519_max: u64,
    pub /: *mut *mut u64 rx_octets; / total # of octets in good frames,
    pub /: *mut *mut u64 rx_octets_bad; / total # of octets in error frames,
    pub /: *mut *mut u64 rx_frames; / all good frames,
    pub /: *mut *mut u64 rx_mcast_frames; / good multicast frames,
    pub /: *mut *mut u64 rx_bcast_frames; / good broadcast frames,
    pub /: *mut *mut u64 rx_pause; / # of received pause frames,
    pub /: *mut *mut u64 rx_fcs_errs; / # of received frames with bad FCS,
    pub /: *mut *mut u64 rx_align_errs; / alignment errors,
    pub /: *mut *mut u64 rx_symbol_errs; / symbol errors,
    pub /: *mut *mut u64 rx_data_errs; / data errors,
    pub /: *mut *mut u64 rx_sequence_errs; / sequence errors,
    pub /: *mut *mut u64 rx_runt; / # of runt frames,
    pub /: *mut *mut u64 rx_jabber; / # of jabber frames,
    pub /: *mut *mut u64 rx_short; / # of short frames,
    pub /: *mut *mut u64 rx_too_long; / # of oversized frames,
    pub /: *mut *mut u64 rx_mac_internal_errs; / # of internal MAC errors on Rx,
    pub /: *mut *mut u64 rx_frames_64; / # of Rx frames in a particular range,
    pub rx_frames_65_127: u64,
    pub rx_frames_128_255: u64,
    pub rx_frames_256_511: u64,
    pub rx_frames_512_1023: u64,
    pub rx_frames_1024_1518: u64,
    pub rx_frames_1519_max: u64,
    pub /: *mut *mut u64 rx_cong_drops; / # of Rx drops due to SGE congestion,
    pub tx_fifo_parity_err: c_ulong,
    pub rx_fifo_parity_err: c_ulong,
    pub tx_fifo_urun: c_ulong,
    pub rx_fifo_ovfl: c_ulong,
    pub serdes_signal_loss: c_ulong,
    pub xaui_pcs_ctc_err: c_ulong,
    pub xaui_pcs_align_change: c_ulong,
    pub /: *mut *mut unsigned long num_toggled; / # times toggled TxEn due to stuck TX,
    pub /: *mut *mut unsigned long num_resets; / # times reset due to stuck TX,
    pub /: *mut *mut unsigned long link_faults; / # detected link faults,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_mib_stats {
    pub ipInReceive_hi: u32,
    pub ipInReceive_lo: u32,
    pub ipInHdrErrors_hi: u32,
    pub ipInHdrErrors_lo: u32,
    pub ipInAddrErrors_hi: u32,
    pub ipInAddrErrors_lo: u32,
    pub ipInUnknownProtos_hi: u32,
    pub ipInUnknownProtos_lo: u32,
    pub ipInDiscards_hi: u32,
    pub ipInDiscards_lo: u32,
    pub ipInDelivers_hi: u32,
    pub ipInDelivers_lo: u32,
    pub ipOutRequests_hi: u32,
    pub ipOutRequests_lo: u32,
    pub ipOutDiscards_hi: u32,
    pub ipOutDiscards_lo: u32,
    pub ipOutNoRoutes_hi: u32,
    pub ipOutNoRoutes_lo: u32,
    pub ipReasmTimeout: u32,
    pub ipReasmReqds: u32,
    pub ipReasmOKs: u32,
    pub ipReasmFails: u32,
    pub reserved: [u32; 8],
    pub tcpActiveOpens: u32,
    pub tcpPassiveOpens: u32,
    pub tcpAttemptFails: u32,
    pub tcpEstabResets: u32,
    pub tcpOutRsts: u32,
    pub tcpCurrEstab: u32,
    pub tcpInSegs_hi: u32,
    pub tcpInSegs_lo: u32,
    pub tcpOutSegs_hi: u32,
    pub tcpOutSegs_lo: u32,
    pub tcpRetransSeg_hi: u32,
    pub tcpRetransSeg_lo: u32,
    pub tcpInErrs_hi: u32,
    pub tcpInErrs_lo: u32,
    pub tcpRtoMin: u32,
    pub tcpRtoMax: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_params {
    pub /: *mut *mut unsigned int nchan; / # of channels,
    pub /: *mut *mut unsigned int pmrx_size; / total PMRX capacity,
    pub /: *mut *mut unsigned int pmtx_size; / total PMTX capacity,
    pub /: *mut *mut unsigned int cm_size; / total CM capacity,
    pub /: *mut *mut unsigned int chan_rx_size; / per channel Rx size,
    pub /: *mut *mut unsigned int chan_tx_size; / per channel Tx size,
    pub /: *mut *mut unsigned int rx_pg_size; / Rx page size,
    pub /: *mut *mut unsigned int tx_pg_size; / Tx page size,
    pub /: *mut *mut unsigned int rx_num_pgs; / # of Rx pages,
    pub /: *mut *mut unsigned int tx_num_pgs; / # of Tx pages,
    pub /: *mut *mut unsigned int ntimer_qs; / # of timer queues,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qset_params {
    pub /: *mut *mut unsigned int polling; / polling/interrupt service for rspq,
    pub /: *mut *mut unsigned int coalesce_usecs; / irq coalescing timer,
    pub /: *mut *mut unsigned int rspq_size; / # of entries in response queue,
    pub /: *mut *mut unsigned int fl_size; / # of entries in regular free list,
    pub /: *mut *mut unsigned int jumbo_size; / # of entries in jumbo free list,
    pub /: *mut *mut unsigned int txq_size[SGE_TXQ_PER_SET]; / Tx queue sizes,
    pub /: *mut *mut unsigned int cong_thres; / FL congestion threshold,
    pub /: *mut *mut unsigned int vector; / Interrupt (line or vector) number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_params {
    pub /: *mut *mut unsigned int max_pkt_size; / max offload pkt size,
    pub qset: [qset_params; SGE_QSETS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc5_params {
    pub /: *mut *mut unsigned int mode; / selects MC5 width,
    pub /: *mut *mut unsigned int nservers; / size of server region,
    pub /: *mut *mut unsigned int nfilters; / size of filter region,
    pub /: *mut *mut unsigned int nroutes; / size of routing region,
}

// Default MC5 region sizes
// MC5 modes, these must be non-0
// MC5 min active region size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpd_params {
    pub cclk: c_uint,
    pub mclk: c_uint,
    pub uclk: c_uint,
    pub mdc: c_uint,
    pub mem_timing: c_uint,
    pub 1]: u8 sn[SERNUM_LEN +,
    pub eth_base: [u8; 6],
    pub port_type: [u8; MAX_NPORTS],
    pub xauicfg: [c_ushort; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_params {
    pub vpd_cap_addr: c_uint,
    pub speed: c_ushort,
    pub width: c_uchar,
    pub variant: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_params {
    pub sge: sge_params,
    pub mc5: mc5_params,
    pub tp: tp_params,
    pub vpd: vpd_params,
    pub pci: pci_params,
    pub info: *const adapter_info,
    pub mtus: [c_ushort; NMTUS],
    pub a_wnd: [c_ushort; NCCTRL_WIN],
    pub b_wnd: [c_ushort; NCCTRL_WIN],
    pub /: *mut *mut unsigned int nports; / # of ethernet ports,
    pub /: *mut *mut unsigned int chan_map; / bitmap of in-use Tx channels,
    pub /: *mut *mut unsigned int stats_update_period; / MAC stats accumulation period,
    pub /: *mut *mut unsigned int linkpoll_period; / link poll period in 0.1s,
    pub /: *mut *mut unsigned int rev; / chip revision,
    pub offload: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_params {
    pub sip: u32,
    pub sip_mask: u32,
    pub dip: u32,
    pub dip_mask: u32,
    pub sport: u16,
    pub sport_mask: u16,
    pub dport: u16,
    pub dport_mask: u16,
    pub vlan:12: u32,
    pub vlan_mask:12: u32,
    pub intf:4: u32,
    pub intf_mask:4: u32,
    pub proto: u8,
    pub proto_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub /: *mut *mut unsigned int supported; / link capabilities,
    pub /: *mut *mut unsigned int advertising; / advertised capabilities,
    pub /: *mut *mut unsigned short requested_speed; / speed user has requested,
    pub /: *mut *mut unsigned short speed; / actual link speed,
    pub /: *mut *mut unsigned char requested_duplex; / duplex user has requested,
    pub /: *mut *mut unsigned char duplex; / actual link duplex,
    pub /: *mut *mut unsigned char requested_fc; / flow control user has requested,
    pub /: *mut *mut unsigned char fc; / actual link flow control,
    pub /: *mut *mut unsigned char autoneg; / autonegotiating?,
    pub /: *mut *mut unsigned int link_ok; / link up?,
}

pub const SPEED_INVALID: c_uint = 0xffff;
pub const DUPLEX_INVALID: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc5 {
    pub adapter: *mut adapter,
    pub tcam_size: c_uint,
    pub part_type: c_uchar,
    pub parity_enabled: c_uchar,
    pub mode: c_uchar,
    pub stats: mc5_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc7 {
    pub /: *mut *mut *mut adapter adapter; / backpointer to adapter,
    pub /: *mut *mut unsigned int size; / memory size in bytes,
    pub /: *mut *mut unsigned int width; / MC7 interface width,
    pub /: *mut *mut unsigned int offset; / register address offset for MC7 instance,
    pub /: *const *const *const char name; / name of MC7 instance,
    pub /: *mut *mut mc7_stats stats; / MC7 statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmac {
    pub adapter: *mut adapter,
    pub offset: c_uint,
    pub /: *mut *mut unsigned int nucast; / # of address filters for unicast MACs,
    pub tx_tcnt: c_uint,
    pub tx_xcnt: c_uint,
    pub tx_mcnt: u64,
    pub rx_xcnt: c_uint,
    pub rx_ocnt: c_uint,
    pub rx_mcnt: u64,
    pub toggle_cnt: c_uint,
    pub txen: c_uint,
    pub rx_pause: u64,
    pub stats: mac_stats,
}

// PHY loopback direction
// PHY interrupt types
// PHY module types
// PHY operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cphy_ops {
    pub wait): *mut *mut *mut int (reset)(struct cphy phy, int,
    pub phy): *mut *mut int (intr_enable)(struct cphy,
    pub phy): *mut *mut int (intr_disable)(struct cphy,
    pub phy): *mut *mut int (intr_clear)(struct cphy,
    pub phy): *mut *mut int (intr_handler)(struct cphy,
    pub phy): *mut *mut int (autoneg_enable)(struct cphy,
    pub phy): *mut *mut int (autoneg_restart)(struct cphy,
    pub advertise_map): *mut *mut *mut int (advertise)(struct cphy phy, unsigned int,
    pub enable): *mut *mut *mut int (set_loopback)(struct cphy phy, int mmd, int dir, int,
    pub duplex): *mut *mut *mut int (set_speed_duplex)(struct cphy phy, int speed, int,
    pub fc): *mut *mut int duplex, int,
    pub enable): *mut *mut *mut int (power_down)(struct cphy phy, int,
    pub mmds: u32,
}

// A PHY instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cphy {
    pub /: *mut *mut u8 modtype; / PHY module type,
    pub /: *mut *mut short priv; / scratch pad,
    pub /: *mut *mut unsigned int caps; / PHY capabilities,
    pub /: *mut *mut *mut adapter adapter; / associated adapter,
    pub /: *const *const *const char desc; / PHY description,
    pub /: *mut *mut unsigned long fifo_errors; / FIFO over/under-flows,
    pub /: *const *const *const cphy_ops ops; / PHY operations,
    pub mdio: mdio_if_info,
    pub /: *mut *mut u16 phy_cache[EDC_MAX_SIZE]; / EDC cache,
}

// Convenience MDIO read/write wrappers
// valp = (rc >= 0) ? rc : -1;
// Convenience initializer
// Accumulate MAC statistics every 180 seconds.  For 1G we multiply by 10.
pub const MAC_STATS_ACCUM_SECS: c_int = 180;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_val_pair {
    pub reg_addr: c_uint,
    pub val: c_uint,
}

extern "C" {
    pub fn t3_phy_reset(phy: *mut cphy, mmd: c_int, wait: c_int) -> c_int;
}
extern "C" {
    pub fn t3_phy_advertise(phy: *mut cphy, advert: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_phy_advertise_fiber(phy: *mut cphy, advert: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_set_phy_speed_duplex(phy: *mut cphy, speed: c_int, duplex: c_int) -> c_int;
}
extern "C" {
    pub fn t3_phy_lasi_intr_enable(phy: *mut cphy) -> c_int;
}
extern "C" {
    pub fn t3_phy_lasi_intr_disable(phy: *mut cphy) -> c_int;
}
extern "C" {
    pub fn t3_phy_lasi_intr_clear(phy: *mut cphy) -> c_int;
}
extern "C" {
    pub fn t3_phy_lasi_intr_handler(phy: *mut cphy) -> c_int;
}
extern "C" {
    pub fn t3_intr_enable(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_intr_disable(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_intr_clear(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_xgm_intr_enable(adapter: *mut adapter, idx: c_int);
}
extern "C" {
    pub fn t3_xgm_intr_disable(adapter: *mut adapter, idx: c_int);
}
extern "C" {
    pub fn t3_port_intr_enable(adapter: *mut adapter, idx: c_int);
}
extern "C" {
    pub fn t3_port_intr_disable(adapter: *mut adapter, idx: c_int);
}
extern "C" {
    pub fn t3_slow_intr_handler(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_phy_intr_handler(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_link_changed(adapter: *mut adapter, port_id: c_int);
}
extern "C" {
    pub fn t3_link_fault(adapter: *mut adapter, port_id: c_int);
}
extern "C" {
    pub fn t3_link_start(phy: *mut cphy, mac: *mut cmac, lc: *mut link_config) -> c_int;
}
extern "C" {
    pub fn t3_seeprom_wp(adapter: *mut adapter, enable: c_int) -> c_int;
}
extern "C" {
    pub fn t3_get_tp_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t3_check_tpsram_version(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_set_proto_sram(adap: *mut adapter, data: *const u8) -> c_int;
}
extern "C" {
    pub fn t3_load_fw(adapter: *mut adapter, fw_data: *const *const u8, size: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_get_fw_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t3_check_fw_version(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_init_hw(adapter: *mut adapter, fw_params: u32) -> c_int;
}
extern "C" {
    pub fn t3_reset_adapter(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_replay_prep_adapter(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t3_led_ready(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_fatal_err(adapter: *mut adapter);
}
extern "C" {
    pub fn t3_set_vlan_accel(adapter: *mut adapter, ports: c_uint, on: c_int);
}
extern "C" {
    pub fn t3_mac_reset(mac: *mut cmac) -> c_int;
}
extern "C" {
    pub fn t3b_pcs_reset(mac: *mut cmac);
}
extern "C" {
    pub fn t3_mac_disable_exact_filters(mac: *mut cmac);
}
extern "C" {
    pub fn t3_mac_enable_exact_filters(mac: *mut cmac);
}
extern "C" {
    pub fn t3_mac_enable(mac: *mut cmac, which: c_int) -> c_int;
}
extern "C" {
    pub fn t3_mac_disable(mac: *mut cmac, which: c_int) -> c_int;
}
extern "C" {
    pub fn t3_mac_set_mtu(mac: *mut cmac, mtu: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_mac_set_rx_mode(mac: *mut cmac, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn t3_mac_set_address(mac: *mut cmac, idx: c_uint, addr[6]: u8) -> c_int;
}
extern "C" {
    pub fn t3_mac_set_num_ucast(mac: *mut cmac, n: c_int) -> c_int;
}
extern "C" {
    pub fn t3_mac_set_speed_duplex_fc(mac: *mut cmac, speed: c_int, duplex: c_int, fc: c_int) -> c_int;
}
extern "C" {
    pub fn t3b2_mac_watchdog_task(mac: *mut cmac) -> c_int;
}
extern "C" {
    pub fn t3_mc5_prep(adapter: *mut adapter, mc5: *mut mc5, mode: c_int);
}
extern "C" {
    pub fn t3_mc5_intr_handler(mc5: *mut mc5);
}
extern "C" {
    pub fn t3_tp_set_offload_mode(adap: *mut adapter, enable: c_int);
}
extern "C" {
    pub fn t3_tp_get_mib_stats(adap: *mut adapter, tps: *mut tp_mib_stats);
}
extern "C" {
    pub fn t3_config_sched(adap: *mut adapter, kbps: c_uint, sched: c_int) -> c_int;
}
extern "C" {
    pub fn t3_sge_prep(adap: *mut adapter, p: *mut sge_params);
}
extern "C" {
    pub fn t3_sge_init(adap: *mut adapter, p: *mut sge_params);
}
extern "C" {
    pub fn t3_sge_enable_ecntxt(adapter: *mut adapter, id: c_uint, enable: c_int) -> c_int;
}
extern "C" {
    pub fn t3_sge_disable_fl(adapter: *mut adapter, id: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_sge_disable_rspcntxt(adapter: *mut adapter, id: c_uint) -> c_int;
}
extern "C" {
    pub fn t3_sge_disable_cqcntxt(adapter: *mut adapter, id: c_uint) -> c_int;
}
