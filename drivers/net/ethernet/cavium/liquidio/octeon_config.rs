//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_config.h
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
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file  octeon_config.h
// \brief Host Driver: Configuration data structures for the host driver.
//
// --------------------------CONFIG VALUES------------------------
// The following macros affect the way the driver data structures
// are generated for Octeon devices.
// They can be modified.
//
// Maximum octeon devices defined as MAX_OCTEON_NICIF to support
// multiple(<= MAX_OCTEON_NICIF) Miniports
//
pub const MAX_OCTEON_NICIF: c_int = 128;

pub const MAX_OCTEON_MULTICAST_ADDR: c_int = 32;
pub const MAX_OCTEON_FILL_COUNT: c_int = 8;
// CN6xxx IQ configuration macros
pub const CN6XXX_MAX_INPUT_QUEUES: c_int = 32;
pub const CN6XXX_MAX_IQ_DESCRIPTORS: c_int = 2048;
pub const CN6XXX_DB_MIN: c_int = 1;
pub const CN6XXX_DB_MAX: c_int = 8;
pub const CN6XXX_DB_TIMEOUT: c_int = 1;
// CN6xxx OQ configuration macros
pub const CN6XXX_MAX_OUTPUT_QUEUES: c_int = 32;
pub const CN6XXX_MAX_OQ_DESCRIPTORS: c_int = 2048;
pub const CN6XXX_OQ_BUF_SIZE: c_int = 1664;

pub const CN6XXX_OQ_INTR_PKT: c_int = 64;
pub const CN6XXX_OQ_INTR_TIME: c_int = 100;
pub const DEFAULT_NUM_NIC_PORTS_66XX: c_int = 2;
pub const DEFAULT_NUM_NIC_PORTS_68XX: c_int = 4;
pub const DEFAULT_NUM_NIC_PORTS_68XX_210NV: c_int = 2;
// CN23xx  IQ configuration macros
pub const CN23XX_MAX_VFS_PER_PF_PASS_1_0: c_int = 8;
pub const CN23XX_MAX_VFS_PER_PF_PASS_1_1: c_int = 31;
pub const CN23XX_MAX_VFS_PER_PF: c_int = 63;
pub const CN23XX_MAX_RINGS_PER_VF: c_int = 8;
pub const CN23XX_MAX_RINGS_PER_PF_PASS_1_0: c_int = 12;
pub const CN23XX_MAX_RINGS_PER_PF_PASS_1_1: c_int = 32;
pub const CN23XX_MAX_RINGS_PER_PF: c_int = 64;
pub const CN23XX_MAX_RINGS_PER_VF: c_int = 8;

pub const CN23XX_MAX_IQ_DESCRIPTORS: c_int = 2048;
pub const CN23XX_DEFAULT_IQ_DESCRIPTORS: c_int = 512;
pub const CN23XX_MIN_IQ_DESCRIPTORS: c_int = 128;
pub const CN23XX_DB_MIN: c_int = 1;
pub const CN23XX_DB_MAX: c_int = 8;
pub const CN23XX_DB_TIMEOUT: c_int = 1;

pub const CN23XX_MAX_OQ_DESCRIPTORS: c_int = 2048;
pub const CN23XX_DEFAULT_OQ_DESCRIPTORS: c_int = 512;
pub const CN23XX_MIN_OQ_DESCRIPTORS: c_int = 128;
pub const CN23XX_OQ_BUF_SIZE: c_int = 1664;
pub const CN23XX_OQ_PKTSPER_INTR: c_int = 128;
// #define CAVIUM_ONLY_CN23XX_RX_PERF
pub const CN23XX_OQ_REFIL_THRESHOLD: c_int = 16;
pub const CN23XX_OQ_INTR_PKT: c_int = 64;
pub const CN23XX_OQ_INTR_TIME: c_int = 100;
pub const DEFAULT_NUM_NIC_PORTS_23XX: c_int = 1;

// PEMs count
pub const CN23XX_MAX_MACS: c_int = 4;
pub const CN23XX_DEF_IQ_INTR_THRESHOLD: c_int = 32;

// common OCTEON configuration macros
pub const CN6XXX_CFG_IO_QUEUES: c_int = 32;
pub const OCTEON_32BYTE_INSTR: c_int = 32;
pub const OCTEON_64BYTE_INSTR: c_int = 64;
pub const OCTEON_MAX_BASE_IOQ: c_int = 4;
pub const OCTEON_DMA_INTR_PKT: c_int = 64;
pub const OCTEON_DMA_INTR_TIME: c_int = 1000;
pub const MAX_TXQS_PER_INTF: c_int = 8;
pub const MAX_RXQS_PER_INTF: c_int = 8;
pub const DEF_TXQS_PER_INTF: c_int = 4;
pub const DEF_RXQS_PER_INTF: c_int = 4;
pub const INVALID_IOQ_NO: c_uint = 0xff;
pub const DEFAULT_POW_GRP: c_int = 0;
// Macros to get octeon config params

// Max IOQs per OCTEON Link
pub const MAX_IOQS_PER_NICIF: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lio_card_type {
    LIO_210SV = 0, /* Two port, 66xx */
    LIO_210NV,     /* Two port, 68xx */
    LIO_410NV,     /* Four port, 68xx */
    LIO_23XX       /* 23xx */
}

// Structure to define the configuration attributes for each Input queue.
// Applicable to all Octeon processors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_iq_config {

    pub reserved:16: u64,
// Tx interrupt packets. Applicable to 23xx only
    pub iq_intr_pkt:16: u64,
// Minimum ticks to wait before checking for pending instructions.
    pub db_timeout:16: u64,
// Minimum number of commands pending to be posted to Octeon
// before driver hits the Input queue doorbell.
//
    pub db_min:8: u64,
// Command size - 32 or 64 bytes
    pub instr_type:32: u64,
// Pending list size (usually set to the sum of the size of all Input
// queues)
//
    pub pending_list_size:32: u64,
// Max number of IQs available
    pub max_iqs:8: u64,

// Max number of IQs available
    pub max_iqs:8: u64,
// Pending list size (usually set to the sum of the size of all Input
// queues)
//
    pub pending_list_size:32: u64,
// Command size - 32 or 64 bytes
    pub instr_type:32: u64,
// Minimum number of commands pending to be posted to Octeon
// before driver hits the Input queue doorbell.
//
    pub db_min:8: u64,
// Minimum ticks to wait before checking for pending instructions.
    pub db_timeout:16: u64,
// Tx interrupt packets. Applicable to 23xx only
    pub iq_intr_pkt:16: u64,
    pub reserved:16: u64,

}

// Structure to define the configuration attributes for each Output queue.
// Applicable to all Octeon processors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_oq_config {

    pub reserved:16: u64,
    pub pkts_per_intr:16: u64,
// Interrupt Coalescing (Time Interval). Octeon will interrupt the
// host if atleast one packet was sent in the time interval specified
// by this field. The driver uses time interval interrupt coalescing
// by default. The time is specified in microseconds.
//
    pub oq_intr_time:16: u64,
// Interrupt Coalescing (Packet Count). Octeon will interrupt the host
// only if it sent as many packets as specified by this field.
// The driver
// usually does not use packet count interrupt coalescing.
//
    pub oq_intr_pkt:16: u64,
// The number of buffers that were consumed during packet processing by
// the driver on this Output queue before the driver attempts to
// replenish
// the descriptor ring with new buffers.
//
    pub refill_threshold:16: u64,
// Max number of OQs available
    pub max_oqs:8: u64,

// Max number of OQs available
    pub max_oqs:8: u64,
// The number of buffers that were consumed during packet processing by
// the driver on this Output queue before the driver attempts to
// replenish
// the descriptor ring with new buffers.
//
    pub refill_threshold:16: u64,
// Interrupt Coalescing (Packet Count). Octeon will interrupt the host
// only if it sent as many packets as specified by this field.
// The driver
// usually does not use packet count interrupt coalescing.
//
    pub oq_intr_pkt:16: u64,
// Interrupt Coalescing (Time Interval). Octeon will interrupt the
// host if atleast one packet was sent in the time interval specified
// by this field. The driver uses time interval interrupt coalescing
// by default.  The time is specified in microseconds.
//
    pub oq_intr_time:16: u64,
    pub pkts_per_intr:16: u64,
    pub reserved:16: u64,

}

// This structure conatins the NIC link configuration attributes,
// common for all the OCTEON Modles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_nic_if_config {

    pub reserved:56: u64,
    pub base_queue:16: u64,
    pub gmx_port_id:8: u64,
// SKB size, We need not change buf size even for Jumbo frames.
// Octeon can send jumbo frames in 4 consecutive descriptors,
//
    pub rx_buf_size:16: u64,
// Num of desc for tx rings
    pub num_tx_descs:16: u64,
// Num of desc for rx rings
    pub num_rx_descs:16: u64,
// Actual configured value. Range could be: 1...max_rxqs
    pub num_rxqs:16: u64,
// Max Rxqs: Half for each of the two ports :max_oq/2
    pub max_rxqs:16: u64,
// Actual configured value. Range could be: 1...max_txqs
    pub num_txqs:16: u64,
// Max Txqs: Half for each of the two ports :max_iq/2
    pub max_txqs:16: u64,

// Max Txqs: Half for each of the two ports :max_iq/2
    pub max_txqs:16: u64,
// Actual configured value. Range could be: 1...max_txqs
    pub num_txqs:16: u64,
// Max Rxqs: Half for each of the two ports :max_oq/2
    pub max_rxqs:16: u64,
// Actual configured value. Range could be: 1...max_rxqs
    pub num_rxqs:16: u64,
// Num of desc for rx rings
    pub num_rx_descs:16: u64,
// Num of desc for tx rings
    pub num_tx_descs:16: u64,
// SKB size, We need not change buf size even for Jumbo frames.
// Octeon can send jumbo frames in 4 consecutive descriptors,
//
    pub rx_buf_size:16: u64,
    pub gmx_port_id:8: u64,
    pub base_queue:16: u64,
    pub reserved:56: u64,

}

// Structure to define the configuration attributes for meta data.
// Applicable to all Octeon processors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_misc_config {

// Host link status polling period
    pub host_link_query_interval:32: u64,
// Oct link status polling period
    pub oct_link_query_interval:32: u64,
    pub enable_sli_oq_bp:1: u64,
// Control IQ Group
    pub ctrlq_grp:4: u64,

// Control IQ Group
    pub ctrlq_grp:4: u64,
// BP for SLI OQ
    pub enable_sli_oq_bp:1: u64,
// Host link status polling period
    pub oct_link_query_interval:32: u64,
// Oct link status polling period
    pub host_link_query_interval:32: u64,

}

// Structure to define the configuration for all OCTEON processors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_config {
    pub card_type: u16,
    pub card_name: *mut c_char,
// Input Queue attributes.
    pub iq: octeon_iq_config,
// Output Queue attributes.
    pub oq: octeon_oq_config,
// NIC Port Configuration
    pub nic_if_cfg: [octeon_nic_if_config; MAX_OCTEON_NICIF],
// Miscellaneous attributes
    pub misc: octeon_misc_config,
    pub num_nic_ports: c_int,
    pub num_def_tx_descs: c_int,
// Num of desc for rx rings
    pub num_def_rx_descs: c_int,
    pub def_rx_buf_size: c_int,
}

// The following config values are fixed and should not be modified.
pub const BAR1_INDEX_DYNAMIC_MAP: c_int = 2;
pub const BAR1_INDEX_STATIC_MAP: c_int = 15;

// Response lists - 1 ordered, 1 unordered-blocking, 1 unordered-nonblocking
// 1 process done list, 1 zombie lists(timeouted sc list)
// NoResponse Lists are now maintained with each IQ. (Dec' 2007).
//
pub const MAX_RESPONSE_LISTS: c_int = 6;
// Opcode hash bits. The opcode is hashed on the lower 6-bits to lookup the
// dispatch table.
//
pub const OPCODE_MASK_BITS: c_int = 6;
// Mask for the 6-bit lookup hash
pub const OCTEON_OPCODE_MASK: c_uint = 0x3f;
// Size of the dispatch table. The 6-bit hash can index into 2^6 entries

// Maximum number of Octeon Instruction (command) queues

// Maximum number of Octeon Instruction (command) queues

pub const MAX_POSSIBLE_VFS: c_int = 64;
