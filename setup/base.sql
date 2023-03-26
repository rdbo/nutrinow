/* Use with PostgreSQL */

\c postgres;
DROP DATABASE IF EXISTS nutrinow;
CREATE DATABASE nutrinow;
\c nutrinow;

CREATE TABLE user_account (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    gender CHAR(1) NOT NULL,
    weight FLOAT NOT NULL /* in kilograms */,
    birthdate DATE NOT NULL,
    password_hash CHAR(64) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE nutrient (
    id SERIAL,
    name VARCHAR(100) UNIQUE NOT NULL,
    unit VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

/* NOTE: There should be a cron job that will automatically delete expired session tokens */
CREATE TABLE user_session (
    id UUID NOT NULL,
    user_id SERIAL,
    expiry_date DATE NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE food (
    id SERIAL,
    name VARCHAR(255) NOT NULL,
    user_id SERIAL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE serving (
    id INTEGER, /* SERIAL cannot be NULL */
    food_id SERIAL,
    unit VARCHAR(100) NOT NULL,
    amount FLOAT NOT NULL,
    relative INTEGER NULL, /* contains the ID of another serving, and a relative amount, or NULL. NOTE: Make sure it is never relative to another relative serving */
    PRIMARY KEY(id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (relative) REFERENCES serving(id)
);

CREATE TABLE serving_nutrient (
    serving_id SERIAL,
    nutrient_id SERIAL,
    amount FLOAT NOT NULL,
    FOREIGN KEY (serving_id) REFERENCES serving(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id)
);

CREATE TABLE diet (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    user_id SERIAL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE diet_nutrition (
    diet_id SERIAL,
    nutrient_id SERIAL,
    daily_intake FLOAT NOT NULL,
    relative BOOLEAN NOT NULL, /* relative to body weight */
    FOREIGN KEY (diet_id) REFERENCES diet(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id)
);

CREATE TABLE meal (
    id SERIAL,
    diet_id SERIAL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (diet_id) REFERENCES diet(id)
);

CREATE TABLE meal_serving (
    meal_id SERIAL,
    serving_id SERIAL,
    amount FLOAT NOT NULL,
    FOREIGN KEY (meal_id) REFERENCES meal(id),
    FOREIGN KEY (serving_id) REFERENCES serving(id)
);

/* TODO: Add nutrient categories */

INSERT INTO user_account(name, email, gender, weight, birthdate, password_hash) VALUES
    ('Admin', 'admin@localhost', 'M', '70', '1970-01-01', 'f1884986dc7b68fb7ac18c5a7dd2bea6467565300378713fca1fe468bc4be6a6'); /* Password: nutrinow_admin */

INSERT INTO nutrient(name, unit) VALUES
    /* Macronutrients */
    ('Protein', 'g'),
    ('Carbohydrates', 'g'), /* Total carbohydrates, including fiber and sugar */
    ('Fats', 'g'), /* Total fat, including saturated, unsaturated, and trans fat */
    /* Carbohydrates */
    ('Sugars', 'g'),
    ('Fiber', 'g'),
    /* Lipids (Fats) */
    ('Saturated Fat', 'g'),
    ('Unsaturated Fat', 'g'),
    ('Trans Fat', 'g'),
    /* Vitamins */
    ('Vitamin A', 'μg'),
    ('Vitamin B1', 'μg'),
    ('Vitamin B2', 'μg'),
    ('Vitamin B3', 'μg'),
    ('Vitamin B5', 'μg'),
    ('Vitamin B6', 'mg'),
    ('Vitamin B7', 'μg'),
    ('Vitamin B9', 'μg'),
    ('Vitamin B12', 'μg'),
    ('Vitamin C', 'mg'),
    ('Vitamin D', 'μg'),
    ('Vitamin E', 'mg'),
    ('Vitamin K', 'μg'),
    /* Minerals */
    ('Calcium', 'mg'),
    ('Iron', 'mg'),
    ('Magnesium', 'mg'),
    ('Phosphorus', 'mg'),
    ('Potassium', 'mg'),
    ('Sodium', 'mg'),
    ('Zinc', 'mg'),
    ('Copper', 'mg'),
    ('Manganese', 'mg'),
    ('Selenium', 'mg'),
    /* Others (not really 'nutrients', but they are essential information) */
    ('Water', 'ml'); /* USDA data has water in grams (g), but 1 gram of water = 1 ml of water */
/*    ('Calories', 'kcal');*/ /* calories will be calculated dynamically (Proteins * 4 + Carbos * 4 + Fats * 9) */

INSERT INTO diet(name, user_id) VALUES
    ('Diet 1', 1),
    ('Cutting', 1),
    ('Bulking', 1);

INSERT INTO diet_nutrition(diet_id, nutrient_id, daily_intake, relative) VALUES
    (1, 1, 100, true),
    (1, 2, 100, true),
    (1, 3, 100, true),
    (1, 4, 100, false),
    (1, 5, 100, false),
    (1, 6, 100, false),
    (1, 7, 100, false),
    (1, 8, 100, false),
    (1, 9, 100, false),
    (1, 10, 100, false),
    (1, 11, 100, false),
    (1, 12, 100, false),
    (1, 13, 100, false),
    (1, 14, 100, false),
    (1, 15, 100, false),
    (1, 16, 100, false),
    (1, 17, 100, false),
    (1, 18, 100, false),
    (1, 19, 100, false),
    (1, 20, 100, false),
    (1, 21, 100, false),
    (1, 22, 100, false),
    (1, 23, 100, false),
    (1, 24, 100, false),
    (1, 25, 100, false),
    (1, 26, 100, false),
    (1, 27, 100, false),
    (1, 28, 100, false),
    (1, 29, 100, false),
    (1, 30, 100, false),
    (1, 31, 100, false),
    (1, 32, 100, false);

INSERT INTO meal(name, diet_id) VALUES
    ('Breakfest', 1),
    ('Lunch', 1);

