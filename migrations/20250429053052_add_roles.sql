INSERT INTO
    role (
        role_name,
        purchasing,
        product_edit_any,
        seller_edit_any,
        acct_edit_any
    )
VALUES
    ('customer', true, false, false, false);

INSERT INTO
    role (
        role_name,
        purchasing,
        product_edit_any,
        seller_edit_any,
        acct_edit_any
    )
VALUES
    ('manager', true, true, false, false);

INSERT INTO
    role (
        role_name,
        purchasing,
        product_edit_any,
        seller_edit_any,
        acct_edit_any
    )
VALUES
    ('admin', true, true, true, true);
