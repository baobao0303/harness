const app = {
  state: {
    products: [],
    categories: [],
    cart: JSON.parse(localStorage.getItem('ec_cart') || '[]'),
    appliedCoupon: null,
    selectedPaymentMethod: 'credit_card',
    currency: localStorage.getItem('ec_currency') || 'USD',
    rates: { USD: 1.0, EUR: 0.92, GBP: 0.78 },
    currencySymbols: { USD: '$', EUR: '€', GBP: '£' },
    filters: {
      category: '',
      price_max: 1500,
      rating_min: 0,
      in_stock: false,
      sort: 'newest'
    },
    currentSlide: 0,
    slideInterval: null,
    searchDebounceTimer: null,
    currentModalProduct: null,
    token: localStorage.getItem('ec_token') || null,
    user: JSON.parse(localStorage.getItem('ec_user') || 'null'),
    authMode: 'login',
    myOrders: []
  },

  init() {
    this.initTheme();
    this.updateCartBadge();
    this.initCarousel();
    this.bindEvents();
    this.fetchCategories();
    this.fetchProducts();
    this.renderCartDrawer();
    this.initAuth();
  },

  // US-EC-001: Light/Dark Theme Switcher
  initTheme() {
    const savedTheme = localStorage.getItem('ec_theme') || 'light';
    document.documentElement.setAttribute('data-theme', savedTheme);
    this.updateThemeIcon(savedTheme);
  },

  toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
    const newTheme = currentTheme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('ec_theme', newTheme);
    this.updateThemeIcon(newTheme);
  },

  updateThemeIcon(theme) {
    const iconEl = document.getElementById('theme-icon');
    if (iconEl) {
      iconEl.textContent = theme === 'dark' ? '☀️' : '🌙';
    }
  },

  // US-EC-001: Cart Badge Counter
  updateCartBadge() {
    const totalQty = this.state.cart.reduce((sum, item) => sum + item.quantity, 0);
    const badgeEl = document.getElementById('cart-badge');
    if (badgeEl) {
      badgeEl.textContent = totalQty;
    }
  },

  addToCart(product, quantity = 1) {
    const existing = this.state.cart.find(item => item.id === product.id);
    const stockLimit = product.stock_quantity !== undefined ? product.stock_quantity : 99;

    if (existing) {
      if (existing.quantity + quantity > stockLimit) {
        existing.quantity = stockLimit;
      } else {
        existing.quantity += quantity;
      }
    } else {
      const initialQty = Math.min(quantity, stockLimit);
      this.state.cart.push({
        id: product.id,
        title: product.title,
        price: product.sale_price || product.price,
        image: product.images ? product.images[0] : null,
        quantity: initialQty,
        stock_quantity: stockLimit
      });
    }
    localStorage.setItem('ec_cart', JSON.stringify(this.state.cart));
    this.updateCartBadge();
    this.renderCartDrawer();
    this.openCartDrawer();
  },

  // US-EC-002: Hero Carousel
  initCarousel() {
    const slides = document.querySelectorAll('.carousel-slide');
    const dots = document.querySelectorAll('.dot');
    if (!slides.length) return;

    this.showSlide(0);

    const prevBtn = document.getElementById('carousel-prev');
    const nextBtn = document.getElementById('carousel-next');

    if (prevBtn) prevBtn.addEventListener('click', () => this.changeSlide(-1));
    if (nextBtn) nextBtn.addEventListener('click', () => this.changeSlide(1));

    dots.forEach((dot, idx) => {
      dot.addEventListener('click', () => this.showSlide(idx));
    });

    // Auto-advance every 5 seconds
    this.state.slideInterval = setInterval(() => this.changeSlide(1), 5000);
  },

  showSlide(index) {
    const slides = document.querySelectorAll('.carousel-slide');
    const dots = document.querySelectorAll('.dot');
    if (!slides.length) return;

    slides.forEach(s => s.classList.remove('active'));
    dots.forEach(d => d.classList.remove('active'));

    this.state.currentSlide = (index + slides.length) % slides.length;
    slides[this.state.currentSlide].classList.add('active');
    if (dots[this.state.currentSlide]) {
      dots[this.state.currentSlide].classList.add('active');
    }
  },

  changeSlide(delta) {
    this.showSlide(this.state.currentSlide + delta);
  },

  // Bind UI Events
  bindEvents() {
    // Theme toggle
    const themeBtn = document.getElementById('theme-toggle');
    if (themeBtn) {
      themeBtn.addEventListener('click', () => this.toggleTheme());
    }

    // Category Filter
    const catSelect = document.getElementById('category-filter');
    if (catSelect) {
      catSelect.addEventListener('change', (e) => {
        this.state.filters.category = e.target.value;
        this.fetchProducts();
      });
    }

    // Price Slider Filter
    const priceSlider = document.getElementById('price-slider');
    const priceValue = document.getElementById('price-slider-value');
    if (priceSlider) {
      priceSlider.addEventListener('input', (e) => {
        const val = e.target.value;
        if (priceValue) priceValue.textContent = this.formatPrice(val);
        this.state.filters.price_max = parseFloat(val);
        this.fetchProducts();
      });
    }

    // Rating Filter
    const ratingSelect = document.getElementById('rating-filter');
    if (ratingSelect) {
      ratingSelect.addEventListener('change', (e) => {
        this.state.filters.rating_min = parseFloat(e.target.value);
        this.fetchProducts();
      });
    }

    // In-Stock Filter
    const stockCheckbox = document.getElementById('in-stock-filter');
    if (stockCheckbox) {
      stockCheckbox.addEventListener('change', (e) => {
        this.state.filters.in_stock = e.target.checked;
        this.fetchProducts();
      });
    }

    // Sort Selection
    const sortSelect = document.getElementById('sort-select');
    if (sortSelect) {
      sortSelect.addEventListener('change', (e) => {
        this.state.filters.sort = e.target.value;
        this.fetchProducts();
      });
    }

    // Reset Filters
    const resetBtn = document.getElementById('reset-filters');
    if (resetBtn) {
      resetBtn.addEventListener('click', () => {
        this.state.filters = {
          category: '',
          price_max: 1500,
          rating_min: 0,
          in_stock: false,
          sort: 'newest'
        };

        if (catSelect) catSelect.value = '';
        if (priceSlider) priceSlider.value = 1500;
        if (priceValue) priceValue.textContent = this.formatPrice(1500);
        if (ratingSelect) ratingSelect.value = '0';
        if (stockCheckbox) stockCheckbox.checked = false;
        if (sortSelect) sortSelect.value = 'newest';

        this.fetchProducts();
      });
    }

    // US-EC-005: Instant Autocomplete Search Bar (<50ms response requirement)
    const searchInput = document.getElementById('search-input');
    const dropdown = document.getElementById('autocomplete-dropdown');

    if (searchInput && dropdown) {
      searchInput.addEventListener('input', (e) => {
        clearTimeout(this.state.searchDebounceTimer);
        const query = e.target.value.trim();

        if (!query) {
          dropdown.classList.remove('active');
          dropdown.innerHTML = '';
          return;
        }

        // Fast debounce 10ms for instant feel
        this.state.searchDebounceTimer = setTimeout(() => {
          this.fetchSearchAutocomplete(query);
        }, 10);
      });

      // Close dropdown when clicking outside
      document.addEventListener('click', (e) => {
        if (!searchInput.contains(e.target) && !dropdown.contains(e.target)) {
          dropdown.classList.remove('active');
        }
      });
    }

    // Mega menu link category navigation
    const megaLinks = document.querySelectorAll('.mega-links a');
    megaLinks.forEach(link => {
      link.addEventListener('click', (e) => {
        e.preventDefault();
        const cat = link.getAttribute('data-category');
        if (cat) this.filterByCategory(cat);
      });
    });

    // Product Modal Close Button
    const modalClose = document.getElementById('modal-close-btn');
    const modalOverlay = document.getElementById('product-modal');
    if (modalClose && modalOverlay) {
      modalClose.addEventListener('click', () => {
        modalOverlay.classList.remove('active');
      });

      modalOverlay.addEventListener('click', (e) => {
        if (e.target === modalOverlay) {
          modalOverlay.classList.remove('active');
        }
      });
    }

    // Cart Button & Drawer Overlay Bindings US-EC-007
    const cartBtn = document.getElementById('cart-btn');
    const drawerOverlay = document.getElementById('cart-drawer-overlay');
    const drawerClose = document.getElementById('cart-drawer-close');

    if (cartBtn) {
      cartBtn.addEventListener('click', () => this.openCartDrawer());
    }
    if (drawerClose && drawerOverlay) {
      drawerClose.addEventListener('click', () => this.closeCartDrawer());
      drawerOverlay.addEventListener('click', (e) => {
        if (e.target === drawerOverlay) this.closeCartDrawer();
      });
    }

    // Coupon Apply Button US-EC-008
    const applyCouponBtn = document.getElementById('apply-coupon-btn');
    if (applyCouponBtn) {
      applyCouponBtn.addEventListener('click', () => this.applyCoupon());
    }

    // Checkout Button & Modal Bindings US-EC-009
    const checkoutBtn = document.getElementById('checkout-btn');
    if (checkoutBtn) {
      checkoutBtn.addEventListener('click', () => this.openCheckoutModal());
    }

    const checkoutModal = document.getElementById('checkout-modal');
    const checkoutClose = document.getElementById('checkout-modal-close');
    if (checkoutClose && checkoutModal) {
      checkoutClose.addEventListener('click', () => this.closeCheckoutModal());
      checkoutModal.addEventListener('click', (e) => {
        if (e.target === checkoutModal) this.closeCheckoutModal();
      });
    }

    // Auth Modal Bindings US-EC-010
    const authModal = document.getElementById('auth-modal');
    const authClose = document.getElementById('auth-modal-close');
    if (authClose && authModal) {
      authClose.addEventListener('click', () => this.closeAuthModal());
      authModal.addEventListener('click', (e) => {
        if (e.target === authModal) this.closeAuthModal();
      });
    }

    // Orders Modal Bindings US-EC-012
    const ordersModal = document.getElementById('orders-modal');
    const ordersClose = document.getElementById('orders-modal-close');
    if (ordersClose && ordersModal) {
      ordersClose.addEventListener('click', () => this.closeOrdersModal());
      ordersModal.addEventListener('click', (e) => {
        if (e.target === ordersModal) this.closeOrdersModal();
      });
    }

    // Invoice Modal Bindings US-EC-012
    const invoiceModal = document.getElementById('invoice-modal');
    if (invoiceModal) {
      invoiceModal.addEventListener('click', (e) => {
        if (e.target === invoiceModal) this.closeInvoiceModal();
      });
    }
  },

  // US-EC-004: Fetch Products with Filters
  async fetchProducts() {
    try {
      const params = new URLSearchParams();
      if (this.state.filters.category) params.append('category', this.state.filters.category);
      if (this.state.filters.price_max < 1500) params.append('price_max', this.state.filters.price_max);
      if (this.state.filters.rating_min > 0) params.append('rating_min', this.state.filters.rating_min);
      if (this.state.filters.in_stock) params.append('in_stock', '1');
      if (this.state.filters.sort) params.append('sort', this.state.filters.sort);

      const res = await fetch(`/api/products?${params.toString()}`);
      const data = await res.json();
      this.state.products = data.products || [];
      this.renderProductGrid();
    } catch (err) {
      console.error('Failed to fetch products:', err);
    }
  },

  filterByCategory(categorySlug) {
    this.state.filters.category = categorySlug;
    const catSelect = document.getElementById('category-filter');
    if (catSelect) catSelect.value = categorySlug;

    // Scroll to catalog
    const catalogSec = document.getElementById('catalog');
    if (catalogSec) catalogSec.scrollIntoView({ behavior: 'smooth' });

    this.fetchProducts();
  },

  // US-EC-004: Render Catalog Product Cards
  renderProductGrid() {
    const grid = document.getElementById('product-grid');
    const countEl = document.getElementById('products-count');
    if (!grid) return;

    if (countEl) {
      countEl.textContent = `Showing ${this.state.products.length} products`;
    }

    if (this.state.products.length === 0) {
      grid.innerHTML = `
        <div style="grid-column: 1 / -1; text-align: center; padding: 3rem; color: var(--text-muted);">
          <h3>No products match your selected filters</h3>
          <p>Try resetting your filter parameters to view more items.</p>
        </div>
      `;
      return;
    }

    grid.innerHTML = this.state.products.map(p => {
      const displayPrice = p.sale_price || p.price;
      const formattedPrice = this.formatPrice(displayPrice);
      const formattedOrigPrice = p.sale_price ? this.formatPrice(p.price) : null;
      const mainImage = p.images && p.images.length ? p.images[0] : '';
      const inStock = p.stock_quantity > 0;

      return `
        <div class="product-card" data-slug="${p.slug}">
          <div class="product-img-wrapper">
            <img src="${mainImage}" alt="${p.title}" class="product-img" loading="lazy" />
            ${p.is_featured ? '<span class="badge-featured">Featured</span>' : ''}
            <span class="badge-stock ${inStock ? 'in-stock' : 'out-stock'}">
              ${inStock ? `In Stock (${p.stock_quantity})` : 'Out of Stock'}
            </span>
          </div>

          <div class="product-body">
            <span class="product-category">${p.category_name || 'Category'}</span>
            <h3 class="product-title">${p.title}</h3>
            
            <div class="product-rating">
              <span>${this.renderStarIcons(p.rating_avg)}</span>
              <span style="font-weight: 700; color: var(--text-primary);">${p.rating_avg.toFixed(1)}</span>
              <span class="rating-count">(${p.rating_count})</span>
            </div>

            <div class="product-footer">
              <div class="price-container">
                <span class="current-price">${formattedPrice}</span>
                ${formattedOrigPrice ? `<span class="original-price">${formattedOrigPrice}</span>` : ''}
              </div>
              <button class="btn-card" onclick="app.openProductModalBySlug('${p.slug}')">Quick View</button>
            </div>
          </div>
        </div>
      `;
    }).join('');
  },

  // US-EC-005: Autocomplete API fetch
  async fetchSearchAutocomplete(query) {
    try {
      const dropdown = document.getElementById('autocomplete-dropdown');
      const res = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
      const data = await res.json();
      const suggestions = data.suggestions || [];

      if (!dropdown) return;

      if (suggestions.length === 0) {
        dropdown.innerHTML = `
          <div style="padding: 1rem; text-align: center; color: var(--text-muted); font-size: 0.85rem;">
            No products found matching "${query}"
          </div>
        `;
        dropdown.classList.add('active');
        return;
      }

      dropdown.innerHTML = suggestions.map(item => `
        <div class="autocomplete-item" onclick="app.openProductModalBySlug('${item.slug}')">
          <img src="${item.image || ''}" alt="${item.title}" class="autocomplete-img" />
          <div class="autocomplete-details">
            <div class="autocomplete-title">${item.title}</div>
            <div class="autocomplete-meta">
              <span style="color: var(--accent-primary); font-weight: 700;">${this.formatPrice(item.effective_price)}</span>
              <span>• ${item.category_name}</span>
              <span>• ★ ${item.rating_avg}</span>
            </div>
          </div>
        </div>
      `).join('');

      dropdown.classList.add('active');
    } catch (err) {
      console.error('Search error:', err);
    }
  },

  // US-EC-006: Detailed Product View Modal
  async openProductModalBySlug(slug) {
    try {
      // Close search autocomplete if open
      const dropdown = document.getElementById('autocomplete-dropdown');
      if (dropdown) dropdown.classList.remove('active');

      const res = await fetch(`/api/products/${slug}`);
      if (!res.ok) return;
      const data = await res.json();
      const p = data.product;
      this.state.currentModalProduct = p;

      const overlay = document.getElementById('product-modal');
      const mainImg = document.getElementById('modal-main-img');
      const thumbsContainer = document.getElementById('modal-thumbs');
      const catEl = document.getElementById('modal-category');
      const titleEl = document.getElementById('modal-title');
      const starsEl = document.getElementById('modal-stars');
      const ratingValEl = document.getElementById('modal-rating-val');
      const ratingCountEl = document.getElementById('modal-rating-count');
      const stockBadge = document.getElementById('modal-stock-badge');
      const priceEl = document.getElementById('modal-price');
      const salePriceEl = document.getElementById('modal-sale-price');
      const descEl = document.getElementById('modal-desc');
      const addBtn = document.getElementById('modal-add-cart-btn');

      if (!p || !overlay) return;

      catEl.textContent = p.category_name;
      titleEl.textContent = p.title;
      starsEl.textContent = this.renderStarIcons(p.rating_avg);
      ratingValEl.textContent = p.rating_avg.toFixed(1);
      ratingCountEl.textContent = `(${p.rating_count} reviews)`;
      descEl.textContent = p.description;

      const displayPrice = p.sale_price || p.price;
      priceEl.textContent = this.formatPrice(displayPrice);
      if (p.sale_price) {
        salePriceEl.textContent = this.formatPrice(p.price);
        salePriceEl.style.display = 'inline';
      } else {
        salePriceEl.style.display = 'none';
      }

      if (p.is_in_stock) {
        stockBadge.className = 'badge-stock in-stock';
        stockBadge.textContent = `In Stock (${p.stock_quantity} available)`;
      } else {
        stockBadge.className = 'badge-stock out-stock';
        stockBadge.textContent = 'Out of Stock';
      }

      // Images Gallery
      if (p.images && p.images.length > 0) {
        mainImg.src = p.images[0];
        thumbsContainer.innerHTML = p.images.map((imgUrl, idx) => `
          <img 
            src="${imgUrl}" 
            class="thumb-img ${idx === 0 ? 'active' : ''}" 
            onclick="app.switchModalImage('${imgUrl}', this)"
          />
        `).join('');
      }

      addBtn.onclick = () => {
        const qty = parseInt(document.getElementById('modal-quantity').value || '1', 10);
        this.addToCart(p, qty);
      };

      overlay.classList.add('active');
    } catch (err) {
      console.error('Failed to open modal:', err);
    }
  },

  switchModalImage(imgUrl, thumbEl) {
    const mainImg = document.getElementById('modal-main-img');
    if (mainImg) mainImg.src = imgUrl;

    const thumbs = document.querySelectorAll('.thumb-img');
    thumbs.forEach(t => t.classList.remove('active'));
    if (thumbEl) thumbEl.classList.add('active');
  },

  // US-EC-003: Newsletter & Currency Handlers
  handleNewsletterSubmit() {
    const emailInput = document.getElementById('newsletter-email');
    const msgEl = document.getElementById('newsletter-msg');
    if (emailInput && msgEl) {
      msgEl.textContent = `✓ Thank you! ${emailInput.value} has been subscribed to Storefront Pro updates.`;
      emailInput.value = '';
    }
  },

  handleCurrencyChange(currency) {
    this.state.currency = currency;
    localStorage.setItem('ec_currency', currency);
    const priceValue = document.getElementById('price-slider-value');
    if (priceValue) {
      priceValue.textContent = this.formatPrice(this.state.filters.price_max);
    }
    this.renderProductGrid();
  },

  formatPrice(amount) {
    const rate = this.state.rates[this.state.currency] || 1.0;
    const symbol = this.state.currencySymbols[this.state.currency] || '$';
    const converted = (amount * rate).toFixed(2);
    return `${symbol}${converted}`;
  },

  renderStarIcons(rating) {
    const fullStars = Math.floor(rating);
    const hasHalf = rating - fullStars >= 0.5;
    let starsStr = '★'.repeat(fullStars);
    if (hasHalf) starsStr += '½';
    const emptyStars = 5 - Math.ceil(rating);
    if (emptyStars > 0) starsStr += '☆'.repeat(emptyStars);
    return starsStr;
  },

  async fetchCategories() {
    try {
      const res = await fetch('/api/categories');
      const data = await res.json();
      this.state.categories = data.categories || [];
    } catch (err) {
      console.error('Failed to fetch categories:', err);
    }
  },

  // US-EC-007: Cart Drawer Methods
  openCartDrawer() {
    const overlay = document.getElementById('cart-drawer-overlay');
    if (overlay) {
      overlay.classList.add('active');
      this.renderCartDrawer();
    }
  },

  closeCartDrawer() {
    const overlay = document.getElementById('cart-drawer-overlay');
    if (overlay) overlay.classList.remove('active');
  },

  renderCartDrawer() {
    const itemsContainer = document.getElementById('cart-drawer-items');
    const countEl = document.getElementById('cart-drawer-count');
    const subtotalEl = document.getElementById('cart-subtotal');
    const discountRow = document.getElementById('cart-discount-row');
    const couponCodeEl = document.getElementById('cart-coupon-code');
    const discountEl = document.getElementById('cart-discount');
    const taxEl = document.getElementById('cart-tax');
    const shippingEl = document.getElementById('cart-shipping');
    const totalEl = document.getElementById('cart-total');

    if (!itemsContainer) return;

    const totalQty = this.state.cart.reduce((sum, i) => sum + i.quantity, 0);
    if (countEl) countEl.textContent = `(${totalQty} ${totalQty === 1 ? 'item' : 'items'})`;

    if (this.state.cart.length === 0) {
      itemsContainer.innerHTML = `
        <div class="cart-empty-state">
          <p style="font-size: 2.5rem; margin-bottom: 0.5rem;">🛒</p>
          <h4 style="color: var(--text-primary);">Your cart is currently empty</h4>
          <p style="font-size: 0.85rem; margin-top: 0.5rem;">Discover our latest products and start shopping!</p>
        </div>
      `;
    } else {
      itemsContainer.innerHTML = this.state.cart.map(item => {
        const itemPrice = item.price;
        const itemSubtotal = itemPrice * item.quantity;
        const mainProduct = this.state.products.find(p => p.id === item.id);
        const maxStock = mainProduct ? mainProduct.stock_quantity : (item.stock_quantity || 99);

        return `
          <div class="cart-item">
            <img src="${item.image || ''}" alt="${item.title}" class="cart-item-img" />
            <div class="cart-item-info">
              <div class="cart-item-title">${item.title}</div>
              <div class="cart-item-price">${this.formatPrice(itemPrice)}</div>
              <div class="cart-item-controls">
                <button class="btn-qty" onclick="app.updateQuantity('${item.id}', ${item.quantity - 1})">-</button>
                <span class="qty-val">${item.quantity}</span>
                <button class="btn-qty" ${item.quantity >= maxStock ? 'disabled' : ''} onclick="app.updateQuantity('${item.id}', ${item.quantity + 1})">+</button>
                <button class="btn-remove" onclick="app.removeFromCart('${item.id}')">🗑️ Remove</button>
              </div>
            </div>
            <div style="font-weight: 700; font-size: 0.9rem;">${this.formatPrice(itemSubtotal)}</div>
          </div>
        `;
      }).join('');
    }

    const subtotal = this.state.cart.reduce((sum, item) => sum + item.price * item.quantity, 0);

    let discount = 0;
    if (this.state.appliedCoupon) {
      const c = this.state.appliedCoupon;
      if (c.discount_type === 'percentage') {
        discount = (subtotal * c.discount_percent) / 100;
      } else {
        discount = c.discount_amount || c.discount || 0;
      }
      if (discount > subtotal) discount = subtotal;
    }

    const taxableAmount = Math.max(0, subtotal - discount);
    const tax = Math.round(taxableAmount * 0.08 * 100) / 100;
    const shipping = subtotal > 0 ? (subtotal >= 100 ? 0 : 10) : 0;
    const total = Math.round((taxableAmount + tax + shipping) * 100) / 100;

    if (subtotalEl) subtotalEl.textContent = this.formatPrice(subtotal);

    if (this.state.appliedCoupon && discount > 0) {
      if (discountRow) discountRow.style.display = 'flex';
      if (couponCodeEl) couponCodeEl.textContent = this.state.appliedCoupon.code;
      if (discountEl) discountEl.textContent = `-${this.formatPrice(discount)}`;
    } else {
      if (discountRow) discountRow.style.display = 'none';
    }

    if (taxEl) taxEl.textContent = this.formatPrice(tax);
    if (shippingEl) shippingEl.textContent = shipping === 0 ? 'FREE' : this.formatPrice(shipping);
    if (totalEl) totalEl.textContent = this.formatPrice(total);
  },

  updateQuantity(productId, newQty) {
    const itemIndex = this.state.cart.findIndex(i => i.id === productId);
    if (itemIndex === -1) return;

    if (newQty <= 0) {
      this.state.cart.splice(itemIndex, 1);
    } else {
      const mainProduct = this.state.products.find(p => p.id === productId);
      const maxStock = mainProduct ? mainProduct.stock_quantity : (this.state.cart[itemIndex].stock_quantity || 99);

      if (newQty > maxStock) {
        this.state.cart[itemIndex].quantity = maxStock;
      } else {
        this.state.cart[itemIndex].quantity = newQty;
      }
    }

    localStorage.setItem('ec_cart', JSON.stringify(this.state.cart));
    this.updateCartBadge();
    this.renderCartDrawer();
  },

  removeFromCart(productId) {
    this.state.cart = this.state.cart.filter(i => i.id !== productId);
    localStorage.setItem('ec_cart', JSON.stringify(this.state.cart));
    this.updateCartBadge();
    this.renderCartDrawer();
  },

  // US-EC-008: Promo Coupon Application
  async applyCoupon() {
    const input = document.getElementById('coupon-code-input');
    const feedback = document.getElementById('coupon-feedback');
    if (!input) return;

    const code = input.value.trim();
    if (!code) {
      if (feedback) {
        feedback.style.color = 'var(--danger)';
        feedback.textContent = 'Please enter a coupon code.';
      }
      return;
    }

    const subtotal = this.state.cart.reduce((sum, item) => sum + item.price * item.quantity, 0);

    try {
      const res = await fetch('/api/cart/coupon', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code, subtotal })
      });

      const data = await res.json();

      if (!res.ok || !data.valid) {
        this.state.appliedCoupon = null;
        if (feedback) {
          feedback.style.color = 'var(--danger)';
          feedback.textContent = `❌ ${data.error || 'Invalid coupon code'}`;
        }
      } else {
        this.state.appliedCoupon = data;
        if (feedback) {
          feedback.style.color = 'var(--success)';
          const label = data.discount_type === 'fixed' ? `$${data.discount_amount} OFF` : `${data.discount_percent}% OFF`;
          feedback.textContent = `✓ Coupon "${data.code}" applied! (${label})`;
        }
      }

      this.renderCartDrawer();
    } catch (err) {
      if (feedback) {
        feedback.style.color = 'var(--danger)';
        feedback.textContent = '❌ Failed to validate coupon code';
      }
    }
  },

  // US-EC-009: 3-Step Checkout Wizard
  openCheckoutModal() {
    if (this.state.cart.length === 0) {
      alert('Your cart is empty. Add items to your cart before checking out!');
      this.openCartDrawer();
      return;
    }

    this.closeCartDrawer();
    const modal = document.getElementById('checkout-modal');
    if (modal) {
      modal.classList.add('active');
      this.goToCheckoutStep(1);
    }
  },

  closeCheckoutModal() {
    const modal = document.getElementById('checkout-modal');
    if (modal) modal.classList.remove('active');
  },

  goToCheckoutStep(stepNum) {
    const s1 = document.getElementById('checkout-step-1');
    const s2 = document.getElementById('checkout-step-2');
    const s3 = document.getElementById('checkout-step-3');
    const sSuccess = document.getElementById('checkout-step-success');

    const ind1 = document.getElementById('step-indicator-1');
    const ind2 = document.getElementById('step-indicator-2');
    const ind3 = document.getElementById('step-indicator-3');

    [s1, s2, s3, sSuccess].forEach(s => s && s.classList.remove('active'));
    [ind1, ind2, ind3].forEach(i => i && i.classList.remove('active', 'completed'));

    if (stepNum === 1) {
      if (s1) s1.classList.add('active');
      if (ind1) ind1.classList.add('active');
    } else if (stepNum === 2) {
      if (s2) s2.classList.add('active');
      if (ind1) ind1.classList.add('completed');
      if (ind2) ind2.classList.add('active');
    } else if (stepNum === 3) {
      if (s3) s3.classList.add('active');
      if (ind1) ind1.classList.add('completed');
      if (ind2) ind2.classList.add('completed');
      if (ind3) ind3.classList.add('active');
      this.renderOrderReview();
    }
  },

  handlePaymentMethodChange(method) {
    this.state.selectedPaymentMethod = method;
    const ccFields = document.getElementById('credit-card-fields');
    const ppFields = document.getElementById('paypal-fields');
    const apFields = document.getElementById('apple-pay-fields');

    const labels = document.querySelectorAll('.payment-radio-card');
    labels.forEach(l => {
      const radio = l.querySelector('input');
      if (radio && radio.value === method) {
        l.classList.add('active');
      } else {
        l.classList.remove('active');
      }
    });

    if (ccFields) ccFields.style.display = method === 'credit_card' ? 'block' : 'none';
    if (ppFields) ppFields.style.display = method === 'paypal' ? 'block' : 'none';
    if (apFields) apFields.style.display = method === 'apple_pay' ? 'block' : 'none';
  },

  validatePaymentStepAndNext() {
    if (this.state.selectedPaymentMethod === 'credit_card') {
      const cardNum = document.getElementById('card-number').value.trim();
      const cardExp = document.getElementById('card-expiry').value.trim();
      const cardCvc = document.getElementById('card-cvc').value.trim();

      if (!cardNum || cardNum.length < 14) {
        alert('Please enter a valid credit card number.');
        return;
      }
      if (!cardExp || !cardExp.includes('/')) {
        alert('Please enter a valid expiration date (MM/YY).');
        return;
      }
      if (!cardCvc || cardCvc.length < 3) {
        alert('Please enter a valid CVC.');
        return;
      }
    }

    this.goToCheckoutStep(3);
  },

  renderOrderReview() {
    const fullname = document.getElementById('shipping-fullname').value;
    const address = document.getElementById('shipping-address').value;
    const city = document.getElementById('shipping-city').value;
    const zip = document.getElementById('shipping-zip').value;
    const country = document.getElementById('shipping-country').value;

    const shipSummary = document.getElementById('review-shipping-summary');
    if (shipSummary) {
      shipSummary.innerHTML = `
        <strong>${fullname}</strong><br />
        ${address}<br />
        ${city}, ${zip}, ${country}
      `;
    }

    const paySummary = document.getElementById('review-payment-summary');
    if (paySummary) {
      let payName = 'Credit Card';
      if (this.state.selectedPaymentMethod === 'paypal') payName = 'PayPal';
      if (this.state.selectedPaymentMethod === 'apple_pay') payName = 'Apple Pay';
      paySummary.innerHTML = `Method: <strong>${payName}</strong>`;
    }

    const itemsList = document.getElementById('review-items-list');
    if (itemsList) {
      itemsList.innerHTML = this.state.cart.map(item => `
        <div class="review-item-row">
          <span>${item.quantity}x ${item.title}</span>
          <span>${this.formatPrice(item.price * item.quantity)}</span>
        </div>
      `).join('');
    }

    const subtotal = this.state.cart.reduce((sum, item) => sum + item.price * item.quantity, 0);

    let discount = 0;
    if (this.state.appliedCoupon) {
      const c = this.state.appliedCoupon;
      if (c.discount_type === 'percentage') {
        discount = (subtotal * c.discount_percent) / 100;
      } else {
        discount = c.discount_amount || c.discount || 0;
      }
      if (discount > subtotal) discount = subtotal;
    }

    const taxableAmount = Math.max(0, subtotal - discount);
    const tax = Math.round(taxableAmount * 0.08 * 100) / 100;
    const shipping = subtotal > 0 ? (subtotal >= 100 ? 0 : 10) : 0;
    const total = Math.round((taxableAmount + tax + shipping) * 100) / 100;

    const revSubtotal = document.getElementById('review-subtotal');
    const revDiscountRow = document.getElementById('review-discount-row');
    const revCouponCode = document.getElementById('review-coupon-code');
    const revDiscount = document.getElementById('review-discount');
    const revTax = document.getElementById('review-tax');
    const revShipping = document.getElementById('review-shipping');
    const revTotal = document.getElementById('review-total');

    if (revSubtotal) revSubtotal.textContent = this.formatPrice(subtotal);

    if (this.state.appliedCoupon && discount > 0) {
      if (revDiscountRow) revDiscountRow.style.display = 'flex';
      if (revCouponCode) revCouponCode.textContent = this.state.appliedCoupon.code;
      if (revDiscount) revDiscount.textContent = `-${this.formatPrice(discount)}`;
    } else {
      if (revDiscountRow) revDiscountRow.style.display = 'none';
    }

    if (revTax) revTax.textContent = this.formatPrice(tax);
    if (revShipping) revShipping.textContent = shipping === 0 ? 'FREE' : this.formatPrice(shipping);
    if (revTotal) revTotal.textContent = this.formatPrice(total);
  },

  async placeOrder() {
    const errorEl = document.getElementById('checkout-error-msg');
    const btn = document.getElementById('place-order-btn');
    if (errorEl) errorEl.textContent = '';
    if (btn) btn.disabled = true;

    try {
      const shippingAddress = {
        fullname: document.getElementById('shipping-fullname').value.trim(),
        address: document.getElementById('shipping-address').value.trim(),
        city: document.getElementById('shipping-city').value.trim(),
        zip: document.getElementById('shipping-zip').value.trim(),
        country: document.getElementById('shipping-country').value
      };

      const payload = {
        user_id: 'u_customer1',
        items: this.state.cart.map(item => ({
          product_id: item.id,
          quantity: item.quantity
        })),
        shipping_address: shippingAddress,
        payment_method: this.state.selectedPaymentMethod,
        coupon_code: this.state.appliedCoupon ? this.state.appliedCoupon.code : null
      };

      const headers = { 'Content-Type': 'application/json' };
      if (this.state.token) {
        headers['Authorization'] = `Bearer ${this.state.token}`;
      }

      const res = await fetch('/api/orders', {
        method: 'POST',
        headers,
        body: JSON.stringify(payload)
      });

      const data = await res.json();

      if (!res.ok || !data.success) {
        if (errorEl) errorEl.textContent = `❌ ${data.error || 'Failed to place order'}`;
        if (btn) btn.disabled = false;
        return;
      }

      // Success! Clear cart & state
      this.state.cart = [];
      this.state.appliedCoupon = null;
      localStorage.removeItem('ec_cart');
      this.updateCartBadge();
      this.renderCartDrawer();

      const successId = document.getElementById('success-order-id');
      if (successId) successId.textContent = data.order_id;

      const s3 = document.getElementById('checkout-step-3');
      const sSuccess = document.getElementById('checkout-step-success');
      const ind3 = document.getElementById('step-indicator-3');

      if (s3) s3.classList.remove('active');
      if (sSuccess) sSuccess.classList.add('active');
      if (ind3) ind3.classList.add('completed');
    } catch (err) {
      if (errorEl) errorEl.textContent = `❌ Network error: ${err.message}`;
    } finally {
      if (btn) btn.disabled = false;
    }
  },

  // US-EC-010: Authentication State Management
  async initAuth() {
    const token = localStorage.getItem('ec_token');
    if (token) {
      this.state.token = token;
      try {
        const res = await fetch('/api/auth/me', {
          headers: { 'Authorization': `Bearer ${token}` }
        });
        if (res.ok) {
          const data = await res.json();
          this.state.user = data.user;
          localStorage.setItem('ec_user', JSON.stringify(data.user));
        } else {
          this.logout();
        }
      } catch (err) {
        console.error('Failed to verify token:', err);
      }
    }
    this.updateAuthUI();
  },

  updateAuthUI() {
    const authBtn = document.getElementById('auth-btn');
    const userMenu = document.getElementById('user-menu');
    const userEmailEl = document.getElementById('user-display-email');

    if (this.state.user && this.state.token) {
      if (authBtn) authBtn.style.display = 'none';
      if (userMenu) userMenu.style.display = 'flex';
      if (userEmailEl) userEmailEl.textContent = this.state.user.email;
    } else {
      if (authBtn) authBtn.style.display = 'inline-block';
      if (userMenu) userMenu.style.display = 'none';
      if (userEmailEl) userEmailEl.textContent = '';
    }
  },

  openAuthModal() {
    const modal = document.getElementById('auth-modal');
    if (modal) {
      modal.classList.add('active');
      this.switchAuthTab('login');
    }
  },

  closeAuthModal() {
    const modal = document.getElementById('auth-modal');
    if (modal) modal.classList.remove('active');
    const errorMsg = document.getElementById('auth-error-msg');
    if (errorMsg) errorMsg.textContent = '';
  },

  switchAuthTab(mode) {
    this.state.authMode = mode;
    const loginTab = document.getElementById('login-tab');
    const regTab = document.getElementById('register-tab');
    const titleEl = document.getElementById('auth-form-title');
    const submitBtn = document.getElementById('auth-submit-btn');
    const errorMsg = document.getElementById('auth-error-msg');

    if (errorMsg) errorMsg.textContent = '';

    if (mode === 'login') {
      if (loginTab) loginTab.classList.add('active');
      if (regTab) regTab.classList.remove('active');
      if (titleEl) titleEl.textContent = 'Customer Login';
      if (submitBtn) submitBtn.textContent = 'Sign In';
    } else {
      if (regTab) regTab.classList.add('active');
      if (loginTab) loginTab.classList.remove('active');
      if (titleEl) titleEl.textContent = 'Create Customer Account';
      if (submitBtn) submitBtn.textContent = 'Create Account';
    }
  },

  async handleAuthSubmit() {
    const email = document.getElementById('auth-email').value.trim();
    const password = document.getElementById('auth-password').value;
    const errorMsg = document.getElementById('auth-error-msg');
    const submitBtn = document.getElementById('auth-submit-btn');

    if (!email || !password) {
      if (errorMsg) errorMsg.textContent = 'Please provide both email and password.';
      return;
    }

    if (submitBtn) submitBtn.disabled = true;
    if (errorMsg) errorMsg.textContent = '';

    const endpoint = this.state.authMode === 'login' ? '/api/auth/login' : '/api/auth/register';

    try {
      const res = await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });

      const data = await res.json();

      if (!res.ok || !data.token) {
        if (errorMsg) errorMsg.textContent = `❌ ${data.error || 'Authentication failed'}`;
        if (submitBtn) submitBtn.disabled = false;
        return;
      }

      this.state.token = data.token;
      this.state.user = data.user;
      localStorage.setItem('ec_token', data.token);
      localStorage.setItem('ec_user', JSON.stringify(data.user));

      this.updateAuthUI();
      this.closeAuthModal();
      const form = document.getElementById('auth-form');
      if (form) form.reset();
    } catch (err) {
      if (errorMsg) errorMsg.textContent = `❌ Network error: ${err.message}`;
    } finally {
      if (submitBtn) submitBtn.disabled = false;
    }
  },

  logout() {
    this.state.token = null;
    this.state.user = null;
    localStorage.removeItem('ec_token');
    localStorage.removeItem('ec_user');
    this.updateAuthUI();
  },

  // US-EC-012: Customer Dashboard & Order History
  async openOrdersModal() {
    const modal = document.getElementById('orders-modal');
    if (modal) {
      modal.classList.add('active');
      await this.fetchMyOrders();
    }
  },

  closeOrdersModal() {
    const modal = document.getElementById('orders-modal');
    if (modal) modal.classList.remove('active');
  },

  async fetchMyOrders() {
    const container = document.getElementById('orders-list-container');
    if (!container) return;

    container.innerHTML = '<div style="text-align: center; padding: 2rem;">Loading your order history...</div>';

    try {
      const headers = {};
      if (this.state.token) {
        headers['Authorization'] = `Bearer ${this.state.token}`;
      }

      const url = this.state.token
        ? '/api/orders/my-orders'
        : `/api/orders/my-orders?user_id=${this.state.user ? this.state.user.id : 'u_customer1'}`;

      const res = await fetch(url, { headers });
      const data = await res.json();

      if (!res.ok) {
        container.innerHTML = `<div style="color: var(--danger); padding: 1.5rem; text-align: center;">${data.error || 'Failed to fetch orders.'}</div>`;
        return;
      }

      this.state.myOrders = data.orders || [];
      this.renderMyOrders();
    } catch (err) {
      container.innerHTML = `<div style="color: var(--danger); padding: 1.5rem; text-align: center;">Error loading orders: ${err.message}</div>`;
    }
  },

  renderMyOrders() {
    const container = document.getElementById('orders-list-container');
    if (!container) return;

    if (!this.state.myOrders || this.state.myOrders.length === 0) {
      container.innerHTML = `
        <div style="text-align: center; padding: 3rem; color: var(--text-muted);">
          <p style="font-size: 2.5rem; margin-bottom: 0.5rem;">📦</p>
          <h4>No orders found</h4>
          <p style="font-size: 0.85rem; margin-top: 0.5rem;">You have not placed any orders yet.</p>
        </div>
      `;
      return;
    }

    container.innerHTML = this.state.myOrders.map(order => {
      const dateStr = new Date(order.created_at || Date.now()).toLocaleDateString('en-US', {
        year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit'
      });

      const steps = ['pending', 'processing', 'shipped', 'delivered'];
      const currentIdx = steps.indexOf(order.status);
      const isCancelled = order.status === 'cancelled';

      const progressPercent = isCancelled ? 0 : Math.max(0, (currentIdx / 3) * 100);

      const timelineHtml = isCancelled ? `
        <div class="tracking-timeline">
          <div class="tracking-timeline-title" style="color: var(--danger);">Order Cancelled</div>
          <p style="font-size: 0.85rem; color: var(--danger); margin: 0;">This order was cancelled and is no longer being processed.</p>
        </div>
      ` : `
        <div class="tracking-timeline">
          <div class="tracking-timeline-title">Order Tracking Timeline</div>
          <div class="tracking-steps">
            <div class="tracking-line">
              <div class="tracking-line-progress" style="width: ${progressPercent}%;"></div>
            </div>
            ${steps.map((step, idx) => {
              const completed = currentIdx >= idx;
              const active = currentIdx === idx;
              const label = step.charAt(0).toUpperCase() + step.slice(1);
              return `
                <div class="tracking-step ${completed ? 'completed' : ''} ${active ? 'active' : ''}">
                  <div class="step-circle">${completed ? '✓' : (idx + 1)}</div>
                  <span class="step-label">${label}</span>
                </div>
              `;
            }).join('')}
          </div>
        </div>
      `;

      const itemsHtml = order.items.map(item => `
        <tr>
          <td>
            <div style="display: flex; align-items: center; gap: 0.75rem;">
              <img src="${item.images && item.images[0] ? item.images[0] : ''}" style="width: 36px; height: 36px; border-radius: 6px; object-fit: cover;" />
              <span style="font-weight: 600;">${item.product_title || 'Product'}</span>
            </div>
          </td>
          <td>${item.quantity}</td>
          <td>${this.formatPrice(item.unit_price)}</td>
          <td style="font-weight: 700;">${this.formatPrice(item.unit_price * item.quantity)}</td>
        </tr>
      `).join('');

      return `
        <div class="order-card" id="order-card-${order.id}">
          <div class="order-card-header">
            <div>
              <span class="order-id-badge">${order.id}</span>
              <span style="font-size: 0.8rem; color: var(--text-muted); margin-left: 0.5rem;">${dateStr}</span>
            </div>
            <span class="order-status-badge status-${order.status}">${order.status}</span>
          </div>

          ${timelineHtml}

          <table class="order-items-table">
            <thead>
              <tr>
                <th>Item</th>
                <th>Qty</th>
                <th>Price</th>
                <th>Subtotal</th>
              </tr>
            </thead>
            <tbody>
              ${itemsHtml}
            </tbody>
          </table>

          <div class="order-card-footer">
            <div>
              <span style="font-size: 0.85rem; color: var(--text-secondary);">Total Paid: </span>
              <span style="font-size: 1.1rem; font-weight: 800; color: var(--accent-primary);">${this.formatPrice(order.total_amount)}</span>
            </div>
            <button class="btn-secondary btn-sm export-invoice-btn" onclick="app.exportInvoice('${order.id}')">📄 Export Invoice</button>
          </div>
        </div>
      `;
    }).join('');
  },

  // US-EC-012: Clean Printable Invoice Export
  exportInvoice(orderId) {
    const order = (this.state.myOrders || []).find(o => o.id === orderId);
    if (!order) {
      alert('Order details not found.');
      return;
    }

    const container = document.getElementById('invoice-content-body');
    const modal = document.getElementById('invoice-modal');
    if (!container || !modal) return;

    const dateStr = new Date(order.created_at || Date.now()).toLocaleDateString('en-US', {
      year: 'numeric', month: 'long', day: 'numeric'
    });

    const addr = order.shipping_address || {};

    const itemsHtml = order.items.map(item => `
      <tr>
        <td><strong>${item.product_title || 'Product'}</strong></td>
        <td style="text-align: center;">${item.quantity}</td>
        <td style="text-align: right;">$${item.unit_price.toFixed(2)}</td>
        <td style="text-align: right;">$${(item.unit_price * item.quantity).toFixed(2)}</td>
      </tr>
    `).join('');

    const subtotal = order.items.reduce((sum, i) => sum + i.unit_price * i.quantity, 0);
    const tax = Math.round(subtotal * 0.08 * 100) / 100;
    const shipping = subtotal >= 100 ? 0 : 10;

    container.innerHTML = `
      <div class="invoice-header-row">
        <div>
          <div class="invoice-brand">🛍️ Storefront Pro</div>
          <div style="font-size: 0.85rem; color: #64748b; margin-top: 0.25rem;">
            100 E-Commerce Way, Suite 500<br />
            San Francisco, CA 94105<br />
            support@storefrontpro.com
          </div>
        </div>
        <div style="text-align: right;">
          <h2 style="margin: 0; font-size: 1.75rem; color: #1e293b;">INVOICE</h2>
          <div style="font-size: 0.9rem; font-family: monospace; font-weight: 700; color: #4f46e5; margin-top: 0.25rem;">#${order.id}</div>
          <div style="font-size: 0.85rem; color: #64748b;">Date: ${dateStr}</div>
          <div style="font-size: 0.85rem; color: #64748b;">Status: <strong style="text-transform: uppercase;">${order.status}</strong></div>
        </div>
      </div>

      <div style="display: flex; justify-content: space-between; margin-bottom: 2rem; padding: 1rem; background: #f8fafc; border-radius: 8px;">
        <div>
          <h4 style="margin: 0 0 0.5rem 0; font-size: 0.85rem; text-transform: uppercase; color: #64748b;">Billed To / Shipping Address</h4>
          <div style="font-size: 0.9rem; line-height: 1.4;">
            <strong>${addr.fullname || 'Customer'}</strong><br />
            ${addr.address || ''}<br />
            ${addr.city || ''}, ${addr.zip || ''}, ${addr.country || ''}
          </div>
        </div>
        <div style="text-align: right;">
          <h4 style="margin: 0 0 0.5rem 0; font-size: 0.85rem; text-transform: uppercase; color: #64748b;">Payment Method</h4>
          <div style="font-size: 0.9rem; font-weight: 600; color: #1e293b;">
            ${(order.payment_method || 'Credit Card').toUpperCase().replace('_', ' ')}
          </div>
          ${order.coupon_code ? `<div style="font-size: 0.85rem; color: #16a34a; margin-top: 0.25rem;">Coupon Applied: ${order.coupon_code}</div>` : ''}
        </div>
      </div>

      <table class="invoice-table">
        <thead>
          <tr>
            <th>Item Description</th>
            <th style="text-align: center;">Qty</th>
            <th style="text-align: right;">Unit Price</th>
            <th style="text-align: right;">Line Total</th>
          </tr>
        </thead>
        <tbody>
          ${itemsHtml}
        </tbody>
      </table>

      <div style="display: flex; justify-content: flex-end; margin-top: 1.5rem;">
        <div style="width: 260px;">
          <div style="display: flex; justify-content: space-between; padding: 0.35rem 0; font-size: 0.9rem; color: #64748b;">
            <span>Subtotal:</span>
            <span>$${subtotal.toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between; padding: 0.35rem 0; font-size: 0.9rem; color: #64748b;">
            <span>Estimated Tax (8%):</span>
            <span>$${tax.toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between; padding: 0.35rem 0; font-size: 0.9rem; color: #64748b;">
            <span>Shipping:</span>
            <span>$${shipping.toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between; padding: 0.75rem 0; font-size: 1.1rem; font-weight: 800; color: #1e293b; border-top: 2px solid #e2e8f0; margin-top: 0.5rem;">
            <span>Total Amount:</span>
            <span>$${order.total_amount.toFixed(2)}</span>
          </div>
        </div>
      </div>

      <div style="margin-top: 3rem; text-align: center; font-size: 0.85rem; color: #94a3b8; border-top: 1px solid #e2e8f0; padding-top: 1rem;">
        Thank you for shopping with Storefront Pro! For support inquiries, contact support@storefrontpro.com.
      </div>
    `;

    modal.classList.add('active');
  },

  closeInvoiceModal() {
    const modal = document.getElementById('invoice-modal');
    if (modal) modal.classList.remove('active');
  }
};

document.addEventListener('DOMContentLoaded', () => {
  app.init();
});
