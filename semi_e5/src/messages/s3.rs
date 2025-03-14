// Copyright © 2025 Nathaniel Hardesty
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

//! # STREAM 3: MATERIAL STATUS
//! **Based on SEMI E5§10.7**
//! 
//! ----------------------------------------------------------------------------
//! 
//! [Message]s which deal with communicating information and actions related
//! to material, including carriers and material-in-process,
//! time-to-completion information, and extraordinary material circumstances.
//! 
//! [Message]: crate::Message

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S3F0
/// 
/// **Abort Transaction**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Used in lieu of an expected reply to abort a transaction.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct Abort;
message_headeronly!{Abort, false, 3, 0}

/// ## S3F1
/// 
/// **Material Status Request (MSR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Request to send the status of all material in process.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct MaterialStatusRequest;
message_headeronly!{MaterialStatusRequest, true, 3, 1}

/// ## S3F2
/// 
/// **Material Status Data (MSD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Material-in-process information.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [MF]
///    2. List - M
///       - List - 3
///          1. [LOC]
///          2. [QUA]
///          3. [MID]
/// 
/// M is the number of locations.
/// 
/// Zero-length M means no such data exists.
/// 
/// [MF]:  MaterialFormat
/// [LOC]: LocationCode
/// [QUA]: Quantity
/// [MID]: MaterialID
pub struct MaterialStatusData(pub (MaterialFormat, VecList<(LocationCode, Quantity, MaterialID)>));
message_data!{MaterialStatusData, false, 3, 2}

/// ## S3F3
/// 
/// **Time To Completion Request (TCR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Request to send the time-to-completion of operations on all material.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct TimeToCompletionRequest;
message_headeronly!{TimeToCompletionRequest, true, 3, 3}

/// ## S3F4
/// 
/// **Time To Completion Data (TCD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Time-to-completion information.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [MF]
///    2. List - M
///       - List - 3
///          1. [TTC]
///          2. [QUA]
///          3. [MID]
/// 
/// Zero-length M means no such data exists.
/// 
/// [MF]:  MaterialFormat
/// [TTC]: TimeToCompletion
/// [QUA]: Quantity
/// [MID]: MaterialID
pub struct TimeToCompletionData(pub (MaterialFormat, VecList<(TimeToCompletion, Quantity, MaterialID)>));
message_data!{TimeToCompletionData, false, 3, 4}

/// ## S3F5
/// 
/// **Material Found Send (MFS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY OPTIONAL**
/// 
/// TODO: Implement optional reply.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Advises the host that unsolicited material has appeared at a sensor.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [MF]
///    2. [QUA]
/// 
/// [MF]:  MaterialFormat
/// [QUA]: Quantity
pub struct MaterialFoundSend(pub (MaterialFormat, Quantity));
message_data!{MaterialFoundSend, true, 3, 5}

/// ## S3F6
/// 
/// **Material Found Acknowledge (MFA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledge or error.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ACKC3]
/// 
/// [ACKC3]: AcknowledgeCode3
pub struct MaterialFoundAcknowledge(pub AcknowledgeCode3);
message_data!{MaterialFoundAcknowledge, false, 3, 6}

/// ## S3F7
/// 
/// **Material Lost Send (MLS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY OPTIONAL**
/// 
/// TODO: Implement optional reply.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Advises the host that material has disappeared from the sensors.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [MF]
///    2. [QUA]
///    3. [MID]
/// 
/// [MF]:  MaterialFormat
/// [QUA]: Quantity
/// [MID]: MaterialID
pub struct MaterialLostSend(pub (MaterialFormat, Quantity, MaterialID));
message_data!{MaterialLostSend, true, 3, 7}

/// ## S3F8
/// 
/// **Material Lost Acknowledge (MFA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledge or error.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ACKC3]
/// 
/// [ACKC3]: AcknowledgeCode3
pub struct MaterialLostAcknowledge(pub AcknowledgeCode3);
message_data!{MaterialLostAcknowledge, false, 3, 8}

/// ## S3F9
/// 
/// **Material ID Equate Send (IES)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Provide an alternative name to be used as equivalent to the original
/// material ID.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [MID]
///    2. [EMID]
/// 
/// [MID]:  MaterialID
/// [EMID]: EquivalentMaterialID
pub struct MaterialIDEquateSend(pub (MaterialID, EquivalentMaterialID));
message_data!{MaterialIDEquateSend, true, 3, 9}

/// ## S3F10
/// 
/// **Material ID Equate Acknowledge (IEA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledge or error.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ACKC3]
/// 
/// [ACKC3]: AcknowledgeCode3
pub struct MaterialIDEquateAcknowledge(pub AcknowledgeCode3);
message_data!{MaterialIDEquateAcknowledge, false, 3, 10}

/// ## S3F11
/// 
/// **Material ID Request (MIDR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests the Material ID of the material at the specified port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [PTN]
/// 
/// [PTN]: PortNumber
pub struct MaterialIDRequest(pub PortNumber);
message_data!{MaterialIDRequest, true, 3, 11}

/// ## S3F12
/// 
/// **Material ID Request Acknowledge (MIRA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the request for the Material ID
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [PTN]
///    2. [MIDRA]
///    3. [MID]
/// 
/// TODO: This message has complicated semantics.
/// 
/// [PTN]:   PortNumber
/// [MIDRA]: MaterialIDRequestAcknowledgeCode
/// [MID]:   MaterialID
pub struct MaterialIDRequestAcknowledge(pub (PortNumber, MaterialIDRequestAcknowledgeCode, MaterialID));
message_data!{MaterialIDRequestAcknowledge, false, 3, 12}

/// ## S3F13
/// 
/// **Material ID Send (MIS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Send the Material ID of the material at the specified port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [PTN]
///    2. [MID]
/// 
/// Zero-length [MID] indicates that no [MID] is available.
/// 
/// [PTN]: PortNumber
/// [MID]: MaterialID
pub struct MaterialIDSend(pub (PortNumber, MaterialID));
message_data!{MaterialIDSend, true, 3, 13}

/// ## S3F14
/// 
/// **Material ID Acknowledge (MIA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledge or error.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [MIDAC]
/// 
/// [MIDAC]: MaterialIDAcknowledgeCode
pub struct MaterialIDAcknowledge(pub MaterialIDAcknowledgeCode);
message_data!{MaterialIDAcknowledge, false, 3, 14}

/// ## S3F15
/// 
/// **Materials Multi-Block Inquire (MMBI)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Establish that sending a multi-block message is allowed prior to sending
/// 
/// TODO: Finish Description
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATAID]
///    2. [DATALENGTH]
/// 
/// [DATAID]:     DataID
/// [DATALENGTH]: DataLength
pub struct MultiBlockInquire(pub (DataID, DataLength));
message_data!{MultiBlockInquire, true, 3, 15}

/// ## S3F16
/// 
/// **Materials Multi-Block Grant (MMBG)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Grant permission to send a multi-block message.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [GRANT]
/// 
/// [GRANT]: Grant
pub struct MultiBlockGrant(pub Grant);
message_data!{MultiBlockGrant, false, 3, 16}

/// ## S3F17
/// 
/// **Carrier Action Request**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests an action to be performed for a specified carrier.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 5
///    1. [DATAID]
///    2. [CARRIERACTION]
///    3. [CARRIERID]
///    4. [PTN]
///    5. List - N
///       - List - 2
///          1. [CATTRID]
///          2. [CATTRDATA]
/// 
/// N is the number of carrier attributes.
/// 
/// If N is zero-length, no carrier attributes are included.
/// 
/// TODO: Message has complex semantics.
/// 
/// [DATAID]:        DataID
/// [CARRIERACTION]: CarrierAction
/// [CARRIERID]:     CarrierID
/// [PTN]:           PortNumber
/// [CATTRID]:       CarrierAttributeID
/// [CATTRDATA]:     CarrierAttributeValue
pub struct CarrierActionRequest(pub (DataID, CarrierAction, CarrierID, PortNumber, VecList<(CarrierAttributeID, CarrierAttributeValue)>));
message_data!{CarrierActionRequest, true, 3, 17}

/// ## S3F18
/// 
/// **Carrier Action Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the carrier action request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// N is the number of errors.
/// 
/// If N is zero-length, there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct CarrierActionAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{CarrierActionAcknowledge, false, 3, 18}

/// ## S3F19
/// 
/// **Cancel All Carrier Out Request**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Cancel all pending carrier out requests.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct CancelAllCarrierOutRequest;
message_headeronly!{CancelAllCarrierOutRequest, true, 3, 19}

/// ## S3F20
/// 
/// **Cancel All Carrier Out Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the cancel carrier out request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// N is the number of errors.
/// 
/// If N is zero-length, there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct CancelAllCarrierOutAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{CancelAllCarrierOutAcknowledge, false, 3, 18}

/// ## S3F21
/// 
/// **Port Group Definition**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Defines mhe port in a port group and provides the initial port access.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [PORTGRPNAME]
///    2. [ACCESSMODE]
///    3. List - N
///       - [PTN]
/// 
/// [PORTGRPNAME]: PortGroupName
/// [ACCESSMODE]:  AccessMode
/// [PTN]:         PortNumber
pub struct PortGroupDefinition(pub (PortGroupName, AccessMode, VecList<PortNumber>));
message_data!{PortGroupDefinition, true, 3, 19}

/// ## S3F22
/// 
/// **Port Group Definiton Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the port group definition.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// N is the number of errors.
/// 
/// If N is zero-length, there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct PortGroupDefinitionAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{PortGroupDefinitionAcknowledge, false, 3, 22}

/// ## S3F23
/// 
/// **Port Group Action Request**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests an action be performed for a port group. The access mode may be
/// changed or the port group may be deleted.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [PGRPACTION]
///    2. [PORTGRPNAME]
///    3. List - M
///       - List - 2
///          1. [PARAMNAME]
///          2. [PARAMVAL]
/// 
/// If M is zero-length, no parameters are provided.
/// 
/// [PGRPACTION]: PortGroupAction
/// [PORTGRPNAME]:   PortGroupName
/// [PARAMNAME]:     ParameterName
/// [PARAMVAL]:      ParameterValue
pub struct PortGroupActionRequest(pub (PortGroupAction, PortGroupName, VecList<(ParameterName, ParameterValue)>));
message_data!{PortGroupActionRequest, true, 3, 23}

/// ## S3F24
/// 
/// **Port Group Action Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the port group action request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// N is the number of errors.
/// 
/// If N is zero-length, there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct PortGroupActionAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{PortGroupActionAcknowledge, false, 3, 24}

/// ## S3F25
/// 
/// **Port Action Request**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests an action be performed for a port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [PORTACTION]
///    2. [PTN]
///    3. List - M
///       - List - 2
///          1. [PARAMNAME]
///          2. [PARAMVALUE]
/// 
/// Zero-length M means no parameters are provided.
/// 
/// [PORTACTION]: PortAction
/// [PTN]:        PortNumber
/// [PARAMNAME]:  ParameterName
/// [PARAMVALUE]: ParameterValue
pub struct PortActionRequest(pub (PortAction, PortNumber, VecList<(ParameterName, ParameterValue)>));
message_data!{PortActionRequest, true, 3, 25}

/// ## S3F26
/// 
/// **Port Action Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the port action request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// N is the number of errors.
/// 
/// If N is zero-length, there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct PortActionAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{PortActionAcknowledge, false, 3, 26}

/// ## S3F27
/// 
/// **Change Access**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Request the equipment to change the access mode for the specified load
/// ports.
/// 
/// If any specified port is already in the specified access mode, then the
/// equipment shall accept the command regardless.
/// 
/// If the equipment is unable to change one or more of the specified ports to
/// the specified access mode, then it shall accept the command, and shall
/// change the access mode for the ports which it is able to.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [ACCESSMODE]
///    2. List - N
///       - [PTN]
/// 
/// Zero-length N means the command applies to all load ports on the equipment.
/// 
/// [ACCESSMODE]: AccessMode
/// [PTN]:        PortNumber
pub struct ChangeAccess(pub (AccessMode, VecList<PortNumber>));
message_data!{ChangeAccess, true, 3, 27}

/// ## S3F28
/// 
/// **Port Action Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the change access request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 3
///          1. [PTN]
///          2. [ERRCODE]
///          3. [ERRTEXT]
/// 
/// [CAACK] of 0 and Zero-Length N means the command is successful.
/// 
/// [CAACK] of 6 and Non-Zero N means the command was successful for some ports.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [PTN]:     PortNumber
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct ChangeAccessAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(PortNumber, ErrorCode, ErrorText)>));
message_data!{ChangeAccessAcknowledge, false, 3, 28}

/// ## S3F29
/// 
/// **Carrier Tag Read Request**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests the equipment to read data from the carrier tag of a carrier. The
/// carrier must be identified by either its location identifier, its carrier
/// identifier, or both.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 4
///    1. [LOCID]
///    2. [CARRIERSPEC]
///    3. [DATASEG]
///    4. [DATALENGTH]
/// 
/// TODO: Message has complex semantics.
/// 
/// [LOCID]:       LocationID
/// [CARRIERSPEC]: CarrierSpecifier
/// [DATASEG]:     DataSegment
/// [DATALENGTH]:  DataLength
pub struct CarrierTagReadRequest(pub (LocationID, CarrierSpecifier, DataSegment, DataLength));
message_data!{CarrierTagReadRequest, true, 3, 29}

/// ## S3F30
/// 
/// **Carrier Tag Read Data (CTRD)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Returns requested information from the carrier tag of the carrier indicated
/// in the request and acknowledges the results of the request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATA]
///    2. List - 2
///       1. [CAACK]
///       2. List - S
///          - List - 2
///             1. [ERRCODE]
///             2. [ERRTEXT]
/// 
/// TODO: Possible to represent considerations about DATA and CAACK?
/// 
/// [DATA]:    Data
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct CarrierTagReadData(pub (Data, (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>)));
message_data!{CarrierTagReadData, false, 3, 30}

/// ## S3F31
/// 
/// **Carrier Tag Write Data Request (CTWDR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests the equipment write data to a carrier tag. The carrier must be
/// identified by either its location identifier, its carrier identifier, or
/// both.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 5
///    1. [LOCID]
///    2. [CARRIERSPEC]
///    3. [DATASEG]
///    4. [DATALENGTH]
///    5. [DATA]
/// 
/// TODO: Message has complex semantics.
/// 
/// [LOCID]:       LocationID
/// [CARRIERSPEC]: CarrierSpecifier
/// [DATASEG]:     DataSegment
/// [DATALENGTH]:  DataLength
/// [DATA]:        Data
pub struct CarrierTagWriteDataRequest(pub (LocationID, CarrierSpecifier, DataSegment, DataLength, Data));
message_data!{CarrierTagWriteDataRequest, true, 3, 31}

/// ## S3F32
/// 
/// **Carrier Tag Write Data Acknowledge (CTWDA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges success or failure of writing data to the requested carrier
/// tag.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// Zero-length N means there are no errors.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct CarrierTagWriteDataAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{CarrierTagWriteDataAcknowledge, false, 3, 32}

/// ## S3F33
/// 
/// **Cancel All Pod Out Request**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Cancels all pending pod out requests.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct CancelAllPodOutRequest;
message_headeronly!{CancelAllPodOutRequest, true, 3, 33}

/// ## S3F34
/// 
/// **Cancel All Pod Out Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the cancel pod out request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CAACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// Zero-length N means no errors exist.
/// 
/// [CAACK]:   CarrierActionAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct CancelAllPodOutAcknowledge(pub (CarrierActionAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{CancelAllPodOutAcknowledge, false, 3, 34}

/// ## S3F35
/// 
/// **Reticle Transfer Job Request**
/// 
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Requests a reticle transfer job be performed or cancelled for a particular
/// pod.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 7
///    1. [JOBACTION]
///    2. [PODID]
///    3. [INPTN]
///    4. [OUTPTN]
///    5. List - N
///       - List - 2
///          1. [ATTRID]
///          2. [ATTRDATA]
///    6. List - M
///       - List - 3
///          1. [RETICLEID]
///          2. [RETREMOVEINSTR]
///          3. List - R
///             - List - 2
///                1. [ATTRID]
///                2. [ATTRDATA]
///    7. List - K
///       - List - 2
///          1. [RETICLEID]
///          2. [RETPLACEINSTR]
/// 
/// If [JOBACTION] = CancelReticleTransferJob, M and N may be zero-length.
/// 
/// [JOBACTION]:      JobAction
/// [PODID]:          PodID
/// [INPTN]:          InputPortNumber
/// [OUTPTN]:         OutputPortNumber
/// [ATTRID]:         AttributeID
/// [ATTRDATA]:       AttributeValue
/// [RETICLEID]:      ReticleID
/// [RETREMOVEINSTR]: ReticleRemoveInstruction
/// [RETPLACEINSTR]:  ReticlePlaceInstruction
pub struct ReticleTransferJobRequest(pub (JobAction, PodID, InputPortNumber, OutputPortNumber, VecList<(AttributeID, AttributeValue)>, VecList<(ReticleID, ReticleRemoveInstruction, VecList<(AttributeID, AttributeValue)>)>, VecList<(ReticleID, ReticlePlaceInstruction)>));
message_data!{ReticleTransferJobRequest, true, 3, 35}

/// ## S3F36
/// 
/// **Reticle Transfer Job Acknowledgement**
/// 
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Description
/// 
/// Acknowledges the reticle transfer job request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [RPMACK]
///    2. List - N
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// [RPMACK]:  ReticlePodManagementAcknowledgeCode
/// [ERRCODE]: ErrorCode
/// [ERRTEXT]: ErrorText
pub struct ReticleTransferJobAcknowledge(pub (ReticlePodManagementAcknowledgeCode, VecList<(ErrorCode, ErrorText)>));
message_data!{ReticleTransferJobAcknowledge, false, 3, 36}
