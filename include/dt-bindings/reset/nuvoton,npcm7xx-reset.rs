//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/nuvoton,npcm7xx-reset.h
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
// Copyright (c) 2019 Nuvoton Technology corporation.
pub const NPCM7XX_RESET_IPSRST1: c_uint = 0x20;
pub const NPCM7XX_RESET_IPSRST2: c_uint = 0x24;
pub const NPCM7XX_RESET_IPSRST3: c_uint = 0x34;
// Reset lines on IP1 reset module (NPCM7XX_RESET_IPSRST1)
pub const NPCM7XX_RESET_FIU3: c_int = 1;
pub const NPCM7XX_RESET_UDC1: c_int = 5;
pub const NPCM7XX_RESET_EMC1: c_int = 6;
pub const NPCM7XX_RESET_UART_2_3: c_int = 7;
pub const NPCM7XX_RESET_UDC2: c_int = 8;
pub const NPCM7XX_RESET_PECI: c_int = 9;
pub const NPCM7XX_RESET_AES: c_int = 10;
pub const NPCM7XX_RESET_UART_0_1: c_int = 11;
pub const NPCM7XX_RESET_MC: c_int = 12;
pub const NPCM7XX_RESET_SMB2: c_int = 13;
pub const NPCM7XX_RESET_SMB3: c_int = 14;
pub const NPCM7XX_RESET_SMB4: c_int = 15;
pub const NPCM7XX_RESET_SMB5: c_int = 16;
pub const NPCM7XX_RESET_PWM_M0: c_int = 18;
pub const NPCM7XX_RESET_TIMER_0_4: c_int = 19;
pub const NPCM7XX_RESET_TIMER_5_9: c_int = 20;
pub const NPCM7XX_RESET_EMC2: c_int = 21;
pub const NPCM7XX_RESET_UDC4: c_int = 22;
pub const NPCM7XX_RESET_UDC5: c_int = 23;
pub const NPCM7XX_RESET_UDC6: c_int = 24;
pub const NPCM7XX_RESET_UDC3: c_int = 25;
pub const NPCM7XX_RESET_ADC: c_int = 27;
pub const NPCM7XX_RESET_SMB6: c_int = 28;
pub const NPCM7XX_RESET_SMB7: c_int = 29;
pub const NPCM7XX_RESET_SMB0: c_int = 30;
pub const NPCM7XX_RESET_SMB1: c_int = 31;
// Reset lines on IP2 reset module (NPCM7XX_RESET_IPSRST2)
pub const NPCM7XX_RESET_MFT0: c_int = 0;
pub const NPCM7XX_RESET_MFT1: c_int = 1;
pub const NPCM7XX_RESET_MFT2: c_int = 2;
pub const NPCM7XX_RESET_MFT3: c_int = 3;
pub const NPCM7XX_RESET_MFT4: c_int = 4;
pub const NPCM7XX_RESET_MFT5: c_int = 5;
pub const NPCM7XX_RESET_MFT6: c_int = 6;
pub const NPCM7XX_RESET_MFT7: c_int = 7;
pub const NPCM7XX_RESET_MMC: c_int = 8;
pub const NPCM7XX_RESET_SDHC: c_int = 9;
pub const NPCM7XX_RESET_GFX_SYS: c_int = 10;
pub const NPCM7XX_RESET_AHB_PCIBRG: c_int = 11;
pub const NPCM7XX_RESET_VDMA: c_int = 12;
pub const NPCM7XX_RESET_ECE: c_int = 13;
pub const NPCM7XX_RESET_VCD: c_int = 14;
pub const NPCM7XX_RESET_OTP: c_int = 16;
pub const NPCM7XX_RESET_SIOX1: c_int = 18;
pub const NPCM7XX_RESET_SIOX2: c_int = 19;
pub const NPCM7XX_RESET_3DES: c_int = 21;
pub const NPCM7XX_RESET_PSPI1: c_int = 22;
pub const NPCM7XX_RESET_PSPI2: c_int = 23;
pub const NPCM7XX_RESET_GMAC2: c_int = 25;
pub const NPCM7XX_RESET_USB_HOST: c_int = 26;
pub const NPCM7XX_RESET_GMAC1: c_int = 28;
pub const NPCM7XX_RESET_CP: c_int = 31;
// Reset lines on IP3 reset module (NPCM7XX_RESET_IPSRST3)
pub const NPCM7XX_RESET_PWM_M1: c_int = 0;
pub const NPCM7XX_RESET_SMB12: c_int = 1;
pub const NPCM7XX_RESET_SPIX: c_int = 2;
pub const NPCM7XX_RESET_SMB13: c_int = 3;
pub const NPCM7XX_RESET_UDC0: c_int = 4;
pub const NPCM7XX_RESET_UDC7: c_int = 5;
pub const NPCM7XX_RESET_UDC8: c_int = 6;
pub const NPCM7XX_RESET_UDC9: c_int = 7;
pub const NPCM7XX_RESET_PCI_MAILBOX: c_int = 9;
pub const NPCM7XX_RESET_SMB14: c_int = 12;
pub const NPCM7XX_RESET_SHA: c_int = 13;
pub const NPCM7XX_RESET_SEC_ECC: c_int = 14;
pub const NPCM7XX_RESET_PCIE_RC: c_int = 15;
pub const NPCM7XX_RESET_TIMER_10_14: c_int = 16;
pub const NPCM7XX_RESET_RNG: c_int = 17;
pub const NPCM7XX_RESET_SMB15: c_int = 18;
pub const NPCM7XX_RESET_SMB8: c_int = 19;
pub const NPCM7XX_RESET_SMB9: c_int = 20;
pub const NPCM7XX_RESET_SMB10: c_int = 21;
pub const NPCM7XX_RESET_SMB11: c_int = 22;
pub const NPCM7XX_RESET_ESPI: c_int = 23;
pub const NPCM7XX_RESET_USB_PHY_1: c_int = 24;
pub const NPCM7XX_RESET_USB_PHY_2: c_int = 25;
