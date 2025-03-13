// Copyright © 2024-2025 Nathaniel Hardesty
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the “Software”), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! # HIGH-SPEED SECS MESSAGE SERVICES (HSMS)
//! 
//! Copyright © 2024-2025 Nathaniel Hardesty, Licensed under the MIT License
//! 
//! This software is created by a third-party and not endorsed or supported by
//! SEMI.
//! 
//! The codebase will be updated to reflect more up-to-date SEMI standards
//! if/when they can be acquired for this purpose.
//! 
//! ----------------------------------------------------------------------------
//! 
//! **Based on:**
//! - **[SEMI E37]-1109**
//! - **[SEMI E37].1-0702**
//! 
//! ----------------------------------------------------------------------------
//! 
//! HSMS is a protocol designed to facilitate the reliable transmission of
//! messages between semiconductor equipment over TCP/IP.
//! 
//! Most commonly, exchanged messages are encoded with the [SECS-II]
//! ([SEMI E5]) protocol.
//! 
//! ----------------------------------------------------------------------------
//! 
//! For ease of programming and extension, the functionality of the protocol
//! has been divided into a few subsets: 
//! 
//! - [Primitive Services] - Manages the TCP/IP connection and the sending of
//!   messages with proper headers.
//! - [Generic Services] - Manages the sending of messages of particular types
//!   and at particular times as allowed by the protocol. 
//! - [Single Selected Session Services] - Manages the restriction of the
//!   protocol to scenarios involving a single host/equipment pair in
//!   communication.
//! 
//! ----------------------------------------------------------------------------
//! 
//! ## TODO
//! 
//! - [Generic Services] - "Simultaneous Select Procedure"
//! - [Generic Services] - "Simultaneous Deselect Procedure"
//! 
//! [SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
//! [SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
//! [SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
//! [SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
//! 
//! [SECS-II]:                          semi_e5
//! [Primitive Services]:               primitive
//! [Generic Services]:                 generic
//! [Single Selected Session Services]: single

pub mod primitive;
pub mod generic;
pub mod single;

/// ## HSMS ERROR
/// 
/// Defines the set of possible reasons a function in this library may fail.
#[derive(Debug)]
pub enum Error {
  /// ### I/O ERROR
  /// 
  /// An error from the standard library has been encountered in attempting to
  /// perform the function.
  IoError(std::io::Error),

  /// ### INVALID RESPONSE
  /// 
  /// A response to a sent message was of a form which is not correlated to it,
  /// or is otherwise malformed in an unhandleable manner.
  InvalidResponse,

  /// ### TIMED OUT
  /// 
  /// The function has timed out waiting for some operation to complete or for a
  /// response from the other end of the connection.
  TimedOut,

  /// ### NOT CONNECTED
  /// 
  /// The function cannot be performed because no connection has been made.
  NotConnected,

  /// ### ALREADY CONNECTED
  /// 
  /// The function cannot be performed because it is intended to form a
  /// connection where one already exists.
  AlreadyConnected,

  /// ### DISCONNECTED
  /// 
  /// During the course of attempting to perform the function, a disconnection
  /// has occurred.
  Disconnected,

  /// ### NOT SELECTED
  /// 
  /// The function cannot be performed because it is not valid before
  /// handshaking establishing a selected state has occurred.
  NotSelected,

  /// ### MESSAGE REJECTED
  /// 
  /// The client on the other end of the connection received the message and
  /// rejected it on the basis that it could not understand the message.
  MessageRejected(u8, u8),

  /// ### PROCEDURE REJECTED
  /// 
  /// The client on the other end of the connection received the message and
  /// rejected the action which was requested to be performed by the message.
  /// 
  /// The single-byte code indicating the reason for message rejection is
  /// provided.
  ProcedureRejected(u8),

  /// ### TRANSACTION OPEN
  /// 
  /// The function cannot be performed because there is an outstanding
  /// transaction which conflicts with the message asked to be sent.
  TransactionOpen,
}

/// ## PRESENTATION TYPE
/// **Based on SEMI E37-1109§8.2.6.4**
/// 
/// Defines the Presentation Layer content of exchanged information.
/// 
/// Values 1-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved and may not be used.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresentationType {
  /// ### SECS II ENCODING
  /// 
  /// Denotes an [HSMS Message], which is often a [SECS-II] formatted
  /// [Data Message].
  /// 
  /// [SECS-II]:      semi_e5
  /// [HSMS Message]: generic::Message
  /// [Data Message]: generic::MessageContents::DataMessage
  SecsII = 0,
}
