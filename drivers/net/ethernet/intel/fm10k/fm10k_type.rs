//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_type.h
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
// Copyright(c) 2013 - 2019 Intel Corporation.
// forward declaration

pub const FM10K_DEV_ID_PF: c_uint = 0x15A4;
pub const FM10K_DEV_ID_VF: c_uint = 0x15A5;
pub const FM10K_DEV_ID_SDI_FM10420_QDA2: c_uint = 0x15D0;
pub const FM10K_DEV_ID_SDI_FM10420_DA2: c_uint = 0x15D5;
pub const FM10K_MAX_QUEUES: c_int = 256;
pub const FM10K_MAX_QUEUES_PF: c_int = 128;
pub const FM10K_MAX_QUEUES_POOL: c_int = 16;
pub const FM10K_48_BIT_MASK: c_uint = 0x0000FFFFFFFFFFFFull;
pub const FM10K_STAT_VALID: c_uint = 0x80000000;
// PCI Bus Info
pub const FM10K_PCIE_LINK_CAP: c_uint = 0x7C;
pub const FM10K_PCIE_LINK_STATUS: c_uint = 0x82;
pub const FM10K_PCIE_LINK_WIDTH: c_uint = 0x3F0;
pub const FM10K_PCIE_LINK_WIDTH_1: c_uint = 0x10;
pub const FM10K_PCIE_LINK_WIDTH_2: c_uint = 0x20;
pub const FM10K_PCIE_LINK_WIDTH_4: c_uint = 0x40;
pub const FM10K_PCIE_LINK_WIDTH_8: c_uint = 0x80;
pub const FM10K_PCIE_LINK_SPEED: c_uint = 0xF;
pub const FM10K_PCIE_LINK_SPEED_2500: c_uint = 0x1;
pub const FM10K_PCIE_LINK_SPEED_5000: c_uint = 0x2;
pub const FM10K_PCIE_LINK_SPEED_8000: c_uint = 0x3;
// PCIe payload size
pub const FM10K_PCIE_DEV_CAP: c_uint = 0x74;
pub const FM10K_PCIE_DEV_CAP_PAYLOAD: c_uint = 0x07;
pub const FM10K_PCIE_DEV_CAP_PAYLOAD_128: c_uint = 0x00;
pub const FM10K_PCIE_DEV_CAP_PAYLOAD_256: c_uint = 0x01;
pub const FM10K_PCIE_DEV_CAP_PAYLOAD_512: c_uint = 0x02;
pub const FM10K_PCIE_DEV_CTRL: c_uint = 0x78;
pub const FM10K_PCIE_DEV_CTRL_PAYLOAD: c_uint = 0xE0;
pub const FM10K_PCIE_DEV_CTRL_PAYLOAD_128: c_uint = 0x00;
pub const FM10K_PCIE_DEV_CTRL_PAYLOAD_256: c_uint = 0x20;
pub const FM10K_PCIE_DEV_CTRL_PAYLOAD_512: c_uint = 0x40;
// PCIe MSI-X Capability info
pub const FM10K_PCI_MSIX_MSG_CTRL: c_uint = 0xB2;
pub const FM10K_PCI_MSIX_MSG_CTRL_TBL_SZ_MASK: c_uint = 0x7FF;
pub const FM10K_MAX_MSIX_VECTORS: c_int = 256;
pub const FM10K_MAX_VECTORS_PF: c_int = 256;
pub const FM10K_MAX_VECTORS_POOL: c_int = 32;
// PCIe SR-IOV Info
pub const FM10K_PCIE_SRIOV_CTRL: c_uint = 0x190;
pub const FM10K_PCIE_SRIOV_CTRL_VFARI: c_uint = 0x10;

pub const FM10K_NOT_IMPLEMENTED: c_uint = 0x7FFFFFFF;
// Start of PF registers
pub const FM10K_CTRL: c_uint = 0x0000;
pub const FM10K_CTRL_BAR4_ALLOWED: c_uint = 0x00000004;
pub const FM10K_CTRL_EXT: c_uint = 0x0001;
pub const FM10K_GCR: c_uint = 0x0003;
pub const FM10K_GCR_EXT: c_uint = 0x0005;
// Interrupt control registers
pub const FM10K_EICR: c_uint = 0x0006;
pub const FM10K_EICR_FAULT_MASK: c_uint = 0x0000003F;
pub const FM10K_EICR_MAILBOX: c_uint = 0x00000040;
pub const FM10K_EICR_SWITCHREADY: c_uint = 0x00000080;
pub const FM10K_EICR_SWITCHNOTREADY: c_uint = 0x00000100;
pub const FM10K_EICR_SWITCHINTERRUPT: c_uint = 0x00000200;
pub const FM10K_EICR_VFLR: c_uint = 0x00000800;
pub const FM10K_EICR_MAXHOLDTIME: c_uint = 0x00001000;
pub const FM10K_EIMR: c_uint = 0x0007;
pub const FM10K_EIMR_PCA_FAULT: c_uint = 0x00000001;
pub const FM10K_EIMR_THI_FAULT: c_uint = 0x00000010;
pub const FM10K_EIMR_FUM_FAULT: c_uint = 0x00000400;
pub const FM10K_EIMR_MAILBOX: c_uint = 0x00001000;
pub const FM10K_EIMR_SWITCHREADY: c_uint = 0x00004000;
pub const FM10K_EIMR_SWITCHNOTREADY: c_uint = 0x00010000;
pub const FM10K_EIMR_SWITCHINTERRUPT: c_uint = 0x00040000;
pub const FM10K_EIMR_SRAMERROR: c_uint = 0x00100000;
pub const FM10K_EIMR_VFLR: c_uint = 0x00400000;
pub const FM10K_EIMR_MAXHOLDTIME: c_uint = 0x01000000;
pub const FM10K_EIMR_ALL: c_uint = 0x55555555;

pub const FM10K_FAULT_ADDR_LO: c_uint = 0x0;
pub const FM10K_FAULT_ADDR_HI: c_uint = 0x1;
pub const FM10K_FAULT_SPECINFO: c_uint = 0x2;
pub const FM10K_FAULT_FUNC: c_uint = 0x3;
pub const FM10K_FAULT_SIZE: c_uint = 0x4;
pub const FM10K_FAULT_FUNC_VALID: c_uint = 0x00008000;
pub const FM10K_FAULT_FUNC_PF: c_uint = 0x00004000;
pub const FM10K_FAULT_FUNC_VF_MASK: c_uint = 0x00003F00;
pub const FM10K_FAULT_FUNC_VF_SHIFT: c_int = 8;
pub const FM10K_FAULT_FUNC_TYPE_MASK: c_uint = 0x000000FF;
pub const FM10K_PCA_FAULT: c_uint = 0x0008;
pub const FM10K_THI_FAULT: c_uint = 0x0010;
pub const FM10K_FUM_FAULT: c_uint = 0x001C;
// Rx queue timeout indicator

// Switch Manager info

// GLORT mapping registers

pub const FM10K_DGLORT_COUNT: c_int = 8;
pub const FM10K_DGLORTMAP_MASK_SHIFT: c_int = 16;
pub const FM10K_DGLORTMAP_ANY: c_uint = 0x00000000;
pub const FM10K_DGLORTMAP_NONE: c_uint = 0x0000FFFF;
pub const FM10K_DGLORTMAP_ZERO: c_uint = 0xFFFF0000;

pub const FM10K_DGLORTDEC_VSILENGTH_SHIFT: c_int = 4;
pub const FM10K_DGLORTDEC_VSIBASE_SHIFT: c_int = 7;
pub const FM10K_DGLORTDEC_PCLENGTH_SHIFT: c_int = 14;
pub const FM10K_DGLORTDEC_QBASE_SHIFT: c_int = 16;
pub const FM10K_DGLORTDEC_RSSLENGTH_SHIFT: c_int = 24;
pub const FM10K_DGLORTDEC_INNERRSS_ENABLE: c_uint = 0x08000000;
pub const FM10K_TUNNEL_CFG: c_uint = 0x0040;
pub const FM10K_TUNNEL_CFG_NVGRE_SHIFT: c_int = 16;
pub const FM10K_TUNNEL_CFG_GENEVE: c_uint = 0x0041;

pub const FM10K_SWPRI_MAX: c_int = 16;

pub const FM10K_RSSRK_SIZE: c_int = 10;
pub const FM10K_RSSRK_ENTRIES_PER_REG: c_int = 4;

pub const FM10K_RETA_SIZE: c_int = 32;
pub const FM10K_RETA_ENTRIES_PER_REG: c_int = 4;
pub const FM10K_MAX_RSS_INDICES: c_int = 128;
// Rate limiting registers

pub const FM10K_TC_CREDIT_CREDIT_MASK: c_uint = 0x001FFFFF;

pub const FM10K_TC_MAXCREDIT_64K: c_uint = 0x00010000;

pub const FM10K_TC_RATE_QUANTA_MASK: c_uint = 0x0000FFFF;
pub const FM10K_TC_RATE_INTERVAL_4US_GEN1: c_uint = 0x00020000;
pub const FM10K_TC_RATE_INTERVAL_4US_GEN2: c_uint = 0x00040000;
pub const FM10K_TC_RATE_INTERVAL_4US_GEN3: c_uint = 0x00080000;
// DMA control registers
pub const FM10K_DMA_CTRL: c_uint = 0x20C3;
pub const FM10K_DMA_CTRL_TX_ENABLE: c_uint = 0x00000001;
pub const FM10K_DMA_CTRL_TX_ACTIVE: c_uint = 0x00000008;
pub const FM10K_DMA_CTRL_RX_ENABLE: c_uint = 0x00000010;
pub const FM10K_DMA_CTRL_RX_ACTIVE: c_uint = 0x00000080;
pub const FM10K_DMA_CTRL_RX_DESC_SIZE: c_uint = 0x00000100;
pub const FM10K_DMA_CTRL_MINMSS_64: c_uint = 0x00008000;
pub const FM10K_DMA_CTRL_MAX_HOLD_1US_GEN3: c_uint = 0x04800000;
pub const FM10K_DMA_CTRL_MAX_HOLD_1US_GEN2: c_uint = 0x04000000;
pub const FM10K_DMA_CTRL_MAX_HOLD_1US_GEN1: c_uint = 0x03800000;
pub const FM10K_DMA_CTRL_DATAPATH_RESET: c_uint = 0x20000000;
pub const FM10K_DMA_CTRL_32_DESC: c_uint = 0x00000000;
pub const FM10K_DMA_CTRL2: c_uint = 0x20C4;
pub const FM10K_DMA_CTRL2_SWITCH_READY: c_uint = 0x00002000;
// TSO flags configuration
// First packet contains all flags except for fin and psh
// Middle packet contains only urg and ack
// Last packet contains urg, ack, fin, and psh
//
pub const FM10K_TSO_FLAGS_LOW: c_uint = 0x00300FF6;
pub const FM10K_TSO_FLAGS_HI: c_uint = 0x00000039;
pub const FM10K_DTXTCPFLGL: c_uint = 0x20C5;
pub const FM10K_DTXTCPFLGH: c_uint = 0x20C6;
pub const FM10K_TPH_CTRL: c_uint = 0x20C7;

pub const FM10K_MRQC_TCP_IPV4: c_uint = 0x00000001;
pub const FM10K_MRQC_IPV4: c_uint = 0x00000002;
pub const FM10K_MRQC_IPV6: c_uint = 0x00000010;
pub const FM10K_MRQC_TCP_IPV6: c_uint = 0x00000020;
pub const FM10K_MRQC_UDP_IPV4: c_uint = 0x00000040;
pub const FM10K_MRQC_UDP_IPV6: c_uint = 0x00000080;

pub const FM10K_TQMAP_TABLE_SIZE: c_int = 2048;

// Hardware Statistics
pub const FM10K_STATS_TIMEOUT: c_uint = 0x3800;
pub const FM10K_STATS_UR: c_uint = 0x3801;
pub const FM10K_STATS_CA: c_uint = 0x3802;
pub const FM10K_STATS_UM: c_uint = 0x3803;
pub const FM10K_STATS_XEC: c_uint = 0x3804;
pub const FM10K_STATS_VLAN_DROP: c_uint = 0x3805;
pub const FM10K_STATS_LOOPBACK_DROP: c_uint = 0x3806;
pub const FM10K_STATS_NODESC_DROP: c_uint = 0x3807;
// PCIe state registers
pub const FM10K_PHYADDR: c_uint = 0x381C;
// Rx ring registers

pub const FM10K_TPH_RXCTRL_DESC_TPHEN: c_uint = 0x00000020;
pub const FM10K_TPH_RXCTRL_DESC_RROEN: c_uint = 0x00000200;
pub const FM10K_TPH_RXCTRL_DATA_WROEN: c_uint = 0x00002000;
pub const FM10K_TPH_RXCTRL_HDR_WROEN: c_uint = 0x00008000;

pub const FM10K_RXQCTL_ENABLE: c_uint = 0x00000001;
pub const FM10K_RXQCTL_PF: c_uint = 0x000000FC;
pub const FM10K_RXQCTL_VF_SHIFT: c_int = 2;
pub const FM10K_RXQCTL_VF: c_uint = 0x00000100;

pub const FM10K_RXDCTL_WRITE_BACK_MIN_DELAY: c_uint = 0x00000001;
pub const FM10K_RXDCTL_DROP_ON_EMPTY: c_uint = 0x00000200;

pub const FM10K_SRRCTL_LOOPBACK_SUPPRESS: c_uint = 0x40000000;
pub const FM10K_SRRCTL_BUFFER_CHAINING_EN: c_uint = 0x80000000;
// Rx Statistics

// Rx GLORT register

// Tx ring registers

// When fist initialized, VFs need to know the Interrupt Throttle Rate (ITR)
// scale which is based on the PCIe speed but the speed information in the PCI
// configuration space may not be accurate. The PF already knows the ITR scale
// but there is no defined method to pass that information from the PF to the
// VF. This is accomplished during VF initialization by temporarily co-opting
// the yet-to-be-used TDLEN register to have the PF store the ITR shift for
// the VF to retrieve before the VF needs to use the TDLEN register for its
// intended purpose, i.e. before the Tx resources are allocated.
//
pub const FM10K_TDLEN_ITR_SCALE_SHIFT: c_int = 9;
pub const FM10K_TDLEN_ITR_SCALE_MASK: c_uint = 0x00000E00;
pub const FM10K_TDLEN_ITR_SCALE_GEN1: c_int = 2;
pub const FM10K_TDLEN_ITR_SCALE_GEN2: c_int = 1;
pub const FM10K_TDLEN_ITR_SCALE_GEN3: c_int = 0;

pub const FM10K_TPH_TXCTRL_DESC_TPHEN: c_uint = 0x00000020;
pub const FM10K_TPH_TXCTRL_DESC_RROEN: c_uint = 0x00000200;
pub const FM10K_TPH_TXCTRL_DESC_WROEN: c_uint = 0x00000800;
pub const FM10K_TPH_TXCTRL_DATA_RROEN: c_uint = 0x00002000;

pub const FM10K_TXDCTL_ENABLE: c_uint = 0x00004000;
pub const FM10K_TXDCTL_MAX_TIME_SHIFT: c_int = 16;

pub const FM10K_TXQCTL_PF: c_uint = 0x0000003F;
pub const FM10K_TXQCTL_VF: c_uint = 0x00000040;

pub const FM10K_TXQCTL_PC_SHIFT: c_int = 7;
pub const FM10K_TXQCTL_PC_MASK: c_uint = 0x00000380;
pub const FM10K_TXQCTL_TC_SHIFT: c_int = 10;
pub const FM10K_TXQCTL_VID_SHIFT: c_int = 16;
pub const FM10K_TXQCTL_VID_MASK: c_uint = 0x0FFF0000;
pub const FM10K_TXQCTL_UNLIMITED_BW: c_uint = 0x10000000;

// Tx Statistics

// Tx Push registers

pub const FM10K_TQDLOC_BASE_32_DESC: c_uint = 0x08;
pub const FM10K_TQDLOC_SIZE_32_DESC: c_uint = 0x00050000;
// Tx GLORT registers

pub const FM10K_PFVTCTL_FTAG_DESC_ENABLE: c_uint = 0x00000001;
// Interrupt moderation and control registers

pub const FM10K_INT_MAP_TIMER0: c_uint = 0x00000000;
pub const FM10K_INT_MAP_TIMER1: c_uint = 0x00000100;
pub const FM10K_INT_MAP_IMMEDIATE: c_uint = 0x00000200;
pub const FM10K_INT_MAP_DISABLE: c_uint = 0x00000300;

pub const FM10K_INT_CTRL: c_uint = 0x12000;
pub const FM10K_INT_CTRL_ENABLEMODERATOR: c_uint = 0x00000400;

pub const FM10K_ITR_INTERVAL1_SHIFT: c_int = 12;
pub const FM10K_ITR_PENDING2: c_uint = 0x10000000;
pub const FM10K_ITR_AUTOMASK: c_uint = 0x20000000;
pub const FM10K_ITR_MASK_SET: c_uint = 0x40000000;
pub const FM10K_ITR_MASK_CLEAR: c_uint = 0x80000000;

pub const FM10K_ITR_REG_COUNT: c_int = 768;
pub const FM10K_ITR_REG_COUNT_PF: c_int = 256;
// Switch manager interrupt registers
pub const FM10K_IP: c_uint = 0x13000;
pub const FM10K_IP_NOTINRESET: c_uint = 0x00000100;
// VLAN registers

pub const FM10K_VLAN_TABLE_SIZE: c_int = 128;
// VLAN specific message offsets
pub const FM10K_VLAN_TABLE_VID_MAX: c_int = 4096;
pub const FM10K_VLAN_TABLE_VSI_MAX: c_int = 64;
pub const FM10K_VLAN_LENGTH_SHIFT: c_int = 16;

// VF FLR event notification registers

// Defines for size of uncacheable memories
pub const FM10K_UC_ADDR_START: c_uint = 0x000000	/* start of standard regs */;
pub const FM10K_UC_ADDR_END: c_uint = 0x100000	/* end of standard regs */;

// Define timeouts for resets and disables
pub const FM10K_QUEUE_DISABLE_TIMEOUT: c_int = 100;
pub const FM10K_RESET_TIMEOUT: c_int = 150;
// Maximum supported combined inner and outer header length for encapsulation
pub const FM10K_TUNNEL_HEADER_LENGTH: c_int = 184;
// VF registers
pub const FM10K_VFCTRL: c_uint = 0x00000;
pub const FM10K_VFCTRL_RST: c_uint = 0x00000008;
pub const FM10K_VFINT_MAP: c_uint = 0x00030;
pub const FM10K_VFSYSTIME: c_uint = 0x00040;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_int_source {
    fm10k_int_mailbox		= 0,
    fm10k_int_pcie_fault		= 1,
    fm10k_int_switch_up_down	= 2,
    fm10k_int_switch_event		= 3,
    fm10k_int_sram			= 4,
    fm10k_int_vflr			= 5,
    fm10k_int_max_hold_time		= 6,
    fm10k_int_sources_max_pf
}

// PCIe bus speeds
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_bus_speed {
    fm10k_bus_speed_unknown	= 0,
    fm10k_bus_speed_2500	= 2500,
    fm10k_bus_speed_5000	= 5000,
    fm10k_bus_speed_8000	= 8000,
    fm10k_bus_speed_reserved
}

// PCIe bus widths
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_bus_width {
    fm10k_bus_width_unknown	= 0,
    fm10k_bus_width_pcie_x1	= 1,
    fm10k_bus_width_pcie_x2	= 2,
    fm10k_bus_width_pcie_x4	= 4,
    fm10k_bus_width_pcie_x8	= 8,
    fm10k_bus_width_reserved
}

// PCIe payload sizes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_bus_payload {
    fm10k_bus_payload_unknown = 0,
    fm10k_bus_payload_128	  = 1,
    fm10k_bus_payload_256	  = 2,
    fm10k_bus_payload_512	  = 3,
    fm10k_bus_payload_reserved
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_bus_info {
    pub speed: fm10k_bus_speed,
    pub width: fm10k_bus_width,
    pub payload: fm10k_bus_payload,
}

// Statistics related declarations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_hw_stat {
    pub count: u64,
    pub base_l: u32,
    pub base_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_hw_stats_q {
    pub tx_bytes: fm10k_hw_stat,
    pub tx_packets: fm10k_hw_stat,

    pub rx_bytes: fm10k_hw_stat,
    pub rx_packets: fm10k_hw_stat,

    pub rx_drops: fm10k_hw_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_hw_stats {
    pub timeout: fm10k_hw_stat,

    pub ur: fm10k_hw_stat,
    pub ca: fm10k_hw_stat,
    pub um: fm10k_hw_stat,
    pub xec: fm10k_hw_stat,
    pub vlan_drop: fm10k_hw_stat,
    pub loopback_drop: fm10k_hw_stat,
    pub nodesc_drop: fm10k_hw_stat,
    pub q: [fm10k_hw_stats_q; FM10K_MAX_QUEUES_PF],
}

// Establish DGLORT feature priority
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_dglortdec_idx {
    fm10k_dglort_default	= 0,
    fm10k_dglort_vf_rsvd0	= 1,
    fm10k_dglort_vf_rss	= 2,
    fm10k_dglort_pf_rsvd0	= 3,
    fm10k_dglort_pf_queue	= 4,
    fm10k_dglort_pf_vsi	= 5,
    fm10k_dglort_pf_rsvd1	= 6,
    fm10k_dglort_pf_rss	= 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_dglort_cfg {
    pub /: *mut *mut u16 glort; / GLORT base,
    pub /: *mut *mut u16 queue_b; / Base value for queue,
    pub /: *mut *mut u8 vsi_b; / Base value for VSI,
    pub /: *mut *mut u8 idx; / index of DGLORTDEC entry,
    pub /: *mut *mut u8 rss_l; / RSS indices,
    pub /: *mut *mut u8 pc_l; / Priority Class indices,
    pub /: *mut *mut u8 vsi_l; / Number of bits from GLORT used to determine VSI,
    pub /: *mut *mut u8 queue_l; / Number of bits from GLORT used to determine queue,
    pub /: *mut *mut u8 shared_l; / Ignored bits from GLORT resulting in shared VSI,
    pub /: *mut *mut u8 inner_rss; / Boolean value if inner header is used for RSS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_pca_fault {
    PCA_NO_FAULT,
    PCA_UNMAPPED_ADDR,
    PCA_BAD_QACCESS_PF,
    PCA_BAD_QACCESS_VF,
    PCA_MALICIOUS_REQ,
    PCA_POISONED_TLP,
    PCA_TLP_ABORT,
    __PCA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_thi_fault {
    THI_NO_FAULT,
    THI_MAL_DIS_Q_FAULT,
    __THI_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_fum_fault {
    FUM_NO_FAULT,
    FUM_UNMAPPED_ADDR,
    FUM_POISONED_TLP,
    FUM_BAD_VF_QACCESS,
    FUM_ADD_DECODE_ERR,
    FUM_RO_ERROR,
    FUM_QPRC_CRC_ERROR,
    FUM_CSR_TIMEOUT,
    FUM_INVALID_TYPE,
    FUM_INVALID_LENGTH,
    FUM_INVALID_BE,
    FUM_INVALID_ALIGN,
    __FUM_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_fault {
    pub /: *mut *mut u64 address; / Address at the time fault was detected,
    pub /: *mut *mut u32 specinfo; / Extra info on this fault (fault dependent),
    pub /: *mut *mut u8 type; / Fault value dependent on subunit,
    pub /: *mut *mut u8 func; / Function number of the fault,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mac_ops {
// basic bring-up and tear-down
    pub ): *mut *mut s32 (reset_hw)(struct fm10k_hw,
    pub ): *mut *mut s32 (init_hw)(struct fm10k_hw,
    pub ): *mut *mut s32 (start_hw)(struct fm10k_hw,
    pub ): *mut *mut s32 (stop_hw)(struct fm10k_hw,
    pub ): *mut *mut s32 (get_bus_info)(struct fm10k_hw,
    pub ): *mut *mut *mut s32 (get_host_state)(struct fm10k_hw , bool,
    pub ): *mut *mut s32 (request_lport_map)(struct fm10k_hw,
    pub bool): *mut *mut *mut s32 (update_vlan)(struct fm10k_hw , u32, u8,,
    pub ): *mut *mut s32 (read_mac_addr)(struct fm10k_hw,
    pub u8): u16, bool,,
    pub bool): *const *const *const *const s32 (update_mc_addr)(struct fm10k_hw , u16, u8 , u16,,
    pub u8): *mut *mut *mut s32 (update_xcast_mode)(struct fm10k_hw , u16,,
    pub ): *mut *mut void (update_int_moderator)(struct fm10k_hw,
    pub bool): *mut *mut *mut s32 (update_lport_state)(struct fm10k_hw , u16, u16,,
    pub ): *mut *mut *mut void (update_hw_stats)(struct fm10k_hw , struct fm10k_hw_stats,
    pub ): *mut *mut *mut void (rebind_hw_stats)(struct fm10k_hw , struct fm10k_hw_stats,
    pub ): *mut fm10k_dglort_cfg,
    pub u64): *mut *mut *mut void (set_dma_mask)(struct fm10k_hw ,,
    pub ): *mut *mut *mut s32 (get_fault)(struct fm10k_hw , int, struct fm10k_fault,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_mac_type {
    fm10k_mac_unknown = 0,
    fm10k_mac_pf,
    fm10k_mac_vf,
    fm10k_num_macs
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mac_info {
    pub ops: fm10k_mac_ops,
    pub type: fm10k_mac_type,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub default_vid: u16,
    pub max_msix_vectors: u16,
    pub max_queues: u16,
    pub vlan_override: bool,
    pub get_host_state: bool,
    pub tx_ready: bool,
    pub dglort_map: u32,
    pub itr_scale: u8,
    pub reset_while_pending: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_swapi_table_info {
    pub used: u32,
    pub avail: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_swapi_info {
    pub status: u32,
    pub mac: fm10k_swapi_table_info,
    pub nexthop: fm10k_swapi_table_info,
    pub ffu: fm10k_swapi_table_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_xcast_modes {
    FM10K_XCAST_MODE_ALLMULTI	= 0,
    FM10K_XCAST_MODE_MULTI		= 1,
    FM10K_XCAST_MODE_PROMISC	= 2,
    FM10K_XCAST_MODE_NONE		= 3,
    FM10K_XCAST_MODE_DISABLE	= 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_vf_info {
// mbx must be first field in struct unless all default IOV message
// handlers are redone as the assumption is that vf_info starts
// at the same offset as the mailbox
//
    pub /: *mut *mut fm10k_mbx_info mbx; / PF side of VF mailbox,
    pub stats: [fm10k_hw_stats_q; FM10K_MAX_QUEUES_POOL],
    pub /: *mut *mut int rate; / Tx BW cap as defined by OS,
    pub /: *mut *mut u16 glort; / resource tag for this VF,
    pub /: *mut *mut u16 sw_vid; / Switch API assigned VLAN,
    pub /: *mut *mut u16 pf_vid; / PF assigned Default VLAN,
    pub /: *mut *mut u8 mac[ETH_ALEN]; / PF Default MAC address,
    pub /: *mut *mut u8 vsi; / VSI identifier,
    pub /: *mut *mut u8 vf_idx; / which VF this is,
    pub modes: *mut *mut u8 vf_flags; / flags indicating what,
// are supported for the port
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_iov_ops {
// IOV related bring-up and tear-down
    pub u16): *mut *mut *mut s32 (assign_resources)(struct fm10k_hw , u16,,
    pub int): *mut *mut *mut s32 (configure_tc)(struct fm10k_hw , u16,,
    pub u16): *mut *mut *mut s32 (assign_int_moderator)(struct fm10k_hw ,,
    pub ): *mut fm10k_vf_info,
    pub ): *mut fm10k_vf_info,
    pub u8): *mut *mut *mut *mut s32 (set_lport)(struct fm10k_hw , struct fm10k_vf_info , u16,,
    pub ): *mut *mut *mut void (reset_lport)(struct fm10k_hw , struct fm10k_vf_info,
    pub u16): *mut *mut *mut *mut void (update_stats)(struct fm10k_hw , struct fm10k_hw_stats_q ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_iov_info {
    pub ops: fm10k_iov_ops,
    pub total_vfs: u16,
    pub num_vfs: u16,
    pub num_pools: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_devices {
    fm10k_device_pf,
    fm10k_device_vf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_info {
    pub mac: fm10k_mac_type,
    pub ): *mut *mut s32 (get_invariants)(struct fm10k_hw,
    pub mac_ops: *const fm10k_mac_ops,
    pub iov_ops: *const fm10k_iov_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_hw {
    pub hw_addr: *mut u32 __iomem,
    pub back: *mut c_void,
    pub mac: fm10k_mac_info,
    pub bus: fm10k_bus_info,
    pub bus_caps: fm10k_bus_info,
    pub iov: fm10k_iov_info,
    pub mbx: fm10k_mbx_info,
    pub swapi: fm10k_swapi_info,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
}

// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const FM10K_REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const FM10K_REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
// Transmit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's data buffer,
    pub /: *mut *mut __le16 buflen; / Length of data to be DMAed,
    pub /: *mut *mut __le16 vlan; / VLAN_ID and VPRI to be inserted in FTAG,
    pub /: *mut *mut __le16 mss; / MSS for segmentation offload,
    pub /: *mut *mut u8 hdrlen; / Header size for segmentation offload,
    pub /: *mut *mut u8 flags; / Status and offload request flags,
}

// Transmit Descriptor Cache Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_tx_desc_cache {
    pub tx_desc: [fm10k_tx_desc; 256],
}

pub const FM10K_TXD_FLAG_INT: c_uint = 0x01;
pub const FM10K_TXD_FLAG_TIME: c_uint = 0x02;
pub const FM10K_TXD_FLAG_CSUM: c_uint = 0x04;
pub const FM10K_TXD_FLAG_FTAG: c_uint = 0x10;
pub const FM10K_TXD_FLAG_RS: c_uint = 0x20;
pub const FM10K_TXD_FLAG_LAST: c_uint = 0x40;
pub const FM10K_TXD_FLAG_DONE: c_uint = 0x80;
// These macros are meant to enable optimal placement of the RS and INT
// bits.  It will point us to the last descriptor in the cache for either the
// start of the packet, or the end of the packet.  If the index is actually
// at the start of the FIFO it will point to the offset for the last index
// in the FIFO to prevent an unnecessary write.
//
pub const FM10K_TXD_WB_FIFO_SIZE: c_int = 4;
// Receive Descriptor - 32B
#[repr(C)]
#[derive(Copy, Clone)]
pub union fm10k_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub /: *mut *mut __le64 reserved; / Empty space, RSS hash,
    pub timestamp: __le64,
    pub /: *mut *mut } q; / Read, Writeback, 64b quad-words,
    pub /: *mut *mut __le32 data; / RSS and header data,
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub staterr: __le32,
    pub vlan_len: __le32,
    pub /: *mut *mut __le32 glort; / sglort/dglort,
    pub /: *mut *mut } d; / Writeback, 32b double-words,
    pub /: *mut *mut __le16 pkt_info; / RSS, Pkt type,
    pub /: *mut *mut __le16 hdr_info; / Splithdr, hdrlen, xC,
    pub rss_lower: __le16,
    pub rss_upper: __le16,
    pub /: *mut *mut __le16 status; / status/error,
    pub /: *mut *mut __le16 csum_err; / checksum or extended error value,
    pub /: *mut *mut __le16 length; / Packet length,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub dglort: __le16,
    pub sglort: __le16,
    pub /: *mut *mut } w; / Writeback, 16b words,
}

pub const FM10K_RXD_RSSTYPE_MASK: c_uint = 0x000F;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_rdesc_rss_type {
    FM10K_RSSTYPE_NONE	= 0x0,
    FM10K_RSSTYPE_IPV4_TCP	= 0x1,
    FM10K_RSSTYPE_IPV4	= 0x2,
    FM10K_RSSTYPE_IPV6_TCP	= 0x3,
// Reserved 0x4
    FM10K_RSSTYPE_IPV6	= 0x5,
// Reserved 0x6
    FM10K_RSSTYPE_IPV4_UDP	= 0x7,
    FM10K_RSSTYPE_IPV6_UDP	= 0x8
// Reserved 0x9 - 0xF
}

pub const FM10K_RXD_HDR_INFO_XC_MASK: c_uint = 0x0006;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_rxdesc_xc {
    FM10K_XC_UNICAST	= 0x0,
    FM10K_XC_MULTICAST	= 0x4,
    FM10K_XC_BROADCAST	= 0x6
}

pub const FM10K_RXD_STATUS_DD: c_uint = 0x0001 /* Descriptor done */;
pub const FM10K_RXD_STATUS_EOP: c_uint = 0x0002 /* End of packet */;
pub const FM10K_RXD_STATUS_L4CS: c_uint = 0x0010 /* Indicates an L4 csum */;
pub const FM10K_RXD_STATUS_L4CS2: c_uint = 0x0040 /* Inner header L4 csum */;
pub const FM10K_RXD_STATUS_L4E2: c_uint = 0x0800 /* Inner header L4 csum err */;
pub const FM10K_RXD_STATUS_IPE2: c_uint = 0x1000 /* Inner header IPv4 csum err */;
pub const FM10K_RXD_STATUS_RXE: c_uint = 0x2000 /* Generic Rx error */;
pub const FM10K_RXD_STATUS_L4E: c_uint = 0x4000 /* L4 csum error */;
pub const FM10K_RXD_STATUS_IPE: c_uint = 0x8000 /* IPv4 csum error */;
pub const FM10K_RXD_ERR_SWITCH_ERROR: c_uint = 0x0001 /* Switch found bad packet */;
pub const FM10K_RXD_ERR_NO_DESCRIPTOR: c_uint = 0x0002 /* No descriptor available */;
pub const FM10K_RXD_ERR_PP_ERROR: c_uint = 0x0004 /* RAM error during processing */;
pub const FM10K_RXD_ERR_SWITCH_READY: c_uint = 0x0008 /* Link transition mid-packet */;
pub const FM10K_RXD_ERR_TOO_BIG: c_uint = 0x0010 /* Pkt too big for single buf */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_ftag {
    pub swpri_type_user: __be16,
    pub vlan: __be16,
    pub sglort: __be16,
    pub dglort: __be16,
}
