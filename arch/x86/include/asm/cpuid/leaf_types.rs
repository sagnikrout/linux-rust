//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpuid/leaf_types.h
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
// Generator: x86-cpuid-db v3.1
//
// Auto-generated file.
// Please submit all updates and bugfixes to https://x86-cpuid.org
//

//
// Leaf 0x0
// Maximum standard leaf + CPU vendor string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x0_0 {
// eax
    pub leaf: u32 max_std_leaf : 32; // Highest standard CPUID,
// ebx
    pub 3: u32 cpu_vendorid_0 : 32; // CPU vendor ID string bytes 0 -,
// ecx
    pub 11: u32 cpu_vendorid_2 : 32; // CPU vendor ID string bytes 8 -,
// edx
    pub 7: u32 cpu_vendorid_1 : 32; // CPU vendor ID string bytes 4 -,
}

//
// Leaf 0x1
// CPU FMS (Family/Model/Stepping) + standard feature flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1_0 {
// eax
    pub Reserved: : 4; //,
// ebx
    pub ID: local_apic_id : 8; // Initial local APIC physical,
// ecx
    pub system: guest_status : 1; // System is running as guest; (para-)virtualized,
// edx
    pub Enable: pbe : 1; // Pending Break,
}

//
// Leaf 0x2
// Intel cache and TLB information one-byte descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x2_0 {
// eax
    pub set: eax_invalid : 1; // Descriptors 1-3 are invalid if,
// ebx
    pub set: ebx_invalid : 1; // Descriptors 4-7 are invalid if,
// ecx
    pub set: ecx_invalid : 1; // Descriptors 8-11 are invalid if,
// edx
    pub set: edx_invalid : 1; // Descriptors 12-15 are invalid if,
}

//
// Leaf 0x4
// Intel deterministic cache parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x4_n {
// eax
    pub package: num_cores_on_die : 6; // Number of cores in the physical,
// ebx
    pub (0-based): cache_nways : 10; // Ways of associativity,
// ecx
    pub Reserved: : 1; //,
// edx
    pub Reserved: : 29; //,
}

pub const LEAF_0x4_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x4_SUBLEAF_N_LAST: c_int = 31;
//
// Leaf 0x5
// MONITOR/MWAIT instructions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x5_0 {
// eax
    pub Reserved: : 16; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: : 30; //,
// edx
    pub C-states: n_c7_substates : 4; // Number of C7 sub,
}

//
// Leaf 0x6
// Thermal and power management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x6_0 {
// eax
    pub Reserved: : 7; //,
// ebx
    pub Reserved: : 28; //,
// ecx
    pub Reserved: : 16; //,
// edx
    pub index: this_lcpu_hwfdbk_idx : 16; // This logical CPU hardware feedback interface,
}

//
// Leaf 0x7
// Extended CPU features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x7_0 {
// eax
    pub subleaves: u32 leaf7_n_subleaves : 32; // Number of leaf 0x7,
// ebx
    pub extensions: avx512vl : 1; // AVX-512 VL (128/256 vector length),
// ecx
    pub pages: pks : 1; // Protection keys for supervisor-mode,
// edx
    pub disable: spec_ctrl_ssbd : 1; // Speculative store bypass,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x7_1 {
// eax
    pub Reserved: : 4; //,
// ebx
    pub Reserved: : 31; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: : 13; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x7_2 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: : 25; //,
}

//
// Leaf 0x9
// Intel DCA (Direct Cache Access)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x9_0 {
// eax
    pub Reserved: : 31; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0xa
// Intel PMU (Performance Monitoring Unit)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xa_0 {
// eax
    pub vector: events_mask_len : 8; // Length of CPUID(0xa).EBX bit,
// ebx
    pub Reserved: : 19; //,
// ecx
    pub bitmap: u32 pmu_fcounters_bitmap : 32; // Fixed-function PMU counters support,
// edx
    pub Reserved: : 16; //,
}

//
// Leaf 0xb
// CPU extended topology v1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xb_n {
// eax
    pub Reserved: : 27; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: : 16; //,
// edx
    pub CPU: u32 x2apic_id : 32; // x2APIC ID of current logical,
}

pub const LEAF_0xb_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0xb_SUBLEAF_N_LAST: c_int = 1;
//
// Leaf 0xd
// CPU extended state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xd_0 {
// eax
    pub Reserved: : 13; //,
// ebx
    pub features: u32 xsave_sz_xcr0 : 32; // XSAVE/XRSTOR area byte size, for XCR0 enabled,
// ecx
    pub features: u32 xsave_sz_max : 32; // XSAVE/XRSTOR area max byte size, all CPU,
// edx
    pub Reserved: : 1; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xd_1 {
// eax
    pub Reserved: : 27; //,
// ebx
    pub features: u32 xsave_sz_xcr0_xss : 32; // XSAVES/XSAVEC area byte size, for XCR0|XSS enabled,
// ecx
    pub Reserved: : 15; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xd_n {
// eax
    pub bytes: u32 xsave_sz : 32; // Subleaf-N feature save area size, in,
// ebx
    pub bytes: u32 xsave_offset : 32; // Subleaf-N feature save area offset, in,
// ecx
    pub Reserved: : 30; //,
// edx
    pub Reserved: u32 : 32; //,
}

pub const LEAF_0xd_SUBLEAF_N_FIRST: c_int = 2;
pub const LEAF_0xd_SUBLEAF_N_LAST: c_int = 63;
//
// Leaf 0xf
// Intel RDT / AMD PQoS resource monitoring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xf_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub (0-based): u32 core_rmid_max : 32; // RMID max within this core,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: : 30; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xf_1 {
// eax
    pub Reserved: : 21; //,
// ebx
    pub bytes: u32 l3c_qm_conver_factor : 32; // QM_CTR MSR conversion factor to,
// ecx
    pub RMID: u32 l3c_qm_rmid_max : 32; // L3 QoS-monitoring max,
// edx
    pub Reserved: : 29; //,
}

//
// Leaf 0x10
// Intel RDT / AMD PQoS allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x10_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 28; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x10_n {
// eax
    pub Reserved: : 27; //,
// ebx
    pub bitmap: u32 cat_units_bitmap : 32; // L3/L2_CAT allocation units,
// ecx
    pub Reserved: : 28; //,
// edx
    pub Reserved: : 16; //,
}

pub const LEAF_0x10_SUBLEAF_N_FIRST: c_int = 1;
pub const LEAF_0x10_SUBLEAF_N_LAST: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x10_3 {
// eax
    pub notation: u32 mba_max_delay : 12, // Max MBA throttling value; minus-one,
    pub Reserved: : 20; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: : 29; //,
// edx
    pub Reserved: : 16; //,
}

//
// Leaf 0x12
// Intel SGX (Software Guard Extensions)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x12_0 {
// eax
    pub Reserved: : 20; //,
// ebx
    pub Reserved: : 30; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: : 16; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x12_1 {
// eax
    pub Reserved: : 21; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: : 13; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x12_n {
// eax
    pub bits: [epc_sec_base_addr_0 : 20; // EPC section base address,; 12:31],
// ebx
    pub Reserved: : 12; //,
// ecx
    pub bits: [epc_sec_size_0 : 20; // EPC section size,; 12:31],
// edx
    pub Reserved: : 12; //,
}

pub const LEAF_0x12_SUBLEAF_N_FIRST: c_int = 2;
pub const LEAF_0x12_SUBLEAF_N_LAST: c_int = 31;
//
// Leaf 0x14
// Intel Processor Trace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x14_0 {
// eax
    pub subleaf: u32 pt_max_subleaf : 32; // Maximum leaf 0x14,
// ebx
    pub preservation: ip_filtering : 1, // IP/TraceStop filtering; Warm-reset PT MSRs,
    pub suppression: mtc_timing : 1, // MTC timing packet; COFI-based packets,
    pub Reserved: : 23; //,
// ecx
    pub included): ip_payloads_lip : 1; // IP payloads have LIP values (CS base,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x14_1 {
// eax
    pub bitmap: mtc_periods_bmp : 16; // MTC period encodings,
// ebx
    pub bitmap: psb_periods_bmp : 16; // Configurable PSB frequency encodings,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x15
// Intel TSC (Time Stamp Counter)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x15_0 {
// eax
    pub ratio: u32 tsc_denominator : 32; // Denominator of the TSC/'core crystal clock',
// ebx
    pub ratio: u32 tsc_numerator : 32; // Numerator of the TSC/'core crystal clock',
// ecx
    pub Hz: u32 cpu_crystal_hz : 32; // Core crystal clock nominal frequency, in,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x16
// Intel processor frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x16_0 {
// eax
    pub Reserved: : 16; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: : 16; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x17
// Intel SoC vendor attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x17_0 {
// eax
    pub subleaf: u32 soc_max_subleaf : 32; // Maximum leaf 0x17,
// ebx
    pub Reserved: : 15; //,
// ecx
    pub vendor: u32 soc_proj_id : 32; // SoC project ID, assigned by,
// edx
    pub vendor: u32 soc_stepping_id : 32; // SoC project stepping ID, assigned by,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x17_n {
// eax
    pub 3): *mut *mut u32 vendor_brand_a : 32; // Vendor Brand ID string, bytes subleaf_nr  (0 ->,
// ebx
    pub 7): *mut *mut u32 vendor_brand_b : 32; // Vendor Brand ID string, bytes subleaf_nr  (4 ->,
// ecx
    pub 11): *mut *mut u32 vendor_brand_c : 32; // Vendor Brand ID string, bytes subleaf_nr  (8 ->,
// edx
    pub 15): *mut *mut u32 vendor_brand_d : 32; // Vendor Brand ID string, bytes subleaf_nr  (12 ->,
}

pub const LEAF_0x17_SUBLEAF_N_FIRST: c_int = 1;
pub const LEAF_0x17_SUBLEAF_N_LAST: c_int = 3;
//
// Leaf 0x18
// Intel deterministic address translation (TLB) parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x18_n {
// eax
    pub subleaf: u32 tlb_max_subleaf : 32; // Maximum leaf 0x18,
// ebx
    pub associativity: n_way_associative : 16; // Ways of,
// ecx
    pub sets: u32 n_sets : 32; // Number of,
// edx
    pub Reserved: : 6; //,
}

pub const LEAF_0x18_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x18_SUBLEAF_N_LAST: c_int = 31;
//
// Leaf 0x19
// Intel key locker
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x19_0 {
// eax
    pub Reserved: : 29; //,
// ebx
    pub Reserved: : 27; //,
// ecx
    pub Reserved: : 30; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x1a
// Intel hybrid CPUs identification (e.g. Atom, Core)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1a_0 {
// eax
    pub type: core_type : 8; // This core's,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x1b
// Intel PCONFIG (Platform configuration)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1b_n {
// eax
    pub Reserved: : 20; //,
// ebx
    pub ID: u32 pconfig_target_id_x : 32; // A supported PCONFIG target,
// ecx
    pub ID: u32 pconfig_target_id_y : 32; // A supported PCONFIG target,
// edx
    pub ID: u32 pconfig_target_id_z : 32; // A supported PCONFIG target,
}

pub const LEAF_0x1b_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x1b_SUBLEAF_N_LAST: c_int = 31;
//
// Leaf 0x1c
// Intel LBR (Last Branch Record)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1c_0 {
// eax
    pub IP): lbr_ip_is_lip : 1; // LBR IP contain Last IP (otherwise effective,
// ebx
    pub Reserved: : 29; //,
// ecx
    pub Reserved: : 12; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x1d
// Intel AMX (Advanced Matrix Extensions) tile information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1d_0 {
// eax
    pub ID: u32 amx_max_palette : 32; // Highest palette ID / subleaf,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1d_1 {
// eax
    pub bytes: amx_tile_size : 16; // AMX single tile's size, in,
// ebx
    pub tiles: amx_palette_nr_tiles : 16; // AMX palette number of,
// ecx
    pub Reserved: : 16; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x1e
// Intel TMUL (Tile-matrix Multiply)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1e_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 8; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x1f
// Intel extended topology v2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x1f_n {
// eax
    pub Reserved: : 27; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: : 16; //,
// edx
    pub CPU: u32 x2apic_id : 32; // x2APIC ID of current logical,
}

pub const LEAF_0x1f_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x1f_SUBLEAF_N_LAST: c_int = 5;
//
// Leaf 0x20
// Intel HRESET (History Reset)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x20_0 {
// eax
    pub 1: u32 hreset_nr_subleaves : 32; // CPUID 0x20 max subleaf +,
// ebx
    pub Reserved: : 31; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x21
// Intel TD (Trust Domain)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x21_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub 3: u32 tdx_vendorid_0 : 32; // TDX vendor ID string bytes 0 -,
// ecx
    pub 11: u32 tdx_vendorid_2 : 32; // TDX vendor ID string bytes 8 -,
// edx
    pub 7: u32 tdx_vendorid_1 : 32; // TDX vendor ID string bytes 4 -,
}

//
// Leaf 0x23
// Intel Architectural Performance Monitoring Extended (ArchPerfmonExt)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_0 {
// eax
    pub Reserved: : 26; //,
// ebx
    pub Reserved: : 29; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_1 {
// eax
    pub counters: u32 gp_counters : 32; // Bitmap of general-purpose PMU,
// ebx
    pub counters: u32 fixed_counters : 32; // Bitmap of fixed PMU,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_2 {
// eax
    pub reloaded: u32 acr_gp_reload : 32; // Bitmap of general-purpose counters that can be,
// ebx
    pub reloaded: u32 acr_fixed_reload : 32; // Bitmap of fixed counters that can be,
// ecx
    pub reloads: u32 acr_gp_trigger : 32; // Bitmap of general-purpose counters that can trigger,
// edx
    pub reloads: u32 acr_fixed_trigger : 32; // Bitmap of fixed counters that can trigger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_3 {
// eax
    pub Reserved: : 20; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_4 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 1; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x23_5 {
// eax
    pub counters: u32 pebs_gp : 32; // Architectural PEBS general-purpose,
// ebx
    pub counters: u32 pebs_pdist_gp : 32; // Architectural PEBS PDIST general-purpose,
// ecx
    pub counters: u32 pebs_fixed : 32; // Architectural PEBS fixed,
// edx
    pub counters: u32 pebs_pdist_fixed : 32; // Architectural PEBS PDIST fixed,
}

//
// Leaf 0x40000000
// Maximum hypervisor leaf + hypervisor vendor string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x40000000_0 {
// eax
    pub leaf: u32 max_hyp_leaf : 32; // Maximum hypervisor,
// ebx
    pub 3: u32 hypervisor_id_0 : 32; // Hypervisor ID string bytes 0 -,
// ecx
    pub 7: u32 hypervisor_id_1 : 32; // Hypervisor ID string bytes 4 -,
// edx
    pub 11: u32 hypervisor_id_2 : 32; // Hypervisor ID string bytes 8 -,
}

//
// Leaf 0x4c780001
// Linux-defined synthetic feature flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x4c780001_0 {
// eax
    pub frequency: tsc_known_freq : 1; // TSC has known,
// ebx
    pub configured: msr_ia32_feat_ctl : 1; // MSR IA32_FEAT_CTL,
// ecx
    pub Reserved: : 9; //,
// edx
    pub microarchitecture: zen1 : 1; // CPU based on Zen1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x4c780001_1 {
// eax
    pub Reserved: : 28; //,
// ebx
    pub Reserved: : 17; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x4c780002
// Linux-defined synthetic CPU bug flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x4c780002_0 {
// eax
    pub isolation: cpu_meltdown : 1, // CPU affected by meltdown; needs kernel page table,
    pub memory: tdx_pw_mce : 1; // CPU may incur #MC if non-TD software does partial write to TDX private,
// ebx
    pub something: old_microcode : 1, // CPU has old microcode; it must be vulnerable to,
    pub affected: its_native_only : 1, // CPU affected by ITS; VMX is not,
    pub Reserved: : 21; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80000000
// Maximum extended leaf + CPU vendor string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000000_0 {
// eax
    pub leaf: u32 max_ext_leaf : 32; // Maximum extended CPUID,
// ebx
    pub 3: u32 cpu_vendorid_0 : 32; // Vendor ID string bytes 0 -,
// ecx
    pub 11: u32 cpu_vendorid_2 : 32; // Vendor ID string bytes 8 -,
// edx
    pub 7: u32 cpu_vendorid_1 : 32; // Vendor ID string bytes 4 -,
}

//
// Leaf 0x80000001
// Extended CPU features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000001_0 {
// eax
    pub Reserved: : 4; //,
// ebx
    pub type: pkg_type : 4; // Package,
// ecx
    pub Reserved: : 1; //,
// edx
    pub instructions: _3dnow : 1; // 3DNow,
}

//
// Leaf 0x80000002
// CPU brand ID string, bytes 0 - 15
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000002_0 {
// eax
    pub 3: u32 cpu_brandid_0 : 32; // CPU brand ID string, bytes 0 -,
// ebx
    pub 7: u32 cpu_brandid_1 : 32; // CPU brand ID string, bytes 4 -,
// ecx
    pub 11: u32 cpu_brandid_2 : 32; // CPU brand ID string, bytes 8 -,
// edx
    pub 15: u32 cpu_brandid_3 : 32; // CPU brand ID string, bytes 12 -,
}

//
// Leaf 0x80000003
// CPU brand ID string, bytes 16 - 31
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000003_0 {
// eax
    pub 19: u32 cpu_brandid_4 : 32; // CPU brand ID string bytes, 16 -,
// ebx
    pub 23: u32 cpu_brandid_5 : 32; // CPU brand ID string bytes, 20 -,
// ecx
    pub 27: u32 cpu_brandid_6 : 32; // CPU brand ID string bytes, 24 -,
// edx
    pub 31: u32 cpu_brandid_7 : 32; // CPU brand ID string bytes, 28 -,
}

//
// Leaf 0x80000004
// CPU brand ID string, bytes 32 - 47
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000004_0 {
// eax
    pub 35: u32 cpu_brandid_8 : 32; // CPU brand ID string, bytes 32 -,
// ebx
    pub 39: u32 cpu_brandid_9 : 32; // CPU brand ID string, bytes 36 -,
// ecx
    pub 43: u32 cpu_brandid_10 : 32; // CPU brand ID string, bytes 40 -,
// edx
    pub 47: u32 cpu_brandid_11 : 32; // CPU brand ID string, bytes 44 -,
}

//
// Leaf 0x80000005
// AMD/Transmeta L1 cache and TLB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000005_0 {
// eax
    pub pages: l1_dtlb_2m_4m_assoc : 8; // L1 DTLB associativity, 2M and 4M,
// ebx
    pub pages: l1_dtlb_4k_assoc : 8; // L1 DTLB associativity, 4K,
// ecx
    pub KB: l1_dcache_size_kb : 8; // L1 dcache size, in,
// edx
    pub KB: l1_icache_size_kb : 8; // L1 icache size, in,
}

//
// Leaf 0x80000006
// (Mostly AMD) L2/L3 cache and TLB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000006_0 {
// eax
    pub pages: l2_dtlb_2m_4m_assoc : 4; // L2 dTLB associativity, 2M and 4M,
// ebx
    pub pages: l2_dtlb_4k_assoc : 4; // L2 dTLB associativity, 4K,
// ecx
    pub KB: l2_size_kb : 16; // L2 cache size, in,
// edx
    pub range: l3_size_range : 14; // L3 cache size,
}

//
// Leaf 0x80000007
// CPU power management (mostly AMD) and AMD RAS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000007_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 28; //,
// ecx
    pub ratio: u32 cpu_pwr_sample_ratio : 32; // CPU power sample time,
// edx
    pub Reserved: : 17; //,
}

//
// Leaf 0x80000008
// CPU capacity parameters and extended feature flags (mostly AMD)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000008_0 {
// eax
    pub Reserved: : 8; //,
// ebx
    pub Sampling: branch_sampling : 1; // Branch,
// ecx
    pub Reserved: : 14; //,
// edx
    pub input): rdpru_max_reg_id : 16; // RDPRU max register ID (ECX,
}

//
// Leaf 0x8000000a
// AMD SVM (Secure Virtual Machine)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000000a_0 {
// eax
    pub Reserved: : 24; //,
// ebx
    pub (ASID): u32 svm_nasid : 32; // Number of address space identifiers,
// ecx
    pub Reserved: : 27; //,
// edx
    pub Reserved: : 3; //,
}

//
// Leaf 0x80000019
// AMD TLB characteristics for 1GB pages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000019_0 {
// eax
    pub pages: l1_dtlb_1g_assoc : 4; // L1 dTLB associativity, 1G,
// ebx
    pub pages: l2_dtlb_1g_assoc : 4; // L2 dTLB associativity, 1G,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x8000001a
// AMD instruction optimizations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001a_0 {
// eax
    pub Reserved: : 29; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x8000001b
// AMD IBS (Instruction-Based Sampling)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001b_0 {
// eax
    pub Reserved: : 20; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x8000001c
// AMD LWP (Lightweight Profiling)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001c_0 {
// eax
    pub overflow: os_lwp_int : 1; // OS: Interrupt on threshold,
// ebx
    pub offset: lwp_event_offset : 8; // Control Block events area,
// ecx
    pub latency: lwp_cache_latency : 1; // Cache-related events: filter by,
// edx
    pub overflow: hw_lwp_int : 1; // HW: Interrupt on threshold,
}

//
// Leaf 0x8000001d
// AMD deterministic cache parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001d_n {
// eax
    pub Reserved: : 6; //,
// ebx
    pub (0-based): cache_nways : 10; // Ways of associativity,
// ecx
    pub Reserved: : 1; //,
// edx
    pub Reserved: : 30; //,
}

pub const LEAF_0x8000001d_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x8000001d_SUBLEAF_N_LAST: c_int = 31;
//
// Leaf 0x8000001e
// AMD CPU topology
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001e_0 {
// eax
    pub ID: u32 ext_apic_id : 32; // Extended APIC,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: : 21; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x8000001f
// AMD encrypted memory capabilities (SME/SEV)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x8000001f_0 {
// eax
    pub Reserved: : 2; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub guests: u32 enc_guests_max : 32; // Max number of simultaneous encrypted,
// edx
    pub guest: u32 min_sev_asid_no_sev_es : 32; // Minimum ASID for SEV-enabled SEV-ES-disabled,
}

//
// Leaf 0x80000020
// AMD PQoS (Platform QoS) extended features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000020_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 25; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000020_1 {
// eax
    pub size: u32 mba_limit_len : 32; // MBA enforcement limit,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub (zero-based): u32 mba_cos_max : 32; // MBA max Class of Service number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000020_2 {
// eax
    pub size: u32 smba_limit_len : 32; // SMBA enforcement limit,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub (zero-based): u32 smba_cos_max : 32; // SMBA max Class of Service number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000020_3 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: : 24; //,
// ecx
    pub Reserved: : 25; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80000021
// AMD extended CPU features 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000021_0 {
// eax
    pub mitigation: srso_msr_fix : 1; // MSR BP_CFG[BpSpecReduce] SRSO,
// ebx
    pub Reserved: : 8; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80000022
// AMD extended performance monitoring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000022_0 {
// eax
    pub Reserved: : 29; //,
// ebx
    pub Reserved: : 10; //,
// ecx
    pub bitmask: u32 active_umc_bitmask : 32; // Active UMCs,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80000023
// AMD multi-key encrypted memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000023_0 {
// eax
    pub Reserved: : 31; //,
// ebx
    pub Reserved: : 16; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80000026
// AMD extended CPU topology
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80000026_n {
// eax
    pub count: domain_core_count_asymm : 1; // The 'Core' domain has asymmetric cores,
// ebx
    pub type: core_type : 4; // This core's,
// ecx
    pub Reserved: : 16; //,
// edx
    pub CPU: u32 x2apic_id : 32; // x2APIC ID of current logical,
}

pub const LEAF_0x80000026_SUBLEAF_N_FIRST: c_int = 0;
pub const LEAF_0x80000026_SUBLEAF_N_LAST: c_int = 3;
//
// Leaf 0x80860000
// Maximum Transmeta leaf + CPU vendor string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860000_0 {
// eax
    pub leaf: u32 max_tra_leaf : 32; // Maximum Transmeta,
// ebx
    pub 3: u32 cpu_vendorid_0 : 32; // Transmeta vendor ID string bytes 0 -,
// ecx
    pub 11: u32 cpu_vendorid_2 : 32; // Transmeta vendor ID string bytes 8 -,
// edx
    pub 7: u32 cpu_vendorid_1 : 32; // Transmeta vendor ID string bytes 4 -,
}

//
// Leaf 0x80860001
// Transmeta extended CPU features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860001_0 {
// eax
    pub Reserved: : 18; //,
// ebx
    pub major: cpu_rev_major : 8; // CPU revision ID,,
// ecx
    pub MHz: u32 cpu_base_mhz : 32; // CPU nominal frequency, in,
// edx
    pub Reserved: : 28; //,
}

//
// Leaf 0x80860002
// Transmeta CMS (Code Morphing Software)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860002_0 {
// eax
    pub ID: u32 cpu_rev_id : 32; // CPU revision,
// ebx
    pub major: cms_rev_major : 8; // CMS revision ID,,
// ecx
    pub 3: u32 cms_rev_mask_3 : 32; // CMS revision ID, mask component,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0x80860003
// Transmeta CPU information string, bytes 0 - 15
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860003_0 {
// eax
    pub 3: u32 cpu_info_0 : 32; // CPU info string bytes 0 -,
// ebx
    pub 7: u32 cpu_info_1 : 32; // CPU info string bytes 4 -,
// ecx
    pub 11: u32 cpu_info_2 : 32; // CPU info string bytes 8 -,
// edx
    pub 15: u32 cpu_info_3 : 32; // CPU info string bytes 12 -,
}

//
// Leaf 0x80860004
// Transmeta CPU information string, bytes 16 - 31
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860004_0 {
// eax
    pub 19: u32 cpu_info_4 : 32; // CPU info string bytes 16 -,
// ebx
    pub 23: u32 cpu_info_5 : 32; // CPU info string bytes 20 -,
// ecx
    pub 27: u32 cpu_info_6 : 32; // CPU info string bytes 24 -,
// edx
    pub 31: u32 cpu_info_7 : 32; // CPU info string bytes 28 -,
}

//
// Leaf 0x80860005
// Transmeta CPU information string, bytes 32 - 47
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860005_0 {
// eax
    pub 35: u32 cpu_info_8 : 32; // CPU info string bytes 32 -,
// ebx
    pub 39: u32 cpu_info_9 : 32; // CPU info string bytes 36 -,
// ecx
    pub 43: u32 cpu_info_10 : 32; // CPU info string bytes 40 -,
// edx
    pub 47: u32 cpu_info_11 : 32; // CPU info string bytes 44 -,
}

//
// Leaf 0x80860006
// Transmeta CPU information string, bytes 48 - 63
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860006_0 {
// eax
    pub 51: u32 cpu_info_12 : 32; // CPU info string bytes 48 -,
// ebx
    pub 55: u32 cpu_info_13 : 32; // CPU info string bytes 52 -,
// ecx
    pub 59: u32 cpu_info_14 : 32; // CPU info string bytes 56 -,
// edx
    pub 63: u32 cpu_info_15 : 32; // CPU info string bytes 60 -,
}

//
// Leaf 0x80860007
// Transmeta live CPU information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x80860007_0 {
// eax
    pub MHz: u32 cpu_cur_mhz : 32; // Current CPU frequency, in,
// ebx
    pub millivolts: u32 cpu_cur_voltage : 32; // Current CPU voltage, in,
// ecx
    pub 100: u32 cpu_cur_perf_pctg : 32; // Current CPU performance percentage, 0 -,
// edx
    pub femtoseconds: u32 cpu_cur_gate_delay : 32; // Current CPU gate delay, in,
}

//
// Leaf 0xc0000000
// Maximum Centaur/Zhaoxin leaf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xc0000000_0 {
// eax
    pub leaf: u32 max_cntr_leaf : 32; // Maximum Centaur/Zhaoxin,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: u32 : 32; //,
}

//
// Leaf 0xc0000001
// Centaur/Zhaoxin extended CPU features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0xc0000001_0 {
// eax
    pub Reserved: u32 : 32; //,
// ebx
    pub Reserved: u32 : 32; //,
// ecx
    pub Reserved: u32 : 32; //,
// edx
    pub Reserved: : 3; //,
}
