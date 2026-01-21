--
-- PostgreSQL database dump
--

\restrict DNeK2Vo3VMtrPszuwC5RSkQJ1crlkJzvCgUclVRFjbm8bKY3JOCThTSH9T09Q1t

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer CONSTRAINT dataplan_table_data_id_not_null NOT NULL,
    data_size text CONSTRAINT dataplan_table_data_size_not_null NOT NULL,
    data_duration integer CONSTRAINT dataplan_table_data_duration_not_null NOT NULL,
    data_price real
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_management integer NOT NULL,
    no integer NOT NULL,
    name text NOT NULL,
    location text NOT NULL,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: employees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employees (
    id integer NOT NULL,
    name text NOT NULL,
    no integer NOT NULL,
    salary real,
    age integer,
    phone real
);


ALTER TABLE public.employees OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    no integer NOT NULL,
    name text NOT NULL,
    duration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    id integer CONSTRAINT customer_table_id_not_null NOT NULL,
    name text CONSTRAINT customer_table_name_not_null NOT NULL,
    email text CONSTRAINT customer_table_email_not_null NOT NULL,
    mobile real CONSTRAINT customer_table_mobile_not_null NOT NULL,
    eid integer,
    data_id integer CONSTRAINT customer_table_data_id_not_null NOT NULL,
    age integer CONSTRAINT customer_table_age_not_null NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_management, no, name, location, pno) FROM stdin;
108	1	ADMINISTRATION	IKEJA	44
101	2	ACCOUNT	EGBEDA	11
100	3	PACKAGING	AJAH	44
120	4	ESEARCH	V.I	33
97	5	ACCOUNT	MAGODO	22
122	6	OPERATIONS	MILE 2	44
107	7	PACKAGING	KETU	55
\.


--
-- Data for Name: employees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employees (id, name, no, salary, age, phone) FROM stdin;
101	ALADE JOY	2	250000	33	8.0230897e+09
100	MUSTAPHA ALI	3	175000	32	8.063286e+09
107	ALOKWE MARTIN	7	380000	48	7.090083e+09
97	DANKADE AMINAT	5	550000	40	9.023689e+09
108	JOSIAH JOSHUA	1	120000	30	8.053759e+09
102	MAKINDE MARY	2	450000	55	9.0234784e+08
120	ADELEKE JANE	4	2e+06	38	7.061046e+09
122	OSAHON MARK	6	320000	44	8.02229e+09
117	SULEMAN AJAYI	3	800000	50	7.0300897e+09
104	KUTI LAWAL	1	750000	35	9.14569e+09
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (no, name, duration, project_managerid) FROM stdin;
11	A	9_MONTHS	102
22	B	14_MONTHS	97
33	C	16_MONTHS	120
44	D	25_MONTHS	108
55	E	9_MONTHS	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (id, name, email, mobile, eid, data_id, age) FROM stdin;
110	MUSTA KARIM	M_KARIM@GMAIL.COM	8.055089e+09	102	5	35
111	LILIAN JAIYA	I_JAIYE@GMAIL.COM	8.0551854e+09	100	3	43
112	ARTHUR MUSA	A_MUSA@GMAIL.COM	7.055703e+09	107	10	50
113	PHILIP AKONJO	P_AKONJO@GMAIL.COM	9.052317e+09	100	2	41
114	MARYLENE MAPA	M_MAPA@GMAIL.COM	8.0868285e+09	120	5	33
115	OGHENERO AGOR	O_AGOR@GMAIL.COM	8.031684e+09	117	11	50
116	ADAMS BREE	A_BREE@GMAIL.COM	8.094236e+09	102	1	33
117	OKAFOR  MATHIAS	O_MATHIUS@GMAIL.COM	8.037219e+09	120	10	45
118	SAMSON ADELEKE	S_ADELEKE@GMAIL.COM	7.0538813e+09	117	11	65
119	LAWAL JOB	L_JOB@GMAIL.COM	8.057798e+09	100	8	44
120	MATTHEW JAKANDE	M_JAKANDE@GMAIL.COM	7.0545377e+09	120	2	21
122	JIMILIA ADEGBOYE	J_ADEGBOYE@GMAIL.COM	8.0796636e+09	107	5	20
\.


--
-- Name: staff customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (id);


--
-- Name: dataplan dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_management);


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict DNeK2Vo3VMtrPszuwC5RSkQJ1crlkJzvCgUclVRFjbm8bKY3JOCThTSH9T09Q1t

