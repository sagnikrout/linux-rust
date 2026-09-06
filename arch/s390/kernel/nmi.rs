//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/nmi.c
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
// Machine check handler
//
// Copyright IBM Corp. 2000, 2009
// Author(s): Ingo Adlung <adlung@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Cornelia Huck <cornelia.huck@de.ibm.com>,
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcck_struct {
    pub 1: unsigned int kill_task :,
    pub 1: unsigned int channel_report :,
    pub 1: unsigned int warning :,
    pub 1: unsigned int stp_queue :,
    pub mcck_code: c_ulong,
}

    static DEFINE_PER_CPU(struct mcck_struct, cpu_mcck);
#[no_mangle]
pub unsafe extern "C" fn nmi_needs_mcesa() -> c_int {
    static inline int nmi_needs_mcesa(void)
    {
    return cpu_has_vx() || cpu_has_gs();
    }
//
// The initial machine check extended save area for the boot CPU.
// It will be replaced on the boot CPU reinit with an allocated
// structure. The structure is required for machine check happening
// early in the boot process.
//
    static struct mcesa boot_mcesa __aligned(MCESA_MAX_SIZE);
#[no_mangle]
pub unsafe extern "C" fn nmi_alloc_mcesa_early(mcesad: *mut u64) -> void __init {
    void __init nmi_alloc_mcesa_early(u64 *mcesad)
    {
    if (!nmi_needs_mcesa())
    return;
// mcesad = __pa(&boot_mcesa);
    if (cpu_has_gs())
// mcesad |= ilog2(MCESA_MAX_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn nmi_alloc_mcesa(mcesad: *mut u64) -> c_int {
    int nmi_alloc_mcesa(u64 *mcesad)
    {
    unsigned long size;
    void *origin;
// mcesad = 0;
    if (!nmi_needs_mcesa())
    return 0;
    size = cpu_has_gs() ? MCESA_MAX_SIZE : MCESA_MIN_SIZE;
    origin = kmalloc(size, GFP_KERNEL);
    if (!origin)
    return -ENOMEM;
// The pointer is stored with mcesa_bits ORed in
    kmemleak_not_leak(origin);
// mcesad = __pa(origin);
    if (cpu_has_gs())
// mcesad |= ilog2(MCESA_MAX_SIZE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nmi_free_mcesa(mcesad: *mut u64) {
    void nmi_free_mcesa(u64 *mcesad)
    {
    if (!nmi_needs_mcesa())
    return;
    kfree(__va(*mcesad & MCESA_ORIGIN_MASK));
    }
    static __always_inline char *nmi_puts(char *dest, const char *src)
    {
    while (*src)
// dest++ = *src++;
// dest = 0;
    return dest;
    }
    static __always_inline char *u64_to_hex(char *dest, u64 val)
    {
    int i, num;
    for (i = 1; i <= 16; i++) {
    num = (val >> (64 - 4 * i)) & 0xf;
    if (num >= 10)
// dest++ = 'A' + num - 10;
    else
// dest++ = '0' + num;
    }
// dest = 0;
    return dest;
    }
#[no_mangle]
unsafe extern "C" fn nmi_print_info() -> notrace void {
    static notrace void nmi_print_info(void)
    {
    struct lowcore *lc = get_lowcore();
    char message[100];
    char *ptr;
    int i;
    ptr = nmi_puts(message, "Unrecoverable machine check, code: ");
    ptr = u64_to_hex(ptr, lc.mcck_interruption_code);
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = nmi_puts(message, init_utsname().release);
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = nmi_puts(message, arch_hw_string);
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = nmi_puts(message, "PSW: ");
    ptr = u64_to_hex(ptr, lc.mcck_old_psw.mask);
    ptr = nmi_puts(ptr, " ");
    ptr = u64_to_hex(ptr, lc.mcck_old_psw.addr);
    ptr = nmi_puts(ptr, " PFX: ");
    ptr = u64_to_hex(ptr, (u64)get_lowcore());
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = nmi_puts(message, "LBA: ");
    ptr = u64_to_hex(ptr, lc.last_break_save_area);
    ptr = nmi_puts(ptr, " EDC: ");
    ptr = u64_to_hex(ptr, lc.external_damage_code);
    ptr = nmi_puts(ptr, " FSA: ");
    ptr = u64_to_hex(ptr, lc.failing_storage_address);
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = nmi_puts(message, "CRS:\n");
    sclp_emergency_printk(message);
    ptr = message;
    for (i = 0; i < 16; i++) {
    ptr = u64_to_hex(ptr, lc.cregs_save_area[i].val);
    ptr = nmi_puts(ptr, " ");
    if ((i + 1) % 4 == 0) {
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = message;
    }
    }
    ptr = nmi_puts(message, "GPRS:\n");
    sclp_emergency_printk(message);
    ptr = message;
    for (i = 0; i < 16; i++) {
    ptr = u64_to_hex(ptr, lc.gpregs_save_area[i]);
    ptr = nmi_puts(ptr, " ");
    if ((i + 1) % 4 == 0) {
    ptr = nmi_puts(ptr, "\n");
    sclp_emergency_printk(message);
    ptr = message;
    }
    }
    ptr = nmi_puts(message, "System stopped\n");
    sclp_emergency_printk(message);
    }
#[no_mangle]
unsafe extern "C" fn s390_handle_damage() -> notrace void __noreturn {
    static notrace void __noreturn s390_handle_damage(void)
    {
    struct lowcore *lc = get_lowcore();
    union ctlreg0 cr0, cr0_new;
    psw_t psw_save;
    smp_emergency_stop();
    diag_amode31_ops.diag308_reset();
//
// Disable low address protection and make machine check new PSW a
// disabled wait PSW. Any additional machine check cannot be handled.
//
    local_ctl_store(0, &cr0.reg);
    cr0_new = cr0;
    cr0_new.lap = 0;
    local_ctl_load(0, &cr0_new.reg);
    psw_save = lc.mcck_new_psw;
    psw_bits(lc.mcck_new_psw).io = 0;
    psw_bits(lc.mcck_new_psw).ext = 0;
    psw_bits(lc.mcck_new_psw).wait = 1;
    nmi_print_info();
//
// Restore machine check new PSW and control register 0 to original
// values. This makes possible system dump analysis easier.
//
    lc.mcck_new_psw = psw_save;
    local_ctl_load(0, &cr0.reg);
    disabled_wait();
    }
    NOKPROBE_SYMBOL(s390_handle_damage);
//
// Main machine check handler function. Will be called with interrupts disabled
// and machine checks enabled.
//
#[no_mangle]
pub unsafe extern "C" fn s390_handle_mcck() {
    void s390_handle_mcck(void)
    {
    struct mcck_struct mcck;
    unsigned long mflags;
//
// Disable machine checks and get the current state of accumulated
// machine checks. Afterwards delete the old state and enable machine
// checks again.
//
    local_mcck_save(mflags);
    mcck = *this_cpu_ptr(&cpu_mcck);
    memset(this_cpu_ptr(&cpu_mcck), 0, sizeof(mcck));
    local_mcck_restore(mflags);
    if (mcck.channel_report)
    crw_handle_channel_report();
//
// A warning may remain for a prolonged period on the bare iron.
// (actually until the machine is powered off, or the problem is gone)
// So we just stop listening for the WARNING MCH and avoid continuously
// being interrupted.  One caveat is however, that we must do this per
// processor and cannot use the smp version of ctl_clear_bit().
// On VM we only get one interrupt per virtally presented machinecheck.
// Though one suffices, we may get one interrupt per (virtual) cpu.
//
    if (mcck.warning) {	/* WARNING pending ? */
    let mut mchchk_wng_posted: static int = 0;
// Use single cpu clear, as we cannot handle smp here.
    local_ctl_clear_bit(14, CR14_WARNING_SUBMASK_BIT);
    if (xchg(&mchchk_wng_posted, 1) == 0)
    kill_cad_pid(SIGPWR, 1);
    }
    if (mcck.stp_queue)
    stp_queue_work();
    if (mcck.kill_task) {
    printk(KERN_EMERG "mcck: Terminating task because of machine "
    "malfunction (code 0x%016lx).\n", mcck.mcck_code);
    printk(KERN_EMERG "mcck: task: %s, pid: %d.\n",
    current.comm, current.pid);
    if (is_global_init(current))
    panic("mcck: Attempting to kill init!\n");
    do_send_sig_info(SIGKILL, SEND_SIG_PRIV, current, PIDTYPE_PID);
    }
    }
//
// nmi_registers_valid - verify if registers are valid
// @mci: machine check interruption code
//
// Inspect a machine check interruption code and verify if all required
// registers are valid. For some registers the corresponding validity bit is
// ignored and the registers are set to the expected value.
// Returns true if all registers are valid, otherwise false.
//
#[no_mangle]
unsafe extern "C" fn nmi_registers_valid(mci: union mci) -> bool notrace {
    static bool notrace nmi_registers_valid(union mci mci)
    {
    union ctlreg2 cr2;
//
// The getcpu vdso syscall reads the CPU number from the programmable
// field of the TOD clock. Disregard the TOD programmable register
// validity bit and load the CPU number into the TOD programmable field
// unconditionally.
//
    set_tod_programmable_field(raw_smp_processor_id());
//
// Set the clock comparator register to the next expected value.
//
    set_clock_comparator(get_lowcore().clock_comparator);
    if (!mci.gr || !mci.fp || !mci.fc)
    return false;
//
// The vector validity must only be checked if not running a
// KVM guest. For KVM guests the machine check is forwarded by
// KVM and it is the responsibility of the guest to take
// appropriate actions. The host vector or FPU values have been
// saved by KVM and will be restored by KVM.
//
    if (!mci.vr && !test_cpu_flag(CIF_MCCK_GUEST))
    return false;
    if (!mci.ar)
    return false;
//
// Two cases for guarded storage registers:
// - machine check in kernel or userspace
// - machine check while running SIE (KVM guest)
// For kernel or userspace the userspace values of guarded storage
// control can not be recreated, the process must be terminated.
// For SIE the guest values of guarded storage can not be recreated.
// This is either due to a bug or due to GS being disabled in the
// guest. The guest will be notified by KVM code and the guests machine
// check handling must take care of this. The host values are saved by
// KVM and are not affected.
//
    cr2.reg = get_lowcore().cregs_save_area[2];
    if (cr2.gse && !mci.gs && !test_cpu_flag(CIF_MCCK_GUEST))
    return false;
    if (!mci.ms || !mci.pm || !mci.ia)
    return false;
    return true;
    }
    NOKPROBE_SYMBOL(nmi_registers_valid);
//
// Backup the guest's machine check info to its description block
//
#[no_mangle]
unsafe extern "C" fn s390_backup_mcck_info(regs: *mut pt_regs) -> void notrace {
    static void notrace s390_backup_mcck_info(struct pt_regs *regs)
    {
    struct mcck_volatile_info *mcck_backup;
    struct sie_page *sie_page;
// r14 contains the sie block, which was set in sie64a
    struct kvm_s390_sie_block *sie_block = phys_to_virt(regs.gprs[14]);
    if (sie_block == core::ptr::null_mut())
// Something's seriously wrong, stop system.
    s390_handle_damage();
    sie_page = container_of(sie_block, struct sie_page, sie_block);
    mcck_backup = &sie_page.mcck_info;
    mcck_backup.mcic = get_lowcore().mcck_interruption_code & ~MCCK_CODE_NO_GUEST;
    mcck_backup.ext_damage_code = get_lowcore().external_damage_code;
    mcck_backup.failing_storage_address = get_lowcore().failing_storage_address;
    }
    NOKPROBE_SYMBOL(s390_backup_mcck_info);
pub const MAX_IPD_COUNT: c_int = 29;

//
// machine check handler.
//
#[no_mangle]
pub unsafe extern "C" fn s390_do_machine_check(regs: *mut pt_regs) -> void notrace {
    void notrace s390_do_machine_check(struct pt_regs *regs)
    {
    bool percpu_needs_fixup;
    static int ipd_count;
    static DEFINE_SPINLOCK(ipd_lock);
    static unsigned long long last_ipd;
    struct lowcore *lc = get_lowcore();
    struct mcck_struct *mcck;
    unsigned long long tmp;
    irqentry_state_t irq_state;
    union mci mci;
    unsigned long mcck_dam_code;
    let mut mcck_pending: c_int = 0;
    percpu_entry(regs);
    irq_state = irqentry_nmi_enter(regs);
    if (user_mode(regs))
    update_timer_mcck();
    inc_irq_stat(NMI_NMI);
    mci.val = lc.mcck_interruption_code;
    mcck = this_cpu_ptr(&cpu_mcck);
//
// Reinject the instruction processing damages' machine checks
// including Delayed Access Exception into the guest
// instead of damaging the host if they happen in the guest.
//
    if (mci.pd && !test_cpu_flag(CIF_MCCK_GUEST)) {
    if (mci.b) {
// Processing backup -> verify if we can survive this
    u64 z_mcic, o_mcic, t_mcic;
    z_mcic = (1ULL<<63 | 1ULL<<59 | 1ULL<<29);
    o_mcic = (1ULL<<43 | 1ULL<<42 | 1ULL<<41 | 1ULL<<40 |
    1ULL<<36 | 1ULL<<35 | 1ULL<<34 | 1ULL<<32 |
    1ULL<<30 | 1ULL<<21 | 1ULL<<20 | 1ULL<<17 |
    1ULL<<16);
    t_mcic = mci.val;
    if (((t_mcic & z_mcic) != 0) ||
    ((t_mcic & o_mcic) != o_mcic)) {
    s390_handle_damage();
    }
//
// Nullifying exigent condition, therefore we might
// retry this instruction.
//
    spin_lock(&ipd_lock);
    tmp = get_tod_clock();
    if (((tmp - last_ipd) >> 12) < MAX_IPD_TIME)
    ipd_count++;
    else
    ipd_count = 1;
    last_ipd = tmp;
    if (ipd_count == MAX_IPD_COUNT)
    s390_handle_damage();
    spin_unlock(&ipd_lock);
    } else {
// Processing damage -> stopping machine
    s390_handle_damage();
    }
    }
    if (!nmi_registers_valid(mci)) {
    if (!user_mode(regs))
    s390_handle_damage();
//
// Couldn't restore all register contents for the
// user space process -> mark task for termination.
//
    mcck.kill_task = 1;
    mcck.mcck_code = mci.val;
    mcck_pending = 1;
    }
//
// Backup the machine check's info if it happens when the guest
// is running.
//
    if (test_cpu_flag(CIF_MCCK_GUEST))
    s390_backup_mcck_info(regs);
    if (mci.cd) {
// Timing facility damage
    s390_handle_damage();
    }
    if (mci.ed && mci.ec) {
// External damage
    if (lc.external_damage_code & (1U << ED_STP_SYNC))
    mcck.stp_queue |= stp_sync_check();
    if (lc.external_damage_code & (1U << ED_STP_ISLAND))
    mcck.stp_queue |= stp_island_check();
    mcck_pending = 1;
    }
//
// Reinject storage related machine checks into the guest if they
// happen when the guest is running.
//
    if (!test_cpu_flag(CIF_MCCK_GUEST)) {
// Storage error uncorrected
    if (mci.se)
    s390_handle_damage();
// Storage key-error uncorrected
    if (mci.ke)
    s390_handle_damage();
// Storage degradation
    if (mci.ds && mci.fa)
    s390_handle_damage();
    }
    if (mci.cp) {
// Channel report word pending
    mcck.channel_report = 1;
    mcck_pending = 1;
    }
    if (mci.w) {
// Warning pending
    mcck.warning = 1;
    mcck_pending = 1;
    }
//
// If there are only Channel Report Pending and External Damage
// machine checks, they will not be reinjected into the guest
// because they refer to host conditions only.
//
    mcck_dam_code = (mci.val & MCIC_SUBCLASS_MASK);
    if (test_cpu_flag(CIF_MCCK_GUEST) &&
    (mcck_dam_code & MCCK_CODE_NO_GUEST) != mcck_dam_code) {
// Set sie return code for host's later handling
    ((struct stack_frame *)regs.gprs[15]).sie_return = SIE64_RETURN_MCCK;
    }
    clear_cpu_flag(CIF_MCCK_GUEST);
    if (mcck_pending)
    schedule_mcck_handler();
    percpu_needs_fixup = percpu_code_check(regs);
    irqentry_nmi_exit(regs, irq_state);
    percpu_exit(regs, percpu_needs_fixup);
    }
    NOKPROBE_SYMBOL(s390_do_machine_check);
#[no_mangle]
unsafe extern "C" fn machine_check_init() -> int __init {
    static int __init machine_check_init(void)
    {
    system_ctl_set_bit(14, CR14_EXTERNAL_DAMAGE_SUBMASK_BIT);
    system_ctl_set_bit(14, CR14_RECOVERY_SUBMASK_BIT);
    system_ctl_set_bit(14, CR14_WARNING_SUBMASK_BIT);
    return 0;
    }
    early_initcall(machine_check_init);
