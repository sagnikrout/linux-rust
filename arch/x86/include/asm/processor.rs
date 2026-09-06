//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/processor.h
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

// Forward declaration, a strange C thing

//
// We handle most unaligned accesses in hardware.  On the other hand
// unaligned DMA can be quite expensive on some Nehalem processors.
//
// Based on this we disable the IP header alignment in network drivers.
//
pub const NET_IP_ALIGN: c_int = 0;
pub const HBP_NUM: c_int = 4;
//
// These alignment constraints are for performance in the vSMP case,
// but in the task_struct case we must also meet hardware imposed
// alignment requirements of the FPU state:
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_topology_cpu_type {
// X86_CPU_TYPE_ANY
    TOPO_CPU_TYPE_ANY = 0,
    TOPO_CPU_TYPE_PERFORMANCE,
    TOPO_CPU_TYPE_EFFICIENCY,
    TOPO_CPU_TYPE_LOW_POWER,
    TOPO_CPU_TYPE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_topology {
// Real APIC ID read from the local APIC
    pub apicid: u32,
// The initial APIC ID provided by CPUID
    pub initial_apicid: u32,
// Physical package ID
    pub pkg_id: u32,
// Physical die ID on AMD, Relative on Intel
    pub die_id: u32,
// Compute unit ID - AMD specific
    pub cu_id: u32,
// Core ID relative to the package
    pub core_id: u32,
// Logical ID mappings
    pub logical_pkg_id: u32,
    pub logical_die_id: u32,
    pub logical_core_id: u32,
// AMD Node ID and Nodes per Package info
    pub amd_node_id: u32,
// Cache level topology IDs
    pub llc_id: u32,
    pub l2c_id: u32,
// Hardware defined CPU-type
    pub hw_cpu_type: u32,
// CPUID.1A.EAX[23-0]
    pub :24: u32 intel_native_model_id,
// CPUID.1A.EAX[31-24]
    pub :8: u32 intel_type,
}

// CPUID 0x80000026.EBX
// Linux vendor-agnostic CPU type
//
// CPU type and hardware bug flags. Kept separately for each CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_x86 {
//
// The particular ordering (low-to-high) of (vendor,
// family, model) is done in case range of models, like
// it is usually done on AMD, need to be compared.
//
    pub x86_model: __u8,
// CPU family
    pub x86: __u8,
// CPU vendor
    pub x86_vendor: __u8,
    pub x86_reserved: __u8,
}

// combined vendor, family, model
// MSR_IA32_PLATFORM_ID[52-50]

// Number of 4K pages in DTLB/ITLB combined(in pages):

// Max extended CPUID function supported:
// Maximum supported CPUID level, -1=no CPUID:
//
// Align to size of unsigned long because the x86_capability array
// is passed to bitops which require the alignment. Use unnamed
// union to enforce the array is aligned to size of unsigned long.
//
// in KB - valid for CPUS which support this call:
// Cache QoS architectural values, valid only on the BSP:
// protected processor identification number
// number of cores as seen by the OS:
// Index into per_cpu list:
// Is SMT active on this core?
// Address space bits used by the cache internally
pub const X86_VENDOR_INTEL: c_int = 0;
pub const X86_VENDOR_CYRIX: c_int = 1;
pub const X86_VENDOR_AMD: c_int = 2;
pub const X86_VENDOR_UMC: c_int = 3;
pub const X86_VENDOR_CENTAUR: c_int = 5;
pub const X86_VENDOR_TRANSMETA: c_int = 7;
pub const X86_VENDOR_NSC: c_int = 8;
pub const X86_VENDOR_HYGON: c_int = 9;
pub const X86_VENDOR_ZHAOXIN: c_int = 10;
pub const X86_VENDOR_VORTEX: c_int = 11;
pub const X86_VENDOR_NUM: c_int = 12;
pub const X86_VENDOR_UNKNOWN: c_uint = 0xff;
//
// capabilities of CPUs
//

extern "C" {
    pub fn cpu_detect(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn BIT_ULL(PAGE_SHIFT: boot_cpu_data.x86_cache_bits - 1 -) -> return;
}
extern "C" {
    pub fn init_cpu_devs();
}
extern "C" {
    pub fn get_cpu_vendor(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn early_cpu_init();
}
extern "C" {
    pub fn identify_secondary_cpu(cpu: c_uint);
}
extern "C" {
    pub fn print_cpu_info(: *mut cpuinfo_x86);
}
extern "C" {
    pub fn print_cpu_msr(: *mut cpuinfo_x86);
}
extern "C" {
    pub fn intel_get_platform_id() -> u32;
}
//
// Friendlier CR3 helpers.
//
// Note that while the legacy 'TSS' name comes from 'Task State Segment',
// on modern x86 CPUs the TSS also holds information important to 64-bit mode,
// unrelated to the task-switch mechanism:
//

// This is the TSS defined by the hardware.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_hw_tss {
    pub __blh: unsigned short back_link,,
    pub sp0: c_ulong,
    pub __ss0h: unsigned short ss0,,
    pub sp1: c_ulong,
//
// We don't use ring 1, so ss1 is a convenient scratch space in
// the same cacheline as sp0.  We use ss1 to cache the value in
// MSR_IA32_SYSENTER_CS.  When we context switch
// MSR_IA32_SYSENTER_CS, we first check if the new value being
// written matches ss1, and, if it's not, then we wrmsr the new
// value and update ss1.
//
// The only reason we context switch MSR_IA32_SYSENTER_CS is
// that we set it to zero in vm86 tasks to avoid corrupting the
// stack if we were to go through the sysenter path from vm86
// mode.
//
    pub /: *mut *mut unsigned short ss1; / MSR_IA32_SYSENTER_CS,
    pub __ss1h: c_ushort,
    pub sp2: c_ulong,
    pub __ss2h: unsigned short ss2,,
    pub __cr3: c_ulong,
    pub ip: c_ulong,
    pub flags: c_ulong,
    pub ax: c_ulong,
    pub cx: c_ulong,
    pub dx: c_ulong,
    pub bx: c_ulong,
    pub sp: c_ulong,
    pub bp: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,
    pub __esh: unsigned short es,,
    pub __csh: unsigned short cs,,
    pub __ssh: unsigned short ss,,
    pub __dsh: unsigned short ds,,
    pub __fsh: unsigned short fs,,
    pub __gsh: unsigned short gs,,
    pub __ldth: unsigned short ldt,,
    pub trace: c_ushort,
    pub io_bitmap_base: c_ushort,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_hw_tss {
    pub reserved1: u32,
    pub sp0: u64,
    pub sp1: u64,
//
// Since Linux does not use ring 2, the 'sp2' slot is unused by
// hardware.  entry_SYSCALL_64 uses it as scratch space to stash
// the user RSP value.
//
    pub sp2: u64,
    pub reserved2: u64,
    pub ist: [u64; 7],
    pub reserved3: u32,
    pub reserved4: u32,
    pub reserved5: u16,
    pub io_bitmap_base: u16,
    pub __attribute__((packed)): },

//
// IO-bitmap sizes:
//
pub const IO_BITMAP_BITS: c_int = 65536;

//
// sizeof(unsigned long) coming from an extra "long" at the end of the
// iobitmap. The limit is inclusive, i.e. the last valid byte.
//

// Base offset outside of TSS_LIMIT so unpriviledged IO causes #GP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry_stack {
    pub stack: [c_char; PAGE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry_stack_page {
    pub stack: entry_stack,
    pub __aligned(PAGE_SIZE): },
//
// All IO bitmap related data stored in the TSS:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_io_bitmap {
// The sequence number of the last active bitmap.
    pub prev_sequence: u64,
//
// Store the dirty size of the last io bitmap offender. The next
// one will have to do the cleanup as the switch out to a non io
// bitmap user will just set x86_tss.io_bitmap_base to a value
// outside of the TSS limit. So for sane tasks there is no need to
// actually touch the io_bitmap at all.
//
    pub prev_max: c_uint,
//
// The extra 1 is there because the CPU will access an
// additional byte beyond the end of the IO permission
// bitmap. The extra byte must be all 1 bits, and must
// be within the limit.
//
    pub 1]: unsigned long bitmap[IO_BITMAP_LONGS +,
//
// Special I/O bitmap to emulate IOPL(3). All bytes zero,
// except the additional byte at the end.
//
    pub 1]: unsigned long mapall[IO_BITMAP_LONGS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tss_struct {
//
// The fixed hardware portion.  This must not cross a page boundary
// at risk of violating the SDM's advice and potentially triggering
// errata.
//
    pub x86_tss: x86_hw_tss,
    pub io_bitmap: x86_io_bitmap,
    pub __aligned(PAGE_SIZE): },
    pub cpu_tss_rw): DECLARE_PER_CPU_PAGE_ALIGNED(struct tss_struct,,
// Per CPU interrupt stacks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_stack {
    pub stack: [c_char; IRQ_STACK_SIZE],
    pub __aligned(IRQ_STACK_SIZE): },
    pub hardirq_stack_ptr): *mut *mut DECLARE_PER_CPU_CACHE_HOT(struct irq_stack ,,

    pub hardirq_stack_inuse): DECLARE_PER_CPU_CACHE_HOT(bool,,

    pub softirq_stack_ptr): *mut *mut DECLARE_PER_CPU_CACHE_HOT(struct irq_stack ,,

    pub cpu_current_top_of_stack): DECLARE_PER_CPU_CACHE_HOT(unsigned long,,
// const-qualified alias provided by the linker.

    pub per_cpu_offset(cpu): return,

    pub 0: return,

    pub entry_SYSCALL32_ignore(void): extern asmlinkage void,
// Save actual FS/GS selectors and bases to current->thread
    pub current_save_fsgs(void): c_void,

    pub perf_event: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
// Cached TLS descriptors:
    pub tls_array: [desc_struct; GDT_ENTRY_TLS_ENTRIES],
    pub sp0: c_ulong,

    pub sp: c_ulong,

    pub sysenter_cs: c_ulong,

    pub es: c_ushort,
    pub ds: c_ushort,
    pub fsindex: c_ushort,
    pub gsindex: c_ushort,

    pub fsbase: c_ulong,
    pub gsbase: c_ulong,

//
// XXX: this could presumably be unsigned short.  Alternatively,
// 32-bit kernels could be taught to use fsindex instead.
//
    pub fs: c_ulong,
    pub gs: c_ulong,

// Save middle states of ptrace breakpoints
    pub ptrace_bps: [*mut perf_event; HBP_NUM],
// Debug status used for traps, single steps, etc...
    pub virtual_dr6: c_ulong,
// Keep track of the exact dr7 value set by the user
    pub ptrace_dr7: c_ulong,
// Fault info:
    pub cr2: c_ulong,
    pub trap_nr: c_ulong,
    pub error_code: c_ulong,

// Virtual 86 mode info
    pub vm86: *mut vm86,

// IO permissions:
    pub io_bitmap: *mut io_bitmap,
//
// IOPL. Privilege level dependent I/O permission which is
// emulated via the I/O bitmap to prevent user space from disabling
// interrupts.
//
    pub iopl_emul: c_ulong,
    pub iopl_warn:1: c_uint,
//
// Protection Keys Register for Userspace.  Loaded immediately on
// context switch. Store it in thread_struct to avoid a lookup in
// the tasks's FPU xstate buffer. This value is only valid when a
// task is scheduled out. For 'current' the authoritative source of
// PKRU is the hardware itself.
//
    pub pkru: u32,

    pub features: c_ulong,
    pub features_locked: c_ulong,
    pub shstk: thread_shstk,

}

extern "C" {
    pub fn fpu_thread_struct_whitelist(offset: *mut c_ulong, size: *mut c_ulong);
}

extern "C" {
    pub fn volatile("memory": "swapgs" :::) -> asm;
}

//
// We can't read directly from tss.sp0: sp0 on x86_32 is special in
// and around vm86 mode and sp0 on x86_64 is special because of the
// entry trampoline.
//
extern "C" {
    pub fn this_cpu_read_const(_arg: const_cpu_current_top_of_stack) -> return;
}
extern "C" {
    pub fn this_cpu_read_stable(_arg: cpu_current_top_of_stack) -> return;
}

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn select_idle_routine();
}
extern "C" {
    pub fn amd_e400_c1e_apic_setup();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idle_boot_override {
    IDLE_POLL};

    extern void enable_sep_cpu(void);


// Defined in head.S
    extern struct desc_ptr		early_gdt_descr;

    extern void switch_gdt_and_percpu_base(int);
    extern void load_direct_gdt(int);
    extern void load_fixmap_gdt(int);
    extern void cpu_init(void);
    extern void cpu_init_exception_handling(bool boot_cpu);
    extern void cpu_init_replace_early_idt(void);
    extern void cr4_init(void);

    extern void set_task_blockstep(struct task_struct *task, bool on);

// Boot loader type from the setup header:
    extern int			bootloader_type;
    extern int			bootloader_version;

    extern char			ignore_fpu_irq;

pub const HAVE_ARCH_PICK_MMAP_LAYOUT: c_int = 1;
// Macro flag: #define ARCH_HAS_PREFETCHW

//
// Prefetch instructions for Pentium III (+) and AMD Athlon (+)
//
// It's not worth to care about 3dnow prefetches for the K6
// because they are microcoded there and very slow.
//
    static inline void prefetch(const void *x)
    {
    alternative_input(BASE_PREFETCH, "prefetchnta %1",
    X86_FEATURE_XMM,
    "m" (*(const char *)x));
    }

//
// 3dnow prefetch to get an exclusive cache line.
// Useful for spinlocks to avoid one state transition in the
// cache coherency protocol:
//
    static __always_inline void prefetchw(const void *x)
    {
    alternative_input(BASE_PREFETCH, "prefetchw %1",
    X86_FEATURE_3DNOWPREFETCH,
    "m" (*(const char *)x));
    }

    TOP_OF_KERNEL_STACK_PADDING)

    ({									\
    unsigned long __ptr = (unsigned long)task_stack_page(task);	\
    __ptr += THREAD_SIZE - TOP_OF_KERNEL_STACK_PADDING;		\
    ((struct pt_regs *)__ptr) - 1;					\
    })

    .sp0			= TOP_OF_INIT_STACK,			  \
    .sysenter_cs		= __KERNEL_CS,				  \
    }

    extern unsigned long __top_init_kernel_stack[];

    .sp	= (unsigned long)&__top_init_kernel_stack,		\
    }

    extern void start_thread(struct pt_regs *regs, unsigned long new_ip,
    unsigned long new_sp);

//
// This decides where the kernel will search for a free chunk of vm
// space during mmap's.
//

// Get/set a process' ability to use the timestamp counter instruction

    extern int get_tsc_mode(unsigned long adr);
    extern int set_tsc_mode(unsigned int val);

    DECLARE_PER_CPU(u64, msr_misc_features_shadow);

    static inline u32 per_cpu_llc_id(unsigned int cpu)
    {
    return per_cpu(cpu_info.topo.llc_id, cpu);
    }

    static inline u32 per_cpu_l2c_id(unsigned int cpu)
    {
    return per_cpu(cpu_info.topo.l2c_id, cpu);
    }

    static inline u32 per_cpu_core_id(unsigned int cpu)
    {
    return per_cpu(cpu_info.topo.core_id, cpu);
    }

//
// Issue a DIV 0/1 insn to clear any division data from previous DIV
// operations.
//
    static __always_inline void amd_clear_divider(void)
    {
    asm volatile(ALTERNATIVE("", "div %2\n\t", X86_BUG_DIV0)
    :: "a" (0), "d" (0), "r" (1));
    }

    extern void amd_check_microcode(void);

    static inline void amd_clear_divider(void)		{ }
    static inline void amd_check_microcode(void)		{ }

    extern unsigned long arch_align_stack(unsigned long sp);
    void free_init_pages(const char *what, unsigned long begin, unsigned long end);
    extern void free_kernel_image_pages(const char *what, void *begin, void *end);

    void default_idle(void);

    bool xen_set_default_idle(void);

pub const xen_set_default_idle: c_int = 0;

    void __noreturn stop_this_cpu(void *dummy);
    extern bool x86_hypervisor_present;
    void microcode_check(struct cpuinfo_x86 *prev_info);
    void store_cpu_caps(struct cpuinfo_x86 *info);

    DECLARE_PER_CPU(bool, cache_state_incoherent);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l1tf_mitigations {
    L1TF_MITIGATION_OFF,
    L1TF_MITIGATION_AUTO,
    L1TF_MITIGATION_FLUSH_NOWARN,
    L1TF_MITIGATION_FLUSH,
    L1TF_MITIGATION_FLUSH_NOSMT,
    L1TF_MITIGATION_FULL,
    L1TF_MITIGATION_FULL_FORCE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mds_mitigations {
    MDS_MITIGATION_OFF,
    MDS_MITIGATION_AUTO,
    MDS_MITIGATION_FULL,
    MDS_MITIGATION_VMWERV,
}

extern "C" {
    pub fn gds_ucode_mitigated() -> bool;
}
//
// Make previous memory operations globally visible before
// a WRMSR.
//
// MFENCE makes writes visible, but only affects load/store
// instructions.  WRMSR is unfortunately not a load/store
// instruction and is unaffected by MFENCE.  The LFENCE ensures
// that the WRMSR is not reordered.
//
// Most WRMSRs are full serializing instructions themselves and
// do not require this barrier.  This is only required for the
// IA32_TSC_DEADLINE and X2APIC MSRs.
//
