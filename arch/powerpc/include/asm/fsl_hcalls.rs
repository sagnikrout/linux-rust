//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fsl_hcalls.h
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
// Freescale hypervisor call interface
//
// Copyright 2008-2010 Freescale Semiconductor, Inc.
//
// Author: Timur Tabi <timur@freescale.com>
//
// This file is provided under a dual BSD/GPL license.  When using or
// redistributing this file, you may do so under either license.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

pub const FH_API_VERSION: c_int = 1;
pub const FH_ERR_GET_INFO: c_int = 1;
pub const FH_PARTITION_GET_DTPROP: c_int = 2;
pub const FH_PARTITION_SET_DTPROP: c_int = 3;
pub const FH_PARTITION_RESTART: c_int = 4;
pub const FH_PARTITION_GET_STATUS: c_int = 5;
pub const FH_PARTITION_START: c_int = 6;
pub const FH_PARTITION_STOP: c_int = 7;
pub const FH_PARTITION_MEMCPY: c_int = 8;
pub const FH_DMA_ENABLE: c_int = 9;
pub const FH_DMA_DISABLE: c_int = 10;
pub const FH_SEND_NMI: c_int = 11;
pub const FH_VMPIC_GET_MSIR: c_int = 12;
pub const FH_SYSTEM_RESET: c_int = 13;
pub const FH_GET_CORE_STATE: c_int = 14;
pub const FH_ENTER_NAP: c_int = 15;
pub const FH_EXIT_NAP: c_int = 16;
pub const FH_CLAIM_DEVICE: c_int = 17;
pub const FH_PARTITION_STOP_DMA: c_int = 18;
// vendor ID: Freescale Semiconductor

//
// We use "uintptr_t" to define a register because it's guaranteed to be a
// 32-bit integer on a 32-bit platform, and a 64-bit integer on a 64-bit
// platform.
//
// All registers are either input/output or output only.  Registers that are
// initialized before making the hypercall are input/output.  All
// input/output registers are represented with "+r".  Output-only registers
// are represented with "=r".  Do not specify any unused registers.  The
// clobber list will tell the compiler that the hypercall modifies those
// registers, which is good enough.
//
// fh_send_nmi - send NMI to virtual cpu(s).
// @vcpu_mask: send NMI to virtual cpu(s) specified by this mask.
//
// Returns 0 for success, or EINVAL for invalid vcpu_mask.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
// Arbitrary limits to avoid excessive memory allocation in hypervisor
pub const FH_DTPROP_MAX_PATHLEN: c_int = 4096;
pub const FH_DTPROP_MAX_PROPLEN: c_int = 32768;
//
// fh_partition_get_dtprop - get a property from a guest device tree.
// @handle: handle of partition whose device tree is to be accessed
// @dtpath_addr: physical address of device tree path to access
// @propname_addr: physical address of name of property
// @propvalue_addr: physical address of property value buffer
// @propvalue_len: length of buffer on entry, length of property on return
//
// Returns zero on success, non-zero on error.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
extern "C" {
    pub fn __asm__(_arg: "r7") -> register uintptr_t r7;
}
extern "C" {
    pub fn __asm__(_arg: "r8") -> register uintptr_t r8;
}
extern "C" {
    pub fn __asm__(_arg: "r9") -> register uintptr_t r9;
}
extern "C" {
    pub fn __asm__(_arg: "r10") -> register uintptr_t r10;
}

// propvalue_len = r4;
//
// Set a property in a guest device tree.
// @handle: handle of partition whose device tree is to be accessed
// @dtpath_addr: physical address of device tree path to access
// @propname_addr: physical address of name of property
// @propvalue_addr: physical address of property value
// @propvalue_len: length of property
//
// Returns zero on success, non-zero on error.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
extern "C" {
    pub fn __asm__(_arg: "r8") -> register uintptr_t r8;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r7") -> register uintptr_t r7;
}
extern "C" {
    pub fn __asm__(_arg: "r9") -> register uintptr_t r9;
}
extern "C" {
    pub fn __asm__(_arg: "r10") -> register uintptr_t r10;
}

//
// fh_partition_restart - reboot the current partition
// @partition: partition ID
//
// Returns an error code if reboot failed.  Does not return if it succeeds.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
pub const FH_PARTITION_STOPPED: c_int = 0;
pub const FH_PARTITION_RUNNING: c_int = 1;
pub const FH_PARTITION_STARTING: c_int = 2;
pub const FH_PARTITION_STOPPING: c_int = 3;
pub const FH_PARTITION_PAUSING: c_int = 4;
pub const FH_PARTITION_PAUSED: c_int = 5;
pub const FH_PARTITION_RESUMING: c_int = 6;
//
// fh_partition_get_status - gets the status of a partition
// @partition: partition ID
// @status: returned status code
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
// status = r4;
//
// fh_partition_start - boots and starts execution of the specified partition
// @partition: partition ID
// @entry_point: guest physical address to start execution
//
// The hypervisor creates a 1-to-1 virtual/physical IMA mapping, so at boot
// time, guest physical address are the same as guest virtual addresses.
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
//
// fh_partition_stop - stops another partition
// @partition: partition ID
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
//
// struct fh_sg_list: definition of the fh_partition_memcpy S/G list
// @source: guest physical address to copy from
// @target: guest physical address to copy to
// @size: number of bytes to copy
// @reserved: reserved, must be zero
//
// The scatter/gather list for fh_partition_memcpy() is an array of these
// structures.  The array must be guest physically contiguous.
//
// This structure must be aligned on 32-byte boundary, so that no single
// strucuture can span two pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fh_sg_list {
    pub /: *mut *mut *mut uint64_t source; /< guest physical address to copy from,
    pub /: *mut *mut *mut uint64_t target; /< guest physical address to copy to,
    pub /: *mut *mut *mut uint64_t size; /< number of bytes to copy,
    pub /: *mut *mut *mut uint64_t reserved; /< reserved, must be zero,
// C attribute field omitted
//
// fh_partition_memcpy - copies data from one guest to another
// @source: the ID of the partition to copy from
// @target: the ID of the partition to copy to
// @sg_list: guest physical address of an array of &fh_sg_list structures
// @count: the number of entries in @sg_list
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub __asm__("r5"): register uintptr_t r5,
    pub __asm__("r6"): register uintptr_t r6,
    pub __asm__("r7"): register uintptr_t r7,
    pub FH_HCALL_TOKEN(FH_PARTITION_MEMCPY): r11 =,
    pub source: r3 =,
    pub target: r4 =,
    pub sg_list: r5 = (uint32_t),

    pub 32: r6 = sg_list >>,

    pub 0: r6 =,

    pub count: r7 =,
    pub r3: return,
//
// fh_dma_enable - enable DMA for the specified device
// @liodn: the LIODN of the I/O device for which to enable DMA
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub FH_HCALL_TOKEN(FH_DMA_ENABLE): r11 =,
    pub liodn: r3 =,
    pub r3: return,
//
// fh_dma_disable - disable DMA for the specified device
// @liodn: the LIODN of the I/O device for which to disable DMA
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub FH_HCALL_TOKEN(FH_DMA_DISABLE): r11 =,
    pub liodn: r3 =,
    pub r3: return,
//
// fh_vmpic_get_msir - returns the MPIC-MSI register value
// @interrupt: the interrupt number
// @msir_val: returned MPIC-MSI register value
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub FH_HCALL_TOKEN(FH_VMPIC_GET_MSIR): r11 =,
    pub interrupt: r3 =,
// msir_val = r4;
    pub r3: return,
//
// fh_system_reset - reset the system
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub FH_HCALL_TOKEN(FH_SYSTEM_RESET): r11 =,
    pub r3: return,
//
// fh_err_get_info - get platform error information
// @queue id:
// 0 for guest error event queue
// 1 for global error event queue
//
// @pointer to store the platform error data:
// platform error data is returned in registers r4 - r11
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub __asm__("r5"): register uintptr_t r5,
    pub __asm__("r6"): register uintptr_t r6,
    pub __asm__("r7"): register uintptr_t r7,
    pub FH_HCALL_TOKEN(FH_ERR_GET_INFO): r11 =,
    pub queue: r3 =,
    pub bufsize: *mut r4 =,
    pub addr_hi: r5 =,
    pub addr_lo: r6 =,
    pub peek: r7 =,
// bufsize = r4;
    pub r3: return,
pub const FH_VCPU_RUN: c_int = 0;
pub const FH_VCPU_IDLE: c_int = 1;
pub const FH_VCPU_NAP: c_int = 2;
//
// fh_get_core_state - get the state of a vcpu
//
// @handle: handle of partition containing the vcpu
// @vcpu: vcpu number within the partition
// @state:the current state of the vcpu, see FH_VCPU_
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub FH_HCALL_TOKEN(FH_GET_CORE_STATE): r11 =,
    pub handle: r3 =,
    pub vcpu: r4 =,
// state = r4;
    pub r3: return,
//
// fh_enter_nap - enter nap on a vcpu
//
// Note that though the API supports entering nap on a vcpu other
// than the caller, this may not be implmented and may return EINVAL.
//
// @handle: handle of partition containing the vcpu
// @vcpu: vcpu number within the partition
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub FH_HCALL_TOKEN(FH_ENTER_NAP): r11 =,
    pub handle: r3 =,
    pub vcpu: r4 =,
    pub r3: return,
//
// fh_exit_nap - exit nap on a vcpu
// @handle: handle of partition containing the vcpu
// @vcpu: vcpu number within the partition
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub __asm__("r4"): register uintptr_t r4,
    pub FH_HCALL_TOKEN(FH_EXIT_NAP): r11 =,
    pub handle: r3 =,
    pub vcpu: r4 =,
    pub r3: return,
//
// fh_claim_device - claim a "claimable" shared device
// @handle: fsl,hv-device-handle of node to claim
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub FH_HCALL_TOKEN(FH_CLAIM_DEVICE): r11 =,
    pub handle: r3 =,
    pub r3: return,
//
// Run deferred DMA disabling on a partition's private devices
//
// This applies to devices which a partition owns either privately,
// or which are claimable and still actively owned by that partition,
// and which do not have the no-dma-disable property.
//
// @handle: partition (must be stopped) whose DMA is to be disabled
//
// Returns 0 for success, or an error code.
//
    pub __asm__("r11"): register uintptr_t r11,
    pub __asm__("r3"): register uintptr_t r3,
    pub FH_HCALL_TOKEN(FH_PARTITION_STOP_DMA): r11 =,
    pub handle: r3 =,
    pub r3: return,
