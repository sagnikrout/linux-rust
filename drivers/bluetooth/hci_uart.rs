//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/hci_uart.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Bluetooth HCI UART driver
//
// Copyright (C) 2000-2001  Qualcomm Incorporated
// Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
// Copyright (C) 2004-2005  Marcel Holtmann <marcel@holtmann.org>
//

pub const N_HCI: c_int = 15;

// Ioctls

// UART protocols
pub const HCI_UART_MAX_PROTO: c_int = 13;
pub const HCI_UART_H4: c_int = 0;
pub const HCI_UART_BCSP: c_int = 1;
pub const HCI_UART_3WIRE: c_int = 2;
pub const HCI_UART_H4DS: c_int = 3;
pub const HCI_UART_LL: c_int = 4;
pub const HCI_UART_ATH3K: c_int = 5;
pub const HCI_UART_INTEL: c_int = 6;
pub const HCI_UART_BCM: c_int = 7;
pub const HCI_UART_QCA: c_int = 8;
pub const HCI_UART_AG6XX: c_int = 9;
pub const HCI_UART_NOKIA: c_int = 10;
pub const HCI_UART_MRVL: c_int = 11;
pub const HCI_UART_AML: c_int = 12;
pub const HCI_UART_RAW_DEVICE: c_int = 0;
pub const HCI_UART_RESET_ON_INIT: c_int = 1;
pub const HCI_UART_INIT_PENDING: c_int = 3;
pub const HCI_UART_EXT_CONFIG: c_int = 4;
pub const HCI_UART_VND_DETECT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_uart_proto {
    pub id: c_uint,
    pub name: *const c_char,
    pub manufacturer: c_uint,
    pub init_speed: c_uint,
    pub oper_speed: c_uint,
    pub hu): *mut *mut int (open)(struct hci_uart,
    pub hu): *mut *mut int (close)(struct hci_uart,
    pub hu): *mut *mut int (flush)(struct hci_uart,
    pub hu): *mut *mut int (setup)(struct hci_uart,
    pub speed): *mut *mut *mut int (set_baudrate)(struct hci_uart hu, unsigned int,
    pub len): *const *const *const *const int (recv)(struct hci_uart hu, void data, int,
    pub skb): *mut *mut *mut int (enqueue)(struct hci_uart hu, struct sk_buff,
    pub hu): *mut *mut *mut sk_buff (dequeue)(hci_uart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_uart {
    pub tty: *mut tty_struct,
    pub serdev: *mut serdev_device,
    pub hdev: *mut hci_dev,
    pub flags: c_ulong,
    pub hdev_flags: c_ulong,
    pub init_ready: work_struct,
    pub write_work: work_struct,
    pub proto: *const hci_uart_proto,
    pub /: *mut *mut percpu_rw_semaphore proto_lock; / Stop work for proto close,
    pub priv: *mut c_void,
    pub tx_skb: *mut sk_buff,
    pub tx_state: c_ulong,
    pub init_speed: c_uint,
    pub oper_speed: c_uint,
    pub alignment: u8,
    pub padding: u8,
}

// HCI_UART proto flag bits
pub const HCI_UART_PROTO_SET: c_int = 0;
pub const HCI_UART_REGISTERED: c_int = 1;
pub const HCI_UART_PROTO_READY: c_int = 2;
pub const HCI_UART_NO_SUSPEND_NOTIFIER: c_int = 3;
pub const HCI_UART_PROTO_INIT: c_int = 4;
// TX states
pub const HCI_UART_SENDING: c_int = 1;
pub const HCI_UART_TX_WAKEUP: c_int = 2;
extern "C" {
    pub fn hci_uart_register_proto(p: *const hci_uart_proto) -> c_int;
}
extern "C" {
    pub fn hci_uart_unregister_proto(p: *const hci_uart_proto) -> c_int;
}
extern "C" {
    pub fn hci_uart_register_device_priv(_arg: hu, _arg: p, _arg: 0) -> return;
}
extern "C" {
    pub fn hci_uart_unregister_device(hu: *mut hci_uart);
}
extern "C" {
    pub fn hci_uart_tx_wakeup(hu: *mut hci_uart) -> c_int;
}
extern "C" {
    pub fn hci_uart_wait_until_sent(hu: *mut hci_uart) -> c_int;
}
extern "C" {
    pub fn hci_uart_init_ready(hu: *mut hci_uart) -> c_int;
}
extern "C" {
    pub fn hci_uart_init_work(work: *mut work_struct);
}
extern "C" {
    pub fn hci_uart_set_baudrate(hu: *mut hci_uart, speed: c_uint);
}
extern "C" {
    pub fn hci_uart_has_flow_control(hu: *mut hci_uart) -> bool;
}
extern "C" {
    pub fn hci_uart_set_flow_control(hu: *mut hci_uart, enable: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h4_recv_pkt {
    pub /: *mut *mut u8 type; / Packet type,
    pub /: *mut *mut u8 hlen; / Header length,
    pub /: *mut *mut u8 loff; / Data length offset in header,
    pub /: *mut *mut u8 lsize; / Data length field size,
    pub /: *mut *mut u16 maxlen; / Max overall packet length,
    pub skb): *mut *mut *mut int (recv)(struct hci_dev hdev, struct sk_buff,
}

extern "C" {
    pub fn h4_init() -> c_int;
}
extern "C" {
    pub fn h4_deinit() -> c_int;
}

extern "C" {
    pub fn bcsp_init() -> c_int;
}
extern "C" {
    pub fn bcsp_deinit() -> c_int;
}

extern "C" {
    pub fn ll_init() -> c_int;
}
extern "C" {
    pub fn ll_deinit() -> c_int;
}

extern "C" {
    pub fn ath_init() -> c_int;
}
extern "C" {
    pub fn ath_deinit() -> c_int;
}

extern "C" {
    pub fn h5_init() -> c_int;
}
extern "C" {
    pub fn h5_deinit() -> c_int;
}

extern "C" {
    pub fn intel_init() -> c_int;
}
extern "C" {
    pub fn intel_deinit() -> c_int;
}

extern "C" {
    pub fn bcm_init() -> c_int;
}
extern "C" {
    pub fn bcm_deinit() -> c_int;
}

extern "C" {
    pub fn qca_init() -> c_int;
}
extern "C" {
    pub fn qca_deinit() -> c_int;
}

extern "C" {
    pub fn ag6xx_init() -> c_int;
}
extern "C" {
    pub fn ag6xx_deinit() -> c_int;
}

extern "C" {
    pub fn mrvl_init() -> c_int;
}
extern "C" {
    pub fn mrvl_deinit() -> c_int;
}

extern "C" {
    pub fn aml_init() -> c_int;
}
extern "C" {
    pub fn aml_deinit() -> c_int;
}
