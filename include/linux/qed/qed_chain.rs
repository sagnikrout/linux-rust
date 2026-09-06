//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_chain.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_chain_mode {
// Each Page contains a next pointer at its end
    QED_CHAIN_MODE_NEXT_PTR,

// Chain is a single page (next ptr) is not required
    QED_CHAIN_MODE_SINGLE,

// Page pointers are located in a side list
    QED_CHAIN_MODE_PBL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_chain_use_mode {
    QED_CHAIN_USE_TO_PRODUCE,			/* Chain starts empty */
    QED_CHAIN_USE_TO_CONSUME,			/* Chain starts full */
    QED_CHAIN_USE_TO_CONSUME_PRODUCE,		/* Chain starts empty */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_chain_cnt_type {
// The chain's size/prod/cons are kept in 16-bit variables
    QED_CHAIN_CNT_TYPE_U16,

// The chain's size/prod/cons are kept in 32-bit variables
    QED_CHAIN_CNT_TYPE_U32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_next {
    pub next_phys: regpair,
    pub next_virt: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_pbl_u16 {
    pub prod_page_idx: u16,
    pub cons_page_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_pbl_u32 {
    pub prod_page_idx: u32,
    pub cons_page_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_u16 {
// Cyclic index of next element to produce/consume
    pub prod_idx: u16,
    pub cons_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_u32 {
// Cyclic index of next element to produce/consume
    pub prod_idx: u32,
    pub cons_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_tbl_entry {
    pub virt_addr: *mut c_void,
    pub dma_map: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain {
// Fastpath portion of the chain - required for commands such
// as produce / consume.
//
// Point to next element to produce/consume
    pub p_prod_elem: *mut c_void,
    pub p_cons_elem: *mut c_void,
// Fastpath portions of the PBL [if exists]
// Table for keeping the virtual and physical addresses of the
// chain pages, respectively to the physical addresses
// in the pbl table.
//
    pub pp_addr_tbl: *mut addr_tbl_entry,
    pub u16: qed_chain_pbl_u16,
    pub u32: qed_chain_pbl_u32,
    pub c: },
    pub pbl: },
    pub chain16: qed_chain_u16,
    pub chain32: qed_chain_u32,
    pub u: },
// Capacity counts only usable elements
    pub capacity: u32,
    pub page_cnt: u32,
    pub mode: qed_chain_mode,
// Elements information for fast calculations
    pub elem_per_page: u16,
    pub elem_per_page_mask: u16,
    pub elem_size: u16,
    pub next_page_mask: u16,
    pub usable_per_page: u16,
    pub elem_unusable: u8,
    pub cnt_type: qed_chain_cnt_type,
// Slowpath of the chain - required for initialization and destruction,
// but isn't involved in regular functionality.
//
    pub page_size: u32,
// Base address of a pre-allocated buffer for pbl
    pub table_virt: *mut __le64,
    pub table_phys: dma_addr_t,
    pub table_size: usize,
    pub pbl_sp: },
// Address of first page of the chain - the address is required
// for fastpath operation [consume/produce] but only for the SINGLE
// flavour which isn't considered fastpath [== SPQ].
//
    pub p_virt_addr: *mut c_void,
    pub p_phys_addr: dma_addr_t,
// Total number of elements [for entire chain]
    pub size: u32,
    pub intended_use: qed_chain_use_mode,
    pub b_external_pbl: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_chain_init_params {
    pub mode: qed_chain_mode,
    pub intended_use: qed_chain_use_mode,
    pub cnt_type: qed_chain_cnt_type,
    pub page_size: u32,
    pub num_elems: u32,
    pub elem_size: usize,
    pub ext_pbl_virt: *mut c_void,
    pub ext_pbl_phys: dma_addr_t,
}

// Accessors
//
// qed_chain_advance_page(): Advance the next element across pages for a
// linked chain.
//
// @p_chain: P_chain.
// @p_next_elem: P_next_elem.
// @idx_to_inc: Idx_to_inc.
// @page_to_inc: page_to_inc.
//
// Return: Void.
//
// p_next_elem = p_next->next_virt;
// (u16 *)idx_to_inc += p_chain->elem_unusable;
// (u32 *)idx_to_inc += p_chain->elem_unusable;
// p_next_elem = p_chain->p_virt_addr;
// (u16 *)page_to_inc = 0;
// (u32 *)page_to_inc = 0;
// p_next_elem = p_chain->pbl.pp_addr_tbl[page_index].virt_addr;

//
// qed_chain_return_produced(): A chain in which the driver "Produces"
// elements should use this API
// to indicate previous produced elements
// are now consumed.
//
// @p_chain: Chain.
//
// Return: Void.
//
// qed_chain_produce(): A chain in which the driver "Produces"
// elements should use this to get a pointer to
// the next element which can be "Produced". It's driver
// responsibility to validate that the chain has room for
// new element.
//
// @p_chain: Chain.
//
// Return: void*, a pointer to next element.
//
// qed_chain_get_capacity(): Get the maximum number of BDs in chain
//
// @p_chain: Chain.
//
// Return: number of unusable BDs.
//
// qed_chain_recycle_consumed(): Returns an element which was
// previously consumed;
// Increments producers so they could
// be written to FW.
//
// @p_chain: Chain.
//
// Return: Void.
//
// qed_chain_consume(): A Chain in which the driver utilizes data written
// by a different source (i.e., FW) should use this to
// access passed buffers.
//
// @p_chain: Chain.
//
// Return: void*, a pointer to the next buffer written.
//
// qed_chain_reset(): Resets the chain to its start state.
//
// @p_chain: pointer to a previously allocated chain.
//
// Return Void.
//
// Use (page_cnt - 1) as a reset value for the prod/cons page's
// indices, to avoid unnecessary page advancing on the first
// call to qed_chain_produce/consume. Instead, the indices
// will be advanced to page_cnt and then will be wrapped to 0.
//
// produce empty elements
// Do nothing
//
// qed_chain_get_last_elem(): Returns a pointer to the last element of the
// chain.
//
// @p_chain: Chain.
//
// Return: void*.
//
// p_virt_addr points at this stage to the last page of the chain
//
// qed_chain_set_prod(): sets the prod to the given value.
//
// @p_chain: Chain.
// @prod_idx: Prod Idx.
// @p_prod_elem: Prod elem.
//
// Return Void.
//
// Assume that number of elements in a page is power of 2
// Use "cur_prod - 1" and "prod_idx - 1" since producer index
// reaches the first element of next page before the page index
// is incremented. See qed_chain_produce().
// Index wrap around is not a problem because the difference
// between current and given producer indices is always
// positive and lower than the chain's capacity.
//
// qed_chain_pbl_zero_mem(): set chain memory to 0.
//
// @p_chain: Chain.
//
// Return: Void.
//
