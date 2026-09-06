//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/plr_tpmi.c
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
// Performance Limit Reasons via TPMI
//
// Copyright (c) 2024, Intel Corporation.
//

pub const PLR_HEADER: c_uint = 0x00;
pub const PLR_MAILBOX_INTERFACE: c_uint = 0x08;
pub const PLR_MAILBOX_DATA: c_uint = 0x10;
pub const PLR_DIE_LEVEL: c_uint = 0x18;

pub const PLR_COMMAND_WRITE: c_int = 1;

pub const PLR_TIMEOUT_US: c_int = 5;
pub const PLR_TIMEOUT_MAX_US: c_int = 1000;
pub const PLR_COARSE_REASON_BITS: c_int = 32;
    struct tpmi_plr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpmi_plr_die {
    pub base: *mut void __iomem,
    pub /: *mut *mut mutex lock; / Protect access to PLR mailbox,
    pub package_id: c_int,
    pub die_id: c_int,
    pub plr: *mut tpmi_plr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpmi_plr {
    pub dbgfs_dir: *mut dentry,
    pub die_info: *mut tpmi_plr_die,
    pub num_dies: c_int,
    pub auxdev: *mut auxiliary_device,
    pub nb: notifier_block,
    pub /: *mut *mut mutex lock; / Protect access to dbgfs_dir,
}

    static const char * const plr_coarse_reasons[] = {
    "FREQUENCY",
    "CURRENT",
    "POWER",
    "THERMAL",
    "PLATFORM",
    "MCP",
    "RAS",
    "MISC",
    "QOS",
    "DFC",
    };
    static const char * const plr_fine_reasons[] = {
    "FREQUENCY_CDYN0",
    "FREQUENCY_CDYN1",
    "FREQUENCY_CDYN2",
    "FREQUENCY_CDYN3",
    "FREQUENCY_CDYN4",
    "FREQUENCY_CDYN5",
    "FREQUENCY_FCT",
    "FREQUENCY_PCS_TRL",
    "CURRENT_MTPMAX",
    "POWER_FAST_RAPL",
    "POWER_PKG_PL1_MSR_TPMI",
    "POWER_PKG_PL1_MMIO",
    "POWER_PKG_PL1_PCS",
    "POWER_PKG_PL2_MSR_TPMI",
    "POWER_PKG_PL2_MMIO",
    "POWER_PKG_PL2_PCS",
    "POWER_PLATFORM_PL1_MSR_TPMI",
    "POWER_PLATFORM_PL1_MMIO",
    "POWER_PLATFORM_PL1_PCS",
    "POWER_PLATFORM_PL2_MSR_TPMI",
    "POWER_PLATFORM_PL2_MMIO",
    "POWER_PLATFORM_PL2_PCS",
    "UNKNOWN(22)",
    "THERMAL_PER_CORE",
    "DFC_UFS",
    "PLATFORM_PROCHOT",
    "PLATFORM_HOT_VR",
    "UNKNOWN(27)",
    "UNKNOWN(28)",
    "MISC_PCS_PSTATE",
    };
#[no_mangle]
unsafe extern "C" fn plr_read(plr_die: *mut tpmi_plr_die, offset: c_int) -> u64 {
    static u64 plr_read(struct tpmi_plr_die *plr_die, int offset)
    {
    return readq(plr_die.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn plr_write(val: u64, plr_die: *mut tpmi_plr_die, offset: c_int) {
    static void plr_write(u64 val, struct tpmi_plr_die *plr_die, int offset)
    {
    writeq(val, plr_die.base + offset);
    }
    static int plr_read_cpu_status(struct tpmi_plr_die *plr_die, int cpu,
    u64 *status)
    {
    u64 regval;
    int ret;
    lockdep_assert_held(&plr_die.lock);
    regval = FIELD_PREP(PLR_MODULE_ID_MASK, tpmi_get_punit_core_number(cpu));
    regval |= PLR_RUN_BUSY;
    plr_write(regval, plr_die, PLR_MAILBOX_INTERFACE);
    ret = readq_poll_timeout(plr_die.base + PLR_MAILBOX_INTERFACE, regval,
    !(regval & PLR_RUN_BUSY), PLR_TIMEOUT_US,
    PLR_TIMEOUT_MAX_US);
    if (ret)
    return ret;
// status = plr_read(plr_die, PLR_MAILBOX_DATA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plr_clear_cpu_status(plr_die: *mut tpmi_plr_die, cpu: c_int) -> c_int {
    static int plr_clear_cpu_status(struct tpmi_plr_die *plr_die, int cpu)
    {
    u64 regval;
    lockdep_assert_held(&plr_die.lock);
    regval = FIELD_PREP(PLR_MODULE_ID_MASK, tpmi_get_punit_core_number(cpu));
    regval |= PLR_RUN_BUSY | PLR_COMMAND_WRITE;
    plr_write(0, plr_die, PLR_MAILBOX_DATA);
    plr_write(regval, plr_die, PLR_MAILBOX_INTERFACE);
    return readq_poll_timeout(plr_die.base + PLR_MAILBOX_INTERFACE, regval,
    !(regval & PLR_RUN_BUSY), PLR_TIMEOUT_US,
    PLR_TIMEOUT_MAX_US);
    }
#[no_mangle]
unsafe extern "C" fn plr_print_bits(s: *mut seq_file, val: u64, bits: c_int) {
    static void plr_print_bits(struct seq_file *s, u64 val, int bits)
    {
    const unsigned long mask[] = { BITMAP_FROM_U64(val) };
    int bit, index;
    for_each_set_bit(bit, mask, bits) {
    const char *str = core::ptr::null_mut();
    if (bit < PLR_COARSE_REASON_BITS) {
    if (bit < ARRAY_SIZE(plr_coarse_reasons))
    str = plr_coarse_reasons[bit];
    } else {
    index = bit - PLR_COARSE_REASON_BITS;
    if (index < ARRAY_SIZE(plr_fine_reasons))
    str = plr_fine_reasons[index];
    }
    if (str)
    seq_printf(s, " %s", str);
    else
    seq_printf(s, " UNKNOWN(%d)", bit);
    }
    if (!val)
    seq_puts(s, " none");
    seq_putc(s, '\n');
    }
#[no_mangle]
unsafe extern "C" fn plr_status_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int plr_status_show(struct seq_file *s, void *unused)
    {
    struct tpmi_plr_die *plr_die = s.private;
    int ret;
    u64 val;
    val = plr_read(plr_die, PLR_DIE_LEVEL);
    seq_puts(s, "cpus");
    plr_print_bits(s, val, 32);
    guard(mutex)(&plr_die.lock);
    for (int cpu = 0; cpu < nr_cpu_ids; cpu++) {
    if (plr_die.die_id != tpmi_get_power_domain_id(cpu))
    continue;
    if (plr_die.package_id != topology_physical_package_id(cpu))
    continue;
    seq_printf(s, "cpu%d", cpu);
    ret = plr_read_cpu_status(plr_die, cpu, &val);
    if (ret) {
    dev_err(&plr_die.plr.auxdev.dev, "Failed to read PLR for cpu %d, ret=%d\n",
    cpu, ret);
    return ret;
    }
    plr_print_bits(s, val, 64);
    }
    return 0;
    }
    static ssize_t plr_status_write(struct file *filp, const char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct seq_file *s = filp.private_data;
    struct tpmi_plr_die *plr_die = s.private;
    bool val;
    int ret;
    ret = kstrtobool_from_user(ubuf, count, &val);
    if (ret)
    return ret;
    if (val != 0)
    return -EINVAL;
    plr_write(0, plr_die, PLR_DIE_LEVEL);
    guard(mutex)(&plr_die.lock);
    for (int cpu = 0; cpu < nr_cpu_ids; cpu++) {
    if (plr_die.die_id != tpmi_get_power_domain_id(cpu))
    continue;
    if (plr_die.package_id != topology_physical_package_id(cpu))
    continue;
    plr_clear_cpu_status(plr_die, cpu);
    }
    return count;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(plr_status);
#[no_mangle]
unsafe extern "C" fn intel_plr_notify(self: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    static int intel_plr_notify(struct notifier_block *self, unsigned long action, void *data)
    {
    struct tpmi_plr *plr = container_of(self, struct tpmi_plr, nb);
    if (action == TPMI_CORE_EXIT) {
    guard(mutex)(&plr.lock);
    plr.dbgfs_dir = core::ptr::null_mut();
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn intel_plr_register_notifier(nb: *mut notifier_block) -> c_int {
    static int intel_plr_register_notifier(struct notifier_block *nb)
    {
    nb.notifier_call = intel_plr_notify;
    nb.priority = 0;
    return tpmi_register_notifier(nb);
    }
#[no_mangle]
unsafe extern "C" fn intel_plr_unregister_notifier(nb: *mut notifier_block) {
    static void intel_plr_unregister_notifier(struct notifier_block *nb)
    {
    tpmi_unregister_notifier(nb);
    }
#[no_mangle]
unsafe extern "C" fn intel_plr_probe(auxdev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int intel_plr_probe(struct auxiliary_device *auxdev, const struct auxiliary_device_id *id)
    {
    struct oobmsm_plat_info *plat_info;
    struct dentry *dentry;
    int i, num_resources;
    struct resource *res;
    struct tpmi_plr *plr;
    void __iomem *base;
    char name[17];
    int err;
    plat_info = tpmi_get_platform_data(auxdev);
    if (!plat_info)
    return dev_err_probe(&auxdev.dev, -EINVAL, "No platform info\n");
    dentry = tpmi_get_debugfs_dir(auxdev);
    if (!dentry)
    return dev_err_probe(&auxdev.dev, -ENODEV, "No TPMI debugfs directory.\n");
    num_resources = tpmi_get_resource_count(auxdev);
    if (!num_resources)
    return -EINVAL;
    plr = devm_kzalloc(&auxdev.dev, sizeof(*plr), GFP_KERNEL);
    if (!plr)
    return -ENOMEM;
    err = devm_mutex_init(&auxdev.dev, &plr.lock);
    if (err)
    return err;
    intel_plr_register_notifier(&plr.nb);
    plr.die_info = devm_kcalloc(&auxdev.dev, num_resources, sizeof(*plr.die_info),
    GFP_KERNEL);
    if (!plr.die_info) {
    err = -ENOMEM;
    goto err_notify;
    }
    plr.num_dies = num_resources;
    plr.dbgfs_dir = debugfs_create_dir("plr", dentry);
    plr.auxdev = auxdev;
    for (i = 0; i < num_resources; i++) {
    res = tpmi_get_resource_at_index(auxdev, i);
    if (!res) {
    err = dev_err_probe(&auxdev.dev, -EINVAL, "No resource\n");
    goto err;
    }
    base = devm_ioremap_resource(&auxdev.dev, res);
    if (IS_ERR(base)) {
    err = PTR_ERR(base);
    goto err;
    }
    plr.die_info[i].base = base;
    plr.die_info[i].package_id = plat_info.package_id;
    plr.die_info[i].die_id = i;
    plr.die_info[i].plr = plr;
    mutex_init(&plr.die_info[i].lock);
    if (plr_read(&plr.die_info[i], PLR_HEADER) == PLR_INVALID)
    continue;
    snprintf(name, sizeof(name), "domain%d", i);
    dentry = debugfs_create_dir(name, plr.dbgfs_dir);
    debugfs_create_file("status", 0644, dentry, &plr.die_info[i],
    &plr_status_fops);
    }
    auxiliary_set_drvdata(auxdev, plr);
    return 0;
    err:
    debugfs_remove_recursive(plr.dbgfs_dir);
    err_notify:
    intel_plr_unregister_notifier(&plr.nb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_plr_remove(auxdev: *mut auxiliary_device) {
    static void intel_plr_remove(struct auxiliary_device *auxdev)
    {
    struct tpmi_plr *plr = auxiliary_get_drvdata(auxdev);
    intel_plr_unregister_notifier(&plr.nb);
    guard(mutex)(&plr.lock);
    debugfs_remove_recursive(plr.dbgfs_dir);
    }
    static const struct auxiliary_device_id intel_plr_id_table[] = {
    { .name = "intel_vsec.tpmi-plr" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, intel_plr_id_table);
    static struct auxiliary_driver intel_plr_aux_driver = {
    .id_table       = intel_plr_id_table,
    .remove         = intel_plr_remove,
    .probe          = intel_plr_probe,
    };
    module_auxiliary_driver(intel_plr_aux_driver);
    MODULE_IMPORT_NS("INTEL_TPMI");
    MODULE_IMPORT_NS("INTEL_TPMI_POWER_DOMAIN");
    MODULE_DESCRIPTION("Intel TPMI PLR Driver");
    MODULE_LICENSE("GPL");
