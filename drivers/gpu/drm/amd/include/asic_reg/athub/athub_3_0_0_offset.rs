//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/athub/athub_3_0_0_offset.h
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


//
// Copyright 2021 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _athub_3_0_0_OFFSET_HEADER
// addressBlock: athub_xpbdec
// base address: 0x3000
pub const regXPB_RTR_SRC_APRTR0: c_uint = 0x0000;
pub const regXPB_RTR_SRC_APRTR0_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR1: c_uint = 0x0001;
pub const regXPB_RTR_SRC_APRTR1_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR2: c_uint = 0x0002;
pub const regXPB_RTR_SRC_APRTR2_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR3: c_uint = 0x0003;
pub const regXPB_RTR_SRC_APRTR3_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR4: c_uint = 0x0004;
pub const regXPB_RTR_SRC_APRTR4_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR5: c_uint = 0x0005;
pub const regXPB_RTR_SRC_APRTR5_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR6: c_uint = 0x0006;
pub const regXPB_RTR_SRC_APRTR6_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR7: c_uint = 0x0007;
pub const regXPB_RTR_SRC_APRTR7_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR8: c_uint = 0x0008;
pub const regXPB_RTR_SRC_APRTR8_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR9: c_uint = 0x0009;
pub const regXPB_RTR_SRC_APRTR9_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR10: c_uint = 0x000a;
pub const regXPB_RTR_SRC_APRTR10_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR11: c_uint = 0x000b;
pub const regXPB_RTR_SRC_APRTR11_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR12: c_uint = 0x000c;
pub const regXPB_RTR_SRC_APRTR12_BASE_IDX: c_int = 0;
pub const regXPB_RTR_SRC_APRTR13: c_uint = 0x000d;
pub const regXPB_RTR_SRC_APRTR13_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP0: c_uint = 0x000e;
pub const regXPB_RTR_DEST_MAP0_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP1: c_uint = 0x000f;
pub const regXPB_RTR_DEST_MAP1_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP2: c_uint = 0x0010;
pub const regXPB_RTR_DEST_MAP2_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP3: c_uint = 0x0011;
pub const regXPB_RTR_DEST_MAP3_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP4: c_uint = 0x0012;
pub const regXPB_RTR_DEST_MAP4_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP5: c_uint = 0x0013;
pub const regXPB_RTR_DEST_MAP5_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP6: c_uint = 0x0014;
pub const regXPB_RTR_DEST_MAP6_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP7: c_uint = 0x0015;
pub const regXPB_RTR_DEST_MAP7_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP8: c_uint = 0x0016;
pub const regXPB_RTR_DEST_MAP8_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP9: c_uint = 0x0017;
pub const regXPB_RTR_DEST_MAP9_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP10: c_uint = 0x0018;
pub const regXPB_RTR_DEST_MAP10_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP11: c_uint = 0x0019;
pub const regXPB_RTR_DEST_MAP11_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP12: c_uint = 0x001a;
pub const regXPB_RTR_DEST_MAP12_BASE_IDX: c_int = 0;
pub const regXPB_RTR_DEST_MAP13: c_uint = 0x001b;
pub const regXPB_RTR_DEST_MAP13_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG0: c_uint = 0x001c;
pub const regXPB_CLG_CFG0_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG1: c_uint = 0x001d;
pub const regXPB_CLG_CFG1_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG2: c_uint = 0x001e;
pub const regXPB_CLG_CFG2_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG3: c_uint = 0x001f;
pub const regXPB_CLG_CFG3_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG4: c_uint = 0x0020;
pub const regXPB_CLG_CFG4_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG5: c_uint = 0x0021;
pub const regXPB_CLG_CFG5_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG6: c_uint = 0x0022;
pub const regXPB_CLG_CFG6_BASE_IDX: c_int = 0;
pub const regXPB_CLG_CFG7: c_uint = 0x0023;
pub const regXPB_CLG_CFG7_BASE_IDX: c_int = 0;
pub const regXPB_CLG_EXTRA: c_uint = 0x0024;
pub const regXPB_CLG_EXTRA_BASE_IDX: c_int = 0;
pub const regXPB_CLG_EXTRA_MSK: c_uint = 0x0025;
pub const regXPB_CLG_EXTRA_MSK_BASE_IDX: c_int = 0;
pub const regXPB_LB_ADDR: c_uint = 0x0026;
pub const regXPB_LB_ADDR_BASE_IDX: c_int = 0;
pub const regXPB_WCB_STS: c_uint = 0x0027;
pub const regXPB_WCB_STS_BASE_IDX: c_int = 0;
pub const regXPB_HST_CFG: c_uint = 0x0028;
pub const regXPB_HST_CFG_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR_CFG: c_uint = 0x0029;
pub const regXPB_P2P_BAR_CFG_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR0: c_uint = 0x002a;
pub const regXPB_P2P_BAR0_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR1: c_uint = 0x002b;
pub const regXPB_P2P_BAR1_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR2: c_uint = 0x002c;
pub const regXPB_P2P_BAR2_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR3: c_uint = 0x002d;
pub const regXPB_P2P_BAR3_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR4: c_uint = 0x002e;
pub const regXPB_P2P_BAR4_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR5: c_uint = 0x002f;
pub const regXPB_P2P_BAR5_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR6: c_uint = 0x0030;
pub const regXPB_P2P_BAR6_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR7: c_uint = 0x0031;
pub const regXPB_P2P_BAR7_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR_SETUP: c_uint = 0x0032;
pub const regXPB_P2P_BAR_SETUP_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR_DELTA_ABOVE: c_uint = 0x0034;
pub const regXPB_P2P_BAR_DELTA_ABOVE_BASE_IDX: c_int = 0;
pub const regXPB_P2P_BAR_DELTA_BELOW: c_uint = 0x0035;
pub const regXPB_P2P_BAR_DELTA_BELOW_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR0: c_uint = 0x0036;
pub const regXPB_PEER_SYS_BAR0_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR1: c_uint = 0x0037;
pub const regXPB_PEER_SYS_BAR1_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR2: c_uint = 0x0038;
pub const regXPB_PEER_SYS_BAR2_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR3: c_uint = 0x0039;
pub const regXPB_PEER_SYS_BAR3_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR4: c_uint = 0x003a;
pub const regXPB_PEER_SYS_BAR4_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR5: c_uint = 0x003b;
pub const regXPB_PEER_SYS_BAR5_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR6: c_uint = 0x003c;
pub const regXPB_PEER_SYS_BAR6_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR7: c_uint = 0x003d;
pub const regXPB_PEER_SYS_BAR7_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR8: c_uint = 0x003e;
pub const regXPB_PEER_SYS_BAR8_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR9: c_uint = 0x003f;
pub const regXPB_PEER_SYS_BAR9_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR10: c_uint = 0x0040;
pub const regXPB_PEER_SYS_BAR10_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR11: c_uint = 0x0041;
pub const regXPB_PEER_SYS_BAR11_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR12: c_uint = 0x0042;
pub const regXPB_PEER_SYS_BAR12_BASE_IDX: c_int = 0;
pub const regXPB_PEER_SYS_BAR13: c_uint = 0x0043;
pub const regXPB_PEER_SYS_BAR13_BASE_IDX: c_int = 0;
pub const regXPB_CLK_GAT: c_uint = 0x0044;
pub const regXPB_CLK_GAT_BASE_IDX: c_int = 0;
pub const regXPB_INTF_CFG: c_uint = 0x0045;
pub const regXPB_INTF_CFG_BASE_IDX: c_int = 0;
pub const regXPB_INTF_STS: c_uint = 0x0046;
pub const regXPB_INTF_STS_BASE_IDX: c_int = 0;
pub const regXPB_PIPE_STS: c_uint = 0x0047;
pub const regXPB_PIPE_STS_BASE_IDX: c_int = 0;
pub const regXPB_SUB_CTRL: c_uint = 0x0048;
pub const regXPB_SUB_CTRL_BASE_IDX: c_int = 0;
pub const regXPB_MAP_INVERT_FLUSH_NUM_LSB: c_uint = 0x0049;
pub const regXPB_MAP_INVERT_FLUSH_NUM_LSB_BASE_IDX: c_int = 0;
pub const regXPB_PERF_KNOBS: c_uint = 0x004a;
pub const regXPB_PERF_KNOBS_BASE_IDX: c_int = 0;
pub const regXPB_STICKY: c_uint = 0x004b;
pub const regXPB_STICKY_BASE_IDX: c_int = 0;
pub const regXPB_STICKY_W1C: c_uint = 0x004c;
pub const regXPB_STICKY_W1C_BASE_IDX: c_int = 0;
pub const regXPB_MISC_CFG: c_uint = 0x004d;
pub const regXPB_MISC_CFG_BASE_IDX: c_int = 0;
pub const regXPB_INTF_CFG2: c_uint = 0x004e;
pub const regXPB_INTF_CFG2_BASE_IDX: c_int = 0;
pub const regXPB_CLG_EXTRA_RD: c_uint = 0x004f;
pub const regXPB_CLG_EXTRA_RD_BASE_IDX: c_int = 0;
pub const regXPB_CLG_EXTRA_MSK_RD: c_uint = 0x0050;
pub const regXPB_CLG_EXTRA_MSK_RD_BASE_IDX: c_int = 0;
pub const regXPB_CLG_GFX_MATCH: c_uint = 0x0051;
pub const regXPB_CLG_GFX_MATCH_BASE_IDX: c_int = 0;
pub const regXPB_CLG_GFX_MATCH_MSK: c_uint = 0x0052;
pub const regXPB_CLG_GFX_MATCH_MSK_BASE_IDX: c_int = 0;
pub const regXPB_CLG_MM_MATCH: c_uint = 0x0053;
pub const regXPB_CLG_MM_MATCH_BASE_IDX: c_int = 0;
pub const regXPB_CLG_MM_MATCH_MSK: c_uint = 0x0054;
pub const regXPB_CLG_MM_MATCH_MSK_BASE_IDX: c_int = 0;
pub const regXPB_CLG_GUS_MATCH: c_uint = 0x0055;
pub const regXPB_CLG_GUS_MATCH_BASE_IDX: c_int = 0;
pub const regXPB_CLG_GUS_MATCH_MSK: c_uint = 0x0056;
pub const regXPB_CLG_GUS_MATCH_MSK_BASE_IDX: c_int = 0;
// addressBlock: athub_rpbdec
// base address: 0x31b0
pub const regRPB_PASSPW_CONF: c_uint = 0x006c;
pub const regRPB_PASSPW_CONF_BASE_IDX: c_int = 0;
pub const regRPB_BLOCKLEVEL_CONF: c_uint = 0x006d;
pub const regRPB_BLOCKLEVEL_CONF_BASE_IDX: c_int = 0;
pub const regRPB_TAG_CONF: c_uint = 0x006e;
pub const regRPB_TAG_CONF_BASE_IDX: c_int = 0;
pub const regRPB_ARB_CNTL: c_uint = 0x0071;
pub const regRPB_ARB_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_ARB_CNTL2: c_uint = 0x0072;
pub const regRPB_ARB_CNTL2_BASE_IDX: c_int = 0;
pub const regRPB_BIF_CNTL: c_uint = 0x0073;
pub const regRPB_BIF_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_BIF_CNTL2: c_uint = 0x0074;
pub const regRPB_BIF_CNTL2_BASE_IDX: c_int = 0;
pub const regATHUB_MISC_CNTL: c_uint = 0x0075;
pub const regATHUB_MISC_CNTL_BASE_IDX: c_int = 0;
pub const regATHUB_MEM_POWER_LS: c_uint = 0x0078;
pub const regATHUB_MEM_POWER_LS_BASE_IDX: c_int = 0;
pub const regRPB_SDPPORT_CNTL: c_uint = 0x007a;
pub const regRPB_SDPPORT_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_NBIF_SDPPORT_CNTL: c_uint = 0x007b;
pub const regRPB_NBIF_SDPPORT_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_DEINTRLV_COMBINE_CNTL: c_uint = 0x007c;
pub const regRPB_DEINTRLV_COMBINE_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_VC_SWITCH_RDWR: c_uint = 0x007d;
pub const regRPB_VC_SWITCH_RDWR_BASE_IDX: c_int = 0;
pub const regRPB_PERF_COUNTER_CNTL: c_uint = 0x007e;
pub const regRPB_PERF_COUNTER_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_PERF_COUNTER_STATUS: c_uint = 0x007f;
pub const regRPB_PERF_COUNTER_STATUS_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER_LO: c_uint = 0x0080;
pub const regRPB_PERFCOUNTER_LO_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER_HI: c_uint = 0x0081;
pub const regRPB_PERFCOUNTER_HI_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER0_CFG: c_uint = 0x0082;
pub const regRPB_PERFCOUNTER0_CFG_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER1_CFG: c_uint = 0x0083;
pub const regRPB_PERFCOUNTER1_CFG_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER2_CFG: c_uint = 0x0084;
pub const regRPB_PERFCOUNTER2_CFG_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER3_CFG: c_uint = 0x0085;
pub const regRPB_PERFCOUNTER3_CFG_BASE_IDX: c_int = 0;
pub const regRPB_PERFCOUNTER_RSLT_CNTL: c_uint = 0x0086;
pub const regRPB_PERFCOUNTER_RSLT_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_ATS_CNTL3: c_uint = 0x0087;
pub const regRPB_ATS_CNTL3_BASE_IDX: c_int = 0;
pub const regRPB_DF_SDPPORT_CNTL: c_uint = 0x0088;
pub const regRPB_DF_SDPPORT_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_ATS_CNTL: c_uint = 0x0089;
pub const regRPB_ATS_CNTL_BASE_IDX: c_int = 0;
pub const regRPB_ATS_CNTL2: c_uint = 0x008a;
pub const regRPB_ATS_CNTL2_BASE_IDX: c_int = 0;
