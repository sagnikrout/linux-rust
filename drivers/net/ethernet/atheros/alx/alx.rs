//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/alx/alx.h
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
// Copyright (c) 2013 Johannes Berg <johannes@sipsolutions.net>
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
// Copyright (c) 2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_buffer {
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_rx_queue {
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub np: *mut alx_napi,
    pub rrd: *mut alx_rrd,
    pub rrd_dma: dma_addr_t,
    pub rfd: *mut alx_rfd,
    pub rfd_dma: dma_addr_t,
    pub bufs: *mut alx_buffer,
    pub count: u16,
    pub read_idx: u16 write_idx,,
    pub rrd_read_idx: u16,
    pub queue_idx: u16,
}

pub const ALX_RX_ALLOC_THRESH: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_tx_queue {
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub tpd: *mut alx_txd,
    pub tpd_dma: dma_addr_t,
    pub bufs: *mut alx_buffer,
    pub count: u16,
    pub read_idx: u16 write_idx,,
    pub queue_idx: u16,
    pub c_reg: u16 p_reg,,
}

pub const ALX_DEFAULT_TX_WORK: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alx_device_quirks {
    ALX_DEV_QUIRK_MSI_INTX_DISABLE_BUG = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_napi {
    pub napi: napi_struct,
    pub alx: *mut alx_priv,
    pub rxq: *mut alx_rx_queue,
    pub txq: *mut alx_tx_queue,
    pub vec_idx: c_int,
    pub vec_mask: u32,
    pub 8]: char irq_lbl[IFNAMSIZ +,
}

pub const ALX_MAX_NAPIS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_priv {
    pub dev: *mut net_device,
    pub hw: alx_hw,
// msi-x vectors
    pub num_vec: c_int,
// all descriptor memory
    pub dma: dma_addr_t,
    pub virt: *mut c_void,
    pub size: c_uint,
    pub descmem: },
    pub qnapi: [*mut alx_napi; ALX_MAX_NAPIS],
    pub num_txq: c_int,
    pub num_rxq: c_int,
    pub num_napi: c_int,
// protect int_mask updates
    pub irq_lock: spinlock_t,
    pub int_mask: u32,
    pub tx_ringsz: c_uint,
    pub rx_ringsz: c_uint,
    pub rxbuf_size: c_uint,
    pub link_check_wk: work_struct,
    pub reset_wk: work_struct,
    pub msg_enable: u16,
// protects hw.stats
    pub stats_lock: spinlock_t,
    pub mtx: mutex,
}
