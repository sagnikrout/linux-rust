//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/pmem.c
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
// Handles hot and cold plug of persistent memory regions on pseries.
//

    static struct device_node *pmem_node;
#[no_mangle]
unsafe extern "C" fn pmem_drc_add_node(drc_index: u32) -> isize {
    static ssize_t pmem_drc_add_node(u32 drc_index)
    {
    struct device_node *dn;
    int rc;
    pr_debug("Attempting to add pmem node, drc index: %x\n", drc_index);
    rc = dlpar_acquire_drc(drc_index);
    if (rc) {
    pr_err("Failed to acquire DRC, rc: %d, drc index: %x\n",
    rc, drc_index);
    return -EINVAL;
    }
    dn = dlpar_configure_connector(cpu_to_be32(drc_index), pmem_node);
    if (!dn) {
    pr_err("configure-connector failed for drc %x\n", drc_index);
    dlpar_release_drc(drc_index);
    return -EINVAL;
    }
// NB: The of reconfig notifier creates platform device from the node
    rc = dlpar_attach_node(dn, pmem_node);
    if (rc) {
    pr_err("Failed to attach node %pOF, rc: %d, drc index: %x\n",
    dn, rc, drc_index);
    if (dlpar_release_drc(drc_index))
    dlpar_free_cc_nodes(dn);
    return rc;
    }
    pr_info("Successfully added %pOF, drc index: %x\n", dn, drc_index);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmem_drc_remove_node(drc_index: u32) -> isize {
    static ssize_t pmem_drc_remove_node(u32 drc_index)
    {
    struct device_node *dn;
    uint32_t index;
    int rc;
    for_each_child_of_node(pmem_node, dn) {
    if (of_property_read_u32(dn, "ibm,my-drc-index", &index))
    continue;
    if (index == drc_index)
    break;
    }
    if (!dn) {
    pr_err("Attempting to remove unused DRC index %x\n", drc_index);
    return -ENODEV;
    }
    pr_debug("Attempting to remove %pOF, drc index: %x\n", dn, drc_index);
// * NB: tears down the ibm,pmemory device as a side-effect
    rc = dlpar_detach_node(dn);
    if (rc)
    return rc;
    rc = dlpar_release_drc(drc_index);
    if (rc) {
    pr_err("Failed to release drc (%x) for CPU %pOFn, rc: %d\n",
    drc_index, dn, rc);
    dlpar_attach_node(dn, pmem_node);
    return rc;
    }
    pr_info("Successfully removed PMEM with drc index: %x\n", drc_index);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_hp_pmem(hp_elog: *mut pseries_hp_errorlog) -> c_int {
    int dlpar_hp_pmem(struct pseries_hp_errorlog *hp_elog)
    {
    u32 drc_index;
    int rc;
// slim chance, but we might get a hotplug event while booting
    if (!pmem_node)
    pmem_node = of_find_node_by_type(core::ptr::null_mut(), "ibm,persistent-memory");
    if (!pmem_node) {
    pr_err("Hotplug event for a pmem device, but none exists\n");
    return -ENODEV;
    }
    if (hp_elog.id_type != PSERIES_HP_ELOG_ID_DRC_INDEX) {
    pr_err("Unsupported hotplug event type %d\n",
    hp_elog.id_type);
    return -EINVAL;
    }
    drc_index = be32_to_cpu(hp_elog._drc_u.drc_index);
    lock_device_hotplug();
    if (hp_elog.action == PSERIES_HP_ELOG_ACTION_ADD) {
    rc = pmem_drc_add_node(drc_index);
    } else if (hp_elog.action == PSERIES_HP_ELOG_ACTION_REMOVE) {
    rc = pmem_drc_remove_node(drc_index);
    } else {
    pr_err("Unsupported hotplug action (%d)\n", hp_elog.action);
    rc = -EINVAL;
    }
    unlock_device_hotplug();
    return rc;
    }
    static const struct of_device_id drc_pmem_match[] = {
    { .type = "ibm,persistent-memory", },
    {}
    };
#[no_mangle]
unsafe extern "C" fn pseries_pmem_init() -> c_int {
    static int pseries_pmem_init(void)
    {
//
// Only supported on POWER8 and above.
//
    if (!cpu_has_feature(CPU_FTR_ARCH_207S))
    return 0;
    pmem_node = of_find_node_by_type(core::ptr::null_mut(), "ibm,persistent-memory");
    if (!pmem_node)
    return 0;
//
// The generic OF bus probe/populate handles creating platform devices
// from the child (ibm,pmemory) nodes. The generic code registers an of
// reconfig notifier to handle the hot-add/remove cases too.
//
    of_platform_bus_probe(pmem_node, drc_pmem_match, core::ptr::null_mut());
    return 0;
    }
    machine_arch_initcall(pseries, pseries_pmem_init);
