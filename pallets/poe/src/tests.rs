use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works(){
    new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(
             Proofs::<Test>::get(&claim),
             ((1,frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
       );
    })
}


#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
        //返回tuple ，None类型不对要转成 (u64,u64)类型
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (0u64,0u64)
        );
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
    new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::NoSuchProof
       );
    })
}

#[test]
fn revoke_claim_failed_when_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),2u64,claim.clone()));
        
        //验证转移之后的结果
        assert_eq!(
              Proofs::<Test>::get(&claim),
              (2,frame_system::Pallet::<Test>::block_number())
        );
        
        //转移之后，不能revoke原来owner的claim
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_is_not_transfer_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), 3u64,claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_claim_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim_temp = vec![2, 3];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1),3u64, claim_temp.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn create_claim_failed_when_claim_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4, 5];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    });
}

