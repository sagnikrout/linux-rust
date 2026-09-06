//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc3/core.h
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
// core.h - DesignWare USB3 DRD Core Header
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

//
// DWC3 Multiport controllers support up to 15 High-Speed PHYs
// and 4 SuperSpeed PHYs.
//
pub const DWC3_USB2_MAX_PORTS: c_int = 15;
pub const DWC3_USB3_MAX_PORTS: c_int = 4;
pub const DWC3_MSG_MAX: c_int = 500;
// Global constants

pub const DWC3_EP0_SETUP_SIZE: c_int = 512;
pub const DWC3_ENDPOINTS_NUM: c_int = 32;
pub const DWC3_XHCI_RESOURCES_NUM: c_int = 2;
pub const DWC3_ISOC_MAX_RETRIES: c_int = 5;

pub const DWC3_EVENT_BUFFERS_SIZE: c_int = 4096;
pub const DWC3_EVENT_TYPE_MASK: c_uint = 0xfe;
pub const DWC3_EVENT_TYPE_DEV: c_int = 0;
pub const DWC3_EVENT_TYPE_CARKIT: c_int = 3;
pub const DWC3_EVENT_TYPE_I2C: c_int = 4;
pub const DWC3_DEVICE_EVENT_DISCONNECT: c_int = 0;
pub const DWC3_DEVICE_EVENT_RESET: c_int = 1;
pub const DWC3_DEVICE_EVENT_CONNECT_DONE: c_int = 2;
pub const DWC3_DEVICE_EVENT_LINK_STATUS_CHANGE: c_int = 3;
pub const DWC3_DEVICE_EVENT_WAKEUP: c_int = 4;
pub const DWC3_DEVICE_EVENT_HIBER_REQ: c_int = 5;
pub const DWC3_DEVICE_EVENT_SUSPEND: c_int = 6;
pub const DWC3_DEVICE_EVENT_SOF: c_int = 7;
pub const DWC3_DEVICE_EVENT_ERRATIC_ERROR: c_int = 9;
pub const DWC3_DEVICE_EVENT_CMD_CMPL: c_int = 10;
pub const DWC3_DEVICE_EVENT_OVERFLOW: c_int = 11;
// Controller's role while using the OTG block
pub const DWC3_OTG_ROLE_IDLE: c_int = 0;
pub const DWC3_OTG_ROLE_HOST: c_int = 1;
pub const DWC3_OTG_ROLE_DEVICE: c_int = 2;
pub const DWC3_GEVNTCOUNT_MASK: c_uint = 0xfffc;

pub const DWC3_GSNPSID_MASK: c_uint = 0xffff0000;
pub const DWC3_GSNPSREV_MASK: c_uint = 0xffff;

// DWC3 registers memory space boundaries
pub const DWC3_XHCI_REGS_START: c_uint = 0x0;
pub const DWC3_XHCI_REGS_END: c_uint = 0x7fff;
pub const DWC3_GLOBALS_REGS_START: c_uint = 0xc100;
pub const DWC3_GLOBALS_REGS_END: c_uint = 0xc6ff;
pub const DWC3_DEVICE_REGS_START: c_uint = 0xc700;
pub const DWC3_DEVICE_REGS_END: c_uint = 0xcbff;
pub const DWC3_OTG_REGS_START: c_uint = 0xcc00;
pub const DWC3_OTG_REGS_END: c_uint = 0xccff;
pub const DWC3_RTK_RTD_GLOBALS_REGS_START: c_uint = 0x8100;
// Global Registers
pub const DWC3_GSBUSCFG0: c_uint = 0xc100;
pub const DWC3_GSBUSCFG1: c_uint = 0xc104;
pub const DWC3_GTXTHRCFG: c_uint = 0xc108;
pub const DWC3_GRXTHRCFG: c_uint = 0xc10c;
pub const DWC3_GCTL: c_uint = 0xc110;
pub const DWC3_GEVTEN: c_uint = 0xc114;
pub const DWC3_GSTS: c_uint = 0xc118;
pub const DWC3_GUCTL1: c_uint = 0xc11c;
pub const DWC3_GSNPSID: c_uint = 0xc120;
pub const DWC3_GGPIO: c_uint = 0xc124;
pub const DWC3_GUID: c_uint = 0xc128;
pub const DWC3_GUCTL: c_uint = 0xc12c;
pub const DWC3_GBUSERRADDR0: c_uint = 0xc130;
pub const DWC3_GBUSERRADDR1: c_uint = 0xc134;
pub const DWC3_GPRTBIMAP0: c_uint = 0xc138;
pub const DWC3_GPRTBIMAP1: c_uint = 0xc13c;
pub const DWC3_GHWPARAMS0: c_uint = 0xc140;
pub const DWC3_GHWPARAMS1: c_uint = 0xc144;
pub const DWC3_GHWPARAMS2: c_uint = 0xc148;
pub const DWC3_GHWPARAMS3: c_uint = 0xc14c;
pub const DWC3_GHWPARAMS4: c_uint = 0xc150;
pub const DWC3_GHWPARAMS5: c_uint = 0xc154;
pub const DWC3_GHWPARAMS6: c_uint = 0xc158;
pub const DWC3_GHWPARAMS7: c_uint = 0xc15c;
pub const DWC3_GDBGFIFOSPACE: c_uint = 0xc160;
pub const DWC3_GDBGLTSSM: c_uint = 0xc164;
pub const DWC3_GDBGBMU: c_uint = 0xc16c;
pub const DWC3_GDBGLSPMUX: c_uint = 0xc170;
pub const DWC3_GDBGLSP: c_uint = 0xc174;
pub const DWC3_GDBGEPINFO0: c_uint = 0xc178;
pub const DWC3_GDBGEPINFO1: c_uint = 0xc17c;
pub const DWC3_GPRTBIMAP_HS0: c_uint = 0xc180;
pub const DWC3_GPRTBIMAP_HS1: c_uint = 0xc184;
pub const DWC3_GPRTBIMAP_FS0: c_uint = 0xc188;
pub const DWC3_GPRTBIMAP_FS1: c_uint = 0xc18c;
pub const DWC3_GUCTL2: c_uint = 0xc19c;
pub const DWC3_VER_NUMBER: c_uint = 0xc1a0;
pub const DWC3_VER_TYPE: c_uint = 0xc1a4;

pub const DWC3_GHWPARAMS8: c_uint = 0xc600;
pub const DWC3_GUCTL3: c_uint = 0xc60c;
pub const DWC3_GFLADJ: c_uint = 0xc630;
pub const DWC3_GHWPARAMS9: c_uint = 0xc6e0;
// Device Registers
pub const DWC3_DCFG: c_uint = 0xc700;
pub const DWC3_DCTL: c_uint = 0xc704;
pub const DWC3_DEVTEN: c_uint = 0xc708;
pub const DWC3_DSTS: c_uint = 0xc70c;
pub const DWC3_DGCMDPAR: c_uint = 0xc710;
pub const DWC3_DGCMD: c_uint = 0xc714;
pub const DWC3_DALEPENA: c_uint = 0xc720;
pub const DWC3_DCFG1: c_uint = 0xc740 /* DWC_usb32 only */;

// OTG Registers
pub const DWC3_OCFG: c_uint = 0xcc00;
pub const DWC3_OCTL: c_uint = 0xcc04;
pub const DWC3_OEVT: c_uint = 0xcc08;
pub const DWC3_OEVTEN: c_uint = 0xcc0C;
pub const DWC3_OSTS: c_uint = 0xcc10;

// Bit fields
// Global SoC Bus Configuration INCRx Register 0

pub const DWC3_GSBUSCFG0_INCRBRST_MASK: c_uint = 0xff;
// Global SoC Bus Configuration Register: AHB-prot/AXI-cache/OCP-ReqInfo

pub const DWC3_GSBUSCFG0_REQINFO_UNSPECIFIED: c_uint = 0xffffffff;
// Global Debug LSP MUX Select

// Global Debug Queue/FIFO Space Available Register

pub const DWC3_TXFIFO: c_int = 0;
pub const DWC3_RXFIFO: c_int = 1;
pub const DWC3_TXREQQ: c_int = 2;
pub const DWC3_RXREQQ: c_int = 3;
pub const DWC3_RXINFOQ: c_int = 4;
pub const DWC3_PSTATQ: c_int = 5;
pub const DWC3_DESCFETCHQ: c_int = 6;
pub const DWC3_EVENTQ: c_int = 7;
pub const DWC3_AUXEVENTQ: c_int = 8;
// Global RX Threshold Configuration Register

// Global TX Threshold Configuration Register

// Global RX Threshold Configuration Register for DWC_usb31 only

// Global TX Threshold Configuration Register for DWC_usb31 only

// Global Configuration Register

pub const DWC3_GCTL_PRTCAP_HOST: c_int = 1;
pub const DWC3_GCTL_PRTCAP_DEVICE: c_int = 2;
pub const DWC3_GCTL_PRTCAP_OTG: c_int = 3;

// Global User Control 1 Register

// Global Status Register

pub const DWC3_GSTS_CURMOD_DEVICE: c_int = 0;
pub const DWC3_GSTS_CURMOD_HOST: c_int = 1;
// Global USB2 PHY Configuration Register

pub const USBTRDTIM_UTMI_8_BIT: c_int = 9;
pub const USBTRDTIM_UTMI_16_BIT: c_int = 5;
pub const UTMI_PHYIF_16_BIT: c_int = 1;
pub const UTMI_PHYIF_8_BIT: c_int = 0;
// Global USB2 PHY Vendor Control Register

// Global USB3 PIPE Control Register

// Global TX Fifo Size Register

// Global RX Fifo Size Register

// Global Event Size Registers

// Global HWPARAMS0 Register

pub const DWC3_GHWPARAMS0_MODE_GADGET: c_int = 0;
pub const DWC3_GHWPARAMS0_MODE_HOST: c_int = 1;
pub const DWC3_GHWPARAMS0_MODE_DRD: c_int = 2;

// Global HWPARAMS1 Register

pub const DWC3_GHWPARAMS1_EN_PWROPT_NO: c_int = 0;
pub const DWC3_GHWPARAMS1_EN_PWROPT_CLK: c_int = 1;
pub const DWC3_GHWPARAMS1_EN_PWROPT_HIB: c_int = 2;

// Global HWPARAMS3 Register

pub const DWC3_GHWPARAMS3_SSPHY_IFC_DIS: c_int = 0;
pub const DWC3_GHWPARAMS3_SSPHY_IFC_GEN1: c_int = 1;

pub const DWC3_GHWPARAMS3_HSPHY_IFC_DIS: c_int = 0;
pub const DWC3_GHWPARAMS3_HSPHY_IFC_UTMI: c_int = 1;
pub const DWC3_GHWPARAMS3_HSPHY_IFC_ULPI: c_int = 2;
pub const DWC3_GHWPARAMS3_HSPHY_IFC_UTMI_ULPI: c_int = 3;

pub const DWC3_GHWPARAMS3_FSPHY_IFC_DIS: c_int = 0;
pub const DWC3_GHWPARAMS3_FSPHY_IFC_ENA: c_int = 1;
// Global HWPARAMS4 Register

pub const DWC3_MAX_HIBER_SCRATCHBUFS: c_int = 15;
// Global HWPARAMS6 Register

// DWC_usb32 only

// Global HWPARAMS7 Register

// Global HWPARAMS9 Register

// Global Frame Length Adjustment Register

pub const DWC3_GFLADJ_30MHZ_MASK: c_uint = 0x3f;

// Global User Control Register
pub const DWC3_GUCTL_REFCLKPER_MASK: c_uint = 0xffc00000;
pub const DWC3_GUCTL_REFCLKPER_SEL: c_int = 22;
// Global User Control Register 2

// Global User Control Register 3

// Device Configuration Register

pub const DWC3_DCFG_NUMP_SHIFT: c_int = 17;

// Device Control Register

// These apply for core versions 1.87a and earlier

// These apply for core versions 1.94a and later

// Device Event Enable Register

// Device Status Register

// This applies for core versions 1.87a and earlier

// These apply for core versions 1.94a and later

// Device Generic Command Register
pub const DWC3_DGCMD_SET_LMP: c_uint = 0x01;
pub const DWC3_DGCMD_SET_PERIODIC_PAR: c_uint = 0x02;
pub const DWC3_DGCMD_XMIT_FUNCTION: c_uint = 0x03;
// These apply for core versions 1.94a and later
pub const DWC3_DGCMD_SET_SCRATCHPAD_ADDR_LO: c_uint = 0x04;
pub const DWC3_DGCMD_SET_SCRATCHPAD_ADDR_HI: c_uint = 0x05;
pub const DWC3_DGCMD_SELECTED_FIFO_FLUSH: c_uint = 0x09;
pub const DWC3_DGCMD_ALL_FIFO_FLUSH: c_uint = 0x0a;
pub const DWC3_DGCMD_SET_ENDPOINT_NRDY: c_uint = 0x0c;
pub const DWC3_DGCMD_SET_ENDPOINT_PRIME: c_uint = 0x0d;
pub const DWC3_DGCMD_RUN_SOC_BUS_LOOPBACK: c_uint = 0x10;
pub const DWC3_DGCMD_DEV_NOTIFICATION: c_uint = 0x07;

// Device Generic Command Parameter Register

// Device Endpoint Command Register
pub const DWC3_DEPCMD_PARAM_SHIFT: c_int = 16;

// This applies for core versions 1.90a and earlier

// This applies for core versions 1.94a and later

// The EP number goes 0..31 so ep0 is always out and ep1 is always in

// DWC_usb32 DCFG1 config

pub const DWC3_DEPCMD_TYPE_CONTROL: c_int = 0;
pub const DWC3_DEPCMD_TYPE_ISOC: c_int = 1;
pub const DWC3_DEPCMD_TYPE_BULK: c_int = 2;
pub const DWC3_DEPCMD_TYPE_INTR: c_int = 3;
pub const DWC3_DEV_IMOD_COUNT_SHIFT: c_int = 16;

pub const DWC3_DEV_IMOD_INTERVAL_SHIFT: c_int = 0;

// OTG Configuration Register

// OTG CTL Register

// OTG Event Register

// OTG Event Enable Register

// OTG Status Register

// Force Gen1 speed on Gen2 link

// Structures
//
// struct dwc3_event_buffer - Software event buffer representation
// @buf: _THE_ buffer
// @cache: The buffer cache used in the threaded interrupt
// @length: size of this buffer
// @lpos: event offset
// @count: cache of last read event count register
// @flags: flags related to this event buffer
// @dma: dma_addr_t
// @dwc: pointer to DWC controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_event_buffer {
    pub buf: *mut c_void,
    pub cache: *mut c_void,
    pub length: c_uint,
    pub lpos: c_uint,
    pub count: c_uint,
    pub flags: c_uint,

    pub dma: dma_addr_t,
    pub dwc: *mut dwc3,
}

pub const DWC3_TRB_NUM: c_int = 256;
//
// struct dwc3_ep - device side endpoint representation
// @endpoint: usb endpoint
// @nostream_work: work for handling bulk NoStream
// @cancelled_list: list of cancelled requests for this endpoint
// @pending_list: list of pending requests for this endpoint
// @started_list: list of started requests on this endpoint
// @trb_pool: array of transaction buffers
// @trb_pool_dma: dma address of @trb_pool
// @trb_enqueue: enqueue 'pointer' into TRB array
// @trb_dequeue: dequeue 'pointer' into TRB array
// @dwc: pointer to DWC controller
// @saved_state: ep state saved during hibernation
// @flags: endpoint flags (wedged, stalled, ...)
// @number: endpoint number (1 - 15)
// @type: set to bmAttributes & USB_ENDPOINT_XFERTYPE_MASK
// @resource_index: Resource transfer index
// @frame_number: set to the frame number we want this transfer to start (ISOC)
// @interval: the interval on which the ISOC transfer is started
// @name: a human readable name e.g. ep1out-bulk
// @direction: true for TX, false for RX
// @stream_capable: true when streams are enabled
// @combo_num: the test combination BIT[15:14] of the frame number to test
// isochronous START TRANSFER command failure workaround
// @start_cmd_status: the status of testing START TRANSFER command with
// combo_num = 'b00
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_ep {
    pub endpoint: usb_ep,
    pub nostream_work: delayed_work,
    pub cancelled_list: list_head,
    pub pending_list: list_head,
    pub started_list: list_head,
    pub trb_pool: *mut dwc3_trb,
    pub trb_pool_dma: dma_addr_t,
    pub dwc: *mut dwc3,
    pub saved_state: u32,
    pub flags: c_uint,

// This last one is specific to EP0

//
// IMPORTANT: we *know* we have 256 TRBs in our @trb_pool, so we will
// use a u8 type here. If anybody decides to increase number of TRBs to
// anything larger than 256 - I can't see why people would want to do
// this though - then this type needs to be changed.
//
// By using u8 types we ensure that our % operator when incrementing
// enqueue and dequeue get optimized away by the compiler.
//
    pub trb_enqueue: u8,
    pub trb_dequeue: u8,
    pub number: u8,
    pub type: u8,
    pub resource_index: u8,
    pub frame_number: u32,
    pub interval: u32,
    pub name: [c_char; 20],
    pub direction:1: unsigned,
    pub stream_capable:1: unsigned,
// For isochronous START TRANSFER workaround only
    pub combo_num: u8,
    pub start_cmd_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc3_phy {
    DWC3_PHY_UNKNOWN = 0,
    DWC3_PHY_USB3,
    DWC3_PHY_USB2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc3_ep0_next {
    DWC3_EP0_UNKNOWN = 0,
    DWC3_EP0_COMPLETE,
    DWC3_EP0_NRDY_DATA,
    DWC3_EP0_NRDY_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc3_ep0_state {
    EP0_UNCONNECTED		= 0,
    EP0_SETUP_PHASE,
    EP0_DATA_PHASE,
    EP0_STATUS_PHASE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc3_link_state {
// In SuperSpeed
    DWC3_LINK_STATE_U0		= 0x00, /* in HS, means ON */
    DWC3_LINK_STATE_U1		= 0x01,
    DWC3_LINK_STATE_U2		= 0x02, /* in HS, means SLEEP */
    DWC3_LINK_STATE_U3		= 0x03, /* in HS, means SUSPEND */
    DWC3_LINK_STATE_SS_DIS		= 0x04,
    DWC3_LINK_STATE_RX_DET		= 0x05, /* in HS, means Early Suspend */
    DWC3_LINK_STATE_SS_INACT	= 0x06,
    DWC3_LINK_STATE_POLL		= 0x07,
    DWC3_LINK_STATE_RECOV		= 0x08,
    DWC3_LINK_STATE_HRESET		= 0x09,
    DWC3_LINK_STATE_CMPLY		= 0x0a,
    DWC3_LINK_STATE_LPBK		= 0x0b,
    DWC3_LINK_STATE_RESET		= 0x0e,
    DWC3_LINK_STATE_RESUME		= 0x0f,
    DWC3_LINK_STATE_MASK		= 0x0f,
}

// TRB Length, PCM and Status

pub const DWC3_TRBSTS_OK: c_int = 0;
pub const DWC3_TRBSTS_MISSED_ISOC: c_int = 1;
pub const DWC3_TRBSTS_SETUP_PENDING: c_int = 2;
pub const DWC3_TRB_STS_XFER_IN_PROG: c_int = 4;
// TRB Control

//
// struct dwc3_trb - transfer request block (hw format)
// @bpl: DW0-3
// @bph: DW4-7
// @size: DW8-B
// @ctrl: DWC-F
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_trb {
    pub bpl: u32,
    pub bph: u32,
    pub size: u32,
    pub ctrl: u32,
    pub __packed: },
//
// struct dwc3_hwparams - copy of HWPARAMS registers
// @hwparams0: GHWPARAMS0
// @hwparams1: GHWPARAMS1
// @hwparams2: GHWPARAMS2
// @hwparams3: GHWPARAMS3
// @hwparams4: GHWPARAMS4
// @hwparams5: GHWPARAMS5
// @hwparams6: GHWPARAMS6
// @hwparams7: GHWPARAMS7
// @hwparams8: GHWPARAMS8
// @hwparams9: GHWPARAMS9
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_hwparams {
    pub hwparams0: u32,
    pub hwparams1: u32,
    pub hwparams2: u32,
    pub hwparams3: u32,
    pub hwparams4: u32,
    pub hwparams5: u32,
    pub hwparams6: u32,
    pub hwparams7: u32,
    pub hwparams8: u32,
    pub hwparams9: u32,
}

// HWPARAMS0

// HWPARAMS1

// HWPARAMS3

// HWPARAMS6

// HWPARAMS7

// HWPARAMS9

//
// struct dwc3_request - representation of a transfer request
// @request: struct usb_request to be transferred
// @list: a list_head used for request queueing
// @dep: struct dwc3_ep owning this request
// @start_sg: pointer to the sg which should be queued next
// @num_pending_sgs: counter to pending sgs
// @remaining: amount of data remaining
// @status: internal dwc3 request status tracking
// @epnum: endpoint number to which this request refers
// @trb: pointer to struct dwc3_trb
// @trb_dma: DMA address of @trb
// @num_trbs: number of TRBs used by this request
// @direction: IN or OUT direction flag
// @mapped: true when request has been dma-mapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_request {
    pub request: usb_request,
    pub list: list_head,
    pub dep: *mut dwc3_ep,
    pub start_sg: *mut scatterlist,
    pub num_pending_sgs: c_uint,
    pub remaining: c_uint,
    pub status: c_uint,
pub const DWC3_REQUEST_STATUS_QUEUED: c_int = 0;
pub const DWC3_REQUEST_STATUS_STARTED: c_int = 1;
pub const DWC3_REQUEST_STATUS_DISCONNECTED: c_int = 2;
pub const DWC3_REQUEST_STATUS_DEQUEUED: c_int = 3;
pub const DWC3_REQUEST_STATUS_STALLED: c_int = 4;
pub const DWC3_REQUEST_STATUS_COMPLETED: c_int = 5;

    pub epnum: u8,
    pub trb: *mut dwc3_trb,
    pub trb_dma: dma_addr_t,
    pub num_trbs: c_uint,
    pub direction:1: c_uint,
    pub mapped:1: c_uint,
}

//
// struct dwc3_scratchpad_array - hibernation scratchpad array
// (format defined by hw)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_scratchpad_array {
    pub dma_adr: [__le64; DWC3_MAX_HIBER_SCRATCHBUFS],
}

//
// struct dwc3_glue_ops - The ops indicate the notifications that
// need to be passed on to glue layer
// @pre_set_role: Notify glue of role switch notifications
// @pre_run_stop: Notify run stop enable/disable information to glue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_glue_ops {
    pub role): *mut *mut *mut void (pre_set_role)(struct dwc3 dwc, enum usb_role,
    pub is_on): *mut *mut *mut void (pre_run_stop)(struct dwc3 dwc, bool,
}

//
// struct dwc3 - representation of our controller
// @drd_work: workqueue used for role swapping
// @ep0_trb: trb which is used for the ctrl_req
// @bounce: address of bounce buffer
// @setup_buf: used while precessing STD USB requests
// @ep0_trb_addr: dma address of @ep0_trb
// @bounce_addr: dma address of @bounce
// @ep0_usb_req: dummy req used while handling STD USB requests
// @ep0_in_setup: one control transfer is completed and enter setup phase
// @lock: for synchronizing
// @mutex: for mode switching
// @dev: pointer to our struct device
// @sysdev: pointer to the DMA-capable device
// @xhci: pointer to our xHCI child
// @xhci_resources: struct resources for our @xhci child
// @ev_buf: struct dwc3_event_buffer pointer
// @eps: endpoint array
// @gadget: device side representation of the peripheral controller
// @gadget_driver: pointer to the gadget driver
// @glue_ops: Vendor callbacks for flattened device implementations.
// @bus_clk: clock for accessing the registers
// @ref_clk: reference clock
// @susp_clk: clock used when the SS phy is in low power (S3) state
// @utmi_clk: clock used for USB2 PHY communication
// @pipe_clk: clock used for USB3 PHY communication
// @reset: reset control
// @regs: base address for our registers
// @regs_size: address space size
// @fladj: frame length adjustment
// @ref_clk_per: reference clock period configuration
// @irq_gadget: peripheral controller's IRQ number
// @otg_irq: IRQ number for OTG IRQs
// @current_otg_role: current role of operation while using the OTG block
// @desired_otg_role: desired role of operation while using the OTG block
// @otg_restart_host: flag that OTG controller needs to restart host
// @u1u2: only used on revisions <1.83a for workaround
// @maximum_speed: maximum speed requested (mainly for testing purposes)
// @max_ssp_rate: SuperSpeed Plus maximum signaling rate and lane count
// @gadget_max_speed: maximum gadget speed requested
// @gadget_ssp_rate: Gadget driver's maximum supported SuperSpeed Plus signaling
// rate and lane count.
// @ip: controller's ID
// @revision: controller's version of an IP
// @version_type: VERSIONTYPE register contents, a sub release of a revision
// @dr_mode: requested mode of operation
// @current_dr_role: current role of operation when in dual-role mode
// @desired_dr_role: desired role of operation when in dual-role mode
// @edev: extcon handle
// @edev_nb: extcon notifier
// @hsphy_mode: UTMI phy mode, one of following:
// - USBPHY_INTERFACE_MODE_UTMI
// - USBPHY_INTERFACE_MODE_UTMIW
// @role_sw: usb_role_switch handle
// @role_switch_default_mode: default operation mode of controller while
// usb role is USB_ROLE_NONE.
// @usb_psy: pointer to power supply interface.
// @usb_psy_name: name of the USB power supply
// @psy_nb: power supply notifier block
// @vbus_draw_work: Work to set the vbus drawing limit
// @current_limit: How much current to draw from vbus, in milliAmperes.
// @usb2_phy: pointer to USB2 PHY
// @usb3_phy: pointer to USB3 PHY
// @usb2_generic_phy: pointer to array of USB2 PHYs
// @usb3_generic_phy: pointer to array of USB3 PHYs
// @num_usb2_ports: number of USB2 ports
// @num_usb3_ports: number of USB3 ports
// @phys_ready: flag to indicate that PHYs are ready
// @ulpi: pointer to ulpi interface
// @ulpi_ready: flag to indicate that ULPI is initialized
// @u2sel: parameter from Set SEL request.
// @u2pel: parameter from Set SEL request.
// @u1sel: parameter from Set SEL request.
// @u1pel: parameter from Set SEL request.
// @num_eps: number of endpoints
// @ep0_next_event: hold the next expected event
// @ep0state: state of endpoint zero
// @link_state: link state
// @speed: device speed (super, high, full, low)
// @hwparams: copy of hwparams registers
// @regset: debugfs pointer to regdump file
// @dbg_lsp_select: current debug lsp mux register selection
// @test_mode: true when we're entering a USB test mode
// @test_mode_nr: test feature selector
// @lpm_nyet_threshold: LPM NYET response threshold
// @hird_threshold: HIRD threshold
// @rx_thr_num_pkt: USB receive packet count
// @rx_max_burst: max USB receive burst size
// @tx_thr_num_pkt: USB transmit packet count
// @tx_max_burst: max USB transmit burst size
// @rx_thr_num_pkt_prd: periodic ESS receive packet count
// @rx_max_burst_prd: max periodic ESS receive burst size
// @tx_thr_num_pkt_prd: periodic ESS transmit packet count
// @tx_max_burst_prd: max periodic ESS transmit burst size
// @tx_fifo_resize_max_num: max number of fifos allocated during txfifo resize
// @clear_stall_protocol: endpoint number that requires a delayed status phase
// @num_hc_interrupters: number of host controller interrupters
// @hsphy_interface: "utmi" or "ulpi"
// @connected: true when we're connected to a host, false otherwise
// @softconnect: true when gadget connect is called, false when disconnect runs
// @delayed_status: true when gadget driver asks for delayed status
// @ep0_bounced: true when we used bounce buffer
// @ep0_expect_in: true when we expect a DATA IN transfer
// @sysdev_is_parent: true when dwc3 device has a parent driver
// @has_lpm_erratum: true when core was configured with LPM Erratum. Note that
// there's now way for software to detect this in runtime.
// @is_utmi_l1_suspend: the core asserts output signal
// 0	- utmi_sleep_n
// 1	- utmi_l1_suspend_n
// @is_fpga: true when we are using the FPGA board
// @pending_events: true when we have pending IRQs to be handled
// @do_fifo_resize: true when txfifo resizing is enabled for dwc3 endpoints
// @pullups_connected: true when Run/Stop bit is set
// @setup_packet_pending: true when there's a Setup Packet in FIFO. Workaround
// @three_stage_setup: set if we perform a three phase setup
// @dis_start_transfer_quirk: set if start_transfer failure SW workaround is
// not needed for DWC_usb31 version 1.70a-ea06 and below
// @usb3_lpm_capable: set if hadrware supports Link Power Management
// @usb2_lpm_disable: set to disable usb2 lpm for host
// @usb2_gadget_lpm_disable: set to disable usb2 lpm for gadget
// @needs_full_reinit: set to indicate the core may lose power and need full
// initialization during system pm
// @disable_scramble_quirk: set if we enable the disable scramble quirk
// @u2exit_lfps_quirk: set if we enable u2exit lfps quirk
// @u2ss_inp3_quirk: set if we enable P3 OK for U2/SS Inactive quirk
// @req_p1p2p3_quirk: set if we enable request p1p2p3 quirk
// @del_p1p2p3_quirk: set if we enable delay p1p2p3 quirk
// @del_phy_power_chg_quirk: set if we enable delay phy power change quirk
// @lfps_filter_quirk: set if we enable LFPS filter quirk
// @rx_detect_poll_quirk: set if we enable rx_detect to polling lfps quirk
// @dis_u3_susphy_quirk: set if we disable usb3 suspend phy
// @dis_u2_susphy_quirk: set if we disable usb2 suspend phy
// @dis_enblslpm_quirk: set if we clear enblslpm in GUSB2PHYCFG,
// disabling the suspend signal to the PHY.
// @dis_u1_entry_quirk: set if link entering into U1 state needs to be disabled.
// @dis_u2_entry_quirk: set if link entering into U2 state needs to be disabled.
// @dis_rxdet_inp3_quirk: set if we disable Rx.Detect in P3
// @async_callbacks: if set, indicate that async callbacks will be used.
//
// @dis_u2_freeclk_exists_quirk : set if we clear u2_freeclk_exists
// in GUSB2PHYCFG, specify that USB2 PHY doesn't
// provide a free-running PHY clock.
// @dis_del_phy_power_chg_quirk: set if we disable delay phy power
// change quirk.
// @dis_tx_ipgap_linecheck_quirk: set if we disable u2mac linestate
// check during HS transmit.
// @resume_hs_terminations: Set if we enable quirk for fixing improper crc
// generation after resume from suspend.
// @ulpi_ext_vbus_drv: Set to confiure the upli chip to drives CPEN pin
// VBUS with an external supply.
// @parkmode_disable_ss_quirk: set if we need to disable all SuperSpeed
// instances in park mode.
// @parkmode_disable_hs_quirk: set if we need to disable all HighSpeed
// instances in park mode.
// @gfladj_refclk_lpm_sel: set if we need to enable SOF/ITP counter
// running based on ref_clk
// @tx_de_emphasis_quirk: set if we enable Tx de-emphasis quirk
// @tx_de_emphasis: Tx de-emphasis value
// 0	- -6dB de-emphasis
// 1	- -3.5dB de-emphasis
// 2	- No de-emphasis
// 3	- Reserved
// @dis_metastability_quirk: set to disable metastability quirk.
// @dis_split_quirk: set to disable split boundary.
// @enable_usb2_transceiver_delay: Set to insert a delay before the
// assertion of the TxValid signal during a HS Chirp.
// @sys_wakeup: set if the device may do system wakeup.
// @wakeup_configured: set if the device is configured for remote wakeup.
// @suspended: set to track suspend event due to U3/L2.
// @susphy_state: state of DWC3_GUSB2PHYCFG_SUSPHY + DWC3_GUSB3PIPECTL_SUSPHY
// before PM suspend.
// @imod_interval: set the interrupt moderation interval in 250ns
// increments or 0 to disable.
// @max_cfg_eps: current max number of IN eps used across all USB configs.
// @last_fifo_depth: last fifo depth used to determine next fifo ram start
// address.
// @num_ep_resized: carries the current number endpoints which have had its tx
// fifo resized.
// @debug_root: root debugfs directory for this device to put its files in.
// @gsbuscfg0_reqinfo: store GSBUSCFG0.DATRDREQINFO, DESRDREQINFO,
// DATWRREQINFO, and DESWRREQINFO value passed from
// glue driver.
// @wakeup_pending_funcs: Indicates whether any interface has requested for
// function wakeup in bitmap format where bit position
// represents interface_id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3 {
    pub drd_work: work_struct,
    pub ep0_trb: *mut dwc3_trb,
    pub bounce: *mut c_void,
    pub setup_buf: *mut u8,
    pub ep0_trb_addr: dma_addr_t,
    pub bounce_addr: dma_addr_t,
    pub ep0_usb_req: dwc3_request,
    pub ep0_in_setup: completion,
// device lock
    pub lock: spinlock_t,
// mode switching lock
    pub mutex: mutex,
    pub dev: *mut device,
    pub sysdev: *mut device,
    pub xhci: *mut platform_device,
    pub xhci_resources: [resource; DWC3_XHCI_RESOURCES_NUM],
    pub ev_buf: *mut dwc3_event_buffer,
    pub eps: [*mut dwc3_ep; DWC3_ENDPOINTS_NUM],
    pub gadget: *mut usb_gadget,
    pub gadget_driver: *mut usb_gadget_driver,
    pub glue_ops: *const dwc3_glue_ops,
    pub bus_clk: *mut clk,
    pub ref_clk: *mut clk,
    pub susp_clk: *mut clk,
    pub utmi_clk: *mut clk,
    pub pipe_clk: *mut clk,
    pub reset: *mut reset_control,
    pub usb2_phy: *mut usb_phy,
    pub usb3_phy: *mut usb_phy,
    pub usb2_generic_phy: [*mut phy; DWC3_USB2_MAX_PORTS],
    pub usb3_generic_phy: [*mut phy; DWC3_USB3_MAX_PORTS],
    pub num_usb2_ports: u8,
    pub num_usb3_ports: u8,
    pub phys_ready: bool,
    pub ulpi: *mut ulpi,
    pub ulpi_ready: bool,
    pub regs: *mut void __iomem,
    pub regs_size: usize,
    pub dr_mode: usb_dr_mode,
    pub current_dr_role: u32,
    pub desired_dr_role: u32,
    pub edev: *mut extcon_dev,
    pub edev_nb: notifier_block,
    pub hsphy_mode: usb_phy_interface,
    pub role_sw: *mut usb_role_switch,
    pub role_switch_default_mode: usb_dr_mode,
    pub usb_psy: *mut power_supply,
    pub usb_psy_name: *const c_char,
    pub psy_nb: notifier_block,
    pub vbus_draw_work: work_struct,
    pub current_limit: c_uint,

    pub fladj: u32,
    pub ref_clk_per: u32,
    pub irq_gadget: u32,
    pub otg_irq: u32,
    pub current_otg_role: u32,
    pub desired_otg_role: u32,
    pub otg_restart_host: bool,
    pub u1u2: u32,
    pub maximum_speed: u32,
    pub gadget_max_speed: u32,
    pub max_ssp_rate: usb_ssp_rate,
    pub gadget_ssp_rate: usb_ssp_rate,
    pub ip: u32,
pub const DWC3_IP: c_uint = 0x5533;
pub const DWC31_IP: c_uint = 0x3331;
pub const DWC32_IP: c_uint = 0x3332;
pub const DWC4_IP: c_uint = 0x3430;
    pub revision: u32,
pub const DWC3_REVISION_ANY: c_uint = 0x0;
pub const DWC3_REVISION_173A: c_uint = 0x5533173a;
pub const DWC3_REVISION_175A: c_uint = 0x5533175a;
pub const DWC3_REVISION_180A: c_uint = 0x5533180a;
pub const DWC3_REVISION_183A: c_uint = 0x5533183a;
pub const DWC3_REVISION_185A: c_uint = 0x5533185a;
pub const DWC3_REVISION_187A: c_uint = 0x5533187a;
pub const DWC3_REVISION_188A: c_uint = 0x5533188a;
pub const DWC3_REVISION_190A: c_uint = 0x5533190a;
pub const DWC3_REVISION_194A: c_uint = 0x5533194a;
pub const DWC3_REVISION_200A: c_uint = 0x5533200a;
pub const DWC3_REVISION_202A: c_uint = 0x5533202a;
pub const DWC3_REVISION_210A: c_uint = 0x5533210a;
pub const DWC3_REVISION_220A: c_uint = 0x5533220a;
pub const DWC3_REVISION_230A: c_uint = 0x5533230a;
pub const DWC3_REVISION_240A: c_uint = 0x5533240a;
pub const DWC3_REVISION_250A: c_uint = 0x5533250a;
pub const DWC3_REVISION_260A: c_uint = 0x5533260a;
pub const DWC3_REVISION_270A: c_uint = 0x5533270a;
pub const DWC3_REVISION_280A: c_uint = 0x5533280a;
pub const DWC3_REVISION_290A: c_uint = 0x5533290a;
pub const DWC3_REVISION_300A: c_uint = 0x5533300a;
pub const DWC3_REVISION_310A: c_uint = 0x5533310a;
pub const DWC3_REVISION_320A: c_uint = 0x5533320a;
pub const DWC3_REVISION_330A: c_uint = 0x5533330a;
pub const DWC31_REVISION_ANY: c_uint = 0x0;
pub const DWC31_REVISION_110A: c_uint = 0x3131302a;
pub const DWC31_REVISION_120A: c_uint = 0x3132302a;
pub const DWC31_REVISION_160A: c_uint = 0x3136302a;
pub const DWC31_REVISION_170A: c_uint = 0x3137302a;
pub const DWC31_REVISION_180A: c_uint = 0x3138302a;
pub const DWC31_REVISION_190A: c_uint = 0x3139302a;
pub const DWC31_REVISION_200A: c_uint = 0x3230302a;
pub const DWC32_REVISION_ANY: c_uint = 0x0;
pub const DWC32_REVISION_100A: c_uint = 0x3130302a;
    pub version_type: u32,
pub const DWC31_VERSIONTYPE_ANY: c_uint = 0x0;
pub const DWC31_VERSIONTYPE_EA01: c_uint = 0x65613031;
pub const DWC31_VERSIONTYPE_EA02: c_uint = 0x65613032;
pub const DWC31_VERSIONTYPE_EA03: c_uint = 0x65613033;
pub const DWC31_VERSIONTYPE_EA04: c_uint = 0x65613034;
pub const DWC31_VERSIONTYPE_EA05: c_uint = 0x65613035;
pub const DWC31_VERSIONTYPE_EA06: c_uint = 0x65613036;
    pub ep0_next_event: dwc3_ep0_next,
    pub ep0state: dwc3_ep0_state,
    pub link_state: dwc3_link_state,
    pub u2sel: u16,
    pub u2pel: u16,
    pub u1sel: u8,
    pub u1pel: u8,
    pub speed: u8,
    pub num_eps: u8,
    pub hwparams: dwc3_hwparams,
    pub regset: *mut debugfs_regset32,
    pub dbg_lsp_select: u32,
    pub test_mode: u8,
    pub test_mode_nr: u8,
    pub lpm_nyet_threshold: u8,
    pub hird_threshold: u8,
    pub rx_thr_num_pkt: u8,
    pub rx_max_burst: u8,
    pub tx_thr_num_pkt: u8,
    pub tx_max_burst: u8,
    pub rx_thr_num_pkt_prd: u8,
    pub rx_max_burst_prd: u8,
    pub tx_thr_num_pkt_prd: u8,
    pub tx_max_burst_prd: u8,
    pub tx_fifo_resize_max_num: u8,
    pub clear_stall_protocol: u8,
    pub num_hc_interrupters: u16,
    pub hsphy_interface: *const c_char,
    pub connected:1: unsigned,
    pub softconnect:1: unsigned,
    pub delayed_status:1: unsigned,
    pub ep0_bounced:1: unsigned,
    pub ep0_expect_in:1: unsigned,
    pub sysdev_is_parent:1: unsigned,
    pub has_lpm_erratum:1: unsigned,
    pub is_utmi_l1_suspend:1: unsigned,
    pub is_fpga:1: unsigned,
    pub pending_events:1: unsigned,
    pub do_fifo_resize:1: unsigned,
    pub pullups_connected:1: unsigned,
    pub setup_packet_pending:1: unsigned,
    pub three_stage_setup:1: unsigned,
    pub dis_start_transfer_quirk:1: unsigned,
    pub usb3_lpm_capable:1: unsigned,
    pub usb2_lpm_disable:1: unsigned,
    pub usb2_gadget_lpm_disable:1: unsigned,
    pub needs_full_reinit:1: unsigned,
    pub disable_scramble_quirk:1: unsigned,
    pub u2exit_lfps_quirk:1: unsigned,
    pub u2ss_inp3_quirk:1: unsigned,
    pub req_p1p2p3_quirk:1: unsigned,
    pub del_p1p2p3_quirk:1: unsigned,
    pub del_phy_power_chg_quirk:1: unsigned,
    pub lfps_filter_quirk:1: unsigned,
    pub rx_detect_poll_quirk:1: unsigned,
    pub dis_u3_susphy_quirk:1: unsigned,
    pub dis_u2_susphy_quirk:1: unsigned,
    pub dis_enblslpm_quirk:1: unsigned,
    pub dis_u1_entry_quirk:1: unsigned,
    pub dis_u2_entry_quirk:1: unsigned,
    pub dis_rxdet_inp3_quirk:1: unsigned,
    pub dis_u2_freeclk_exists_quirk:1: unsigned,
    pub dis_del_phy_power_chg_quirk:1: unsigned,
    pub dis_tx_ipgap_linecheck_quirk:1: unsigned,
    pub resume_hs_terminations:1: unsigned,
    pub ulpi_ext_vbus_drv:1: unsigned,
    pub parkmode_disable_ss_quirk:1: unsigned,
    pub parkmode_disable_hs_quirk:1: unsigned,
    pub gfladj_refclk_lpm_sel:1: unsigned,
    pub tx_de_emphasis_quirk:1: unsigned,
    pub tx_de_emphasis:2: unsigned,
    pub dis_metastability_quirk:1: unsigned,
    pub dis_split_quirk:1: unsigned,
    pub enable_usb2_transceiver_delay:1: unsigned,
    pub async_callbacks:1: unsigned,
    pub sys_wakeup:1: unsigned,
    pub wakeup_configured:1: unsigned,
    pub suspended:1: unsigned,
    pub susphy_state:1: unsigned,
    pub imod_interval: u16,
    pub max_cfg_eps: c_int,
    pub last_fifo_depth: c_int,
    pub num_ep_resized: c_int,
    pub debug_root: *mut dentry,
    pub gsbuscfg0_reqinfo: u32,
    pub wakeup_pending_funcs: u32,
}

pub const INCRX_BURST_MODE: c_int = 0;
pub const INCRX_UNDEF_LENGTH_BURST_MODE: c_int = 1;

// --------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_event_type {
    pub is_devspec:1: u32,
    pub type:7: u32,
    pub reserved8_31:24: u32,
    pub __packed: },
pub const DWC3_DEPEVT_XFERCOMPLETE: c_uint = 0x01;
pub const DWC3_DEPEVT_XFERINPROGRESS: c_uint = 0x02;
pub const DWC3_DEPEVT_XFERNOTREADY: c_uint = 0x03;
pub const DWC3_DEPEVT_RXTXFIFOEVT: c_uint = 0x04;
pub const DWC3_DEPEVT_STREAMEVT: c_uint = 0x06;
pub const DWC3_DEPEVT_EPCMDCMPLT: c_uint = 0x07;
//
// struct dwc3_event_depevt - Device Endpoint Events
// @one_bit: indicates this is an endpoint event (not used)
// @endpoint_number: number of the endpoint
// @endpoint_event: The event we have:
// 0x00	- Reserved
// 0x01	- XferComplete
// 0x02	- XferInProgress
// 0x03	- XferNotReady
// 0x04	- RxTxFifoEvt (IN->Underrun, OUT->Overrun)
// 0x05	- Reserved
// 0x06	- StreamEvt
// 0x07	- EPCmdCmplt
// @reserved11_10: Reserved, don't use.
// @status: Indicates the status of the event. Refer to databook for
// more information.
// @parameters: Parameters of the current event. Refer to databook for
// more information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_event_depevt {
    pub one_bit:1: u32,
    pub endpoint_number:5: u32,
    pub endpoint_event:4: u32,
    pub reserved11_10:2: u32,
    pub status:4: u32,
// Within XferNotReady

// Within XferComplete or XferInProgress

// Stream event only
pub const DEPEVT_STREAMEVT_FOUND: c_int = 1;
pub const DEPEVT_STREAMEVT_NOTFOUND: c_int = 2;
// Stream event parameter
pub const DEPEVT_STREAM_PRIME: c_uint = 0xfffe;
pub const DEPEVT_STREAM_NOSTREAM: c_uint = 0x0;
// Control-only Status
pub const DEPEVT_STATUS_CONTROL_DATA: c_int = 1;
pub const DEPEVT_STATUS_CONTROL_STATUS: c_int = 2;

// In response to Start Transfer
pub const DEPEVT_TRANSFER_NO_RESOURCE: c_int = 1;
pub const DEPEVT_TRANSFER_BUS_EXPIRY: c_int = 2;
    pub parameters:16: u32,
// For Command Complete Events

    pub __packed: },
//
// struct dwc3_event_devt - Device Events
// @one_bit: indicates this is a non-endpoint event (not used)
// @device_event: indicates it's a device event. Should read as 0x00
// @type: indicates the type of device event.
// 0	- DisconnEvt
// 1	- USBRst
// 2	- ConnectDone
// 3	- ULStChng
// 4	- WkUpEvt
// 5	- Reserved
// 6	- Suspend (EOPF on revisions 2.10a and prior)
// 7	- SOF
// 8	- Reserved
// 9	- ErrticErr
// 10	- CmdCmplt
// 11	- EvntOverflow
// 12	- VndrDevTstRcved
// @reserved15_12: Reserved, not used
// @event_info: Information about this event
// @reserved31_25: Reserved, not used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_event_devt {
    pub one_bit:1: u32,
    pub device_event:7: u32,
    pub type:4: u32,
    pub reserved15_12:4: u32,
    pub event_info:9: u32,
    pub reserved31_25:7: u32,
    pub __packed: },
//
// struct dwc3_event_gevt - Other Core Events
// @one_bit: indicates this is a non-endpoint event (not used)
// @device_event: indicates it's (0x03) Carkit or (0x04) I2C event.
// @phy_port_number: self-explanatory
// @reserved31_12: Reserved, not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_event_gevt {
    pub one_bit:1: u32,
    pub device_event:7: u32,
    pub phy_port_number:4: u32,
    pub reserved31_12:20: u32,
    pub __packed: },
//
// union dwc3_event - representation of Event Buffer contents
// @raw: raw 32-bit event
// @type: the type of the event
// @depevt: Device Endpoint Event
// @devt: Device Event
// @gevt: Global Event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dwc3_event {
    pub raw: u32,
    pub type: dwc3_event_type,
    pub depevt: dwc3_event_depevt,
    pub devt: dwc3_event_devt,
    pub gevt: dwc3_event_gevt,
}

//
// struct dwc3_gadget_ep_cmd_params - representation of endpoint command
// parameters
// @param2: third parameter
// @param1: second parameter
// @param0: first parameter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_gadget_ep_cmd_params {
    pub param2: u32,
    pub param1: u32,
    pub param0: u32,
}

//
// DWC3 Features to be used as Driver Data
//

// prototypes
extern "C" {
    pub fn dwc3_set_prtcap(dwc: *mut dwc3, mode: u32, ignore_susphy: bool);
}
extern "C" {
    pub fn dwc3_set_mode(dwc: *mut dwc3, mode: u32);
}
extern "C" {
    pub fn dwc3_core_fifo_space(dep: *mut dwc3_ep, type: u8) -> u32;
}

//
// dwc3_mdwidth - get MDWIDTH value in bits
// @dwc: pointer to our context structure
//
// Return MDWIDTH configuration value in bits.
//
extern "C" {
    pub fn dwc3_has_imod(dwc: *mut dwc3) -> bool;
}
extern "C" {
    pub fn dwc3_event_buffers_setup(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_event_buffers_cleanup(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_core_soft_reset(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_enable_susphy(dwc: *mut dwc3, enable: bool);
}

extern "C" {
    pub fn dwc3_host_init(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_host_exit(dwc: *mut dwc3);
}

extern "C" {
    pub fn dwc3_gadget_init(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_exit(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_gadget_set_test_mode(dwc: *mut dwc3, mode: c_int) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_get_link_state(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_set_link_state(dwc: *mut dwc3, state: dwc3_link_state) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_clear_tx_fifos(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_remove_requests(dwc: *mut dwc3, dep: *mut dwc3_ep, status: c_int);
}

extern "C" {
    pub fn dwc3_drd_init(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_drd_exit(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_otg_init(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_otg_exit(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_otg_update(dwc: *mut dwc3, ignore_idstatus: bool);
}
extern "C" {
    pub fn dwc3_otg_host_init(dwc: *mut dwc3);
}

// power management interface

extern "C" {
    pub fn dwc3_gadget_suspend(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_resume(dwc: *mut dwc3) -> c_int;
}

extern "C" {
    pub fn dwc3_ulpi_init(dwc: *mut dwc3) -> c_int;
}
extern "C" {
    pub fn dwc3_ulpi_exit(dwc: *mut dwc3);
}

