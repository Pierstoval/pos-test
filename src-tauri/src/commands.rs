use rusqlite::params;
use tauri::State;
use uuid::Uuid;

use crate::db::DbState;
use crate::models::*;

// ── Inner functions (testable without Tauri runtime) ────────────────────────

pub(crate) fn list_products_inner(db: &DbState) -> Result<Vec<Product>, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let mut stmt = conn
        .prepare("SELECT id, name, price, category, available FROM products ORDER BY name")
        .map_err(|e| format!("Query error: {e}"))?;

    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                category: row.get(3)?,
                available: row.get::<_, i64>(4)? != 0,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    Ok(products)
}

pub(crate) fn create_product_inner(
    db: &DbState,
    payload: CreateProductPayload,
) -> Result<Product, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO products (id, name, price, category, available) VALUES (?1, ?2, ?3, ?4, 1)",
        params![id, payload.name, payload.price, payload.category],
    )
    .map_err(|e| format!("Insert error: {e}"))?;

    Ok(Product {
        id,
        name: payload.name,
        price: payload.price,
        category: payload.category,
        available: true,
    })
}

pub(crate) fn update_product_inner(
    db: &DbState,
    payload: UpdateProductPayload,
) -> Result<Product, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let available_int: i64 = if payload.available { 1 } else { 0 };

    let rows_affected = conn
        .execute(
            "UPDATE products SET name = ?1, price = ?2, category = ?3, available = ?4 WHERE id = ?5",
            params![
                payload.name,
                payload.price,
                payload.category,
                available_int,
                payload.id
            ],
        )
        .map_err(|e| format!("Update error: {e}"))?;

    if rows_affected == 0 {
        return Err(format!("Product not found: {}", payload.id));
    }

    Ok(Product {
        id: payload.id,
        name: payload.name,
        price: payload.price,
        category: payload.category,
        available: payload.available,
    })
}

pub(crate) fn toggle_product_availability_inner(
    db: &DbState,
    product_id: String,
) -> Result<bool, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    // Read current availability.
    let current: i64 = conn
        .query_row(
            "SELECT available FROM products WHERE id = ?1",
            params![product_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Product not found ({}): {e}", product_id))?;

    let new_value: i64 = if current != 0 { 0 } else { 1 };

    conn.execute(
        "UPDATE products SET available = ?1 WHERE id = ?2",
        params![new_value, product_id],
    )
    .map_err(|e| format!("Update error: {e}"))?;

    Ok(new_value != 0)
}

pub(crate) fn create_order_inner(
    db: &DbState,
    payload: CreateOrderPayload,
) -> Result<OrderWithItems, String> {
    let mut conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    if payload.items.is_empty() {
        return Err("Cannot create an order with no items".to_string());
    }

    // Compute totals.
    let mut order_items: Vec<OrderItem> = Vec::with_capacity(payload.items.len());
    let order_id = Uuid::new_v4().to_string();
    let mut order_total: i64 = 0;

    for item in &payload.items {
        if item.quantity <= 0 {
            return Err(format!(
                "Invalid quantity {} for product {}",
                item.quantity, item.product_id
            ));
        }
        let line_total = item.unit_price * item.quantity;
        order_total += line_total;
        order_items.push(OrderItem {
            id: Uuid::new_v4().to_string(),
            order_id: order_id.clone(),
            product_id: item.product_id.clone(),
            product_name: item.product_name.clone(),
            unit_price: item.unit_price,
            quantity: item.quantity,
            total: line_total,
        });
    }

    let created_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Execute inside a database transaction for atomicity.
    let tx = conn
        .transaction()
        .map_err(|e| format!("Transaction begin error: {e}"))?;

    tx.execute(
        "INSERT INTO orders (id, created_at, total, payment_method) VALUES (?1, ?2, ?3, ?4)",
        params![order_id, created_at, order_total, payload.payment_method.as_db_str()],
    )
    .map_err(|e| format!("Insert order error: {e}"))?;

    for oi in &order_items {
        tx.execute(
            "INSERT INTO order_items (id, order_id, product_id, product_name, unit_price, quantity, total)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                oi.id,
                oi.order_id,
                oi.product_id,
                oi.product_name,
                oi.unit_price,
                oi.quantity,
                oi.total
            ],
        )
        .map_err(|e| format!("Insert order item error: {e}"))?;
    }

    tx.commit()
        .map_err(|e| format!("Transaction commit error: {e}"))?;

    Ok(OrderWithItems {
        order: Order {
            id: order_id,
            created_at,
            total: order_total,
            payment_method: payload.payment_method,
        },
        items: order_items,
    })
}

pub(crate) fn list_orders_inner(db: &DbState) -> Result<Vec<OrderWithItems>, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    // Fetch all orders.
    let mut order_stmt = conn
        .prepare(
            "SELECT id, created_at, total, payment_method FROM orders ORDER BY created_at DESC",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let orders: Vec<Order> = order_stmt
        .query_map([], |row| {
            let pm_str: String = row.get(3)?;
            let payment_method = PaymentMethod::from_db_str(&pm_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::from(e),
                )
            })?;
            Ok(Order {
                id: row.get(0)?,
                created_at: row.get(1)?,
                total: row.get(2)?,
                payment_method,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    // Fetch all items and group by order_id.
    let mut item_stmt = conn
        .prepare(
            "SELECT id, order_id, product_id, product_name, unit_price, quantity, total
             FROM order_items
             ORDER BY order_id",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let all_items: Vec<OrderItem> = item_stmt
        .query_map([], |row| {
            Ok(OrderItem {
                id: row.get(0)?,
                order_id: row.get(1)?,
                product_id: row.get(2)?,
                product_name: row.get(3)?,
                unit_price: row.get(4)?,
                quantity: row.get(5)?,
                total: row.get(6)?,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    // Build a map of order_id -> items.
    let mut items_map: std::collections::HashMap<String, Vec<OrderItem>> =
        std::collections::HashMap::new();
    for item in all_items {
        items_map
            .entry(item.order_id.clone())
            .or_default()
            .push(item);
    }

    let result: Vec<OrderWithItems> = orders
        .into_iter()
        .map(|order| {
            let items = items_map.remove(&order.id).unwrap_or_default();
            OrderWithItems { order, items }
        })
        .collect();

    Ok(result)
}

pub(crate) fn get_dashboard_summary_inner(db: &DbState) -> Result<DashboardSummary, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    // Grand totals.
    let (total_revenue, total_transactions): (i64, i64) = conn
        .query_row(
            "SELECT COALESCE(SUM(total), 0), COUNT(*) FROM orders",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Query error: {e}"))?;

    // Per-product summary.
    let mut prod_stmt = conn
        .prepare(
            "SELECT product_id, product_name,
                    SUM(quantity) AS total_qty,
                    SUM(total) AS total_rev
             FROM order_items
             GROUP BY product_id
             ORDER BY total_rev DESC",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let per_product: Vec<ProductSalesSummary> = prod_stmt
        .query_map([], |row| {
            Ok(ProductSalesSummary {
                product_id: row.get(0)?,
                product_name: row.get(1)?,
                total_quantity: row.get(2)?,
                total_revenue: row.get(3)?,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    // Per-payment-method breakdown.
    let mut pm_stmt = conn
        .prepare(
            "SELECT payment_method,
                    SUM(total) AS total_rev,
                    COUNT(*) AS tx_count
             FROM orders
             GROUP BY payment_method
             ORDER BY payment_method",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let per_payment_method: Vec<PaymentMethodBreakdown> = pm_stmt
        .query_map([], |row| {
            let pm_str: String = row.get(0)?;
            let payment_method = PaymentMethod::from_db_str(&pm_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::from(e),
                )
            })?;
            Ok(PaymentMethodBreakdown {
                payment_method,
                total_revenue: row.get(1)?,
                transaction_count: row.get(2)?,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    Ok(DashboardSummary {
        total_revenue,
        total_transactions,
        per_product,
        per_payment_method,
    })
}

pub(crate) fn reset_database_inner(db: &DbState) -> Result<(), String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    conn.execute_batch(
        "DROP TABLE IF EXISTS order_items;
         DROP TABLE IF EXISTS orders;
         DROP TABLE IF EXISTS products;",
    )
    .map_err(|e| format!("Drop tables error: {e}"))?;

    crate::db::create_tables(&conn)?;

    Ok(())
}

// ── Tauri command wrappers ──────────────────────────────────────────────────

#[tauri::command]
pub fn list_products(state: State<'_, DbState>) -> Result<Vec<Product>, String> {
    list_products_inner(&state)
}

#[tauri::command]
pub fn create_product(
    state: State<'_, DbState>,
    payload: CreateProductPayload,
) -> Result<Product, String> {
    create_product_inner(&state, payload)
}

#[tauri::command]
pub fn update_product(
    state: State<'_, DbState>,
    payload: UpdateProductPayload,
) -> Result<Product, String> {
    update_product_inner(&state, payload)
}

#[tauri::command]
pub fn toggle_product_availability(
    state: State<'_, DbState>,
    product_id: String,
) -> Result<bool, String> {
    toggle_product_availability_inner(&state, product_id)
}

#[tauri::command]
pub fn create_order(
    state: State<'_, DbState>,
    payload: CreateOrderPayload,
) -> Result<OrderWithItems, String> {
    create_order_inner(&state, payload)
}

#[tauri::command]
pub fn list_orders(state: State<'_, DbState>) -> Result<Vec<OrderWithItems>, String> {
    list_orders_inner(&state)
}

#[tauri::command]
pub fn get_dashboard_summary(state: State<'_, DbState>) -> Result<DashboardSummary, String> {
    get_dashboard_summary_inner(&state)
}

#[tauri::command]
pub fn reset_database(state: State<'_, DbState>) -> Result<(), String> {
    reset_database_inner(&state)
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db_in_memory;

    fn make_product(db: &DbState, name: &str, price: i64, category: &str) -> Product {
        create_product_inner(
            db,
            CreateProductPayload {
                name: name.to_string(),
                price,
                category: category.to_string(),
            },
        )
        .expect("create_product_inner failed")
    }

    #[test]
    fn list_products_empty() {
        let db = init_db_in_memory();
        let products = list_products_inner(&db).unwrap();
        assert!(products.is_empty());
    }

    #[test]
    fn create_and_list_products() {
        let db = init_db_in_memory();
        let created = make_product(&db, "Cola", 150, "soft_drink");
        assert_eq!(created.name, "Cola");
        assert_eq!(created.price, 150);
        assert_eq!(created.category, "soft_drink");
        assert!(created.available);

        let products = list_products_inner(&db).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, created.id);
        assert_eq!(products[0].name, "Cola");
    }

    #[test]
    fn update_product_changes_fields() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Chips", 200, "snack");

        let updated = update_product_inner(
            &db,
            UpdateProductPayload {
                id: p.id.clone(),
                name: "Crisps".to_string(),
                price: 250,
                category: "snack".to_string(),
                available: false,
            },
        )
        .unwrap();

        assert_eq!(updated.id, p.id);
        assert_eq!(updated.name, "Crisps");
        assert_eq!(updated.price, 250);
        assert!(!updated.available);

        // Verify via list
        let products = list_products_inner(&db).unwrap();
        assert_eq!(products[0].name, "Crisps");
        assert!(!products[0].available);
    }

    #[test]
    fn update_product_not_found() {
        let db = init_db_in_memory();
        let result = update_product_inner(
            &db,
            UpdateProductPayload {
                id: "nonexistent".to_string(),
                name: "X".to_string(),
                price: 100,
                category: "snack".to_string(),
                available: true,
            },
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Product not found"));
    }

    #[test]
    fn toggle_availability() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Water", 100, "soft_drink");
        assert!(p.available);

        // Toggle off
        let new_state = toggle_product_availability_inner(&db, p.id.clone()).unwrap();
        assert!(!new_state);

        // Toggle back on
        let new_state = toggle_product_availability_inner(&db, p.id.clone()).unwrap();
        assert!(new_state);

        // Toggle off again
        let new_state = toggle_product_availability_inner(&db, p.id).unwrap();
        assert!(!new_state);
    }

    #[test]
    fn create_order_success() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Candy", 50, "sweets");

        let order = create_order_inner(
            &db,
            CreateOrderPayload {
                items: vec![CreateOrderItemPayload {
                    product_id: p.id.clone(),
                    product_name: "Candy".to_string(),
                    unit_price: 50,
                    quantity: 3,
                }],
                payment_method: PaymentMethod::Cash,
            },
        )
        .unwrap();

        assert_eq!(order.order.total, 150);
        assert_eq!(order.order.payment_method, PaymentMethod::Cash);
        assert_eq!(order.items.len(), 1);
        assert_eq!(order.items[0].product_id, p.id);
        assert_eq!(order.items[0].quantity, 3);
        assert_eq!(order.items[0].total, 150);

        // Verify via list_orders
        let orders = list_orders_inner(&db).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order.total, 150);
        assert_eq!(orders[0].items.len(), 1);
    }

    #[test]
    fn create_order_empty_items_fails() {
        let db = init_db_in_memory();
        let result = create_order_inner(
            &db,
            CreateOrderPayload {
                items: vec![],
                payment_method: PaymentMethod::Card,
            },
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no items"));
    }

    #[test]
    fn dashboard_summary_reflects_orders() {
        let db = init_db_in_memory();
        let p1 = make_product(&db, "Soda", 200, "soft_drink");
        let p2 = make_product(&db, "Bar", 100, "snack");

        // Order 1: 2x Soda, paid by cash => 400
        create_order_inner(
            &db,
            CreateOrderPayload {
                items: vec![CreateOrderItemPayload {
                    product_id: p1.id.clone(),
                    product_name: "Soda".to_string(),
                    unit_price: 200,
                    quantity: 2,
                }],
                payment_method: PaymentMethod::Cash,
            },
        )
        .unwrap();

        // Order 2: 1x Soda + 3x Bar, paid by card => 200 + 300 = 500
        create_order_inner(
            &db,
            CreateOrderPayload {
                items: vec![
                    CreateOrderItemPayload {
                        product_id: p1.id.clone(),
                        product_name: "Soda".to_string(),
                        unit_price: 200,
                        quantity: 1,
                    },
                    CreateOrderItemPayload {
                        product_id: p2.id.clone(),
                        product_name: "Bar".to_string(),
                        unit_price: 100,
                        quantity: 3,
                    },
                ],
                payment_method: PaymentMethod::Card,
            },
        )
        .unwrap();

        let summary = get_dashboard_summary_inner(&db).unwrap();
        assert_eq!(summary.total_revenue, 900);
        assert_eq!(summary.total_transactions, 2);

        // Per-product: Soda = 600 (3 units), Bar = 300 (3 units). Ordered by revenue DESC.
        assert_eq!(summary.per_product.len(), 2);
        assert_eq!(summary.per_product[0].product_name, "Soda");
        assert_eq!(summary.per_product[0].total_quantity, 3);
        assert_eq!(summary.per_product[0].total_revenue, 600);
        assert_eq!(summary.per_product[1].product_name, "Bar");
        assert_eq!(summary.per_product[1].total_quantity, 3);
        assert_eq!(summary.per_product[1].total_revenue, 300);

        // Per-payment-method: ordered by payment_method ASC => card, cash
        assert_eq!(summary.per_payment_method.len(), 2);
        assert_eq!(summary.per_payment_method[0].payment_method, PaymentMethod::Card);
        assert_eq!(summary.per_payment_method[0].total_revenue, 500);
        assert_eq!(summary.per_payment_method[0].transaction_count, 1);
        assert_eq!(summary.per_payment_method[1].payment_method, PaymentMethod::Cash);
        assert_eq!(summary.per_payment_method[1].total_revenue, 400);
        assert_eq!(summary.per_payment_method[1].transaction_count, 1);
    }
}
