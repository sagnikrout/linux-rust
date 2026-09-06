//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/pcc-cpufreq.c
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
// pcc-cpufreq.c - Processor Clocking Control firmware cpufreq interface
//
// Copyright (C) 2009 Red Hat, Matthew Garrett <mjg@redhat.com>
// Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
// Nagananda Chumbalkar <nagananda.chumbalkar@hp.com>
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or NON
// INFRINGEMENT. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

pub const POLL_LOOPS: c_int = 300;
pub const CMD_COMPLETE: c_uint = 0x1;
pub const CMD_GET_FREQ: c_uint = 0x0;
pub const CMD_SET_FREQ: c_uint = 0x1;
pub const BUF_SZ: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_register_resource {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_memory_resource {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub resource_usage: u8,
    pub type_specific: u8,
    pub granularity: u64,
    pub minimum: u64,
    pub maximum: u64,
    pub translation_offset: u64,
    pub address_length: u64,
// C attribute field omitted
    pub pcc_cpufreq_driver: static struct cpufreq_driver,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_header {
    pub signature: u32,
    pub length: u16,
    pub major: u8,
    pub minor: u8,
    pub features: u32,
    pub command: u16,
    pub status: u16,
    pub latency: u32,
    pub minimum_time: u32,
    pub maximum_time: u32,
    pub nominal: u32,
    pub throttled_frequency: u32,
    pub minimum_frequency: u32,
}

    static void __iomem *pcch_virt_addr;
    static struct pcc_header __iomem *pcch_hdr;
    static DEFINE_SPINLOCK(pcc_lock);
    static struct acpi_generic_address doorbell;
    static u64 doorbell_preserve;
    static u64 doorbell_write;
    static u8 OSC_UUID[16] = {0x9F, 0x2C, 0x9B, 0x63, 0x91, 0x70, 0x1f, 0x49,
    0xBB, 0x4F, 0xA5, 0x98, 0x2F, 0xA1, 0xB5, 0x46};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_cpu {
    pub input_offset: u32,
    pub output_offset: u32,
}

    static struct pcc_cpu __percpu *pcc_cpu_info;
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_verify(policy: *mut cpufreq_policy_data) -> c_int {
    static int pcc_cpufreq_verify(struct cpufreq_policy_data *policy)
    {
    cpufreq_verify_within_cpu_limits(policy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pcc_cmd() {
    static inline void pcc_cmd(void)
    {
    u64 doorbell_value;
    int i;
    acpi_read(&doorbell_value, &doorbell);
    acpi_write((doorbell_value & doorbell_preserve) | doorbell_write,
    &doorbell);
    for (i = 0; i < POLL_LOOPS; i++) {
    if (ioread16(&pcch_hdr.status) & CMD_COMPLETE)
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pcc_clear_mapping() {
    static inline void pcc_clear_mapping(void)
    {
    if (pcch_virt_addr)
    iounmap(pcch_virt_addr);
    pcch_virt_addr = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pcc_get_freq(cpu: c_uint) -> c_uint {
    static unsigned int pcc_get_freq(unsigned int cpu)
    {
    struct pcc_cpu *pcc_cpu_data;
    unsigned int curr_freq;
    unsigned int freq_limit;
    u16 status;
    u32 input_buffer;
    u32 output_buffer;
    spin_lock(&pcc_lock);
    pr_debug("get: get_freq for CPU %d\n", cpu);
    pcc_cpu_data = per_cpu_ptr(pcc_cpu_info, cpu);
    input_buffer = 0x1;
    iowrite32(input_buffer,
    (pcch_virt_addr + pcc_cpu_data.input_offset));
    iowrite16(CMD_GET_FREQ, &pcch_hdr.command);
    pcc_cmd();
    output_buffer =
    ioread32(pcch_virt_addr + pcc_cpu_data.output_offset);
// Clear the input buffer - we are done with the current command
    memset_io((pcch_virt_addr + pcc_cpu_data.input_offset), 0, BUF_SZ);
    status = ioread16(&pcch_hdr.status);
    if (status != CMD_COMPLETE) {
    pr_debug("get: FAILED: for CPU %d, status is %d\n",
    cpu, status);
    goto cmd_incomplete;
    }
    iowrite16(0, &pcch_hdr.status);
    curr_freq = (((ioread32(&pcch_hdr.nominal) * (output_buffer & 0xff))
    / 100) * 1000);
    pr_debug("get: SUCCESS: (virtual) output_offset for cpu %d is "
    "0x%p, contains a value of: 0x%x. Speed is: %d MHz\n",
    cpu, (pcch_virt_addr + pcc_cpu_data.output_offset),
    output_buffer, curr_freq);
    freq_limit = (output_buffer >> 8) & 0xff;
    if (freq_limit != 0xff) {
    pr_debug("get: frequency for cpu %d is being temporarily"
    " capped at %d\n", cpu, curr_freq);
    }
    spin_unlock(&pcc_lock);
    return curr_freq;
    cmd_incomplete:
    iowrite16(0, &pcch_hdr.status);
    spin_unlock(&pcc_lock);
    return 0;
    }
    static int pcc_cpufreq_target(struct cpufreq_policy *policy,
    unsigned int target_freq,
    unsigned int relation)
    {
    struct pcc_cpu *pcc_cpu_data;
    struct cpufreq_freqs freqs;
    u16 status;
    u32 input_buffer;
    int cpu;
    cpu = policy.cpu;
    pcc_cpu_data = per_cpu_ptr(pcc_cpu_info, cpu);
    pr_debug("target: CPU %d should go to target freq: %d "
    "(virtual) input_offset is 0x%p\n",
    cpu, target_freq,
    (pcch_virt_addr + pcc_cpu_data.input_offset));
    freqs.old = policy.cur;
    freqs.new = target_freq;
    cpufreq_freq_transition_begin(policy, &freqs);
    spin_lock(&pcc_lock);
    input_buffer = 0x1 | (((target_freq * 100)
    / (ioread32(&pcch_hdr.nominal) * 1000)) << 8);
    iowrite32(input_buffer,
    (pcch_virt_addr + pcc_cpu_data.input_offset));
    iowrite16(CMD_SET_FREQ, &pcch_hdr.command);
    pcc_cmd();
// Clear the input buffer - we are done with the current command
    memset_io((pcch_virt_addr + pcc_cpu_data.input_offset), 0, BUF_SZ);
    status = ioread16(&pcch_hdr.status);
    iowrite16(0, &pcch_hdr.status);
    spin_unlock(&pcc_lock);
    cpufreq_freq_transition_end(policy, &freqs, status != CMD_COMPLETE);
    if (status != CMD_COMPLETE) {
    pr_debug("target: FAILED for cpu %d, with status: 0x%x\n",
    cpu, status);
    return -EINVAL;
    }
    pr_debug("target: was SUCCESSFUL for cpu %d\n", cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcc_get_offset(cpu: c_int) -> c_int {
    static int pcc_get_offset(int cpu)
    {
    acpi_status status;
    let mut buffer: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    union acpi_object *pccp, *offset;
    struct pcc_cpu *pcc_cpu_data;
    struct acpi_processor *pr;
    let mut ret: c_int = 0;
    pr = per_cpu(processors, cpu);
    pcc_cpu_data = per_cpu_ptr(pcc_cpu_info, cpu);
    if (!pr)
    return -ENODEV;
    status = acpi_evaluate_object(pr.handle, "PCCP", core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    pccp = buffer.pointer;
    if (!pccp || pccp.type != ACPI_TYPE_PACKAGE) {
    ret = -ENODEV;
    goto out_free;
    }
    offset = &(pccp.package.elements[0]);
    if (!offset || offset.type != ACPI_TYPE_INTEGER) {
    ret = -ENODEV;
    goto out_free;
    }
    pcc_cpu_data.input_offset = offset.integer.value;
    offset = &(pccp.package.elements[1]);
    if (!offset || offset.type != ACPI_TYPE_INTEGER) {
    ret = -ENODEV;
    goto out_free;
    }
    pcc_cpu_data.output_offset = offset.integer.value;
    memset_io((pcch_virt_addr + pcc_cpu_data.input_offset), 0, BUF_SZ);
    memset_io((pcch_virt_addr + pcc_cpu_data.output_offset), 0, BUF_SZ);
    pr_debug("pcc_get_offset: for CPU %d: pcc_cpu_data "
    "input_offset: 0x%x, pcc_cpu_data output_offset: 0x%x\n",
    cpu, pcc_cpu_data.input_offset, pcc_cpu_data.output_offset);
    out_free:
    kfree(buffer.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_do_osc(handle: *mut acpi_handle) -> int __init {
    static int __init pcc_cpufreq_do_osc(acpi_handle *handle)
    {
    acpi_status status;
    struct acpi_object_list input;
    let mut output: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    union acpi_object in_params[4];
    union acpi_object *out_obj;
    u32 capabilities[2];
    u32 errors;
    u32 supported;
    let mut ret: c_int = 0;
    input.count = 4;
    input.pointer = in_params;
    in_params[0].type               = ACPI_TYPE_BUFFER;
    in_params[0].buffer.length      = 16;
    in_params[0].buffer.pointer     = OSC_UUID;
    in_params[1].type               = ACPI_TYPE_INTEGER;
    in_params[1].integer.value      = 1;
    in_params[2].type               = ACPI_TYPE_INTEGER;
    in_params[2].integer.value      = 2;
    in_params[3].type               = ACPI_TYPE_BUFFER;
    in_params[3].buffer.length      = 8;
    in_params[3].buffer.pointer     = (u8 *)&capabilities;
    capabilities[0] = OSC_QUERY_ENABLE;
    capabilities[1] = 0x1;
    status = acpi_evaluate_object(*handle, "_OSC", &input, &output);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    if (!output.length)
    return -ENODEV;
    out_obj = output.pointer;
    if (out_obj.type != ACPI_TYPE_BUFFER) {
    ret = -ENODEV;
    goto out_free;
    }
    errors = *((u32 *)out_obj.buffer.pointer) & ~(1 << 0);
    if (errors) {
    ret = -ENODEV;
    goto out_free;
    }
    supported = *((u32 *)(out_obj.buffer.pointer + 4));
    if (!(supported & 0x1)) {
    ret = -ENODEV;
    goto out_free;
    }
    kfree(output.pointer);
    output.pointer = core::ptr::null_mut();
    output.length = ACPI_ALLOCATE_BUFFER;
    capabilities[0] = 0x0;
    capabilities[1] = 0x1;
    status = acpi_evaluate_object(*handle, "_OSC", &input, &output);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    if (!output.length)
    return -ENODEV;
    out_obj = output.pointer;
    if (out_obj.type != ACPI_TYPE_BUFFER) {
    ret = -ENODEV;
    goto out_free;
    }
    errors = *((u32 *)out_obj.buffer.pointer) & ~(1 << 0);
    if (errors) {
    ret = -ENODEV;
    goto out_free;
    }
    supported = *((u32 *)(out_obj.buffer.pointer + 4));
    if (!(supported & 0x1)) {
    ret = -ENODEV;
    goto out_free;
    }
    out_free:
    kfree(output.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_evaluate() -> int __init {
    static int __init pcc_cpufreq_evaluate(void)
    {
    acpi_status status;
    let mut output: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    struct pcc_memory_resource *mem_resource;
    struct pcc_register_resource *reg_resource;
    union acpi_object *out_obj, *member;
    acpi_handle handle, osc_handle;
    let mut ret: c_int = 0;
    status = acpi_get_handle(core::ptr::null_mut(), "\\_SB", &handle);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    if (!acpi_has_method(handle, "PCCH"))
    return -ENODEV;
    status = acpi_get_handle(handle, "_OSC", &osc_handle);
    if (ACPI_SUCCESS(status)) {
    ret = pcc_cpufreq_do_osc(&osc_handle);
    if (ret)
    pr_debug("probe: _OSC evaluation did not succeed\n");
// Firmware's use of _OSC is optional
    ret = 0;
    }
    status = acpi_evaluate_object(handle, "PCCH", core::ptr::null_mut(), &output);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    out_obj = output.pointer;
    if (out_obj.type != ACPI_TYPE_PACKAGE) {
    ret = -ENODEV;
    goto out_free;
    }
    member = &out_obj.package.elements[0];
    if (member.type != ACPI_TYPE_BUFFER) {
    ret = -ENODEV;
    goto out_free;
    }
    mem_resource = (struct pcc_memory_resource *)member.buffer.pointer;
    pr_debug("probe: mem_resource descriptor: 0x%x,"
    " length: %d, space_id: %d, resource_usage: %d,"
    " type_specific: %d, granularity: 0x%llx,"
    " minimum: 0x%llx, maximum: 0x%llx,"
    " translation_offset: 0x%llx, address_length: 0x%llx\n",
    mem_resource.descriptor, mem_resource.length,
    mem_resource.space_id, mem_resource.resource_usage,
    mem_resource.type_specific, mem_resource.granularity,
    mem_resource.minimum, mem_resource.maximum,
    mem_resource.translation_offset,
    mem_resource.address_length);
    if (mem_resource.space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY) {
    ret = -ENODEV;
    goto out_free;
    }
    pcch_virt_addr = ioremap(mem_resource.minimum,
    mem_resource.address_length);
    if (pcch_virt_addr == core::ptr::null_mut()) {
    pr_debug("probe: could not map shared mem region\n");
    ret = -ENOMEM;
    goto out_free;
    }
    pcch_hdr = pcch_virt_addr;
    pr_debug("probe: PCCH header (virtual) addr: 0x%p\n", pcch_hdr);
    pr_debug("probe: PCCH header is at physical address: 0x%llx,"
    " signature: 0x%x, length: %d bytes, major: %d, minor: %d,"
    " supported features: 0x%x, command field: 0x%x,"
    " status field: 0x%x, nominal latency: %d us\n",
    mem_resource.minimum, ioread32(&pcch_hdr.signature),
    ioread16(&pcch_hdr.length), ioread8(&pcch_hdr.major),
    ioread8(&pcch_hdr.minor), ioread32(&pcch_hdr.features),
    ioread16(&pcch_hdr.command), ioread16(&pcch_hdr.status),
    ioread32(&pcch_hdr.latency));
    pr_debug("probe: min time between commands: %d us,"
    " max time between commands: %d us,"
    " nominal CPU frequency: %d MHz,"
    " minimum CPU frequency: %d MHz,"
    " minimum CPU frequency without throttling: %d MHz\n",
    ioread32(&pcch_hdr.minimum_time),
    ioread32(&pcch_hdr.maximum_time),
    ioread32(&pcch_hdr.nominal),
    ioread32(&pcch_hdr.throttled_frequency),
    ioread32(&pcch_hdr.minimum_frequency));
    member = &out_obj.package.elements[1];
    if (member.type != ACPI_TYPE_BUFFER) {
    ret = -ENODEV;
    goto pcch_free;
    }
    reg_resource = (struct pcc_register_resource *)member.buffer.pointer;
    doorbell.space_id = reg_resource.space_id;
    doorbell.bit_width = reg_resource.bit_width;
    doorbell.bit_offset = reg_resource.bit_offset;
    doorbell.access_width = 4;
    doorbell.address = reg_resource.address;
    pr_debug("probe: doorbell: space_id is %d, bit_width is %d, "
    "bit_offset is %d, access_width is %d, address is 0x%llx\n",
    doorbell.space_id, doorbell.bit_width, doorbell.bit_offset,
    doorbell.access_width, reg_resource.address);
    member = &out_obj.package.elements[2];
    if (member.type != ACPI_TYPE_INTEGER) {
    ret = -ENODEV;
    goto pcch_free;
    }
    doorbell_preserve = member.integer.value;
    member = &out_obj.package.elements[3];
    if (member.type != ACPI_TYPE_INTEGER) {
    ret = -ENODEV;
    goto pcch_free;
    }
    doorbell_write = member.integer.value;
    pr_debug("probe: doorbell_preserve: 0x%llx,"
    " doorbell_write: 0x%llx\n",
    doorbell_preserve, doorbell_write);
    pcc_cpu_info = alloc_percpu(struct pcc_cpu);
    if (!pcc_cpu_info) {
    ret = -ENOMEM;
    goto pcch_free;
    }
    printk(KERN_DEBUG "pcc-cpufreq: (v%s) driver loaded with frequency"
    " limits: %d MHz, %d MHz\n", PCC_VERSION,
    ioread32(&pcch_hdr.minimum_frequency),
    ioread32(&pcch_hdr.nominal));
    kfree(output.pointer);
    return ret;
    pcch_free:
    pcc_clear_mapping();
    out_free:
    kfree(output.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int pcc_cpufreq_cpu_init(struct cpufreq_policy *policy)
    {
    let mut cpu: c_uint = policy.cpu;
    let mut result: c_uint = 0;
    if (!pcch_virt_addr) {
    result = -1;
    goto out;
    }
    result = pcc_get_offset(cpu);
    if (result) {
    pr_debug("init: PCCP evaluation failed\n");
    goto out;
    }
    policy.cpuinfo.max_freq = ioread32(&pcch_hdr.nominal) * 1000;
    policy.cpuinfo.min_freq = ioread32(&pcch_hdr.minimum_frequency) * 1000;
    pr_debug("init: max_freq is %d, min_freq is %d\n",
    policy.cpuinfo.max_freq, policy.cpuinfo.min_freq);
    out:
    return result;
    }
    static struct cpufreq_driver pcc_cpufreq_driver = {
    .flags = CPUFREQ_CONST_LOOPS,
    .get = pcc_get_freq,
    .verify = pcc_cpufreq_verify,
    .target = pcc_cpufreq_target,
    .init = pcc_cpufreq_cpu_init,
    .name = "pcc-cpufreq",
    };
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_probe(pdev: *mut platform_device) -> int __init {
    static int __init pcc_cpufreq_probe(struct platform_device *pdev)
    {
    int ret;
// Skip initialization if another cpufreq driver is there.
    if (cpufreq_get_current_driver())
    return -ENODEV;
    if (acpi_disabled)
    return -ENODEV;
    ret = pcc_cpufreq_evaluate();
    if (ret) {
    pr_debug("pcc_cpufreq_probe: PCCH evaluation failed\n");
    return ret;
    }
    if (num_present_cpus() > 4) {
    pcc_cpufreq_driver.flags |= CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING;
    pr_err("%s: Too many CPUs, dynamic performance scaling disabled\n",
    __func__);
    pr_err("%s: Try to enable another scaling driver through BIOS settings\n",
    __func__);
    pr_err("%s: and complain to the system vendor\n", __func__);
    }
    ret = cpufreq_register_driver(&pcc_cpufreq_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_remove(pdev: *mut platform_device) {
    static void pcc_cpufreq_remove(struct platform_device *pdev)
    {
    cpufreq_unregister_driver(&pcc_cpufreq_driver);
    pcc_clear_mapping();
    free_percpu(pcc_cpu_info);
    }
    static struct platform_driver pcc_cpufreq_platdrv = {
    .driver = {
    .name	= "pcc-cpufreq",
    },
    .remove		= pcc_cpufreq_remove,
    };
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_init() -> int __init {
    static int __init pcc_cpufreq_init(void)
    {
    return platform_driver_probe(&pcc_cpufreq_platdrv, pcc_cpufreq_probe);
    }
#[no_mangle]
unsafe extern "C" fn pcc_cpufreq_exit() -> void __exit {
    static void __exit pcc_cpufreq_exit(void)
    {
    platform_driver_unregister(&pcc_cpufreq_platdrv);
    }
    MODULE_ALIAS("platform:pcc-cpufreq");
    MODULE_AUTHOR("Matthew Garrett, Naga Chumbalkar");
    MODULE_VERSION(PCC_VERSION);
    MODULE_DESCRIPTION("Processor Clocking Control interface driver");
    MODULE_LICENSE("GPL");
    late_initcall(pcc_cpufreq_init);
    module_exit(pcc_cpufreq_exit);
