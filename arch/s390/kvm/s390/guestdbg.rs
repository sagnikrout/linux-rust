//! Automatically rewritten from C to Rust
//! Source: arch/s390/kvm/s390/guestdbg.c
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
// kvm guest debug support
//
// Copyright IBM Corp. 2014
//
// Author(s): David Hildenbrand <dahi@linux.vnet.ibm.com>
//

//
// Extends the address range given by *start and *stop to include the address
// range starting with estart and the length len. Takes care of overflowing
// intervals and tries to minimize the overall interval size.
//
#[no_mangle]
unsafe extern "C" fn extend_address_range(start: *mut u64, stop: *mut u64, estart: u64, len: c_int) {
    static void extend_address_range(u64 *start, u64 *stop, u64 estart, int len)
    {
    u64 estop;
    if (len > 0)
    len--;
    else
    len = 0;
    estop = estart + len;
// 0-0 range represents "not set"
    if ((*start == 0) && (*stop == 0)) {
// start = estart;
// stop = estop;
    } else if (*start <= *stop) {
// increase the existing range
    if (estart < *start)
// start = estart;
    if (estop > *stop)
// stop = estop;
    } else {
// "overflowing" interval, whereby *stop > *start
    if (estart <= *stop) {
    if (estop > *stop)
// stop = estop;
    } else if (estop > *start) {
    if (estart < *start)
// start = estart;
    }
// minimize the range
#[no_mangle]
pub unsafe extern "C" fn if(estart): *mut *mut *mut (estop - stop) < (start -) -> else {
    else if ((estop - *stop) < (*start - estart))
// stop = estop;
    else
// start = estart;
    }
    }
pub const MAX_INST_SIZE: c_int = 6;
#[no_mangle]
unsafe extern "C" fn enable_all_hw_bp(vcpu: *mut kvm_vcpu) {
    static void enable_all_hw_bp(struct kvm_vcpu *vcpu)
    {
    unsigned long start, len;
    u64 *cr9 = &vcpu.arch.sie_block.gcr[9];
    u64 *cr10 = &vcpu.arch.sie_block.gcr[10];
    u64 *cr11 = &vcpu.arch.sie_block.gcr[11];
    int i;
    if (vcpu.arch.guestdbg.nr_hw_bp <= 0 ||
    vcpu.arch.guestdbg.hw_bp_info == core::ptr::null_mut())
    return;
//
// If the guest is not interested in branching events, we can safely
// limit them to the PER address range.
//
    if (!(*cr9 & PER_EVENT_BRANCH))
// cr9 |= PER_CONTROL_BRANCH_ADDRESS;
// cr9 |= PER_EVENT_IFETCH | PER_EVENT_BRANCH;
    for (i = 0; i < vcpu.arch.guestdbg.nr_hw_bp; i++) {
    start = vcpu.arch.guestdbg.hw_bp_info[i].addr;
    len = vcpu.arch.guestdbg.hw_bp_info[i].len;
//
// The instruction in front of the desired bp has to
// report instruction-fetching events
//
    if (start < MAX_INST_SIZE) {
    len += start;
    start = 0;
    } else {
    start -= MAX_INST_SIZE;
    len += MAX_INST_SIZE;
    }
    extend_address_range(cr10, cr11, start, len);
    }
    }
#[no_mangle]
unsafe extern "C" fn enable_all_hw_wp(vcpu: *mut kvm_vcpu) {
    static void enable_all_hw_wp(struct kvm_vcpu *vcpu)
    {
    unsigned long start, len;
    u64 *cr9 = &vcpu.arch.sie_block.gcr[9];
    u64 *cr10 = &vcpu.arch.sie_block.gcr[10];
    u64 *cr11 = &vcpu.arch.sie_block.gcr[11];
    int i;
    if (vcpu.arch.guestdbg.nr_hw_wp <= 0 ||
    vcpu.arch.guestdbg.hw_wp_info == core::ptr::null_mut())
    return;
// if host uses storage alternation for special address
// spaces, enable all events and give all to the guest
    if (*cr9 & PER_EVENT_STORE && *cr9 & PER_CONTROL_ALTERATION) {
// cr9 &= ~PER_CONTROL_ALTERATION;
// cr10 = 0;
// cr11 = -1UL;
    } else {
// cr9 &= ~PER_CONTROL_ALTERATION;
// cr9 |= PER_EVENT_STORE;
    for (i = 0; i < vcpu.arch.guestdbg.nr_hw_wp; i++) {
    start = vcpu.arch.guestdbg.hw_wp_info[i].addr;
    len = vcpu.arch.guestdbg.hw_wp_info[i].len;
    extend_address_range(cr10, cr11, start, len);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_backup_guest_per_regs(vcpu: *mut kvm_vcpu) {
    void kvm_s390_backup_guest_per_regs(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.guestdbg.cr0 = vcpu.arch.sie_block.gcr[0];
    vcpu.arch.guestdbg.cr9 = vcpu.arch.sie_block.gcr[9];
    vcpu.arch.guestdbg.cr10 = vcpu.arch.sie_block.gcr[10];
    vcpu.arch.guestdbg.cr11 = vcpu.arch.sie_block.gcr[11];
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_restore_guest_per_regs(vcpu: *mut kvm_vcpu) {
    void kvm_s390_restore_guest_per_regs(struct kvm_vcpu *vcpu)
    {
    vcpu.arch.sie_block.gcr[0] = vcpu.arch.guestdbg.cr0;
    vcpu.arch.sie_block.gcr[9] = vcpu.arch.guestdbg.cr9;
    vcpu.arch.sie_block.gcr[10] = vcpu.arch.guestdbg.cr10;
    vcpu.arch.sie_block.gcr[11] = vcpu.arch.guestdbg.cr11;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_patch_guest_per_regs(vcpu: *mut kvm_vcpu) {
    void kvm_s390_patch_guest_per_regs(struct kvm_vcpu *vcpu)
    {
//
// TODO: if guest psw has per enabled, otherwise 0s!
// This reduces the amount of reported events.
// Need to intercept all psw changes!
//
    if (guestdbg_sstep_enabled(vcpu)) {
// disable timer (clock-comparator) interrupts
    vcpu.arch.sie_block.gcr[0] &= ~CR0_CLOCK_COMPARATOR_SUBMASK;
    vcpu.arch.sie_block.gcr[9] |= PER_EVENT_IFETCH;
    vcpu.arch.sie_block.gcr[10] = 0;
    vcpu.arch.sie_block.gcr[11] = -1UL;
    }
    if (guestdbg_hw_bp_enabled(vcpu)) {
    enable_all_hw_bp(vcpu);
    enable_all_hw_wp(vcpu);
    }
// TODO: Instruction-fetching-nullification not allowed for now
    if (vcpu.arch.sie_block.gcr[9] & PER_EVENT_NULLIFICATION)
    vcpu.arch.sie_block.gcr[9] &= ~PER_EVENT_NULLIFICATION;
    }
pub const MAX_WP_SIZE: c_int = 100;
    static int __import_wp_info(struct kvm_vcpu *vcpu,
    struct kvm_hw_breakpoint *bp_data,
    struct kvm_hw_wp_info_arch *wp_info)
    {
    let mut ret: c_int = 0;
    wp_info.len = bp_data.len;
    wp_info.addr = bp_data.addr;
    wp_info.phys_addr = bp_data.phys_addr;
    wp_info.old_data = core::ptr::null_mut();
    if (wp_info.len < 0 || wp_info.len > MAX_WP_SIZE)
    return -EINVAL;
    wp_info.old_data = kmalloc(wp_info.len, GFP_KERNEL_ACCOUNT);
    if (!wp_info.old_data)
    return -ENOMEM;
// try to backup the original value
    ret = read_guest_abs(vcpu, wp_info.phys_addr, wp_info.old_data,
    wp_info.len);
    if (ret) {
    kfree(wp_info.old_data);
    wp_info.old_data = core::ptr::null_mut();
    }
    return ret;
    }
pub const MAX_BP_COUNT: c_int = 50;
    int kvm_s390_import_bp_data(struct kvm_vcpu *vcpu,
    struct kvm_guest_debug *dbg)
    {
    let mut ret: c_int = 0, nr_wp = 0, nr_bp = 0, i;
    struct kvm_hw_breakpoint *bp_data = core::ptr::null_mut();
    struct kvm_hw_wp_info_arch *wp_info = core::ptr::null_mut();
    struct kvm_hw_bp_info_arch *bp_info = core::ptr::null_mut();
    if (dbg.arch.nr_hw_bp <= 0 || !dbg.arch.hw_bp)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(MAX_BP_COUNT: dbg->arch.nr_hw_bp >) -> else {
    else if (dbg.arch.nr_hw_bp > MAX_BP_COUNT)
    return -EINVAL;
    bp_data = memdup_array_user(dbg.arch.hw_bp, dbg.arch.nr_hw_bp,
    sizeof(*bp_data));
    if (IS_ERR(bp_data))
    return PTR_ERR(bp_data);
    for (i = 0; i < dbg.arch.nr_hw_bp; i++) {
    switch (bp_data[i].type) {
    case KVM_HW_WP_WRITE:
    nr_wp++;
    break;
    case KVM_HW_BP:
    nr_bp++;
    break;
    default:
    break;
    }
    }
    if (nr_wp > 0) {
    wp_info = kmalloc_objs(*wp_info, nr_wp, GFP_KERNEL_ACCOUNT);
    if (!wp_info) {
    ret = -ENOMEM;
    goto error;
    }
    }
    if (nr_bp > 0) {
    bp_info = kmalloc_objs(*bp_info, nr_bp, GFP_KERNEL_ACCOUNT);
    if (!bp_info) {
    ret = -ENOMEM;
    goto error;
    }
    }
    for (nr_wp = 0, nr_bp = 0, i = 0; i < dbg.arch.nr_hw_bp; i++) {
    switch (bp_data[i].type) {
    case KVM_HW_WP_WRITE:
    ret = __import_wp_info(vcpu, &bp_data[i],
    &wp_info[nr_wp]);
    if (ret)
    goto error_wp;
    nr_wp++;
    break;
    case KVM_HW_BP:
    bp_info[nr_bp].len = bp_data[i].len;
    bp_info[nr_bp].addr = bp_data[i].addr;
    nr_bp++;
    break;
    }
    }
    vcpu.arch.guestdbg.nr_hw_bp = nr_bp;
    vcpu.arch.guestdbg.hw_bp_info = bp_info;
    vcpu.arch.guestdbg.nr_hw_wp = nr_wp;
    vcpu.arch.guestdbg.hw_wp_info = wp_info;
    kfree(bp_data);
    return 0;
    error_wp:
    while (nr_wp--)
    kfree(wp_info[nr_wp].old_data);
    error:
    kfree(bp_data);
    kfree(wp_info);
    kfree(bp_info);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_clear_bp_data(vcpu: *mut kvm_vcpu) {
    void kvm_s390_clear_bp_data(struct kvm_vcpu *vcpu)
    {
    int i;
    struct kvm_hw_wp_info_arch *hw_wp_info = core::ptr::null_mut();
    for (i = 0; i < vcpu.arch.guestdbg.nr_hw_wp; i++) {
    hw_wp_info = &vcpu.arch.guestdbg.hw_wp_info[i];
    kfree(hw_wp_info.old_data);
    hw_wp_info.old_data = core::ptr::null_mut();
    }
    kfree(vcpu.arch.guestdbg.hw_wp_info);
    vcpu.arch.guestdbg.hw_wp_info = core::ptr::null_mut();
    kfree(vcpu.arch.guestdbg.hw_bp_info);
    vcpu.arch.guestdbg.hw_bp_info = core::ptr::null_mut();
    vcpu.arch.guestdbg.nr_hw_wp = 0;
    vcpu.arch.guestdbg.nr_hw_bp = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn in_addr_range(addr: u64, a: u64, b: u64) -> c_int {
    static inline int in_addr_range(u64 addr, u64 a, u64 b)
    {
    if (a <= b)
    return (addr >= a) && (addr <= b);
    else
// "overflowing" interval
    return (addr >= a) || (addr <= b);
    }

    static struct kvm_hw_bp_info_arch *find_hw_bp(struct kvm_vcpu *vcpu,
    unsigned long addr)
    {
    struct kvm_hw_bp_info_arch *bp_info = vcpu.arch.guestdbg.hw_bp_info;
    int i;
    if (vcpu.arch.guestdbg.nr_hw_bp == 0)
    return core::ptr::null_mut();
    for (i = 0; i < vcpu.arch.guestdbg.nr_hw_bp; i++) {
// addr is directly the start or in the range of a bp
    if (addr == bp_info.addr)
    goto found;
    if (bp_info.len > 0 &&
    in_addr_range(addr, bp_info.addr, end_of_range(bp_info)))
    goto found;
    bp_info++;
    }
    return core::ptr::null_mut();
    found:
    return bp_info;
    }
    static struct kvm_hw_wp_info_arch *any_wp_changed(struct kvm_vcpu *vcpu)
    {
    int i;
    struct kvm_hw_wp_info_arch *wp_info = core::ptr::null_mut();
    void *temp = core::ptr::null_mut();
    if (vcpu.arch.guestdbg.nr_hw_wp == 0)
    return core::ptr::null_mut();
    for (i = 0; i < vcpu.arch.guestdbg.nr_hw_wp; i++) {
    wp_info = &vcpu.arch.guestdbg.hw_wp_info[i];
    if (!wp_info || !wp_info.old_data || wp_info.len <= 0)
    continue;
    temp = kmalloc(wp_info.len, GFP_KERNEL_ACCOUNT);
    if (!temp)
    continue;
// refetch the wp data and compare it to the old value
    if (!read_guest_abs(vcpu, wp_info.phys_addr, temp,
    wp_info.len)) {
    if (memcmp(temp, wp_info.old_data, wp_info.len)) {
    kfree(temp);
    return wp_info;
    }
    }
    kfree(temp);
    temp = core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_prepare_debug_exit(vcpu: *mut kvm_vcpu) {
    void kvm_s390_prepare_debug_exit(struct kvm_vcpu *vcpu)
    {
    vcpu.run.exit_reason = KVM_EXIT_DEBUG;
    vcpu.guest_debug &= ~KVM_GUESTDBG_EXIT_PENDING;
    }

    (code & (PER_CODE_IFETCH | PER_CODE_BRANCH))

    (code & (PER_CODE_STORE | PER_CODE_STORE_REAL))
    static int debug_exit_required(struct kvm_vcpu *vcpu, u8 perc,
    unsigned long peraddr)
    {
    struct kvm_debug_exit_arch *debug_exit = &vcpu.run.debug.arch;
    struct kvm_hw_wp_info_arch *wp_info = core::ptr::null_mut();
    struct kvm_hw_bp_info_arch *bp_info = core::ptr::null_mut();
    let mut addr: c_ulong = vcpu.arch.sie_block.gpsw.addr;
    if (guestdbg_hw_bp_enabled(vcpu)) {
    if (per_write_wp_event(perc) &&
    vcpu.arch.guestdbg.nr_hw_wp > 0) {
    wp_info = any_wp_changed(vcpu);
    if (wp_info) {
    debug_exit.addr = wp_info.addr;
    debug_exit.type = KVM_HW_WP_WRITE;
    goto exit_required;
    }
    }
    if (per_bp_event(perc) &&
    vcpu.arch.guestdbg.nr_hw_bp > 0) {
    bp_info = find_hw_bp(vcpu, addr);
// remove duplicate events if PC==PER address
    if (bp_info && (addr != peraddr)) {
    debug_exit.addr = addr;
    debug_exit.type = KVM_HW_BP;
    vcpu.arch.guestdbg.last_bp = addr;
    goto exit_required;
    }
// breakpoint missed
    bp_info = find_hw_bp(vcpu, peraddr);
    if (bp_info && vcpu.arch.guestdbg.last_bp != peraddr) {
    debug_exit.addr = peraddr;
    debug_exit.type = KVM_HW_BP;
    goto exit_required;
    }
    }
    }
    if (guestdbg_sstep_enabled(vcpu) && per_bp_event(perc)) {
    debug_exit.addr = addr;
    debug_exit.type = KVM_SINGLESTEP;
    goto exit_required;
    }
    return 0;
    exit_required:
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn per_fetched_addr(vcpu: *mut kvm_vcpu, addr: *mut c_ulong) -> c_int {
    static int per_fetched_addr(struct kvm_vcpu *vcpu, unsigned long *addr)
    {
    let mut exec_ilen: u8 = 0;
    u16 opcode[3];
    int rc;
    if (vcpu.arch.sie_block.icptcode == ICPT_PROGI) {
// PER address references the fetched or the execute instr
// addr = vcpu->arch.sie_block->peraddr;
//
// Manually detect if we have an EXECUTE instruction. As
// instructions are always 2 byte aligned we can read the
// first two bytes unconditionally
//
    rc = read_guest_instr(vcpu, *addr, &opcode, 2);
    if (rc)
    return rc;
    if (opcode[0] >> 8 == 0x44)
    exec_ilen = 4;
    if ((opcode[0] & 0xff0f) == 0xc600)
    exec_ilen = 6;
    } else {
// instr was suppressed, calculate the responsible instr
// addr = __rewind_psw(vcpu->arch.sie_block->gpsw,
    kvm_s390_get_ilen(vcpu));
    if (vcpu.arch.sie_block.icptstatus & 0x01) {
    exec_ilen = (vcpu.arch.sie_block.icptstatus & 0x60) >> 4;
    if (!exec_ilen)
    exec_ilen = 4;
    }
    }
    if (exec_ilen) {
// read the complete EXECUTE instr to detect the fetched addr
    rc = read_guest_instr(vcpu, *addr, &opcode, exec_ilen);
    if (rc)
    return rc;
    if (exec_ilen == 6) {
// EXECUTE RELATIVE LONG - RIL-b format
    let mut rl: i32 = *((s32 *) (opcode + 1));
// rl is a _signed_ 32 bit value specifying halfwords
// addr += (u64)(s64) rl * 2;
    } else {
// EXECUTE - RX-a format
    let mut base: u32 = (opcode[1] & 0xf000) >> 12;
    let mut disp: u32 = opcode[1] & 0x0fff;
    let mut index: u32 = opcode[0] & 0x000f;
// addr = base ? vcpu->run->s.regs.gprs[base] : 0;
// addr += index ? vcpu->run->s.regs.gprs[index] : 0;
// addr += disp;
    }
// addr = kvm_s390_logical_to_effective(vcpu, *addr);
    }
    return 0;
    }

    (vcpu.arch.sie_block.gpsw.mask & PSW_MASK_PER)
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_handle_per_ifetch_icpt(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_s390_handle_per_ifetch_icpt(struct kvm_vcpu *vcpu)
    {
    let mut cr10: u64 = vcpu.arch.sie_block.gcr[10];
    let mut cr11: u64 = vcpu.arch.sie_block.gcr[11];
    let mut ilen: u8 = kvm_s390_get_ilen(vcpu);
    struct kvm_s390_pgm_info pgm_info = {
    .code = PGM_PER,
    .per_code = PER_CODE_IFETCH,
    .per_address = __rewind_psw(vcpu.arch.sie_block.gpsw, ilen),
    };
    unsigned long fetched_addr;
    int rc;
//
// The PSW points to the next instruction, therefore the intercepted
// instruction generated a PER i-fetch event. PER address therefore
// points at the previous PSW address (could be an EXECUTE function).
//
    if (!guestdbg_enabled(vcpu))
    return kvm_s390_inject_prog_irq(vcpu, &pgm_info);
    if (debug_exit_required(vcpu, pgm_info.per_code, pgm_info.per_address))
    vcpu.guest_debug |= KVM_GUESTDBG_EXIT_PENDING;
    if (!guest_per_enabled(vcpu) ||
    !(vcpu.arch.sie_block.gcr[9] & PER_EVENT_IFETCH))
    return 0;
    rc = per_fetched_addr(vcpu, &fetched_addr);
    if (rc < 0)
    return rc;
    if (rc)
// instruction-fetching exceptions
    return kvm_s390_inject_program_int(vcpu, PGM_ADDRESSING);
    if (in_addr_range(fetched_addr, cr10, cr11))
    return kvm_s390_inject_prog_irq(vcpu, &pgm_info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn filter_guest_per_event(vcpu: *mut kvm_vcpu) -> c_int {
    static int filter_guest_per_event(struct kvm_vcpu *vcpu)
    {
    let mut perc: u8 = vcpu.arch.sie_block.perc;
    let mut addr: u64 = vcpu.arch.sie_block.gpsw.addr;
    let mut cr9: u64 = vcpu.arch.sie_block.gcr[9];
    let mut cr10: u64 = vcpu.arch.sie_block.gcr[10];
    let mut cr11: u64 = vcpu.arch.sie_block.gcr[11];
// filter all events, demanded by the guest
    let mut guest_perc: u8 = perc & (cr9 >> 24) & PER_CODE_MASK;
    unsigned long fetched_addr;
    int rc;
    if (!guest_per_enabled(vcpu))
    guest_perc = 0;
// filter "successful-branching" events
    if (guest_perc & PER_CODE_BRANCH &&
    cr9 & PER_CONTROL_BRANCH_ADDRESS &&
    !in_addr_range(addr, cr10, cr11))
    guest_perc &= ~PER_CODE_BRANCH;
// filter "instruction-fetching" events
    if (guest_perc & PER_CODE_IFETCH) {
    rc = per_fetched_addr(vcpu, &fetched_addr);
    if (rc < 0)
    return rc;
//
// Don't inject an irq on exceptions. This would make handling
// on icpt code 8 very complex (as PSW was already rewound).
//
    if (rc || !in_addr_range(fetched_addr, cr10, cr11))
    guest_perc &= ~PER_CODE_IFETCH;
    }
// All other PER events will be given to the guest
// TODO: Check altered address/address space
    vcpu.arch.sie_block.perc = guest_perc;
    if (!guest_perc)
    vcpu.arch.sie_block.iprcc &= ~PGM_PER;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn kvm_s390_handle_per_event(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_s390_handle_per_event(struct kvm_vcpu *vcpu)
    {
    int rc, new_as;
    if (debug_exit_required(vcpu, vcpu.arch.sie_block.perc,
    vcpu.arch.sie_block.peraddr))
    vcpu.guest_debug |= KVM_GUESTDBG_EXIT_PENDING;
    rc = filter_guest_per_event(vcpu);
    if (rc)
    return rc;
//
// Only RP, SAC, SACF, PT, PTI, PR, PC instructions can trigger
// a space-switch event. PER events enforce space-switch events
// for these instructions. So if no PER event for the guest is left,
// we might have to filter the space-switch element out, too.
//
    if (vcpu.arch.sie_block.iprcc == PGM_SPACE_SWITCH) {
    vcpu.arch.sie_block.iprcc = 0;
    new_as = psw_bits(vcpu.arch.sie_block.gpsw).as;
//
// If the AS changed from / to home, we had RP, SAC or SACF
// instruction. Check primary and home space-switch-event
// controls. (theoretically home -> home produced no event)
//
    if (((new_as == PSW_BITS_AS_HOME) ^ old_as_is_home(vcpu)) &&
    (pssec(vcpu) || hssec(vcpu)))
    vcpu.arch.sie_block.iprcc = PGM_SPACE_SWITCH;
//
// PT, PTI, PR, PC instruction operate on primary AS only. Check
// if the primary-space-switch-event control was or got set.
//
    if (new_as == PSW_BITS_AS_PRIMARY && !old_as_is_home(vcpu) &&
    (pssec(vcpu) || old_ssec(vcpu)))
    vcpu.arch.sie_block.iprcc = PGM_SPACE_SWITCH;
    }
    return 0;
    }
