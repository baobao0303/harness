const express = require('express');
const cors = require('cors');
const path = require('path');
const { db, initDatabase } = require('./db');
const { seedDatabase } = require('./seed');

const app = express();
const PORT = process.env.PORT || 3000;

// Initialize database schema and auto-seed if empty
initDatabase();
const productCount = db.prepare('SELECT COUNT(*) as count FROM products').get().count;
if (productCount === 0) {
  seedDatabase();
}

app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname, '..', 'public')));

// Categories Endpoint (For Navbar Mega-Menu & Filters)
app.get('/api/categories', (req, res) => {
  try {
    const categories = db.prepare(`
      SELECT c1.id, c1.name, c1.slug, c1.parent_id, c2.name as parent_name
      FROM categories c1
      LEFT JOIN categories c2 ON c1.parent_id = c2.id
      ORDER BY c1.parent_id IS NULL DESC, c1.name ASC
    `).all();

    // Group into top-level categories with subcategories
    const topLevel = categories.filter(c => !c.parent_id).map(c => ({
      ...c,
      subcategories: categories.filter(sub => sub.parent_id === c.id)
    }));

    res.json({ categories: topLevel, flat: categories });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// GET /api/products — Catalog list with multi-attribute filtering & sorting
app.get('/api/products', (req, res) => {
  try {
    const { category, price_min, price_max, rating_min, in_stock, is_featured, sort } = req.query;

    let whereClause = ['1=1'];
    let params = [];

    if (category) {
      // Find category and child categories
      const catRow = db.prepare('SELECT id FROM categories WHERE slug = ? OR id = ?').get(category, category);
      if (catRow) {
        const childCats = db.prepare('SELECT id FROM categories WHERE parent_id = ?').all(catRow.id);
        const catIds = [catRow.id, ...childCats.map(c => c.id)];
        const placeholders = catIds.map(() => '?').join(',');
        whereClause.push(`p.category_id IN (${placeholders})`);
        params.push(...catIds);
      } else {
        whereClause.push(`(c.slug = ? OR c.id = ?)`);
        params.push(category, category);
      }
    }

    if (price_min) {
      whereClause.push('COALESCE(p.sale_price, p.price) >= ?');
      params.push(parseFloat(price_min));
    }

    if (price_max) {
      whereClause.push('COALESCE(p.sale_price, p.price) <= ?');
      params.push(parseFloat(price_max));
    }

    if (rating_min) {
      whereClause.push('p.rating_avg >= ?');
      params.push(parseFloat(rating_min));
    }

    if (in_stock === 'true' || in_stock === '1') {
      whereClause.push('p.stock_quantity > 0');
    }

    if (is_featured === 'true' || is_featured === '1') {
      whereClause.push('p.is_featured = 1');
    }

    let orderBy = 'p.created_at DESC';
    if (sort === 'price_asc') {
      orderBy = 'COALESCE(p.sale_price, p.price) ASC';
    } else if (sort === 'price_desc') {
      orderBy = 'COALESCE(p.sale_price, p.price) DESC';
    } else if (sort === 'rating') {
      orderBy = 'p.rating_avg DESC';
    } else if (sort === 'newest') {
      orderBy = 'p.created_at DESC';
    }

    const query = `
      SELECT p.*, c.name as category_name, c.slug as category_slug
      FROM products p
      JOIN categories c ON p.category_id = c.id
      WHERE ${whereClause.join(' AND ')}
      ORDER BY ${orderBy}
    `;

    const rawProducts = db.prepare(query).all(...params);

    const products = rawProducts.map(p => ({
      ...p,
      images: JSON.parse(p.images_json || '[]'),
      is_in_stock: p.stock_quantity > 0
    }));

    res.json({ products, count: products.length });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// GET /api/search — Instant autocomplete search (<50ms response requirement)
app.get('/api/search', (req, res) => {
  const startTime = Date.now();
  try {
    const q = (req.query.q || '').trim();
    if (!q) {
      return res.json({ suggestions: [], query: '', duration_ms: Date.now() - startTime });
    }

    const searchTerm = `%${q}%`;
    const query = `
      SELECT p.id, p.title, p.slug, p.price, p.sale_price, p.stock_quantity,
             p.rating_avg, p.images_json, c.name as category_name
      FROM products p
      JOIN categories c ON p.category_id = c.id
      WHERE p.title LIKE ? OR p.description LIKE ? OR c.name LIKE ?
      ORDER BY 
        CASE WHEN p.title LIKE ? THEN 1 ELSE 2 END,
        p.rating_avg DESC
      LIMIT 8
    `;

    const exactMatchTerm = `${q}%`;
    const rows = db.prepare(query).all(searchTerm, searchTerm, searchTerm, exactMatchTerm);

    const suggestions = rows.map(r => {
      const images = JSON.parse(r.images_json || '[]');
      return {
        id: r.id,
        title: r.title,
        slug: r.slug,
        price: r.price,
        sale_price: r.sale_price,
        effective_price: r.sale_price || r.price,
        category_name: r.category_name,
        stock_quantity: r.stock_quantity,
        is_in_stock: r.stock_quantity > 0,
        rating_avg: r.rating_avg,
        image: images[0] || null
      };
    });

    const duration_ms = Date.now() - startTime;
    res.json({ suggestions, query: q, duration_ms });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// GET /api/products/:slug — Detailed product view
app.get('/api/products/:slug', (req, res) => {
  try {
    const { slug } = req.params;
    const query = `
      SELECT p.*, c.name as category_name, c.slug as category_slug
      FROM products p
      JOIN categories c ON p.category_id = c.id
      WHERE p.slug = ?
    `;

    const product = db.prepare(query).get(slug);

    if (!product) {
      return res.status(404).json({ error: 'Product not found' });
    }

    const formattedProduct = {
      ...product,
      images: JSON.parse(product.images_json || '[]'),
      is_in_stock: product.stock_quantity > 0
    };

    res.json({ product: formattedProduct });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// POST /api/cart/coupon — Validate & calculate promo coupon discount (US-EC-008)
app.post('/api/cart/coupon', (req, res) => {
  try {
    const { code, subtotal = 0 } = req.body;
    if (!code || !code.trim()) {
      return res.status(400).json({ error: 'Coupon code is required' });
    }

    const coupon = db.prepare('SELECT * FROM coupons WHERE UPPER(code) = UPPER(?)').get(code.trim());

    if (!coupon) {
      return res.status(404).json({ error: 'Invalid coupon code' });
    }

    if (coupon.current_uses >= coupon.max_uses) {
      return res.status(400).json({ error: 'Coupon usage limit reached' });
    }

    const now = new Date();
    const validUntil = new Date(coupon.valid_until);
    if (now > validUntil) {
      return res.status(400).json({ error: 'Coupon code has expired' });
    }

    let discountAmount = 0;
    const type = coupon.discount_type || (coupon.discount_percent > 0 ? 'percentage' : 'fixed');

    if (type === 'percentage') {
      discountAmount = (parseFloat(subtotal) * coupon.discount_percent) / 100;
    } else {
      discountAmount = coupon.discount_amount || coupon.discount_percent || 0;
    }

    if (parseFloat(subtotal) > 0 && discountAmount > parseFloat(subtotal)) {
      discountAmount = parseFloat(subtotal);
    }

    res.json({
      valid: true,
      code: coupon.code,
      discount_percent: coupon.discount_percent,
      discount_type: type,
      discount_amount: coupon.discount_amount || 0,
      discount: Math.round(discountAmount * 100) / 100
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// POST /api/orders — Insert order & order_items, decrement stock, clear cart (US-EC-009)
app.post('/api/orders', (req, res) => {
  try {
    const { user_id, items, shipping_address, payment_method, coupon_code } = req.body;

    if (!items || !Array.isArray(items) || items.length === 0) {
      return res.status(400).json({ error: 'Order must contain at least one item' });
    }

    if (!shipping_address || !shipping_address.fullname || !shipping_address.address || !shipping_address.city || !shipping_address.zip || !shipping_address.country) {
      return res.status(400).json({ error: 'Complete shipping address is required (Full Name, Address, City, Zip, Country)' });
    }

    if (!payment_method) {
      return res.status(400).json({ error: 'Payment method is required' });
    }

    const userId = user_id || 'u_customer1';
    const addressJson = JSON.stringify(shipping_address);

    let subtotal = 0;
    const itemDetails = [];

    for (const item of items) {
      const product = db.prepare('SELECT * FROM products WHERE id = ?').get(item.product_id);
      if (!product) {
        return res.status(400).json({ error: `Product not found: ${item.product_id}` });
      }
      if (product.stock_quantity < item.quantity) {
        return res.status(400).json({ error: `Insufficient stock for product: ${product.title}` });
      }
      const unitPrice = product.sale_price || product.price;
      subtotal += unitPrice * item.quantity;
      itemDetails.push({
        product_id: product.id,
        quantity: item.quantity,
        unit_price: unitPrice,
        title: product.title
      });
    }

    let discountAmount = 0;
    let validCouponCode = null;

    if (coupon_code) {
      const coupon = db.prepare('SELECT * FROM coupons WHERE UPPER(code) = UPPER(?)').get(coupon_code.trim());
      if (coupon && coupon.current_uses < coupon.max_uses && new Date() <= new Date(coupon.valid_until)) {
        validCouponCode = coupon.code;
        const type = coupon.discount_type || (coupon.discount_percent > 0 ? 'percentage' : 'fixed');
        if (type === 'percentage') {
          discountAmount = (subtotal * coupon.discount_percent) / 100;
        } else {
          discountAmount = coupon.discount_amount || coupon.discount_percent || 0;
        }
        if (discountAmount > subtotal) discountAmount = subtotal;
      }
    }

    const tax = Math.round((subtotal - discountAmount) * 0.08 * 100) / 100;
    const shipping = subtotal >= 100 ? 0 : 10;
    const totalAmount = Math.round((subtotal - discountAmount + tax + shipping) * 100) / 100;

    const orderId = 'ord_' + Date.now() + '_' + Math.random().toString(36).substring(2, 7);

    const createOrderTx = db.transaction(() => {
      db.prepare(`
        INSERT INTO orders (id, user_id, status, total_amount, coupon_code, shipping_address_json, payment_method)
        VALUES (?, ?, 'pending', ?, ?, ?, ?)
      `).run(orderId, userId, totalAmount, validCouponCode, addressJson, payment_method);

      const insertItem = db.prepare(`
        INSERT INTO order_items (id, order_id, product_id, quantity, unit_price)
        VALUES (?, ?, ?, ?, ?)
      `);

      const updateStock = db.prepare(`
        UPDATE products SET stock_quantity = stock_quantity - ? WHERE id = ?
      `);

      for (const item of itemDetails) {
        const itemId = 'item_' + Date.now() + '_' + Math.random().toString(36).substring(2, 7);
        insertItem.run(itemId, orderId, item.product_id, item.quantity, item.unit_price);
        updateStock.run(item.quantity, item.product_id);
      }

      if (validCouponCode) {
        db.prepare('UPDATE coupons SET current_uses = current_uses + 1 WHERE code = ?').run(validCouponCode);
      }
    });

    createOrderTx();

    const createdOrder = db.prepare('SELECT * FROM orders WHERE id = ?').get(orderId);

    res.status(201).json({
      success: true,
      message: 'Order created successfully',
      order_id: orderId,
      order: {
        ...createdOrder,
        shipping_address: JSON.parse(createdOrder.shipping_address_json),
        items: itemDetails
      }
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

if (require.main === module) {
  app.listen(PORT, () => {
    console.log(`[Storefront Server] Running on http://localhost:${PORT}`);
  });
}

module.exports = app;
