//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_config.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//
// Tx instruction types by length
pub const OCTEP_32BYTE_INSTR: c_int = 32;
pub const OCTEP_64BYTE_INSTR: c_int = 64;
// Tx Queue: maximum descriptors per ring
// This needs to be a power of 2
pub const OCTEP_IQ_MAX_DESCRIPTORS: c_int = 1024;
// Minimum input (Tx) requests to be enqueued to ring doorbell
pub const OCTEP_DB_MIN: c_int = 8;
// Packet threshold for Tx queue interrupt
pub const OCTEP_IQ_INTR_THRESHOLD: c_uint = 0x0;
// Minimum watermark for backpressure
pub const OCTEP_OQ_WMARK_MIN: c_int = 256;
// Rx Queue: maximum descriptors per ring
pub const OCTEP_OQ_MAX_DESCRIPTORS: c_int = 1024;
// Rx buffer size: Use page size buffers.
// Build skb from allocated page buffer once the packet is received.
// When a gathered packet is received, make head page as skb head and
// page buffers in consecutive Rx descriptors as fragments.
//

pub const OCTEP_OQ_PKTS_PER_INTR: c_int = 128;

pub const OCTEP_OQ_INTR_PKT_THRESHOLD: c_int = 1;
pub const OCTEP_OQ_INTR_TIME_THRESHOLD: c_int = 10;

// Tx Queue wake threshold
// wakeup a stopped Tx queue if minimum 2 descriptors are available.
// Even a skb with fragments consume only one Tx queue descriptor entry.
//
pub const OCTEP_WAKE_QUEUE_THRESHOLD: c_int = 2;
// Minimum MTU supported by Octeon network interface

// Default MTU
pub const OCTEP_DEFAULT_MTU: c_int = 1500;
// pf heartbeat interval in milliseconds
pub const OCTEP_DEFAULT_FW_HB_INTERVAL: c_int = 1000;
// pf heartbeat miss count
pub const OCTEP_DEFAULT_FW_HB_MISS_COUNT: c_int = 20;
// Macros to get octeon config params

// Hardware Tx Queue configuration.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iq_config {
// Size of the Input queue (number of commands)
    pub num_descs: u16,
// Command size - 32 or 64 bytes
    pub instr_type: u16,
// Minimum number of commands pending to be posted to Octeon before driver
// hits the Input queue doorbell.
//
    pub db_min: u16,
// Trigger the IQ interrupt when processed cmd count reaches
// this level.
//
    pub intr_threshold: u32,
}

// Hardware Rx Queue configuration.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq_config {
// Size of Output queue (number of descriptors)
    pub num_descs: u16,
// Size of buffer in this Output queue.
    pub buf_size: u16,
// The number of buffers that were consumed during packet processing
// by the driver on this Output queue before the driver attempts to
// replenish the descriptor ring with new buffers.
//
    pub refill_threshold: u16,
// Interrupt Coalescing (Packet Count). Octeon will interrupt the host
// only if it sent as many packets as specified by this field.
// The driver usually does not use packet count interrupt coalescing.
//
    pub oq_intr_pkt: u32,
// Interrupt Coalescing (Time Interval). Octeon will interrupt the host
// if at least one packet was sent in the time interval specified by
// this field. The driver uses time interval interrupt coalescing by
// default. The time is specified in microseconds.
//
    pub oq_intr_time: u32,
// Water mark for backpressure.
// Output queue sends backpressure signal to source when
// free buffer count falls below wmark.
//
    pub wmark: u32,
}

// Tx/Rx configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_pf_ring_config {
// Max number of IOQs
    pub max_io_rings: u16,
// Number of active IOQs
    pub active_io_rings: u16,
// Starting IOQ number: this changes based on which PEM is used
    pub srn: u16,
}

// Octeon Hardware SRIOV config
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_sriov_config {
// Max number of VF devices supported
    pub max_vfs: u16,
// Number of VF devices enabled
    pub active_vfs: u16,
// Max number of rings assigned to VF
    pub max_rings_per_vf: u8,
// Number of rings enabled per VF
    pub active_rings_per_vf: u8,
// starting ring number of VF's: ring-0 of VF-0 of the PF
    pub vf_srn: u16,
}

// Octeon MSI-x config.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_msix_config {
// Number of IOQ interrupts
    pub ioq_msix: u16,
// Number of Non IOQ interrupts
    pub non_ioq_msix: u16,
// Names of Non IOQ interrupts
    pub non_ioq_msix_names: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_mbox_config {
// Barmem address for control mbox
    pub barmem_addr: *mut void __iomem,
}

// Info from firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_fw_info {
// interface pkind
    pub pkind: u8,
// front size data
    pub fsz: u8,
// heartbeat interval in milliseconds
    pub hb_interval: u16,
// heartbeat miss count
    pub hb_miss_count: u16,
// reserved
    pub reserved1: u16,
// supported rx offloads OCTEP_ETH_RX_OFFLOAD_*
    pub rx_ol_flags: u16,
// supported tx offloads OCTEP_ETH_TX_OFFLOAD_*
    pub tx_ol_flags: u16,
// reserved
    pub reserved_offloads: u32,
// extra offload flags
    pub ext_ol_flags: u64,
// supported features
    pub features: [u64; 2],
// reserved
    pub reserved2: [u64; 3],
}

// Data Structure to hold configuration limits and active config
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_config {
// Input Queue attributes.
    pub iq: octep_iq_config,
// Output Queue attributes.
    pub oq: octep_oq_config,
// NIC Port Configuration
    pub pf_ring_cfg: octep_pf_ring_config,
// SRIOV configuration of the PF
    pub sriov_cfg: octep_sriov_config,
// MSI-X interrupt config
    pub msix_cfg: octep_msix_config,
// ctrl mbox config
    pub ctrl_mbox_cfg: octep_ctrl_mbox_config,
// fw info
    pub fw_info: octep_fw_info,
}
