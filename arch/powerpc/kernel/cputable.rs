//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/cputable.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
//
// Modifications for ppc64:
// Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
//

    static struct cpu_spec the_cpu_spec __ro_after_init;
    let mut __ro_after_init: *mut cpu_spec cur_cpu_spec = core::ptr::null_mut();
    EXPORT_SYMBOL(cur_cpu_spec);
// The platform string corresponding to the real PVR
    const char *powerpc_base_platform;

#[no_mangle]
pub unsafe extern "C" fn set_cur_cpu_spec(s: *mut cpu_spec) -> void __init {
    void __init set_cur_cpu_spec(struct cpu_spec *s)
    {
    struct cpu_spec *t = &the_cpu_spec;
    t = PTRRELOC(t);
//
// use memcpy() instead of *t = *s so that GCC replaces it
// by __memcpy() when KASAN is active
//
    memcpy(t, s, sizeof(*t));
// PTRRELOC(&cur_cpu_spec) = &the_cpu_spec;
    }
    static struct cpu_spec * __init setup_cpu_spec(unsigned long offset,
    struct cpu_spec *s)
    {
    struct cpu_spec *t = &the_cpu_spec;
    struct cpu_spec old;
    t = PTRRELOC(t);
    old = *t;
//
// Copy everything, then do fixups. Use memcpy() instead of *t = *s
// so that GCC replaces it by __memcpy() when KASAN is active
//
    memcpy(t, s, sizeof(*t));
//
// If we are overriding a previous value derived from the real
// PVR with a new value obtained using a logical PVR value,
// don't modify the performance monitor fields.
//
    if (old.num_pmcs && !s.num_pmcs) {
    t.num_pmcs = old.num_pmcs;
    t.pmc_type = old.pmc_type;
//
// Let's ensure that the
// fix for the PMAO bug is enabled on compatibility mode.
//
    t.cpu_features |= old.cpu_features & CPU_FTR_PMAO_BUG;
    }
// Set kuap ON at startup, will be disabled later if cmdline has 'nosmap'
    if (IS_ENABLED(CONFIG_PPC_KUAP) && IS_ENABLED(CONFIG_PPC32))
    t.mmu_features |= MMU_FTR_KUAP;
// PTRRELOC(&cur_cpu_spec) = &the_cpu_spec;
//
// Set the base platform string once; assumes
// we're called with real pvr first.
//
    if (*PTRRELOC(&powerpc_base_platform) == core::ptr::null_mut())
// PTRRELOC(&powerpc_base_platform) = t->platform;

// ppc64 and booke expect identify_cpu to also call setup_cpu for
// that processor. I will consolidate that at a later time, for now,
// just use #ifdef. We also don't need to PTRRELOC the function
// pointer on ppc64 and booke as we are running at 0 in real mode
// on ppc64 and reloc_offset is always 0 on booke.
//
    if (t.cpu_setup) {
    t.cpu_setup(offset, t);
    }

    return t;
    }
#[no_mangle]
pub unsafe extern "C" fn identify_cpu(offset: c_ulong, pvr: c_uint) -> *mut cpu_spec  __init {
    struct cpu_spec * __init identify_cpu(unsigned long offset, unsigned int pvr)
    {
    struct cpu_spec *s = cpu_specs;
    int i;
    BUILD_BUG_ON(!ARRAY_SIZE(cpu_specs));
    s = PTRRELOC(s);
    for (i = 0; i < ARRAY_SIZE(cpu_specs); i++,s++) {
    if ((pvr & s.pvr_mask) == s.pvr_value)
    return setup_cpu_spec(offset, s);
    }
    BUG();
    return core::ptr::null_mut();
    }
//
// Used by cpufeatures to get the name for CPUs with a PVR table.
// If they don't hae a PVR table, cpufeatures gets the name from
// cpu device-tree node.
//
#[no_mangle]
pub unsafe extern "C" fn identify_cpu_name(pvr: c_uint) -> void __init {
    void __init identify_cpu_name(unsigned int pvr)
    {
    struct cpu_spec *s = cpu_specs;
    struct cpu_spec *t = &the_cpu_spec;
    int i;
    s = PTRRELOC(s);
    t = PTRRELOC(t);
    for (i = 0; i < ARRAY_SIZE(cpu_specs); i++,s++) {
    if ((pvr & s.pvr_mask) == s.pvr_value) {
    t.cpu_name = s.cpu_name;
    return;
    }
    }
    }

    struct static_key_true cpu_feature_keys[NUM_CPU_FTR_KEYS] = {
    [0 ... NUM_CPU_FTR_KEYS - 1] = STATIC_KEY_TRUE_INIT
    };
    EXPORT_SYMBOL_GPL(cpu_feature_keys);
#[no_mangle]
pub unsafe extern "C" fn cpu_feature_keys_init() -> void __init {
    void __init cpu_feature_keys_init(void)
    {
    int i;
    for (i = 0; i < NUM_CPU_FTR_KEYS; i++) {
    let mut f: c_ulong = 1ul << i;
    if (!(cur_cpu_spec.cpu_features & f))
    static_branch_disable(&cpu_feature_keys[i]);
    }
    }
    struct static_key_true mmu_feature_keys[NUM_MMU_FTR_KEYS] = {
    [0 ... NUM_MMU_FTR_KEYS - 1] = STATIC_KEY_TRUE_INIT
    };
    EXPORT_SYMBOL(mmu_feature_keys);
#[no_mangle]
pub unsafe extern "C" fn mmu_feature_keys_init() -> void __init {
    void __init mmu_feature_keys_init(void)
    {
    int i;
    for (i = 0; i < NUM_MMU_FTR_KEYS; i++) {
    let mut f: c_ulong = 1ul << i;
    if (!(cur_cpu_spec.mmu_features & f))
    static_branch_disable(&mmu_feature_keys[i]);
    }
    }
