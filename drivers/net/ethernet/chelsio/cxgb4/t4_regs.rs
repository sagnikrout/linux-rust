//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_regs.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2014 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const MYPF_BASE: c_uint = 0x1b000;

pub const PF0_BASE: c_uint = 0x1e000;

pub const PF_STRIDE: c_uint = 0x400;

pub const NUM_CIM_CTL_TSCH_CHANNEL_INSTANCES: c_int = 4;
pub const NUM_CIM_CTL_TSCH_CHANNEL_TSCH_CLASS_INSTANCES: c_int = 16;
pub const MYPORT_BASE: c_uint = 0x1c000;

pub const PORT0_BASE: c_uint = 0x20000;

pub const PORT_STRIDE: c_uint = 0x2000;

pub const NUM_LE_DB_DBGI_REQ_DATA_INSTANCES: c_int = 17;
pub const NUM_LE_DB_DBGI_RSP_DATA_INSTANCES: c_int = 17;
pub const SGE_PF_KDOORBELL_A: c_uint = 0x0;
pub const QID_S: c_int = 15;

pub const DBPRIO_S: c_int = 14;

pub const PIDX_S: c_int = 0;

pub const SGE_VF_KDOORBELL_A: c_uint = 0x0;
pub const DBTYPE_S: c_int = 13;

pub const PIDX_T5_S: c_int = 0;
pub const PIDX_T5_M: c_uint = 0x1fffU;

pub const SGE_PF_GTS_A: c_uint = 0x4;
pub const INGRESSQID_S: c_int = 16;

pub const TIMERREG_S: c_int = 13;

pub const SEINTARM_S: c_int = 12;

pub const CIDXINC_S: c_int = 0;
pub const CIDXINC_M: c_uint = 0xfffU;

pub const SGE_CONTROL_A: c_uint = 0x1008;
pub const SGE_CONTROL2_A: c_uint = 0x1124;
pub const RXPKTCPLMODE_S: c_int = 18;

pub const EGRSTATUSPAGESIZE_S: c_int = 17;

pub const PKTSHIFT_S: c_int = 10;
pub const PKTSHIFT_M: c_uint = 0x7U;

pub const INGPCIEBOUNDARY_S: c_int = 7;

pub const INGPADBOUNDARY_S: c_int = 4;
pub const INGPADBOUNDARY_M: c_uint = 0x7U;

pub const EGRPCIEBOUNDARY_S: c_int = 1;

pub const INGPACKBOUNDARY_S: c_int = 16;
pub const INGPACKBOUNDARY_M: c_uint = 0x7U;

pub const VFIFO_ENABLE_S: c_int = 10;

pub const SGE_DBVFIFO_BADDR_A: c_uint = 0x1138;
pub const DBVFIFO_SIZE_S: c_int = 6;
pub const DBVFIFO_SIZE_M: c_uint = 0xfffU;

pub const T6_DBVFIFO_SIZE_S: c_int = 0;
pub const T6_DBVFIFO_SIZE_M: c_uint = 0x1fffU;

pub const SGE_CTXT_CMD_A: c_uint = 0x11fc;
pub const BUSY_S: c_int = 31;

pub const CTXTTYPE_S: c_int = 24;
pub const CTXTTYPE_M: c_uint = 0x3U;

pub const CTXTQID_S: c_int = 0;
pub const CTXTQID_M: c_uint = 0x1ffffU;

pub const SGE_CTXT_DATA0_A: c_uint = 0x1200;
pub const SGE_CTXT_DATA5_A: c_uint = 0x1214;
pub const GLOBALENABLE_S: c_int = 0;

pub const SGE_HOST_PAGE_SIZE_A: c_uint = 0x100c;
pub const HOSTPAGESIZEPF7_S: c_int = 28;
pub const HOSTPAGESIZEPF7_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF6_S: c_int = 24;
pub const HOSTPAGESIZEPF6_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF5_S: c_int = 20;
pub const HOSTPAGESIZEPF5_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF4_S: c_int = 16;
pub const HOSTPAGESIZEPF4_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF3_S: c_int = 12;
pub const HOSTPAGESIZEPF3_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF2_S: c_int = 8;
pub const HOSTPAGESIZEPF2_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF1_S: c_int = 4;
pub const HOSTPAGESIZEPF1_M: c_uint = 0xfU;

pub const HOSTPAGESIZEPF0_S: c_int = 0;
pub const HOSTPAGESIZEPF0_M: c_uint = 0xfU;

pub const SGE_EGRESS_QUEUES_PER_PAGE_PF_A: c_uint = 0x1010;
pub const SGE_EGRESS_QUEUES_PER_PAGE_VF_A: c_uint = 0x1014;
pub const QUEUESPERPAGEPF1_S: c_int = 4;
pub const QUEUESPERPAGEPF0_S: c_int = 0;
pub const QUEUESPERPAGEPF0_M: c_uint = 0xfU;

pub const SGE_INT_CAUSE1_A: c_uint = 0x1024;
pub const SGE_INT_CAUSE2_A: c_uint = 0x1030;
pub const SGE_INT_CAUSE3_A: c_uint = 0x103c;
pub const ERR_FLM_DBP_S: c_int = 31;

pub const ERR_FLM_IDMA1_S: c_int = 30;

pub const ERR_FLM_IDMA0_S: c_int = 29;

pub const ERR_FLM_HINT_S: c_int = 28;

pub const ERR_PCIE_ERROR3_S: c_int = 27;

pub const ERR_PCIE_ERROR2_S: c_int = 26;

pub const ERR_PCIE_ERROR1_S: c_int = 25;

pub const ERR_PCIE_ERROR0_S: c_int = 24;

pub const ERR_CPL_EXCEED_IQE_SIZE_S: c_int = 22;

pub const ERR_INVALID_CIDX_INC_S: c_int = 21;

pub const ERR_CPL_OPCODE_0_S: c_int = 19;

pub const ERR_DROPPED_DB_S: c_int = 18;

pub const ERR_DATA_CPL_ON_HIGH_QID1_S: c_int = 17;

pub const ERR_DATA_CPL_ON_HIGH_QID0_S: c_int = 16;

pub const ERR_BAD_DB_PIDX3_S: c_int = 15;

pub const ERR_BAD_DB_PIDX2_S: c_int = 14;

pub const ERR_BAD_DB_PIDX1_S: c_int = 13;

pub const ERR_BAD_DB_PIDX0_S: c_int = 12;

pub const ERR_ING_CTXT_PRIO_S: c_int = 10;

pub const ERR_EGR_CTXT_PRIO_S: c_int = 9;

pub const DBFIFO_HP_INT_S: c_int = 8;

pub const DBFIFO_LP_INT_S: c_int = 7;

pub const INGRESS_SIZE_ERR_S: c_int = 5;

pub const EGRESS_SIZE_ERR_S: c_int = 4;

pub const SGE_INT_ENABLE3_A: c_uint = 0x1040;
pub const SGE_FL_BUFFER_SIZE0_A: c_uint = 0x1044;
pub const SGE_FL_BUFFER_SIZE1_A: c_uint = 0x1048;
pub const SGE_FL_BUFFER_SIZE2_A: c_uint = 0x104c;
pub const SGE_FL_BUFFER_SIZE3_A: c_uint = 0x1050;
pub const SGE_FL_BUFFER_SIZE4_A: c_uint = 0x1054;
pub const SGE_FL_BUFFER_SIZE5_A: c_uint = 0x1058;
pub const SGE_FL_BUFFER_SIZE6_A: c_uint = 0x105c;
pub const SGE_FL_BUFFER_SIZE7_A: c_uint = 0x1060;
pub const SGE_FL_BUFFER_SIZE8_A: c_uint = 0x1064;
pub const SGE_IMSG_CTXT_BADDR_A: c_uint = 0x1088;
pub const SGE_FLM_CACHE_BADDR_A: c_uint = 0x108c;
pub const SGE_FLM_CFG_A: c_uint = 0x1090;
pub const NOHDR_S: c_int = 18;

pub const HDRSTARTFLQ_S: c_int = 11;
pub const HDRSTARTFLQ_M: c_uint = 0x7U;

pub const SGE_INGRESS_RX_THRESHOLD_A: c_uint = 0x10a0;
pub const THRESHOLD_0_S: c_int = 24;
pub const THRESHOLD_0_M: c_uint = 0x3fU;

pub const THRESHOLD_1_S: c_int = 16;
pub const THRESHOLD_1_M: c_uint = 0x3fU;

pub const THRESHOLD_2_S: c_int = 8;
pub const THRESHOLD_2_M: c_uint = 0x3fU;

pub const THRESHOLD_3_S: c_int = 0;
pub const THRESHOLD_3_M: c_uint = 0x3fU;

pub const SGE_CONM_CTRL_A: c_uint = 0x1094;
pub const EGRTHRESHOLD_S: c_int = 8;
pub const EGRTHRESHOLD_M: c_uint = 0x3fU;

pub const EGRTHRESHOLDPACKING_S: c_int = 14;
pub const EGRTHRESHOLDPACKING_M: c_uint = 0x3fU;

pub const T6_EGRTHRESHOLDPACKING_S: c_int = 16;
pub const T6_EGRTHRESHOLDPACKING_M: c_uint = 0xffU;

pub const SGE_TIMESTAMP_LO_A: c_uint = 0x1098;
pub const SGE_TIMESTAMP_HI_A: c_uint = 0x109c;
pub const TSOP_S: c_int = 28;
pub const TSOP_M: c_uint = 0x3U;

pub const TSVAL_S: c_int = 0;
pub const TSVAL_M: c_uint = 0xfffffffU;

pub const SGE_DBFIFO_STATUS_A: c_uint = 0x10a4;
pub const SGE_DBVFIFO_SIZE_A: c_uint = 0x113c;
pub const HP_INT_THRESH_S: c_int = 28;
pub const HP_INT_THRESH_M: c_uint = 0xfU;

pub const LP_INT_THRESH_S: c_int = 12;
pub const LP_INT_THRESH_M: c_uint = 0xfU;

pub const SGE_DOORBELL_CONTROL_A: c_uint = 0x10a8;
pub const NOCOALESCE_S: c_int = 26;

pub const ENABLE_DROP_S: c_int = 13;

pub const SGE_TIMER_VALUE_0_AND_1_A: c_uint = 0x10b8;
pub const TIMERVALUE0_S: c_int = 16;
pub const TIMERVALUE0_M: c_uint = 0xffffU;

pub const TIMERVALUE1_S: c_int = 0;
pub const TIMERVALUE1_M: c_uint = 0xffffU;

pub const SGE_TIMER_VALUE_2_AND_3_A: c_uint = 0x10bc;
pub const TIMERVALUE2_S: c_int = 16;
pub const TIMERVALUE2_M: c_uint = 0xffffU;

pub const TIMERVALUE3_S: c_int = 0;
pub const TIMERVALUE3_M: c_uint = 0xffffU;

pub const SGE_TIMER_VALUE_4_AND_5_A: c_uint = 0x10c0;
pub const TIMERVALUE4_S: c_int = 16;
pub const TIMERVALUE4_M: c_uint = 0xffffU;

pub const TIMERVALUE5_S: c_int = 0;
pub const TIMERVALUE5_M: c_uint = 0xffffU;

pub const SGE_DEBUG_INDEX_A: c_uint = 0x10cc;
pub const SGE_DEBUG_DATA_HIGH_A: c_uint = 0x10d0;
pub const SGE_DEBUG_DATA_LOW_A: c_uint = 0x10d4;
pub const SGE_DEBUG_DATA_LOW_INDEX_2_A: c_uint = 0x12c8;
pub const SGE_DEBUG_DATA_LOW_INDEX_3_A: c_uint = 0x12cc;
pub const SGE_DEBUG_DATA_HIGH_INDEX_10_A: c_uint = 0x12a8;
pub const SGE_INGRESS_QUEUES_PER_PAGE_PF_A: c_uint = 0x10f4;
pub const SGE_INGRESS_QUEUES_PER_PAGE_VF_A: c_uint = 0x10f8;
pub const SGE_ERROR_STATS_A: c_uint = 0x1100;
pub const UNCAPTURED_ERROR_S: c_int = 18;

pub const ERROR_QID_VALID_S: c_int = 17;

pub const ERROR_QID_S: c_int = 0;
pub const ERROR_QID_M: c_uint = 0x1ffffU;

pub const SGE_INT_CAUSE5_A: c_uint = 0x110c;
pub const ERR_T_RXCRC_S: c_int = 31;

pub const HP_INT_THRESH_S: c_int = 28;
pub const HP_INT_THRESH_M: c_uint = 0xfU;

pub const HP_COUNT_S: c_int = 16;
pub const HP_COUNT_M: c_uint = 0x7ffU;

pub const LP_INT_THRESH_S: c_int = 12;
pub const LP_INT_THRESH_M: c_uint = 0xfU;

pub const LP_COUNT_S: c_int = 0;
pub const LP_COUNT_M: c_uint = 0x7ffU;

pub const LP_INT_THRESH_T5_S: c_int = 18;
pub const LP_INT_THRESH_T5_M: c_uint = 0xfffU;

pub const LP_COUNT_T5_S: c_int = 0;
pub const LP_COUNT_T5_M: c_uint = 0x3ffffU;

pub const SGE_DOORBELL_CONTROL_A: c_uint = 0x10a8;
pub const SGE_STAT_TOTAL_A: c_uint = 0x10e4;
pub const SGE_STAT_MATCH_A: c_uint = 0x10e8;
pub const SGE_STAT_CFG_A: c_uint = 0x10ec;
pub const STATMODE_S: c_int = 2;

pub const STATSOURCE_T5_S: c_int = 9;
pub const STATSOURCE_T5_M: c_uint = 0xfU;

pub const T6_STATMODE_S: c_int = 0;

pub const SGE_DBFIFO_STATUS2_A: c_uint = 0x1118;
pub const HP_INT_THRESH_T5_S: c_int = 10;
pub const HP_INT_THRESH_T5_M: c_uint = 0xfU;

pub const HP_COUNT_T5_S: c_int = 0;
pub const HP_COUNT_T5_M: c_uint = 0x3ffU;

pub const ENABLE_DROP_S: c_int = 13;

pub const DROPPED_DB_S: c_int = 0;

pub const SGE_CTXT_CMD_A: c_uint = 0x11fc;
pub const SGE_DBQ_CTXT_BADDR_A: c_uint = 0x1084;
// registers for module PCIE
pub const PCIE_PF_CFG_A: c_uint = 0x40;
pub const AIVEC_S: c_int = 4;
pub const AIVEC_M: c_uint = 0x3ffU;

pub const PCIE_PF_CLI_A: c_uint = 0x44;
pub const PCIE_PF_EXPROM_OFST_A: c_uint = 0x4c;
pub const OFFSET_S: c_int = 10;
pub const OFFSET_M: c_uint = 0x3fffU;

pub const PCIE_INT_CAUSE_A: c_uint = 0x3004;
pub const UNXSPLCPLERR_S: c_int = 29;

pub const PCIEPINT_S: c_int = 28;

pub const PCIESINT_S: c_int = 27;

pub const RPLPERR_S: c_int = 26;

pub const RXWRPERR_S: c_int = 25;

pub const RXCPLPERR_S: c_int = 24;

pub const PIOTAGPERR_S: c_int = 23;

pub const MATAGPERR_S: c_int = 22;

pub const INTXCLRPERR_S: c_int = 21;

pub const FIDPERR_S: c_int = 20;

pub const CFGSNPPERR_S: c_int = 19;

pub const HRSPPERR_S: c_int = 18;

pub const HREQPERR_S: c_int = 17;

pub const HCNTPERR_S: c_int = 16;

pub const DRSPPERR_S: c_int = 15;

pub const DREQPERR_S: c_int = 14;

pub const DCNTPERR_S: c_int = 13;

pub const CRSPPERR_S: c_int = 12;

pub const CREQPERR_S: c_int = 11;

pub const CCNTPERR_S: c_int = 10;

pub const TARTAGPERR_S: c_int = 9;

pub const PIOREQPERR_S: c_int = 8;

pub const PIOCPLPERR_S: c_int = 7;

pub const MSIXDIPERR_S: c_int = 6;

pub const MSIXDATAPERR_S: c_int = 5;

pub const MSIXADDRHPERR_S: c_int = 4;

pub const MSIXADDRLPERR_S: c_int = 3;

pub const MSIDATAPERR_S: c_int = 2;

pub const MSIADDRHPERR_S: c_int = 1;

pub const MSIADDRLPERR_S: c_int = 0;

pub const READRSPERR_S: c_int = 29;

pub const TRGT1GRPPERR_S: c_int = 28;

pub const IPSOTPERR_S: c_int = 27;

pub const IPRETRYPERR_S: c_int = 26;

pub const IPRXDATAGRPPERR_S: c_int = 25;

pub const IPRXHDRGRPPERR_S: c_int = 24;

pub const MAGRPPERR_S: c_int = 22;

pub const VFIDPERR_S: c_int = 21;

pub const HREQWRPERR_S: c_int = 16;

pub const DREQWRPERR_S: c_int = 13;

pub const CREQRDPERR_S: c_int = 11;

pub const MSTTAGQPERR_S: c_int = 10;

pub const PIOREQGRPPERR_S: c_int = 8;

pub const PIOCPLGRPPERR_S: c_int = 7;

pub const MSIXSTIPERR_S: c_int = 2;

pub const MSTTIMEOUTPERR_S: c_int = 1;

pub const MSTGRPPERR_S: c_int = 0;

pub const PCIE_NONFAT_ERR_A: c_uint = 0x3010;
pub const PCIE_CFG_SPACE_REQ_A: c_uint = 0x3060;
pub const PCIE_CFG_SPACE_DATA_A: c_uint = 0x3064;
pub const PCIE_MEM_ACCESS_BASE_WIN_A: c_uint = 0x3068;
pub const PCIEOFST_S: c_int = 10;
pub const PCIEOFST_M: c_uint = 0x3fffffU;

pub const BIR_S: c_int = 8;
pub const BIR_M: c_uint = 0x3U;

pub const WINDOW_S: c_int = 0;
pub const WINDOW_M: c_uint = 0xffU;

pub const PCIE_MEM_ACCESS_OFFSET_A: c_uint = 0x306c;
pub const ENABLE_S: c_int = 30;

pub const LOCALCFG_S: c_int = 28;

pub const FUNCTION_S: c_int = 12;

pub const REGISTER_S: c_int = 0;

pub const T6_ENABLE_S: c_int = 31;

pub const PFNUM_S: c_int = 0;

pub const PCIE_FW_A: c_uint = 0x30b8;
pub const PCIE_FW_PF_A: c_uint = 0x30bc;
pub const PCIE_CORE_UTL_SYSTEM_BUS_AGENT_STATUS_A: c_uint = 0x5908;
pub const RNPP_S: c_int = 31;

pub const RPCP_S: c_int = 29;

pub const RCIP_S: c_int = 27;

pub const RCCP_S: c_int = 26;

pub const RFTP_S: c_int = 23;

pub const PTRP_S: c_int = 20;

pub const PCIE_CORE_UTL_PCI_EXPRESS_PORT_STATUS_A: c_uint = 0x59a4;
pub const TPCP_S: c_int = 30;

pub const TNPP_S: c_int = 29;

pub const TFTP_S: c_int = 28;

pub const TCAP_S: c_int = 27;

pub const TCIP_S: c_int = 26;

pub const RCAP_S: c_int = 25;

pub const PLUP_S: c_int = 23;

pub const PLDN_S: c_int = 22;

pub const OTDD_S: c_int = 21;

pub const GTRP_S: c_int = 20;

pub const RDPE_S: c_int = 18;

pub const TDCE_S: c_int = 17;

pub const TDUE_S: c_int = 16;

// SPARE2 register contains 32-bit value at offset 0x6 in Serial INIT
// Configuration flashed on EEPROM. This value corresponds to 32-bit
// Serial Configuration Version information.
//
pub const PCIE_STATIC_SPARE2_A: c_uint = 0x5bfc;
// registers for module MC
pub const MC_INT_CAUSE_A: c_uint = 0x7518;
pub const MC_P_INT_CAUSE_A: c_uint = 0x41318;
pub const ECC_UE_INT_CAUSE_S: c_int = 2;

pub const ECC_CE_INT_CAUSE_S: c_int = 1;

pub const PERR_INT_CAUSE_S: c_int = 0;

pub const DBG_GPIO_EN_A: c_uint = 0x6010;
pub const XGMAC_PORT_CFG_A: c_uint = 0x1000;
pub const MAC_PORT_CFG_A: c_uint = 0x800;
pub const SIGNAL_DET_S: c_int = 14;

pub const MC_ECC_STATUS_A: c_uint = 0x751c;
pub const MC_P_ECC_STATUS_A: c_uint = 0x4131c;
pub const ECC_CECNT_S: c_int = 16;
pub const ECC_CECNT_M: c_uint = 0xffffU;

pub const ECC_UECNT_S: c_int = 0;
pub const ECC_UECNT_M: c_uint = 0xffffU;

pub const MC_BIST_CMD_A: c_uint = 0x7600;
pub const START_BIST_S: c_int = 31;

pub const BIST_CMD_GAP_S: c_int = 8;

pub const BIST_OPCODE_S: c_int = 0;

pub const MC_BIST_CMD_ADDR_A: c_uint = 0x7604;
pub const MC_BIST_CMD_LEN_A: c_uint = 0x7608;
pub const MC_BIST_DATA_PATTERN_A: c_uint = 0x760c;
pub const MC_BIST_STATUS_RDATA_A: c_uint = 0x7688;
// registers for module MA
pub const MA_EDRAM0_BAR_A: c_uint = 0x77c0;
pub const EDRAM0_BASE_S: c_int = 16;
pub const EDRAM0_BASE_M: c_uint = 0xfffU;

pub const EDRAM0_SIZE_S: c_int = 0;
pub const EDRAM0_SIZE_M: c_uint = 0xfffU;

pub const MA_EDRAM1_BAR_A: c_uint = 0x77c4;
pub const EDRAM1_BASE_S: c_int = 16;
pub const EDRAM1_BASE_M: c_uint = 0xfffU;

pub const EDRAM1_SIZE_S: c_int = 0;
pub const EDRAM1_SIZE_M: c_uint = 0xfffU;

pub const MA_EXT_MEMORY_BAR_A: c_uint = 0x77c8;
pub const EXT_MEM_BASE_S: c_int = 16;
pub const EXT_MEM_BASE_M: c_uint = 0xfffU;

pub const EXT_MEM_SIZE_S: c_int = 0;
pub const EXT_MEM_SIZE_M: c_uint = 0xfffU;

pub const MA_EXT_MEMORY1_BAR_A: c_uint = 0x7808;
pub const HMA_MUX_S: c_int = 5;

pub const EXT_MEM1_BASE_S: c_int = 16;
pub const EXT_MEM1_BASE_M: c_uint = 0xfffU;

pub const EXT_MEM1_SIZE_S: c_int = 0;
pub const EXT_MEM1_SIZE_M: c_uint = 0xfffU;

pub const MA_EXT_MEMORY0_BAR_A: c_uint = 0x77c8;
pub const EXT_MEM0_BASE_S: c_int = 16;
pub const EXT_MEM0_BASE_M: c_uint = 0xfffU;

pub const EXT_MEM0_SIZE_S: c_int = 0;
pub const EXT_MEM0_SIZE_M: c_uint = 0xfffU;

pub const MA_TARGET_MEM_ENABLE_A: c_uint = 0x77d8;
pub const EXT_MEM_ENABLE_S: c_int = 2;

pub const EDRAM1_ENABLE_S: c_int = 1;

pub const EDRAM0_ENABLE_S: c_int = 0;

pub const EXT_MEM1_ENABLE_S: c_int = 4;

pub const EXT_MEM0_ENABLE_S: c_int = 2;

pub const MA_INT_CAUSE_A: c_uint = 0x77e0;
pub const MEM_PERR_INT_CAUSE_S: c_int = 1;

pub const MEM_WRAP_INT_CAUSE_S: c_int = 0;

pub const MA_INT_WRAP_STATUS_A: c_uint = 0x77e4;
pub const MEM_WRAP_ADDRESS_S: c_int = 4;
pub const MEM_WRAP_ADDRESS_M: c_uint = 0xfffffffU;

pub const MEM_WRAP_CLIENT_NUM_S: c_int = 0;
pub const MEM_WRAP_CLIENT_NUM_M: c_uint = 0xfU;

pub const MA_PARITY_ERROR_STATUS_A: c_uint = 0x77f4;
pub const MA_PARITY_ERROR_STATUS1_A: c_uint = 0x77f4;
pub const MA_PARITY_ERROR_STATUS2_A: c_uint = 0x7804;
// registers for module EDC_0
pub const EDC_0_BASE_ADDR: c_uint = 0x7900;
pub const EDC_BIST_CMD_A: c_uint = 0x7904;
pub const EDC_BIST_CMD_ADDR_A: c_uint = 0x7908;
pub const EDC_BIST_CMD_LEN_A: c_uint = 0x790c;
pub const EDC_BIST_DATA_PATTERN_A: c_uint = 0x7910;
pub const EDC_BIST_STATUS_RDATA_A: c_uint = 0x7928;
pub const EDC_INT_CAUSE_A: c_uint = 0x7978;
pub const ECC_UE_PAR_S: c_int = 5;

pub const ECC_CE_PAR_S: c_int = 4;

pub const PERR_PAR_CAUSE_S: c_int = 3;

pub const EDC_ECC_STATUS_A: c_uint = 0x797c;
// registers for module EDC_1
pub const EDC_1_BASE_ADDR: c_uint = 0x7980;
// registers for module CIM
pub const CIM_BOOT_CFG_A: c_uint = 0x7b00;
pub const CIM_SDRAM_BASE_ADDR_A: c_uint = 0x7b14;
pub const CIM_SDRAM_ADDR_SIZE_A: c_uint = 0x7b18;
pub const CIM_EXTMEM2_BASE_ADDR_A: c_uint = 0x7b1c;
pub const CIM_EXTMEM2_ADDR_SIZE_A: c_uint = 0x7b20;
pub const CIM_PF_MAILBOX_CTRL_SHADOW_COPY_A: c_uint = 0x290;
pub const BOOTADDR_M: c_uint = 0xffffff00U;
pub const UPCRST_S: c_int = 0;

pub const CIM_PF_MAILBOX_DATA_A: c_uint = 0x240;
pub const CIM_PF_MAILBOX_CTRL_A: c_uint = 0x280;
pub const MBMSGVALID_S: c_int = 3;

pub const MBINTREQ_S: c_int = 2;

pub const MBOWNER_S: c_int = 0;
pub const MBOWNER_M: c_uint = 0x3U;

pub const CIM_PF_HOST_INT_ENABLE_A: c_uint = 0x288;
pub const MBMSGRDYINTEN_S: c_int = 19;

pub const CIM_PF_HOST_INT_CAUSE_A: c_uint = 0x28c;
pub const MBMSGRDYINT_S: c_int = 19;

pub const CIM_HOST_INT_CAUSE_A: c_uint = 0x7b2c;
pub const TIEQOUTPARERRINT_S: c_int = 20;

pub const TIEQINPARERRINT_S: c_int = 19;

pub const TIMER0INT_S: c_int = 2;

pub const PREFDROPINT_S: c_int = 1;

pub const UPACCNONZERO_S: c_int = 0;

pub const MBHOSTPARERR_S: c_int = 18;

pub const MBUPPARERR_S: c_int = 17;

pub const IBQTP0PARERR_S: c_int = 16;

pub const IBQTP1PARERR_S: c_int = 15;

pub const IBQULPPARERR_S: c_int = 14;

pub const IBQSGELOPARERR_S: c_int = 13;

pub const IBQSGEHIPARERR_S: c_int = 12;

pub const IBQNCSIPARERR_S: c_int = 11;

pub const OBQULP0PARERR_S: c_int = 10;

pub const OBQULP1PARERR_S: c_int = 9;

pub const OBQULP2PARERR_S: c_int = 8;

pub const OBQULP3PARERR_S: c_int = 7;

pub const OBQSGEPARERR_S: c_int = 6;

pub const OBQNCSIPARERR_S: c_int = 5;

pub const CIM_HOST_UPACC_INT_CAUSE_A: c_uint = 0x7b34;
pub const EEPROMWRINT_S: c_int = 30;

pub const TIMEOUTMAINT_S: c_int = 29;

pub const TIMEOUTINT_S: c_int = 28;

pub const RSPOVRLOOKUPINT_S: c_int = 27;

pub const REQOVRLOOKUPINT_S: c_int = 26;

pub const BLKWRPLINT_S: c_int = 25;

pub const BLKRDPLINT_S: c_int = 24;

pub const SGLWRPLINT_S: c_int = 23;

pub const SGLRDPLINT_S: c_int = 22;

pub const BLKWRCTLINT_S: c_int = 21;

pub const BLKRDCTLINT_S: c_int = 20;

pub const SGLWRCTLINT_S: c_int = 19;

pub const SGLRDCTLINT_S: c_int = 18;

pub const BLKWREEPROMINT_S: c_int = 17;

pub const BLKRDEEPROMINT_S: c_int = 16;

pub const SGLWREEPROMINT_S: c_int = 15;

pub const SGLRDEEPROMINT_S: c_int = 14;

pub const BLKWRFLASHINT_S: c_int = 13;

pub const BLKRDFLASHINT_S: c_int = 12;

pub const SGLWRFLASHINT_S: c_int = 11;

pub const SGLRDFLASHINT_S: c_int = 10;

pub const BLKWRBOOTINT_S: c_int = 9;

pub const BLKRDBOOTINT_S: c_int = 8;

pub const SGLWRBOOTINT_S: c_int = 7;

pub const SGLRDBOOTINT_S: c_int = 6;

pub const ILLWRBEINT_S: c_int = 5;

pub const ILLRDBEINT_S: c_int = 4;

pub const ILLRDINT_S: c_int = 3;

pub const ILLWRINT_S: c_int = 2;

pub const ILLTRANSINT_S: c_int = 1;

pub const RSVDSPACEINT_S: c_int = 0;

// registers for module TP
pub const DBGLAWHLF_S: c_int = 23;

pub const DBGLAWPTR_S: c_int = 16;
pub const DBGLAWPTR_M: c_uint = 0x7fU;

pub const DBGLAENABLE_S: c_int = 12;

pub const DBGLARPTR_S: c_int = 0;
pub const DBGLARPTR_M: c_uint = 0x7fU;

pub const CRXPKTENC_S: c_int = 3;

pub const TP_DBG_LA_DATAL_A: c_uint = 0x7ed8;
pub const TP_DBG_LA_CONFIG_A: c_uint = 0x7ed4;
pub const TP_OUT_CONFIG_A: c_uint = 0x7d04;
pub const TP_GLOBAL_CONFIG_A: c_uint = 0x7d08;
pub const ACTIVEFILTERCOUNTS_S: c_int = 22;

pub const TP_CMM_TCB_BASE_A: c_uint = 0x7d10;
pub const TP_CMM_MM_BASE_A: c_uint = 0x7d14;
pub const TP_CMM_TIMER_BASE_A: c_uint = 0x7d18;
pub const TP_PMM_TX_BASE_A: c_uint = 0x7d20;
pub const TP_PMM_RX_BASE_A: c_uint = 0x7d28;
pub const TP_PMM_RX_PAGE_SIZE_A: c_uint = 0x7d2c;
pub const TP_PMM_RX_MAX_PAGE_A: c_uint = 0x7d30;
pub const TP_PMM_TX_PAGE_SIZE_A: c_uint = 0x7d34;
pub const TP_PMM_TX_MAX_PAGE_A: c_uint = 0x7d38;
pub const TP_CMM_MM_MAX_PSTRUCT_A: c_uint = 0x7e6c;
pub const PMRXNUMCHN_S: c_int = 31;

pub const PMTXNUMCHN_S: c_int = 30;
pub const PMTXNUMCHN_M: c_uint = 0x3U;

pub const PMTXMAXPAGE_S: c_int = 0;
pub const PMTXMAXPAGE_M: c_uint = 0x1fffffU;

pub const PMRXMAXPAGE_S: c_int = 0;
pub const PMRXMAXPAGE_M: c_uint = 0x1fffffU;

pub const DBGLAMODE_S: c_int = 14;
pub const DBGLAMODE_M: c_uint = 0x3U;

pub const FIVETUPLELOOKUP_S: c_int = 17;
pub const FIVETUPLELOOKUP_M: c_uint = 0x3U;

pub const TP_PARA_REG2_A: c_uint = 0x7d68;
pub const MAXRXDATA_S: c_int = 16;
pub const MAXRXDATA_M: c_uint = 0xffffU;

pub const TP_TIMER_RESOLUTION_A: c_uint = 0x7d90;
pub const TIMERRESOLUTION_S: c_int = 16;
pub const TIMERRESOLUTION_M: c_uint = 0xffU;

pub const TIMESTAMPRESOLUTION_S: c_int = 8;
pub const TIMESTAMPRESOLUTION_M: c_uint = 0xffU;

pub const DELAYEDACKRESOLUTION_S: c_int = 0;
pub const DELAYEDACKRESOLUTION_M: c_uint = 0xffU;

pub const TP_SHIFT_CNT_A: c_uint = 0x7dc0;
pub const TP_RXT_MIN_A: c_uint = 0x7d98;
pub const TP_RXT_MAX_A: c_uint = 0x7d9c;
pub const TP_PERS_MIN_A: c_uint = 0x7da0;
pub const TP_PERS_MAX_A: c_uint = 0x7da4;
pub const TP_KEEP_IDLE_A: c_uint = 0x7da8;
pub const TP_KEEP_INTVL_A: c_uint = 0x7dac;
pub const TP_INIT_SRTT_A: c_uint = 0x7db0;
pub const TP_DACK_TIMER_A: c_uint = 0x7db4;
pub const TP_FINWAIT2_TIMER_A: c_uint = 0x7db8;
pub const INITSRTT_S: c_int = 0;
pub const INITSRTT_M: c_uint = 0xffffU;

pub const PERSMAX_S: c_int = 0;
pub const PERSMAX_M: c_uint = 0x3fffffffU;

pub const SYNSHIFTMAX_S: c_int = 24;
pub const SYNSHIFTMAX_M: c_uint = 0xffU;

pub const RXTSHIFTMAXR1_S: c_int = 20;
pub const RXTSHIFTMAXR1_M: c_uint = 0xfU;

pub const RXTSHIFTMAXR2_S: c_int = 16;
pub const RXTSHIFTMAXR2_M: c_uint = 0xfU;

pub const PERSHIFTBACKOFFMAX_S: c_int = 12;
pub const PERSHIFTBACKOFFMAX_M: c_uint = 0xfU;

pub const PERSHIFTMAX_S: c_int = 8;
pub const PERSHIFTMAX_M: c_uint = 0xfU;

pub const KEEPALIVEMAXR1_S: c_int = 4;
pub const KEEPALIVEMAXR1_M: c_uint = 0xfU;

pub const KEEPALIVEMAXR2_S: c_int = 0;
pub const KEEPALIVEMAXR2_M: c_uint = 0xfU;

pub const ROWINDEX_S: c_int = 16;

pub const TP_CCTRL_TABLE_A: c_uint = 0x7ddc;
pub const TP_PACE_TABLE_A: c_uint = 0x7dd8;
pub const TP_MTU_TABLE_A: c_uint = 0x7de4;
pub const MTUINDEX_S: c_int = 24;

pub const MTUWIDTH_S: c_int = 16;
pub const MTUWIDTH_M: c_uint = 0xfU;

pub const MTUVALUE_S: c_int = 0;
pub const MTUVALUE_M: c_uint = 0x3fffU;

pub const TP_RSS_LKP_TABLE_A: c_uint = 0x7dec;
pub const TP_CMM_MM_RX_FLST_BASE_A: c_uint = 0x7e60;
pub const TP_CMM_MM_TX_FLST_BASE_A: c_uint = 0x7e64;
pub const TP_CMM_MM_PS_FLST_BASE_A: c_uint = 0x7e68;
pub const LKPTBLROWVLD_S: c_int = 31;

pub const LKPTBLQUEUE1_S: c_int = 10;
pub const LKPTBLQUEUE1_M: c_uint = 0x3ffU;

pub const LKPTBLQUEUE0_S: c_int = 0;
pub const LKPTBLQUEUE0_M: c_uint = 0x3ffU;

pub const TP_TM_PIO_ADDR_A: c_uint = 0x7e18;
pub const TP_TM_PIO_DATA_A: c_uint = 0x7e1c;
pub const TP_MOD_CONFIG_A: c_uint = 0x7e24;
pub const TIMERMODE_S: c_int = 8;
pub const TIMERMODE_M: c_uint = 0xffU;

pub const TP_TX_MOD_Q1_Q0_TIMER_SEPARATOR_A: c_uint = 0x3;
pub const TP_TX_MOD_Q1_Q0_RATE_LIMIT_A: c_uint = 0x8;
pub const TP_PIO_ADDR_A: c_uint = 0x7e40;
pub const TP_PIO_DATA_A: c_uint = 0x7e44;
pub const TP_MIB_INDEX_A: c_uint = 0x7e50;
pub const TP_MIB_DATA_A: c_uint = 0x7e54;
pub const TP_INT_CAUSE_A: c_uint = 0x7e74;
pub const TP_FLM_FREE_PS_CNT_A: c_uint = 0x7e80;
pub const TP_FLM_FREE_RX_CNT_A: c_uint = 0x7e84;
pub const FREEPSTRUCTCOUNT_S: c_int = 0;
pub const FREEPSTRUCTCOUNT_M: c_uint = 0x1fffffU;

pub const FREERXPAGECOUNT_S: c_int = 0;
pub const FREERXPAGECOUNT_M: c_uint = 0x1fffffU;

pub const TP_FLM_FREE_TX_CNT_A: c_uint = 0x7e88;
pub const FREETXPAGECOUNT_S: c_int = 0;
pub const FREETXPAGECOUNT_M: c_uint = 0x1fffffU;

pub const FLMTXFLSTEMPTY_S: c_int = 30;

pub const TP_TX_ORATE_A: c_uint = 0x7ebc;
pub const OFDRATE3_S: c_int = 24;
pub const OFDRATE3_M: c_uint = 0xffU;

pub const OFDRATE2_S: c_int = 16;
pub const OFDRATE2_M: c_uint = 0xffU;

pub const OFDRATE1_S: c_int = 8;
pub const OFDRATE1_M: c_uint = 0xffU;

pub const OFDRATE0_S: c_int = 0;
pub const OFDRATE0_M: c_uint = 0xffU;

pub const TP_TX_TRATE_A: c_uint = 0x7ed0;
pub const TNLRATE3_S: c_int = 24;
pub const TNLRATE3_M: c_uint = 0xffU;

pub const TNLRATE2_S: c_int = 16;
pub const TNLRATE2_M: c_uint = 0xffU;

pub const TNLRATE1_S: c_int = 8;
pub const TNLRATE1_M: c_uint = 0xffU;

pub const TNLRATE0_S: c_int = 0;
pub const TNLRATE0_M: c_uint = 0xffU;

pub const TP_VLAN_PRI_MAP_A: c_uint = 0x140;
pub const FRAGMENTATION_S: c_int = 9;

pub const MPSHITTYPE_S: c_int = 8;

pub const MACMATCH_S: c_int = 7;

pub const ETHERTYPE_S: c_int = 6;

pub const PROTOCOL_S: c_int = 5;

pub const TOS_S: c_int = 4;

pub const VLAN_S: c_int = 3;

pub const VNIC_ID_S: c_int = 2;

pub const PORT_S: c_int = 1;

pub const FCOE_S: c_int = 0;

pub const FILTERMODE_S: c_int = 15;

pub const FCOEMASK_S: c_int = 14;

pub const TP_INGRESS_CONFIG_A: c_uint = 0x141;
pub const VNIC_S: c_int = 11;

pub const USE_ENC_IDX_S: c_int = 13;

pub const CSUM_HAS_PSEUDO_HDR_S: c_int = 10;

pub const TP_MIB_MAC_IN_ERR_0_A: c_uint = 0x0;
pub const TP_MIB_HDR_IN_ERR_0_A: c_uint = 0x4;
pub const TP_MIB_TCP_IN_ERR_0_A: c_uint = 0x8;
pub const TP_MIB_TCP_OUT_RST_A: c_uint = 0xc;
pub const TP_MIB_TCP_IN_SEG_HI_A: c_uint = 0x10;
pub const TP_MIB_TCP_IN_SEG_LO_A: c_uint = 0x11;
pub const TP_MIB_TCP_OUT_SEG_HI_A: c_uint = 0x12;
pub const TP_MIB_TCP_OUT_SEG_LO_A: c_uint = 0x13;
pub const TP_MIB_TCP_RXT_SEG_HI_A: c_uint = 0x14;
pub const TP_MIB_TCP_RXT_SEG_LO_A: c_uint = 0x15;
pub const TP_MIB_TNL_CNG_DROP_0_A: c_uint = 0x18;
pub const TP_MIB_OFD_CHN_DROP_0_A: c_uint = 0x1c;
pub const TP_MIB_TCP_V6IN_ERR_0_A: c_uint = 0x28;
pub const TP_MIB_TCP_V6OUT_RST_A: c_uint = 0x2c;
pub const TP_MIB_OFD_ARP_DROP_A: c_uint = 0x36;
pub const TP_MIB_CPL_IN_REQ_0_A: c_uint = 0x38;
pub const TP_MIB_CPL_OUT_RSP_0_A: c_uint = 0x3c;
pub const TP_MIB_TNL_DROP_0_A: c_uint = 0x44;
pub const TP_MIB_FCOE_DDP_0_A: c_uint = 0x48;
pub const TP_MIB_FCOE_DROP_0_A: c_uint = 0x4c;
pub const TP_MIB_FCOE_BYTE_0_HI_A: c_uint = 0x50;
pub const TP_MIB_OFD_VLN_DROP_0_A: c_uint = 0x58;
pub const TP_MIB_USM_PKTS_A: c_uint = 0x5c;
pub const TP_MIB_RQE_DFR_PKT_A: c_uint = 0x64;
pub const ULP_TX_INT_CAUSE_A: c_uint = 0x8dcc;
pub const ULP_TX_TPT_LLIMIT_A: c_uint = 0x8dd4;
pub const ULP_TX_TPT_ULIMIT_A: c_uint = 0x8dd8;
pub const ULP_TX_PBL_LLIMIT_A: c_uint = 0x8ddc;
pub const ULP_TX_PBL_ULIMIT_A: c_uint = 0x8de0;
pub const ULP_TX_ERR_TABLE_BASE_A: c_uint = 0x8e04;
pub const PBL_BOUND_ERR_CH3_S: c_int = 31;

pub const PBL_BOUND_ERR_CH2_S: c_int = 30;

pub const PBL_BOUND_ERR_CH1_S: c_int = 29;

pub const PBL_BOUND_ERR_CH0_S: c_int = 28;

pub const PM_RX_INT_CAUSE_A: c_uint = 0x8fdc;
pub const PM_RX_STAT_CONFIG_A: c_uint = 0x8fc8;
pub const PM_RX_STAT_COUNT_A: c_uint = 0x8fcc;
pub const PM_RX_STAT_LSB_A: c_uint = 0x8fd0;
pub const PM_RX_DBG_CTRL_A: c_uint = 0x8fd0;
pub const PM_RX_DBG_DATA_A: c_uint = 0x8fd4;
pub const PM_RX_DBG_STAT_MSB_A: c_uint = 0x10013;
pub const PMRX_FRAMING_ERROR_F: c_uint = 0x003ffff0U;
pub const ZERO_E_CMD_ERROR_S: c_int = 22;

pub const OCSPI_PAR_ERROR_S: c_int = 3;

pub const DB_OPTIONS_PAR_ERROR_S: c_int = 2;

pub const IESPI_PAR_ERROR_S: c_int = 1;

pub const ULP_TX_LA_RDPTR_0_A: c_uint = 0x8ec0;
pub const ULP_TX_LA_RDDATA_0_A: c_uint = 0x8ec4;
pub const ULP_TX_LA_WRPTR_0_A: c_uint = 0x8ec8;
pub const ULP_TX_ASIC_DEBUG_CTRL_A: c_uint = 0x8f70;
pub const ULP_TX_ASIC_DEBUG_0_A: c_uint = 0x8f74;
pub const ULP_TX_ASIC_DEBUG_1_A: c_uint = 0x8f78;
pub const ULP_TX_ASIC_DEBUG_2_A: c_uint = 0x8f7c;
pub const ULP_TX_ASIC_DEBUG_3_A: c_uint = 0x8f80;
pub const ULP_TX_ASIC_DEBUG_4_A: c_uint = 0x8f84;
// registers for module PM_RX
pub const PM_RX_BASE_ADDR: c_uint = 0x8fc0;
pub const PMRX_E_PCMD_PAR_ERROR_S: c_int = 0;

pub const PM_TX_INT_CAUSE_A: c_uint = 0x8ffc;
pub const PM_TX_STAT_CONFIG_A: c_uint = 0x8fe8;
pub const PM_TX_STAT_COUNT_A: c_uint = 0x8fec;
pub const PM_TX_STAT_LSB_A: c_uint = 0x8ff0;
pub const PM_TX_DBG_CTRL_A: c_uint = 0x8ff0;
pub const PM_TX_DBG_DATA_A: c_uint = 0x8ff4;
pub const PM_TX_DBG_STAT_MSB_A: c_uint = 0x1001a;
pub const PCMD_LEN_OVFL0_S: c_int = 31;

pub const PCMD_LEN_OVFL1_S: c_int = 30;

pub const PCMD_LEN_OVFL2_S: c_int = 29;

pub const ZERO_C_CMD_ERROR_S: c_int = 28;

pub const PMTX_FRAMING_ERROR_F: c_uint = 0x0ffffff0U;
pub const OESPI_PAR_ERROR_S: c_int = 3;

pub const ICSPI_PAR_ERROR_S: c_int = 1;

pub const PMTX_C_PCMD_PAR_ERROR_S: c_int = 0;

pub const MPS_PORT_STAT_TX_PORT_BYTES_L: c_uint = 0x400;
pub const MPS_PORT_STAT_TX_PORT_BYTES_H: c_uint = 0x404;
pub const MPS_PORT_STAT_TX_PORT_FRAMES_L: c_uint = 0x408;
pub const MPS_PORT_STAT_TX_PORT_FRAMES_H: c_uint = 0x40c;
pub const MPS_PORT_STAT_TX_PORT_BCAST_L: c_uint = 0x410;
pub const MPS_PORT_STAT_TX_PORT_BCAST_H: c_uint = 0x414;
pub const MPS_PORT_STAT_TX_PORT_MCAST_L: c_uint = 0x418;
pub const MPS_PORT_STAT_TX_PORT_MCAST_H: c_uint = 0x41c;
pub const MPS_PORT_STAT_TX_PORT_UCAST_L: c_uint = 0x420;
pub const MPS_PORT_STAT_TX_PORT_UCAST_H: c_uint = 0x424;
pub const MPS_PORT_STAT_TX_PORT_ERROR_L: c_uint = 0x428;
pub const MPS_PORT_STAT_TX_PORT_ERROR_H: c_uint = 0x42c;
pub const MPS_PORT_STAT_TX_PORT_64B_L: c_uint = 0x430;
pub const MPS_PORT_STAT_TX_PORT_64B_H: c_uint = 0x434;
pub const MPS_PORT_STAT_TX_PORT_65B_127B_L: c_uint = 0x438;
pub const MPS_PORT_STAT_TX_PORT_65B_127B_H: c_uint = 0x43c;
pub const MPS_PORT_STAT_TX_PORT_128B_255B_L: c_uint = 0x440;
pub const MPS_PORT_STAT_TX_PORT_128B_255B_H: c_uint = 0x444;
pub const MPS_PORT_STAT_TX_PORT_256B_511B_L: c_uint = 0x448;
pub const MPS_PORT_STAT_TX_PORT_256B_511B_H: c_uint = 0x44c;
pub const MPS_PORT_STAT_TX_PORT_512B_1023B_L: c_uint = 0x450;
pub const MPS_PORT_STAT_TX_PORT_512B_1023B_H: c_uint = 0x454;
pub const MPS_PORT_STAT_TX_PORT_1024B_1518B_L: c_uint = 0x458;
pub const MPS_PORT_STAT_TX_PORT_1024B_1518B_H: c_uint = 0x45c;
pub const MPS_PORT_STAT_TX_PORT_1519B_MAX_L: c_uint = 0x460;
pub const MPS_PORT_STAT_TX_PORT_1519B_MAX_H: c_uint = 0x464;
pub const MPS_PORT_STAT_TX_PORT_DROP_L: c_uint = 0x468;
pub const MPS_PORT_STAT_TX_PORT_DROP_H: c_uint = 0x46c;
pub const MPS_PORT_STAT_TX_PORT_PAUSE_L: c_uint = 0x470;
pub const MPS_PORT_STAT_TX_PORT_PAUSE_H: c_uint = 0x474;
pub const MPS_PORT_STAT_TX_PORT_PPP0_L: c_uint = 0x478;
pub const MPS_PORT_STAT_TX_PORT_PPP0_H: c_uint = 0x47c;
pub const MPS_PORT_STAT_TX_PORT_PPP1_L: c_uint = 0x480;
pub const MPS_PORT_STAT_TX_PORT_PPP1_H: c_uint = 0x484;
pub const MPS_PORT_STAT_TX_PORT_PPP2_L: c_uint = 0x488;
pub const MPS_PORT_STAT_TX_PORT_PPP2_H: c_uint = 0x48c;
pub const MPS_PORT_STAT_TX_PORT_PPP3_L: c_uint = 0x490;
pub const MPS_PORT_STAT_TX_PORT_PPP3_H: c_uint = 0x494;
pub const MPS_PORT_STAT_TX_PORT_PPP4_L: c_uint = 0x498;
pub const MPS_PORT_STAT_TX_PORT_PPP4_H: c_uint = 0x49c;
pub const MPS_PORT_STAT_TX_PORT_PPP5_L: c_uint = 0x4a0;
pub const MPS_PORT_STAT_TX_PORT_PPP5_H: c_uint = 0x4a4;
pub const MPS_PORT_STAT_TX_PORT_PPP6_L: c_uint = 0x4a8;
pub const MPS_PORT_STAT_TX_PORT_PPP6_H: c_uint = 0x4ac;
pub const MPS_PORT_STAT_TX_PORT_PPP7_L: c_uint = 0x4b0;
pub const MPS_PORT_STAT_TX_PORT_PPP7_H: c_uint = 0x4b4;
pub const MPS_PORT_STAT_LB_PORT_BYTES_L: c_uint = 0x4c0;
pub const MPS_PORT_STAT_LB_PORT_BYTES_H: c_uint = 0x4c4;
pub const MPS_PORT_STAT_LB_PORT_FRAMES_L: c_uint = 0x4c8;
pub const MPS_PORT_STAT_LB_PORT_FRAMES_H: c_uint = 0x4cc;
pub const MPS_PORT_STAT_LB_PORT_BCAST_L: c_uint = 0x4d0;
pub const MPS_PORT_STAT_LB_PORT_BCAST_H: c_uint = 0x4d4;
pub const MPS_PORT_STAT_LB_PORT_MCAST_L: c_uint = 0x4d8;
pub const MPS_PORT_STAT_LB_PORT_MCAST_H: c_uint = 0x4dc;
pub const MPS_PORT_STAT_LB_PORT_UCAST_L: c_uint = 0x4e0;
pub const MPS_PORT_STAT_LB_PORT_UCAST_H: c_uint = 0x4e4;
pub const MPS_PORT_STAT_LB_PORT_ERROR_L: c_uint = 0x4e8;
pub const MPS_PORT_STAT_LB_PORT_ERROR_H: c_uint = 0x4ec;
pub const MPS_PORT_STAT_LB_PORT_64B_L: c_uint = 0x4f0;
pub const MPS_PORT_STAT_LB_PORT_64B_H: c_uint = 0x4f4;
pub const MPS_PORT_STAT_LB_PORT_65B_127B_L: c_uint = 0x4f8;
pub const MPS_PORT_STAT_LB_PORT_65B_127B_H: c_uint = 0x4fc;
pub const MPS_PORT_STAT_LB_PORT_128B_255B_L: c_uint = 0x500;
pub const MPS_PORT_STAT_LB_PORT_128B_255B_H: c_uint = 0x504;
pub const MPS_PORT_STAT_LB_PORT_256B_511B_L: c_uint = 0x508;
pub const MPS_PORT_STAT_LB_PORT_256B_511B_H: c_uint = 0x50c;
pub const MPS_PORT_STAT_LB_PORT_512B_1023B_L: c_uint = 0x510;
pub const MPS_PORT_STAT_LB_PORT_512B_1023B_H: c_uint = 0x514;
pub const MPS_PORT_STAT_LB_PORT_1024B_1518B_L: c_uint = 0x518;
pub const MPS_PORT_STAT_LB_PORT_1024B_1518B_H: c_uint = 0x51c;
pub const MPS_PORT_STAT_LB_PORT_1519B_MAX_L: c_uint = 0x520;
pub const MPS_PORT_STAT_LB_PORT_1519B_MAX_H: c_uint = 0x524;
pub const MPS_PORT_STAT_LB_PORT_DROP_FRAMES: c_uint = 0x528;
pub const MPS_PORT_STAT_LB_PORT_DROP_FRAMES_L: c_uint = 0x528;
pub const MPS_PORT_STAT_RX_PORT_BYTES_L: c_uint = 0x540;
pub const MPS_PORT_STAT_RX_PORT_BYTES_H: c_uint = 0x544;
pub const MPS_PORT_STAT_RX_PORT_FRAMES_L: c_uint = 0x548;
pub const MPS_PORT_STAT_RX_PORT_FRAMES_H: c_uint = 0x54c;
pub const MPS_PORT_STAT_RX_PORT_BCAST_L: c_uint = 0x550;
pub const MPS_PORT_STAT_RX_PORT_BCAST_H: c_uint = 0x554;
pub const MPS_PORT_STAT_RX_PORT_MCAST_L: c_uint = 0x558;
pub const MPS_PORT_STAT_RX_PORT_MCAST_H: c_uint = 0x55c;
pub const MPS_PORT_STAT_RX_PORT_UCAST_L: c_uint = 0x560;
pub const MPS_PORT_STAT_RX_PORT_UCAST_H: c_uint = 0x564;
pub const MPS_PORT_STAT_RX_PORT_MTU_ERROR_L: c_uint = 0x568;
pub const MPS_PORT_STAT_RX_PORT_MTU_ERROR_H: c_uint = 0x56c;
pub const MPS_PORT_STAT_RX_PORT_MTU_CRC_ERROR_L: c_uint = 0x570;
pub const MPS_PORT_STAT_RX_PORT_MTU_CRC_ERROR_H: c_uint = 0x574;
pub const MPS_PORT_STAT_RX_PORT_CRC_ERROR_L: c_uint = 0x578;
pub const MPS_PORT_STAT_RX_PORT_CRC_ERROR_H: c_uint = 0x57c;
pub const MPS_PORT_STAT_RX_PORT_LEN_ERROR_L: c_uint = 0x580;
pub const MPS_PORT_STAT_RX_PORT_LEN_ERROR_H: c_uint = 0x584;
pub const MPS_PORT_STAT_RX_PORT_SYM_ERROR_L: c_uint = 0x588;
pub const MPS_PORT_STAT_RX_PORT_SYM_ERROR_H: c_uint = 0x58c;
pub const MPS_PORT_STAT_RX_PORT_64B_L: c_uint = 0x590;
pub const MPS_PORT_STAT_RX_PORT_64B_H: c_uint = 0x594;
pub const MPS_PORT_STAT_RX_PORT_65B_127B_L: c_uint = 0x598;
pub const MPS_PORT_STAT_RX_PORT_65B_127B_H: c_uint = 0x59c;
pub const MPS_PORT_STAT_RX_PORT_128B_255B_L: c_uint = 0x5a0;
pub const MPS_PORT_STAT_RX_PORT_128B_255B_H: c_uint = 0x5a4;
pub const MPS_PORT_STAT_RX_PORT_256B_511B_L: c_uint = 0x5a8;
pub const MPS_PORT_STAT_RX_PORT_256B_511B_H: c_uint = 0x5ac;
pub const MPS_PORT_STAT_RX_PORT_512B_1023B_L: c_uint = 0x5b0;
pub const MPS_PORT_STAT_RX_PORT_512B_1023B_H: c_uint = 0x5b4;
pub const MPS_PORT_STAT_RX_PORT_1024B_1518B_L: c_uint = 0x5b8;
pub const MPS_PORT_STAT_RX_PORT_1024B_1518B_H: c_uint = 0x5bc;
pub const MPS_PORT_STAT_RX_PORT_1519B_MAX_L: c_uint = 0x5c0;
pub const MPS_PORT_STAT_RX_PORT_1519B_MAX_H: c_uint = 0x5c4;
pub const MPS_PORT_STAT_RX_PORT_PAUSE_L: c_uint = 0x5c8;
pub const MPS_PORT_STAT_RX_PORT_PAUSE_H: c_uint = 0x5cc;
pub const MPS_PORT_STAT_RX_PORT_PPP0_L: c_uint = 0x5d0;
pub const MPS_PORT_STAT_RX_PORT_PPP0_H: c_uint = 0x5d4;
pub const MPS_PORT_STAT_RX_PORT_PPP1_L: c_uint = 0x5d8;
pub const MPS_PORT_STAT_RX_PORT_PPP1_H: c_uint = 0x5dc;
pub const MPS_PORT_STAT_RX_PORT_PPP2_L: c_uint = 0x5e0;
pub const MPS_PORT_STAT_RX_PORT_PPP2_H: c_uint = 0x5e4;
pub const MPS_PORT_STAT_RX_PORT_PPP3_L: c_uint = 0x5e8;
pub const MPS_PORT_STAT_RX_PORT_PPP3_H: c_uint = 0x5ec;
pub const MPS_PORT_STAT_RX_PORT_PPP4_L: c_uint = 0x5f0;
pub const MPS_PORT_STAT_RX_PORT_PPP4_H: c_uint = 0x5f4;
pub const MPS_PORT_STAT_RX_PORT_PPP5_L: c_uint = 0x5f8;
pub const MPS_PORT_STAT_RX_PORT_PPP5_H: c_uint = 0x5fc;
pub const MPS_PORT_STAT_RX_PORT_PPP6_L: c_uint = 0x600;
pub const MPS_PORT_STAT_RX_PORT_PPP6_H: c_uint = 0x604;
pub const MPS_PORT_STAT_RX_PORT_PPP7_L: c_uint = 0x608;
pub const MPS_PORT_STAT_RX_PORT_PPP7_H: c_uint = 0x60c;
pub const MPS_PORT_STAT_RX_PORT_LESS_64B_L: c_uint = 0x610;
pub const MPS_PORT_STAT_RX_PORT_LESS_64B_H: c_uint = 0x614;
pub const MAC_PORT_MAGIC_MACID_LO: c_uint = 0x824;
pub const MAC_PORT_MAGIC_MACID_HI: c_uint = 0x828;
pub const MAC_PORT_TX_TS_VAL_LO: c_uint = 0x928;
pub const MAC_PORT_TX_TS_VAL_HI: c_uint = 0x92c;
pub const MAC_PORT_EPIO_DATA0_A: c_uint = 0x8c0;
pub const MAC_PORT_EPIO_DATA1_A: c_uint = 0x8c4;
pub const MAC_PORT_EPIO_DATA2_A: c_uint = 0x8c8;
pub const MAC_PORT_EPIO_DATA3_A: c_uint = 0x8cc;
pub const MAC_PORT_EPIO_OP_A: c_uint = 0x8d0;
pub const MAC_PORT_CFG2_A: c_uint = 0x818;
pub const MAC_PORT_PTP_SUM_LO_A: c_uint = 0x990;
pub const MAC_PORT_PTP_SUM_HI_A: c_uint = 0x994;
pub const MPS_CMN_CTL_A: c_uint = 0x9000;
pub const COUNTPAUSEMCRX_S: c_int = 5;

pub const COUNTPAUSESTATRX_S: c_int = 4;

pub const COUNTPAUSEMCTX_S: c_int = 3;

pub const COUNTPAUSESTATTX_S: c_int = 2;

pub const NUMPORTS_S: c_int = 0;
pub const NUMPORTS_M: c_uint = 0x3U;

pub const MPS_INT_CAUSE_A: c_uint = 0x9008;
pub const MPS_TX_INT_CAUSE_A: c_uint = 0x9408;
pub const MPS_STAT_CTL_A: c_uint = 0x9600;
pub const FRMERR_S: c_int = 15;

pub const SECNTERR_S: c_int = 14;

pub const BUBBLE_S: c_int = 13;

pub const TXDESCFIFO_S: c_int = 9;
pub const TXDESCFIFO_M: c_uint = 0xfU;

pub const TXDATAFIFO_S: c_int = 5;
pub const TXDATAFIFO_M: c_uint = 0xfU;

pub const NCSIFIFO_S: c_int = 4;

pub const TPFIFO_S: c_int = 0;
pub const TPFIFO_M: c_uint = 0xfU;

pub const MPS_STAT_PERR_INT_CAUSE_SRAM_A: c_uint = 0x9614;
pub const MPS_STAT_PERR_INT_CAUSE_TX_FIFO_A: c_uint = 0x9620;
pub const MPS_STAT_PERR_INT_CAUSE_RX_FIFO_A: c_uint = 0x962c;
pub const MPS_STAT_RX_BG_0_MAC_DROP_FRAME_L: c_uint = 0x9640;
pub const MPS_STAT_RX_BG_0_MAC_DROP_FRAME_H: c_uint = 0x9644;
pub const MPS_STAT_RX_BG_1_MAC_DROP_FRAME_L: c_uint = 0x9648;
pub const MPS_STAT_RX_BG_1_MAC_DROP_FRAME_H: c_uint = 0x964c;
pub const MPS_STAT_RX_BG_2_MAC_DROP_FRAME_L: c_uint = 0x9650;
pub const MPS_STAT_RX_BG_2_MAC_DROP_FRAME_H: c_uint = 0x9654;
pub const MPS_STAT_RX_BG_3_MAC_DROP_FRAME_L: c_uint = 0x9658;
pub const MPS_STAT_RX_BG_3_MAC_DROP_FRAME_H: c_uint = 0x965c;
pub const MPS_STAT_RX_BG_0_LB_DROP_FRAME_L: c_uint = 0x9660;
pub const MPS_STAT_RX_BG_0_LB_DROP_FRAME_H: c_uint = 0x9664;
pub const MPS_STAT_RX_BG_1_LB_DROP_FRAME_L: c_uint = 0x9668;
pub const MPS_STAT_RX_BG_1_LB_DROP_FRAME_H: c_uint = 0x966c;
pub const MPS_STAT_RX_BG_2_LB_DROP_FRAME_L: c_uint = 0x9670;
pub const MPS_STAT_RX_BG_2_LB_DROP_FRAME_H: c_uint = 0x9674;
pub const MPS_STAT_RX_BG_3_LB_DROP_FRAME_L: c_uint = 0x9678;
pub const MPS_STAT_RX_BG_3_LB_DROP_FRAME_H: c_uint = 0x967c;
pub const MPS_STAT_RX_BG_0_MAC_TRUNC_FRAME_L: c_uint = 0x9680;
pub const MPS_STAT_RX_BG_0_MAC_TRUNC_FRAME_H: c_uint = 0x9684;
pub const MPS_STAT_RX_BG_1_MAC_TRUNC_FRAME_L: c_uint = 0x9688;
pub const MPS_STAT_RX_BG_1_MAC_TRUNC_FRAME_H: c_uint = 0x968c;
pub const MPS_STAT_RX_BG_2_MAC_TRUNC_FRAME_L: c_uint = 0x9690;
pub const MPS_STAT_RX_BG_2_MAC_TRUNC_FRAME_H: c_uint = 0x9694;
pub const MPS_STAT_RX_BG_3_MAC_TRUNC_FRAME_L: c_uint = 0x9698;
pub const MPS_STAT_RX_BG_3_MAC_TRUNC_FRAME_H: c_uint = 0x969c;
pub const MPS_STAT_RX_BG_0_LB_TRUNC_FRAME_L: c_uint = 0x96a0;
pub const MPS_STAT_RX_BG_0_LB_TRUNC_FRAME_H: c_uint = 0x96a4;
pub const MPS_STAT_RX_BG_1_LB_TRUNC_FRAME_L: c_uint = 0x96a8;
pub const MPS_STAT_RX_BG_1_LB_TRUNC_FRAME_H: c_uint = 0x96ac;
pub const MPS_STAT_RX_BG_2_LB_TRUNC_FRAME_L: c_uint = 0x96b0;
pub const MPS_STAT_RX_BG_2_LB_TRUNC_FRAME_H: c_uint = 0x96b4;
pub const MPS_STAT_RX_BG_3_LB_TRUNC_FRAME_L: c_uint = 0x96b8;
pub const MPS_STAT_RX_BG_3_LB_TRUNC_FRAME_H: c_uint = 0x96bc;
pub const MPS_TRC_CFG_A: c_uint = 0x9800;
pub const TRCFIFOEMPTY_S: c_int = 4;

pub const TRCIGNOREDROPINPUT_S: c_int = 3;

pub const TRCKEEPDUPLICATES_S: c_int = 2;

pub const TRCEN_S: c_int = 1;

pub const TRCMULTIFILTER_S: c_int = 0;

pub const MPS_TRC_RSS_CONTROL_A: c_uint = 0x9808;
pub const MPS_TRC_FILTER1_RSS_CONTROL_A: c_uint = 0x9ff4;
pub const MPS_TRC_FILTER2_RSS_CONTROL_A: c_uint = 0x9ffc;
pub const MPS_TRC_FILTER3_RSS_CONTROL_A: c_uint = 0xa004;
pub const MPS_T5_TRC_RSS_CONTROL_A: c_uint = 0xa00c;
pub const RSSCONTROL_S: c_int = 16;

pub const QUEUENUMBER_S: c_int = 0;

pub const TFINVERTMATCH_S: c_int = 24;

pub const TFEN_S: c_int = 22;

pub const TFPORT_S: c_int = 18;
pub const TFPORT_M: c_uint = 0xfU;

pub const TFLENGTH_S: c_int = 8;
pub const TFLENGTH_M: c_uint = 0x1fU;

pub const TFOFFSET_S: c_int = 0;
pub const TFOFFSET_M: c_uint = 0x1fU;

pub const T5_TFINVERTMATCH_S: c_int = 25;

pub const T5_TFEN_S: c_int = 23;

pub const T5_TFPORT_S: c_int = 18;
pub const T5_TFPORT_M: c_uint = 0x1fU;

pub const MPS_TRC_FILTER_MATCH_CTL_A_A: c_uint = 0x9810;
pub const MPS_TRC_FILTER_MATCH_CTL_B_A: c_uint = 0x9820;
pub const TFMINPKTSIZE_S: c_int = 16;
pub const TFMINPKTSIZE_M: c_uint = 0x1ffU;

pub const TFCAPTUREMAX_S: c_int = 0;
pub const TFCAPTUREMAX_M: c_uint = 0x3fffU;

pub const MPS_TRC_FILTER0_MATCH_A: c_uint = 0x9c00;
pub const MPS_TRC_FILTER0_DONT_CARE_A: c_uint = 0x9c80;
pub const MPS_TRC_FILTER1_MATCH_A: c_uint = 0x9d00;
pub const TP_RSS_CONFIG_A: c_uint = 0x7df0;
pub const TNL4TUPENIPV6_S: c_int = 31;

pub const TNL2TUPENIPV6_S: c_int = 30;

pub const TNL4TUPENIPV4_S: c_int = 29;

pub const TNL2TUPENIPV4_S: c_int = 28;

pub const TNLTCPSEL_S: c_int = 27;

pub const TNLIP6SEL_S: c_int = 26;

pub const TNLVRTSEL_S: c_int = 25;

pub const TNLMAPEN_S: c_int = 24;

pub const OFDHASHSAVE_S: c_int = 19;

pub const OFDVRTSEL_S: c_int = 18;

pub const OFDMAPEN_S: c_int = 17;

pub const OFDLKPEN_S: c_int = 16;

pub const SYN4TUPENIPV6_S: c_int = 15;

pub const SYN2TUPENIPV6_S: c_int = 14;

pub const SYN4TUPENIPV4_S: c_int = 13;

pub const SYN2TUPENIPV4_S: c_int = 12;

pub const SYNIP6SEL_S: c_int = 11;

pub const SYNVRTSEL_S: c_int = 10;

pub const SYNMAPEN_S: c_int = 9;

pub const SYNLKPEN_S: c_int = 8;

pub const CHANNELENABLE_S: c_int = 7;

pub const PORTENABLE_S: c_int = 6;

pub const TNLALLLOOKUP_S: c_int = 5;

pub const VIRTENABLE_S: c_int = 4;

pub const CONGESTIONENABLE_S: c_int = 3;

pub const HASHTOEPLITZ_S: c_int = 2;

pub const UDPENABLE_S: c_int = 1;

pub const DISABLE_S: c_int = 0;

pub const TP_RSS_CONFIG_TNL_A: c_uint = 0x7df4;
pub const MASKSIZE_S: c_int = 28;
pub const MASKSIZE_M: c_uint = 0xfU;

pub const MASKFILTER_S: c_int = 16;
pub const MASKFILTER_M: c_uint = 0x7ffU;

pub const USEWIRECH_S: c_int = 0;

pub const HASHALL_S: c_int = 2;

pub const HASHETH_S: c_int = 1;

pub const TP_RSS_CONFIG_OFD_A: c_uint = 0x7df8;
pub const RRCPLMAPEN_S: c_int = 20;

pub const RRCPLQUEWIDTH_S: c_int = 16;
pub const RRCPLQUEWIDTH_M: c_uint = 0xfU;

pub const TP_RSS_CONFIG_SYN_A: c_uint = 0x7dfc;
pub const TP_RSS_CONFIG_VRT_A: c_uint = 0x7e00;
pub const VFRDRG_S: c_int = 25;

pub const VFRDEN_S: c_int = 24;

pub const VFPERREN_S: c_int = 23;

pub const KEYPERREN_S: c_int = 22;

pub const DISABLEVLAN_S: c_int = 21;

pub const ENABLEUP0_S: c_int = 20;

pub const HASHDELAY_S: c_int = 16;
pub const HASHDELAY_M: c_uint = 0xfU;

pub const VFWRADDR_S: c_int = 8;
pub const VFWRADDR_M: c_uint = 0x7fU;

pub const KEYMODE_S: c_int = 6;
pub const KEYMODE_M: c_uint = 0x3U;

pub const VFWREN_S: c_int = 5;

pub const KEYWREN_S: c_int = 4;

pub const KEYWRADDR_S: c_int = 0;
pub const KEYWRADDR_M: c_uint = 0xfU;

pub const KEYWRADDRX_S: c_int = 30;
pub const KEYWRADDRX_M: c_uint = 0x3U;

pub const KEYEXTEND_S: c_int = 26;

pub const LKPIDXSIZE_S: c_int = 24;
pub const LKPIDXSIZE_M: c_uint = 0x3U;

pub const TP_RSS_VFL_CONFIG_A: c_uint = 0x3a;
pub const TP_RSS_VFH_CONFIG_A: c_uint = 0x3b;
pub const ENABLEUDPHASH_S: c_int = 31;

pub const VFUPEN_S: c_int = 30;

pub const VFVLNEX_S: c_int = 28;

pub const VFPRTEN_S: c_int = 27;

pub const VFCHNEN_S: c_int = 26;

pub const DEFAULTQUEUE_S: c_int = 16;
pub const DEFAULTQUEUE_M: c_uint = 0x3ffU;

pub const VFIP6TWOTUPEN_S: c_int = 6;

pub const VFIP4FOURTUPEN_S: c_int = 5;

pub const VFIP4TWOTUPEN_S: c_int = 4;

pub const KEYINDEX_S: c_int = 0;
pub const KEYINDEX_M: c_uint = 0xfU;

pub const MAPENABLE_S: c_int = 31;

pub const CHNENABLE_S: c_int = 30;

pub const LE_DB_DBGI_CONFIG_A: c_uint = 0x19cf0;
pub const DBGICMDBUSY_S: c_int = 3;

pub const DBGICMDSTRT_S: c_int = 2;

pub const DBGICMDMODE_S: c_int = 0;
pub const DBGICMDMODE_M: c_uint = 0x3U;

pub const LE_DB_DBGI_REQ_TCAM_CMD_A: c_uint = 0x19cf4;
pub const DBGICMD_S: c_int = 20;
pub const DBGICMD_M: c_uint = 0xfU;

pub const DBGITID_S: c_int = 0;
pub const DBGITID_M: c_uint = 0xfffffU;

pub const LE_DB_DBGI_REQ_DATA_A: c_uint = 0x19d00;
pub const LE_DB_DBGI_RSP_STATUS_A: c_uint = 0x19d94;
pub const LE_DB_DBGI_RSP_DATA_A: c_uint = 0x19da0;
pub const PRTENABLE_S: c_int = 29;

pub const UDPFOURTUPEN_S: c_int = 28;

pub const IP6FOURTUPEN_S: c_int = 27;

pub const IP6TWOTUPEN_S: c_int = 26;

pub const IP4FOURTUPEN_S: c_int = 25;

pub const IP4TWOTUPEN_S: c_int = 24;

pub const IVFWIDTH_S: c_int = 20;
pub const IVFWIDTH_M: c_uint = 0xfU;

pub const CH1DEFAULTQUEUE_S: c_int = 10;
pub const CH1DEFAULTQUEUE_M: c_uint = 0x3ffU;

pub const CH0DEFAULTQUEUE_S: c_int = 0;
pub const CH0DEFAULTQUEUE_M: c_uint = 0x3ffU;

pub const VFLKPIDX_S: c_int = 8;
pub const VFLKPIDX_M: c_uint = 0xffU;

pub const T6_VFWRADDR_S: c_int = 8;
pub const T6_VFWRADDR_M: c_uint = 0xffU;

pub const TP_RSS_CONFIG_CNG_A: c_uint = 0x7e04;
pub const TP_RSS_SECRET_KEY0_A: c_uint = 0x40;
pub const TP_RSS_PF0_CONFIG_A: c_uint = 0x30;
pub const TP_RSS_PF_MAP_A: c_uint = 0x38;
pub const TP_RSS_PF_MSK_A: c_uint = 0x39;
pub const PF1LKPIDX_S: c_int = 3;
pub const PF0LKPIDX_M: c_uint = 0x7U;
pub const PF1MSKSIZE_S: c_int = 4;
pub const PF1MSKSIZE_M: c_uint = 0xfU;
pub const CHNCOUNT3_S: c_int = 31;

pub const CHNCOUNT2_S: c_int = 30;

pub const CHNCOUNT1_S: c_int = 29;

pub const CHNCOUNT0_S: c_int = 28;

pub const CHNUNDFLOW3_S: c_int = 27;

pub const CHNUNDFLOW2_S: c_int = 26;

pub const CHNUNDFLOW1_S: c_int = 25;

pub const CHNUNDFLOW0_S: c_int = 24;

pub const RSTCHN3_S: c_int = 19;

pub const RSTCHN2_S: c_int = 18;

pub const RSTCHN1_S: c_int = 17;

pub const RSTCHN0_S: c_int = 16;

pub const UPDVLD_S: c_int = 15;

pub const XOFF_S: c_int = 14;

pub const UPDCHN3_S: c_int = 13;

pub const UPDCHN2_S: c_int = 12;

pub const UPDCHN1_S: c_int = 11;

pub const UPDCHN0_S: c_int = 10;

pub const QUEUE_S: c_int = 0;
pub const QUEUE_M: c_uint = 0x3ffU;

pub const MPS_TRC_INT_CAUSE_A: c_uint = 0x985c;
pub const MISCPERR_S: c_int = 8;

pub const PKTFIFO_S: c_int = 4;
pub const PKTFIFO_M: c_uint = 0xfU;

pub const FILTMEM_S: c_int = 0;
pub const FILTMEM_M: c_uint = 0xfU;

pub const MPS_CLS_INT_CAUSE_A: c_uint = 0xd028;
pub const HASHSRAM_S: c_int = 2;

pub const MATCHTCAM_S: c_int = 1;

pub const MATCHSRAM_S: c_int = 0;

pub const MPS_RX_PG_RSV0_A: c_uint = 0x11010;
pub const MPS_RX_PG_RSV4_A: c_uint = 0x11020;
pub const MPS_RX_PERR_INT_CAUSE_A: c_uint = 0x11074;
pub const MPS_RX_MAC_BG_PG_CNT0_A: c_uint = 0x11208;
pub const MPS_RX_LPBK_BG_PG_CNT0_A: c_uint = 0x11218;
pub const MPS_RX_VXLAN_TYPE_A: c_uint = 0x11234;
pub const VXLAN_EN_S: c_int = 16;

pub const VXLAN_S: c_int = 0;
pub const VXLAN_M: c_uint = 0xffffU;

pub const MPS_RX_GENEVE_TYPE_A: c_uint = 0x11238;
pub const GENEVE_EN_S: c_int = 16;

pub const GENEVE_S: c_int = 0;
pub const GENEVE_M: c_uint = 0xffffU;

pub const MPS_CLS_TCAM_Y_L_A: c_uint = 0xf000;
pub const MPS_CLS_TCAM_DATA0_A: c_uint = 0xf000;
pub const MPS_CLS_TCAM_DATA1_A: c_uint = 0xf004;
pub const CTLREQID_S: c_int = 30;

pub const MPS_VF_RPLCT_MAP0_A: c_uint = 0x1111c;
pub const MPS_VF_RPLCT_MAP1_A: c_uint = 0x11120;
pub const MPS_VF_RPLCT_MAP2_A: c_uint = 0x11124;
pub const MPS_VF_RPLCT_MAP3_A: c_uint = 0x11128;
pub const MPS_VF_RPLCT_MAP4_A: c_uint = 0x11300;
pub const MPS_VF_RPLCT_MAP5_A: c_uint = 0x11304;
pub const MPS_VF_RPLCT_MAP6_A: c_uint = 0x11308;
pub const MPS_VF_RPLCT_MAP7_A: c_uint = 0x1130c;
pub const VIDL_S: c_int = 16;
pub const VIDL_M: c_uint = 0xffffU;

pub const DATALKPTYPE_S: c_int = 10;
pub const DATALKPTYPE_M: c_uint = 0x3U;

pub const DATAPORTNUM_S: c_int = 12;
pub const DATAPORTNUM_M: c_uint = 0xfU;

pub const DATALKPTYPE_S: c_int = 10;
pub const DATALKPTYPE_M: c_uint = 0x3U;

pub const DATADIPHIT_S: c_int = 8;

pub const DATAVIDH2_S: c_int = 7;

pub const DATAVIDH1_S: c_int = 0;
pub const DATAVIDH1_M: c_uint = 0x7fU;

pub const MPS_CLS_TCAM_RDATA0_REQ_ID1_A: c_uint = 0xf020;
pub const MPS_CLS_TCAM_RDATA1_REQ_ID1_A: c_uint = 0xf024;
pub const MPS_CLS_TCAM_RDATA2_REQ_ID1_A: c_uint = 0xf028;
pub const USED_S: c_int = 16;
pub const USED_M: c_uint = 0x7ffU;

pub const ALLOC_S: c_int = 0;
pub const ALLOC_M: c_uint = 0x7ffU;

pub const T5_USED_S: c_int = 16;
pub const T5_USED_M: c_uint = 0xfffU;

pub const T5_ALLOC_S: c_int = 0;
pub const T5_ALLOC_M: c_uint = 0xfffU;

pub const DMACH_S: c_int = 0;
pub const DMACH_M: c_uint = 0xffffU;

pub const MPS_CLS_TCAM_X_L_A: c_uint = 0xf008;
pub const MPS_CLS_TCAM_DATA2_CTL_A: c_uint = 0xf008;
pub const CTLCMDTYPE_S: c_int = 31;

pub const CTLTCAMSEL_S: c_int = 25;

pub const CTLTCAMINDEX_S: c_int = 17;

pub const CTLXYBITSEL_S: c_int = 16;

pub const NUM_MPS_CLS_TCAM_Y_L_INSTANCES: c_int = 512;

pub const NUM_MPS_CLS_TCAM_X_L_INSTANCES: c_int = 512;
pub const MPS_CLS_SRAM_L_A: c_uint = 0xe000;
pub const T6_MULTILISTEN0_S: c_int = 26;
pub const T6_SRAM_PRIO3_S: c_int = 23;
pub const T6_SRAM_PRIO3_M: c_uint = 0x7U;

pub const T6_SRAM_PRIO2_S: c_int = 20;
pub const T6_SRAM_PRIO2_M: c_uint = 0x7U;

pub const T6_SRAM_PRIO1_S: c_int = 17;
pub const T6_SRAM_PRIO1_M: c_uint = 0x7U;

pub const T6_SRAM_PRIO0_S: c_int = 14;
pub const T6_SRAM_PRIO0_M: c_uint = 0x7U;

pub const T6_SRAM_VLD_S: c_int = 13;

pub const T6_REPLICATE_S: c_int = 12;

pub const T6_PF_S: c_int = 9;
pub const T6_PF_M: c_uint = 0x7U;

pub const T6_VF_VALID_S: c_int = 8;

pub const T6_VF_S: c_int = 0;
pub const T6_VF_M: c_uint = 0xffU;

pub const MPS_CLS_SRAM_H_A: c_uint = 0xe004;

pub const NUM_MPS_CLS_SRAM_L_INSTANCES: c_int = 336;

pub const NUM_MPS_CLS_SRAM_H_INSTANCES: c_int = 336;
pub const MULTILISTEN0_S: c_int = 25;
pub const REPLICATE_S: c_int = 11;

pub const PF_S: c_int = 8;
pub const PF_M: c_uint = 0x7U;

pub const VF_VALID_S: c_int = 7;

pub const VF_S: c_int = 0;
pub const VF_M: c_uint = 0x7fU;

pub const SRAM_PRIO3_S: c_int = 22;
pub const SRAM_PRIO3_M: c_uint = 0x7U;

pub const SRAM_PRIO2_S: c_int = 19;
pub const SRAM_PRIO2_M: c_uint = 0x7U;

pub const SRAM_PRIO1_S: c_int = 16;
pub const SRAM_PRIO1_M: c_uint = 0x7U;

pub const SRAM_PRIO0_S: c_int = 13;
pub const SRAM_PRIO0_M: c_uint = 0x7U;

pub const SRAM_VLD_S: c_int = 12;

pub const PORTMAP_S: c_int = 0;
pub const PORTMAP_M: c_uint = 0xfU;

pub const CPL_INTR_CAUSE_A: c_uint = 0x19054;
pub const CIM_OP_MAP_PERR_S: c_int = 5;

pub const CIM_OVFL_ERROR_S: c_int = 4;

pub const TP_FRAMING_ERROR_S: c_int = 3;

pub const SGE_FRAMING_ERROR_S: c_int = 2;

pub const CIM_FRAMING_ERROR_S: c_int = 1;

pub const ZERO_SWITCH_ERROR_S: c_int = 0;

pub const SMB_INT_CAUSE_A: c_uint = 0x19090;
pub const MSTTXFIFOPARINT_S: c_int = 21;

pub const MSTRXFIFOPARINT_S: c_int = 20;

pub const SLVFIFOPARINT_S: c_int = 19;

pub const ULP_RX_INT_CAUSE_A: c_uint = 0x19158;
pub const ULP_RX_ISCSI_LLIMIT_A: c_uint = 0x1915c;
pub const ULP_RX_ISCSI_ULIMIT_A: c_uint = 0x19160;
pub const ULP_RX_ISCSI_TAGMASK_A: c_uint = 0x19164;
pub const ULP_RX_ISCSI_PSZ_A: c_uint = 0x19168;
pub const ULP_RX_TDDP_LLIMIT_A: c_uint = 0x1916c;
pub const ULP_RX_TDDP_ULIMIT_A: c_uint = 0x19170;
pub const ULP_RX_STAG_LLIMIT_A: c_uint = 0x1917c;
pub const ULP_RX_STAG_ULIMIT_A: c_uint = 0x19180;
pub const ULP_RX_RQ_LLIMIT_A: c_uint = 0x19184;
pub const ULP_RX_RQ_ULIMIT_A: c_uint = 0x19188;
pub const ULP_RX_PBL_LLIMIT_A: c_uint = 0x1918c;
pub const ULP_RX_PBL_ULIMIT_A: c_uint = 0x19190;
pub const ULP_RX_CTX_BASE_A: c_uint = 0x19194;
pub const ULP_RX_RQUDP_LLIMIT_A: c_uint = 0x191a4;
pub const ULP_RX_RQUDP_ULIMIT_A: c_uint = 0x191a8;
pub const ULP_RX_LA_CTL_A: c_uint = 0x1923c;
pub const ULP_RX_LA_RDPTR_A: c_uint = 0x19240;
pub const ULP_RX_LA_RDDATA_A: c_uint = 0x19244;
pub const ULP_RX_LA_WRPTR_A: c_uint = 0x19248;
pub const ULP_RX_TLS_KEY_LLIMIT_A: c_uint = 0x192ac;
pub const ULP_RX_TLS_KEY_ULIMIT_A: c_uint = 0x192b0;
pub const HPZ3_S: c_int = 24;

pub const HPZ2_S: c_int = 16;

pub const HPZ1_S: c_int = 8;

pub const HPZ0_S: c_int = 0;

pub const ULP_RX_TDDP_PSZ_A: c_uint = 0x19178;
// registers for module SF
pub const SF_DATA_A: c_uint = 0x193f8;
pub const SF_OP_A: c_uint = 0x193fc;
pub const SF_BUSY_S: c_int = 31;

pub const SF_LOCK_S: c_int = 4;

pub const SF_CONT_S: c_int = 3;

pub const BYTECNT_S: c_int = 1;

pub const OP_S: c_int = 0;

pub const PL_PF_INT_CAUSE_A: c_uint = 0x3c0;
pub const PFSW_S: c_int = 3;

pub const PFCIM_S: c_int = 1;

pub const PL_PF_INT_ENABLE_A: c_uint = 0x3c4;
pub const PL_PF_CTL_A: c_uint = 0x3c8;
pub const PL_WHOAMI_A: c_uint = 0x19400;
pub const SOURCEPF_S: c_int = 8;
pub const SOURCEPF_M: c_uint = 0x7U;

pub const T6_SOURCEPF_S: c_int = 9;
pub const T6_SOURCEPF_M: c_uint = 0x7U;

pub const PL_INT_CAUSE_A: c_uint = 0x1940c;
pub const ULP_TX_S: c_int = 27;

pub const SGE_S: c_int = 26;

pub const CPL_SWITCH_S: c_int = 24;

pub const ULP_RX_S: c_int = 23;

pub const PM_RX_S: c_int = 22;

pub const PM_TX_S: c_int = 21;

pub const MA_S: c_int = 20;

pub const TP_S: c_int = 19;

pub const LE_S: c_int = 18;

pub const EDC1_S: c_int = 17;

pub const EDC0_S: c_int = 16;

pub const MC_S: c_int = 15;

pub const PCIE_S: c_int = 14;

pub const XGMAC_KR1_S: c_int = 12;

pub const XGMAC_KR0_S: c_int = 11;

pub const XGMAC1_S: c_int = 10;

pub const XGMAC0_S: c_int = 9;

pub const SMB_S: c_int = 8;

pub const SF_S: c_int = 7;

pub const PL_S: c_int = 6;

pub const NCSI_S: c_int = 5;

pub const MPS_S: c_int = 4;

pub const CIM_S: c_int = 0;

pub const MC1_S: c_int = 31;

pub const PL_INT_ENABLE_A: c_uint = 0x19410;
pub const PL_INT_MAP0_A: c_uint = 0x19414;
pub const PL_RST_A: c_uint = 0x19428;
pub const PIORST_S: c_int = 1;

pub const PIORSTMODE_S: c_int = 0;

pub const PL_PL_INT_CAUSE_A: c_uint = 0x19430;
pub const FATALPERR_S: c_int = 4;

pub const PERRVFID_S: c_int = 0;

pub const PL_REV_A: c_uint = 0x1943c;
pub const REV_S: c_int = 0;
pub const REV_M: c_uint = 0xfU;

pub const HASHTBLMEMCRCERR_S: c_int = 27;

pub const CMDTIDERR_S: c_int = 22;

pub const T6_UNKNOWNCMD_S: c_int = 3;

pub const T6_LIP0_S: c_int = 2;

pub const T6_LIPMISS_S: c_int = 1;

pub const LE_DB_CONFIG_A: c_uint = 0x19c04;
pub const LE_DB_ROUTING_TABLE_INDEX_A: c_uint = 0x19c10;
pub const LE_DB_ACTIVE_TABLE_START_INDEX_A: c_uint = 0x19c10;
pub const LE_DB_FILTER_TABLE_INDEX_A: c_uint = 0x19c14;
pub const LE_DB_SERVER_INDEX_A: c_uint = 0x19c18;
pub const LE_DB_SRVR_START_INDEX_A: c_uint = 0x19c18;
pub const LE_DB_CLIP_TABLE_INDEX_A: c_uint = 0x19c1c;
pub const LE_DB_ACT_CNT_IPV4_A: c_uint = 0x19c20;
pub const LE_DB_ACT_CNT_IPV6_A: c_uint = 0x19c24;
pub const LE_DB_HASH_CONFIG_A: c_uint = 0x19c28;
pub const HASHTIDSIZE_S: c_int = 16;
pub const HASHTIDSIZE_M: c_uint = 0x3fU;

pub const HASHTBLSIZE_S: c_int = 3;
pub const HASHTBLSIZE_M: c_uint = 0x1ffffU;

pub const LE_DB_HASH_TID_BASE_A: c_uint = 0x19c30;
pub const LE_DB_HASH_TBL_BASE_ADDR_A: c_uint = 0x19c30;
pub const LE_DB_INT_CAUSE_A: c_uint = 0x19c3c;
pub const LE_DB_CLCAM_TID_BASE_A: c_uint = 0x19df4;
pub const LE_DB_TID_HASHBASE_A: c_uint = 0x19df8;
pub const T6_LE_DB_HASH_TID_BASE_A: c_uint = 0x19df8;
pub const HASHEN_S: c_int = 20;

pub const ASLIPCOMPEN_S: c_int = 17;

pub const REQQPARERR_S: c_int = 16;

pub const UNKNOWNCMD_S: c_int = 15;

pub const PARITYERR_S: c_int = 6;

pub const LIPMISS_S: c_int = 5;

pub const LIP0_S: c_int = 4;

pub const BASEADDR_S: c_int = 3;
pub const BASEADDR_M: c_uint = 0x1fffffffU;

pub const TCAMINTPERR_S: c_int = 13;

pub const SSRAMINTPERR_S: c_int = 10;

pub const LE_DB_RSP_CODE_0_A: c_uint = 0x19c74;
pub const TCAM_ACTV_HIT_S: c_int = 0;
pub const TCAM_ACTV_HIT_M: c_uint = 0x1fU;

pub const LE_DB_RSP_CODE_1_A: c_uint = 0x19c78;
pub const HASH_ACTV_HIT_S: c_int = 25;
pub const HASH_ACTV_HIT_M: c_uint = 0x1fU;

pub const LE_3_DB_HASH_MASK_GEN_IPV4_T6_A: c_uint = 0x19eac;
pub const LE_4_DB_HASH_MASK_GEN_IPV4_T6_A: c_uint = 0x19eb0;
pub const NCSI_INT_CAUSE_A: c_uint = 0x1a0d8;
pub const CIM_DM_PRTY_ERR_S: c_int = 8;

pub const MPS_DM_PRTY_ERR_S: c_int = 7;

pub const TXFIFO_PRTY_ERR_S: c_int = 1;

pub const RXFIFO_PRTY_ERR_S: c_int = 0;

pub const XGMAC_PORT_CFG2_A: c_uint = 0x1018;
pub const PATEN_S: c_int = 18;

pub const MAGICEN_S: c_int = 17;

pub const XGMAC_PORT_MAGIC_MACID_LO: c_uint = 0x1024;
pub const XGMAC_PORT_MAGIC_MACID_HI: c_uint = 0x1028;
pub const XGMAC_PORT_EPIO_DATA0_A: c_uint = 0x10c0;
pub const XGMAC_PORT_EPIO_DATA1_A: c_uint = 0x10c4;
pub const XGMAC_PORT_EPIO_DATA2_A: c_uint = 0x10c8;
pub const XGMAC_PORT_EPIO_DATA3_A: c_uint = 0x10cc;
pub const XGMAC_PORT_EPIO_OP_A: c_uint = 0x10d0;
pub const EPIOWR_S: c_int = 8;

pub const ADDRESS_S: c_int = 0;

pub const MAC_PORT_INT_CAUSE_A: c_uint = 0x8dc;
pub const XGMAC_PORT_INT_CAUSE_A: c_uint = 0x10dc;
pub const TP_TX_MOD_QUEUE_REQ_MAP_A: c_uint = 0x7e28;
pub const TP_TX_MOD_QUEUE_WEIGHT0_A: c_uint = 0x7e30;
pub const TP_TX_MOD_CHANNEL_WEIGHT_A: c_uint = 0x7e34;
pub const TX_MOD_QUEUE_REQ_MAP_S: c_int = 0;

pub const TX_MODQ_WEIGHT3_S: c_int = 24;

pub const TX_MODQ_WEIGHT2_S: c_int = 16;

pub const TX_MODQ_WEIGHT1_S: c_int = 8;

pub const TX_MODQ_WEIGHT0_S: c_int = 0;

pub const TP_TX_SCHED_HDR_A: c_uint = 0x23;
pub const TP_TX_SCHED_FIFO_A: c_uint = 0x24;
pub const TP_TX_SCHED_PCMD_A: c_uint = 0x25;
pub const NUM_MPS_CLS_SRAM_L_INSTANCES: c_int = 336;
pub const NUM_MPS_T5_CLS_SRAM_L_INSTANCES: c_int = 512;
pub const T5_PORT0_BASE: c_uint = 0x30000;
pub const T5_PORT_STRIDE: c_uint = 0x4000;

pub const MC_0_BASE_ADDR: c_uint = 0x40000;
pub const MC_1_BASE_ADDR: c_uint = 0x48000;

pub const MC_P_BIST_CMD_A: c_uint = 0x41400;
pub const MC_P_BIST_CMD_ADDR_A: c_uint = 0x41404;
pub const MC_P_BIST_CMD_LEN_A: c_uint = 0x41408;
pub const MC_P_BIST_DATA_PATTERN_A: c_uint = 0x4140c;
pub const MC_P_BIST_STATUS_RDATA_A: c_uint = 0x41488;
pub const EDC_T50_BASE_ADDR: c_uint = 0x50000;
pub const EDC_H_BIST_CMD_A: c_uint = 0x50004;
pub const EDC_H_BIST_CMD_ADDR_A: c_uint = 0x50008;
pub const EDC_H_BIST_CMD_LEN_A: c_uint = 0x5000c;
pub const EDC_H_BIST_DATA_PATTERN_A: c_uint = 0x50010;
pub const EDC_H_BIST_STATUS_RDATA_A: c_uint = 0x50028;
pub const EDC_H_ECC_ERR_ADDR_A: c_uint = 0x50084;
pub const EDC_T51_BASE_ADDR: c_uint = 0x50800;

pub const PL_VF_REV_A: c_uint = 0x4;
pub const PL_VF_WHOAMI_A: c_uint = 0x0;
pub const PL_VF_REVISION_A: c_uint = 0x8;
// registers for module CIM
pub const CIM_HOST_ACC_CTRL_A: c_uint = 0x7b50;
pub const CIM_HOST_ACC_DATA_A: c_uint = 0x7b54;
pub const UP_UP_DBG_LA_CFG_A: c_uint = 0x140;
pub const UP_UP_DBG_LA_DATA_A: c_uint = 0x144;
pub const HOSTBUSY_S: c_int = 17;

pub const HOSTWRITE_S: c_int = 16;

pub const CIM_IBQ_DBG_CFG_A: c_uint = 0x7b60;
pub const IBQDBGADDR_S: c_int = 16;
pub const IBQDBGADDR_M: c_uint = 0xfffU;

pub const IBQDBGBUSY_S: c_int = 1;

pub const IBQDBGEN_S: c_int = 0;

pub const CIM_OBQ_DBG_CFG_A: c_uint = 0x7b64;
pub const OBQDBGADDR_S: c_int = 16;
pub const OBQDBGADDR_M: c_uint = 0xfffU;

pub const OBQDBGBUSY_S: c_int = 1;

pub const OBQDBGEN_S: c_int = 0;

pub const CIM_IBQ_DBG_DATA_A: c_uint = 0x7b68;
pub const CIM_OBQ_DBG_DATA_A: c_uint = 0x7b6c;
pub const CIM_DEBUGCFG_A: c_uint = 0x7b70;
pub const CIM_DEBUGSTS_A: c_uint = 0x7b74;
pub const POLADBGRDPTR_S: c_int = 23;
pub const POLADBGRDPTR_M: c_uint = 0x1ffU;

pub const POLADBGWRPTR_S: c_int = 16;
pub const POLADBGWRPTR_M: c_uint = 0x1ffU;

pub const PILADBGRDPTR_S: c_int = 14;
pub const PILADBGRDPTR_M: c_uint = 0x1ffU;

pub const PILADBGWRPTR_S: c_int = 0;
pub const PILADBGWRPTR_M: c_uint = 0x1ffU;

pub const LADBGEN_S: c_int = 12;

pub const CIM_PO_LA_DEBUGDATA_A: c_uint = 0x7b78;
pub const CIM_PI_LA_DEBUGDATA_A: c_uint = 0x7b7c;
pub const CIM_PO_LA_MADEBUGDATA_A: c_uint = 0x7b80;
pub const CIM_PI_LA_MADEBUGDATA_A: c_uint = 0x7b84;
pub const UPDBGLARDEN_S: c_int = 1;

pub const UPDBGLAEN_S: c_int = 0;

pub const UPDBGLARDPTR_S: c_int = 2;
pub const UPDBGLARDPTR_M: c_uint = 0xfffU;

pub const UPDBGLAWRPTR_S: c_int = 16;
pub const UPDBGLAWRPTR_M: c_uint = 0xfffU;

pub const UPDBGLACAPTPCONLY_S: c_int = 30;

pub const CIM_QUEUE_CONFIG_REF_A: c_uint = 0x7b48;
pub const CIM_QUEUE_CONFIG_CTRL_A: c_uint = 0x7b4c;
pub const CIMQSIZE_S: c_int = 24;
pub const CIMQSIZE_M: c_uint = 0x3fU;

pub const CIMQBASE_S: c_int = 16;
pub const CIMQBASE_M: c_uint = 0x3fU;

pub const QUEFULLTHRSH_S: c_int = 0;
pub const QUEFULLTHRSH_M: c_uint = 0x1ffU;

pub const UP_IBQ_0_RDADDR_A: c_uint = 0x10;
pub const UP_IBQ_0_SHADOW_RDADDR_A: c_uint = 0x280;
pub const UP_OBQ_0_REALADDR_A: c_uint = 0x104;
pub const UP_OBQ_0_SHADOW_REALADDR_A: c_uint = 0x394;
pub const IBQRDADDR_S: c_int = 0;
pub const IBQRDADDR_M: c_uint = 0x1fffU;

pub const IBQWRADDR_S: c_int = 0;
pub const IBQWRADDR_M: c_uint = 0x1fffU;

pub const QUERDADDR_S: c_int = 0;
pub const QUERDADDR_M: c_uint = 0x7fffU;

pub const QUEREMFLITS_S: c_int = 0;
pub const QUEREMFLITS_M: c_uint = 0x7ffU;

pub const QUEEOPCNT_S: c_int = 16;
pub const QUEEOPCNT_M: c_uint = 0xfffU;

pub const QUESOPCNT_S: c_int = 0;
pub const QUESOPCNT_M: c_uint = 0xfffU;

pub const OBQSELECT_S: c_int = 4;

pub const IBQSELECT_S: c_int = 3;

pub const QUENUMSELECT_S: c_int = 0;

