const Database = require('better-sqlite3');
const path = require('path');

const dbPath = process.env.DATABASE_PATH || path.join(__dirname, '..', 'ecommerce.db');
const db = new Database(dbPath);

// Enable foreign keys and WAL mode for high performance
db.pragma('foreign_keys = ON');
db.pragma('journal_mode = WAL');

function initDatabase() {
  db.exec(`
    CREATE TABLE IF NOT EXISTS users (
      id TEXT PRIMARY KEY,
      email TEXT UNIQUE NOT NULL,
      password_hash TEXT NOT NULL,
      role TEXT NOT NULL DEFAULT 'customer',
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS categories (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      slug TEXT UNIQUE NOT NULL,
      parent_id TEXT REFERENCES categories(id)
    );

    CREATE TABLE IF NOT EXISTS products (
      id TEXT PRIMARY KEY,
      category_id TEXT NOT NULL REFERENCES categories(id),
      title TEXT NOT NULL,
      slug TEXT UNIQUE NOT NULL,
      description TEXT NOT NULL,
      price REAL NOT NULL,
      sale_price REAL,
      stock_quantity INTEGER NOT NULL DEFAULT 0,
      rating_avg REAL DEFAULT 5.0,
      rating_count INTEGER DEFAULT 0,
      images_json TEXT NOT NULL,
      is_featured INTEGER DEFAULT 0,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS orders (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id),
      status TEXT NOT NULL DEFAULT 'pending',
      total_amount REAL NOT NULL,
      coupon_code TEXT,
      shipping_address_json TEXT NOT NULL,
      payment_method TEXT NOT NULL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS order_items (
      id TEXT PRIMARY KEY,
      order_id TEXT NOT NULL REFERENCES orders(id),
      product_id TEXT NOT NULL REFERENCES products(id),
      quantity INTEGER NOT NULL,
      unit_price REAL NOT NULL
    );

    CREATE TABLE IF NOT EXISTS coupons (
      id TEXT PRIMARY KEY,
      code TEXT UNIQUE NOT NULL,
      discount_percent REAL NOT NULL DEFAULT 0,
      discount_type TEXT NOT NULL DEFAULT 'percentage',
      discount_amount REAL DEFAULT 0,
      max_uses INTEGER NOT NULL,
      current_uses INTEGER DEFAULT 0,
      valid_until TIMESTAMP NOT NULL
    );

    -- Create indexes for performance (<50ms response requirement for search)
    CREATE INDEX IF NOT EXISTS idx_products_slug ON products(slug);
    CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
    CREATE INDEX IF NOT EXISTS idx_products_price ON products(price);
    CREATE INDEX IF NOT EXISTS idx_products_rating ON products(rating_avg);
    CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug);
  `);

  // Migrate coupons table if missing new columns
  const couponColumns = db.prepare('PRAGMA table_info(coupons)').all().map(c => c.name);
  if (!couponColumns.includes('discount_type')) {
    db.exec("ALTER TABLE coupons ADD COLUMN discount_type TEXT NOT NULL DEFAULT 'percentage'");
  }
  if (!couponColumns.includes('discount_amount')) {
    db.exec('ALTER TABLE coupons ADD COLUMN discount_amount REAL DEFAULT 0');
  }

  return db;
}

module.exports = {
  db,
  initDatabase
};
