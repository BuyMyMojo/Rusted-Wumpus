use sea_orm_migration::{ prelude::*, sea_orm::ConnectionTrait };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE OR REPLACE FUNCTION generate_uid(
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
            
            ALTER FUNCTION generate_uid(integer)
                OWNER TO postgres;"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP FUNCTION IF EXISTS generate_uid(integer);").await?;

        Ok(())
    }
}