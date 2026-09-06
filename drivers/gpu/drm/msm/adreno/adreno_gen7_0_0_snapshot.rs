//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/adreno_gen7_0_0_snapshot.h
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

// Block: GRAS Cluster: A7XX_CLUSTER_GRAS Pipeline: PIPE_BR
// Block: GRAS Cluster: A7XX_CLUSTER_GRAS Pipeline: PIPE_BV
// Block: PC Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BR
// Block: PC Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BV
// Block: RB_RAC Cluster: A7XX_CLUSTER_PS Pipeline: PIPE_BR
// Block: RB_RBP Cluster: A7XX_CLUSTER_PS Pipeline: PIPE_BR
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BR Location: HLSQ_STATE
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_LPAC Location: HLSQ_STATE
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BR Location: HLSQ_DP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_LPAC Location: HLSQ_DP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BR Location: SP_TOP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_LPAC Location: SP_TOP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BR Location: uSPTP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_LPAC Location: uSPTP
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BR Location: HLSQ_STATE
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BV Location: HLSQ_STATE
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BR Location: SP_TOP
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BV Location: SP_TOP
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BR Location: uSPTP
// Block: SP Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BV Location: uSPTP
// Block: TPL1 Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BR
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BV Location: HLSQ_STATE
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BV Location: SP_TOP
// Block: SP Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BV Location: uSPTP
// Block: TPL1 Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_BV
// Block: TPL1 Cluster: A7XX_CLUSTER_SP_PS Pipeline: PIPE_LPAC
// Block: TPL1 Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BR
// Block: TPL1 Cluster: A7XX_CLUSTER_SP_VS Pipeline: PIPE_BV
// Block: VFD Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BR
// Block: VFD Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BV
// Block: VPC Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BR
// Block: VPC Cluster: A7XX_CLUSTER_FE Pipeline: PIPE_BV
// Block: VPC Cluster: A7XX_CLUSTER_PC_VS Pipeline: PIPE_BR
// Block: VPC Cluster: A7XX_CLUSTER_PC_VS Pipeline: PIPE_BV
// Block: VPC Cluster: A7XX_CLUSTER_VPC_PS Pipeline: PIPE_BR
// Block: VPC Cluster: A7XX_CLUSTER_VPC_PS Pipeline: PIPE_BV
// Block: SP Cluster: noncontext Pipeline: PIPE_BR Location: HLSQ_STATE
// Block: SP Cluster: noncontext Pipeline: PIPE_BR Location: SP_TOP
// Block: SP Cluster: noncontext Pipeline: PIPE_BR Location: uSPTP
// Block: SP Cluster: noncontext Pipeline: PIPE_LPAC Location: HLSQ_STATE
// Block: SP Cluster: noncontext Pipeline: PIPE_LPAC Location: SP_TOP
// Block: SP Cluster: noncontext Pipeline: PIPE_LPAC Location: uSPTP
// Block: TPl1 Cluster: noncontext Pipeline: PIPE_NONE
// Block: TPl1 Cluster: noncontext Pipeline: PIPE_BR
// Block: TPl1 Cluster: noncontext Pipeline: PIPE_LPAC
