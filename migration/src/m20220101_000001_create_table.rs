use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::IsDel).tiny_integer().not_null().default(0).comment("0-未删除1-已删除"))
                    .col(ColumnDef::new(Post::CreatedAt).date_time().comment("创建时间"))
                    .col(ColumnDef::new(Post::UpdatedAt).date_time().comment("更新时间"))
                    .col(ColumnDef::new(Post::DeletedAt).date_time().comment("删除时间"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(Table::create()
                .table(Tags::Table)
                .if_not_exists()
                .col(ColumnDef::new(Tags::Id).big_integer().auto_increment().primary_key().not_null())
                              .col(ColumnDef::new(Tags::Name).string().not_null())
                              .col(ColumnDef::new(Tags::IsDel).not_null().boolean().default(false))
                              .to_owned(),
            )
            .await?;
        manager
            .create_table(Table::create()
                              .table(PostTags::Table)
                              .if_not_exists()
                              .col(ColumnDef::new(PostTags::Id).big_integer().auto_increment().primary_key().not_null())
                              .col(ColumnDef::new(PostTags::PostId).big_integer().not_null())
                              .col(ColumnDef::new(PostTags::TagId).big_integer().not_null())
                              .col(ColumnDef::new(PostTags::IdDel).not_null().boolean().default(false))
                              .to_owned(),
            )
            .await?;
            manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Uuid).string().not_null())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(ColumnDef::new(User::Email).string())
                    .col(ColumnDef::new(User::IsDel).tiny_integer().default(0).not_null())
                    .col(ColumnDef::new(User::CreatedAt).date_time().null())
                    .col(ColumnDef::new(User::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(User::DeletedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PostTags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    IsDel,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
    Name,
    IsDel,
}
#[derive(DeriveIden)]
enum  PostTags {
    Table,
    Id,
    PostId,
    TagId,
    IdDel,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Uuid,
    Phone,
    Username,
    Password,
    Avatar,
    Email,
    IsDel,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}