//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/syscall_32.c
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
// 32-bit system call dispatch

//
// The sys_call_table[] is no longer used for system calls, but
// kernel/trace/trace_syscalls.c still wants to know the system
// call address.
//

    const sys_call_ptr_t sys_call_table[] = {

    };

// The unsigned int @nr argument is intentional as it creates denser code in a 64-bit build
#[no_mangle]
unsafe extern "C" fn ia32_sys_call(regs: *const pt_regs, nr: c_uint) -> noinline long {
    static noinline long ia32_sys_call(const struct pt_regs *regs, unsigned int nr)
    {
    switch (nr) {

    default: return __ia32_sys_ni_syscall(regs);
    }
    }
#[no_mangle]
unsafe extern "C" fn syscall_32_enter(regs: *mut pt_regs) -> __always_inline long {
    static __always_inline long syscall_32_enter(struct pt_regs *regs)
    {
    if (IS_ENABLED(CONFIG_IA32_EMULATION))
    current_thread_info().status |= TS_COMPAT;
    return (int)regs.orig_ax;
    }

    let mut __ro_after_init: bool __ia32_enabled = !IS_ENABLED(CONFIG_IA32_EMULATION_DEFAULT_DISABLED);
#[no_mangle]
unsafe extern "C" fn ia32_emulation_override_cmdline(arg: *mut c_char) -> int __init {
    static int __init ia32_emulation_override_cmdline(char *arg)
    {
    return kstrtobool(arg, &__ia32_enabled);
    }
    early_param("ia32_emulation", ia32_emulation_override_cmdline);

//
// Invoke a 32-bit syscall.  Called with IRQs on in CT_STATE_KERNEL.
//
#[no_mangle]
unsafe extern "C" fn do_syscall_32_irqs_on(regs: *mut pt_regs, nr: c_ulong) -> __always_inline void {
    static __always_inline void do_syscall_32_irqs_on(struct pt_regs *regs, unsigned long nr)
    {
    if (likely(nr < IA32_NR_syscalls)) {
    nr = array_index_nospec(nr, IA32_NR_syscalls);
    regs.ax = ia32_sys_call(regs, (unsigned int)nr);
    }
    }

#[no_mangle]
unsafe extern "C" fn int80_is_external() -> __always_inline bool {
    static __always_inline bool int80_is_external(void)
    {
    let mut offs: c_uint = (0x80 / 32) * 0x10;
    let mut bit: u32 = BIT(0x80 % 32);
// The local APIC on XENPV guests is fake
    if (cpu_feature_enabled(X86_FEATURE_XENPV))
    return false;
//
// If vector 0x80 is set in the APIC ISR then this is an external
// interrupt. Either from broken hardware or injected by a VMM.
//
// Note: In guest mode this is only valid for secure guests where
// the secure module fully controls the vAPIC exposed to the guest.
//
    return apic_read(APIC_ISR + offs) & bit;
    }
//
// do_int80_emulation - 32-bit legacy syscall C entry from asm
// @regs: syscall arguments in struct pt_args on the stack.
//
// This entry point can be used by 32-bit and 64-bit programs to perform
// 32-bit system calls.  Instances of INT $0x80 can be found inline in
// various programs and libraries.  It is also used by the vDSO's
// __kernel_vsyscall fallback for hardware that doesn't support a faster
// entry method.  Restarted 32-bit system calls also fall back to INT
// $0x80 regardless of what instruction was originally used to do the
// system call.
//
// This is considered a slow path.  It is not used by most libc
// implementations on modern hardware except during process startup.
//
// The arguments for the INT $0x80 based syscall are on stack in the
// pt_regs structure:
// eax:				system call number
// ebx, ecx, edx, esi, edi, ebp:	arg1 - arg 6
//
#[no_mangle]
pub unsafe extern "C" fn do_int80_emulation(regs: *mut pt_regs) -> __visible noinstr void {
    __visible noinstr void do_int80_emulation(struct pt_regs *regs)
    {
    long nr;
// Kernel does not use INT $0x80!
    if (unlikely(!user_mode(regs))) {
    irqentry_enter(regs);
    instrumentation_begin();
    panic("Unexpected external interrupt 0x80\n");
    }
//
// Establish kernel context for instrumentation, including for
// int80_is_external() below which calls into the APIC driver.
// Identical for soft and external interrupts.
//
    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();
// Validate that this is a soft interrupt to the extent possible
    if (unlikely(int80_is_external()))
    panic("Unexpected external interrupt 0x80\n");
//
// The low level idtentry code pushed -1 into regs::orig_ax
// and regs::ax contains the syscall number.
//
// User tracing code (ptrace or signal handlers) might assume
// that the regs::orig_ax contains a 32-bit number on invoking
// a 32-bit syscall.
//
// Establish the syscall convention by saving the 32bit truncated
// syscall number in regs::orig_ax and by invalidating regs::ax.
//
    regs.orig_ax = regs.ax & GENMASK(31, 0);
    regs.ax = -ENOSYS;
    nr = syscall_32_enter(regs);
    local_irq_enable();
    if (likely(syscall_enter_from_user_mode_work(regs, &nr)))
    do_syscall_32_irqs_on(regs, nr);
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
    }

//
// A FRED-specific INT80 handler is warranted for the follwing reasons:
//
// 1) As INT instructions and hardware interrupts are separate event
// types, FRED does not preclude the use of vector 0x80 for external
// interrupts. As a result, the FRED setup code does not reserve
// vector 0x80 and calling int80_is_external() is not merely
// suboptimal but actively incorrect: it could cause a system call
// to be incorrectly ignored.
//
// 2) It is called only for handling vector 0x80 of event type
// EVENT_TYPE_SWINT and will never be called to handle any external
// interrupt (event type EVENT_TYPE_EXTINT).
//
// 3) FRED has separate entry flows depending on if the event came from
// user space or kernel space, and because the kernel does not use
// INT insns, the FRED kernel entry handler fred_entry_from_kernel()
// falls through to fred_bad_type() if the event type is
// EVENT_TYPE_SWINT, i.e., INT insns. So if the kernel is handling
// an INT insn, it can only be from a user level.
//
// 4) int80_emulation() does a CLEAR_BRANCH_HISTORY. While FRED will
// likely take a different approach if it is ever needed: it
// probably belongs in either fred_intx()/ fred_other() or
// asm_fred_entrypoint_user(), depending on if this ought to be done
// for all entries from userspace or only system
// calls.
//
// 5) INT $0x80 is the fast path for 32-bit system calls under FRED.
//
    DEFINE_FREDENTRY_RAW(int80_emulation)
    {
    long nr;
    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();
//
// FRED pushed 0 into regs::orig_ax and regs::ax contains the
// syscall number.
//
// User tracing code (ptrace or signal handlers) might assume
// that the regs::orig_ax contains a 32-bit number on invoking
// a 32-bit syscall.
//
// Establish the syscall convention by saving the 32bit truncated
// syscall number in regs::orig_ax and by invalidating regs::ax.
//
    regs.orig_ax = regs.ax & GENMASK(31, 0);
    regs.ax = -ENOSYS;
    nr = syscall_32_enter(regs);
    local_irq_enable();
    if (likely(syscall_enter_from_user_mode_work(regs, &nr)))
    do_syscall_32_irqs_on(regs, nr);
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
    }

// Handles int $0x80 on a 32bit kernel
#[no_mangle]
pub unsafe extern "C" fn do_int80_syscall_32(regs: *mut pt_regs) -> __visible noinstr void {
    __visible noinstr void do_int80_syscall_32(struct pt_regs *regs)
    {
    let mut nr: c_long = syscall_32_enter(regs);
//
// Subtlety here: if ptrace pokes something larger than 2^31-1 into
// orig_ax, the int return value truncates it. This matches
// the semantics of syscall_get_nr().
//
    if (likely(syscall_enter_from_user_mode_randomize_stack(regs, &nr))) {
    instrumentation_begin();
    do_syscall_32_irqs_on(regs, nr);
    instrumentation_end();
    }
    syscall_exit_to_user_mode(regs);
    }

#[no_mangle]
unsafe extern "C" fn __do_fast_syscall_32(regs: *mut pt_regs) -> noinstr bool {
    static noinstr bool __do_fast_syscall_32(struct pt_regs *regs)
    {
    let mut nr: c_long = syscall_32_enter(regs);
    int res;
    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();
    local_irq_enable();
// Fetch EBP from where the vDSO stashed it.
    if (IS_ENABLED(CONFIG_X86_64)) {
//
// Micro-optimization: the pointer we're following is
// explicitly 32 bits, so it can't be out of range.
//
    res = __get_user(*(u32 *)&regs.bp,
    (u32 __user  *)(unsigned long)(u32)regs.sp);
    } else {
    res = get_user(*(u32 *)&regs.bp,
    (u32 __user  *)(unsigned long)(u32)regs.sp);
    }
    if (res) {
// User code screwed up.
    regs.ax = -EFAULT;
    local_irq_disable();
    instrumentation_end();
    irqentry_exit_to_user_mode(regs);
    return false;
    }
    if (likely(syscall_enter_from_user_mode_work(regs, &nr)))
    do_syscall_32_irqs_on(regs, nr);
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
    return true;
    }
// Returns true to return using SYSEXIT/SYSRETL, or false to use IRET
#[no_mangle]
pub unsafe extern "C" fn do_fast_syscall_32(regs: *mut pt_regs) -> __visible noinstr bool {
    __visible noinstr bool do_fast_syscall_32(struct pt_regs *regs)
    {
//
// Called using the internal vDSO SYSENTER/SYSCALL32 calling
// convention.  Adjust regs so it looks like we entered using int80.
//
    unsigned long landing_pad = (unsigned long)current.mm.context.vdso +
    vdso32_image.sym_int80_landing_pad;
//
// SYSENTER loses EIP, and even SYSCALL32 needs us to skip forward
// so that 'regs->ip -= 2' lands back on an int $0x80 instruction.
// Fix it up.
//
    regs.ip = landing_pad;
// Invoke the syscall. If it failed, keep it simple: use IRET.
    if (!__do_fast_syscall_32(regs))
    return false;
//
// Check that the register state is valid for using SYSRETL/SYSEXIT
// to exit to userspace.  Otherwise use the slower but fully capable
// IRET exit path.
//
// XEN PV guests always use the IRET path
    if (cpu_feature_enabled(X86_FEATURE_XENPV))
    return false;
// EIP must point to the VDSO landing pad
    if (unlikely(regs.ip != landing_pad))
    return false;
// CS and SS must match the values set in MSR_STAR
    if (unlikely(regs.cs != __USER32_CS || regs.ss != __USER_DS))
    return false;
// If the TF, RF, or VM flags are set, use IRET
    if (unlikely(regs.flags & (X86_EFLAGS_RF | X86_EFLAGS_TF | X86_EFLAGS_VM)))
    return false;
// Use SYSRETL/SYSEXIT to exit to userspace
    return true;
    }
// Returns true to return using SYSEXIT/SYSRETL, or false to use IRET
#[no_mangle]
pub unsafe extern "C" fn do_SYSENTER_32(regs: *mut pt_regs) -> __visible noinstr bool {
    __visible noinstr bool do_SYSENTER_32(struct pt_regs *regs)
    {
// SYSENTER loses RSP, but the vDSO saved it in RBP.
    regs.sp = regs.bp;
// SYSENTER clobbers EFLAGS.IF.  Assume it was set in usermode.
    regs.flags |= X86_EFLAGS_IF;
    return do_fast_syscall_32(regs);
    }
