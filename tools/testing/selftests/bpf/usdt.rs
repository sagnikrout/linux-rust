//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/usdt.h
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


// SPDX-License-Identifier: BSD-2-Clause
//
// This single-header library defines a collection of variadic macros for
// defining and triggering USDTs (User Statically-Defined Tracepoints):
//
// - For USDTs without associated semaphore:
// USDT(group, name, args...)
//
// - For USDTs with implicit (transparent to the user) semaphore:
// USDT_WITH_SEMA(group, name, args...)
// USDT_IS_ACTIVE(group, name)
//
// - For USDTs with explicit (user-defined and provided) semaphore:
// USDT_WITH_EXPLICIT_SEMA(sema, group, name, args...)
// USDT_SEMA_IS_ACTIVE(sema)
//
// all of which emit a NOP instruction into the instruction stream, and so
// have *zero* overhead for the surrounding code. USDTs are identified by
// a combination of `group` and `name` identifiers, which is used by external
// tracing tooling (tracers) for identifying exact USDTs of interest.
//
// USDTs can have an associated (2-byte) activity counter (USDT semaphore),
// automatically maintained by Linux kernel whenever any correctly written
// BPF-based tracer is attached to the USDT. This USDT semaphore can be used
// to check whether there is a need to do any extra data collection and
// processing for a given USDT (if necessary), and otherwise avoid extra work
// for a common case of USDT not being traced ("active").
//
// See documentation for USDT_WITH_SEMA()/USDT_IS_ACTIVE() or
// USDT_WITH_EXPLICIT_SEMA()/USDT_SEMA_IS_ACTIVE() APIs below for details on
// working with USDTs with implicitly or explicitly associated
// USDT semaphores, respectively.
//
// There is also some additional data recorded into an auxiliary note
// section. The data in the note section describes the operands, in terms of
// size and location, used by tracing tooling to know where to find USDT
// arguments. Each location is encoded as an assembler operand string.
// Tracing tools (bpftrace and BPF-based tracers, systemtap, etc) insert
// breakpoints on top of the nop, and decode the location operand-strings,
// like an assembler, to find the values being passed.
//
// The operand strings are selected by the compiler for each operand.
// They are constrained by inline-assembler codes.The default is:
//
// #define USDT_ARG_CONSTRAINT nor
//
// This is a good default if the operands tend to be integral and
// moderate in number (smaller than number of registers). In other
// cases, the compiler may report "'asm' requires impossible reload" or
// similar. In this case, consider simplifying the macro call (fewer
// and simpler operands), reduce optimization, or override the default
// constraints string via:
//
// #define USDT_ARG_CONSTRAINT g
// #include <usdt.h>
//
// For some historical description of USDT v3 format (the one used by this
// library and generally recognized and assumed by BPF-based tracing tools)
// see [0]. The more formal specification can be found at [1]. Additional
// argument constraints information can be found at [2].
//
// Original SystemTap's sys/sdt.h implementation ([3]) was used as a base for
// this USDT library implementation. Current implementation differs *a lot* in
// terms of exposed user API and general usability, which was the main goal
// and focus of the reimplementation work. Nevertheless, underlying recorded
// USDT definitions are fully binary compatible and any USDT-based tooling
// should work equally well with USDTs defined by either SystemTap's or this
// library's USDT implementation.
//
// [0] https://ecos.sourceware.org/ml/systemtap/2010-q3/msg00145.html
// [1] https://sourceware.org/systemtap/wiki/UserSpaceProbeImplementation
// [2] https://gcc.gnu.org/onlinedocs/gcc/Constraints.html
// [3] https://sourceware.org/git/?p=systemtap.git;a=blob;f=includes/sys/sdt.h
//
// Changelog:
//
// 0.1.0
// -----
// - Initial release
//
pub const USDT_MAJOR_VERSION: c_int = 0;
pub const USDT_MINOR_VERSION: c_int = 1;
pub const USDT_PATCH_VERSION: c_int = 0;
// C++20 and C23 added __VA_OPT__ as a standard replacement for non-standard `##__VA_ARGS__` extension

pub const __usdt_va_opt: c_int = 1;

//
// Trigger USDT with `group`:`name` identifier and pass through `args` as its
// arguments. Zero arguments are acceptable as well. No USDT semaphore is
// associated with this USDT.
//
// Such "semaphoreless" USDTs are commonly used when there is no extra data
// collection or processing needed to collect and prepare USDT arguments and
// they are just available in the surrounding code. USDT() macro will just
// record their locations in CPU registers or in memory for tracing tooling to
// be able to access them, if necessary.
//

//
// Trigger USDT with `group`:`name` identifier and pass through `args` as its
// arguments. Zero arguments are acceptable as well. USDT also get an
// implicitly-defined associated USDT semaphore, which will be "activated" by
// tracing tooling and can be used to check whether USDT is being actively
// observed.
//
// USDTs with semaphore are commonly used when there is a need to perform
// additional data collection and processing to prepare USDT arguments, which
// otherwise might not be necessary for the rest of application logic. In such
// case, USDT semaphore can be used to avoid unnecessary extra work. If USDT
// is not traced (which is presumed to be a common situation), the associated
// USDT semaphore is "inactive", and so there is no need to waste resources to
// prepare USDT arguments. Use USDT_IS_ACTIVE(group, name) to check whether
// USDT is "active".
//
// N.B. There is an inherent (albeit short) gap between checking whether USDT
// is active and triggering corresponding USDT, in which external tracer can
// be attached to an USDT and activate USDT semaphore after the activity check.
// If such a race occurs, tracers might miss one USDT execution. Tracers are
// expected to accommodate such possibility and this is expected to not be
// a problem for applications and tracers.
//
// N.B. Implicit USDT semaphore defined by USDT_WITH_SEMA() is contained
// within a single executable or shared library and is not shared outside
// them. I.e., if you use USDT_WITH_SEMA() with the same USDT group and name
// identifier across executable and shared library, it will work and won't
// conflict, per se, but will define independent USDT semaphores, one for each
// shared library/executable in which USDT_WITH_SEMA(group, name) is used.
// That is, if you attach to this USDT in one shared library (or executable),
// then only USDT semaphore within that shared library (or executable) will be
// updated by the kernel, while other libraries (or executable) will not see
// activated USDT semaphore. In short, it's best to use unique USDT group:name
// identifiers across different shared libraries (and, equivalently, between
// executable and shared library). This is advanced consideration and is
// rarely (if ever) seen in practice, but just to avoid surprises this is
// called out here. (Static libraries become a part of final executable, once
// linked by linker, so the above considerations don't apply to them.)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usdt_sema {
//
// Check if USDT with `group`:`name` identifier is "active" (i.e., whether it
// is attached to by external tracing tooling and is actively observed).
//
// This macro can be used to decide whether any additional and potentially
// expensive data collection or processing should be done to pass extra
// information into the given USDT. It is assumed that USDT is triggered with
// USDT_WITH_SEMA() macro which will implicitly define associated USDT
// semaphore. (If one needs more control over USDT semaphore, see
// USDT_DEFINE_SEMA() and USDT_WITH_EXPLICIT_SEMA() macros below.)
//
// N.B. Such checks are necessarily racy and speculative. Between checking
// whether USDT is active and triggering the USDT itself, tracer can be
// detached with no notification. This race should be extremely rare and worst
// case should result in one-time wasted extra data collection and processing.
//

    pub \: __usdt_asm_name(__usdt_sema_name(group, name));,
    pub \: __usdt_sema_implicit(__usdt_sema_name(group, name));,
    pub \: __usdt_sema_name(group, name).active > 0;,
//
// APIs for working with user-defined explicit USDT semaphores.
//
// This is a less commonly used advanced API for use cases in which user needs
// an explicit control over (potentially shared across multiple USDTs) USDT
// semaphore instance. This can be used when there is a group of logically
// related USDTs that all need extra data collection and processing whenever
// any of a family of related USDTs are "activated" (i.e., traced). In such
// a case, all such related USDTs will be associated with the same shared USDT
// semaphore defined with USDT_DEFINE_SEMA() and the USDTs themselves will be
// triggered with USDT_WITH_EXPLICIT_SEMA() macros, taking an explicit extra
// USDT semaphore identifier as an extra parameter.
//
// Underlying C global variable name for user-defined USDT semaphore with
// `sema` identifier. Could be useful for debugging, but normally shouldn't be
// used explicitly.
//

//
// Define storage for user-defined USDT semaphore `sema`.
//
// Should be used only once in non-header source file to let compiler allocate
// space for the semaphore variable. Just like with any other global variable.
//
// This macro can be used anywhere where global variable declaration is
// allowed. Just like with global variable definitions, there should be only
// one definition of user-defined USDT semaphore with given `sema` identifier,
// otherwise compiler or linker will complain about duplicate variable
// definition.
//
// For C++, it is allowed to use USDT_DEFINE_SEMA() both in global namespace
// and inside namespaces (including nested namespaces). Just make sure that
// USDT_DECLARE_SEMA() is placed within the namespace where this semaphore is
// referenced, or any of its parent namespaces, so the C++ language-level
// identifier is visible to the code that needs to reference the semaphore.
// At the lowest layer, USDT semaphores have global naming and visibility
// (they have a corresponding `__usdt_sema_<name>` symbol, which can be linked
// against from C or C++ code, if necessary). To keep it simple, putting
// USDT_DECLARE_SEMA() declarations into global namespaces is the simplest
// no-brainer solution. All these aspects are irrelevant for plain C, because
// C doesn't have namespaces and everything is always in the global namespace.
//
// N.B. Due to USDT metadata being recorded in non-allocatable ELF note
// section, it has limitations when it comes to relocations, which, in
// practice, means that it's not possible to correctly share USDT semaphores
// between main executable and shared libraries, or even between multiple
// shared libraries. USDT semaphore has to be contained to individual shared
// library or executable to avoid unpleasant surprises with half-working USDT
// semaphores. We enforce this by marking semaphore ELF symbols as having
// a hidden visibility. This is quite an advanced use case and consideration
// and for most users this should have no consequences whatsoever.
//

//
// Declare extern reference to user-defined USDT semaphore `sema`.
//
// Refers to a variable defined in another compilation unit by
// USDT_DEFINE_SEMA() and allows to use the same USDT semaphore across
// multiple compilation units (i.e., .c and .cpp files).
//
// See USDT_DEFINE_SEMA() notes above for C++ language usage peculiarities.
//

//
// Check if user-defined USDT semaphore `sema` is "active" (i.e., whether it
// is attached to by external tracing tooling and is actively observed).
//
// This macro can be used to decide whether any additional and potentially
// expensive data collection or processing should be done to pass extra
// information into USDT(s) associated with USDT semaphore `sema`.
//
// N.B. Such checks are necessarily racy. Between checking the state of USDT
// semaphore and triggering associated USDT(s), the active tracer might attach
// or detach. This race should be extremely rare and worst case should result
// in one-time missed USDT event or wasted extra data collection and
// processing. USDT-using tracers should be written with this in mind and is
// not a concern of the application defining USDTs with associated semaphore.
//

//
// Invoke USDT specified by `group` and `name` identifiers and associate
// explicitly user-defined semaphore `sema` with it. Pass through `args` as
// USDT arguments. `args` are optional and zero arguments are acceptable.
//
// Semaphore is defined with the help of USDT_DEFINE_SEMA() macro and can be
// checked whether active with USDT_SEMA_IS_ACTIVE().
//

//
// Adjustable implementation aspects
//

//
// Implementation details
//
// USDT name for implicitly-defined USDT semaphore, derived from group:name

// ELF section into which USDT semaphores are put

// "semaphoreless" USDT case

// Macro flag: #define __usdt_sema_none(sema)

// implicitly defined __usdt_sema__group__name semaphore (using weak symbols)

// externally defined semaphore using USDT_DEFINE_SEMA() and passed explicitly by user

    pub (sema)): __asm__ __volatile__ ("" :: "m",

// main USDT definition (nop and .note.stapsdt metadata)

    pub \: );,
//
// NB: gdb PR24541 highlighted an unspecified corner of the sdt.h
// operand note format.
//
// The named register may be a longer or shorter (!) alias for the
// storage where the value in question is found. For example, on
// i386, 64-bit value may be put in register pairs, and a register
// name stored would identify just one of them. Previously, gcc was
// asked to emit the %w[id] (16-bit alias of some registers holding
// operands), even when a wider 32-bit value was used.
//
// Bottom line: the byte-width given before the @ sign governs. If
// there is a mismatch between that width and that of the named
// register, then a sys/sdt.h note consumer may need to employ
// architecture-specific heuristics to figure out where the compiler
// has actually put the complete value.
//

//
// We can't use __builtin_choose_expr() in C++, so fall back to table-based
// signedness determination for known types, utilizing templates magic.
//

    pub }: template<typename T> struct __usdt_t { static bool is_signed = false;,
    pub {}: *mut *mut template<typename A> struct __usdt_t<A[]> : public __usdt_t<A >,
    pub {}: *mut *mut template<typename A, size_t N> struct __usdt_t<A[N]> : public __usdt_t<A >,

    pub \: template<> struct __usdt_t<T> { static bool is_signed = true; };,
    pub \: template<> struct __usdt_t<T> { static bool is_signed = true; };,
    pub \: template<> struct __usdt_t<volatile T> { static bool is_signed = true; };,
    pub }: template<> struct __usdt_t<volatile T> { static bool is_signed = true;,

    pub \: template<> struct __usdt_t<T> { static bool is_signed = (T)-1 < (T)1; };,
    pub \: template<> struct __usdt_t<T> { static bool is_signed = (T)-1 < (T)1; };,
    pub \: template<> struct __usdt_t<volatile T> { static bool is_signed = (T)-1 < (T)1; };,
    pub }: template<> struct __usdt_t<volatile T> { static bool is_signed = (T)-1 < (T)1;,
    pub char): __usdt_def_signed(signed,
    pub long): __usdt_def_signed(long,

