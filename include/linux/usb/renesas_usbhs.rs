//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/renesas_usbhs.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Renesas USB
//
// Copyright (C) 2011 Renesas Solutions Corp.
// Copyright (C) 2019 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// module type
//
// it will be return value from get_id
//
// callback functions for platform
//
// These functions are called from driver for platform
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_usbhs_platform_callback {
//
// option:
//
// Hardware init function for platform.
// it is called when driver was probed.
//
    pub pdev): *mut *mut int (hardware_init)(struct platform_device,
//
// option:
//
// Hardware exit function for platform.
// it is called when driver was removed
//
    pub pdev): *mut *mut int (hardware_exit)(struct platform_device,
//
// option:
//
// for board specific clock control
//
    pub enable): *mut *mut void __iomem base, int,
//
// option:
//
// Phy reset for platform
//
    pub pdev): *mut *mut int (phy_reset)(struct platform_device,
//
// get USB ID function
// - USBHS_HOST
// - USBHS_GADGET
//
    pub pdev): *mut *mut int (get_id)(struct platform_device,
//
// get VBUS status function.
//
    pub pdev): *mut *mut int (get_vbus)(struct platform_device,
//
// option:
//
// VBUS control is needed for Host
//
    pub enable): *mut *mut *mut int (set_vbus)(struct platform_device pdev, int,
//
// option:
// extcon notifier to set host/peripheral mode.
//
    pub data): *mut c_void,
}

//
// parameters for renesas usbhs
//
// some register needs USB chip specific parameters.
// This struct show it to driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_usbhs_driver_pipe_config {
    pub /: *mut *mut u8 type; / USB_ENDPOINT_XFER_xxx,
    pub bufsize: u16,
    pub bufnum: u8,
    pub double_buf: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_usbhs_driver_param {
//
// pipe settings
//
    pub pipe_configs: *mut renesas_usbhs_driver_pipe_config,
    pub /: *mut *mut int pipe_size; / pipe_configs array size,
//
// option:
//
// for BUSWAIT :: BWAIT
// see
// renesas_usbhs/common.c :: usbhsc_set_buswait()
//
    pub buswait_bwait: c_int,
//
// option:
//
// delay time from notify_hotplug callback
//
    pub /: *mut *mut int detection_delay; / msec,
//
// option:
//
// dma id for dmaengine
// The data transfer direction on D0FIFO/D1FIFO should be
// fixed for keeping consistency.
// So, the platform id settings will be..
// .d0_tx_id = xx_TX,
// .d1_rx_id = xx_RX,
// or
// .d1_tx_id = xx_TX,
// .d0_rx_id = xx_RX,
//
    pub d0_tx_id: c_int,
    pub d0_rx_id: c_int,
    pub d1_tx_id: c_int,
    pub d1_rx_id: c_int,
    pub d2_tx_id: c_int,
    pub d2_rx_id: c_int,
    pub d3_tx_id: c_int,
    pub d3_rx_id: c_int,
//
// option:
//
// pio <--> dma border.
//
    pub /: *mut *mut int pio_dma_border; / default is 64byte,
//
// option:
//
    pub /: *mut *mut u32 has_usb_dmac:1; / for USB-DMAC,
    pub runtime_pwctrl:1: u32,
    pub has_cnen:1: u32,
    pub /: *mut *mut u32 cfifo_byte_addr:1; / CFIFO is byte addressable,

    pub multi_clks:1: u32,
    pub has_new_pipe_configs:1: u32,
}

//
// option:
//
// platform information for renesas_usbhs driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_usbhs_platform_info {
//
// option:
//
// platform set these functions before
// call platform_add_devices if needed
//
    pub platform_callback: renesas_usbhs_platform_callback,
//
// option:
//
// driver use these param for some register
//
    pub driver_param: renesas_usbhs_driver_param,
}
