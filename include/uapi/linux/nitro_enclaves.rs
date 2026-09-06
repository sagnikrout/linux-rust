//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nitro_enclaves.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright 2020-2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//

//
// DOC: Nitro Enclaves (NE) Kernel Driver Interface
//
// NE_CREATE_VM - The command is used to create a slot that is associated with
// an enclave VM.
// The generated unique slot id is an output parameter.
// The ioctl can be invoked on the /dev/nitro_enclaves fd, before
// setting any resources, such as memory and vCPUs, for an
// enclave. Memory and vCPUs are set for the slot mapped to an enclave.
// A NE CPU pool has to be set before calling this function. The
// pool can be set after the NE driver load, using
// /sys/module/nitro_enclaves/parameters/ne_cpus.
// Its format is the detailed in the cpu-lists section:
// https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html
// CPU 0 and its siblings have to remain available for the
// primary / parent VM, so they cannot be set for enclaves. Full
// CPU core(s), from the same NUMA node, need(s) to be included
// in the CPU pool.
//
// Context: Process context.
// Return:
// * Enclave file descriptor		- Enclave file descriptor used with
// ioctl calls to set vCPUs and memory
// regions, then start the enclave.
// *  -1				- There was a failure in the ioctl logic.
// On failure, errno is set to:
// * EFAULT				- copy_to_user() failure.
// * ENOMEM				- Memory allocation failure for internal
// bookkeeping variables.
// * NE_ERR_NO_CPUS_AVAIL_IN_POOL	- No NE CPU pool set / no CPUs available
// in the pool.
// * Error codes from get_unused_fd_flags() and anon_inode_getfile().
// * Error codes from the NE PCI device request.
//

//
// NE_ADD_VCPU - The command is used to set a vCPU for an enclave. The vCPU can
// be auto-chosen from the NE CPU pool or it can be set by the
// caller, with the note that it needs to be available in the NE
// CPU pool. Full CPU core(s), from the same NUMA node, need(s) to
// be associated with an enclave.
// The vCPU id is an input / output parameter. If its value is 0,
// then a CPU is chosen from the enclave CPU pool and returned via
// this parameter.
// The ioctl can be invoked on the enclave fd, before an enclave
// is started.
//
// Context: Process context.
// Return:
// * 0					- Logic successfully completed.
// *  -1				- There was a failure in the ioctl logic.
// On failure, errno is set to:
// * EFAULT				- copy_from_user() / copy_to_user() failure.
// * ENOMEM				- Memory allocation failure for internal
// bookkeeping variables.
// * EIO				- Current task mm is not the same as the one
// that created the enclave.
// * NE_ERR_NO_CPUS_AVAIL_IN_POOL	- No CPUs available in the NE CPU pool.
// * NE_ERR_VCPU_ALREADY_USED		- The provided vCPU is already used.
// * NE_ERR_VCPU_NOT_IN_CPU_POOL	- The provided vCPU is not available in the
// NE CPU pool.
// * NE_ERR_VCPU_INVALID_CPU_CORE	- The core id of the provided vCPU is invalid
// or out of range.
// * NE_ERR_NOT_IN_INIT_STATE		- The enclave is not in init state
// (init = before being started).
// * NE_ERR_INVALID_VCPU		- The provided vCPU is not in the available
// CPUs range.
// * Error codes from the NE PCI device request.
//

//
// NE_GET_IMAGE_LOAD_INFO - The command is used to get information needed for
// in-memory enclave image loading e.g. offset in
// enclave memory to start placing the enclave image.
// The image load info is an input / output parameter.
// It includes info provided by the caller - flags -
// and returns the offset in enclave memory where to
// start placing the enclave image.
// The ioctl can be invoked on the enclave fd, before
// an enclave is started.
//
// Context: Process context.
// Return:
// * 0				- Logic successfully completed.
// *  -1			- There was a failure in the ioctl logic.
// On failure, errno is set to:
// * EFAULT			- copy_from_user() / copy_to_user() failure.
// * NE_ERR_NOT_IN_INIT_STATE	- The enclave is not in init state (init =
// before being started).
// * NE_ERR_INVALID_FLAG_VALUE	- The value of the provided flag is invalid.
//

//
// NE_SET_USER_MEMORY_REGION - The command is used to set a memory region for an
// enclave, given the allocated memory from the
// userspace. Enclave memory needs to be from the
// same NUMA node as the enclave CPUs.
// The user memory region is an input parameter. It
// includes info provided by the caller - flags,
// memory size and userspace address.
// The ioctl can be invoked on the enclave fd,
// before an enclave is started.
//
// Context: Process context.
// Return:
// * 0					- Logic successfully completed.
// *  -1				- There was a failure in the ioctl logic.
// On failure, errno is set to:
// * EFAULT				- copy_from_user() failure.
// * EINVAL				- Invalid physical memory region(s) e.g.
// unaligned address.
// * EIO				- Current task mm is not the same as
// the one that created the enclave.
// * ENOMEM				- Memory allocation failure for internal
// bookkeeping variables.
// * NE_ERR_NOT_IN_INIT_STATE		- The enclave is not in init state
// (init = before being started).
// * NE_ERR_INVALID_MEM_REGION_SIZE	- The memory size of the region is not
// multiple of 2 MiB.
// * NE_ERR_INVALID_MEM_REGION_ADDR	- Invalid user space address given.
// * NE_ERR_UNALIGNED_MEM_REGION_ADDR	- Unaligned user space address given.
// * NE_ERR_MEM_REGION_ALREADY_USED	- The memory region is already used.
// * NE_ERR_MEM_NOT_HUGE_PAGE		- The memory region is not backed by
// huge pages.
// * NE_ERR_MEM_DIFFERENT_NUMA_NODE	- The memory region is not from the same
// NUMA node as the CPUs.
// * NE_ERR_MEM_MAX_REGIONS		- The number of memory regions set for
// the enclave reached maximum.
// * NE_ERR_INVALID_PAGE_SIZE		- The memory region is not backed by
// pages multiple of 2 MiB.
// * NE_ERR_INVALID_FLAG_VALUE		- The value of the provided flag is invalid.
// * Error codes from get_user_pages().
// * Error codes from the NE PCI device request.
//

//
// NE_START_ENCLAVE - The command is used to trigger enclave start after the
// enclave resources, such as memory and CPU, have been set.
// The enclave start info is an input / output parameter. It
// includes info provided by the caller - enclave cid and
// flags - and returns the cid (if input cid is 0).
// The ioctl can be invoked on the enclave fd, after an
// enclave slot is created and resources, such as memory and
// vCPUs are set for an enclave.
//
// Context: Process context.
// Return:
// * 0					- Logic successfully completed.
// *  -1				- There was a failure in the ioctl logic.
// On failure, errno is set to:
// * EFAULT				- copy_from_user() / copy_to_user() failure.
// * NE_ERR_NOT_IN_INIT_STATE		- The enclave is not in init state
// (init = before being started).
// * NE_ERR_NO_MEM_REGIONS_ADDED	- No memory regions are set.
// * NE_ERR_NO_VCPUS_ADDED		- No vCPUs are set.
// *  NE_ERR_FULL_CORES_NOT_USED	- Full core(s) not set for the enclave.
// * NE_ERR_ENCLAVE_MEM_MIN_SIZE	- Enclave memory is less than minimum
// memory size (64 MiB).
// * NE_ERR_INVALID_FLAG_VALUE		- The value of the provided flag is invalid.
// *  NE_ERR_INVALID_ENCLAVE_CID	- The provided enclave CID is invalid.
// * Error codes from the NE PCI device request.
//

//
// DOC: NE specific error codes
//
// NE_ERR_VCPU_ALREADY_USED - The provided vCPU is already used.
//

//
// NE_ERR_VCPU_NOT_IN_CPU_POOL - The provided vCPU is not available in the
// NE CPU pool.
//

//
// NE_ERR_VCPU_INVALID_CPU_CORE - The core id of the provided vCPU is invalid
// or out of range of the NE CPU pool.
//

//
// NE_ERR_INVALID_MEM_REGION_SIZE - The user space memory region size is not
// multiple of 2 MiB.
//

//
// NE_ERR_INVALID_MEM_REGION_ADDR - The user space memory region address range
// is invalid.
//

//
// NE_ERR_UNALIGNED_MEM_REGION_ADDR - The user space memory region address is
// not aligned.
//

//
// NE_ERR_MEM_REGION_ALREADY_USED - The user space memory region is already used.
//

//
// NE_ERR_MEM_NOT_HUGE_PAGE - The user space memory region is not backed by
// contiguous physical huge page(s).
//

//
// NE_ERR_MEM_DIFFERENT_NUMA_NODE - The user space memory region is backed by
// pages from different NUMA nodes than the CPUs.
//

//
// NE_ERR_MEM_MAX_REGIONS - The supported max memory regions per enclaves has
// been reached.
//

//
// NE_ERR_NO_MEM_REGIONS_ADDED - The command to start an enclave is triggered
// and no memory regions are added.
//

//
// NE_ERR_NO_VCPUS_ADDED - The command to start an enclave is triggered and no
// vCPUs are added.
//

//
// NE_ERR_ENCLAVE_MEM_MIN_SIZE - The enclave memory size is lower than the
// minimum supported.
//

//
// NE_ERR_FULL_CORES_NOT_USED - The command to start an enclave is triggered and
// full CPU cores are not set.
//

//
// NE_ERR_NOT_IN_INIT_STATE - The enclave is not in init state when setting
// resources or triggering start.
//

//
// NE_ERR_INVALID_VCPU - The provided vCPU is out of range of the available CPUs.
//

//
// NE_ERR_NO_CPUS_AVAIL_IN_POOL - The command to create an enclave is triggered
// and no CPUs are available in the pool.
//

//
// NE_ERR_INVALID_PAGE_SIZE - The user space memory region is not backed by pages
// multiple of 2 MiB.
//

//
// NE_ERR_INVALID_FLAG_VALUE - The provided flag value is invalid.
//

//
// NE_ERR_INVALID_ENCLAVE_CID - The provided enclave CID is invalid, either
// being a well-known value or the CID of the
// parent / primary VM.
//

//
// DOC: Image load info flags
//
// NE_EIF_IMAGE - Enclave Image Format (EIF)
//

//
// struct ne_image_load_info - Info necessary for in-memory enclave image
// loading (in / out).
// @flags:		Flags to determine the enclave image type
// (e.g. Enclave Image Format - EIF) (in).
// @memory_offset:	Offset in enclave memory where to start placing the
// enclave image (out).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_image_load_info {
    pub flags: __u64,
    pub memory_offset: __u64,
}

//
// DOC: User memory region flags
//
// NE_DEFAULT_MEMORY_REGION - Memory region for enclave general usage.
//

//
// struct ne_user_memory_region - Memory region to be set for an enclave (in).
// @flags:		Flags to determine the usage for the memory region (in).
// @memory_size:	The size, in bytes, of the memory region to be set for
// an enclave (in).
// @userspace_addr:	The start address of the userspace allocated memory of
// the memory region to set for an enclave (in).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_user_memory_region {
    pub flags: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
}

//
// DOC: Enclave start info flags
//
// NE_ENCLAVE_PRODUCTION_MODE - Start enclave in production mode.
//

//
// NE_ENCLAVE_DEBUG_MODE - Start enclave in debug mode.
//

//
// struct ne_enclave_start_info - Setup info necessary for enclave start (in / out).
// @flags:		Flags for the enclave to start with (e.g. debug mode) (in).
// @enclave_cid:	Context ID (CID) for the enclave vsock device. If 0 as
// input, the CID is autogenerated by the hypervisor and
// returned back as output by the driver (in / out).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_enclave_start_info {
    pub flags: __u64,
    pub enclave_cid: __u64,
}
