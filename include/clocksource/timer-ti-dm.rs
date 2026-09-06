//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/timer-ti-dm.h
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


//
// OMAP Dual-Mode Timers
//
// Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com
// Tarun Kanti DebBarma <tarun.kanti@ti.com>
// Thara Gopinath <thara@ti.com>
//
// Platform device conversion and hwmod support.
//
// Copyright (C) 2005 Nokia Corporation
// Author: Lauri Leukkunen <lauri.leukkunen@nokia.com>
// PWM and clock framwork support by Timo Teras.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
// NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// You should have received a copy of the  GNU General Public License along
// with this program; if not, write  to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//

// clock sources
pub const OMAP_TIMER_SRC_SYS_CLK: c_uint = 0x00;
pub const OMAP_TIMER_SRC_32_KHZ: c_uint = 0x01;
pub const OMAP_TIMER_SRC_EXT_CLK: c_uint = 0x02;
// timer interrupt enable bits

// trigger types
pub const OMAP_TIMER_TRIGGER_NONE: c_uint = 0x00;
pub const OMAP_TIMER_TRIGGER_OVERFLOW: c_uint = 0x01;
pub const OMAP_TIMER_TRIGGER_OVERFLOW_AND_COMPARE: c_uint = 0x02;
// timer capabilities used in hwmod database
pub const OMAP_TIMER_SECURE: c_uint = 0x80000000;
pub const OMAP_TIMER_ALWON: c_uint = 0x40000000;
pub const OMAP_TIMER_HAS_PWM: c_uint = 0x20000000;
pub const OMAP_TIMER_NEEDS_RESET: c_uint = 0x10000000;
pub const OMAP_TIMER_HAS_DSP_IRQ: c_uint = 0x08000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dm_timer {
}

extern "C" {
    pub fn omap_dm_timer_modify_idlect_mask(inputmask: u32) -> u32;
}
//
// Do not use the defines below, they are not needed. They should be only
// used by dmtimer.c and sys_timer related code.
//
// The interrupt registers are different between v1 and v2 ip.
// These registers are offsets from timer->iobase.
//
pub const OMAP_TIMER_ID_OFFSET: c_uint = 0x00;
pub const OMAP_TIMER_OCP_CFG_OFFSET: c_uint = 0x10;
pub const OMAP_TIMER_V1_SYS_STAT_OFFSET: c_uint = 0x14;
pub const OMAP_TIMER_V1_STAT_OFFSET: c_uint = 0x18;
pub const OMAP_TIMER_V1_INT_EN_OFFSET: c_uint = 0x1c;
pub const OMAP_TIMER_V2_IRQSTATUS_RAW: c_uint = 0x24;
pub const OMAP_TIMER_V2_IRQSTATUS: c_uint = 0x28;
pub const OMAP_TIMER_V2_IRQENABLE_SET: c_uint = 0x2c;
pub const OMAP_TIMER_V2_IRQENABLE_CLR: c_uint = 0x30;
//
// The functional registers have a different base on v1 and v2 ip.
// These registers are offsets from timer->func_base. The func_base
// is samae as io_base for v1 and io_base + 0x14 for v2 ip.
//
pub const OMAP_TIMER_V2_FUNC_OFFSET: c_uint = 0x14;
pub const _OMAP_TIMER_WAKEUP_EN_OFFSET: c_uint = 0x20;
pub const _OMAP_TIMER_CTRL_OFFSET: c_uint = 0x24;

pub const _OMAP_TIMER_COUNTER_OFFSET: c_uint = 0x28;
pub const _OMAP_TIMER_LOAD_OFFSET: c_uint = 0x2c;
pub const _OMAP_TIMER_TRIGGER_OFFSET: c_uint = 0x30;
pub const _OMAP_TIMER_WRITE_PEND_OFFSET: c_uint = 0x34;

pub const _OMAP_TIMER_MATCH_OFFSET: c_uint = 0x38;
pub const _OMAP_TIMER_CAPTURE_OFFSET: c_uint = 0x3c;
pub const _OMAP_TIMER_IF_CTRL_OFFSET: c_uint = 0x40;
pub const _OMAP_TIMER_CAPTURE2_OFFSET: c_uint = 0x44	/* TCAR2, 34xx only */;
pub const _OMAP_TIMER_TICK_POS_OFFSET: c_uint = 0x48	/* TPIR, 34xx only */;
pub const _OMAP_TIMER_TICK_NEG_OFFSET: c_uint = 0x4c	/* TNIR, 34xx only */;
pub const _OMAP_TIMER_TICK_COUNT_OFFSET: c_uint = 0x50	/* TCVR, 34xx only */;
pub const _OMAP_TIMER_TICK_INT_MASK_SET_OFFSET: c_uint = 0x54	/* TOCR, 34xx only */;
pub const _OMAP_TIMER_TICK_INT_MASK_COUNT_OFFSET: c_uint = 0x58	/* TOWR, 34xx only */;
