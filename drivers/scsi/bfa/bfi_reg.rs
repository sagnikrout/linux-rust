//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfi_reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//
// bfi_reg.h ASIC register defines for all QLogic BR-series adapter ASICs
//
pub const HOSTFN0_INT_STATUS: c_uint = 0x00014000	/* cb/ct	*/;
pub const HOSTFN1_INT_STATUS: c_uint = 0x00014100	/* cb/ct	*/;
pub const HOSTFN2_INT_STATUS: c_uint = 0x00014300	/* ct		*/;
pub const HOSTFN3_INT_STATUS: c_uint = 0x00014400	/* ct		*/;
pub const HOSTFN0_INT_MSK: c_uint = 0x00014004	/* cb/ct	*/;
pub const HOSTFN1_INT_MSK: c_uint = 0x00014104	/* cb/ct	*/;
pub const HOSTFN2_INT_MSK: c_uint = 0x00014304	/* ct		*/;
pub const HOSTFN3_INT_MSK: c_uint = 0x00014404	/* ct		*/;
pub const HOST_PAGE_NUM_FN0: c_uint = 0x00014008	/* cb/ct	*/;
pub const HOST_PAGE_NUM_FN1: c_uint = 0x00014108	/* cb/ct	*/;
pub const HOST_PAGE_NUM_FN2: c_uint = 0x00014308	/* ct		*/;
pub const HOST_PAGE_NUM_FN3: c_uint = 0x00014408	/* ct		*/;
pub const APP_PLL_LCLK_CTL_REG: c_uint = 0x00014204	/* cb/ct	*/;
pub const __P_LCLK_PLL_LOCK: c_uint = 0x80000000;
pub const __APP_PLL_LCLK_SRAM_USE_100MHZ: c_uint = 0x00100000;
pub const __APP_PLL_LCLK_RESET_TIMER_MK: c_uint = 0x000e0000;
pub const __APP_PLL_LCLK_RESET_TIMER_SH: c_int = 17;

pub const __APP_PLL_LCLK_LOGIC_SOFT_RESET: c_uint = 0x00010000;
pub const __APP_PLL_LCLK_CNTLMT0_1_MK: c_uint = 0x0000c000;
pub const __APP_PLL_LCLK_CNTLMT0_1_SH: c_int = 14;

pub const __APP_PLL_LCLK_JITLMT0_1_MK: c_uint = 0x00003000;
pub const __APP_PLL_LCLK_JITLMT0_1_SH: c_int = 12;

pub const __APP_PLL_LCLK_HREF: c_uint = 0x00000800;
pub const __APP_PLL_LCLK_HDIV: c_uint = 0x00000400;
pub const __APP_PLL_LCLK_P0_1_MK: c_uint = 0x00000300;
pub const __APP_PLL_LCLK_P0_1_SH: c_int = 8;

pub const __APP_PLL_LCLK_Z0_2_MK: c_uint = 0x000000e0;
pub const __APP_PLL_LCLK_Z0_2_SH: c_int = 5;

pub const __APP_PLL_LCLK_RSEL200500: c_uint = 0x00000010;
pub const __APP_PLL_LCLK_ENARST: c_uint = 0x00000008;
pub const __APP_PLL_LCLK_BYPASS: c_uint = 0x00000004;
pub const __APP_PLL_LCLK_LRESETN: c_uint = 0x00000002;
pub const __APP_PLL_LCLK_ENABLE: c_uint = 0x00000001;
pub const APP_PLL_SCLK_CTL_REG: c_uint = 0x00014208	/* cb/ct	*/;
pub const __P_SCLK_PLL_LOCK: c_uint = 0x80000000;
pub const __APP_PLL_SCLK_RESET_TIMER_MK: c_uint = 0x000e0000;
pub const __APP_PLL_SCLK_RESET_TIMER_SH: c_int = 17;

pub const __APP_PLL_SCLK_LOGIC_SOFT_RESET: c_uint = 0x00010000;
pub const __APP_PLL_SCLK_CNTLMT0_1_MK: c_uint = 0x0000c000;
pub const __APP_PLL_SCLK_CNTLMT0_1_SH: c_int = 14;

pub const __APP_PLL_SCLK_JITLMT0_1_MK: c_uint = 0x00003000;
pub const __APP_PLL_SCLK_JITLMT0_1_SH: c_int = 12;

pub const __APP_PLL_SCLK_HREF: c_uint = 0x00000800;
pub const __APP_PLL_SCLK_HDIV: c_uint = 0x00000400;
pub const __APP_PLL_SCLK_P0_1_MK: c_uint = 0x00000300;
pub const __APP_PLL_SCLK_P0_1_SH: c_int = 8;

pub const __APP_PLL_SCLK_Z0_2_MK: c_uint = 0x000000e0;
pub const __APP_PLL_SCLK_Z0_2_SH: c_int = 5;

pub const __APP_PLL_SCLK_RSEL200500: c_uint = 0x00000010;
pub const __APP_PLL_SCLK_ENARST: c_uint = 0x00000008;
pub const __APP_PLL_SCLK_BYPASS: c_uint = 0x00000004;
pub const __APP_PLL_SCLK_LRESETN: c_uint = 0x00000002;
pub const __APP_PLL_SCLK_ENABLE: c_uint = 0x00000001;
pub const __ENABLE_MAC_AHB_1: c_uint = 0x00800000	/* ct		*/;
pub const __ENABLE_MAC_AHB_0: c_uint = 0x00400000	/* ct		*/;
pub const __ENABLE_MAC_1: c_uint = 0x00200000	/* ct		*/;
pub const __ENABLE_MAC_0: c_uint = 0x00100000	/* ct		*/;
pub const HOST_SEM0_REG: c_uint = 0x00014230	/* cb/ct	*/;
pub const HOST_SEM1_REG: c_uint = 0x00014234	/* cb/ct	*/;
pub const HOST_SEM2_REG: c_uint = 0x00014238	/* cb/ct	*/;
pub const HOST_SEM3_REG: c_uint = 0x0001423c	/* cb/ct	*/;
pub const HOST_SEM4_REG: c_uint = 0x00014610	/* cb/ct	*/;
pub const HOST_SEM5_REG: c_uint = 0x00014614	/* cb/ct	*/;
pub const HOST_SEM6_REG: c_uint = 0x00014618	/* cb/ct	*/;
pub const HOST_SEM7_REG: c_uint = 0x0001461c	/* cb/ct	*/;
pub const HOST_SEM0_INFO_REG: c_uint = 0x00014240	/* cb/ct	*/;
pub const HOST_SEM1_INFO_REG: c_uint = 0x00014244	/* cb/ct	*/;
pub const HOST_SEM2_INFO_REG: c_uint = 0x00014248	/* cb/ct	*/;
pub const HOST_SEM3_INFO_REG: c_uint = 0x0001424c	/* cb/ct	*/;
pub const HOST_SEM4_INFO_REG: c_uint = 0x00014620	/* cb/ct	*/;
pub const HOST_SEM5_INFO_REG: c_uint = 0x00014624	/* cb/ct	*/;
pub const HOST_SEM6_INFO_REG: c_uint = 0x00014628	/* cb/ct	*/;
pub const HOST_SEM7_INFO_REG: c_uint = 0x0001462c	/* cb/ct	*/;
pub const HOSTFN0_LPU0_CMD_STAT: c_uint = 0x00019000	/* cb/ct	*/;
pub const HOSTFN0_LPU1_CMD_STAT: c_uint = 0x00019004	/* cb/ct	*/;
pub const HOSTFN1_LPU0_CMD_STAT: c_uint = 0x00019010	/* cb/ct	*/;
pub const HOSTFN1_LPU1_CMD_STAT: c_uint = 0x00019014	/* cb/ct	*/;
pub const HOSTFN2_LPU0_CMD_STAT: c_uint = 0x00019150	/* ct		*/;
pub const HOSTFN2_LPU1_CMD_STAT: c_uint = 0x00019154	/* ct		*/;
pub const HOSTFN3_LPU0_CMD_STAT: c_uint = 0x00019160	/* ct		*/;
pub const HOSTFN3_LPU1_CMD_STAT: c_uint = 0x00019164	/* ct		*/;
pub const LPU0_HOSTFN0_CMD_STAT: c_uint = 0x00019008	/* cb/ct	*/;
pub const LPU1_HOSTFN0_CMD_STAT: c_uint = 0x0001900c	/* cb/ct	*/;
pub const LPU0_HOSTFN1_CMD_STAT: c_uint = 0x00019018	/* cb/ct	*/;
pub const LPU1_HOSTFN1_CMD_STAT: c_uint = 0x0001901c	/* cb/ct	*/;
pub const LPU0_HOSTFN2_CMD_STAT: c_uint = 0x00019158	/* ct		*/;
pub const LPU1_HOSTFN2_CMD_STAT: c_uint = 0x0001915c	/* ct		*/;
pub const LPU0_HOSTFN3_CMD_STAT: c_uint = 0x00019168	/* ct		*/;
pub const LPU1_HOSTFN3_CMD_STAT: c_uint = 0x0001916c	/* ct		*/;
pub const PSS_CTL_REG: c_uint = 0x00018800	/* cb/ct	*/;
pub const __PSS_I2C_CLK_DIV_MK: c_uint = 0x007f0000;
pub const __PSS_I2C_CLK_DIV_SH: c_int = 16;

pub const __PSS_LMEM_INIT_DONE: c_uint = 0x00001000;
pub const __PSS_LMEM_RESET: c_uint = 0x00000200;
pub const __PSS_LMEM_INIT_EN: c_uint = 0x00000100;
pub const __PSS_LPU1_RESET: c_uint = 0x00000002;
pub const __PSS_LPU0_RESET: c_uint = 0x00000001;
pub const PSS_ERR_STATUS_REG: c_uint = 0x00018810	/* cb/ct	*/;
pub const ERR_SET_REG: c_uint = 0x00018818	/* cb/ct	*/;
pub const PSS_GPIO_OUT_REG: c_uint = 0x000188c0	/* cb/ct	*/;
pub const __PSS_GPIO_OUT_REG: c_uint = 0x00000fff;
pub const PSS_GPIO_OE_REG: c_uint = 0x000188c8	/* cb/ct	*/;
pub const __PSS_GPIO_OE_REG: c_uint = 0x000000ff;
pub const HOSTFN0_LPU_MBOX0_0: c_uint = 0x00019200	/* cb/ct	*/;
pub const HOSTFN1_LPU_MBOX0_8: c_uint = 0x00019260	/* cb/ct	*/;
pub const LPU_HOSTFN0_MBOX0_0: c_uint = 0x00019280	/* cb/ct	*/;
pub const LPU_HOSTFN1_MBOX0_8: c_uint = 0x000192e0	/* cb/ct	*/;
pub const HOSTFN2_LPU_MBOX0_0: c_uint = 0x00019400	/* ct		*/;
pub const HOSTFN3_LPU_MBOX0_8: c_uint = 0x00019460	/* ct		*/;
pub const LPU_HOSTFN2_MBOX0_0: c_uint = 0x00019480	/* ct		*/;
pub const LPU_HOSTFN3_MBOX0_8: c_uint = 0x000194e0	/* ct		*/;
pub const HOST_MSIX_ERR_INDEX_FN0: c_uint = 0x0001400c	/* ct		*/;
pub const HOST_MSIX_ERR_INDEX_FN1: c_uint = 0x0001410c	/* ct		*/;
pub const HOST_MSIX_ERR_INDEX_FN2: c_uint = 0x0001430c	/* ct		*/;
pub const HOST_MSIX_ERR_INDEX_FN3: c_uint = 0x0001440c	/* ct		*/;
pub const MBIST_CTL_REG: c_uint = 0x00014220	/* ct		*/;
pub const __EDRAM_BISTR_START: c_uint = 0x00000004;
pub const MBIST_STAT_REG: c_uint = 0x00014224	/* ct		*/;
pub const ETH_MAC_SER_REG: c_uint = 0x00014288	/* ct		*/;
pub const __APP_EMS_CKBUFAMPIN: c_uint = 0x00000020;
pub const __APP_EMS_REFCLKSEL: c_uint = 0x00000010;
pub const __APP_EMS_CMLCKSEL: c_uint = 0x00000008;
pub const __APP_EMS_REFCKBUFEN2: c_uint = 0x00000004;
pub const __APP_EMS_REFCKBUFEN1: c_uint = 0x00000002;
pub const __APP_EMS_CHANNEL_SEL: c_uint = 0x00000001;
pub const FNC_PERS_REG: c_uint = 0x00014604	/* ct		*/;
pub const __F3_FUNCTION_ACTIVE: c_uint = 0x80000000;
pub const __F3_FUNCTION_MODE: c_uint = 0x40000000;
pub const __F3_PORT_MAP_MK: c_uint = 0x30000000;
pub const __F3_PORT_MAP_SH: c_int = 28;

pub const __F3_VM_MODE: c_uint = 0x08000000;
pub const __F3_INTX_STATUS_MK: c_uint = 0x07000000;
pub const __F3_INTX_STATUS_SH: c_int = 24;

pub const __F2_FUNCTION_ACTIVE: c_uint = 0x00800000;
pub const __F2_FUNCTION_MODE: c_uint = 0x00400000;
pub const __F2_PORT_MAP_MK: c_uint = 0x00300000;
pub const __F2_PORT_MAP_SH: c_int = 20;

pub const __F2_VM_MODE: c_uint = 0x00080000;
pub const __F2_INTX_STATUS_MK: c_uint = 0x00070000;
pub const __F2_INTX_STATUS_SH: c_int = 16;

pub const __F1_FUNCTION_ACTIVE: c_uint = 0x00008000;
pub const __F1_FUNCTION_MODE: c_uint = 0x00004000;
pub const __F1_PORT_MAP_MK: c_uint = 0x00003000;
pub const __F1_PORT_MAP_SH: c_int = 12;

pub const __F1_VM_MODE: c_uint = 0x00000800;
pub const __F1_INTX_STATUS_MK: c_uint = 0x00000700;
pub const __F1_INTX_STATUS_SH: c_int = 8;

pub const __F0_FUNCTION_ACTIVE: c_uint = 0x00000080;
pub const __F0_FUNCTION_MODE: c_uint = 0x00000040;
pub const __F0_PORT_MAP_MK: c_uint = 0x00000030;
pub const __F0_PORT_MAP_SH: c_int = 4;

pub const __F0_VM_MODE: c_uint = 0x00000008;
pub const __F0_INTX_STATUS: c_uint = 0x00000007;
pub const OP_MODE: c_uint = 0x0001460c	/* ct		*/;
pub const __APP_ETH_CLK_LOWSPEED: c_uint = 0x00000004;
pub const __GLOBAL_CORECLK_HALFSPEED: c_uint = 0x00000002;
pub const __GLOBAL_FCOE_MODE: c_uint = 0x00000001;
pub const FW_INIT_HALT_P0: c_uint = 0x000191ac	/* ct		*/;
pub const __FW_INIT_HALT_P: c_uint = 0x00000001;
pub const FW_INIT_HALT_P1: c_uint = 0x000191bc	/* ct		*/;
pub const PMM_1T_RESET_REG_P0: c_uint = 0x0002381c	/* ct		*/;
pub const __PMM_1T_RESET_P: c_uint = 0x00000001;
pub const PMM_1T_RESET_REG_P1: c_uint = 0x00023c1c	/* ct		*/;
//
// Catapult-2 specific defines
//
pub const CT2_PCI_CPQ_BASE: c_uint = 0x00030000;
pub const CT2_PCI_APP_BASE: c_uint = 0x00030100;
pub const CT2_PCI_ETH_BASE: c_uint = 0x00030400;
//
// APP block registers
//

pub const __PME_STATUS_: c_uint = 0x00200000;
pub const __PF_VF_BAR_SIZE_MODE__MK: c_uint = 0x00180000;
pub const __PF_VF_BAR_SIZE_MODE__SH: c_int = 19;

pub const __FC_LL_PORT_MAP__MK: c_uint = 0x00060000;
pub const __FC_LL_PORT_MAP__SH: c_int = 17;

pub const __PF_VF_ACTIVE_: c_uint = 0x00010000;
pub const __PF_VF_CFG_RDY_: c_uint = 0x00008000;
pub const __PF_VF_ENABLE_: c_uint = 0x00004000;
pub const __PF_DRIVER_ACTIVE_: c_uint = 0x00002000;
pub const __PF_PME_SEND_ENABLE_: c_uint = 0x00001000;
pub const __PF_EXROM_OFFSET__MK: c_uint = 0x00000ff0;
pub const __PF_EXROM_OFFSET__SH: c_int = 4;

pub const __FC_LL_MODE_: c_uint = 0x00000008;
pub const __PF_INTX_PIN_: c_uint = 0x00000007;

pub const __PF_NUM_QUEUES1__MK: c_uint = 0xff000000;
pub const __PF_NUM_QUEUES1__SH: c_int = 24;

pub const __PF_VF_QUE_OFFSET1__MK: c_uint = 0x00ff0000;
pub const __PF_VF_QUE_OFFSET1__SH: c_int = 16;

pub const __PF_VF_NUM_QUEUES__MK: c_uint = 0x0000ff00;
pub const __PF_VF_NUM_QUEUES__SH: c_int = 8;

pub const __PF_VF_QUE_OFFSET_: c_uint = 0x000000ff;

//
// Catapult-2 CPQ block registers
//

pub const CT2_HOST_SEM0_REG: c_uint = 0x000148f0;
pub const CT2_HOST_SEM1_REG: c_uint = 0x000148f4;
pub const CT2_HOST_SEM2_REG: c_uint = 0x000148f8;
pub const CT2_HOST_SEM3_REG: c_uint = 0x000148fc;
pub const CT2_HOST_SEM4_REG: c_uint = 0x00014900;
pub const CT2_HOST_SEM5_REG: c_uint = 0x00014904;
pub const CT2_HOST_SEM6_REG: c_uint = 0x00014908;
pub const CT2_HOST_SEM7_REG: c_uint = 0x0001490c;
pub const CT2_HOST_SEM0_INFO_REG: c_uint = 0x000148b0;
pub const CT2_HOST_SEM1_INFO_REG: c_uint = 0x000148b4;
pub const CT2_HOST_SEM2_INFO_REG: c_uint = 0x000148b8;
pub const CT2_HOST_SEM3_INFO_REG: c_uint = 0x000148bc;
pub const CT2_HOST_SEM4_INFO_REG: c_uint = 0x000148c0;
pub const CT2_HOST_SEM5_INFO_REG: c_uint = 0x000148c4;
pub const CT2_HOST_SEM6_INFO_REG: c_uint = 0x000148c8;
pub const CT2_HOST_SEM7_INFO_REG: c_uint = 0x000148cc;
pub const CT2_APP_PLL_LCLK_CTL_REG: c_uint = 0x00014808;
pub const __APP_LPUCLK_HALFSPEED: c_uint = 0x40000000;
pub const __APP_PLL_LCLK_LOAD: c_uint = 0x20000000;
pub const __APP_PLL_LCLK_FBCNT_MK: c_uint = 0x1fe00000;
pub const __APP_PLL_LCLK_FBCNT_SH: c_int = 21;

pub const __APP_PLL_LCLK_EXTFB: c_uint = 0x00000800;
pub const __APP_PLL_LCLK_ENOUTS: c_uint = 0x00000400;
pub const __APP_PLL_LCLK_RATE: c_uint = 0x00000010;
pub const CT2_APP_PLL_SCLK_CTL_REG: c_uint = 0x0001480c;
pub const __P_SCLK_PLL_LOCK: c_uint = 0x80000000;
pub const __APP_PLL_SCLK_REFCLK_SEL: c_uint = 0x40000000;
pub const __APP_PLL_SCLK_CLK_DIV2: c_uint = 0x20000000;
pub const __APP_PLL_SCLK_LOAD: c_uint = 0x10000000;
pub const __APP_PLL_SCLK_FBCNT_MK: c_uint = 0x0ff00000;
pub const __APP_PLL_SCLK_FBCNT_SH: c_int = 20;

pub const __APP_PLL_SCLK_EXTFB: c_uint = 0x00000800;
pub const __APP_PLL_SCLK_ENOUTS: c_uint = 0x00000400;
pub const __APP_PLL_SCLK_RATE: c_uint = 0x00000010;
pub const CT2_PCIE_MISC_REG: c_uint = 0x00014804;
pub const __ETH_CLK_ENABLE_PORT1: c_uint = 0x00000010;
pub const CT2_CHIP_MISC_PRG: c_uint = 0x000148a4;
pub const __ETH_CLK_ENABLE_PORT0: c_uint = 0x00004000;
pub const __APP_LPU_SPEED: c_uint = 0x00000002;
pub const CT2_MBIST_STAT_REG: c_uint = 0x00014818;
pub const CT2_MBIST_CTL_REG: c_uint = 0x0001481c;
pub const CT2_PMM_1T_CONTROL_REG_P0: c_uint = 0x0002381c;
pub const __PMM_1T_PNDB_P: c_uint = 0x00000002;
pub const CT2_PMM_1T_CONTROL_REG_P1: c_uint = 0x00023c1c;
pub const CT2_WGN_STATUS: c_uint = 0x00014990;
pub const __A2T_AHB_LOAD: c_uint = 0x00000800;
pub const __WGN_READY: c_uint = 0x00000400;
pub const __GLBL_PF_VF_CFG_RDY: c_uint = 0x00000200;
pub const CT2_NFC_STS_REG: c_uint = 0x00027410;
pub const CT2_NFC_CSR_CLR_REG: c_uint = 0x00027420;
pub const CT2_NFC_CSR_SET_REG: c_uint = 0x00027424;
pub const __HALT_NFC_CONTROLLER: c_uint = 0x00000002;
pub const __NFC_CONTROLLER_HALTED: c_uint = 0x00001000;
pub const CT2_RSC_GPR15_REG: c_uint = 0x0002765c;
pub const CT2_CSI_FW_CTL_REG: c_uint = 0x00027080;
pub const CT2_CSI_FW_CTL_SET_REG: c_uint = 0x00027088;
pub const __RESET_AND_START_SCLK_LCLK_PLLS: c_uint = 0x00010000;
pub const CT2_CSI_MAC0_CONTROL_REG: c_uint = 0x000270d0;
pub const __CSI_MAC_RESET: c_uint = 0x00000010;
pub const __CSI_MAC_AHB_RESET: c_uint = 0x00000008;
pub const CT2_CSI_MAC1_CONTROL_REG: c_uint = 0x000270d4;

pub const CT2_NFC_FLASH_STS_REG: c_uint = 0x00014834;
pub const __FLASH_PLL_INIT_AND_RESET_IN_PROGRESS: c_uint = 0x00000020;
//
// Name semaphore registers based on usage
//

//
// CT2 semaphore register locations changed
//

//
// And corresponding host interrupt status bit field defines
//
pub const __HFN_INT_CPE_Q0: c_uint = 0x00000001U;
pub const __HFN_INT_CPE_Q1: c_uint = 0x00000002U;
pub const __HFN_INT_CPE_Q2: c_uint = 0x00000004U;
pub const __HFN_INT_CPE_Q3: c_uint = 0x00000008U;
pub const __HFN_INT_CPE_Q4: c_uint = 0x00000010U;
pub const __HFN_INT_CPE_Q5: c_uint = 0x00000020U;
pub const __HFN_INT_CPE_Q6: c_uint = 0x00000040U;
pub const __HFN_INT_CPE_Q7: c_uint = 0x00000080U;
pub const __HFN_INT_RME_Q0: c_uint = 0x00000100U;
pub const __HFN_INT_RME_Q1: c_uint = 0x00000200U;
pub const __HFN_INT_RME_Q2: c_uint = 0x00000400U;
pub const __HFN_INT_RME_Q3: c_uint = 0x00000800U;
pub const __HFN_INT_RME_Q4: c_uint = 0x00001000U;
pub const __HFN_INT_RME_Q5: c_uint = 0x00002000U;
pub const __HFN_INT_RME_Q6: c_uint = 0x00004000U;
pub const __HFN_INT_RME_Q7: c_uint = 0x00008000U;
pub const __HFN_INT_ERR_EMC: c_uint = 0x00010000U;
pub const __HFN_INT_ERR_LPU0: c_uint = 0x00020000U;
pub const __HFN_INT_ERR_LPU1: c_uint = 0x00040000U;
pub const __HFN_INT_ERR_PSS: c_uint = 0x00080000U;
pub const __HFN_INT_MBOX_LPU0: c_uint = 0x00100000U;
pub const __HFN_INT_MBOX_LPU1: c_uint = 0x00200000U;
pub const __HFN_INT_MBOX1_LPU0: c_uint = 0x00400000U;
pub const __HFN_INT_MBOX1_LPU1: c_uint = 0x00800000U;
pub const __HFN_INT_LL_HALT: c_uint = 0x01000000U;
pub const __HFN_INT_CPE_MASK: c_uint = 0x000000ffU;
pub const __HFN_INT_RME_MASK: c_uint = 0x0000ff00U;

//
// Host interrupt status defines for catapult-2
//
pub const __HFN_INT_MBOX_LPU0_CT2: c_uint = 0x00010000U;
pub const __HFN_INT_MBOX_LPU1_CT2: c_uint = 0x00020000U;
pub const __HFN_INT_ERR_PSS_CT2: c_uint = 0x00040000U;
pub const __HFN_INT_ERR_LPU0_CT2: c_uint = 0x00080000U;
pub const __HFN_INT_ERR_LPU1_CT2: c_uint = 0x00100000U;
pub const __HFN_INT_CPQ_HALT_CT2: c_uint = 0x00200000U;
pub const __HFN_INT_ERR_WGN_CT2: c_uint = 0x00400000U;
pub const __HFN_INT_ERR_LEHRX_CT2: c_uint = 0x00800000U;
pub const __HFN_INT_ERR_LEHTX_CT2: c_uint = 0x01000000U;

//
// asic memory map.
//
pub const PSS_SMEM_PAGE_START: c_uint = 0x8000;

