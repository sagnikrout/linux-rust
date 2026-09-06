//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/ctucanfd/ctucanfd.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// CTU CAN FD IP Core
//
// Copyright (C) 2015-2018 Ondrej Ille <ondrej.ille@gmail.com> FEE CTU
// Copyright (C) 2018-2021 Ondrej Ille <ondrej.ille@gmail.com> self-funded
// Copyright (C) 2018-2019 Martin Jerabek <martin.jerabek01@gmail.com> FEE CTU
// Copyright (C) 2018-2021 Pavel Pisa <pisa@cmp.felk.cvut.cz> FEE CTU/self-funded
//
// Project advisors:
// Jiri Novak <jnovak@fel.cvut.cz>
// Pavel Pisa <pisa@cmp.felk.cvut.cz>
//
// Department of Measurement         (http://meas.fel.cvut.cz/)
// Faculty of Electrical Engineering (http://www.fel.cvut.cz)
// Czech Technical University        (http://www.cvut.cz/)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctucan_priv {
    pub /: *mut *mut can_priv can; / must be first member!,
    pub mem_base: *mut void __iomem,
    pub reg): ctu_can_fd_can_registers,
    pub val): ctu_can_fd_can_registers reg, u32,
    pub txb_head: c_uint,
    pub txb_tail: c_uint,
    pub txb_prio: u32,
    pub ntxbufs: c_uint,
    pub /: *mut *mut spinlock_t tx_lock; / spinlock to serialize allocation and processing of TX buffers,
    pub napi: napi_struct,
    pub dev: *mut device,
    pub can_clk: *mut clk,
    pub irq_flags: c_int,
    pub drv_flags: c_ulong,
    pub rxfrm_first_word: u32,
    pub peers_on_pdev: list_head,
}

//
// ctucan_probe_common - Device type independent registration call
//
// This function does all the memory allocation and registration for the CAN
// device.
//
// @dev:	Handle to the generic device structure
// @addr:	Base address of CTU CAN FD core address
// @irq:	Interrupt number
// @ntxbufs:	Number of implemented Tx buffers
// @can_clk_rate: Clock rate, if 0 then clock are taken from device node
// @pm_enable_call: Whether pm_runtime_enable should be called
// @set_drvdata_fnc: Function to set network driver data for physical device
//
// Return: 0 on success and failure value on error
//
