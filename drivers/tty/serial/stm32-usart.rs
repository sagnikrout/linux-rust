//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/stm32-usart.h
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
// Copyright (C) Maxime Coquelin 2015
// Copyright (C) STMicroelectronics SA 2017
// Authors:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
// Gerald Baeza <gerald_baeza@yahoo.fr>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_usart_offsets {
    pub cr1: u16,
    pub cr2: u16,
    pub cr3: u16,
    pub brr: u16,
    pub gtpr: u16,
    pub rtor: u16,
    pub rqr: u16,
    pub isr: u16,
    pub icr: u16,
    pub rdr: u16,
    pub tdr: u16,
    pub presc: u16,
    pub hwcfgr1: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_usart_config {
    pub /: *mut *mut u8 uart_enable_bit; / USART_CR1_UE,
    pub has_7bits_data: bool,
    pub has_swap: bool,
    pub has_wakeup: bool,
    pub has_fifo: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_usart_info {
    pub ofs: stm32_usart_offsets,
    pub cfg: stm32_usart_config,
}

pub const UNDEF_REG: c_uint = 0xffff;
// USART_SR (F4) / USART_ISR (F7)

// Dummy bits

// USART_DR

// USART_BRR

pub const USART_BRR_DIV_M_SHIFT: c_int = 4;
pub const USART_BRR_04_R_SHIFT: c_int = 1;

// USART_CR1

pub const USART_CR1_DEAT_SHIFT: c_int = 21;
pub const USART_CR1_DEDT_SHIFT: c_int = 16;
// USART_CR2

// USART_CR3

// USART_GTPR

// USART_RTOR

// USART_RQR

// USART_ICR

// USART_PRESC

// USART_HWCFCR1

pub const STM32_MAX_PORTS: c_int = 9;
pub const STM32H7_USART_FIFO_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_port {
    pub port: uart_port,
    pub clk: *mut clk,
    pub info: *const stm32_usart_info,
    pub /: *mut *mut *mut dma_chan rx_ch; / dma rx channel,
    pub /: *mut *mut dma_addr_t rx_dma_buf; / dma rx buffer bus address,
    pub /: *mut *mut *mut unsigned char rx_buf; / dma rx buffer cpu address,
    pub /: *mut *mut *mut dma_chan tx_ch; / dma tx channel,
    pub /: *mut *mut dma_addr_t tx_dma_buf; / dma tx buffer bus address,
    pub /: *mut *mut *mut unsigned char tx_buf; / dma tx buffer cpu address,
    pub /: *mut *mut u32 cr1_irq; / USART_CR1_RXNEIE or RTOIE,
    pub /: *mut *mut u32 cr3_irq; / USART_CR3_RXFTIE,
    pub last_res: c_int,
    pub /: *mut *mut bool tx_dma_busy; / dma tx transaction in progress,
    pub /: *mut *mut bool rx_dma_busy; / dma rx transaction in progress,
    pub /: *mut *mut bool throttled; / port throttled,
    pub hw_flow_control: bool,
    pub /: *mut *mut bool swap; / swap RX & TX pins,
    pub fifoen: bool,
    pub /: *mut *mut int rxftcfg; / RX FIFO threshold CFG,
    pub /: *mut *mut int txftcfg; / TX FIFO threshold CFG,
    pub wakeup_src: bool,
    pub /: *mut *mut int rdr_mask; / receive data register mask,
    pub /: *mut *mut *mut mctrl_gpios gpios; / modem control gpios,
    pub rx_dma_state: dma_tx_state,
}
