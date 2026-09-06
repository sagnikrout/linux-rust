//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/uv_hub.h
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// SGI UV architectural definitions
//
// (C) Copyright 2020 Hewlett Packard Enterprise Development LP
// Copyright (C) 2007-2014 Silicon Graphics, Inc. All rights reserved.
//

//
// Addressing Terminology
//
// M       - The low M bits of a physical address represent the offset
// into the blade local memory. RAM memory on a blade is physically
// contiguous (although various IO spaces may punch holes in
// it)..
//
// N	- Number of bits in the node portion of a socket physical
// address.
//
// NASID   - network ID of a router, Mbrick or Cbrick. Nasid values of
// routers always have low bit of 1, C/MBricks have low bit
// equal to 0. Most addressing macros that target UV hub chips
// right shift the NASID by 1 to exclude the always-zero bit.
// NASIDs contain up to 15 bits.
//
// GNODE   - NASID right shifted by 1 bit. Most mmrs contain gnodes instead
// of nasids.
//
// PNODE   - the low N bits of the GNODE. The PNODE is the most useful variant
// of the nasid for socket usage.
//
// GPA	- (global physical address) a socket physical address converted
// so that it can be used by the GRU as a global address. Socket
// physical addresses 1) need additional NASID (node) bits added
// to the high end of the address, and 2) unaliased if the
// partition does not have a physical address 0. In addition, on
// UV2 rev 1, GPAs need the gnode left shifted to bits 39 or 40.
//
// NumaLink Global Physical Address Format:
// +--------------------------------+---------------------+
// |00..000|      GNODE             |      NodeOffset     |
// +--------------------------------+---------------------+
// |<-------53 - M bits --->|<--------M bits ----->
//
// M - number of node offset bits (35 .. 40)
//
// Memory/UV-HUB Processor Socket Address Format:
// +----------------+---------------+---------------------+
// |00..000000000000|   PNODE       |      NodeOffset     |
// +----------------+---------------+---------------------+
// <--- N bits --->|<--------M bits ----->
//
// M - number of node offset bits (35 .. 40)
// N - number of PNODE bits (0 .. 10)
//
// Note: M + N cannot currently exceed 44 (x86_64) or 46 (IA64).
// The actual values are configuration dependent and are set at
// boot time. M & N values are set by the hardware/BIOS at boot.
//
// APICID format
// NOTE!!!!!! This is the current format of the APICID. However, code
// should assume that this will change in the future. Use functions
// in this file for all APICID bit manipulations and conversion.
//
// 1111110000000000
// 5432109876543210
// pppppppppplc0cch	Nehalem-EX (12 bits in hdw reg)
// ppppppppplcc0cch	Westmere-EX (12 bits in hdw reg)
// pppppppppppcccch	SandyBridge (15 bits in hdw reg)
// sssssssssss
//
// p  = pnode bits
// l =  socket number on board
// c  = core
// h  = hyperthread
// s  = bits that are in the SOCKET_ID CSR
//
// Note: Processor may support fewer bits in the APICID register. The ACPI
// tables hold all 16 bits. Software needs to be aware of this.
//
// Unless otherwise specified, all references to APICID refer to
// the FULL value contained in ACPI tables, not the subset in the
// processor APICID register.
//
// Maximum number of bricks in all partitions and in all coherency domains.
// This is the total number of bricks accessible in the numalink fabric. It
// includes all C & M bricks. Routers are NOT included.
//
// This value is also the value of the maximum number of non-router NASIDs
// in the numalink fabric.
//
// NOTE: a brick may contain 1 or 2 OS nodes. Don't get these confused.
//
pub const UV_MAX_NUMALINK_BLADES: c_int = 16384;
//
// Maximum number of C/Mbricks within a software SSI (hardware may support
// more).
//
pub const UV_MAX_SSI_BLADES: c_int = 256;
//
// The largest possible NASID of a C or M brick (+ 2)
//

// GAM (globally addressed memory) range table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_gam_range_s {
    pub /: *mut *mut u32 limit; / PA bits 56:26 (GAM_RANGE_SHFT),
    pub /: *mut *mut u16 nasid; / node's global physical address,
    pub /: *mut *mut s8 base; / entry index of node's base addr,
    pub reserved: u8,
}

//
// The following defines attributes of the HUB chip. These attributes are
// frequently referenced and are kept in a common per hub struct.
// After setup, the struct is read only, so it should be readily
// available in the L3 cache on the cpu socket for the node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_hub_info_s {
    pub hub_type: c_uint,
    pub hub_revision: c_uchar,
    pub global_mmr_base: c_ulong,
    pub global_mmr_shift: c_ulong,
    pub gpa_mask: c_ulong,
    pub socket_to_node: *mut c_ushort,
    pub socket_to_pnode: *mut c_ushort,
    pub pnode_to_socket: *mut c_ushort,
    pub gr_table: *mut uv_gam_range_s,
    pub min_socket: c_ushort,
    pub min_pnode: c_ushort,
    pub m_val: c_uchar,
    pub n_val: c_uchar,
    pub gr_table_len: c_uchar,
    pub apic_pnode_shift: c_uchar,
    pub gpa_shift: c_uchar,
    pub nasid_shift: c_uchar,
    pub m_shift: c_uchar,
    pub n_lshift: c_uchar,
    pub gnode_extra: c_uint,
    pub gnode_upper: c_ulong,
    pub lowmem_remap_top: c_ulong,
    pub lowmem_remap_base: c_ulong,
    pub global_gru_base: c_ulong,
    pub global_gru_shift: c_ulong,
    pub pnode: c_ushort,
    pub pnode_mask: c_ushort,
    pub coherency_domain_number: c_ushort,
    pub numa_blade_id: c_ushort,
    pub nr_possible_cpus: c_ushort,
    pub nr_online_cpus: c_ushort,
    pub memory_nid: c_short,
    pub node_to_socket: *mut c_ushort,
}

// CPU specific info with a pointer to the hub common info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cpu_info_s {
    pub p_uv_hub_info: *mut c_void,
    pub blade_cpu_id: c_uchar,
    pub reserved: *mut c_void,
}

// Node specific hub common info struct

//
// HUB revision ranges for each UV HUB architecture.
// This is a software convention - NOT the hardware revision numbers in
// the hub chip.
//
pub const UV2_HUB_REVISION_BASE: c_int = 3;
pub const UV3_HUB_REVISION_BASE: c_int = 5;
pub const UV4_HUB_REVISION_BASE: c_int = 7;

pub const UV5_HUB_REVISION_BASE: c_int = 9;
//
// UV4A is a revision of UV4.  So on UV4A, both is_uv4_hub() and
// is_uv4a_hub() return true, While on UV4, only is_uv4_hub()
// returns true.  So to get true results, first test if is UV4A,
// then test if is UV4.
//
// UVX class: UV2,3,4
// UVY class: UV5,..?
// Any UV Hubbed System
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_apicid {
    pub v: c_ulong,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_apicid_s {
    pub 24: unsigned long local_apic_mask :,
    pub 5: unsigned long local_apic_shift :,
    pub 3: unsigned long unused1 :,
    pub 24: unsigned long pnode_mask :,
    pub 5: unsigned long pnode_shift :,
    pub 3: unsigned long unused2 :,
    pub s: },
}

//
// Local & Global MMR space macros.
// Note: macros are intended to be used ONLY by inline functions
// in this file - not by other kernel code.
// n -  NASID (full 15-bit global nasid)
// g -  GNODE (full 15-bit global nasid, right shifted 1)
// p -  PNODE (local part of nsids, right shifted 1)
//

pub const UV2_LOCAL_MMR_BASE: c_uint = 0xfa000000UL;
pub const UV2_GLOBAL_MMR32_BASE: c_uint = 0xfc000000UL;

pub const UV3_LOCAL_MMR_BASE: c_uint = 0xfa000000UL;
pub const UV3_GLOBAL_MMR32_BASE: c_uint = 0xfc000000UL;

pub const UV4_LOCAL_MMR_BASE: c_uint = 0xfa000000UL;
pub const UV4_GLOBAL_MMR32_BASE: c_int = 0;

pub const UV4_GLOBAL_MMR32_SIZE: c_int = 0;
pub const UV5_LOCAL_MMR_BASE: c_uint = 0xfa000000UL;
pub const UV5_GLOBAL_MMR32_BASE: c_int = 0;

pub const UV5_GLOBAL_MMR32_SIZE: c_int = 0;

pub const UV_GLOBAL_GRU_MMR_BASE: c_uint = 0x4000000;
pub const UV_GLOBAL_MMR32_PNODE_SHIFT: c_int = 15;
pub const _UV_GLOBAL_MMR64_PNODE_SHIFT: c_int = 26;

pub const UVH_APICID: c_uint = 0x002D0E00L;
pub const UV_APIC_PNODE_SHIFT: c_int = 6;
// Local Bus from cpu's perspective
pub const LOCAL_BUS_BASE: c_uint = 0x1c00000;

//
// System Controller Interface Reg
//
// Note there are NO leds on a UV system.  This register is only
// used by the system controller to monitor system-wide operation.
// There are 64 regs per node.  With Nehalem cpus (2 cores per node,
// 8 cpus per core, 2 threads per cpu) there are 32 cpu threads on
// a node.
//
// The window is located at top of ACPI MMR space
//
pub const SCIR_WINDOW_COUNT: c_int = 64;

pub const SCIR_CPU_HEARTBEAT: c_uint = 0x01	/* timer interrupt */;
pub const SCIR_CPU_ACTIVITY: c_uint = 0x02	/* not idle */;

// Loop through all installed blades

//
// Macros for converting between kernel virtual addresses, socket local physical
// addresses, and UV global physical addresses.
// Note: use the standard __pa() & __va() macros for converting
// between socket virtual and socket physical addresses.
//
// global bits offset - number of local address bits in gpa for this UV arch
// Macro flag: #define	_uv_gpa_shift
// Find node that has the address range that contains global address
// Return base address of node that contains global address
// socket phys RAM --> UV global NASID (UV4+)
// Macro flag: #define	_uv_soc_phys_ram_to_nasid
// socket virtual --> UV global NASID (UV4+)
extern "C" {
    pub fn uv_soc_phys_ram_to_nasid(_arg: __pa(v)) -> return;
}
// socket phys RAM --> UV global physical address
// socket virtual --> UV global physical address
extern "C" {
    pub fn uv_soc_phys_ram_to_gpa(_arg: __pa(v)) -> return;
}
// Top two bits indicate the requested address is in MMR space.
// UV global physical address --> socket phys RAM
// gpa -> gnode
// gpa -> pnode
// gpa -> node offset
// Convert socket to node
extern "C" {
    pub fn _uv_socket_to_node(_arg: socket, _arg: uv_hub_info->socket_to_node) -> return;
}
// pnode, offset --> socket virtual
extern "C" {
    pub fn __va(offset: ((unsigned long)pnode << m_val) |) -> return;
}
// limit address of previous socket is our base, except node 0 is 0
extern "C" {
    pub fn __va(long)offset: (unsigned) -> return;
}
extern "C" {
    pub fn __va(offset: base << UV_GAM_RANGE_SHFT |) -> return;
}
// Extract/Convert a PNODE from an APICID (full apicid, not processor subset)
//
// Access global MMRs using the low memory MMR32 space. This region supports
// faster MMR access but not all MMRs are accessible in this space.
//
extern "C" {
    pub fn readq(_arg: uv_global_mmr32_address(pnode, _arg: offset)) -> return;
}
//
// Access Global MMR space using the MMR space located at the top of physical
// memory.
//
extern "C" {
    pub fn readq(_arg: uv_global_mmr64_address(pnode, _arg: offset)) -> return;
}
extern "C" {
    pub fn readb(_arg: uv_global_mmr64_address(pnode, _arg: offset)) -> return;
}
//
// Access hub local MMRs. Faster than using global space but only local MMRs
// are accessible.
//
extern "C" {
    pub fn __va(offset: UV_LOCAL_MMR_BASE |) -> return;
}
extern "C" {
    pub fn readq(_arg: uv_local_mmr_address(offset)) -> return;
}
extern "C" {
    pub fn readb(_arg: uv_local_mmr_address(offset)) -> return;
}
// Blade-local cpu number of current cpu. Numbered 0 .. <# cpus on the blade>
// Blade-local cpu number of cpu N. Numbered 0 .. <# cpus on the blade>
// Blade number to Node number (UV2..UV4 is 1:1)
extern "C" {
    pub fn uv_socket_to_node(_arg: blade) -> return;
}
// Blade number of current cpu. Numbered 0 .. <#blades -1>
//
// Convert linux node number to the UV blade number.
// .. Currently for UV2 thru UV4 the node and the blade are identical.
// .. UV5 needs conversion when sub-numa clustering is enabled.
//
// Convert a CPU number to the UV blade number
// Convert a blade id to the PNODE of the blade
// Nid of memory node on blade. -1 if no blade-local memory
// Determine the number of possible cpus on a blade
// Determine the number of online cpus on a blade
// Convert a cpu id to the PNODE of the blade containing the cpu
// Convert a linux node number to the PNODE of the blade
// Maximum possible number of blades
// Per Hub NMI support
extern "C" {
    pub fn uv_nmi_setup();
}
extern "C" {
    pub fn uv_nmi_setup_hubless();
}
// BIOS/Kernel flags exchange MMR

// TSC sync valid, set by BIOS

pub const UVH_TSC_SYNC_SHIFT: c_int = 10;

// BMC sets a bit this MMR non-zero before sending an NMI

pub const UVH_NMI_MMR_SHIFT: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_hub_nmi_s {
    pub nmi_lock: raw_spinlock_t,
    pub /: *mut *mut atomic_t in_nmi; / flag this node in UV NMI IRQ,
    pub /: *mut *mut atomic_t cpu_owner; / last locker of this struct,
    pub /: *mut *mut atomic_t read_mmr_count; / count of MMR reads,
    pub /: *mut *mut atomic_t nmi_count; / count of true UV NMIs,
    pub /: *mut *mut unsigned long nmi_value; / last value read from NMI MMR,
    pub /: *mut *mut bool hub_present; / false means UV hubless system,
    pub /: *mut *mut bool pch_owner; / indicates this hub owns PCH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cpu_nmi_s {
    pub hub: *mut uv_hub_nmi_s,
    pub state: c_int,
    pub pinging: c_int,
    pub queries: c_int,
    pub pings: c_int,
}

// uv_cpu_nmi_states
pub const UV_NMI_STATE_OUT: c_int = 0;
pub const UV_NMI_STATE_IN: c_int = 1;
pub const UV_NMI_STATE_DUMP: c_int = 2;
pub const UV_NMI_STATE_DUMP_DONE: c_int = 3;
//
// Get the minimum revision number of the hub chips within the partition.
// (See UVx_HUB_REVISION_BASE above for specific values.)
//

