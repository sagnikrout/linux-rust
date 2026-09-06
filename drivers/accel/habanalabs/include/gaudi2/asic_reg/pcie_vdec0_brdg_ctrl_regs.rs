//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_vdec0_brdg_ctrl_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PCIE_VDEC0_BRDG_CTRL
// (Prototype: VDEC_BRDG_CTRL)
//
pub const mmPCIE_VDEC0_BRDG_CTRL_CGM_DISABLE: c_uint = 0x4F03100;
pub const mmPCIE_VDEC0_BRDG_CTRL_IDLE_MASK: c_uint = 0x4F03104;
pub const mmPCIE_VDEC0_BRDG_CTRL_APB_CGM_CNT: c_uint = 0x4F03108;
pub const mmPCIE_VDEC0_BRDG_CTRL_APB_ARB_WDOG_CNT: c_uint = 0x4F0310C;
pub const mmPCIE_VDEC0_BRDG_CTRL_GRACEFUL: c_uint = 0x4F03110;
pub const mmPCIE_VDEC0_BRDG_CTRL_IDLE_CGM_CNT: c_uint = 0x4F03114;
pub const mmPCIE_VDEC0_BRDG_CTRL_CAUSE_INTR: c_uint = 0x4F03120;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_AXI_VIOL_CAUSE: c_uint = 0x4F03124;
pub const mmPCIE_VDEC0_BRDG_CTRL_LBW_AXI_VIOL_CAUSE: c_uint = 0x4F03128;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXI_VIOL_CLR_STICKY_TERM: c_uint = 0x4F0312C;
pub const mmPCIE_VDEC0_BRDG_CTRL_CAUSE_INTR_MASK: c_uint = 0x4F03130;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_AXI_VIOL_MASK: c_uint = 0x4F03134;
pub const mmPCIE_VDEC0_BRDG_CTRL_LBW_AXI_VIOL_MASK: c_uint = 0x4F03138;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_GIC_INTR_MASK: c_uint = 0x4F03160;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_GIC_INTR_MASK: c_uint = 0x4F03170;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_GIC_INTR_MASK: c_uint = 0x4F03180;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_GIC_INTR_MASK: c_uint = 0x4F03190;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_HBW_AWPROT: c_uint = 0x4F031A0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_HBW_ARPROT: c_uint = 0x4F031A4;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_LBW_AWPROT: c_uint = 0x4F031B0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_LBW_ARPROT: c_uint = 0x4F031B4;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_LBW_SLV_AWPROT: c_uint = 0x4F031C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_LBW_SLV_ARPROT: c_uint = 0x4F031C4;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_AXI_LEGAL_AXSIZE: c_uint = 0x4F031D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_ARC_MSG_MASK: c_uint = 0x4F03200;
pub const mmPCIE_VDEC0_BRDG_CTRL_ARC_START_LBW_WDATA: c_uint = 0x4F03230;
pub const mmPCIE_VDEC0_BRDG_CTRL_ARC_FINISH_LBW_WDATA: c_uint = 0x4F03260;
pub const mmPCIE_VDEC0_BRDG_CTRL_HWEVENT_TRACE_SEL: c_uint = 0x4F03270;
pub const mmPCIE_VDEC0_BRDG_CTRL_HWEVENT_TRACE_ADDR: c_uint = 0x4F03280;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_FREE_RUN_CNT_L: c_uint = 0x4F03290;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_FREE_RUN_CNT_H: c_uint = 0x4F03294;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_FREE_RUN_SET_VALUE_L: c_uint = 0x4F032A0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_FREE_RUN_SET_VALUE_H: c_uint = 0x4F032A4;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_BUSY_CNT_L: c_uint = 0x4F032B0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_BUSY_CNT_H: c_uint = 0x4F032B4;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_BUSY_SET_VALUE_L: c_uint = 0x4F032C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_BUSY_SET_VALUE_H: c_uint = 0x4F032C4;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_CNTR_EN: c_uint = 0x4F032D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_INTR_MASK: c_uint = 0x4F03300;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_MSIX_FLOW_MASK: c_uint = 0x4F03310;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_WAIT_CNTR: c_uint = 0x4F03320;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_MSIX_WAIT_CNTR: c_uint = 0x4F03330;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_VCD_WAIT_CNTR: c_uint = 0x4F03334;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_VCD_MSIX_WAIT_CNTR: c_uint = 0x4F03338;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_SWREG1_ADDR: c_uint = 0x4F03340;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_APB_WR_ADDR: c_uint = 0x4F03350;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_APB_WR_DATA: c_uint = 0x4F03360;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_CPLQ_HBW_AWPROT: c_uint = 0x4F03380;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_CPLQ_HBW_AWADDR_L: c_uint = 0x4F03390;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_CPLQ_HBW_AWADDR_H: c_uint = 0x4F03394;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_MSIX_LBW_AWPROT: c_uint = 0x4F033C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_MSIX_LBW_AWADDR: c_uint = 0x4F033D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_VCD_MSIX_LBW_WDATA: c_uint = 0x4F033E0;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_INTR_MASK: c_uint = 0x4F03400;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_MSIX_FLOW_MASK: c_uint = 0x4F03410;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_WAIT_CNTR: c_uint = 0x4F03420;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_MSIX_WAIT_CNTR: c_uint = 0x4F03430;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_L2C_WAIT_CNTR: c_uint = 0x4F03434;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_L2C_MSIX_WAIT_CNTR: c_uint = 0x4F03438;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_SWREG1_ADDR: c_uint = 0x4F03440;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_APB_WR_ADDR: c_uint = 0x4F03450;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_APB_WR_DATA: c_uint = 0x4F03460;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_CPLQ_HBW_AWPROT: c_uint = 0x4F03480;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_CPLQ_HBW_AWADDR_L: c_uint = 0x4F03490;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_CPLQ_HBW_AWADDR_H: c_uint = 0x4F03494;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_MSIX_LBW_AWPROT: c_uint = 0x4F034C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_MSIX_LBW_AWADDR: c_uint = 0x4F034D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_L2C_MSIX_LBW_WDATA: c_uint = 0x4F034E0;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_INTR_MASK: c_uint = 0x4F03500;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_MSIX_FLOW_MASK: c_uint = 0x4F03510;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_WAIT_CNTR: c_uint = 0x4F03520;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_MSIX_WAIT_CNTR: c_uint = 0x4F03530;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_NRM_WAIT_CNTR: c_uint = 0x4F03534;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_NRM_MSIX_WAIT_CNTR: c_uint = 0x4F03538;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_SWREG1_ADDR: c_uint = 0x4F03540;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_APB_WR_ADDR: c_uint = 0x4F03550;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_APB_WR_DATA: c_uint = 0x4F03560;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_CPLQ_HBW_AWPROT: c_uint = 0x4F03580;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_CPLQ_HBW_AWADDR_L: c_uint = 0x4F03590;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_CPLQ_HBW_AWADDR_H: c_uint = 0x4F03594;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_MSIX_LBW_AWPROT: c_uint = 0x4F035C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_MSIX_LBW_AWADDR: c_uint = 0x4F035D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_NRM_MSIX_LBW_WDATA: c_uint = 0x4F035E0;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_INTR_MASK: c_uint = 0x4F03600;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_MSIX_FLOW_MASK: c_uint = 0x4F03610;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_WAIT_CNTR: c_uint = 0x4F03620;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_MSIX_WAIT_CNTR: c_uint = 0x4F03630;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_ABNRM_WAIT_CNTR: c_uint = 0x4F03634;
pub const mmPCIE_VDEC0_BRDG_CTRL_STAT_ABNRM_MSIX_WAIT_CNTR: c_uint = 0x4F03638;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_SWREG1_ADDR: c_uint = 0x4F03640;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_APB_WR_ADDR: c_uint = 0x4F03650;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_APB_WR_DATA: c_uint = 0x4F03660;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_CPLQ_HBW_AWPROT: c_uint = 0x4F03680;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_CPLQ_HBW_AWADDR_L: c_uint = 0x4F03690;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_CPLQ_HBW_AWADDR_H: c_uint = 0x4F03694;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_MSIX_LBW_AWPROT: c_uint = 0x4F036C0;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_MSIX_LBW_AWADDR: c_uint = 0x4F036D0;
pub const mmPCIE_VDEC0_BRDG_CTRL_ABNRM_MSIX_LBW_WDATA: c_uint = 0x4F036E0;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXI_SPLIT_BRESP_ERR_ID: c_uint = 0x4F03700;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXI_SPLIT_CFG: c_uint = 0x4F03704;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXI_SPLIT_NO_WR_INFLIGHT: c_uint = 0x4F03708;
pub const mmPCIE_VDEC0_BRDG_CTRL_HWEVENT_MASK: c_uint = 0x4F0370C;
pub const mmPCIE_VDEC0_BRDG_CTRL_HWEVENT_CNTXT: c_uint = 0x4F03714;
pub const mmPCIE_VDEC0_BRDG_CTRL_LBW_SLV_TERM_ERR_RESP: c_uint = 0x4F03718;
pub const mmPCIE_VDEC0_BRDG_CTRL_LBW_MSTR_TERM_ERR_RESP: c_uint = 0x4F0371C;
pub const mmPCIE_VDEC0_BRDG_CTRL_DEC_HBW_MSTR_ERR_RESP: c_uint = 0x4F03720;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_VIOL_TERM_STATUS: c_uint = 0x4F03724;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_LAST_AWADDR_TERM_L: c_uint = 0x4F03728;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_LAST_AWADDR_TERM_H: c_uint = 0x4F0372C;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_LAST_ARADDR_TERM_L: c_uint = 0x4F03730;
pub const mmPCIE_VDEC0_BRDG_CTRL_HBW_LAST_ARADDR_TERM_H: c_uint = 0x4F03734;
