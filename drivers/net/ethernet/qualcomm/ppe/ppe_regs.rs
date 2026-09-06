//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/ppe/ppe_regs.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// PPE hardware register and table declarations.

// PPE scheduler configurations for buffer manager block.
pub const PPE_BM_SCH_CTRL_ADDR: c_uint = 0xb000;
pub const PPE_BM_SCH_CTRL_INC: c_int = 4;

// PPE drop counters.
pub const PPE_DROP_CNT_TBL_ADDR: c_uint = 0xb024;
pub const PPE_DROP_CNT_TBL_ENTRIES: c_int = 8;
pub const PPE_DROP_CNT_TBL_INC: c_int = 4;
// BM port drop counters.
pub const PPE_DROP_STAT_TBL_ADDR: c_uint = 0xe000;
pub const PPE_DROP_STAT_TBL_ENTRIES: c_int = 30;
pub const PPE_DROP_STAT_TBL_INC: c_uint = 0x10;
// Egress VLAN counters.
pub const PPE_EG_VSI_COUNTER_TBL_ADDR: c_uint = 0x41000;
pub const PPE_EG_VSI_COUNTER_TBL_ENTRIES: c_int = 64;
pub const PPE_EG_VSI_COUNTER_TBL_INC: c_uint = 0x10;
// Port TX counters.
pub const PPE_PORT_TX_COUNTER_TBL_ADDR: c_uint = 0x45000;
pub const PPE_PORT_TX_COUNTER_TBL_ENTRIES: c_int = 8;
pub const PPE_PORT_TX_COUNTER_TBL_INC: c_uint = 0x10;
// Virtual port TX counters.
pub const PPE_VPORT_TX_COUNTER_TBL_ADDR: c_uint = 0x47000;
pub const PPE_VPORT_TX_COUNTER_TBL_ENTRIES: c_int = 256;
pub const PPE_VPORT_TX_COUNTER_TBL_INC: c_uint = 0x10;
// Queue counters.
pub const PPE_QUEUE_TX_COUNTER_TBL_ADDR: c_uint = 0x4a000;
pub const PPE_QUEUE_TX_COUNTER_TBL_ENTRIES: c_int = 300;
pub const PPE_QUEUE_TX_COUNTER_TBL_INC: c_uint = 0x10;
// RSS settings are to calculate the random RSS hash value generated during
// packet receive to ARM cores. This hash is then used to generate the queue
// offset used to determine the queue used to transmit the packet to ARM cores.
//
pub const PPE_RSS_HASH_MASK_ADDR: c_uint = 0xb4318;

pub const PPE_RSS_HASH_SEED_ADDR: c_uint = 0xb431c;

pub const PPE_RSS_HASH_MIX_ADDR: c_uint = 0xb4320;
pub const PPE_RSS_HASH_MIX_ENTRIES: c_int = 11;
pub const PPE_RSS_HASH_MIX_INC: c_int = 4;

pub const PPE_RSS_HASH_FIN_ADDR: c_uint = 0xb4350;
pub const PPE_RSS_HASH_FIN_ENTRIES: c_int = 5;
pub const PPE_RSS_HASH_FIN_INC: c_int = 4;

pub const PPE_RSS_HASH_MASK_IPV4_ADDR: c_uint = 0xb4380;

pub const PPE_RSS_HASH_SEED_IPV4_ADDR: c_uint = 0xb4384;

pub const PPE_RSS_HASH_MIX_IPV4_ADDR: c_uint = 0xb4390;
pub const PPE_RSS_HASH_MIX_IPV4_ENTRIES: c_int = 5;
pub const PPE_RSS_HASH_MIX_IPV4_INC: c_int = 4;

pub const PPE_RSS_HASH_FIN_IPV4_ADDR: c_uint = 0xb43b0;
pub const PPE_RSS_HASH_FIN_IPV4_ENTRIES: c_int = 5;
pub const PPE_RSS_HASH_FIN_IPV4_INC: c_int = 4;

pub const PPE_BM_SCH_CFG_TBL_ADDR: c_uint = 0xc000;
pub const PPE_BM_SCH_CFG_TBL_ENTRIES: c_int = 128;
pub const PPE_BM_SCH_CFG_TBL_INC: c_uint = 0x10;

// PPE service code configuration for the ingress direction functions,
// including bypass configuration for relevant PPE switch core functions
// such as flow entry lookup bypass.
//
pub const PPE_SERVICE_TBL_ADDR: c_uint = 0x15000;
pub const PPE_SERVICE_TBL_ENTRIES: c_int = 256;
pub const PPE_SERVICE_TBL_INC: c_uint = 0x10;

// PPE port egress VLAN configurations.
pub const PPE_PORT_EG_VLAN_TBL_ADDR: c_uint = 0x20020;
pub const PPE_PORT_EG_VLAN_TBL_ENTRIES: c_int = 8;
pub const PPE_PORT_EG_VLAN_TBL_INC: c_int = 4;

// PPE queue counters enable/disable control.
pub const PPE_EG_BRIDGE_CONFIG_ADDR: c_uint = 0x20044;

// PPE service code configuration on the egress direction.
pub const PPE_EG_SERVICE_TBL_ADDR: c_uint = 0x43000;
pub const PPE_EG_SERVICE_TBL_ENTRIES: c_int = 256;
pub const PPE_EG_SERVICE_TBL_INC: c_uint = 0x10;

// PPE port bridge configuration
pub const PPE_PORT_BRIDGE_CTRL_ADDR: c_uint = 0x60300;
pub const PPE_PORT_BRIDGE_CTRL_ENTRIES: c_int = 8;
pub const PPE_PORT_BRIDGE_CTRL_INC: c_int = 4;

// PPE port control configurations for the traffic to the multicast queues.
pub const PPE_MC_MTU_CTRL_TBL_ADDR: c_uint = 0x60a00;
pub const PPE_MC_MTU_CTRL_TBL_ENTRIES: c_int = 8;
pub const PPE_MC_MTU_CTRL_TBL_INC: c_int = 4;

// PPE VSI configurations
pub const PPE_VSI_TBL_ADDR: c_uint = 0x63800;
pub const PPE_VSI_TBL_ENTRIES: c_int = 64;
pub const PPE_VSI_TBL_INC: c_uint = 0x10;

// PPE port control configurations for the traffic to the unicast queues.
pub const PPE_MRU_MTU_CTRL_TBL_ADDR: c_uint = 0x65000;
pub const PPE_MRU_MTU_CTRL_TBL_ENTRIES: c_int = 256;
pub const PPE_MRU_MTU_CTRL_TBL_INC: c_uint = 0x10;

// PPE service code configuration for destination port and counter.
pub const PPE_IN_L2_SERVICE_TBL_ADDR: c_uint = 0x66000;
pub const PPE_IN_L2_SERVICE_TBL_ENTRIES: c_int = 256;
pub const PPE_IN_L2_SERVICE_TBL_INC: c_uint = 0x10;

// L2 Port configurations
pub const PPE_L2_VP_PORT_TBL_ADDR: c_uint = 0x98000;
pub const PPE_L2_VP_PORT_TBL_ENTRIES: c_int = 256;
pub const PPE_L2_VP_PORT_TBL_INC: c_uint = 0x10;

// Port RX and RX drop counters.
pub const PPE_PORT_RX_CNT_TBL_ADDR: c_uint = 0x150000;
pub const PPE_PORT_RX_CNT_TBL_ENTRIES: c_int = 256;
pub const PPE_PORT_RX_CNT_TBL_INC: c_uint = 0x20;
// Physical port RX and RX drop counters.
pub const PPE_PHY_PORT_RX_CNT_TBL_ADDR: c_uint = 0x156000;
pub const PPE_PHY_PORT_RX_CNT_TBL_ENTRIES: c_int = 8;
pub const PPE_PHY_PORT_RX_CNT_TBL_INC: c_uint = 0x20;
// Counters for the packet to CPU port.
pub const PPE_DROP_CPU_CNT_TBL_ADDR: c_uint = 0x160000;
pub const PPE_DROP_CPU_CNT_TBL_ENTRIES: c_int = 1280;
pub const PPE_DROP_CPU_CNT_TBL_INC: c_uint = 0x10;
// VLAN counters.
pub const PPE_VLAN_CNT_TBL_ADDR: c_uint = 0x178000;
pub const PPE_VLAN_CNT_TBL_ENTRIES: c_int = 64;
pub const PPE_VLAN_CNT_TBL_INC: c_uint = 0x10;
// PPE L2 counters.
pub const PPE_PRE_L2_CNT_TBL_ADDR: c_uint = 0x17c000;
pub const PPE_PRE_L2_CNT_TBL_ENTRIES: c_int = 64;
pub const PPE_PRE_L2_CNT_TBL_INC: c_uint = 0x20;
// Port TX drop counters.
pub const PPE_PORT_TX_DROP_CNT_TBL_ADDR: c_uint = 0x17d000;
pub const PPE_PORT_TX_DROP_CNT_TBL_ENTRIES: c_int = 8;
pub const PPE_PORT_TX_DROP_CNT_TBL_INC: c_uint = 0x10;
// Virtual port TX counters.
pub const PPE_VPORT_TX_DROP_CNT_TBL_ADDR: c_uint = 0x17e000;
pub const PPE_VPORT_TX_DROP_CNT_TBL_ENTRIES: c_int = 256;
pub const PPE_VPORT_TX_DROP_CNT_TBL_INC: c_uint = 0x10;
// Counters for the tunnel packet.
pub const PPE_TPR_PKT_CNT_TBL_ADDR: c_uint = 0x1d0080;
pub const PPE_TPR_PKT_CNT_TBL_ENTRIES: c_int = 8;
pub const PPE_TPR_PKT_CNT_TBL_INC: c_int = 4;
// Counters for the all packet received.
pub const PPE_IPR_PKT_CNT_TBL_ADDR: c_uint = 0x1e0080;
pub const PPE_IPR_PKT_CNT_TBL_ENTRIES: c_int = 8;
pub const PPE_IPR_PKT_CNT_TBL_INC: c_int = 4;
// PPE service code configuration for the tunnel packet.
pub const PPE_TL_SERVICE_TBL_ADDR: c_uint = 0x306000;
pub const PPE_TL_SERVICE_TBL_ENTRIES: c_int = 256;
pub const PPE_TL_SERVICE_TBL_INC: c_int = 4;

// Port scheduler global config.
pub const PPE_PSCH_SCH_DEPTH_CFG_ADDR: c_uint = 0x400000;
pub const PPE_PSCH_SCH_DEPTH_CFG_INC: c_int = 4;

// PPE queue level scheduler configurations.
pub const PPE_L0_FLOW_MAP_TBL_ADDR: c_uint = 0x402000;
pub const PPE_L0_FLOW_MAP_TBL_ENTRIES: c_int = 300;
pub const PPE_L0_FLOW_MAP_TBL_INC: c_uint = 0x10;

pub const PPE_L0_C_FLOW_CFG_TBL_ADDR: c_uint = 0x404000;
pub const PPE_L0_C_FLOW_CFG_TBL_ENTRIES: c_int = 512;
pub const PPE_L0_C_FLOW_CFG_TBL_INC: c_uint = 0x10;

pub const PPE_L0_E_FLOW_CFG_TBL_ADDR: c_uint = 0x406000;
pub const PPE_L0_E_FLOW_CFG_TBL_ENTRIES: c_int = 512;
pub const PPE_L0_E_FLOW_CFG_TBL_INC: c_uint = 0x10;

pub const PPE_L0_FLOW_PORT_MAP_TBL_ADDR: c_uint = 0x408000;
pub const PPE_L0_FLOW_PORT_MAP_TBL_ENTRIES: c_int = 300;
pub const PPE_L0_FLOW_PORT_MAP_TBL_INC: c_uint = 0x10;

pub const PPE_L0_COMP_CFG_TBL_ADDR: c_uint = 0x428000;
pub const PPE_L0_COMP_CFG_TBL_ENTRIES: c_int = 300;
pub const PPE_L0_COMP_CFG_TBL_INC: c_uint = 0x10;

// PPE queue to Ethernet DMA ring mapping table.
pub const PPE_RING_Q_MAP_TBL_ADDR: c_uint = 0x42a000;
pub const PPE_RING_Q_MAP_TBL_ENTRIES: c_int = 24;
pub const PPE_RING_Q_MAP_TBL_INC: c_uint = 0x40;
// Table addresses for per-queue dequeue setting.
pub const PPE_DEQ_OPR_TBL_ADDR: c_uint = 0x430000;
pub const PPE_DEQ_OPR_TBL_ENTRIES: c_int = 300;
pub const PPE_DEQ_OPR_TBL_INC: c_uint = 0x10;

// PPE flow level scheduler configurations.
pub const PPE_L1_FLOW_MAP_TBL_ADDR: c_uint = 0x440000;
pub const PPE_L1_FLOW_MAP_TBL_ENTRIES: c_int = 64;
pub const PPE_L1_FLOW_MAP_TBL_INC: c_uint = 0x10;

pub const PPE_L1_C_FLOW_CFG_TBL_ADDR: c_uint = 0x442000;
pub const PPE_L1_C_FLOW_CFG_TBL_ENTRIES: c_int = 64;
pub const PPE_L1_C_FLOW_CFG_TBL_INC: c_uint = 0x10;

pub const PPE_L1_E_FLOW_CFG_TBL_ADDR: c_uint = 0x444000;
pub const PPE_L1_E_FLOW_CFG_TBL_ENTRIES: c_int = 64;
pub const PPE_L1_E_FLOW_CFG_TBL_INC: c_uint = 0x10;

pub const PPE_L1_FLOW_PORT_MAP_TBL_ADDR: c_uint = 0x446000;
pub const PPE_L1_FLOW_PORT_MAP_TBL_ENTRIES: c_int = 64;
pub const PPE_L1_FLOW_PORT_MAP_TBL_INC: c_uint = 0x10;

pub const PPE_L1_COMP_CFG_TBL_ADDR: c_uint = 0x46a000;
pub const PPE_L1_COMP_CFG_TBL_ENTRIES: c_int = 64;
pub const PPE_L1_COMP_CFG_TBL_INC: c_uint = 0x10;

// PPE port scheduler configurations for egress.
pub const PPE_PSCH_SCH_CFG_TBL_ADDR: c_uint = 0x47a000;
pub const PPE_PSCH_SCH_CFG_TBL_ENTRIES: c_int = 128;
pub const PPE_PSCH_SCH_CFG_TBL_INC: c_uint = 0x10;

// There are 15 BM ports and 4 BM groups supported by PPE.
// BM port (0-7) is for EDMA port 0, BM port (8-13) is for
// PPE physical port 1-6 and BM port 14 is for EIP port.
//
pub const PPE_BM_PORT_FC_MODE_ADDR: c_uint = 0x600100;
pub const PPE_BM_PORT_FC_MODE_ENTRIES: c_int = 15;
pub const PPE_BM_PORT_FC_MODE_INC: c_uint = 0x4;

pub const PPE_BM_PORT_GROUP_ID_ADDR: c_uint = 0x600180;
pub const PPE_BM_PORT_GROUP_ID_ENTRIES: c_int = 15;
pub const PPE_BM_PORT_GROUP_ID_INC: c_uint = 0x4;

// Counters for PPE buffers used for packets cached.
pub const PPE_BM_USED_CNT_TBL_ADDR: c_uint = 0x6001c0;
pub const PPE_BM_USED_CNT_TBL_ENTRIES: c_int = 15;
pub const PPE_BM_USED_CNT_TBL_INC: c_uint = 0x4;

// Counters for PPE buffers used for packets received after pause frame sent.
pub const PPE_BM_REACT_CNT_TBL_ADDR: c_uint = 0x600240;
pub const PPE_BM_REACT_CNT_TBL_ENTRIES: c_int = 15;
pub const PPE_BM_REACT_CNT_TBL_INC: c_uint = 0x4;

pub const PPE_BM_SHARED_GROUP_CFG_ADDR: c_uint = 0x600290;
pub const PPE_BM_SHARED_GROUP_CFG_ENTRIES: c_int = 4;
pub const PPE_BM_SHARED_GROUP_CFG_INC: c_uint = 0x4;

pub const PPE_BM_PORT_FC_CFG_TBL_ADDR: c_uint = 0x601000;
pub const PPE_BM_PORT_FC_CFG_TBL_ENTRIES: c_int = 15;
pub const PPE_BM_PORT_FC_CFG_TBL_INC: c_uint = 0x10;

// The queue base configurations based on destination port,
// service code or CPU code.
//
pub const PPE_UCAST_QUEUE_MAP_TBL_ADDR: c_uint = 0x810000;
pub const PPE_UCAST_QUEUE_MAP_TBL_ENTRIES: c_int = 3072;
pub const PPE_UCAST_QUEUE_MAP_TBL_INC: c_uint = 0x10;

// The queue offset configurations based on RSS hash value.
pub const PPE_UCAST_HASH_MAP_TBL_ADDR: c_uint = 0x830000;
pub const PPE_UCAST_HASH_MAP_TBL_ENTRIES: c_int = 4096;
pub const PPE_UCAST_HASH_MAP_TBL_INC: c_uint = 0x10;

// The queue offset configurations based on PPE internal priority.
pub const PPE_UCAST_PRIORITY_MAP_TBL_ADDR: c_uint = 0x842000;
pub const PPE_UCAST_PRIORITY_MAP_TBL_ENTRIES: c_int = 256;
pub const PPE_UCAST_PRIORITY_MAP_TBL_INC: c_uint = 0x10;

// PPE unicast queue (0-255) configurations.
pub const PPE_AC_UNICAST_QUEUE_CFG_TBL_ADDR: c_uint = 0x848000;
pub const PPE_AC_UNICAST_QUEUE_CFG_TBL_ENTRIES: c_int = 256;
pub const PPE_AC_UNICAST_QUEUE_CFG_TBL_INC: c_uint = 0x10;

// PPE multicast queue (256-299) configurations.
pub const PPE_AC_MULTICAST_QUEUE_CFG_TBL_ADDR: c_uint = 0x84a000;
pub const PPE_AC_MULTICAST_QUEUE_CFG_TBL_ENTRIES: c_int = 44;
pub const PPE_AC_MULTICAST_QUEUE_CFG_TBL_INC: c_uint = 0x10;

// PPE admission control group (0-3) configurations
pub const PPE_AC_GRP_CFG_TBL_ADDR: c_uint = 0x84c000;
pub const PPE_AC_GRP_CFG_TBL_ENTRIES: c_uint = 0x4;
pub const PPE_AC_GRP_CFG_TBL_INC: c_uint = 0x10;

// Counters for packets handled by unicast queues (0-255).
pub const PPE_AC_UNICAST_QUEUE_CNT_TBL_ADDR: c_uint = 0x84e000;
pub const PPE_AC_UNICAST_QUEUE_CNT_TBL_ENTRIES: c_int = 256;
pub const PPE_AC_UNICAST_QUEUE_CNT_TBL_INC: c_uint = 0x10;

// Counters for packets handled by multicast queues (256-299).
pub const PPE_AC_MULTICAST_QUEUE_CNT_TBL_ADDR: c_uint = 0x852000;
pub const PPE_AC_MULTICAST_QUEUE_CNT_TBL_ENTRIES: c_int = 44;
pub const PPE_AC_MULTICAST_QUEUE_CNT_TBL_INC: c_uint = 0x10;

// Table addresses for per-queue enqueue setting.
pub const PPE_ENQ_OPR_TBL_ADDR: c_uint = 0x85c000;
pub const PPE_ENQ_OPR_TBL_ENTRIES: c_int = 300;
pub const PPE_ENQ_OPR_TBL_INC: c_uint = 0x10;

// Unicast drop count includes the possible drops with WRED for the green,
// yellow and red categories.
//
pub const PPE_UNICAST_DROP_CNT_TBL_ADDR: c_uint = 0x9e0000;
pub const PPE_UNICAST_DROP_CNT_TBL_ENTRIES: c_int = 1536;
pub const PPE_UNICAST_DROP_CNT_TBL_INC: c_uint = 0x10;
pub const PPE_UNICAST_DROP_TYPES: c_int = 6;
pub const PPE_UNICAST_DROP_FORCE_OFFSET: c_int = 3;
// There are 16 multicast queues dedicated to CPU port 0. Multicast drop
// count includes the force drop for green, yellow and red category packets.
//
pub const PPE_P0_MULTICAST_DROP_CNT_TBL_ADDR: c_uint = 0x9f0000;
pub const PPE_P0_MULTICAST_DROP_CNT_TBL_ENTRIES: c_int = 48;
pub const PPE_P0_MULTICAST_DROP_CNT_TBL_INC: c_uint = 0x10;
pub const PPE_P0_MULTICAST_QUEUE_NUM: c_int = 16;
// Each PPE physical port has four dedicated multicast queues, providing
// a total of 12 entries per port. The multicast drop count includes forced
// drops for green, yellow, and red category packets.
//
pub const PPE_MULTICAST_QUEUE_PORT_ADDR_INC: c_uint = 0x1000;
pub const PPE_MULTICAST_DROP_CNT_TBL_INC: c_uint = 0x10;
pub const PPE_MULTICAST_DROP_TYPES: c_int = 3;
pub const PPE_MULTICAST_QUEUE_NUM: c_int = 4;
pub const PPE_MULTICAST_DROP_CNT_TBL_ENTRIES: c_int = 12;

