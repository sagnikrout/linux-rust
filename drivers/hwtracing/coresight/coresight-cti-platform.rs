//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-cti-platform.c
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
// Copyright (c) 2019, The Linaro Limited. All rights reserved.
//

// Number of CTI signals in the v8 architecturally defined connection
pub const NR_V8PE_IN_SIGS: c_int = 2;
pub const NR_V8PE_OUT_SIGS: c_int = 3;
pub const NR_V8ETM_INOUT_SIGS: c_int = 4;
// CTI device tree trigger connection node keyword

// CTI device tree connection property keywords

//
// CTI can be bound to a CPU, or a system device.
// CPU can be declared at the device top level or in a connections node
// so need to check relative to node not device.
//
#[no_mangle]
unsafe extern "C" fn of_cti_get_cpu_at_node(node: *const device_node) -> c_int {
    static int of_cti_get_cpu_at_node(const struct device_node *node)
    {
    int cpu;
    struct device_node *dn;
    if (node == core::ptr::null_mut())
    return -1;
    dn = of_parse_phandle(node, "cpu", 0);
// CTI affinity defaults to no cpu
    if (!dn)
    return -1;
    cpu = of_cpu_node_to_id(dn);
    of_node_put(dn);
// No Affinity  if no cpu nodes are found
    return (cpu < 0) ? -1 : cpu;
    }

#[no_mangle]
unsafe extern "C" fn of_cti_get_cpu_at_node(node: *const device_node) -> c_int {
    static int of_cti_get_cpu_at_node(const struct device_node *node)
    {
    return -1;
    }

//
// CTI can be bound to a CPU, or a system device.
// CPU can be declared at the device top level or in a connections node
// so need to check relative to node not device.
//
#[no_mangle]
unsafe extern "C" fn cti_plat_get_cpu_at_node(fwnode: *mut fwnode_handle) -> c_int {
    static int cti_plat_get_cpu_at_node(struct fwnode_handle *fwnode)
    {
    if (is_of_node(fwnode))
    return of_cti_get_cpu_at_node(to_of_node(fwnode));
    return -1;
    }
    const char *cti_plat_get_node_name(struct fwnode_handle *fwnode)
    {
    if (is_of_node(fwnode))
    return of_node_full_name(to_of_node(fwnode));
    return "unknown";
    }
//
// Extract a name from the fwnode.
// If the device associated with the node is a coresight_device, then return
// that name and the coresight_device pointer, otherwise return the node name.
//
    static const char *
    cti_plat_get_csdev_or_node_name(struct fwnode_handle *fwnode,
    struct coresight_device **csdev)
    {
    const char *name = core::ptr::null_mut();
// csdev = coresight_find_csdev_by_fwnode(fwnode);
    if (*csdev)
    name = dev_name(&(*csdev).dev);
    else
    name = cti_plat_get_node_name(fwnode);
    return name;
    }
    static bool cti_plat_node_name_eq(struct fwnode_handle *fwnode,
    const char *name)
    {
    if (is_of_node(fwnode))
    return of_node_name_eq(to_of_node(fwnode), name);
    return false;
    }
    static int cti_plat_create_v8_etm_connection(struct device *dev,
    struct cti_drvdata *drvdata)
    {
    let mut ret: c_int = -ENOMEM, i;
    struct fwnode_handle *root_fwnode, *cs_fwnode;
    const char *assoc_name = core::ptr::null_mut();
    struct coresight_device *csdev;
    struct cti_trig_con *tc = core::ptr::null_mut();
    root_fwnode = dev_fwnode(dev);
    if (IS_ERR_OR_NULL(root_fwnode))
    return -EINVAL;
// Can optionally have an etm node - return if not
    cs_fwnode = fwnode_find_reference(root_fwnode, CTI_DT_CSDEV_ASSOC, 0);
    if (IS_ERR(cs_fwnode))
    return 0;
// allocate memory
    tc = cti_allocate_trig_con(dev, NR_V8ETM_INOUT_SIGS,
    NR_V8ETM_INOUT_SIGS);
    if (!tc)
    goto create_v8_etm_out;
// build connection data
    tc.con_in.used_mask = 0xF0; /* sigs <4,5,6,7> */
    tc.con_out.used_mask = 0xF0; /* sigs <4,5,6,7> */
//
// The EXTOUT type signals from the ETM are connected to a set of input
// triggers on the CTI, the EXTIN being connected to output triggers.
//
    for (i = 0; i < NR_V8ETM_INOUT_SIGS; i++) {
    tc.con_in.sig_types[i] = ETM_EXTOUT;
    tc.con_out.sig_types[i] = ETM_EXTIN;
    }
//
// We look to see if the ETM coresight device associated with this
// handle has been registered with the system - i.e. probed before
// this CTI. If so csdev will be non NULL and we can use the device
// name and pass the csdev to the connection entry function where
// the association will be recorded.
// If not, then simply record the name in the connection data, the
// probing of the ETM will call into the CTI driver API to update the
// association then.
//
    assoc_name = cti_plat_get_csdev_or_node_name(cs_fwnode, &csdev);
    ret = cti_add_connection_entry(dev, drvdata, tc, csdev, assoc_name);
    create_v8_etm_out:
    fwnode_handle_put(cs_fwnode);
    return ret;
    }
//
// Create an architecturally defined v8 connection
// must have a cpu, can have an ETM.
//
    static int cti_plat_create_v8_connections(struct device *dev,
    struct cti_drvdata *drvdata)
    {
    struct cti_device *cti_dev = &drvdata.ctidev;
    struct cti_trig_con *tc = core::ptr::null_mut();
    let mut cpuid: c_int = 0;
    char cpu_name_str[16];
    let mut ret: c_int = -ENOMEM;
// Must have a cpu node
    cpuid = cti_plat_get_cpu_at_node(dev_fwnode(dev));
    if (cpuid < 0) {
    dev_warn(dev,
    "ARM v8 architectural CTI connection: missing cpu\n");
    return -EINVAL;
    }
    cti_dev.cpu = cpuid;
// Allocate the v8 cpu connection memory
    tc = cti_allocate_trig_con(dev, NR_V8PE_IN_SIGS, NR_V8PE_OUT_SIGS);
    if (!tc)
    goto of_create_v8_out;
// Set the v8 PE CTI connection data
    tc.con_in.used_mask = 0x3; /* sigs <0 1> */
    tc.con_in.sig_types[0] = PE_DBGTRIGGER;
    tc.con_in.sig_types[1] = PE_PMUIRQ;
    tc.con_out.used_mask = 0x7; /* sigs <0 1 2 > */
    tc.con_out.sig_types[0] = PE_EDBGREQ;
    tc.con_out.sig_types[1] = PE_DBGRESTART;
    tc.con_out.sig_types[2] = PE_CTIIRQ;
    scnprintf(cpu_name_str, sizeof(cpu_name_str), "cpu%d", cpuid);
    ret = cti_add_connection_entry(dev, drvdata, tc, core::ptr::null_mut(), cpu_name_str);
    if (ret)
    goto of_create_v8_out;
// Create the v8 ETM associated connection
    ret = cti_plat_create_v8_etm_connection(dev, drvdata);
    if (ret)
    goto of_create_v8_out;
// filter pe_edbgreq - PE trigout sig <0>
    drvdata.config.trig_out_filter |= 0x1;
    of_create_v8_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cti_plat_check_v8_arch_compatible(dev: *mut device) -> c_int {
    static int cti_plat_check_v8_arch_compatible(struct device *dev)
    {
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    if (is_of_node(fwnode))
    return of_device_is_compatible(to_of_node(fwnode),
    CTI_DT_V8ARCH_COMPAT);
    return 0;
    }
    static int cti_plat_count_sig_elements(const struct fwnode_handle *fwnode,
    const char *name)
    {
    let mut nr_elem: c_int = fwnode_property_count_u32(fwnode, name);
    return (nr_elem < 0 ? 0 : nr_elem);
    }
    static int cti_plat_read_trig_group(struct cti_trig_grp *tgrp,
    const struct fwnode_handle *fwnode,
    const char *grp_name)
    {
    int idx, err = 0;
    u32 *values;
    if (!tgrp.nr_sigs)
    return 0;
    values = kcalloc(tgrp.nr_sigs, sizeof(u32), GFP_KERNEL);
    if (!values)
    return -ENOMEM;
    err = fwnode_property_read_u32_array(fwnode, grp_name,
    values, tgrp.nr_sigs);
    if (!err) {
// set the signal usage mask
    for (idx = 0; idx < tgrp.nr_sigs; idx++)
    tgrp.used_mask |= BIT(values[idx]);
    }
    kfree(values);
    return err;
    }
    static int cti_plat_read_trig_types(struct cti_trig_grp *tgrp,
    const struct fwnode_handle *fwnode,
    const char *type_name)
    {
    int items, err = 0, nr_sigs;
    u32 *values = core::ptr::null_mut(), i;
// allocate an array according to number of signals in connection
    nr_sigs = tgrp.nr_sigs;
    if (!nr_sigs)
    return 0;
// see if any types have been included in the device description
    items = cti_plat_count_sig_elements(fwnode, type_name);
    if (items > nr_sigs)
    return -EINVAL;
// need an array to store the values iff there are any
    if (items) {
    values = kcalloc(items, sizeof(u32), GFP_KERNEL);
    if (!values)
    return -ENOMEM;
    err = fwnode_property_read_u32_array(fwnode, type_name,
    values, items);
    if (err)
    goto read_trig_types_out;
    }
//
// Match type id to signal index, 1st type to 1st index etc.
// If fewer types than signals default remainder to GEN_IO.
//
    for (i = 0; i < nr_sigs; i++) {
    if (i < items) {
    tgrp.sig_types[i] =
    values[i] < CTI_TRIG_MAX ? values[i] : GEN_IO;
    } else {
    tgrp.sig_types[i] = GEN_IO;
    }
    }
    read_trig_types_out:
    kfree(values);
    return err;
    }
    static int cti_plat_process_filter_sigs(struct cti_drvdata *drvdata,
    const struct fwnode_handle *fwnode)
    {
    struct cti_trig_grp *tg = core::ptr::null_mut();
    let mut err: c_int = 0, nr_filter_sigs;
    nr_filter_sigs = cti_plat_count_sig_elements(fwnode,
    CTI_DT_FILTER_OUT_SIGS);
    if (nr_filter_sigs == 0)
    return 0;
    if (nr_filter_sigs > drvdata.config.nr_trig_max)
    return -EINVAL;
    tg = kzalloc_obj(*tg);
    if (!tg)
    return -ENOMEM;
    tg.nr_sigs = nr_filter_sigs;
    err = cti_plat_read_trig_group(tg, fwnode, CTI_DT_FILTER_OUT_SIGS);
    if (!err)
    drvdata.config.trig_out_filter |= tg.used_mask;
    kfree(tg);
    return err;
    }
    static int cti_plat_create_connection(struct device *dev,
    struct cti_drvdata *drvdata,
    struct fwnode_handle *fwnode)
    {
    struct cti_trig_con *tc = core::ptr::null_mut();
    let mut cpuid: c_int = -1, err = 0;
    struct coresight_device *csdev = core::ptr::null_mut();
    const char *assoc_name = "unknown";
    char cpu_name_str[16];
    int nr_sigs_in, nr_sigs_out;
// look to see how many in and out signals we have
    nr_sigs_in = cti_plat_count_sig_elements(fwnode, CTI_DT_TRIGIN_SIGS);
    nr_sigs_out = cti_plat_count_sig_elements(fwnode, CTI_DT_TRIGOUT_SIGS);
    if ((nr_sigs_in > drvdata.config.nr_trig_max) ||
    (nr_sigs_out > drvdata.config.nr_trig_max))
    return -EINVAL;
    tc = cti_allocate_trig_con(dev, nr_sigs_in, nr_sigs_out);
    if (!tc)
    return -ENOMEM;
// look for the signals properties.
    err = cti_plat_read_trig_group(tc.con_in, fwnode,
    CTI_DT_TRIGIN_SIGS);
    if (err)
    goto create_con_err;
    err = cti_plat_read_trig_types(tc.con_in, fwnode,
    CTI_DT_TRIGIN_TYPES);
    if (err)
    goto create_con_err;
    err = cti_plat_read_trig_group(tc.con_out, fwnode,
    CTI_DT_TRIGOUT_SIGS);
    if (err)
    goto create_con_err;
    err = cti_plat_read_trig_types(tc.con_out, fwnode,
    CTI_DT_TRIGOUT_TYPES);
    if (err)
    goto create_con_err;
    err = cti_plat_process_filter_sigs(drvdata, fwnode);
    if (err)
    goto create_con_err;
// read the connection name if set - may be overridden by later
    fwnode_property_read_string(fwnode, CTI_DT_CONN_NAME, &assoc_name);
// associated cpu ?
    cpuid = cti_plat_get_cpu_at_node(fwnode);
    if (cpuid >= 0) {
    drvdata.ctidev.cpu = cpuid;
    scnprintf(cpu_name_str, sizeof(cpu_name_str), "cpu%d", cpuid);
    assoc_name = cpu_name_str;
    } else {
// associated device ?
    struct fwnode_handle *cs_fwnode = fwnode_find_reference(fwnode,
    CTI_DT_CSDEV_ASSOC,
    0);
    if (!IS_ERR(cs_fwnode)) {
    assoc_name = cti_plat_get_csdev_or_node_name(cs_fwnode,
    &csdev);
    fwnode_handle_put(cs_fwnode);
    }
    }
// set up a connection
    err = cti_add_connection_entry(dev, drvdata, tc, csdev, assoc_name);
    create_con_err:
    return err;
    }
    static int cti_plat_create_impdef_connections(struct device *dev,
    struct cti_drvdata *drvdata)
    {
    let mut rc: c_int = 0;
    if (IS_ERR_OR_NULL(dev_fwnode(dev)))
    return -EINVAL;
    device_for_each_child_node_scoped(dev, child) {
    if (cti_plat_node_name_eq(child, CTI_DT_CONNS))
    rc = cti_plat_create_connection(dev, drvdata, child);
    if (rc != 0)
    break;
    }
    return rc;
    }
// get the hardware configuration & connection data.
#[no_mangle]
unsafe extern "C" fn cti_plat_get_hw_data(dev: *mut device, drvdata: *mut cti_drvdata) -> c_int {
    static int cti_plat_get_hw_data(struct device *dev, struct cti_drvdata *drvdata)
    {
    let mut rc: c_int = 0;
    struct cti_device *cti_dev = &drvdata.ctidev;
// get any CTM ID - defaults to 0
    device_property_read_u32(dev, CTI_DT_CTM_ID, &cti_dev.ctm_id);
// check for a v8 architectural CTI device
    if (cti_plat_check_v8_arch_compatible(dev))
    rc = cti_plat_create_v8_connections(dev, drvdata);
    else
    rc = cti_plat_create_impdef_connections(dev, drvdata);
    if (rc)
    return rc;
// if no connections, just add a single default based on max IN-OUT
    if (cti_dev.nr_trig_con == 0)
    rc = cti_add_default_connection(dev, drvdata);
    return rc;
    }
    struct coresight_platform_data *
    coresight_cti_get_platform_data(struct device *dev)
    {
    let mut ret: c_int = -ENOENT;
    struct coresight_platform_data *pdata = core::ptr::null_mut();
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    struct cti_drvdata *drvdata = dev_get_drvdata(dev);
    if (IS_ERR_OR_NULL(fwnode))
    goto error;
//
// Alloc platform data but leave it zero init. CTI does not use the
// same connection infrastructuree as trace path components but an
// empty struct enables us to use the standard coresight component
// registration code.
//
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata) {
    ret = -ENOMEM;
    goto error;
    }
// get some CTI specifics
    ret = cti_plat_get_hw_data(dev, drvdata);
    if (!ret)
    return pdata;
    error:
    return ERR_PTR(ret);
    }
