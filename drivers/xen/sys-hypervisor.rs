//! Automatically rewritten from C to Rust
//! Source: drivers/xen/sys-hypervisor.c
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
// copyright (c) 2006 IBM Corporation
// Authored by: Mike D. Day <ncmike@us.ibm.com>
//

    static struct hyp_sysfs_attr _name##_attr = __ATTR_RO(_name)

    static struct hyp_sysfs_attr _name##_attr = __ATTR_RW(_name)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyp_sysfs_attr {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct hyp_sysfs_attr , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct hyp_sysfs_attr , char ,,
    union {
    pub hyp_attr_data: *mut c_void,
    pub hyp_attr_value: c_ulong,
}

    };
#[no_mangle]
unsafe extern "C" fn type_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t type_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    return sprintf(buffer, "xen\n");
    }
    HYPERVISOR_ATTR_RO(type);
#[no_mangle]
unsafe extern "C" fn xen_sysfs_type_init() -> int __init {
    static int __init xen_sysfs_type_init(void)
    {
    return sysfs_create_file(hypervisor_kobj, &type_attr.attr);
    }
#[no_mangle]
unsafe extern "C" fn guest_type_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t guest_type_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    const char *type;
    switch (xen_domain_type) {
    case XEN_NATIVE:
// ARM only.
    type = "Xen";
    break;
    case XEN_PV_DOMAIN:
    type = "PV";
    break;
    case XEN_HVM_DOMAIN:
    type = xen_pvh_domain() ? "PVH" : "HVM";
    break;
    default:
    return -EINVAL;
    }
    return sprintf(buffer, "%s\n", type);
    }
    HYPERVISOR_ATTR_RO(guest_type);
#[no_mangle]
unsafe extern "C" fn xen_sysfs_guest_type_init() -> int __init {
    static int __init xen_sysfs_guest_type_init(void)
    {
    return sysfs_create_file(hypervisor_kobj, &guest_type_attr.attr);
    }
// xen version attributes
#[no_mangle]
unsafe extern "C" fn major_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t major_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut version: c_int = HYPERVISOR_xen_version(XENVER_version, core::ptr::null_mut());
    if (version)
    return sprintf(buffer, "%d\n", version >> 16);
    return -ENODEV;
    }
    HYPERVISOR_ATTR_RO(major);
#[no_mangle]
unsafe extern "C" fn minor_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t minor_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut version: c_int = HYPERVISOR_xen_version(XENVER_version, core::ptr::null_mut());
    if (version)
    return sprintf(buffer, "%d\n", version & 0xff);
    return -ENODEV;
    }
    HYPERVISOR_ATTR_RO(minor);
#[no_mangle]
unsafe extern "C" fn extra_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t extra_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    char *extra;
    extra = kmalloc(XEN_EXTRAVERSION_LEN, GFP_KERNEL);
    if (extra) {
    ret = HYPERVISOR_xen_version(XENVER_extraversion, extra);
    if (!ret)
    ret = sprintf(buffer, "%s\n", extra);
    kfree(extra);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(extra);
    static struct attribute *version_attrs[] = {
    &major_attr.attr,
    &minor_attr.attr,
    &extra_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group version_group = {
    .name = "version",
    .attrs = version_attrs,
    };
#[no_mangle]
unsafe extern "C" fn xen_sysfs_version_init() -> int __init {
    static int __init xen_sysfs_version_init(void)
    {
    return sysfs_create_group(hypervisor_kobj, &version_group);
    }
// UUID
#[no_mangle]
unsafe extern "C" fn uuid_show_fallback(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t uuid_show_fallback(struct hyp_sysfs_attr *attr, char *buffer)
    {
    char *vm, *val;
    int ret;
    extern int xenstored_ready;
    if (!xenstored_ready)
    return -EBUSY;
    vm = xenbus_read(XBT_NIL, "vm", "", core::ptr::null_mut());
    if (IS_ERR(vm))
    return PTR_ERR(vm);
    val = xenbus_read(XBT_NIL, vm, "uuid", core::ptr::null_mut());
    kfree(vm);
    if (IS_ERR(val))
    return PTR_ERR(val);
    ret = sprintf(buffer, "%s\n", val);
    kfree(val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uuid_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t uuid_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    xen_domain_handle_t uuid;
    int ret;
    ret = HYPERVISOR_xen_version(XENVER_guest_handle, uuid);
    if (ret)
    return uuid_show_fallback(attr, buffer);
    ret = sprintf(buffer, "%pU\n", uuid);
    return ret;
    }
    HYPERVISOR_ATTR_RO(uuid);
#[no_mangle]
unsafe extern "C" fn xen_sysfs_uuid_init() -> int __init {
    static int __init xen_sysfs_uuid_init(void)
    {
    return sysfs_create_file(hypervisor_kobj, &uuid_attr.attr);
    }
// xen compilation attributes
#[no_mangle]
unsafe extern "C" fn compiler_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t compiler_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    struct xen_compile_info *info;
    info = kmalloc_obj(struct xen_compile_info);
    if (info) {
    ret = HYPERVISOR_xen_version(XENVER_compile_info, info);
    if (!ret)
    ret = sprintf(buffer, "%s\n", info.compiler);
    kfree(info);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(compiler);
#[no_mangle]
unsafe extern "C" fn compiled_by_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t compiled_by_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    struct xen_compile_info *info;
    info = kmalloc_obj(struct xen_compile_info);
    if (info) {
    ret = HYPERVISOR_xen_version(XENVER_compile_info, info);
    if (!ret)
    ret = sprintf(buffer, "%s\n", info.compile_by);
    kfree(info);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(compiled_by);
#[no_mangle]
unsafe extern "C" fn compile_date_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t compile_date_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    struct xen_compile_info *info;
    info = kmalloc_obj(struct xen_compile_info);
    if (info) {
    ret = HYPERVISOR_xen_version(XENVER_compile_info, info);
    if (!ret)
    ret = sprintf(buffer, "%s\n", info.compile_date);
    kfree(info);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(compile_date);
    static struct attribute *xen_compile_attrs[] = {
    &compiler_attr.attr,
    &compiled_by_attr.attr,
    &compile_date_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group xen_compilation_group = {
    .name = "compilation",
    .attrs = xen_compile_attrs,
    };
#[no_mangle]
unsafe extern "C" fn xen_sysfs_compilation_init() -> int __init {
    static int __init xen_sysfs_compilation_init(void)
    {
    return sysfs_create_group(hypervisor_kobj, &xen_compilation_group);
    }
// xen properties info
#[no_mangle]
unsafe extern "C" fn capabilities_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t capabilities_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    char *caps;
    caps = kmalloc(XEN_CAPABILITIES_INFO_LEN, GFP_KERNEL);
    if (caps) {
    ret = HYPERVISOR_xen_version(XENVER_capabilities, caps);
    if (!ret)
    ret = sprintf(buffer, "%s\n", caps);
    kfree(caps);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(capabilities);
#[no_mangle]
unsafe extern "C" fn changeset_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t changeset_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    char *cset;
    cset = kmalloc(XEN_CHANGESET_INFO_LEN, GFP_KERNEL);
    if (cset) {
    ret = HYPERVISOR_xen_version(XENVER_changeset, cset);
    if (!ret)
    ret = sprintf(buffer, "%s\n", cset);
    kfree(cset);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(changeset);
#[no_mangle]
unsafe extern "C" fn virtual_start_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t virtual_start_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    let mut ret: c_int = -ENOMEM;
    struct xen_platform_parameters *parms;
    parms = kmalloc_obj(struct xen_platform_parameters);
    if (parms) {
    ret = HYPERVISOR_xen_version(XENVER_platform_parameters,
    parms);
    if (!ret)
    ret = sprintf(buffer, "%"PRI_xen_ulong"\n",
    parms.virt_start);
    kfree(parms);
    }
    return ret;
    }
    HYPERVISOR_ATTR_RO(virtual_start);
#[no_mangle]
unsafe extern "C" fn pagesize_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t pagesize_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    int ret;
    ret = HYPERVISOR_xen_version(XENVER_pagesize, core::ptr::null_mut());
    if (ret > 0)
    ret = sprintf(buffer, "%x\n", ret);
    return ret;
    }
    HYPERVISOR_ATTR_RO(pagesize);
#[no_mangle]
unsafe extern "C" fn xen_feature_show(index: c_int, buffer: *mut c_char) -> isize {
    static ssize_t xen_feature_show(int index, char *buffer)
    {
    ssize_t ret;
    struct xen_feature_info info;
    info.submap_idx = index;
    ret = HYPERVISOR_xen_version(XENVER_get_features, &info);
    if (!ret)
    ret = sprintf(buffer, "%08x", info.submap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn features_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t features_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    ssize_t len;
    int i;
    len = 0;
    for (i = XENFEAT_NR_SUBMAPS-1; i >= 0; i--) {
    let mut ret: c_int = xen_feature_show(i, buffer + len);
    if (ret < 0) {
    if (len == 0)
    len = ret;
    break;
    }
    len += ret;
    }
    if (len > 0)
    buffer[len++] = '\n';
    return len;
    }
    HYPERVISOR_ATTR_RO(features);
#[no_mangle]
unsafe extern "C" fn buildid_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t buildid_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    ssize_t ret;
    struct xen_build_id *buildid;
    ret = HYPERVISOR_xen_version(XENVER_build_id, core::ptr::null_mut());
    if (ret < 0) {
    if (ret == -EPERM)
    ret = sprintf(buffer, "<denied>");
    return ret;
    }
    if (ret > PAGE_SIZE)
    return -ENOSPC;
    buildid = kmalloc(sizeof(*buildid) + ret, GFP_KERNEL);
    if (!buildid)
    return -ENOMEM;
    buildid.len = ret;
    ret = HYPERVISOR_xen_version(XENVER_build_id, buildid);
    if (ret > 0) {
// Build id is binary, not a string.
    memcpy(buffer, buildid.buf, ret);
    }
    kfree(buildid);
    return ret;
    }
    HYPERVISOR_ATTR_RO(buildid);
    static struct attribute *xen_properties_attrs[] = {
    &capabilities_attr.attr,
    &changeset_attr.attr,
    &virtual_start_attr.attr,
    &pagesize_attr.attr,
    &features_attr.attr,
    &buildid_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group xen_properties_group = {
    .name = "properties",
    .attrs = xen_properties_attrs,
    };
#[no_mangle]
unsafe extern "C" fn xen_sysfs_properties_init() -> int __init {
    static int __init xen_sysfs_properties_init(void)
    {
    return sysfs_create_group(hypervisor_kobj, &xen_properties_group);
    }

    static_assert(sizeof(xen_start_flags) <=
    sizeof_field(struct hyp_sysfs_attr, hyp_attr_value));
#[no_mangle]
unsafe extern "C" fn flag_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t flag_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    char *p = buffer;
// p++ = '0' + ((xen_start_flags & attr->hyp_attr_value) != 0);
// p++ = '\n';
    return p - buffer;
    }

    [ilog2(flag)] = {				\
    .attr = { .name = #node, .mode = 0444 },\
    .show = flag_show,			\
    .hyp_attr_value = flag			\
    }
//
// Add new, known flags here.  No other changes are required, but
// note that each known flag wastes one entry in flag_unames[].
// The code/complexity machinations to avoid this isn't worth it
// for a few entries, but keep it in mind.
//
    static struct hyp_sysfs_attr flag_attrs[FLAG_COUNT] = {
    FLAG_NODE(SIF_PRIVILEGED, privileged),
    FLAG_NODE(SIF_INITDOMAIN, initdomain)
    };
    static struct attribute_group xen_flags_group = {
    .name = "start_flags",
    .attrs = (struct attribute *[FLAG_COUNT + 1]){}
    };
    static char flag_unames[FLAG_COUNT][FLAG_UNAME_MAX];
#[no_mangle]
unsafe extern "C" fn xen_sysfs_flags_init() -> int __init {
    static int __init xen_sysfs_flags_init(void)
    {
    for (unsigned fnum = 0; fnum != FLAG_COUNT; fnum++) {
    if (likely(flag_attrs[fnum].attr.name == core::ptr::null_mut())) {
    sprintf(flag_unames[fnum], FLAG_UNAME_FMT, fnum);
    flag_attrs[fnum].attr.name = flag_unames[fnum];
    flag_attrs[fnum].attr.mode = 0444;
    flag_attrs[fnum].show = flag_show;
    flag_attrs[fnum].hyp_attr_value = 1 << fnum;
    }
    xen_flags_group.attrs[fnum] = &flag_attrs[fnum].attr;
    }
    return sysfs_create_group(hypervisor_kobj, &xen_flags_group);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_mode {
    pub name: *const c_char,
    pub mode: u32,
}

    static struct pmu_mode pmu_modes[] = {
    {"off", XENPMU_MODE_OFF},
    {"self", XENPMU_MODE_SELF},
    {"hv", XENPMU_MODE_HV},
    {"all", XENPMU_MODE_ALL}
    };
    static ssize_t pmu_mode_store(struct hyp_sysfs_attr *attr,
    const char *buffer, size_t len)
    {
    int ret;
    struct xen_pmu_params xp;
    int i;
    for (i = 0; i < ARRAY_SIZE(pmu_modes); i++) {
    if (strncmp(buffer, pmu_modes[i].name, len - 1) == 0) {
    xp.val = pmu_modes[i].mode;
    break;
    }
    }
    if (i == ARRAY_SIZE(pmu_modes))
    return -EINVAL;
    xp.version.maj = XENPMU_VER_MAJ;
    xp.version.min = XENPMU_VER_MIN;
    ret = HYPERVISOR_xenpmu_op(XENPMU_mode_set, &xp);
    if (ret)
    return ret;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn pmu_mode_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t pmu_mode_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    int ret;
    struct xen_pmu_params xp;
    int i;
    uint32_t mode;
    xp.version.maj = XENPMU_VER_MAJ;
    xp.version.min = XENPMU_VER_MIN;
    ret = HYPERVISOR_xenpmu_op(XENPMU_mode_get, &xp);
    if (ret)
    return ret;
    mode = (uint32_t)xp.val;
    for (i = 0; i < ARRAY_SIZE(pmu_modes); i++) {
    if (mode == pmu_modes[i].mode)
    return sprintf(buffer, "%s\n", pmu_modes[i].name);
    }
    return -EINVAL;
    }
    HYPERVISOR_ATTR_RW(pmu_mode);
    static ssize_t pmu_features_store(struct hyp_sysfs_attr *attr,
    const char *buffer, size_t len)
    {
    int ret;
    uint32_t features;
    struct xen_pmu_params xp;
    ret = kstrtou32(buffer, 0, &features);
    if (ret)
    return ret;
    xp.val = features;
    xp.version.maj = XENPMU_VER_MAJ;
    xp.version.min = XENPMU_VER_MIN;
    ret = HYPERVISOR_xenpmu_op(XENPMU_feature_set, &xp);
    if (ret)
    return ret;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn pmu_features_show(attr: *mut hyp_sysfs_attr, buffer: *mut c_char) -> isize {
    static ssize_t pmu_features_show(struct hyp_sysfs_attr *attr, char *buffer)
    {
    int ret;
    struct xen_pmu_params xp;
    xp.version.maj = XENPMU_VER_MAJ;
    xp.version.min = XENPMU_VER_MIN;
    ret = HYPERVISOR_xenpmu_op(XENPMU_feature_get, &xp);
    if (ret)
    return ret;
    return sprintf(buffer, "0x%x\n", (uint32_t)xp.val);
    }
    HYPERVISOR_ATTR_RW(pmu_features);
    static struct attribute *xen_pmu_attrs[] = {
    &pmu_mode_attr.attr,
    &pmu_features_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group xen_pmu_group = {
    .name = "pmu",
    .attrs = xen_pmu_attrs,
    };
#[no_mangle]
unsafe extern "C" fn xen_sysfs_pmu_init() -> int __init {
    static int __init xen_sysfs_pmu_init(void)
    {
    return sysfs_create_group(hypervisor_kobj, &xen_pmu_group);
    }

#[no_mangle]
unsafe extern "C" fn hyper_sysfs_init() -> int __init {
    static int __init hyper_sysfs_init(void)
    {
    int ret;
    if (!xen_domain())
    return -ENODEV;
    ret = xen_sysfs_type_init();
    if (ret)
    goto out;
    ret = xen_sysfs_guest_type_init();
    if (ret)
    goto guest_type_out;
    ret = xen_sysfs_version_init();
    if (ret)
    goto version_out;
    ret = xen_sysfs_compilation_init();
    if (ret)
    goto comp_out;
    ret = xen_sysfs_uuid_init();
    if (ret)
    goto uuid_out;
    ret = xen_sysfs_properties_init();
    if (ret)
    goto prop_out;
    ret = xen_sysfs_flags_init();
    if (ret)
    goto flags_out;

    if (xen_initial_domain()) {
    ret = xen_sysfs_pmu_init();
    if (ret) {
    sysfs_remove_group(hypervisor_kobj, &xen_flags_group);
    goto flags_out;
    }
    }

    goto out;
    flags_out:
    sysfs_remove_group(hypervisor_kobj, &xen_properties_group);
    prop_out:
    sysfs_remove_file(hypervisor_kobj, &uuid_attr.attr);
    uuid_out:
    sysfs_remove_group(hypervisor_kobj, &xen_compilation_group);
    comp_out:
    sysfs_remove_group(hypervisor_kobj, &version_group);
    version_out:
    sysfs_remove_file(hypervisor_kobj, &guest_type_attr.attr);
    guest_type_out:
    sysfs_remove_file(hypervisor_kobj, &type_attr.attr);
    out:
    return ret;
    }
    device_initcall(hyper_sysfs_init);
    static ssize_t hyp_sysfs_show(struct kobject *kobj,
    struct attribute *attr,
    char *buffer)
    {
    struct hyp_sysfs_attr *hyp_attr;
    hyp_attr = container_of(attr, struct hyp_sysfs_attr, attr);
    if (hyp_attr.show)
    return hyp_attr.show(hyp_attr, buffer);
    return 0;
    }
    static ssize_t hyp_sysfs_store(struct kobject *kobj,
    struct attribute *attr,
    const char *buffer,
    size_t len)
    {
    struct hyp_sysfs_attr *hyp_attr;
    hyp_attr = container_of(attr, struct hyp_sysfs_attr, attr);
    if (hyp_attr.store)
    return hyp_attr.store(hyp_attr, buffer, len);
    return 0;
    }
    static const struct sysfs_ops hyp_sysfs_ops = {
    .show = hyp_sysfs_show,
    .store = hyp_sysfs_store,
    };
    static const struct kobj_type hyp_sysfs_kobj_type = {
    .sysfs_ops = &hyp_sysfs_ops,
    };
#[no_mangle]
unsafe extern "C" fn hypervisor_subsys_init() -> int __init {
    static int __init hypervisor_subsys_init(void)
    {
    if (!xen_domain())
    return -ENODEV;
    hypervisor_kobj.ktype = &hyp_sysfs_kobj_type;
    return 0;
    }
    device_initcall(hypervisor_subsys_init);
