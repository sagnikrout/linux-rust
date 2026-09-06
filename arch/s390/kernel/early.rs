//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/early.c
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
// Copyright IBM Corp. 2007, 2009
// Author(s): Hongjie Yang <hongjie@us.ibm.com>,
//

    static int __init ignore_decompressor_param_##func(char *s)	\
    {								\
    return 0;						\
    }								\
    early_param(#param, ignore_decompressor_param_##func)

    decompressor_handled_param(mem);
    decompressor_handled_param(vmalloc);
    decompressor_handled_param(dfltcc);
    decompressor_handled_param(facilities);
    decompressor_handled_param(nokaslr);
    decompressor_handled_param(cmma);
    decompressor_handled_param(relocate_lowcore);
    decompressor_handled_param(bootdebug);
    __decompressor_handled_param(debug_alternative, debug-alternative);

    decompressor_handled_param(prot_virt);

#[no_mangle]
unsafe extern "C" fn kasan_early_init() -> void __init {
    static void __init kasan_early_init(void)
    {

    init_task.kasan_depth = 0;
    kasan_init_generic();

    }
//
// Initialize storage key for kernel pages
//
#[no_mangle]
unsafe extern "C" fn init_kernel_storage_key() -> noinline __init void {
    static noinline __init void init_kernel_storage_key(void)
    {

    unsigned long end_pfn, init_pfn;
    end_pfn = PFN_UP(__pa(_end));
    for (init_pfn = 0 ; init_pfn < end_pfn; init_pfn++)
    page_set_storage_key(init_pfn << PAGE_SHIFT,
    PAGE_DEFAULT_KEY, 0);

    }
    static __initdata char sysinfo_page[PAGE_SIZE] __aligned(PAGE_SIZE);
// Remove leading, trailing and double whitespace.
#[no_mangle]
pub unsafe extern "C" fn strim_all(str: *mut c_char) {
    static inline void strim_all(char *str)
    {
    char *s;
    s = strim(str);
    if (s != str)
    memmove(str, s, strlen(s));
    while (*str) {
    if (!isspace(*str++))
    continue;
    if (isspace(*str)) {
    s = skip_spaces(str);
    memmove(str, s, strlen(s) + 1);
    }
    }
    }
    char arch_hw_string[128];
#[no_mangle]
unsafe extern "C" fn setup_arch_string() -> noinline __init void {
    static noinline __init void setup_arch_string(void)
    {
    struct sysinfo_1_1_1 *mach = (struct sysinfo_1_1_1 *)&sysinfo_page;
    struct sysinfo_3_2_2 *vm = (struct sysinfo_3_2_2 *)&sysinfo_page;
    char mstr[80], hvstr[17];
    if (stsi(mach, 1, 1, 1))
    return;
    EBCASC(mach.manufacturer, sizeof(mach.manufacturer));
    EBCASC(mach.type, sizeof(mach.type));
    EBCASC(mach.model, sizeof(mach.model));
    EBCASC(mach.model_capacity, sizeof(mach.model_capacity));
    scnprintf(mstr, sizeof(mstr), "%-16.16s %-4.4s %-16.16s %-16.16s",
    mach.manufacturer, mach.type,
    mach.model, mach.model_capacity);
    strim_all(mstr);
    if (stsi(vm, 3, 2, 2) == 0 && vm.count) {
    EBCASC(vm.vm[0].cpi, sizeof(vm.vm[0].cpi));
    scnprintf(hvstr, sizeof(hvstr), "%-16.16s", vm.vm[0].cpi);
    strim_all(hvstr);
    } else {
    scnprintf(hvstr, sizeof(hvstr), "%s",
    machine_is_lpar() ? "LPAR" :
    machine_is_vm() ? "z/VM" :
    machine_is_kvm() ? "KVM" : "unknown");
    }
    scnprintf(arch_hw_string, sizeof(arch_hw_string), "HW: %s (%s)", mstr, hvstr);
    dump_stack_set_arch_desc("%s (%s)", mstr, hvstr);
    }
#[no_mangle]
unsafe extern "C" fn setup_topology() -> __init void {
    static __init void setup_topology(void)
    {
    int max_mnest;
    if (!cpu_has_topology())
    return;
    for (max_mnest = 6; max_mnest > 1; max_mnest--) {
    if (stsi(&sysinfo_page, 15, 1, max_mnest) == 0)
    break;
    }
    topology_max_mnest = max_mnest;
    }
#[no_mangle]
pub unsafe extern "C" fn __do_early_pgm_check(regs: *mut pt_regs) -> void __init {
    void __init __do_early_pgm_check(struct pt_regs *regs)
    {
    struct lowcore *lc = get_lowcore();
    unsigned long ip;
    regs.int_code = lc.pgm_int_code;
    regs.int_parm_long = lc.trans_exc_code;
    regs.last_break = lc.pgm_last_break;
    ip = __rewind_psw(regs.psw, regs.int_code >> 16);
// Monitor Event? Might be a warning
    if ((regs.int_code & PGM_INT_CODE_MASK) == 0x40) {
    if (report_bug(ip, regs) == BUG_TRAP_TYPE_WARN)
    return;
    }
    if (fixup_exception(regs))
    return;
//
// Unhandled exception - system cannot continue but try to get some
// helpful messages to the console. Use early_printk() to print
// some basic information in case it is too early for printk().
//
    register_early_console();
    early_printk("PANIC: early exception %04x PSW: %016lx %016lx\n",
    regs.int_code & 0xffff, regs.psw.mask, regs.psw.addr);
    show_regs(regs);
    disabled_wait();
    }
#[no_mangle]
unsafe extern "C" fn setup_lowcore_early() -> noinline __init void {
    static noinline __init void setup_lowcore_early(void)
    {
    struct lowcore *lc = get_lowcore();
    psw_t psw;
    psw.addr = (unsigned long)early_pgm_check_handler;
    psw.mask = PSW_KERNEL_BITS;
    lc.program_new_psw = psw;
    lc.preempt_count = INIT_PREEMPT_COUNT;
    lc.return_lpswe = gen_lpswe(__LC_RETURN_PSW);
    lc.return_mcck_lpswe = gen_lpswe(__LC_RETURN_MCCK_PSW);
    }
#[no_mangle]
pub unsafe extern "C" fn save_vector_registers() {
    static inline void save_vector_registers(void)
    {

    if (cpu_has_vx())
    save_vx_regs(boot_cpu_vector_save_area);

    }
#[no_mangle]
pub unsafe extern "C" fn setup_low_address_protection() {
    static inline void setup_low_address_protection(void)
    {
    system_ctl_set_bit(0, CR0_LOW_ADDRESS_PROTECTION_BIT);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_access_registers() {
    static inline void setup_access_registers(void)
    {
    unsigned int acrs[NUM_ACRS] = { 0 };
    restore_access_regs(acrs);
    }
    char __bootdata(early_command_line)[COMMAND_LINE_SIZE];
#[no_mangle]
unsafe extern "C" fn setup_boot_command_line() -> void __init {
    static void __init setup_boot_command_line(void)
    {
// copy arch command line
    strscpy(boot_command_line, early_command_line, COMMAND_LINE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn sort_amode31_extable() -> void __init {
    static void __init sort_amode31_extable(void)
    {
    sort_extable(__start_amode31_ex_table, __stop_amode31_ex_table);
    }
#[no_mangle]
pub unsafe extern "C" fn startup_init() -> void __init {
    void __init startup_init(void)
    {
    kasan_early_init();
    time_early_init();
    init_kernel_storage_key();
    lockdep_off();
    sort_amode31_extable();
    setup_lowcore_early();
    setup_arch_string();
    setup_boot_command_line();
    save_vector_registers();
    setup_topology();
    sclp_early_detect();
    setup_low_address_protection();
    setup_access_registers();
    lockdep_on();
    }
