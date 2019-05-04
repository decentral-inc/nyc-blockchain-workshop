use support::{decl_storage, decl_module, StorageValue, dispatch::Result, decl_event};
use {balances, system::{self, ensure_signed}};
use runtime_primitives::traits::{Hash, As};
use parity_codec::Encode;


pub trait Trait: balances::Trait {
    type Bet: As<u64>;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}



decl_storage! {
    trait Store for Module<T: Trait> as Storage {
        // Declare storage and getter function here
        Trial get(get_trial): u64;
        Payment get(payment): Option<T::Balance>;
        Pot get(pot): T::Balance;
    }
}



decl_event!(
    pub enum Event<T>
    where
        <T as balances::Trait>::Balance,
        <T as Trait>::Bet::_IMPL_DECODE_FOR_Event::_parity_codec::Decode
    {
        Win(Balance),
        Deposit(Balance),
        Trial(Bet),
        Payment(Balance),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;


        fn play(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let payment = Self::payment().ok_or("Must have payment amount set")?;

            <balances::Module<T>>::decrease_free_balance(&sender, payment)?;


            if <Trial<T>>::get() == 42  {
                <Trial<T>>::mutate(|trial| *trial = 0);
                Self::deposit_event(RawEvent::Win(<Pot<T>>::get()));
                <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
            }

            if(<system::Module<T>>::random_seed(), &sender).using_encoded(<T as system::Trait>::Hashing::hash).using_encoded(|e| e[0] < 128)
            {
               Self::deposit_event(RawEvent::Win(<Pot<T>>::get()));

               <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take()); 
            }

            <Pot<T>>::mutate(|pot| *pot += payment);    
            <Trial<T>>::mutate(|trial| *trial += 1);
            Self::deposit_event(RawEvent::Deposit(payment));
            
            Ok(())        
        } 

        fn set_payment(_origin, value: T::Balance) -> Result {
            <Trial<T>>::put(0);
            let trial = 0;
            Self::deposit_event(RawEvent::Trial(trial));
  
            //If the payment has not been set...
            if Self::payment().is_none() {
            // ... we will set it to the value we passed in.
            <Payment<T>>::put(value);
    
            // We will also put that initial value into the pot for someone to win
            <Pot<T>>::put(value);
            }  

            Self::deposit_event(RawEvent::Payment(value));
  
            Ok(())
        }       
    }
}
