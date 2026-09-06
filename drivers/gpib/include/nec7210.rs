//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/nec7210.h
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
// copyright            : (C) 2002 by Frank Mori Hess
//

// struct used to provide variables local to a nec7210 chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nec7210_priv {

    pub iobase: u32,

    pub mmiobase: *mut void __iomem,
    pub addresses: unsigned int offset; // offset between successive nec7210 io,
    pub dma_channel: c_uint,
    pub dma_buffer: *mut u8,
    pub buffer: unsigned int dma_buffer_length; // length of dma,
    pub dma: dma_addr_t dma_buffer_addr; // bus address of board->buffer for use with,
// software copy of bits written to registers
    pub reg_bits: [u8; 8],
    pub A: u8 auxa_bits; // bits written to auxiliary register,
    pub B: u8 auxb_bits; // bits written to auxiliary register,
// used to keep track of board's state, bit definitions given below
    pub state: c_ulong,
// lock for chips that extend the nec7210 registers by paging in alternate regs
    pub register_page_lock: spinlock_t,
// wrappers for outb, inb, readb, or writeb
    pub register_number): *mut *mut *mut u8 (read_byte)(struct nec7210_priv priv, unsigned int,
    pub register_number): *mut *mut *mut void (write_byte)(struct nec7210_priv priv, u8 byte, unsigned int,
    pub type: nec7210_chipset,
    pub talker_state: talker_function_state,
    pub listener_state: listener_function_state,
    pub private: *mut c_void,
    pub 1: unsigned srq_pending :,
}

// slightly shorter way to access read_byte and write_byte
// struct nec7210_priv.state bit numbers
// interface functions
extern "C" {
    pub fn nec7210_take_control(board: *mut gpib_board, priv: *mut nec7210_priv, syncronous: c_int) -> c_int;
}
extern "C" {
    pub fn nec7210_go_to_standby(board: *mut gpib_board, priv: *mut nec7210_priv) -> c_int;
}
extern "C" {
    pub fn nec7210_interface_clear(board: *mut gpib_board, priv: *mut nec7210_priv, assert: c_int);
}
extern "C" {
    pub fn nec7210_remote_enable(board: *mut gpib_board, priv: *mut nec7210_priv, enable: c_int);
}
extern "C" {
    pub fn nec7210_disable_eos(board: *mut gpib_board, priv: *mut nec7210_priv);
}
extern "C" {
    pub fn nec7210_update_status_nolock(board: *mut gpib_board, priv: *mut nec7210_priv) -> c_uint;
}
extern "C" {
    pub fn nec7210_parallel_poll(board: *mut gpib_board, priv: *mut nec7210_priv, result: *mut u8) -> c_int;
}
extern "C" {
    pub fn nec7210_serial_poll_status(board: *mut gpib_board, priv: *mut nec7210_priv) -> u8;
}
extern "C" {
    pub fn nec7210_return_to_local(board: *const gpib_board, priv: *mut nec7210_priv);
}
// utility functions
extern "C" {
    pub fn nec7210_board_reset(priv: *mut nec7210_priv, board: *const gpib_board);
}
extern "C" {
    pub fn nec7210_board_online(priv: *mut nec7210_priv, board: *const gpib_board);
}
extern "C" {
    pub fn nec7210_set_handshake_mode(board: *mut gpib_board, priv: *mut nec7210_priv, mode: c_int);
}
extern "C" {
    pub fn nec7210_release_rfd_holdoff(board: *mut gpib_board, priv: *mut nec7210_priv);
}
extern "C" {
    pub fn nec7210_read_data_in(board: *mut gpib_board, priv: *mut nec7210_priv, end: *mut c_int) -> u8;
}
// wrappers for io functions
extern "C" {
    pub fn nec7210_ioport_read_byte(priv: *mut nec7210_priv, register_num: c_uint) -> u8;
}
extern "C" {
    pub fn nec7210_ioport_write_byte(priv: *mut nec7210_priv, data: u8, register_num: c_uint);
}
extern "C" {
    pub fn nec7210_iomem_read_byte(priv: *mut nec7210_priv, register_num: c_uint) -> u8;
}
extern "C" {
    pub fn nec7210_iomem_write_byte(priv: *mut nec7210_priv, data: u8, register_num: c_uint);
}
extern "C" {
    pub fn nec7210_locking_ioport_read_byte(priv: *mut nec7210_priv, register_num: c_uint) -> u8;
}
extern "C" {
    pub fn nec7210_locking_iomem_read_byte(priv: *mut nec7210_priv, register_num: c_uint) -> u8;
}
// interrupt service routine
extern "C" {
    pub fn nec7210_interrupt(board: *mut gpib_board, priv: *mut nec7210_priv) -> irqreturn_t;
}
