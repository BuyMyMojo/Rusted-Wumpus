use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub struct GenerateUid;

impl Iden for GenerateUid {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "generate_uid").unwrap();
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quote::Id)
                            .text()
                            .not_null()
                            .primary_key()
                            .default(Func::cust(GenerateUid).arg(8)),
                    )
                    .col(ColumnDef::new(Quote::Quote).char_len(512).not_null())
                    .col(ColumnDef::new(Quote::Author).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_quote_author")
                            .from(Quote::Table, Quote::Author)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-quote_content")
                    .table(Quote::Table)
                    .col(Quote::Quote)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-quote_content").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Quote {
    Table,
    Id,
    Quote,
    Author,
}
