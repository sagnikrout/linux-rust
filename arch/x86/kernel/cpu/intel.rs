//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/intel.c
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
// Processors which have self-snooping capability can handle conflicting
// memory type across CPUs by snooping its own cache. However, there exists
// CPU models in which having conflicting memory types still leads to
// unpredictable behavior, machine check errors, or hangs. Clear this
// feature to prevent its use on machines with known erratas.
//
#[no_mangle]
unsafe extern "C" fn check_memory_type_self_snoop_errata(c: *mut cpuinfo_x86) {
    static void check_memory_type_self_snoop_errata(struct cpuinfo_x86 *c)
    {
    switch (c.x86_vfm) {
    case INTEL_CORE_YONAH:
    case INTEL_CORE2_MEROM:
    case INTEL_CORE2_MEROM_L:
    case INTEL_CORE2_PENRYN:
    case INTEL_CORE2_DUNNINGTON:
    case INTEL_NEHALEM:
    case INTEL_NEHALEM_G:
    case INTEL_NEHALEM_EP:
    case INTEL_NEHALEM_EX:
    case INTEL_WESTMERE:
    case INTEL_WESTMERE_EP:
    case INTEL_SANDYBRIDGE:
    setup_clear_cpu_cap(X86_FEATURE_SELFSNOOP);
    }
    }
    static bool ring3mwait_disabled __read_mostly;
#[no_mangle]
unsafe extern "C" fn ring3mwait_disable(__unused: *mut c_char) -> int __init {
    static int __init ring3mwait_disable(char *__unused)
    {
    ring3mwait_disabled = true;
    return 1;
    }
    __setup("ring3mwait=disable", ring3mwait_disable);
#[no_mangle]
unsafe extern "C" fn probe_xeon_phi_r3mwait(c: *mut cpuinfo_x86) {
    static void probe_xeon_phi_r3mwait(struct cpuinfo_x86 *c)
    {
//
// Ring 3 MONITOR/MWAIT feature cannot be detected without
// cpu model and family comparison.
//
    if (c.x86 != 6)
    return;
    switch (c.x86_vfm) {
    case INTEL_XEON_PHI_KNL:
    case INTEL_XEON_PHI_KNM:
    break;
    default:
    return;
    }
    if (ring3mwait_disabled)
    return;
    set_cpu_cap(c, X86_FEATURE_RING3MWAIT);
    this_cpu_or(msr_misc_features_shadow,
    1UL << MSR_MISC_FEATURES_ENABLES_RING3MWAIT_BIT);
    if (c == &boot_cpu_data)
    ELF_HWCAP2 |= HWCAP2_RING3MWAIT;
    }
//
// Early microcode releases for the Spectre v2 mitigation were broken.
// Information taken from;
// - https://newsroom.intel.com/wp-content/uploads/sites/11/2018/03/microcode-update-guidance.pdf
// - https://kb.vmware.com/s/article/52345
// - Microcode revisions observed in the wild
// - Release note from 20180108 microcode release
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sku_microcode {
    pub vfm: u32,
    pub stepping: u8,
    pub microcode: u32,
}

    static const struct sku_microcode spectre_bad_microcodes[] = {
    { INTEL_KABYLAKE,	0x0B,	0x80 },
    { INTEL_KABYLAKE,	0x0A,	0x80 },
    { INTEL_KABYLAKE,	0x09,	0x80 },
    { INTEL_KABYLAKE_L,	0x0A,	0x80 },
    { INTEL_KABYLAKE_L,	0x09,	0x80 },
    { INTEL_SKYLAKE_X,	0x03,	0x0100013e },
    { INTEL_SKYLAKE_X,	0x04,	0x0200003c },
    { INTEL_BROADWELL,	0x04,	0x28 },
    { INTEL_BROADWELL_G,	0x01,	0x1b },
    { INTEL_BROADWELL_D,	0x02,	0x14 },
    { INTEL_BROADWELL_D,	0x03,	0x07000011 },
    { INTEL_BROADWELL_X,	0x01,	0x0b000025 },
    { INTEL_HASWELL_L,	0x01,	0x21 },
    { INTEL_HASWELL_G,	0x01,	0x18 },
    { INTEL_HASWELL,	0x03,	0x23 },
    { INTEL_HASWELL_X,	0x02,	0x3b },
    { INTEL_HASWELL_X,	0x04,	0x10 },
    { INTEL_IVYBRIDGE_X,	0x04,	0x42a },
// Observed in the wild
    { INTEL_SANDYBRIDGE_X,	0x06,	0x61b },
    { INTEL_SANDYBRIDGE_X,	0x07,	0x712 },
    };
#[no_mangle]
unsafe extern "C" fn bad_spectre_microcode(c: *mut cpuinfo_x86) -> bool {
    static bool bad_spectre_microcode(struct cpuinfo_x86 *c)
    {
    int i;
//
// We know that the hypervisor lie to us on the microcode version so
// we may as well hope that it is running the correct version.
//
    if (cpu_has(c, X86_FEATURE_HYPERVISOR))
    return false;
    for (i = 0; i < ARRAY_SIZE(spectre_bad_microcodes); i++) {
    if (c.x86_vfm == spectre_bad_microcodes[i].vfm &&
    c.x86_stepping == spectre_bad_microcodes[i].stepping)
    return (c.microcode <= spectre_bad_microcodes[i].microcode);
    }
    return false;
    }
pub const MSR_IA32_TME_ACTIVATE: c_uint = 0x982;
// Helpers to access TME_ACTIVATE MSR

#[no_mangle]
unsafe extern "C" fn detect_tme_early(c: *mut cpuinfo_x86) {
    static void detect_tme_early(struct cpuinfo_x86 *c)
    {
    u64 tme_activate;
    int keyid_bits;
    rdmsrq(MSR_IA32_TME_ACTIVATE, tme_activate);
    if (!TME_ACTIVATE_LOCKED(tme_activate) || !TME_ACTIVATE_ENABLED(tme_activate)) {
    pr_info_once("x86/tme: not enabled by BIOS\n");
    clear_cpu_cap(c, X86_FEATURE_TME);
    return;
    }
    pr_info_once("x86/tme: enabled by BIOS\n");
    keyid_bits = TME_ACTIVATE_KEYID_BITS(tme_activate);
    if (!keyid_bits)
    return;
//
// KeyID bits are set by BIOS and can be present regardless
// of whether the kernel is using them. They effectively lower
// the number of physical address bits.
//
// Update cpuinfo_x86::x86_phys_bits accordingly.
//
    c.x86_phys_bits -= keyid_bits;
    pr_info_once("x86/mktme: BIOS enabled: x86_phys_bits reduced by %d\n",
    keyid_bits);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_unlock_cpuid_leafs(c: *mut cpuinfo_x86) {
    void intel_unlock_cpuid_leafs(struct cpuinfo_x86 *c)
    {
    if (boot_cpu_data.x86_vendor != X86_VENDOR_INTEL)
    return;
    if (c.x86_vfm < INTEL_PENTIUM_M_DOTHAN)
    return;
//
// The BIOS can have limited CPUID to leaf 2, which breaks feature
// enumeration. Unlock it and update the maximum leaf info.
//
    if (msr_clear_bit(MSR_IA32_MISC_ENABLE, MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT) > 0)
    c.cpuid_level = cpuid_eax(0);
    }
//
// Use CPUID to generate a "vfm" value. Useful before cpuinfo_x86
// structures are populated.
//
#[no_mangle]
unsafe extern "C" fn intel_cpuid_vfm() -> u32 {
    static u32 intel_cpuid_vfm(void)
    {
    let mut eax: u32 = cpuid_eax(1);
    let mut fam: u32 = x86_family(eax);
    let mut model: u32 = x86_model(eax);
    return IFM(fam, model);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_get_platform_id() -> u32 {
    u32 intel_get_platform_id(void)
    {
    unsigned int val[2];
    if (x86_hypervisor_present)
    return 0;
//
// This can be called early. Use CPUID directly instead of
// relying on cpuinfo_x86 which may not be fully initialized.
// The PII does not have MSR_IA32_PLATFORM_ID. Everything
// before _it_ has no microcode (for Linux at least).
//
    if (intel_cpuid_vfm() <= INTEL_PENTIUM_II_KLAMATH)
    return 0;
// get processor flags from MSR 0x17
    native_rdmsr(MSR_IA32_PLATFORM_ID, val[0], val[1]);
    return (val[1] >> 18) & 7;
    }
#[no_mangle]
unsafe extern "C" fn early_init_intel(c: *mut cpuinfo_x86) {
    static void early_init_intel(struct cpuinfo_x86 *c)
    {
    u64 misc_enable;
    if (c.x86 >= 6 && !cpu_has(c, X86_FEATURE_IA64))
    c.microcode = intel_get_microcode_revision();
    c.intel_platform_id = intel_get_platform_id();
// Now if any of them are set, check the blacklist and clear the lot
    if ((cpu_has(c, X86_FEATURE_SPEC_CTRL) ||
    cpu_has(c, X86_FEATURE_INTEL_STIBP) ||
    cpu_has(c, X86_FEATURE_IBRS) || cpu_has(c, X86_FEATURE_IBPB) ||
    cpu_has(c, X86_FEATURE_STIBP)) && bad_spectre_microcode(c)) {
    pr_warn("Intel Spectre v2 broken microcode detected; disabling Speculation Control\n");
    setup_clear_cpu_cap(X86_FEATURE_IBRS);
    setup_clear_cpu_cap(X86_FEATURE_IBPB);
    setup_clear_cpu_cap(X86_FEATURE_STIBP);
    setup_clear_cpu_cap(X86_FEATURE_SPEC_CTRL);
    setup_clear_cpu_cap(X86_FEATURE_MSR_SPEC_CTRL);
    setup_clear_cpu_cap(X86_FEATURE_INTEL_STIBP);
    setup_clear_cpu_cap(X86_FEATURE_SSBD);
    setup_clear_cpu_cap(X86_FEATURE_SPEC_CTRL_SSBD);
    }
//
// Atom erratum AAE44/AAF40/AAG38/AAH41:
//
// A race condition between speculative fetches and invalidating
// a large page.  This is worked around in microcode, but we
// need the microcode to have already been loaded... so if it is
// not, recommend a BIOS update and disable large pages.
//
    if (c.x86_vfm == INTEL_ATOM_BONNELL && c.x86_stepping <= 2 &&
    c.microcode < 0x20e) {
    pr_warn("Atom PSE erratum detected, BIOS microcode update recommended\n");
    clear_cpu_cap(c, X86_FEATURE_PSE);
    }

// Netburst reports 64 bytes clflush size, but does IO in 128 bytes
    if (c.x86 == 15 && c.x86_cache_alignment == 64)
    c.x86_cache_alignment = 128;

// CPUID workaround for 0F33/0F34 CPU
    if (c.x86_vfm == INTEL_P4_PRESCOTT &&
    (c.x86_stepping == 0x3 || c.x86_stepping == 0x4))
    c.x86_phys_bits = 36;
//
// c->x86_power is 8000_0007 edx. Bit 8 is TSC runs at constant rate
// with P/T states and does not stop in deep C-states.
//
// It is also reliable across cores and sockets. (but not across
// cabinets - we turn it off in that case explicitly.)
//
// Use a model-specific check for some older CPUs that have invariant
// TSC but may not report it architecturally via 8000_0007.
//
    if (c.x86_power & (1 << 8)) {
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    } else if ((c.x86_vfm >= INTEL_P4_PRESCOTT && c.x86_vfm <= INTEL_P4_CEDARMILL) ||
    (c.x86_vfm >= INTEL_CORE_YONAH  && c.x86_vfm <= INTEL_IVYBRIDGE)) {
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    }
// Penwell and Cloverview have the TSC which doesn't sleep on S3
    switch (c.x86_vfm) {
    case INTEL_ATOM_SALTWELL_MID:
    case INTEL_ATOM_SALTWELL_TABLET:
    case INTEL_ATOM_SILVERMONT_MID:
    case INTEL_ATOM_AIRMONT_NP:
    set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC_S3);
    break;
    }
//
// PAT is broken on early family 6 CPUs, the last of which
// is "Yonah" where the erratum is named "AN7":
//
// Page with PAT (Page Attribute Table) Set to USWC
// (Uncacheable Speculative Write Combine) While
// Associated MTRR (Memory Type Range Register) Is UC
// (Uncacheable) May Consolidate to UC
//
// Disable PAT and fall back to MTRR on these CPUs.
//
    if (c.x86_vfm >= INTEL_PENTIUM_PRO &&
    c.x86_vfm <= INTEL_CORE_YONAH)
    clear_cpu_cap(c, X86_FEATURE_PAT);
//
// Modern CPUs are generally expected to have a sane fast string
// implementation. However, BIOSes typically have a knob to tweak
// the architectural MISC_ENABLE.FAST_STRING enable bit.
//
// Adhere to the preference and program the Linux-defined fast
// string flag and enhanced fast string capabilities accordingly.
//
    if (c.x86_vfm >= INTEL_PENTIUM_M_DOTHAN) {
    rdmsrq(MSR_IA32_MISC_ENABLE, misc_enable);
    if (misc_enable & MSR_IA32_MISC_ENABLE_FAST_STRING) {
// X86_FEATURE_ERMS is set based on CPUID
    set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    } else {
    pr_info("Disabled fast string operations\n");
    setup_clear_cpu_cap(X86_FEATURE_REP_GOOD);
    setup_clear_cpu_cap(X86_FEATURE_ERMS);
    }
    }
//
// Intel Quark Core DevMan_001.pdf section 6.4.11
// "The operating system also is required to invalidate (i.e., flush)
// the TLB when any changes are made to any of the page table entries.
// The operating system must reload CR3 to cause the TLB to be flushed"
//
// As a result, boot_cpu_has(X86_FEATURE_PGE) in arch/x86/include/asm/tlbflush.h
// should be false so that __flush_tlb_all() causes CR3 instead of CR4.PGE
// to be modified.
//
    if (c.x86_vfm == INTEL_QUARK_X1000) {
    pr_info("Disabling PGE capability bit\n");
    setup_clear_cpu_cap(X86_FEATURE_PGE);
    }
    check_memory_type_self_snoop_errata(c);
//
// Adjust the number of physical bits early because it affects the
// valid bits of the MTRR mask registers.
//
    if (cpu_has(c, X86_FEATURE_TME))
    detect_tme_early(c);
    }
#[no_mangle]
unsafe extern "C" fn bsp_init_intel(c: *mut cpuinfo_x86) {
    static void bsp_init_intel(struct cpuinfo_x86 *c)
    {
    resctrl_cpu_detect(c);
    }

//
// Early probe support logic for ppro memory erratum #50
//
// This is called before we do cpu ident work
//
#[no_mangle]
pub unsafe extern "C" fn ppro_with_ram_bug() -> c_int {
    int ppro_with_ram_bug(void)
    {
// Uses data from early_cpu_detect now
    if (boot_cpu_data.x86_vfm == INTEL_PENTIUM_PRO &&
    boot_cpu_data.x86_stepping < 8) {
    pr_info("Pentium Pro with Errata#50 detected. Taking evasive action.\n");
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_smp_check(c: *mut cpuinfo_x86) {
    static void intel_smp_check(struct cpuinfo_x86 *c)
    {
// calling is from identify_secondary_cpu() ?
    if (!c.cpu_index)
    return;
//
// Mask B, Pentium, but not Pentium MMX
//
    if (c.x86_vfm >= INTEL_FAM5_START && c.x86_vfm < INTEL_PENTIUM_MMX &&
    c.x86_stepping >= 1 && c.x86_stepping <= 4) {
//
// Remember we have B step Pentia with bugs
//
    WARN_ONCE(1, "WARNING: SMP operation may be unreliable"
    "with B stepping processors.\n");
    }
    }
    static int forcepae;
#[no_mangle]
unsafe extern "C" fn forcepae_setup(__unused: *mut c_char) -> int __init {
    static int __init forcepae_setup(char *__unused)
    {
    forcepae = 1;
    return 1;
    }
    __setup("forcepae", forcepae_setup);
#[no_mangle]
unsafe extern "C" fn intel_workarounds(c: *mut cpuinfo_x86) {
    static void intel_workarounds(struct cpuinfo_x86 *c)
    {
//
// All models of Pentium and Pentium with MMX technology CPUs
// have the F0 0F bug, which lets nonprivileged users lock up the
// system. The fault handler always checks for it.
// The Quark is also family 5, but does not have the same bug.
//
    if (IS_ENABLED(CONFIG_X86_F00F_BUG) &&
    (c.x86_vfm >= INTEL_FAM5_START && c.x86_vfm < INTEL_QUARK_X1000))
    set_cpu_bug(c, X86_BUG_F00F);
//
// SEP CPUID bug: Pentium Pro reports SEP but doesn't have it until
// model 3 mask 3
//
    if ((c.x86_vfm == INTEL_PENTIUM_II_KLAMATH && c.x86_stepping < 3) ||
    c.x86_vfm < INTEL_PENTIUM_II_KLAMATH)
    clear_cpu_cap(c, X86_FEATURE_SEP);
//
// PAE CPUID issue: many Pentium M report no PAE but may have a
// functionally usable PAE implementation.
// Forcefully enable PAE if kernel parameter "forcepae" is present.
//
    if (forcepae) {
    pr_warn("PAE forced!\n");
    set_cpu_cap(c, X86_FEATURE_PAE);
    add_taint(TAINT_CPU_OUT_OF_SPEC, LOCKDEP_NOW_UNRELIABLE);
    }
//
// P4 Xeon erratum 037 workaround.
// Hardware prefetcher may cause stale data to be loaded into the cache.
//
    if (c.x86_vfm == INTEL_P4_WILLAMETTE && c.x86_stepping == 1) {
    if (msr_set_bit(MSR_IA32_MISC_ENABLE,
    MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT) > 0) {
    pr_info("CPU: C0 stepping P4 Xeon detected.\n");
    pr_info("CPU: Disabling hardware prefetching (Erratum 037)\n");
    }
    }
//
// See if we have a good local APIC by checking for buggy Pentia,
// i.e. all B steppings and the C2 stepping of P54C when using their
// integrated APIC (see 11AP erratum in "Pentium Processor
// Specification Update").
//
    if (boot_cpu_has(X86_FEATURE_APIC) && c.x86_vfm == INTEL_PENTIUM_75 &&
    (c.x86_stepping < 0x6 || c.x86_stepping == 0xb))
    set_cpu_bug(c, X86_BUG_11AP);

//
// MOVSL bulk memory moves can be slow when source and dest are not
// both 8-byte aligned. PII/PIII only like MOVSL with 8-byte alignment.
//
// Set the preferred alignment for Pentium Pro and newer processors, as
// it has only been tested on these.
//
    if (c.x86_vfm >= INTEL_PENTIUM_PRO)
    movsl_mask.mask = 7;

    intel_smp_check(c);
    }

#[no_mangle]
unsafe extern "C" fn intel_workarounds(c: *mut cpuinfo_x86) {
    static void intel_workarounds(struct cpuinfo_x86 *c)
    {
    }

#[no_mangle]
unsafe extern "C" fn srat_detect_node(c: *mut cpuinfo_x86) {
    static void srat_detect_node(struct cpuinfo_x86 *c)
    {

    unsigned node;
    let mut cpu: c_int = smp_processor_id();
// Don't do the funky fallback heuristics the AMD version employs
    for now. */
    node = numa_cpu_node(cpu);
    if (node == NUMA_NO_NODE || !node_online(node)) {
// reuse the value from init_cpu_to_node()
    node = cpu_to_node(cpu);
    }
    numa_set_node(cpu, node);

    }
#[no_mangle]
unsafe extern "C" fn init_cpuid_fault(c: *mut cpuinfo_x86) {
    static void init_cpuid_fault(struct cpuinfo_x86 *c)
    {
    u64 msr;
    if (!rdmsrq_safe(MSR_PLATFORM_INFO, &msr)) {
    if (msr & MSR_PLATFORM_INFO_CPUID_FAULT)
    set_cpu_cap(c, X86_FEATURE_CPUID_FAULT);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_intel_misc_features(c: *mut cpuinfo_x86) {
    static void init_intel_misc_features(struct cpuinfo_x86 *c)
    {
    u64 msr;
    if (rdmsrq_safe(MSR_MISC_FEATURES_ENABLES, &msr))
    return;
// Clear all MISC features
    this_cpu_write(msr_misc_features_shadow, 0);
// Check features and update capabilities and shadow control bits
    init_cpuid_fault(c);
    probe_xeon_phi_r3mwait(c);
    msr = this_cpu_read(msr_misc_features_shadow);
    wrmsrq(MSR_MISC_FEATURES_ENABLES, msr);
    }
//
// This is a list of Intel CPUs that are known to suffer from downclocking when
// ZMM registers (512-bit vectors) are used.  On these CPUs, when the kernel
// executes SIMD-optimized code such as cryptography functions or CRCs, it
// should prefer 256-bit (YMM) code to 512-bit (ZMM) code.
//
    static const struct x86_cpu_id zmm_exclusion_list[] = {
    X86_MATCH_VFM(INTEL_SKYLAKE_X,		0),
    X86_MATCH_VFM(INTEL_ICELAKE_X,		0),
    X86_MATCH_VFM(INTEL_ICELAKE_D,		0),
    X86_MATCH_VFM(INTEL_ICELAKE,		0),
    X86_MATCH_VFM(INTEL_ICELAKE_L,		0),
    X86_MATCH_VFM(INTEL_ICELAKE_NNPI,	0),
    X86_MATCH_VFM(INTEL_TIGERLAKE_L,	0),
    X86_MATCH_VFM(INTEL_TIGERLAKE,		0),
// Allow Rocket Lake and later, and Sapphire Rapids and later.
    {},
    };
#[no_mangle]
unsafe extern "C" fn init_intel(c: *mut cpuinfo_x86) {
    static void init_intel(struct cpuinfo_x86 *c)
    {
    early_init_intel(c);
    intel_workarounds(c);
    init_intel_cacheinfo(c);
    if (c.cpuid_level > 9) {
    let mut eax: unsigned = cpuid_eax(10);
// Check for version and the number of counters
    if ((eax & 0xff) && (((eax>>8) & 0xff) > 1))
    set_cpu_cap(c, X86_FEATURE_ARCH_PERFMON);
    }
    if (cpu_has(c, X86_FEATURE_XMM2))
    set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);
    if (boot_cpu_has(X86_FEATURE_DS)) {
    u64 l;
    rdmsrq(MSR_IA32_MISC_ENABLE, l);
    if (!(l & MSR_IA32_MISC_ENABLE_BTS_UNAVAIL))
    set_cpu_cap(c, X86_FEATURE_BTS);
    if (!(l & MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL))
    set_cpu_cap(c, X86_FEATURE_PEBS);
    }
    if (boot_cpu_has(X86_FEATURE_CLFLUSH) &&
    (c.x86_vfm == INTEL_CORE2_DUNNINGTON ||
    c.x86_vfm == INTEL_NEHALEM_EX ||
    c.x86_vfm == INTEL_WESTMERE_EX))
    set_cpu_bug(c, X86_BUG_CLFLUSH_MONITOR);
    if (boot_cpu_has(X86_FEATURE_MWAIT) &&
    (c.x86_vfm == INTEL_ATOM_GOLDMONT ||
    c.x86_vfm == INTEL_LUNARLAKE_M))
    set_cpu_bug(c, X86_BUG_MONITOR);

    if (c.x86 == 15)
    c.x86_cache_alignment = c.x86_clflush_size * 2;

//
// Names for the Pentium II/Celeron processors
// detectable only by also checking the cache size.
// Dixon is NOT a Celeron.
//
    if (c.x86 == 6) {
    let mut l2: c_uint = c.x86_cache_size;
    char *p = core::ptr::null_mut();
    switch (c.x86_model) {
    case 5:
    if (l2 == 0)
    p = "Celeron (Covington)";
#[no_mangle]
pub unsafe extern "C" fn if(256: l2 ==) -> else {
    else if (l2 == 256)
    p = "Mobile Pentium II (Dixon)";
    break;
    case 6:
    if (l2 == 128)
    p = "Celeron (Mendocino)";
#[no_mangle]
pub unsafe extern "C" fn if(5: c->x86_stepping == 0 || c->x86_stepping ==) -> else {
    else if (c.x86_stepping == 0 || c.x86_stepping == 5)
    p = "Celeron-A";
    break;
    case 8:
    if (l2 == 128)
    p = "Celeron (Coppermine)";
    break;
    }
    if (p)
    strcpy(c.x86_model_id, p);
    }

    if (x86_match_cpu(zmm_exclusion_list))
    set_cpu_cap(c, X86_FEATURE_PREFER_YMM);
// Work around errata
    srat_detect_node(c);
    init_ia32_feat_ctl(c);
    init_intel_misc_features(c);
    split_lock_init();
    intel_init_thermal(c);
    }

#[no_mangle]
unsafe extern "C" fn intel_size_cache(c: *mut cpuinfo_x86, size: c_uint) -> c_uint {
    static unsigned int intel_size_cache(struct cpuinfo_x86 *c, unsigned int size)
    {
//
// Intel PIII Tualatin. This comes in two flavours.
// One has 256kb of cache, the other 512. We have no way
// to determine which, so we use a boottime override
// for the 512kb model, and assume 256 otherwise.
//
    if (c.x86_vfm == INTEL_PENTIUM_III_TUALATIN && size == 0)
    size = 256;
//
// Intel Quark SoC X1000 contains a 4-way set associative
// 16K cache with a 16 byte cache line and 256 lines per tag
//
    if (c.x86_vfm == INTEL_QUARK_X1000)
    size = 16;
    return size;
    }

#[no_mangle]
unsafe extern "C" fn intel_tlb_lookup(desc: *const leaf_0x2_table) {
    static void intel_tlb_lookup(const struct leaf_0x2_table *desc)
    {
    let mut entries: c_short = desc.entries;
    switch (desc.t_type) {
    case STLB_4K:
    tlb_lli_4k = max(tlb_lli_4k, entries);
    tlb_lld_4k = max(tlb_lld_4k, entries);
    break;
    case STLB_4K_2M:
    tlb_lli_4k = max(tlb_lli_4k, entries);
    tlb_lld_4k = max(tlb_lld_4k, entries);
    tlb_lli_2m = max(tlb_lli_2m, entries);
    tlb_lld_2m = max(tlb_lld_2m, entries);
    tlb_lli_4m = max(tlb_lli_4m, entries);
    tlb_lld_4m = max(tlb_lld_4m, entries);
    break;
    case TLB_INST_ALL:
    tlb_lli_4k = max(tlb_lli_4k, entries);
    tlb_lli_2m = max(tlb_lli_2m, entries);
    tlb_lli_4m = max(tlb_lli_4m, entries);
    break;
    case TLB_INST_4K:
    tlb_lli_4k = max(tlb_lli_4k, entries);
    break;
    case TLB_INST_4M:
    tlb_lli_4m = max(tlb_lli_4m, entries);
    break;
    case TLB_INST_2M_4M:
    tlb_lli_2m = max(tlb_lli_2m, entries);
    tlb_lli_4m = max(tlb_lli_4m, entries);
    break;
    case TLB_DATA_4K:
    case TLB_DATA0_4K:
    tlb_lld_4k = max(tlb_lld_4k, entries);
    break;
    case TLB_DATA_4M:
    case TLB_DATA0_4M:
    tlb_lld_4m = max(tlb_lld_4m, entries);
    break;
    case TLB_DATA_2M_4M:
    case TLB_DATA0_2M_4M:
    tlb_lld_2m = max(tlb_lld_2m, entries);
    tlb_lld_4m = max(tlb_lld_4m, entries);
    break;
    case TLB_DATA_4K_4M:
    tlb_lld_4k = max(tlb_lld_4k, entries);
    tlb_lld_4m = max(tlb_lld_4m, entries);
    break;
    case TLB_DATA_1G_2M_4M:
    tlb_lld_2m = max(tlb_lld_2m, TLB_0x63_2M_4M_ENTRIES);
    tlb_lld_4m = max(tlb_lld_4m, TLB_0x63_2M_4M_ENTRIES);
    fallthrough;
    case TLB_DATA_1G:
    tlb_lld_1g = max(tlb_lld_1g, entries);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn intel_detect_tlb(c: *mut cpuinfo_x86) {
    static void intel_detect_tlb(struct cpuinfo_x86 *c)
    {
    const struct leaf_0x2_table *desc;
    union leaf_0x2_regs regs;
    u8 *ptr;
    if (c.cpuid_level < 2)
    return;
    cpuid_leaf_0x2(&regs);
    for_each_cpuid_0x2_desc(regs, ptr, desc)
    intel_tlb_lookup(desc);
    }
    static const struct cpu_dev intel_cpu_dev = {
    .c_vendor	= "Intel",
    .c_ident	= { "GenuineIntel" },

    .legacy_models = {
    { .family = 4, .model_names =
    {
    [0] = "486 DX-25/33",
    [1] = "486 DX-50",
    [2] = "486 SX",
    [3] = "486 DX/2",
    [4] = "486 SL",
    [5] = "486 SX/2",
    [7] = "486 DX/2-WB",
    [8] = "486 DX/4",
    [9] = "486 DX/4-WB"
    }
    },
    { .family = 5, .model_names =
    {
    [0] = "Pentium 60/66 A-step",
    [1] = "Pentium 60/66",
    [2] = "Pentium 75 - 200",
    [3] = "OverDrive PODP5V83",
    [4] = "Pentium MMX",
    [7] = "Mobile Pentium 75 - 200",
    [8] = "Mobile Pentium MMX",
    [9] = "Quark SoC X1000",
    }
    },
    { .family = 6, .model_names =
    {
    [0] = "Pentium Pro A-step",
    [1] = "Pentium Pro",
    [3] = "Pentium II (Klamath)",
    [4] = "Pentium II (Deschutes)",
    [5] = "Pentium II (Deschutes)",
    [6] = "Mobile Pentium II",
    [7] = "Pentium III (Katmai)",
    [8] = "Pentium III (Coppermine)",
    [10] = "Pentium III (Cascades)",
    [11] = "Pentium III (Tualatin)",
    }
    },
    { .family = 15, .model_names =
    {
    [0] = "Pentium 4 (Unknown)",
    [1] = "Pentium 4 (Willamette)",
    [2] = "Pentium 4 (Northwood)",
    [4] = "Pentium 4 (Foster)",
    [5] = "Pentium 4 (Foster)",
    }
    },
    },
    .legacy_cache_size = intel_size_cache,

    .c_detect_tlb	= intel_detect_tlb,
    .c_early_init   = early_init_intel,
    .c_bsp_init	= bsp_init_intel,
    .c_init		= init_intel,
    .c_x86_vendor	= X86_VENDOR_INTEL,
    };
    cpu_dev_register(intel_cpu_dev);
