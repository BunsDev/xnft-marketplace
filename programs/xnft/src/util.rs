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

use anchor_lang::prelude::AccountInfo;
use anchor_lang::solana_program::{self, system_instruction};

pub fn send_payment<'info>(
    price: u64,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    program: AccountInfo<'info>,
) -> anchor_lang::Result<()> {
    solana_program::program::invoke(
        &system_instruction::transfer(from.key, to.key, price),
        &[from, to, program],
    )
    .map_err(Into::into)
}

macro_rules! verify_optional_pubkey {
    ($fn_name:ident, $field:ident, $err:expr) => {
        pub fn $fn_name(&self, pk: &anchor_lang::prelude::Pubkey) -> anchor_lang::Result<()> {
            if let Some(key) = self.$field {
                if key != *pk {
                    return Err(anchor_lang::prelude::error!($err));
                }
            }
            Ok(())
        }
    };
}
pub(crate) use verify_optional_pubkey;
