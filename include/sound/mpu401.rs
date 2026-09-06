//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/mpu401.h
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
// Header file for MPU-401 and compatible cards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

pub const MPU401_MODE_BIT_INPUT: c_int = 0;
pub const MPU401_MODE_BIT_OUTPUT: c_int = 1;
pub const MPU401_MODE_BIT_INPUT_TRIGGER: c_int = 2;
pub const MPU401_MODE_BIT_OUTPUT_TRIGGER: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mpu401 {
    pub rmidi: *mut snd_rawmidi,
    pub /: *mut *mut unsigned short hardware; / MPU401_HW_XXXX,
    pub /: *mut *mut unsigned int info_flags; / MPU401_INFO_XXX,
    pub /: *mut *mut unsigned long port; / base port of MPU-401 chip,
    pub /: *mut *mut unsigned long cport; / port + 1 (usually),
    pub /: *mut *mut *mut resource res; / port resource,
    pub /: *mut *mut int irq; / IRQ number of MPU-401 chip,
    pub /: *mut *mut unsigned long mode; / MPU401_MODE_XXXX,
    pub timer_invoked: c_int,
    pub mpu): *mut *mut *mut int (open_input) (struct snd_mpu401,
    pub mpu): *mut *mut *mut void (close_input) (struct snd_mpu401,
    pub mpu): *mut *mut *mut int (open_output) (struct snd_mpu401,
    pub mpu): *mut *mut *mut void (close_output) (struct snd_mpu401,
    pub private_data: *mut c_void,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub timer_lock: spinlock_t,
    pub timer: timer_list,
    pub addr): *mut *mut *mut void (write) (struct snd_mpu401  mpu, unsigned char data, unsigned long,
    pub addr): *mut *mut *mut unsigned char (read) (struct snd_mpu401 mpu, unsigned long,
}

// I/O ports

//
// control register bits
//
// read MPU401C()
pub const MPU401_RX_EMPTY: c_uint = 0x80;
pub const MPU401_TX_FULL: c_uint = 0x40;
// write MPU401C()
pub const MPU401_RESET: c_uint = 0xff;
pub const MPU401_ENTER_UART: c_uint = 0x3f;
// read MPU401D()
pub const MPU401_ACK: c_uint = 0xfe;
//
extern "C" {
    pub fn snd_mpu401_uart_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_mpu401_uart_interrupt_tx(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
