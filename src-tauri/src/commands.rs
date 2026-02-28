use rusqlite::params;
use tauri::State;
use uuid::Uuid;

use crate::db::DbState;
use crate::models::*;

// ── Inner functions (testable without Tauri runtime) ────────────────────────

pub(crate) fn list_categories_inner(db: &DbState) -> Result<Vec<Category>, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let mut stmt = conn
        .prepare("SELECT id, label, color FROM categories ORDER BY label")
        .map_err(|e| format!("Query error: {e}"))?;

    let categories = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                label: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {e}"))?;

    Ok(categories)
}

pub(crate) fn create_category_inner(
    db: &DbState,
    payload: CreateCategoryPayload,
) -> Result<Category, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    conn.execute(
        "INSERT INTO categories (id, label, color) VALUES (?1, ?2, ?3)",
        params![payload.id, payload.label, payload.color],
    )
    .map_err(|e| format!("Insert error: {e}"))?;

    Ok(Category {
        id: payload.id,
        label: payload.label,
        color: payload.color,
    })
}

pub(crate) fn update_category_inner(
    db: &DbState,
    payload: UpdateCategoryPayload,
) -> Result<Category, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let rows_affected = conn
        .execute(
            "UPDATE categories SET label = ?1, color = ?2 WHERE id = ?3",
            params![payload.label, payload.color, payload.id],
        )
        .map_err(|e| format!("Update error: {e}"))?;

    if rows_affected == 0 {
        return Err(format!("Category not found: {}", payload.id));
    }

    Ok(Category {
        id: payload.id,
        label: payload.label,
        color: payload.color,
    })
}

pub(crate) fn list_products_inner(db: &DbState) -> Result<Vec<Product>, String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    let mut stmt = conn
        .prepare("SELECT id, name, price, category_id, available FROM products ORDER BY name")
        .map_err(|e| format!("Query error: {e}"))?;

    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                category_id: row.get(3)?,
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
        "INSERT INTO products (id, name, price, category_id, available) VALUES (?1, ?2, ?3, ?4, 1)",
        params![id, payload.name, payload.price, payload.category_id],
    )
    .map_err(|e| format!("Insert error: {e}"))?;

    Ok(Product {
        id,
        name: payload.name,
        price: payload.price,
        category_id: payload.category_id,
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
            "UPDATE products SET name = ?1, price = ?2, category_id = ?3, available = ?4 WHERE id = ?5",
            params![
                payload.name,
                payload.price,
                payload.category_id,
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
        category_id: payload.category_id,
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

pub(crate) fn delete_product_inner(db: &DbState, product_id: String) -> Result<(), String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    // Check whether any order items reference this product.
    let order_item_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM order_items WHERE product_id = ?1",
            params![product_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Query error: {e}"))?;

    if order_item_count > 0 {
        return Err(format!(
            "Cannot delete product '{}': it is referenced by {} order item(s)",
            product_id, order_item_count
        ));
    }

    let rows_affected = conn
        .execute("DELETE FROM products WHERE id = ?1", params![product_id])
        .map_err(|e| format!("Delete error: {e}"))?;

    if rows_affected == 0 {
        return Err(format!("Product not found: {}", product_id));
    }

    Ok(())
}

pub(crate) fn delete_category_inner(db: &DbState, category_id: String) -> Result<(), String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    // Check whether any products reference this category.
    let product_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM products WHERE category_id = ?1",
            params![category_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Query error: {e}"))?;

    if product_count > 0 {
        return Err(format!(
            "Cannot delete category '{}': it is referenced by {} product(s)",
            category_id, product_count
        ));
    }

    let rows_affected = conn
        .execute("DELETE FROM categories WHERE id = ?1", params![category_id])
        .map_err(|e| format!("Delete error: {e}"))?;

    if rows_affected == 0 {
        return Err(format!("Category not found: {}", category_id));
    }

    Ok(())
}

pub(crate) fn reset_database_inner(db: &DbState) -> Result<(), String> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| format!("DB lock error: {e}"))?;

    conn.execute_batch(
        "DROP TABLE IF EXISTS order_items;
         DROP TABLE IF EXISTS orders;
         DROP TABLE IF EXISTS products;
         DROP TABLE IF EXISTS categories;",
    )
    .map_err(|e| format!("Drop tables error: {e}"))?;

    crate::db::create_tables(&conn)?;
    crate::db::create_default_data(&conn);

    Ok(())
}

// ── Tauri command wrappers ──────────────────────────────────────────────────

#[tauri::command]
pub fn list_categories(state: State<'_, DbState>) -> Result<Vec<Category>, String> {
    list_categories_inner(&state)
}

#[tauri::command]
pub fn create_category(
    state: State<'_, DbState>,
    payload: CreateCategoryPayload,
) -> Result<Category, String> {
    create_category_inner(&state, payload)
}

#[tauri::command]
pub fn update_category(
    state: State<'_, DbState>,
    payload: UpdateCategoryPayload,
) -> Result<Category, String> {
    update_category_inner(&state, payload)
}

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
pub fn delete_product(state: State<'_, DbState>, product_id: String) -> Result<(), String> {
    delete_product_inner(&state, product_id)
}

#[tauri::command]
pub fn delete_category(state: State<'_, DbState>, category_id: String) -> Result<(), String> {
    delete_category_inner(&state, category_id)
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

    fn make_product(db: &DbState, name: &str, price: i64, category_id: &str) -> Product {
        create_product_inner(
            db,
            CreateProductPayload {
                name: name.to_string(),
                price,
                category_id: category_id.to_string(),
            },
        )
        .expect("create_product_inner failed")
    }

    #[test]
    fn list_products_returns_defaults() {
        let db = init_db_in_memory();
        let products = list_products_inner(&db).unwrap();
        assert_eq!(products.len(), 20);
    }

    #[test]
    fn create_and_list_products() {
        let db = init_db_in_memory();
        let created = make_product(&db, "Cola", 150, "boisson-sans-alcool");
        assert_eq!(created.name, "Cola");
        assert_eq!(created.price, 150);
        assert_eq!(created.category_id, "boisson-sans-alcool");
        assert!(created.available);

        let products = list_products_inner(&db).unwrap();
        assert_eq!(products.len(), 21);
        let cola = products.iter().find(|p| p.id == created.id).expect("Cola should be in the list");
        assert_eq!(cola.name, "Cola");
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
                category_id: "snack".to_string(),
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
        let crisps = products.iter().find(|p| p.id == updated.id).expect("Crisps should be in the list");
        assert_eq!(crisps.name, "Crisps");
        assert!(!crisps.available);
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
                category_id: "snack".to_string(),
                available: true,
            },
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Product not found"));
    }

    #[test]
    fn toggle_availability() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Water", 100, "boisson-sans-alcool");
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
        let p = make_product(&db, "Candy", 50, "sucreries");

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
        let p1 = make_product(&db, "Soda", 200, "boisson-sans-alcool");
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

    #[test]
    fn delete_product_success() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Temp Item", 100, "snack");

        let before = list_products_inner(&db).unwrap().len();
        delete_product_inner(&db, p.id.clone()).unwrap();
        let after = list_products_inner(&db).unwrap().len();

        assert_eq!(after, before - 1);
        assert!(list_products_inner(&db).unwrap().iter().all(|prod| prod.id != p.id));
    }

    #[test]
    fn delete_product_not_found() {
        let db = init_db_in_memory();
        let result = delete_product_inner(&db, "nonexistent".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Product not found"));
    }

    #[test]
    fn delete_product_with_order_items_fails() {
        let db = init_db_in_memory();
        let p = make_product(&db, "Ordered Item", 200, "snack");

        // Create an order referencing this product.
        create_order_inner(
            &db,
            CreateOrderPayload {
                items: vec![CreateOrderItemPayload {
                    product_id: p.id.clone(),
                    product_name: "Ordered Item".to_string(),
                    unit_price: 200,
                    quantity: 1,
                }],
                payment_method: PaymentMethod::Cash,
            },
        )
        .unwrap();

        let result = delete_product_inner(&db, p.id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("referenced by"));
    }

    #[test]
    fn delete_category_success() {
        let db = init_db_in_memory();

        // Create a fresh category with no products.
        create_category_inner(
            &db,
            CreateCategoryPayload {
                id: "test-cat".to_string(),
                label: "Test Cat".to_string(),
                color: "#ff0000".to_string(),
            },
        )
        .unwrap();

        let before = list_categories_inner(&db).unwrap().len();
        delete_category_inner(&db, "test-cat".to_string()).unwrap();
        let after = list_categories_inner(&db).unwrap().len();

        assert_eq!(after, before - 1);
    }

    #[test]
    fn delete_category_not_found() {
        let db = init_db_in_memory();
        let result = delete_category_inner(&db, "nonexistent".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Category not found"));
    }

    #[test]
    fn delete_category_with_products_fails() {
        let db = init_db_in_memory();

        // The "snack" category has default products referencing it.
        let result = delete_category_inner(&db, "snack".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("referenced by"));
    }
}
