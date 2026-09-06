//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/riscv/cppc.c
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
// Implement CPPC FFH helper routines for RISC-V.
//
// Copyright (C) 2024 Ventana Micro Systems Inc.
//

pub const SBI_EXT_CPPC: c_uint = 0x43505043;
// CPPC interfaces defined in SBI spec
pub const SBI_CPPC_PROBE: c_uint = 0x0;
pub const SBI_CPPC_READ: c_uint = 0x1;
pub const SBI_CPPC_READ_HI: c_uint = 0x2;
pub const SBI_CPPC_WRITE: c_uint = 0x3;
// RISC-V FFH definitions from RISC-V FFH spec

pub const FFH_CPPC_SBI: c_uint = 0x1;
pub const FFH_CPPC_CSR: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_cppc_data {
    pub val: u64,
    pub reg: u32,
    pub ret: sbiret,
}

    static bool cppc_ext_present;
#[no_mangle]
unsafe extern "C" fn sbi_cppc_init() -> int __init {
    static int __init sbi_cppc_init(void)
    {
    if (sbi_spec_version >= sbi_mk_version(2, 0) &&
    sbi_probe_extension(SBI_EXT_CPPC) > 0) {
    cppc_ext_present = true;
    } else {
    cppc_ext_present = false;
    }
    return 0;
    }
    device_initcall(sbi_cppc_init);
#[no_mangle]
unsafe extern "C" fn sbi_cppc_read(read_data: *mut c_void) {
    static void sbi_cppc_read(void *read_data)
    {
    struct sbi_cppc_data *data = (struct sbi_cppc_data *)read_data;
    data.ret = sbi_ecall(SBI_EXT_CPPC, SBI_CPPC_READ,
    data.reg, 0, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn sbi_cppc_write(write_data: *mut c_void) {
    static void sbi_cppc_write(void *write_data)
    {
    struct sbi_cppc_data *data = (struct sbi_cppc_data *)write_data;
    data.ret = sbi_ecall(SBI_EXT_CPPC, SBI_CPPC_WRITE,
    data.reg, data.val, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn cppc_ffh_csr_read(read_data: *mut c_void) {
    static void cppc_ffh_csr_read(void *read_data)
    {
    struct sbi_cppc_data *data = (struct sbi_cppc_data *)read_data;
    switch (data.reg) {
// Support only TIME CSR for now
    case CSR_TIME:
    data.ret.value = csr_read(CSR_TIME);
    data.ret.error = 0;
    break;
    default:
    data.ret.error = -EINVAL;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn cppc_ffh_csr_write(write_data: *mut c_void) {
    static void cppc_ffh_csr_write(void *write_data)
    {
    struct sbi_cppc_data *data = (struct sbi_cppc_data *)write_data;
    data.ret.error = -EINVAL;
    }
//
// Refer to drivers/acpi/cppc_acpi.c for the description of the functions
// below.
//
#[no_mangle]
pub unsafe extern "C" fn cpc_ffh_supported() -> bool {
    bool cpc_ffh_supported(void)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn cpc_read_ffh(cpu: c_int, reg: *mut cpc_reg, val: *mut u64) -> c_int {
    int cpc_read_ffh(int cpu, struct cpc_reg *reg, u64 *val)
    {
    struct sbi_cppc_data data;
    if (WARN_ON_ONCE(irqs_disabled()))
    return -EPERM;
    if (FFH_CPPC_TYPE(reg.address) == FFH_CPPC_SBI) {
    if (!cppc_ext_present)
    return -EINVAL;
    data.reg = FFH_CPPC_SBI_REG(reg.address);
    smp_call_function_single(cpu, sbi_cppc_read, &data, 1);
// val = data.ret.value;
    return (data.ret.error) ? sbi_err_map_linux_errno(data.ret.error) : 0;
    } else if (FFH_CPPC_TYPE(reg.address) == FFH_CPPC_CSR) {
    data.reg = FFH_CPPC_CSR_NUM(reg.address);
    smp_call_function_single(cpu, cppc_ffh_csr_read, &data, 1);
// val = data.ret.value;
    return data.ret.error;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn cpc_write_ffh(cpu: c_int, reg: *mut cpc_reg, val: u64) -> c_int {
    int cpc_write_ffh(int cpu, struct cpc_reg *reg, u64 val)
    {
    struct sbi_cppc_data data;
    if (WARN_ON_ONCE(irqs_disabled()))
    return -EPERM;
    if (FFH_CPPC_TYPE(reg.address) == FFH_CPPC_SBI) {
    if (!cppc_ext_present)
    return -EINVAL;
    data.reg = FFH_CPPC_SBI_REG(reg.address);
    data.val = val;
    smp_call_function_single(cpu, sbi_cppc_write, &data, 1);
    return (data.ret.error) ? sbi_err_map_linux_errno(data.ret.error) : 0;
    } else if (FFH_CPPC_TYPE(reg.address) == FFH_CPPC_CSR) {
    data.reg = FFH_CPPC_CSR_NUM(reg.address);
    data.val = val;
    smp_call_function_single(cpu, cppc_ffh_csr_write, &data, 1);
    return data.ret.error;
    }
    return -EINVAL;
    }
