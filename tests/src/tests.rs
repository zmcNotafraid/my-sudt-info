use super::*;
use ckb_tool::ckb_types::{
    bytes::Bytes,
    core::{TransactionBuilder, TransactionView},
    packed::*,
    prelude::*,
};
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};

const MAX_CYCLES: u64 = 100_0000;
// errors
const OWNER_LOCK_SCRIPT_NOT_EXIST: i8 = 5;

fn build_test_context(
    sudt_info_type_args: Option<Bytes>
) -> (Context, TransactionView) {
    // deploy contract
    let mut context = Context::default();
    let sudt_info_contract_bin: Bytes = Loader::default().load_binary("my-sudt-info");
    let sudt_info_out_point = context.deploy_cell(sudt_info_contract_bin);

    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // prepare cells
    let lock_hash = lock_script.calc_script_hash().raw_data();

    let sudt_info_script = context
        .build_script(&sudt_info_out_point, sudt_info_type_args.unwrap_or(lock_hash))
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
            .capacity(829u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(170u64.pack())
            .lock(lock_script.clone())
            .type_(Some(sudt_info_script.clone()).pack())
            .build()
    ];

    let outputs_data = vec![
        Bytes::from(hex::decode("").unwrap()),
        Bytes::from(hex::decode("060a55534420436f696e0a555344430a546f74616c737570706c793a31303030303030302e3030303030300a4f66666963616c20536974653a68747470733a2f2f7777772e63656e7472652e696f2f0a4465736372697074696f6e3a78787878").unwrap())
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
    let (mut context, tx) = build_test_context(None);
    let tx = context.complete_tx(tx);

    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("cycles: {}", cycles);
}

#[test]
fn test_input_not_exist_owner_lock_script() {
    let wrong_sudt_info_type_args = Bytes::from(&b"Hello world"[..]);
    let (mut context, tx) = build_test_context(Some(wrong_sudt_info_type_args));
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    let script_cell_index = 1;
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(OWNER_LOCK_SCRIPT_NOT_EXIST).output_type_script(script_cell_index)
    );
}

