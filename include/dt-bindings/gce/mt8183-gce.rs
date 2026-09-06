//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gce/mt8183-gce.h
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
// Copyright (c) 2019 MediaTek Inc.
// Author: Bibby Hsieh <bibby.hsieh@mediatek.com>
//
pub const CMDQ_NO_TIMEOUT: c_uint = 0xffffffff;
// GCE HW thread priority
pub const CMDQ_THR_PRIO_LOWEST: c_int = 0;
pub const CMDQ_THR_PRIO_HIGHEST: c_int = 1;
// GCE SUBSYS
pub const SUBSYS_1300XXXX: c_int = 0;
pub const SUBSYS_1400XXXX: c_int = 1;
pub const SUBSYS_1401XXXX: c_int = 2;
pub const SUBSYS_1402XXXX: c_int = 3;
pub const SUBSYS_1502XXXX: c_int = 4;
pub const SUBSYS_1880XXXX: c_int = 5;
pub const SUBSYS_1881XXXX: c_int = 6;
pub const SUBSYS_1882XXXX: c_int = 7;
pub const SUBSYS_1883XXXX: c_int = 8;
pub const SUBSYS_1884XXXX: c_int = 9;
pub const SUBSYS_1000XXXX: c_int = 10;
pub const SUBSYS_1001XXXX: c_int = 11;
pub const SUBSYS_1002XXXX: c_int = 12;
pub const SUBSYS_1003XXXX: c_int = 13;
pub const SUBSYS_1004XXXX: c_int = 14;
pub const SUBSYS_1005XXXX: c_int = 15;
pub const SUBSYS_1020XXXX: c_int = 16;
pub const SUBSYS_1028XXXX: c_int = 17;
pub const SUBSYS_1700XXXX: c_int = 18;
pub const SUBSYS_1701XXXX: c_int = 19;
pub const SUBSYS_1702XXXX: c_int = 20;
pub const SUBSYS_1703XXXX: c_int = 21;
pub const SUBSYS_1800XXXX: c_int = 22;
pub const SUBSYS_1801XXXX: c_int = 23;
pub const SUBSYS_1802XXXX: c_int = 24;
pub const SUBSYS_1804XXXX: c_int = 25;
pub const SUBSYS_1805XXXX: c_int = 26;
pub const SUBSYS_1808XXXX: c_int = 27;
pub const SUBSYS_180aXXXX: c_int = 28;
pub const SUBSYS_180bXXXX: c_int = 29;
pub const CMDQ_EVENT_DISP_RDMA0_SOF: c_int = 0;
pub const CMDQ_EVENT_DISP_RDMA1_SOF: c_int = 1;
pub const CMDQ_EVENT_MDP_RDMA0_SOF: c_int = 2;
pub const CMDQ_EVENT_MDP_RSZ0_SOF: c_int = 4;
pub const CMDQ_EVENT_MDP_RSZ1_SOF: c_int = 5;
pub const CMDQ_EVENT_MDP_TDSHP_SOF: c_int = 6;
pub const CMDQ_EVENT_MDP_WROT0_SOF: c_int = 7;
pub const CMDQ_EVENT_MDP_WDMA0_SOF: c_int = 8;
pub const CMDQ_EVENT_DISP_OVL0_SOF: c_int = 9;
pub const CMDQ_EVENT_DISP_OVL0_2L_SOF: c_int = 10;
pub const CMDQ_EVENT_DISP_OVL1_2L_SOF: c_int = 11;
pub const CMDQ_EVENT_DISP_WDMA0_SOF: c_int = 12;
pub const CMDQ_EVENT_DISP_COLOR0_SOF: c_int = 13;
pub const CMDQ_EVENT_DISP_CCORR0_SOF: c_int = 14;
pub const CMDQ_EVENT_DISP_AAL0_SOF: c_int = 15;
pub const CMDQ_EVENT_DISP_GAMMA0_SOF: c_int = 16;
pub const CMDQ_EVENT_DISP_DITHER0_SOF: c_int = 17;
pub const CMDQ_EVENT_DISP_PWM0_SOF: c_int = 18;
pub const CMDQ_EVENT_DISP_DSI0_SOF: c_int = 19;
pub const CMDQ_EVENT_DISP_DPI0_SOF: c_int = 20;
pub const CMDQ_EVENT_DISP_RSZ_SOF: c_int = 22;
pub const CMDQ_EVENT_MDP_AAL_SOF: c_int = 23;
pub const CMDQ_EVENT_MDP_CCORR_SOF: c_int = 24;
pub const CMDQ_EVENT_DISP_DBI_SOF: c_int = 25;
pub const CMDQ_EVENT_DISP_RDMA0_EOF: c_int = 26;
pub const CMDQ_EVENT_DISP_RDMA1_EOF: c_int = 27;
pub const CMDQ_EVENT_MDP_RDMA0_EOF: c_int = 28;
pub const CMDQ_EVENT_MDP_RSZ0_EOF: c_int = 30;
pub const CMDQ_EVENT_MDP_RSZ1_EOF: c_int = 31;
pub const CMDQ_EVENT_MDP_TDSHP_EOF: c_int = 32;
pub const CMDQ_EVENT_MDP_WROT0_EOF: c_int = 33;
pub const CMDQ_EVENT_MDP_WDMA0_EOF: c_int = 34;
pub const CMDQ_EVENT_DISP_OVL0_EOF: c_int = 35;
pub const CMDQ_EVENT_DISP_OVL0_2L_EOF: c_int = 36;
pub const CMDQ_EVENT_DISP_OVL1_2L_EOF: c_int = 37;
pub const CMDQ_EVENT_DISP_WDMA0_EOF: c_int = 38;
pub const CMDQ_EVENT_DISP_COLOR0_EOF: c_int = 39;
pub const CMDQ_EVENT_DISP_CCORR0_EOF: c_int = 40;
pub const CMDQ_EVENT_DISP_AAL0_EOF: c_int = 41;
pub const CMDQ_EVENT_DISP_GAMMA0_EOF: c_int = 42;
pub const CMDQ_EVENT_DISP_DITHER0_EOF: c_int = 43;
pub const CMDQ_EVENT_DSI0_EOF: c_int = 44;
pub const CMDQ_EVENT_DPI0_EOF: c_int = 45;
pub const CMDQ_EVENT_DISP_RSZ_EOF: c_int = 47;
pub const CMDQ_EVENT_MDP_AAL_EOF: c_int = 48;
pub const CMDQ_EVENT_MDP_CCORR_EOF: c_int = 49;
pub const CMDQ_EVENT_DBI_EOF: c_int = 50;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE0: c_int = 130;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE1: c_int = 131;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE2: c_int = 132;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE3: c_int = 133;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE4: c_int = 134;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE5: c_int = 135;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE6: c_int = 136;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE7: c_int = 137;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE8: c_int = 138;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE9: c_int = 139;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE10: c_int = 140;
pub const CMDQ_EVENT_MUTEX_STREAM_DONE11: c_int = 141;
pub const CMDQ_EVENT_DISP_RDMA0_BUF_UNDERRUN_EVEN: c_int = 142;
pub const CMDQ_EVENT_DISP_RDMA1_BUF_UNDERRUN_EVEN: c_int = 143;
pub const CMDQ_EVENT_DSI0_TE_EVENT: c_int = 144;
pub const CMDQ_EVENT_DSI0_IRQ_EVENT: c_int = 145;
pub const CMDQ_EVENT_DSI0_DONE_EVENT: c_int = 146;
pub const CMDQ_EVENT_DISP_WDMA0_SW_RST_DONE: c_int = 150;
pub const CMDQ_EVENT_MDP_WDMA_SW_RST_DONE: c_int = 151;
pub const CMDQ_EVENT_MDP_WROT0_SW_RST_DONE: c_int = 152;
pub const CMDQ_EVENT_MDP_RDMA0_SW_RST_DONE: c_int = 154;
pub const CMDQ_EVENT_DISP_OVL0_FRAME_RST_DONE_PULE: c_int = 155;
pub const CMDQ_EVENT_DISP_OVL0_2L_FRAME_RST_DONE_ULSE: c_int = 156;
pub const CMDQ_EVENT_DISP_OVL1_2L_FRAME_RST_DONE_ULSE: c_int = 157;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_0: c_int = 257;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_1: c_int = 258;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_2: c_int = 259;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_3: c_int = 260;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_4: c_int = 261;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_5: c_int = 262;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_6: c_int = 263;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_7: c_int = 264;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_8: c_int = 265;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_9: c_int = 266;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_10: c_int = 267;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_11: c_int = 268;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_12: c_int = 269;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_13: c_int = 270;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_14: c_int = 271;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_15: c_int = 272;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_16: c_int = 273;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_17: c_int = 274;
pub const CMDQ_EVENT_ISP_FRAME_DONE_P2_18: c_int = 275;
pub const CMDQ_EVENT_AMD_FRAME_DONE: c_int = 276;
pub const CMDQ_EVENT_DVE_DONE: c_int = 277;
pub const CMDQ_EVENT_WMFE_DONE: c_int = 278;
pub const CMDQ_EVENT_RSC_DONE: c_int = 279;
pub const CMDQ_EVENT_MFB_DONE: c_int = 280;
pub const CMDQ_EVENT_WPE_A_DONE: c_int = 281;
pub const CMDQ_EVENT_SPE_B_DONE: c_int = 282;
pub const CMDQ_EVENT_OCC_DONE: c_int = 283;
pub const CMDQ_EVENT_VENC_CMDQ_FRAME_DONE: c_int = 289;
pub const CMDQ_EVENT_JPG_ENC_CMDQ_DONE: c_int = 290;
pub const CMDQ_EVENT_JPG_DEC_CMDQ_DONE: c_int = 291;
pub const CMDQ_EVENT_VENC_CMDQ_MB_DONE: c_int = 292;
pub const CMDQ_EVENT_VENC_CMDQ_128BYTE_DONE: c_int = 293;
pub const CMDQ_EVENT_ISP_FRAME_DONE_A: c_int = 321;
pub const CMDQ_EVENT_ISP_FRAME_DONE_B: c_int = 322;
pub const CMDQ_EVENT_CAMSV0_PASS1_DONE: c_int = 323;
pub const CMDQ_EVENT_CAMSV1_PASS1_DONE: c_int = 324;
pub const CMDQ_EVENT_CAMSV2_PASS1_DONE: c_int = 325;
pub const CMDQ_EVENT_TSF_DONE: c_int = 326;
pub const CMDQ_EVENT_SENINF_CAM0_FIFO_FULL: c_int = 327;
pub const CMDQ_EVENT_SENINF_CAM1_FIFO_FULL: c_int = 328;
pub const CMDQ_EVENT_SENINF_CAM2_FIFO_FULL: c_int = 329;
pub const CMDQ_EVENT_SENINF_CAM3_FIFO_FULL: c_int = 330;
pub const CMDQ_EVENT_SENINF_CAM4_FIFO_FULL: c_int = 331;
pub const CMDQ_EVENT_SENINF_CAM5_FIFO_FULL: c_int = 332;
pub const CMDQ_EVENT_SENINF_CAM6_FIFO_FULL: c_int = 333;
pub const CMDQ_EVENT_SENINF_CAM7_FIFO_FULL: c_int = 334;
pub const CMDQ_EVENT_IPU_CORE0_DONE0: c_int = 353;
pub const CMDQ_EVENT_IPU_CORE0_DONE1: c_int = 354;
pub const CMDQ_EVENT_IPU_CORE0_DONE2: c_int = 355;
pub const CMDQ_EVENT_IPU_CORE0_DONE3: c_int = 356;
pub const CMDQ_EVENT_IPU_CORE1_DONE0: c_int = 385;
pub const CMDQ_EVENT_IPU_CORE1_DONE1: c_int = 386;
pub const CMDQ_EVENT_IPU_CORE1_DONE2: c_int = 387;
pub const CMDQ_EVENT_IPU_CORE1_DONE3: c_int = 388;
