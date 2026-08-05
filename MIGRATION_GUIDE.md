# Running Database Migrations

Your app uses SQLx for database migrations. To set up the database schema:

## Option 1: Manual SQL (Quick)
Connect to your Railway PostgreSQL database and run the SQL in `backend/migrations/001_init.sql`:

```bash
psql $DATABASE_URL < backend/migrations/001_init.sql
```

## Option 2: Using SQLx CLI (Recommended)
Install sqlx-cli:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Run migrations:
```bash
sqlx migrate run --database-url $DATABASE_URL
```

## Option 3: Railway Console
1. Go to your Railway dashboard
2. Open the Postgres service
3. Click "Connect" → "Database" tab
4. Use the connection string to connect via psql or your DB client
5. Paste the SQL from `backend/migrations/001_init.sql`

## Tables Created
- **users**: User accounts with email/password
- **technicians**: Repair technicians with location and scores
- **repair_bookings**: Repair job bookings linking users and technicians

All migrations are in `backend/migrations/001_init.sql`.

