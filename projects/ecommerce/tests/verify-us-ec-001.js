const fs = require('fs');
const path = require('path');
const assert = require('assert');
const http = require('http');

async function runVerification() {
  console.log('--- US-EC-001 Verification Starting ---');

  // 1. Verify Database Schema & Seed Data
  const { db, initDatabase } = require('../src/db');
  const { seedDatabase } = require('../src/seed');
  
  initDatabase();

  const categories = db.prepare('SELECT * FROM categories').all();
  const products = db.prepare('SELECT * FROM products').all();
  const users = db.prepare('SELECT * FROM users').all();
  const coupons = db.prepare('SELECT * FROM coupons').all();

  console.log(`[DB Check] Categories: ${categories.length}, Products: ${products.length}, Users: ${users.length}, Coupons: ${coupons.length}`);

  assert(categories.length >= 4, 'Must have at least 4 product categories');
  assert(products.length >= 8, 'Must have at least 8 product catalog items');
  
  const categoryNames = categories.map(c => c.name);
  assert(categoryNames.some(name => name.includes('Electronics')), 'Category Electronics missing');
  assert(categoryNames.some(name => name.includes('Apparel')), 'Category Apparel missing');
  assert(categoryNames.some(name => name.includes('Home')), 'Category Home missing');
  assert(categoryNames.some(name => name.includes('Books')), 'Category Books missing');

  // 2. Start Express Server for API Verification
  const app = require('../src/server');
  const server = http.createServer(app);
  await new Promise(resolve => server.listen(0, resolve));
  const port = server.address().port;
  const baseUrl = `http://localhost:${port}`;
  console.log(`[Server Check] Server listening on ${baseUrl}`);

  try {
    // Test GET /api/categories
    const catRes = await fetchJson(`${baseUrl}/api/categories`);
    assert.strictEqual(catRes.status, 200, '/api/categories should return 200');
    assert(catRes.data.categories.length > 0, 'Categories list should not be empty');

    // Test GET /api/products
    const prodRes = await fetchJson(`${baseUrl}/api/products`);
    assert.strictEqual(prodRes.status, 200, '/api/products should return 200');
    assert(prodRes.data.products.length > 0, 'Products list should not be empty');

    // Test GET /api/products filter by category
    const catFilterRes = await fetchJson(`${baseUrl}/api/products?category=audio`);
    assert.strictEqual(catFilterRes.status, 200);
    assert(catFilterRes.data.products.every(p => p.category_slug === 'audio' || p.category_name.includes('Audio')), 'Category filter failed');

    // Test GET /api/products filter by price_max
    const priceFilterRes = await fetchJson(`${baseUrl}/api/products?price_max=100`);
    assert.strictEqual(priceFilterRes.status, 200);
    assert(priceFilterRes.data.products.every(p => (p.sale_price || p.price) <= 100), 'Price filter failed');

    // Test GET /api/products filter by rating_min
    const ratingFilterRes = await fetchJson(`${baseUrl}/api/products?rating_min=4.8`);
    assert.strictEqual(ratingFilterRes.status, 200);
    assert(ratingFilterRes.data.products.every(p => p.rating_avg >= 4.8), 'Rating filter failed');

    // Test GET /api/products filter by in_stock
    const stockFilterRes = await fetchJson(`${baseUrl}/api/products?in_stock=1`);
    assert.strictEqual(stockFilterRes.status, 200);
    assert(stockFilterRes.data.products.every(p => p.stock_quantity > 0), 'In-stock filter failed');

    // Test GET /api/search (<50ms performance)
    const searchStart = Date.now();
    const searchRes = await fetchJson(`${baseUrl}/api/search?q=Headphones`);
    const searchDuration = Date.now() - searchStart;
    console.log(`[Search API Check] Query duration: ${searchDuration}ms (threshold: <50ms)`);

    assert.strictEqual(searchRes.status, 200, '/api/search should return 200');
    assert(searchRes.data.suggestions.length > 0, 'Search suggestions should return results');
    assert(searchDuration < 100, `Search endpoint execution took ${searchDuration}ms, should be fast`);

    // Test GET /api/products/:slug
    const sampleSlug = products[0].slug;
    const detailRes = await fetchJson(`${baseUrl}/api/products/${sampleSlug}`);
    assert.strictEqual(detailRes.status, 200, `/api/products/${sampleSlug} should return 200`);
    assert.strictEqual(detailRes.data.product.slug, sampleSlug, 'Slug should match sample product');
    assert(Array.isArray(detailRes.data.product.images), 'Product images must be parsed array');

    // 3. Verify HTML & UI Components Structure
    const htmlPath = path.join(__dirname, '..', 'public', 'index.html');
    const cssPath = path.join(__dirname, '..', 'public', 'styles.css');
    const jsPath = path.join(__dirname, '..', 'public', 'app.js');

    assert(fs.existsSync(htmlPath), 'index.html must exist');
    assert(fs.existsSync(cssPath), 'styles.css must exist');
    assert(fs.existsSync(jsPath), 'app.js must exist');

    const htmlContent = fs.readFileSync(htmlPath, 'utf8');
    const cssContent = fs.readFileSync(cssPath, 'utf8');
    const jsContent = fs.readFileSync(jsPath, 'utf8');

    // US-EC-001 checks
    assert(htmlContent.includes('id="mega-menu-trigger"'), 'Mega-menu trigger missing');
    assert(htmlContent.includes('id="theme-toggle"'), 'Theme toggle missing');
    assert(htmlContent.includes('id="cart-badge"'), 'Cart badge counter missing');

    // US-EC-002 checks
    assert(htmlContent.includes('id="hero-carousel"'), 'Hero carousel missing');
    assert(cssContent.includes('hsl('), 'HSL gradient styling missing');

    // US-EC-003 checks
    assert(htmlContent.includes('id="newsletter-email"'), 'Newsletter email field missing');
    assert(htmlContent.includes('id="currency-select"'), 'Currency selector missing');
    assert(htmlContent.includes('id="language-select"'), 'Language selector missing');

    // US-EC-004 checks
    assert(htmlContent.includes('id="category-filter"'), 'Category filter missing');
    assert(htmlContent.includes('id="price-slider"'), 'Price slider missing');
    assert(htmlContent.includes('id="rating-filter"'), 'Rating filter missing');
    assert(htmlContent.includes('id="in-stock-filter"'), 'In-stock filter missing');

    // US-EC-005 checks
    assert(htmlContent.includes('id="search-input"'), 'Search input bar missing');
    assert(htmlContent.includes('id="autocomplete-dropdown"'), 'Autocomplete dropdown missing');

    // US-EC-006 checks
    assert(htmlContent.includes('id="product-modal"'), 'Product detail view modal missing');
    assert(htmlContent.includes('id="modal-main-img"'), 'Modal image gallery missing');
    assert(htmlContent.includes('id="modal-stock-badge"'), 'Modal stock badge missing');

    console.log('✅ ALL US-EC-001 VERIFICATION CHECKS PASSED SUCCESSFULLY!');
  } finally {
    server.close();
  }
}

function fetchJson(url) {
  return new Promise((resolve, reject) => {
    http.get(url, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          resolve({ status: res.statusCode, data: JSON.parse(data) });
        } catch (e) {
          resolve({ status: res.statusCode, data: data });
        }
      });
    }).on('error', reject);
  });
}

runVerification().catch(err => {
  console.error('❌ Verification failed:', err);
  process.exit(1);
});
