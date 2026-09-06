//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/ifs/core.c
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
// Copyright(c) 2022 Intel Corporation.

    X86_MATCH_VFM_FEATURE(vfm, X86_FEATURE_CORE_CAPABILITIES, array_gen)
    static const struct x86_cpu_id ifs_cpu_ids[] __initconst = {
    X86_MATCH(INTEL_SAPPHIRERAPIDS_X, ARRAY_GEN0),
    X86_MATCH(INTEL_EMERALDRAPIDS_X, ARRAY_GEN0),
    X86_MATCH(INTEL_GRANITERAPIDS_X, ARRAY_GEN0),
    X86_MATCH(INTEL_GRANITERAPIDS_D, ARRAY_GEN0),
    X86_MATCH(INTEL_ATOM_CRESTMONT_X, ARRAY_GEN1),
    X86_MATCH(INTEL_ATOM_DARKMONT_X, ARRAY_GEN1),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, ifs_cpu_ids);
    ATTRIBUTE_GROUPS(plat_ifs);
    ATTRIBUTE_GROUPS(plat_ifs_array);
    bool *ifs_pkg_auth;
    static const struct ifs_test_caps scan_test = {
    .integrity_cap_bit = MSR_INTEGRITY_CAPS_PERIODIC_BIST_BIT,
    .test_num = IFS_TYPE_SAF,
    .image_suffix = "scan",
    };
    static const struct ifs_test_caps array_test = {
    .integrity_cap_bit = MSR_INTEGRITY_CAPS_ARRAY_BIST_BIT,
    .test_num = IFS_TYPE_ARRAY_BIST,
    };
    static const struct ifs_test_msrs scan_msrs = {
    .copy_hashes = MSR_COPY_SCAN_HASHES,
    .copy_hashes_status = MSR_SCAN_HASHES_STATUS,
    .copy_chunks = MSR_AUTHENTICATE_AND_COPY_CHUNK,
    .copy_chunks_status = MSR_CHUNKS_AUTHENTICATION_STATUS,
    .test_ctrl = MSR_SAF_CTRL,
    };
    static const struct ifs_test_msrs sbaf_msrs = {
    .copy_hashes = MSR_COPY_SBAF_HASHES,
    .copy_hashes_status = MSR_SBAF_HASHES_STATUS,
    .copy_chunks = MSR_AUTHENTICATE_AND_COPY_SBAF_CHUNK,
    .copy_chunks_status = MSR_SBAF_CHUNKS_AUTHENTICATION_STATUS,
    .test_ctrl = MSR_SBAF_CTRL,
    };
    static const struct ifs_test_caps sbaf_test = {
    .integrity_cap_bit = MSR_INTEGRITY_CAPS_SBAF_BIT,
    .test_num = IFS_TYPE_SBAF,
    .image_suffix = "sbft",
    };
    static struct ifs_device ifs_devices[] = {
    [IFS_TYPE_SAF] = {
    .test_caps = &scan_test,
    .test_msrs = &scan_msrs,
    .misc = {
    .name = "intel_ifs_0",
    .minor = MISC_DYNAMIC_MINOR,
    .groups = plat_ifs_groups,
    },
    },
    [IFS_TYPE_ARRAY_BIST] = {
    .test_caps = &array_test,
    .misc = {
    .name = "intel_ifs_1",
    .minor = MISC_DYNAMIC_MINOR,
    .groups = plat_ifs_array_groups,
    },
    },
    [IFS_TYPE_SBAF] = {
    .test_caps = &sbaf_test,
    .test_msrs = &sbaf_msrs,
    .misc = {
    .name = "intel_ifs_2",
    .minor = MISC_DYNAMIC_MINOR,
    .groups = plat_ifs_groups,
    },
    },
    };

#[no_mangle]
unsafe extern "C" fn ifs_cleanup() {
    static void ifs_cleanup(void)
    {
    int i;
    for (i = 0; i < IFS_NUMTESTS; i++) {
    if (ifs_devices[i].misc.this_device)
    misc_deregister(&ifs_devices[i].misc);
    }
    kfree(ifs_pkg_auth);
    }
#[no_mangle]
unsafe extern "C" fn ifs_init() -> int __init {
    static int __init ifs_init(void)
    {
    const struct x86_cpu_id *m;
    u64 msrval;
    int i, ret;
    m = x86_match_cpu(ifs_cpu_ids);
    if (!m)
    return -ENODEV;
    if (rdmsrq_safe(MSR_IA32_CORE_CAPS, &msrval))
    return -ENODEV;
    if (!(msrval & MSR_IA32_CORE_CAPS_INTEGRITY_CAPS))
    return -ENODEV;
    if (rdmsrq_safe(MSR_INTEGRITY_CAPS, &msrval))
    return -ENODEV;
    ifs_pkg_auth = kmalloc_objs(bool, topology_max_packages());
    if (!ifs_pkg_auth)
    return -ENOMEM;
    for (i = 0; i < IFS_NUMTESTS; i++) {
    if (!(msrval & BIT(ifs_devices[i].test_caps.integrity_cap_bit)))
    continue;
    ifs_devices[i].rw_data.generation = FIELD_GET(MSR_INTEGRITY_CAPS_SAF_GEN_MASK,
    msrval);
    ifs_devices[i].rw_data.array_gen = (u32)m.driver_data;
    ret = misc_register(&ifs_devices[i].misc);
    if (ret)
    goto err_exit;
    }
    return 0;
    err_exit:
    ifs_cleanup();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ifs_exit() -> void __exit {
    static void __exit ifs_exit(void)
    {
    ifs_cleanup();
    }
    module_init(ifs_init);
    module_exit(ifs_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Intel In Field Scan (IFS) device");
