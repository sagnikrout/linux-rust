//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/amd/debugfs.c
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
// AMD IOMMU driver
//
// Copyright (C) 2018 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <gary.hook@amd.com>
//

    static struct dentry *amd_iommu_debugfs;
pub const MAX_NAME_LEN: c_int = 20;
pub const OFS_IN_SZ: c_int = 8;
pub const DEVID_IN_SZ: c_int = 16;
    let mut sbdf: static int = -1;
    static ssize_t iommu_mmio_write(struct file *filp, const char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    struct seq_file *m = filp.private_data;
    struct amd_iommu *iommu = m.private;
    int ret, dbg_mmio_offset = iommu.dbg_mmio_offset = -1;
    if (cnt > OFS_IN_SZ)
    return -EINVAL;
    ret = kstrtos32_from_user(ubuf, cnt, 0, &dbg_mmio_offset);
    if (ret)
    return ret;
    if (dbg_mmio_offset < 0 || dbg_mmio_offset >
    iommu.mmio_phys_end - sizeof(u64))
    return -EINVAL;
    iommu.dbg_mmio_offset = dbg_mmio_offset;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn iommu_mmio_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int iommu_mmio_show(struct seq_file *m, void *unused)
    {
    struct amd_iommu *iommu = m.private;
    u64 value;
    let mut dbg_mmio_offset: c_int = iommu.dbg_mmio_offset;
    if (dbg_mmio_offset < 0 || dbg_mmio_offset >
    iommu.mmio_phys_end - sizeof(u64)) {
    seq_puts(m, "Please provide mmio register's offset\n");
    return 0;
    }
    value = readq(iommu.mmio_base + dbg_mmio_offset);
    seq_printf(m, "Offset:0x%x Value:0x%016llx\n", dbg_mmio_offset, value);
    return 0;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(iommu_mmio);
    static ssize_t iommu_capability_write(struct file *filp, const char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    struct seq_file *m = filp.private_data;
    struct amd_iommu *iommu = m.private;
    int ret, dbg_cap_offset = iommu.dbg_cap_offset = -1;
    if (cnt > OFS_IN_SZ)
    return -EINVAL;
    ret = kstrtos32_from_user(ubuf, cnt, 0, &dbg_cap_offset);
    if (ret)
    return ret;
// Capability register at offset 0x14 is the last IOMMU capability register.
    if (dbg_cap_offset < 0 || dbg_cap_offset > 0x14)
    return -EINVAL;
    iommu.dbg_cap_offset = dbg_cap_offset;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn iommu_capability_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int iommu_capability_show(struct seq_file *m, void *unused)
    {
    struct amd_iommu *iommu = m.private;
    u32 value;
    int err, dbg_cap_offset = iommu.dbg_cap_offset;
    if (dbg_cap_offset < 0 || dbg_cap_offset > 0x14) {
    seq_puts(m, "Please provide capability register's offset in the range [0x00 - 0x14]\n");
    return 0;
    }
    err = pci_read_config_dword(iommu.dev, iommu.cap_ptr + dbg_cap_offset, &value);
    if (err) {
    seq_printf(m, "Not able to read capability register at 0x%x\n",
    dbg_cap_offset);
    return 0;
    }
    seq_printf(m, "Offset:0x%x Value:0x%08x\n", dbg_cap_offset, value);
    return 0;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(iommu_capability);
#[no_mangle]
unsafe extern "C" fn iommu_cmdbuf_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int iommu_cmdbuf_show(struct seq_file *m, void *unused)
    {
    struct amd_iommu *iommu = m.private;
    struct iommu_cmd *cmd;
    unsigned long flag;
    u32 head, tail;
    int i;
    raw_spin_lock_irqsave(&iommu.lock, flag);
    head = readl(iommu.mmio_base + MMIO_CMD_HEAD_OFFSET);
    tail = readl(iommu.mmio_base + MMIO_CMD_TAIL_OFFSET);
    seq_printf(m, "CMD Buffer Head Offset:%d Tail Offset:%d\n",
    (head >> 4) & 0x7fff, (tail >> 4) & 0x7fff);
    for (i = 0; i < CMD_BUFFER_ENTRIES; i++) {
    cmd = (struct iommu_cmd *)(iommu.cmd_buf + i * sizeof(*cmd));
    seq_printf(m, "%3d: %08x %08x %08x %08x\n", i, cmd.data[0],
    cmd.data[1], cmd.data[2], cmd.data[3]);
    }
    raw_spin_unlock_irqrestore(&iommu.lock, flag);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(iommu_cmdbuf);
    static ssize_t devid_write(struct file *filp, const char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    struct amd_iommu_pci_seg *pci_seg;
    int seg, bus, slot, func;
    struct amd_iommu *iommu;
    char *srcid_ptr;
    u16 devid;
    int i;
    sbdf = -1;
    if (cnt >= DEVID_IN_SZ)
    return -EINVAL;
    srcid_ptr = memdup_user_nul(ubuf, cnt);
    if (IS_ERR(srcid_ptr))
    return PTR_ERR(srcid_ptr);
    i = sscanf(srcid_ptr, "%x:%x:%x.%x", &seg, &bus, &slot, &func);
    if (i != 4) {
    i = sscanf(srcid_ptr, "%x:%x.%x", &bus, &slot, &func);
    if (i != 3) {
    kfree(srcid_ptr);
    return -EINVAL;
    }
    seg = 0;
    }
    devid = PCI_DEVID(bus, PCI_DEVFN(slot, func));
// Check if user device id input is a valid input
    for_each_pci_segment(pci_seg) {
    if (pci_seg.id != seg)
    continue;
    if (devid > pci_seg.last_bdf) {
    kfree(srcid_ptr);
    return -EINVAL;
    }
    iommu = pci_seg.rlookup_table[devid];
    if (!iommu) {
    kfree(srcid_ptr);
    return -ENODEV;
    }
    sbdf = PCI_SEG_DEVID_TO_SBDF(seg, devid);
    kfree(srcid_ptr);
    return cnt;
    }
    kfree(srcid_ptr);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn devid_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int devid_show(struct seq_file *m, void *unused)
    {
    u16 devid;
    let mut sbdf_shadow: c_int = sbdf;
    if (sbdf_shadow >= 0) {
    devid = PCI_SBDF_TO_DEVID(sbdf_shadow);
    seq_printf(m, "%04x:%02x:%02x.%x\n", PCI_SBDF_TO_SEGID(sbdf_shadow),
    PCI_BUS_NUM(devid), PCI_SLOT(devid), PCI_FUNC(devid));
    } else
    seq_puts(m, "No or Invalid input provided\n");
    return 0;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(devid);
#[no_mangle]
unsafe extern "C" fn dump_dte(m: *mut seq_file, pci_seg: *mut amd_iommu_pci_seg, devid: u16) {
    static void dump_dte(struct seq_file *m, struct amd_iommu_pci_seg *pci_seg, u16 devid)
    {
    struct dev_table_entry *dev_table;
    struct amd_iommu *iommu;
    iommu = pci_seg.rlookup_table[devid];
    if (!iommu)
    return;
    dev_table = get_dev_table(iommu);
    if (!dev_table) {
    seq_puts(m, "Device table not found");
    return;
    }
    seq_printf(m, "%-12s %16s %16s %16s %16s iommu\n", "DeviceId",
    "QWORD[3]", "QWORD[2]", "QWORD[1]", "QWORD[0]");
    seq_printf(m, "%04x:%02x:%02x.%x ", pci_seg.id, PCI_BUS_NUM(devid),
    PCI_SLOT(devid), PCI_FUNC(devid));
    for (int i = 3; i >= 0; --i)
    seq_printf(m, "%016llx ", dev_table[devid].data[i]);
    seq_printf(m, "iommu%d\n", iommu.index);
    }
#[no_mangle]
unsafe extern "C" fn iommu_devtbl_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int iommu_devtbl_show(struct seq_file *m, void *unused)
    {
    struct amd_iommu_pci_seg *pci_seg;
    u16 seg, devid;
    let mut sbdf_shadow: c_int = sbdf;
    if (sbdf_shadow < 0) {
    seq_puts(m, "Enter a valid device ID to 'devid' file\n");
    return 0;
    }
    seg = PCI_SBDF_TO_SEGID(sbdf_shadow);
    devid = PCI_SBDF_TO_DEVID(sbdf_shadow);
    for_each_pci_segment(pci_seg) {
    if (pci_seg.id != seg)
    continue;
    dump_dte(m, pci_seg, devid);
    break;
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(iommu_devtbl);
#[no_mangle]
unsafe extern "C" fn dump_128_irte(m: *mut seq_file, table: *mut irq_remap_table, int_tab_len: u16) {
    static void dump_128_irte(struct seq_file *m, struct irq_remap_table *table, u16 int_tab_len)
    {
    struct irte_ga *ptr, *irte;
    int index;
    for (index = 0; index < int_tab_len; index++) {
    ptr = (struct irte_ga *)table.table;
    irte = &ptr[index];
    if (AMD_IOMMU_GUEST_IR_VAPIC(amd_iommu_guest_ir) &&
    !irte.lo.fields_vapic.valid)
    continue;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !irte->lo.fields_remap.valid) -> else {
    else if (!irte.lo.fields_remap.valid)
    continue;
    seq_printf(m, "IRT[%04d] %016llx %016llx\n", index, irte.hi.val, irte.lo.val);
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_32_irte(m: *mut seq_file, table: *mut irq_remap_table, int_tab_len: u16) {
    static void dump_32_irte(struct seq_file *m, struct irq_remap_table *table, u16 int_tab_len)
    {
    union irte *ptr, *irte;
    int index;
    for (index = 0; index < int_tab_len; index++) {
    ptr = (union irte *)table.table;
    irte = &ptr[index];
    if (!irte.fields.valid)
    continue;
    seq_printf(m, "IRT[%04d] %08x\n", index, irte.val);
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_irte(m: *mut seq_file, devid: u16, pci_seg: *mut amd_iommu_pci_seg) {
    static void dump_irte(struct seq_file *m, u16 devid, struct amd_iommu_pci_seg *pci_seg)
    {
    struct dev_table_entry *dev_table;
    struct irq_remap_table *table;
    struct amd_iommu *iommu;
    unsigned long flags;
    u16 int_tab_len;
    table = pci_seg.irq_lookup_table[devid];
    if (!table) {
    seq_printf(m, "IRQ lookup table not set for %04x:%02x:%02x:%x\n",
    pci_seg.id, PCI_BUS_NUM(devid), PCI_SLOT(devid), PCI_FUNC(devid));
    return;
    }
    iommu = pci_seg.rlookup_table[devid];
    if (!iommu)
    return;
    dev_table = get_dev_table(iommu);
    if (!dev_table) {
    seq_puts(m, "Device table not found");
    return;
    }
    int_tab_len = dev_table[devid].data[2] & DTE_INTTABLEN_MASK;
    if (int_tab_len != DTE_INTTABLEN_512 && int_tab_len != DTE_INTTABLEN_2K) {
    seq_puts(m, "The device's DTE contains an invalid IRT length value.");
    return;
    }
    seq_printf(m, "DeviceId %04x:%02x:%02x.%x\n", pci_seg.id, PCI_BUS_NUM(devid),
    PCI_SLOT(devid), PCI_FUNC(devid));
    raw_spin_lock_irqsave(&table.lock, flags);
    if (AMD_IOMMU_GUEST_IR_GA(amd_iommu_guest_ir))
    dump_128_irte(m, table, BIT(int_tab_len >> 1));
    else
    dump_32_irte(m, table, BIT(int_tab_len >> 1));
    seq_puts(m, "\n");
    raw_spin_unlock_irqrestore(&table.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iommu_irqtbl_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int iommu_irqtbl_show(struct seq_file *m, void *unused)
    {
    struct amd_iommu_pci_seg *pci_seg;
    u16 devid, seg;
    let mut sbdf_shadow: c_int = sbdf;
    if (!irq_remapping_enabled) {
    seq_puts(m, "Interrupt remapping is disabled\n");
    return 0;
    }
    if (sbdf_shadow < 0) {
    seq_puts(m, "Enter a valid device ID to 'devid' file\n");
    return 0;
    }
    seg = PCI_SBDF_TO_SEGID(sbdf_shadow);
    devid = PCI_SBDF_TO_DEVID(sbdf_shadow);
    for_each_pci_segment(pci_seg) {
    if (pci_seg.id != seg)
    continue;
    dump_irte(m, devid, pci_seg);
    break;
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(iommu_irqtbl);
#[no_mangle]
pub unsafe extern "C" fn amd_iommu_debugfs_setup() {
    void amd_iommu_debugfs_setup(void)
    {
    struct amd_iommu *iommu;
    char name[MAX_NAME_LEN + 1];
    amd_iommu_debugfs = debugfs_create_dir("amd", iommu_debugfs_dir);
    for_each_iommu(iommu) {
    iommu.dbg_mmio_offset = -1;
    iommu.dbg_cap_offset = -1;
    snprintf(name, MAX_NAME_LEN, "iommu%02d", iommu.index);
    iommu.debugfs = debugfs_create_dir(name, amd_iommu_debugfs);
    debugfs_create_file("mmio", 0644, iommu.debugfs, iommu,
    &iommu_mmio_fops);
    debugfs_create_file("capability", 0644, iommu.debugfs, iommu,
    &iommu_capability_fops);
    debugfs_create_file("cmdbuf", 0444, iommu.debugfs, iommu,
    &iommu_cmdbuf_fops);
    }
    debugfs_create_file("devid", 0644, amd_iommu_debugfs, core::ptr::null_mut(),
    &devid_fops);
    debugfs_create_file("devtbl", 0444, amd_iommu_debugfs, core::ptr::null_mut(),
    &iommu_devtbl_fops);
    debugfs_create_file("irqtbl", 0444, amd_iommu_debugfs, core::ptr::null_mut(),
    &iommu_irqtbl_fops);
    }
