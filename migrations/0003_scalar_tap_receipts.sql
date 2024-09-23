CREATE TABLE IF NOT EXISTS scalar_tap_receipts_invalid (
    id BIGSERIAL PRIMARY KEY, -- id being SERIAL is important for the function of tap-agent
    signer_address CHAR(40) NOT NULL,

    -- Values below are the individual fields of the EIP-712 receipt
    signature BYTEA NOT NULL,
    allocation_id CHAR(40) NOT NULL,
    timestamp_ns NUMERIC(20) NOT NULL,
    nonce NUMERIC(20) NOT NULL,
    value NUMERIC(39) NOT NULL
);