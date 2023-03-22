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
    name VARCHAR(100) NOT NULL,
    user_id SERIAL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE serving (
    id SERIAL,
    food_id SERIAL,
    unit VARCHAR(100) NOT NULL,
    amount FLOAT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (food_id) REFERENCES food(id)
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

INSERT INTO user_account(name, email, gender, weight, birthdate, password_hash) VALUES
    ('Admin', 'admin@localhost', 'M', '70', '1970-01-01', 'f1884986dc7b68fb7ac18c5a7dd2bea6467565300378713fca1fe468bc4be6a6'); /* Password: nutrinow_admin */

INSERT INTO nutrient(name, unit) VALUES
    /* Macronutrients */
    ('Protein', 'g'),
    ('Carbohydrates', 'g'), /* Total carbohydrates, including fiber and sugar */
    ('Fats', 'g'), /* Total fat, including saturated, unsaturated, and trans fat */
    ('Sugar', 'g'),
    ('Fiber', 'g'),
    ('Saturated Fat', 'g'),
    ('Unsaturated Fat', 'g'),
    ('Trans Fat', 'g'),
    /* Micronutrients */
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
    ('Cholesterol', 'mg'),
    /* Macrominerals */
    ('Calcium', 'mg'),
    ('Magnesium', 'mg'),
    ('Potassium', 'mg'),
    ('Phosphorus', 'mg'),
    ('Sodium', 'mg'),
    /* Trace Minerals */
    ('Iron', 'mg'),
    ('Zinc', 'mg'),
    /* Others (not really 'nutrients', but they are essential information) */
    ('Water', 'ml'),
    ('Calories', 'kcal');

/* TODO: Import foods from USDA database */
INSERT INTO food(name, user_id) VALUES
    ('Chicken Breast', 1),
    ('Skimmed Milk', 1);

INSERT INTO serving(food_id, amount, unit) VALUES
    (1, '100', 'g'), /* Chicken Breast */
    (1, '200', 'g'), /* Chicken Breast */
    (2, '200', 'ml'); /* Skimmed Milk */

INSERT INTO serving_nutrient(serving_id, nutrient_id, amount) VALUES
    (1, 1, 31), /* Protein */
    (1, 2, 0), /* Carbohydrates */
    (1, 3, 3.57), /* Fats */
    (1, 4, 0), /* Sugar */
    (1, 5, 0), /* Fiber */
    (1, 6, 1.01), /* Saturated Fat */
    (1, 7, 2.01), /* Unsaturated Fat */
    (1, 8, 0), /* Trans Fat */
    (1, 9, 6), /* Vitamin A */
    (1, 10, 0), /* Vitamin B1 */
    (1, 11, 0), /* Vitamin B2 */
    (1, 12, 0), /* Vitamin B3 */
    (1, 13, 0), /* Vitamin B5 */
    (1, 14, 0.6), /* Vitamin B6 */
    (1, 15, 0), /* Vitamin B7 */
    (1, 16, 0), /* Vitamin B9 */
    (1, 17, 0.34), /* Vitamin B12 */
    (1, 18, 0), /* Vitamin C */
    (1, 19, 0.1), /* Vitamin D */
    (1, 20, 0.27), /* Vitamin E */
    (1, 21, 0.3), /* Vitamin K */
    (1, 22, 85), /* Cholesterol */
    (1, 23, 15), /* Calcium */
    (1, 24, 29), /* Magnesium */
    (1, 25, 256), /* Potassium */
    (1, 26, 228), /* Phosphorus */
    (1, 27, 74), /* Sodium */
    (1, 28, 1.04), /* Iron */
    (1, 29, 1), /* Zinc */
    (1, 30, 0.0653), /* Water */
    (1, 31, 165); /* Calories */

/* TODO: Update values */
INSERT INTO serving_nutrient(serving_id, nutrient_id, amount) VALUES
    (2, 1, 31), /* Protein */
    (2, 2, 0), /* Carbohydrates */
    (2, 3, 0), /* Sugar */
    (2, 4, 0), /* Fiber */
    (2, 5, 3.57), /* Fats */
    (2, 6, 1.01), /* Saturated Fat */
    (2, 7, 2.01), /* Unsaturated Fat */
    (2, 8, 0), /* Trans Fat */
    (2, 9, 6), /* Vitamin A */
    (2, 10, 0), /* Vitamin B1 */
    (2, 11, 0), /* Vitamin B2 */
    (2, 12, 0), /* Vitamin B3 */
    (2, 13, 0), /* Vitamin B5 */
    (2, 14, 0.6), /* Vitamin B6 */
    (2, 15, 0), /* Vitamin B7 */
    (2, 16, 0), /* Vitamin B9 */
    (2, 17, 0.34), /* Vitamin B12 */
    (2, 18, 0), /* Vitamin C */
    (2, 19, 0.1), /* Vitamin D */
    (2, 20, 0.27), /* Vitamin E */
    (2, 21, 0.3), /* Vitamin K */
    (2, 22, 85), /* Cholesterol */
    (2, 23, 15), /* Calcium */
    (2, 24, 29), /* Magnesium */
    (2, 25, 256), /* Potassium */
    (2, 26, 228), /* Phosphorus */
    (2, 27, 74), /* Sodium */
    (2, 28, 1.04), /* Iron */
    (2, 29, 1), /* Zinc */
    (2, 30, 0.0653), /* Water */
    (2, 31, 165); /* Calories */


/* TODO: Add data for Skimmed Milk */

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
    (1, 31, 100, false);

INSERT INTO meal(name, diet_id) VALUES
    ('Breakfest', 1),
    ('Lunch', 1);

INSERT INTO meal_serving(meal_id, serving_id, amount) VALUES
    (1, 1, 50),
    (2, 1, 100);

