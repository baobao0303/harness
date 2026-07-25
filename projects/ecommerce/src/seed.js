const { db, initDatabase } = require('./db');

function createSvgDataUrl(title, bgGradientStart, bgGradientEnd, textEmoji) {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="600" height="400" viewBox="0 0 600 400">
    <defs>
      <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="${bgGradientStart}" />
        <stop offset="100%" stop-color="${bgGradientEnd}" />
      </linearGradient>
    </defs>
    <rect width="600" height="400" fill="url(#grad)" rx="16" />
    <circle cx="300" cy="170" r="70" fill="rgba(255,255,255,0.15)" />
    <text x="300" y="190" font-size="64" text-anchor="middle" dominant-baseline="middle">${textEmoji}</text>
    <text x="300" y="300" font-family="system-ui, -apple-system, sans-serif" font-size="22" font-weight="600" fill="#ffffff" text-anchor="middle">${title}</text>
  </svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}

function seedDatabase() {
  initDatabase();

  // Clear existing tables
  db.exec('DELETE FROM order_items;');
  db.exec('DELETE FROM orders;');
  db.exec('DELETE FROM products;');
  db.exec('DELETE FROM categories;');
  db.exec('DELETE FROM users;');
  db.exec('DELETE FROM coupons;');

  // Insert Users
  const insertUser = db.prepare(`
    INSERT INTO users (id, email, password_hash, role) VALUES (?, ?, ?, ?)
  `);
  insertUser.run('u_admin', 'admin@ecommerce.com', '$2a$10$hashedadminpass', 'admin');
  insertUser.run('u_customer1', 'alex@example.com', '$2a$10$hashedcustpass', 'customer');

  // Insert Categories
  const insertCategory = db.prepare(`
    INSERT INTO categories (id, name, slug, parent_id) VALUES (?, ?, ?, ?)
  `);

  // Main Categories
  insertCategory.run('cat_elec', 'Electronics', 'electronics', null);
  insertCategory.run('cat_apparel', 'Apparel', 'apparel', null);
  insertCategory.run('cat_home', 'Home & Living', 'home', null);
  insertCategory.run('cat_books', 'Books & Media', 'books', null);

  // Subcategories
  insertCategory.run('cat_audio', 'Audio & Headphones', 'audio', 'cat_elec');
  insertCategory.run('cat_laptops', 'Laptops & Computers', 'laptops', 'cat_elec');
  insertCategory.run('cat_mens', "Men's Wear", 'mens-wear', 'cat_apparel');
  insertCategory.run('cat_womens', "Women's Wear", 'womens-wear', 'cat_apparel');
  insertCategory.run('cat_kitchen', 'Kitchen & Dining', 'kitchen', 'cat_home');
  insertCategory.run('cat_furniture', 'Furniture & Decor', 'furniture', 'cat_home');
  insertCategory.run('cat_fiction', 'Fiction & Novels', 'fiction', 'cat_books');
  insertCategory.run('cat_tech_books', 'Tech & Science', 'tech-books', 'cat_books');

  // Insert Products
  const insertProduct = db.prepare(`
    INSERT INTO products (
      id, category_id, title, slug, description, price, sale_price,
      stock_quantity, rating_avg, rating_count, images_json, is_featured
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `);

  const productsData = [
    {
      id: 'p_1',
      category_id: 'cat_audio',
      title: 'AuraSound Wireless ANC Headphones',
      slug: 'aurasound-wireless-anc-headphones',
      description: 'Premium active noise-canceling wireless headphones with 40-hour battery life, spatial audio, and memory foam ear cushions.',
      price: 299.99,
      sale_price: 249.99,
      stock_quantity: 24,
      rating_avg: 4.8,
      rating_count: 142,
      images: [
        createSvgDataUrl('AuraSound ANC', '#4f46e5', '#9333ea', '🎧'),
        createSvgDataUrl('AuraSound Side View', '#3b82f6', '#1d4ed8', '🎵'),
        createSvgDataUrl('AuraSound Case', '#6366f1', '#4338ca', '💼')
      ],
      is_featured: 1
    },
    {
      id: 'p_2',
      category_id: 'cat_laptops',
      title: 'ZenBook Pro Ultra 15" M3',
      slug: 'zenbook-pro-ultra-15-m3',
      description: 'Ultra-thin aluminum chassis, 3.2K OLED Touch Display, 32GB RAM, 1TB NVMe SSD, designed for creative professionals.',
      price: 1399.00,
      sale_price: 1249.00,
      stock_quantity: 8,
      rating_avg: 4.9,
      rating_count: 89,
      images: [
        createSvgDataUrl('ZenBook Pro 15', '#0f172a', '#334155', '💻'),
        createSvgDataUrl('OLED Display Detail', '#1e293b', '#475569', '🖥️')
      ],
      is_featured: 1
    },
    {
      id: 'p_3',
      category_id: 'cat_audio',
      title: 'SonicPulse Portable Bluetooth Speaker',
      slug: 'sonicpulse-portable-bluetooth-speaker',
      description: 'IPX7 waterproof outdoor Bluetooth 5.3 speaker with deep bass radiator and 20-hour playback.',
      price: 89.99,
      sale_price: 69.99,
      stock_quantity: 45,
      rating_avg: 4.6,
      rating_count: 67,
      images: [
        createSvgDataUrl('SonicPulse Speaker', '#059669', '#10b981', '🔊')
      ],
      is_featured: 0
    },
    {
      id: 'p_4',
      category_id: 'cat_mens',
      title: 'Merino Wool Ergonomic Crewneck Sweater',
      slug: 'merino-wool-ergonomic-crewneck-sweater',
      description: '100% Australian Merino Wool knit sweater offering breathable warmth, anti-odor properties, and tailored modern fit.',
      price: 110.00,
      sale_price: 85.00,
      stock_quantity: 30,
      rating_avg: 4.7,
      rating_count: 54,
      images: [
        createSvgDataUrl('Merino Wool Sweater', '#d97706', '#b45309', '🧶'),
        createSvgDataUrl('Fabric Close-up', '#f59e0b', '#d97706', '👕')
      ],
      is_featured: 1
    },
    {
      id: 'p_5',
      category_id: 'cat_womens',
      title: 'StormShield Waterproof Parka Coat',
      slug: 'stormshield-waterproof-parka-coat',
      description: 'Triple-layer breathable membrane with sealed seams, adjustable hood, and fleece-lined handwarmer pockets.',
      price: 189.99,
      sale_price: null,
      stock_quantity: 15,
      rating_avg: 4.5,
      rating_count: 38,
      images: [
        createSvgDataUrl('StormShield Parka', '#0284c7', '#0369a1', '🧥')
      ],
      is_featured: 0
    },
    {
      id: 'p_6',
      category_id: 'cat_mens',
      title: 'Japanese Selvage Indigo Denim Shirt',
      slug: 'japanese-selvage-indigo-denim-shirt',
      description: 'Crafted from 12oz shuttle-loomed Japanese denim with brass snap buttons and reinforced stitching.',
      price: 79.99,
      sale_price: 64.99,
      stock_quantity: 0, // Out of stock example
      rating_avg: 4.4,
      rating_count: 29,
      images: [
        createSvgDataUrl('Indigo Denim Shirt', '#1e40af', '#1e3a8a', '👔')
      ],
      is_featured: 0
    },
    {
      id: 'p_7',
      category_id: 'cat_kitchen',
      title: 'BaristaTouch Smart Espresso Machine',
      slug: 'baristatouch-smart-espresso-machine',
      description: '19-bar Italian pump pressure, integrated conical burr grinder, automatic milk micro-foaming, and intuitive touchscreen interface.',
      price: 599.00,
      sale_price: 499.00,
      stock_quantity: 10,
      rating_avg: 4.9,
      rating_count: 215,
      images: [
        createSvgDataUrl('BaristaTouch Espresso', '#78350f', '#451a03', '☕'),
        createSvgDataUrl('Steam Wand Detail', '#92400e', '#78350f', '🥛')
      ],
      is_featured: 1
    },
    {
      id: 'p_8',
      category_id: 'cat_furniture',
      title: 'Nordic Velvet Accent Armchair',
      slug: 'nordic-velvet-accent-armchair',
      description: 'Solid oak wood frame wrapped in stain-resistant velvet fabric. Ergonomic curved backrest for supreme living room comfort.',
      price: 349.00,
      sale_price: null,
      stock_quantity: 6,
      rating_avg: 4.8,
      rating_count: 73,
      images: [
        createSvgDataUrl('Nordic Armchair', '#be123c', '#9f1239', '🪑')
      ],
      is_featured: 1
    },
    {
      id: 'p_9',
      category_id: 'cat_kitchen',
      title: 'Artisan Ceramic Teapot & Cup Set',
      slug: 'artisan-ceramic-teapot-cup-set',
      description: 'Hand-thrown stoneware ceramic teapot with removable stainless steel infuser and 4 matching cups.',
      price: 48.00,
      sale_price: 38.00,
      stock_quantity: 22,
      rating_avg: 4.6,
      rating_count: 91,
      images: [
        createSvgDataUrl('Ceramic Teapot Set', '#854d0e', '#713f12', '🫖')
      ],
      is_featured: 0
    },
    {
      id: 'p_10',
      category_id: 'cat_tech_books',
      title: 'Building Distributed Systems at Scale',
      slug: 'building-distributed-systems-at-scale',
      description: 'Comprehensive guide covering consensus algorithms, event-sourcing, CQRS, dynamic routing, and fault-tolerant architecture.',
      price: 49.99,
      sale_price: 39.99,
      stock_quantity: 60,
      rating_avg: 4.95,
      rating_count: 312,
      images: [
        createSvgDataUrl('Distributed Systems Book', '#065f46', '#047857', '📚')
      ],
      is_featured: 1
    },
    {
      id: 'p_11',
      category_id: 'cat_tech_books',
      title: 'High Performance Node.js & SQLite Patterns',
      slug: 'high-performance-nodejs-sqlite-patterns',
      description: 'Learn WAL mode optimizations, index strategies, express middleware benchmarking, and asynchronous execution loops.',
      price: 44.95,
      sale_price: null,
      stock_quantity: 40,
      rating_avg: 4.75,
      rating_count: 128,
      images: [
        createSvgDataUrl('Node.js & SQLite Book', '#15803d', '#166534', '📖')
      ],
      is_featured: 0
    },
    {
      id: 'p_12',
      category_id: 'cat_fiction',
      title: 'The Chronicles of Aethelgard (Special Edition)',
      slug: 'the-chronicles-of-aethelgard-special-edition',
      description: 'Hardcover epic fantasy novel featuring gilded edges, custom chapter illustrations, and author annotations.',
      price: 32.00,
      sale_price: 26.50,
      stock_quantity: 18,
      rating_avg: 4.4,
      rating_count: 84,
      images: [
        createSvgDataUrl('Aethelgard Hardcover', '#701a75', '#581c87', '📕')
      ],
      is_featured: 0
    }
  ];

  for (const item of productsData) {
    insertProduct.run(
      item.id,
      item.category_id,
      item.title,
      item.slug,
      item.description,
      item.price,
      item.sale_price,
      item.stock_quantity,
      item.rating_avg,
      item.rating_count,
      JSON.stringify(item.images),
      item.is_featured
    );
  }

  // Insert Coupons
  const insertCoupon = db.prepare(`
    INSERT INTO coupons (id, code, discount_percent, discount_type, discount_amount, max_uses, current_uses, valid_until)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
  `);
  insertCoupon.run('c_wel10', 'WELCOME10', 10, 'percentage', 0, 1000, 42, '2027-12-31 23:59:59');
  insertCoupon.run('c_sum20', 'SUMMER20', 20, 'percentage', 0, 500, 110, '2027-12-31 23:59:59');
  insertCoupon.run('c_har50', 'HARNESS50', 50, 'percentage', 0, 100, 5, '2027-12-31 23:59:59');
  insertCoupon.run('c_flat15', 'FLAT15', 0, 'fixed', 15.00, 200, 10, '2027-12-31 23:59:59');

  console.log(`[Seed] Seeded ${productsData.length} products, categories, and coupons successfully.`);
}

if (require.main === module) {
  seedDatabase();
}

module.exports = {
  seedDatabase
};
