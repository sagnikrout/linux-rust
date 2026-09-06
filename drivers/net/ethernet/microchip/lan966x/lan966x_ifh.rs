//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_ifh.h
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


// SPDX-License-Identifier: GPL-2.0+
// Fields with description (*) should just be cleared upon injection
// IFH is transmitted MSByte first (Highest bit pos sent as MSB of first byte)
//
pub const IFH_LEN: c_int = 7;

// Timestamp for frame
pub const IFH_POS_TIMESTAMP: c_int = 192;
// Bypass analyzer with a prefilled IFH
pub const IFH_POS_BYPASS: c_int = 191;
// Masqueraded injection with masq_port defining logical source port
pub const IFH_POS_MASQ: c_int = 190;
// Masqueraded port number for injection
pub const IFH_POS_MASQ_PORT: c_int = 186;
// Frame length (*)
pub const IFH_POS_LEN: c_int = 178;
// Cell filling mode. Full(0),Etype(1), LlctOpt(2), Llct(3)
pub const IFH_POS_WRDMODE: c_int = 176;
// Frame has 16 bits rtag removed compared to line data
pub const IFH_POS_RTAG48: c_int = 175;
// Frame has a redundancy tag
pub const IFH_POS_HAS_RED_TAG: c_int = 174;
// Frame has been cut through forwarded (*)
pub const IFH_POS_CUTTHRU: c_int = 173;
// Rewriter command
pub const IFH_POS_REW_CMD: c_int = 163;
// Enable OAM-related rewriting. PDU_TYPE encodes OAM type.
pub const IFH_POS_REW_OAM: c_int = 162;
// PDU type. Encoding: (0-NONE, 1-Y1731_CCM, 2-MRP_TST, 3-MRP_ITST, 4-DLR_BCN,
// 5-DLR_ADV, 6-RTE_NULL_INJ, 7-IPV4, 8-IPV6, 9-Y1731_NON_CCM).
//
pub const IFH_POS_PDU_TYPE: c_int = 158;
// Update FCS before transmission
pub const IFH_POS_FCS_UPD: c_int = 157;
// Classified DSCP value of frame
pub const IFH_POS_DSCP: c_int = 151;
// Yellow indication
pub const IFH_POS_DP: c_int = 150;
// Process in RTE/inbound
pub const IFH_POS_RTE_INB_UPDATE: c_int = 149;
// Number of tags to pop from frame
pub const IFH_POS_POP_CNT: c_int = 147;
// Number of tags in front of the ethertype
pub const IFH_POS_ETYPE_OFS: c_int = 145;
// Logical source port of frame (*)
pub const IFH_POS_SRCPORT: c_int = 141;
// Sequence number in redundancy tag
pub const IFH_POS_SEQ_NUM: c_int = 120;
// Stagd flag and classified TCI of frame (PCP/DEI/VID)
pub const IFH_POS_TCI: c_int = 103;
// Classified internal priority for queuing
pub const IFH_POS_QOS_CLASS: c_int = 100;
// Bit mask with eight cpu copy classes
pub const IFH_POS_CPUQ: c_int = 92;
// Relearn + learn flags (*)
pub const IFH_POS_LEARN_FLAGS: c_int = 90;
// SFLOW identifier for frame (0-8: Tx port, 9: Rx sampling, 15: No sampling)
pub const IFH_POS_SFLOW_ID: c_int = 86;
// Set if an ACL/S2 rule was hit (*).
// Super priority: acl_hit=0 and acl_hit(4)=1.
//
pub const IFH_POS_ACL_HIT: c_int = 85;
// S2 rule index hit (*)
pub const IFH_POS_ACL_IDX: c_int = 79;
// ISDX as classified by S1
pub const IFH_POS_ISDX: c_int = 71;
// Destination ports for frame
pub const IFH_POS_DSTS: c_int = 62;
// Storm policer to be applied: None/Uni/Multi/Broad (*)
pub const IFH_POS_FLOOD: c_int = 60;
// Redundancy tag operation
pub const IFH_POS_SEQ_OP: c_int = 58;
// Classified internal priority for resourcemgt, tagging etc
pub const IFH_POS_IPV: c_int = 55;
// Frame is for AFI use
pub const IFH_POS_AFI: c_int = 54;
// Internal aging value (*)
pub const IFH_POS_AGED: c_int = 52;
// RTP Identifier
pub const IFH_POS_RTP_ID: c_int = 42;
// RTP MRPD flow
pub const IFH_POS_RTP_SUBID: c_int = 41;
// Profinet DataStatus or opcua GroupVersion MSB
pub const IFH_POS_PN_DATA_STATUS: c_int = 33;
// Profinet transfer status (1 iff the status is 0)
pub const IFH_POS_PN_TRANSF_STATUS_ZERO: c_int = 32;
// Profinet cycle counter or opcua NetworkMessageNumber
pub const IFH_POS_PN_CC: c_int = 16;
pub const IFH_WID_TIMESTAMP: c_int = 32;
pub const IFH_WID_BYPASS: c_int = 1;
pub const IFH_WID_MASQ: c_int = 1;
pub const IFH_WID_MASQ_PORT: c_int = 4;
pub const IFH_WID_LEN: c_int = 14;
pub const IFH_WID_WRDMODE: c_int = 2;
pub const IFH_WID_RTAG48: c_int = 1;
pub const IFH_WID_HAS_RED_TAG: c_int = 1;
pub const IFH_WID_CUTTHRU: c_int = 1;
pub const IFH_WID_REW_CMD: c_int = 10;
pub const IFH_WID_REW_OAM: c_int = 1;
pub const IFH_WID_PDU_TYPE: c_int = 4;
pub const IFH_WID_FCS_UPD: c_int = 1;
pub const IFH_WID_DSCP: c_int = 6;
pub const IFH_WID_DP: c_int = 1;
pub const IFH_WID_RTE_INB_UPDATE: c_int = 1;
pub const IFH_WID_POP_CNT: c_int = 2;
pub const IFH_WID_ETYPE_OFS: c_int = 2;
pub const IFH_WID_SRCPORT: c_int = 4;
pub const IFH_WID_SEQ_NUM: c_int = 16;
pub const IFH_WID_TCI: c_int = 17;
pub const IFH_WID_QOS_CLASS: c_int = 3;
pub const IFH_WID_CPUQ: c_int = 8;
pub const IFH_WID_LEARN_FLAGS: c_int = 2;
pub const IFH_WID_SFLOW_ID: c_int = 4;
pub const IFH_WID_ACL_HIT: c_int = 1;
pub const IFH_WID_ACL_IDX: c_int = 6;
pub const IFH_WID_ISDX: c_int = 8;
pub const IFH_WID_DSTS: c_int = 9;
pub const IFH_WID_FLOOD: c_int = 2;
pub const IFH_WID_SEQ_OP: c_int = 2;
pub const IFH_WID_IPV: c_int = 3;
pub const IFH_WID_AFI: c_int = 1;
pub const IFH_WID_AGED: c_int = 2;
pub const IFH_WID_RTP_ID: c_int = 10;
pub const IFH_WID_RTP_SUBID: c_int = 1;
pub const IFH_WID_PN_DATA_STATUS: c_int = 8;
pub const IFH_WID_PN_TRANSF_STATUS_ZERO: c_int = 1;
pub const IFH_WID_PN_CC: c_int = 16;
