#![deny(missing_docs)]
//! HTS-style full-context label structure and parser/serializer from/to string.
//!
//! ```rust
//! # use std::error::Error;
//! use jlabel::{Label, Mora, Phoneme, Word};
//!
//! use std::str::FromStr;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let label_str = concat!(
//!     "sil^n-i+h=o",
//!     "/A:-3+1+7",
//!     "/B:xx-xx_xx",
//!     "/C:02_xx+xx",
//!     "/D:02+xx_xx",
//!     "/E:xx_xx!xx_xx-xx",
//!     "/F:7_4#0_xx@1_3|1_12",
//!     "/G:4_4%0_xx_1",
//!     "/H:xx_xx",
//!     "/I:3-12@1+2&1-8|1+41",
//!     "/J:5_29",
//!     "/K:2+8-41"
//! );
//! let label = Label::from_str(label_str)?;
//!
//! assert_eq!(
//!     label.phoneme,
//!     Phoneme {
//!         p2: Some("sil".to_string()),
//!         p1: Some("n".to_string()),
//!         c: Some("i".to_string()),
//!         n1: Some("h".to_string()),
//!         n2: Some("o".to_string()),
//!     }
//! );
//! assert_eq!(
//!     label.mora,
//!     Some(Mora {
//!         relative_accent_position: -3,
//!         position_forward: 1,
//!         position_backward: 7,
//!     })
//! );
//! assert_eq!(
//!     label.word_next,
//!     Some(Word {
//!         pos: Some(2),
//!         ctype: None,
//!         cform: None,
//!     })
//! );
//! assert_eq!(label.breath_group_prev, None);
//! #
//! #     Ok(())
//! # }
//! ```

mod fullcontext_label;
mod parser;
mod serializer;

pub use fullcontext_label::*;
pub use parser::ParseError;
