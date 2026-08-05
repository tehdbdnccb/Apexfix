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

    // Technicians table with comprehensive fields
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS technicians (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            shop_name VARCHAR(255) NOT NULL,
            phone_number VARCHAR(20),
            shop_address TEXT,
            location_name VARCHAR(255) NOT NULL,
            latitude DOUBLE PRECISION NOT NULL,
            longitude DOUBLE PRECISION NOT NULL,
            certification_level DOUBLE PRECISION NOT NULL DEFAULT 1.0,
            part_authenticity_score DOUBLE PRECISION NOT NULL DEFAULT 1.0,
            speed_score DOUBLE PRECISION NOT NULL DEFAULT 1.0,
            years_experience INTEGER DEFAULT 1,
            specializations TEXT DEFAULT '[]',
            bio TEXT,
            rating DOUBLE PRECISION,
            total_reviews INTEGER DEFAULT 0,
            completion_rate DOUBLE PRECISION DEFAULT 0.0,
            response_time_hours INTEGER DEFAULT 24,
            is_verified BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
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

    // Create indexes for optimal performance
    
    // Location-based queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_location ON technicians(latitude, longitude);"
    )
    .execute(pool)
    .await?;

    // Verification status (for filtering verified only)
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_verified ON technicians(is_verified);"
    )
    .execute(pool)
    .await?;

    // Matching algorithm scores
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_certification ON technicians(certification_level DESC);"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_authenticity ON technicians(part_authenticity_score DESC);"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_speed ON technicians(speed_score DESC);"
    )
    .execute(pool)
    .await?;

    // Rating lookup (only create if column exists after ALTER TABLE)
    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_technicians_rating ON technicians(rating DESC) WHERE rating IS NOT NULL;"
    )
    .execute(pool)
    .await;

    // Booking indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_user_id ON repair_bookings(user_id);")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_technician_id ON repair_bookings(technician_id);")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_bookings_status ON repair_bookings(status);"
    )
    .execute(pool)
    .await?;

    // User indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);")
        .execute(pool)
        .await?;

    println!("✅ Database migrations completed successfully!");
    Ok(())
}

