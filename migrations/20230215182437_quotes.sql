-- Table: public.quotes

-- DROP TABLE IF EXISTS public.quotes;

CREATE TABLE IF NOT EXISTS public.quotes
(
    id text COLLATE pg_catalog."default" NOT NULL DEFAULT generate_uid(8),
    quote character varying(512) COLLATE pg_catalog."default" NOT NULL,
    author text REFERENCES public.users (id) NOT NULL,
    CONSTRAINT pk_quotes PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.quotes
    OWNER to postgres;