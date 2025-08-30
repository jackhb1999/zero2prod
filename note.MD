PS D:\rust\zero2prod> sqlx database create --database-url postgres://postgres:fackpg@localhost:5432/newsletter
PS D:\rust\zero2prod> sqlx migrate add create_subscriptions_table
Creating migrations\20250820030230_create_subscriptions_table.sql
PS D:\rust\zero2prod> sqlx migrate run
error: the `--database-url` option or the `DATABASE_URL` environment variable must be provided
PS D:\rust\zero2prod> sqlx migrate run --database-url postgres://postgres:fackpg@localhost:5432/newsletter    
Applied 20250820030230/migrate create subscriptions table (9.667ms)



