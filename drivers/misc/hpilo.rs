//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/hpilo.h
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
// linux/drivers/char/hpilo.h
//
// Copyright (C) 2008 Hewlett-Packard Development Company, L.P.
// David Altobelli <david.altobelli@hp.com>
//

// iLO ASIC PCI revision id
pub const PCI_REV_ID_NECHES: c_int = 7;
// max number of open channel control blocks per device, hw limited to 32
pub const MAX_CCB: c_int = 24;
// min number of open channel control blocks per device, hw limited to 32
pub const MIN_CCB: c_int = 8;
// max number of supported devices
pub const MAX_ILO_DEV: c_int = 1;
// max number of files

// total wait time in usec
pub const MAX_WAIT_TIME: c_int = 10000;
// per spin wait time in usec
pub const WAIT_TIME: c_int = 10;
// spin counter for open/close delay

//
// Per device, used to track global memory allocations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilo_hwinfo {
// mmio registers on device
    pub mmio_vaddr: *mut char __iomem,
// doorbell registers on device
    pub db_vaddr: *mut char __iomem,
// shared memory on device used for channel control blocks
    pub ram_vaddr: *mut char __iomem,
// files corresponding to this device
    pub ccb_alloc: [*mut ccb_data; MAX_CCB],
    pub ilo_dev: *mut pci_dev,
//
// open_lock      serializes ccb_cnt during open and close
// [ irq disabled ]
// -> alloc_lock  used when adding/removing/searching ccb_alloc,
// which represents all ccbs open on the device
// --> fifo_lock  controls access to fifo queues shared with hw
//
// Locks must be taken in this order, but open_lock and alloc_lock
// are optional, they do not need to be held in order to take a
// lower level lock.
//
    pub open_lock: spinlock_t,
    pub alloc_lock: spinlock_t,
    pub fifo_lock: spinlock_t,
    pub cdev: cdev,
}

// offset from mmio_vaddr for enabling doorbell interrupts
pub const DB_IRQ: c_uint = 0xB2;
// offset from mmio_vaddr for outbound communications
pub const DB_OUT: c_uint = 0xD4;
// DB_OUT reset bit
pub const DB_RESET: c_int = 26;
//
// Channel control block. Used to manage hardware queues.
// The format must match hw's version.  The hw ccb is 128 bytes,
// but the context area shouldn't be touched by the driver.
//
pub const ILOSW_CCB_SZ: c_int = 64;
pub const ILOHW_CCB_SZ: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb {
    pub send_fifobar: *mut c_char,
    pub send_fifobar_pa: u64,
    pub ccb_u1: },
    pub send_desc: *mut c_char,
    pub send_desc_pa: u64,
    pub ccb_u2: },
    pub send_ctrl: u64,
    pub recv_fifobar: *mut c_char,
    pub recv_fifobar_pa: u64,
    pub ccb_u3: },
    pub recv_desc: *mut c_char,
    pub recv_desc_pa: u64,
    pub ccb_u4: },
    pub recv_ctrl: u64,
    pub db_base: *mut char __iomem,
    pub padding5: u64,
    pub ccb_u5: },
    pub channel: u64,
// unused context area (64 bytes)
}

// ccb queue parameters
pub const SENDQ: c_int = 1;
pub const RECVQ: c_int = 2;
pub const NR_QENTRY: c_int = 4;
pub const L2_QENTRY_SZ: c_int = 12;
// ccb ctrl bitfields
pub const CTRL_BITPOS_L2SZ: c_int = 0;
pub const CTRL_BITPOS_FIFOINDEXMASK: c_int = 4;
pub const CTRL_BITPOS_DESCLIMIT: c_int = 18;
pub const CTRL_BITPOS_A: c_int = 30;
pub const CTRL_BITPOS_G: c_int = 31;
// ccb doorbell macros
pub const L2_DB_SIZE: c_int = 14;

//
// Per fd structure used to track the ccb allocated to that dev file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb_data {
// software version of ccb, using virtual addrs
    pub driver_ccb: ccb,
// hardware version of ccb, using physical addrs
    pub ilo_ccb: ccb,
// hardware ccb is written to this shared mapped device memory
    pub mapped_ccb: *mut ccb __iomem,
// dma'able memory used for send/recv queues
    pub dma_va: *mut c_void,
    pub dma_pa: dma_addr_t,
    pub dma_size: usize,
// pointer to hardware device info
    pub ilo_hw: *mut ilo_hwinfo,
// queue for this ccb to wait for recv data
    pub ccb_waitq: wait_queue_head_t,
// usage count, to allow for shared ccb's
    pub ccb_cnt: c_int,
// open wanted exclusive access to this ccb
    pub ccb_excl: c_int,
}

//
// FIFO queue structure, shared with hw.
//
pub const ILO_START_ALIGN: c_int = 4096;
pub const ILO_CACHE_SZ: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo {
    pub /: *mut *mut u64 nrents; / user requested number of fifo entries,
    pub /: *mut *mut u64 imask; / mask to extract valid fifo index,
    pub /: *mut *mut u64 merge; / O/C bits to merge in during enqueue operation,
    pub /: *mut *mut u64 reset; / set to non-zero when the target device resets,
    pub 4)]: *mut *mut u8 pad_0[ILO_CACHE_SZ - (sizeof(u64),
    pub head: u64,
    pub (sizeof(u64))]: u8 pad_1[ILO_CACHE_SZ -,
    pub tail: u64,
    pub (sizeof(u64))]: u8 pad_2[ILO_CACHE_SZ -,
    pub fifobar: [u64; ],
}

// convert between struct fifo, and the fifobar, which is saved in the ccb

// the number of qwords to consume from the entry descriptor
pub const ENTRY_BITPOS_QWORDS: c_int = 0;
// descriptor index number (within a specified queue)
pub const ENTRY_BITPOS_DESCRIPTOR: c_int = 10;
// state bit, fifo entry consumed by consumer
pub const ENTRY_BITPOS_C: c_int = 22;
// state bit, fifo entry is occupied
pub const ENTRY_BITPOS_O: c_int = 23;
pub const ENTRY_BITS_QWORDS: c_int = 10;
pub const ENTRY_BITS_DESCRIPTOR: c_int = 12;
pub const ENTRY_BITS_C: c_int = 1;
pub const ENTRY_BITS_O: c_int = 1;

// extract various entry fields

