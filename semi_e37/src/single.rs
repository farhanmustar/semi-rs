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
//! [Client]:                           Client
//! [New Client]:                       Client::new
//! [Connect Procedure]:                Client::connect
//! [Disconnect Procedure]:             Client::disconnect
//! [Data Procedure]:                   Client::data
//! [Linktest Procedure]:               Client::linktest
//! [Reject Procedure]:                 Client::reject
//! [Message]:                          Message
//! [Message ID]:                       MessageID
//! [Message Contents]:                 MessageContents
//! [Data Message]:                     MessageContents::DataMessage
//! [Linktest.req]:                     MessageContents::LinktestRequest
//! [Reject.req]:                       MessageContents::RejectRequest
//! [Connection State]:                 crate::primitive::ConnectionState
//! [Selection State]:                  SelectionState
//! [Parameter Settings]:               ParameterSettings
//! [Procedure Callbacks]:              ProcedureCallbacks

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
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

/// ## CLIENT
/// 
/// Encapsulates the full functionality of the [HSMS] protocol with respect to
/// the [Single Selected Session Services] subsidiary protocol.
/// 
/// [HSMS]:                             crate
/// [Generic Services]:                 crate::generic
/// [Single Selected Session Services]: crate::single
pub struct Client {
  /// ### GENERIC CLIENT
  /// 
  /// The [Generic Client] responsible for handling the [Connection State],
  /// [Selection State], and all fundamental procedures, by undertaking the
  /// responsibilities outlined in the [Generic Services].
  /// 
  /// [Generic Client]: crate::generic::Client
  /// [Connection State]: crate::primitive::ConnectionState
  /// [Selection State]:  crate::generic::SelectionState
  /// [Generic Services]: crate::generic
  generic_client: Arc<generic::Client>,
}

/// ## CONNECTION PROCEDURES
/// **Based on SEMI E37.1-0702§6-7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// establishing and breaking a TCP/IP connection.
/// 
/// - [New Client]
/// - [Connect Procedure]
/// - [Disconnect Procedure]
/// 
/// [Client]:               Client
/// [New Client]:           Client::new
/// [Connect Procedure]:    Client::connect
/// [Disconnect Procedure]: Client::disconnect
impl Client {
  /// ### NEW CLIENT
  /// 
  /// Creates a [Client] in the [NOT CONNECTED] state, ready to initiate the
  /// [Connect Procedure].
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [NOT CONNECTED]:     crate::primitive::ConnectionState::NotConnected
  pub fn new(
    parameter_settings: ParameterSettings,
  ) -> Arc<Self> {
    // CREATE CLIENT
    //
    // All of the data required to return a client can be created in one motion,
    // in an infallable manner.
    Arc::new(Client {
      // GENERIC CLIENT
      //
      // Setting up the generic client to work properly for HSMS-SS scenarios is
      // the only concern.
      generic_client: generic::Client::new(
        // PARAMETER SETTINGS
        //
        // Although referenced outside of the generic client, the parameter
        // settings are stored within it.
        parameter_settings,

        // PROCEDURE CALLBACKS
        //
        // The behavior of the HSMS-SS protocol is well defined with respect to
        // the Select, Deselect, and Separate procedures, so the callbacks can
        // be created here without any outside input.
        //
        // An important point is that any violations of the HSMS-SS specific
        // restrictions on these procedures are considered to be communications
        // failures, requiring the client to disconnect.
        ProcedureCallbacks {
          // SELECT PROCEDURE CALLBACK
          //
          // For the Select Procedure, separate behaviors are defined based on
          // whether the client is set to the active or passive connection mode.
          select: match parameter_settings.connect_mode {
            // PASSIVE CLIENT
            //
            // The select procedure may only be initiated by the active client,
            // so can be received by the passive client.
            ConnectionMode::Passive => Arc::new(|session_id, selection_count| -> Option<SelectStatus> {
              // PROCEDURE VALIDITY
              //
              // In HSMS-SS, only a single session may be initiated, and only a
              // Session ID of 0xFFFF is valid.
              if selection_count == 0 && session_id == 0xFFFF {
                // VALID PROCEDURE
                //
                // Once verified to be the correct scenario for the procedure,
                // the callback returns with an Ok status.
                Some(SelectStatus::Ok)
              } else {
                // INVALID PROCEDURE
                //
                // If the procedure's scenario is not valid, it is a
                // communications failure.
                None
              }
            }),

            // ACTIVE CLIENT
            //
            // The select procedure may only be initiated by the active client,
            // so should not be received by it in turn.
            ConnectionMode::Active => Arc::new(|_session_id, _selection_count| -> Option<SelectStatus> {
              // INVALID PROCEDURE
              //
              // Any reception of this procedure by the active client is a
              // communications failure.
              None
            }),
          },

          // DESELECT PROCEDURE CALLBACK
          //
          // The deselect procedure is invalid and may not be used in HSMS-SS.
          deselect: Arc::new(|_session_id, _selection_count| -> Option<DeselectStatus> {
            // INVALID PROCEDURE
            //
            // Any reception of this procedure is a communications failure.
            None
          }),

          // SEPARATE PROCEDURE CALLBACK
          //
          // The separate procedure is required to use a Session ID of 0xFFFF,
          // and the client must immediately disconnect upon receiving it.
          separate: Arc::new(move |_session_id, _selection_count| -> Option<bool> {
            // DISCONNECT
            //
            // Technically, the proper response to receiving the separate
            // procedure is always to disconnect, since it is either valid,
            // requiring an immediate disconnect, or invalid, so treated as a
            // communications failure, so there is no need to care if the
            // Session ID was correct or not.
            None
          }),
        },
      ),
    })
  }


  /// ### CONNECT PROCEDURE
  /// **Based on SEMI E37.1-0702§6,7.1**
  /// 
  /// Connects the [Client] to the Remote Entity.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] state to use this
  /// procedure.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connect Procedure] has two different behaviors based on the
  /// [Connection Mode] provided to it:
  /// - [PASSIVE] - The socket address of the Local Entity must be provided,
  ///   and the [Client] listens for and accepts the [Connect Procedure] when
  ///   initiated by the Remote Entity. On the event of a successful connection,
  ///   the [Client] will wait for up to the time specified by [T7] for the
  ///   [Select Procedure] to be initiated by the active entity, and initiate
  ///   the [Disconnect Procedure] otherwise.
  /// - [ACTIVE] - The socket address of the Remote Entity must be provided,
  ///   and the [Client] initiates the [Connect Procedure] and waits up to the
  ///   time specified by [T5] for the Remote Entity to respond. On the event of
  ///   a successful connection, the [Client] will then initiate and attempt to
  ///   complete the [Select Procedure], and initiate the [Disconnect Procedure]
  ///   if it does not complete successfully.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Connect Procedure], the [T8] parameter is set as
  /// the TCP stream's read and write timeout, and the [CONNECTED] state is
  /// entered.
  /// 
  /// [Connection State]:     crate::primitive::ConnectionState
  /// [NOT CONNECTED]:        crate::primitive::ConnectionState::NotConnected
  /// [CONNECTED]:            crate::primitive::ConnectionState::Connected
  /// [Connection Mode]:      crate::primitive::ConnectionMode
  /// [PASSIVE]:              crate::primitive::ConnectionMode::Passive
  /// [ACTIVE]:               crate::primitive::ConnectionMode::Active
  /// [Client]:               Client
  /// [Connect Procedure]:    Client::connect
  /// [Disconnect Procedure]: Client::disconnect
  /// [Select Procedure]:     crate::generic::Client::select
  /// [T5]:                   ParameterSettings::t5
  /// [T7]:                   ParameterSettings::t7
  /// [T8]:                   ParameterSettings::t8
  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
  ) -> Result<(SocketAddr, Receiver<(MessageID, semi_e5::Message)>), Error> {
    // CONNECT GENERIC CLIENT
    //
    // The generic client is told to initiate a connection using the provided
    // entity and saved connection mode. This operation is fallable and extends
    // all the way to the primitive client.
    let connection: (SocketAddr, Receiver<(MessageID, semi_e5::Message)>) = self.generic_client.connect(entity)?;

    // COMPLETE CONNECTION
    //
    // In HSMS-SS, the connect procedure is always followed by the select
    // procedure before any other communications can occur, so that is handled
    // here, with separate behaviors based on the connection mode.
    match self.generic_client.parameter_settings.connect_mode {
      // PASSIVE CLIENT
      //
      // The passive client must wait for the select procedure to complete.
      ConnectionMode::Passive => {
        // TODO: Add some kind of "wait to be selected" code here.
        Ok(connection)
      }

      // ACTIVE CLIENT
      //
      // The active client is responsible for initiating the select procedure
      // after the connection is established.
      ConnectionMode::Active => {
        // SELECT PROCEDURE
        //
        // The select procedure is initiated here and must complete, either
        // successfully or unsucessfully, before proceeding.
        match self.generic_client.select(MessageID {
          session: 0xFFFF,
          system: 0,
        }).join().unwrap() {
          // UNSUCESSFUL
          //
          // In the case that the select procedure was unsucessful, this is
          // considered a communications failure, and the client must disconnect
          // rather than providing the connection information from the generic
          // client.
          Err(error) => {
            let _ = self.generic_client.disconnect();
            Err(error)
          }

          // SUCCESSFUL
          //
          // In the case that the select procedure was successful, the HSMS-SS
          // connect procedure is now complete.
          Ok(()) => Ok(connection),
        }
      }
    }
  }

  /// ### DISCONNECT PROCEDURE
  /// **Based on SEMI E37.1-0702§6,7.6**
  /// 
  /// Disconnects the [Client] from the Remote Entity.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// If the [Client] is in the [SELECTED] state upon initiation of this
  /// procedure, it will first complete the [Separate Procedure] in order to
  /// enter the [NOT SELECTED] state, then upon completion of the
  /// [Disconnect Procedure], the [NOT CONNECTED] state is entered.
  /// 
  /// [Connection State]:     crate::primitive::ConnectionState
  /// [NOT CONNECTED]:        crate::primitive::ConnectionState::NotConnected
  /// [CONNECTED]:            crate::primitive::ConnectionState::Connected
  /// [SELECTED]:             crate::generic::SelectionState::Selected
  /// [NOT SELECTED]:         crate::generic::SelectionState::NotSelected
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Separate Procedure]:   crate::generic::Client::separate
  pub fn disconnect(
    self: &Arc<Self>,
  ) -> Result<(), Error> {
    // SEPARATE PROCEDURE
    //
    // In the case that the selected state is entered, the separate procedure
    // must be used when breaking communications.
    if let SelectionState::Selected = self.generic_client.selection_state.load(Relaxed) {
      self.generic_client.separate(MessageID {session: 0xFFFF, system: 0}).join().unwrap()?;
    }

    // DISCONNECT GENERIC CLIENT
    //
    // The generic client can now be disconnected, no further HSMS-SS specific
    // cases must be handled.
    self.generic_client.disconnect()
  }
}

/// ## MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37.1-0702§7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// exchanging [Message]s.
/// 
/// - [Data Procedure] - [Data Message]s
/// - [Linktest Procedure] - [Linktest.req] and [Linktest.rsp]
/// - [Reject Procedure] - [Reject.req]
/// 
/// [Message]:            crate::generic::Message
/// [Client]:             Client
/// [Data Procedure]:     Client::data
/// [Linktest Procedure]: Client::linktest
/// [Reject Procedure]:   Client::reject
/// [Data Message]:       MessageContents::DataMessage
/// [Linktest.req]:       MessageContents::LinktestRequest
/// [Linktest.rsp]:       MessageContents::LinktestResponse
/// [Reject.req]:         MessageContents::RejectRequest
impl Client {
  /// ### DATA PROCEDURE
  /// **Based on SEMI E37.1-0702§7.2**
  /// 
  /// Asks the [Client] to initiate the [Data Procedure] by transmitting a
  /// [Data Message] and waiting for the corresponding response to be received
  /// if it is necessary to do so.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// When a Response [Data Message] is necessary, the [Client] will wait
  /// to receive it for the amount of time specified by [T3] before it will
  /// consider it a communications failure and initiate the
  /// [Disconnect Procedure].
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will respond to having received a [Data Message] based on its
  /// contents and the current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [Data Procedure] and
  ///   completing the [Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [Client] will send the
  ///   [Data Message] to the hook provided by the [Connect Procedure].
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [Data Procedure] if successful,
  ///   or if unsuccessful by transmitting a [Reject.req] message, rejecting
  ///   the [Data Procedure] and completing the [Reject Procedure].
  /// 
  /// [Connection State]:     crate::primitive::ConnectionState
  /// [CONNECTED]:            crate::primitive::ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T3]:                   ParameterSettings::t3
  /// [Client]:               Client
  /// [Connect Procedure]:    Client::connect
  /// [Disconnect Procedure]: Client::disconnect
  /// [Data Procedure]:       Client::data
  /// [Reject Procedure]:     Client::reject
  /// [Data Message]:         MessageContents::DataMessage
  /// [Reject.req]:           MessageContents::RejectRequest
  pub fn data(
    self: &Arc<Self>,
    id: MessageID,
    message: semi_e5::Message,
  ) -> JoinHandle<Result<Option<semi_e5::Message>, Error>> {
    // GENERIC DATA PROCEDURE
    //
    // No HSMS-SS specific scenarios must be handled.
    self.generic_client.data(id, message)
  }

  /// ### LINKTEST PROCEDURE
  /// **Based on SEMI E37.1-0702§7.4**
  /// 
  /// Asks the [Client] to initiate the [Linktest Procedure] by transmitting a
  /// [Linktest.req] message and waiting for the corresponding [Linktest.rsp]
  /// message to be received.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state, and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// The [Client] will wait to receive the [Linktest.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the
  /// [CONNECTED] state will respond to having received a [Linktest.req]
  /// message with a [Linktest.rsp], completing the [Linktest Procedure].
  /// 
  /// [Connection State]:     crate::primitive::ConnectionState
  /// [CONNECTED]:            crate::primitive::ConnectionState::Connected
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Linktest Procedure]:   Client::linktest
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T6]:                   ParameterSettings::t6
  /// [Linktest.req]:         MessageContents::LinktestRequest
  /// [Linktest.rsp]:         MessageContents::LinktestResponse
  pub fn linktest(
    self: &Arc<Self>,
    system: u32,
  ) -> JoinHandle<Result<(), Error>> {
    // SELECTION STATE
    //
    // Uniquely to HSMS-SS, the Linktest procedure is only valid in the selected
    // state.
    match self.generic_client.selection_state.load(Relaxed) {
      // NOT SELECTED
      //
      // In the not selected state, the linktest procedure is invalid. Due to
      // the Connect and Select states being so closely tied in HSMS-SS, the
      // assumption if this path is reached is that a connection has not been
      // properly established.
      SelectionState::NotSelected => thread::spawn(|| {Err(Error::from(ErrorKind::NotConnected))}),

      // SELECTED
      //
      // In the selected state, the linktest procedure is valid, and no further
      // HSMS-SS specific scenarios must be handled.
      SelectionState::Selected => self.generic_client.linktest(system),
    }
  }

  /// ### REJECT PROCEDURE
  /// **Based on SEMI E37.1-0702§7.5**
  /// 
  /// Asks the [Client] to complete the [Reject Procedure] by transmitting a
  /// [Reject.req] message.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// --------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will respond to having received a [Reject.req] by correlating the
  /// message to a previously sent message which is awaiting a reply, aborting
  /// a previously initiated [Data Procedure], [Select Procedure],
  /// [Deselect Procedure], or [Linktest Procedure], and completing the
  /// [Reject Procedure].
  /// 
  /// [Connection State]:   crate::primitive::ConnectionState
  /// [CONNECTED]:          crate::primitive::ConnectionState::Connected
  /// [Client]:             Client
  /// [Data Procedure]:     Client::data
  /// [Select Procedure]:   crate::generic::Client::select
  /// [Deselect Procedure]: crate::generic::Client::deselect
  /// [Linktest Procedure]: Client::linktest
  /// [Reject Procedure]:   Client::reject
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [Reject.req]:         MessageContents::RejectRequest
  pub fn reject(
    self: &Arc<Self>,
    id: MessageID,
    ps_type: u8,
    reason: RejectReason,
  ) -> JoinHandle<Result<(), Error>> {
    // GENERIC REJECT PROCEDURE
    //
    // No HSMS-SS specific scenarios must be handled.
    self.generic_client.reject(id, ps_type, reason)
  }
}
