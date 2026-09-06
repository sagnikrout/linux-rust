//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/thunder/nic.h
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
// Copyright (C) 2015 Cavium, Inc.
//

// PCI device IDs
pub const PCI_DEVICE_ID_THUNDER_NIC_PF: c_uint = 0xA01E;
pub const PCI_DEVICE_ID_THUNDER_PASS1_NIC_VF: c_uint = 0x0011;
pub const PCI_DEVICE_ID_THUNDER_NIC_VF: c_uint = 0xA034;
pub const PCI_DEVICE_ID_THUNDER_BGX: c_uint = 0xA026;
// Subsystem device IDs
pub const PCI_SUBSYS_DEVID_88XX_NIC_PF: c_uint = 0xA11E;
pub const PCI_SUBSYS_DEVID_81XX_NIC_PF: c_uint = 0xA21E;
pub const PCI_SUBSYS_DEVID_83XX_NIC_PF: c_uint = 0xA31E;
pub const PCI_SUBSYS_DEVID_88XX_PASS1_NIC_VF: c_uint = 0xA11E;
pub const PCI_SUBSYS_DEVID_88XX_NIC_VF: c_uint = 0xA134;
pub const PCI_SUBSYS_DEVID_81XX_NIC_VF: c_uint = 0xA234;
pub const PCI_SUBSYS_DEVID_83XX_NIC_VF: c_uint = 0xA334;
// PCI BAR nos
pub const PCI_CFG_REG_BAR_NUM: c_int = 0;
pub const PCI_MSIX_REG_BAR_NUM: c_int = 4;
// NIC SRIOV VF count
pub const MAX_NUM_VFS_SUPPORTED: c_int = 128;
pub const DEFAULT_NUM_VF_ENABLED: c_int = 8;
pub const NIC_TNS_BYPASS_MODE: c_int = 0;
pub const NIC_TNS_MODE: c_int = 1;
// NIC priv flags

// Min/Max packet size
pub const NIC_HW_MIN_FRS: c_int = 64;

// Max pkinds
pub const NIC_MAX_PKIND: c_int = 16;
// Max when CPI_ALG is IP diffserv
pub const NIC_MAX_CPI_PER_LMAC: c_int = 64;
// NIC VF Interrupts
pub const NICVF_INTR_CQ: c_int = 0;
pub const NICVF_INTR_SQ: c_int = 1;
pub const NICVF_INTR_RBDR: c_int = 2;
pub const NICVF_INTR_PKT_DROP: c_int = 3;
pub const NICVF_INTR_TCP_TIMER: c_int = 4;
pub const NICVF_INTR_MBOX: c_int = 5;
pub const NICVF_INTR_QS_ERR: c_int = 6;
pub const NICVF_INTR_CQ_SHIFT: c_int = 0;
pub const NICVF_INTR_SQ_SHIFT: c_int = 8;
pub const NICVF_INTR_RBDR_SHIFT: c_int = 16;
pub const NICVF_INTR_PKT_DROP_SHIFT: c_int = 20;
pub const NICVF_INTR_TCP_TIMER_SHIFT: c_int = 21;
pub const NICVF_INTR_MBOX_SHIFT: c_int = 22;
pub const NICVF_INTR_QS_ERR_SHIFT: c_int = 23;

// MSI-X interrupts
pub const NIC_PF_MSIX_VECTORS: c_int = 10;
pub const NIC_VF_MSIX_VECTORS: c_int = 20;
pub const NIC_PF_INTR_ID_ECC0_SBE: c_int = 0;
pub const NIC_PF_INTR_ID_ECC0_DBE: c_int = 1;
pub const NIC_PF_INTR_ID_ECC1_SBE: c_int = 2;
pub const NIC_PF_INTR_ID_ECC1_DBE: c_int = 3;
pub const NIC_PF_INTR_ID_ECC2_SBE: c_int = 4;
pub const NIC_PF_INTR_ID_ECC2_DBE: c_int = 5;
pub const NIC_PF_INTR_ID_ECC3_SBE: c_int = 6;
pub const NIC_PF_INTR_ID_ECC3_DBE: c_int = 7;
pub const NIC_PF_INTR_ID_MBOX0: c_int = 8;
pub const NIC_PF_INTR_ID_MBOX1: c_int = 9;
// Minimum FIFO level before all packets for the CQ are dropped
//
// This value ensures that once a packet has been "accepted"
// for reception it will not get dropped due to non-availability
// of CQ descriptor. An errata in HW mandates this value to be
// atleast 0x100.
//
pub const NICPF_CQM_MIN_DROP_LEVEL: c_uint = 0x100;
// Global timer for CQ timer thresh interrupts
// Calculated for SCLK of 700Mhz
// value written should be a 1/16th of what is expected
//
// 1 tick per 0.025usec
//
pub const NICPF_CLK_PER_INT_TICK: c_int = 1;
// Time to wait before we decide that a SQ is stuck.
//
// Since both pkt rx and tx notifications are done with same CQ,
// when packets are being received at very high rate (eg: L2 forwarding)
// then freeing transmitted skbs will be delayed and watchdog
// will kick in, resetting interface. Hence keeping this value high.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_cq_poll {
    pub nicvf: *mut nicvf,
    pub /: *mut *mut u8 cq_idx; / Completion queue index,
    pub napi: napi_struct,
}

pub const NIC_MAX_RSS_HASH_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_rss_info {
    pub enable: bool,

    pub cfg: u64,
    pub hash_bits: u8,
    pub rss_size: u16,
    pub ind_tbl: [u8; NIC_MAX_RSS_IDR_TBL_SIZE],
    pub key: [u64; RSS_HASH_KEY_SIZE],
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_pfc {
    pub autoneg: u8,
    pub fc_rx: u8,
    pub fc_tx: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_stats_reg_offset {
    RX_OCTS = 0x0,
    RX_UCAST = 0x1,
    RX_BCAST = 0x2,
    RX_MCAST = 0x3,
    RX_RED = 0x4,
    RX_RED_OCTS = 0x5,
    RX_ORUN = 0x6,
    RX_ORUN_OCTS = 0x7,
    RX_FCS = 0x8,
    RX_L2ERR = 0x9,
    RX_DRP_BCAST = 0xa,
    RX_DRP_MCAST = 0xb,
    RX_DRP_L3BCAST = 0xc,
    RX_DRP_L3MCAST = 0xd,
    RX_STATS_ENUM_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_stats_reg_offset {
    TX_OCTS = 0x0,
    TX_UCAST = 0x1,
    TX_BCAST = 0x2,
    TX_MCAST = 0x3,
    TX_DROP = 0x4,
    TX_STATS_ENUM_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_hw_stats {
    pub rx_bytes: u64,
    pub rx_frames: u64,
    pub rx_ucast_frames: u64,
    pub rx_bcast_frames: u64,
    pub rx_mcast_frames: u64,
    pub rx_drops: u64,
    pub rx_drop_red: u64,
    pub rx_drop_red_bytes: u64,
    pub rx_drop_overrun: u64,
    pub rx_drop_overrun_bytes: u64,
    pub rx_drop_bcast: u64,
    pub rx_drop_mcast: u64,
    pub rx_drop_l3_bcast: u64,
    pub rx_drop_l3_mcast: u64,
    pub rx_fcs_errors: u64,
    pub rx_l2_errors: u64,
    pub tx_bytes: u64,
    pub tx_frames: u64,
    pub tx_ucast_frames: u64,
    pub tx_bcast_frames: u64,
    pub tx_mcast_frames: u64,
    pub tx_drops: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_drv_stats {
// CQE Rx errs
    pub rx_bgx_truncated_pkts: u64,
    pub rx_jabber_errs: u64,
    pub rx_fcs_errs: u64,
    pub rx_bgx_errs: u64,
    pub rx_prel2_errs: u64,
    pub rx_l2_hdr_malformed: u64,
    pub rx_oversize: u64,
    pub rx_undersize: u64,
    pub rx_l2_len_mismatch: u64,
    pub rx_l2_pclp: u64,
    pub rx_ip_ver_errs: u64,
    pub rx_ip_csum_errs: u64,
    pub rx_ip_hdr_malformed: u64,
    pub rx_ip_payload_malformed: u64,
    pub rx_ip_ttl_errs: u64,
    pub rx_l3_pclp: u64,
    pub rx_l4_malformed: u64,
    pub rx_l4_csum_errs: u64,
    pub rx_udp_len_errs: u64,
    pub rx_l4_port_errs: u64,
    pub rx_tcp_flag_errs: u64,
    pub rx_tcp_offset_errs: u64,
    pub rx_l4_pclp: u64,
    pub rx_truncated_pkts: u64,
// CQE Tx errs
    pub tx_desc_fault: u64,
    pub tx_hdr_cons_err: u64,
    pub tx_subdesc_err: u64,
    pub tx_max_size_exceeded: u64,
    pub tx_imm_size_oflow: u64,
    pub tx_data_seq_err: u64,
    pub tx_mem_seq_err: u64,
    pub tx_lock_viol: u64,
    pub tx_data_fault: u64,
    pub tx_tstmp_conflict: u64,
    pub tx_tstmp_timeout: u64,
    pub tx_mem_fault: u64,
    pub tx_csum_overlap: u64,
    pub tx_csum_overflow: u64,
// driver debug stats
    pub tx_tso: u64,
    pub tx_timeout: u64,
    pub txq_stop: u64,
    pub txq_wake: u64,
    pub rcv_buffer_alloc_failures: u64,
    pub page_alloc: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcast_addr_list {
    pub count: c_int,
    pub mc: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_work {
    pub work: work_struct,
    pub mode: u8,
    pub mc: *mut xcast_addr_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf {
    pub pnicvf: *mut nicvf,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub reg_base: *mut void __iomem,
    pub xdp_prog: *mut bpf_prog,
pub const MAX_QUEUES_PER_QSET: c_int = 8;
    pub qs: *mut queue_set,
    pub iommu_domain: *mut c_void,
    pub vf_id: u8,
    pub sqs_id: u8,
    pub sqs_mode: bool,
    pub hw_tso: bool,
    pub t88: bool,
// Receive buffer alloc
    pub rb_page_offset: u32,
    pub rb_pageref: u16,
    pub rb_alloc_fail: bool,
    pub rb_work_scheduled: bool,
    pub rb_page: *mut page,
    pub rbdr_work: delayed_work,
    pub rbdr_task: tasklet_struct,
// Secondary Qset
    pub sqs_count: u8,
pub const MAX_SQS_PER_VF_SINGLE_NODE: c_int = 5;
pub const MAX_SQS_PER_VF: c_int = 11;
    pub snicvf: [*mut nicvf; MAX_SQS_PER_VF],
// Queue count
    pub rx_queues: u8,
    pub tx_queues: u8,
    pub xdp_tx_queues: u8,
    pub max_queues: u8,
    pub node: u8,
    pub cpi_alg: u8,
    pub link_up: bool,
    pub mac_type: u8,
    pub duplex: u8,
    pub speed: u32,
    pub tns_mode: bool,
    pub loopback_supported: bool,
    pub rss_info: nicvf_rss_info,
    pub pfc: nicvf_pfc,
    pub qs_err_task: tasklet_struct,
    pub reset_task: work_struct,
    pub rx_mode_work: nicvf_work,
// spinlock to protect workqueue arguments from concurrent access
    pub rx_mode_wq_lock: spinlock_t,
// workqueue for handling kernel ndo_set_rx_mode() calls
    pub nicvf_rx_mode_wq: *mut workqueue_struct,
// mutex to protect VF's mailbox contents from concurrent access
    pub rx_mode_mtx: mutex,
    pub link_change_work: delayed_work,
// PTP timestamp
    pub ptp_clock: *mut cavium_ptp,
// Inbound timestamping is on
    pub hw_rx_tstamp: bool,
// When the packet that requires timestamping is sent, hardware inserts
// two entries to the completion queue.  First is the regular
// CQE_TYPE_SEND entry that signals that the packet was sent.
// The second is CQE_TYPE_SEND_PTP that contains the actual timestamp
// for that packet.
// `ptp_skb` is initialized in the handler for the CQE_TYPE_SEND
// entry and is used and zeroed in the handler for the CQE_TYPE_SEND_PTP
// entry.
// So `ptp_skb` is used to hold the pointer to the packet between
// the calls to CQE_TYPE_SEND and CQE_TYPE_SEND_PTP handlers.
//
    pub ptp_skb: *mut sk_buff,
// `tx_ptp_skbs` is set when the hardware is sending a packet that
// requires timestamping.  Cavium hardware can not process more than one
// such packet at once so this is set each time the driver submits
// a packet that requires timestamping to the send queue and clears
// each time it receives the entry on the completion queue saying
// that such packet was sent.
// So `tx_ptp_skbs` prevents driver from submitting more than one
// packet that requires timestamping to the hardware for transmitting.
//
    pub tx_ptp_skbs: core::sync::atomic::AtomicI32,
// Interrupt coalescing settings
    pub cq_coalesce_usecs: u32,
    pub msg_enable: u32,
// Stats
    pub hw_stats: nicvf_hw_stats,
    pub drv_stats: *mut nicvf_drv_stats __percpu,
    pub bgx_stats: bgx_stats,
// Napi
    pub napi: [*mut nicvf_cq_poll; 8],
// MSI-X
    pub num_vec: u8,
    pub 15]: char irq_name[NIC_VF_MSIX_VECTORS][IFNAMSIZ +,
    pub irq_allocated: [bool; NIC_VF_MSIX_VECTORS],
    pub affinity_mask: [cpumask_var_t; NIC_VF_MSIX_VECTORS],
// VF <-> PF mailbox communication
    pub pf_acked: bool,
    pub pf_nacked: bool,
    pub set_mac_pending: bool,
    pub ____cacheline_aligned_in_smp: },
// PF <--> VF Mailbox communication
// Eight 64bit registers are shared between PF and VF.
// Separate set for each VF.
// Writing '1' into last register mbx7 means end of message.
//
// PF <--> VF mailbox communication
pub const NIC_PF_VF_MAILBOX_SIZE: c_int = 2;

// Mailbox message types
pub const NIC_MBOX_MSG_READY: c_uint = 0x01	/* Is PF ready to rcv msgs */;
pub const NIC_MBOX_MSG_ACK: c_uint = 0x02	/* ACK the message received */;
pub const NIC_MBOX_MSG_NACK: c_uint = 0x03	/* NACK the message received */;
pub const NIC_MBOX_MSG_QS_CFG: c_uint = 0x04	/* Configure Qset */;
pub const NIC_MBOX_MSG_RQ_CFG: c_uint = 0x05	/* Configure receive queue */;
pub const NIC_MBOX_MSG_SQ_CFG: c_uint = 0x06	/* Configure Send queue */;
pub const NIC_MBOX_MSG_RQ_DROP_CFG: c_uint = 0x07	/* Configure receive queue */;
pub const NIC_MBOX_MSG_SET_MAC: c_uint = 0x08	/* Add MAC ID to DMAC filter */;
pub const NIC_MBOX_MSG_SET_MAX_FRS: c_uint = 0x09	/* Set max frame size */;
pub const NIC_MBOX_MSG_CPI_CFG: c_uint = 0x0A	/* Config CPI, RSSI */;
pub const NIC_MBOX_MSG_RSS_SIZE: c_uint = 0x0B	/* Get RSS indir_tbl size */;
pub const NIC_MBOX_MSG_RSS_CFG: c_uint = 0x0C	/* Config RSS table */;
pub const NIC_MBOX_MSG_RSS_CFG_CONT: c_uint = 0x0D	/* RSS config continuation */;
pub const NIC_MBOX_MSG_RQ_BP_CFG: c_uint = 0x0E	/* RQ backpressure config */;
pub const NIC_MBOX_MSG_RQ_SW_SYNC: c_uint = 0x0F	/* Flush inflight pkts to RQ */;
pub const NIC_MBOX_MSG_BGX_STATS: c_uint = 0x10	/* Get stats from BGX */;
pub const NIC_MBOX_MSG_BGX_LINK_CHANGE: c_uint = 0x11	/* BGX:LMAC link status */;
pub const NIC_MBOX_MSG_ALLOC_SQS: c_uint = 0x12	/* Allocate secondary Qset */;
pub const NIC_MBOX_MSG_NICVF_PTR: c_uint = 0x13	/* Send nicvf ptr to PF */;
pub const NIC_MBOX_MSG_PNICVF_PTR: c_uint = 0x14	/* Get primary qset nicvf ptr */;
pub const NIC_MBOX_MSG_SNICVF_PTR: c_uint = 0x15	/* Send sqet nicvf ptr to PVF */;
pub const NIC_MBOX_MSG_LOOPBACK: c_uint = 0x16	/* Set interface in loopback */;
pub const NIC_MBOX_MSG_RESET_STAT_COUNTER: c_uint = 0x17	/* Reset statistics counters */;
pub const NIC_MBOX_MSG_PFC: c_uint = 0x18	/* Pause frame control */;
pub const NIC_MBOX_MSG_PTP_CFG: c_uint = 0x19	/* HW packet timestamp */;
pub const NIC_MBOX_MSG_CFG_DONE: c_uint = 0xF0	/* VF configuration done */;
pub const NIC_MBOX_MSG_SHUTDOWN: c_uint = 0xF1	/* VF is being shutdown */;
pub const NIC_MBOX_MSG_RESET_XCAST: c_uint = 0xF2    /* Reset DCAM filtering mode */;
pub const NIC_MBOX_MSG_ADD_MCAST: c_uint = 0xF3    /* Add MAC to DCAM filters */;
pub const NIC_MBOX_MSG_SET_XCAST: c_uint = 0xF4    /* Set MCAST/BCAST RX mode */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic_cfg_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub node_id: u8,
    pub tns_mode:1: u8,
    pub sqs_mode:1: u8,
    pub loopback_supported:1: u8,
    pub mac_addr: [u8; ETH_ALEN],
}

// Qset configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qs_cfg_msg {
    pub msg: u8,
    pub num: u8,
    pub sqs_count: u8,
    pub cfg: u64,
}

// Receive queue configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_cfg_msg {
    pub msg: u8,
    pub qs_num: u8,
    pub rq_num: u8,
    pub cfg: u64,
}

// Send queue configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_cfg_msg {
    pub msg: u8,
    pub qs_num: u8,
    pub sq_num: u8,
    pub sqs_mode: bool,
    pub cfg: u64,
}

// Set VF's MAC address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_mac_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub mac_addr: [u8; ETH_ALEN],
}

// Set Maximum frame size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_frs_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub max_frs: u16,
}

// Set CPI algorithm type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpi_cfg_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub rq_cnt: u8,
    pub cpi_alg: u8,
}

// Get RSS table size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_sz_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub ind_tbl_size: u16,
}

// Set RSS configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_cfg_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub hash_bits: u8,
    pub tbl_len: u8,
    pub tbl_offset: u8,
pub const RSS_IND_TBL_LEN_PER_MBX_MSG: c_int = 8;
    pub ind_tbl: [u8; RSS_IND_TBL_LEN_PER_MBX_MSG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgx_stats_msg {
    pub msg: u8,
    pub vf_id: u8,
    pub rx: u8,
    pub idx: u8,
    pub stats: u64,
}

// Physical interface link status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgx_link_status {
    pub msg: u8,
    pub mac_type: u8,
    pub link_up: u8,
    pub duplex: u8,
    pub speed: u32,
}

// Get Extra Qset IDs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sqs_alloc {
    pub msg: u8,
    pub vf_id: u8,
    pub qs_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nicvf_ptr {
    pub msg: u8,
    pub vf_id: u8,
    pub sqs_mode: bool,
    pub sqs_id: u8,
    pub nicvf: u64,
}

// Set interface in loopback mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_loopback {
    pub msg: u8,
    pub vf_id: u8,
    pub enable: bool,
}

// Reset statistics counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_stat_cfg {
    pub msg: u8,
// Bitmap to select NIC_PF_VNIC(vf_id)_RX_STAT(0..13)
    pub rx_stat_mask: u16,
// Bitmap to select NIC_PF_VNIC(vf_id)_TX_STAT(0..4)
    pub tx_stat_mask: u8,
// Bitmap to select NIC_PF_QS(0..127)_RQ(0..7)_STAT(0..1)
// bit14, bit15 NIC_PF_QS(vf_id)_RQ7_STAT(0..1)
// bit12, bit13 NIC_PF_QS(vf_id)_RQ6_STAT(0..1)
// ..
// bit2, bit3 NIC_PF_QS(vf_id)_RQ1_STAT(0..1)
// bit0, bit1 NIC_PF_QS(vf_id)_RQ0_STAT(0..1)
//
    pub rq_stat_mask: u16,
// Bitmap to select NIC_PF_QS(0..127)_SQ(0..7)_STAT(0..1)
// bit14, bit15 NIC_PF_QS(vf_id)_SQ7_STAT(0..1)
// bit12, bit13 NIC_PF_QS(vf_id)_SQ6_STAT(0..1)
// ..
// bit2, bit3 NIC_PF_QS(vf_id)_SQ1_STAT(0..1)
// bit0, bit1 NIC_PF_QS(vf_id)_SQ0_STAT(0..1)
//
    pub sq_stat_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfc {
    pub msg: u8,
    pub /: *mut *mut u8 get; / Get or set PFC settings,
    pub autoneg: u8,
    pub fc_rx: u8,
    pub fc_tx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_ptp {
    pub msg: u8,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcast {
    pub msg: u8,
    pub mode: u8,
    pub mac:48: u64,
}

// 128 bit shared memory between PF and each VF
#[repr(C)]
#[derive(Copy, Clone)]
pub union nic_mbx {
    pub msg: { u8 msg; },
    pub nic_cfg: nic_cfg_msg,
    pub qs: qs_cfg_msg,
    pub rq: rq_cfg_msg,
    pub sq: sq_cfg_msg,
    pub mac: set_mac_msg,
    pub frs: set_frs_msg,
    pub cpi_cfg: cpi_cfg_msg,
    pub rss_size: rss_sz_msg,
    pub rss_cfg: rss_cfg_msg,
    pub bgx_stats: bgx_stats_msg,
    pub link_status: bgx_link_status,
    pub sqs_alloc: sqs_alloc,
    pub nicvf: nicvf_ptr,
    pub lbk: set_loopback,
    pub reset_stat: reset_stat_cfg,
    pub pfc: pfc,
    pub ptp: set_ptp,
    pub xcast: xcast,
}

pub const NIC_NODE_ID_MASK: c_uint = 0x03;
pub const NIC_NODE_ID_SHIFT: c_int = 44;
extern "C" {
    pub fn nicvf_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn nicvf_stop(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn nicvf_send_msg_to_pf(vf: *mut nicvf, mbx: *mut nic_mbx) -> c_int;
}
extern "C" {
    pub fn nicvf_config_rss(nic: *mut nicvf);
}
extern "C" {
    pub fn nicvf_set_rss_key(nic: *mut nicvf);
}
extern "C" {
    pub fn nicvf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn nicvf_update_stats(nic: *mut nicvf);
}
extern "C" {
    pub fn nicvf_update_lmac_stats(nic: *mut nicvf);
}
