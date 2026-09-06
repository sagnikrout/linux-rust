//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/be2iscsi/be.h
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
// Copyright 2017 Broadcom. All Rights Reserved.
// The term "Broadcom" refers to Broadcom Limited and/or its subsidiaries.
//
// Contact Information:
// linux-drivers@broadcom.com
//

pub const FW_VER_LEN: c_int = 32;
pub const MCC_Q_LEN: c_int = 128;
pub const MCC_CQ_LEN: c_int = 256;
pub const MAX_MCC_CMD: c_int = 16;
// BladeEngine Generation numbers
pub const BE_GEN2: c_int = 2;
pub const BE_GEN3: c_int = 3;
pub const BE_GEN4: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_dma_mem {
    pub va: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_queue_info {
    pub dma_mem: be_dma_mem,
    pub len: u16,
    pub /: *mut *mut u16 entry_size; / Size of an element in the queue,
    pub id: u16,
    pub head: u16 tail,,
    pub created: bool,
    pub /: *mut *mut u16 used; / Number of valid elements in the queue,
}

// index = MODULO((*index + 1), limit);
// ISCSI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_aic_obj {
    pub jiffies: c_ulong,
    pub /: *mut *mut u32 eq_prev; / Used to calculate eqe,
    pub prev_eqd: u32,
pub const BEISCSI_EQ_DELAY_MIN: c_int = 0;
pub const BEISCSI_EQ_DELAY_DEF: c_int = 32;
pub const BEISCSI_EQ_DELAY_MAX: c_int = 128;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eq_obj {
    pub cq_count: u32,
    pub q: be_queue_info,
    pub phba: *mut beiscsi_hba,
    pub cq: *mut be_queue_info,
    pub /: *mut *mut work_mcc_work; / Work Item,
    pub iopoll: irq_poll,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_obj {
    pub q: be_queue_info,
    pub cq: be_queue_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_mcc_tag_state {
    pub tag_state: c_ulong,
pub const MCC_TAG_STATE_RUNNING: c_int = 0;
pub const MCC_TAG_STATE_TIMEOUT: c_int = 1;
pub const MCC_TAG_STATE_ASYNC: c_int = 2;
pub const MCC_TAG_STATE_IGNORE: c_int = 3;
    pub int): *mut *mut *mut void (cbfn)(struct beiscsi_hba , unsigned,
    pub tag_mem_state: be_dma_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ctrl_info {
    pub csr: *mut u8 __iomem,
    pub /: *mut *mut *mut u8 __iomem db; / Door Bell,
    pub /: *mut *mut *mut u8 __iomem pcicfg; / PCI config space,
    pub pdev: *mut pci_dev,
// Mbox used for cmd request/response
    pub /: *mut *mut mutex mbox_lock; / For serializing mbox cmds to BE card,
    pub mbox_mem: be_dma_mem,
// Mbox mem is adjusted to align to 16 bytes. The allocated addr
// is stored for freeing purpose
    pub mbox_mem_alloced: be_dma_mem,
// MCC Rings
    pub mcc_obj: be_mcc_obj,
    pub /: *mut *mut spinlock_t mcc_lock; / For serializing mcc cmds to BE card,
    pub 1]: wait_queue_head_t mcc_wait[MAX_MCC_CMD +,
    pub mcc_tag: [c_uint; MAX_MCC_CMD],
    pub 1]: unsigned int mcc_tag_status[MAX_MCC_CMD +,
    pub mcc_alloc_index: c_ushort,
    pub mcc_free_index: c_ushort,
    pub mcc_tag_available: c_uint,
    pub 1]: beiscsi_mcc_tag_state ptag_state[MAX_MCC_CMD +,
}

// WRB index mask for MCC_Q_LEN queue entries

// TAG is from 1...MAX_MCC_CMD, MASK includes MAX_MCC_CMD

pub const PAGE_SHIFT_4K: c_int = 12;

// Returns number of pages spanned by the data starting at the given addr

// Returns bit offset within a DWORD of a bitfield

// Returns the bit mask of the field that is NOT shifted into location.
// dw &= ~(mask << offset);
// dw |= (mask & value) << offset;

// dw = cpu_to_le32(*dw);

