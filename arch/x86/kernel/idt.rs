//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/idt.c
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
// Interrupt descriptor table related code
//

pub const DPL0: c_uint = 0x0;
pub const DPL3: c_uint = 0x3;
pub const DEFAULT_STACK: c_int = 0;

    {						\
    .vector		= _vector,		\
    .bits.ist	= _ist,			\
    .bits.type	= _type,		\
    .bits.dpl	= _dpl,			\
    .bits.p		= 1,			\
    .addr		= _addr,		\
    .segment	= _segment,		\
    }
// Interrupt gate

    G(_vector, _addr, DEFAULT_STACK, GATE_INTERRUPT, DPL0, __KERNEL_CS)
// System interrupt gate

    G(_vector, _addr, DEFAULT_STACK, GATE_INTERRUPT, DPL3, __KERNEL_CS)

//
// Interrupt gate with interrupt stack. The _ist index is the index in
// the tss.ist[] array, but for the descriptor it needs to start at 1.
//

    G(_vector, _addr, _ist + 1, GATE_INTERRUPT, DPL0, __KERNEL_CS)

// Task gate

    G(_vector, core::ptr::null_mut(), DEFAULT_STACK, GATE_TASK, DPL0, _gdt << 3)

    static bool idt_setup_done __initdata;
//
// Early traps running on the DEFAULT_STACK because the other interrupt
// stacks work only after cpu_init().
//
    static const __initconst struct idt_data early_idts[] = {
    INTG(X86_TRAP_DB,		asm_exc_debug),
    SYSG(X86_TRAP_BP,		asm_exc_int3),

//
// Not possible on 64-bit. See idt_setup_early_pf() for details.
//
    INTG(X86_TRAP_PF,		asm_exc_page_fault),

    INTG(X86_TRAP_VE,		asm_exc_virtualization_exception),

    };
//
// The default IDT entries which are set up in trap_init() before
// cpu_init() is invoked. Interrupt stacks cannot be used at that point and
// the traps which use them are reinitialized with IST after cpu_init() has
// set up TSS.
//
    static const __initconst struct idt_data def_idts[] = {
    INTG(X86_TRAP_DE,		asm_exc_divide_error),
    ISTG(X86_TRAP_NMI,		asm_exc_nmi, IST_INDEX_NMI),
    INTG(X86_TRAP_BR,		asm_exc_bounds),
    INTG(X86_TRAP_UD,		asm_exc_invalid_op),
    INTG(X86_TRAP_NM,		asm_exc_device_not_available),
    INTG(X86_TRAP_OLD_MF,		asm_exc_coproc_segment_overrun),
    INTG(X86_TRAP_TS,		asm_exc_invalid_tss),
    INTG(X86_TRAP_NP,		asm_exc_segment_not_present),
    INTG(X86_TRAP_SS,		asm_exc_stack_segment),
    INTG(X86_TRAP_GP,		asm_exc_general_protection),
    INTG(X86_TRAP_SPURIOUS,		asm_exc_spurious_interrupt_bug),
    INTG(X86_TRAP_MF,		asm_exc_coprocessor_error),
    INTG(X86_TRAP_AC,		asm_exc_alignment_check),
    INTG(X86_TRAP_XF,		asm_exc_simd_coprocessor_error),

    TSKG(X86_TRAP_DF,		GDT_ENTRY_DOUBLEFAULT_TSS),

    ISTG(X86_TRAP_DF,		asm_exc_double_fault, IST_INDEX_DF),

    ISTG(X86_TRAP_DB,		asm_exc_debug, IST_INDEX_DB),

    ISTG(X86_TRAP_MC,		asm_exc_machine_check, IST_INDEX_MCE),

    INTG(X86_TRAP_CP,		asm_exc_control_protection),

    ISTG(X86_TRAP_VC,		asm_exc_vmm_communication, IST_INDEX_VC),

    SYSG(X86_TRAP_OF,		asm_exc_overflow),
    };
    static const struct idt_data ia32_idt[] __initconst = {

    SYSG(IA32_SYSCALL_VECTOR,	asm_int80_emulation),

    SYSG(IA32_SYSCALL_VECTOR,	entry_INT80_32),

    };
//
// The APIC and SMP idt entries
//
    static const __initconst struct idt_data apic_idts[] = {

    INTG(RESCHEDULE_VECTOR,			asm_sysvec_reschedule_ipi),
    INTG(CALL_FUNCTION_VECTOR,		asm_sysvec_call_function),
    INTG(CALL_FUNCTION_SINGLE_VECTOR,	asm_sysvec_call_function_single),
    INTG(REBOOT_VECTOR,			asm_sysvec_reboot),

    INTG(THERMAL_APIC_VECTOR,		asm_sysvec_thermal),

    INTG(THRESHOLD_APIC_VECTOR,		asm_sysvec_threshold),

    INTG(DEFERRED_ERROR_VECTOR,		asm_sysvec_deferred_error),

    INTG(LOCAL_TIMER_VECTOR,		asm_sysvec_apic_timer_interrupt),
    INTG(X86_PLATFORM_IPI_VECTOR,		asm_sysvec_x86_platform_ipi),

    INTG(POSTED_INTR_VECTOR,		asm_sysvec_kvm_posted_intr_ipi),
    INTG(POSTED_INTR_WAKEUP_VECTOR,		asm_sysvec_kvm_posted_intr_wakeup_ipi),
    INTG(POSTED_INTR_NESTED_VECTOR,		asm_sysvec_kvm_posted_intr_nested_ipi),

    INTG(PERF_GUEST_MEDIATED_PMI_VECTOR,	asm_sysvec_perf_guest_mediated_pmi_handler),

    INTG(IRQ_WORK_VECTOR,			asm_sysvec_irq_work),

    INTG(SPURIOUS_APIC_VECTOR,		asm_sysvec_spurious_apic_interrupt),
    INTG(ERROR_APIC_VECTOR,			asm_sysvec_error_interrupt),

    INTG(POSTED_MSI_NOTIFICATION_VECTOR,	asm_sysvec_posted_msi_notification),

    };
// Must be page-aligned because the real IDT is used in the cpu entry area
    static gate_desc idt_table[IDT_ENTRIES] __page_aligned_bss;
    static struct desc_ptr idt_descr __ro_after_init = {
    .size		= IDT_TABLE_SIZE - 1,
    .address	= (unsigned long) idt_table,
    };
#[no_mangle]
pub unsafe extern "C" fn load_current_idt() {
    void load_current_idt(void)
    {
    lockdep_assert_irqs_disabled();
    load_idt(&idt_descr);
    }

#[no_mangle]
pub unsafe extern "C" fn idt_is_f00f_address(address: c_ulong) -> bool {
    bool idt_is_f00f_address(unsigned long address)
    {
    return ((address - idt_descr.address) >> 3) == 6;
    }

    static __init void
    idt_setup_from_table(gate_desc *idt, const struct idt_data *t, int size, bool sys)
    {
    gate_desc desc;
    for (; size > 0; t++, size--) {
    idt_init_desc(&desc, t);
    write_idt_entry(idt, t.vector, &desc);
    if (sys)
    set_bit(t.vector, system_vectors);
    }
    }
#[no_mangle]
unsafe extern "C" fn set_intr_gate(n: c_uint, addr: *const c_void) -> __init void {
    static __init void set_intr_gate(unsigned int n, const void *addr)
    {
    struct idt_data data;
    init_idt_data(&data, n, addr);
    idt_setup_from_table(idt_table, &data, 1, false);
    }
//
// idt_setup_early_traps - Initialize the idt table with early traps
//
// On X8664 these traps do not use interrupt stacks as they can't work
// before cpu_init() is invoked and sets up TSS. The IST variants are
// installed after that.
//
#[no_mangle]
pub unsafe extern "C" fn idt_setup_early_traps() -> void __init {
    void __init idt_setup_early_traps(void)
    {
    idt_setup_from_table(idt_table, early_idts, ARRAY_SIZE(early_idts),
    true);
    load_idt(&idt_descr);
    }
//
// idt_setup_traps - Initialize the idt table with default traps
//
#[no_mangle]
pub unsafe extern "C" fn idt_setup_traps() -> void __init {
    void __init idt_setup_traps(void)
    {
    idt_setup_from_table(idt_table, def_idts, ARRAY_SIZE(def_idts), true);
    if (ia32_enabled())
    idt_setup_from_table(idt_table, ia32_idt, ARRAY_SIZE(ia32_idt), true);
    }

//
// Early traps running on the DEFAULT_STACK because the other interrupt
// stacks work only after cpu_init().
//
    static const __initconst struct idt_data early_pf_idts[] = {
    INTG(X86_TRAP_PF,		asm_exc_page_fault),
    };
//
// idt_setup_early_pf - Initialize the idt table with early pagefault handler
//
// On X8664 this does not use interrupt stacks as they can't work before
// cpu_init() is invoked and sets up TSS. The IST variant is installed
// after that.
//
// Note, that X86_64 cannot install the real #PF handler in
// idt_setup_early_traps() because the memory initialization needs the #PF
// handler from the early_idt_handler_array to initialize the early page
// tables.
//
#[no_mangle]
pub unsafe extern "C" fn idt_setup_early_pf() -> void __init {
    void __init idt_setup_early_pf(void)
    {
    idt_setup_from_table(idt_table, early_pf_idts,
    ARRAY_SIZE(early_pf_idts), true);
    }

#[no_mangle]
pub unsafe extern "C" fn idt_entry_from_kvm(vector: c_uint) {
    void idt_entry_from_kvm(unsigned int vector)
    {
    idt_do_interrupt_irqoff(gate_offset(idt_table + vector));
    }

#[no_mangle]
unsafe extern "C" fn idt_map_in_cea() -> void __init {
    static void __init idt_map_in_cea(void)
    {
//
// Set the IDT descriptor to a fixed read-only location in the cpu
// entry area, so that the "sidt" instruction will not leak the
// location of the kernel, and to defend the IDT against arbitrary
// memory write vulnerabilities.
//
    cea_set_pte(CPU_ENTRY_AREA_RO_IDT_VADDR, __pa_symbol(idt_table),
    PAGE_KERNEL_RO);
    idt_descr.address = CPU_ENTRY_AREA_RO_IDT;
    }
//
// idt_setup_apic_and_irq_gates - Setup APIC/SMP and normal interrupt gates
//
#[no_mangle]
pub unsafe extern "C" fn idt_setup_apic_and_irq_gates() -> void __init {
    void __init idt_setup_apic_and_irq_gates(void)
    {
    let mut i: c_int = FIRST_EXTERNAL_VECTOR;
    void *entry;
    idt_setup_from_table(idt_table, apic_idts, ARRAY_SIZE(apic_idts), true);
    for_each_clear_bit_from(i, system_vectors, FIRST_SYSTEM_VECTOR) {
    entry = irq_entries_start + IDT_ALIGN * (i - FIRST_EXTERNAL_VECTOR);
    set_intr_gate(i, entry);
    }

    for_each_clear_bit_from(i, system_vectors, NR_VECTORS) {
//
// Don't set the non assigned system vectors in the
// system_vectors bitmap. Otherwise they show up in
// /proc/interrupts.
//
    entry = spurious_entries_start + IDT_ALIGN * (i - FIRST_SYSTEM_VECTOR);
    set_intr_gate(i, entry);
    }

// Map IDT into CPU entry area and reload it.
    idt_map_in_cea();
    load_idt(&idt_descr);
// Make the IDT table read only
    set_memory_ro((unsigned long)&idt_table, 1);
    idt_setup_done = true;
    }
//
// idt_setup_early_handler - Initializes the idt table with early handlers
//
#[no_mangle]
pub unsafe extern "C" fn idt_setup_early_handler() -> void __init {
    void __init idt_setup_early_handler(void)
    {
    int i;
    for (i = 0; i < NUM_EXCEPTION_VECTORS; i++)
    set_intr_gate(i, early_idt_handler_array[i]);

    for ( ; i < NR_VECTORS; i++)
    set_intr_gate(i, early_ignore_irq);

    load_idt(&idt_descr);
    }
//
// idt_invalidate - Invalidate interrupt descriptor table
//
#[no_mangle]
pub unsafe extern "C" fn idt_invalidate() {
    void idt_invalidate(void)
    {
    let mut idt: static struct desc_ptr = { .address = 0, .size = 0 };
    load_idt(&idt);
    }
#[no_mangle]
pub unsafe extern "C" fn idt_install_sysvec(n: c_uint, function: *const c_void) -> void __init {
    void __init idt_install_sysvec(unsigned int n, const void *function)
    {
    if (WARN_ON(n < FIRST_SYSTEM_VECTOR))
    return;
    if (WARN_ON(idt_setup_done))
    return;
    if (!WARN_ON(test_and_set_bit(n, system_vectors)))
    set_intr_gate(n, function);
    }
