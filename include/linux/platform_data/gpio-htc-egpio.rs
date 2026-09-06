//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/gpio-htc-egpio.h
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
// HTC simple EGPIO irq and gpio extender
//
// Descriptive values for all-in or all-out htc_egpio_chip descriptors.

pub const HTC_EGPIO_INPUT: c_int = 0;
//
// struct htc_egpio_chip - descriptor to create gpio_chip for register range
// @reg_start: index of first register
// @gpio_base: gpio number of first pin in this register range
// @num_gpios: number of gpios in this register range, max BITS_PER_LONG
// (number of registers = DIV_ROUND_UP(num_gpios, reg_width))
// @direction: bitfield, '0' = input, '1' = output,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_egpio_chip {
    pub reg_start: c_int,
    pub gpio_base: c_int,
    pub num_gpios: c_int,
    pub direction: c_ulong,
    pub initial_values: c_ulong,
}

//
// struct htc_egpio_platform_data - description provided by the arch
// @irq_base: beginning of available IRQs (eg, IRQ_BOARD_START)
// @num_irqs: number of irqs
// @reg_width: number of bits per register, either 8 or 16 bit
// @bus_width: alignment of the registers, either 16 or 32 bit
// @invert_acks: set if chip requires writing '0' to ack an irq, instead of '1'
// @ack_register: location of the irq/ack register
// @chip: pointer to array of htc_egpio_chip descriptors
// @num_chips: number of egpio chip descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_egpio_platform_data {
    pub bus_width: c_int,
    pub reg_width: c_int,
    pub irq_base: c_int,
    pub num_irqs: c_int,
    pub invert_acks: c_int,
    pub ack_register: c_int,
    pub chip: *mut htc_egpio_chip,
    pub num_chips: c_int,
}
