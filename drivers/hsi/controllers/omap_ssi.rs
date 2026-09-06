//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hsi/controllers/omap_ssi.h
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


// SPDX-License-Identifier: GPL-2.0-only
// OMAP SSI internal interface.
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
// Copyright (C) 2013 Sebastian Reichel
//
// Contact: Carlos Chinea <carlos.chinea@nokia.com>
//

pub const SSI_MAX_CHANNELS: c_int = 8;
pub const SSI_MAX_GDD_LCH: c_int = 8;

pub const SSI_WAKE_EN: c_int = 0;
//
// struct omap_ssm_ctx - OMAP synchronous serial module (TX/RX) context
// @mode: Bit transmission mode
// @channels: Number of channels
// @framesize: Frame size in bits
// @timeout: RX frame timeout
// @divisor: TX divider
// @arb_mode: Arbitration mode for TX frame (Round robin, priority)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_ssm_ctx {
    pub mode: u32,
    pub channels: u32,
    pub frame_size: u32,
    pub /: *mut *mut u32 timeout; / Rx Only,
    pub arb_mode: u32,
    pub divisor: u32,
}

//
// struct omap_ssi_port - OMAP SSI port data
// @dev: device associated to the port (HSI port)
// @pdev: platform device associated to the port
// @sst_dma: SSI transmitter physical base address
// @ssr_dma: SSI receiver physical base address
// @sst_base: SSI transmitter base address
// @ssr_base: SSI receiver base address
// @wk_lock: spin lock to serialize access to the wake lines
// @lock: Spin lock to serialize access to the SSI port
// @channels: Current number of channels configured (1,2,4 or 8)
// @txqueue: TX message queues
// @rxqueue: RX message queues
// @brkqueue: Queue of incoming HWBREAK requests (FRAME mode)
// @errqueue: Queue for failed messages
// @errqueue_work: Delayed Work for failed messages
// @irq: IRQ number
// @wake_irq: IRQ number for incoming wake line (-1 if none)
// @wake_gpio: GPIO number for incoming wake line (-1 if none)
// @flags: flags to keep track of states
// @wk_refcount: Reference count for output wake line
// @work: worker for starting TX
// @sys_mpu_enable: Context for the interrupt enable register for irq 0
// @sst: Context for the synchronous serial transmitter
// @ssr: Context for the synchronous serial receiver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_ssi_port {
    pub dev: *mut device,
    pub pdev: *mut device,
    pub sst_dma: dma_addr_t,
    pub ssr_dma: dma_addr_t,
    pub sst_base: *mut void __iomem,
    pub ssr_base: *mut void __iomem,
    pub wk_lock: spinlock_t,
    pub lock: spinlock_t,
    pub channels: c_uint,
    pub txqueue: [list_head; SSI_MAX_CHANNELS],
    pub rxqueue: [list_head; SSI_MAX_CHANNELS],
    pub brkqueue: list_head,
    pub errqueue: list_head,
    pub errqueue_work: delayed_work,
    pub irq: c_uint,
    pub wake_irq: c_int,
    pub wake_gpio: *mut gpio_desc,
    pub /: *mut *mut bool wktest:1; / FIXME: HACK to be removed,
    pub flags: c_ulong,
    pub wk_refcount: c_uint,
    pub work: work_struct,
// OMAP SSI port context
    pub /: *mut *mut u32 sys_mpu_enable; / We use only one irq,
    pub sst: omap_ssm_ctx,
    pub ssr: omap_ssm_ctx,
    pub loss_count: u32,
    pub port_id: u32,

    pub dir: *mut dentry,

}

//
// struct gdd_trn - GDD transaction data
// @msg: Pointer to the HSI message being served
// @sg: Pointer to the current sg entry being served
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdd_trn {
    pub msg: *mut hsi_msg,
    pub sg: *mut scatterlist,
}

//
// struct omap_ssi_controller - OMAP SSI controller data
// @dev: device associated to the controller (HSI controller)
// @sys: SSI I/O base address
// @gdd: GDD I/O base address
// @fck: SSI functional clock
// @gdd_irq: IRQ line for GDD
// @gdd_tasklet: bottom half for DMA transfers
// @gdd_trn: Array of GDD transaction data for ongoing GDD transfers
// @lock: lock to serialize access to GDD
// @fck_nb: DVFS notfifier block
// @fck_rate: clock rate
// @loss_count: To follow if we need to restore context or not
// @max_speed: Maximum TX speed (Kb/s) set by the clients.
// @gdd_gcr: SSI GDD saved context
// @get_loss: Pointer to omap_pm_get_dev_context_loss_count, if any
// @port: Array of pointers of the ports of the controller
// @dir: Debugfs SSI root directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_ssi_controller {
    pub dev: *mut device,
    pub sys: *mut void __iomem,
    pub gdd: *mut void __iomem,
    pub fck: *mut clk,
    pub gdd_irq: c_uint,
    pub gdd_tasklet: tasklet_struct,
    pub gdd_trn: [gdd_trn; SSI_MAX_GDD_LCH],
    pub lock: spinlock_t,
    pub fck_nb: notifier_block,
    pub fck_rate: c_ulong,
    pub loss_count: u32,
    pub max_speed: u32,
// OMAP SSI Controller context
    pub gdd_gcr: u32,
    pub dev): *mut *mut int (get_loss)(struct device,
    pub port: *mut omap_ssi_port,

    pub dir: *mut dentry,

}
