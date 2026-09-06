//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/quirks.h
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
// This file contains work-arounds for many known SD/MMC
// and SDIO hardware bugs.
//
// Copyright (c) 2011 Andrei Warkentin <andreiw@motorola.com>
// Copyright (c) 2011 Pierre Tardy <tardyp@gmail.com>
// Inspired from pci fixup code:
// Copyright (c) 1999 Martin Mares <mj@ucw.cz>
//

//
// Kingston Canvas Go! Plus microSD cards never finish SD cache flush.
// This has so far only been observed on cards from 11/2019, while new
// cards from 2023/05 do not exhibit this behavior.
//
// GIGASTONE Gaming Plus microSD cards manufactured on 02/2022 never
// clear Flush Cache bit and set Poweroff Notification Ready bit.
//
// Swissbit series S46-u cards throw I/O errors during tuning requests
// after the initial tuning request expectedly times out. This has
// only been observed on cards manufactured on 01/2019 that are using
// Bay Trail host controllers.
//
// Some SD cards reports discard support while they don't
//
pub const INAND_CMD38_ARG_EXT_CSD: c_int = 113;
pub const INAND_CMD38_ARG_ERASE: c_uint = 0x00;
pub const INAND_CMD38_ARG_TRIM: c_uint = 0x01;
pub const INAND_CMD38_ARG_SECERASE: c_uint = 0x80;
pub const INAND_CMD38_ARG_SECTRIM1: c_uint = 0x81;
pub const INAND_CMD38_ARG_SECTRIM2: c_uint = 0x88;
// CMD38 argument is passed through EXT_CSD[113]
//
// Some MMC cards experience performance degradation with CMD23
// instead of CMD12-bounded multiblock transfers. For now we'll
// black list what's bad...
// - Certain Toshiba cards.
//
// N.B. This doesn't affect SD cards.
//
// Some SD cards lockup while using CMD23 multiblock transfers.
//
// Some MMC cards need longer data read timeout than indicated in CSD.
//
// On these Samsung MoviNAND parts, performing secure erase or
// secure trim can result in unrecoverable corruption due to a
// firmware bug.
//
// On Some Kingston eMMCs, performing trim can result in
// unrecoverable data conrruption occasionally due to a firmware bug.
//
// Micron MTFC4GACAJCN-1M supports TRIM but does not appear to support
// WRITE_ZEROES offloading. It also supports caching, but the cache can
// only be flushed after a write has occurred.
//
// Kingston EMMC04G-M627 advertises TRIM but it does not seems to
// support being used to offload WRITE_ZEROES.
//
// On Some Kingston eMMCs, secure erase/trim time is independent
// of erase size, fixed at approximately 2 seconds.
//
// Certain Hynix eMMC 4.41 cards might get broken when HPI feature
// is used so disable the HPI feature for such buggy cards.
//
// Certain Micron (Numonyx) eMMC 4.5 cards might get broken when HPI
// feature is used so disable the HPI feature for such buggy cards.
//
