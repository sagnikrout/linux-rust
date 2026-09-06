//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/x2apic_savic.c
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
// AMD Secure AVIC Support (SEV-SNP Guests)
//
// Copyright (C) 2024 Advanced Micro Devices, Inc.
//
// Author: Neeraj Upadhyay <Neeraj.Upadhyay@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_avic_page {
    pub regs: [u8; PAGE_SIZE],
    pub __aligned(PAGE_SIZE): },
    pub __ro_after_init: *mut *mut static struct secure_avic_page __percpu savic_page,
#[no_mangle]
unsafe extern "C" fn savic_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int savic_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    pub cc_platform_has(CC_ATTR_SNP_SECURE_AVIC): return x2apic_enabled() &&,
    }
    static inline void *get_reg_bitmap(unsigned int cpu, unsigned int offset)
    {
    pub cpu)->regs[offset]: return &per_cpu_ptr(savic_page,,
    }
    static inline void update_vector(unsigned int cpu, unsigned int offset,
    unsigned int vector, bool set)
    {
    pub offset): *mut *mut void bitmap = get_reg_bitmap(cpu,,
    if (set)
    pub bitmap): apic_set_vector(vector,,
    else
    pub bitmap): apic_clear_vector(vector,,
    }
pub const SAVIC_ALLOWED_IRR: c_uint = 0x204;
//
// When Secure AVIC is enabled, RDMSR/WRMSR of the APIC registers
// result in #VC exception (for non-accelerated register accesses)
// with VMEXIT_AVIC_NOACCEL error code. The #VC exception handler
// can read/write the x2APIC register in the guest APIC backing page.
//
// Since doing this would increase the latency of accessing x2APIC
// registers, instead of doing RDMSR/WRMSR based accesses and
// handling the APIC register reads/writes in the #VC exception handler,
// the read() and write() callbacks directly read/write the APIC register
// from/to the vCPU's APIC backing page.
//
#[no_mangle]
unsafe extern "C" fn savic_read(reg: u32) -> u32 {
    static u32 savic_read(u32 reg)
    {
    pub this_cpu_ptr(savic_page): *mut *mut void ap =,
    switch (reg) {
    case APIC_LVTT:
    case APIC_TMICT:
    case APIC_TMCCT:
    case APIC_TDCR:
    case APIC_LVTTHMR:
    case APIC_LVTPC:
    case APIC_LVT0:
    case APIC_LVT1:
    case APIC_LVTERR:
    pub savic_ghcb_msr_read(reg): return,
    case APIC_ID:
    case APIC_LVR:
    case APIC_TASKPRI:
    case APIC_ARBPRI:
    case APIC_PROCPRI:
    case APIC_LDR:
    case APIC_SPIV:
    case APIC_ESR:
    case APIC_EFEAT:
    case APIC_ECTRL:
    case APIC_SEOI:
    case APIC_IER:
    case APIC_EILVTn(0) ... APIC_EILVTn(3):
    pub reg): return apic_get_reg(ap,,
    case APIC_ICR:
    pub reg): return (u32)apic_get_reg64(ap,,
    case APIC_ISR ... APIC_ISR + 0x70:
    case APIC_TMR ... APIC_TMR + 0x70:
    if (WARN_ONCE(!IS_ALIGNED(reg, 16),
    "APIC register read offset 0x%x not aligned at 16 bytes", reg))
    pub 0: return,
    pub reg): return apic_get_reg(ap,,
// IRR and ALLOWED_IRR offset range
    case APIC_IRR ... APIC_IRR + 0x74:
//
// Valid APIC_IRR/SAVIC_ALLOWED_IRR registers are at 16 bytes strides from
// their respective base offset. APIC_IRRs are in the range
//
// (0x200, 0x210,  ..., 0x270)
//
// while the SAVIC_ALLOWED_IRR range starts 4 bytes later, in the range
//
// (0x204, 0x214, ..., 0x274).
//
// Filter out everything else.
//
    if (WARN_ONCE(!(IS_ALIGNED(reg, 16) ||
    IS_ALIGNED(reg - 4, 16)),
    "Misaligned APIC_IRR/ALLOWED_IRR APIC register read offset 0x%x", reg))
    pub 0: return,
    pub reg): return apic_get_reg(ap,,
    default:
    pub reg): pr_err("Error reading unknown Secure AVIC reg offset 0x%x\n",,
    pub 0: return,
    }
    }
pub const SAVIC_NMI_REQ: c_uint = 0x278;
//
// On WRMSR to APIC_SELF_IPI register by the guest, Secure AVIC hardware
// updates the APIC_IRR in the APIC backing page of the vCPU. In addition,
// hardware evaluates the new APIC_IRR update for interrupt injection to
// the vCPU. So, self IPIs are hardware-accelerated.
//
#[no_mangle]
pub unsafe extern "C" fn self_ipi_reg_write(vector: c_uint) {
    static inline void self_ipi_reg_write(unsigned int vector)
    {
    pub vector): native_apic_msr_write(APIC_SELF_IPI,,
    }
#[no_mangle]
unsafe extern "C" fn send_ipi_dest(cpu: c_uint, vector: c_uint, nmi: bool) {
    static void send_ipi_dest(unsigned int cpu, unsigned int vector, bool nmi)
    {
    if (nmi)
    pub 1): apic_set_reg(per_cpu_ptr(savic_page, cpu), SAVIC_NMI_REQ,,
    else
    pub true): update_vector(cpu, APIC_IRR, vector,,
    }
#[no_mangle]
unsafe extern "C" fn send_ipi_allbut(vector: c_uint, nmi: bool) {
    static void send_ipi_allbut(unsigned int vector, bool nmi)
    {
    pub src_cpu: unsigned int cpu,,
    pub raw_smp_processor_id(): src_cpu =,
    for_each_cpu(cpu, cpu_online_mask) {
    if (cpu == src_cpu)
    pub nmi): send_ipi_dest(cpu, vector,,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn self_ipi(vector: c_uint, nmi: bool) {
    static inline void self_ipi(unsigned int vector, bool nmi)
    {
    pub vector: u32 icr_low = APIC_SELF_IPI |,
    if (nmi)
    pub APIC_DM_NMI: icr_low |=,
    pub 0): native_x2apic_icr_write(icr_low,,
    }
#[no_mangle]
unsafe extern "C" fn savic_icr_write(icr_low: u32, icr_high: u32) {
    static void savic_icr_write(u32 icr_low, u32 icr_high)
    {
    pub vector: unsigned int dsh,,
    pub icr_data: u64,
    pub nmi: bool,
    pub APIC_DEST_ALLBUT: dsh = icr_low &,
    pub APIC_VECTOR_MASK: vector = icr_low &,
    pub APIC_DM_NMI): nmi = ((icr_low & APIC_DM_FIXED_MASK) ==,
    switch (dsh) {
    case APIC_DEST_SELF:
    pub nmi): self_ipi(vector,,
    case APIC_DEST_ALLINC:
    pub nmi): self_ipi(vector,,
    case APIC_DEST_ALLBUT:
    pub nmi): send_ipi_allbut(vector,,
    default:
    pub nmi): send_ipi_dest(icr_high, vector,,
    }
    pub icr_low: icr_data = ((u64)icr_high) << 32 |,
    if (dsh != APIC_DEST_SELF)
    pub icr_data): savic_ghcb_msr_write(APIC_ICR,,
    pub icr_data): apic_set_reg64(this_cpu_ptr(savic_page), APIC_ICR,,
    }
#[no_mangle]
unsafe extern "C" fn savic_write(reg: u32, data: u32) {
    static void savic_write(u32 reg, u32 data)
    {
    pub this_cpu_ptr(savic_page): *mut *mut void ap =,
    switch (reg) {
    case APIC_LVTT:
    case APIC_TMICT:
    case APIC_TDCR:
    case APIC_LVT0:
    case APIC_LVT1:
    case APIC_LVTTHMR:
    case APIC_LVTPC:
    case APIC_LVTERR:
    pub data): savic_ghcb_msr_write(reg,,
    case APIC_TASKPRI:
    case APIC_EOI:
    case APIC_SPIV:
    case SAVIC_NMI_REQ:
    case APIC_ESR:
    case APIC_ECTRL:
    case APIC_SEOI:
    case APIC_IER:
    case APIC_EILVTn(0) ... APIC_EILVTn(3):
    pub data): apic_set_reg(ap, reg,,
    case APIC_ICR:
    pub 0): savic_icr_write(data,,
    case APIC_SELF_IPI:
// ALLOWED_IRR offsets are writable
    case SAVIC_ALLOWED_IRR ... SAVIC_ALLOWED_IRR + 0x70:
    if (IS_ALIGNED(reg - 4, 16)) {
    pub data): apic_set_reg(ap, reg,,
    }
    default:
    pub reg): pr_err("Error writing unknown Secure AVIC reg offset 0x%x\n",,
    }
    }
#[no_mangle]
unsafe extern "C" fn send_ipi(dest: u32, vector: c_uint, dsh: c_uint) {
    static void send_ipi(u32 dest, unsigned int vector, unsigned int dsh)
    {
    pub dest): savic_icr_write(__prepare_ICR(dsh, vector, APIC_DEST_PHYSICAL),,
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi(cpu: c_int, vector: c_int) {
    static void savic_send_ipi(int cpu, int vector)
    {
    pub cpu): u32 dest = per_cpu(x86_cpu_to_apicid,,
    pub 0): send_ipi(dest, vector,,
    }
#[no_mangle]
unsafe extern "C" fn send_ipi_mask(mask: *const cpumask, vector: c_uint, excl_self: bool) {
    static void send_ipi_mask(const struct cpumask *mask, unsigned int vector, bool excl_self)
    {
    pub this_cpu: unsigned int cpu,,
    pub raw_smp_processor_id(): this_cpu =,
    for_each_cpu(cpu, mask) {
    if (excl_self && cpu == this_cpu)
    pub 0): send_ipi(per_cpu(x86_cpu_to_apicid, cpu), vector,,
    }
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi_mask(mask: *const cpumask, vector: c_int) {
    static void savic_send_ipi_mask(const struct cpumask *mask, int vector)
    {
    pub false): send_ipi_mask(mask, vector,,
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi_mask_allbutself(mask: *const cpumask, vector: c_int) {
    static void savic_send_ipi_mask_allbutself(const struct cpumask *mask, int vector)
    {
    pub true): send_ipi_mask(mask, vector,,
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi_allbutself(vector: c_int) {
    static void savic_send_ipi_allbutself(int vector)
    {
    pub APIC_DEST_ALLBUT): send_ipi(0, vector,,
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi_all(vector: c_int) {
    static void savic_send_ipi_all(int vector)
    {
    pub APIC_DEST_ALLINC): send_ipi(0, vector,,
    }
#[no_mangle]
unsafe extern "C" fn savic_send_ipi_self(vector: c_int) {
    static void savic_send_ipi_self(int vector)
    {
    }
#[no_mangle]
unsafe extern "C" fn savic_update_vector(cpu: c_uint, vector: c_uint, set: bool) {
    static void savic_update_vector(unsigned int cpu, unsigned int vector, bool set)
    {
    pub set): update_vector(cpu, SAVIC_ALLOWED_IRR, vector,,
    }
#[no_mangle]
unsafe extern "C" fn savic_eoi() {
    static void savic_eoi(void)
    {
    pub cpu: c_uint,
    pub vec: c_int,
    pub raw_smp_processor_id(): cpu =,
    pub APIC_ISR)): vec = apic_find_highest_vector(get_reg_bitmap(cpu,,
    if (WARN_ONCE(vec == -1, "EOI write while no active interrupt in APIC_ISR"))
// Is level-triggered interrupt?
    if (apic_test_vector(vec, get_reg_bitmap(cpu, APIC_TMR))) {
    pub false): update_vector(cpu, APIC_ISR, vec,,
//
// Propagate the EOI write to the hypervisor for level-triggered
// interrupts. Return to the guest from GHCB protocol event takes
// care of re-evaluating interrupt state.
//
    pub 0): savic_ghcb_msr_write(APIC_EOI,,
    } else {
//
// Hardware clears APIC_ISR and re-evaluates the interrupt state
// to determine if there is any pending interrupt which can be
// delivered to CPU.
//
    }
    }
#[no_mangle]
unsafe extern "C" fn savic_teardown() {
    static void savic_teardown(void)
    {
// Disable Secure AVIC
    pub 0): native_wrmsrq(MSR_AMD64_SAVIC_CONTROL,,
    }
#[no_mangle]
unsafe extern "C" fn savic_setup() {
    static void savic_setup(void)
    {
    pub this_cpu_ptr(savic_page): *mut *mut void ap =,
    pub res: enum es_result,
    pub gpa: c_ulong,
//
// Before Secure AVIC is enabled, APIC MSR reads are intercepted.
// APIC_ID MSR read returns the value from the hypervisor.
//
    pub native_apic_msr_read(APIC_ID)): apic_set_reg(ap, APIC_ID,,
    pub __pa(ap): gpa =,
//
// The NPT entry for a vCPU's APIC backing page must always be
// present when the vCPU is running in order for Secure AVIC to
// function. A VMEXIT_BUSY is returned on VMRUN and the vCPU cannot
// be resumed if the NPT entry for the APIC backing page is not
// present. Notify GPA of the vCPU's APIC backing page to the
// hypervisor by calling savic_register_gpa(). Before executing
// VMRUN, the hypervisor makes use of this information to make sure
// the APIC backing page is mapped in NPT.
//
    pub savic_register_gpa(gpa): res =,
    if (res != ES_OK)
    pub GHCB_TERM_SAVIC_FAIL): sev_es_terminate(SEV_TERM_SET_LINUX,,
    native_wrmsrq(MSR_AMD64_SAVIC_CONTROL,
    pub MSR_AMD64_SAVIC_ALLOWEDNMI): gpa | MSR_AMD64_SAVIC_EN |,
    }
#[no_mangle]
unsafe extern "C" fn savic_probe() -> c_int {
    static int savic_probe(void)
    {
    if (!cc_platform_has(CC_ATTR_SNP_SECURE_AVIC))
    pub 0: return,
    if (!x2apic_mode) {
    pub mode\n"): pr_err("Secure AVIC enabled in non x2APIC,
    pub GHCB_TERM_SAVIC_FAIL): sev_es_terminate(SEV_TERM_SET_LINUX,,
// unreachable
    }
    pub secure_avic_page): savic_page = alloc_percpu(struct,
    if (!savic_page)
    pub GHCB_TERM_SAVIC_FAIL): sev_es_terminate(SEV_TERM_SET_LINUX,,
    pub 1: return,
    }
    static struct apic apic_x2apic_savic __ro_after_init = {
    .name				= "secure avic x2apic",
    .probe				= savic_probe,
    .acpi_madt_oem_check		= savic_acpi_madt_oem_check,
    .setup				= savic_setup,
    .teardown			= savic_teardown,
    .dest_mode_logical		= false,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .x2apic_set_max_apicid		= true,
    .get_apic_id			= x2apic_get_apic_id,
    .calc_dest_apicid		= apic_default_calc_apicid,
    .send_IPI			= savic_send_ipi,
    .send_IPI_mask			= savic_send_ipi_mask,
    .send_IPI_mask_allbutself	= savic_send_ipi_mask_allbutself,
    .send_IPI_allbutself		= savic_send_ipi_allbutself,
    .send_IPI_all			= savic_send_ipi_all,
    .send_IPI_self			= savic_send_ipi_self,
    .nmi_to_offline_cpu		= true,
    .read				= savic_read,
    .write				= savic_write,
    .eoi				= savic_eoi,
    .icr_read			= native_x2apic_icr_read,
    .icr_write			= savic_icr_write,
    .update_vector			= savic_update_vector,
}

    apic_driver(apic_x2apic_savic);
