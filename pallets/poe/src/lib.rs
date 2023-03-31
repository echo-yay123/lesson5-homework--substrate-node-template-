
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>


pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of claim that can be added.
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId,Vec<u8>),
		ClaimTransfered(T::AccountId,T::AccountId,Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}


	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8,T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;


	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		
		//This function will create a proof of exsitence.
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim:Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);
	
			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
	

			Self::deposit_event(Event::ClaimCreated(sender,claim));

			Ok(().into())
		}
		
		// This function will revoke a proof of exsitence.
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;
			
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			
			// Get owner of the claim, if none return an error.
			let (owner,_)  = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			// Verify that sender of the current call is the claim owner.
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			// Remove claim from storage.
			Proofs::<T>::remove(&bounded_claim);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender,claim));

			Ok(().into())	
		}
	
		// This function will transfer a proof of exsitence from a sender to a receiver.
	
		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, receiver: <T as frame_system::Config>::AccountId, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;
			
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			
			// Get owner of the claim, if none return an error.
			let (owner,_)  = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			// Verify that sender of the current call is the claim owner.
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			//Remove the sender's claim.		 
			Proofs::<T>::remove(&bounded_claim);
			  
            //Insert the receiver's claim.
            Proofs::<T>::insert(
				&bounded_claim,
				(receiver.clone(),frame_system::Pallet::<T>::block_number())
			);

            // Emit an event that the claim was transfered.
            Self::deposit_event(Event::ClaimTransfered(sender,receiver,claim));
            // 交易发送成功			
			Ok(().into())	
		}
		
	}
}
