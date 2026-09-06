//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/spk_priv_keyinfo.h
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


// SPDX-License-Identifier: GPL-2.0+
// spk_priv.h
// review functions for the speakup screen review package.
// originally written by: Kirk Reiser and Andy Berdan.
//
// extensively modified by David Borowski.
//
// Copyright (C) 1998  Kirk Reiser.
// Copyright (C) 2003  David Borowski.
//

// 0 is reserved for no remap
pub const SPEAKUP_GOTO: c_uint = 0x01;
pub const SPEECH_KILL: c_uint = 0x02;
pub const SPEAKUP_QUIET: c_uint = 0x03;
pub const SPEAKUP_CUT: c_uint = 0x04;
pub const SPEAKUP_PASTE: c_uint = 0x05;
pub const SAY_FIRST_CHAR: c_uint = 0x06;
pub const SAY_LAST_CHAR: c_uint = 0x07;
pub const SAY_CHAR: c_uint = 0x08;
pub const SAY_PREV_CHAR: c_uint = 0x09;
pub const SAY_NEXT_CHAR: c_uint = 0x0a;
pub const SAY_WORD: c_uint = 0x0b;
pub const SAY_PREV_WORD: c_uint = 0x0c;
pub const SAY_NEXT_WORD: c_uint = 0x0d;
pub const SAY_LINE: c_uint = 0x0e;
pub const SAY_PREV_LINE: c_uint = 0x0f;
pub const SAY_NEXT_LINE: c_uint = 0x10;
pub const TOP_EDGE: c_uint = 0x11;
pub const BOTTOM_EDGE: c_uint = 0x12;
pub const LEFT_EDGE: c_uint = 0x13;
pub const RIGHT_EDGE: c_uint = 0x14;
pub const SPELL_PHONETIC: c_uint = 0x15;
pub const SPELL_WORD: c_uint = 0x16;
pub const SAY_SCREEN: c_uint = 0x17;
pub const SAY_POSITION: c_uint = 0x18;
pub const SAY_ATTRIBUTES: c_uint = 0x19;
pub const SPEAKUP_OFF: c_uint = 0x1a;
pub const SPEAKUP_PARKED: c_uint = 0x1b;
pub const SAY_LINE_INDENT: c_uint = 0x1c;
pub const SAY_FROM_TOP: c_uint = 0x1d;
pub const SAY_TO_BOTTOM: c_uint = 0x1e;
pub const SAY_FROM_LEFT: c_uint = 0x1f;
pub const SAY_TO_RIGHT: c_uint = 0x20;
pub const SAY_CHAR_NUM: c_uint = 0x21;
pub const EDIT_SOME: c_uint = 0x22;
pub const EDIT_MOST: c_uint = 0x23;
pub const SAY_PHONETIC_CHAR: c_uint = 0x24;
pub const EDIT_DELIM: c_uint = 0x25;
pub const EDIT_REPEAT: c_uint = 0x26;
pub const EDIT_EXNUM: c_uint = 0x27;
pub const SET_WIN: c_uint = 0x28;
pub const CLEAR_WIN: c_uint = 0x29;
pub const ENABLE_WIN: c_uint = 0x2a;
pub const SAY_WIN: c_uint = 0x2b;
pub const SPK_LOCK: c_uint = 0x2c;
pub const SPEAKUP_HELP: c_uint = 0x2d;
pub const TOGGLE_CURSORING: c_uint = 0x2e;
pub const READ_ALL_DOC: c_uint = 0x2f;
// one greater than the last func handler
pub const SPKUP_MAX_FUNC: c_uint = 0x30;
pub const SPK_KEY: c_uint = 0x80;
pub const FIRST_EDIT_BITS: c_uint = 0x22;

// increase if adding more than 0x3f functions
pub const VAR_START: c_uint = 0x40;
// keys for setting variables, must be ordered same as the enum for var_ids
// with dec being even and inc being 1 greater

