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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-16, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_referenda
// --extrinsic=*
// --steps=2
// --repeat=1
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_referenda.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: FellowshipCollective Members (r:1 w:0)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumCount (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:0 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `161305`
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(38_000_000)
			.saturating_add(Weight::from_proof_size(161305))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `319953`
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_ref_time(60_000_000)
			.saturating_add(Weight::from_proof_size(319953))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1941`
		//  Estimated: `9151`
		// Minimum execution time: 89_000 nanoseconds.
		Weight::from_ref_time(89_000_000)
			.saturating_add(Weight::from_proof_size(9151))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1982`
		//  Estimated: `9151`
		// Minimum execution time: 82_000 nanoseconds.
		Weight::from_ref_time(82_000_000)
			.saturating_add(Weight::from_proof_size(9151))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `833`
		//  Estimated: `324931`
		// Minimum execution time: 161_000 nanoseconds.
		Weight::from_ref_time(161_000_000)
			.saturating_add(Weight::from_proof_size(324931))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `643`
		//  Estimated: `8353`
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_ref_time(60_000_000)
			.saturating_add(Weight::from_proof_size(8353))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381`
		//  Estimated: `3375`
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(38_000_000)
			.saturating_add(Weight::from_proof_size(3375))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199`
		//  Estimated: `3375`
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(Weight::from_proof_size(3375))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `319953`
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(45_000_000)
			.saturating_add(Weight::from_proof_size(319953))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:0)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `581`
		//  Estimated: `332942`
		// Minimum execution time: 170_000 nanoseconds.
		Weight::from_ref_time(170_000_000)
			.saturating_add(Weight::from_proof_size(332942))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:0)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `5776`
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(13_000_000)
			.saturating_add(Weight::from_proof_size(5776))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3991`
		//  Estimated: `325729`
		// Minimum execution time: 236_000 nanoseconds.
		Weight::from_ref_time(236_000_000)
			.saturating_add(Weight::from_proof_size(325729))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3991`
		//  Estimated: `325729`
		// Minimum execution time: 293_000 nanoseconds.
		Weight::from_ref_time(293_000_000)
			.saturating_add(Weight::from_proof_size(325729))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3589`
		//  Estimated: `164951`
		// Minimum execution time: 160_000 nanoseconds.
		Weight::from_ref_time(160_000_000)
			.saturating_add(Weight::from_proof_size(164951))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3542`
		//  Estimated: `164951`
		// Minimum execution time: 146_000 nanoseconds.
		Weight::from_ref_time(146_000_000)
			.saturating_add(Weight::from_proof_size(164951))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3528`
		//  Estimated: `167440`
		// Minimum execution time: 161_000 nanoseconds.
		Weight::from_ref_time(161_000_000)
			.saturating_add(Weight::from_proof_size(167440))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3583`
		//  Estimated: `167440`
		// Minimum execution time: 158_000 nanoseconds.
		Weight::from_ref_time(158_000_000)
			.saturating_add(Weight::from_proof_size(167440))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295`
		//  Estimated: `161664`
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_ref_time(32_000_000)
			.saturating_add(Weight::from_proof_size(161664))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `161664`
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(Weight::from_proof_size(161664))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3375`
		// Minimum execution time: 20_000 nanoseconds.
		Weight::from_ref_time(20_000_000)
			.saturating_add(Weight::from_proof_size(3375))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611`
		//  Estimated: `166642`
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(42_000_000)
			.saturating_add(Weight::from_proof_size(166642))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `166642`
		// Minimum execution time: 92_000 nanoseconds.
		Weight::from_ref_time(92_000_000)
			.saturating_add(Weight::from_proof_size(166642))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `164153`
		// Minimum execution time: 89_000 nanoseconds.
		Weight::from_ref_time(89_000_000)
			.saturating_add(Weight::from_proof_size(164153))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `782`
		//  Estimated: `164153`
		// Minimum execution time: 103_000 nanoseconds.
		Weight::from_ref_time(103_000_000)
			.saturating_add(Weight::from_proof_size(164153))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `164153`
		// Minimum execution time: 84_000 nanoseconds.
		Weight::from_ref_time(84_000_000)
			.saturating_add(Weight::from_proof_size(164153))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `803`
		//  Estimated: `164153`
		// Minimum execution time: 91_000 nanoseconds.
		Weight::from_ref_time(91_000_000)
			.saturating_add(Weight::from_proof_size(164153))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `803`
		//  Estimated: `324965`
		// Minimum execution time: 151_000 nanoseconds.
		Weight::from_ref_time(151_000_000)
			.saturating_add(Weight::from_proof_size(324965))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `164153`
		// Minimum execution time: 92_000 nanoseconds.
		Weight::from_ref_time(92_000_000)
			.saturating_add(Weight::from_proof_size(164153))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:0 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `5941`
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_000_000)
			.saturating_add(Weight::from_proof_size(5941))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `317`
		//  Estimated: `5902`
		// Minimum execution time: 22_000 nanoseconds.
		Weight::from_ref_time(22_000_000)
			.saturating_add(Weight::from_proof_size(5902))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
