//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/pgm_check.c
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

#[no_mangle]
pub unsafe extern "C" fn print_stacktrace(sp: c_ulong) {
    void print_stacktrace(unsigned long sp)
    {
    struct stack_info boot_stack = { STACK_TYPE_TASK, (unsigned long)_stack_start,
    (unsigned long)_stack_end };
    let mut first: bool = true;
    boot_emerg("Call Trace:\n");
    while (!(sp & 0x7) && on_stack(&boot_stack, sp, sizeof(struct stack_frame))) {
    struct stack_frame *sf = (struct stack_frame *)sp;
    if (first)
    boot_emerg("(sp:%016lx [<%016lx>] %pS)\n", sp, sf.gprs[8], (void *)sf.gprs[8]);
    else
    boot_emerg(" sp:%016lx [<%016lx>] %pS\n", sp, sf.gprs[8], (void *)sf.gprs[8]);
    if (sf.back_chain <= sp)
    break;
    sp = sf.back_chain;
    first = false;
    }
    }
    extern struct exception_table_entry __start___ex_table[];
    extern struct exception_table_entry __stop___ex_table[];
#[no_mangle]
pub unsafe extern "C" fn extable_insn(x: *const exception_table_entry) -> c_ulong {
    static inline unsigned long extable_insn(const struct exception_table_entry *x)
    {
    return (unsigned long)&x.insn + x.insn;
    }
#[no_mangle]
unsafe extern "C" fn ex_handler(regs: *mut pt_regs) -> bool {
    static bool ex_handler(struct pt_regs *regs)
    {
    const struct exception_table_entry *ex;
    for (ex = __start___ex_table; ex < __stop___ex_table; ex++) {
    if (extable_insn(ex) != regs.psw.addr)
    continue;
    if (ex.type != EX_TYPE_FIXUP)
    return false;
    regs.psw.addr = extable_fixup(ex);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn do_pgm_check(regs: *mut pt_regs) {
    void do_pgm_check(struct pt_regs *regs)
    {
    struct psw_bits *psw = &psw_bits(regs.psw);
    unsigned long *gpregs = regs.gprs;
    if (ex_handler(regs))
    return;
    if (bootdebug)
    boot_rb_dump();
    boot_emerg("Linux version %s\n", kernel_version);
    if (!is_prot_virt_guest() && early_command_line[0])
    boot_emerg("Kernel command line: %s\n", early_command_line);
    boot_emerg("Kernel fault: interruption code %04x ilc:%d\n",
    regs.int_code & 0xffff, regs.int_code >> 17);
    if (kaslr_enabled()) {
    boot_emerg("Kernel random base: %lx\n", __kaslr_offset);
    boot_emerg("Kernel random base phys: %lx\n", __kaslr_offset_phys);
    }
    boot_emerg("PSW : %016lx %016lx (%pS)\n",
    regs.psw.mask, regs.psw.addr, (void *)regs.psw.addr);
    boot_emerg("      R:%x T:%x IO:%x EX:%x Key:%x M:%x W:%x P:%x AS:%x CC:%x PM:%x RI:%x EA:%x\n",
    psw.per, psw.dat, psw.io, psw.ext, psw.key, psw.mcheck,
    psw.wait, psw.pstate, psw.as, psw.cc, psw.pm, psw.ri, psw.eaba);
    boot_emerg("GPRS: %016lx %016lx %016lx %016lx\n", gpregs[0], gpregs[1], gpregs[2], gpregs[3]);
    boot_emerg("      %016lx %016lx %016lx %016lx\n", gpregs[4], gpregs[5], gpregs[6], gpregs[7]);
    boot_emerg("      %016lx %016lx %016lx %016lx\n", gpregs[8], gpregs[9], gpregs[10], gpregs[11]);
    boot_emerg("      %016lx %016lx %016lx %016lx\n", gpregs[12], gpregs[13], gpregs[14], gpregs[15]);
    print_stacktrace(gpregs[15]);
    boot_emerg("Last Breaking-Event-Address:\n");
    boot_emerg(" [<%016lx>] %pS\n", regs.last_break, (void *)regs.last_break);
// Convert to disabled wait PSW
    psw.io = 0;
    psw.ext = 0;
    psw.wait = 1;
    }
