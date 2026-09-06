//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/cpufeature.c
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
// Copyright IBM Corp. 2022
//

    enum {
    TYPE_HWCAP,
    TYPE_FACILITY,
    TYPE_MACHINE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_cpu_feature {
    pub 4: unsigned int type :,
    pub 28: unsigned int num :,
}

    static struct s390_cpu_feature s390_cpu_features[MAX_CPU_FEATURES] = {
    [S390_CPU_FEATURE_MSA]	= {.type = TYPE_HWCAP, .num = HWCAP_NR_MSA},
    [S390_CPU_FEATURE_VXRS]	= {.type = TYPE_HWCAP, .num = HWCAP_NR_VXRS},
    [S390_CPU_FEATURE_UV]	= {.type = TYPE_FACILITY, .num = 158},
    [S390_CPU_FEATURE_D288]	= {.type = TYPE_MACHINE, .num = MFEATURE_DIAG288},
    };
//
// cpu_have_feature - Test CPU features on module initialization
//
#[no_mangle]
pub unsafe extern "C" fn cpu_have_feature(num: c_uint) -> c_int {
    int cpu_have_feature(unsigned int num)
    {
    struct s390_cpu_feature *feature;
    if (WARN_ON_ONCE(num >= MAX_CPU_FEATURES))
    return 0;
    feature = &s390_cpu_features[num];
    switch (feature.type) {
    case TYPE_HWCAP:
    return !!(elf_hwcap & BIT(feature.num));
    case TYPE_FACILITY:
    return test_facility(feature.num);
    case TYPE_MACHINE:
    return test_machine_feature(feature.num);
    default:
    WARN_ON_ONCE(1);
    return 0;
    }
    }
    EXPORT_SYMBOL(cpu_have_feature);
