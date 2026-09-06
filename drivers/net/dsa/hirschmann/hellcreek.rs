//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/hirschmann/hellcreek.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// DSA driver for:
// Hirschmann Hellcreek TSN switch.
//
// Copyright (C) 2019-2021 Linutronix GmbH
// Author Kurt Kanzenbach <kurt@linutronix.de>
//

// Ports:
// - 0: CPU
// - 1: Tunnel
// - 2: TSN front port 1
// - 3: TSN front port 2
// - ...
//
pub const CPU_PORT: c_int = 0;
pub const TUNNEL_PORT: c_int = 1;
pub const HELLCREEK_VLAN_NO_MEMBER: c_uint = 0x0;
pub const HELLCREEK_VLAN_UNTAGGED_MEMBER: c_uint = 0x1;
pub const HELLCREEK_VLAN_TAGGED_MEMBER: c_uint = 0x3;
pub const HELLCREEK_NUM_EGRESS_QUEUES: c_int = 8;
pub const HELLCREEK_DEFAULT_MAX_SDU: c_int = 1536;
// Register definitions

pub const HR_PSEL_PTWSEL_SHIFT: c_int = 4;

pub const HR_PSEL_PRTCWSEL_SHIFT: c_int = 0;

pub const HR_PTCFG_PPRIO_SHIFT: c_int = 4;

pub const HR_PRTCCFG_PCP_TC_MAP_SHIFT: c_int = 0;

pub const HR_PTPRTCCFG_MAXSDU_SHIFT: c_int = 0;

pub const HR_CSEL_SHIFT: c_int = 0;

pub const HR_FDBMDRD_PORTMASK_SHIFT: c_int = 0;

pub const HR_FDBMDRD_AGE_SHIFT: c_int = 4;

pub const HR_FDBMDRD_REPRIO_TC_SHIFT: c_int = 12;

pub const HR_FDBWRM0_PORTMASK_SHIFT: c_int = 0;

pub const HR_FDBWRM0_REPRIO_TC_SHIFT: c_int = 12;

pub const HR_SWCFG_LAS_MODE_SHIFT: c_int = 12;

pub const HR_VIDCFG_VID_SHIFT: c_int = 0;

pub const HR_VIDMBRCFG_P0MBR_SHIFT: c_int = 0;

pub const HR_VIDMBRCFG_P1MBR_SHIFT: c_int = 2;

pub const HR_VIDMBRCFG_P2MBR_SHIFT: c_int = 4;

pub const HR_VIDMBRCFG_P3MBR_SHIFT: c_int = 6;

pub const HR_FEABITS0_FDBBINS_SHIFT: c_int = 4;

pub const HR_FEABITS0_PCNT_SHIFT: c_int = 8;

pub const HR_FEABITS0_MCNT_SHIFT: c_int = 12;

pub const TR_TGDVER_REV_MIN_SHIFT: c_int = 0;

pub const TR_TGDVER_REV_MAJ_SHIFT: c_int = 8;

pub const TR_TGDSEL_TDGSEL_SHIFT: c_int = 0;

pub const TR_TGDCTRL_ADMINGATESTATES_SHIFT: c_int = 8;

pub const TR_ESTCMD_ESTSEC_SHIFT: c_int = 0;

pub const TR_EETCMD_EETSEC_SHIFT: c_int = 0;

pub const TR_GCLDAT_GCLWRGATES_SHIFT: c_int = 0;

pub const TR_GCLCMD_GCLWRADR_SHIFT: c_int = 0;

pub const TR_GCLCMD_INIT_GATE_STATES_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_counter {
    pub offset: u8,
    pub name: *const c_char,
}

// State flags for hellcreek_port_hwtstamp::state
// A structure to hold hardware timestamping information per port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_port_hwtstamp {
// Timestamping state
    pub state: c_ulong,
// Resources for receive timestamping
    pub /: *mut *mut sk_buff_head rx_queue; / For synchronization messages,
// Resources for transmit timestamping
    pub tx_tstamp_start: c_ulong,
    pub tx_skb: *mut sk_buff,
// Current timestamp configuration
    pub tstamp_config: kernel_hwtstamp_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_port {
    pub hellcreek: *mut hellcreek,
    pub vlan_dev_bitmap: *mut c_ulong,
    pub port: c_int,
    pub /: *mut *mut u16 ptcfg; / ptcfg shadow,
    pub counter_values: *mut u64,
// Per-port timestamping resources
    pub port_hwtstamp: hellcreek_port_hwtstamp,
// Per-port Qbv schedule information
    pub current_schedule: *mut tc_taprio_qopt_offload,
    pub schedule_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_fdb_entry {
    pub idx: usize,
    pub mac: [c_uchar; ETH_ALEN],
    pub portmask: u8,
    pub age: u8,
    pub is_obt: u8,
    pub pass_blocked: u8,
    pub is_static: u8,
    pub reprio_tc: u8,
    pub reprio_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek {
    pub pdata: *const hellcreek_platform_data,
    pub dev: *mut device,
    pub ds: *mut dsa_switch,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub overflow_work: delayed_work,
    pub led_is_gm: led_classdev,
    pub led_sync_good: led_classdev,
    pub /: *mut *mut mutex reg_lock; / Switch IP register lock,
    pub /: *mut *mut mutex vlan_lock; / VLAN bitmaps lock,
    pub /: *mut *mut mutex ptp_lock; / PTP IP register lock,
    pub vlan_region: *mut devlink_region,
    pub fdb_region: *mut devlink_region,
    pub base: *mut void __iomem,
    pub ptp_base: *mut void __iomem,
    pub /: *mut *mut u16 swcfg; / swcfg shadow,
    pub /: *mut *mut *mut u8 vidmbrcfg; / vidmbrcfg shadow,
    pub /: *mut *mut u64 seconds; / PTP seconds,
    pub /: *mut *mut u64 last_ts; / Used for overflow detection,
    pub /: *mut *mut u16 status_out; / ptp.status_out shadow,
    pub fdb_entries: usize,
    pub ports: [hellcreek_port; ],
}

// A Qbv schedule can only started up to 8 seconds in the future. If the delta
// between the base time and the current ptp time is larger than 8 seconds, then
// use periodic work to check for the schedule to be started. The delayed work
// cannot be armed directly to $base_time - 8 + X, because for large deltas the
// PTP frequency matters.
//

// Devlink resources
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hellcreek_devlink_resource_id {
    HELLCREEK_DEVLINK_PARAM_ID_NONE,  /* DEVLINK_RESOURCE_ID_PARENT_TOP */
    HELLCREEK_DEVLINK_PARAM_ID_VLAN_TABLE,
    HELLCREEK_DEVLINK_PARAM_ID_FDB_TABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_devlink_vlan_entry {
    pub vid: u16,
    pub member: u16,
}
