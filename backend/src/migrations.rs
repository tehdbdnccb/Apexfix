use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Enable UUID extension
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
        .execute(pool)
        .await?;

    // Enable PostGIS extension for geographic queries
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"postgis\";")
        .execute(pool)
        .await;

    // Users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            full_name VARCHAR(255) NOT NULL,
            phone_number VARCHAR(20),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Technicians table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS technicians (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            shop_name VARCHAR(255) NOT NULL,
            location_name VARCHAR(255) NOT NULL,
            latitude DOUBLE PRECISION NOT NULL,
            longitude DOUBLE PRECISION NOT NULL,
            certification_level FLOAT NOT NULL DEFAULT 1.0,
            part_authenticity_score FLOAT NOT NULL DEFAULT 1.0,
            speed_score FLOAT NOT NULL DEFAULT 1.0,
            is_verified BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Repair bookings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS repair_bookings (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            technician_id UUID NOT NULL REFERENCES technicians(id) ON DELETE CASCADE,
            device_model VARCHAR(255) NOT NULL,
            issue_description TEXT NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
            agreed_price DOUBLE PRECISION NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_technicians_location ON technicians(latitude, longitude);")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_user_id ON repair_bookings(user_id);")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_technician_id ON repair_bookings(technician_id);")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);")
        .execute(pool)
        .await?;

    println!("✅ Database migrations completed successfully!");
    Ok(())
}

