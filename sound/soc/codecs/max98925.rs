//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98925.h
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
// max98925.h -- MAX98925 ALSA SoC Audio driver
//
// Copyright 2013-2015 Maxim Integrated Products
//
pub const MAX98925_VERSION: c_uint = 0x51;
pub const MAX98925_VERSION1: c_uint = 0x80;
pub const MAX98925_VBAT_DATA: c_uint = 0x00;
pub const MAX98925_VBST_DATA: c_uint = 0x01;
pub const MAX98925_LIVE_STATUS0: c_uint = 0x02;
pub const MAX98925_LIVE_STATUS1: c_uint = 0x03;
pub const MAX98925_LIVE_STATUS2: c_uint = 0x04;
pub const MAX98925_STATE0: c_uint = 0x05;
pub const MAX98925_STATE1: c_uint = 0x06;
pub const MAX98925_STATE2: c_uint = 0x07;
pub const MAX98925_FLAG0: c_uint = 0x08;
pub const MAX98925_FLAG1: c_uint = 0x09;
pub const MAX98925_FLAG2: c_uint = 0x0A;
pub const MAX98925_IRQ_ENABLE0: c_uint = 0x0B;
pub const MAX98925_IRQ_ENABLE1: c_uint = 0x0C;
pub const MAX98925_IRQ_ENABLE2: c_uint = 0x0D;
pub const MAX98925_IRQ_CLEAR0: c_uint = 0x0E;
pub const MAX98925_IRQ_CLEAR1: c_uint = 0x0F;
pub const MAX98925_IRQ_CLEAR2: c_uint = 0x10;
pub const MAX98925_MAP0: c_uint = 0x11;
pub const MAX98925_MAP1: c_uint = 0x12;
pub const MAX98925_MAP2: c_uint = 0x13;
pub const MAX98925_MAP3: c_uint = 0x14;
pub const MAX98925_MAP4: c_uint = 0x15;
pub const MAX98925_MAP5: c_uint = 0x16;
pub const MAX98925_MAP6: c_uint = 0x17;
pub const MAX98925_MAP7: c_uint = 0x18;
pub const MAX98925_MAP8: c_uint = 0x19;
pub const MAX98925_DAI_CLK_MODE1: c_uint = 0x1A;
pub const MAX98925_DAI_CLK_MODE2: c_uint = 0x1B;
pub const MAX98925_DAI_CLK_DIV_M_MSBS: c_uint = 0x1C;
pub const MAX98925_DAI_CLK_DIV_M_LSBS: c_uint = 0x1D;
pub const MAX98925_DAI_CLK_DIV_N_MSBS: c_uint = 0x1E;
pub const MAX98925_DAI_CLK_DIV_N_LSBS: c_uint = 0x1F;
pub const MAX98925_FORMAT: c_uint = 0x20;
pub const MAX98925_TDM_SLOT_SELECT: c_uint = 0x21;
pub const MAX98925_DOUT_CFG_VMON: c_uint = 0x22;
pub const MAX98925_DOUT_CFG_IMON: c_uint = 0x23;
pub const MAX98925_DOUT_CFG_VBAT: c_uint = 0x24;
pub const MAX98925_DOUT_CFG_VBST: c_uint = 0x25;
pub const MAX98925_DOUT_CFG_FLAG: c_uint = 0x26;
pub const MAX98925_DOUT_HIZ_CFG1: c_uint = 0x27;
pub const MAX98925_DOUT_HIZ_CFG2: c_uint = 0x28;
pub const MAX98925_DOUT_HIZ_CFG3: c_uint = 0x29;
pub const MAX98925_DOUT_HIZ_CFG4: c_uint = 0x2A;
pub const MAX98925_DOUT_DRV_STRENGTH: c_uint = 0x2B;
pub const MAX98925_FILTERS: c_uint = 0x2C;
pub const MAX98925_GAIN: c_uint = 0x2D;
pub const MAX98925_GAIN_RAMPING: c_uint = 0x2E;
pub const MAX98925_SPK_AMP: c_uint = 0x2F;
pub const MAX98925_THRESHOLD: c_uint = 0x30;
pub const MAX98925_ALC_ATTACK: c_uint = 0x31;
pub const MAX98925_ALC_ATTEN_RLS: c_uint = 0x32;
pub const MAX98925_ALC_HOLD_RLS: c_uint = 0x33;
pub const MAX98925_ALC_CONFIGURATION: c_uint = 0x34;
pub const MAX98925_BOOST_CONVERTER: c_uint = 0x35;
pub const MAX98925_BLOCK_ENABLE: c_uint = 0x36;
pub const MAX98925_CONFIGURATION: c_uint = 0x37;
pub const MAX98925_GLOBAL_ENABLE: c_uint = 0x38;
pub const MAX98925_BOOST_LIMITER: c_uint = 0x3A;
pub const MAX98925_REV_VERSION: c_uint = 0xFF;

// MAX98925 Register Bit Fields
// MAX98925_R002_LIVE_STATUS0

pub const M98925_THERMWARN_STATUS_SHIFT: c_int = 3;
pub const M98925_THERMWARN_STATUS_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_STATUS_SHIFT: c_int = 1;
pub const M98925_THERMSHDN_STATUS_WIDTH: c_int = 1;
// MAX98925_R003_LIVE_STATUS1

pub const M98925_SPKCURNT_STATUS_SHIFT: c_int = 5;
pub const M98925_SPKCURNT_STATUS_WIDTH: c_int = 1;

pub const M98925_WATCHFAIL_STATUS_SHIFT: c_int = 4;
pub const M98925_WATCHFAIL_STATUS_WIDTH: c_int = 1;

pub const M98925_ALCINFH_STATUS_SHIFT: c_int = 3;
pub const M98925_ALCINFH_STATUS_WIDTH: c_int = 1;

pub const M98925_ALCACT_STATUS_SHIFT: c_int = 2;
pub const M98925_ALCACT_STATUS_WIDTH: c_int = 1;

pub const M98925_ALCMUT_STATUS_SHIFT: c_int = 1;
pub const M98925_ALCMUT_STATUS_WIDTH: c_int = 1;

pub const M98925_ACLP_STATUS_SHIFT: c_int = 0;
pub const M98925_ACLP_STATUS_WIDTH: c_int = 1;
// MAX98925_R004_LIVE_STATUS2

pub const M98925_SLOTOVRN_STATUS_SHIFT: c_int = 6;
pub const M98925_SLOTOVRN_STATUS_WIDTH: c_int = 1;

pub const M98925_INVALSLOT_STATUS_SHIFT: c_int = 5;
pub const M98925_INVALSLOT_STATUS_WIDTH: c_int = 1;

pub const M98925_SLOTCNFLT_STATUS_SHIFT: c_int = 4;
pub const M98925_SLOTCNFLT_STATUS_WIDTH: c_int = 1;

pub const M98925_VBSTOVFL_STATUS_SHIFT: c_int = 3;
pub const M98925_VBSTOVFL_STATUS_WIDTH: c_int = 1;

pub const M98925_VBATOVFL_STATUS_SHIFT: c_int = 2;
pub const M98925_VBATOVFL_STATUS_WIDTH: c_int = 1;

pub const M98925_IMONOVFL_STATUS_SHIFT: c_int = 1;
pub const M98925_IMONOVFL_STATUS_WIDTH: c_int = 1;

pub const M98925_VMONOVFL_STATUS_SHIFT: c_int = 0;
pub const M98925_VMONOVFL_STATUS_WIDTH: c_int = 1;
// MAX98925_R005_STATE0

pub const M98925_THERMWARN_END_STATE_SHIFT: c_int = 3;
pub const M98925_THERMWARN_END_STATE_WIDTH: c_int = 1;

pub const M98925_THERMWARN_BGN_STATE_SHIFT: c_int = 1;
pub const M98925_THERMWARN_BGN_STATE_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_END_STATE_SHIFT: c_int = 1;
pub const M98925_THERMSHDN_END_STATE_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_BGN_STATE_SHIFT: c_int = 0;
pub const M98925_THERMSHDN_BGN_STATE_WIDTH: c_int = 1;
// MAX98925_R006_STATE1

pub const M98925_SPRCURNT_STATE_SHIFT: c_int = 5;
pub const M98925_SPRCURNT_STATE_WIDTH: c_int = 1;

pub const M98925_WATCHFAIL_STATE_SHIFT: c_int = 4;
pub const M98925_WATCHFAIL_STATE_WIDTH: c_int = 1;

pub const M98925_ALCINFH_STATE_SHIFT: c_int = 3;
pub const M98925_ALCINFH_STATE_WIDTH: c_int = 1;

pub const M98925_ALCACT_STATE_SHIFT: c_int = 2;
pub const M98925_ALCACT_STATE_WIDTH: c_int = 1;

pub const M98925_ALCMUT_STATE_SHIFT: c_int = 1;
pub const M98925_ALCMUT_STATE_WIDTH: c_int = 1;

pub const M98925_ALCP_STATE_SHIFT: c_int = 0;
pub const M98925_ALCP_STATE_WIDTH: c_int = 1;
// MAX98925_R007_STATE2

pub const M98925_SLOTOVRN_STATE_SHIFT: c_int = 6;
pub const M98925_SLOTOVRN_STATE_WIDTH: c_int = 1;

pub const M98925_INVALSLOT_STATE_SHIFT: c_int = 5;
pub const M98925_INVALSLOT_STATE_WIDTH: c_int = 1;

pub const M98925_SLOTCNFLT_STATE_SHIFT: c_int = 4;
pub const M98925_SLOTCNFLT_STATE_WIDTH: c_int = 1;

pub const M98925_VBSTOVFL_STATE_SHIFT: c_int = 3;
pub const M98925_VBSTOVFL_STATE_WIDTH: c_int = 1;

pub const M98925_VBATOVFL_STATE_SHIFT: c_int = 2;
pub const M98925_VBATOVFL_STATE_WIDTH: c_int = 1;

pub const M98925_IMONOVFL_STATE_SHIFT: c_int = 1;
pub const M98925_IMONOVFL_STATE_WIDTH: c_int = 1;

pub const M98925_VMONOVFL_STATE_SHIFT: c_int = 0;
pub const M98925_VMONOVFL_STATE_WIDTH: c_int = 1;
// MAX98925_R008_FLAG0

pub const M98925_THERMWARN_END_FLAG_SHIFT: c_int = 3;
pub const M98925_THERMWARN_END_FLAG_WIDTH: c_int = 1;

pub const M98925_THERMWARN_BGN_FLAG_SHIFT: c_int = 2;
pub const M98925_THERMWARN_BGN_FLAG_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_END_FLAG_SHIFT: c_int = 1;
pub const M98925_THERMSHDN_END_FLAG_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_BGN_FLAG_SHIFT: c_int = 0;
pub const M98925_THERMSHDN_BGN_FLAG_WIDTH: c_int = 1;
// MAX98925_R009_FLAG1

pub const M98925_SPKCURNT_FLAG_SHIFT: c_int = 5;
pub const M98925_SPKCURNT_FLAG_WIDTH: c_int = 1;

pub const M98925_WATCHFAIL_FLAG_SHIFT: c_int = 4;
pub const M98925_WATCHFAIL_FLAG_WIDTH: c_int = 1;

pub const M98925_ALCINFH_FLAG_SHIFT: c_int = 3;
pub const M98925_ALCINFH_FLAG_WIDTH: c_int = 1;

pub const M98925_ALCACT_FLAG_SHIFT: c_int = 2;
pub const M98925_ALCACT_FLAG_WIDTH: c_int = 1;

pub const M98925_ALCMUT_FLAG_SHIFT: c_int = 1;
pub const M98925_ALCMUT_FLAG_WIDTH: c_int = 1;

pub const M98925_ALCP_FLAG_SHIFT: c_int = 0;
pub const M98925_ALCP_FLAG_WIDTH: c_int = 1;
// MAX98925_R00A_FLAG2

pub const M98925_SLOTOVRN_FLAG_SHIFT: c_int = 6;
pub const M98925_SLOTOVRN_FLAG_WIDTH: c_int = 1;

pub const M98925_INVALSLOT_FLAG_SHIFT: c_int = 5;
pub const M98925_INVALSLOT_FLAG_WIDTH: c_int = 1;

pub const M98925_SLOTCNFLT_FLAG_SHIFT: c_int = 4;
pub const M98925_SLOTCNFLT_FLAG_WIDTH: c_int = 1;

pub const M98925_VBSTOVFL_FLAG_SHIFT: c_int = 3;
pub const M98925_VBSTOVFL_FLAG_WIDTH: c_int = 1;

pub const M98925_VBATOVFL_FLAG_SHIFT: c_int = 2;
pub const M98925_VBATOVFL_FLAG_WIDTH: c_int = 1;

pub const M98925_IMONOVFL_FLAG_SHIFT: c_int = 1;
pub const M98925_IMONOVFL_FLAG_WIDTH: c_int = 1;

pub const M98925_VMONOVFL_FLAG_SHIFT: c_int = 0;
pub const M98925_VMONOVFL_FLAG_WIDTH: c_int = 1;
// MAX98925_R00B_IRQ_ENABLE0

pub const M98925_THERMWARN_END_EN_SHIFT: c_int = 3;
pub const M98925_THERMWARN_END_EN_WIDTH: c_int = 1;

pub const M98925_THERMWARN_BGN_EN_SHIFT: c_int = 2;
pub const M98925_THERMWARN_BGN_EN_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_END_EN_SHIFT: c_int = 1;
pub const M98925_THERMSHDN_END_EN_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_BGN_EN_SHIFT: c_int = 0;
pub const M98925_THERMSHDN_BGN_EN_WIDTH: c_int = 1;
// MAX98925_R00C_IRQ_ENABLE1

pub const M98925_SPKCURNT_EN_SHIFT: c_int = 5;
pub const M98925_SPKCURNT_EN_WIDTH: c_int = 1;

pub const M98925_WATCHFAIL_EN_SHIFT: c_int = 4;
pub const M98925_WATCHFAIL_EN_WIDTH: c_int = 1;

pub const M98925_ALCINFH_EN_SHIFT: c_int = 3;
pub const M98925_ALCINFH_EN_WIDTH: c_int = 1;

pub const M98925_ALCACT_EN_SHIFT: c_int = 2;
pub const M98925_ALCACT_EN_WIDTH: c_int = 1;

pub const M98925_ALCMUT_EN_SHIFT: c_int = 1;
pub const M98925_ALCMUT_EN_WIDTH: c_int = 1;

pub const M98925_ALCP_EN_SHIFT: c_int = 0;
pub const M98925_ALCP_EN_WIDTH: c_int = 1;
// MAX98925_R00D_IRQ_ENABLE2

pub const M98925_SLOTOVRN_EN_SHIFT: c_int = 6;
pub const M98925_SLOTOVRN_EN_WIDTH: c_int = 1;

pub const M98925_INVALSLOT_EN_SHIFT: c_int = 5;
pub const M98925_INVALSLOT_EN_WIDTH: c_int = 1;

pub const M98925_SLOTCNFLT_EN_SHIFT: c_int = 4;
pub const M98925_SLOTCNFLT_EN_WIDTH: c_int = 1;

pub const M98925_VBSTOVFL_EN_SHIFT: c_int = 3;
pub const M98925_VBSTOVFL_EN_WIDTH: c_int = 1;

pub const M98925_VBATOVFL_EN_SHIFT: c_int = 2;
pub const M98925_VBATOVFL_EN_WIDTH: c_int = 1;

pub const M98925_IMONOVFL_EN_SHIFT: c_int = 1;
pub const M98925_IMONOVFL_EN_WIDTH: c_int = 1;

pub const M98925_VMONOVFL_EN_SHIFT: c_int = 0;
pub const M98925_VMONOVFL_EN_WIDTH: c_int = 1;
// MAX98925_R00E_IRQ_CLEAR0

pub const M98925_THERMWARN_END_CLR_SHIFT: c_int = 3;
pub const M98925_THERMWARN_END_CLR_WIDTH: c_int = 1;

pub const M98925_THERMWARN_BGN_CLR_SHIFT: c_int = 2;
pub const M98925_THERMWARN_BGN_CLR_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_END_CLR_SHIFT: c_int = 1;
pub const M98925_THERMSHDN_END_CLR_WIDTH: c_int = 1;

pub const M98925_THERMSHDN_BGN_CLR_SHIFT: c_int = 0;
pub const M98925_THERMSHDN_BGN_CLR_WIDTH: c_int = 1;
// MAX98925_R00F_IRQ_CLEAR1

pub const M98925_SPKCURNT_CLR_SHIFT: c_int = 5;
pub const M98925_SPKCURNT_CLR_WIDTH: c_int = 1;

pub const M98925_WATCHFAIL_CLR_SHIFT: c_int = 4;
pub const M98925_WATCHFAIL_CLR_WIDTH: c_int = 1;

pub const M98925_ALCINFH_CLR_SHIFT: c_int = 3;
pub const M98925_ALCINFH_CLR_WIDTH: c_int = 1;

pub const M98925_ALCACT_CLR_SHIFT: c_int = 2;
pub const M98925_ALCACT_CLR_WIDTH: c_int = 1;

pub const M98925_ALCMUT_CLR_SHIFT: c_int = 1;
pub const M98925_ALCMUT_CLR_WIDTH: c_int = 1;

pub const M98925_ALCP_CLR_SHIFT: c_int = 0;
pub const M98925_ALCP_CLR_WIDTH: c_int = 1;
// MAX98925_R010_IRQ_CLEAR2

pub const M98925_SLOTOVRN_CLR_SHIFT: c_int = 6;
pub const M98925_SLOTOVRN_CLR_WIDTH: c_int = 1;

pub const M98925_INVALSLOT_CLR_SHIFT: c_int = 5;
pub const M98925_INVALSLOT_CLR_WIDTH: c_int = 1;

pub const M98925_SLOTCNFLT_CLR_SHIFT: c_int = 4;
pub const M98925_SLOTCNFLT_CLR_WIDTH: c_int = 1;

pub const M98925_VBSTOVFL_CLR_SHIFT: c_int = 3;
pub const M98925_VBSTOVFL_CLR_WIDTH: c_int = 1;

pub const M98925_VBATOVFL_CLR_SHIFT: c_int = 2;
pub const M98925_VBATOVFL_CLR_WIDTH: c_int = 1;

pub const M98925_IMONOVFL_CLR_SHIFT: c_int = 1;
pub const M98925_IMONOVFL_CLR_WIDTH: c_int = 1;

pub const M98925_VMONOVFL_CLR_SHIFT: c_int = 0;
pub const M98925_VMONOVFL_CLR_WIDTH: c_int = 1;
// MAX98925_R011_MAP0

pub const M98925_ER_THERMWARN_EN_SHIFT: c_int = 7;
pub const M98925_ER_THERMWARN_EN_WIDTH: c_int = 1;

pub const M98925_ER_THERMWARN_MAP_SHIFT: c_int = 4;
pub const M98925_ER_THERMWARN_MAP_WIDTH: c_int = 3;
// MAX98925_R012_MAP1

pub const M98925_ER_ALCMUT_EN_SHIFT: c_int = 7;
pub const M98925_ER_ALCMUT_EN_WIDTH: c_int = 1;

pub const M98925_ER_ALCMUT_MAP_SHIFT: c_int = 4;
pub const M98925_ER_ALCMUT_MAP_WIDTH: c_int = 3;

pub const M98925_ER_ALCP_EN_SHIFT: c_int = 3;
pub const M98925_ER_ALCP_EN_WIDTH: c_int = 1;

pub const M98925_ER_ALCP_MAP_SHIFT: c_int = 0;
pub const M98925_ER_ALCP_MAP_WIDTH: c_int = 3;
// MAX98925_R013_MAP2

pub const M98925_ER_ALCINFH_EN_SHIFT: c_int = 7;
pub const M98925_ER_ALCINFH_EN_WIDTH: c_int = 1;

pub const M98925_ER_ALCINFH_MAP_SHIFT: c_int = 4;
pub const M98925_ER_ALCINFH_MAP_WIDTH: c_int = 3;

pub const M98925_ER_ALCACT_EN_SHIFT: c_int = 3;
pub const M98925_ER_ALCACT_EN_WIDTH: c_int = 1;

pub const M98925_ER_ALCACT_MAP_SHIFT: c_int = 0;
pub const M98925_ER_ALCACT_MAP_WIDTH: c_int = 3;
// MAX98925_R014_MAP3

pub const M98925_ER_SPKCURNT_EN_SHIFT: c_int = 7;
pub const M98925_ER_SPKCURNT_EN_WIDTH: c_int = 1;

pub const M98925_ER_SPKCURNT_MAP_SHIFT: c_int = 4;
pub const M98925_ER_SPKCURNT_MAP_WIDTH: c_int = 3;
// MAX98925_R015_MAP4
// RESERVED
// MAX98925_R016_MAP5

pub const M98925_ER_IMONOVFL_EN_SHIFT: c_int = 7;
pub const M98925_ER_IMONOVFL_EN_WIDTH: c_int = 1;

pub const M98925_ER_IMONOVFL_MAP_SHIFT: c_int = 4;
pub const M98925_ER_IMONOVFL_MAP_WIDTH: c_int = 3;

pub const M98925_ER_VMONOVFL_EN_SHIFT: c_int = 3;
pub const M98925_ER_VMONOVFL_EN_WIDTH: c_int = 1;

pub const M98925_ER_VMONOVFL_MAP_SHIFT: c_int = 0;
pub const M98925_ER_VMONOVFL_MAP_WIDTH: c_int = 3;
// MAX98925_R017_MAP6

pub const M98925_ER_VBSTOVFL_EN_SHIFT: c_int = 7;
pub const M98925_ER_VBSTOVFL_EN_WIDTH: c_int = 1;

pub const M98925_ER_VBSTOVFL_MAP_SHIFT: c_int = 4;
pub const M98925_ER_VBSTOVFL_MAP_WIDTH: c_int = 3;

pub const M98925_ER_VBATOVFL_EN_SHIFT: c_int = 3;
pub const M98925_ER_VBATOVFL_EN_WIDTH: c_int = 1;

pub const M98925_ER_VBATOVFL_MAP_SHIFT: c_int = 0;
pub const M98925_ER_VBATOVFL_MAP_WIDTH: c_int = 3;
// MAX98925_R018_MAP7

pub const M98925_ER_INVALSLOT_EN_SHIFT: c_int = 7;
pub const M98925_ER_INVALSLOT_EN_WIDTH: c_int = 1;

pub const M98925_ER_INVALSLOT_MAP_SHIFT: c_int = 4;
pub const M98925_ER_INVALSLOT_MAP_WIDTH: c_int = 3;

pub const M98925_ER_SLOTCNFLT_EN_SHIFT: c_int = 3;
pub const M98925_ER_SLOTCNFLT_EN_WIDTH: c_int = 1;

pub const M98925_ER_SLOTCNFLT_MAP_SHIFT: c_int = 0;
pub const M98925_ER_SLOTCNFLT_MAP_WIDTH: c_int = 3;
// MAX98925_R019_MAP8

pub const M98925_ER_SLOTOVRN_EN_SHIFT: c_int = 3;
pub const M98925_ER_SLOTOVRN_EN_WIDTH: c_int = 1;

pub const M98925_ER_SLOTOVRN_MAP_SHIFT: c_int = 0;
pub const M98925_ER_SLOTOVRN_MAP_WIDTH: c_int = 3;
// MAX98925_R01A_DAI_CLK_MODE1

pub const M98925_DAI_CLK_SOURCE_SHIFT: c_int = 6;
pub const M98925_DAI_CLK_SOURCE_WIDTH: c_int = 1;

pub const M98925_MDLL_MULT_SHIFT: c_int = 0;
pub const M98925_MDLL_MULT_WIDTH: c_int = 4;
pub const M98925_MDLL_MULT_MCLKx8: c_int = 6;
pub const M98925_MDLL_MULT_MCLKx16: c_int = 8;
// MAX98925_R01B_DAI_CLK_MODE2

pub const M98925_DAI_SR_SHIFT: c_int = 4;
pub const M98925_DAI_SR_WIDTH: c_int = 4;

pub const M98925_DAI_MAS_SHIFT: c_int = 3;
pub const M98925_DAI_MAS_WIDTH: c_int = 1;

pub const M98925_DAI_BSEL_SHIFT: c_int = 0;
pub const M98925_DAI_BSEL_WIDTH: c_int = 3;

// MAX98925_R01C_DAI_CLK_DIV_M_MSBS

pub const M98925_DAI_M_MSBS_SHIFT: c_int = 0;
pub const M98925_DAI_M_MSBS_WIDTH: c_int = 8;
// MAX98925_R01D_DAI_CLK_DIV_M_LSBS

pub const M98925_DAI_M_LSBS_SHIFT: c_int = 0;
pub const M98925_DAI_M_LSBS_WIDTH: c_int = 8;
// MAX98925_R01E_DAI_CLK_DIV_N_MSBS

pub const M98925_DAI_N_MSBS_SHIFT: c_int = 0;
pub const M98925_DAI_N_MSBS_WIDTH: c_int = 7;
// MAX98925_R01F_DAI_CLK_DIV_N_LSBS

pub const M98925_DAI_N_LSBS_SHIFT: c_int = 0;
pub const M98925_DAI_N_LSBS_WIDTH: c_int = 8;
// MAX98925_R020_FORMAT

pub const M98925_DAI_CHANSZ_SHIFT: c_int = 6;
pub const M98925_DAI_CHANSZ_WIDTH: c_int = 2;

pub const M98925_DAI_EXTBCLK_HIZ_SHIFT: c_int = 4;
pub const M98925_DAI_EXTBCLK_HIZ_WIDTH: c_int = 1;

pub const M98925_DAI_WCI_SHIFT: c_int = 3;
pub const M98925_DAI_WCI_WIDTH: c_int = 1;

pub const M98925_DAI_BCI_SHIFT: c_int = 2;
pub const M98925_DAI_BCI_WIDTH: c_int = 1;

pub const M98925_DAI_DLY_SHIFT: c_int = 1;
pub const M98925_DAI_DLY_WIDTH: c_int = 1;

pub const M98925_DAI_TDM_SHIFT: c_int = 0;
pub const M98925_DAI_TDM_WIDTH: c_int = 1;

// MAX98925_R021_TDM_SLOT_SELECT

pub const M98925_DAI_DO_EN_SHIFT: c_int = 7;
pub const M98925_DAI_DO_EN_WIDTH: c_int = 1;

pub const M98925_DAI_DIN_EN_SHIFT: c_int = 6;
pub const M98925_DAI_DIN_EN_WIDTH: c_int = 1;

pub const M98925_DAI_INR_SOURCE_SHIFT: c_int = 3;
pub const M98925_DAI_INR_SOURCE_WIDTH: c_int = 3;

pub const M98925_DAI_INL_SOURCE_SHIFT: c_int = 0;
pub const M98925_DAI_INL_SOURCE_WIDTH: c_int = 3;
// MAX98925_R022_DOUT_CFG_VMON

pub const M98925_DAI_VMON_EN_SHIFT: c_int = 5;
pub const M98925_DAI_VMON_EN_WIDTH: c_int = 1;

pub const M98925_DAI_VMON_SLOT_SHIFT: c_int = 0;
pub const M98925_DAI_VMON_SLOT_WIDTH: c_int = 5;

// MAX98925_R023_DOUT_CFG_IMON

pub const M98925_DAI_IMON_EN_SHIFT: c_int = 5;
pub const M98925_DAI_IMON_EN_WIDTH: c_int = 1;

pub const M98925_DAI_IMON_SLOT_SHIFT: c_int = 0;
pub const M98925_DAI_IMON_SLOT_WIDTH: c_int = 5;

// MAX98925_R024_DOUT_CFG_VBAT

pub const M98925_DAI_VBAT_EN_SHIFT: c_int = 5;
pub const M98925_DAI_VBAT_EN_WIDTH: c_int = 1;

pub const M98925_DAI_VBAT_SLOT_SHIFT: c_int = 0;
pub const M98925_DAI_VBAT_SLOT_WIDTH: c_int = 5;
// MAX98925_R025_DOUT_CFG_VBST

pub const M98925_DAI_VBST_EN_SHIFT: c_int = 5;
pub const M98925_DAI_VBST_EN_WIDTH: c_int = 1;

pub const M98925_DAI_VBST_SLOT_SHIFT: c_int = 0;
pub const M98925_DAI_VBST_SLOT_WIDTH: c_int = 5;
// MAX98925_R026_DOUT_CFG_FLAG

pub const M98925_DAI_FLAG_EN_SHIFT: c_int = 5;
pub const M98925_DAI_FLAG_EN_WIDTH: c_int = 1;

pub const M98925_DAI_FLAG_SLOT_SHIFT: c_int = 0;
pub const M98925_DAI_FLAG_SLOT_WIDTH: c_int = 5;
// MAX98925_R027_DOUT_HIZ_CFG1

pub const M98925_DAI_SLOT_HIZ_CFG1_SHIFT: c_int = 0;
pub const M98925_DAI_SLOT_HIZ_CFG1_WIDTH: c_int = 8;
// MAX98925_R028_DOUT_HIZ_CFG2

pub const M98925_DAI_SLOT_HIZ_CFG2_SHIFT: c_int = 0;
pub const M98925_DAI_SLOT_HIZ_CFG2_WIDTH: c_int = 8;
// MAX98925_R029_DOUT_HIZ_CFG3

pub const M98925_DAI_SLOT_HIZ_CFG3_SHIFT: c_int = 0;
pub const M98925_DAI_SLOT_HIZ_CFG3_WIDTH: c_int = 8;
// MAX98925_R02A_DOUT_HIZ_CFG4

pub const M98925_DAI_SLOT_HIZ_CFG4_SHIFT: c_int = 0;
pub const M98925_DAI_SLOT_HIZ_CFG4_WIDTH: c_int = 8;
// MAX98925_R02B_DOUT_DRV_STRENGTH

pub const M98925_DAI_OUT_DRIVE_SHIFT: c_int = 0;
pub const M98925_DAI_OUT_DRIVE_WIDTH: c_int = 2;
// MAX98925_R02C_FILTERS

pub const M98925_ADC_DITHER_EN_SHIFT: c_int = 7;
pub const M98925_ADC_DITHER_EN_WIDTH: c_int = 1;

pub const M98925_IV_DCB_EN_SHIFT: c_int = 6;
pub const M98925_IV_DCB_EN_WIDTH: c_int = 1;

pub const M98925_DAC_DITHER_EN_SHIFT: c_int = 4;
pub const M98925_DAC_DITHER_EN_WIDTH: c_int = 1;

pub const M98925_DAC_FILTER_MODE_SHIFT: c_int = 3;
pub const M98925_DAC_FILTER_MODE_WIDTH: c_int = 1;

pub const M98925_DAC_HPF_SHIFT: c_int = 0;
pub const M98925_DAC_HPF_WIDTH: c_int = 3;

// MAX98925_R02D_GAIN

pub const M98925_DAC_IN_SEL_SHIFT: c_int = 5;
pub const M98925_DAC_IN_SEL_WIDTH: c_int = 2;

pub const M98925_SPK_GAIN_SHIFT: c_int = 0;
pub const M98925_SPK_GAIN_WIDTH: c_int = 5;

// MAX98925_R02E_GAIN_RAMPING

pub const M98925_SPK_RMP_EN_SHIFT: c_int = 1;
pub const M98925_SPK_RMP_EN_WIDTH: c_int = 1;

pub const M98925_SPK_ZCD_EN_SHIFT: c_int = 0;
pub const M98925_SPK_ZCD_EN_WIDTH: c_int = 1;
// MAX98925_R02F_SPK_AMP

pub const M98925_SPK_MODE_SHIFT: c_int = 0;
pub const M98925_SPK_MODE_WIDTH: c_int = 1;
// MAX98925_R030_THRESHOLD

pub const M98925_ALC_EN_SHIFT: c_int = 5;
pub const M98925_ALC_EN_WIDTH: c_int = 1;

pub const M98925_ALC_TH_SHIFT: c_int = 0;
pub const M98925_ALC_TH_WIDTH: c_int = 5;
// MAX98925_R031_ALC_ATTACK

pub const M98925_ALC_ATK_STEP_SHIFT: c_int = 4;
pub const M98925_ALC_ATK_STEP_WIDTH: c_int = 4;

pub const M98925_ALC_ATK_RATE_SHIFT: c_int = 0;
pub const M98925_ALC_ATK_RATE_WIDTH: c_int = 3;
// MAX98925_R032_ALC_ATTEN_RLS

pub const M98925_ALC_MAX_ATTEN_SHIFT: c_int = 4;
pub const M98925_ALC_MAX_ATTEN_WIDTH: c_int = 4;

pub const M98925_ALC_RLS_RATE_SHIFT: c_int = 0;
pub const M98925_ALC_RLS_RATE_WIDTH: c_int = 3;
// MAX98925_R033_ALC_HOLD_RLS

pub const M98925_ALC_RLS_TGR_SHIFT: c_int = 0;
pub const M98925_ALC_RLS_TGR_WIDTH: c_int = 1;
// MAX98925_R034_ALC_CONFIGURATION

pub const M98925_ALC_MUTE_EN_SHIFT: c_int = 7;
pub const M98925_ALC_MUTE_EN_WIDTH: c_int = 1;

pub const M98925_ALC_MUTE_DLY_SHIFT: c_int = 4;
pub const M98925_ALC_MUTE_DLY_WIDTH: c_int = 3;

pub const M98925_ALC_RLS_DBT_SHIFT: c_int = 0;
pub const M98925_ALC_RLS_DBT_WIDTH: c_int = 3;
// MAX98925_R035_BOOST_CONVERTER

pub const M98925_BST_SYNC_SHIFT: c_int = 7;
pub const M98925_BST_SYNC_WIDTH: c_int = 1;

pub const M98925_BST_PHASE_SHIFT: c_int = 4;
pub const M98925_BST_PHASE_WIDTH: c_int = 2;

pub const M98925_BST_SKIP_MODE_SHIFT: c_int = 0;
pub const M98925_BST_SKIP_MODE_WIDTH: c_int = 2;
// MAX98925_R036_BLOCK_ENABLE

pub const M98925_BST_EN_SHIFT: c_int = 7;
pub const M98925_BST_EN_WIDTH: c_int = 1;

pub const M98925_WATCH_EN_SHIFT: c_int = 6;
pub const M98925_WATCH_EN_WIDTH: c_int = 1;

pub const M98925_CLKMON_EN_SHIFT: c_int = 5;
pub const M98925_CLKMON_EN_WIDTH: c_int = 1;

pub const M98925_SPK_EN_SHIFT: c_int = 4;
pub const M98925_SPK_EN_WIDTH: c_int = 1;

pub const M98925_ADC_VBST_EN_SHIFT: c_int = 3;
pub const M98925_ADC_VBST_EN_WIDTH: c_int = 1;

pub const M98925_ADC_VBAT_EN_SHIFT: c_int = 2;
pub const M98925_ADC_VBAT_EN_WIDTH: c_int = 1;

pub const M98925_ADC_IMON_EN_SHIFT: c_int = 1;
pub const M98925_ADC_IMON_EN_WIDTH: c_int = 1;

pub const M98925_ADC_VMON_EN_SHIFT: c_int = 0;
pub const M98925_ADC_VMON_EN_WIDTH: c_int = 1;
// MAX98925_R037_CONFIGURATION

pub const M98925_BST_VOUT_SHIFT: c_int = 4;
pub const M98925_BST_VOUT_WIDTH: c_int = 4;

pub const M98925_THERMWARN_LEVEL_SHIFT: c_int = 2;
pub const M98925_THERMWARN_LEVEL_WIDTH: c_int = 2;

pub const M98925_WATCH_TIME_SHIFT: c_int = 0;
pub const M98925_WATCH_TIME_WIDTH: c_int = 2;
// MAX98925_R038_GLOBAL_ENABLE

pub const M98925_EN_SHIFT: c_int = 7;
pub const M98925_EN_WIDTH: c_int = 1;
// MAX98925_R03A_BOOST_LIMITER

pub const M98925_BST_ILIM_SHIFT: c_int = 3;
pub const M98925_BST_ILIM_WIDTH: c_int = 5;
// MAX98925_R0FF_VERSION

pub const M98925_REV_ID_SHIFT: c_int = 0;
pub const M98925_REV_ID_WIDTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98925_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub pdata: *mut max98925_pdata,
    pub sysclk: c_uint,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub spk_gain: c_uint,
    pub ch_size: c_uint,
}
