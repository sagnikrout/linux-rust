//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_ana.h
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
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//

pub const ANA_STORMLIMIT_CFG_RSZ: c_uint = 0x4;

pub const ANA_FLOODING_RSZ: c_uint = 0x4;

pub const ANA_SFLOW_CFG_RSZ: c_uint = 0x4;

pub const ANA_PORT_MODE_RSZ: c_uint = 0x4;

pub const ANA_CUT_THRU_CFG_RSZ: c_uint = 0x4;
pub const ANA_PGID_PGID_RSZ: c_uint = 0x4;

pub const MACACCESS_CMD_IDLE: c_int = 0;
pub const MACACCESS_CMD_LEARN: c_int = 1;
pub const MACACCESS_CMD_FORGET: c_int = 2;
pub const MACACCESS_CMD_AGE: c_int = 3;
pub const MACACCESS_CMD_GET_NEXT: c_int = 4;
pub const MACACCESS_CMD_INIT: c_int = 5;
pub const MACACCESS_CMD_READ: c_int = 6;
pub const MACACCESS_CMD_WRITE: c_int = 7;

pub const ANA_TABLES_VLANACCESS_CMD_IDLE: c_uint = 0x0;
pub const ANA_TABLES_VLANACCESS_CMD_WRITE: c_uint = 0x2;
pub const ANA_TABLES_VLANACCESS_CMD_INIT: c_uint = 0x3;

pub const ANA_TABLES_ENTRYLIM_RSZ: c_uint = 0x4;

pub const SFIDACCESS_CMD_IDLE: c_int = 0;
pub const SFIDACCESS_CMD_READ: c_int = 1;
pub const SFIDACCESS_CMD_WRITE: c_int = 2;
pub const SFIDACCESS_CMD_INIT: c_int = 3;

pub const ANA_MSTI_STATE_RSZ: c_uint = 0x4;
pub const ANA_OAM_UPM_LM_CNT_RSZ: c_uint = 0x4;

pub const ANA_SG_GCL_GS_CONFIG_RSZ: c_uint = 0x4;

pub const ANA_SG_GCL_TI_CONFIG_RSZ: c_uint = 0x4;

pub const ANA_PORT_VLAN_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_DROP_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_QOS_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_VCAP_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_VCAP_S1_KEY_CFG_GSZ: c_uint = 0x100;
pub const ANA_PORT_VCAP_S1_KEY_CFG_RSZ: c_uint = 0x4;

pub const ANA_PORT_VCAP_S2_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_PCP_DEI_MAP_GSZ: c_uint = 0x100;
pub const ANA_PORT_PCP_DEI_MAP_RSZ: c_uint = 0x4;

pub const ANA_PORT_CPU_FWD_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_CPU_FWD_BPDU_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_CPU_FWD_GARP_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_CPU_FWD_CCM_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_PORT_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_POL_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_PTP_CFG_GSZ: c_uint = 0x100;

pub const ANA_PORT_PTP_DLY1_CFG_GSZ: c_uint = 0x100;
pub const ANA_PORT_PTP_DLY2_CFG_GSZ: c_uint = 0x100;
pub const ANA_PORT_SFID_CFG_GSZ: c_uint = 0x100;
pub const ANA_PORT_SFID_CFG_RSZ: c_uint = 0x4;

pub const ANA_PFC_PFC_CFG_GSZ: c_uint = 0x40;

pub const ANA_PFC_PFC_TIMER_GSZ: c_uint = 0x40;
pub const ANA_PFC_PFC_TIMER_RSZ: c_uint = 0x4;
pub const ANA_IPT_OAM_MEP_CFG_GSZ: c_uint = 0x8;

pub const ANA_IPT_IPT_GSZ: c_uint = 0x8;

pub const ANA_PPT_PPT_RSZ: c_uint = 0x4;
pub const ANA_FID_MAP_FID_MAP_RSZ: c_uint = 0x4;

pub const ANA_CPUQ_8021_CFG_RSZ: c_uint = 0x4;

pub const ANA_DSCP_CFG_RSZ: c_uint = 0x4;

pub const ANA_DSCP_REWR_CFG_RSZ: c_uint = 0x4;
pub const ANA_VCAP_RNG_TYPE_CFG_RSZ: c_uint = 0x4;
pub const ANA_VCAP_RNG_VAL_CFG_RSZ: c_uint = 0x4;

pub const ANA_POL_PIR_CFG_GSZ: c_uint = 0x20;

pub const ANA_POL_CIR_CFG_GSZ: c_uint = 0x20;

pub const ANA_POL_MODE_CFG_GSZ: c_uint = 0x20;

pub const ANA_POL_PIR_STATE_GSZ: c_uint = 0x20;
pub const ANA_POL_CIR_STATE_GSZ: c_uint = 0x20;
pub const ANA_POL_STATE_GSZ: c_uint = 0x20;
pub const ANA_POL_FLOWC_RSZ: c_uint = 0x4;

