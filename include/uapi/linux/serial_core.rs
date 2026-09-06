//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/serial_core.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright (C) 2000 Deep Blue Solutions Ltd.
//

//
// The type definitions.  These are from Ted Ts'o's serial.h
// By historical reasons the values from 0 to 13 are defined
// in the include/uapi/linux/serial.h, do not define them here.
// Values 0 to 19 are used by setserial from busybox and must never
// be modified.
//
pub const PORT_NS16550A: c_int = 14;
pub const PORT_XSCALE: c_int = 15;

pub const PORT_BRCM_TRUMANAGE: c_int = 25;

//
// ARM specific type numbers.  These are not currently guaranteed
// to be implemented, and will change in the future.  These are
// separate so any additions to the old serial.c that occur before
// we are merged can be easily merged here.
//
pub const PORT_PXA: c_int = 31;
pub const PORT_AMBA: c_int = 32;
pub const PORT_CLPS711X: c_int = 33;
pub const PORT_SA1100: c_int = 34;
pub const PORT_UART00: c_int = 35;
pub const PORT_OWL: c_int = 36;
pub const PORT_21285: c_int = 37;
// Sparc type numbers.
pub const PORT_SUNZILOG: c_int = 38;
pub const PORT_SUNSAB: c_int = 39;
// Nuvoton UART
pub const PORT_NPCM: c_int = 40;
// NVIDIA Tegra Combined UART
pub const PORT_TEGRA_TCU: c_int = 41;
// ASPEED AST2x00 virtual UART
pub const PORT_ASPEED_VUART: c_int = 42;
// Intel EG20
pub const PORT_PCH_8LINE: c_int = 44;
pub const PORT_PCH_2LINE: c_int = 45;
// DEC
pub const PORT_DZ: c_int = 46;
pub const PORT_ZS: c_int = 47;
// Parisc type numbers.
pub const PORT_MUX: c_int = 48;
// Atmel AT91 SoC
pub const PORT_ATMEL: c_int = 49;
// Macintosh Zilog type numbers

pub const PORT_PMAC_ZILOG: c_int = 51;
// SH-SCI
pub const PORT_SCI: c_int = 52;
pub const PORT_SCIF: c_int = 53;
pub const PORT_IRDA: c_int = 54;
// SGI IP22 aka Indy / Challenge S / Indigo 2
pub const PORT_IP22ZILOG: c_int = 56;
// PPC CPM type number
pub const PORT_CPM: c_int = 58;
// MPC52xx (and MPC512x) type numbers
pub const PORT_MPC52xx: c_int = 59;
// IBM icom
pub const PORT_ICOM: c_int = 60;
// Motorola i.MX SoC
pub const PORT_IMX: c_int = 62;
// TXX9 type number
pub const PORT_TXX9: c_int = 64;
// Moxa MUEx50 UART
pub const PORT_MUEX50: c_int = 65;
// Digi jsm
pub const PORT_JSM: c_int = 69;
// SUN4V Hypervisor Console
pub const PORT_SUNHV: c_int = 72;
// Xilinx uartlite
pub const PORT_UARTLITE: c_int = 74;
// Broadcom BCM7271 UART
pub const PORT_BCM7271: c_int = 76;
// Broadcom SB1250, etc. SOC
pub const PORT_SB1250_DUART: c_int = 77;
// Freescale ColdFire
pub const PORT_MCF: c_int = 78;
pub const PORT_SC26XX: c_int = 82;
// SH-SCI
pub const PORT_SCIFA: c_int = 83;
pub const PORT_S3C6400: c_int = 84;
// MAX3100
pub const PORT_MAX3100: c_int = 86;
// Timberdale UART
pub const PORT_TIMBUART: c_int = 87;
// Qualcomm MSM SoCs
pub const PORT_MSM: c_int = 88;
// BCM63xx family SoCs
pub const PORT_BCM63XX: c_int = 89;
// Aeroflex Gaisler GRLIB APBUART
pub const PORT_APBUART: c_int = 90;
// Altera UARTs
pub const PORT_ALTERA_JTAGUART: c_int = 91;
pub const PORT_ALTERA_UART: c_int = 92;
// SH-SCI
pub const PORT_SCIFB: c_int = 93;
// MAX310X
pub const PORT_MAX310X: c_int = 94;
// TI DA8xx/66AK2x
pub const PORT_DA830: c_int = 95;
// TI OMAP-UART
pub const PORT_OMAP: c_int = 96;
// VIA VT8500 SoC
pub const PORT_VT8500: c_int = 97;
// Cadence (Xilinx Zynq) UART
pub const PORT_XUARTPS: c_int = 98;
// Atheros AR933X SoC
pub const PORT_AR933X: c_int = 99;
// MCHP 16550A UART with 256 byte FIFOs
pub const PORT_MCHP16550A: c_int = 100;
// ARC (Synopsys) on-chip UART
pub const PORT_ARC: c_int = 101;
// Rocketport EXPRESS/INFINITY
pub const PORT_RP2: c_int = 102;
// Freescale lpuart
pub const PORT_LPUART: c_int = 103;
// SH-SCI
pub const PORT_HSCIF: c_int = 104;
// ST ASC type numbers
pub const PORT_ASC: c_int = 105;
// MEN 16z135 UART
pub const PORT_MEN_Z135: c_int = 107;
// SC16IS7xx
pub const PORT_SC16IS7XX: c_int = 108;
// MESON
pub const PORT_MESON: c_int = 109;
// Conexant Digicolor
pub const PORT_DIGICOLOR: c_int = 110;
// SPRD SERIAL
pub const PORT_SPRD: c_int = 111;
// STM32 USART
pub const PORT_STM32: c_int = 113;
// MVEBU UART
pub const PORT_MVEBU: c_int = 114;
// Microchip PIC32 UART
pub const PORT_PIC32: c_int = 115;
// MPS2 UART
pub const PORT_MPS2UART: c_int = 116;
// MediaTek BTIF
pub const PORT_MTK_BTIF: c_int = 117;
// RDA UART
pub const PORT_RDA: c_int = 118;
// Socionext Milbeaut UART
pub const PORT_MLB_USIO: c_int = 119;
// SiFive UART
pub const PORT_SIFIVE_V0: c_int = 120;
// Sunix UART
pub const PORT_SUNIX: c_int = 121;
// Freescale LINFlexD UART
pub const PORT_LINFLEXUART: c_int = 122;
// Sunplus UART
pub const PORT_SUNPLUS: c_int = 123;
// Generic type identifier for ports which type is not important to userspace.

