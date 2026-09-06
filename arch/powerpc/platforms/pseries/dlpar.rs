//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/dlpar.c
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
// Support for dynamic reconfiguration for PCI, Memory, and CPU
// Hotplug and Dynamic Logical Partitioning on RPA platforms.
//
// Copyright (C) 2009 Nathan Fontenot
// Copyright (C) 2009 IBM Corporation
//

    static struct workqueue_struct *pseries_hp_wq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_hp_work {
    pub work: work_struct,
    pub errlog: *mut pseries_hp_errorlog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_workarea {
    pub drc_index: __be32,
    pub zero: __be32,
    pub name_offset: __be32,
    pub prop_length: __be32,
    pub prop_offset: __be32,
}

#[no_mangle]
pub unsafe extern "C" fn dlpar_free_cc_property(prop: *mut property) {
    void dlpar_free_cc_property(struct property *prop)
    {
    kfree(prop.name);
    kfree(prop.value);
    kfree(prop);
    }
    static struct property *dlpar_parse_cc_property(struct cc_workarea *ccwa)
    {
    struct property *prop;
    char *name;
    char *value;
    prop = kzalloc_obj(*prop);
    if (!prop)
    return core::ptr::null_mut();
    name = (char *)ccwa + be32_to_cpu(ccwa.name_offset);
    prop.name = kstrdup(name, GFP_KERNEL);
    if (!prop.name) {
    dlpar_free_cc_property(prop);
    return core::ptr::null_mut();
    }
    prop.length = be32_to_cpu(ccwa.prop_length);
    value = (char *)ccwa + be32_to_cpu(ccwa.prop_offset);
    prop.value = kmemdup(value, prop.length, GFP_KERNEL);
    if (!prop.value) {
    dlpar_free_cc_property(prop);
    return core::ptr::null_mut();
    }
    return prop;
    }
    static struct device_node *dlpar_parse_cc_node(struct cc_workarea *ccwa)
    {
    struct device_node *dn;
    const char *name;
    dn = kzalloc_obj(*dn);
    if (!dn)
    return core::ptr::null_mut();
    name = (const char *)ccwa + be32_to_cpu(ccwa.name_offset);
    dn.full_name = kstrdup(name, GFP_KERNEL);
    if (!dn.full_name) {
    kfree(dn);
    return core::ptr::null_mut();
    }
    of_node_set_flag(dn, OF_DYNAMIC);
    of_node_init(dn);
    return dn;
    }
#[no_mangle]
unsafe extern "C" fn dlpar_free_one_cc_node(dn: *mut device_node) {
    static void dlpar_free_one_cc_node(struct device_node *dn)
    {
    struct property *prop;
    while (dn.properties) {
    prop = dn.properties;
    dn.properties = prop.next;
    dlpar_free_cc_property(prop);
    }
    kfree(dn.full_name);
    kfree(dn);
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_free_cc_nodes(dn: *mut device_node) {
    void dlpar_free_cc_nodes(struct device_node *dn)
    {
    if (dn.child)
    dlpar_free_cc_nodes(dn.child);
    if (dn.sibling)
    dlpar_free_cc_nodes(dn.sibling);
    dlpar_free_one_cc_node(dn);
    }
pub const COMPLETE: c_int = 0;
pub const NEXT_SIBLING: c_int = 1;
pub const NEXT_CHILD: c_int = 2;
pub const NEXT_PROPERTY: c_int = 3;
pub const PREV_PARENT: c_int = 4;
pub const MORE_MEMORY: c_int = 5;

    struct device_node *dlpar_configure_connector(__be32 drc_index,
    struct device_node *parent)
    {
    struct device_node *dn;
    struct device_node *first_dn = core::ptr::null_mut();
    struct device_node *last_dn = core::ptr::null_mut();
    struct property *property;
    struct property *last_property = core::ptr::null_mut();
    struct cc_workarea *ccwa;
    struct rtas_work_area *work_area;
    char *data_buf;
    int cc_token;
    let mut rc: c_int = -1;
    cc_token = rtas_function_token(RTAS_FN_IBM_CONFIGURE_CONNECTOR);
    if (cc_token == RTAS_UNKNOWN_SERVICE)
    return core::ptr::null_mut();
    work_area = rtas_work_area_alloc(SZ_4K);
    data_buf = rtas_work_area_raw_buf(work_area);
    ccwa = (struct cc_workarea *)&data_buf[0];
    ccwa.drc_index = drc_index;
    ccwa.zero = 0;
    do {
    do {
    rc = rtas_call(cc_token, 2, 1, core::ptr::null_mut(),
    rtas_work_area_phys(work_area), core::ptr::null_mut());
    } while (rtas_busy_delay(rc));
    switch (rc) {
    case COMPLETE:
    break;
    case NEXT_SIBLING:
    dn = dlpar_parse_cc_node(ccwa);
    if (!dn)
    goto cc_error;
    dn.parent = last_dn.parent;
    last_dn.sibling = dn;
    last_dn = dn;
    break;
    case NEXT_CHILD:
    dn = dlpar_parse_cc_node(ccwa);
    if (!dn)
    goto cc_error;
    if (!first_dn) {
    dn.parent = parent;
    first_dn = dn;
    } else {
    dn.parent = last_dn;
    if (last_dn)
    last_dn.child = dn;
    }
    last_dn = dn;
    break;
    case NEXT_PROPERTY:
    property = dlpar_parse_cc_property(ccwa);
    if (!property)
    goto cc_error;
    if (!last_dn.properties)
    last_dn.properties = property;
    else
    last_property.next = property;
    last_property = property;
    break;
    case PREV_PARENT:
    last_dn = last_dn.parent;
    break;
    case MORE_MEMORY:
    case ERR_CFG_USE:
    default:
    printk(KERN_ERR "Unexpected Error (%d) "
    "returned from configure-connector\n", rc);
    goto cc_error;
    }
    } while (rc);
    cc_error:
    rtas_work_area_free(work_area);
    if (rc) {
    if (first_dn)
    dlpar_free_cc_nodes(first_dn);
    return core::ptr::null_mut();
    }
    return first_dn;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_attach_node(dn: *mut device_node, parent: *mut device_node) -> c_int {
    int dlpar_attach_node(struct device_node *dn, struct device_node *parent)
    {
    int rc;
    dn.parent = parent;
    rc = of_attach_node(dn);
    if (rc) {
    printk(KERN_ERR "Failed to add device node %pOF\n", dn);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_detach_node(dn: *mut device_node) -> c_int {
    int dlpar_detach_node(struct device_node *dn)
    {
    struct device_node *child;
    int rc;
    for_each_child_of_node(dn, child)
    dlpar_detach_node(child);
    rc = of_detach_node(dn);
    if (rc)
    return rc;
    of_node_put(dn);
    return 0;
    }
    static int dlpar_changeset_attach_cc_nodes(struct of_changeset *ocs,
    struct device_node *dn)
    {
    int rc;
    rc = of_changeset_attach_node(ocs, dn);
    if (!rc && dn.child)
    rc = dlpar_changeset_attach_cc_nodes(ocs, dn.child);
    if (!rc && dn.sibling)
    rc = dlpar_changeset_attach_cc_nodes(ocs, dn.sibling);
    return rc;
    }
pub const DR_ENTITY_SENSE: c_int = 9003;
pub const DR_ENTITY_PRESENT: c_int = 1;
pub const DR_ENTITY_UNUSABLE: c_int = 2;
pub const ALLOCATION_STATE: c_int = 9003;
pub const ALLOC_UNUSABLE: c_int = 0;
pub const ALLOC_USABLE: c_int = 1;
pub const ISOLATION_STATE: c_int = 9001;
pub const ISOLATE: c_int = 0;
pub const UNISOLATE: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn dlpar_acquire_drc(drc_index: u32) -> c_int {
    int dlpar_acquire_drc(u32 drc_index)
    {
    int dr_status, rc;
    rc = rtas_get_sensor(DR_ENTITY_SENSE, drc_index, &dr_status);
    if (rc || dr_status != DR_ENTITY_UNUSABLE)
    return -1;
    rc = rtas_set_indicator(ALLOCATION_STATE, drc_index, ALLOC_USABLE);
    if (rc)
    return rc;
    rc = rtas_set_indicator(ISOLATION_STATE, drc_index, UNISOLATE);
    if (rc) {
    rtas_set_indicator(ALLOCATION_STATE, drc_index, ALLOC_UNUSABLE);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_release_drc(drc_index: u32) -> c_int {
    int dlpar_release_drc(u32 drc_index)
    {
    int dr_status, rc;
    rc = rtas_get_sensor(DR_ENTITY_SENSE, drc_index, &dr_status);
    if (rc || dr_status != DR_ENTITY_PRESENT)
    return -1;
    rc = rtas_set_indicator(ISOLATION_STATE, drc_index, ISOLATE);
    if (rc)
    return rc;
    rc = rtas_set_indicator(ALLOCATION_STATE, drc_index, ALLOC_UNUSABLE);
    if (rc) {
    rtas_set_indicator(ISOLATION_STATE, drc_index, UNISOLATE);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_unisolate_drc(drc_index: u32) -> c_int {
    int dlpar_unisolate_drc(u32 drc_index)
    {
    int dr_status, rc;
    rc = rtas_get_sensor(DR_ENTITY_SENSE, drc_index, &dr_status);
    if (rc || dr_status != DR_ENTITY_PRESENT)
    return -1;
    rtas_set_indicator(ISOLATION_STATE, drc_index, UNISOLATE);
    return 0;
    }
    static struct device_node *
    get_device_node_with_drc_index(u32 index)
    {
    struct device_node *np = core::ptr::null_mut();
    u32 node_index;
    int rc;
    for_each_node_with_property(np, "ibm,my-drc-index") {
    rc = of_property_read_u32(np, "ibm,my-drc-index",
    &node_index);
    if (rc) {
    pr_err("%s: %pOF: of_property_read_u32 %s: %d\n",
    __func__, np, "ibm,my-drc-index", rc);
    of_node_put(np);
    return core::ptr::null_mut();
    }
    if (index == node_index)
    break;
    }
    return np;
    }
    static struct device_node *
    get_device_node_with_drc_info(u32 index)
    {
    struct device_node *np = core::ptr::null_mut();
    struct of_drc_info drc;
    struct property *info;
    const __be32 *value;
    u32 node_index;
    int i, j, count;
    for_each_node_with_property(np, "ibm,drc-info") {
    info = of_find_property(np, "ibm,drc-info", core::ptr::null_mut());
    if (info == core::ptr::null_mut()) {
// XXX can this happen?
    of_node_put(np);
    return core::ptr::null_mut();
    }
    value = of_prop_next_u32(info, core::ptr::null_mut(), &count);
    if (value == core::ptr::null_mut())
    continue;
    value++;
    for (i = 0; i < count; i++) {
    if (of_read_drc_info_cell(&info, &value, &drc))
    break;
    if (index > drc.last_drc_index)
    continue;
    node_index = drc.drc_index_start;
    for (j = 0; j < drc.num_sequential_elems; j++) {
    if (index == node_index)
    return np;
    node_index += drc.sequential_inc;
    }
    }
    }
    return core::ptr::null_mut();
    }
    static struct device_node *
    get_device_node_with_drc_indexes(u32 drc_index)
    {
    struct device_node *np = core::ptr::null_mut();
    u32 nr_indexes, index;
    int i, rc;
    for_each_node_with_property(np, "ibm,drc-indexes") {
//
// First element in the array is the total number of
// DRC indexes returned.
//
    rc = of_property_read_u32_index(np, "ibm,drc-indexes",
    0, &nr_indexes);
    if (rc)
    goto out_put_np;
//
// Retrieve DRC index from the list and return the
// device node if matched with the specified index.
//
    for (i = 0; i < nr_indexes; i++) {
    rc = of_property_read_u32_index(np, "ibm,drc-indexes",
    i+1, &index);
    if (rc)
    goto out_put_np;
    if (drc_index == index)
    return np;
    }
    }
    return core::ptr::null_mut();
    out_put_np:
    of_node_put(np);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dlpar_hp_dt_add(index: u32) -> c_int {
    static int dlpar_hp_dt_add(u32 index)
    {
    struct device_node *np, *nodes;
    struct of_changeset ocs;
    int rc;
//
// Do not add device node(s) if already exists in the
// device tree.
//
    np = get_device_node_with_drc_index(index);
    if (np) {
    pr_err("%s: Adding device node for index (%d), but "
    "already exists in the device tree\n",
    __func__, index);
    rc = -EINVAL;
    goto out;
    }
//
// Recent FW provides ibm,drc-info property. So search
// for the user specified DRC index from ibm,drc-info
// property. If this property is not available, search
// in the indexes array from ibm,drc-indexes property.
//
    np = get_device_node_with_drc_info(index);
    if (!np) {
    np = get_device_node_with_drc_indexes(index);
    if (!np)
    return -EIO;
    }
// Next, configure the connector.
    nodes = dlpar_configure_connector(cpu_to_be32(index), np);
    if (!nodes) {
    rc = -EIO;
    goto out;
    }
//
// Add the new nodes from dlpar_configure_connector() onto
// the device-tree.
//
    of_changeset_init(&ocs);
    rc = dlpar_changeset_attach_cc_nodes(&ocs, nodes);
    if (!rc)
    rc = of_changeset_apply(&ocs);
    else
    dlpar_free_cc_nodes(nodes);
    of_changeset_destroy(&ocs);
    out:
    of_node_put(np);
    return rc;
    }
    static int changeset_detach_node_recursive(struct of_changeset *ocs,
    struct device_node *node)
    {
    struct device_node *child;
    int rc;
    for_each_child_of_node(node, child) {
    rc = changeset_detach_node_recursive(ocs, child);
    if (rc) {
    of_node_put(child);
    return rc;
    }
    }
    return of_changeset_detach_node(ocs, node);
    }
#[no_mangle]
unsafe extern "C" fn dlpar_hp_dt_remove(drc_index: u32) -> c_int {
    static int dlpar_hp_dt_remove(u32 drc_index)
    {
    struct device_node *np;
    struct of_changeset ocs;
    u32 index;
    let mut rc: c_int = 0;
//
// Prune all nodes with a matching index.
//
    of_changeset_init(&ocs);
    for_each_node_with_property(np, "ibm,my-drc-index") {
    rc = of_property_read_u32(np, "ibm,my-drc-index", &index);
    if (rc) {
    pr_err("%s: %pOF: of_property_read_u32 %s: %d\n",
    __func__, np, "ibm,my-drc-index", rc);
    of_node_put(np);
    goto out;
    }
    if (index == drc_index) {
    rc = changeset_detach_node_recursive(&ocs, np);
    if (rc) {
    of_node_put(np);
    goto out;
    }
    }
    }
    rc = of_changeset_apply(&ocs);
    out:
    of_changeset_destroy(&ocs);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn dlpar_hp_dt(phpe: *mut pseries_hp_errorlog) -> c_int {
    static int dlpar_hp_dt(struct pseries_hp_errorlog *phpe)
    {
    u32 drc_index;
    int rc;
    if (phpe.id_type != PSERIES_HP_ELOG_ID_DRC_INDEX)
    return -EINVAL;
    drc_index = be32_to_cpu(phpe._drc_u.drc_index);
    lock_device_hotplug();
    switch (phpe.action) {
    case PSERIES_HP_ELOG_ACTION_ADD:
    rc = dlpar_hp_dt_add(drc_index);
    break;
    case PSERIES_HP_ELOG_ACTION_REMOVE:
    rc = dlpar_hp_dt_remove(drc_index);
    break;
    default:
    pr_err("Invalid action (%d) specified\n", phpe.action);
    rc = -EINVAL;
    break;
    }
    unlock_device_hotplug();
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_dlpar_errorlog(hp_elog: *mut pseries_hp_errorlog) -> c_int {
    int handle_dlpar_errorlog(struct pseries_hp_errorlog *hp_elog)
    {
    int rc;
    switch (hp_elog.resource) {
    case PSERIES_HP_ELOG_RESOURCE_MEM:
    rc = dlpar_memory(hp_elog);
    break;
    case PSERIES_HP_ELOG_RESOURCE_CPU:
    rc = dlpar_cpu(hp_elog);
    break;
    case PSERIES_HP_ELOG_RESOURCE_PMEM:
    rc = dlpar_hp_pmem(hp_elog);
    break;
    case PSERIES_HP_ELOG_RESOURCE_DT:
    rc = dlpar_hp_dt(hp_elog);
    break;
    default:
    pr_warn_ratelimited("Invalid resource (%d) specified\n",
    hp_elog.resource);
    rc = -EINVAL;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pseries_hp_work_fn(work: *mut work_struct) {
    static void pseries_hp_work_fn(struct work_struct *work)
    {
    struct pseries_hp_work *hp_work =
    container_of(work, struct pseries_hp_work, work);
    handle_dlpar_errorlog(hp_work.errlog);
    kfree(hp_work.errlog);
    kfree(work);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_hotplug_event(hp_errlog: *mut pseries_hp_errorlog) {
    void queue_hotplug_event(struct pseries_hp_errorlog *hp_errlog)
    {
    struct pseries_hp_work *work;
    struct pseries_hp_errorlog *hp_errlog_copy;
    hp_errlog_copy = kmemdup(hp_errlog, sizeof(*hp_errlog), GFP_ATOMIC);
    if (!hp_errlog_copy)
    return;
    work = kmalloc_obj(struct pseries_hp_work, GFP_ATOMIC);
    if (work) {
    INIT_WORK((struct work_struct *)work, pseries_hp_work_fn);
    work.errlog = hp_errlog_copy;
    queue_work(pseries_hp_wq, (struct work_struct *)work);
    } else {
    kfree(hp_errlog_copy);
    }
    }
#[no_mangle]
unsafe extern "C" fn dlpar_parse_resource(cmd: *mut c_char, hp_elog: *mut pseries_hp_errorlog) -> c_int {
    static int dlpar_parse_resource(char **cmd, struct pseries_hp_errorlog *hp_elog)
    {
    char *arg;
    arg = strsep(cmd, " ");
    if (!arg)
    return -EINVAL;
    if (sysfs_streq(arg, "memory")) {
    hp_elog.resource = PSERIES_HP_ELOG_RESOURCE_MEM;
    } else if (sysfs_streq(arg, "cpu")) {
    hp_elog.resource = PSERIES_HP_ELOG_RESOURCE_CPU;
    } else if (sysfs_streq(arg, "dt")) {
    hp_elog.resource = PSERIES_HP_ELOG_RESOURCE_DT;
    } else {
    pr_err("Invalid resource specified.\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dlpar_parse_action(cmd: *mut c_char, hp_elog: *mut pseries_hp_errorlog) -> c_int {
    static int dlpar_parse_action(char **cmd, struct pseries_hp_errorlog *hp_elog)
    {
    char *arg;
    arg = strsep(cmd, " ");
    if (!arg)
    return -EINVAL;
    if (sysfs_streq(arg, "add")) {
    hp_elog.action = PSERIES_HP_ELOG_ACTION_ADD;
    } else if (sysfs_streq(arg, "remove")) {
    hp_elog.action = PSERIES_HP_ELOG_ACTION_REMOVE;
    } else {
    pr_err("Invalid action specified.\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dlpar_parse_id_type(cmd: *mut c_char, hp_elog: *mut pseries_hp_errorlog) -> c_int {
    static int dlpar_parse_id_type(char **cmd, struct pseries_hp_errorlog *hp_elog)
    {
    char *arg;
    u32 count, index;
    arg = strsep(cmd, " ");
    if (!arg)
    return -EINVAL;
    if (sysfs_streq(arg, "indexed-count")) {
    hp_elog.id_type = PSERIES_HP_ELOG_ID_DRC_IC;
    arg = strsep(cmd, " ");
    if (!arg) {
    pr_err("No DRC count specified.\n");
    return -EINVAL;
    }
    if (kstrtou32(arg, 0, &count)) {
    pr_err("Invalid DRC count specified.\n");
    return -EINVAL;
    }
    arg = strsep(cmd, " ");
    if (!arg) {
    pr_err("No DRC Index specified.\n");
    return -EINVAL;
    }
    if (kstrtou32(arg, 0, &index)) {
    pr_err("Invalid DRC Index specified.\n");
    return -EINVAL;
    }
    hp_elog._drc_u.ic.count = cpu_to_be32(count);
    hp_elog._drc_u.ic.index = cpu_to_be32(index);
    } else if (sysfs_streq(arg, "index")) {
    hp_elog.id_type = PSERIES_HP_ELOG_ID_DRC_INDEX;
    arg = strsep(cmd, " ");
    if (!arg) {
    pr_err("No DRC Index specified.\n");
    return -EINVAL;
    }
    if (kstrtou32(arg, 0, &index)) {
    pr_err("Invalid DRC Index specified.\n");
    return -EINVAL;
    }
    hp_elog._drc_u.drc_index = cpu_to_be32(index);
    } else if (sysfs_streq(arg, "count")) {
    hp_elog.id_type = PSERIES_HP_ELOG_ID_DRC_COUNT;
    arg = strsep(cmd, " ");
    if (!arg) {
    pr_err("No DRC count specified.\n");
    return -EINVAL;
    }
    if (kstrtou32(arg, 0, &count)) {
    pr_err("Invalid DRC count specified.\n");
    return -EINVAL;
    }
    hp_elog._drc_u.drc_count = cpu_to_be32(count);
    } else {
    pr_err("Invalid id_type specified.\n");
    return -EINVAL;
    }
    return 0;
    }
    static ssize_t dlpar_store(const struct class *class, const struct class_attribute *attr,
    const char *buf, size_t count)
    {
    struct pseries_hp_errorlog hp_elog;
    char *argbuf;
    char *args;
    int rc;
    args = argbuf = kstrdup(buf, GFP_KERNEL);
    if (!argbuf)
    return -ENOMEM;
//
// Parse out the request from the user, this will be in the form:
// <resource> <action> <id_type> <id>
//
    rc = dlpar_parse_resource(&args, &hp_elog);
    if (rc)
    goto dlpar_store_out;
    rc = dlpar_parse_action(&args, &hp_elog);
    if (rc)
    goto dlpar_store_out;
    rc = dlpar_parse_id_type(&args, &hp_elog);
    if (rc)
    goto dlpar_store_out;
    rc = handle_dlpar_errorlog(&hp_elog);
    dlpar_store_out:
    kfree(argbuf);
    if (rc)
    pr_err("Could not handle DLPAR request \"%s\"\n", buf);
    return rc ? rc : count;
    }
    static ssize_t dlpar_show(const struct class *class, const struct class_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n", "memory,cpu,dt");
    }
    static CLASS_ATTR_RW(dlpar);
#[no_mangle]
pub unsafe extern "C" fn dlpar_workqueue_init() -> int __init {
    int __init dlpar_workqueue_init(void)
    {
    if (pseries_hp_wq)
    return 0;
    pseries_hp_wq = alloc_ordered_workqueue("pseries hotplug workqueue", 0);
    return pseries_hp_wq ? 0 : -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn dlpar_sysfs_init() -> int __init {
    static int __init dlpar_sysfs_init(void)
    {
    int rc;
    rc = dlpar_workqueue_init();
    if (rc)
    return rc;
    return sysfs_create_file(kernel_kobj, &class_attr_dlpar.attr);
    }
    machine_device_initcall(pseries, dlpar_sysfs_init);
