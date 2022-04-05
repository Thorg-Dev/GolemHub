--
-- PostgreSQL database dump
--

-- Dumped from database version 14.2
-- Dumped by pg_dump version 14.2

-- Started on 2022-04-05 15:43:28

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
-- TOC entry 3313 (class 1262 OID 16394)
-- Name: GolemHub; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE "GolemHub" WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE = 'English_United States.1252';


ALTER DATABASE "GolemHub" OWNER TO postgres;

\connect "GolemHub"

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 210 (class 1259 OID 24584)
-- Name: projects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.projects (
    id integer NOT NULL,
    name character varying,
    homepage character varying,
    developer character varying,
    images character varying[],
    icon bytea
);


ALTER TABLE public.projects OWNER TO postgres;

--
-- TOC entry 209 (class 1259 OID 24583)
-- Name: Projects_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Projects_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Projects_id_seq" OWNER TO postgres;

--
-- TOC entry 3314 (class 0 OID 0)
-- Dependencies: 209
-- Name: Projects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Projects_id_seq" OWNED BY public.projects.id;


--
-- TOC entry 3164 (class 2604 OID 24587)
-- Name: projects id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public."Projects_id_seq"'::regclass);


--
-- TOC entry 3307 (class 0 OID 24584)
-- Dependencies: 210
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.projects (id, name, homepage, developer, images, icon) FROM stdin;
\.


--
-- TOC entry 3315 (class 0 OID 0)
-- Dependencies: 209
-- Name: Projects_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Projects_id_seq"', 5, true);


--
-- TOC entry 3166 (class 2606 OID 24591)
-- Name: projects Projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT "Projects_pkey" PRIMARY KEY (id);


-- Completed on 2022-04-05 15:43:28

--
-- PostgreSQL database dump complete
--

