#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		dispatch::Dispatchable,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use risc0_zkvm::{SegmentReceipt, SessionReceipt};

	// We more or less know image id will always be this so, declare it here and not in Config
	type ImageId = [u32; 8];

	#[pallet::pallet]
	// TODO: Needs proper BoundedVec encoding from offchain in order to get bounded types working
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ From<Call<Self>>
			+ IsType<<Self as frame_system::Config>::RuntimeCall>
			+ From<frame_system::Call<Self>>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>
			+ ReservableCurrency<Self::AccountId>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// Length of the stored hash commitments expected by this pallet
		type HashLength: Get<u32>;
	}

	#[pallet::storage]
	/// Store all commitments
	pub(super) type Commitments<T: Config> =
	StorageMap<_, Blake2_128Concat,
	// The hash
	BoundedVec<u8, T::HashLength>,
	// Was verified or not
	bool, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Committed(BoundedVec<u8, T::HashLength>),
		OtherCalled,
		/// Proof was successfully verified and will be stored
		ProofVerified(ImageId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Already submitted this hash
		AlreadySubmitted,
		/// Proof could not be verified.
		ProofNotVerified
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		// TODO: Weights
		#[pallet::weight({1000000})]
		pub fn commit(
			origin: OriginFor<T>,
			// image_id: ImageId,
			hash: BoundedVec<u8, T::HashLength>
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			ensure!(!Commitments::<T>::contains_key(&hash), Error::<T>::AlreadySubmitted);

			<Commitments<T>>::insert(&hash, false);

			Self::deposit_event(Event::Committed(hash));
			Ok(())
		}

		#[pallet::call_index(2)]
		// TODO: Weights
		#[pallet::weight({1000000})]
		pub fn verify_preimage_proof(
			origin: OriginFor<T>,
			receipt_data: Vec<(Vec<u32>, u32)>,
			journal: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin.clone())?;

			let segments: Vec<SegmentReceipt> = receipt_data
				.clone()
				.into_iter()
				.map(|(seal, index)| SegmentReceipt { seal, index })
				.collect();

			// Unique identifier of the program
			// TODO: We can map from stored image ids -> extrinsics and call the relevant extrinsic if that image id is verified
			let image_id = [3979446179, 1473523645, 3571476172, 1865387470, 3136038554, 1943042092, 3537243349, 1796109856];
		
			let receipt = SessionReceipt { segments, journal };
			// Verify the proof
			receipt.verify(image_id).map_err(|_| Error::<T>::ProofNotVerified)?;

			// Demonstrate that we verified the proof for a given image
			Self::deposit_event(Event::<T>::ProofVerified(image_id));

			// // The user is verified. They can now proceed to the call
			let call = Call::do_other_thing {};
			// Get runtime-level call. Assuming we want to call any extrinsic in the runtime
			let call = <T as Config>::RuntimeCall::from(call);
			// There are a number of ways to do this in Substrate
			let _ = call.dispatch(origin);

			Ok(())
		}

		#[pallet::call_index(3)]
		// TODO: Weights
		#[pallet::weight({1000000})]
		/// Represents some extrinsic action deferred to as a result of a verification of a proof whose image id
		/// is related to this extrinsic
		pub fn do_other_thing(
			_origin: OriginFor<T>,
		) -> DispatchResult {
			Self::deposit_event(Event::<T>::OtherCalled);
			Ok(())
		}
	}
}
