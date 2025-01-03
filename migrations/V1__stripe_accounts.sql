CREATE OR REPLACE FUNCTION updated_at_now()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE stripe_accounts (
  stripe_account_id VARCHAR NOT NULL PRIMARY KEY,
  market_booth_id UUID NOT NULL,
  user_id VARCHAR NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_shops_updated_at BEFORE UPDATE
    ON stripe_accounts FOR EACH ROW EXECUTE PROCEDURE 
    updated_at_now();
