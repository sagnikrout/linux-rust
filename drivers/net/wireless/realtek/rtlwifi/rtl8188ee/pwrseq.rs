//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8188ee/pwrseq.h
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
// Copyright(c) 2009-2013  Realtek Corporation.

// Check document WM-20110607-Paul-RTL8188EE_Power_Architecture-R02.vsd
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
// PWR SEQ Version: rtl8188ee_PwrSeq_V09.h
//
pub const RTL8188EE_TRANS_CARDEMU_TO_ACT_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_ACT_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_CARDEMU_TO_SUS_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_SUS_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_CARDEMU_TO_PDN_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_PDN_TO_CARDEMU_STEPS: c_int = 10;
pub const RTL8188EE_TRANS_ACT_TO_LPS_STEPS: c_int = 15;
pub const RTL8188EE_TRANS_LPS_TO_ACT_STEPS: c_int = 15;
pub const RTL8188EE_TRANS_END_STEPS: c_int = 1;
// The following macros have the following format:
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value
// comments },
//

// wait till 0x04[17] = 1    power ready*/},			\
// 0x02[1:0] = 0	reset BB*/},				\
// 0x24[23] = 2b'01 schmit trigger */},				\
// 0x04[15] = 0 disable HWPDN (control by DRV)*/},		\
// 0x04[12:11] = 2b'00 disable WL suspend*/},			\
// 0x04[8] = 1 polling until return 0*/},			\
// wait till 0x04[8] = 0*/},					\
// LDO normal mode*/},						\
// SDIO Driving*/},

// 0x1F[7:0] = 0 turn off RF*/},					\
// LDO Sleep mode*/},						\
// 0x04[9] = 1 turn off MAC by HW state machine*/},		\
// wait till 0x04[9] = 0 polling until return 0 to disable*/},

// 0x04[12:11] = 2b'01enable WL suspend*/},			\
// 0x04[12:11] = 2b'11enable WL suspend for PCIe*/},		\
// 0x04[31:30] = 2b'10 enable enable bandgap mbias in suspend */},\
// Clear SIC_EN register 0x40[12] = 1'b0 */},			\
// Set USB suspend enable local register  0xfe10[4]=1 */},	\
// Set SDIO suspend local register*/},				\
// wait power state to suspend*/},

// Set SDIO suspend local register*/},				\
// wait power state to suspend*/},				\
// 0x04[12:11] = 2b'00 disable WL suspend*/},

// 0x24[23] = 2b'01 schmit trigger */},				\
// 0x04[12:11] = 2b'01 enable WL suspend*/},			\
// 0x04[31:30] = 2b'10 enable enable bandgap mbias in suspend */},\
// Clear SIC_EN register 0x40[12] = 1'b0 */},			\
// Set USB suspend enable local register  0xfe10[4]=1 */},	\
// Set SDIO suspend local register*/},				\
// wait power state to suspend*/},

// Set SDIO suspend local register*/},				\
// wait power state to suspend*/},				\
// 0x04[12:11] = 2b'00 disable WL suspend*/},

// 0x04[15] = 1*/},

// Tx Pause*/},							\
// Should be zero if no packet is transmitting*/},		\
// CCK and OFDM are disabled,and clock are gated*/},		\
// Delay 1us*/},							\
// Reset MAC TRX*/},						\
// check if removed later*/},					\
// Respond TxOK to scheduler*/},

// SDIO RPWM*/},							\
// USB RPWM*/},							\
// PCIe RPWM*/},							\
// Delay*/},							\
// .	0x08[4] = 0		 switch TSF to 40M*/},		\
// Polling 0x109[7]=0  TSF in 40M*/},				\
// .	0x29[7:6] = 2b'00	 enable BB clock*/},		\
// .	0x101[1] = 1*/},					\
// .	0x100[7:0] = 0xFF	 enable WMAC TRX*/},		\
// .	0x02[1:0] = 2b'11	 enable BB macro*/},		\
// .	0x522 = 0*/},

// RTL8723 Power Configuration CMDs for PCIe interface

