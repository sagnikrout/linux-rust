//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/atmel_usba_udc.h
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
// Driver for the Atmel USBA high speed USB device controller
//
// Copyright (C) 2005-2007 Atmel Corporation
//

// USB register offsets
pub const USBA_CTRL: c_uint = 0x0000;
pub const USBA_FNUM: c_uint = 0x0004;
pub const USBA_INT_ENB: c_uint = 0x0010;
pub const USBA_INT_STA: c_uint = 0x0014;
pub const USBA_INT_CLR: c_uint = 0x0018;
pub const USBA_EPT_RST: c_uint = 0x001c;
pub const USBA_TST: c_uint = 0x00e0;
// USB endpoint register offsets
pub const USBA_EPT_CFG: c_uint = 0x0000;
pub const USBA_EPT_CTL_ENB: c_uint = 0x0004;
pub const USBA_EPT_CTL_DIS: c_uint = 0x0008;
pub const USBA_EPT_CTL: c_uint = 0x000c;
pub const USBA_EPT_SET_STA: c_uint = 0x0014;
pub const USBA_EPT_CLR_STA: c_uint = 0x0018;
pub const USBA_EPT_STA: c_uint = 0x001c;
// USB DMA register offsets
pub const USBA_DMA_NXT_DSC: c_uint = 0x0000;
pub const USBA_DMA_ADDRESS: c_uint = 0x0004;
pub const USBA_DMA_CONTROL: c_uint = 0x0008;
pub const USBA_DMA_STATUS: c_uint = 0x000c;
// Bitfields in CTRL
pub const USBA_DEV_ADDR_OFFSET: c_int = 0;
pub const USBA_DEV_ADDR_SIZE: c_int = 7;

// Bitfields in FNUM
pub const USBA_MICRO_FRAME_NUM_OFFSET: c_int = 0;
pub const USBA_MICRO_FRAME_NUM_SIZE: c_int = 3;
pub const USBA_FRAME_NUMBER_OFFSET: c_int = 3;
pub const USBA_FRAME_NUMBER_SIZE: c_int = 11;

// Bitfields in INT_ENB/INT_STA/INT_CLR

pub const USBA_EPT_INT_OFFSET: c_int = 8;
pub const USBA_EPT_INT_SIZE: c_int = 16;
pub const USBA_DMA_INT_OFFSET: c_int = 24;
pub const USBA_DMA_INT_SIZE: c_int = 8;
// Bitfields in EPT_RST
pub const USBA_RST_OFFSET: c_int = 0;
pub const USBA_RST_SIZE: c_int = 16;
// Bitfields in USBA_TST
pub const USBA_SPEED_CFG_OFFSET: c_int = 0;
pub const USBA_SPEED_CFG_SIZE: c_int = 2;

// Bitfields in EPT_CFG
pub const USBA_EPT_SIZE_OFFSET: c_int = 0;
pub const USBA_EPT_SIZE_SIZE: c_int = 3;

pub const USBA_EPT_TYPE_OFFSET: c_int = 4;
pub const USBA_EPT_TYPE_SIZE: c_int = 2;
pub const USBA_BK_NUMBER_OFFSET: c_int = 6;
pub const USBA_BK_NUMBER_SIZE: c_int = 2;
pub const USBA_NB_TRANS_OFFSET: c_int = 8;
pub const USBA_NB_TRANS_SIZE: c_int = 2;

// Bitfields in EPT_CTL/EPT_CTL_ENB/EPT_CTL_DIS

// Bits 8-15 and 31 enable interrupts for respective bits in EPT_STA

// Bitfields in EPT_SET_STA/EPT_CLR_STA/EPT_STA

pub const USBA_TOGGLE_SEQ_OFFSET: c_int = 6;
pub const USBA_TOGGLE_SEQ_SIZE: c_int = 2;

pub const USBA_CURRENT_BANK_OFFSET: c_int = 16;
pub const USBA_CURRENT_BANK_SIZE: c_int = 2;
pub const USBA_BUSY_BANKS_OFFSET: c_int = 18;
pub const USBA_BUSY_BANKS_SIZE: c_int = 2;
pub const USBA_BYTE_COUNT_OFFSET: c_int = 20;
pub const USBA_BYTE_COUNT_SIZE: c_int = 11;

// Bitfields in DMA_CONTROL

pub const USBA_DMA_BUF_LEN_OFFSET: c_int = 16;
pub const USBA_DMA_BUF_LEN_SIZE: c_int = 16;
// Bitfields in DMA_STATUS

// Constants for SPEED_CFG
pub const USBA_SPEED_CFG_NORMAL: c_int = 0;
pub const USBA_SPEED_CFG_FORCE_HIGH: c_int = 2;
pub const USBA_SPEED_CFG_FORCE_FULL: c_int = 3;
// Constants for EPT_SIZE
pub const USBA_EPT_SIZE_8: c_int = 0;
pub const USBA_EPT_SIZE_16: c_int = 1;
pub const USBA_EPT_SIZE_32: c_int = 2;
pub const USBA_EPT_SIZE_64: c_int = 3;
pub const USBA_EPT_SIZE_128: c_int = 4;
pub const USBA_EPT_SIZE_256: c_int = 5;
pub const USBA_EPT_SIZE_512: c_int = 6;
pub const USBA_EPT_SIZE_1024: c_int = 7;
// Constants for EPT_TYPE
pub const USBA_EPT_TYPE_CONTROL: c_int = 0;
pub const USBA_EPT_TYPE_ISO: c_int = 1;
pub const USBA_EPT_TYPE_BULK: c_int = 2;
pub const USBA_EPT_TYPE_INT: c_int = 3;
// Constants for BK_NUMBER
pub const USBA_BK_NUMBER_ZERO: c_int = 0;
pub const USBA_BK_NUMBER_ONE: c_int = 1;
pub const USBA_BK_NUMBER_DOUBLE: c_int = 2;
pub const USBA_BK_NUMBER_TRIPLE: c_int = 3;
// Bit manipulation macros

// Register access macros

// Calculate base address for a given endpoint or DMA controller

// Synth parameters
pub const USBA_NR_DMAS: c_int = 7;
pub const EP0_FIFO_SIZE: c_int = 64;

pub const EP0_NR_BANKS: c_int = 1;
pub const FIFO_IOMEM_ID: c_int = 0;
pub const CTRL_IOMEM_ID: c_int = 1;
pub const DBG_ERR: c_uint = 0x0001	/* report all error returns */;
pub const DBG_HW: c_uint = 0x0002	/* debug hardware initialization */;
pub const DBG_GADGET: c_uint = 0x0004	/* calls to/from gadget driver */;
pub const DBG_INT: c_uint = 0x0008	/* interrupts */;
pub const DBG_BUS: c_uint = 0x0010	/* report changes in bus state */;
pub const DBG_QUEUE: c_uint = 0x0020  /* debug request queue processing */;
pub const DBG_FIFO: c_uint = 0x0040  /* debug FIFO contents */;
pub const DBG_DMA: c_uint = 0x0080  /* debug DMA handling */;
pub const DBG_REQ: c_uint = 0x0100	/* print out queued request length */;
pub const DBG_ALL: c_uint = 0xffff;
pub const DBG_NONE: c_uint = 0x0000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usba_ctrl_state {
    WAIT_FOR_SETUP,
    DATA_STAGE_IN,
    DATA_STAGE_OUT,
    STATUS_STAGE_IN,
    STATUS_STAGE_OUT,
    STATUS_STAGE_ADDR,
    STATUS_STAGE_TEST,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_dma_desc {
    pub next: dma_addr_t,
    pub addr: dma_addr_t,
    pub ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_fifo_cfg {
    pub hw_ep_num: u8,
    pub fifo_size: u16,
    pub nr_banks: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_ep {
    pub state: c_int,
    pub ep_regs: *mut void __iomem,
    pub dma_regs: *mut void __iomem,
    pub fifo: *mut void __iomem,
    pub name: [c_char; 8],
    pub ep: usb_ep,
    pub udc: *mut usba_udc,
    pub queue: list_head,
    pub fifo_size: u16,
    pub nr_banks: u8,
    pub index: u8,
    pub can_dma:1: c_uint,
    pub can_isoc:1: c_uint,
    pub is_isoc:1: c_uint,
    pub is_in:1: c_uint,
    pub ept_cfg: c_ulong,

    pub last_dma_status: u32,
    pub debugfs_dir: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_ep_config {
    pub nr_banks: u8,
    pub can_dma:1: c_uint,
    pub can_isoc:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_request {
    pub req: usb_request,
    pub queue: list_head,
    pub ctrl: u32,
    pub submitted:1: c_uint,
    pub last_transaction:1: c_uint,
    pub using_dma:1: c_uint,
    pub mapped:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_udc_errata {
    pub is_on): *mut *mut *mut void (toggle_bias)(struct usba_udc udc, int,
    pub udc): *mut *mut void (pulse_bias)(struct usba_udc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_udc_config {
    pub errata: *const usba_udc_errata,
    pub config: *const usba_ep_config,
    pub num_ep: c_int,
    pub ep_prealloc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usba_udc {
// Protect hw registers from concurrent modifications
    pub lock: spinlock_t,
// Mutex to prevent concurrent start or stop
    pub vbus_mutex: mutex,
    pub regs: *mut void __iomem,
    pub fifo: *mut void __iomem,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub pdev: *mut platform_device,
    pub errata: *const usba_udc_errata,
    pub irq: c_int,
    pub vbus_pin: *mut gpio_desc,
    pub num_ep: c_int,
    pub fifo_cfg: *mut usba_fifo_cfg,
    pub pclk: *mut clk,
    pub hclk: *mut clk,
    pub usba_ep: *mut usba_ep,
    pub bias_pulse_needed: bool,
    pub clocked: bool,
    pub suspended: bool,
    pub ep_prealloc: bool,
    pub devstatus: u16,
    pub test_mode: u16,
    pub vbus_prev: c_int,
    pub int_enb_cache: u32,

    pub debugfs_root: *mut dentry,

    pub pmc: *mut regmap,
}

extern "C" {
    pub fn container_of(_arg: ep, usba_ep: struct, _arg: ep) -> return;
}
extern "C" {
    pub fn container_of(_arg: req, usba_request: struct, _arg: req) -> return;
}
extern "C" {
    pub fn container_of(_arg: gadget, usba_udc: struct, _arg: gadget) -> return;
}

