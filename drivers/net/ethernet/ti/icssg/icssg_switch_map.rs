//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icssg_switch_map.h
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
// Texas Instruments ICSSG Ethernet driver
//
// Copyright (C) 2022 Texas Instruments Incorporated - https://www.ti.com
//
// Ethernet Switch Constants
// if bucket size is changed in firmware then this too should be changed
// because it directly impacts FDB ageing calculation
//

// This is fixed in ICSSG

// Time after which FDB entries are checked for aged out values.
// Values are in nanoseconds
//
pub const FDB_AGEING_TIMEOUT_OFFSET: c_uint = 0x0014;
// Default VLAN tag for Host Port
pub const HOST_PORT_DF_VLAN_OFFSET: c_uint = 0x001C;
// Same as HOST_PORT_DF_VLAN_OFFSET

// Default VLAN tag for P1 Port
pub const P1_PORT_DF_VLAN_OFFSET: c_uint = 0x0020;
// Same as P1_PORT_DF_VLAN_OFFSET

// default VLAN tag for P2 Port
pub const P2_PORT_DF_VLAN_OFFSET: c_uint = 0x0024;
// Same as P2_PORT_DF_VLAN_OFFSET

// VLAN-FID Table offset. 4096 VIDs. 2B per VID = 8KB = 0x2000
pub const VLAN_STATIC_REG_TABLE_OFFSET: c_uint = 0x0100;
// VLAN-FID Table offset for EMAC

// Packet descriptor Q reserved memory
pub const PORT_DESC0_HI: c_uint = 0x2104;
// Packet descriptor Q reserved memory
pub const PORT_DESC0_LO: c_uint = 0x2F6C;
// Packet descriptor Q reserved memory
pub const PORT_DESC1_HI: c_uint = 0x3DD4;
// Packet descriptor Q reserved memory
pub const PORT_DESC1_LO: c_uint = 0x4C3C;
// Packet descriptor Q reserved memory
pub const HOST_DESC0_HI: c_uint = 0x5AA4;
// Packet descriptor Q reserved memory
pub const HOST_DESC0_LO: c_uint = 0x5F0C;
// Packet descriptor Q reserved memory
pub const HOST_DESC1_HI: c_uint = 0x6374;
// Packet descriptor Q reserved memory
pub const HOST_DESC1_LO: c_uint = 0x67DC;
// Special packet descriptor Q reserved memory
pub const HOST_SPPD0: c_uint = 0x7AAC;
// Special acket descriptor Q reserved memory
pub const HOST_SPPD1: c_uint = 0x7EAC;
// IEP count cycle counter
pub const TIMESYNC_FW_WC_CYCLECOUNT_OFFSET: c_uint = 0x83EC;
// IEP count hi roll over count
pub const TIMESYNC_FW_WC_HI_ROLLOVER_COUNT_OFFSET: c_uint = 0x83F4;
// IEP count hi sw counter
pub const TIMESYNC_FW_WC_COUNT_HI_SW_OFFSET_OFFSET: c_uint = 0x83F8;
// Set clock descriptor
pub const TIMESYNC_FW_WC_SETCLOCK_DESC_OFFSET: c_uint = 0x83FC;
// IEP count syncout reduction factor
pub const TIMESYNC_FW_WC_SYNCOUT_REDUCTION_FACTOR_OFFSET: c_uint = 0x843C;
// IEP count syncout reduction counter
pub const TIMESYNC_FW_WC_SYNCOUT_REDUCTION_COUNT_OFFSET: c_uint = 0x8440;
// IEP count syncout start time cycle counter
pub const TIMESYNC_FW_WC_SYNCOUT_START_TIME_CYCLECOUNT_OFFSET: c_uint = 0x8444;
// Control variable to generate SYNC1
pub const TIMESYNC_FW_WC_ISOM_PIN_SIGNAL_EN_OFFSET: c_uint = 0x844C;
// SystemTime Sync0 periodicity
pub const TIMESYNC_FW_ST_SYNCOUT_PERIOD_OFFSET: c_uint = 0x8450;
// pktTxDelay for P1 = link speed dependent p1 mac delay + p1 phy delay
pub const TIMESYNC_FW_WC_PKTTXDELAY_P1_OFFSET: c_uint = 0x8454;
// pktTxDelay for P2 = link speed dependent p2 mac delay + p2 phy delay
pub const TIMESYNC_FW_WC_PKTTXDELAY_P2_OFFSET: c_uint = 0x8458;
// Set clock operation done signal for next task
pub const TIMESYNC_FW_SIG_PNFW_OFFSET: c_uint = 0x845C;
// Set clock operation done signal for next task
pub const TIMESYNC_FW_SIG_TIMESYNCFW_OFFSET: c_uint = 0x8460;
// New list is copied at this time
pub const TAS_CONFIG_CHANGE_TIME: c_uint = 0x000C;
// config change error counter
pub const TAS_CONFIG_CHANGE_ERROR_COUNTER: c_uint = 0x0014;
// TAS List update pending flag
pub const TAS_CONFIG_PENDING: c_uint = 0x0018;
// TAS list update trigger flag
pub const TAS_CONFIG_CHANGE: c_uint = 0x0019;
// List length for new TAS schedule
pub const TAS_ADMIN_LIST_LENGTH: c_uint = 0x001A;
// Currently active TAS list index
pub const TAS_ACTIVE_LIST_INDEX: c_uint = 0x001B;
// Cycle time for the new TAS schedule
pub const TAS_ADMIN_CYCLE_TIME: c_uint = 0x001C;
// Cycle counts remaining till the TAS list update
pub const TAS_CONFIG_CHANGE_CYCLE_COUNT: c_uint = 0x0020;
// Base Flow ID for sending  Packets to Host for Slice0
pub const PSI_L_REGULAR_FLOW_ID_BASE_OFFSET: c_uint = 0x0024;
// Same as PSI_L_REGULAR_FLOW_ID_BASE_OFFSET

// Base Flow ID for sending mgmt and Tx TS to Host for Slice0
pub const PSI_L_MGMT_FLOW_ID_OFFSET: c_uint = 0x0026;
// Same as PSI_L_MGMT_FLOW_ID_OFFSET

// Queue number for Special  Packets written here
pub const SPL_PKT_DEFAULT_PRIORITY: c_uint = 0x0028;
// Express Preemptible Queue Mask
pub const EXPRESS_PRE_EMPTIVE_Q_MASK: c_uint = 0x0029;
// Port1/Port2 Default Queue number for untagged  Packets, only 1B is used
pub const QUEUE_NUM_UNTAGGED: c_uint = 0x002A;
// Stores the table used for priority regeneration. 1B per PCP/Queue
pub const PORT_Q_PRIORITY_REGEN_OFFSET: c_uint = 0x002C;
// For marking Packet as priority/express (this feature is disabled) or
// cut-through/S&F.
//
pub const EXPRESS_PRE_EMPTIVE_Q_MAP: c_uint = 0x0034;
// Stores the table used for priority mapping. 1B per PCP/Queue
pub const PORT_Q_PRIORITY_MAPPING_OFFSET: c_uint = 0x003C;
// Used to notify the FW of the current link speed
pub const PORT_LINK_SPEED_OFFSET: c_uint = 0x00A8;
// 2k memory pointer reserved for default writes by PRU0
pub const DEFAULT_MSMC_Q_OFFSET: c_uint = 0x00AC;
// TAS gate mask for windows list0
pub const TAS_GATE_MASK_LIST0: c_uint = 0x0100;
// TAS gate mask for windows list1
pub const TAS_GATE_MASK_LIST1: c_uint = 0x0350;
// Memory to Enable/Disable Preemption on TX side
pub const PRE_EMPTION_ENABLE_TX: c_uint = 0x05A0;
// Active State of Preemption on TX side
pub const PRE_EMPTION_ACTIVE_TX: c_uint = 0x05A1;
// Memory to Enable/Disable Verify State Machine Preemption
pub const PRE_EMPTION_ENABLE_VERIFY: c_uint = 0x05A2;
// Verify Status of State Machine
pub const PRE_EMPTION_VERIFY_STATUS: c_uint = 0x05A3;
// Non Final Fragment Size supported by Link Partner
pub const PRE_EMPTION_ADD_FRAG_SIZE_REMOTE: c_uint = 0x05A4;
// Non Final Fragment Size supported by Firmware
pub const PRE_EMPTION_ADD_FRAG_SIZE_LOCAL: c_uint = 0x05A6;
// Time in ms the State machine waits for respond Packet
pub const PRE_EMPTION_VERIFY_TIME: c_uint = 0x05A8;
// Memory used for R30 related management commands
pub const MGR_R30_CMD_OFFSET: c_uint = 0x05AC;
// HW Buffer Pool0 base address
pub const BUFFER_POOL_0_ADDR_OFFSET: c_uint = 0x05BC;
// 16B for Host Egress MSMC Q (Pre-emptible) context
pub const HOST_RX_Q_PRE_CONTEXT_OFFSET: c_uint = 0x0684;
// Buffer for 8 FDB entries to be added by 'Add Multiple FDB entries IOCTL'
pub const FDB_CMD_BUFFER: c_uint = 0x0894;
// TAS queue max sdu length list
pub const TAS_QUEUE_MAX_SDU_LIST: c_uint = 0x08FA;
// Used by FW to generate random number with the SEED value
pub const HD_RAND_SEED_OFFSET: c_uint = 0x0934;
// 16B for Host Egress MSMC Q (Express) context
pub const HOST_RX_Q_EXP_CONTEXT_OFFSET: c_uint = 0x0940;
// Start of 32 bits PA_STAT counters
pub const PA_STAT_32b_START_OFFSET: c_uint = 0x0080;
pub const FW_RTU_PKT_DROP: c_uint = 0x0088;
pub const FW_Q0_OVERFLOW: c_uint = 0x0090;
pub const FW_Q1_OVERFLOW: c_uint = 0x0098;
pub const FW_Q2_OVERFLOW: c_uint = 0x00A0;
pub const FW_Q3_OVERFLOW: c_uint = 0x00A8;
pub const FW_Q4_OVERFLOW: c_uint = 0x00B0;
pub const FW_Q5_OVERFLOW: c_uint = 0x00B8;
pub const FW_Q6_OVERFLOW: c_uint = 0x00C0;
pub const FW_Q7_OVERFLOW: c_uint = 0x00C8;
pub const FW_DROPPED_PKT: c_uint = 0x00F8;
pub const FW_RX_ERROR: c_uint = 0x0100;
pub const FW_RX_DS_INVALID: c_uint = 0x0108;
pub const FW_TX_DROPPED_PACKET: c_uint = 0x0110;
pub const FW_TX_TS_DROPPED_PACKET: c_uint = 0x0118;
pub const FW_INF_PORT_DISABLED: c_uint = 0x0120;
pub const FW_INF_SAV: c_uint = 0x0128;
pub const FW_INF_SA_DL: c_uint = 0x0130;
pub const FW_INF_PORT_BLOCKED: c_uint = 0x0138;
pub const FW_INF_DROP_TAGGED: c_uint = 0x0140;
pub const FW_INF_DROP_PRIOTAGGED: c_uint = 0x0148;
pub const FW_INF_DROP_NOTAG: c_uint = 0x0150;
pub const FW_INF_DROP_NOTMEMBER: c_uint = 0x0158;
pub const FW_RX_EOF_SHORT_FRMERR: c_uint = 0x0188;
pub const FW_RX_B0_DROP_EARLY_EOF: c_uint = 0x0190;
pub const FW_TX_JUMBO_FRM_CUTOFF: c_uint = 0x0198;
pub const FW_RX_EXP_FRAG_Q_DROP: c_uint = 0x01A0;
pub const FW_RX_FIFO_OVERRUN: c_uint = 0x01A8;
pub const FW_CUT_THR_PKT: c_uint = 0x01B0;
pub const FW_HOST_RX_PKT_CNT: c_uint = 0x0248;
pub const FW_HOST_TX_PKT_CNT: c_uint = 0x0250;
pub const FW_HOST_EGRESS_Q_PRE_OVERFLOW: c_uint = 0x0258;
pub const FW_HOST_EGRESS_Q_EXP_OVERFLOW: c_uint = 0x0260;
