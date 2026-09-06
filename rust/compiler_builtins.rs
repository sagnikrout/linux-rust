
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

//! Our own `compiler_builtins`.
//!
//! Rust provides [`compiler_builtins`] as a port of LLVM's [`compiler-rt`].
//! Since we do not need the vast majority of them, we avoid the dependency
//! by providing this file.
//!
//! At the moment, some builtins are required that should not be. For instance,
//! [`core`] has 128-bit integers functionality which we should not be compiling
//! in. We will work with upstream [`core`] to provide feature flags to disable
//! the parts we do not need. For the moment, we define them to [`panic!`] at
//! runtime for simplicity to catch mistakes, instead of performing surgery
//! on `core.o`.
//!
//! In any case, all these symbols are weakened to ensure we do not override
//! those that may be provided by the rest of the kernel.
//!
//! [`compiler_builtins`]: https://github.com/rust-lang/compiler-builtins
//! [`compiler-rt`]: https://compiler-rt.llvm.org/

#![allow(internal_features)]
#![feature(compiler_builtins)]
#![compiler_builtins]
#![no_builtins]
#![no_std]

macro_rules! define_panicking_intrinsics(
    ($reason: tt, { $($ident: ident, )* }) => {
        $(
            #[doc(hidden)]
            #[export_name = concat!("__rust", stringify!($ident))]
            pub extern "C" fn $ident() {
                panic!($reason);
            }
        )*
    }
);

define_panicking_intrinsics!("`f32` should not be used", {
    __addsf3,
    __eqsf2,
    __extendsfdf2,
    __gesf2,
    __lesf2,
    __ltsf2,
    __mulsf3,
    __nesf2,
    __truncdfsf2,
    __unordsf2,
});

define_panicking_intrinsics!("`f64` should not be used", {
    __adddf3,
    __eqdf2,
    __ledf2,
    __ltdf2,
    __muldf3,
    __unorddf2,
});

define_panicking_intrinsics!("`i128` should not be used", {
    __ashrti3,
    __muloti4,
    __multi3,
});

define_panicking_intrinsics!("`u128` should not be used", {
    __ashlti3,
    __lshrti3,
    __udivmodti4,
    __udivti3,
    __umodti3,
});

#[cfg(target_arch = "arm")]
define_panicking_intrinsics!("`f32` should not be used", {
    __aeabi_fadd,
    __aeabi_fmul,
    __aeabi_fcmpeq,
    __aeabi_fcmple,
    __aeabi_fcmplt,
    __aeabi_fcmpun,
});

#[cfg(target_arch = "arm")]
define_panicking_intrinsics!("`f64` should not be used", {
    __aeabi_dadd,
    __aeabi_dmul,
    __aeabi_dcmple,
    __aeabi_dcmplt,
    __aeabi_dcmpun,
});

#[cfg(target_arch = "arm")]
define_panicking_intrinsics!("`u64` division/modulo should not be used", {
    __aeabi_uldivmod,
});

#[cfg(target_arch = "powerpc")]
define_panicking_intrinsics!("`u64` division/modulo should not be used", {
    __udivdi3,
    __umoddi3,
});

// NOTE: if you are adding a new intrinsic here, you should also add it to
// `redirect-intrinsics` in `rust/Makefile`.
