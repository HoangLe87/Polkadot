#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {

    /*
    *************** IMPORTS ***************
    */
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            psp22::{
                Internal,
                *,
            },
        },
        traits::Storage,
    };
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };

    /*
    *************** EVENTS ***************
    */
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /*
    *************** STORAGE ***************
    */
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PairContract {
        #[storage_field]
        psp22: psp22::Data,
    }

    /*
    *************** IMPLEMENTATION ***************
    */
    impl PSP22 for PairContract {}
    // contructor
    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }
    }
    //override events of default PSP22
    impl Internal for PairContract {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }
    
        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
        }
    }
}