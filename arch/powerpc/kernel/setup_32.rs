//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/setup_32.c
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
// Common prep/pmac/chrp boot and setup code.
//

// Macro flag: #define DBG(fmt...)
    extern void bootx_init(unsigned long r4, unsigned long phys);
    int boot_cpuid_phys;
    EXPORT_SYMBOL_GPL(boot_cpuid_phys);
    int smp_hw_index[NR_CPUS];
    EXPORT_SYMBOL(smp_hw_index);
    unsigned int DMA_MODE_READ;
    unsigned int DMA_MODE_WRITE;
    EXPORT_SYMBOL(DMA_MODE_READ);
    EXPORT_SYMBOL(DMA_MODE_WRITE);
//
// This is run before start_kernel(), the kernel has been relocated
// and we are running with enough of the MMU enabled to have our
// proper kernel virtual addresses
//
// We do the initial parsing of the flat device-tree and prepares
// for the MMU to be fully initialized.
//
#[no_mangle]
pub unsafe extern "C" fn machine_init(dt_ptr: u64) -> notrace void __init {
    notrace void __init machine_init(u64 dt_ptr)
    {
    u32 *addr = (u32 *)patch_site_addr(&patch__memset_nocache);
    ppc_inst_t insn;
// Configure static keys first, now that we're relocated.
    setup_feature_keys();
    early_ioremap_init();
// Enable early debugging if any specified (see udbg.h)
    udbg_early_init();
    patch_instruction_site(&patch__memcpy_nocache, ppc_inst(PPC_RAW_NOP()));
    create_cond_branch(&insn, addr, branch_target(addr), 0x820000);
    patch_instruction(addr, insn);	/* replace b by bne cr0 */
// Do some early initialization based on the flat device tree
    early_init_devtree(__va(dt_ptr));
    early_init_mmu();
    setup_kdump_trampoline();
    }
// Checks "l2cr=xxxx" command-line option
#[no_mangle]
unsafe extern "C" fn ppc_setup_l2cr(str: *mut c_char) -> int __init {
    static int __init ppc_setup_l2cr(char *str)
    {
    if (cpu_has_feature(CPU_FTR_L2CR)) {
    let mut val: c_ulong = simple_strtoul(str, core::ptr::null_mut(), 0);
    printk(KERN_INFO "l2cr set to %lx\n", val);
    _set_L2CR(0);		/* force invalidate by disable cache */
    _set_L2CR(val);		/* and enable it */
    }
    return 1;
    }
    __setup("l2cr=", ppc_setup_l2cr);
// Checks "l3cr=xxxx" command-line option
#[no_mangle]
unsafe extern "C" fn ppc_setup_l3cr(str: *mut c_char) -> int __init {
    static int __init ppc_setup_l3cr(char *str)
    {
    if (cpu_has_feature(CPU_FTR_L3CR)) {
    let mut val: c_ulong = simple_strtoul(str, core::ptr::null_mut(), 0);
    printk(KERN_INFO "l3cr set to %lx\n", val);
    _set_L3CR(val);		/* and enable it */
    }
    return 1;
    }
    __setup("l3cr=", ppc_setup_l3cr);
#[no_mangle]
unsafe extern "C" fn ppc_init() -> int __init {
    static int __init ppc_init(void)
    {
// clear the progress line
    if (ppc_md.progress)
    ppc_md.progress("             ", 0xffff);
// call platform init
    if (ppc_md.init != core::ptr::null_mut()) {
    ppc_md.init();
    }
    return 0;
    }
    arch_initcall(ppc_init);
#[no_mangle]
unsafe extern "C" fn alloc_stack() -> *mut void __init {
    static void *__init alloc_stack(void)
    {
    return memblock_alloc_or_panic(THREAD_SIZE, THREAD_ALIGN);
    }
#[no_mangle]
pub unsafe extern "C" fn irqstack_early_init() -> void __init {
    void __init irqstack_early_init(void)
    {
    unsigned int i;
    if (IS_ENABLED(CONFIG_VMAP_STACK))
    return;
// interrupt stacks must be in lowmem, we get that for free on ppc32
// as the memblock is limited to lowmem by default
    for_each_possible_cpu(i) {
    softirq_ctx[i] = alloc_stack();
    hardirq_ctx[i] = alloc_stack();
    }
    }

    void *emergency_ctx[NR_CPUS] __ro_after_init = {[0] = &init_stack};
#[no_mangle]
pub unsafe extern "C" fn emergency_stack_init() -> void __init {
    void __init emergency_stack_init(void)
    {
    unsigned int i;
    for_each_possible_cpu(i)
    emergency_ctx[i] = alloc_stack();
    }

#[no_mangle]
pub unsafe extern "C" fn exc_lvl_early_init() -> void __init {
    void __init exc_lvl_early_init(void)
    {
    unsigned int i, hw_cpu;
// interrupt stacks must be in lowmem, we get that for free on ppc32
// as the memblock is limited to lowmem by MEMBLOCK_REAL_LIMIT
    for_each_possible_cpu(i) {

    hw_cpu = get_hard_smp_processor_id(i);

    hw_cpu = 0;

    critirq_ctx[hw_cpu] = alloc_stack();

    dbgirq_ctx[hw_cpu] = alloc_stack();
    mcheckirq_ctx[hw_cpu] = alloc_stack();

    }
    }

#[no_mangle]
pub unsafe extern "C" fn setup_power_save() -> void __init {
    void __init setup_power_save(void)
    {

    if (cpu_has_feature(CPU_FTR_CAN_DOZE) ||
    cpu_has_feature(CPU_FTR_CAN_NAP))
    ppc_md.power_save = ppc6xx_idle;

    if (cpu_has_feature(CPU_FTR_CAN_DOZE) ||
    cpu_has_feature(CPU_FTR_CAN_NAP))
    ppc_md.power_save = e500_idle;

    }
#[no_mangle]
pub unsafe extern "C" fn initialize_cache_info() -> __init void {
    __init void initialize_cache_info(void)
    {
//
// Set cache line size based on type of cpu as a default.
// Systems with OF can look in the properties on the cpu node(s)
// for a possibly more accurate value.
//
    dcache_bsize = cur_cpu_spec.dcache_bsize;
    icache_bsize = cur_cpu_spec.icache_bsize;
    }
