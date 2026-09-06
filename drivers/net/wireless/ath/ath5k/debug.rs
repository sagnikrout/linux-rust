//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/debug.h
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


//
// Copyright (c) 2007 Bruno Randolf <bruno@thinktube.com>
//
// This file is free software: you may copy, redistribute and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 2 of the License, or (at your
// option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
// Copyright (c) 2002-2005 Sam Leffler, Errno Consulting
// Copyright (c) 2004-2005 Atheros Communications, Inc.
// Copyright (c) 2006 Devicescape Software, Inc.
// Copyright (c) 2007 Jiri Slaby <jirislaby@gmail.com>
// Copyright (c) 2007 Luis R. Rodriguez <mcgrof@winlab.rutgers.edu>
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// similar to the "NO WARRANTY" disclaimer below ("Disclaimer") and any
// redistribution must be conditioned upon including a substantially
// similar Disclaimer requirement for further binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF NONINFRINGEMENT, MERCHANTIBILITY
// AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY,
// OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER
// IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
// THE POSSIBILITY OF SUCH DAMAGES.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_dbg_info {
    pub /: *mut *mut unsigned int level; / debug level,
}

//
// enum ath5k_debug_level - ath5k debug level
//
// @ATH5K_DEBUG_RESET: reset processing
// @ATH5K_DEBUG_INTR: interrupt handling
// @ATH5K_DEBUG_MODE: mode init/setup
// @ATH5K_DEBUG_XMIT: basic xmit operation
// @ATH5K_DEBUG_BEACON: beacon handling
// @ATH5K_DEBUG_CALIBRATE: periodic calibration
// @ATH5K_DEBUG_TXPOWER: transmit power setting
// @ATH5K_DEBUG_LED: led management
// @ATH5K_DEBUG_DUMPBANDS: dump bands
// @ATH5K_DEBUG_DMA: debug dma start/stop
// @ATH5K_DEBUG_ANI: debug Adaptive Noise Immunity
// @ATH5K_DEBUG_DESC: descriptor setup
// @ATH5K_DEBUG_ANY: show at any debug level
//
// The debug level is used to control the amount and type of debugging output
// we want to see. The debug level is given in calls to ATH5K_DBG to specify
// where the message should appear, and the user can control the debugging
// messages he wants to see, either by the module parameter 'debug' on module
// load, or dynamically by using debugfs 'ath5k/phyX/debug'. these levels can
// be combined together by bitwise OR.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_debug_level {
    ATH5K_DEBUG_RESET	= 0x00000001,
    ATH5K_DEBUG_INTR	= 0x00000002,
    ATH5K_DEBUG_MODE	= 0x00000004,
    ATH5K_DEBUG_XMIT	= 0x00000008,
    ATH5K_DEBUG_BEACON	= 0x00000010,
    ATH5K_DEBUG_CALIBRATE	= 0x00000020,
    ATH5K_DEBUG_TXPOWER	= 0x00000040,
    ATH5K_DEBUG_LED		= 0x00000080,
    ATH5K_DEBUG_DUMPBANDS	= 0x00000400,
    ATH5K_DEBUG_DMA		= 0x00000800,
    ATH5K_DEBUG_ANI		= 0x00002000,
    ATH5K_DEBUG_DESC	= 0x00004000,
    ATH5K_DEBUG_ANY		= 0xffffffff
}

