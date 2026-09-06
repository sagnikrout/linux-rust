//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/vmlinux.lds.h
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


//
// Helper macros to support writing architecture specific
// linker scripts.
//
// A minimal linker scripts has following content:
// [This is a sample, architectures may have special requirements]
//
// OUTPUT_FORMAT(...)
// OUTPUT_ARCH(...)
// ENTRY(...)
// SECTIONS
// {
// . = START;
// __init_begin = .;
// HEAD_TEXT_SECTION
// INIT_TEXT_SECTION(PAGE_SIZE)
// INIT_DATA_SECTION(...)
// PERCPU_SECTION(CACHELINE_SIZE)
// __init_end = .;
//
// _stext = .;
// TEXT_SECTION = 0
// _etext = .;
//
// _sdata = .;
// RO_DATA(PAGE_SIZE)
// RW_DATA(...)
// _edata = .;
//
// EXCEPTION_TABLE(...)
//
// BSS_SECTION(0, 0, 0)
// _end = .;
//
// STABS_DEBUG
// DWARF_DEBUG
// ELF_DETAILS
//
// DISCARDS		// must be the last
// }
//
// [__init_begin, __init_end] is the init section that may be freed after init
// // __init_begin and __init_end should be page aligned, so that we can
// // free the whole .init memory
// [_stext, _etext] is the text section
// [_sdata, _edata] is the data section
//
// Some of the included output section have their own set of constants.
// Examples are: [__initramfs_start, __initramfs_end] for initramfs and
// [__nosave_begin, __nosave_end] for the nosave data
//

pub const LOAD_OFFSET: c_int = 0;

//
// Only some architectures want to have the .notes segment visible in
// a separate PT_NOTE ELF Program Header. When this happens, it needs
// to be visible in both the kernel text's PT_LOAD and the PT_NOTE
// Program Headers. In this case, though, the PT_LOAD needs to be made
// the default again so that all the following sections don't also end
// up in the PT_NOTE Program Header.
//

// Macro flag: #define NOTES_HEADERS
// Macro flag: #define NOTES_HEADERS_RESTORE

//
// Some architectures have non-executable read-only exception tables.
// They can be added to the RO_DATA segment by specifying their desired
// alignment.
//

// Macro flag: #define RO_EXCEPTION_TABLE

// Align . function alignment.

//
// Support -ffunction-sections by matching .text and .text.*,
// but exclude '.text..*', .text.startup[.*], and .text.exit[.*].
//
// .text.startup and .text.startup.* are matched later by INIT_TEXT, and
// .text.exit and .text.exit.* are matched later by EXIT_TEXT, so they must be
// explicitly excluded here.
//
// Other .text.* sections that are typically grouped separately, such as
// .text.unlikely or .text.hot, must be matched explicitly before using
// TEXT_MAIN.
//
// NOTE: builds *with* and *without* -ffunction-sections are both supported by
// this single macro.  Even with -ffunction-sections, there may be some objects
// NOT compiled with the flag due to the use of a specific Makefile override
// like cflags-y or AUTOFDO_PROFILE_foo.o.  So this single catchall rule is
// needed to support mixed object builds.
//
// One implication is that functions named startup(), exit(), split(),
// unlikely(), hot(), and unknown() are not allowed in the kernel due to the
// ambiguity of their section names with -ffunction-sections.  For example,
// .text.startup could be __attribute__((constructor)) code in a *non
// ffunction-sections object, which should be placed in .init.text; or it could
// be an actual function named startup() in an ffunction-sections object, which
// should be placed in .text.  The build will detect and complain about any such
// ambiguously named functions.
//

//
// Support -fdata-sections by matching .data, .data.*, and others,
// but exclude '.data..*'.
//

//
// GCC 4.5 and later have a 32 bytes section alignment for structures.
// Except GCC 4.9, that feels the need to align on 64 bytes.
//
pub const STRUCT_ALIGNMENT: c_int = 32;

//
// The order of the sched class addresses are important, as they are
// used to determine the order of the priority of each sched class in
// relation to each other.
//

// (__stop_sched_class)			\
// (__dl_sched_class)			\
// (__rt_sched_class)			\
// (__fair_sched_class)			\
// (__ext_sched_class)			\
// (__idle_sched_class)			\
// The actual configuration determine if the init/exit sections
// are handled as text/data or they can be discarded (which
// often happens at runtime)
//

// Macro flag: #define PATCHABLE_DISCARDS

// Macro flag: #define KEEP_PATCHABLE

//
// Simply points to ftrace_stub, but with the proper protocol.
// Defined by the linker script in linux/vmlinux.lds.h
//

// Macro flag: #define FTRACE_STUB_HACK

//
// The ftrace call sites are logged to a section whose name depends on the
// compiler option used. A given kernel image will only use one, AKA
// FTRACE_CALLSITE_SECTION. We capture all of them here to avoid header
// dependencies for FTRACE_CALLSITE_SECTION's definition.
//
// ftrace_ops_list_func will be defined as arch_ftrace_ops_list_func
// as some archs will have a different prototype for that function
// but ftrace_ops_list_func() will have a single prototype.
//

// Macro flag: #define LIKELY_PROFILE()

// Macro flag: #define BRANCH_PROFILE()

// Macro flag: #define KPROBE_BLACKLIST()

// Macro flag: #define ERROR_INJECT_WHITELIST()

// Macro flag: #define FTRACE_EVENTS()

// Macro flag: #define TRACE_PRINTKS()
// Macro flag: #define TRACEPOINT_STR()

// Macro flag: #define TRACE_SYSCALLS()

// Macro flag: #define BPF_RAW_TP()

// Macro flag: #define EARLYCON_TABLE()

// Macro flag: #define LSM_TABLE()
// Macro flag: #define EARLY_LSM_TABLE()

// Macro flag: #define _OF_TABLE_0(name)

// Macro flag: #define ACPI_PROBE_TABLE(name)

// Macro flag: #define THERMAL_TABLE(name)

//
// .data section
//

// (.xiptext)							\
// (DATA_MAIN)							\
// (.data..decrypted)						\
// (.ref.data)							\
// (.data..shared_aligned) /* percpu related */			\
// (.data..unlikely)						\
// (.data..once)							\
// (.data..do_once)						\
// (__tracepoints)						\
// implement dynamic printk debug */				\
//
// Data section helpers
//

// (.data..nosave)						\

// (SORT_BY_ALIGNMENT(.data..hot.*))				\

// (.data..page_aligned)						\

// (.data..read_mostly)						\

// (.data..cacheline_aligned)

// Macro flag: #define STATIC_CALL_DATA

//
// Allow architectures to handle ro_after_init data on their
// own by defining an empty RO_AFTER_INIT_DATA.
//

// (.data..ro_after_init)						\

//
// .kcfi_traps contains a list KCFI trap locations.
//

// Macro flag: #define KCFI_TRAPS

//
// Read only Data
//

// (.rodata) *(.rodata.*) *(.data.rel.ro*)		\
// (__tracepoints_strings)/* Tracepoints: strings */	\
// (.rodata1)						\
// PCI quirks */						\
// Kernel symbol table */					\
// Kernel symbol CRC table */					\
// Kernel symbol flags table */					\
// Kernel symbol table: strings */				\
// (__ksymtab_strings)					\
// __*init sections */						\
// (.ref.rodata)						\
// Built-in module parameters. */				\
// Built-in module versions. */					\
//
// Non-instrumentable text section
//

// (.noinstr.text)					\
// (.cpuidle.text)					\

// (.text.split .text.split.[0-9a-zA-Z_]*)		\

// (.text.unlikely .text.unlikely.*)			\

// (.text.hot .text.hot.*)				\
//
// .text section. Map to function alignment to avoid address changes
// during second ld run in second ld pass when generating System.map
//
// TEXT_MAIN here will match symbols with a fixed pattern (for example,
// .text.hot or .text.unlikely).  Match those before TEXT_MAIN to ensure
// they get grouped together.
//
// Also placing .text.hot section at the beginning of a page, this
// would help the TLB performance.
//

// (.text.asan.* .text.tsan.*)				\
// (.text.unknown .text.unknown.*)			\
// (TEXT_MAIN .text.fixup)				\
// (.ref.text)
// sched.text is aling to function alignment to secure we have same
// address even at second ld pass when generating System.map

// (.sched.text)						\
// spinlock.text is aling to function alignment to secure we have same
// address even at second ld pass when generating System.map

// (.spinlock.text)					\

// (.kprobes.text)					\

// (.entry.text)						\

// (.irqentry.text)					\

// (.softirqentry.text)					\

// (.static_call.text)					\
// Section used for early init (in .S files)

//
// Exception table
//

//
// .BTF
//

// (.BTF_ids)						\

// Macro flag: #define BTF

//
// Init task
//

// Macro flag: #define KERNEL_CTORS()

// init and exit section handling

// (.init.data .init.data.*)					\
// (.init.rodata .init.rodata.*)					\

// (.init.text .init.text.*)					\
// (.text.startup .text.startup.*)

// (.exit.data .exit.data.*)					\
// (.fini_array .fini_array.*)					\
// (.dtors .dtors.*)

// (.exit.text)							\
// (.text.exit .text.exit.*)

// (.exitcall.exit)
//
// bss (Block Started by Symbol) - uninitialized data
// zeroed during startup
//

// (.dynsbss)						\
// (SBSS_MAIN)						\
// (.scommon)						\
//
// Allow archectures to redefine BSS_FIRST_SECTIONS to add extra
// sections to the front of bss.
//

// Macro flag: #define BSS_FIRST_SECTIONS

// (.bss..page_aligned)					\
// (.dynbss)						\
// (BSS_MAIN)						\
// (COMMON)						\
//
// DWARF debug sections.
// Symbols in the DWARF debugging sections are relative to
// the beginning of the section so we begin them at 0.
//

// DWARF 1 */						\
// GNU DWARF 1 extensions */				\
// DWARF 1.1 and DWARF 2 */				\
// DWARF 2 */						\
// DWARF 3 */						\
// SGI/MIPS DWARF 2 extensions */			\
// GNU DWARF 2 extensions */				\
// DWARF 4 */						\
// DWARF 5 */						\
// Stabs debugging sections.

// Macro flag: #define KLP_SYMID

// Required sections not related to debugging.

// Macro flag: #define BUG_TABLE

// Macro flag: #define ORC_UNWIND_TABLE

// Built-in firmware blobs

// Macro flag: #define FW_LOADER_BUILT_IN_DATA

// Macro flag: #define TRACEDATA

// Macro flag: #define PRINTK_INDEX

//
// Discard .note.GNU-stack, which is emitted as PROGBITS by the compiler.
// Otherwise, the type of .notes section would become PROGBITS instead of NOTES.
//
// Also, discard .note.gnu.property, otherwise it forces the notes section to
// be 8-byte aligned which causes alignment mismatches with the kernel's custom
// 4-byte aligned notes.
//

// (.note.GNU-stack)					\
// (.note.gnu.property)					\

// Alignment must be consistent with (kunit_suite *) in include/kunit/test.h

// Alignment must be consistent with (kunit_suite *) in include/kunit/test.h

// Macro flag: #define INIT_RAM_FS

//
// Memory encryption operates on a page basis. Since we need to clear
// the memory encryption mask for this section, it needs to be aligned
// on a page boundary and be a page-size multiple in length.
//
// Note: We use a separate section so that only this section gets
// decrypted to avoid exposing more than we wish.
//

// (.data..percpu..decrypted)					\

// Macro flag: #define PERCPU_DECRYPTED_SECTION

// Macro flag: #define PROPELLER_DATA

//
// Default discarded sections.
//
// Some archs want to discard exit text/data at runtime rather than
// link time due to cross-section references such as alt instructions,
// bug table, eh_frame, etc.  DISCARDS must be the last of output
// section definitions so that such archs put those in earlier section
// definitions.
//

// Macro flag: #define EXIT_DISCARDS

//
// Clang's -fprofile-arcs, -fsanitize=kernel-address, and
// -fsanitize=thread produce unwanted sections (.eh_frame
// and .init_array.*), but CONFIG_CONSTRUCTORS wants to
// keep any .init_array.* sections.
// https://llvm.org/pr46478
//

// Macro flag: #define DISCARD_EH_FRAME

// (.init_array) *(.init_array.*)					\

// (.discard)							\
// (.discard.*)							\
// (.export_symbol)						\
// (.no_trim_symbol)						\
// ld.bfd warns about .gnu.version* even when not emitted */	\
// (.gnu.version*)						\
// (__tracepoint_check)						\

//
// PERCPU_INPUT - the percpu input sections
// @cacheline: cacheline size
//
// The core percpu section names and core symbols which do not rely
// directly upon load addresses.
//
// @cacheline is used to align subsections to avoid false cacheline
// sharing between subsections for different purposes.
//

// (.data..percpu..page_aligned)					\
// (SORT_BY_ALIGNMENT(.data..percpu..hot.*))			\
// (.data..percpu..read_mostly)					\
// (.data..percpu)						\
// (.data..percpu..shared_aligned)				\
//
// PERCPU_SECTION - define output section for percpu area
// @cacheline: cacheline size
//
// Macro which expands to output section for percpu area.
//
// @cacheline is used to align subsections to avoid false cacheline
// sharing between subsections for different purposes.
//

//
// Definition of the high level *_SECTION macros
// They will fit only a subset of the architectures
//
// Writeable data.
// All sections are combined in a single .data section.
// The sections following CONSTRUCTORS are arranged so their
// typical alignment matches.
// A cacheline is typical/always less than a PAGE_SIZE so
// the sections that has this restriction (or similar)
// is located before the ones requiring PAGE_SIZE alignment.
// NOSAVE_DATA starts and ends with a PAGE_SIZE alignment which
// matches the requirement of PAGE_ALIGNED_DATA.
//
// use 0 as page_align if page_aligned data is not used

