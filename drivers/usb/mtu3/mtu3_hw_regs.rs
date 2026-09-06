//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/mtu3/mtu3_hw_regs.h
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
// mtu3_hw_regs.h - MediaTek USB3 DRD register and field definitions
//
// Copyright (C) 2016 MediaTek Inc.
//
// Author: Chunfeng Yun <chunfeng.yun@mediatek.com>
//
// segment offset of MAC register
pub const SSUSB_DEV_BASE: c_uint = 0x0000;
pub const SSUSB_EPCTL_CSR_BASE: c_uint = 0x0800;
pub const SSUSB_USB3_MAC_CSR_BASE: c_uint = 0x1400;
pub const SSUSB_USB3_SYS_CSR_BASE: c_uint = 0x1400;
pub const SSUSB_USB2_CSR_BASE: c_uint = 0x2400;
// IPPC register in Infra
pub const SSUSB_SIFSLV_IPPC_BASE: c_uint = 0x0000;
// --------------- SSUSB_DEV REGISTER DEFINITION ---------------

// ---------------- SSUSB_DEV FIELD DEFINITION ---------------
// U3D_LV1ISR

// U3D_LV1IECR

// U3D_EPISR

// U3D_EP0CSR

// U3D_TX1CSR0

// U3D_TX1CSR1

// for TX_TYPE & RX_TYPE

// U3D_TX1CSR2

// U3D_RX1CSR0

// U3D_RX1CSR1

// U3D_RX1CSR2

// U3D_QCR0

// U3D_QCR1

// U3D_QCR3

// U3D_TXQHIAR1
// U3D_RXQHIAR1

// U3D_TXQCSR1
// U3D_RXQCSR1

// U3D_QISAR0, U3D_QIER0, U3D_QIESR0, U3D_QIECR0

// U3D_QISAR1, U3D_QIER1, U3D_QIESR1, U3D_QIECR1

// U3D_TQERRIR0, U3D_TQERRIER0, U3D_TQERRIESR0, U3D_TQERRIECR0

// U3D_RQERRIR0, U3D_RQERRIER0, U3D_RQERRIESR0, U3D_RQERRIECR0

// U3D_RQERRIR1, U3D_RQERRIER1, U3D_RQERRIESR1, U3D_RQERRIECR1

// U3D_CAP_EPINFO

// U3D_MISC_CTRL

// ---------------- SSUSB_EPCTL_CSR REGISTER DEFINITION ----------------

// ---------------- SSUSB_EPCTL_CSR FIELD DEFINITION ----------------
// U3D_DEVICE_CONF

// U3D_EP_RST

// U3D_DEV_LINK_INTR_ENABLE
// U3D_DEV_LINK_INTR

// ---------------- SSUSB_USB3_MAC_CSR REGISTER DEFINITION ----------------

// ---------------- SSUSB_USB3_MAC_CSR FIELD DEFINITION ----------------
// U3D_LTSSM_CTRL

// U3D_USB3_CONFIG

// U3D_LINK_STATE_MACHINE

// U3D_LTSSM_INTR_ENABLE
// U3D_LTSSM_INTR

// U3D_U3U2_SWITCH_CTRL

// ---------------- SSUSB_USB3_SYS_CSR REGISTER DEFINITION ----------------

// ---------------- SSUSB_USB3_SYS_CSR FIELD DEFINITION ----------------
// U3D_LINK_UX_INACT_TIMER

// U3D_LINK_POWER_CONTROL

// U3D_LINK_ERR_COUNT

// U3D_DEV_NOTIF_0

// Notification Type

// ---------------- SSUSB_USB2_CSR REGISTER DEFINITION ----------------

// ---------------- SSUSB_USB2_CSR FIELD DEFINITION ----------------
// U3D_POWER_MANAGEMENT

// U3D_DEVICE_CONTROL

// U3D_USB2_TEST_MODE

// U3D_COMMON_USB_INTR_ENABLE
// U3D_COMMON_USB_INTR

// U3D_LINK_RESET_INFO

// U3D_USB20_LPM_PARAMETER

// U3D_USB20_MISC_CONTROL

// ---------------- SSUSB_SIFSLV_IPPC REGISTER DEFINITION ----------------

// ---------------- SSUSB_SIFSLV_IPPC FIELD DEFINITION ----------------
// U3D_SSUSB_IP_PW_CTRL0

// U3D_SSUSB_IP_PW_CTRL1

// U3D_SSUSB_IP_PW_CTRL2

// U3D_SSUSB_IP_PW_CTRL3

// U3D_SSUSB_IP_PW_STS1

// U3D_SSUSB_IP_PW_STS2

// U3D_SSUSB_OTG_STS

// U3D_SSUSB_OTG_STS_CLR

// U3D_SSUSB_IP_XHCI_CAP

// U3D_SSUSB_IP_DEV_CAP

// U3D_SSUSB_OTG_INT_EN

// U3D_SSUSB_U3_CTRL_0P

// U3D_SSUSB_U2_CTRL_0P

// U3D_SSUSB_DEV_RST_CTRL

// U3D_SSUSB_IP_TRUNK_VERS

