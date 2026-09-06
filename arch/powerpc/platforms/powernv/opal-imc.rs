//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-imc.c
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
// OPAL IMC interface detection driver
// Supported on POWERNV platform
//
// Copyright	(C) 2017 Madhavan Srinivasan, IBM Corporation.
// (C) 2017 Anju T Sudhakar, IBM Corporation.
// (C) 2017 Hemant K Shaw, IBM Corporation.
//

    static struct dentry *imc_debugfs_parent;
// Helpers to export imc command and mode via debugfs
#[no_mangle]
unsafe extern "C" fn imc_mem_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int imc_mem_get(void *data, u64 *val)
    {
// val = cpu_to_be64(*(u64 *)data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imc_mem_set(data: *mut c_void, val: u64) -> c_int {
    static int imc_mem_set(void *data, u64 val)
    {
// (u64 *)data = cpu_to_be64(val);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_imc_x64, imc_mem_get, imc_mem_set, "0x%016llx\n");
    static void imc_debugfs_create_x64(const char *name, umode_t mode,
    struct dentry *parent, u64  *value)
    {
    debugfs_create_file_unsafe(name, mode, parent, value, &fops_imc_x64);
    }
//
// export_imc_mode_and_cmd: Create a debugfs interface
// for imc_cmd and imc_mode
// for each node in the system.
// imc_mode and imc_cmd can be changed by echo into
// this interface.
//
    static void export_imc_mode_and_cmd(struct device_node *node,
    struct imc_pmu *pmu_ptr)
    {
    static u64 loc, *imc_mode_addr, *imc_cmd_addr;
    char mode[16], cmd[16];
    u32 cb_offset;
    struct imc_mem_info *ptr = pmu_ptr.mem_info;
    imc_debugfs_parent = debugfs_create_dir("imc", arch_debugfs_dir);
    if (of_property_read_u32(node, "cb_offset", &cb_offset))
    cb_offset = IMC_CNTL_BLK_OFFSET;
    while (ptr.vbase != core::ptr::null_mut()) {
    loc = (u64)(ptr.vbase) + cb_offset;
    imc_mode_addr = (u64 *)(loc + IMC_CNTL_BLK_MODE_OFFSET);
    sprintf(mode, "imc_mode_%d", (u32)(ptr.id));
    imc_debugfs_create_x64(mode, 0600, imc_debugfs_parent,
    imc_mode_addr);
    imc_cmd_addr = (u64 *)(loc + IMC_CNTL_BLK_CMD_OFFSET);
    sprintf(cmd, "imc_cmd_%d", (u32)(ptr.id));
    imc_debugfs_create_x64(cmd, 0600, imc_debugfs_parent,
    imc_cmd_addr);
    ptr++;
    }
    }
//
// imc_get_mem_addr_nest: Function to get nest counter memory region
// for each chip
//
    static int imc_get_mem_addr_nest(struct device_node *node,
    struct imc_pmu *pmu_ptr,
    u32 offset)
    {
    let mut nr_chips: c_int = 0, i;
    u64 *base_addr_arr, baddr;
    u32 *chipid_arr;
    nr_chips = of_property_count_u32_elems(node, "chip-id");
    if (nr_chips <= 0)
    return -ENODEV;
    base_addr_arr = kcalloc(nr_chips, sizeof(*base_addr_arr), GFP_KERNEL);
    if (!base_addr_arr)
    return -ENOMEM;
    chipid_arr = kcalloc(nr_chips, sizeof(*chipid_arr), GFP_KERNEL);
    if (!chipid_arr) {
    kfree(base_addr_arr);
    return -ENOMEM;
    }
    if (of_property_read_u32_array(node, "chip-id", chipid_arr, nr_chips))
    goto error;
    if (of_property_read_u64_array(node, "base-addr", base_addr_arr,
    nr_chips))
    goto error;
    pmu_ptr.mem_info = kzalloc_objs(*pmu_ptr.mem_info, nr_chips + 1);
    if (!pmu_ptr.mem_info)
    goto error;
    for (i = 0; i < nr_chips; i++) {
    pmu_ptr.mem_info[i].id = chipid_arr[i];
    baddr = base_addr_arr[i] + offset;
    pmu_ptr.mem_info[i].vbase = phys_to_virt(baddr);
    }
    pmu_ptr.imc_counter_mmaped = true;
    kfree(base_addr_arr);
    kfree(chipid_arr);
    return 0;
    error:
    kfree(base_addr_arr);
    kfree(chipid_arr);
    return -1;
    }
//
// imc_pmu_create : Takes the parent device which is the pmu unit, pmu_index
// and domain as the inputs.
// Allocates memory for the struct imc_pmu, sets up its domain, size and offsets
//
    static struct imc_pmu *imc_pmu_create(struct device_node *parent, int pmu_index, int domain)
    {
    let mut ret: c_int = 0;
    struct imc_pmu *pmu_ptr;
    u32 offset;
// Return for unknown domain
    if (domain < 0)
    return core::ptr::null_mut();
// memory for pmu
    pmu_ptr = kzalloc_obj(*pmu_ptr);
    if (!pmu_ptr)
    return core::ptr::null_mut();
// Set the domain
    pmu_ptr.domain = domain;
    ret = of_property_read_u32(parent, "size", &pmu_ptr.counter_mem_size);
    if (ret)
    goto free_pmu;
    if (!of_property_read_u32(parent, "offset", &offset)) {
    if (imc_get_mem_addr_nest(parent, pmu_ptr, offset))
    goto free_pmu;
    }
// Function to register IMC pmu
    ret = init_imc_pmu(parent, pmu_ptr, pmu_index);
    if (ret) {
    pr_err("IMC PMU %s Register failed\n", pmu_ptr.pmu.name);
    kfree(pmu_ptr.pmu.name);
    if (pmu_ptr.domain == IMC_DOMAIN_NEST)
    kfree(pmu_ptr.mem_info);
    kfree(pmu_ptr);
    return core::ptr::null_mut();
    }
    return pmu_ptr;
    free_pmu:
    kfree(pmu_ptr);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn disable_nest_pmu_counters() {
    static void disable_nest_pmu_counters(void)
    {
    int nid, cpu;
    const struct cpumask *l_cpumask;
    cpus_read_lock();
    for_each_node_with_cpus(nid) {
    l_cpumask = cpumask_of_node(nid);
    cpu = cpumask_first_and(l_cpumask, cpu_online_mask);
    if (cpu >= nr_cpu_ids)
    continue;
    opal_imc_counters_stop(OPAL_IMC_COUNTERS_NEST,
    get_hard_smp_processor_id(cpu));
    }
    cpus_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn disable_core_pmu_counters() {
    static void disable_core_pmu_counters(void)
    {
    int cpu, rc;
    cpus_read_lock();
// Disable the IMC Core functions
    for_each_online_cpu(cpu) {
    if (cpu_first_thread_sibling(cpu) != cpu)
    continue;
    rc = opal_imc_counters_stop(OPAL_IMC_COUNTERS_CORE,
    get_hard_smp_processor_id(cpu));
    if (rc)
    pr_err("%s: Failed to stop Core (cpu = %d)\n",
    __func__, cpu);
    }
    cpus_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn get_max_nest_dev() -> c_int {
    int get_max_nest_dev(void)
    {
    struct device_node *node;
    let mut pmu_units: u32 = 0, type;
    for_each_compatible_node(node, core::ptr::null_mut(), IMC_DTB_UNIT_COMPAT) {
    if (of_property_read_u32(node, "type", &type))
    continue;
    if (type == IMC_TYPE_CHIP)
    pmu_units++;
    }
    return pmu_units;
    }
#[no_mangle]
unsafe extern "C" fn opal_imc_counters_probe(pdev: *mut platform_device) -> c_int {
    static int opal_imc_counters_probe(struct platform_device *pdev)
    {
    struct device_node *imc_dev = pdev.dev.of_node;
    struct imc_pmu *pmu;
    let mut pmu_count: c_int = 0, domain;
    let mut core_imc_reg: bool = false, thread_imc_reg = false;
    u32 type;
//
// Check whether this is kdump kernel. If yes, force the engines to
// stop and return.
//
    if (is_kdump_kernel()) {
    disable_nest_pmu_counters();
    disable_core_pmu_counters();
    return -ENODEV;
    }
    for_each_compatible_node(imc_dev, core::ptr::null_mut(), IMC_DTB_UNIT_COMPAT) {
    pmu = core::ptr::null_mut();
    if (of_property_read_u32(imc_dev, "type", &type)) {
    pr_warn("IMC Device without type property\n");
    continue;
    }
    switch (type) {
    case IMC_TYPE_CHIP:
    domain = IMC_DOMAIN_NEST;
    break;
    case IMC_TYPE_CORE:
    domain =IMC_DOMAIN_CORE;
    break;
    case IMC_TYPE_THREAD:
    domain = IMC_DOMAIN_THREAD;
    break;
    case IMC_TYPE_TRACE:
    domain = IMC_DOMAIN_TRACE;
    break;
    default:
    pr_warn("IMC Unknown Device type \n");
    domain = -1;
    break;
    }
    pmu = imc_pmu_create(imc_dev, pmu_count, domain);
    if (pmu != core::ptr::null_mut()) {
    if (domain == IMC_DOMAIN_NEST) {
    if (!imc_debugfs_parent)
    export_imc_mode_and_cmd(imc_dev, pmu);
    pmu_count++;
    }
    if (domain == IMC_DOMAIN_CORE)
    core_imc_reg = true;
    if (domain == IMC_DOMAIN_THREAD)
    thread_imc_reg = true;
    }
    }
// If core imc is not registered, unregister thread-imc
    if (!core_imc_reg && thread_imc_reg)
    unregister_thread_imc();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opal_imc_counters_shutdown(pdev: *mut platform_device) {
    static void opal_imc_counters_shutdown(struct platform_device *pdev)
    {
//
// Function only stops the engines which is bare minimum.
// TODO: Need to handle proper memory cleanup and pmu
// unregister.
//
    disable_nest_pmu_counters();
    disable_core_pmu_counters();
    }
    static const struct of_device_id opal_imc_match[] = {
    { .compatible = IMC_DTB_COMPAT },
    {},
    };
    static struct platform_driver opal_imc_driver = {
    .driver = {
    .name = "opal-imc-counters",
    .of_match_table = opal_imc_match,
    },
    .probe = opal_imc_counters_probe,
    .shutdown = opal_imc_counters_shutdown,
    };
    builtin_platform_driver(opal_imc_driver);
