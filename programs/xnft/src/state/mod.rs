// Copyright (C) 2022 Blue Coral, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use anchor_lang::prelude::*;

mod access;
mod install;
mod review;
mod xnft;

pub use self::xnft::*; // use `self::` prefix to remove crate vs module ambiguity during builds
pub use access::*;
pub use install::*;
pub use review::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
pub enum Kind {
    #[cfg_attr(test, default)]
    App,
    Collection,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[cfg_attr(test, derive(Default))]
pub enum L1 {
    #[cfg_attr(test, default)]
    Solana,
    Ethereum,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[cfg_attr(test, derive(Default))]
pub enum Tag {
    #[cfg_attr(test, default)]
    None,
    Defi,
    Game,
    Nft,
}
