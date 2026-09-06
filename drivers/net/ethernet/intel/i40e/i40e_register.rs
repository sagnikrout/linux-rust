//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_register.h
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
// Copyright(c) 2013 - 2021 Intel Corporation.
// I40E_MASK is a macro used on 32 bit registers

pub const I40E_GL_ATQLEN_ATQCRIT_SHIFT: c_int = 30;

pub const I40E_PF_ARQBAH: c_uint = 0x00080180 /* Reset: EMPR */;
pub const I40E_PF_ARQBAL: c_uint = 0x00080080 /* Reset: EMPR */;
pub const I40E_PF_ARQH: c_uint = 0x00080380 /* Reset: EMPR */;
pub const I40E_PF_ARQH_ARQH_SHIFT: c_int = 0;

pub const I40E_PF_ARQLEN: c_uint = 0x00080280 /* Reset: EMPR */;
pub const I40E_PF_ARQLEN_ARQVFE_SHIFT: c_int = 28;

pub const I40E_PF_ARQLEN_ARQOVFL_SHIFT: c_int = 29;

pub const I40E_PF_ARQLEN_ARQCRIT_SHIFT: c_int = 30;

pub const I40E_PF_ARQLEN_ARQENABLE_SHIFT: c_int = 31;

pub const I40E_PF_ARQT: c_uint = 0x00080480 /* Reset: EMPR */;
pub const I40E_PF_ATQBAH: c_uint = 0x00080100 /* Reset: EMPR */;
pub const I40E_PF_ATQBAL: c_uint = 0x00080000 /* Reset: EMPR */;
pub const I40E_PF_ATQH: c_uint = 0x00080300 /* Reset: EMPR */;
pub const I40E_PF_ATQLEN: c_uint = 0x00080200 /* Reset: EMPR */;
pub const I40E_PF_ATQLEN_ATQVFE_SHIFT: c_int = 28;

pub const I40E_PF_ATQLEN_ATQOVFL_SHIFT: c_int = 29;

pub const I40E_PF_ATQLEN_ATQCRIT_SHIFT: c_int = 30;

pub const I40E_PF_ATQLEN_ATQENABLE_SHIFT: c_int = 31;

pub const I40E_PF_ATQT: c_uint = 0x00080400 /* Reset: EMPR */;
pub const I40E_PRT_SWR_PM_THR: c_uint = 0x0026CD00 /* Reset: CORER */;
pub const I40E_PRT_SWR_PM_THR_THRESHOLD_SHIFT: c_int = 0;

pub const I40E_PRTDCB_FCCFG: c_uint = 0x001E4640 /* Reset: GLOBR */;
pub const I40E_PRTDCB_FCCFG_TFCE_SHIFT: c_int = 3;

pub const I40E_PRTDCB_GENC: c_uint = 0x00083000 /* Reset: CORER */;
pub const I40E_PRTDCB_GENC_NUMTC_SHIFT: c_int = 2;

pub const I40E_PRTDCB_GENC_PFCLDA_SHIFT: c_int = 16;

pub const I40E_PRTDCB_GENS: c_uint = 0x00083020 /* Reset: CORER */;
pub const I40E_PRTDCB_GENS_DCBX_STATUS_SHIFT: c_int = 0;

pub const I40E_PRTDCB_MFLCN: c_uint = 0x001E2400 /* Reset: GLOBR */;
pub const I40E_PRTDCB_MFLCN_PMCF_SHIFT: c_int = 0;

pub const I40E_PRTDCB_MFLCN_DPF_SHIFT: c_int = 1;

pub const I40E_PRTDCB_MFLCN_RPFCM_SHIFT: c_int = 2;

pub const I40E_PRTDCB_MFLCN_RFCE_SHIFT: c_int = 3;

pub const I40E_PRTDCB_MFLCN_RPFCE_SHIFT: c_int = 4;

pub const I40E_PRTDCB_RETSC: c_uint = 0x001223E0 /* Reset: CORER */;
pub const I40E_PRTDCB_RETSC_ETS_MODE_SHIFT: c_int = 0;

pub const I40E_PRTDCB_RETSC_NON_ETS_MODE_SHIFT: c_int = 1;

pub const I40E_PRTDCB_RETSC_ETS_MAX_EXP_SHIFT: c_int = 2;

pub const I40E_PRTDCB_RETSC_LLTC_SHIFT: c_int = 8;

pub const I40E_PRTDCB_RETSTCC_MAX_INDEX: c_int = 7;
pub const I40E_PRTDCB_RETSTCC_BWSHARE_SHIFT: c_int = 0;

pub const I40E_PRTDCB_RETSTCC_UPINTC_MODE_SHIFT: c_int = 30;

pub const I40E_PRTDCB_RETSTCC_ETSTC_SHIFT: c_int = 31;

pub const I40E_PRTDCB_RPPMC: c_uint = 0x001223A0 /* Reset: CORER */;
pub const I40E_PRTDCB_RPPMC_LANRPPM_SHIFT: c_int = 0;

pub const I40E_PRTDCB_RPPMC_RDMARPPM_SHIFT: c_int = 8;

pub const I40E_PRTDCB_RPPMC_RX_FIFO_SIZE_SHIFT: c_int = 16;

pub const I40E_PRTDCB_RUP: c_uint = 0x001C0B00 /* Reset: CORER */;
pub const I40E_PRTDCB_RUP_NOVLANUP_SHIFT: c_int = 0;

pub const I40E_PRTDCB_RUP2TC: c_uint = 0x001C09A0 /* Reset: CORER */;
pub const I40E_PRTDCB_RUP2TC_UP0TC_SHIFT: c_int = 0;

pub const I40E_PRTDCB_RUP2TC_UP1TC_SHIFT: c_int = 3;

pub const I40E_PRTDCB_RUP2TC_UP2TC_SHIFT: c_int = 6;

pub const I40E_PRTDCB_RUP2TC_UP3TC_SHIFT: c_int = 9;

pub const I40E_PRTDCB_RUP2TC_UP4TC_SHIFT: c_int = 12;

pub const I40E_PRTDCB_RUP2TC_UP5TC_SHIFT: c_int = 15;

pub const I40E_PRTDCB_RUP2TC_UP6TC_SHIFT: c_int = 18;

pub const I40E_PRTDCB_RUP2TC_UP7TC_SHIFT: c_int = 21;

pub const I40E_PRTDCB_RUPTQ_MAX_INDEX: c_int = 7;
pub const I40E_PRTDCB_RUPTQ_RXQNUM_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TC2PFC: c_uint = 0x001C0980 /* Reset: CORER */;
pub const I40E_PRTDCB_TC2PFC_TC2PFC_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TCMSTC_MAX_INDEX: c_int = 7;
pub const I40E_PRTDCB_TCMSTC_MSTC_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TCPMC: c_uint = 0x000A21A0 /* Reset: CORER */;
pub const I40E_PRTDCB_TCPMC_CPM_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TCPMC_LLTC_SHIFT: c_int = 13;

pub const I40E_PRTDCB_TCPMC_TCPM_MODE_SHIFT: c_int = 30;

pub const I40E_PRTDCB_TCWSTC_MAX_INDEX: c_int = 7;
pub const I40E_PRTDCB_TCWSTC_MSTC_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TDPMC: c_uint = 0x000A0180 /* Reset: CORER */;
pub const I40E_PRTDCB_TDPMC_DPM_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TDPMC_TCPM_MODE_SHIFT: c_int = 30;

pub const I40E_PRTDCB_TETSC_TCB: c_uint = 0x000AE060 /* Reset: CORER */;
pub const I40E_PRTDCB_TETSC_TCB_EN_LL_STRICT_PRIORITY_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TETSC_TCB_LLTC_SHIFT: c_int = 8;

pub const I40E_PRTDCB_TETSC_TPB: c_uint = 0x00098060 /* Reset: CORER */;
pub const I40E_PRTDCB_TETSC_TPB_EN_LL_STRICT_PRIORITY_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TETSC_TPB_LLTC_SHIFT: c_int = 8;

pub const I40E_PRTDCB_TFCS: c_uint = 0x001E4560 /* Reset: GLOBR */;
pub const I40E_PRTDCB_TFCS_TXOFF_SHIFT: c_int = 0;

pub const I40E_PRTDCB_TFCS_TXOFF0_SHIFT: c_int = 8;

pub const I40E_PRTDCB_TFCS_TXOFF1_SHIFT: c_int = 9;

pub const I40E_PRTDCB_TFCS_TXOFF2_SHIFT: c_int = 10;

pub const I40E_PRTDCB_TFCS_TXOFF3_SHIFT: c_int = 11;

pub const I40E_PRTDCB_TFCS_TXOFF4_SHIFT: c_int = 12;

pub const I40E_PRTDCB_TFCS_TXOFF5_SHIFT: c_int = 13;

pub const I40E_PRTDCB_TFCS_TXOFF6_SHIFT: c_int = 14;

pub const I40E_PRTDCB_TFCS_TXOFF7_SHIFT: c_int = 15;

pub const I40E_PRTDCB_TPFCTS_MAX_INDEX: c_int = 7;
pub const I40E_PRTDCB_TPFCTS_PFCTIMER_SHIFT: c_int = 0;

pub const I40E_GL_FWSTS: c_uint = 0x00083048 /* Reset: POR */;
pub const I40E_GL_FWSTS_FWS1B_SHIFT: c_int = 16;

pub const I40E_GLGEN_GPIO_CTL_MAX_INDEX: c_int = 29;
pub const I40E_GLGEN_GPIO_CTL_PRT_NUM_SHIFT: c_int = 0;

pub const I40E_GLGEN_GPIO_CTL_PRT_NUM_NA_SHIFT: c_int = 3;

pub const I40E_GLGEN_GPIO_CTL_PIN_DIR_SHIFT: c_int = 4;
pub const I40E_GLGEN_GPIO_CTL_TRI_CTL_SHIFT: c_int = 5;
pub const I40E_GLGEN_GPIO_CTL_OUT_CTL_SHIFT: c_int = 6;
pub const I40E_GLGEN_GPIO_CTL_PIN_FUNC_SHIFT: c_int = 7;

pub const I40E_GLGEN_GPIO_CTL_LED_BLINK_SHIFT: c_int = 11;
pub const I40E_GLGEN_GPIO_CTL_LED_MODE_SHIFT: c_int = 12;

pub const I40E_GLGEN_GPIO_CTL_OUT_DEFAULT_SHIFT: c_int = 19;

pub const I40E_GLGEN_GPIO_CTL_PHY_PIN_NAME_SHIFT: c_int = 20;
pub const I40E_GLGEN_GPIO_SET: c_uint = 0x00088184 /* Reset: POR */;
pub const I40E_GLGEN_GPIO_SET_SDP_DATA_SHIFT: c_int = 5;
pub const I40E_GLGEN_GPIO_SET_DRIVE_SDP_SHIFT: c_int = 6;

pub const I40E_GLGEN_MSCA_MDIADD_SHIFT: c_int = 0;
pub const I40E_GLGEN_MSCA_DEVADD_SHIFT: c_int = 16;
pub const I40E_GLGEN_MSCA_PHYADD_SHIFT: c_int = 21;
pub const I40E_GLGEN_MSCA_OPCODE_SHIFT: c_int = 26;

pub const I40E_GLGEN_MSCA_STCODE_SHIFT: c_int = 28;

pub const I40E_GLGEN_MSCA_MDICMD_SHIFT: c_int = 30;

pub const I40E_GLGEN_MSCA_MDIINPROGEN_SHIFT: c_int = 31;

pub const I40E_GLGEN_MSRWD_MDIWRDATA_SHIFT: c_int = 0;
pub const I40E_GLGEN_MSRWD_MDIRDDATA_SHIFT: c_int = 16;

pub const I40E_GLGEN_PCIFCNCNT: c_uint = 0x001C0AB4 /* Reset: PCIR */;
pub const I40E_GLGEN_PCIFCNCNT_PCIPFCNT_SHIFT: c_int = 0;

pub const I40E_GLGEN_PCIFCNCNT_PCIVFCNT_SHIFT: c_int = 16;

pub const I40E_GLGEN_RSTAT: c_uint = 0x000B8188 /* Reset: POR */;
pub const I40E_GLGEN_RSTAT_DEVSTATE_SHIFT: c_int = 0;

pub const I40E_GLGEN_RSTAT_RESET_TYPE_SHIFT: c_int = 2;

pub const I40E_GLGEN_RSTCTL: c_uint = 0x000B8180 /* Reset: POR */;
pub const I40E_GLGEN_RSTCTL_GRSTDEL_SHIFT: c_int = 0;

pub const I40E_GLGEN_RTRIG: c_uint = 0x000B8190 /* Reset: CORER */;
pub const I40E_GLGEN_RTRIG_CORER_SHIFT: c_int = 0;

pub const I40E_GLGEN_RTRIG_GLOBR_SHIFT: c_int = 1;

pub const I40E_GLGEN_STAT: c_uint = 0x000B612C /* Reset: POR */;

pub const I40E_GLVFGEN_TIMER: c_uint = 0x000881BC /* Reset: CORER */;
pub const I40E_PFGEN_CTRL: c_uint = 0x00092400 /* Reset: PFR */;
pub const I40E_PFGEN_CTRL_PFSWR_SHIFT: c_int = 0;

pub const I40E_PFGEN_PORTNUM: c_uint = 0x001C0480 /* Reset: CORER */;
pub const I40E_PFGEN_PORTNUM_PORT_NUM_SHIFT: c_int = 0;

pub const I40E_PRTGEN_CNF: c_uint = 0x000B8120 /* Reset: POR */;
pub const I40E_PRTGEN_CNF_PORT_DIS_SHIFT: c_int = 0;

pub const I40E_PRTGEN_STATUS: c_uint = 0x000B8100 /* Reset: POR */;

pub const I40E_VPGEN_VFRSTAT_VFRD_SHIFT: c_int = 0;

pub const I40E_VPGEN_VFRTRIG_VFSWR_SHIFT: c_int = 0;

pub const I40E_GLHMC_FCOEDDPBASE_FPMFCOEDDPBASE_SHIFT: c_int = 0;

pub const I40E_GLHMC_FCOEDDPOBJSZ: c_uint = 0x000C2010 /* Reset: CORER */;

pub const I40E_GLHMC_FCOEFBASE_FPMFCOEFBASE_SHIFT: c_int = 0;

pub const I40E_GLHMC_FCOEFMAX: c_uint = 0x000C20D0 /* Reset: CORER */;
pub const I40E_GLHMC_FCOEFMAX_PMFCOEFMAX_SHIFT: c_int = 0;

pub const I40E_GLHMC_FCOEFOBJSZ: c_uint = 0x000C2018 /* Reset: CORER */;
pub const I40E_GLHMC_FCOEMAX: c_uint = 0x000C2014 /* Reset: CORER */;
pub const I40E_GLHMC_LANQMAX: c_uint = 0x000C2008 /* Reset: CORER */;

pub const I40E_GLHMC_LANRXBASE_FPMLANRXBASE_SHIFT: c_int = 0;

pub const I40E_GLHMC_LANRXOBJSZ: c_uint = 0x000C200c /* Reset: CORER */;

pub const I40E_GLHMC_LANTXBASE_FPMLANTXBASE_SHIFT: c_int = 0;

pub const I40E_GLHMC_LANTXOBJSZ: c_uint = 0x000C2004 /* Reset: CORER */;
pub const I40E_PFHMC_ERRORDATA: c_uint = 0x000C0500 /* Reset: PFR */;
pub const I40E_PFHMC_ERRORINFO: c_uint = 0x000C0400 /* Reset: PFR */;
pub const I40E_PFHMC_PDINV: c_uint = 0x000C0300 /* Reset: PFR */;
pub const I40E_PFHMC_PDINV_PMSDIDX_SHIFT: c_int = 0;
pub const I40E_PFHMC_PDINV_PMPDIDX_SHIFT: c_int = 16;
pub const I40E_PFHMC_SDCMD: c_uint = 0x000C0000 /* Reset: PFR */;
pub const I40E_PFHMC_SDCMD_PMSDWR_SHIFT: c_int = 31;
pub const I40E_PFHMC_SDDATAHIGH: c_uint = 0x000C0200 /* Reset: PFR */;
pub const I40E_PFHMC_SDDATALOW: c_uint = 0x000C0100 /* Reset: PFR */;
pub const I40E_PFHMC_SDDATALOW_PMSDVALID_SHIFT: c_int = 0;
pub const I40E_PFHMC_SDDATALOW_PMSDTYPE_SHIFT: c_int = 1;
pub const I40E_PFHMC_SDDATALOW_PMSDBPCOUNT_SHIFT: c_int = 2;
pub const I40E_PFGEN_PORTMDIO_NUM: c_uint = 0x0003F100 /* Reset: CORER */;
pub const I40E_PFGEN_PORTMDIO_NUM_VFLINK_STAT_ENA_SHIFT: c_int = 4;

pub const I40E_PFINT_AEQCTL: c_uint = 0x00038700 /* Reset: CORER */;
pub const I40E_PFINT_AEQCTL_MSIX_INDX_SHIFT: c_int = 0;
pub const I40E_PFINT_AEQCTL_ITR_INDX_SHIFT: c_int = 11;
pub const I40E_PFINT_AEQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_PFINT_CEQCTL_MSIX_INDX_SHIFT: c_int = 0;
pub const I40E_PFINT_CEQCTL_ITR_INDX_SHIFT: c_int = 11;
pub const I40E_PFINT_CEQCTL_NEXTQ_INDX_SHIFT: c_int = 16;
pub const I40E_PFINT_CEQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_GLINT_CTL: c_uint = 0x0003F800 /* Reset: CORER */;
pub const I40E_GLINT_CTL_DIS_AUTOMASK_VF0_SHIFT: c_int = 1;

pub const I40E_PFINT_DYN_CTL0: c_uint = 0x00038480 /* Reset: PFR */;
pub const I40E_PFINT_DYN_CTL0_INTENA_SHIFT: c_int = 0;

pub const I40E_PFINT_DYN_CTL0_CLEARPBA_SHIFT: c_int = 1;

pub const I40E_PFINT_DYN_CTL0_SWINT_TRIG_SHIFT: c_int = 2;

pub const I40E_PFINT_DYN_CTL0_ITR_INDX_SHIFT: c_int = 3;

pub const I40E_PFINT_DYN_CTL0_SW_ITR_INDX_ENA_SHIFT: c_int = 24;

pub const I40E_PFINT_DYN_CTL0_SW_ITR_INDX_SHIFT: c_int = 25;

pub const I40E_PFINT_DYN_CTL0_INTENA_MSK_SHIFT: c_int = 31;

pub const I40E_PFINT_DYN_CTLN_INTENA_SHIFT: c_int = 0;

pub const I40E_PFINT_DYN_CTLN_CLEARPBA_SHIFT: c_int = 1;

pub const I40E_PFINT_DYN_CTLN_SWINT_TRIG_SHIFT: c_int = 2;

pub const I40E_PFINT_DYN_CTLN_ITR_INDX_SHIFT: c_int = 3;

pub const I40E_PFINT_DYN_CTLN_INTERVAL_SHIFT: c_int = 5;

pub const I40E_PFINT_DYN_CTLN_SW_ITR_INDX_ENA_SHIFT: c_int = 24;

pub const I40E_PFINT_DYN_CTLN_SW_ITR_INDX_SHIFT: c_int = 25;

pub const I40E_PFINT_ICR0: c_uint = 0x00038780 /* Reset: CORER */;
pub const I40E_PFINT_ICR0_INTEVENT_SHIFT: c_int = 0;

pub const I40E_PFINT_ICR0_QUEUE_0_SHIFT: c_int = 1;

pub const I40E_PFINT_ICR0_ECC_ERR_SHIFT: c_int = 16;

pub const I40E_PFINT_ICR0_MAL_DETECT_SHIFT: c_int = 19;

pub const I40E_PFINT_ICR0_GRST_SHIFT: c_int = 20;

pub const I40E_PFINT_ICR0_PCI_EXCEPTION_SHIFT: c_int = 21;

pub const I40E_PFINT_ICR0_TIMESYNC_SHIFT: c_int = 23;

pub const I40E_PFINT_ICR0_HMC_ERR_SHIFT: c_int = 26;

pub const I40E_PFINT_ICR0_PE_CRITERR_SHIFT: c_int = 28;

pub const I40E_PFINT_ICR0_VFLR_SHIFT: c_int = 29;

pub const I40E_PFINT_ICR0_ADMINQ_SHIFT: c_int = 30;

pub const I40E_PFINT_ICR0_SWINT_SHIFT: c_int = 31;

pub const I40E_PFINT_ICR0_ENA: c_uint = 0x00038800 /* Reset: CORER */;
pub const I40E_PFINT_ICR0_ENA_ECC_ERR_SHIFT: c_int = 16;

pub const I40E_PFINT_ICR0_ENA_MAL_DETECT_SHIFT: c_int = 19;

pub const I40E_PFINT_ICR0_ENA_GRST_SHIFT: c_int = 20;

pub const I40E_PFINT_ICR0_ENA_PCI_EXCEPTION_SHIFT: c_int = 21;

pub const I40E_PFINT_ICR0_ENA_GPIO_SHIFT: c_int = 22;

pub const I40E_PFINT_ICR0_ENA_TIMESYNC_SHIFT: c_int = 23;

pub const I40E_PFINT_ICR0_ENA_HMC_ERR_SHIFT: c_int = 26;

pub const I40E_PFINT_ICR0_ENA_PE_CRITERR_SHIFT: c_int = 28;

pub const I40E_PFINT_ICR0_ENA_VFLR_SHIFT: c_int = 29;

pub const I40E_PFINT_ICR0_ENA_ADMINQ_SHIFT: c_int = 30;

pub const I40E_PFINT_LNKLST0: c_uint = 0x00038500 /* Reset: PFR */;
pub const I40E_PFINT_LNKLST0_FIRSTQ_INDX_SHIFT: c_int = 0;

pub const I40E_PFINT_LNKLSTN_FIRSTQ_INDX_SHIFT: c_int = 0;

pub const I40E_PFINT_LNKLSTN_FIRSTQ_TYPE_SHIFT: c_int = 11;

pub const I40E_PFINT_STAT_CTL0: c_uint = 0x00038400 /* Reset: CORER */;

pub const I40E_QINT_RQCTL_MSIX_INDX_SHIFT: c_int = 0;

pub const I40E_QINT_RQCTL_ITR_INDX_SHIFT: c_int = 11;

pub const I40E_QINT_RQCTL_MSIX0_INDX_SHIFT: c_int = 13;

pub const I40E_QINT_RQCTL_NEXTQ_INDX_SHIFT: c_int = 16;

pub const I40E_QINT_RQCTL_NEXTQ_TYPE_SHIFT: c_int = 27;
pub const I40E_QINT_RQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_QINT_RQCTL_INTEVENT_SHIFT: c_int = 31;

pub const I40E_QINT_TQCTL_MSIX_INDX_SHIFT: c_int = 0;

pub const I40E_QINT_TQCTL_ITR_INDX_SHIFT: c_int = 11;

pub const I40E_QINT_TQCTL_MSIX0_INDX_SHIFT: c_int = 13;

pub const I40E_QINT_TQCTL_NEXTQ_INDX_SHIFT: c_int = 16;

pub const I40E_QINT_TQCTL_NEXTQ_TYPE_SHIFT: c_int = 27;
pub const I40E_QINT_TQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_QINT_TQCTL_INTEVENT_SHIFT: c_int = 31;

pub const I40E_VFINT_DYN_CTLN_CLEARPBA_SHIFT: c_int = 1;

pub const I40E_VFINT_ICR0_ADMINQ_SHIFT: c_int = 30;

pub const I40E_VPINT_AEQCTL_MSIX_INDX_SHIFT: c_int = 0;
pub const I40E_VPINT_AEQCTL_ITR_INDX_SHIFT: c_int = 11;
pub const I40E_VPINT_AEQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_VPINT_CEQCTL_MSIX_INDX_SHIFT: c_int = 0;
pub const I40E_VPINT_CEQCTL_ITR_INDX_SHIFT: c_int = 11;
pub const I40E_VPINT_CEQCTL_NEXTQ_INDX_SHIFT: c_int = 16;

pub const I40E_VPINT_CEQCTL_NEXTQ_TYPE_SHIFT: c_int = 27;

pub const I40E_VPINT_CEQCTL_CAUSE_ENA_SHIFT: c_int = 30;

pub const I40E_VPINT_LNKLST0_FIRSTQ_INDX_SHIFT: c_int = 0;

pub const I40E_VPINT_LNKLSTN_FIRSTQ_INDX_SHIFT: c_int = 0;

pub const I40E_VPINT_LNKLSTN_FIRSTQ_TYPE_SHIFT: c_int = 11;

pub const I40E_GLLAN_RCTL_0: c_uint = 0x0012A500 /* Reset: CORER */;
pub const I40E_GLLAN_RCTL_0_PXE_MODE_SHIFT: c_int = 0;

pub const I40E_GLLAN_TSOMSK_F: c_uint = 0x000442D8 /* Reset: CORER */;
pub const I40E_GLLAN_TSOMSK_L: c_uint = 0x000442E0 /* Reset: CORER */;
pub const I40E_GLLAN_TSOMSK_M: c_uint = 0x000442DC /* Reset: CORER */;

pub const I40E_GLLAN_TXPRE_QDIS_QINDX_SHIFT: c_int = 0;

pub const I40E_GLLAN_TXPRE_QDIS_SET_QDIS_SHIFT: c_int = 30;

pub const I40E_GLLAN_TXPRE_QDIS_CLEAR_QDIS_SHIFT: c_int = 31;

pub const I40E_PFLAN_QALLOC: c_uint = 0x001C0400 /* Reset: CORER */;
pub const I40E_PFLAN_QALLOC_FIRSTQ_SHIFT: c_int = 0;

pub const I40E_PFLAN_QALLOC_LASTQ_SHIFT: c_int = 16;

pub const I40E_PFLAN_QALLOC_VALID_SHIFT: c_int = 31;

pub const I40E_QRX_ENA_QENA_REQ_SHIFT: c_int = 0;

pub const I40E_QRX_ENA_QENA_STAT_SHIFT: c_int = 2;

pub const I40E_QTX_CTL_PFVF_Q_SHIFT: c_int = 0;

pub const I40E_QTX_CTL_PF_INDX_SHIFT: c_int = 2;

pub const I40E_QTX_CTL_VFVM_INDX_SHIFT: c_int = 7;

pub const I40E_QTX_ENA_QENA_REQ_SHIFT: c_int = 0;

pub const I40E_QTX_ENA_QENA_STAT_SHIFT: c_int = 2;

pub const I40E_VPLAN_MAPENA_TXRX_ENA_SHIFT: c_int = 0;

pub const I40E_VPLAN_QTABLE_QINDEX_SHIFT: c_int = 0;

pub const I40E_VSILAN_QBASE_VSIQTABLE_ENA_SHIFT: c_int = 11;

pub const I40E_PRTGL_SAH: c_uint = 0x001E2140 /* Reset: GLOBR */;
pub const I40E_PRTGL_SAH_FC_SAH_SHIFT: c_int = 0;

pub const I40E_PRTGL_SAH_MFS_SHIFT: c_int = 16;

pub const I40E_PRTGL_SAL: c_uint = 0x001E2120 /* Reset: GLOBR */;
pub const I40E_PRTGL_SAL_FC_SAL_SHIFT: c_int = 0;

pub const I40E_PRTMAC_HSEC_CTL_RX_ENABLE_GPP: c_uint = 0x001E3260 /* Reset: GLOBR */;
pub const I40E_PRTMAC_HSEC_CTL_RX_ENABLE_GPP_SHIFT: c_int = 0;

pub const I40E_PRTMAC_HSEC_CTL_RX_ENABLE_PPP: c_uint = 0x001E32E0 /* Reset: GLOBR */;
pub const I40E_PRTMAC_HSEC_CTL_RX_ENABLE_PPP_SHIFT: c_int = 0;

pub const I40E_PRTMAC_HSEC_CTL_RX_PAUSE_ENABLE: c_uint = 0x001E30C0 /* Reset: GLOBR */;
pub const I40E_PRTMAC_HSEC_CTL_RX_PAUSE_ENABLE_SHIFT: c_int = 0;

pub const I40E_PRTMAC_HSEC_CTL_TX_PAUSE_ENABLE: c_uint = 0x001E30D0 /* Reset: GLOBR */;
pub const I40E_PRTMAC_HSEC_CTL_TX_PAUSE_ENABLE_SHIFT: c_int = 0;

pub const I40E_PRTMAC_HSEC_CTL_TX_PAUSE_REFRESH_TIMER_MAX_INDEX: c_int = 8;
pub const I40E_PRTMAC_HSEC_CTL_TX_PAUSE_REFRESH_TIMER_SHIFT: c_int = 0;

pub const I40E_GLNVM_FLA: c_uint = 0x000B6108 /* Reset: POR */;
pub const I40E_GLNVM_FLA_LOCKED_SHIFT: c_int = 6;

pub const I40E_GLNVM_GENS: c_uint = 0x000B6100 /* Reset: POR */;
pub const I40E_GLNVM_GENS_SR_SIZE_SHIFT: c_int = 5;

pub const I40E_GLNVM_SRCTL: c_uint = 0x000B6110 /* Reset: POR */;
pub const I40E_GLNVM_SRCTL_ADDR_SHIFT: c_int = 14;
pub const I40E_GLNVM_SRCTL_START_SHIFT: c_int = 30;
pub const I40E_GLNVM_SRCTL_DONE_SHIFT: c_int = 31;

pub const I40E_GLNVM_SRDATA: c_uint = 0x000B6114 /* Reset: POR */;
pub const I40E_GLNVM_SRDATA_RDDATA_SHIFT: c_int = 16;

pub const I40E_GLNVM_ULD: c_uint = 0x000B6008 /* Reset: POR */;
pub const I40E_GLNVM_ULD_CONF_CORE_DONE_SHIFT: c_int = 3;

pub const I40E_GLNVM_ULD_CONF_GLOBAL_DONE_SHIFT: c_int = 4;

pub const I40E_GLPCI_CAPSUP: c_uint = 0x000BE4A8 /* Reset: PCIR */;
pub const I40E_GLPCI_CAPSUP_ARI_EN_SHIFT: c_int = 4;

pub const I40E_GLPCI_CNF2: c_uint = 0x000BE494 /* Reset: PCIR */;
pub const I40E_GLPCI_CNF2_MSI_X_PF_N_SHIFT: c_int = 2;

pub const I40E_GLPCI_CNF2_MSI_X_VF_N_SHIFT: c_int = 13;

pub const I40E_GLPCI_LBARCTRL: c_uint = 0x000BE484 /* Reset: POR */;
pub const I40E_GLPCI_LBARCTRL_FL_SIZE_SHIFT: c_int = 6;

pub const I40E_PF_FUNC_RID: c_uint = 0x0009C000 /* Reset: PCIR */;
pub const I40E_PF_PCI_CIAA: c_uint = 0x0009C080 /* Reset: FLR */;
pub const I40E_PF_PCI_CIAA_VF_NUM_SHIFT: c_int = 12;
pub const I40E_PF_PCI_CIAD: c_uint = 0x0009C100 /* Reset: FLR */;
pub const I40E_PRTPM_EEE_STAT: c_uint = 0x001E4320 /* Reset: GLOBR */;
pub const I40E_PFPCI_SUBSYSID: c_uint = 0x000BE100 /* Reset: PCIR */;
pub const I40E_PRTPM_EEE_STAT_RX_LPI_STATUS_SHIFT: c_int = 30;

pub const I40E_PRTPM_EEE_STAT_TX_LPI_STATUS_SHIFT: c_int = 31;

pub const I40E_PRTPM_EEER_TX_LPI_EN_SHIFT: c_int = 16;

pub const I40E_PRTPM_RLPIC: c_uint = 0x001E43A0 /* Reset: GLOBR */;
pub const I40E_PRTPM_TLPIC: c_uint = 0x001E43C0 /* Reset: GLOBR */;

pub const I40E_PRTRPB_DHW_DHW_TCN_SHIFT: c_int = 0;

pub const I40E_PRTRPB_DLW_DLW_TCN_SHIFT: c_int = 0;

pub const I40E_PRTRPB_DPS_DPS_TCN_SHIFT: c_int = 0;

pub const I40E_PRTRPB_SHT_SHT_TCN_SHIFT: c_int = 0;

pub const I40E_PRTRPB_SHW: c_uint = 0x000AC580 /* Reset: CORER */;
pub const I40E_PRTRPB_SHW_SHW_SHIFT: c_int = 0;

pub const I40E_PRTRPB_SLT_SLT_TCN_SHIFT: c_int = 0;

pub const I40E_PRTRPB_SLW: c_uint = 0x000AC6A0 /* Reset: CORER */;
pub const I40E_PRTRPB_SLW_SLW_SHIFT: c_int = 0;

pub const I40E_PRTRPB_SPS: c_uint = 0x000AC7C0 /* Reset: CORER */;
pub const I40E_PRTRPB_SPS_SPS_SHIFT: c_int = 0;

pub const I40E_GLQF_FDCNT_0: c_uint = 0x00269BAC /* Reset: CORER */;
pub const I40E_GLQF_FDCNT_0_GUARANT_CNT_SHIFT: c_int = 0;

pub const I40E_GLQF_FDCNT_0_BESTCNT_SHIFT: c_int = 13;

pub const I40E_GLQF_HKEY_MAX_INDEX: c_int = 12;

pub const I40E_PFQF_CTL_0: c_uint = 0x001C0AC0 /* Reset: CORER */;
pub const I40E_PFQF_CTL_0_PEHSIZE_SHIFT: c_int = 0;

pub const I40E_PFQF_CTL_0_PEDSIZE_SHIFT: c_int = 5;

pub const I40E_PFQF_CTL_0_PFFCHSIZE_SHIFT: c_int = 10;

pub const I40E_PFQF_CTL_0_PFFCDSIZE_SHIFT: c_int = 14;

pub const I40E_PFQF_CTL_0_HASHLUTSIZE_SHIFT: c_int = 16;

pub const I40E_PFQF_CTL_0_FD_ENA_SHIFT: c_int = 17;

pub const I40E_PFQF_CTL_0_ETYPE_ENA_SHIFT: c_int = 18;

pub const I40E_PFQF_CTL_0_MACVLAN_ENA_SHIFT: c_int = 19;

pub const I40E_PFQF_CTL_1: c_uint = 0x00245D80 /* Reset: CORER */;
pub const I40E_PFQF_CTL_1_CLEARFDTABLE_SHIFT: c_int = 0;

pub const I40E_PFQF_FDSTAT: c_uint = 0x00246380 /* Reset: CORER */;
pub const I40E_PFQF_FDSTAT_GUARANT_CNT_SHIFT: c_int = 0;

pub const I40E_PFQF_FDSTAT_BEST_CNT_SHIFT: c_int = 16;

pub const I40E_PFQF_HKEY_MAX_INDEX: c_int = 12;

pub const I40E_PFQF_HLUT_MAX_INDEX: c_int = 127;

pub const I40E_PRTQF_FD_INSET_MAX_INDEX: c_int = 63;
pub const I40E_PRTQF_FD_INSET_INSET_SHIFT: c_int = 0;

pub const I40E_PRTQF_FD_INSET_MAX_INDEX: c_int = 63;
pub const I40E_PRTQF_FD_INSET_INSET_SHIFT: c_int = 0;

pub const I40E_PRTQF_FLX_PIT_SOURCE_OFF_SHIFT: c_int = 0;

pub const I40E_PRTQF_FLX_PIT_FSIZE_SHIFT: c_int = 5;

pub const I40E_PRTQF_FLX_PIT_DEST_OFF_SHIFT: c_int = 10;

pub const I40E_VFQF_HKEY1_MAX_INDEX: c_int = 12;

pub const I40E_VFQF_HLUT1_MAX_INDEX: c_int = 15;

pub const I40E_GL_RXERR1H_MAX_INDEX: c_int = 143;
pub const I40E_GL_RXERR1H_RXERR1H_SHIFT: c_int = 0;

pub const I40E_GL_RXERR1L_MAX_INDEX: c_int = 143;
pub const I40E_GL_RXERR1L_RXERR1L_SHIFT: c_int = 0;

pub const I40E_PRTTSYN_CTL0: c_uint = 0x001E4200 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_CTL0_TXTIME_INT_ENA_SHIFT: c_int = 1;

pub const I40E_PRTTSYN_CTL0_EVENT_INT_ENA_SHIFT: c_int = 2;

pub const I40E_PRTTSYN_CTL0_PF_ID_SHIFT: c_int = 8;

pub const I40E_PRTTSYN_CTL0_TSYNENA_SHIFT: c_int = 31;

pub const I40E_PRTTSYN_CTL1: c_uint = 0x00085020 /* Reset: CORER */;
pub const I40E_PRTTSYN_CTL1_V1MESSTYPE0_SHIFT: c_int = 0;

pub const I40E_PRTTSYN_CTL1_V2MESSTYPE0_SHIFT: c_int = 16;

pub const I40E_PRTTSYN_CTL1_TSYNTYPE_SHIFT: c_int = 24;

// Timestamp UDP v1 packets

// Timestamp L2 and UDP v2 packets with message type < 8

pub const I40E_PRTTSYN_CTL1_UDP_ENA_SHIFT: c_int = 26;

// Timestamp UDP packets on port 319

pub const I40E_PRTTSYN_CTL1_TSYNENA_SHIFT: c_int = 31;

pub const I40E_PRTTSYN_INC_H: c_uint = 0x001E4060 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_INC_L: c_uint = 0x001E4040 /* Reset: GLOBR */;

pub const I40E_PRTTSYN_RXTIME_L_MAX_INDEX: c_int = 3;
pub const I40E_PRTTSYN_STAT_0: c_uint = 0x001E4220 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_STAT_0_EVENT0_SHIFT: c_int = 0;

pub const I40E_PRTTSYN_STAT_0_TXTIME_SHIFT: c_int = 4;

pub const I40E_PRTTSYN_STAT_1: c_uint = 0x00085140 /* Reset: CORER */;
pub const I40E_PRTTSYN_TIME_H: c_uint = 0x001E4120 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_TIME_L: c_uint = 0x001E4100 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_TXTIME_H: c_uint = 0x001E41E0 /* Reset: GLOBR */;
pub const I40E_PRTTSYN_TXTIME_L: c_uint = 0x001E41C0 /* Reset: GLOBR */;

pub const I40E_PRTTSYN_AUX_0_OUT_ENA_SHIFT: c_int = 0;
pub const I40E_PRTTSYN_AUX_0_OUTMOD_SHIFT: c_int = 1;
pub const I40E_PRTTSYN_AUX_0_EVNTLVL_SHIFT: c_int = 16;
pub const I40E_PRTTSYN_AUX_0_PTPFLAG_SHIFT: c_int = 17;

pub const I40E_PRTTSYN_AUX_1_INSTNT_SHIFT: c_int = 0;

pub const I40E_PRTTSYN_ADJ: c_uint = 0x001E4280 /* Reset: GLOBR */;
pub const I40E_GL_MDET_RX: c_uint = 0x0012A510 /* Reset: CORER */;
pub const I40E_GL_MDET_RX_FUNCTION_SHIFT: c_int = 0;

pub const I40E_GL_MDET_RX_EVENT_SHIFT: c_int = 8;

pub const I40E_GL_MDET_RX_QUEUE_SHIFT: c_int = 17;

pub const I40E_GL_MDET_RX_VALID_SHIFT: c_int = 31;

pub const I40E_GL_MDET_TX: c_uint = 0x000E6480 /* Reset: CORER */;
pub const I40E_GL_MDET_TX_QUEUE_SHIFT: c_int = 0;

pub const I40E_GL_MDET_TX_VF_NUM_SHIFT: c_int = 12;

pub const I40E_GL_MDET_TX_PF_NUM_SHIFT: c_int = 21;

pub const I40E_GL_MDET_TX_EVENT_SHIFT: c_int = 25;

pub const I40E_GL_MDET_TX_VALID_SHIFT: c_int = 31;

pub const I40E_PF_MDET_RX: c_uint = 0x0012A400 /* Reset: CORER */;
pub const I40E_PF_MDET_RX_VALID_SHIFT: c_int = 0;

pub const I40E_PF_MDET_TX: c_uint = 0x000E6400 /* Reset: CORER */;
pub const I40E_PF_MDET_TX_VALID_SHIFT: c_int = 0;

pub const I40E_PF_VT_PFALLOC: c_uint = 0x001C0500 /* Reset: CORER */;
pub const I40E_PF_VT_PFALLOC_FIRSTVF_SHIFT: c_int = 0;

pub const I40E_PF_VT_PFALLOC_LASTVF_SHIFT: c_int = 8;

pub const I40E_PF_VT_PFALLOC_VALID_SHIFT: c_int = 31;

pub const I40E_VP_MDET_RX_VALID_SHIFT: c_int = 0;

pub const I40E_VP_MDET_TX_VALID_SHIFT: c_int = 0;

pub const I40E_PFPM_APM: c_uint = 0x000B8080 /* Reset: POR */;
pub const I40E_PFPM_APM_APME_SHIFT: c_int = 0;

pub const I40E_PFPM_WUFC: c_uint = 0x0006B400 /* Reset: POR */;
pub const I40E_PFPM_WUFC_MAG_SHIFT: c_int = 1;

pub const I40E_VFQF_HLUT_MAX_INDEX: c_int = 15;
pub const I40E_PFINT_DYN_CTL0_WB_ON_ITR_SHIFT: c_int = 30;

pub const I40E_PFINT_DYN_CTLN_WB_ON_ITR_SHIFT: c_int = 30;

pub const I40E_GLNVM_FLA: c_uint = 0x000B6108 /* Reset: POR */;
pub const I40E_GLNVM_FLA_LOCKED_SHIFT: c_int = 6;

pub const I40E_GLNVM_ULD: c_uint = 0x000B6008 /* Reset: POR */;

pub const I40E_GLQF_ORT_PIT_INDX_SHIFT: c_int = 0;

pub const I40E_GLQF_ORT_FIELD_CNT_SHIFT: c_int = 5;

pub const I40E_GLQF_ORT_FLX_PAYLOAD_SHIFT: c_int = 7;

pub const I40E_FDEVICT_PCTYPE_DEFAULT: c_uint = 0xc03;
// Redefined for X722 family
pub const I40E_GLGEN_STAT_CLEAR: c_uint = 0x00390004 /* Reset: CORER */;
