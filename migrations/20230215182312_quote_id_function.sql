-- FUNCTION: public.generate_uid(integer)

-- DROP FUNCTION IF EXISTS public.generate_uid(integer);

CREATE OR REPLACE FUNCTION public.generate_uid(
	size integer)
    RETURNS text
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE PARALLEL UNSAFE
AS $BODY$
DECLARE
  characters TEXT := 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  bytes BYTEA := gen_random_bytes(size);
  l INT := length(characters);
  i INT := 0;
  output TEXT := '';
BEGIN
  WHILE i < size LOOP
    output := output || substr(characters, get_byte(bytes, i) % l + 1, 1);
    i := i + 1;
  END LOOP;
  RETURN output;
END;
$BODY$;

ALTER FUNCTION public.generate_uid(integer)
    OWNER TO postgres;