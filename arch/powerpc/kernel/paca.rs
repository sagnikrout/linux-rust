//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/paca.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// c 2001 PPC 64 Team, IBM Corp
//

pub const boot_cpuid: c_int = 0;

    static void *__init alloc_paca_data(unsigned long size, unsigned long align,
    unsigned long limit, int cpu)
    {
    void *ptr;
    int nid;
//
// boot_cpuid paca is allocated very early before cpu_to_node is up.
// Set bottom-up mode, because the boot CPU should be on node-0,
// which will put its paca in the right place.
//
    if (cpu == boot_cpuid) {
    nid = NUMA_NO_NODE;
    memblock_set_bottom_up(true);
    } else {
    nid = early_cpu_to_node(cpu);
    }
    ptr = memblock_alloc_try_nid(size, align, MEMBLOCK_LOW_LIMIT,
    limit, nid);
    if (!ptr)
    panic("cannot allocate paca data");
    if (cpu == boot_cpuid)
    memblock_set_bottom_up(false);
    return ptr;
    }

pub const LPPACA_SIZE: c_uint = 0x400;
    static void *__init alloc_shared_lppaca(unsigned long size, unsigned long limit,
    int cpu)
    {
    let mut shared_lppaca_total_size: usize = PAGE_ALIGN(nr_cpu_ids * LPPACA_SIZE);
    static unsigned long shared_lppaca_size;
    static void *shared_lppaca;
    void *ptr;
    if (!shared_lppaca) {
    memblock_set_bottom_up(true);
//
// See Documentation/arch/powerpc/ultravisor.rst for more details.
//
// UV/HV data sharing is in PAGE_SIZE granularity. In order to
// minimize the number of pages shared, align the allocation to
// PAGE_SIZE.
//
    shared_lppaca =
    memblock_alloc_try_nid(shared_lppaca_total_size,
    PAGE_SIZE, MEMBLOCK_LOW_LIMIT,
    limit, NUMA_NO_NODE);
    if (!shared_lppaca)
    panic("cannot allocate shared data");
    memblock_set_bottom_up(false);
    uv_share_page(PHYS_PFN(__pa(shared_lppaca)),
    shared_lppaca_total_size >> PAGE_SHIFT);
    }
    ptr = shared_lppaca + shared_lppaca_size;
    shared_lppaca_size += size;
//
// This is very early in boot, so no harm done if the kernel crashes at
// this point.
//
    BUG_ON(shared_lppaca_size > shared_lppaca_total_size);
    return ptr;
    }
//
// See asm/lppaca.h for more detail.
//
// lppaca structures must must be 1kB in size, L1 cache line aligned,
// and not cross 4kB boundary. A 1kB size and 1kB alignment will satisfy
// these requirements.
//
#[no_mangle]
pub unsafe extern "C" fn init_lppaca(lppaca: *mut lppaca) {
    static inline void init_lppaca(struct lppaca *lppaca)
    {
    BUILD_BUG_ON(sizeof(struct lppaca) != 640);
// lppaca = (struct lppaca) {
    .desc = cpu_to_be32(0xd397d781),	/* "LpPa" */
    .size = cpu_to_be16(LPPACA_SIZE),
    .fpregs_in_use = 1,
    .slb_count = cpu_to_be16(64),
    .vmxregs_in_use = 0,
    .page_ins = 0, };
    };
#[no_mangle]
unsafe extern "C" fn new_lppaca(cpu: c_int, limit: c_ulong) -> *mut lppaca  __init {
    static struct lppaca * __init new_lppaca(int cpu, unsigned long limit)
    {
    struct lppaca *lp;
    BUILD_BUG_ON(sizeof(struct lppaca) > LPPACA_SIZE);
    if (early_cpu_has_feature(CPU_FTR_HVMODE))
    return core::ptr::null_mut();
    if (is_secure_guest())
    lp = alloc_shared_lppaca(LPPACA_SIZE, limit, cpu);
    else
    lp = alloc_paca_data(LPPACA_SIZE, 0x400, limit, cpu);
    init_lppaca(lp);
    return lp;
    }

//
// 3 persistent SLBs are allocated here.  The buffer will be zero
// initially, hence will all be invaild until we actually write them.
//
// If you make the number of persistent SLB entries dynamic, please also
// update PR KVM to flush and restore them accordingly.
//
#[no_mangle]
unsafe extern "C" fn new_slb_shadow(cpu: c_int, limit: c_ulong) -> *mut slb_shadow  __init {
    static struct slb_shadow * __init new_slb_shadow(int cpu, unsigned long limit)
    {
    struct slb_shadow *s;
    if (cpu != boot_cpuid) {
//
// Boot CPU comes here before early_radix_enabled
// is parsed (e.g., for disable_radix). So allocate
// always and this will be fixed up in free_unused_pacas.
//
    if (early_radix_enabled())
    return core::ptr::null_mut();
    }
    s = alloc_paca_data(sizeof(*s), L1_CACHE_BYTES, limit, cpu);
    s.persistent = cpu_to_be32(SLB_NUM_BOLTED);
    s.buffer_length = cpu_to_be32(sizeof(*s));
    return s;
    }

// The Paca is an array with one entry per processor.  Each contains an
// lppaca, which contains the information shared between the
// hypervisor and Linux.
// On systems with hardware multi-threading, there are two threads
// per processor.  The Paca array must contain an entry for each thread.
// The VPD Areas will give a max logical processors = 2 * max physical
// processors.  The processor VPD array needs one entry per physical
// processor (not thread).
//
    struct paca_struct **paca_ptrs __read_mostly;
    EXPORT_SYMBOL(paca_ptrs);
#[no_mangle]
pub unsafe extern "C" fn initialise_paca(new_paca: *mut paca_struct, cpu: c_int) -> void __init {
    void __init initialise_paca(struct paca_struct *new_paca, int cpu)
    {

    new_paca.lppaca_ptr = core::ptr::null_mut();

    new_paca.kernel_pgd = swapper_pg_dir;

    new_paca.lock_token = 0x8000;
    new_paca.paca_index = cpu;

    new_paca.kernel_toc = kernel_toc_addr();

    new_paca.kernelbase = (unsigned long) _stext;
// Only set MSR:IR/DR when MMU is initialized
    new_paca.kernel_msr = MSR_KERNEL & ~(MSR_IR | MSR_DR);
    new_paca.hw_cpu_id = 0xffff;
    new_paca.kexec_state = KEXEC_STATE_NONE;
    new_paca.__current = &init_task;
    new_paca.data_offset = 0xfeeeeeeeeeeeeeeeULL;

    new_paca.slb_shadow_ptr = core::ptr::null_mut();

// For now -- if we have threads this will be adjusted later
    new_paca.tcd_ptr = &new_paca.tcd;

    }
// Put the paca pointer into r13 and SPRG_PACA
#[no_mangle]
pub unsafe extern "C" fn setup_paca(new_paca: *mut paca_struct) {
    void setup_paca(struct paca_struct *new_paca)
    {
// Setup r13
    local_paca = new_paca;

// On Book3E, initialize the TLB miss exception frames
    mtspr(SPRN_SPRG_TLB_EXFRAME, local_paca.extlb);

//
// In HV mode, we setup both HPACA and PACA to avoid problems
// if we do a GET_PACA() before the feature fixups have been
// applied.
//
// Normally you should test against CPU_FTR_HVMODE, but CPU features
// are not yet set up when we first reach here.
//
    if (mfmsr() & MSR_HV)
    mtspr(SPRN_SPRG_HPACA, local_paca);

    mtspr(SPRN_SPRG_PACA, local_paca);
    }
    static int __initdata paca_nr_cpu_ids;
    static int __initdata paca_ptrs_size;
    static int __initdata paca_struct_size;
#[no_mangle]
pub unsafe extern "C" fn allocate_paca_ptrs() -> void __init {
    void __init allocate_paca_ptrs(void)
    {
    paca_nr_cpu_ids = nr_cpu_ids;
    paca_ptrs_size = sizeof(struct paca_struct *) * nr_cpu_ids;
    paca_ptrs = memblock_alloc_raw(paca_ptrs_size, SMP_CACHE_BYTES);
    if (!paca_ptrs)
    panic("Failed to allocate %d bytes for paca pointers\n",
    paca_ptrs_size);
    memset(paca_ptrs, 0x88, paca_ptrs_size);
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_paca(cpu: c_int) -> void __init {
    void __init allocate_paca(int cpu)
    {
    u64 limit;
    struct paca_struct *paca;
    BUG_ON(cpu >= paca_nr_cpu_ids);

//
// We access pacas in real mode, and cannot take SLB faults
// on them when in virtual mode, so allocate them accordingly.
//
    limit = min(ppc64_bolted_size(), ppc64_rma_size);

    limit = ppc64_rma_size;

    paca = alloc_paca_data(sizeof(struct paca_struct), L1_CACHE_BYTES,
    limit, cpu);
    paca_ptrs[cpu] = paca;
    initialise_paca(paca, cpu);

    paca.lppaca_ptr = new_lppaca(cpu, limit);

    paca.slb_shadow_ptr = new_slb_shadow(cpu, limit);

    paca_struct_size += sizeof(struct paca_struct);
    }
#[no_mangle]
pub unsafe extern "C" fn free_unused_pacas() -> void __init {
    void __init free_unused_pacas(void)
    {
    int new_ptrs_size;
    new_ptrs_size = sizeof(struct paca_struct *) * nr_cpu_ids;
    if (new_ptrs_size < paca_ptrs_size)
    memblock_phys_free(__pa(paca_ptrs) + new_ptrs_size,
    paca_ptrs_size - new_ptrs_size);
    paca_nr_cpu_ids = nr_cpu_ids;
    paca_ptrs_size = new_ptrs_size;

    if (early_radix_enabled()) {
// Ugly fixup, see new_slb_shadow()
    memblock_phys_free(__pa(paca_ptrs[boot_cpuid].slb_shadow_ptr),
    sizeof(struct slb_shadow));
    paca_ptrs[boot_cpuid].slb_shadow_ptr = core::ptr::null_mut();
    }

    printk(KERN_DEBUG "Allocated %u bytes for %u pacas\n",
    paca_ptrs_size + paca_struct_size, nr_cpu_ids);
    }

#[no_mangle]
pub unsafe extern "C" fn copy_mm_to_paca(mm: *mut mm_struct) {
    void copy_mm_to_paca(struct mm_struct *mm)
    {
    mm_context_t *context = &mm.context;
    VM_BUG_ON(!mm_ctx_slb_addr_limit(context));
    memcpy(&get_paca().mm_ctx_low_slices_psize, mm_ctx_low_slices(context),
    LOW_SLICE_ARRAY_SZ);
    memcpy(&get_paca().mm_ctx_high_slices_psize, mm_ctx_high_slices(context),
    TASK_SLICE_ARRAY_SZ(context));
    }
