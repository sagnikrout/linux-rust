//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gce/mt8186-gce.h
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
// Copyright (C) 2022 MediaTek Inc.
// Author: Yongqiang Niu <yongqiang.niu@mediatek.com>
//
// assign timeout 0 also means default
pub const CMDQ_NO_TIMEOUT: c_uint = 0xffffffff;
pub const CMDQ_TIMEOUT_DEFAULT: c_int = 1000;
// GCE thread priority
pub const CMDQ_THR_PRIO_LOWEST: c_int = 0;
pub const CMDQ_THR_PRIO_1: c_int = 1;
pub const CMDQ_THR_PRIO_2: c_int = 2;
pub const CMDQ_THR_PRIO_3: c_int = 3;
pub const CMDQ_THR_PRIO_4: c_int = 4;
pub const CMDQ_THR_PRIO_5: c_int = 5;
pub const CMDQ_THR_PRIO_6: c_int = 6;
pub const CMDQ_THR_PRIO_HIGHEST: c_int = 7;
// CPR count in 32bit register
pub const GCE_CPR_COUNT: c_int = 1312;
// GCE subsys table
pub const SUBSYS_1300XXXX: c_int = 0;
pub const SUBSYS_1400XXXX: c_int = 1;
pub const SUBSYS_1401XXXX: c_int = 2;
pub const SUBSYS_1402XXXX: c_int = 3;
pub const SUBSYS_1502XXXX: c_int = 4;
pub const SUBSYS_1582XXXX: c_int = 5;
pub const SUBSYS_1B00XXXX: c_int = 6;
pub const SUBSYS_1C00XXXX: c_int = 7;
pub const SUBSYS_1C10XXXX: c_int = 8;
pub const SUBSYS_1000XXXX: c_int = 9;
pub const SUBSYS_1001XXXX: c_int = 10;
pub const SUBSYS_1020XXXX: c_int = 11;
pub const SUBSYS_1021XXXX: c_int = 12;
pub const SUBSYS_1022XXXX: c_int = 13;
pub const SUBSYS_1023XXXX: c_int = 14;
pub const SUBSYS_1060XXXX: c_int = 15;
pub const SUBSYS_1602XXXX: c_int = 16;
pub const SUBSYS_1608XXXX: c_int = 17;
pub const SUBSYS_1700XXXX: c_int = 18;
pub const SUBSYS_1701XXXX: c_int = 19;
pub const SUBSYS_1702XXXX: c_int = 20;
pub const SUBSYS_1703XXXX: c_int = 21;
pub const SUBSYS_1706XXXX: c_int = 22;
pub const SUBSYS_1A00XXXX: c_int = 23;
pub const SUBSYS_1A01XXXX: c_int = 24;
pub const SUBSYS_1A02XXXX: c_int = 25;
pub const SUBSYS_1A03XXXX: c_int = 26;
pub const SUBSYS_1A04XXXX: c_int = 27;
pub const SUBSYS_1A05XXXX: c_int = 28;
pub const SUBSYS_1A06XXXX: c_int = 29;
pub const SUBSYS_NO_SUPPORT: c_int = 99;
// GCE General Purpose Register (GPR) support
// Leave note for scenario usage here
//
// GCE: write mask
pub const GCE_GPR_R00: c_uint = 0x00;
pub const GCE_GPR_R01: c_uint = 0x01;
// MDP: P1: JPEG dest
pub const GCE_GPR_R02: c_uint = 0x02;
pub const GCE_GPR_R03: c_uint = 0x03;
// MDP: PQ color
pub const GCE_GPR_R04: c_uint = 0x04;
// MDP: 2D sharpness
pub const GCE_GPR_R05: c_uint = 0x05;
// DISP: poll esd
pub const GCE_GPR_R06: c_uint = 0x06;
pub const GCE_GPR_R07: c_uint = 0x07;
// MDP: P4: 2D sharpness dst
pub const GCE_GPR_R08: c_uint = 0x08;
pub const GCE_GPR_R09: c_uint = 0x09;
// VCU: poll with timeout for GPR timer
pub const GCE_GPR_R10: c_uint = 0x0A;
pub const GCE_GPR_R11: c_uint = 0x0B;
// CMDQ: debug
pub const GCE_GPR_R12: c_uint = 0x0C;
pub const GCE_GPR_R13: c_uint = 0x0D;
// CMDQ: P7: debug
pub const GCE_GPR_R14: c_uint = 0x0E;
pub const GCE_GPR_R15: c_uint = 0x0F;
// GCE hardware events
// VDEC
pub const CMDQ_EVENT_LINE_COUNT_THRESHOLD_INTERRUPT: c_int = 0;
pub const CMDQ_EVENT_VDEC_INT: c_int = 1;
pub const CMDQ_EVENT_VDEC_PAUSE: c_int = 2;
pub const CMDQ_EVENT_VDEC_DEC_ERROR: c_int = 3;
pub const CMDQ_EVENT_MDEC_TIMEOUT: c_int = 4;
pub const CMDQ_EVENT_DRAM_ACCESS_DONE: c_int = 5;
pub const CMDQ_EVENT_INI_FETCH_RDY: c_int = 6;
pub const CMDQ_EVENT_PROCESS_FLAG: c_int = 7;
pub const CMDQ_EVENT_SEARCH_START_CODE_DONE: c_int = 8;
pub const CMDQ_EVENT_REF_REORDER_DONE: c_int = 9;
pub const CMDQ_EVENT_WP_TBLE_DONE: c_int = 10;
pub const CMDQ_EVENT_COUNT_SRAM_CLR_DONE: c_int = 11;
pub const CMDQ_EVENT_GCE_CNT_OP_THRESHOLD: c_int = 15;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_0: c_int = 16;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_1: c_int = 17;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_2: c_int = 18;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_3: c_int = 19;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_4: c_int = 20;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_5: c_int = 21;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_6: c_int = 22;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_7: c_int = 23;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_8: c_int = 24;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_9: c_int = 25;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_10: c_int = 26;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_11: c_int = 27;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_12: c_int = 28;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_13: c_int = 29;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_14: c_int = 30;
pub const CMDQ_EVENT_VDEC_MINI_MDP_EVENT_15: c_int = 31;
pub const CMDQ_EVENT_WPE_GCE_FRAME_DONE: c_int = 32;
// CAM
pub const CMDQ_EVENT_ISP_FRAME_DONE_A: c_int = 65;
pub const CMDQ_EVENT_ISP_FRAME_DONE_B: c_int = 66;
pub const CMDQ_EVENT_CAMSV1_PASS1_DONE: c_int = 70;
pub const CMDQ_EVENT_CAMSV2_PASS1_DONE: c_int = 71;
pub const CMDQ_EVENT_CAMSV3_PASS1_DONE: c_int = 72;
pub const CMDQ_EVENT_MRAW_0_PASS1_DONE: c_int = 73;
pub const CMDQ_EVENT_SENINF_CAM0_FIFO_FULL: c_int = 75;
pub const CMDQ_EVENT_SENINF_CAM1_FIFO_FULL: c_int = 76;
pub const CMDQ_EVENT_SENINF_CAM2_FIFO_FULL: c_int = 77;
pub const CMDQ_EVENT_SENINF_CAM3_FIFO_FULL: c_int = 78;
pub const CMDQ_EVENT_SENINF_CAM4_FIFO_FULL: c_int = 79;
pub const CMDQ_EVENT_SENINF_CAM5_FIFO_FULL: c_int = 80;
pub const CMDQ_EVENT_SENINF_CAM6_FIFO_FULL: c_int = 81;
pub const CMDQ_EVENT_SENINF_CAM7_FIFO_FULL: c_int = 82;
pub const CMDQ_EVENT_SENINF_CAM8_FIFO_FULL: c_int = 83;
pub const CMDQ_EVENT_SENINF_CAM9_FIFO_FULL: c_int = 84;
pub const CMDQ_EVENT_SENINF_CAM10_FIFO_FULL: c_int = 85;
pub const CMDQ_EVENT_SENINF_CAM11_FIFO_FULL: c_int = 86;
pub const CMDQ_EVENT_SENINF_CAM12_FIFO_FULL: c_int = 87;
pub const CMDQ_EVENT_TG_OVRUN_A_INT: c_int = 88;
pub const CMDQ_EVENT_DMA_R1_ERROR_A_INT: c_int = 89;
pub const CMDQ_EVENT_TG_OVRUN_B_INT: c_int = 90;
pub const CMDQ_EVENT_DMA_R1_ERROR_B_INT: c_int = 91;
pub const CMDQ_EVENT_TG_OVRUN_M0_INT: c_int = 94;
pub const CMDQ_EVENT_R1_ERROR_M0_INT: c_int = 95;
pub const CMDQ_EVENT_TG_GRABERR_M0_INT: c_int = 96;
pub const CMDQ_EVENT_TG_GRABERR_A_INT: c_int = 98;
pub const CMDQ_EVENT_CQ_VR_SNAP_A_INT: c_int = 99;
pub const CMDQ_EVENT_TG_GRABERR_B_INT: c_int = 100;
pub const CMDQ_EVENT_CQ_VR_SNAP_B_INT: c_int = 101;
// VENC
pub const CMDQ_EVENT_VENC_CMDQ_FRAME_DONE: c_int = 129;
pub const CMDQ_EVENT_VENC_CMDQ_PAUSE_DONE: c_int = 130;
pub const CMDQ_EVENT_JPGENC_CMDQ_DONE: c_int = 131;
pub const CMDQ_EVENT_VENC_CMDQ_MB_DONE: c_int = 132;
pub const CMDQ_EVENT_VENC_CMDQ_128BYTE_CNT_DONE: c_int = 133;
pub const CMDQ_EVENT_VENC_CMDQ_PPS_DONE: c_int = 136;
pub const CMDQ_EVENT_VENC_CMDQ_SPS_DONE: c_int = 137;
pub const CMDQ_EVENT_VENC_CMDQ_VPS_DONE: c_int = 138;
// IPE
pub const CMDQ_EVENT_FDVT_DONE: c_int = 161;
pub const CMDQ_EVENT_FE_DONE: c_int = 162;
pub const CMDQ_EVENT_RSC_DONE: c_int = 163;
pub const CMDQ_EVENT_DVS_DONE_ASYNC_SHOT: c_int = 164;
pub const CMDQ_EVENT_DVP_DONE_ASYNC_SHOT: c_int = 165;
// IMG2
pub const CMDQ_EVENT_GCE_IMG2_EVENT0: c_int = 193;
pub const CMDQ_EVENT_GCE_IMG2_EVENT1: c_int = 194;
pub const CMDQ_EVENT_GCE_IMG2_EVENT2: c_int = 195;
pub const CMDQ_EVENT_GCE_IMG2_EVENT3: c_int = 196;
pub const CMDQ_EVENT_GCE_IMG2_EVENT4: c_int = 197;
pub const CMDQ_EVENT_GCE_IMG2_EVENT5: c_int = 198;
pub const CMDQ_EVENT_GCE_IMG2_EVENT6: c_int = 199;
pub const CMDQ_EVENT_GCE_IMG2_EVENT7: c_int = 200;
pub const CMDQ_EVENT_GCE_IMG2_EVENT8: c_int = 201;
pub const CMDQ_EVENT_GCE_IMG2_EVENT9: c_int = 202;
pub const CMDQ_EVENT_GCE_IMG2_EVENT10: c_int = 203;
pub const CMDQ_EVENT_GCE_IMG2_EVENT11: c_int = 204;
pub const CMDQ_EVENT_GCE_IMG2_EVENT12: c_int = 205;
pub const CMDQ_EVENT_GCE_IMG2_EVENT13: c_int = 206;
pub const CMDQ_EVENT_GCE_IMG2_EVENT14: c_int = 207;
pub const CMDQ_EVENT_GCE_IMG2_EVENT15: c_int = 208;
pub const CMDQ_EVENT_GCE_IMG2_EVENT16: c_int = 209;
pub const CMDQ_EVENT_GCE_IMG2_EVENT17: c_int = 210;
pub const CMDQ_EVENT_GCE_IMG2_EVENT18: c_int = 211;
pub const CMDQ_EVENT_GCE_IMG2_EVENT19: c_int = 212;
pub const CMDQ_EVENT_GCE_IMG2_EVENT20: c_int = 213;
pub const CMDQ_EVENT_GCE_IMG2_EVENT21: c_int = 214;
pub const CMDQ_EVENT_GCE_IMG2_EVENT22: c_int = 215;
pub const CMDQ_EVENT_GCE_IMG2_EVENT23: c_int = 216;
// IMG1
pub const CMDQ_EVENT_GCE_IMG1_EVENT0: c_int = 225;
pub const CMDQ_EVENT_GCE_IMG1_EVENT1: c_int = 226;
pub const CMDQ_EVENT_GCE_IMG1_EVENT2: c_int = 227;
pub const CMDQ_EVENT_GCE_IMG1_EVENT3: c_int = 228;
pub const CMDQ_EVENT_GCE_IMG1_EVENT4: c_int = 229;
pub const CMDQ_EVENT_GCE_IMG1_EVENT5: c_int = 230;
pub const CMDQ_EVENT_GCE_IMG1_EVENT6: c_int = 231;
pub const CMDQ_EVENT_GCE_IMG1_EVENT7: c_int = 232;
pub const CMDQ_EVENT_GCE_IMG1_EVENT8: c_int = 233;
pub const CMDQ_EVENT_GCE_IMG1_EVENT9: c_int = 234;
pub const CMDQ_EVENT_GCE_IMG1_EVENT10: c_int = 235;
pub const CMDQ_EVENT_GCE_IMG1_EVENT11: c_int = 236;
pub const CMDQ_EVENT_GCE_IMG1_EVENT12: c_int = 237;
pub const CMDQ_EVENT_GCE_IMG1_EVENT13: c_int = 238;
pub const CMDQ_EVENT_GCE_IMG1_EVENT14: c_int = 239;
pub const CMDQ_EVENT_GCE_IMG1_EVENT15: c_int = 240;
pub const CMDQ_EVENT_GCE_IMG1_EVENT16: c_int = 241;
pub const CMDQ_EVENT_GCE_IMG1_EVENT17: c_int = 242;
pub const CMDQ_EVENT_GCE_IMG1_EVENT18: c_int = 243;
pub const CMDQ_EVENT_GCE_IMG1_EVENT19: c_int = 244;
pub const CMDQ_EVENT_GCE_IMG1_EVENT20: c_int = 245;
pub const CMDQ_EVENT_GCE_IMG1_EVENT21: c_int = 246;
pub const CMDQ_EVENT_GCE_IMG1_EVENT22: c_int = 247;
pub const CMDQ_EVENT_GCE_IMG1_EVENT23: c_int = 248;
// MDP
pub const CMDQ_EVENT_MDP_RDMA0_SOF: c_int = 256;
pub const CMDQ_EVENT_MDP_RDMA1_SOF: c_int = 257;
pub const CMDQ_EVENT_MDP_AAL0_SOF: c_int = 258;
pub const CMDQ_EVENT_MDP_AAL1_SOF: c_int = 259;
pub const CMDQ_EVENT_MDP_HDR0_SOF: c_int = 260;
pub const CMDQ_EVENT_MDP_RSZ0_SOF: c_int = 261;
pub const CMDQ_EVENT_MDP_RSZ1_SOF: c_int = 262;
pub const CMDQ_EVENT_MDP_WROT0_SOF: c_int = 263;
pub const CMDQ_EVENT_MDP_WROT1_SOF: c_int = 264;
pub const CMDQ_EVENT_MDP_TDSHP0_SOF: c_int = 265;
pub const CMDQ_EVENT_MDP_TDSHP1_SOF: c_int = 266;
pub const CMDQ_EVENT_IMG_DL_RELAY0_SOF: c_int = 267;
pub const CMDQ_EVENT_IMG_DL_RELAY1_SOF: c_int = 268;
pub const CMDQ_EVENT_MDP_COLOR0_SOF: c_int = 269;
pub const CMDQ_EVENT_MDP_WROT3_FRAME_DONE: c_int = 288;
pub const CMDQ_EVENT_MDP_WROT2_FRAME_DONE: c_int = 289;
pub const CMDQ_EVENT_MDP_WROT1_FRAME_DONE: c_int = 290;
pub const CMDQ_EVENT_MDP_WROT0_FRAME_DONE: c_int = 291;
pub const CMDQ_EVENT_MDP_TDSHP3_FRAME_DONE: c_int = 292;
pub const CMDQ_EVENT_MDP_TDSHP2_FRAME_DONE: c_int = 293;
pub const CMDQ_EVENT_MDP_TDSHP1_FRAME_DONE: c_int = 294;
pub const CMDQ_EVENT_MDP_TDSHP0_FRAME_DONE: c_int = 295;
pub const CMDQ_EVENT_MDP_RSZ3_FRAME_DONE: c_int = 296;
pub const CMDQ_EVENT_MDP_RSZ2_FRAME_DONE: c_int = 297;
pub const CMDQ_EVENT_MDP_RSZ1_FRAME_DONE: c_int = 298;
pub const CMDQ_EVENT_MDP_RSZ0_FRAME_DONE: c_int = 299;
pub const CMDQ_EVENT_MDP_RDMA3_FRAME_DONE: c_int = 300;
pub const CMDQ_EVENT_MDP_RDMA2_FRAME_DONE: c_int = 301;
pub const CMDQ_EVENT_MDP_RDMA1_FRAME_DONE: c_int = 302;
pub const CMDQ_EVENT_MDP_RDMA0_FRAME_DONE: c_int = 303;
pub const CMDQ_EVENT_MDP_HDR1_FRAME_DONE: c_int = 304;
pub const CMDQ_EVENT_MDP_HDR0_FRAME_DONE: c_int = 305;
pub const CMDQ_EVENT_MDP_COLOR0_FRAME_DONE: c_int = 306;
pub const CMDQ_EVENT_MDP_AAL3_FRAME_DONE: c_int = 307;
pub const CMDQ_EVENT_MDP_AAL2_FRAME_DONE: c_int = 308;
pub const CMDQ_EVENT_MDP_AAL1_FRAME_DONE: c_int = 309;
pub const CMDQ_EVENT_MDP_AAL0_FRAME_DONE: c_int = 310;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_0: c_int = 320;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_1: c_int = 321;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_2: c_int = 322;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_3: c_int = 323;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_4: c_int = 324;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_5: c_int = 325;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_6: c_int = 326;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_7: c_int = 327;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_8: c_int = 328;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_9: c_int = 329;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_10: c_int = 330;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_11: c_int = 331;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_12: c_int = 332;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_13: c_int = 333;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_14: c_int = 334;
pub const CMDQ_EVENT_MDP_STREAM_DONE_ENG_EVENT_15: c_int = 335;
pub const CMDQ_EVENT_MDP_WROT3_SW_RST_DONE_ENG_EVENT: c_int = 336;
pub const CMDQ_EVENT_MDP_WROT2_SW_RST_DONE_ENG_EVENT: c_int = 337;
pub const CMDQ_EVENT_MDP_WROT1_SW_RST_DONE_ENG_EVENT: c_int = 338;
pub const CMDQ_EVENT_MDP_WROT0_SW_RST_DONE_ENG_EVENT: c_int = 339;
pub const CMDQ_EVENT_MDP_RDMA3_SW_RST_DONE_ENG_EVENT: c_int = 340;
pub const CMDQ_EVENT_MDP_RDMA2_SW_RST_DONE_ENG_EVENT: c_int = 341;
pub const CMDQ_EVENT_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 342;
pub const CMDQ_EVENT_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 343;
// DISP
pub const CMDQ_EVENT_DISP_OVL0_SOF: c_int = 384;
pub const CMDQ_EVENT_DISP_OVL0_2L_SOF: c_int = 385;
pub const CMDQ_EVENT_DISP_RDMA0_SOF: c_int = 386;
pub const CMDQ_EVENT_DISP_RSZ0_SOF: c_int = 387;
pub const CMDQ_EVENT_DISP_COLOR0_SOF: c_int = 388;
pub const CMDQ_EVENT_DISP_CCORR0_SOF: c_int = 389;
pub const CMDQ_EVENT_DISP_CCORR1_SOF: c_int = 390;
pub const CMDQ_EVENT_DISP_AAL0_SOF: c_int = 391;
pub const CMDQ_EVENT_DISP_GAMMA0_SOF: c_int = 392;
pub const CMDQ_EVENT_DISP_POSTMASK0_SOF: c_int = 393;
pub const CMDQ_EVENT_DISP_DITHER0_SOF: c_int = 394;
pub const CMDQ_EVENT_DISP_CM0_SOF: c_int = 395;
pub const CMDQ_EVENT_DISP_SPR0_SOF: c_int = 396;
pub const CMDQ_EVENT_DISP_DSC_WRAP0_SOF: c_int = 397;
pub const CMDQ_EVENT_DSI0_SOF: c_int = 398;
pub const CMDQ_EVENT_DISP_WDMA0_SOF: c_int = 399;
pub const CMDQ_EVENT_DISP_PWM0_SOF: c_int = 400;
pub const CMDQ_EVENT_DSI0_FRAME_DONE: c_int = 410;
pub const CMDQ_EVENT_DISP_WDMA0_FRAME_DONE: c_int = 411;
pub const CMDQ_EVENT_DISP_SPR0_FRAME_DONE: c_int = 412;
pub const CMDQ_EVENT_DISP_RSZ0_FRAME_DONE: c_int = 413;
pub const CMDQ_EVENT_DISP_RDMA0_FRAME_DONE: c_int = 414;
pub const CMDQ_EVENT_DISP_POSTMASK0_FRAME_DONE: c_int = 415;
pub const CMDQ_EVENT_DISP_OVL0_FRAME_DONE: c_int = 416;
pub const CMDQ_EVENT_DISP_OVL0_2L_FRAME_DONE: c_int = 417;
pub const CMDQ_EVENT_DISP_GAMMA0_FRAME_DONE: c_int = 418;
pub const CMDQ_EVENT_DISP_DSC_WRAP0_CORE0_FRAME_DONE: c_int = 420;
pub const CMDQ_EVENT_DISP_DITHER0_FRAME_DONE: c_int = 421;
pub const CMDQ_EVENT_DISP_COLOR0_FRAME_DONE: c_int = 422;
pub const CMDQ_EVENT_DISP_CM0_FRAME_DONE: c_int = 423;
pub const CMDQ_EVENT_DISP_CCORR1_FRAME_DONE: c_int = 424;
pub const CMDQ_EVENT_DISP_CCORR0_FRAME_DONE: c_int = 425;
pub const CMDQ_EVENT_DISP_AAL0_FRAME_DONE: c_int = 426;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_0: c_int = 434;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_1: c_int = 435;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_2: c_int = 436;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_3: c_int = 437;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_4: c_int = 438;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_5: c_int = 439;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_6: c_int = 440;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_7: c_int = 441;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_8: c_int = 442;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_9: c_int = 443;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_10: c_int = 444;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_11: c_int = 445;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_12: c_int = 446;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_13: c_int = 447;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_14: c_int = 448;
pub const CMDQ_EVENT_DISP_STREAM_DONE_ENG_EVENT_15: c_int = 449;
pub const CMDQ_EVENT_DSI0_TE_ENG_EVENT: c_int = 450;
pub const CMDQ_EVENT_DSI0_IRQ_ENG_EVENT: c_int = 451;
pub const CMDQ_EVENT_DSI0_DONE_ENG_EVENT: c_int = 452;
pub const CMDQ_EVENT_DISP_WDMA0_SW_RST_DONE_ENG_EVENT: c_int = 453;
pub const CMDQ_EVENT_DISP_SMIASSERT_ENG_EVENT: c_int = 454;
pub const CMDQ_EVENT_DISP_POSTMASK0_RST_DONE_ENG_EVENT: c_int = 455;
pub const CMDQ_EVENT_DISP_OVL0_RST_DONE_ENG_EVENT: c_int = 456;
pub const CMDQ_EVENT_DISP_OVL0_2L_RST_DONE_ENG_EVENT: c_int = 457;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_0: c_int = 458;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_1: c_int = 459;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_2: c_int = 460;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_3: c_int = 461;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_4: c_int = 462;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_5: c_int = 463;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_6: c_int = 464;
pub const CMDQ_EVENT_BUF_UNDERRUN_ENG_EVENT_7: c_int = 465;
pub const CMDQ_EVENT_OUT_EVENT_0: c_int = 898;
// CMDQ sw tokens
// Following definitions are gce sw token which may use by clients
// event operation API.
// Note that token 512 to 639 may set secure
//
// end of hw event and begin of sw token
pub const CMDQ_MAX_HW_EVENT: c_int = 512;
// Config thread notify trigger thread
pub const CMDQ_SYNC_TOKEN_CONFIG_DIRTY: c_int = 640;
// Trigger thread notify config thread
pub const CMDQ_SYNC_TOKEN_STREAM_EOF: c_int = 641;
// Block Trigger thread until the ESD check finishes.
pub const CMDQ_SYNC_TOKEN_ESD_EOF: c_int = 642;
pub const CMDQ_SYNC_TOKEN_STREAM_BLOCK: c_int = 643;
// check CABC setup finish
pub const CMDQ_SYNC_TOKEN_CABC_EOF: c_int = 644;
// Notify normal CMDQ there are some secure task done
// MUST NOT CHANGE, this token sync with secure world
//
pub const CMDQ_SYNC_SECURE_THR_EOF: c_int = 647;
// CMDQ use sw token
pub const CMDQ_SYNC_TOKEN_USER_0: c_int = 649;
pub const CMDQ_SYNC_TOKEN_USER_1: c_int = 650;
pub const CMDQ_SYNC_TOKEN_POLL_MONITOR: c_int = 651;
pub const CMDQ_SYNC_TOKEN_TPR_LOCK: c_int = 652;
// ISP sw token
pub const CMDQ_SYNC_TOKEN_MSS: c_int = 665;
pub const CMDQ_SYNC_TOKEN_MSF: c_int = 666;
// DISP sw token
pub const CMDQ_SYNC_TOKEN_SODI: c_int = 671;
// GPR access tokens (for register backup)
// There are 15 32-bit GPR, 3 GPR form a set
// (64-bit for address, 32-bit for value)
// MUST NOT CHANGE, these tokens sync with MDP
//
pub const CMDQ_SYNC_TOKEN_GPR_SET_0: c_int = 700;
pub const CMDQ_SYNC_TOKEN_GPR_SET_1: c_int = 701;
pub const CMDQ_SYNC_TOKEN_GPR_SET_2: c_int = 702;
pub const CMDQ_SYNC_TOKEN_GPR_SET_3: c_int = 703;
pub const CMDQ_SYNC_TOKEN_GPR_SET_4: c_int = 704;
// Resource lock event to control resource in GCE thread
pub const CMDQ_SYNC_RESOURCE_WROT0: c_int = 710;
pub const CMDQ_SYNC_RESOURCE_WROT1: c_int = 711;
// event for gpr timer, used in sleep and poll with timeout
pub const CMDQ_TOKEN_GPR_TIMER_R0: c_int = 994;
pub const CMDQ_TOKEN_GPR_TIMER_R1: c_int = 995;
pub const CMDQ_TOKEN_GPR_TIMER_R2: c_int = 996;
pub const CMDQ_TOKEN_GPR_TIMER_R3: c_int = 997;
pub const CMDQ_TOKEN_GPR_TIMER_R4: c_int = 998;
pub const CMDQ_TOKEN_GPR_TIMER_R5: c_int = 999;
pub const CMDQ_TOKEN_GPR_TIMER_R6: c_int = 1000;
pub const CMDQ_TOKEN_GPR_TIMER_R7: c_int = 1001;
pub const CMDQ_TOKEN_GPR_TIMER_R8: c_int = 1002;
pub const CMDQ_TOKEN_GPR_TIMER_R9: c_int = 1003;
pub const CMDQ_TOKEN_GPR_TIMER_R10: c_int = 1004;
pub const CMDQ_TOKEN_GPR_TIMER_R11: c_int = 1005;
pub const CMDQ_TOKEN_GPR_TIMER_R12: c_int = 1006;
pub const CMDQ_TOKEN_GPR_TIMER_R13: c_int = 1007;
pub const CMDQ_TOKEN_GPR_TIMER_R14: c_int = 1008;
pub const CMDQ_TOKEN_GPR_TIMER_R15: c_int = 1009;
pub const CMDQ_EVENT_MAX: c_uint = 0x3FF;
// CMDQ sw tokens END
