#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_error,decl_event, dispatch, traits::Get};
use frame_system::ensure_signed;

use frame_support::traits::Vec;
use codec::{Encode,Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[derive(Encode,Decode,Default,Clone,PartialEq)]
pub struct Servicedetails<AccountId>{
	manufacturer:AccountId,
	chassi_number:u32,
	engine_number:u32,
	model:Vec<u8>,
	service_description:Vec<u8>,
	servicing_date:Vec<u8>,
	servicecenter_address:Vec<u8>,
	vehical_number:Vec<u8>,
}


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
pub type ServiceDetailsOf<T>= Servicedetails<<T as frame_system::Config>::AccountId>;


// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as ServicingDetailsModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		ServiceDetails get(fn servicedetails):map hasher(blake2_128_concat) u32 => Option<Servicedetails<T::AccountId>>;

	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		Insertservicedetails(AccountId,u32,u32,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}


		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn insertservicedetails(origin,
			chassi_number:u32,
			engine_number:u32,
			model:Vec<u8>,
			service_description:Vec<u8>,
			servicing_date:Vec<u8>,
			servicecenter_address:Vec<u8>,
			vehical_number:Vec<u8> ) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			
			let insert_servicedetails=Servicedetails{
				manufacturer:who.clone(),
				chassi_number,
				engine_number,
				model:model.clone(),
				service_description:service_description.clone(),
				servicing_date:servicing_date.clone(),
				servicecenter_address:servicecenter_address.clone(),
				vehical_number:vehical_number.clone(),
			};

			ServiceDetails::<T>::insert(&chassi_number,insert_servicedetails);
			
			Self::deposit_event(RawEvent::Insertservicedetails(who,chassi_number,engine_number,model,service_description,servicing_date,servicecenter_address,vehical_number));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
