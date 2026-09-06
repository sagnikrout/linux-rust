//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/mte.c
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
// Copyright (C) 2020 ARM Ltd.
//

    static DEFINE_PER_CPU_READ_MOSTLY(u64, mte_tcf_preferred);

//
// The asynchronous and asymmetric MTE modes have the same behavior for
// store operations. This flag is set when either of these modes is enabled.
//
    DEFINE_STATIC_KEY_FALSE(mte_async_or_asymm_mode);
    EXPORT_SYMBOL_GPL(mte_async_or_asymm_mode);

#[no_mangle]
pub unsafe extern "C" fn mte_sync_tags(pte: pte_t, nr_pages: c_uint) {
    void mte_sync_tags(pte_t pte, unsigned int nr_pages)
    {
    struct page *page = pte_page(pte);
    struct folio *folio = page_folio(page);
    unsigned long i;
    if (folio_test_hugetlb(folio)) {
    let mut nr: c_ulong = folio_nr_pages(folio);
// Hugetlb MTE flags are set for head page only
    if (folio_try_hugetlb_mte_tagging(folio)) {
    for (i = 0; i < nr; i++, page++)
    mte_clear_page_tags(page_address(page));
    folio_set_hugetlb_mte_tagged(folio);
    }
// ensure the tags are visible before the PTE is set
    smp_wmb();
    return;
    }
// if PG_mte_tagged is set, tags have already been initialised
    for (i = 0; i < nr_pages; i++, page++) {
    if (try_page_mte_tagging(page)) {
    mte_clear_page_tags(page_address(page));
    set_page_mte_tagged(page);
    }
    }
// ensure the tags are visible before the PTE is set
    smp_wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn memcmp_pages(page1: *mut page, page2: *mut page) -> c_int {
    int memcmp_pages(struct page *page1, struct page *page2)
    {
    char *addr1, *addr2;
    int ret;
    addr1 = page_address(page1);
    addr2 = page_address(page2);
    ret = memcmp(addr1, addr2, PAGE_SIZE);
    if (!system_supports_mte() || ret)
    return ret;
//
// If the page content is identical but at least one of the pages is
// tagged, return non-zero to avoid KSM merging. If only one of the
// pages is tagged, __set_ptes() may zero or change the tags of the
// other page via mte_sync_tags().
//
    if (page_mte_tagged(page1) || page_mte_tagged(page2))
    return addr1 != addr2;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __mte_enable_kernel(mode: *const c_char, tcf: c_ulong) {
    static inline void __mte_enable_kernel(const char *mode, unsigned long tcf)
    {
// Enable MTE Sync Mode for EL1.
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_TCF_MASK,
    SYS_FIELD_PREP(SCTLR_EL1, TCF, tcf));
    isb();
    pr_info_once("MTE: enabled in %s mode at EL1\n", mode);
    }

#[no_mangle]
pub unsafe extern "C" fn mte_enable_kernel_sync() {
    void mte_enable_kernel_sync(void)
    {
//
// Make sure we enter this function when no PE has set
// async mode previously.
//
    WARN_ONCE(system_uses_mte_async_or_asymm_mode(),
    "MTE async mode enabled system wide!");
    __mte_enable_kernel("synchronous", SCTLR_EL1_TCF_SYNC);
    }
#[no_mangle]
pub unsafe extern "C" fn mte_enable_kernel_async() {
    void mte_enable_kernel_async(void)
    {
    __mte_enable_kernel("asynchronous", SCTLR_EL1_TCF_ASYNC);
//
// MTE async mode is set system wide by the first PE that
// executes this function.
//
// Note: If in future KASAN acquires a runtime switching
// mode in between sync and async, this strategy needs
// to be reviewed.
//
    if (!system_uses_mte_async_or_asymm_mode())
    static_branch_enable(&mte_async_or_asymm_mode);
    }
#[no_mangle]
pub unsafe extern "C" fn mte_enable_kernel_asymm() {
    void mte_enable_kernel_asymm(void)
    {
    if (cpus_have_cap(ARM64_MTE_ASYMM)) {
    __mte_enable_kernel("asymmetric", SCTLR_EL1_TCF_ASYMM);
//
// MTE asymm mode behaves as async mode for store
// operations. The mode is set system wide by the
// first PE that executes this function.
//
// Note: If in future KASAN acquires a runtime switching
// mode in between sync and async, this strategy needs
// to be reviewed.
//
    if (!system_uses_mte_async_or_asymm_mode())
    static_branch_enable(&mte_async_or_asymm_mode);
    } else {
//
// If the CPU does not support MTE asymmetric mode the
// kernel falls back on synchronous mode which is the
// default for kasan=on.
//
    mte_enable_kernel_sync();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mte_enable_kernel_store_only() -> c_int {
    int mte_enable_kernel_store_only(void)
    {
//
// If the CPU does not support MTE store only,
// the kernel checks all operations.
//
    if (!cpus_have_cap(ARM64_MTE_STORE_ONLY))
    return -EINVAL;
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_TCSO_MASK,
    SYS_FIELD_PREP(SCTLR_EL1, TCSO, 1));
    isb();
    pr_info_once("MTE: enabled store only mode at EL1\n");
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn mte_check_tfsr_el1() {
    void mte_check_tfsr_el1(void)
    {
    let mut tfsr_el1: u64 = read_sysreg_s(SYS_TFSR_EL1);
    if (unlikely(tfsr_el1 & SYS_TFSR_EL1_TF1)) {
//
// Note: isb() is not required after this direct write
// because there is no indirect read subsequent to it
// (per ARM DDI 0487F.c table D13-1).
//
    write_sysreg_s(0, SYS_TFSR_EL1);
    kasan_report_async();
    }
    }

//
// This is where we actually resolve the system and process MTE mode
// configuration into an actual value in SCTLR_EL1 that affects
// userspace.
//
#[no_mangle]
unsafe extern "C" fn mte_update_sctlr_user(task: *mut task_struct) {
    static void mte_update_sctlr_user(struct task_struct *task)
    {
//
// This must be called with preemption disabled and can only be called
// on the current or next task since the CPU must match where the thread
// is going to run. The caller is responsible for calling
// update_sctlr_el1() later in the same preemption disabled block.
//
    let mut sctlr: c_ulong = task.thread.sctlr_user;
    let mut mte_ctrl: c_ulong = task.thread.mte_ctrl;
    unsigned long pref, resolved_mte_tcf;
    pref = __this_cpu_read(mte_tcf_preferred);
//
// If there is no overlap between the system preferred and
// program requested values go with what was requested.
//
    resolved_mte_tcf = (mte_ctrl & pref) ? pref : mte_ctrl;
    sctlr &= ~(SCTLR_EL1_TCF0_MASK | SCTLR_EL1_TCSO0_MASK);
//
// Pick an actual setting. The order in which we check for
// set bits and map into register values determines our
// default order.
//
    if (resolved_mte_tcf & MTE_CTRL_TCF_ASYMM)
    sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, ASYMM);
#[no_mangle]
pub unsafe extern "C" fn if(MTE_CTRL_TCF_ASYNC: resolved_mte_tcf &) -> else {
    else if (resolved_mte_tcf & MTE_CTRL_TCF_ASYNC)
    sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, ASYNC);
#[no_mangle]
pub unsafe extern "C" fn if(MTE_CTRL_TCF_SYNC: resolved_mte_tcf &) -> else {
    else if (resolved_mte_tcf & MTE_CTRL_TCF_SYNC)
    sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, SYNC);
    if (mte_ctrl & MTE_CTRL_STORE_ONLY)
    sctlr |= SYS_FIELD_PREP(SCTLR_EL1, TCSO0, 1);
    task.thread.sctlr_user = sctlr;
    }
#[no_mangle]
unsafe extern "C" fn mte_update_gcr_excl(task: *mut task_struct) {
    static void mte_update_gcr_excl(struct task_struct *task)
    {
//
// SYS_GCR_EL1 will be set to current->thread.mte_ctrl value by
// mte_set_user_gcr() in kernel_exit, but only if KASAN is enabled.
//
    if (kasan_hw_tags_enabled())
    return;
    write_sysreg_s(
    ((task.thread.mte_ctrl >> MTE_CTRL_GCR_USER_EXCL_SHIFT) &
    SYS_GCR_EL1_EXCL_MASK) | SYS_GCR_EL1_RRND,
    SYS_GCR_EL1);
    }

// Only called from assembly, silence sparse
    void __init kasan_hw_tags_enable(struct alt_instr *alt, __le32 *origptr,
    __le32 *updptr, int nr_inst);
    void __init kasan_hw_tags_enable(struct alt_instr *alt, __le32 *origptr,
    __le32 *updptr, int nr_inst)
    {
    BUG_ON(nr_inst != 1); /* Branch . NOP */
    if (kasan_hw_tags_enabled())
// updptr = cpu_to_le32(aarch64_insn_gen_nop());
    }

#[no_mangle]
pub unsafe extern "C" fn mte_thread_init_user() {
    void mte_thread_init_user(void)
    {
    if (!system_supports_mte())
    return;
// clear any pending asynchronous tag fault
    dsb(ish);
    write_sysreg_s(0, SYS_TFSRE0_EL1);
    clear_thread_flag(TIF_MTE_ASYNC_FAULT);
// disable tag checking and reset tag generation mask
    set_mte_ctrl(current, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mte_thread_switch(next: *mut task_struct) {
    void mte_thread_switch(struct task_struct *next)
    {
    if (!system_supports_mte())
    return;
    mte_update_sctlr_user(next);
    mte_update_gcr_excl(next);
// TCO may not have been disabled on exception entry for the current task.
    mte_disable_tco_entry(next);
    if (!system_uses_mte_async_or_asymm_mode())
    return;
//
// Check if an async tag exception occurred at EL1.
//
// Note: On the context switch path we rely on the dsb() present
// in __switch_to() to guarantee that the indirect writes to TFSR_EL1
// are synchronized before this point.
//
    isb();
    mte_check_tfsr_el1();
    }
#[no_mangle]
pub unsafe extern "C" fn mte_cpu_setup() {
    void mte_cpu_setup(void)
    {
    u64 rgsr;
//
// CnP must be enabled only after the MAIR_EL1 register has been set
// up. Inconsistent MAIR_EL1 between CPUs sharing the same TLB may
// lead to the wrong memory type being used for a brief window during
// CPU power-up.
//
// CnP is not a boot feature so MTE gets enabled before CnP, but let's
// make sure that is the case.
//
    BUG_ON(read_sysreg(ttbr0_el1) & TTBRx_EL1_CnP);
    BUG_ON(read_sysreg(ttbr1_el1) & TTBRx_EL1_CnP);
// Normal Tagged memory type at the corresponding MAIR index
    sysreg_clear_set(mair_el1,
    MAIR_ATTRIDX(MAIR_ATTR_MASK, MT_NORMAL_TAGGED),
    MAIR_ATTRIDX(MAIR_ATTR_NORMAL_TAGGED,
    MT_NORMAL_TAGGED));
    write_sysreg_s(KERNEL_GCR_EL1, SYS_GCR_EL1);
//
// If GCR_EL1.RRND=1 is implemented the same way as RRND=0, then
// RGSR_EL1.SEED must be non-zero for IRG to produce
// pseudorandom numbers. As RGSR_EL1 is UNKNOWN out of reset, we
// must initialize it.
//
    rgsr = (read_sysreg(CNTVCT_EL0) & SYS_RGSR_EL1_SEED_MASK) <<
    SYS_RGSR_EL1_SEED_SHIFT;
    if (rgsr == 0)
    rgsr = 1 << SYS_RGSR_EL1_SEED_SHIFT;
    write_sysreg_s(rgsr, SYS_RGSR_EL1);
// clear any pending tag check faults in TFSR*_EL1
    write_sysreg_s(0, SYS_TFSR_EL1);
    write_sysreg_s(0, SYS_TFSRE0_EL1);
    local_flush_tlb_all();
    }
#[no_mangle]
pub unsafe extern "C" fn mte_suspend_enter() {
    void mte_suspend_enter(void)
    {
    if (!system_supports_mte())
    return;
    if (!system_uses_mte_async_or_asymm_mode())
    return;
//
// The barriers are required to guarantee that the indirect writes
// to TFSR_EL1 are synchronized before we report the state.
//
    dsb(nsh);
    isb();
// Report SYS_TFSR_EL1 before suspend entry
    mte_check_tfsr_el1();
    }
#[no_mangle]
pub unsafe extern "C" fn mte_suspend_exit() {
    void mte_suspend_exit(void)
    {
    if (!system_supports_mte())
    return;
    mte_cpu_setup();
    }
#[no_mangle]
pub unsafe extern "C" fn set_mte_ctrl(task: *mut task_struct, arg: c_ulong) -> c_long {
    long set_mte_ctrl(struct task_struct *task, unsigned long arg)
    {
    u64 mte_ctrl = (~((arg & PR_MTE_TAG_MASK) >> PR_MTE_TAG_SHIFT) &
    SYS_GCR_EL1_EXCL_MASK) << MTE_CTRL_GCR_USER_EXCL_SHIFT;
    if (!system_supports_mte())
    return 0;
    if (arg & PR_MTE_TCF_ASYNC)
    mte_ctrl |= MTE_CTRL_TCF_ASYNC;
    if (arg & PR_MTE_TCF_SYNC)
    mte_ctrl |= MTE_CTRL_TCF_SYNC;
//
// If the system supports it and both sync and async modes are
// specified then implicitly enable asymmetric mode.
// Userspace could see a mix of both sync and async anyway due
// to differing or changing defaults on CPUs.
//
    if (cpus_have_cap(ARM64_MTE_ASYMM) &&
    (arg & PR_MTE_TCF_ASYNC) &&
    (arg & PR_MTE_TCF_SYNC))
    mte_ctrl |= MTE_CTRL_TCF_ASYMM;
    if (arg & PR_MTE_STORE_ONLY)
    mte_ctrl |= MTE_CTRL_STORE_ONLY;
    task.thread.mte_ctrl = mte_ctrl;
    if (task == current) {
    preempt_disable();
    mte_update_sctlr_user(task);
    mte_update_gcr_excl(task);
    update_sctlr_el1(task.thread.sctlr_user);
    preempt_enable();
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_mte_ctrl(task: *mut task_struct) -> c_long {
    long get_mte_ctrl(struct task_struct *task)
    {
    unsigned long ret;
    let mut mte_ctrl: u64 = task.thread.mte_ctrl;
    u64 incl = (~mte_ctrl >> MTE_CTRL_GCR_USER_EXCL_SHIFT) &
    SYS_GCR_EL1_EXCL_MASK;
    if (!system_supports_mte())
    return 0;
    ret = incl << PR_MTE_TAG_SHIFT;
    if (mte_ctrl & MTE_CTRL_TCF_ASYNC)
    ret |= PR_MTE_TCF_ASYNC;
    if (mte_ctrl & MTE_CTRL_TCF_SYNC)
    ret |= PR_MTE_TCF_SYNC;
    if (mte_ctrl & MTE_CTRL_STORE_ONLY)
    ret |= PR_MTE_STORE_ONLY;
    return ret;
    }
//
// Access MTE tags in another process' address space as given in mm. Update
// the number of tags copied. Return 0 if any tags copied, error otherwise.
// Inspired by __access_remote_vm().
//
    static int __access_remote_tags(struct mm_struct *mm, unsigned long addr,
    struct iovec *kiov, unsigned int gup_flags)
    {
    void __user *buf = kiov.iov_base;
    let mut len: usize = kiov.iov_len;
    let mut err: c_int = 0;
    let mut write: c_int = gup_flags & FOLL_WRITE;
    if (!access_ok(buf, len))
    return -EFAULT;
    if (mmap_read_lock_killable(mm))
    return -EIO;
    while (len) {
    struct vm_area_struct *vma;
    unsigned long tags, offset;
    void *maddr;
    struct page *page = get_user_page_vma_remote(mm, addr,
    gup_flags, &vma);
    struct folio *folio;
    if (IS_ERR(page)) {
    err = PTR_ERR(page);
    break;
    }
//
// Only copy tags if the page has been mapped as PROT_MTE
// (PG_mte_tagged set). Otherwise the tags are not valid and
// not accessible to user. Moreover, an mprotect(PROT_MTE)
// would cause the existing tags to be cleared if the page
// was never mapped with PROT_MTE.
//
    if (!(vma.vm_flags & VM_MTE)) {
    err = -EOPNOTSUPP;
    put_page(page);
    break;
    }
    folio = page_folio(page);
    if (folio_test_hugetlb(folio))
    WARN_ON_ONCE(!folio_test_hugetlb_mte_tagged(folio) &&
    !is_huge_zero_folio(folio));
    else
    WARN_ON_ONCE(!page_mte_tagged(page) && !is_zero_page(page));
// limit access to the end of the page
    offset = offset_in_page(addr);
    tags = min(len, (PAGE_SIZE - offset) / MTE_GRANULE_SIZE);
    maddr = page_address(page);
    if (write) {
    tags = mte_copy_tags_from_user(maddr + offset, buf, tags);
    set_page_dirty_lock(page);
    } else {
    tags = mte_copy_tags_to_user(buf, maddr + offset, tags);
    }
    put_page(page);
// error accessing the tracer's buffer
    if (!tags)
    break;
    len -= tags;
    buf += tags;
    addr += tags * MTE_GRANULE_SIZE;
    }
    mmap_read_unlock(mm);
// return an error if no tags copied
    kiov.iov_len = buf - kiov.iov_base;
    if (!kiov.iov_len) {
// check for error accessing the tracee's address space
    if (err)
    return -EIO;
    else
    return -EFAULT;
    }
    return 0;
    }
//
// Copy MTE tags in another process' address space at 'addr' to/from tracer's
// iovec buffer. Return 0 on success. Inspired by ptrace_access_vm().
//
    static int access_remote_tags(struct task_struct *tsk, unsigned long addr,
    struct iovec *kiov, unsigned int gup_flags)
    {
    struct mm_struct *mm;
    int ret;
    mm = get_task_mm(tsk);
    if (!mm)
    return -EPERM;
    if (!ptracer_access_allowed(tsk)) {
    mmput(mm);
    return -EPERM;
    }
    ret = __access_remote_tags(mm, addr, kiov, gup_flags);
    mmput(mm);
    return ret;
    }
    int mte_ptrace_copy_tags(struct task_struct *child, long request,
    unsigned long addr, unsigned long data)
    {
    int ret;
    struct iovec kiov;
    struct iovec __user *uiov = (void __user *)data;
    let mut gup_flags: c_uint = FOLL_FORCE;
    if (!system_supports_mte())
    return -EIO;
    if (get_user(kiov.iov_base, &uiov.iov_base) ||
    get_user(kiov.iov_len, &uiov.iov_len))
    return -EFAULT;
    if (request == PTRACE_POKEMTETAGS)
    gup_flags |= FOLL_WRITE;
// align addr to the MTE tag granule
    addr &= MTE_GRANULE_MASK;
    ret = access_remote_tags(child, addr, &kiov, gup_flags);
    if (!ret)
    ret = put_user(kiov.iov_len, &uiov.iov_len);
    return ret;
    }
    static ssize_t mte_tcf_preferred_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    switch (per_cpu(mte_tcf_preferred, dev.id)) {
    case MTE_CTRL_TCF_ASYNC:
    return sysfs_emit(buf, "async\n");
    case MTE_CTRL_TCF_SYNC:
    return sysfs_emit(buf, "sync\n");
    case MTE_CTRL_TCF_ASYMM:
    return sysfs_emit(buf, "asymm\n");
    default:
    return sysfs_emit(buf, "???\n");
    }
    }
    static ssize_t mte_tcf_preferred_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u64 tcf;
    if (sysfs_streq(buf, "async"))
    tcf = MTE_CTRL_TCF_ASYNC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "sync")) -> else {
    else if (sysfs_streq(buf, "sync"))
    tcf = MTE_CTRL_TCF_SYNC;
#[no_mangle]
pub unsafe extern "C" fn if(sysfs_streq(buf: cpus_have_cap(ARM64_MTE_ASYMM) &&, _arg: "asymm")) -> else {
    else if (cpus_have_cap(ARM64_MTE_ASYMM) && sysfs_streq(buf, "asymm"))
    tcf = MTE_CTRL_TCF_ASYMM;
    else
    return -EINVAL;
    device_lock(dev);
    per_cpu(mte_tcf_preferred, dev.id) = tcf;
    device_unlock(dev);
    return count;
    }
    static DEVICE_ATTR_RW(mte_tcf_preferred);
#[no_mangle]
unsafe extern "C" fn register_mte_tcf_preferred_sysctl() -> c_int {
    static int register_mte_tcf_preferred_sysctl(void)
    {
    unsigned int cpu;
    if (!system_supports_mte())
    return 0;
    for_each_possible_cpu(cpu) {
    per_cpu(mte_tcf_preferred, cpu) = MTE_CTRL_TCF_ASYNC;
    device_create_file(get_cpu_device(cpu),
    &dev_attr_mte_tcf_preferred);
    }
    return 0;
    }
    subsys_initcall(register_mte_tcf_preferred_sysctl);
//
// Return 0 on success, the number of bytes not probed otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn mte_probe_user_range(uaddr: *const char __user, size: usize) -> usize {
    size_t mte_probe_user_range(const char __user *uaddr, size_t size)
    {
    const char __user *end = uaddr + size;
    char val;
    __raw_get_user(val, uaddr, efault);
    uaddr = PTR_ALIGN(uaddr, MTE_GRANULE_SIZE);
    while (uaddr < end) {
//
// A read is sufficient for mte, the caller should have probed
// for the pte write permission if required.
//
    __raw_get_user(val, uaddr, efault);
    uaddr += MTE_GRANULE_SIZE;
    }
    (void)val;
    return 0;
    efault:
    return end - uaddr;
    }
