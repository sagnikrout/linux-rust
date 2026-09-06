//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/qcom_stats.c
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
// Copyright (c) 2011-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2022-2025, Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const RPM_DYNAMIC_ADDR: c_uint = 0x14;
pub const RPM_DYNAMIC_ADDR_MASK: c_uint = 0xFFFF;
pub const DDR_STATS_MAGIC_KEY: c_uint = 0xA1157A75;
pub const DDR_STATS_MAX_NUM_MODES: c_int = 20;
pub const DDR_STATS_MAGIC_KEY_ADDR: c_uint = 0x0;
pub const DDR_STATS_NUM_MODES_ADDR: c_uint = 0x4;
pub const DDR_STATS_ENTRY_START_ADDR: c_uint = 0x8;

    static struct qmp *qcom_stats_qmp;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsystem_data {
    pub name: *const c_char,
    pub smem_item: u32,
    pub pid: u32,
}

    static const struct subsystem_data subsystems[] = {
    { "modem", 605, 1 },
    { "wpss", 605, 13 },
    { "adsp", 606, 2 },
    { "cdsp", 607, 5 },
    { "cdsp1", 607, 12 },
    { "gpdsp0", 607, 17 },
    { "gpdsp1", 607, 18 },
    { "soccp", 607, 19 },
    { "dcp", 607, 22 },
    { "slpi", 608, 3 },
    { "gpu", 609, 0 },
    { "display", 610, 0 },
    { "adsp_island", 613, 2 },
    { "slpi_island", 613, 3 },
    { "apss", 631, QCOM_SMEM_HOST_ANY },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_config {
    pub stats_offset: usize,
    pub ddr_stats_offset: usize,
    pub num_records: usize,
    pub appended_stats_avail: bool,
    pub dynamic_offset: bool,
    pub subsystem_stats_in_smem: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_stats_entry {
    pub name: u32,
    pub count: u32,
    pub duration: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_data {
    pub appended_stats_avail: bool,
    pub base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_stats {
    pub stat_type: u32,
    pub count: u32,
    pub last_entered_at: u64,
    pub last_exited_at: u64,
    pub accumulated: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct appended_stats {
    pub client_votes: u32,
    pub reserved: [u32; 3],
}

#[no_mangle]
unsafe extern "C" fn qcom_print_stats(s: *mut seq_file, stat: *const sleep_stats) {
    static void qcom_print_stats(struct seq_file *s, const struct sleep_stats *stat)
    {
    let mut accumulated: u64 = stat.accumulated;
//
// If a subsystem is in sleep when reading the sleep stats adjust
// the accumulated sleep duration to show actual sleep time.
//
    if (stat.last_entered_at > stat.last_exited_at)
    accumulated += arch_timer_read_counter() - stat.last_entered_at;
    seq_printf(s, "Count: %u\n", stat.count);
    seq_printf(s, "Last Entered At: %llu\n", stat.last_entered_at);
    seq_printf(s, "Last Exited At: %llu\n", stat.last_exited_at);
    seq_printf(s, "Accumulated Duration: %llu\n", accumulated);
    }
#[no_mangle]
unsafe extern "C" fn qcom_subsystem_sleep_stats_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int qcom_subsystem_sleep_stats_show(struct seq_file *s, void *unused)
    {
    struct subsystem_data *subsystem = s.private;
    struct sleep_stats *stat;
// Items are allocated lazily, so lookup pointer each time
    stat = qcom_smem_get(subsystem.pid, subsystem.smem_item, core::ptr::null_mut());
    if (IS_ERR(stat))
    return 0;
    qcom_print_stats(s, stat);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_soc_sleep_stats_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int qcom_soc_sleep_stats_show(struct seq_file *s, void *unused)
    {
    struct stats_data *d = s.private;
    void __iomem *reg = d.base;
    struct sleep_stats stat;
    memcpy_fromio(&stat, reg, sizeof(stat));
    qcom_print_stats(s, &stat);
    if (d.appended_stats_avail) {
    struct appended_stats votes;
    memcpy_fromio(&votes, reg + sizeof(struct sleep_stats), sizeof(votes));
    seq_printf(s, "Client Votes: %#x\n", votes.client_votes);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ddr_stats_print(s: *mut seq_file, data: *mut ddr_stats_entry) {
    static void qcom_ddr_stats_print(struct seq_file *s, struct ddr_stats_entry *data)
    {
    u32 cp_idx;
//
// DDR statistic have two different types of details encoded.
// (1) DDR LPM Stats
// (2) DDR Frequency Stats
//
// The name field have details like which type of DDR stat (bits 8:15)
// along with other details as explained below
//
// In case of DDR LPM stat, name field will be encoded as,
// Bits	 -  Meaning
// 0:7	 -  DDR LPM name, can be of 0xd4, 0xd3, 0x11 and 0xd0.
// 8:15	 -  0x0 (indicates its a LPM stat)
// 16:31 -  Unused
//
// In case of DDR FREQ stats, name field will be encoded as,
// Bits  -  Meaning
// 0:4   -  DDR Clock plan index (CP IDX)
// 5:7   -  Unused
// 8:15  -  0x1 (indicates its Freq stat)
// 16:31 -  Frequency value in Mhz
//
    switch (DDR_STATS_TYPE(data.name)) {
    case 0:
    seq_printf(s, "DDR LPM Stat Name:0x%lx\tcount:%u\tDuration (ticks):%llu\n",
    DDR_STATS_LPM_NAME(data.name), data.count, data.duration);
    break;
    case 1:
    if (!data.count || !DDR_STATS_FREQ(data.name))
    return;
    cp_idx = DDR_STATS_CP_IDX(data.name);
    seq_printf(s, "DDR Freq %luMhz:\tCP IDX:%u\tcount:%u\tDuration (ticks):%llu\n",
    DDR_STATS_FREQ(data.name), cp_idx, data.count, data.duration);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_ddr_stats_show(s: *mut seq_file, d: *mut c_void) -> c_int {
    static int qcom_ddr_stats_show(struct seq_file *s, void *d)
    {
    struct ddr_stats_entry data[DDR_STATS_MAX_NUM_MODES];
    void __iomem *reg = (void __iomem *)s.private;
    u32 entry_count;
    int i, ret;
    entry_count = readl_relaxed(reg + DDR_STATS_NUM_MODES_ADDR);
    if (entry_count > DDR_STATS_MAX_NUM_MODES)
    return -EINVAL;
    if (qcom_stats_qmp) {
//
// Recent SoCs (SM8450 onwards) do not have duration field
// populated from boot up onwards for both DDR LPM Stats
// and DDR Frequency Stats.
//
// Send QMP message to Always on processor which will
// populate duration field into MSG RAM area.
//
// Sent every time to read latest data.
//
    ret = qmp_send(qcom_stats_qmp, "{class: ddr, action: freqsync}");
    if (ret)
    return ret;
    }
    reg += DDR_STATS_ENTRY_START_ADDR;
    memcpy_fromio(data, reg, sizeof(struct ddr_stats_entry) * entry_count);
    for (i = 0; i < entry_count; i++)
    qcom_ddr_stats_print(s, &data[i]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(qcom_soc_sleep_stats);
    DEFINE_SHOW_ATTRIBUTE(qcom_subsystem_sleep_stats);
    DEFINE_SHOW_ATTRIBUTE(qcom_ddr_stats);
    static void qcom_create_ddr_stat_files(struct dentry *root, void __iomem *reg,
    const struct stats_config *config)
    {
    u32 key;
    if (!config.ddr_stats_offset)
    return;
    key = readl_relaxed(reg + config.ddr_stats_offset + DDR_STATS_MAGIC_KEY_ADDR);
    if (key == DDR_STATS_MAGIC_KEY)
    debugfs_create_file("ddr_stats", 0400, root,
    ( void *)reg + config.ddr_stats_offset,
    &qcom_ddr_stats_fops);
    }
    static void qcom_create_soc_sleep_stat_files(struct dentry *root, void __iomem *reg,
    struct stats_data *d,
    const struct stats_config *config)
    {
    char stat_type[sizeof(u32) + 1] = {0};
    let mut stats_offset: usize = config.stats_offset;
    let mut offset: u32 = 0, type;
    int i, j;
//
// On RPM targets, stats offset location is dynamic and changes from target
// to target and sometimes from build to build for same target.
//
// In such cases the dynamic address is present at 0x14 offset from base
// address in devicetree. The last 16bits indicates the stats_offset.
//
    if (config.dynamic_offset) {
    stats_offset = readl(reg + RPM_DYNAMIC_ADDR);
    stats_offset &= RPM_DYNAMIC_ADDR_MASK;
    }
    for (i = 0; i < config.num_records; i++) {
    d[i].base = reg + offset + stats_offset;
//
// Read the low power mode name and create debugfs file for it.
// The names read could be of below,
// (may change depending on low power mode supported).
// For rpmh-sleep-stats: "aosd", "cxsd" and "ddr".
// For rpm-sleep-stats: "vmin" and "vlow".
//
    type = readl(d[i].base);
    for (j = 0; j < sizeof(u32); j++) {
    stat_type[j] = type & 0xff;
    type = type >> 8;
    }
    strim(stat_type);
    debugfs_create_file(stat_type, 0400, root, &d[i],
    &qcom_soc_sleep_stats_fops);
    offset += sizeof(struct sleep_stats);
    if (d[i].appended_stats_avail)
    offset += sizeof(struct appended_stats);
    }
    }
    static void qcom_create_subsystem_stat_files(struct dentry *root,
    const struct stats_config *config)
    {
    int i;
    if (!config.subsystem_stats_in_smem)
    return;
    for (i = 0; i < ARRAY_SIZE(subsystems); i++)
    debugfs_create_file(subsystems[i].name, 0400, root, (void *)&subsystems[i],
    &qcom_subsystem_sleep_stats_fops);
    }
#[no_mangle]
unsafe extern "C" fn qcom_stats_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_stats_probe(struct platform_device *pdev)
    {
    void __iomem *reg;
    struct dentry *root;
    const struct stats_config *config;
    struct stats_data *d;
    int i;
    config = device_get_match_data(&pdev.dev);
    if (!config)
    return -ENODEV;
    reg = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(reg))
    return -ENOMEM;
    d = devm_kcalloc(&pdev.dev, config.num_records,
    sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    for (i = 0; i < config.num_records; i++)
    d[i].appended_stats_avail = config.appended_stats_avail;
//
// QMP is used for DDR stats syncing to MSG RAM for recent SoCs (SM8450 onwards).
// The prior SoCs do not need QMP handle as the required stats are already present
// in MSG RAM, provided the DDR_STATS_MAGIC_KEY matches.
//
    qcom_stats_qmp = qmp_get(&pdev.dev);
    if (IS_ERR(qcom_stats_qmp)) {
// We ignore error if QMP is not defined/needed
    if (!of_property_present(pdev.dev.of_node, "qcom,qmp"))
    qcom_stats_qmp = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(-EPROBE_DEFER: PTR_ERR(qcom_stats_qmp) ==) -> else {
    else if (PTR_ERR(qcom_stats_qmp) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    else
    return PTR_ERR(qcom_stats_qmp);
    }
    root = debugfs_create_dir("qcom_stats", core::ptr::null_mut());
    qcom_create_subsystem_stat_files(root, config);
    qcom_create_soc_sleep_stat_files(root, reg, d, config);
    qcom_create_ddr_stat_files(root, reg, config);
    platform_set_drvdata(pdev, root);
    device_set_pm_not_required(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_stats_remove(pdev: *mut platform_device) {
    static void qcom_stats_remove(struct platform_device *pdev)
    {
    struct dentry *root = platform_get_drvdata(pdev);
    debugfs_remove_recursive(root);
    }
    static const struct stats_config rpm_data = {
    .stats_offset = 0,
    .num_records = 2,
    .appended_stats_avail = true,
    .dynamic_offset = true,
    .subsystem_stats_in_smem = false,
    };
// Older RPM firmwares have the stats at a fixed offset instead
    static const struct stats_config rpm_data_dba0 = {
    .stats_offset = 0xdba0,
    .num_records = 2,
    .appended_stats_avail = true,
    .dynamic_offset = false,
    .subsystem_stats_in_smem = false,
    };
    static const struct stats_config rpmh_data_sdm845 = {
    .stats_offset = 0x48,
    .num_records = 2,
    .appended_stats_avail = false,
    .dynamic_offset = false,
    .subsystem_stats_in_smem = true,
    };
    static const struct stats_config rpmh_data = {
    .stats_offset = 0x48,
    .ddr_stats_offset = 0xb8,
    .num_records = 3,
    .appended_stats_avail = false,
    .dynamic_offset = false,
    .subsystem_stats_in_smem = true,
    };
    static const struct of_device_id qcom_stats_table[] = {
    { .compatible = "qcom,apq8084-rpm-stats", .data = &rpm_data_dba0 },
    { .compatible = "qcom,msm8226-rpm-stats", .data = &rpm_data_dba0 },
    { .compatible = "qcom,msm8916-rpm-stats", .data = &rpm_data_dba0 },
    { .compatible = "qcom,msm8974-rpm-stats", .data = &rpm_data_dba0 },
    { .compatible = "qcom,rpm-stats", .data = &rpm_data },
    { .compatible = "qcom,rpmh-stats", .data = &rpmh_data },
    { .compatible = "qcom,sdm845-rpmh-stats", .data = &rpmh_data_sdm845 },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_stats_table);
    static struct platform_driver qcom_stats = {
    .probe = qcom_stats_probe,
    .remove = qcom_stats_remove,
    .driver = {
    .name = "qcom_stats",
    .of_match_table = qcom_stats_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_stats_init() -> int __init {
    static int __init qcom_stats_init(void)
    {
    return platform_driver_register(&qcom_stats);
    }
    late_initcall(qcom_stats_init);
#[no_mangle]
unsafe extern "C" fn qcom_stats_exit() -> void __exit {
    static void __exit qcom_stats_exit(void)
    {
    platform_driver_unregister(&qcom_stats);
    }
    module_exit(qcom_stats_exit)
    MODULE_DESCRIPTION("Qualcomm Technologies, Inc. (QTI) Stats driver");
    MODULE_LICENSE("GPL v2");
