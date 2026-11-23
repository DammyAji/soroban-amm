use soroban_amm_sdk::types::SdkAmmError;
use amm::AmmError;
use soroban_sdk::{Env, TryFromVal, TryIntoVal};

#[test]
fn sdk_amm_error_covers_codes_25_through_30() {
    let variants = [
        (25, AmmError::QuorumNotReached, SdkAmmError::QuorumNotReached),
        (26, AmmError::InvalidQuorum, SdkAmmError::InvalidQuorum),
        (27, AmmError::EmergencyDelayNotElapsed, SdkAmmError::EmergencyDelayNotElapsed),
        (28, AmmError::InvalidRecipient, SdkAmmError::InvalidRecipient),
        (29, AmmError::MultisigGuardActive, SdkAmmError::MultisigGuardActive),
        (30, AmmError::InvalidEmergencyDelay, SdkAmmError::InvalidEmergencyDelay),
    ];

    for (code, contract_error, sdk_error) in variants {
        assert_eq!(contract_error as u32, code);
        assert_eq!(sdk_error as u32, code);
    }
}

#[test]
fn sdk_amm_error_decodes_codes_25_through_30() {
    let env = Env::default();
    let variants = [
        (25, SdkAmmError::QuorumNotReached),
        (26, SdkAmmError::InvalidQuorum),
        (27, SdkAmmError::EmergencyDelayNotElapsed),
        (28, SdkAmmError::InvalidRecipient),
        (29, SdkAmmError::MultisigGuardActive),
        (30, SdkAmmError::InvalidEmergencyDelay),
    ];

    for (code, expected) in variants {
        let val = code.try_into_val(&env).unwrap();
        let decoded = SdkAmmError::try_from_val(&env, &val).unwrap();
        assert_eq!(decoded, expected);
    }
}
