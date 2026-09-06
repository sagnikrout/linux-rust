//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-p2pdma.h
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
// PCI Peer 2 Peer DMA support.
//
// Copyright (c) 2016-2018, Logan Gunthorpe
// Copyright (c) 2016-2017, Microsemi Corporation
// Copyright (c) 2017, Christoph Hellwig
// Copyright (c) 2018, Eideticom Inc.
//

//
// struct p2pdma_provider
//
// A p2pdma provider is a range of MMIO address space available to the CPU.
// @owner: Device to which this provider belongs.
// @bus_offset: Bus offset for p2p communication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p2pdma_provider {
    pub owner: *mut device,
    pub bus_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_p2pdma_map_type {
//
// PCI_P2PDMA_MAP_UNKNOWN: Used internally as an initial state before
// the mapping type has been calculated. Exported routines for the API
// will never return this value.
//
    PCI_P2PDMA_MAP_UNKNOWN = 0,

//
// Not a PCI P2PDMA transfer.
//
    PCI_P2PDMA_MAP_NONE,

//
// PCI_P2PDMA_MAP_NOT_SUPPORTED: Indicates the transaction will
// traverse the host bridge and the host bridge is not in the
// allowlist. DMA Mapping routines should return an error when
// this is returned.
//
    PCI_P2PDMA_MAP_NOT_SUPPORTED,

//
// PCI_P2PDMA_MAP_BUS_ADDR: Indicates that two devices can talk to
// each other directly through a PCI switch and the transaction will
// not traverse the host bridge. Such a mapping should program
// the DMA engine with PCI bus addresses.
//
    PCI_P2PDMA_MAP_BUS_ADDR,

//
// PCI_P2PDMA_MAP_THRU_HOST_BRIDGE: Indicates two devices can talk
// to each other, but the transaction traverses a host bridge on the
// allowlist. In this case, a normal mapping either with CPU physical
// addresses (in the case of dma-direct) or IOVA addresses (in the
// case of IOMMUs) should be used to program the DMA engine.
//
    PCI_P2PDMA_MAP_THRU_HOST_BRIDGE,
}

extern "C" {
    pub fn pcim_p2pdma_init(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_free_p2pmem(pdev: *mut pci_dev, addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn pci_p2pmem_virt_to_bus(pdev: *mut pci_dev, addr: *mut c_void) -> pci_bus_addr_t;
}
extern "C" {
    pub fn pci_p2pmem_free_sgl(pdev: *mut pci_dev, sgl: *mut scatterlist);
}
extern "C" {
    pub fn pci_p2pmem_publish(pdev: *mut pci_dev, publish: bool);
}

// use_p2pdma = false;
extern "C" {
    pub fn sprintf(_arg: page, _arg: "none\n") -> return;
}

extern "C" {
    pub fn pci_p2pdma_distance_many(_arg: provider, _arg: &client, _arg: 1, _arg: verbose) -> return;
}
extern "C" {
    pub fn pci_p2pmem_find_many(_arg: &client, _arg: 1) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_p2pdma_map_state {
    pub mem: *mut p2pdma_provider,
    pub map: pci_p2pdma_map_type,
}

// helper for pci_p2pdma_state(), do not use directly
//
// pci_p2pdma_state - check the P2P transfer state of a page
// @state:	P2P state structure
// @dev:	device to transfer to/from
// @page:	page to map
//
// Check if @page is a PCI P2PDMA page, and if yes of what kind.  Returns the
// map type, and updates @state with all information needed for a P2P transfer.
//
// pci_p2pdma_bus_addr_map - Translate a physical address to a bus address
// for a PCI_P2PDMA_MAP_BUS_ADDR transfer.
// @provider:	P2P provider structure
// @paddr:	physical address to map
//
// Map a physically contiguous PCI_P2PDMA_MAP_BUS_ADDR transfer.
//
