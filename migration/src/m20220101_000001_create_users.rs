use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
        .create_table(
            sea_query::Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                    .text()
                    .not_null()
                    .primary_key()
                )
                .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).if_exists().to_owned())
        .await?;


        Ok(())
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
}