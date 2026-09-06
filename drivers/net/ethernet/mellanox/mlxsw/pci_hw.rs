//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/pci_hw.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2015-2018 Mellanox Technologies. All rights reserved

pub const MLXSW_PCI_PAGE_SIZE: c_int = 4096;
pub const MLXSW_PCI_CIR_BASE: c_uint = 0x71000;

pub const MLXSW_PCI_CIR_CTRL_OPCODE_MOD_SHIFT: c_int = 12;
pub const MLXSW_PCI_CIR_CTRL_STATUS_SHIFT: c_int = 24;
pub const MLXSW_PCI_CIR_TIMEOUT_MSECS: c_int = 1000;
pub const MLXSW_PCI_SW_RESET_TIMEOUT_MSECS: c_int = 900000;
pub const MLXSW_PCI_SW_RESET_WAIT_MSECS: c_int = 400;
pub const MLXSW_PCI_FW_READY: c_uint = 0xA1844;
pub const MLXSW_PCI_FW_READY_MASK: c_uint = 0xFFFF;
pub const MLXSW_PCI_FW_READY_MAGIC: c_uint = 0x5E;
pub const MLXSW_PCI_DOORBELL_SDQ_OFFSET: c_uint = 0x000;
pub const MLXSW_PCI_DOORBELL_RDQ_OFFSET: c_uint = 0x200;
pub const MLXSW_PCI_DOORBELL_CQ_OFFSET: c_uint = 0x400;
pub const MLXSW_PCI_DOORBELL_EQ_OFFSET: c_uint = 0x600;
pub const MLXSW_PCI_DOORBELL_ARM_CQ_OFFSET: c_uint = 0x800;
pub const MLXSW_PCI_DOORBELL_ARM_EQ_OFFSET: c_uint = 0xA00;

pub const MLXSW_PCI_CQS_MAX: c_int = 96;
pub const MLXSW_PCI_EQS_MAX: c_int = 2;
pub const MLXSW_PCI_EQS_COUNT: c_int = 1;
pub const MLXSW_PCI_EQ_COMP_NUM: c_int = 1;

pub const MLXSW_PCI_SDQ_EMAD_INDEX: c_int = 0;
pub const MLXSW_PCI_SDQ_EMAD_TC: c_int = 0;
pub const MLXSW_PCI_SDQ_CTL_TC: c_int = 3;
pub const MLXSW_PCI_AQ_PAGES: c_int = 8;

pub const MLXSW_PCI_EQE_UPDATE_COUNT: c_uint = 0x80;
pub const MLXSW_PCI_WQE_SG_ENTRIES: c_int = 3;
pub const MLXSW_PCI_WQE_TYPE_ETHERNET: c_uint = 0xA;
// pci_wqe_c
// If set it indicates that a completion should be reported upon
// execution of this descriptor.
//
// pci_wqe_lp
// Local Processing, set if packet should be processed by the local
// switch hardware:
// For Ethernet EMAD (Direct Route and non Direct Route) -
// must be set if packet destination is local device
// For InfiniBand CTL - must be set if packet destination is local device
// Otherwise it must be clear
// Local Process packets must not exceed the size of 2K (including payload
// and headers).
//
// pci_wqe_type
// Packet type.
//
// pci_wqe_ipcs
// Calculate IPv4 and TCP / UDP checksums.
//
// pci_wqe_byte_count
// Size of i-th scatter/gather entry, 0 if entry is unused.
//
// pci_wqe_address
// Physical address of i-th scatter/gather entry.
// Gather Entries must be 2Byte aligned.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_pci_cqe_v {
    MLXSW_PCI_CQE_V0,
    MLXSW_PCI_CQE_V1,
    MLXSW_PCI_CQE_V2,
}

// pci_cqe_lag
// Packet arrives from a port which is a LAG
//
// pci_cqe_system_port/lag_id
// When lag=0: System port on which the packet was received
// When lag=1:
// bits [15:4] LAG ID on which the packet was received
// bits [3:0] sub_port on which the packet was received
//
// pci_cqe_wqe_counter
// WQE count of the WQEs completed on the associated dqn
//
// pci_cqe_byte_count
// Byte count of received packets including additional two
// Reserved Bytes that are append to the end of the frame.
// Reserved for Send CQE.
//
pub const MLXSW_PCI_CQE2_MIRROR_CONG_INVALID: c_uint = 0xFFFF;
// pci_cqe_mirror_cong_high
// Congestion level in units of 8KB of the egress traffic class of the original
// packet that does mirroring to the CPU. Value of 0xFFFF means that the
// congestion level is invalid.
//
// pci_cqe_trap_id
// Trap ID that captured the packet.
//
// pci_cqe_crc
// Length include CRC. Indicates the length field includes
// the packet's CRC.
//
// pci_cqe_e
// CQE with Error.
//
// pci_cqe_sr
// 1 - Send Queue
// 0 - Receive Queue
//
// pci_cqe_dqn
// Descriptor Queue (DQ) Number.
//
// pci_cqe_time_stamp_low
// Time stamp of the CQE
// Format according to time_stamp_type:
// 0: uSec - 1.024uSec (default for devices which do not support
// time_stamp_type). Only bits 15:0 are valid
// 1: FRC - Free Running Clock - units of 1nSec
// 2: UTC - time_stamp[37:30] = Sec
// - time_stamp[29:0] = nSec
// 3: Mirror_UTC. UTC time stamp of the original packet that has
// MIRROR_SESSION traps
// - time_stamp[37:30] = Sec
// - time_stamp[29:0] = nSec
// Formats 0..2 are configured by
// CONFIG_PROFILE.cqe_time_stamp_type for PTP traps
// Format 3 is used for MIRROR_SESSION traps
// Note that Spectrum does not reveal FRC, UTC and Mirror_UTC
//
pub const MLXSW_PCI_CQE2_MIRROR_TCLASS_INVALID: c_uint = 0x1F;
// pci_cqe_mirror_tclass
// The egress traffic class of the original packet that does mirroring to the
// CPU. Value of 0x1F means that the traffic class is invalid.
//
// pci_cqe_tx_lag
// The Tx port of a packet that is mirrored / sampled to the CPU is a LAG.
//
// pci_cqe_tx_lag_subport
// The port index within the LAG of a packet that is mirrored / sampled to the
// CPU. Reserved when tx_lag is 0.
//
pub const MLXSW_PCI_CQE2_TX_PORT_MULTI_PORT: c_uint = 0xFFFE;
pub const MLXSW_PCI_CQE2_TX_PORT_INVALID: c_uint = 0xFFFF;
// pci_cqe_tx_lag_id
// The Tx LAG ID of the original packet that is mirrored / sampled to the CPU.
// Value of 0xFFFE means multi-port. Value fo 0xFFFF means that the Tx LAG ID
// is invalid. Reserved when tx_lag is 0.
//
// pci_cqe_tx_system_port
// The Tx port of the original packet that is mirrored / sampled to the CPU.
// Value of 0xFFFE means multi-port. Value fo 0xFFFF means that the Tx port is
// invalid. Reserved when tx_lag is 1.
//
// pci_cqe_mirror_cong_low
// Congestion level in units of 8KB of the egress traffic class of the original
// packet that does mirroring to the CPU. Value of 0xFFFF means that the
// congestion level is invalid.
//

// pci_cqe_user_def_val_orig_pkt_len
// When trap_id is an ACL: User defined value from policy engine action.
//
// pci_cqe_mirror_reason
// Mirror reason.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_pci_cqe_time_stamp_type {
    MLXSW_PCI_CQE_TIME_STAMP_TYPE_USEC,
    MLXSW_PCI_CQE_TIME_STAMP_TYPE_FRC,
    MLXSW_PCI_CQE_TIME_STAMP_TYPE_UTC,
    MLXSW_PCI_CQE_TIME_STAMP_TYPE_MIRROR_UTC,
}

// pci_cqe_time_stamp_type
// Time stamp type:
// 0: uSec - 1.024uSec (default for devices which do not support
// time_stamp_type)
// 1: FRC - Free Running Clock - units of 1nSec
// 2: UTC
// 3: Mirror_UTC. UTC time stamp of the original packet that has
// MIRROR_SESSION traps
//
pub const MLXSW_PCI_CQE2_MIRROR_LATENCY_INVALID: c_uint = 0xFFFFFF;
// pci_cqe_time_stamp_high
// Time stamp of the CQE
// Format according to time_stamp_type:
// 0: uSec - 1.024uSec (default for devices which do not support
// time_stamp_type). Only bits 15:0 are valid
// 1: FRC - Free Running Clock - units of 1nSec
// 2: UTC - time_stamp[37:30] = Sec
// - time_stamp[29:0] = nSec
// 3: Mirror_UTC. UTC time stamp of the original packet that has
// MIRROR_SESSION traps
// - time_stamp[37:30] = Sec
// - time_stamp[29:0] = nSec
// Formats 0..2 are configured by
// CONFIG_PROFILE.cqe_time_stamp_type for PTP traps
// Format 3 is used for MIRROR_SESSION traps
// Note that Spectrum does not reveal FRC, UTC and Mirror_UTC
//
// pci_cqe_mirror_latency
// End-to-end latency of the original packet that does mirroring to the CPU.
// Value of 0xFFFFFF means that the latency is invalid. Units are according to
// MOGCR.mirror_latency_units.
//
// pci_cqe_owner
// Ownership bit.
//
// pci_eqe_event_type
// Event type.
//
pub const MLXSW_PCI_EQE_EVENT_TYPE_COMP: c_uint = 0x00;
pub const MLXSW_PCI_EQE_EVENT_TYPE_CMD: c_uint = 0x0A;
// pci_eqe_event_sub_type
// Event type.
//
// pci_eqe_cqn
// Completion Queue that triggered this EQE.
//
// pci_eqe_owner
// Ownership bit.
//
// pci_eqe_cmd_token
// Command completion event - token
//
// pci_eqe_cmd_status
// Command completion event - status
//
// pci_eqe_cmd_out_param_h
// Command completion event - output parameter - higher part
//
// pci_eqe_cmd_out_param_l
// Command completion event - output parameter - lower part
//
