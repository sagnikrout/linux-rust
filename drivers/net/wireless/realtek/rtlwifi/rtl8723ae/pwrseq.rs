//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723ae/pwrseq.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

//
// Check document WM-20110607-Paul-RTL8723A_Power_Architecture-R02.vsd
// There are 6 HW Power States:
// 0: POFF--Power Off
// 1: PDN--Power Down
// 2: CARDEMU--Card Emulation
// 3: ACT--Active Mode
// 4: LPS--Low Power State
// 5: SUS--Suspend
//
// The transision from different states are defined below
// TRANS_CARDEMU_TO_ACT
// TRANS_ACT_TO_CARDEMU
// TRANS_CARDEMU_TO_SUS
// TRANS_SUS_TO_CARDEMU
// TRANS_CARDEMU_TO_PDN
// TRANS_ACT_TO_LPS
// TRANS_LPS_TO_ACT
//
// TRANS_END
//
pub const RTL8723A_TRANS_CARDEMU_TO_ACT_STEPS: c_int = 10;
pub const RTL8723A_TRANS_ACT_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8723A_TRANS_CARDEMU_TO_SUS_STEPS: c_int = 10;
pub const RTL8723A_TRANS_SUS_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8723A_TRANS_CARDEMU_TO_PDN_STEPS: c_int = 10;
pub const RTL8723A_TRANS_PDN_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8723A_TRANS_ACT_TO_LPS_STEPS: c_int = 15;
pub const RTL8723A_TRANS_LPS_TO_ACT_STEPS: c_int = 15;
pub const RTL8723A_TRANS_END_STEPS: c_int = 1;
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value }

// disable SW LPS 0x04[10]=0*/	\
// wait till 0x04[17] = 1    power ready*/	\
// release WLON reset  0x04[16]=1*/	\
// disable HWPDN 0x04[15]=0*/ \
// disable WL suspend*/ \
// polling until return 0*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// 0x1F[7:0] = 0 turn off RF*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// 0x04[12:11] = 2b'11 enable WL suspend for PCIe*/	\
// 0x04[12:11] = 2b'01 enable WL suspend*/	\
// 0x04[12:11] = 2b'11 enable WL suspend for PCIe*/ \
// Set SDIO suspend local register*/	\
// wait power state to suspend*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// Set SDIO suspend local register*/	\
// wait power state to suspend*/ \
// 0x04[12:11] = 2b'00 disable WL suspend*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// 0x04[12:11] = 2b'01 enable WL suspend*/	 \
// 0x04[10] = 1, enable SW LPS*/	\
// Set SDIO suspend local register*/ \
// wait power state to suspend*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },
// Macro flag: #define RTL8723A_TRANS_CARDDIS_TO_CARDEMU\
// Set SDIO suspend local register*/	\
// wait power state to suspend*/ \
// 0x04[12:11] = 2b'00 disable WL suspend*/ \
// PCIe DMA start*/ \
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// Should be zero if no packet is transmitting*/	\
// CCK and OFDM are disabled,and clock are gated*/ \
// Respond TxOK to scheduler*/	\
// Macro flag: #define RTL8723A_TRANS_LPS_TO_ACT\
// format */	\
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value }, */	\
// .	0x08[4] = 0		 switch TSF to 40M*/\
// Polling 0x109[7]=0  TSF in 40M*/\
// .	0x29[7:6] = 2b'00	 enable BB clock*/\
// .	0x101[1] = 1*/\
// .	0x100[7:0] = 0xFF	 enable WMAC TRX*/\
// .	0x02[1:0] = 2b'11	 enable BB macro*/\
// format
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value },

// RTL8723 Power Configuration CMDs for PCIe interface

