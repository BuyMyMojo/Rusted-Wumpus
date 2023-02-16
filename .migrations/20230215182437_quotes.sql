-- Table: public.quotes

-- DROP TABLE IF EXISTS public.quotes;

CREATE TABLE IF NOT EXISTS public.quotes
(
    id text COLLATE pg_catalog."default" NOT NULL DEFAULT generate_uid(8),
    quote character varying(512) COLLATE pg_catalog."default" NOT NULL,
    author text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT quotes_pkey PRIMARY KEY (id),
    CONSTRAINT quotes_author_fkey FOREIGN KEY (author)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.quotes
    OWNER to postgres;