//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_host.h
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
// MUSB OTG driver host defines
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//

// stored in "usb_host_endpoint.hcpriv" for scheduled endpoints
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_qh {
    pub /: *mut *mut *mut usb_host_endpoint hep; / usbcore info,
    pub dev: *mut usb_device,
    pub /: *mut *mut *mut musb_hw_ep hw_ep; / current binding,
    pub /: *mut *mut list_head ring; / of musb_qh,
// struct musb_qh		*next; */	/* for periodic tree
    pub /: *mut *mut u8 mux; / qh multiplexed to hw_ep,
    pub /: *mut *mut unsigned offset; / in urb->transfer_buffer,
    pub /: *mut *mut unsigned segsize; / current xfer fragment,
    pub /: *mut *mut u8 type_reg; / {rx,tx} type register,
    pub /: *mut *mut u8 intv_reg; / {rx,tx} interval register,
    pub /: *mut *mut u8 addr_reg; / device address register,
    pub /: *mut *mut u8 h_addr_reg; / hub address register,
    pub /: *mut *mut u8 h_port_reg; / hub port register,
    pub /: *mut *mut u8 is_ready; / safe to modify hw_ep,
    pub /: *mut *mut *mut u8 type; / XFERTYPE_,
    pub epnum: u8,
    pub /: *mut *mut u8 hb_mult; / high bandwidth pkts per uf,
    pub maxpacket: u16,
    pub /: *mut *mut u16 frame; / for periodic schedule,
    pub /: *mut *mut unsigned iso_idx; / in urb->iso_frame_desc[],
    pub /: *mut *mut sg_mapping_iter sg_miter; / for highmem in PIO mode,
    pub /: *mut *mut bool use_sg; / to track urb using sglist,
}

// map from control or bulk queue head to the first qh on that ring
extern "C" {
    pub fn list_entry(_arg: q->next, musb_qh: struct, _arg: ring) -> return;
}

extern "C" {
    pub fn musb_h_ep0_irq(: *mut musb) -> irqreturn_t;
}
extern "C" {
    pub fn musb_host_alloc(: *mut musb) -> c_int;
}
extern "C" {
    pub fn musb_host_setup(: *mut musb, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn musb_host_cleanup(: *mut musb);
}
extern "C" {
    pub fn musb_host_tx(: *mut musb, _arg: u8);
}
extern "C" {
    pub fn musb_host_rx(: *mut musb, _arg: u8);
}
extern "C" {
    pub fn musb_root_disconnect(musb: *mut musb);
}
extern "C" {
    pub fn musb_host_free(: *mut musb);
}
extern "C" {
    pub fn musb_host_resume_root_hub(musb: *mut musb);
}
extern "C" {
    pub fn musb_host_poke_root_hub(musb: *mut musb);
}
extern "C" {
    pub fn musb_port_suspend(musb: *mut musb, do_suspend: bool) -> c_int;
}
extern "C" {
    pub fn musb_port_reset(musb: *mut musb, do_reset: bool);
}
extern "C" {
    pub fn musb_host_finish_resume(work: *mut work_struct);
}

extern "C" {
    pub fn musb_hub_status_data(hcd: *mut usb_hcd, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn list_entry(_arg: queue->next, urb: struct, _arg: urb_list) -> return;
}
