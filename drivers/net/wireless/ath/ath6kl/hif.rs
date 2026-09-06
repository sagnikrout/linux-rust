//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/hif.h
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
// Copyright (c) 2004-2011 Atheros Communications Inc.
// Copyright (c) 2011 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const BUS_REQUEST_MAX_NUM: c_int = 64;
pub const HIF_MBOX_BLOCK_SIZE: c_int = 128;
pub const HIF_MBOX0_BLOCK_SIZE: c_int = 1;

pub const CMD53_FIXED_ADDRESS: c_int = 1;
pub const CMD53_INCR_ADDRESS: c_int = 2;
pub const MAX_SCATTER_REQUESTS: c_int = 4;
pub const MAX_SCATTER_ENTRIES_PER_REQ: c_int = 16;

// Mailbox address in SDIO address space
pub const HIF_MBOX_BASE_ADDR: c_uint = 0x800;
pub const HIF_MBOX_WIDTH: c_uint = 0x800;

// version 1 of the chip has only a 12K extended mbox range
pub const HIF_MBOX0_EXT_BASE_ADDR: c_uint = 0x4000;

// GMBOX addresses
pub const HIF_GMBOX_BASE_ADDR: c_uint = 0x7000;
pub const HIF_GMBOX_WIDTH: c_uint = 0x4000;
// interrupt mode register
pub const CCCR_SDIO_IRQ_MODE_REG: c_uint = 0xF0;
// mode to enable special 4-bit interrupt assertion without clock

// HTC runs over mailbox 0
pub const HTC_MAILBOX: c_int = 0;
pub const ATH6KL_TARGET_DEBUG_INTR_MASK: c_uint = 0x01;
// FIXME: are these duplicates with MAX_SCATTER_ values in hif.h?
pub const ATH6KL_SCATTER_ENTRIES_PER_REQ: c_int = 16;

pub const ATH6KL_SCATTER_REQS: c_int = 4;
pub const ATH6KL_HIF_COMMUNICATION_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_request {
    pub list: list_head,
// request data
    pub address: u32,
    pub buffer: *mut u8,
    pub length: u32,
    pub request: u32,
    pub packet: *mut htc_packet,
    pub status: c_int,
// this is a scatter request
    pub scat_req: *mut hif_scatter_req,
}

// direction of transfer (read/write)
pub const HIF_READ: c_uint = 0x00000001;
pub const HIF_WRITE: c_uint = 0x00000002;

//
// emode - This indicates the whether the command is to be executed in a
// blocking or non-blocking fashion (HIF_SYNCHRONOUS
// HIF_ASYNCHRONOUS). The read/write data paths in HTC have been
// implemented using the asynchronous mode allowing the bus
// driver to indicate the completion of operation through the
// registered callback routine. The requirement primarily comes
// from the contexts these operations get called from (a driver's
// transmit context or the ISR context in case of receive).
// Support for both of these modes is essential.
//
pub const HIF_SYNCHRONOUS: c_uint = 0x00000010;
pub const HIF_ASYNCHRONOUS: c_uint = 0x00000020;

//
// dmode - An interface may support different kinds of commands based on
// the tradeoff between the amount of data it can carry and the
// setup time. Byte and Block modes are supported (HIF_BYTE_BASIS
// HIF_BLOCK_BASIS). In case of latter, the data is rounded off
// to the nearest block size by padding. The size of the block is
// configurable at compile time using the HIF_BLOCK_SIZE and is
// negotiated with the target during initialization after the
// ATH6KL interrupts are enabled.
//
pub const HIF_BYTE_BASIS: c_uint = 0x00000040;
pub const HIF_BLOCK_BASIS: c_uint = 0x00000080;

//
// amode - This indicates if the address has to be incremented on ATH6KL
// after every read/write operation (HIF?FIXED_ADDRESS
// HIF_INCREMENTAL_ADDRESS).
//
pub const HIF_FIXED_ADDRESS: c_uint = 0x00000100;
pub const HIF_INCREMENTAL_ADDRESS: c_uint = 0x00000200;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hif_scatter_item {
    pub buf: *mut u8,
    pub len: c_int,
    pub packet: *mut htc_packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hif_scatter_req {
    pub list: list_head,
// address for the read/write operation
    pub addr: u32,
// request flags
    pub req: u32,
// total length of entire transfer
    pub len: u32,
    pub virt_scat: bool,
    pub ): *mut *mut *mut void (complete) (struct htc_target , struct hif_scatter_req,
    pub status: c_int,
    pub scat_entries: c_int,
    pub busrequest: *mut bus_request,
    pub sgentries: *mut scatterlist,
// bounce buffer for upper layers to copy to/from
    pub virt_dma_buf: *mut u8,
    pub scat_q_depth: u32,
    pub scat_list: [hif_scatter_item; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_irq_proc_registers {
    pub host_int_status: u8,
    pub cpu_int_status: u8,
    pub error_int_status: u8,
    pub counter_int_status: u8,
    pub mbox_frame: u8,
    pub rx_lkahd_valid: u8,
    pub host_int_status2: u8,
    pub gmbox_rx_avail: u8,
    pub rx_lkahd: [__le32; 2],
    pub rx_gmbox_lkahd_alias: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_irq_enable_reg {
    pub int_status_en: u8,
    pub cpu_int_status_en: u8,
    pub err_int_status_en: u8,
    pub cntr_int_status_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_device {
// protects irq_proc_reg and irq_en_reg below
    pub lock: spinlock_t,
    pub irq_proc_reg: ath6kl_irq_proc_registers,
    pub irq_en_reg: ath6kl_irq_enable_reg,
    pub htc_cnxt: *mut htc_target,
    pub ar: *mut ath6kl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_hif_ops {
    pub request): u32 len, u32,
    pub packet): *mut u32 length, u32 request, struct htc_packet,
    pub ar): *mut *mut void (irq_enable)(struct ath6kl,
    pub ar): *mut *mut void (irq_disable)(struct ath6kl,
    pub ar): *mut *mut *mut hif_scatter_req (scatter_req_get)(ath6kl,
    pub s_req): *mut hif_scatter_req,
    pub ar): *mut *mut int (enable_scatter)(struct ath6kl,
    pub scat_req): *mut hif_scatter_req,
    pub ar): *mut *mut void (cleanup_scatter)(struct ath6kl,
    pub wow): *mut *mut *mut int (suspend)(struct ath6kl ar, struct cfg80211_wowlan,
    pub ar): *mut *mut int (resume)(struct ath6kl,
    pub value): *mut *mut *mut int (diag_read32)(struct ath6kl ar, u32 address, u32,
    pub value): *mut *mut *mut int (diag_write32)(struct ath6kl ar, u32 address, __le32,
    pub len): *mut *mut *mut *mut int (bmi_read)(struct ath6kl ar, u8 buf, u32,
    pub len): *mut *mut *mut *mut int (bmi_write)(struct ath6kl ar, u8 buf, u32,
    pub ar): *mut *mut int (power_on)(struct ath6kl,
    pub ar): *mut *mut int (power_off)(struct ath6kl,
    pub ar): *mut *mut void (stop)(struct ath6kl,
    pub buf): *mut sk_buff,
    pub pipe_dl): *mut *mut *mut *mut void (pipe_get_default)(struct ath6kl ar, u8 pipe_ul, u8,
    pub pipe_dl): *mut u8,
    pub pipe): *mut *mut *mut u16 (pipe_get_free_queue_number)(struct ath6kl ar, u8,
}

extern "C" {
    pub fn ath6kl_hif_setup(dev: *mut ath6kl_device) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_unmask_intrs(dev: *mut ath6kl_device) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_mask_intrs(dev: *mut ath6kl_device) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_rx_control(dev: *mut ath6kl_device, enable_rx: bool) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_disable_intrs(dev: *mut ath6kl_device) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_rw_comp_handler(context: *mut c_void, status: c_int) -> c_int;
}
extern "C" {
    pub fn ath6kl_hif_intr_bh_handler(ar: *mut ath6kl) -> c_int;
}
// Scatter Function and Definitions
