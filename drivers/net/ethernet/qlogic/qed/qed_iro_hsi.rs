//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_iro_hsi.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2019-2021 Marvell International Ltd.
//

// Pstorm LiteL2 queue statistics

// Tstorm LightL2 queue statistics

// Ustorm LiteL2 queue statistics

// Tstorm Eth limit Rx rate

// Mstorm ETH PF queues producers

// Mstorm pf statistics

// Mstorm ETH VF queues producers offset in RAM. Used in default VF zone
// size mode.
//

// Mstorm Integration Test Data

// Mstorm iSCSI RX stats

// Mstorm overlay buffer host address

// Mstorm queue statistics

// Mstorm error level for assert

// Mstorm bdq-external-producer of given BDQ resource ID, BDqueue-id

// TPA agregation timeout in us resolution (on ASIC)

// Control frame's EthType configuration for TX control frame security

// Pstorm pf statistics

// Pstorm FCoE TX stats

// Pstorm Integration Test Data

// Pstorm iSCSI TX stats

// Pstorm overlay buffer host address

// Pstorm LL2 packet duplication configuration. Use pstorm_pkt_dup_cfg
// data type.
//

// Pstorm queue statistics

// Pstorm error level for assert

// Pstorm RDMA queue statistics

// DCQCN Sent Statistics

// Tstorm last parser message

// RSS indirection table entry update command per PF offset in TSTORM PF BAR0.
// Use eth_tstorm_rss_update_data for update.
//

// Macro flag: #define TSTORM_ETH_RSS_UPDATE_GTT_SIZE\
// Tstorm FCoE RX stats

// Tstorm Integration Test Data

// Tstorm iSCSI RX stats

// Tstorm ll2 port statistics

// Tstorm producers

// Tstorm overlay buffer host address

// Tstorm LL2 packet duplication configuration.
// Use tstorm_pkt_dup_cfg data type.
//

// Tstorm port statistics

// Tstorm error level for assert

// Tstorm RDMA queue statistics

// Tstorm RoCE Event Statistics

// Tstorm (reflects M-Storm) bdq-external-producer of given function ID,
// BDqueue-id.
//

// Tstorm cmdq-cons of given command queue-id

// Ustorm Common Queue ring consumer

// Ustorm Event ring consumer

// Ustorm pf statistics

// Ustorm eth queue zone

// Ustorm Final flr cleanup ack

// Ustorm Integration Test Data

// Ustorm iSCSI RX stats

// Ustorm overlay buffer host address

// Ustorm queue statistics

// Ustorm error level for assert

// RoCE CQEs Statistics

// Ustorm cqe producer

// Ustorm grq producer

// Ustorm VF-PF Channel ready flag

// Xstorm queue zone

// Xstorm Integration Test Data

// Xstorm iSCSI TX stats

// Xstorm iWARP rxmit stats

// Xstorm overlay buffer host address

// Xstorm common PQ info

// Xstorm error level for assert

// Ystorm flow control mode. Use enum fw_flow_ctrl_mode

// Ystorm Integration Test Data

// Ystorm iSCSI TX stats

// Ystorm overlay buffer host address

// Ystorm error level for assert

// DCQCN Received Statistics

// RoCE Error Statistics

// Ystorm cqe producer

// Per-chip offsets in iro_arr in dwords
pub const E4_IRO_ARR_OFFSET: c_int = 0;
