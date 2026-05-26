#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Symbol, Vec,
};

#[contract]
pub struct BloodLinkContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Donor(Address),
    Request(u32),
    RequestCount,
}

#[derive(Clone)]
#[contracttype]
pub struct Donor {
    pub blood_type: Symbol,
    pub available: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct BloodRequest {
    pub patient_name: String,
    pub blood_type: Symbol,
    pub matched_donor: Address,
}

#[contractimpl]
impl BloodLinkContract {

    // Register a verified donor
    pub fn register_donor(
        env: Env,
        donor: Address,
        blood_type: Symbol,
    ) {
        donor.require_auth();

        let donor_data = Donor {
            blood_type,
            available: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Donor(donor), &donor_data);
    }

    // Submit urgent blood request
    pub fn request_blood(
        env: Env,
        patient_name: String,
        blood_type: Symbol,
        donor: Address,
    ) -> u32 {

        let donor_data: Donor = env.storage()
            .persistent()
            .get(&DataKey::Donor(donor.clone()))
            .unwrap();

        // Verify donor compatibility
        if donor_data.blood_type != blood_type {
            panic!("Incompatible donor");
        }

        // Verify donor availability
        if donor_data.available == false {
            panic!("Donor unavailable");
        }

        let mut request_count: u32 = env.storage()
            .persistent()
            .get(&DataKey::RequestCount)
            .unwrap_or(0);

        request_count += 1;

        let request = BloodRequest {
            patient_name,
            blood_type,
            matched_donor: donor.clone(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Request(request_count), &request);

        // Mark donor unavailable temporarily
        let updated_donor = Donor {
            blood_type: donor_data.blood_type,
            available: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Donor(donor), &updated_donor);

        env.storage()
            .persistent()
            .set(&DataKey::RequestCount, &request_count);

        request_count
    }

    // Get donor info
    pub fn get_donor(env: Env, donor: Address) -> Donor {
        env.storage()
            .persistent()
            .get(&DataKey::Donor(donor))
            .unwrap()
    }

    // Get request details
    pub fn get_request(env: Env, id: u32) -> BloodRequest {
        env.storage()
            .persistent()
            .get(&DataKey::Request(id))
            .unwrap()
    }
}