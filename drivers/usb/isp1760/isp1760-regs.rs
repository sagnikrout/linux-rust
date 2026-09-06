//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/isp1760/isp1760-regs.h
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
// Driver for the NXP ISP1760 chip
//
// Copyright 2021 Linaro, Rui Miguel Silva
// Copyright 2014 Laurent Pinchart
// Copyright 2007 Sebastian Siewior
//
// Contacts:
// Sebastian Siewior <bigeasy@linutronix.de>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Rui Miguel Silva <rui.silva@linaro.org>
//

// Macro flag: #define _ISP176x_REGS_H_
// -----------------------------------------------------------------------------
// Host Controller
//
// ISP1760/31
// EHCI capability registers
pub const ISP176x_HC_VERSION: c_uint = 0x002;
pub const ISP176x_HC_HCSPARAMS: c_uint = 0x004;
pub const ISP176x_HC_HCCPARAMS: c_uint = 0x008;
// EHCI operational registers
pub const ISP176x_HC_USBCMD: c_uint = 0x020;
pub const ISP176x_HC_USBSTS: c_uint = 0x024;
pub const ISP176x_HC_FRINDEX: c_uint = 0x02c;
pub const ISP176x_HC_CONFIGFLAG: c_uint = 0x060;
pub const ISP176x_HC_PORTSC1: c_uint = 0x064;
pub const ISP176x_HC_ISO_PTD_DONEMAP: c_uint = 0x130;
pub const ISP176x_HC_ISO_PTD_SKIPMAP: c_uint = 0x134;
pub const ISP176x_HC_ISO_PTD_LASTPTD: c_uint = 0x138;
pub const ISP176x_HC_INT_PTD_DONEMAP: c_uint = 0x140;
pub const ISP176x_HC_INT_PTD_SKIPMAP: c_uint = 0x144;
pub const ISP176x_HC_INT_PTD_LASTPTD: c_uint = 0x148;
pub const ISP176x_HC_ATL_PTD_DONEMAP: c_uint = 0x150;
pub const ISP176x_HC_ATL_PTD_SKIPMAP: c_uint = 0x154;
pub const ISP176x_HC_ATL_PTD_LASTPTD: c_uint = 0x158;
// Configuration Register
pub const ISP176x_HC_HW_MODE_CTRL: c_uint = 0x300;
pub const ISP176x_HC_CHIP_ID: c_uint = 0x304;
pub const ISP176x_HC_SCRATCH: c_uint = 0x308;
pub const ISP176x_HC_RESET: c_uint = 0x30c;
pub const ISP176x_HC_BUFFER_STATUS: c_uint = 0x334;
pub const ISP176x_HC_MEMORY: c_uint = 0x33c;
// Interrupt Register
pub const ISP176x_HC_INTERRUPT: c_uint = 0x310;
pub const ISP176x_HC_INTERRUPT_ENABLE: c_uint = 0x314;
pub const ISP176x_HC_ISO_IRQ_MASK_OR: c_uint = 0x318;
pub const ISP176x_HC_INT_IRQ_MASK_OR: c_uint = 0x31c;
pub const ISP176x_HC_ATL_IRQ_MASK_OR: c_uint = 0x320;
pub const ISP176x_HC_ISO_IRQ_MASK_AND: c_uint = 0x324;
pub const ISP176x_HC_INT_IRQ_MASK_AND: c_uint = 0x328;
pub const ISP176x_HC_ATL_IRQ_MASK_AND: c_uint = 0x32c;
pub const ISP176x_HC_OTG_CTRL: c_uint = 0x374;
pub const ISP176x_HC_OTG_CTRL_SET: c_uint = 0x374;
pub const ISP176x_HC_OTG_CTRL_CLEAR: c_uint = 0x376;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp176x_host_controller_fields {
// HC_PORTSC1
    PORT_OWNER, PORT_POWER, PORT_LSTATUS, PORT_RESET, PORT_SUSPEND,
    PORT_RESUME, PORT_PE, PORT_CSC, PORT_CONNECT,
// HC_HCSPARAMS
    HCS_PPC, HCS_N_PORTS,
// HC_HCCPARAMS
    HCC_ISOC_CACHE, HCC_ISOC_THRES,
// HC_USBCMD
    CMD_LRESET, CMD_RESET, CMD_RUN,
// HC_USBSTS
    STS_PCD,
// HC_FRINDEX
    HC_FRINDEX,
// HC_CONFIGFLAG
    FLAG_CF,
// ISO/INT/ATL PTD
    HC_ISO_PTD_DONEMAP, HC_ISO_PTD_SKIPMAP, HC_ISO_PTD_LASTPTD,
    HC_INT_PTD_DONEMAP, HC_INT_PTD_SKIPMAP, HC_INT_PTD_LASTPTD,
    HC_ATL_PTD_DONEMAP, HC_ATL_PTD_SKIPMAP, HC_ATL_PTD_LASTPTD,
// HC_HW_MODE_CTRL
    ALL_ATX_RESET, HW_ANA_DIGI_OC, HW_DEV_DMA, HW_COMN_IRQ, HW_COMN_DMA,
    HW_DATA_BUS_WIDTH, HW_DACK_POL_HIGH, HW_DREQ_POL_HIGH, HW_INTR_HIGH_ACT,
    HW_INTF_LOCK, HW_INTR_EDGE_TRIG, HW_GLOBAL_INTR_EN,
// HC_CHIP_ID
    HC_CHIP_ID_HIGH, HC_CHIP_ID_LOW, HC_CHIP_REV,
// HC_SCRATCH
    HC_SCRATCH,
// HC_RESET
    SW_RESET_RESET_ATX, SW_RESET_RESET_HC, SW_RESET_RESET_ALL,
// HC_BUFFER_STATUS
    ISO_BUF_FILL, INT_BUF_FILL, ATL_BUF_FILL,
// HC_MEMORY
    MEM_BANK_SEL, MEM_START_ADDR,
// HC_DATA
    HC_DATA,
// HC_INTERRUPT
    HC_INTERRUPT,
// HC_INTERRUPT_ENABLE
    HC_INT_IRQ_ENABLE, HC_ATL_IRQ_ENABLE,
// INTERRUPT MASKS
    HC_ISO_IRQ_MASK_OR, HC_INT_IRQ_MASK_OR, HC_ATL_IRQ_MASK_OR,
    HC_ISO_IRQ_MASK_AND, HC_INT_IRQ_MASK_AND, HC_ATL_IRQ_MASK_AND,
// HW_OTG_CTRL_SET
    HW_OTG_DISABLE, HW_SW_SEL_HC_DC, HW_VBUS_DRV, HW_SEL_CP_EXT,
    HW_DM_PULLDOWN, HW_DP_PULLDOWN, HW_DP_PULLUP, HW_HC_2_DIS,
// HW_OTG_CTRL_CLR
    HW_OTG_DISABLE_CLEAR, HW_SW_SEL_HC_DC_CLEAR, HW_VBUS_DRV_CLEAR,
    HW_SEL_CP_EXT_CLEAR, HW_DM_PULLDOWN_CLEAR, HW_DP_PULLDOWN_CLEAR,
    HW_DP_PULLUP_CLEAR, HW_HC_2_DIS_CLEAR,
// Last element
    HC_FIELD_MAX,
}

// ISP1763
// EHCI operational registers
pub const ISP1763_HC_USBCMD: c_uint = 0x8c;
pub const ISP1763_HC_USBSTS: c_uint = 0x90;
pub const ISP1763_HC_FRINDEX: c_uint = 0x98;
pub const ISP1763_HC_CONFIGFLAG: c_uint = 0x9c;
pub const ISP1763_HC_PORTSC1: c_uint = 0xa0;
pub const ISP1763_HC_ISO_PTD_DONEMAP: c_uint = 0xa4;
pub const ISP1763_HC_ISO_PTD_SKIPMAP: c_uint = 0xa6;
pub const ISP1763_HC_ISO_PTD_LASTPTD: c_uint = 0xa8;
pub const ISP1763_HC_INT_PTD_DONEMAP: c_uint = 0xaa;
pub const ISP1763_HC_INT_PTD_SKIPMAP: c_uint = 0xac;
pub const ISP1763_HC_INT_PTD_LASTPTD: c_uint = 0xae;
pub const ISP1763_HC_ATL_PTD_DONEMAP: c_uint = 0xb0;
pub const ISP1763_HC_ATL_PTD_SKIPMAP: c_uint = 0xb2;
pub const ISP1763_HC_ATL_PTD_LASTPTD: c_uint = 0xb4;
// Configuration Register
pub const ISP1763_HC_HW_MODE_CTRL: c_uint = 0xb6;
pub const ISP1763_HC_CHIP_REV: c_uint = 0x70;
pub const ISP1763_HC_CHIP_ID: c_uint = 0x72;
pub const ISP1763_HC_SCRATCH: c_uint = 0x78;
pub const ISP1763_HC_RESET: c_uint = 0xb8;
pub const ISP1763_HC_BUFFER_STATUS: c_uint = 0xba;
pub const ISP1763_HC_MEMORY: c_uint = 0xc4;
pub const ISP1763_HC_DATA: c_uint = 0xc6;
// Interrupt Register
pub const ISP1763_HC_INTERRUPT: c_uint = 0xd4;
pub const ISP1763_HC_INTERRUPT_ENABLE: c_uint = 0xd6;
pub const ISP1763_HC_ISO_IRQ_MASK_OR: c_uint = 0xd8;
pub const ISP1763_HC_INT_IRQ_MASK_OR: c_uint = 0xda;
pub const ISP1763_HC_ATL_IRQ_MASK_OR: c_uint = 0xdc;
pub const ISP1763_HC_ISO_IRQ_MASK_AND: c_uint = 0xde;
pub const ISP1763_HC_INT_IRQ_MASK_AND: c_uint = 0xe0;
pub const ISP1763_HC_ATL_IRQ_MASK_AND: c_uint = 0xe2;
pub const ISP1763_HC_OTG_CTRL_SET: c_uint = 0xe4;
pub const ISP1763_HC_OTG_CTRL_CLEAR: c_uint = 0xe6;
// -----------------------------------------------------------------------------
// Peripheral Controller
//

pub const ISP176x_DC_ENDPTYP_ISOC: c_uint = 0x01;
pub const ISP176x_DC_ENDPTYP_BULK: c_uint = 0x02;
pub const ISP176x_DC_ENDPTYP_INTERRUPT: c_uint = 0x03;
// Initialization Registers
pub const ISP176x_DC_ADDRESS: c_uint = 0x0200;
pub const ISP176x_DC_MODE: c_uint = 0x020c;
pub const ISP176x_DC_INTCONF: c_uint = 0x0210;
pub const ISP176x_DC_DEBUG: c_uint = 0x0212;
pub const ISP176x_DC_INTENABLE: c_uint = 0x0214;
// Data Flow Registers
pub const ISP176x_DC_EPMAXPKTSZ: c_uint = 0x0204;
pub const ISP176x_DC_EPTYPE: c_uint = 0x0208;
pub const ISP176x_DC_BUFLEN: c_uint = 0x021c;
pub const ISP176x_DC_BUFSTAT: c_uint = 0x021e;
pub const ISP176x_DC_DATAPORT: c_uint = 0x0220;
pub const ISP176x_DC_CTRLFUNC: c_uint = 0x0228;
pub const ISP176x_DC_EPINDEX: c_uint = 0x022c;
// DMA Registers
pub const ISP176x_DC_DMACMD: c_uint = 0x0230;
pub const ISP176x_DC_DMATXCOUNT: c_uint = 0x0234;
pub const ISP176x_DC_DMACONF: c_uint = 0x0238;
pub const ISP176x_DC_DMAHW: c_uint = 0x023c;
pub const ISP176x_DC_DMAINTREASON: c_uint = 0x0250;
pub const ISP176x_DC_DMAINTEN: c_uint = 0x0254;
pub const ISP176x_DC_DMAEP: c_uint = 0x0258;
pub const ISP176x_DC_DMABURSTCOUNT: c_uint = 0x0264;
// General Registers
pub const ISP176x_DC_INTERRUPT: c_uint = 0x0218;
pub const ISP176x_DC_CHIPID: c_uint = 0x0270;
pub const ISP176x_DC_FRAMENUM: c_uint = 0x0274;
pub const ISP176x_DC_SCRATCH: c_uint = 0x0278;
pub const ISP176x_DC_UNLOCKDEV: c_uint = 0x027c;
pub const ISP176x_DC_INTPULSEWIDTH: c_uint = 0x0280;
pub const ISP176x_DC_TESTMODE: c_uint = 0x0284;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp176x_device_controller_fields {
// DC_ADDRESS
    DC_DEVEN, DC_DEVADDR,
// DC_MODE
    DC_VBUSSTAT, DC_SFRESET, DC_GLINTENA,
// DC_INTCONF
    DC_CDBGMOD_ACK, DC_DDBGMODIN_ACK, DC_DDBGMODOUT_ACK, DC_INTPOL,
// DC_INTENABLE
    DC_IEPRXTX_7, DC_IEPRXTX_6, DC_IEPRXTX_5, DC_IEPRXTX_4, DC_IEPRXTX_3,
    DC_IEPRXTX_2, DC_IEPRXTX_1, DC_IEPRXTX_0,
    DC_IEP0SETUP, DC_IEVBUS, DC_IEHS_STA, DC_IERESM, DC_IESUSP, DC_IEBRST,
// DC_EPINDEX
    DC_EP0SETUP, DC_ENDPIDX, DC_EPDIR,
// DC_CTRLFUNC
    DC_CLBUF, DC_VENDP, DC_DSEN, DC_STATUS, DC_STALL,
// DC_BUFLEN
    DC_BUFLEN,
// DC_EPMAXPKTSZ
    DC_FFOSZ,
// DC_EPTYPE
    DC_EPENABLE, DC_ENDPTYP,
// DC_FRAMENUM
    DC_FRAMENUM, DC_UFRAMENUM,
// DC_CHIP_ID
    DC_CHIP_ID_HIGH, DC_CHIP_ID_LOW,
// DC_SCRATCH
    DC_SCRATCH,
// Last element
    DC_FIELD_MAX,
}

// ISP1763
// Initialization Registers
pub const ISP1763_DC_ADDRESS: c_uint = 0x00;
pub const ISP1763_DC_MODE: c_uint = 0x0c;
pub const ISP1763_DC_INTCONF: c_uint = 0x10;
pub const ISP1763_DC_INTENABLE: c_uint = 0x14;
// Data Flow Registers
pub const ISP1763_DC_EPMAXPKTSZ: c_uint = 0x04;
pub const ISP1763_DC_EPTYPE: c_uint = 0x08;
pub const ISP1763_DC_BUFLEN: c_uint = 0x1c;
pub const ISP1763_DC_BUFSTAT: c_uint = 0x1e;
pub const ISP1763_DC_DATAPORT: c_uint = 0x20;
pub const ISP1763_DC_CTRLFUNC: c_uint = 0x28;
pub const ISP1763_DC_EPINDEX: c_uint = 0x2c;
// DMA Registers
pub const ISP1763_DC_DMACMD: c_uint = 0x30;
pub const ISP1763_DC_DMATXCOUNT: c_uint = 0x34;
pub const ISP1763_DC_DMACONF: c_uint = 0x38;
pub const ISP1763_DC_DMAHW: c_uint = 0x3c;
pub const ISP1763_DC_DMAINTREASON: c_uint = 0x50;
pub const ISP1763_DC_DMAINTEN: c_uint = 0x54;
pub const ISP1763_DC_DMAEP: c_uint = 0x58;
pub const ISP1763_DC_DMABURSTCOUNT: c_uint = 0x64;
// General Registers
pub const ISP1763_DC_INTERRUPT: c_uint = 0x18;
pub const ISP1763_DC_CHIPID_LOW: c_uint = 0x70;
pub const ISP1763_DC_CHIPID_HIGH: c_uint = 0x72;
pub const ISP1763_DC_FRAMENUM: c_uint = 0x74;
pub const ISP1763_DC_SCRATCH: c_uint = 0x78;
pub const ISP1763_DC_UNLOCKDEV: c_uint = 0x7c;
pub const ISP1763_DC_INTPULSEWIDTH: c_uint = 0x80;
pub const ISP1763_DC_TESTMODE: c_uint = 0x84;
