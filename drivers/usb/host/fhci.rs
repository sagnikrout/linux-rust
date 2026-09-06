//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/fhci.h
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
// Freescale QUICC Engine USB Host Controller Driver
//
// Copyright (c) Freescale Semicondutor, Inc. 2006.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) Logic Product Development, Inc. 2007
// Peter Barada <peterb@logicpd.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

pub const USB_CLOCK: c_int = 48000000;
pub const FHCI_PRAM_SIZE: c_uint = 0x100;
pub const MAX_EDS: c_int = 32;
pub const MAX_TDS: c_int = 32;
// CRC16 field size
pub const CRC_SIZE: c_int = 2;
// USB protocol overhead for each frame transmitted from the host
pub const PROTOCOL_OVERHEAD: c_int = 7;
// Packet structure, info field
pub const PKT_PID_DATA0: c_uint = 0x80000000 /* PID - Data toggle zero */;
pub const PKT_PID_DATA1: c_uint = 0x40000000 /* PID - Data toggle one  */;
pub const PKT_PID_SETUP: c_uint = 0x20000000 /* PID - Setup bit */;
pub const PKT_SETUP_STATUS: c_uint = 0x10000000 /* Setup status bit */;
pub const PKT_SETADDR_STATUS: c_uint = 0x08000000 /* Set address status bit */;
pub const PKT_SET_HOST_LAST: c_uint = 0x04000000 /* Last data packet */;
pub const PKT_HOST_DATA: c_uint = 0x02000000 /* Data packet */;
pub const PKT_FIRST_IN_FRAME: c_uint = 0x01000000 /* First packet in the frame */;
pub const PKT_TOKEN_FRAME: c_uint = 0x00800000 /* Token packet */;
pub const PKT_ZLP: c_uint = 0x00400000 /* Zero length packet */;
pub const PKT_IN_TOKEN_FRAME: c_uint = 0x00200000 /* IN token packet */;
pub const PKT_OUT_TOKEN_FRAME: c_uint = 0x00100000 /* OUT token packet */;
pub const PKT_SETUP_TOKEN_FRAME: c_uint = 0x00080000 /* SETUP token packet */;
pub const PKT_STALL_FRAME: c_uint = 0x00040000 /* STALL packet */;
pub const PKT_NACK_FRAME: c_uint = 0x00020000 /* NACK packet */;
pub const PKT_NO_PID: c_uint = 0x00010000 /* No PID */;
pub const PKT_NO_CRC: c_uint = 0x00008000 /* don't append CRC */;
pub const PKT_HOST_COMMAND: c_uint = 0x00004000 /* Host command packet */;
pub const PKT_DUMMY_PACKET: c_uint = 0x00002000 /* Dummy packet, used for mmm */;
pub const PKT_LOW_SPEED_PACKET: c_uint = 0x00001000 /* Low-Speed packet */;

pub const PS_INT: c_int = 0;
pub const PS_DISCONNECTED: c_int = 1;
pub const PS_CONNECTED: c_int = 2;
pub const PS_READY: c_int = 3;
pub const PS_MISSING: c_int = 4;
// Transfer Descriptor status field
pub const USB_TD_OK: c_uint = 0x00000000 /* TD transmited or received ok */;
pub const USB_TD_INPROGRESS: c_uint = 0x80000000 /* TD is being transmitted */;
pub const USB_TD_RX_ER_NONOCT: c_uint = 0x40000000 /* Tx Non Octet Aligned Packet */;
pub const USB_TD_RX_ER_BITSTUFF: c_uint = 0x20000000 /* Frame Aborted-Received pkt */;
pub const USB_TD_RX_ER_CRC: c_uint = 0x10000000 /* CRC error */;
pub const USB_TD_RX_ER_OVERUN: c_uint = 0x08000000 /* Over - run occurred */;
pub const USB_TD_RX_ER_PID: c_uint = 0x04000000 /* wrong PID received */;
pub const USB_TD_RX_DATA_UNDERUN: c_uint = 0x02000000 /* shorter than expected */;
pub const USB_TD_RX_DATA_OVERUN: c_uint = 0x01000000 /* longer than expected */;
pub const USB_TD_TX_ER_NAK: c_uint = 0x00800000 /* NAK handshake */;
pub const USB_TD_TX_ER_STALL: c_uint = 0x00400000 /* STALL handshake */;
pub const USB_TD_TX_ER_TIMEOUT: c_uint = 0x00200000 /* transmit time out */;
pub const USB_TD_TX_ER_UNDERUN: c_uint = 0x00100000 /* transmit underrun */;

// Transfer Descriptor toggle field
pub const USB_TD_TOGGLE_DATA0: c_int = 0;
pub const USB_TD_TOGGLE_DATA1: c_int = 1;
pub const USB_TD_TOGGLE_CARRY: c_int = 2;
// #define MULTI_DATA_BUS
// Bus mode register RBMR/TBMR
pub const BUS_MODE_GBL: c_uint = 0x20	/* Global snooping */;
pub const BUS_MODE_BO: c_uint = 0x18	/* Byte ordering */;
pub const BUS_MODE_BO_BE: c_uint = 0x10	/* Byte ordering - Big-endian */;
pub const BUS_MODE_DTB: c_uint = 0x02	/* Data bus */;
// FHCI QE USB Register Description
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
// Freescale USB HOST
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_pram {
    pub /: *mut *mut __be16 ep_ptr[4]; / Endpoint porter reg,
    pub /: *mut *mut __be32 rx_state; / Rx internal state,
    pub /: *mut *mut __be32 rx_ptr; / Rx internal data pointer,
    pub /: *mut *mut __be16 frame_num; / Frame number,
    pub /: *mut *mut __be16 rx_cnt; / Rx byte count,
    pub /: *mut *mut __be32 rx_temp; / Rx temp,
    pub /: *mut *mut __be32 rx_data_temp; / Rx data temp,
    pub /: *mut *mut __be16 rx_u_ptr; / Rx microcode return address temp,
    pub /: *mut *mut u8 reserved1[2]; / reserved area,
    pub /: *mut *mut __be32 sof_tbl; / SOF lookup table pointer,
    pub /: *mut *mut u8 sof_u_crc_temp; / SOF micorcode CRC5 temp reg,
    pub reserved2: [u8; 0xdb],
}

// Freescale USB Endpoint
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_ep_pram {
    pub /: *mut *mut __be16 rx_base; / Rx BD base address,
    pub /: *mut *mut __be16 tx_base; / Tx BD base address,
    pub /: *mut *mut u8 rx_func_code; / Rx function code,
    pub /: *mut *mut u8 tx_func_code; / Tx function code,
    pub /: *mut *mut __be16 rx_buff_len; / Rx buffer length,
    pub /: *mut *mut __be16 rx_bd_ptr; / Rx BD pointer,
    pub /: *mut *mut __be16 tx_bd_ptr; / Tx BD pointer,
    pub /: *mut *mut __be32 tx_state; / Tx internal state,
    pub /: *mut *mut __be32 tx_ptr; / Tx internal data pointer,
    pub /: *mut *mut __be16 tx_crc; / temp transmit CRC,
    pub /: *mut *mut __be16 tx_cnt; / Tx byte count,
    pub /: *mut *mut __be32 tx_temp; / Tx temp,
    pub /: *mut *mut __be16 tx_u_ptr; / Tx microcode return address temp,
    pub reserved: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_controller_list {
    pub /: *mut *mut list_head ctrl_list; / control endpoints,
    pub /: *mut *mut list_head bulk_list; / bulk endpoints,
    pub /: *mut *mut list_head iso_list; / isochronous endpoints,
    pub /: *mut *mut list_head intr_list; / interruput endpoints,
    pub /: *mut *mut list_head done_list; / done transfers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtual_root_hub {
    pub /: *mut *mut int dev_num; / USB address of the root hub,
    pub /: *mut *mut u32 feature; / indicates what feature has been set,
    pub hub: usb_hub_status,
    pub port: usb_port_status,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_gpios {
    GPIO_USBOE = 0,
    GPIO_USBTP,
    GPIO_USBTN,
    GPIO_USBRP,
    GPIO_USBRN,
// these are optional
    GPIO_SPEED,
    GPIO_POWER,
    NUM_GPIOS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_pins {
    PIN_USBOE = 0,
    PIN_USBTP,
    PIN_USBTN,
    NUM_PINS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_hcd {
    pub fullspeed_clk: qe_clock,
    pub lowspeed_clk: qe_clock,
    pub pins: [*mut qe_pin; NUM_PINS],
    pub gpiods: [*mut gpio_desc; NUM_GPIOS],
    pub /: *mut *mut *mut qe_usb_ctlr __iomem regs; / I/O memory used to communicate,
    pub /: *mut *mut *mut fhci_pram __iomem pram; / Parameter RAM,
    pub timer: *mut gtm_timer,
    pub lock: spinlock_t,
    pub /: *mut *mut *mut fhci_usb usb_lld; / Low-level driver,
    pub /: *mut *mut *mut virtual_root_hub vroot_hub; / the virtual root hub,
    pub active_urbs: c_int,
    pub hc_list: *mut fhci_controller_list,
    pub /: *mut *mut *mut tasklet_process_done_task; / tasklet for done list,
    pub empty_eds: list_head,
    pub empty_tds: list_head,
    pub usb_irq_stat: [c_int; 13],
    pub dfs_root: *mut dentry,

}

pub const USB_FRAME_USAGE: c_int = 90;

pub const MAX_PERIODIC_FRAME_USAGE: c_int = 90;
// transaction type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_ta_type {
    FHCI_TA_IN = 0,	/* input transaction */
    FHCI_TA_OUT,	/* output transaction */
    FHCI_TA_SETUP,	/* setup transaction */
}

// transfer mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_tf_mode {
    FHCI_TF_CTRL = 0,
    FHCI_TF_ISO,
    FHCI_TF_BULK,
    FHCI_TF_INTR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_speed {
    FHCI_FULL_SPEED,
    FHCI_LOW_SPEED,
}

// endpoint state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_ed_state {
    FHCI_ED_NEW = 0, /* pipe is new */
    FHCI_ED_OPER,    /* pipe is operating */
    FHCI_ED_URB_DEL, /* pipe is in hold because urb is being deleted */
    FHCI_ED_SKIP,    /* skip this pipe */
    FHCI_ED_HALTED,  /* pipe is halted */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_port_status {
    FHCI_PORT_POWER_OFF = 0,
    FHCI_PORT_DISABLED,
    FHCI_PORT_DISCONNECTING,
    FHCI_PORT_WAITING,	/* waiting for connection */
    FHCI_PORT_FULL,		/* full speed connected */
    FHCI_PORT_LOW,		/* low speed connected */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhci_mem_alloc {
    MEM_CACHABLE_SYS = 0x00000001,	/* primary DDR,cachable */
    MEM_NOCACHE_SYS = 0x00000004,	/* primary DDR,non-cachable */
    MEM_SECONDARY = 0x00000002,	/* either secondary DDR or SDRAM */
    MEM_PRAM = 0x00000008,		/* multi-user RAM identifier */
}

// USB default parameters
pub const DEFAULT_RING_LEN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ed {
    pub /: *mut *mut u8 dev_addr; / device address,
    pub /: *mut *mut u8 ep_addr; / endpoint address,
    pub /: *mut *mut fhci_tf_mode mode; / USB transfer mode,
    pub speed: fhci_speed,
    pub max_pkt_size: c_uint,
    pub state: fhci_ed_state,
    pub /: *mut *mut list_head td_list; / a list of all queued TD to this pipe,
    pub node: list_head,
// read only parameters, should be cleared upon initialization
    pub /: *mut *mut u8 toggle_carry; / toggle carry from the last TD submitted,
    pub /: *mut *mut u16 next_iso; / time stamp of next queued ISO transfer,
    pub /: *mut *mut *mut td td_head; / a pointer to the current TD handled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct td {
    pub /: *mut *mut *mut void data; / a pointer to the data buffer,
    pub /: *mut *mut unsigned int len; / length of the data to be submitted,
    pub /: *mut *mut unsigned int actual_len; / actual bytes transferred on this td,
    pub /: *mut *mut fhci_ta_type type; / transaction type,
    pub /: *mut *mut u8 toggle; / toggle for next trans. within this TD,
    pub /: *mut *mut u16 iso_index; / ISO transaction index,
    pub /: *mut *mut u16 start_frame; / start frame time stamp,
    pub /: *mut *mut u16 interval; / interval between trans. (for ISO/Intr),
    pub /: *mut *mut u32 status; / status of the TD,
    pub /: *mut *mut *mut ed ed; / a handle to the corresponding ED,
    pub /: *mut *mut *mut urb urb; / a handle to the corresponding URB,
    pub /: *mut *mut bool ioc; / Inform On Completion,
    pub node: list_head,
// read only parameters should be cleared upon initialization
    pub pkt: *mut packet,
    pub nak_cnt: c_int,
    pub error_cnt: c_int,
    pub frame_lh: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet {
    pub /: *mut *mut *mut u8 data; / packet data,
    pub /: *mut *mut u32 len; / packet length,
    pub status: *mut *mut u32 status; / status of the packet - equivalent to the,
// field for the corresponding structure td
    pub /: *mut *mut u32 info; / packet information,
    pub /: *mut *mut *mut void __iomem priv_data; / private data of the driver (TDs or BDs),
}

// struct for each URB
pub const URB_INPROGRESS: c_int = 0;
pub const URB_DEL: c_int = 1;
// URB states (state field)
pub const US_BULK: c_int = 0;
pub const US_BULK0: c_int = 1;
// three setup states
pub const US_CTRL_SETUP: c_int = 2;
pub const US_CTRL_DATA: c_int = 1;
pub const US_CTRL_ACK: c_int = 0;
pub const EP_ZERO: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_priv {
    pub num_of_tds: c_int,
    pub tds_cnt: c_int,
    pub state: c_int,
    pub ed: *mut ed,
    pub time_out: timer_list,
    pub __counted_by(num_of_tds): *mut *mut td tds[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct endpoint {
// Pointer to ep parameter RAM
    pub ep_pram_ptr: *mut fhci_ep_pram __iomem,
// Host transactions
    pub /: *mut *mut *mut usb_td __iomem td_base; / first TD in the ring,
    pub /: *mut *mut *mut usb_td __iomem conf_td; / next TD for confirm after transac,
    pub /: *mut *mut *mut usb_td __iomem empty_td;/ next TD for new transaction req.,
    pub /: *mut *mut kfifo empty_frame_Q; / Empty frames list to use,
    pub /: *mut *mut kfifo conf_frame_Q; / frames passed to TDs,waiting for tx,
    pub /: *mut *mut kfifo dummy_packets_Q;/ dummy packets for the CRC overun,
    pub already_pushed_dummy_bd: bool,
}

// struct for each 1mSec frame time
pub const FRAME_IS_TRANSMITTED: c_uint = 0x00;
pub const FRAME_TIMER_END_TRANSMISSION: c_uint = 0x01;
pub const FRAME_DATA_END_TRANSMISSION: c_uint = 0x02;
pub const FRAME_END_TRANSMISSION: c_uint = 0x03;
pub const FRAME_IS_PREPARED: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_time_frame {
    pub /: *mut *mut u16 frame_num; / frame number,
    pub /: *mut *mut u16 total_bytes; / total bytes submitted within this frame,
    pub /: *mut *mut u8 frame_status; / flag that indicates to stop fill this frame,
    pub /: *mut *mut list_head tds_list; / all tds of this frame,
}

// internal driver structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhci_usb {
    pub /: *mut *mut u16 saved_msk; / saving of the USB mask register,
    pub /: *mut *mut *mut endpoint ep0; / pointer for endpoint0 structure,
    pub /: *mut *mut int intr_nesting_cnt; / interrupt nesting counter,
    pub /: *mut *mut u16 max_frame_usage; / max frame time usage,in micro-sec,
    pub /: *mut *mut u16 max_bytes_per_frame; / max byte can be tx in one time frame,
    pub /: *mut *mut u32 sw_transaction_time; / sw complete trans time,in micro-sec,
    pub actual_frame: *mut fhci_time_frame,
    pub /: *mut *mut *mut fhci_controller_list hc_list; / main structure for hc,
    pub vroot_hub: *mut virtual_root_hub,
    pub /: *mut *mut fhci_port_status port_status; / v_rh port status,
    pub fhci): *mut *mut u32 (transfer_confirm)(struct fhci_hcd,
    pub fhci: *mut fhci_hcd,
}

//
// Various helpers and prototypes below.
//

extern "C" {
    pub fn container_of()fhci: *mut (void, usb_hcd: struct, _arg: hcd_priv) -> return;
}
// fifo of pointers
extern "C" {
    pub fn kfifo_alloc(_arg: fifo, ): *mut *mut size  sizeof(void, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn kfifo_len(: *mut kfifo) / sizeof(void) -> return;
}
extern "C" {
    pub fn kfifo_in(_arg: kfifo, )&p: *mut (void, _arg: sizeof(p)) -> return;
}
// fhci-hcd.c
extern "C" {
    pub fn fhci_start_sof_timer(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_stop_sof_timer(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_get_sof_timer_count(usb: *mut fhci_usb) -> u16;
}
extern "C" {
    pub fn fhci_usb_enable_interrupt(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_usb_disable_interrupt(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_ioports_check_bus_state(fhci: *mut fhci_hcd) -> c_int;
}
// fhci-mem.c
extern "C" {
    pub fn fhci_recycle_empty_td(fhci: *mut fhci_hcd, td: *mut td);
}
extern "C" {
    pub fn fhci_recycle_empty_ed(fhci: *mut fhci_hcd, ed: *mut ed);
}
extern "C" {
    pub fn fhci_add_tds_to_ed(ed: *mut ed, td_list: *mut td, number: c_int);
}
// fhci-hub.c
extern "C" {
    pub fn fhci_port_disable(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_port_enable(lld: *mut c_void);
}
extern "C" {
    pub fn fhci_io_port_generate_reset(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_port_reset(lld: *mut c_void);
}
extern "C" {
    pub fn fhci_hub_status_data(hcd: *mut usb_hcd, buf: *mut c_char) -> c_int;
}
// fhci-tds.c
extern "C" {
    pub fn fhci_flush_bds(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_flush_actual_frame(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_host_transmit_actual_frame(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_tx_conf_interrupt(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_push_dummy_bd(ep: *mut endpoint);
}
extern "C" {
    pub fn fhci_ep0_free(usb: *mut fhci_usb);
}
// fhci-sched.c
extern "C" {
    pub fn fhci_transaction_confirm(usb: *mut fhci_usb, pkt: *mut packet);
}
extern "C" {
    pub fn fhci_flush_all_transmissions(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_schedule_transactions(usb: *mut fhci_usb);
}
extern "C" {
    pub fn fhci_device_connected_interrupt(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_device_disconnected_interrupt(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_queue_urb(fhci: *mut fhci_hcd, urb: *mut urb);
}
extern "C" {
    pub fn fhci_transfer_confirm_callback(fhci: *mut fhci_hcd) -> u32;
}
extern "C" {
    pub fn fhci_irq(hcd: *mut usb_hcd) -> irqreturn_t;
}
extern "C" {
    pub fn fhci_frame_limit_timer_irq(irq: c_int, _hcd: *mut c_void) -> irqreturn_t;
}
// fhci-q.h
extern "C" {
    pub fn fhci_urb_complete_free(fhci: *mut fhci_hcd, urb: *mut urb);
}
extern "C" {
    pub fn fhci_move_td_from_ed_to_done_list(usb: *mut fhci_usb, ed: *mut ed);
}
extern "C" {
    pub fn fhci_add_td_to_frame(frame: *mut fhci_time_frame, td: *mut td);
}
extern "C" {
    pub fn fhci_done_td(urb: *mut urb, td: *mut td);
}
extern "C" {
    pub fn fhci_del_ed_list(fhci: *mut fhci_hcd, ed: *mut ed);
}

extern "C" {
    pub fn fhci_dbg_isr(fhci: *mut fhci_hcd, usb_er: c_int);
}
extern "C" {
    pub fn fhci_dfs_destroy(fhci: *mut fhci_hcd);
}
extern "C" {
    pub fn fhci_dfs_create(fhci: *mut fhci_hcd);
}

