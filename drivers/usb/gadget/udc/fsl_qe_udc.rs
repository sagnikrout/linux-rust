//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/fsl_qe_udc.h
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
// drivers/usb/gadget/qe_udc.h
//
// Copyright (C) 2006-2008 Freescale Semiconductor, Inc. All rights reserved.
//
// Xiaobo Xie <X.Xie@freescale.com>
// Li Yang <leoli@freescale.com>
//
// Description:
// Freescale USB device/endpoint management registers
//
// SoC type
pub const PORT_CPM: c_int = 0;
pub const PORT_QE: c_int = 1;
pub const USB_MAX_ENDPOINTS: c_int = 4;

pub const USB_EP0_MAX_SIZE: c_int = 64;
pub const USB_MAX_CTRL_PAYLOAD: c_uint = 0x4000;
pub const USB_BDRING_LEN: c_int = 16;
pub const USB_BDRING_LEN_RX: c_int = 256;
pub const USB_BDRING_LEN_TX: c_int = 16;
pub const MIN_EMPTY_BDS: c_int = 128;
pub const MAX_DATA_BDS: c_int = 8;
pub const USB_CRC_SIZE: c_int = 2;
pub const USB_DIR_BOTH: c_uint = 0x88;
pub const R_BUF_MAXSIZE: c_uint = 0x800;
pub const USB_EP_PARA_ALIGNMENT: c_int = 32;
// USB Mode Register bit define
pub const USB_MODE_EN: c_uint = 0x01;
pub const USB_MODE_HOST: c_uint = 0x02;
pub const USB_MODE_TEST: c_uint = 0x04;
pub const USB_MODE_SFTE: c_uint = 0x08;
pub const USB_MODE_RESUME: c_uint = 0x40;
pub const USB_MODE_LSS: c_uint = 0x80;
// USB Slave Address Register Mask
pub const USB_SLVADDR_MASK: c_uint = 0x7F;
// USB Endpoint register define
pub const USB_EPNUM_MASK: c_uint = 0xF000;
pub const USB_EPNUM_SHIFT: c_int = 12;
pub const USB_TRANS_MODE_SHIFT: c_int = 8;
pub const USB_TRANS_CTR: c_uint = 0x0000;
pub const USB_TRANS_INT: c_uint = 0x0100;
pub const USB_TRANS_BULK: c_uint = 0x0200;
pub const USB_TRANS_ISO: c_uint = 0x0300;
pub const USB_EP_MF: c_uint = 0x0020;
pub const USB_EP_RTE: c_uint = 0x0010;
pub const USB_THS_SHIFT: c_int = 2;
pub const USB_THS_MASK: c_uint = 0x000c;
pub const USB_THS_NORMAL: c_uint = 0x0;
pub const USB_THS_IGNORE_IN: c_uint = 0x0004;
pub const USB_THS_NACK: c_uint = 0x0008;
pub const USB_THS_STALL: c_uint = 0x000c;
pub const USB_RHS_SHIFT: c_int = 0;
pub const USB_RHS_MASK: c_uint = 0x0003;
pub const USB_RHS_NORMAL: c_uint = 0x0;
pub const USB_RHS_IGNORE_OUT: c_uint = 0x0001;
pub const USB_RHS_NACK: c_uint = 0x0002;
pub const USB_RHS_STALL: c_uint = 0x0003;
pub const USB_RTHS_MASK: c_uint = 0x000f;
// USB Command Register define
pub const USB_CMD_STR_FIFO: c_uint = 0x80;
pub const USB_CMD_FLUSH_FIFO: c_uint = 0x40;
pub const USB_CMD_ISFT: c_uint = 0x20;
pub const USB_CMD_DSFT: c_uint = 0x10;
pub const USB_CMD_EP_MASK: c_uint = 0x03;
// USB Event and Mask Register define
pub const USB_E_MSF_MASK: c_uint = 0x0800;
pub const USB_E_SFT_MASK: c_uint = 0x0400;
pub const USB_E_RESET_MASK: c_uint = 0x0200;
pub const USB_E_IDLE_MASK: c_uint = 0x0100;
pub const USB_E_TXE4_MASK: c_uint = 0x0080;
pub const USB_E_TXE3_MASK: c_uint = 0x0040;
pub const USB_E_TXE2_MASK: c_uint = 0x0020;
pub const USB_E_TXE1_MASK: c_uint = 0x0010;
pub const USB_E_SOF_MASK: c_uint = 0x0008;
pub const USB_E_BSY_MASK: c_uint = 0x0004;
pub const USB_E_TXB_MASK: c_uint = 0x0002;
pub const USB_E_RXB_MASK: c_uint = 0x0001;
pub const USBER_ALL_CLEAR: c_uint = 0x0fff;

// USB Status Register define
pub const USB_IDLE_STATUS_MASK: c_uint = 0x01;
// USB Start of Frame Timer
pub const USB_USSFT_MASK: c_uint = 0x3FFF;
// USB Frame Number Register
pub const USB_USFRN_MASK: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_device_para {
    pub epptr: [u16; 4],
    pub rstate: u32,
    pub rptr: u32,
    pub frame_n: u16,
    pub rbcnt: u16,
    pub rtemp: u32,
    pub rxusb_data: u32,
    pub rxuptr: u16,
    pub reso: [u8; 2],
    pub softbl: u32,
    pub sofucrctemp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ep_para {
    pub rbase: u16,
    pub tbase: u16,
    pub rbmr: u8,
    pub tbmr: u8,
    pub mrblr: u16,
    pub rbptr: u16,
    pub tbptr: u16,
    pub tstate: u32,
    pub tptr: u32,
    pub tcrc: u16,
    pub tbcnt: u16,
    pub ttemp: u32,
    pub txusbu_ptr: u16,
    pub reserve: [u8; 2],
}

pub const USB_BUSMODE_GBL: c_uint = 0x20;
pub const USB_BUSMODE_BO_MASK: c_uint = 0x18;
pub const USB_BUSMODE_BO_SHIFT: c_uint = 0x3;
pub const USB_BUSMODE_BE: c_uint = 0x2;
pub const USB_BUSMODE_CETM: c_uint = 0x04;
pub const USB_BUSMODE_DTB: c_uint = 0x02;
// Endpoint basic handle

// ep0 transfer state
pub const WAIT_FOR_SETUP: c_int = 0;
pub const DATA_STATE_XMIT: c_int = 1;
pub const DATA_STATE_NEED_ZLP: c_int = 2;
pub const WAIT_FOR_OUT_STATUS: c_int = 3;
pub const DATA_STATE_RECV: c_int = 4;
// ep tramsfer mode
pub const USBP_TM_CTL: c_int = 0;
pub const USBP_TM_ISO: c_int = 1;
pub const USBP_TM_BULK: c_int = 2;
pub const USBP_TM_INT: c_int = 3;
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_frame {
    pub data: *mut u8,
    pub len: u32,
    pub status: u32,
    pub info: u32,
    pub privdata: *mut c_void,
    pub node: list_head,
}

// Frame structure, info field.
pub const PID_DATA0: c_uint = 0x80000000 /* Data toggle zero */;
pub const PID_DATA1: c_uint = 0x40000000 /* Data toggle one  */;
pub const PID_SETUP: c_uint = 0x20000000 /* setup bit */;
pub const SETUP_STATUS: c_uint = 0x10000000 /* setup status bit */;
pub const SETADDR_STATUS: c_uint = 0x08000000 /* setupup address status bit */;
pub const NO_REQ: c_uint = 0x04000000 /* Frame without request */;
pub const HOST_DATA: c_uint = 0x02000000 /* Host data frame */;
pub const FIRST_PACKET_IN_FRAME: c_uint = 0x01000000 /* first packet in the frame */;
pub const TOKEN_FRAME: c_uint = 0x00800000 /* Host token frame */;
pub const ZLP: c_uint = 0x00400000 /* Zero length packet */;
pub const IN_TOKEN_FRAME: c_uint = 0x00200000 /* In token package */;
pub const OUT_TOKEN_FRAME: c_uint = 0x00100000 /* Out token package */;
pub const SETUP_TOKEN_FRAME: c_uint = 0x00080000 /* Setup token package */;
pub const STALL_FRAME: c_uint = 0x00040000 /* Stall handshake */;
pub const NACK_FRAME: c_uint = 0x00020000 /* Nack handshake */;
pub const NO_PID: c_uint = 0x00010000 /* No send PID */;
pub const NO_CRC: c_uint = 0x00008000 /* No send CRC */;
pub const HOST_COMMAND: c_uint = 0x00004000 /* Host command frame   */;
// Frame status field
// Receive side
pub const FRAME_OK: c_uint = 0x00000000 /* Frame transmitted or received OK */;
pub const FRAME_ERROR: c_uint = 0x80000000 /* Error occurred on frame */;
pub const START_FRAME_LOST: c_uint = 0x40000000 /* START_FRAME_LOST */;
pub const END_FRAME_LOST: c_uint = 0x20000000 /* END_FRAME_LOST */;
pub const RX_ER_NONOCT: c_uint = 0x10000000 /* Rx Non Octet Aligned Packet */;
pub const RX_ER_BITSTUFF: c_uint = 0x08000000 /* Frame Aborted --Received packet;
pub const RX_ER_CRC: c_uint = 0x04000000 /* Received packet with CRC error */;
pub const RX_ER_OVERUN: c_uint = 0x02000000 /* Over-run occurred on reception */;
pub const RX_ER_PID: c_uint = 0x01000000 /* Wrong PID received */;
// Tranmit side
pub const TX_ER_NAK: c_uint = 0x00800000 /* Received NAK handshake */;
pub const TX_ER_STALL: c_uint = 0x00400000 /* Received STALL handshake */;
pub const TX_ER_TIMEOUT: c_uint = 0x00200000 /* Transmit time out */;
pub const TX_ER_UNDERUN: c_uint = 0x00100000 /* Transmit underrun */;
pub const FRAME_INPROGRESS: c_uint = 0x00080000 /* Frame is being transmitted */;
pub const ER_DATA_UNDERUN: c_uint = 0x00040000 /* Frame is shorter then expected */;
pub const ER_DATA_OVERUN: c_uint = 0x00020000 /* Frame is longer then expected */;
// QE USB frame operation functions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_req {
    pub req: usb_request,
    pub queue: list_head,
// ep_queue() func will add
    pub ep: *mut qe_ep,
    pub mapped:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub udc: *mut qe_udc,
    pub gadget: *mut usb_gadget,
    pub state: u8,
    pub rxbase: *mut qe_bd __iomem,
    pub n_rxbd: *mut qe_bd __iomem,
    pub e_rxbd: *mut qe_bd __iomem,
    pub txbase: *mut qe_bd __iomem,
    pub n_txbd: *mut qe_bd __iomem,
    pub c_txbd: *mut qe_bd __iomem,
    pub rxframe: *mut qe_frame,
    pub rxbuffer: *mut u8,
    pub rxbuf_d: dma_addr_t,
    pub rxbufmap: u8,
    pub localnack: c_uchar,
    pub has_data: c_int,
    pub txframe: *mut qe_frame,
    pub tx_req: *mut qe_req,
    pub /: *mut *mut int sent; /data already sent,
    pub time*/: *mut *mut int last; /data sent in the last,
    pub dir: u8,
    pub epnum: u8,
    pub /: *mut *mut u8 tm; / transfer mode,
    pub data01: u8,
    pub init: u8,
    pub already_seen: u8,
    pub enable_tasklet: u8,
    pub setup_stage: u8,
    pub /: *mut *mut u32 last_io; / timestamp,
    pub name: [c_char; 14],
    pub double_buf:1: unsigned,
    pub stopped:1: unsigned,
    pub fnf:1: unsigned,
    pub has_dma:1: unsigned,
    pub ackwait: u8,
    pub dma_channel: u8,
    pub dma_counter: u16,
    pub lch: c_int,
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_udc {
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub dev: *mut device,
    pub eps: [qe_ep; USB_MAX_ENDPOINTS],
    pub local_setup_buff: usb_ctrlrequest,
    pub /: *mut *mut spinlock_t lock; / lock for set/config qe_udc,
    pub /: *mut *mut unsigned long soc_type; / QE or CPM soc,
    pub /: *mut *mut *mut qe_req status_req; / ep0 status request,
// USB and EP Parameter Block pointer
    pub usb_param: *mut usb_device_para __iomem,
    pub ep_param: [*mut usb_ep_para __iomem; 4],
    pub /: *mut *mut u32 max_pipes; / Device max pipes,
    pub /: *mut *mut u32 max_use_endpts; / Max endpointes to be used,
    pub /: *mut *mut u32 bus_reset; / Device is bus reseting,
    pub resume*/: *mut *mut u32 resume_state; / USB state to,
    pub /: *mut *mut u32 usb_state; / USB current state,
    pub /: *mut *mut u32 usb_next_state; / USB next state,
    pub /: *mut *mut u32 ep0_state; / Endpoint zero state,
    pub be: *mut *mut u32 ep0_dir; / Endpoint zero direction: can,
    pub /: *mut *mut u32 usb_sof_count; / SOF count,
    pub /: *mut *mut u32 errors; / USB ERRORs count,
    pub tmpbuf: *mut u8,
    pub c_start: u32,
    pub c_end: u32,
    pub nullbuf: *mut u8,
    pub statusbuf: *mut u8,
    pub nullp: dma_addr_t,
    pub nullmap: u8,
    pub /: *mut *mut u8 device_address; / Device USB address,
    pub usb_clock: c_uint,
    pub usb_irq: c_uint,
    pub usb_regs: *mut usb_ctlr __iomem,
    pub rx_tasklet: tasklet_struct,
    pub /: *mut *mut *mut completion done; / to make sure release() is done,
}

pub const EP_STATE_IDLE: c_int = 0;
pub const EP_STATE_NACK: c_int = 1;
pub const EP_STATE_STALL: c_int = 2;
//
// transmit BD's status
//
pub const T_R: c_uint = 0x80000000         /* ready bit */;
pub const T_W: c_uint = 0x20000000         /* wrap bit */;
pub const T_I: c_uint = 0x10000000         /* interrupt on completion */;
pub const T_L: c_uint = 0x08000000         /* last */;
pub const T_TC: c_uint = 0x04000000         /* transmit CRC */;
pub const T_CNF: c_uint = 0x02000000         /* wait for  transmit confirm */;
pub const T_LSP: c_uint = 0x01000000         /* Low-speed transaction */;
pub const T_PID: c_uint = 0x00c00000         /* packet id */;
pub const T_NAK: c_uint = 0x00100000         /* No ack. */;
pub const T_STAL: c_uint = 0x00080000         /* Stall received */;
pub const T_TO: c_uint = 0x00040000         /* time out */;
pub const T_UN: c_uint = 0x00020000         /* underrun */;

pub const T_PID_SHIFT: c_int = 6;
pub const T_PID_DATA0: c_uint = 0x00800000         /* Data 0 toggle */;
pub const T_PID_DATA1: c_uint = 0x00c00000         /* Data 1 toggle */;
//
// receive BD's status
//
pub const R_E: c_uint = 0x80000000         /* buffer empty */;
pub const R_W: c_uint = 0x20000000         /* wrap bit */;
pub const R_I: c_uint = 0x10000000         /* interrupt on reception */;
pub const R_L: c_uint = 0x08000000         /* last */;
pub const R_F: c_uint = 0x04000000         /* first */;
pub const R_PID: c_uint = 0x00c00000         /* packet id */;
pub const R_NO: c_uint = 0x00100000         /* Rx Non Octet Aligned Packet */;
pub const R_AB: c_uint = 0x00080000         /* Frame Aborted */;
pub const R_CR: c_uint = 0x00040000         /* CRC Error */;
pub const R_OV: c_uint = 0x00020000         /* Overrun */;

pub const R_PID_DATA0: c_uint = 0x00000000;
pub const R_PID_DATA1: c_uint = 0x00400000;
pub const R_PID_SETUP: c_uint = 0x00800000;
pub const CPM_USB_STOP_TX: c_uint = 0x2e600000;
pub const CPM_USB_RESTART_TX: c_uint = 0x2e600000;
pub const CPM_USB_STOP_TX_OPCODE: c_uint = 0x0a;
pub const CPM_USB_RESTART_TX_OPCODE: c_uint = 0x0b;
pub const CPM_USB_EP_SHIFT: c_int = 5;
