//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/traps_misaligned.c
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
//

    extern void put_f32_reg(unsigned long fp_reg, unsigned long value);
    static int set_f32_rd(unsigned long insn, struct pt_regs *regs,
    unsigned long val)
    {
    let mut fp_reg: c_ulong = FP_GET_RD(insn);
    put_f32_reg(fp_reg, val);
    regs.status |= SR_FS_DIRTY;
    return 0;
    }
    extern void put_f64_reg(unsigned long fp_reg, unsigned long value);
#[no_mangle]
unsafe extern "C" fn set_f64_rd(insn: c_ulong, regs: *mut pt_regs, val: u64) -> c_int {
    static int set_f64_rd(unsigned long insn, struct pt_regs *regs, u64 val)
    {
    let mut fp_reg: c_ulong = FP_GET_RD(insn);
    unsigned long value;

    value = (unsigned long) &val;

    value = val;

    put_f64_reg(fp_reg, value);
    regs.status |= SR_FS_DIRTY;
    return 0;
    }

    extern void get_f64_reg(unsigned long fp_reg, u64 *value);
    static u64 get_f64_rs(unsigned long insn, u8 fp_reg_offset,
    struct pt_regs *regs)
    {
    let mut fp_reg: c_ulong = (insn >> fp_reg_offset) & 0x1F;
    u64 val;
    get_f64_reg(fp_reg, &val);
    regs.status |= SR_FS_DIRTY;
    return val;
    }

    extern unsigned long get_f64_reg(unsigned long fp_reg);
    static unsigned long get_f64_rs(unsigned long insn, u8 fp_reg_offset,
    struct pt_regs *regs)
    {
    let mut fp_reg: c_ulong = (insn >> fp_reg_offset) & 0x1F;
    unsigned long val;
    val = get_f64_reg(fp_reg);
    regs.status |= SR_FS_DIRTY;
    return val;
    }

    extern unsigned long get_f32_reg(unsigned long fp_reg);
    static unsigned long get_f32_rs(unsigned long insn, u8 fp_reg_offset,
    struct pt_regs *regs)
    {
    let mut fp_reg: c_ulong = (insn >> fp_reg_offset) & 0x1F;
    unsigned long val;
    val = get_f32_reg(fp_reg);
    regs.status |= SR_FS_DIRTY;
    return val;
    }

    static void set_f32_rd(unsigned long insn, struct pt_regs *regs,
    unsigned long val) {}
    static void set_f64_rd(unsigned long insn, struct pt_regs *regs, u64 val) {}
    static unsigned long get_f64_rs(unsigned long insn, u8 fp_reg_offset,
    struct pt_regs *regs)
    {
    return 0;
    }
    static unsigned long get_f32_rs(unsigned long insn, u8 fp_reg_offset,
    struct pt_regs *regs)
    {
    return 0;
    }

    ({							\
    int __ret;					\
    \
    if (user_mode(regs)) {				\
    __ret = get_user(insn, (type __user *) insn_addr); \
    } else {					\
    insn = *(type *)insn_addr;		\
    __ret = 0;				\
    }						\
    \
    __ret;						\
    })
#[no_mangle]
pub unsafe extern "C" fn get_insn(regs: *mut pt_regs, epc: c_ulong, r_insn: *mut c_ulong) -> c_int {
    static inline int get_insn(struct pt_regs *regs, ulong epc, ulong *r_insn)
    {
    let mut insn: c_ulong = 0;
    if (epc & 0x2) {
    let mut tmp: c_ulong = 0;
    if (__read_insn(regs, insn, epc, u16))
    return -EFAULT;
// __get_user() uses regular "lw" which sign extend the loaded
// value make sure to clear higher order bits in case we "or" it
// below with the upper 16 bits half.
//
    insn &= GENMASK(15, 0);
    if ((insn & __INSN_LENGTH_MASK) != __INSN_LENGTH_32) {
// r_insn = insn;
    return 0;
    }
    epc += sizeof(u16);
    if (__read_insn(regs, tmp, epc, u16))
    return -EFAULT;
// r_insn = (tmp << 16) | insn;
    return 0;
    } else {
    if (__read_insn(regs, insn, epc, u32))
    return -EFAULT;
    if ((insn & __INSN_LENGTH_MASK) == __INSN_LENGTH_32) {
// r_insn = insn;
    return 0;
    }
    insn &= GENMASK(15, 0);
// r_insn = insn;
    return 0;
    }
    }
    union reg_data {
    u8 data_bytes[8];
    ulong data_ulong;
    u64 data_u64;
    };
// sysctl hooks
    int unaligned_enabled __read_mostly = 1;	/* Enabled by default */

#[no_mangle]
unsafe extern "C" fn handle_vector_misaligned_load(regs: *mut pt_regs) -> c_int {
    static int handle_vector_misaligned_load(struct pt_regs *regs)
    {
    let mut epc: c_ulong = regs.epc;
    unsigned long insn;
    if (get_insn(regs, epc, &insn))
    return -1;
// Only return 0 when in check_vector_unaligned_access_emulated
    if (*this_cpu_ptr(&vector_misaligned_access) == RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN) {
// this_cpu_ptr(&vector_misaligned_access) = RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED;
    regs.epc = epc + INSN_LEN(insn);
    return 0;
    }
// If vector instruction we don't emulate it yet
    regs.epc = epc;
    return -1;
    }

#[no_mangle]
unsafe extern "C" fn handle_vector_misaligned_load(regs: *mut pt_regs) -> c_int {
    static int handle_vector_misaligned_load(struct pt_regs *regs)
    {
    return -1;
    }

#[no_mangle]
unsafe extern "C" fn handle_scalar_misaligned_load(regs: *mut pt_regs) -> c_int {
    static int handle_scalar_misaligned_load(struct pt_regs *regs)
    {
    union reg_data val;
    let mut epc: c_ulong = regs.epc;
    unsigned long insn;
    let mut addr: c_ulong = regs.badaddr;
    let mut fp: c_int = 0, shift = 0, len = 0;
    perf_sw_event(PERF_COUNT_SW_ALIGNMENT_FAULTS, 1, regs, addr);
// this_cpu_ptr(&misaligned_access_speed) = RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED;
    if (!unaligned_enabled)
    return -1;
    if (user_mode(regs) && (current.thread.align_ctl & PR_UNALIGN_SIGBUS))
    return -1;
    if (get_insn(regs, epc, &insn))
    return -1;
    regs.epc = 0;
    if ((insn & INSN_MASK_LW) == INSN_MATCH_LW) {
    len = 4;
    shift = 8 * (sizeof(unsigned long) - len);

    } else if ((insn & INSN_MASK_LD) == INSN_MATCH_LD) {
    len = 8;
    shift = 8 * (sizeof(unsigned long) - len);
    } else if ((insn & INSN_MASK_LWU) == INSN_MATCH_LWU) {
    len = 4;

    } else if ((insn & INSN_MASK_FLD) == INSN_MATCH_FLD) {
    fp = 1;
    len = 8;
    } else if ((insn & INSN_MASK_FLW) == INSN_MATCH_FLW) {
    fp = 1;
    len = 4;
    } else if ((insn & INSN_MASK_LH) == INSN_MATCH_LH) {
    len = 2;
    shift = 8 * (sizeof(unsigned long) - len);
    } else if ((insn & INSN_MASK_LHU) == INSN_MATCH_LHU) {
    len = 2;

    } else if ((insn & INSN_MASK_C_LD) == INSN_MATCH_C_LD) {
    len = 8;
    shift = 8 * (sizeof(unsigned long) - len);
    insn = RVC_RS2S(insn) << SH_RD;
    } else if ((insn & INSN_MASK_C_LDSP) == INSN_MATCH_C_LDSP &&
    ((insn >> SH_RD) & 0x1f)) {
    len = 8;
    shift = 8 * (sizeof(unsigned long) - len);

    } else if ((insn & INSN_MASK_C_LW) == INSN_MATCH_C_LW) {
    len = 4;
    shift = 8 * (sizeof(unsigned long) - len);
    insn = RVC_RS2S(insn) << SH_RD;
    } else if ((insn & INSN_MASK_C_LWSP) == INSN_MATCH_C_LWSP &&
    ((insn >> SH_RD) & 0x1f)) {
    len = 4;
    shift = 8 * (sizeof(unsigned long) - len);
    } else if ((insn & INSN_MASK_C_FLD) == INSN_MATCH_C_FLD) {
    fp = 1;
    len = 8;
    insn = RVC_RS2S(insn) << SH_RD;
    } else if ((insn & INSN_MASK_C_FLDSP) == INSN_MATCH_C_FLDSP) {
    fp = 1;
    len = 8;

    } else if ((insn & INSN_MASK_C_FLW) == INSN_MATCH_C_FLW) {
    fp = 1;
    len = 4;
    insn = RVC_RS2S(insn) << SH_RD;
    } else if ((insn & INSN_MASK_C_FLWSP) == INSN_MATCH_C_FLWSP) {
    fp = 1;
    len = 4;

    } else if ((insn & INSN_MASK_C_LHU) == INSN_MATCH_C_LHU) {
    len = 2;
    insn = RVC_RS2S(insn) << SH_RD;
    } else if ((insn & INSN_MASK_C_LH) == INSN_MATCH_C_LH) {
    len = 2;
    shift = 8 * (sizeof(ulong) - len);
    insn = RVC_RS2S(insn) << SH_RD;
    } else {
    regs.epc = epc;
    return -1;
    }
    if (!IS_ENABLED(CONFIG_FPU) && fp)
    return -EOPNOTSUPP;
    val.data_u64 = 0;
    if (user_mode(regs)) {
    if (copy_from_user(&val, (u8 __user *)addr, len))
    return -1;
    } else {
    memcpy(&val, (u8 *)addr, len);
    }
    if (!fp)
    SET_RD(insn, regs, (long)(val.data_ulong << shift) >> shift);
#[no_mangle]
pub unsafe extern "C" fn if(8: len ==) -> else {
    else if (len == 8)
    set_f64_rd(insn, regs, val.data_u64);
    else
    set_f32_rd(insn, regs, val.data_ulong);
    regs.epc = epc + INSN_LEN(insn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn handle_scalar_misaligned_store(regs: *mut pt_regs) -> c_int {
    static int handle_scalar_misaligned_store(struct pt_regs *regs)
    {
    union reg_data val;
    let mut epc: c_ulong = regs.epc;
    unsigned long insn;
    let mut addr: c_ulong = regs.badaddr;
    let mut len: c_int = 0, fp = 0;
    perf_sw_event(PERF_COUNT_SW_ALIGNMENT_FAULTS, 1, regs, addr);
    if (!unaligned_enabled)
    return -1;
    if (user_mode(regs) && (current.thread.align_ctl & PR_UNALIGN_SIGBUS))
    return -1;
    if (get_insn(regs, epc, &insn))
    return -1;
    regs.epc = 0;
    val.data_ulong = GET_RS2(insn, regs);
    if ((insn & INSN_MASK_SW) == INSN_MATCH_SW) {
    len = 4;

    } else if ((insn & INSN_MASK_SD) == INSN_MATCH_SD) {
    len = 8;

    } else if ((insn & INSN_MASK_FSD) == INSN_MATCH_FSD) {
    fp = 1;
    len = 8;
    val.data_u64 = GET_F64_RS2(insn, regs);
    } else if ((insn & INSN_MASK_FSW) == INSN_MATCH_FSW) {
    fp = 1;
    len = 4;
    val.data_ulong = GET_F32_RS2(insn, regs);
    } else if ((insn & INSN_MASK_SH) == INSN_MATCH_SH) {
    len = 2;

    } else if ((insn & INSN_MASK_C_SD) == INSN_MATCH_C_SD) {
    len = 8;
    val.data_ulong = GET_RS2S(insn, regs);
    } else if ((insn & INSN_MASK_C_SDSP) == INSN_MATCH_C_SDSP) {
    len = 8;
    val.data_ulong = GET_RS2C(insn, regs);

    } else if ((insn & INSN_MASK_C_SW) == INSN_MATCH_C_SW) {
    len = 4;
    val.data_ulong = GET_RS2S(insn, regs);
    } else if ((insn & INSN_MASK_C_SWSP) == INSN_MATCH_C_SWSP) {
    len = 4;
    val.data_ulong = GET_RS2C(insn, regs);
    } else if ((insn & INSN_MASK_C_FSD) == INSN_MATCH_C_FSD) {
    fp = 1;
    len = 8;
    val.data_u64 = GET_F64_RS2S(insn, regs);
    } else if ((insn & INSN_MASK_C_FSDSP) == INSN_MATCH_C_FSDSP) {
    fp = 1;
    len = 8;
    val.data_u64 = GET_F64_RS2C(insn, regs);

    } else if ((insn & INSN_MASK_C_FSW) == INSN_MATCH_C_FSW) {
    fp = 1;
    len = 4;
    val.data_ulong = GET_F32_RS2S(insn, regs);
    } else if ((insn & INSN_MASK_C_FSWSP) == INSN_MATCH_C_FSWSP) {
    fp = 1;
    len = 4;
    val.data_ulong = GET_F32_RS2C(insn, regs);

    } else if ((insn & INSN_MASK_C_SH) == INSN_MATCH_C_SH) {
    len = 2;
    val.data_ulong = GET_RS2S(insn, regs);
    } else {
    regs.epc = epc;
    return -1;
    }
    if (!IS_ENABLED(CONFIG_FPU) && fp)
    return -EOPNOTSUPP;
    if (user_mode(regs)) {
    if (copy_to_user((u8 __user *)addr, &val, len))
    return -1;
    } else {
    memcpy((u8 *)addr, &val, len);
    }
    regs.epc = epc + INSN_LEN(insn);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_misaligned_load(regs: *mut pt_regs) -> c_int {
    int handle_misaligned_load(struct pt_regs *regs)
    {
    let mut epc: c_ulong = regs.epc;
    unsigned long insn;
    if (IS_ENABLED(CONFIG_RISCV_VECTOR_MISALIGNED)) {
    if (get_insn(regs, epc, &insn))
    return -1;
    if (insn_is_vector(insn))
    return handle_vector_misaligned_load(regs);
    }
    if (IS_ENABLED(CONFIG_RISCV_SCALAR_MISALIGNED))
    return handle_scalar_misaligned_load(regs);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_misaligned_store(regs: *mut pt_regs) -> c_int {
    int handle_misaligned_store(struct pt_regs *regs)
    {
    if (IS_ENABLED(CONFIG_RISCV_SCALAR_MISALIGNED))
    return handle_scalar_misaligned_store(regs);
    return -1;
    }

#[no_mangle]
pub unsafe extern "C" fn check_vector_unaligned_access_emulated(__always_unused: *mut *mut work_work) {
    void check_vector_unaligned_access_emulated(struct work_struct *work __always_unused)
    {
    long *mas_ptr = this_cpu_ptr(&vector_misaligned_access);
    unsigned long tmp_var;
// mas_ptr = RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN;
    kernel_vector_begin();
//
// In pre-13.0.0 versions of GCC, vector registers cannot appear in
// the clobber list. This inline asm clobbers v0, but since we do not
// currently build the kernel with V enabled, the v0 clobber arg is not
// needed (as the compiler will not emit vector code itself). If the kernel
// is changed to build with V enabled, the clobber arg will need to be
// added here.
//
    __asm__ __volatile__ (
    ".balign 4\n\t"
    ".option push\n\t"
    ".option arch, +zve32x\n\t"
    "       vsetivli zero, 1, e16, m1, ta, ma\n\t"	// Vectors of 16b
    "       vle16.v v0, (%[ptr])\n\t"		// Load bytes
    ".option pop\n\t"
    : : [ptr] "r" ((u8 *)&tmp_var + 1));
    kernel_vector_end();
    }
#[no_mangle]
pub unsafe extern "C" fn check_vector_unaligned_access_emulated_all_cpus() -> bool __init {
    bool __init check_vector_unaligned_access_emulated_all_cpus(void)
    {
    int cpu;
//
// While being documented as very slow, schedule_on_each_cpu() is used since
// kernel_vector_begin() expects irqs to be enabled or it will panic()
//
    schedule_on_each_cpu(check_vector_unaligned_access_emulated);
    for_each_online_cpu(cpu)
    if (per_cpu(vector_misaligned_access, cpu)
    == RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN)
    return false;
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn check_vector_unaligned_access_emulated_all_cpus() -> bool __init {
    bool __init check_vector_unaligned_access_emulated_all_cpus(void)
    {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn all_cpus_unaligned_scalar_access_emulated() -> bool {
    static bool all_cpus_unaligned_scalar_access_emulated(void)
    {
    int cpu;
    for_each_online_cpu(cpu)
    if (per_cpu(misaligned_access_speed, cpu) !=
    RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED)
    return false;
    return true;
    }

    static bool unaligned_ctl __read_mostly;
#[no_mangle]
unsafe extern "C" fn check_unaligned_access_emulated(__always_unused: *mut *mut void arg) {
    static void check_unaligned_access_emulated(void *arg __always_unused)
    {
    let mut cpu: c_int = smp_processor_id();
    unsigned long tmp_var, tmp_val;
    if (per_cpu(misaligned_access_speed, cpu) != RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN)
    return;
    __asm__ __volatile__ (
    "       "REG_L" %[tmp], 1(%[ptr])\n"
    : [tmp] "=r" (tmp_val) : [ptr] "r" (&tmp_var) : "memory");
    }
#[no_mangle]
unsafe extern "C" fn cpu_online_check_unaligned_access_emulated(cpu: c_uint) -> c_int {
    static int cpu_online_check_unaligned_access_emulated(unsigned int cpu)
    {
    long *mas_ptr = per_cpu_ptr(&misaligned_access_speed, cpu);
    check_unaligned_access_emulated(core::ptr::null_mut());
//
// If unaligned_ctl is already set, this means that we detected that all
// CPUS uses emulated misaligned access at boot time. If that changed
// when hotplugging the new cpu, this is something we don't handle.
//
    if (unlikely(unaligned_ctl && (*mas_ptr != RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED))) {
    pr_crit("CPU misaligned accesses non homogeneous (expected all emulated)\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn check_unaligned_access_emulated_all_cpus() -> bool __init {
    bool __init check_unaligned_access_emulated_all_cpus(void)
    {
//
// We can only support PR_UNALIGN controls if all CPUs have misaligned
// accesses emulated since tasks requesting such control can run on any
// CPU.
//
    on_each_cpu(check_unaligned_access_emulated, core::ptr::null_mut(), 1);
    if (!all_cpus_unaligned_scalar_access_emulated())
    return false;
    unaligned_ctl = true;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn unaligned_ctl_available() -> bool {
    bool unaligned_ctl_available(void)
    {
    return unaligned_ctl;
    }

#[no_mangle]
pub unsafe extern "C" fn check_unaligned_access_emulated_all_cpus() -> bool __init {
    bool __init check_unaligned_access_emulated_all_cpus(void)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cpu_online_check_unaligned_access_emulated(cpu: c_uint) -> c_int {
    static int cpu_online_check_unaligned_access_emulated(unsigned int cpu)
    {
    return 0;
    }

    static bool misaligned_traps_delegated;

#[no_mangle]
unsafe extern "C" fn cpu_online_sbi_unaligned_setup(cpu: c_uint) -> c_int {
    static int cpu_online_sbi_unaligned_setup(unsigned int cpu)
    {
    if (sbi_fwft_set(SBI_FWFT_MISALIGNED_EXC_DELEG, 1, 0) &&
    misaligned_traps_delegated) {
    pr_crit("Misaligned trap delegation non homogeneous (expected delegated)");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unaligned_access_init() -> void __init {
    void __init unaligned_access_init(void)
    {
    int ret;
    ret = sbi_fwft_set_online_cpus(SBI_FWFT_MISALIGNED_EXC_DELEG, 1, 0);
    if (ret)
    return;
    misaligned_traps_delegated = true;
    pr_info("SBI misaligned access exception delegation ok\n");
//
// Note that we don't have to take any specific action here, if
// the delegation is successful, then
// check_unaligned_access_emulated() will verify that indeed the
// platform traps on misaligned accesses.
//
    }

    void __init unaligned_access_init(void) {}
#[no_mangle]
unsafe extern "C" fn cpu_online_sbi_unaligned_setup(__always_unused: unsigned int cpu) -> c_int {
    static int cpu_online_sbi_unaligned_setup(unsigned int cpu __always_unused)
    {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_online_unaligned_access_init(cpu: c_uint) -> c_int {
    int cpu_online_unaligned_access_init(unsigned int cpu)
    {
    int ret;
    ret = cpu_online_sbi_unaligned_setup(cpu);
    if (ret)
    return ret;
    return cpu_online_check_unaligned_access_emulated(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn misaligned_traps_can_delegate() -> bool {
    bool misaligned_traps_can_delegate(void)
    {
//
// Either we successfully requested misaligned traps delegation for all
// CPUs, or the SBI does not implement the FWFT extension but delegated
// the exception by default.
//
    return misaligned_traps_delegated ||
    all_cpus_unaligned_scalar_access_emulated();
    }
    EXPORT_SYMBOL_GPL(misaligned_traps_can_delegate);
