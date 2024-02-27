-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sessions (
    session_token BYTEA PRIMARY KEY,
    user_id integer NOT NULL
    -- Removed direct foreign key reference here
);

-- BBGO 資料格式定義 --
-- 交易所 API 金鑰
CREATE TABLE IF NOT EXISTS exchange_api_keys (
    id BIGSERIAL PRIMARY KEY,
    api_key VARCHAR(255) NOT NULL,
    secret_key VARCHAR(255) NOT NULL
);

-- 交易策略
CREATE TABLE IF NOT EXISTS trading_strategy (
    id BIGSERIAL PRIMARY KEY,
    entry_criteria TEXT NOT NULL,           -- 進入條件
    exit_criteria TEXT NOT NULL,            -- 退出條件
    risk_percentage DECIMAL(28, 8) NOT NULL, -- 風險比例
    position_size DECIMAL(28, 8) NOT NULL,  -- 交易量
    timeframe VARCHAR(100) NOT NULL,        -- 時間框架
    signal_filter TEXT NOT NULL,            -- 訊號過濾器
    holding_period INTERVAL NOT NULL,       -- 持倉期
    take_profit_strategy TEXT NOT NULL,     -- 止盈條件
    stop_loss_strategy TEXT NOT NULL,       -- 停損條件
    dynamic_adjustment TEXT NOT NULL,       -- 動態調整策略
    information_output TEXT NOT NULL,       -- 資訊輸出和日誌
    trading_type VARCHAR(100) NOT NULL,     -- 交易類型
    simulation_params TEXT NOT NULL         -- 模擬和回測參數
    -- others...
);

-- 風險管理
CREATE TABLE IF NOT EXISTS risk_management (
    id BIGSERIAL PRIMARY KEY,
    max_risk_per_trade DECIMAL(28, 8) NOT NULL,  -- 每筆交易的最大風險
    max_drawdown DECIMAL(28, 8) NOT NULL,       -- 最大回撤限制
    max_portfolio_risk DECIMAL(28, 8) NOT NULL, -- 最大投資組合風險
    max_daily_loss DECIMAL(28, 8) NOT NULL,     -- 每日最大損失
    max_consecutive_losses INTEGER NOT NULL     -- 最大連續虧損次數限制
    -- others...
);

-- 監控交易
CREATE TABLE IF NOT EXISTS logging_monitoring_settings (
    id BIGSERIAL PRIMARY KEY,
    enable_logging BOOLEAN NOT NULL,       -- 啟用日誌記錄
    enable_monitoring BOOLEAN NOT NULL,    -- 啟用監控
    log_file_path TEXT,                    -- 日誌文件路徑
    log_level INTEGER                      -- 日誌級別（例如：1 = INFO, 2 = WARNING, 3 = ERROR）
    -- others...
);

-- 技術指標
-- MA
CREATE TABLE IF NOT EXISTS moving_average_indicator (
    id BIGSERIAL PRIMARY KEY,
    period INTEGER NOT NULL,               -- 移動平均的期間（天數）
    type VARCHAR(50) NOT NULL              -- 移動平均的類型（簡單移動平均、指數移動平均等）
    -- others...
);

-- Relative Strength (RS) Indicator
CREATE TABLE IF NOT EXISTS rs_indicator (
    id BIGSERIAL PRIMARY KEY,
    period INTEGER NOT NULL,               -- RS指標的期間（天數）
    type VARCHAR(50) NOT NULL              -- RS指標的類型（14 天、9 天）
    -- others...
);

-- others indicators...

-- 啟動機器人資料格式
CREATE TABLE IF NOT EXISTS bbgo_start_info (
    id BIGSERIAL PRIMARY KEY,
    api_keys_id INTEGER,
    pair_id INTEGER,
    strategy_id INTEGER,
    risk_id INTEGER,
    logs_id INTEGER,
    ma_id INTEGER,
    rs_id INTEGER
    -- Removed direct foreign key references here
    -- others...
);

-- caculate PNL 架構規劃 --
-- 幣別
CREATE TABLE IF NOT EXISTS currencies (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL, -- 幣別代碼，例如：USD
    name VARCHAR(50) NOT NULL -- 幣別名稱，例如：US Dollar
);

-- 幣對
CREATE TABLE IF NOT EXISTS trading_pairs (
    id BIGSERIAL PRIMARY KEY,
    base_currency_id INTEGER NOT NULL,
    quote_currency_id INTEGER NOT NULL,
	UNIQUE(base_currency_id, quote_currency_id)
);

-- 訂單
CREATE TABLE IF NOT EXISTS orders (
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP NOT NULL, 
    state INTEGER NOT NULL, -- 0: pending, 1: success, 2: fail
    buyin BOOLEAN NOT NULL, -- 買或賣
    trading_pair_id BIGINT NOT NULL, -- 幣對 ID
    position_id BIGINT NOT NULL,
    price DECIMAL(28, 8) NOT NULL, -- 金額
    qty INTEGER NOT NULL, -- 數量
    sell INTEGER NOT NULL, -- 已經賣出多少，用來計算 FIFO
    profit DECIMAL(28, 8) NOT NULL -- 利潤
);

-- 報價
CREATE TABLE IF NOT EXISTS quotations ( -- fix base
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP NOT NULL,
    base_currency_id INTEGER NOT NULL -- 幣別 ID
);

-- 部位
CREATE TABLE IF NOT EXISTS positions ( -- fix base/quote
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP NOT NULL, -- 用來記錄更新的時間
    yesterday_time TIMESTAMP NOT NULL, -- 紀錄每日的更新時間，用來算 incoming
    trading_pair_id BIGINT NOT NULL, -- 幣對 ID
	quotation_id BIGINT NOT NULL,
    qty INTEGER NOT NULL, -- 數量（即時更新或每日更新？）
    yesterday_qty INTEGER NOT NULL, -- 昨天以前的 qty 用來算 incoming
    realized_profit DECIMAL(28, 8) NOT NULL, -- 該幣種目前的 realized PNL 從 opened 的 order 相加
    unreal_profit DECIMAL(28, 8) NOT NULL, -- 從 pending 的 order 相加
    fifo_info INTEGER NOT NULL, -- 一個用來記錄 FIFO 的東西，時間或 ID
    portfolio_id BIGINT NOT NULL,
	lock_state INTEGER NOT NULL, -- 0: no lock, 1: sell-locked, 2: buy-locked, 3: trade-locked, 4: 限制 Portfolio 在 Total PnL 達到一個數值的時候不能 buy/sell
	lock_threshold DECIMAL(28, 8) NOT NULL DEFAULT 0.0
);

-- 投資組合
CREATE TABLE IF NOT EXISTS portfolios ( -- 多種幣對（部位）的組合
    id BIGSERIAL PRIMARY KEY,
    time_stamp TIMESTAMP NOT NULL,
    trading_pair_id BIGINT NOT NULL, -- 幣對 ID
    qty INTEGER NOT NULL,
    admin_account_id BIGINT NOT NULL,
    trader_account_id BIGINT NOT NULL,
    portfolio_type INTEGER NOT NULL, -- 0: 一般策略, 1: 雜費, 2: 剩餘資金
    available_balance DECIMAL(28, 8) NOT NULL, -- 可動用資金
    lock_state INTEGER NOT NULL, -- 0: no lock, 1: sell-locked, 2: buy-locked, 3: trade-locked, 4: 限制 Portfolio 在 Total PnL 達到一個數值的時候不能 buy/sell
    lock_threashold DECIMAL(28, 8) NOT NULL DEFAULT 0.0
);

-- 帳戶
CREATE TABLE IF NOT EXISTS accounts (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE, -- 電子郵件地址，必填且唯一
    username VARCHAR(50) NOT NULL, -- 使用者名稱，必填
    password VARCHAR(100) NOT NULL, -- 密碼，必填
    full_name VARCHAR(100), -- 使用者全名
    phone_number VARCHAR(20), -- 電話號碼
    date_of_birth DATE, -- 出生日期
    time_stamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 帳戶建立時間，預設為當前時間
    account_type INTEGER, -- 0: admin, 1: trader, 2: customer
    balance DECIMAL(28, 8) NOT NULL DEFAULT 0.0, -- 資金餘額
    admin_account_id BIGINT DEFAULT currval('accounts_id_seq') -- 預設為自己的ID
);

-- 帳戶內轉帳請求
CREATE TABLE IF NOT EXISTS intra_account_transfer_requests (
    id BIGSERIAL PRIMARY KEY,
    admin_account_id BIGINT NOT NULL,
    trader_account_id BIGINT NOT NULL,
    position_id BIGINT NOT NULL, -- 要轉移的部位 ID
    from_portfolio_id BIGINT NOT NULL,
    to_portfolio_id BIGINT NOT NULL,
    price DECIMAL(28, 8) NOT NULL,
    quantity DECIMAL(28, 8) NOT NULL,
    fee DECIMAL(28, 8) NOT NULL,
    is_approved BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 跨帳戶轉帳請求
CREATE TABLE IF NOT EXISTS inter_account_transfer_requests (
    id BIGSERIAL PRIMARY KEY,
    admin_account_id BIGINT NOT NULL,
    from_account_id BIGINT NOT NULL,
    to_account_id BIGINT NOT NULL,
    price DECIMAL(28, 8) NOT NULL,
    quantity DECIMAL(28, 8) NOT NULL,
    fee DECIMAL(28, 8) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- After all tables are created, add foreign key constraints
ALTER TABLE sessions ADD CONSTRAINT fk_sessions_user_id FOREIGN KEY (user_id) REFERENCES accounts(id) ON DELETE CASCADE;

ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_api_keys_id FOREIGN KEY (api_keys_id) REFERENCES exchange_api_keys(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_pair_id FOREIGN KEY (pair_id) REFERENCES trading_pairs(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_strategy_id FOREIGN KEY (strategy_id) REFERENCES trading_strategy(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_risk_id FOREIGN KEY (risk_id) REFERENCES risk_management(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_logs_id FOREIGN KEY (logs_id) REFERENCES logging_monitoring_settings(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_ma_id FOREIGN KEY (ma_id) REFERENCES moving_average_indicator(id);
ALTER TABLE bbgo_start_info ADD CONSTRAINT fk_bbgo_rs_id FOREIGN KEY (rs_id) REFERENCES rs_indicator(id);

ALTER TABLE trading_pairs ADD CONSTRAINT fk_base_currency_id FOREIGN KEY (base_currency_id) REFERENCES currencies(id);
ALTER TABLE trading_pairs ADD CONSTRAINT fk_quote_currency_id FOREIGN KEY (quote_currency_id) REFERENCES currencies(id);

ALTER TABLE orders ADD CONSTRAINT fk_trading_pair_id FOREIGN KEY (trading_pair_id) REFERENCES trading_pairs(id);

ALTER TABLE quotations ADD CONSTRAINT fk_base_currency_id FOREIGN KEY (base_currency_id) REFERENCES currencies(id);

ALTER TABLE positions ADD CONSTRAINT fk_trading_pair_id FOREIGN KEY (trading_pair_id) REFERENCES trading_pairs(id);
ALTER TABLE positions ADD CONSTRAINT fk_quotation_id FOREIGN KEY (quotation_id) REFERENCES quotations(id);

ALTER TABLE portfolios ADD CONSTRAINT fk_trading_pair_id FOREIGN KEY (trading_pair_id) REFERENCES trading_pairs(id);
ALTER TABLE portfolios ADD CONSTRAINT fk_admin_account_id FOREIGN KEY (admin_account_id) REFERENCES accounts(id);
ALTER TABLE portfolios ADD CONSTRAINT fk_trader_account_id FOREIGN KEY (trader_account_id) REFERENCES accounts(id);

ALTER TABLE accounts ADD CONSTRAINT fk_admin_account_id FOREIGN KEY (admin_account_id) REFERENCES accounts(id);

ALTER TABLE intra_account_transfer_requests ADD CONSTRAINT fk_admin_account_id FOREIGN KEY (admin_account_id) REFERENCES accounts(id);
ALTER TABLE intra_account_transfer_requests ADD CONSTRAINT fk_trader_account_id FOREIGN KEY (trader_account_id) REFERENCES accounts(id);
ALTER TABLE intra_account_transfer_requests ADD CONSTRAINT fk_position_id FOREIGN KEY (position_id) REFERENCES positions(id);
ALTER TABLE intra_account_transfer_requests ADD CONSTRAINT fk_from_portfolio_id FOREIGN KEY (from_portfolio_id) REFERENCES portfolios(id);
ALTER TABLE intra_account_transfer_requests ADD CONSTRAINT fk_to_portfolio_id FOREIGN KEY (to_portfolio_id) REFERENCES portfolios(id);

ALTER TABLE inter_account_transfer_requests ADD CONSTRAINT fk_admin_account_id FOREIGN KEY (admin_account_id) REFERENCES accounts(id);
ALTER TABLE inter_account_transfer_requests ADD CONSTRAINT fk_from_account_id FOREIGN KEY (from_account_id) REFERENCES accounts(id);
ALTER TABLE inter_account_transfer_requests ADD CONSTRAINT fk_to_account_id FOREIGN KEY (to_account_id) REFERENCES accounts(id);