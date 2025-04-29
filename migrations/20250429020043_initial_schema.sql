-- (User) Acct
CREATE TABLE IF NOT EXISTS acct (
    acct_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    acct_email text NOT NULL,
    acct_country text NOT NULL,
    acct_password text NOT NULL
);

-- Seller
CREATE TABLE IF NOT EXISTS seller (
    seller_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    seller_country text NOT NULL,
    seller_name text NOT NULL
);

-- (Order) Purchase
CREATE TABLE IF NOT EXISTS purchase (
    purchase_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    purchase_address text NOT NULL,
    purchase_date timestamp NOT NULL,
    -- OrderedBy
    acct_key int REFERENCES acct
);

CREATE TABLE IF NOT EXISTS product (
    product_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    product_name text NOT NULL,
    product_desc text NOT NULL,
    product_price money NOT NULL,
    product_qty int NOT NULL,
    product_listed bool NOT NULL,
    -- SoldBy
    seller_key int REFERENCES seller
);

-- OrderOf (needs its own primary- multiple instances of the same product could be in order)
CREATE TABLE IF NOT EXISTS purchase_item (
    purchase_item_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    purchased_price money NOT NULL,
    purchase_key int REFERENCES purchase,
    product_key int REFERENCES product
);

-- HasInCart
CREATE TABLE IF NOT EXISTS cart_item (
    cart_item_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    acct_key int REFERENCES acct,
    product_key int REFERENCES product
);

-- This doesn't match up perfectly with the ERD, but this design for roles makes more sense
-- UserRole
CREATE TABLE IF NOT EXISTS role (
    role_key int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    role_name text UNIQUE NOT NULL,
    purchasing bool NOT NULL, -- can place order
    product_edit_any bool NOT NULL, -- can create and any products
    seller_edit_any bool NOT NULL, -- can edit any sellers
    acct_edit_any bool NOT NULL -- can edit any users
);

-- HasRole
CREATE TABLE IF NOT EXISTS acct_role (
    role_key int REFERENCES role,
    acct_key int REFERENCES acct
);

--AdminOf
CREATE TABLE IF NOT EXISTS seller_admin_role (
    role_key int REFERENCES role,
    seller_key int REFERENCES seller
);
