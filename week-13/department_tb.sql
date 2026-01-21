--
-- PostgreSQL database dump
--

\restrict JHcxsP7dOGCBdG1WhDlg3x4aSYdqcsUY7E3BV06UnPWxXAABn6GbaSL31D1IODk

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
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_management);


--
-- PostgreSQL database dump complete
--

\unrestrict JHcxsP7dOGCBdG1WhDlg3x4aSYdqcsUY7E3BV06UnPWxXAABn6GbaSL31D1IODk

