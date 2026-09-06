//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/nitro_enclaves/ne_misc_dev.h
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
// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//

//
// struct ne_mem_region - Entry in the enclave user space memory regions list.
// @mem_region_list_entry:	Entry in the list of enclave memory regions.
// @memory_size:		Size of the user space memory region.
// @nr_pages:			Number of pages that make up the memory region.
// @pages:			Pages that make up the user space memory region.
// @userspace_addr:		User space address of the memory region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_mem_region {
    pub mem_region_list_entry: list_head,
    pub memory_size: u64,
    pub nr_pages: c_ulong,
    pub pages: *mut page,
    pub userspace_addr: u64,
}

//
// struct ne_enclave - Per-enclave data used for enclave lifetime management.
// @enclave_info_mutex :	Mutex for accessing this internal state.
// @enclave_list_entry :	Entry in the list of created enclaves.
// @eventq:			Wait queue used for out-of-band event notifications
// triggered from the PCI device event handler to
// the enclave process via the poll function.
// @has_event:			Variable used to determine if the out-of-band event
// was triggered.
// @max_mem_regions:		The maximum number of memory regions that can be
// handled by the hypervisor.
// @mem_regions_list:		Enclave user space memory regions list.
// @mem_size:			Enclave memory size.
// @mm :			Enclave process abstraction mm data struct.
// @nr_mem_regions:		Number of memory regions associated with the enclave.
// @nr_parent_vm_cores :	The size of the threads per core array. The
// total number of CPU cores available on the
// parent / primary VM.
// @nr_threads_per_core:	The number of threads that a full CPU core has.
// @nr_vcpus:			Number of vcpus associated with the enclave.
// @numa_node:			NUMA node of the enclave memory and CPUs.
// @slot_uid:			Slot unique id mapped to the enclave.
// @state:			Enclave state, updated during enclave lifetime.
// @threads_per_core:		Enclave full CPU cores array, indexed by core id,
// consisting of cpumasks with all their threads.
// Full CPU cores are taken from the NE CPU pool
// and are available to the enclave.
// @vcpu_ids:			Cpumask of the vCPUs that are set for the enclave.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_enclave {
    pub enclave_info_mutex: mutex,
    pub enclave_list_entry: list_head,
    pub eventq: wait_queue_head_t,
    pub has_event: bool,
    pub max_mem_regions: u64,
    pub mem_regions_list: list_head,
    pub mem_size: u64,
    pub mm: *mut mm_struct,
    pub nr_mem_regions: c_uint,
    pub nr_parent_vm_cores: c_uint,
    pub nr_threads_per_core: c_uint,
    pub nr_vcpus: c_uint,
    pub numa_node: c_int,
    pub slot_uid: u64,
    pub state: u16,
    pub threads_per_core: *mut cpumask_var_t,
    pub vcpu_ids: cpumask_var_t,
}

//
// enum ne_state - States available for an enclave.
// @NE_STATE_INIT:	The enclave has not been started yet.
// @NE_STATE_RUNNING:	The enclave was started and is running as expected.
// @NE_STATE_STOPPED:	The enclave exited without userspace interaction.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ne_state {
    NE_STATE_INIT		= 0,
    NE_STATE_RUNNING	= 2,
    NE_STATE_STOPPED	= U16_MAX,
}

//
// struct ne_devs - Data structure to keep refs to the NE misc and PCI devices.
// @ne_misc_dev:	Nitro Enclaves misc device.
// @ne_pci_dev :	Nitro Enclaves PCI device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_devs {
    pub ne_misc_dev: *mut miscdevice,
    pub ne_pci_dev: *mut ne_pci_dev,
}

// Nitro Enclaves (NE) data structure for keeping refs to the NE misc and PCI devices.
