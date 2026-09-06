//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/mediatek/mt8196-gce.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (c) 2025 MediaTek Inc.
//
// GCE Thread Priority
// The GCE core has multiple GCE threads, each of which can independently
// execute its own sequence of instructions.
// However, the GCE threads on the same core cannot run in parallel.
// Different GCE threads can determine thread priority based on the scenario,
// thereby serving different user needs.
//
// Low priority thread is executed when no high priority thread is active.
// Same priority thread is scheduled by round robin.
//
pub const CMDQ_THR_PRIO_LOWEST: c_int = 0;
pub const CMDQ_THR_PRIO_1: c_int = 1;
pub const CMDQ_THR_PRIO_2: c_int = 2;
pub const CMDQ_THR_PRIO_3: c_int = 3;
pub const CMDQ_THR_PRIO_4: c_int = 4;
pub const CMDQ_THR_PRIO_5: c_int = 5;
pub const CMDQ_THR_PRIO_6: c_int = 6;
pub const CMDQ_THR_PRIO_HIGHEST: c_int = 7;
//
// GCE0 Hardware Event IDs
// Different SoCs will have varying numbers of hardware event signals,
// which are sent from the corresponding hardware to the GCE.
// Each hardware event signal corresponds to an event ID in the GCE.
// The CMDQ driver can use the following event ID definitions to allow
// the client driver to use wait and clear APIs provided by CMDQ, enabling
// the GCE to execute operations in the instructions for that event ID.
//
// The event IDs of GCE0 are mainly used by display hardware.
//
// CMDQ_EVENT_DISP0_STREAM_SOF0 ~ 15: 0 ~ 15

// CMDQ_EVENT_DISP0_FRAME_DONE_SEL0 ~ 15: 16 ~ 31

pub const CMDQ_EVENT_DISP0_DISP_WDMA0_TARGET_LINE_END_ENG_EVENT: c_int = 32;
pub const CMDQ_EVENT_DISP0_DISP_WDMA0_SW_RST_DONE_ENG_EVENT: c_int = 33;
pub const CMDQ_EVENT_DISP0_DISP_POSTMASK1_RST_DONE_ENG_EVENT: c_int = 34;
pub const CMDQ_EVENT_DISP0_DISP_POSTMASK0_RST_DONE_ENG_EVENT: c_int = 35;
pub const CMDQ_EVENT_DISP0_DISP_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 36;
// CMDQ_EVENT_DISP0_DISP_MUTEX0_REG_UPDATE_ENG_EVENT0 ~ 15: 37 ~ 52

pub const CMDQ_EVENT_DISP0_DISP_MUTEX0_GET_RELEASE_ENG_EVENT: c_int = 53;
pub const CMDQ_EVENT_DISP0_DISP_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 54;
// CMDQ_EVENT_DISP1_STREAM_SOF0 ~ 15: 55 ~ 70

// CMDQ_EVENT_DISP1_FRAME_DONE_SEL0 ~ 15: 71 ~ 86

// CMDQ_EVENT_DISP1_STREAM_DONE_ENG_EVENT0 ~ 15: 87 ~ 102

// CMDQ_EVENT_DISP1_REG_UPDATE_DONE_ENG_EVENT0 ~ 15: 103 ~ 118

pub const CMDQ_EVENT_DISP1_OCIP_SUBSYS_SRAM_ISOINT_ENG_EVENT: c_int = 119;
pub const CMDQ_EVENT_DISP1_DISP_WDMA4_TARGET_LINE_END_ENG_EVENT: c_int = 120;
pub const CMDQ_EVENT_DISP1_DISP_WDMA4_SW_RST_DONE_ENG_EVENT: c_int = 121;
pub const CMDQ_EVENT_DISP1_DISP_WDMA3_TARGET_LINE_END_ENG_EVENT: c_int = 122;
pub const CMDQ_EVENT_DISP1_DISP_WDMA3_SW_RST_DONE_ENG_EVENT: c_int = 123;
pub const CMDQ_EVENT_DISP1_DISP_WDMA2_TARGET_LINE_END_ENG_EVENT: c_int = 124;
pub const CMDQ_EVENT_DISP1_DISP_WDMA2_SW_RST_DONE_ENG_EVENT: c_int = 125;
pub const CMDQ_EVENT_DISP1_DISP_WDMA1_TARGET_LINE_END_ENG_EVENT: c_int = 126;
pub const CMDQ_EVENT_DISP1_DISP_WDMA1_SW_RST_DONE_ENG_EVENT: c_int = 127;
pub const CMDQ_EVENT_DISP1_DISP_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 128;
pub const CMDQ_EVENT_DISP1_DISP_MUTEX0_GET_RLZ_ENG_EVENT: c_int = 129;
pub const CMDQ_EVENT_DISP1_DISP_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 130;
pub const CMDQ_EVENT_DISP1_DISP_GDMA0_SW_RST_DONE_ENG_EVENT: c_int = 131;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VSYNC_START_ENG_EVENT: c_int = 132;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VSYNC_END_ENG_EVENT: c_int = 133;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VRR_VFP_LAST_SAFE_BLANK_ENG_EVENT: c_int = 134;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VFP_START_ENG_EVENT: c_int = 135;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VFP_LAST_LINE_ENG_EVENT: c_int = 136;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_VDE_END_ENG_EVENT: c_int = 137;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_TRIGGER_LOOP_CLR_ENG_EVENT: c_int = 138;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_TARGET_LINE1_ENG_EVENT: c_int = 139;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_INT_TG_TARGET_LINE0_ENG_EVENT: c_int = 140;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_EXT_TG_VSYNC_START_ENG_EVENT: c_int = 141;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_EXT_TG_VSYNC_END_ENG_EVENT: c_int = 142;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_EXT_TG_VDE_START_ENG_EVENT: c_int = 143;
pub const CMDQ_EVENT_DISP1_DISP_DVO0_DVO_EXT_TG_VDE_END_ENG_EVENT: c_int = 144;
// CMDQ_EVENT_DISP1_DISP_DSI2_ENG_EVENT0 ~ 10: 145 ~ 155

// CMDQ_EVENT_DISP1_DISP_DSI1_ENG_EVENT0 ~ 21: 156 ~ 177

// CMDQ_EVENT_DISP1_DISP_DSI0_ENG_EVENT0 ~ 10: 178 ~ 188

pub const CMDQ_EVENT_DISP1_DISP_DP_INTF1_VSYNC_START_ENG_EVENT: c_int = 189;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF1_VSYNC_END_ENG_EVENT: c_int = 190;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF1_VDE_START_ENG_EVENT: c_int = 191;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF1_VDE_END_ENG_EVENT: c_int = 192;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF1_TARGET_LINE_ENG_EVENT: c_int = 193;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF0_VSYNC_START_ENG_EVENT: c_int = 194;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF0_VSYNC_END_ENG_EVENT: c_int = 195;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF0_VDE_START_ENG_EVENT: c_int = 196;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF0_VDE_END_ENG_EVENT: c_int = 197;
pub const CMDQ_EVENT_DISP1_DISP_DP_INTF0_TARGET_LINE_ENG_EVENT: c_int = 198;
// CMDQ_EVENT_DISP1_BUF_UNDERRUN_ENG_EVENT0 ~ 10: 199 ~ 209

// CMDQ_EVENT_MML0_STREAM_SOF0 ~ 15: 210 ~ 225

// CMDQ_EVENT_MML0_FRAME_DONE_SEL0 ~ 15: 226 ~ 241

// CMDQ_EVENT_MML0_REG_UPDATE_DONE_ENG_EVENT0 ~ 15: 242 ~ 257

pub const CMDQ_EVENT_MML0_MDP_WROT2_SW_RST_DONE_ENG_EVENT: c_int = 258;
pub const CMDQ_EVENT_MML0_MDP_WROT1_SW_RST_DONE_ENG_EVENT: c_int = 259;
pub const CMDQ_EVENT_MML0_MDP_WROT0_SW_RST_DONE_ENG_EVENT: c_int = 260;
pub const CMDQ_EVENT_MML0_MDP_RROT0_SW_RST_DONE_ENG_EVENT: c_int = 261;
pub const CMDQ_EVENT_MML0_MDP_RDMA2_SW_RST_DONE_ENG_EVENT: c_int = 262;
pub const CMDQ_EVENT_MML0_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 263;
pub const CMDQ_EVENT_MML0_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 264;
pub const CMDQ_EVENT_MML0_MDP_MERGE0_SW_RST_DONE_ENG_EVENT: c_int = 265;
pub const CMDQ_EVENT_MML0_DISP_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 266;
pub const CMDQ_EVENT_MML0_DISP_MUTEX0_GET_RLZ_ENG_EVENT: c_int = 267;
// CMDQ_EVENT_MML1_STREAM_SOF0 ~ 15: 268 ~ 283

// CMDQ_EVENT_MML1_FRAME_DONE_SEL0 ~ 15: 284 ~ 299

// CMDQ_EVENT_MML1_REG_UPDATE_DONE_ENG_EVENT0 ~ 15: 300 ~ 315

pub const CMDQ_EVENT_MML1_MDP_WROT2_SW_RST_DONE_ENG_EVENT: c_int = 316;
pub const CMDQ_EVENT_MML1_MDP_WROT1_SW_RST_DONE_ENG_EVENT: c_int = 317;
pub const CMDQ_EVENT_MML1_MDP_WROT0_SW_RST_DONE_ENG_EVENT: c_int = 318;
pub const CMDQ_EVENT_MML1_MDP_RROT0_SW_RST_DONE_ENG_EVENT: c_int = 319;
pub const CMDQ_EVENT_MML1_MDP_RDMA2_SW_RST_DONE_ENG_EVENT: c_int = 320;
pub const CMDQ_EVENT_MML1_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 321;
pub const CMDQ_EVENT_MML1_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 322;
pub const CMDQ_EVENT_MML1_MDP_MERGE0_SW_RST_DONE_ENG_EVENT: c_int = 323;
pub const CMDQ_EVENT_MML1_DISP_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 324;
pub const CMDQ_EVENT_MML1_DISP_MUTEX0_GET_RLZ_ENG_EVENT: c_int = 325;
// CMDQ_EVENT_OVL0_STREAM_SOF0 ~ 15: 326 ~ 341

// CMDQ_EVENT_OVL0_FRAME_DONE_SEL0 ~ 15: 342 ~ 357

pub const CMDQ_EVENT_OVL0_OVL_UFBC_WDMA0_TARGET_LINE_END_ENG_EVENT: c_int = 358;
pub const CMDQ_EVENT_OVL0_OVL_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 359;
// CMDQ_EVENT_OVL0_OVL_MUTEX0_REG_UPDATE_DONE_ENG_EVENT0 ~ 15: 360 ~ 375

pub const CMDQ_EVENT_OVL0_OVL_MUTEX0_GET_RELEASE_ENG_EVENT: c_int = 376;
pub const CMDQ_EVENT_OVL0_OVL_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 377;
pub const CMDQ_EVENT_OVL0_OVL_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 378;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA9_FRAME_RESET_DONE_ENG_EVENT: c_int = 379;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA8_FRAME_RESET_DONE_ENG_EVENT: c_int = 380;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA7_FRAME_RESET_DONE_ENG_EVENT: c_int = 381;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA6_FRAME_RESET_DONE_ENG_EVENT: c_int = 382;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA5_FRAME_RESET_DONE_ENG_EVENT: c_int = 383;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA4_FRAME_RESET_DONE_ENG_EVENT: c_int = 384;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA3_FRAME_RESET_DONE_ENG_EVENT: c_int = 385;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA2_FRAME_RESET_DONE_ENG_EVENT: c_int = 386;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA1_FRAME_RESET_DONE_ENG_EVENT: c_int = 387;
pub const CMDQ_EVENT_OVL0_OVL_EXDMA0_FRAME_RESET_DONE_ENG_EVENT: c_int = 388;
pub const CMDQ_EVENT_OVL0_OVL_DISP_WDMA1_TARGET_LINE_END_ENG_EVENT: c_int = 389;
pub const CMDQ_EVENT_OVL0_OVL_DISP_WDMA1_SW_RST_DONE_END_ENG_EVENT: c_int = 390;
pub const CMDQ_EVENT_OVL0_OVL_DISP_WDMA0_TARGET_LINE_END_ENG_EVENT: c_int = 391;
pub const CMDQ_EVENT_OVL0_OVL_DISP_WDMA0_SW_RST_DONE_END_ENG_EVENT: c_int = 392;
pub const CMDQ_EVENT_OVL0_OVL_BWM0_FRAME_RESET_DONE_ENG_EVENT: c_int = 393;
// CMDQ_EVENT_OVL1_STREAM_SOF0 ~ 15: 394 ~ 409

// CMDQ_EVENT_OVL1_FRAME_DONE_SEL0 ~ 15: 410 ~ 425

pub const CMDQ_EVENT_OVL1_OVL_UFBC_WDMA0_TARGET_LINE_END_ENG_EVENT: c_int = 426;
pub const CMDQ_EVENT_OVL1_OVL_MUTEX0_TIMEOUT_ENG_EVENT: c_int = 427;
// CMDQ_EVENT_OVL1_OVL_MUTEX0_REG_UPDATE_DONE_ENG_EVENT0 ~ 15: 428 ~ 443

pub const CMDQ_EVENT_OVL1_OVL_MUTEX0_GET_RELEASE_ENG_EVENT: c_int = 444;
pub const CMDQ_EVENT_OVL1_OVL_MDP_RDMA1_SW_RST_DONE_ENG_EVENT: c_int = 445;
pub const CMDQ_EVENT_OVL1_OVL_MDP_RDMA0_SW_RST_DONE_ENG_EVENT: c_int = 446;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA9_FRAME_RESET_DONE_ENG_EVENT: c_int = 447;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA8_FRAME_RESET_DONE_ENG_EVENT: c_int = 448;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA7_FRAME_RESET_DONE_ENG_EVENT: c_int = 449;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA6_FRAME_RESET_DONE_ENG_EVENT: c_int = 450;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA5_FRAME_RESET_DONE_ENG_EVENT: c_int = 451;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA4_FRAME_RESET_DONE_ENG_EVENT: c_int = 452;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA3_FRAME_RESET_DONE_ENG_EVENT: c_int = 453;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA2_FRAME_RESET_DONE_ENG_EVENT: c_int = 454;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA1_FRAME_RESET_DONE_ENG_EVENT: c_int = 455;
pub const CMDQ_EVENT_OVL1_OVL_EXDMA0_FRAME_RESET_DONE_ENG_EVENT: c_int = 456;
pub const CMDQ_EVENT_OVL1_OVL_DISP_WDMA1_TARGET_LINE_END_ENG_EVENT: c_int = 457;
pub const CMDQ_EVENT_OVL1_OVL_DISP_WDMA1_SW_RST_DONE_END_ENG_EVENT: c_int = 458;
pub const CMDQ_EVENT_OVL1_OVL_DISP_WDMA0_TARGET_LINE_END_ENG_EVENT: c_int = 459;
pub const CMDQ_EVENT_OVL1_OVL_DISP_WDMA0_SW_RST_DONE_END_ENG_EVENT: c_int = 460;
pub const CMDQ_EVENT_OVL1_OVL_BWM0_FRAME_RESET_DONE_ENG_EVENT: c_int = 461;
pub const CMDQ_EVENT_DPC_DT_DONE0: c_int = 462;
pub const CMDQ_EVENT_DPC_DT_DONE1: c_int = 463;
pub const CMDQ_EVENT_DPC_DT_DONE2_0_MERGE: c_int = 464;
pub const CMDQ_EVENT_DPC_DT_DONE2_1_MERGE: c_int = 465;
pub const CMDQ_EVENT_DPC_DT_DONE2_2_MERGE: c_int = 466;
pub const CMDQ_EVENT_DPC_DT_DONE2_3_MERGE: c_int = 467;
pub const CMDQ_EVENT_DPC_DT_DONE3: c_int = 468;
pub const CMDQ_EVENT_DPC_DT_DONE4_MERGE: c_int = 469;
pub const CMDQ_EVENT_DPC_DT_DONE5: c_int = 470;
pub const CMDQ_EVENT_DPC_DT_DONE6_0_MERGE: c_int = 471;
pub const CMDQ_EVENT_DPC_DT_DONE6_1_MERGE: c_int = 472;
pub const CMDQ_EVENT_DPC_DT_DONE6_2_MERGE: c_int = 473;
pub const CMDQ_EVENT_DPC_DT_DONE6_3_MERGE: c_int = 474;
pub const CMDQ_EVENT_DPC_DT_DONE7: c_int = 475;
pub const CMDQ_EVENT_DPC_DT_DONE32_MERGE: c_int = 476;
pub const CMDQ_EVENT_DPC_DT_DONE33: c_int = 477;
pub const CMDQ_EVENT_DPC_DT_DONE34_0: c_int = 478;
pub const CMDQ_EVENT_DPC_DT_DONE35: c_int = 479;
pub const CMDQ_EVENT_DPC_DISP_SSYS_DT_ERR_ON_BEFORE_OFF: c_int = 480;
pub const CMDQ_EVENT_DPC_DISP_SSYS_DT_ERR_PRETE_BEFORE_ON: c_int = 481;
pub const CMDQ_EVENT_DPC_DISP_DVFS_DT_ERR_ON_BEFORE_OFF: c_int = 482;
pub const CMDQ_EVENT_DPC_DISP_DVFS_DT_ERR_PRETE_BEFORE_ON: c_int = 483;
pub const CMDQ_EVENT_DPC_DISP_SB_DT_ERR_ON_BEFORE_OFF: c_int = 484;
pub const CMDQ_EVENT_DPC_DISP_SB_DT_ERR_PRETE_BEFORE_ON: c_int = 485;
pub const CMDQ_EVENT_DPC_DISP_SW_CONFIG_WHEN_MTCMOS_OFF: c_int = 486;
pub const CMDQ_EVENT_DPC_MML_SSYS_DT_ERR_ON_BEFORE_OFF: c_int = 487;
pub const CMDQ_EVENT_DPC_MML_SSYS_DT_ERR_PRETE_BEFORE_ON: c_int = 488;
pub const CMDQ_EVENT_DPC_MML_DVFS_DT_ERR_ON_BEFORE_OFF: c_int = 489;
pub const CMDQ_EVENT_DPC_MML_DVFS_DT_ERR_PRETE_BEFORE_ON: c_int = 490;
pub const CMDQ_EVENT_DPC_MML_SB_DT_ERR_ON_BEFORE_OFF: c_int = 491;
pub const CMDQ_EVENT_DPC_MML_SB_DT_ERR_PRETE_BEFORE_ON: c_int = 492;
pub const CMDQ_EVENT_DPC_MML_SW_CONFIG_WHEN_MTCMOS_OFF: c_int = 493;
// CMDQ_EVENT_DPTX_DPTX_EVENT0 ~ 3: 494 ~ 497

// CMDQ_EVENT_EDPTX_EDPTX_EVENT0 ~ 1: 498 ~ 499

pub const CMDQ_EVENT_DSI0_TE_I_DSI0_TE_I: c_int = 898;
pub const CMDQ_EVENT_DSI1_TE_I_DSI1_TE_I: c_int = 899;
pub const CMDQ_EVENT_DSI2_TE_I_DSI2_TE_I: c_int = 900;
// CMDQ_EVENT_POWEREVENT_GCE_EVENT_SUBSYS_PWR_ACK0 ~ 23: 901 ~ 924

// CMDQ_EVENT_GCE_EVENT_DPTX_GCE_EVENT_DPTX0 ~ 1: 925 ~ 926

// CMDQ_EVENT_GCE_EVENT_DPTX_P1_GCE_EVENT_DPTX_P10 ~ 1: 927 ~ 928

// CMDQ_EVENT_GCE_EVENT_EDPTX_GCE_EVENT_EDPTX0 ~ 1: 929 ~ 930

pub const CMDQ_EVENT_DSI3_TE_I_DSI3_TE_I: c_int = 931;
pub const CMDQ_EVENT_SPI0_FINISH_EVENT_DSI4_TE_I: c_int = 932;
pub const CMDQ_EVENT_SPI0_EVENT_EVENT_DSI5_TE_I: c_int = 933;
//
// GCE1 Hardware Event IDs
// Different SoCs will have varying numbers of hardware event signals,
// which are sent from the corresponding hardware to the GCE.
// Each hardware event signal corresponds to an event ID in the GCE.
// The CMDQ driver can use the following event ID definitions to allow
// the client driver to use wait and clear APIs provided by CMDQ, enabling
// the GCE to execute operations in the instructions for that event ID.
//
// The event IDs of GCE1 are mainly used by non-display hardware.
//
pub const CMDQ_EVENT_VENC3_VENC_RESERVED: c_int = 0;
pub const CMDQ_EVENT_VENC3_VENC_FRAME_DONE: c_int = 1;
pub const CMDQ_EVENT_VENC3_VENC_PAUSE_DONE: c_int = 2;
pub const CMDQ_EVENT_VENC3_JPGENC_DONE: c_int = 3;
pub const CMDQ_EVENT_VENC3_VENC_MB_DONE: c_int = 4;
pub const CMDQ_EVENT_VENC3_VENC_128BYTE_DONE: c_int = 5;
pub const CMDQ_EVENT_VENC3_JPGDEC_DONE: c_int = 6;
pub const CMDQ_EVENT_VENC3_JPGDEC_C1_DONE: c_int = 7;
pub const CMDQ_EVENT_VENC3_JPGDEC_INSUFF_DONE: c_int = 8;
pub const CMDQ_EVENT_VENC3_JPGDEC_C1_INSUFF_DONE: c_int = 9;
pub const CMDQ_EVENT_VENC3_WP_2ND_STAGE_DONE: c_int = 10;
pub const CMDQ_EVENT_VENC3_WP_3RD_STAGE_DONE: c_int = 11;
pub const CMDQ_EVENT_VENC3_PPS_HEADER_DONE: c_int = 12;
pub const CMDQ_EVENT_VENC3_SPS_HEADER_DONE: c_int = 13;
pub const CMDQ_EVENT_VENC3_VPS_HEADER_DONE: c_int = 14;
pub const CMDQ_EVENT_VENC3_VENC_SLICE_DONE: c_int = 15;
pub const CMDQ_EVENT_VENC3_VENC_SOC_SLICE_DONE: c_int = 16;
pub const CMDQ_EVENT_VENC3_VENC_SOC_FRAME_DONE: c_int = 17;
pub const CMDQ_EVENT_VENC2_VENC_FRAME_DONE: c_int = 33;
pub const CMDQ_EVENT_VENC2_VENC_PAUSE_DONE: c_int = 34;
pub const CMDQ_EVENT_VENC2_JPGENC_DONE: c_int = 35;
pub const CMDQ_EVENT_VENC2_VENC_MB_DONE: c_int = 36;
pub const CMDQ_EVENT_VENC2_VENC_128BYTE_DONE: c_int = 37;
pub const CMDQ_EVENT_VENC2_JPGDEC_DONE: c_int = 38;
pub const CMDQ_EVENT_VENC2_JPGDEC_C1_DONE: c_int = 39;
pub const CMDQ_EVENT_VENC2_JPGDEC_INSUFF_DONE: c_int = 40;
pub const CMDQ_EVENT_VENC2_JPGDEC_C1_INSUFF_DONE: c_int = 41;
pub const CMDQ_EVENT_VENC2_WP_2ND_STAGE_DONE: c_int = 42;
pub const CMDQ_EVENT_VENC2_WP_3RD_STAGE_DONE: c_int = 43;
pub const CMDQ_EVENT_VENC2_PPS_HEADER_DONE: c_int = 44;
pub const CMDQ_EVENT_VENC2_SPS_HEADER_DONE: c_int = 45;
pub const CMDQ_EVENT_VENC2_VPS_HEADER_DONE: c_int = 46;
pub const CMDQ_EVENT_VENC2_VENC_SLICE_DONE: c_int = 47;
pub const CMDQ_EVENT_VENC2_VENC_SOC_SLICE_DONE: c_int = 48;
pub const CMDQ_EVENT_VENC2_VENC_SOC_FRAME_DONE: c_int = 49;
pub const CMDQ_EVENT_VENC1_VENC_FRAME_DONE: c_int = 65;
pub const CMDQ_EVENT_VENC1_VENC_PAUSE_DONE: c_int = 66;
pub const CMDQ_EVENT_VENC1_JPGENC_DONE: c_int = 67;
pub const CMDQ_EVENT_VENC1_VENC_MB_DONE: c_int = 68;
pub const CMDQ_EVENT_VENC1_VENC_128BYTE_DONE: c_int = 69;
pub const CMDQ_EVENT_VENC1_JPGDEC_DONE: c_int = 70;
pub const CMDQ_EVENT_VENC1_JPGDEC_C1_DONE: c_int = 71;
pub const CMDQ_EVENT_VENC1_JPGDEC_INSUFF_DONE: c_int = 72;
pub const CMDQ_EVENT_VENC1_JPGDEC_C1_INSUFF_DONE: c_int = 73;
pub const CMDQ_EVENT_VENC1_WP_2ND_STAGE_DONE: c_int = 74;
pub const CMDQ_EVENT_VENC1_WP_3RD_STAGE_DONE: c_int = 75;
pub const CMDQ_EVENT_VENC1_PPS_HEADER_DONE: c_int = 76;
pub const CMDQ_EVENT_VENC1_SPS_HEADER_DONE: c_int = 77;
pub const CMDQ_EVENT_VENC1_VPS_HEADER_DONE: c_int = 78;
pub const CMDQ_EVENT_VENC1_VENC_SLICE_DONE: c_int = 79;
pub const CMDQ_EVENT_VENC1_VENC_SOC_SLICE_DONE: c_int = 80;
pub const CMDQ_EVENT_VENC1_VENC_SOC_FRAME_DONE: c_int = 81;
pub const CMDQ_EVENT_VDEC1_VDEC_LINE_CNT_INT: c_int = 192;
pub const CMDQ_EVENT_VDEC1_VDEC_INT: c_int = 193;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_2: c_int = 194;
pub const CMDQ_EVENT_VDEC1_VDEC_DEC_ERR: c_int = 195;
pub const CMDQ_EVENT_VDEC1_VDEC_BUSY_OVERFLOW: c_int = 196;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_5: c_int = 197;
pub const CMDQ_EVENT_VDEC1_VDEC_INI_FETCH_RDY: c_int = 198;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_7: c_int = 199;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_8: c_int = 200;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_9: c_int = 201;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_10: c_int = 202;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_11: c_int = 203;
pub const CMDQ_EVENT_VDEC1_VDEC_GCE_CNT_OP_THR: c_int = 207;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_32: c_int = 224;
pub const CMDQ_EVENT_VDEC1_VDEC_LAT_INT: c_int = 225;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_34: c_int = 226;
pub const CMDQ_EVENT_VDEC1_VDEC_LAT_DEC_ERR: c_int = 227;
pub const CMDQ_EVENT_VDEC1_VDEC_LAT_BUSY_OVERFLOW: c_int = 228;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_37: c_int = 229;
pub const CMDQ_EVENT_VDEC1_VDEC_LAT_INI_FETCH_RDY: c_int = 230;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_39: c_int = 231;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_40: c_int = 232;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_41: c_int = 233;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_42: c_int = 234;
pub const CMDQ_EVENT_VDEC1_VDEC1_EVENT_43: c_int = 235;
pub const CMDQ_EVENT_VDEC1_VDEC_LAT_GCE_CNT_OP_THR: c_int = 239;
pub const CMDQ_EVENT_IMG_IMG_EVENT_0: c_int = 256;
// CMDQ_EVENT_IMG_TRAW0_CQ_THR_DONE_TRAW0_0 ~ 5: 257 ~  262

pub const CMDQ_EVENT_IMG_TRAW0_DMA_ERR_EVENT: c_int = 263;
pub const CMDQ_EVENT_IMG_TRAW0_DUMMY_0: c_int = 264;
// CMDQ_EVENT_IMG_TRAW1_CQ_THR_DONE_TRAW0_0 ~ 5: 265 ~ 270

pub const CMDQ_EVENT_IMG_TRAW1_DMA_ERR_EVENT: c_int = 271;
pub const CMDQ_EVENT_IMG_ADL_TILE_DONE_EVENT: c_int = 272;
pub const CMDQ_EVENT_IMG_ADLWR0_TILE_DONE_EVENT: c_int = 273;
pub const CMDQ_EVENT_IMG_ADLWR1_TILE_DONE_EVENT: c_int = 274;
pub const CMDQ_EVENT_IMG_IMGSYS_IPE_ME_DONE: c_int = 275;
pub const CMDQ_EVENT_IMG_IMGSYS_IPE_MMG_DONE: c_int = 276;
// CMDQ_EVENT_IMG_QOF_ACK_EVENT0 ~ 19: 277 ~ 296

// CMDQ_EVENT_IMG_QOF_ON_EVENT0 ~ 4: 297 ~ 301

// CMDQ_EVENT_IMG_QOF_OFF_EVENT0 ~ 4: 302 ~ 306

// CMDQ_EVENT_IMG_QOF_SAVE_EVENT0 ~ 4: 307 ~ 311

// CMDQ_EVENT_IMG_QOF_RESTORE_EVENT0 ~ 4: 312 ~ 316

// CMDQ_EVENT_IMG_DIP_CQ_THR_DONE_P20~5: 317 ~ 322

pub const CMDQ_EVENT_IMG_DIP_DMA_ERR_EVENT: c_int = 323;
pub const CMDQ_EVENT_IMG_DIP_NR_DMA_ERR_EVENT: c_int = 324;
pub const CMDQ_EVENT_IMG_DIP_DUMMY_0: c_int = 325;
pub const CMDQ_EVENT_IMG_WPE_EIS_GCE_FRAME_DONE: c_int = 326;
pub const CMDQ_EVENT_IMG_WPE_EIS_DONE_SYNC_OUT: c_int = 327;
// CMDQ_EVENT_IMG_WPE_EIS_CQ_THR_DONE_P20 ~ 5: 328 ~ 333

// CMDQ_EVENT_IMG_PQDIP_A_CQ_THR_DONE_P20 ~ 5: 334 ~ 339

pub const CMDQ_EVENT_IMG_PQA_DMA_ERR_EVENT: c_int = 340;
// CMDQ_EVENT_IMG_WPE0_DUMMY0~2: 341 ~ 343

pub const CMDQ_EVENT_IMG_OMC_TNR_GCE_FRAME_DONE: c_int = 344;
pub const CMDQ_EVENT_IMG_OMC_TNR_DONE_SYNC_OUT: c_int = 345;
// CMDQ_EVENT_IMG_OMC_TNR_CQ_THR_DONE_P20 ~ 5: 346 ~ 351

// CMDQ_EVENT_IMG_PQDIP_B_CQ_THR_DONE_P20 ~ 5: 352 ~ 357

pub const CMDQ_EVENT_IMG_PQB_DMA_ERR_EVENT: c_int = 358;
// CMDQ_EVENT_IMG_WPE1_DUMMY0 ~ 2: 359 ~ 361

pub const CMDQ_EVENT_IMG_WPE_LITE_GCE_FRAME_DONE: c_int = 362;
pub const CMDQ_EVENT_IMG_WPE_LITE_DONE_SYNC_OUT: c_int = 363;
// CMDQ_EVENT_IMG_WPE_LITE_CQ_THR_DONE_P20 ~ 5: 364 ~ 369

pub const CMDQ_EVENT_IMG_OMC_LITE_GCE_FRAME_DONE: c_int = 370;
pub const CMDQ_EVENT_IMG_OMC_LITE_DONE_SYNC_OUT: c_int = 371;
// CMDQ_EVENT_IMG_OMC_LITE_CQ_THR_DONE_P20 ~ 5: 372 ~ 377

// CMDQ_EVENT_IMG_WPE2_DUMMY0 ~ 2: 378 ~ 380

pub const CMDQ_EVENT_IMG_IMGSYS_IPE_FDVT0_DONE: c_int = 381;
pub const CMDQ_EVENT_IMG_IMG_EVENT_126: c_int = 382;
pub const CMDQ_EVENT_IMG_IMG_EVENT_127: c_int = 383;
pub const CMDQ_EVENT_CAM_CAM_EVENT_0: c_int = 384;
pub const CMDQ_EVENT_CAM_CAM_SUBA_SW_PASS1_DONE: c_int = 385;
pub const CMDQ_EVENT_CAM_CAM_SUBB_SW_PASS1_DONE: c_int = 386;
pub const CMDQ_EVENT_CAM_CAM_SUBC_SW_PASS1_DONE: c_int = 387;
pub const CMDQ_EVENT_CAM_CAM_SUBA_TFMR_PASS1_DONE: c_int = 388;
pub const CMDQ_EVENT_CAM_CAM_SUBB_TFMR_PASS1_DONE: c_int = 389;
pub const CMDQ_EVENT_CAM_CAM_SUBC_TFMR_PASS1_DONE: c_int = 390;
// CMDQ_EVENT_CAM_CAMSV_A_SW_PASS1_DONE0 ~ 3: 391 ~ 394

// CMDQ_EVENT_CAM_CAMSV_B_SW_PASS1_DONE0 ~ 3: 395 ~ 398

// CMDQ_EVENT_CAM_CAMSV_C_SW_PASS1_DONE0 ~ 3: 399 + 402

// CMDQ_EVENT_CAM_CAMSV_D_SW_PASS1_DONE0 ~ 3: 403 ~ 406

// CMDQ_EVENT_CAM_CAMSV_E_SW_PASS1_DONE0 ~ 3: 407 ~ 409

// CMDQ_EVENT_CAM_CAMSV_F_SW_PASS1_DONE0 ~ 3: 411 ~ 413

pub const CMDQ_EVENT_CAM_MRAW0_SW_PASS1_DONE: c_int = 415;
pub const CMDQ_EVENT_CAM_MRAW1_SW_PASS1_DONE: c_int = 416;
pub const CMDQ_EVENT_CAM_MRAW2_SW_PASS1_DONE: c_int = 417;
pub const CMDQ_EVENT_CAM_MRAW3_SW_PASS1_DONE: c_int = 418;
pub const CMDQ_EVENT_CAM_UISP_SW_PASS1_DONE: c_int = 419;
pub const CMDQ_EVENT_CAM_TG_MRAW0_OUT_SOF: c_int = 420;
pub const CMDQ_EVENT_CAM_TG_MRAW1_OUT_SOF: c_int = 421;
pub const CMDQ_EVENT_CAM_TG_MRAW2_OUT_SOF: c_int = 422;
pub const CMDQ_EVENT_CAM_TG_MRAW3_OUT_SOF: c_int = 423;
pub const CMDQ_EVENT_CAM_PDA0_IRQO_EVENT_DONE_D1: c_int = 424;
pub const CMDQ_EVENT_CAM_PDA1_IRQO_EVENT_DONE_D1: c_int = 425;
pub const CMDQ_EVENT_CAM_DPE_DVP_CMQ_EVENT: c_int = 426;
pub const CMDQ_EVENT_CAM_DPE_DVS_CMQ_EVENT: c_int = 427;
pub const CMDQ_EVENT_CAM_DPE_DVFG_CMQ_EVENT: c_int = 428;
pub const CMDQ_EVENT_CAM_CAM_EVENT_45: c_int = 429;
pub const CMDQ_EVENT_CAM_CAM_EVENT_46: c_int = 430;
pub const CMDQ_EVENT_CAM_CAM_EVENT_47: c_int = 431;
pub const CMDQ_EVENT_CAM_CAM_EVENT_48: c_int = 432;
// CMDQ_EVENT_CAM_CAM_SUBA_TG_INT1 ~ 4: 433 ~ 436

// CMDQ_EVENT_CAM_CAM_SUBB_TG_INT1 ~ 4: 437 ~ 440

// CMDQ_EVENT_CAM_CAM_SUBC_TG_INT1 ~ 4: 441 ~ 444

pub const CMDQ_EVENT_CAM_RAW_O_SOF_SUBA: c_int = 445;
pub const CMDQ_EVENT_CAM_RAW_O_SOF_SUBB: c_int = 446;
pub const CMDQ_EVENT_CAM_RAW_O_SOF_SUBC: c_int = 447;
pub const CMDQ_EVENT_CAM_TFMR_RAW_O_SOF_SUBA: c_int = 448;
pub const CMDQ_EVENT_CAM_TFMR_RAW_O_SOF_SUBB: c_int = 449;
pub const CMDQ_EVENT_CAM_TFMR_RAW_O_SOF_SUBC: c_int = 450;
pub const CMDQ_EVENT_CAM_RAW_SEL_SOF_UISP: c_int = 451;
pub const CMDQ_EVENT_CAM_CAM_SUBA_RING_BUFFER_OVERFLOW_INT_IN: c_int = 452;
pub const CMDQ_EVENT_CAM_CAM_SUBB_RING_BUFFER_OVERFLOW_INT_IN: c_int = 453;
pub const CMDQ_EVENT_CAM_CAM_SUBC_RING_BUFFER_OVERFLOW_INT_IN: c_int = 454;
pub const CMDQ_EVENT_CAM_CAM_EVENT_71: c_int = 455;
pub const CMDQ_EVENT_CAM_ADL_WR_FRAME_DONE: c_int = 456;
pub const CMDQ_EVENT_CAM_ADL_RD_FRAME_DONE: c_int = 457;
pub const CMDQ_EVENT_CAM_QOF_RAWA_POWER_ON_EVENT: c_int = 458;
pub const CMDQ_EVENT_CAM_QOF_RAWB_POWER_ON_EVENT: c_int = 459;
pub const CMDQ_EVENT_CAM_QOF_RAWC_POWER_ON_EVENT: c_int = 460;
pub const CMDQ_EVENT_CAM_QOF_RAWA_POWER_OFF_EVENT: c_int = 461;
pub const CMDQ_EVENT_CAM_QOF_RAWB_POWER_OFF_EVENT: c_int = 462;
pub const CMDQ_EVENT_CAM_QOF_RAWC_POWER_OFF_EVENT: c_int = 463;
pub const CMDQ_EVENT_CAM_QOF_RAWA_SAVE_EVENT: c_int = 464;
pub const CMDQ_EVENT_CAM_QOF_RAWB_SAVE_EVENT: c_int = 465;
pub const CMDQ_EVENT_CAM_QOF_RAWC_SAVE_EVENT: c_int = 466;
pub const CMDQ_EVENT_CAM_QOF_RAWA_RESTORE_EVENT: c_int = 467;
pub const CMDQ_EVENT_CAM_QOF_RAWB_RESTORE_EVENT: c_int = 468;
pub const CMDQ_EVENT_CAM_QOF_RAWC_RESTORE_EVENT: c_int = 469;
// CMDQ_EVENT_CAM_QOF_CAM_EVENT0 ~ 11: 470 ~ 481

// CMDQ_EVENT_CAM_SENINF_CFG_DONE_EVENT0 ~ 11: 482 ~ 495

pub const CMDQ_EVENT_CAM_CCU0_TO_GCE_NON_SEC_IRQ: c_int = 496;
pub const CMDQ_EVENT_CAM_CCU0_TO_GCE_SEC_IRQ: c_int = 497;
pub const CMDQ_EVENT_CAM_CCU0_TO_GCE_VM_IRQ: c_int = 498;
pub const CMDQ_EVENT_CAM_CCU0_TO_GCE_EXCH_VM_IRQ: c_int = 499;
pub const CMDQ_EVENT_CAM_CCU1_TO_GCE_NON_SEC_IRQ: c_int = 500;
pub const CMDQ_EVENT_CAM_CCU1_TO_GCE_SEC_IRQ: c_int = 501;
pub const CMDQ_EVENT_CAM_CCU1_TO_GCE_VM_IRQ: c_int = 502;
pub const CMDQ_EVENT_CAM_CCU1_TO_GCE_EXCH_VM_IRQ: c_int = 503;
// CMDQ_EVENT_CAM_I2C_CH2_EVENT0 ~ 4: 504 ~ 509

pub const CMDQ_EVENT_CAM_CAM_EVENT_125: c_int = 509;
pub const CMDQ_EVENT_CAM_CAM_EVENT_126: c_int = 510;
pub const CMDQ_EVENT_CAM_CAM_EVENT_127: c_int = 511;
pub const CMDQ_EVENT_SMI_EVENT_MMINFRA_SMI_MMSRAM_COMM_SMIASSER: c_int = 898;
pub const CMDQ_EVENT_SMI_EVENT_MMINFRA_SMI_MDP_COMM_SMIASSER: c_int = 899;
pub const CMDQ_EVENT_SMI_EVENT_MMINFRA_SMI_DISP_COMM_SMIASSER: c_int = 900;
//
// GCE Software Tokens
// Apart from the event IDs that are already bound to hardware event signals,
// the remaining event IDs can be used as software tokens.
// This allows the client driver to name and operate them independently,
// and their usage is the same as that of hardware events.
//
// Begin of GCE0 software token
// Config thread notify trigger thread
pub const CMDQ_SYNC_TOKEN_CONFIG_DIRTY: c_int = 640;
// Trigger thread notify config thread
pub const CMDQ_SYNC_TOKEN_STREAM_EOF: c_int = 641;
// Block Trigger thread until the ESD check finishes
pub const CMDQ_SYNC_TOKEN_ESD_EOF: c_int = 642;
pub const CMDQ_SYNC_TOKEN_STREAM_BLOCK: c_int = 643;
// Check CABC setup finish
pub const CMDQ_SYNC_TOKEN_CABC_EOF: c_int = 644;
// VFP period token for Msync
pub const CMDQ_SYNC_TOKEN_VFP_PERIOD: c_int = 645;
// Software sync token for dual display
pub const CMDQ_SYNC_TOKEN_CONFIG_DIRTY_1: c_int = 694;
pub const CMDQ_SYNC_TOKEN_STREAM_EOF_1: c_int = 695;
pub const CMDQ_SYNC_TOKEN_ESD_EOF_1: c_int = 696;
pub const CMDQ_SYNC_TOKEN_STREAM_BLOCK_1: c_int = 697;
pub const CMDQ_SYNC_TOKEN_CABC_EOF_1: c_int = 698;
//
// GPR access tokens (for HW register backup)
// There are 15 32-bit GPR, form 3 GPR as a set
// (64-bit for address, 32-bit for value)
//
// CMDQ_SYNC_TOKEN_GPR_SET0 ~ 4: 700 ~ 704
//

pub const CMDQ_SYNC_TOKEN_TE_0: c_int = 705;
pub const CMDQ_SYNC_TOKEN_PREFETCH_TE_0: c_int = 706;
pub const CMDQ_SYNC_TOKEN_VIDLE_POWER_ON: c_int = 707;
pub const CMDQ_SYNC_TOKEN_CHECK_TRIGGER_MERGE: c_int = 708;
// Resource lock event to control resource in GCE thread
pub const CMDQ_SYNC_RESOURCE_WROT0: c_int = 710;
pub const CMDQ_SYNC_RESOURCE_WROT1: c_int = 711;
// Hardware TRACE software token
pub const CMDQ_SYNC_TOKEN_HW_TRACE_WAIT: c_int = 712;
pub const CMDQ_SYNC_TOKEN_HW_TRACE_LOCK: c_int = 713;
// Software sync token for dual display
pub const CMDQ_SYNC_TOKEN_CONFIG_DIRTY_3: c_int = 714;
pub const CMDQ_SYNC_TOKEN_STREAM_EOF_3: c_int = 715;
pub const CMDQ_SYNC_TOKEN_ESD_EOF_3: c_int = 716;
pub const CMDQ_SYNC_TOKEN_STREAM_BLOCK_3: c_int = 717;
pub const CMDQ_SYNC_TOKEN_CABC_EOF_3: c_int = 718;
// End of GCE0 software token
// Begin of GCE1 software token
// CMDQ_SYNC_TOKEN_IMGSYS_POOL0 ~ 300: 512 ~ 812

// ISP software token
pub const CMDQ_SYNC_TOKEN_IMGSYS_WPE_EIS: c_int = 813;
pub const CMDQ_SYNC_TOKEN_IMGSYS_OMC_TNR: c_int = 814;
pub const CMDQ_SYNC_TOKEN_IMGSYS_WPE_LITE: c_int = 815;
pub const CMDQ_SYNC_TOKEN_IMGSYS_TRAW: c_int = 816;
pub const CMDQ_SYNC_TOKEN_IMGSYS_LTRAW: c_int = 817;
pub const CMDQ_SYNC_TOKEN_IMGSYS_XTRAW: c_int = 818;
pub const CMDQ_SYNC_TOKEN_IMGSYS_DIP: c_int = 819;
pub const CMDQ_SYNC_TOKEN_IMGSYS_PQDIP_A: c_int = 820;
pub const CMDQ_SYNC_TOKEN_IMGSYS_PQDIP_B: c_int = 821;
pub const CMDQ_SYNC_TOKEN_IPESYS_ME: c_int = 822;
pub const CMDQ_SYNC_TOKEN_APUSYS_APU: c_int = 823;
pub const CMDQ_SYNC_TOKEN_IMGSYS_VSS_TRAW: c_int = 824;
pub const CMDQ_SYNC_TOKEN_IMGSYS_VSS_LTRAW: c_int = 825;
pub const CMDQ_SYNC_TOKEN_IMGSYS_VSS_XTRAW: c_int = 826;
pub const CMDQ_SYNC_TOKEN_IMGSYS_VSS_DIP: c_int = 827;
pub const CMDQ_SYNC_TOKEN_IMGSYS_OMC_LITE: c_int = 828;
// IMG software token for QoS
pub const CMDQ_SYNC_TOKEN_IMGSYS_QOS_LOCK: c_int = 829;
// IMG software token for Qof
pub const CMDQ_SYNC_TOKEN_DIP_POWER_CTRL: c_int = 830;
pub const CMDQ_SYNC_TOKEN_DIP_TRIG_PWR_ON: c_int = 831;
pub const CMDQ_SYNC_TOKEN_DIP_PWR_ON: c_int = 832;
pub const CMDQ_SYNC_TOKEN_DIP_TRIG_PWR_OFF: c_int = 833;
pub const CMDQ_SYNC_TOKEN_DIP_PWR_OFF: c_int = 834;
pub const CMDQ_SYNC_TOKEN_DIP_PWR_HAND_SHAKE: c_int = 835;
pub const CMDQ_SYNC_TOKEN_TRAW_POWER_CTRL: c_int = 836;
pub const CMDQ_SYNC_TOKEN_TRAW_TRIG_PWR_ON: c_int = 837;
pub const CMDQ_SYNC_TOKEN_TRAW_PWR_ON: c_int = 838;
pub const CMDQ_SYNC_TOKEN_TRAW_TRIG_PWR_OFF: c_int = 839;
pub const CMDQ_SYNC_TOKEN_TRAW_PWR_OFF: c_int = 840;
pub const CMDQ_SYNC_TOKEN_TRAW_PWR_HAND_SHAKE: c_int = 841;
// End of GCE1 software token
// Begin of common software token
//
// Notify normal CMDQ there are some secure task done
// MUST NOT CHANGE, this token sync with secure world
//
pub const CMDQ_SYNC_SECURE_THR_EOF: c_int = 940;
// CMDQ use software token
pub const CMDQ_SYNC_TOKEN_USER_0: c_int = 941;
pub const CMDQ_SYNC_TOKEN_USER_1: c_int = 942;
pub const CMDQ_SYNC_TOKEN_POLL_MONITOR: c_int = 943;
pub const CMDQ_SYNC_TOKEN_TPR_LOCK: c_int = 942;
// TZMP software token
pub const CMDQ_SYNC_TOKEN_TZMP_DISP_WAIT: c_int = 943;
pub const CMDQ_SYNC_TOKEN_TZMP_DISP_SET: c_int = 944;
pub const CMDQ_SYNC_TOKEN_TZMP_ISP_WAIT: c_int = 945;
pub const CMDQ_SYNC_TOKEN_TZMP_ISP_SET: c_int = 946;
pub const CMDQ_SYNC_TOKEN_TZMP_AIE_WAIT: c_int = 947;
pub const CMDQ_SYNC_TOKEN_TZMP_AIE_SET: c_int = 948;
pub const CMDQ_SYNC_TOKEN_TZMP_ADL_WAIT: c_int = 949;
pub const CMDQ_SYNC_TOKEN_TZMP_ADL_SET: c_int = 950;
// PREBUILT software token
pub const CMDQ_SYNC_TOKEN_PREBUILT_MDP_LOCK: c_int = 951;
pub const CMDQ_SYNC_TOKEN_PREBUILT_MML_LOCK: c_int = 952;
pub const CMDQ_SYNC_TOKEN_PREBUILT_VFMT_LOCK: c_int = 953;
pub const CMDQ_SYNC_TOKEN_PREBUILT_DISP_LOCK: c_int = 954;
pub const CMDQ_SYNC_TOKEN_DISP_VA_START: c_int = 955;
pub const CMDQ_SYNC_TOKEN_DISP_VA_END: c_int = 956;
//
// Event for GPR timer, used in sleep and poll with timeout
//
// CMDQ_TOKEN_GPR_TIMER_R0~15: 994 ~ 1009
//

// End of common software token
