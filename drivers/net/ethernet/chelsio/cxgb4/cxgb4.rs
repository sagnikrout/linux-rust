//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
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

// Suspend an Ethernet Tx queue with fewer available descriptors than this.
// This is the same as calc_tx_descs() for a TSO packet with
// nr_frags == MAX_SKB_FRAGS.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_master {
    MASTER_CANT,
    MASTER_MAY,
    MASTER_MUST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_state {
    DEV_STATE_UNINIT,
    DEV_STATE_INIT,
    DEV_STATE_ERR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_pause {
    PAUSE_RX      = 1 << 0,
    PAUSE_TX      = 1 << 1,
    PAUSE_AUTONEG = 1 << 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_fec {
    FEC_AUTO      = 1 << 0,	 /* IEEE 802.3 "automatic" */
    FEC_RS        = 1 << 1,  /* Reed-Solomon */
    FEC_BASER_RS  = 1 << 2   /* BaseR/Reed-Solomon */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_netdev_tls_ops {
    CXGB4_TLSDEV_OPS  = 1,
    CXGB4_XFRMDEV_OPS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_bootcfg_data {
    pub signature: __le16,
    pub reserved: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_pcir_data {
    pub /: *mut *mut __le32 signature; / Signature. The string "PCIR",
    pub /: *mut *mut __le16 vendor_id; / Vendor Identification,
    pub /: *mut *mut __le16 device_id; / Device Identification,
    pub /: *mut *mut __u8 vital_product[2]; / Pointer to Vital Product Data,
    pub /: *mut *mut __u8 length[2]; / PCIR Data Structure Length,
    pub /: *mut *mut __u8 revision; / PCIR Data Structure Revision,
    pub /: *mut *mut __u8 class_code[3]; / Class Code,
    pub /: *mut *mut __u8 image_length[2]; / Image Length. Multiple of 512B,
    pub /: *mut *mut __u8 code_revision[2]; / Revision Level of Code/Data,
    pub code_type: __u8,
    pub indicator: __u8,
    pub reserved: [__u8; 2],
}

// BIOS boot headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_pci_exp_rom_header {
    pub /: *mut *mut __le16 signature; / ROM Signature. Should be 0xaa55,
    pub /: *mut *mut __u8 reserved[22]; / Reserved per processor Architecture data,
    pub /: *mut *mut __le16 pcir_offset; / Offset to PCI Data Structure,
}

// Legacy PCI Expansion ROM Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_pci_rom_hdr {
    pub /: *mut *mut __u8 signature[2]; / ROM Signature. Should be 0xaa55,
    pub /: *mut *mut __u8 size512; / Current Image Size in units of 512 bytes,
    pub initentry_point: [__u8; 4],
    pub /: *mut *mut __u8 cksum; / Checksum computed on the entire Image,
    pub /: *mut *mut __u8 reserved[16]; / Reserved,
    pub /: *mut *mut __le16 pcir_offset; / Offset to PCI Data Struture,
}

pub const CXGB4_HDR_CODE1: c_uint = 0x00;
pub const CXGB4_HDR_CODE2: c_uint = 0x03;
pub const CXGB4_HDR_INDI: c_uint = 0x80;
// BOOT constants
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_stats {
    pub /: *mut *mut u64 tx_octets; / total # of octets in good frames,
    pub /: *mut *mut u64 tx_frames; / all good frames,
    pub /: *mut *mut u64 tx_bcast_frames; / all broadcast frames,
    pub /: *mut *mut u64 tx_mcast_frames; / all multicast frames,
    pub /: *mut *mut u64 tx_ucast_frames; / all unicast frames,
    pub /: *mut *mut u64 tx_error_frames; / all error frames,
    pub /: *mut *mut u64 tx_frames_64; / # of Tx frames in a particular range,
    pub tx_frames_65_127: u64,
    pub tx_frames_128_255: u64,
    pub tx_frames_256_511: u64,
    pub tx_frames_512_1023: u64,
    pub tx_frames_1024_1518: u64,
    pub tx_frames_1519_max: u64,
    pub /: *mut *mut u64 tx_drop; / # of dropped Tx frames,
    pub /: *mut *mut u64 tx_pause; / # of transmitted pause frames,
    pub /: *mut *mut u64 tx_ppp0; / # of transmitted PPP prio 0 frames,
    pub /: *mut *mut u64 tx_ppp1; / # of transmitted PPP prio 1 frames,
    pub /: *mut *mut u64 tx_ppp2; / # of transmitted PPP prio 2 frames,
    pub /: *mut *mut u64 tx_ppp3; / # of transmitted PPP prio 3 frames,
    pub /: *mut *mut u64 tx_ppp4; / # of transmitted PPP prio 4 frames,
    pub /: *mut *mut u64 tx_ppp5; / # of transmitted PPP prio 5 frames,
    pub /: *mut *mut u64 tx_ppp6; / # of transmitted PPP prio 6 frames,
    pub /: *mut *mut u64 tx_ppp7; / # of transmitted PPP prio 7 frames,
    pub /: *mut *mut u64 rx_octets; / total # of octets in good frames,
    pub /: *mut *mut u64 rx_frames; / all good frames,
    pub /: *mut *mut u64 rx_bcast_frames; / all broadcast frames,
    pub /: *mut *mut u64 rx_mcast_frames; / all multicast frames,
    pub /: *mut *mut u64 rx_ucast_frames; / all unicast frames,
    pub /: *mut *mut u64 rx_too_long; / # of frames exceeding MTU,
    pub /: *mut *mut u64 rx_jabber; / # of jabber frames,
    pub /: *mut *mut u64 rx_fcs_err; / # of received frames with bad FCS,
    pub /: *mut *mut u64 rx_len_err; / # of received frames with length error,
    pub /: *mut *mut u64 rx_symbol_err; / symbol errors,
    pub /: *mut *mut u64 rx_runt; / # of short frames,
    pub /: *mut *mut u64 rx_frames_64; / # of Rx frames in a particular range,
    pub rx_frames_65_127: u64,
    pub rx_frames_128_255: u64,
    pub rx_frames_256_511: u64,
    pub rx_frames_512_1023: u64,
    pub rx_frames_1024_1518: u64,
    pub rx_frames_1519_max: u64,
    pub /: *mut *mut u64 rx_pause; / # of received pause frames,
    pub /: *mut *mut u64 rx_ppp0; / # of received PPP prio 0 frames,
    pub /: *mut *mut u64 rx_ppp1; / # of received PPP prio 1 frames,
    pub /: *mut *mut u64 rx_ppp2; / # of received PPP prio 2 frames,
    pub /: *mut *mut u64 rx_ppp3; / # of received PPP prio 3 frames,
    pub /: *mut *mut u64 rx_ppp4; / # of received PPP prio 4 frames,
    pub /: *mut *mut u64 rx_ppp5; / # of received PPP prio 5 frames,
    pub /: *mut *mut u64 rx_ppp6; / # of received PPP prio 6 frames,
    pub /: *mut *mut u64 rx_ppp7; / # of received PPP prio 7 frames,
    pub /: *mut *mut u64 rx_ovflow0; / drops due to buffer-group 0 overflows,
    pub /: *mut *mut u64 rx_ovflow1; / drops due to buffer-group 1 overflows,
    pub /: *mut *mut u64 rx_ovflow2; / drops due to buffer-group 2 overflows,
    pub /: *mut *mut u64 rx_ovflow3; / drops due to buffer-group 3 overflows,
    pub /: *mut *mut u64 rx_trunc0; / buffer-group 0 truncated packets,
    pub /: *mut *mut u64 rx_trunc1; / buffer-group 1 truncated packets,
    pub /: *mut *mut u64 rx_trunc2; / buffer-group 2 truncated packets,
    pub /: *mut *mut u64 rx_trunc3; / buffer-group 3 truncated packets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_port_stats {
    pub octets: u64,
    pub frames: u64,
    pub bcast_frames: u64,
    pub mcast_frames: u64,
    pub ucast_frames: u64,
    pub error_frames: u64,
    pub frames_64: u64,
    pub frames_65_127: u64,
    pub frames_128_255: u64,
    pub frames_256_511: u64,
    pub frames_512_1023: u64,
    pub frames_1024_1518: u64,
    pub frames_1519_max: u64,
    pub drop: u64,
    pub ovflow0: u64,
    pub ovflow1: u64,
    pub ovflow2: u64,
    pub ovflow3: u64,
    pub trunc0: u64,
    pub trunc1: u64,
    pub trunc2: u64,
    pub trunc3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_tcp_stats {
    pub tcp_out_rsts: u32,
    pub tcp_in_segs: u64,
    pub tcp_out_segs: u64,
    pub tcp_retrans_segs: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_usm_stats {
    pub frames: u32,
    pub drops: u32,
    pub octets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_fcoe_stats {
    pub frames_ddp: u32,
    pub frames_drop: u32,
    pub octets_ddp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_err_stats {
    pub mac_in_errs: [u32; 4],
    pub hdr_in_errs: [u32; 4],
    pub tcp_in_errs: [u32; 4],
    pub tnl_cong_drops: [u32; 4],
    pub ofld_chan_drops: [u32; 4],
    pub tnl_tx_drops: [u32; 4],
    pub ofld_vlan_drops: [u32; 4],
    pub tcp6_in_errs: [u32; 4],
    pub ofld_no_neigh: u32,
    pub ofld_cong_defer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_cpl_stats {
    pub req: [u32; 4],
    pub rsp: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_rdma_stats {
    pub rqe_dfr_pkt: u32,
    pub rqe_dfr_mod: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_params {
    pub /: *mut *mut u32 hps; / host page size for our PF/VF,
    pub /: *mut *mut u32 eq_qpp; / egress queues/page for our PF/VF,
    pub /: *mut *mut u32 iq_qpp; / egress queues/page for our PF/VF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_params {
    pub /: *mut *mut unsigned int tre; / log2 of core clocks per TP tick,
    pub /: *mut *mut unsigned int la_mask; / what events are recorded by TP LA,
    pub /: *mut *mut unsigned short tx_modq_map; / TX modulation scheduler queue to,
// channel map
    pub /: *mut *mut uint32_t dack_re; / DACK timer resolution,
    pub /: *mut *mut unsigned short tx_modq[NCHAN]; / channel to modulation queue map,
    pub /: *mut *mut u32 vlan_pri_map; / cached TP_VLAN_PRI_MAP,
    pub filter_mask: u32,
    pub /: *mut *mut u32 ingress_config; / cached TP_INGRESS_CONFIG,
// cached TP_OUT_CONFIG compressed error vector
// and passing outer header info for encapsulated packets.
//
    pub rx_pkt_encap: c_int,
// TP_VLAN_PRI_MAP Compressed Filter Tuple field offsets.  This is a
// subset of the set of fields which may be present in the Compressed
// Filter Tuple portion of filters and TCP TCB connections.  The
// fields which are present are controlled by the TP_VLAN_PRI_MAP.
// Since a variable number of fields may or may not be present, their
// shifted field positions within the Compressed Filter Tuple may
// vary, or not even be present if the field isn't selected in
// TP_VLAN_PRI_MAP.  Since some of these fields are needed in various
// places we store their offsets here, or a -1 if the field isn't
// present.
//
    pub fcoe_shift: c_int,
    pub port_shift: c_int,
    pub vnic_shift: c_int,
    pub vlan_shift: c_int,
    pub tos_shift: c_int,
    pub protocol_shift: c_int,
    pub ethertype_shift: c_int,
    pub macmatch_shift: c_int,
    pub matchtype_shift: c_int,
    pub frag_shift: c_int,
    pub hash_filter_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpd_params {
    pub cclk: c_uint,
    pub 1]: u8 sn[SERNUM_LEN +,
    pub 1]: u8 id[ID_LEN +,
    pub 1]: u8 pn[PN_LEN +,
    pub 1]: u8 na[MACADDR_LEN +,
}

// Maximum resources provisioned for a PCI PF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_resources {
    pub /: *mut *mut unsigned int nvi; / N virtual interfaces,
    pub /: *mut *mut unsigned int neq; / N egress Qs,
    pub /: *mut *mut unsigned int nethctrl; / N egress ETH or CTRL Qs,
    pub /: *mut *mut unsigned int niqflint; / N ingress Qs/w free list(s) & intr,
    pub /: *mut *mut unsigned int niq; / N ingress Qs,
    pub /: *mut *mut unsigned int tc; / PCI-E traffic class,
    pub /: *mut *mut unsigned int pmask; / port access rights mask,
    pub /: *mut *mut unsigned int nexactf; / N exact MPS filters,
    pub /: *mut *mut unsigned int r_caps; / read capabilities,
    pub /: *mut *mut unsigned int wx_caps; / write/execute capabilities,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_params {
    pub speed: c_uchar,
    pub width: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlog_params {
    pub /: *mut *mut u32 memtype; / which memory (EDC0, EDC1, MC),
    pub /: *mut *mut u32 start; / start of log in firmware memory,
    pub /: *mut *mut u32 size; / size of log,
}

// Stores chip specific parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_specific_params {
    pub nchan: u8,
    pub pm_stats_cnt: u8,
    pub /: *mut *mut u8 cng_ch_bits_log; / congestion channel map bits width,
    pub mps_rplc_size: u16,
    pub vfcount: u16,
    pub sge_fl_db: u32,
    pub mps_tcam_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_params {
    pub sge: sge_params,
    pub tp: tp_params,
    pub vpd: vpd_params,
    pub pfres: pf_resources,
    pub pci: pci_params,
    pub devlog: devlog_params,
    pub drv_memwin: pcie_memwin,
    pub cim_la_size: c_uint,
    pub /: *mut *mut unsigned int sf_size; / serial flash size in bytes,
    pub /: *mut *mut unsigned int sf_nsec; / # of flash sectors,
    pub /: *mut *mut unsigned int fw_vers; / firmware version,
    pub /: *mut *mut unsigned int bs_vers; / bootstrap version,
    pub /: *mut *mut unsigned int tp_vers; / TP microcode version,
    pub /: *mut *mut unsigned int er_vers; / expansion ROM version,
    pub /: *mut *mut unsigned int scfg_vers; / Serial Configuration version,
    pub /: *mut *mut unsigned int vpd_vers; / VPD Version,
    pub api_vers: [u8; 7],
    pub mtus: [c_ushort; NMTUS],
    pub a_wnd: [c_ushort; NCCTRL_WIN],
    pub b_wnd: [c_ushort; NCCTRL_WIN],
    pub /: *mut *mut unsigned char nports; / # of ethernet ports,
    pub portvec: c_uchar,
    pub /: *mut *mut chip_type chip; / chip code,
    pub /: *mut *mut arch_specific_params arch; / chip specific params,
    pub offload: c_uchar,
    pub /: *mut *mut unsigned char crypto; / HW capability for crypto,
    pub /: *mut *mut unsigned char ethofld; / QoS support,
    pub bypass: c_uchar,
    pub hash_filter: c_uchar,
    pub ofldq_wr_cred: c_uint,
    pub /: *mut *mut bool ulptx_memwrite_dsgl; / use of T5 DSGL allowed,
    pub /: *mut *mut unsigned int nsched_cls; / number of traffic classes,
    pub /: *mut *mut unsigned int max_ordird_qp; / Max read depth per RDMA QP,
    pub /: *mut *mut unsigned int max_ird_adapter; / Max read depth per adapter,
    pub /: *mut *mut bool fr_nsmr_tpte_wr_support; / FW support for FR_NSMR_TPTE_WR,
    pub /: *mut *mut u8 fw_caps_support; / 32-bit Port Capabilities,
    pub /: *mut *mut bool filter2_wr_support; / FW support for FILTER2_WR,
    pub /: *mut *mut unsigned int viid_smt_extn_support:1; / FW returns vin and smt index,
// MPS Buffer Group Map[per Port].  Bit i is set if buffer group i is
// used by the Port
//
    pub /: *mut *mut u8 mps_bg_map[MAX_NPORTS]; / MPS Buffer Group Map,
    pub /: *mut *mut bool write_w_imm_support; / FW supports WRITE_WITH_IMMEDIATE,
    pub /: *mut *mut bool write_cmpl_support; / FW supports WRITE_CMPL,
}

// State needed to monitor the forward progress of SGE Ingress DMA activities
// and possible hangs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_idma_monitor_state {
    pub /: *mut *mut unsigned int idma_1s_thresh; / 1s threshold in Core Clock ticks,
    pub /: *mut *mut unsigned int idma_stalled[2]; / synthesized stalled timers in HZ,
    pub /: *mut *mut unsigned int idma_state[2]; / IDMA Hang detect state,
    pub /: *mut *mut unsigned int idma_qid[2]; / IDMA Hung Ingress Queue ID,
    pub /: *mut *mut unsigned int idma_warn[2]; / time to warning in HZ,
}

// Firmware Mailbox Command/Reply log.  All values are in Host-Endian format.
// The access and execute times are signed in order to accommodate negative
// error returns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_cmd {
    pub /: *mut *mut u64 cmd[MBOX_LEN / 8]; / a Firmware Mailbox Command/Reply,
    pub /: *mut *mut u64 timestamp; / OS-dependent timestamp,
    pub /: *mut *mut u32 seqno; / sequence number,
    pub /: *mut *mut s16 access; / time (ms) to access mailbox,
    pub /: *mut *mut s16 execute; / time (ms) to execute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_cmd_log {
    pub /: *mut *mut unsigned int size; / number of entries in the log,
    pub /: *mut *mut unsigned int cursor; / next position in the log to write,
    pub /: *mut *mut u32 seqno; / next sequence number,
// variable length mailbox command log starts here
}

// Given a pointer to a Firmware Mailbox Command Log and a log entry index,
// return a pointer to the specified entry.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_ethtool_lb_test {
    pub completion: completion,
    pub result: c_int,
    pub loopback: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_info {
    pub chip: u8,
    pub fs_name: *mut c_char,
    pub fw_mod_name: *mut c_char,
    pub fw_hdr: fw_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_params {
    pub 4]: u32 data[TRACE_LEN /,
    pub 4]: u32 mask[TRACE_LEN /,
    pub snap_len: c_ushort,
    pub min_len: c_ushort,
    pub skip_ofst: c_uchar,
    pub skip_len: c_uchar,
    pub invert: c_uchar,
    pub port: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_fw_data {
    pub signature: __be32,
    pub reserved: [__u8; 4],
}

// Firmware Port Capabilities types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps {
    FW_CAPS_UNKNOWN	= 0,	/* 0'ed out initial state */
    FW_CAPS16	= 1,	/* old Firmware: 16-bit Port Capabilities */
    FW_CAPS32	= 2,	/* new Firmware: 32-bit Port Capabilities */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub /: *mut *mut fw_port_cap32_t pcaps; / link capabilities,
    pub /: *mut *mut fw_port_cap32_t def_acaps; / default advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t acaps; / advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t lpacaps; / peer advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t speed_caps; / speed(s) user has requested,
    pub /: *mut *mut unsigned int speed; / actual link speed (Mb/s),
    pub /: *mut *mut cc_pause requested_fc; / flow control user has requested,
    pub /: *mut *mut cc_pause fc; / actual link flow control,
    pub /: *mut *mut cc_pause advertised_fc; / actual advertised flow control,
    pub /: *mut *mut cc_fec requested_fec; / Forward Error Correction:,
    pub /: *mut *mut cc_fec fec; / requested and actual in use,
    pub /: *mut *mut unsigned char autoneg; / autonegotiating?,
    pub /: *mut *mut unsigned char link_ok; / link up?,
    pub /: *mut *mut unsigned char link_down_rc; / link down reason,
    pub /: *mut *mut bool new_module; / ->OS Transceiver Module inserted,
    pub /: *mut *mut bool redo_l1cfg; / ->CC redo current "sticky" L1 CFG,
}

// forwarded interrupts

pub const PRIV_FLAGS_ADAP: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_info {
    pub adapter: *mut adapter,
    pub viid: u16,
    pub /: *mut *mut int xact_addr_filt; / index of exact MAC address filter,
    pub /: *mut *mut u16 rss_size; / size of VI's RSS table slice,
    pub mdio_addr: i8,
    pub port_type: fw_port_type,
    pub mod_type: u8,
    pub port_id: u8,
    pub tx_chan: u8,
    pub /: *mut *mut u8 lport; / associated offload logical port,
    pub /: *mut *mut u8 nqsets; / # of qsets,
    pub /: *mut *mut u8 first_qset; / index of first qset,
    pub rss_mode: u8,
    pub link_cfg: link_config,
    pub rss: *mut u16,
    pub stats_base: port_stats,

    pub /: *mut *mut port_dcb_info dcb; / Data Center Bridging support,

    pub fcoe: cxgb_fcoe,

    pub /: *mut *mut bool rxtstamp; / Enable TS,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_enable: bool,
    pub sched_tbl: *mut sched_table,
    pub eth_flags: u32,
// viid and smt fields either returned by fw
// or decoded by parsing viid by driver.
//
    pub vin: u8,
    pub vivld: u8,
    pub smt_idx: u8,
    pub rx_cchan: u8,
    pub tc_block_shared: bool,
// Mirror VI information
    pub viid_mirror: u16,
    pub nmirrorqsets: u16,
    pub vi_mirror_count: u32,
    pub /: *mut *mut mutex vi_mirror_mutex; / Sync access to Mirror VI info,
    pub ethtool_lb: cxgb4_ethtool_lb_test,
}

pub const CXGB4_MIRROR_RXQ_DEFAULT_DESC_NUM: c_int = 1024;
pub const CXGB4_MIRROR_RXQ_DEFAULT_DESC_SIZE: c_int = 64;
pub const CXGB4_MIRROR_RXQ_DEFAULT_INTR_USEC: c_int = 5;
pub const CXGB4_MIRROR_RXQ_DEFAULT_PKT_CNT: c_int = 8;
pub const CXGB4_MIRROR_FLQ_DEFAULT_DESC_NUM: c_int = 72;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_fl {
    pub /: *mut *mut unsigned int avail; / # of available Rx buffers,
    pub /: *mut *mut unsigned int pend_cred; / new buffers since last FL DB ring,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned long alloc_failed; / # of times buffer allocation failed,
    pub large_alloc_failed: c_ulong,
    pub /: *mut *mut unsigned long mapping_err; / # of RX Buffer DMA Mapping failures,
    pub /: *mut *mut unsigned long low; / # of times momentarily starving,
    pub starving: c_ulong,
// RO fields
    pub /: *mut *mut unsigned int cntxt_id; / SGE context id for the free list,
    pub /: *mut *mut unsigned int size; / capacity of free list,
    pub /: *mut *mut *mut rx_sw_desc sdesc; / address of SW Rx descriptor ring,
    pub /: *mut *mut *mut __be64 desc; / address of HW Rx descriptor ring,
    pub /: *mut *mut dma_addr_t addr; / bus address of HW ring start,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
}

// A packet gather list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_gl {
    pub /: *mut *mut u64 sgetstamp; / SGE Time Stamp for Ingress Packet,
    pub frags: [page_frag; MAX_SKB_FRAGS],
    pub /: *mut *mut *mut void va; / virtual address of first byte,
    pub /: *mut *mut unsigned int nfrags; / # of fragments,
    pub /: *mut *mut unsigned int tot_len; / total length of fragments,
}

extern "C" {
    pub fn void(q: *mut *mut rspq_flush_handler_t)(struct sge_rspq) -> typedef;
}
// LRO related declarations for ULD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_lro_mgr {
pub const MAX_LRO_SESSIONS: c_int = 64;
    pub /: *mut *mut u8 lro_session_cnt; / # of sessions to aggregate,
    pub /: *mut *mut unsigned long lro_pkts; / # of LRO super packets,
    pub /: *mut *mut unsigned long lro_merged; / # of wire packets merged by LRO,
    pub /: *mut *mut sk_buff_head lroq; / list of aggregated sessions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_rspq {
    pub napi: napi_struct,
    pub /: *const *const *const __be64 cur_desc; / current descriptor in queue,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut u8 gen; / current generation bit,
    pub /: *mut *mut u8 intr_params; / interrupt holdoff parameters,
    pub /: *mut *mut u8 next_intr_params; / holdoff params for next interrupt,
    pub adaptive_rx: u8,
    pub /: *mut *mut u8 pktcnt_idx; / interrupt packet threshold,
    pub /: *mut *mut u8 uld; / ULD handling this queue,
    pub /: *mut *mut u8 idx; / queue index within its group,
    pub /: *mut *mut int offset; / offset into current Rx buffer,
    pub /: *mut *mut u16 cntxt_id; / SGE context id for the response q,
    pub /: *mut *mut u16 abs_id; / absolute SGE id for the response q,
    pub /: *mut *mut *mut __be64 desc; / address of HW response ring,
    pub /: *mut *mut dma_addr_t phys_addr; / physical address of the ring,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
    pub /: *mut *mut unsigned int iqe_len; / entry size,
    pub /: *mut *mut unsigned int size; / capacity of response queue,
    pub adap: *mut adapter,
    pub /: *mut *mut *mut net_device netdev; / associated net device,
    pub handler: rspq_handler_t,
    pub flush_handler: rspq_flush_handler_t,
    pub lro_mgr: t4_lro_mgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_stats {
    pub /: *mut *mut unsigned long pkts; / # of ethernet packets,
    pub /: *mut *mut unsigned long lro_pkts; / # of LRO super packets,
    pub /: *mut *mut unsigned long lro_merged; / # of wire packets merged by LRO,
    pub /: *mut *mut unsigned long rx_cso; / # of Rx checksum offloads,
    pub /: *mut *mut unsigned long vlan_ex; / # of Rx VLAN extractions,
    pub /: *mut *mut unsigned long rx_drops; / # of packets dropped due to no mem,
    pub /: *mut *mut unsigned long bad_rx_pkts; / # of packets with err_vec!=0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_rxq {
    pub rspq: sge_rspq,
    pub fl: sge_fl,
    pub stats: sge_eth_stats,
    pub msix: *mut msix_info,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_ofld_stats {
    pub /: *mut *mut unsigned long pkts; / # of packets,
    pub /: *mut *mut unsigned long imm; / # of immediate-data packets,
    pub /: *mut *mut unsigned long an; / # of asynchronous notifications,
    pub /: *mut *mut unsigned long nomem; / # of responses deferred due to no mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_ofld_rxq {
    pub rspq: sge_rspq,
    pub fl: sge_fl,
    pub stats: sge_ofld_stats,
    pub msix: *mut msix_info,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
    pub flit: [__be64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_sw_desc {
    pub /: *mut *mut *mut sk_buff skb; / SKB to free after getting completion,
    pub /: *mut *mut dma_addr_t addr[MAX_SKB_FRAGS + 1]; / DMA mapped addresses,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_txq {
    pub /: *mut *mut unsigned int in_use; / # of in-use Tx descriptors,
    pub /: *mut *mut unsigned int q_type; / Q type Eth/Ctrl/Ofld,
    pub /: *mut *mut unsigned int size; / # of descriptors,
    pub /: *mut *mut unsigned int cidx; / SW consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned long stops; / # of times q has been stopped,
    pub /: *mut *mut unsigned long restarts; / # of queue restarts,
    pub /: *mut *mut unsigned int cntxt_id; / SGE context id for the Tx q,
    pub /: *mut *mut *mut tx_desc desc; / address of HW Tx descriptor ring,
    pub /: *mut *mut *mut tx_sw_desc sdesc; / address of SW Tx descriptor ring,
    pub /: *mut *mut *mut sge_qstat stat; / queue status entry,
    pub /: *mut *mut dma_addr_t phys_addr; / physical address of the ring,
    pub db_lock: spinlock_t,
    pub db_disabled: c_int,
    pub db_pidx: c_ushort,
    pub db_pidx_inc: c_ushort,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_txq {
    pub q: sge_txq,
    pub /: *mut *mut *mut netdev_queue txq; / associated netdev TX queue,

    pub /: *mut *mut u8 dcb_prio; / DCB Priority bound to queue,

    pub /: *mut *mut u8 dbqt; / SGE Doorbell Queue Timer in use,
    pub /: *mut *mut unsigned int dbqtimerix; / SGE Doorbell Queue Timer Index,
    pub /: *mut *mut unsigned long tso; / # of TSO requests,
    pub /: *mut *mut unsigned long uso; / # of USO requests,
    pub /: *mut *mut unsigned long tx_cso; / # of Tx checksum offloads,
    pub /: *mut *mut unsigned long vlan_ins; / # of Tx VLAN insertions,
    pub /: *mut *mut unsigned long mapping_err; / # of I/O MMU packet mapping errors,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_uld_txq {
    pub q: sge_txq,
    pub adap: *mut adapter,
    pub /: *mut *mut sk_buff_head sendq; / list of backpressured packets,
    pub /: *mut *mut tasklet_qresume_tsk; / restarts the queue,
    pub /: *mut *mut bool service_ofldq_running; / service_ofldq() is processing sendq,
    pub /: *mut *mut u8 full; / the Tx ring is full,
    pub /: *mut *mut unsigned long mapping_err; / # of I/O MMU packet mapping errors,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_ctrl_txq {
    pub q: sge_txq,
    pub adap: *mut adapter,
    pub /: *mut *mut sk_buff_head sendq; / list of backpressured packets,
    pub /: *mut *mut tasklet_qresume_tsk; / restarts the queue,
    pub /: *mut *mut u8 full; / the Tx ring is full,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_uld_rxq_info {
    pub /: *mut *mut char name[IFNAMSIZ]; / name of ULD driver,
    pub /: *mut *mut *mut sge_ofld_rxq uldrxq; / Rxq's for ULD,
    pub /: *mut *mut *mut u16 rspq_id; / response queue id's of rxq,
    pub /: *mut *mut u16 nrxq; / # of ingress uld queues,
    pub /: *mut *mut u16 nciq; / # of completion queues,
    pub /: *mut *mut u8 uld; / uld type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_uld_txq_info {
    pub /: *mut *mut *mut sge_uld_txq uldtxq; / Txq's for ULD,
    pub /: *mut *mut atomic_t users; / num users,
    pub /: *mut *mut u16 ntxq; / # of egress uld queues,
}

// struct to maintain ULD list to reallocate ULD resources on hotplug
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_uld_list {
    pub uld_info: cxgb4_uld_info,
    pub list_node: list_head,
    pub uld_type: cxgb4_uld,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sge_eosw_state {
    CXGB4_EO_STATE_CLOSED = 0, /* Not ready to accept traffic */
    CXGB4_EO_STATE_FLOWC_OPEN_SEND, /* Send FLOWC open request */
    CXGB4_EO_STATE_FLOWC_OPEN_REPLY, /* Waiting for FLOWC open reply */
    CXGB4_EO_STATE_ACTIVE, /* Ready to accept traffic */
    CXGB4_EO_STATE_FLOWC_CLOSE_SEND, /* Send FLOWC close request */
    CXGB4_EO_STATE_FLOWC_CLOSE_REPLY, /* Waiting for FLOWC close reply */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eosw_txq {
    pub /: *mut *mut spinlock_t lock; / Per queue lock to synchronize completions,
    pub /: *mut *mut sge_eosw_state state; / Current ETHOFLD State,
    pub /: *mut *mut *mut tx_sw_desc desc; / Descriptor ring to hold packets,
    pub /: *mut *mut u32 ndesc; / Number of descriptors,
    pub /: *mut *mut u32 pidx; / Current Producer Index,
    pub /: *mut *mut u32 last_pidx; / Last successfully transmitted Producer Index,
    pub /: *mut *mut u32 cidx; / Current Consumer Index,
    pub /: *mut *mut u32 last_cidx; / Last successfully reclaimed Consumer Index,
    pub /: *mut *mut u32 flowc_idx; / Descriptor containing a FLOWC request,
    pub /: *mut *mut u32 inuse; / Number of packets held in ring,
    pub /: *mut *mut u32 cred; / Current available credits,
    pub /: *mut *mut u32 ncompl; / # of completions posted,
    pub /: *mut *mut u32 last_compl; / # of credits consumed since last completion req,
    pub /: *mut *mut u32 eotid; / Index into EOTID table in software,
    pub /: *mut *mut u32 hwtid; / Hardware EOTID index,
    pub /: *mut *mut u32 hwqid; / Underlying hardware queue index,
    pub /: *mut *mut *mut net_device netdev; / Pointer to netdevice,
    pub /: *mut *mut tasklet_qresume_tsk; / Restarts the queue,
    pub /: *mut *mut completion completion; / completion for FLOWC rendezvous,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eohw_txq {
    pub /: *mut *mut spinlock_t lock; / Per queue lock,
    pub /: *mut *mut sge_txq q; / HW Txq,
    pub /: *mut *mut *mut adapter adap; / Backpointer to adapter,
    pub /: *mut *mut unsigned long tso; / # of TSO requests,
    pub /: *mut *mut unsigned long uso; / # of USO requests,
    pub /: *mut *mut unsigned long tx_cso; / # of Tx checksum offloads,
    pub /: *mut *mut unsigned long vlan_ins; / # of Tx VLAN insertions,
    pub /: *mut *mut unsigned long mapping_err; / # of I/O MMU packet mapping errors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge {
    pub ethtxq: [sge_eth_txq; MAX_ETH_QSETS],
    pub ptptxq: sge_eth_txq,
    pub ctrlq: [sge_ctrl_txq; MAX_CTRL_QUEUES],
    pub ethrxq: [sge_eth_rxq; MAX_ETH_QSETS],
    pub ____cacheline_aligned_in_smp: sge_rspq fw_evtq,
    pub uld_rxq_info: *mut sge_uld_rxq_info,
    pub uld_txq_info: *mut sge_uld_txq_info,
    pub ____cacheline_aligned_in_smp: sge_rspq intrq,
    pub intrq_lock: spinlock_t,
    pub eohw_txq: *mut sge_eohw_txq,
    pub eohw_rxq: *mut sge_ofld_rxq,
    pub mirror_rxq: [*mut sge_eth_rxq; NCHAN],
    pub /: *mut *mut u16 max_ethqsets; / # of available Ethernet queue sets,
    pub /: *mut *mut u16 ethqsets; / # of active Ethernet queue sets,
    pub /: *mut *mut u16 ethtxq_rover; / Tx queue to clean up next,
    pub /: *mut *mut u16 ofldqsets; / # of active ofld queue sets,
    pub /: *mut *mut u16 nqs_per_uld; / # of Rx queues per ULD,
    pub /: *mut *mut u16 eoqsets; / # of ETHOFLD queues,
    pub /: *mut *mut u16 mirrorqsets; / # of Mirror queues,
    pub timer_val: [u16; SGE_NTIMERS],
    pub counter_val: [u8; SGE_NCOUNTERS],
    pub dbqtimer_tick: u16,
    pub dbqtimer_val: [u16; SGE_NDBQTIMERS],
    pub /: *mut *mut u32 fl_pg_order; / large page allocation size,
    pub /: *mut *mut u32 stat_len; / length of status page at ring end,
    pub /: *mut *mut u32 pktshift; / padding between CPL & packet data,
    pub /: *mut *mut u32 fl_align; / response queue message alignment,
    pub /: *mut *mut u32 fl_starve_thres; / Free List starvation threshold,
    pub idma_monitor: sge_idma_monitor_state,
    pub egr_start: c_uint,
    pub egr_sz: c_uint,
    pub ingr_start: c_uint,
    pub ingr_sz: c_uint,
    pub /: *mut *mut *mut *mut void egr_map; / qid->queue egress queue map,
    pub /: *mut *mut *mut *mut sge_rspq ingr_map; / qid->queue ingress queue map,
    pub starving_fl: *mut c_ulong,
    pub txq_maperr: *mut c_ulong,
    pub blocked_fl: *mut c_ulong,
    pub /: *mut *mut timer_list rx_timer; / refills starving FLs,
    pub /: *mut *mut timer_list tx_timer; / checks Tx queues,
    pub /: *mut *mut int fwevtq_msix_idx; / Index to firmware event queue MSI-X info,
    pub /: *mut *mut int nd_msix_idx; / Index to non-data interrupts MSI-X info,
}

// T4 supports SRIOV on PF0-3 and T5 on PF0-7.  However, the Serial
// Configuration initialization for T5 only has SR-IOV functionality enabled
// on PF0-3 in order to simplify everything.
//
pub const NUM_OF_PF_WITH_SRIOV: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct doorbell_stats {
    pub db_drop: u32,
    pub db_empty: u32,
    pub db_full: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_mac_addr {
    pub list: list_head,
    pub addr: [u8; ETH_ALEN],
    pub iface_mac: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msix_bmap {
    pub msix_bmap: *mut c_ulong,
    pub mapsize: c_uint,
    pub /: *mut *mut spinlock_t lock; / lock for acquiring bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msix_info {
    pub vec: c_ushort,
    pub 10]: char desc[IFNAMSIZ +,
    pub idx: c_uint,
    pub aff_mask: cpumask_var_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_info {
    pub vf_mac_addr: [c_uchar; ETH_ALEN],
    pub tx_rate: c_uint,
    pub pf_set_mac: bool,
    pub vlan: u16,
    pub link_state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hma_data {
    pub flags: c_uchar,
    pub sgt: *mut sg_table,
    pub /: *mut *mut *mut dma_addr_t phy_addr; / physical address of the page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_list {
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_thermal {
    pub tzdev: *mut thermal_zone_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mps_entries_ref {
    pub list: list_head,
    pub addr: [u8; ETH_ALEN],
    pub mask: [u8; ETH_ALEN],
    pub idx: u16,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_ethtool_filter_info {
    pub /: *mut *mut *mut u32 loc_array; / Array holding the actual TIDs set to filters,
    pub /: *mut *mut *mut unsigned long bmap; / Bitmap for managing filters in use,
    pub /: *mut *mut u32 in_use; / # of filters in use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_ethtool_filter {
    pub /: *mut *mut u32 nentries; / Adapter wide number of supported filters,
    pub /: *mut *mut *mut cxgb4_ethtool_filter_info port; / Per port entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter {
    pub regs: *mut void __iomem,
    pub bar2: *mut void __iomem,
    pub t4_bar0: u32,
    pub pdev: *mut pci_dev,
    pub pdev_dev: *mut device,
    pub name: *const c_char,
    pub mbox: c_uint,
    pub pf: c_uint,
    pub flags: c_uint,
    pub adap_idx: c_uint,
    pub chip: chip_type,
    pub eth_flags: u32,
    pub msg_enable: c_int,
    pub vxlan_port: __be16,
    pub geneve_port: __be16,
    pub params: adapter_params,
    pub vres: cxgb4_virt_res,
    pub swintr: c_uint,
// MSI-X Info for NIC and OFLD queues
    pub msix_info: *mut msix_info,
    pub msix_bmap: msix_bmap,
    pub db_stats: doorbell_stats,
    pub sge: sge,
    pub port: [*mut net_device; MAX_NPORTS],
    pub /: *mut *mut u8 chan_map[NCHAN]; / channel -> port map,
    pub vfinfo: *mut vf_info,
    pub num_vfs: u8,
    pub filter_mode: u32,
    pub l2t_start: c_uint,
    pub l2t_end: c_uint,
    pub l2t: *mut l2t_data,
    pub clipt_start: c_uint,
    pub clipt_end: c_uint,
    pub clipt: *mut clip_tbl,
    pub rawf_start: c_uint,
    pub rawf_cnt: c_uint,
    pub smt: *mut smt_data,
    pub uld: *mut cxgb4_uld_info,
    pub uld_handle: [*mut c_void; CXGB4_ULD_MAX],
    pub num_uld: c_uint,
    pub num_ofld_uld: c_uint,
    pub list_node: list_head,
    pub rcu_node: list_head,
    pub /: *mut *mut list_head mac_hlist; / list of MAC addresses in MPS Hash,
    pub mps_ref: list_head,
    pub /: *mut *mut spinlock_t mps_ref_lock; / lock for syncing mps ref/def activities,
    pub iscsi_ppm: *mut c_void,
    pub tids: tid_info,
    pub tid_release_head: *mut c_void,
    pub tid_release_lock: spinlock_t,
    pub workq: *mut workqueue_struct,
    pub tid_release_task: work_struct,
    pub db_full_task: work_struct,
    pub db_drop_task: work_struct,
    pub fatal_err_notify_task: work_struct,
    pub tid_release_task_busy: bool,
// lock for mailbox cmd list
    pub mbox_lock: spinlock_t,
    pub mlist: mbox_list,
// support for mailbox command/reply logging
pub const T4_OS_LOG_MBOX_CMDS: c_int = 256;
    pub mbox_log: *mut mbox_cmd_log,
    pub uld_mutex: mutex,
    pub debugfs_root: *mut dentry,
    pub /: *mut *mut bool use_bd; / Use SGE Back Door intfc for reading SGE Contexts,
    pub is: *mut *mut bool trace_rss; / 1 implies that different RSS flit per filter,
// used per filter else if 0 default RSS flit is
// used for all 4 filters.
//
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub ptp_tx_skb: *mut sk_buff,
// ptp lock
    pub ptp_lock: spinlock_t,
    pub stats_lock: spinlock_t,
    pub ____cacheline_aligned_in_smp: spinlock_t win0_lock,
// TC u32 offload
    pub tc_u32: *mut cxgb4_tc_u32_table,
    pub chcr_ktls: chcr_ktls,
    pub chcr_stats: chcr_stats_debug,

    pub ch_ktls_stats: ch_ktls_stats_debug,

    pub ch_ipsec_stats: ch_ipsec_stats_debug,

// TC flower offload
    pub tc_flower_initialized: bool,
    pub flower_tbl: rhashtable,
    pub flower_ht_params: rhashtable_params,
    pub flower_stats_timer: timer_list,
    pub flower_stats_work: work_struct,
// HMA
    pub hma: hma_data,
    pub srq: *mut srq_data,
// Dump buffer for collecting logs in kdump kernel
    pub vmcoredd: vmcoredd_data,

    pub ch_thermal: ch_thermal,

// TC MQPRIO offload
    pub tc_mqprio: *mut cxgb4_tc_mqprio,
// TC MATCHALL classifier offload
    pub tc_matchall: *mut cxgb4_tc_matchall,
// Ethtool n-tuple
    pub ethtool_filters: *mut cxgb4_ethtool_filter,
// Ethtool Dump
// Must be last - ends in a flex-array member.
    pub eth_dump: ethtool_dump,
}

// Support for "sched-class" command to allow a TX Scheduling Class to be
// programmed with various parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_sched_params {
    pub /: *mut *mut u8 type; / packet or flow,
    pub /: *mut *mut u8 level; / scheduler hierarchy level,
    pub /: *mut *mut u8 mode; / per-class or per-flow,
    pub /: *mut *mut u8 rateunit; / bit or packet rate,
    pub /: *mut *mut u8 ratemode; / %port relative or kbps absolute,
    pub /: *mut *mut u8 channel; / scheduler channel [0..N],
    pub /: *mut *mut u8 class; / scheduler class [0..N],
    pub /: *mut *mut u32 minrate; / minimum rate,
    pub /: *mut *mut u32 maxrate; / maximum rate,
    pub /: *mut *mut u16 weight; / percent weight,
    pub /: *mut *mut u16 pktsize; / average packet size,
    pub /: *mut *mut u16 burstsize; / burst buffer size,
    pub params: },
    pub u: },
}

// Support for "sched_queue" command to allow one or more NIC TX Queues
// to be bound to a TX Scheduling Class.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_sched_queue {
    pub /: *mut *mut s8 queue; / queue index,
    pub /: *mut *mut s8 class; / class index,
}

// Support for "sched_flowc" command to allow one or more FLOWC
// to be bound to a TX Scheduling Class.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_sched_flowc {
    pub /: *mut *mut s32 tid; / TID to bind,
    pub /: *mut *mut s8 class; / class index,
}

// Defined bit width of user definable filter tuples
//
pub const ETHTYPE_BITWIDTH: c_int = 16;
pub const FRAG_BITWIDTH: c_int = 1;
pub const MACIDX_BITWIDTH: c_int = 9;
pub const FCOE_BITWIDTH: c_int = 1;
pub const IPORT_BITWIDTH: c_int = 3;
pub const MATCHTYPE_BITWIDTH: c_int = 3;
pub const PROTO_BITWIDTH: c_int = 8;
pub const TOS_BITWIDTH: c_int = 8;
pub const PF_BITWIDTH: c_int = 8;
pub const VF_BITWIDTH: c_int = 8;
pub const IVLAN_BITWIDTH: c_int = 16;
pub const OVLAN_BITWIDTH: c_int = 16;
pub const ENCAP_VNI_BITWIDTH: c_int = 24;
// Filter matching rules.  These consist of a set of ingress packet field
// (value, mask) tuples.  The associated ingress packet field matches the
// tuple when ((field & mask) == value).  (Thus a wildcard "don't care" field
// rule can be constructed by specifying a tuple of (0, 0).)  A filter rule
// matches an ingress packet when all of the individual field
// matching rules are true.
//
// Partial field masks are always valid, however, while it may be easy to
// understand their meanings for some fields (e.g. IP address to match a
// subnet), for others making sensible partial masks is less intuitive (e.g.
// MPS match type) ...
//
// Most of the following data structures are modeled on T4 capabilities.
// Drivers for earlier chips use the subsets which make sense for those chips.
// We really need to come up with a hardware-independent mechanism to
// represent hardware filter capabilities ...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_filter_tuple {
// Compressed header matching field rules.  The TP_VLAN_PRI_MAP
// register selects which of these fields will participate in the
// filter match rules -- up to a maximum of 36 bits.  Because
// TP_VLAN_PRI_MAP is a global register, all filters must use the same
// set of fields.
//
    pub /: *mut *mut uint32_t ethtype:ETHTYPE_BITWIDTH; / Ethernet type,
    pub /: *mut *mut uint32_t frag:FRAG_BITWIDTH; / IP fragmentation header,
    pub /: *mut *mut uint32_t ivlan_vld:1; / inner VLAN valid,
    pub /: *mut *mut uint32_t ovlan_vld:1; / outer VLAN valid,
    pub /: *mut *mut uint32_t pfvf_vld:1; / PF/VF valid,
    pub /: *mut *mut uint32_t encap_vld:1; / Encapsulation valid,
    pub /: *mut *mut uint32_t macidx:MACIDX_BITWIDTH; / exact match MAC index,
    pub /: *mut *mut uint32_t fcoe:FCOE_BITWIDTH; / FCoE packet,
    pub /: *mut *mut uint32_t iport:IPORT_BITWIDTH; / ingress port,
    pub /: *mut *mut uint32_t matchtype:MATCHTYPE_BITWIDTH; / MPS match type,
    pub /: *mut *mut uint32_t proto:PROTO_BITWIDTH; / protocol type,
    pub /: *mut *mut uint32_t tos:TOS_BITWIDTH; / TOS/Traffic Type,
    pub /: *mut *mut uint32_t pf:PF_BITWIDTH; / PCI-E PF ID,
    pub /: *mut *mut uint32_t vf:VF_BITWIDTH; / PCI-E VF ID,
    pub /: *mut *mut uint32_t ivlan:IVLAN_BITWIDTH; / inner VLAN,
    pub /: *mut *mut uint32_t ovlan:OVLAN_BITWIDTH; / outer VLAN,
    pub /: *mut *mut uint32_t vni:ENCAP_VNI_BITWIDTH; / VNI of tunnel,
// Uncompressed header matching field rules.  These are always
// available for field rules.
//
    pub /: *mut *mut uint8_t lip[16]; / local IP address (IPv4 in [3:0]),
    pub /: *mut *mut uint8_t fip[16]; / foreign IP address (IPv4 in [3:0]),
    pub /: *mut *mut uint16_t lport; / local port,
    pub /: *mut *mut uint16_t fport; / foreign port,
}

// A filter ioctl command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_filter_specification {
// Administrative fields for filter.
//
    pub /: *mut *mut uint32_t hitcnts:1; / count filter hits in TCB,
    pub /: *mut *mut uint32_t prio:1; / filter has priority over active/server,
// Fundamental filter typing.  This is the one element of filter
// matching that doesn't exist as a (value, mask) tuple.
//
    pub /: *mut *mut uint32_t type:1; / 0 => IPv4, 1 => IPv6,
    pub /: *mut *mut u32 hash:1; / 0 => wild-card, 1 => exact-match,
// Packet dispatch information.  Ingress packets which match the
// filter rules will be dropped, passed to the host or switched back
// out as egress packets.
//
    pub /: *mut *mut uint32_t action:2; / drop, pass, switch,
    pub /: *mut *mut uint32_t rpttid:1; / report TID in RSS hash field,
    pub /: *mut *mut uint32_t dirsteer:1; / 0 => RSS, 1 => steer to iq,
    pub /: *mut *mut uint32_t iq:10; / ingress queue,
    pub /: *mut *mut uint32_t maskhash:1; / dirsteer=0: store RSS hash in TCB,
    pub /: *mut *mut uint32_t dirsteerhash:1;/ dirsteer=1: 0 => TCB contains RSS hash,
// 1 => TCB contains IQ ID
// Switch proxy/rewrite fields.  An ingress packet which matches a
// filter with "switch" set will be looped back out as an egress
// packet -- potentially with some Ethernet header rewriting.
//
    pub /: *mut *mut uint32_t eport:2; / egress port to switch packet out,
    pub /: *mut *mut uint32_t newdmac:1; / rewrite destination MAC address,
    pub /: *mut *mut uint32_t newsmac:1; / rewrite source MAC address,
    pub /: *mut *mut uint32_t newvlan:2; / rewrite VLAN Tag,
    pub /: *mut *mut uint32_t nat_mode:3; / specify NAT operation mode,
    pub /: *mut *mut uint8_t dmac[ETH_ALEN]; / new destination MAC address,
    pub /: *mut *mut uint8_t smac[ETH_ALEN]; / new source MAC address,
    pub /: *mut *mut uint16_t vlan; / VLAN Tag to insert,
    pub /: *mut *mut u8 nat_lip[16]; / local IP to use after NAT'ing,
    pub /: *mut *mut u8 nat_fip[16]; / foreign IP to use after NAT'ing,
    pub /: *mut *mut u16 nat_lport; / local port to use after NAT'ing,
    pub /: *mut *mut u16 nat_fport; / foreign port to use after NAT'ing,
    pub /: *mut *mut u32 tc_prio; / TC's filter priority index,
    pub /: *mut *mut u64 tc_cookie; / Unique cookie identifying TC rules,
// reservation for future additions
    pub rsvd: [u8; 12],
// Filter rule value/mask pairs.
//
    pub val: ch_filter_tuple,
    pub mask: ch_filter_tuple,
}

pub const CXGB4_FILTER_TYPE_MAX: c_int = 2;
// Host shadow copy of ingress filter entry.  This is in host native format
// and doesn't match the ordering or bit order, etc. of the hardware of the
// firmware command.  The use of bit-field structure elements is purely to
// remind ourselves of the field size limitations and save memory in the case
// where the filter table is large.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_entry {
// Administrative fields for filter.
    pub /: *mut *mut u32 valid:1; / filter allocated and valid,
    pub /: *mut *mut u32 locked:1; / filter is administratively locked,
    pub /: *mut *mut u32 pending:1; / filter action is pending firmware reply,
    pub /: *mut *mut *mut filter_ctx ctx; / Caller's completion hook,
    pub /: *mut *mut *mut l2t_entry l2t; / Layer Two Table entry for dmac,
    pub /: *mut *mut *mut smt_entry smt; / Source Mac Table entry for smac,
    pub /: *mut *mut *mut net_device dev; / Associated net device,
    pub /: *mut *mut u32 tid; / This will store the actual tid,
// The filter itself.  Most of this is a straight copy of information
// provided by the extended ioctl().  Some fields are translated to
// internal forms -- for instance the Ingress Queue ID passed in from
// the ioctl() is translated into the Absolute Ingress Queue ID.
//
    pub fs: ch_filter_specification,
}

extern "C" {
    pub fn readl(reg_addr: adap->regs +) -> return;
}

extern "C" {
    pub fn readl(32: addr) + ((u64)readl(addr + 4) <<) -> return;
}

extern "C" {
    pub fn readq(reg_addr: adap->regs +) -> return;
}
//
// t4_set_hw_addr - store a port's MAC address in SW
// @adapter: the adapter
// @port_idx: the port index
// @hw_addr: the Ethernet address
//
// Store the Ethernet address of the given port in SW.  Called by the common
// code when it retrieves a port's Ethernet address from EEPROM.
//
// netdev2pinfo - return the port_info structure associated with a net_device
// @dev: the netdev
//
// Return the struct port_info associated with a net_device
//
extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
//
// adap2pinfo - return the port_info of a port
// @adap: the adapter
// @idx: the port index
//
// Return the port_info structure for the port of the given index.
//
extern "C" {
    pub fn netdev_priv(_arg: adap->port[idx]) -> return;
}
//
// netdev2adap - return the adapter structure associated with a net_device
// @dev: the netdev
//
// Return the struct adapter associated with a net_device
//
// Return a version number to identify the type of adapter.  The scheme is:
// - bits 0..9: chip version
// - bits 10..15: chip revision
// - bits 16..23: register dump version
//
// Return a queue's interrupt hold-off time in us.  0 means no timer.
// driver name used for ethtool_drvinfo
extern "C" {
    pub fn t4_os_portmod_changed(adap: *mut adapter, port_id: c_int);
}
extern "C" {
    pub fn t4_os_link_changed(adap: *mut adapter, port_id: c_int, link_stat: c_int);
}
extern "C" {
    pub fn t4_free_sge_resources(adap: *mut adapter);
}
extern "C" {
    pub fn t4_intr_handler(adap: *mut adapter) -> irq_handler_t;
}
extern "C" {
    pub fn t4_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn cxgb4_selftest_lb_pkt(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn t4_mgmt_tx(adap: *mut adapter, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t4_ofld_send(adap: *mut adapter, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn t4_sge_free_ethofld_txq(adap: *mut adapter, txq: *mut sge_eohw_txq);
}
extern "C" {
    pub fn t4_sge_intr_msix(irq: c_int, cookie: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn t4_sge_init(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_sge_start(adap: *mut adapter);
}
extern "C" {
    pub fn t4_sge_stop(adap: *mut adapter);
}
extern "C" {
    pub fn cxgb4_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn cxgb4_write_rss(pi: *const port_info, queues: *const u16) -> c_int;
}
extern "C" {
    pub fn cxgb_encap_offload_supported(skb: *mut sk_buff) -> cpl_tx_tnl_lso_type;
}

// this should be set based upon device capabilities
// add Core Clock / 2 to round ticks to nearest uS
extern "C" {
    pub fn t4_wr_mbox_meat(_arg: adap, _arg: mbox, _arg: cmd, _arg: size, _arg: rpl, _arg: true) -> return;
}
extern "C" {
    pub fn t4_wr_mbox_meat(_arg: adap, _arg: mbox, _arg: cmd, _arg: size, _arg: rpl, _arg: false) -> return;
}
//
// hash_mac_addr - return the hash value of a MAC address
// @addr: the 48-bit Ethernet MAC address
//
// Hashes a MAC address according to the hash function used by HW inexact
// (hash) address matching.
//
// t4_is_inserted_mod_type - is a plugged in Firmware Module Type
// @fw_mod_type: the Firmware Mofule Type
//
// Return whether the Firmware Module Type represents a real Transceiver
// Module/Cable Module Type which has been inserted.
//
extern "C" {
    pub fn t4_hw_pci_read_cfg4(adapter: *mut adapter, reg: c_int, val: *mut u32);
}
extern "C" {
    pub fn t4_intr_enable(adapter: *mut adapter);
}
extern "C" {
    pub fn t4_intr_disable(adapter: *mut adapter);
}
extern "C" {
    pub fn t4_slow_intr_handler(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_wait_dev_ready(regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn t4_restart_aneg(adap: *mut adapter, mbox: c_uint, port: c_uint) -> c_int;
}
extern "C" {
    pub fn t4_read_pcie_cfg4(adap: *mut adapter, reg: c_int) -> u32;
}
extern "C" {
    pub fn t4_get_util_window(adap: *mut adapter) -> u32;
}
extern "C" {
    pub fn t4_setup_memwin(adap: *mut adapter, memwin_base: u32, window: u32);
}
extern "C" {
    pub fn t4_memory_update_win(adap: *mut adapter, win: c_int, addr: u32);
}
pub const T4_MEMORY_WRITE: c_int = 0;
pub const T4_MEMORY_READ: c_int = 1;
extern "C" {
    pub fn t4_memory_rw(_arg: adap, _arg: 0, _arg: mtype, _arg: addr, _arg: len, _arg: buf, _arg: 0) -> return;
}
extern "C" {
    pub fn t4_get_regs_len(adapter: *mut adapter) -> c_uint;
}
extern "C" {
    pub fn t4_get_regs(adap: *mut adapter, buf: *mut c_void, buf_size: usize);
}
extern "C" {
    pub fn t4_eeprom_ptov(phys_addr: c_uint, fn: c_uint, sz: c_uint) -> c_int;
}
extern "C" {
    pub fn t4_seeprom_wp(adapter: *mut adapter, enable: bool) -> c_int;
}
extern "C" {
    pub fn t4_get_raw_vpd_params(adapter: *mut adapter, p: *mut vpd_params) -> c_int;
}
extern "C" {
    pub fn t4_get_vpd_params(adapter: *mut adapter, p: *mut vpd_params) -> c_int;
}
extern "C" {
    pub fn t4_get_pfres(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_load_fw(adapter: *mut adapter, fw_data: *const u8, size: c_uint) -> c_int;
}
extern "C" {
    pub fn t4_phy_fw_ver(adap: *mut adapter, phy_fw_ver: *mut c_int) -> c_int;
}
extern "C" {
    pub fn t4_fwcache(adap: *mut adapter, op: fw_params_param_dev_fwcache) -> c_int;
}
extern "C" {
    pub fn t4_fl_pkt_align(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_flash_cfg_addr(adapter: *mut adapter) -> c_uint;
}
extern "C" {
    pub fn t4_check_fw_version(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_load_cfg(adapter: *mut adapter, cfg_data: *const u8, size: c_uint) -> c_int;
}
extern "C" {
    pub fn t4_get_fw_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_bs_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_tp_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_exprom_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_scfg_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_vpd_version(adapter: *mut adapter, vers: *mut u32) -> c_int;
}
extern "C" {
    pub fn t4_get_version_info(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_dump_version_info(adapter: *mut adapter);
}
extern "C" {
    pub fn t4_prep_adapter(adapter: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4_shutdown_adapter(adapter: *mut adapter) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t4_bar2_qtype {
    int t4_bar2_sge_qregs(struct adapter *adapter,
    unsigned int qid,
    enum t4_bar2_qtype qtype,
    int user,
    u64 *pbar2_qoffset,
    unsigned int *pbar2_qid);

    unsigned int qtimer_val(const struct adapter *adap,
    const struct sge_rspq *q);

    int t4_init_devlog_params(struct adapter *adapter);
    int t4_init_sge_params(struct adapter *adapter);
    int t4_init_tp_params(struct adapter *adap, bool sleep_ok);
    int t4_filter_field_shift(const struct adapter *adap, int filter_sel);
    int t4_init_rss_mode(struct adapter *adap, int mbox);
    int t4_init_portinfo(struct port_info *pi, int mbox,
    int port, int pf, int vf, u8 mac[]);
    int t4_port_init(struct adapter *adap, int mbox, int pf, int vf);
    int t4_init_port_mirror(struct port_info *pi, u8 mbox, u8 port, u8 pf, u8 vf,
    u16 *mirror_viid);
    void t4_fatal_err(struct adapter *adapter);
    unsigned int t4_chip_rss_size(struct adapter *adapter);
    int t4_config_rss_range(struct adapter *adapter, int mbox, unsigned int viid,
    int start, int n, const u16 *rspq, unsigned int nrspq);
    int t4_config_glbl_rss(struct adapter *adapter, int mbox, unsigned int mode,
    unsigned int flags);
    int t4_config_vi_rss(struct adapter *adapter, int mbox, unsigned int viid,
    unsigned int flags, unsigned int defq);
    int t4_read_rss(struct adapter *adapter, u16 *entries);
    void t4_read_rss_key(struct adapter *adapter, u32 *key, bool sleep_ok);
    void t4_write_rss_key(struct adapter *adap, const u32 *key, int idx,
    bool sleep_ok);
    void t4_read_rss_pf_config(struct adapter *adapter, unsigned int index,
    u32 *valp, bool sleep_ok);
    void t4_read_rss_vf_config(struct adapter *adapter, unsigned int index,
    u32 *vfl, u32 *vfh, bool sleep_ok);
    u32 t4_read_rss_pf_map(struct adapter *adapter, bool sleep_ok);
    u32 t4_read_rss_pf_mask(struct adapter *adapter, bool sleep_ok);

    unsigned int t4_get_mps_bg_map(struct adapter *adapter, int pidx);
    unsigned int t4_get_tp_ch_map(struct adapter *adapter, int pidx);
    void t4_pmtx_get_stats(struct adapter *adap, u32 cnt[], u64 cycles[]);
    void t4_pmrx_get_stats(struct adapter *adap, u32 cnt[], u64 cycles[]);
    int t4_read_cim_ibq(struct adapter *adap, unsigned int qid, u32 *data,
    size_t n);
    int t4_read_cim_obq(struct adapter *adap, unsigned int qid, u32 *data,
    size_t n);
    int t4_cim_read(struct adapter *adap, unsigned int addr, unsigned int n,
    unsigned int *valp);
    int t4_cim_write(struct adapter *adap, unsigned int addr, unsigned int n,
    const unsigned int *valp);
    int t4_cim_read_la(struct adapter *adap, u32 *la_buf, unsigned int *wrptr);
    void t4_cim_read_pif_la(struct adapter *adap, u32 *pif_req, u32 *pif_rsp,
    unsigned int *pif_req_wrptr,
    unsigned int *pif_rsp_wrptr);
    void t4_cim_read_ma_la(struct adapter *adap, u32 *ma_req, u32 *ma_rsp);
    void t4_read_cimq_cfg(struct adapter *adap, u16 *base, u16 *size, u16 *thres);
    const char *t4_get_port_type_description(enum fw_port_type port_type);
    void t4_get_port_stats(struct adapter *adap, int idx, struct port_stats *p);
    void t4_get_port_stats_offset(struct adapter *adap, int idx,
    struct port_stats *stats,
    struct port_stats *offset);
    void t4_get_lb_stats(struct adapter *adap, int idx, struct lb_port_stats *p);
    void t4_read_mtu_tbl(struct adapter *adap, u16 *mtus, u8 *mtu_log);
    void t4_read_cong_tbl(struct adapter *adap, u16 incr[NMTUS][NCCTRL_WIN]);
    void t4_tp_wr_bits_indirect(struct adapter *adap, unsigned int addr,
    unsigned int mask, unsigned int val);
    void t4_tp_read_la(struct adapter *adap, u64 *la_buf, unsigned int *wrptr);
    void t4_tp_get_err_stats(struct adapter *adap, struct tp_err_stats *st,
    bool sleep_ok);
    void t4_tp_get_cpl_stats(struct adapter *adap, struct tp_cpl_stats *st,
    bool sleep_ok);
    void t4_tp_get_rdma_stats(struct adapter *adap, struct tp_rdma_stats *st,
    bool sleep_ok);
    void t4_get_usm_stats(struct adapter *adap, struct tp_usm_stats *st,
    bool sleep_ok);
    void t4_tp_get_tcp_stats(struct adapter *adap, struct tp_tcp_stats *v4,
    struct tp_tcp_stats *v6, bool sleep_ok);
    void t4_get_fcoe_stats(struct adapter *adap, unsigned int idx,
    struct tp_fcoe_stats *st, bool sleep_ok);
    void t4_load_mtus(struct adapter *adap, const unsigned short *mtus,
    const unsigned short *alpha, const unsigned short *beta);

    void t4_ulprx_read_la(struct adapter *adap, u32 *la_buf);

    void t4_get_chan_txrate(struct adapter *adap, u64 *nic_rate, u64 *ofld_rate);
    void t4_mk_filtdelwr(unsigned int ftid, struct fw_filter_wr *wr, int qid);

    int t4_fw_hello(struct adapter *adap, unsigned int mbox, unsigned int evt_mbox,
    enum dev_master master, enum dev_state *state);
    int t4_fw_bye(struct adapter *adap, unsigned int mbox);
    int t4_early_init(struct adapter *adap, unsigned int mbox);
    int t4_fw_reset(struct adapter *adap, unsigned int mbox, int reset);
    int t4_fixup_host_params(struct adapter *adap, unsigned int page_size,
    unsigned int cache_line_size);
    int t4_fw_initialize(struct adapter *adap, unsigned int mbox);
    int t4_query_params(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int nparams, const u32 *params,
    u32 *val);
    int t4_query_params_ns(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int nparams, const u32 *params,
    u32 *val);
    int t4_query_params_rw(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int nparams, const u32 *params,
    u32 *val, int rw, bool sleep_ok);
    int t4_set_params_timeout(struct adapter *adap, unsigned int mbox,
    unsigned int pf, unsigned int vf,
    unsigned int nparams, const u32 *params,
    const u32 *val, int timeout);
    int t4_set_params(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int nparams, const u32 *params,
    const u32 *val);
    int t4_cfg_pfvf(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int txq, unsigned int txq_eth_ctrl,
    unsigned int rxqi, unsigned int rxq, unsigned int tc,
    unsigned int vi, unsigned int cmask, unsigned int pmask,
    unsigned int nexact, unsigned int rcaps, unsigned int wxcaps);
    int t4_alloc_vi(struct adapter *adap, unsigned int mbox, unsigned int port,
    unsigned int pf, unsigned int vf, unsigned int nmac, u8 *mac,
    unsigned int *rss_size, u8 *vivld, u8 *vin);
    int t4_free_vi(struct adapter *adap, unsigned int mbox,
    unsigned int pf, unsigned int vf,
    unsigned int viid);
    int t4_set_rxmode(struct adapter *adap, unsigned int mbox, unsigned int viid,
    unsigned int viid_mirror, int mtu, int promisc, int all_multi,
    int bcast, int vlanex, bool sleep_ok);
    int t4_free_raw_mac_filt(struct adapter *adap, unsigned int viid,
    const u8 *addr, const u8 *mask, unsigned int idx,
    u8 lookup_type, u8 port_id, bool sleep_ok);
    int t4_free_encap_mac_filt(struct adapter *adap, unsigned int viid, int idx,
    bool sleep_ok);
    int t4_alloc_encap_mac_filt(struct adapter *adap, unsigned int viid,
    const u8 *addr, const u8 *mask, unsigned int vni,
    unsigned int vni_mask, u8 dip_hit, u8 lookup_type,
    bool sleep_ok);
    int t4_alloc_raw_mac_filt(struct adapter *adap, unsigned int viid,
    const u8 *addr, const u8 *mask, unsigned int idx,
    u8 lookup_type, u8 port_id, bool sleep_ok);
    int t4_alloc_mac_filt(struct adapter *adap, unsigned int mbox,
    unsigned int viid, bool free, unsigned int naddr,
    const u8 **addr, u16 *idx, u64 *hash, bool sleep_ok);
    int t4_free_mac_filt(struct adapter *adap, unsigned int mbox,
    unsigned int viid, unsigned int naddr,
    const u8 **addr, bool sleep_ok);
    int t4_change_mac(struct adapter *adap, unsigned int mbox, unsigned int viid,
    int idx, const u8 *addr, bool persist, u8 *smt_idx);
    int t4_set_addr_hash(struct adapter *adap, unsigned int mbox, unsigned int viid,
    bool ucast, u64 vec, bool sleep_ok);
    int t4_enable_vi_params(struct adapter *adap, unsigned int mbox,
    unsigned int viid, bool rx_en, bool tx_en, bool dcb_en);
    int t4_enable_pi_params(struct adapter *adap, unsigned int mbox,
    struct port_info *pi,
    bool rx_en, bool tx_en, bool dcb_en);
    int t4_enable_vi(struct adapter *adap, unsigned int mbox, unsigned int viid,
    bool rx_en, bool tx_en);
    int t4_identify_port(struct adapter *adap, unsigned int mbox, unsigned int viid,
    unsigned int nblinks);
    int t4_mdio_rd(struct adapter *adap, unsigned int mbox, unsigned int phy_addr,
    unsigned int mmd, unsigned int reg, u16 *valp);
    int t4_mdio_wr(struct adapter *adap, unsigned int mbox, unsigned int phy_addr,
    unsigned int mmd, unsigned int reg, u16 val);
    int t4_iq_stop(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int iqtype, unsigned int iqid,
    unsigned int fl0id, unsigned int fl1id);
    int t4_iq_free(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int iqtype, unsigned int iqid,
    unsigned int fl0id, unsigned int fl1id);
    int t4_eth_eq_free(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int eqid);
    int t4_ctrl_eq_free(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int eqid);
    int t4_ofld_eq_free(struct adapter *adap, unsigned int mbox, unsigned int pf,
    unsigned int vf, unsigned int eqid);
    int t4_sge_ctxt_flush(struct adapter *adap, unsigned int mbox, int ctxt_type);
    int t4_read_sge_dbqtimers(struct adapter *adap, unsigned int ndbqtimers,
    u16 *dbqtimers);
    void t4_handle_get_port_info(struct port_info *pi, const __be64 *rpl);
    int t4_update_port_info(struct port_info *pi);
    int t4_get_link_params(struct port_info *pi, unsigned int *link_okp,
    unsigned int *speedp, unsigned int *mtup);
    int t4_handle_fw_rpl(struct adapter *adap, const __be64 *rpl);
    void t4_db_full(struct adapter *adapter);
    void t4_db_dropped(struct adapter *adapter);
    int t4_set_trace_filter(struct adapter *adapter, const struct trace_params *tp,
    int filter_index, int enable);
    void t4_get_trace_filter(struct adapter *adapter, struct trace_params *tp,
    int filter_index, int *enabled);
    int t4_fwaddrspace_write(struct adapter *adap, unsigned int mbox,
    u32 addr, u32 val);
    void t4_read_pace_tbl(struct adapter *adap, unsigned int pace_vals[NTX_SCHED]);
    void t4_get_tx_sched(struct adapter *adap, unsigned int sched,
    unsigned int *kbps, unsigned int *ipg, bool sleep_ok);
    int t4_sge_ctxt_rd(struct adapter *adap, unsigned int mbox, unsigned int cid,
    enum ctxt_type ctype, u32 *data);
    int t4_sge_ctxt_rd_bd(struct adapter *adap, unsigned int cid,
    enum ctxt_type ctype, u32 *data);
    int t4_sched_params(struct adapter *adapter, u8 type, u8 level, u8 mode,
    u8 rateunit, u8 ratemode, u8 channel, u8 class,
    u32 minrate, u32 maxrate, u16 weight, u16 pktsize,
    u16 burstsize);
    void t4_sge_decode_idma_state(struct adapter *adapter, int state);
    void t4_idma_monitor_init(struct adapter *adapter,
    struct sge_idma_monitor_state *idma);
    void t4_idma_monitor(struct adapter *adapter,
    struct sge_idma_monitor_state *idma,
    int hz, int ticks);
    int t4_set_vf_mac_acl(struct adapter *adapter, unsigned int vf,
    u8 start, unsigned int naddr, u8 *addr);
    void t4_tp_pio_read(struct adapter *adap, u32 *buff, u32 nregs,
    u32 start_index, bool sleep_ok);
    void t4_tp_tm_pio_read(struct adapter *adap, u32 *buff, u32 nregs,
    u32 start_index, bool sleep_ok);
    void t4_tp_mib_read(struct adapter *adap, u32 *buff, u32 nregs,
    u32 start_index, bool sleep_ok);

    void t4_uld_mem_free(struct adapter *adap);
    int t4_uld_mem_alloc(struct adapter *adap);
    void t4_uld_clean_up(struct adapter *adap);
    void t4_register_netevent_notifier(void);
    int t4_i2c_rd(struct adapter *adap, unsigned int mbox, int port,
    unsigned int devid, unsigned int offset,
    unsigned int len, u8 *buf);
    int t4_load_boot(struct adapter *adap, u8 *boot_data,
    unsigned int boot_addr, unsigned int size);
    int t4_load_bootcfg(struct adapter *adap,
    const u8 *cfg_data, unsigned int size);
    void free_rspq_fl(struct adapter *adap, struct sge_rspq *rq, struct sge_fl *fl);
    void free_tx_desc(struct adapter *adap, struct sge_txq *q,
    unsigned int n, bool unmap);
    void cxgb4_eosw_txq_free_desc(struct adapter *adap, struct sge_eosw_txq *txq,
    u32 ndesc);
    int cxgb4_ethofld_send_flowc(struct net_device *dev, u32 eotid, u32 tc);
    void cxgb4_ethofld_restart(struct tasklet_struct *t);
    int cxgb4_ethofld_rx_handler(struct sge_rspq *q, const __be64 *rsp,
    const struct pkt_gl *si);
    void free_txq(struct adapter *adap, struct sge_txq *q);
    void cxgb4_reclaim_completed_tx(struct adapter *adap,
    struct sge_txq *q, bool unmap);
    int cxgb4_map_skb(struct device *dev, const struct sk_buff *skb,
    dma_addr_t *addr);
    void cxgb4_inline_tx_skb(const struct sk_buff *skb, const struct sge_txq *q,
    void *pos);
    void cxgb4_write_sgl(const struct sk_buff *skb, struct sge_txq *q,
    struct ulptx_sgl *sgl, u64 *end, unsigned int start,
    const dma_addr_t *addr);
    void cxgb4_write_partial_sgl(const struct sk_buff *skb, struct sge_txq *q,
    struct ulptx_sgl *sgl, u64 *end,
    const dma_addr_t *addr, u32 start, u32 send_len);
    void cxgb4_ring_tx_db(struct adapter *adap, struct sge_txq *q, int n);
    int t4_set_vlan_acl(struct adapter *adap, unsigned int mbox, unsigned int vf,
    u16 vlan);
    int cxgb4_dcb_enabled(const struct net_device *dev);

    int cxgb4_thermal_init(struct adapter *adap);
    int cxgb4_thermal_remove(struct adapter *adap);
    int cxgb4_set_msix_aff(struct adapter *adap, unsigned short vec,
    cpumask_var_t *aff_mask, int idx);
    void cxgb4_clear_msix_aff(unsigned short vec, cpumask_var_t aff_mask);

    int cxgb4_change_mac(struct port_info *pi, unsigned int viid,
    int *tcam_idx, const u8 *addr,
    bool persistent, u8 *smt_idx);

    int cxgb4_alloc_mac_filt(struct adapter *adap, unsigned int viid,
    bool free, unsigned int naddr,
    const u8 **addr, u16 *idx,
    u64 *hash, bool sleep_ok);
    int cxgb4_free_mac_filt(struct adapter *adap, unsigned int viid,
    unsigned int naddr, const u8 **addr, bool sleep_ok);
    int cxgb4_init_mps_ref_entries(struct adapter *adap);
    void cxgb4_free_mps_ref_entries(struct adapter *adap);
    int cxgb4_update_mac_filt(struct port_info *pi, unsigned int viid,
    int *tcam_idx, const u8 *addr,
    bool persistent, u8 *smt_idx);
    int cxgb4_get_msix_idx_from_bmap(struct adapter *adap);
    void cxgb4_free_msix_idx_in_bmap(struct adapter *adap, u32 msix_idx);
    void cxgb4_enable_rx(struct adapter *adap, struct sge_rspq *q);
    void cxgb4_quiesce_rx(struct sge_rspq *q);
    int cxgb4_port_mirror_alloc(struct net_device *dev);
    void cxgb4_port_mirror_free(struct net_device *dev);

    int cxgb4_set_ktls_feature(struct adapter *adap, bool enable);

