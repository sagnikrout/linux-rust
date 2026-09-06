//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_fw_defs.h
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


// bnx2x_fw_defs.h: Qlogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

// IRO[142].m2) + ((sbId) * IRO[142].m3))

pub const COMMON_ASM_INVALID_ASSERT_OPCODE: c_uint = 0x0;
// eth hsi version

// Ethernet Ring parameters
pub const X_ETH_LOCAL_RING_SIZE: c_int = 13;
pub const FIRST_BD_IN_PKT: c_int = 0;
pub const PARSE_BD_INDEX: c_int = 1;

pub const U_ETH_NUM_OF_SGES_TO_FETCH: c_int = 8;
pub const U_ETH_MAX_SGES_FOR_PACKET: c_int = 3;
// Rx ring params
pub const U_ETH_LOCAL_BD_RING_SIZE: c_int = 8;
pub const U_ETH_LOCAL_SGE_RING_SIZE: c_int = 10;
pub const U_ETH_SGL_SIZE: c_int = 8;
// The fw will padd the buffer with this value, so the IP header \
pub const IP_HEADER_ALIGNMENT_PADDING: c_int = 2;

pub const U_ETH_UNDEFINED_Q: c_uint = 0xFF;
pub const T_ETH_INDIRECTION_TABLE_SIZE: c_int = 128;
pub const T_ETH_RSS_KEY: c_int = 10;
pub const ETH_NUM_OF_RSS_ENGINES_E2: c_int = 72;
pub const FILTER_RULES_COUNT: c_int = 16;
pub const MULTICAST_RULES_COUNT: c_int = 16;
pub const CLASSIFY_RULES_COUNT: c_int = 16;
// The CRC32 seed, that is used for the hash(reduction) multicast address
pub const ETH_CRC32_HASH_SEED: c_uint = 0x00000000;

// Maximal L2 clients supported
pub const ETH_MAX_RX_CLIENTS_E1: c_int = 18;
pub const ETH_MAX_RX_CLIENTS_E1H: c_int = 28;
pub const ETH_MAX_RX_CLIENTS_E2: c_int = 152;
// Maximal statistics client Ids
pub const MAX_STAT_COUNTER_ID_E1: c_int = 36;
pub const MAX_STAT_COUNTER_ID_E1H: c_int = 56;
pub const MAX_STAT_COUNTER_ID_E2: c_int = 140;

// Maximal aggregation queues supported
pub const ETH_MAX_AGGREGATION_QUEUES_E1: c_int = 32;
pub const ETH_MAX_AGGREGATION_QUEUES_E1H_E2: c_int = 64;
pub const ETH_NUM_OF_MCAST_BINS: c_int = 256;
pub const ETH_NUM_OF_MCAST_ENGINES_E2: c_int = 72;

pub const DISABLE_STATISTIC_COUNTER_ID_VALUE: c_int = 0;
// This file defines HSI constants common to all microcode flows
pub const PROTOCOL_STATE_BIT_OFFSET: c_int = 6;

// microcode fixed page size 4K (chains and ring segments)
pub const MC_PAGE_SIZE: c_int = 4096;
// Number of indices per slow-path SB
pub const HC_SP_SB_MAX_INDICES: c_int = 16;
// Number of indices per SB
pub const HC_SB_MAX_INDICES_E1X: c_int = 8;
pub const HC_SB_MAX_INDICES_E2: c_int = 8;
pub const HC_SB_MAX_SB_E1X: c_int = 32;
pub const HC_SB_MAX_SB_E2: c_int = 136;
pub const HC_SP_SB_ID: c_uint = 0xde;
pub const HC_SB_MAX_SM: c_int = 2;
pub const HC_SB_MAX_DYNAMIC_INDICES: c_int = 4;
// max number of slow path commands per port
pub const MAX_RAMRODS_PER_PORT: c_int = 8;
// DEFINES FOR TIMERS/CLOCKS RESOLUTIONS

// END DEFINES FOR TIMERS/CLOCKS RESOLUTIONS
pub const XSTORM_IP_ID_ROLL_HALF: c_uint = 0x8000;
pub const XSTORM_IP_ID_ROLL_ALL: c_int = 0;
pub const FW_LOG_LIST_SIZE: c_int = 50;
pub const NUM_OF_SAFC_BITS: c_int = 16;
pub const MAX_COS_NUMBER: c_int = 4;
pub const MAX_TRAFFIC_TYPES: c_int = 8;
pub const MAX_PFC_PRIORITIES: c_int = 8;
pub const MAX_VLAN_PRIORITIES: c_int = 8;
// used by array traffic_type_to_priority[] to mark traffic type \
pub const LLFC_TRAFFIC_TYPE_TO_PRIORITY_UNMAPPED: c_uint = 0xFF;

pub const STATS_QUERY_CMD_COUNT: c_int = 16;
pub const AFEX_LIST_TABLE_SIZE: c_int = 4096;
pub const INVALID_VNIC_ID: c_uint = 0xFF;
pub const UNDEF_IRO: c_uint = 0x80000000;
// used for defining the amount of FCoE tasks supported for PF
pub const MAX_FCOE_FUNCS_PER_ENGINE: c_int = 2;
pub const MAX_NUM_FCOE_TASKS_PER_ENGINE: c_int = 4096;
