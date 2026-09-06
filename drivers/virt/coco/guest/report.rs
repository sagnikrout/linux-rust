//! Automatically rewritten from C to Rust
//! Source: drivers/virt/coco/guest/report.c
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
// Copyright(c) 2023 Intel Corporation. All rights reserved.

    static struct tsm_provider {
    const struct tsm_report_ops *ops;
    void *data;
    atomic_t count;
    } provider;
    static DECLARE_RWSEM(tsm_rwsem);
//
// DOC: Trusted Security Module (TSM) Attestation Report Interface
//
// The TSM report interface is a common provider of blobs that facilitate
// attestation of a TVM (confidential computing guest) by an attestation
// service. A TSM report combines a user-defined blob (likely a public-key with
// a nonce for a key-exchange protocol) with a signed attestation report. That
// combined blob is then used to obtain secrets provided by an agent that can
// validate the attestation report. The expectation is that this interface is
// invoked infrequently, however configfs allows for multiple agents to
// own their own report generation instances to generate reports as
// often as needed.
//
// The attestation report format is TSM provider specific, when / if a standard
// materializes that can be published instead of the vendor layout. Until then
// the 'provider' attribute indicates the format of 'outblob', and optionally
// 'auxblob' and 'manifestblob'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_report_state {
    pub report: tsm_report,
    pub write_generation: c_ulong,
    pub read_generation: c_ulong,
    pub cfg: config_item,
}

    enum tsm_data_select {
    TSM_REPORT,
    TSM_CERTS,
    TSM_MANIFEST,
    };
    static struct tsm_report *to_tsm_report(struct config_item *cfg)
    {
    struct tsm_report_state *state =
    container_of(cfg, struct tsm_report_state, cfg);
    return &state.report;
    }
    static struct tsm_report_state *to_state(struct tsm_report *report)
    {
    return container_of(report, struct tsm_report_state, report);
    }
#[no_mangle]
unsafe extern "C" fn try_advance_write_generation(report: *mut tsm_report) -> c_int {
    static int try_advance_write_generation(struct tsm_report *report)
    {
    struct tsm_report_state *state = to_state(report);
    lockdep_assert_held_write(&tsm_rwsem);
//
// Malicious or broken userspace has written enough times for
// read_generation == write_generation by modular arithmetic without an
// interim read. Stop accepting updates until the current report
// configuration is read.
//
    if (state.write_generation == state.read_generation - 1)
    return -EBUSY;
    state.write_generation++;
    return 0;
    }
    static ssize_t tsm_report_privlevel_store(struct config_item *cfg,
    const char *buf, size_t len)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    unsigned int val;
    int rc;
    rc = kstrtouint(buf, 0, &val);
    if (rc)
    return rc;
    guard(rwsem_write)(&tsm_rwsem);
    if (!provider.ops)
    return -ENXIO;
//
// The valid privilege levels that a TSM might accept, if it accepts a
// privilege level setting at all, are a max of TSM_PRIVLEVEL_MAX (see
// SEV-SNP GHCB) and a minimum of a TSM selected floor value no less
// than 0.
//
    if (provider.ops.privlevel_floor > val || val > TSM_REPORT_PRIVLEVEL_MAX)
    return -EINVAL;
    rc = try_advance_write_generation(report);
    if (rc)
    return rc;
    report.desc.privlevel = val;
    return len;
    }
    CONFIGFS_ATTR_WO(tsm_report_, privlevel);
    static ssize_t tsm_report_privlevel_floor_show(struct config_item *cfg,
    char *buf)
    {
    guard(rwsem_read)(&tsm_rwsem);
    if (!provider.ops)
    return -ENXIO;
    return sysfs_emit(buf, "%u\n", provider.ops.privlevel_floor);
    }
    CONFIGFS_ATTR_RO(tsm_report_, privlevel_floor);
    static ssize_t tsm_report_service_provider_store(struct config_item *cfg,
    const char *buf, size_t len)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    size_t sp_len;
    char *sp;
    int rc;
    guard(rwsem_write)(&tsm_rwsem);
    rc = try_advance_write_generation(report);
    if (rc)
    return rc;
    sp_len = (buf[len - 1] != '\n') ? len : len - 1;
    sp = kstrndup(buf, sp_len, GFP_KERNEL);
    if (!sp)
    return -ENOMEM;
    kfree(report.desc.service_provider);
    report.desc.service_provider = sp;
    return len;
    }
    CONFIGFS_ATTR_WO(tsm_report_, service_provider);
    static ssize_t tsm_report_service_guid_store(struct config_item *cfg,
    const char *buf, size_t len)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    int rc;
    guard(rwsem_write)(&tsm_rwsem);
    rc = try_advance_write_generation(report);
    if (rc)
    return rc;
    report.desc.service_guid = guid_null;
    rc = guid_parse(buf, &report.desc.service_guid);
    if (rc)
    return rc;
    return len;
    }
    CONFIGFS_ATTR_WO(tsm_report_, service_guid);
    static ssize_t tsm_report_service_manifest_version_store(struct config_item *cfg,
    const char *buf, size_t len)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    unsigned int val;
    int rc;
    rc = kstrtouint(buf, 0, &val);
    if (rc)
    return rc;
    guard(rwsem_write)(&tsm_rwsem);
    rc = try_advance_write_generation(report);
    if (rc)
    return rc;
    report.desc.service_manifest_version = val;
    return len;
    }
    CONFIGFS_ATTR_WO(tsm_report_, service_manifest_version);
    static ssize_t tsm_report_inblob_write(struct config_item *cfg,
    const void *buf, size_t count)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    int rc;
    guard(rwsem_write)(&tsm_rwsem);
    rc = try_advance_write_generation(report);
    if (rc)
    return rc;
    report.desc.inblob_len = count;
    memcpy(report.desc.inblob, buf, count);
    return count;
    }
    CONFIGFS_BIN_ATTR_WO(tsm_report_, inblob, core::ptr::null_mut(), TSM_REPORT_INBLOB_MAX);
#[no_mangle]
unsafe extern "C" fn tsm_report_generation_show(cfg: *mut config_item, buf: *mut c_char) -> isize {
    static ssize_t tsm_report_generation_show(struct config_item *cfg, char *buf)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    struct tsm_report_state *state = to_state(report);
    guard(rwsem_read)(&tsm_rwsem);
    return sysfs_emit(buf, "%lu\n", state.write_generation);
    }
    CONFIGFS_ATTR_RO(tsm_report_, generation);
#[no_mangle]
unsafe extern "C" fn tsm_report_provider_show(cfg: *mut config_item, buf: *mut c_char) -> isize {
    static ssize_t tsm_report_provider_show(struct config_item *cfg, char *buf)
    {
    guard(rwsem_read)(&tsm_rwsem);
    if (!provider.ops)
    return -ENXIO;
    return sysfs_emit(buf, "%s\n", provider.ops.name);
    }
    CONFIGFS_ATTR_RO(tsm_report_, provider);
    static ssize_t __read_report(struct tsm_report *report, void *buf, size_t count,
    enum tsm_data_select select)
    {
    let mut offset: loff_t = 0;
    ssize_t len;
    u8 *out;
    if (select == TSM_REPORT) {
    out = report.outblob;
    len = report.outblob_len;
    } else if (select == TSM_MANIFEST) {
    out = report.manifestblob;
    len = report.manifestblob_len;
    } else {
    out = report.auxblob;
    len = report.auxblob_len;
    }
//
// Recall that a NULL @buf is configfs requesting the size of
// the buffer.
//
    if (!buf)
    return len;
    return memory_read_from_buffer(buf, count, &offset, out, len);
    }
    static ssize_t read_cached_report(struct tsm_report *report, void *buf,
    size_t count, enum tsm_data_select select)
    {
    struct tsm_report_state *state = to_state(report);
    guard(rwsem_read)(&tsm_rwsem);
    if (!report.desc.inblob_len)
    return -EINVAL;
//
// A given TSM backend always fills in ->outblob regardless of
// whether the report includes an auxblob/manifestblob or not.
//
    if (!report.outblob ||
    state.read_generation != state.write_generation)
    return -EWOULDBLOCK;
    return __read_report(report, buf, count, select);
    }
    static ssize_t tsm_report_read(struct tsm_report *report, void *buf,
    size_t count, enum tsm_data_select select)
    {
    struct tsm_report_state *state = to_state(report);
    const struct tsm_report_ops *ops;
    ssize_t rc;
// try to read from the existing report if present and valid...
    rc = read_cached_report(report, buf, count, select);
    if (rc >= 0 || rc != -EWOULDBLOCK)
    return rc;
// slow path, report may need to be regenerated...
    guard(rwsem_write)(&tsm_rwsem);
    ops = provider.ops;
    if (!ops)
    return -ENXIO;
    if (!report.desc.inblob_len)
    return -EINVAL;
// did another thread already generate this report?
    if (report.outblob &&
    state.read_generation == state.write_generation)
    goto out;
    kvfree(report.outblob);
    kvfree(report.auxblob);
    kvfree(report.manifestblob);
    report.outblob = core::ptr::null_mut();
    report.auxblob = core::ptr::null_mut();
    report.manifestblob = core::ptr::null_mut();
    rc = ops.report_new(report, provider.data);
    if (rc < 0)
    return rc;
    state.read_generation = state.write_generation;
    out:
    return __read_report(report, buf, count, select);
    }
    static ssize_t tsm_report_outblob_read(struct config_item *cfg, void *buf,
    size_t count)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    return tsm_report_read(report, buf, count, TSM_REPORT);
    }
    CONFIGFS_BIN_ATTR_RO(tsm_report_, outblob, core::ptr::null_mut(), TSM_REPORT_OUTBLOB_MAX);
    static ssize_t tsm_report_auxblob_read(struct config_item *cfg, void *buf,
    size_t count)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    return tsm_report_read(report, buf, count, TSM_CERTS);
    }
    CONFIGFS_BIN_ATTR_RO(tsm_report_, auxblob, core::ptr::null_mut(), TSM_REPORT_OUTBLOB_MAX);
    static ssize_t tsm_report_manifestblob_read(struct config_item *cfg, void *buf,
    size_t count)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    return tsm_report_read(report, buf, count, TSM_MANIFEST);
    }
    CONFIGFS_BIN_ATTR_RO(tsm_report_, manifestblob, core::ptr::null_mut(), TSM_REPORT_OUTBLOB_MAX);
    static struct configfs_attribute *tsm_report_attrs[] = {
    [TSM_REPORT_GENERATION] = &tsm_report_attr_generation,
    [TSM_REPORT_PROVIDER] = &tsm_report_attr_provider,
    [TSM_REPORT_PRIVLEVEL] = &tsm_report_attr_privlevel,
    [TSM_REPORT_PRIVLEVEL_FLOOR] = &tsm_report_attr_privlevel_floor,
    [TSM_REPORT_SERVICE_PROVIDER] = &tsm_report_attr_service_provider,
    [TSM_REPORT_SERVICE_GUID] = &tsm_report_attr_service_guid,
    [TSM_REPORT_SERVICE_MANIFEST_VER] = &tsm_report_attr_service_manifest_version,
    core::ptr::null_mut(),
    };
    static struct configfs_bin_attribute *tsm_report_bin_attrs[] = {
    [TSM_REPORT_INBLOB] = &tsm_report_attr_inblob,
    [TSM_REPORT_OUTBLOB] = &tsm_report_attr_outblob,
    [TSM_REPORT_AUXBLOB] = &tsm_report_attr_auxblob,
    [TSM_REPORT_MANIFESTBLOB] = &tsm_report_attr_manifestblob,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn tsm_report_item_release(cfg: *mut config_item) {
    static void tsm_report_item_release(struct config_item *cfg)
    {
    struct tsm_report *report = to_tsm_report(cfg);
    struct tsm_report_state *state = to_state(report);
    kvfree(report.manifestblob);
    kvfree(report.auxblob);
    kvfree(report.outblob);
    kfree(report.desc.service_provider);
    kfree(state);
    }
    static struct configfs_item_operations tsm_report_item_ops = {
    .release = tsm_report_item_release,
    };
    static bool tsm_report_is_visible(struct config_item *item,
    struct configfs_attribute *attr, int n)
    {
    guard(rwsem_read)(&tsm_rwsem);
    if (!provider.ops)
    return false;
    if (!provider.ops.report_attr_visible)
    return true;
    return provider.ops.report_attr_visible(n);
    }
    static bool tsm_report_is_bin_visible(struct config_item *item,
    struct configfs_bin_attribute *attr, int n)
    {
    guard(rwsem_read)(&tsm_rwsem);
    if (!provider.ops)
    return false;
    if (!provider.ops.report_bin_attr_visible)
    return true;
    return provider.ops.report_bin_attr_visible(n);
    }
    static struct configfs_group_operations tsm_report_attr_group_ops = {
    .is_visible = tsm_report_is_visible,
    .is_bin_visible = tsm_report_is_bin_visible,
    };
    static const struct config_item_type tsm_report_type = {
    .ct_owner = THIS_MODULE,
    .ct_bin_attrs = tsm_report_bin_attrs,
    .ct_attrs = tsm_report_attrs,
    .ct_item_ops = &tsm_report_item_ops,
    .ct_group_ops = &tsm_report_attr_group_ops,
    };
    static struct config_item *tsm_report_make_item(struct config_group *group,
    const char *name)
    {
    struct tsm_report_state *state;
    guard(rwsem_read)(&tsm_rwsem);
    if (!provider.ops)
    return ERR_PTR(-ENXIO);
    state = kzalloc_obj(*state);
    if (!state)
    return ERR_PTR(-ENOMEM);
    atomic_inc(&provider.count);
    config_item_init_type_name(&state.cfg, name, &tsm_report_type);
    return &state.cfg;
    }
#[no_mangle]
unsafe extern "C" fn tsm_report_drop_item(group: *mut config_group, item: *mut config_item) {
    static void tsm_report_drop_item(struct config_group *group, struct config_item *item)
    {
    config_item_put(item);
    atomic_dec(&provider.count);
    }
    static struct configfs_group_operations tsm_report_group_ops = {
    .make_item = tsm_report_make_item,
    .drop_item = tsm_report_drop_item,
    };
    static const struct config_item_type tsm_reports_type = {
    .ct_owner = THIS_MODULE,
    .ct_group_ops = &tsm_report_group_ops,
    };
    static const struct config_item_type tsm_root_group_type = {
    .ct_owner = THIS_MODULE,
    };
    static struct configfs_subsystem tsm_configfs = {
    .su_group = {
    .cg_item = {
    .ci_namebuf = "tsm",
    .ci_type = &tsm_root_group_type,
    },
    },
    .su_mutex = __MUTEX_INITIALIZER(tsm_configfs.su_mutex),
    };
#[no_mangle]
pub unsafe extern "C" fn tsm_report_register(ops: *const tsm_report_ops, priv: *mut c_void) -> c_int {
    int tsm_report_register(const struct tsm_report_ops *ops, void *priv)
    {
    const struct tsm_report_ops *conflict;
    guard(rwsem_write)(&tsm_rwsem);
    conflict = provider.ops;
    if (conflict) {
    pr_err("\"%s\" ops already registered\n", conflict.name);
    return -EBUSY;
    }
    if (atomic_read(&provider.count)) {
    pr_err("configfs/tsm/report not empty\n");
    return -EBUSY;
    }
    provider.ops = ops;
    provider.data = priv;
    return 0;
    }
    EXPORT_SYMBOL_GPL(tsm_report_register);
#[no_mangle]
pub unsafe extern "C" fn tsm_report_unregister(ops: *const tsm_report_ops) -> c_int {
    int tsm_report_unregister(const struct tsm_report_ops *ops)
    {
    guard(rwsem_write)(&tsm_rwsem);
    if (ops != provider.ops)
    return -EBUSY;
    if (atomic_read(&provider.count))
    pr_warn("\"%s\" unregistered with items present in configfs/tsm/report\n",
    provider.ops.name);
    provider.ops = core::ptr::null_mut();
    provider.data = core::ptr::null_mut();
    return 0;
    }
    EXPORT_SYMBOL_GPL(tsm_report_unregister);
    static struct config_group *tsm_report_group;
#[no_mangle]
unsafe extern "C" fn tsm_report_init() -> int __init {
    static int __init tsm_report_init(void)
    {
    struct config_group *root = &tsm_configfs.su_group;
    struct config_group *tsm;
    int rc;
    config_group_init(root);
    rc = configfs_register_subsystem(&tsm_configfs);
    if (rc)
    return rc;
    tsm = configfs_register_default_group(root, "report",
    &tsm_reports_type);
    if (IS_ERR(tsm)) {
    configfs_unregister_subsystem(&tsm_configfs);
    return PTR_ERR(tsm);
    }
    tsm_report_group = tsm;
    return 0;
    }
    module_init(tsm_report_init);
#[no_mangle]
unsafe extern "C" fn tsm_report_exit() -> void __exit {
    static void __exit tsm_report_exit(void)
    {
    configfs_unregister_default_group(tsm_report_group);
    configfs_unregister_subsystem(&tsm_configfs);
    }
    module_exit(tsm_report_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Provide Trusted Security Module attestation reports via configfs");
