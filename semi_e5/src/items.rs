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

//! # ITEMS
//! **Based on SEMI E5§9.6**
//! 
//! ----------------------------------------------------------------------------
//! 
//! Standards compliant [Item] structures and enums designed to express and
//! enforce the specific [Format] of each [Item].
//! 
//! Each such item defined herein implements:
//! - [From]\<T\> for [Item]
//! - [TryFrom]\<[Item]\> for T
//! 
//! ----------------------------------------------------------------------------
//! 
//! As well as the list of specific [Item]s as defined in **Table 3 - Data Item
//! Dictionary**, certain shorthands for varying usage of [List]s are provided.
//! 
//! - [Optional Item]: used to represent an [Item] which may optionally take
//!   the form of a [List] with zero elements.
//! - [Vectorized List]: used to represent a [List] with a variable number of
//!   elements of homogeneous structure.
//! - Rust's Native Unit Type (): Used to represent a [List] with zero
//!   elements.
//! - Rust's Native Tuple Types (A, B, ...): Used to represent a [List] with a
//!   set number of elements of heterogeneous structure.
//!    - Currently, only Tuples of length up to 7 are supported.
//! 
//! [Optional Item]:   OptionItem
//! [Vectorized List]: VecList
//! [Item]:            crate::Item
//! [Format]:          crate::format
//! [List]:            crate::Item::List

use crate::Item;
use crate::Error::{self, *};
use std::ascii::Char;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// ## OPTIONAL ITEM
/// 
/// Represents an [Item] which may alternatively take the form of a [List] with
/// zero elements.
/// 
/// [Item]: crate::Item
/// [List]: crate::Item::List
pub struct OptionItem<T>(pub Option<T>);

/// ## ITEM -> OPTIONAL ITEM
impl<A: TryFrom<Item, Error = Error> + Sized> TryFrom<Item> for OptionItem<A> {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    if let Item::List(list) = item {
      if list.is_empty() {
        return Ok(Self(None));
      } else {
        return Ok(Self(Some(Item::List(list).try_into()?)));
      }
    }
    Ok(Self(Some(item.try_into()?)))
  }
}

/// ## OPTIONAL ITEM -> ITEM
impl<A: Into<Item>> From<OptionItem<A>> for Item {
  fn from(option_list: OptionItem<A>) -> Self {
    match option_list.0 {
      Some(item) => item.into(),
      None => Item::List(vec![]),
    }
  }
}

/// ## VECTORIZED LIST
/// 
/// Represents a List with a variable number of elements of homogeneous
/// structure. The intent is that type T will be a specific item.
pub struct VecList<T>(pub Vec<T>);

/// ## ITEM -> VECTORIZED LIST
impl<A: TryFrom<Item, Error = Error> + Sized> TryFrom<Item> for VecList<A> {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        let mut vec = vec![];
        for list_item in list {
          vec.push(list_item.try_into()?)
        }
        Ok(Self(vec))
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## VECTORIZED LIST -> ITEM
impl<A: Into<Item>> From<VecList<A>> for Item {
  fn from(vec_list: VecList<A>) -> Self {
    let mut vec = vec![];
    for item in vec_list.0 {
      vec.push(item.into())
    }
    Item::List(vec)
  }
}

// EMPTY LIST IS IMPLEMENTED BY THE USE OF THE UNIT TYPE ()

/// ## ITEM -> EMPTY LIST
impl TryFrom<Item> for () {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.is_empty() {
          Ok(())
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## EMPTY LIST -> ITEM
impl From<()> for Item {
  fn from(_empty_list: ()) -> Self {
    Item::List(vec![])
  }
}

// HETEROGENEOUS LISTS ARE IMPLEMENTED BY USE OF TUPLE TYPES (...)

/// ## ITEM -> HETEROGENEOUS LIST (2 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 2 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (2 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
> From<(A, B)> for Item {
  fn from(value: (A, B)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (3 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 3 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (3 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
> From<(A, B, C)> for Item {
  fn from(value: (A, B, C)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (4 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 4 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (4 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
> From<(A, B, C, D)> for Item {
  fn from(value: (A, B, C, D)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (5 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
  E: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D, E) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 5 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
            list[4].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (5 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
  E: Into<Item>,
> From<(A, B, C, D, E)> for Item {
  fn from(value: (A, B, C, D, E)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
      value.4.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (6 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
  E: TryFrom<Item, Error = Error>,
  F: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D, E, F) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 6 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
            list[4].clone().try_into()?,
            list[5].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (6 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
  E: Into<Item>,
  F: Into<Item>,
> From<(A, B, C, D, E, F)> for Item {
  fn from(value: (A, B, C, D, E, F)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
      value.4.into(),
      value.5.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (7 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
  E: TryFrom<Item, Error = Error>,
  F: TryFrom<Item, Error = Error>,
  G: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D, E, F, G) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 6 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
            list[4].clone().try_into()?,
            list[5].clone().try_into()?,
            list[6].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (7 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
  E: Into<Item>,
  F: Into<Item>,
  G: Into<Item>,
> From<(A, B, C, D, E, F, G)> for Item {
  fn from(value: (A, B, C, D, E, F, G)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
      value.4.into(),
      value.5.into(),
      value.6.into(),
    ])
  }
}

// TODO: ITEM -> HETEROGENEOUS LIST, UP TO 15 ELEMENTS
// TODO: HETEROGENEOUS LIST -> ITEM, UP TO 15 ELEMENTS
// NOTE: To implement Stream 1, only lengths of 2 and 3 are required.

// IMPLEMENTATION MACROS

/// ## DATA ITEM MACRO: SINGLE FORMAT
/// 
/// #### Arguments:
/// 
/// - **$name**: Name of struct.
/// - **$format**: Item format.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion:
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! singleformat {
  (
    $name:ident,
    $format:ident
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(vec![value.0])
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: SINGLE FORMAT, VEC
/// 
/// #### Arguments:
/// - **$name**: Name of struct.
/// - **$format**: Item format.
/// - Optional:
///    - **$range**: Range expression limiting vector length.
///    - **$type**: Type contained in vector.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion:
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
/// - Optional:
///    - new(Vec\<$type\>) -> Option\<Self\>
///    - read(&self) -> &Vec\<$type\>
macro_rules! singleformat_vec {
  (
    $name:ident,
    $format:ident
    $(,$range:expr, $type:ty)?
  ) => {
    $(impl $name {
      pub fn new(vec: Vec<$type>) -> Option<Self> {
        if $range.contains(&vec.len()) {
          Some(Self(vec))
        } else {
          None
        }
      }
      pub fn read(&self) -> &Vec<$type> {
        &self.0
      }
    })?
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(value.0)
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            $(if !$range.contains(&vec.len()) {
              return Err(WrongFormat)
            })?
            Ok(Self(vec))
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: SINGLE FORMAT, ENUM
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
/// - From\<Vec\<$name\>\> for Item
/// - TryFrom\<Item\> for Vec\<$name\>
macro_rules! singleformat_enum {
  (
    $name:ident,
    $format:ident
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(vec![value.into()])
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              $name::try_from(vec[0]).map_err(|_| -> Self::Error {WrongFormat})
            } else {
              Err(WrongFormat)
            }
          },
          _ => Err(WrongFormat),
        }
      }
    }
    impl From<Vec<$name>> for Item {
      fn from(vec: Vec<$name>) -> Item {
        let mut newvec = vec![];
        for value in vec {
          newvec.push(value.into());
        }
        Item::$format(newvec)
      }
    }
    impl TryFrom<Item> for Vec<$name> {
      type Error = Error;

      fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
          Item::$format(vec) => {
            let mut newvec: Vec<$name> = vec![];
            for value in vec {
              newvec.push($name::try_from(value).map_err(|_| -> Self::Error {WrongFormat})?);
            }
            Ok(newvec)
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::$format(val) => Item::$format(vec![val]),
          $(
            $name::$formats(val) => Item::$formats(vec![val]),
          )*
        }
        
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self::$format(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          $(
            Item::$formats(vec) => {
              if vec.len() == 1 {
                Ok(Self::$formats(vec[0]))
              } else {
                Err(WrongFormat)
              }
            },
          )*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT + ASCII
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat_ascii {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::Ascii(vec) => Item::Ascii(vec),
          $name::$format(val) => Item::$format(vec![val]),
          $($name::$formats(val) => Item::$formats(vec![val]),)*
        }
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
          Item::Ascii(vec) => Ok($name::Ascii(vec)),
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self::$format(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          $(Item::$formats(vec) => {
            if vec.len() == 1 {
              Ok(Self::$formats(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },)*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT, VEC
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat_vec {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::$format(vec) => Item::$format(vec),
          $(
            $name::$formats(vec) => Item::$formats(vec),
          )*
        }
        
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            Ok(Self::$format(vec))
          },
          $(
            Item::$formats(vec) => {
              Ok(Self::$formats(vec))
            },
          )*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

// ITEMS

/// ## ABS
/// 
/// Any binary string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F25], [S2F26]
/// 
/// [S2F25]: crate::messages::s2::LoopbackDiagnosticRequest
/// [S2F26]: crate::messages::s2::LoopbackDiagnosticData
#[derive(Clone, Debug)]
pub struct AnyBinaryString(pub Vec<u8>);
singleformat_vec!{AnyBinaryString, Bin}

/// ## ACCESSMODE
/// 
/// **Load Port Access Mode**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F21], [S3F27]
/// 
/// [S3F21]: crate::messages::s3::PortGroupDefinition
/// [S3F27]: crate::messages::s3::ChangeAccess
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AccessMode {
  Manual = 0,
  Auto = 1,
}
singleformat_enum!{AccessMode, U1}

/// ## ACDS
/// 
/// After Command Codes
/// 
/// Vector of all command codes which the defined command must succeed
/// within the same block.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Debug)]
pub enum AfterCommandCodes {
  I2(Vec<i16>),
  U2(Vec<u16>),
}
multiformat_vec!{AfterCommandCodes, I2, U2}

/// ## ACKA
/// 
/// Request success, true is successful, false is unsuccessful.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F14, S5F15, S5F18
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24, S16F26, S16F28,
///   S16F30
/// - S17F4, S17F8, S17F14
#[derive(Clone, Copy, Debug)]
pub struct AcknowledgeAny(pub bool);
singleformat!{AcknowledgeAny, Bool}

/// ## ACKC3
/// 
/// **Acknowledge Code: Stream 3**
/// 
/// TODO: How to deal with reserved and non-reserved values?
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F6], [S3F8], [S3F10]
/// 
/// [S3F6]:  crate::messages::s3::MaterialFoundAcknowledge
/// [S3F8]:  crate::messages::s3::MaterialLostAcknowledge
/// [S3F10]: crate::messages::s3::MaterialIDEquateAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AcknowledgeCode3 {
  Accepted = 0,
}
singleformat_enum!{AcknowledgeCode3, Bin}

// TODO: ACKC5
// How to deal with 1-63 being reserved but the rest being open for user values?

// TODO: ACKC6
// How to deal with 1-63 being reserved but the rest being open for user values?

// TODO: ACKC7
// How to deal with 7-63 being reserved but the rest being open for user values?

// TODO: ACKC7A
// How to deal with 6-63 being reserved but the rest being open for user values?

// TODO: ACKC10
// How to deal with 3-63 being reserved but the rest being open for user values?

// TODO: ACKC13
// How to deal with 11-127 being reserved but the rest being open for user values?

// TODO: ACKC15
// How to deal with 5-63 being reserved but the rest being open for user values?

/// ## AGENT
/// 
/// TODO: Document variable based on appearances in streams.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S15F11, S15F12, S15F21, S15F22, S15F25
#[derive(Clone, Debug)]
pub struct Agent(pub Vec<Char>);
singleformat_vec!{Agent, Ascii}

/// ## ALCD
/// 
/// Alarm code byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - bit 8 = 1 - Alarm Set
/// - bit 8 = 0 - Alarm Cleared
/// - bit 7-1 - Alarm Category
///   - 0 - Not Used
///   - 1 - Personal Safety
///   - 2 - Equipment Safety
///   - 3 - Parameter Control Warning
///   - 4 - Parameter Control Error
///   - 5 - Irrecoverable Error
///   - 6 - Equipment Status Warning
///   - 7 - Attention Flags
///   - 8 - Data Integrity
///   - \>8 - Other Categories
///   - 9-63 - Reserved
/// 
/// TODO: Implement Set/Cleared and Category Manually?
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F6, S5F8
#[derive(Clone, Copy, Debug)]
pub struct AlarmCode(pub u8);
singleformat!{AlarmCode, Bin}

/// ## ALED
/// 
/// Alarm Enable/Disable Code, 1 Byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - Bit 8
///   - 0 = Disable Alarm
///   - 1 = Enable Alarm
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F3
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AlarmEnableDisable {
  Disable = 0,
  Enable = 128,
}
singleformat_enum!{AlarmEnableDisable, Bin}

/// ## ALID
/// 
/// Alarm identification.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F3, S5F5, S5F6, S5F8
#[derive(Clone, Copy, Debug)]
pub enum AlarmID {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{AlarmID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## ALTX
/// 
/// Alarm text, maximum 120 characters.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F6, S5F8
#[derive(Clone, Debug)]
pub struct AlarmText(Vec<Char>);
singleformat_vec!{AlarmText, Ascii, 0..=120, Char}

/// ## ATTRDATA
/// 
/// Specific attribute value for a specific object.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - [S3F35]
/// - S13F13, S13F16
/// - S14F1, S14F2, S14F3, S14F4, S14F9, S14F10, S14F11, S14F12, S14F13,
///   S14F14, S14F15, S14F16, S14F17, S14F18, S14F19
/// - S18F1, S18F3
/// 
/// [S1F20]: crate::messages::s1::AttributeData
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
pub enum AttributeValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{AttributeValue, List, Bin, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ATTRID
/// 
/// Identifier for an attribute for a type of object.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - [S3F35]
/// - S13F13, S13F16
/// - S14F1, S14F2, S14F3, S14F4, S14F8, S14F9, S14F10, S14F11, S14F12,
///   S14F13, S14F14, S14F15, S14F16, S14F17, S14F18, S14F19
/// - S18F1, S18F3
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
pub enum AttributeID {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{AttributeID, U1, U2, U4, U8}

/// ## ATTRRELN
/// 
/// The relationship between a qualyfing value and the value of an attribute
/// of an object instance (i.e. value of interest).
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S14F1
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AttributeRelation {
  /// ### EQUAL TO
  /// 
  /// The qualifying value is equal to the value of interest.
  EqualTo = 0,

  /// ### NOT EQUAL TO
  /// 
  /// The qualifying value is not equal to the value of interest.
  NotEqualTo = 1,

  /// ### LESS THAN
  /// 
  /// The qualifying value is less than the value of interest.
  LessThan = 2,

  /// ### LESS THAN OR EQUAL TO
  /// 
  /// The qualifying value is less than or equal to the value of interest.
  LessThanOrEqualTo = 3,

  /// ### GREATER THAN
  /// 
  /// The qualifying value is greater than the value of interest.
  GreaterThan = 4,

  /// ### GREATER THAN OR EQUAL TO
  /// 
  /// The qualifying value is greater than or equal to the value of interest.
  GreaterThanOrEqualTo = 5,

  /// ### PRESENT
  /// 
  /// The qualifying value is present in the set of the value of interest.
  Present = 6,

  /// ### ABSENT
  /// 
  /// The qualifying value is absent from the set of the value of interest.
  Absent = 7,
}
singleformat_enum!{AttributeRelation, U1}

/// ## BCDS
/// 
/// Before Command Codes
/// 
/// Vector of all command codes which the defined command must preceed within
/// the same block.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Debug)]
pub enum BeforeCommandCodes {
  I2(Vec<i16>),
  U2(Vec<u16>),
}
multiformat_vec!{BeforeCommandCodes, I2, U2}

/// ## BCEQU
/// 
/// Bin code equivalents.
/// 
/// Array of all codes that are to be processed.
/// 
/// Must be same format as [BINLT] and [NULBC].
/// 
/// Zero length indicates that all codes be sent.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F3, S12F4
/// 
/// [BINLT]: BinList
/// [NULBC]: NullBinCode
#[derive(Clone, Debug)]
pub enum BinCodeEquivalents {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{BinCodeEquivalents, Ascii, U1}

/// ## BINLT
/// 
/// The bin list.
/// 
/// Array of bin values.
/// 
/// Must be same format as [BCEQU] and [NULBC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F7, S12F9, S12F11, S12F14, S12F16, S12F18
/// 
/// [BCEQU]: BinCodeEquivalents
/// [NULBC]: NullBinCode
#[derive(Clone, Debug)]
pub enum BinList {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{BinList, Ascii, U1}

/// ## BLKDEF
/// 
/// Block Definition
/// 
/// Specifies whether a command being defined starts, terminates, or is
/// within the body of a block.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum BlockDefinition {
  /// ### TERMINATE
  /// 
  /// Command terminates a block body.
  Terminate = -1,

  /// ### WITHIN
  /// 
  /// Command neither starts or terminates a block body.
  Within = 0,

  /// ### START
  /// 
  /// Command starts a block body.
  Start = 1,
}
singleformat_enum!{BlockDefinition, I1}

/// ## BPD
/// 
/// Boot program data.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S8F2
#[derive(Clone, Debug)]
pub struct BootProgramData(pub Vec<u8>);
singleformat_vec!{BootProgramData, Bin}

// TODO: BYTMAX
// How to deal with negative values being invalid even though you can use signed int?

/// ## CAACK
/// 
/// **Carrier Action Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F18], [S3F20], [S3F22], [S3F24], [S3F26], [S3F28], [S3F30], [S3F32],
///   [S3F34]
/// 
/// [S3F18]: crate::messages::s3::CarrierActionAcknowledge
/// [S3F20]: crate::messages::s3::CancelAllCarrierOutAcknowledge
/// [S3F22]: crate::messages::s3::PortGroupDefinitionAcknowledge
/// [S3F24]: crate::messages::s3::PortGroupActionAcknowledge
/// [S3F26]: crate::messages::s3::PortActionAcknowledge
/// [S3F28]: crate::messages::s3::ChangeAccessAcknowledge
/// [S3F30]: crate::messages::s3::CarrierTagReadData
/// [S3F32]: crate::messages::s3::CarrierTagWriteDataAcknowledge
/// [S3F34]: crate::messages::s3::CancelAllPodOutAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CarrierActionAcknowledgeCode {
  Ok = 0,
  InvalidCommand = 1,
  CannotPerformNow = 2,
  InvalidData = 3,
  ActionWillBePerformed = 4,
  InvalidState = 5,
  PerformedWithErrors = 6,
}
singleformat_enum!{CarrierActionAcknowledgeCode, U1}

/// ## CARRIERACTION
/// 
/// Specifies the action requested for a carrier.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F17]
/// 
/// [S3F17]: crate::messages::s3::CarrierActionRequest
#[derive(Clone, Debug)]
pub struct CarrierAction(pub Vec<Char>);
singleformat_vec!{CarrierAction, Ascii}

/// ## CARRIERID
/// 
/// The identifier of a carrier.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F17]
/// - S16F11, S16F15
/// 
/// [S3F17]: crate::messages::s3::CarrierActionRequest
#[derive(Clone, Debug)]
pub struct CarrierID(pub Vec<Char>);
singleformat_vec!{CarrierID, Ascii}

/// ## CARRIERSPEC
/// 
/// **Carrier Specifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F29], [S3F31]
/// 
/// [OBJSPEC]: ObjectSpecifier
/// [S3F29]:   crate::messages::s3::CarrierTagReadRequest
/// [S3F31]:   crate::messages::s3::CarrierTagWriteDataRequest
pub type CarrierSpecifier = ObjectSpecifier;

/// ## CATTRDATA
/// 
/// **Carrier Attribute Data**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [ATTRDATA].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F17]
/// 
/// [ATTRDATA]: AttributeValue
/// [S3F17]:    crate::messages::s3::CarrierActionRequest
pub type CarrierAttributeValue = AttributeValue;

/// ## CATTRID
/// 
/// The name of a carrier attribute.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F17]
/// 
/// [S3F17]: crate::messages::s3::CarrierActionRequest
#[derive(Clone, Debug)]
pub struct CarrierAttributeID(pub Vec<Char>);
singleformat_vec!{CarrierAttributeID, Ascii}

/// ## CCODE
/// 
/// Command code.
/// 
/// Each command code corresponds to a unique process operation the machine
/// is capable of performing.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
#[derive(Clone, Debug)]
pub enum CommandCode {
  Ascii(Vec<Char>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  U2(Vec<u16>),
  U4(Vec<u32>),
}
multiformat_vec!{CommandCode, Ascii, I2, I4, U2, U4}

/// ## CEED
/// 
/// Collection event or trace enable/disable code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - False = Disable
/// - True = Enable
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F37]
/// - S17F5
/// 
/// [S2F37]: crate::messages::s2::EnableDisableEventReport
#[derive(Clone, Debug)]
pub struct CollectionEventEnableDisable(pub bool);
singleformat!{CollectionEventEnableDisable, Bool}

/// ## CEID
/// 
/// Collection event ID.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F23], [S1F24]
/// - [S2F35], [S2F37]
/// - S6F3, S6F8, S6F9, S6F11, S6F13, S6F15, S6F16, S6F17, S6F18
/// - S17F5, S17F9, S17F10, S17F11, S17F12
/// 
/// [S1F23]: crate::messages::s1::CollectionEventNamelistRequest
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
/// [S2F35]: crate::messages::s2::LinkEventReport
/// [S2F37]: crate::messages::s2::EnableDisableEventReport
pub enum CollectionEventID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{CollectionEventID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## CENAME
/// 
/// Collection event name.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F24]
/// 
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
#[derive(Clone, Debug)]
pub struct CollectionEventName(pub Vec<Char>);
singleformat_vec!{CollectionEventName, Ascii}

/// ## CEPACK
/// 
/// **Command Enhanced Paramater Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// Alternatively, if the [CEPVAL] of concern is of a list format, this item
/// will also be of a list format.
/// 
/// TODO: Implement list format.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F50]
/// 
/// [CEPVAL]: CommandEnhancedParameterValue
/// [S2F50]:  crate::messages::s2::EnhancedRemoteCommandAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandEnhancedParameterAcknowledgeCode {
  Ok = 0,
  ParameterNameDoesNotExist = 1,
  IllegalValue = 2,
  IllegalFormat = 3,
  ParameterNameNotValidAsUsed = 4,
}
singleformat_enum!{CommandEnhancedParameterAcknowledgeCode, U1}

/// ## CEPVAL
/// 
/// **Command Enhanced Parameter Value**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Allowed forms:
/// 
/// 1. A single non-list value.
/// 2. A list of single items of identical format.
/// 3. A list of items of the form of a list of two items containing another
///    name-value pair.
/// 
/// TODO: Enforce format.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F49]
/// 
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
pub enum CommandEnhancedParameterValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{CommandEnhancedParameterValue, List, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## CKPNT
/// 
/// Checkpoint as defined by the sending system.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F3, S13F6
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(pub u32);
singleformat!{Checkpoint, U4}

/// ## CMDA
/// 
/// Command acknowledge code.
/// 
/// TODO: Implement Format 31.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F22], [S2F28]
/// 
/// [S2F22]: crate::messages::s2::RemoteCommandAcknowledge
/// [S2F28]: crate::messages::s2::InitiateProcessingAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandAcknowledge {
  Ok = 0,
  CommandDoesNotExist = 1,
  CannotPerformNow = 2,
}
singleformat_enum!{CommandAcknowledge, U1}

// TODO: CMDMAX
// How to deal with negative values being invalid even though you can use signed int?

/// ## CNAME
/// 
/// Command name, maximum 16 characters.
/// 
/// A text string which is unique among other command names in a PCD, which
/// describes the processing done by the equipment for the corresponding
/// [CCODE].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
/// 
/// [CCODE]: CommandCode
#[derive(Clone, Debug)]
pub struct CommandName(Vec<Char>);
singleformat_vec!{CommandName, Ascii, 0..=16, Char}

/// ## COLCT
/// 
/// Column count, in die increments.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F1, S12F4
#[derive(Clone, Copy, Debug)]
pub enum ColumnCount {
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{ColumnCount, U1, U2, U4, U8}

/// ## COLHDR
/// 
/// Text description of contents of [TBLELT], 1-20 characters.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
/// 
/// [TBLELT]: TableElement
#[derive(Clone, Debug)]
pub struct ColumnHeader(Vec<Char>);
singleformat_vec!{ColumnHeader, Ascii, 1..=20, Char}

/// ## COMMACK
/// 
/// Establish Communications Acknowledge Code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F14]
/// 
/// [S1F14]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommAck {
  /// ### ACCEPTED
  Accepted = 0,

  /// ### DENIED
  Denied = 1,
}
singleformat_enum!{CommAck, Bin}

/// ## COMPARISONOPERATOR
/// 
/// Choice of available operators that compare the supplied value to the
/// current attribute value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S19F1
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ComparisonOperator {
  /// ### EQ
  /// 
  /// Equals, numeric or string.
  EqualTo = 0,

  /// ### NOTEQ
  /// 
  /// Not Equal, numeric or string.
  NotEqualTo = 1,

  /// ### LT
  /// 
  /// Less Than, numeric.
  LessThan = 2,

  /// ### LE
  /// 
  /// Less than or equal to, numeric.
  LessThanOrEqualTo = 3,

  /// ### GT
  /// 
  /// Greater than, numeric.
  GreaterThan = 4,

  /// ### GE
  /// 
  /// Greater than or equal to, numeric.
  GreaterThanOrEqualTo = 5,

  /// ### LIKE
  /// 
  /// Contains the substring, string.
  Like = 6,

  /// ### NOTLIKE
  /// 
  /// Does not contain the substring, string.
  NotLike = 7,
}
singleformat_enum!{ComparisonOperator, U1}

/// ## CONDITION
/// 
/// Provides condition information for a subsystem component.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [CONDITIONLIST]
/// 
/// [CONDITIONLIST]: ConditionList
#[derive(Clone, Debug)]
pub struct Condition(pub Vec<Char>);
singleformat_vec!{Condition, Ascii}

/// ## CONDITIONLIST
/// 
/// A list of [CONDITION] data sent in a fixed order.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S18F16
/// 
/// [CONDITION]: Condition
pub type ConditionList = VecList<Condition>;

/// ## CPACK
/// 
/// Command parameter acknowledge code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F42]
/// 
/// [S2F42]: crate::messages::s2::HostCommandAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandParameterAcknowledgeCode {
  /// CPNAME does not exist.
  ParameterNameDoesNotExist = 1,

  /// Illegal value specified for CPVAL.
  IllegalValue = 2,

  /// Illegal format specified for CPVAL.
  IllegalFormat = 3,
}
singleformat_enum!{CommandParameterAcknowledgeCode, Bin}

/// ## CPNAME
/// 
/// **Command Parameter Name**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F41], [S2F42], [S2F49], [S2F50]
/// - S4F21, S4F29
/// - S16F5, S16F27
/// 
/// [S2F41]: crate::messages::s2::HostCommandSend
/// [S2F42]: crate::messages::s2::HostCommandAcknowledge
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
/// [S2F50]: crate::messages::s2::EnhancedRemoteCommandAcknowledge
pub enum CommandParameterName {
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
}
multiformat_vec!{CommandParameterName, Ascii, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## CPVAL
/// 
/// **Command Parameter Value**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F41], [S2F49]
/// - S4F21, S4F29
/// - S16F5, S16F27
/// - S18F13
/// 
/// [S2F41]: crate::messages::s2::HostCommandSend
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
pub enum CommandParameterValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
}
multiformat_vec!{CommandParameterValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## CSAACK
/// 
/// Equipment acknowledge code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F8]
/// 
/// [S2F8]: crate::messages::s2::ServiceProgramRunAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ServiceAcknowledgeCode {
  Ok = 0,
  Busy = 1,
  InvalidSPID = 2,
  InvalidData = 3,
}
singleformat_enum!{ServiceAcknowledgeCode, Bin}

/// ## CTLJOBCMD
/// 
/// Control Job command code.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S16F27
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ControlJobCommand {
  /// ### CJStart
  Start = 1,

  /// ### CJPause
  Pause = 2,

  /// ### CJResume
  Resume = 3,

  /// ### CJCancel
  Cancel = 4,

  /// ### CJDeselect
  Deselect = 5,

  /// ### CJStop
  Stop = 6,

  /// ### CJAbort
  Abort = 7,

  /// ### CJHOQ
  HeadOfQueue = 8,
}
singleformat_enum!{ControlJobCommand, U1}

// TODO: CTLJOBID
// Something about OBJID.

/// ## DATA
/// 
/// A string of unformatted data.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F30], [S3F31]
/// - S18F6, S18F7
/// 
/// [S3F30]: crate::messages::s3::CarrierTagReadData
/// [S3F31]: crate::messages::s3::CarrierTagWriteDataRequest
#[derive(Clone, Debug)]
pub struct Data(pub Vec<Char>);
singleformat_vec!{Data, Ascii}

/// ## DATAACK
/// 
/// Data acknowledge code.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S14F22
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DataAcknowledge {
  Ok = 0,
  UnknownDataID = 1,
  InvalidParameter = 2,
}
singleformat_enum!{DataAcknowledge, Bin}

/// ## DATAID
/// 
/// **Data ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F33], [S2F35], [S2F39], [S2F45], [S2F49]
/// - [S3F15], [S3F17]
/// - S4F19, S4F25
/// - S6F3, S6F5, S6F7, S6F8, S6F9, S6F11, S6F13, S6F16, S6F18, S6F25, S6F27
/// - S13F11, S13F13, S13F15
/// - S14F19, S14F21, S14F23
/// - S15F1, S15F13, S15F15, S15F21, S15F23, S15F25, S15F27, S15F29, S15F33,
///   S15F35, S15F39, S15F41, S15F43, S15F45, S15F47, S15F49
/// - S16F1, S16F3, S16F5, S16F11, S16F15
/// - S17F1, S17F5, S17F9
/// 
/// [S2F33]: crate::messages::s2::DefineReport
/// [S2F35]: crate::messages::s2::LinkEventReport
/// [S2F39]: crate::messages::s2::MultiBlockInquire
/// [S2F45]: crate::messages::s2::DefineVariableLimitAttributes
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
/// [S3F15]: crate::messages::s3::MultiBlockInquire
/// [S3F17]: crate::messages::s3::CarrierActionRequest
#[derive(Clone, Debug)]
pub enum DataID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{DataID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## DATALENGTH
/// 
/// Total bytes to be sent.
/// 
/// TODO: Do negative numbers need to be restricted?
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F39]
/// - [S3F15], [S3F29], [S3F31]
/// - S4F25
/// - S6F5
/// - S13F11
/// - S14F23
/// - S16F1
/// - S18F5, S18F7
/// - S19F19
/// 
/// [S2F39]: crate::messages::s2::MultiBlockInquire
/// [S3F15]: crate::messages::s3::MultiBlockInquire
/// [S3F29]: crate::messages::s3::CarrierTagReadRequest
/// [S3F31]: crate::messages::s3::CarrierTagWriteDataRequest
#[derive(Clone, Debug)]
pub enum DataLength {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{DataLength, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## DATASEG
/// 
/// **Data Segment**
/// 
/// Identifies requested data.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F29], [S3F31]
/// - S18F5, S18F7
/// 
/// [S3F29]: crate::messages::s3::CarrierTagReadRequest
/// [S3F31]: crate::messages::s3::CarrierTagWriteDataRequest
#[derive(Clone, Debug)]
pub struct DataSegment(pub Vec<Char>);
singleformat_vec!{DataSegment, Ascii}

/// ## DRACK
/// 
/// **Define Report Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F34]
/// 
/// [S2F34]: crate::messages::s2::DefineReportAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DefineReportAcknowledgeCode {
  Ok = 0,
  InsufficientSpace = 1,
  InvalidFormat = 2,
  ReportAlreadyDefined = 3,
  VariableDoesNotExist = 4,
}
singleformat_enum!{DefineReportAcknowledgeCode, Bin}

/// ## DSPER
/// 
/// Data sample period.
/// 
/// TODO: Implement format restrictions.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// Format 1:
/// - hhmmss
///    - hh = Hours
///    - mm = Minutes
///    - ss = Seconds
/// 
/// Format 2:
/// - hhmmsscc
///    - hh = Hours
///    - mm = Minutes
///    - ss = Seconds
///    - cc = CentiSeconds
/// 
/// Equipment must implement Format 1, and may optionally implement Format 2.
/// 
/// Support for Format 2 does not necessitate a trace resolution of 0.01sec.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F23]
/// 
/// [S2F23]: crate::messages::s2::TraceInitializeSend
#[derive(Clone, Debug)]
pub struct DataSamplePeriod(pub Vec<Char>);
singleformat_vec!{DataSamplePeriod, Ascii}

/// ## DVVALNAME
/// 
/// Descriptive name for a data variable.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F22]
/// 
/// [S1F22]: crate::messages::s1::DataVariableNamelist
#[derive(Clone, Debug)]
pub struct DataVariableValueName(pub Vec<Char>);
singleformat_vec!{DataVariableValueName, Ascii}

/// ## EAC
/// 
/// EquipmentAcknowledgeCode, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F16]
/// 
/// [S2F16]: crate::messages::s2::NewEquipmentConstantAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum EquipmentAcknowledgeCode {
  Acknowledge = 0,
  DoesNotExist = 1,
  Busy = 2,
  OutOfRange = 3,
}
singleformat_enum!{EquipmentAcknowledgeCode, Bin}

/// ## ECDEF
/// 
/// Equipment constant default value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F30]
/// 
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
pub enum EquipmentConstantDefaultValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantDefaultValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECID
/// 
/// **Equipment Constant ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F13], [S2F15], [S2F29], [S2F30]
/// 
/// [S2F13]: crate::messages::s2::EquipmentConstantRequest
/// [S2F15]: crate::messages::s2::NewEquipmentConstantSend
/// [S2F29]: crate::messages::s2::EquipmentConstantNamelistRequest
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
#[derive(Clone, Debug)]
pub enum EquipmentConstantID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{EquipmentConstantID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## ECMAX
/// 
/// **Equipment Constant Maximum Value**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F30]
/// 
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
pub enum EquipmentConstantMaximumValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantMaximumValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECMIN
/// 
/// **Equipment Constant Minimum Value**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F30]
/// 
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
pub enum EquipmentConstantMinimumValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantMinimumValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECNAME
/// 
/// **Equipment Constant Name**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F30]
/// 
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
pub struct EquipmentConstantName(pub Vec<Char>);
singleformat_vec!{EquipmentConstantName, Ascii}

/// ## ECV
/// 
/// **Equipment Constant Value**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F14], [S2F15]
/// 
/// [S2F14]: crate::messages::s2::EquipmentConstantData
/// [S2F15]: crate::messages::s2::NewEquipmentConstantSend
#[derive(Clone, Debug)]
pub enum EquipmentConstantValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## EMID
/// 
/// **Equivalent Material ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Binary or ASCII, 16 bytes maximum.
/// 
/// TODO: Implement Binary.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used by
/// 
/// - [S3F9]
/// 
/// [S3F9]: crate::messages::s3::MaterialIDEquateSend
pub struct EquivalentMaterialID(Vec<Char>);
singleformat_vec!(EquivalentMaterialID, Ascii, 0..=16, Char);

/// ## ERRCODE
/// 
/// Code identifying an error.
/// 
/// TODO: Implement user defined errors.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - [S3F18], [S3F20], [S3F22], [S3F24], [S3F26], [S3F28], [S3F30], [S3F32],
///   [S3F34], [S3F36]
/// - S4F20, S4F22, S4F23, S4F31, S4F33
/// - S5F14, S5F15, S5F18
/// - S6F25, S6F30
/// - S13F14, S13F16
/// - S14F2, S14F4, S14F5, S14F6, S14F8, S14F10, S14F12,
///   S14F14, S14F16, S14F18, S14F20, S14F21, S14F26, S14F28
/// - S15F4, S15F6, S15F8, S15F10, S15F12, S15F14, S15F16,
///   S15F18, S15F20, S15F22, S15F24, S15F26, S15F28, S15F30,
///   S15F32, S15F34, S15F36, S15F38, S15F40, S15F42, S15F44,
///   S15F48, S15F53
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24,
///   S16F26, S16F28
/// - S17F2, S17F4, S17F6, S17F8, S17F10, S17F12, S17F14
/// 
/// [S1F20]: crate::messages::s1::AttributeData
/// [S3F18]: crate::messages::s3::CarrierActionAcknowledge
/// [S3F20]: crate::messages::s3::CancelAllCarrierOutAcknowledge
/// [S3F22]: crate::messages::s3::PortGroupDefinitionAcknowledge
/// [S3F24]: crate::messages::s3::PortGroupActionAcknowledge
/// [S3F26]: crate::messages::s3::PortActionAcknowledge
/// [S3F28]: crate::messages::s3::ChangeAccessAcknowledge
/// [S3F30]: crate::messages::s3::CarrierTagReadData
/// [S3F32]: crate::messages::s3::CarrierTagWriteDataAcknowledge
/// [S3F34]: crate::messages::s3::CancelAllPodOutAcknowledge
/// [S3F36]: crate::messages::s3::ReticleTransferJobAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum ErrorCode {
  NoError                         = 0,
  UnknownObjectInObjectSpecifier  = 1,
  UnknownTargetObjectType         = 2,
  UnknownObjectInstance           = 3,
  UnknownAttributeName            = 4,
  ReadonlyAttributeAccessDenied   = 5,
  UnknownObjectType               = 6,
  InvalidAttributeValue           = 7,
  SyntaxError                     = 8,
  VerificationError               = 9,
  ValidationError                 = 10,
  ObjectIdentifierInUse           = 11,
  ParametersImproperlySpecified   = 12,
  InsufficientParametersSpecified = 13,
  UnsupportedOptionRequested      = 14,
  Busy                            = 15,
  NotAvailableForProcessing       = 16,
  CommandNotValidForCurrentState  = 17,
  NoMaterialAltered               = 18,
  MaterialPartiallyProcessed      = 19,
  AllMaterialProcessed            = 20,
  RecipeSpecificationError        = 21,
  FailedDuringProcessing          = 22,
  FailedWhileNotProcessing        = 23,
  FailedDueToLackOfMaterial       = 24,
  JobAborted                      = 25,
  JobStopped                      = 26,
  JobCancelled                    = 27,
  CannotChangeSelectedRecipe      = 28,
  UnknownEvent                    = 29,
  DuplicateReportID               = 30,
  UnknownDataReport               = 31,
  DataReportNotLinked             = 32,
  UnknownTraceReport              = 33,
  DuplicateTraceID                = 34,
  TooManyDataReports              = 35,
  SamplePeriodOutOfRange          = 36,
  GroupSizeTooLarge               = 37,
  RecoveryActionCurrentlyInvalid  = 38,
  BusyWithAnotherRecovery         = 39,
  NoActiveRecoveryAction          = 40,
  ExceptionRecoveryFailed         = 41,
  ExceptionRecoveryAborted        = 42,
  InvalidTableElement             = 43,
  UnknownTableElement             = 44,
  CannotDeletePredefined          = 45,
  InvalidToken                    = 46,
  InvalidParameter                = 47,
  LoadPortDoesNotExist            = 48,
  LoadPortAlreadyInUse            = 49,
  MissingCarrier                  = 50,
  //51-63: Reserved
  //64-32767: User Defined
  ActionWillBePerformed           = 32768,
  ActionCannotBePerformedNow      = 32769,
  ActionFailedDueToErrors         = 32770,
  InvalidCommand                  = 32771,
  ClientAlr                       = 32772,
  DuplicateClientID               = 32773,
  InvalidClientType               = 32774,
  IncompatibleVersions            = 32775,
  UnrecognizedClientID            = 32776,
  FailedCompletedUnsuccessfully   = 32777,
  FailedUnsafe                    = 32778,
  SensorDetectedObstacle          = 32779,
  MaterialNotSent                 = 32780,
  MaterialNotReceived             = 32781,
  MaterialLost                    = 32782,
  HardwareFailure                 = 32783,
  TransferCancelled               = 32784,
  //32785-32789: Reserved for SEMI E127
  //32793-65335: Reserved
  //65536+: User Defined
}
impl From<ErrorCode> for Item {
  fn from(value: ErrorCode) -> Self {
    let number: u64 = value.into();
    if number < 256 {
      Item::U1(vec![number as u8])
    } else if number < 65536 {
      Item::U2(vec![number as u16])
    } else {
      Item::U8(vec![number])
    }
  }
}
impl TryFrom<Item> for ErrorCode {
  type Error = Error;

  fn try_from(value: Item) -> Result<Self, Self::Error> {
    match value {
      Item::U1(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U2(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U4(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U8(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0]).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      _ => Err(WrongFormat),
    }
  }
}

/// ## ERACK
/// 
/// **Enable/Disable Event Report Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F38]
/// 
/// [S2F38]: crate::messages::s2::EnableDisableEventReportAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum EnableDisableEventReportAcknowledgeCode {
  Ok = 0,
  CollectionEventDoesNotExist = 1,
}
singleformat_enum!{EnableDisableEventReportAcknowledgeCode, Bin}

/// ## ERRTEXT
/// 
/// Text string describing the error noted in the corresponding [ERRCODE].
/// 
/// Maximum 120 characters.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - [S3F18], [S3F20], [S3F22], [S3F24], [S3F26], [S3F28], [S3F30], [S3F32],
///   [S3F34], [S3F36]
/// - S4F20, S4F22, S4F23, S4F31, S4F33
/// - S5F14, S5F15, S5F18
/// - S6F25
/// - S13F14, S13F16
/// - S14F2, S14F4, S14F6, S14F8, S14F10, S14F12, S14F14, S14F16, S14F18,
///   S14F20, S14F21, S14F26, S14F28
/// - S15F4, S15F6, S15F8, S15F10, S15F12, S15F14, S15F16, S15F18, S15F20,
///   S15F22, S15F24, S15F26, S15F28, S15F30, S15F32, S15F34, S15F36, S15F38,
///   S15F40, S15F42, S15F44, S15F48, S15F53
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24, S16F26, S16F28
/// - S17F4, S17F8, S17F18
/// 
/// [ERRCODE]: ErrorCode
/// [S1F20]:   crate::messages::s1::AttributeData
/// [S3F18]: crate::messages::s3::CarrierActionAcknowledge
/// [S3F20]: crate::messages::s3::CancelAllCarrierOutAcknowledge
/// [S3F22]: crate::messages::s3::PortGroupDefinitionAcknowledge
/// [S3F24]: crate::messages::s3::PortGroupActionAcknowledge
/// [S3F26]: crate::messages::s3::PortActionAcknowledge
/// [S3F28]: crate::messages::s3::ChangeAccessAcknowledge
/// [S3F30]: crate::messages::s3::CarrierTagReadData
/// [S3F32]: crate::messages::s3::CarrierTagWriteDataAcknowledge
/// [S3F34]: crate::messages::s3::CancelAllPodOutAcknowledge
/// [S3F36]: crate::messages::s3::ReticleTransferJobAcknowledge
#[derive(Clone, Debug)]
pub struct ErrorText(Vec<Char>);
singleformat_vec!{ErrorText, Ascii, 0..=120, Char}

/// ## FCNID
/// 
/// **Function ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F43], [S2F44]
/// 
/// [S2F43]: crate::messages::s2::ResetSpoolingStreamsAndFunctions
/// [S2F44]: crate::messages::s2::ResetSpoolingAcknowledge
pub struct FunctionID(pub u8);
singleformat!{FunctionID, U1}

/// ## GRANT
/// 
/// Grant code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F2], [S2F40]
/// - [S3F16]
/// - S4F26
/// - S13F12
/// - S14F24
/// - S16F2
/// - S19F20
/// 
/// [S2F2]:  crate::messages::s2::ServiceProgramLoadGrant
/// [S2F40]: crate::messages::s2::MultiBlockGrant
/// [S3F16]: crate::messages::s3::MultiBlockGrant
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Grant {
  Granted = 0,
  Busy = 1,
  NoSpaceAvailable = 2,
  DuplicateDataID = 3,
}
singleformat_enum!{Grant, Bin}

/// ## HCACK
/// 
/// **Host Command Parameter Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F42], [S2F50]
/// 
/// [S2F42]: crate::messages::s2::HostCommandAcknowledge
/// [S2F50]: crate::messages::s2::EnhancedRemoteCommandAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum HostCommandAcknowledgeCode {
  Ok = 0,
  CommandDoesNotExist = 1,
  CannotPerformNow = 2,
  ParameterInvalid = 3,
  ToBeCompleted = 4,
  AlreadyInDesiredCondition = 5,
  ObjectDoesNotExist = 6,
}
singleformat_enum!{HostCommandAcknowledgeCode, Bin}

/// ## INPTN
/// 
/// **Input Port Number**
/// 
/// Specialized version of [PTN] indicating the input port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [PTN].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [PTN]:   PortNumber
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
pub type InputPortNumber = PortNumber;

/// ## JOBACTION
/// 
/// **Job Action**
/// 
/// Specifies the action of a reticle transfer job.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
pub struct JobAction(pub Vec<Char>);
singleformat_vec!{JobAction, Ascii}

/// ## LENGTH
/// 
/// Length of the service program or process program in bytes.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F1]
/// - S7F1, S7F29
/// 
/// [S2F1]: crate::messages::s2::ServiceProgramLoadInquire
#[derive(Clone, Copy, Debug)]
pub enum Length {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{Length, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## LIMITACK
/// 
/// **Variable Limit Attribute Set Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F46]
/// 
/// [S2F46]: crate::messages::s2::VariableLimitAttributeAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VariableLimitAttributeSetAcknowledgeCode {
  LimitIDDoesNotExist = 1,
  UpperDeadbandGreaterThanLimitMax = 2,
  LowerDeadbandLessThanLimitMin = 3,
  UpperDeadbandLessThanLowerDeadband = 4,
  IllegalFormat = 5,
  AsciiValueNonNumeric = 6,
  DuplicateLimitDefinition = 7,
}
singleformat_enum!{VariableLimitAttributeSetAcknowledgeCode, Bin}

/// ## LIMITID
/// 
/// **Limit Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identifier for a specific set of limits for a variable to which the
/// corresponding limit attributes refer.
/// 
/// Single-byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F45], [S2F46], [S2F48]
/// 
/// [S2F45]: crate::messages::s2::DefineVariableLimitAttributes
/// [S2F46]: crate::messages::s2::VariableLimitAttributeAcknowledge
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub struct LimitID(pub u8);
singleformat!{LimitID, Bin}

/// ## LIMITMAX
/// 
/// **Limit Maximum**
/// 
/// ----------------------------------------------------------------------------
/// 
/// The maximum allowed value for the limit values of a variable.
/// 
/// The format must match that of the specified variable.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F48]
/// 
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub enum LimitMaximum {
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{LimitMaximum, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## LIMITMIN
/// 
/// **Limit Minimum**
/// 
/// ----------------------------------------------------------------------------
/// 
/// The minimum allowed value for the limit values of a variable.
/// 
/// The format must match that of the specified variable.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F48]
/// 
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub enum LimitMinimum {
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{LimitMinimum, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## LOC
/// 
/// **Machine Material Location Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F27]
/// - [S3F2]
/// 
/// [S2F27]: crate::messages::s2::InitiateProcessingRequest
/// [S3F2]:  crate::messages::s3::MaterialStatusData
#[derive(Clone, Copy, Debug)]
pub struct LocationCode(pub u8);
singleformat!{LocationCode, Bin}

/// ## LOCID
/// 
/// **Material Location Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F29], [S3F31]
/// 
/// [S3F29]: crate::messages::s3::CarrierTagReadRequest
/// [S3F31]: crate::messages::s3::CarrierTagWriteDataRequest
#[derive(Clone, Debug)]
pub struct LocationID(pub Vec<Char>);
singleformat_vec!{LocationID, Ascii}

/// ## LOWERDB
/// 
/// **Lower Deadband**
/// 
/// Variable limit attribute which defines the lower boundary of the deadband
/// of a limit. The value applies to a single limit for a specified variable.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F45], [S2F48]
/// 
/// [S2F45]: crate::messages::s2::DefineVariableLimitAttributes
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub enum LowerDeadband {
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{LowerDeadband, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## LRACK
/// 
/// **Link Report Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F36]
/// 
/// [S2F36]: crate::messages::s2::LinkEventReportAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LinkReportAcknowledgeCode {
  Ok = 0,
  InsufficientSpace = 1,
  InvalidFormat = 2,
  CollectionEventLinkAlreadyDefined = 3,
  CollectionEventDoesNotExist = 4,
  ReportDoesNotExist = 5,
}
singleformat_enum!{LinkReportAcknowledgeCode, Bin}

/// ## LVACK
/// 
/// **Variable Limit Definition Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F46]
/// 
/// [S2F46]: crate::messages::s2::VariableLimitAttributeAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VariableLimitDefinitonAcknowledgeCode {
  VariableDoesNotExist = 1,
  VariableHasNoLimitsCapability = 2,
  VariableRepeatedInMessage = 3,
  LimitValueError = 4,
}
singleformat_enum!{VariableLimitDefinitonAcknowledgeCode, Bin}

/// ## MDLN
/// 
/// Equipment Model Type, 20 bytes max.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F2], [S1F13H], [S1F13E], [S1F14H], [S1F14E]
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
/// 
/// [S1F2]:   crate::messages::s1::OnLineDataEquipment
/// [S1F13H]: crate::messages::s1::HostCR
/// [S1F13E]: crate::messages::s1::EquipmentCR
/// [S1F14H]: crate::messages::s1::HostCRA
/// [S1F14E]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Debug)]
pub struct ModelName(Vec<Char>);
singleformat_vec!{ModelName, Ascii, 0..=20, Char}

/// ## MF
/// 
/// Material Format Code
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Either a single-byte enumerated value, or an ASCII formatted unit string.
/// 
/// TODO: Implement this variable using the units module.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F2], [S3F4], [S3F5], [S3F7]
/// - S16F3, S16F11, S16F15
/// 
/// [S3F2]: crate::messages::s3::MaterialStatusData
/// [S3F4]: crate::messages::s3::TimeToCompletionData
/// [S3F5]: crate::messages::s3::MaterialFoundSend
/// [S3F7]: crate::messages::s3::MaterialLostSend
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum MaterialFormat {
  Unit(Vec<Char>) = 0,
  Wafers          = 1,
  Cassettes       = 2,
  Dies            = 3,
  Boats           = 4,
  Ingots          = 5,
  LeadFrames      = 6,
  Lots            = 7,
  Magazines       = 8,
  Packages        = 9,
  Plates          = 10,
  Tubes           = 11,
  WaferFrames     = 12,
  Carriers        = 13,
  Substrates      = 14,
}
impl From<MaterialFormat> for Item {
  fn from(value: MaterialFormat) -> Self {
    match value {
      MaterialFormat::Unit(vec)   => Item::Ascii(vec),
      MaterialFormat::Wafers      => Item::Bin(vec![1]),
      MaterialFormat::Cassettes   => Item::Bin(vec![2]),
      MaterialFormat::Dies        => Item::Bin(vec![3]),
      MaterialFormat::Boats       => Item::Bin(vec![4]),
      MaterialFormat::Ingots      => Item::Bin(vec![5]),
      MaterialFormat::LeadFrames  => Item::Bin(vec![6]),
      MaterialFormat::Lots        => Item::Bin(vec![7]),
      MaterialFormat::Magazines   => Item::Bin(vec![8]),
      MaterialFormat::Packages    => Item::Bin(vec![9]),
      MaterialFormat::Plates      => Item::Bin(vec![10]),
      MaterialFormat::Tubes       => Item::Bin(vec![11]),
      MaterialFormat::WaferFrames => Item::Bin(vec![12]),
      MaterialFormat::Carriers    => Item::Bin(vec![13]),
      MaterialFormat::Substrates  => Item::Bin(vec![14]),
    }
  }
}
impl TryFrom<Item> for MaterialFormat {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::Ascii(vec) => Ok(MaterialFormat::Unit(vec)),
      Item::Bin(vec) => {
        if vec.len() == 1 {
          match vec[0] {
            1  => Ok(MaterialFormat::Wafers),
            2  => Ok(MaterialFormat::Cassettes),
            3  => Ok(MaterialFormat::Dies),
            4  => Ok(MaterialFormat::Boats),
            5  => Ok(MaterialFormat::Ingots),
            6  => Ok(MaterialFormat::LeadFrames),
            7  => Ok(MaterialFormat::Lots),
            8  => Ok(MaterialFormat::Magazines),
            9  => Ok(MaterialFormat::Packages),
            10 => Ok(MaterialFormat::Plates),
            11 => Ok(MaterialFormat::Tubes),
            12 => Ok(MaterialFormat::WaferFrames),
            13 => Ok(MaterialFormat::Carriers),
            14 => Ok(MaterialFormat::Substrates),
            _  => Err(WrongFormat),
          }
        } else {
          Err(WrongFormat)
        }
      },
      _ => Err(WrongFormat),
    }
  }
}

/// ## MID
/// 
/// Material ID.
/// 
/// Maximum 80 characters.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F27]
/// - [S3F2], [S3F4], [S3F7], [S3F9], [S3F12], [S3F13]
/// - S4F1, S4F3, S4F5, S4F7, S4F9, S4F11, S4F13, S4F15, S4F17
/// - S7F7, S7F8, S7F10, S7F11, S7F13, S7F35, S7F36
/// - S12F1, S12F3, S12F4, S12F5, S12F7, S12F9, S12F11, S12F13, S12F14, S12F15
///   S12F16, S12F17, S12F18
/// - S16F3, S16F11, S16F15
/// - S18F10, S18F11, S18F16
/// 
/// [S2F27]: crate::messages::s2::InitiateProcessingRequest
/// [S3F2]:  crate::messages::s3::MaterialStatusData
/// [S3F4]:  crate::messages::s3::TimeToCompletionData
/// [S3F7]:  crate::messages::s3::MaterialLostSend
/// [S3F9]:  crate::messages::s3::MaterialIDEquateSend
/// [S3F12]: crate::messages::s3::MaterialIDRequestAcknowledge
/// [S3F13]: crate::messages::s3::MaterialIDSend
pub struct MaterialID(Vec<Char>);
singleformat_vec!{MaterialID, Ascii, 0..=80, Char}

/// ## MIDAC
/// 
/// **Material ID Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F14]
/// 
/// [S3F14]: crate::messages::s3::MaterialIDAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MaterialIDAcknowledgeCode {
  Accepted = 0,
  InvalidPortNumber = 1,
  MaterialNotPresent = 2,
}
singleformat_enum!{MaterialIDAcknowledgeCode, Bin}

/// ## MIDRA
/// 
/// **Material ID Request Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F12]
/// 
/// [S3F12]: crate::messages::s3::MaterialIDRequestAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MaterialIDRequestAcknowledgeCode {
  MaterialIDFollows = 0,
  MaterialIDDenied = 1,
  MaterialIDLater = 2,
}
singleformat_enum!{MaterialIDRequestAcknowledgeCode, Bin}

/// ## NULBC
/// 
/// Null bin code value.
/// 
/// Used to indicate no die at a location.
/// 
/// Must be the same format as [BCEQU] and [BINLT].
/// 
/// Zero length indicates not used.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F1, S12F3, S12F4
/// 
/// [BCEQU]: BinCodeEquivalents
/// [BINLT]: BinList
pub enum NullBinCode {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{NullBinCode, Ascii, U1}

/// ## OBJID
/// 
/// Identifier for an object.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - S14F1, S14F2, S14F3, S14F4
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
pub enum ObjectID {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ObjectID, U1, U2, U4, U8}

/// ## OBJSPEC
/// 
/// **Object Specifier**
/// 
/// Text string that has an internal format and that is used to point to a
/// specific object instance.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string, formed out of a sequence of formatted substrings, each
/// specifying an object's type, and name. The substring format has the
/// following four parts:
/// 
/// 1. Object Type
/// 2. Colon Character ':'
/// 3. Object Name
/// 4. Greater-Than Symbol '>'
/// 
/// The Object Type may be ommitted (along with the Colon Character), if it is
/// not necessary to uniquely identify an object. The final Greater-Than Symbol
/// is optional.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F49]
/// - S13F11, S13F13, S13F15
/// - S14F1, S14F3, S14F5, S14F7, S14F9, S14F10, S14F11, S14F13, S14F15,
///   S14F17, S14F19, S14F25, S14F27
/// - S15F7, S15F23, S15F43, S15F47
/// 
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
#[derive(Clone, Debug)]
pub struct ObjectSpecifier(pub Vec<(Option<Vec<Char>>, Vec<Char>)>);
impl From<ObjectSpecifier> for Item {
  fn from(object_specifier: ObjectSpecifier) -> Self {
    let mut output: Vec<Char> = vec![];
    for object_id in object_specifier.0 {
      if let Some(object_type) = object_id.0 {
        output.extend(object_type);
        output.push(Char::Colon);
      }
      output.extend(object_id.1);
      output.push(Char::GreaterThanSign);
    }
    Item::Ascii(output)
  }
}
impl TryFrom<Item> for ObjectSpecifier {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::Ascii(vec) => {
        let mut output: Vec<(Option<Vec<Char>>, Vec<Char>)> = vec![];
        if vec.len() > 0 {
          let mut temp_a: Vec<Char> = vec![];
          let mut temp_b: Vec<Char> = vec![];
          let mut colon_seen: bool = false;
          for char in vec {
            if char == Char::Colon {
              if temp_a.len() == 0 || colon_seen {
                return Err(WrongFormat)
              } else {
                colon_seen = true;
              }
            } else if char == Char::GreaterThanSign {
              if colon_seen {
                if temp_b.len() == 0 {
                  return Err(WrongFormat)
                } else {
                  output.push((Some(temp_a), temp_b));
                  colon_seen = false;
                  temp_a = vec![];
                  temp_b = vec![];
                }
              } else {
                if temp_a.len() == 0 {
                  return Err(WrongFormat)
                } else {
                  output.push((None, temp_a));
                  colon_seen = false;
                  temp_a = vec![];
                }
              }
            } else {
              if colon_seen {
                temp_b.push(char);
              } else {
                temp_a.push(char);
              }
            }
          }
          if temp_a.len() > 0 {
            if colon_seen {
              if temp_b.len() == 0 {
                return Err(WrongFormat)
              } else {
                output.push((Some(temp_a), temp_b))
              }
            } else {
              output.push((None, temp_a));
            }
          }
        }
        Ok(ObjectSpecifier(output))
      },
      _ => Err(WrongFormat)
    }
  }
}

/// ## OBJTYPE
/// 
/// **Object Type**
/// 
/// An identifier for a class of objects; all objects of the same type must have
/// the same set of attributes.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - S14F1, S14F3, S14F6, S14F7, S14F8, S14F9, S14F25, S14F26, S14F27
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
#[derive(Clone, Debug)]
pub enum ObjectType {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ObjectType, U1, U2, U4, U8}

/// ## OFLACK
/// 
/// Acknowledge code for OFF-LINE request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F16]
/// 
/// [S1F16]: crate::messages::s1::OffLineAck
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OffLineAcknowledge {
  Acknowledge = 0,
}
singleformat_enum!{OffLineAcknowledge, Bin}

/// ## ONLACK
/// 
/// Acknowledge code for ON-LINE request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F18]
/// 
/// [S1F18]: crate::messages::s1::OnLineAck
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OnLineAcknowledge {
  Accepted      = 0,
  NotAllowed    = 1,
  AlreadyOnLine = 2,
}
singleformat_enum!{OnLineAcknowledge, Bin}

/// ## OUTPTN
/// 
/// **Output Port Number**
/// 
/// Specialized version of [PTN] indicating the output port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [PTN].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [PTN]:   PortNumber
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
pub type OutputPortNumber = PortNumber;

/// ## PARAMNAME
/// 
/// **Parameter Name**
/// 
/// The name of a parameter in a request.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F23], [S3F25]
/// 
/// [S3F23]: crate::messages::s3::PortGroupActionRequest
/// [S3F25]: crate::messages::s3::PortActionRequest
pub struct ParameterName(pub Vec<Char>);
singleformat_vec!{ParameterName, Ascii}

/// ## PARAMVAL
/// 
/// **Parameter Value**
/// 
/// The value of a parameter named by [PARAMNAME].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// TODO: Values that are lists are restricted to lists of single items of the
/// same format type.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F23], [S3F25]
/// 
/// [PARAMNAME]: ParameterName
/// [S3F23]:     crate::messages::s3::PortGroupActionRequest
/// [S3F25]:     crate::messages::s3::PortActionRequest
#[derive(Clone, Debug)]
pub enum ParameterValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{ParameterValue, List, Bin, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## PODID
/// 
/// **Pod Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [OBJSPEC]: ObjectSpecifier
/// [S3F35]:   crate::messages::s3::ReticleTransferJobRequest
pub type PodID = ObjectSpecifier;

/// ## PPID
/// 
/// Process Program ID
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Maximum 120 bytes.
/// 
/// Format is host dependent. For the internal use of the equipment, it can be
/// treated as a unique binary pattern. If the equipment is not prepared to
/// display the transmitted code, it should be displayed in hexadecimal.
/// 
/// TODO: Implement format 10.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F27]
/// - S7F1, S7F3, S7F5, S7F6, S7F8, S7F10, S7F11, S7F13, S7F17, S7F20, S7F23,
///   S7F25, S7F26, S7F27, S7F31, S7F33, S7F34, S7F36, S7F39, S7F43
/// 
/// [S2F27]: crate::messages::s2::InitiateProcessingRequest
pub struct ProcessProgramID(Vec<Char>);
singleformat_vec!{ProcessProgramID, Ascii, 0..=120, Char}

/// ## PORTACTION
/// 
/// **Port Action**
/// 
/// The action to be performed on a port.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F25]
/// 
/// [S3F25]: crate::messages::s3::PortActionRequest
#[derive(Clone, Debug)]
pub struct PortAction(pub Vec<Char>);
singleformat_vec!{PortAction, Ascii}

/// ## PGRPACTION
/// 
/// **Port Group Action**
/// 
/// The action to be performed on a port group.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F23]
/// 
/// [S3F23]: crate::messages::s3::PortGroupActionRequest
#[derive(Clone, Debug)]
pub struct PortGroupAction(pub Vec<Char>);
singleformat_vec!{PortGroupAction, Ascii}

/// ## PORTGRPNAME
/// 
/// **Port Group Name**
/// 
/// The identifier of a group of ports.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// ASCII string.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F21], [S3F23]
/// 
/// [S3F21]: crate::messages::s3::PortGroupDefinition
/// [S3F23]: crate::messages::s3::PortGroupActionRequest
#[derive(Clone, Debug)]
pub struct PortGroupName(pub Vec<Char>);
singleformat_vec!{PortGroupName, Ascii}

/// ## PTN
/// 
/// **Material Port Number**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F11], [S3F12], [S3F13], [S3F17], [S3F21], [S3F25], [S3F27], [S3F28]
/// - S4F1, S4F3, S4F5, S4F7, S4F9, S4F11, S4F13, S4F15, S4F17
/// 
/// [S3F11]: crate::messages::s3::MaterialIDRequest
/// [S3F12]: crate::messages::s3::MaterialIDRequestAcknowledge
/// [S3F13]: crate::messages::s3::MaterialIDSend
/// [S3F17]: crate::messages::s3::CarrierActionRequest
/// [S3F21]: crate::messages::s3::PortGroupDefinition
/// [S3F25]: crate::messages::s3::PortActionRequest
/// [S3F27]: crate::messages::s3::ChangeAccess
/// [S3F28]: crate::messages::s3::ChangeAccessAcknowledge
#[derive(Clone, Copy, Debug)]
pub enum PortNumber {
  Bin(u8),
  U1(u8),
}
multiformat!{PortNumber, Bin, U1}

/// ## QUA
/// 
/// **Quantity**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F2], [S3F4], [S3F5], [S3F7]
/// 
/// [S3F2]: crate::messages::s3::MaterialStatusData
/// [S3F4]: crate::messages::s3::TimeToCompletionData
/// [S3F5]: crate::messages::s3::MaterialFoundSend
/// [S3F7]: crate::messages::s3::MaterialLostSend
#[derive(Clone, Copy, Debug)]
pub struct Quantity(pub u8);
singleformat!{Quantity, U1}

/// ## RAC
/// 
/// Reset acknowledge code, 1 byte.
/// 
/// TODO: Implement Format 31.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F20]
/// 
/// [S2F20]: crate::messages::s2::ResetAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetAcknowledgeCode {
  Ok = 0,
  Denied = 1,
}
singleformat_enum!{ResetAcknowledgeCode, U1}

/// ## RCMD
/// 
/// Remote command.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F21], [S2F41], [S2F49]
/// 
/// [S2F21]: crate::messages::s2::RemoteCommandSend
/// [S2F41]: crate::messages::s2::HostCommandSend
/// [S2F49]: crate::messages::s2::EnhancedRemoteCommand
#[derive(Clone, Debug)]
pub enum RemoteCommand {
  Ascii(Vec<Char>),
  I1(i8),
  U1(u8),
}
multiformat_ascii!{RemoteCommand, I1, U1}

/// ## RCPID
/// 
/// **Recipe Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S15F21, S15F23, S15F28, S15F29, S15F30, S15F33, S15F35, S15F37, S15F41,
///   S15F44, S15F43
/// 
/// [OBJSPEC]: ObjectSpecifier
pub type RecipeID = ObjectSpecifier;

/// ## REPGSZ
/// 
/// Reporting group size.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F23]
/// - S17F5
/// 
/// [S2F23]: crate::messages::s2::TraceInitializeSend
#[derive(Clone, Debug)]
pub enum ReportingGroupSize {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ReportingGroupSize, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## RETICLEID
/// 
/// **Reticle Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [OBJSPEC]: ObjectSpecifier
/// [S3F35]:   crate::messages::s3::ReticleTransferJobRequest
pub type ReticleID = ObjectSpecifier;

/// ## RETPLACEINSTR
/// 
/// **Reticle Place Instruction**
/// 
/// Indicates which pod slots will have reticles placed.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ReticlePlaceInstruction {
  Place = 0,
  PassBy = 1,
  CurrentlyOccupied = 2,
}
singleformat_enum!{ReticlePlaceInstruction, U1}

/// ## RETREMOVEINSTR
/// 
/// **Reticle Remove Instruction**
/// 
/// Indicates which pod slots will have reticles removed.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F35]
/// 
/// [S3F35]: crate::messages::s3::ReticleTransferJobRequest
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ReticleRemoveInstruction {
  Remove = 0,
  PassBy = 1,
}
singleformat_enum!{ReticleRemoveInstruction, U1}

/// ## RIC
/// 
/// Reset code, 1 byte.
/// 
/// TODO: Implement Format 31.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F19]
/// 
/// [S2F19]: crate::messages::s2::ResetInitializeSend
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetCode {
  NotUsed = 0,
  PowerUpReset = 1,
}
singleformat_enum!{ResetCode, U1}

/// ## RPMACK
/// 
/// **Reticle Pod Management Service Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F36
/// 
/// [S3F36]: crate::messages::s3::ReticleTransferJobAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ReticlePodManagementAcknowledgeCode {
  Ok = 0,
  ServiceDoesNotExist = 1,
  CannotPerformNow = 2,
  ParameterDoesNotExist = 3,
  ToBeCompleted = 4,
  ServiceFailed = 5,
  ObjectDoesNotExist = 6,
}
singleformat_enum!{ReticlePodManagementAcknowledgeCode, U1}

/// ## RPTID
/// 
/// **Report ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F33], [S2F35]
/// - S6F11, S6F13, S6F16, S6F18, S6F19, S6F21, S6F27, S6F30
/// - S17F1, S17F2, S17F3, S17F4, S17F5, S17F9, S17F11, S17F12
/// 
/// [S2F33]: crate::messages::s2::DefineReport
/// [S2F35]: crate::messages::s2::LinkEventReport
#[derive(Clone, Debug)]
pub enum ReportID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ReportID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## RSPACK
/// 
/// **Reset Spooling Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F44]
/// 
/// [S2F44]: crate::messages::s2::ResetSpoolingAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetSpoolingAcknowledgeCode {
  Ok = 0,
  Rejected = 1,
}
singleformat_enum!{ResetSpoolingAcknowledgeCode, Bin}

/// ## SFCD
/// 
/// Status form code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F5], [S1F7]
/// 
/// [S1F5]: crate::messages::s1::FormattedStatusRequest
/// [S1F7]: crate::messages::s1::FixedFormRequest
#[derive(Clone, Copy, Debug)]
pub struct StatusFormCode(pub u8);
singleformat!{StatusFormCode, Bin}

/// ## SOFTREV
/// 
/// Software Revision Code, 20 bytes max.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F2E], [S1F13H], [S1F13E], [S1F14H], [S1F14E]
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
/// 
/// [S1F2E]:  crate::messages::s1::OnLineDataEquipment
/// [S1F13H]: crate::messages::s1::HostCR
/// [S1F13E]: crate::messages::s1::EquipmentCR
/// [S1F14H]: crate::messages::s1::HostCRA
/// [S1F14E]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Debug)]
pub struct SoftwareRevision(Vec<Char>);
singleformat_vec!{SoftwareRevision, Ascii, 0..=20, Char}

/// ## SPAACK
/// 
/// Service program acknowledge code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F4]
/// 
/// [S2F4]: crate::messages::s2::ServiceProgramSendAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ServiceProgramAcknowledge {
  Ok = 0,
  InvalidData = 1,
}
singleformat_enum!{ServiceProgramAcknowledge, Bin}

/// ## SPD
/// 
/// Service program data.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F3], [S2F6]
/// 
/// [S2F3]: crate::messages::s2::ServiceProgramSend
/// [S2F6]: crate::messages::s2::ServiceProgramLoadData
#[derive(Clone, Debug)]
pub struct ServiceProgramData(pub Vec<u8>);
singleformat_vec!{ServiceProgramData, Bin}

/// ## SPID
/// 
/// Service program ID, 6 characters.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F1], [S2F4], [S2F7], [S2F9], [S2F12]
/// 
/// [S2F1]:  crate::messages::s2::ServiceProgramLoadInquire
/// [S2F4]:  crate::messages::s2::ServiceProgramSendAcknowledge
/// [S2F7]:  crate::messages::s2::ServiceProgramRunSend
/// [S2F9]:  crate::messages::s2::ServiceProgramResultsRequest
/// [S2F12]: crate::messages::s2::ServiceProgramDirectoryData
#[derive(Clone, Copy, Debug)]
pub struct ServiceProgramID(pub [Char; 6]);
impl From<ServiceProgramID> for Item {
  fn from(value: ServiceProgramID) -> Self {
    let mut vec = vec![];
    vec.extend_from_slice(&value.0);
    Item::Ascii(vec)
  }
}
impl TryFrom<Item> for ServiceProgramID {
  type Error = Error;
  
  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::Ascii(vec) => {
        if vec.len() == 6 {
          Ok(Self(vec[0..6].try_into().unwrap()))
        } else {
          Err(WrongFormat)
        }
      },
      _ => Err(WrongFormat),
    }
  }
}

/// ## SPR
/// 
/// Service program results.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F10]
/// 
/// [S2F10]: crate::messages::s2::ServiceProgramResultsData
pub type ServiceProgramResults = Item;

/// ## STRACK
/// 
/// **Spool Stream Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F44]
/// 
/// [S2F44]: crate::messages::s2::ResetSpoolingAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SpoolStreamAcknowledgeCode {
  SpoolingDisallowed = 1,
  StreamUnknown = 2,
  FunctionUnknown = 3,
  SecondaryFunctionDisallowed = 4,
}
singleformat_enum!{SpoolStreamAcknowledgeCode, Bin}

/// ## STRID
/// 
/// **Stream ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F43], [S2F44]
/// 
/// [S2F43]: crate::messages::s2::ResetSpoolingStreamsAndFunctions
/// [S2F44]: crate::messages::s2::ResetSpoolingAcknowledge
pub struct StreamID(pub u8);
singleformat!{StreamID, U1}

/// ## SV
/// 
/// Status variable value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F4]
/// - S6F1
/// 
/// [S1F4]: crate::messages::s1::SelectedEquipmentStatusData
#[derive(Clone, Debug)]
pub enum StatusVariableValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{StatusVariableValue, List, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## SVID
/// 
/// Status variable ID.
/// 
/// TODO: Add ASCII.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F3], [S1F11], [S1F12]
/// - [S2F23]
/// 
/// [S1F3]:  crate::messages::s1::SelectedEquipmentStatusRequest
/// [S1F11]: crate::messages::s1::StatusVariableNamelistRequest
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
/// [S2F23]: crate::messages::s2::TraceInitializeSend
#[derive(Clone, Copy, Debug)]
pub enum StatusVariableID {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{StatusVariableID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## SVNAME
/// 
/// Status variable name.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F12]
/// 
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
#[derive(Clone, Debug)]
pub struct StatusVariableName(pub Vec<Char>);
singleformat_vec!{StatusVariableName, Ascii}

/// ## TARGETID
/// 
/// **Target Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S18F1, S18F2, S18F3, S18F4, S18F5, S18F6, S18F7, S18F8, S18F9, S18F10,
///   S18F11, S18F12, S18F13, S18F14, S18F15, S18F16
/// 
/// [OBJSPEC]: ObjectSpecifier
pub type TargetID = ObjectSpecifier;

/// ## TBLELT
/// 
/// Table element.
/// 
/// The first table element in a row is used to identify the row.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
pub enum TableElement {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{TableElement, List, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## TBLID
/// 
/// **Table Identifier**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJSPEC].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
/// 
/// [OBJSPEC]: ObjectSpecifier
pub type TableID = ObjectSpecifier;

/// ## TBLTYP
/// 
/// **Table Type**
/// 
/// A reserved text string to denot the format and application of a table.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Identical to [OBJTYPE].
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
/// 
/// [OBJTYPE]: ObjectType
pub type TableType = ObjectType;

/// ## TIAACK
/// 
/// Equipment acknowledge code, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F24]
/// 
/// [S2F24]: crate::messages::s2::TraceInitializeAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TraceInitializeAcknowledgeCode {
  Ok = 0,
  TooManySVID = 1,
  TooManyTraces = 2,
  InvalidPeriod = 3,
  UnknownSVID = 4,
  InvalidREPGSZ = 5,
}
singleformat_enum!{TraceInitializeAcknowledgeCode, Bin}

/// ## TIACK
/// 
/// **Time Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// 1 Byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F32]
/// 
/// [S2F32]: crate::messages::s2::DateTimeSetAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TimeAcknowledgeCode {
  Ok = 0,
  ErrorNotDone = 1,
}
singleformat_enum!{TimeAcknowledgeCode, Bin}

/// ## TIME
/// 
/// Time of day.
/// 
/// TODO: Implement specific format restrictions.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// 12-byte format:
/// - YYMMDDhhmmss
///    - YY = Year,   00 to 99
///    - MM = Month,  01 to 12
///    - DD = Day,    01 to 31
///    - hh = Hour,   00 to 23
///    - mm = Minute, 00 to 59
///    - ss = Second, 00 to 59
/// 
/// 16-byte format:
/// - YYYYMMDDhhmmsscc
///    - YYYY = Year,      0000 to 9999
///    -   MM = Month,       01 to   12
///    -   DD = Day,         01 to   31
///    -   hh = Hour,        00 to   23
///    -   mm = Minute,      00 to   59
///    -   ss = Second,      00 to   59
///    -   cc = Centisecond, 00 to   99
/// 
/// Extended format (Maximum 32 Bytes)
/// - YYYY-MM-DDThh:mm:ss.sTZD
///    - YYYY = Year,     0000 to 9999
///    -   MM = Month,      01 to   12
///    -   DD = Day,        01 to   31
///    -    T = Special Separator
///    -   hh = Hour,       00 to   23
///    -   mm = Minute,     00 to   59
///    -   ss = Second,     00 to   59
///    -   .s = Fraction,  One to Six Digits
///    -  TZD = Time Zone Designator
///       - Local Time: +hh:mm or -hh:mm
///       - UTC: Z 
/// - See SEMI E148 for more information.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F18], [S2F31]
/// 
/// [S2F18]: crate::messages::s2::DateTimeData
/// [S2F31]: crate::messages::s2::DateTimeSetRequest
#[derive(Clone, Debug)]
pub struct Time(pub Vec<Char>);
singleformat_vec!{Time, Ascii}

/// ## TOTSMP
/// 
/// Total samples to be made.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F23]
/// - S17F5
/// 
/// [S2F23]: crate::messages::s2::TraceInitializeSend
#[derive(Clone, Debug)]
pub enum TotalSamples {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{TotalSamples, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## TRID
/// 
/// Trace request ID.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F23]
/// - S6F1, S6F27, S6F28, S6F29, S6F30
/// - S17F5, S17F6, S17F7, S17F8, S17F13, S17F14
/// 
/// [S2F23]: crate::messages::s2::TraceInitializeSend
#[derive(Clone, Debug)]
pub enum TraceRequestID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{TraceRequestID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## TSIP
/// 
/// Transfer status of input port, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F10]
/// 
/// [S1F10]: crate::messages::s1::MaterialTransferStatusData
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TransferStatusInputPort {
  Idle            = 1,
  Prep            = 2,
  TrackOn         = 3,
  StuckInReceiver = 4,
}
singleformat_enum!{TransferStatusInputPort, Bin}

/// ## TSOP
/// 
/// Transfer status of output port, 1 byte.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F10]
/// 
/// [S1F10]: crate::messages::s1::MaterialTransferStatusData
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TransferStatusOutputPort {
  Idle          = 1,
  Prep          = 2,
  TrackOn       = 3,
  StuckInSender = 4,
  Completed     = 5,
}
singleformat_enum!{TransferStatusOutputPort, Bin}

/// ## TTC
/// 
/// **Time To Completion**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S3F4]
/// 
/// [S3F4]: crate::messages::s3::TimeToCompletionData
#[derive(Clone, Copy, Debug)]
pub enum TimeToCompletion{
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{TimeToCompletion, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## UNITS
/// 
/// Units identifier.
/// 
/// TODO: Implement this variable using the units module.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F12], [S1F22]
/// - [S2F30], [S2F38]
/// - S7F22
/// 
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
/// [S1F22]: crate::messages::s1::DataVariableNamelist
/// [S2F30]: crate::messages::s2::EquipmentConstantNamelist
/// [S2F38]: crate::messages::s2::EnableDisableEventReportAcknowledge
pub struct Units(pub Vec<Char>);
singleformat_vec!{Units, Ascii}

/// ## UPPERDB
/// 
/// **Upper Deadband**
/// 
/// Variable limit attribute which defines the upper boundary of the deadband
/// of a limit. The value applies to a single limit for a specified variable.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F45], [S2F48]
/// 
/// [S2F45]: crate::messages::s2::DefineVariableLimitAttributes
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub enum UpperDeadband {
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{UpperDeadband, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## VID
/// 
/// **Variable ID**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F21], [S1F22], [S1F24]
/// - [S2F33], [S2F45], [S2F46], [S2F47], [S2F48]
/// - S6F13, S6F18, S6F22
/// - S16F9
/// - S17F1
/// 
/// [S1F21]: crate::messages::s1::DataVariableNamelistRequest
/// [S1F22]: crate::messages::s1::DataVariableNamelist
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
/// [S2F33]: crate::messages::s2::DefineReport
/// [S2F45]: crate::messages::s2::DefineVariableLimitAttributes
/// [S2F46]: crate::messages::s2::VariableLimitAttributeAcknowledge
/// [S2F47]: crate::messages::s2::VariableLimitAttributeRequest
/// [S2F48]: crate::messages::s2::VariableLimitAttributeSend
pub enum VariableID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{VariableID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## VLAACK
/// 
/// **Variable Limit Attribute Acknowledge Code**
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Single-byte enumerated value.
/// 
/// ----------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S2F46]
/// 
/// [S2F46]: crate::messages::s2::VariableLimitAttributeAcknowledge
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VariableLimitAttributeAcknowledgeCode {
  Ok = 0,
  LimitAttributeDefinitionError = 1,
  CannotPerformNow = 2,
}
singleformat_enum!{VariableLimitAttributeAcknowledgeCode, Bin}
