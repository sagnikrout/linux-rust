//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stm32-timers.h
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
// Copyright (C) STMicroelectronics 2016
// Author: Benjamin Gaignard <benjamin.gaignard@st.com>
//

pub const TIM_CR1: c_uint = 0x00			/* Control Register 1			*/;
pub const TIM_CR2: c_uint = 0x04			/* Control Register 2			*/;
pub const TIM_SMCR: c_uint = 0x08			/* Slave mode control reg		*/;
pub const TIM_DIER: c_uint = 0x0C			/* DMA/interrupt register		*/;
pub const TIM_SR: c_uint = 0x10			/* Status register			*/;
pub const TIM_EGR: c_uint = 0x14			/* Event Generation Reg			*/;
pub const TIM_CCMR1: c_uint = 0x18			/* Capt/Comp 1 Mode Reg			*/;
pub const TIM_CCMR2: c_uint = 0x1C			/* Capt/Comp 2 Mode Reg			*/;
pub const TIM_CCER: c_uint = 0x20			/* Capt/Comp Enable Reg			*/;
pub const TIM_CNT: c_uint = 0x24			/* Counter				*/;
pub const TIM_PSC: c_uint = 0x28			/* Prescaler				*/;
pub const TIM_ARR: c_uint = 0x2c			/* Auto-Reload Register			*/;

pub const TIM_BDTR: c_uint = 0x44			/* Break and Dead-Time Reg		*/;
pub const TIM_DCR: c_uint = 0x48			/* DMA control register			*/;
pub const TIM_DMAR: c_uint = 0x4C			/* DMA register for transfer		*/;
pub const TIM_TISEL: c_uint = 0x68			/* Input Selection			*/;
pub const TIM_HWCFGR2: c_uint = 0x3EC			/* hardware configuration 2 Reg (MP25)	*/;
pub const TIM_HWCFGR1: c_uint = 0x3F0			/* hardware configuration 1 Reg (MP25)	*/;
pub const TIM_IPIDR: c_uint = 0x3F8			/* IP identification Reg (MP25)		*/;

pub const MAX_TIM_PSC: c_uint = 0xFFFF;
pub const MAX_TIM_ICPSC: c_uint = 0x3;
pub const TIM_CR2_MMS_SHIFT: c_int = 4;
pub const TIM_CR2_MMS2_SHIFT: c_int = 20;

pub const TIM_SMCR_TS_SHIFT: c_int = 4;
pub const TIM_BDTR_BKF_MASK: c_uint = 0xF;

pub const STM32MP25_TIM_IPIDR: c_uint = 0x00120002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_timers_dmas {
    STM32_TIMERS_DMA_CH1,
    STM32_TIMERS_DMA_CH2,
    STM32_TIMERS_DMA_CH3,
    STM32_TIMERS_DMA_CH4,
    STM32_TIMERS_DMA_UP,
    STM32_TIMERS_DMA_TRIG,
    STM32_TIMERS_DMA_COM,
    STM32_TIMERS_MAX_DMAS,
}

// STM32 Timer may have either a unique global interrupt or 4 interrupt lines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_timers_irqs {
    STM32_TIMERS_IRQ_GLOBAL_BRK, /* global or brk IRQ */
    STM32_TIMERS_IRQ_UP,
    STM32_TIMERS_IRQ_TRG_COM,
    STM32_TIMERS_IRQ_CC,
    STM32_TIMERS_MAX_IRQS,
}

//
// struct stm32_timers_dma - STM32 timer DMA handling.
// @completion:		end of DMA transfer completion
// @phys_base:		control registers physical base address
// @lock:		protect DMA access
// @chan:		DMA channel in use
// @chans:		DMA channels available for this timer instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_timers_dma {
    pub completion: completion,
    pub phys_base: phys_addr_t,
    pub lock: mutex,
    pub chan: *mut dma_chan,
    pub chans: [*mut dma_chan; STM32_TIMERS_MAX_DMAS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_timers {
    pub clk: *mut clk,
    pub ipidr: u32,
    pub regmap: *mut regmap,
    pub max_arr: u32,
    pub /: *mut *mut stm32_timers_dma dma; / Only to be used by the parent,
    pub nr_irqs: c_uint,
    pub irq: [c_int; STM32_TIMERS_MAX_IRQS],
}

