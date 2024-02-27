// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        full_name -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        time_stamp -> Nullable<Timestamp>,
        account_type -> Nullable<Int4>,
        balance -> Numeric,
        admin_account_id -> Nullable<Int8>,
    }
}

diesel::table! {
    bbgo_start_info (id) {
        id -> Int8,
        api_keys_id -> Nullable<Int4>,
        pair_id -> Nullable<Int4>,
        strategy_id -> Nullable<Int4>,
        risk_id -> Nullable<Int4>,
        logs_id -> Nullable<Int4>,
        ma_id -> Nullable<Int4>,
        rs_id -> Nullable<Int4>,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    exchange_api_keys (id) {
        id -> Int8,
        #[max_length = 255]
        api_key -> Varchar,
        #[max_length = 255]
        secret_key -> Varchar,
    }
}

diesel::table! {
    inter_account_transfer_requests (id) {
        id -> Int8,
        admin_account_id -> Int8,
        from_account_id -> Int8,
        to_account_id -> Int8,
        price -> Numeric,
        quantity -> Numeric,
        fee -> Numeric,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    intra_account_transfer_requests (id) {
        id -> Int8,
        admin_account_id -> Int8,
        trader_account_id -> Int8,
        position_id -> Int8,
        from_portfolio_id -> Int8,
        to_portfolio_id -> Int8,
        price -> Numeric,
        quantity -> Numeric,
        fee -> Numeric,
        is_approved -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    logging_monitoring_settings (id) {
        id -> Int8,
        enable_logging -> Bool,
        enable_monitoring -> Bool,
        log_file_path -> Nullable<Text>,
        log_level -> Nullable<Int4>,
    }
}

diesel::table! {
    moving_average_indicator (id) {
        id -> Int8,
        period -> Int4,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        state -> Int4,
        buyin -> Bool,
        trading_pair_id -> Int8,
        position_id -> Int8,
        price -> Numeric,
        qty -> Int4,
        sell -> Int4,
        profit -> Numeric,
    }
}

diesel::table! {
    portfolios (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        trading_pair_id -> Int8,
        qty -> Int4,
        admin_account_id -> Int8,
        trader_account_id -> Int8,
        portfolio_type -> Int4,
        available_balance -> Numeric,
        lock_state -> Int4,
        lock_threashold -> Numeric,
    }
}

diesel::table! {
    positions (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        yesterday_time -> Timestamp,
        trading_pair_id -> Int8,
        quotation_id -> Int8,
        qty -> Int4,
        yesterday_qty -> Int4,
        realized_profit -> Numeric,
        unreal_profit -> Numeric,
        fifo_info -> Int4,
        portfolio_id -> Int8,
        lock_state -> Int4,
        lock_threshold -> Numeric,
    }
}

diesel::table! {
    quotations (id) {
        id -> Int8,
        time_stamp -> Timestamp,
        base_currency_id -> Int4,
    }
}

diesel::table! {
    risk_management (id) {
        id -> Int8,
        max_risk_per_trade -> Numeric,
        max_drawdown -> Numeric,
        max_portfolio_risk -> Numeric,
        max_daily_loss -> Numeric,
        max_consecutive_losses -> Int4,
    }
}

diesel::table! {
    rs_indicator (id) {
        id -> Int8,
        period -> Int4,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
    }
}

diesel::table! {
    sessions (session_token) {
        session_token -> Bytea,
        user_id -> Int4,
    }
}

diesel::table! {
    trading_pairs (id) {
        id -> Int8,
        base_currency_id -> Int4,
        quote_currency_id -> Int4,
    }
}

diesel::table! {
    trading_strategy (id) {
        id -> Int8,
        entry_criteria -> Text,
        exit_criteria -> Text,
        risk_percentage -> Numeric,
        position_size -> Numeric,
        #[max_length = 100]
        timeframe -> Varchar,
        signal_filter -> Text,
        holding_period -> Interval,
        take_profit_strategy -> Text,
        stop_loss_strategy -> Text,
        dynamic_adjustment -> Text,
        information_output -> Text,
        #[max_length = 100]
        trading_type -> Varchar,
        simulation_params -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
    }
}

diesel::joinable!(bbgo_start_info -> exchange_api_keys (api_keys_id));
diesel::joinable!(bbgo_start_info -> logging_monitoring_settings (logs_id));
diesel::joinable!(bbgo_start_info -> moving_average_indicator (ma_id));
diesel::joinable!(bbgo_start_info -> risk_management (risk_id));
diesel::joinable!(bbgo_start_info -> rs_indicator (rs_id));
diesel::joinable!(bbgo_start_info -> trading_pairs (pair_id));
diesel::joinable!(bbgo_start_info -> trading_strategy (strategy_id));
diesel::joinable!(intra_account_transfer_requests -> positions (position_id));
diesel::joinable!(orders -> trading_pairs (trading_pair_id));
diesel::joinable!(portfolios -> trading_pairs (trading_pair_id));
diesel::joinable!(positions -> quotations (quotation_id));
diesel::joinable!(positions -> trading_pairs (trading_pair_id));
diesel::joinable!(quotations -> currencies (base_currency_id));
diesel::joinable!(sessions -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    bbgo_start_info,
    currencies,
    exchange_api_keys,
    inter_account_transfer_requests,
    intra_account_transfer_requests,
    logging_monitoring_settings,
    moving_average_indicator,
    orders,
    portfolios,
    positions,
    quotations,
    risk_management,
    rs_indicator,
    sessions,
    trading_pairs,
    trading_strategy,
    users,
);
