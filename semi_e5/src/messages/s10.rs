// Copyright © 2024 Nathaniel Hardesty
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! # STREAM 10: TERMINAL SERVICES
//! **Based on SEMI E5§10.14**
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]s which provide multi-line and single-line terminal display
//! services.
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]: crate::Message

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S10F0
///
/// **Abort Transaction**
///
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Used in lieu of an expected reply to abort a transaction.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// Header only.
pub struct Abort;
message_headeronly!{Abort, false, 10, 0}

/// ## S10F1
///
/// **Terminal Request**
///
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY EXPECTED**
///
/// ---------------------------------------------------------------------------
///
/// Request from equipment to display a message on a terminal.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 2
///    1. [TID]
///    2. [TEXT]
///
/// [TID]:  TerminalID
/// [TEXT]: Text
pub struct TerminalRequest(pub (TerminalID, Text));
message_data!{TerminalRequest, true, 10, 1}

/// ## S10F2
///
/// **Terminal Acknowledge**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Acknowledge terminal display request.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [ACKC10]
///
/// [ACKC10]: AcknowledgeCode10
pub struct TerminalAcknowledge(pub AcknowledgeCode10);
message_data!{TerminalAcknowledge, false, 10, 2}

/// ## S10F3
///
/// **Terminal Display, Single**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY EXPECTED**
///
/// ---------------------------------------------------------------------------
///
/// Request from host to display a message on a single terminal.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 2
///    1. [TID]
///    2. [TEXT]
///
/// [TID]:  TerminalID
/// [TEXT]: Text
pub struct TerminalDisplaySingle(pub (TerminalID, Text));
message_data!{TerminalDisplaySingle, true, 10, 3}

/// ## S10F4
///
/// **Terminal Display, Single - Acknowledge**
///
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Acknowledge single terminal display request.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [ACKC10]
///
/// [ACKC10]: AcknowledgeCode10
pub struct TerminalDisplaySingleAcknowledge(pub AcknowledgeCode10);
message_data!{TerminalDisplaySingleAcknowledge, false, 10, 4}
