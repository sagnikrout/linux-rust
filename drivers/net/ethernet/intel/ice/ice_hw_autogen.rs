//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_hw_autogen.h
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
// Copyright (c) 2018-2023, Intel Corporation.
// Machine-generated file

pub const GLCOMM_QUANTA_PROF_MAX_INDEX: c_int = 15;
pub const GLCOMM_QUANTA_PROF_QUANTA_SIZE_S: c_int = 0;

pub const GLCOMM_QUANTA_PROF_MAX_CMD_S: c_int = 16;

pub const GLCOMM_QUANTA_PROF_MAX_DESC_S: c_int = 24;

pub const QTX_COMM_HEAD_MAX_INDEX: c_int = 16383;
pub const QTX_COMM_HEAD_HEAD_S: c_int = 0;

pub const PF_FW_ARQBAH: c_uint = 0x00080180;
pub const PF_FW_ARQBAL: c_uint = 0x00080080;
pub const PF_FW_ARQH: c_uint = 0x00080380;

pub const PF_FW_ARQLEN: c_uint = 0x00080280;

pub const PF_FW_ARQT: c_uint = 0x00080480;
pub const PF_FW_ATQBAH: c_uint = 0x00080100;
pub const PF_FW_ATQBAL: c_uint = 0x00080000;
pub const PF_FW_ATQH: c_uint = 0x00080300;

pub const PF_FW_ATQLEN: c_uint = 0x00080200;

pub const PF_FW_ATQT: c_uint = 0x00080400;
pub const PF_MBX_ARQBAH: c_uint = 0x0022E400;
pub const PF_MBX_ARQBAL: c_uint = 0x0022E380;
pub const PF_MBX_ARQH: c_uint = 0x0022E500;

pub const PF_MBX_ARQLEN: c_uint = 0x0022E480;

pub const PF_MBX_ARQT: c_uint = 0x0022E580;
pub const PF_MBX_ATQBAH: c_uint = 0x0022E180;
pub const PF_MBX_ATQBAL: c_uint = 0x0022E100;
pub const PF_MBX_ATQH: c_uint = 0x0022E280;

pub const PF_MBX_ATQLEN: c_uint = 0x0022E200;

pub const PF_MBX_ATQT: c_uint = 0x0022E300;
pub const PF_SB_ARQBAH: c_uint = 0x0022FF00;
pub const PF_SB_ARQBAH_ARQBAH_S: c_int = 0;

pub const PF_SB_ARQBAL: c_uint = 0x0022FE80;
pub const PF_SB_ARQBAL_ARQBAL_LSB_S: c_int = 0;

pub const PF_SB_ARQBAL_ARQBAL_S: c_int = 6;

pub const PF_SB_ARQH: c_uint = 0x00230000;
pub const PF_SB_ARQH_ARQH_S: c_int = 0;

pub const PF_SB_ARQLEN: c_uint = 0x0022FF80;
pub const PF_SB_ARQLEN_ARQLEN_S: c_int = 0;

pub const PF_SB_ARQLEN_ARQVFE_S: c_int = 28;

pub const PF_SB_ARQLEN_ARQOVFL_S: c_int = 29;

pub const PF_SB_ARQLEN_ARQCRIT_S: c_int = 30;

pub const PF_SB_ARQLEN_ARQENABLE_S: c_int = 31;

pub const PF_SB_ARQT: c_uint = 0x00230080;
pub const PF_SB_ARQT_ARQT_S: c_int = 0;

pub const PF_SB_ATQBAH: c_uint = 0x0022FC80;
pub const PF_SB_ATQBAH_ATQBAH_S: c_int = 0;

pub const PF_SB_ATQBAL: c_uint = 0x0022FC00;
pub const PF_SB_ATQBAL_ATQBAL_S: c_int = 6;

pub const PF_SB_ATQH: c_uint = 0x0022FD80;
pub const PF_SB_ATQH_ATQH_S: c_int = 0;

pub const PF_SB_ATQLEN: c_uint = 0x0022FD00;
pub const PF_SB_ATQLEN_ATQLEN_S: c_int = 0;

pub const PF_SB_ATQLEN_ATQVFE_S: c_int = 28;

pub const PF_SB_ATQLEN_ATQOVFL_S: c_int = 29;

pub const PF_SB_ATQLEN_ATQCRIT_S: c_int = 30;

pub const PF_SB_ATQLEN_ATQENABLE_S: c_int = 31;

pub const PF_SB_ATQT: c_uint = 0x0022FE00;
pub const PF_SB_ATQT_ATQT_S: c_int = 0;

pub const PF_SB_REM_DEV_CTL: c_uint = 0x002300F0;
pub const PRTDCB_GENC: c_uint = 0x00083000;
pub const PRTDCB_GENC_PFCLDA_S: c_int = 16;

pub const PRTDCB_GENS: c_uint = 0x00083020;
pub const PRTDCB_GENS_DCBX_STATUS_S: c_int = 0;

pub const PRTDCB_TUP2TC: c_uint = 0x001D26C0;

pub const GLFLXP_RXDID_FLAGS_FLEXIFLAG_4N_S: c_int = 0;

pub const GLFLXP_RXDID_FLX_WRD_0_PROT_MDID_S: c_int = 0;

pub const GLFLXP_RXDID_FLX_WRD_0_RXDID_OPCODE_S: c_int = 30;

pub const GLFLXP_RXDID_FLX_WRD_1_PROT_MDID_S: c_int = 0;

pub const GLFLXP_RXDID_FLX_WRD_1_RXDID_OPCODE_S: c_int = 30;

pub const GLFLXP_RXDID_FLX_WRD_2_PROT_MDID_S: c_int = 0;

pub const GLFLXP_RXDID_FLX_WRD_2_RXDID_OPCODE_S: c_int = 30;

pub const GLFLXP_RXDID_FLX_WRD_3_PROT_MDID_S: c_int = 0;

pub const GLFLXP_RXDID_FLX_WRD_3_RXDID_OPCODE_S: c_int = 30;

pub const QRXFLXP_CNTXT_RXDID_IDX_S: c_int = 0;

pub const QRXFLXP_CNTXT_RXDID_PRIO_S: c_int = 8;

pub const GLGEN_CLKSTAT_SRC_PSM_CLK_SRC_S: c_int = 4;

pub const GLGEN_CLKSTAT_SRC: c_uint = 0x000B826C;

pub const GLGEN_GPIO_CTL_PIN_FUNC_S: c_int = 8;

pub const GLGEN_RSTAT: c_uint = 0x000B8188;

pub const GLGEN_RSTCTL: c_uint = 0x000B8180;
pub const GLGEN_RSTCTL_GRSTDEL_S: c_int = 0;

pub const GLGEN_RSTAT_RESET_TYPE_S: c_int = 2;

pub const GLGEN_RTRIG: c_uint = 0x000B8190;

pub const GLGEN_STAT: c_uint = 0x000B612C;
pub const GLGEN_SWITCH_MODE_CONFIG: c_uint = 0x000B81E0;

pub const PFGEN_CTRL: c_uint = 0x00091000;

pub const PFGEN_STATE: c_uint = 0x00088000;
pub const PRTGEN_STATUS: c_uint = 0x000B8100;

pub const GLINT_CTL: c_uint = 0x0016CC54;

pub const GLINT_CTL_ITR_GRAN_200_S: c_int = 16;

pub const GLINT_CTL_ITR_GRAN_100_S: c_int = 20;

pub const GLINT_CTL_ITR_GRAN_50_S: c_int = 24;

pub const GLINT_CTL_ITR_GRAN_25_S: c_int = 28;

pub const GLGEN_MAC_LINK_TOPO: c_uint = 0x000B81DC;

pub const GLINT_DYN_CTL_ITR_INDX_S: c_int = 3;

pub const GLINT_DYN_CTL_INTERVAL_S: c_int = 5;

pub const GLINT_DYN_CTL_SW_ITR_INDX_S: c_int = 25;

pub const GLINT_VECT2FUNC_VF_NUM_S: c_int = 0;

pub const GLINT_VECT2FUNC_PF_NUM_S: c_int = 12;

pub const GLINT_VECT2FUNC_IS_PF_S: c_int = 16;

pub const PFINT_ALLOC: c_uint = 0x001D2600;

pub const PFINT_FW_CTL: c_uint = 0x0016C800;

pub const PFINT_FW_CTL_ITR_INDX_S: c_int = 11;

pub const PFINT_MBX_CTL: c_uint = 0x0016B280;

pub const PFINT_MBX_CTL_ITR_INDX_S: c_int = 11;

pub const PFINT_OICR: c_uint = 0x0016CA00;

pub const PFINT_OICR_CTL: c_uint = 0x0016CA80;

pub const PFINT_OICR_CTL_ITR_INDX_S: c_int = 11;

pub const PFINT_OICR_ENA: c_uint = 0x0016C900;
pub const PFINT_SB_CTL: c_uint = 0x0016B600;

pub const PFINT_TSYN_MSK: c_uint = 0x0016C980;

pub const QINT_RQCTL_MSIX_INDX_S: c_int = 0;

pub const QINT_RQCTL_ITR_INDX_S: c_int = 11;

pub const QINT_TQCTL_MSIX_INDX_S: c_int = 0;

pub const QINT_TQCTL_ITR_INDX_S: c_int = 11;

pub const VPINT_ALLOC_FIRST_S: c_int = 0;

pub const VPINT_ALLOC_LAST_S: c_int = 12;

pub const VPINT_ALLOC_PCI_FIRST_S: c_int = 0;

pub const VPINT_ALLOC_PCI_LAST_S: c_int = 12;

pub const GLLAN_RCTL_0: c_uint = 0x002941F8;

pub const QRX_CTRL_MAX_INDEX: c_int = 2047;
pub const QRX_CTRL_QENA_REQ_S: c_int = 0;

pub const QRX_CTRL_QENA_STAT_S: c_int = 2;

pub const QRX_TAIL_MAX_INDEX: c_int = 2047;
pub const QRX_TAIL_TAIL_S: c_int = 0;

pub const VPLAN_RX_QBASE_VFFIRSTQ_S: c_int = 0;

pub const VPLAN_RX_QBASE_VFNUMQ_S: c_int = 16;

pub const VPLAN_TX_QBASE_VFFIRSTQ_S: c_int = 0;

pub const VPLAN_TX_QBASE_VFNUMQ_S: c_int = 16;

pub const E800_PRTMAC_HSEC_CTL_TX_PS_QNT_MAX: c_int = 8;

pub const GL_MDCK_TX_TDPU: c_uint = 0x00049348;

pub const GL_MDET_RX: c_uint = 0x00294C00;
pub const GL_MDET_RX_QNUM_S: c_int = 0;

pub const GL_MDET_RX_VF_NUM_S: c_int = 15;

pub const GL_MDET_RX_PF_NUM_S: c_int = 23;

pub const GL_MDET_RX_MAL_TYPE_S: c_int = 26;

pub const GL_MDET_TX_PQM: c_uint = 0x002D2E00;
pub const GL_MDET_TX_PQM_PF_NUM_S: c_int = 0;

pub const GL_MDET_TX_PQM_VF_NUM_S: c_int = 4;

pub const GL_MDET_TX_PQM_QNUM_S: c_int = 12;

pub const GL_MDET_TX_PQM_MAL_TYPE_S: c_int = 26;

pub const E800_GL_MDET_TX_TCLAN: c_uint = 0x000FC068;
pub const E830_GL_MDET_TX_TCLAN: c_uint = 0x000FCCC0;
pub const GL_MDET_TX_TCLAN_QNUM_S: c_int = 0;

pub const GL_MDET_TX_TCLAN_VF_NUM_S: c_int = 15;

pub const GL_MDET_TX_TCLAN_PF_NUM_S: c_int = 23;

pub const GL_MDET_TX_TCLAN_MAL_TYPE_S: c_int = 26;

pub const PF_MDET_RX: c_uint = 0x00294280;

pub const PF_MDET_TX_PQM: c_uint = 0x002D2C80;

pub const E800_PF_MDET_TX_TCLAN: c_uint = 0x000FC000;
pub const E830_PF_MDET_TX_TCLAN: c_uint = 0x000FCC00;

pub const GL_MNG_FWSM: c_uint = 0x000B6134;

pub const GLNVM_FLA: c_uint = 0x000B6108;

pub const GLNVM_GENS: c_uint = 0x000B6100;
pub const GLNVM_GENS_SR_SIZE_S: c_int = 5;

pub const GLNVM_ULD: c_uint = 0x000B6008;

pub const GLCOMM_QTX_CNTX_CTL: c_uint = 0x002D2DC8;

pub const GLCOMM_QTX_CNTX_CTL_CMD_READ: c_int = 0;
pub const GLCOMM_QTX_CNTX_CTL_CMD_WRITE: c_int = 1;
pub const GLCOMM_QTX_CNTX_CTL_CMD_RESET: c_int = 3;
pub const GLCOMM_QTX_CNTX_CTL_CMD_WRITE_NO_DYN: c_int = 4;

pub const GLPCI_CNF2: c_uint = 0x000BE004;

pub const PF_FUNC_RID: c_uint = 0x0009E880;
pub const PF_FUNC_RID_FUNC_NUM_S: c_int = 0;

pub const PF_PCI_CIAA: c_uint = 0x0009E580;
pub const PF_PCI_CIAA_VF_NUM_S: c_int = 12;
pub const PF_PCI_CIAD: c_uint = 0x0009E500;
pub const GL_PWR_MODE_CTL: c_uint = 0x000B820C;
pub const GL_PWR_MODE_CTL_CAR_MAX_BW_S: c_int = 30;

pub const GLQF_FD_CNT: c_uint = 0x00460018;

pub const GLQF_FD_CNT_FD_BCNT_S: c_int = 16;

pub const GLQF_FD_SIZE: c_uint = 0x00460010;
pub const GLQF_FD_SIZE_FD_GSIZE_S: c_int = 0;

pub const GLQF_FD_SIZE_FD_BSIZE_S: c_int = 16;

pub const GLQF_FDMASK_MAX_INDEX: c_int = 31;
pub const GLQF_FDMASK_MSK_INDEX_S: c_int = 0;

pub const GLQF_FDMASK_MASK_S: c_int = 16;

pub const GLQF_HMASK_MAX_INDEX: c_int = 31;
pub const GLQF_HMASK_MSK_INDEX_S: c_int = 0;

pub const GLQF_HMASK_MASK_S: c_int = 16;

pub const GLQF_HMASK_SEL_MAX_INDEX: c_int = 127;
pub const GLQF_HMASK_SEL_MASK_SEL_S: c_int = 0;

pub const GLQF_HSYMM_REG_SIZE: c_int = 4;
pub const GLQF_HSYMM_REG_PER_PROF: c_int = 6;

pub const PFQF_FD_ENA: c_uint = 0x0043A000;

pub const PFQF_FD_SIZE: c_uint = 0x00460100;
pub const GLDCB_RTCTQ_RXQNUM_S: c_int = 0;

pub const PRTRPB_RDPC: c_uint = 0x000AC260;
pub const GLHH_ART_CTL: c_uint = 0x000A41D4;

pub const GLHH_ART_TIME_H: c_uint = 0x000A41D8;
pub const GLHH_ART_TIME_L: c_uint = 0x000A41DC;

pub const GLTSYN_CMD: c_uint = 0x00088810;
pub const GLTSYN_CMD_SYNC: c_uint = 0x00088814;

pub const GLTSYN_SYNC_DLAY: c_uint = 0x00088818;

pub const PFHH_SEM: c_uint = 0x000A4200 /* Reset Source: PFR */;

pub const PFTSYN_SEM: c_uint = 0x00088880;

pub const VSIQF_FD_CNT_FD_GCNT_S: c_int = 0;

pub const VSIQF_FD_CNT_FD_BCNT_S: c_int = 16;

pub const VSIQF_HKEY_MAX_INDEX: c_int = 12;
pub const PFPM_APM: c_uint = 0x000B8080;

pub const PFPM_WUFC: c_uint = 0x0009DC00;

pub const PFPM_WUS: c_uint = 0x0009DB80;

pub const E830_PRTMAC_TS_TX_MEM_VALID_H: c_uint = 0x001E2020;
pub const E830_PRTMAC_TS_TX_MEM_VALID_L: c_uint = 0x001E2000;
pub const E830_PRTMAC_CL01_PS_QNT: c_uint = 0x001E32A0;

pub const E830_PRTMAC_CL01_QNT_THR: c_uint = 0x001E3320;

pub const E830_GLPTM_ART_CTL: c_uint = 0x00088B50;

pub const E830_GLPTM_ART_TIME_H: c_uint = 0x00088B54;
pub const E830_GLPTM_ART_TIME_L: c_uint = 0x00088B58;

pub const E830_PFPTM_SEM: c_uint = 0x00088B00;

pub const E830_MBX_PF_IN_FLIGHT_VF_MSGS_THRESH: c_uint = 0x00234000;

