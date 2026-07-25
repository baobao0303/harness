const fs = require('fs');
const path = require('path');
const assert = require('assert');
const http = require('http');

async function runVerification() {
  console.log('--- US-EC-005 / R3 Verification Starting ---');

  // 1. Verify Database Schema & Seed Data
  const { db, initDatabase } = require('../src/db');
  const { seedDatabase } = require('../src/seed');

  // Re-seed database to ensure clean test state
  seedDatabase();

  const users = db.prepare('SELECT * FROM users').all();
  const products = db.prepare('SELECT * FROM products').all();
  console.log(`[DB Check] Users: ${users.length}, Products: ${products.length}`);

  // 2. Start Express Server for API Verification
  const app = require('../src/server');
  const server = http.createServer(app);
  await new Promise(resolve => server.listen(0, resolve));
  const port = server.address().port;
  const baseUrl = `http://localhost:${port}`;
  console.log(`[Server Check] Server listening on ${baseUrl}`);

  try {
    // --- US-EC-010: Customer Authentication API Tests ---
    const testEmail = `testuser_${Date.now()}@example.com`;
    const testPassword = 'Password123!';

    // 1. POST /api/auth/register
    console.log('[Auth Test] Testing POST /api/auth/register...');
    const regRes = await postJson(`${baseUrl}/api/auth/register`, {
      email: testEmail,
      password: testPassword
    });

    assert.strictEqual(regRes.status, 201, 'Register API should return 201 Created');
    assert.strictEqual(regRes.data.success, true, 'Register response success should be true');
    assert(regRes.data.token, 'Register response must include JWT token');
    assert(regRes.data.user && regRes.data.user.id, 'Register response must include user object with id');
    assert.strictEqual(regRes.data.user.email, testEmail, 'User email in response must match registered email');

    const authToken = regRes.data.token;
    const authUserId = regRes.data.user.id;

    // Test duplicate registration attempt
    const dupRegRes = await postJson(`${baseUrl}/api/auth/register`, {
      email: testEmail,
      password: testPassword
    });
    assert.strictEqual(dupRegRes.status, 400, 'Duplicate registration should return 400 Bad Request');

    // 2. POST /api/auth/login
    console.log('[Auth Test] Testing POST /api/auth/login...');
    const loginRes = await postJson(`${baseUrl}/api/auth/login`, {
      email: testEmail,
      password: testPassword
    });

    assert.strictEqual(loginRes.status, 200, 'Login API should return 200 OK');
    assert.strictEqual(loginRes.data.success, true, 'Login success should be true');
    assert(loginRes.data.token, 'Login response must include JWT token');
    assert.strictEqual(loginRes.data.user.email, testEmail, 'Logged in user email should match');

    // Test invalid credentials
    const invalidLoginRes = await postJson(`${baseUrl}/api/auth/login`, {
      email: testEmail,
      password: 'WrongPassword999'
    });
    assert.strictEqual(invalidLoginRes.status, 401, 'Invalid login password should return 401 Unauthorized');

    // 3. GET /api/auth/me
    console.log('[Auth Test] Testing GET /api/auth/me...');
    const meRes = await fetchWithAuth(`${baseUrl}/api/auth/me`, authToken);
    assert.strictEqual(meRes.status, 200, 'GET /api/auth/me with Bearer token should return 200 OK');
    assert.strictEqual(meRes.data.user.email, testEmail, 'GET /api/auth/me user email should match');

    const unauthMeRes = await fetchWithAuth(`${baseUrl}/api/auth/me`, null);
    assert.strictEqual(unauthMeRes.status, 401, 'GET /api/auth/me without token should return 401 Unauthorized');


    // --- US-EC-011: Order Engine State Machine Tests ---
    console.log('[Order State Machine Test] Creating test order...');
    const sampleProduct = products[0];

    const orderRes = await postJsonWithAuth(`${baseUrl}/api/orders`, {
      items: [{ product_id: sampleProduct.id, quantity: 1 }],
      shipping_address: {
        fullname: 'Test Customer',
        address: '456 Innovation Way',
        city: 'Seattle',
        zip: '98101',
        country: 'United States'
      },
      payment_method: 'credit_card'
    }, authToken);

    assert.strictEqual(orderRes.status, 201, 'Order creation should return 201 Created');
    const orderId = orderRes.data.order_id;
    assert(orderId, 'Order ID must exist');

    // Check initial status = pending
    let dbOrder = db.prepare('SELECT status FROM orders WHERE id = ?').get(orderId);
    assert.strictEqual(dbOrder.status, 'pending', 'Initial order status must be pending');

    // Test valid transition: pending -> processing
    console.log('[Order State Machine Test] Testing pending -> processing...');
    const trans1 = await patchJson(`${baseUrl}/api/orders/${orderId}/status`, { status: 'processing' });
    assert.strictEqual(trans1.status, 200, 'Transition pending -> processing should return 200');
    assert.strictEqual(trans1.data.order.status, 'processing');

    // Test valid transition: processing -> shipped
    console.log('[Order State Machine Test] Testing processing -> shipped...');
    const trans2 = await patchJson(`${baseUrl}/api/orders/${orderId}/status`, { status: 'shipped' });
    assert.strictEqual(trans2.status, 200, 'Transition processing -> shipped should return 200');
    assert.strictEqual(trans2.data.order.status, 'shipped');

    // Test valid transition: shipped -> delivered
    console.log('[Order State Machine Test] Testing shipped -> delivered...');
    const trans3 = await patchJson(`${baseUrl}/api/orders/${orderId}/status`, { status: 'delivered' });
    assert.strictEqual(trans3.status, 200, 'Transition shipped -> delivered should return 200');
    assert.strictEqual(trans3.data.order.status, 'delivered');

    // Test ILLEGAL transition: delivered -> cancelled (should return 400 Bad Request)
    console.log('[Order State Machine Test] Testing illegal transition: delivered -> cancelled (should fail 400)...');
    const illegalTrans1 = await patchJson(`${baseUrl}/api/orders/${orderId}/status`, { status: 'cancelled' });
    assert.strictEqual(illegalTrans1.status, 400, 'Illegal transition delivered -> cancelled must return 400 Bad Request');
    assert(illegalTrans1.data.error, 'Error message must be returned for illegal transition');

    // Create a second order to test cancellation & illegal transition from cancelled
    const orderRes2 = await postJsonWithAuth(`${baseUrl}/api/orders`, {
      items: [{ product_id: sampleProduct.id, quantity: 1 }],
      shipping_address: {
        fullname: 'Test Customer 2',
        address: '789 Tech Ave',
        city: 'Austin',
        zip: '73301',
        country: 'United States'
      },
      payment_method: 'paypal'
    }, authToken);

    const orderId2 = orderRes2.data.order_id;
    
    // Test valid transition: pending -> cancelled
    const cancelRes = await patchJson(`${baseUrl}/api/orders/${orderId2}/status`, { status: 'cancelled' });
    assert.strictEqual(cancelRes.status, 200, 'Transition pending -> cancelled should return 200');
    assert.strictEqual(cancelRes.data.order.status, 'cancelled');

    // Test ILLEGAL transition: cancelled -> shipped (should return 400 Bad Request)
    console.log('[Order State Machine Test] Testing illegal transition: cancelled -> shipped (should fail 400)...');
    const illegalTrans2 = await patchJson(`${baseUrl}/api/orders/${orderId2}/status`, { status: 'shipped' });
    assert.strictEqual(illegalTrans2.status, 400, 'Illegal transition cancelled -> shipped must return 400 Bad Request');


    // --- US-EC-012: Customer Dashboard & Invoice Export Tests ---
    console.log('[Dashboard Test] Testing GET /api/orders/my-orders...');
    const myOrdersRes = await fetchWithAuth(`${baseUrl}/api/orders/my-orders`, authToken);
    assert.strictEqual(myOrdersRes.status, 200, 'GET /api/orders/my-orders with token should return 200 OK');
    assert(Array.isArray(myOrdersRes.data.orders), 'Response must contain orders array');
    assert(myOrdersRes.data.orders.length >= 2, 'User order history should contain placed test orders');

    const firstOrder = myOrdersRes.data.orders.find(o => o.id === orderId);
    assert(firstOrder, 'Placed order must exist in user orders list');
    assert(Array.isArray(firstOrder.items), 'Order history must include item breakdown array');
    assert(firstOrder.items.length > 0, 'Item breakdown must contain ordered items');
    assert(firstOrder.timeline, 'Order history must include tracking timeline object');
    assert.strictEqual(firstOrder.timeline.status, 'delivered', 'Timeline status should match');

    // Test unauthenticated GET /api/orders/my-orders
    const unauthOrdersRes = await fetchWithAuth(`${baseUrl}/api/orders/my-orders`, null);
    assert.strictEqual(unauthOrdersRes.status, 401, 'Unauthenticated /api/orders/my-orders should return 401');


    // --- Verify HTML & UI Components Structure ---
    console.log('[UI Check] Verifying frontend files...');
    const htmlPath = path.join(__dirname, '..', 'public', 'index.html');
    const cssPath = path.join(__dirname, '..', 'public', 'styles.css');
    const jsPath = path.join(__dirname, '..', 'public', 'app.js');

    assert(fs.existsSync(htmlPath), 'index.html must exist');
    assert(fs.existsSync(cssPath), 'styles.css must exist');
    assert(fs.existsSync(jsPath), 'app.js must exist');

    const htmlContent = fs.readFileSync(htmlPath, 'utf8');
    const cssContent = fs.readFileSync(cssPath, 'utf8');
    const jsContent = fs.readFileSync(jsPath, 'utf8');

    // US-EC-010 Auth UI Checks
    assert(htmlContent.includes('id="auth-modal"'), 'Auth modal missing in index.html');
    assert(htmlContent.includes('id="auth-email"'), 'Auth email input missing in index.html');
    assert(htmlContent.includes('id="auth-password"'), 'Auth password input missing in index.html');
    assert(htmlContent.includes('id="login-tab"'), 'Login tab missing in index.html');
    assert(htmlContent.includes('id="register-tab"'), 'Register tab missing in index.html');
    assert(jsContent.includes('handleAuthSubmit'), 'Auth handler missing in app.js');

    // US-EC-012 Customer Dashboard & Invoice UI Checks
    assert(htmlContent.includes('id="orders-modal"'), 'Orders modal missing in index.html');
    assert(htmlContent.includes('id="orders-list-container"'), 'Orders list container missing in index.html');
    assert(htmlContent.includes('id="invoice-modal"'), 'Invoice modal missing in index.html');
    assert(cssContent.includes('.tracking-timeline'), 'Tracking timeline styling missing in styles.css');
    assert(cssContent.includes('.tracking-steps'), 'Tracking steps styling missing in styles.css');
    assert(jsContent.includes('fetchMyOrders'), 'Fetch my orders handler missing in app.js');
    assert(jsContent.includes('exportInvoice'), 'Export invoice handler missing in app.js');

    console.log('✅ ALL US-EC-005 / R3 VERIFICATION CHECKS PASSED SUCCESSFULLY!');
  } finally {
    server.close();
  }
}

function fetchWithAuth(url, token) {
  return new Promise((resolve, reject) => {
    const u = new URL(url);
    const headers = {};
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
    const req = http.request({
      hostname: u.hostname,
      port: u.port,
      path: u.pathname + u.search,
      method: 'GET',
      headers
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
    req.end();
  });
}

function postJson(url, data) {
  return postJsonWithAuth(url, data, null);
}

function postJsonWithAuth(url, data, token) {
  return new Promise((resolve, reject) => {
    const u = new URL(url);
    const postData = JSON.stringify(data);
    const headers = {
      'Content-Type': 'application/json',
      'Content-Length': Buffer.byteLength(postData)
    };
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
    const req = http.request({
      hostname: u.hostname,
      port: u.port,
      path: u.pathname,
      method: 'POST',
      headers
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

function patchJson(url, data) {
  return new Promise((resolve, reject) => {
    const u = new URL(url);
    const postData = JSON.stringify(data);
    const req = http.request({
      hostname: u.hostname,
      port: u.port,
      path: u.pathname,
      method: 'PATCH',
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
