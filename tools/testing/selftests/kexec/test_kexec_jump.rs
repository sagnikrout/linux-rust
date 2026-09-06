//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/kexec/test_kexec_jump.c
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


    asm(
    "  .code64\n"
    "  .data\n"
    "purgatory_start:\n"
// Trigger kexec debug exception handling
    "  int3\n"
// Set load address for next time
    "  leaq purgatory_start_b(%rip), %r11\n"
    "  movq %r11, 8(%rsp)\n"
// Back to Linux
    "  ret\n"
// Same again
    "purgatory_start_b:\n"
// Trigger kexec debug exception handling
    "  int3\n"
// Set load address for next time
    "  leaq purgatory_start(%rip), %r11\n"
    "  movq %r11, 8(%rsp)\n"
// Back to Linux
    "  ret\n"
    "purgatory_end:\n"
    ".previous"
    );
    extern char purgatory_start[], purgatory_end[];
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main (void)
    {
    let mut segment: kexec_segment = {};
    int ret;
    segment.buf = purgatory_start;
    segment.bufsz = purgatory_end - purgatory_start;
    segment.mem = (void *)0x400000;
    segment.memsz = 0x1000;
    ret = syscall(__NR_kexec_load, 0x400000, 1, &segment, KEXEC_PRESERVE_CONTEXT);
    if (ret) {
    perror("kexec_load");
    exit(1);
    }
    ret = syscall(__NR_reboot, LINUX_REBOOT_MAGIC1, LINUX_REBOOT_MAGIC2, LINUX_REBOOT_CMD_KEXEC);
    if (ret) {
    perror("kexec reboot");
    exit(1);
    }
    ret = syscall(__NR_reboot, LINUX_REBOOT_MAGIC1, LINUX_REBOOT_MAGIC2, LINUX_REBOOT_CMD_KEXEC);
    if (ret) {
    perror("kexec reboot");
    exit(1);
    }
    printf("Success\n");
    return 0;
    }
