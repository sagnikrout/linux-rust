//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv090x_reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

// Macro flag: #define __STV090x_REG_H
pub const STV090x_MID: c_uint = 0xf100;
pub const STV090x_OFFST_MCHIP_IDENT_FIELD: c_int = 4;
pub const STV090x_WIDTH_MCHIP_IDENT_FIELD: c_int = 4;
pub const STV090x_OFFST_MRELEASE_FIELD: c_int = 0;
pub const STV090x_WIDTH_MRELEASE_FIELD: c_int = 4;
pub const STV090x_DACR1: c_uint = 0xf113;
pub const STV090x_OFFST_DACR1_MODE_FIELD: c_int = 5;
pub const STV090x_WIDTH_DACR1_MODE_FIELD: c_int = 3;
pub const STV090x_OFFST_DACR1_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_DACR1_VALUE_FIELD: c_int = 4;
pub const STV090x_DACR2: c_uint = 0xf114;
pub const STV090x_OFFST_DACR2_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_DACR2_VALUE_FIELD: c_int = 8;
pub const STV090x_OUTCFG: c_uint = 0xf11c;
pub const STV090x_OFFST_OUTSERRS1_HZ_FIELD: c_int = 6;
pub const STV090x_WIDTH_OUTSERRS1_HZ_FIELD: c_int = 1;
pub const STV090x_OFFST_OUTSERRS2_HZ_FIELD: c_int = 5;
pub const STV090x_WIDTH_OUTSERRS2_HZ_FIELD: c_int = 1;
pub const STV090x_OFFST_OUTSERRS3_HZ_FIELD: c_int = 4;
pub const STV090x_WIDTH_OUTSERRS3_HZ_FIELD: c_int = 1;
pub const STV090x_OFFST_OUTPARRS3_HZ_FIELD: c_int = 3;
pub const STV090x_WIDTH_OUTPARRS3_HZ_FIELD: c_int = 1;
pub const STV090x_MODECFG: c_uint = 0xf11d;
pub const STV090x_IRQSTATUS3: c_uint = 0xf120;
pub const STV090x_OFFST_SPLL_LOCK_FIELD: c_int = 5;
pub const STV090x_WIDTH_SPLL_LOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_SSTREAM_LCK_3_FIELD: c_int = 4;
pub const STV090x_WIDTH_SSTREAM_LCK_3_FIELD: c_int = 1;
pub const STV090x_OFFST_SSTREAM_LCK_2_FIELD: c_int = 3;
pub const STV090x_WIDTH_SSTREAM_LCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SSTREAM_LCK_1_FIELD: c_int = 2;
pub const STV090x_WIDTH_SSTREAM_LCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SDVBS1_PRF_2_FIELD: c_int = 1;
pub const STV090x_WIDTH_SDVBS1_PRF_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SDVBS1_PRF_1_FIELD: c_int = 0;
pub const STV090x_WIDTH_SDVBS1_PRF_1_FIELD: c_int = 1;
pub const STV090x_IRQSTATUS2: c_uint = 0xf121;
pub const STV090x_OFFST_SSPY_ENDSIM_3_FIELD: c_int = 7;
pub const STV090x_WIDTH_SSPY_ENDSIM_3_FIELD: c_int = 1;
pub const STV090x_OFFST_SSPY_ENDSIM_2_FIELD: c_int = 6;
pub const STV090x_WIDTH_SSPY_ENDSIM_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SSPY_ENDSIM_1_FIELD: c_int = 5;
pub const STV090x_WIDTH_SSPY_ENDSIM_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SPKTDEL_ERROR_2_FIELD: c_int = 4;
pub const STV090x_WIDTH_SPKTDEL_ERROR_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SPKTDEL_LOCKB_2_FIELD: c_int = 3;
pub const STV090x_WIDTH_SPKTDEL_LOCKB_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SPKTDEL_LOCK_2_FIELD: c_int = 2;
pub const STV090x_WIDTH_SPKTDEL_LOCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SPKTDEL_ERROR_1_FIELD: c_int = 1;
pub const STV090x_WIDTH_SPKTDEL_ERROR_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SPKTDEL_LOCKB_1_FIELD: c_int = 0;
pub const STV090x_WIDTH_SPKTDEL_LOCKB_1_FIELD: c_int = 1;
pub const STV090x_IRQSTATUS1: c_uint = 0xf122;
pub const STV090x_OFFST_SPKTDEL_LOCK_1_FIELD: c_int = 7;
pub const STV090x_WIDTH_SPKTDEL_LOCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SDEMOD_LOCKB_2_FIELD: c_int = 2;
pub const STV090x_WIDTH_SDEMOD_LOCKB_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SDEMOD_LOCK_2_FIELD: c_int = 1;
pub const STV090x_WIDTH_SDEMOD_LOCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_SDEMOD_IRQ_2_FIELD: c_int = 0;
pub const STV090x_WIDTH_SDEMOD_IRQ_2_FIELD: c_int = 1;
pub const STV090x_IRQSTATUS0: c_uint = 0xf123;
pub const STV090x_OFFST_SDEMOD_LOCKB_1_FIELD: c_int = 7;
pub const STV090x_WIDTH_SDEMOD_LOCKB_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SDEMOD_LOCK_1_FIELD: c_int = 6;
pub const STV090x_WIDTH_SDEMOD_LOCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SDEMOD_IRQ_1_FIELD: c_int = 5;
pub const STV090x_WIDTH_SDEMOD_IRQ_1_FIELD: c_int = 1;
pub const STV090x_OFFST_SBCH_ERRFLAG_FIELD: c_int = 4;
pub const STV090x_WIDTH_SBCH_ERRFLAG_FIELD: c_int = 1;
pub const STV090x_OFFST_SDISEQC2RX_IRQ_FIELD: c_int = 3;
pub const STV090x_WIDTH_SDISEQC2RX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_SDISEQC2TX_IRQ_FIELD: c_int = 2;
pub const STV090x_WIDTH_SDISEQC2TX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_SDISEQC1RX_IRQ_FIELD: c_int = 1;
pub const STV090x_WIDTH_SDISEQC1RX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_SDISEQC1TX_IRQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_SDISEQC1TX_IRQ_FIELD: c_int = 1;
pub const STV090x_IRQMASK3: c_uint = 0xf124;
pub const STV090x_OFFST_MPLL_LOCK_FIELD: c_int = 5;
pub const STV090x_WIDTH_MPLL_LOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_MSTREAM_LCK_3_FIELD: c_int = 4;
pub const STV090x_WIDTH_MSTREAM_LCK_3_FIELD: c_int = 1;
pub const STV090x_OFFST_MSTREAM_LCK_2_FIELD: c_int = 3;
pub const STV090x_WIDTH_MSTREAM_LCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MSTREAM_LCK_1_FIELD: c_int = 2;
pub const STV090x_WIDTH_MSTREAM_LCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MDVBS1_PRF_2_FIELD: c_int = 1;
pub const STV090x_WIDTH_MDVBS1_PRF_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MDVBS1_PRF_1_FIELD: c_int = 0;
pub const STV090x_WIDTH_MDVBS1_PRF_1_FIELD: c_int = 1;
pub const STV090x_IRQMASK2: c_uint = 0xf125;
pub const STV090x_OFFST_MSPY_ENDSIM_3_FIELD: c_int = 7;
pub const STV090x_WIDTH_MSPY_ENDSIM_3_FIELD: c_int = 1;
pub const STV090x_OFFST_MSPY_ENDSIM_2_FIELD: c_int = 6;
pub const STV090x_WIDTH_MSPY_ENDSIM_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MSPY_ENDSIM_1_FIELD: c_int = 5;
pub const STV090x_WIDTH_MSPY_ENDSIM_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MPKTDEL_ERROR_2_FIELD: c_int = 4;
pub const STV090x_WIDTH_MPKTDEL_ERROR_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MPKTDEL_LOCKB_2_FIELD: c_int = 3;
pub const STV090x_WIDTH_MPKTDEL_LOCKB_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MPKTDEL_LOCK_2_FIELD: c_int = 2;
pub const STV090x_WIDTH_MPKTDEL_LOCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MPKTDEL_ERROR_1_FIELD: c_int = 1;
pub const STV090x_WIDTH_MPKTDEL_ERROR_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MPKTDEL_LOCKB_1_FIELD: c_int = 0;
pub const STV090x_WIDTH_MPKTDEL_LOCKB_1_FIELD: c_int = 1;
pub const STV090x_IRQMASK1: c_uint = 0xf126;
pub const STV090x_OFFST_MPKTDEL_LOCK_1_FIELD: c_int = 7;
pub const STV090x_WIDTH_MPKTDEL_LOCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MEXTPINB2_FIELD: c_int = 6;
pub const STV090x_WIDTH_MEXTPINB2_FIELD: c_int = 1;
pub const STV090x_OFFST_MEXTPIN2_FIELD: c_int = 5;
pub const STV090x_WIDTH_MEXTPIN2_FIELD: c_int = 1;
pub const STV090x_OFFST_MEXTPINB1_FIELD: c_int = 4;
pub const STV090x_WIDTH_MEXTPINB1_FIELD: c_int = 1;
pub const STV090x_OFFST_MEXTPIN1_FIELD: c_int = 3;
pub const STV090x_WIDTH_MEXTPIN1_FIELD: c_int = 1;
pub const STV090x_OFFST_MDEMOD_LOCKB_2_FIELD: c_int = 2;
pub const STV090x_WIDTH_MDEMOD_LOCKB_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MDEMOD_LOCK_2_FIELD: c_int = 1;
pub const STV090x_WIDTH_MDEMOD_LOCK_2_FIELD: c_int = 1;
pub const STV090x_OFFST_MDEMOD_IRQ_2_FIELD: c_int = 0;
pub const STV090x_WIDTH_MDEMOD_IRQ_2_FIELD: c_int = 1;
pub const STV090x_IRQMASK0: c_uint = 0xf127;
pub const STV090x_OFFST_MDEMOD_LOCKB_1_FIELD: c_int = 7;
pub const STV090x_WIDTH_MDEMOD_LOCKB_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MDEMOD_LOCK_1_FIELD: c_int = 6;
pub const STV090x_WIDTH_MDEMOD_LOCK_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MDEMOD_IRQ_1_FIELD: c_int = 5;
pub const STV090x_WIDTH_MDEMOD_IRQ_1_FIELD: c_int = 1;
pub const STV090x_OFFST_MBCH_ERRFLAG_FIELD: c_int = 4;
pub const STV090x_WIDTH_MBCH_ERRFLAG_FIELD: c_int = 1;
pub const STV090x_OFFST_MDISEQC2RX_IRQ_FIELD: c_int = 3;
pub const STV090x_WIDTH_MDISEQC2RX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_MDISEQC2TX_IRQ_FIELD: c_int = 2;
pub const STV090x_WIDTH_MDISEQC2TX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_MDISEQC1RX_IRQ_FIELD: c_int = 1;
pub const STV090x_WIDTH_MDISEQC1RX_IRQ_FIELD: c_int = 1;
pub const STV090x_OFFST_MDISEQC1TX_IRQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_MDISEQC1TX_IRQ_FIELD: c_int = 1;
pub const STV090x_I2CCFG: c_uint = 0xf129;
pub const STV090x_OFFST_12C_FASTMODE_FIELD: c_int = 3;
pub const STV090x_WIDTH_12C_FASTMODE_FIELD: c_int = 1;
pub const STV090x_OFFST_12CADDR_INC_FIELD: c_int = 0;
pub const STV090x_WIDTH_12CADDR_INC_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_I2CT_ON_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_I2CT_ON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ENARPT_LEVEL_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_ENARPT_LEVEL_FIELD: c_int = 3;
pub const STV090x_OFFST_Px_SCLT_DELAY_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_SCLT_DELAY_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_STOP_ENABLE_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_STOP_ENABLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_STOP_SDAT2SDA_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_STOP_SDAT2SDA_FIELD: c_int = 1;
pub const STV090x_CLKI2CFG: c_uint = 0xf140;
pub const STV090x_OFFST_CLKI2_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_CLKI2_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_CLKI2_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_CLKI2_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_CLKI2_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_CLKI2_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_GPIOx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_GPIOx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_GPIOx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_GPIOx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_GPIOx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_GPIOx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_CSX_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_CSX_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_CSX_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_CSX_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_CSX_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_CSX_XOR_FIELD: c_int = 1;
pub const STV090x_STDBYCFG: c_uint = 0xf150;
pub const STV090x_OFFST_STDBY_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_STDBY_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_STDBY_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_STDBY_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_STDBY_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_STDBY_XOR_FIELD: c_int = 1;
pub const STV090x_DIRCLKCFG: c_uint = 0xf151;
pub const STV090x_OFFST_DIRCLK_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_DIRCLK_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_DIRCLK_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_DIRCLK_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_DIRCLK_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_DIRCLK_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_AGCRFx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_AGCRFx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_AGCRFx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_AGCRFx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_AGCRFx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_AGCRFx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_SDATx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_SDATx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_SDATx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_SDATx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_SDATx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_SDATx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_SCLTx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_SCLTx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_SCLTx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_SCLTx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_SCLTx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_SCLTx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_DISEQCOx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_DISEQCOx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_DISEQCOx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_DISEQCOx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_DISEQCOx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_DISEQCOx_XOR_FIELD: c_int = 1;
pub const STV090x_CLKOUT27CFG: c_uint = 0xf15a;
pub const STV090x_OFFST_CLKOUT27_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_CLKOUT27_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_CLKOUT27_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_CLKOUT27_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_CLKOUT27_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_CLKOUT27_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_ERRORx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_ERRORx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_ERRORx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_ERRORx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_ERRORx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_ERRORx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_DPNx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_DPNx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_DPNx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_DPNx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_DPNx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_DPNx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_STROUTx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_STROUTx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_STROUTx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_STROUTx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_STROUTx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_STROUTx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_CLKOUTx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_CLKOUTx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_CLKOUTx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_CLKOUTx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_CLKOUTx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_CLKOUTx_XOR_FIELD: c_int = 1;

pub const STV090x_OFFST_DATAx_OPD_FIELD: c_int = 7;
pub const STV090x_WIDTH_DATAx_OPD_FIELD: c_int = 1;
pub const STV090x_OFFST_DATAx_CONFIG_FIELD: c_int = 1;
pub const STV090x_WIDTH_DATAx_CONFIG_FIELD: c_int = 6;
pub const STV090x_OFFST_DATAx_XOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_DATAx_XOR_FIELD: c_int = 1;
pub const STV090x_NCOARSE: c_uint = 0xf1b3;
pub const STV090x_OFFST_M_DIV_FIELD: c_int = 0;
pub const STV090x_WIDTH_M_DIV_FIELD: c_int = 8;
pub const STV090x_SYNTCTRL: c_uint = 0xf1b6;
pub const STV090x_OFFST_STANDBY_FIELD: c_int = 7;
pub const STV090x_WIDTH_STANDBY_FIELD: c_int = 1;
pub const STV090x_OFFST_BYPASSPLLCORE_FIELD: c_int = 6;
pub const STV090x_WIDTH_BYPASSPLLCORE_FIELD: c_int = 1;
pub const STV090x_OFFST_SELX1RATIO_FIELD: c_int = 5;
pub const STV090x_WIDTH_SELX1RATIO_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_PLL_FIELD: c_int = 3;
pub const STV090x_WIDTH_STOP_PLL_FIELD: c_int = 1;
pub const STV090x_OFFST_BYPASSPLLFSK_FIELD: c_int = 2;
pub const STV090x_WIDTH_BYPASSPLLFSK_FIELD: c_int = 1;
pub const STV090x_OFFST_SELOSCI_FIELD: c_int = 1;
pub const STV090x_WIDTH_SELOSCI_FIELD: c_int = 1;
pub const STV090x_OFFST_BYPASSPLLADC_FIELD: c_int = 0;
pub const STV090x_WIDTH_BYPASSPLLADC_FIELD: c_int = 1;
pub const STV090x_FILTCTRL: c_uint = 0xf1b7;
pub const STV090x_OFFST_INV_CLK135_FIELD: c_int = 7;
pub const STV090x_WIDTH_INV_CLK135_FIELD: c_int = 1;
pub const STV090x_OFFST_SEL_FSKCKDIV_FIELD: c_int = 2;
pub const STV090x_WIDTH_SEL_FSKCKDIV_FIELD: c_int = 1;
pub const STV090x_OFFST_INV_CLKFSK_FIELD: c_int = 1;
pub const STV090x_WIDTH_INV_CLKFSK_FIELD: c_int = 1;
pub const STV090x_OFFST_BYPASS_APPLI_FIELD: c_int = 0;
pub const STV090x_WIDTH_BYPASS_APPLI_FIELD: c_int = 1;
pub const STV090x_PLLSTAT: c_uint = 0xf1b8;
pub const STV090x_OFFST_PLLLOCK_FIELD: c_int = 0;
pub const STV090x_WIDTH_PLLLOCK_FIELD: c_int = 1;
pub const STV090x_STOPCLK1: c_uint = 0xf1c2;
pub const STV090x_OFFST_STOP_CLKPKDT2_FIELD: c_int = 6;
pub const STV090x_WIDTH_STOP_CLKPKDT2_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKPKDT1_FIELD: c_int = 5;
pub const STV090x_WIDTH_STOP_CLKPKDT1_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKFEC_FIELD: c_int = 4;
pub const STV090x_WIDTH_STOP_CLKFEC_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKADCI2_FIELD: c_int = 3;
pub const STV090x_WIDTH_STOP_CLKADCI2_FIELD: c_int = 1;
pub const STV090x_OFFST_INV_CLKADCI2_FIELD: c_int = 2;
pub const STV090x_WIDTH_INV_CLKADCI2_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKADCI1_FIELD: c_int = 1;
pub const STV090x_WIDTH_STOP_CLKADCI1_FIELD: c_int = 1;
pub const STV090x_OFFST_INV_CLKADCI1_FIELD: c_int = 0;
pub const STV090x_WIDTH_INV_CLKADCI1_FIELD: c_int = 1;
pub const STV090x_STOPCLK2: c_uint = 0xf1c3;
pub const STV090x_OFFST_STOP_CLKSAMP2_FIELD: c_int = 4;
pub const STV090x_WIDTH_STOP_CLKSAMP2_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKSAMP1_FIELD: c_int = 3;
pub const STV090x_WIDTH_STOP_CLKSAMP1_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKVIT2_FIELD: c_int = 2;
pub const STV090x_WIDTH_STOP_CLKVIT2_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKVIT1_FIELD: c_int = 1;
pub const STV090x_WIDTH_STOP_CLKVIT1_FIELD: c_int = 1;
pub const STV090x_OFFST_STOP_CLKTS_FIELD: c_int = 0;
pub const STV090x_WIDTH_STOP_CLKTS_FIELD: c_int = 1;
pub const STV090x_TSTTNR0: c_uint = 0xf1df;
pub const STV090x_OFFST_SEL_FSK_FIELD: c_int = 7;
pub const STV090x_WIDTH_SEL_FSK_FIELD: c_int = 1;
pub const STV090x_OFFST_FSK_PON_FIELD: c_int = 2;
pub const STV090x_WIDTH_FSK_PON_FIELD: c_int = 1;
pub const STV090x_TSTTNR1: c_uint = 0xf1e0;
pub const STV090x_OFFST_ADC1_PON_FIELD: c_int = 1;
pub const STV090x_WIDTH_ADC1_PON_FIELD: c_int = 1;
pub const STV090x_OFFST_ADC1_INMODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_ADC1_INMODE_FIELD: c_int = 1;
pub const STV090x_TSTTNR2: c_uint = 0xf1e1;
pub const STV090x_OFFST_DISEQC1_PON_FIELD: c_int = 5;
pub const STV090x_WIDTH_DISEQC1_PON_FIELD: c_int = 1;
pub const STV090x_TSTTNR3: c_uint = 0xf1e2;
pub const STV090x_OFFST_ADC2_PON_FIELD: c_int = 1;
pub const STV090x_WIDTH_ADC2_PON_FIELD: c_int = 1;
pub const STV090x_OFFST_ADC2_INMODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_ADC2_INMODE_FIELD: c_int = 1;
pub const STV090x_TSTTNR4: c_uint = 0xf1e3;
pub const STV090x_OFFST_DISEQC2_PON_FIELD: c_int = 5;
pub const STV090x_WIDTH_DISEQC2_PON_FIELD: c_int = 1;
pub const STV090x_FSKTFC2: c_uint = 0xf170;
pub const STV090x_OFFST_FSKT_KMOD_FIELD: c_int = 2;
pub const STV090x_WIDTH_FSKT_KMOD_FIELD: c_int = 6;
pub const STV090x_OFFST_FSKT_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKT_CAR_FIELD: c_int = 2;
pub const STV090x_FSKTFC1: c_uint = 0xf171;
pub const STV090x_OFFST_FSKTC1_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKTC1_CAR_FIELD: c_int = 8;
pub const STV090x_FSKTFC0: c_uint = 0xf172;
pub const STV090x_OFFST_FSKTC0_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKTC0_CAR_FIELD: c_int = 8;
pub const STV090x_FSKTDELTAF1: c_uint = 0xf173;
pub const STV090x_OFFST_FSKTF1_DELTAF_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKTF1_DELTAF_FIELD: c_int = 4;
pub const STV090x_FSKTDELTAF0: c_uint = 0xf174;
pub const STV090x_OFFST_FSKTF0_DELTAF_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKTF0_DELTAF_FIELD: c_int = 8;
pub const STV090x_FSKTCTRL: c_uint = 0xf175;
pub const STV090x_OFFST_FSKT_EN_SGN_FIELD: c_int = 6;
pub const STV090x_WIDTH_FSKT_EN_SGN_FIELD: c_int = 1;
pub const STV090x_OFFST_FSKT_MOD_SGN_FIELD: c_int = 5;
pub const STV090x_WIDTH_FSKT_MOD_SGN_FIELD: c_int = 1;
pub const STV090x_OFFST_FSKT_MOD_EN_FIELD: c_int = 2;
pub const STV090x_WIDTH_FSKT_MOD_EN_FIELD: c_int = 3;
pub const STV090x_OFFST_FSKT_DACMODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKT_DACMODE_FIELD: c_int = 2;
pub const STV090x_FSKRFC2: c_uint = 0xf176;
pub const STV090x_OFFST_FSKRC2_DETSGN_FIELD: c_int = 6;
pub const STV090x_WIDTH_FSKRC2_DETSGN_FIELD: c_int = 1;
pub const STV090x_OFFST_FSKRC2_OUTSGN_FIELD: c_int = 5;
pub const STV090x_WIDTH_FSKRC2_OUTSGN_FIELD: c_int = 1;
pub const STV090x_OFFST_FSKRC2_KAGC_FIELD: c_int = 2;
pub const STV090x_WIDTH_FSKRC2_KAGC_FIELD: c_int = 3;
pub const STV090x_OFFST_FSKRC2_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKRC2_CAR_FIELD: c_int = 2;
pub const STV090x_FSKRFC1: c_uint = 0xf177;
pub const STV090x_OFFST_FSKRC1_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKRC1_CAR_FIELD: c_int = 8;
pub const STV090x_FSKRFC0: c_uint = 0xf178;
pub const STV090x_OFFST_FSKRC0_CAR_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKRC0_CAR_FIELD: c_int = 8;
pub const STV090x_FSKRK1: c_uint = 0xf179;
pub const STV090x_OFFST_FSKR_K1_EXP_FIELD: c_int = 5;
pub const STV090x_WIDTH_FSKR_K1_EXP_FIELD: c_int = 3;
pub const STV090x_OFFST_FSKR_K1_MANT_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_K1_MANT_FIELD: c_int = 5;
pub const STV090x_FSKRK2: c_uint = 0xf17a;
pub const STV090x_OFFST_FSKR_K2_EXP_FIELD: c_int = 5;
pub const STV090x_WIDTH_FSKR_K2_EXP_FIELD: c_int = 3;
pub const STV090x_OFFST_FSKR_K2_MANT_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_K2_MANT_FIELD: c_int = 5;
pub const STV090x_FSKRAGCR: c_uint = 0xf17b;
pub const STV090x_OFFST_FSKR_OUTCTL_FIELD: c_int = 6;
pub const STV090x_WIDTH_FSKR_OUTCTL_FIELD: c_int = 2;
pub const STV090x_OFFST_FSKR_AGC_REF_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_AGC_REF_FIELD: c_int = 6;
pub const STV090x_FSKRAGC: c_uint = 0xf17c;
pub const STV090x_OFFST_FSKR_AGC_ACCU_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_AGC_ACCU_FIELD: c_int = 8;
pub const STV090x_FSKRALPHA: c_uint = 0xf17d;
pub const STV090x_OFFST_FSKR_ALPHA_EXP_FIELD: c_int = 2;
pub const STV090x_WIDTH_FSKR_ALPHA_EXP_FIELD: c_int = 3;
pub const STV090x_OFFST_FSKR_ALPHA_M_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_ALPHA_M_FIELD: c_int = 2;
pub const STV090x_FSKRPLTH1: c_uint = 0xf17e;
pub const STV090x_OFFST_FSKR_BETA_FIELD: c_int = 4;
pub const STV090x_WIDTH_FSKR_BETA_FIELD: c_int = 4;
pub const STV090x_OFFST_FSKR_PLL_TRESH1_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_PLL_TRESH1_FIELD: c_int = 4;
pub const STV090x_FSKRPLTH0: c_uint = 0xf17f;
pub const STV090x_OFFST_FSKR_PLL_TRESH0_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_PLL_TRESH0_FIELD: c_int = 8;
pub const STV090x_FSKRDF1: c_uint = 0xf180;
pub const STV090x_OFFST_FSKR_DELTAF1_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_DELTAF1_FIELD: c_int = 5;
pub const STV090x_FSKRDF0: c_uint = 0xf181;
pub const STV090x_OFFST_FSKR_DELTAF0_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_DELTAF0_FIELD: c_int = 8;
pub const STV090x_FSKRSTEPP: c_uint = 0xf182;
pub const STV090x_OFFST_FSKR_STEP_PLUS_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_STEP_PLUS_FIELD: c_int = 8;
pub const STV090x_FSKRSTEPM: c_uint = 0xf183;
pub const STV090x_OFFST_FSKR_STEP_MINUS_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_STEP_MINUS_FIELD: c_int = 8;
pub const STV090x_FSKRDET1: c_uint = 0xf184;
pub const STV090x_OFFST_FSKR_CARDET1_ACCU_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_CARDET1_ACCU_FIELD: c_int = 4;
pub const STV090x_FSKRDET0: c_uint = 0xf185;
pub const STV090x_OFFST_FSKR_CARDET0_ACCU_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_CARDET0_ACCU_FIELD: c_int = 8;
pub const STV090x_FSKRDTH1: c_uint = 0xf186;
pub const STV090x_OFFST_FSKR_CARLOSS_THRESH1_FIELD: c_int = 4;
pub const STV090x_WIDTH_FSKR_CARLOSS_THRESH1_FIELD: c_int = 4;
pub const STV090x_OFFST_FSKR_CARDET_THRESH1_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_CARDET_THRESH1_FIELD: c_int = 4;
pub const STV090x_FSKRDTH0: c_uint = 0xf187;
pub const STV090x_OFFST_FSKR_CARDET_THRESH0_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_CARDET_THRESH0_FIELD: c_int = 8;
pub const STV090x_FSKRLOSS: c_uint = 0xf188;
pub const STV090x_OFFST_FSKR_CARLOSS_THRESH_FIELD: c_int = 0;
pub const STV090x_WIDTH_FSKR_CARLOSS_THRESH_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TIM_OFF_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TIM_OFF_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DISEQC_RESET_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_DISEQC_RESET_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TIM_CMD_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_TIM_CMD_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_DIS_PRECHARGE_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_DIS_PRECHARGE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DISTX_MODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DISTX_MODE_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_RECEIVER_ON_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_RECEIVER_ON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_IGNO_SHORT22K_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_IGNO_SHORT22K_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ONECHIP_TRX_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_ONECHIP_TRX_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_EXT_ENVELOP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_EXT_ENVELOP_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_PIN_SELECT_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_PIN_SELECT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_IRQ_RXEND_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_IRQ_RXEND_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_IRQ_4NBYTES_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_IRQ_4NBYTES_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_RX_END_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_RX_END_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_RX_ACTIVE_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_RX_ACTIVE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SHORT_22KHZ_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_SHORT_22KHZ_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_CONT_TONE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CONT_TONE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_4BREADY_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_FIFO_4BREADY_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_EMPTY_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_FIFO_EMPTY_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ABORT_DISRX_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ABORT_DISRX_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_RX_FAIL_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_RX_FAIL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_PARITYFAIL_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_FIFO_PARITYFAIL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_RX_NONBYTE_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_RX_NONBYTE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_OVERFLOW_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_FIFO_OVERFLOW_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_BYTENBR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FIFO_BYTENBR_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DISRX_DATA_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DISRX_DATA_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DISEQC_FIFO_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DISEQC_FIFO_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TX_FAIL_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TX_FAIL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIFO_FULL_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_FIFO_FULL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TX_IDLE_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_TX_IDLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_GAP_BURST_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_GAP_BURST_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TXFIFO_BYTES_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TXFIFO_BYTES_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_F22_REG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_F22_REG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_F22RX_REG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_F22RX_REG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ACR_PRESC_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ACR_PRESC_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_ACR_DIV_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ACR_DIV_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CONSTEL_SELECT_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_CONSTEL_SELECT_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_NOSPLH_BETA_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_NOSPLH_BETA_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_NOSDATA_BETA_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSDATA_BETA_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_I_SYMBOL_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_I_SYMBOL_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_Q_SYMBOL_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_Q_SYMBOL_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DC_FROZEN_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DC_FROZEN_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DC_CORRECT_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_DC_CORRECT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AMM_FROZEN_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_AMM_FROZEN_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AMM_CORRECT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_AMM_CORRECT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_QUAD_FROZEN_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_QUAD_FROZEN_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_QUAD_CORRECT_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_QUAD_CORRECT_FIELD: c_int = 1;

pub const STV090x_WIDTH_Px_AGC1_LOCKED_FIELD: c_int = 7;
pub const STV090x_OFFST_Px_AGC1_LOCKED_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AGC1_MINPOWER_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_AGC1_MINPOWER_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AGCOUT_FAST_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_AGCOUT_FAST_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AGCIQ_BETA_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGCIQ_BETA_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_AGCIQ_REF_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGCIQ_REF_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_IAVERAGE_ADJ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_IAVERAGE_ADJ_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_QAVERAGE_ADJ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_QAVERAGE_ADJ_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_POWER_I_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_POWER_I_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_POWER_Q_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_POWER_Q_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_AMM_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AMM_VALUE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_QUAD_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_QUAD_VALUE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_AGCIQ_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGCIQ_VALUE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_MANUAL_S2ROLLOFF_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_MANUAL_S2ROLLOFF_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DEMOD_STOP_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_DEMOD_STOP_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SPECINV_CONTROL_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_SPECINV_CONTROL_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_FORCE_ENASAMP_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_FORCE_ENASAMP_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_MANUAL_SXROLLOFF_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_MANUAL_SXROLLOFF_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ROLLOFF_CONTROL_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ROLLOFF_CONTROL_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_MANUAL_MODCOD_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_MANUAL_MODCOD_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DEMOD_MODCOD_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_DEMOD_MODCOD_FIELD: c_int = 5;
pub const STV090x_OFFST_Px_DEMOD_TYPE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DEMOD_TYPE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_CAR_LOCK_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_CAR_LOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TMGLOCK_QUALITY_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_TMGLOCK_QUALITY_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_LOCK_DEFINITIF_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_LOCK_DEFINITIF_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DEMOD_DELOCK_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DEMOD_DELOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AGC1_NOSIGNALACK_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_AGC1_NOSIGNALACK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_AGC2_OVERFLOW_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_AGC2_OVERFLOW_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_CFR_OVERFLOW_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_CFR_OVERFLOW_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_GAMMA_OVERUNDER_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_GAMMA_OVERUNDER_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DVBS2_ENABLE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DVBS2_ENABLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DVBS1_ENABLE_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_DVBS1_ENABLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SCAN_ENABLE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_SCAN_ENABLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_CFR_AUTOSCAN_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_CFR_AUTOSCAN_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_NOFORCE_RELOCK_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_NOFORCE_RELOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TUN_RNG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TUN_RNG_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_S1S2_SEQUENTIAL_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_S1S2_SEQUENTIAL_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_I2C_DEMOD_MODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_I2C_DEMOD_MODE_FIELD: c_int = 5;

pub const STV090x_OFFST_Px_HEADER_MODE_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_HEADER_MODE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_I2C_IRQVAL_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_I2C_IRQVAL_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_FLYWHEEL_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FLYWHEEL_CPT_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DEMOD_CFGMODE_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_DEMOD_CFGMODE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_NOSTOP_FIFOFULL_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_NOSTOP_FIFOFULL_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_CORREL_MANT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CORREL_MANT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CORREL_ABS_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CORREL_ABS_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CORREL_ABSEXP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CORREL_ABSEXP_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_CORREL_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CORREL_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_SPECINV_DEMOD_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_SPECINV_DEMOD_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_PLH_MODCOD_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_PLH_MODCOD_FIELD: c_int = 5;
pub const STV090x_OFFST_Px_PLH_TYPE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PLH_TYPE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_AGC2_REF_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGC2_REF_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_AGC1_ADJUSTED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGC1_ADJUSTED_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_AGC2_INTEGRATOR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_AGC2_INTEGRATOR_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_EN_CAR2CENTER_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_EN_CAR2CENTER_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ROTATON_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_ROTATON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_PH_DET_ALGO_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PH_DET_ALGO_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_CAR_ALPHA_MANT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR_ALPHA_MANT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR_ALPHA_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR_ALPHA_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR_BETA_MANT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR_BETA_MANT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR_BETA_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR_BETA_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_KC_COARSE_EXP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_KC_COARSE_EXP_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_BETA_FREQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_BETA_FREQ_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_FREQ_HDR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FREQ_HDR_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CARLOCK_THRES_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CARLOCK_THRES_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CARLOCK_THRES2_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CARLOCK_THRES2_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NEG_CFRSTEP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NEG_CFRSTEP_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_CFR_UP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CFR_UP_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CFR_LOW_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CFR_LOW_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CFR_INIT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CFR_INIT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CFR_INC1_FIELD: c_int = 0;

pub const STV090x_WIDTH_Px_CFR_INC0_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR_FREQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR_FREQ_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_LOCK_DET_INTEGR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_LOCK_DET_INTEGR_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGLOCK_BETA_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TMGLOCK_BETA_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_DO_TIMING_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DO_TIMING_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TMG_MINFREQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMG_MINFREQ_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_TMGALPHA_EXP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_TMGALPHA_EXP_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_TMGBETA_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGBETA_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_TMGALPHAS2_EXP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_TMGALPHAS2_EXP_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_TMGBETAS2_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGBETAS2_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_TMGLOCK_THRISE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGLOCK_THRISE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGLOCK_THFALL_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGLOCK_THFALL_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SFR_UPRATIO_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFR_UPRATIO_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SFR_LOWRATIO_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFR_LOWRATIO_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_KREF_TMG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_KREF_TMG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SFR_SCANSTEP_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_SFR_SCANSTEP_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_SFR_CENTERSTEP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFR_CENTERSTEP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_SFRRATIO_FINE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFRRATIO_FINE_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_SFR_INIT1_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFR_INIT1_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_SFR_INIT0_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SFR_INIT0_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SYMB_FREQ_UP1_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYMB_FREQ_UP1_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_SYMB_FREQ_UP0_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYMB_FREQ_UP0_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SYMB_FREQ_LOW1_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYMB_FREQ_LOW1_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_SYMB_FREQ_LOW0_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYMB_FREQ_LOW0_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SYMB_FREQ_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYMB_FREQ_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGREG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGREG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGREG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGREG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGREG_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGREG_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TMGLOCK_LEVEL_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TMGLOCK_LEVEL_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ROLLOFF_STATUS_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_ROLLOFF_STATUS_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_EQUAL_ON_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_EQUAL_ON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_MU_EQUALDFE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_MU_EQUALDFE_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_EQUA_ACCIy_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_EQUA_ACCIy_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_EQUA_ACCQy_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_EQUA_ACCQy_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSDATAT_NORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSDATAT_NORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSDATA_NORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSDATA_NORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSPLHT_NORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSPLHT_NORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSPLH_NORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSPLH_NORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSDATAT_UNNORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSDATAT_UNNORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSDATA_UNNORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSDATA_UNNORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSPLHT_UNNORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSPLHT_UNNORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_NOSPLH_UNNORMED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NOSPLH_UNNORMED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_PN4_SELECT_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_PN4_SELECT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_CFR2_STOPDVBS1_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_CFR2_STOPDVBS1_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ROTA2ON_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_ROTA2ON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_PH_DET_ALGO2_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PH_DET_ALGO2_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_CAR2_ALPHA_MANT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2_ALPHA_MANT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2_ALPHA_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2_ALPHA_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2_BETA_MANT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2_BETA_MANT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2_BETA_EXP_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2_BETA_EXP_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_ENAB_SPSKSYMB_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_ENAB_SPSKSYMB_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_CAR2S2_Q_ALPH_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_Q_ALPH_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_Q_ALPH_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_Q_ALPH_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_8_ALPH_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_8_ALPH_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_8_ALPH_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_8_ALPH_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_16A_ALPH_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_16A_ALPH_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_16A_ALPH_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_16A_ALPH_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_32A_ALPH_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_32A_ALPH_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_32A_ALPH_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_32A_ALPH_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_Q_BETA_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_Q_BETA_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_Q_BETA_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_Q_BETA_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_8_BETA_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_8_BETA_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_8_BETA_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_8_BETA_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_16A_BETA_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_16A_BETA_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_16A_BETA_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_16A_BETA_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_CAR2S2_32A_BETA_M_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_CAR2S2_32A_BETA_M_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_CAR2S2_32A_BETA_E_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CAR2S2_32A_BETA_E_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_PLSCRAMB_MODE_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_PLSCRAMB_MODE_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_PLSCRAMB_ROOT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PLSCRAMB_ROOT_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_PLSCRAMB_ROOT1_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PLSCRAMB_ROOT1_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_PLSCRAMB_ROOT0_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PLSCRAMB_ROOT0_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DIS_MODCOD29_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_MODCOD29_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_32PSK_9_10_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_32PSK_9_10_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_32PSK_8_9_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_32PSK_8_9_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_32PSK_5_6_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_32PSK_5_6_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_32PSK_4_5_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_32PSK_4_5_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_32PSK_3_4_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_32PSK_3_4_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_16PSK_9_10_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_16PSK_9_10_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_16PSK_8_9_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_16PSK_8_9_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_16PSK_5_6_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_16PSK_5_6_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_16PSK_4_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_16PSK_4_5_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_16PSK_3_4_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_16PSK_3_4_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_16PSK_2_3_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_16PSK_2_3_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_8P_9_10_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_8P_9_10_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_8P_8_9_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_8P_8_9_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_8P_5_6_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_8P_5_6_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_8P_3_4_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_8P_3_4_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_8P_2_3_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_8P_2_3_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_8P_3_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_8P_3_5_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_9_10_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_9_10_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_QP_8_9_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_QP_8_9_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_5_6_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_5_6_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_QP_4_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_QP_4_5_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_3_4_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_3_4_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_QP_2_3_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_QP_2_3_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_3_5_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_3_5_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_QP_1_2_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_QP_1_2_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_2_5_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_2_5_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DIS_QP_1_3_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_QP_1_3_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DIS_QP_1_4_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DIS_QP_1_4_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_EN_CCIMODE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_EN_CCIMODE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_R0_GAUSSIEN_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_R0_GAUSSIEN_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_CCIDETECT_PLH_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_CCIDETECT_PLH_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_R0_CCI_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_R0_CCI_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_CCI_BETA_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_CCI_BETA_FIELD: c_int = 3;
pub const STV090x_OFFST_Px_CCI_QUANT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CCI_QUANT_FIELD: c_int = 5;

pub const STV090x_OFFST_Px_CCI_THRESHOLD_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CCI_THRESHOLD_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_CCI_VALUE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_CCI_VALUE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DMDRES_RESET_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DMDRES_RESET_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DMDRES_RESNBR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DMDRES_RESNBR_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_DMDRES_DATA_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DMDRES_DATA_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FFE_ACCIy_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FFE_ACCIy_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FFE_ACCQy_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FFE_ACCQy_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_EQUALFFE_ON_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_EQUALFFE_ON_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DIS_QSCALE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DIS_QSCALE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SMAPCOEF_Q_LLR12_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SMAPCOEF_Q_LLR12_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_ADJ_8PSKLLR1_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_ADJ_8PSKLLR1_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_OLD_8PSKLLR1_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_OLD_8PSKLLR1_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DIS_AB8PSK_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DIS_AB8PSK_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DIS_8SCALE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DIS_8SCALE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SMAPCOEF_8P_LLR23_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SMAPCOEF_8P_LLR23_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_PLH_STATISTIC_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PLH_STATISTIC_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DEMOD_LOCKTIME_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DEMOD_LOCKTIME_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TUN_IQSWAP_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TUN_IQSWAP_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_NVTH_NOSRANGE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_NVTH_NOSRANGE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_VERROR_MAXMODE_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_VERROR_MAXMODE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_NSLOWSN_LOCKED_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_NSLOWSN_LOCKED_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DIS_RSFLOCK_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_DIS_RSFLOCK_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_DSS_DVB_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_DSS_DVB_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DSS_SRCH_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DSS_SRCH_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SYNCVIT_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_SYNCVIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_IQINV_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_IQINV_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_VTH12_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH12_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VTH23_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH23_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VTH34_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH34_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VTH56_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH56_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VTH67_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH67_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VTH78_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VTH78_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_VIT_CURPUN_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VIT_CURPUN_FIELD: c_int = 5;

pub const STV090x_OFFST_Px_REGERR_VIT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_REGERR_VIT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DIS_VTHLOCK_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_DIS_VTHLOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E7_8VIT_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_E7_8VIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E6_7VIT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_E6_7VIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E5_6VIT_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_E5_6VIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E3_4VIT_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_E3_4VIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E2_3VIT_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_E2_3VIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_E1_2VIT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_E1_2VIT_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_SNVIT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_SNVIT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_TOVVIT_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_TOVVIT_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_HYPVIT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_HYPVIT_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_PRFVIT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_PRFVIT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_LOCKEDVIT_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_LOCKEDVIT_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_VIT_INUSE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_VIT_INUSE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_K_DIVIDER_12_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_12_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_K_DIVIDER_23_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_23_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_K_DIVIDER_34_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_34_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_K_DIVIDER_56_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_56_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_K_DIVIDER_67_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_67_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_K_DIVIDER_78_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_K_DIVIDER_78_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_INV_MISMASK_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_INV_MISMASK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FILTER_EN_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_FILTER_EN_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_EN_MIS00_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_EN_MIS00_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ALGOSWRST_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ALGOSWRST_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_FORCE_CONTINUOUS: c_int = 7;
pub const STV090x_WIDTH_Px_FORCE_CONTINUOUS: c_int = 1;
pub const STV090x_OFFST_Px_RESET_UPKO_COUNT: c_int = 6;
pub const STV090x_WIDTH_Px_RESET_UPKO_COUNT: c_int = 1;
pub const STV090x_OFFST_Px_USER_PKTDELIN_NB: c_int = 5;
pub const STV090x_WIDTH_Px_USER_PKTDELIN_NB: c_int = 1;
pub const STV090x_OFFST_Px_FORCE_LOCKED: c_int = 4;
pub const STV090x_WIDTH_Px_FORCE_LOCKED: c_int = 1;
pub const STV090x_OFFST_Px_DATA_UNBBSCRAM: c_int = 3;
pub const STV090x_WIDTH_Px_DATA_UNBBSCRAM: c_int = 1;
pub const STV090x_OFFST_Px_FORCE_LONGPACKET: c_int = 2;
pub const STV090x_WIDTH_Px_FORCE_LONGPACKET: c_int = 1;
pub const STV090x_OFFST_Px_FRAME_MODE_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_FRAME_MODE_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_UNLCK_THRESH_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_UNLCK_THRESH_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_DELIN_LCK_THRESH_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DELIN_LCK_THRESH_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_ISI_ENTRY_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ISI_ENTRY_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ISI_BIT_EN_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ISI_BIT_EN_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_MATYPE_CURRENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_MATYPE_CURRENT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_UPL_CURRENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_UPL_CURRENT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_DFL_CURRENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_DFL_CURRENT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SYNC_CURRENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYNC_CURRENT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SYNCD_CURRENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SYNCD_CURRENT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_PKTDELIN_LOCK_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_PKTDELIN_LOCK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FIRST_LOCK_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FIRST_LOCK_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_FRAME_MODCOD_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_FRAME_MODCOD_FIELD: c_int = 5;
pub const STV090x_OFFST_Px_FRAME_TYPE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FRAME_TYPE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_BBHCRC_KOCNT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_BBHCRC_KOCNT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_BBHCRC_KOCNT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_BBHCRC_KOCNT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_PKTCRC_KOCNT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PKTCRC_KOCNT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_PKTCRC_KOCNT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_PKTCRC_KOCNT_FIELD: c_int = 8;

pub const STV090x_NBITERNOERR: c_uint = 0xFA3F;
pub const STV090x_OFFST_NBITER_STOP_CRIT_FIELD: c_int = 0;
pub const STV090x_WIDTH_NBITER_STOP_CRIT_FIELD: c_int = 4;

pub const STV090x_OFFST_GAINLLR_NF_QP_1_2_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_1_2_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_3_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_3_5_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_2_3_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_2_3_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_3_4_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_3_4_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_4_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_4_5_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_5_6_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_5_6_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_8_9_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_8_9_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_QP_9_10_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_QP_9_10_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_3_5_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_3_5_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_2_3_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_2_3_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_3_4_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_3_4_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_5_6_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_5_6_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_8_9_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_8_9_FIELD: c_int = 7;

pub const STV090x_OFFST_GAINLLR_NF_8P_9_10_FIELD: c_int = 0;
pub const STV090x_WIDTH_GAINLLR_NF_8P_9_10_FIELD: c_int = 7;
pub const STV090x_GENCFG: c_uint = 0xFA86;
pub const STV090x_OFFST_BROADCAST_FIELD: c_int = 4;
pub const STV090x_WIDTH_BROADCAST_FIELD: c_int = 1;
pub const STV090x_OFFST_PRIORITY_FIELD: c_int = 1;
pub const STV090x_WIDTH_PRIORITY_FIELD: c_int = 1;
pub const STV090x_OFFST_DDEMOD_FIELD: c_int = 0;
pub const STV090x_WIDTH_DDEMOD_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_LDPC_ERRORS_COUNTER_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_LDPC_ERRORS_COUNTER_FIELD: c_int = 8;
pub const STV090x_BCHERR: c_uint = 0xFA98;
pub const STV090x_OFFST_Px_ERRORFLAG_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_ERRORFLAG_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_BCH_ERRORS_COUNTER_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_BCH_ERRORS_COUNTER_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_TSDIL_ON_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TSDIL_ON_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSRS_ON_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_TSRS_ON_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_DVBCI_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TSFIFO_DVBCI_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_SERIAL_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSFIFO_SERIAL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_TEIUPDATE_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_TSFIFO_TEIUPDATE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_DUTY50_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_TSFIFO_DUTY50_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_HSGNLOUT_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_TSFIFO_HSGNLOUT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_ERRORMODE_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_TSFIFO_ERRORMODE_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_RST_HWARE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_RST_HWARE_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_MANSPEED_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSFIFO_MANSPEED_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_TSFIFO_PERMDATA_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_TSFIFO_PERMDATA_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_INVDATA_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TSFIFO_INVDATA_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_BCLKDEL1CK_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSFIFO_BCLKDEL1CK_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_BCHERROR_MODE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_BCHERROR_MODE_FIELD: c_int = 2;
pub const STV090x_OFFST_Px_TSFIFO_NSGNL2DATA_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_TSFIFO_NSGNL2DATA_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_EMBINDVB_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_TSFIFO_EMBINDVB_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_DPUNACT_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_TSFIFO_DPUNACT_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSDEL_SYNCBYTE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TSDEL_SYNCBYTE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSDEL_XXHEADER_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSDEL_XXHEADER_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_OUTSPEED_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TSFIFO_OUTSPEED_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_TSFIFO_LINEOK_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TSFIFO_LINEOK_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_ERROR_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSFIFO_ERROR_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_DEMODSEL_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_TSFIFO_DEMODSEL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFOSPEED_STORE_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_TSFIFOSPEED_STORE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DILXX_RESET_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_DILXX_RESET_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSSERIAL_IMPOS_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_TSSERIAL_IMPOS_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SCRAMBDETECT_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_SCRAMBDETECT_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSFIFO_BITRATE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_TSFIFO_BITRATE_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ERR_SOURCE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_ERR_SOURCE_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_NUM_EVENT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NUM_EVENT_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_ERRCNT1_OLDVALUE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_ERRCNT1_OLDVALUE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ERR_CNT12_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT12_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_ERR_CNT11_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT11_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ERR_CNT10_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT10_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ERR_SOURCE2_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_ERR_SOURCE2_FIELD: c_int = 4;
pub const STV090x_OFFST_Px_NUM_EVENT2_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_NUM_EVENT2_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_ERRCNT2_OLDVALUE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_ERRCNT2_OLDVALUE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ERR_CNT2_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT2_FIELD: c_int = 7;

pub const STV090x_OFFST_Px_ERR_CNT21_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT21_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_ERR_CNT20_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_ERR_CNT20_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_SPY_ENABLE_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_SPY_ENABLE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_BERMETER_DATAMAODE_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_BERMETER_DATAMAODE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_RST_ON_ERROR_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_RST_ON_ERROR_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_ONE_SHOT_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_ONE_SHOT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_I2C_MODE_FIELD: c_int = 2;
pub const STV090x_WIDTH_Px_I2C_MODE_FIELD: c_int = 2;

pub const STV090x_OFFST_Px_SPY_STUFFING_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_SPY_STUFFING_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SPY_CNULLPKT_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_SPY_CNULLPKT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_SPY_OUTDATA_MODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_SPY_OUTDATA_MODE_FIELD: c_int = 5;

pub const STV090x_OFFST_Px_FSPY_DIRECT_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_FSPY_DIRECT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_STUFF_MODE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_STUFF_MODE_FIELD: c_int = 3;

pub const STV090x_OFFST_Px_SPY_ENDSIM_FIELD: c_int = 7;
pub const STV090x_WIDTH_Px_SPY_ENDSIM_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_VALID_SIM_FIELD: c_int = 6;
pub const STV090x_WIDTH_Px_VALID_SIM_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FOUND_SIGNAL_FIELD: c_int = 5;
pub const STV090x_WIDTH_Px_FOUND_SIGNAL_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_DSS_SYNCBYTE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_DSS_SYNCBYTE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_RESULT_STATE_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_RESULT_STATE_FIELD: c_int = 4;

pub const STV090x_OFFST_Px_FBERMETER_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FBERMETER_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FBERMETER_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FBERMETER_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FBERMETER_CPT_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FBERMETER_CPT_ERR_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FBERMETER_CPT_ERR_FIELD: c_int = 8;

pub const STV090x_OFFST_Px_FSPYBER_SYNCBYTE_FIELD: c_int = 4;
pub const STV090x_WIDTH_Px_FSPYBER_SYNCBYTE_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FSPYBER_UNSYNC_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_FSPYBER_UNSYNC_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_FSPYBER_CTIME_FIELD: c_int = 0;
pub const STV090x_WIDTH_Px_FSPYBER_CTIME_FIELD: c_int = 3;
pub const STV090x_RCCFGH: c_uint = 0xf600;
pub const STV090x_TSGENERAL: c_uint = 0xF630;
pub const STV090x_OFFST_Px_MUXSTREAM_OUT_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_MUXSTREAM_OUT_FIELD: c_int = 1;
pub const STV090x_OFFST_Px_TSFIFO_PERMPARAL_FIELD: c_int = 1;
pub const STV090x_WIDTH_Px_TSFIFO_PERMPARAL_FIELD: c_int = 2;
pub const STV090x_TSGENERAL1X: c_uint = 0xf670;
pub const STV090x_CFGEXT: c_uint = 0xfa80;
pub const STV090x_TSTRES0: c_uint = 0xFF11;
pub const STV090x_OFFST_FRESFEC_FIELD: c_int = 7;
pub const STV090x_WIDTH_FRESFEC_FIELD: c_int = 1;

pub const STV090x_OFFST_Px_TSTDISRX_SELECT_FIELD: c_int = 3;
pub const STV090x_WIDTH_Px_TSTDISRX_SELECT_FIELD: c_int = 1;
