//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/cpacf.c
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
// Copyright IBM Corp. 2024
//

    static ssize_t name##_query_raw_read(struct file *fp,				\
    struct kobject *kobj,			\
    const struct bin_attribute *attr,		\
    char *buf, loff_t offs,			\
    size_t count)				\
    {										\
    cpacf_mask_t mask;							\
    \
    if (!cpacf_query(CPACF_##instruction, &mask))				\
    return -EOPNOTSUPP;						\
    return memory_read_from_buffer(buf, count, &offs, &mask, sizeof(mask));	\
    }										\
#[no_mangle]
unsafe extern "C" fn BIN_ATTR_RO(_arg: name##_query_raw, _arg: sizeof(cpacf_mask_t)) -> const {
    static const BIN_ATTR_RO(name##_query_raw, sizeof(cpacf_mask_t))
    CPACF_QUERY(km, KM);
    CPACF_QUERY(kmc, KMC);
    CPACF_QUERY(kimd, KIMD);
    CPACF_QUERY(klmd, KLMD);
    CPACF_QUERY(kmac, KMAC);
    CPACF_QUERY(pckmo, PCKMO);
    CPACF_QUERY(kmf, KMF);
    CPACF_QUERY(kmctr, KMCTR);
    CPACF_QUERY(kmo, KMO);
    CPACF_QUERY(pcc, PCC);
    CPACF_QUERY(prno, PRNO);
    CPACF_QUERY(kma, KMA);
    CPACF_QUERY(kdsa, KDSA);

    static ssize_t name##_query_auth_info_raw_read(				\
    struct file *fp, struct kobject *kobj,				\
    const struct bin_attribute *attr, char *buf, loff_t offs,	\
    size_t count)							\
    {									\
    cpacf_qai_t qai;						\
    \
    if (!cpacf_qai(CPACF_##instruction, &qai))			\
    return -EOPNOTSUPP;					\
    return memory_read_from_buffer(buf, count, &offs, &qai,		\
    sizeof(qai));			\
    }									\
#[no_mangle]
unsafe extern "C" fn BIN_ATTR_RO(_arg: name##_query_auth_info_raw, _arg: sizeof(cpacf_qai_t)) -> const {
    static const BIN_ATTR_RO(name##_query_auth_info_raw, sizeof(cpacf_qai_t))
    CPACF_QAI(km, KM);
    CPACF_QAI(kmc, KMC);
    CPACF_QAI(kimd, KIMD);
    CPACF_QAI(klmd, KLMD);
    CPACF_QAI(kmac, KMAC);
    CPACF_QAI(pckmo, PCKMO);
    CPACF_QAI(kmf, KMF);
    CPACF_QAI(kmctr, KMCTR);
    CPACF_QAI(kmo, KMO);
    CPACF_QAI(pcc, PCC);
    CPACF_QAI(prno, PRNO);
    CPACF_QAI(kma, KMA);
    CPACF_QAI(kdsa, KDSA);
    static const struct bin_attribute *const cpacf_attrs[] = {
    &bin_attr_km_query_raw,
    &bin_attr_kmc_query_raw,
    &bin_attr_kimd_query_raw,
    &bin_attr_klmd_query_raw,
    &bin_attr_kmac_query_raw,
    &bin_attr_pckmo_query_raw,
    &bin_attr_kmf_query_raw,
    &bin_attr_kmctr_query_raw,
    &bin_attr_kmo_query_raw,
    &bin_attr_pcc_query_raw,
    &bin_attr_prno_query_raw,
    &bin_attr_kma_query_raw,
    &bin_attr_kdsa_query_raw,
    &bin_attr_km_query_auth_info_raw,
    &bin_attr_kmc_query_auth_info_raw,
    &bin_attr_kimd_query_auth_info_raw,
    &bin_attr_klmd_query_auth_info_raw,
    &bin_attr_kmac_query_auth_info_raw,
    &bin_attr_pckmo_query_auth_info_raw,
    &bin_attr_kmf_query_auth_info_raw,
    &bin_attr_kmctr_query_auth_info_raw,
    &bin_attr_kmo_query_auth_info_raw,
    &bin_attr_pcc_query_auth_info_raw,
    &bin_attr_prno_query_auth_info_raw,
    &bin_attr_kma_query_auth_info_raw,
    &bin_attr_kdsa_query_auth_info_raw,
    core::ptr::null_mut(),
    };
    static const struct attribute_group cpacf_attr_grp = {
    .name = "cpacf",
    .bin_attrs = cpacf_attrs,
    };
#[no_mangle]
unsafe extern "C" fn cpacf_init() -> int __init {
    static int __init cpacf_init(void)
    {
    struct device *cpu_root;
    let mut rc: c_int = 0;
    cpu_root = bus_get_dev_root(&cpu_subsys);
    if (cpu_root) {
    rc = sysfs_create_group(&cpu_root.kobj, &cpacf_attr_grp);
    put_device(cpu_root);
    }
    return rc;
    }
    device_initcall(cpacf_init);
