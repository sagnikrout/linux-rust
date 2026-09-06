//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/fsm.h
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
// Define this to get debugging messages.
//
pub const FSM_DEBUG: c_int = 0;
//
// Define this to get debugging massages for
// timer handling.
//
pub const FSM_TIMER_DEBUG: c_int = 0;
//
// Define these to record a history of
// Events/Statechanges and print it if a
// action_function is not found.
//
pub const FSM_DEBUG_HISTORY: c_int = 0;
pub const FSM_HISTORY_SIZE: c_int = 40;
//
// Definition of an action function, called by a FSM
//
extern "C" {
    pub fn void(: *mut *mut fsm_function_t)(struct fsm_instance_t, _arg: c_int, : *mut c_void) -> typedef;
}
//
// Internal jump table for a FSM
//

//
// Element of State/Event history used for debugging.
//

//
// Representation of a FSM
//

//
// Description of a state-event combination
//
// Description of a FSM Timer.
//
// init_fsm - Creates a finite state machine
// @name: Name of this instance for logging purposes
// @state_names: Array of names for all states for logging purposes
// @event_names: Array of names for all events for logging purposes
// @nr_states: Number of states for this instance
// @nr_events: Number of events for this instance
// @tmpl: Pointer to fsm_node array describing this FSM
// @tmpl_len: Number of entries in the tmpl array
// @order: GFP flags for memory allocation (e.g. GFP_KERNEL)
//
// Allocates and initializes a finite state machine instance with the
// specified states, events, and transition table.
//
// Return: Pointer to initialized FSM instance, or NULL on failure
//
// kfree_fsm - Releases a finite state machine
// @fi: Pointer to FSM instance, previously created with init_fsm()
//
// Frees all memory associated with the FSM instance.
//
extern "C" {
    pub fn kfree_fsm(fi: *mut fsm_instance);
}

extern "C" {
    pub fn fsm_print_history(fi: *mut fsm_instance);
}
extern "C" {
    pub fn fsm_record_history(fi: *mut fsm_instance, state: c_int, event: c_int);
}

//
// fsm_event - Emits an event to a finite state machine
// @fi: Pointer to FSM which should receive the event
// @event: The event to be delivered
// @arg: Generic argument, passed to the action function
//
// If an action function is defined for the current state/event
// combination, that function is called with the provided arguments.
//
// Return:
// * 0 - Success, action function was called
// * 1 - State/event out of range, or no action function defined
//

//
// fsm_newstate - Modifies the state of a finite state machine
// @fi: Pointer to FSM
// @newstate: The new state for this FSM
//
// This does not trigger an event or call an action function.
// Wakes up any processes waiting on the FSM's wait queue.
//

//
// fsm_getstate - Retrieves the current state of a finite state machine
// @fi: Pointer to FSM
//
// Return: Current state number
//
extern "C" {
    pub fn atomic_read(_arg: &fi->state) -> return;
}
//
// fsm_getstate_str - Retrieves the name of the current FSM state
// @fi: Pointer to FSM
//
// Return: State name string, or "Invalid" if state is out of range
//
// fsm_settimer - Initializes a timer for a finite state machine
// @fi: Pointer to FSM
// @this: The timer to be initialized
//
// Prepares an fsm_timer for usage with fsm_addtimer().
//
extern "C" {
    pub fn fsm_settimer(fi: *mut fsm_instance, this: *mut fsm_timer);
}
//
// fsm_deltimer - Clears a pending timer of an FSM instance
// @timer: The timer to clear
//
// Stops and removes the timer. Safe to call on an inactive timer.
//
extern "C" {
    pub fn fsm_deltimer(timer: *mut fsm_timer);
}
//
// fsm_addtimer - Adds and starts a timer for an FSM instance
// @timer: The timer to be added (timer->fi must point to the FSM instance)
// @millisec: Duration in milliseconds after which the timer expires
// @event: Event to trigger when timer expires
// @arg: Generic argument provided to the event handler
//
// Starts a timer that will trigger the specified event after the given
// duration. The timer must have been initialized with fsm_settimer().
//
// Return: Always returns 0
//
extern "C" {
    pub fn fsm_addtimer(timer: *mut fsm_timer, millisec: c_int, event: c_int, arg: *mut c_void) -> c_int;
}
//
// fsm_modtimer - Modifies a timer of a finite state machine
// @timer: The timer to modify
// @millisec: New duration in milliseconds after which the timer expires
// @event: Event to trigger when timer expires
// @arg: Generic argument provided to the event handler
//
// Stops the existing timer and restarts it with new parameters.
//
extern "C" {
    pub fn fsm_modtimer(timer: *mut fsm_timer, millisec: c_int, event: c_int, arg: *mut c_void);
}
