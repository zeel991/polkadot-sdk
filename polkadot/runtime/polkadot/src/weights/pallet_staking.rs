// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-18, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_staking.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking::WeightInfo for WeightInfo<T> {
	fn bond() -> Weight {
		(66_434_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn bond_extra() -> Weight {
		(51_531_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn unbond() -> Weight {
		(54_336_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(47_505_000 as Weight)
			// Standard Error: 0
			.saturating_add((24_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(77_865_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_194_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn validate() -> Weight {
		(28_555_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn kick(k: u32, ) -> Weight {
		(10_886_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((15_538_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	fn nominate(n: u32, ) -> Weight {
		(34_708_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((5_034_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn chill() -> Weight {
		(15_837_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	fn set_payee() -> Weight {
		(10_696_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_controller() -> Weight {
		(23_482_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_validator_count() -> Weight {
		(1_815_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_no_eras() -> Weight {
		(2_048_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era() -> Weight {
		(2_052_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era_always() -> Weight {
		(2_047_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_invulnerables(v: u32, ) -> Weight {
		(1_969_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_unstake(s: u32, ) -> Weight {
		(54_672_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_189_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(3_374_694_000 as Weight)
			// Standard Error: 221_000
			.saturating_add((19_788_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(95_732_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((45_189_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(123_745_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((58_409_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn rebond(l: u32, ) -> Weight {
		(44_585_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((63_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 69_000
			.saturating_add((31_931_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	fn reap_stash(s: u32, ) -> Weight {
		(65_438_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_182_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 786_000
			.saturating_add((286_767_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 39_000
			.saturating_add((46_380_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 93_000
			.saturating_add((23_941_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 93_000
			.saturating_add((26_505_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_172_000
			.saturating_add((25_012_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
	}
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 31_000
			.saturating_add((10_079_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	fn set_staking_limits() -> Weight {
		(4_524_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn chill_other() -> Weight {
		(30_191_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
