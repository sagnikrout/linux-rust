//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/pcie_wrap_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PCIE_WRAP (Prototype: PCIE_WRAP)
//
pub const mmPCIE_WRAP_PHY_RST_N: c_uint = 0xC01300;
pub const mmPCIE_WRAP_OUTSTAND_TRANS: c_uint = 0xC01400;
pub const mmPCIE_WRAP_MASK_REQ: c_uint = 0xC01404;
pub const mmPCIE_WRAP_IND_AWADDR_L: c_uint = 0xC01500;
pub const mmPCIE_WRAP_IND_AWADDR_H: c_uint = 0xC01504;
pub const mmPCIE_WRAP_IND_AWLEN: c_uint = 0xC01508;
pub const mmPCIE_WRAP_IND_AWSIZE: c_uint = 0xC0150C;
pub const mmPCIE_WRAP_IND_AWBURST: c_uint = 0xC01510;
pub const mmPCIE_WRAP_IND_AWLOCK: c_uint = 0xC01514;
pub const mmPCIE_WRAP_IND_AWCACHE: c_uint = 0xC01518;
pub const mmPCIE_WRAP_IND_AWPROT: c_uint = 0xC0151C;
pub const mmPCIE_WRAP_IND_AWVALID: c_uint = 0xC01520;
pub const mmPCIE_WRAP_IND_WDATA_0: c_uint = 0xC01524;
pub const mmPCIE_WRAP_IND_WDATA_1: c_uint = 0xC01528;
pub const mmPCIE_WRAP_IND_WDATA_2: c_uint = 0xC0152C;
pub const mmPCIE_WRAP_IND_WDATA_3: c_uint = 0xC01530;
pub const mmPCIE_WRAP_IND_WSTRB: c_uint = 0xC01544;
pub const mmPCIE_WRAP_IND_WLAST: c_uint = 0xC01548;
pub const mmPCIE_WRAP_IND_WVALID: c_uint = 0xC0154C;
pub const mmPCIE_WRAP_IND_BRESP: c_uint = 0xC01550;
pub const mmPCIE_WRAP_IND_BVALID: c_uint = 0xC01554;
pub const mmPCIE_WRAP_IND_ARADDR_0: c_uint = 0xC01558;
pub const mmPCIE_WRAP_IND_ARADDR_1: c_uint = 0xC0155C;
pub const mmPCIE_WRAP_IND_ARLEN: c_uint = 0xC01560;
pub const mmPCIE_WRAP_IND_ARSIZE: c_uint = 0xC01564;
pub const mmPCIE_WRAP_IND_ARBURST: c_uint = 0xC01568;
pub const mmPCIE_WRAP_IND_ARLOCK: c_uint = 0xC0156C;
pub const mmPCIE_WRAP_IND_ARCACHE: c_uint = 0xC01570;
pub const mmPCIE_WRAP_IND_ARPROT: c_uint = 0xC01574;
pub const mmPCIE_WRAP_IND_ARVALID: c_uint = 0xC01578;
pub const mmPCIE_WRAP_IND_RDATA_0: c_uint = 0xC0157C;
pub const mmPCIE_WRAP_IND_RDATA_1: c_uint = 0xC01580;
pub const mmPCIE_WRAP_IND_RDATA_2: c_uint = 0xC01584;
pub const mmPCIE_WRAP_IND_RDATA_3: c_uint = 0xC01588;
pub const mmPCIE_WRAP_IND_RLAST: c_uint = 0xC0159C;
pub const mmPCIE_WRAP_IND_RRESP: c_uint = 0xC015A0;
pub const mmPCIE_WRAP_IND_RVALID: c_uint = 0xC015A4;
pub const mmPCIE_WRAP_IND_AWMISC_INFO: c_uint = 0xC015A8;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_HDR_34DW_0: c_uint = 0xC015AC;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_HDR_34DW_1: c_uint = 0xC015B0;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_P_TAG: c_uint = 0xC015B4;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_ATU_BYPAS: c_uint = 0xC015B8;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_FUNC_NUM: c_uint = 0xC015BC;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_VFUNC_ACT: c_uint = 0xC015C0;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_VFUNC_NUM: c_uint = 0xC015C4;
pub const mmPCIE_WRAP_IND_AWMISC_INFO_TLPPRFX: c_uint = 0xC015C8;
pub const mmPCIE_WRAP_IND_ARMISC_INFO: c_uint = 0xC015CC;
pub const mmPCIE_WRAP_IND_ARMISC_INFO_TLPPRFX: c_uint = 0xC015D0;
pub const mmPCIE_WRAP_IND_ARMISC_INFO_ATU_BYP: c_uint = 0xC015D4;
pub const mmPCIE_WRAP_IND_ARMISC_INFO_FUNC_NUM: c_uint = 0xC015D8;
pub const mmPCIE_WRAP_IND_ARMISC_INFO_VFUNC_ACT: c_uint = 0xC015DC;
pub const mmPCIE_WRAP_IND_ARMISC_INFO_VFUNC_NUM: c_uint = 0xC015E0;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO: c_uint = 0xC01800;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_HDR_34DW_0: c_uint = 0xC01804;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_HDR_34DW_1: c_uint = 0xC01808;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_P_TAG: c_uint = 0xC0180C;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_ATU_BYPAS: c_uint = 0xC01810;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_FUNC_NUM: c_uint = 0xC01814;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_VFUNC_ACT: c_uint = 0xC01818;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_VFUNC_NUM: c_uint = 0xC0181C;
pub const mmPCIE_WRAP_SLV_AWMISC_INFO_TLPPRFX: c_uint = 0xC01820;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO: c_uint = 0xC01824;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO_TLPPRFX: c_uint = 0xC01828;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO_ATU_BYP: c_uint = 0xC0182C;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO_FUNC_NUM: c_uint = 0xC01830;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO_VFUNC_ACT: c_uint = 0xC01834;
pub const mmPCIE_WRAP_SLV_ARMISC_INFO_VFUNC_NUM: c_uint = 0xC01838;
pub const mmPCIE_WRAP_MAX_QID: c_uint = 0xC01900;
pub const mmPCIE_WRAP_DB_BASE_ADDR_L_0: c_uint = 0xC01910;
pub const mmPCIE_WRAP_DB_BASE_ADDR_L_1: c_uint = 0xC01914;
pub const mmPCIE_WRAP_DB_BASE_ADDR_L_2: c_uint = 0xC01918;
pub const mmPCIE_WRAP_DB_BASE_ADDR_L_3: c_uint = 0xC0191C;
pub const mmPCIE_WRAP_DB_BASE_ADDR_H_0: c_uint = 0xC01920;
pub const mmPCIE_WRAP_DB_BASE_ADDR_H_1: c_uint = 0xC01924;
pub const mmPCIE_WRAP_DB_BASE_ADDR_H_2: c_uint = 0xC01928;
pub const mmPCIE_WRAP_DB_BASE_ADDR_H_3: c_uint = 0xC0192C;
pub const mmPCIE_WRAP_DB_MASK: c_uint = 0xC01940;
pub const mmPCIE_WRAP_SQ_BASE_ADDR_H: c_uint = 0xC01A00;
pub const mmPCIE_WRAP_SQ_BASE_ADDR_L: c_uint = 0xC01A04;
pub const mmPCIE_WRAP_SQ_STRIDE_ACCRESS: c_uint = 0xC01A08;
pub const mmPCIE_WRAP_SQ_POP_CMD: c_uint = 0xC01A10;
pub const mmPCIE_WRAP_SQ_POP_DATA: c_uint = 0xC01A14;
pub const mmPCIE_WRAP_DB_INTR_0: c_uint = 0xC01A20;
pub const mmPCIE_WRAP_DB_INTR_1: c_uint = 0xC01A24;
pub const mmPCIE_WRAP_DB_INTR_2: c_uint = 0xC01A28;
pub const mmPCIE_WRAP_DB_INTR_3: c_uint = 0xC01A2C;
pub const mmPCIE_WRAP_DB_INTR_4: c_uint = 0xC01A30;
pub const mmPCIE_WRAP_DB_INTR_5: c_uint = 0xC01A34;
pub const mmPCIE_WRAP_DB_INTR_6: c_uint = 0xC01A38;
pub const mmPCIE_WRAP_DB_INTR_7: c_uint = 0xC01A3C;
pub const mmPCIE_WRAP_MMU_BYPASS_DMA: c_uint = 0xC01A80;
pub const mmPCIE_WRAP_MMU_BYPASS_NON_DMA: c_uint = 0xC01A84;
pub const mmPCIE_WRAP_ASID_NON_DMA: c_uint = 0xC01A90;
pub const mmPCIE_WRAP_ASID_DMA_0: c_uint = 0xC01AA0;
pub const mmPCIE_WRAP_ASID_DMA_1: c_uint = 0xC01AA4;
pub const mmPCIE_WRAP_ASID_DMA_2: c_uint = 0xC01AA8;
pub const mmPCIE_WRAP_ASID_DMA_3: c_uint = 0xC01AAC;
pub const mmPCIE_WRAP_ASID_DMA_4: c_uint = 0xC01AB0;
pub const mmPCIE_WRAP_ASID_DMA_5: c_uint = 0xC01AB4;
pub const mmPCIE_WRAP_ASID_DMA_6: c_uint = 0xC01AB8;
pub const mmPCIE_WRAP_ASID_DMA_7: c_uint = 0xC01ABC;
pub const mmPCIE_WRAP_CPU_HOT_RST: c_uint = 0xC01AE0;
pub const mmPCIE_WRAP_AXI_PROT_OVR: c_uint = 0xC01AE4;
pub const mmPCIE_WRAP_CACHE_OVR: c_uint = 0xC01B00;
pub const mmPCIE_WRAP_LOCK_OVR: c_uint = 0xC01B04;
pub const mmPCIE_WRAP_PROT_OVR: c_uint = 0xC01B08;
pub const mmPCIE_WRAP_ARUSER_OVR: c_uint = 0xC01B0C;
pub const mmPCIE_WRAP_AWUSER_OVR: c_uint = 0xC01B10;
pub const mmPCIE_WRAP_ARUSER_OVR_EN: c_uint = 0xC01B14;
pub const mmPCIE_WRAP_AWUSER_OVR_EN: c_uint = 0xC01B18;
pub const mmPCIE_WRAP_MAX_OUTSTAND: c_uint = 0xC01B20;
pub const mmPCIE_WRAP_MST_IN: c_uint = 0xC01B24;
pub const mmPCIE_WRAP_RSP_OK: c_uint = 0xC01B28;
pub const mmPCIE_WRAP_LBW_CACHE_OVR: c_uint = 0xC01B40;
pub const mmPCIE_WRAP_LBW_LOCK_OVR: c_uint = 0xC01B44;
pub const mmPCIE_WRAP_LBW_PROT_OVR: c_uint = 0xC01B48;
pub const mmPCIE_WRAP_LBW_ARUSER_OVR: c_uint = 0xC01B4C;
pub const mmPCIE_WRAP_LBW_AWUSER_OVR: c_uint = 0xC01B50;
pub const mmPCIE_WRAP_LBW_ARUSER_OVR_EN: c_uint = 0xC01B58;
pub const mmPCIE_WRAP_LBW_AWUSER_OVR_EN: c_uint = 0xC01B5C;
pub const mmPCIE_WRAP_LBW_MAX_OUTSTAND: c_uint = 0xC01B60;
pub const mmPCIE_WRAP_LBW_MST_IN: c_uint = 0xC01B64;
pub const mmPCIE_WRAP_LBW_RSP_OK: c_uint = 0xC01B68;
pub const mmPCIE_WRAP_QUEUE_INIT: c_uint = 0xC01C00;
pub const mmPCIE_WRAP_AXI_SPLIT_INTR_0: c_uint = 0xC01C10;
pub const mmPCIE_WRAP_AXI_SPLIT_INTR_1: c_uint = 0xC01C14;
pub const mmPCIE_WRAP_DB_AWUSER: c_uint = 0xC01D00;
pub const mmPCIE_WRAP_DB_ARUSER: c_uint = 0xC01D04;
pub const mmPCIE_WRAP_PCIE_AWUSER: c_uint = 0xC01D08;
pub const mmPCIE_WRAP_PCIE_ARUSER: c_uint = 0xC01D0C;
pub const mmPCIE_WRAP_PSOC_AWUSER: c_uint = 0xC01D10;
pub const mmPCIE_WRAP_PSOC_ARUSER: c_uint = 0xC01D14;
pub const mmPCIE_WRAP_SCH_Q_AWUSER: c_uint = 0xC01D18;
pub const mmPCIE_WRAP_SCH_Q_ARUSER: c_uint = 0xC01D1C;
pub const mmPCIE_WRAP_PSOC2PCI_AWUSER: c_uint = 0xC01D40;
pub const mmPCIE_WRAP_PSOC2PCI_ARUSER: c_uint = 0xC01D44;
pub const mmPCIE_WRAP_DRAIN_TIMEOUT: c_uint = 0xC01D50;
pub const mmPCIE_WRAP_DRAIN_CFG: c_uint = 0xC01D54;
pub const mmPCIE_WRAP_DB_AXI_ERR: c_uint = 0xC01DE0;
pub const mmPCIE_WRAP_SPMU_INTR: c_uint = 0xC01DE4;
pub const mmPCIE_WRAP_AXI_INTR: c_uint = 0xC01DE8;
pub const mmPCIE_WRAP_E2E_CTRL: c_uint = 0xC01DF0;
