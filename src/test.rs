#[cfg(test)]
mod test {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use crate::{execute, instantiate, query};
    use crate::msg::{ExecMsg, IncrementResp, InstantiateMsg, QueryMsg, ValueResp};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let initial_value = 1;
        let contract_addr = app.
            instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: initial_value},
                &[],
                "Counting contract",
                None,
            ).unwrap();


        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: initial_value})
    }

    #[test]
    fn query_increment() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.
            instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 2 },
                &[],
                "Counting contract",
                None,
            ).unwrap();

        let value = 1;
        let res: IncrementResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Increment { value })
            .unwrap();

        let incremented_value = value + 1;
        assert_eq!(res, IncrementResp{value: incremented_value })
    }

    #[test]
    fn poke() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());
        let sender = Addr::unchecked("sender");
        let initial_counter = 0;
        let contract_addr = app.
            instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg { counter: initial_counter },
                &[],
                "Counting contract",
                None,
            ).unwrap();

        // will update the counter by 1
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecMsg::Poke {},
            &[],
        ).unwrap();

        let res: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(res, ValueResp {value:  initial_counter + 1})
    }

    #[test]
    fn reset() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());
        let sender = Addr::unchecked("sender");
        let initial_counter = 0;
        let contract_addr = app.
            instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg { counter: initial_counter },
                &[],
                "Counting contract",
                None,
            ).unwrap();

        // will update the counter by 1
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Poke {},
            &[],
        ).unwrap();

        let reset_val = 10;
        // reset the counter to 10
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Reset { counter: reset_val},
            &[],
        ).unwrap();

        let res: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(res, ValueResp{value: reset_val})
    }
}

