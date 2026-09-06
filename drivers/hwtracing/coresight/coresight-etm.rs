//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-etm.h
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
// Copyright (c) 2014-2015, The Linux Foundation. All rights reserved.
//

//
// Device registers:
// 0x000 - 0x2FC: Trace         registers
// 0x300 - 0x314: Management    registers
// 0x318 - 0xEFC: Trace         registers
//
// Coresight registers
// 0xF00 - 0xF9C: Management    registers
// 0xFA0 - 0xFA4: Management    registers in PFTv1.0
// Trace         registers in PFTv1.1
// 0xFA8 - 0xFFC: Management    registers
//
// Trace registers (0x000-0x2FC)
pub const ETMCR: c_uint = 0x000;
pub const ETMCCR: c_uint = 0x004;
pub const ETMTRIGGER: c_uint = 0x008;
pub const ETMSR: c_uint = 0x010;
pub const ETMSCR: c_uint = 0x014;
pub const ETMTSSCR: c_uint = 0x018;
pub const ETMTECR2: c_uint = 0x01c;
pub const ETMTEEVR: c_uint = 0x020;
pub const ETMTECR1: c_uint = 0x024;
pub const ETMFFLR: c_uint = 0x02c;

pub const ETMSQ12EVR: c_uint = 0x180;
pub const ETMSQ21EVR: c_uint = 0x184;
pub const ETMSQ23EVR: c_uint = 0x188;
pub const ETMSQ31EVR: c_uint = 0x18c;
pub const ETMSQ32EVR: c_uint = 0x190;
pub const ETMSQ13EVR: c_uint = 0x194;
pub const ETMSQR: c_uint = 0x19c;

pub const ETMCIDCMR: c_uint = 0x1bc;
pub const ETMIMPSPEC0: c_uint = 0x1c0;
pub const ETMIMPSPEC1: c_uint = 0x1c4;
pub const ETMIMPSPEC2: c_uint = 0x1c8;
pub const ETMIMPSPEC3: c_uint = 0x1cc;
pub const ETMIMPSPEC4: c_uint = 0x1d0;
pub const ETMIMPSPEC5: c_uint = 0x1d4;
pub const ETMIMPSPEC6: c_uint = 0x1d8;
pub const ETMIMPSPEC7: c_uint = 0x1dc;
pub const ETMSYNCFR: c_uint = 0x1e0;
pub const ETMIDR: c_uint = 0x1e4;
pub const ETMCCER: c_uint = 0x1e8;
pub const ETMEXTINSELR: c_uint = 0x1ec;
pub const ETMTESSEICR: c_uint = 0x1f0;
pub const ETMEIBCR: c_uint = 0x1f4;
pub const ETMTSEVR: c_uint = 0x1f8;
pub const ETMAUXCR: c_uint = 0x1fc;
pub const ETMTRACEIDR: c_uint = 0x200;
pub const ETMVMIDCVR: c_uint = 0x240;
// Management registers (0x300-0x314)
pub const ETMOSLAR: c_uint = 0x300;
pub const ETMOSLSR: c_uint = 0x304;
pub const ETMOSSRR: c_uint = 0x308;
pub const ETMPDCR: c_uint = 0x310;
pub const ETMPDSR: c_uint = 0x314;
pub const ETM_MAX_ADDR_CMP: c_int = 16;
pub const ETM_MAX_CNTR: c_int = 4;
pub const ETM_MAX_CTXID_CMP: c_int = 3;
// Register definition
// ETMCR - 0x00

// ETMCCR - 0x04

// ETMPDCR - 0x310

// ETMTECR1 - 0x024

// ETMCCER - 0x1E8

pub const ETM_SQR_MASK: c_uint = 0x3;
pub const ETM_TRACEID_MASK: c_uint = 0x3f;
pub const ETM_EVENT_MASK: c_uint = 0x1ffff;
pub const ETM_SYNC_MASK: c_uint = 0xfff;
pub const ETM_ALL_MASK: c_uint = 0xffffffff;
pub const ETMSR_PROG_BIT: c_int = 1;

// Resource index A */		\

// Resource index B */		\

//
// struct etm_config - configuration information related to an ETM
// @mode:	controls various modes supported by this ETM/PTM.
// @ctrl:	used in conjunction with @mode.
// @trigger_event: setting for register ETMTRIGGER.
// @startstop_ctrl: setting for register ETMTSSCR.
// @enable_event: setting for register ETMTEEVR.
// @enable_ctrl1: setting for register ETMTECR1.
// @enable_ctrl2: setting for register ETMTECR2.
// @fifofull_level: setting for register ETMFFLR.
// @addr_idx:	index for the address comparator selection.
// @addr_val:	value for address comparator register.
// @addr_acctype: access type for address comparator register.
// @addr_type:	current status of the comparator register.
// @cntr_idx:	index for the counter register selection.
// @cntr_rld_val: reload value of a counter register.
// @cntr_event:	control for counter enable register.
// @cntr_rld_event: value for counter reload event register.
// @cntr_val:	counter value register.
// @seq_12_event: event causing the transition from 1 to 2.
// @seq_21_event: event causing the transition from 2 to 1.
// @seq_23_event: event causing the transition from 2 to 3.
// @seq_31_event: event causing the transition from 3 to 1.
// @seq_32_event: event causing the transition from 3 to 2.
// @seq_13_event: event causing the transition from 1 to 3.
// @seq_curr_state: current value of the sequencer register.
// @ctxid_idx: index for the context ID registers.
// @ctxid_pid: value for the context ID to trigger on.
// @ctxid_mask: mask applicable to all the context IDs.
// @sync_freq:	Synchronisation frequency.
// @timestamp_event: Defines an event that requests the insertion
// of a timestamp into the trace stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etm_config {
    pub mode: u32,
    pub ctrl: u32,
    pub trigger_event: u32,
    pub startstop_ctrl: u32,
    pub enable_event: u32,
    pub enable_ctrl1: u32,
    pub enable_ctrl2: u32,
    pub fifofull_level: u32,
    pub addr_idx: u8,
    pub addr_val: [u32; ETM_MAX_ADDR_CMP],
    pub addr_acctype: [u32; ETM_MAX_ADDR_CMP],
    pub addr_type: [u32; ETM_MAX_ADDR_CMP],
    pub cntr_idx: u8,
    pub cntr_rld_val: [u32; ETM_MAX_CNTR],
    pub cntr_event: [u32; ETM_MAX_CNTR],
    pub cntr_rld_event: [u32; ETM_MAX_CNTR],
    pub cntr_val: [u32; ETM_MAX_CNTR],
    pub seq_12_event: u32,
    pub seq_21_event: u32,
    pub seq_23_event: u32,
    pub seq_31_event: u32,
    pub seq_32_event: u32,
    pub seq_13_event: u32,
    pub seq_curr_state: u32,
    pub ctxid_idx: u8,
    pub ctxid_pid: [u32; ETM_MAX_CTXID_CMP],
    pub ctxid_mask: u32,
    pub sync_freq: u32,
    pub timestamp_event: u32,
}

//
// struct etm_drvdata - specifics associated to an ETM component
// @base:	memory mapped base address for this component.
// @atclk:	optional clock for the core parts of the ETM.
// @csdev:	component vitals needed by the framework.
// @spinlock:	only one at a time pls.
// @cpu:	the cpu this component is affined to.
// @port_size:	port size as reported by ETMCR bit 4-6 and 21.
// @arch:	ETM/PTM version number.
// @use_cpu14:	true if management registers need to be accessed via CP14.
// @sticky_enable: true if ETM base configuration has been done.
// @boot_enable:true if we should start tracing at boot time.
// @os_unlock:	true if access to management registers is allowed.
// @nr_addr_cmp:Number of pairs of address comparators as found in ETMCCR.
// @nr_cntr:	Number of counters as found in ETMCCR bit 13-15.
// @nr_ext_inp:	Number of external input as found in ETMCCR bit 17-19.
// @nr_ext_out:	Number of external output as found in ETMCCR bit 20-22.
// @nr_ctxid_cmp: Number of contextID comparators as found in ETMCCR bit 24-25.
// @etmccr:	value of register ETMCCR.
// @etmccer:	value of register ETMCCER.
// @traceid:	value of the current ID for this component.
// @config:	structure holding configuration parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etm_drvdata {
    pub csa: csdev_access,
    pub atclk: *mut clk,
    pub csdev: *mut coresight_device,
    pub spinlock: spinlock_t,
    pub cpu: c_int,
    pub port_size: c_int,
    pub arch: u8,
    pub use_cp14: bool,
    pub sticky_enable: bool,
    pub boot_enable: bool,
    pub os_unlock: bool,
    pub nr_addr_cmp: u8,
    pub nr_cntr: u8,
    pub nr_ext_inp: u8,
    pub nr_ext_out: u8,
    pub nr_ctxid_cmp: u8,
    pub etmccr: u32,
    pub etmccer: u32,
    pub traceid: u32,
    pub config: etm_config,
}

extern "C" {
    pub fn etm_set_default(config: *mut etm_config);
}
extern "C" {
    pub fn etm_config_trace_mode(config: *mut etm_config);
}
extern "C" {
    pub fn etm_release_trace_id(drvdata: *mut etm_drvdata);
}
