//! ## Exercise 2: Basic principles
//! This task introduces different methods of task IPC & control: timing, events and messages.
//!
//! ### Exercise 2.1
//! The user has to create a ticker and loop over it. Inside the loop the user needs to wait
//! on the ticker and call an extern function afterwards.
//!
//! After 5 calls in the correct time frame, a flag will be returned using a `Result`.
//!
//! ### Exercise 2.2
//! The user has to obtain an handle to a hidden task using the given predefined task name and
//! NameServer; following, the user needs to send an event to the task which releases the flag
//! in the defmt output.
//!
//! ### Exercise 2.3
//! In addition to (2.2) the hidden task also sends a message containing the second flag. This
//! requires the participant to understand how to receive messages. The flag must be decoded
//! and printed.
//!
//! ### Exercise 2.4
//! The participant must send a message to the hidden task with predefined contents (i.e., the
//! secret flag from 2.3). The hidden task will respond with the secret flag.
//!
//! This exercise requires the participant to understand how to send a message.
#![no_std]

use veecle_pxros::pxros::name_server::TaskName;

/// This is pxros-specific way on how to identify tasks at runtime via a name.
///
/// In this case the name "2" can be used to query the runtime ID of
/// the task via [veecle_pxros::pxros::name_server].
const VALIDATION_TASK_NAME: TaskName = TaskName::new(2);
/// Task's name for exercise 2.3
const RECEIVE_TASK_NAME: TaskName = TaskName::new(3);
/// Task's name for exercise 2.4
const SALT_TASK_NAME: TaskName = TaskName::new(5);

bitflags::bitflags! {
    /// List of **private** events used only by takes to create
    /// tickers, for instance.
    #[derive(Copy, Clone)]
    pub struct MyEvents: u32 {
        /// The ticker event to use for exercise 1.
        const TickerEx1 = 0b0001_0000;
        /// The ticker event to use for exercise 2.
        const TickerEx2 = 0b0010_0000;
        /// The ticker event to use for exercise 3.
        const TickerEx3 = 0b0100_0000;
        /// The ticker event to use for exercise 4.
        const TickerEx4 = 0b1000_0000;
    }

    /// List of **public** events used to communicate with the secret flag
    /// task.
    #[derive(Copy, Clone)]
    pub struct FlagEvents: u32 {
        const PrintSecretFlag   = 0b0000_0001;
    }
}

mod backend;
mod ex2_1;
mod ex2_2;
mod ex2_3;
mod ex2_4;
