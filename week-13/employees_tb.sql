--
-- PostgreSQL database dump
--

\restrict aLBEgsAq3rCk43GKS8v3gbWgGhh8O5acE9KAzifizXJ3of0tOAhv6ViCSnb7XdG

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
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict aLBEgsAq3rCk43GKS8v3gbWgGhh8O5acE9KAzifizXJ3of0tOAhv6ViCSnb7XdG

