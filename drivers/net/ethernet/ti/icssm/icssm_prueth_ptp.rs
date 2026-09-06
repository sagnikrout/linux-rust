//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssm/icssm_prueth_ptp.h
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
//
// Copyright (C) 2020-2021 Texas Instruments Incorporated - https://www.ti.com
//
pub const RX_SYNC_TIMESTAMP_OFFSET_P1: c_uint = 0x8    /* 8 bytes */;
pub const RX_PDELAY_REQ_TIMESTAMP_OFFSET_P1: c_uint = 0x14   /* 12 bytes */;
pub const DISABLE_PTP_FRAME_FORWARDING_CTRL_OFFSET: c_uint = 0x14	/* 1 byte */;
pub const RX_PDELAY_RESP_TIMESTAMP_OFFSET_P1: c_uint = 0x20   /* 12 bytes */;
pub const RX_SYNC_TIMESTAMP_OFFSET_P2: c_uint = 0x2c   /* 12 bytes */;
pub const RX_PDELAY_REQ_TIMESTAMP_OFFSET_P2: c_uint = 0x38   /* 12 bytes */;
pub const RX_PDELAY_RESP_TIMESTAMP_OFFSET_P2: c_uint = 0x44   /* 12 bytes */;
pub const TIMESYNC_DOMAIN_NUMBER_LIST: c_uint = 0x50   /* 2 bytes */;
pub const P1_SMA_LINE_DELAY_OFFSET: c_uint = 0x52   /* 4 bytes */;
pub const P2_SMA_LINE_DELAY_OFFSET: c_uint = 0x56   /* 4 bytes */;
pub const TIMESYNC_SECONDS_COUNT_OFFSET: c_uint = 0x5a   /* 6 bytes */;
pub const TIMESYNC_TC_RCF_OFFSET: c_uint = 0x60   /* 4 bytes */;
pub const DUT_IS_MASTER_OFFSET: c_uint = 0x64   /* 1 byte */;
pub const MASTER_PORT_NUM_OFFSET: c_uint = 0x65   /* 1 byte */;
pub const SYNC_MASTER_MAC_OFFSET: c_uint = 0x66   /* 6 bytes */;
pub const TX_TS_NOTIFICATION_OFFSET_SYNC_P1: c_uint = 0x6c   /* 1 byte */;
pub const TX_TS_NOTIFICATION_OFFSET_PDEL_REQ_P1: c_uint = 0x6d   /* 1 byte */;
pub const TX_TS_NOTIFICATION_OFFSET_PDEL_RES_P1: c_uint = 0x6e   /* 1 byte */;
pub const TX_TS_NOTIFICATION_OFFSET_SYNC_P2: c_uint = 0x6f   /* 1 byte */;
pub const TX_TS_NOTIFICATION_OFFSET_PDEL_REQ_P2: c_uint = 0x70   /* 1 byte */;
pub const TX_TS_NOTIFICATION_OFFSET_PDEL_RES_P2: c_uint = 0x71   /* 1 byte */;
pub const TX_SYNC_TIMESTAMP_OFFSET_P1: c_uint = 0x72   /* 12 bytes */;
pub const TX_PDELAY_REQ_TIMESTAMP_OFFSET_P1: c_uint = 0x7e   /* 12 bytes */;
pub const TX_PDELAY_RESP_TIMESTAMP_OFFSET_P1: c_uint = 0x8a   /* 12 bytes */;
pub const TX_SYNC_TIMESTAMP_OFFSET_P2: c_uint = 0x96   /* 12 bytes */;
pub const TX_PDELAY_REQ_TIMESTAMP_OFFSET_P2: c_uint = 0xa2   /* 12 bytes */;
pub const TX_PDELAY_RESP_TIMESTAMP_OFFSET_P2: c_uint = 0xae   /* 12 bytes */;
pub const TIMESYNC_CTRL_VAR_OFFSET: c_uint = 0xba   /* 1 byte */;
pub const DISABLE_SWITCH_SYNC_RELAY_OFFSET: c_uint = 0xbb   /* 1 byte */;
pub const MII_RX_CORRECTION_OFFSET: c_uint = 0xbc   /* 2 bytes */;
pub const MII_TX_CORRECTION_OFFSET: c_uint = 0xbe   /* 2 bytes */;
pub const TIMESYNC_CMP1_CMP_OFFSET: c_uint = 0xc0   /* 8 bytes */;
pub const TIMESYNC_SYNC0_CMP_OFFSET: c_uint = 0xc8   /* 8 bytes */;
pub const TIMESYNC_CMP1_PERIOD_OFFSET: c_uint = 0xd0   /* 4 bytes */;
pub const TIMESYNC_SYNC0_WIDTH_OFFSET: c_uint = 0xd4   /* 4 bytes */;
pub const SINGLE_STEP_IEP_OFFSET_P1: c_uint = 0xd8   /* 8 bytes */;
pub const SINGLE_STEP_SECONDS_OFFSET_P1: c_uint = 0xe0   /* 8 bytes */;
pub const SINGLE_STEP_IEP_OFFSET_P2: c_uint = 0xe8   /* 8 bytes */;
pub const SINGLE_STEP_SECONDS_OFFSET_P2: c_uint = 0xf0   /* 8 bytes */;
pub const LINK_LOCAL_FRAME_HAS_HSR_TAG: c_uint = 0xf8   /* 1 bytes */;
pub const PTP_PREV_TX_TIMESTAMP_P1: c_uint = 0xf9  /* 8 bytes */;
pub const PTP_PREV_TX_TIMESTAMP_P2: c_uint = 0x101  /* 8 bytes */;
pub const PTP_CLK_IDENTITY_OFFSET: c_uint = 0x109  /* 8 bytes */;
pub const PTP_SCRATCH_MEM: c_uint = 0x111  /* 16 byte */;
pub const PTP_IPV4_UDP_E2E_ENABLE: c_uint = 0x121  /* 1 byte */;
pub const PRUETH_PTP_TS_SIZE: c_int = 12;
pub const PRUETH_PTP_TS_NOTIFY_SIZE: c_int = 1;
pub const PRUETH_PTP_TS_NOTIFY_MASK: c_uint = 0xff;
// Bit definitions for TIMESYNC_CTRL

