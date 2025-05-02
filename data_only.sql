--
-- PostgreSQL database dump
--

-- Dumped from database version 14.17 (Homebrew)
-- Dumped by pg_dump version 14.17 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: acct; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.acct (acct_key, acct_email, acct_country, acct_password) FROM stdin;
2	bob@example.com	US	$argon2id$v=19$m=19456,t=2,p=1$b1LockLMi88fi+1Uou1h1A$si466rnyaocyi6jHa23bthAYAwd+5JsUVOI07kW2tmw
3	sally@example.com	UK	$argon2id$v=19$m=19456,t=2,p=1$51y9Yzs10cx/KxFunylGyQ$Me7pFp3gCjW5XKVKmYhHKJlHt9VP2uX/5w/huQia4vo
4	sue@example.com	DE	$argon2id$v=19$m=19456,t=2,p=1$jAdN67t/15/evFQIxDtGCQ$rmyFkDELM46RkHRNWmnwJm1AHxlaRLPIADbbQJRiXOE
5	mary@example.com	DE	$argon2id$v=19$m=19456,t=2,p=1$qnjgVHvN7zM+Np8eeEzYug$5kG7tu2Z8K/5HibF359Rt7XQ6ORlwj0o/DdIIt96lsw
6	john@example.com	DE	$argon2id$v=19$m=19456,t=2,p=1$iPBVQQ9Zl+1rNrTotHC7SQ$FUu+3qxwbbcLC5HYEv7Zcl+cdBIyr4+2UpJUpITXI50
7	jack@example.com	DE	$argon2id$v=19$m=19456,t=2,p=1$9mqEFDqtDdWDwMls4onzIQ$wepADYPXUuhS70aSXSe2i3YfykcjAE6HKIayaMLHY4E
\.


--
-- Data for Name: role; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.role (role_key, role_name, purchasing, product_edit_any, seller_edit_any, acct_edit_any) FROM stdin;
1	customer	t	f	f	f
2	manager	t	t	f	f
3	admin	t	t	t	t
4	sue-seller	f	f	f	f
\.


--
-- Data for Name: acct_role; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.acct_role (role_key, acct_key) FROM stdin;
1	2
1	3
1	4
4	4
1	5
1	6
1	7
\.


--
-- Data for Name: seller; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.seller (seller_key, seller_country, seller_name) FROM stdin;
1	UK	Fish
2	CA	Cat
\.


--
-- Data for Name: product; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.product (product_key, product_name, product_desc, product_price, product_qty, product_listed, seller_key) FROM stdin;
1	purple compact snack	Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus	$222.58	69	t	1
2	orange durable planner	s tellus. Aenean scelerisque lorem vitae neque lacinia, suscipit convallis quam ullamcorper. Nunc sollicitudin velit turpis, et pulvinar ipsum feu	$86.95	67	t	2
3	purple eco-friendly t-shirt	eget ac enim. Proin lobortis gravida mauris et tempor. 	$199.71	42	t	2
4	red efficient e-reader	consequat auctor libero, at tincidunt arcu suscipit vitae. Cras est elit, hendrerit eget ligula a, feugiat sodales mauris. Fusce sit amet mi lacus. Vivamus in sodales sapien, non fringilla risus. Praesent dapibus purus in mollis vulputate	$365.05	61	t	1
6	orange high-quality t-shirt	Donec consequat auctor libero, at tincidunt arcu suscipit vitae. Cras est elit, hendrerit eget ligula a, feugiat sodales mauris. Fusce sit amet mi lacus. Vivamus in sodales	$17.62	11	f	2
7	orange reliable pen	que at, suscipit aliquet dolor. Aliquam et libero vel elit tincidunt semper. Aliquam condimentum turpis q	$56.70	63	t	2
8	yellow reliable smartphone	Morbi in leo enim. Nullam et lorem at turpis malesuada congue. Nam ut ligula quam. Etiam se	$499.71	16	t	2
10	pink eco-friendly planner	Lin velit turpis, et pulvinar ipsum feugiat in. Nulla eleifend egestas euismod. Sed et sem ut dolor placerat placerat. Fusce nunc nisl, blandit sit amet neque at, suscipit aliquet dolor. Aliquam et libero vel elit tincidun	$252.49	30	t	1
12	blue eco-friendly snack	ndit. Maecenas sit amet augue massa. Ut ultrices magna massa, i	$245.08	15	f	1
13	pink durable t-shirt	in velit turpis, et pulvinar ipsum feugiat in. Nulla eleifend egestas euismod. Sed et sem ut dolor placerat placerat. Fusce nunc nisl, blandit sit amet neque at, suscipit aliquet dolor. Aliquam et libero vel elit tincidun	$22.50	32	t	1
14	yellow reliable smartphone	Lorem ipsum dolor	$388.94	13	t	1
17	green portable bedding	Lorem ipsum dolor	$226.50	43	t	2
18	red lightweight marker	in velit turpis, et pulvinar ipsum feugiat in. Nulla eleifend egestas euismod. Sed et sem ut dolor placerat placerat. Fusce nunc nisl, blandit sit amet neque at, suscipit aliquet dolor. Aliquam et libero vel elit tincidun	$261.09	65	t	2
19	yellow efficient notebook	ndit. Maecenas sit amet augue massa. Ut ultrices magna massa, i	$436.58	7	t	1
9	orange innovative snack	ce sit amet mi lacus. Vivamus in sodales sapien, non fringilla risus. Praesent dapibus 	$270.67	2	t	1
11	yellow reliable pen	ce sit amet mi lacus. Vivamus in sodales sapien, non fringilla risus. Praesent dapibus 	$155.43	24	t	2
15	blue lightweight marker	Lorem ipsum dolor	$162.31	80	t	1
16	blue eco-friendly t-shirt	ndit. Maecenas sit amet augue massa. Ut ultrices magna massa, i	$151.99	20	t	2
20	green innovative bedding	ndit. Maecenas sit amet augue massa. Ut ultrices magna massa, ir	$100.46	62	f	1
5	green durable smartphone	ero, cursus nec euismod in, pellentesque hendrerit ligula. Nulla eu lorem tempus, finibu	$403.74	48	t	1
\.


--
-- Data for Name: cart_item; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.cart_item (cart_item_key, acct_key, product_key) FROM stdin;
4	6	8
5	6	7
6	7	14
8	6	13
9	7	7
10	7	17
13	6	20
15	7	8
20	5	1
21	5	14
\.


--
-- Data for Name: purchase; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.purchase (purchase_key, purchase_address, purchase_date, acct_key) FROM stdin;
1	511 e univ dr	2025-05-02 13:46:35.523395	5
2	412 postgres dr	2025-05-02 13:55:24.004573	5
\.


--
-- Data for Name: purchase_item; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.purchase_item (purchase_item_key, purchased_price, purchase_key, product_key) FROM stdin;
1	$270.67	1	9
2	$155.43	1	11
3	$162.31	1	15
4	$151.99	1	16
5	$100.46	1	20
6	$403.74	2	5
\.


--
-- Data for Name: seller_admin_role; Type: TABLE DATA; Schema: public; Owner: snek
--

COPY public.seller_admin_role (role_key, seller_key) FROM stdin;
4	1
4	2
\.


--
-- Name: acct_acct_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.acct_acct_key_seq', 7, true);


--
-- Name: cart_item_cart_item_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.cart_item_cart_item_key_seq', 21, true);


--
-- Name: product_product_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.product_product_key_seq', 20, true);


--
-- Name: purchase_item_purchase_item_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.purchase_item_purchase_item_key_seq', 6, true);


--
-- Name: purchase_purchase_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.purchase_purchase_key_seq', 2, true);


--
-- Name: role_role_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.role_role_key_seq', 4, true);


--
-- Name: seller_seller_key_seq; Type: SEQUENCE SET; Schema: public; Owner: snek
--

SELECT pg_catalog.setval('public.seller_seller_key_seq', 2, true);


--
-- PostgreSQL database dump complete
--

