//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/boot/dts/spacemit/k3-pdma.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// DMA request number (DRQ) definitions for non-secure peripherals of
// the SpacemiT K3 PDMA.
//
// Copyright (c) 2025 SpacemiT
// Copyright (c) 2026 Guodong Xu <docular.xu@gmail.com>
//
// UART DMA request numbers
pub const K3_PDMA_UART0_TX: c_int = 3;
pub const K3_PDMA_UART0_RX: c_int = 4;
pub const K3_PDMA_UART2_TX: c_int = 5;
pub const K3_PDMA_UART2_RX: c_int = 6;
pub const K3_PDMA_UART3_TX: c_int = 7;
pub const K3_PDMA_UART3_RX: c_int = 8;
pub const K3_PDMA_UART4_TX: c_int = 9;
pub const K3_PDMA_UART4_RX: c_int = 10;
pub const K3_PDMA_UART5_TX: c_int = 25;
pub const K3_PDMA_UART5_RX: c_int = 26;
pub const K3_PDMA_UART6_TX: c_int = 27;
pub const K3_PDMA_UART6_RX: c_int = 28;
pub const K3_PDMA_UART7_TX: c_int = 29;
pub const K3_PDMA_UART7_RX: c_int = 30;
pub const K3_PDMA_UART8_TX: c_int = 31;
pub const K3_PDMA_UART8_RX: c_int = 32;
pub const K3_PDMA_UART9_TX: c_int = 33;
pub const K3_PDMA_UART9_RX: c_int = 34;
pub const K3_PDMA_UART10_TX: c_int = 53;
pub const K3_PDMA_UART10_RX: c_int = 54;
// I2C DMA request numbers
pub const K3_PDMA_I2C0_TX: c_int = 11;
pub const K3_PDMA_I2C0_RX: c_int = 12;
pub const K3_PDMA_I2C1_TX: c_int = 13;
pub const K3_PDMA_I2C1_RX: c_int = 14;
pub const K3_PDMA_I2C2_TX: c_int = 15;
pub const K3_PDMA_I2C2_RX: c_int = 16;
pub const K3_PDMA_I2C4_TX: c_int = 17;
pub const K3_PDMA_I2C4_RX: c_int = 18;
pub const K3_PDMA_I2C5_TX: c_int = 35;
pub const K3_PDMA_I2C5_RX: c_int = 36;
pub const K3_PDMA_I2C6_TX: c_int = 37;
pub const K3_PDMA_I2C6_RX: c_int = 38;
pub const K3_PDMA_I2C8_TX: c_int = 41;
pub const K3_PDMA_I2C8_RX: c_int = 42;
// SSP/SPI DMA request numbers
pub const K3_PDMA_SSP3_TX: c_int = 19;
pub const K3_PDMA_SSP3_RX: c_int = 20;
pub const K3_PDMA_SSPA0_TX: c_int = 21;
pub const K3_PDMA_SSPA0_RX: c_int = 22;
pub const K3_PDMA_SSPA1_TX: c_int = 23;
pub const K3_PDMA_SSPA1_RX: c_int = 24;
pub const K3_PDMA_SSPA2_TX: c_int = 56;
pub const K3_PDMA_SSPA2_RX: c_int = 57;
pub const K3_PDMA_SSPA3_TX: c_int = 58;
pub const K3_PDMA_SSPA3_RX: c_int = 59;
pub const K3_PDMA_SSPA4_TX: c_int = 60;
pub const K3_PDMA_SSPA4_RX: c_int = 61;
pub const K3_PDMA_SSPA5_TX: c_int = 62;
pub const K3_PDMA_SSPA5_RX: c_int = 63;
// CAN DMA request numbers
pub const K3_PDMA_CAN0_RX: c_int = 43;
pub const K3_PDMA_CAN1_RX: c_int = 44;
pub const K3_PDMA_CAN2_RX: c_int = 51;
pub const K3_PDMA_CAN3_RX: c_int = 52;
// SSP0/1 DMA request numbers
pub const K3_PDMA_SSP0_TX: c_int = 64;
pub const K3_PDMA_SSP0_RX: c_int = 65;
pub const K3_PDMA_SSP1_TX: c_int = 66;
pub const K3_PDMA_SSP1_RX: c_int = 67;
// QSPI DMA request numbers
pub const K3_PDMA_QSPI_RX: c_int = 84;
pub const K3_PDMA_QSPI_TX: c_int = 85;
