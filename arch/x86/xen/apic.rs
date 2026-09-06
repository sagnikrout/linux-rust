//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/apic.c
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

#[no_mangle]
unsafe extern "C" fn xen_io_apic_read(apic: unsigned, reg: unsigned) -> c_uint {
    static unsigned int xen_io_apic_read(unsigned apic, unsigned reg)
    {
    struct physdev_apic apic_op;
    int ret;
    apic_op.apic_physbase = mpc_ioapic_addr(apic);
    apic_op.reg = reg;
    ret = HYPERVISOR_physdev_op(PHYSDEVOP_apic_read, &apic_op);
    if (!ret)
    return apic_op.value;
// fallback to return an emulated IO_APIC values
    if (reg == 0x1)
    return 0x00170020;
#[no_mangle]
pub unsafe extern "C" fn if(0x0: reg ==) -> else {
    else if (reg == 0x0)
    return apic << 24;
    return 0xfd;
    }
#[no_mangle]
unsafe extern "C" fn xen_get_apic_id(x: u32) -> u32 {
    static u32 xen_get_apic_id(u32 x)
    {
    return ((x)>>24) & 0xFFu;
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_read(reg: u32) -> u32 {
    static u32 xen_apic_read(u32 reg)
    {
    struct xen_platform_op op = {
    .cmd = XENPF_get_cpuinfo,
    .interface_version = XENPF_INTERFACE_VERSION,
    };
    int ret, cpu;
    if (reg == APIC_LVR)
    return 0x14;
    if (reg != APIC_ID)
    return 0;
    cpu = smp_processor_id();
    if (!xen_initial_domain())
    return cpu ? cpuid_to_apicid[cpu] << 24 : 0;
    op.u.pcpu_info.xen_cpuid = cpu;
    ret = HYPERVISOR_platform_op(&op);
    if (ret)
    op.u.pcpu_info.apic_id = BAD_APICID;
    return op.u.pcpu_info.apic_id << 24;
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_write(reg: u32, val: u32) {
    static void xen_apic_write(u32 reg, u32 val)
    {
    if (reg == APIC_LVTPC) {
    (void)pmu_apic_update(reg);
    return;
    }
// Warn to see if there's any stray references
    WARN(1,"register: %x, value: %x\n", reg, val);
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_eoi() {
    static void xen_apic_eoi(void)
    {
    WARN_ON_ONCE(1);
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_icr_read() -> u64 {
    static u64 xen_apic_icr_read(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_icr_write(low: u32, id: u32) {
    static void xen_apic_icr_write(u32 low, u32 id)
    {
// Warn to see if there's any stray references
    WARN_ON(1);
    }
#[no_mangle]
unsafe extern "C" fn xen_apic_probe_pv() -> c_int {
    static int xen_apic_probe_pv(void)
    {
    if (xen_pv_domain())
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int xen_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    return xen_pv_domain();
    }
#[no_mangle]
unsafe extern "C" fn xen_cpu_present_to_apicid(cpu: c_int) -> u32 {
    static u32 xen_cpu_present_to_apicid(int cpu)
    {
    if (cpu_present(cpu))
    return cpu_data(cpu).topo.apicid;
    else
    return BAD_APICID;
    }
    static struct apic xen_pv_apic __ro_after_init = {
    .name				= "Xen PV",
    .probe				= xen_apic_probe_pv,
    .acpi_madt_oem_check		= xen_madt_oem_check,
// .delivery_mode and .dest_mode_logical not used by XENPV
    .disable_esr			= 0,
    .cpu_present_to_apicid		= xen_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .get_apic_id			= xen_get_apic_id,
    .calc_dest_apicid		= apic_flat_calc_apicid,

    .send_IPI_mask			= xen_send_IPI_mask,
    .send_IPI_mask_allbutself	= xen_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= xen_send_IPI_allbutself,
    .send_IPI_all			= xen_send_IPI_all,
    .send_IPI_self			= xen_send_IPI_self,

    .read				= xen_apic_read,
    .write				= xen_apic_write,
    .eoi				= xen_apic_eoi,
    .icr_read			= xen_apic_icr_read,
    .icr_write			= xen_apic_icr_write,
    };
    apic_driver(xen_pv_apic);
#[no_mangle]
pub unsafe extern "C" fn xen_init_apic() -> void __init {
    void __init xen_init_apic(void)
    {
    x86_apic_ops.io_apic_read = xen_io_apic_read;
    }
