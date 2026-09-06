//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/xen-mca.h
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


// SPDX-License-Identifier: MIT
//
// arch-x86/mca.h
// Guest OS machine check interface to x86 Xen.
//
// Contributed by Advanced Micro Devices, Inc.
// Author: Christoph Egger <Christoph.Egger@amd.com>
//
// Updated by Intel Corporation
// Author: Liu, Jinsong <jinsong.liu@intel.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Hypercall

pub const XEN_MCA_INTERFACE_VERSION: c_uint = 0x01ecc003;
// IN: Dom0 calls hypercall to retrieve nonurgent error log entry
pub const XEN_MC_NONURGENT: c_uint = 0x1;
// IN: Dom0 calls hypercall to retrieve urgent error log entry
pub const XEN_MC_URGENT: c_uint = 0x2;
// IN: Dom0 acknowledges previosly-fetched error log entry
pub const XEN_MC_ACK: c_uint = 0x4;
// OUT: All is ok
pub const XEN_MC_OK: c_uint = 0x0;
// OUT: Domain could not fetch data.
pub const XEN_MC_FETCHFAILED: c_uint = 0x1;
// OUT: There was no machine check data to fetch.
pub const XEN_MC_NODATA: c_uint = 0x2;
// vIRQ injected to Dom0

//
// mc_info entry types
// mca machine check info are recorded in mc_info entries.
// when fetch mca info, it can use MC_TYPE_... to distinguish
// different mca info.
//
pub const MC_TYPE_GLOBAL: c_int = 0;
pub const MC_TYPE_BANK: c_int = 1;
pub const MC_TYPE_EXTENDED: c_int = 2;
pub const MC_TYPE_RECOVERY: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_common {
    pub /: *mut *mut uint16_t type; / structure type,
    pub /: *mut *mut uint16_t size; / size of this struct in bytes,
}

// contains x86 global mc information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_global {
    pub common: mcinfo_common,
    pub /: *mut *mut uint16_t mc_domid; / running domain at the time in error,
    pub /: *mut *mut uint16_t mc_vcpuid; / virtual cpu scheduled for mc_domid,
    pub /: *mut *mut uint32_t mc_socketid; / physical socket of the physical core,
    pub /: *mut *mut uint16_t mc_coreid; / physical impacted core,
    pub /: *mut *mut uint16_t mc_core_threadid; / core thread of physical core,
    pub mc_apicid: u32,
    pub mc_flags: u32,
    pub /: *mut *mut uint64_t mc_gstatus; / global status,
}

// contains x86 bank mc information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_bank {
    pub common: mcinfo_common,
    pub /: *mut *mut uint16_t mc_bank; / bank nr,
    pub /: *mut *mut uint16_t mc_domid; / domain referenced by mc_addr if valid,
    pub /: *mut *mut uint64_t mc_status; / bank status,
    pub /: *mut *mut uint64_t mc_addr; / bank address,
    pub mc_misc: u64,
    pub mc_ctrl2: u64,
    pub mc_tsc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_msr {
    pub /: *mut *mut uint64_t reg; / MSR,
    pub /: *mut *mut uint64_t value; / MSR value,
}

// contains mc information from other or additional mc MSRs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_extended {
    pub common: mcinfo_common,
    pub /: *mut *mut uint32_t mc_msrs; / Number of msr with valid values.,
//
// Currently Intel extended MSR (32/64) include all gp registers
// and E(R)FLAGS, E(R)IP, E(R)MISC, up to 11/19 of them might be
// useful at present. So expand this array to 16/32 to leave room.
//
    pub 4]: *mut *mut *mut mcinfo_msr mc_msr[sizeof(void ),
}

// Recovery Action flags. Giving recovery result information to DOM0
// Xen takes successful recovery action, the error is recovered

// No action is performed by XEN

// It's possible DOM0 might take action ownership in some case

//
// Different Recovery Action types, if the action is performed successfully,
// REC_ACTION_RECOVERED flag will be returned.
//
// Page Offline Action

// CPU offline Action

// L3 cache disable Action

//
// Below interface used between XEN/DOM0 for passing XEN's recovery action
// information to DOM0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_offline_action {
// Params for passing the offlined page number to DOM0
    pub mfn: u64,
    pub status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_offline_action {
// Params for passing the identity of the offlined CPU to DOM0
    pub mc_socketid: u32,
    pub mc_coreid: u16,
    pub mc_core_threadid: u16,
}

pub const MAX_UNION_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_recovery {
    pub common: mcinfo_common,
    pub /: *mut *mut uint16_t mc_bank; / bank nr,
    pub action_flags: u8,
    pub action_types: u8,
    pub page_retire: page_offline_action,
    pub cpu_offline: cpu_offline_action,
    pub pad: [u8; MAX_UNION_SIZE],
    pub action_info: },
}

pub const MCINFO_MAXSIZE: c_int = 768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_info {
// Number of mcinfo_* entries in mi_data
    pub mi_nentries: u32,
    pub flags: u32,
    pub 8]: uint64_t mi_data[(MCINFO_MAXSIZE - 1) /,
}

pub const __MC_MSR_ARRAYSIZE: c_int = 8;
pub const __MC_NMSRS: c_int = 1;
pub const MC_NCAPS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_logical_cpu {
    pub mc_cpunr: u32,
    pub mc_chipid: u32,
    pub mc_coreid: u16,
    pub mc_threadid: u16,
    pub mc_apicid: u32,
    pub mc_clusterid: u32,
    pub mc_ncores: u32,
    pub mc_ncores_active: u32,
    pub mc_nthreads: u32,
    pub mc_cpuid_level: u32,
    pub mc_family: u32,
    pub mc_vendor: u32,
    pub mc_model: u32,
    pub mc_step: u32,
    pub mc_vendorid: [c_char; 16],
    pub mc_brandid: [c_char; 64],
    pub mc_cpu_caps: [u32; MC_NCAPS],
    pub mc_cache_size: u32,
    pub mc_cache_alignment: u32,
    pub mc_nmsrvals: u32,
    pub mc_msrvalues: [mcinfo_msr; __MC_MSR_ARRAYSIZE],
}

//
// Prototype:
// uint32_t x86_mcinfo_nentries(struct mc_info *mi);
//

//
// Prototype:
// struct mcinfo_common *x86_mcinfo_first(struct mc_info *mi);
//

//
// Prototype:
// struct mcinfo_common *x86_mcinfo_next(struct mcinfo_common *mic);
//

//
// Prototype:
// void x86_mcinfo_lookup(void *ret, struct mc_info *mi, uint16_t type);
//
// ret = found ? mic : NULL;
//
// Fetch machine check data from hypervisor.
//
pub const XEN_MC_fetch: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc_fetch {
//
// IN: XEN_MC_NONURGENT, XEN_MC_URGENT,
// XEN_MC_ACK if ack'king an earlier fetch
// OUT: XEN_MC_OK, XEN_MC_FETCHAILED, XEN_MC_NODATA
//
    pub flags: u32,
    pub _pad0: u32,
// OUT: id for ack, IN: id we are ack'ing
    pub fetch_id: u64,
// OUT variables.
    pub data: GUEST_HANDLE(mc_info),
}

//
// This tells the hypervisor to notify a DomU about the machine check error
//
pub const XEN_MC_notifydomain: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc_notifydomain {
// IN variables
    pub /: *mut *mut uint16_t mc_domid; / The unprivileged domain to notify,
    pub /: *mut *mut uint16_t mc_vcpuid; / The vcpu in mc_domid to notify,
// IN/OUT variables
    pub flags: u32,
}

pub const XEN_MC_physcpuinfo: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc_physcpuinfo {
// IN/OUT
    pub ncpus: u32,
    pub _pad0: u32,
// OUT
    pub info: GUEST_HANDLE(mcinfo_logical_cpu),
}

pub const XEN_MC_msrinject: c_int = 4;
pub const MC_MSRINJ_MAXMSRS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc_msrinject {
// IN
    pub /: *mut *mut uint32_t mcinj_cpunr; / target processor id,
    pub /: *mut *mut *mut uint32_t mcinj_flags; / see MC_MSRINJ_F_ below,
    pub /: *mut *mut uint32_t mcinj_count; / 0 .. count-1 in array are valid,
    pub _pad0: u32,
    pub mcinj_msr: [mcinfo_msr; MC_MSRINJ_MAXMSRS],
}

// Flags for mcinj_flags above; bits 16-31 are reserved
pub const MC_MSRINJ_F_INTERPOSE: c_uint = 0x1;
pub const XEN_MC_mceinject: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc_mceinject {
    pub /: *mut *mut unsigned int mceinj_cpunr; / target processor id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mc {
    pub cmd: u32,
    pub /: *mut *mut uint32_t interface_version; / XEN_MCA_INTERFACE_VERSION,
    pub mc_fetch: xen_mc_fetch,
    pub mc_notifydomain: xen_mc_notifydomain,
    pub mc_physcpuinfo: xen_mc_physcpuinfo,
    pub mc_msrinject: xen_mc_msrinject,
    pub mc_mceinject: xen_mc_mceinject,
    pub u: },
}

//
// Fields are zero when not available. Also, this struct is shared with
// userspace mcelog and thus must keep existing fields at current offsets.
// Only add new fields to the end of the structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mce {
    pub status: __u64,
    pub misc: __u64,
    pub addr: __u64,
    pub mcgstatus: __u64,
    pub ip: __u64,
    pub /: *mut *mut __u64 tsc; / cpu time stamp counter,
    pub /: *mut *mut __u64 time; / wall time_t when error was detected,
    pub /: *mut *mut __u8 cpuvendor; / cpu vendor as encoded in system.h,
    pub /: *mut *mut __u8 inject_flags; / software inject flags,
    pub pad: __u16,
    pub /: *mut *mut __u32 cpuid; / CPUID 1 EAX,
    pub /: *mut *mut __u8 cs; / code segment,
    pub /: *mut *mut __u8 bank; / machine check bank,
    pub /: *mut *mut __u8 cpu; / cpu number; obsolete; use extcpu now,
    pub /: *mut *mut __u8 finished; / entry is valid,
    pub /: *mut *mut __u32 extcpu; / linux cpu number that detected the error,
    pub /: *mut *mut __u32 socketid; / CPU socket ID,
    pub /: *mut *mut __u32 apicid; / CPU initial apic ID,
    pub /: *mut *mut __u64 mcgcap; / MCGCAP MSR: machine check capabilities of CPU,
    pub /: *mut *mut __u64 synd; / MCA_SYND MSR: only valid on SMCA systems,
    pub /: *mut *mut __u64 ipid; / MCA_IPID MSR: only valid on SMCA systems,
    pub /: *mut *mut __u64 ppin; / Protected Processor Inventory Number,
}

//
// This structure contains all data related to the MCE log.  Also
// carries a signature to make it easier to find from external
// debugging tools.  Each entry is only valid when its finished flag
// is set.
//
pub const XEN_MCE_LOG_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mce_log {
    pub /: *mut *mut char signature[12] __nonstring; / "MACHINECHECK",
    pub /: *mut *mut unsigned len; / = XEN_MCE_LOG_LEN,
    pub next: unsigned,
    pub flags: unsigned,
    pub /: *mut *mut unsigned recordlen; / length of struct xen_mce,
    pub entry: [xen_mce; XEN_MCE_LOG_LEN],
}

