CREATE TABLE products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE stock_units (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    brand TEXT NOT NULL,
    country TEXT NOT NULL,
    product_id INTEGER NOT NULL,
    options TEXT
);

CREATE TABLE product_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    product_id INTEGER NOT NULL
);

INSERT INTO products (id, name)
VALUES (1, 'Пиво');

INSERT INTO product_options (id, name, description, product_id)
VALUES (1, 'Темное', 'Пиво темное вкусненькое', 1),
       (2, 'Светлое', 'Пиво светлое вкусненькое', 1),
       (3, 'Пшеничное', 'Пшеничное как квасик', 1),
       (4, 'Фильтрованное', 'Ну норм', 1);

INSERT INTO stock_units (brand, country, product_id, options)
VALUES ('Clausthaler', 'Германия', 1, '1, 2'),
       ('Балтика', 'Россия', 1, '1');