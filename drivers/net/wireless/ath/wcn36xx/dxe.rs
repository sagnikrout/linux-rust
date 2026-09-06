//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/dxe.h
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
// Copyright (c) 2013 Eugene Krasnikov <k.eugene.e@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// DXE registers
pub const WCN36XX_DXE_MEM_REG: c_int = 0;
pub const WCN36XX_CCU_DXE_INT_SELECT_RIVA: c_uint = 0x310;
pub const WCN36XX_CCU_DXE_INT_SELECT_PRONTO: c_uint = 0x10dc;
// Descriptor valid

// End of packet

// BD handling bit

// Source is a queue

// Destination is a queue

// Pointer address is a queue

// Release PDU when done

// STOP channel processing

// INT on descriptor done

// Endian byte swap enable

// Master endianness

// Transfer type
pub const WCN36xx_DXE_CTRL_XTYPE_SHIFT: c_int = 1;

// BMU Threshold select
pub const WCN36xx_DXE_CTRL_BTHLD_SEL_SHIFT: c_int = 9;

// Priority
pub const WCN36xx_DXE_CTRL_PRIO_SHIFT: c_int = 13;

// BD Template index
pub const WCN36xx_DXE_CTRL_BDT_IDX_SHIFT: c_int = 18;

// Transfer types:
// Host to host

// Host to BMU

// BMU to host

// TODO This must calculated properly but not hardcoded

pub const WCN36XX_DXE_WQ_RX_L: c_uint = 0xB;
pub const WCN36XX_DXE_WQ_RX_H: c_uint = 0x4;
// Channel enable or restart

// End of packet bit

// BD Handling bit

// Source is queue

// Destination is queue

// Pointer descriptor is queue

// Relase PDU when done

// Stop channel processing

// Enable external descriptor interrupt

// Enable channel interrupt on errors

// Enable Channel interrupt when done

// External descriptor enable

// Wait for valid bit

// Endianness is little endian

// Abort transfer

// Long descriptor format

// Endian byte swap enable

// Transfer type
pub const WCN36xx_DXE_CH_CTRL_XTYPE_SHIFT: c_int = 1;

// Channel BMU Threshold select
pub const WCN36xx_DXE_CH_CTRL_BTHLD_SEL_SHIFT: c_int = 9;

// Channel Priority
pub const WCN36xx_DXE_CH_CTRL_PRIO_SHIFT: c_int = 13;

// Counter select
pub const WCN36xx_DXE_CH_CTRL_SEL_SHIFT: c_int = 22;

// Channel BD template index
pub const WCN36xx_DXE_CH_CTRL_BDT_IDX_SHIFT: c_int = 29;

// DXE default control register values

// Common DXE registers

// #define WCN36XX_DXE_INT_CH6_MASK	0x00000040
// #define WCN36XX_DXE_INT_CH5_MASK	0x00000020
pub const WCN36XX_DXE_INT_CH4_MASK: c_uint = 0x00000010;
pub const WCN36XX_DXE_INT_CH3_MASK: c_uint = 0x00000008;
// #define WCN36XX_DXE_INT_CH2_MASK	0x00000004
pub const WCN36XX_DXE_INT_CH1_MASK: c_uint = 0x00000002;
pub const WCN36XX_DXE_INT_CH0_MASK: c_uint = 0x00000001;

pub const WCN36XX_CH_STAT_INT_DONE_MASK: c_uint = 0x00008000;
pub const WCN36XX_CH_STAT_INT_ERR_MASK: c_uint = 0x00004000;
pub const WCN36XX_CH_STAT_INT_ED_MASK: c_uint = 0x00002000;

pub const WCN36XX_DXE_REG_RESET: c_uint = 0x5c89;
// Temporary BMU Workqueue 4
pub const WCN36XX_DXE_BMU_WQ_RX_LOW: c_uint = 0xB;
pub const WCN36XX_DXE_BMU_WQ_RX_HIGH: c_uint = 0x4;
// DMA channel offset
pub const WCN36XX_DXE_TX_LOW_OFFSET: c_uint = 0x400;
pub const WCN36XX_DXE_TX_HIGH_OFFSET: c_uint = 0x500;
pub const WCN36XX_DXE_RX_LOW_OFFSET: c_uint = 0x440;
pub const WCN36XX_DXE_RX_HIGH_OFFSET: c_uint = 0x4C0;
// Address of the next DXE descriptor
pub const WCN36XX_DXE_CH_NEXT_DESC_ADDR: c_uint = 0x001C;

// DXE Descriptor source address
pub const WCN36XX_DXE_CH_SRC_ADDR: c_uint = 0x000C;

// DXE Descriptor address destination address
pub const WCN36XX_DXE_CH_DEST_ADDR: c_uint = 0x0014;

// Interrupt status
pub const WCN36XX_DXE_CH_STATUS_REG_ADDR: c_uint = 0x0004;

// DXE default control register

pub const WCN36XX_SMSM_WLAN_TX_ENABLE: c_uint = 0x00000400;
pub const WCN36XX_SMSM_WLAN_TX_RINGS_EMPTY: c_uint = 0x00000200;
// Interrupt control channel mask
pub const WCN36XX_INT_MASK_CHAN_TX_L: c_uint = 0x00000001;
pub const WCN36XX_INT_MASK_CHAN_RX_L: c_uint = 0x00000002;
pub const WCN36XX_INT_MASK_CHAN_RX_H: c_uint = 0x00000008;
pub const WCN36XX_INT_MASK_CHAN_TX_H: c_uint = 0x00000010;
pub const WCN36XX_BD_CHUNK_SIZE: c_int = 128;
pub const WCN36XX_PKT_SIZE: c_uint = 0xF20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_dxe_ch_type {
    WCN36XX_DXE_CH_TX_L,
    WCN36XX_DXE_CH_TX_H,
    WCN36XX_DXE_CH_RX_L,
    WCN36XX_DXE_CH_RX_H
}

// amount of descriptors per channel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_dxe_ch_desc_num {
    WCN36XX_DXE_CH_DESC_NUMB_TX_L		= 128,
    WCN36XX_DXE_CH_DESC_NUMB_TX_H		= 10,
    WCN36XX_DXE_CH_DESC_NUMB_RX_L		= 512,
    WCN36XX_DXE_CH_DESC_NUMB_RX_H		= 40
}

//
// struct wcn36xx_dxe_desc - describes descriptor of one DXE buffer
//
// @ctrl: is a union that consists of following bits:
// union {
// u32	valid		:1; //0 = DMA stop, 1 = DMA continue with this
// //descriptor
// u32	transfer_type	:2; //0 = Host to Host space
// u32	eop		:1; //End of Packet
// u32	bd_handling	:1; //if transferType = Host to BMU, then 0
// // means first 128 bytes contain BD, and 1
// // means create new empty BD
// u32	siq		:1; // SIQ
// u32	diq		:1; // DIQ
// u32	pdu_rel		:1; //0 = don't release BD and PDUs when done,
// // 1 = release them
// u32	bthld_sel	:4; //BMU Threshold Select
// u32	prio		:3; //Specifies the priority level to use for
// // the transfer
// u32	stop_channel	:1; //1 = DMA stops processing further, channel
// //requires re-enabling after this
// u32	intr		:1; //Interrupt on Descriptor Done
// u32	rsvd		:1; //reserved
// u32	size		:14;//14 bits used - ignored for BMU transfers,
// //only used for host to host transfers?
// } ctrl;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_dxe_desc {
    pub ctrl: u32,
    pub fr_len: u32,
    pub src_addr_l: u32,
    pub dst_addr_l: u32,
    pub phy_next_l: u32,
    pub src_addr_h: u32,
    pub dst_addr_h: u32,
    pub phy_next_h: u32,
    pub __packed: },
// DXE Control block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_dxe_ctl {
    pub next: *mut wcn36xx_dxe_ctl,
    pub desc: *mut wcn36xx_dxe_desc,
    pub desc_phy_addr: c_uint,
    pub ctl_blk_order: c_int,
    pub skb: *mut sk_buff,
    pub bd_cpu_addr: *mut c_void,
    pub bd_phy_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_dxe_ch {
    pub /: *mut *mut spinlock_t lock; / protects head/tail ptrs,
    pub ch_type: wcn36xx_dxe_ch_type,
    pub cpu_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub desc_num: wcn36xx_dxe_ch_desc_num,
// DXE control block ring
    pub head_blk_ctl: *mut wcn36xx_dxe_ctl,
    pub tail_blk_ctl: *mut wcn36xx_dxe_ctl,
// DXE channel specific configs
    pub dxe_wq: u32,
    pub ctrl_bd: u32,
    pub ctrl_skb: u32,
    pub reg_ctrl: u32,
    pub def_ctrl: u32,
}

// Memory Pool for BD headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_dxe_mem_pool {
    pub chunk_size: c_int,
    pub virt_addr: *mut c_void,
    pub phy_addr: dma_addr_t,
}

extern "C" {
    pub fn wcn36xx_dxe_allocate_mem_pools(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_dxe_free_mem_pools(wcn: *mut wcn36xx);
}
extern "C" {
    pub fn wcn36xx_dxe_rx_frame(wcn: *mut wcn36xx);
}
extern "C" {
    pub fn wcn36xx_dxe_alloc_ctl_blks(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_dxe_free_ctl_blks(wcn: *mut wcn36xx);
}
extern "C" {
    pub fn wcn36xx_dxe_init(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_dxe_deinit(wcn: *mut wcn36xx);
}
extern "C" {
    pub fn wcn36xx_dxe_init_channels(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_dxe_tx_flush(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_dxe_tx_ack_ind(wcn: *mut wcn36xx, status: u32);
}
