const fs = require('fs');
const path = require('path');
const assert = require('assert');
const http = require('http');

async function runVerification() {
  console.log('--- US-EC-003 / R2 Verification Starting ---');

  // 1. Verify Database Schema & Seed Data
  const { db, initDatabase } = require('../src/db');
  const { seedDatabase } = require('../src/seed');

  // Re-seed to clean test state
  seedDatabase();

  const coupons = db.prepare('SELECT * FROM coupons').all();
  const products = db.prepare('SELECT * FROM products').all();

  console.log(`[DB Check] Coupons: ${coupons.length}, Products: ${products.length}`);
  assert(coupons.length >= 3, 'Must have at least 3 seeded coupons');

  const welcomeCoupon = coupons.find(c => c.code === 'WELCOME10');
  assert(welcomeCoupon, 'WELCOME10 coupon missing from database');
  assert.strictEqual(welcomeCoupon.discount_percent, 10, 'WELCOME10 should have 10% discount');

  const flatCoupon = coupons.find(c => c.code === 'FLAT15');
  assert(flatCoupon, 'FLAT15 coupon missing from database');

  // 2. Start Express Server for API Verification
  const app = require('../src/server');
  const server = http.createServer(app);
  await new Promise(resolve => server.listen(0, resolve));
  const port = server.address().port;
  const baseUrl = `http://localhost:${port}`;
  console.log(`[Server Check] Server listening on ${baseUrl}`);

  try {
    // --- Test 1: POST /api/cart/coupon (Percentage Discount) ---
    const coupRes1 = await postJson(`${baseUrl}/api/cart/coupon`, {
      code: 'WELCOME10',
      subtotal: 200.00
    });
    assert.strictEqual(coupRes1.status, 200, 'Percentage coupon API should return 200');
    assert.strictEqual(coupRes1.data.valid, true, 'Coupon should be valid');
    assert.strictEqual(coupRes1.data.code, 'WELCOME10', 'Coupon code should match');
    assert.strictEqual(coupRes1.data.discount, 20.00, '10% of 200.00 should be 20.00 discount');

    // --- Test 2: POST /api/cart/coupon (Fixed Discount) ---
    const coupRes2 = await postJson(`${baseUrl}/api/cart/coupon`, {
      code: 'FLAT15',
      subtotal: 100.00
    });
    assert.strictEqual(coupRes2.status, 200, 'Fixed discount coupon API should return 200');
    assert.strictEqual(coupRes2.data.valid, true);
    assert.strictEqual(coupRes2.data.discount, 15.00, 'FLAT15 discount should be 15.00');

    // --- Test 3: POST /api/cart/coupon (Invalid Code) ---
    const invalidRes = await postJson(`${baseUrl}/api/cart/coupon`, {
      code: 'INVALID_CODE_999',
      subtotal: 100.00
    });
    assert.strictEqual(invalidRes.status, 404, 'Invalid coupon should return 404 status');

    // --- Test 4: POST /api/orders (Order Creation & Transaction Verification) ---
    const sampleProd = products[0];
    const initialStock = sampleProd.stock_quantity;
    const initialCouponUses = welcomeCoupon.current_uses;

    const orderPayload = {
      user_id: 'u_customer1',
      items: [
        { product_id: sampleProd.id, quantity: 2 }
      ],
      shipping_address: {
        fullname: 'Jane Doe',
        address: '100 Tech Blvd, Suite 200',
        city: 'San Jose',
        zip: '95110',
        country: 'United States'
      },
      payment_method: 'credit_card',
      coupon_code: 'WELCOME10'
    };

    const orderRes = await postJson(`${baseUrl}/api/orders`, orderPayload);
    assert.strictEqual(orderRes.status, 201, 'Order creation API should return 201 Created');
    assert.strictEqual(orderRes.data.success, true, 'Order creation response success should be true');
    assert(orderRes.data.order_id && orderRes.data.order_id.startsWith('ord_'), 'order_id should be valid string');

    const createdOrderId = orderRes.data.order_id;

    // --- Test 5: DB Record Inspection ---
    const dbOrder = db.prepare('SELECT * FROM orders WHERE id = ?').get(createdOrderId);
    assert(dbOrder, 'Order record must exist in orders table');
    assert.strictEqual(dbOrder.status, 'pending', 'Order initial status must be pending');
    assert.strictEqual(dbOrder.coupon_code, 'WELCOME10', 'Coupon code in order must match WELCOME10');

    const dbOrderItems = db.prepare('SELECT * FROM order_items WHERE order_id = ?').all(createdOrderId);
    assert.strictEqual(dbOrderItems.length, 1, 'Should insert 1 item into order_items');
    assert.strictEqual(dbOrderItems[0].product_id, sampleProd.id);
    assert.strictEqual(dbOrderItems[0].quantity, 2);

    // Verify stock decrement
    const updatedProd = db.prepare('SELECT stock_quantity FROM products WHERE id = ?').get(sampleProd.id);
    assert.strictEqual(updatedProd.stock_quantity, initialStock - 2, 'Product stock quantity must be decremented by 2');

    // Verify coupon usage increment
    const updatedCoupon = db.prepare('SELECT current_uses FROM coupons WHERE code = ?').get('WELCOME10');
    assert.strictEqual(updatedCoupon.current_uses, initialCouponUses + 1, 'Coupon current_uses must be incremented by 1');

    // --- Test 6: POST /api/orders Validation Errors ---
    const emptyItemsRes = await postJson(`${baseUrl}/api/orders`, {
      user_id: 'u_customer1',
      items: [],
      shipping_address: orderPayload.shipping_address,
      payment_method: 'credit_card'
    });
    assert.strictEqual(emptyItemsRes.status, 400, 'Empty items order should return 400');

    const missingAddrRes = await postJson(`${baseUrl}/api/orders`, {
      user_id: 'u_customer1',
      items: [{ product_id: sampleProd.id, quantity: 1 }],
      shipping_address: { fullname: 'Incomplete' },
      payment_method: 'credit_card'
    });
    assert.strictEqual(missingAddrRes.status, 400, 'Incomplete address order should return 400');

    // --- Test 7: Verify HTML & UI Components Structure ---
    const htmlPath = path.join(__dirname, '..', 'public', 'index.html');
    const cssPath = path.join(__dirname, '..', 'public', 'styles.css');
    const jsPath = path.join(__dirname, '..', 'public', 'app.js');

    assert(fs.existsSync(htmlPath), 'index.html must exist');
    assert(fs.existsSync(cssPath), 'styles.css must exist');
    assert(fs.existsSync(jsPath), 'app.js must exist');

    const htmlContent = fs.readFileSync(htmlPath, 'utf8');
    const cssContent = fs.readFileSync(cssPath, 'utf8');
    const jsContent = fs.readFileSync(jsPath, 'utf8');

    // US-EC-007 Cart Drawer Checks
    assert(htmlContent.includes('id="cart-drawer"') || htmlContent.includes('id="cart-drawer-overlay"'), 'Cart drawer overlay missing in index.html');
    assert(htmlContent.includes('id="cart-drawer-close"'), 'Cart drawer close button missing in index.html');
    assert(cssContent.includes('.cart-drawer'), 'Cart drawer styling missing in styles.css');
    assert(jsContent.includes('openCartDrawer'), 'Cart drawer open logic missing in app.js');

    // US-EC-008 Promo Coupon System Checks
    assert(htmlContent.includes('id="coupon-code-input"'), 'Coupon code input field missing in index.html');
    assert(htmlContent.includes('id="apply-coupon-btn"'), 'Apply coupon button missing in index.html');
    assert(jsContent.includes('applyCoupon'), 'Coupon application handler missing in app.js');

    // US-EC-009 3-Step Checkout Wizard Checks
    assert(htmlContent.includes('id="checkout-modal"'), 'Checkout modal missing in index.html');
    assert(htmlContent.includes('id="shipping-fullname"'), 'Shipping full name field missing');
    assert(htmlContent.includes('id="shipping-address"'), 'Shipping address field missing');
    assert(htmlContent.includes('id="shipping-city"'), 'Shipping city field missing');
    assert(htmlContent.includes('id="shipping-zip"'), 'Shipping zip field missing');
    assert(htmlContent.includes('id="shipping-country"'), 'Shipping country field missing');
    assert(htmlContent.includes('id="place-order-btn"'), 'Place order button missing in index.html');
    assert(jsContent.includes('placeOrder'), 'Order creation handler missing in app.js');

    console.log('✅ ALL US-EC-003 / R2 VERIFICATION CHECKS PASSED SUCCESSFULLY!');
  } finally {
    server.close();
  }
}

function postJson(url, data) {
  return new Promise((resolve, reject) => {
    const u = new URL(url);
    const postData = JSON.stringify(data);
    const req = http.request({
      hostname: u.hostname,
      port: u.port,
      path: u.pathname,
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(postData)
      }
    }, (res) => {
      let body = '';
      res.on('data', chunk => body += chunk);
      res.on('end', () => {
        try {
          resolve({ status: res.statusCode, data: JSON.parse(body) });
        } catch (e) {
          resolve({ status: res.statusCode, data: body });
        }
      });
    });
    req.on('error', reject);
    req.write(postData);
    req.end();
  });
}

runVerification().catch(err => {
  console.error('❌ Verification failed:', err);
  process.exit(1);
});
