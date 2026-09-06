//! Automatically rewritten from C to Rust
//! Source: arch/x86/coco/core.c
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
// Confidential Computing Platform Capability checks
//
// Copyright (C) 2021 Advanced Micro Devices, Inc.
// Copyright (C) 2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

    let mut __ro_after_init: enum cc_vendor cc_vendor = CC_VENDOR_NONE;
    SYM_PIC_ALIAS(cc_vendor);
    u64 cc_mask __ro_after_init;
    SYM_PIC_ALIAS(cc_mask);
    static struct cc_attr_flags {
    __u64 host_sev_snp	: 1,
    __resv		: 63;
    } cc_flags;
#[no_mangle]
unsafe extern "C" fn intel_cc_platform_has(attr: enum cc_attr) -> bool noinstr {
    static bool noinstr intel_cc_platform_has(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_GUEST_UNROLL_STRING_IO:
    case CC_ATTR_GUEST_MEM_ENCRYPT:
    case CC_ATTR_MEM_ENCRYPT:
    return true;
    default:
    return false;
    }
    }
//
// Handle the SEV-SNP vTOM case where sme_me_mask is zero, and
// the other levels of SME/SEV functionality, including C-bit
// based SEV-SNP, are not enabled.
//
#[no_mangle]
unsafe extern "C" fn amd_cc_platform_vtom(attr: enum cc_attr) -> __maybe_unused __always_inline bool {
    static __maybe_unused __always_inline bool amd_cc_platform_vtom(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_GUEST_MEM_ENCRYPT:
    case CC_ATTR_MEM_ENCRYPT:
    return true;
    default:
    return false;
    }
    }
//
// SME and SEV are very similar but they are not the same, so there are
// times that the kernel will need to distinguish between SME and SEV. The
// cc_platform_has() function is used for this.  When a distinction isn't
// needed, the CC_ATTR_MEM_ENCRYPT attribute can be used.
//
// The trampoline code is a good example for this requirement.  Before
// paging is activated, SME will access all memory as decrypted, but SEV
// will access all memory as encrypted.  So, when APs are being brought
// up under SME the trampoline area cannot be encrypted, whereas under SEV
// the trampoline area must be encrypted.
//
#[no_mangle]
unsafe extern "C" fn amd_cc_platform_has(attr: enum cc_attr) -> bool noinstr {
    static bool noinstr amd_cc_platform_has(enum cc_attr attr)
    {

    if (sev_status & MSR_AMD64_SNP_VTOM)
    return amd_cc_platform_vtom(attr);
    switch (attr) {
    case CC_ATTR_MEM_ENCRYPT:
    return sme_me_mask;
    case CC_ATTR_HOST_MEM_ENCRYPT:
    return sme_me_mask && !(sev_status & MSR_AMD64_SEV_ENABLED);
    case CC_ATTR_GUEST_MEM_ENCRYPT:
    return sev_status & MSR_AMD64_SEV_ENABLED;
    case CC_ATTR_GUEST_STATE_ENCRYPT:
    return sev_status & MSR_AMD64_SEV_ES_ENABLED;
//
// With SEV, the rep string I/O instructions need to be unrolled
// but SEV-ES supports them through the #VC handler.
//
    case CC_ATTR_GUEST_UNROLL_STRING_IO:
    return (sev_status & MSR_AMD64_SEV_ENABLED) &&
    !(sev_status & MSR_AMD64_SEV_ES_ENABLED);
    case CC_ATTR_GUEST_SEV_SNP:
    return sev_status & MSR_AMD64_SEV_SNP_ENABLED;
    case CC_ATTR_GUEST_SNP_SECURE_TSC:
    return sev_status & MSR_AMD64_SNP_SECURE_TSC;
    case CC_ATTR_HOST_SEV_SNP:
    return cc_flags.host_sev_snp;
    case CC_ATTR_SNP_SECURE_AVIC:
    return sev_status & MSR_AMD64_SNP_SECURE_AVIC;
    default:
    return false;
    }

    return false;

    }
#[no_mangle]
pub unsafe extern "C" fn cc_platform_has(attr: enum cc_attr) -> bool noinstr {
    bool noinstr cc_platform_has(enum cc_attr attr)
    {
    switch (cc_vendor) {
    case CC_VENDOR_AMD:
    return amd_cc_platform_has(attr);
    case CC_VENDOR_INTEL:
    return intel_cc_platform_has(attr);
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(cc_platform_has);
#[no_mangle]
pub unsafe extern "C" fn cc_mkenc(val: u64) -> u64 {
    u64 cc_mkenc(u64 val)
    {
//
// Both AMD and Intel use a bit in the page table to indicate
// encryption status of the page.
//
// - for AMD, bit *set* means the page is encrypted
// - for AMD with vTOM and for Intel, *clear* means encrypted
//
    switch (cc_vendor) {
    case CC_VENDOR_AMD:
    if (sev_status & MSR_AMD64_SNP_VTOM)
    return val & ~cc_mask;
    else
    return val | cc_mask;
    case CC_VENDOR_INTEL:
    return val & ~cc_mask;
    default:
    return val;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cc_mkdec(val: u64) -> u64 {
    u64 cc_mkdec(u64 val)
    {
// See comment in cc_mkenc()
    switch (cc_vendor) {
    case CC_VENDOR_AMD:
    if (sev_status & MSR_AMD64_SNP_VTOM)
    return val | cc_mask;
    else
    return val & ~cc_mask;
    case CC_VENDOR_INTEL:
    return val | cc_mask;
    default:
    return val;
    }
    }
    EXPORT_SYMBOL_GPL(cc_mkdec);
#[no_mangle]
unsafe extern "C" fn amd_cc_platform_clear(attr: enum cc_attr) {
    static void amd_cc_platform_clear(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_HOST_SEV_SNP:
    cc_flags.host_sev_snp = 0;
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cc_platform_clear(attr: enum cc_attr) {
    void cc_platform_clear(enum cc_attr attr)
    {
    switch (cc_vendor) {
    case CC_VENDOR_AMD:
    amd_cc_platform_clear(attr);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_cc_platform_set(attr: enum cc_attr) {
    static void amd_cc_platform_set(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_HOST_SEV_SNP:
    cc_flags.host_sev_snp = 1;
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cc_platform_set(attr: enum cc_attr) {
    void cc_platform_set(enum cc_attr attr)
    {
    switch (cc_vendor) {
    case CC_VENDOR_AMD:
    amd_cc_platform_set(attr);
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cc_random_init() -> __init void {
    __init void cc_random_init(void)
    {
//
// The seed is 32 bytes (in units of longs), which is 256 bits, which
// is the security level that the RNG is targeting.
//
    unsigned long rng_seed[32 / sizeof(long)];
    size_t i, longs;
    if (!cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    return;
//
// Since the CoCo threat model includes the host, the only reliable
// source of entropy that can be neither observed nor manipulated is
// RDRAND. Usually, RDRAND failure is considered tolerable, but since
// CoCo guests have no other unobservable source of entropy, it's
// important to at least ensure the RNG gets some initial random seeds.
//
    for (i = 0; i < ARRAY_SIZE(rng_seed); i += longs) {
    longs = arch_get_random_longs(&rng_seed[i], ARRAY_SIZE(rng_seed) - i);
//
// A zero return value means that the guest doesn't have RDRAND
// or the CPU is physically broken, and in both cases that
// means most crypto inside of the CoCo instance will be
// broken, defeating the purpose of CoCo in the first place. So
// just panic here because it's absolutely unsafe to continue
// executing.
//
    if (longs == 0)
    panic("RDRAND is defective.");
    }
    add_device_randomness(rng_seed, sizeof(rng_seed));
    memzero_explicit(rng_seed, sizeof(rng_seed));
    }
