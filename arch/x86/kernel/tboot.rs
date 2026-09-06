//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/tboot.c
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
// tboot.c: main implementation of helper functions used by kernel for
// runtime support of Intel(R) Trusted Execution Technology
//
// Copyright (c) 2006-2009, Intel Corporation
//

// Global pointer to shared data; NULL means no measured launch.
    static struct tboot *tboot __read_mostly;
// timeout for APs (in secs) to enter wait-for-SIPI state during shutdown
pub const AP_WAIT_TIMEOUT: c_int = 1;

    static u8 tboot_uuid[16] __initdata = TBOOT_UUID;
#[no_mangle]
pub unsafe extern "C" fn tboot_enabled() -> bool {
    bool tboot_enabled(void)
    {
    return tboot != core::ptr::null_mut();
    }
// noinline to prevent gcc from warning about dereferencing constant fixaddr
#[no_mangle]
unsafe extern "C" fn check_tboot_version() -> noinline __init bool {
    static noinline __init bool check_tboot_version(void)
    {
    if (memcmp(&tboot_uuid, &tboot.uuid, sizeof(tboot.uuid))) {
    pr_warn("tboot at 0x%llx is invalid\n", boot_params.tboot_addr);
    return false;
    }
    if (tboot.version < 5) {
    pr_warn("tboot version is invalid: %u\n", tboot.version);
    return false;
    }
    pr_info("found shared page at phys addr 0x%llx:\n",
    boot_params.tboot_addr);
    pr_debug("version: %d\n", tboot.version);
    pr_debug("log_addr: 0x%08x\n", tboot.log_addr);
    pr_debug("shutdown_entry: 0x%x\n", tboot.shutdown_entry);
    pr_debug("tboot_base: 0x%08x\n", tboot.tboot_base);
    pr_debug("tboot_size: 0x%x\n", tboot.tboot_size);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn tboot_probe() -> void __init {
    void __init tboot_probe(void)
    {
// Look for valid page-aligned address for shared page.
    if (!boot_params.tboot_addr)
    return;
//
// also verify that it is mapped as we expect it before calling
// set_fixmap(), to reduce chance of garbage value causing crash
//
    if (!e820__mapped_any(boot_params.tboot_addr,
    boot_params.tboot_addr, E820_TYPE_RESERVED)) {
    pr_warn("non-0 tboot_addr but it is not of type E820_TYPE_RESERVED\n");
    return;
    }
// Map and check for tboot UUID.
    set_fixmap(FIX_TBOOT_BASE, boot_params.tboot_addr);
    tboot = (void *)fix_to_virt(FIX_TBOOT_BASE);
    if (!check_tboot_version())
    tboot = core::ptr::null_mut();
    }
    static pgd_t *tboot_pg_dir;
    static struct mm_struct tboot_mm = {
    .mm_mt          = MTREE_INIT_EXT(mm_mt, MM_MT_FLAGS, tboot_mm.mmap_lock),
    .pgd            = swapper_pg_dir,
    .mm_users       = ATOMIC_INIT(2),
    .mm_count       = ATOMIC_INIT(1),
    .write_protect_seq = SEQCNT_ZERO(tboot_mm.write_protect_seq),
    MMAP_LOCK_INITIALIZER(init_mm)
    .page_table_lock =  __SPIN_LOCK_UNLOCKED(init_mm.page_table_lock),
    .mmlist         = LIST_HEAD_INIT(init_mm.mmlist),
    };
#[no_mangle]
pub unsafe extern "C" fn switch_to_tboot_pt() {
    static inline void switch_to_tboot_pt(void)
    {
    write_cr3(virt_to_phys(tboot_pg_dir));
    }
    static int map_tboot_page(unsigned long vaddr, unsigned long pfn,
    pgprot_t prot)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    pgd = pgd_offset(&tboot_mm, vaddr);
    p4d = p4d_alloc(&tboot_mm, pgd, vaddr);
    if (!p4d)
    return -1;
    pud = pud_alloc(&tboot_mm, p4d, vaddr);
    if (!pud)
    return -1;
    pmd = pmd_alloc(&tboot_mm, pud, vaddr);
    if (!pmd)
    return -1;
    pte = pte_alloc_map(&tboot_mm, pmd, vaddr);
    if (!pte)
    return -1;
    set_pte_at(&tboot_mm, vaddr, pte, pfn_pte(pfn, prot));
    pte_unmap(pte);
//
// PTI poisons low addresses in the kernel page tables in the
// name of making them unusable for userspace.  To execute
// code at such a low address, the poison must be cleared.
//
// Note: 'pgd' actually gets set in p4d_alloc() _or_
// pud_alloc() depending on 4/5-level paging.
//
    pgd.pgd &= ~_PAGE_NX;
    return 0;
    }
    static int map_tboot_pages(unsigned long vaddr, unsigned long start_pfn,
    unsigned long nr)
    {
// Reuse the original kernel mapping
    tboot_pg_dir = pgd_alloc(&tboot_mm);
    if (!tboot_pg_dir)
    return -1;
    for (; nr > 0; nr--, vaddr += PAGE_SIZE, start_pfn++) {
    if (map_tboot_page(vaddr, start_pfn, PAGE_KERNEL_EXEC))
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tboot_create_trampoline() {
    static void tboot_create_trampoline(void)
    {
    u32 map_base, map_size;
// Create identity map for tboot shutdown code.
    map_base = PFN_DOWN(tboot.tboot_base);
    map_size = PFN_UP(tboot.tboot_size);
    if (map_tboot_pages(map_base << PAGE_SHIFT, map_base, map_size))
    panic("tboot: Error mapping tboot pages (mfns) @ 0x%x, 0x%x\n",
    map_base, map_size);
    }

#[no_mangle]
unsafe extern "C" fn add_mac_region(start: phys_addr_t, size: c_ulong) {
    static void add_mac_region(phys_addr_t start, unsigned long size)
    {
    struct tboot_mac_region *mr;
    let mut end: phys_addr_t = start + size;
    if (tboot.num_mac_regions >= MAX_TB_MAC_REGIONS)
    panic("tboot: Too many MAC regions\n");
    if (start && size) {
    mr = &tboot.mac_regions[tboot.num_mac_regions++];
    mr.start = round_down(start, PAGE_SIZE);
    mr.size  = round_up(end, PAGE_SIZE) - mr.start;
    }
    }
#[no_mangle]
unsafe extern "C" fn tboot_setup_sleep() -> c_int {
    static int tboot_setup_sleep(void)
    {
    int i;
    tboot.num_mac_regions = 0;
    for (i = 0; i < e820_table.nr_entries; i++) {
    if (e820_table.entries[i].type != E820_TYPE_RAM)
    continue;
    add_mac_region(e820_table.entries[i].addr, e820_table.entries[i].size);
    }
    tboot.acpi_sinfo.kernel_s3_resume_vector =
    real_mode_header.wakeup_start;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tboot_setup_sleep() -> c_int {
    static int tboot_setup_sleep(void)
    {
// S3 shutdown requested, but S3 not supported by the kernel...
    BUG();
    return -1;
    }

#[no_mangle]
pub unsafe extern "C" fn tboot_shutdown(shutdown_type: u32) {
    void tboot_shutdown(u32 shutdown_type)
    {
    void (*shutdown)(void);
    if (!tboot_enabled())
    return;
//
// if we're being called before the 1:1 mapping is set up then just
// return and let the normal shutdown happen; this should only be
// due to very early panic()
//
    if (!tboot_pg_dir)
    return;
// if this is S3 then set regions to MAC
    if (shutdown_type == TB_SHUTDOWN_S3)
    if (tboot_setup_sleep())
    return;
    tboot.shutdown_type = shutdown_type;
    switch_to_tboot_pt();
    shutdown = (void(*)(void))(unsigned long)tboot.shutdown_entry;
    shutdown();
// should not reach here
    while (1)
    halt();
    }
#[no_mangle]
unsafe extern "C" fn tboot_copy_fadt(fadt: *const acpi_table_fadt) {
    static void tboot_copy_fadt(const struct acpi_table_fadt *fadt)
    {

    tbg.space_id     = g.space_id;		\
    tbg.bit_width    = g.bit_width;		\
    tbg.bit_offset   = g.bit_offset;	\
    tbg.access_width = g.access_width;	\
    tbg.address      = g.address;
    TB_COPY_GAS(tboot.acpi_sinfo.pm1a_cnt_blk, fadt.xpm1a_control_block);
    TB_COPY_GAS(tboot.acpi_sinfo.pm1b_cnt_blk, fadt.xpm1b_control_block);
    TB_COPY_GAS(tboot.acpi_sinfo.pm1a_evt_blk, fadt.xpm1a_event_block);
    TB_COPY_GAS(tboot.acpi_sinfo.pm1b_evt_blk, fadt.xpm1b_event_block);
//
// We need phys addr of waking vector, but can't use virt_to_phys() on
// &acpi_gbl_FACS because it is ioremap'ed, so calc from FACS phys
// addr.
//
    tboot.acpi_sinfo.wakeup_vector = fadt.facs +
    offsetof(struct acpi_table_facs, firmware_waking_vector);
    }
#[no_mangle]
unsafe extern "C" fn tboot_sleep(sleep_state: u8, pm1a_control: u32, pm1b_control: u32) -> c_int {
    static int tboot_sleep(u8 sleep_state, u32 pm1a_control, u32 pm1b_control)
    {
    static u32 acpi_shutdown_map[ACPI_S_STATE_COUNT] = {
// S0,1,2: */ -1, -1, -1,
// S3: */ TB_SHUTDOWN_S3,
// S4: */ TB_SHUTDOWN_S4,
// S5: */ TB_SHUTDOWN_S5 };
    if (!tboot_enabled())
    return 0;
    tboot_copy_fadt(&acpi_gbl_FADT);
    tboot.acpi_sinfo.pm1a_cnt_val = pm1a_control;
    tboot.acpi_sinfo.pm1b_cnt_val = pm1b_control;
// we always use the 32b wakeup vector
    tboot.acpi_sinfo.vector_width = 32;
    if (sleep_state >= ACPI_S_STATE_COUNT ||
    acpi_shutdown_map[sleep_state] == -1) {
    pr_warn("unsupported sleep state 0x%x\n", sleep_state);
    return -1;
    }
    tboot_shutdown(acpi_shutdown_map[sleep_state]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tboot_extended_sleep(sleep_state: u8, val_a: u32, val_b: u32) -> c_int {
    static int tboot_extended_sleep(u8 sleep_state, u32 val_a, u32 val_b)
    {
    if (!tboot_enabled())
    return 0;
    pr_warn("tboot is not able to suspend on platforms with reduced hardware sleep (ACPIv5)");
    return -ENODEV;
    }
    static atomic_t ap_wfs_count;
#[no_mangle]
unsafe extern "C" fn tboot_wait_for_aps(num_aps: c_int) -> c_int {
    static int tboot_wait_for_aps(int num_aps)
    {
    unsigned long timeout;
    timeout = AP_WAIT_TIMEOUT*HZ;
    while (atomic_read((atomic_t *)&tboot.num_in_wfs) != num_aps &&
    timeout) {
    mdelay(1);
    timeout--;
    }
    if (timeout)
    pr_warn("tboot wait for APs timeout\n");
    return !(atomic_read((atomic_t *)&tboot.num_in_wfs) == num_aps);
    }
#[no_mangle]
unsafe extern "C" fn tboot_dying_cpu(cpu: c_uint) -> c_int {
    static int tboot_dying_cpu(unsigned int cpu)
    {
    atomic_inc(&ap_wfs_count);
    if (num_online_cpus() == 1) {
    if (tboot_wait_for_aps(atomic_read(&ap_wfs_count)))
    return -EBUSY;
    }
    return 0;
    }

    0x4c, 0x84, 0xa3, 0xe9, 0x53, 0xb8, 0x81, 0x74 }
pub const TBOOT_SERIAL_LOG_ADDR: c_uint = 0x60000;
pub const TBOOT_SERIAL_LOG_SIZE: c_uint = 0x08000;
pub const LOG_MAX_SIZE_OFF: c_int = 16;
pub const LOG_BUF_OFF: c_int = 24;
    static uint8_t tboot_log_uuid[16] = TBOOT_LOG_UUID;
#[no_mangle]
unsafe extern "C" fn tboot_log_read(file: *mut file, user_buf: *mut char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t tboot_log_read(struct file *file, char __user *user_buf, size_t count, loff_t *ppos)
    {
    void __iomem *log_base;
    u8 log_uuid[16];
    u32 max_size;
    void *kbuf;
    let mut ret: c_int = -EFAULT;
    log_base = ioremap(TBOOT_SERIAL_LOG_ADDR, TBOOT_SERIAL_LOG_SIZE);
    if (!log_base)
    return ret;
    memcpy_fromio(log_uuid, log_base, sizeof(log_uuid));
    if (memcmp(&tboot_log_uuid, log_uuid, sizeof(log_uuid)))
    goto err_iounmap;
    max_size = readl(log_base + LOG_MAX_SIZE_OFF);
    if (*ppos >= max_size) {
    ret = 0;
    goto err_iounmap;
    }
    if (*ppos + count > max_size)
    count = max_size - *ppos;
    kbuf = kmalloc(count, GFP_KERNEL);
    if (!kbuf) {
    ret = -ENOMEM;
    goto err_iounmap;
    }
    memcpy_fromio(kbuf, log_base + LOG_BUF_OFF + *ppos, count);
    if (copy_to_user(user_buf, kbuf, count))
    goto err_kfree;
// ppos += count;
    ret = count;
    err_kfree:
    kfree(kbuf);
    err_iounmap:
    iounmap(log_base);
    return ret;
    }
    static const struct file_operations tboot_log_fops = {
    .read	= tboot_log_read,
    .llseek	= default_llseek,
    };

#[no_mangle]
unsafe extern "C" fn tboot_late_init() -> __init int {
    static __init int tboot_late_init(void)
    {
    if (!tboot_enabled())
    return 0;
    tboot_create_trampoline();
    atomic_set(&ap_wfs_count, 0);
    cpuhp_setup_state(CPUHP_AP_X86_TBOOT_DYING, "x86/tboot:dying", core::ptr::null_mut(),
    tboot_dying_cpu);

    debugfs_create_file("tboot_log", S_IRUSR,
    arch_debugfs_dir, core::ptr::null_mut(), &tboot_log_fops);

    acpi_os_set_prepare_sleep(&tboot_sleep);
    acpi_os_set_prepare_extended_sleep(&tboot_extended_sleep);
    return 0;
    }
    late_initcall(tboot_late_init);
//
// TXT configuration registers (offsets from TXT_{PUB, PRIV}_CONFIG_REGS_BASE)
//
pub const TXT_PUB_CONFIG_REGS_BASE: c_uint = 0xfed30000;
pub const TXT_PRIV_CONFIG_REGS_BASE: c_uint = 0xfed20000;
// # pages for each config regs space - used by fixmap

    TXT_PRIV_CONFIG_REGS_BASE) >> PAGE_SHIFT)
// offsets from pub/priv config space
pub const TXTCR_HEAP_BASE: c_uint = 0x0300;
pub const TXTCR_HEAP_SIZE: c_uint = 0x0308;
pub const SHA1_SIZE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha1_hash {
    pub hash: [u8; SHA1_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sinit_mle_data {
    pub /: *mut *mut u32 version; / currently 6,
    pub bios_acm_id: sha1_hash,
    pub edx_senter_flags: u32,
    pub mseg_valid: u64,
    pub sinit_hash: sha1_hash,
    pub mle_hash: sha1_hash,
    pub stm_hash: sha1_hash,
    pub lcp_policy_hash: sha1_hash,
    pub lcp_policy_control: u32,
    pub rlp_wakeup_addr: u32,
    pub reserved: u32,
    pub num_mdrs: u32,
    pub mdrs_off: u32,
    pub num_vtd_dmars: u32,
    pub vtd_dmars_off: u32,
    pub __packed: },
    struct acpi_table_header *tboot_get_dmar_table(struct acpi_table_header *dmar_tbl)
    {
    pub config: *mut *mut *mut void heap_base, heap_ptr,,
    if (!tboot_enabled())
    pub dmar_tbl: return,
//
// ACPI tables may not be DMA protected by tboot, so use DMAR copy
// SINIT saved in SinitMleData in TXT heap (which is DMA protected)
//
// map config space in order to get heap addr
    config = ioremap(TXT_PUB_CONFIG_REGS_BASE, NR_TXT_CONFIG_PAGES *
    if (!config)
    pub NULL: return,
// now map TXT heap
    heap_base = ioremap(*(u64 *)(config + TXTCR_HEAP_BASE),
// (u64 *)(config + TXTCR_HEAP_SIZE));
    if (!heap_base)
    pub NULL: return,
// walk heap to SinitMleData
// skip BiosData
    pub )heap_base: *mut *mut heap_ptr = heap_base + (u64,
// skip OsMleData
    pub )heap_ptr: *mut *mut heap_ptr += (u64,
// skip OsSinitData
    pub )heap_ptr: *mut *mut heap_ptr += (u64,
// now points to SinitMleDataSize; set to SinitMleData
    pub sizeof(u64): heap_ptr +=,
// get addr of DMAR table
    dmar_tbl = (struct acpi_table_header *)(heap_ptr +
    ((struct sinit_mle_data *)heap_ptr).vtd_dmars_off -
// don't unmap heap because dmar.c needs access to this
    pub dmar_tbl: return,
    }
