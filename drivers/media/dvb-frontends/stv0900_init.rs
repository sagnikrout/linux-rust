//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0900_init.h
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
// stv0900_init.h
//
// Driver for ST STV0900 satellite demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//

// DVBS2 C/N Look-Up table
// RF level C/N Look-Up table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_car_loop_optim {
    pub modcode: fe_stv0900_modcode,
    pub car_loop_pilots_on_2: u8,
    pub car_loop_pilots_off_2: u8,
    pub car_loop_pilots_on_5: u8,
    pub car_loop_pilots_off_5: u8,
    pub car_loop_pilots_on_10: u8,
    pub car_loop_pilots_off_10: u8,
    pub car_loop_pilots_on_20: u8,
    pub car_loop_pilots_off_20: u8,
    pub car_loop_pilots_on_30: u8,
    pub car_loop_pilots_off_30: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_short_frames_car_loop_optim {
    pub modulation: fe_stv0900_modulation,
    pub /: *mut *mut u8 car_loop_cut12_2; / Cut 1.2, SR<=3msps,
    pub /: *mut *mut u8 car_loop_cut20_2; / Cut 2.0, SR<3msps,
    pub /: *mut *mut u8 car_loop_cut12_5; / Cut 1.2, 3<SR<=7msps,
    pub /: *mut *mut u8 car_loop_cut20_5; / Cut 2.0, 3<SR<=7msps,
    pub /: *mut *mut u8 car_loop_cut12_10; / Cut 1.2, 7<SR<=15msps,
    pub /: *mut *mut u8 car_loop_cut20_10; / Cut 2.0, 7<SR<=15msps,
    pub /: *mut *mut u8 car_loop_cut12_20; / Cut 1.2, 10<SR<=25msps,
    pub /: *mut *mut u8 car_loop_cut20_20; / Cut 2.0, 10<SR<=25msps,
    pub /: *mut *mut u8 car_loop_cut12_30; / Cut 1.2, 25<SR<=45msps,
    pub /: *mut *mut u8 car_loop_cut20_30; / Cut 2.0, 10<SR<=45msps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_short_frames_car_loop_optim_vs_mod {
    pub modulation: fe_stv0900_modulation,
    pub /: *mut *mut u8 car_loop_2; / SR<3msps,
    pub /: *mut *mut u8 car_loop_5; / 3<SR<=7msps,
    pub /: *mut *mut u8 car_loop_10; / 7<SR<=15msps,
    pub /: *mut *mut u8 car_loop_20; / 10<SR<=25msps,
    pub /: *mut *mut u8 car_loop_30; / 10<SR<=45msps,
}

// Cut 1.x Tracking carrier loop carrier QPSK 1/2 to 8PSK 9/10 long Frame
// Modcod		2MPon	2MPoff	5MPon	5MPoff	10MPon
// Cut 2.0 Tracking carrier loop carrier QPSK 1/2 to 8PSK 9/10 long Frame
// Modcod		2MPon	2MPoff	5MPon	5MPoff	10MPon
// Cut 2.0 Tracking carrier loop carrier 16APSK 2/3 to 32APSK 9/10 long Frame
// Modcod		2MPon	2MPoff	5MPon	5MPoff	10MPon
// Cut 2.0 Tracking carrier loop carrier QPSK 1/4 to QPSK 2/5 long Frame
// Modcod		2MPon	2MPoff	5MPon	5MPoff	10MPon
// Cut 2.0 Tracking carrier loop carrier  short Frame, cut 1.2 and 2.0
// Mod		2Mcut1.2 2Mcut2.0 5Mcut1.2 5Mcut2.0 10Mcut1.2
// Modcod		2MPon	2MPoff	5MPon	5MPoff	10MPon
// Mod		2Mcut3.0 5Mcut3.0 10Mcut3.0 20Mcut3.0 30Mcut3.0
