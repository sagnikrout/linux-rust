//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/gaudi_regs.h
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

pub const GAUDI_ECC_MEM_SEL_OFFSET: c_uint = 0xF18;
pub const GAUDI_ECC_ADDRESS_OFFSET: c_uint = 0xF1C;
pub const GAUDI_ECC_SYNDROME_OFFSET: c_uint = 0xF20;
pub const GAUDI_ECC_MEM_INFO_CLR_OFFSET: c_uint = 0xF28;

pub const GAUDI_ECC_SERR0_OFFSET: c_uint = 0xF30;
pub const GAUDI_ECC_DERR0_OFFSET: c_uint = 0xF40;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_SOB_OBJ_0: c_uint = 0x492000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_ADDRL_0: c_uint = 0x494000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_ADDRH_0: c_uint = 0x494800;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_DATA_0: c_uint = 0x495000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0: c_uint = 0x495800;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0: c_uint = 0x496000;
pub const mmSYNC_MNGR_E_S_SYNC_MNGR_OBJS_SOB_OBJ_0: c_uint = 0x4B2000;
pub const mmSYNC_MNGR_E_S_SYNC_MNGR_OBJS_MON_STATUS_0: c_uint = 0x4B6000;
pub const mmSYNC_MNGR_W_N_SYNC_MNGR_OBJS_SOB_OBJ_0: c_uint = 0x4D2000;
pub const mmSYNC_MNGR_W_N_SYNC_MNGR_OBJS_MON_STATUS_0: c_uint = 0x4D6000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_0: c_uint = 0x4F2000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_1: c_uint = 0x4F2004;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_2047: c_uint = 0x4F3FFC;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_ADDRL_0: c_uint = 0x4F4000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_ADDRH_0: c_uint = 0x4F4800;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_DATA_0: c_uint = 0x4F5000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_ARM_0: c_uint = 0x4F5800;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_0: c_uint = 0x4F6000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_511: c_uint = 0x4F67FC;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_HIT_AW: c_uint = 0x300400;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_HIT_AW: c_uint = 0x310400;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_HIT_AW: c_uint = 0x320400;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_HIT_AW: c_uint = 0x330400;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_HIT_AW: c_uint = 0x340400;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_HIT_AW: c_uint = 0x350400;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_HIT_AW: c_uint = 0x360400;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_HIT_AW: c_uint = 0x370400;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_HIT_AR: c_uint = 0x300490;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_HIT_AR: c_uint = 0x310490;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_HIT_AR: c_uint = 0x320490;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_HIT_AR: c_uint = 0x330490;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_HIT_AR: c_uint = 0x340490;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_HIT_AR: c_uint = 0x350490;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_HIT_AR: c_uint = 0x360490;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_HIT_AR: c_uint = 0x370490;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x300410;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x310410;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x320410;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x330410;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x340410;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x350410;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x360410;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x370410;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x300450;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x310450;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x320450;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x330450;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x340450;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x350450;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x360450;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x370450;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3004A0;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3104A0;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3204A0;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3304A0;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3404A0;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3504A0;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3604A0;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3704A0;
pub const mmSIF_RTR_0_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3004E0;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3104E0;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3204E0;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3304E0;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3404E0;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3504E0;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3604E0;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3704E0;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_HIT_AW: c_uint = 0x380400;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_HIT_AW: c_uint = 0x390400;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3A0400;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3B0400;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3C0400;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3D0400;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3E0400;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_HIT_AW: c_uint = 0x3F0400;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_HIT_AR: c_uint = 0x380490;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_HIT_AR: c_uint = 0x390490;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3A0490;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3B0490;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3C0490;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3D0490;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3E0490;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_HIT_AR: c_uint = 0x3F0490;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x380410;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x390410;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3A0410;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3B0410;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3C0410;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3D0410;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3E0410;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MIN_AW_0: c_uint = 0x3F0410;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x380450;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x390450;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3A0450;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3B0450;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3C0450;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3D0450;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3E0450;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MAX_AW_0: c_uint = 0x3F0450;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3804A0;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3904A0;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3A04A0;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3B04A0;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3C04A0;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3D04A0;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3E04A0;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MIN_AR_0: c_uint = 0x3F04A0;
pub const mmNIF_RTR_0_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3804E0;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3904E0;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3A04E0;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3B04E0;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3C04E0;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3D04E0;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3E04E0;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MAX_AR_0: c_uint = 0x3F04E0;
pub const mmDMA_IF_W_S_DOWN_RSP_MID_WGHT_0: c_uint = 0x489030;
pub const mmDMA_IF_W_S_DOWN_RSP_MID_WGHT_1: c_uint = 0x489034;
pub const mmDMA_IF_E_S_DOWN_RSP_MID_WGHT_0: c_uint = 0x4A9030;
pub const mmDMA_IF_E_S_DOWN_RSP_MID_WGHT_1: c_uint = 0x4A9034;
pub const mmDMA_IF_W_N_DOWN_RSP_MID_WGHT_0: c_uint = 0x4C9030;
pub const mmDMA_IF_W_N_DOWN_RSP_MID_WGHT_1: c_uint = 0x4C9034;
pub const mmDMA_IF_E_N_DOWN_RSP_MID_WGHT_0: c_uint = 0x4E9030;
pub const mmDMA_IF_E_N_DOWN_RSP_MID_WGHT_1: c_uint = 0x4E9034;
pub const mmMME1_QM_GLBL_CFG0: c_uint = 0xE8000;
pub const mmMME1_QM_GLBL_STS0: c_uint = 0xE8038;
pub const mmMME0_SBAB_SB_STALL: c_uint = 0x4002C;
pub const mmMME0_SBAB_ARUSER0: c_uint = 0x40034;
pub const mmMME0_SBAB_ARUSER1: c_uint = 0x40038;
pub const mmMME0_SBAB_PROT: c_uint = 0x40050;
pub const mmMME1_SBAB_SB_STALL: c_uint = 0xC002C;
pub const mmMME1_SBAB_ARUSER0: c_uint = 0xC0034;
pub const mmMME1_SBAB_ARUSER1: c_uint = 0xC0038;
pub const mmMME1_SBAB_PROT: c_uint = 0xC0050;
pub const mmMME2_SBAB_SB_STALL: c_uint = 0x14002C;
pub const mmMME2_SBAB_ARUSER0: c_uint = 0x140034;
pub const mmMME2_SBAB_ARUSER1: c_uint = 0x140038;
pub const mmMME2_SBAB_PROT: c_uint = 0x140050;
pub const mmMME3_SBAB_SB_STALL: c_uint = 0x1C002C;
pub const mmMME3_SBAB_ARUSER0: c_uint = 0x1C0034;
pub const mmMME3_SBAB_ARUSER1: c_uint = 0x1C0038;
pub const mmMME3_SBAB_PROT: c_uint = 0x1C0050;
pub const mmMME0_ACC_ACC_STALL: c_uint = 0x20028;
pub const mmMME0_ACC_WBC: c_uint = 0x20038;
pub const mmMME0_ACC_PROT: c_uint = 0x20050;
pub const mmMME1_ACC_ACC_STALL: c_uint = 0xA0028;
pub const mmMME1_ACC_WBC: c_uint = 0xA0038;
pub const mmMME1_ACC_PROT: c_uint = 0xA0050;
pub const mmMME2_ACC_ACC_STALL: c_uint = 0x120028;
pub const mmMME2_ACC_WBC: c_uint = 0x120038;
pub const mmMME2_ACC_PROT: c_uint = 0x120050;
pub const mmMME3_ACC_ACC_STALL: c_uint = 0x1A0028;
pub const mmMME3_ACC_WBC: c_uint = 0x1A0038;
pub const mmMME3_ACC_PROT: c_uint = 0x1A0050;
pub const mmGIC_DISTRIBUTOR__5_GICD_SETSPI_NSR: c_uint = 0x800040;
pub const mmPSOC_EFUSE_READ: c_uint = 0xC4A000;
pub const mmPSOC_EFUSE_DATA_0: c_uint = 0xC4A080;
pub const mmPCIE_WRAP_MAX_OUTSTAND: c_uint = 0xC01B20;
pub const mmPCIE_WRAP_LBW_PROT_OVR: c_uint = 0xC01B48;
pub const mmPCIE_WRAP_HBW_DRAIN_CFG: c_uint = 0xC01D54;
pub const mmPCIE_WRAP_LBW_DRAIN_CFG: c_uint = 0xC01D5C;
pub const mmPCIE_MSI_INTR_0: c_uint = 0xC13000;
pub const mmPCIE_DBI_DEVICE_ID_VENDOR_ID_REG: c_uint = 0xC02000;
pub const mmPCIE_AUX_FLR_CTRL: c_uint = 0xC07394;
pub const mmPCIE_AUX_DBI: c_uint = 0xC07490;
pub const mmPCIE_CORE_MSI_REQ: c_uint = 0xC04100;
pub const mmPSOC_PCI_PLL_NR: c_uint = 0xC72100;
pub const mmSRAM_W_PLL_NR: c_uint = 0x4C8100;
pub const mmPSOC_HBM_PLL_NR: c_uint = 0xC74100;
pub const mmNIC0_PLL_NR: c_uint = 0xCF9100;
pub const mmDMA_W_PLL_NR: c_uint = 0x487100;
pub const mmMESH_W_PLL_NR: c_uint = 0x4C7100;
pub const mmPSOC_MME_PLL_NR: c_uint = 0xC71100;
pub const mmPSOC_TPC_PLL_NR: c_uint = 0xC73100;
pub const mmIF_W_PLL_NR: c_uint = 0x488100;
pub const mmPCIE_WRAP_RR_ELBI_RD_SEC_REG_CTRL: c_uint = 0xC01208;
