// Copyright © 2024 Nathaniel Hardesty
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

//! # SINGLE SELECTED SESSION SERVICES
//! 
//! Defines the functionality of the [HSMS] protocol persuant to the HSMS Single
//! Selected Session (HSMS-SS) subsidiary protocol.
//! 
//! ----------------------------------------------------------------------------
//! 
//! To use the [Single Selected Session Services]:
//! 
//! - Create a [Client] by providing the [New Client] function with
//!   [Parameter Settings].
//! - Manage the [Connection State] and [Selection State] with the
//!   [Connect Procedure] and [Disconnect Procedure].
//! - Receive [Data Message]s with the hook provided by the
//!   [Connect Procedure].
//! - Send [Data Message]s with the [Data Procedure].
//! - Test connection integrity with the [Linktest Procedure].
//! - Send [Reject.req] messages with the [Reject Procedure].
//! 
//! [HSMS]:                             crate
//! [Single Selected Session Services]: crate::single
//! [Client]:               Client
//! [New Client]:           Client::new
//! [Connect Procedure]:    Client::connect
//! [Disconnect Procedure]: Client::disconnect
//! [Data Procedure]:       Client::data
//! [Linktest Procedure]:   Client::linktest
//! [Reject Procedure]:     Client::reject
//! [Message]:              Message
//! [Message ID]:           MessageID
//! [Message Contents]:     MessageContents
//! [Data Message]:         MessageContents::DataMessage
//! [Linktest.req]:         MessageContents::LinktestRequest
//! [Reject.req]:           MessageContents::RejectRequest
//! [Connection State]:     crate::primitive::ConnectionState
//! [Selection State]:      SelectionState
//! [Parameter Settings]:   ParameterSettings
//! [Procedure Callbacks]:  ProcedureCallbacks

pub use crate::primitive::ConnectionMode;
pub use crate::generic::ParameterSettings;
pub use crate::generic::MessageID;
pub use crate::generic::MessageContents;
pub use crate::generic::RejectReason;

use crate::generic;
use crate::generic::DeselectStatus;
use crate::generic::ProcedureCallbacks;
use crate::generic::SelectionState;
use crate::generic::SelectStatus;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct Client {
  generic_client: Arc<generic::Client>,
}

// Connection procedures
impl Client {
  pub fn new(
    parameter_settings: ParameterSettings,
  ) -> Arc<Self> {
    Arc::new(Client {
      generic_client: generic::Client::new(
        parameter_settings,
        ProcedureCallbacks {
          select: match parameter_settings.connect_mode {
            ConnectionMode::Passive => Arc::new(|session_id, selection_count| -> Option<SelectStatus> {
              // In HSMS-SS, only a single session may be initiated, and only a
              // Session ID of 0xFFFF is valid.
              if selection_count == 0 && session_id == 0xFFFF {
                Some(SelectStatus::Ok)
              } else {
                None
              }
            }),
            ConnectionMode::Active => Arc::new(|_session_id, _selection_count| -> Option<SelectStatus> {
              // In HSMS-SS, only the active entity may initiate the Select
              // Procedure.
              None
            }),
          },
          deselect: Arc::new(|_session_id, _selection_count| -> Option<DeselectStatus> {
            // In HSMS-SS, the Deselect Procedure is forbidden.
            None
          }),
          separate: Arc::new(move |_session_id, _selection_count| -> Option<bool> {
            // Technically, only a Session ID of 0xFFFF is valid, however, whether
            // this specific ID is recieved, the proper response is to disconnect,
            // so the result is the same either way.
            None
          }),
        },
      ),
    })
  }

  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
  ) -> Result<(SocketAddr, Receiver<(MessageID, semi_e5::Message)>), Error> {
    let connection: (SocketAddr, Receiver<(MessageID, semi_e5::Message)>) = self.generic_client.connect(entity)?;
    match self.generic_client.parameter_settings.connect_mode {
      ConnectionMode::Passive => {
        // TODO: Add some kind of "wait to be selected" code here.
        Ok(connection)
      }
      ConnectionMode::Active => {
        match self.generic_client.select(MessageID {
          session: 0xFFFF,
          system: 0,
        }).join().unwrap() {
          Ok(_) => Ok(connection),
          Err(error) => {
            let _ = self.generic_client.disconnect();
            Err(error)
          }
        }
      }
    }
  }

  pub fn disconnect(
    self: &Arc<Self>,
  ) -> Result<(), Error> {
    if let SelectionState::Selected = self.generic_client.selection_state.load(Relaxed) {
      self.generic_client.separate(MessageID {session: 0xFFFF, system: 0}).join().unwrap()?;
    }
    self.generic_client.disconnect()
  }
}

// Message Exchange Procedures
impl Client {
  pub fn data(
    self: &Arc<Self>,
    id: MessageID,
    message: semi_e5::Message,
  ) -> JoinHandle<Result<Option<semi_e5::Message>, Error>> {
    self.generic_client.data(id, message)
  }

  pub fn linktest(
    self: &Arc<Self>,
    system: u32,
  ) -> JoinHandle<Result<(), Error>> {
    // TODO: only allowed in selected state
    self.generic_client.linktest(system)
  }

  pub fn reject(
    self: &Arc<Self>,
    id: MessageID,
    ps_type: u8,
    reason: RejectReason,
  ) -> JoinHandle<Result<(), Error>> {
    self.generic_client.reject(id, ps_type, reason)
  }
}
