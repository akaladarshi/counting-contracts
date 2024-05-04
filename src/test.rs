#[cfg(test)]
mod test {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use crate::{execute, instantiate, query};
    use crate::msg::{IncrementResp, QueryMsg, ValueResp};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.
            instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &Empty {},
                &[],
                "Counting contract",
                None,
            ).unwrap();


        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0})
    }

    #[test]
    fn query_increment() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.
            instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &Empty {},
                &[],
                "Counting contract",
                None,
            ).unwrap();

        let initial_val = 1;
        let res: IncrementResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Increment { value: initial_val,})
            .unwrap();

        let expected_val = initial_val + 1;
        assert_eq!(res, IncrementResp{value: expected_val})
    }
}

