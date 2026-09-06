//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/adreno_gen7_9_0_snapshot.h
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
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2022-2023 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// Block   : ['PRE_CRASHDUMPER', 'GBIF']
// pairs   : 2 (Regs:5), 5 (Regs:38)
//
// Block   : ['BROADCAST', 'CP', 'GRAS', 'GXCLKCTL']
// Block   : ['PC', 'RBBM', 'RDVM', 'UCHE']
// Block   : ['VFD', 'VPC', 'VSC']
// Pipeline: PIPE_NONE
// pairs   : 196 (Regs:1778)
//
// Block   : ['GMUAO', 'GMUCX', 'GMUCX_RAM']
// Pipeline: PIPE_NONE
// pairs   : 134 (Regs:429)
//
// Block   : ['GMUGX']
// Pipeline: PIPE_NONE
// pairs   : 44 (Regs:454)
//
// Block   : ['CX_MISC']
// Pipeline: PIPE_NONE
// pairs   : 7 (Regs:56)
//
// Block   : ['DBGC']
// Pipeline: PIPE_NONE
// pairs   : 19 (Regs:155)
//
// Block   : ['CX_DBGC']
// Pipeline: PIPE_NONE
// pairs   : 7 (Regs:75)
//
// Block   : ['BROADCAST', 'CP', 'CX_DBGC', 'CX_MISC', 'DBGC', 'GBIF']
// Block   : ['GMUAO', 'GMUCX', 'GMUGX', 'GRAS', 'GXCLKCTL', 'PC']
// Block   : ['RBBM', 'RDVM', 'UCHE', 'VFD', 'VPC', 'VSC']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// pairs   : 29 (Regs:573)
//
// Block   : ['BROADCAST', 'CP', 'CX_DBGC', 'CX_MISC', 'DBGC', 'GBIF']
// Block   : ['GMUAO', 'GMUCX', 'GMUGX', 'GRAS', 'GXCLKCTL', 'PC']
// Block   : ['RBBM', 'RDVM', 'UCHE', 'VFD', 'VPC', 'VSC']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_NONE
// pairs   : 29 (Regs:573)
//
// Block   : ['BROADCAST', 'CP', 'CX_DBGC', 'CX_MISC', 'DBGC', 'GBIF']
// Block   : ['GMUAO', 'GMUCX', 'GMUGX', 'GRAS', 'GXCLKCTL', 'PC']
// Block   : ['RBBM', 'RDVM', 'UCHE', 'VFD', 'VPC', 'VSC']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_NONE
// pairs   : 2 (Regs:7)
//
// Block   : ['RB']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// pairs   : 5 (Regs:37)
//
// Block   : ['RB']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// pairs   : 15 (Regs:66)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_HLSQ_STATE
// pairs   : 4 (Regs:28)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_SP_TOP
// pairs   : 10 (Regs:61)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_USPTP
// pairs   : 12 (Regs:62)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_HLSQ_DP_STR
// pairs   : 2 (Regs:5)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_HLSQ_STATE
// pairs   : 1 (Regs:5)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_SP_TOP
// pairs   : 1 (Regs:6)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_USPTP
// pairs   : 2 (Regs:9)
//
// Block   : ['TPL1']
// Pipeline: PIPE_NONE
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_USPTP
// pairs   : 5 (Regs:29)
//
// Block   : ['TPL1']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_USPTP
// pairs   : 1 (Regs:1)
//
// Block   : ['TPL1']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_NONE
// Location: A7XX_USPTP
// pairs   : 1 (Regs:1)
//
// Block   : ['GRAS']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_GRAS
// pairs   : 14 (Regs:293)
//
// Block   : ['GRAS']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_GRAS
// pairs   : 14 (Regs:293)
//
// Block   : ['PC']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_FE
// pairs   : 6 (Regs:31)
//
// Block   : ['PC']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_FE
// pairs   : 6 (Regs:31)
//
// Block   : ['VFD']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_FE
// pairs   : 2 (Regs:236)
//
// Block   : ['VFD']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_FE
// pairs   : 2 (Regs:236)
//
// Block   : ['VPC']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_FE
// pairs   : 2 (Regs:18)
//
// Block   : ['VPC']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_PC_VS
// pairs   : 3 (Regs:30)
//
// Block   : ['VPC']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_VPC_PS
// pairs   : 5 (Regs:76)
//
// Block   : ['VPC']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_FE
// pairs   : 2 (Regs:18)
//
// Block   : ['VPC']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_PC_VS
// pairs   : 3 (Regs:30)
//
// Block   : ['VPC']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_VPC_PS
// pairs   : 5 (Regs:76)
//
// Block   : ['RB']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_PS
// pairs   : 39 (Regs:133)
//
// Block   : ['RB']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_PS
// pairs   : 34 (Regs:100)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_HLSQ_STATE
// pairs   : 29 (Regs:215)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_SP_TOP
// pairs   : 22 (Regs:73)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_USPTP
// pairs   : 16 (Regs:269)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_HLSQ_STATE
// pairs   : 21 (Regs:334)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_HLSQ_DP
// pairs   : 3 (Regs:19)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_SP_TOP
// pairs   : 18 (Regs:77)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_USPTP
// pairs   : 17 (Regs:333)
//
// Block   : ['SP']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_HLSQ_DP_STR
// pairs   : 1 (Regs:6)
//
// Block   : ['SP']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_HLSQ_STATE
// pairs   : 28 (Regs:213)
//
// Block   : ['SP']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_SP_TOP
// pairs   : 21 (Regs:71)
//
// Block   : ['SP']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_USPTP
// pairs   : 16 (Regs:266)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_HLSQ_STATE
// pairs   : 14 (Regs:299)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_HLSQ_DP
// pairs   : 2 (Regs:13)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_SP_TOP
// pairs   : 9 (Regs:34)
//
// Block   : ['SP']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_USPTP
// pairs   : 11 (Regs:279)
//
// Block   : ['TPL1']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_USPTP
// pairs   : 3 (Regs:10)
//
// Block   : ['TPL1']
// Pipeline: PIPE_BR
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_USPTP
// pairs   : 6 (Regs:42)
//
// Block   : ['TPL1']
// Pipeline: PIPE_BV
// Cluster : A7XX_CLUSTER_SP_VS
// Location: A7XX_USPTP
// pairs   : 3 (Regs:10)
//
// Block   : ['TPL1']
// Pipeline: PIPE_LPAC
// Cluster : A7XX_CLUSTER_SP_PS
// Location: A7XX_USPTP
// pairs   : 5 (Regs:7)
//
