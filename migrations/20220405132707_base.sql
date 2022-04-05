-- Add migration script here
CREATE TABLE public.projects (
                                 id integer NOT NULL,
                                 name character varying,
                                 homepage character varying,
                                 developer character varying,
                                 images character varying[],
                                 icon bytea
);


ALTER TABLE public.projects OWNER TO postgres;

CREATE SEQUENCE public."Projects_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Projects_id_seq" OWNER TO postgres;

ALTER SEQUENCE public."Projects_id_seq" OWNED BY public.projects.id;

ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public."Projects_id_seq"'::regclass);

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT "Projects_pkey" PRIMARY KEY (id);