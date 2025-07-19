-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: Ticker
-- ------------------------------

DEFINE TABLE Ticker TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD components ON Ticker TYPE array<record<underlying_stock>> PERMISSIONS FULL;
DEFINE FIELD components[*] ON Ticker TYPE record<underlying_stock> PERMISSIONS FULL;
DEFINE FIELD current_market_price ON Ticker TYPE number PERMISSIONS FULL;
DEFINE FIELD historical_price_data ON Ticker TYPE record<historial_price_data> PERMISSIONS FULL;
DEFINE FIELD symbol ON Ticker TYPE string PERMISSIONS FULL;
DEFINE FIELD technicals ON Ticker TYPE record<technicals> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: Ticker
-- ------------------------------


-- ------------------------------
-- TABLE: historial_price_data
-- ------------------------------

DEFINE TABLE historial_price_data TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD close ON historial_price_data TYPE number PERMISSIONS FULL;
DEFINE FIELD high ON historial_price_data TYPE number PERMISSIONS FULL;
DEFINE FIELD low ON historial_price_data TYPE number PERMISSIONS FULL;
DEFINE FIELD open ON historial_price_data TYPE number PERMISSIONS FULL;
DEFINE FIELD t ON historial_price_data TYPE datetime PERMISSIONS FULL;
DEFINE FIELD volume ON historial_price_data TYPE number PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: historial_price_data
-- ------------------------------


-- ------------------------------
-- TABLE: order
-- ------------------------------

DEFINE TABLE order TYPE RELATION IN Ticker OUT trade SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD asset_class ON order TYPE 'call' | 'put' | 'common_stock' PERMISSIONS FULL;
DEFINE FIELD in ON order TYPE record<Ticker> PERMISSIONS FULL;
DEFINE FIELD operation ON order TYPE 'buy' | 'sell' PERMISSIONS FULL;
DEFINE FIELD order_type ON order TYPE 'stop_loss' | 'take_profit' | 'standard' PERMISSIONS FULL;
DEFINE FIELD out ON order TYPE record<trade> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: order
-- ------------------------------


-- ------------------------------
-- TABLE: technicals
-- ------------------------------

DEFINE TABLE technicals TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD macd ON technicals TYPE array<number> PERMISSIONS FULL;
DEFINE FIELD macd[*] ON technicals TYPE number PERMISSIONS FULL;
DEFINE FIELD rsi ON technicals TYPE array<number> PERMISSIONS FULL;
DEFINE FIELD rsi[*] ON technicals TYPE number PERMISSIONS FULL;
DEFINE FIELD sma_100 ON technicals TYPE array<number> PERMISSIONS FULL;
DEFINE FIELD sma_100[*] ON technicals TYPE number PERMISSIONS FULL;
DEFINE FIELD sma_50 ON technicals TYPE array<number> PERMISSIONS FULL;
DEFINE FIELD sma_50[*] ON technicals TYPE number PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: technicals
-- ------------------------------


-- ------------------------------
-- TABLE: trade
-- ------------------------------

DEFINE TABLE trade TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD asset_type ON trade TYPE 'option' | 'common_stock' PERMISSIONS FULL;
DEFINE FIELD market_value ON trade TYPE number PERMISSIONS FULL;
DEFINE FIELD num_shares ON trade TYPE number PERMISSIONS FULL;
DEFINE FIELD symbol ON trade TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: trade
-- ------------------------------


-- ------------------------------
-- TABLE: underlying_stock
-- ------------------------------

DEFINE TABLE underlying_stock TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD symbol ON underlying_stock TYPE string PERMISSIONS FULL;
DEFINE FIELD symbol_market_price ON underlying_stock TYPE number PERMISSIONS FULL;
DEFINE FIELD technicals ON underlying_stock TYPE record<technicals> PERMISSIONS FULL;
DEFINE FIELD ticker_weighting ON underlying_stock TYPE number PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: underlying_stock
-- ------------------------------


