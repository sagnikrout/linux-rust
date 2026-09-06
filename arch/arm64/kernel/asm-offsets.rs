//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/asm-offsets.c
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
// Based on arch/arm/kernel/asm-offsets.c
//
// Copyright (C) 1995-2003 Russell King
// 2001-2002 Keith Owens
// Copyright (C) 2012 ARM Ltd.
//
// Macro flag: #define COMPILE_OFFSETS

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    DEFINE(TSK_TI_CPU,		offsetof(struct task_struct, thread_info.cpu));
    DEFINE(TSK_TI_FLAGS,		offsetof(struct task_struct, thread_info.flags));
    DEFINE(TSK_TI_PREEMPT,	offsetof(struct task_struct, thread_info.preempt_count));

    DEFINE(TSK_TI_TTBR0,		offsetof(struct task_struct, thread_info.ttbr0));

    DEFINE(TSK_TI_SCS_BASE,	offsetof(struct task_struct, thread_info.scs_base));
    DEFINE(TSK_TI_SCS_SP,		offsetof(struct task_struct, thread_info.scs_sp));

    DEFINE(TSK_STACK,		offsetof(struct task_struct, stack));

    DEFINE(TSK_STACK_CANARY,	offsetof(struct task_struct, stack_canary));

    BLANK();
    DEFINE(THREAD_CPU_CONTEXT,	offsetof(struct task_struct, thread.cpu_context));
    DEFINE(THREAD_SCTLR_USER,	offsetof(struct task_struct, thread.sctlr_user));

    DEFINE(THREAD_KEYS_USER,	offsetof(struct task_struct, thread.keys_user));

    DEFINE(THREAD_KEYS_KERNEL,	offsetof(struct task_struct, thread.keys_kernel));

    DEFINE(THREAD_MTE_CTRL,	offsetof(struct task_struct, thread.mte_ctrl));

    BLANK();
    DEFINE(S_X0,			offsetof(struct pt_regs, regs[0]));
    DEFINE(S_X2,			offsetof(struct pt_regs, regs[2]));
    DEFINE(S_X4,			offsetof(struct pt_regs, regs[4]));
    DEFINE(S_X6,			offsetof(struct pt_regs, regs[6]));
    DEFINE(S_X8,			offsetof(struct pt_regs, regs[8]));
    DEFINE(S_X10,			offsetof(struct pt_regs, regs[10]));
    DEFINE(S_X12,			offsetof(struct pt_regs, regs[12]));
    DEFINE(S_X14,			offsetof(struct pt_regs, regs[14]));
    DEFINE(S_X16,			offsetof(struct pt_regs, regs[16]));
    DEFINE(S_X18,			offsetof(struct pt_regs, regs[18]));
    DEFINE(S_X20,			offsetof(struct pt_regs, regs[20]));
    DEFINE(S_X22,			offsetof(struct pt_regs, regs[22]));
    DEFINE(S_X24,			offsetof(struct pt_regs, regs[24]));
    DEFINE(S_X26,			offsetof(struct pt_regs, regs[26]));
    DEFINE(S_X28,			offsetof(struct pt_regs, regs[28]));
    DEFINE(S_FP,			offsetof(struct pt_regs, regs[29]));
    DEFINE(S_LR,			offsetof(struct pt_regs, regs[30]));
    DEFINE(S_SP,			offsetof(struct pt_regs, sp));
    DEFINE(S_PC,			offsetof(struct pt_regs, pc));
    DEFINE(S_PSTATE,		offsetof(struct pt_regs, pstate));
    DEFINE(S_SYSCALLNO,		offsetof(struct pt_regs, syscallno));
    DEFINE(S_SDEI_TTBR1,		offsetof(struct pt_regs, sdei_ttbr1));
    DEFINE(S_PMR,			offsetof(struct pt_regs, pmr));
    DEFINE(S_STACKFRAME,		offsetof(struct pt_regs, stackframe));
    DEFINE(S_STACKFRAME_TYPE,	offsetof(struct pt_regs, stackframe.type));
    DEFINE(PT_REGS_SIZE,		sizeof(struct pt_regs));
    BLANK();

    DEFINE(FREGS_X0,		offsetof(struct __arch_ftrace_regs, regs[0]));
    DEFINE(FREGS_X2,		offsetof(struct __arch_ftrace_regs, regs[2]));
    DEFINE(FREGS_X4,		offsetof(struct __arch_ftrace_regs, regs[4]));
    DEFINE(FREGS_X6,		offsetof(struct __arch_ftrace_regs, regs[6]));
    DEFINE(FREGS_X8,		offsetof(struct __arch_ftrace_regs, regs[8]));
    DEFINE(FREGS_FP,		offsetof(struct __arch_ftrace_regs, fp));
    DEFINE(FREGS_LR,		offsetof(struct __arch_ftrace_regs, lr));
    DEFINE(FREGS_SP,		offsetof(struct __arch_ftrace_regs, sp));
    DEFINE(FREGS_PC,		offsetof(struct __arch_ftrace_regs, pc));

    DEFINE(FREGS_DIRECT_TRAMP,	offsetof(struct __arch_ftrace_regs, direct_tramp));

    DEFINE(FREGS_SIZE,		sizeof(struct __arch_ftrace_regs));
    BLANK();

    DEFINE(CPU_BOOT_TASK,		offsetof(struct secondary_data, task));
    BLANK();
    DEFINE(FTR_OVR_VAL_OFFSET,	offsetof(struct arm64_ftr_override, val));
    DEFINE(FTR_OVR_MASK_OFFSET,	offsetof(struct arm64_ftr_override, mask));
    BLANK();

    DEFINE(VCPU_CONTEXT,		offsetof(struct kvm_vcpu, arch.ctxt));
    DEFINE(VCPU_FAULT_DISR,	offsetof(struct kvm_vcpu, arch.fault.disr_el1));
    DEFINE(VCPU_HCR_EL2,		offsetof(struct kvm_vcpu, arch.hcr_el2));
    DEFINE(CPU_USER_PT_REGS,	offsetof(struct kvm_cpu_context, regs));
    DEFINE(CPU_ELR_EL2,		offsetof(struct kvm_cpu_context, sys_regs[ELR_EL2]));
    DEFINE(CPU_RGSR_EL1,		offsetof(struct kvm_cpu_context, sys_regs[RGSR_EL1]));
    DEFINE(CPU_GCR_EL1,		offsetof(struct kvm_cpu_context, sys_regs[GCR_EL1]));
    DEFINE(CPU_APIAKEYLO_EL1,	offsetof(struct kvm_cpu_context, sys_regs[APIAKEYLO_EL1]));
    DEFINE(CPU_APIBKEYLO_EL1,	offsetof(struct kvm_cpu_context, sys_regs[APIBKEYLO_EL1]));
    DEFINE(CPU_APDAKEYLO_EL1,	offsetof(struct kvm_cpu_context, sys_regs[APDAKEYLO_EL1]));
    DEFINE(CPU_APDBKEYLO_EL1,	offsetof(struct kvm_cpu_context, sys_regs[APDBKEYLO_EL1]));
    DEFINE(CPU_APGAKEYLO_EL1,	offsetof(struct kvm_cpu_context, sys_regs[APGAKEYLO_EL1]));
    DEFINE(HOST_CONTEXT_VCPU,	offsetof(struct kvm_cpu_context, __hyp_running_vcpu));
    DEFINE(HOST_DATA_CONTEXT,	offsetof(struct kvm_host_data, host_ctxt));
    DEFINE(NVHE_INIT_MAIR_EL2,	offsetof(struct kvm_nvhe_init_params, mair_el2));
    DEFINE(NVHE_INIT_TCR_EL2,	offsetof(struct kvm_nvhe_init_params, tcr_el2));
    DEFINE(NVHE_INIT_TPIDR_EL2,	offsetof(struct kvm_nvhe_init_params, tpidr_el2));
    DEFINE(NVHE_INIT_STACK_HYP_VA,	offsetof(struct kvm_nvhe_init_params, stack_hyp_va));
    DEFINE(NVHE_INIT_PGD_PA,	offsetof(struct kvm_nvhe_init_params, pgd_pa));
    DEFINE(NVHE_INIT_HCR_EL2,	offsetof(struct kvm_nvhe_init_params, hcr_el2));
    DEFINE(NVHE_INIT_VTTBR,	offsetof(struct kvm_nvhe_init_params, vttbr));
    DEFINE(NVHE_INIT_VTCR,	offsetof(struct kvm_nvhe_init_params, vtcr));

    DEFINE(CPU_CTX_SP,		offsetof(struct cpu_suspend_ctx, sp));
    DEFINE(MPIDR_HASH_MASK,	offsetof(struct mpidr_hash, mask));
    DEFINE(MPIDR_HASH_SHIFTS,	offsetof(struct mpidr_hash, shift_aff));
    DEFINE(SLEEP_STACK_DATA_SYSTEM_REGS,	offsetof(struct sleep_stack_data, system_regs));
    DEFINE(SLEEP_STACK_DATA_CALLEE_REGS,	offsetof(struct sleep_stack_data, callee_saved_regs));

    DEFINE(ARM_SMCCC_RES_X0_OFFS,		offsetof(struct arm_smccc_res, a0));
    DEFINE(ARM_SMCCC_RES_X2_OFFS,		offsetof(struct arm_smccc_res, a2));
    DEFINE(ARM_SMCCC_QUIRK_ID_OFFS,	offsetof(struct arm_smccc_quirk, id));
    DEFINE(ARM_SMCCC_QUIRK_STATE_OFFS,	offsetof(struct arm_smccc_quirk, state));
    DEFINE(ARM_SMCCC_1_2_REGS_X0_OFFS,	offsetof(struct arm_smccc_1_2_regs, a0));
    DEFINE(ARM_SMCCC_1_2_REGS_X2_OFFS,	offsetof(struct arm_smccc_1_2_regs, a2));
    DEFINE(ARM_SMCCC_1_2_REGS_X4_OFFS,	offsetof(struct arm_smccc_1_2_regs, a4));
    DEFINE(ARM_SMCCC_1_2_REGS_X6_OFFS,	offsetof(struct arm_smccc_1_2_regs, a6));
    DEFINE(ARM_SMCCC_1_2_REGS_X8_OFFS,	offsetof(struct arm_smccc_1_2_regs, a8));
    DEFINE(ARM_SMCCC_1_2_REGS_X10_OFFS,	offsetof(struct arm_smccc_1_2_regs, a10));
    DEFINE(ARM_SMCCC_1_2_REGS_X12_OFFS,	offsetof(struct arm_smccc_1_2_regs, a12));
    DEFINE(ARM_SMCCC_1_2_REGS_X14_OFFS,	offsetof(struct arm_smccc_1_2_regs, a14));
    DEFINE(ARM_SMCCC_1_2_REGS_X16_OFFS,	offsetof(struct arm_smccc_1_2_regs, a16));
    BLANK();
    DEFINE(HIBERN_PBE_ORIG,	offsetof(struct pbe, orig_address));
    DEFINE(HIBERN_PBE_ADDR,	offsetof(struct pbe, address));
    DEFINE(HIBERN_PBE_NEXT,	offsetof(struct pbe, next));
    DEFINE(ARM64_FTR_SYSVAL,	offsetof(struct arm64_ftr_reg, sys_val));
    BLANK();

    DEFINE(TRAMP_VALIAS,		TRAMP_VALIAS);

    DEFINE(SDEI_EVENT_INTREGS,	offsetof(struct sdei_registered_event, interrupted_regs));
    DEFINE(SDEI_EVENT_PRIORITY,	offsetof(struct sdei_registered_event, priority));

    DEFINE(PTRAUTH_USER_KEY_APIA,		offsetof(struct ptrauth_keys_user, apia));

    DEFINE(PTRAUTH_KERNEL_KEY_APIA,	offsetof(struct ptrauth_keys_kernel, apia));

    BLANK();

    DEFINE(KIMAGE_ARCH_DTB_MEM,		offsetof(struct kimage, arch.dtb_mem));
    DEFINE(KIMAGE_ARCH_EL2_VECTORS,	offsetof(struct kimage, arch.el2_vectors));
    DEFINE(KIMAGE_ARCH_ZERO_PAGE,		offsetof(struct kimage, arch.zero_page));
    DEFINE(KIMAGE_ARCH_PHYS_OFFSET,	offsetof(struct kimage, arch.phys_offset));
    DEFINE(KIMAGE_ARCH_TTBR1,		offsetof(struct kimage, arch.ttbr1));
    DEFINE(KIMAGE_HEAD,			offsetof(struct kimage, head));
    DEFINE(KIMAGE_START,			offsetof(struct kimage, start));
    BLANK();

    DEFINE(FTRACE_OPS_FUNC,		offsetof(struct ftrace_ops, func));

    BLANK();

    DEFINE(FTRACE_OPS_DIRECT_CALL,	offsetof(struct ftrace_ops, direct_call));

    DEFINE(PIE_E0_ASM, PIE_E0);
    DEFINE(PIE_E1_ASM, PIE_E1);
    return 0;
    }
