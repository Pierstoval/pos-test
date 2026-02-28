export interface Category {
	id: string;
	label: string;
	color: string;
}

export interface Product {
	id: string;
	name: string;
	/** Price in cents (e.g. 150 = 1.50 EUR). */
	price: number;
	/** Foreign key referencing the categories table. */
	category_id: string;
	available: boolean;
}

export interface Order {
	id: string;
	created_at: string;
	total: number;
	payment_method: "cash" | "card";
}

export interface OrderItem {
	id: string;
	order_id: string;
	product_id: string;
	product_name: string;
	unit_price: number;
	quantity: number;
	total: number;
}

/** Flattened via #[serde(flatten)] on the Rust side. */
export interface OrderWithItems extends Order {
	items: OrderItem[];
}

export interface CreateOrderItemPayload {
	product_id: string;
	product_name: string;
	unit_price: number;
	quantity: number;
}

export interface CreateOrderPayload {
	items: CreateOrderItemPayload[];
	payment_method: "cash" | "card";
}

export interface CreateProductPayload {
	name: string;
	/** Price in cents. */
	price: number;
	category_id: string;
}

export interface UpdateProductPayload {
	id: string;
	name: string;
	/** Price in cents. */
	price: number;
	category_id: string;
	available: boolean;
}

/** Client-side cart item (product + chosen quantity). */
export interface CartItem {
	product: Product;
	quantity: number;
}
