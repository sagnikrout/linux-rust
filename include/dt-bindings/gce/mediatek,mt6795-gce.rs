//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gce/mediatek,mt6795-gce.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2023 Collabora Ltd.
// Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
// GCE HW thread priority
pub const CMDQ_THR_PRIO_LOWEST: c_int = 0;
pub const CMDQ_THR_PRIO_NORMAL: c_int = 1;
pub const CMDQ_THR_PRIO_NORMAL_2: c_int = 2;
pub const CMDQ_THR_PRIO_MEDIUM: c_int = 3;
pub const CMDQ_THR_PRIO_MEDIUM_2: c_int = 4;
pub const CMDQ_THR_PRIO_HIGH: c_int = 5;
pub const CMDQ_THR_PRIO_HIGHER: c_int = 6;
pub const CMDQ_THR_PRIO_HIGHEST: c_int = 7;
// GCE SUBSYS
pub const SUBSYS_1300XXXX: c_int = 0;
pub const SUBSYS_1400XXXX: c_int = 1;
pub const SUBSYS_1401XXXX: c_int = 2;
pub const SUBSYS_1402XXXX: c_int = 3;
pub const SUBSYS_1500XXXX: c_int = 4;
pub const SUBSYS_1600XXXX: c_int = 5;
pub const SUBSYS_1700XXXX: c_int = 6;
pub const SUBSYS_1800XXXX: c_int = 7;
pub const SUBSYS_1000XXXX: c_int = 8;
pub const SUBSYS_1001XXXX: c_int = 9;
pub const SUBSYS_1002XXXX: c_int = 10;
pub const SUBSYS_1003XXXX: c_int = 11;
pub const SUBSYS_1004XXXX: c_int = 12;
pub const SUBSYS_1005XXXX: c_int = 13;
pub const SUBSYS_1020XXXX: c_int = 14;
pub const SUBSYS_1021XXXX: c_int = 15;
pub const SUBSYS_1120XXXX: c_int = 16;
pub const SUBSYS_1121XXXX: c_int = 17;
pub const SUBSYS_1122XXXX: c_int = 18;
pub const SUBSYS_1123XXXX: c_int = 19;
pub const SUBSYS_1124XXXX: c_int = 20;
pub const SUBSYS_1125XXXX: c_int = 21;
pub const SUBSYS_1126XXXX: c_int = 22;
// GCE HW EVENT
pub const CMDQ_EVENT_MDP_RDMA0_SOF: c_int = 0;
pub const CMDQ_EVENT_MDP_RDMA1_SOF: c_int = 1;
pub const CMDQ_EVENT_MDP_DSI0_TE_SOF: c_int = 2;
pub const CMDQ_EVENT_MDP_DSI1_TE_SOF: c_int = 3;
pub const CMDQ_EVENT_MDP_MVW_SOF: c_int = 4;
pub const CMDQ_EVENT_MDP_TDSHP0_SOF: c_int = 5;
pub const CMDQ_EVENT_MDP_TDSHP1_SOF: c_int = 6;
pub const CMDQ_EVENT_MDP_WDMA_SOF: c_int = 7;
pub const CMDQ_EVENT_MDP_WROT0_SOF: c_int = 8;
pub const CMDQ_EVENT_MDP_WROT1_SOF: c_int = 9;
pub const CMDQ_EVENT_MDP_CROP_SOF: c_int = 10;
pub const CMDQ_EVENT_DISP_OVL0_SOF: c_int = 11;
pub const CMDQ_EVENT_DISP_OVL1_SOF: c_int = 12;
pub const CMDQ_EVENT_DISP_RDMA0_SOF: c_int = 13;
pub const CMDQ_EVENT_DISP_RDMA1_SOF: c_int = 14;
pub const CMDQ_EVENT_DISP_RDMA2_SOF: c_int = 15;
pub const CMDQ_EVENT_DISP_WDMA0_SOF: c_int = 16;
pub const CMDQ_EVENT_DISP_WDMA1_SOF: c_int = 17;
pub const CMDQ_EVENT_DISP_COLOR0_SOF: c_int = 18;
pub const CMDQ_EVENT_DISP_COLOR1_SOF: c_int = 19;
pub const CMDQ_EVENT_DISP_AAL_SOF: c_int = 20;
pub const CMDQ_EVENT_DISP_GAMMA_SOF: c_int = 21;
pub const CMDQ_EVENT_DISP_UFOE_SOF: c_int = 22;
pub const CMDQ_EVENT_DISP_PWM0_SOF: c_int = 23;
pub const CMDQ_EVENT_DISP_PWM1_SOF: c_int = 24;
pub const CMDQ_EVENT_DISP_OD_SOF: c_int = 25;
pub const CMDQ_EVENT_MDP_RDMA0_EOF: c_int = 26;
pub const CMDQ_EVENT_MDP_RDMA1_EOF: c_int = 27;
pub const CMDQ_EVENT_MDP_RSZ0_EOF: c_int = 28;
pub const CMDQ_EVENT_MDP_RSZ1_EOF: c_int = 29;
pub const CMDQ_EVENT_MDP_RSZ2_EOF: c_int = 30;
pub const CMDQ_EVENT_MDP_TDSHP0_EOF: c_int = 31;
pub const CMDQ_EVENT_MDP_TDSHP1_EOF: c_int = 32;
pub const CMDQ_EVENT_MDP_WDMA_EOF: c_int = 33;
pub const CMDQ_EVENT_MDP_WROT0_WRITE_EOF: c_int = 34;
pub const CMDQ_EVENT_MDP_WROT0_READ_EOF: c_int = 35;
pub const CMDQ_EVENT_MDP_WROT1_WRITE_EOF: c_int = 36;
pub const CMDQ_EVENT_MDP_WROT1_READ_EOF: c_int = 37;
pub const CMDQ_EVENT_MDP_CROP_EOF: c_int = 38;
pub const CMDQ_EVENT_DISP_OVL0_EOF: c_int = 39;
pub const CMDQ_EVENT_DISP_OVL1_EOF: c_int = 40;
pub const CMDQ_EVENT_DISP_RDMA0_EOF: c_int = 41;
pub const CMDQ_EVENT_DISP_RDMA1_EOF: c_int = 42;
pub const CMDQ_EVENT_DISP_RDMA2_EOF: c_int = 43;
pub const CMDQ_EVENT_DISP_WDMA0_EOF: c_int = 44;
pub const CMDQ_EVENT_DISP_WDMA1_EOF: c_int = 45;
pub const CMDQ_EVENT_DISP_COLOR0_EOF: c_int = 46;
pub const CMDQ_EVENT_DISP_COLOR1_EOF: c_int = 47;
pub const CMDQ_EVENT_DISP_AAL_EOF: c_int = 48;
pub const CMDQ_EVENT_DISP_GAMMA_EOF: c_int = 49;
pub const CMDQ_EVENT_DISP_UFOE_EOF: c_int = 50;
pub const CMDQ_EVENT_DISP_DPI0_EOF: c_int = 51;
pub const CMDQ_EVENT_MUTEX0_STREAM_EOF: c_int = 52;
pub const CMDQ_EVENT_MUTEX1_STREAM_EOF: c_int = 53;
pub const CMDQ_EVENT_MUTEX2_STREAM_EOF: c_int = 54;
pub const CMDQ_EVENT_MUTEX3_STREAM_EOF: c_int = 55;
pub const CMDQ_EVENT_MUTEX4_STREAM_EOF: c_int = 56;
pub const CMDQ_EVENT_MUTEX5_STREAM_EOF: c_int = 57;
pub const CMDQ_EVENT_MUTEX6_STREAM_EOF: c_int = 58;
pub const CMDQ_EVENT_MUTEX7_STREAM_EOF: c_int = 59;
pub const CMDQ_EVENT_MUTEX8_STREAM_EOF: c_int = 60;
pub const CMDQ_EVENT_MUTEX9_STREAM_EOF: c_int = 61;
pub const CMDQ_EVENT_DISP_RDMA0_UNDERRUN: c_int = 62;
pub const CMDQ_EVENT_DISP_RDMA1_UNDERRUN: c_int = 63;
pub const CMDQ_EVENT_DISP_RDMA2_UNDERRUN: c_int = 64;
pub const CMDQ_EVENT_ISP_PASS2_2_EOF: c_int = 129;
pub const CMDQ_EVENT_ISP_PASS2_1_EOF: c_int = 130;
pub const CMDQ_EVENT_ISP_PASS2_0_EOF: c_int = 131;
pub const CMDQ_EVENT_ISP_PASS1_1_EOF: c_int = 132;
pub const CMDQ_EVENT_ISP_PASS1_0_EOF: c_int = 133;
pub const CMDQ_EVENT_CAMSV_2_PASS1_EOF: c_int = 134;
pub const CMDQ_EVENT_CAMSV_1_PASS1_EOF: c_int = 135;
pub const CMDQ_EVENT_SENINF_CAM1_2_3_FIFO_FULL: c_int = 136;
pub const CMDQ_EVENT_SENINF_CAM0_FIFO_FULL: c_int = 137;
pub const CMDQ_EVENT_JPGENC_PASS2_EOF: c_int = 257;
pub const CMDQ_EVENT_JPGENC_PASS1_EOF: c_int = 258;
pub const CMDQ_EVENT_JPGDEC_EOF: c_int = 259;
