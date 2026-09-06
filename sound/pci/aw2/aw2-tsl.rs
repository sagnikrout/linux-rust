//! Automatically rewritten from C to Rust
//! Source: sound/pci/aw2/aw2-tsl.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
// Jean-Christian Hassler <jhassler@free.fr>
// Copyright 1998 Emagic Soft- und Hardware GmbH
// Copyright 2002 Martijn Sipkema
//
// This file is part of the Audiowerk2 ALSA driver
//

// Audiowerk8 hardware setup:
// WS0, SD4, TSL1  - Analog/ digital in
// WS1, SD0, TSL1  - Analog out #1, digital out
// WS2, SD2, TSL1  - Analog out #2
// WS3, SD1, TSL2  - Analog out #3
// WS4, SD3, TSL2  - Analog out #4
// Audiowerk8 timing:
// Timeslot:     | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | ...
// A1_INPUT:
// SD4:          <_ADC-L_>-------<_ADC-R_>-------<
// WS0:          _______________/---------------\_
// A1_OUTPUT:
// SD0:          <_1-L___>-------<_1-R___>-------<
// WS1:          _______________/---------------\_
// SD2:          >-------<_2-L___>-------<_2-R___>
// WS2:          -------\_______________/---------
// A2_OUTPUT:
// SD1:          <_3-L___>-------<_3-R___>-------<
// WS3:          _______________/---------------\_
// SD3:          >-------<_4-L___>-------<_4-R___>
// WS4:          -------\_______________/---------
    static const int tsl1[8] = {
    1 * TSL_SDW_A1 | 3 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_LF_A1,
    1 * TSL_SDW_A1 | 2 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,
    0 * TSL_SDW_A1 | 3 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,
    0 * TSL_SDW_A1 | 2 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,
    1 * TSL_SDW_A1 | 1 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,
    1 * TSL_SDW_A1 | 0 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,
    0 * TSL_SDW_A1 | 1 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,
    0 * TSL_SDW_A1 | 0 * TSL_BSEL_A1 | 0 * TSL_DIS_A1 |
    0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0 | TSL_SF_A1 | TSL_EOS,
    };
    static const int tsl2[8] = {
    0 * TSL_SDW_A2 | 3 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_LF_A2,
    0 * TSL_SDW_A2 | 2 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 3 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 2 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 1 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 0 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 1 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 0 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2 | TSL_EOS
    };
