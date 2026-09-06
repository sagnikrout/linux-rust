//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/bdc/bdc.h
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
// bdc.h - header for the BRCM BDC USB3.0 device controller
//
// Copyright (C) 2014 Broadcom Corporation
//
// Author: Ashwini Pahuja
//

// BDC command operation timeout in usec
pub const BDC_CMD_TIMEOUT: c_int = 1000;
// BDC controller operation timeout in usec
pub const BDC_COP_TIMEOUT: c_int = 500;
//
// Maximum size of ep0 response buffer for ch9 requests,
// the set_sel request uses 6 so far, the max.
//
pub const EP0_RESPONSE_BUFF: c_int = 6;
// Start with SS as default
pub const EP0_MAX_PKT_SIZE: c_int = 512;
// 64 entries in a SRR
pub const NUM_SR_ENTRIES: c_int = 64;
// Num of bds per table
pub const NUM_BDS_PER_TABLE: c_int = 64;
// Num of tables in bd list for control,bulk and Int ep
pub const NUM_TABLES: c_int = 2;
// Num of tables in bd list for Isoch ep
pub const NUM_TABLES_ISOCH: c_int = 6;
// U1 Timeout default: 248usec
pub const U1_TIMEOUT: c_uint = 0xf8;
// Interrupt coalescence in usec
pub const INT_CLS: c_int = 500;
// Register offsets
// Configuration and Capability registers
pub const BDC_BDCCFG0: c_uint = 0x00;
pub const BDC_BDCCFG1: c_uint = 0x04;
pub const BDC_BDCCAP0: c_uint = 0x08;
pub const BDC_BDCCAP1: c_uint = 0x0c;
pub const BDC_CMDPAR0: c_uint = 0x10;
pub const BDC_CMDPAR1: c_uint = 0x14;
pub const BDC_CMDPAR2: c_uint = 0x18;
pub const BDC_CMDSC: c_uint = 0x1c;
pub const BDC_USPC: c_uint = 0x20;
pub const BDC_USPPMS: c_uint = 0x28;
pub const BDC_USPPM2: c_uint = 0x2c;
pub const BDC_SPBBAL: c_uint = 0x38;
pub const BDC_SPBBAH: c_uint = 0x3c;
pub const BDC_BDCSC: c_uint = 0x40;
pub const BDC_XSFNTF: c_uint = 0x4c;
pub const BDC_DVCSA: c_uint = 0x50;
pub const BDC_DVCSB: c_uint = 0x54;
pub const BDC_EPSTS0: c_uint = 0x60;
pub const BDC_EPSTS1: c_uint = 0x64;
pub const BDC_EPSTS2: c_uint = 0x68;
pub const BDC_EPSTS3: c_uint = 0x6c;
pub const BDC_EPSTS4: c_uint = 0x70;
pub const BDC_EPSTS5: c_uint = 0x74;
pub const BDC_EPSTS6: c_uint = 0x78;
pub const BDC_EPSTS7: c_uint = 0x7c;

// Extended capability regs
pub const BDC_FSCNOC: c_uint = 0xcd4;
pub const BDC_FSCNIC: c_uint = 0xce4;

// Register bit fields and Masks
// BDC Configuration 0

// BDC Capability1

// BDC Command register
pub const BDC_CMD_FH: c_uint = 0xe;
pub const BDC_CMD_DNC: c_uint = 0x6;
pub const BDC_CMD_EPO: c_uint = 0x4;
pub const BDC_CMD_BLA: c_uint = 0x3;
pub const BDC_CMD_EPC: c_uint = 0x2;
pub const BDC_CMD_DVC: c_uint = 0x1;

// Reset sequence number

// CMD completion status
pub const BDC_CMDS_SUCC: c_uint = 0x1;
pub const BDC_CMDS_PARA: c_uint = 0x3;
pub const BDC_CMDS_STAT: c_uint = 0x4;
pub const BDC_CMDS_FAIL: c_uint = 0x5;
pub const BDC_CMDS_INTL: c_uint = 0x6;
pub const BDC_CMDS_BUSY: c_uint = 0xf;
// CMDSC Param 2 shifts
pub const EPT_SHIFT: c_int = 22;
pub const MP_SHIFT: c_int = 10;
pub const MB_SHIFT: c_int = 6;
pub const EPM_SHIFT: c_int = 4;
// BDC USPSC

pub const BDC_SPEED_FS: c_uint = 0x1;
pub const BDC_SPEED_LS: c_uint = 0x2;
pub const BDC_SPEED_HS: c_uint = 0x3;
pub const BDC_SPEED_SS: c_uint = 0x4;

pub const BDC_PST_MASK: c_uint = 0xf;
// USPPMS

pub const BDC_U1T_MASK: c_uint = 0xff;
// USBPM2
// Hardware LPM Enable

// BDC Status and Control

pub const BDC_HLT: c_int = 1;
pub const BDC_NOR: c_int = 2;
pub const BDC_OIP: c_int = 7;
// Buffer descriptor and Status report bit fields and masks

pub const BD_CHAIN: c_uint = 0xf;
pub const BD_TFS_SHIFT: c_int = 4;

pub const BDC_SRR_DPI_MASK: c_uint = 0x00ff0000;

// Control transfer BD specific fields

pub const BDC_PTC_MASK: c_uint = 0xf0000000;
// status report defines
pub const SR_XSF: c_int = 0;
pub const SR_USPC: c_int = 4;

pub const XSF_SUCC: c_uint = 0x1;
pub const XSF_SHORT: c_uint = 0x3;
pub const XSF_BABB: c_uint = 0x4;
pub const XSF_SETUP_RECV: c_uint = 0x6;
pub const XSF_DATA_START: c_uint = 0x7;
pub const XSF_STATUS_START: c_uint = 0x8;

// Transfer BD fields

pub const BD_TYPE_DS: c_uint = 0x1;
pub const BD_TYPE_SS: c_uint = 0x2;

// One BD can transfer max 65536 bytes

// Maximum bytes in one XFR, Refer to BDC spec
pub const MAX_XFR_LEN: c_int = 16777215;
// defines for Force Header command
pub const DEV_NOTF_TYPE: c_int = 6;
pub const FWK_SUBTYPE: c_int = 1;
pub const TRA_PACKET: c_int = 4;

// FUNCTION WAKE DEV NOTIFICATION interval, USB3 spec table 8.13

// Devstatus bitfields

// On disconnect, preserve these bits and clear rest

// Hardware and software Data structures
// Endpoint bd: buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_bd {
    pub offset: [__le32; 4],
}

// Status report in Status report ring(srr)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_sr {
    pub offset: [__le32; 4],
}

// bd_table: contiguous bd's in a table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_table {
    pub start_bd: *mut bdc_bd,
// dma address of start bd of table
    pub dma: dma_addr_t,
}

//
// Each endpoint has a bdl(buffer descriptor list), bdl consists of 1 or more bd
// table's chained to each other through a chain bd, every table has equal
// number of bds. the software uses bdi(bd index) to refer to particular bd in
// the list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_list {
// Array of bd table pointers
    pub bd_table_array: *mut bd_table,
// How many tables chained to each other
    pub num_tabs: c_int,
// Max_bdi = num_tabs * num_bds_table - 1
    pub max_bdi: c_int,
// current enq bdi from sw point of view
    pub eqp_bdi: c_int,
// current deq bdi from sw point of view
    pub hwd_bdi: c_int,
// numbers of bds per table
    pub num_bds_table: c_int,
}

// Representation of a transfer, one transfer can have multiple bd's
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_transfer {
    pub req: *mut bdc_req,
// start bd index
    pub start_bdi: c_int,
// this will be the next hw dqp when this transfer completes
    pub next_hwd_bdi: c_int,
// number of bds in this transfer
    pub num_bds: c_int,
}

//
// Representation of a gadget request, every gadget request is contained
// by 1 bd_transfer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_req {
    pub usb_req: usb_request,
    pub queue: list_head,
    pub ep: *mut bdc_ep,
// only one Transfer per request
    pub bd_xfr: bd_transfer,
    pub epnum: c_int,
}

// scratchpad buffer needed by bdc hardware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_scratchpad {
    pub sp_dma: dma_addr_t,
    pub buff: *mut c_void,
    pub size: u32,
}

// endpoint representation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_ep {
    pub usb_ep: usb_ep,
    pub queue: list_head,
    pub bdc: *mut bdc,
    pub ep_type: u8,
    pub dir: u8,
    pub ep_num: u8,
    pub comp_desc: *const usb_ss_ep_comp_descriptor,
    pub desc: *const usb_endpoint_descriptor,
    pub flags: c_uint,
    pub name: [c_char; 20],
// endpoint bd list
    pub bd_list: bd_list,
//
// HW generates extra event for multi bd tranfers, this flag helps in
// ignoring the extra event
//
    pub ignore_next_sr: bool,
}

// bdc cmmand parameter structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc_cmd_params {
    pub param2: u32,
    pub param1: u32,
    pub param0: u32,
}

// status report ring(srr), currently one srr is supported for entire system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srr {
    pub sr_bds: *mut bdc_sr,
    pub eqp_index: u16,
    pub dqp_index: u16,
    pub dma_addr: dma_addr_t,
}

// EP0 states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bdc_ep0_state {
    WAIT_FOR_SETUP = 0,
    WAIT_FOR_DATA_START,
    WAIT_FOR_DATA_XMIT,
    WAIT_FOR_STATUS_START,
    WAIT_FOR_STATUS_XMIT,
    STATUS_PENDING
}

// Link states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bdc_link_state {
    BDC_LINK_STATE_U0	= 0x00,
    BDC_LINK_STATE_U3	= 0x03,
    BDC_LINK_STATE_RX_DET	= 0x05,
    BDC_LINK_STATE_RESUME	= 0x0f
}

// representation of bdc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdc {
    pub gadget: usb_gadget,
    pub gadget_driver: *mut usb_gadget_driver,
    pub dev: *mut device,
// device lock
    pub lock: spinlock_t,
// generic phy
    pub phys: *mut phy,
    pub num_phys: c_int,
// num of endpoints for a particular instantiation of IP
    pub num_eps: c_uint,
//
// Array of ep's, it uses the same index covention as bdc hw i.e.
// 1 for ep0, 2 for 1out,3 for 1in ....
//
    pub bdc_ep_array: *mut bdc_ep,
    pub regs: *mut void __iomem,
    pub scratchpad: bdc_scratchpad,
    pub sp_buff_size: u32,
// current driver supports 1 status ring
    pub srr: srr,
// Last received setup packet
    pub setup_pkt: usb_ctrlrequest,
    pub ep0_req: bdc_req,
    pub status_req: bdc_req,
    pub ep0_state: bdc_ep0_state,
    pub delayed_status: bool,
    pub zlp_needed: bool,
    pub reinit: bool,
    pub pullup: bool,
// Bits 0-15 are standard and 16-31 for proprietary information
    pub devstatus: u32,
    pub irq: c_int,
    pub mem: *mut c_void,
    pub dev_addr: u32,
// DMA pools
    pub bd_table_pool: *mut dma_pool,
    pub test_mode: u8,
// array of callbacks for various status report handlers
    pub ): *mut *mut *mut void (sr_handler[2])(struct bdc , struct bdc_sr,
// ep0 callback handlers
    pub ): *mut *mut *mut void (sr_xsf_ep0[3])(struct bdc , struct bdc_sr,
// ep0 response buffer for ch9 requests like GET_STATUS and SET_SEL
    pub ep0_response_buff: [c_uchar; EP0_RESPONSE_BUFF],
//
// Timer to check if host resumed transfer after bdc sent Func wake
// notification  packet after a remote wakeup. if not, then resend the
// Func Wake packet every 2.5 secs. Refer to USB3 spec section 8.5.6.4
//
    pub func_wake_notify: delayed_work,
    pub clk: *mut clk,
}

extern "C" {
    pub fn readl(offset: base +) -> return;
}
// Buffer descriptor list operations
extern "C" {
    pub fn bdc_notify_xfr(bdc: *mut bdc, epnum: u32);
}
extern "C" {
    pub fn bdc_softconn(bdc: *mut bdc);
}
extern "C" {
    pub fn bdc_softdisconn(bdc: *mut bdc);
}
extern "C" {
    pub fn bdc_run(bdc: *mut bdc) -> c_int;
}
extern "C" {
    pub fn bdc_stop(bdc: *mut bdc) -> c_int;
}
extern "C" {
    pub fn bdc_reset(bdc: *mut bdc) -> c_int;
}
extern "C" {
    pub fn bdc_udc_init(bdc: *mut bdc) -> c_int;
}
extern "C" {
    pub fn bdc_udc_exit(bdc: *mut bdc);
}
extern "C" {
    pub fn bdc_reinit(bdc: *mut bdc) -> c_int;
}
// Status report handlers
// Upstream port status change sr
extern "C" {
    pub fn bdc_sr_uspc(bdc: *mut bdc, sreport: *mut bdc_sr);
}
// transfer sr
extern "C" {
    pub fn bdc_sr_xsf(bdc: *mut bdc, sreport: *mut bdc_sr);
}
// EP0 XSF handlers
extern "C" {
    pub fn bdc_xsf_ep0_setup_recv(bdc: *mut bdc, sreport: *mut bdc_sr);
}
extern "C" {
    pub fn bdc_xsf_ep0_data_start(bdc: *mut bdc, sreport: *mut bdc_sr);
}
extern "C" {
    pub fn bdc_xsf_ep0_status_start(bdc: *mut bdc, sreport: *mut bdc_sr);
}
