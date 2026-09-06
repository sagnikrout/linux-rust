//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/amd5536udc.h
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
// amd5536.h -- header for AMD 5536 UDC high/full speed USB device controller
//
// Copyright (C) 2007 AMD (https://www.amd.com)
// Author: Thomas Dahlmann
//
// debug control
// #define UDC_VERBOSE

// various constants
pub const UDC_RDE_TIMER_SECONDS: c_int = 1;
pub const UDC_RDE_TIMER_DIV: c_int = 10;
pub const UDC_POLLSTALL_TIMER_USECONDS: c_int = 500;
// Hs AMD5536 chip rev.
pub const UDC_HSA0_REV: c_int = 1;
pub const UDC_HSB1_REV: c_int = 2;
// Broadcom chip rev.
pub const UDC_BCM_REV: c_int = 10;
//
// SETUP usb commands
// needed, because some SETUP's are handled in hw, but must be passed to
// gadget driver above
// SET_CONFIG
//
pub const UDC_SETCONFIG_DWORD0: c_uint = 0x00000900;
pub const UDC_SETCONFIG_DWORD0_VALUE_MASK: c_uint = 0xffff0000;
pub const UDC_SETCONFIG_DWORD0_VALUE_OFS: c_int = 16;
pub const UDC_SETCONFIG_DWORD1: c_uint = 0x00000000;
// SET_INTERFACE
pub const UDC_SETINTF_DWORD0: c_uint = 0x00000b00;
pub const UDC_SETINTF_DWORD0_ALT_MASK: c_uint = 0xffff0000;
pub const UDC_SETINTF_DWORD0_ALT_OFS: c_int = 16;
pub const UDC_SETINTF_DWORD1: c_uint = 0x00000000;
pub const UDC_SETINTF_DWORD1_INTF_MASK: c_uint = 0x0000ffff;
pub const UDC_SETINTF_DWORD1_INTF_OFS: c_int = 0;
// Mass storage reset
pub const UDC_MSCRES_DWORD0: c_uint = 0x0000ff21;
pub const UDC_MSCRES_DWORD1: c_uint = 0x00000000;
// Global CSR's -------------------------------------------------------------
pub const UDC_CSR_ADDR: c_uint = 0x500;
// EP NE bits
// EP number
pub const UDC_CSR_NE_NUM_MASK: c_uint = 0x0000000f;
pub const UDC_CSR_NE_NUM_OFS: c_int = 0;
// EP direction
pub const UDC_CSR_NE_DIR_MASK: c_uint = 0x00000010;
pub const UDC_CSR_NE_DIR_OFS: c_int = 4;
// EP type
pub const UDC_CSR_NE_TYPE_MASK: c_uint = 0x00000060;
pub const UDC_CSR_NE_TYPE_OFS: c_int = 5;
// EP config number
pub const UDC_CSR_NE_CFG_MASK: c_uint = 0x00000780;
pub const UDC_CSR_NE_CFG_OFS: c_int = 7;
// EP interface number
pub const UDC_CSR_NE_INTF_MASK: c_uint = 0x00007800;
pub const UDC_CSR_NE_INTF_OFS: c_int = 11;
// EP alt setting
pub const UDC_CSR_NE_ALT_MASK: c_uint = 0x00078000;
pub const UDC_CSR_NE_ALT_OFS: c_int = 15;
// max pkt
pub const UDC_CSR_NE_MAX_PKT_MASK: c_uint = 0x3ff80000;
pub const UDC_CSR_NE_MAX_PKT_OFS: c_int = 19;
// Device Config Register ---------------------------------------------------
pub const UDC_DEVCFG_ADDR: c_uint = 0x400;
pub const UDC_DEVCFG_SOFTRESET: c_int = 31;
pub const UDC_DEVCFG_HNPSFEN: c_int = 30;
pub const UDC_DEVCFG_DMARST: c_int = 29;
pub const UDC_DEVCFG_SET_DESC: c_int = 18;
pub const UDC_DEVCFG_CSR_PRG: c_int = 17;
pub const UDC_DEVCFG_STATUS: c_int = 7;
pub const UDC_DEVCFG_DIR: c_int = 6;
pub const UDC_DEVCFG_PI: c_int = 5;
pub const UDC_DEVCFG_SS: c_int = 4;
pub const UDC_DEVCFG_SP: c_int = 3;
pub const UDC_DEVCFG_RWKP: c_int = 2;
pub const UDC_DEVCFG_SPD_MASK: c_uint = 0x3;
pub const UDC_DEVCFG_SPD_OFS: c_int = 0;
pub const UDC_DEVCFG_SPD_HS: c_uint = 0x0;
pub const UDC_DEVCFG_SPD_FS: c_uint = 0x1;
pub const UDC_DEVCFG_SPD_LS: c_uint = 0x2;
// #define UDC_DEVCFG_SPD_FS			0x3
// Device Control Register --------------------------------------------------
pub const UDC_DEVCTL_ADDR: c_uint = 0x404;
pub const UDC_DEVCTL_THLEN_MASK: c_uint = 0xff000000;
pub const UDC_DEVCTL_THLEN_OFS: c_int = 24;
pub const UDC_DEVCTL_BRLEN_MASK: c_uint = 0x00ff0000;
pub const UDC_DEVCTL_BRLEN_OFS: c_int = 16;
pub const UDC_DEVCTL_SRX_FLUSH: c_int = 14;
pub const UDC_DEVCTL_CSR_DONE: c_int = 13;
pub const UDC_DEVCTL_DEVNAK: c_int = 12;
pub const UDC_DEVCTL_SD: c_int = 10;
pub const UDC_DEVCTL_MODE: c_int = 9;
pub const UDC_DEVCTL_BREN: c_int = 8;
pub const UDC_DEVCTL_THE: c_int = 7;
pub const UDC_DEVCTL_BF: c_int = 6;
pub const UDC_DEVCTL_BE: c_int = 5;
pub const UDC_DEVCTL_DU: c_int = 4;
pub const UDC_DEVCTL_TDE: c_int = 3;
pub const UDC_DEVCTL_RDE: c_int = 2;
pub const UDC_DEVCTL_RES: c_int = 0;
// Device Status Register ---------------------------------------------------
pub const UDC_DEVSTS_ADDR: c_uint = 0x408;
pub const UDC_DEVSTS_TS_MASK: c_uint = 0xfffc0000;
pub const UDC_DEVSTS_TS_OFS: c_int = 18;
pub const UDC_DEVSTS_SESSVLD: c_int = 17;
pub const UDC_DEVSTS_PHY_ERROR: c_int = 16;
pub const UDC_DEVSTS_RXFIFO_EMPTY: c_int = 15;
pub const UDC_DEVSTS_ENUM_SPEED_MASK: c_uint = 0x00006000;
pub const UDC_DEVSTS_ENUM_SPEED_OFS: c_int = 13;
pub const UDC_DEVSTS_ENUM_SPEED_FULL: c_int = 1;
pub const UDC_DEVSTS_ENUM_SPEED_HIGH: c_int = 0;
pub const UDC_DEVSTS_SUSP: c_int = 12;
pub const UDC_DEVSTS_ALT_MASK: c_uint = 0x00000f00;
pub const UDC_DEVSTS_ALT_OFS: c_int = 8;
pub const UDC_DEVSTS_INTF_MASK: c_uint = 0x000000f0;
pub const UDC_DEVSTS_INTF_OFS: c_int = 4;
pub const UDC_DEVSTS_CFG_MASK: c_uint = 0x0000000f;
pub const UDC_DEVSTS_CFG_OFS: c_int = 0;
// Device Interrupt Register ------------------------------------------------
pub const UDC_DEVINT_ADDR: c_uint = 0x40c;
pub const UDC_DEVINT_SVC: c_int = 7;
pub const UDC_DEVINT_ENUM: c_int = 6;
pub const UDC_DEVINT_SOF: c_int = 5;
pub const UDC_DEVINT_US: c_int = 4;
pub const UDC_DEVINT_UR: c_int = 3;
pub const UDC_DEVINT_ES: c_int = 2;
pub const UDC_DEVINT_SI: c_int = 1;
pub const UDC_DEVINT_SC: c_int = 0;
// Device Interrupt Mask Register -------------------------------------------
pub const UDC_DEVINT_MSK_ADDR: c_uint = 0x410;
pub const UDC_DEVINT_MSK: c_uint = 0x7f;
// Endpoint Interrupt Register ----------------------------------------------
pub const UDC_EPINT_ADDR: c_uint = 0x414;
pub const UDC_EPINT_OUT_MASK: c_uint = 0xffff0000;
pub const UDC_EPINT_OUT_OFS: c_int = 16;
pub const UDC_EPINT_IN_MASK: c_uint = 0x0000ffff;
pub const UDC_EPINT_IN_OFS: c_int = 0;
pub const UDC_EPINT_IN_EP0: c_int = 0;
pub const UDC_EPINT_IN_EP1: c_int = 1;
pub const UDC_EPINT_IN_EP2: c_int = 2;
pub const UDC_EPINT_IN_EP3: c_int = 3;
pub const UDC_EPINT_OUT_EP0: c_int = 16;
pub const UDC_EPINT_OUT_EP1: c_int = 17;
pub const UDC_EPINT_OUT_EP2: c_int = 18;
pub const UDC_EPINT_OUT_EP3: c_int = 19;
pub const UDC_EPINT_EP0_ENABLE_MSK: c_uint = 0x001e001e;
// Endpoint Interrupt Mask Register -----------------------------------------
pub const UDC_EPINT_MSK_ADDR: c_uint = 0x418;
pub const UDC_EPINT_OUT_MSK_MASK: c_uint = 0xffff0000;
pub const UDC_EPINT_OUT_MSK_OFS: c_int = 16;
pub const UDC_EPINT_IN_MSK_MASK: c_uint = 0x0000ffff;
pub const UDC_EPINT_IN_MSK_OFS: c_int = 0;
pub const UDC_EPINT_MSK_DISABLE_ALL: c_uint = 0xffffffff;
// mask non-EP0 endpoints
pub const UDC_EPDATAINT_MSK_DISABLE: c_uint = 0xfffefffe;
// mask all dev interrupts
pub const UDC_DEV_MSK_DISABLE: c_uint = 0x7f;
// Endpoint-specific CSR's --------------------------------------------------
pub const UDC_EPREGS_ADDR: c_uint = 0x0;
pub const UDC_EPIN_REGS_ADDR: c_uint = 0x0;
pub const UDC_EPOUT_REGS_ADDR: c_uint = 0x200;
pub const UDC_EPCTL_ADDR: c_uint = 0x0;
pub const UDC_EPCTL_RRDY: c_int = 9;
pub const UDC_EPCTL_CNAK: c_int = 8;
pub const UDC_EPCTL_SNAK: c_int = 7;
pub const UDC_EPCTL_NAK: c_int = 6;
pub const UDC_EPCTL_ET_MASK: c_uint = 0x00000030;
pub const UDC_EPCTL_ET_OFS: c_int = 4;
pub const UDC_EPCTL_ET_CONTROL: c_int = 0;
pub const UDC_EPCTL_ET_ISO: c_int = 1;
pub const UDC_EPCTL_ET_BULK: c_int = 2;
pub const UDC_EPCTL_ET_INTERRUPT: c_int = 3;
pub const UDC_EPCTL_P: c_int = 3;
pub const UDC_EPCTL_SN: c_int = 2;
pub const UDC_EPCTL_F: c_int = 1;
pub const UDC_EPCTL_S: c_int = 0;
// Endpoint Status Registers ------------------------------------------------
pub const UDC_EPSTS_ADDR: c_uint = 0x4;
pub const UDC_EPSTS_RX_PKT_SIZE_MASK: c_uint = 0x007ff800;
pub const UDC_EPSTS_RX_PKT_SIZE_OFS: c_int = 11;
pub const UDC_EPSTS_TDC: c_int = 10;
pub const UDC_EPSTS_HE: c_int = 9;
pub const UDC_EPSTS_BNA: c_int = 7;
pub const UDC_EPSTS_IN: c_int = 6;
pub const UDC_EPSTS_OUT_MASK: c_uint = 0x00000030;
pub const UDC_EPSTS_OUT_OFS: c_int = 4;
pub const UDC_EPSTS_OUT_DATA: c_int = 1;
pub const UDC_EPSTS_OUT_DATA_CLEAR: c_uint = 0x10;
pub const UDC_EPSTS_OUT_SETUP: c_int = 2;
pub const UDC_EPSTS_OUT_SETUP_CLEAR: c_uint = 0x20;
pub const UDC_EPSTS_OUT_CLEAR: c_uint = 0x30;
// Endpoint Buffer Size IN/ Receive Packet Frame Number OUT Registers ------
pub const UDC_EPIN_BUFF_SIZE_ADDR: c_uint = 0x8;
pub const UDC_EPOUT_FRAME_NUMBER_ADDR: c_uint = 0x8;
pub const UDC_EPIN_BUFF_SIZE_MASK: c_uint = 0x0000ffff;
pub const UDC_EPIN_BUFF_SIZE_OFS: c_int = 0;
// EP0in txfifo = 128 bytes
pub const UDC_EPIN0_BUFF_SIZE: c_int = 32;
// EP0in fullspeed txfifo = 128 bytes
pub const UDC_FS_EPIN0_BUFF_SIZE: c_int = 32;
// fifo size mult = fifo size / max packet
pub const UDC_EPIN_BUFF_SIZE_MULT: c_int = 2;
// EPin data fifo size = 1024 bytes DOUBLE BUFFERING
pub const UDC_EPIN_BUFF_SIZE: c_int = 256;
// EPin small INT data fifo size = 128 bytes
pub const UDC_EPIN_SMALLINT_BUFF_SIZE: c_int = 32;
// EPin fullspeed data fifo size = 128 bytes DOUBLE BUFFERING
pub const UDC_FS_EPIN_BUFF_SIZE: c_int = 32;
pub const UDC_EPOUT_FRAME_NUMBER_MASK: c_uint = 0x0000ffff;
pub const UDC_EPOUT_FRAME_NUMBER_OFS: c_int = 0;
// Endpoint Buffer Size OUT/Max Packet Size Registers -----------------------
pub const UDC_EPOUT_BUFF_SIZE_ADDR: c_uint = 0x0c;
pub const UDC_EP_MAX_PKT_SIZE_ADDR: c_uint = 0x0c;
pub const UDC_EPOUT_BUFF_SIZE_MASK: c_uint = 0xffff0000;
pub const UDC_EPOUT_BUFF_SIZE_OFS: c_int = 16;
pub const UDC_EP_MAX_PKT_SIZE_MASK: c_uint = 0x0000ffff;
pub const UDC_EP_MAX_PKT_SIZE_OFS: c_int = 0;
// EP0in max packet size = 64 bytes
pub const UDC_EP0IN_MAX_PKT_SIZE: c_int = 64;
// EP0out max packet size = 64 bytes
pub const UDC_EP0OUT_MAX_PKT_SIZE: c_int = 64;
// EP0in fullspeed max packet size = 64 bytes
pub const UDC_FS_EP0IN_MAX_PKT_SIZE: c_int = 64;
// EP0out fullspeed max packet size = 64 bytes
pub const UDC_FS_EP0OUT_MAX_PKT_SIZE: c_int = 64;
//
// Endpoint dma descriptors ------------------------------------------------
//
// Setup data, Status dword
//
pub const UDC_DMA_STP_STS_CFG_MASK: c_uint = 0x0fff0000;
pub const UDC_DMA_STP_STS_CFG_OFS: c_int = 16;
pub const UDC_DMA_STP_STS_CFG_ALT_MASK: c_uint = 0x000f0000;
pub const UDC_DMA_STP_STS_CFG_ALT_OFS: c_int = 16;
pub const UDC_DMA_STP_STS_CFG_INTF_MASK: c_uint = 0x00f00000;
pub const UDC_DMA_STP_STS_CFG_INTF_OFS: c_int = 20;
pub const UDC_DMA_STP_STS_CFG_NUM_MASK: c_uint = 0x0f000000;
pub const UDC_DMA_STP_STS_CFG_NUM_OFS: c_int = 24;
pub const UDC_DMA_STP_STS_RX_MASK: c_uint = 0x30000000;
pub const UDC_DMA_STP_STS_RX_OFS: c_int = 28;
pub const UDC_DMA_STP_STS_BS_MASK: c_uint = 0xc0000000;
pub const UDC_DMA_STP_STS_BS_OFS: c_int = 30;
pub const UDC_DMA_STP_STS_BS_HOST_READY: c_int = 0;
pub const UDC_DMA_STP_STS_BS_DMA_BUSY: c_int = 1;
pub const UDC_DMA_STP_STS_BS_DMA_DONE: c_int = 2;
pub const UDC_DMA_STP_STS_BS_HOST_BUSY: c_int = 3;
// IN data, Status dword
pub const UDC_DMA_IN_STS_TXBYTES_MASK: c_uint = 0x0000ffff;
pub const UDC_DMA_IN_STS_TXBYTES_OFS: c_int = 0;
pub const UDC_DMA_IN_STS_FRAMENUM_MASK: c_uint = 0x07ff0000;
pub const UDC_DMA_IN_STS_FRAMENUM_OFS: c_int = 0;
pub const UDC_DMA_IN_STS_L: c_int = 27;
pub const UDC_DMA_IN_STS_TX_MASK: c_uint = 0x30000000;
pub const UDC_DMA_IN_STS_TX_OFS: c_int = 28;
pub const UDC_DMA_IN_STS_BS_MASK: c_uint = 0xc0000000;
pub const UDC_DMA_IN_STS_BS_OFS: c_int = 30;
pub const UDC_DMA_IN_STS_BS_HOST_READY: c_int = 0;
pub const UDC_DMA_IN_STS_BS_DMA_BUSY: c_int = 1;
pub const UDC_DMA_IN_STS_BS_DMA_DONE: c_int = 2;
pub const UDC_DMA_IN_STS_BS_HOST_BUSY: c_int = 3;
// OUT data, Status dword
pub const UDC_DMA_OUT_STS_RXBYTES_MASK: c_uint = 0x0000ffff;
pub const UDC_DMA_OUT_STS_RXBYTES_OFS: c_int = 0;
pub const UDC_DMA_OUT_STS_FRAMENUM_MASK: c_uint = 0x07ff0000;
pub const UDC_DMA_OUT_STS_FRAMENUM_OFS: c_int = 0;
pub const UDC_DMA_OUT_STS_L: c_int = 27;
pub const UDC_DMA_OUT_STS_RX_MASK: c_uint = 0x30000000;
pub const UDC_DMA_OUT_STS_RX_OFS: c_int = 28;
pub const UDC_DMA_OUT_STS_BS_MASK: c_uint = 0xc0000000;
pub const UDC_DMA_OUT_STS_BS_OFS: c_int = 30;
pub const UDC_DMA_OUT_STS_BS_HOST_READY: c_int = 0;
pub const UDC_DMA_OUT_STS_BS_DMA_BUSY: c_int = 1;
pub const UDC_DMA_OUT_STS_BS_DMA_DONE: c_int = 2;
pub const UDC_DMA_OUT_STS_BS_HOST_BUSY: c_int = 3;
// max ep0in packet
pub const UDC_EP0IN_MAXPACKET: c_int = 1000;
// max dma packet
pub const UDC_DMA_MAXPACKET: c_int = 65536;
// un-usable DMA address

// other Endpoint register addresses and values-----------------------------
pub const UDC_EP_SUBPTR_ADDR: c_uint = 0x10;
pub const UDC_EP_DESPTR_ADDR: c_uint = 0x14;
pub const UDC_EP_WRITE_CONFIRM_ADDR: c_uint = 0x1c;
// EP number as layouted in AHB space
pub const UDC_EP_NUM: c_int = 32;
pub const UDC_EPIN_NUM: c_int = 16;
pub const UDC_EPIN_NUM_USED: c_int = 5;
pub const UDC_EPOUT_NUM: c_int = 16;
// EP number of EP's really used = EP0 + 8 data EP's
pub const UDC_USED_EP_NUM: c_int = 9;
// UDC CSR regs are aligned but AHB regs not - offset for OUT EP's
pub const UDC_CSR_EP_OUT_IX_OFS: c_int = 12;
pub const UDC_EP0OUT_IX: c_int = 16;
pub const UDC_EP0IN_IX: c_int = 0;
// Rx fifo address and size = 1k -------------------------------------------
pub const UDC_RXFIFO_ADDR: c_uint = 0x800;
pub const UDC_RXFIFO_SIZE: c_uint = 0x400;
// Tx fifo address and size = 1.5k -----------------------------------------
pub const UDC_TXFIFO_ADDR: c_uint = 0xc00;
pub const UDC_TXFIFO_SIZE: c_uint = 0x600;
// default data endpoints --------------------------------------------------
pub const UDC_EPIN_STATUS_IX: c_int = 1;
pub const UDC_EPIN_IX: c_int = 2;
pub const UDC_EPOUT_IX: c_int = 18;
// general constants -------------------------------------------------------
pub const UDC_DWORD_BYTES: c_int = 4;
pub const UDC_BITS_PER_BYTE_SHIFT: c_int = 3;
pub const UDC_BYTE_MASK: c_uint = 0xff;
pub const UDC_BITS_PER_BYTE: c_int = 8;
// ---------------------------------------------------------------------------
// UDC CSR's
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_csrs {
// sca - setup command address
    pub sca: u32,
// ep ne's
    pub ne: [u32; UDC_USED_EP_NUM],
// C attribute field omitted
// AHB subsystem CSR registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_regs {
// device configuration
    pub cfg: u32,
// device control
    pub ctl: u32,
// device status
    pub sts: u32,
// device interrupt
    pub irqsts: u32,
// device interrupt mask
    pub irqmsk: u32,
// endpoint interrupt
    pub ep_irqsts: u32,
// endpoint interrupt mask
    pub ep_irqmsk: u32,
// C attribute field omitted
// endpoint specific registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_ep_regs {
// endpoint control
    pub ctl: u32,
// endpoint status
    pub sts: u32,
// endpoint buffer size in/ receive packet frame number out
    pub bufin_framenum: u32,
// endpoint buffer size out/max packet size
    pub bufout_maxpkt: u32,
// endpoint setup buffer pointer
    pub subptr: u32,
// endpoint data descriptor pointer
    pub desptr: u32,
// reserved
    pub reserved: u32,
// write/read confirmation
    pub confirm: u32,
// C attribute field omitted
// control data DMA desc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_stp_dma {
// status quadlet
    pub status: u32,
// reserved
    pub _reserved: u32,
// first setup word
    pub data12: u32,
// second setup word
    pub data34: u32,
// C attribute field omitted
// normal data DMA desc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_data_dma {
// status quadlet
    pub status: u32,
// reserved
    pub _reserved: u32,
// buffer pointer
    pub bufptr: u32,
// next descriptor pointer
    pub next: u32,
// C attribute field omitted
// request packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_request {
// embedded gadget ep
    pub req: usb_request,
// flags
    pub 1: dma_done :,
// phys. address
    pub td_phys: dma_addr_t,
// first dma desc. of chain
    pub td_data: *mut udc_data_dma,
// last dma desc. of chain
    pub td_data_last: *mut udc_data_dma,
    pub queue: list_head,
// chain length
    pub chain_len: unsigned,
}

// UDC specific endpoint parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_ep {
    pub ep: usb_ep,
    pub regs: *mut udc_ep_regs __iomem,
    pub txfifo: *mut u32 __iomem,
    pub dma: *mut u32 __iomem,
    pub td_phys: dma_addr_t,
    pub td_stp_dma: dma_addr_t,
    pub td_stp: *mut udc_stp_dma,
    pub td: *mut udc_data_dma,
// temp request
    pub req: *mut udc_request,
    pub req_used: unsigned,
    pub req_completed: unsigned,
// dummy DMA desc for BNA dummy
    pub bna_dummy_req: *mut udc_request,
    pub bna_occurred: unsigned,
// NAK state
    pub naking: unsigned,
    pub dev: *mut udc,
// queue for requests
    pub queue: list_head,
    pub halted: unsigned,
    pub cancel_transfer: unsigned,
    pub 1: in :,
}

// device struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc {
    pub gadget: usb_gadget,
    pub /: *mut *mut spinlock_t lock; / protects all state,
// all endpoints
    pub ep: [udc_ep; UDC_EP_NUM],
    pub driver: *mut usb_gadget_driver,
// operational flags
    pub chiprev: u16,
// registers
    pub pdev: *mut pci_dev,
    pub csr: *mut udc_csrs __iomem,
    pub regs: *mut udc_regs __iomem,
    pub ep_regs: *mut udc_ep_regs __iomem,
    pub rxfifo: *mut u32 __iomem,
    pub txfifo: *mut u32 __iomem,
// DMA desc pools
    pub data_requests: *mut dma_pool,
    pub stp_requests: *mut dma_pool,
// device data
    pub phys_addr: c_ulong,
    pub virt_addr: *mut void __iomem,
    pub irq: unsigned,
// states
    pub cur_config: u16,
    pub cur_intf: u16,
    pub cur_alt: u16,
// for platform device and extcon support
    pub dev: *mut device,
    pub udc_phy: *mut phy,
    pub edev: *mut extcon_dev,
    pub extcon_nb: extcon_specific_cable_nb,
    pub nb: notifier_block,
    pub drd_work: delayed_work,
    pub conn_type: u32,
}

// setup request data
#[repr(C)]
#[derive(Copy, Clone)]
pub union udc_setup_data {
    pub data: [u32; 2],
    pub request: usb_ctrlrequest,
}

// Function declarations
extern "C" {
    pub fn udc_enable_dev_setup_interrupts(dev: *mut udc) -> c_int;
}
extern "C" {
    pub fn udc_mask_unused_interrupts(dev: *mut udc) -> c_int;
}
extern "C" {
    pub fn udc_irq(irq: c_int, pdev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn gadget_release(pdev: *mut device);
}
extern "C" {
    pub fn empty_req_queue(ep: *mut udc_ep);
}
extern "C" {
    pub fn udc_basic_init(dev: *mut udc);
}
extern "C" {
    pub fn free_dma_pools(dev: *mut udc);
}
extern "C" {
    pub fn init_dma_pools(dev: *mut udc) -> c_int;
}
extern "C" {
    pub fn udc_remove(dev: *mut udc);
}
extern "C" {
    pub fn udc_probe(dev: *mut udc) -> c_int;
}
// DMA usage flag
// packet per buffer dma
// with per descr. update
// full speed only mode
// module parameters
//
// ---------------------------------------------------------------------------
// SET and GET bitfields in u32 values
// via constants for mask/offset:
// <bit_field_stub_name> is the text between
// UDC_ and _MASK|_OFS of appropriate
// constant
//
// set bitfield value in u32 u32Val
//

//
// set bitfield value in zero-initialized u32 u32Val
// => bitfield bits in u32Val are all zero
//

// get bitfield value from u32 u32Val

// SET and GET bits in u32 values ------------------------------------------

// debug macros ------------------------------------------------------------

