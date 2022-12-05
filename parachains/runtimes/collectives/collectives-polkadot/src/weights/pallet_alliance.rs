// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_alliance.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		// Minimum execution time: 42_836 nanoseconds.
		Weight::from_ref_time(47_391_581 as u64)
			// Standard Error: 2_376
			.saturating_add(Weight::from_ref_time(44_684 as u64).saturating_mul(y as u64))
			// Standard Error: 2_168
			.saturating_add(Weight::from_ref_time(178_869 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 46_102 nanoseconds.
		Weight::from_ref_time(43_162_398 as u64)
			// Standard Error: 39_839
			.saturating_add(Weight::from_ref_time(613_745 as u64).saturating_mul(x as u64))
			// Standard Error: 3_371
			.saturating_add(Weight::from_ref_time(97_670 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		// Minimum execution time: 33_850 nanoseconds.
		Weight::from_ref_time(36_976_101 as u64)
			// Standard Error: 1_670
			.saturating_add(Weight::from_ref_time(186_739 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		// Minimum execution time: 50_658 nanoseconds.
		Weight::from_ref_time(46_302_122 as u64)
			// Standard Error: 30_636
			.saturating_add(Weight::from_ref_time(53_733 as u64).saturating_mul(x as u64))
			// Standard Error: 2_985
			.saturating_add(Weight::from_ref_time(64_964 as u64).saturating_mul(y as u64))
			// Standard Error: 2_666
			.saturating_add(Weight::from_ref_time(162_948 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		// Minimum execution time: 59_160 nanoseconds.
		Weight::from_ref_time(59_826_448 as u64)
			// Standard Error: 237
			.saturating_add(Weight::from_ref_time(422 as u64).saturating_mul(b as u64))
			// Standard Error: 2_736
			.saturating_add(Weight::from_ref_time(60_434 as u64).saturating_mul(y as u64))
			// Standard Error: 2_444
			.saturating_add(Weight::from_ref_time(192_397 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: Alliance Rule (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		// Minimum execution time: 67_960 nanoseconds.
		Weight::from_ref_time(63_842_848 as u64)
			// Standard Error: 2_800
			.saturating_add(Weight::from_ref_time(92_901 as u64).saturating_mul(y as u64))
			// Standard Error: 2_501
			.saturating_add(Weight::from_ref_time(207_448 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		// Minimum execution time: 52_105 nanoseconds.
		Weight::from_ref_time(49_988_359 as u64)
			// Standard Error: 2_762
			.saturating_add(Weight::from_ref_time(68_407 as u64).saturating_mul(y as u64))
			// Standard Error: 2_467
			.saturating_add(Weight::from_ref_time(187_212 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	/// The range of component `x` is `[1, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(x: u32, y: u32, z: u32, ) -> Weight {
		// Minimum execution time: 49_820 nanoseconds.
		Weight::from_ref_time(32_041_298 as u64)
			// Standard Error: 7_531
			.saturating_add(Weight::from_ref_time(242_874 as u64).saturating_mul(x as u64))
			// Standard Error: 797
			.saturating_add(Weight::from_ref_time(191_102 as u64).saturating_mul(y as u64))
			// Standard Error: 719
			.saturating_add(Weight::from_ref_time(164_605 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:101 w:50)
	// Storage: System Account (r:50 w:50)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		// Minimum execution time: 254_510 nanoseconds.
		Weight::from_ref_time(20_132_511 as u64)
			// Standard Error: 12_759
			.saturating_add(Weight::from_ref_time(1_328_779 as u64).saturating_mul(x as u64))
			// Standard Error: 12_609
			.saturating_add(Weight::from_ref_time(1_428_121 as u64).saturating_mul(y as u64))
			// Standard Error: 25_197
			.saturating_add(Weight::from_ref_time(11_624_380 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(y as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(z as u64)))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(z as u64)))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		// Minimum execution time: 19_076 nanoseconds.
		Weight::from_ref_time(19_478_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		// Minimum execution time: 21_293 nanoseconds.
		Weight::from_ref_time(21_897_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		// Minimum execution time: 23_548 nanoseconds.
		Weight::from_ref_time(23_811_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		// Minimum execution time: 55_466 nanoseconds.
		Weight::from_ref_time(56_496_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		// Minimum execution time: 41_854 nanoseconds.
		Weight::from_ref_time(42_135_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		// Minimum execution time: 37_973 nanoseconds.
		Weight::from_ref_time(39_012_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		// Minimum execution time: 40_480 nanoseconds.
		Weight::from_ref_time(41_266_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		// Minimum execution time: 44_869 nanoseconds.
		Weight::from_ref_time(45_401_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		// Minimum execution time: 127_075 nanoseconds.
		Weight::from_ref_time(128_042_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Minimum execution time: 17_383 nanoseconds.
		Weight::from_ref_time(17_584_000 as u64)
			// Standard Error: 3_838
			.saturating_add(Weight::from_ref_time(1_352_363 as u64).saturating_mul(n as u64))
			// Standard Error: 1_503
			.saturating_add(Weight::from_ref_time(82_716 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Minimum execution time: 18_039 nanoseconds.
		Weight::from_ref_time(18_130_000 as u64)
			// Standard Error: 214_795
			.saturating_add(Weight::from_ref_time(15_007_302 as u64).saturating_mul(n as u64))
			// Standard Error: 84_123
			.saturating_add(Weight::from_ref_time(743_958 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
