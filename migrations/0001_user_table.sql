CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TYPE expense_category AS ENUM (
    'Groceries', 'Leisure', 'Electronics', 'Utilities', 'Clothing', 'Health', 'Others'
);

CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    price NUMERIC NOT NULL,
    description TEXT,
    category expense_category NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
