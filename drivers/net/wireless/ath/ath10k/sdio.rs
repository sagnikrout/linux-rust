//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/sdio.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2004-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
// Copyright (c) 2016-2017 Erik Stromdahl <erik.stromdahl@gmail.com>
//
pub const ATH10K_HIF_MBOX_BLOCK_SIZE: c_int = 256;

// Mailbox address in SDIO address space
pub const ATH10K_HIF_MBOX_BASE_ADDR: c_uint = 0x1000;
pub const ATH10K_HIF_MBOX_WIDTH: c_uint = 0x800;

pub const ATH10K_HIF_MBOX0_EXT_BASE_ADDR: c_uint = 0x5000;

pub const ATH10K_HIF_MBOX_NUM_MAX: c_int = 4;
pub const ATH10K_SDIO_BUS_REQUEST_MAX_NUM: c_int = 1024;

// HTC runs over mailbox 0
pub const ATH10K_HTC_MAILBOX: c_int = 0;

// GMBOX addresses
pub const ATH10K_HIF_GMBOX_BASE_ADDR: c_uint = 0x7000;
pub const ATH10K_HIF_GMBOX_WIDTH: c_uint = 0x4000;
// Modified versions of the sdio.h macros.
// The macros in sdio.h can't be used easily with the FIELD_{PREP|GET}
// macros in bitfield.h, so we define our own macros here.
//

pub const ATH10K_SDIO_DRIVE_DTSX_TYPE_B: c_int = 0;
pub const ATH10K_SDIO_DRIVE_DTSX_TYPE_A: c_int = 1;
pub const ATH10K_SDIO_DRIVE_DTSX_TYPE_C: c_int = 2;
pub const ATH10K_SDIO_DRIVE_DTSX_TYPE_D: c_int = 3;
// SDIO CCCR register definitions
pub const CCCR_SDIO_IRQ_MODE_REG: c_uint = 0xF0;
pub const CCCR_SDIO_IRQ_MODE_REG_SDIO3: c_uint = 0x16;
pub const CCCR_SDIO_DRIVER_STRENGTH_ENABLE_ADDR: c_uint = 0xF2;
pub const CCCR_SDIO_DRIVER_STRENGTH_ENABLE_A: c_uint = 0x02;
pub const CCCR_SDIO_DRIVER_STRENGTH_ENABLE_C: c_uint = 0x04;
pub const CCCR_SDIO_DRIVER_STRENGTH_ENABLE_D: c_uint = 0x08;
pub const CCCR_SDIO_ASYNC_INT_DELAY_ADDRESS: c_uint = 0xF0;
pub const CCCR_SDIO_ASYNC_INT_DELAY_MASK: c_uint = 0xC0;
// mode to enable special 4-bit interrupt assertion without clock

pub const ATH10K_SDIO_TARGET_DEBUG_INTR_MASK: c_uint = 0x01;
// The theoretical maximum number of RX messages that can be fetched
// from the mbox interrupt handler in one loop is derived in the following
// way:
//
// Let's assume that each packet in a bundle of the maximum bundle size
// (HTC_HOST_MAX_MSG_PER_RX_BUNDLE) has the HTC header bundle count set
// to the maximum value (HTC_HOST_MAX_MSG_PER_RX_BUNDLE).
//
// in this case the driver must allocate
// (HTC_HOST_MAX_MSG_PER_RX_BUNDLE * 2) skb's.
//

pub const ATH10K_FIFO_TIMEOUT_AND_CHIP_CONTROL: c_uint = 0x00000868u;
pub const ATH10K_FIFO_TIMEOUT_AND_CHIP_CONTROL_DISABLE_SLEEP_OFF: c_uint = 0xFFFEFFFF;
pub const ATH10K_FIFO_TIMEOUT_AND_CHIP_CONTROL_DISABLE_SLEEP_ON: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdio_mbox_state {
    SDIO_MBOX_UNKNOWN_STATE = 0,
    SDIO_MBOX_REQUEST_TO_SLEEP_STATE = 1,
    SDIO_MBOX_SLEEP_STATE = 2,
    SDIO_MBOX_AWAKE_STATE = 3,
}

pub const ATH10K_CIS_READ_WAIT_4_RTC_CYCLE_IN_US: c_int = 125;
pub const ATH10K_CIS_RTC_STATE_ADDR: c_uint = 0x1138;
pub const ATH10K_CIS_RTC_STATE_ON: c_uint = 0x01;
pub const ATH10K_CIS_XTAL_SETTLE_DURATION_IN_US: c_int = 1500;
pub const ATH10K_CIS_READ_RETRY: c_int = 10;
pub const ATH10K_MIN_SLEEP_INACTIVITY_TIME_MS: c_int = 50;
// TODO: remove this and use skb->cb instead, much cleaner approach
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio_bus_request {
    pub list: list_head,
// sdio address
    pub address: u32,
    pub skb: *mut sk_buff,
    pub eid: ath10k_htc_ep_id,
    pub status: c_int,
// Specifies if the current request is an HTC message.
// If not, the eid is not applicable an the TX completion handler
// associated with the endpoint will not be invoked.
//
    pub htc_msg: bool,
// Completion that (if set) will be invoked for non HTC requests
// (htc_msg == false) when the request has been processed.
//
    pub comp: *mut completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio_rx_data {
    pub skb: *mut sk_buff,
    pub alloc_len: usize,
    pub act_len: usize,
    pub eid: ath10k_htc_ep_id,
    pub part_of_bundle: bool,
    pub last_in_bundle: bool,
    pub trailer_only: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio_irq_proc_regs {
    pub host_int_status: u8,
    pub cpu_int_status: u8,
    pub error_int_status: u8,
    pub counter_int_status: u8,
    pub mbox_frame: u8,
    pub rx_lookahead_valid: u8,
    pub host_int_status2: u8,
    pub gmbox_rx_avail: u8,
    pub ATH10K_HIF_MBOX_NUM_MAX]: *mut *mut __le32 rx_lookahead[2,
    pub int_status_enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio_irq_enable_regs {
    pub int_status_en: u8,
    pub cpu_int_status_en: u8,
    pub err_int_status_en: u8,
    pub cntr_int_status_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio_irq_data {
// protects irq_proc_reg and irq_en_reg below.
// We use a mutex here and not a spinlock since we will have the
// mutex locked while calling the sdio_memcpy_ functions.
// These function require non atomic context, and hence, spinlocks
// can be held while calling these functions.
//
    pub mtx: mutex,
    pub irq_proc_reg: *mut ath10k_sdio_irq_proc_regs,
    pub irq_en_reg: *mut ath10k_sdio_irq_enable_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_mbox_ext_info {
    pub htc_ext_addr: u32,
    pub htc_ext_sz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_mbox_info {
    pub htc_addr: u32,
    pub ext_info: [ath10k_mbox_ext_info; 2],
    pub block_size: u32,
    pub block_mask: u32,
    pub gmbox_addr: u32,
    pub gmbox_sz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sdio {
    pub func: *mut sdio_func,
    pub mbox_info: ath10k_mbox_info,
    pub swap_mbox: bool,
    pub mbox_addr: [u32; ATH10K_HTC_EP_COUNT],
    pub mbox_size: [u32; ATH10K_HTC_EP_COUNT],
// available bus requests
    pub bus_req: [ath10k_sdio_bus_request; ATH10K_SDIO_BUS_REQUEST_MAX_NUM],
// free list of bus requests
    pub bus_req_freeq: list_head,
    pub rx_head: sk_buff_head,
// protects access to bus_req_freeq
    pub lock: spinlock_t,
    pub rx_pkts: [ath10k_sdio_rx_data; ATH10K_SDIO_MAX_RX_MSGS],
    pub n_rx_pkts: usize,
    pub ar: *mut ath10k,
    pub irq_data: ath10k_sdio_irq_data,
// temporary buffer for sdio read.
// It is allocated when probe, and used for receive bundled packets,
// the read for bundled packets is not parallel, so it does not need
// protected.
//
    pub vsg_buffer: *mut u8,
// temporary buffer for BMI requests
    pub bmi_buf: *mut u8,
    pub is_disabled: bool,
    pub workqueue: *mut workqueue_struct,
    pub wr_async_work: work_struct,
    pub wr_asyncq: list_head,
// protects access to wr_asyncq
    pub wr_async_lock: spinlock_t,
    pub async_work_rx: work_struct,
    pub sleep_timer: timer_list,
    pub mbox_state: sdio_mbox_state,
}
