//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amd/isp4/isp4_hw_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

pub const ISP_SOFT_RESET: c_uint = 0x62000;
pub const ISP_SYS_INT0_EN: c_uint = 0x62010;
pub const ISP_SYS_INT0_STATUS: c_uint = 0x62014;
pub const ISP_SYS_INT0_ACK: c_uint = 0x62018;
pub const ISP_CCPU_CNTL: c_uint = 0x62054;
pub const ISP_STATUS: c_uint = 0x62058;
pub const ISP_LOG_RB_BASE_LO0: c_uint = 0x62148;
pub const ISP_LOG_RB_BASE_HI0: c_uint = 0x6214c;
pub const ISP_LOG_RB_SIZE0: c_uint = 0x62150;
pub const ISP_LOG_RB_RPTR0: c_uint = 0x62154;
pub const ISP_LOG_RB_WPTR0: c_uint = 0x62158;
pub const ISP_RB_BASE_LO1: c_uint = 0x62170;
pub const ISP_RB_BASE_HI1: c_uint = 0x62174;
pub const ISP_RB_SIZE1: c_uint = 0x62178;
pub const ISP_RB_RPTR1: c_uint = 0x6217c;
pub const ISP_RB_WPTR1: c_uint = 0x62180;
pub const ISP_RB_BASE_LO2: c_uint = 0x62184;
pub const ISP_RB_BASE_HI2: c_uint = 0x62188;
pub const ISP_RB_SIZE2: c_uint = 0x6218c;
pub const ISP_RB_RPTR2: c_uint = 0x62190;
pub const ISP_RB_WPTR2: c_uint = 0x62194;
pub const ISP_RB_BASE_LO3: c_uint = 0x62198;
pub const ISP_RB_BASE_HI3: c_uint = 0x6219c;
pub const ISP_RB_SIZE3: c_uint = 0x621a0;
pub const ISP_RB_RPTR3: c_uint = 0x621a4;
pub const ISP_RB_WPTR3: c_uint = 0x621a8;
pub const ISP_RB_BASE_LO4: c_uint = 0x621ac;
pub const ISP_RB_BASE_HI4: c_uint = 0x621b0;
pub const ISP_RB_SIZE4: c_uint = 0x621b4;
pub const ISP_RB_RPTR4: c_uint = 0x621b8;
pub const ISP_RB_WPTR4: c_uint = 0x621bc;
pub const ISP_RB_BASE_LO5: c_uint = 0x621c0;
pub const ISP_RB_BASE_HI5: c_uint = 0x621c4;
pub const ISP_RB_SIZE5: c_uint = 0x621c8;
pub const ISP_RB_RPTR5: c_uint = 0x621cc;
pub const ISP_RB_WPTR5: c_uint = 0x621d0;
pub const ISP_RB_BASE_LO6: c_uint = 0x621d4;
pub const ISP_RB_BASE_HI6: c_uint = 0x621d8;
pub const ISP_RB_SIZE6: c_uint = 0x621dc;
pub const ISP_RB_RPTR6: c_uint = 0x621e0;
pub const ISP_RB_WPTR6: c_uint = 0x621e4;
pub const ISP_RB_BASE_LO7: c_uint = 0x621e8;
pub const ISP_RB_BASE_HI7: c_uint = 0x621ec;
pub const ISP_RB_SIZE7: c_uint = 0x621f0;
pub const ISP_RB_RPTR7: c_uint = 0x621f4;
pub const ISP_RB_WPTR7: c_uint = 0x621f8;
pub const ISP_RB_BASE_LO8: c_uint = 0x621fc;
pub const ISP_RB_BASE_HI8: c_uint = 0x62200;
pub const ISP_RB_SIZE8: c_uint = 0x62204;
pub const ISP_RB_RPTR8: c_uint = 0x62208;
pub const ISP_RB_WPTR8: c_uint = 0x6220c;
pub const ISP_RB_BASE_LO9: c_uint = 0x62210;
pub const ISP_RB_BASE_HI9: c_uint = 0x62214;
pub const ISP_RB_SIZE9: c_uint = 0x62218;
pub const ISP_RB_RPTR9: c_uint = 0x6221c;
pub const ISP_RB_WPTR9: c_uint = 0x62220;
pub const ISP_RB_BASE_LO10: c_uint = 0x62224;
pub const ISP_RB_BASE_HI10: c_uint = 0x62228;
pub const ISP_RB_SIZE10: c_uint = 0x6222c;
pub const ISP_RB_RPTR10: c_uint = 0x62230;
pub const ISP_RB_WPTR10: c_uint = 0x62234;
pub const ISP_RB_BASE_LO11: c_uint = 0x62238;
pub const ISP_RB_BASE_HI11: c_uint = 0x6223c;
pub const ISP_RB_SIZE11: c_uint = 0x62240;
pub const ISP_RB_RPTR11: c_uint = 0x62244;
pub const ISP_RB_WPTR11: c_uint = 0x62248;
pub const ISP_RB_BASE_LO12: c_uint = 0x6224c;
pub const ISP_RB_BASE_HI12: c_uint = 0x62250;
pub const ISP_RB_SIZE12: c_uint = 0x62254;
pub const ISP_RB_RPTR12: c_uint = 0x62258;
pub const ISP_RB_WPTR12: c_uint = 0x6225c;
pub const ISP_POWER_STATUS: c_uint = 0x60000;
// ISP_SOFT_RESET
pub const ISP_SOFT_RESET__CCPU_SOFT_RESET_MASK: c_uint = 0x00000001UL;
// ISP_CCPU_CNTL
pub const ISP_CCPU_CNTL__CCPU_HOST_SOFT_RST_MASK: c_uint = 0x00040000UL;
// ISP_STATUS
pub const ISP_STATUS__CCPU_REPORT_MASK: c_uint = 0x000000feUL;
// ISP_SYS_INT0_STATUS
pub const ISP_SYS_INT0_STATUS__SYS_INT_RINGBUFFER_WPT9_INT_MASK: c_uint = 0x00010000UL;
pub const ISP_SYS_INT0_STATUS__SYS_INT_RINGBUFFER_WPT10_INT_MASK: c_uint = 0x00040000UL;
pub const ISP_SYS_INT0_STATUS__SYS_INT_RINGBUFFER_WPT11_INT_MASK: c_uint = 0x00100000UL;
pub const ISP_SYS_INT0_STATUS__SYS_INT_RINGBUFFER_WPT12_INT_MASK: c_uint = 0x00400000UL;
// ISP_SYS_INT0_EN
pub const ISP_SYS_INT0_EN__SYS_INT_RINGBUFFER_WPT9_EN_MASK: c_uint = 0x00010000UL;
pub const ISP_SYS_INT0_EN__SYS_INT_RINGBUFFER_WPT10_EN_MASK: c_uint = 0x00040000UL;
pub const ISP_SYS_INT0_EN__SYS_INT_RINGBUFFER_WPT11_EN_MASK: c_uint = 0x00100000UL;
pub const ISP_SYS_INT0_EN__SYS_INT_RINGBUFFER_WPT12_EN_MASK: c_uint = 0x00400000UL;
// ISP_SYS_INT0_ACK
pub const ISP_SYS_INT0_ACK__SYS_INT_RINGBUFFER_WPT9_ACK_MASK: c_uint = 0x00010000UL;
pub const ISP_SYS_INT0_ACK__SYS_INT_RINGBUFFER_WPT10_ACK_MASK: c_uint = 0x00040000UL;
pub const ISP_SYS_INT0_ACK__SYS_INT_RINGBUFFER_WPT11_ACK_MASK: c_uint = 0x00100000UL;
pub const ISP_SYS_INT0_ACK__SYS_INT_RINGBUFFER_WPT12_ACK_MASK: c_uint = 0x00400000UL;
// Helper functions for reading isp registers
extern "C" {
    pub fn readl(reg: base +) -> return;
}
// Helper functions for writing isp registers
extern "C" {
    pub fn writel(_arg: val, reg: base +) -> return;
}
