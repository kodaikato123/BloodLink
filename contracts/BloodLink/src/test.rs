#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, String, Symbol,
};

use crate::{BloodLinkContract};

mod tests {

    use super::*;

    // Test 1: Happy path
    #[test]
    fn test_blood_request_success() {
        let env = Env::default();

        let donor = Address::generate(&env);

        BloodLinkContract::register_donor(
            env.clone(),
            donor.clone(),
            Symbol::short("A+"),
        );

        let request_id = BloodLinkContract::request_blood(
            env.clone(),
            String::from_str(&env, "Juan"),
            Symbol::short("A+"),
            donor,
        );

        assert_eq!(request_id, 1);
    }

    // Test 2: Edge case
    #[test]
    #[should_panic(expected = "Incompatible donor")]
    fn test_wrong_blood_type() {
        let env = Env::default();

        let donor = Address::generate(&env);

        BloodLinkContract::register_donor(
            env.clone(),
            donor.clone(),
            Symbol::short("B+"),
        );

        BloodLinkContract::request_blood(
            env,
            String::from_str(&Env::default(), "Maria"),
            Symbol::short("A+"),
            donor,
        );
    }

    // Test 3: State verification
    #[test]
    fn test_donor_unavailable_after_match() {
        let env = Env::default();

        let donor = Address::generate(&env);

        BloodLinkContract::register_donor(
            env.clone(),
            donor.clone(),
            Symbol::short("O+"),
        );

        BloodLinkContract::request_blood(
            env.clone(),
            String::from_str(&env, "Pedro"),
            Symbol::short("O+"),
            donor.clone(),
        );

        let donor_info =
            BloodLinkContract::get_donor(
                env,
                donor,
            );

        assert_eq!(donor_info.available, false);
    }

    // Test 4
    #[test]
    fn test_get_request() {
        let env = Env::default();

        let donor = Address::generate(&env);

        BloodLinkContract::register_donor(
            env.clone(),
            donor.clone(),
            Symbol::short("AB"),
        );

        BloodLinkContract::request_blood(
            env.clone(),
            String::from_str(&env, "Liza"),
            Symbol::short("AB"),
            donor.clone(),
        );

        let request =
            BloodLinkContract::get_request(
                env,
                1,
            );

        assert_eq!(
            request.patient_name,
            String::from_str(&Env::default(), "Liza")
        );
    }

    // Test 5
    #[test]
    fn test_register_donor() {
        let env = Env::default();

        let donor = Address::generate(&env);

        BloodLinkContract::register_donor(
            env.clone(),
            donor.clone(),
            Symbol::short("O-"),
        );

        let donor_info =
            BloodLinkContract::get_donor(
                env,
                donor,
            );

        assert_eq!(
            donor_info.blood_type,
            Symbol::short("O-")
        );
    }
}