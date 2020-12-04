use super::*;
use ckb_tool::ckb_types::{
    bytes::Bytes,
    core::{TransactionBuilder, TransactionView},
    packed::*,
    prelude::*,
};
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};

const MAX_CYCLES: u64 = 1_000_000;
// errors
const INFO_TYPE_ARGS_NOT_MATCH: i8 = 5;
const OWNER_LOCK_SCRIPT_NOT_MATCH: i8 = 6;
const WRONG_DATA_STRUCT: i8 = 7;

fn build_test_context(
    sudt_type_args: Option<Bytes>,
    sudt_info_type_args: Option<Bytes>,
    sudt_info_data: Bytes
) -> (Context, TransactionView) {
    // deploy contract
    let mut context = Context::default();
    let sudt_info_contract_bin: Bytes = Loader::default().load_binary("my-sudt-info");
    let sudt_info_out_point = context.deploy_cell(sudt_info_contract_bin);

    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let sudt_always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // prepare cells
    let lock_hash: [u8; 32] = lock_script.calc_script_hash().unpack();
    let owner_lock_script_args: Bytes = lock_hash.to_vec().into();
    let sudt_script = context
        .build_script(&sudt_always_success_out_point.clone(), sudt_type_args.unwrap_or(owner_lock_script_args.clone()))
        .expect("script");

    let sudt_script_hash_args: Bytes = sudt_script.calc_script_hash().raw_data();
    let sudt_info_script = context
        .build_script(&sudt_info_out_point, sudt_info_type_args.unwrap_or(sudt_script_hash_args))
        .expect("script");

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
        .capacity(1000u64.pack())
        .lock(lock_script.clone())
        .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(sudt_script.clone()).pack())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(sudt_info_script.clone()).pack())
            .build()
    ];

    let outputs_data = vec![
        Bytes::from(hex::decode("3e800000000000000000000000000000").unwrap()),
        sudt_info_data
    ];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .build();
    (context, tx)
}

#[test]
fn test_verify_succeed() {
    let (mut context, tx) = build_test_context(
            None,
            None,
            Bytes::from(
                hex::decode("060a55534420436f696e0a555344430a546f74616c737570706c793a31303030303030302e3030303030300a4f66666963616c20536974653a68747470733a2f2f7777772e63656e7472652e696f2f0a4465736372697074696f6e3a78787878").unwrap()
            )
        );
    let tx = context.complete_tx(tx);

    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("cycles: {}", cycles);
}

#[test]
fn test_sudt_info_type_args_not_equal_sudt_type_hash() {
    let wrong_sudt_info_type_args = Bytes::from(&b"Hello world"[..]);
    let (mut context, tx) = build_test_context(None, Some(wrong_sudt_info_type_args), Bytes::from(hex::decode("060a55534420436f696e0a555344430a546f74616c737570706c793a31303030303030302e3030303030300a4f66666963616c20536974653a68747470733a2f2f7777772e63656e7472652e696f2f0a4465736372697074696f6e3a78787878").unwrap()));
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(INFO_TYPE_ARGS_NOT_MATCH).output_type_script(script_cell_index)
    );
}

#[test]
fn test_input_lock_hash_not_include_sudt_type_args() {
    let wrong_sudt_type_args = Bytes::from(&b"Hello world"[..]);
    let (mut context, tx) = build_test_context(Some(wrong_sudt_type_args), None, Bytes::from(hex::decode("060a55534420436f696e0a555344430a546f74616c737570706c793a31303030303030302e3030303030300a4f66666963616c20536974653a68747470733a2f2f7777772e63656e7472652e696f2f0a4465736372697074696f6e3a78787878").unwrap()));
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(OWNER_LOCK_SCRIPT_NOT_MATCH).output_type_script(script_cell_index)
    );
}

#[test]
fn test_data_is_null() {
    let (mut context, tx) = build_test_context(
            None,
            None,
            Bytes::from(hex::decode("").unwrap())
        );
    let tx = context.complete_tx(tx);

        // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(WRONG_DATA_STRUCT).output_type_script(script_cell_index)
    );
}

#[test]
fn test_data_len_less_than_three() {
    let (mut context, tx) = build_test_context(
            None,
            None,
            Bytes::from(
                hex::decode("060a55534420436f696e").unwrap()
            )
        );
    let tx = context.complete_tx(tx);

    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(WRONG_DATA_STRUCT).output_type_script(script_cell_index)
    );
}

#[test]
fn test_data_decimal_not_a_byte() {
    let (mut context, tx) = build_test_context(
            None,
            None,
            Bytes::from(
                hex::decode("55534420436f696e0a55534420436f696e0a555344430a546f74616c737570706c793a31303030303030302e3030303030300a4f66666963616c20536974653a68747470733a2f2f7777772e63656e7472652e696f2f0a4465736372697074696f6e3a78787878").unwrap()
            )
        );
    let tx = context.complete_tx(tx);

        // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(WRONG_DATA_STRUCT).output_type_script(script_cell_index)
    );
}
