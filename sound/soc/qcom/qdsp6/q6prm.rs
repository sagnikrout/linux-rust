//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6prm.h
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
pub const LPAIF_MI2S_MCLK: c_int = 1;
pub const LPAIF_MI2S_BCLK: c_int = 2;
// Clock ID for Primary I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_PRI_MI2S_IBIT: c_uint = 0x100;
// Clock ID for Primary I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_PRI_MI2S_EBIT: c_uint = 0x101;
// Clock ID for Secondary I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_SEC_MI2S_IBIT: c_uint = 0x102;
// Clock ID for Secondary I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_SEC_MI2S_EBIT: c_uint = 0x103;
// Clock ID for Tertiary I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_TER_MI2S_IBIT: c_uint = 0x104;
// Clock ID for Tertiary I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_TER_MI2S_EBIT: c_uint = 0x105;
// Clock ID for Quartnery I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_QUAD_MI2S_IBIT: c_uint = 0x106;
// Clock ID for Quartnery I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_QUAD_MI2S_EBIT: c_uint = 0x107;
// Clock ID for Speaker I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_IBIT: c_uint = 0x108;
// Clock ID for Speaker I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_EBIT: c_uint = 0x109;
// Clock ID for Speaker I2S OSR
pub const Q6PRM_LPASS_CLK_ID_SPEAKER_I2S_OSR: c_uint = 0x10A;
// Clock ID for QUINARY  I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_IBIT: c_uint = 0x10B;
// Clock ID for QUINARY  I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_EBIT: c_uint = 0x10C;
// Clock ID for SENARY  I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_SEN_MI2S_IBIT: c_uint = 0x10D;
// Clock ID for SENARY  I2S EBIT
pub const Q6PRM_LPASS_CLK_ID_SEN_MI2S_EBIT: c_uint = 0x10E;
// Clock ID for INT0 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT0_MI2S_IBIT: c_uint = 0x10F;
// Clock ID for INT1 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT1_MI2S_IBIT: c_uint = 0x110;
// Clock ID for INT2 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT2_MI2S_IBIT: c_uint = 0x111;
// Clock ID for INT3 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT3_MI2S_IBIT: c_uint = 0x112;
// Clock ID for INT4 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT4_MI2S_IBIT: c_uint = 0x113;
// Clock ID for INT5 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT5_MI2S_IBIT: c_uint = 0x114;
// Clock ID for INT6 I2S IBIT
pub const Q6PRM_LPASS_CLK_ID_INT6_MI2S_IBIT: c_uint = 0x115;
// Clock ID for QUINARY MI2S OSR CLK
pub const Q6PRM_LPASS_CLK_ID_QUI_MI2S_OSR: c_uint = 0x116;
// Clock ID for MCLK1
pub const Q6PRM_LPASS_CLK_ID_MCLK_1: c_uint = 0x300;
// Clock ID for MCLK2
pub const Q6PRM_LPASS_CLK_ID_MCLK_2: c_uint = 0x301;
// Clock ID for MCLK3
pub const Q6PRM_LPASS_CLK_ID_MCLK_3: c_uint = 0x302;
// Clock ID for MCLK4
pub const Q6PRM_LPASS_CLK_ID_MCLK_4: c_uint = 0x303;
// Clock ID for MCLK5
pub const Q6PRM_LPASS_CLK_ID_MCLK_5: c_uint = 0x304;
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_MCLK: c_uint = 0x305;
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_NPL_MCLK: c_uint = 0x306;
pub const Q6PRM_LPASS_CLK_ID_VA_CORE_MCLK: c_uint = 0x307;
pub const Q6PRM_LPASS_CLK_ID_VA_CORE_2X_MCLK: c_uint = 0x308;
pub const Q6PRM_LPASS_CLK_ID_TX_CORE_MCLK: c_uint = 0x30c;
pub const Q6PRM_LPASS_CLK_ID_TX_CORE_NPL_MCLK: c_uint = 0x30d;
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_MCLK: c_uint = 0x30e;
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_NPL_MCLK: c_uint = 0x30f;
// Clock ID for MCLK for WSA2 core
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_MCLK: c_uint = 0x310;
// Clock ID for NPL MCLK for WSA2 core
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_2X_MCLK: c_uint = 0x311;
// Clock ID for RX Core TX MCLK
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_TX_MCLK: c_uint = 0x312;
// Clock ID for RX CORE TX 2X MCLK
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_TX_2X_MCLK: c_uint = 0x313;
// Clock ID for WSA core TX MCLK
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_TX_MCLK: c_uint = 0x314;
// Clock ID for WSA core TX 2X MCLK
pub const Q6PRM_LPASS_CLK_ID_WSA_CORE_TX_2X_MCLK: c_uint = 0x315;
// Clock ID for WSA2 core TX MCLK
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_TX_MCLK: c_uint = 0x316;
// Clock ID for WSA2 core TX 2X MCLK
pub const Q6PRM_LPASS_CLK_ID_WSA2_CORE_TX_2X_MCLK: c_uint = 0x317;
// Clock ID for RX CORE MCLK2 2X  MCLK
pub const Q6PRM_LPASS_CLK_ID_RX_CORE_MCLK2_2X_MCLK: c_uint = 0x318;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF0_IBIT: c_uint = 0x500;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF0_EBIT: c_uint = 0x501;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF1_IBIT: c_uint = 0x502;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF1_EBIT: c_uint = 0x503;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF2_IBIT: c_uint = 0x504;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF2_EBIT: c_uint = 0x505;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF3_IBIT: c_uint = 0x506;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF3_EBIT: c_uint = 0x507;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF4_IBIT: c_uint = 0x508;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF4_EBIT: c_uint = 0x509;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF5_IBIT: c_uint = 0x50A;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF5_EBIT: c_uint = 0x50B;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF6_IBIT: c_uint = 0x50C;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF6_EBIT: c_uint = 0x50D;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF7_IBIT: c_uint = 0x50E;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF7_EBIT: c_uint = 0x50F;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF8_IBIT: c_uint = 0x510;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF8_EBIT: c_uint = 0x511;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF9_IBIT: c_uint = 0x512;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF9_EBIT: c_uint = 0x513;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF10_IBIT: c_uint = 0x514;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF10_EBIT: c_uint = 0x515;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF11_IBIT: c_uint = 0x516;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF11_EBIT: c_uint = 0x517;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF12_IBIT: c_uint = 0x518;
pub const Q6PRM_LPASS_CLK_ID_QAIF_IF12_EBIT: c_uint = 0x519;
pub const Q6PRM_LPASS_CLK_ID_VA_QAIF_IF0_IBIT: c_uint = 0x550;
pub const Q6PRM_LPASS_CLK_ID_VA_QAIF_IF0_EBIT: c_uint = 0x551;
pub const Q6PRM_LPASS_CLK_SRC_INTERNAL: c_int = 1;
pub const Q6PRM_LPASS_CLK_ROOT_DEFAULT: c_int = 0;
pub const Q6PRM_HW_CORE_ID_LPASS: c_int = 1;
pub const Q6PRM_HW_CORE_ID_DCODEC: c_int = 2;
pub const Q6PRM_HW_LPR_VOTE: c_int = 3;
