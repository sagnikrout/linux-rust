//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/annotate.h
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

pub const ANNOTATION__IPC_WIDTH: c_int = 6;
pub const ANNOTATION__CYCLES_WIDTH: c_int = 6;
pub const ANNOTATION__MINMAX_CYCLES_WIDTH: c_int = 19;
pub const ANNOTATION__AVG_IPC_WIDTH: c_int = 36;
pub const ANNOTATION__BR_CNTR_WIDTH: c_int = 30;
pub const ANNOTATION_DUMMY_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_disassembler {
    PERF_DISASM_UNKNOWN = 0,
    PERF_DISASM_LLVM,
    PERF_DISASM_CAPSTONE,
    PERF_DISASM_OBJDUMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotation_options {
    pub offset_level: u8,
    pub disassemblers: [u8; MAX_DISASSEMBLERS],
    pub disassembler_used: u8,
    pub min_pcnt: c_int,
    pub max_lines: c_int,
    pub context: c_int,
    pub objdump_path: *mut c_char,
    pub disassembler_style: *mut c_char,
    pub prefix: *const c_char,
    pub prefix_strip: *const c_char,
    pub percent_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_hist_entry {
    pub nr_samples: u64,
    pub period: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotation_data {
    pub percent: [double; PERCENT_MAX],
    pub percent_sum: double,
    pub he: sym_hist_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cycles_info {
    pub ipc: float,
    pub avg: u64,
    pub max: u64,
    pub min: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotation_line {
    pub node: list_head,
    pub rb_node: rb_node,
    pub offset: i64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
    pub path: *mut c_char,
    pub cycles: *mut cycles_info,
    pub num_aggr: c_int,
    pub br_cntr_nr: c_int,
    pub br_cntr: *mut u64,
    pub evsel: *mut evsel,
    pub jump_sources: c_int,
    pub idx: u32,
    pub idx_asm: c_int,
    pub data_nr: c_int,
    pub data: [annotation_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disasm_line {
    pub ins: ins,
    pub ops: ins_operands,
    pub bytes: [u8; 4],
    pub raw_insn: u32,
    pub raw: },
// This needs to be at the end.
    pub al: annotation_line,
}

extern "C" {
    pub fn annotation_line__add(al: *mut annotation_line, head: *mut list_head);
}
//
// Is this offset in the same function as the line it is used?
// asm functions jump to other functions, for instance.
//
// Can we draw an arrow from the jump to its target, for instance? I.e.
// is the jump and its target in the same function?
//
extern "C" {
    pub fn disasm_line__is_valid_local_jump(dl: *mut disasm_line, sym: *mut symbol) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotation_write_ops {
    pub change_color: bool first_line, current_entry,,
    pub width: c_int,
    pub obj: *mut c_void,
    pub color): *mut *mut *mut int (set_color)(void obj, int,
    pub current): *mut *mut *mut void (set_percent_color)(void obj, double percent, bool,
    pub current): *mut *mut *mut int (set_jumps_percent_color)(void obj, int nr, bool,
    pub ...): *const *const *const *const void (printf)(void obj, char fmt,,
    pub graph): *mut *mut *mut void (write_graph)(void obj, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotation_print_data {
    pub he: *mut hist_entry,
    pub evsel: *mut evsel,
    pub arch: *const arch,
    pub dbg: *mut debuginfo,
// save data type info keyed by al->offset
    pub type_hash: *mut hashmap,
// It'll be set in hist_entry__annotate_printf()
    pub addr_fmt_width: c_int,
}

extern "C" {
    pub fn disasm__fprintf(head: *mut list_head, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel);
}
//
// struct sym_hist - symbol histogram information for an event
//
// @nr_samples: Total number of samples.
// @period: Sum of sample periods.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_hist {
    pub nr_samples: u64,
    pub period: u64,
}

//
// struct cyc_hist - (CPU) cycle histogram for a basic block
//
// @start: Start address of current block (if known).
// @cycles: Sum of cycles for the longest basic block.
// @cycles_aggr: Total cycles for this address.
// @cycles_max: Max cycles for this address.
// @cycles_min: Min cycles for this address.
// @cycles_spark: History of cycles for the longest basic block.
// @num: Number of samples for the longest basic block.
// @num_aggr: Total number of samples for this address.
// @have_start: Whether the current branch info has a start address.
// @reset: Number of resets due to a different start address.
//
// If sample has branch_stack and cycles info, it can construct basic blocks
// between two adjacent branches.  It'd have start and end addresses but
// sometimes the start address may not be available.  So the cycles are
// accounted at the end address.  If multiple basic blocks end at the same
// address, it will take the longest one.
//
// The @start, @cycles, @cycles_spark and @num fields are used for the longest
// block only.  Other fields are used for all cases.
//
// See __symbol__account_cycles().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyc_hist {
    pub start: u64,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub cycles_max: u64,
    pub cycles_min: u64,
    pub cycles_spark: [i64; NUM_SPARKS],
    pub num: u32,
    pub num_aggr: u32,
    pub have_start: u8,
// 1 byte padding
    pub reset: u16,
}

//
// struct annotated_source - symbols with hits have this attached as in annotation
//
// @source: List head for annotated_line (embeded in disasm_line).
// @histograms: Array of symbol histograms per event to maintain the total number
// of samples and period.
// @nr_histograms: This may not be the same as evsel->evlist->core.nr_entries if
// we have more than a group in a evlist, where we will want
// to see each group separately, that is why symbol__annotate2()
// sets src->nr_histograms to evsel->nr_members.
// @samples: Hash map of sym_hist_entry.  Keyed by event index and offset in symbol.
// @nr_events: Number of events in the current output.
// @nr_entries: Number of annotated_line in the source list.
// @nr_asm_entries: Number of annotated_line with actual asm instruction in the
// source list.
// @max_jump_sources: Maximum number of jump instructions targeting to the same
// instruction.
// @widths: Precalculated width of each column in the TUI output.
//
// disasm_lines are allocated, percentages calculated and all sorted by percentage
// when the annotation is about to be presented, so the percentages are for
// one of the entries in the histogram array, i.e. for the event/counter being
// presented. It is deallocated right after symbol__{tui,tty,etc}_annotate
// returns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_source {
    pub source: list_head,
    pub histograms: *mut sym_hist,
    pub samples: *mut hashmap,
    pub nr_histograms: c_int,
    pub nr_events: c_int,
    pub nr_entries: c_int,
    pub nr_asm_entries: c_int,
    pub max_jump_sources: c_int,
    pub tried_source: bool,
    pub start: u64,
    pub addr: u8,
    pub jumps: u8,
    pub target: u8,
    pub min_addr: u8,
    pub max_addr: u8,
    pub max_ins_name: u8,
    pub max_line_len: u16,
    pub widths: },
}

// A branch counter once saturated

//
// struct annotated_branch - basic block and IPC information for a symbol.
//
// @hit_cycles: Total executed cycles.
// @hit_insn: Total number of instructions executed.
// @total_insn: Number of instructions in the function.
// @cover_insn: Number of distinct, actually executed instructions.
// @cycles_hist: Array of cyc_hist for each instruction.
// @max_coverage: Maximum number of covered basic block (used for block-range).
// @br_cntr: Array of the occurrences of events (branch counters) during a block.
//
// This struct is used by two different codes when the sample has branch stack
// and cycles information.  annotation__compute_ipc() calculates average IPC
// using @hit_insn / @hit_cycles.  The actual coverage can be calculated using
// @cover_insn / @total_insn.  The @cycles_hist can give IPC for each (longest)
// basic block ends at the given address.
// process_basic_block() calculates coverage of instructions (or basic blocks)
// in the function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_branch {
    pub hit_cycles: u64,
    pub hit_insn: u64,
    pub total_insn: c_uint,
    pub cover_insn: c_uint,
    pub cycles_hist: *mut cyc_hist,
    pub max_coverage: u64,
    pub br_cntr: *mut u64,
}

extern "C" {
    pub fn annotation__exit(notes: *mut annotation);
}
extern "C" {
    pub fn annotation__lock(EXCLUSIVE_LOCK_FUNCTION(*notes: *mut *mut annotation notes));
}
extern "C" {
    pub fn annotation__unlock(UNLOCK_FUNCTION(*notes: *mut *mut annotation notes));
}
extern "C" {
    pub fn annotation__trylock(EXCLUSIVE_TRYLOCK_FUNCTION(true: *mut *mut annotation notes), _arg: *mut notes) -> bool;
}
extern "C" {
    pub fn annotation__update_column_widths(notes: *mut annotation);
}
extern "C" {
    pub fn annotation__toggle_full_addr(notes: *mut annotation, ms: *mut map_symbol);
}
extern "C" {
    pub fn annotated_source__histogram(_arg: notes->src, _arg: evsel) -> return;
}
extern "C" {
    pub fn addr_map_symbol__inc_samples(ams: *mut addr_map_symbol, sample: *mut perf_sample) -> c_int;
}
extern "C" {
    pub fn hist_entry__inc_addr_samples(he: *mut hist_entry, sample: *mut perf_sample, addr: u64) -> c_int;
}
extern "C" {
    pub fn symbol__annotate_zero_histograms(sym: *mut symbol);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_disassemble_errno {
    SYMBOL_ANNOTATE_ERRNO__SUCCESS		= 0,

//
// Choose an arbitrary negative big number not to clash with standard
// errno since SUS requires the errno has distinct positive values.
// See 'Issue 6' in the link below.
//
// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
//
    __SYMBOL_ANNOTATE_ERRNO__START		= -10000,

    SYMBOL_ANNOTATE_ERRNO__NO_VMLINUX	= __SYMBOL_ANNOTATE_ERRNO__START,
    SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF,
    SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING,
    SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP,
    SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE,
    SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF,
    SYMBOL_ANNOTATE_ERRNO__COULDNT_DETERMINE_FILE_TYPE,

    __SYMBOL_ANNOTATE_ERRNO__END,
}

extern "C" {
    pub fn symbol__strerror_disassemble(ms: *mut map_symbol, errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int;
}
extern "C" {
    pub fn symbol__annotate_zero_histogram(sym: *mut symbol, evsel: *mut evsel);
}
extern "C" {
    pub fn symbol__annotate_decay_histogram(sym: *mut symbol, evsel: *mut evsel);
}
extern "C" {
    pub fn annotated_source__purge(as: *mut annotated_source);
}
extern "C" {
    pub fn ui__has_annotation() -> bool;
}
extern "C" {
    pub fn hist_entry__annotate_printf(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn hist_entry__tty_annotate(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn hist_entry__tty_annotate2(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn annotation_options__init();
}
extern "C" {
    pub fn annotation_options__exit();
}
extern "C" {
    pub fn annotation_config__init();
}
extern "C" {
    pub fn annotate_check_args() -> c_int;
}
//
// struct annotated_op_loc - Location info of instruction operand
// @reg1: First register in the operand
// @reg2: Second register in the operand
// @offset: Memory access offset in the operand
// @segment: Segment selector register
// @mem_ref: Whether the operand accesses memory
// @multi_regs: Whether the second register is used
// @imm: Whether the operand is an immediate value (in offset)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_op_loc {
    pub reg1: c_int,
    pub reg2: c_int,
    pub offset: c_int,
    pub segment: u8,
    pub mem_ref: bool,
    pub multi_regs: bool,
    pub imm: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum annotated_insn_ops {
    INSN_OP_SOURCE = 0,
    INSN_OP_TARGET = 1,

    INSN_OP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum annotated_x86_segment {
    INSN_SEG_NONE = 0,

    INSN_SEG_X86_CS,
    INSN_SEG_X86_DS,
    INSN_SEG_X86_ES,
    INSN_SEG_X86_FS,
    INSN_SEG_X86_GS,
    INSN_SEG_X86_SS,
}

//
// struct annotated_insn_loc - Location info of instruction
// @ops: Array of location info for source and target operands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_insn_loc {
    pub ops: [annotated_op_loc; INSN_OP_MAX],
}

// Get detailed location info in the instruction
// Returns a data type from the sample instruction (if any)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_item_stat {
    pub list: list_head,
    pub name: *mut c_char,
    pub good: c_int,
    pub bad: c_int,
}

// Calculate PC-relative address
//
// struct annotated_basic_block - Basic block of instructions
// @list: List node
// @begin: start instruction in the block
// @end: end instruction in the block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_basic_block {
    pub list: list_head,
    pub begin: *mut disasm_line,
    pub end: *mut disasm_line,
}

// Get a list of basic blocks from src to dst addresses
extern "C" {
    pub fn debuginfo_cache__delete();
}
extern "C" {
    pub fn annotation_br_cntr_abbr_list(str: *mut c_char, evsel: *mut evsel, header: bool) -> c_int;
}
extern "C" {
    pub fn map_symbol__get_arch(ms: *mut map_symbol, parch: *const arch) -> c_int;
}
