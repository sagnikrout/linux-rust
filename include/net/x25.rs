//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/x25.h
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
// Declarations of X.25 Packet Layer type objects.
//
// History
// nov/17/96	Jonathan Naylor	  Initial version.
// mar/20/00	Daniela Squassoni Disabling/enabling of facilities
// negotiation.
//

pub const X25_ADDR_LEN: c_int = 16;

pub const X25_STD_MIN_LEN: c_int = 3;
pub const X25_EXT_MIN_LEN: c_int = 4;
pub const X25_GFI_SEQ_MASK: c_uint = 0x30;
pub const X25_GFI_STDSEQ: c_uint = 0x10;
pub const X25_GFI_EXTSEQ: c_uint = 0x20;
pub const X25_Q_BIT: c_uint = 0x80;
pub const X25_D_BIT: c_uint = 0x40;
pub const X25_STD_M_BIT: c_uint = 0x10;
pub const X25_EXT_M_BIT: c_uint = 0x01;
pub const X25_CALL_REQUEST: c_uint = 0x0B;
pub const X25_CALL_ACCEPTED: c_uint = 0x0F;
pub const X25_CLEAR_REQUEST: c_uint = 0x13;
pub const X25_CLEAR_CONFIRMATION: c_uint = 0x17;
pub const X25_DATA: c_uint = 0x00;
pub const X25_INTERRUPT: c_uint = 0x23;
pub const X25_INTERRUPT_CONFIRMATION: c_uint = 0x27;
pub const X25_RR: c_uint = 0x01;
pub const X25_RNR: c_uint = 0x05;
pub const X25_REJ: c_uint = 0x09;
pub const X25_RESET_REQUEST: c_uint = 0x1B;
pub const X25_RESET_CONFIRMATION: c_uint = 0x1F;
pub const X25_REGISTRATION_REQUEST: c_uint = 0xF3;
pub const X25_REGISTRATION_CONFIRMATION: c_uint = 0xF7;
pub const X25_RESTART_REQUEST: c_uint = 0xFB;
pub const X25_RESTART_CONFIRMATION: c_uint = 0xFF;
pub const X25_DIAGNOSTIC: c_uint = 0xF1;
pub const X25_ILLEGAL: c_uint = 0xFD;
// Define the various conditions that may exist
pub const X25_COND_ACK_PENDING: c_uint = 0x01;
pub const X25_COND_OWN_RX_BUSY: c_uint = 0x02;
pub const X25_COND_PEER_RX_BUSY: c_uint = 0x04;
// Define Link State constants.

pub const X25_DEFAULT_THROUGHPUT: c_uint = 0x0A			/* Default Throughput */;
pub const X25_DEFAULT_REVERSE: c_uint = 0x00			/* Default Reverse Charging */;
pub const X25_SMODULUS: c_int = 8;
pub const X25_EMODULUS: c_int = 128;
//
// X.25 Facilities constants.
//
pub const X25_FAC_CLASS_MASK: c_uint = 0xC0;
pub const X25_FAC_CLASS_A: c_uint = 0x00;
pub const X25_FAC_CLASS_B: c_uint = 0x40;
pub const X25_FAC_CLASS_C: c_uint = 0x80;
pub const X25_FAC_CLASS_D: c_uint = 0xC0;
pub const X25_FAC_REVERSE: c_uint = 0x01			/* also fast select */;
pub const X25_FAC_THROUGHPUT: c_uint = 0x02;
pub const X25_FAC_PACKET_SIZE: c_uint = 0x42;
pub const X25_FAC_WINDOW_SIZE: c_uint = 0x43;
pub const X25_MAX_FAC_LEN: c_int = 60;
pub const X25_MAX_CUD_LEN: c_int = 128;
pub const X25_FAC_CALLING_AE: c_uint = 0xCB;
pub const X25_FAC_CALLED_AE: c_uint = 0xC9;
pub const X25_MARKER: c_uint = 0x00;
pub const X25_DTE_SERVICES: c_uint = 0x0F;

// Bitset in x25_sock->flags for misc flags
pub const X25_Q_BIT_FLAG: c_int = 0;
pub const X25_INTERRUPT_FLAG: c_int = 1;
pub const X25_ACCPT_APPRV_FLAG: c_int = 2;
//
// struct x25_route - x25 routing entry
// @node - entry in x25_list_lock
// @address - Start of address range
// @sigdigits - Number of sig digits
// @dev - More than one for MLP
// @refcnt - reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_route {
    pub node: list_head,
    pub address: x25_address,
    pub sigdigits: c_uint,
    pub dev: *mut net_device,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_neigh {
    pub node: list_head,
    pub dev: *mut net_device,
    pub state: c_uint,
    pub extended: c_uint,
    pub queue: sk_buff_head,
    pub t20: c_ulong,
    pub t20timer: timer_list,
    pub global_facil_mask: c_ulong,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_sock {
    pub sk: sock,
    pub dest_addr: x25_address source_addr,,
    pub neighbour: *mut x25_neigh,
    pub cudmatchlength: unsigned int lci,,
    pub condition: unsigned char state,,
    pub vl: unsigned short vs, vr, va,,
    pub t23: unsigned long t2, t21, t22,,
    pub fraglen: c_ushort,
    pub flags: c_ulong,
    pub ack_queue: sk_buff_head,
    pub fragment_queue: sk_buff_head,
    pub interrupt_in_queue: sk_buff_head,
    pub interrupt_out_queue: sk_buff_head,
    pub timer: timer_list,
    pub causediag: x25_causediag,
    pub facilities: x25_facilities,
    pub dte_facilities: x25_dte_facilities,
    pub calluserdata: x25_calluserdata,
    pub /: *mut *mut unsigned long vc_facil_mask; / inc_call facilities mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_forward {
    pub node: list_head,
    pub lci: c_uint,
    pub dev1: *mut net_device,
    pub dev2: *mut net_device,
    pub refcnt: core::sync::atomic::AtomicI32,
}

// af_x25.c
extern "C" {
    pub fn x25_addr_ntoa(: *mut c_uchar, : *mut x25_address, : *mut x25_address) -> c_int;
}
extern "C" {
    pub fn x25_addr_aton(: *mut c_uchar, : *mut x25_address, : *mut x25_address) -> c_int;
}
extern "C" {
    pub fn x25_destroy_socket_from_timer(: *mut sock);
}
extern "C" {
    pub fn x25_rx_call_request(: *mut sk_buff, : *mut x25_neigh, int: unsigned) -> c_int;
}
extern "C" {
    pub fn x25_kill_by_neigh(: *mut x25_neigh);
}
// x25_dev.c
extern "C" {
    pub fn x25_send_frame(: *mut sk_buff, : *mut x25_neigh);
}
extern "C" {
    pub fn x25_establish_link(: *mut x25_neigh);
}
// x25_facilities.c
extern "C" {
    pub fn x25_limit_facilities(: *mut x25_facilities, : *mut x25_neigh);
}
// x25_forward.c
extern "C" {
    pub fn x25_clear_forward_by_lci(lci: c_uint);
}
extern "C" {
    pub fn x25_clear_forward_by_dev(: *mut net_device);
}
extern "C" {
    pub fn x25_forward_data(_arg: c_int, : *mut x25_neigh, : *mut sk_buff) -> c_int;
}
// x25_in.c
extern "C" {
    pub fn x25_process_rx_frame(: *mut sock, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn x25_backlog_rcv(: *mut sock, : *mut sk_buff) -> c_int;
}
// x25_link.c
extern "C" {
    pub fn x25_link_control(: *mut sk_buff, : *mut x25_neigh, short: unsigned);
}
extern "C" {
    pub fn x25_link_device_up(: *mut net_device);
}
extern "C" {
    pub fn x25_link_device_down(: *mut net_device);
}
extern "C" {
    pub fn x25_link_established(: *mut x25_neigh);
}
extern "C" {
    pub fn x25_link_terminated(: *mut x25_neigh);
}
extern "C" {
    pub fn x25_transmit_link(: *mut sk_buff, : *mut x25_neigh);
}
extern "C" {
    pub fn x25_subscr_ioctl(int: unsigned, : *mut void __user) -> c_int;
}
extern "C" {
    pub fn x25_link_free();
}
// x25_neigh.c
// x25_out.c
extern "C" {
    pub fn x25_output(: *mut sock, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn x25_kick(: *mut sock);
}
extern "C" {
    pub fn x25_enquiry_response(: *mut sock);
}
// x25_route.c
extern "C" {
    pub fn x25_route_device_down(dev: *mut net_device);
}
extern "C" {
    pub fn x25_route_ioctl(int: unsigned, : *mut void __user) -> c_int;
}
extern "C" {
    pub fn x25_route_free();
}
// x25_subr.c
extern "C" {
    pub fn x25_clear_queues(: *mut sock);
}
extern "C" {
    pub fn x25_frames_acked(: *mut sock, short: unsigned);
}
extern "C" {
    pub fn x25_requeue_frames(: *mut sock);
}
extern "C" {
    pub fn x25_validate_nr(: *mut sock, short: unsigned) -> c_int;
}
extern "C" {
    pub fn x25_write_internal(: *mut sock, _arg: c_int);
}
extern "C" {
    pub fn x25_disconnect(: *mut sock, _arg: c_int, char: unsigned, char: unsigned);
}
// x25_timer.c
extern "C" {
    pub fn x25_init_timers(sk: *mut sock);
}
extern "C" {
    pub fn x25_start_heartbeat(: *mut sock);
}
extern "C" {
    pub fn x25_start_t2timer(: *mut sock);
}
extern "C" {
    pub fn x25_start_t21timer(: *mut sock);
}
extern "C" {
    pub fn x25_start_t22timer(: *mut sock);
}
extern "C" {
    pub fn x25_start_t23timer(: *mut sock);
}
extern "C" {
    pub fn x25_stop_heartbeat(: *mut sock);
}
extern "C" {
    pub fn x25_stop_timer(: *mut sock);
}
extern "C" {
    pub fn x25_display_timer(: *mut sock) -> c_ulong;
}
extern "C" {
    pub fn x25_check_rbuf(: *mut sock);
}
// sysctl_net_x25.c

extern "C" {
    pub fn x25_register_sysctl() -> c_int;
}
extern "C" {
    pub fn x25_unregister_sysctl();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_skb_cb {
    pub flags: c_uint,
}

extern "C" {
    pub fn x25_proc_init() -> c_int;
}
extern "C" {
    pub fn x25_proc_exit();
}
