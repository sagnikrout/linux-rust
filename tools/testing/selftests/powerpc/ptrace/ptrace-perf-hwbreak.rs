//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-perf-hwbreak.c
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


// SPDX-License-Identifier: GPL-2.0+

//
// Child subroutine that performs a load on the address, then traps
//
    void same_watch_addr_child(unsigned long *addr);
// Address of the ld instruction in same_watch_addr_child()
    extern char same_watch_addr_load[];
// Address of the end trap instruction in same_watch_addr_child()
    extern char same_watch_addr_trap[];
//
// Child subroutine that performs a load on the first address, then a load on
// the second address (with no instructions separating this from the first
// load), then traps.
//
    void perf_then_ptrace_child(unsigned long *first_addr, unsigned long *second_addr);
// Address of the first ld instruction in perf_then_ptrace_child()
    extern char perf_then_ptrace_load1[];
// Address of the second ld instruction in perf_then_ptrace_child()
    extern char perf_then_ptrace_load2[];
// Address of the end trap instruction in perf_then_ptrace_child()
    extern char perf_then_ptrace_trap[];
#[no_mangle]
pub unsafe extern "C" fn sys_ptrace(request: c_long, pid: pid_t, addr: c_ulong, data: c_ulong) -> c_long {
    static inline long sys_ptrace(long request, pid_t pid, unsigned long addr, unsigned long data)
    {
    return syscall(__NR_ptrace, request, pid, addr, data);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_traceme() -> c_long {
    static long ptrace_traceme(void)
    {
    return sys_ptrace(PTRACE_TRACEME, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_getregs(pid: pid_t, result: *mut pt_regs) -> c_long {
    static long ptrace_getregs(pid_t pid, struct pt_regs *result)
    {
    return sys_ptrace(PTRACE_GETREGS, pid, 0, (unsigned long)result);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_setregs(pid: pid_t, result: *mut pt_regs) -> c_long {
    static long ptrace_setregs(pid_t pid, struct pt_regs *result)
    {
    return sys_ptrace(PTRACE_SETREGS, pid, 0, (unsigned long)result);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_cont(pid: pid_t, signal: c_long) -> c_long {
    static long ptrace_cont(pid_t pid, long signal)
    {
    return sys_ptrace(PTRACE_CONT, pid, 0, signal);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_singlestep(pid: pid_t, signal: c_long) -> c_long {
    static long ptrace_singlestep(pid_t pid, long signal)
    {
    return sys_ptrace(PTRACE_SINGLESTEP, pid, 0, signal);
    }
#[no_mangle]
unsafe extern "C" fn ppc_ptrace_gethwdbginfo(pid: pid_t, dbginfo: *mut ppc_debug_info) -> c_long {
    static long ppc_ptrace_gethwdbginfo(pid_t pid, struct ppc_debug_info *dbginfo)
    {
    return sys_ptrace(PPC_PTRACE_GETHWDBGINFO, pid, 0, (unsigned long)dbginfo);
    }
#[no_mangle]
unsafe extern "C" fn ppc_ptrace_sethwdbg(pid: pid_t, bp_info: *mut ppc_hw_breakpoint) -> c_long {
    static long ppc_ptrace_sethwdbg(pid_t pid, struct ppc_hw_breakpoint *bp_info)
    {
    return sys_ptrace(PPC_PTRACE_SETHWDEBUG, pid, 0, (unsigned long)bp_info);
    }
#[no_mangle]
unsafe extern "C" fn ppc_ptrace_delhwdbg(pid: pid_t, bp_id: c_int) -> c_long {
    static long ppc_ptrace_delhwdbg(pid_t pid, int bp_id)
    {
    return sys_ptrace(PPC_PTRACE_DELHWDEBUG, pid, 0L, bp_id);
    }
#[no_mangle]
unsafe extern "C" fn ptrace_getreg_pc(pid: pid_t, pc: *mut c_void) -> c_long {
    static long ptrace_getreg_pc(pid_t pid, void **pc)
    {
    struct pt_regs regs;
    long err;
    err = ptrace_getregs(pid, &regs);
    if (err)
    return err;
// pc = (void *)regs.nip;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptrace_setreg_pc(pid: pid_t, pc: *mut c_void) -> c_long {
    static long ptrace_setreg_pc(pid_t pid, void *pc)
    {
    struct pt_regs regs;
    long err;
    err = ptrace_getregs(pid, &regs);
    if (err)
    return err;
    regs.nip = (unsigned long)pc;
    err = ptrace_setregs(pid, &regs);
    if (err)
    return err;
    return 0;
    }
    static int perf_event_open(struct perf_event_attr *attr, pid_t pid, int cpu,
    int group_fd, unsigned long flags)
    {
    return syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags);
    }
#[no_mangle]
unsafe extern "C" fn perf_user_event_attr_set(attr: *mut perf_event_attr, addr: *mut c_void, len: u64) {
    static void perf_user_event_attr_set(struct perf_event_attr *attr, void *addr, u64 len)
    {
    memset(attr, 0, sizeof(struct perf_event_attr));
    attr.type		= PERF_TYPE_BREAKPOINT;
    attr.size		= sizeof(struct perf_event_attr);
    attr.bp_type		= HW_BREAKPOINT_R;
    attr.bp_addr		= (u64)addr;
    attr.bp_len		= len;
    attr.exclude_kernel	= 1;
    attr.exclude_hv	= 1;
    }
#[no_mangle]
unsafe extern "C" fn perf_watchpoint_open(child_pid: pid_t, addr: *mut c_void, len: u64) -> c_int {
    static int perf_watchpoint_open(pid_t child_pid, void *addr, u64 len)
    {
    struct perf_event_attr attr;
    perf_user_event_attr_set(&attr, addr, len);
    return perf_event_open(&attr, child_pid, -1, -1, 0);
    }
#[no_mangle]
unsafe extern "C" fn perf_read_counter(perf_fd: c_int, count: *mut u64) -> c_int {
    static int perf_read_counter(int perf_fd, u64 *count)
    {
//
// A perf counter is retrieved by the read() syscall. It contains
// the current count as 8 bytes that are interpreted as a u64
//
    let mut len: isize = read(perf_fd, count, sizeof(*count));
    if (len != sizeof(*count))
    return -1;
    return 0;
    }
    static void ppc_ptrace_init_breakpoint(struct ppc_hw_breakpoint *info,
    int type, void *addr, int len)
    {
    info.version = 1;
    info.trigger_type = type;
    info.condition_mode = PPC_BREAKPOINT_CONDITION_NONE;
    info.addr = (u64)addr;
    info.addr2 = (u64)addr + len;
    info.condition_value = 0;
    if (!len)
    info.addr_mode = PPC_BREAKPOINT_MODE_EXACT;
    else
    info.addr_mode = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    }
//
// Checks if we can place at least 2 watchpoints on the child process
//
#[no_mangle]
unsafe extern "C" fn check_watchpoints(pid: pid_t) -> c_int {
    static int check_watchpoints(pid_t pid)
    {
    struct ppc_debug_info dbginfo;
    FAIL_IF_MSG(ppc_ptrace_gethwdbginfo(pid, &dbginfo), "PPC_PTRACE_GETHWDBGINFO failed");
    SKIP_IF_MSG(dbginfo.num_data_bps <= 1, "Not enough data watchpoints (need at least 2)");
    return 0;
    }
//
// Wrapper around a plain fork() call that sets up the child for
// ptrace-ing. Both the parent and child return from this, though
// the child is stopped until ptrace_cont(pid) is run by the parent.
//
#[no_mangle]
unsafe extern "C" fn ptrace_fork_child(pid: *mut pid_t) -> c_int {
    static int ptrace_fork_child(pid_t *pid)
    {
    int status;
// pid = fork();
    if (*pid < 0)
    FAIL_IF_MSG(1, "Failed to fork child");
    if (!*pid) {
    FAIL_IF_EXIT_MSG(ptrace_traceme(), "PTRACE_TRACEME failed");
    FAIL_IF_EXIT_MSG(raise(SIGSTOP), "Child failed to raise SIGSTOP");
    } else {
// Synchronise on child SIGSTOP
    FAIL_IF_MSG(waitpid(*pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    }
    return 0;
    }
//
// Tests the interaction between ptrace and perf watching the same data.
//
// We expect ptrace to take 'priority', as it is has before-execute
// semantics.
//
// The perf counter should not be incremented yet because perf has after-execute
// semantics. E.g., if ptrace changes the child PC, we don't even execute the
// instruction at all.
//
// When the child is stopped for ptrace, we test both continue and single step.
// Both should increment the perf counter. We also test changing the PC somewhere
// different and stepping, which should not increment the perf counter.
//
#[no_mangle]
pub unsafe extern "C" fn same_watch_addr_test() -> c_int {
    int same_watch_addr_test(void)
    {
    struct ppc_hw_breakpoint bp_info;	/* ptrace breakpoint info */
    int bp_id;	/* Breakpoint handle of ptrace watchpoint */
    int perf_fd;	/* File descriptor of perf performance counter */
    u64 perf_count;	/* Most recently fetched perf performance counter value */
    pid_t pid;	/* PID of child process */
    void *pc;	/* Most recently fetched child PC value */
    int status;	/* Stop status of child after waitpid */
    unsigned long value;	/* Dummy value to be read/written to by child */
    int err;
    err = ptrace_fork_child(&pid);
    if (err)
    return err;
    if (!pid) {
    same_watch_addr_child(&value);
    exit(1);
    }
    err = check_watchpoints(pid);
    if (err)
    return err;
// Place a perf watchpoint counter on value
    perf_fd = perf_watchpoint_open(pid, &value, sizeof(value));
    FAIL_IF_MSG(perf_fd < 0, "Failed to open perf performance counter");
// Place a ptrace watchpoint on value
    ppc_ptrace_init_breakpoint(&bp_info, PPC_BREAKPOINT_TRIGGER_READ, &value, sizeof(value));
    bp_id = ppc_ptrace_sethwdbg(pid, &bp_info);
    FAIL_IF_MSG(bp_id < 0, "Failed to set ptrace watchpoint");
// Let the child run. It should stop on the ptrace watchpoint
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_load, "Child did not stop on load instruction");
//
// We stopped before executing the load, so perf should not have
// recorded any events yet
//
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 0, "perf recorded unexpected event");
// Single stepping over the load should increment the perf counter
    FAIL_IF_MSG(ptrace_singlestep(pid, 0), "Failed to single step child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_load + 4, "Failed to single step load instruction");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 1, "perf counter did not increment");
//
// Set up a ptrace watchpoint on the value again and trigger it.
// The perf counter should not have incremented because we do not
// execute the load yet.
//
    FAIL_IF_MSG(ppc_ptrace_delhwdbg(pid, bp_id), "Failed to remove old ptrace watchpoint");
    bp_id = ppc_ptrace_sethwdbg(pid, &bp_info);
    FAIL_IF_MSG(bp_id < 0, "Failed to set ptrace watchpoint");
    FAIL_IF_MSG(ptrace_setreg_pc(pid, same_watch_addr_load), "Failed to set child PC");
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_load, "Child did not stop on load trap");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 1, "perf counter should not have changed");
// Continuing over the load should increment the perf counter
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_trap, "Child did not stop on end trap");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 2, "perf counter did not increment");
//
// If we set the child PC back to the load instruction, then continue,
// we should reach the end trap (because ptrace is one-shot) and have
// another perf event.
//
    FAIL_IF_MSG(ptrace_setreg_pc(pid, same_watch_addr_load), "Failed to set child PC");
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_trap, "Child did not stop on end trap");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 3, "perf counter did not increment");
//
// If we set the child PC back to the load instruction, set a ptrace
// watchpoint on the load, then continue, we should immediately get
// the ptrace trap without incrementing the perf counter
//
    FAIL_IF_MSG(ppc_ptrace_delhwdbg(pid, bp_id), "Failed to remove old ptrace watchpoint");
    bp_id = ppc_ptrace_sethwdbg(pid, &bp_info);
    FAIL_IF_MSG(bp_id < 0, "Failed to set ptrace watchpoint");
    FAIL_IF_MSG(ptrace_setreg_pc(pid, same_watch_addr_load), "Failed to set child PC");
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_load, "Child did not stop on load instruction");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 3, "perf counter should not have changed");
//
// If we change the PC while stopped on the load instruction, we should
// not increment the perf counter (because ptrace is before-execute,
// perf is after-execute).
//
    FAIL_IF_MSG(ptrace_setreg_pc(pid, same_watch_addr_load + 4), "Failed to set child PC");
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != same_watch_addr_trap, "Child did not stop on end trap");
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 3, "perf counter should not have changed");
// Clean up child
    FAIL_IF_MSG(kill(pid, SIGKILL) != 0, "Failed to kill child");
    return 0;
    }
//
// Tests the interaction between ptrace and perf when:
// 1. perf watches a value
// 2. ptrace watches a different value
// 3. The perf value is read, then the ptrace value is read immediately after
//
// A breakpoint implementation may accidentally misattribute/skip one of
// the ptrace or perf handlers, as interrupt based work is done after perf
// and before ptrace.
//
// We expect the perf counter to increment before the ptrace watchpoint
// triggers.
//
#[no_mangle]
pub unsafe extern "C" fn perf_then_ptrace_test() -> c_int {
    int perf_then_ptrace_test(void)
    {
    struct ppc_hw_breakpoint bp_info;	/* ptrace breakpoint info */
    int bp_id;	/* Breakpoint handle of ptrace watchpoint */
    int perf_fd;	/* File descriptor of perf performance counter */
    u64 perf_count;	/* Most recently fetched perf performance counter value */
    pid_t pid;	/* PID of child process */
    void *pc;	/* Most recently fetched child PC value */
    int status;	/* Stop status of child after waitpid */
    unsigned long perf_value;	/* Dummy value to be watched by perf */
    unsigned long ptrace_value;	/* Dummy value to be watched by ptrace */
    int err;
    err = ptrace_fork_child(&pid);
    if (err)
    return err;
//
// If we are the child, run a subroutine that reads the perf value,
// then reads the ptrace value with consecutive load instructions
//
    if (!pid) {
    perf_then_ptrace_child(&perf_value, &ptrace_value);
    exit(0);
    }
    err = check_watchpoints(pid);
    if (err)
    return err;
// Place a perf watchpoint counter
    perf_fd = perf_watchpoint_open(pid, &perf_value, sizeof(perf_value));
    FAIL_IF_MSG(perf_fd < 0, "Failed to open perf performance counter");
// Place a ptrace watchpoint
    ppc_ptrace_init_breakpoint(&bp_info, PPC_BREAKPOINT_TRIGGER_READ,
    &ptrace_value, sizeof(ptrace_value));
    bp_id = ppc_ptrace_sethwdbg(pid, &bp_info);
    FAIL_IF_MSG(bp_id < 0, "Failed to set ptrace watchpoint");
// Let the child run. It should stop on the ptrace watchpoint
    FAIL_IF_MSG(ptrace_cont(pid, 0), "Failed to continue child");
    FAIL_IF_MSG(waitpid(pid, &status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG(ptrace_getreg_pc(pid, &pc), "Failed to get child PC");
    FAIL_IF_MSG(pc != perf_then_ptrace_load2, "Child did not stop on ptrace load");
// perf should have recorded the first load
    FAIL_IF_MSG(perf_read_counter(perf_fd, &perf_count), "Failed to read perf counter");
    FAIL_IF_MSG(perf_count != 1, "perf counter did not increment");
// Clean up child
    FAIL_IF_MSG(kill(pid, SIGKILL) != 0, "Failed to kill child");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut err: c_int = 0;
    err |= test_harness(same_watch_addr_test, "same_watch_addr");
    err |= test_harness(perf_then_ptrace_test, "perf_then_ptrace");
    return err;
    }
