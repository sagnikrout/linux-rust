//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/athub/athub_2_0_0_default.h
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
// Copyright (C) 2019  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _athub_2_0_0_DEFAULT_HEADER
// addressBlock: athub_atsdec
pub const mmATC_ATS_CNTL_DEFAULT: c_uint = 0x009a0c00;
pub const mmATC_ATS_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_FAULT_CNTL_DEFAULT: c_uint = 0x000001ff;
pub const mmATC_ATS_FAULT_STATUS_INFO_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_FAULT_STATUS_ADDR_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_DEFAULT_PAGE_LOW_DEFAULT: c_uint = 0x00000000;
pub const mmATC_TRANS_FAULT_RSPCNTRL_DEFAULT: c_uint = 0xffffffff;
pub const mmATC_ATS_FAULT_STATUS_INFO2_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_MISC_CNTL_DEFAULT: c_uint = 0x001c0200;
pub const mmATC_VMID_PASID_MAPPING_UPDATE_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID0_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID1_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID2_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID3_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID4_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID5_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID6_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID7_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID8_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID9_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID10_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID11_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID12_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID13_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID14_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID15_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_VMID_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_GFX_ATCL2_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER0_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER1_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER2_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER3_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER_RSLT_CNTL_DEFAULT: c_uint = 0x04000000;
pub const mmATC_PERFCOUNTER_LO_DEFAULT: c_uint = 0x00000000;
pub const mmATC_PERFCOUNTER_HI_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_PASID_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_PAGE_REQ_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_OUTSTAND_PAGE_REQ_ALLOC_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_COMMAND_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_0_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_1_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_2_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_3_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_4_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_5_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_6_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_7_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_8_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_9_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_10_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_11_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_12_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_13_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_14_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_15_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_16_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_17_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_18_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_19_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_20_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_21_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_22_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_23_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_24_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_25_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_26_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_27_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_28_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_29_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_PCIE_ATS_CNTL_VF_30_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_MEM_POWER_LS_DEFAULT: c_uint = 0x00000208;
pub const mmATS_IH_CREDIT_DEFAULT: c_uint = 0x00150002;
pub const mmATHUB_IH_CREDIT_DEFAULT: c_uint = 0x00020002;
pub const mmATC_VMID16_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID17_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID18_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID19_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID20_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID21_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID22_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID23_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID24_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID25_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID26_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID27_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID28_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID29_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID30_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_VMID31_PASID_MAPPING_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_MMHUB_ATCL2_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_SHARED_VIRT_RESET_REQ_DEFAULT: c_uint = 0x00000000;
pub const mmATHUB_SHARED_ACTIVE_FCN_ID_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_SDPPORT_CNTL_DEFAULT: c_uint = 0x03ffa210;
pub const mmATC_ATS_VMID_SNAPSHOT_GFX_STAT_DEFAULT: c_uint = 0x00000000;
pub const mmATC_ATS_VMID_SNAPSHOT_MMHUB_STAT_DEFAULT: c_uint = 0x00000000;
// addressBlock: athub_xpbdec
pub const mmXPB_RTR_SRC_APRTR0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR4_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR5_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR7_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR8_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_SRC_APRTR9_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_SRC_APRTR0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_SRC_APRTR1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_SRC_APRTR2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_SRC_APRTR3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP4_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP5_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP7_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP8_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_RTR_DEST_MAP9_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_DEST_MAP0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_DEST_MAP1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_DEST_MAP2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_RTR_DEST_MAP3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG4_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG5_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_CFG7_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_EXTRA_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_EXTRA_MSK_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_LB_ADDR_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_WCB_STS_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_HST_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR_CFG_DEFAULT: c_uint = 0x0000000f;
pub const mmXPB_P2P_BAR0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR4_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR5_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR7_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR_SETUP_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR_DELTA_ABOVE_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_P2P_BAR_DELTA_BELOW_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR4_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR5_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR7_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR8_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PEER_SYS_BAR9_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_PEER_SYS_BAR0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_PEER_SYS_BAR1_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_PEER_SYS_BAR2_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_XDMA_PEER_SYS_BAR3_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLK_GAT_DEFAULT: c_uint = 0x00040400;
pub const mmXPB_INTF_CFG_DEFAULT: c_uint = 0x000f1040;
pub const mmXPB_INTF_STS_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PIPE_STS_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_SUB_CTRL_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_MAP_INVERT_FLUSH_NUM_LSB_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_PERF_KNOBS_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_STICKY_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_STICKY_W1C_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_MISC_CFG_DEFAULT: c_uint = 0x4d585042;
pub const mmXPB_INTF_CFG2_DEFAULT: c_uint = 0x00000040;
pub const mmXPB_CLG_EXTRA_RD_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_EXTRA_MSK_RD_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GFX_MATCH_DEFAULT: c_uint = 0x03000000;
pub const mmXPB_CLG_GFX_MATCH_MSK_DEFAULT: c_uint = 0x003cf3cf;
pub const mmXPB_CLG_MM_MATCH_DEFAULT: c_uint = 0x00003000;
pub const mmXPB_CLG_MM_MATCH_MSK_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GUS_MATCH_DEFAULT: c_uint = 0x00000040;
pub const mmXPB_CLG_GUS_MATCH_MSK_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GFX_UNITID_MAPPING0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GFX_UNITID_MAPPING1_DEFAULT: c_uint = 0x00000040;
pub const mmXPB_CLG_GFX_UNITID_MAPPING2_DEFAULT: c_uint = 0x00000080;
pub const mmXPB_CLG_GFX_UNITID_MAPPING3_DEFAULT: c_uint = 0x000000c0;
pub const mmXPB_CLG_GFX_UNITID_MAPPING4_DEFAULT: c_uint = 0x00000100;
pub const mmXPB_CLG_GFX_UNITID_MAPPING5_DEFAULT: c_uint = 0x00000140;
pub const mmXPB_CLG_GFX_UNITID_MAPPING6_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GFX_UNITID_MAPPING7_DEFAULT: c_uint = 0x000001c0;
pub const mmXPB_CLG_MM_UNITID_MAPPING0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_MM_UNITID_MAPPING1_DEFAULT: c_uint = 0x00000040;
pub const mmXPB_CLG_MM_UNITID_MAPPING2_DEFAULT: c_uint = 0x00000080;
pub const mmXPB_CLG_MM_UNITID_MAPPING3_DEFAULT: c_uint = 0x000000c0;
pub const mmXPB_CLG_GUS_UNITID_MAPPING0_DEFAULT: c_uint = 0x00000000;
pub const mmXPB_CLG_GUS_UNITID_MAPPING1_DEFAULT: c_uint = 0x00000040;
pub const mmXPB_CLG_GUS_UNITID_MAPPING2_DEFAULT: c_uint = 0x00000080;
pub const mmXPB_CLG_GUS_UNITID_MAPPING3_DEFAULT: c_uint = 0x000000c0;
pub const mmXPB_CLG_GUS_UNITID_MAPPING4_DEFAULT: c_uint = 0x00000100;
pub const mmXPB_CLG_GUS_UNITID_MAPPING5_DEFAULT: c_uint = 0x00000140;
pub const mmXPB_CLG_GUS_UNITID_MAPPING6_DEFAULT: c_uint = 0x00000180;
pub const mmXPB_CLG_GUS_UNITID_MAPPING7_DEFAULT: c_uint = 0x000001c0;
// addressBlock: athub_rpbdec
pub const mmRPB_PASSPW_CONF_DEFAULT: c_uint = 0x00000230;
pub const mmRPB_BLOCKLEVEL_CONF_DEFAULT: c_uint = 0x000000f0;
pub const mmRPB_TAG_CONF_DEFAULT: c_uint = 0x08040080;
pub const mmRPB_EFF_CNTL_DEFAULT: c_uint = 0x00001010;
pub const mmRPB_ARB_CNTL_DEFAULT: c_uint = 0x00040404;
pub const mmRPB_ARB_CNTL2_DEFAULT: c_uint = 0x00040104;
pub const mmRPB_BIF_CNTL_DEFAULT: c_uint = 0x01000404;
pub const mmRPB_WR_SWITCH_CNTL_DEFAULT: c_uint = 0x02040810;
pub const mmRPB_WR_COMBINE_CNTL_DEFAULT: c_uint = 0x00000013;
pub const mmRPB_RD_SWITCH_CNTL_DEFAULT: c_uint = 0x02040810;
pub const mmRPB_CID_QUEUE_WR_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_CID_QUEUE_RD_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERF_COUNTER_CNTL_DEFAULT: c_uint = 0x00000010;
pub const mmRPB_PERF_COUNTER_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_CID_QUEUE_EX_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_CID_QUEUE_EX_DATA_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_SWITCH_CNTL2_DEFAULT: c_uint = 0x02040810;
pub const mmRPB_DEINTRLV_COMBINE_CNTL_DEFAULT: c_uint = 0x00000204;
pub const mmRPB_VC_SWITCH_RDWR_DEFAULT: c_uint = 0x00204040;
pub const mmRPB_PERFCOUNTER_LO_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER_HI_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER0_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER1_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER2_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER3_CFG_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_PERFCOUNTER_RSLT_CNTL_DEFAULT: c_uint = 0x04000000;
pub const mmRPB_BIF_CNTL2_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_RD_QUEUE_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_RD_QUEUE_CNTL2_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_WR_QUEUE_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_WR_QUEUE_CNTL2_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_EA_QUEUE_WR_DEFAULT: c_uint = 0x00000000;
pub const mmRPB_ATS_CNTL_DEFAULT: c_uint = 0x58088422;
pub const mmRPB_ATS_CNTL2_DEFAULT: c_uint = 0x00050b13;
pub const mmRPB_DF_SDPPORT_CNTL_DEFAULT: c_uint = 0x00003820;
pub const mmRPB_SDPPORT_CNTL_DEFAULT: c_uint = 0x0fd14010;
pub const mmRPB_NBIF_SDPPORT_CNTL_DEFAULT: c_uint = 0x08084020;
